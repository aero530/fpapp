//! Types of financial accounts
//!
//! Simulate accounts such as income, expense, retirement, 529, loan, mortgage, etc.

use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::inputs::{
    ContributionOptions, EmployerMatch, ExpenseOptions, PaymentOptions, PercentInput, Settings,
    TaxStatus, WithdrawalOptions, YearEvalType, YearInput,
};
use crate::plot::scatter_plot;
use crate::simulation::{
    Dates, LoanTables, SavingsTables, SingleTable, Table, YearRange, YearlyImpact, YearlyTotals,
};

mod college;
mod expense;
mod hsa;
mod income;
mod loan;
mod mortgage;
mod retirement;
mod savings;
mod ssa;

use college::College;
use expense::Expense;
use hsa::Hsa;
use income::Income;
use loan::Loan;
use mortgage::Mortgage;
use retirement::Retirement;
use savings::Savings;
use ssa::Ssa;

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
        linked_dates: Option<Dates>,
        settings: &Settings,
    ) -> Result<Vec<(u32, YearlyImpact)>, Box<dyn Error>>;

    // /// Return the value for the specified year
    fn get_value(&self, year: u32) -> Option<f64>;

    // /// Return the income value for the specified year
    // fn get_income(&self, year: u32) -> Option<f64>;

    // /// Return the expense value for the specified year
    // fn get_expense(&self, year: u32) -> Option<f64>;

    /// Return start_in and end_in
    fn get_range_in(&self, settings: &Settings, linked_dates: Option<Dates>) -> Option<YearRange>;

    /// Return start_out and end_out
    fn get_range_out(&self, settings: &Settings, linked_dates: Option<Dates>) -> Option<YearRange>;

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
            AccountType::Expense,
            AccountType::Hsa, // Expenses must be run before HSA to be able to compute HSA withdrawal amount
            AccountType::Mortgage,
            AccountType::Loan,
            AccountType::College,
            AccountType::Retirement,
            AccountType::Savings,
        ]
    }
}

/// Common result structure used in yearly account simulation
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct WorkingValues {
    /// overall account value
    pub value: f64,
    /// earnings is money that an account gains (ie interest for a savings account or retirement account.  for an income account earnings is the yearly income)
    pub earning: f64,
    /// interest is money that must be payed off (ie for a loan or mortgage)
    pub interest: f64,
    /// contribution is money that goes from income to a savings type account (savings, college, retirement, etc)
    pub contribution: f64,
    /// amount contributed by employer
    pub employer_contribution: f64,
    /// payment is money that must come out of income
    pub payment: f64,
    /// withdrawal is money that may be considered income (dependIng on account type)
    pub withdrawal: f64,
    pub expense: f64,
}

impl WorkingValues {
    /// Limit the withdrawal amount to some value (generally the account value)
    pub fn limit_withdrawal(&mut self, limit: f64) {
        if self.withdrawal > limit {
            self.withdrawal = limit;
        }
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
