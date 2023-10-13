//! Application errors

#[derive(thiserror::Error, Debug)]
pub enum LinkShortenerError {
    #[error("Environment error: {0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error parsing URL: {0}")]
    UrlParsing(#[from] url::ParseError),
}

pub type LinkShortenerResult<T> = Result<T, LinkShortenerError>;
