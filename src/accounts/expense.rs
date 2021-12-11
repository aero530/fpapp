//! Generic expense account (things you spend money on)

use serde::{Deserialize, Serialize};
use std::error::Error;

use super::*;

/// Account type to represent generic expense
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Expense<T: std::cmp::Ord> {
    /// String describing this account
    name: String,
    /// Table of account expence for each year
    table: Table<T>,
    /// Calendar year when then expense of this account started to have impact
    start_out: YearInput,
    /// Calendar year when then expense of this account no longer has impact
    end_out: YearInput,
    /// Determines how to interpret expense_value
    expense_type: ExpenseOptions,
    /// Yearly cost of the expense
    expense_value: f64,
    /// This expense account is for healthcare costs.  If so it will pull first from HSA accounts.
    is_healthcare: bool,
    /// Link this account to an income source
    hsa_link: Option<String>,
    /// General information to store with this account
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    /// Tables used to store simulation results
    #[serde(skip)]
    analysis: SingleTable,
    /// Calculated date values as a year based on input values
    #[serde(skip)]
    dates: Dates,
}

impl From<Expense<String>> for Expense<u32> {
    fn from(other: Expense<String>) -> Self {
        Self {
            name: other.name,
            table: other.table.into(),
            start_out: other.start_out,
            end_out: other.end_out,
            expense_type: other.expense_type,
            expense_value: other.expense_value,
            is_healthcare: other.is_healthcare,
            hsa_link: other.hsa_link,
            notes: other.notes,
            analysis: other.analysis,
            dates: other.dates,
        }
    }
}

impl Account for Expense<u32> {
    fn type_id(&self) -> AccountType {
        AccountType::Expense
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
        self.analysis = SingleTable::default();
        self.dates = Dates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        };
        Ok(Vec::new())
    }
    fn get_value(&self, year: u32) -> Option<f64> {
        self.analysis.value.get(year)
    }
    fn get_range_in(
        &self,
        _settings: &Settings,
        _linked_dates: Option<Dates>,
    ) -> Option<YearRange> {
        None
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
            vec![("Amount".into(), &self.analysis.value)],
            self.name(),
        );
    }
    fn simulate(
        &mut self,
        year: u32,
        _totals: &YearlyTotals,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let start = self.dates.year_out.unwrap().start;
        let mut result = WorkingValues::default();
        self.analysis.add_year(year, false)?;

        // Calculate expense
        if self.dates.year_out.unwrap().contains(year) {
            // Calculate expense amount for fixed, fixed_with_inflation
            match self.expense_type {
                ExpenseOptions::Fixed => {
                    // if type is a fixed value set expense to the value
                    result.expense = self.expense_value;
                }
                ExpenseOptions::FixedWithInflation => {
                    // if type is a fixed number but should be compensated for with inflation
                    let raise = settings.inflation_base / 100.0 + 1.0;
                    let value = self.expense_value * f64::powf(raise, (year - start) as f64); // set expense to the value multiplied by an increase due to inflation
                    result.expense = value;
                }
            }
        }

        // Update value table with expense value
        self.analysis.value.update(year, result.expense);

        match self.is_healthcare {
            true => Ok(YearlyImpact {
                expense: 0_f64,
                healthcare_expense: result.expense, // positive is outstanding (unpaid) expenses
                col: result.expense,
                saving: 0_f64,
                income_taxable: 0_f64,
                income: 0_f64,
                hsa: 0_f64,
            }),
            false => Ok(YearlyImpact {
                expense: result.expense,
                healthcare_expense: 0_f64,
                col: result.expense,
                saving: 0_f64,
                income_taxable: 0_f64,
                income: 0_f64,
                hsa: 0_f64,
            }),
        }
    }
    fn write(&self, filepath: String) {
        self.analysis.write(filepath);
    }
}
