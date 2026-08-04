//! Generic savings account

use serde::{Deserialize, Serialize};

use super::*;

/// Generic savings account
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Savings {
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

impl Account for Savings {
    fn type_id(&self) -> AccountType {
        AccountType::Savings
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
        require_non_negative(self.contribution_value, "contribution value")?;
        require_non_negative(self.withdrawal_value, "withdrawal value")?;
        require_rate_above_neg_100(self.yearly_return.value(settings), "yearly return")?;
        self.table.validate_non_negative()?;

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
        let mut result = WorkingValues::default();

        // Skip add_year for pre-seeded years; the table value is the starting balance
        if self.analysis.value.get(year).is_none() {
            self.analysis.add_year(year, true)?;
        }

        if self.analysis.value.get(year).unwrap() < 0_f64 {
            return Err(Error::internal("savings account value is negative"));
        }

        // Calculate earnings
        result.earning =
            self.analysis.value.get(year).unwrap() * (self.yearly_return.value(settings) / 100.0); // calculate earnings from interest

        // Add earnings to earnings table & increase account value by earnings
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

        // Add contribution to contribution table & increase account value by contribution
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

        let (income_taxable, capital_gains) = match self.tax_status {
            TaxStatus::ContributeTaxedEarningsUntaxedWhenUsed => (0_f64, 0_f64),
            TaxStatus::ContributeTaxedEarningsTaxed => (0_f64, result.earning),
            TaxStatus::ContributePretaxTaxedWhenUsed => {
                (result.withdrawal - result.contribution, 0_f64)
            }
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

    #[test]
    fn savings_accumulates_earnings_and_contributions() {
        let mut account = Savings {
            name: "Savings".into(),
            table: Table([(2000, 1000.0)].into_iter().collect()),
            contributions: None,
            earnings: None,
            withdrawals: None,
            start_in: YearInput::ConstantInt(2000),
            end_in: YearInput::ConstantInt(2010),
            start_out: YearInput::ConstantInt(2011),
            end_out: YearInput::ConstantInt(2080),
            contribution_value: 100.0,
            contribution_type: ContributionOptions::Fixed,
            yearly_return: PercentInput::ConstantFloat(10.0),
            withdrawal_type: WithdrawalOptions::Other,
            withdrawal_value: 0.0,
            tax_status: TaxStatus::ContributeTaxedEarningsUntaxedWhenUsed,
            notes: None,
            analysis: SavingsTables::default(),
            dates: Dates::default(),
        };
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let mut totals = YearlyTotals::new();
        totals.add_year(2000, false).unwrap();

        let impact = account.simulate(2000, &totals, &settings, None).unwrap();
        // 1000 + 10% earnings + 100 contribution
        assert_approx_eq!(f64, account.get_value(2000).unwrap(), 1200.0);
        assert_approx_eq!(f64, impact.expense, 100.0);
        assert_approx_eq!(f64, impact.income_taxable, 0.0);

        totals.add_year(2001, true).unwrap();
        account.simulate(2001, &totals, &settings, None).unwrap();
        // 1200 + 120 earnings + 100 contribution
        assert_approx_eq!(f64, account.get_value(2001).unwrap(), 1420.0);
    }
}
