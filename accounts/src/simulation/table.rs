//! Standard format to represent dollar values across multiple years

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::Error;

/// Table is a map keyed by year that holds account values/amounts.
///
/// Tables are stored as keyed on string but must be converted to
/// be keyed on a u32 year prior to use for analysis.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Table<T: std::cmp::Ord>(
    /// Ordered map of (year, dollar amount) pairs
    pub(crate) BTreeMap<T, f64>,
);

impl Table<u32> {
    /// Add year with value. Return Error if it already exists.
    pub fn add(&mut self, year: u32, value: f64) -> Result<(), Error> {
        match self.get(year) {
            Some(_x) => Err(Error::internal(format!(
                "the year {} already exists in table",
                year
            ))),
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
        let previous_value = self.get(year).unwrap_or_default();
        self.insert(year, previous_value + delta);
    }

    /// Return the value for a given year
    pub fn get(&self, year: u32) -> Option<f64> {
        self.0.get(&year).copied()
    }
    /// Return the value of the most recent year strictly before `year`.
    ///
    /// Entries at or after `year` are ignored so that future (pre-seeded
    /// historical) values never leak backwards into earlier years. Zero and
    /// negative values are carried forward like any other value.
    pub fn most_recent_value_before(&self, year: u32) -> Option<f64> {
        self.0.range(..year).next_back().map(|(_k, v)| *v)
    }
    /// Copy the value of the most recent year prior to `year` into `year`
    pub fn pull_value_forward(&mut self, year: u32) {
        if let Some(value) = self.most_recent_value_before(year) {
            self.0.insert(year, value);
        }
    }
    /// Return the minimum table value (dollar amount)
    fn min_value(&self) -> f64 {
        self.0.values().fold(f64::NAN, |m, v| v.min(m))
    }
    /// Return the maximum table value (dollar amount)
    pub fn max_value(&self) -> f64 {
        self.0.values().fold(f64::NAN, |m, v| v.max(m))
    }
    /// Return the min and max key (year) values, or None if the table is empty
    pub fn domain(&self) -> Option<(u32, u32)> {
        match (self.0.first_key_value(), self.0.last_key_value()) {
            (Some((min, _)), Some((max, _))) => Some((*min, *max)),
            _ => None,
        }
    }
    /// Return the min and max value (dollar amount); NaN when the table is empty
    pub fn range(&self) -> (f64, f64) {
        (self.min_value(), self.max_value())
    }
    /// Return values in year order
    pub fn values(&self) -> Vec<f64> {
        self.0.values().copied().collect()
    }
    /// Return years in ascending order (BTreeMap keys are already sorted)
    pub fn years(&self) -> Vec<u32> {
        self.0.keys().copied().collect()
    }
}

impl IntoIterator for Table<u32> {
    type Item = (u32, f64);
    type IntoIter = std::collections::btree_map::IntoIter<u32, f64>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl TryFrom<Table<String>> for Table<u32> {
    type Error = Error;
    /// Convert a string-keyed table (as stored in the data file) into a
    /// year-keyed table. A malformed year key produces an error instead of a panic.
    fn try_from(other: Table<String>) -> Result<Self, Self::Error> {
        other
            .0
            .into_iter()
            .map(|(k, v)| {
                k.trim()
                    .parse::<u32>()
                    .map(|year| (year, v))
                    .map_err(|_| Error::data(format!("invalid year '{}' in table", k)))
            })
            .collect::<Result<BTreeMap<u32, f64>, Self::Error>>()
            .map(Self)
    }
}

impl From<(Vec<u32>, Vec<f64>)> for Table<u32> {
    fn from(other: (Vec<u32>, Vec<f64>)) -> Self {
        Self(other.0.into_iter().zip(other.1).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(entries: &[(u32, f64)]) -> Table<u32> {
        Table(entries.iter().copied().collect())
    }

    #[test]
    fn pull_value_forward_carries_negative_and_zero_values() {
        // Regression test for A1: debt (negative net) must roll forward, not be
        // replaced by the most recent positive year.
        let mut t = table(&[(2020, 100.0), (2021, -50.0)]);
        t.insert(2022, 0.0);
        t.pull_value_forward(2022);
        assert_eq!(t.get(2022), Some(-50.0));

        let mut t = table(&[(2020, 100.0), (2021, 0.0)]);
        t.insert(2022, 123.0);
        t.pull_value_forward(2022);
        assert_eq!(t.get(2022), Some(0.0));
    }

    #[test]
    fn most_recent_value_before_ignores_future_years() {
        // Regression test for A3: a pre-seeded future year must not leak backwards.
        let t = table(&[(2020, 100.0), (2025, 500.0)]);
        assert_eq!(t.most_recent_value_before(2021), Some(100.0));
        assert_eq!(t.most_recent_value_before(2026), Some(500.0));
        assert_eq!(t.most_recent_value_before(2020), None);
    }

    #[test]
    fn try_from_rejects_malformed_year_keys() {
        // Regression test for the panic on hand-edited data files.
        let mut bad = Table::<String>::default();
        bad.0.insert("20x0".into(), 1.0);
        assert!(Table::<u32>::try_from(bad).is_err());

        let mut good = Table::<String>::default();
        good.0.insert("2020".into(), 1.0);
        good.0.insert(" 2021 ".into(), 2.0);
        let converted = Table::<u32>::try_from(good).unwrap();
        assert_eq!(converted.get(2020), Some(1.0));
        assert_eq!(converted.get(2021), Some(2.0));
    }

    #[test]
    fn domain_of_empty_table_is_none() {
        assert_eq!(Table::<u32>::default().domain(), None);
        assert_eq!(
            table(&[(2020, 1.0), (2030, 2.0)]).domain(),
            Some((2020, 2030))
        );
    }
}
