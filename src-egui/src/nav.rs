use eframe::egui;
use serde_json::json;
use uuid::Uuid;

use crate::app::{FpApp, Page};

const ACCOUNT_TYPES: &[(&str, &str)] = &[
    ("income", "Income"),
    ("ssa", "SSA"),
    ("retirement", "Retirement"),
    ("hsa", "HSA"),
    ("college", "College"),
    ("expense", "Expense"),
    ("loan", "Loan"),
    ("mortgage", "Mortgage"),
    ("savings", "Savings"),
];

pub fn show_nav(app: &mut FpApp, ctx: &egui::Context) {
    egui::SidePanel::left("nav_panel")
        .exact_width(230.0)
        .show(ctx, |ui| {
            ui.add_space(4.0);

            // Top-level nav links
            if ui
                .selectable_label(app.selected == Page::Dashboard, "📊  Dashboard")
                .clicked()
            {
                app.selected = Page::Dashboard;
            }
            if ui
                .selectable_label(app.selected == Page::Settings, "⚙  Settings")
                .clicked()
            {
                app.selected = Page::Settings;
            }

            ui.separator();

            // Scrollable account tree
            egui::ScrollArea::vertical()
                .id_salt("nav_scroll")
                .max_height(ui.available_height() - 48.0)
                .show(ui, |ui| {
                    egui::Frame::none()
                        .inner_margin(egui::Margin { left: 0.0, right: 10.0, top: 0.0, bottom: 0.0 })
                        .show(ui, |ui| {
                    if app.data.is_null() {
                        ui.weak("Open a file to see accounts");
                        return;
                    }

                    // Collect info up-front to avoid borrow issues
                    let accounts_info: Vec<(String, String, String)> =
                        if let Some(accounts) = app.data["accounts"].as_object() {
                            accounts
                                .iter()
                                .map(|(uuid, v)| {
                                    (
                                        uuid.clone(),
                                        v["type"].as_str().unwrap_or("").to_string(),
                                        v["name"].as_str().unwrap_or("Unnamed").to_string(),
                                    )
                                })
                                .collect()
                        } else {
                            vec![]
                        };

                    for (type_key, type_label) in ACCOUNT_TYPES {
                        let type_accounts: Vec<(String, String)> = accounts_info
                            .iter()
                            .filter(|(_, t, _)| t == type_key)
                            .map(|(uuid, _, name)| (uuid.clone(), name.clone()))
                            .collect();

                        let header_text =
                            format!("{} ({})", type_label, type_accounts.len());

                        egui::CollapsingHeader::new(header_text)
                            .id_salt(*type_key)
                            .show(ui, |ui| {
                                for (uuid, name) in &type_accounts {
                                    let selected =
                                        app.selected == Page::Account(uuid.clone());
                                    if ui.selectable_label(selected, name).clicked() {
                                        app.selected = Page::Account(uuid.clone());
                                    }
                                }
                                if ui.small_button("+ Add").clicked() {
                                    add_account(app, type_key);
                                }
                            });
                    }
                    }); // Frame
                });

            // Pinned footer with Open / Save
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Open...").clicked() {
                    open_file(app);
                }
                if ui.button("Save").clicked() {
                    save_file(app);
                }
                if ui.button("Save As...").clicked() {
                    save_file_as(app);
                }
            });

            if let Some(err) = &app.error.clone() {
                ui.add_space(4.0);
                ui.colored_label(
                    egui::Color32::from_rgb(220, 60, 60),
                    format!("⚠ {}", err),
                );
            }
        });
}

pub fn open_file(app: &mut FpApp) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .pick_file()
    {
        match std::fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(data) => {
                    app.data = data;
                    app.file_path = Some(path);
                    app.selected = Page::Dashboard;
                    app.error = None;
                    app.dirty = true;
                }
                Err(e) => {
                    app.error = Some(format!("Failed to parse file: {}", e));
                }
            },
            Err(e) => {
                app.error = Some(format!("Failed to open file: {}", e));
            }
        }
    }
}

pub fn save_file(app: &mut FpApp) {
    if let Some(path) = &app.file_path.clone() {
        write_file(app, path.to_str().unwrap_or(""));  // file_path already set; no update needed
    } else {
        save_file_as(app);
    }
}

pub fn save_file_as(app: &mut FpApp) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .save_file()
    {
        let path_str = path.to_str().unwrap_or("").to_string();
        if write_file(app, &path_str) {
            app.file_path = Some(path);
        }
    }
}

fn write_file(app: &mut FpApp, path: &str) -> bool {
    match serde_json::to_string_pretty(&app.data) {
        Ok(json) => {
            if let Err(e) = std::fs::write(path, json) {
                app.error = Some(format!("Failed to save file: {}", e));
                false
            } else {
                app.error = None;
                true
            }
        }
        Err(e) => {
            app.error = Some(format!("Failed to serialize data: {}", e));
            false
        }
    }
}

fn add_account(app: &mut FpApp, account_type: &str) {
    let uuid = Uuid::new_v4().to_string();
    let account = default_account(account_type);
    if let Some(accounts) = app.data["accounts"].as_object_mut() {
        accounts.insert(uuid.clone(), account);
        app.selected = Page::Account(uuid);
        app.dirty = true;
    }
}

fn default_account(account_type: &str) -> serde_json::Value {
    match account_type {
        "income" => json!({
            "type": "income",
            "name": "New Income",
            "table": {},
            "base": 0.0,
            "startIn": "yearStart",
            "endIn": "yearRetire",
            "raise": 3.0,
            "notes": null
        }),
        "ssa" => json!({
            "type": "ssa",
            "name": "Social Security",
            "table": {},
            "base": 0.0,
            "startIn": "yearRetire",
            "endIn": "yearDie",
            "notes": null
        }),
        "retirement" => json!({
            "type": "retirement",
            "name": "New Retirement",
            "table": {},
            "startIn": "yearStart",
            "endIn": "yearRetire",
            "startOut": "yearRetire",
            "endOut": "yearDie",
            "contributionValue": 10000.0,
            "contributionType": "fixed",
            "yearlyReturn": 6.0,
            "withdrawalType": "end_at_zero",
            "withdrawalValue": 0.0,
            "taxStatus": "contribute_pretax_taxed_when_used",
            "incomeLink": null,
            "matching": null,
            "notes": null
        }),
        "hsa" => json!({
            "type": "hsa",
            "name": "New HSA",
            "table": {},
            "startIn": "yearStart",
            "endIn": "yearRetire",
            "startOut": "yearStart",
            "endOut": "yearDie",
            "contributionValue": 3000.0,
            "contributionType": "fixed",
            "employerContribution": 0.0,
            "yearlyReturn": 5.0,
            "withdrawalType": "end_at_zero",
            "withdrawalValue": 0.0,
            "taxStatus": "contribute_pretax_untaxed_when_used",
            "notes": null
        }),
        "college" => json!({
            "type": "college",
            "name": "New College Fund",
            "table": {},
            "startIn": "yearStart",
            "endIn": 2036,
            "startOut": 2036,
            "endOut": 2040,
            "contributionValue": 5000.0,
            "contributionType": "fixed",
            "yearlyReturn": 5.0,
            "withdrawalType": "end_at_zero",
            "withdrawalValue": 0.0,
            "taxStatus": "contribute_taxed_earnings_untaxed_when_used",
            "notes": null
        }),
        "expense" => json!({
            "type": "expense",
            "name": "New Expense",
            "table": {},
            "startOut": "yearStart",
            "endOut": "yearDie",
            "expenseType": "fixed_with_inflation",
            "expenseValue": 1000.0,
            "isHealthcare": false,
            "scalesWithCol": true,
            "hsaLink": null,
            "notes": null
        }),
        "loan" => json!({
            "type": "loan",
            "name": "New Loan",
            "table": {},
            "startOut": "yearStart",
            "endOut": 2034,
            "paymentType": "fixed",
            "paymentValue": 500.0,
            "rate": 5.0,
            "notes": null
        }),
        "mortgage" => json!({
            "type": "mortgage",
            "name": "New Mortgage",
            "table": {},
            "startOut": "yearStart",
            "endOut": 2054,
            "paymentType": "fixed",
            "paymentValue": 2000.0,
            "rate": 6.5,
            "compoundTime": 12.0,
            "mortgageInsurance": 0.0,
            "ltvLimit": 80.0,
            "escrowValue": 300.0,
            "homeValue": 400000.0,
            "notes": null
        }),
        "savings" => json!({
            "type": "savings",
            "name": "New Savings",
            "table": {},
            "startIn": "yearStart",
            "endIn": "yearRetire",
            "startOut": "yearRetire",
            "endOut": "yearDie",
            "contributionValue": 5000.0,
            "contributionType": "fixed",
            "yearlyReturn": 5.0,
            "withdrawalType": "end_at_zero",
            "withdrawalValue": 0.0,
            "taxStatus": "contribute_taxed_earnings_taxed",
            "notes": null
        }),
        _ => json!({"type": account_type, "name": "New Account"}),
    }
}
