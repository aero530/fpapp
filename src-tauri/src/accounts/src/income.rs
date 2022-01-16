//! Source of income

use serde::{Deserialize, Serialize};
use std::error::Error;
use image::{ImageBuffer, Rgba};
use serde_json::json;

use super::*;

/// Account to represent sources of income
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Income<T: std::cmp::Ord> {
    /// String describing this account
    name: String,
    /// Table of account income
    table: Table<T>,
    /// Base pay (with bonuses) [in today's dollars]
    base: f64,
    /// Calendar year when money starts being earned by this account
    start_in: YearInput,
    /// Calendar year when money stops being earned by this account
    end_in: YearInput,
    /// Yearly increase in income as a percent
    raise: PercentInput,
    /// General information to store with this account
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    /// Tables used to store simulation results
    #[serde(skip)]
    analysis: SingleTable,
    /// Calculated date values as a year based on input values
    #[serde(skip)]
    dates: Dates,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IncomeInput {
    /// String describing this account
    name: String,
    /// Table of account income
    //table: [TableRow],
    /// Base pay (with bonuses) [in today's dollars]
    base: f64,
    /// Calendar year when money starts being earned by this account
    start_in: String,
    /// Calendar year when money stops being earned by this account
    end_in: String,
    /// Yearly increase in income as a percent
    raise: String,
    /// General information to store with this account
    notes: String,
}

impl From<Income<String>> for Income<u32> {
    fn from(other: Income<String>) -> Self {
        Self {
            name: other.name,
            base: other.base,
            table: other.table.into(),
            start_in: other.start_in,
            end_in: other.end_in,
            raise: other.raise,
            notes: other.notes,
            analysis: other.analysis,
            dates: other.dates,
        }
    }
}

impl Account for Income<u32> {
    fn type_id(&self) -> AccountType {
        AccountType::Income
    }
    fn link_id(&self) -> Option<String> {
        None
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn init(
        &mut self,
        linked_dates: Option<Dates>,
        settings: &Settings,
    ) -> Result<Vec<(u32, YearlyImpact)>, Box<dyn Error>> {
        if linked_dates.is_some() {
            return Err(String::from("Linked account dates provided but not used").into());
        }
        self.analysis = SingleTable::new(&self.table);
        self.dates = Dates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        };
        Ok(Vec::new())
    }
    fn get_value(&self, year: u32) -> Option<f64> {
        self.analysis.value.get(year)
    }
    fn get_range_in(&self, settings: &Settings, linked_dates: Option<Dates>) -> Option<YearRange> {
        Some(YearRange {
            start: self
                .start_in
                .value(settings, linked_dates, YearEvalType::StartIn),
            end: self
                .end_in
                .value(settings, linked_dates, YearEvalType::EndIn),
        })
    }
    fn get_range_out(
        &self,
        _settings: &Settings,
        _linked_dates: Option<Dates>,
    ) -> Option<YearRange> {
        None
    }
    fn get_inputs(&self) -> String {
        json!(IncomeInput { 
            name: self.name.clone(), 
            base: self.base, 
            start_in: json!(self.start_in).to_string(),
            end_in: json!(self.end_in).to_string(),
            raise: json!(self.raise).to_string(),
            notes: json!(self.notes).to_string(),
        }).to_string()
    }
    fn plot_to_file(&self, filepath: String, width: u32, height: u32) {
        scatter_plot_file(
            filepath,
            vec![("Amount".into(), &self.analysis.value)],
            self.name(),
            width,
            height,
        );
    }
    fn plot_to_buf(&self, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        scatter_plot_buf(
            vec![("Amount".into(), &self.analysis.value)],
            self.name(),
            width,
            height,
        )
    }
    fn get_plot_data(&self) -> Vec<PlotDataPoint> {
        self.analysis.get_plot_data()
    }
    fn simulate(
        &mut self,
        year: u32,
        _totals: &YearlyTotals,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let start_in = self.dates.year_in.unwrap().start;
        let mut result = WorkingValues::default();
        self.analysis.add_year(year, false)?;

        // Calculate earnings
        if self.dates.year_in.unwrap().contains(year) {
            let raise = self.raise.value(settings) / 100.0 + 1.0;
            result.earning = self.base * f64::powf(raise, (year - start_in) as f64);
        }

        // Add earnings to value tables
        self.analysis.value.update(year, result.earning);

        Ok(YearlyImpact {
            expense: 0_f64,
            healthcare_expense: 0_f64,
            col: 0_f64,
            saving: 0_f64,
            income_taxable: result.earning,
            income: result.earning,
            hsa: 0_f64,
        })
    }
    fn write(&self, filepath: String) {
        self.analysis.write(filepath);
    }
}
