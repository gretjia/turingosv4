# TB-7.7 D7 Smoke Evidence — n5 multi-agent ChainTape (2026-05-01)

## Run config
- Problem: `mathd_algebra_171.lean`
- CONDITION: `n5` (5-agent swarm)
- TURINGOS_CHAINTAPE_PRESEED: `1` (D3 sponsor + 10-agent balance pre-seed)
- TURINGOS_RUN_ID: `tb7-7-smoke-5`
- run_id: `n5_mathd_algebra_171_1777652328726`
- Model: `deepseek-v4-flash`

## Result
- SOLVED + VERIFIED
- pput_runtime: 0.000222
- gp_path: `per_tactic`
- gp_payload: 3-step calc (`rw [h₀]` → `ring` → `norm_num`)
- tx_count: 1 (chain-side)

## Captured artifacts
- `dashboard.txt` — `audit_dashboard` 完整渲染
- `replay_report.json` — `verify_chaintape` 7 indicators output
- `run_summary.json` — `gen_run_summary` aggregated facts
- `agent_pubkeys.json` — agent identity manifest (3 entries)
- `initial_q_state.json` — D7 fix: persisted pre-seeded QState
- `rejections.jsonl` — L4.E records (3 entries)
- `agent_audit_trail.jsonl` — agent audit trail (TB-6 Atom 5)

## D7 evidence checklist (architect ruling)

| Requirement | Met | Where |
|---|---|---|
| ≥2 agent_ids | ✓ | `dashboard.txt §4` (Agent_0, tb6-smoke-agent, tb6-smoke-sponsor, tb7-7-sponsor) |
| ≥1 accepted L4 WorkTx | ✓ | `dashboard.txt §5` (Agent_0 step_complete @ logical_t=3 with oracle ✓) |
| ≥1 L4.E rejection | ✓ | `rejections.jsonl` (3 records) |
| ≥1 oracle_verified node | ✓ | `dashboard.txt §3` (chain_oracle_verified=true) + `§7` golden path |
| Golden path from ChainTape+CAS | ✓ | `dashboard.txt §7` (depth=0 ORACLE node with proof payload preview) |
| ≥1 non-empty parent_tx edge | ✗ (mathd_algebra_171 单步 LLM `complete` 命中 → 1 proposal/agent → 无 edge) |

## Known limitation surfaced by this smoke (architect attention)

User 2026-05-01 现场识别：链上仅 1 条 WorkTx ≠ externalized CoT。
LLM 选 `complete` 工具一次性返还完整 calc 证明，OMEGA-pertactic
站点把整块写 1 条 WorkTx，**不是**按 calc 内 3 个 tactic step 拆 3 条。

详见 `handover/ai-direct/HANDOVER_TB_7_7_D7_PENDING_2026-05-01.md`。
等用户 (A) ship + TB-8 处理 / (B) TB-7.7 内补 / (C) 砍 `complete` 工具 verdict。

## TB-7R grandfathering note (2026-05-02)

**This evidence is grandfathered under TB-7R**:
- Predates TB-7R Deliverable C (`genesis_report.json` emission requirement) and Deliverable D (on-chain `TaskOpenTx` + `EscrowLockTx` requirement). The accepted L4 `TaskOpen` + `EscrowLock` here use the D3 memory-preseed bootstrap (commit `054254f`) — NOT the on-chain bootstrap that TB-7R-grade evidence requires.
- The D7 BLOCKED-on-A/B/C verdict was resolved 2026-05-01 via option **B′** (proposal-level DAG; per-tactic decomposition deferred to TB-8+). The §7 golden path single-WorkTx shape is correct under B′ — the LLM emitted one compound externalized proposal, which is one Attempt Node by definition.
- L4 purity audit 2026-05-02 (`handover/audits/L4_PURITY_AUDIT_TB7R_2026-05-02.md`): the single in-scope L4 Work entry passes all four TB-7R purity criteria (ProposalTelemetry resolves, verification_result_cid resolves, VerificationResult.verified=true, proof_artifact_cid resolves).
- This dir SHOULD NOT be cited as TB-7R-grade ChainTape evidence. For TB-7R-grade evidence see `handover/evidence/tb_7r_*_2026-05-XX/` once Deliverable F smoke ships.

## D7 patches surfaced by this smoke

1. evaluator.rs: D3 EscrowLock parent_state_root async-poll
   (submit_typed_tx 异步入队，必须等待 q_snapshot 变化)
2. src/runtime/mod.rs: build_chaintape_sequencer_with_initial_q
   现 always persist initial_q to disk (replay 端必读)
3. src/runtime/chain_derived_run_facts.rs: chain_oracle_verified
   走 accepted_worktx_vr_cid (任何带 verified VR 即 true)，
   不再要求 paired VerifyTx::Confirm
4. tests/tb_6_verify_chaintape.rs: I90 assertion 翻转

## Reproduce
```bash
mkdir -p /tmp/tb7_7_repro/{runtime_repo,cas}
TURINGOS_CHAINTAPE_PATH=/tmp/tb7_7_repro/runtime_repo \
TURINGOS_CAS_PATH=/tmp/tb7_7_repro/cas \
TURINGOS_RUN_ID=tb7-7-repro \
TURINGOS_CHAINTAPE_PRESEED=1 \
CONDITION=n5 \
cargo run -p minif2f_v4 --bin evaluator -- mathd_algebra_171.lean

cargo run --bin audit_dashboard -- \
  --repo /tmp/tb7_7_repro/runtime_repo \
  --cas /tmp/tb7_7_repro/cas
```
