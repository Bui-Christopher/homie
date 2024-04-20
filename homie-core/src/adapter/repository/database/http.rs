use super::common::Session;
use crate::domain::dataset::DatasetPersist;

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {}
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient::new()
    }
}

impl Session for HttpClient {}

impl DatasetPersist for HttpClient {
    fn read(&self, _key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling dataset read from HttpClient.");
        Ok(true)
    }
}
