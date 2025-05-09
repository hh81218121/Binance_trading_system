# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/event_router
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/event_router/target/release/event_router /usr/local/bin/app
CMD ["app"]