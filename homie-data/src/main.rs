use std::error::Error;

use database::postgres::Postgres;

use crate::config::Config;
use crate::model::common::Datasets;
use crate::reader::Reader;
use crate::writer::Writer;

mod config;
mod database;
mod model;
mod reader;
mod writer;

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Does this need to be global?
    let config = Config::new();

    // TODO: Abstract this
    let postgres = Postgres::new();

    // TODO: Switch to builder pattern
    let reader = Reader::new(&config);
    let postgres_writer: Writer<_, Datasets> = Writer::new(postgres);

    let datasets = reader.read_datasets()?;
    postgres_writer.write_datasets(&datasets)?;

    Ok(())
}
