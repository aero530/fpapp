use eframe::egui;
use serde_json::{Value, json};

use super::{EXPENSE_OPTIONS, form_grid};
use crate::widgets;

pub(super) fn show(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= widgets::string_field(ui, "Name:", a, "name",
            "Human friendly name for the account");
        ui.end_row();
        c |= widgets::year_input(ui, "Start Year:", uuid, &mut a["startOut"],
            "When this expense will start");
        ui.end_row();
        c |= widgets::year_input(ui, "End Year:", uuid, &mut a["endOut"],
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
            .on_hover_text("Is this a healthcare cost? Healthcare costs are paid out of HSA accounts first; any remainder comes out of net.")
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

        c |= widgets::notes_field(ui, a);
        ui.end_row();
    });
    c
}
