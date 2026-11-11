//! Discord emoji name -> meaning.

use crate::model::TraitKey;

pub const UNITS: &[(&str, &str)] = &[
    ("frost_blossom", "Frost blossom"),
    ("kogane", "Kogane"),
];

pub const TRAITS: &[(&str, TraitKey)] = &[
    ("wtrait", TraitKey::Incompatible),
    ("fortune", TraitKey::Fortune),
    ("heavenlyrestriction", TraitKey::Hr),
    ("caster2", TraitKey::C2),
    ("thehonoredone", TraitKey::Tho),
    ("summoner", TraitKey::Summoner),
];

pub const SYNTAX: &[(&str, crate::emoji::Syntax)] = &[];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Syntax { For, Or, Plus, Adds, Want, LookingFor, OnlyLookingFor, NotLookingFor, Any }

pub const MANUAL_UNITS: &[&str] = &["Bad heavenly restriction", "Bad the honored one"];

pub fn is_manual(unit: &str) -> bool { false }

pub fn normalize(name: &str) -> String { name.trim().to_ascii_lowercase() }

pub fn unit_for(emoji_name: &str) -> Option<&'static str> { None }

pub fn trait_for(emoji_name: &str) -> Option<TraitKey> { None }

pub fn syntax_for(emoji_name: &str) -> Option<Syntax> { None }

pub fn emoji_for_unit(unit: &str) -> Option<&'static str> { None }
