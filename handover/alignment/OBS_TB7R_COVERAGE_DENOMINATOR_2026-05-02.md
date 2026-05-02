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
- Reject → goes to in-memory counter (token-cost only; see §2.1 update)

### §2.1 Codex round-1 ship-audit refinement (2026-05-02)

Codex round-1 ship audit Q2/Q10 surfaced a sharper framing of the gap and
corrected one premise:

**(a) PartialOk → Complete proof-prefix dependency** (Codex Q2 CHALLENGE,
new constitutional debt #2): the issue is not just "PartialOk is unchained
LLM activity," but specifically that a later `Complete` action verifies
`tape_chain + tactic` (`evaluator.rs:2132`) while storing **only `tactic`**
as the WorkTx artifact (`evaluator.rs:2222`). This means an accepted L4
WorkTx's `proof_artifact_cid` resolves to a `tactic` payload that, on its
own, is not Lean-verifiable — the reconstructable proof depends on
prior unchained PartialOk steps in `kernel.tape`.

Replay from L4 + CAS alone CAN reconstruct the QState + EconomicState +
predicate_results (`replay_report.json` shows this on every TB-7R smoke
run). What replay CANNOT reconstruct from L4 + CAS alone is the *Lean
proof object* that justified the predicate-pass — that proof object is
the tape_chain prefix that lived in unchained kernel.tape during the run.

Under TB-7R's strict three-node taxonomy + verdict A1=B′ (proposal-level
DAG, per-tactic deferred), this is *internally consistent* because
"externalized" is defined as `submit_typed_tx`-routed and the WorkTx
*acceptance signal* is on chain even if the proof *artifact* is partial.
But under the broader Tape Canonical Axiom (Art. 0.2) + acceptance
clause 2 ("predicate evidence resolves from CAS"), the proof artifact
should be self-contained.

**(b) Reject path correction** (Codex Q10 PASS for prompt isolation but
clarifying premise): `PartialVerdict::Reject` records a bounded rejection
class label via `bus.record_rejection(agent_id, class.label())` plus
`acc.record_tool_stdout(&reason)` for token-cost accounting only
(`cost_aggregator.rs:57`). The raw `reason` does NOT flow into the next
prompt — `prompt_builder` reads bounded class labels from
`evaluator.rs:1344` and `bus.rs:576`, NOT raw Lean stderr. This means
the original OBS-2 (`OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md`)
had a stale premise: there is no Art. III.4 prompt-pollution risk; only
the Reject side of the coverage-denominator question (raw oracle output
not reaching chain even as L4.E rejection evidence) remains.

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
3. **Self-contained proof artifact**: when a `Complete` action follows
   prior PartialOk steps, the accepted WorkTx's `proof_artifact_cid`
   must resolve to the *full reconstructable proof* (tape_chain + tactic),
   not just the final tactic. Either store the full prefix on chain via
   the §4.1 reroute, or store the concatenated `tape_chain + tactic`
   blob in CAS as the WorkTx artifact and update verify_partial to
   verify against that committed blob alone. (Codex round-1 §2.1.a)
4. Verify the strict invariant: every LLM tool-call action that runs
   Lean (or any oracle) must produce exactly one chain entry — either
   L4 accepted or L4.E rejected — never an unchained tool_dist counter
   bump. AND every accepted L4 WorkTx's proof artifact must be
   Lean-verifiable from CAS alone (no kernel.tape dependency).

The Sequencer's existing `apply_one` + `predicate_results` machinery
already supports the routing change in §4.1+§4.2; the proof-self-containment
change in §4.3 is at the evaluator dispatch site (or at the Lean proof
serialization site within the `complete` tool).

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
