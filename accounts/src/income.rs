//! Source of income

use serde::{Deserialize, Serialize};

use super::*;

/// Account to represent sources of income
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Income {
    /// String describing this account
    name: String,
    /// Table of account income
    table: Table,
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

impl Account for Income {
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
        if self.base < 0_f64 {
            return Err(Error::config(format!(
                "base pay must not be negative (got {})",
                self.base
            )));
        }
        // Unlike balance-holding accounts, income is a flow, not a balance —
        // there is no downstream invariant that requires it to be
        // non-negative, and a recorded net-loss year (e.g. a small business
        // or side income) is legitimate historical data.
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

        // If this year is pre-seeded from the historical table, use it as the
        // actual income — even outside the account's active date range, a
        // recorded actual is what really happened (this matches how Expense
        // treats its historical entries)
        if let Some(actual) = self.analysis.value.get(year) {
            return Ok(YearlyImpact {
                income: actual,
                income_taxable: actual,
                ..Default::default()
            });
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::test_fixtures::test_settings_values;
    use float_cmp::assert_approx_eq;

    fn test_account(table: Table) -> Income {
        Income {
            name: "Income".into(),
            table,
            base: 1000.0,
            start_in: YearInput::ConstantInt(2005),
            end_in: YearInput::ConstantInt(2020),
            raise: PercentInput::ConstantFloat(0.0),
            notes: None,
            analysis: SingleTable::default(),
            dates: Dates::default(),
        }
    }

    #[test]
    fn income_uses_historical_actuals_even_outside_window() {
        // A recorded actual is what really happened — it counts as income even
        // when the year falls outside the account's start/end window, matching
        // how Expense treats its historical entries.
        let mut account = test_account(Table([(2002, 999.0)].into_iter().collect()));
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let totals = YearlyTotals::new();
        let impact = account.simulate(2002, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, impact.income, 999.0);
        assert_approx_eq!(f64, impact.income_taxable, 999.0);
    }

    #[test]
    fn income_outside_window_without_actual_is_zero() {
        let mut account = test_account(Table::default());
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let totals = YearlyTotals::new();
        let impact = account.simulate(2002, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, impact.income, 0.0);
    }

    #[test]
    fn income_rejects_negative_base_at_init() {
        let mut account = test_account(Table::default());
        account.base = -5.0;
        let settings = test_settings_values();
        assert!(account.init(None, &settings).is_err());
    }

    #[test]
    fn income_allows_negative_historical_actual() {
        // A net-loss year (e.g. small-business or side income) is legitimate
        // historical data, not a config error — income is a flow, not a
        // balance, so there is no invariant it would violate downstream.
        let mut account = test_account(Table([(2010, -660.0)].into_iter().collect()));
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let totals = YearlyTotals::new();
        let impact = account.simulate(2010, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, impact.income, -660.0);
        assert_approx_eq!(f64, impact.income_taxable, -660.0);
    }
}
