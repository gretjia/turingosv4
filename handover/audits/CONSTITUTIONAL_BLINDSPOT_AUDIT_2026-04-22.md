# 宪法盲点审计 (Constitutional Blindspot Audit) — 2026-04-22

**Auditor**: auditor subagent (read-only mode per `.claude/agents/auditor.md`)
**Driver**: 2026-04-22 PPUT (C-052) 事件暴露"宪法强制指标被口语忘掉"的结构性问题。扫描宪法全条找出其他类似盲点。

---

## § 1. 盲点表 (20 条)

| # | 条款 | 盲点 | 状态 | 严重度 | 证据 | 建议判例 |
|---|---|---|---|---|---|---|
| B-01 | Art. I.2 信誉累积 | 没有"方案被他人调用次数"计数器（宪法明文："统计某个 Agent 提出的方案在后续流程中被其他 Agent 成功调用的总次数"）| 完全缺失 | **Critical** | `grep -rn "citation.count\|cite_count\|reputation" src/` → 0 匹配；`ledger.rs` 只有 `reverse_citations: Vec<NodeId>` 但不按 author 聚合 | C-053 |
| B-02 | Art. I.2 效用评分 | 没有"期望/方差"体检报告（仅 WalletTool.balance 这一个标量）| 完全缺失 | High | `grep -rn "mean\|variance\|std_dev" src/` → 0；`bus.snapshot().balances == HashMap::new()` | C-054 |
| B-03 | Art. I.2 共识提取 | 没有众数/中位数抽取模块（多 Agent 独立出不同答案时的收敛信号）| 完全缺失 | High | `grep -rn "mode\|median\|consensus" src/` → 0；`omega_payload_hashes` 只数 uniqueness，不求 mode | C-054 |
| B-04 | Art. II.1 典型错误阈值 | TopKClasses 广播没有频率门槛；1 个错误就广播 ≈ 和 100 个错误一样 | 部分实现 | **High** | `bus.rs:519-537` 直接对所有 count 排序取 top-K，没有 `count ≥ threshold` 过滤；宪法明文要求"多个 Agent 都在同一个地方跌倒"才抽象 | C-055 |
| B-05 | Art. II.1 全局架构文档更新 | "抽象后的规则广播"只存活在 bus 进程内存，未更新任何文档（宪法步骤 2："更新全局架构文档"）| 部分实现 | Medium | `graveyard: HashMap` 在 `bus.rs:48`，无持久化；`librarian.rs` 写 agent 个人 memory，不写全局架构文档 | C-055 |
| B-06 | Art. II.2 价格时间序列 | 只有瞬时 `market_ticker` 快照，无价格演化 history；agent 永远看"当下"，看不到"涨/跌" | 完全缺失 | High | `bus.rs:554-565` `market_ticker()` 每次从 `markets` 即时算；`grep -rn "price_history" src/` → 0 | C-056 |
| B-07 | Art. II.2 价格驱动行为验证 | 没有 metric 测价格信号是否真的驱动 agent 决策（只数 `invest` 次数，不数 "invest 的 node 是否在 top-K price"）| 完全缺失 | **High** | `evaluator.rs:680` 只记 `tool_dist["invest"] += 1`；无"决策 ↔ 价格"相关性 metric | C-056 |
| B-08 | Art. II.2.1 探索/利用 metric | Boltzmann T 是配置量（非 metric）。没有输出层"实际选择分布"的熵指标监控 | 部分实现 | Medium | `actor.rs:16` `temperature: 0.5`；无 `entropy(chosen_parents)` 或"top-1 dominance rate"指标 | C-057 |
| B-09 | Art. II.2.1 TEMP_LADDER 孤岛 | 温度阶梯只在 evaluator 实现，是 env 开关；不是宪法级机制，agent 看不见温度；不可跨实验复现 | 部分实现 | Low | `evaluator.rs:356,501-506` 硬编码 `(0.10 + i*0.15).min(1.30)`；无 `src/` 模块封装 | C-057 |
| B-10 | Art. III.1 GC 之外屏蔽 | 除 tape 园丁缺失（C-045）外，**Librarian 学过的 learned.md 永不被 GC/校验**；过时学习沉淀为永久技术债 | 完全缺失 | Medium | `librarian.rs:62-67` 直接 `fs::write` 覆盖，无 TTL / 版本；陈旧错误模式会长期污染"学到的 skill" | C-058 |
| B-11 | Art. III.2 按需加载 metric | 无 "每 agent 查阅的 doc 数量 / 不同 agent 查阅分布" 监控 | 完全缺失 | Medium | `search_cache/search_count` 存在于 `evaluator.rs:371,376` 但从不进 PputResult；无法回答"agent 真的在按需加载 vs. 不在加载" | C-058 |
| B-12 | Art. III.3 相关性 metric | 无 pairwise 相似度（Jaccard / BLEU / embedding cosine）于 agent 输出之间；只有全局 `unique_payload_ratio` | 部分实现 | **High** | `evaluator.rs:62,874` 只有 `|unique| / |total|`。这是"全局去重率"不是"两两相关性"。N=8 TEMP_LADDER 全是 hash-unique 也可能高度同质（改个空格就 hash 不同）| C-059 |
| B-13 | Art. III.4 Goodhart 逆向探测 detector | 无检测器识别"agent 是否在尝试逆向度量"（如：同一 agent 反复提交 payload 仅微变以探测 oracle 边界）| 完全缺失 | High | `grep -rn "goodhart\|reverse_engineer\|probe" src/` → 0；无 per-agent "oracle query rate / unique-payload-per-query" 异常监控 | C-060 |
| B-14 | Art. IV q-halt 状态机 | 没有显式 `q ∈ {run, halt}` 机器状态；没有 `EventType::Halt` 事件；`halt_and_settle()` 只是一个函数 | 完全缺失 | **Critical** | `ledger.rs:148-160` `EventType` 里有 RunStart/RunEnd/OmegaAccepted，**没有 Halt**；`grep -rn "q_halt\|q_state" src/` → 0；宪法 Art. IV mermaid 图明确 `q=halt ⇒ HALT 双圈终态` | C-061 |
| B-15 | Art. IV Tape 大小无界 | Tape 可无限增长，无 cap 也无 per-run 硬上限；崩溃恢复时 WAL 整文件回放（O(N²)）| 完全缺失 | Medium | `ledger.rs:31-44` `Tape.nodes: HashMap` 无 cap；`wal.rs:70-94` `replay` 一次性读全文件；宪法 Art. V.2 明确要求"总算力上限 $10000$" | C-062 |
| B-16 | Art. IV WAL 一致性每 tick 检查 | WAL 写入后无 periodic replay-整性 checksum；ledger `verify()` 有 hash chain 但从不在 runtime 调用，只在 test 里调 | 部分实现 | Medium | `ledger.rs:255-298` `verify()` 存在但 `grep -rn "ledger.verify()" src/ experiments/` 仅命中 test；runtime 永不校验 | C-062 |
| B-17 | Art. V.1 ArchitectAI quota | `.claude/agents/proposer.md` 纯 prompt，无 "每 week 最多 N 次提议" 限制；无提议计数 | 完全缺失 | High | `.claude/agents/proposer.md` 无 rate config；`routines/` 只有 daily_drift（JudgeAI-advisory）；无 proposer routine 也无 quota | C-063 |
| B-18 | Art. V.2 总算力消耗上限 | 宪法示例列出"$10000$ 总算力"，代码无任何 cumulative compute counter | 完全缺失 | High | `evaluator.rs:349` `max_transactions = 200` 是 **per-problem** 硬编码；无全局 budget；`grep -rn "compute_cap\|TOTAL_COMPUTE" .` → 0 | C-064 |
| B-19 | Art. V.2 24h 结果 | "必须在 24 小时内给出结果" 无执行；单问题超时从未产生 HALT，只让 run 跑到 `max_transactions` | 完全缺失 | Medium | 无 wall-clock cap 在 run_swarm；`Instant::now()` 只用于测 PPUT 分母，不用于强制终止 | C-064 |
| B-20 | Art. V.2 可逆性 | "任何状态变更必须具有可逆性（总是能够回滚到 Q_{t-1}）" 无实现；tape append-only 是"不忘"，不是"可回滚" | 完全缺失 | **Critical** | `ledger.rs:27-35` 明确注释 "NEVER modified or removed"；无 snapshot/checkpoint 原语；宪法原文"总是能够回滚到 Q_{t-1}"被误读成"历史不变"而落空 | C-065 |

---

## § 2. 立判例建议 (13 条, 按严重度)

### Critical (必立，阻塞级)

**C-053: 信誉累积计数器缺失 (Art. I.2)**
Ruling: "被他人调用次数"是 Art. I.2 明文要求的三大统计信号之一，必须作为 per-agent 累积标量暴露给 top management + agent snapshot。实现：`Tape` 级增加 `reputation_by_author: HashMap<String, u32>`，每次 `append` 把所有 `citations` 的 author 的 count +1；进入 `UniverseSnapshot`；PputResult 增加 `reputation_at_end` 字段。

**C-061: q-halt 状态机缺失 (Art. IV)**
Ruling: 宪法 Art. IV mermaid 图明确 `q ∈ {run, halt}` 是 Q_t 的三元组成员之一（`Q_t = ⟨q_t, HEAD_t, tape_t⟩`）。实现：`Ledger` 新增 `EventType::Halt { reason: HaltReason }`，其中 `HaltReason` = enum (OmegaAccepted / MaxTxExhausted / WallClockCap / ComputeCapViolated / ErrorHalt)；`bus.generation` 升级为 `q_state: QState` 显式字段。

**C-065: 可逆性原语缺失 (Art. V.2)**
Ruling: 宪法 Art. V.2 的"总是能够回滚到 Q_{t-1}"不等于"tape append-only"。实现：WAL 已经逐 tick 持久化 Q_t；增加 `Wal::rewind_to(seq: u64)` 原语 + `ledger.snapshot_at(seq)`。

### High

**C-055**: Art. II.1 "典型错误"误读为"所有错误广播" — 引入 `MIN_CLASS_COUNT_TO_BROADCAST`（env, 默认 3）+ 全局架构文档更新路径
**C-056**: Art. II.2 价格时间序列 + 驱动行为 metric — per-market ring buffer + PputResult 增加 `price_tracking_correlation`
**C-059**: Art. III.3 pairwise 解相关 metric — 每 tick 采样最近 K 个 OMEGA payload 两两 Levenshtein / Jaccard；`pairwise_diversity_mean` + `pairwise_diversity_min`
**C-060**: Art. III.4 Goodhart 逆向探测 detector — per-agent `(oracle_reject_rate, unique_payload_rate)` 双维度指纹
**C-063**: Art. V.1.2 ArchitectAI quota — `.claude/agents/proposer.md` 增加"最多 1 次/week" 限制，存档到 `handover/directives/YYYY-WW/`
**C-064**: Art. V.2 compute + 24h 上限 — `COMPUTE_BUDGET_SECONDS`（默认 86400）+ `MAX_TX_GLOBAL`（默认 10000）；超限产生 `EventType::Halt`

### Medium

**C-054**: Art. I.2 效用评分 + 共识提取（期望/方差/众数/中位数）
**C-057**: Art. II.2.1 探索熵 metric — TEMP_LADDER 升级为 `src/sdk/exploration.rs`；`parent_selection_entropy` 进 PputResult
**C-058**: Art. III.1+III.2 学习文档 GC + 按需加载审计 — Librarian learned.md 加 TTL；`search_count` 进 PputResult
**C-062**: Art. IV tape 大小 + WAL runtime 校验 — `MAX_TAPE_NODES`（默认 1000）+ bus 每 tick 调 `ledger.verify()`

---

## § 3. 应升入 CLAUDE.md Report Standard（PPUT 级升格）

### 升格 #1: 信誉累积 (C-053)
> CHECKPOINT / LATEST 必须同时列 `reputation_distribution`（per-agent 被引用计数 p50/p90/max）。Art. I.2 三大信号缺一即违宪。

### 升格 #2: q-halt 显式终态 (C-061)
> 所有 run 级汇报必须列 `halt_reason_distribution`（OmegaAccepted / MaxTxExhausted / WallClockCap / ComputeCapViolated / ErrorHalt）。只汇报 solve rate + PPUT 不足以让读者区分"未尝试"和"尝试失败"。

### 升格 #3: 探索熵 + pairwise 多样性 (C-057+C-059)
> 多 agent 实验（n≥2）必须列 `parent_selection_entropy` 和 `pairwise_payload_diversity_mean`；任一低于 0.25 即 Art. II.2.1 "过度利用"告警。

---

## § 4. 与外审 C-044~C-051 的交集

20 盲点中 **17 条独立新增**、3 条正交递进：
- B-10 ↔ C-045 (tape gardener / librarian learned.md gardener 姊妹)
- B-13 ↔ C-051 (Goodhart 保密退化 / 探测器)
- B-17 ↔ C-044 (Art. V 未实现 / ArchitectAI quota 细化)

**外审 C-044~C-051 与本审 0% 直接重叠**。外审抓"mechanism 实现错误"，本审抓"mechanism 无监控"，互补。

---

## § 5. 关键文件路径

- `constitution.md` (宪法全文)
- `src/ledger.rs:27-35,148-160` (append-only + EventType 无 Halt)
- `src/bus.rs:494-537` (TopKClasses 无阈值)
- `src/bus.rs:554-565` (market_ticker 无历史)
- `src/sdk/snapshot.rs:22-30` (UniverseSnapshot 无 reputation)
- `experiments/minif2f_v4/src/bin/evaluator.rs:349` (max_transactions 硬编码)
- `experiments/minif2f_v4/src/bin/evaluator.rs:56-62,874` (unique_payload_ratio 为唯一多样性)
- `.claude/agents/proposer.md` (无 quota)
- `cases/C-052_pput_as_sole_metric.yaml` (基线判例)
