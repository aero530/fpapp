//! Generic retirement account type applicable for 401K, Roth IRA, IRA, etc.

use serde::{Deserialize, Serialize};

use super::*;

/// Generic retirement account type applicable for 401K, Roth IRA, IRA, etc.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Retirement<T: std::cmp::Ord> {
    /// String describing this account
    name: String,
    /// Table of account balance
    table: Table<T>,
    /// Table of contributions to this account
    contributions: Option<Table<T>>,
    /// Table of account earnings
    earnings: Option<Table<T>>,
    /// Table of withdrawals from this account
    withdrawals: Option<Table<T>>,
    /// Table of employer contributions to this account [in today's dollars]
    employer_contributions: Option<Table<T>>,
    /// Calendar year when money starts being added to this account
    start_in: YearInput,
    /// Calendar year when money is no longer added to this account (this value is inclusive and is often yearRetire-1)
    end_in: YearInput,
    /// Calendar year when money starts being withdrawn from this account
    start_out: YearInput,
    /// Calendar year when money stops being withdrawn from this account
    end_out: YearInput,
    /// Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage. [in today's dollars]
    contribution_value: f64,
    /// Determines how to interpret the value in yearly_contribution
    contribution_type: ContributionOptions,
    /// Percent interest earned each year
    yearly_return: PercentInput,
    /// Determines how to interpret the value in withdrawal_value
    withdrawal_type: WithdrawalOptions,
    /// How much money should be take out per year (either as a percentage or a fixed dollar amount) [in today's dollars]
    withdrawal_value: f64,
    /// How cashflow in this account is treated for tax purposes
    tax_status: TaxStatus,
    /// Link to income account used with employer contributions and some contribution types
    income_link: Option<String>,
    /// Percent of your contribution that your employer matches
    matching: Option<EmployerMatch>,
    /// General information to store with this account
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    /// Tables used to store simulation results
    #[serde(skip)]
    analysis: SavingsTables,
    /// Calculated date values as a year based on input values
    #[serde(skip)]
    dates: Dates,
}

impl TryFrom<Retirement<String>> for Retirement<u32> {
    type Error = Error;
    fn try_from(other: Retirement<String>) -> Result<Self, Self::Error> {
        Ok(Self {
            name: other.name,
            table: other.table.try_into()?,
            contributions: other.contributions.map(|v| v.try_into()).transpose()?,
            earnings: other.earnings.map(|v| v.try_into()).transpose()?,
            withdrawals: other.withdrawals.map(|v| v.try_into()).transpose()?,
            employer_contributions: other
                .employer_contributions
                .map(|v| v.try_into())
                .transpose()?,
            start_in: other.start_in,
            end_in: other.end_in,
            start_out: other.start_out,
            end_out: other.end_out,
            contribution_value: other.contribution_value,
            contribution_type: other.contribution_type,
            yearly_return: other.yearly_return,
            withdrawal_type: other.withdrawal_type,
            withdrawal_value: other.withdrawal_value,
            tax_status: other.tax_status,
            income_link: other.income_link,
            matching: other.matching,
            notes: other.notes,
            analysis: other.analysis,
            dates: other.dates,
        })
    }
}

impl Account for Retirement<u32> {
    fn type_id(&self) -> AccountType {
        AccountType::Retirement
    }
    fn link_id(&self) -> Option<String> {
        self.income_link.clone()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn init(&mut self, linked_dates: Option<Dates>, settings: &Settings) -> Result<(), Error> {
        self.analysis = SavingsTables::new(
            &self.table,
            &self.contributions,
            &self.employer_contributions,
            &self.earnings,
            &self.withdrawals,
        );
        self.dates = Dates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        };
        Ok(())
    }
    fn get_value(&self, year: u32) -> Option<f64> {
        self.analysis.value.get(year)
    }
    fn get_range_in(&self, settings: &Settings, linked_dates: Option<Dates>) -> Option<YearRange> {
        Some(YearRange {
            start: self
                .start_in
                .value(settings, linked_dates, YearEvalType::StartIn),
            end: self
                .end_in
                .value(settings, linked_dates, YearEvalType::EndIn),
        })
    }
    fn get_range_out(&self, settings: &Settings, linked_dates: Option<Dates>) -> Option<YearRange> {
        Some(YearRange {
            start: self
                .start_out
                .value(settings, linked_dates, YearEvalType::StartOut),
            end: self
                .end_out
                .value(settings, linked_dates, YearEvalType::EndOut),
        })
    }
    fn get_plot_data(&self) -> Vec<PlotDataSet> {
        match &self.matching {
            Some(_) => self.analysis.get_matching_plot_data(),
            None => self.analysis.get_plot_data(),
        }
    }
    fn simulate(
        &mut self,
        year: u32,
        totals: &YearlyTotals,
        settings: &Settings,
        linked_value: Option<f64>,
    ) -> Result<YearlyImpact, Error> {
        let year_in = self.dates.require_in()?;
        let year_out = self.dates.require_out()?;
        let mut result = WorkingValues::default();

        // Init value table with previous year's value; skip for pre-seeded years (table has the starting balance)
        if self.analysis.value.get(year).is_none() {
            self.analysis.add_year(year, true)?;
        }

        if self.analysis.value.get(year).unwrap() < 0_f64 {
            return Err(Error::internal("retirement account value is negative"));
        }

        // Calculate earnings
        result.earning =
            self.analysis.value.get(year).unwrap() * (self.yearly_return.value(settings) / 100.0); // calculate earnings from interest

        // Add earnings to earnings and value tables
        self.analysis.earnings.update(year, result.earning);
        self.analysis.value.update(year, result.earning);

        // Calculate contribution
        if year_in.contains(year) {
            result.contribution = self.contribution_type.value(
                self.contribution_value,
                linked_value.unwrap_or_else(|| totals.get_income(year)),
                year,
                settings,
            );

            if let Some(employer_match) = &self.matching {
                // Employer matching requires a linked income account to compute contribution%.
                // If no link is configured, skip matching for this year rather than aborting.
                let link_income = match linked_value {
                    Some(v) => v,
                    None => {
                        log::warn!(
                            "Account '{}': employer matching configured but no income account is linked — skipping match",
                            self.name
                        );
                        0_f64
                    }
                };

                // Income account absent or inactive this year (e.g. outside its
                // date range): no income means no match
                if link_income > 0_f64 {
                    let contribution_pct = result.contribution / link_income * 100_f64;
                    result.employer_contribution =
                        if contribution_pct >= employer_match.limit.value(settings) {
                            // Employee has contributed at or above the match cap; employer contributes
                            // up to the limit percentage of income.
                            link_income
                                * (employer_match.limit.value(settings) / 100_f64)
                                * (employer_match.amount.value(settings) / 100_f64)
                        } else {
                            // Employee is below the cap; employer matches the full contribution.
                            result.contribution * (employer_match.amount.value(settings) / 100_f64)
                        };
                }
            }
        }

        // Add contribution to contribution and value tables
        self.analysis
            .contributions
            .update(year, result.contribution);
        self.analysis
            .employer_contributions
            .update(year, result.employer_contribution);
        self.analysis
            .value
            .update(year, result.contribution + result.employer_contribution);

        // Calculate withdrawal
        if year_out.contains(year) {
            result.withdrawal = self.withdrawal_type.value(
                self.withdrawal_value,
                year,
                settings,
                self.dates.year_out,
                &self.analysis.value,
                totals,
                self.tax_status,
            );
        }

        // Add withdrawal to withdrawal table and subtract from value tables
        self.analysis.withdrawals.update(year, result.withdrawal);
        self.analysis.value.update(year, -result.withdrawal);

        // Contributions always count as an expense (they are subtracted from
        // net for the year); the tax status determines how contributions,
        // earnings, and withdrawals hit taxable income
        let (income_taxable, capital_gains) = match self.tax_status {
            // Paid with taxed income, earnings are not taxed, withdrawals are not taxed (Roth)
            TaxStatus::ContributeTaxedEarningsUntaxedWhenUsed => (0_f64, 0_f64),
            // Paid with taxed income, earnings are taxed in the year earned as capital gains
            TaxStatus::ContributeTaxedEarningsTaxed => (0_f64, result.earning),
            // Paid with pretax income (a deduction) and taxed in year of use as income (401k/IRA)
            TaxStatus::ContributePretaxTaxedWhenUsed => {
                (result.withdrawal - result.contribution, 0_f64)
            }
            // Paid with pretax income and not taxed as income
            TaxStatus::ContributePretaxUntaxedWhenUsed => (0_f64 - result.contribution, 0_f64),
        };

        Ok(YearlyImpact {
            expense: result.contribution,
            healthcare_expense: 0_f64,
            col: 0_f64,
            income_taxable,
            capital_gains,
            income: result.withdrawal,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::test_fixtures::test_settings_values;
    use float_cmp::assert_approx_eq;

    fn test_account(matching: Option<EmployerMatch>) -> Retirement<u32> {
        Retirement {
            name: "401k".into(),
            table: Table([(2000, 10000.0)].into_iter().collect()),
            contributions: None,
            earnings: None,
            withdrawals: None,
            employer_contributions: None,
            start_in: YearInput::ConstantInt(2000),
            end_in: YearInput::ConstantInt(2029),
            start_out: YearInput::ConstantInt(2030),
            end_out: YearInput::ConstantInt(2080),
            contribution_value: 5.0,
            contribution_type: ContributionOptions::PercentOfIncome,
            yearly_return: PercentInput::ConstantFloat(0.0),
            withdrawal_type: WithdrawalOptions::Other,
            withdrawal_value: 0.0,
            tax_status: TaxStatus::ContributePretaxTaxedWhenUsed,
            income_link: Some("income-uuid".into()),
            matching,
            notes: None,
            analysis: SavingsTables::default(),
            dates: Dates::default(),
        }
    }

    #[test]
    fn employer_match_below_cap_matches_full_contribution() {
        let mut account = test_account(Some(EmployerMatch {
            amount: PercentInput::ConstantFloat(50.0),
            limit: PercentInput::ConstantFloat(6.0),
        }));
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let mut totals = YearlyTotals::new();
        totals.add_year(2000, false).unwrap();
        let impact = account
            .simulate(2000, &totals, &settings, Some(100000.0))
            .unwrap();
        // contribution = 5% of 100k = 5000; below the 6% cap so employer matches 50%
        assert_approx_eq!(f64, impact.expense, 5000.0);
        let employer = account.analysis.employer_contributions.get(2000).unwrap();
        assert_approx_eq!(f64, employer, 2500.0);
        // pretax: contribution is a deduction
        assert_approx_eq!(f64, impact.income_taxable, -5000.0);
    }

    #[test]
    fn employer_match_at_cap_is_limited() {
        let mut account = test_account(Some(EmployerMatch {
            amount: PercentInput::ConstantFloat(50.0),
            limit: PercentInput::ConstantFloat(3.0),
        }));
        account.contribution_value = 10.0; // contribute 10%, above the 3% cap
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let mut totals = YearlyTotals::new();
        totals.add_year(2000, false).unwrap();
        account
            .simulate(2000, &totals, &settings, Some(100000.0))
            .unwrap();
        // employer match = income * 3% * 50% = 1500
        let employer = account.analysis.employer_contributions.get(2000).unwrap();
        assert_approx_eq!(f64, employer, 1500.0);
    }

    #[test]
    fn employer_match_without_income_is_skipped() {
        // Regression test for B8: inactive/absent income means no match (and no
        // divide-by-zero), rather than relying on exact float equality.
        let mut account = test_account(Some(EmployerMatch {
            amount: PercentInput::ConstantFloat(50.0),
            limit: PercentInput::ConstantFloat(6.0),
        }));
        account.contribution_type = ContributionOptions::Fixed;
        account.contribution_value = 1000.0;
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let mut totals = YearlyTotals::new();
        totals.add_year(2000, false).unwrap();
        account.simulate(2000, &totals, &settings, None).unwrap();
        let employer = account.analysis.employer_contributions.get(2000).unwrap();
        assert_approx_eq!(f64, employer, 0.0);
    }

    #[test]
    fn earnings_taxed_as_capital_gains() {
        // Regression test for B4: ContributeTaxedEarningsTaxed earnings are
        // reported as capital gains, not ordinary income.
        let mut account = test_account(None);
        account.tax_status = TaxStatus::ContributeTaxedEarningsTaxed;
        account.yearly_return = PercentInput::ConstantFloat(10.0);
        account.contribution_type = ContributionOptions::Fixed;
        account.contribution_value = 0.0;
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let mut totals = YearlyTotals::new();
        totals.add_year(2000, false).unwrap();
        let impact = account.simulate(2000, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, impact.capital_gains, 1000.0);
        assert_approx_eq!(f64, impact.income_taxable, 0.0);
    }
}
