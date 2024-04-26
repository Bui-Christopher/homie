use chrono::NaiveDate;

#[derive(Debug)]
pub struct Zhvi {
    pub home_type: String, // AllHomes/CondoCoOps/SingleFamilyHomes
    pub prices: Prices,
    pub region_type: String, // Zipcode, City, County
    pub region_name: String,
    pub percentile: String, // Bottom, Middle, Top
}

#[derive(Debug)]
pub struct Price {
    date: NaiveDate,
    price: f64,
}

pub type Prices = Vec<Price>;
