//! Definitions for user input fields

use serde::{Deserialize, Serialize};

use super::settings;
use crate::Dates;

/// description used to populate account dropdown for contribution type selection
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContributionOptions {
    /// fixed dollar amount
    Fixed,
    /// percent of income
    PercentOfIncome,
    /// fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)
    FixedWithInflation,
}

impl ContributionOptions {
    pub fn value(self, contribution: f64, income: f64, duration: u32, inflation: f64) -> f64 {
        match self {
            ContributionOptions::Fixed => {
                // set the contribution amount to the value input
                contribution
            }
            ContributionOptions::PercentOfIncome => {
                // calculate the contribution using the total income for the year
                income * contribution / 100_f64
            }
            ContributionOptions::FixedWithInflation => {
                // increase the value by inflation
                contribution * f64::powf(1_f64 + inflation / 100_f64, duration as f64)
            }
        }
    }
}

/// Employer matching for retirement accounts
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct EmployerMatch {
    /// Percentage that the employer will match (such as 50%)
    pub amount: PercentInput,
    /// Limit of employer's match (such as up to the first 6% that the employee saves)
    pub limit: PercentInput,
}

/// used to populate account dropdown for expense type selection
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExpenseOptions {
    /// fixed dollar amount
    Fixed,
    /// fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)
    FixedWithInflation,
}

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
                payment * f64::powf(1_f64 + inflation / 100_f64, duration as f64)
            }
        };
        if output > outstanding_balance {
            outstanding_balance
        } else {
            output
        }
    }
}

/// These values can be input as constants or as computed values (strings)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum PercentInput {
    /// Calculated value based on suggested options
    Calculate(PercentSuggestions),
    /// Constant value
    ConstantFloat(f64),
    /// Constant string
    ConstantString(String),
}

impl PercentInput {
    pub fn value(&self, settings: &settings::Settings) -> f64 {
        match self {
            Self::Calculate(input) => input.value(settings),
            Self::ConstantFloat(input) => *input,
            Self::ConstantString(input) => input.parse().unwrap(),
        }
    }
}

/// Options for strings on percent inputs
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PercentSuggestions {
    InflationBase,
}

impl PercentSuggestions {
    pub fn value(&self, settings: &settings::Settings) -> f64 {
        match self {
            Self::InflationBase => settings.inflation_base,
        }
    }
}

/// used to populate account dropdown for tax status selection
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxStatus {
    /// Paid with taxed income, earnings are not taxed, withdrawals are not taxed
    ///
    /// Contributions count as an expense (will be subtracted from net for the year).
    /// Contributions do not impact taxable income (as they are made with dollars that have already been taxed).
    /// Withdrawals count as income but do not to taxable income.
    /// aka 0
    ContributeTaxedEarningsUntaxedWhenUsed,
    /// Paid with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed (tax free as long as used for intended purpose)
    ///
    /// Contributions count as an expense (will be subtracted from net for the year).
    /// Contributions do not impact taxable income (as they are made with dollars that have already been taxed).
    /// Withdrawals count as income but do not to taxable income.
    /// aka 1
    ContributeTaxedEarningsTaxed,
    // not implemented.
    // NOT IMPLEMENTED ## 2=payed with taxed income, earnings are taxed in year taken out as capital gains, withdrawals are not taxed
    // aka 2
    // NotImpliemented,
    /// Paid with pretax income and taxed in year of use as income
    ///
    /// Contributions count as an expense (will be subtracted from net for the year).
    /// Contributions reduce taxable income (they are a deduction).
    /// Withdrawals count as income and add to taxable income.
    /// aka 3
    ContributePretaxTaxedWhenUsed,
    /// Paid with pretax income and not taxed as income (use with HSA)
    ///
    /// Contributions count as an expense (will be subtracted from net for the year).
    /// Contributions reduce taxable income (they are a deduction).
    /// Withdrawals count as income but do not add to taxable income.
    /// aka 4
    ContributePretaxUntaxedWhenUsed,
}

/// used to populate account dropdown for withdrawal type selection
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WithdrawalOptions {
    /// fixed dollar amount
    Fixed,
    /// fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)
    FixedWithInflation,
    /// take money out in equal amounts each year such that the balance at endOut is zero
    EndAtZero,
    /// cost of living fraction of total savings
    /// take out the current cost of living * (this accounts value / total savings)
    ColFracOfSavings,
}

impl WithdrawalOptions {
    /// Calculate the value of the withdrawal
    ///
    /// match reference_value {
    ///     EndAtZero => current value of the account
    ///     ColFracOfSavings => previous year's value of the account
    ///
    pub fn value(
        &self,
        withdrawal: f64,
        inflation: f64,
        dates: Dates,
        year: u32,
        account_value: f64,
        prev_account_value: f64,
        col: f64,
        prev_savings: f64,
        tax_income: f64,
        tax_status: TaxStatus,
    ) -> f64 {
        let output = match self {
            WithdrawalOptions::Fixed => {
                // debug!("{}", withdrawal);
                withdrawal
            }
            WithdrawalOptions::FixedWithInflation => {
                // let i = inflation.ok_or::<String>("Inflation not provided".into())?;
                // let y = year.ok_or::<String>("Year not provided".into())?;
                // let s = start_in.ok_or::<String>("Start In not provided".into())?;
                let start_in = dates.year_in.unwrap().start;
                // debug!("{} {}", withdrawal, start_in);
                withdrawal * f64::powf(1_f64 + inflation / 100_f64, (year - start_in) as f64)
            }
            WithdrawalOptions::EndAtZero => {
                let end_out = dates.year_out.unwrap().end;
                // debug!("{} {} {}", year, end_out, account_value);
                if year <= end_out {
                    // if the year to stop taking money out of the account is beyond or equal to the current year
                    // calculate the fraction of the account balance to withdraw
                    account_value / (end_out - year + 1) as f64
                } else {
                    0_f64
                }
            }
            WithdrawalOptions::ColFracOfSavings => {
                // debug!(
                //     "pv{} ps{} ti{} col{} ts{:?}",
                //     prev_account_value, prev_savings, tax_income, col, tax_status
                // );
                if prev_account_value > 0_f64 && prev_savings > 0_f64 {
                    // if there is money left in the account
                    // withdrawal from this account = total expenses this year  * fraction of total savings this account represents
                    // total expenses this year is reduced by the income during retirement for the year.
                    // incomeDuringRetirement is tracked because withdrawals from retirement accounts go into the income table but we want to
                    // pay for expenses from money earned in this year before pulling from retirement accounts.
                    //      const totalExpensesThisYear = Object.values(expenseTotal[yearCurrent]).reduce((acc, cur) => acc + cur, 0) - incomeDuringRetirement[yearCurrent];
                    //      withdrawal = (totalExpensesThisYear * account.table[yearCurrent - 1]) / savingsTotalTable[yearCurrent - 1];

                    match tax_status {
                        TaxStatus::ContributePretaxTaxedWhenUsed => {
                            // add extra to amount withdrawal value to account for taxes.
                            // col * (prev_account_value / prev_savings) * (tax_income / 100_f64 + 1_f64)
                            col * (prev_account_value / prev_savings) / (1_f64 - tax_income / 100_f64)
                        }
                        _ => col * (prev_account_value / prev_savings),
                    }
                } else {
                    0_f64
                }
            }
        };
        match output > account_value {
            true => account_value,
            false => output,
        }
    }
}

/// Options for strings on year inputs
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum YearEvalType {
    StartIn,
    EndIn,
    StartOut,
    EndOut,
}

/// Struct to hold info about computed year values
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct YearComputation {
    base: YearSuggestion,
    delta: i32,
}

/// These values can be input as constants or as computed values (strings)
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum YearInput {
    /// Calculated value based on suggested options
    Calculate(YearComputation),
    /// Suggested values
    Suggested(YearSuggestion),
    /// Constant value
    ConstantInt(u32),
}

impl YearInput {
    pub fn value(
        &self,
        settings: &settings::Settings,
        linked_dates: Option<Dates>,
        eval_type: YearEvalType,
    ) -> u32 {
        match self {
            Self::Calculate(input) => {
                (input.base.value(settings, linked_dates, eval_type) as i32 + input.delta) as u32
            }
            Self::Suggested(input) => input.value(settings, linked_dates, eval_type),
            Self::ConstantInt(input) => *input,
        }
    }
}

/// Options for strings on year inputs
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum YearSuggestion {
    /// Start of simulation
    //#[serde(rename(deserialize="yearStart"))]
    YearStart,
    /// When you plan to retire
    //#[serde(rename="yearRetire")]
    YearRetire,
    /// When you plan to die
    //#[serde(rename="yearDie")]
    YearDie,
    /// Last year of the simulation
    //#[serde(rename="yearEnd")]
    YearEnd,
    /// Pull date from linked account
    IncomeLink,
}

impl YearSuggestion {
    pub fn value(
        &self,
        settings: &settings::Settings,
        linked_dates: Option<Dates>,
        eval_type: YearEvalType,
    ) -> u32 {
        match self {
            Self::YearStart => settings.year_start(),
            Self::YearRetire => settings.year_retire(),
            Self::YearDie => settings.year_die(),
            Self::YearEnd => settings.year_end(),
            Self::IncomeLink => match eval_type {
                YearEvalType::StartIn => linked_dates.unwrap().year_in.unwrap().start,
                YearEvalType::EndIn => linked_dates.unwrap().year_in.unwrap().end,
                YearEvalType::StartOut => linked_dates.unwrap().year_out.unwrap().start,
                YearEvalType::EndOut => linked_dates.unwrap().year_out.unwrap().end,
            },
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;
    use crate::inputs::Settings;
    use crate::simulation::YearRange;

    #[test]
    fn contribution_options() {
        let cont1 = ContributionOptions::Fixed;
        let cont2 = ContributionOptions::PercentOfIncome;
        let cont3 = ContributionOptions::FixedWithInflation;
        assert_approx_eq!(f64, cont1.value(500_f64, 10000_f64, 10_u32, 10_f64), 500_f64);
        assert_approx_eq!(f64, cont2.value(10_f64, 10000_f64, 1_u32, 10_f64), 1000_f64);
        assert_approx_eq!(f64, cont3.value(500_f64, 10000_f64, 1_u32, 10_f64), 550_f64, epsilon = 0.001);
        assert_approx_eq!(f64, cont3.value(500_f64, 10000_f64, 10_u32, 10_f64), 1296.8712, epsilon = 0.001);
    }

    #[test]
    fn payment_options() {
        let payment1 = PaymentOptions::Fixed;
        let payment2 = PaymentOptions::FixedWithInflation;
        assert_approx_eq!(f64, payment1.value(500_f64, 10_f64, 10_u32, 1000_f64), 500_f64);
        assert_approx_eq!(f64, payment1.value(500_f64, 10_f64, 10_u32, 100_f64), 100_f64);
        assert_approx_eq!(f64, payment2.value(500_f64, 10_f64, 10_u32, 5000_f64), 1296.8712, epsilon = 0.001);
        assert_approx_eq!(f64, payment2.value(500_f64, 10_f64, 10_u32, 500_f64), 500_f64);
    }

    #[test]
    fn percent_input() {
        let p1 = PercentInput::Calculate(PercentSuggestions::InflationBase);
        let p2 = PercentInput::ConstantFloat(75_f64);
        let p3 = PercentInput::ConstantString("40".into());

        assert_approx_eq!(f64, p1.value(&Settings::test_values()), 5_f64);
        assert_approx_eq!(f64, p2.value(&Settings::test_values()), 75_f64);
        assert_approx_eq!(f64, p3.value(&Settings::test_values()), 40_f64);
    }

    #[test]
    fn withdrawal_options() {
        let w1 = WithdrawalOptions::Fixed;
        let w2 = WithdrawalOptions::FixedWithInflation;
        let w3 = WithdrawalOptions::ColFracOfSavings;
        let w4 = WithdrawalOptions::EndAtZero;

        let withdrawal = 100_f64;
        let inflation = 10_f64;
        let dates = Dates {
            year_in: Some(YearRange{start: 2020, end: 2080}),
            year_out: Some(YearRange{start: 2020, end: 2080}),
        };
        let year = 2030;
        let account_value = 5000_f64;
        let prev_account_value = 6000_f64;
        let col = 10000_f64;
        let prev_savings = 20000_f64;
        let tax_income = 20_f64;
        let tax_status = TaxStatus::ContributePretaxTaxedWhenUsed;

        assert_approx_eq!(f64, w1.value(withdrawal, inflation, dates, year, account_value, prev_account_value, col, prev_savings, tax_income, tax_status), withdrawal);

        // Don't allow more to be taken out than there is in the account
        assert_approx_eq!(f64, w1.value(100000_f64, inflation, dates, year, account_value, prev_account_value, col, prev_savings, tax_income, tax_status), account_value);
        
        assert_approx_eq!(f64, w2.value(withdrawal, inflation, dates, year, account_value, prev_account_value, col, prev_savings, tax_income, tax_status), 259.374, epsilon = 0.001);
        assert_approx_eq!(f64, w2.value(withdrawal, inflation, dates, dates.year_out.unwrap().start+1, account_value, prev_account_value, col, prev_savings, tax_income, tax_status), 110_f64);

        assert_approx_eq!(f64, w3.value(withdrawal, inflation, dates, year, account_value, 0_f64, col, prev_savings, tax_income, tax_status), 0_f64);
        assert_approx_eq!(f64, w3.value(withdrawal, inflation, dates, year, account_value, prev_account_value, 0_f64, prev_savings, tax_income, tax_status), 0_f64);
        assert_approx_eq!(f64, w3.value(withdrawal, inflation, dates, year, account_value, 0_f64, 0_f64, prev_savings, tax_income, tax_status), 0_f64);
        assert_approx_eq!(f64, w3.value(withdrawal, inflation, dates, year, account_value, prev_account_value, col, prev_savings, tax_income, TaxStatus::ContributePretaxUntaxedWhenUsed), 3000_f64);
        assert_approx_eq!(f64, w3.value(withdrawal, inflation, dates, year, account_value, prev_account_value, col, prev_savings, tax_income, TaxStatus::ContributePretaxTaxedWhenUsed), 3750_f64);
        
        assert_approx_eq!(f64, w4.value(withdrawal, inflation, dates, dates.year_out.unwrap().start, account_value, prev_account_value, col, prev_savings, tax_income, tax_status), 81.967, epsilon = 0.001);
    }

    #[test]
    fn year_input() {
        todo!()
    }
}


