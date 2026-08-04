//! Social Security Account

use serde::{Deserialize, Serialize};

use super::*;

/// Social Security Account
#[derive(Debug, Clone, Deserialize, Serialize)]
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
    fn init(&mut self, linked_dates: Option<Dates>, settings: &Settings) -> Result<(), Error> {
        if linked_dates.is_some() {
            return Err(Error::config("linked account dates provided but not used"));
        }
        require_non_negative(self.base, "base benefit")?;
        self.analysis = SingleTable::default();
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
        totals: &YearlyTotals,
        settings: &Settings,
        _linked_value: Option<f64>,
    ) -> Result<YearlyImpact, Error> {
        let year_in = self.dates.require_in()?;
        let mut result = WorkingValues::default();

        if self.analysis.value.get(year).is_none() {
            self.analysis.add_year(year, false)?;
        }

        // Calculate earnings
        if year_in.contains(year) {
            result.earning = self.base;
        }

        // Add earnings to value tables
        self.analysis.value.update(year, result.earning);

        // Determine how much of the SSA benefit is taxable based on combined
        // income.  Ssa accounts are simulated last within a year, so income
        // already includes wages plus retirement/savings withdrawals for this
        // year (another Ssa account simulated earlier contributes its full
        // benefit rather than half — an accepted approximation).
        //
        // This follows the IRS worksheet structure using the configured
        // breakpoints and percentage tiers (federal defaults: breakpoints
        // 25k/34k single, percentages 50%/85%):
        //   combined <= low:          nothing is taxable
        //   low < combined <= high:   pct_low of the excess over low,
        //                             capped at pct_low of the benefit
        //   combined > high:          pct_high of the excess over high plus the
        //                             middle-tier amount, capped at pct_high
        //                             of the benefit
        //
        // Breakpoint and percentage ordering (low <= high) is enforced by
        // Settings::validate at the start of the run.
        let benefit = result.earning;
        let combined_income = totals.get_income(year) + benefit / 2.0;
        let low = settings.ssa.breakpoints.low;
        let high = settings.ssa.breakpoints.high;
        let pct_low = settings.ssa.taxable_income_percentage.low / 100_f64;
        let pct_high = settings.ssa.taxable_income_percentage.high / 100_f64;

        let taxable_benefit = if combined_income <= low {
            0_f64
        } else if combined_income <= high {
            (pct_low * (combined_income - low)).min(pct_low * benefit)
        } else {
            (pct_high * (combined_income - high) + (pct_low * (high - low)).min(pct_low * benefit))
                .min(pct_high * benefit)
        };

        Ok(YearlyImpact {
            income_taxable: taxable_benefit,
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

    fn test_account() -> Ssa {
        Ssa {
            name: "SSA".into(),
            base: 10000.0,
            start_in: YearInput::ConstantInt(2000),
            end_in: YearInput::ConstantInt(2080),
            notes: None,
            analysis: SingleTable::default(),
            dates: Dates::default(),
        }
    }

    fn totals_with_income(year: u32, income: f64) -> YearlyTotals {
        let mut totals = YearlyTotals::new();
        totals.add_year(year, false).unwrap();
        totals.update(
            year,
            YearlyImpact {
                income,
                ..Default::default()
            },
        );
        totals
    }

    #[test]
    fn ssa_untaxed_below_low_breakpoint() {
        // combined = 20000 + 5000 = 25000 <= 30000
        let mut account = test_account();
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let impact = account
            .simulate(2010, &totals_with_income(2010, 20000.0), &settings, None)
            .unwrap();
        assert_approx_eq!(f64, impact.income_taxable, 0.0);
        assert_approx_eq!(f64, impact.income, 10000.0);
    }

    #[test]
    fn ssa_middle_tier_uses_low_percentage() {
        // Regression test for A6: the low tier percentage must be used.
        // combined = 27000 + 5000 = 32000; excess over low = 2000
        // taxable = min(50% * 2000, 50% * 10000) = 1000
        let mut account = test_account();
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let impact = account
            .simulate(2010, &totals_with_income(2010, 27000.0), &settings, None)
            .unwrap();
        assert_approx_eq!(f64, impact.income_taxable, 1000.0);
    }

    #[test]
    fn ssa_top_tier_capped_at_high_percentage_of_benefit() {
        // combined = 50000 + 5000 = 55000 > 40000
        // taxable = min(80% * 15000 + min(50% * 10000, 50% * 10000), 80% * 10000)
        //         = min(12000 + 5000, 8000) = 8000
        let mut account = test_account();
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let impact = account
            .simulate(2010, &totals_with_income(2010, 50000.0), &settings, None)
            .unwrap();
        assert_approx_eq!(f64, impact.income_taxable, 8000.0);
    }

    #[test]
    fn simulate_before_init_is_an_error_not_a_panic() {
        // Regression test: dates unresolved must produce an error.
        let mut account = test_account();
        let settings = test_settings_values();
        let totals = YearlyTotals::new();
        assert!(account.simulate(2010, &totals, &settings, None).is_err());
    }
}
