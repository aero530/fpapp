//! College savings account (529)

//use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::inputs::{ContributionOptions, PercentInput, TaxStatus, WithdrawalOptions, YearEvalType, YearInput};
use crate::settings::Settings;
use super::{Account, AccountType, AnalysisDates, PullForward, SavingsTables, AccountResult, YearRange,YearlyTotal, YearlyImpact};

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
        linked_dates: Option<AnalysisDates>,
        settings: &Settings,
    ) -> Result<(), Box<dyn Error>> {
        if linked_dates.is_some() {
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
        let start_in = self.dates.as_ref().unwrap().year_in.unwrap().start;
        //let end_out = self.dates.as_ref().unwrap().year_out.unwrap().end;
        let tables = self.analysis.as_mut().unwrap();

        let mut result = AccountResult::default();

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

        // match tables.pull_value_forward(year) {
        //     Ok(_) => {},
        //     Err(e) => return Err(e),
        // }

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
        }

        // Add contribution to contribution and value tables
        if let Some(x) = tables.contributions.get_mut(&year.to_string()) {
            *x = result.contribution;
        }
        if let Some(x) = tables.value.get_mut(&year.to_string()) {
            *x += result.contribution;
        }

        // Calculate withdrawal
        if self
            .dates
            .as_ref()
            .unwrap()
            .year_out
            .unwrap()
            .contains(year)
        {
            result.withdrawal = self.withdrawal_type.value(
                self.withdrawal_value,
                settings.inflation_base,
                self.dates.unwrap(),
                year,
                tables.value[&year.to_string()],
                tables.value[&(year - 1).to_string()],
                totals.col,
                totals.saving,
                settings.tax_income,
                self.tax_status,
            );
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
            // contribute taxed income
            // payed with taxed income, earnings are not taxed, withdrawals are not taxed
            TaxStatus::ContributeTaxedEarningsUntaxedWhenUsed => Ok(YearlyImpact {
                expense: result.contribution,
                col: 0_f64,
                saving: 0_f64,
                income_taxable: 0_f64,
                income: 0_f64,
            }),
            TaxStatus::ContributeTaxedEarningsTaxed => todo!(),
            TaxStatus::ContributePretaxTaxedWhenUsed => todo!(),
            TaxStatus::ContributePretaxUntaxedWhenUsed => todo!(),
        }

    }
}
