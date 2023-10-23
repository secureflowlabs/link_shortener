use crate::error::LinkShortenerError;
pub mod link;
pub type DbResult<T> = Result<T, LinkShortenerError>;

pub const CREATE_TABLE: &str = r#"
        CREATE TABLE IF NOT EXISTS urls (
            id varchar(10) UNIQUE NOT NULL,
            original_url TEXT NOT NULL
        )
    "#;
pub const INSERT_LINK: &str = r#"
    INSERT INTO urls (id, original_url)
    VALUES
    (?, ?)
    RETURNING id, original_url"#;
pub const GET_REDIRECT_URL: &str = r#"SELECT original_url FROM urls where id = ?"#;
