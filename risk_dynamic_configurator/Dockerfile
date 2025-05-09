# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/risk_dynamic_configurator
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/risk_dynamic_configurator/target/release/risk_dynamic_configurator /usr/local/bin/app
CMD ["app"]