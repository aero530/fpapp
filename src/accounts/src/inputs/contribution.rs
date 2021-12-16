//! User input contributions & employer matching values

use serde::{Deserialize, Serialize};

use super::PercentInput;
// use super::fixed_with_inflation;

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

/// Employer matching for retirement accounts
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct EmployerMatch {
    /// Percentage that the employer will match (such as 50%)
    pub amount: PercentInput,
    /// Limit of employer's match (such as up to the first 6% that the employee saves)
    pub limit: PercentInput,
}