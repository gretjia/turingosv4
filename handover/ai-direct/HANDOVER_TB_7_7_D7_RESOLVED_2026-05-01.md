---
date: 2026-05-01
session_close: TB-7.7 D7 RESOLVED via architect verdict (B′)
phase_id: P2 (Frame B finalization carry-forward to TB-7R)
predecessor: handover/ai-direct/HANDOVER_TB_7_7_D7_PENDING_2026-05-01.md
successor: handover/tracer_bullets/TB-7R_charter_2026-05-01.md
---

# Handover — TB-7.7 D7 Resolved (2026-05-01)

## TL;DR

Architect verdict 2026-05-01 resolved D7 BLOCKED state with **option B′**:

```text
TB-7R does NOT do per-tactic decomposition.
Keep `complete` tool.
Compound proposal (LLM-output whole calc block in 1 call) = 1 Attempt Node.
Per-tactic decomposition deferred to TB-8+.
```

D7 evidence at `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/`
is **provisionally accepted** under B′. New TB-7R-grade evidence will
supersede it once TB-7R deliverables A-H ship.

## Why B′ (not A or C)

| Option | Outcome | Why not |
|---|---|---|
| A — accept + TB-8 | Ship D7 with documented limitation; defer all per-tactic concerns. | Architect chose stronger framing: not just defer the concern, but reframe the semantic. |
| B — per-tactic split inside TB-7.7 | Walk calc block tactics; emit N WorkTxs with parent_tx chain. | Conflates externalized proposal with private CoT. ChainTape records what the system externalized, not what the model thought internally. If the LLM emits one compound proof in one tool call, that IS one externalized proposal. |
| C — cut `complete` tool | Force per-turn one-tactic; chain naturally lengthens. | Constrains agent capability surface for the sake of a chain-shape claim. Solves the wrong end of the problem (system shape, not what got externalized). |
| **B′** — keep tool, define 1 compound proposal = 1 Attempt Node | TB-7R: proposal-level DAG. TB-8+: if per-tactic tool-call DAG is desired, charter separately for systems that actually emit per-tactic tool calls. | Aligns with constitutional **selective shielding** principle — TuringOS records what the system externalized, not what the model privately considered. |

## What "BLOCKED" status now means

D7 evidence dir `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/`:
- All 7 PASS / GREEN remains valid under B′.
- `chain_oracle_verified: true` remains valid.
- §7 Golden path render remains valid.
- BUT: this evidence is pre-TB-7R. It does NOT satisfy TB-7R's
  Deliverable C (genesis_report.json), Deliverable D (on-chain
  TaskOpenTx + EscrowLockTx), or Deliverable E (annotated as
  `evaluator-attested` for Lean result, not `chain-oracle-derived`).
- Per verdict B1+B4: this dir is grandfathered, NOT rewritten.
  The README will receive a TB-7R grandfathering note as part of
  TB-7R Deliverable E.

## Carry-forward to TB-7R

D7 work product has been re-classified as TB-7.7 closure. The new
work axis is TB-7R per `handover/tracer_bullets/TB-7R_charter_2026-05-01.md`.

Net new TB-7R deliverables (post-verdict):

```text
A. L4 purity audit (read-only) → OBS if violations
B. ChainTape-mode fail-closed for oneshot / OMEGA-full / OMEGA-pertactic
C. genesis_report.json emission at chaintape bootstrap
D. On-chain TaskOpenTx + EscrowLockTx replacing memory preseed in NEW runs
E. Historical evidence README annotations
F. TB-7R smoke (single → half → full)
G. Codex micro-audit + Codex+Gemini ship audit
H. TB-7R ship report
```

## Test gate at handover

`cargo test --workspace`: per `e9cb023` HEAD state, 698 passed / 0 failed
/ 150 ignored (carried forward from D7 PENDING handover; re-verify
before TB-7R Checkpoint 2).

## Next session start

1. Read this file + `handover/tracer_bullets/TB-7R_charter_2026-05-01.md`
   + `handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md`.
2. Confirm verdict bounds, then begin TB-7R Deliverable A (L4 purity
   audit, read-only) as the lowest-risk first step.
3. Update memory entry `project_tb_7r_authorized.md` as TB-7R progresses.
