# REAL-15 Decision Packet

decision: `PROCEED`
label: `E3 candidate pending audit`

This packet does not claim `E3 achieved`, `E4 achieved`, `market emergence
proven`, or `market mechanism shipped`.

## Summary

REAL-15 builds and runs an executable role-differentiation verifier over the
REAL-14G and REAL-14H true-problem evidence.

The verifier reports:

```text
e3_candidate: true
verdict: PROCEED
run_count: 2
audit_tape_proceed_count: 2
persistent_active_role_count: 2
distinct_action_signature_count: 2
```

The two persistent distinct roles are:

```text
BullTrader: exact-join market action distribution, 19 BuyYes / 0 BuyNo
Solver: CAS SubmitProof role activity distribution, 34 SubmitProof outcomes
```

BearTrader adds two BuyNo actions in REAL-14H, supporting the existing
two-sided-market candidate, but is not counted as one of the two persistent
roles for E3 because it is active in only one run.

## Evidence

| item | value |
| --- | --- |
| role verifier JSON | `handover/directives/market_autonomy_lab/REAL15_ROLE_DIFFERENTIATION_REAL14G_REAL14H.json` |
| role verifier JSON sha256 | `8e6088d697601716c7125fa25cf18921b47e2e3f6e1fe128b015b4fcbbe33e9c` |
| role verifier Markdown | `handover/directives/market_autonomy_lab/REAL15_ROLE_DIFFERENTIATION_REAL14G_REAL14H.md` |
| role verifier Markdown sha256 | `577221728730cafe3b637ec3a43d1cb5209dda11e5bdd8caf71eb24afa074ae2` |
| REAL-14G verifier JSON sha256 | `08425d9b11a5684e16ca8d3c8a6cf22a08cbc954a36e8a18ebdd7e2906193331` |
| REAL-14H verifier JSON sha256 | `111034f23367a16abf7956e55b44df957dd8002f7e14e0c6a5d29a02493d86fb` |
| clean-context audit | `PROCEED` |

## Code And Gates

Code:

```text
src/runtime/role_differentiation.rs
src/bin/real15_role_differentiation_verifier.rs
tests/constitution_real15_role_differentiation.rs
src/runtime/mod.rs
genesis_payload.toml
```

Gates:

```text
cargo fmt --all -- --check
PASS

cargo test --test constitution_real15_role_differentiation -- --test-threads=1
6 passed / 0 failed

cargo test --test constitution_real14_e2_candidate_verifier -- --test-threads=1
8 passed / 0 failed

cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
PASS
```

The broad constitution runner was not treated as a VETO because its two RED
entries were linker/resource failures, not assertion failures. Both affected
tests passed when rerun sequentially with `CARGO_BUILD_JOBS=1`.

## Claim Boundary

Allowed:

```text
E3 candidate pending audit
```

Still forbidden:

```text
E3 achieved
E4 achieved
market emergence proven
market mechanism shipped
```

## Current Ladder State

| ladder item | status |
| --- | --- |
| E2 candidate | present |
| E2 replicated candidate | present |
| Two-sided market candidate | present |
| E3 candidate pending audit | present |
| E4 candidate pending audit | not established |
| market emergence candidate pending final audit | not established |

## Next Recommendation

Open:

```text
REAL-16 -- Market Performance / E4 Candidate Benchmark
```

The next question is whether the market mechanism improves system behavior
under pinned A/B evidence. REAL-16 must compare:

```text
A baseline market-visible
B EV scaffold
C EV + BCAST
D EV + BCAST + PnL + role-specialized action-conversion view
```

No E4 wording is allowed unless pinned A/B evidence and clean-context audit
support only:

```text
E4 candidate pending audit
```

