//! Types of financial accounts
//!
//! Simulate accounts such as income, expense, retirement, 529, loan, mortgage, etc.

//use log::error;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use std::error::Error;
use std::io::Write;
use plotters::prelude::*;

use crate::analysis_types::{AccountResult, AnalysisDates, YearRange, YearlyImpact, YearlyTotals};
use crate::settings::Settings;

mod expense;
use expense::Expense;

mod income;
use income::Income;

mod ssa;
use ssa::Ssa;

mod college;
use college::College;

mod hsa;
use hsa::Hsa;

mod retirement;
use retirement::Retirement;

mod savings;
use savings::Savings;

mod loan;
use loan::Loan;

mod mortgage;
use mortgage::Mortgage;



pub const COLORS : [RGBColor; 9] = [
    RGBColor(24, 171, 221),
    RGBColor(176, 75, 207),
    RGBColor(29, 229, 188),
    RGBColor(234, 115, 105),
    RGBColor(220, 75, 179),
    RGBColor(223, 84, 44),
    RGBColor(234, 189, 60),
    RGBColor(110, 240, 210),
    RGBColor(239, 166, 143),
    ];


pub fn range(input: Vec<&Table<u32>>) -> (u32, u32, f64, f64) {
    let x_min = *input
        .iter()
        .map(|table| {
            *table
                .0
                .keys()
                .copied()
                .collect::<Vec<u32>>()
                .iter()
                .min()
                .unwrap()
        })
        .collect::<Vec<u32>>()
        .iter()
        .min()
        .unwrap();
    let x_max = *input
        .iter()
        .map(|table| {
            *table
                .0
                .keys()
                .copied()
                .collect::<Vec<u32>>()
                .iter()
                .max()
                .unwrap()
        })
        .collect::<Vec<u32>>()
        .iter()
        .max()
        .unwrap();
    let y_min = input
        .iter()
        .map(|table| {
            table
                .0
                .values()
                .copied()
                .collect::<Vec<f64>>()
                .iter()
                .fold(0.0/0.0, |m, v| v.min(m))
        })
        .collect::<Vec<f64>>()
        .iter()
        .fold(0.0/0.0, |m, v| v.min(m));
    let y_max = input
        .iter()
        .map(|table| {
            table
                .0
                .values()
                .copied()
                .collect::<Vec<f64>>()
                .iter()
                .fold(0.0/0.0, |m, v| v.max(m))
        })
        .collect::<Vec<f64>>()
        .iter()
        .fold(0.0/0.0, |m, v| v.max(m));
    (x_min, x_max, y_min, y_max)
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Table<T: std::cmp::Eq + std::hash::Hash + std::cmp::PartialEq + std::cmp::Ord>(
    BTreeMap<T, f64>,
);

impl Table<u32> {
    pub fn get(&self, year: u32) -> Option<f64> {
        match self.0.get(&year) {
            Some(v) => Some(*v),
            None => None,
        }
    }
    pub fn to_vec(&self) -> (Vec<u32>, Vec<f64>) {
        let years: Vec<u32> = self.0.keys().copied().collect();
        let values: Vec<f64> = self.0.values().copied().collect();
        (years, values)
    }
    // pub fn plotter(&self) -> core::slice::Iter<(u32, f64)> {
    //     self.0.keys().copied().map(|year| {
    //         (year, *self.0.get(&year).unwrap())
    //     })
    // }
}

// and we'll implement IntoIterator
impl IntoIterator for Table<u32> {
    type Item = (u32,f64);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        //self.0.into_iter()
        //self.0.keys().copied().map(|year| {
        //    (year, *self.0.get(&year).unwrap())
        //}).collect::<Vec<u32, f64>>().into_iter()
        self.0.keys().zip(self.0.values()).map(|(x,y)| (*x,*y)).collect::<Vec<(u32, f64)>>().into_iter()
    }
}

impl From<Table<String>> for Table<u32> {
    fn from(other: Table<String>) -> Self {
        Self(
            other
                .0
                .iter()
                .map(|(k, v)| (k.parse::<u32>().unwrap(), *v))
                .collect(),
        )
    }
}

/// A single table of account values
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SingleTable {
    value: Table<u32>,
}

impl SingleTable {
    fn new(value: &Table<u32>) -> SingleTable {
        SingleTable {
            value: value.clone(),
        }
    }
    pub fn write(&self, filename: String) {
        let years: Vec<u32> = self.value.0.keys().copied().collect();

        let mut file = std::fs::File::create(filename).unwrap();
        file.write_all("year, value\n".as_bytes()).unwrap();

        years.iter().for_each(|year| {
            file.write_all(
                format!("{}, {:.2}\n", year, self.value.get(*year).unwrap_or(0_f64),).as_bytes(),
            )
            .unwrap();
        });
    }
}

pub trait PullForward: std::fmt::Debug {
    /// Return the most recent year in the value table that has a value greater than zero
    fn most_recent_populated_year(&self) -> Option<u32>;

    /// If the most recent non-zero value is in year-1 then set the value table entry for year to the value tables entry from year - 1
    fn pull_value_forward(&mut self, year: u32);
}

/// A set of tables for use with loans and mortgage accounts
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoanTables {
    /// Outstanding loan amount
    value: Table<u32>,
    /// Interest accrued this year
    interest: Table<u32>,
    /// Payments made against the loan
    payments: Table<u32>,
    /// Escrow amount used for mortgage type loans
    escrow: Option<Table<u32>>,
    /// PMI used for mortgage type loans
    insurance: Option<Table<u32>>,
}

impl PullForward for LoanTables {
    fn most_recent_populated_year(&self) -> Option<u32> {
        self.value
            .0
            .iter()
            .filter(|(_k, v)| **v > f64::EPSILON) // only take years that have a value associated with them
            .map(|(k, _v)| *k) // pull just the year (we don't need the value anymore)
            .collect::<Vec<u32>>() // put into an
            .iter()
            .copied()
            .max()
    }
    /// if there was a value in this account last year then pull it forward
    fn pull_value_forward(&mut self, year: u32) {
        match self.most_recent_populated_year() {
            Some(recent_year) => {
                if recent_year == year - 1 {
                    *(self.value.0).get_mut(&year).unwrap() = self.value.0[&(year - 1)];
                }
            }
            None => {}
        }
    }
}
impl LoanTables {
    fn new(
        value: &Table<u32>,
        interest: &Table<u32>,
        payments: &Table<u32>,
        escrow: &Option<Table<u32>>,
        insurance: &Option<Table<u32>>,
    ) -> LoanTables {
        LoanTables {
            // These keys must always have tables
            value: value.clone(),
            interest: interest.clone(),
            payments: payments.clone(),
            // These keys will only have tables if mortgage type
            escrow: escrow.clone(),
            insurance: insurance.clone(),
        }
    }

    pub fn write(&self, filename: String) {
        let years: Vec<u32> = self.value.0.keys().copied().collect();

        let mut file = std::fs::File::create(filename).unwrap();
        file.write_all("year, value, interest, payments, escrow, insurance\n".as_bytes())
            .unwrap();

        years.iter().for_each(|year| {
            file.write_all(
                format!(
                    "{}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}\n",
                    year,
                    self.value.get(*year).unwrap_or(0_f64),
                    self.interest.get(*year).unwrap_or(0_f64),
                    self.payments.get(*year).unwrap_or(0_f64),
                    self.escrow
                        .as_ref()
                        .unwrap_or(&Table::default())
                        .get(*year)
                        .unwrap_or(0_f64),
                    self.insurance
                        .as_ref()
                        .unwrap_or(&Table::default())
                        .get(*year)
                        .unwrap_or(0_f64),
                )
                .as_bytes(),
            )
            .unwrap();
        });
    }
}

/// A set of tables for use with savings types of accounts
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SavingsTables {
    value: Table<u32>,
    contributions: Table<u32>,
    employer_contributions: Option<Table<u32>>,
    earnings: Table<u32>,
    withdrawals: Table<u32>,
}

impl PullForward for SavingsTables {
    fn most_recent_populated_year(&self) -> Option<u32> {
        self.value
            .0
            .iter()
            .filter(|(_k, v)| **v > f64::EPSILON)
            .map(|(k, _v)| *k)
            .collect::<Vec<u32>>()
            .iter()
            .copied()
            .max()
    }
    /// if there was a value in this account last year then pull it forward
    fn pull_value_forward(&mut self, year: u32) {
        match self.most_recent_populated_year() {
            Some(recent_year) => {
                if recent_year == year - 1 {
                    *(self.value.0).get_mut(&year).unwrap() = self.value.0[&(year - 1)];
                }
            }
            None => {}
        }
    }
}
impl SavingsTables {
    fn new(
        value: &Table<u32>,
        contributions: &Option<Table<u32>>,
        employer_contributions: &Option<Table<u32>>,
        earnings: &Option<Table<u32>>,
        withdrawals: &Option<Table<u32>>,
    ) -> SavingsTables {
        SavingsTables {
            value: value.clone(),
            contributions: match contributions {
                Some(table) => table.clone(),
                None => Table::default(),
            },
            employer_contributions: employer_contributions.clone(),
            earnings: match earnings {
                Some(table) => table.clone(),
                None => Table::default(),
            },
            withdrawals: match withdrawals {
                Some(table) => table.clone(),
                None => Table::default(),
            },
        }
    }

    pub fn write(&self, filename: String) {
        let mut years: Vec<u32> = self.value.0.keys().map(|k| *k).collect();
        years.sort();

        let mut file = std::fs::File::create(filename).unwrap();
        file.write_all(
            "year, value, contributions, employer_contributions, earnings, withdrawals\n"
                .as_bytes(),
        )
        .unwrap();

        years.iter().for_each(|year| {
            file.write_all(
                format!(
                    "{}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}\n",
                    year,
                    self.value.get(*year).unwrap_or(0_f64),
                    self.contributions.get(*year).unwrap_or(0_f64),
                    self.employer_contributions
                        .as_ref()
                        .unwrap_or(&Table::default())
                        .get(*year)
                        .unwrap_or(0_f64),
                    self.earnings.get(*year).unwrap_or(0_f64),
                    self.withdrawals.get(*year).unwrap_or(0_f64),
                )
                .as_bytes(),
            )
            .unwrap();
        });
    }
}

/// Trait used to define what each account type must be able to provide
pub trait Account: std::fmt::Debug {
    /// Return the type of the account
    fn type_id(&self) -> AccountType;

    /// Return the name of the account
    fn name(&self) -> String;

    /// Return link id if the account is linked to another account
    fn link_id(&self) -> Option<String>;

    /// Initialize analysis tables with a value for every year in years.  Fill with
    /// values from user data file first then backfill with 0 for years that do not
    /// have a value in user data.  Also initializes the dates used for analysis.
    fn init(
        &mut self,
        years: &Vec<u32>,
        linked_dates: Option<AnalysisDates>,
        settings: &Settings,
    ) -> Result<(), Box<dyn Error>>;

    /// Return the value for the specified year
    fn get_value(&self, year: u32) -> Option<f64>;

    // /// Return the income value for the specified year
    // fn get_income(&self, year: u32) -> Option<f64>;

    // /// Return the expense value for the specified year
    // fn get_expense(&self, year: u32) -> Option<f64>;

    /// Return start_in and end_in
    fn get_range_in(
        &self,
        settings: &Settings,
        linked_dates: Option<AnalysisDates>,
    ) -> Option<YearRange>;

    /// Return start_out and end_out
    fn get_range_out(
        &self,
        settings: &Settings,
        linked_dates: Option<AnalysisDates>,
    ) -> Option<YearRange>;

    /// Compute the value for a year (this needs to be done in time order)
    ///  year: year to compute values for
    ///  income: total income for that year
    fn simulate(
        &mut self,
        year: u32,
        totals: &YearlyTotals,
        settings: &Settings,
    ) -> Result<YearlyImpact, Box<dyn Error>>;

    fn write(&self, filepath: String);

    fn plot(&self, filepath: String);
}


pub fn scatter_plot(filepath: String, data: Vec<(String, &Table<u32>)>, title: String) {

    let (x_min, x_max, y_min, y_max) = range(data.iter().map(|(_table_name, table)| *table).collect());

    let root = BitMapBackend::new(&filepath, (1600, 1200)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 60).into_font())
        .margin(25)
        .x_label_area_size(60)
        .y_label_area_size(100)
        .build_cartesian_2d(x_min..x_max, y_min..y_max).unwrap();

    chart
        .configure_mesh()
        .x_label_style(("sans-serif", 25).into_font())
        .y_label_style(("sans-serif", 25).into_font())
        .bold_line_style(&BLACK.mix(0.8))
        .light_line_style(&BLACK.mix(0.1))
        .y_label_formatter(&|v| format!("${}", v))
        .draw()
        .unwrap();
    
    chart
        .configure_mesh()
        .disable_x_axis()
        .disable_y_axis()
        .x_label_style(("sans-serif", 40).into_font())
        .x_desc("Year")
        .draw()
        .unwrap();

    data.iter().enumerate().for_each(|(idx, (table_name, table))| {
        chart
            .draw_series(LineSeries::new(table.0.clone().into_iter(),COLORS[idx % COLORS.len()].stroke_width(4))).unwrap()
            .label(table_name)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 30, y)], COLORS[idx % COLORS.len()].stroke_width(4)));
    });
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .legend_area_size(40)
        .label_font(("sans-serif", 20).into_font())
        .position(SeriesLabelPosition::UpperRight)
        .draw().unwrap();
}


/// List of the types of accounts that are available
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub enum AccountType {
    Income,
    Ssa,
    Retirement,
    Hsa,
    College,
    Expense,
    Loan,
    Mortgage,
    Savings,
}

/// Account Wrapper for json data storage
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum AccountWrapper {
    Income(Income<String>),
    Ssa(Ssa),
    Retirement(Retirement<String>),
    Hsa(Hsa<String>),
    College(College<String>),
    Expense(Expense<String>),
    Loan(Loan<String>),
    Mortgage(Mortgage<String>),
    Savings(Savings<String>),
}

impl AccountWrapper {
    pub fn to_account_object(self) -> Box<dyn Account> {
        match self {
            AccountWrapper::Income(account) => Box::new(Income::<u32>::from(account)),
            AccountWrapper::Ssa(account) => Box::new(account),
            AccountWrapper::Retirement(account) => Box::new(Retirement::<u32>::from(account)),
            AccountWrapper::Hsa(account) => Box::new(Hsa::<u32>::from(account)),
            AccountWrapper::College(account) => Box::new(College::<u32>::from(account)),
            AccountWrapper::Expense(account) => Box::new(Expense::<u32>::from(account)),
            AccountWrapper::Loan(account) => Box::new(Loan::<u32>::from(account)),
            AccountWrapper::Mortgage(account) => Box::new(Mortgage::<u32>::from(account)),
            AccountWrapper::Savings(account) => Box::new(Savings::<u32>::from(account)),
        }
    }
    pub fn order() -> Vec<AccountType> {
        vec![
            AccountType::Income,
            AccountType::Ssa,
            AccountType::Hsa,
            AccountType::Expense,
            AccountType::Mortgage,
            AccountType::Loan,
            AccountType::College,
            AccountType::Retirement,
            AccountType::Savings,
        ]
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn income_value() {
//         let data = TabularData {
//             metrics: vec!["metric1".into(), "metric2".into(), "metric3".into()],
//             times: vec![1.0, 2.0, 3.0],
//             data: vec![
//                 vec![Some(1.0), Some(3.5), Some(2.3)],
//                 vec![Some(-5.6), Some(2.5), Some(7.9)],
//                 vec![Some(0.5), Some(8.0), Some(2.1)],
//             ],
//         };
//         let metadata = TelemetryInfo {
//             metrics: vec!["metric1".into(), "metric2".into(), "metric3".into()],
//             time_range: Limit { min: 1.0, max: 3.0 },
//         };
//         assert_eq!(TelemetryInfo::from(data), metadata);
//     }

// }
