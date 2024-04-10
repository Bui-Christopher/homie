use std::error::Error;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Entry(Vec<String>);

#[derive(Debug, Serialize, Deserialize)]
struct TreasuryYield {
    date: NaiveDate,
    yield_return: f64,
}

fn to_ymd_date(year: i32, month: u32, day: Option<u32>) -> Result<NaiveDate, Box<dyn Error>> {
    // Some datasets are an average of the month
    // Thus, default to the fiftheenth
    let day = day.unwrap_or(15);
    NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| "Invalid date".into())
}

fn main() -> Result<(), Box<dyn Error>> {
    // let zillow_entries = read_zillow()?;
    // write_zillow();

    // let hpi_entries = read_fhfa_hpi()?;
    // write_fhfa_hpi();

    // let zipcode_entries = read_huduser_crosswalk()?;
    // write_huduser_crosswalk();

    let treasury_yields = read_fed_h15()?;
    // write_fed_h15();

    // let entries = zillow_entries;
    // let entries = hpi_entries;
    // let entries = zipcode_entries;
    let entries = treasury_yields;
    println!("{:#?}", entries);

    Ok(())
}

// let _date = NaiveDate::from_ymd_opt(2014, 1, 1).unwrap();

fn read_zillow() -> Result<Vec<Entry>, Box<dyn Error>> {
    let zillow_mid_all_city = "datasets/zillow-zhvi/all-homes/mid-tier/\
                               City_zhvi_uc_sfrcondo_tier_0.33_0.67_sm_sa_month.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(zillow_mid_all_city)?;

    let mut entries = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        entries.push(r);
    }
    Ok(entries)
}

fn read_fhfa_hpi() -> Result<Vec<Entry>, Box<dyn Error>> {
    let zillow_mid_all_city = "datasets/fed-h15/FRB_H15.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(zillow_mid_all_city)?;

    let mut entries = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        entries.push(r);
    }
    Ok(entries)
}

fn read_huduser_crosswalk() -> Result<Vec<Entry>, Box<dyn Error>> {
    let huduser_crosswalk = "datasets/huduser-crosswalk/COUNTY_ZIP_122023.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(huduser_crosswalk)?;

    let mut entries = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        entries.push(r);
    }
    Ok(entries)
}

fn read_fed_h15() -> Result<Vec<TreasuryYield>, Box<dyn Error>> {
    let fed_h15 = "datasets/fed-h15/FRB_H15.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(fed_h15)?;

    let mut entries = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        entries.push(r);
    }
    Ok(entries
        .into_iter()
        .map(|entry| {
            let parts: Vec<&str> = entry.0[0].split('-').collect();
            let year = parts[0].parse().unwrap();
            let month = parts[1].parse().unwrap();
            let date = to_ymd_date(year, month, None).unwrap();
            let yield_return = entry.0[1].parse().unwrap();
            TreasuryYield { date, yield_return }
        })
        .collect())
}
