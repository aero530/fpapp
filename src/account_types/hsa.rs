//! Health savings account
//!
use std::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::inputs::{ContributionOptions, PercentInput, TaxStatus, YearInput, YearEvalType};
use super::{Account, AccountType, YearRange, AnalysisDates, SavingsTables};
use crate::settings::Settings;

/// Health Savings Account
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Hsa {
    name: String,
    table: HashMap<String, f64>,
    start_in: YearInput,
    end_in: YearInput,
    start_out: YearInput,
    end_out: YearInput,
    yearly_contribution: f64,
    contribution_type: ContributionOptions,
    employer_contribution: f64,
    yearly_return: PercentInput,
    tax_status: TaxStatus,
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    #[serde(skip)]
    analysis: Option<SavingsTables>,
    #[serde(skip)]
    dates: Option<AnalysisDates>,
}

impl Account for Hsa {
    fn type_id(&self) -> AccountType {
        AccountType::Hsa
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
        let mut output: SavingsTables = SavingsTables {
            value: self.table.clone(),
            contributions: HashMap::new(),
            employer_contributions: Some(HashMap::new()),
            earnings: HashMap::new(),
            withdrawals: HashMap::new(),
        };
        years.iter().for_each(|year| {
            output.value.entry(year.to_string()).or_insert(0.0);
            output.contributions.insert(year.to_string(), 0.0);
            output
                .employer_contributions
                .as_mut()
                .unwrap()
                .insert(year.to_string(), 0.0);
            output.earnings.insert(year.to_string(), 0.0);
            output.withdrawals.insert(year.to_string(), 0.0);
        });
        self.analysis = Some(output);
        self.dates = Some(AnalysisDates {
            year_in: self.get_range_in(settings),
            year_out: self.get_range_out(settings),
        });
        Ok(())
    }
    fn get_value(&self, year: &String) -> Option<f64> {
        self.analysis.as_ref().unwrap().value.get(year).map(|v| *v)
    }
    fn get_income(&self, year: &String) -> Option<f64> {
        self.analysis
            .as_ref()
            .unwrap()
            .withdrawals
            .get(year)
            .map(|v| *v)
    }
    fn get_expense(&self, year: &String) -> Option<f64> {
        self.analysis
            .as_ref()
            .unwrap()
            .contributions
            .get(year)
            .map(|v| *v)
    }
    fn get_range_in(&self, settings: &Settings) -> Option<YearRange> {
        Some(YearRange {
            start: self.start_in.value(settings, None, YearEvalType::StartIn),
            end: self.end_in.value(settings, None, YearEvalType::EndIn)
        })
    }
    fn get_range_out(&self, settings: &Settings) -> Option<YearRange> {
        Some(YearRange {
            start: self.start_out.value(settings, None, YearEvalType::StartOut),
            end: self.end_out.value(settings, None, YearEvalType::EndOut)
        })
    }
    fn simulate(&mut self, year: u32, settings: &Settings) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
