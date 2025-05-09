# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/oms_retry_engine
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/oms_retry_engine/target/release/oms_retry_engine /usr/local/bin/app
CMD ["app"]