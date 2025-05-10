use reqwest::Client;  // 使用非同步的 reqwest::Client
use std::error::Error;
use crate::model::{ApiResponse, Asset};

pub async fn fetch_assets(api_key: &str, exchange_id: u32) -> Result<Vec<Asset>, Box<dyn Error>> {
    let url = format!("https://pro-api.coinmarketcap.com/v1/exchange/assets?CMC_PRO_API_KEY={}&id={}", api_key, exchange_id);
    let client = Client::new();
    let response = client.get(&url).send().await?.json::<ApiResponse>().await?;
    Ok(response.data)
}
