use axum::http::{HeaderMap, HeaderValue, header};
use reqwest::Client;

use crate::env::VERSION;

pub fn user_agent() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let user_agent = format!("Voidsong/{VERSION}");
    let user_agent =
        HeaderValue::from_str(&user_agent).unwrap_or_else(|_| HeaderValue::from_static("Voidsong"));
    headers.insert(header::USER_AGENT, user_agent);

    headers
}

#[derive(Clone)]
pub struct AppContext {
    pub client: Client,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl Default for AppContext {
    fn default() -> Self {
        Self::new()
    }
}
