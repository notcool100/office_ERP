# Backend Dockerfile for Coolify
# Builds the Rust axum backend and serves it on port 3117.
ARG RUST_VERSION=1.88.0
FROM rust:${RUST_VERSION} AS builder
WORKDIR /usr/src/app

# Copy manifest and install dependencies for build caching
COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src
RUN echo 'fn main() { println!("dummy"); }' > src/main.rs
RUN cargo build --release || true

# Copy full backend source and build the release binary
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/be /usr/local/bin/be
WORKDIR /app

EXPOSE 3117
ENV PORT=3117
CMD ["be"]
