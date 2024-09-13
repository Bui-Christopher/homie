use serde::{Deserialize, Serialize};

use crate::error::DomainError;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum DateInterval {
    Day,
    Month,
    #[default]
    Year,
}

impl TryFrom<&str> for DateInterval {
    type Error = crate::error::DomainError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "day" => Ok(DateInterval::Day),
            "month" => Ok(DateInterval::Month),
            "year" => Ok(DateInterval::Year),
            _ => Err(DomainError::Parse(
                "Failed to parse DateInterval".to_string(),
            )),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "region_type", rename_all = "lowercase")]
pub enum RegionType {
    ThreeZip,
    FiveZip,
    #[default]
    City,
    County,
}

impl TryFrom<&str> for RegionType {
    type Error = crate::error::DomainError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "threezip" => Ok(RegionType::ThreeZip),
            "fivezip" => Ok(RegionType::FiveZip),
            "city" => Ok(RegionType::City),
            "county" => Ok(RegionType::County),
            _ => Err(DomainError::Parse("Failed to parse RegionType".to_string())),
        }
    }
}

impl std::fmt::Display for RegionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegionType::ThreeZip => write!(f, "threezip"),
            RegionType::FiveZip => write!(f, "fivezip"),
            RegionType::City => write!(f, "city"),
            RegionType::County => write!(f, "county"),
        }
    }
}
