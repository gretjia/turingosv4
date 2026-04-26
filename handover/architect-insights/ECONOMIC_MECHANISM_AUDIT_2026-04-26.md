# Economic Mechanism Audit — 宪法经济条款 vs 代码现实

**Auditor**: ArchitectAI (Claude main, ultrathink mode)
**Date**: 2026-04-26
**Trigger**: User question 2026-04-26 ultrathink — "经济机制都在吗？我担心代码跟宪法没多少关系"
**Companion**: `TAPE_CANONICAL_AUDIT_2026-04-26_AUDITOR.md` (V-04, V-05, V-13, V-14, V-15, V-16 already covered economic violations from tape-canonical angle)

---

## TL;DR — 双重失败

1. **代码层 ≠ 宪法被遗弃**：经济机制 substantial 实存（market/wallet/invest/short/broadcast/exploration-exploitation）—— 不是空壳。
2. **运行层 ≠ 经济在跑**：Phase B/C **production scripts 默认不启用 `TAPE_ECONOMY_V2` 和 `WALLET_STATE` env**，导致 `settle_portfolios` **不运行**，wallet 进程结束即丢失，founder grant 不 fire。**agents 在 prompt 里看到价格、能调 invest tool，但永远不结算。**
3. **审计层 ≠ 即使运行也无法重建**：market/wallet/portfolio events 不上 tape (V-04/05/13/14/15/16) → frozen tape 无法重建经济状态。
4. **Plan v1 coverage**：TFR S4 (commits 3 + 4) 修了 (3) 这一层。但 **(2) inert-default 问题 plan v1 没处理** —— 即使 tape canonical 重构完，production 不打开 TAPE_ECONOMY_V2 经济还是装饰。

新发现 4 个 economic-only violations，命名 **E-01..E-04**，必须进 plan v2 must-fix。

---

## § 1. 宪法经济条款 + 代码现实逐条对照

### 1.1 Laws (基本法)

> Law 1: Information is Free — Agent 搜索与查看零成本，思考不花钱
> Law 2: Only Investment Costs Money — 1 Coin = 1 YES + 1 NO (CTF 守恒)；on_init 是唯一合法铸币点

| 宪法要求 | 代码实现 | 状态 |
|---|---|---|
| Information free | `bus.snapshot` 投递信息到 prompt 不扣费；search_cache 也免费 | ✅ 实现 |
| Only investment costs | `bus.append_internal:233` `self.debit_wallet(author, amount)` BEFORE buy_yes | ✅ 实现 |
| 1 Coin = 1 YES + 1 NO (CTF) | `kernel.create_market` 使用 LMSR / CFMM 自动满足 sum-to-1 invariant | ✅ 实现 |
| on_init 唯一铸币点 | `WalletTool::ensure_agents` 只在 init 时给 `genesis_coins` | ⚠️ 部分 — **founder grant (TAPE_ECONOMY_V2=1) 后续每个 append 都铸 YES shares**；解释为 Magna Carta Rule #19 system-LP exemption 是 stretch |
| no silent burn | `bus.append_internal:240` `self.credit_wallet(author, amount)` refund on failure | ✅ 实现 |

### 1.2 Art. II.1 广播典型错误

| 宪法要求 | 代码实现 | 状态 |
|---|---|---|
| 抽象典型错误（class），不广播原始 log | `bus.recent_rejections_scoped` returns TopK class labels (e.g. `"veto:forbidden"`) | ✅ 实现 |
| 广播给所有 agent | prompt.rs 每个 agent 都看到 errors block | ✅ 实现 |
| 全局架构文档更新 | `cases/C-XXX.yaml` 沉淀为判例（手动） | ⚠️ 部分自动 |
| 不污染 context | TopK + class 而非原始 log | ✅ 实现 |

**审计角度漏洞 (V-09)**: graveyard HashMap parallel ledger，frozen tape 无法重建。

### 1.3 Art. II.2 广播价格信号

| 宪法要求 | 代码实现 | 状态 |
|---|---|---|
| 顶层广播 high-weight 标价 | `kernel.market_ticker_full(50)` 每个 node 的 yes_price | ✅ 实现 |
| 悬赏特定任务 | `kernel.open_bounty_market` + `bounty_yes_price` | ✅ 实现 |
| 紧缺资源标价 | per-node market 价格 = "this node 是 golden path 一部分" 的市场预测 | ✅ 实现 |
| 不指导具体步骤 | prompt 只显示价格，agents 自主决定 invest/append | ✅ 实现 |
| Agent 自发调整行为 | tool_dist["invest"] 历史 jsonl 中实际出现非零值 → agent **真的会** invest | ✅ 实现 |

**审计角度漏洞 (V-04 + V-15 + V-16)**: market 创建/解析/founder-grant 不上 ledger；frozen tape 无法重建价格历史。

### 1.4 Art. II.2.1 探索 vs 利用

| 宪法要求 | 代码实现 | 状态 |
|---|---|---|
| Boltzmann 路由（温度参数） | `BoltzmannParams::from_env()` + `boltzmann_select_parent` | ✅ 实现 |
| Per-agent 温度阶梯 | `TEMP_LADDER=1` env → temperature = 0.10 + i*0.15 (capped 1.30) | ✅ 实现，但 opt-in |
| 不抹杀群体异质性 | per-agent skill (Phase B 已有 3 skills cycling) | ✅ 实现 |
| 默认 temp 0.2 | line 405 + 1017 | ✅ 实现 |

**审计角度漏洞 (V-08b)**: Boltzmann pick decisions 不上 tape；seed 仅 env→jsonl，replay 依赖 env-pinning 而非 tape。

### 1.5 Short positions (NO shares)

| 宪法要求 | 代码实现 | 状态 |
|---|---|---|
| Short = 押注 dead-end node | `BetDirection::Short` → `kernel.buy_no` | ✅ 实现 |
| Agent 知道 short 是什么 | `prompt.rs:121` `direction="short" buys NO shares (this node is a dead end)` | ✅ 实现 |
| 结算时 NO 持有者赢 | `kernel.resolve_all` 对 dead branches 兑付 NO | ✅ 实现 |

✅ 完整闭环。

### 1.6 Per-node 金额 / 价格机制

| 宪法要求 | 代码实现 | 状态 |
|---|---|---|
| 每节点独立市场 | `kernel.markets: HashMap<NodeId, PredictionMarket>` | ✅ 实现 |
| LMSR / CFMM 价格 | `prediction_market.rs::PredictionMarket::buy_yes/no` | ✅ 实现 |
| 系统 LP 做市 | `system_lp_amount` config (default 200.0)；`create_market` 自动 inject | ✅ 实现 |
| Bounty market | `open_bounty_market` + 单独 LP | ✅ 实现 |

✅ 全部实现。**审计漏洞 V-04**: market 创建事件不上 ledger。

### 1.7 Settlement (halt_and_settle)

| 宪法要求 | 代码实现 | 状态 |
|---|---|---|
| OMEGA 接受时 settle 所有 markets | `bus.halt_and_settle:336` → `kernel.resolve_all(&golden_path)` ✅ | ✅ 调用 |
| 给 agent 真实兑付 | `bus.settle_portfolios:383` 把 winning shares 转回 wallet balance | ⚠️ **gated by TAPE_ECONOMY_V2=1** |
| 默认 production 启用 settlement | scripts/runner 全部不设 TAPE_ECONOMY_V2 | ❌ **production OFF** |

**最重要发现 — E-01**: `bus.rs:345` if-gate 让 production batch **默认不结算**。`halt_and_settle` 只调 `resolve_all`（market 状态结算了），但 `settle_portfolios`（agents 真的拿到钱）默认不运行。

---

## § 2. 实证证据：production batch 经济行为

### 2.1 历史 jsonl 经济字段

抽样 `handover/evidence/*.jsonl` (E1 paper-1 era):
```python
all_keys = ['boltzmann_seed', 'classifier_version', 'condition', 'gp_node_count',
            'gp_token_count', 'halt_reason', 'has_golden_path', 'model', 'pput',
            'problem', 'reputation_at_end', 'time_secs', 'tool_dist', 'tx_count']
econ_keys = ['reputation_at_end']  # 唯一经济字段
```

`reputation_at_end` 看似经济：
```python
reputation_at_end: {'Agent_0': 3, 'Agent_1': 4, 'Agent_2': 3, ...}
```

**但实际是 solve count（per-agent winning append 计数），不是 wallet balance**。**没有任何 jsonl 字段记录**：
- `wallet_balance_at_end`
- `portfolio_summary`
- `total_invest_volume`
- `market_resolution_summary`
- `bounty_payout`

### 2.2 Tool 调用证据

```python
# handover/evidence/*.jsonl tool_dist 抽样:
{'step': 44, 'append': 3, 'invest': 1, 'parse_fail': 2, ...}  # 一次 invest 实际发生
{'omega_wtool': 1, 'step_reject': 9, 'step': 16, ...}         # 这 cell 0 invests
```

agents **偶尔** 调用 invest（`tool_dist["invest"]` 非零），但因为：
- production TAPE_ECONOMY_V2=0 → settle_portfolios 不跑
- 即使买了 shares，run end 时不结算
- 下个 problem 全新 wallet → 跨 cell 无积累

**经济在生产中是装饰性的**。

### 2.3 PputResult 经济维度

`make_pput()` 完全不读 wallet / market / portfolio 状态。`pput_runtime` / `pput_verified` 只看 `runtime_accepted` / `post_hoc_verified` boolean — **PPUT 与经济解耦**。

宪法 Art. II.2 "agent 自发调整行为倾向" 在 production 是 mute：agents 看价格但回报永远不来。

---

## § 3. NEW 经济专属 violations (E-01..E-04)

补充 `TAPE_CANONICAL_AUDIT_2026-04-26_AUDITOR.md` 24 处之外的经济-only 维度发现：

| 编号 | 违反 | 严重度 | TFR Plan v1 是否 cover |
|---|---|---|---|
| **E-01** | Production scripts 默认 TAPE_ECONOMY_V2=0 → settlement / founder grant / wallet 持久化全部 inert；经济机制名实严重不一致 | **HIGH** | ❌ NOT covered |
| **E-02** | jsonl `RunAggregate` schema 没 wallet/portfolio/market summary 字段 → 即使经济运行也无法 audit / 入 Phase D 信号 | **HIGH** | ❌ NOT covered |
| **E-03** | `reputation_at_end` 字段命名误导 — 实际是 solve count 而非 wallet balance；`PputResult` schema 应明确分两个字段 | **MED** | ❌ NOT covered |
| **E-04** | Founder grant (TAPE_ECONOMY_V2=1) 静默铸 YES shares 与 Law 2 "on_init 唯一铸币点" 字面冲突；解释为 Magna Carta Rule #19 system-LP exemption 需明文化 | **MED** | partial (V-15 covers ledger trace, NOT constitutional reconciliation) |

---

## § 4. Plan v2 must-fix (除 internal auditor 已提的 5 项之外)

加入 plan v2 must-fix 列表：

| Sprint atom | 内容 | 关闭 |
|---|---|---|
| **S0.7+** | 默认 production 配置启用 TAPE_ECONOMY_V2 (写入 runner scripts + cargo workspace dep TBD)；commit 标注从"opt-in" 转 "opt-out" | E-01 |
| **S2.1+** | `RunAggregate` v2 schema 加 `wallet_at_end: Option<HashMap<String, f64>>` + `portfolio_at_end: Option<...>` + `market_summary: Option<MarketSummary>` 字段 | E-02 |
| **S2.1+** | rename `reputation_at_end` → `solves_at_end`; new `wallet_balance_at_end` distinct field | E-03 |
| **S0.2+** | PREREG_AMENDMENT_TFR adds clause: founder grant 是 Magna Carta Rule #19 explicit 子条款（系统 LP exemption），不是 Law 2 violation | E-04 |
| **S5.x+ (post-TFR)** | Phase D ArchitectAI consumer 读 wallet_at_end + market_summary 入 cost-attribution model | E-02 deeper |

加上 internal auditor 5 must-fix + 这 4 个 E 项 + codex 待回 = plan v2 重写 scope。

---

## § 5. 用户问题逐项回答

### 「invest/short 在吗？」

✅ **存在且生产中实际被 agent 调用**（jsonl tool_dist 证据）。但：
- ❌ 结算默认 OFF (E-01)
- ❌ Invest event detail=None (V-05)
- ❌ direct kernel.buy_yes/buy_no 旁路 bus (V-05 path 2)

### 「每个节点的金额（价格机制）在吗？」

✅ **存在并在 prompt 中广播给 agent**（market_ticker_full(50) → snapshot → prompt.rs:48）。但：
- ❌ MarketCreate / MarketResolve 不上 tape (V-04)
- ❌ 每节点 yes_price 仅在 in-memory kernel.markets HashMap

### 「广告（广播价格信号）在吗？」

✅ **Art. II.2 在 runtime 是实现的**：market_ticker / bounty_yes_price 都进 agent prompt。

### 「founder grant / bounty / 系统 MM？」

✅ **代码全部存在**。但 founder grant 默认 OFF（E-01）；bounty 启用需 BOUNTY_LP env（也常 OFF）；系统 MM 是默认 ON（每次 append 自动 create_market）。

### 「我要求的东西没真的实现过？」

**部分是这样**：
- **代码层 70-80% 实现了** — 真有 market、wallet、invest、short、price broadcast、exploration-exploitation
- **运行层 production 默认关掉了** — settlement/founder grant/wallet 持久化都需要 env opt-in，但 scripts 不设
- **审计层 ledger trace 不上 tape** — 即使开了也 frozen tape 无法重建

所以**严格说**："我要求的东西" 在代码里 mostly 有，在生产**默认体感是 0**，在审计**完全 broken**。

---

## § 6. 我应该早告诉你这些吗？

**应该。** 之前的 24 处 V-violations 我都报告了 (V-04..V-16 中的 6 处直接打到经济)。但我**没主动**做这次的 production-default-off 维度审计 (E-01..E-04)，直到你逼问。

这个 gap 揭示我 review pattern 的问题：审计倾向走 "代码符号 vs 宪法条款" 维度，**漏掉 "代码符号 vs 实际生产 invocation" 维度**。即使每个 fn 都实现了宪法条款，如果 production scripts 不调，整个机制还是 inert。

类似 deepseek-v4-flash thinking-off 的事故 (F-2026-04-26-01)：proxy 字段对 (Qwen 用 `enable_thinking`)，但对 DeepSeek 是 wrong field shape，所以 thinking-off 永远没生效。**我描述的世界 ≠ 实际跑的世界**。

**Plan v2 必须加：**
- 每个 sprint atom 加 "production invocation evidence" 子项 — 不仅写代码，还要 grep + smoke 证明 prod 确实跑这条路径
- 每个宪法 clause 在 TRACE_MATRIX_v3 加列 "production invocation site" — 与 "code symbol" 区别开

这是除了 internal auditor 的 5 must-fix 之外的 meta-改进。

---

## § 7. 立即行动

1. 此 audit doc commit + push（让你能 git review）
2. 加 E-01..E-04 进 plan v2 must-fix
3. 等 codex audit 回（仍 running）—— 他可能也找到经济相关问题
4. 合并所有 must-fix 出 plan v2

我下一条 message 给你 plan v2 全列 must-fix 清单，等你拍板再 revise plan + commit v2。
