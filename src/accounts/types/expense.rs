//! Generic expense account (things you spend money on)
//!
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::super::{
    scatter_plot, Account, AccountResult, AccountType, AnalysisDates, SingleTable, Table,
    YearRange, YearlyImpact, YearlyTotals,
};
use crate::inputs::{ExpenseOptions, YearEvalType, YearInput};
use crate::settings::Settings;

/// Account type to represent generic expense
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Expense<T: std::cmp::Eq + std::hash::Hash + std::cmp::PartialEq + std::cmp::Ord> {
    name: String,
    table: Table<T>,
    start_out: YearInput,
    end_out: YearInput,
    expense_type: ExpenseOptions,
    expense_value: f64,
    is_healthcare: Option<bool>,
    hsa_link: Option<String>,
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    #[serde(skip)]
    analysis: SingleTable,
    #[serde(skip)]
    dates: AnalysisDates,
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
        years: &Vec<u32>,
        linked_dates: Option<AnalysisDates>,
        settings: &Settings,
    ) -> Result<(), Box<dyn Error>> {
        if linked_dates.is_some() {
            return Err(String::from("Linked account dates provided but not used").into());
        }
        // let mut output: SingleTable = SingleTable {
        //     value: HashMap::new(),
        // };
        let mut output = SingleTable::default();
        years.iter().copied().for_each(|year| {
            output.value.0.insert(year, 0.0);
        });
        self.analysis = output;
        self.dates = AnalysisDates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        };
        Ok(())
    }
    // fn get_value(&self, year: u32) -> Option<f64> {
    //     match &self.analysis {
    //         Some(result) => result.value.0.get(&year).map(|v| *v),
    //         None => None,
    //     }
    // }
    fn get_range_in(
        &self,
        _settings: &Settings,
        _linked_dates: Option<AnalysisDates>,
    ) -> Option<YearRange> {
        None
    }
    fn get_range_out(
        &self,
        settings: &Settings,
        linked_dates: Option<AnalysisDates>,
    ) -> Option<YearRange> {
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
        let tables = &mut self.analysis;

        let mut result = AccountResult::default();

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
        if let Some(x) = tables.value.0.get_mut(&year) {
            *x = result.expense;
        }

        Ok(YearlyImpact {
            expense: result.expense,
            col: result.expense,
            saving: 0_f64,
            income_taxable: 0_f64,
            income: 0_f64,
        })
    }
    fn write(&self, filepath: String) {
        self.analysis.write(filepath);
    }
}
