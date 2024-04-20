use crate::adapter::repository::Persist;
use crate::domain::t_yield::TYieldPersist;

pub struct HttpClient;

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

impl TYieldPersist for HttpClient {
    fn read_t_yield_by_id(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling t_yield read with id: {id} from HttpClient.");
        Ok(true)
    }
}

impl Persist for HttpClient {}
