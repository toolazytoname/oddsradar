//! Public venue APIs → implied Yes probability (decimal string).

use crate::spread::{compare_event, parse_prob, CompareRow, Quote};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LiveError {
    #[error("http {0}")]
    Http(String),
    #[error("json {0}")]
    Json(String),
    #[error("unknown venue {0}")]
    Venue(String),
    #[error("no yes price on {0}")]
    NoPrice(String),
}

#[derive(Debug, Deserialize)]
pub struct MapRow {
    pub event_id: String,
    pub venue: String,
    pub market_id: String,
}

pub fn load_map(path: &Path) -> Result<Vec<MapRow>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in rdr.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub fn fetch_yes(venue: &str, market_id: &str) -> Result<String, LiveError> {
    match venue {
        "polymarket" => fetch_polymarket(market_id),
        "kalshi" => fetch_kalshi(market_id),
        other => Err(LiveError::Venue(other.into())),
    }
}

fn fetch_polymarket(id: &str) -> Result<String, LiveError> {
    let url = if id.chars().all(|c| c.is_ascii_digit()) {
        format!("https://gamma-api.polymarket.com/markets/{id}")
    } else {
        format!("https://gamma-api.polymarket.com/markets?slug={id}")
    };
    let v = get_json(&url)?;
    let market = if v.is_array() {
        v.get(0).cloned().unwrap_or(serde_json::Value::Null)
    } else {
        v
    };
    let prices = market
        .get("outcomePrices")
        .and_then(|p| p.as_str())
        .ok_or_else(|| LiveError::NoPrice(id.into()))?;
    let arr: Vec<String> = serde_json::from_str(prices).map_err(|e| LiveError::Json(e.to_string()))?;
    arr.first()
        .cloned()
        .ok_or_else(|| LiveError::NoPrice(id.into()))
}

fn fetch_kalshi(ticker: &str) -> Result<String, LiveError> {
    let url = format!("https://api.elections.kalshi.com/trade-api/v2/markets/{ticker}");
    let v = get_json(&url)?;
    let m = v.get("market").cloned().unwrap_or(v);
    for key in ["last_price_dollars", "yes_ask_dollars", "yes_bid_dollars", "last_price"] {
        if let Some(s) = m.get(key).and_then(|x| x.as_str()).filter(|s| !s.is_empty()) {
            if s == "0.0000" && key == "last_price_dollars" {
                continue;
            }
            return Ok(normalize_kalshi_price(key, s));
        }
        if let Some(n) = m.get(key).and_then(|x| x.as_i64()) {
            return Ok(normalize_kalshi_price(key, &n.to_string()));
        }
    }
    Err(LiveError::NoPrice(ticker.into()))
}

fn normalize_kalshi_price(key: &str, s: &str) -> String {
    if key == "last_price" && !s.contains('.') {
        // cents 0-100 → probability 0-1
        if let Ok(c) = s.parse::<i64>() {
            return format!("{}.{:02}", c / 100, c % 100);
        }
    }
    s.to_string()
}

fn get_json(url: &str) -> Result<serde_json::Value, LiveError> {
    let resp = ureq::get(url)
        .set("User-Agent", "oddsradar/0.1")
        .call()
        .map_err(|e| LiveError::Http(e.to_string()))?;
    resp.into_json().map_err(|e| LiveError::Json(e.to_string()))
}

pub fn compare_live(map_path: &Path, threshold: i64) -> Result<Vec<CompareRow>, Box<dyn std::error::Error>> {
    let map = load_map(map_path)?;
    let mut by_event: HashMap<String, Vec<Quote>> = HashMap::new();
    let mut errors = Vec::new();
    for row in map {
        match fetch_yes(&row.venue, &row.market_id) {
            Ok(yes) => {
                by_event.entry(row.event_id.clone()).or_default().push(Quote {
                    event_id: row.event_id,
                    venue: row.venue,
                    market_id: row.market_id,
                    yes: parse_prob(&yes)?,
                });
            }
            Err(e) => errors.push(format!("{} {}: {e}", row.venue, row.market_id)),
        }
    }
    let mut rows = Vec::new();
    let mut ids: Vec<_> = by_event.keys().cloned().collect();
    ids.sort();
    for id in ids {
        let qs = &by_event[&id];
        if qs.len() < 2 {
            for q in qs {
                let mut venues = std::collections::BTreeMap::new();
                venues.insert(q.venue.clone(), q.yes);
                rows.push(CompareRow {
                    kind: "quote".into(),
                    event_id: q.event_id.clone(),
                    spread_millionths: 0,
                    threshold_millionths: threshold,
                    venues,
                });
            }
            continue;
        }
        rows.push(compare_event(qs, threshold)?);
    }
    if rows.is_empty() && !errors.is_empty() {
        return Err(errors.join("; ").into());
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_polymarket_recorded() {
        let v: serde_json::Value =
            serde_json::from_str(include_str!("../fixtures/polymarket_market.json")).unwrap();
        let prices = v["outcomePrices"].as_str().unwrap();
        let arr: Vec<String> = serde_json::from_str(prices).unwrap();
        assert_eq!(arr[0], "0.0445");
        assert_eq!(parse_prob(&arr[0]).unwrap(), 44_500);
    }

    #[test]
    fn kalshi_cents_to_prob_string() {
        assert_eq!(normalize_kalshi_price("last_price", "62"), "0.62");
        assert_eq!(normalize_kalshi_price("yes_ask_dollars", "0.4100"), "0.4100");
    }
}
