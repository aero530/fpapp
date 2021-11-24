//! Source of income
//!
use std::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::inputs::{PercentInput, YearInput, YearEvalType};
use super::{Account, AccountType, YearRange, AnalysisDates, Table};
use crate::settings::Settings;

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
    fn init(&mut self, years: &Vec<u32>, dates: Option<AnalysisDates>, settings: &Settings) -> Result<(), Box<dyn Error>> {
        if dates.is_some() {
            return Err(String::from("Linked account dates provided but not used").into())
        }
        let mut output: Table = Table {
            value: self.table.clone(),
        };
        years.iter().for_each(|year| {
            output.value.entry(year.to_string()).or_insert(0.0);
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
            end: self.end_in.value(settings, None, YearEvalType::EndIn)
        })
    }
    fn get_range_out(&self, _settings: &Settings) -> Option<YearRange> {
        None
    }
    fn simulate(&mut self, year: u32, settings: &Settings) -> Result<(), Box<dyn Error>> {
        let start = self.dates.as_ref().unwrap().year_in.unwrap().start;
        let end = self.dates.as_ref().unwrap().year_in.unwrap().end;

        if year == start {
            *(&mut self.analysis.as_mut().unwrap().value).get_mut(&year.to_string()).unwrap() = self.base;
        } else if year > start  && year < end {
            let prev_value = self.analysis.as_ref().unwrap().value.get(&(year-1).to_string()).unwrap();
            let increase = self.raise.value(settings)/100.0 + 1.0;
            let value = prev_value * increase;
            // let thing = &mut self.analysis.as_mut().unwrap().value;
            // *thing.entry(year.to_string()).or_insert(value) += value;
            
            //*(&mut self.analysis.as_mut().unwrap().value).entry(year.to_string()).or_insert(value) += value;
            *(&mut self.analysis.as_mut().unwrap().value).get_mut(&year.to_string()).unwrap() = value;
        } else {
            //*self.analysis.unwrap().value.get_mut(&year.to_string()).unwrap() = 0.0;
            *(&mut self.analysis.as_mut().unwrap().value).get_mut(&year.to_string()).unwrap() = 0.0;
        }
        Ok(())
    }
}
