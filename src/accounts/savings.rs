//! Generic savings account

use serde::{Deserialize, Serialize};
use std::error::Error;

use super::*;

/// Generic savings account
#[derive(Debug, Clone, Deserialize, Serialize)]
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
    yearly_contribution: f64,
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
            contributions: match other.contributions {
                Some(v) => Some(v.into()),
                None => None,
            },
            earnings: match other.earnings {
                Some(v) => Some(v.into()),
                None => None,
            },
            withdrawals: match other.withdrawals {
                Some(v) => Some(v.into()),
                None => None,
            },
            start_in: other.start_in,
            end_in: other.end_in,
            start_out: other.start_out,
            end_out: other.end_out,
            yearly_contribution: other.yearly_contribution,
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
        years: &Vec<u32>,
        linked_dates: Option<Dates>,
        settings: &Settings,
    ) -> Result<(), Box<dyn Error>> {
        if linked_dates.is_some() {
            return Err(String::from("Linked account dates provided but not used").into());
        }
        let mut output = SavingsTables::new(
            &self.table,
            &self.contributions,
            &None,
            &self.earnings,
            &self.withdrawals,
        );
        years.iter().copied().for_each(|year| {
            output.value.0.entry(year).or_insert(0.0);
            output.contributions.0.entry(year).or_insert(0.0);
            output.earnings.0.entry(year).or_insert(0.0);
            output.withdrawals.0.entry(year).or_insert(0.0);
        });
        self.analysis = output;
        self.dates = Dates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        };
        Ok(())
    }
    // fn get_value(&self, year: u32) -> Option<f64> {
    //     self.analysis
    //         .as_ref()
    //         .unwrap()
    //         .value
    //         .0
    //         .get(&year)
    //         .map(|v| *v)
    // }
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
        let start_in = self.dates.year_in.unwrap().start;
        //let end_out = self.dates.as_ref().unwrap().year_out.unwrap().end;
        let tables = &mut self.analysis;

        let mut result = WorkingValues::default();

        tables.value.pull_value_forward(year);

        // Calculate earnings
        result.earning = tables.value.0[&year] * (self.yearly_return.value(settings) / 100.0); // calculate earnings from interest

        // Add earnings to earnings table
        if let Some(x) = tables.earnings.0.get_mut(&year) {
            *x = result.earning;
        }
        // Increase account value by earnings
        if let Some(x) = tables.value.0.get_mut(&year) {
            *x += result.earning;
        }

        // Calculate contribution
        if self.dates.year_in.unwrap().contains(year) {
            result.contribution = self.contribution_type.value(
                self.yearly_contribution,
                totals.get(year).income,
                year - start_in,
                settings.inflation_base,
            );
        }

        // Add contribution to contribution table
        if let Some(x) = tables.contributions.0.get_mut(&year) {
            *x = result.contribution;
        }
        // Increase account value by contribution
        if let Some(x) = tables.value.0.get_mut(&year) {
            *x += result.contribution;
        }

        // Calculate withdrawal
        if self.dates.year_out.unwrap().contains(year) {
            result.withdrawal = self.withdrawal_type.value(
                self.withdrawal_value,
                settings.inflation_base,
                self.dates,
                year,
                tables.value.0[&year],
                tables.value.0[&(year - 1)],
                totals.get(year - 1).col,
                totals.get(year - 1).saving,
                settings.tax_income,
                self.tax_status,
            );
        }

        // Add withdrawal to withdrawal table and subtract from value tables
        if let Some(x) = tables.withdrawals.0.get_mut(&year) {
            *x = result.withdrawal;
        }
        if let Some(x) = tables.value.0.get_mut(&year) {
            *x -= result.withdrawal;
        }

        Ok(YearlyImpact {
            expense: result.contribution,
            col: 0_f64,
            saving: result.contribution + result.earning - result.withdrawal, // delta to savings total for the year
            income_taxable: result.earning,
            income: result.withdrawal,
        })
    }
    fn write(&self, filepath: String) {
        self.analysis.write(filepath);
    }
}
