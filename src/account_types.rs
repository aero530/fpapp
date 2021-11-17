use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::settings;

/// Options for strings on year inputs
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum YearSuggestion {
    /// Start of simulation
    //#[serde(rename(deserialize="yearStart"))]
    YearStart,
    /// When you plan to retire
    //#[serde(rename="yearRetire")]
    YearRetire,
    /// When you plan to die
    //#[serde(rename="yearDie")]
    YearDie,
    /// Last year of the simulation
    //#[serde(rename="yearEnd")]
    YearEnd,
    /// Pull date from linked account
    IncomeLink,
}

impl YearSuggestion {
    pub fn value(&self, settings : settings::Settings) -> f64 {
        match self {
            Self::YearStart => settings.year_start,
            Self::YearRetire => settings.year_born + settings.age_retire,
            Self::YearDie => settings.year_born + settings.age_die,
            Self::YearEnd => settings.year_born + settings.age_die,
            Self::IncomeLink => {todo!()},
        }
    }
}

/// Struct to hold info about computed year values
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct YearComputation {
    base: YearSuggestion,
    delta: f64,
}

/// These values can be input as constants or as computed values (strings)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum YearInput {
    /// Calculated value based on suggested options
    Calculate(YearComputation),
    /// Suggested values
    Suggested(YearSuggestion),
    /// Constant value
    Constant(f64),
}

impl YearInput {
    pub fn value(&self, settings : settings::Settings) -> f64 {
        match self {
            Self::Calculate(input) => {
                input.base.value(settings) + input.delta
            },
            Self::Suggested(input) => input.value(settings),
            Self::Constant(input) => *input,
        }
    }
}

/// Options for strings on percent inputs
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PercentSuggestions {
    InflationBase,
}

/// These values can be input as constants or as computed values (strings)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum PercentInput {
    /// Calculated value based on suggested options
    Calculate(String),
    /// Constant value
    Constant(f64),
}

// /// Object used to store a table of year vs account value
// #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
// #[serde(rename_all = "camelCase")]
// pub struct DataTable {
//     year: f64,
//     value: f64,
// }



/// used to populate account dropdown for tax status selection
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxStatus {
    /// contribute taxed income - earnings taxed deferred.
    /// payed with taxed income, earnings are tax deferred, withdrawals are not taxed
    ContributeTaxedEarningsTaxDeferred,
    /// contribute taxed income - earings are capital gains.
    /// payed with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed (tax free as long as used for intended purpose)
    ContributeTaxedEarningsTaxed,
    /// not implemented.
    /// NOT IMPLEMENTED ## 2=payed with taxed income, earnings are taxed in year taken out as capital gains, withdrawals are not taxed
    NotImpliemented,
    /// contribute pretax income - taxed as income when used.
    /// payed pretax and taxed in year of use as income
    ContributePretaxTaxedWhenUsed,
    /// contribute pretax income - withdrawal not taxed as income (HSA).
    /// payed pretax and not taxed as income (use with HSA)
    ContributePretaxUntaxedWhenUsed,
}

/// description used to populate account dropdown for contribution type selection
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContributionOptions {
    /// fixed dollar amount
    Fixed,
    /// percent of cost of current living
    PercentOfIncome,
    /// fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)
    FixedWithInflation,
}

/// used to populate account dropdown for expense type selection
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExpenseOptions {
    /// fixed dollar amount
    Fixed,
    /// fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)
    FixedWithInflation,
}

/// used to populate account dropdown for withdrawal type selection
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WithdrawalOptions {
    /// fixed dollar amount
    Fixed,
    /// fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)
    FixedWithInflation,
    /// take money out in equal amounts each year such that the balance at endOut is zero
    EndAtZero,
    /// cost of living fraction of total savings
    /// take out the current cost of living * (this accounts value / total savings)
    ColFracOfSavings,
    /// take out a percent of income in each year
    PercentOfIncome,
}

/// used to populate account dropdown for payment type selection
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentOptions {
    /// fixed dollar amount
    Fixed,
    /// fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)
    FixedWithInflation,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Income {
    name: String,
    base: f64,
    table: HashMap<String, f64>,
    start_in: YearInput,
    end_in: YearInput,
    raise: PercentInput,
    notes: Option<String>
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ssa {
    name: String,
    base: f64,
    start_in: YearInput,
    end_in: YearInput,
    notes: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Retirement {
    name: String,
    table: HashMap<String, f64>,
    start_in: YearInput,
    end_in: YearInput,
    start_out: YearInput,
    end_out: YearInput,
    yearly_contribution: f64,
    contribution_type: ContributionOptions,
    yearly_return: PercentInput,
    withdrawal_type: WithdrawalOptions,
    withdrawal_value: f64,
    tax_status: TaxStatus,
    income_link: Option<String>,
    employer_match: Option<f64>,
    match_limit: Option<f64>,
    notes: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Hsa {
    name: String,
    table: HashMap<String, f64>,
    start_in: YearInput,
    end_in: YearInput,
    start_out: YearInput,
    end_out: YearInput,
    yearly_contribution: f64,
    contribution_type: ContributionOptions,
    employer_contribution: f64,
    yearly_return: PercentInput,
    tax_status: TaxStatus,
    notes: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct College {
    name: String,
    table: HashMap<String, f64>,
    start_in: YearInput,
    end_in: YearInput,
    start_out: YearInput,
    end_out: YearInput,
    yearly_contribution: f64,
    contribution_type: ContributionOptions,
    yearly_return: PercentInput,
    withdrawal_type: WithdrawalOptions,
    withdrawal_value: f64,
    tax_status: TaxStatus,
    notes: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Expense {
    name: String,
    table: HashMap<String, f64>,
    start_out: YearInput,
    end_out: YearInput,
    expense_type: ExpenseOptions,
    expense_value: f64,
    is_healthcare: Option<bool>,
    hsa_link: Option<String>,
    notes: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Loan {
    name: String,
    table: HashMap<String, f64>,
    start_out: YearInput,
    end_out: YearInput,
    payment_type: PaymentOptions,
    payment_value: f64,
    rate: PercentInput,
    notes: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Mortgage {
    name: String,
    table: HashMap<String, f64>,
    start_out: YearInput,
    end_out: YearInput,
    payment_type: PaymentOptions,
    payment_value: f64,
    rate: PercentInput,
    compound_time: f64,
    mortgage_insurance: f64,
    ltv_limit: f64,
    escrow_value: f64,
    home_value: f64,
    notes: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Savings {
    name: String,
    table: HashMap<String, f64>,
    start_in: YearInput,
    end_in: YearInput,
    start_out: YearInput,
    end_out: YearInput,
    yearly_contribution: f64,
    contribution_type: ContributionOptions,
    yearly_return: PercentInput,
    withdrawal_type: WithdrawalOptions,
    withdrawal_value: f64,
    tax_status: TaxStatus,
    notes: Option<String>,
}

// https://www.educative.io/edpresso/what-are-traits-in-rust
//
// http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/traits.html
//
// Look into trait bounds

trait Details {
    fn name(&self) -> String;
}


/// Account types
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", tag="type")]
pub enum Account {
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

impl Details for Income {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl Details for Ssa {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl Details for Retirement {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl Details for Hsa {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl Details for College {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl Details for Expense {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl Details for Loan {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl Details for Mortgage {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl Details for Savings {
    fn name(&self) -> String {
        self.name.clone()
    }
}


impl Account {
    pub fn name(&self) -> String {
        match self {
            Account::Income(account) => account.name(),
            Account::Ssa(account) => account.name(),
            Account::Retirement(account) => account.name(),
            Account::Hsa(account) => account.name(),
            Account::College(account) => account.name(),
            Account::Expense(account) => account.name(),
            Account::Loan(account) => account.name(),
            Account::Mortgage(account) => account.name(),
            Account::Savings(account) => account.name(),
        }
    }
}

// impl Details for Account {
//     fn name(&self) -> String {
//         match self {
//             Account::Income(account) => account.name(),
//             Account::Ssa(account) => account.name(),
//             Account::Retirement(account) => account.name(),
//             Account::Hsa(account) => account.name(),
//             Account::College(account) => account.name(),
//             Account::Expense(account) => account.name(),
//             Account::Loan(account) => account.name(),
//             Account::Mortgage(account) => account.name(),
//             Account::Savings(account) => account.name(),
//         }
//     }
// }

// impl Account {
//     pub fn name(&self) -> String {
//         match self {
//             Account::Income(account) => account.name.clone(),
//             Account::Ssa(account) => account.name.clone(),
//             Account::Retirement(account) => account.name.clone(),
//             Account::Hsa(account) => account.name.clone(),
//             Account::College(account) => account.name.clone(),
//             Account::Expense(account) => account.name.clone(),
//             Account::Loan(account) => account.name.clone(),
//             Account::Mortgage(account) => account.name.clone(),
//             Account::Savings(account) => account.name.clone(),
//         }
//     }
// }



