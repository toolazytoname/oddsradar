//! Read-only cross-venue prediction-market spread radar.

pub mod map;
pub mod secrets;
pub mod spread;

pub fn cli_name() -> &'static str {
    "oddsradar"
}
