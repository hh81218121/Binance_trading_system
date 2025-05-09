# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/oms_core
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/oms_core/target/release/oms_core /usr/local/bin/app
CMD ["app"]