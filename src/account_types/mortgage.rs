//! Loan type specifically tailored for mortgages
//! 
use std::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::inputs::{PaymentOptions, PercentInput, YearInput, YearEvalType};
use super::{Account, AccountType, YearRange, AnalysisDates, LoanTables};
use crate::settings::Settings;

/// Loan type specifically tailored for mortgages
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Mortgage {
    name: String,
    table: HashMap<String, f64>,
    start_out: YearInput,
    end_out: YearInput,
    payment_type: PaymentOptions,
    payment_value: f64,
    rate: PercentInput,
    compound_time: f64,
    mortgage_insurance: f64,
    ltv_limit: f64,
    escrow_value: f64,
    home_value: f64,
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    #[serde(skip)]
    analysis: Option<LoanTables>,
    #[serde(skip)]
    dates: Option<AnalysisDates>,
}

impl Account for Mortgage {
    fn type_id(&self) -> AccountType {
        AccountType::Mortgage
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
        let mut output: LoanTables = LoanTables {
            value: self.table.clone(),
            interest: HashMap::new(),
            payments: HashMap::new(),
            escrow: Some(HashMap::new()),
        };
        years.iter().for_each(|year| {
            output.value.entry(year.to_string()).or_insert(0.0);
            output.interest.insert(year.to_string(), 0.0);
            output.payments.insert(year.to_string(), 0.0);
            output.escrow.as_mut().unwrap().insert(year.to_string(), 0.0);
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
    fn get_income(&self, _year: &String) -> Option<f64> {
        None
    }
    fn get_expense(&self, year: &String) -> Option<f64> {
        self.analysis
            .as_ref()
            .unwrap()
            .payments
            .get(year)
            .map(|v| *v)
    }
    fn get_range_in(&self, _settings: &Settings) -> Option<YearRange> {
        None
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


