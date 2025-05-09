# Build Stage
FROM rust:1.77 as builder

WORKDIR /usr/src/coldstart_recovery_loader
COPY . .

RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/coldstart_recovery_loader/target/release/coldstart_recovery_loader /usr/local/bin/app
CMD ["app"]