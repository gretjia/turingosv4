# REAL-15 E3 Candidate Report

claim_boundary: `E3 candidate pending audit`

This report does not claim `E3 achieved`, `market emergence proven`, `market
mechanism shipped`, or `E4 achieved`.

## Summary

REAL-15 adds a candidate-only role-differentiation verifier over REAL-14G and
REAL-14H evidence.

The verifier derives role activity from:

```text
ChainTape/CAS role traces
independent REAL-14 exact-join verifier JSON
```

It does not use dashboard text as source of truth.

## Candidate Result

```text
verdict: PROCEED
e3_candidate: true
run_count: 2
audit_tape_proceed_count: 2
persistent_active_role_count: 2
distinct_action_signature_count: 2
failure_reasons: []
```

## Role Evidence

| role | active runs | turns | tasks | market actions | buy_yes | buy_no | other activity | signature |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |
| BullTrader | 2 | 39 | 20 | 19 | 19 | 0 | none | `work:0|verify:0|challenge:0|buy_yes:19|buy_no:0` |
| Solver | 2 | 37 | 19 | 0 | 0 | 0 | 34 CAS `SubmitProof` role outcomes | `work:34|verify:0|challenge:0|buy_yes:0|buy_no:0` |
| BearTrader | 1 | 39 | 20 | 2 | 0 | 2 | none | `work:0|verify:0|challenge:0|buy_yes:0|buy_no:2` |
| Verifier | 1 | 37 | 19 | 0 | 0 | 0 | 2 verify role outcomes | `work:0|verify:2|challenge:0|buy_yes:0|buy_no:0` |

The two persistent roles supporting the E3 candidate are BullTrader and Solver.
BearTrader and Verifier provide additional side/activity evidence, but they are
not treated as the two persistent roles because each is active in one of the two
input runs.

## Source Files

Implementation:

```text
src/runtime/role_differentiation.rs
src/bin/real15_role_differentiation_verifier.rs
tests/constitution_real15_role_differentiation.rs
```

Verifier outputs:

```text
handover/directives/market_autonomy_lab/REAL15_ROLE_DIFFERENTIATION_REAL14G_REAL14H.json
sha256=8e6088d697601716c7125fa25cf18921b47e2e3f6e1fe128b015b4fcbbe33e9c

handover/directives/market_autonomy_lab/REAL15_ROLE_DIFFERENTIATION_REAL14G_REAL14H.md
sha256=577221728730cafe3b637ec3a43d1cb5209dda11e5bdd8caf71eb24afa074ae2
```

Input exact-join verifier outputs:

```text
REAL-14G:
handover/evidence/market_autonomy_lab_hard10_real14G_action_conversion_20260517T022457Z/REAL14G_E2_CANDIDATE_VERIFIER_EXPECT8.json
sha256=08425d9b11a5684e16ca8d3c8a6cf22a08cbc954a36e8a18ebdd7e2906193331

REAL-14H:
handover/evidence/market_autonomy_lab_hard10_real14H_frozen_real14G_replication_20260517T025400Z/REAL14H_E2_CANDIDATE_VERIFIER_EXPECT13.json
sha256=111034f23367a16abf7956e55b44df957dd8002f7e14e0c6a5d29a02493d86fb
```

## Verification

```text
cargo fmt --all -- --check
PASS

cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
PASS after allowed research-envelope rehash of src/runtime/mod.rs

cargo test --test constitution_real15_role_differentiation -- --test-threads=1
6 passed / 0 failed

cargo test --test constitution_real14_e2_candidate_verifier -- --test-threads=1
8 passed / 0 failed

TURINGOS_RESEARCH_ENVELOPE=MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2 bash scripts/run_market_autonomy_research_preflight.sh
Level2 allowed Trust Root rehash checkpoint; Trust Root rerun passed
```

The full constitution gate runner hit linker/resource failures on two G3 tests:

```text
constitution_g3_pnl_trajectory_evidence_binding: linker Bus error
constitution_g3_your_position_prompt: linker Bus error
```

Both tests passed when rerun sequentially with constrained build jobs:

```text
CARGO_BUILD_JOBS=1 cargo test --test constitution_g3_pnl_trajectory_evidence_binding -- --test-threads=1
6 passed / 0 failed

CARGO_BUILD_JOBS=1 cargo test --test constitution_g3_your_position_prompt -- --test-threads=1
8 passed / 0 failed
```

## Residual Risks

1. Solver `work_count` is CAS `RoleTurnOutcome::SubmitProof` role activity,
   not an accepted WorkTx production claim.
2. BearTrader and Verifier are active in only one of the two runs, so they
   support side/activity context but not the two persistent-role threshold.
3. Upstream REAL-14 exact-join reports retain provenance residual warnings on
   some matched rows; REAL-15 surfaces these instead of hiding them.
4. E4 performance evidence is not established.

## Decision

REAL-15 may be labeled:

```text
E3 candidate pending audit
```

The current ladder state is:

| ladder item | status |
| --- | --- |
| E2 candidate | present |
| E2 replicated candidate | present |
| Two-sided market candidate | present |
| E3 candidate pending audit | present |
| E4 candidate pending audit | not established |
| market emergence candidate pending final audit | not established |

