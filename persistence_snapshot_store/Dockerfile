# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/persistence_snapshot_store
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/persistence_snapshot_store/target/release/persistence_snapshot_store /usr/local/bin/app
CMD ["app"]