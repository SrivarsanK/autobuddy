# Build stage
FROM rust:1.75-slim-bookworm as builder

WORKDIR /usr/src/autobuddy
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl ca-certificates systemd && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/autobuddy/target/release/autobuddy /usr/local/bin/autobuddy
COPY autobuddy.toml /app/autobuddy.toml

CMD ["autobuddy"]
