# syntax=docker/dockerfile:1.7

FROM rust:1.88-bookworm AS dev
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        libpq-dev \
        postgresql-client \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch --locked

COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src \
    && echo 'fn main() {}' > src/main.rs \
    && cargo fetch

COPY . .

EXPOSE 8080
CMD ["cargo", "watch", "-x", "run"]

FROM lukemathwalker/cargo-chef:latest-rust-1.88-bookworm AS chef
WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src \
    && echo 'fn main() {}' > src/main.rs

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.88-bookworm AS builder
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --locked

FROM debian:bookworm-slim AS prod
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
        libpq5 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /app/backend

EXPOSE 8080
CMD ["/app/backend"]
