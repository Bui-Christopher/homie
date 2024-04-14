use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::model::common::Entry;

#[derive(Debug, Serialize, Deserialize)]
pub enum RegionHPI {
    CountyHPI {
        county: String,
        year: u32,
        hpi: Option<f32>,
        annual_change: Option<f32>,
        hpi_1990_base: Option<f32>,
        hpi_2000_base: Option<f32>,
    },
    ZipcodeHPI {
        zip: String,
        year: u32,
        hpi: Option<f32>,
        annual_change: Option<f32>,
        hpi_1990_base: Option<f32>,
        hpi_2000_base: Option<f32>,
    },
}

type RegionHPIs = Vec<RegionHPI>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HPIData {
    three_zip_hpis: RegionHPIs,
    five_zip_hpis: RegionHPIs,
    county_hpis: RegionHPIs,
}

// TODO:
// impl From<Entry> for ZipHPI
// impl From<Entry> for CountyHPI
// Unit tests

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

fn read_three_zip_fhfa_hpis() -> Result<RegionHPIs, Box<dyn Error>> {
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
            RegionHPI::ZipcodeHPI {
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

fn read_five_zip_fhfa_hpis() -> Result<RegionHPIs, Box<dyn Error>> {
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
            RegionHPI::ZipcodeHPI {
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

fn read_county_fhfa_hpis() -> Result<RegionHPIs, Box<dyn Error>> {
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
            let county = entry.0[1].clone();
            let year = entry.0[3].parse().unwrap();
            let annual_change = entry.0[4].parse().ok();
            let hpi = entry.0[5].parse().ok();
            let hpi_1990_base = entry.0[6].parse().ok();
            let hpi_2000_base = entry.0[7].parse().ok();
            RegionHPI::CountyHPI {
                county,
                year,
                annual_change,
                hpi,
                hpi_1990_base,
                hpi_2000_base,
            }
        })
        .collect())
}
