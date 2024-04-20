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
    datasets.read(postgres_writer.session(), "key")?;

    let file_writer = Repository::new(config);
    datasets.read(file_writer.session(), "key")?;

    Ok(())
}
