# Claude 内部审计核心结论 — 供外部审计对照

**日期**：2026-04-21
**commit**：`e0a75ec`
**5 路 sub-agent 并行**：auditor (constitutional alignment) + Explore×3 (topology / Karpathy / DAG) + proposer (launch roadmap)

---

## § 1. 宪法对齐判决摘要

| 条款 | 判决 | 关键证据 |
|---|---|---|
| Art. I.1 布尔谓词 | 🟡 部分合规 | 谓词散在 `bus.rs:182-208` + `lean4_oracle.rs:57-117`，无统一入口表 |
| Art. I.1.1 PCP | ✅ | F-20-05 三层封堵：`lean4_oracle.rs:113` + `bus.rs:182` + `routines/daily_drift.yaml` |
| Art. I.2 统计信号 | 🟡 | 信誉计数器未作一等信号 |
| Art. II.1 广播 | ✅ 语义 | TopKClasses 默认开启 (`bus.rs:495`) |
| Art. II.2 价格 | ✅ | Hayek bounty `kernel.rs:63-103` |
| Art. II.2.1 探索/利用 | 🟡 | TEMP_LADDER 仍 env-gated |
| **Art. III.1 园丁 GC** | 🔴 **缺失** | grep 零匹配 |
| Art. III.2 目录接口 | ✅ | search 返回路径 |
| Art. III.3 异质性 | 🟡 | 广播 TopK 给所有 agent 与"屏蔽横向相关性"张力 |
| Art. III.4 Goodhart | ✅ | 度量代码对 prompt 保密 |
| Art. IV 回路 | ✅ | rtool→δ→∏p→wtool 闭合 |
| Art. IV q-halt | 🟡 | halt 由 evaluator 外部触发，非 Q_t 一等字段 |
| **Art. V.1.1 宪法只读** | 🔴 | `constitution.md` mode 644，无 CI lock |
| **Art. V.1.2 ArchitectAI** | 🔴 空壳 | `src/` 零匹配 |
| **Art. V.1.3 JudgeAI** | 🔴 空壳 | `handover/directives/` + `handover/audits/` 全空 |
| Law 2 守恒 | 🟡 | invest refund + Hayek bounty payout 未进 `reward_pull_conservation.rs` |

**关键裁决**：Art. V 三权分立只是 Claude Code sub-agent prompt，未入 Rust 微内核。

---

## § 2. Art. IV 拓扑逐行核对

| 图示节点 | 代码落位 | 判决 |
|---|---|---|
| Q_t `<q, HEAD, tape>` | `kernel.rs:20-21` + 隐式 state | 🟡 结构等价，无显式记号 |
| rtool | `evaluator.rs:450-495` build_agent_prompt | 🟡 字符串化，无 `<q_i, s_i>` 输出 |
| δ (AI) | `evaluator.rs:514-516` | ✅ |
| ∏ predicates | `lean4_oracle.rs:249-258` `PartialVerdict` 三路判决 | ✅ |
| wtool | `bus.rs:174-177` `append_oracle_accepted` + F-20-05 封堵 | ✅ |
| q_1 + HEAD_1 | `evaluator.rs:616, 622-624` + `wal.rs:50-51` sync | ✅ |
| p=0 拒绝 | `lean4_oracle.rs:256-257` Reject 不调 wtool | ✅ |
| q==halt 自动判定 | 无 Q_t 一等字段，halt 外部触发 | 🟡 |

**综合**：92% 完整，两处记号偏差。

---

## § 3. Karpathy TOP-10 代码优化

排序按收益：
1. `src/bus.rs:244-246` InvestOnly 三重 clone — ~10-20% per tx
2. `src/bus.rs:256-262` author/payload to_string — 20-30% heap
3. `src/bus.rs:416-424` `Box<dyn>` + 4× downcast — ~5× 加速
4. `src/wal.rs:54-55` evt_clone 再传引用 — 消除二次 clone
5. `src/bus.rs:402` settle_portfolios credits.push clone — 改 &str
6. `src/bus.rs:48, 444-539` graveyard + TopKClasses 双写 — ~40% 内存
7. `src/bus.rs:386-388` settle 临时 HashMap — 消除分配
8. `src/ledger.rs:115-135` trace_ancestors HashSet alloc — Vec<&str>
9. `src/prediction_market.rs:188-344` f64 无 epsilon — 量化 fixed-point
10. 化石：`RejectionScope::PerAuthor` 未用、`Box<dyn TuringTool>` 3 个实现可改 enum dispatch

**聚合估计**：append 热路径 20-40% 加速 + heap 30-50% 降低。

---

## § 4. Phase 7 DAG 分析核心发现

报告：`experiments/minif2f_v4/analysis/PHASE7_DAG_ANALYSIS.md`

关键数字：
- N=20, TURING_STEP_ONLY=1
- Solved: 9/20 (45%)，vs monolithic baseline 17/20 (85%)
- Depth histogram `{1:5, 3:1, 17:1, 20:1, 23:1}` — 首次非 delta-function
- depth-23 imo_1964_p2 + depth-20 mathd_algebra_332 + depth-17 imo_1981_p6，9/9 外部 Lean 重跑通过
- tool_dist: step=132, step_partial_ok=59, step_reject=64, omega_wtool=9
- 11 timeouts — per-tactic Lean elaboration 延迟累加
- **经济层未激活**：Hayek payout 未进日志，Satoshi rebate Phase 3B 未合并，Librarian 板无 agent 读取证据，emergent roles 未发生（9/9 solve 均为单 agent 祖先链）

---

## § 5. 上线路线图 P0

报告：`handover/ai-direct/ROADMAP_LAUNCH_2026-04-21.md`

7 个 P0 Launch Gating Issues：
1. 身份签名 (Ed25519) — M
2. 女巫防御 (PoW + reputation γ 曲线) — L
3. Tape 污染 + 园丁 GC — M
4. 经济常量固化 (γ/β/θ 编译期) — S
5. Oracle DoS 防御 (fingerprint cache + 梯度定价) — M
6. 争议治理通道 (Art. V.1 落地) — L
7. Quickstart + 参考 agent 客户端 — S-M

---

## § 6. Co-worker 补充清单

- Art. V 落地方案：进程化 InitAI/ArchitectAI/JudgeAI
- 宪法文件 chmod 444 + GPG commit hook
- Lean + Mathlib toolchain drift (preflight lock)
- Deterministic replay (WAL 重放一致性)
- Kill switch / emergency pause
- Schema version 字段 (WAL/tape/Event)
- N=50×32 agents soak test
- Oracle 计费 vs Coin 通胀稳态模型
- Gini/HHI 指标进 monitoring
- 英文文档 + LICENSE + ToS
- 隐私声明（tape 公开）
- Telemetry opt-out

---

## § 7. 建议立新判例

- **C-044** Meta Architecture Not Implemented
- **C-045** Gardener Agent Missing (Art. III.1)
- **C-046** Constitution File Not Readonly (Art. V.1.1 违宪)
- **C-047** `recent_rejections` author 参数吞没 (Art. III.3 vs C-022 张力)
