//! Application errors

#[derive(thiserror::Error, Debug)]
pub enum LinkShortenerError {
    #[error("Unknown DB error: {0}")]
    SurrealDbError(#[from] surrealdb::Error),
    #[error("Surreal DB error: {0}")]
    SurrealDb(#[from] surrealdb::error::Db),
    #[error("Surreal API error: {0}")]
    SurrealApi(#[from] surrealdb::error::Api),
    #[error("Environment error: {0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("Error parsing URL: {0}")]
    UrlParsing(#[from] url::ParseError),
}
