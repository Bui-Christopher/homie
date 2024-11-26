use crate::domain::util::{to_ymd_date, CsvRecord};
use crate::domain::zhvi::{HomeType, Percentile, RegionType, Zhvi, ZhviData, ZhviPrice, Zhvis};
use crate::error::DomainError;

#[derive(Clone, Debug, Default)]
pub(crate) struct ZhviConfig {
    bot_city_all_homes_path: Option<String>,
    mid_zip_all_homes_path: Option<String>,
    mid_city_all_homes_path: Option<String>,
    mid_county_all_homes_path: Option<String>,
}

#[cfg(feature = "db")]
impl ZhviConfig {
    pub fn new(
        bot_city_all_homes_path: Option<String>,
        mid_zip_all_homes_path: Option<String>,
        mid_city_all_homes_path: Option<String>,
        mid_county_all_homes_path: Option<String>,
    ) -> Self {
        ZhviConfig {
            bot_city_all_homes_path,
            mid_zip_all_homes_path,
            mid_city_all_homes_path,
            mid_county_all_homes_path,
        }
    }

    fn bot_city_all_homes_path(&self) -> Option<&str> {
        self.bot_city_all_homes_path.as_deref()
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

#[cfg(feature = "db")]
pub(crate) fn read_zillow_zhvis(zhvi_config: &ZhviConfig) -> Result<ZhviData, DomainError> {
    let zhvi_data = ZhviData {
        all_homes_zhvis: read_all_homes_zhvis(zhvi_config)?,
        // condo_coops_zhvis = read_condo_coops_zhvis(zhvi_config)?;
        // single_family_homes_zhvis = read_single_family_homes_zhvis(zhvi_config)?;
        ..Default::default()
    };

    Ok(zhvi_data)
}

#[cfg(feature = "db")]
fn read_all_homes_zhvis(zhvi_config: &ZhviConfig) -> Result<Zhvis, DomainError> {
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

    if let Some(bot_city_all_homes_path) = zhvi_config.bot_city_all_homes_path() {
        all_homes.append(&mut read_bot_city_all_homes(bot_city_all_homes_path)?);
    }
    Ok(all_homes)
}

#[cfg(feature = "db")]
fn read_mid_city_all_homes(mid_city_all_homes_path: &str) -> Result<Zhvis, DomainError> {
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
                .ok_or(DomainError::Parse(
                    "Failed to parse string to date".to_string(),
                ))?
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

#[cfg(feature = "db")]
fn read_mid_county_all_homes(mid_county_all_homes_path: &str) -> Result<Zhvis, DomainError> {
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
                .ok_or(DomainError::Parse(
                    "Failed to parse string to date".to_string(),
                ))?
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

#[cfg(feature = "db")]
fn read_mid_zip_all_homes(mid_zip_all_homes_path: &str) -> Result<Zhvis, DomainError> {
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
                .ok_or(DomainError::Parse(
                    "Failed to parse string to date".to_string(),
                ))?
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

#[cfg(feature = "db")]
fn read_bot_city_all_homes(bot_city_all_homes_path: &str) -> Result<Zhvis, DomainError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(bot_city_all_homes_path)?;

    let mut bot_all_homes = vec![];
    let entries: Vec<CsvRecord> = rdr.deserialize().filter_map(Result::ok).collect();
    let headers = rdr.headers()?;
    for entry in entries.into_iter() {
        let mut prices = vec![];
        // start at 8
        for i in 9..entry.0.len() {
            let parts: Vec<&str> = headers
                .iter()
                .nth(i)
                .ok_or(DomainError::Parse(
                    "Failed to parse string to date".to_string(),
                ))?
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
        let percentile = Percentile::Bottom;
        bot_all_homes.push(Zhvi {
            home_type,
            region_type,
            region_name,
            percentile,
            prices,
        });
    }

    Ok(bot_all_homes)
}
