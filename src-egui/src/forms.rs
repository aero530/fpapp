use eframe::egui;
use serde_json::{json, Value};

use crate::app::FpApp;
use crate::widgets;

const CONTRIBUTION_OPTIONS: &[(&str, &str)] = &[
    ("fixed", "Fixed Amount"),
    ("percent_of_income", "% of Income"),
    ("fixed_with_inflation", "Fixed + Inflation"),
];

const WITHDRAWAL_OPTIONS: &[(&str, &str)] = &[
    ("fixed", "Fixed Amount"),
    ("fixed_with_inflation", "Fixed + Inflation"),
    ("end_at_zero", "Draw Down to Zero"),
    ("col_frac_of_savings", "Fraction of Savings"),
    ("other", "Other"),
];

const TAX_STATUS_OPTIONS: &[(&str, &str)] = &[
    ("contribute_taxed_earnings_untaxed_when_used", "Roth (post-tax, tax-free withdrawal)"),
    ("contribute_taxed_earnings_taxed", "Taxed Both Ways"),
    ("contribute_pretax_taxed_when_used", "Traditional (pre-tax, taxed on withdrawal)"),
    ("contribute_pretax_untaxed_when_used", "Tax-Free (HSA / 529)"),
];

const COLLEGE_TAX_OPTIONS: &[(&str, &str)] = &[
    ("contribute_taxed_earnings_untaxed_when_used", "Post-tax, tax-free withdrawal (529-style)"),
];

const EXPENSE_OPTIONS: &[(&str, &str)] = &[
    ("fixed", "Fixed Amount"),
    ("fixed_with_inflation", "Fixed + Inflation"),
];

const PAYMENT_OPTIONS: &[(&str, &str)] = &[
    ("fixed", "Fixed Amount"),
    ("fixed_with_inflation", "Fixed + Inflation"),
];

fn form_grid(ui: &mut egui::Ui) -> egui::Grid {
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
            // Clear any stale incomeLink / hsaLink references to the deleted account
            for (_, acct) in accounts.iter_mut() {
                if acct["incomeLink"].as_str() == Some(uuid) {
                    acct["incomeLink"] = Value::Null;
                }
                if acct["hsaLink"].as_str() == Some(uuid) {
                    acct["hsaLink"] = Value::Null;
                }
            }
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
            widgets::plot_datasets(ui, uuid, plot_data, "Projection", 480.0);
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
    form_grid(ui).show(ui, |ui| {
        c |= widgets::string_field(ui, "Name:", a, "name",
            "Human friendly name for the account");
        ui.end_row();
        c |= widgets::f64_field(ui, "Base Pay ($):", a, "base", 500.0,
            "Base pay (with bonuses) [in today's dollars]");
        ui.end_row();
        c |= widgets::year_input(ui, "Start Year:", &mut a["startIn"],
            "Calendar year when money starts being earned by this account");
        ui.end_row();
        c |= widgets::year_input(ui, "End Year:", &mut a["endIn"],
            "Calendar year when money stops being earned by this account");
        ui.end_row();
        c |= widgets::percent_input(ui, "Yearly Raise:", &mut a["raise"],
            "Yearly increase in income as a percent");
        ui.end_row();
        c |= widgets::notes_field(ui, a);
        ui.end_row();
    });
    c
}

// ─── SSA ─────────────────────────────────────────────────────────────────────

fn ssa_form(ui: &mut egui::Ui, a: &mut Value, _uuid: &str) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= widgets::string_field(ui, "Name:", a, "name",
            "Human friendly name for the account");
        ui.end_row();
        c |= widgets::f64_field(ui, "Base Benefit ($):", a, "base", 100.0,
            "Base income from Social Security");
        ui.end_row();
        c |= widgets::year_input(ui, "Start Year:", &mut a["startIn"],
            "When Social Security payments will start");
        ui.end_row();
        c |= widgets::year_input(ui, "End Year:", &mut a["endIn"],
            "When Social Security payments will stop");
        ui.end_row();
        c |= widgets::notes_field(ui, a);
        ui.end_row();
    });
    c
}

// ─── Retirement ──────────────────────────────────────────────────────────────

fn retirement_form(ui: &mut egui::Ui, a: &mut Value, uuid: &str, app: &FpApp) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= widgets::string_field(ui, "Name:", a, "name",
            "Human friendly name for the account");
        ui.end_row();
        c |= widgets::year_input(ui, "Contribution Start:", &mut a["startIn"],
            "When money will start going into this account");
        ui.end_row();
        c |= widgets::year_input(ui, "Contribution End:", &mut a["endIn"],
            "When money will stop going into this account");
        ui.end_row();
        c |= widgets::year_input(ui, "Withdrawal Start:", &mut a["startOut"],
            "When money will start coming out of this account");
        ui.end_row();
        c |= widgets::year_input(ui, "Withdrawal End:", &mut a["endOut"],
            "When money will stop coming out of this account");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Contribution Type:", a, "contributionType",
            CONTRIBUTION_OPTIONS, &format!("ret_ct_{}", uuid),
            "How money is put into the account:\n\
             Fixed Amount — fixed dollar amount\n\
             % of Income — percent of total income\n\
             Fixed + Inflation — fixed amount adjusted for inflation",
        );
        ui.end_row();
        c |= widgets::f64_field(ui, "Contribution Value:", a, "contributionValue", 500.0,
            "Amount put into this account every year [in today's dollars]");
        ui.end_row();
        c |= widgets::percent_input(ui, "Yearly Return (%):", &mut a["yearlyReturn"],
            "Percent interest earned each year");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Withdrawal Type:", a, "withdrawalType",
            WITHDRAWAL_OPTIONS, &format!("ret_wt_{}", uuid),
            "How money is taken out of the account:\n\
             Fixed Amount — fixed dollar amount\n\
             Fixed + Inflation — fixed amount adjusted for inflation\n\
             Draw Down to Zero — equal amounts so balance reaches zero at end\n\
             Fraction of Savings — proportional to total savings",
        );
        ui.end_row();
        c |= widgets::f64_field(ui, "Withdrawal Value:", a, "withdrawalValue", 500.0,
            "How much money to take out per year [in today's dollars]");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Tax Status:", a, "taxStatus",
            TAX_STATUS_OPTIONS, &format!("ret_ts_{}", uuid),
            "How taxes impact this account:\n\
             Roth — paid with after-tax income, tax-free withdrawals\n\
             Taxed Both Ways — taxed on contributions and withdrawals\n\
             Traditional — pre-tax contributions, taxed on withdrawal\n\
             Tax-Free (HSA/529) — pre-tax contributions, tax-free withdrawals",
        );
        ui.end_row();

        ui.label("Income Link:")
            .on_hover_text("Link to an income account to base percentage contributions on that account's value");
        c |= income_link_combo(ui, a, app, &format!("ret_il_{}", uuid));
        ui.end_row();

        c |= widgets::notes_field(ui, a);
        ui.end_row();
    });

    ui.add_space(4.0);
    let has_matching = !a["matching"].is_null();
    let mut enable_matching = has_matching;
    if ui.checkbox(&mut enable_matching, "Employer Matching")
        .on_hover_text("Does your employer match your contributions into this account?")
        .changed()
    {
        a["matching"] = if enable_matching {
            json!({ "limit": 6.0, "amount": 50.0 })
        } else {
            Value::Null
        };
        c = true;
    }
    if enable_matching && !a["matching"].is_null() {
        ui.indent("matching", |ui| {
            form_grid(ui).show(ui, |ui| {
                if widgets::percent_input(ui, "Limit (% of income):", &mut a["matching"]["limit"],
                    "Percent of income at which the employer stops matching") {
                    c = true;
                }
                ui.end_row();
                if widgets::percent_input(ui, "Amount (% of contribution):", &mut a["matching"]["amount"],
                    "Percent of your contribution that the employer matches") {
                    c = true;
                }
                ui.end_row();
            });
        });
    }

    c
}

// ─── HSA ─────────────────────────────────────────────────────────────────────

fn hsa_form(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= widgets::string_field(ui, "Name:", a, "name",
            "Human friendly name for the account");
        ui.end_row();
        c |= widgets::year_input(ui, "Contribution Start:", &mut a["startIn"],
            "When money will start going into this account");
        ui.end_row();
        c |= widgets::year_input(ui, "Contribution End:", &mut a["endIn"],
            "When money will stop going into this account");
        ui.end_row();
        c |= widgets::year_input(ui, "Withdrawal Start:", &mut a["startOut"],
            "When money will start coming out of this account");
        ui.end_row();
        c |= widgets::year_input(ui, "Withdrawal End:", &mut a["endOut"],
            "When money will stop coming out of this account");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Contribution Type:", a, "contributionType",
            CONTRIBUTION_OPTIONS, &format!("hsa_ct_{}", uuid),
            "How money is put into the account:\n\
             Fixed Amount — fixed dollar amount\n\
             % of Income — percent of total income\n\
             Fixed + Inflation — fixed amount adjusted for inflation",
        );
        ui.end_row();
        c |= widgets::f64_field(ui, "Contribution Value:", a, "contributionValue", 100.0,
            "Amount put into this account every year [in today's dollars]");
        ui.end_row();
        c |= widgets::f64_field(ui, "Employer Contribution ($):", a, "employerContribution", 100.0,
            "Employer contributions to this account as a yearly dollar amount [in today's dollars]");
        ui.end_row();
        c |= widgets::percent_input(ui, "Yearly Return (%):", &mut a["yearlyReturn"],
            "Percent interest earned each year");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Tax Status:", a, "taxStatus",
            TAX_STATUS_OPTIONS, &format!("hsa_ts_{}", uuid),
            "How taxes impact this account:\n\
             Roth — paid with after-tax income, tax-free withdrawals\n\
             Taxed Both Ways — taxed on contributions and withdrawals\n\
             Traditional — pre-tax contributions, taxed on withdrawal\n\
             Tax-Free (HSA/529) — pre-tax contributions, tax-free withdrawals",
        );
        ui.end_row();
        c |= widgets::notes_field(ui, a);
        ui.end_row();
    });
    c
}

// ─── College ─────────────────────────────────────────────────────────────────

fn college_form(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= widgets::string_field(ui, "Name:", a, "name",
            "Human friendly name for the account");
        ui.end_row();
        c |= widgets::year_input(ui, "Contribution Start:", &mut a["startIn"],
            "When money will start going into this account");
        ui.end_row();
        c |= widgets::year_input(ui, "Contribution End:", &mut a["endIn"],
            "When money will stop going into this account");
        ui.end_row();
        c |= widgets::year_input(ui, "Withdrawal Start:", &mut a["startOut"],
            "When money will start coming out of this account");
        ui.end_row();
        c |= widgets::year_input(ui, "Withdrawal End:", &mut a["endOut"],
            "When money will stop coming out of this account");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Contribution Type:", a, "contributionType",
            CONTRIBUTION_OPTIONS, &format!("col_ct_{}", uuid),
            "How money is put into the account:\n\
             Fixed Amount — fixed dollar amount\n\
             % of Income — percent of total income\n\
             Fixed + Inflation — fixed amount adjusted for inflation",
        );
        ui.end_row();
        c |= widgets::f64_field(ui, "Contribution Value:", a, "contributionValue", 500.0,
            "Amount put into this account every year [in today's dollars]");
        ui.end_row();
        c |= widgets::percent_input(ui, "Yearly Return (%):", &mut a["yearlyReturn"],
            "Percent interest earned each year");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Withdrawal Type:", a, "withdrawalType",
            WITHDRAWAL_OPTIONS, &format!("col_wt_{}", uuid),
            "How money is taken out of the account:\n\
             Fixed Amount — fixed dollar amount\n\
             Fixed + Inflation — fixed amount adjusted for inflation\n\
             Draw Down to Zero — equal amounts so balance reaches zero at end\n\
             Fraction of Savings — proportional to total savings",
        );
        ui.end_row();
        c |= widgets::f64_field(ui, "Withdrawal Value:", a, "withdrawalValue", 500.0,
            "How much money to take out per year [in today's dollars]");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Tax Status:", a, "taxStatus",
            COLLEGE_TAX_OPTIONS, &format!("col_ts_{}", uuid),
            "How taxes impact this account:\n\
             Post-tax contributions, tax-free withdrawals for qualified education expenses (529-style)",
        );
        ui.end_row();
        c |= widgets::notes_field(ui, a);
        ui.end_row();
    });
    c
}

// ─── Expense ─────────────────────────────────────────────────────────────────

fn expense_form(ui: &mut egui::Ui, a: &mut Value, uuid: &str, app: &FpApp) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= widgets::string_field(ui, "Name:", a, "name",
            "Human friendly name for the account");
        ui.end_row();
        c |= widgets::year_input(ui, "Start Year:", &mut a["startOut"],
            "When this expense will start");
        ui.end_row();
        c |= widgets::year_input(ui, "End Year:", &mut a["endOut"],
            "When this expense will stop");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Expense Type:", a, "expenseType",
            EXPENSE_OPTIONS, &format!("exp_et_{}", uuid),
            "How the expense value is interpreted:\n\
             Fixed Amount — fixed dollar amount each year\n\
             Fixed + Inflation — fixed amount adjusted for inflation",
        );
        ui.end_row();
        c |= widgets::f64_field(ui, "Expense Value ($):", a, "expenseValue", 100.0,
            "Yearly cost of the expense [in today's dollars]");
        ui.end_row();

        ui.label("");
        let mut is_hc = a["isHealthcare"].as_bool().unwrap_or(false);
        if ui.checkbox(&mut is_hc, "Healthcare expense")
            .on_hover_text("Is this a healthcare cost that should be paid out of an HSA account?")
            .changed()
        {
            a["isHealthcare"] = json!(is_hc);
            c = true;
        }
        ui.end_row();

        ui.label("");
        let mut scales = a["scalesWithCol"].as_bool().unwrap_or(true);
        if ui.checkbox(&mut scales, "Scales with retirement cost-of-living factor")
            .on_hover_text("Should this expense scale proportionally with the retirement cost-of-living setting?")
            .changed()
        {
            a["scalesWithCol"] = json!(scales);
            c = true;
        }
        ui.end_row();

        ui.label("HSA Link:")
            .on_hover_text("Link to an HSA account that will pay for this healthcare expense");
        c |= hsa_link_combo(ui, a, app, &format!("exp_hl_{}", uuid));
        ui.end_row();

        c |= widgets::notes_field(ui, a);
        ui.end_row();
    });
    c
}

// ─── Loan ────────────────────────────────────────────────────────────────────

fn loan_form(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= widgets::string_field(ui, "Name:", a, "name",
            "Human friendly name for the account");
        ui.end_row();
        c |= widgets::year_input(ui, "Start Year:", &mut a["startOut"],
            "When loan payments will start");
        ui.end_row();
        c |= widgets::year_input(ui, "End Year:", &mut a["endOut"],
            "When loan payments will end");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Payment Type:", a, "paymentType",
            PAYMENT_OPTIONS, &format!("loan_pt_{}", uuid),
            "How the payment value is interpreted:\n\
             Fixed Amount — fixed dollar amount each year\n\
             Fixed + Inflation — fixed amount adjusted for inflation",
        );
        ui.end_row();
        c |= widgets::f64_field(ui, "Annual Payment ($):", a, "paymentValue", 500.0,
            "Total amount paid per year toward this loan [in today's dollars]");
        ui.end_row();
        c |= widgets::f64_field(ui, "Interest Rate (%):", a, "rate", 0.1,
            "Interest rate on borrowed money (APR, compounded monthly)");
        ui.end_row();
        c |= widgets::notes_field(ui, a);
        ui.end_row();
    });
    c
}

// ─── Mortgage ────────────────────────────────────────────────────────────────

fn mortgage_form(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= widgets::string_field(ui, "Name:", a, "name",
            "Human friendly name for the account");
        ui.end_row();
        c |= widgets::year_input(ui, "Start Year:", &mut a["startOut"],
            "When mortgage payments will start");
        ui.end_row();
        c |= widgets::year_input(ui, "End Year:", &mut a["endOut"],
            "When mortgage payments will end (loan payoff year)");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Payment Type:", a, "paymentType",
            PAYMENT_OPTIONS, &format!("mort_pt_{}", uuid),
            "How the payment value is interpreted:\n\
             Fixed Amount — fixed dollar amount each year\n\
             Fixed + Inflation — fixed amount adjusted for inflation",
        );
        ui.end_row();
        c |= widgets::f64_field(ui, "Annual Payment ($):", a, "paymentValue", 500.0,
            "Total amount paid per year including principal, interest, escrow, and mortgage insurance [in today's dollars]");
        ui.end_row();
        c |= widgets::f64_field(ui, "Interest Rate (%):", a, "rate", 0.05,
            "Interest rate on borrowed money (APR, compounded based on compound periods setting)");
        ui.end_row();
        c |= widgets::f64_field(ui, "Compound Periods/Year:", a, "compoundTime", 1.0,
            "Number of times per year interest is compounded (1=yearly, 12=monthly)");
        ui.end_row();
        c |= widgets::f64_field(ui, "Mortgage Insurance ($):", a, "mortgageInsurance", 10.0,
            "Yearly mortgage insurance payment [in today's dollars]");
        ui.end_row();
        c |= widgets::f64_field(ui, "LTV Limit (%):", a, "ltvLimit", 1.0,
            "Loan-to-value ratio at which mortgage insurance is no longer required");
        ui.end_row();
        c |= widgets::f64_field(ui, "Escrow ($/yr):", a, "escrowValue", 100.0,
            "Yearly amount set aside for property taxes [in today's dollars]");
        ui.end_row();
        c |= widgets::f64_field(ui, "Home Value ($):", a, "homeValue", 5000.0,
            "Current value of the home, used to compute loan-to-value ratio [in today's dollars]");
        ui.end_row();
        c |= widgets::notes_field(ui, a);
        ui.end_row();
    });
    c
}

// ─── Savings ─────────────────────────────────────────────────────────────────

fn savings_form(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= widgets::string_field(ui, "Name:", a, "name",
            "Human friendly name for the account");
        ui.end_row();
        c |= widgets::year_input(ui, "Contribution Start:", &mut a["startIn"],
            "When money will start going into this account");
        ui.end_row();
        c |= widgets::year_input(ui, "Contribution End:", &mut a["endIn"],
            "When money will stop going into this account");
        ui.end_row();
        c |= widgets::year_input(ui, "Withdrawal Start:", &mut a["startOut"],
            "When money will start coming out of this account");
        ui.end_row();
        c |= widgets::year_input(ui, "Withdrawal End:", &mut a["endOut"],
            "When money will stop coming out of this account");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Contribution Type:", a, "contributionType",
            CONTRIBUTION_OPTIONS, &format!("sav_ct_{}", uuid),
            "How money is put into the account:\n\
             Fixed Amount — fixed dollar amount\n\
             % of Income — percent of total income\n\
             Fixed + Inflation — fixed amount adjusted for inflation",
        );
        ui.end_row();
        c |= widgets::f64_field(ui, "Contribution Value:", a, "contributionValue", 500.0,
            "Amount put into this account every year [in today's dollars]");
        ui.end_row();
        c |= widgets::percent_input(ui, "Yearly Return (%):", &mut a["yearlyReturn"],
            "Percent interest earned each year");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Withdrawal Type:", a, "withdrawalType",
            WITHDRAWAL_OPTIONS, &format!("sav_wt_{}", uuid),
            "How money is taken out of the account:\n\
             Fixed Amount — fixed dollar amount\n\
             Fixed + Inflation — fixed amount adjusted for inflation\n\
             Draw Down to Zero — equal amounts so balance reaches zero at end\n\
             Fraction of Savings — proportional to total savings",
        );
        ui.end_row();
        c |= widgets::f64_field(ui, "Withdrawal Value:", a, "withdrawalValue", 500.0,
            "How much money to take out per year [in today's dollars]");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Tax Status:", a, "taxStatus",
            TAX_STATUS_OPTIONS, &format!("sav_ts_{}", uuid),
            "How taxes impact this account:\n\
             Roth — paid with after-tax income, tax-free withdrawals\n\
             Taxed Both Ways — taxed on contributions and withdrawals\n\
             Traditional — pre-tax contributions, taxed on withdrawal\n\
             Tax-Free (HSA/529) — pre-tax contributions, tax-free withdrawals",
        );
        ui.end_row();
        c |= widgets::notes_field(ui, a);
        ui.end_row();
    });
    c
}

// ─── Link combo helpers ───────────────────────────────────────────────────────

fn income_link_combo(ui: &mut egui::Ui, a: &mut Value, app: &FpApp, id: &str) -> bool {
    let current = a["incomeLink"].as_str().unwrap_or("").to_string();
    let accounts: Vec<(String, String)> = app.data["accounts"]
        .as_object()
        .map(|map| {
            map.iter()
                .filter(|(_, v)| v["type"].as_str() == Some("income"))
                .map(|(k, v)| (k.clone(), v["name"].as_str().unwrap_or("Unnamed").to_string()))
                .collect()
        })
        .unwrap_or_default();

    let label = if current.is_empty() {
        "None".to_string()
    } else {
        accounts.iter().find(|(k, _)| k == &current)
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

fn hsa_link_combo(ui: &mut egui::Ui, a: &mut Value, app: &FpApp, id: &str) -> bool {
    let current = a["hsaLink"].as_str().unwrap_or("").to_string();
    let accounts: Vec<(String, String)> = app.data["accounts"]
        .as_object()
        .map(|map| {
            map.iter()
                .filter(|(_, v)| v["type"].as_str() == Some("hsa"))
                .map(|(k, v)| (k.clone(), v["name"].as_str().unwrap_or("Unnamed").to_string()))
                .collect()
        })
        .unwrap_or_default();

    let label = if current.is_empty() {
        "None".to_string()
    } else {
        accounts.iter().find(|(k, _)| k == &current)
            .map(|(_, n)| n.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    };

    let mut changed = false;
    egui::ComboBox::new(id, "")
        .selected_text(&label)
        .show_ui(ui, |ui| {
            if ui.selectable_label(current.is_empty(), "None").clicked() {
                a["hsaLink"] = Value::Null;
                changed = true;
            }
            for (uuid, name) in &accounts {
                if ui.selectable_label(current == *uuid, name).clicked() {
                    a["hsaLink"] = json!(uuid);
                    changed = true;
                }
            }
        });
    changed
}
