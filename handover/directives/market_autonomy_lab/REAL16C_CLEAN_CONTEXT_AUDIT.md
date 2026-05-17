# REAL-16C Clean-Context Audit

REAL-17 main-CAS integration note: this document is preserved as pre-CAS-repair
historical context only. It is not forward claim-bearing on
`codex/real17-emergence-hardening-20260517`; REAL-17 must regenerate
ChainTape/CAS/verifier evidence on the updated CAS Git commit-chain baseline
before any forward claim can rely on it.

date_utc: 2026-05-17
reviewer: GPT-5.5 high clean context
question: Can the REAL-16C opportunity-trace-fix rerun be labeled exactly `E2 candidate pending audit`?
verdict: PROCEED

## Findings

The reviewer found no production defect blocking the narrow
`E2 candidate pending audit` label.

Evidence reviewed:

```text
handover/evidence/market_autonomy_lab_real16C_hard10_opportunity_trace_fix_20260517T093652Z/arm_C/REAL16_ARM_C_E2_VERIFIER.json
handover/evidence/market_autonomy_lab_real16C_hard10_opportunity_trace_fix_20260517T093652Z/arm_C/REAL16_ARM_C_E2_VERIFIER.md
handover/evidence/market_autonomy_lab_real16C_hard10_opportunity_trace_fix_20260517T093652Z/arm_C/aggregate_verdict.json
handover/evidence/market_autonomy_lab_real16C_hard10_opportunity_trace_fix_20260517T093652Z/REAL16_MARKET_PERFORMANCE_REPORT.json
```

Accepted narrow facts:

```text
audit_tape: PROCEED
exact_join_count: 17
l4_router_tx_count: 17
submitted_trace_tx_count: 17
duplicate_l4_router_tx_id_count: 0
duplicate_submitted_trace_tx_id_count: 0
scripted_fixture_tx_count: 0
policy_counts_for_e2: false
E2 verifier verdict: PROCEED
```

The reviewer confirmed that the top-level REAL-16 report is not an E4
candidate because this was a C-only rerun:

```text
failure_reasons: ["fewer_than_two_ab_arms"]
```

That checkpoint does not block the narrow Arm C E2-candidate label.

## Residual Risks

- PromptCapsule linkage is indirect via EVDecisionTrace rather than a direct
  `MarketDecisionTrace` field.
- This run is YES-side only: BullTrader / BuyYes. It must not be described as
  two-sided market evidence.
- The reviewer identified a scaffold gap: the runner recorded
  `TURINGOS_REAL11_MARKET_OPPORTUNITY_TRACE` in per-arm toggles but did not
  list it in `arm_diff_allowlist.txt`.

## Follow-Up

The scaffold gap was converted into a failing gate and fixed in:

```text
tests/constitution_real16_market_performance.rs
scripts/run_real16_market_performance_benchmark.sh
```

Verification after the fix:

```text
cargo fmt --all -- --check: PASS
cargo test --test constitution_real16_market_performance -- --test-threads=1: PASS
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1: PASS
```

## Claim Boundary

Allowed:

```text
E2 candidate pending audit
```

Not allowed:

```text
E2 achieved
E3 achieved
E4 achieved
market emergence proven
market mechanism shipped
```
