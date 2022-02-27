//! Standard format to represent dollar values across multiple years

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use ts_rs::TS;

/// Table is a map keyed by year that holds account values/amounts.
///
/// Tables are stored as keyed on string but must be converted to
/// be keyed on a u32 year prior to use for analysis.
#[derive(TS, Debug, Default, Clone, Deserialize, Serialize)]
#[ts(export)]
pub struct Table<T: std::cmp::Ord>(
    /// Ordered map of (year, dollar amount) pairs
    pub BTreeMap<T, f64>,
);

impl Table<u32> {
    /// Add year with value. Return Error if it already exists.
    pub fn add(&mut self, year: u32, value: f64) -> Result<(), Box<dyn Error>> {
        match self.get(year) {
            Some(_x) => return Err(format!("The year {} already exists in table", year).into()),
            None => {
                self.0.insert(year, value);
                Ok(())
            }
        }
    }
    /// Insert / replace value for given year
    ///
    /// If the map did not have this key present, None is returned.
    /// If the map did have this key present, the value is updated, and the old value is returned.
    pub fn insert(&mut self, year: u32, value: f64) -> Option<f64> {
        self.0.insert(year, value)
    }
    /// Add the delta value to the current value
    pub fn update(&mut self, year: u32, delta: f64) {
        //self.expense.insert(year, self.expense.get(year).unwrap()+update.expense);
        let previous_value = match self.0.get(&year) {
            Some(x) => *x,
            None => 0_f64,
        };
        self.insert(year, previous_value + delta);
    }

    /// Return the value for a given year
    pub fn get(&self, year: u32) -> Option<f64> {
        self.0.get(&year).copied()
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
    /// Find the most recent year that has a non-zero value
    pub fn most_recent_value(&self) -> Option<f64> {
        self.0.iter().last().map(|(_k, v)| *v)
    }
    /// Move the most recent previous value forward if
    /// the most previous year is prior to the current year
    pub fn pull_value_forward(&mut self, year: u32) {
        if let Some(recent_year) = self.most_recent_populated_year() {
            if recent_year < year {
                *self.0.get_mut(&year).unwrap() = self.0[&recent_year];
            }
        }
    }
    /// Return the minimum table value (dollar amount)
    fn min_value(&self) -> f64 {
        self.0
            .values()
            .copied()
            .collect::<Vec<f64>>()
            .iter()
            .fold(f64::NAN, |m, v| v.min(m))
    }
    /// Return the maximum table value (dollar amount)
    pub fn max_value(&self) -> f64 {
        self.0
            .values()
            .copied()
            .collect::<Vec<f64>>()
            .iter()
            .fold(f64::NAN, |m, v| v.max(m))
    }
    /// Return the minimum table year
    fn min_key(&self) -> u32 {
        *self
            .0
            .keys()
            .cloned()
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
            .cloned()
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
    /// Return values
    pub fn values(&self) -> Vec<f64> {
        self.0.values().cloned().collect()
    }
    /// Return values
    pub fn years(&self) -> Vec<u32> {
        let mut years = self.0.keys().cloned().collect::<Vec<u32>>();
        years.sort_unstable();
        years
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
