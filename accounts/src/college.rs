//! College savings account (529)
use serde::{Deserialize, Serialize};

use super::*;

/// College savings accounts specifically designed to represent 529 accounts
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct College {
    /// String describing this account
    name: String,
    /// Table of account balance
    table: Table,
    /// Table of contributions to this account
    contributions: Option<Table>,
    /// Table of account earnings
    earnings: Option<Table>,
    /// Table of withdrawals from this account
    withdrawals: Option<Table>,
    /// Calendar year when money starts being added to this account
    start_in: YearInput,
    /// Calendar year when money is no longer added to this account (this value is inclusive)
    end_in: YearInput,
    /// Calendar year when money starts being withdrawn from this account
    start_out: YearInput,
    /// Calendar year when money stops being withdrawn from this account
    end_out: YearInput,
    /// Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage. [in today's dollars]
    contribution_value: f64,
    /// Determines how to interpret yearly_contribution
    contribution_type: ContributionOptions,
    /// Percent interest earned each year
    yearly_return: PercentInput,
    /// Determines how to interpret withdrawal_value
    withdrawal_type: WithdrawalOptions,
    /// How much money should be take out per year (either as a percentage or a fixed dollar amount) [in today's dollars]
    withdrawal_value: f64,
    /// How cashflow in this account is treated for tax purposes
    tax_status: TaxStatus,
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

impl Account for College {
    fn type_id(&self) -> AccountType {
        AccountType::College
    }
    fn link_id(&self) -> Option<String> {
        None
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn init(&mut self, linked_dates: Option<Dates>, settings: &Settings) -> Result<(), Error> {
        if linked_dates.is_some() {
            return Err(Error::config("linked account dates provided but not used"));
        }
        // Fail fast: only the 529-style tax treatment is implemented for
        // college accounts.  Catching this at init (the runner adds the
        // account name as context) beats aborting mid-simulation.
        if self.tax_status != TaxStatus::ContributeTaxedEarningsUntaxedWhenUsed {
            return Err(Error::config(
                "only tax status 'contribute_taxed_earnings_untaxed_when_used' is supported for college accounts",
            ));
        }
        require_non_negative(self.contribution_value, "contribution value")?;
        require_non_negative(self.withdrawal_value, "withdrawal value")?;
        require_rate_above_neg_100(self.yearly_return.value(settings), "yearly return")?;
        self.table.validate_non_negative()?;
        // College balances are excluded from the savings pool, so a
        // fraction-of-savings withdrawal computes this account's share of a
        // pool it is not part of — the amounts would be meaningless.
        if self.withdrawal_type == WithdrawalOptions::ColFracOfSavings {
            log::warn!(
                "college account '{}' uses fraction-of-savings withdrawals, but college balances are not part of the savings pool — the withdrawal amounts will not be meaningful",
                self.name
            );
        }

        // Init the analysis object with values from the stored tables
        self.analysis = SavingsTables::new(
            &self.table,
            &self.contributions,
            &None,
            &self.earnings,
            &self.withdrawals,
        );
        self.dates = Dates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        };
        Ok(())
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
    fn get_value(&self, year: u32) -> Option<f64> {
        self.analysis.value.get(year)
    }
    fn get_plot_data(&self) -> Vec<PlotDataSet> {
        self.analysis.get_plot_data()
    }
    fn simulate(
        &mut self,
        year: u32,
        totals: &YearlyTotals,
        settings: &Settings,
        _linked_value: Option<f64>,
    ) -> Result<YearlyImpact, Error> {
        let year_in = self.dates.require_in()?;
        let year_out = self.dates.require_out()?;

        // Init value table with previous year's value; skip for pre-seeded years (table has the starting balance)
        if self.analysis.value.get(year).is_none() {
            self.analysis.add_year(year, true)?;
        }
        let mut result = WorkingValues::default();

        if self.analysis.value.get(year).unwrap() < 0_f64 {
            return Err(Error::internal("college fund account value is negative"));
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
                totals.get_income(year),
                year,
                settings,
            );
        }

        // Add contribution to contribution and value tables
        self.analysis
            .contributions
            .update(year, result.contribution);
        self.analysis.value.update(year, result.contribution);

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

        // Contribute taxed income: paid with taxed income, earnings are not
        // taxed, withdrawals are not taxed (the only supported status —
        // validated in init).  College balances are not part of the general
        // savings pool: withdrawals pay education costs directly rather than
        // flowing into income, and the runner excludes College balances from
        // the savings total used by ColFracOfSavings.
        Ok(YearlyImpact {
            expense: result.contribution,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::test_fixtures::test_settings_values;

    #[test]
    fn college_rejects_unsupported_tax_status_at_init() {
        // Regression test: a pretax 529 must fail fast at init with a clear
        // message instead of aborting mid-simulation.
        let mut account = College {
            name: "529".into(),
            table: Table::default(),
            contributions: None,
            earnings: None,
            withdrawals: None,
            start_in: YearInput::ConstantInt(2000),
            end_in: YearInput::ConstantInt(2010),
            start_out: YearInput::ConstantInt(2011),
            end_out: YearInput::ConstantInt(2015),
            contribution_value: 100.0,
            contribution_type: ContributionOptions::Fixed,
            yearly_return: PercentInput::ConstantFloat(0.0),
            withdrawal_type: WithdrawalOptions::EndAtZero,
            withdrawal_value: 0.0,
            tax_status: TaxStatus::ContributePretaxTaxedWhenUsed,
            notes: None,
            analysis: SavingsTables::default(),
            dates: Dates::default(),
        };
        let settings = test_settings_values();
        let err = account.init(None, &settings).unwrap_err();
        assert!(err.to_string().contains("college"));
    }
}
