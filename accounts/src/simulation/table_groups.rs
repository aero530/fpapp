//! Groups of [tables](Table) to provide standard format for simulating different account types

use serde::{Deserialize, Serialize};

use super::{PlotDataPoint, PlotDataSet, Table};
use crate::Error;

/// Trait for table groups
pub trait TableGroup {
    /// Return analysis data to use in UI plotting
    fn get_plot_data(&self) -> Vec<PlotDataSet>;
    /// Add a new year to the tables in this group
    fn add_year(&mut self, year: u32, pull_value_forward: bool) -> Result<(), Error>;
}

/// Build a plot series from a table
fn plot_set(label: &str, table: &Table) -> PlotDataSet {
    PlotDataSet {
        label: String::from(label),
        data: table
            .0
            .iter()
            .map(|(year, value)| PlotDataPoint {
                x: *year,
                y: *value,
            })
            .collect(),
    }
}

/// The starting balance for a new year: the most recent balance prior to
/// `year` when rolling forward (never a future pre-seeded value), otherwise zero
fn new_year_value(value: &Table, year: u32, pull_value_forward: bool) -> f64 {
    match pull_value_forward {
        true => value.most_recent_value_before(year).unwrap_or_default(),
        false => 0_f64,
    }
}

/// Insert a zero for the year if the table does not already have a value
/// (pre-seeded historical values are kept)
fn ensure_year(table: &mut Table, year: u32) {
    if table.get(year).is_none() {
        table.insert(year, 0_f64);
    }
}

/// A single [table](Table) of values for simple account types
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SingleTable {
    /// Account value (meaning depends on account type)
    pub value: Table,
}

impl SingleTable {
    pub fn new(value: &Table) -> SingleTable {
        SingleTable {
            value: value.clone(),
        }
    }
}

impl TableGroup for SingleTable {
    fn get_plot_data(&self) -> Vec<PlotDataSet> {
        vec![plot_set("Value", &self.value)]
    }
    fn add_year(&mut self, year: u32, pull_value_forward: bool) -> Result<(), Error> {
        match self.value.0.contains_key(&year) {
            true => Err(Error::internal("year already exists")),
            false => {
                let prev_value = new_year_value(&self.value, year, pull_value_forward);
                self.value.add(year, prev_value)
            }
        }
    }
}

/// A set of [tables](Table) for use with loan and mortgage accounts
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct LoanTables {
    /// Outstanding loan amount
    pub value: Table,
    /// Interest accrued this year
    pub interest: Table,
    /// Payments made against the loan in each year
    pub payments: Table,
    /// Escrow amount used for mortgage type loans in each year
    pub escrow: Table,
    /// PMI used for mortgage type loans in each year
    pub insurance: Table,
}

impl LoanTables {
    pub fn new(
        value: &Table,
        interest: &Table,
        payments: &Table,
        escrow: &Table,
        insurance: &Table,
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
    /// Return analysis data to use in UI plotting
    pub fn get_mortgage_plot_data(&self) -> Vec<PlotDataSet> {
        vec![
            plot_set("Value", &self.value),
            plot_set("Interest", &self.interest),
            plot_set("Payments", &self.payments),
            plot_set("Escrow", &self.escrow),
            plot_set("Insurance", &self.insurance),
        ]
    }
}

impl TableGroup for LoanTables {
    /// Return analysis data to use in UI plotting
    fn get_plot_data(&self) -> Vec<PlotDataSet> {
        vec![
            plot_set("Value", &self.value),
            plot_set("Interest", &self.interest),
            plot_set("Payments", &self.payments),
        ]
    }

    /// Initialize a new year
    fn add_year(&mut self, year: u32, pull_value_forward: bool) -> Result<(), Error> {
        match self.value.0.contains_key(&year) {
            true => Err(Error::internal("year already exists")),
            false => {
                let prev_value = new_year_value(&self.value, year, pull_value_forward);
                self.value.add(year, prev_value)?;
                ensure_year(&mut self.interest, year);
                ensure_year(&mut self.payments, year);
                ensure_year(&mut self.escrow, year);
                ensure_year(&mut self.insurance, year);
                Ok(())
            }
        }
    }
}

/// A set of [tables](Table) for use with savings types of accounts
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SavingsTables {
    /// Account balance
    pub value: Table,
    /// Amount of money put into the account in each year
    pub contributions: Table,
    /// Amount of money put into the account by an employer in each year
    pub employer_contributions: Table,
    /// Amount of interest earned by the account in each year
    pub earnings: Table,
    /// Amount of money withdrawn from the account in each year
    pub withdrawals: Table,
}

impl SavingsTables {
    pub fn new(
        value: &Table,
        contributions: &Option<Table>,
        employer_contributions: &Option<Table>,
        earnings: &Option<Table>,
        withdrawals: &Option<Table>,
    ) -> SavingsTables {
        SavingsTables {
            value: value.clone(),
            contributions: contributions.clone().unwrap_or_default(),
            employer_contributions: employer_contributions.clone().unwrap_or_default(),
            earnings: earnings.clone().unwrap_or_default(),
            withdrawals: withdrawals.clone().unwrap_or_default(),
        }
    }
    /// Return analysis data to use in UI plotting
    pub fn get_matching_plot_data(&self) -> Vec<PlotDataSet> {
        vec![
            plot_set("Value", &self.value),
            plot_set("Contributions", &self.contributions),
            plot_set("Employer Contributions", &self.employer_contributions),
            plot_set("Earnings", &self.earnings),
            plot_set("Withdrawals", &self.withdrawals),
        ]
    }
}

impl TableGroup for SavingsTables {
    /// Return analysis data to use in UI plotting
    fn get_plot_data(&self) -> Vec<PlotDataSet> {
        vec![
            plot_set("Value", &self.value),
            plot_set("Contributions", &self.contributions),
            plot_set("Earnings", &self.earnings),
            plot_set("Withdrawals", &self.withdrawals),
        ]
    }
    /// Initialize a new year
    fn add_year(&mut self, year: u32, pull_value_forward: bool) -> Result<(), Error> {
        match self.value.0.contains_key(&year) {
            true => Err(Error::internal("year already exists")),
            false => {
                let prev_value = new_year_value(&self.value, year, pull_value_forward);
                self.value.add(year, prev_value)?;
                ensure_year(&mut self.contributions, year);
                ensure_year(&mut self.employer_contributions, year);
                ensure_year(&mut self.earnings, year);
                ensure_year(&mut self.withdrawals, year);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_year_pulls_forward_prior_value_not_future_value() {
        // Regression test for A3: a pre-seeded future year must not become the
        // starting balance for an earlier year.
        let mut tables = SavingsTables::new(
            &Table([(2020, 100.0), (2025, 500.0)].into_iter().collect()),
            &None,
            &None,
            &None,
            &None,
        );
        tables.add_year(2021, true).unwrap();
        assert_eq!(tables.value.get(2021), Some(100.0));
        // and the future year is untouched
        assert_eq!(tables.value.get(2025), Some(500.0));
    }

    #[test]
    fn add_year_keeps_seeded_subtable_values() {
        // A historical contributions entry for a year without a balance entry
        // must not abort the run.
        let mut tables = SavingsTables::new(
            &Table([(2020, 100.0)].into_iter().collect()),
            &Some(Table([(2021, 42.0)].into_iter().collect())),
            &None,
            &None,
            &None,
        );
        tables.add_year(2021, true).unwrap();
        assert_eq!(tables.contributions.get(2021), Some(42.0));
    }
}
