//! Financial Planning Application
//!
//! Application to simulate financial standing over time.
//! The calculations and subsiquent types are all defined in the [Accounts](accounts) crate
use log::{info, trace, LevelFilter};
use std::error::Error;
use std::fs::read_to_string;

extern crate image;
use image::{ImageBuffer, Rgba};

use sixtyfps::{Image, ModelHandle, Rgba8Pixel, SharedPixelBuffer, SharedString, VecModel};
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
    let json_file_str =
        read_to_string(std::path::Path::new(&filename)).expect("Input file note found.");

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
    yearly_totals.plot_to_file("target/totals.png".into());

    account_order.iter().for_each(|uuid| {
        let account = data.accounts.get(uuid).unwrap();
        account.plot_to_file(format!("target/{}.png", account.name()), 1600, 1200);
        // account.write(format!("target/{}.csv", account.name()));
    });

    // Start application ui
    let main_window = AppWindow::new();

    // Initialize vec to hold ui plot data
    let mut ui_income: Vec<AccountData> = Vec::new();
    let mut ui_ssa: Vec<AccountData> = Vec::new();
    let mut ui_retirement: Vec<AccountData> = Vec::new();
    let mut ui_hsa: Vec<AccountData> = Vec::new();
    let mut ui_college: Vec<AccountData> = Vec::new();
    let mut ui_expense: Vec<AccountData> = Vec::new();
    let mut ui_loan: Vec<AccountData> = Vec::new();
    let mut ui_mortgage: Vec<AccountData> = Vec::new();
    let mut ui_savings: Vec<AccountData> = Vec::new();
    let mut ui_dashboard: Vec<AccountData> = Vec::new();

    // Loop through accounts and generate a vector of thier plots
    account_order.iter().for_each(|uuid| {
        let account = data.accounts.get(uuid).unwrap();
        match account.type_id() {
            accounts::AccountType::Income => {
                ui_income.push(AccountData {
                    graph: image_buf_to_image(account.plot_to_buf(1600, 1200)),
                    input: SharedString::from("input stuff".to_string()),
                });
            },
            accounts::AccountType::Ssa => {
                ui_ssa.push(AccountData {
                    graph: image_buf_to_image(account.plot_to_buf(1600, 1200)),
                    input: SharedString::from("input stuff".to_string()),
                });
            },
            accounts::AccountType::Retirement => {
                ui_retirement.push(AccountData {
                    graph: image_buf_to_image(account.plot_to_buf(1600, 1200)),
                    input: SharedString::from("input stuff".to_string()),
                });
            },
            accounts::AccountType::Hsa => {
                ui_hsa.push(AccountData {
                    graph: image_buf_to_image(account.plot_to_buf(1600, 1200)),
                    input: SharedString::from("input stuff".to_string()),
                });
            },
            accounts::AccountType::College => {
                ui_college.push(AccountData {
                    graph: image_buf_to_image(account.plot_to_buf(1600, 1200)),
                    input: SharedString::from("input stuff".to_string()),
                });
            },
            accounts::AccountType::Expense => {
                ui_expense.push(AccountData {
                    graph: image_buf_to_image(account.plot_to_buf(1600, 1200)),
                    input: SharedString::from("input stuff".to_string()),
                });
            },
            accounts::AccountType::Loan => {
                ui_loan.push(AccountData {
                    graph: image_buf_to_image(account.plot_to_buf(1600, 1200)),
                    input: SharedString::from("input stuff".to_string()),
                });
            },
            accounts::AccountType::Mortgage => {
                ui_mortgage.push(AccountData {
                    graph: image_buf_to_image(account.plot_to_buf(1600, 1200)),
                    input: SharedString::from("input stuff".to_string()),
                });
            },
            accounts::AccountType::Savings => {
                ui_savings.push(AccountData {
                    graph: image_buf_to_image(account.plot_to_buf(1600, 1200)),
                    input: SharedString::from("input stuff".to_string()),
                });
            },
        }
        // ui_accounts.push(GraphImage {
        //     image: image_buf_to_image(account.plot_to_buf(1600, 1200)),
        //     account_type: SharedString::from(account.type_id().to_string()),
        // });
    });


    ui_dashboard.push(AccountData {
        graph: image_buf_to_image(yearly_totals.plot_to_buf(1600, 1200)),
        input: SharedString::from("input stuff".to_string()),
    });


    let income_model = Rc::new(VecModel::from(ui_income));
    let ssa_model = Rc::new(VecModel::from(ui_ssa));
    let retirement_model = Rc::new(VecModel::from(ui_retirement));
    let hsa_model = Rc::new(VecModel::from(ui_hsa));
    let college_model = Rc::new(VecModel::from(ui_college));
    let expense_model = Rc::new(VecModel::from(ui_expense));
    let loan_model = Rc::new(VecModel::from(ui_loan));
    let mortgage_model = Rc::new(VecModel::from(ui_mortgage));
    let savings_model = Rc::new(VecModel::from(ui_savings));
    let dashboard_model = Rc::new(VecModel::from(ui_dashboard));
    
    main_window.set_income(ModelHandle::new(income_model.clone()));
    main_window.set_ssa(ModelHandle::new(ssa_model.clone()));
    main_window.set_retirement(ModelHandle::new(retirement_model.clone()));
    main_window.set_hsa(ModelHandle::new(hsa_model.clone()));
    main_window.set_college(ModelHandle::new(college_model.clone()));
    main_window.set_expense(ModelHandle::new(expense_model.clone()));
    main_window.set_loan(ModelHandle::new(loan_model.clone()));
    main_window.set_mortgage(ModelHandle::new(mortgage_model.clone()));
    main_window.set_savings(ModelHandle::new(savings_model.clone()));
    main_window.set_dashboard(ModelHandle::new(dashboard_model.clone()));

    main_window.run();

    Ok(())
}

/// Convert image buffer to sixtyfps image
fn image_buf_to_image(input: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Image {
    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        input.as_raw(),
        input.width() as _,
        input.height() as _,
    );
    Image::from_rgba8(buffer)
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
