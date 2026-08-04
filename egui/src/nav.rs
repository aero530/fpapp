use eframe::egui;
use serde_json::json;
use uuid::Uuid;

use crate::app::{FpApp, Page};
use crate::platform::{self, SaveMode};

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

pub fn show_nav(app: &mut FpApp, ui: &mut egui::Ui) {
    egui::Panel::left("nav_panel")
        .exact_size(230.0)
        .show(ui, |ui| {
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
                    egui::Frame::new()
                        .inner_margin(egui::Margin {
                            left: 0,
                            right: 10,
                            top: 0,
                            bottom: 0,
                        })
                        .show(ui, |ui| {
                            if app.data.is_null() {
                                ui.weak("Open a file to see accounts");
                                return;
                            }

                            // Group accounts by type in a single pass (collected
                            // up-front to avoid borrow issues while rendering)
                            let mut groups: std::collections::HashMap<
                                String,
                                Vec<(String, String)>,
                            > = std::collections::HashMap::new();
                            if let Some(accounts) = app.data["accounts"].as_object() {
                                for (uuid, v) in accounts {
                                    if let Some(type_key) = v["type"].as_str() {
                                        groups.entry(type_key.to_string()).or_default().push((
                                            uuid.clone(),
                                            v["name"].as_str().unwrap_or("Unnamed").to_string(),
                                        ));
                                    }
                                }
                            }

                            for (type_key, type_label) in ACCOUNT_TYPES {
                                let mut type_accounts =
                                    groups.remove(*type_key).unwrap_or_default();
                                // sort by name (uuid as tiebreak) — map order is
                                // uuid order, which looks random to the user
                                type_accounts.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

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

            // Pinned footer with New / Open / Save
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("New").clicked() {
                    new_file(app);
                }
                if ui.button("Open...").clicked() {
                    open_file(app);
                }
            });
            let can_save = !app.data.is_null();
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(can_save, egui::Button::new("Save"))
                    .clicked()
                {
                    save_file(app);
                }
                if ui
                    .add_enabled(can_save, egui::Button::new("Save As..."))
                    .clicked()
                {
                    save_file_as(app);
                }
            });

            if let Some(err) = &app.error.clone() {
                ui.add_space(4.0);
                ui.colored_label(egui::Color32::from_rgb(220, 60, 60), format!("⚠ {}", err));
            } else if let Some(status) = &app.status.clone() {
                // Confirmation for the last file operation.  Mostly for the web
                // build, where a save is otherwise silent.
                ui.add_space(4.0);
                ui.weak(status);
            }

            // Engine warnings from the last analysis (misconfigurations that
            // were worked around rather than aborting the run)
            if !app.warnings.is_empty() {
                const MAX_SHOWN: usize = 4;
                ui.add_space(4.0);
                for warning in app.warnings.iter().take(MAX_SHOWN) {
                    ui.colored_label(
                        egui::Color32::from_rgb(220, 150, 40),
                        format!("⚠ {}", warning),
                    );
                }
                if app.warnings.len() > MAX_SHOWN {
                    ui.weak(format!("… and {} more", app.warnings.len() - MAX_SHOWN));
                }
            }
        });
}

/// A new plan: default settings, no accounts.
fn default_plan(year_start: u32) -> serde_json::Value {
    json!({
        "settings": {
            "ageRetire": 65,
            "ageDie": 90,
            "yearBorn": year_start - 35,
            "yearStart": year_start,
            "inflationBase": 3.0,
            "taxIncome": 22.0,
            "taxCapitalGains": 15.0,
            "retirementCostOfLiving": 80.0,
            "ssa": {
                "breakpoints": { "low": 25000.0, "high": 34000.0 },
                "taxableIncomePercentage": { "low": 50.0, "high": 85.0 }
            }
        },
        "accounts": {}
    })
}

/// Start a new plan with sensible default settings and no accounts.
pub fn new_file(app: &mut FpApp) {
    app.data = default_plan(platform::current_year());
    app.file_path = None;
    app.file_name = None;
    app.selected = Page::Settings;
    app.error = None;
    app.status = None;
    app.dirty = true;
}

/// Ask for a plan file.  The read lands in `FpApp` via the file-event queue —
/// on the web the picker is asynchronous, so it is not open yet when this
/// returns.
pub fn open_file(app: &mut FpApp) {
    app.file_io.open();
}

pub fn save_file(app: &mut FpApp) {
    request_save(app, SaveMode::InPlace);
}

pub fn save_file_as(app: &mut FpApp) {
    request_save(app, SaveMode::Prompt);
}

fn request_save(app: &mut FpApp, mode: SaveMode) {
    match serde_json::to_string_pretty(&app.data) {
        Ok(json) => {
            app.file_io.save(
                mode,
                app.file_path.as_deref(),
                &app.suggested_file_name(),
                json,
            );
        }
        Err(e) => {
            app.error = Some(format!("Failed to serialize data: {}", e));
        }
    }
}

fn add_account(app: &mut FpApp, account_type: &str) {
    if !app.data.is_object() {
        // No plan open (the tree is not shown in this state, but guard anyway)
        return;
    }
    // Older or hand-edited files may lack the accounts object entirely —
    // create it rather than silently dropping the new account
    if !app.data["accounts"].is_object() {
        app.data["accounts"] = json!({});
    }
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
        // no "table": the SSA account has no historical-data field in the engine
        "ssa" => json!({
            "type": "ssa",
            "name": "Social Security",
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
            "taxStatus": "contribute_pretax_untaxed_when_used",
            "notes": null
        }),
        "college" => json!({
            "type": "college",
            "name": "New College Fund",
            "table": {},
            // contributions until college starts ~16 years out, then draw down over 4 years
            "startIn": "yearStart",
            "endIn": {"base": "yearStart", "delta": 16},
            "startOut": {"base": "yearStart", "delta": 16},
            "endOut": {"base": "yearStart", "delta": 20},
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
            "notes": null
        }),
        "loan" => json!({
            "type": "loan",
            "name": "New Loan",
            "table": {},
            "startOut": "yearStart",
            "endOut": {"base": "yearStart", "delta": 10},
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
            "endOut": {"base": "yearStart", "delta": 30},
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_plan_matches_the_engine_schema() {
        // The "New" plan template must deserialize and simulate cleanly, so a
        // brand-new plan never greets the user with a parse error.
        let plan = default_plan(2026);
        let data: accounts::UserData<accounts::SimAccount> =
            serde_json::from_value(plan).expect("default plan does not deserialize");
        accounts::run(data).expect("default plan does not simulate");
    }

    #[test]
    fn default_account_templates_match_the_accounts_schema() {
        // Every "+ Add" template must deserialize into the engine's
        // SimAccount — this catches schema drift between the UI templates
        // and the accounts crate (removed/renamed fields).
        for (type_key, _) in ACCOUNT_TYPES {
            let template = default_account(type_key);
            let result: Result<accounts::SimAccount, _> = serde_json::from_value(template.clone());
            assert!(
                result.is_ok(),
                "default template for '{}' does not deserialize: {}\n{}",
                type_key,
                result.err().unwrap(),
                serde_json::to_string_pretty(&template).unwrap()
            );
        }
    }
}
