//! Social Security Account
//!
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use super::inputs::{YearEvalType, YearInput};
use super::{Account, AccountType, AnalysisDates, Table, YearRange, SimResult, YearlyTotal};
use crate::settings::Settings;

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
    fn get_income(&self, year: &String) -> Option<f64> {
        self.get_value(year)
    }
    fn get_expense(&self, _year: &String) -> Option<f64> {
        None
    }
    fn get_range_in(&self, settings: &Settings) -> Option<YearRange> {
        Some(YearRange {
            start: self.start_in.value(settings, None, YearEvalType::StartIn),
            end: self.end_in.value(settings, None, YearEvalType::EndIn),
        })
    }
    fn get_range_out(&self, _settings: &Settings) -> Option<YearRange> {
        None
    }
    fn simulate(&mut self, year: u32, totals: YearlyTotal, settings: &Settings) -> Result<SimResult, Box<dyn Error>> {
        let mut result = SimResult::default();

        Ok(result)
    }
}
