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
    /// UUID of account awaiting delete confirmation; None when no dialog is open.
    pub confirm_delete: Option<String>,
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
            confirm_delete: None,
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
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.dirty {
            if !self.data.is_null() {
                self.run_analysis();
            }
            self.dirty = false;
        }

        // If the user navigated away from the account pending deletion, cancel the dialog
        if let Some(ref uuid) = self.confirm_delete.clone() {
            if self.selected != Page::Account(uuid.clone()) {
                self.confirm_delete = None;
            }
        }

        // Delete confirmation modal — shown before panels so it overlays everything
        if let Some(pending_uuid) = self.confirm_delete.clone() {
            let pending_name = self.data["accounts"][pending_uuid.as_str()]["name"]
                .as_str()
                .unwrap_or("this account")
                .to_string();
            let mut confirmed = false;
            let mut cancelled = false;
            egui::Window::new("Delete Account")
                .collapsible(false)
                .resizable(false)
                .movable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ui.ctx(), |ui| {
                    ui.set_min_width(280.0);
                    ui.label(format!("Delete \"{}\"?", pending_name));
                    ui.label("This cannot be undone.");
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui
                            .button(
                                egui::RichText::new("Delete")
                                    .color(egui::Color32::from_rgb(200, 60, 60)),
                            )
                            .clicked()
                        {
                            confirmed = true;
                        }
                        if ui.button("Cancel").clicked() {
                            cancelled = true;
                        }
                    });
                });
            if cancelled {
                self.confirm_delete = None;
            } else if confirmed {
                if let Some(accounts) = self.data["accounts"].as_object_mut() {
                    accounts.remove(&pending_uuid);
                    for (_, acct) in accounts.iter_mut() {
                        if acct["incomeLink"].as_str() == Some(pending_uuid.as_str()) {
                            acct["incomeLink"] = Value::Null;
                        }
                        if acct["hsaLink"].as_str() == Some(pending_uuid.as_str()) {
                            acct["hsaLink"] = Value::Null;
                        }
                    }
                    self.confirm_delete = None;
                    self.selected = Page::Dashboard;
                    self.dirty = true;
                } else {
                    self.confirm_delete = None;
                    self.error = Some(String::from("Delete failed: account data is malformed."));
                }
            }
        }

        crate::nav::show_nav(self, ui);

        let page = self.selected.clone();
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(ui.visuals().panel_fill)
                    .inner_margin(egui::Margin { left: 8, right: 0, top: 8, bottom: 8 }),
            )
            .show(ui, |ui| {
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
                egui::Frame::new()
                    .inner_margin(egui::Margin { left: 0, right: 16, top: 0, bottom: 0 })
                    .show(ui, |ui| {
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
        });
    }
}
