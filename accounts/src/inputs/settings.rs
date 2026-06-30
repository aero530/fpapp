//! Generic settings that impact the simulation / analysis results

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Generic span (something that has a min and max value)
#[derive(TS, Debug, Clone, Deserialize, Serialize, PartialEq)]
#[ts(export)]
pub struct Span<T> {
    /// Minimum value
    pub low: T,
    /// Maximum value
    pub high: T,
}

/// Social Security span settings
#[derive(TS, Debug, Clone, Deserialize, Serialize, PartialEq)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct SsaSettings {
    /// SSA breakpoints to interpolate between
    pub breakpoints: Span<f64>,
    /// taxable_income_percentage
    pub taxable_income_percentage: Span<f64>,
}

/// Analysis user settings
#[derive(TS, Debug, Clone, Deserialize, Serialize, PartialEq)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// Age you plan to retire at
    pub age_retire: u32,
    /// How long you plan to live
    pub age_die: u32,
    /// Year you were born in
    pub year_born: u32,
    /// Year to start the simulation
    pub year_start: u32,
    /// Base rate of inflation (percent)
    pub inflation_base: f64,
    /// Tax rate for your income bracket
    pub tax_income: f64,
    /// Tax rate for capital gains
    pub tax_capital_gains: f64,
    /// Fraction of current spending when retired (such as in retirement you will spend 80% of what you spend now)
    pub retirement_cost_of_living: f64,
    /// Social Security settings
    pub ssa: SsaSettings,
}

impl Settings {
    // pub fn new() -> Self {
    //     let bp = Span {
    //         low: 10.0,
    //         high: 50.0,
    //     };
    //     let tip = Span {
    //         low: 10000.0,
    //         high: 50000.0,
    //     };
    //     let ssa_values = SsaSettings {
    //         breakpoints: bp,
    //         taxable_income_percentage: tip,
    //     };
    //     Settings {
    //         age_retire: 65,
    //         age_die: 100,
    //         year_born: 1950,
    //         year_start: 2000,
    //         inflation_base: 3.0,
    //         tax_income: 20.0,
    //         tax_capital_gains: 20.0,
    //         retirement_cost_of_living: 100.0,
    //         ssa: ssa_values,
    //     }
    // }
    pub fn year_start(&self) -> u32 {
        self.year_start
    }
    pub fn year_retire(&self) -> u32 {
        self.year_born + self.age_retire
    }
    pub fn year_die(&self) -> u32 {
        self.year_born + self.age_die
    }
    pub fn year_end(&self) -> u32 {
        self.year_born + self.age_die
    }
    pub fn is_retired(&self, year: u32) -> bool {
        year >= self.year_retire()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_settings_values() -> Settings {
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

    #[test]
    fn years() {
        let settings = test_settings_values();
        assert_eq!(settings.year_start(), 2000);
        assert_eq!(settings.year_retire(), 2030);
        assert_eq!(settings.year_die(), 2080);
        assert_eq!(settings.year_end(), 2080);
    }

    #[test]
    fn retirement() {
        let settings = test_settings_values();
        assert_eq!(settings.is_retired(2010), false);
        assert_eq!(settings.is_retired(2090), true);
        assert_eq!(settings.is_retired(2040), true);
    }
}
