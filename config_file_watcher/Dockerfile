# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/config_file_watcher
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/config_file_watcher/target/release/config_file_watcher /usr/local/bin/app
CMD ["app"]