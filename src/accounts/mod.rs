//! Types of financial accounts
//!
//! Simulate accounts such as income, expense, retirement, 529, loan, mortgage, etc.

use serde::{Deserialize, Serialize};

use std::error::Error;

use crate::analysis_types::{AccountResult, AnalysisDates, YearRange, YearlyImpact, YearlyTotals};
use crate::settings::Settings;

mod types;
use types::{College, Expense, Hsa, Income, Loan, Mortgage, Retirement, Savings, Ssa};

pub mod plotting;
use plotting::scatter_plot;

mod tables;
use tables::{LoanTables, SavingsTables, SingleTable, Table};

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

    // /// Return the value for the specified year
    // fn get_value(&self, year: u32) -> Option<f64>;

    // /// Return the income value for the specified year
    // fn get_income(&self, year: u32) -> Option<f64>;

    // /// Return the expense value for the specified year
    // fn get_expense(&self, year: u32) -> Option<f64>;

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
        totals: &YearlyTotals,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>>;

    fn write(&self, filepath: String);

    fn plot(&self, filepath: String);
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
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum AccountWrapper {
    Income(Income<String>),
    Ssa(Ssa),
    Retirement(Retirement<String>),
    Hsa(Hsa<String>),
    College(College<String>),
    Expense(Expense<String>),
    Loan(Loan<String>),
    Mortgage(Mortgage<String>),
    Savings(Savings<String>),
}

impl AccountWrapper {
    pub fn to_account_object(self) -> Box<dyn Account> {
        match self {
            AccountWrapper::Income(account) => Box::new(Income::<u32>::from(account)),
            AccountWrapper::Ssa(account) => Box::new(account),
            AccountWrapper::Retirement(account) => Box::new(Retirement::<u32>::from(account)),
            AccountWrapper::Hsa(account) => Box::new(Hsa::<u32>::from(account)),
            AccountWrapper::College(account) => Box::new(College::<u32>::from(account)),
            AccountWrapper::Expense(account) => Box::new(Expense::<u32>::from(account)),
            AccountWrapper::Loan(account) => Box::new(Loan::<u32>::from(account)),
            AccountWrapper::Mortgage(account) => Box::new(Mortgage::<u32>::from(account)),
            AccountWrapper::Savings(account) => Box::new(Savings::<u32>::from(account)),
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
