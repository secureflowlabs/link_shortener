use std::collections::HashMap;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use crate::error::LinkShortenerError;
pub type LinkResult<T> = Result<T, LinkShortenerError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkMeta {
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[table(name = "links")]
pub struct Link {
    id: Option<Thing>,
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
        let id = uuid::Uuid::new_v4().to_string();
        let id = Some(Thing::from_str(&id).unwrap());

        Ok(Self {
            id,
            target,
            shortened,
            meta,
        })
    }
}
