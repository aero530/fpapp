//! Financial Planning Application
//!
//! Application to simulate financial standing over time.
//!
use log::{info, debug, trace, LevelFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

mod account_types;
mod config;
mod settings;

use account_types::{Account, AccountWrapper, AccountType};

use crate::account_types::AnalysisDates;

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
    fn total_income(&self, year: &String) -> f64 {
        self.accounts
            .iter()
            .fold(0.0, |acc, (_uuid, account)| {
                acc + account.get_income(year).unwrap_or(0.0)
            })
    }
    fn total_expenses(&self, year: &String) -> f64 {
        self.accounts
            .iter()
            .fold(0.0, |acc, (_uuid, account)| {
                acc + account.get_expense(year).unwrap_or(0.0)
            })
    }
}

/// Main loop
fn main() -> Result<(), Box<dyn Error>> {
    // Initialize and gather config
    let config = config::Config::new()?;
    initialize_logger(&config.log_level);

    // Read in simulated system pattern
    let filename = "archive/fp_data.json";
    let json_file_str = read_to_string(std::path::Path::new(&filename))?;

    let mut data: UserData<Box<dyn Account>> =
        serde_json::from_str::<UserData<AccountWrapper>>(&json_file_str)?.into();

    // Loop through accounts to determine what order they should be processed in
    let mut account_order: Vec<String> = Vec::new();

    for type_id in AccountWrapper::order().iter() {
        for (uuid, account) in data.accounts.iter() {
            if account.type_id() == *type_id {
                account_order.push(uuid.to_string());
            }
        }
    }

    let years: Vec<u32> = (data.settings.year_start()..data.settings.year_end())
        .collect::<Vec<u32>>();
    let mut net: HashMap<String, f64> = HashMap::new();

    // Initialize analysis tables
    account_order.iter().for_each(|uuid| {

        let dates : Option<AnalysisDates> = match data.accounts.get(uuid).unwrap().type_id() {
            AccountType::Retirement => {
                let link_id = data.accounts.get(uuid).unwrap().link_id();
                match link_id {
                    Some(id) => {
                        Some(AnalysisDates {
                            year_in: data.accounts.get(&id).unwrap().get_range_in(&data.settings),
                            year_out: data.accounts.get(&id).unwrap().get_range_out(&data.settings),
                        })
                    },
                    None => None
                }
            },
            _ => None,
        };

        data.accounts.get_mut(uuid).unwrap().init(&years, dates, &data.settings).unwrap();

        trace!("{:?}", data.accounts.get(uuid).unwrap());
    });


    // ----------------------------------------------------------------------
    // Main loop to loop through each year
    // ----------------------------------------------------------------------
    years.iter().for_each(|year| {
        trace!("{:?}", year);

        // If this is not the first year then pull the value from year-1 into the current year's value of net
        if *year > years[0] {
            let net_prev = net[&(year-1).to_string()];
            net.insert(year.to_string(), net_prev);
        }

        account_order.iter().for_each(|uuid| {
            data.accounts.get_mut(uuid).unwrap().simulate(*year, &data.settings).unwrap();
            let a = data.accounts.get(uuid).unwrap().get_income(&year.to_string());
            match a {
                Some(v) => {
                    *(&mut net).entry(year.to_string()).or_insert(v) += v;
                },
                None => {}
            }
        })
    });




    // data.accounts.get("c188e2d0-c5bb-11e8-a00d-d173fe7faee3").unwrap().get_range_in(settings, linked_account)



    years.iter().for_each(|year| {
        info!("{:?} - {:?}", year, net.get(&year.to_string()).unwrap());
    });
    
    
    debug!("{:?}", data.total_income(&"2020".to_string()));
    debug!("{:?}", data.total_expenses(&"2020".to_string()));

    Ok(())
}

/// Initialize the logger used in the application with the specified log level
///
/// # Panics
/// If the specified `log_level` is not one of `off`, `error`, `warn`, `info`, `debug`, or `trace`.
fn initialize_logger(log_level: &str) {
    let log_level = log_level
        .parse::<LevelFilter>()
        .expect("Unable to parse log level");
    env_logger::builder()
        .filter(Some("fpapp"), log_level)
        .init();
    info!("Initializing...");
}

// macros

// monomorphized generic traits - could not be all stored in teh same collection

// dynamic trait object
//  - needs Box dyn trait name

//  account is a trait
//  dynamic trait objects

//  account as an enum with a function that does a match and boxes it then

//  json -> serde to figure out variant of enum.
//  enum has method in impl block that destructures the account the returns boxed version

//  https://doc.rust-lang.org/book/ch17-02-trait-objects.html
