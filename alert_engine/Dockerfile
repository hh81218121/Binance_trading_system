# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/alert_engine
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/alert_engine/target/release/alert_engine /usr/local/bin/app
CMD ["app"]