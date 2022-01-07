//! Types used during the analysis / simulation

use log::error;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Write;
use image::{ImageBuffer, Rgba};

use super::Table;
use crate::plot::{scatter_plot_file, scatter_plot_buf};

/// How the results of the simulation of an account impact a YearlyTotal
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct YearlyImpact {
    /// Expenses get pulled out of net (dollars we already paid tax on)
    pub expense: f64,
    /// Healthcare costs that can be paid for with hsa dollars
    pub healthcare_expense: f64,
    /// Impact to cost of living (tracks to total of the 'expense' account type)
    pub col: f64,
    /// Change in total savings accounts dollars
    pub saving: f64,
    /// Change in total hsa dollars
    pub hsa: f64,
    /// Taxable income
    pub income_taxable: f64,
    /// Total income (taxable + non-taxable)
    pub income: f64,
}

/// Set of YearlyTotal tracked over multiple years
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct YearlyTotals {
    /// Overall cash account that all money flows in and out of (the value of this account rolls over from year to year)
    pub net: Table<u32>,
    /// total expenses for a year
    pub expense: Table<u32>,
    /// total healthcare costs that have not been paid (these are paid for with HSA dollars first)
    pub healthcare_expense: Table<u32>,
    /// cost of living
    pub col: Table<u32>,
    /// total value of all savings accounts (the value of this account rolls over from year to year)
    pub saving: Table<u32>,
    /// total value of all hsa accounts (the value of this account rolls over from year to year)
    pub hsa: Table<u32>,
    /// total taxable income for a year
    pub income_taxable: Table<u32>,
    /// total income for a year
    pub income: Table<u32>,
    /// amount of income tax paid for a year
    pub tax_burden: Table<u32>,
    /// currently unused
    pub income_during_retirement: Table<u32>,
}

impl YearlyTotals {
    /// Initiate a new object with an empty hashmap
    pub fn new() -> YearlyTotals {
        YearlyTotals::default()
    }
    /// Initialize a new year and pull forward net, savings, and hsa when told to
    pub fn add_year(&mut self, year: u32, pull_value_forward: bool) -> Result<(), Box<dyn Error>> {
        // need to only update if there is not a value in this year yet.  in year 1 i init somewhere else

        match self.net.0.contains_key(&year) {
            true => Err(String::from("Year already exists.").into()),
            false => {
                self.net.insert(year, 0_f64);
                self.expense.insert(year, 0_f64);
                self.healthcare_expense.insert(year, 0_f64);
                self.col.insert(year, 0_f64);
                self.saving.insert(year, 0_f64);
                self.hsa.insert(year, 0_f64);
                self.income_taxable.insert(year, 0_f64);
                self.income.insert(year, 0_f64);
                self.tax_burden.insert(year, 0_f64);
                self.income_during_retirement.insert(year, 0_f64);
                if pull_value_forward {
                    self.pull_value_forward(year);
                }
                Ok(())
            }
        }
    }
    /// If there is a prev year then pull forward that value
    fn pull_value_forward(&mut self, year: u32) {
        if self.net.0.contains_key(&year) {
            self.net.pull_value_forward(year);
            self.saving.pull_value_forward(year);
            self.hsa.pull_value_forward(year);
        } else {
            error!("Year must be added to YearlyTotals before pulling previous values forward.");
        }
    }
    /// Update the data for a specified year
    ///
    /// Check if self get_years contains year.  If so then update that year.  If not create it then update it.
    pub fn update(&mut self, year: u32, update: YearlyImpact) {
        match self.net.0.contains_key(&year) {
            true => {
                self.expense.update(year, update.expense);
                self.healthcare_expense
                    .update(year, update.healthcare_expense);
                self.col.update(year, update.col);
                self.saving.update(year, update.saving);
                self.hsa.update(year, update.hsa);
                self.income_taxable.update(year, update.income_taxable);
                self.income.update(year, update.income);
            }
            false => {
                error!("Updating a year that does not exist.  Previous values not pulled forward");
                self.add_year(year, false).unwrap();
                self.update(year, update);
            }
        }
    }
    /// Add income to net
    pub fn deposit_income_in_net(&mut self, year: u32) {
        //self.net += self.income;
        self.net.update(year, self.income.get(year).unwrap());
    }
    /// Pay income tax for the year
    pub fn pay_income_tax_from_net(&mut self, year: u32, tax_rate: f64) {
        let tax_burden = self.income_taxable.get(year).unwrap() * (tax_rate / 100_f64);
        // log what income was after paying taxes
        self.tax_burden.insert(year, tax_burden);

        // take income tax payment out of net
        self.net.update(year, -1_f64 * tax_burden);
    }
    /// Pay for expenses for the year
    pub fn pay_expenses_from_net(&mut self, year: u32) {
        self.net
            .update(year, -1_f64 * self.expense.get(year).unwrap());
    }
    /// Remove healthcare expenses from net (these could also be covered by HSA accounts)
    pub fn pay_healthcare_expenses_from_net(&mut self, year: u32) {
        if self.healthcare_expense.get(year).unwrap() > 0_f64 {
            self.net
                .update(year, -1_f64 * self.healthcare_expense.get(year).unwrap());
            self.healthcare_expense.insert(year, 0_f64);
        }
    }
    /// Write yearly total data to a csv file
    pub fn write_summary(&self, filename: String) {
        let mut file = std::fs::File::create(filename).unwrap();
        file.write_all("year, totals.net, totals.saving, totals.hsa, totals.healthcare_expense, totals.expense, totals.col, totals.income, totals.income_taxable, totals.tax_burden\n".as_bytes()).unwrap();

        self.years().iter().for_each(|year| {
            file.write_all(
                format!(
                    "{},\t{:.2},\t{:.2},\t{:.2},\t{:.2},\t{:.2},\t{:.2},\t{:.2},\t{:.2},\t{:.2}\n",
                    year,
                    self.net.get(*year).unwrap_or_default(),
                    self.saving.get(*year).unwrap_or_default(),
                    self.hsa.get(*year).unwrap_or_default(),
                    self.healthcare_expense.get(*year).unwrap_or_default(),
                    self.expense.get(*year).unwrap_or_default(),
                    self.col.get(*year).unwrap_or_default(),
                    self.income.get(*year).unwrap_or_default(),
                    self.income_taxable.get(*year).unwrap_or_default(),
                    self.tax_burden.get(*year).unwrap_or_default()
                )
                .as_bytes(),
            )
            .unwrap();
        });
    }
    /// Generate plot
    pub fn plot_to_file(&self, filepath: String) {
        let net: Vec<f64> = self.net.values();
        let saving: Vec<f64> = self.saving.values();
        let hsa: Vec<f64> = self.hsa.values();
        let healthcare_expense: Vec<f64> = self.healthcare_expense.values();
        let expense: Vec<f64> = self.expense.values();
        let col: Vec<f64> = self.col.values();
        let income: Vec<f64> = self.income.values();
        let income_taxable: Vec<f64> = self.income_taxable.values();
        let tax_burden: Vec<f64> = self.tax_burden.values();

        scatter_plot_file(
            filepath,
            vec![
                ("Net".into(), &(self.years(), net).into()),
                ("Saving".into(), &(self.years(), saving).into()),
                ("HSA".into(), &(self.years(), hsa).into()),
                (
                    "Healthcare Expense".into(),
                    &(self.years(), healthcare_expense).into(),
                ),
                ("Expense".into(), &(self.years(), expense).into()),
                ("COL".into(), &(self.years(), col).into()),
                ("Income".into(), &(self.years(), income).into()),
                (
                    "Taxable Income".into(),
                    &(self.years(), income_taxable).into(),
                ),
                ("Tax Burden".into(), &(self.years(), tax_burden).into()),
            ],
            "Summary".into(),
            1600,
            1200,
        );
    }
    /// Plot the account and return it as a vec
    pub fn plot_to_buf(&self, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let net: Vec<f64> = self.net.values();
        let saving: Vec<f64> = self.saving.values();
        let hsa: Vec<f64> = self.hsa.values();
        let healthcare_expense: Vec<f64> = self.healthcare_expense.values();
        let expense: Vec<f64> = self.expense.values();
        let col: Vec<f64> = self.col.values();
        let income: Vec<f64> = self.income.values();
        let income_taxable: Vec<f64> = self.income_taxable.values();
        let tax_burden: Vec<f64> = self.tax_burden.values();
        scatter_plot_buf(
            vec![
                ("Net".into(), &(self.years(), net).into()),
                ("Saving".into(), &(self.years(), saving).into()),
                ("HSA".into(), &(self.years(), hsa).into()),
                (
                    "Healthcare Expense".into(),
                    &(self.years(), healthcare_expense).into(),
                ),
                ("Expense".into(), &(self.years(), expense).into()),
                ("COL".into(), &(self.years(), col).into()),
                ("Income".into(), &(self.years(), income).into()),
                (
                    "Taxable Income".into(),
                    &(self.years(), income_taxable).into(),
                ),
                ("Tax Burden".into(), &(self.years(), tax_burden).into()),
            ],
            "Summary".into(),
            width,
            height,
        )
    }
    

    /// Get the cost of living for the specified year
    ///
    /// If the year is not found then zero is returned
    pub fn get_col(&self, year: u32) -> f64 {
        self.col.get(year).unwrap_or_default()
    }
    /// Get the income for the specified year
    ///
    /// If the year is not found then zero is returned
    pub fn get_income(&self, year: u32) -> f64 {
        self.income.get(year).unwrap_or_default()
    }
    /// Get the savings total for the specified year
    ///
    /// If the year is not found then zero is returned
    pub fn get_saving(&self, year: u32) -> f64 {
        self.saving.get(year).unwrap_or_default()
    }
    /// Get the healthcare_expense for the specified year
    ///
    /// If the year is not found then zero is returned
    pub fn get_healthcare_expense(&self, year: u32) -> f64 {
        self.healthcare_expense.get(year).unwrap_or_default()
    }
    /// Return a sorted list of keys (years)
    ///
    /// There should not be a way for the elements of self to contain
    /// different key sets so we just pull the keys from net.
    pub fn years(&self) -> Vec<u32> {
        self.net.years()
    }
    /// Check if this year already exists
    pub fn contains_year(&self, year: u32) -> bool {
        self.net.0.contains_key(&year)
    }
}
