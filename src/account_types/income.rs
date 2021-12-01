//! Source of income
//!
//use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::inputs::{PercentInput, YearEvalType, YearInput};
use crate::settings::Settings;
use super::{Account, AccountType, AnalysisDates, AccountResult, Table, YearRange, YearlyTotal, YearlyImpact};

/// Account to represent sources of income
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Income {
    name: String,
    base: f64,
    table: HashMap<String, f64>,
    start_in: YearInput,
    end_in: YearInput,
    raise: PercentInput,
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    #[serde(skip)]
    analysis: Option<Table>,
    #[serde(skip)]
    dates: Option<AnalysisDates>,
}

impl Account for Income {
    fn type_id(&self) -> AccountType {
        AccountType::Income
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
        let mut output: Table = Table {
            value: self.table.clone(),
        };
        years.iter().for_each(|year| {
            output.value.entry(year.to_string()).or_insert(0.0);
        });
        self.analysis = Some(output);
        self.dates = Some(AnalysisDates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        });
        Ok(())
    }
    fn get_value(&self, year: &String) -> Option<f64> {
        match &self.analysis {
            Some(result) => result.value.get(year).map(|v| *v),
            None => None,
        }
    }
    fn get_income(&self, year: &String) -> Option<f64> {
        self.get_value(year)
    }
    fn get_expense(&self, _year: &String) -> Option<f64> {
        None
    }
    fn get_range_in(
        &self,
        settings: &Settings,
        linked_dates: Option<AnalysisDates>,
    ) -> Option<YearRange> {
        Some(YearRange {
            start: self
                .start_in
                .value(settings, linked_dates, YearEvalType::StartIn),
            end: self
                .end_in
                .value(settings, linked_dates, YearEvalType::EndIn),
        })
    }
    fn get_range_out(
        &self,
        _settings: &Settings,
        _linked_dates: Option<AnalysisDates>,
    ) -> Option<YearRange> {
        None
    }
    fn simulate(
        &mut self,
        year: u32,
        _totals: YearlyTotal,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let start_in = self.dates.as_ref().unwrap().year_in.unwrap().start;
        let tables = &mut self.analysis.as_mut().unwrap();

        let mut result = AccountResult::default();

        // Calculate earnings
        if self.dates.as_ref().unwrap().year_in.unwrap().contains(year) {
            let raise = self.raise.value(settings) / 100.0 + 1.0;
            result.earning = self.base * f64::powf(raise, (year - start_in) as f64);
        }

        // Add earnings to value tables
        if let Some(x) = tables.value.get_mut(&year.to_string()) {
            *x = result.earning;
        }

        Ok(YearlyImpact {
            expense: 0_f64,
            col: 0_f64,
            saving: 0_f64,
            income_taxable: result.earning,
            income: result.earning,
        })
    }
}
