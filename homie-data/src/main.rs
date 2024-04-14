use std::error::Error;

use crate::config::Config;
use crate::hpi::read_fhfa_hpis;
use crate::model::*;
use crate::reader::Reader;
use crate::region::read_huduser_regions;
use crate::t_yield::read_fed_yields;
use crate::writer::Writer;
use crate::zhvi::read_zillow_zhvis;

mod config;
mod database;
mod model;
mod reader;
mod writer;

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Switch to builder pattern
    let config = Config::new();
    let reader = Reader::new(&config);
    let writer = Writer::new(&config);
    let datasets = reader.read_datasets()?;
    writer.write_datasets(&datasets)?;

    // TODO: Delete (testing)
    println!("{:#?}", datasets);

    Ok(())
}
