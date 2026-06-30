use eframe::egui;
use serde_json::{json, Value};

use crate::app::FpApp;
use crate::widgets;
use super::{form_grid, income_link_combo, CONTRIBUTION_OPTIONS, WITHDRAWAL_OPTIONS, TAX_STATUS_OPTIONS};

pub(super) fn show(ui: &mut egui::Ui, a: &mut Value, uuid: &str, app: &FpApp) -> bool {
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
