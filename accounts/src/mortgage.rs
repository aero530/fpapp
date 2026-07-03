//! Loan type specifically tailored for mortgages

use serde::{Deserialize, Serialize};

use super::*;

/// Loan type specifically tailored for mortgages
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mortgage<T: std::cmp::Ord> {
    /// String describing this account
    name: String,
    /// Table of outstanding mortgage balance
    table: Table<T>,
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
    /// Number of times per year that interest is compounded. (1=yearly, 12=monthly)
    compound_time: f64,
    /// Mortgage insurance payment expressed as a yearly fixed number [in today's dollars]
    mortgage_insurance: f64,
    /// Loan to Value amount when mortgage insurance is no longer pulled from payment.  Since monthly payment does not change over time, after the insurance is done there is more money going to the principal each payment
    ltv_limit: f64,
    /// Amount of money going into escrow every year to pay for property tax.  This number is currently assumed to be constant (ie property taxes do not increase) [in today's dollars]
    escrow_value: f64,
    /// Current value of the home.  This is used to compute loan to value [in today's dollars]
    home_value: f64,
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

impl TryFrom<Mortgage<String>> for Mortgage<u32> {
    type Error = Error;
    fn try_from(other: Mortgage<String>) -> Result<Self, Self::Error> {
        Ok(Self {
            name: other.name,
            table: other.table.try_into()?,
            start_out: other.start_out,
            end_out: other.end_out,
            payment_type: other.payment_type,
            payment_value: other.payment_value,
            rate: other.rate,
            compound_time: other.compound_time,
            mortgage_insurance: other.mortgage_insurance,
            ltv_limit: other.ltv_limit,
            escrow_value: other.escrow_value,
            home_value: other.home_value,
            notes: other.notes,
            analysis: other.analysis,
            dates: other.dates,
        })
    }
}

impl Account for Mortgage<u32> {
    fn type_id(&self) -> AccountType {
        AccountType::Mortgage
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
        // Fail fast on inputs that would otherwise produce NaN in the interest math
        if self.compound_time <= 0_f64 {
            return Err(Error::config(format!(
                "Mortgage '{}': compound_time must be greater than zero (got {})",
                self.name, self.compound_time
            )));
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
        self.analysis.get_mortgage_plot_data()
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
            return Err(Error::internal("mortgage account value is negative"));
        }

        // Calculate interest (accrues regardless of payment window)
        // The formula for compound interest is P (1 + r/n)^(nt)
        //  P is the initial principal balance
        //  r is the interest rate
        //  n is the number of times interest is compounded per time period
        //  t is the number of time periods
        result.interest = self.analysis.value.get(year).unwrap()
            * f64::powf(
                1_f64 + (self.rate.value(settings) / 100_f64) / self.compound_time,
                self.compound_time,
            )
            - self.analysis.value.get(year).unwrap();

        self.analysis.interest.update(year, result.interest);
        self.analysis.value.update(year, result.interest);

        // Insurance, escrow, and payment only apply within the configured payment
        // window and while a balance remains (once the mortgage is paid off,
        // escrow and insurance are no longer collected through the payment)
        let balance = self.analysis.value.get(year).unwrap();
        if year_out.contains(year) && balance > 0_f64 {
            // Mortgage insurance applies while the loan-to-value ratio is above the
            // configured limit.  A home value of zero means LTV can't be computed;
            // treat it as no insurance rather than dividing by zero.
            let insurance_payment = if self.home_value > 0_f64 {
                let loan_to_value = balance / self.home_value * 100_f64;
                if loan_to_value > self.ltv_limit {
                    self.mortgage_insurance
                } else {
                    0_f64
                }
            } else {
                0_f64
            };
            self.analysis.insurance.update(year, insurance_payment);

            self.analysis.escrow.update(year, self.escrow_value);

            // The scheduled payment covers insurance and escrow first; the rest
            // pays down principal.  In the payoff year the payment is capped at
            // the remaining balance PLUS insurance and escrow, so the principal
            // actually reaches zero instead of leaving a residual balance.
            let scheduled = self.payment_type.value(self.payment_value, year, settings);
            result.payment = scheduled.min(balance + insurance_payment + self.escrow_value);
            self.analysis.payments.update(year, result.payment);

            let principal_payment =
                (result.payment - insurance_payment - self.escrow_value).max(0_f64);
            self.analysis.value.update(year, -principal_payment);
            // Zero out floating point dust left over from the final payment
            if self.analysis.value.get(year).unwrap() < 0.0001 {
                self.analysis.value.insert(year, 0_f64);
            }
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

    fn test_account(balance: f64) -> Mortgage<u32> {
        Mortgage {
            name: "Mortgage".into(),
            table: Table([(2000, balance)].into_iter().collect()),
            start_out: YearInput::ConstantInt(2000),
            end_out: YearInput::ConstantInt(2030),
            payment_type: PaymentOptions::Fixed,
            payment_value: 600.0,
            rate: PercentInput::ConstantFloat(0.0),
            compound_time: 12.0,
            mortgage_insurance: 50.0,
            ltv_limit: 0.0, // insurance always applies while a balance remains
            escrow_value: 50.0,
            home_value: 100000.0,
            notes: None,
            analysis: LoanTables::default(),
            dates: Dates::default(),
        }
    }

    #[test]
    fn mortgage_payoff_reaches_zero() {
        // Regression test for A4: the final payment must retire the balance even
        // though part of each payment goes to insurance and escrow.
        let mut account = test_account(1000.0);
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let totals = YearlyTotals::new();

        // Year 1: 600 payment - 50 insurance - 50 escrow = 500 principal
        let impact = account.simulate(2000, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, impact.expense, 600.0);
        assert_approx_eq!(f64, account.get_value(2000).unwrap(), 500.0);

        // Year 2: payment capped at 500 + 100 fees = 600; principal reaches zero
        let impact = account.simulate(2001, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, impact.expense, 600.0);
        assert_approx_eq!(f64, account.get_value(2001).unwrap(), 0.0);

        // Year 3: nothing left to pay
        let impact = account.simulate(2002, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, impact.expense, 0.0);
        assert_approx_eq!(f64, account.get_value(2002).unwrap(), 0.0);
    }

    #[test]
    fn mortgage_rejects_zero_compound_time() {
        // Regression test for B7: fail fast instead of producing NaN.
        let mut account = test_account(1000.0);
        account.compound_time = 0.0;
        let settings = test_settings_values();
        assert!(account.init(None, &settings).is_err());
    }

    #[test]
    fn mortgage_zero_home_value_means_no_insurance() {
        // Regression test for B7: no divide-by-zero on LTV.
        let mut account = test_account(1000.0);
        account.home_value = 0.0;
        let settings = test_settings_values();
        account.init(None, &settings).unwrap();
        let totals = YearlyTotals::new();
        account.simulate(2000, &totals, &settings, None).unwrap();
        assert_approx_eq!(f64, account.analysis.insurance.get(2000).unwrap(), 0.0);
        // full payment minus escrow goes to principal
        assert_approx_eq!(f64, account.get_value(2000).unwrap(), 1000.0 - 550.0);
    }
}
