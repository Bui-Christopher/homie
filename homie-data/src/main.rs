#![deny(clippy::all)]

use std::error::Error;
use std::sync::OnceLock;

use homie_core::adapter::importer::Importer;
use homie_core::adapter::repository::database::file::FileStorage;
use homie_core::adapter::repository::database::postgres::Postgres;
use homie_core::adapter::repository::{Config, Repository};

static CONFIG: OnceLock<Config> = OnceLock::new();

fn main() -> Result<(), Box<dyn Error>> {
    let config = CONFIG.get_or_init(Config::load_config);

    let reader = Importer::new(config);
    let datasets = reader.read_datasets()?;

    // TODO: Remove (testing)
    // let _reader = Reader::new(CONFIG.get_or_init(Config::load_config));
    // let datasets = model::common::Datasets::default();

    // TODO: Abstract the repo client by config
    let postgres = Postgres::new();
    let postgres_writer = Repository::new(config, postgres);
    postgres_writer.write_data(&datasets)?;

    let file_storage = FileStorage::new();
    let file_writer = Repository::new(config, file_storage);
    file_writer.write_data(&datasets)?;

    Ok(())
}
