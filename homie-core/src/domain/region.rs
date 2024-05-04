use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::adapter::repository::Persist;
use crate::domain::util::CsvRecord;
use crate::error::Error;

pub type City = String;
pub type Zipcode = String;

#[derive(Clone, Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Region {
    pub(crate) city: City,
    pub(crate) zipcode: Zipcode,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RegionData {
    pub regions: Regions,
}

impl RegionData {
    pub fn regions(&self) -> &Regions {
        &self.regions
    }
}

pub type Regions = Vec<Region>;

#[async_trait]
pub trait RegionPersist: Send + Sync {
    async fn create_region(&self, region: &Region) -> Result<Zipcode, Error>;
    async fn read_region_by_id(&self, id: &str) -> Result<Region, Error>;
    async fn read_regions_by_city(&self, id: &str) -> Result<Regions, Error>;
    // async fn read_regions_by_query(&self, query: &RegionQuery) -> Result<Regions,
    // Error>; fn update_region(&self, region: &Region) -> Result<bool,
    // Error>;
    async fn delete_region_by_id(&self, id: &str) -> Result<Zipcode, Error>;
}

impl Region {
    pub async fn create(&self, client: &dyn Persist) -> Result<Zipcode, Error> {
        client.create_region(self).await
    }

    pub async fn read(client: &dyn Persist, id: &str) -> Result<Regions, Error> {
        client.read_regions_by_city(id).await
    }

    // pub async fn update(&self, client: &dyn Persist) -> Result<(), Error> {
    //     client.update_region(self).await
    // }
    //
    // pub async fn read_by_query(
    //     client: &dyn Persist,
    //     query: &RegionQuery,
    // ) -> Result<Regions, Error> {
    //     client.read_regions_by_query(query).await
    // }

    pub async fn delete(client: &dyn Persist, id: &str) -> Result<Zipcode, Error> {
        client.delete_region_by_id(id).await
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn zipcode(&self) -> &str {
        &self.zipcode
    }
}

#[derive(Clone, Debug, Default)]
pub struct RegionConfig {
    cities_path: Option<String>,
    zip_county_path: Option<String>,
}

impl RegionConfig {
    pub(crate) fn new(cities_path: Option<String>, zip_county_path: Option<String>) -> Self {
        RegionConfig {
            cities_path,
            zip_county_path,
        }
    }

    fn cities_path(&self) -> Option<&str> {
        self.cities_path.as_deref()
    }

    fn zip_county_path(&self) -> Option<&str> {
        self.zip_county_path.as_deref()
    }
}

pub fn read_huduser_regions(region_config: &RegionConfig) -> Result<RegionData, Error> {
    let mut region_data = RegionData::default();

    if let (Some(cities_path), Some(zip_county_path)) =
        (region_config.cities_path(), region_config.zip_county_path())
    {
        let cities = read_select_cities(cities_path);
        let cities_map: HashSet<_> = cities.iter().collect();

        let zip_city_pairs = read_csv_city_zip_pairs(zip_county_path)?;
        let mut regions = vec![];
        for pair in zip_city_pairs {
            if cities_map.contains(&pair.0) {
                regions.push(Region {
                    city: pair.0,
                    zipcode: pair.1,
                })
            }
        }
        region_data.regions = regions;
    }

    Ok(region_data)
}

fn read_select_cities(cities_path: &str) -> Vec<String> {
    let mut cities: Vec<String> = Vec::new();
    if let Ok(file) = File::open(cities_path) {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            cities.push(line.to_uppercase());
        }
    }
    cities
}

fn read_csv_city_zip_pairs(zip_county_path: &str) -> Result<Vec<(String, String)>, Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(zip_county_path)?;
    let mut pairs = vec![];
    let entries: Vec<CsvRecord> = rdr.deserialize().filter_map(Result::ok).collect();
    for entry in entries.into_iter() {
        let zipcode = entry.0[0].clone();
        let city = entry.0[2].clone();
        pairs.push((city, zipcode));
    }
    Ok(pairs)
}
