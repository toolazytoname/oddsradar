//! Implied-probability spread. Millionths integer scale (1_000_000 = 100%).

use serde::Serialize;
use std::collections::BTreeMap;
use thiserror::Error;

pub const PROB_SCALE: i64 = 1_000_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quote {
    pub event_id: String,
    pub venue: String,
    pub market_id: String,
    pub yes: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompareRow {
    pub kind: String,
    pub event_id: String,
    pub spread_millionths: i64,
    pub threshold_millionths: i64,
    pub venues: BTreeMap<String, i64>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SpreadError {
    #[error("need at least two venue quotes")]
    NeedTwoVenues,
    #[error("quotes must share event_id")]
    MixedEvents,
    #[error("bad prob: {0}")]
    BadProb(String),
    #[error("prob out of range: {0}")]
    OutOfRange(String),
}

pub fn parse_prob(text: &str) -> Result<i64, SpreadError> {
    let s = text.trim();
    if s.is_empty() {
        return Err(SpreadError::BadProb(text.to_string()));
    }
    let val = if let Some((whole, frac)) = s.split_once('.') {
        let whole = if whole.is_empty() { "0" } else { whole };
        if whole.starts_with('-') || !whole.chars().all(|c| c.is_ascii_digit()) || !frac.chars().all(|c| c.is_ascii_digit())
        {
            return Err(SpreadError::BadProb(text.to_string()));
        }
        let mut frac = frac.to_string();
        frac.truncate(6);
        while frac.len() < 6 {
            frac.push('0');
        }
        let w: i64 = whole.parse().map_err(|_| SpreadError::BadProb(text.to_string()))?;
        let f: i64 = frac.parse().map_err(|_| SpreadError::BadProb(text.to_string()))?;
        w * PROB_SCALE + f
    } else {
        if !s.chars().all(|c| c.is_ascii_digit()) {
            return Err(SpreadError::BadProb(text.to_string()));
        }
        let n: i64 = s.parse().map_err(|_| SpreadError::BadProb(text.to_string()))?;
        if n <= 1 {
            n * PROB_SCALE
        } else {
            n
        }
    };
    if !(0..=PROB_SCALE).contains(&val) {
        return Err(SpreadError::OutOfRange(text.to_string()));
    }
    Ok(val)
}

pub fn spread_millionths(quotes: &[Quote]) -> Result<i64, SpreadError> {
    if quotes.len() < 2 {
        return Err(SpreadError::NeedTwoVenues);
    }
    let max = quotes.iter().map(|q| q.yes).max().unwrap();
    let min = quotes.iter().map(|q| q.yes).min().unwrap();
    Ok(max - min)
}

pub fn compare_event(quotes: &[Quote], threshold: i64) -> Result<CompareRow, SpreadError> {
    if quotes.is_empty() {
        return Err(SpreadError::NeedTwoVenues);
    }
    let event_id = &quotes[0].event_id;
    if quotes.iter().any(|q| q.event_id != *event_id) {
        return Err(SpreadError::MixedEvents);
    }
    let spr = spread_millionths(quotes)?;
    let mut venues = BTreeMap::new();
    for q in quotes {
        venues.insert(q.venue.clone(), q.yes);
    }
    let kind = if spr > threshold { "spread" } else { "ok" };
    Ok(CompareRow {
        kind: kind.into(),
        event_id: event_id.clone(),
        spread_millionths: spr,
        threshold_millionths: threshold,
        venues,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(venue: &str, yes: &str) -> Quote {
        Quote {
            event_id: "e".into(),
            venue: venue.into(),
            market_id: venue.into(),
            yes: parse_prob(yes).unwrap(),
        }
    }

    #[test]
    fn parse_and_spread() {
        assert_eq!(parse_prob("0.62").unwrap(), 620_000);
        let a = q("pm", "0.62");
        let b = q("kx", "0.50");
        assert_eq!(spread_millionths(&[a.clone(), b.clone()]).unwrap(), 120_000);
        let wide = compare_event(&[a, b], 50_000).unwrap();
        assert_eq!(wide.kind, "spread");
        let tight = compare_event(&[q("pm", "0.619"), q("kx", "0.618")], 50_000).unwrap();
        assert_eq!(tight.kind, "ok");
    }

    #[test]
    fn one_quote_rejected() {
        assert_eq!(spread_millionths(&[q("pm", "0.5")]), Err(SpreadError::NeedTwoVenues));
    }
}
