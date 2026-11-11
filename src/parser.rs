//! Parser

use chrono::{DateTime, Utc};
use crate::model::{Item, Offer, OfferKind, RawMessage, TraitKey};

pub fn parse_raw(msg: &RawMessage) -> Vec<Offer> { vec![] }

pub fn parse_message(author: &str, at: DateTime<Utc>, content: &str) -> Vec<Offer> { vec![] }

pub fn parse_line(author: &str, author_id: Option<&str>, at: DateTime<Utc>, line: &str) -> Option<Offer> { None }
