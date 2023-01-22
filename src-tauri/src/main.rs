//! Financial Planning Application
//!
//! Application to simulate financial standing over time.
//! The calculations and subsiquent types are all defined in the [Accounts](accounts) crate

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
  )]

use std::collections::HashMap;

use flexi_logger::Logger;

// use log::{info, trace, LevelFilter};
use std::fs::read_to_string;
use serde::{Deserialize, Serialize};

mod menu;
mod logconfig;

use accounts::{Account, AccountWrapper, Dates, UserData, YearlyTotals, PlotDataSet};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RequestBody {
  id: i32,
  name: String,
}

#[derive(Debug, Clone, Serialize)]
struct MenuEvent {
  name: String,
}

#[tauri::command]
fn my_custom_command() -> String {
    println!("I was invoked from JS!");
    String::from("This is some stuff")
}

#[tauri::command]
fn file_open(path: String) -> Result<UserData<AccountWrapper>, String> {
    let json_file_str;
    match read_to_string(std::path::Path::new(&path)) {
        Ok(data) => json_file_str = data,
        Err(e) => return Err(format!("Unable to open file {}",e)),
    }

    let data = match serde_json::from_str::<UserData<AccountWrapper>>(&json_file_str) {
        Ok(data) => data,
        Err(e) => return Err(format!("Unable to process input data file {}", e)),
    };

    Ok(data)
}

#[tauri::command]
fn file_save(path: String, data: UserData<AccountWrapper> ) -> Result<String, String> {

    let json = match serde_json::to_string(&data) {
        Ok(value) => value,
        Err(e) => return Err(format!("Unable to convert data to json {}", e)),
    };

    match std::fs::write(path,json) {
        Ok(_) => Ok("File saved".into()),
        Err(e) => return Err(format!("Unable to save json data {}", e)),
    }
}

#[tauri::command]
fn run_analysis(input: UserData<AccountWrapper>) -> (HashMap<String, Vec<PlotDataSet>>, YearlyTotals) {
  let data : UserData<Box<dyn Account>> = input.into();
  analyze(data)
}

#[tauri::command]
fn do_a_thing(body: RequestBody) -> String {
  println!("{:?}", body);
  format!("{:?}", body)
  // "message response".into()
}

fn analyze(mut data: UserData<Box<dyn Account>>) -> (HashMap<String, Vec<PlotDataSet>>, YearlyTotals) {
    // Loop through accounts to determine what order they should be processed in
    let mut account_order: Vec<String> = Vec::new();

    for type_id in AccountWrapper::order().iter() {
        for (uuid, account) in data.accounts.iter() {
            if account.type_id() == *type_id {
                account_order.push(uuid.to_string());
            }
        }
    }

    // Initialize vector of year values
    let years: Vec<u32> =
        (data.settings.year_start()..data.settings.year_end()).collect::<Vec<u32>>();

    // Initilize object to keep track of yearly totals across all accounts
    let mut yearly_totals = YearlyTotals::new();

    // Initialize accounts
    account_order.iter().for_each(|uuid| {
        // Get dates from the linked account if this account has a link ID
        let linked_dates: Option<Dates> = match data.accounts.get(uuid).unwrap().link_id() {
            Some(link_id) => {
                log::trace!("Liunk ID {:?}",&link_id);
                // This explicitly does not allow recursion in linked_dates
                Some(Dates {
                    year_in: data
                        .accounts
                        .get(&link_id)
                        .expect("Unable to get linked account")
                        .get_range_in(&data.settings, None),
                    year_out: data
                        .accounts
                        .get(&link_id)
                        .expect("Unable to get linked account")
                        .get_range_out(&data.settings, None),
                })
            }
            None => None,
        };

        // Initialize the account & get the impacts it has based on the tables of historical data the user has input
        let impacts = data
            .accounts
            .get_mut(uuid)
            .unwrap()
            .init(linked_dates, &data.settings)
            .unwrap();

        // Apply the impacts to yearly totals
        impacts.iter().for_each(|(year, impact)| {
            if !yearly_totals.contains_year(*year) {
                yearly_totals.add_year(*year, false).unwrap();
            }
            yearly_totals.update(*year, *impact);
        });

        log::trace!(
            "{:?} {:?} {:?}",
            data.accounts.get(uuid).unwrap().type_id(),
            uuid,
            data.accounts.get(uuid).unwrap().name(),
        );
    });

    log::info!("Main Loop");

    // Main loop to loop through each year
    years.iter().copied().for_each(|year| {
        // Add a new year to yearly_totals and pull some of the previous values forward
        // If the year already exists (as it might if a user has historical data that
        // conflicts with this analysis year) then skip analysis and leave the yearly total
        // tables as they are.
        if yearly_totals.add_year(year, true).is_ok() {
            // Loop through accounts to make contributions and withdrawals
            account_order.iter().for_each(|uuid| {
                // Get the linked account uuid (if this account has a linked account)
                let link_id = data.accounts.get(uuid).unwrap().link_id();

                let link_value = match link_id {
                    Some(id) => {
                        match data.accounts.get(&id) {
                            Some(linked_account) => {linked_account.get_value(year)},
                            None => None
                        }
                    },
                    None => None,
                };

                let account = data.accounts.get_mut(uuid).unwrap();

                // Simulate this year for the account with specified uuid
                let impact = account
                    .simulate(year, &yearly_totals, &data.settings, link_value)
                    .unwrap();
                // Apply the impact for this account to yearly_totals
                yearly_totals.update(year, impact);
            });

            // Close out the year
            yearly_totals.deposit_income_in_net(year);
            yearly_totals.pay_income_tax_from_net(year, data.settings.tax_income);
            yearly_totals.pay_expenses_from_net(year);
            yearly_totals.pay_healthcare_expenses_from_net(year);
        }
    });

    let mut plot_data : HashMap<String, Vec<PlotDataSet>> = HashMap::new();

    for (uuid, account) in data.accounts.iter() {
        plot_data.insert(uuid.to_string(), account.get_plot_data());
    }

    (plot_data, yearly_totals)
}


/// Main loop
fn main() {
    
    // Initialize and gather config
    let logconfig = logconfig::Logconfig::new().expect("Unable to create config file.");    
    Logger::try_with_str(logconfig.log_level).expect("Could not parse log level.").format(flexi_logger::colored_default_format).start().unwrap();

    tauri::Builder::default()
        .menu(menu::get_menu())
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "quit" => {
                std::process::exit(0);
                },
                "open" => {
                    let data = MenuEvent {
                        name: "file-open".to_string(),
                    };
                    event.window().emit("rust-event", data).expect("failed to emit");
                },
                "save" => {
                    let data = MenuEvent {
                        name: "file-save".to_string(),
                    };
                    event.window().emit("rust-event", data).expect("failed to emit");
                },
                "saveas" => {
                    let data = MenuEvent {
                        name: "file-saveas".to_string(),
                    };
                    event.window().emit("rust-event", data).expect("failed to emit");
                },
                _ => {
                println!("{:?}", event.menu_item_id());
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            my_custom_command,
            do_a_thing,
            file_open,
            file_save,
            run_analysis,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
