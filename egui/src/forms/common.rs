//! Shared form rows for the savings-style account types
//! (retirement, hsa, college, savings), which are ~80% identical.

use eframe::egui;
use serde_json::Value;

use crate::widgets;
use super::{CONTRIBUTION_OPTIONS, WITHDRAWAL_OPTIONS};

pub(super) fn name_row(ui: &mut egui::Ui, a: &mut Value) -> bool {
    let c = widgets::string_field(ui, "Name:", a, "name",
        "Human friendly name for the account");
    ui.end_row();
    c
}

/// Contribution start/end + withdrawal start/end year rows
pub(super) fn in_out_year_rows(ui: &mut egui::Ui, a: &mut Value) -> bool {
    let mut c = false;
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
    c
}

/// Contribution type combo + contribution value rows
pub(super) fn contribution_rows(
    ui: &mut egui::Ui,
    a: &mut Value,
    id_prefix: &str,
    uuid: &str,
    value_speed: f64,
) -> bool {
    let mut c = false;
    c |= widgets::combo_field(
        ui, "Contribution Type:", a, "contributionType",
        CONTRIBUTION_OPTIONS, &format!("{}_ct_{}", id_prefix, uuid),
        "How money is put into the account:\n\
         Fixed Amount — fixed dollar amount\n\
         % of Income — percent of total income\n\
         Fixed + Inflation — fixed amount adjusted for inflation",
    );
    ui.end_row();
    c |= widgets::f64_field(ui, "Contribution Value:", a, "contributionValue", value_speed,
        "Amount put into this account every year [in today's dollars]");
    ui.end_row();
    c
}

pub(super) fn yearly_return_row(ui: &mut egui::Ui, a: &mut Value) -> bool {
    let c = widgets::percent_input(ui, "Yearly Return (%):", &mut a["yearlyReturn"],
        "Percent interest earned each year");
    ui.end_row();
    c
}

/// Withdrawal type combo + withdrawal value rows
pub(super) fn withdrawal_rows(ui: &mut egui::Ui, a: &mut Value, id_prefix: &str, uuid: &str) -> bool {
    let mut c = false;
    c |= widgets::combo_field(
        ui, "Withdrawal Type:", a, "withdrawalType",
        WITHDRAWAL_OPTIONS, &format!("{}_wt_{}", id_prefix, uuid),
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
    c
}

/// Tax status combo row with the given option set
pub(super) fn tax_status_row(
    ui: &mut egui::Ui,
    a: &mut Value,
    id_prefix: &str,
    uuid: &str,
    options: &[(&str, &str)],
    tooltip: &str,
) -> bool {
    let c = widgets::combo_field(
        ui, "Tax Status:", a, "taxStatus",
        options, &format!("{}_ts_{}", id_prefix, uuid),
        tooltip,
    );
    ui.end_row();
    c
}

pub(super) const TAX_STATUS_TOOLTIP: &str =
    "How taxes impact this account:\n\
     Roth — paid with after-tax income, tax-free withdrawals\n\
     Taxed Both Ways — taxed on contributions, earnings taxed as capital gains\n\
     Traditional — pre-tax contributions, taxed on withdrawal\n\
     Tax-Free (HSA/529) — pre-tax contributions, tax-free withdrawals";

pub(super) fn notes_row(ui: &mut egui::Ui, a: &mut Value) -> bool {
    let c = widgets::notes_field(ui, a);
    ui.end_row();
    c
}
