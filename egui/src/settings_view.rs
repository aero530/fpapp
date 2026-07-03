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

    // Edit the settings object in place (no per-frame clone); `changed` is
    // tracked by the widgets themselves.
    let settings = &mut app.data["settings"];
    let mut changed = false;

    egui::Grid::new("settings_grid")
        .num_columns(2)
        .spacing([12.0, 5.0])
        .min_col_width(200.0)
        .show(ui, |ui| {
            changed |= widgets::u32_field(ui, "Year Born:", settings, "yearBorn", 1850..=2200,
                "The year you were born — used to calculate retirement and death years");
            ui.end_row();
            changed |= widgets::u32_field(ui, "Year Start:", settings, "yearStart", 1850..=2200,
                "The first year of the simulation (usually the current year)");
            ui.end_row();
            changed |= widgets::u32_field(ui, "Age at Retire:", settings, "ageRetire", 0..=130,
                "The age at which you plan to retire — sets the yearRetire variable");
            ui.end_row();
            changed |= widgets::u32_field(ui, "Age at Death:", settings, "ageDie", 0..=130,
                "The age used as the end of the simulation — sets the yearDie variable");
            ui.end_row();

            ui.separator();
            ui.separator();
            ui.end_row();

            ui.label("Income Tax Rate (%):")
                .on_hover_text("Marginal income tax rate applied to taxable withdrawals and ordinary income");
            let mut v = settings["taxIncome"].as_f64().unwrap_or(22.0);
            if ui.add(egui::DragValue::new(&mut v).speed(0.1).range(0.0..=100.0)).changed() {
                settings["taxIncome"] = json!(v);
                changed = true;
            }
            ui.end_row();

            ui.label("Capital Gains Tax Rate (%):")
                .on_hover_text("Tax rate applied to capital gains (investment earnings in taxable accounts)");
            let mut v = settings["taxCapitalGains"].as_f64().unwrap_or(15.0);
            if ui.add(egui::DragValue::new(&mut v).speed(0.1).range(0.0..=100.0)).changed() {
                settings["taxCapitalGains"] = json!(v);
                changed = true;
            }
            ui.end_row();

            ui.label("Inflation Rate (%):")
                .on_hover_text("Expected yearly inflation rate — used when account inputs reference inflationBase");
            let mut v = settings["inflationBase"].as_f64().unwrap_or(3.0);
            if ui.add(egui::DragValue::new(&mut v).speed(0.05).range(0.0..=20.0)).changed() {
                settings["inflationBase"] = json!(v);
                changed = true;
            }
            ui.end_row();

            ui.label("Retirement Cost of Living (%):")
                .on_hover_text("Your expected spending in retirement as a percent of pre-retirement spending (e.g. 80 means you'll spend 80% of what you spend now)");
            let mut v = settings["retirementCostOfLiving"].as_f64().unwrap_or(80.0);
            if ui.add(egui::DragValue::new(&mut v).speed(0.5).range(0.0..=200.0)).changed() {
                settings["retirementCostOfLiving"] = json!(v);
                changed = true;
            }
            ui.end_row();

            ui.separator();
            ui.separator();
            ui.end_row();

            // The breakpoint and percentage pairs are each constrained so low
            // cannot be dragged past high (the engine also validates this)
            let bp_low = settings["ssa"]["breakpoints"]["low"].as_f64().unwrap_or(25000.0);
            let bp_high = settings["ssa"]["breakpoints"]["high"].as_f64().unwrap_or(34000.0);

            ui.label("SSA Breakpoint Low ($):")
                .on_hover_text("Combined income threshold below which 0% of Social Security benefits are taxable");
            let mut v = bp_low;
            if ui.add(egui::DragValue::new(&mut v).speed(500.0).range(0.0..=bp_high)).changed() {
                settings["ssa"]["breakpoints"]["low"] = json!(v);
                changed = true;
            }
            ui.end_row();

            ui.label("SSA Breakpoint High ($):")
                .on_hover_text("Combined income threshold above which the maximum percentage of Social Security benefits are taxable");
            let mut v = bp_high;
            if ui.add(egui::DragValue::new(&mut v).speed(500.0).range(bp_low..=f64::MAX)).changed() {
                settings["ssa"]["breakpoints"]["high"] = json!(v);
                changed = true;
            }
            ui.end_row();

            let pct_low = settings["ssa"]["taxableIncomePercentage"]["low"].as_f64().unwrap_or(50.0);
            let pct_high = settings["ssa"]["taxableIncomePercentage"]["high"].as_f64().unwrap_or(85.0);

            ui.label("SSA Taxable % (Low):")
                .on_hover_text("Percent of Social Security benefits that are taxable when income is between the low and high breakpoints");
            let mut v = pct_low;
            if ui.add(egui::DragValue::new(&mut v).speed(0.5).range(0.0..=pct_high)).changed() {
                settings["ssa"]["taxableIncomePercentage"]["low"] = json!(v);
                changed = true;
            }
            ui.end_row();

            ui.label("SSA Taxable % (High):")
                .on_hover_text("Percent of Social Security benefits that are taxable when income is above the high breakpoint");
            let mut v = pct_high;
            if ui.add(egui::DragValue::new(&mut v).speed(0.5).range(pct_low..=100.0)).changed() {
                settings["ssa"]["taxableIncomePercentage"]["high"] = json!(v);
                changed = true;
            }
            ui.end_row();
        });

    if changed {
        app.dirty = true;
    }

    // Derived info (saturating: existing files may hold out-of-range values)
    let year_born = app.data["settings"]["yearBorn"].as_u64().unwrap_or(1970) as u32;
    let age_retire = app.data["settings"]["ageRetire"].as_u64().unwrap_or(65) as u32;
    let age_die = app.data["settings"]["ageDie"].as_u64().unwrap_or(90) as u32;
    ui.add_space(12.0);
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(format!(
            "Retire in: {}",
            year_born.saturating_add(age_retire)
        ));
        ui.separator();
        ui.label(format!("End year: {}", year_born.saturating_add(age_die)));
    });
}
