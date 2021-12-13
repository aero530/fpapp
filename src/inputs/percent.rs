//! User input percent values
//!
//! Some fields are allowed to be input as a percent.  These types parse those
//! fields to return meaningful values based on text, calculate, and contant value inputs.

use serde::{Deserialize, Serialize};

use super::settings;

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

/// Options for strings on percent inputs
///
/// These are smart values that correlate to values previously defined in the application.
/// When selected by the user the correlating value will be returned.  For example,
/// the base inflation rate is defined in the general application settings but can be
/// selected as the value for any field that is a percent such that if you change the base
/// inflation setting then that account's percent value (such as increase in an expense) will
/// change to reflect the newly set value.
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

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    fn test_settings_values() -> settings::Settings {
        settings::Settings {
            age_retire: 50,
            age_die: 100,
            year_born: 1980,
            year_start: 2000,
            inflation_base: 5.0,
            tax_income: 20.0,
            tax_capital_gains: 10.0,
            retirement_cost_of_living: 80.0,
            ssa: settings::SsaSettings {
                breakpoints: settings::Span {
                    low: 30000_f64,
                    high: 40000_f64,
                },
                taxable_income_percentage: settings::Span {
                    low: 50_f64,
                    high: 80_f64,
                },
            },
        }
    }

    #[test]
    fn percent_input() {
        let p1 = PercentInput::Calculate(PercentSuggestions::InflationBase);
        let p2 = PercentInput::ConstantFloat(75_f64);
        let p3 = PercentInput::ConstantString("40".into());

        assert_approx_eq!(f64, p1.value(&test_settings_values()), 5_f64);
        assert_approx_eq!(f64, p2.value(&test_settings_values()), 75_f64);
        assert_approx_eq!(f64, p3.value(&test_settings_values()), 40_f64);
    }
}
