use std::error::Error;

use homie_core::domain::zhvi::Zhvis;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let zhvis = read_zestimates().await?;
    let prices = zhvis.first().unwrap().prices();
    let mut x_axis = vec![];
    let mut y_axis = vec![];

    for price in prices {
        x_axis.push(price.date);
        y_axis.push(price.value);
    }
    // TODO: Create graph from axes
    // TODO: Convert to WASM binary

    // Print the JSON to the console
    println!("X-Axis:{:?}", x_axis);
    println!("Y-Axis:{:?}", y_axis);

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
