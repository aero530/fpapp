//! Health savings account
//!
use log::error;
use serde::{Deserialize, Serialize};
use std::error::Error;
use plotters::prelude::*;

use super::{
    Account, AccountResult, AccountType, AnalysisDates, PullForward, SavingsTables, Table,
    YearRange, YearlyImpact, YearlyTotals, range
};
use crate::inputs::{ContributionOptions, PercentInput, TaxStatus, YearEvalType, YearInput};
use crate::settings::Settings;

/// Health Savings Account
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Hsa<T: std::cmp::Eq + std::hash::Hash + std::cmp::PartialEq + std::cmp::Ord> {
    name: String,
    table: Table<T>,
    start_in: YearInput,
    end_in: YearInput,
    start_out: YearInput,
    end_out: YearInput,
    yearly_contribution: f64,
    contribution_type: ContributionOptions,
    employer_contribution: f64,
    yearly_return: PercentInput,
    tax_status: TaxStatus,
    notes: Option<String>,
    // The following items are used when running the program and are not stored with the user data
    #[serde(skip)]
    analysis: Option<SavingsTables>,
    #[serde(skip)]
    dates: Option<AnalysisDates>,
}

impl From<Hsa<String>> for Hsa<u32> {
    fn from(other: Hsa<String>) -> Self {
        Self {
            name: other.name,
            table: other.table.into(),
            start_in: other.start_in,
            end_in: other.end_in,
            start_out: other.start_out,
            end_out: other.end_out,
            yearly_contribution: other.yearly_contribution,
            contribution_type: other.contribution_type,
            employer_contribution: other.employer_contribution,
            yearly_return: other.yearly_return,
            tax_status: other.tax_status,
            notes: other.notes,
            analysis: other.analysis,
            dates: other.dates,
        }
    }
}

impl Account for Hsa<u32> {
    fn type_id(&self) -> AccountType {
        AccountType::Hsa
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
            &Some(Table::default()),
            &Some(Table::default()),
            &Some(Table::default()),
            &Some(Table::default()),
        );
        // let mut output: SavingsTables = SavingsTables {
        //     value: self.table.clone(),
        //     contributions: HashMap::new(),
        //     employer_contributions: Some(HashMap::new()),
        //     earnings: HashMap::new(),
        //     withdrawals: HashMap::new(),
        // };
        years.iter().copied().for_each(|year| {
            output.value.0.entry(year).or_insert(0.0);
            output.contributions.0.insert(year, 0.0);
            output
                .employer_contributions
                .as_mut()
                .unwrap()
                .0
                .insert(year, 0.0);
            output.earnings.0.insert(year, 0.0);
            output.withdrawals.0.insert(year, 0.0);
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
    fn get_income(&self, year: u32) -> Option<f64> {
        self.analysis
            .as_ref()
            .unwrap()
            .withdrawals
            .0
            .get(&year)
            .map(|v| *v)
    }
    fn get_expense(&self, year: u32) -> Option<f64> {
        self.analysis
            .as_ref()
            .unwrap()
            .contributions
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
        let value = self.analysis.as_ref().unwrap().value.clone();
        let (x_min, x_max, y_min, y_max) = range(vec![&value]);

        let root = BitMapBackend::new(&filepath, (640, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .caption(self.name(), ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_min..x_max, y_min..y_max).unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(value.into_iter(),&RED)).unwrap()
            .label("balance")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw().unwrap();
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

        // Add earnings to earnings and value tables
        if let Some(x) = tables.earnings.0.get_mut(&year) {
            *x = result.earning;
        }
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

        //
        // add in
        // employer_contribution
        //
        // if (account.contributionType === 'fixed_with_inflation') {
        //     // if inflation needs to be accounted for in the contribution
        //     employerMatch = account.employerContribution * ((1 + inflationBase / 100) ** (yearCurrent - yearStart)); // increase the value by inflation
        // } else if (account.contributionType === 'fixed') {
        //     // otherwise if the contribution is a fixed value
        //     employerMatch = account.employerContribution; // set the contribution amount to the value input
        // } else {
        //     console.log('Employer Contribution type not implemented');
        //     errors.push({ title: `${account.name} ${yearCurrent}`, message: 'employer contribution type not implemented' });
        // }
        // account.employerContributionTable[yearCurrent] = employerMatch;
        //

        // Add contribution to contribution and value tables
        if let Some(x) = tables.contributions.0.get_mut(&year) {
            *x = result.contribution;
        }
        if let Some(x) = tables.value.0.get_mut(&year) {
            *x += result.contribution;
        }

        // Calculate withdrawal
        error!("Not done yet");

        Ok(YearlyImpact {
            expense: 0_f64,
            col: 0_f64,
            saving: 0_f64,
            income_taxable: 0_f64,
            income: 0_f64,
        })
    }
    fn write(&self, filepath: String) {
        match &self.analysis {
            Some(results) => results.write(filepath),
            None => {}
        }
    }
}
