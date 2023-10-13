use crate::error::LinkShortenerError;
pub mod link;
pub type DbResult<T> = Result<T, LinkShortenerError>;

const INSERT_LINK: &str = "INSERT INTO links () VALUES ()";
const GET_REDIRECT_URL: &str = "SELECT target_url FROM links where key = :key";
