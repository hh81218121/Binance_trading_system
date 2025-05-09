# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/conn_failover_router
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/conn_failover_router/target/release/conn_failover_router /usr/local/bin/app
CMD ["app"]