use tokio::net::TcpListener;

async fn init_tracing() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
}

async fn setup_server(host: &str, port: &str) -> (axum::Router, tokio::net::TcpListener) {
    let app = voidsong::routes::root::routes();
    let listener: TcpListener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();

    (app, listener)
}

#[tokio::main]
async fn main() {
    init_tracing().await;

    let (host, port) = voidsong::env::load();
    let (app, listener) = setup_server(&host, &port).await;

    tracing::info!("Running Voidsong v{}", voidsong::env::VERSION);
    tracing::info!("Listening on http://{}:{}", host, port);

    axum::serve(listener, app).await.unwrap();
}
