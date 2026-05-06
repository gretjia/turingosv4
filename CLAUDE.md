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

### 12 个持久 Constitutional CI tests (Class 3/4 必须实现并维护)
```
fc1_every_externalized_attempt_is_tape_visible
fc1_predicate_pass_goes_l4_fail_goes_l4e
fc2_run_replayable_from_genesis_tape_cas
fc2_no_memory_only_preseed
fc3_capsule_derived_from_tape_cas
no_global_markov_pointer
no_dashboard_source_of_truth
no_legacy_authoritative_append
no_fake_accepted_nodes
no_f64_money_path
total_coin_conserved
system_tx_not_agent_submittable
```

### Kill gates (任一即停)
1. `evaluator_reported_tx_count != chain_attempt_count`
2. N>1 externalized attempts but `chain_attempt_count = 1`
3. Lean reject 仅在 stdout，不在 L4.E / EvidenceCapsule
4. Final composite proof 缺 `attempt_chain_root` 或等价 lineage
5. Dashboard 需 evaluator stdout 才能重建核心事实
6. 出现 fake accepted node
7. CTF conservation 失败
8. ChainTape mode 静默回落 legacy `bus.append`
9. Global Markov pointer 复现
10. PartialAccepted schema 产生 untyped `exit_code=0, verified=false, error_class=None` 歧义

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

## Active state (动态指针，不存判决)
- TB 总账 (authoritative): `handover/tracer_bullets/TB_LOG.tsv`
- 当前操作模式: **Constitutional Harness Engineering** (since 2026-05-06)
- **当前 charter: TB-C0 — CLOSURE CANDIDATE** (closure report: `handover/tracer_bullets/TB-C0_CLOSURE_REPORT_2026-05-06.md`)
- TB-C0 empirical state: 25 FC nodes; 21 GREEN + 4 AMBER (structural-only by design) + 0 RED + 0 GAP per `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate.json`
- **Awaiting**: (1) architect §8 sign-off on TB-C0 closure, (2) Codex + Gemini external dual audit (CR-C0.8 — happens AFTER MVP gates green)
- **3 known accounting bugs** documented in `handover/alignment/OBS_TBC0_FC1_INV3_THREE_BUGS_2026-05-06.md` (Bug 1 Class 2: runner uses tx_count vs LLM-cycle count; Bug 2 Class 3: synthetic L4.E gate; Bug 3 Class 4 STEP_B: missing `capsule_anchored_attempt_count` field). FORWARD-bound; NOT bundled into TB-C0
- TB-18R status: **subordinate to TB-C0**; CANDIDATE REMEDIATION; ships after TB-C0 closes via final dual audit + §8
- **HARD FREEZE (until TB-C0 SHIPS FINAL)**: ALL feature TBs (TB-19+, NodeMarket, Polymarket, PriceIndex, public-chain, real-world-readiness), MiniF2F M1/M2/M3, formal benchmark claims
- **Constitution gate runner**: `bash scripts/run_constitution_gates.sh` or `make constitution` → 54/0/1 GREEN. CI workflow `.github/workflows/constitution_gates.yml` is required merge gate
- **FC-witness extractor**: `python3 scripts/fc_witness_extract.py <run_dir>` (single-problem) + `scripts/fc_witness_aggregate.py <batch_dir>` (multi-problem)
- **Real-problem catalog**: `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md` (binds every FC node to real existing MiniF2F problems; per `feedback_real_problems_not_designed`)

## Memory (跨 session 持久; auto-loaded MEMORY.md 是 hot index)
- 高频 rule (feedback_*) + 项目状态 (project_*) + 外部引用 (reference_*) 路径: `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/`
- 写新 memory: 文件名 `<type>_<topic>.md` 加 frontmatter + 在 MEMORY.md 加 ≤150 字符 hook
- 不要在 memory 里复述 TB_LOG.tsv 已有的 ship facts; 只记 session-level 教训和 surprise

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
