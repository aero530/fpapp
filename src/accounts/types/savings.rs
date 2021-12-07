//! Generic savings account
//!
// use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::super::{
    Account, AccountResult, AccountType, AnalysisDates, PullForward, SavingsTables, Table,
    YearRange, YearlyImpact, YearlyTotals, scatter_plot,
};
use crate::inputs::{
    ContributionOptions, PercentInput, TaxStatus, WithdrawalOptions, YearEvalType, YearInput,
};
use crate::settings::Settings;

/// Generic savings account
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Savings<T: std::cmp::Eq + std::hash::Hash + std::cmp::PartialEq + std::cmp::Ord> {
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
    analysis: Option<SavingsTables>,
    #[serde(skip)]
    dates: Option<AnalysisDates>,
}

impl From<Savings<String>> for Savings<u32> {
    fn from(other: Savings<String>) -> Self {
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

impl Account for Savings<u32> {
    fn type_id(&self) -> AccountType {
        AccountType::Savings
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
        self.analysis = Some(output);
        self.dates = Some(AnalysisDates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        });
        Ok(())
    }
    fn get_value(&self, year: u32) -> Option<f64> {
        self.analysis
            .as_ref()
            .unwrap()
            .value
            .0
            .get(&year)
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
    fn plot(&self, filepath: String) {
        scatter_plot(
            filepath, 
            vec![
                ("Balance".into(), &self.analysis.as_ref().unwrap().value),
                ("Contributions".into(), &self.analysis.as_ref().unwrap().contributions),
                ("Earnings".into(), &self.analysis.as_ref().unwrap().earnings),
                ("Withdrawals".into(), &self.analysis.as_ref().unwrap().withdrawals),
                ],
            self.name()
        );
    }
    fn simulate(
        &mut self,
        year: u32,
        totals: &YearlyTotals,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        let start_in = self.dates.as_ref().unwrap().year_in.unwrap().start;
        //let end_out = self.dates.as_ref().unwrap().year_out.unwrap().end;
        let tables = &mut self.analysis.as_mut().unwrap();

        let mut result = AccountResult::default();

        tables.pull_value_forward(year);

        // Calculate earnings
        result.earning = tables.value.0[&year] * (self.yearly_return.value(settings) / 100.0); // calculate earnings from interest

        // Add earnings to earnings table
        if let Some(x) = tables.earnings.0.get_mut(&year) {
            *x = result.earning;
        }
        // Increase account value by earnings
        if let Some(x) = tables.value.0.get_mut(&year) {
            *x += result.earning;
        }

        // Calculate contribution
        if self.dates.as_ref().unwrap().year_in.unwrap().contains(year) {
            result.contribution = self.contribution_type.value(
                self.yearly_contribution,
                totals.get(year).income,
                year - start_in,
                settings.inflation_base,
            );
        }

        // Add contribution to contribution table
        if let Some(x) = tables.contributions.0.get_mut(&year) {
            *x = result.contribution;
        }
        // Increase account value by contribution
        if let Some(x) = tables.value.0.get_mut(&year) {
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
                tables.value.0[&year],
                tables.value.0[&(year - 1)],
                totals.get(year - 1).col,
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

        Ok(YearlyImpact {
            expense: result.contribution,
            col: 0_f64,
            saving: result.contribution + result.earning - result.withdrawal, // delta to savings total for the year
            income_taxable: result.earning,
            income: result.withdrawal,
        })
    }
    fn write(&self, filepath: String) {
        match &self.analysis {
            Some(results) => results.write(filepath),
            None => {}
        }
    }
}
