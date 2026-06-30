//! Financial Planning Application
//!
//! Application to simulate financial standing over time.
//! The calculations and subsequent types are all defined in the [Accounts](accounts) crate

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
  )]

use std::collections::HashMap;

use flexi_logger::Logger;
use tauri::{menu::{Menu, MenuItemBuilder, SubmenuBuilder}, Emitter};

use std::fs::read_to_string;
use serde::{Deserialize, Serialize};


mod log_config;

use accounts::{Account, AccountType, AccountWrapper, Dates, UserData, YearlyTotals, PlotDataSet};

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
    String::from("This is some stuff")
}

#[tauri::command]
fn file_open(path: String) -> Result<UserData<AccountWrapper>, String> {
    let json_file_str;
    let a = std::path::Path::new(&path);
    match read_to_string(a) {
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
fn run_analysis(input: UserData<AccountWrapper>) -> Result<(HashMap<String, Vec<PlotDataSet>>, YearlyTotals), String> {
  let data : UserData<Box<dyn Account>> = input.into();
  analyze(data).map_err(|e| e.to_string())
}

#[tauri::command]
fn do_a_thing(body: RequestBody) -> String {
  format!("{:?}", body)
}

fn analyze(mut data: UserData<Box<dyn Account>>) -> Result<(HashMap<String, Vec<PlotDataSet>>, YearlyTotals), Box<dyn std::error::Error>> {
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

    // Initialize object to keep track of yearly totals across all accounts
    let mut yearly_totals = YearlyTotals::new();

    // Initialize accounts
    account_order.iter().for_each(|uuid| {
        // Get dates from the linked account if this account has a link ID
        let linked_dates: Option<Dates> = match data.accounts.get(uuid).unwrap().link_id() {
            Some(link_id) => {
                log::trace!("Link ID {:?}",&link_id);
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
    for year in years.iter().copied() {
        // Add a new year to yearly_totals and pull some of the previous values forward.
        // If the year already exists (as it might if a user has historical data that
        // conflicts with this analysis year) then skip analysis and leave the yearly total
        // tables as they are.
        if yearly_totals.add_year(year, true).is_ok() {
            // Loop through accounts to make contributions and withdrawals
            for uuid in account_order.iter() {
                // Get the linked account uuid (if this account has a linked account)
                let link_id = data.accounts.get(uuid).unwrap().link_id();

                // Only use get_value from Income accounts — other account types (Savings,
                // Retirement) return a running balance via get_value, not yearly income,
                // which would produce incorrect PercentOfIncome contributions.
                let link_value = match link_id {
                    Some(ref id) => {
                        match data.accounts.get(id) {
                            Some(linked_account) if linked_account.type_id() == AccountType::Income => {
                                linked_account.get_value(year)
                            }
                            Some(linked_account) => {
                                log::warn!(
                                    "Account '{}': income_link points to a {:?} account, not Income; falling back to total income for PercentOfIncome contributions",
                                    data.accounts.get(uuid).unwrap().name(),
                                    linked_account.type_id()
                                );
                                None
                            }
                            None => None,
                        }
                    }
                    None => None,
                };

                let account = data.accounts.get_mut(uuid).unwrap();

                // Simulate this year for the account. Propagate errors so the user sees
                // a clear message rather than a silently corrupt simulation.
                let impact = account
                    .simulate(year, &yearly_totals, &data.settings, link_value)
                    .map_err(|e| format!("Account '{}', year {}: {}", account.name(), year, e))?;
                yearly_totals.update(year, impact);
            }

            // Close out the year
            yearly_totals.deposit_income_in_net(year);
            yearly_totals.pay_income_tax_from_net(year, data.settings.tax_income);
            yearly_totals.pay_expenses_from_net(year);
            yearly_totals.pay_healthcare_expenses_from_net(year);
        }
    }

    let mut plot_data : HashMap<String, Vec<PlotDataSet>> = HashMap::new();








    // data.write_tables(&account_order, years, "out.csv".to_owned());
    // data.accounts["c56b7430-c5bb-11e8-a00d-d173fe7faee3"].write("mort.csv".to_owned());








    for (uuid, account) in data.accounts.iter() {
        plot_data.insert(uuid.to_string(), account.get_plot_data());
    }

    Ok((plot_data, yearly_totals))
}


/// Main loop
fn main() {
    
    // Initialize and gather config
    let log_config = log_config::LogConfig::new().expect("Unable to create config file.");    
    Logger::try_with_str(log_config.log_level).expect("Could not parse log level.").format(flexi_logger::colored_default_format).start().unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {

            let handle = app.handle();
            let menu = Menu::new(handle)?;

            let open = MenuItemBuilder::with_id("open", "Open").build(app)?;
            let save = MenuItemBuilder::with_id("save", "Save").build(app)?;
            let save_as = MenuItemBuilder::with_id("save_as", "Save As").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            
            let submenu = SubmenuBuilder::new(handle, "File").items(&[ 
                &open, &save, &save_as
             ]).build()?;

            menu.append(&submenu)?;
            menu.append(&quit)?;

            app.set_menu(menu)?;

            app.on_menu_event(move |app_handle, event| {
                if event.id() == quit.id() {
                    std::process::exit(0);
                } else if event.id() == open.id() {
                    let data = MenuEvent {
                        name: "file-open".to_string(),
                    };
                    app_handle.emit("rust-event", data).expect("failed to emit");
                }  else if event.id() == save.id() {
                    let data = MenuEvent {
                        name: "file-save".to_string(),
                    };
                    app_handle.emit("rust-event", data).expect("failed to emit");
                } else if event.id() == save_as.id() {
                    let data = MenuEvent {
                        name: "file-save_as".to_string(),
                    };
                    app_handle.emit("rust-event", data).expect("failed to emit");
                } else  {
                    log::warn!("Unhandled menu event id: {:?}", event.id());
                }
            });
            Ok(())
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
