#![deny(clippy::all)]
use std::error::Error;

use database::file::FileStorage;
use database::postgres::Postgres;

use crate::config::Config;
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

    // TODO: Switch to builder pattern
    // TODO: Rename to DataImporter/DatasetImporter/DatasetReader
    let reader = Reader::new(&config);
    let datasets = reader.read_datasets()?;

    // TODO: Remove (testing)
    // let _reader = Reader::new(&config);
    // let datasets = model::common::Datasets::default();

    let postgres = Postgres::new();
    let postgres_writer = Writer::new(postgres);
    postgres_writer.write_datasets(&datasets)?;

    let file_storage = FileStorage::new();
    let file_writer = Writer::new(file_storage);
    file_writer.write_datasets(&datasets)?;

    Ok(())
}
