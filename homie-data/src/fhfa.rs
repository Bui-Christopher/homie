use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::Entry;

#[derive(Debug, Serialize, Deserialize)]
struct ZipcodeHPI {
    zip: String,
    year: u32,
    annual_change: Option<f32>,
    hpi: Option<f32>,
    hpi_1990_base: Option<f32>,
    hpi_2000_base: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CountyHPI {
    state: String,
    county: String,
    fips_code: String,
    year: u32,
    annual_change: Option<f32>,
    hpi: Option<f32>,
    hpi_1990_base: Option<f32>,
    hpi_2000_base: Option<f32>,
}
// TODO: From<Entry> for ZipHPI
// TODO: From<Entry> for CountyHPI

type ZipcodeHPIs = Vec<ZipcodeHPI>;
type CountiesHPI = Vec<CountyHPI>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HPIData {
    three_zip_hpis: ZipcodeHPIs,
    five_zip_hpis: ZipcodeHPIs,
    county_hpis: CountiesHPI,
}

pub fn read_fhfa_hpis() -> Result<HPIData, Box<dyn Error>> {
    let three_zip_hpis = read_three_zip_fhfa_hpis()?;
    let five_zip_hpis = read_five_zip_fhfa_hpis()?;
    let county_hpis = read_county_fhfa_hpis()?;
    Ok(HPIData {
        three_zip_hpis,
        five_zip_hpis,
        county_hpis,
    })
}

fn read_three_zip_fhfa_hpis() -> Result<Vec<ZipcodeHPI>, Box<dyn Error>> {
    let three_zip_hpi = "datasets/fhfa-hpi/HPI_AT_BDL_ZIP3.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(three_zip_hpi)?;

    // TODO: rdr.deserialize().into_iter()?.into().collect();
    let mut entries = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        entries.push(r);
    }
    Ok(entries
        .into_iter()
        .map(|entry| {
            let zip = entry.0[0].clone();
            let year = entry.0[1].parse().unwrap();
            let annual_change = entry.0[2].parse().ok();
            let hpi = entry.0[3].parse().ok();
            let hpi_1990_base = entry.0[4].parse().ok();
            let hpi_2000_base = entry.0[5].parse().ok();
            ZipcodeHPI {
                zip,
                year,
                annual_change,
                hpi,
                hpi_1990_base,
                hpi_2000_base,
            }
        })
        .collect())
}

fn read_five_zip_fhfa_hpis() -> Result<Vec<ZipcodeHPI>, Box<dyn Error>> {
    let three_zip_hpi = "datasets/fhfa-hpi/HPI_AT_BDL_ZIP5.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(three_zip_hpi)?;

    // TODO: rdr.deserialize().into_iter()?.into().collect();
    let mut entries = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        entries.push(r);
    }

    Ok(entries
        .into_iter()
        .map(|entry| {
            let zip = entry.0[0].clone();
            let year = entry.0[1].parse().unwrap();
            let annual_change = entry.0[2].parse().ok();
            let hpi = entry.0[3].parse().ok();
            let hpi_1990_base = entry.0[4].parse().ok();
            let hpi_2000_base = entry.0[5].parse().ok();
            ZipcodeHPI {
                zip,
                year,
                annual_change,
                hpi,
                hpi_1990_base,
                hpi_2000_base,
            }
        })
        .collect())
}

fn read_county_fhfa_hpis() -> Result<Vec<CountyHPI>, Box<dyn Error>> {
    let three_zip_hpi = "datasets/fhfa-hpi/HPI_AT_BDL_county.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(three_zip_hpi)?;

    // TODO: rdr.deserialize().into_iter()?.into().collect();
    let mut entries = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        entries.push(r);
    }

    Ok(entries
        .into_iter()
        .map(|entry| {
            let state = entry.0[0].clone();
            let county = entry.0[1].clone();
            let fips_code = entry.0[2].clone();
            let year = entry.0[3].parse().unwrap();
            let annual_change = entry.0[4].parse().ok();
            let hpi = entry.0[5].parse().ok();
            let hpi_1990_base = entry.0[6].parse().ok();
            let hpi_2000_base = entry.0[7].parse().ok();
            CountyHPI {
                state,
                county,
                fips_code,
                year,
                annual_change,
                hpi,
                hpi_1990_base,
                hpi_2000_base,
            }
        })
        .collect())
}
