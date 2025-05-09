use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let port = env::var("APP_PORT").unwrap_or_else(|_| "8080".to_string());
    let grpc_port = env::var("GRPC_PORT").unwrap_or_else(|_| "50051".to_string());
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());

    info!("Service `alert_engine` starting...");
    info!("Listening on port {} | gRPC on {} | NATS at {}", port, grpc_port, nats_url);

    // TODO: Replace with actual NATS/gRPC/metrics logic

    Ok(())
}