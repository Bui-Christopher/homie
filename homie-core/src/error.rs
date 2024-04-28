use std::error::Error as StdError;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Config(String),
    ConvertDomain(String),
    Database(String),
    Parse(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut self::Formatter) -> fmt::Result {
        match *self {
            Error::Config(ref message) => write!(f, "Config error: {}", message),
            Error::ConvertDomain(ref message) => write!(f, "Parse error: {}", message),
            Error::Database(ref message) => write!(f, "Database error: {}", message),
            Error::Parse(ref message) => write!(f, "Parse error: {}", message),
        }
    }
}

impl From<csv::Error> for Error {
    fn from(_value: csv::Error) -> Self {
        todo!()
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}
// Ok(entries
//     .into_iter()
//     .map(|entry| {
//         let region = entry.0[0].clone();
//         let year = entry.0[1].parse().map_err(|e|
// Error::Parse(e.to_string()))?;         let annual_change =
// entry.0[2].parse().map_err(|e| Error::Parse(e.to_string()))?;         let hpi
// = entry.0[3].parse().ok();         let hpi_1990_base =
// entry.0[4].parse().map_err(|e| Error::Parse(e.to_string()))?;         let
// hpi_2000_base = entry.0[5].parse().map_err(|e| Error::Parse(e.to_string()));
//         Ok(Hpi {
//             region,
//             year,
//             annual_change,
//             hpi,
//             hpi_1990_base,
//             hpi_2000_base,
//         })
//     })
//     .collect())
