use egui_plot::{Line, PlotPoints};

use crate::app::FpApp;
use crate::widgets::dollar_plot;

pub fn show(app: &FpApp, ui: &mut eframe::egui::Ui) {
    ui.heading("Dashboard");
    ui.add_space(8.0);

    if let Some(totals) = &app.yearly_totals {
        let years_net = totals.net.years();
        let years_income = totals.income.years();
        let years_expense = totals.expense.years();
        let years_saving = totals.saving.years();
        let years_hsa = totals.hsa.years();
        let years_col = totals.col.years();
        let years_hc = totals.healthcare_expense_total.years();
        let years_tax = totals.tax_burden.years();

        let vals_net = totals.net.values();
        let vals_income = totals.income.values();
        let vals_expense = totals.expense.values();
        let vals_saving = totals.saving.values();
        let vals_hsa = totals.hsa.values();
        let vals_col = totals.col.values();
        let vals_hc = totals.healthcare_expense_total.values();
        let vals_tax = totals.tax_burden.values();

        let make_points = |years: &[u32], vals: &[f64]| -> PlotPoints {
            years
                .iter()
                .zip(vals.iter())
                .map(|(&x, &y)| [x as f64, y])
                .collect()
        };

        ui.label("Net / Income / Expense");
        dollar_plot("chart_nie", 440.0).show(ui, |plot_ui| {
            plot_ui.line(Line::new("Net", make_points(&years_net, &vals_net)));
            plot_ui.line(Line::new(
                "Income",
                make_points(&years_income, &vals_income),
            ));
            plot_ui.line(Line::new(
                "Expense",
                make_points(&years_expense, &vals_expense),
            ));
        });

        ui.add_space(12.0);
        ui.label("Savings & HSA");
        dollar_plot("chart_saving", 440.0).show(ui, |plot_ui| {
            plot_ui.line(Line::new(
                "Savings",
                make_points(&years_saving, &vals_saving),
            ));
            plot_ui.line(Line::new("HSA", make_points(&years_hsa, &vals_hsa)));
        });

        ui.add_space(12.0);
        ui.label("Cost of Living");
        dollar_plot("chart_col", 440.0).show(ui, |plot_ui| {
            plot_ui.line(Line::new(
                "Cost of Living",
                make_points(&years_col, &vals_col),
            ));
        });

        ui.add_space(12.0);
        ui.label("Healthcare & Tax");
        dollar_plot("chart_hc_tax", 440.0).show(ui, |plot_ui| {
            plot_ui.line(Line::new("Healthcare", make_points(&years_hc, &vals_hc)));
            plot_ui.line(Line::new("Tax Burden", make_points(&years_tax, &vals_tax)));
        });
    } else if app.error.is_none() {
        ui.centered_and_justified(|ui| {
            ui.label("Open a data file to see the dashboard.");
        });
    } else {
        ui.colored_label(
            eframe::egui::Color32::from_rgb(220, 60, 60),
            "Analysis error — check the error message in the sidebar.",
        );
    }
}
