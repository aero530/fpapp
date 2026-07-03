//! Types of financial accounts
//!
//! Simulate accounts such as income, expense, retirement, 529, loan, mortgage, etc.

use serde::{Deserialize, Serialize};
use std::error::Error;
#[cfg(feature = "plotters-backend")]
use image::{ImageBuffer, Rgba};

mod inputs;
use inputs::{
    ContributionOptions, EmployerMatch, ExpenseOptions, PaymentOptions, PercentInput, Settings,
    TaxStatus, WithdrawalOptions, YearEvalType, YearInput,
};
// re-exported for use outside this lib
pub use inputs::UserData;

mod simulation;
use simulation::{LoanTables, SavingsTables, SingleTable, Table, YearRange, YearlyImpact};
// re-exported for use outside this lib
pub use simulation::{Dates, PlotDataSet, TableGroup, YearlyTotals};

#[cfg(feature = "plotters-backend")]
mod plot;
#[cfg(feature = "plotters-backend")]
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

mod runner;
pub use runner::run;

/// Trait used to define what each account type must be able to provide
pub trait Account: std::fmt::Debug {
    /// Return the type of the account
    fn type_id(&self) -> AccountType;

    /// Return the name of the account
    fn name(&self) -> String;

    /// Return link id if the account is linked to another account
    fn link_id(&self) -> Option<String>;

    /// Initialize analysis tables from historical user data and resolve the
    /// year ranges used during simulation.  Must be called before `simulate`.
    fn init(
        &mut self,
        linked_dates: Option<Dates>,
        settings: &Settings,
    ) -> Result<(), Box<dyn Error>>;

    /// Return the value for the specified year
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

    /// Plot the account simulation results & save to a file
    #[cfg(feature = "plotters-backend")]
    fn plot_to_file(&self, filepath: String, width: u32, height: u32);

    /// Plot the account and return it as an image buffer
    #[cfg(feature = "plotters-backend")]
    fn plot_to_buf(&self, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>>;

    /// Get plot data for UI plotting
    fn get_plot_data(&self) -> Vec<PlotDataSet>;
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

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            AccountType::Income => "income",
            AccountType::Ssa => "ssa",
            AccountType::Retirement => "retirement",
            AccountType::Hsa => "hsa",
            AccountType::College => "college",
            AccountType::Expense => "expense",
            AccountType::Loan => "loan",
            AccountType::Mortgage => "mortgage",
            AccountType::Savings => "savings",
        };
        write!(f, "{}", name)
    }
}

impl AccountType {
    /// The order account types are simulated in within a year.
    ///
    /// Ordering constraints:
    /// - Expense must run before Hsa so the HSA withdrawal can cover the
    ///   year's outstanding healthcare expenses.
    /// - Income must run before Retirement/Savings so percent-of-income
    ///   contributions and employer matching see this year's income.
    /// - Ssa runs last so the taxable-benefit calculation sees all other
    ///   income for the year, including retirement/savings withdrawals
    ///   (which dominate income during retirement).
    pub fn order() -> Vec<AccountType> {
        vec![
            AccountType::Income,
            AccountType::Expense,
            AccountType::Hsa,
            AccountType::Mortgage,
            AccountType::Loan,
            AccountType::College,
            AccountType::Retirement,
            AccountType::Savings,
            AccountType::Ssa,
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
    pub fn to_account_object(self) -> Result<Box<dyn Account>, Box<dyn Error>> {
        Ok(match self {
            AccountWrapper::Income(account) => Box::new(Income::<u32>::try_from(account)?),
            AccountWrapper::Ssa(account) => Box::new(account),
            AccountWrapper::Retirement(account) => Box::new(Retirement::<u32>::try_from(account)?),
            AccountWrapper::Hsa(account) => Box::new(Hsa::<u32>::try_from(account)?),
            AccountWrapper::College(account) => Box::new(College::<u32>::try_from(account)?),
            AccountWrapper::Expense(account) => Box::new(Expense::<u32>::try_from(account)?),
            AccountWrapper::Loan(account) => Box::new(Loan::<u32>::try_from(account)?),
            AccountWrapper::Mortgage(account) => Box::new(Mortgage::<u32>::try_from(account)?),
            AccountWrapper::Savings(account) => Box::new(Savings::<u32>::try_from(account)?),
        })
    }
}

/// Common result structure used in yearly account simulation
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct WorkingValues {
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

#[cfg(test)]
mod sample_plan_tests {
    use super::*;
    use crate::inputs::UserData;

    #[test]
    fn sample_plan_deserializes() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/sample_plan.json");
        let json = std::fs::read_to_string(path)
            .expect("could not read examples/sample_plan.json");
        let data: UserData<AccountWrapper> = serde_json::from_str(&json)
            .unwrap_or_else(|e| panic!("sample_plan.json failed to deserialize: {e}"));
        assert_eq!(data.accounts.len(), 14);
    }

    #[test]
    fn sample_plan_runs() {
        // End-to-end smoke test: the bundled example must convert and simulate cleanly.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/sample_plan.json");
        let json = std::fs::read_to_string(path)
            .expect("could not read examples/sample_plan.json");
        let data: UserData<AccountWrapper> = serde_json::from_str(&json).unwrap();
        let boxed: UserData<Box<dyn Account>> = data.try_into().unwrap();
        let (plot_data, totals) = crate::run(boxed).unwrap();
        assert_eq!(plot_data.len(), 14);
        assert!(!totals.years().is_empty());
    }
}
