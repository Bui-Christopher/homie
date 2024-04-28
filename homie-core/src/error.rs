use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Error {
    Config(String),
    ConvertDomain(String),
    Database(String),
    Parse(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Config(ref message) => write!(f, "Config error: {}", message),
            Error::ConvertDomain(ref message) => write!(f, "Parse error: {}", message),
            Error::Database(ref message) => write!(f, "Database error: {}", message),
            Error::Parse(ref message) => write!(f, "Parse error: {}", message),
        }
    }
}

impl From<csv::Error> for Error {
    fn from(value: csv::Error) -> Self {
        Error::Parse(format!("Failed to read from csv: {}", value))
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::Database(format!("Failed DB request: {}", value))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Error::Parse(format!("Failed to parse integer: {}", value))
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(value: std::num::ParseFloatError) -> Self {
        Error::Parse(format!("Failed to parse float: {}", value))
    }
}

impl From<std::env::VarError> for Error {
    fn from(value: std::env::VarError) -> Self {
        Error::Parse(format!("Failed to read from env: {}", value))
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
