//! Loan type specifically tailored for mortgages

use serde::{Deserialize, Serialize};
use std::error::Error;
use image::{ImageBuffer, Rgba};

use crate::inputs::fixed_with_inflation;
use account_payment_derive::AccountPayment;

use super::*;

/// Loan type specifically tailored for mortgages
#[derive(Debug, Clone, Deserialize, Serialize, AccountPayment)]
#[serde(rename_all = "camelCase")]
pub struct Mortgage<T: std::cmp::Ord> {
    /// String describing this account
    name: String,
    /// Table of outstanding mortgage balance
    table: Table<T>,
    /// Calendar year when payments to this account start
    start_out: YearInput,
    /// Calendar year when payments to this account stop
    end_out: YearInput,
    /// Determines how to interpret payment_value
    payment_type: PaymentOptions,
    /// How much money should be payed each year (either as a percentage or a fixed dollar amount) [in today's dollars]
    payment_value: f64,
    /// Interest rate on borrowed money. This is an APR this is then compounded based on the compound time setting.  Used for LOAN and MORTGAGE account types.
    rate: PercentInput,
    /// Number of times per year that interest is compounded. (1=yearly, 12=monthly)
    compound_time: f64,
    /// Mortgage insurance payment expressed as a yearly fixed number [in today's dollars]
    mortgage_insurance: f64,
    /// Loan to Value amount when mortgage insurance is no longer pulled from payment.  Since monthly payment does not change over time, after the insurance is done there is more money going to the principal each payment
    ltv_limit: f64,
    /// Amount of money going into escrow every year to pay for property tax.  This number is currently assumed to be constant (ie property taxes do not increase) [in today's dollars]
    escrow_value: f64,
    /// Current value of the home.  This is used to compute loan to value [in today's dollars]
    home_value: f64,
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

impl From<Mortgage<String>> for Mortgage<u32> {
    fn from(other: Mortgage<String>) -> Self {
        Self {
            name: other.name,
            table: other.table.into(),
            start_out: other.start_out,
            end_out: other.end_out,
            payment_type: other.payment_type,
            payment_value: other.payment_value,
            rate: other.rate,
            compound_time: other.compound_time,
            mortgage_insurance: other.mortgage_insurance,
            ltv_limit: other.ltv_limit,
            escrow_value: other.escrow_value,
            home_value: other.home_value,
            notes: other.notes,
            analysis: other.analysis,
            dates: other.dates,
        }
    }
}

impl Account for Mortgage<u32> {
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
    fn get_inputs(&self) -> String {
        String::from("Hello")
    }
    fn plot_to_file(&self, filepath: String, width: u32, height: u32) {
        scatter_plot_file(
            filepath,
            vec![
                ("Balance".into(), &self.analysis.value),
                ("Interest".into(), &self.analysis.interest),
                ("Payments".into(), &self.analysis.payments),
                ("Escrow".into(), &self.analysis.escrow),
                ("Insurance".into(), &self.analysis.insurance),
            ],
            self.name(),
            width,
            height,
        );
    }
    fn plot_to_buf(&self, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        scatter_plot_buf(
            vec![
                ("Balance".into(), &self.analysis.value),
                ("Interest".into(), &self.analysis.interest),
                ("Payments".into(), &self.analysis.payments),
                ("Escrow".into(), &self.analysis.escrow),
                ("Insurance".into(), &self.analysis.insurance),
            ],
            self.name(),
            width,
            height,
        )
    }
    fn simulate(
        &mut self,
        year: u32,
        _totals: &YearlyTotals,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let mut result = WorkingValues::default();
        self.analysis.add_year(year, true)?;

        if self.analysis.value.get(year).unwrap() < 0_f64 {
            return Err(String::from("Mortgage account value is negative.").into());
        }

        // Calculate insurance
        let loan_to_value = self.analysis.value.get(year).unwrap() / self.home_value * 100_f64;
        let insurance_payment = match loan_to_value > self.ltv_limit {
            true => self.mortgage_insurance,
            false => 0.0,
        };
        // Add insurance to table and pull out of payment
        self.analysis.insurance.update(year, insurance_payment);

        // Calculate escrow
        // Pull escrow out of payment and add to escrow table
        self.analysis.escrow.update(year, self.escrow_value);

        // Calculate interest
        // The formula for compound interest is P (1 + r/n)^(nt)
        //  P is the initial principal balance
        //  r is the interest rate
        //  n is the number of times interest is compounded per time period
        //  t is the number of time periods
        result.interest = self.analysis.value.get(year).unwrap()
            * f64::powf(
                1_f64 + (self.rate.value(settings) / 100_f64) / self.compound_time,
                self.compound_time,
            )
            - self.analysis.value.get(year).unwrap();

        self.analysis.interest.update(year, result.interest);
        self.analysis.value.update(year, result.interest);

        // Calculate payment available
        result.payment = self.get_payment(year, settings);

        // Add payment to payment and value tables
        self.analysis.payments.update(year, result.payment);

        // Calculate how much of the payment will actually go toward the loan (principal & interest)
        let mut remaining_payment = result.payment; // initial amount that is set to be paid to this loan
        remaining_payment -= insurance_payment; // reduced by the insurance costs for the year
        remaining_payment -= self.escrow_value; // reduced by escrow / property taxes for the year

        // Apply remaining payment to loan balance
        self.analysis.value.update(year, -remaining_payment);
        // Limit min value of the loan balance to account for floating point math rounding
        if self.analysis.value.get(year).unwrap() < 0.0001 {
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
