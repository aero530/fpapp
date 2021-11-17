use serde::{Deserialize, Serialize};

/// Generic range
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Range<T> {
    pub low: T,
    pub high: T,
}

/// Social Security range settings
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SsaSettings {
    pub breakpoints: Range<f64>,
    pub taxable_income_percentage: Range<f64>,
}

/// Analysis user settings
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub age_retire: f64,
    pub age_die: f64,
    pub year_born: f64,
    pub year_start: f64,
    pub inflation_base: f64,
    pub tax_income: f64,
    pub tax_capital_gains: f64,
    pub retirement_cost_of_living: f64,
    pub ssa: SsaSettings,
}