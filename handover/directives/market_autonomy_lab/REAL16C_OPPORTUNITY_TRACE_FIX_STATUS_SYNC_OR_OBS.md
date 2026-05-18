# REAL-16C Opportunity Trace Fix Status Sync / OBS

REAL-17 main-CAS integration note: this document is preserved as pre-CAS-repair
historical context only. It is not forward claim-bearing on
`codex/real17-emergence-hardening-20260517`; REAL-17 must regenerate
ChainTape/CAS/verifier evidence on the updated CAS Git commit-chain baseline
before any forward claim can rely on it.

date_utc: 2026-05-17
mode: Constitutional Research Mode
envelope: `MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2`

## Status

REAL-16C reran the C arm only after enabling
`TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE=1` in the REAL-16 benchmark runner.

Evidence dir:

```text
handover/evidence/market_autonomy_lab_real16C_hard10_opportunity_trace_fix_20260517T093652Z
```

Allowed current label for this run:

```text
E2 candidate pending audit
```

Still not established by this run:

```text
E4 candidate pending audit
Two-sided market candidate for this run
market emergence candidate pending final audit
```

Still forbidden:

```text
E2 achieved
E3 achieved
E4 achieved
market emergence proven
market mechanism shipped
```

## Evidence

Arm C audit:

```text
audit_tape: PROCEED
assertions: 41 passed / 0 failed
```

Arm C exact-join verifier:

```text
verdict: PROCEED
exact_join_count: 17
l4_router_tx_count: 17
submitted_trace_tx_count: 17
duplicate_l4_router_tx_id_count: 0
duplicate_submitted_trace_tx_id_count: 0
scripted_fixture_tx_count: 0
policy_counts_for_e2: false
failure_reasons: []
```

Matched provenance summary:

```text
live role: BullTrader
direction: BuyYes
MarketOpportunityTrace count per matched tx: >= 1
EVDecisionTrace count per matched tx: >= 1
RoleTurnTrace count per matched tx: >= 1
PromptCapsule linkage: indirect via EVDecisionTrace
BCAST shielding: PASS
```

The run is YES-side only. It must not be described as two-sided market evidence.

## E4 Boundary

The top-level REAL-16 performance report is intentionally not an E4 candidate:

```text
claim_boundary: clean-negative; no E4 candidate
verdict: Veto
failure_reasons: ["fewer_than_two_ab_arms"]
```

This is expected for a C-only provenance-fix rerun. It does not VETO the narrow
Arm C E2-candidate label because the Arm C ChainTape/CAS verifier is PROCEED.

## Residual Risks

- PromptCapsule linkage is indirect via matched EVDecisionTrace, not a direct
  `MarketDecisionTrace` field.
- Multiple EVDecisionTrace rows can match an agent/event/action; the router tx
  is disambiguated by exact L4/CAS tx-id join.
- This run is BullTrader/BuyYes only.
- This run is not a pinned A/B performance benchmark.

## Next

1. Complete clean-context audit for the narrow question:

```text
Can REAL-16C be labeled E2 candidate pending audit?
```

2. If clean-context audit is PROCEED, rerun full REAL-16 A/B with the
   MarketOpportunityTrace fix enabled.

3. If full A/B produces candidate evidence, evaluate only:

```text
E4 candidate pending audit
```

Do not claim E4 achieved or market emergence proven.
