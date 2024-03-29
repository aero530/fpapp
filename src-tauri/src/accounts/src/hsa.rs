//! Health savings account

use serde::{Deserialize, Serialize};
use std::error::Error;
use ts_rs::TS;
use image::{ImageBuffer, Rgba};

use crate::inputs::fixed_with_inflation;
use account_savings_derive::AccountSavings;

use super::*;

/// Health Savings Account
#[derive(TS, Debug, Clone, Deserialize, Serialize, AccountSavings)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Hsa<T: std::cmp::Ord> {
    /// String describing this account
    name: String,
    /// Table of account balance
    table: Table<T>,
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
    /// Employer contributions to this account as a dollar amount [in today's dollars]
    employer_contribution: f64,
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

impl From<Hsa<String>> for Hsa<u32> {
    fn from(other: Hsa<String>) -> Self {
        Self {
            name: other.name,
            table: other.table.into(),
            start_in: other.start_in,
            end_in: other.end_in,
            start_out: other.start_out,
            end_out: other.end_out,
            contribution_value: other.contribution_value,
            contribution_type: other.contribution_type,
            employer_contribution: other.employer_contribution,
            yearly_return: other.yearly_return,
            tax_status: other.tax_status,
            notes: other.notes,
            analysis: other.analysis,
            dates: other.dates,
            withdrawal_type: WithdrawalOptions::Other,
            withdrawal_value: 0_f64,
        }
    }
}

impl Account for Hsa<u32> {
    fn type_id(&self) -> AccountType {
        AccountType::Hsa
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
        self.analysis = SavingsTables::new(&self.table, &None, &None, &None, &None);
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
                        hsa: *value,
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
                (
                    "Employer Contributions".into(),
                    &self.analysis.employer_contributions,
                ),
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
                (
                    "Employer Contributions".into(),
                    &self.analysis.employer_contributions,
                ),
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
            return Err(String::from("HSA account value is negative.").into());
        }

        // Calculate earnings
        result.earning =
            self.analysis.value.get(year).unwrap() * (self.yearly_return.value(settings) / 100.0); // calculate earnings from interest

        // Add earnings to earnings and value tables
        self.analysis.earnings.update(year, result.earning);
        self.analysis.value.update(year, result.earning);

        // Calculate contribution
        if self.dates.year_in.unwrap().contains(year) {
            result.contribution = self.get_contribution(year, totals, settings);
            result.employer_contribution = fixed_with_inflation(self.employer_contribution, year, settings);
        }

        // Add contribution to contribution and value tables
        self.analysis
            .contributions
            .update(year, result.contribution + result.employer_contribution);
        self.analysis
            .value
            .update(year, result.contribution + result.employer_contribution);

        // Calculate withdrawal based on outstanding healthcare expenses
        // This account is used to cover as much of the healthcare expenses
        // as it can based on current account value.  Any remaining expenses
        // must be taken from net in the main loop of the simulation.
        // Unpaid healthcare_expenses are positive values (>0)
        let healthcare_expense = totals.get_healthcare_expense(year);
        if healthcare_expense < 0_f64 {
            return Err(String::from("Negative healthcare expense.").into());
        }
        if self.dates.year_out.unwrap().contains(year) {
            result.withdrawal = match healthcare_expense < self.analysis.value.get(year).unwrap() {
                true => healthcare_expense,
                false => self.analysis.value.get(year).unwrap(),
            }
        }

        // Add withdrawal to withdrawal table and subtract from value tables
        self.analysis.withdrawals.update(year, result.withdrawal);
        self.analysis.value.update(year, -result.withdrawal);

        Ok(YearlyImpact {
            expense: 0_f64,
            healthcare_expense: -result.withdrawal, // reduce this years healthcare expense by the amount paid for from this account
            col: 0_f64,
            saving: 0_f64,
            income_taxable: 0_f64,
            income: 0_f64,
            hsa: result.contribution + result.employer_contribution + result.earning
                - result.withdrawal,
        })
    }
    fn write(&self, filepath: String) {
        self.analysis.write(filepath);
    }
}
