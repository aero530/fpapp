//! Generic loan

use serde::{Deserialize, Serialize};
use std::error::Error;

use super::*;

/// Generic loan
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Loan<T: std::cmp::Ord> {
    /// String describing this account
    name: String,
    /// Table of outstanding loan balance
    table: Table<T>,
    /// Calendar year when payments to this account start
    start_out: YearInput,
    /// Calendar year when payments to this account stop
    end_out: YearInput,
    /// Determines how to interpret payment_value
    payment_type: PaymentOptions,
    /// How much money should be payed each year (either as a percentage or a fixed dollar amount)
    payment_value: f64,
    /// Interest rate on borrowed money. This is an APR this is then compounded based on the compound time setting.  Used for LOAN and MORTGAGE account types.
    rate: PercentInput,
    /// General information to store with this account
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    /// Tables used to store simulation results
    #[serde(skip)]
    analysis: LoanTables,
    /// Calculated date values as a year based on input values
    #[serde(skip)]
    dates: Dates,
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
        linked_dates: Option<Dates>,
        settings: &Settings,
    ) -> Result<Vec<(u32, YearlyImpact)>, Box<dyn Error>> {
        if linked_dates.is_some() {
            return Err(String::from("Linked account dates provided but not used").into());
        }

        self.analysis = LoanTables::new(
            &self.table,
            &Table::default(),
            &Table::default(),
            &Table::default(),
            &Table::default(),
        );
        self.dates = Dates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        };
        Ok(Vec::new())
    }
    fn get_value(&self, year: u32) -> Option<f64> {
        self.analysis.value.get(year)
    }
    fn get_range_in(
        &self,
        _settings: &Settings,
        _linked_dates: Option<Dates>,
    ) -> Option<YearRange> {
        None
    }
    fn get_range_out(&self, settings: &Settings, linked_dates: Option<Dates>) -> Option<YearRange> {
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
                ("Balance".into(), &self.analysis.value),
                ("Interest".into(), &self.analysis.interest),
                ("Payments".into(), &self.analysis.payments),
            ],
            self.name(),
        );
    }
    fn simulate(
        &mut self,
        year: u32,
        _totals: &YearlyTotals,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let start_out = self.dates.year_out.unwrap().start;
        let mut result = WorkingValues::default();
        self.analysis.add_year(year, true)?;

        if self.analysis.value.get(year).unwrap() < 0_f64 {
            return Err(String::from("Loan account value is negative.").into());
        }

        // Calculate interest
        result.interest =
            self.analysis.value.get(year).unwrap() * self.rate.value(settings) / 100_f64;

        // Add interest to interest and value tables
        self.analysis.interest.update(year, result.interest);
        self.analysis.value.update(year, result.interest);

        // Calculate payment amount
        if self.dates.year_out.unwrap().contains(year) {
            result.payment = self.payment_type.value(
                self.payment_value,
                settings.inflation_base,
                year - start_out,
                self.analysis.value.get(year).unwrap(),
            );
        }

        // Add payment to payment and value tables
        self.analysis.payments.update(year, result.payment);
        self.analysis.value.update(year, -result.payment);
        // Limit min value of the loan balance to account for floating point math rounding
        if result.payment < 0.0001 {
            self.analysis.value.insert(year, 0_f64);
        }

        Ok(YearlyImpact {
            expense: result.payment,
            healthcare_expense: 0_f64,
            col: 0_f64,
            saving: 0_f64,
            income_taxable: 0_f64,
            income: 0_f64,
            hsa: 0_f64,
        })
    }
    fn write(&self, filepath: String) {
        self.analysis.write(filepath);
    }
}
