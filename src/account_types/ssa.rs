//! Social Security Account
//!
use serde::{Deserialize, Serialize};
use std::error::Error;
use plotters::prelude::*;

use super::{
    Account, AccountResult, AccountType, AnalysisDates, SingleTable, YearRange, YearlyImpact,
    YearlyTotals, range
};
use crate::inputs::{YearEvalType, YearInput};
use crate::settings::Settings;

/// Social Security Account
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ssa {
    name: String,
    base: f64,
    start_in: YearInput,
    end_in: YearInput,
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    #[serde(skip)]
    analysis: Option<SingleTable>,
    #[serde(skip)]
    dates: Option<AnalysisDates>,
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
        years: &Vec<u32>,
        linked_dates: Option<AnalysisDates>,
        settings: &Settings,
    ) -> Result<(), Box<dyn Error>> {
        if linked_dates.is_some() {
            return Err(String::from("Linked account dates provided but not used").into());
        }
        let mut output = SingleTable::default();
        years.iter().for_each(|year| {
            output.value.0.insert(*year, 0.0);
        });
        self.analysis = Some(output);
        self.dates = Some(AnalysisDates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        });
        Ok(())
    }
    fn get_value(&self, year: u32) -> Option<f64> {
        match &self.analysis {
            Some(result) => result.value.0.get(&year).map(|v| *v),
            None => None,
        }
    }
    fn get_income(&self, year: u32) -> Option<f64> {
        self.get_value(year)
    }
    fn get_expense(&self, _year: u32) -> Option<f64> {
        None
    }
    fn get_range_in(
        &self,
        settings: &Settings,
        linked_dates: Option<AnalysisDates>,
    ) -> Option<YearRange> {
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
        _linked_dates: Option<AnalysisDates>,
    ) -> Option<YearRange> {
        None
    }
    fn plot(&self, filepath: String) {

        let value = self.analysis.as_ref().unwrap().value.clone();
        let (x_min, x_max, y_min, y_max) = range(vec![&value]);

        let root = BitMapBackend::new(&filepath, (640, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .caption(self.name(), ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_min..x_max, y_min..y_max).unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(value.into_iter(),&RED)).unwrap()
            .label("balance")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw().unwrap();
    }
    fn simulate(
        &mut self,
        _year: u32,
        _totals: &YearlyTotals,
        _settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let mut _result = AccountResult::default();

        Ok(YearlyImpact {
            expense: 0_f64,
            col: 0_f64,
            saving: 0_f64,
            income_taxable: 0_f64,
            income: 0_f64,
        })
    }
    fn write(&self, filepath: String) {
        match &self.analysis {
            Some(results) => results.write(filepath),
            None => {}
        }
    }
}
