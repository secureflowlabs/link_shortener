#[derive(sqlx::FromRow, Debug)]
pub struct ShortenedURL {
    pub id: String,
    pub original_url: String,
}
