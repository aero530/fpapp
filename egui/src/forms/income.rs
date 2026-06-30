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
