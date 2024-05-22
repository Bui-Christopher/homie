use std::error::Error;

use chrono::NaiveDate;
use homie_core::domain::zhvi::Zhvis;
use plotly::{Plot, Scatter};
use reqwest::Client;

struct Line {
    dates: Vec<NaiveDate>,
    prices: Vec<f64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let zhvis = read_zestimates().await?;

    let mut lines = vec![];
    for zhvi in zhvis {
        let mut dates = vec![];
        let mut prices = vec![];
        for price in zhvi.prices() {
            dates.push(price.date);
            prices.push(price.value);
        }
        let line = Line { dates, prices };
        lines.push(line);
    }

    // Print the JSON to the console
    // Create graph from axes
    for line in lines {
        let mut plot = Plot::new();
        let trace = Scatter::new(line.dates, line.prices);
        plot.add_trace(trace);
        plot.show();
        // let name = "graph_name";
        // plot.use_local_plotly();
        // plot.write_html(format!("{name}.html"));
    }

    // TODO: Convert to WASM binary

    Ok(())
}

async fn read_zestimates() -> Result<Zhvis, Box<dyn Error>> {
    // Prepare the URL with query parameters
    let url = "http://127.0.0.1:8080/zhvis?start_date=2023-1-1&end_date=2024-12-31&date_interval=month&home_type=AllHomes&region_type=City&region_name=Irvine&percentile=Middle";

    // Make a GET request using reqwest
    let response = Client::new().get(url).send().await?;

    // Ensure the request was successful (status code 2xx)
    if !response.status().is_success() {
        return Err(format!("Request failed with status code: {}", response.status()).into());
    }

    // Deserialize the response JSON using serde_json
    let json_string = response.text().await?;
    let zhvis: Zhvis = serde_json::from_str(&json_string)?;
    Ok(zhvis)
}
