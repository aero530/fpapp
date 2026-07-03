//! Types of financial accounts
//!
//! Simulate accounts such as income, expense, retirement, 529, loan, mortgage, etc.

use serde::{Deserialize, Serialize};

mod error;
pub use error::Error;

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
pub use runner::{AnalysisOutput, run};

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
    fn init(&mut self, linked_dates: Option<Dates>, settings: &Settings) -> Result<(), Error>;

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
    ) -> Result<YearlyImpact, Error>;

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
    pub fn to_account_object(self) -> Result<SimAccount, Error> {
        Ok(match self {
            AccountWrapper::Income(account) => SimAccount::Income(account.try_into()?),
            AccountWrapper::Ssa(account) => SimAccount::Ssa(account),
            AccountWrapper::Retirement(account) => SimAccount::Retirement(account.try_into()?),
            AccountWrapper::Hsa(account) => SimAccount::Hsa(account.try_into()?),
            AccountWrapper::College(account) => SimAccount::College(account.try_into()?),
            AccountWrapper::Expense(account) => SimAccount::Expense(account.try_into()?),
            AccountWrapper::Loan(account) => SimAccount::Loan(account.try_into()?),
            AccountWrapper::Mortgage(account) => SimAccount::Mortgage(account.try_into()?),
            AccountWrapper::Savings(account) => SimAccount::Savings(account.try_into()?),
        })
    }
}

/// A simulation-ready account (year-keyed tables), one variant per account type.
///
/// Enum dispatch instead of `Box<dyn Account>`: no heap indirection, and the
/// set of account types stays closed and exhaustively matchable.
#[derive(Debug)]
pub enum SimAccount {
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

/// Apply an expression to whichever concrete account this SimAccount holds
macro_rules! delegate {
    ($self:expr, $account:ident => $body:expr) => {
        match $self {
            SimAccount::Income($account) => $body,
            SimAccount::Ssa($account) => $body,
            SimAccount::Retirement($account) => $body,
            SimAccount::Hsa($account) => $body,
            SimAccount::College($account) => $body,
            SimAccount::Expense($account) => $body,
            SimAccount::Loan($account) => $body,
            SimAccount::Mortgage($account) => $body,
            SimAccount::Savings($account) => $body,
        }
    };
}

impl Account for SimAccount {
    fn type_id(&self) -> AccountType {
        delegate!(self, a => a.type_id())
    }
    fn name(&self) -> String {
        delegate!(self, a => a.name())
    }
    fn link_id(&self) -> Option<String> {
        delegate!(self, a => a.link_id())
    }
    fn init(&mut self, linked_dates: Option<Dates>, settings: &Settings) -> Result<(), Error> {
        delegate!(self, a => a.init(linked_dates, settings))
    }
    fn get_value(&self, year: u32) -> Option<f64> {
        delegate!(self, a => a.get_value(year))
    }
    fn get_range_in(&self, settings: &Settings, linked_dates: Option<Dates>) -> Option<YearRange> {
        delegate!(self, a => a.get_range_in(settings, linked_dates))
    }
    fn get_range_out(&self, settings: &Settings, linked_dates: Option<Dates>) -> Option<YearRange> {
        delegate!(self, a => a.get_range_out(settings, linked_dates))
    }
    fn simulate(
        &mut self,
        year: u32,
        totals: &YearlyTotals,
        settings: &Settings,
        linked_value: Option<f64>,
    ) -> Result<YearlyImpact, Error> {
        delegate!(self, a => a.simulate(year, totals, settings, linked_value))
    }
    fn get_plot_data(&self) -> Vec<PlotDataSet> {
        delegate!(self, a => a.get_plot_data())
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
        let json = std::fs::read_to_string(path).expect("could not read examples/sample_plan.json");
        let data: UserData<AccountWrapper> = serde_json::from_str(&json)
            .unwrap_or_else(|e| panic!("sample_plan.json failed to deserialize: {e}"));
        assert_eq!(data.accounts.len(), 14);
    }

    #[test]
    fn sample_plan_runs() {
        // End-to-end smoke test: the bundled example must convert and simulate cleanly.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/sample_plan.json");
        let json = std::fs::read_to_string(path).expect("could not read examples/sample_plan.json");
        let data: UserData<AccountWrapper> = serde_json::from_str(&json).unwrap();
        let sim: UserData<SimAccount> = data.try_into().unwrap();
        let (plot_data, totals) = crate::run(sim).unwrap();
        assert_eq!(plot_data.len(), 14);
        assert!(!totals.years().is_empty());
    }
}
