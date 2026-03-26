# build stage
FROM rust:1.87 AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# runtime stage
FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/my_app /usr/local/bin/my_app

CMD ["staffback_rs"]