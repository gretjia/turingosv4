# OBS — Coverage Denominator (post-TB-7R hardening) (2026-05-02)

**Class**: Observation (post-TB-7R audit risk)
**Driver**: Architect verdict 2026-05-02 (`2026-05-02_TB7R_PARENT_TX_DAG_SMOKE_VERDICT.md` §6 "Next insight")
**Status**: ACTIVE — post-TB-7R follow-up; NOT a TB-7R blocker.

---

## §1 The risk

Architect 2026-05-02:

> "ChainTape can only prove what reached it. The next hardening step is
> to ensure every LLM response that becomes an externalized proposal is
> counted before submission and must either land in L4/L4.E or fail-closed.
> Without this, a hidden legacy path can still produce unchained proposals."

The strict three-node taxonomy used in TB-7R defines an "externalized
proposal" as `bus.submit_typed_tx`. Under that strict definition,
TB-7R smoke passes "every externalized proposal in L4 or L4.E"
trivially — because the only path to externalization IS submit_typed_tx,
and submit_typed_tx routes through Sequencer to L4 or L4.E.

**The risk is in the IMPLICIT step**: how does an LLM output become an
externalized proposal? If a code path consumes an LLM response, processes
it, and bypasses submit_typed_tx (e.g. into `bus.append` shadow_only,
into `bus.record_rejection` counter, or into a future legacy path), then
that LLM output is "consumed but not chained." The denominator of
"all LLM proposals" is unprotected.

## §2 Concrete current-state inventory

In `experiments/minif2f_v4/src/bin/evaluator.rs`, the `step` tool's
PartialVerdict dispatch:

```text
"step" => match oracle.verify_partial(prefix) {
    PartialVerdict::Complete   → bus.submit_typed_tx → L4 (or L4.E)  [chained]
    PartialVerdict::PartialOk  → bus.append_oracle_accepted          [shadow_only / not chained]
    PartialVerdict::Reject     → bus.record_rejection (counter)      [in-memory / not chained]
}
```

For the `mathd_*` smoke problems where the LLM emits a one-shot `complete`
action that Lean accepts, only the Complete branch fires — and that's
chained. For harder problems where the LLM emits intermediate `step`
actions:
- PartialOk → goes to kernel.tape (shadow_only) but NOT chain
- Reject → goes to in-memory counter, AND the raw `reason` flows back
  into the next prompt via `acc.record_tool_stdout(&reason)` (see
  `OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md`)

## §3 Why this is post-TB-7R

Architect verdict 2026-05-02 explicitly **frames this as the next
hardening step, not a TB-7R blocker**. Under the strict three-node
interpretation TB-7R adopts, the current state is internally consistent.

The TB-7R smoke shows the natural consequence: aime_1997_p9 ran 20 step
actions (18 reject + 2 partial-OK), but **0 of those reached chain**
because the chain-routing path is gated on `Complete` outcome only.
This is documented in `handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/stdout`
(see `tool_dist`).

## §4 Recommended hardening (post-TB-7R)

A future TB (TB-7.5? TB-8?) should:

1. Re-route `PartialVerdict::PartialOk` through `submit_typed_tx` with
   `predicate_passes=true` and a distinct acceptance class
   (`lean_partial`), landing intermediate progress in L4 with a
   non-OMEGA-terminating semantics. This puts every `step` action's
   verified-progress claim on chain.
2. Re-route `PartialVerdict::Reject` through `submit_typed_tx` with
   `predicate_passes=false`, landing in L4.E with
   `rejection_class = LeanFailed` and `raw_diagnostic_cid` shielded.
3. Verify the strict invariant: every LLM tool-call action that runs
   Lean (or any oracle) must produce exactly one chain entry — either
   L4 accepted or L4.E rejected — never an unchained tool_dist counter
   bump.

The Sequencer's existing `apply_one` + `predicate_results` machinery
already supports this; the change is at the evaluator dispatch site.

## §5 Conformance criterion (post-implementation)

```text
For every run:
  externalized_proposal_count ==
    L4 Work entries + L4.E Work entries
  (no LLM oracle action lands only in tool_dist or only in kernel.tape)
```

This is stronger than TB-7R's strict three-node interpretation
("every submit_typed_tx call lands in L4 or L4.E") because it closes
the implicit step from "LLM output" to "submit_typed_tx call".

## §6 Cross-references

- Verdict: `handover/directives/2026-05-02_TB7R_PARENT_TX_DAG_SMOKE_VERDICT.md` §6
- Companion OBS: `handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md`
- Smoke evidence (aime run with 20 step actions, 0 on chain):
  `handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/stdout`
- Three-node taxonomy: `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md`

## §7 Closure path

This OBS closes when a future TB ships the §4 hardening AND a smoke
demonstrates `externalized_proposal_count == chain_proposal_count`
across runs that exercise PartialOk + Reject paths.
