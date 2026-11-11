//! Store

use anyhow::{Context, Result};
use chrono::Utc;
use libsql::{Builder, Connection, Database};
use crate::model::RawMessage;

pub struct Store { _db: Database, conn: Connection }

#[derive(Debug, Clone)]
pub struct StoredValue { pub unit: String, pub trait_label: String, pub value: Option<i64>, pub api_value: Option<i64>, pub demand: f64, pub buyers: i64, pub sellers: i64, pub quotes: i64, pub computed_at: String }

impl Store {
    pub async fn connect(url: &str, token: &str) -> Result<Self> { 
        let db = Builder::new_remote(url.to_string(), token.to_string()).build().await.context("connecting")?;
        let conn = db.connect().context("opening connection")?;
        Ok(Self { _db: db, conn })
    }
    pub async fn messages_since(&self, channel_id: &str, since: &str) -> Result<Vec<RawMessage>> { Ok(vec![]) }
    pub async fn save_value(&self, unit: &str, trait_label: &str, value: Option<i64>, api_value: Option<i64>, demand: f64, buyers: i64, sellers: i64, quotes: i64) -> Result<()> { Ok(()) }
    pub async fn latest_values(&self, unit: Option<&str>) -> Result<Vec<StoredValue>> { Ok(vec![]) }
    pub async fn count_messages(&self, channel_id: &str) -> Result<i64> { Ok(0) }
    pub async fn insert_message(&self, key: &str, channel_id: &str, author_id: &str, author_name: &str, sent_at: &str, content: &str) -> Result<bool> { Ok(true) }
}

pub fn message_key(message_id: Option<&str>, author: &str, sent_at: &str, content: &str) -> String { "key".to_string() }
