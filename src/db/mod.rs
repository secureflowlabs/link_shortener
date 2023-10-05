use mongodb::Client;
use crate::error::LinkShortenerError;

mod link;

pub type DbResult<T> = Result<T, LinkShortenerError>;

pub async fn db_client() -> DbResult<Client> {
    let db_url = std::env::var("DATABASE_URL")?;

    Ok(Client::with_uri_str(db_url).await?)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::db::{db_client};
    use crate::db::link::{Link, LinkMeta};

    #[tokio::test]
    pub async fn get_db_client() -> anyhow::Result<()> {
        dotenv::dotenv().ok();

        let client = db_client().await?;

        client.default_database().unwrap();

        Ok(())
    }

    #[tokio::test]
    pub async fn create_link() -> anyhow::Result<()> {
        dotenv::dotenv().ok();

        let client = db_client().await?;
        let mut headers: HashMap<String, String> = HashMap::new();

        headers.insert("Foo".to_string(), "bar".to_string());

        let target_url = url::Url::parse("https://test.com")?;
        let link_meta = LinkMeta::new(headers);
        let link = Link::new(target_url, link_meta)?;

        link.write(&client, &link).await?;

        Ok(())
    }
}
