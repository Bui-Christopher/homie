use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::Type;

use crate::adapter::repository::Persist;
use crate::domain::common::DateInterval;
use crate::domain::util::{to_ymd_date, CsvRecord};
use crate::error::Error;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Zhvi {
    pub(crate) home_type: HomeType,
    pub(crate) region_type: RegionType,
    pub(crate) region_name: String,
    pub(crate) percentile: Percentile,
    pub(crate) prices: ZhviPrices,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
#[sqlx(type_name = "home_type", rename_all = "lowercase")]
pub enum HomeType {
    AllHomes,
    CondoCoOps,
    SingleFamilyHomes,
}

impl Default for HomeType {
    fn default() -> Self {
        Self::AllHomes
    }
}

impl TryFrom<&str> for HomeType {
    type Error = crate::error::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "allhomes" => Ok(HomeType::AllHomes),
            "condocoops" => Ok(HomeType::CondoCoOps),
            "singlefamilyhomes" => Ok(HomeType::SingleFamilyHomes),
            _ => Err(Error::Parse("Failed to parse HomeType".to_string())),
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

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
#[sqlx(type_name = "region_type", rename_all = "lowercase")]
pub enum RegionType {
    ThreeZip,
    FiveZip,
    City,
    County,
}

impl Default for RegionType {
    fn default() -> Self {
        Self::City
    }
}

impl TryFrom<&str> for RegionType {
    type Error = crate::error::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "threezip" => Ok(RegionType::ThreeZip),
            "fivezip" => Ok(RegionType::FiveZip),
            "city" => Ok(RegionType::City),
            "county" => Ok(RegionType::County),
            _ => Err(Error::Parse("Failed to parse RegionType".to_string())),
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

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
#[sqlx(type_name = "percentile", rename_all = "lowercase")]
pub enum Percentile {
    Bottom,
    Middle,
    Top,
}

impl Default for Percentile {
    fn default() -> Self {
        Self::Middle
    }
}

impl TryFrom<&str> for Percentile {
    type Error = crate::error::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "bottom" => Ok(Percentile::Bottom),
            "middle" => Ok(Percentile::Middle),
            "top" => Ok(Percentile::Top),
            _ => Err(Error::Parse("Failed to parse Percentile".to_string())),
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZhviPrice {
    pub(crate) date: NaiveDate,
    pub(crate) value: f64,
}

pub type ZhviPrices = Vec<ZhviPrice>;
pub type Zhvis = Vec<Zhvi>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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

#[derive(Debug)]
pub struct ZhviQuery {
    start_date: NaiveDate,
    end_date: NaiveDate,
    date_interval: DateInterval,
    home_type: HomeType,
    region_type: RegionType,
    region_name: String,
    percentile: Percentile,
}

impl ZhviQuery {
    pub fn new(
        start_date: NaiveDate,
        end_date: NaiveDate,
        date_interval: DateInterval,
        home_type: HomeType,
        region_type: RegionType,
        region_name: String,
        percentile: Percentile,
    ) -> Self {
        ZhviQuery {
            start_date,
            end_date,
            date_interval,
            home_type,
            region_type,
            region_name,
            percentile,
        }
    }

    pub(crate) fn start_date(&self) -> &NaiveDate {
        &self.start_date
    }

    pub(crate) fn end_date(&self) -> &NaiveDate {
        &self.end_date
    }

    pub(crate) fn date_interval(&self) -> &DateInterval {
        &self.date_interval
    }

    pub(crate) fn home_type(&self) -> &HomeType {
        &self.home_type
    }

    pub(crate) fn region_type(&self) -> &RegionType {
        &self.region_type
    }

    pub(crate) fn region_name(&self) -> &str {
        &self.region_name
    }

    pub(crate) fn percentile(&self) -> &Percentile {
        &self.percentile
    }
}

#[async_trait]
pub trait ZhviPersist: Send + Sync {
    // TODO: Return Keys instead of unit type
    async fn create_zhvi(&self, zhvi: &Zhvi) -> Result<(), Error>;
    async fn read_zhvi_by_id(&self, id: (&str, &str, &str, &str)) -> Result<Zhvi, Error>;
    async fn update_zhvi(&self, zhvi: &Zhvi) -> Result<(), Error>;
    async fn delete_zhvi_by_id(&self, id: (&str, &str, &str, &str)) -> Result<(), Error>;
    async fn read_zhvi_by_query(&self, query: &ZhviQuery) -> Result<Zhvis, Error>;
}

impl Zhvi {
    pub(crate) fn home_type(&self) -> &HomeType {
        &self.home_type
    }

    pub(crate) fn region_type(&self) -> &RegionType {
        &self.region_type
    }

    pub(crate) fn region_name(&self) -> &str {
        &self.region_name
    }

    pub(crate) fn percentile(&self) -> &Percentile {
        &self.percentile
    }

    pub(crate) fn prices(&self) -> &ZhviPrices {
        &self.prices
    }

    pub async fn create(&self, client: &dyn Persist) -> Result<(), Error> {
        client.create_zhvi(self).await
    }

    pub async fn read(client: &dyn Persist, id: (&str, &str, &str, &str)) -> Result<Zhvi, Error> {
        client.read_zhvi_by_id(id).await
    }

    pub async fn update(&self, client: &dyn Persist) -> Result<(), Error> {
        client.update_zhvi(self).await
    }

    pub async fn delete(client: &dyn Persist, id: (&str, &str, &str, &str)) -> Result<(), Error> {
        client.delete_zhvi_by_id(id).await
    }

    pub async fn read_by_query(client: &dyn Persist, query: &ZhviQuery) -> Result<Zhvis, Error> {
        client.read_zhvi_by_query(query).await
    }
}

// TODO:
// impl Zhvi {
//     fn from_entry_to_all_homes() -> Self {},
//     fn from_entry_to_condo_coops() -> Self {},
//     fn from_entry_to_single_family_homes() -> Self {},
// }
// Unit Tests

#[derive(Clone, Debug)]
pub(crate) struct ZhviConfig {
    mid_zip_all_homes_path: Option<String>,
    mid_city_all_homes_path: Option<String>,
    mid_county_all_homes_path: Option<String>,
}

impl ZhviConfig {
    pub fn new(
        mid_zip_all_homes_path: Option<String>,
        mid_city_all_homes_path: Option<String>,
        mid_county_all_homes_path: Option<String>,
    ) -> Self {
        ZhviConfig {
            mid_zip_all_homes_path,
            mid_city_all_homes_path,
            mid_county_all_homes_path,
        }
    }

    fn mid_zip_all_homes_path(&self) -> Option<&str> {
        self.mid_zip_all_homes_path.as_deref()
    }

    fn mid_city_all_homes_path(&self) -> Option<&str> {
        self.mid_city_all_homes_path.as_deref()
    }

    fn mid_county_all_homes_path(&self) -> Option<&str> {
        self.mid_county_all_homes_path.as_deref()
    }
}

pub(crate) fn read_zillow_zhvis(zhvi_config: &ZhviConfig) -> Result<ZhviData, Error> {
    let zhvi_data = ZhviData {
        all_homes_zhvis: read_all_homes_zhvis(zhvi_config)?,
        // condo_coops_zhvis = read_condo_coops_zhvis(zhvi_config)?;
        // single_family_homes_zhvis = read_single_family_homes_zhvis(zhvi_config)?;
        ..Default::default()
    };

    Ok(zhvi_data)
}

fn read_all_homes_zhvis(zhvi_config: &ZhviConfig) -> Result<Zhvis, Error> {
    let mut all_homes = Zhvis::default();
    if let Some(mid_zip_all_homes_path) = zhvi_config.mid_zip_all_homes_path() {
        all_homes.append(&mut read_mid_zip_all_homes(mid_zip_all_homes_path)?);
    }

    if let Some(mid_city_all_homes_path) = zhvi_config.mid_city_all_homes_path() {
        all_homes.append(&mut read_mid_city_all_homes(mid_city_all_homes_path)?);
    }

    if let Some(mid_county_all_homes_path) = zhvi_config.mid_county_all_homes_path() {
        all_homes.append(&mut read_mid_county_all_homes(mid_county_all_homes_path)?);
    }
    Ok(all_homes)
}

fn read_mid_city_all_homes(mid_city_all_homes_path: &str) -> Result<Zhvis, Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(mid_city_all_homes_path)?;
    let mut mid_all_homes = vec![];
    let entries: Vec<CsvRecord> = rdr.deserialize().filter_map(Result::ok).collect();
    let headers = rdr.headers()?;

    for entry in entries.into_iter() {
        // start at 8
        let mut prices = vec![];
        for i in 8..entry.0.len() {
            let parts: Vec<&str> = headers
                .iter()
                .nth(i)
                .ok_or(Error::Parse("Failed to parse string to date".to_string()))?
                .split('-')
                .collect();
            let year = parts[0].parse()?;
            let month = parts[1].parse()?;
            let day = parts[2].parse()?;
            let date = to_ymd_date(year, month, day)?;
            let value = entry.0[i].parse().unwrap_or_default();
            prices.push(ZhviPrice { date, value });
        }
        let home_type = HomeType::AllHomes;
        let region_type = RegionType::City;
        let region_name = entry.0[2].clone();
        let percentile = Percentile::Middle;
        mid_all_homes.push(Zhvi {
            home_type,
            region_type,
            region_name,
            percentile,
            prices,
        });
    }

    Ok(mid_all_homes)
}

fn read_mid_county_all_homes(mid_county_all_homes_path: &str) -> Result<Zhvis, Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(mid_county_all_homes_path)?;

    let mut mid_all_homes = vec![];
    let entries: Vec<CsvRecord> = rdr.deserialize().filter_map(Result::ok).collect();
    let headers = rdr.headers()?;
    for entry in entries.into_iter() {
        // start at 8
        let mut prices = vec![];
        for i in 9..entry.0.len() {
            let parts: Vec<&str> = headers
                .iter()
                .nth(i)
                .ok_or(Error::Parse("Failed to parse string to date".to_string()))?
                .split('-')
                .collect();
            let year = parts[0].parse()?;
            let month = parts[1].parse()?;
            let day = parts[2].parse()?;
            let date = to_ymd_date(year, month, day)?;
            let value = entry.0[i].parse().unwrap_or_default();
            prices.push(ZhviPrice { date, value });
        }
        let home_type = HomeType::AllHomes;
        let region_type = RegionType::County;
        let region_name = entry.0[2].clone();
        let percentile = Percentile::Middle;
        mid_all_homes.push(Zhvi {
            home_type,
            region_type,
            region_name,
            percentile,
            prices,
        });
    }

    Ok(mid_all_homes)
}

fn read_mid_zip_all_homes(mid_zip_all_homes_path: &str) -> Result<Zhvis, Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(mid_zip_all_homes_path)?;

    let mut mid_all_homes = vec![];
    let entries: Vec<CsvRecord> = rdr.deserialize().filter_map(Result::ok).collect();
    let headers = rdr.headers()?;
    for entry in entries.into_iter() {
        let mut prices = vec![];
        // start at 8
        for i in 9..entry.0.len() {
            let parts: Vec<&str> = headers
                .iter()
                .nth(i)
                .ok_or(Error::Parse("Failed to parse string to date".to_string()))?
                .split('-')
                .collect();
            let year = parts[0].parse()?;
            let month = parts[1].parse()?;
            let day = parts[2].parse()?;
            let date = to_ymd_date(year, month, day)?;
            let value = entry.0[i].parse().unwrap_or_default();
            prices.push(ZhviPrice { date, value });
        }
        let home_type = HomeType::AllHomes;
        let region_type = RegionType::FiveZip;
        let region_name = entry.0[2].clone();
        let percentile = Percentile::Middle;
        mid_all_homes.push(Zhvi {
            home_type,
            region_type,
            region_name,
            percentile,
            prices,
        });
    }

    Ok(mid_all_homes)
}
