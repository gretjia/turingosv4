# TuringOS Harness Manual

这份手册是给未来运行本项目的 Codex、Claude Code、fast executor、
clean-context reviewer 和人类架构者看的操作手册。它的目标不是增加一层
僵硬流程，而是让每个 agent 都能通过同一个智慧型 harness contract 工作：
先理解意图，再确定风险和宪法节点，再执行最小可证实变更，最后用证据和独立审计收束。

本手册管“怎么做任务”。最高法仍然是 `constitution.md`。共享入口仍然是
`AGENTS.md`。架构说明见 `HARNESS.md`。

## 1. 最小心智模型

TuringOS 是 tape-first 的 agent OS。任何开发任务都应被看成一次小型的
自举式运行：

```text
Human Intent
-> Cortex plan
-> Module / Molecule / Atom contract
-> Executor implementation
-> Verification
-> Clean-context review when required
-> Close / summarize
```

核心原则：

- 报告不是事实源。stdout 不是事实源。dashboard 不是事实源。
- 可重建的 tape/CAS/gate/evidence 才能支撑有效主张。
- Harness 的智慧来自“分级、证据、独立审计、可复原”，不是来自更长的 prompt。
- Molecule 是默认效率单位。Atom 是高风险保险丝。
- Veto-AI 只判违宪 `{PASS, VETO}`。普通工程审计由 clean-context reviewer 判
  `PROCEED | CHALLENGE | VETO`。

## 2. Cold Start

任何非平凡任务开始前，先读这些文件：

1. `AGENTS.md`
2. `HARNESS_MANUAL.md`
3. `CLAUDE.md`，如果当前 agent 是 Claude Code
4. `constitution.md`
5. `handover/ai-direct/LATEST.md`
6. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
7. `handover/alignment/TRACE_FLOWCHART_MATRIX.md`
8. 当前任务涉及的 source、tests、runner、evidence 文档

读取后必须能回答：

- 这次 human intent 是什么？
- 触碰哪些 FC1/FC2/FC3 节点或不变量？
- 风险等级是 Class 0 到 Class 4 的哪一级？
- 这是 module、molecule 还是 atom？
- 允许修改哪些路径？
- 需要跑哪些 acceptance commands？
- 是否需要 clean-context audit（任意有能力平台）？

如果这些问题答不出来，不要开始写代码。

## 3. Truth Order

冲突时按这个顺序裁决（3-tier 扁平化；见 `handover/architect-insights/K-2-2_TRUTH_TIER_GREP_RECEIPTS.md` 获取 src/ 查询证据）：

**Tier 1: Axioms**（编译/启动时检查，不可变）
1. `constitution.md`
2. 3 个规范 flowchart hashes（存储在测试和文档中）

**Tier 2: Facts**（实时状态机）
3. ChainTape（L4 + L4.E transitions）
4. CAS（evidence objects，indexed by content hash）
5. Replay/audit verifier（从 ChainTape + CAS 确定性重建）

**Tier 3: Workspace pointers**（可变，派生）
6. Current charter / directive / ratification
7. `handover/ai-direct/LATEST.md`（explicit derived view；冲突时信 ChainTape）

**Derived views**（所有 tier 3 以下 — per K-2.2 receipts 无 src/ runtime reader）：
- `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- `handover/alignment/TRACE_FLOWCHART_MATRIX.md`
- `handover/tracer_bullets/TB_LOG.tsv`
- dashboard、report、README、stdout、agent summary

如果 stdout 说成功，但 gate 或 tape 不能重建成功，按失败处理。
如果 report 和 ChainTape/CAS 冲突，信 ChainTape/CAS。
如果 ChainTape/CAS 和 constitution gate 冲突，停止并升级。

## 4. Task Contract

每个非平凡任务都应先形成一个 task contract：

```text
title: human-readable task title
module: long-lived capability line
unit: molecule | atom
risk_class: 0 | 1 | 2 | 3 | 4
fc_nodes: FC1/FC2/FC3 nodes or invariants
allowed_paths: write surface
acceptance_commands: verification commands
audit_required: derived from risk and blast radius
```

术语：

- Module: 长期能力线，例如 `Harness`、`G3 observability`、`PromptCapsule runtime`。
- Molecule: 默认执行单位，适合低/中风险相关变更共享上下文和验证。
- Atom: 高风险单位，适合 Class 3/4、schema、sequencer、canonical signing、
  CAS integrity、constitution/flowchart。

Cortex 或 planning agent 可以建议压缩为 molecule，也可以建议拆成 atom。
但它没有越权权。静态路径 hard-lock 和宪法风险地板可以强制升级。

## 5. Risk Classes

用这个表做初判：

| Class | 含义 | 默认要求 |
| --- | --- | --- |
| 0 | docs、计划、handover、非权威说明 | diff check，必要时 manual review |
| 1 | additive helper、parser、formatter、非权威 view | targeted test 或 command evidence |
| 2 | production wire-up、evaluator adapter、dashboard、replay verifier、benchmark harness | targeted tests，相关 gates，command evidence |
| 3 | auth、money、CAS integrity、capability、market/economic state、production evidence、`audit_tape` | real evidence，broad tests，clean-context audit |
| 4 | constitution、flowcharts、sequencer admission、typed tx schema、canonical signing payload、RootBox/kernel authority | per-atom section-8 ratification，real evidence，clean-context audit |

Class 4 必须有明确的 per-atom §8 architect/user ratification。`ok`、`go`、
`continue`、`can` 这种一句话不是授权。

## 6. Restricted Surfaces

任何实际 diff 触碰这些路径或权威面，都必须停下来重新分级：

- `src/state/sequencer.rs`
- `src/state/typed_tx.rs`
- `src/kernel.rs`
- `src/bus.rs`
- `src/sdk/tools/wallet.rs`
- `src/bottom_white/cas/schema.rs`
- canonical signing payload 或 system key surface
- ChainTape transition ledger authority
- `genesis_payload.toml`
- `constitution.md`
- canonical flowchart/hash authority documents

实际修改这些路径通常是 Class 3/4 候选。文档里引用这些路径不等于触碰
restricted surface，但如果文档改变了规则权威、宪法解释、flowchart/hash，
仍可能成为 Class 4。

## 7. Execution Loop

标准执行循环：

1. 先识别或写下会失败的 gate/test/check。
2. 做最小 scoped edit。
3. 根据失败证据修复。
4. 跑 acceptance commands，保留输出作为证据。
5. 需要审计时，准备 clean-context payload。
6. 验证 gates/evidence/audit 一致后再 ship。

命令失败时，失败本身也是证据。不要删除失败证据；修复后再重跑一遍并保留新输出。

常用 verification：

```bash
git diff --check
cargo check
cargo test --workspace --no-fail-fast
bash scripts/run_constitution_gates.sh
make constitution
cargo fmt --all
cargo clippy --workspace --tests --no-deps
```

开发中可以先跑 targeted tests。ship-level 或 Class 3/4 需要按风险扩大。

## 8. Clean-Context Audit

默认审计是 **platform-agnostic clean-context audit** —— 任意有能力平台上的一名
全新 clean-context 审计者（Claude / Codex / Antigravity / …），不得持有实现
transcript；一名独立见证者即足够。见 `AGENTS.md §9`。

需要 audit 的典型场景：

- Class 3/4。
- ship-path production code。
- restricted surface 候选。
- evidence runner、replay verifier、audit path 改动。
- 任何 executor 不能自证的高风险变更。

Reviewer payload 必须包含：

- human intent
- risk class
- touched FC nodes/invariants
- harness audit standard
- current diff
- evidence paths
- exact verification output
- required verdict: `PROCEED | CHALLENGE | VETO`

Reviewer payload 不能包含：

- executor 的 implementation transcript
- chain-of-thought
- 试错心理过程
- 无关的长期上下文噪音

裁决解释：

- `PROCEED`: 可以继续，但仍不能替代 gates/evidence。
- `CHALLENGE`: 必须修复，或明确 forward deferral 并说明理由。
- `VETO`: 阻断 ship。

## 9. Validate Before Ship

收束任务前，按 fail-closed 原则核对，任何一条不满足都不能 ship：

- acceptance command 缺失或失败，不能 ship。
- audit required 但缺失，不能 ship。
- restricted path 与风险声明不一致，不能 ship。
- Class 4 缺 per-atom §8 ratification，不能 ship。

不要把失败凑成通过。证据不能重建的主张按未完成处理。

## 10. Runner And Batch Policy

任何 runner 会写 `handover/evidence/` 或执行真实问题集时，先调用
`/runner-preflight`，如果当前工具没有这个命令，就手工确认：

1. worktree 是干净的，或 dirty changes 已理解并隔离。
2. binary 和当前 source/HEAD 匹配。
3. evidence 目录不会被重写。
4. risk class 已声明。
5. FC trace 已声明。
6. charter/directive 完整。
7. audit round 状态明确。

不要在 active batch 中编辑 Trust-Root-pinned source files。需要修复时，先中止
batch 或接受该批次废弃，再改代码。

大型 benchmark 前，相关 surface 至少需要 P38/P49 equality、M0、constitution
gates、`HEAD_t`、PromptCapsule、PCP synthetic corpus 处于可接受状态。

## 11. Common Recovery Paths

测试失败：

- 保留失败 command 的 stdout/stderr 作为证据。
- 根据 stdout/stderr 修复。
- 修复后重跑同一条 command，保留新输出。
- 不要把失败输出从证据中删除。

风险升级：

- 停止当前实现。
- 重新声明 risk、FC nodes、allowed paths。
- 如果进入 Class 4，等待 per-atom §8 ratification。

磁盘空间不足：

- 不要删除 evidence。
- 可以清理构建缓存，例如 `target/`，但要知道这会让之后需要重新构建。
- 重新跑失败的 command 并保留新证据。

旧文档过期：

- 不要改历史证据来迎合新规则。
- 在 living docs 或 OBS/annotation 中 forward-supersede。

diff 超出 allowed paths：

- 停止。
- 判断是 accidental drift 还是 task scope 必要扩展。
- 必要时重新声明或更新 task contract，而不是悄悄继续。

## 12. Final Response Checklist

结束任务前，agent 的最终回复应说明：

- 改了哪些文件。
- 跑了哪些 verification commands，以及对应输出/证据路径。
- 哪些命令没有跑，以及为什么。
- 如果有 audit，给出 verdict。
- 如果有风险或后续事项，明确指出。

不要声称“完成”而没有验证证据。不要把计划当成完成结果。
