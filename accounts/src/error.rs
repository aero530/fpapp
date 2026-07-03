//! Crate error type

/// Errors produced while loading, validating, or simulating a plan.
///
/// The variants let a UI distinguish "your file is malformed" from "your plan
/// is inconsistent" from "the simulation itself failed".
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Malformed data in the user's data file (bad year keys, unparseable values)
    #[error("invalid data: {0}")]
    Data(String),
    /// Configuration that cannot be simulated (validation failures)
    #[error("invalid configuration: {0}")]
    Config(String),
    /// A failure during the year-by-year simulation
    #[error("account '{account}', year {year}: {message}")]
    Simulation {
        account: String,
        year: u32,
        message: String,
    },
    /// An internal invariant was violated (a bug, not a user error)
    #[error("internal error: {0}")]
    Internal(String),
}

impl Error {
    pub fn data(message: impl Into<String>) -> Self {
        Error::Data(message.into())
    }
    pub fn config(message: impl Into<String>) -> Self {
        Error::Config(message.into())
    }
    pub fn internal(message: impl Into<String>) -> Self {
        Error::Internal(message.into())
    }
}
