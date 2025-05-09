# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/strategy_signal_dispatcher
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/strategy_signal_dispatcher/target/release/strategy_signal_dispatcher /usr/local/bin/app
CMD ["app"]