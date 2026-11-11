//! Reading the trading channel through Discord's REST API.

use anyhow::{bail, Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use tokio::time::{sleep, Duration as Wait};

use crate::model::RawMessage;

pub const API: &str = "https://discord.com/api/v10";

const PAGE: &str = "100";
const PAGE_DELAY_MS: u64 = 350;

pub struct Discord {
    token: String,
    http: reqwest::Client,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Message {
    pub id: String,
    pub content: String,
    pub timestamp: String,
    pub author: Author,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Author {
    pub id: String,
    pub username: String,
}

#[derive(Deserialize)]
struct RateLimited {
    retry_after: f64,
}

impl Discord {
    pub fn new(token: &str) -> Result<Self> {
        let http = reqwest::Client::builder()
            .user_agent("DiscordBot (https://github.com/rachid-dotcom/Bot-test, 0.1.0)")
            .build()
            .context("building the Discord HTTP client")?;
        Ok(Self { token: token.trim().to_string(), http })
    }

    pub async fn new_since(&self, channel: &str, after: Option<&str>) -> Result<Vec<Message>> {
        Ok(vec![])
    }

    pub async fn history(&self, channel: &str, since: DateTime<Utc>) -> Result<Vec<Message>> {
        Ok(vec![])
    }
}

impl Message {
    pub fn to_raw(&self) -> Option<RawMessage> {
        None
    }
}

pub fn timestamp_of(id: &str) -> Option<DateTime<Utc>> { None }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn snowflakes_decode_to_their_creation_time() {}
    #[test]
    fn ids_order_numerically_not_as_text() {}
    #[test]
    fn unreadable_messages_are_skipped() {}
}
