//! College savings account (529)

//use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::super::{
    scatter_plot, Account, AccountResult, AccountType, AnalysisDates, SavingsTables, Table,
    YearRange, YearlyImpact, YearlyTotals,
};
use crate::inputs::{
    ContributionOptions, PercentInput, TaxStatus, WithdrawalOptions, YearEvalType, YearInput,
};
use crate::settings::Settings;

/// College savings accounts specifically designed to represent 529 accounts
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct College<T: std::cmp::Eq + std::hash::Hash + std::cmp::PartialEq + std::cmp::Ord> {
    name: String,
    table: Table<T>,
    contributions: Option<Table<T>>,
    earnings: Option<Table<T>>,
    withdrawals: Option<Table<T>>,
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
    analysis: SavingsTables,
    #[serde(skip)]
    dates: AnalysisDates,
}

impl From<College<String>> for College<u32> {
    fn from(other: College<String>) -> Self {
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
            notes: other.notes,
            analysis: other.analysis,
            dates: other.dates,
        }
    }
}

impl Account for College<u32> {
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
        let mut output = SavingsTables::new(
            &self.table,
            &self.contributions,
            &None,
            &self.earnings,
            &self.withdrawals,
        );

        years.iter().copied().for_each(|year| {
            output.value.0.entry(year).or_insert(0.0);
            output.contributions.0.entry(year).or_insert(0.0);
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
    // fn get_income(&self, _year: u32) -> Option<f64> {
    //     None
    // }
    // fn get_expense(&self, year: u32) -> Option<f64> {
    //     self.analysis
    //         .as_ref()
    //         .unwrap()
    //         .contributions
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
        scatter_plot(
            filepath,
            vec![
                ("Balance".into(), &self.analysis.value),
                ("Contributions".into(), &self.analysis.contributions),
                ("Earnings".into(), &self.analysis.earnings),
                ("Withdrawals".into(), &self.analysis.withdrawals),
            ],
            self.name(),
        );
    }
    fn simulate(
        &mut self,
        year: u32,
        totals: &YearlyTotals,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let start_in = self.dates.year_in.unwrap().start;
        //let end_out = self.dates.as_ref().unwrap().year_out.unwrap().end;
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
        }

        // Add contribution to contribution and value tables
        if let Some(x) = tables.contributions.0.get_mut(&year) {
            *x = result.contribution;
        }
        if let Some(x) = tables.value.0.get_mut(&year) {
            *x += result.contribution;
        }

        // Calculate withdrawal
        if self.dates.year_out.unwrap().contains(year) {
            result.withdrawal = self.withdrawal_type.value(
                self.withdrawal_value,
                settings.inflation_base,
                self.dates,
                year,
                tables.value.0[&year],
                tables.value.0[&(year - 1)],
                totals.get(year).col,
                totals.get(year - 1).saving,
                settings.tax_income,
                self.tax_status,
            );
        }

        // Dont allow an account to become overdrawn
        if result.withdrawal > tables.value.0[&year] {
            result.withdrawal = tables.value.0[&year];
        }

        // Add withdrawal to withdrawal table and subtract from value tables
        if let Some(x) = tables.withdrawals.0.get_mut(&year) {
            *x = result.withdrawal;
        }
        if let Some(x) = tables.value.0.get_mut(&year) {
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
    fn write(&self, filepath: String) {
        self.analysis.write(filepath);
    }
}
