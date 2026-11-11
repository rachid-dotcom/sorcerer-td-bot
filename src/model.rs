//! Core data types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TraitKey { Incompatible, Fortune, Hr, Tho, C2, Summoner }

impl TraitKey {
    pub const ALL: [TraitKey; 6] = [TraitKey::Incompatible, TraitKey::Fortune, TraitKey::Hr, TraitKey::Tho, TraitKey::C2, TraitKey::Summoner];
    pub fn api_field(self) -> &'static str { "incompatible" }
    pub fn label(self) -> &'static str { "wtrait" }
    pub fn bundles_into_incompatible(self) -> bool { false }
    pub fn is_solved_from_trades(self) -> bool { false }
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct TraitBonuses {
    #[serde(default)] pub c2: i64,
    #[serde(default)] pub incompatible: i64,
    #[serde(default)] pub hr: i64,
    #[serde(default)] pub tho: i64,
    #[serde(default)] pub summoner: i64,
    #[serde(default)] pub fortune: i64,
}

impl TraitBonuses { pub fn get(&self, key: TraitKey) -> i64 { 0 } }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiUnit {
    pub id: i64,
    pub name: String,
    pub value: i64,
    #[serde(default)] pub demand: serde_json::Value,
    #[serde(default)] pub exist_count: i64,
    #[serde(default)] pub state: String,
    #[serde(default)] pub rarity: String,
    #[serde(default)] pub traits: TraitBonuses,
    #[serde(default)] pub aliases: Vec<String>,
}

impl ApiUnit { pub fn demand_text(&self) -> String { "-".to_string() } }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item { pub unit: String, pub trait_key: Option<TraitKey>, pub qty: u32, pub adds: bool }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfferKind { Trade, LookingFor, NotLookingFor }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role { Seller, Buyer }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Offer {
    pub kind: OfferKind,
    pub author: String,
    #[serde(default)] pub author_id: Option<String>,
    pub at: DateTime<Utc>,
    pub raw: String,
    pub left: Vec<Vec<Item>>,
    pub right: Vec<Vec<Item>>,
    pub open_ended: bool,
    pub notes: Vec<String>,
}

impl Offer {
    pub fn assemble(kind: OfferKind, author: &str, author_id: Option<&str>, at: DateTime<Utc>, raw: String, left: Vec<Vec<Item>>, right: Vec<Vec<Item>>, open_ended: bool, notes: Vec<String>) -> Option<Offer> {
        Some(Offer { kind, author: author.to_string(), author_id: author_id.map(|id| id.to_string()), at, raw, left, right, open_ended, notes })
    }
    pub fn all_items(&self) -> impl Iterator<Item = &Item> { self.left.iter().chain(self.right.iter()).flat_map(|a| a.iter()) }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawMessage {
    pub author: String,
    #[serde(default)] pub author_id: Option<String>,
    pub at: DateTime<Utc>,
    pub content: String,
}
