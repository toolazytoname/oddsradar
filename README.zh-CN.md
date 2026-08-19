<p align="center">
  <img src="learn/assets/cover.jpg" alt="oddsradar：同一事件在两家预测市场上的价格对照" width="880">
</p>

<h1 align="center">oddsradar</h1>

<p align="center">
  <strong>只读的预测市场跨所价差雷达。</strong><br>
  同一件事，几家盘口，一张对照表。不下注。
</p>

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.zh-CN.md"><strong>中文</strong></a> ·
  <a href="learn/README.md">学习</a> ·
  <a href="docs/PROJECT-PLAN.md">计划</a> ·
  <a href="SECURITY.md">安全</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-1F6FEB" alt="version 0.1.0">
  <img src="https://img.shields.io/badge/rust-1.85-DEA584" alt="Rust 1.85">
  <img src="https://img.shields.io/badge/license-MIT-0B6E4F" alt="MIT license">
  <img src="https://img.shields.io/badge/mode-read--only-111827" alt="只读">
</p>

---

同一事件在 Polymarket、Kalshi、Hyperliquid outcome 上的隐含 Yes 概率差过大时，打印一行 `spread`（也可追加到 JSONL）。

> 这不是盘口，也不是套利机器人。是对照表 + 提醒。要不要自己去两边下，是你的决定、你的账户。

## 为什么做这个

预测市场卖的是「这件事会不会发生」的份额。Yes 成交价 0.62，粗看是 62% 的隐含概率——还有流动性和费用，别当成精确统计。

同一条新闻可以同时在多家交易。价差大，要么是信息还没传过去，要么规则其实不是同一件事（截止日期、标的定义差一个字就不算）。世界上没有统一的事件 ID，所以**映射表由你手维护**。这是脏活，也是诚实的部分。

## 能力

| | |
|---|---|
| **手工映射** | CSV `event_id,venue,market_id`。语义在你这边。 |
| **整数概率** | millionths（`1_000_000 = 100%`）。比较只做整数减法。 |
| **fixture 与 live 同一引擎** | 网络层只负责取字符串。 |
| **单所也输出** | 只有一家有报价时打 `kind: quote`，半接好的 map 不会假装程序坏了。 |
| **只用公开 API** | Polymarket Gamma + Kalshi REST。有官方接口就不爬 HTML。 |

## 怎么工作

<p align="center">
  <img src="learn/assets/architecture.svg" alt="oddsradar 架构：市场映射加上 fixture 或 live 报价，进入 parse_prob 与 compare_event" width="880">
</p>

| `kind` | 含义 |
|---|---|
| `spread` | 至少两所，`max(yes) − min(yes)` ≥ 阈值。 |
| `ok` | 至少两所，价差未超阈。 |
| `quote` | 这个 `event_id` 只有一所报了价。 |

默认阈值 `50000` millionths（5 个百分点），可用 `--threshold` 或 `config.threshold_millionths` 覆盖。

## 环境

- [Rust](https://www.rust-lang.org/tools/install) **1.85**（`rust-toolchain.toml` 钉死）
- `--live` 需要能访问 Polymarket / Kalshi

```bash
git clone https://github.com/toolazytoname/oddsradar.git
cd oddsradar
cargo test
```

## 快速开始

**Fixture（离线）：**

```bash
cargo run -- compare \
  --config fixtures/config.ok.json \
  --map fixtures/markets.csv \
  --quotes fixtures/quotes_wide.json
```

`btc-100k` 应为 `kind: spread`；`fed-cut` 应为 `ok`。

**Live（公开 API，仍然不下单）：**

```bash
cargo run -- compare \
  --config fixtures/config.ok.json \
  --map fixtures/markets.live.example.csv \
  --live
```

示例 map 只有 Polymarket 一侧，所以会看到 `kind: quote` 和真实隐含概率——不是编的。在确认**结算规则真的一样**之前，不要把两行强行并成同一个 `event_id`。

## 映射格式

`fixtures/markets.csv`：

```csv
event_id,venue,market_id
btc-100k,polymarket,pm-btc-100k
btc-100k,kalshi,kx-btc-100k
btc-100k,hyperliquid,hl-btc-100k
```

Live 时 Polymarket 的 `market_id` 是 Gamma 数字 id（见 `fixtures/markets.live.example.csv`）。Kalshi 用 ticker。

## 命令

| 命令 | 作用 |
|---|---|
| `doctor --config FILE` | 拒绝密钥字段名；打印阈值。 |
| `compare --config FILE --map FILE --quotes FILE` | 离线对照。 |
| `compare --config FILE --map FILE --live` | 拉公开接口。 |
| `compare … --threshold N --notify-file PATH` | 覆盖阈值；把 `spread` 行追加成 JSONL。 |

## 测试

```bash
cargo test
```

live 解析用录下来的官方 JSON（`fixtures/polymarket_market.json`）钉死。字段改名时测试先红，而不是默默返回 0。

## 安全

请读 **[`SECURITY.md`](SECURITY.md)**。本进程不得持有交易凭证或钱包种子。有官方 API 就不要爬页。通知 token（以后加的话）放在 `chmod 0600` 的 `.env`，不进 git。

## 明确不做

- 下注或下 outcome 单
- 自己当庄
- 自动对齐全世界选举市场（对错一次就是假套利）
- 做成 200 个工具的目录站

## 学习

[`learn/`](learn/) 讲隐含概率、Polymarket `outcomePrices` 被包成字符串的坑，以及 Kalshi `last_price_dollars = "0.0000"` 常常只是「还没成交」。封面动画：[`learn/assets/cover.mp4`](learn/assets/cover.mp4)。

## 相关

- [hlsentry](https://github.com/toolazytoname/hlsentry) — Hyperliquid 清算哨兵
- [chaintail](https://github.com/toolazytoname/chaintail) — 本机 EVM 日志尾巴
- [x402-stall](https://github.com/toolazytoname/x402-stall) — 给独特数据用的 HTTP 402 收银台

## 许可

[MIT](LICENSE) © 2026 toolazytoname
