use eframe::egui;
use serde_json::{Value, json};

use crate::app::FpApp;
use crate::widgets;

mod college;
mod common;
mod expense;
mod hsa;
mod income;
mod loan;
mod mortgage;
mod retirement;
mod savings;
mod ssa;

pub(super) const CONTRIBUTION_OPTIONS: &[(&str, &str)] = &[
    ("fixed", "Fixed Amount"),
    ("percent_of_income", "% of Income"),
    ("fixed_with_inflation", "Fixed + Inflation"),
];

pub(super) const WITHDRAWAL_OPTIONS: &[(&str, &str)] = &[
    ("fixed", "Fixed Amount"),
    ("fixed_with_inflation", "Fixed + Inflation"),
    ("end_at_zero", "Draw Down to Zero"),
    ("col_frac_of_savings", "Fraction of Savings"),
    ("other", "Other"),
];

pub(super) const TAX_STATUS_OPTIONS: &[(&str, &str)] = &[
    (
        "contribute_taxed_earnings_untaxed_when_used",
        "Roth (post-tax, tax-free withdrawal)",
    ),
    ("contribute_taxed_earnings_taxed", "Taxed Both Ways"),
    (
        "contribute_pretax_taxed_when_used",
        "Traditional (pre-tax, taxed on withdrawal)",
    ),
    (
        "contribute_pretax_untaxed_when_used",
        "Tax-Free (HSA / 529)",
    ),
];

pub(super) const COLLEGE_TAX_OPTIONS: &[(&str, &str)] = &[(
    "contribute_taxed_earnings_untaxed_when_used",
    "Post-tax, tax-free withdrawal (529-style)",
)];

pub(super) const HSA_TAX_OPTIONS: &[(&str, &str)] = &[(
    "contribute_pretax_untaxed_when_used",
    "Pre-tax, tax-free withdrawal (HSA)",
)];

pub(super) const EXPENSE_OPTIONS: &[(&str, &str)] = &[
    ("fixed", "Fixed Amount"),
    ("fixed_with_inflation", "Fixed + Inflation"),
];

pub(super) const PAYMENT_OPTIONS: &[(&str, &str)] = &[
    ("fixed", "Fixed Amount"),
    ("fixed_with_inflation", "Fixed + Inflation"),
];

pub(super) fn form_grid(ui: &mut egui::Ui) -> egui::Grid {
    egui::Grid::new(ui.next_auto_id())
        .num_columns(2)
        .spacing([12.0, 5.0])
        .min_col_width(140.0)
}

pub fn show_account(app: &mut FpApp, ui: &mut egui::Ui, uuid: &str) {
    let account = app.data["accounts"][uuid].clone();
    if account.is_null() {
        ui.label("Account not found.");
        return;
    }

    let account_type = account["type"].as_str().unwrap_or("").to_string();
    let account_name = account["name"].as_str().unwrap_or("Unnamed").to_string();

    ui.horizontal(|ui| {
        ui.heading(&account_name);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .button(egui::RichText::new("Delete").color(egui::Color32::from_rgb(200, 60, 60)))
                .clicked()
            {
                app.confirm_delete = Some(uuid.to_string());
            }
        });
    });

    ui.add_space(6.0);

    let mut account = account;
    let mut changed = false;

    match account_type.as_str() {
        "income" => changed = income::show(ui, &mut account, uuid),
        "ssa" => changed = ssa::show(ui, &mut account, uuid),
        "retirement" => changed = retirement::show(ui, &mut account, uuid, app),
        "hsa" => changed = hsa::show(ui, &mut account, uuid),
        "college" => changed = college::show(ui, &mut account, uuid),
        "expense" => changed = expense::show(ui, &mut account, uuid),
        "loan" => changed = loan::show(ui, &mut account, uuid),
        "mortgage" => changed = mortgage::show(ui, &mut account, uuid),
        "savings" => changed = savings::show(ui, &mut account, uuid),
        _ => {
            ui.label(format!("Unknown account type: {}", account_type));
        }
    }

    // Per-account projection chart
    if let Some(plot_data) = app.plot_data.get(uuid)
        && !plot_data.is_empty()
    {
        ui.add_space(12.0);
        ui.separator();
        widgets::plot_datasets(ui, uuid, plot_data, "Projection", 480.0);
    }

    // Historical data table
    ui.add_space(8.0);
    ui.separator();
    ui.label("Historical Data:");
    if widgets::table_editor(ui, &mut account, "table") {
        changed = true;
    }

    if changed {
        app.data["accounts"][uuid] = account;
        app.dirty = true;
    }
}

pub(super) fn income_link_combo(ui: &mut egui::Ui, a: &mut Value, app: &FpApp, id: &str) -> bool {
    let current = a["incomeLink"].as_str().unwrap_or("").to_string();
    let mut accounts: Vec<(String, String)> = app.data["accounts"]
        .as_object()
        .map(|map| {
            map.iter()
                .filter(|(_, v)| v["type"].as_str() == Some("income"))
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v["name"].as_str().unwrap_or("Unnamed").to_string(),
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    // sort by name (uuid as tiebreak) so the dropdown order is predictable
    accounts.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    let label = if current.is_empty() {
        "None".to_string()
    } else {
        accounts
            .iter()
            .find(|(k, _)| k == &current)
            .map(|(_, n)| n.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    };

    let mut changed = false;
    egui::ComboBox::new(id, "")
        .selected_text(&label)
        .show_ui(ui, |ui| {
            if ui.selectable_label(current.is_empty(), "None").clicked() {
                a["incomeLink"] = Value::Null;
                changed = true;
            }
            for (uuid, name) in &accounts {
                if ui.selectable_label(current == *uuid, name).clicked() {
                    a["incomeLink"] = json!(uuid);
                    changed = true;
                }
            }
        });
    changed
}
