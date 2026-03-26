# build stage
FROM rust:1.93.1 AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# runtime stage
FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/staffback_rs /usr/local/bin/staffback_rs

CMD ["staffback_rs"]