![Banner](assets/banner.png)

## Voidsong

Voidsong is a versatile, self-hosted API aggregator designed to streamline your interaction with various APIs by providing a unified endpoint for all your content needs. Whether you're looking for random images, trivia facts, or jokes. It serves as a reliable intermediary, fetching and delivering content seamlessly from multiple external APIs.

The project is written in Rust, using the [Axum](https://github.com/tokio-rs/axum) web framework.

### Building and Running

You can start the service by building the project and running the binary, with the following command:

> **Note:** Make sure you have Rust installed on your system. You can install it from [rust-lang.org](https://www.rust-lang.org/).

```bash
cargo run --release
```

By default, the service will be available at `http://localhost:8080`, you can change the port by setting an environment variable file named `.env` with the following content:

```env
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

### API Routes

#### Media

These routes return random images of animals.

- `GET /random/media/cat` - Returns a random cat image.
- `GET /random/media/dog` - Returns a random dog image.
- `GET /random/media/fox` - Returns a random fox image.
- `GET /random/media/bunny` - Returns a random bunny image.
- `GET /random/media/duck` - Returns a random duck image.

#### Trivia

These routes return random trivia facts.

- `GET /random/trivia/fact` - Returns a random trivia fact.
- `GET /random/trivia/catfact` - Returns a random cat fact.
- `GET /random/trivia/dogfact` - Returns a random dog fact.

#### Humor

These routes return random jokes.

- `GET /random/humor/chucknorris` - Returns a random Chuck Norris joke.
- `GET /random/humor/dadjoke` - Returns a random dad joke.

### License

This project is proudly provided under the [MIT License](LICENSE).
