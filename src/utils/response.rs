use std::pin::Pin;

use axum::Json;
use axum::body::Body;
use axum::http::{HeaderName, StatusCode, header};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use futures_util::Stream;
use serde::Serialize;

use crate::env::VERSION;

pub enum VoidsongError {
    FailedToFetchImage,
    FailedToFetchContent,
    ServiceUnavailable,
    InvalidRoute,
}

impl IntoResponse for VoidsongError {
    fn into_response(self) -> Response {
        let headers = [
            (header::CACHE_CONTROL, "no-cache"),
            (HeaderName::from_static("x-voidsong-version"), VERSION),
        ];

        let (status, body) = match self {
            VoidsongError::FailedToFetchImage => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to fetch image data",
            ),
            VoidsongError::FailedToFetchContent => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to fetch content data",
            ),
            VoidsongError::ServiceUnavailable => {
                (StatusCode::SERVICE_UNAVAILABLE, "service unavailable")
            }
            VoidsongError::InvalidRoute => (
                StatusCode::NOT_FOUND,
                "invalid route accessed for this path",
            ),
        };

        (status, headers, body).into_response()
    }
}

/* -------------------------------------------------------------------------- */

#[derive(Serialize)]
pub struct VoidsongHumor {
    pub joke: String,
}

impl IntoResponse for VoidsongHumor {
    fn into_response(self) -> Response {
        let headers = [
            (header::CACHE_CONTROL, "no-cache"),
            (HeaderName::from_static("x-voidsong-version"), VERSION),
        ];

        (StatusCode::OK, headers, Json(self)).into_response()
    }
}

/* -------------------------------------------------------------------------- */

#[derive(Serialize)]
pub struct VoidsongTrivia {
    pub fact: String,
}

impl IntoResponse for VoidsongTrivia {
    fn into_response(self) -> Response {
        let headers = [
            (header::CACHE_CONTROL, "no-cache"),
            (HeaderName::from_static("x-voidsong-version"), VERSION),
        ];

        (StatusCode::OK, headers, Json(self)).into_response()
    }
}

/* -------------------------------------------------------------------------- */

pub struct VoidsongImage {
    pub content_type: String,
    pub stream: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
}

impl IntoResponse for VoidsongImage {
    fn into_response(self) -> Response {
        let headers = [
            (header::CONTENT_TYPE, self.content_type.as_str()),
            (header::CACHE_CONTROL, "no-cache"),
            (HeaderName::from_static("x-voidsong-version"), VERSION),
        ];

        let stream = Body::from_stream(self.stream);

        (StatusCode::OK, headers, stream).into_response()
    }
}
