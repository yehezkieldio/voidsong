FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# ---------------------------------------------------------------------------- #

FROM chef AS planner
COPY . .
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build --release --bin voidsong

# ---------------------------------------------------------------------------- #

FROM debian:bookworm-slim AS final

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser

COPY --from=builder /app/target/release/voidsong /usr/local/bin/voidsong
RUN chown appuser /usr/local/bin/voidsong

USER appuser
WORKDIR /usr/local/bin

ENV SERVER_HOST=127.0.0.1
ENV SERVER_PORT=9090

ENTRYPOINT ["voidsong"]
