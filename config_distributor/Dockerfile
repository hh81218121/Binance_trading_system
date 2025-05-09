# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/config_distributor
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/config_distributor/target/release/config_distributor /usr/local/bin/app
CMD ["app"]