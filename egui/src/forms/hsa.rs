use eframe::egui;
use serde_json::Value;

use super::common::{
    contribution_rows, in_out_year_rows, name_row, notes_row, tax_status_row, yearly_return_row,
};
use super::{HSA_TAX_OPTIONS, form_grid};
use crate::widgets;

pub(super) fn show(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= name_row(ui, a);
        c |= in_out_year_rows(ui, a, uuid);
        c |= contribution_rows(ui, a, "hsa", uuid, 100.0);
        c |= widgets::f64_field(
            ui,
            "Employer Contribution ($):",
            a,
            "employerContribution",
            100.0,
            "Employer contributions to this account as a yearly dollar amount [in today's dollars]",
        );
        ui.end_row();
        c |= yearly_return_row(ui, a, uuid);
        c |= tax_status_row(
            ui,
            a,
            "hsa",
            uuid,
            HSA_TAX_OPTIONS,
            "How taxes impact this account:\n\
             Pre-tax contributions (a deduction), tax-free withdrawals for healthcare",
        );
        c |= notes_row(ui, a);
    });
    c
}
