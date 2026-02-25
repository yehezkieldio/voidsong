use axum::extract::State;
use reqwest::Response;
use serde::Deserialize;

use crate::utils::{
    response::{VoidsongError, VoidsongTrivia},
    state::{AppContext, user_agent},
    url::preflight_check,
};

#[derive(Deserialize)]
struct NekosLifeFact {
    fact: String,
}

pub async fn fact(State(state): State<AppContext>) -> Result<VoidsongTrivia, VoidsongError> {
    let urls = ["https://nekos.life/api/v2/fact"];

    // Check if the APIs are available
    let Some(url) = preflight_check(&state.client, &urls).await else {
        return Err(VoidsongError::ServiceUnavailable);
    };

    // Get the image URL
    let get_url: Response = match state.client.get(url).headers(user_agent()).send().await {
        Ok(response) => response,
        Err(_) => return Err(VoidsongError::FailedToFetchContent),
    };

    let fact: String = match get_url.json::<NekosLifeFact>().await {
        Ok(response) => response.fact,
        Err(_) => return Err(VoidsongError::FailedToFetchContent),
    };

    Ok(VoidsongTrivia { fact })
}

/* -------------------------------------------------------------------------- */

#[derive(Deserialize)]
struct CatFactNinja {
    fact: String,
}

pub async fn cat_fact(State(state): State<AppContext>) -> Result<VoidsongTrivia, VoidsongError> {
    let urls = ["https://catfact.ninja/fact"];

    // Check if the APIs are available
    let Some(url) = preflight_check(&state.client, &urls).await else {
        return Err(VoidsongError::ServiceUnavailable);
    };

    // Get the image URL
    let get_url: Response = match state.client.get(url).headers(user_agent()).send().await {
        Ok(response) => response,
        Err(_) => return Err(VoidsongError::FailedToFetchContent),
    };

    let fact: String = match get_url.json::<CatFactNinja>().await {
        Ok(response) => response.fact,
        Err(_) => return Err(VoidsongError::FailedToFetchContent),
    };

    Ok(VoidsongTrivia { fact })
}

/* -------------------------------------------------------------------------- */

#[derive(Deserialize)]
pub struct DogAPIKindUff {
    facts: Vec<String>,
}

pub async fn dog_fact(State(state): State<AppContext>) -> Result<VoidsongTrivia, VoidsongError> {
    let urls = ["https://dog-api.kinduff.com/api/facts?number=1"];

    // Check if the APIs are available
    let Some(url) = preflight_check(&state.client, &urls).await else {
        return Err(VoidsongError::ServiceUnavailable);
    };

    // Get the image URL
    let get_url: Response = match state.client.get(url).headers(user_agent()).send().await {
        Ok(response) => response,
        Err(_) => return Err(VoidsongError::FailedToFetchContent),
    };

    let fact: String = match get_url.json::<DogAPIKindUff>().await {
        Ok(response) => match response.facts.into_iter().next() {
            Some(fact) => fact,
            None => return Err(VoidsongError::FailedToFetchContent),
        },
        Err(_) => return Err(VoidsongError::FailedToFetchContent),
    };

    Ok(VoidsongTrivia { fact })
}
