//! Generic loan

use serde::{Deserialize, Serialize};

use super::*;

/// Generic loan
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Loan {
    /// String describing this account
    name: String,
    /// Table of outstanding loan balance
    table: Table,
    /// Calendar year when payments to this account start
    start_out: YearInput,
    /// Calendar year when payments to this account stop
    end_out: YearInput,
    /// Determines how to interpret payment_value
    payment_type: PaymentOptions,
    /// How much money should be payed each year (either as a percentage or a fixed dollar amount) [in today's dollars]
    payment_value: f64,
    /// Interest rate on borrowed money. This is an APR this is then compounded based on the compound time setting.  Used for LOAN and MORTGAGE account types.
    rate: PercentInput,
    /// General information to store with this account
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    /// Tables used to store simulation results
    #[serde(skip)]
    analysis: LoanTables,
    /// Calculated date values as a year based on input values
    #[serde(skip)]
    dates: Dates,
}

impl Account for Loan {
    fn type_id(&self) -> AccountType {
        AccountType::Loan
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

        self.analysis = LoanTables::new(
            &self.table,
            &Table::default(),
            &Table::default(),
            &Table::default(),
            &Table::default(),
        );
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

        // Skip add_year for pre-seeded years; the table value is the starting balance
        if self.analysis.value.get(year).is_none() {
            self.analysis.add_year(year, true)?;
        }

        if self.analysis.value.get(year).unwrap() < 0_f64 {
            return Err(Error::internal("loan account value is negative"));
        }

        // Calculate interest
        result.interest =
            self.analysis.value.get(year).unwrap() * self.rate.value(settings) / 100_f64;

        // Add interest to interest and value tables
        self.analysis.interest.update(year, result.interest);
        self.analysis.value.update(year, result.interest);

        // Calculate payment amount (capped at the outstanding balance)
        if year_out.contains(year) {
            let scheduled = self.payment_type.value(self.payment_value, year, settings);
            result.payment = scheduled.min(self.analysis.value.get(year).unwrap());
        }

        // Add payment to payment and value tables
        self.analysis.payments.update(year, result.payment);
        self.analysis.value.update(year, -result.payment);
        // Limit min value of the loan balance to account for floating point math rounding
        if self.analysis.value.get(year).unwrap() < 0.0001 {
            self.analysis.value.insert(year, 0_f64);
        }

        Ok(YearlyImpact {
            expense: result.payment,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::test_fixtures::test_settings_values;
    use float_cmp::assert_approx_eq;

    fn test_account(payment_type: PaymentOptions, payment_value: f64, rate: f64) -> Loan {
        Loan {
            name: "Loan".into(),
            table: Table([(2000, 1000.0)].into_iter().collect()),
            start_out: YearInput::ConstantInt(2000),
            end_out: YearInput::ConstantInt(2030),
            payment_type,
            payment_value,
            rate: PercentInput::ConstantFloat(rate),
            notes: None,
            analysis: LoanTables::default(),
            dates: Dates::default(),
        }
    }

    #[test]
    fn loan_amortizes_and_stops_paying_at_zero() {
        let mut account = test_account(PaymentOptions::Fixed, 550.0, 10.0);
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let totals = YearlyTotals::new();

        // Year 1: 1000 * 1.1 = 1100; pay 550 -> 550
        let impact = account.simulate(2000, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, impact.expense, 550.0);
        assert_approx_eq!(f64, account.get_value(2000).unwrap(), 550.0);

        // Year 2: 550 * 1.1 = 605; pay 550 -> 55
        account.simulate(2001, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, account.get_value(2001).unwrap(), 55.0);

        // Year 3: 55 * 1.1 = 60.5; payment capped at 60.5 -> 0
        let impact = account.simulate(2002, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, impact.expense, 60.5);
        assert_approx_eq!(f64, account.get_value(2002).unwrap(), 0.0);

        // Year 4: nothing left to pay
        let impact = account.simulate(2003, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, impact.expense, 0.0);
    }

    #[test]
    fn loan_inflation_adjusted_payment_grows() {
        // FixedWithInflation payments grow with the inflation setting (5%).
        let mut account = test_account(PaymentOptions::FixedWithInflation, 100.0, 0.0);
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let totals = YearlyTotals::new();

        let impact = account.simulate(2000, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, impact.expense, 100.0);
        let impact = account.simulate(2001, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, impact.expense, 105.0);
        let impact = account.simulate(2002, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, impact.expense, 110.25, epsilon = 0.001);
    }
}
