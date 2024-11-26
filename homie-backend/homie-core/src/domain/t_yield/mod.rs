#[cfg(feature = "db")]
pub mod local;
#[cfg(feature = "db")]
pub mod persist;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(feature = "db", sqlx(type_name = "term", rename_all = "lowercase"))]
pub enum Term {
    #[default]
    TenYear,
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::TenYear => write!(f, "ten_year"),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "db", derive(sqlx::FromRow))]
pub struct TYield {
    pub(crate) term: Term,
    pub(crate) date: NaiveDate,
    pub(crate) yield_return: Option<f32>,
}

#[cfg(feature = "db")]
impl TYield {
    pub(crate) fn term(&self) -> &Term {
        &self.term
    }

    pub(crate) fn date(&self) -> &NaiveDate {
        &self.date
    }

    pub(crate) fn yield_return(&self) -> &Option<f32> {
        &self.yield_return
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TYieldData {
    ten_year_yields: TYields,
}

impl TYieldData {
    pub fn ten_year_yields(&self) -> &TYields {
        &self.ten_year_yields
    }
}

pub type TYields = Vec<TYield>;
