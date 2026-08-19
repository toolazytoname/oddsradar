# 学习模块 · oddsradar

先跑：

```bash
cd oddsradar
cargo test
cargo run -- compare --config fixtures/config.ok.json \
  --map fixtures/markets.csv --quotes fixtures/quotes_wide.json
cargo run -- compare --config fixtures/config.ok.json \
  --map fixtures/markets.live.example.csv --live
```

宽价差那次，`btc-100k` 应是 `kind: spread`，`fed-cut` 应是 `ok`。  
live 示例只有 Polymarket 一侧，所以会打出 `kind: quote`——你看见的是真实隐含概率，不是编的。

---

## 场景：预测市场在卖什么

不是在卖币，是在卖「这件事会不会发生」的份额。  
Yes 成交价 0.62，意思是市场隐含「会发生」的概率大约 62%（还有流动性、费用，别当成精确统计）。

同一件事可以同时在 Polymarket、Kalshi 上交易。两边价差大，要么是信息还没传过去，要么是规则其实不是同一件事（截止日期、标的定义差一个字就不算同一市场）。

本仓库**不下注**。它只做对照表。跨所「是不是同一事件」没有统一 ID，所以用你手维护的 CSV 当真相——这是脏活，也是诚实。

---

## 知识点 → 代码落点

| 词 | 人话 | 落在哪 |
|---|---|---|
| 隐含概率 | 价格当成 P(yes) | `parse_prob`，单位 millionths，`1_000_000 = 100%` |
| 价差 | 各所 Yes 价的 max − min | `spread_millionths` |
| 市场映射 | 人说「是同一件事」 | `fixtures/markets.csv` 的 `event_id` |
| CLOB / 盘口 | 挂单簿，不是 AMM | Polymarket Gamma 的 `outcomePrices` |
| Kalshi 报价 | 有的接口是美元字符串，老接口是 0–100 美分 | `normalize_kalshi_price` |

Polymarket 的坑：`outcomePrices` 常常是**字符串包着 JSON 数组** `"[\"0.0445\",\"0.9555\"]"`，不是真数组。见 `src/live.rs` 的 `fetch_polymarket`。第一次对接几乎人人踩。

Kalshi 的坑：`last_price_dollars = "0.0000"` 经常只是「还没成交」，不能当概率；要退到 `yes_ask_dollars`。

---

## 设计

- **映射表是产品，不是权宜之计。** 自动对齐全世界选举市场，听起来美，做错一次就是假套利。手工 CSV 是在承认语义不在链上。
- **一种内部单位。** 不管盘口给 0.62、620000 还是 62 美分，进引擎前都变成 millionths。比较只做整数减法。
- **live 和 fixture 共用 `compare_event`。** 网只负责取字符串。
- **单所也输出 `quote`。** 学习阶段你往往先接上一边；闷声没输出会让人以为程序坏了。

精读：`src/spread.rs` 的 `parse_prob`（两种输入约定）、`src/live.rs` 里对两个 API 脏形状的处理。

---

## 动手

1. 把 `quotes_wide.json` 里 polymarket 的 `0.62` 改成 `0.51`，spread 应变 `ok`。
2. 在 `markets.live.example.csv` 给 `xi-2027` 加一行你认为对应的 Kalshi ticker，再 `--live`，看会不会变成 `spread`。加之前先确认**结算规则真的一样**。
3. 读一条 Polymarket 原始 JSON，自己数 `outcomePrices` 为什么是字符串。

---

## 故意没做

自动套利、下单、把「全世界事件」做成知识图谱。那些是交易公司，不是学习雷达。
