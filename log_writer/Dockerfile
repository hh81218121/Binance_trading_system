# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/log_writer
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/log_writer/target/release/log_writer /usr/local/bin/app
CMD ["app"]