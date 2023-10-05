//! Application errors

#[derive(thiserror::Error, Debug)]
pub enum LinkShortenerError {
    #[error("MongoDB error: {0}")]
    MongoDb(#[from] mongodb::error::Error),
    #[error("Environment error: {0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("Error parsing URL: {0}")]
    UrlParsing(#[from] url::ParseError),
}
