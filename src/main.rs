//! Financial Planning Application
//!
//! Application to simulate financial standing over time.
//! The calculations and subsiquent types are all defined in the [Accounts](accounts) crate
use log::{info, trace, LevelFilter};
use std::error::Error;
use std::fs::read_to_string;

extern crate image;

use sixtyfps::{SharedPixelBuffer, Rgba8Pixel, Image, Model, ModelHandle, VecModel};
use std::rc::Rc;

mod config;

use accounts::{Account, AccountWrapper, Dates, UserData, YearlyTotals};

sixtyfps::include_modules!();

/// Main loop
fn main() -> Result<(), Box<dyn Error>> {
    // Initialize and gather config
    let config = config::Config::new()?;
    initialize_logger(&config.log_level);

    // Read in user json data
    let filename = "archive/fp_data.json";
    let json_file_str = read_to_string(std::path::Path::new(&filename)).expect("Input file note found.");

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
        // Add a new year to yearly_totals and pull some of the previous values forward
        // If the year already exists (as it might if a user has historical data that
        // conflicts with this analysis year) then skip analysis and leave the yearly total
        // tables as they are.
        if yearly_totals.add_year(year, true).is_ok() {
            // Loop through accounts to make contributions and withdrawals
            account_order.iter().for_each(|uuid| {
                // Simulate this year for the account with specified uuid
                let account = data.accounts.get_mut(uuid).unwrap();
                let impact = account
                    .simulate(year, &yearly_totals, &data.settings)
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

    // Write results to files & do some plotting
    data.write_tables(&account_order, years, "target/tables.csv".into());
    yearly_totals.write_summary("target/summary.csv".into());
    yearly_totals.plot("target/totals.png".into());

    account_order.iter().for_each(|uuid| {
        let account = data.accounts.get(uuid).unwrap();
        account.plot(format!("target/{}.png", account.name()));
        // account.write(format!("target/{}.csv", account.name()));
    });

    
    // Start application ui
    let main_window = AppWindow::new();

    // Initialize vec to hold ui plot data
    let mut ui_graphs: Vec<GraphImage> = Vec::new();

    // Loop through accounts and generate a vector of thier plots
    account_order.iter().for_each(|uuid| {
        let account = data.accounts.get(uuid).unwrap();
        let graph = account.plot_into_rgba8(1600, 1200);
        let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
            graph.as_raw(),
            graph.width() as _,
            graph.height() as _,
        );
        let new_graph = Image::from_rgba8(buffer);
        ui_graphs.push(GraphImage{image: new_graph});
    });


    let graphs_model = Rc::new(VecModel::from(ui_graphs));
    main_window.set_graphs(ModelHandle::new(graphs_model.clone()));

    main_window.run();

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
