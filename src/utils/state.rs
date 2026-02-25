use axum::http::HeaderMap;
use reqwest::Client;

use crate::env::VERSION;

pub fn user_agent() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "User-Agent",
        format!("Voidsong/{}", VERSION).parse().unwrap(),
    );

    headers
}

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
