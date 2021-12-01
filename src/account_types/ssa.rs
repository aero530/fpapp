//! Social Security Account
//!
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::inputs::{YearEvalType, YearInput};
use crate::settings::Settings;
use super::{Account, AccountType, AnalysisDates, AccountResult, Table, YearRange, YearlyTotal, YearlyImpact};

/// Social Security Account
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ssa {
    name: String,
    base: f64,
    start_in: YearInput,
    end_in: YearInput,
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    #[serde(skip)]
    analysis: Option<Table>,
    #[serde(skip)]
    dates: Option<AnalysisDates>,
}

impl Account for Ssa {
    fn type_id(&self) -> AccountType {
        AccountType::Ssa
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
            value: HashMap::new(),
        };
        years.iter().for_each(|year| {
            output.value.insert(year.to_string(), 0.0);
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
        _year: u32,
        _totals: YearlyTotal,
        _settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let mut _result = AccountResult::default();

        Ok(YearlyImpact {
            expense: 0_f64,
            col: 0_f64,
            saving: 0_f64,
            income_taxable: 0_f64,
            income: 0_f64,
        })
    }
}
