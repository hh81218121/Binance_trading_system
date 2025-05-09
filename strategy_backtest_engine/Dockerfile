# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/strategy_backtest_engine
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/strategy_backtest_engine/target/release/strategy_backtest_engine /usr/local/bin/app
CMD ["app"]