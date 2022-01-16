//! Aggregate various accounts into yearly impact on financial standing

use serde::{Deserialize, Serialize};

mod table;
mod table_groups;
mod totals;

pub use table::*;
pub use table_groups::*;
pub use totals::*;

/// Defines a time range with start and end values
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct YearRange {
    /// Beginning of the time range
    pub start: u32,
    /// End of the time range
    pub end: u32,
}

impl YearRange {
    /// Determine if the specified year is within the time range (inclusive)
    pub fn contains(self, year: u32) -> bool {
        (year >= self.start) && (year <= self.end)
    }
}

/// Set of year ranges used for analysis
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct Dates {
    /// Time range when the account has positive cashflow
    pub year_in: Option<YearRange>,
    /// Time range when the account has negative cashflow
    pub year_out: Option<YearRange>,
}

/// Data point used in UI plotting
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct PlotDataPoint {
    pub group: String,
    pub year: u32,
    pub value: f64,
}