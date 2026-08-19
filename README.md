# oddsradar

**English** · [中文](README.zh-CN.md) — plan: [docs/PROJECT-PLAN.md](docs/PROJECT-PLAN.md) · 学习: [learn/](learn/)

**Read-only** cross-venue prediction-market spread radar.

Watch the same event on Polymarket, Kalshi, and Hyperliquid outcome markets. When implied probabilities diverge past a threshold, notify. No betting, no custody, no automated orders.

> This is **not** a bookie and **not** an arb bot. It is a comparison table with alerts.

## Status

**v0.1 runtime (Rust 1.85).** One-shot compare of a hand-maintained market map + fixture quotes. Implied probability is integer millionths.

```bash
cd oddsradar
cargo test
cargo run -- compare --config fixtures/config.ok.json \
  --map fixtures/markets.csv --quotes fixtures/quotes_wide.json
# 真实行情：map 里填 Polymarket id / Kalshi ticker
cargo run -- compare --config fixtures/config.ok.json \
  --map fixtures/markets.live.example.csv --live
```

## What we will not do

- Place bets or outcome-contract orders
- Become a market / house
- Scrape in violation of a venue’s terms if a public API exists
- A directory of 200 unrelated tools

## License

MIT.

## Security

Read [`SECURITY.md`](SECURITY.md).
