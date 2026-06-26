use eframe::egui;
use serde_json::json;

use crate::app::FpApp;

pub fn show(app: &mut FpApp, ui: &mut egui::Ui) {
    ui.heading("Settings");
    ui.add_space(8.0);

    if app.data.is_null() {
        return;
    }

    // Clone settings for editing, write back if changed
    let mut settings = app.data["settings"].clone();
    let mut changed = false;

    egui::Grid::new("settings_grid")
        .num_columns(2)
        .spacing([16.0, 6.0])
        .show(ui, |ui| {
            // Simulation years
            ui.label("Year Born:");
            let mut year_born = settings["yearBorn"].as_u64().unwrap_or(1970) as u32;
            if ui.add(egui::DragValue::new(&mut year_born)).changed() {
                settings["yearBorn"] = json!(year_born);
                changed = true;
            }
            ui.end_row();

            ui.label("Year Start:");
            let mut year_start = settings["yearStart"].as_u64().unwrap_or(2024) as u32;
            if ui.add(egui::DragValue::new(&mut year_start)).changed() {
                settings["yearStart"] = json!(year_start);
                changed = true;
            }
            ui.end_row();

            ui.label("Age at Retire:");
            let mut age_retire = settings["ageRetire"].as_u64().unwrap_or(65) as u32;
            if ui.add(egui::DragValue::new(&mut age_retire)).changed() {
                settings["ageRetire"] = json!(age_retire);
                changed = true;
            }
            ui.end_row();

            ui.label("Age at Death:");
            let mut age_die = settings["ageDie"].as_u64().unwrap_or(90) as u32;
            if ui.add(egui::DragValue::new(&mut age_die)).changed() {
                settings["ageDie"] = json!(age_die);
                changed = true;
            }
            ui.end_row();

            ui.separator();
            ui.separator();
            ui.end_row();

            // Tax & inflation rates
            ui.label("Income Tax Rate (%):");
            let mut tax_income = settings["taxIncome"].as_f64().unwrap_or(22.0);
            if ui
                .add(egui::DragValue::new(&mut tax_income).speed(0.1).range(0.0..=100.0))
                .changed()
            {
                settings["taxIncome"] = json!(tax_income);
                changed = true;
            }
            ui.end_row();

            ui.label("Capital Gains Tax Rate (%):");
            let mut tax_cg = settings["taxCapitalGains"].as_f64().unwrap_or(15.0);
            if ui
                .add(egui::DragValue::new(&mut tax_cg).speed(0.1).range(0.0..=100.0))
                .changed()
            {
                settings["taxCapitalGains"] = json!(tax_cg);
                changed = true;
            }
            ui.end_row();

            ui.label("Inflation Rate (%):");
            let mut inflation = settings["inflationBase"].as_f64().unwrap_or(3.0);
            if ui
                .add(egui::DragValue::new(&mut inflation).speed(0.05).range(0.0..=20.0))
                .changed()
            {
                settings["inflationBase"] = json!(inflation);
                changed = true;
            }
            ui.end_row();

            ui.label("Retirement Cost of Living (%):");
            let mut col = settings["retirementCostOfLiving"].as_f64().unwrap_or(80.0);
            if ui
                .add(egui::DragValue::new(&mut col).speed(0.5).range(0.0..=200.0))
                .changed()
            {
                settings["retirementCostOfLiving"] = json!(col);
                changed = true;
            }
            ui.end_row();

            ui.separator();
            ui.separator();
            ui.end_row();

            // SSA breakpoints
            ui.label("SSA Breakpoint Low ($):");
            let mut bp_low = settings["ssa"]["breakpoints"]["low"]
                .as_f64()
                .unwrap_or(25000.0);
            if ui.add(egui::DragValue::new(&mut bp_low).speed(500.0)).changed() {
                settings["ssa"]["breakpoints"]["low"] = json!(bp_low);
                changed = true;
            }
            ui.end_row();

            ui.label("SSA Breakpoint High ($):");
            let mut bp_high = settings["ssa"]["breakpoints"]["high"]
                .as_f64()
                .unwrap_or(34000.0);
            if ui.add(egui::DragValue::new(&mut bp_high).speed(500.0)).changed() {
                settings["ssa"]["breakpoints"]["high"] = json!(bp_high);
                changed = true;
            }
            ui.end_row();

            ui.label("SSA Taxable % (Low):");
            let mut tip_low = settings["ssa"]["taxableIncomePercentage"]["low"]
                .as_f64()
                .unwrap_or(50.0);
            if ui
                .add(egui::DragValue::new(&mut tip_low).speed(0.5).range(0.0..=100.0))
                .changed()
            {
                settings["ssa"]["taxableIncomePercentage"]["low"] = json!(tip_low);
                changed = true;
            }
            ui.end_row();

            ui.label("SSA Taxable % (High):");
            let mut tip_high = settings["ssa"]["taxableIncomePercentage"]["high"]
                .as_f64()
                .unwrap_or(85.0);
            if ui
                .add(egui::DragValue::new(&mut tip_high).speed(0.5).range(0.0..=100.0))
                .changed()
            {
                settings["ssa"]["taxableIncomePercentage"]["high"] = json!(tip_high);
                changed = true;
            }
            ui.end_row();
        });

    if changed {
        app.data["settings"] = settings;
        app.dirty = true;
    }

    // Derived info display
    let year_born = app.data["settings"]["yearBorn"].as_u64().unwrap_or(1970) as u32;
    let age_retire = app.data["settings"]["ageRetire"].as_u64().unwrap_or(65) as u32;
    let age_die = app.data["settings"]["ageDie"].as_u64().unwrap_or(90) as u32;
    ui.add_space(12.0);
    ui.separator();
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(format!("Retire in: {}", year_born + age_retire));
        ui.separator();
        ui.label(format!("End year: {}", year_born + age_die));
    });
}
