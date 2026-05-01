---
date: 2026-05-01
session_close: TB-7.7 D7 mid-flight, BLOCKED on user verdict
phase_id: P2 (Frame B finalization carry-forward)
charter: handover/tracer_bullets/TB-7.7_charter_2026-05-01.md
---

# Handover — TB-7.7 D7 Pending User Verdict (2026-05-01)

## TL;DR

TB-7.7 D1-D6 已 commit。D7 (n5 multi-agent smoke) 在收集证据时
surfaced 一个**架构 bug**（用户 2026-05-01 当面识别）：

> "为什么会有单步 solve 的题，这明显不对"

链上仅 1 条 WorkTx ≠ 真正的 CoT externalization。**等待用户对 A/B/C
裁决**再 ship D7。

D1-D7 涉及的修复 patch 已 stage（未 commit）以避免丢失。

## Session Status

### Already Committed (HEAD = 07b6067)
- D1: payload bytes → CAS (a39c31b)
- D2: parent_tx wire (a39c31b 同一 commit)
- D3: pre-seed for L4 accept (054254f)
- D4: VerificationResult CAS object (89cd448)
- D5: chain_oracle_verified / chain_economic_finalized split (901062b)
- D6: audit_dashboard DAG + golden path rendering (07b6067)

### Uncommitted (this session) — D7 path corrections
1. **`experiments/minif2f_v4/src/bin/evaluator.rs`** — D3 EscrowLock
   parent_state_root polling fix. `submit_typed_tx` is async-queued
   (bus.rs:127-130); without polling `bundle.sequencer.q_snapshot()`
   until `state_root_t` advances past ZERO, EscrowLock使用 stale
   parent root → StaleParent rejection.

2. **`src/runtime/mod.rs`** — `build_chaintape_sequencer_with_initial_q`
   现在 always persist `initial_q` 到 `<runtime_repo>/initial_q_state.json`。
   verify_chaintape:264-272 唯一从此文件读 initial_q；缺则 fallback 到
   `QState::genesis()`，pre-seeded balances 丢失，replay 时 EscrowLock
   认 sponsor 余额不足 → state_reconstructed=false。

3. **`src/runtime/chain_derived_run_facts.rs`** — `chain_oracle_verified`
   改走 `accepted_worktx_vr_cid`（任何 accepted L4 WorkTx 带 verified VR
   即 true），不再要求 paired VerifyTx::Confirm。VerificationResult
   是 oracle witness，VerifyTx 是 agent 经济宣言；单 solver run
   (n=1, 无 verifier) 可正确 flip。

4. **`tests/tb_6_verify_chaintape.rs`** — I90 assertion 翻转
   `!initial_q_state_loaded_from_disk` → `initial_q_state_loaded_from_disk`，
   因为 build_chaintape_sequencer_with_initial_q 现总持久化。

5. **`genesis_payload.toml`** — 4 个 hash refresh:
   - `experiments/minif2f_v4/src/bin/evaluator.rs`
   - `src/runtime/mod.rs`
   - `src/runtime/chain_derived_run_facts.rs`
   - `tests/tb_6_verify_chaintape.rs`

### Test Gate
`cargo test --workspace`: **698 passed / 0 failed / 150 ignored** ✓

### D7 Smoke Evidence (captured @ run #5)
`handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/`
- `runtime_repo/` — 3 L4 entries (TaskOpen + EscrowLock + Work✓oracle)
  + 3 L4.E entries (synthetic-seed L4.E gate fixtures)
- `cas/` — 完整 CAS 副本
- `dashboard.txt` — `audit_dashboard` 渲染输出，证据齐全：
  - ALL 7 PASS: GREEN
  - chain_oracle_verified: **true ✓**
  - §7 Golden path 渲染 (root → ORACLE depth=0)
  - initial_q_state_loaded_from_disk: true

**Run config**: `mathd_algebra_171.lean`, CONDITION=n5, TURINGOS_CHAINTAPE_PRESEED=1,
TURINGOS_RUN_ID=tb7-7-smoke-5, run_id="n5_mathd_algebra_171_1777652328726".

## 架构师裁决要求 (架构 finding)

**用户 2026-05-01 现场识别**：mathd_algebra_171 链上仅 1 条 WorkTx，
而 PPUT_RESULT.tool_dist 是 `{step:1, omega_wtool:1}`（多 tactic 推理）。
LLM 选择 `complete` 工具一次性返还完整 calc 证明：

```lean
calc
  f 1 = 5 * 1 + 4 := by rw [h₀]
  _ = 5 + 4 := by ring
  _ = 9 := by norm_num
```

evaluator.rs L2153 OMEGA-pertactic 站点把整个 calc 块打包写 1 条
ProposalTelemetry + 1 条 WorkTx，**不是** 把 calc 内 3 个 tactic step 拆 3 条。

**违反 v4 哲学** "TuringOS scaffold IS externalized CoT" — 链上 = 答案，
不是思维过程。

### 三选项

- **(A)** 接受现状 + TB-8 处理。TB-7.7 ship D7 with documented
  known limitation。
- **(B)** TB-7.7 内补 per-tactic 拆 WorkTx。OMEGA-pertactic 站点
  遍历 tactic vec 写 N 条 WorkTx，parent_tx = 前一 tactic 的 tx_id。
- **(C)** 砍 agent 的 `complete` 工具，强制每 turn 仅一个 tactic
  step → chain 自然长成。

## Next Session 启动指引

1. 读本文件 + `handover/tracer_bullets/TB-7.7_charter_2026-05-01.md`
2. 确认 D7 patches 5 个文件 stage state（git status 应显示
   evaluator.rs / mod.rs / chain_derived_run_facts.rs /
   tb_6_verify_chaintape.rs / genesis_payload.toml uncommitted；
   或者本 handover 提交时已合并 — 见 git log）
3. 等待用户 A/B/C verdict → 决定 D7 ship 范畴
4. ship 时：commit + 更新 TB_LOG.tsv (TB-7.7 row) + RECURSIVE_AUDIT
   (D1-D7 line-grounded)

## 跨 session 状态 reference

- Charter: `handover/tracer_bullets/TB-7.7_charter_2026-05-01.md`
- Architect ruling 2026-05-01 (Path Y, 2 layers): same file §1
- Forbidden #34/#35/#36 (no new TypedTx variant for VR;
  chain_economic_finalized stays false in TB-7): charter §6
- Phase B 5-problem evidence (pre-D7): `handover/evidence/tb_7_chaintape_smoke_2026-05-01/`
- D7 evidence: `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/`
