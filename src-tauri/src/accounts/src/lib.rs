//! Types of financial accounts
//!
//! Simulate accounts such as income, expense, retirement, 529, loan, mortgage, etc.

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use std::error::Error;
// use ts_rs::TS;
use image::{ImageBuffer, Rgba};

mod inputs;
use inputs::{
    ContributionOptions, EmployerMatch, ExpenseOptions, PaymentOptions, PercentInput, Settings,
    TaxStatus, WithdrawalOptions, YearEvalType, YearInput
};
// re-exported for use outside this lib
pub use inputs::UserData;

mod simulation;
use simulation::{
    LoanTables, SavingsTables, SingleTable, Table, YearRange, YearlyImpact,
};
// re-exported for use outside this lib
pub use simulation::{Dates, YearlyTotals, PlotDataSet};

mod plot;
use plot::{scatter_plot_buf, scatter_plot_file};

mod college;
use college::College;

mod expense;
use expense::Expense;

mod hsa;
use hsa::Hsa;

mod income;
use income::Income;

mod loan;
use loan::Loan;

mod mortgage;
use mortgage::Mortgage;

mod retirement;
use retirement::Retirement;

mod savings;
use savings::Savings;

mod ssa;
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
        linked_value: Option<f64>,
    ) -> Result<YearlyImpact, Box<dyn Error>>;

    /// Save the account simulation results to a csv file
    fn write(&self, filepath: String);

    /// Plot the account simulation results & save to a files
    fn plot_to_file(&self, filepath: String, width: u32, height: u32);

    /// Plot the account and return it as a vec
    fn plot_to_buf(&self, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>>;

    /// Get plot data for UI plotting
    fn get_plot_data(&self) -> Vec<PlotDataSet>;

    /// Return string json of the inputs for the account
    fn get_inputs(&self) -> String;
}

/// Trait for savings accounts of various types that have contributions & withdrawals
pub trait AccountSavings: Account {
    /// Calculate the contribution amount for the specified year
    fn get_contribution(&self, year:u32, totals: &YearlyTotals, settings: &Settings ) -> f64;
    /// Calculate the withdrawal amount for the specified year.  This value is limited by the 
    /// account value for that year (so the account can not become overdrawn).
    fn get_withdrawal(&self, year:u32, totals: &YearlyTotals, settings: &Settings ) -> f64;
}

/// Trait for accounts of various types that have payments (loans)
pub trait AccountPayment: Account {
    /// Calculate the contribution amount for the specified year
    fn get_payment(&self, year:u32, settings: &Settings ) -> f64;
}

/// Trait for accounts of various types that are considered expenses
pub trait AccountExpense: Account {
    /// Calculate the contribution amount for the specified year
    fn get_expense(&self, year:u32, settings: &Settings ) -> f64;
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

/// String representation of the enum value
impl AccountType {
    pub fn to_string(&self) -> String {
        match self {
            AccountType::Income => "income".to_string(),
            AccountType::Ssa => "ssa".to_string(),
            AccountType::Retirement => "retirement".to_string(),
            AccountType::Hsa => "hsa".to_string(),
            AccountType::College => "college".to_string(),
            AccountType::Expense => "expense".to_string(),
            AccountType::Loan => "loan".to_string(),
            AccountType::Mortgage => "mortgage".to_string(),
            AccountType::Savings => "savings".to_string(),
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

/// Clone of Account Wrapper for the UI with number values in the tables
#[derive(TS, Debug, Clone, Deserialize, Serialize)]
#[ts(export)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum AccountWrapperUI {
    Income(Income<u32>),
    Ssa(Ssa),
    Retirement(Retirement<u32>),
    Hsa(Hsa<u32>),
    College(College<u32>),
    Expense(Expense<u32>),
    Loan(Loan<u32>),
    Mortgage(Mortgage<u32>),
    Savings(Savings<u32>),
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