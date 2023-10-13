//! Create DB tables

use rusqlite::Connection;

const CREATE_LINK_TABLE: &str = "CREATE TABLE IF NOT EXISTS links (\
                        id varchar(10) unique not null primary key,\
                        target text not null
                        )";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let file = std::env::var("DB_PATH")?;
    let conn = Connection::open(file.into())?;

    conn.execute(CREATE_LINK_TABLE,
        (), // empty list of parameters.
    )?;

    Ok(())
}
