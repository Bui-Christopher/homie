#![deny(clippy::all)]

use std::error::Error;
use std::sync::OnceLock;

use homie_core::adapter::importer::Importer;
use homie_core::adapter::repository::{Config, Repository};
use homie_core::domain::t_yield::TYield;

static CONFIG: OnceLock<Config> = OnceLock::new();

fn main() -> Result<(), Box<dyn Error>> {
    let config = CONFIG.get_or_init(Config::load_config);

    let _reader = Importer::new(config);
    _reader.read_and_import_datasets()?;
    let t_yield = TYield::default();

    let postgres_writer = Repository::new(config);
    t_yield.read(postgres_writer.session(), "key")?;

    let file_writer = Repository::new(config);
    t_yield.read(file_writer.session(), "key")?;

    Ok(())
}
