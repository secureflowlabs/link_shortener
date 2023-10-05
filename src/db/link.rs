use std::collections::HashMap;
use mongodb::Client;
use mongodb::results::{InsertManyResult, InsertOneResult};
use serde::{Serialize, Deserialize};
use crate::db::DbResult;
use crate::error::LinkShortenerError;

pub type LinkResult<T> = Result<T, LinkShortenerError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkMeta {
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    headers: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    target: url::Url,
    shortened: url::Url,
    meta: LinkMeta,
}

impl LinkMeta {
    pub fn new(headers: HashMap<String, String>) -> Self {
        Self {
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            headers
        }
    }
}

impl Link {
    pub fn new(target: url::Url, meta: LinkMeta) -> LinkResult<Self> {
        let shortened = url::Url::parse("https://test.com")?;

        Ok(Self {
            target,
            shortened,
            meta,
        })
    }

    pub async fn write(&self,client: &Client, link: &Link) -> DbResult<InsertOneResult> {
        let db = client.default_database().expect("Default database not found");

        Ok(db.collection::<Link>("links").insert_one(link.to_owned(), None).await?)
    }

    pub async fn write_all(&self,client: &Client, links: Vec<&Link>) -> DbResult<InsertManyResult> {
        let db = client.default_database().expect("Default database not found");

        Ok(db.collection::<Link>("links").insert_many(links, None).await?)
    }
}
