# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/config_api_server
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/config_api_server/target/release/config_api_server /usr/local/bin/app
CMD ["app"]