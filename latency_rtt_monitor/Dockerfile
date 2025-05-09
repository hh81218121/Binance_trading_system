# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/latency_rtt_monitor
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/latency_rtt_monitor/target/release/latency_rtt_monitor /usr/local/bin/app
CMD ["app"]