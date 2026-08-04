use eframe::egui;
use serde_json::Value;

use super::{PAYMENT_OPTIONS, form_grid};
use crate::widgets;

pub(super) fn show(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= widgets::string_field(
            ui,
            "Name:",
            a,
            "name",
            "Human friendly name for the account",
        );
        ui.end_row();
        c |= widgets::year_input(
            ui,
            "Start Year:",
            uuid,
            &mut a["startOut"],
            "When loan payments will start",
        );
        ui.end_row();
        c |= widgets::year_input(
            ui,
            "End Year:",
            uuid,
            &mut a["endOut"],
            "When loan payments will end",
        );
        ui.end_row();
        c |= widgets::combo_field(
            ui,
            "Payment Type:",
            a,
            "paymentType",
            PAYMENT_OPTIONS,
            &format!("loan_pt_{}", uuid),
            "How the payment value is interpreted:\n\
             Fixed Amount — fixed dollar amount each year\n\
             Fixed + Inflation — fixed amount adjusted for inflation",
        );
        ui.end_row();
        c |= widgets::f64_field(
            ui,
            "Annual Payment ($):",
            a,
            "paymentValue",
            500.0,
            "Total amount paid per year toward this loan [in today's dollars]",
        );
        ui.end_row();
        c |= widgets::percent_input(
            ui,
            "Interest Rate (%):",
            uuid,
            &mut a["rate"],
            "Interest rate on borrowed money (APR)",
        );
        ui.end_row();
        c |= widgets::notes_field(ui, a);
        ui.end_row();
    });
    c
}
