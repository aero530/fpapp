use std::collections::HashMap;
use std::error::Error;
use std::fs::{read_to_string};
use log::{info, LevelFilter};
use serde::{Deserialize, Serialize};


mod config;
mod settings;
mod account_types;


/// Represents the user data file
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct UserData {
    /// The system level configuration
    pub settings: settings::Settings,
    /// The metrics that data will be generated for
    pub accounts: HashMap<String, account_types::Account>,
}


fn main()-> Result<(), Box<dyn Error>> {
    // Initialize and gather config
    let config = config::Config::new()?;
    initialize_logger(&config.log_level);

    println!("Hello, world!");


    // Read in simulated system pattern
    let filename = "fp_data.json";
    let json_file_str = read_to_string(std::path::Path::new(&filename))?;
    
    let file_data: UserData = serde_json::from_str(&json_file_str)?;
    println!("{:#?}",file_data);

    for (uuid, account) in file_data.accounts.iter() {
        // println!("{} {:?}",uuid, (*account).name());
        println!("{} {:?}",uuid, account.name());
    }

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
