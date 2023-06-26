extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct SymbolData {
    symbol: String,
    name: String,
    exchange: String,
    country: String,
    currency: String,
}

fn main() {
    // Make sure to replace YOUR_API_KEY with your actual API key
    let api_key = "649954668f1d54.62736115";
    let symbol = "AAPL";

    // Build the API URL
    let url = format!(
        "https://eodhistoricaldata.com/api/symbols/{symbol}?api_token={api_key}",
        symbol = symbol,
        api_key = api_key
    );

    // Send the HTTP GET request
    let mut response = reqwest::get(&url).unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    // Deserialize the JSON response
    let symbol_data: Result<SymbolData, _> = serde_json::from_str(&body);

    match symbol_data {
        Ok(data) => println!("{:#?}", data),
        Err(err) => eprintln!("Failed to parse symbol data: {}", err),
    }
}