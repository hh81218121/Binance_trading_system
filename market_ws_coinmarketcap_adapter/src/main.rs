use std::error::Error;
use std::time::Duration;

use dotenv::dotenv;
use market_ws_coinmarketcap_adapter::cmc_fetcher::fetch_assets;
use market_ws_coinmarketcap_adapter::cmc_writer::persist_to_mongodb;
use tokio::time::{self, sleep};
use tracing::{error, info};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, Registry};

const MAX_RETRIES: u32 = 5;

fn init_logger() {
    std::fs::create_dir_all("logs").expect("Failed to create log directory");

    // ✅ 每日切檔 logs/app-YYYY-MM-DD.log
    let file_appender: RollingFileAppender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // 注意：_guard 需保持活著直到程式結束，否則可能遺失部份日誌

    let subscriber = Registry::default().with(
        fmt::Layer::new()
            .json()
            .with_writer(non_blocking)
            .with_target(true)
            .with_level(true)
            .with_file(true)
            .with_line_number(true)
            .with_thread_names(true)
            .with_thread_ids(true)
            .flatten_event(true), // 展開 fields 為頂層 JSON 屬性
    );

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global tracing subscriber");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    init_logger();

    let api_key = std::env::var("CMC_API_KEY")?;
    let exchange_id = 270;
    let mut interval = time::interval(Duration::from_secs(30));

    loop {
        interval.tick().await;
        let mut attempts = 0;

        loop {
            match fetch_assets(&api_key, exchange_id).await {
                Ok(assets) => match persist_to_mongodb(&assets).await {
                    Ok(_) => {
                        info!("Assets persisted to MongoDB.");
                        break;
                    }
                    Err(err) => {
                        attempts += 1;
                        error!(
                            "MongoDB insert failed: {} (attempt {}/{})",
                            err, attempts, MAX_RETRIES
                        );
                        if attempts >= MAX_RETRIES {
                            error!("Exceeded max retry attempts for MongoDB.");
                            break;
                        }
                    }
                },
                Err(err) => {
                    attempts += 1;
                    error!(
                        "Fetch from CoinMarketCap failed: {} (attempt {}/{})",
                        err, attempts, MAX_RETRIES
                    );
                    if attempts >= MAX_RETRIES {
                        error!("Exceeded max retry attempts for fetch.");
                        break;
                    }
                }
            }

            let backoff = Duration::from_secs(2u64.pow(attempts.min(5)) + rand::random::<u8>() as u64 % 3);
            info!("Backing off for {:?}...", backoff);
            sleep(backoff).await;
        }
    }
}
