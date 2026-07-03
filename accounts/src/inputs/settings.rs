//! Generic settings that impact the simulation / analysis results

use serde::{Deserialize, Serialize};

use crate::Error;

/// Generic span (something that has a min and max value)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Span<T> {
    /// Minimum value
    pub low: T,
    /// Maximum value
    pub high: T,
}

/// Social Security span settings
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SsaSettings {
    /// SSA breakpoints to interpolate between
    pub breakpoints: Span<f64>,
    /// taxable_income_percentage
    pub taxable_income_percentage: Span<f64>,
}

/// Analysis user settings
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// Age you plan to retire at
    pub age_retire: u32,
    /// How long you plan to live
    pub age_die: u32,
    /// Year you were born in
    pub year_born: u32,
    /// Year to start the simulation
    pub year_start: u32,
    /// Base rate of inflation (percent)
    pub inflation_base: f64,
    /// Tax rate for your income bracket
    pub tax_income: f64,
    /// Tax rate for capital gains
    pub tax_capital_gains: f64,
    /// Fraction of current spending when retired (such as in retirement you will spend 80% of what you spend now)
    pub retirement_cost_of_living: f64,
    /// Social Security settings
    pub ssa: SsaSettings,
}

impl Settings {
    pub fn year_start(&self) -> u32 {
        self.year_start
    }
    pub fn year_retire(&self) -> u32 {
        self.year_born + self.age_retire
    }
    pub fn year_die(&self) -> u32 {
        self.year_born + self.age_die
    }
    pub fn year_end(&self) -> u32 {
        self.year_born + self.age_die
    }
    pub fn is_retired(&self, year: u32) -> bool {
        year >= self.year_retire()
    }
    /// Reject configurations that would silently produce an empty or corrupt
    /// simulation.  Called once at the start of a run.
    pub fn validate(&self) -> Result<(), Error> {
        if self
            .year_born
            .checked_add(self.age_die.max(self.age_retire))
            .is_none()
        {
            return Err(Error::config(
                "year_born plus age_retire/age_die overflows a year value",
            ));
        }
        if self.year_start > self.year_end() {
            return Err(Error::config(format!(
                "year_start ({}) is after the end of the simulation ({}) — nothing to simulate",
                self.year_start,
                self.year_end()
            )));
        }
        if self.ssa.breakpoints.low > self.ssa.breakpoints.high {
            return Err(Error::config(format!(
                "SSA breakpoints are out of order: low ({}) must not exceed high ({})",
                self.ssa.breakpoints.low, self.ssa.breakpoints.high
            )));
        }
        if self.ssa.taxable_income_percentage.low > self.ssa.taxable_income_percentage.high {
            return Err(Error::config(format!(
                "SSA taxable percentages are out of order: low ({}) must not exceed high ({})",
                self.ssa.taxable_income_percentage.low, self.ssa.taxable_income_percentage.high
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_fixtures::test_settings_values;

    #[test]
    fn years() {
        let settings = test_settings_values();
        assert_eq!(settings.year_start(), 2000);
        assert_eq!(settings.year_retire(), 2030);
        assert_eq!(settings.year_die(), 2080);
        assert_eq!(settings.year_end(), 2080);
    }

    #[test]
    fn retirement() {
        let settings = test_settings_values();
        assert!(!settings.is_retired(2010));
        assert!(settings.is_retired(2090));
        assert!(settings.is_retired(2040));
    }

    #[test]
    fn validate_accepts_sane_settings() {
        assert!(test_settings_values().validate().is_ok());
    }

    #[test]
    fn validate_rejects_misordered_ssa_breakpoints() {
        // Regression test: low > high used to feed a negative term into the
        // SSA taxable-benefit formula.
        let mut settings = test_settings_values();
        settings.ssa.breakpoints.low = 50000.0;
        assert!(settings.validate().is_err());

        let mut settings = test_settings_values();
        settings.ssa.taxable_income_percentage.low = 95.0;
        assert!(settings.validate().is_err());
    }

    #[test]
    fn validate_rejects_empty_year_range() {
        let mut settings = test_settings_values();
        settings.year_start = 2100; // after year_end (2080)
        assert!(settings.validate().is_err());
    }

    #[test]
    fn validate_rejects_year_overflow() {
        let mut settings = test_settings_values();
        settings.year_born = u32::MAX - 10;
        assert!(settings.validate().is_err());
    }
}
