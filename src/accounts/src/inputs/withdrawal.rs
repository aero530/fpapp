//! User input withdrawal and tax status values

use serde::{Deserialize, Serialize};

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
    /// Withdrawals are manually calculated or are none
    Other,
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::simulation::YearRange;
//     use float_cmp::assert_approx_eq;

//     #[test]
//     fn withdrawal_options() {
//         let w1 = WithdrawalOptions::Fixed;
//         let w2 = WithdrawalOptions::FixedWithInflation;
//         let w3 = WithdrawalOptions::ColFracOfSavings;
//         let w4 = WithdrawalOptions::EndAtZero;

//         let withdrawal = 100_f64;
//         let inflation = 10_f64;
//         let dates = Dates {
//             year_in: Some(YearRange {
//                 start: 2020,
//                 end: 2080,
//             }),
//             year_out: Some(YearRange {
//                 start: 2020,
//                 end: 2080,
//             }),
//         };
//         let year = 2030;
//         let account_value = 5000_f64;
//         let prev_account_value = 6000_f64;
//         let col = 10000_f64;
//         let prev_savings = 20000_f64;
//         let tax_income = 20_f64;
//         let tax_status = TaxStatus::ContributePretaxTaxedWhenUsed;

//         assert_approx_eq!(
//             f64,
//             w1.value(
//                 withdrawal,
//                 inflation,
//                 dates,
//                 year,
//                 account_value,
//                 prev_account_value,
//                 col,
//                 prev_savings,
//                 tax_income,
//                 tax_status
//             ),
//             withdrawal
//         );

//         // Don't allow more to be taken out than there is in the account
//         assert_approx_eq!(
//             f64,
//             w1.value(
//                 100000_f64,
//                 inflation,
//                 dates,
//                 year,
//                 account_value,
//                 prev_account_value,
//                 col,
//                 prev_savings,
//                 tax_income,
//                 tax_status
//             ),
//             account_value
//         );

//         assert_approx_eq!(
//             f64,
//             w2.value(
//                 withdrawal,
//                 inflation,
//                 dates,
//                 year,
//                 account_value,
//                 prev_account_value,
//                 col,
//                 prev_savings,
//                 tax_income,
//                 tax_status
//             ),
//             259.374,
//             epsilon = 0.001
//         );
//         assert_approx_eq!(
//             f64,
//             w2.value(
//                 withdrawal,
//                 inflation,
//                 dates,
//                 dates.year_out.unwrap().start + 1,
//                 account_value,
//                 prev_account_value,
//                 col,
//                 prev_savings,
//                 tax_income,
//                 tax_status
//             ),
//             110_f64
//         );

//         assert_approx_eq!(
//             f64,
//             w3.value(
//                 withdrawal,
//                 inflation,
//                 dates,
//                 year,
//                 account_value,
//                 0_f64,
//                 col,
//                 prev_savings,
//                 tax_income,
//                 tax_status
//             ),
//             0_f64
//         );
//         assert_approx_eq!(
//             f64,
//             w3.value(
//                 withdrawal,
//                 inflation,
//                 dates,
//                 year,
//                 account_value,
//                 prev_account_value,
//                 0_f64,
//                 prev_savings,
//                 tax_income,
//                 tax_status
//             ),
//             0_f64
//         );
//         assert_approx_eq!(
//             f64,
//             w3.value(
//                 withdrawal,
//                 inflation,
//                 dates,
//                 year,
//                 account_value,
//                 0_f64,
//                 0_f64,
//                 prev_savings,
//                 tax_income,
//                 tax_status
//             ),
//             0_f64
//         );
//         assert_approx_eq!(
//             f64,
//             w3.value(
//                 withdrawal,
//                 inflation,
//                 dates,
//                 year,
//                 account_value,
//                 prev_account_value,
//                 col,
//                 prev_savings,
//                 tax_income,
//                 TaxStatus::ContributePretaxUntaxedWhenUsed
//             ),
//             3000_f64
//         );
//         assert_approx_eq!(
//             f64,
//             w3.value(
//                 withdrawal,
//                 inflation,
//                 dates,
//                 year,
//                 account_value,
//                 prev_account_value,
//                 col,
//                 prev_savings,
//                 tax_income,
//                 TaxStatus::ContributePretaxTaxedWhenUsed
//             ),
//             3750_f64
//         );

//         assert_approx_eq!(
//             f64,
//             w4.value(
//                 withdrawal,
//                 inflation,
//                 dates,
//                 dates.year_out.unwrap().start,
//                 account_value,
//                 prev_account_value,
//                 col,
//                 prev_savings,
//                 tax_income,
//                 tax_status
//             ),
//             81.967,
//             epsilon = 0.001
//         );
//     }
// }
