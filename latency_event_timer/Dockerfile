# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/latency_event_timer
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/latency_event_timer/target/release/latency_event_timer /usr/local/bin/app
CMD ["app"]