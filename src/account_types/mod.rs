//! Types of financial accounts
//!
//! Simulate accounts such as income, expense, retirement, 529, loan, mortgage, etc.
//!
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::settings::Settings;

mod inputs;

mod expense;
use expense::Expense;

mod income;
use income::Income;

mod ssa;
use ssa::Ssa;

mod college;
use college::College;

mod hsa;
use hsa::Hsa;

mod retirement;
use retirement::Retirement;

mod savings;
use savings::Savings;

mod loan;
use loan::Loan;

mod mortgage;
use mortgage::Mortgage;

/// Running values for income and expenses
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct YearlyTotal {
    // /// money made this year
    // income: f64,
    // /// money spent this year
    // expenses: f64,
    // /// total amount saved
    // savings: f64,
    // /// cost of living
    // col: f64,
    net: f64,
    expense: f64,
    saving: f64,
    income_taxable: f64,
    income: f64,
    income_after_tax: f64,
    income_during_retirement: f64,
}



/// Results of an account simulation that impacts overall running totals
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct SimResult {
    /// earnings is money that an account gains (ie interest for a savings account or retirement account.  for an income account earnings is the yearly income)
    earning: f64,
    /// interest is money that must be payed off (ie for a loan or mortgage)
    interest: f64,
    /// contribution is money that goes from income to a savings type account (savings, college, retirement, etc)
    contribution: f64, 
    /// set employerMatch to zero
    employer_match: f64,
    /// payment is money that must come out of income
    payment: f64,
    /// withdrawal is money that may be considered income (dependIng on account type)
    withdrawal: f64, 
    expense: f64,
}

/// A single table of account values
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Table {
    value: HashMap<String, f64>,
}

pub trait PullForward: std::fmt::Debug {
    fn most_recent_populated_year(&self) -> u32;

    /// if there was a value in this account last year then pull it forward
    fn pull_value_forward(&mut self, year: u32);
}

/// A set of tables for use with loans and mortgage accounts
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct LoanTables {
    value: HashMap<String, f64>,
    interest: HashMap<String, f64>,
    payments: HashMap<String, f64>,
    escrow: Option<HashMap<String, f64>>,
    insurance: Option<HashMap<String, f64>>,
}

impl PullForward for LoanTables {
    fn most_recent_populated_year(&self) -> u32 {
        *self
            .value
            .iter()
            .filter(|(_k, v)| **v > f64::EPSILON)
            .map(|(k, _v)| k.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .max()
            .unwrap()
    }
    /// if there was a value in this account last year then pull it forward
    fn pull_value_forward(&mut self, year: u32) {
        if self.most_recent_populated_year() == year-1 {
            *(self.value)
                .get_mut(&year.to_string())
                .unwrap() = self.value[&(year-1).to_string()];
        }
    }
}


/// A set of tables for use with savings types of accounts
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SavingsTables {
    value: HashMap<String, f64>,
    contributions: HashMap<String, f64>,
    employer_contributions: Option<HashMap<String, f64>>,
    earnings: HashMap<String, f64>,
    withdrawals: HashMap<String, f64>,
}

impl PullForward for SavingsTables {
    fn most_recent_populated_year(&self) -> u32 {
        *self
            .value
            .iter()
            .filter(|(_k, v)| **v > f64::EPSILON)
            .map(|(k, _v)| k.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .max()
            .unwrap()
    }
    /// if there was a value in this account last year then pull it forward
    fn pull_value_forward(&mut self, year: u32) {
        if self.most_recent_populated_year() == year-1 {
            *(self.value)
                .get_mut(&year.to_string())
                .unwrap() = self.value[&(year-1).to_string()];
        }
    }
}

/// Set of year ranges used for analysis
/// year_in is the time range when the account has positive cashflow
/// year_out is the time range when the account has negative cashflow
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AnalysisDates {
    pub year_in: Option<YearRange>,
    pub year_out: Option<YearRange>,
}

/// Defines a time range with start and end values
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct YearRange {
    pub start: u32,
    pub end: u32,
}

impl YearRange {
    fn contains(self, year: u32) -> bool {
        (year >= self.start) && (year <= self.end)
    }
}

/// Trait used to define what each account type must be able to provide
pub trait Account: std::fmt::Debug {
    /// Return the type of the account
    fn type_id(&self) -> AccountType;

    /// Return the name of the account
    fn name(&self) -> String;

    /// Return link id if the account is linked to another account
    fn link_id(&self) -> Option<String>;

    /// Initialize analysis tables with a value for every year in years.  Fill with
    /// values from user data file first then backfill with 0 for years that do not
    /// have a value in user data.  Also initializes the dates used for analysis.
    fn init(
        &mut self,
        years: &Vec<u32>,
        linked_dates: Option<AnalysisDates>,
        settings: &Settings,
    ) -> Result<(), Box<dyn Error>>;

    /// Return the value for the specified year
    fn get_value(&self, year: &String) -> Option<f64>;

    /// Return the income value for the specified year
    fn get_income(&self, year: &String) -> Option<f64>;

    /// Return the expense value for the specified year
    fn get_expense(&self, year: &String) -> Option<f64>;

    /// Return start_in and end_in
    fn get_range_in(&self, settings: &Settings) -> Option<YearRange>;

    /// Return start_out and end_out
    fn get_range_out(&self, settings: &Settings) -> Option<YearRange>;

    /// Compute the value for a year (this needs to be done in time order)
    ///  year: year to compute values for
    ///  income: total income for that year
    fn simulate(&mut self, year: u32, totals: YearlyTotal, settings: &Settings) -> Result<SimResult, Box<dyn Error>>;
}

/// List of the types of accounts that are available
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub enum AccountType {
    Income,
    Ssa,
    Retirement,
    Hsa,
    College,
    Expense,
    Loan,
    Mortgage,
    Savings,
}

/// Account Wrapper for json data storage
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum AccountWrapper {
    Income(Income),
    Ssa(Ssa),
    Retirement(Retirement),
    Hsa(Hsa),
    College(College),
    Expense(Expense),
    Loan(Loan),
    Mortgage(Mortgage),
    Savings(Savings),
}

impl AccountWrapper {
    pub fn to_account_object(self) -> Box<dyn Account> {
        match self {
            AccountWrapper::Income(account) => Box::new(account),
            AccountWrapper::Ssa(account) => Box::new(account),
            AccountWrapper::Retirement(account) => Box::new(account),
            AccountWrapper::Hsa(account) => Box::new(account),
            AccountWrapper::College(account) => Box::new(account),
            AccountWrapper::Expense(account) => Box::new(account),
            AccountWrapper::Loan(account) => Box::new(account),
            AccountWrapper::Mortgage(account) => Box::new(account),
            AccountWrapper::Savings(account) => Box::new(account),
        }
    }
    pub fn order() -> Vec<AccountType> {
        vec![
            AccountType::Income,
            AccountType::Ssa,
            AccountType::Hsa,
            AccountType::Expense,
            AccountType::Mortgage,
            AccountType::Loan,
            AccountType::College,
            AccountType::Retirement,
            AccountType::Savings,
        ]
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn income_value() {
//         let data = TabularData {
//             metrics: vec!["metric1".into(), "metric2".into(), "metric3".into()],
//             times: vec![1.0, 2.0, 3.0],
//             data: vec![
//                 vec![Some(1.0), Some(3.5), Some(2.3)],
//                 vec![Some(-5.6), Some(2.5), Some(7.9)],
//                 vec![Some(0.5), Some(8.0), Some(2.1)],
//             ],
//         };
//         let metadata = TelemetryInfo {
//             metrics: vec!["metric1".into(), "metric2".into(), "metric3".into()],
//             time_range: Limit { min: 1.0, max: 3.0 },
//         };
//         assert_eq!(TelemetryInfo::from(data), metadata);
//     }

// }
