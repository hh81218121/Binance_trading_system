# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/conn_binance_monitor
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/conn_binance_monitor/target/release/conn_binance_monitor /usr/local/bin/app
CMD ["app"]