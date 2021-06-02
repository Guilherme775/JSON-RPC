use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize, Debug)]
struct Response {
    jsonrpc: String,
    result: u64,
    id: u64,
}

fn main() {
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getMinimumBalanceForRentExemption",
        "params": [0]
    });

    let request_url = "https://api.mainnet-beta.solana.com";

    let response = jsonrpc(body, request_url);

    println!("{:?}", response);
}

#[tokio::main]
async fn jsonrpc(body: Value, url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let req = Client::new()
        .post(url)
        .json(&body)
        .send().await?;

    let res: Response = req.json().await?;

    Ok(res)
}
