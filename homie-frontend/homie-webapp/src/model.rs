// This file is currently used as referencing homie-core
// causes WASM target failing to build.

use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Zhvi {
    pub region_name: String,
    pub region_type: RegionType,
    pub home_type: HomeType,
    pub percentile: Percentile,
    pub prices: ZhviPrices,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum HomeType {
    #[default]
    AllHomes,
    CondoCoOps,
    SingleFamilyHomes,
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
pub enum Percentile {
    Bottom,
    #[default]
    Middle,
    Top,
}

impl std::fmt::Display for Percentile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Percentile::Bottom => write!(f, "Bottom"),
            Percentile::Middle => write!(f, "Middle"),
            Percentile::Top => write!(f, "Top"),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ZhviPrice {
    pub date: NaiveDate,
    pub value: f64,
}

pub type ZhviPrices = Vec<ZhviPrice>;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum RegionType {
    ThreeZip,
    FiveZip,
    #[default]
    City,
    County,
}

pub type Zhvis = Vec<Zhvi>;

pub struct Line {
    pub name: String,
    pub x: Vec<NaiveDate>,
    pub y: Vec<f64>,
}

impl Line {
    pub fn from_zhvi(name: &str, zhvi: &Zhvi) -> Line {
        Line {
            name: name.to_string(),
            x: zhvi.prices.iter().map(|price| price.date).collect(),
            y: zhvi.prices.iter().map(|price| price.value).collect(),
        }
    }
}
