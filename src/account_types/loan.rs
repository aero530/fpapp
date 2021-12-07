//! Generic loan
//!
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::{
    Account, AccountResult, AccountType, AnalysisDates, LoanTables, PullForward, Table, YearRange,
    YearlyImpact, YearlyTotals, scatter_plot,
};
use crate::inputs::{PaymentOptions, PercentInput, YearEvalType, YearInput};
use crate::settings::Settings;

/// Generic loan
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Loan<T: std::cmp::Eq + std::hash::Hash + std::cmp::PartialEq + std::cmp::Ord> {
    name: String,
    table: Table<T>,
    start_out: YearInput,
    end_out: YearInput,
    payment_type: PaymentOptions,
    payment_value: f64,
    rate: PercentInput,
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    #[serde(skip)]
    analysis: Option<LoanTables>,
    #[serde(skip)]
    dates: Option<AnalysisDates>,
}

impl From<Loan<String>> for Loan<u32> {
    fn from(other: Loan<String>) -> Self {
        Self {
            name: other.name,
            table: other.table.into(),
            start_out: other.start_out,
            end_out: other.end_out,
            payment_type: other.payment_type,
            payment_value: other.payment_value,
            rate: other.rate,
            notes: other.notes,
            analysis: other.analysis,
            dates: other.dates,
        }
    }
}

impl Account for Loan<u32> {
    fn type_id(&self) -> AccountType {
        AccountType::Loan
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

        let mut output = LoanTables::new(
            &self.table,
            &Table::default(),
            &Table::default(),
            &None,
            &None,
        );

        years.iter().copied().for_each(|year| {
            output.value.0.entry(year).or_insert(0.0);
            output.interest.0.insert(year, 0.0);
            output.payments.0.insert(year, 0.0);
        });
        self.analysis = Some(output);
        self.dates = Some(AnalysisDates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        });
        Ok(())
    }
    fn get_value(&self, year: u32) -> Option<f64> {
        self.analysis
            .as_ref()
            .unwrap()
            .value
            .0
            .get(&year)
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
    fn plot(&self, filepath: String) {

        scatter_plot(
            filepath, 
            vec![
                ("Balance".into(), &self.analysis.as_ref().unwrap().value),
                ("Interest".into(), &self.analysis.as_ref().unwrap().interest),
                ("Payments".into(), &self.analysis.as_ref().unwrap().payments),
                ],
            self.name()
        );
    }
    fn simulate(
        &mut self,
        year: u32,
        _totals: &YearlyTotals,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let start_out = self.dates.as_ref().unwrap().year_out.unwrap().start;
        let tables = &mut self.analysis.as_mut().unwrap();

        let mut result = AccountResult::default();

        tables.pull_value_forward(year);

        // Calculate interest
        result.interest = tables.value.0[&year] * self.rate.value(settings) / 100_f64;

        // Add interest to interest and value tables
        if let Some(x) = tables.interest.0.get_mut(&year) {
            *x = result.interest;
        }
        if let Some(x) = tables.value.0.get_mut(&year) {
            *x += result.interest;
        }

        // Calculate payment amount
        if self
            .dates
            .as_ref()
            .unwrap()
            .year_out
            .unwrap()
            .contains(year)
        {
            result.payment = self.payment_type.value(
                self.payment_value,
                settings.inflation_base,
                year - start_out,
                *tables.value.0.get(&year).unwrap(),
            );
        }

        // Add payment to payment and value tables
        if let Some(x) = tables.payments.0.get_mut(&year) {
            *x = result.payment;
        }
        if let Some(x) = tables.value.0.get_mut(&year) {
            *x -= result.payment;
            // Limit min value of the loan balance to account for floating point math rounding
            if *x < 0.0001 {
                *x = 0_f64;
            }
        }

        Ok(YearlyImpact {
            expense: result.payment,
            col: 0_f64,
            saving: 0_f64,
            income_taxable: 0_f64,
            income: 0_f64,
        })
    }
    fn write(&self, filepath: String) {
        match &self.analysis {
            Some(results) => results.write(filepath),
            None => {}
        }
    }
}
