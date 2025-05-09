# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/event_nats
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/event_nats/target/release/event_nats /usr/local/bin/app
CMD ["app"]