# oddsradar

**English** · [中文](README.zh-CN.md) — plan: [docs/PROJECT-PLAN.md](docs/PROJECT-PLAN.md)

**Read-only** cross-venue prediction-market spread radar.

Watch the same event on Polymarket, Kalshi, and Hyperliquid outcome markets. When implied probabilities diverge past a threshold, notify. No betting, no custody, no automated orders.

> This is **not** a bookie and **not** an arb bot. It is a comparison table with alerts.

## Status

Scaffold. Spec is in `docs/`. No runtime yet.

## v0.1 (target)

- 10–20 high-liquidity events you actually care about
- One table or one Telegram bot
- Alert when spread exceeds a configured percent

## What we will not do

- Place bets or outcome-contract orders
- Become a market / house
- Scrape in violation of a venue’s terms if a public API exists
- A directory of 200 unrelated tools

## License

MIT.

## Security

Read [`SECURITY.md`](SECURITY.md).
