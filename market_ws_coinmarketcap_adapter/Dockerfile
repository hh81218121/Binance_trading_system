# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/market_ws_coinmarketcap_adapter
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/market_ws_coinmarketcap_adapter/target/release/market_ws_coinmarketcap_adapter /usr/local/bin/app
CMD ["app"]