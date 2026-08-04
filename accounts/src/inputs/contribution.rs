//! User input contributions & employer matching values

use serde::{Deserialize, Serialize};

use super::{PercentInput, Settings, fixed_with_inflation};

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
    /// The contribution amount for the year.
    ///
    /// `income` is the income the contribution is based on for percent-of-income
    /// contributions: the linked income account's value when a link exists,
    /// otherwise the total income accumulated so far this year.
    pub fn value(
        &self,
        contribution_value: f64,
        income: f64,
        year: u32,
        settings: &Settings,
    ) -> f64 {
        match self {
            ContributionOptions::Fixed => contribution_value,
            ContributionOptions::PercentOfIncome => income * contribution_value / 100_f64,
            ContributionOptions::FixedWithInflation => {
                fixed_with_inflation(contribution_value, year, settings)
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
    use super::super::test_fixtures::test_settings_values;
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn contribution_fixed() {
        let settings = test_settings_values();
        assert_approx_eq!(
            f64,
            ContributionOptions::Fixed.value(500_f64, 10000_f64, 2010, &settings),
            500_f64
        );
    }

    #[test]
    fn contribution_percent_of_income() {
        let settings = test_settings_values();
        assert_approx_eq!(
            f64,
            ContributionOptions::PercentOfIncome.value(10_f64, 10000_f64, 2010, &settings),
            1000_f64
        );
    }

    #[test]
    fn contribution_fixed_with_inflation() {
        let settings = test_settings_values();
        // 500 * 1.05^10 = 814.447
        assert_approx_eq!(
            f64,
            ContributionOptions::FixedWithInflation.value(500_f64, 10000_f64, 2010, &settings),
            814.447,
            epsilon = 0.001
        );
    }
}
