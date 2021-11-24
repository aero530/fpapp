//! Generic retirement account type applicable for 401K, Roth IRA, IRA, etc.
//!
use std::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::inputs::{ContributionOptions, PercentInput, TaxStatus, WithdrawalOptions, YearInput, YearEvalType};
use super::{Account, AccountType, YearRange, AnalysisDates, SavingsTables};
use crate::settings::Settings;

/// Generic retirement account type applicable for 401K, Roth IRA, IRA, etc.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Retirement {
    name: String,
    table: HashMap<String, f64>,
    contributions: Option<HashMap<String, f64>>,
    earnings: Option<HashMap<String, f64>>,
    withdrawals: Option<HashMap<String, f64>>,
    employer_contributions: Option<HashMap<String, f64>>,
    start_in: YearInput,
    end_in: YearInput,
    start_out: YearInput,
    end_out: YearInput,
    yearly_contribution: f64,
    contribution_type: ContributionOptions,
    yearly_return: PercentInput,
    withdrawal_type: WithdrawalOptions,
    withdrawal_value: f64,
    tax_status: TaxStatus,
    income_link: Option<String>,
    employer_match: Option<f64>,
    match_limit: Option<f64>,
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    #[serde(skip)]
    analysis: Option<SavingsTables>,
    #[serde(skip)]
    dates: Option<AnalysisDates>,
}

impl Account for Retirement {
    fn type_id(&self) -> AccountType {
        AccountType::Retirement
    }
    fn link_id(&self) -> Option<String> {
        self.income_link.clone()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn init(&mut self, years: &Vec<u32>, dates: Option<AnalysisDates>, _settings: &Settings) -> Result<(), Box<dyn Error>> {
        
        let mut output: SavingsTables = SavingsTables {
            value: self.table.clone(),
            contributions: match &self.contributions {
                Some(table) => table.clone(),
                None => HashMap::new(),
            },
            employer_contributions: Some(match &self.employer_contributions {
                Some(table) => table.clone(),
                None => HashMap::new(),
            }),
            earnings: match &self.earnings {
                Some(table) => table.clone(),
                None => HashMap::new(),
            },
            withdrawals: match &self.withdrawals {
                Some(table) => table.clone(),
                None => HashMap::new(),
            },
        };
        years.iter().for_each(|year| {
            output.value.entry(year.to_string()).or_insert(0.0);
            output.contributions.entry(year.to_string()).or_insert(0.0);
            output
                .employer_contributions
                .as_mut()
                .unwrap()
                .entry(year.to_string())
                .or_insert(0.0);
            output.earnings.entry(year.to_string()).or_insert(0.0);
            output.withdrawals.entry(year.to_string()).or_insert(0.0);
        });
        self.analysis = Some(output);
        self.dates = dates;
        Ok(())
    }
    fn get_value(&self, year: &String) -> Option<f64> {
        self.analysis.as_ref().unwrap().value.get(year).map(|v| *v)
    }
    fn get_income(&self, _year: &String) -> Option<f64> {
        None
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
