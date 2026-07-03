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
    /// The bare message, without the variant's Display prefix.  Use this when
    /// nesting an error inside another so prefixes don't stack up.
    pub fn message(&self) -> &str {
        match self {
            Error::Data(m) | Error::Config(m) | Error::Internal(m) => m,
            Error::Simulation { message, .. } => message,
        }
    }
    /// Prepend context to the message while keeping the variant (so wrapping
    /// does not turn everything into a differently-typed error or duplicate
    /// the Display prefix).
    pub(crate) fn with_context(self, context: impl std::fmt::Display) -> Self {
        match self {
            Error::Data(m) => Error::Data(format!("{context}: {m}")),
            Error::Config(m) => Error::Config(format!("{context}: {m}")),
            Error::Internal(m) => Error::Internal(format!("{context}: {m}")),
            e @ Error::Simulation { .. } => e,
        }
    }
}
