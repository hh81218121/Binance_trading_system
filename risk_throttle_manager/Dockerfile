# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/risk_throttle_manager
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/risk_throttle_manager/target/release/risk_throttle_manager /usr/local/bin/app
CMD ["app"]