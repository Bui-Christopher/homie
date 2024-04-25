use std::error::Error;

use async_trait::async_trait;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::adapter::repository::Persist;
use crate::domain::common::CsvRecord;

#[derive(Clone, Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Hpi {
    region: String,
    year: i32,
    hpi: Option<f32>,
    annual_change: Option<f32>,
    hpi_1990_base: Option<f32>,
    hpi_2000_base: Option<f32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
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

    // TODO: Delete
    pub fn generate_dummy_data() -> Vec<Hpi> {
        let mut rng = rand::thread_rng();
        let mut dummy_data = Vec::new();

        // Generate dummy data for County variant
        for _ in 0..2 {
            let region = "Orange".to_string();
            let year = rng.gen_range(2000..=2022);
            let hpi = Some(rng.gen::<f32>());
            let annual_change = Some(rng.gen::<f32>());
            let hpi_1990_base = None;
            let hpi_2000_base = None;
            let county_hpi = Hpi {
                region,
                year,
                hpi,
                annual_change,
                hpi_1990_base,
                hpi_2000_base,
            };
            dummy_data.push(county_hpi);
        }

        dummy_data
    }
}

pub type Hpis = Vec<Hpi>;

#[derive(Debug, Default)]
pub struct HpiQuery {}

#[async_trait]
pub trait HpiPersist: Send + Sync {
    async fn create_hpi(&self, hpi: &Hpi) -> Result<(String, i32), Box<dyn Error>>;
    async fn read_hpi_by_id(&self, id: (&str, i32)) -> Result<Hpi, Box<dyn Error>>;
    async fn update_hpi(&self, hpi: &Hpi) -> Result<(), Box<dyn Error>>;
    async fn delete_hpi_by_id(&self, id: (&str, i32)) -> Result<(), Box<dyn Error>>;
    fn read_hpi_by_query(&self, query: &HpiQuery) -> Result<Hpis, Box<dyn Error>>; // TODO
}

impl Hpi {
    pub async fn create(&self, client: &dyn Persist) -> Result<(String, i32), Box<dyn Error>> {
        client.create_hpi(self).await
    }

    pub async fn read(client: &dyn Persist, id: (&str, i32)) -> Result<Hpi, Box<dyn Error>> {
        client.read_hpi_by_id(id).await
    }

    pub async fn update(&self, client: &dyn Persist) -> Result<(), Box<dyn Error>> {
        client.update_hpi(self).await
    }

    pub async fn delete(client: &dyn Persist, id: (&str, i32)) -> Result<(), Box<dyn Error>> {
        client.delete_hpi_by_id(id).await
    }

    pub fn read_by_query(client: &dyn Persist, query: &HpiQuery) -> Result<Hpis, Box<dyn Error>> {
        client.read_hpi_by_query(query)
    }
}

// TODO:
// impl From<Entry> for RegionHPI
// Unit tests

pub struct HpiConfig {
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

    fn has_three_zip_hpi_path(&self) -> bool {
        self.three_zip_hpis_path.is_some()
    }

    fn has_five_zip_hpi_path(&self) -> bool {
        self.five_zip_hpis_path.is_some()
    }

    fn has_county_hpi_path(&self) -> bool {
        self.county_hpis_path.is_some()
    }

    fn three_zip_hpi_path(&self) -> &str {
        self.three_zip_hpis_path.as_ref().unwrap()
    }
}

pub fn read_fhfa_hpis(hpi_config: &HpiConfig) -> Result<HpiData, Box<dyn Error>> {
    let mut hpi_data = HpiData::default();
    if hpi_config.has_three_zip_hpi_path() {
        hpi_data.three_zip_hpis = read_three_zip_fhfa_hpis(hpi_config.three_zip_hpi_path())?;
    }
    if hpi_config.has_five_zip_hpi_path() {
        hpi_data.five_zip_hpis = read_five_zip_fhfa_hpis()?;
    }
    if hpi_config.has_county_hpi_path() {
        hpi_data.county_hpis = read_county_fhfa_hpis()?;
    }
    Ok(hpi_data)
}

fn read_three_zip_fhfa_hpis(three_zip_path: &str) -> Result<Hpis, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(three_zip_path)?;

    // TODO: rdr.deserialize().into_iter()?.into().collect();
    let mut entries = vec![];
    for result in rdr.deserialize() {
        let r: CsvRecord = result?;
        entries.push(r);
    }
    Ok(entries
        .into_iter()
        .map(|entry| {
            let region = entry.0[0].clone();
            let year = entry.0[1].parse().unwrap();
            let annual_change = entry.0[2].parse().ok();
            let hpi = entry.0[3].parse().ok();
            let hpi_1990_base = entry.0[4].parse().ok();
            let hpi_2000_base = entry.0[5].parse().ok();
            Hpi {
                region,
                year,
                annual_change,
                hpi,
                hpi_1990_base,
                hpi_2000_base,
            }
        })
        .collect())
}

fn read_five_zip_fhfa_hpis() -> Result<Hpis, Box<dyn Error>> {
    let three_zip_hpi = "datasets/fhfa-hpi/HPI_AT_BDL_ZIP5.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(three_zip_hpi)?;

    // TODO: rdr.deserialize().into_iter()?.into().collect();
    let mut entries = vec![];
    for result in rdr.deserialize() {
        let r: CsvRecord = result?;
        entries.push(r);
    }

    Ok(entries
        .into_iter()
        .map(|entry| {
            let region = entry.0[0].clone();
            let year = entry.0[1].parse().unwrap();
            let annual_change = entry.0[2].parse().ok();
            let hpi = entry.0[3].parse().ok();
            let hpi_1990_base = entry.0[4].parse().ok();
            let hpi_2000_base = entry.0[5].parse().ok();
            Hpi {
                region,
                year,
                annual_change,
                hpi,
                hpi_1990_base,
                hpi_2000_base,
            }
        })
        .collect())
}

fn read_county_fhfa_hpis() -> Result<Hpis, Box<dyn Error>> {
    let three_zip_hpi = "datasets/fhfa-hpi/HPI_AT_BDL_county.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(three_zip_hpi)?;

    // TODO: rdr.deserialize().into_iter()?.into().collect();
    let mut entries = vec![];
    for result in rdr.deserialize() {
        let r: CsvRecord = result?;
        entries.push(r);
    }

    Ok(entries
        .into_iter()
        .map(|entry| {
            let region = entry.0[1].clone();
            let year = entry.0[3].parse().unwrap();
            let annual_change = entry.0[4].parse().ok();
            let hpi = entry.0[5].parse().ok();
            let hpi_1990_base = entry.0[6].parse().ok();
            let hpi_2000_base = entry.0[7].parse().ok();
            Hpi {
                region,
                year,
                annual_change,
                hpi,
                hpi_1990_base,
                hpi_2000_base,
            }
        })
        .collect())
}
