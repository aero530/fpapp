//! College savings account (529)
//!
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use super::inputs::{
    ContributionOptions, PercentInput, TaxStatus, WithdrawalOptions, YearEvalType, YearInput,
};
use super::{Account, AccountType, PullForward, AnalysisDates, SavingsTables, YearRange, SimResult, YearlyTotal};
use crate::settings::Settings;

/// College savings accounts specifically designed to represent 529 accounts
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct College {
    name: String,
    table: HashMap<String, f64>,
    contributions: Option<HashMap<String, f64>>,
    earnings: Option<HashMap<String, f64>>,
    withdrawals: Option<HashMap<String, f64>>,
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
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    #[serde(skip)]
    analysis: Option<SavingsTables>,
    #[serde(skip)]
    dates: Option<AnalysisDates>,
}

impl Account for College {
    fn type_id(&self) -> AccountType {
        AccountType::College
    }
    fn link_id(&self) -> Option<String> {
        None
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn init(
        &mut self,
        years: &Vec<u32>,
        dates: Option<AnalysisDates>,
        settings: &Settings,
    ) -> Result<(), Box<dyn Error>> {
        if dates.is_some() {
            return Err(String::from("Linked account dates provided but not used").into());
        }
        let mut output: SavingsTables = SavingsTables {
            value: self.table.clone(),
            contributions: match &self.contributions {
                Some(table) => table.clone(),
                None => HashMap::new(),
            },
            employer_contributions: None,
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
            output.earnings.entry(year.to_string()).or_insert(0.0);
            output.withdrawals.entry(year.to_string()).or_insert(0.0);
        });
        self.analysis = Some(output);
        self.dates = Some(AnalysisDates {
            year_in: self.get_range_in(settings),
            year_out: self.get_range_out(settings),
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
    fn get_range_in(&self, settings: &Settings) -> Option<YearRange> {
        Some(YearRange {
            start: self.start_in.value(settings, None, YearEvalType::StartIn),
            end: self.end_in.value(settings, None, YearEvalType::EndIn),
        })
    }
    fn get_range_out(&self, settings: &Settings) -> Option<YearRange> {
        Some(YearRange {
            start: self.start_out.value(settings, None, YearEvalType::StartOut),
            end: self.end_out.value(settings, None, YearEvalType::EndOut),
        })
    }
    fn simulate(&mut self, year: u32, totals: YearlyTotal, settings: &Settings) -> Result<SimResult, Box<dyn Error>> {
        let start_in = self.dates.as_ref().unwrap().year_in.unwrap().start;
        let end_out = self.dates.as_ref().unwrap().year_out.unwrap().end;
        let tables = &mut self.analysis.as_mut().unwrap();

        let mut result = SimResult::default();
        
        // Init value table with previous year's value
        // tables.value.entry(year.to_string()).or_insert(prev_value);

        // pull the most recent year data forward to the current year
        // if most_recent_year == year {
        //     account.table[yearCurrent] = account.table[mostRecentYear];
        // } else if Object.hasOwnProperty.call(account.table, yearCurrent - 1) {
        //     account.table[yearCurrent] = account.table[yearCurrent - 1];
        // } else if mostRecentYear < yearCurrent {
        //     account.table[yearCurrent] = account.table[mostRecentYear];
        // } else {
        //     account.table[yearCurrent] = 0;
        // }
        
        tables.pull_value_forward(year);

        // Calculate earnings
        result.earning = tables.value[&year.to_string()] * ( self.yearly_return.value(settings) / 100.0); // calculate earnings from interest

        // Add earnings to earnings and value tables
        if let Some(x) = tables.earnings.get_mut(&year.to_string()) {
            *x = result.earning;
        }
        if let Some(x) = tables.value.get_mut(&year.to_string()) {
            *x += result.earning;
        }

        // Calculate contribution
        if self.dates.as_ref().unwrap().year_in.unwrap().contains(year) {
            result.contribution = self.contribution_type.value(self.yearly_contribution, totals.income, year-start_in, settings.inflation_base);
        }

        // Add contribution to contribution and value tables
        if let Some(x) = tables.contributions.get_mut(&year.to_string()) {
            *x = result.contribution;
        }
        if let Some(x) = tables.value.get_mut(&year.to_string()) {
            *x += result.contribution;
        }

        // Calculate withdrawal
        if self.dates.as_ref().unwrap().year_out.unwrap().contains(year) {
            match self.withdrawal_type {
                WithdrawalOptions::Fixed => {
                    result.withdrawal = self.withdrawal_value;
                },
                WithdrawalOptions::FixedWithInflation => {
                    result.withdrawal = self.withdrawal_value * f64::powf(1_f64 + settings.inflation_base / 100_f64, (year - start_in) as f64);
                },
                WithdrawalOptions::EndAtZero => {
                    if year <= end_out {
                        // and if the year to stop taking money out of the account is beyond or equal to the current year
                        result.withdrawal = tables.value[&year.to_string()] / (end_out - year + 1) as f64; // calculate the fraction of the account balance to withdraw
                    }
                },
                WithdrawalOptions::ColFracOfSavings => {
                    // otherwise if type is cost of living fraction of total savings
                    // withdrawal = costOfLiving['table'][yearIndex] * account[accountIndex].table(yearIndex-1)./savingsTotal.table(yearIndex-1)
                    //
                    // 
                    // if (yearCurrent > yearStart) {
                    //     // withdrawal = costOfLiving['table'][yearIndex] * account[accountIndex].table(yearIndex-1)./savingsTotal.table(yearIndex-1)
                    //     // account for retirement cost of living and for capital gains in this line...its a hack and probably not very correct
                    //     if (account.table[yearCurrent - 1] > 0) {
                    //         // if there is money left in the account
                    //         // withdrawal from this account = total expenses this year  * fraction of total savings this account represents
                    //         // total expenses this year is reduced by the income during retirement for the year.
                    //         // incomeDuringRetirement is tracked because withdrawals from retirement accounts go into the income table but we want to
                    //         // pay for expenses from money earned in this year before pulling from retirement accounts.
                    //         const totalExpensesThisYear = Object.values(expenseTotal[yearCurrent]).reduce((acc, cur) => acc + cur, 0) - incomeDuringRetirement[yearCurrent];
                    //         withdrawal = (totalExpensesThisYear * account.table[yearCurrent - 1]) / savingsTotalTable[yearCurrent - 1];
                    //         if (Object.prototype.hasOwnProperty.call(account, 'taxStatus') && account.taxStatus === 3) {
                    //         withdrawal *= (taxIncome / 100 + 1); // add extra to amount withdrawal value to account for taxes.
                    //         }
                    //     }
                    //  } else {
                    //     console.log('ERROR - Can not compute withdrawal amount');
                    //     errors.push({ title: `${account.name} ${yearCurrent}`, message: 'can not compute withdrawal amount < 0' });
                    //  }
                    todo!()
                },
            }
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

        Ok(result)
    }
}
