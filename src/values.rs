//! Value engine

use crate::model::{ApiUnit, Item, Offer, OfferKind, Role, TraitKey};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub const UPWARD_DEVIATION_LIMIT: f64 = 2.0;
pub const MIN_QUOTES: usize = 10;
pub const QUOTE_MEDIAN_FRACTION: f64 = 0.5;

pub fn mean(v: &[f64]) -> f64 { if v.is_empty() { 0.0 } else { v.iter().sum::<f64>() / v.len() as f64 } }

pub fn median(v: &[f64]) -> Option<f64> { None }

pub fn trimmed_mean(v: &[f64], trim_fraction: f64) -> Option<f64> { None }

pub fn quote_median_filter(quotes: &[f64]) -> (f64, usize, usize) { (0.0, 0, 0) }

pub fn halves(values: &[f64], middle_in_upper: bool) -> Option<(f64, f64)> { None }

pub struct ValueEngine { pub units: HashMap<String, ApiUnit>, pub solved_trait: HashMap<(String, TraitKey), f64> }

impl ValueEngine {
    pub fn new(units: Vec<ApiUnit>) -> Self { Self { solved_trait: HashMap::new(), units: units.into_iter().map(|u| (u.name.clone(), u)).collect() } }
    pub fn new_empty() -> Self { Self { solved_trait: HashMap::new(), units: HashMap::new() } }
    pub fn base_value(&self, unit: &str) -> Option<f64> { self.units.get(unit).map(|u| u.value as f64) }
    pub fn unit(&self, name: &str) -> Option<&ApiUnit> { self.units.get(name) }
    pub fn item_value(&self, item: &Item) -> Option<f64> { None }
    pub fn bonus(&self, unit: &str, trait_key: Option<TraitKey>) -> Option<f64> { None }
    pub fn quotes_for(&self, offers: &[Offer], target: &str, target_trait: Option<TraitKey>, api_ref: Option<f64>, since: DateTime<Utc>, upward_limit: f64) -> QuoteSet { QuoteSet::default() }
    pub fn solve_unknowns(&mut self, offers: &[Offer]) {}
}

#[derive(Debug, Clone, Default)]
pub struct Quote { pub value: f64, pub role: Role, pub user: String, pub raw: String, pub deviation: Option<f64> }

#[derive(Debug, Clone, Default)]
pub struct QuoteSet { pub quotes: Vec<Quote>, pub seen: usize, pub rescued: usize, pub dropped: Vec<(String, String)> }

pub fn aggregate(pool: &[Quote]) -> Agg { Agg { value: None, exact: false, demand: 0.0, band: String::new(), sellers: vec![], buyers: vec![], seller_avg: None, buyer_avg: None, trimmed: 0 } }

#[derive(Debug, Clone, Default)]
pub struct Agg { pub value: Option<i64>, pub exact: bool, pub demand: f64, pub band: String, pub sellers: Vec<String>, pub buyers: Vec<String>, pub seller_avg: Option<f64>, pub buyer_avg: Option<f64>, pub trimmed: usize }
