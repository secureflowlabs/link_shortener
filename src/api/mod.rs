use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Serialize, Deserialize};
use crate::error::LinkShortenerResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
}

// Generate shortened key for URL
pub fn shortened_key() -> LinkShortenerResult<String> {
    Ok(rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect())
}
