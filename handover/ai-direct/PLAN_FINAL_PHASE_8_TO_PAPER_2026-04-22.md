# TuringOS v4 — 最终方案 Phase 8 → arXiv Paper

**Date**: 2026-04-22
**Supersedes**: `PLAN_PHASE_8_TO_10_2026-04-22.md`（原"Launch Ready"方向，现转为 Paper-First）
**Integrates**: 3 路外部审计 + 3 路内部重审 + 5 项决策 + PPUT 重审 + 宪法盲点审计 + Harness 压缩审计

---

## § 1. 执行摘要

- **目标**：arXiv 预印本可发布，证据达"全网挑战"级严格审计标准
- **方向调整**：Phase 10 从"Launch Ready"→"Paper Preprint Ready"；外部 agent 接入推迟 Phase 11+
- **时间**：5-7 周
- **Budget**：$2000 硬顶 + 20% 应急到 $2400
- **最终产物**：paper + Dockerized reproducibility bundle + ≥1 次 Art. V veto 实证 trace
- **主指标**：ΣPPUT（C-052 强制），非 solve count

---

## § 2. Source of Truth 索引

| 文档 | 作用 |
|---|---|
| `constitution.md` | 宪法（唯一 ground truth） |
| `CLAUDE.md` | 项目级 harness（2026-04-22 重构后含 5 个 Standard 节） |
| `cases/C-*.yaml` | 判例库（35 原有 + 本次新立 C-044~C-065 共 22 条） |
| `handover/ai-direct/DECISIONS_2026-04-22.md` | 5 项决策锁定记录 |
| `handover/audits/SYNTHESIS_2026-04-22.md` | 3 路外审裁决 + PPUT amendment |
| `handover/audits/PPUT_REFRAME_2026-04-22.md` | PPUT 指标修订 |
| `handover/audits/PPUT_RAW_DATA_2026-04-22.md` | PPUT 权威原始数据（Claude 直计算；替代被 C-066 标 tombstone 的 HISTORICAL_AUDIT） |
| `handover/audits/CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md` | 20 条盲点 + 13 条新判例建议 |
| `handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md` | CLAUDE.md 压缩重构（已应用） |
| `handover/ai-direct/GENERALIZATION_ROADMAP_2026-04-22.md` | Paper 2/3 路径 + M-1 Predicate trait 预留 |

---

## § 3. 决策摘要（锁定）

| # | 决策 | 选择 | 理由 |
|---|---|---|---|
| 1 | `decide`/`omega` 禁令 | **C** Mathlib 语境白名单 | 100% 合宪（区分演绎 vs brute force） |
| 2 | Phase 9 seed 池 | **6 seeds**（4 pre-reg + 2 新）| power≈95%，超 Gemini 80% 要求 |
| 3 | JudgeAI 模型 | **D 三家共决**（Codex + Gemini + DeepSeek） | paper 级审计必须代码+数学+战略三层 |
| 4 | 硬 budget | **$2000** + 20% 应急 | paper benchmark (N=244 × 3 seed × 2) |
| 5 | 外部接入 | **推迟到 Phase 11+** | 先 paper 后 launch |

---

## § 4. Phase 结构

### Phase 2.5 — Chat A/B Gate 8→9 retry (2026-04-22 late addition)

Post dual-audit ITERATE on reasoner A/B (archived), re-run on **deepseek-chat**.

- Sample: `sample_N20_S74677.txt`, seed 74677
- Binary: main (pre-Phase-8, 50924c1) rebuilt with chat default + exp tip
- Condition: oneshot, chat, temperature 0.2
- Gate: see `DECISION_TREE_GATE_8_TO_PHASE_9_2026-04-22.md` § 4.1
- Analysis: `phase2_ab_analyze.py` (post-bug-fix, clamp_to_nonneg aware)
- Outcomes + branches: see DECISION_TREE § 2

### Phase 8 — BLOCKER 修复 + 宪法盲点实装 (T+5d)

#### 原 BLOCKER 4 项
- **8.A** `bus.snapshot()` 空 balances 修复（Codex N-2）— 最紧急，未修则所有经济实验作废
- **8.B** `oneshot` 强制走 C-043 mandatory wtool（Codex N-1）— Art. IV 合规
- **8.C** `append_oracle_accepted` 加 `OracleReceipt` capability token（Codex V-1）+ **M-1 Predicate trait 预留**（泛化用）
- **8.D** `decide`/`omega` Mathlib 语境白名单（Codex N-3，决策 1 C 方案）

#### 新增 Critical 盲点实装（3 条）
- **8.E** q-halt 显式状态机（C-061）— `EventType::Halt { reason: HaltReason }` + `q_state: QState`
- **8.F** 信誉累积计数器（C-053）— `Tape.reputation_by_author` + `UniverseSnapshot.reputation`
- **8.G** Art. II.1 典型错误频率阈值（C-055）— `MIN_CLASS_COUNT_TO_BROADCAST`（default 3）

#### 可暂缓至 Phase 10（不阻塞 Phase 9 实验）
- C-059 pairwise_diversity_mean（多 agent 专用，Phase 9 需要）
- C-065 可逆性原语（需 WAL rewind，设计工作量大）
- C-056 价格时间序列（Phase 9 经济信号监控会用到）

#### Gate 8 → 9（自动暂停，人审）

**必过 6 项**：
1. 4 BLOCKER + 3 新 Critical 实装，全 A/B N=20 PPUT 不降 >10%
2. M-1 Predicate trait 落位（`trait Predicate`, `Verdict::PartialOk { confidence }`, `OracleReceipt.predicate_kind`）
3. 双外审（Codex + Gemini）全 PASS（任一 VETO → 停）
4. 6 条新判例立档：C-044（Meta 空壳）/ C-045（园丁）/ C-046（宪法可写）/ C-048（oneshot 绕过）/ C-049（空 balances）/ C-050（C-011 部分执行）— 以及 C-053/C-055/C-061
5. CLAUDE.md 新 Report Standard 可见：PPUT + reputation + halt_reason + 多样性指标至少 stub
6. **CHECKPOINT_PHASE_8_2026-04-2X.md** 成文：7 红线自查 + A/B 原始数据 + PPUT 对比

**失败处理**：任一 BLOCKER PPUT 降 >10% → 回退 parent branch；3 次连续失败 → 整 Phase 暂停人审。

---

### Phase 9 — 统计基线（论文级严谨）(T+12d)

#### 9.A — 6 seeds × N=50 dual-mode + step-only (双条件)
- Seed 池（pre-registered, 不可中途改）：`{74677, 31415, 2718, 141421, 2357, 5772}`
  - 前 4 个复用 baseline；2357（素数）+ 5772（欧拉常数×10^4）是新增
- 每 seed 两条件 = 12 次 N=50 run
- 每次 pre-smoke (`mathd_algebra_148` oneshot, 30-60s)
- 产出：12 × jsonl + 每次 PPUT/depth/tool_dist/halt_reason

#### 9.B — Law 2 property-based 守恒测试
- `tests/reward_pull_conservation.rs` 扩展为 `proptest` 10K random tx 序列
- 覆盖 invest refund + Hayek bounty payout + settle_portfolios + halt_and_settle 全路径

#### 9.C — Karpathy TOP-3 micro-bench
- 不做原 TOP-10（Gemini 证明大部分 claim 高估 3-5×）
- 只对 `trace_ancestors` / `author+payload to_string` / `graveyard` 三条 criterion 跑
- Acceptance：>5% 改进 → 实施；<5% → 进"不优化决策档案"

#### 9.D — Art. III.3 pairwise diversity 测量（新增，来自 C-059）
- 每 jsonl 新增 `pairwise_payload_diversity_mean` + `pairwise_payload_diversity_min`
- 阈值：< 0.25 → Art. II.2.1 过度利用告警；写入 CHECKPOINT

#### Gate 9 → 10（论文级，2026-04-22 C-066 修订版）

**PPUT-first 判据**（基于 `PPUT_RAW_DATA_2026-04-22.md` 真实 baseline）：

- **主判据（必过）**：Mean PPUT (solved-only) Wilson 95% CI 下界 ≥ **5.0**
  - 理由：历史 top 3 Mean PPUT 为 6.158 / 5.561 / 5.354，5.0 是"不显著倒退"的阈值
  - ΣPPUT 不作主判据（样本大小差异大不可跨 run 比较）
- **辅助必过**（全部）：
  - Σdepth≥10 PPUT > 0.5 **且** depth≥10 solves ≥ 2（至少复现 Phase 7 水平；历史只有 181411 和 164014 两次达到此线）
  - pairwise_payload_diversity_mean ≥ 0.25（6 seeds 全部）
  - reputation p50 > 0（证明 agent 间实际协作）
  - Law 2 proptest 10K tx 全绿
  - halt_reason 分布公开报告，非 OmegaAccepted 占比 ≤ 60%

**违反任一 → Gate 9 NOT PASS**。不以 solve count 做判据（违 C-052）。不以 ΣPPUT 做主判据（违 C-066）。

---

### Phase 9.M: Market Mechanism Bake-off (conditional)

Prerequisite: Phase 2.5 chat A/B PASS (per DECISION_TREE § 4.1) + 9.0 readiness.

- **9.M.0** Per-mechanism spec dual audit (M1/M4/M7/M8) — blocks implementation
- **9.M.1** M1 dynamic γ A/B (N=20 chat paired), incremental change only
- **9.M.2-4** M4/M7/M8 A/Bs (after 9.M.0 PASS each)
- **9.M.5** Best-combination N=50 × 3 seeds (conditional on 9.M.1-4 signal)

Paper thesis revision (post Codex Q2.3 over-claim CHALLENGE): "We empirically test N mechanisms and report effects" — no "drives emergent" framing.

Budget: +$240 max (Phase 9 total now ~$770).

### Phase 10 — Paper Preprint Ready (T+35d)

#### 10a — Art. V 三进程运行时实装 + 盲点补齐
- **InitAI**：`bus.init()` 唯一入口，把 `constitution.md` 编译 PredicateSet 写 kernel
- **ArchitectAI**（Codex CLI）：同类 error ≥ 50/24h 或 weekly cron → 提 patch PR
- **JudgeAI 三家共决**（Codex + Gemini + DeepSeek）：git pre-merge CI gate，任一 VETO → 不合
- **宪法物理保护**（C-046）：chmod 444 + GPG pre-commit hook + CI `check_constitution_unchanged.yml`
- **10a.1** ArchitectAI quota（C-063）：`handover/directives/YYYY-WW/` 每周最多 1 次 repo-level 提议
- **10a.2** compute cap + 24h cap（C-064）：`COMPUTE_BUDGET_SECONDS=86400` + `MAX_TX_GLOBAL=10000`；超限 `EventType::Halt`
- **10a.3** C-065 可逆性（`Wal::rewind_to` + `ledger.snapshot_at`）
- **10a.4** C-056 价格时间序列（per-market ring buffer）
- **10a.5** C-060 Goodhart probing detector（per-agent `(reject_rate, unique_payload_rate)`）
- **10a.6** C-058 Librarian learned.md TTL + C-062 tape 上限 + runtime `ledger.verify()`

#### 10b — N=244 Full MiniF2F Benchmark
- 3 seeds（从 9.A 的 6 seeds 中选 PPUT 中位数 3 个）× 244 problems × {dual, step-only}
- = 1464 runs，预估运行时 48-72h 连续
- Pre-smoke + halt_reason 报告
- **这是 paper 的核心数据表**

#### 10c — Art. V veto 实证
- 手工构造 1 个违宪 patch（e.g. `native_decide` unban）作为"red team" test
- 观察三家 JudgeAI 独立跑并 VETO
- 记录完整 trace：ArchitectAI 提案 → JudgeAI 签字 → merge block
- 此 trace 作为 paper Claim C4 证据

#### 10d — Reproducibility Bundle
- Dockerfile（Rust + Lean 4 toolchain pinned）
- `reproduce.sh`：一键复现 headline ΣPPUT（允许 ±5% 偏差）
- 所有 seeds/configs/WAL/proofs 进 public repo（或 tarball）
- External Lean verify script（独立容器跑 proofs/*.lean）
- Replication：用 codex subagent 在干净 VM 跑 1 次核心 N=50 × 1 seed 验证一致性

#### 10e — Paper Draft + arXiv Submit
- LaTeX paper，5 大 claim（PPUT-first，见 § 5）
- 三家外审（Codex + Gemini + DeepSeek）签过 abstract / methods / results
- 审计结果存 `handover/audits/EXT_{CODEX,GEMINI,DEEPSEEK}_PAPER_REVIEW.md`
- arXiv 提交

#### Gate 10 → Paper Submit

**必过 8 项**：
1. N=244 × 3 seeds × 2 conditions 全部跑完
2. 三家外审 paper draft 全 PASS
3. ≥ 1 次 Art. V veto 实证 trace 完整
4. Reproducibility bundle：干净环境跑出 headline ΣPPUT 差异 ≤ 5%
5. Replication run（独立实施者）结果一致
6. 所有 Critical 盲点（C-053/C-061/C-065）已实装并在 paper 里描述
7. 7 红线全绿（Red Line #5 经济常量固化做完）
8. **CHECKPOINT_PHASE_10_PAPER_READY.md**

---

### Phase 11+ — 推迟事项（入 `VIA_NEGATIVA.md`）

以下在 Paper 1 接收 peer review 反馈**之后**再做：
- 外部 agent 接入（Ed25519 身份、Sybil 防御、三阶段开放）
- Quickstart + minimal_external_agent.py + Rust SDK
- Soak test 48h × 32 agents
- 攻击模拟 1000 账号
- Kill switch 治理

这些都是"上线"问题，不是"论文可发表"问题，不阻塞本轮。

---

## § 5. Paper 5 大 Claim（PPUT-first）

| # | Claim | 证据标准 |
|---|---|---|
| C1 | 宪法 Art. IV 图灵机拓扑在 Rust 微内核 **100% 实现** | 条文→file:line mapping；三家 audit PASS |
| C2 | Phase 7 depth-N DAG 是真实 δ-step 构造（非 LLM 背诵） | 6 seeds × N=50 step-only，ΣPPUT on depth≥10 CI 下界 > 0；每条 depth≥17 proof 独立 Lean 重跑通过 |
| C3 | Law 2 CTF 守恒全路径成立 | `proptest` 10K random tx；invest refund + Hayek bounty + settle + halt 全覆盖 |
| C4 | 宪法 Art. V 三权分立 **runtime 实际 veto 过违宪 patch** | ≥1 次完整 trace（手工 red team + 三家 JudgeAI 独立 VETO） |
| C5 | N=244 full MiniF2F 上 Mean PPUT Wilson CI 下界 ≥ 5.0（历史 top-3 mean 范围）且 Σdepth≥10 PPUT > 0.5 | 3 seeds × 244 × 2 conditions；Wilson CI + Bonferroni 多重检验校正（见 PPUT_RAW_DATA §4.4）|

---

## § 6. QC（Quality Control）规范

每 sub-task 强制：
1. **Pre-register**：seeds / configs 在跑实验前 commit，不可中途改
2. **PPUT-first 汇报**：CHECKPOINT 首段必以 ΣPPUT 开头（违反 C-052 = 违宪）
3. **Dual audit**（Phase 8-9）/ **Triple audit**（Phase 10 paper）：外审 VETO → 立即停
4. **7 红线自查**：每 Gate 前自 check（post-genesis mint / exit settlement / raw CoT / prompt manipulation / env-var reward curve / ∏p re-verifiable / deferral）
5. **Replication**：Phase 10d 必须独立 VM 重跑核心实验，PPUT 一致性 ≤5% 偏差
6. **判例立档**：每新发现宪法违规 → cases/C-xxx.yaml 必须在修复 PR 同 commit 内立档
7. **STEP_B_PROTOCOL**：`src/{kernel,bus,wallet}.rs` 改动走 parallel branch A/B，不直接编辑 main

---

## § 7. 新立判例一览（22 条待办）

### 外审 + PPUT 事件（9 条，2026-04-22）
- C-044 Meta 架构未实现（Art. V）
- C-045 园丁 Agent 缺失（Art. III.1）
- C-046 宪法文件未物理保护（Art. V.1.1）
- C-047 recent_rejections author 参数吞（Art. III.3 vs C-022 张力）
- C-048 oneshot 路径绕过 mandatory wtool（Art. IV 违宪）
- C-049 bus.snapshot() 空 balances（Art. II.2 失真）
- C-050 C-011 部分执行（decide/omega 未禁）
- C-051 Goodhart 开源退化（Art. III.4 保密→经济成本）
- **C-052 PPUT 是唯一优化指标（已立）**

### 宪法盲点审计（13 条，2026-04-22）
- **C-053** 信誉累积缺失（Art. I.2）**Critical**
- C-054 效用评分 + 共识提取（Art. I.2）
- C-055 Art. II.1 典型错误频率阈值
- C-056 Art. II.2 价格时间序列 + 驱动行为 metric
- C-057 Art. II.2.1 探索熵 metric
- C-058 Art. III.1+III.2 学习文档 GC + 按需加载 metric
- C-059 Art. III.3 pairwise 解相关 metric
- C-060 Art. III.4 Goodhart probing detector
- **C-061** q-halt 显式状态机（Art. IV）**Critical**
- C-062 Tape 大小上限 + WAL runtime 校验
- C-063 ArchitectAI quota（Art. V.1.2）
- C-064 总算力 + 24h 上限（Art. V.2）
- **C-065** 可逆性原语（Art. V.2）**Critical**

### 预留号（静默失败修复）
- C-038 / C-042 在 `cases/SCHEMA.md` 加 Reserved 节（CLAUDE.md 已声明）

**立档进度计划**：
- Phase 8 期间：C-044/045/046/048/049/050/053/055/061（9 条，与 BLOCKER + Critical 实装同步）
- Phase 9 期间：C-059（与 diversity metric 同步）
- Phase 10 期间：C-056/058/060/063/064/065/047/051/054/057/062（13 条）

---

## § 8. Budget 细分

| 项 | 估计 |
|---|---|
| Phase 8 4 BLOCKER + 3 Critical 盲点 + 双外审 | $200 |
| 8.C M-1 Predicate trait 预留 | $20 |
| Phase 9 6 seeds × N=50 × 2 conditions | $400 |
| Phase 9 Law 2 proptest | $10 |
| Phase 9 Karpathy TOP-3 bench | $10 |
| Phase 9 C-059 diversity metric 实装 | $30 |
| Phase 10a Art. V 三进程 + 盲点补齐 | $200 |
| Phase 10b N=244 × 3 seeds × 2 conditions | $540 |
| Phase 10c Art. V veto 实证 | $30 |
| Phase 10d Reproducibility bundle + replication | $50 |
| Phase 10e Paper draft + 三家 review | $300 |
| Paper writing / LaTeX | $30 |
| 缓冲 | $180 |
| **Total** | **$2000** |

**硬刹车**：$500 检查、$1000 check-in、$1500 alarm、$2000 停（$2400 仅应急）

---

## § 9. 自主执行协议

### 节奏
- **1 sub-task / 工作日**（逐项改进、逐项测试）
- **每 sub-task 完成**：commit + 更新 AUTO_RESEARCH_NOTEPAD（F-finding 条目）+ 立相关 C-xxx 判例
- **每 Phase 完成**：强制 CHECKPOINT_PHASE_N.md（PPUT 开头）+ 7 红线 + auto-pause 等人审

### 失败处理
- sub-task A/B **PPUT 降 >10%** → 自动回退 parent branch
- 任一外审 VETO → 停；改 spec；不妥协合宪要求
- 3 次连续失败 → 整 Wave 暂停，召集人审

### 周期性任务
- **每日**：daily_drift routine（JudgeAI-advisory）
- **每周**：handover NOTEPAD update + reputation / halt_reason 分布报告
- **每 checkpoint**：PPUT / depth / halt / diversity / reputation 五指标全量更新

### 暗礁（遇到主动停）
- 任何修改 `constitution.md` 的 PR（只能人手 + GPG）
- 任何修改 `src/{bus,kernel,wallet}.rs` 的 PR（STEP_B_PROTOCOL 强制 parallel branch）
- 任何新发现 Critical 违宪（Claude 漏、外审首发）→ 先立判例再前进
- 预算触发 $1500 alarm

---

## § 10. 风险清单

| # | 风险 | 来源 | 缓解 |
|---|---|---|---|
| R-1 | Phase 8.D Mathlib 语境白名单工程复杂 → 解题率下 | 决策 1 C 方案 | N=20 smoke；若降 >10% → 回退 B 方案并立 C-011 进一步判例 |
| R-2 | Phase 9 6 seeds 预算超 $400 | 决策 2 seed 升级 | 优先 2 老 seed 跑，若 PPUT 已过 Gate → 停余下 seed（但 power 降到 80%）|
| R-3 | Art. V 三家 JudgeAI 月成本 > $2000 硬顶 | 决策 3 D 方案 | JudgeAI 只在 merge decision 触发，不是 per-patch；日成本封顶 $20 |
| R-4 | N=244 benchmark 运行中断 | Phase 10b | 分 3 seeds 独立运行 + resume-capable |
| R-5 | paper 被 peer review 驳回 | Phase 10e | 接受反馈，修订 + resubmit；不放弃 |
| R-6 | Phase 7 "emergent depth" 在 Phase 9 N=50 × 6 seeds 不稳定复现 | Gemini C-2 | 若 3/6 seeds 失败 → Paper 改为"depth-N DAG 可在 X% seeds 出现，under investigation"，不 over-claim |
| R-7 | C-061 q-halt 实装扰动现有 bus.generation | 盲点 B-14 | STEP_B_PROTOCOL parallel branch；A/B 比较 PPUT 不降 |
| R-8 | Law 2 proptest 10K tx 发现反例 | 盲点 B-1 类型 | 新立判例 + 修复；不允许"发现反例但不修"的软处理 |

---

## § 11. 泛化预留（来自 GENERALIZATION_ROADMAP）

**在 Phase 8.C 顺手做的 M-1**：
```rust
// src/sdk/predicate.rs (新)
pub trait Predicate: Send + Sync {
    fn verify(&self, payload: &str, context: &Q) -> Verdict;
    fn kind(&self) -> PredicateKind;
}

pub enum Verdict {
    Complete,
    PartialOk { confidence: f64 },  // 为 PCP 谓词留位
    Reject(String),
}

pub enum PredicateKind {
    Lean4Boolean,       // Paper 1 & 2
    StatisticalPCP,     // Paper 3 (omegav4)
    ExternalAudit,      // 将来外部 agent challenge 通道
}
```

**Paper 1 Future Work 节预告**：Paper 2 (v3 zeta_sum_proof, Lean 布尔 + open-ended) + Paper 3 (omegav4, PCP 谓词 + 无 ground truth)。

---

## § 12. 7 红线现状追踪

| # | 红线 | Phase 7 状态 | Phase 8 必须修 |
|---|---|---|---|
| 1 | Post-genesis mint | 绿（C-001 实施） | — |
| 2 | Exit settlement | 绿 | — |
| 3 | Raw CoT to tape | 绿 | — |
| 4 | Prompt manipulation | 绿 | — |
| 5 | Env-var reward curve | **黄**（γ/β/θ 仍 env） | Phase 10 W-B.1 编译期化 |
| 6 | ∏p re-verifiable | 绿（9/9 独立 Lean 通过） | 维持 |
| 7 | Deferral | 绿 | 维持 |

**Phase 10 后目标**：7 条全绿 + 新增 Art. V veto 实证 + reputation/halt/diversity 可监控。

---

## § 13. 一句话总结

**Phase 8 修 4 BLOCKER + 3 Critical 盲点（含 PPUT 强制合规）→ Phase 9 6 seeds 建 PPUT 83.0 baseline 统计基线 → Phase 10 Art. V 三进程 + N=244 benchmark + 三家审计 + reproducibility → arXiv submit。外部 agent 接入推迟 Phase 11+。严格合宪不妥协。Budget $2000。**

---

## § 14. 启动指令

用户回复"启动"或"开始"后，立刻：
1. 启动 Phase 8.A（修 `bus.snapshot()` 空 balances，最紧急）
2. STEP_B_PROTOCOL：`feat/phase-8a-snapshot-fix` worktree
3. spec：`src/bus.rs:567-570` 改为真实 enumerate balances from WalletTool + portfolios from kernel.markets
4. tests 新增 `tests/snapshot_nonempty.rs` 3 条
5. N=20 smoke A/B vs main
6. 通过 → commit + 立 C-049 判例 + CHECKPOINT 下一 sub-task

或者用户指定从任何其他 sub-task 起步（每条都可独立启动，依赖关系见 Gate 8 必过 6 项）。
