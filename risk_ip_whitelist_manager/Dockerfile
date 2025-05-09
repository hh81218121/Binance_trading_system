# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/risk_ip_whitelist_manager
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/risk_ip_whitelist_manager/target/release/risk_ip_whitelist_manager /usr/local/bin/app
CMD ["app"]