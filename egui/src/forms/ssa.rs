use eframe::egui;
use serde_json::Value;

use crate::widgets;
use super::form_grid;

pub(super) fn show(ui: &mut egui::Ui, a: &mut Value, _uuid: &str) -> bool {
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
