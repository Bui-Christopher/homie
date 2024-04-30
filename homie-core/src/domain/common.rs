use crate::error::Error;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum DateInterval {
    Day,
    Month,
    #[default]
    Year,
}

impl TryFrom<&str> for DateInterval {
    type Error = crate::error::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "day" => Ok(DateInterval::Day),
            "month" => Ok(DateInterval::Month),
            "year" => Ok(DateInterval::Year),
            _ => Err(Error::Parse("Failed to parse DateInterval".to_string())),
        }
    }
}
