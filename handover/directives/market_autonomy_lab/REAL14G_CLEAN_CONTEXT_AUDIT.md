# REAL-14G Clean-Context Audit

REAL-17 main-CAS integration note: this document is preserved as pre-CAS-repair
historical context only. It is not forward claim-bearing on
`codex/real17-emergence-hardening-20260517`; REAL-17 must regenerate
ChainTape/CAS/verifier evidence on the updated CAS Git commit-chain baseline
before any forward claim can rely on it.

audit_question: `Can REAL-14G be labeled E2 candidate pending audit?`
reviewer: clean-context GPT-5.5 high
verdict: `PROCEED`

## Findings

No blocking production defect was found. The reviewer approved only the narrow
label:

```text
E2 candidate pending audit
```

The audit explicitly does not authorize `E2 achieved`, E3/E4, market emergence
proof, or ship evidence.

## Evidence Cited By Reviewer

The reviewer checked that the independent verifier:

- recomputes exact join from L4 `BuyWithCoinRouter` tx ids and CAS submitted
  `MarketDecisionTrace` router tx ids,
- does not parse dashboard text as truth,
- fails closed on duplicate tx ids, scripted fixture contamination,
  PolicyTrader-for-E2 contamination, provenance mismatch, actor mismatch,
  amount/direction mismatch, and BCAST shielding failures.

Acceptance facts:

```text
l4_router_tx_count: 8
submitted_trace_tx_count: 8
exact_join_count: 8
duplicate_l4_router_tx_id_count: 0
duplicate_submitted_trace_tx_id_count: 0
scripted_fixture_tx_count: 0
policy_counts_for_e2: false
verdict: PROCEED
failure_reasons: []
```

The reviewer also verified:

```text
YES-side only:
  router_buy_yes: 8
  router_buy_no: 0

No price-as-truth:
  price_index_is_view_only: Pass

BCAST shielding:
  verdict: PASS
  failure_reasons: []
```

## Residual Risk

The reviewer recorded three non-blocking residual risks:

1. Prompt provenance is indirect. `MarketDecisionTrace` does not directly carry
   a PromptCapsule field; the verifier links through matched `EVDecisionTrace`.
2. The first two matched tx ids each have two matching EVDecisionTrace rows;
   exact router tx_id join disambiguates the candidate count.
3. Dashboard G7 guard rows include false values in an N/A context where
   `g7_guard_cas_count: 0`; this is reporting ambiguity, not an E2-candidate
   blocker.

## Fresh Commands Reported By Reviewer

```bash
cargo test --test constitution_real14_e2_candidate_verifier -- --test-threads=1
target/debug/real14_e2_candidate_verifier ... --expect-count 8
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
```

Reviewer-reported outcomes:

```text
constitution_real14_e2_candidate_verifier: 8 passed / 0 failed
real14_e2_candidate_verifier --expect-count 8: exit 0, PROCEED
Trust Root unit test: 1 passed / 0 failed
```

## Verdict

```text
PROCEED
```

Claim boundary remains:

```text
E2 candidate pending audit
```
