use std::pin::Pin;

use super::{response::VoidsongImage, state::user_agent};
use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use reqwest::Client;

pub async fn preflight_check<'a>(client: &Client, urls: &'a [&'a str]) -> Option<&'a str> {
    for &url in urls {
        let response = client.head(url).headers(user_agent()).send().await;

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    return Some(url);
                }
            }
            Err(_) => continue,
        }
    }
    None
}

pub async fn fetch_image(client: &Client, url: &str) -> Option<VoidsongImage> {
    let response = client.get(url).headers(user_agent()).send().await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                let content_type = response
                    .headers()
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|value| value.to_str().ok())
                    .unwrap_or("image/jpeg")
                    .to_string();

                let stream: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>> =
                    response.bytes_stream().boxed();

                Some(VoidsongImage {
                    content_type,
                    stream,
                })
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
