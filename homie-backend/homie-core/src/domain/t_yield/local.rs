use crate::domain::t_yield::{TYield, TYieldData, TYields, Term};
use crate::domain::util::{to_ymd_date, CsvRecord};
use crate::error::DomainError;

#[cfg(feature = "db")]
#[derive(Clone, Debug, Default)]
pub(crate) struct TYieldConfig {
    ten_year_yield_path: Option<String>,
}

#[cfg(feature = "db")]
impl TYieldConfig {
    pub(crate) fn new(ten_year_yield_path: Option<String>) -> Self {
        TYieldConfig {
            ten_year_yield_path,
        }
    }

    fn ten_year_yield_path(&self) -> Option<&str> {
        self.ten_year_yield_path.as_deref()
    }
}

#[cfg(feature = "db")]
pub(crate) fn read_fed_yields(t_yield_config: &TYieldConfig) -> Result<TYieldData, DomainError> {
    let mut t_yield_data = TYieldData::default();
    if let Some(ten_year_yield_path) = t_yield_config.ten_year_yield_path() {
        t_yield_data.ten_year_yields = read_fed_ten_yields(ten_year_yield_path)?;
    }
    Ok(t_yield_data)
}

#[cfg(feature = "db")]
fn read_fed_ten_yields(fed_h15: &str) -> Result<TYields, DomainError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(fed_h15)?;

    let mut ten_year_yields = vec![];
    let entries: Vec<CsvRecord> = rdr.deserialize().filter_map(Result::ok).collect();

    for entry in entries.into_iter() {
        let parts: Vec<&str> = entry.0[0].split('-').collect();
        let year = parts[0].parse()?;
        let month = parts[1].parse()?;
        let term = Term::TenYear;
        let date = to_ymd_date(year, month, 1)?; // TODO: Random day here... Why?
        let yield_return = entry.0[1].parse().ok();
        ten_year_yields.push(TYield {
            term,
            date,
            yield_return,
        });
    }

    Ok(ten_year_yields)
}
