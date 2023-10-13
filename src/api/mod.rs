use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Serialize, Deserialize};
use crate::error::LinkShortenerResult;

#[derive(Serialize, Deserialize)]
pub struct ShortenRequest {
    pub(crate) url: String,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub(crate) short_url: String,
}

pub fn generate_short_url() -> LinkShortenerResult<String> {
    Ok(format!("{}{}", std::env::var("URL_PREFIX")?, shortened_key()))
}

// Generate shortened key for URL
pub fn shortened_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}
