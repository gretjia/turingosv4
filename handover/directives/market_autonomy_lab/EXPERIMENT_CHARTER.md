# Market Autonomy Lab Experiment Charter

## Objective

Achieve real voluntary agent market-mechanism emergence under the TuringOS
constitution, continuing through clean-negative cycles until either:

- an `E2 candidate pending audit` appears, or
- a constitutional/resource hard stop blocks further autonomous work.

Clean-negative evidence is progress, not completion.

## FC Nodes And Risk

Touched FC nodes/invariants:

```text
FC1-N5 / FC1-N6: role-scoped read view and prompt input.
FC1-N8 / FC1-N10: externalized agent market action.
FC2-INV1: Trust Root / replay preflight.
FC3-N31 / FC3-INV3: logs/archive shielding and dashboard/digest materialized views.
```

Risk floor:

```text
Class 4 for any Trust-Root-pinned runtime/evaluator/dashboard/genesis_payload edit.
Class 3 for real evidence, CAS integrity, market/economic state, and audit_tape.
Class 0/1 only for source archive, planning, and non-authoritative docs.
```

## Tracks

```text
Constitutional Research Track:
  Allowed to produce candidate evidence.
  No forced trade.
  No scripted/PolicyTrader action counted as E2.

Unsafe Red-Track Sandbox:
  Requires TURINGOS_UNSAFE_RESEARCH=1 and separate evidence root.
  NOT SHIP EVIDENCE.
  NOT E2.
  NOT AUTONOMY CLAIM.
```

## Operating Loop

1. Preserve architect source verbatim.
2. Open/maintain independent worktree and branch.
3. Classify FC nodes and risk.
4. Write failing gates first.
5. Patch only constitution-preserving mechanisms after required ratification.
6. Run true MiniF2F/Lean evidence with sufficient difficulty.
7. Audit evidence.
8. If E2 is absent, write clean-negative mechanism report and continue to the
   next bottleneck.

## Hard Stops

- Trust Root failure before evidence run.
- Constitution gate failure not understood.
- Need to touch restricted surfaces without explicit per-atom Class-4
  ratification.
- Any mechanism requiring forced trade, price-as-truth, ghost liquidity,
  off-tape truth, f64/f32 money, raw CoT/log broadcast, or scripted action
  counted as E2.
- Resource/budget expansion beyond the authorized experiment envelope.

