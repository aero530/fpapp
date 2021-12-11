//! Interpret user input from UI / data files

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;

use crate::accounts::{Account, AccountWrapper};

mod input_options;
mod settings;

pub use input_options::*;
pub use settings::*;

/// Represents the user data file
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct UserData<T> {
    /// The system level configuration
    pub settings: Settings,
    /// The metrics that data will be generated for
    pub accounts: HashMap<String, T>,
}

impl From<UserData<AccountWrapper>> for UserData<Box<dyn Account>> {
    fn from(other: UserData<AccountWrapper>) -> Self {
        Self {
            settings: other.settings,
            accounts: other
                .accounts
                .into_iter()
                .map(|(k, v)| (k, v.to_account_object()))
                .collect(),
        }
    }
}

impl UserData<Box<dyn Account>> {
    /// Write all account values to a single csv
    pub fn write_tables(&self, order: &[String], years: Vec<u32>, filepath: String) {
        let mut file = std::fs::File::create(filepath).unwrap();
        file.write_all("year".as_bytes()).unwrap();
        order.iter().for_each(|uuid| {
            file.write_all(format!(",\t{}", self.accounts[uuid].name()).as_bytes())
                .unwrap();
        });
        file.write_all("\n".as_bytes()).unwrap();

        years.iter().for_each(|year| {
            file.write_all(format!("{}", year).as_bytes()).unwrap();
            order.iter().for_each(|uuid| {
                self.accounts[uuid].get_value(*year);
                file.write_all(
                    format!(
                        ",\t{:.2}",
                        self.accounts[uuid].get_value(*year).unwrap_or_default()
                    )
                    .as_bytes(),
                )
                .unwrap();
            });
            file.write_all("\n".as_bytes()).unwrap();
        });
    }
}
