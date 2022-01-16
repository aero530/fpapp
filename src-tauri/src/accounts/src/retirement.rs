//! Generic retirement account type applicable for 401K, Roth IRA, IRA, etc.

use log::trace;
use serde::{Deserialize, Serialize};
use std::error::Error;
use image::{ImageBuffer, Rgba};

use crate::inputs::fixed_with_inflation;
use account_savings_derive::AccountSavings;

use super::*;

/// Generic retirement account type applicable for 401K, Roth IRA, IRA, etc.
#[derive(Debug, Clone, Deserialize, Serialize, AccountSavings)]
#[serde(rename_all = "camelCase")]
pub struct Retirement<T: std::cmp::Ord> {
    /// String describing this account
    name: String,
    /// Table of account balance
    table: Table<T>,
    /// Table of contributions to this account
    contributions: Option<Table<T>>,
    /// Table of account earnings
    earnings: Option<Table<T>>,
    /// Table of withdrawals from this account
    withdrawals: Option<Table<T>>,
    /// Table of employer contributions to this account [in today's dollars]
    employer_contributions: Option<Table<T>>,
    /// Calendar year when money starts being added to this account
    start_in: YearInput,
    /// Calendar year when money is no longer added to this account (this value is inclusive and is often yearRetire-1)
    end_in: YearInput,
    /// Calendar year when money starts being withdrawn from this account
    start_out: YearInput,
    /// Calendar year when money stops being withdrawn from this account
    end_out: YearInput,
    /// Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage. [in today's dollars]
    contribution_value: f64,
    /// Determines how to interpret the value in yearly_contribution
    contribution_type: ContributionOptions,
    /// Percent interest earned each year
    yearly_return: PercentInput,
    /// Determines how to interpret the value in withdrawal_value
    withdrawal_type: WithdrawalOptions,
    /// How much money should be take out per year (either as a percentage or a fixed dollar amount) [in today's dollars]
    withdrawal_value: f64,
    /// How cashflow in this account is treated for tax purposes
    tax_status: TaxStatus,
    /// Link to income account used with employer contributions and some contribution types
    income_link: Option<String>,
    /// Percent of your contribution that your employer matches
    matching: Option<EmployerMatch>,
    /// General information to store with this account
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    /// Tables used to store simulation results
    #[serde(skip)]
    analysis: SavingsTables,
    /// Calculated date values as a year based on input values
    #[serde(skip)]
    dates: Dates,
}

impl From<Retirement<String>> for Retirement<u32> {
    fn from(other: Retirement<String>) -> Self {
        Self {
            name: other.name,
            table: other.table.into(),
            contributions: other.contributions.map(|v| v.into()),
            earnings: other.earnings.map(|v| v.into()),
            withdrawals: other.withdrawals.map(|v| v.into()),
            employer_contributions: other.employer_contributions.map(|v| v.into()),
            start_in: other.start_in,
            end_in: other.end_in,
            start_out: other.start_out,
            end_out: other.end_out,
            contribution_value: other.contribution_value,
            contribution_type: other.contribution_type,
            yearly_return: other.yearly_return,
            withdrawal_type: other.withdrawal_type,
            withdrawal_value: other.withdrawal_value,
            tax_status: other.tax_status,
            income_link: other.income_link,
            matching: other.matching,
            notes: other.notes,
            analysis: other.analysis,
            dates: other.dates,
        }
    }
}

impl Account for Retirement<u32> {
    fn type_id(&self) -> AccountType {
        AccountType::Retirement
    }
    fn link_id(&self) -> Option<String> {
        trace!("Link ID - {:?}", self.income_link);
        self.income_link.clone()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn init(
        &mut self,
        linked_dates: Option<Dates>,
        settings: &Settings,
    ) -> Result<Vec<(u32, YearlyImpact)>, Box<dyn Error>> {
        self.analysis = SavingsTables::new(
            &self.table,
            &self.contributions,
            &self.employer_contributions,
            &self.earnings,
            &self.withdrawals,
        );
        self.dates = Dates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        };

        Ok(self
            .table
            .0
            .iter()
            .map(|(year, value)| {
                (
                    *year,
                    YearlyImpact {
                        saving: *value,
                        ..Default::default()
                    },
                )
            })
            .collect())
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
    fn get_inputs(&self) -> String {
        String::from("Hello")
    }
    fn plot_to_file(&self, filepath: String, width: u32, height: u32) {
        scatter_plot_file(
            filepath,
            vec![
                ("Balance".into(), &self.analysis.value),
                ("Contributions".into(), &self.analysis.contributions),
                (
                    "Employer Contributions".into(),
                    &self.analysis.employer_contributions,
                ),
                ("Earnings".into(), &self.analysis.earnings),
                ("Withdrawals".into(), &self.analysis.withdrawals),
            ],
            self.name(),
            width,
            height,
        );
    }
    fn plot_to_buf(&self, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        scatter_plot_buf(
            vec![
                ("Balance".into(), &self.analysis.value),
                ("Contributions".into(), &self.analysis.contributions),
                (
                    "Employer Contributions".into(),
                    &self.analysis.employer_contributions,
                ),
                ("Earnings".into(), &self.analysis.earnings),
                ("Withdrawals".into(), &self.analysis.withdrawals),
            ],
            self.name(),
            width,
            height,
        )
    }
    fn get_plot_data(&self) -> Vec<PlotDataPoint> {
        self.analysis.get_plot_data()
    }
    fn simulate(
        &mut self,
        year: u32,
        totals: &YearlyTotals,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let mut result = WorkingValues::default();

        // Init value table with previous year's value
        self.analysis.add_year(year, true)?;

        if self.analysis.value.get(year).unwrap() < 0_f64 {
            return Err(String::from("Retirement account value is negative.").into());
        }

        // Calculate earnings
        result.earning =
            self.analysis.value.get(year).unwrap() * (self.yearly_return.value(settings) / 100.0); // calculate earnings from interest

        // Add earnings to earnings and value tables
        self.analysis.earnings.update(year, result.earning);
        self.analysis.value.update(year, result.earning);

        // Calculate contribution
        if self.dates.year_in.unwrap().contains(year) {
            result.contribution = self.get_contribution(year, totals, settings);

            match &self.matching {
                Some(employer_match) => {
                    if self.income_link.is_some() {
                        // somehow get the income value from income link
                    } else {
                        return Err(
                            String::from("Matching is set but there is no linked account").into(),
                        );
                    }

                    // if (account.matchLimit.length > 1) {
                    //     // and if it is a complex employer matching (more than one level)
                    //     if (contribution >= (account.matchLimit[0] / 100 + account.matchLimit[1] / 100) * accounts[account.incomeLink].table[yearCurrent]) {
                    //         // and if the contribution is above the highest employer matching level
                    //         employerMatch = accounts[account.incomeLink].table[yearCurrent] * ((account.employerMatch[1] / 100) * (account.matchLimit[1] / 100) + (account.employerMatch[0] / 100) * (account.matchLimit[0] / 100)); // calculate the employer matching based on the match limits
                    //     } else if (contribution >= (account.matchLimit[0] / 100) * accounts[account.incomeLink].table[yearCurrent]) {
                    //         // otherwise if the contribution is between the employer matching levels ) {
                    //         employerMatch = accounts[account.incomeLink].table[yearCurrent] * ((account.employerMatch[0] / 100) * (account.matchLimit[0] / 100) + (account.employerMatch[1] / 100) * (account.matchLimit[1] / 100) * (contribution / accounts[account.incomeLink].table[yearCurrent] - account.matchLimit[0] / 100)); // calculate the employer matching with all the first level and part of the second level
                    //     } else {
                    //         employerMatch = contribution * (account.employerMatch[0] / 100); // the employer contribution is computed based on the entire contribution
                    //     }
                    // } else {
                    //     // if it is a simple employer matching (only one level)
                    //     if (contribution >= account.matchLimit[0] * accounts[account.incomeLink].table[yearCurrent]) {
                    //         // and if the contribution is above the highest employer matching level
                    //         employerMatch = accounts[account.incomeLink].table[yearCurrent] * (account.employerMatch[0] / 100) * (account.matchLimit[0] / 100); // calculate the employer matching based on the match limits
                    //     } else {
                    //         // otherwise  if below the employer match limit
                    //         employerMatch = contribution * (account.employerMatch[0] / 100); // the employer contribution is computed based on the entire contribution
                    //     }
                    // }
                    let link_income = 500_f64;
                    result.employer_contribution = match result.contribution
                        >= employer_match.limit.value(settings) / 100_f64 * link_income
                    {
                        true => {
                            link_income
                                * (employer_match.amount.value(settings) / 100_f64)
                                * (employer_match.limit.value(settings) / 100_f64)
                        } // calculate the employer matching based on the match limits,
                        false => {
                            result.contribution * (employer_match.amount.value(settings) / 100_f64)
                        } // the employer contribution is computed based on the entire contribution,
                    };
                }
                None => {}
            }
        }

        // Add contribution to contribution and value tables
        self.analysis
            .contributions
            .update(year, result.contribution + result.employer_contribution);
        self.analysis
            .value
            .update(year, result.contribution + result.employer_contribution);

        // Calculate withdrawal
        if self.dates.year_out.unwrap().contains(year) {
            result.withdrawal = self.get_withdrawal(year, &totals, &settings);
            // result.limit_withdrawal(self.analysis.value.get(year).unwrap());
        }

        // Add withdrawal to withdrawal table and subtract from value tables
        self.analysis.withdrawals.update(year, result.withdrawal);
        self.analysis.value.update(year, -result.withdrawal);

        match self.tax_status {
            // Paid with taxed income, earnings are not taxed, withdrawals are not taxed
            //
            // Contributions count as an expense (will be subtracted from net for the year)
            // Contributions do not impact taxable income (as they are made with dollars that have already been taxed)
            // Withdrawals count as income but do not to taxable income
            TaxStatus::ContributeTaxedEarningsUntaxedWhenUsed => Ok(YearlyImpact {
                expense: result.contribution,
                healthcare_expense: 0_f64,
                col: 0_f64,
                saving: result.contribution + result.earning - result.withdrawal, // delta to savings total for the year
                income_taxable: 0_f64,
                income: result.withdrawal,
                hsa: 0_f64,
            }),
            // Paid with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed (tax free as long as used for intended purpose)
            //
            // Contributions count as an expense (will be subtracted from net for the year)
            // Contributions do not impact taxable income (as they are made with dollars that have already been taxed)
            // Withdrawals count as income but do not to taxable income
            TaxStatus::ContributeTaxedEarningsTaxed => Ok(YearlyImpact {
                expense: result.contribution,
                healthcare_expense: 0_f64,
                col: 0_f64,
                saving: result.contribution + result.earning - result.withdrawal, // delta to savings total for the year
                income_taxable: result.earning,
                // todo ! something different to account for earnings as cap gains
                income: result.withdrawal,
                hsa: 0_f64,
            }),
            // Paid with pretax income and taxed in year of use as income
            //
            // Contributions count as an expense (will be subtracted from net for the year)
            // Contributions reduce taxable income (they are a deduction)
            // Withdrawals count as income and add to taxable income
            TaxStatus::ContributePretaxTaxedWhenUsed => Ok(YearlyImpact {
                expense: result.contribution,
                healthcare_expense: 0_f64,
                col: 0_f64,
                saving: result.contribution + result.earning - result.withdrawal, // delta to savings total for the year
                income_taxable: result.withdrawal - result.contribution,
                income: result.withdrawal,
                hsa: 0_f64,
            }),
            // Paid with pretax income and not taxed as income (use with HSA)
            //
            // Contributions count as an expense (will be subtracted from net for the year)
            // Contributions reduce taxable income (they are a deduction)
            // Withdrawals count as income but do not add to taxable income
            TaxStatus::ContributePretaxUntaxedWhenUsed => Ok(YearlyImpact {
                expense: result.contribution,
                healthcare_expense: 0_f64,
                col: 0_f64,
                saving: result.contribution + result.earning - result.withdrawal, // delta to savings total for the year
                income_taxable: 0_f64 - result.contribution,
                income: result.withdrawal,
                hsa: 0_f64,
            }),
        }
    }
    fn write(&self, filepath: String) {
        self.analysis.write(filepath);
    }
}
