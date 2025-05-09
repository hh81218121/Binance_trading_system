# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/conn_socket_tuner
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/conn_socket_tuner/target/release/conn_socket_tuner /usr/local/bin/app
CMD ["app"]