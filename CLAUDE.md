# TuringOS v4

## What
Silicon-Native Microkernel for LLM Formal Verification Swarm.
Rust 2021, tokio, serde_json. Mission: MiniF2F Lean 4.

## Why
- 唯一对齐文档: `constitution.md` (反奥利奥架构)
- 压缩即智能: 抽象原则进宪法 / 具体情境进 `cases/`
- 机制 > 参数 > 提示 (Art. V + C-021/C-031/C-034/C-043)

## Code Standard (Art. I.1 + C-004 + C-027)
- `cargo check` / `cargo test --workspace` 必过；`.env` 永不 commit
- STEP_B_PROTOCOL（不直接编辑 main）适用于:
  - `src/kernel.rs` + `src/bus.rs` + `src/sdk/tools/wallet.rs` + `src/state/sequencer.rs` (kernel/economy/admission)
  - `src/state/typed_tx.rs` + `src/bottom_white/cas/schema.rs` (Class-4 typed-tx + CAS schema; TB-18R 2026-05-06 加入)
- 任何影响行为的参数必须 env/config 可覆盖，不可硬编码

## Audit Standard (Art. V.1 + C-010 + C-023 + C-035)
- Generator ≠ Evaluator：代码作者不可是唯一审计者
- 所有 merge / phase 决策双外审（Codex + Gemini）；VETO > CHALLENGE > PASS
- 宪法违规立即 BLOCKER，不可延期、不可"可接受"

## Report Standard (Art. I.2 + Art. II.2.1 + Art. IV 强制, C-052 + C-053 + C-057 + C-059 + C-061)
- **主指标**（每报必填）: ΣPPUT + Mean PPUT (solved) + 95% CI (Wilson)
- Art. I.2 三大统计信号不可缺: **信誉** (reputation_distribution p50/p90/max) + 效用 (PPUT) + 共识 (如适用)
- Art. IV 终态区分: `halt_reason_distribution` {OmegaAccepted, MaxTxExhausted, WallClockCap, ComputeCapViolated, ErrorHalt}
- 多 agent (n≥2) 专用: `parent_selection_entropy` + `pairwise_payload_diversity_mean`；任一 < 0.25 = Art. II.2.1 告警
- solve count 不可独立陈述，必须配对 PPUT；以 solve count 起头 = 违宪

## Reproducibility Standard (Art. I + C-012/C-016/C-032/C-039)
- OMEGA accept 必留 self-contained artifact (`proofs/*.lean` + `gp_payload`)
- 度量工具上线即冻结；Oracle 参数冻结；实验禁混 Oracle 模式
- 中间件若修改数学内容 → 是 ArchitectAI 贡献，不是 swarm 涌现（C-023）

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
- 当前 charter: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md` (Class 4, 等 Codex Gate 1)
- 当前 FREEZE: TB-18 M1/M2/M3 + NodeMarket + PriceIndex + Polymarket-signal + public-chain + real-world-readiness; 全部解锁需 TB-18R SHIPPED FINAL with G2 PASS

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
