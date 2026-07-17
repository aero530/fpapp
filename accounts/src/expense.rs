//! Generic expense account (things you spend money on)

use serde::{Deserialize, Serialize};

use super::*;

fn default_true() -> bool {
    true
}

/// Account type to represent generic expense
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Expense {
    /// String describing this account
    name: String,
    /// Table of account expense for each year
    table: Table,
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

impl Account for Expense {
    fn type_id(&self) -> AccountType {
        AccountType::Expense
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
        require_non_negative(self.expense_value, "expense value")?;
        // A negative healthcare actual would corrupt the HSA settlement; treat
        // refunds/rebates as income instead of a negative expense
        self.table.validate_non_negative()?;
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
        let year_out = self.dates.require_out()?;
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
            if year_out.contains(year) {
                result.expense =
                    self.expense_type.value(self.expense_value, year, settings) * col_scale;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::test_fixtures::test_settings_values;
    use float_cmp::assert_approx_eq;

    fn test_account(table: Table) -> Expense {
        Expense {
            name: "Expense Account".into(),
            table,
            start_out: YearInput::ConstantInt(2000),
            end_out: YearInput::ConstantInt(2080),
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

    #[test]
    fn expense_scales_with_retirement_cost_of_living() {
        // In retirement (year >= 2030 in the test settings) a scaling expense
        // is reduced to retirement_cost_of_living (80%).
        let mut account = test_account(Table::default());
        let yearly_totals = YearlyTotals::new();
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let update = account
            .simulate(2040, &yearly_totals, &settings, None)
            .unwrap();
        assert_approx_eq!(f64, update.expense, 400.0);

        // A non-scaling expense stays at full value in retirement.
        let mut fixed = test_account(Table::default());
        fixed.scales_with_col = false;
        fixed.init(None, &settings).unwrap();
        let update = fixed
            .simulate(2040, &yearly_totals, &settings, None)
            .unwrap();
        assert_approx_eq!(f64, update.expense, 500.0);
    }
}
