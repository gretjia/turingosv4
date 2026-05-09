# Codex Stage C P-M2 Audit R1

HEAD: `66f4e34103bc577d1f90fb4f1956518cf41c1023`
Date: 2026-05-09
Round: R1
Final aggregate verdict: CHALLENGE

Fresh verification run:
- `cargo test --test constitution_completeset_merge` -> 5 passed / 0 failed
- `cargo test --test constitution_architect_verbatim_struct_binding` -> 5 passed / 0 failed
- `cargo test --test constitution_economy_strict_equality` -> 8 passed / 0 failed
- `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` -> 1 passed / 0 failed

Q1: PASS - `CompleteSetMergeTx` in `src/state/typed_tx.rs` is exactly the architect §7.3 six-field wire shape `{ tx_id, parent_state_root, event_id, owner, amount, signature }`; there is no `timestamp_logical` field in that struct. `CompleteSetMergeSigningPayload` is the five-field projection with `signature` excluded. `tests/constitution_architect_verbatim_struct_binding.rs` contains both P-M2 bindings, wire and signing payload, with `LandingStatus::Landed`, and the binding gate passed fresh.

Q2: CHALLENGE - The five tests pass and each P-M2 merge assertion goes through `submit_and_apply` against the live `Sequencer`; the two negative tests do return an apply-time `TransitionError::InsufficientSharesForMerge` string, so they are not vacuous pre-submit assertions. However, `merge_requires_both_sides` and `merge_unavailable_after_final_redeem_if_shares_exhausted` clone the post-mint `QState`, manually flip `task_markets_t` to `Bankrupt`/`Finalized`, and reseat a fresh harness before redeeming. That is fixture setup for the resolution state, not a fully live tape-derived path, and it falls short of the stated "not fixture-forge" test-body standard.

Q3: CHALLENGE - The positive CompleteSetMerge accept arm maps all six architect §7.3 semantic lines: it requires owner YES and NO balances, burns both sides, debits `conditional_collateral_t[event]`, and credits `balances_t[owner]` one-for-one. It also adds an explicit `amount.units == 0` rejection returning `InsufficientSharesForMerge`; zero is not excluded by the architect §7.3 semantics block, where the two balance preconditions would admit a no-op. Parent-root, collateral-coverage, invariant, and state-root checks are normal sequencer defenses, but the zero-amount rejection is an extra policy clause that needs either removal or architect ratification.

Q4: PASS - The implementation correctly omits an event-state gate for merge. Architect §7.3 differs from mint by specifying only share-balance preconditions, and post-resolution merge of a matched YES+NO pair is economically equivalent to consuming residual collateral for the still-backed winning side while burning the losing side. The `merge_unavailable_after_final_redeem_if_shares_exhausted` test relies on share exhaustion rather than state rejection, which is faithful to the spec intent.

Q5: PASS - For reachable positive-amount states, merge preserves the six-holding CTF total because the collateral debit exactly equals the owner balance credit, while YES/NO shares are claims and not counted as Coin. The arm calls `assert_total_ctf_conserved` and `assert_complete_set_balanced` after mutation, rejects insufficient collateral before subtraction, and `assert_no_post_init_mint` now allow-lists `TypedTx::CompleteSetMerge`. Zero amount is pre-rejected, so it does not create a conservation break; Q3 covers that as a verbatim-alignment challenge rather than a monetary invariant failure.

Q6: PASS - F-DEFERRAL-2 is closed for P-M2: the BINDINGS array includes both `CompleteSetMergeTx` and `CompleteSetMergeSigningPayload`, both Landed, with strict `(field_name, type_first_token)` equality. `cargo test --test constitution_architect_verbatim_struct_binding` passed fresh with 5/5 tests.

Q7: PASS - `complete_set_merge_accept_state_root` is deterministic: SHA-256 over `COMPLETE_SET_MERGE_DOMAIN_V1`, previous root, and canonical-encoded `TypedTx`. `TxKind::CompleteSetMerge = 14` is present. The six current file hashes for `typed_tx.rs`, `sequencer.rs`, `transition_ledger.rs`, `monetary_invariant.rs`, `verify.rs`, and `run_summary.rs` match `genesis_payload.toml`, and `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` passed fresh.

Q8: PASS - I do not see a current P-M2 substrate break that should block on future P-M4/P-M6 work, but future pool/router atoms must be explicit about ownership and rollback. Merge consumes owner-held matched YES+NO inventory, so P-M4/P-M5 must ensure pool reserves are pool-controlled and not accidentally mergeable by an LP/provider account, and P-M6 must atomically roll back the mint-before-swap path so no matched residual YES+NO plus collateral remains available for a later merge. The existing asymmetric `assert_complete_set_balanced` branch remains a future P-M6 audit focus once swaps can create pre-resolution YES/NO imbalance.

## VERDICT: CHALLENGE
Conviction: high
Recommendation: FIX-THEN-PROCEED
Remediations:
- Replace the manual `QState` resolution-state reseats in the two negative tests with live sequencer/system-transition setup, or add a separate live-resolution witness and narrow these tests' comments so they no longer claim full no-fixture coverage.
- Resolve the zero-amount policy drift in the CompleteSetMerge arm: either remove the `amount.units == 0` rejection for strict §7.3 semantics, or obtain architect-ratified wording and add an explicit zero-amount rejection test.
