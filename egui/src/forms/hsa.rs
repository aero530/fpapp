use eframe::egui;
use serde_json::Value;

use crate::widgets;
use super::{form_grid, CONTRIBUTION_OPTIONS, TAX_STATUS_OPTIONS};

pub(super) fn show(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
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
