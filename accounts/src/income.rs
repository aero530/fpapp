//! Source of income

use serde::{Deserialize, Serialize};

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

impl TryFrom<Income<String>> for Income<u32> {
    type Error = Error;
    fn try_from(other: Income<String>) -> Result<Self, Self::Error> {
        Ok(Self {
            name: other.name,
            base: other.base,
            table: other.table.try_into()?,
            start_in: other.start_in,
            end_in: other.end_in,
            raise: other.raise,
            notes: other.notes,
            analysis: other.analysis,
            dates: other.dates,
        })
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
    fn init(&mut self, linked_dates: Option<Dates>, settings: &Settings) -> Result<(), Error> {
        if linked_dates.is_some() {
            return Err(Error::config("linked account dates provided but not used"));
        }
        self.analysis = SingleTable::new(&self.table);
        self.dates = Dates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        };
        Ok(())
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
    fn get_plot_data(&self) -> Vec<PlotDataSet> {
        self.analysis.get_plot_data()
    }
    fn simulate(
        &mut self,
        year: u32,
        _totals: &YearlyTotals,
        settings: &Settings,
        _linked_value: Option<f64>,
    ) -> Result<YearlyImpact, Error> {
        let year_in = self.dates.require_in()?;
        let mut result = WorkingValues::default();

        // If this year is pre-seeded from the historical table, use it as the actual income
        if let Some(actual) = self.analysis.value.get(year) {
            if year_in.contains(year) {
                return Ok(YearlyImpact {
                    income: actual,
                    income_taxable: actual,
                    ..Default::default()
                });
            }
            // Pre-seeded but outside active date range — record zero income
            return Ok(YearlyImpact::default());
        }

        self.analysis.add_year(year, false)?;

        // Calculate earnings
        if year_in.contains(year) {
            let raise = self.raise.value(settings) / 100.0 + 1.0;
            result.earning =
                self.base * f64::powf(raise, year.saturating_sub(year_in.start) as f64);
        }

        // Add earnings to value tables
        self.analysis.value.update(year, result.earning);

        Ok(YearlyImpact {
            income_taxable: result.earning,
            income: result.earning,
            ..Default::default()
        })
    }
}
