#[cfg(feature = "db")]
pub mod local;
#[cfg(feature = "db")]
pub mod persist;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
#[cfg(feature = "docs")]
use utoipa::ToSchema;

use crate::domain::common::RegionType;
use crate::error::DomainError;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "docs", derive(ToSchema))]
pub struct Zhvi {
    pub region_name: String,
    pub region_type: RegionType,
    pub home_type: HomeType,
    pub percentile: Percentile,
    pub prices: ZhviPrices,
}

impl Zhvi {
    pub fn region_name(&self) -> &str {
        &self.region_name
    }

    pub fn region_type(&self) -> &RegionType {
        &self.region_type
    }

    pub fn home_type(&self) -> &HomeType {
        &self.home_type
    }

    pub fn percentile(&self) -> &Percentile {
        &self.percentile
    }

    pub fn prices(&self) -> &ZhviPrices {
        &self.prices
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(
    feature = "db",
    sqlx(type_name = "home_type", rename_all = "lowercase")
)]
#[cfg_attr(feature = "docs", derive(ToSchema))]
pub enum HomeType {
    #[default]
    AllHomes,
    CondoCoOps,
    SingleFamilyHomes,
}

impl TryFrom<&str> for HomeType {
    type Error = crate::error::DomainError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "allhomes" => Ok(HomeType::AllHomes),
            "condocoops" => Ok(HomeType::CondoCoOps),
            "singlefamilyhomes" => Ok(HomeType::SingleFamilyHomes),
            _ => Err(DomainError::Parse("Failed to parse HomeType".to_string())),
        }
    }
}

impl std::fmt::Display for HomeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HomeType::AllHomes => write!(f, "allhomes"),
            HomeType::CondoCoOps => write!(f, "condocoops"),
            HomeType::SingleFamilyHomes => write!(f, "singlefamilyhomes"),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(
    feature = "db",
    sqlx(type_name = "percentile", rename_all = "lowercase")
)]
#[cfg_attr(feature = "docs", derive(ToSchema))]
pub enum Percentile {
    Bottom,
    #[default]
    Middle,
    Top,
}

impl TryFrom<&str> for Percentile {
    type Error = crate::error::DomainError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "bottom" => Ok(Percentile::Bottom),
            "middle" => Ok(Percentile::Middle),
            "top" => Ok(Percentile::Top),
            _ => Err(DomainError::Parse("Failed to parse Percentile".to_string())),
        }
    }
}

impl std::fmt::Display for Percentile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Percentile::Bottom => write!(f, "bottom"),
            Percentile::Middle => write!(f, "middle"),
            Percentile::Top => write!(f, "top"),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "docs", derive(ToSchema))]
pub struct ZhviPrice {
    pub date: NaiveDate,
    pub value: f64,
}

pub type ZhviPrices = Vec<ZhviPrice>;
pub type Zhvis = Vec<Zhvi>;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ZhviData {
    all_homes_zhvis: Zhvis,
    condo_coops_zhvis: Zhvis,
    single_family_homes_zhvis: Zhvis,
}

impl ZhviData {
    pub fn all_homes_zhvis(&self) -> &Zhvis {
        &self.all_homes_zhvis
    }

    pub fn condo_coops_zhvis(&self) -> &Zhvis {
        &self.condo_coops_zhvis
    }

    pub fn single_family_homes_zhvis(&self) -> &Zhvis {
        &self.single_family_homes_zhvis
    }
}
