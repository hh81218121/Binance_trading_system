# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/metrics_exporter
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/metrics_exporter/target/release/metrics_exporter /usr/local/bin/app
CMD ["app"]