use eframe::egui;
use serde_json::{json, Value};

use crate::app::FpApp;
use crate::widgets;
use super::common::{
    contribution_rows, in_out_year_rows, name_row, notes_row, tax_status_row, withdrawal_rows,
    yearly_return_row, TAX_STATUS_TOOLTIP,
};
use super::{form_grid, income_link_combo, TAX_STATUS_OPTIONS};

pub(super) fn show(ui: &mut egui::Ui, a: &mut Value, uuid: &str, app: &FpApp) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= name_row(ui, a);
        c |= in_out_year_rows(ui, a);
        c |= contribution_rows(ui, a, "ret", uuid, 500.0);
        c |= yearly_return_row(ui, a);
        c |= withdrawal_rows(ui, a, "ret", uuid);
        c |= tax_status_row(ui, a, "ret", uuid, TAX_STATUS_OPTIONS, TAX_STATUS_TOOLTIP);

        ui.label("Income Link:")
            .on_hover_text("Link to an income account to base percentage contributions on that account's value");
        c |= income_link_combo(ui, a, app, &format!("ret_il_{}", uuid));
        ui.end_row();

        c |= notes_row(ui, a);
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
