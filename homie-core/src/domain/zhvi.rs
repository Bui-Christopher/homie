use std::error::Error;

use async_trait::async_trait;
use chrono::NaiveDate;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::adapter::repository::Persist;
use crate::domain::common::{to_ymd_date, CsvRecord};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Zhvi {
    pub home_type: String,   // AllHomes/CondoCoOps/SingleFamilyHomes
    pub region_type: String, // Zipcode, City, County
    pub region_name: String,
    pub percentile: String, // Bottom, Middle, Top
    pub prices: ZhviPrices,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZhviPrice {
    pub date: NaiveDate,
    pub value: f64,
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
    pub fn start_date(&self) -> &NaiveDate {
        &self.start_date
    }

    pub fn end_date(&self) -> &NaiveDate {
        &self.end_date
    }

    pub fn interval_date(&self) -> &str {
        &self.interval_date
    }

    pub fn home_type(&self) -> &str {
        &self.home_type
    }

    pub fn region_type(&self) -> &str {
        &self.region_type
    }

    pub fn region_name(&self) -> &str {
        &self.region_name
    }

    pub fn percentile(&self) -> &str {
        &self.percentile
    }
}

#[async_trait]
pub trait ZhviPersist: Send + Sync {
    // TODO: Return Keys instead of unit type
    async fn create_zhvi(&self, zhvi: &Zhvi) -> Result<(), Box<dyn Error>>;
    async fn read_zhvi_by_id(&self, id: (&str, &str, &str, &str)) -> Result<Zhvi, Box<dyn Error>>;
    async fn update_zhvi(&self, zhvi: &Zhvi) -> Result<(), Box<dyn Error>>;
    async fn delete_zhvi_by_id(&self, id: (&str, &str, &str, &str)) -> Result<(), Box<dyn Error>>;
    async fn read_zhvi_by_query(&self, query: &ZhviQuery) -> Result<Zhvis, Box<dyn Error>>;
}

impl Zhvi {
    pub fn home_type(&self) -> &str {
        &self.home_type
    }

    pub fn region_type(&self) -> &str {
        &self.region_type
    }

    pub fn region_name(&self) -> &str {
        &self.region_name
    }

    pub fn percentile(&self) -> &str {
        &self.percentile
    }

    pub fn prices(&self) -> &ZhviPrices {
        &self.prices
    }

    pub async fn create(&self, client: &dyn Persist) -> Result<(), Box<dyn Error>> {
        client.create_zhvi(self).await
    }

    pub async fn read(
        client: &dyn Persist,
        id: (&str, &str, &str, &str),
    ) -> Result<Zhvi, Box<dyn Error>> {
        client.read_zhvi_by_id(id).await
    }

    pub async fn update(&self, client: &dyn Persist) -> Result<(), Box<dyn Error>> {
        client.update_zhvi(self).await
    }

    pub async fn delete(
        client: &dyn Persist,
        id: (&str, &str, &str, &str),
    ) -> Result<(), Box<dyn Error>> {
        client.delete_zhvi_by_id(id).await
    }

    pub async fn read_by_query(
        client: &dyn Persist,
        query: &ZhviQuery,
    ) -> Result<Zhvis, Box<dyn Error>> {
        client.read_zhvi_by_query(query).await
    }

    // TODO: Delete
    pub fn generate_dummy_data() -> Vec<Zhvi> {
        let mut rng = rand::thread_rng();
        let mut dummy_data = Vec::new();
        for _ in 0..2 {
            let home_type = "SingleFamilyHomes".to_string();
            let region_type = "City".to_string();
            let region_name = "Irvine".to_string();
            let percentile = "Middle".to_string();
            let prices = generate_dummy_prices(&mut rng);
            let zhvi = Zhvi {
                home_type,
                region_type,
                region_name,
                percentile,
                prices,
            };
            dummy_data.push(zhvi);
        }

        dummy_data
    }
}

// TODO: Delete
fn generate_dummy_prices(rng: &mut impl Rng) -> ZhviPrices {
    let mut prices = Vec::new();
    for year in 2022..=2022 {
        for month in 7..=12 {
            let date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let value = rng.gen_range(100_000.0..=1_000_000.0);
            prices.push(ZhviPrice { date, value });
        }
    }
    prices
}

// TODO:
// impl Zhvi {
//     fn from_entry_to_all_homes() -> Self {},
//     fn from_entry_to_condo_coops() -> Self {},
//     fn from_entry_to_single_family_homes() -> Self {},
// }
// Unit Tests

pub struct ZhviConfig {
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

    fn has_mid_zip_all_homes_path(&self) -> bool {
        self.mid_zip_all_homes_path.is_some()
    }

    fn has_mid_city_all_homes_path(&self) -> bool {
        self.mid_city_all_homes_path.is_some()
    }

    fn has_mid_county_all_homes_path(&self) -> bool {
        self.mid_county_all_homes_path.is_some()
    }

    fn mid_zip_all_homes_path(&self) -> &str {
        self.mid_zip_all_homes_path.as_ref().unwrap()
    }

    fn mid_city_all_homes_path(&self) -> &str {
        self.mid_city_all_homes_path.as_ref().unwrap()
    }

    fn mid_county_all_homes_path(&self) -> &str {
        self.mid_county_all_homes_path.as_ref().unwrap()
    }
}

pub fn read_zillow_zhvis(zhvi_config: &ZhviConfig) -> Result<ZhviData, Box<dyn Error>> {
    let zhvi_data = ZhviData {
        all_homes_zhvis: read_all_homes_zhvis(zhvi_config)?,
        // condo_coops_zhvis = read_condo_coops_zhvis(zhvi_config)?;
        // single_family_homes_zhvis = read_single_family_homes_zhvis(zhvi_config)?;
        ..Default::default()
    };

    Ok(zhvi_data)
}

fn read_all_homes_zhvis(zhvi_config: &ZhviConfig) -> Result<Zhvis, Box<dyn Error>> {
    let mut all_homes = Zhvis::default();
    if zhvi_config.has_mid_zip_all_homes_path() {
        all_homes.append(&mut read_mid_zip_all_homes(
            zhvi_config.mid_zip_all_homes_path(),
        )?);
    }
    if zhvi_config.has_mid_city_all_homes_path() {
        all_homes.append(&mut read_mid_city_all_homes(
            zhvi_config.mid_city_all_homes_path(),
        )?);
    }
    if zhvi_config.has_mid_county_all_homes_path() {
        all_homes.append(&mut read_mid_county_all_homes(
            zhvi_config.mid_county_all_homes_path(),
        )?);
    }
    Ok(all_homes)
}

fn read_mid_city_all_homes(mid_city_all_homes_path: &str) -> Result<Zhvis, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(mid_city_all_homes_path)?;
    // TODO: rdr.deserialize().into_iter()?.into().collect();

    let mut entries = vec![];
    let mut mid_all_homes = vec![];
    for result in rdr.deserialize() {
        let r: CsvRecord = result?;
        entries.push(r);
    }
    let headers = rdr.headers()?;

    for entry in entries.into_iter() {
        // start at 8
        let mut prices = vec![];
        for i in 8..entry.0.len() {
            let parts: Vec<&str> = headers.iter().nth(i).unwrap().split('-').collect();
            let year = parts[0].parse().unwrap();
            let month = parts[1].parse().unwrap();
            let day = parts[2].parse().unwrap();
            let date = to_ymd_date(year, month, Some(day)).unwrap();
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

fn read_mid_county_all_homes(mid_county_all_homes_path: &str) -> Result<Zhvis, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(mid_county_all_homes_path)?;
    // TODO: rdr.deserialize().into_iter()?.into().collect();

    let mut entries = vec![];
    let mut mid_all_homes = vec![];
    for result in rdr.deserialize() {
        let r: CsvRecord = result?;
        entries.push(r);
    }
    let headers = rdr.headers()?;

    for entry in entries.into_iter() {
        // start at 8
        let mut prices = vec![];
        for i in 9..entry.0.len() {
            let parts: Vec<&str> = headers.iter().nth(i).unwrap().split('-').collect();
            let year = parts[0].parse().unwrap();
            let month = parts[1].parse().unwrap();
            let day = parts[2].parse().unwrap();
            let date = to_ymd_date(year, month, Some(day)).unwrap();
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

fn read_mid_zip_all_homes(mid_zip_all_homes_path: &str) -> Result<Zhvis, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(mid_zip_all_homes_path)?;
    // TODO: rdr.deserialize().into_iter()?.into().collect();

    let mut entries = vec![];
    let mut mid_all_homes = vec![];
    for result in rdr.deserialize() {
        let r: CsvRecord = result?;
        entries.push(r);
    }
    let headers = rdr.headers()?;

    for entry in entries.into_iter() {
        let mut prices = vec![];
        // start at 8
        for i in 9..entry.0.len() {
            let parts: Vec<&str> = headers.iter().nth(i).unwrap().split('-').collect();
            let year = parts[0].parse().unwrap();
            let month = parts[1].parse().unwrap();
            let day = parts[2].parse().unwrap();
            let date = to_ymd_date(year, month, Some(day)).unwrap();
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
