//! Social Security Account

use serde::{Deserialize, Serialize};
use std::error::Error;
use ts_rs::TS;
use image::{ImageBuffer, Rgba};

use super::*;

/// Social Security Account
#[derive(TS, Debug, Clone, Deserialize, Serialize)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Ssa {
    /// String describing this account
    name: String,
    /// Base income from social security
    base: f64,
    /// Calendar year when SSA benefits start
    start_in: YearInput,
    /// Calendar year when SSA benefits end
    end_in: YearInput,
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

impl Account for Ssa {
    fn type_id(&self) -> AccountType {
        AccountType::Ssa
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
        self.analysis = SingleTable::default();
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
        String::from("Hello")
    }
    fn plot_to_file(&self, filepath: String, width: u32, height: u32) {
        scatter_plot_file(
            filepath,
            vec![("Balance".into(), &self.analysis.value)],
            self.name(),
            width,
            height,
        );
    }
    fn plot_to_buf(&self, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        scatter_plot_buf(
            vec![
                ("Balance".into(), &self.analysis.value)
            ],
            self.name(),
            width,
            height,
        )
    }
    fn get_plot_data(&self) -> Vec<PlotDataSet> {
        self.analysis.get_plot_data()
    }
    fn simulate(
        &mut self,
        year: u32,
        totals: &YearlyTotals,
        settings: &Settings,
        _linked_value: Option<f64>,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let mut result = WorkingValues::default();

        if self.analysis.value.get(year).is_none() {
            self.analysis.add_year(year, false)?;
        }

        // Calculate earnings
        if self.dates.year_in.unwrap().contains(year) {
            result.earning = self.base;
        }

        // Add earnings to value tables
        self.analysis.value.update(year, result.earning);

        // Determine what fraction of SSA income is taxable based on combined income.
        // Other income is already accumulated in totals by the time SSA is simulated
        // (Income accounts run before Ssa in account_order).
        let other_income = totals.get_income(year);
        let combined_income = other_income + result.earning / 2.0;
        let taxable_fraction = if combined_income <= settings.ssa.breakpoints.low {
            0.0 // Below the low threshold: no SSA benefits are taxable (IRS rule)
        } else if combined_income >= settings.ssa.breakpoints.high {
            settings.ssa.taxable_income_percentage.high / 100.0
        } else {
            let t = (combined_income - settings.ssa.breakpoints.low)
                / (settings.ssa.breakpoints.high - settings.ssa.breakpoints.low);
            (settings.ssa.taxable_income_percentage.low
                + t * (settings.ssa.taxable_income_percentage.high
                    - settings.ssa.taxable_income_percentage.low))
                / 100.0
        };

        Ok(YearlyImpact {
            expense: 0_f64,
            healthcare_expense: 0_f64,
            col: 0_f64,
            saving: 0_f64,
            income_taxable: result.earning * taxable_fraction,
            income: result.earning,
            hsa: 0_f64,
        })
    }
    fn write(&self, filepath: String) {
        self.analysis.write(filepath);
    }
}
