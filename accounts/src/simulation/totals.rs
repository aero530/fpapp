//! Types used during the analysis / simulation

use log::error;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Write;
#[cfg(feature = "plotters-backend")]
use image::{ImageBuffer, Rgba};

use super::Table;
#[cfg(feature = "plotters-backend")]
use crate::plot::{scatter_plot_file, scatter_plot_buf};

/// How the results of the simulation of an account impact a YearlyTotal
///
/// Savings and HSA balances are not part of this delta: the runner recomputes
/// those totals each year as the sum of the relevant account balances, which
/// keeps them consistent with historical (pre-seeded) balance overrides.
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct YearlyImpact {
    /// Expenses get pulled out of net (dollars we already paid tax on)
    pub expense: f64,
    /// Healthcare costs that can be paid for with hsa dollars
    pub healthcare_expense: f64,
    /// Impact to cost of living (tracks to total of the 'expense' account type)
    pub col: f64,
    /// Taxable income (taxed at the income tax rate)
    pub income_taxable: f64,
    /// Earnings taxed as capital gains (taxed at the capital gains rate)
    pub capital_gains: f64,
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
    /// outstanding healthcare costs not yet paid (zeroed each year after HSA + net settlement)
    pub healthcare_expense: Table<u32>,
    /// gross healthcare costs for the year before HSA/net settlement — use this for reporting
    pub healthcare_expense_total: Table<u32>,
    /// cost of living
    pub col: Table<u32>,
    /// total value of all savings & retirement accounts (set by the runner from account balances each year)
    pub saving: Table<u32>,
    /// total value of all hsa accounts (set by the runner from account balances each year)
    pub hsa: Table<u32>,
    /// total taxable income for a year
    pub income_taxable: Table<u32>,
    /// total earnings taxed as capital gains for a year
    pub capital_gains: Table<u32>,
    /// total income for a year
    pub income: Table<u32>,
    /// amount of income tax paid for a year
    pub tax_burden: Table<u32>,
}

impl YearlyTotals {
    /// Initiate a new object with an empty hashmap
    pub fn new() -> YearlyTotals {
        YearlyTotals::default()
    }
    /// Initialize a new year and pull forward net when told to
    pub fn add_year(&mut self, year: u32, pull_value_forward: bool) -> Result<(), Box<dyn Error>> {
        match self.net.0.contains_key(&year) {
            true => Err(String::from("Year already exists.").into()),
            false => {
                self.net.insert(year, 0_f64);
                self.expense.insert(year, 0_f64);
                self.healthcare_expense.insert(year, 0_f64);
                self.healthcare_expense_total.insert(year, 0_f64);
                self.col.insert(year, 0_f64);
                self.saving.insert(year, 0_f64);
                self.hsa.insert(year, 0_f64);
                self.income_taxable.insert(year, 0_f64);
                self.capital_gains.insert(year, 0_f64);
                self.income.insert(year, 0_f64);
                self.tax_burden.insert(year, 0_f64);
                if pull_value_forward {
                    // net rolls over year to year (savings & hsa totals are
                    // recomputed from account balances by the runner instead)
                    self.net.pull_value_forward(year);
                }
                Ok(())
            }
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
                if update.healthcare_expense > 0.0 {
                    self.healthcare_expense_total
                        .update(year, update.healthcare_expense);
                }
                self.col.update(year, update.col);
                self.income_taxable.update(year, update.income_taxable);
                self.capital_gains.update(year, update.capital_gains);
                self.income.update(year, update.income);
            }
            false => {
                error!("Updating a year that does not exist.  Previous values not pulled forward");
                if self.add_year(year, false).is_ok() {
                    self.update(year, update);
                }
            }
        }
    }
    /// Record the total savings & retirement balance for the year
    pub fn set_saving(&mut self, year: u32, value: f64) {
        self.saving.insert(year, value);
    }
    /// Record the total hsa balance for the year
    pub fn set_hsa(&mut self, year: u32, value: f64) {
        self.hsa.insert(year, value);
    }
    /// Add income to net
    pub fn deposit_income_in_net(&mut self, year: u32) {
        self.net.update(year, self.income.get(year).unwrap_or_default());
    }
    /// Pay income and capital gains tax for the year
    ///
    /// Taxable totals are floored at zero: deductions (negative taxable income
    /// from pretax contributions) can offset other income for the year but a
    /// negative total does not produce a refund.
    pub fn pay_income_tax_from_net(&mut self, year: u32, tax_rate: f64, capital_gains_rate: f64) {
        let taxable_income = self.income_taxable.get(year).unwrap_or_default().max(0_f64);
        let taxable_gains = self.capital_gains.get(year).unwrap_or_default().max(0_f64);
        let tax_burden =
            taxable_income * (tax_rate / 100_f64) + taxable_gains * (capital_gains_rate / 100_f64);
        // log what income was after paying taxes
        self.tax_burden.insert(year, tax_burden);

        // take income tax payment out of net
        self.net.update(year, -tax_burden);
    }
    /// Pay for expenses for the year
    pub fn pay_expenses_from_net(&mut self, year: u32) {
        self.net
            .update(year, -self.expense.get(year).unwrap_or_default());
    }
    /// Remove healthcare expenses from net (these could also be covered by HSA accounts)
    pub fn pay_healthcare_expenses_from_net(&mut self, year: u32) {
        if self.healthcare_expense.get(year).unwrap_or_default() > 0_f64 {
            self.net
                .update(year, -self.healthcare_expense.get(year).unwrap_or_default());
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
    #[cfg(feature = "plotters-backend")]
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
    #[cfg(feature = "plotters-backend")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn net_rolls_forward_including_debt() {
        // Regression test for A1 at the totals level.
        let mut totals = YearlyTotals::new();
        totals.add_year(2020, true).unwrap();
        totals.net.insert(2020, -500.0);
        totals.add_year(2021, true).unwrap();
        assert_eq!(totals.net.get(2021), Some(-500.0));
    }

    #[test]
    fn negative_taxable_income_is_not_a_refund() {
        // Regression test for B5: deductions floor at zero total tax.
        let mut totals = YearlyTotals::new();
        totals.add_year(2020, true).unwrap();
        totals.update(
            2020,
            YearlyImpact {
                income_taxable: -5000.0,
                ..Default::default()
            },
        );
        totals.pay_income_tax_from_net(2020, 20.0, 10.0);
        assert_eq!(totals.tax_burden.get(2020), Some(0.0));
        assert_eq!(totals.net.get(2020), Some(0.0));
    }

    #[test]
    fn capital_gains_taxed_at_capital_gains_rate() {
        // Regression test for B4: capital gains use their own rate.
        let mut totals = YearlyTotals::new();
        totals.add_year(2020, true).unwrap();
        totals.update(
            2020,
            YearlyImpact {
                income_taxable: 1000.0,
                capital_gains: 2000.0,
                ..Default::default()
            },
        );
        totals.pay_income_tax_from_net(2020, 20.0, 10.0);
        // 1000 * 20% + 2000 * 10% = 400
        assert_eq!(totals.tax_burden.get(2020), Some(400.0));
    }
}
