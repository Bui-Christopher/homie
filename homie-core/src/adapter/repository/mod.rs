use self::database::file::FileStorage;
use self::database::postgres::Postgres;
use crate::adapter::repository::database::common::Session;

pub mod database;

pub struct Repository {
    pub session: Box<dyn Session>,
}

impl Repository {
    pub fn new(config: &'static Config) -> Self {
        let session = establish_session(config);
        Repository { session }
    }

    pub fn session(&self) -> &dyn Session {
        &*self.session
    }
}
fn establish_session(config: &'static Config) -> Box<dyn Session> {
    if config.use_db {
        Box::new(Postgres::new())
    } else {
        Box::new(FileStorage::new())
    }
}

// TODO: Refactor out
pub struct Config {
    use_db: bool,
}

impl Config {
    pub fn load_config() -> Config {
        Config { use_db: false }
    }
}
