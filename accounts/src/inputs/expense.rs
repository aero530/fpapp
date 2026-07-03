//! User input expense values

use serde::{Deserialize, Serialize};

use super::{fixed_with_inflation, Settings};

/// used to populate account dropdown for expense type selection
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExpenseOptions {
    /// fixed dollar amount
    Fixed,
    /// fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)
    FixedWithInflation,
}

impl ExpenseOptions {
    /// The expense amount for the year
    pub fn value(&self, expense_value: f64, year: u32, settings: &Settings) -> f64 {
        match self {
            ExpenseOptions::Fixed => expense_value,
            ExpenseOptions::FixedWithInflation => {
                fixed_with_inflation(expense_value, year, settings)
            }
        }
    }
}
