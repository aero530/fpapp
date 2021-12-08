//! Types used during the analysis / simulation

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::io::Write;

// use log::error;
use crate::accounts::plotting::scatter_plot;

/// Set of year ranges used for analysis
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct AnalysisDates {
    /// Time range when the account has positive cashflow
    pub year_in: Option<YearRange>,
    /// Time range when the account has negative cashflow
    pub year_out: Option<YearRange>,
}

/// Common result structure used in yearly account simulation
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct AccountResult {
    /// earnings is money that an account gains (ie interest for a savings account or retirement account.  for an income account earnings is the yearly income)
    pub earning: f64,
    /// interest is money that must be payed off (ie for a loan or mortgage)
    pub interest: f64,
    /// contribution is money that goes from income to a savings type account (savings, college, retirement, etc)
    pub contribution: f64,
    /// set employerMatch to zero
    pub employer_match: f64,
    /// payment is money that must come out of income
    pub payment: f64,
    /// withdrawal is money that may be considered income (dependIng on account type)
    pub withdrawal: f64,
    pub expense: f64,
}

/// Results of the simulation of an account that impact the YearlyTotal
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct YearlyImpact {
    /// Expenses get pulled out of net (dollars we already paid tax on)
    pub expense: f64,
    /// Cost of Living - tracks to total of the 'expense' account type
    pub col: f64,
    pub saving: f64,
    /// Taxable income
    pub income_taxable: f64,
    /// Total income (taxable + non-taxable)
    pub income: f64,
}

/// Running totals within a single year
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct YearlyTotal {
    pub net: f64,
    pub expense: f64,
    pub col: f64,
    pub saving: f64,
    pub income_taxable: f64,
    pub income: f64,
    pub tax_burden: f64,
    pub income_during_retirement: f64,
}

impl YearlyTotal {
    /// Increment (add to) the yearly total values with the yearly impact results
    ///
    /// This function does not update self.net, self.saving, self.tax_burden, or self.income_during_retirement.
    /// Self.net is set with set_net and modified by pay_income_tax_from_net, deposit_income_in_net, and pay_expenses_from_net
    /// Self.saving is set with set_savings
    /// Self.tax_burden is set with pay_income_tax_from_net
    pub fn update(&mut self, update: YearlyImpact) {
        self.expense += update.expense;
        self.col += update.col;
        self.saving += update.saving;
        self.income_taxable += update.income_taxable;
        self.income += update.income;
    }
    /// Set (define) the net value
    pub fn set_net(&mut self, update: f64) {
        self.net = update;
    }
    /// Set (define) the savings value
    pub fn set_savings(&mut self, update: f64) {
        self.saving = update;
    }
    /// Calculate tax burden and remove from self.net (pay taxes)
    pub fn pay_income_tax_from_net(&mut self, tax_rate: f64) {
        // log what income was after paying taxes
        // println!("{}", tax_rate);
        self.tax_burden = self.income_taxable * (tax_rate / 100_f64);
        // take income tax payment out of net
        self.net -= self.tax_burden;
    }
    /// Add self.income to self.net
    pub fn deposit_income_in_net(&mut self) {
        self.net += self.income;
    }
    /// Remove self.expenses from self.net
    pub fn pay_expenses_from_net(&mut self) {
        self.net -= self.expense;
    }
}

/// YearlyTotals tracked over multiple years
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct YearlyTotals(pub BTreeMap<u32, YearlyTotal>);

impl YearlyTotals {
    /// Initiate a new object with an empty hashmap
    pub fn new() -> YearlyTotals {
        YearlyTotals(BTreeMap::new())
    }
    /// Initialize a new year, pulling forward net & savings if they exist in the previous year
    pub fn init(&mut self, year: u32) {
        let prev_net = match self.0.get(&(year - 1)) {
            Some(v) => v.net,
            None => 0_f64,
        };
        let prev_savings = match self.0.get(&(year - 1)) {
            Some(v) => v.saving,
            None => 0_f64,
        };
        let mut new = YearlyTotal::default();
        new.set_net(prev_net);
        new.set_savings(prev_savings);
        self.0.insert(year, new);
    }
    /// Insert a new YearlyTotal at a specified year
    // pub fn insert(&mut self, year: u32, yearly_total: YearlyTotal) {
    //     self.0.insert(year, yearly_total);
    // }

    /// Update the data for a specified year
    pub fn update(&mut self, year: u32, update: YearlyImpact) {
        match self.0.get_mut(&year) {
            Some(v) => {
                v.update(update);
            }
            None => {
                let mut new = YearlyTotal::default();
                new.update(update);
                self.0.insert(year, new);
            }
        }
    }
    /// Insert a new YearlyTotal at a specified year
    pub fn deposit_income_in_net(&mut self, year: u32) -> Result<(), Box<dyn Error>> {
        match self.0.get_mut(&year) {
            Some(v) => {
                v.deposit_income_in_net();
                Ok(())
            }
            None => {
                return Err(
                    String::from("Unable to deposit income in net.  Year does not exist.").into(),
                );
            }
        }
    }
    /// Insert a new YearlyTotal at a specified year
    pub fn pay_income_tax_from_net(
        &mut self,
        year: u32,
        tax_rate: f64,
    ) -> Result<(), Box<dyn Error>> {
        match self.0.get_mut(&year) {
            Some(v) => {
                v.pay_income_tax_from_net(tax_rate);
                Ok(())
            }
            None => {
                return Err(String::from(
                    "Unable to pay income tax from net.  Year does not exist.",
                )
                .into());
            }
        }
    }
    /// Insert a new YearlyTotal at a specified year
    pub fn pay_expenses_from_net(&mut self, year: u32) -> Result<(), Box<dyn Error>> {
        match self.0.get_mut(&year) {
            Some(v) => {
                v.pay_expenses_from_net();
                Ok(())
            }
            None => {
                return Err(
                    String::from("Unable to pay expenses from net.  Year does not exist.").into(),
                );
            }
        }
    }
    /// Write yearly total data to a csv file
    pub fn write_summary(&self, filename: String) {
        let mut file = std::fs::File::create(filename).unwrap();
        file.write_all("year, totals.net, totals.saving, totals.expense, totals.col, totals.income, totals.income_taxable, totals.tax_burden\n".as_bytes()).unwrap();

        self.get_years().iter().for_each(|year| {
            let total = self.0[year];
            file.write_all(
                format!(
                    "{}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}\n",
                    year,
                    total.net,
                    total.saving,
                    total.expense,
                    total.col,
                    total.income,
                    total.income_taxable,
                    total.tax_burden
                )
                .as_bytes(),
            )
            .unwrap();
        });
    }
    /// Generate plot
    pub fn plot(&self, filepath: String) {
        let net: Vec<f64> = self.0.values().map(|v| v.net).collect();
        let saving: Vec<f64> = self.0.values().map(|v| v.saving).collect();
        let expense: Vec<f64> = self.0.values().map(|v| v.expense).collect();
        let col: Vec<f64> = self.0.values().map(|v| v.col).collect();
        let income: Vec<f64> = self.0.values().map(|v| v.income).collect();
        let income_taxable: Vec<f64> = self.0.values().map(|v| v.income_taxable).collect();
        let tax_burden: Vec<f64> = self.0.values().map(|v| v.tax_burden).collect();

        scatter_plot(
            filepath,
            vec![
                ("Net".into(), &(self.get_years(), net).into()),
                ("Saving".into(), &(self.get_years(), saving).into()),
                ("Expense".into(), &(self.get_years(), expense).into()),
                ("COL".into(), &(self.get_years(), col).into()),
                ("Income".into(), &(self.get_years(), income).into()),
                (
                    "Taxable Income".into(),
                    &(self.get_years(), income_taxable).into(),
                ),
                ("Tax Burden".into(), &(self.get_years(), tax_burden).into()),
            ],
            "Summary".into(),
        );
    }

    /// Get the YearlyTotal for the specified year
    /// If the year is not found then a default object is returned (containing zeros)
    pub fn get(&self, year: u32) -> YearlyTotal {
        match self.0.get(&year) {
            Some(v) => *v,
            None => YearlyTotal::default(),
        }
    }
    /// Return a sorted list of keys (years)
    pub fn get_years(&self) -> Vec<u32> {
        self.0.keys().map(|k| *k).collect()
    }
}

/// Defines a time range with start and end values
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct YearRange {
    /// Beginning of the time range
    pub start: u32,
    /// End of the time range
    pub end: u32,
}

impl YearRange {
    /// Determine if the specified year is within the time range (inclusive)
    pub fn contains(self, year: u32) -> bool {
        (year >= self.start) && (year <= self.end)
    }
}
