//! Loan type specifically tailored for mortgages
//!
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::inputs::{PaymentOptions, PercentInput, YearEvalType, YearInput};
use crate::settings::Settings;
use super::{
    Account, AccountType, AnalysisDates, LoanTables, PullForward, AccountResult, YearRange, YearlyTotal, YearlyImpact,
};

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
    fn init(
        &mut self,
        years: &Vec<u32>,
        linked_dates: Option<AnalysisDates>,
        settings: &Settings,
    ) -> Result<(), Box<dyn Error>> {
        if linked_dates.is_some() {
            return Err(String::from("Linked account dates provided but not used").into());
        }
        let mut output: LoanTables = LoanTables {
            value: self.table.clone(),
            interest: HashMap::new(),
            payments: HashMap::new(),
            escrow: Some(HashMap::new()),
            insurance: Some(HashMap::new()),
        };
        years.iter().for_each(|year| {
            output.value.entry(year.to_string()).or_insert(0.0);
            output.interest.insert(year.to_string(), 0.0);
            output.payments.insert(year.to_string(), 0.0);
            output
                .escrow
                .as_mut()
                .unwrap()
                .insert(year.to_string(), 0.0);
            output
                .insurance
                .as_mut()
                .unwrap()
                .insert(year.to_string(), 0.0);
        });
        self.analysis = Some(output);
        self.dates = Some(AnalysisDates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
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
    fn simulate(
        &mut self,
        year: u32,
        _totals: YearlyTotal,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let start_out = self.dates.as_ref().unwrap().year_out.unwrap().start;
        let tables = &mut self.analysis.as_mut().unwrap();

        let mut result = AccountResult::default();

        tables.pull_value_forward(year);

        // Calculate payment available
        result.payment = self.payment_type.value(
            self.payment_value,
            settings.inflation_base,
            year - start_out,
        );
        let mut remaining_payment = result.payment;
        // Add payment to payment and value tables
        if let Some(x) = tables.payments.get_mut(&year.to_string()) {
            *x = remaining_payment;
        }

        // Calculate insurance
        let loan_to_value = tables.value[&year.to_string()] / self.home_value * 100_f64;
        let insurance_payment = match loan_to_value > self.ltv_limit {
            true => self.mortgage_insurance,
            false => 0.0,
        };
        // Add insurance to table and pull out of payment
        remaining_payment -= insurance_payment;
        if let Some(x) = tables
            .insurance
            .as_mut()
            .unwrap()
            .get_mut(&year.to_string())
        {
            *x = insurance_payment;
        }

        // Calculate escrow
        // Pull escrow out of payment and add to escrow table
        remaining_payment -= self.escrow_value;
        if let Some(x) = tables.escrow.as_mut().unwrap().get_mut(&year.to_string()) {
            *x = self.escrow_value;
        }

        // Calculate interest
        result.interest = tables.value[&year.to_string()]
            * f64::powf(
                self.rate.value(settings) / 100_f64 / self.compound_time,
                self.compound_time,
            );
        if let Some(x) = tables.interest.get_mut(&year.to_string()) {
            *x = result.interest;
        }
        if let Some(x) = tables.value.get_mut(&year.to_string()) {
            *x += result.interest;
        }

        // Apply remaining payment to value
        if let Some(x) = tables.value.get_mut(&year.to_string()) {
            *x -= remaining_payment;
        }

        Ok(YearlyImpact {
            expense: result.payment,
            col: 0_f64,
            saving: 0_f64,
            income_taxable: 0_f64,
            income: 0_f64,
        })
    }
}
