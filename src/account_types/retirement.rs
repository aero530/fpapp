//! Generic retirement account type applicable for 401K, Roth IRA, IRA, etc.
//!
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::inputs::{
    ContributionOptions, EmployerMatch, PercentInput, TaxStatus, WithdrawalOptions, YearEvalType, YearInput,
};
use crate::settings::Settings;
use super::{
    Account, AccountType, AnalysisDates, PullForward, SavingsTables, AccountResult, YearRange,
    YearlyTotal, YearlyImpact,
};

/// Generic retirement account type applicable for 401K, Roth IRA, IRA, etc.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Retirement {
    name: String,
    table: HashMap<String, f64>,
    contributions: Option<HashMap<String, f64>>,
    earnings: Option<HashMap<String, f64>>,
    withdrawals: Option<HashMap<String, f64>>,
    employer_contributions: Option<HashMap<String, f64>>,
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
    // employer_match: Option<f64>,
    // match_limit: Option<f64>,
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    #[serde(skip)]
    analysis: Option<SavingsTables>,
    #[serde(skip)]
    dates: Option<AnalysisDates>,
}

impl Account for Retirement {
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
        let mut output: SavingsTables = SavingsTables {
            value: self.table.clone(),
            contributions: match &self.contributions {
                Some(table) => table.clone(),
                None => HashMap::new(),
            },
            employer_contributions: Some(match &self.employer_contributions {
                Some(table) => table.clone(),
                None => HashMap::new(),
            }),
            earnings: match &self.earnings {
                Some(table) => table.clone(),
                None => HashMap::new(),
            },
            withdrawals: match &self.withdrawals {
                Some(table) => table.clone(),
                None => HashMap::new(),
            },
        };
        years.iter().for_each(|year| {
            output.value.entry(year.to_string()).or_insert(0.0);
            output.contributions.entry(year.to_string()).or_insert(0.0);
            output
                .employer_contributions
                .as_mut()
                .unwrap()
                .entry(year.to_string())
                .or_insert(0.0);
            output.earnings.entry(year.to_string()).or_insert(0.0);
            output.withdrawals.entry(year.to_string()).or_insert(0.0);
        });
        self.analysis = Some(output);
        self.dates = Some(AnalysisDates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        });
        Ok(())
    }
    fn get_value(&self, year: &String) -> Option<f64> {
        self.analysis.as_ref().unwrap().value.get(year).map(|v| *v)
    }
    fn get_income(&self, _year: &String) -> Option<f64> {
        None
    }
    fn get_expense(&self, year: &String) -> Option<f64> {
        self.analysis
            .as_ref()
            .unwrap()
            .contributions
            .get(year)
            .map(|v| *v)
    }
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
    fn simulate(
        &mut self,
        year: u32,
        totals: YearlyTotal,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let start_in = self.dates.unwrap().year_in.unwrap().start;
        //let end_out = self.dates.as_ref().unwrap().year_out.unwrap().end;
        let tables = self.analysis.as_mut().unwrap();

        let mut result = AccountResult::default();

        // Init value table with previous year's value
        tables.pull_value_forward(year);

        // Calculate earnings
        result.earning =
            tables.value[&year.to_string()] * (self.yearly_return.value(settings) / 100.0); // calculate earnings from interest

        // Add earnings to earnings and value tables
        if let Some(x) = tables.earnings.get_mut(&year.to_string()) {
            *x = result.earning;
        }
        if let Some(x) = tables.value.get_mut(&year.to_string()) {
            *x += result.earning;
        }

        // Calculate contribution
        if self.dates.as_ref().unwrap().year_in.unwrap().contains(year) {
            result.contribution = self.contribution_type.value(
                self.yearly_contribution,
                totals.income,
                year - start_in,
                settings.inflation_base,
            );

            match &self.matching {
                Some(employer_match) => {
                    if self.income_link.is_some() {
                        // somehow get the income value from income link
                    } else {
                        return Err(String::from("Matching is set but there is no linked account").into());
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
                    let emp_cont = match result.contribution >= employer_match.limit.value(settings) / 100_f64 * link_income {
                        true => link_income * (employer_match.amount.value(settings) / 100_f64) * (employer_match.limit.value(settings) / 100_f64), // calculate the employer matching based on the match limits,
                        false => result.contribution * (employer_match.amount.value(settings) / 100_f64) // the employer contribution is computed based on the entire contribution,
                    };
                    println!("{}",emp_cont);
                },
                None => {}
            }
            
        }

        // Add contribution to contribution and value tables
        if let Some(x) = tables.contributions.get_mut(&year.to_string()) {
            *x = result.contribution;
        }
        if let Some(x) = tables.value.get_mut(&year.to_string()) {
            *x += result.contribution;
        }

        //
        //
        //
        // ADD EMPLOYER CONTRIBUTION !!
        //
        //
        

        // Calculate withdrawal
        if self
            .dates
            .as_ref()
            .unwrap()
            .year_out
            .unwrap()
            .contains(year)
        {
            
            let col_scale = match settings.is_retired(year) {
                true => settings.retirement_cost_of_living / 100_f64,
                false => 1_f64,
            };

            result.withdrawal = self.withdrawal_type.value(
                self.withdrawal_value,
                settings.inflation_base,
                self.dates.unwrap(),
                year,
                tables.value[&year.to_string()],
                tables.value[&(year - 1).to_string()],
                totals.col * col_scale,
                totals.saving,
                settings.tax_income,
                self.tax_status,
            );
            debug!("{}", result.withdrawal);
        }

        // Dont allow an account to become overdrawn
        if result.withdrawal > tables.value[&year.to_string()] {
            result.withdrawal = tables.value[&year.to_string()];
        }

        // Add withdrawal to withdrawal table and subtract from value tables
        if let Some(x) = tables.withdrawals.get_mut(&year.to_string()) {
            *x = result.withdrawal;
        }
        if let Some(x) = tables.value.get_mut(&year.to_string()) {
            *x -= result.withdrawal;
        }

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
}
