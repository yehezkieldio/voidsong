#![allow(unused_crate_dependencies)]

use tokio::net::TcpListener;

async fn init_tracing() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
}

async fn setup_server(host: &str, port: &str) -> std::io::Result<(axum::Router, TcpListener)> {
    let app = voidsong::routes::root::routes();
    let listener = TcpListener::bind(format!("{host}:{port}")).await?;

    Ok((app, listener))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_tracing().await;

    let (host, port) = voidsong::env::load();
    let (app, listener) = setup_server(&host, &port).await?;

    tracing::info!("Running Voidsong v{}", voidsong::env::VERSION);
    tracing::info!("Listening on http://{}:{}", host, port);

    axum::serve(listener, app).await
}
