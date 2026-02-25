<div align="center">
<img src="assets/avatar.png" align="center" width="120px" height="120px" />
<h3>Voidsong</h3>
<p>Self-hosted random content API for media, trivia, and humor</p>

<a href="https://github.com/yehezkieldio/voidsong/releases"><img src="https://img.shields.io/github/v/release/yehezkieldio/voidsong?style=flat&labelColor=1C2C2E&color=C96329&logo=GitHub&logoColor=white"></a>
<a href="https://github.com/yehezkieldio/voidsong/pkgs/container/voidsong"><img src="https://img.shields.io/github/actions/workflow/status/yehezkieldio/voidsong/release.yml?style=flat&labelColor=1C2C2E&color=C96329&label=docker&logo=Docker&logoColor=white"></a>
</div>

---

Voidsong is a lightweight API aggregator written in Rust with [Axum](https://github.com/tokio-rs/axum).  
It exposes stable endpoints for random animal media, trivia facts, and jokes while handling upstream API fetches behind a single base URL.

## Features

- **Single API surface**: one service with grouped routes under `/random`.
- **Upstream availability preflight**: handlers check source health before fetching data.
- **Streaming media responses**: image endpoints stream bytes directly with the original content type.
- **Consistent response headers**: includes `x-voidsong-version` and `cache-control: no-cache`.
- **Container-ready runtime**: multi-stage Docker build with non-root runtime user.

## API Routes

### Media

- `GET /random/media/cat`
- `GET /random/media/dog`
- `GET /random/media/fox`
- `GET /random/media/bunny`
- `GET /random/media/duck`

### Trivia

- `GET /random/trivia/fact`
- `GET /random/trivia/catfact`
- `GET /random/trivia/dogfact`

### Humor

- `GET /random/humor/chucknorris`
- `GET /random/humor/dadjoke`

## Building from Source

### Prerequisites

- [Rust stable](https://rustup.rs/)
- `cargo-nextest` (for `just test`)
- [`just`](https://just.systems/) (optional, for convenience commands)

### Build and run

```sh
git clone https://github.com/yehezkieldio/voidsong.git
cd voidsong
cargo run --release
```

The service listens on `0.0.0.0:8080` by default.

## Configuration

Configuration is loaded from `.env` and process environment variables.

| Variable      | Default   | Description            |
| ------------- | --------- | ---------------------- |
| `SERVER_HOST` | `0.0.0.0` | Bind address           |
| `SERVER_PORT` | `8080`    | Bind port              |
| `RUST_LOG`    | `info`    | Logging verbosity      |

Example `.env`:

```env
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=info
```

## Docker

Build the production runtime image locally:

```sh
docker build --target runtime -t voidsong:latest .
```

Run it:

```sh
docker run --rm -p 8080:8080 \
  -e SERVER_HOST=0.0.0.0 \
  -e SERVER_PORT=8080 \
  -e RUST_LOG=info \
  voidsong:latest
```

Published images are available from GHCR: `ghcr.io/<owner>/voidsong:<tag>`.

## Development

```sh
just check
just clippy
just test --no-tests=pass
just fmt
```

## License

This project is licensed under the [MIT License](LICENSE).
