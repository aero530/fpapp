use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Write;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Table<T: std::cmp::Eq + std::hash::Hash + std::cmp::PartialEq + std::cmp::Ord>(
    pub BTreeMap<T, f64>,
);

impl Table<u32> {
    pub fn get(&self, year: u32) -> Option<f64> {
        match self.0.get(&year) {
            Some(v) => Some(*v),
            None => None,
        }
    }
    pub fn most_recent_populated_year(&self) -> Option<u32> {
        self.0
            .iter()
            .filter(|(_k, v)| **v > f64::EPSILON)
            .map(|(k, _v)| *k)
            .collect::<Vec<u32>>()
            .iter()
            .copied()
            .max()
    }
    pub fn pull_value_forward(&mut self, year: u32) {
        match self.most_recent_populated_year() {
            Some(recent_year) => {
                if recent_year == year - 1 {
                    *self.0.get_mut(&year).unwrap() = self.0[&(year - 1)];
                }
            }
            None => {}
        }
    }
}

// and we'll implement IntoIterator
impl IntoIterator for Table<u32> {
    type Item = (u32, f64);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .keys()
            .zip(self.0.values())
            .map(|(x, y)| (*x, *y))
            .collect::<Vec<(u32, f64)>>()
            .into_iter()
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

impl From<(Vec<u32>, Vec<f64>)> for Table<u32> {
    fn from(other: (Vec<u32>, Vec<f64>)) -> Self {
        let mut map = BTreeMap::new();
        other
            .0
            .iter()
            .zip(other.1)
            .into_iter()
            .for_each(|(year, value)| {
                map.insert(*year, value);
            });
        Self(map)
    }
}

// /// Trait used to define what each account type must be able to provide
// pub trait Result: std::fmt::Debug {
//     fn new(&self) -> AccountType;
//     fn write(&self, filepath: String);
//     fn plot(&self, filepath: String);
// }

/// A single table of values
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SingleTable {
    pub value: Table<u32>,
}

impl SingleTable {
    pub fn new(value: &Table<u32>) -> SingleTable {
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

/// A set of tables for use with loans and mortgage accounts
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct LoanTables {
    /// Outstanding loan amount
    pub value: Table<u32>,
    /// Interest accrued this year
    pub interest: Table<u32>,
    /// Payments made against the loan
    pub payments: Table<u32>,
    /// Escrow amount used for mortgage type loans
    pub escrow: Option<Table<u32>>,
    /// PMI used for mortgage type loans
    pub insurance: Option<Table<u32>>,
}

impl LoanTables {
    pub fn new(
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
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SavingsTables {
    pub value: Table<u32>,
    pub contributions: Table<u32>,
    pub employer_contributions: Option<Table<u32>>,
    pub earnings: Table<u32>,
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

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub enum AnalysisTable {
//     Single(SingleTable),
//     Savings(SavingsTables),
//     Loan(LoanTables),
// }

// impl AnalysisTable {
//     // pub fn most_recent_populated_year(&self) -> Option<u32> {
//     //     match self {
//     //         AnalysisTable::Single(tables) => tables.value.most_recent_populated_year(),
//     //         AnalysisTable::Savings(tables) => tables.value.most_recent_populated_year(),
//     //         AnalysisTable::Loan(tables) => tables.value.most_recent_populated_year(),
//     //     }
//     // }
//     pub fn pull_value_forward(&mut self, year: u32) {
//         match self {
//             AnalysisTable::Single(tables) => {tables.value.pull_value_forward(year);},
//             AnalysisTable::Savings(tables) => {tables.value.pull_value_forward(year);},
//             AnalysisTable::Loan(tables) => {tables.value.pull_value_forward(year);},
//         }
//     }
//     pub fn value_table(&self) -> &Table<u32> {
//         match self {
//             AnalysisTable::Single(tables) => &tables.value,
//             AnalysisTable::Savings(tables) => &tables.value,
//             AnalysisTable::Loan(tables) => &tables.value,
//         }
//     }
//     // pub fn value_table_as_mut(&self) -> &mut Table<u32> {
//     //     match self {
//     //         AnalysisTable::Single(tables) => {

//     //             &mut tables.as_mut().value
//     //         },
//     //         AnalysisTable::Savings(tables) => &mut tables.value,
//     //         AnalysisTable::Loan(tables) => &mut tables.value,
//     //     }
//     // }
//     pub fn contributions_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Savings(tables) => Some(&tables.contributions),
//             _ => None,
//         }
//     }
//     pub fn employer_contributions_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Savings(tables) => tables.employer_contributions.as_ref(),
//             _ => None,
//         }
//     }
//     pub fn earnings_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Savings(tables) => Some(&tables.earnings),
//             _ => None,
//         }
//     }
//     pub fn withdrawals_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Savings(tables) => Some(&tables.withdrawals),
//             _ => None,
//         }
//     }
//     pub fn interest_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Loan(tables) => Some(&tables.interest),
//             _ => None,
//         }
//     }
//     pub fn payments_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Loan(tables) => Some(&tables.payments),
//             _ => None,
//         }
//     }
//     pub fn escrow_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Loan(tables) => tables.escrow.as_ref(),
//             _ => None,
//         }
//     }
//     pub fn insurance_table(&self) -> Option<&Table<u32>> {
//         match self {
//             AnalysisTable::Loan(tables) => tables.insurance.as_ref(),
//             _ => None,
//         }
//     }
//     pub fn value(&self, year: u32) -> f64 {
//         self.value_table().get(year).unwrap_or_default()
//         // self.analysis
//         //     .as_ref()
//         //     .unwrap()
//         //     .value
//         //     .0
//         //     .get(&year)
//         //     .map(|v| *v)
//     }
//     pub fn write(&self, filepath: String) {
//         println!("{}", filepath);

//     }

// }
