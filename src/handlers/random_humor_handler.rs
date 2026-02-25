use axum::extract::State;
use reqwest::Response;
use serde::Deserialize;

use crate::utils::{
    response::{VoidsongError, VoidsongHumor},
    state::{AppContext, user_agent},
    url::preflight_check,
};

#[derive(Deserialize)]
struct ChuckNorrisFact {
    value: String,
}

pub async fn chuck_norris(State(state): State<AppContext>) -> Result<VoidsongHumor, VoidsongError> {
    let urls = ["https://api.chucknorris.io/jokes/random"];

    // Check if the APIs are available
    let Some(url) = preflight_check(&state.client, &urls).await else {
        return Err(VoidsongError::ServiceUnavailable);
    };

    // Get the image URL
    let get_url: Response = match state.client.get(url).headers(user_agent()).send().await {
        Ok(response) => response,
        Err(_) => return Err(VoidsongError::FailedToFetchContent),
    };

    let joke: String = match get_url.json::<ChuckNorrisFact>().await {
        Ok(response) => response.value,
        Err(_) => return Err(VoidsongError::FailedToFetchContent),
    };

    Ok(VoidsongHumor { joke })
}

/* -------------------------------------------------------------------------- */

#[derive(Deserialize)]
struct ICanHazDadJoke {
    joke: String,
}

pub async fn dad_joke(State(state): State<AppContext>) -> Result<VoidsongHumor, VoidsongError> {
    let urls = ["https://icanhazdadjoke.com"];

    // Check if the APIs are available
    let Some(url) = preflight_check(&state.client, &urls).await else {
        return Err(VoidsongError::ServiceUnavailable);
    };

    // Get the image URL
    let get_url: Response = match state.client.get(url).headers(user_agent()).send().await {
        Ok(response) => response,
        Err(_) => return Err(VoidsongError::FailedToFetchContent),
    };

    let joke: String = match get_url.json::<ICanHazDadJoke>().await {
        Ok(response) => response.joke,
        Err(_) => return Err(VoidsongError::FailedToFetchContent),
    };

    Ok(VoidsongHumor { joke })
}
