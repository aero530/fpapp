use std::collections::HashMap;
use std::path::PathBuf;

use eframe::egui;
use serde_json::Value;
use accounts::{PlotDataSet, YearlyTotals};

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Dashboard,
    Settings,
    Account(String),
}

pub struct FpApp {
    pub data: Value,
    pub file_path: Option<PathBuf>,
    pub selected: Page,
    pub dirty: bool,
    pub plot_data: HashMap<String, Vec<PlotDataSet>>,
    pub yearly_totals: Option<YearlyTotals>,
    pub error: Option<String>,
}

impl FpApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            data: Value::Null,
            file_path: None,
            selected: Page::Dashboard,
            dirty: false,
            plot_data: HashMap::new(),
            yearly_totals: None,
            error: None,
        }
    }

    pub fn run_analysis(&mut self) {
        match crate::analyze::run_analysis(&self.data) {
            Ok((plot_data, totals)) => {
                self.plot_data = plot_data;
                self.yearly_totals = Some(totals);
                self.error = None;
            }
            Err(e) => {
                self.error = Some(e);
                self.plot_data.clear();
                self.yearly_totals = None;
            }
        }
    }
}

impl eframe::App for FpApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.dirty {
            if !self.data.is_null() {
                self.run_analysis();
            }
            self.dirty = false;
        }

        crate::nav::show_nav(self, ctx);

        let page = self.selected.clone();
        egui::CentralPanel::default()
            .frame(
                egui::Frame::central_panel(&ctx.style())
                    .inner_margin(egui::Margin { left: 8.0, right: 16.0, top: 8.0, bottom: 8.0 }),
            )
            .show(ctx, |ui| {
            if self.data.is_null() {
                ui.vertical_centered(|ui| {
                    ui.add_space(120.0);
                    ui.heading("Financial Planner");
                    ui.add_space(16.0);
                    ui.label("Open a data file to get started.");
                    ui.add_space(12.0);
                    if ui.button("Open File...").clicked() {
                        crate::nav::open_file(self);
                    }
                });
                return;
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                match &page {
                    Page::Dashboard => crate::dashboard::show(self, ui),
                    Page::Settings => crate::settings_view::show(self, ui),
                    Page::Account(uuid) => {
                        let uuid = uuid.clone();
                        crate::forms::show_account(self, ui, &uuid);
                    }
                }
            });
        });
    }
}
