# TuringOS v4

## What
Silicon-Native Microkernel for LLM Formal Verification Swarm.
Rust 2021, tokio, serde_json. Mission: MiniF2F Lean 4.

## Why
- 唯一对齐文档: `constitution.md` (反奥利奥架构)
- 压缩即智能: 抽象原则进宪法 / 具体情境进 `cases/`
- 机制 > 参数 > 提示 (Art. V + C-021/C-031/C-034/C-043)
- **Tape-first**: 纸 / write tool / 橡皮 / 严格 predicates 决定 L4 / L4.E。没有 tape activity 的测试，一律不算 TuringOS 测试 (架构师 2026-05-06)

## ⚡ PRIME OPERATING MODE — Constitutional Harness Engineering (since 2026-05-06)

**取代 Atomic Agentic Engineering**。来源: `handover/directives/2026-05-06_TB18R_EMERGENCY_HARNESS_RESET_DIRECTIVE.md`.

### Order of operations (硬规则)
```
1. Constitutional harness as executable tests   ← FIRST
2. Minimal real run that exercises tape         ← SECOND
3. External audit ONLY after evidence passes    ← THIRD
4. Documentation packages proof, never substitutes for tape
```

**禁止反序**: 不再 `charter → atom → self-audit → external audit → more docs → delayed test`。
**禁止 ceremony**: 流程必须服务 tape，不是 tape 服务流程。
**没有 tape evidence = 不算测试**：评估器 stdout / 私有日志不构成 TuringOS 证据。

### 三个 Flowchart Gate (Class 3/4 必过)

**FC1 — Runtime Loop Gate**
- 循环: `Q_t → rtool/context → Agent output → predicate/oracle → wtool → L4 or L4.E`
- 硬不变量: `externalized_attempt_count == L4_WorkTx_attempt_count + L4E_WorkTx_rejection_count + explicitly_anchored_capsule_attempt_count`
- 影响 proof state / future prompt / Lean check / final composite proof 的外部化 LLM-Lean cycle 必须 tape-visible：
  - predicate pass → L4
  - predicate fail → L4.E
  - high-volume logs → CAS EvidenceCapsule + L4 anchor (不是 attempt 替代)

**FC2 — Boot / Genesis Gate**
- 每个真实 run 必须可从以下重建: `genesis_report + ChainTape + CAS + agent registry + system pubkeys`
- 禁止: memory-only preseed / retroactive evidence rewrite / global pointer source-of-truth (TB-16 OBS_R022 教训)

**FC3 — Meta / Markov Gate**
- EvidenceCapsule + Markov capsule = derived view，不是隐藏 ground truth
- raw logs shielded; capsule 必须 derivable from ChainTape + CAS
- dashboard = materialized view only (不可成 source of truth)

### Constitutional CI tests + Kill gates
权威列表 + 当前状态：`bash scripts/run_constitution_gates.sh` 或 `make constitution`。
每个 gate = 一个 `tests/constitution_*.rs` cargo test。任一 fail = halt + 不算 TuringOS run。
CI required workflow：`.github/workflows/constitution_gates.yml`（merge gate）。
**CLAUDE.md 不编码具体 gate 名称 / 数量**（rot 风险；TB-C0 ship 时已 64 个，将持续增长 — 跟脚本，不跟 CLAUDE.md）。

### Audit policy 调整
- **Class 0/1** (docs / additive): self-audit 即可
- **Class 2** (production wire-up): self-audit + smoke
- **Class 3** (auth/crypto/money/capability replay): harness → real-run-passes → external audit (顺序不可逆)
- **Class 4** (constitution / sequencer admission / typed-tx schema / canonical signing payload): harness → minimal real run → architect ratification → external audit
- **若真实样本失败 → 直接回实现层，不进入审计** (节省 audit dispatch entropy)

## Code Standard (Art. I.1 + C-004 + C-027)
- `cargo check` / `cargo test --workspace` 必过；`.env` 永不 commit
- STEP_B_PROTOCOL（不直接编辑 main）适用于:
  - `src/kernel.rs` + `src/bus.rs` + `src/sdk/tools/wallet.rs` + `src/state/sequencer.rs` (kernel/economy/admission)
  - `src/state/typed_tx.rs` + `src/bottom_white/cas/schema.rs` (Class-4 typed-tx + CAS schema; TB-18R 2026-05-06 加入)
- 任何影响行为的参数必须 env/config 可覆盖，不可硬编码

## Audit Standard (Art. V.1 + C-010 + C-023 + C-035, 2026-05-06 reset)
- Generator ≠ Evaluator：代码作者不可是唯一审计者
- **新顺序**: external audit AFTER tape evidence；不再先审 schema 再决定要不要跑 (`feedback_audit_after_evidence` 升级为全 Class-3/4 适用)
- 所有 merge / phase 决策双外审（Codex + Gemini）；VETO > CHALLENGE > PASS
- 宪法违规立即 BLOCKER，不可延期、不可"可接受"

## Report Standard (Art. I.2 + Art. II.2.1 + Art. IV 强制, C-052 + C-053 + C-057 + C-059 + C-061)
- **主指标**（每报必填）: ΣPPUT + Mean PPUT (solved) + 95% CI (Wilson)
- Art. I.2 三大统计信号不可缺: **信誉** (reputation_distribution p50/p90/max) + 效用 (PPUT) + 共识 (如适用)
- Art. IV 终态区分: `halt_reason_distribution` {OmegaAccepted, MaxTxExhausted, WallClockCap, ComputeCapViolated, ErrorHalt}
- 多 agent (n≥2) 专用: `parent_selection_entropy` + `pairwise_payload_diversity_mean`；任一 < 0.25 = Art. II.2.1 告警
- solve count 不可独立陈述，必须配对 PPUT；以 solve count 起头 = 违宪
- **新强制 (2026-05-06)**: 每个 run 必填 `attempt_count_equality_report` (`evaluator_reported_tx_count == chain_attempt_count`)；不等 = halt + 不算 TuringOS run

## Reproducibility Standard (Art. I + C-012/C-016/C-032/C-039)
- OMEGA accept 必留 self-contained artifact (`proofs/*.lean` + `gp_payload`)
- 度量工具上线即冻结；Oracle 参数冻结；实验禁混 Oracle 模式
- 中间件若修改数学内容 → 是 ArchitectAI 贡献，不是 swarm 涌现（C-023）
- **每个 evidence run 必须 replayable from `genesis_report + ChainTape + CAS + agent registry + system pubkeys`** (FC2 Gate)

## Alignment Standard (Art. IV + C-069)
- 权威对齐文件: `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md`
  (后续 rev: `TRACE_MATRIX_vN.md`)
- 每个 src/ pub 符号必须映射到宪法 flowchart 元素、标 orphan+justification、
  或 BLOCK merge。doc-comment backlink 格式: `/// TRACE_MATRIX <FC-id>: <role>`
- Conformance tests: `tests/fc_alignment_conformance.rs` — 每个 ✅ 行 ≥1
  witness test；`#[ignore]` stub 覆盖 📅 deferred rows
- 宪法 flowchart 修改仅 human architect 可触发，需重跑 Phase Z′ 6-stage
- constitution.md hygiene 观察登记到 `handover/alignment/OBS_*.md`，不改宪法

## Common Law (宪法 + 判例)
宪法高度压缩，具体裁决查 `cases/C-xxx.yaml` (facts → ruling → precedent)
- 按条款查: `grep -l "Art. I.1" cases/*.yaml`
- 映射：`cases/V3_LESSONS.md` (50 v3 教训 → 现行判例)
- 编号跳号：C-038 / C-042 为 reserved（见 C-041/C-043 预引用）

## Active state (动态；单一来源)
- **Session-level state**: `handover/ai-direct/LATEST.md`（每 session 末尾更新；ship 状态 / freeze / 当前 charter / forward-bound items 全在这里）
- **Ship 总账 (authoritative)**: `handover/tracer_bullets/TB_LOG.tsv`
- **当前操作模式**: Constitutional Harness Engineering（since 2026-05-06；reinforced post-TB-C0 ship 2026-05-07）
- **CR-C0.10**: 每个新 feature TB merge 前必过 `bash scripts/run_constitution_gates.sh`
- **CLAUDE.md 不编码 ship 状态 / gate 数量 / round 数 / freeze 状态 / TB 名单**（rot 风险；这些都属于 LATEST.md 或 TB_LOG.tsv）
- **永久工具入口**（不变）：
  - `bash scripts/run_constitution_gates.sh` — constitution gate runner
  - `python3 scripts/fc_witness_extract.py <run_dir>` — FC-witness 单题
  - `scripts/regenerate_post_fix_evidence.sh` — STRICT 聚合（EXPECTED_FC_NODES universe）
  - `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md` — real-problem 绑定
  - `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` — clause→code→test→smoke 矩阵

## Memory (跨 session 持久; auto-loaded MEMORY.md 是 hot index)
- 高频 rule (feedback_*) + 项目状态 (project_*) + 外部引用 (reference_*) 路径: `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/`
- 写新 memory: 文件名 `<type>_<topic>.md` 加 frontmatter + 在 MEMORY.md 加 ≤150 字符 hook
- 不要在 memory 里复述 TB_LOG.tsv 已有的 ship facts; 只记 session-level 教训和 surprise

## Pre-action gates + Cadence (mechanism > norm)
- **Before any runner script** that mutates `handover/evidence/` 或运行真题评估：invoke `/runner-preflight` skill（7 stages: tree clean / binary mtime / evidence immutability / Class classification / FC-trace / charter / audit-round count）。memory: `feedback_pre_runner_checklist`
- **After TB SHIPPED FINAL** 或 audit rounds > 3：invoke `/harness-reflect` skill。memory: `feedback_harness_reflect_cadence`
- **Before writing new `feedback_*.md`**: 先问 "什么 mechanism 拦截这个违规？" 没有 mechanism 就先建 mechanism（hook / preflight / cargo test / CI gate），不要只加 norm。memory: `feedback_norm_needs_mechanism`

## Docs (按需加载)
| 文档 | 何时加载 |
|------|---------|
| `docs/architecture.md` | 修改 src/ 核心模块时 |
| `docs/economics.md` | 修改经济引擎 (wallet/market) 时 |
| `docs/hardware.md` | SSH/部署/远程操作时 |
| `docs/experiments.md` | 创建或运行实验时 |
| `docs/rules.md` | 触发规则或修改规则时 |

## User
独狼研究员, 零编程基础 vibe coder. 中文为主, 技术术语英文可.
