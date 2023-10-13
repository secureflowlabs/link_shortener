use std::collections::HashMap;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use crate::error::LinkShortenerError;
pub type LinkResult<T> = Result<T, LinkShortenerError>;

#[derive(sqlx::FromRow)]
pub struct ShortenedURL {
    id: i64,
    original_url: String,
    short_url: String,
}
