//! Generic savings account

use serde::{Deserialize, Serialize};
use std::error::Error;
use image::{ImageBuffer, Rgba};

use crate::inputs::fixed_with_inflation;
use account_savings_derive::AccountSavings;

use super::*;

/// Generic savings account
#[derive(Debug, Clone, Deserialize, Serialize, AccountSavings)]
#[serde(rename_all = "camelCase")]
pub struct Savings<T: std::cmp::Ord> {
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

impl From<Savings<String>> for Savings<u32> {
    fn from(other: Savings<String>) -> Self {
        Self {
            name: other.name,
            table: other.table.into(),
            contributions: other.contributions.map(|v| v.into()),
            earnings: other.earnings.map(|v| v.into()),
            withdrawals: other.withdrawals.map(|v| v.into()),
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
            notes: other.notes,
            analysis: other.analysis,
            dates: other.dates,
        }
    }
}

impl Account for Savings<u32> {
    fn type_id(&self) -> AccountType {
        AccountType::Savings
    }
    fn link_id(&self) -> Option<String> {
        None
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn init(
        &mut self,
        linked_dates: Option<Dates>,
        settings: &Settings,
    ) -> Result<Vec<(u32, YearlyImpact)>, Box<dyn Error>> {
        if linked_dates.is_some() {
            return Err(String::from("Linked account dates provided but not used").into());
        }

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

        Ok(self
            .table
            .0
            .iter()
            .map(|(year, value)| {
                (
                    *year,
                    YearlyImpact {
                        saving: *value,
                        ..Default::default()
                    },
                )
            })
            .collect())
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
    fn get_inputs(&self) -> String {
        String::from("Hello")
    }
    fn plot_to_file(&self, filepath: String, width: u32, height: u32) {
        scatter_plot_file(
            filepath,
            vec![
                ("Balance".into(), &self.analysis.value),
                ("Contributions".into(), &self.analysis.contributions),
                ("Earnings".into(), &self.analysis.earnings),
                ("Withdrawals".into(), &self.analysis.withdrawals),
            ],
            self.name(),
            width,
            height,
        );
    }
    fn plot_to_buf(&self, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        scatter_plot_buf(
            vec![
                ("Balance".into(), &self.analysis.value),
                ("Contributions".into(), &self.analysis.contributions),
                ("Earnings".into(), &self.analysis.earnings),
                ("Withdrawals".into(), &self.analysis.withdrawals),
            ],
            self.name(),
            width,
            height,
        )
    }
    fn get_plot_data(&self) -> Vec<PlotDataSet> {
        self.analysis.get_plot_data()
    }
    fn simulate(
        &mut self,
        year: u32,
        totals: &YearlyTotals,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let mut result = WorkingValues::default();
        self.analysis.add_year(year, true)?;

        if self.analysis.value.get(year).unwrap() < 0_f64 {
            return Err(String::from("Savings account value is negative.").into());
        }

        // Calculate earnings
        result.earning =
            self.analysis.value.get(year).unwrap() * (self.yearly_return.value(settings) / 100.0); // calculate earnings from interest

        // Add earnings to earnings table & increase account value by earnings
        self.analysis.earnings.update(year, result.earning);
        self.analysis.value.update(year, result.earning);

        // Calculate contribution
        if self.dates.year_in.unwrap().contains(year) {
            result.contribution = self.get_contribution(year, totals, settings);
        }

        // Add contribution to contribution table & increase account value by contribution
        self.analysis
            .contributions
            .update(year, result.contribution);
        self.analysis.value.update(year, result.contribution);

        // Calculate withdrawal
        if self.dates.year_out.unwrap().contains(year) {
            result.withdrawal = self.get_withdrawal(year, &totals, &settings);
        }

        // Add withdrawal to withdrawal table and subtract from value tables
        self.analysis.withdrawals.update(year, result.withdrawal);
        self.analysis.value.update(year, -result.withdrawal);

        Ok(YearlyImpact {
            expense: result.contribution,
            healthcare_expense: 0_f64,
            col: 0_f64,
            saving: result.contribution + result.earning - result.withdrawal, // delta to savings total for the year
            income_taxable: result.earning,
            income: result.withdrawal,
            hsa: 0_f64,
        })
    }
    fn write(&self, filepath: String) {
        self.analysis.write(filepath);
    }
}



#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use crate::inputs::{Settings, Span, SsaSettings};
    use super::*;

    /// Generate settings object for testing
    fn test_settings_values() -> Settings {
        Settings {
            age_retire: 50,
            age_die: 100,
            year_born: 1980,
            year_start: 2000,
            inflation_base: 5.0,
            tax_income: 20.0,
            tax_capital_gains: 10.0,
            retirement_cost_of_living: 80.0,
            ssa: SsaSettings {
                breakpoints: Span {
                    low: 30000_f64,
                    high: 40000_f64,
                },
                taxable_income_percentage: Span {
                    low: 50_f64,
                    high: 80_f64,
                },
            },
        }
    }

    /// Generate account for testing
    fn test_account() -> Savings<u32> {
        Savings {
            name: "Savings Account".into(),
            table: Table::default(),
            start_out: YearInput::ConstantInt(2000),
            end_out: YearInput::ConstantInt(2020),
            notes: None,
            analysis: SavingsTables::default(),
            dates: Dates::default(),
            contributions: Some(Table::default()),
            earnings: Some(Table::default()),
            withdrawals: Some(Table::default()),
            start_in: YearInput::ConstantInt(2000),
            end_in: YearInput::ConstantInt(2020),
            contribution_value: 500_f64,
            contribution_type: ContributionOptions::Fixed,
            yearly_return: PercentInput::ConstantFloat(20_f64),
            withdrawal_type: WithdrawalOptions::Fixed,
            withdrawal_value: 100_f64,
            tax_status: TaxStatus::ContributePretaxTaxedWhenUsed,
        }
    }

    /// Tests get_contribution when type is ContributionOptions::Fixed
    #[test]
    fn contribution_fixed() {
        let settings = test_settings_values();
        let year = 2010_u32;

        let yearly_totals = YearlyTotals::new();
        
        let mut account = test_account();
        account.contribution_type = ContributionOptions::Fixed;
        account.contribution_value = 500_f64;
        account.init(None, &settings).unwrap();
        
        let contribution = account.get_contribution(year, &yearly_totals, &settings);
        assert_approx_eq!(f64, contribution, 500_f64);
    }

    /// Tests get_contribution when type is ContributionOptions::PercentOfIncome
    #[test]
    fn contribution_percent_of_income() {
        let settings = test_settings_values();
        let year = 2010_u32;
        
        let mut yearly_totals = YearlyTotals::new();
        yearly_totals.add_year(year, false).unwrap();
        let mut update = YearlyImpact::default();
        update.income = 10_000_f64;
        yearly_totals.update(year, update);

        let mut account = test_account();
        account.contribution_type = ContributionOptions::PercentOfIncome;
        account.contribution_value = 25_f64;
        account.init(None, &settings).unwrap();
        
        let contribution = account.get_contribution(year, &yearly_totals, &settings);
        assert_approx_eq!(f64, contribution, 2500_f64);
    }

    /// Tests get_contribution when type is ContributionOptions::FixedWithInflation
    #[test]
    fn contribution_fixed_with_inflation() {
        let settings = test_settings_values();
        let year = 2010_u32;
        let yearly_totals = YearlyTotals::new();

        let mut account = test_account();
        account.contribution_type = ContributionOptions::FixedWithInflation;
        account.contribution_value = 500_f64;
        account.init(None, &settings).unwrap();
        
        let contribution = account.get_contribution(year, &yearly_totals, &settings);
        assert_approx_eq!(f64, contribution, 814.447, epsilon=0.001);
    }

    /// Tests get_withdrawal when type is WithdrawalOptions::Fixed
    #[test]
    fn withdrawal_fixed() {
        let settings = test_settings_values();
        let year = 2010_u32;
        let yearly_totals = YearlyTotals::new();

        let mut account = test_account();
        account.withdrawal_type = WithdrawalOptions::Fixed;
        account.withdrawal_value = 500_f64;
        account.init(None, &settings).unwrap();
        account.analysis.add_year(year, false).unwrap();

        // before adding money to the account we should get zero back if we try to calculate
        // a withdrawal (as the account does not have a positive balance)
        let withdrawal = account.get_withdrawal(year, &yearly_totals, &settings);
        assert_approx_eq!(f64, withdrawal, 0_f64, epsilon=0.001);

        // add money to the account so we can withdraw it
        account.analysis.value.update(year, 880_f64); 
        let withdrawal = account.get_withdrawal(year, &yearly_totals, &settings);
        assert_approx_eq!(f64, withdrawal, 500_f64, epsilon=0.001);
    }

    /// Tests get_withdrawal when type is WithdrawalOptions::FixedWithInflation
    #[test]
    fn withdrawal_fixed_with_inflation() {
        let settings = test_settings_values();
        let year = 2010_u32;
        let yearly_totals = YearlyTotals::new();

        let mut account = test_account();
        account.withdrawal_type = WithdrawalOptions::FixedWithInflation;
        account.withdrawal_value = 500_f64;
        account.init(None, &settings).unwrap();
        account.analysis.add_year(year, false).unwrap();

        // add money to the account so we can withdraw it
        account.analysis.value.update(year, 20_000_f64); 
        let withdrawal = account.get_withdrawal(year, &yearly_totals, &settings);
        assert_approx_eq!(f64, withdrawal, 814.447, epsilon=0.001);
    }

    /// Tests get_withdrawal when type is WithdrawalOptions::EndAtZero
    #[test]
    fn withdrawal_end_at_zero() {
        let settings = test_settings_values();
        let year = 2010_u32;
        let yearly_totals = YearlyTotals::new();

        let mut account = test_account();
        account.withdrawal_type = WithdrawalOptions::EndAtZero;
        account.init(None, &settings).unwrap();
        account.analysis.add_year(year, false).unwrap();

        // add money to the account so we can withdraw it
        account.analysis.value.update(year, 880_f64); 
        let withdrawal = account.get_withdrawal(year, &yearly_totals, &settings);
        assert_approx_eq!(f64, withdrawal, 80_f64, epsilon=0.001);
    }

    /// Tests get_withdrawal when type is WithdrawalOptions::ColFracOfSavings
    #[test]
    fn withdrawal_cost_of_living() {
        let settings = test_settings_values();
        let year = 2010_u32;
        
        let mut yearly_totals = YearlyTotals::new();
        yearly_totals.add_year(year-1, false).unwrap();
        yearly_totals.add_year(year, false).unwrap();
        let mut update = YearlyImpact::default();
        update.saving = 40_000_f64;
        update.col = 1_000_f64;
        yearly_totals.update(year-1, update);
        yearly_totals.update(year, update);
        

        let mut account = test_account();
        account.withdrawal_type = WithdrawalOptions::ColFracOfSavings;
        account.init(None, &settings).unwrap();
        account.analysis.add_year(year-1, false).unwrap();
        account.analysis.add_year(year, false).unwrap();
        account.analysis.value.update(year-1, 20_000_f64); 
        account.analysis.value.update(year, 18_000_f64); 

        account.tax_status = TaxStatus::ContributePretaxUntaxedWhenUsed;
        let withdrawal = account.get_withdrawal(year, &yearly_totals, &settings);
        assert_approx_eq!(f64, withdrawal, 500_f64, epsilon=0.001);

        // When withdrawals are going to be taxed we take out extra money to cover those taxes
        account.tax_status = TaxStatus::ContributePretaxTaxedWhenUsed;
        let withdrawal = account.get_withdrawal(year, &yearly_totals, &settings);
        assert_approx_eq!(f64, withdrawal, 625_f64, epsilon=0.001);

    }

    /// Tests get_withdrawal when type is WithdrawalOptions::Other
    #[test]
    fn withdrawal_other() {
        let settings = test_settings_values();
        let year = 2010_u32;
        let yearly_totals = YearlyTotals::new();

        let mut account = test_account();
        account.withdrawal_type = WithdrawalOptions::Other;
        account.withdrawal_value = 500_f64;
        account.init(None, &settings).unwrap();
        account.analysis.add_year(year, false).unwrap();

        // add money to the account so we can withdraw it
        account.analysis.value.update(year, 20_000_f64); 
        let withdrawal = account.get_withdrawal(year, &yearly_totals, &settings);
        assert_approx_eq!(f64, withdrawal, 0_f64, epsilon=0.001);
    }



}
