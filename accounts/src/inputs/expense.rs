//! User input expense values

use serde::{Deserialize, Serialize};
use ts_rs::TS;

// use super::fixed_with_inflation;

/// used to populate account dropdown for expense type selection
#[derive(TS, Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[ts(export)]
#[serde(rename_all = "snake_case")]
pub enum ExpenseOptions {
    /// fixed dollar amount
    Fixed,
    /// fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)
    FixedWithInflation,
}

// impl ExpenseOptions {
//     pub fn value(
//         self,
//         expense: f64,
//         inflation: f64,
//         duration: u32,
//     ) -> f64 {
//         match self {
//             ExpenseOptions::Fixed => expense,
//             ExpenseOptions::FixedWithInflation => {
//                 fixed_with_inflation(expense, inflation, duration)
//             }
//         }
//     }
// }