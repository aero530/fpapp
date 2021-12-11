//! Groups of [tables](Table) to provide standard format for simulating different [account](crate::accounts) types

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Write;

use super::Table;

/// A single [table](Table) of values for simple [account](crate::accounts) types
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SingleTable {
    /// Account value (meaning depends on account type)
    pub value: Table<u32>,
}

impl SingleTable {
    pub fn new(value: &Table<u32>) -> SingleTable {
        SingleTable {
            value: value.clone(),
        }
    }
    /// Write account values out to csv file
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
    /// Initialize a new year
    pub fn add_year(&mut self, year: u32, pull_value_forward: bool) -> Result<(), Box<dyn Error>> {
        match self.value.0.contains_key(&year) {
            true => Err(String::from("Year already exists.").into()),
            false => {
                let prev_value = match pull_value_forward {
                    true => self.value.most_recent_value().unwrap_or_default(),
                    false => 0_f64,
                };
                self.value.add(year, prev_value)
            }
        }
    }
}

/// A set of [tables](Table) for use with loan and mortgage [accounts](crate::accounts)
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct LoanTables {
    /// Outstanding loan amount
    pub value: Table<u32>,
    /// Interest accrued this year
    pub interest: Table<u32>,
    /// Payments made against the loan in each year
    pub payments: Table<u32>,
    /// Escrow amount used for mortgage type loans in each year
    pub escrow: Table<u32>,
    /// PMI used for mortgage type loans in each year
    pub insurance: Table<u32>,
}

impl LoanTables {
    pub fn new(
        value: &Table<u32>,
        interest: &Table<u32>,
        payments: &Table<u32>,
        escrow: &Table<u32>,
        insurance: &Table<u32>,
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
    /// Write account values out to csv file
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
                    self.escrow.get(*year).unwrap_or(0_f64),
                    self.insurance.get(*year).unwrap_or(0_f64),
                )
                .as_bytes(),
            )
            .unwrap();
        });
    }
    /// Initialize a new year
    pub fn add_year(&mut self, year: u32, pull_value_forward: bool) -> Result<(), Box<dyn Error>> {
        match self.value.0.contains_key(&year) {
            true => Err(String::from("Year already exists.").into()),
            false => {
                let prev_value = match pull_value_forward {
                    true => self.value.most_recent_value().unwrap_or_default(),
                    false => 0_f64,
                };
                self.value.add(year, prev_value)?;
                self.interest.add(year, 0_f64)?;
                self.payments.add(year, 0_f64)?;
                self.escrow.add(year, 0_f64)?;
                self.insurance.add(year, 0_f64)?;
                Ok(())
            }
        }
    }
}

/// A set of [tables](Table) for use with savings types of [accounts](crate::accounts)
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SavingsTables {
    /// Account balance
    pub value: Table<u32>,
    /// Amount of money put into the account in each year
    pub contributions: Table<u32>,
    /// Amount of money put into the account by an employer in each year
    pub employer_contributions: Table<u32>,
    /// Amount of interest earned by the account in each year
    pub earnings: Table<u32>,
    /// Amount of money withdrawn from the account in each year
    pub withdrawals: Table<u32>,
}

impl SavingsTables {
    pub fn new(
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
            employer_contributions: match employer_contributions {
                Some(table) => table.clone(),
                None => Table::default(),
            },
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
    /// Write account values out to csv file
    pub fn write(&self, filename: String) {
        let mut years: Vec<u32> = self.value.years();
        years.sort_unstable();

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
                    self.employer_contributions.get(*year).unwrap_or(0_f64),
                    self.earnings.get(*year).unwrap_or(0_f64),
                    self.withdrawals.get(*year).unwrap_or(0_f64),
                )
                .as_bytes(),
            )
            .unwrap();
        });
    }
    /// Initialize a new year
    pub fn add_year(&mut self, year: u32, pull_value_forward: bool) -> Result<(), Box<dyn Error>> {
        match self.value.0.contains_key(&year) {
            true => Err(String::from("Year already exists.").into()),
            false => {
                let prev_value = match pull_value_forward {
                    true => self.value.most_recent_value().unwrap_or_default(),
                    false => 0_f64,
                };
                self.value.add(year, prev_value)?;
                self.contributions.add(year, 0_f64)?;
                self.employer_contributions.add(year, 0_f64)?;
                self.earnings.add(year, 0_f64)?;
                self.withdrawals.add(year, 0_f64)?;
                Ok(())
            }
        }
    }
}
