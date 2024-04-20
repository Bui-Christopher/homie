use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct CsvRecord(pub(super) Vec<String>);
