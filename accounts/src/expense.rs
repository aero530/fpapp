//! Generic expense account (things you spend money on)

use serde::{Deserialize, Serialize};
use std::error::Error;
#[cfg(feature = "plotters-backend")]
use image::{ImageBuffer, Rgba};

use super::*;

fn default_true() -> bool {
    true
}

/// Account type to represent generic expense
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Expense<T: std::cmp::Ord> {
    /// String describing this account
    name: String,
    /// Table of account expense for each year
    table: Table<T>,
    /// Calendar year when then expense of this account started to have impact
    start_out: YearInput,
    /// Calendar year when then expense of this account no longer has impact
    end_out: YearInput,
    /// Determines how to interpret expense_value
    expense_type: ExpenseOptions,
    /// Yearly cost of the expense [in today's dollars]
    expense_value: f64,
    /// This expense account is for healthcare costs.  If so it will pull first from HSA accounts.
    #[serde(default)] // default bool is false
    is_healthcare: bool,
    /// When false, this expense is not scaled by the retirement cost-of-living factor.
    /// Use this for fixed obligations (loan payments, insurance premiums) that do not
    /// change with retirement lifestyle. Defaults to true.
    #[serde(default = "default_true")]
    scales_with_col: bool,
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

impl TryFrom<Expense<String>> for Expense<u32> {
    type Error = Box<dyn Error>;
    fn try_from(other: Expense<String>) -> Result<Self, Self::Error> {
        Ok(Self {
            name: other.name,
            table: other.table.try_into()?,
            start_out: other.start_out,
            end_out: other.end_out,
            expense_type: other.expense_type,
            expense_value: other.expense_value,
            is_healthcare: other.is_healthcare,
            scales_with_col: other.scales_with_col,
            notes: other.notes,
            analysis: other.analysis,
            dates: other.dates,
        })
    }
}

impl Account for Expense<u32> {
    fn type_id(&self) -> AccountType {
        AccountType::Expense
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
    ) -> Result<(), Box<dyn Error>> {
        if linked_dates.is_some() {
            return Err(String::from("Linked account dates provided but not used").into());
        }
        // Seed the analysis with historical expense values so recorded actuals
        // are used (and plotted) instead of being discarded
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
    fn get_range_in(
        &self,
        _settings: &Settings,
        _linked_dates: Option<Dates>,
    ) -> Option<YearRange> {
        None
    }
    fn get_range_out(&self, settings: &Settings, linked_dates: Option<Dates>) -> Option<YearRange> {
        Some(YearRange {
            start: self
                .start_out
                .value(settings, linked_dates, YearEvalType::StartOut),
            end: self
                .end_out
                .value(settings, linked_dates, YearEvalType::EndOut),
        })
    }
    #[cfg(feature = "plotters-backend")]
    fn plot_to_file(&self, filepath: String, width: u32, height: u32) {
        scatter_plot_file(
            filepath,
            vec![("Amount".into(), &self.analysis.value)],
            self.name(),
            width,
            height,
        );
    }
    #[cfg(feature = "plotters-backend")]
    fn plot_to_buf(&self, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        scatter_plot_buf(
            vec![("Amount".into(), &self.analysis.value)],
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
        _totals: &YearlyTotals,
        settings: &Settings,
        _linked_value: Option<f64>,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let mut result = WorkingValues::default();

        // If this year is pre-seeded from the historical table, treat the
        // recorded amount as the actual expense for the year
        if let Some(actual) = self.analysis.value.get(year) {
            result.expense = actual;
        } else {
            self.analysis.add_year(year, false)?;

            // Cost of living scale to apply to the expense. Only applied when the account
            // is configured to scale with retirement cost-of-living (scales_with_col: true).
            let col_scale = if settings.is_retired(year) && self.scales_with_col {
                settings.retirement_cost_of_living / 100_f64
            } else {
                1_f64
            };

            // Calculate expense
            if self.dates.year_out.unwrap().contains(year) {
                result.expense = self.expense_type.value(self.expense_value, year, settings)
                    * col_scale;
            }

            // Update value table with expense value
            self.analysis.value.update(year, result.expense);
        }

        match self.is_healthcare {
            true => Ok(YearlyImpact {
                healthcare_expense: result.expense, // positive is outstanding (unpaid) expenses
                col: result.expense,
                ..Default::default()
            }),
            false => Ok(YearlyImpact {
                expense: result.expense,
                col: result.expense,
                ..Default::default()
            }),
        }
    }
    fn write(&self, filepath: String) {
        self.analysis.write(filepath);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::test_fixtures::test_settings_values;

    fn test_account(table: Table<u32>) -> Expense<u32> {
        Expense {
            name: "Expense Account".into(),
            table,
            start_out: YearInput::ConstantInt(2000),
            end_out: YearInput::ConstantInt(2020),
            expense_type: ExpenseOptions::Fixed,
            expense_value: 500_f64,
            is_healthcare: false,
            scales_with_col: true,
            notes: None,
            analysis: SingleTable::default(),
            dates: Dates::default(),
        }
    }

    #[test]
    fn expense_simulation() {
        let mut account = test_account(Table::default());
        let yearly_totals = YearlyTotals::new();
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let year = 2010_u32;
        let update = account
            .simulate(year, &yearly_totals, &settings, None)
            .unwrap();

        assert_eq!(
            account.analysis.value.get(year).unwrap(),
            account.expense_value
        );
        assert_eq!(update.expense, account.expense_value);
        assert_eq!(update.col, account.expense_value);
    }

    #[test]
    fn expense_uses_historical_actuals() {
        // Regression test for B3: recorded actual expenses override computed values.
        let mut account = test_account(Table([(2010, 750.0)].into_iter().collect()));
        let yearly_totals = YearlyTotals::new();
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let update = account
            .simulate(2010, &yearly_totals, &settings, None)
            .unwrap();
        assert_eq!(update.expense, 750.0);
    }
}
