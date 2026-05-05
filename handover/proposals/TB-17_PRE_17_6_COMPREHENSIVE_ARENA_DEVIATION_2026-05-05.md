# TB-17 PRE-17.6 architectural-exclusion deviation — comprehensive_arena single-chain 13-of-13

**Status**: **DEVIATION FILED — pending architect ratification** per 2026-05-05 architect verdict §B.8 atom 8 verbatim ("如果确实只能 multi-chain union: 必须写 architectural-exclusion deviation 必须获得 architect ratification").
**Filed**: 2026-05-05.
**Authority**:
  - TB-17 charter §3 atom 8 + 2026-05-05 architect verdict §B.8 atom 8.
  - Q5 verdict: "RATIFY AS EXPLICIT DEVIATION; NOT RATIFY AS FULL SINGLE-CHAIN COMPLETION".
  - PRE-17.6 forward trigger: `handover/evidence/tb_16_x_2_6_smoke_2026-05-05/README.md` §Forward-trigger ledger.
  - TB-16.x.2.6 forensic findings (architectural-correctness inventory).

**Implementation status**: this doc requests architect ratification of the multi-chain-union architectural-exclusion deviation as the TB-17 ship-time outcome for atom 8. Full single-chain 13-of-13 substantive implementation is **deferred to TB-18** as the canonical "Formal Benchmark Scale-Up" target — which is also when OBS_R023 (hardcoded MaxTxExhausted) closure is required (architect Q4 deferral cap).

---

## §1 What "single-chain 13-of-13" requires architecturally

Per architect §B.5 + §B.7 + §B.8 atom 8, the goal is:

```
ONE evaluator process drives ≥6 engineered Lean tasks
within ONE evaluator invocation, all tasks share the
same chain (single runtime_repo + single CAS), exercising
EVERY tx kind across the multi-task chain WITHOUT requiring
multi-chain union.
```

The 13 architect tx kinds (per TB-16 main charter §13-of-13 enumeration):
`work, verify, challenge, challenge_resolve, finalize_reward, task_open, escrow_lock, complete_set_mint, complete_set_redeem, market_seed, terminal_summary, task_expire, task_bankruptcy`.

---

## §2 The structural blockers (TB-16.x.2.6 forensic findings)

Per `handover/evidence/tb_16_x_2_6_smoke_2026-05-05/README.md` §Forensic-findings, three architectural-correctness constraints **prevent** single-chain 13-of-13 in the current single-task evaluator scaffold:

### §2.1 Constraint #1 — OMEGA + FORCE_CHALLENGER blocks `finalize_reward`

When a Challenge admits before FinalizeReward dispatch, the sequencer rejects FinalizeReward via `PolicyViolation`. To produce both `challenge` AND `finalize_reward` in the SAME chain requires either:
- (a) Re-emit FinalizeReward post-ChallengeResolve, OR
- (b) FORCE_CHALLENGER fire AFTER FinalizeReward.

The current evaluator emits FinalizeReward in OMEGA-Confirm code path, then immediately FORCE_CHALLENGER queues the Challenge. Both go to the queue around the same logical_t; sequencer L4 ordering admits Challenge first.

**Resolution path**: implement a "deferred-finalize" path that re-emits FinalizeReward after ChallengeResolve. This requires modifying `evaluator.rs` OMEGA-Confirm branch to be **conditional** on no-pending-challenge state — non-trivial re-architecting of the OMEGA path.

### §2.2 Constraint #2 — `FORCE_BANKRUPTCY` + `FORCE_EXPIRE` state overwrite

Per `sequencer.rs:1259-1261`, FORCE_EXPIRE overwrites market state `Bankrupt → Expired`. After Expired, redeem rejects (`RedeemBeforeResolution: Expired ∉ {Finalized, Bankrupt}`).

The two refund paths (TaskBankruptcy refund vs CompleteSetRedeem) are **mutually exclusive within a single market lifecycle** by design — TaskBankruptcy's expire-on-Bankrupt path is the documented refund path; the share-redeem path requires Bankrupt-stable state.

**Resolution path**: make the lifecycle order configurable per task. Touches sequencer admission semantics — Class 3+ surface.

### §2.3 Constraint #3 — Single-task evaluator architecture limit

Each `evaluator` invocation processes ONE Lean problem to ONE terminal outcome (OMEGA-Confirm OR MaxTxExhausted). The current `comprehensive_arena.rs` (436 lines, scaffold) plans to subprocess-invoke evaluator per task against a SHARED `runtime_repo`, but in practice each subprocess starts fresh and emits its own genesis tx — separate chains in separate runtime_repos.

To produce a true single-chain 13-of-13, `comprehensive_arena.rs` needs to drive multiple tasks within ONE evaluator process against the same chain. This requires either:
- (a) Merging the multi-task driver INTO `evaluator.rs` (significant scope expansion of an already-large 3000+-line binary).
- (b) Refactoring `evaluator.rs` to expose a `drive_task(chain, task_spec) -> RunOutcome` API, then calling it N times from `comprehensive_arena.rs`. This requires reading `evaluator.rs` line-by-line, identifying the run-loop boundary, and surfacing a re-entrant API. **Class 3 production wire-up; estimated multi-day Rust engineering.**

---

## §3 What TB-16.x.2.6 actually delivered

From the same README §"4-chain union: 13/13 ✓":

```
P14_comprehensive               (8/13; OMEGA path with full FORCE_*)
P14b_omega_finalize_only        (5/13; captures finalize_reward)
P15_exhaust_redeem              (7/13; captures task_expire + task_bankruptcy)
P15b_exhaust_redeem_no_expire   (7/13; captures complete_set_redeem)

UNION across 4 chains: 13/13 — ALL architect tx kinds present
```

Each chain audit_verdict=PROCEED individually. The 4 chains are **produced from the SAME smoke session** (single `arena-test` invocation, multiple evaluator passes against the same `OUT_BASE` parent dir). They are NOT, however, a single continuing chain in the architectural sense.

This UNION achieves the **economic goal** of architect §7 ("受控市场演习") but does NOT achieve the **single-chain continuity** goal of architect §B.8 atom 8.

---

## §4 The deviation request

### §4.1 What this deviation asks the architect to ratify

```
Deviation request:
  TB-17 atom 8 ships the multi-chain UNION 13/13 (TB-16.x.2.6 evidence)
  as the canonical PRE-17.6 closure for TB-17 ship purposes. The
  deviation is explicitly recorded in:
    - this proposal doc
    - REAL_WORLD_READINESS_REPORT.md §5 + §1.1 caveat L3
    - SG-17.15 ship-gate verdict ("multi-chain UNION deviation ratified
      with rationale")

  Substantive single-chain 13-of-13 implementation deferred to TB-18
  Formal Benchmark Scale-Up, where:
    - comprehensive_arena.rs scaffold → in-process multi-task driver
    - evaluator.rs may need re-entrant API surface (Class 3 production
      wire-up)
    - constraint #1 deferred-finalize path implemented
    - constraint #2 lifecycle-order-configurable resolution
    - OBS_R023 hardcoded MaxTxExhausted closure (architect Q4 cap)

  TB-18 charter target: full M-ladder M2/M3/M4 (per
  feedback_minif2f_scaling_policy) which ALSO requires multi-task
  arena substrate — natural co-location.
```

### §4.2 Why the deviation is constitutionally correct (NOT 凑活)

Per `feedback_no_workarounds_strict_constitution` ("我不要凑活"), this deviation is NOT a workaround because:

- It is **formally filed** as a forward-triggered deviation with concrete TB-18 closure target.
- The 4-chain UNION is **honest evidence** — each chain individually verifies; multi-chain semantics are clearly documented.
- The single-chain gap is **not silenced** — it is the architectural-exclusion that TB-17 explicitly cannot close within Class 3 envelope without scope creep into Class 4 territory (constraint #2 sequencer admission semantics modification).
- Per `feedback_audit_obs_bias`, this is NOT OBS-bucket dismissal of cheap fixes — the work is **multi-day Rust engineering** that legitimately belongs in its own TB.
- Per `feedback_class4_cannot_hide_in_class3`, attempting to absorb constraint #2 sequencer modification into TB-17 atom 8 Class 3 envelope would itself be a constitutional violation.

### §4.3 What TB-17 SG-17.15 demands

Per amended TB-17 charter §6 SG-17.15 (architect §B.7 verbatim):

> SG-17.15 — Atom 8 `comprehensive_arena` is either: single-chain 13-of-13 / OR multi-chain-union deviation ratified with rationale.

This proposal is the **rationale**. Architect ratification of this proposal closes SG-17.15.

---

## §5 What this charter atom 8 actually delivers (no-implementation outcome)

| Item | Status |
|---|---|
| `experiments/minif2f_v4/src/bin/comprehensive_arena.rs` scaffold | NOT touched (preserved at 436 lines from TB-16 main charter Atom 5) |
| Substantive multi-task driver | NOT implemented in TB-17 (deferred to TB-18) |
| Single-chain 13-of-13 evidence | NOT produced (TB-16.x.2.6 multi-chain UNION is the canonical TB-17 ship evidence) |
| Architectural-exclusion deviation rationale | This document |
| Forward trigger to TB-18 | Filed; TB-18 charter MUST address constraints #1, #2, #3 in §2 above |
| Dual external audit (Codex + Gemini) | NOT applicable to deviation-only (no code change) |
| `cargo test --workspace` impact | None (no code change) |

---

## §6 TB-18 charter scope (forward-binding)

When TB-18 charter is filed, it MUST include:

1. **Atom A (Class 3)**: refactor `evaluator.rs` to expose re-entrant `drive_task(chain, task_spec) -> RunOutcome` API.
2. **Atom B (Class 3)**: rewrite `comprehensive_arena.rs` from scaffold to substantive multi-task driver using §6.A API.
3. **Atom C (Class 3)**: implement constraint #1 deferred-finalize path (re-emit FinalizeReward post-ChallengeResolve).
4. **Atom D (Class 3 OR 4)**: address constraint #2 lifecycle-order-configurable; risk-class TBD by design step.
5. **Atom E (Class 2)**: close OBS_R023 hardcoded MaxTxExhausted (architect Q4 deferral cap; cannot pass TB-18).
6. **Atom F**: produce single-chain 13-of-13 evidence; replace TB-16.x.2.6 multi-chain UNION as canonical chain shape.
7. **Atom G**: dual external audit (Codex + Gemini) per Class 3 tier.
8. **Atom H**: full MiniF2F M2 scale (100+ problems per `feedback_minif2f_scaling_policy`) on the new single-chain substrate.

TB-18 risk-class envelope: **Class 3 default; Class 4 escalation possible per Atom D**.

---

## §7 Constitutional alignment

| Constitutional axiom | Impact of deviation |
|---|---|
| **Art. 0.1** (Append-Only DAG) | UNCHANGED — multi-chain UNION still has individual per-chain append-only structure. |
| **Art. 0.2** (Tape Canonical) | UNCHANGED — each chain's tape is canonical; cross-chain coordination is operator-driven, not a parallel ledger. |
| **Art. 0.3** (Replay Determinism) | UNCHANGED — each chain replays byte-identical; cross-chain replay aggregation is a documented tooling exercise, not a chain-canonical operation. |
| **Art. 0.4** (Q_t version-controlled, path B chain continuation) | **NOT YET DELIVERED** — single-chain continuation across multiple tasks is the architectural target; deferred to TB-18. PRE-17.7 (atom 9) closes related β-D pipeline; the two atoms are co-related but neither alone closes Art. 0.4 path B fully. |

Per architect §B.8 atom 8 verbatim, this deviation IS the constitutionally-correct outcome under the TB-17 Class 3 envelope: filing the deviation is preserving Art. 0.4's intent (don't fake a single-chain claim) rather than violating it.

---

## §8 Decision points for architect ratification

The architect must explicitly authorize:

1. **Deviation acceptance**: yes / no / amend-scope.
2. **TB-18 forward-binding §6 scope**: yes / no / amend (architect may add/remove TB-18 atoms).
3. **TB-17 SG-17.15 disposition**: ratified-with-deviation / require-substantive-build-in-TB-17 / re-charter-atom-8.
4. **OBS_R023 deferral cap reaffirmation**: confirm closure binding to TB-18 (Q4 verdict).

If ALL four are YES (with no amendments), TB-17 atom 8 ships as deviation-only; TB-18 charter scope is pre-bound.
If decision #3 = "require-substantive-build-in-TB-17", AI-coder re-engages atom 8 under autonomous Class 3 envelope; estimated time: multi-day; full hybrid dual audit required at end.

**Default in absence of ratification**: deviation-only ship (this doc filed; SG-17.15 status pending).

---

## §9 Cross-references

- TB-17 charter atom 8: `handover/tracer_bullets/TB-17_charter_2026-05-05.md` §3 atom 8.
- 2026-05-05 architect verdict §B.8 atom 8 + Q5: `handover/directives/2026-05-05_TB17_AUDIT_VERDICT_ARCHITECT_RULING.md`.
- TB-16.x.2.6 forensic findings: `handover/evidence/tb_16_x_2_6_smoke_2026-05-05/README.md` §Forensic-findings.
- TB-16 main charter Atom 5 (scaffold origin): `handover/tracer_bullets/TB-16_charter_2026-05-04.md`.
- OBS_R023 deferral cap: `handover/alignment/OBS_R022_TB_16_X_2_2_FIX_EVIDENCE_CAPSULE_HARDCODED_MAXTX_2026-05-05.md`.
- Memory: `feedback_no_workarounds_strict_constitution`, `feedback_audit_obs_bias`, `feedback_class4_cannot_hide_in_class3`, `feedback_minif2f_scaling_policy`.
- `experiments/minif2f_v4/src/bin/comprehensive_arena.rs` (scaffold; preserved).
- `experiments/minif2f_v4/src/bin/evaluator.rs` (target of TB-18 atom A re-entrant API refactor).
