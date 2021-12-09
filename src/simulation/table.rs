//! Standard format to represent dollar values across multiple years

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Table is a map keyed by year that holds account values/amounts.
///
/// Tables are stored as keyed on string but must be converted to
/// be keyed on a u32 year prior to use for analysis.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Table<T: std::cmp::Ord>(
    /// Ordered map of (year, dollar amount) pairs
    pub BTreeMap<T, f64>,
);

impl Table<u32> {
    /// Return the value for a given year
    pub fn get(&self, year: u32) -> Option<f64> {
        match self.0.get(&year) {
            Some(v) => Some(*v),
            None => None,
        }
    }
    /// Find the most recent year that has a non-zero value
    fn most_recent_populated_year(&self) -> Option<u32> {
        self.0
            .iter()
            .filter(|(_k, v)| **v > f64::EPSILON)
            .map(|(k, _v)| *k)
            .collect::<Vec<u32>>()
            .iter()
            .copied()
            .max()
    }
    /// Move the previous year's table value to the specified year
    pub fn pull_value_forward(&mut self, year: u32) {
        match self.most_recent_populated_year() {
            Some(recent_year) => {
                if recent_year == year - 1 {
                    *self.0.get_mut(&year).unwrap() = self.0[&(year - 1)];
                }
            }
            None => {}
        }
    }
    /// Return the minimum table value (dollar amount)
    fn min_value(&self) -> f64 {
        self.0
            .values()
            .copied()
            .collect::<Vec<f64>>()
            .iter()
            .fold(0.0 / 0.0, |m, v| v.min(m))
    }
    /// Return the maximum table value (dollar amount)
    pub fn max_value(&self) -> f64 {
        self.0
            .values()
            .copied()
            .collect::<Vec<f64>>()
            .iter()
            .fold(0.0 / 0.0, |m, v| v.max(m))
    }
    /// Return the minimum table year
    fn min_key(&self) -> u32 {
        *self
            .0
            .keys()
            .copied()
            .collect::<Vec<u32>>()
            .iter()
            .min()
            .unwrap()
    }
    /// Return the maximum table year
    fn max_key(&self) -> u32 {
        *self
            .0
            .keys()
            .copied()
            .collect::<Vec<u32>>()
            .iter()
            .max()
            .unwrap()
    }
    /// Return the min and max key (year) values
    pub fn domain(&self) -> (u32, u32) {
        (self.min_key(), self.max_key())
    }
    /// Return the min and max value (dollar amount)
    pub fn range(&self) -> (f64, f64) {
        (self.min_value(), self.max_value())
    }
}

impl IntoIterator for Table<u32> {
    type Item = (u32, f64);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0
            .keys()
            .zip(self.0.values())
            .map(|(x, y)| (*x, *y))
            .collect::<Vec<(u32, f64)>>()
            .into_iter()
    }
}

impl From<Table<String>> for Table<u32> {
    fn from(other: Table<String>) -> Self {
        Self(
            other
                .0
                .iter()
                .map(|(k, v)| (k.parse::<u32>().unwrap(), *v))
                .collect(),
        )
    }
}

impl From<(Vec<u32>, Vec<f64>)> for Table<u32> {
    fn from(other: (Vec<u32>, Vec<f64>)) -> Self {
        let mut map = BTreeMap::new();
        other
            .0
            .iter()
            .zip(other.1)
            .into_iter()
            .for_each(|(year, value)| {
                map.insert(*year, value);
            });
        Self(map)
    }
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub enum AnalysisTable {
//     Single(SingleTable),
//     Savings(SavingsTables),
//     Loan(LoanTables),
// }

// impl AnalysisTable {
//     // pub fn most_recent_populated_year(&self) -> Option<u32> {
//     //     match self {
//     //         AnalysisTable::Single(tables) => tables.value.most_recent_populated_year(),
//     //         AnalysisTable::Savings(tables) => tables.value.most_recent_populated_year(),
//     //         AnalysisTable::Loan(tables) => tables.value.most_recent_populated_year(),
//     //     }
//     // }
//     pub fn pull_value_forward(&mut self, year: u32) {
//         match self {
//             AnalysisTable::Single(tables) => {tables.value.pull_value_forward(year);},
//             AnalysisTable::Savings(tables) => {tables.value.pull_value_forward(year);},
//             AnalysisTable::Loan(tables) => {tables.value.pull_value_forward(year);},
//         }
//     }
//     pub fn value_table(&self) -> &Table<u32> {
//         match self {
//             AnalysisTable::Single(tables) => &tables.value,
//             AnalysisTable::Savings(tables) => &tables.value,
//             AnalysisTable::Loan(tables) => &tables.value,
//         }
//     }
//     // pub fn value_table_as_mut(&self) -> &mut Table<u32> {
//     //     match self {
//     //         AnalysisTable::Single(tables) => {

//     //             &mut tables.as_mut().value
//     //         },
//     //         AnalysisTable::Savings(tables) => &mut tables.value,
//     //         AnalysisTable::Loan(tables) => &mut tables.value,
//     //     }
//     // }
//     pub fn contributions_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Savings(tables) => Some(&tables.contributions),
//             _ => None,
//         }
//     }
//     pub fn employer_contributions_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Savings(tables) => tables.employer_contributions.as_ref(),
//             _ => None,
//         }
//     }
//     pub fn earnings_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Savings(tables) => Some(&tables.earnings),
//             _ => None,
//         }
//     }
//     pub fn withdrawals_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Savings(tables) => Some(&tables.withdrawals),
//             _ => None,
//         }
//     }
//     pub fn interest_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Loan(tables) => Some(&tables.interest),
//             _ => None,
//         }
//     }
//     pub fn payments_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Loan(tables) => Some(&tables.payments),
//             _ => None,
//         }
//     }
//     pub fn escrow_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Loan(tables) => tables.escrow.as_ref(),
//             _ => None,
//         }
//     }
//     pub fn insurance_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Loan(tables) => tables.insurance.as_ref(),
//             _ => None,
//         }
//     }
//     pub fn value(&self, year: u32) -> f64 {
//         self.value_table().get(year).unwrap_or_default()
//         // self.analysis
//         //     .as_ref()
//         //     .unwrap()
//         //     .value
//         //     .0
//         //     .get(&year)
//         //     .map(|v| *v)
//     }
//     pub fn write(&self, filepath: String) {
//         println!("{}", filepath);

//     }

// }
