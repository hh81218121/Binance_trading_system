use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Platform {
    pub crypto_id: u32,
    pub symbol: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Currency {
    pub crypto_id: u32,
    pub price_usd: f64,
    pub symbol: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Asset {
    pub wallet_address: String,
    pub balance: f64,
    pub platform: Platform,
    pub currency: Currency,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub data: Vec<Asset>,
}
