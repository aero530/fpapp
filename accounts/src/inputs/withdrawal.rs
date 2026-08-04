//! User input withdrawal and tax status values

use serde::{Deserialize, Serialize};

use super::{Settings, fixed_with_inflation};
#[cfg(test)]
use crate::simulation::YearlyImpact;
use crate::simulation::{Table, YearRange, YearlyTotals};

/// used to populate account dropdown for withdrawal type selection
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WithdrawalOptions {
    /// fixed dollar amount
    Fixed,
    /// fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)
    FixedWithInflation,
    /// take money out in equal amounts each year such that the balance at endOut is zero
    EndAtZero,
    /// cost of living fraction of total savings
    /// take out the current cost of living * (this accounts value / total savings)
    ColFracOfSavings,
    /// Withdrawals are manually calculated or are none
    Other,
}

impl WithdrawalOptions {
    /// The withdrawal amount for the year, capped by the account balance for
    /// that year (so the account can not become overdrawn).
    #[allow(clippy::too_many_arguments)]
    pub fn value(
        &self,
        withdrawal_value: f64,
        year: u32,
        settings: &Settings,
        year_out: Option<YearRange>,
        value_table: &Table,
        totals: &YearlyTotals,
        tax_status: TaxStatus,
    ) -> f64 {
        let output = match self {
            WithdrawalOptions::Other => 0_f64,
            WithdrawalOptions::Fixed => withdrawal_value,
            WithdrawalOptions::FixedWithInflation => {
                fixed_with_inflation(withdrawal_value, year, settings)
            }
            WithdrawalOptions::EndAtZero => match year_out {
                // withdraw the fraction of the account balance that empties it by end_out
                Some(range) if year <= range.end => {
                    let account_value = value_table.get(year).unwrap_or_default();
                    account_value / (range.end - year + 1) as f64
                }
                _ => 0_f64,
            },
            WithdrawalOptions::ColFracOfSavings => {
                let prev_year = year.checked_sub(1);
                let prev_account_value = prev_year
                    .and_then(|py| value_table.get(py))
                    .unwrap_or_default();
                let prev_savings = prev_year
                    .map(|py| totals.get_saving(py))
                    .unwrap_or_default();
                // The cost of living to cover excludes healthcare that HSA
                // accounts have already paid this year (HSAs run earlier in
                // the account order), so savings are not drawn down for costs
                // that are already covered.
                let covered_by_hsa = (totals.get_healthcare_expense_total(year)
                    - totals.get_healthcare_expense(year))
                .max(0_f64);
                let col = (totals.get_col(year) - covered_by_hsa).max(0_f64);
                if prev_account_value > 0_f64 && prev_savings > 0_f64 {
                    // withdrawal from this account = cost of living this year
                    // * fraction of total savings this account represents
                    let gross = col * (prev_account_value / prev_savings);
                    match tax_status {
                        // add extra to the withdrawal to cover the income tax due on it
                        TaxStatus::ContributePretaxTaxedWhenUsed => {
                            if settings.tax_income < 100_f64 {
                                gross / (1_f64 - settings.tax_income / 100_f64)
                            } else {
                                log::warn!(
                                    "tax_income is {}% — skipping withdrawal tax gross-up",
                                    settings.tax_income
                                );
                                gross
                            }
                        }
                        _ => gross,
                    }
                } else {
                    0_f64
                }
            }
        };

        // cap the withdrawal at the account balance so the account can not go negative
        let account_value = value_table.get(year).unwrap_or_default();
        output.min(account_value)
    }
}

/// used to populate account dropdown for tax status selection
// variant names are stable identifiers in the user data file format
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxStatus {
    /// Paid with taxed income, earnings are not taxed, withdrawals are not taxed
    ///
    /// Contributions count as an expense (will be subtracted from net for the year).
    /// Contributions do not impact taxable income (as they are made with dollars that have already been taxed).
    /// Withdrawals count as income but do not to taxable income.
    /// aka 0
    /// contribute_taxed_earnings_untaxed_when_used
    ContributeTaxedEarningsUntaxedWhenUsed,
    /// Paid with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed (tax free as long as used for intended purpose)
    ///
    /// Contributions count as an expense (will be subtracted from net for the year).
    /// Contributions do not impact taxable income (as they are made with dollars that have already been taxed).
    /// Withdrawals count as income but do not to taxable income.
    /// aka 1
    ContributeTaxedEarningsTaxed,
    // not implemented.
    // NOT IMPLEMENTED ## 2=payed with taxed income, earnings are taxed in year taken out as capital gains, withdrawals are not taxed
    // aka 2
    // NotImpliemented,
    /// Paid with pretax income and taxed in year of use as income
    ///
    /// Contributions count as an expense (will be subtracted from net for the year).
    /// Contributions reduce taxable income (they are a deduction).
    /// Withdrawals count as income and add to taxable income.
    /// aka 3
    /// contribute_pretax_taxed_when_used
    ContributePretaxTaxedWhenUsed,
    /// Paid with pretax income and not taxed as income (use with HSA)
    ///
    /// Contributions count as an expense (will be subtracted from net for the year).
    /// Contributions reduce taxable income (they are a deduction).
    /// Withdrawals count as income but do not add to taxable income.
    /// aka 4
    /// contribute_pretax_untaxed_when_used
    ContributePretaxUntaxedWhenUsed,
}

#[cfg(test)]
mod tests {
    use super::super::test_fixtures::test_settings_values;
    use super::*;
    use float_cmp::assert_approx_eq;

    fn value_table(entries: &[(u32, f64)]) -> Table {
        Table(entries.iter().copied().collect())
    }

    #[test]
    fn withdrawal_fixed() {
        let settings = test_settings_values();
        let table = value_table(&[(2010, 10000.0)]);
        let totals = YearlyTotals::new();
        assert_approx_eq!(
            f64,
            WithdrawalOptions::Fixed.value(
                500_f64,
                2010,
                &settings,
                None,
                &table,
                &totals,
                TaxStatus::ContributeTaxedEarningsUntaxedWhenUsed
            ),
            500_f64
        );
        // capped by an empty balance
        let empty = value_table(&[(2010, 0.0)]);
        assert_approx_eq!(
            f64,
            WithdrawalOptions::Fixed.value(
                500_f64,
                2010,
                &settings,
                None,
                &empty,
                &totals,
                TaxStatus::ContributeTaxedEarningsUntaxedWhenUsed
            ),
            0_f64
        );
    }

    #[test]
    fn withdrawal_fixed_with_inflation() {
        let settings = test_settings_values();
        let table = value_table(&[(2010, 10000.0)]);
        let totals = YearlyTotals::new();
        // 500 * 1.05^10
        assert_approx_eq!(
            f64,
            WithdrawalOptions::FixedWithInflation.value(
                500_f64,
                2010,
                &settings,
                None,
                &table,
                &totals,
                TaxStatus::ContributeTaxedEarningsUntaxedWhenUsed
            ),
            814.447,
            epsilon = 0.001
        );
    }

    #[test]
    fn withdrawal_end_at_zero() {
        let settings = test_settings_values();
        let table = value_table(&[(2010, 10000.0)]);
        let totals = YearlyTotals::new();
        let range = Some(YearRange {
            start: 2010,
            end: 2014,
        });
        // 10000 over 5 remaining years
        assert_approx_eq!(
            f64,
            WithdrawalOptions::EndAtZero.value(
                0_f64,
                2010,
                &settings,
                range,
                &table,
                &totals,
                TaxStatus::ContributeTaxedEarningsUntaxedWhenUsed
            ),
            2000_f64
        );
    }

    #[test]
    fn withdrawal_cost_of_living() {
        let settings = test_settings_values();
        let table = value_table(&[(2009, 500.0), (2010, 10000.0)]);
        let mut totals = YearlyTotals::new();
        totals.add_year(2009, false).unwrap();
        totals.set_saving(2009, 1000.0);
        totals.add_year(2010, false).unwrap();
        totals.update(
            2010,
            YearlyImpact {
                col: 1000.0,
                ..Default::default()
            },
        );

        // col * (prev value / prev savings) = 1000 * 0.5 = 500 (untaxed)
        assert_approx_eq!(
            f64,
            WithdrawalOptions::ColFracOfSavings.value(
                0_f64,
                2010,
                &settings,
                None,
                &table,
                &totals,
                TaxStatus::ContributeTaxedEarningsUntaxedWhenUsed
            ),
            500_f64
        );
        // grossed up for 20% income tax: 500 / 0.8 = 625
        assert_approx_eq!(
            f64,
            WithdrawalOptions::ColFracOfSavings.value(
                0_f64,
                2010,
                &settings,
                None,
                &table,
                &totals,
                TaxStatus::ContributePretaxTaxedWhenUsed
            ),
            625_f64
        );
    }

    #[test]
    fn withdrawal_cost_of_living_excludes_hsa_covered_healthcare() {
        // Healthcare already paid by an HSA must not also be funded from savings.
        let settings = test_settings_values();
        let table = value_table(&[(2009, 500.0), (2010, 10000.0)]);
        let mut totals = YearlyTotals::new();
        totals.add_year(2009, false).unwrap();
        totals.set_saving(2009, 1000.0);
        totals.add_year(2010, false).unwrap();
        // 600 of ordinary cost of living
        totals.update(
            2010,
            YearlyImpact {
                col: 600.0,
                ..Default::default()
            },
        );
        // 400 of healthcare expenses ...
        totals.update(
            2010,
            YearlyImpact {
                healthcare_expense: 400.0,
                col: 400.0,
                ..Default::default()
            },
        );
        // ... fully covered by an HSA withdrawal
        totals.update(
            2010,
            YearlyImpact {
                healthcare_expense: -400.0,
                ..Default::default()
            },
        );

        // effective col = 1000 - 400 covered = 600; withdrawal = 600 * 0.5
        assert_approx_eq!(
            f64,
            WithdrawalOptions::ColFracOfSavings.value(
                0_f64,
                2010,
                &settings,
                None,
                &table,
                &totals,
                TaxStatus::ContributeTaxedEarningsUntaxedWhenUsed
            ),
            300_f64
        );
    }

    #[test]
    fn withdrawal_other() {
        let settings = test_settings_values();
        let table = value_table(&[(2010, 10000.0)]);
        let totals = YearlyTotals::new();
        assert_approx_eq!(
            f64,
            WithdrawalOptions::Other.value(
                500_f64,
                2010,
                &settings,
                None,
                &table,
                &totals,
                TaxStatus::ContributeTaxedEarningsUntaxedWhenUsed
            ),
            0_f64
        );
    }
}
