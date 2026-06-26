use eframe::egui;
use serde_json::{json, Value};

use crate::app::FpApp;
use crate::widgets;

const CONTRIBUTION_OPTIONS: &[(&str, &str)] = &[
    ("fixed", "Fixed"),
    ("percentOfIncome", "% of Income"),
    ("percentOfSalary", "% of Salary"),
];

const WITHDRAWAL_OPTIONS: &[(&str, &str)] = &[
    ("fixed", "Fixed"),
    ("percentOfSavings", "% of Savings"),
    ("fullWithdrawal", "Full Withdrawal"),
    ("fixedWithInflation", "Fixed + Inflation"),
];

const TAX_STATUS_OPTIONS: &[(&str, &str)] = &[
    ("contributePretax", "Pre-tax Contributions"),
    ("taxedNow", "Taxed Now (Roth-style)"),
    ("taxedWhenUsed", "Taxed When Used"),
    ("contributePretaxTaxedWhenUsed", "Pre-tax, Taxed on Withdrawal"),
];

const EXPENSE_OPTIONS: &[(&str, &str)] = &[
    ("fixed", "Fixed"),
    ("fixedWithInflation", "Fixed + Inflation"),
];

const PAYMENT_OPTIONS: &[(&str, &str)] = &[("fixed", "Fixed")];

pub fn show_account(app: &mut FpApp, ui: &mut egui::Ui, uuid: &str) {
    let account = app.data["accounts"][uuid].clone();
    if account.is_null() {
        ui.label("Account not found.");
        return;
    }

    let account_type = account["type"].as_str().unwrap_or("").to_string();
    let account_name = account["name"].as_str().unwrap_or("Unnamed").to_string();

    // Header with delete button
    let mut deleted = false;
    ui.horizontal(|ui| {
        ui.heading(&account_name);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .button(egui::RichText::new("Delete").color(egui::Color32::from_rgb(200, 60, 60)))
                .clicked()
            {
                deleted = true;
            }
        });
    });
    if deleted {
        if let Some(accounts) = app.data["accounts"].as_object_mut() {
            accounts.remove(uuid);
        }
        app.selected = crate::app::Page::Dashboard;
        app.dirty = true;
        return;
    }

    ui.add_space(6.0);

    let mut account = account;
    let mut changed = false;

    match account_type.as_str() {
        "income" => changed = income_form(ui, &mut account, uuid),
        "ssa" => changed = ssa_form(ui, &mut account, uuid),
        "retirement" => changed = retirement_form(ui, &mut account, uuid, app),
        "hsa" => changed = hsa_form(ui, &mut account, uuid),
        "college" => changed = college_form(ui, &mut account, uuid),
        "expense" => changed = expense_form(ui, &mut account, uuid, app),
        "loan" => changed = loan_form(ui, &mut account, uuid),
        "mortgage" => changed = mortgage_form(ui, &mut account, uuid),
        "savings" => changed = savings_form(ui, &mut account, uuid),
        _ => {
            ui.label(format!("Unknown account type: {}", account_type));
        }
    }

    // Per-account projection chart
    if let Some(plot_data) = app.plot_data.get(uuid) {
        if !plot_data.is_empty() {
            ui.add_space(12.0);
            ui.separator();
            widgets::plot_datasets(ui, uuid, plot_data, "Projection", 240.0);
        }
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

// ─── Income ─────────────────────────────────────────────────────────────────

fn income_form(ui: &mut egui::Ui, a: &mut Value, _uuid: &str) -> bool {
    let mut c = false;
    c |= widgets::string_field(ui, "Name:", a, "name");
    c |= widgets::f64_field(ui, "Base Pay ($):", a, "base", 500.0);
    c |= widgets::year_input(ui, "Start Year:", &mut a["startIn"]);
    c |= widgets::year_input(ui, "End Year:", &mut a["endIn"]);
    c |= widgets::percent_input(ui, "Yearly Raise:", &mut a["raise"]);
    c |= widgets::notes_field(ui, a);
    c
}

// ─── SSA ─────────────────────────────────────────────────────────────────────

fn ssa_form(ui: &mut egui::Ui, a: &mut Value, _uuid: &str) -> bool {
    let mut c = false;
    c |= widgets::string_field(ui, "Name:", a, "name");
    c |= widgets::f64_field(ui, "Base Benefit ($):", a, "base", 100.0);
    c |= widgets::year_input(ui, "Start Year:", &mut a["startIn"]);
    c |= widgets::year_input(ui, "End Year:", &mut a["endIn"]);
    c |= widgets::notes_field(ui, a);
    c
}

// ─── Retirement ──────────────────────────────────────────────────────────────

fn retirement_form(ui: &mut egui::Ui, a: &mut Value, uuid: &str, app: &FpApp) -> bool {
    let mut c = false;
    c |= widgets::string_field(ui, "Name:", a, "name");
    c |= widgets::year_input(ui, "Contribution Start:", &mut a["startIn"]);
    c |= widgets::year_input(ui, "Contribution End:", &mut a["endIn"]);
    c |= widgets::year_input(ui, "Withdrawal Start:", &mut a["startOut"]);
    c |= widgets::year_input(ui, "Withdrawal End:", &mut a["endOut"]);
    c |= widgets::combo_field(
        ui,
        "Contribution Type:",
        a,
        "contributionType",
        CONTRIBUTION_OPTIONS,
        &format!("ret_ct_{}", uuid),
    );
    c |= widgets::f64_field(ui, "Contribution Value:", a, "contributionValue", 500.0);
    c |= widgets::percent_input(ui, "Yearly Return (%):", &mut a["yearlyReturn"]);
    c |= widgets::combo_field(
        ui,
        "Withdrawal Type:",
        a,
        "withdrawalType",
        WITHDRAWAL_OPTIONS,
        &format!("ret_wt_{}", uuid),
    );
    c |= widgets::f64_field(ui, "Withdrawal Value:", a, "withdrawalValue", 500.0);
    c |= widgets::combo_field(
        ui,
        "Tax Status:",
        a,
        "taxStatus",
        TAX_STATUS_OPTIONS,
        &format!("ret_ts_{}", uuid),
    );

    // Income link
    ui.horizontal(|ui| {
        ui.label("Income Link:");
        income_link_combo(ui, a, app, &format!("ret_il_{}", uuid));
    });

    // Employer matching
    ui.add_space(4.0);
    let has_matching = !a["matching"].is_null();
    let mut enable_matching = has_matching;
    if ui.checkbox(&mut enable_matching, "Employer Matching").changed() {
        if enable_matching {
            a["matching"] = json!({
                "limit": {"constantFloat": 6.0},
                "amount": {"constantFloat": 50.0}
            });
        } else {
            a["matching"] = Value::Null;
        }
        c = true;
    }
    if enable_matching && !a["matching"].is_null() {
        ui.indent("matching_indent", |ui| {
            if widgets::percent_input(ui, "Match Limit (% of income):", &mut a["matching"]["limit"]) {
                c = true;
            }
            if widgets::percent_input(ui, "Match Amount (% of contribution):", &mut a["matching"]["amount"]) {
                c = true;
            }
        });
    }

    c |= widgets::notes_field(ui, a);
    c
}

// ─── HSA ─────────────────────────────────────────────────────────────────────

fn hsa_form(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    c |= widgets::string_field(ui, "Name:", a, "name");
    c |= widgets::year_input(ui, "Contribution Start:", &mut a["startIn"]);
    c |= widgets::year_input(ui, "Contribution End:", &mut a["endIn"]);
    c |= widgets::year_input(ui, "Withdrawal Start:", &mut a["startOut"]);
    c |= widgets::year_input(ui, "Withdrawal End:", &mut a["endOut"]);
    c |= widgets::combo_field(
        ui,
        "Contribution Type:",
        a,
        "contributionType",
        CONTRIBUTION_OPTIONS,
        &format!("hsa_ct_{}", uuid),
    );
    c |= widgets::f64_field(ui, "Contribution Value:", a, "contributionValue", 100.0);
    c |= widgets::f64_field(ui, "Employer Contribution ($):", a, "employerContribution", 100.0);
    c |= widgets::percent_input(ui, "Yearly Return (%):", &mut a["yearlyReturn"]);
    c |= widgets::combo_field(
        ui,
        "Tax Status:",
        a,
        "taxStatus",
        TAX_STATUS_OPTIONS,
        &format!("hsa_ts_{}", uuid),
    );
    c |= widgets::notes_field(ui, a);
    c
}

// ─── College ─────────────────────────────────────────────────────────────────

fn college_form(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    c |= widgets::string_field(ui, "Name:", a, "name");
    c |= widgets::year_input(ui, "Contribution Start:", &mut a["startIn"]);
    c |= widgets::year_input(ui, "Contribution End:", &mut a["endIn"]);
    c |= widgets::year_input(ui, "Withdrawal Start:", &mut a["startOut"]);
    c |= widgets::year_input(ui, "Withdrawal End:", &mut a["endOut"]);
    c |= widgets::combo_field(
        ui,
        "Contribution Type:",
        a,
        "contributionType",
        CONTRIBUTION_OPTIONS,
        &format!("col_ct_{}", uuid),
    );
    c |= widgets::f64_field(ui, "Contribution Value:", a, "contributionValue", 500.0);
    c |= widgets::percent_input(ui, "Yearly Return (%):", &mut a["yearlyReturn"]);
    c |= widgets::combo_field(
        ui,
        "Withdrawal Type:",
        a,
        "withdrawalType",
        WITHDRAWAL_OPTIONS,
        &format!("col_wt_{}", uuid),
    );
    c |= widgets::f64_field(ui, "Withdrawal Value:", a, "withdrawalValue", 500.0);
    c |= widgets::combo_field(
        ui,
        "Tax Status:",
        a,
        "taxStatus",
        TAX_STATUS_OPTIONS,
        &format!("col_ts_{}", uuid),
    );
    c |= widgets::notes_field(ui, a);
    c
}

// ─── Expense ─────────────────────────────────────────────────────────────────

fn expense_form(ui: &mut egui::Ui, a: &mut Value, uuid: &str, app: &FpApp) -> bool {
    let mut c = false;
    c |= widgets::string_field(ui, "Name:", a, "name");
    c |= widgets::year_input(ui, "Start Year:", &mut a["startOut"]);
    c |= widgets::year_input(ui, "End Year:", &mut a["endOut"]);
    c |= widgets::combo_field(
        ui,
        "Expense Type:",
        a,
        "expenseType",
        EXPENSE_OPTIONS,
        &format!("exp_et_{}", uuid),
    );
    c |= widgets::f64_field(ui, "Expense Value ($):", a, "expenseValue", 100.0);

    let mut is_hc = a["isHealthcare"].as_bool().unwrap_or(false);
    if ui.checkbox(&mut is_hc, "Healthcare expense").changed() {
        a["isHealthcare"] = json!(is_hc);
        c = true;
    }

    let mut scales = a["scalesWithCol"].as_bool().unwrap_or(true);
    if ui.checkbox(&mut scales, "Scales with retirement cost-of-living factor").changed() {
        a["scalesWithCol"] = json!(scales);
        c = true;
    }

    ui.horizontal(|ui| {
        ui.label("HSA Link:");
        hsa_link_combo(ui, a, app, &format!("exp_hl_{}", uuid));
    });

    c |= widgets::notes_field(ui, a);
    c
}

// ─── Loan ────────────────────────────────────────────────────────────────────

fn loan_form(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    c |= widgets::string_field(ui, "Name:", a, "name");
    c |= widgets::year_input(ui, "Start Year:", &mut a["startOut"]);
    c |= widgets::year_input(ui, "End Year:", &mut a["endOut"]);
    c |= widgets::combo_field(
        ui,
        "Payment Type:",
        a,
        "paymentType",
        PAYMENT_OPTIONS,
        &format!("loan_pt_{}", uuid),
    );
    c |= widgets::f64_field(ui, "Monthly Payment ($):", a, "paymentValue", 50.0);
    c |= widgets::f64_field(ui, "Interest Rate (%):", a, "rate", 0.1);
    c |= widgets::notes_field(ui, a);
    c
}

// ─── Mortgage ────────────────────────────────────────────────────────────────

fn mortgage_form(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    c |= widgets::string_field(ui, "Name:", a, "name");
    c |= widgets::year_input(ui, "Start Year:", &mut a["startOut"]);
    c |= widgets::year_input(ui, "End Year:", &mut a["endOut"]);
    c |= widgets::combo_field(
        ui,
        "Payment Type:",
        a,
        "paymentType",
        PAYMENT_OPTIONS,
        &format!("mort_pt_{}", uuid),
    );
    c |= widgets::f64_field(ui, "Monthly Payment ($):", a, "paymentValue", 50.0);
    c |= widgets::f64_field(ui, "Interest Rate (%):", a, "rate", 0.05);
    c |= widgets::f64_field(ui, "Compound Periods/Year:", a, "compoundTime", 1.0);
    c |= widgets::f64_field(ui, "Mortgage Insurance ($):", a, "mortgageInsurance", 10.0);
    c |= widgets::f64_field(ui, "LTV Limit (%):", a, "ltvLimit", 1.0);
    c |= widgets::f64_field(ui, "Escrow ($/mo):", a, "escrowValue", 10.0);
    c |= widgets::f64_field(ui, "Home Value ($):", a, "homeValue", 5000.0);
    c |= widgets::notes_field(ui, a);
    c
}

// ─── Savings ─────────────────────────────────────────────────────────────────

fn savings_form(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    c |= widgets::string_field(ui, "Name:", a, "name");
    c |= widgets::year_input(ui, "Contribution Start:", &mut a["startIn"]);
    c |= widgets::year_input(ui, "Contribution End:", &mut a["endIn"]);
    c |= widgets::year_input(ui, "Withdrawal Start:", &mut a["startOut"]);
    c |= widgets::year_input(ui, "Withdrawal End:", &mut a["endOut"]);
    c |= widgets::combo_field(
        ui,
        "Contribution Type:",
        a,
        "contributionType",
        CONTRIBUTION_OPTIONS,
        &format!("sav_ct_{}", uuid),
    );
    c |= widgets::f64_field(ui, "Contribution Value:", a, "contributionValue", 500.0);
    c |= widgets::percent_input(ui, "Yearly Return (%):", &mut a["yearlyReturn"]);
    c |= widgets::combo_field(
        ui,
        "Withdrawal Type:",
        a,
        "withdrawalType",
        WITHDRAWAL_OPTIONS,
        &format!("sav_wt_{}", uuid),
    );
    c |= widgets::f64_field(ui, "Withdrawal Value:", a, "withdrawalValue", 500.0);
    c |= widgets::combo_field(
        ui,
        "Tax Status:",
        a,
        "taxStatus",
        TAX_STATUS_OPTIONS,
        &format!("sav_ts_{}", uuid),
    );
    c |= widgets::notes_field(ui, a);
    c
}

// ─── Helper widgets ──────────────────────────────────────────────────────────

fn income_link_combo(ui: &mut egui::Ui, a: &mut Value, app: &FpApp, id: &str) {
    let current_link = a["incomeLink"].as_str().unwrap_or("").to_string();
    let income_accounts: Vec<(String, String)> =
        if let Some(accounts) = app.data["accounts"].as_object() {
            accounts
                .iter()
                .filter(|(_, v)| v["type"].as_str() == Some("income"))
                .map(|(k, v)| (k.clone(), v["name"].as_str().unwrap_or("Unnamed").to_string()))
                .collect()
        } else {
            vec![]
        };

    let current_label = if current_link.is_empty() {
        "None".to_string()
    } else {
        income_accounts
            .iter()
            .find(|(uuid, _)| uuid == &current_link)
            .map(|(_, name)| name.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    };

    egui::ComboBox::new(id, "")
        .selected_text(&current_label)
        .show_ui(ui, |ui| {
            if ui.selectable_label(current_link.is_empty(), "None").clicked() {
                a["incomeLink"] = Value::Null;
            }
            for (uuid, name) in &income_accounts {
                if ui.selectable_label(current_link == *uuid, name).clicked() {
                    a["incomeLink"] = json!(uuid);
                }
            }
        });
}

fn hsa_link_combo(ui: &mut egui::Ui, a: &mut Value, app: &FpApp, id: &str) {
    let current_link = a["hsaLink"].as_str().unwrap_or("").to_string();
    let hsa_accounts: Vec<(String, String)> =
        if let Some(accounts) = app.data["accounts"].as_object() {
            accounts
                .iter()
                .filter(|(_, v)| v["type"].as_str() == Some("hsa"))
                .map(|(k, v)| (k.clone(), v["name"].as_str().unwrap_or("Unnamed").to_string()))
                .collect()
        } else {
            vec![]
        };

    let current_label = if current_link.is_empty() {
        "None".to_string()
    } else {
        hsa_accounts
            .iter()
            .find(|(uuid, _)| uuid == &current_link)
            .map(|(_, name)| name.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    };

    egui::ComboBox::new(id, "")
        .selected_text(&current_label)
        .show_ui(ui, |ui| {
            if ui.selectable_label(current_link.is_empty(), "None").clicked() {
                a["hsaLink"] = Value::Null;
            }
            for (uuid, name) in &hsa_accounts {
                if ui.selectable_label(current_link == *uuid, name).clicked() {
                    a["hsaLink"] = json!(uuid);
                }
            }
        });
}
