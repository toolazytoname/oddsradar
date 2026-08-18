# oddsradar

[English](README.md) · **中文** — 计划见 [docs/PROJECT-PLAN.md](docs/PROJECT-PLAN.md)

**只读**的预测市场跨所价差雷达。

同一事件在 Polymarket、Kalshi、Hyperliquid outcome 上的隐含概率差过大时告警。不下注，不托管，不自动下单。

> 这不是盘口，也不是套利机器人。是对照表 + 提醒。

## 状态

**v0.1 可运行。** `PYTHONPATH=. python3 -m oddsradar compare --config fixtures/config.ok.json --map fixtures/markets.csv --quotes fixtures/quotes_wide.json`

## 明确不做

- 下注或下 outcome 单
- 自己当庄
- 在已有公开 API 时违规爬页
- 做成 200 个工具的目录站

后续工作在这个文件夹里展开。先读 `docs/PROJECT-PLAN.md`。
