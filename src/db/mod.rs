use crate::error::LinkShortenerError;

pub mod link;

pub type DbResult<T> = Result<T, LinkShortenerError>;
