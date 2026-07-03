//! User input payment values

use serde::{Deserialize, Serialize};

use super::{Settings, fixed_with_inflation};

/// used to populate account dropdown for payment type selection
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentOptions {
    /// fixed dollar amount
    Fixed,
    /// fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)
    FixedWithInflation,
}

impl PaymentOptions {
    /// The scheduled payment amount for the year (not capped by the loan balance —
    /// callers cap based on what the payment must cover)
    pub fn value(&self, payment_value: f64, year: u32, settings: &Settings) -> f64 {
        match self {
            PaymentOptions::Fixed => payment_value,
            PaymentOptions::FixedWithInflation => {
                fixed_with_inflation(payment_value, year, settings)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_fixtures::test_settings_values;
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn payment_options() {
        let settings = test_settings_values();
        assert_approx_eq!(
            f64,
            PaymentOptions::Fixed.value(500_f64, 2010, &settings),
            500_f64
        );
        // 500 * 1.05^10
        assert_approx_eq!(
            f64,
            PaymentOptions::FixedWithInflation.value(500_f64, 2010, &settings),
            814.4473,
            epsilon = 0.001
        );
    }
}
