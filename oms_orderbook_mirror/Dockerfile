# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/oms_orderbook_mirror
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/oms_orderbook_mirror/target/release/oms_orderbook_mirror /usr/local/bin/app
CMD ["app"]