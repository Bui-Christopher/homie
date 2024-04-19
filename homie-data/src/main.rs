#![deny(clippy::all)]

use std::error::Error;
use std::sync::OnceLock;

use homie_core::adapter::importer::Importer;
use homie_core::adapter::repository::{Config, Repository};
// use homie_core::domain::common::Datasets;

static CONFIG: OnceLock<Config> = OnceLock::new();

fn main() -> Result<(), Box<dyn Error>> {
    let config = CONFIG.get_or_init(Config::load_config);

    let _reader = Importer::new(config);
    let datasets = _reader.read_datasets()?;
    // let datasets = Datasets::default();

    let postgres_writer = Repository::new(config);
    postgres_writer.call_all_crud(&datasets)?;

    let file_writer = Repository::new(config);
    file_writer.call_all_crud(&datasets)?;

    Ok(())
}
