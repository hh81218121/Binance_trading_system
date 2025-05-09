# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/market_feed_dispatcher
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/market_feed_dispatcher/target/release/market_feed_dispatcher /usr/local/bin/app
CMD ["app"]