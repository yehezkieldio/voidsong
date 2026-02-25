use axum::Router;
use tower_http::{
    compression::CompressionLayer,
    normalize_path::NormalizePathLayer,
    trace::{self, TraceLayer},
};
use tracing::Level;

use crate::utils::response::VoidsongError;

use super::random_route;

pub fn routes() -> Router {
    Router::new()
        .nest("/random", random_route::routes())
        .fallback(handler_404)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(CompressionLayer::new())
        .layer(NormalizePathLayer::trim_trailing_slash())
}

async fn handler_404() -> VoidsongError {
    VoidsongError::InvalidRoute
}
