# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/oms_pretrade_risk_checker
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/oms_pretrade_risk_checker/target/release/oms_pretrade_risk_checker /usr/local/bin/app
CMD ["app"]