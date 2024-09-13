#[derive(std::fmt::Debug)]
pub enum DomainError {
    Config(String),
    ConvertDomain(String),
    Database(String),
    Parse(String),
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DomainError::Config(ref message) => write!(f, "Config error: {}", message),
            DomainError::ConvertDomain(ref message) => write!(f, "Parse error: {}", message),
            DomainError::Database(ref message) => write!(f, "Database error: {}", message),
            DomainError::Parse(ref message) => write!(f, "Parse error: {}", message),
        }
    }
}

impl From<csv::Error> for DomainError {
    fn from(value: csv::Error) -> Self {
        DomainError::Parse(format!("Failed to read from csv: {}", value))
    }
}

impl From<sqlx::Error> for DomainError {
    fn from(value: sqlx::Error) -> Self {
        DomainError::Database(format!("Failed DB request: {}", value))
    }
}

impl From<std::num::ParseIntError> for DomainError {
    fn from(value: std::num::ParseIntError) -> Self {
        DomainError::Parse(format!("Failed to parse integer: {}", value))
    }
}

impl From<std::num::ParseFloatError> for DomainError {
    fn from(value: std::num::ParseFloatError) -> Self {
        DomainError::Parse(format!("Failed to parse float: {}", value))
    }
}

impl From<std::env::VarError> for DomainError {
    fn from(value: std::env::VarError) -> Self {
        DomainError::Parse(format!("Failed to read from env: {}", value))
    }
}

impl std::error::Error for DomainError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
