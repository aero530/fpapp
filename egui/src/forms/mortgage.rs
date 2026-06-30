use eframe::egui;
use serde_json::Value;

use crate::widgets;
use super::{form_grid, PAYMENT_OPTIONS};

pub(super) fn show(ui: &mut egui::Ui, a: &mut Value, uuid: &str) -> bool {
    let mut c = false;
    form_grid(ui).show(ui, |ui| {
        c |= widgets::string_field(ui, "Name:", a, "name",
            "Human friendly name for the account");
        ui.end_row();
        c |= widgets::year_input(ui, "Start Year:", &mut a["startOut"],
            "When mortgage payments will start");
        ui.end_row();
        c |= widgets::year_input(ui, "End Year:", &mut a["endOut"],
            "When mortgage payments will end (loan payoff year)");
        ui.end_row();
        c |= widgets::combo_field(
            ui, "Payment Type:", a, "paymentType",
            PAYMENT_OPTIONS, &format!("mort_pt_{}", uuid),
            "How the payment value is interpreted:\n\
             Fixed Amount — fixed dollar amount each year\n\
             Fixed + Inflation — fixed amount adjusted for inflation",
        );
        ui.end_row();
        c |= widgets::f64_field(ui, "Annual Payment ($):", a, "paymentValue", 500.0,
            "Total amount paid per year including principal, interest, escrow, and mortgage insurance [in today's dollars]");
        ui.end_row();
        c |= widgets::f64_field(ui, "Interest Rate (%):", a, "rate", 0.05,
            "Interest rate on borrowed money (APR, compounded based on compound periods setting)");
        ui.end_row();
        c |= widgets::f64_field(ui, "Compound Periods/Year:", a, "compoundTime", 1.0,
            "Number of times per year interest is compounded (1=yearly, 12=monthly)");
        ui.end_row();
        c |= widgets::f64_field(ui, "Mortgage Insurance ($):", a, "mortgageInsurance", 10.0,
            "Yearly mortgage insurance payment [in today's dollars]");
        ui.end_row();
        c |= widgets::f64_field(ui, "LTV Limit (%):", a, "ltvLimit", 1.0,
            "Loan-to-value ratio at which mortgage insurance is no longer required");
        ui.end_row();
        c |= widgets::f64_field(ui, "Escrow ($/yr):", a, "escrowValue", 100.0,
            "Yearly amount set aside for property taxes [in today's dollars]");
        ui.end_row();
        c |= widgets::f64_field(ui, "Home Value ($):", a, "homeValue", 5000.0,
            "Current value of the home, used to compute loan-to-value ratio [in today's dollars]");
        ui.end_row();
        c |= widgets::notes_field(ui, a);
        ui.end_row();
    });
    c
}
