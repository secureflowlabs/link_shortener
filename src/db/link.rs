use crate::error::LinkShortenerError;
pub type LinkResult<T> = Result<T, LinkShortenerError>;

#[derive(sqlx::FromRow, Debug)]
pub struct ShortenedURL {
    id: i64,
    pub original_url: String,
    pub short_url: String,
}
