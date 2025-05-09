# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/latency_collector
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/latency_collector/target/release/latency_collector /usr/local/bin/app
CMD ["app"]