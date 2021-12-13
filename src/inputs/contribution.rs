//! User input contributions & employer matching values

use serde::{Deserialize, Serialize};

use super::PercentInput;
use super::fixed_with_inflation;

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
            }
            ContributionOptions::PercentOfIncome => {
                // calculate the contribution using the total income for the year
                income * contribution / 100_f64
            }
            ContributionOptions::FixedWithInflation => {
                // increase the value by inflation
                fixed_with_inflation(contribution, inflation, duration)
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn contribution_options() {
        let cont1 = ContributionOptions::Fixed;
        let cont2 = ContributionOptions::PercentOfIncome;
        let cont3 = ContributionOptions::FixedWithInflation;
        assert_approx_eq!(
            f64,
            cont1.value(500_f64, 10000_f64, 10_u32, 10_f64),
            500_f64
        );
        assert_approx_eq!(f64, cont2.value(10_f64, 10000_f64, 1_u32, 10_f64), 1000_f64);
        assert_approx_eq!(
            f64,
            cont3.value(500_f64, 10000_f64, 1_u32, 10_f64),
            550_f64,
            epsilon = 0.001
        );
        assert_approx_eq!(
            f64,
            cont3.value(500_f64, 10000_f64, 10_u32, 10_f64),
            1296.8712,
            epsilon = 0.001
        );
    }
}
