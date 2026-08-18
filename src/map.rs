use crate::spread::{compare_event, parse_prob, CompareRow, Quote, SpreadError};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct MapRow {
    event_id: String,
    venue: String,
    market_id: String,
}

#[derive(Debug, Deserialize)]
struct QuotesFile {
    quotes: Vec<QuoteRow>,
}

#[derive(Debug, Deserialize)]
struct QuoteRow {
    venue: String,
    market_id: String,
    yes: String,
}

pub fn compare_files(map_path: &Path, quotes_path: &Path, threshold: i64) -> Result<Vec<CompareRow>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(map_path)?;
    let mut key: HashMap<(String, String), String> = HashMap::new();
    for row in rdr.deserialize() {
        let row: MapRow = row?;
        key.insert((row.venue, row.market_id), row.event_id);
    }
    let quotes_file: QuotesFile = serde_json::from_str(&std::fs::read_to_string(quotes_path)?)?;
    let mut by_event: HashMap<String, Vec<Quote>> = HashMap::new();
    for q in quotes_file.quotes {
        if let Some(event_id) = key.get(&(q.venue.clone(), q.market_id.clone())) {
            by_event.entry(event_id.clone()).or_default().push(Quote {
                event_id: event_id.clone(),
                venue: q.venue,
                market_id: q.market_id,
                yes: parse_prob(&q.yes)?,
            });
        }
    }
    let mut rows = Vec::new();
    let mut ids: Vec<_> = by_event.keys().cloned().collect();
    ids.sort();
    for id in ids {
        let qs = &by_event[&id];
        if qs.len() < 2 {
            continue;
        }
        rows.push(compare_event(qs, threshold)?);
    }
    let _ = SpreadError::NeedTwoVenues; // keep type used if no rows
    Ok(rows)
}
