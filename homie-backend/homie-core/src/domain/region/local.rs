use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::domain::region::{Region, RegionData};
use crate::domain::util::CsvRecord;
use crate::error::DomainError;

#[cfg(feature = "db")]
#[derive(Clone, Debug, Default)]
pub struct RegionConfig {
    cities_path: Option<String>,
    zip_county_path: Option<String>,
}

#[cfg(feature = "db")]
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

#[cfg(feature = "db")]
pub fn read_huduser_regions(region_config: &RegionConfig) -> Result<RegionData, DomainError> {
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

#[cfg(feature = "db")]
fn read_select_cities(cities_path: &str) -> Vec<String> {
    let mut cities: Vec<String> = Vec::new();
    if let Ok(file) = File::open(cities_path) {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            cities.push(line.to_lowercase());
        }
    }
    cities
}

#[cfg(feature = "db")]
fn read_csv_city_zip_pairs(zip_county_path: &str) -> Result<Vec<(String, String)>, DomainError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(zip_county_path)?;
    let mut pairs = vec![];
    let entries: Vec<CsvRecord> = rdr.deserialize().filter_map(Result::ok).collect();
    for entry in entries.into_iter() {
        if entry.0[3] == "CA" {
            let zipcode = entry.0[0].clone();
            let city = entry.0[2].to_lowercase().clone();
            pairs.push((city, zipcode));
        }
    }
    Ok(pairs)
}
