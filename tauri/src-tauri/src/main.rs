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
use serde::Serialize;


mod log_config;

use accounts::{Account, AccountWrapper, UserData, YearlyTotals, PlotDataSet};

#[derive(Debug, Clone, Serialize)]
struct MenuEvent {
  name: String,
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
    let data: UserData<Box<dyn Account>> = input.try_into().map_err(|e: Box<dyn std::error::Error>| e.to_string())?;
    accounts::run(data).map_err(|e| e.to_string())
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
            file_open,
            file_save,
            run_analysis,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
