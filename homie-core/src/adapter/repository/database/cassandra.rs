pub struct CassandraClient {}

impl CassandraClient {
    pub fn new() -> Self {
        CassandraClient {}
    }
}

impl Default for CassandraClient {
    fn default() -> Self {
        CassandraClient::new()
    }
}
