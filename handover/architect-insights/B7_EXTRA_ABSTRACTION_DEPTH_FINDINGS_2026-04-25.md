# B7-extra Abstraction-Depth Findings — 2026-04-25

**Status**: filed for Phase Z merge planning + Phase D ArchitectAI design constraints. Not blocking Phase B → C transition.
**Trigger**: thesis re-alignment audit (user, 2026-04-25 mid-session) found two abstraction-depth gaps where the implementation works correctly *within its scope* but is below the constitutionally cleanest design depth.
**Constitutional anchors**: thesis claims 7 ("white-box predicates settle state transitions") + 11 ("legible/enforceable/reversible state").

---

## Finding (A) — Synthetic predicate at evaluator-level, not bus-level

### What is

`SIMULATE_ROLLBACK_AT_TX_50=1` triggers a short-circuit at evaluator's `run_swarm` loop (`experiments/minif2f_v4/src/bin/evaluator.rs:503-518`) via `rollback_sim::should_simulate_rollback(tx, enabled)`. The kernel/bus is not informed; from `TuringBus`'s perspective tx 50 is just a tx that never happened.

### What it should be

Constitutionally cleanest = a `Predicate` impl `AlwaysRejectAfterTxN(50)` registered via `bus.register_predicate(...)` at swarm boot, returning `Reject` for all tx ≥ 50. Then the bus's existing `evaluate_predicates(ctx, payload)` routes through `Result::Vetoed` for every subsequent append, naturally exhausts at `max_transactions`, and emits `HaltReason::MaxTxExhausted` from the bus's own machinery.

This implementation properly exercises the **FC1-N11 ∏p product gate** (TRACE_MATRIX FC1-N11). The current evaluator-level short-circuit BYPASSES the ∏p gate for these synthetic ticks — the bus has no record of "we tried to append at tx 50-199 and were vetoed", because we never asked the bus.

### Why it's not done that way

`Predicate` trait + `bus.register_predicate` API live on the unmerged `phase-z-wtool-tools` branch (commit `74b2ce7`). TRACE_MATRIX_v0 row FC1-N11 references them aspirationally but they never landed on `main`. Reviving Phase Z to enable a one-line predicate registration in B7-extra would be ~10x scope-creep on a measurement task.

### Where the gap matters

| Layer | Effect |
|---|---|
| Production runs (control, no toggle) | **No effect** — production goes through the existing inline forbidden_patterns + Lean4Oracle white-box predicate surface (`src/bus.rs:183`) |
| Calibration treatment runs | The bus's tape never records 150 vetoed tx attempts; only the evaluator's tx_count==50 stamp + synthetic_short_circuit=true field signal what happened |
| Phase D ArchitectAI design | When ArchitectAI generates user-space artifacts, the *real* predicate path will be exercised by `bus.append → ∏p → wtool` for every artifact-generated proposal. Phase D MUST land before any production system can enforce capability-compilation invariants at FC1-N11 depth — Phase D's predicate surface IS the full Phase Z ambition restored |
| Phase B → C audit | Auditor should be told: "B7-extra synthetic veto is at evaluator layer; the bus's ∏p path is not exercised by calibration treatment. The control path (production) IS exercised, so the cost/time/progress measurements are valid for production-equivalent runs." |

### Action items

1. **Phase Z merge** (when scheduled): port `phase-z-wtool-tools` Predicate trait + `bus.register_predicate` + 3 default impls (`ForbiddenPatternPredicate`, `SorryPredicate`, `PayloadSizePredicate`) onto main.
2. **B7-extra refactor** (post-Phase Z merge): replace evaluator short-circuit with `bus.register_predicate(AlwaysRejectAfterTxN(ROLLBACK_TX_THRESHOLD))` at run_swarm boot when toggle is on. Drop the short-circuit. The synthetic_short_circuit flag becomes an artifact of how p_0 was historically calibrated; do not re-calibrate (Trust Root has the frozen value).
3. **Phase D design** (PREREG § 6 D2): document that ArchitectAI's user-space artifacts traverse the real ∏p path, so the abstraction depth gap closes naturally.

---

## Finding (B) — Cost asymmetry in synthetic-short-circuit treatment runs

### What is

When `synthetic_short_circuit=true` is stamped on a calibration treatment run's jsonl row, the `total_run_token_count` (C_i) reflects only the cost of tx 0-49 (the actual LLM calls that happened). A "true" 150-tx vetoed loop would have cost ~3x more (LLM calls every tx, all rejected).

### Why it's not perfect

PREREG § 5.5 conceives of `--simulate-rollback-at-tx-50` as "synthetic mid-run rollback mimicking worst-case stochastic edge case Phase E artifacts could trigger". A worst-case Phase E artifact would corrupt state but the loop would continue spending budget. We short-circuit instead — saving wall-clock but understating cost.

### Where the gap matters

| Analysis | Effect |
|---|---|
| `compute_p0.py` (PREREG § 5.5 estimator) | **None** — only joins on SOLVED/UNSOLVED, ignores cost. p_0 is unaffected. |
| Per-row PPUT computation on calibration jsonl | Affected — `pput_runtime` and `pput_verified` for synthetic_short_circuit rows are 0/x = 0 (correct, since solved=false), but if anyone interpreted the cost C_i as "what a real Phase E artifact would have cost", that would be wrong |
| Aggregate cost statistics if these rows included | Significantly underreports |

### Mitigation in place

Field doc-comment on `PputResult::synthetic_short_circuit` warns:

> Crucially: when `synthetic_short_circuit == Some(true)`, the run's `total_run_token_count` (C_i) is **understated** vs a true 150-tx vetoed loop, because the LLM calls for tx 51-199 never happened. `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so p_0 estimation is unaffected; downstream PPUT analysis on these rows MUST honor this flag and exclude or specially treat them.

### Action items

1. **Any future PPUT aggregation tool**: filter out `synthetic_short_circuit=true` rows or document explicitly that they are calibration-treatment-only.
2. **Phase B → C audit packet**: include this finding so external auditors know what they're looking at.
3. **Future calibration redesigns** (if needed): consider whether to (a) accept this asymmetry and remove the short-circuit (run all 200 tx, ~3x calibration cost ≈ +$10 over current $3-5), or (b) keep short-circuit and document.

---

## Why these findings are not thesis-level drift

Thesis claim 7 (white-box predicates settle state) is satisfied for the **production path**: every real proposal flows through `bus.append → forbidden_patterns + Lean4Oracle` (white-box, deterministic, inspectable). The synthetic predicate's measurement-only path bypasses this for the specific purpose of estimating p_0 (a measurement quantity). The thesis applies to the system's claim about how it handles real proposals, not to how it does internal calibration measurements on itself.

Thesis claim 11 (legible/enforceable/reversible) is satisfied: the synthetic_short_circuit flag makes the calibration disambiguator legible; the cost-asymmetry doc-comment makes it enforceable (any tool ignoring the flag is observably wrong); the calibration is reversible because it produces a frozen p_0 in genesis_payload.toml that can be re-measured if the toggle definition changes.

Both findings are **abstraction-depth** issues, recorded for Phase Z merge planning and Phase D design constraints, not blockers for Phase B → C transition.

---

## Cross-references

- `experiments/minif2f_v4/src/rollback_sim.rs` — module + 6 unit tests
- `experiments/minif2f_v4/src/bin/evaluator.rs:503-518` — short-circuit site
- `experiments/minif2f_v4/src/bin/evaluator.rs` `synthetic_short_circuit` field — disambiguator with cost-asymmetry warning
- `handover/alignment/TRACE_MATRIX_v1_2026-04-25.md` § 7.2 — implementation note
- `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` § 5.5 — calibration protocol spec
- Unmerged branch `phase-z-wtool-tools` commit `74b2ce7` — full Phase Z Predicate trait + bus.register_predicate API
