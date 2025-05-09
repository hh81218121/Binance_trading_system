# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/persistence_trade_logger
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/persistence_trade_logger/target/release/persistence_trade_logger /usr/local/bin/app
CMD ["app"]