//! Types of financial accounts
//!
//! Simulate accounts such as income, expense, retirement, 529, loan, mortgage, etc.
//!
use std::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// A single table of account values
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Table {
    value: HashMap<String, f64>,
}

/// A set of tables for use with loans and mortgage accounts
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct LoanTables {
    value: HashMap<String, f64>,
    interest: HashMap<String, f64>,
    payments: HashMap<String, f64>,
    escrow: Option<HashMap<String, f64>>,
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

/// Set of year ranges used for analysis
/// year_in is the time range when the account has positive cashflow
/// year_out is the time range when the account has negative cashflow
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AnalysisDates {
    pub year_in: Option<YearRange>,
    pub year_out: Option<YearRange>
}

/// Defines a time range with start and end values
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct YearRange {
    pub start: u32,
    pub end: u32,
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
    fn init(&mut self, years: &Vec<u32>, linked_dates: Option<AnalysisDates>, settings: &Settings) -> Result<(), Box<dyn Error>>;
    
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
    fn simulate(&mut self, year: u32, settings: &Settings) -> Result<(), Box<dyn Error>>;
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
