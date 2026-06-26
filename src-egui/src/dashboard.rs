use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

use crate::app::FpApp;

pub fn show(app: &FpApp, ui: &mut egui::Ui) {
    ui.heading("Dashboard");
    ui.add_space(8.0);

    if let Some(totals) = &app.yearly_totals {
        // Build (years, values) pairs from public Table<u32> fields
        let years_net = totals.net.years();
        let years_income = totals.income.years();
        let years_expense = totals.expense.years();
        let years_saving = totals.saving.years();
        let years_col = totals.col.years();
        let years_hc = totals.healthcare_expense.years();
        let years_tax = totals.tax_burden.years();

        let vals_net = totals.net.values();
        let vals_income = totals.income.values();
        let vals_expense = totals.expense.values();
        let vals_saving = totals.saving.values();
        let vals_col = totals.col.values();
        let vals_hc = totals.healthcare_expense.values();
        let vals_tax = totals.tax_burden.values();

        let make_points = |years: &[u32], vals: &[f64]| -> PlotPoints {
            years
                .iter()
                .zip(vals.iter())
                .map(|(&x, &y)| [x as f64, y])
                .collect()
        };

        egui::Grid::new("dashboard_grid")
            .num_columns(2)
            .spacing([16.0, 12.0])
            .show(ui, |ui| {
                // Chart 1: Net / Income / Expense
                ui.vertical(|ui| {
                    ui.label("Net / Income / Expense");
                    Plot::new("chart_nie")
                        .height(220.0)
                        .legend(Legend::default())
                        .show(ui, |plot_ui| {
                            plot_ui.line(
                                Line::new(make_points(&years_net, &vals_net)).name("Net"),
                            );
                            plot_ui.line(
                                Line::new(make_points(&years_income, &vals_income))
                                    .name("Income"),
                            );
                            plot_ui.line(
                                Line::new(make_points(&years_expense, &vals_expense))
                                    .name("Expense"),
                            );
                        });
                });

                // Chart 2: Savings
                ui.vertical(|ui| {
                    ui.label("Savings");
                    Plot::new("chart_saving")
                        .height(220.0)
                        .legend(Legend::default())
                        .show(ui, |plot_ui| {
                            plot_ui.line(
                                Line::new(make_points(&years_saving, &vals_saving))
                                    .name("Savings"),
                            );
                        });
                });

                ui.end_row();

                // Chart 3: Cost of Living
                ui.vertical(|ui| {
                    ui.label("Cost of Living");
                    Plot::new("chart_col")
                        .height(220.0)
                        .legend(Legend::default())
                        .show(ui, |plot_ui| {
                            plot_ui.line(
                                Line::new(make_points(&years_col, &vals_col))
                                    .name("Cost of Living"),
                            );
                        });
                });

                // Chart 4: Healthcare & Tax
                ui.vertical(|ui| {
                    ui.label("Healthcare & Tax");
                    Plot::new("chart_hc_tax")
                        .height(220.0)
                        .legend(Legend::default())
                        .show(ui, |plot_ui| {
                            plot_ui.line(
                                Line::new(make_points(&years_hc, &vals_hc))
                                    .name("Healthcare"),
                            );
                            plot_ui.line(
                                Line::new(make_points(&years_tax, &vals_tax))
                                    .name("Tax Burden"),
                            );
                        });
                });

                ui.end_row();
            });
    } else if app.error.is_none() {
        ui.centered_and_justified(|ui| {
            ui.label("Open a data file to see the dashboard.");
        });
    } else {
        ui.colored_label(
            egui::Color32::from_rgb(220, 60, 60),
            "Analysis error — check the error message in the sidebar.",
        );
    }
}
