//! College savings account (529)

use serde::{Deserialize, Serialize};
use std::error::Error;

use super::*;

/// College savings accounts specifically designed to represent 529 accounts
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct College<T: std::cmp::Ord> {
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
    /// Percent interest earned each year
    yearly_return: PercentInput,
    /// Determines how to interpret withdrawal_value
    withdrawal_type: WithdrawalOptions,
    /// How much money should be take out per year (either as a percentage or a fixed dollar amount)
    withdrawal_value: f64,
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
        linked_dates: Option<Dates>,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>> {
        if linked_dates.is_some() {
            return Err(String::from("Linked account dates provided but not used").into());
        }
        
        // Init the analysis object with values from the stored tables
        let mut analysis = SavingsTables::new(
            &self.table,
            &self.contributions,
            &None,
            &self.earnings,
            &self.withdrawals,
        );

        // Populate all the years needed in simulation
        years.iter().copied().for_each(|year| {
            analysis.value.0.entry(year).or_insert(0.0);
            analysis.contributions.0.entry(year).or_insert(0.0);
            analysis.earnings.0.entry(year).or_insert(0.0);
            analysis.withdrawals.0.entry(year).or_insert(0.0);
        });
        self.analysis = analysis;
        self.dates = Dates {
            year_in: self.get_range_in(settings, linked_dates),
            year_out: self.get_range_out(settings, linked_dates),
        };

        let mut initial_values = YearlyImpact::default();
        initial_values.saving = match self.analysis.value.get(years[0]) {
            Some(x) => x,
            None => 0_f64,
        };
        Ok(initial_values)
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
    fn get_value(&self, year: u32) -> Option<f64> {
        self.analysis.value.get(year)
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

        let mut result = WorkingValues::default();

        // Init value table with previous year's value
        tables.value.pull_value_forward(year);

        if tables.value.0[&year] < 0_f64 {
            return Err(String::from("College fund account value is negative.").into());
        }

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
                totals.get_col(year),
                totals.get_saving(year - 1),
                settings.tax_income,
                self.tax_status,
            );
            result.limit_withdrawal(tables.value.get(year).unwrap());
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
                healthcare_expense: 0_f64,
                col: 0_f64,
                saving: 0_f64,
                income_taxable: 0_f64,
                income: 0_f64,
                hsa: 0_f64,
            }),
            TaxStatus::ContributeTaxedEarningsTaxed => {
                return Err(String::from(
                    "This tax status type is not implemented for college accounts.",
                )
                .into())
            }
            TaxStatus::ContributePretaxTaxedWhenUsed => {
                return Err(String::from(
                    "This tax status type is not implemented for college accounts.",
                )
                .into())
            }
            TaxStatus::ContributePretaxUntaxedWhenUsed => {
                return Err(String::from(
                    "This tax status type is not implemented for college accounts.",
                )
                .into())
            }
        }
    }
    fn write(&self, filepath: String) {
        self.analysis.write(filepath);
    }
}
