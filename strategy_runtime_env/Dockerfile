# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/strategy_runtime_env
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/strategy_runtime_env/target/release/strategy_runtime_env /usr/local/bin/app
CMD ["app"]