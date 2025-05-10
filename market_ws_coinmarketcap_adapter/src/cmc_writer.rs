use std::error::Error;
use mongodb::{Client as MongoClient, bson::doc};
use mongodb::options::ClientOptions;
use chrono::Utc;
use crate::model::Asset;

pub async fn persist_to_mongodb(assets: &[Asset]) -> Result<(), Box<dyn Error>> {
    // 解析 MongoDB 客戶端的配置
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = MongoClient::with_options(client_options)?;

    // 取得目標集合
    let collection = client.database("exchange").collection("assets");

    // 轉換每個資產為 MongoDB 的文檔
    let docs: Vec<_> = assets.iter().map(|asset| {
        let now = Utc::now().timestamp_nanos_opt().unwrap_or_else(|| {
            // 使用當前時間戳作為備選方案，如果發生溢位
            Utc::now().timestamp_nanos()
        }).to_string();

        doc! {
            "wallet_address": &asset.wallet_address,
            "balance": asset.balance,
            "platform": {
                "crypto_id": asset.platform.crypto_id,
                "symbol": &asset.platform.symbol,
                "name": &asset.platform.name,
            },
            "currency": {
                "crypto_id": asset.currency.crypto_id,
                "price_usd": asset.currency.price_usd,
                "symbol": &asset.currency.symbol,
                "name": &asset.currency.name,
            },
            "exchange": "binance",
            "timestamp": now,
        }
    }).collect();

    // 插入資料到 MongoDB，處理插入過程中的錯誤
    match collection.insert_many(docs, None).await {
        Ok(_) => Ok(()),
        Err(err) => {
            // 這裡可以根據需要做更多錯誤處理
            Err(Box::new(err))
        }
    }
}
