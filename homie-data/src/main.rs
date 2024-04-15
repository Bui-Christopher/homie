#![deny(clippy::all)]
use std::error::Error;
use std::sync::OnceLock;

use database::file::FileStorage;
use database::postgres::Postgres;
use repository::reader::Reader;
use repository::writer::Writer;

use crate::config::Config;

mod config;
mod database;
mod model;
mod repository;

static CONFIG: OnceLock<Config> = OnceLock::new();

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Switch to builder pattern
    // TODO: Rename to DataImporter/DatasetImporter/DatasetReader
    let reader = Reader::new(CONFIG.get_or_init(Config::load_config));
    let datasets = reader.read_datasets()?;

    // TODO: Remove (testing)
    // let _reader = Reader::new(CONFIG.get_or_init(Config::load_config));
    // let datasets = model::common::Datasets::default();

    let postgres = Postgres::new();
    let postgres_writer = Writer::new(postgres);
    postgres_writer.write_datasets(&datasets)?;

    let file_storage = FileStorage::new();
    let file_writer = Writer::new(file_storage);
    file_writer.write_datasets(&datasets)?;

    Ok(())
}
