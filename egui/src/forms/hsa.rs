use eframe::egui;
use serde_json::Value;

use crate::widgets;
use super::common::{
    contribution_rows, in_out_year_rows, name_row, notes_row, tax_status_row, yearly_return_row,
    TAX_STATUS_TOOLTIP,
};
use super::{form_grid, TAX_STATUS_OPTIONS};

pub(super) fn show(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= name_row(ui, a);
        c |= in_out_year_rows(ui, a);
        c |= contribution_rows(ui, a, "hsa", uuid, 100.0);
        c |= widgets::f64_field(ui, "Employer Contribution ($):", a, "employerContribution", 100.0,
            "Employer contributions to this account as a yearly dollar amount [in today's dollars]");
        ui.end_row();
        c |= yearly_return_row(ui, a);
        c |= tax_status_row(ui, a, "hsa", uuid, TAX_STATUS_OPTIONS, TAX_STATUS_TOOLTIP);
        c |= notes_row(ui, a);
    });
    c
}
