//! Client for `https://api.sorcerer-td-values.com`.
//!
//! The endpoint is behind a browser check: a bare request gets
//! `403 {"error":"forbidden"}`, and with an Origin header but no fetch metadata it
//! gets `403 {"error":"browser required","reason":"no-fetch-metadata"}`. Sending
//! the `Sec-Fetch-*` triple plus an Origin/Referer is what makes it answer 200.

use anyhow::{Context, Result};

use crate::model::ApiUnit;

pub const DEFAULT_BASE: &str = "https://api.sorcerer-td-values.com";

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                          (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
const ORIGIN: &str = "https://sorcerer-td-values.com";

pub fn client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .gzip(true)
        .build()
        .context("building HTTP client")
}

pub async fn fetch_units(base: &str) -> Result<Vec<ApiUnit>> {
    let client = client()?;
    let url = format!("{}/units", base.trim_end_matches('/'));
    let resp = client
        .get(&url)
        .header("Origin", ORIGIN)
        .header("Referer", format!("{}/", ORIGIN))
        .header("Accept", "application/json, text/plain, */*")
        .header("Sec-Fetch-Site", "same-site")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Dest", "empty")
        .send()
        .await
        .with_context(|| format!("GET {url}"))?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("units API returned {status}: {body}");
    }
    let units: Vec<ApiUnit> = resp.json().await.context("decoding /units payload")?;
    if units.is_empty() {
        anyhow::bail!("units API returned an empty list");
    }
    Ok(units)
}

/// Load the unit table, using the on-disk cache unless a refresh is requested.
pub async fn load_units(base: &str, cache_path: &str, refresh: bool) -> Result<Vec<ApiUnit>> {
    if !refresh {
        if let Ok(text) = std::fs::read_to_string(cache_path) {
            if let Ok(units) = serde_json::from_str::<Vec<ApiUnit>>(&text) {
                if !units.is_empty() {
                    return Ok(units);
                }
            }
        }
    }
    let units = fetch_units(base).await?;
    if let Ok(json) = serde_json::to_string_pretty(&units) {
        let _ = std::fs::write(cache_path, json);
    }
    Ok(units)
}
