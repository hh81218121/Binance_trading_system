# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/market_ws_binance_spot
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/market_ws_binance_spot/target/release/market_ws_binance_spot /usr/local/bin/app
CMD ["app"]