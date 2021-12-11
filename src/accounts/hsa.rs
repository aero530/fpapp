//! Health savings account

use serde::{Deserialize, Serialize};
use std::error::Error;

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
    ) -> Result<Vec<(u32, YearlyImpact)>, Box<dyn Error>> {
        if linked_dates.is_some() {
            return Err(String::from("Linked account dates provided but not used").into());
        }
        let mut analysis = SavingsTables::new(
            &self.table,
            &Some(Table::default()),
            &Some(Table::default()),
            &Some(Table::default()),
            &Some(Table::default()),
        );
        years.iter().copied().for_each(|year| {
            analysis.value.0.entry(year).or_insert(0.0);
            analysis.contributions.0.insert(year, 0.0);
            analysis
                .employer_contributions
                .as_mut()
                .unwrap()
                .0
                .insert(year, 0.0);
            analysis.earnings.0.insert(year, 0.0);
            analysis.withdrawals.0.insert(year, 0.0);
        });
        self.analysis = analysis;
        self.dates = Dates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        };
        
        // let mut initial_values = YearlyImpact::default();
        // initial_values.hsa = match self.analysis.value.get(years[0]) {
        //     Some(x) => x,
        //     None => 0_f64,
        // };
        // Ok(initial_values)
        println!("table {:?}",self.table.0);
        let mut output = Vec::new();
        self.table.0.iter().for_each(|(year, value)| {
            let mut impact = YearlyImpact::default();
            impact.hsa = *value;
            output.push((*year, impact));
        });
        println!("impact {:?}",output);
        Ok(output)
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
    fn plot(&self, filepath: String) {
        scatter_plot(
            filepath,
            vec![
                ("Balance".into(), &self.analysis.value),
                ("Contributions".into(), &self.analysis.contributions),
                (
                    "Employer Contributions".into(),
                    &self.analysis.employer_contributions.as_ref().unwrap(),
                ),
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

        let mut result = WorkingValues::default();

        tables.value.pull_value_forward(year);

        if tables.value.0[&year] < 0_f64 {
            return Err(String::from("HSA account value is negative.").into());
        }

        // println!("bi {:.2}\t", tables.value.0[&year]);

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
                totals.get_income(year),
                year - start_in,
                settings.inflation_base,
            );
            result.employer_contribution = self.contribution_type.value(
                self.employer_contribution,
                totals.get_income(year),
                year - start_in,
                settings.inflation_base,
            );
        }

        // Add contribution to contribution and value tables
        if let Some(x) = tables.contributions.0.get_mut(&year) {
            *x = result.contribution + result.employer_contribution;
        }
        if let Some(x) = tables.value.0.get_mut(&year) {
            *x += result.contribution;
            *x += result.employer_contribution;
        }

        // print!("e {:.2}\t", result.earning);
        // print!("c {:.2}\t", result.contribution);
        // print!("ec {:.2}\t", result.employer_contribution);
        // print!("bf {:.2}\t", tables.value.0[&year]);

        // Calculate withdrawal based on outstanding healthcare expenses
        // This account is used to cover as much of the healthcare expenses
        // as it can based on current account value.  Any remaining expenses
        // must be taken from net in the main loop of the simulation.
        // Unpaid healthcare_expenses are positive values (>0)
        let healthcare_expense = totals.get_healthcare_expense(year);
        if healthcare_expense < 0_f64 {
            return Err(String::from("Negative healthcare expense.").into());
        }
        if self.dates.year_out.unwrap().contains(year) {
            result.withdrawal = match healthcare_expense < tables.value.get(year).unwrap() {
                true => healthcare_expense,
                false => tables.value.get(year).unwrap(),
            }
        }

        // Add withdrawal to withdrawal table and subtract from value tables
        if let Some(x) = tables.withdrawals.0.get_mut(&year) {
            *x = result.withdrawal;
        }
        if let Some(x) = tables.value.0.get_mut(&year) {
            *x -= result.withdrawal;
        }

        // print!("h {:.2}\t", healthcare_expense);
        // print!("w {:.2}\t", result.withdrawal);
        // println!("d {:.2}\t", result.contribution + result.employer_contribution + result.earning - result.withdrawal);

        Ok(YearlyImpact {
            expense: 0_f64,
            healthcare_expense: -result.withdrawal, // reduce this years healthcare expense by the amount paid for from this account
            col: 0_f64,
            saving: 0_f64,
            income_taxable: 0_f64,
            income: 0_f64,
            hsa: result.contribution + result.employer_contribution + result.earning
                - result.withdrawal,
        })
    }
    fn write(&self, filepath: String) {
        self.analysis.write(filepath);
    }
}
