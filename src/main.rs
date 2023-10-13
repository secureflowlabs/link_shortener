use rusqlite::Connection;

pub mod api;
pub mod db;
pub mod error;
mod setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let file = std::env::var("DB_PATH")?;
    let conn = Connection::open(file.into())?;

    // TODO add actix server actually handling the requests

    Ok(())
}
