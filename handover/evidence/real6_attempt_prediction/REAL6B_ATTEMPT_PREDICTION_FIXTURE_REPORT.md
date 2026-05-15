# REAL-6B AttemptPredictionMarket Fixture Report

Date: 2026-05-15

Harness run:

- `dev_1778822123466_1203906`

Directive source:

- `handover/directives/2026-05-15_REAL5S_REAL6_REAL7_REAL8_REAL9_ARCHITECT_ORIGINAL.md`
- `handover/directives/2026-05-15_REAL5S_REAL6_REAL7_REAL8_REAL9_EXECUTION_PLAN_APPROVED.md`

## Claim Boundary

This report claims only:

```text
REAL-6B sealed-oracle AttemptPredictionMarket has a deterministic scripted
fixture that proves K logical tape ticks, MarketClose-before-OracleResolve,
tape-visible role-window actions, no sleep, Lean oracle as absolute truth,
price-as-signal only, and no ghost liquidity.
```

This report does not claim:

```text
production SubmitCandidateTx / MarketCloseTx / OracleResolveTx wire schema,
sequencer admission changes,
live real-LLM ship,
spontaneous trading emergence,
or price-influenced verification.
```

## Implementation Summary

Files:

- `src/runtime/real6_attempt_prediction.rs`
- `src/runtime/mod.rs`
- `tests/constitution_real6_attempt_prediction_market.rs`
- `handover/alignment/DECISION_REAL6B_ATTEMPT_PREDICTION_SEALED_ORACLE.md`

The runtime helper exposes:

```text
AttemptPredictionFixture
AttemptPredictionStep
AttemptPredictionStepKind
LeanOracleResult
build_scripted_attempt_prediction_fixture
validate_attempt_prediction_fixture
```

The scripted fixture creates:

```text
SubmitCandidate at logical_t = open_t - 1
AttemptPredictionMarketOpen at logical_t = open_t
Trader / Verifier / Challenger window at open_t + 1 .. open_t + K
MarketClose at open_t + K + 1
OracleResolve at MarketClose + 1
```

Post-R1 remediation requires `open_t > 0`; otherwise the builder rejects the
fixture because SubmitCandidate must be strictly before AttemptPredictionMarketOpen.

All steps are deterministic from explicit inputs. No fixture field uses
wall-clock sleep. Role-window actions are marked `chain_tape_visible=true`.
`price_affects_verification=false`, and the OracleResolve step marks
`oracle_is_absolute_truth=true`.

## Red-Green Evidence

- `command_0001`: intentionally RED because `runtime::real6_attempt_prediction` did not exist.
- `command_0002`: partially green implementation, but failed because the test expectation incorrectly placed the first role-window tick at `open_t + 2`.
- `command_0003`: `cargo test --test constitution_real6_attempt_prediction_market` passed after correcting the gate expectation to `open_t + 1 .. open_t + K`.
- `command_0004`: Trust Root intentionally RED after adding `src/runtime/mod.rs` export; expected hash `3547694c...`, actual hash `2a6ade14...`.
- `command_0005`: `cargo fmt --all -- --check` RED after adding the new module.
- `command_0006`: `cargo fmt --all` passed.
- `command_0007`: `cargo fmt --all -- --check` passed.
- `command_0008`: `cargo test --test constitution_real6_attempt_prediction_market` passed after formatting.
- `command_0009`: Trust Root verify passed after rehashing `src/runtime/mod.rs`.
- `command_0010`: `bash scripts/run_constitution_gates.sh` passed, `436 passed / 0 failed / 1 ignored`.
- `command_0011`: `cargo test --workspace --no-fail-fast -- --test-threads=1` exited 0.
- `command_0014`: intentionally RED post-audit gate proving `opened_at_logical_t=0` could collapse SubmitCandidate and MarketOpen logical ticks before remediation.
- `command_0015`: targeted `sg_6b_submit_candidate_strictly_precedes_market_open` passed after rejecting `open_t=0` and adding validation that SubmitCandidate strictly precedes AttemptPredictionMarketOpen.
- `command_0016`: full `cargo test --test constitution_real6_attempt_prediction_market` passed after R1 remediation.
- `command_0017`: `cargo fmt --all -- --check` passed after R1 remediation.
- `command_0018`: Trust Root verify passed after R1 remediation and `genesis_payload.toml` comment update.
- `command_0019`: `bash scripts/run_constitution_gates.sh` passed, `436 passed / 0 failed / 1 ignored`.
- `command_0020`: `cargo test --workspace --no-fail-fast -- --test-threads=1` exited 0.

## Clean-Context Review R1

Review file:

- `handover/audits/CODEX_REAL6B_IMPLEMENTATION_REVIEW_R1.md`

Verdict:

```text
CHALLENGE
```

Findings accepted:

- P1: Branch-level `genesis_payload.toml` diff was broader than the report's narrow Trust Root claim.
- P2: `opened_at_logical_t=0` could collapse SubmitCandidate and AttemptPredictionMarketOpen onto the same logical tick.

Remediation:

- Added `sg_6b_submit_candidate_strictly_precedes_market_open`, which was RED at `command_0014` and GREEN at `command_0015`.
- Builder now rejects `opened_at_logical_t == 0`.
- Validator now requires SubmitCandidate logical_t strictly before AttemptPredictionMarketOpen logical_t.
- Added `REAL6B_TRUST_ROOT_SCOPE_NOTE.md`.
- Updated `genesis_payload.toml` `src/runtime/mod.rs` comment to name REAL-6B and the scripted-only scope.

## SG-6B Evidence Map

SG-6B.1 No sleep-based artificial blocking.

- Gate: `sg_6b_no_sleep_and_k_logical_ticks_are_deterministic`.
- Fixture validation rejects any step with `uses_wall_clock_sleep=true`.

SG-6B.2 K logical tape ticks are deterministic and replayable.

- Gate: `sg_6b_no_sleep_and_k_logical_ticks_are_deterministic`.
- Fixture builds equal values for equal inputs and asserts exact ticks `[11, 12, 13]` for `open_t=10`, `K=3`.

SG-6B.3 Lean oracle remains absolute truth.

- Gate: `sg_6b_oracle_is_absolute_and_price_not_truth`.
- Oracle step carries `oracle_is_absolute_truth=true`.

SG-6B.4 MarketCloseTx happens before OracleResolveTx.

- Gate: `sg_6b_market_close_precedes_oracle_resolve`.
- Fixture validates `MarketClose.logical_t < OracleResolve.logical_t`.

SG-6B.5 Trader actions during window are ChainTape-visible.

- Gate: `sg_6b_role_actions_during_window_are_tape_visible`.
- Scripted Trader, Verifier, and Challenger window steps are all `chain_tape_visible=true`.

SG-6B.6 Price does not affect verification.

- Gate: `sg_6b_oracle_is_absolute_and_price_not_truth`.
- Fixture-level `price_affects_verification=false`.

SG-6B.7 No ghost liquidity.

- Gate: `sg_6b_no_ghost_liquidity`.
- Fixture uses equal YES and NO seeded liquidity and disallows reserved market action amount above seeded liquidity.

## Forward Deferral

The architect explicitly limited current REAL-6B to:

```text
REAL-6B = design + scripted fixture only.
No live real-LLM ship until explicit Class-4 ratification.
```

Therefore production typed transaction schema for `SubmitCandidateTx`,
`MarketCloseTx`, and `OracleResolveTx` remains forward-deferred. That future
work must not be batched into this scripted fixture atom.

## Trust Root

`src/runtime/mod.rs` is Trust Root pinned. Adding the exported
`real6_attempt_prediction` helper required a narrow hash update in
`genesis_payload.toml`.

Final Trust Root evidence:

- `command_0009`: `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` passed.

Scope clarification:

- `handover/evidence/real6_attempt_prediction/REAL6B_TRUST_ROOT_SCOPE_NOTE.md`

The branch-level `genesis_payload.toml` diff includes prior REAL-6A / dirty
worktree Trust Root changes because this branch has not been split into
per-REAL commits. REAL-6B's semantic Trust Root change is the
`src/runtime/mod.rs` hash/comment update for exporting `real6_attempt_prediction`.
