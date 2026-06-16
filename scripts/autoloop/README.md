# autoloop — audited autoresearch loop harness

Implements `handover/AUTORESEARCH_LOOP_DESIGN_2026-06-16.md`. Operator constraint:
**never spend on a buggy or invalid run.** So the driver is deterministic and free;
the only paid step it runs itself is the positive-control probe (cached on the harness
hash). It refuses to authorize execution until GATE-1 passes and the breakers are clear.

## Components

- `autoloop.py` — driver core (zero-LLM, bypass-proof). Subcommands:
  - `init --config <cfg>` — create the loop tape (records the V1 harness baseline hashes).
  - `preflight --state <tape>` — **GATE-1**: V1 harness-integrity, V2 positive control (cached on
    harness hash), V3 negative control, V4 evidence-contract, V5 budget-binding-declared, V6
    scope/reachability/no-restricted-surface. Exit 0 iff all pass.
  - `breakers --state <tape> --atom-hash <h>` — **driver circuit breakers**: iteration cap, token/$
    cap, cost-velocity, duplicate-input-hash (K=2), no-progress (K=3). Exit 0 iff clear.
  - `record --state <tape> --iter-json <f>` — append an iteration (execute result + GATE-2 verdict +
    decision); updates spend, seen-hashes, no-progress streak, loop status.
  - `status --state <tape>` — print status + the next allowed action.
- `gate2_audit.workflow.js` — **GATE-2**: reusable independent clean-context REFUTE-default audit of one
  iteration (L1 predicate recompute-from-tape ∥ L3 claim-refutation → adjudicated loop verdict).
  Invoke with `Workflow({scriptPath, args})`, args = `{repo, goal_predicate, iteration{...}}`.
- `estimate_calibration_cost.sh` — V5 budget-binding canary (one real run → full-sweep projection).
- `*.config.json` / `*_loop_state.json` — per-experiment config and the loop tape.

## The driver protocol (checkpoint mode — start here)

The outer driver (a human, or Claude as orchestrator) runs each iteration and pauses for a human
CONTINUE. One iteration:

```
1. python3 autoloop.py preflight --state TAPE          # GATE-1; if exit!=0 → FIX the harness, do not spend
2. python3 autoloop.py breakers  --state TAPE --atom-hash H   # if exit!=0 → STOP-DEAD (record the trip)
3. EXECUTE the atom               # run the carrier; produce a tape-canonical manifest. Spend metered.
4. Workflow(gate2_audit.workflow.js, args={iteration...})     # GATE-2 → structured verdict
5. write the iteration record (atom, atom_hash, execute summary, GATE-2 decision, advanced, spend)
   python3 autoloop.py record --state TAPE --iter-json REC
6. report to the human; on a verdict of CONTINUE, wait for confirmation, then loop.
   on ESCALATE-HUMAN → park and surface the exact human-only gate. on STOP-* → terminate.
```

Rules the driver MUST honor (they are why money is not wasted):
- never run step 3 (paid) unless steps 1 and 2 both exit 0;
- GATE-2 is independent — never let the agent that did step 3 also audit it;
- a human-only gate (Class-4 §8, architect sign-off, paid authorization, ambiguous VETO) is
  ESCALATE-HUMAN → **park, do not retry** (the loop cannot self-clear it).

## Graduation to auto (Cherny stage 3)

The audit node's auto-CONTINUE authority is *earned*: run GATE-2 in **shadow** (record its verdict
beside the human's call) until agreement ≥ 0.80 over a window with zero gate-passed/human-rejected
cases. Only then wrap steps 1–6 in an unattended `while` driver (cron / `/loop`). The breakers and
GATE-1 are identical in both modes; only the human-confirm in step 6 is removed. Keep the human rail —
SmartPause re-escalates when GATE-2 confidence drops below threshold.

## H-HET-2 instantiation (the first loop)

`h2_calibration.config.json` drives the H-HET-2 dynamic-model-budget experiment.
- Goal predicate = prereg §4 (Primary-A positive ∧ ≥1 Primary-B, replay+axiom-clean, §17 G1–G6).
- V1 harness = `lean_judge.rs` + `routing_policy.rs` + `budget_allocation_telemetry.rs` + `verify_chaintape.rs`.
- V2 positive control = the carrier on `calib_core_add_comm` (must reach omega).
- V3 negative control = `lean_judge_axiom_gate` test (must reject bad axioms).
- **Iteration 1 = the gate-#4 deep-theorem calibration** (per-(model,theorem) coverage over the
  ~37 non-det pool theorems at tx/agent≳20). It is **architect-gated paid prep** → the driver parks at
  ESCALATE-HUMAN until the architect authorizes it. The V5 canary (`estimate_calibration_cost.sh`) runs
  first to confirm the budget binds.
- Remaining audit-found refinements before the paid confirmatory: served_model provenance (#1),
  MODEL_RATES→CAS (#2) — see `H2_AGENT_ECONOMY_STATE.md`.
