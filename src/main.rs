//! Financial Planning Application
//!
//! Application to simulate financial standing over time.
//!
use log::{info, debug, trace, LevelFilter};
use std::error::Error;
use std::fs::read_to_string;

mod accounts;
// mod analysis_types;
mod config;
mod inputs;
mod plot;
// mod settings;
// mod tables;
mod simulation;

use accounts::{Account, AccountWrapper};
// use analysis_types::{AnalysisDates, YearlyTotals};
use inputs::UserData;
use simulation::{Dates, YearlyTotals};

/// Main loop
fn main() -> Result<(), Box<dyn Error>> {
    // Initialize and gather config
    let config = config::Config::new()?;
    initialize_logger(&config.log_level);

    // Read in simulated system pattern
    let filename = "archive/fp_data.json";
    let json_file_str = read_to_string(std::path::Path::new(&filename))?;

    // let _a = serde_json::from_str::<UserData<AccountWrapper>>(&json_file_str)?;

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

    let years: Vec<u32> =
        (data.settings.year_start()..data.settings.year_end()).collect::<Vec<u32>>();

    let mut yearly_totals = YearlyTotals::new();
    //yearly_totals.add_year(years[0]);

    // Initialize analysis tables
    account_order.iter().for_each(|uuid| {
        // Get dates from the linked account if this account has a link ID
        let linked_dates: Option<Dates> = match data.accounts.get(uuid).unwrap().link_id() {
            Some(link_id) => {
                // This explicitly does not allow recursion in linked_dates
                Some(Dates {
                    year_in: data
                        .accounts
                        .get(&link_id)
                        .unwrap()
                        .get_range_in(&data.settings, None),
                    year_out: data
                        .accounts
                        .get(&link_id)
                        .unwrap()
                        .get_range_out(&data.settings, None),
                })
            }
            None => None,
        };

        let impacts = data.accounts
            .get_mut(uuid)
            .unwrap()
            .init(&years, linked_dates, &data.settings)
            .unwrap();
        
        impacts.iter().for_each(|(year, impact)| {
            //yearly_totals.update(years[0], impact);
            yearly_totals.update(*year, *impact);
        });
        

        trace!(
            "{:?} {:?} {:?}",
            data.accounts.get(uuid).unwrap().type_id(),
            uuid,
            data.accounts.get(uuid).unwrap().name(),
        );
    });

    info!("Main Loop");

    // Main loop to loop through each year
    years.iter().copied().for_each(|year| {
        // debug!("year {:?}", year);

        // Initialize this year.  Year[0] already initialized during account initialization
        // if year > years[0] {
        //     yearly_totals.add_year(year);
        // }

        // Only pull values forward if this is a newly created year
        if yearly_totals.add_year(year).is_ok() {
            yearly_totals.pull_value_forward(year);

            // Loop through accounts to make contributions and withdrawals
            account_order.iter().for_each(|uuid| {
                let account = data.accounts.get_mut(uuid).unwrap();
                let impact = account
                    .simulate(year, &yearly_totals, &data.settings)
                    .unwrap();
                yearly_totals.update(year, impact);
                //this_year.update(result);
            });

            yearly_totals.deposit_income_in_net(year);
            yearly_totals
                .pay_income_tax_from_net(year, data.settings.tax_income);
            yearly_totals.pay_expenses_from_net(year);
            yearly_totals
                .pay_healthcare_expenses_from_net(year);
        }
        
        
        


    });

    data.write_tables(&account_order, years.clone(), "target/tables.csv".into());
    yearly_totals.write_summary("target/summary.csv".into());
    yearly_totals.plot("target/totals.png".into());

    account_order.iter().for_each(|uuid| {
        let account = data.accounts.get(uuid).unwrap();
        // account.write(format!("target/{}.csv", account.name()));
        account.plot(format!("target/{}.png", account.name()));
    });

    // debug!("{:?}", data.total_income(&"2020".to_string()));
    // debug!("{:?}", data.total_expenses(&"2020".to_string()));
    //debug!("{:?}", data.accounts.get("c56b7430-c5bb-11e8-a00d-d173fe7faee3"));

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
