#![deny(clippy::all)]

use std::error::Error;
use std::sync::OnceLock;

use homie_core::adapter::config::Config;
use homie_core::adapter::importer::Importer;
use homie_core::adapter::repository::Repository;

use crate::util::read_and_write_datasets;

static CONFIG: OnceLock<Config> = OnceLock::new();

mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = CONFIG.get_or_init(Config::load_config);
    let importer = Importer::new(config);
    let repository = Repository::new(config).await?;

    read_and_write_datasets(&importer, &repository).await?;

    Ok(())
}
