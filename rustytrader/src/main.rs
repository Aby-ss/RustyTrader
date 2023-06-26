use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct SymbolData {
    symbol: String,
    name: String,
    exchange: String,
    country: String,
    currency: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let response = reqwest::get(&url).await?.text().await?;

    // Deserialize the JSON response
    let symbol_data: SymbolData = serde_json::from_str(&response)?;

    // Print the symbol data
    println!("{:#?}", symbol_data);

    Ok(())
}