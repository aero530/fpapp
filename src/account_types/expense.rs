//! Generic expense account (things you spend money on)
//!
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use super::inputs::{ExpenseOptions, YearEvalType, YearInput};
use super::{Account, AccountType, AnalysisDates, Table, YearRange, SimResult, YearlyTotal};
use crate::settings::Settings;

/// Account type to represent generic expense
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Expense {
    name: String,
    table: HashMap<String, f64>,
    start_out: YearInput,
    end_out: YearInput,
    expense_type: ExpenseOptions,
    expense_value: f64,
    is_healthcare: Option<bool>,
    hsa_link: Option<String>,
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    #[serde(skip)]
    analysis: Option<Table>,
    #[serde(skip)]
    dates: Option<AnalysisDates>,
}

impl Account for Expense {
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
        dates: Option<AnalysisDates>,
        settings: &Settings,
    ) -> Result<(), Box<dyn Error>> {
        if dates.is_some() {
            return Err(String::from("Linked account dates provided but not used").into());
        }
        let mut output: Table = Table {
            value: HashMap::new(),
        };
        years.iter().for_each(|year| {
            output.value.insert(year.to_string(), 0.0);
        });
        self.analysis = Some(output);
        self.dates = Some(AnalysisDates {
            year_in: self.get_range_in(settings),
            year_out: self.get_range_out(settings),
        });
        Ok(())
    }
    fn get_value(&self, year: &String) -> Option<f64> {
        match &self.analysis {
            Some(result) => result.value.get(year).map(|v| *v),
            None => None,
        }
    }
    fn get_income(&self, _year: &String) -> Option<f64> {
        None
    }
    fn get_expense(&self, year: &String) -> Option<f64> {
        self.get_value(year)
    }
    fn get_range_in(&self, _settings: &Settings) -> Option<YearRange> {
        None
    }
    fn get_range_out(&self, settings: &Settings) -> Option<YearRange> {
        Some(YearRange {
            start: self.start_out.value(settings, None, YearEvalType::StartOut),
            end: self.end_out.value(settings, None, YearEvalType::EndOut),
        })
    }
    fn simulate(&mut self, year: u32, _totals: YearlyTotal, settings: &Settings) -> Result<SimResult, Box<dyn Error>> {
        let start = self.dates.as_ref().unwrap().year_out.unwrap().start;
        let tables = &mut self.analysis.as_mut().unwrap();

        let mut result = SimResult::default();

        // Calculate expense
        if self.dates.as_ref().unwrap().year_out.unwrap().contains(year) {
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
        if let Some(x) = tables.value.get_mut(&year.to_string()) {
            *x = result.expense;
        }

        Ok(result)
    }
}
