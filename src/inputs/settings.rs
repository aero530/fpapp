//! Generic settings that impact the simulation / analysis results

use serde::{Deserialize, Serialize};

/// Generic span (something that has a min and max value)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Span<T> {
    /// Minimum value
    pub low: T,
    /// Maximum value
    pub high: T,
}

/// Social Security span settings
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SsaSettings {
    /// SSA breakpoints to interpolate between
    pub breakpoints: Span<f64>,
    /// taxable_income_percentage
    pub taxable_income_percentage: Span<f64>,
}

/// Analysis user settings
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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