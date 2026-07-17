//! Standard format to represent dollar values across multiple years

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::Error;

/// Table is a map keyed by year that holds account values/amounts.
///
/// The on-disk format is `{"2020": 100.5}` — JSON object keys are always
/// strings, so deserialization parses them into u32 years (see the manual
/// `Deserialize` impl below) and serialization writes them back as strings.
#[derive(Debug, Default, Clone, Serialize)]
pub struct Table(
    /// Ordered map of (year, dollar amount) pairs
    pub(crate) BTreeMap<u32, f64>,
);

/// A year map key that accepts both string keys (the JSON representation and
/// the buffered form used by internally-tagged enums) and integer keys.
struct YearKey(u32);

impl<'de> Deserialize<'de> for YearKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct KeyVisitor;
        impl serde::de::Visitor<'_> for KeyVisitor {
            type Value = YearKey;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a calendar year")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<YearKey, E> {
                v.trim()
                    .parse::<u32>()
                    .map(YearKey)
                    .map_err(|_| E::custom(format!("invalid year '{}' in table", v)))
            }
            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<YearKey, E> {
                u32::try_from(v)
                    .map(YearKey)
                    .map_err(|_| E::custom(format!("invalid year '{}' in table", v)))
            }
        }
        deserializer.deserialize_any(KeyVisitor)
    }
}

impl<'de> Deserialize<'de> for Table {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct TableVisitor;
        impl<'de> serde::de::Visitor<'de> for TableVisitor {
            type Value = Table;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a map of year to dollar amount")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Table, A::Error> {
                let mut out = BTreeMap::new();
                while let Some((key, value)) = map.next_entry::<YearKey, f64>()? {
                    out.insert(key.0, value);
                }
                Ok(Table(out))
            }
        }
        deserializer.deserialize_map(TableVisitor)
    }
}

impl Table {
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
    /// Iterate over (year, value) pairs in year order
    pub fn iter(&self) -> impl Iterator<Item = (u32, f64)> + '_ {
        self.0.iter().map(|(k, v)| (*k, *v))
    }
    /// Return values in year order
    pub fn values(&self) -> Vec<f64> {
        self.0.values().copied().collect()
    }
    /// Return years in ascending order (BTreeMap keys are already sorted)
    pub fn years(&self) -> Vec<u32> {
        self.0.keys().copied().collect()
    }
    /// Config error if any entry is negative.  Used by account `init`
    /// validation: a negative balance/amount seed would otherwise surface
    /// later as a confusing mid-simulation internal error (or, for healthcare
    /// expenses, corrupt the HSA settlement).
    pub(crate) fn validate_non_negative(&self) -> Result<(), Error> {
        for (year, value) in self.iter() {
            if value < 0_f64 {
                return Err(Error::config(format!(
                    "historical table value for year {} is negative ({})",
                    year, value
                )));
            }
        }
        Ok(())
    }
}

impl IntoIterator for Table {
    type Item = (u32, f64);
    type IntoIter = std::collections::btree_map::IntoIter<u32, f64>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(entries: &[(u32, f64)]) -> Table {
        Table(entries.iter().copied().collect())
    }

    #[test]
    fn deserializes_string_keys_into_years() {
        // JSON object keys are strings; serde must parse them into u32 years
        // and serialize them back identically.
        let t: Table = serde_json::from_str(r#"{"2020": 100.5, "2021": 200.0}"#).unwrap();
        assert_eq!(t.get(2020), Some(100.5));
        assert_eq!(t.get(2021), Some(200.0));
        assert_eq!(
            serde_json::to_string(&t).unwrap(),
            r#"{"2020":100.5,"2021":200.0}"#
        );
        // a malformed year key is a parse error, not a panic
        assert!(serde_json::from_str::<Table>(r#"{"20x0": 1.0}"#).is_err());
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
    fn validate_non_negative_rejects_negative_entries() {
        assert!(
            table(&[(2020, 0.0), (2021, 5.0)])
                .validate_non_negative()
                .is_ok()
        );
        let err = table(&[(2020, -1.0)]).validate_non_negative().unwrap_err();
        assert!(err.to_string().contains("2020"));
    }

    #[test]
    fn most_recent_value_before_ignores_future_years() {
        // Regression test for A3: a pre-seeded future year must not leak backwards.
        let t = table(&[(2020, 100.0), (2025, 500.0)]);
        assert_eq!(t.most_recent_value_before(2021), Some(100.0));
        assert_eq!(t.most_recent_value_before(2026), Some(500.0));
        assert_eq!(t.most_recent_value_before(2020), None);
    }
}
