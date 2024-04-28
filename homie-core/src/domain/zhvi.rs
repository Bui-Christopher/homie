use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::adapter::repository::Persist;
use crate::domain::common::{to_ymd_date, CsvRecord};
use crate::error::Error;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Zhvi {
    pub(crate) home_type: String,   // AllHomes/CondoCoOps/SingleFamilyHomes
    pub(crate) region_type: String, // Zipcode, City, County
    pub(crate) region_name: String,
    pub(crate) percentile: String, // Bottom, Middle, Top
    pub(crate) prices: ZhviPrices,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZhviPrice {
    pub(crate) date: NaiveDate,
    pub(crate) value: f64,
}

pub type ZhviPrices = Vec<ZhviPrice>;
pub type Zhvis = Vec<Zhvi>;

#[derive(Debug, Default, Serialize, Deserialize)]
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

#[derive(Debug, Default)]
pub struct ZhviQuery {
    start_date: NaiveDate,
    end_date: NaiveDate,
    interval_date: String, // Monthly or Yearly
    home_type: String,
    region_type: String,
    region_name: String,
    percentile: String,
}

impl ZhviQuery {
    pub fn new(
        start_date: NaiveDate,
        end_date: NaiveDate,
        interval_date: String,
        home_type: String,
        region_type: String,
        region_name: String,
        percentile: String,
    ) -> Self {
        ZhviQuery {
            start_date,
            end_date,
            interval_date,
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

    pub(crate) fn interval_date(&self) -> &str {
        &self.interval_date
    }

    pub(crate) fn home_type(&self) -> &str {
        &self.home_type
    }

    pub(crate) fn region_type(&self) -> &str {
        &self.region_type
    }

    pub(crate) fn region_name(&self) -> &str {
        &self.region_name
    }

    pub(crate) fn percentile(&self) -> &str {
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
    pub(crate) fn home_type(&self) -> &str {
        &self.home_type
    }

    pub(crate) fn region_type(&self) -> &str {
        &self.region_type
    }

    pub(crate) fn region_name(&self) -> &str {
        &self.region_name
    }

    pub(crate) fn percentile(&self) -> &str {
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
        let home_type = "AllHomes".to_string();
        let region_type = "City".to_string();
        let region_name = entry.0[2].clone();
        let percentile = "Middle".to_string();
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
        let home_type = "AllHomes".to_string();
        let region_type = "County".to_string();
        let region_name = entry.0[2].clone();
        let percentile = "Middle".to_string();
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
        let home_type = "AllHomes".to_string();
        let region_type = "Zipcode".to_string();
        let region_name = entry.0[2].clone();
        let percentile = "Middle".to_string();
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
