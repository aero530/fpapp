//! Types of financial accounts
//!
//! Simulate accounts such as income, expense, retirement, 529, loan, mortgage, etc.
//!
//use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::settings::Settings;
use crate::analysis_types::{AnalysisDates, AccountResult, YearlyImpact, YearlyTotal, YearRange};

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

pub trait PullForward: std::fmt::Debug {
    /// Return the most recent year in the value table that has a value greater than zero
    fn most_recent_populated_year(&self) -> Option<u32>;

    /// If the most recent non-zero value is in year-1 then set the value table entry for year to the value tables entry from year - 1 
    fn pull_value_forward(&mut self, year: u32);
}

/// A set of tables for use with loans and mortgage accounts
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct LoanTables {
    /// Outstanding loan amount
    value: HashMap<String, f64>,
    /// Interest accrued this year
    interest: HashMap<String, f64>,
    /// Payments made against the loan
    payments: HashMap<String, f64>,
    /// Escrow amount used for mortgage type loans
    escrow: Option<HashMap<String, f64>>,
    /// PMI used for mortgage type loans
    insurance: Option<HashMap<String, f64>>,
}

impl PullForward for LoanTables {
    fn most_recent_populated_year(&self) -> Option<u32> {
        self.value
            .iter()
            .filter(|(_k, v)| **v > f64::EPSILON) // only take years that have a value associated with them
            .map(|(k, _v)| k.parse::<u32>().unwrap()) // pull just the year (we don't need the value anymore)
            .collect::<Vec<u32>>() // put into an
            .iter()
            .copied()
            .max()
    }
    /// if there was a value in this account last year then pull it forward
    fn pull_value_forward(&mut self, year: u32) {
        match self.most_recent_populated_year() {
            Some(recent_year) => {
                if recent_year == year - 1 {
                    *(self.value).get_mut(&year.to_string()).unwrap() =
                        self.value[&(year - 1).to_string()];
                }
            }
            None => {}
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
    fn most_recent_populated_year(&self) -> Option<u32> {
        self.value
            .iter()
            .filter(|(_k, v)| **v > f64::EPSILON)
            .map(|(k, _v)| k.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .copied()
            .max()
    }
    /// if there was a value in this account last year then pull it forward
    fn pull_value_forward(&mut self, year: u32) {
        match self.most_recent_populated_year() {
            Some(recent_year) => {
                if recent_year == year - 1 {
                    *(self.value).get_mut(&year.to_string()).unwrap() =
                        self.value[&(year - 1).to_string()];
                }
            }
            None => {}
        }
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
    fn get_range_in(
        &self,
        settings: &Settings,
        linked_dates: Option<AnalysisDates>,
    ) -> Option<YearRange>;

    /// Return start_out and end_out
    fn get_range_out(
        &self,
        settings: &Settings,
        linked_dates: Option<AnalysisDates>,
    ) -> Option<YearRange>;

    /// Compute the value for a year (this needs to be done in time order)
    ///  year: year to compute values for
    ///  income: total income for that year
    fn simulate(
        &mut self,
        year: u32,
        totals: YearlyTotal,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>>;
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
