use crate::domain::hpi::{Hpi, HpiData, Hpis, RegionType};
use crate::domain::util::CsvRecord;
use crate::error::DomainError;

#[cfg(feature = "db")]
#[derive(Clone, Debug)]
pub(crate) struct HpiConfig {
    three_zip_hpis_path: Option<String>,
    five_zip_hpis_path: Option<String>,
    county_hpis_path: Option<String>,
}

#[cfg(feature = "db")]
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

#[cfg(feature = "db")]
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

#[cfg(feature = "db")]
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

#[cfg(feature = "db")]
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

#[cfg(feature = "db")]
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
