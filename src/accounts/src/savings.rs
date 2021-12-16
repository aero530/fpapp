//! Generic savings account

use serde::{Deserialize, Serialize};
use std::error::Error;

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
    /// Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage.
    contribution_value: f64,
    /// Determines how to interpret the value in yearly_contribution
    contribution_type: ContributionOptions,
    /// Percent interest earned each year
    yearly_return: PercentInput,
    /// Determines how to interpret the value in withdrawal_value
    withdrawal_type: WithdrawalOptions,
    /// How much money should be take out per year (either as a percentage or a fixed dollar amount)
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
    fn plot(&self, filepath: String) {
        scatter_plot(
            filepath,
            vec![
                ("Balance".into(), &self.analysis.value),
                ("Contributions".into(), &self.analysis.contributions),
                ("Earnings".into(), &self.analysis.earnings),
                ("Withdrawals".into(), &self.analysis.withdrawals),
            ],
            self.name(),
        );
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

    #[test]
    fn contribution_options() {
        let mut account = Savings {
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
        };
        let yearly_totals = YearlyTotals::new();
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        
        let year = 2010_u32;

        let c1 = account.get_contribution(year, &yearly_totals, &settings);


        assert_approx_eq!(
                f64,
                c1,
                500_f64
            );

        // let update = account.simulate(year, &yearly_totals, &settings).unwrap();
        // println!("{:?}", account.analysis.value.get(year));
        // println!("{:?}", update);

        // assert_eq!(
        //     account.analysis.value.get(year).unwrap(),
        //     account.expense_value
        // );

        // let cont1 = ContributionOptions::Fixed;
        // let cont2 = ContributionOptions::PercentOfIncome;
        // let cont3 = ContributionOptions::FixedWithInflation;
        // assert_approx_eq!(
        //     f64,
        //     cont1.value(500_f64, 10000_f64, 10_u32, 10_f64),
        //     500_f64
        // );
        // assert_approx_eq!(f64, cont2.value(10_f64, 10000_f64, 1_u32, 10_f64), 1000_f64);
        // assert_approx_eq!(
        //     f64,
        //     cont3.value(500_f64, 10000_f64, 1_u32, 10_f64),
        //     550_f64,
        //     epsilon = 0.001
        // );
        // assert_approx_eq!(
        //     f64,
        //     cont3.value(500_f64, 10000_f64, 10_u32, 10_f64),
        //     1296.8712,
        //     epsilon = 0.001
        // );
    }
}
