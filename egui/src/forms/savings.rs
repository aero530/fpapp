use eframe::egui;
use serde_json::Value;

use super::common::{
    TAX_STATUS_TOOLTIP, contribution_rows, in_out_year_rows, name_row, notes_row, tax_status_row,
    withdrawal_rows, yearly_return_row,
};
use super::{TAX_STATUS_OPTIONS, form_grid};

pub(super) fn show(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= name_row(ui, a);
        c |= in_out_year_rows(ui, a, uuid);
        c |= contribution_rows(ui, a, "sav", uuid, 500.0);
        c |= yearly_return_row(ui, a, uuid);
        c |= withdrawal_rows(ui, a, "sav", uuid);
        c |= tax_status_row(ui, a, "sav", uuid, TAX_STATUS_OPTIONS, TAX_STATUS_TOOLTIP);
        c |= notes_row(ui, a);
    });
    c
}
