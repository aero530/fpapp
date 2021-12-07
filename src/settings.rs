//! Settings that impact the simulation / analysis results

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;

use crate::accounts::{Account, AccountWrapper};
use crate::settings;

/// Generic range
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Range<T> {
    /// Minimum value
    pub low: T,
    /// Maximum value
    pub high: T,
}

/// Social Security range settings
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SsaSettings {
    /// SSA breakpoints to interpolate between
    pub breakpoints: Range<f64>,
    /// taxable_income_percentage
    pub taxable_income_percentage: Range<f64>,
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

/// Represents the user data file
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct UserData<T> {
    /// The system level configuration
    pub settings: settings::Settings,
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
    // pub fn total_income(&self, year: u32) -> f64 {
    //     self.accounts.iter().fold(0.0, |acc, (_uuid, account)| {
    //         acc + account.get_income(year).unwrap_or(0.0)
    //     })
    // }
    // pub fn total_expenses(&self, year: u32) -> f64 {
    //     self.accounts.iter().fold(0.0, |acc, (_uuid, account)| {
    //         acc + account.get_expense(year).unwrap_or(0.0)
    //     })
    // }
    // pub fn print_year(&self, order: &Vec<String>, year: u32) {
    //     print!("{:?} ", year);
    //     order.iter().for_each(|uuid| {
    //         let account = self.accounts.get(uuid).unwrap();
    //         let value = account.get_value(year).unwrap();
    //         print!("{:.2} ", value);
    //     });
    //     println!("");
    // }
    /// Write value of each account for each year to a csv file
    pub fn write_tables(&self, order: &Vec<String>, years: Vec<u32>, filename: String) {
        let mut file = std::fs::File::create(filename).unwrap();
        file.write_all("year, ".as_bytes()).unwrap();
        order.iter().for_each(|uuid| {
            let account = self.accounts.get(uuid).unwrap();
            let value = account.name();
            file.write_all(format!("{}, ", value).as_bytes()).unwrap();
        });
        file.write_all("\n".as_bytes()).unwrap();
        years.iter().for_each(|year| {
            file.write_all(format!("{:?}, ", year).as_bytes()).unwrap();
            order.iter().for_each(|uuid| {
                let account = self.accounts.get(uuid).unwrap();
                let value = account.get_value(*year).unwrap();
                file.write_all(format!("{:.2}, ", value).as_bytes())
                    .unwrap();
            });
            file.write_all("\n".as_bytes()).unwrap();
        });
    }
}
