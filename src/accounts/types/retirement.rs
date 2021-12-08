//! Generic retirement account type applicable for 401K, Roth IRA, IRA, etc.
//!
use log::trace;
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::super::{
    scatter_plot, Account, AccountResult, AccountType, AnalysisDates, SavingsTables, Table,
    YearRange, YearlyImpact, YearlyTotals,
};
use crate::inputs::{
    ContributionOptions, EmployerMatch, PercentInput, TaxStatus, WithdrawalOptions, YearEvalType,
    YearInput,
};
use crate::settings::Settings;

/// Generic retirement account type applicable for 401K, Roth IRA, IRA, etc.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Retirement<T: std::cmp::Eq + std::hash::Hash + std::cmp::PartialEq + std::cmp::Ord> {
    name: String,
    table: Table<T>,
    contributions: Option<Table<T>>,
    earnings: Option<Table<T>>,
    withdrawals: Option<Table<T>>,
    employer_contributions: Option<Table<T>>,
    start_in: YearInput,
    end_in: YearInput,
    start_out: YearInput,
    end_out: YearInput,
    yearly_contribution: f64,
    contribution_type: ContributionOptions,
    yearly_return: PercentInput,
    withdrawal_type: WithdrawalOptions,
    withdrawal_value: f64,
    tax_status: TaxStatus,
    income_link: Option<String>,
    matching: Option<EmployerMatch>,
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    #[serde(skip)]
    analysis: SavingsTables,
    #[serde(skip)]
    dates: AnalysisDates,
}

impl From<Retirement<String>> for Retirement<u32> {
    fn from(other: Retirement<String>) -> Self {
        Self {
            name: other.name,
            table: other.table.into(),
            contributions: match other.contributions {
                Some(v) => Some(v.into()),
                None => None,
            },
            earnings: match other.earnings {
                Some(v) => Some(v.into()),
                None => None,
            },
            withdrawals: match other.withdrawals {
                Some(v) => Some(v.into()),
                None => None,
            },
            employer_contributions: match other.employer_contributions {
                Some(v) => Some(v.into()),
                None => None,
            },
            start_in: other.start_in,
            end_in: other.end_in,
            start_out: other.start_out,
            end_out: other.end_out,
            yearly_contribution: other.yearly_contribution,
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
        years: &Vec<u32>,
        linked_dates: Option<AnalysisDates>,
        settings: &Settings,
    ) -> Result<(), Box<dyn Error>> {
        let mut output = SavingsTables::new(
            &self.table,
            &self.contributions,
            &self.employer_contributions,
            &self.earnings,
            &self.withdrawals,
        );
        years.iter().copied().for_each(|year| {
            output.value.0.entry(year).or_insert(0.0);
            output.contributions.0.entry(year).or_insert(0.0);
            match output.employer_contributions.as_mut() {
                Some(v) => {
                    v.0.entry(year).or_insert(0.0);
                }
                None => {}
            };
            output.earnings.0.entry(year).or_insert(0.0);
            output.withdrawals.0.entry(year).or_insert(0.0);
        });
        self.analysis = output;
        self.dates = AnalysisDates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        };
        Ok(())
    }
    // fn get_value(&self, year: u32) -> Option<f64> {
    //     self.analysis
    //         .as_ref()
    //         .unwrap()
    //         .value
    //         .0
    //         .get(&year)
    //         .map(|v| *v)
    // }
    fn get_range_in(
        &self,
        settings: &Settings,
        linked_dates: Option<AnalysisDates>,
    ) -> Option<YearRange> {
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
        settings: &Settings,
        linked_dates: Option<AnalysisDates>,
    ) -> Option<YearRange> {
        Some(YearRange {
            start: self
                .start_out
                .value(settings, linked_dates, YearEvalType::StartOut),
            end: self
                .end_out
                .value(settings, linked_dates, YearEvalType::EndOut),
        })
    }
    fn plot(&self, filepath: String) {
        let mut data = vec![
            ("Balance".into(), &self.analysis.value),
            ("Contributions".into(), &self.analysis.contributions),
            ("Earnings".into(), &self.analysis.earnings),
            ("Withdrawals".into(), &self.analysis.withdrawals),
        ];
        match self.analysis.employer_contributions.as_ref().is_some() {
            true => {
                data.insert(
                    2,
                    (
                        "Employer Contributions".into(),
                        &self.analysis.employer_contributions.as_ref().unwrap(),
                    ),
                );
            }
            false => {}
        };
        scatter_plot(filepath, data, self.name());
    }
    fn simulate(
        &mut self,
        year: u32,
        totals: &YearlyTotals,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let start_in = self.dates.year_in.unwrap().start;
        let tables = &mut self.analysis;

        let mut result = AccountResult::default();

        // Init value table with previous year's value
        tables.value.pull_value_forward(year);

        // Calculate earnings
        result.earning = tables.value.0[&year] * (self.yearly_return.value(settings) / 100.0); // calculate earnings from interest

        // Add earnings to earnings and value tables
        if let Some(x) = tables.earnings.0.get_mut(&year) {
            *x = result.earning;
        }
        if let Some(x) = tables.value.0.get_mut(&year) {
            *x += result.earning;
        }

        // Calculate contribution
        if self.dates.year_in.unwrap().contains(year) {
            result.contribution = self.contribution_type.value(
                self.yearly_contribution,
                totals.get(year).income,
                year - start_in,
                settings.inflation_base,
            );

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
                    result.employer_match = match result.contribution
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
                    //println!("{}",emp_cont);
                }
                None => {}
            }
        }

        // Add contribution to contribution and value tables
        if let Some(x) = tables.contributions.0.get_mut(&year) {
            *x = result.contribution;
            *x += result.employer_match;
        }
        if let Some(x) = tables.value.0.get_mut(&year) {
            *x += result.contribution;
            *x += result.employer_match; // ADD EMPLOYER CONTRIBUTION
        }

        // Calculate withdrawal
        if self.dates.year_out.unwrap().contains(year) {
            let col_scale = match settings.is_retired(year) {
                true => settings.retirement_cost_of_living / 100_f64,
                false => 1_f64,
            };

            result.withdrawal = self.withdrawal_type.value(
                self.withdrawal_value,
                settings.inflation_base,
                self.dates,
                year,
                tables.value.0[&year],
                tables.value.0[&(year - 1)],
                totals.get(year).col * col_scale,
                totals.get(year - 1).saving,
                settings.tax_income,
                self.tax_status,
            );
        }

        // Add withdrawal to withdrawal table and subtract from value tables
        if let Some(x) = tables.withdrawals.0.get_mut(&year) {
            *x = result.withdrawal;
        }
        if let Some(x) = tables.value.0.get_mut(&year) {
            *x -= result.withdrawal;
        }

        // debug!(
        //     "w{:?} c{:?} e{:?}",
        //     result.withdrawal, result.contribution, result.earning
        // );
        match self.tax_status {
            // Paid with taxed income, earnings are not taxed, withdrawals are not taxed
            //
            // Contributions count as an expense (will be subtracted from net for the year)
            // Contributions do not impact taxable income (as they are made with dollars that have already been taxed)
            // Withdrawals count as income but do not to taxable income
            TaxStatus::ContributeTaxedEarningsUntaxedWhenUsed => Ok(YearlyImpact {
                expense: result.contribution,
                col: 0_f64,
                saving: result.contribution + result.earning - result.withdrawal, // delta to savings total for the year
                income_taxable: 0_f64,
                income: result.withdrawal,
            }),
            // Paid with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed (tax free as long as used for intended purpose)
            //
            // Contributions count as an expense (will be subtracted from net for the year)
            // Contributions do not impact taxable income (as they are made with dollars that have already been taxed)
            // Withdrawals count as income but do not to taxable income
            TaxStatus::ContributeTaxedEarningsTaxed => Ok(YearlyImpact {
                expense: result.contribution,
                col: 0_f64,
                saving: result.contribution + result.earning - result.withdrawal, // delta to savings total for the year
                income_taxable: result.earning,
                // todo ! something different to account for earnings as cap gains
                income: result.withdrawal,
            }),
            // Paid with pretax income and taxed in year of use as income
            //
            // Contributions count as an expense (will be subtracted from net for the year)
            // Contributions reduce taxable income (they are a deduction)
            // Withdrawals count as income and add to taxable income
            TaxStatus::ContributePretaxTaxedWhenUsed => Ok(YearlyImpact {
                expense: result.contribution,
                col: 0_f64,
                saving: result.contribution + result.earning - result.withdrawal, // delta to savings total for the year
                income_taxable: result.withdrawal - result.contribution,
                income: result.withdrawal,
            }),
            // Paid with pretax income and not taxed as income (use with HSA)
            //
            // Contributions count as an expense (will be subtracted from net for the year)
            // Contributions reduce taxable income (they are a deduction)
            // Withdrawals count as income but do not add to taxable income
            TaxStatus::ContributePretaxUntaxedWhenUsed => Ok(YearlyImpact {
                expense: result.contribution,
                col: 0_f64,
                saving: result.contribution + result.earning - result.withdrawal, // delta to savings total for the year
                income_taxable: 0_f64 - result.contribution,
                income: result.withdrawal,
            }),
        }
    }
    fn write(&self, filepath: String) {
        self.analysis.write(filepath);
    }
}
