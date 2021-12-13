//! User input payment values

use serde::{Deserialize, Serialize};

use super::fixed_with_inflation;

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
    pub fn value(
        self,
        payment: f64,
        inflation: f64,
        duration: u32,
        outstanding_balance: f64,
    ) -> f64 {
        let output = match self {
            PaymentOptions::Fixed => payment,
            PaymentOptions::FixedWithInflation => {
                fixed_with_inflation(payment, inflation, duration)
            }
        };
        if output > outstanding_balance {
            outstanding_balance
        } else {
            output
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn payment_options() {
        let payment1 = PaymentOptions::Fixed;
        let payment2 = PaymentOptions::FixedWithInflation;
        assert_approx_eq!(
            f64,
            payment1.value(500_f64, 10_f64, 10_u32, 1000_f64),
            500_f64
        );
        assert_approx_eq!(
            f64,
            payment1.value(500_f64, 10_f64, 10_u32, 100_f64),
            100_f64
        );
        assert_approx_eq!(
            f64,
            payment2.value(500_f64, 10_f64, 10_u32, 5000_f64),
            1296.8712,
            epsilon = 0.001
        );
        assert_approx_eq!(
            f64,
            payment2.value(500_f64, 10_f64, 10_u32, 500_f64),
            500_f64
        );
    }
}
