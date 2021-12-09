//! Health savings account

use serde::{Deserialize, Serialize};
use std::error::Error;
use log::error;

use super::*;

/// Health Savings Account
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Hsa<T: std::cmp::Ord> {
    /// String describing this account
    name: String,
    /// Table of account balance
    table: Table<T>,
    /// Calendar year when money starts being added to this account
    start_in: YearInput,
    /// Calendar year when money is no longer added to this account (this value is inclusive)
    end_in: YearInput,
    /// Calendar year when money starts being withdrawn from this account
    start_out: YearInput,
    /// Calendar year when money stops being withdrawn from this account
    end_out: YearInput,
    /// Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage.
    yearly_contribution: f64,
    /// Determines how to interpret yearly_contribution
    contribution_type: ContributionOptions,
    /// Employer contributions to this account as a dollar amount
    employer_contribution: f64,
    /// Percent interest earned each year
    yearly_return: PercentInput,
    /// How cashflow in this account is treated for tax purposes
    tax_status: TaxStatus,
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
        linked_dates: Option<Dates>,
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
        self.analysis = output;
        self.dates = Dates {
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
    fn plot(&self, filepath: String) {
        scatter_plot(
            filepath,
            vec![("Balance".into(), &self.analysis.value)],
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

        let mut result = WorkingValues::default();

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
        self.analysis.write(filepath);
    }
}
