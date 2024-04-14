use std::error::Error;

use database::file::FileStorage;

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
    let file_storage = FileStorage::new();

    // TODO: Switch to builder pattern
    let reader = Reader::new(&config);
    let writer: Writer<_, Datasets> = Writer::new(file_storage);

    let datasets = reader.read_datasets()?;
    writer.write_datasets(&datasets)?;

    Ok(())
}
