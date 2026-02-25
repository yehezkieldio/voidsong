# syntax=docker/dockerfile:1.7

ARG RUST_IMAGE_TAG=latest-rust-1.85-bookworm
ARG RUNTIME_IMAGE_TAG=bookworm-slim

# --- 1. Toolchain (cargo-chef) ---
# Build in a dedicated toolchain image so dependency layers are reusable.
FROM lukemathwalker/cargo-chef:${RUST_IMAGE_TAG} AS chef
WORKDIR /app

# --- 2. Planner ---
# Generate a dependency recipe from Cargo manifests.
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- 3. Builder ---
# Build dependencies first, then the final binary.
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked --bin voidsong

# --- 4. Runtime ---
# Minimal runtime image for public deployment.
FROM debian:${RUNTIME_IMAGE_TAG} AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Run as a non-root user (UID/GID 10001 is a common container convention).
RUN groupadd --system --gid 10001 voidsong \
    && useradd --system --uid 10001 --gid 10001 \
    --create-home --home-dir /home/voidsong --shell /usr/sbin/nologin voidsong

COPY --from=builder /app/target/release/voidsong /usr/local/bin/voidsong
RUN chown voidsong:voidsong /usr/local/bin/voidsong

WORKDIR /home/voidsong
ENV SERVER_HOST=0.0.0.0 \
    SERVER_PORT=8080 \
    RUST_LOG=info

EXPOSE 8080
USER voidsong:voidsong
ENTRYPOINT ["/usr/local/bin/voidsong"]
