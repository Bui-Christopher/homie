use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::common::RegionType;
use crate::adapter::repository::Persist;
use crate::domain::util::CsvRecord;
use crate::error::DomainError;

#[derive(Clone, Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Hpi {
    pub(crate) region_type: RegionType,
    pub(crate) region_name: String,
    pub(crate) year: i32,
    pub(crate) hpi: Option<f32>,
    pub(crate) annual_change: Option<f32>,
    pub(crate) hpi_1990_base: Option<f32>,
    pub(crate) hpi_2000_base: Option<f32>,
}

impl Hpi {
    pub(crate) fn region_type(&self) -> &RegionType {
        &self.region_type
    }

    pub(crate) fn region_name(&self) -> &String {
        &self.region_name
    }

    pub(crate) fn year(&self) -> i32 {
        self.year
    }

    pub(crate) fn hpi(&self) -> Option<f32> {
        self.hpi
    }

    pub(crate) fn annual_change(&self) -> Option<f32> {
        self.annual_change
    }

    pub(crate) fn hpi_1990_base(&self) -> Option<f32> {
        self.hpi_1990_base
    }

    pub(crate) fn hpi_2000_base(&self) -> Option<f32> {
        self.hpi_2000_base
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HpiData {
    three_zip_hpis: Hpis,
    five_zip_hpis: Hpis,
    county_hpis: Hpis,
}

impl HpiData {
    pub fn three_zip_hpis(&self) -> &Hpis {
        &self.three_zip_hpis
    }

    pub fn five_zip_hpis(&self) -> &Hpis {
        &self.five_zip_hpis
    }

    pub fn county_hpis(&self) -> &Hpis {
        &self.county_hpis
    }
}

pub type Hpis = Vec<Hpi>;

#[derive(Clone, Debug, Default)]
pub struct HpiQuery {
    // region_type: RegionType
    region_name: String,
    start_date: i32,
    end_date: i32,
    // annual_change: Option<bool>,
    // hpi_2000_base: Option<bool>,
}

impl HpiQuery {
    pub fn new(region_name: String, start_date: i32, end_date: i32) -> Self {
        Self {
            region_name,
            start_date,
            end_date,
        }
    }

    pub(crate) fn region_name(&self) -> &str {
        &self.region_name
    }

    pub(crate) fn start_date(&self) -> i32 {
        self.start_date
    }

    pub(crate) fn end_date(&self) -> i32 {
        self.end_date
    }
}

#[async_trait]
pub trait HpiPersist: Send + Sync {
    async fn create_hpi(&self, hpi: &Hpi) -> Result<(String, i32), DomainError>;
    async fn read_hpi_by_id(&self, id: (&str, i32)) -> Result<Hpi, DomainError>;
    async fn update_hpi(&self, hpi: &Hpi) -> Result<(), DomainError>;
    async fn delete_hpi_by_id(&self, id: (&str, i32)) -> Result<(), DomainError>;
    async fn read_hpi_by_query(&self, query: &HpiQuery) -> Result<Hpis, DomainError>;
}

impl Hpi {
    pub async fn create(&self, client: &dyn Persist) -> Result<(String, i32), DomainError> {
        client.create_hpi(self).await
    }

    pub async fn read(client: &dyn Persist, id: (&str, i32)) -> Result<Hpi, DomainError> {
        client.read_hpi_by_id(id).await
    }

    pub async fn update(&self, client: &dyn Persist) -> Result<(), DomainError> {
        client.update_hpi(self).await
    }

    pub async fn delete(client: &dyn Persist, id: (&str, i32)) -> Result<(), DomainError> {
        client.delete_hpi_by_id(id).await
    }

    pub async fn read_by_query(
        client: &dyn Persist,
        query: &HpiQuery,
    ) -> Result<Hpis, DomainError> {
        client.read_hpi_by_query(query).await
    }
}

#[derive(Clone, Debug)]
pub(crate) struct HpiConfig {
    three_zip_hpis_path: Option<String>,
    five_zip_hpis_path: Option<String>,
    county_hpis_path: Option<String>,
}

impl HpiConfig {
    pub fn new(
        three_zip_hpis_path: Option<String>,
        five_zip_hpis_path: Option<String>,
        county_hpis_path: Option<String>,
    ) -> Self {
        HpiConfig {
            three_zip_hpis_path,
            five_zip_hpis_path,
            county_hpis_path,
        }
    }

    fn three_zip_hpi_path(&self) -> Option<&str> {
        self.three_zip_hpis_path.as_deref()
    }

    fn five_zip_hpi_path(&self) -> Option<&str> {
        self.five_zip_hpis_path.as_deref()
    }

    fn county_hpi_path(&self) -> Option<&str> {
        self.county_hpis_path.as_deref()
    }
}

pub(crate) fn read_fhfa_hpis(hpi_config: &HpiConfig) -> Result<HpiData, DomainError> {
    let mut hpi_data = HpiData::default();

    if let Some(three_zpi_hpi_path) = hpi_config.three_zip_hpi_path() {
        hpi_data.three_zip_hpis = read_three_zip_fhfa_hpis(three_zpi_hpi_path)?;
    }
    if let Some(five_zpi_hpi_path) = hpi_config.five_zip_hpi_path() {
        hpi_data.five_zip_hpis = read_five_zip_fhfa_hpis(five_zpi_hpi_path)?;
    }
    if let Some(county_hpi_path) = hpi_config.county_hpi_path() {
        hpi_data.county_hpis = read_county_fhfa_hpis(county_hpi_path)?;
    }
    Ok(hpi_data)
}

fn read_three_zip_fhfa_hpis(three_zip_path: &str) -> Result<Hpis, DomainError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(three_zip_path)?;
    let entries: Vec<CsvRecord> = rdr.deserialize().filter_map(Result::ok).collect();
    Ok(entries
        .into_iter()
        .map(|entry| {
            let region_type = RegionType::ThreeZip;
            let region_name = entry.0[0].clone();
            let year = entry.0[1].parse().unwrap();
            let annual_change = entry.0[2].parse().ok();
            let hpi = entry.0[3].parse().ok();
            let hpi_1990_base = entry.0[4].parse().ok();
            let hpi_2000_base = entry.0[5].parse().ok();
            Hpi {
                region_type,
                region_name,
                year,
                annual_change,
                hpi,
                hpi_1990_base,
                hpi_2000_base,
            }
        })
        .collect())
}

fn read_five_zip_fhfa_hpis(five_zip_path: &str) -> Result<Hpis, DomainError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(five_zip_path)?;

    let entries: Vec<CsvRecord> = rdr.deserialize().filter_map(Result::ok).collect();
    Ok(entries
        .into_iter()
        .map(|entry| {
            let region_type = RegionType::FiveZip;
            let region_name = entry.0[0].clone();
            let year = entry.0[1].parse().unwrap();
            let annual_change = entry.0[2].parse().ok();
            let hpi = entry.0[3].parse().ok();
            let hpi_1990_base = entry.0[4].parse().ok();
            let hpi_2000_base = entry.0[5].parse().ok();
            Hpi {
                region_type,
                region_name,
                year,
                annual_change,
                hpi,
                hpi_1990_base,
                hpi_2000_base,
            }
        })
        .collect())
}

fn read_county_fhfa_hpis(county_path: &str) -> Result<Hpis, DomainError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(county_path)?;

    let entries: Vec<CsvRecord> = rdr.deserialize().filter_map(Result::ok).collect();
    Ok(entries
        .into_iter()
        .map(|entry| {
            let region_type = RegionType::County;
            let region_name = entry.0[1].clone();
            let year = entry.0[3].parse().unwrap();
            let annual_change = entry.0[4].parse().ok();
            let hpi = entry.0[5].parse().ok();
            let hpi_1990_base = entry.0[6].parse().ok();
            let hpi_2000_base = entry.0[7].parse().ok();
            Hpi {
                region_type,
                region_name,
                year,
                annual_change,
                hpi,
                hpi_1990_base,
                hpi_2000_base,
            }
        })
        .collect())
}
