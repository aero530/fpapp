//! Interpret user input from UI / data files

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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