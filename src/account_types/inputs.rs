//! Definitions for user input fields
//!
use serde::{Deserialize, Serialize};

use super::Account;
use crate::settings;

/// Options for strings on year inputs
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum YearEvalType {
    StartIn,
    EndIn,
    StartOut,
    EndOut,
}

/// Options for strings on year inputs
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
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
    pub fn value(
        &self,
        settings: &settings::Settings,
        linked_account: Option<Box<dyn Account>>,
        eval_type: YearEvalType,
    ) -> u32 {
        match self {
            Self::YearStart => settings.year_start(),
            Self::YearRetire => settings.year_retire(),
            Self::YearDie => settings.year_die(),
            Self::YearEnd => settings.year_end(),
            Self::IncomeLink => match eval_type {
                YearEvalType::StartIn => {
                    linked_account
                        .as_ref()
                        .unwrap()
                        .get_range_in(settings)
                        .unwrap()
                        .start
                }
                YearEvalType::EndIn => {
                    linked_account
                        .as_ref()
                        .unwrap()
                        .get_range_in(settings)
                        .unwrap()
                        .end
                }
                YearEvalType::StartOut => {
                    linked_account
                        .as_ref()
                        .unwrap()
                        .get_range_out(settings)
                        .unwrap()
                        .start
                }
                YearEvalType::EndOut => {
                    linked_account
                        .as_ref()
                        .unwrap()
                        .get_range_out(settings)
                        .unwrap()
                        .end
                }
            },
        }
    }
}

/// Struct to hold info about computed year values
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct YearComputation {
    base: YearSuggestion,
    delta: i32,
}

/// These values can be input as constants or as computed values (strings)
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum YearInput {
    /// Calculated value based on suggested options
    Calculate(YearComputation),
    /// Suggested values
    Suggested(YearSuggestion),
    /// Constant value
    ConstantInt(u32),
}

impl YearInput {
    pub fn value(
        &self,
        settings: &settings::Settings,
        linked_account: Option<Box<dyn Account>>,
        eval_type: YearEvalType,
    ) -> u32 {
        match self {
            Self::Calculate(input) => {
                (input.base.value(settings, linked_account, eval_type) as i32 + input.delta) as u32
            }
            Self::Suggested(input) => input.value(settings, linked_account, eval_type),
            Self::ConstantInt(input) => *input,
        }
    }
}

/// Options for strings on percent inputs
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PercentSuggestions {
    InflationBase,
}

impl PercentSuggestions {
    pub fn value(&self, settings: &settings::Settings) -> f64 {
        match self {
            Self::InflationBase => settings.inflation_base,
        }
    }
}

/// These values can be input as constants or as computed values (strings)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum PercentInput {
    /// Calculated value based on suggested options
    Calculate(PercentSuggestions),
    /// Constant value
    ConstantFloat(f64),
    /// Constant string
    ConstantString(String),
}

impl PercentInput {
    pub fn value(&self, settings: &settings::Settings) -> f64 {
        match self {
            Self::Calculate(input) => input.value(settings),
            Self::ConstantFloat(input) => *input,
            Self::ConstantString(input) => input.parse().unwrap(),
        }
    }
}

/// used to populate account dropdown for tax status selection
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxStatus {
    /// contribute taxed income - earnings taxed deferred.
    /// payed with taxed income, earnings are tax deferred, withdrawals are not taxed
    ContributeTaxedEarningsTaxDeferred,
    /// contribute taxed income - earnings are capital gains.
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
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContributionOptions {
    /// fixed dollar amount
    Fixed,
    /// percent of income
    PercentOfIncome,
    /// fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)
    FixedWithInflation,
}

impl ContributionOptions {
    pub fn value(self, contribution: f64, income: f64, duration: u32, inflation: f64) -> f64 {
        match self {
            ContributionOptions::Fixed => {
                // set the contribution amount to the value input
                contribution
            },
            ContributionOptions::PercentOfIncome => {
                // calculate the contribution using the total income for the year
                income * contribution / 100_f64
            },
            ContributionOptions::FixedWithInflation => {
                // increase the value by inflation
                contribution * f64::powf(1_f64 + inflation / 100_f64, duration as f64)
            },
        }
    }
}


/// used to populate account dropdown for expense type selection
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExpenseOptions {
    /// fixed dollar amount
    Fixed,
    /// fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)
    FixedWithInflation,
}

/// used to populate account dropdown for withdrawal type selection
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
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
}

/// used to populate account dropdown for payment type selection
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentOptions {
    /// fixed dollar amount
    Fixed,
    /// fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)
    FixedWithInflation,
}

impl PaymentOptions {

    pub fn value(self, payment: f64, inflation: f64, duration: u32) -> f64 {
        match self {
            PaymentOptions::Fixed => {
                payment
            },
            PaymentOptions::FixedWithInflation => {
                payment * f64::powf(1_f64 + inflation / 100_f64, duration as f64)
            },
        }
    }
    
}