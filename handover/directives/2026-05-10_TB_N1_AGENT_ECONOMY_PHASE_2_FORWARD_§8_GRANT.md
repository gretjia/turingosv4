# TB-N1-AGENT-ECONOMY Phase 2 — Forward §8 Grant (2026-05-10 session #35)

**Status**: ACTIVE forward Class-4 grant. Scope: A3 + A4 serial execution.

## §1. User verbatim authorization

Session #35 2026-05-10:

> 批准 charter + 授权 A3 + A4 串行全授权

**Multi-clause structural analysis** (CLAUDE.md §10):

| Clause | Named act | Scope | Type |
|--------|-----------|-------|------|
| 1 | `批准` (approve) | charter `handover/tracer_bullets/TB_N1_AGENT_ECONOMY_PHASE_2_charter_2026-05-10.md` | Charter ratification |
| 2 | `授权` (authorize) + `串行全授权` (serial full authorization) | A3 then A4 (sequential; no batching) | Forward Class-4 grant |

**Structural equivalence with canonical Class-4 §8 forms**:
- TB-C0 "好，确认可以 ship"
- Stage A3 "同意 sign-off"
- Stage C "授权自主执行直到polymarket全部落地并自主开展真题测试"
- Stage C P-M2/P-M4/P-M6 各自 per-atom §8

User's clause 2 authorization is structurally equivalent to Stage C overall §8 multi-clause grant — same forward batch shape; conditional on PRE-§8 dual audit PASS per atom (per `feedback_dual_audit` Class-4 timing rule + `feedback_no_batch_class4_signoff`).

## §2. Authorized scope

| Item | Status |
|------|--------|
| **Charter** TB-N1-AGENT-ECONOMY Phase 2 | RATIFIED |
| **A3** agent-decided stake (Class-4 STEP_B) | AUTHORIZED for execution |
| **A4** agent-callable verify-peer (Class-4 STEP_B) | AUTHORIZED for execution after A3 ships |
| Per-atom §8 (forward grant; conditional on dual audit PASS) | PRE-RATIFIED |

## §3. Forbidden scope (out of grant)

Inherited from charter §4 + CLAUDE.md §20 freeze conditions:
- **NO M2 batch run** (1800-cell SG-B3.1-6) during Phase 2 — sequencer admission change in flight
- **NO Polymarket-agent-bridge (A6)** — Stage D-aligned; needs separate architect §8
- **NO swarm n>1 batch** — substrate must close before swarm makes sense
- **NO new typed_tx variant** — RejectionClass tail-append only
- **NO canonical signing payload change** — WorkTx + VerifyTx signing payloads unchanged
- **NO push to origin/main** without architect §8 sign-off (per-atom)

## §4. Conditional gates (must hold for grant to remain valid)

For A3 (and similarly A4):
1. STEP_B parallel-branch development (NOT direct main edit) per `feedback_step_b_protocol`
2. PRE-§8 dual audit (Codex G2 + Gemini DeepThink) — BOTH PROCEED required per `feedback_dual_audit` Class-4
3. If either audit returns VETO or CHALLENGE → remediate + re-audit; round cap = 2 per `feedback_elon_mode_policy` (round 3+ requires explicit user authorization + `/harness-reflect` first)
4. Conservative-merge resolution: VETO > CHALLENGE > PASS per `feedback_dual_audit_conflict`
5. Per-atom §8 sign-off file: `handover/directives/2026-05-XX_TB_N1_AGENT_ECONOMY_A<n>_§8_SIGN_OFF.md` cites this forward grant + R<final> dual audit PASS

## §5. Rollback requirement

If A3 ships then A4 PRE-§8 dual audit returns VETO that cannot be remediated within 2 rounds:
- A3 stays shipped (already ratified by its own §8)
- A4 enters `handover/directives/2026-05-XX_A4_VETO_REMEDIATION_DIRECTIVE.md` per CLAUDE.md §10
- Forward grant is NOT auto-revoked; user may explicitly revoke via verbatim "撤销 A4 授权" or equivalent

If A3 itself returns VETO that cannot be remediated within 2 rounds:
- Roll back any partial A3 commits per `feedback_no_workarounds_strict_constitution`
- File `handover/directives/2026-05-XX_A3_VETO_REMEDIATION_DIRECTIVE.md`
- Forward grant for A4 PAUSED until A3 closure path determined

## §6. Allowed files / surfaces

Per charter §6 risk envelope:

**Class-4 surfaces (STEP_B parallel-branch required)**:
- `src/state/sequencer.rs` — admission arm extension
- `src/state/typed_tx.rs` — RejectionClass tail-append (NOT new variant)

**Class-3 surfaces (audit before ship)**:
- `src/sdk/protocol.rs` — tool action schema
- `src/sdk/prompt.rs` — tool schema doc
- `experiments/minif2f_v4/src/bin/evaluator.rs` — action dispatch + WorkTx construction

**Trust Root files affected (rehash required at each atom ship)**:
- `src/state/sequencer.rs`
- `src/state/typed_tx.rs`
- `experiments/minif2f_v4/src/bin/evaluator.rs`

## §7. Ship gates per atom

A3:
- SG-N1-A3.1..6 per charter §2 + Trust Root rehash + `bash scripts/run_constitution_gates.sh` GREEN

A4:
- SG-N1-A4.1..7 per charter §2 + Trust Root rehash + `bash scripts/run_constitution_gates.sh` GREEN

## §8. Forward grant declaration

This document constitutes the architect §8 forward grant for TB-N1-AGENT-ECONOMY Phase 2. Per CLAUDE.md §10 multi-clause analysis: user's verbatim "批准 charter + 授权 A3 + A4 串行全授权" satisfies Class-4 §8 form. Conditional on:
1. Per-atom PRE-§8 dual audit PASS (BOTH Codex + Gemini PROCEED)
2. Strict ordered execution: A3 fully ships BEFORE A4 starts
3. STEP_B parallel-branch protocol per `feedback_step_b_protocol`
4. NO batching of §8 sign-offs per `feedback_no_batch_class4_signoff`

Conditions met → atom ships under this forward grant + per-atom §8 sign-off doc. Conditions failed → grant pauses for that atom; user re-authorization required to proceed.

---

**End of TB-N1-AGENT-ECONOMY Phase 2 forward §8 grant.**
