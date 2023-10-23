use crate::error::LinkShortenerError;
pub type LinkResult<T> = Result<T, LinkShortenerError>;

#[derive(sqlx::FromRow, Debug)]
pub struct ShortenedURL {
    pub id: String,
    pub original_url: String,
}
