//! Interpret user input from UI / data files

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod contribution;
mod expense;
mod payment;
mod percent;
mod settings;
mod withdrawal;
mod year;

pub use contribution::*;
pub use expense::*;
pub use payment::*;
pub use percent::*;
pub use settings::*;
pub use withdrawal::*;
pub use year::*;

/// Represents the user data file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData<T> {
    /// The system level configuration
    pub settings: Settings,
    /// The metrics that data will be generated for
    pub accounts: HashMap<String, T>,
}

/// Increase the value by inflation from year_start to year
pub fn fixed_with_inflation(initial_value: f64, year: u32, settings: &Settings) -> f64 {
    let years_elapsed = year.saturating_sub(settings.year_start) as f64;
    initial_value * f64::powf(1_f64 + settings.inflation_base / 100_f64, years_elapsed)
}

/// Shared settings fixture for unit tests across the inputs / account modules
#[cfg(test)]
pub mod test_fixtures {
    use super::settings::{Settings, Span, SsaSettings};

    pub fn test_settings_values() -> Settings {
        Settings {
            age_retire: 50,
            age_die: 100,
            year_born: 1980,
            year_start: 2000,
            inflation_base: 5.0,
            tax_income: 20.0,
            tax_capital_gains: 10.0,
            retirement_cost_of_living: 80.0,
            ssa: SsaSettings {
                breakpoints: Span {
                    low: 30000_f64,
                    high: 40000_f64,
                },
                taxable_income_percentage: Span {
                    low: 50_f64,
                    high: 80_f64,
                },
            },
        }
    }
}
