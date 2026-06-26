use eframe::egui;
use serde_json::json;

use crate::app::FpApp;
use crate::widgets;

pub fn show(app: &mut FpApp, ui: &mut egui::Ui) {
    ui.heading("Settings");
    ui.add_space(8.0);

    if app.data.is_null() {
        return;
    }

    let mut settings = app.data["settings"].clone();
    let mut changed = false;

    egui::Grid::new("settings_grid")
        .num_columns(2)
        .spacing([12.0, 5.0])
        .min_col_width(200.0)
        .show(ui, |ui| {
            changed |= widgets::u32_field(ui, "Year Born:", &mut settings, "yearBorn");
            ui.end_row();
            changed |= widgets::u32_field(ui, "Year Start:", &mut settings, "yearStart");
            ui.end_row();
            changed |= widgets::u32_field(ui, "Age at Retire:", &mut settings, "ageRetire");
            ui.end_row();
            changed |= widgets::u32_field(ui, "Age at Death:", &mut settings, "ageDie");
            ui.end_row();

            ui.separator();
            ui.separator();
            ui.end_row();

            // f64 rate fields — use DragValue directly since widgets::f64_field works fine
            ui.label("Income Tax Rate (%):");
            let mut v = settings["taxIncome"].as_f64().unwrap_or(22.0);
            if ui.add(egui::DragValue::new(&mut v).speed(0.1).range(0.0..=100.0)).changed() {
                settings["taxIncome"] = json!(v);
                changed = true;
            }
            ui.end_row();

            ui.label("Capital Gains Tax Rate (%):");
            let mut v = settings["taxCapitalGains"].as_f64().unwrap_or(15.0);
            if ui.add(egui::DragValue::new(&mut v).speed(0.1).range(0.0..=100.0)).changed() {
                settings["taxCapitalGains"] = json!(v);
                changed = true;
            }
            ui.end_row();

            ui.label("Inflation Rate (%):");
            let mut v = settings["inflationBase"].as_f64().unwrap_or(3.0);
            if ui.add(egui::DragValue::new(&mut v).speed(0.05).range(0.0..=20.0)).changed() {
                settings["inflationBase"] = json!(v);
                changed = true;
            }
            ui.end_row();

            ui.label("Retirement Cost of Living (%):");
            let mut v = settings["retirementCostOfLiving"].as_f64().unwrap_or(80.0);
            if ui.add(egui::DragValue::new(&mut v).speed(0.5).range(0.0..=200.0)).changed() {
                settings["retirementCostOfLiving"] = json!(v);
                changed = true;
            }
            ui.end_row();

            ui.separator();
            ui.separator();
            ui.end_row();

            ui.label("SSA Breakpoint Low ($):");
            let mut v = settings["ssa"]["breakpoints"]["low"].as_f64().unwrap_or(25000.0);
            if ui.add(egui::DragValue::new(&mut v).speed(500.0)).changed() {
                settings["ssa"]["breakpoints"]["low"] = json!(v);
                changed = true;
            }
            ui.end_row();

            ui.label("SSA Breakpoint High ($):");
            let mut v = settings["ssa"]["breakpoints"]["high"].as_f64().unwrap_or(34000.0);
            if ui.add(egui::DragValue::new(&mut v).speed(500.0)).changed() {
                settings["ssa"]["breakpoints"]["high"] = json!(v);
                changed = true;
            }
            ui.end_row();

            ui.label("SSA Taxable % (Low):");
            let mut v = settings["ssa"]["taxableIncomePercentage"]["low"].as_f64().unwrap_or(50.0);
            if ui.add(egui::DragValue::new(&mut v).speed(0.5).range(0.0..=100.0)).changed() {
                settings["ssa"]["taxableIncomePercentage"]["low"] = json!(v);
                changed = true;
            }
            ui.end_row();

            ui.label("SSA Taxable % (High):");
            let mut v = settings["ssa"]["taxableIncomePercentage"]["high"].as_f64().unwrap_or(85.0);
            if ui.add(egui::DragValue::new(&mut v).speed(0.5).range(0.0..=100.0)).changed() {
                settings["ssa"]["taxableIncomePercentage"]["high"] = json!(v);
                changed = true;
            }
            ui.end_row();
        });

    if changed {
        app.data["settings"] = settings;
        app.dirty = true;
    }

    // Derived info
    let year_born = app.data["settings"]["yearBorn"].as_u64().unwrap_or(1970) as u32;
    let age_retire = app.data["settings"]["ageRetire"].as_u64().unwrap_or(65) as u32;
    let age_die = app.data["settings"]["ageDie"].as_u64().unwrap_or(90) as u32;
    ui.add_space(12.0);
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(format!("Retire in: {}", year_born + age_retire));
        ui.separator();
        ui.label(format!("End year: {}", year_born + age_die));
    });
}
