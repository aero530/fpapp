//! Types used during the analysis / simulation

use std::io::Write;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Set of year ranges used for analysis
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
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

pub enum Year {
    Int(u32),
    String(String)
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
pub struct YearlyTotals(pub HashMap<String, YearlyTotal>);

impl YearlyTotals {
    /// Initiate a new object with an empty hashmap
    pub fn new() -> YearlyTotals {
        YearlyTotals(HashMap::new())
    }
    /// Insert a new YearlyTotal at a specified year
    pub fn insert(&mut self, year: String, yearly_total: YearlyTotal) {
        self.0.insert(year, yearly_total);
    }
    /// Write yearly total data to a csv file
    pub fn write_summary(&self, filename: String) {
        let mut years : Vec<u32> = self.0.keys().map(|k| k.parse::<u32>().unwrap()).collect();
        years.sort();

        let mut file = std::fs::File::create(filename).unwrap();
        file.write_all("year, totals.net, totals.saving, totals.expense, totals.income, totals.income_taxable, totals.income_after_tax\n".as_bytes()).unwrap();

        years.iter().for_each(|year| {
            let total = self.0[&year.to_string()];
            file.write_all(format!("{}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}\n", year, total.net, total.saving, total.expense, total.income, total.income_taxable, total.tax_burden).as_bytes()).unwrap();
        });
    }
    /// Get the YearlyTotal for the specified year
    pub fn get(&mut self, year: u32) -> YearlyTotal {
        self.0[&(year).to_string()]
    }
}

/// Defines a time range with start and end values
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
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