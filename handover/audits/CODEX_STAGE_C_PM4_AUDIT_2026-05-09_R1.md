# Codex Audit: Stage C P-M4 CpmmPool Rebuild

- HEAD: `023fe321ddecece847975ec660f16471ceebb6fe`
- Date: 2026-05-09
- Round: R1
- Final Aggregate Verdict: PASS

Q1: PASS - `CpmmPool` in `src/state/q_state.rs` matches architect section 7.5 exactly as a 5-field state struct: `event_id: EventId`, `pool_yes: ShareAmount`, `pool_no: ShareAmount`, `lp_total_shares: LpShareAmount`, and `status: PoolStatus`. The implementation uses path-qualified `crate::state::typed_tx::{EventId, ShareAmount}`, and the binding parser correctly normalizes those to the architect type tokens. `CpmmPoolSigningPayload` is the 6-field projection of the 7-field wire tx, excluding only `signature`. I found no `event_id_kind` or `timestamp_logical` in either P-M4 struct. The E.1 BINDINGS entries for `CpmmPool` and `CpmmPoolSigningPayload` are both `LandingStatus::Landed`; `cargo test --test constitution_architect_verbatim_struct_binding` passed 5/5.

Q2: PASS - The four tests in `tests/constitution_cpmm_pool.rs` use the live `Sequencer::submit_agent_tx` plus `try_apply_one` path through `submit_and_apply`; they do not forge post-state fixtures. `pool_created_from_seed_inventory` mints inventory first, then submits `CpmmPoolTx`, and asserts the resulting `cpmm_pools_t`, `conditional_share_balances_t`, and `lp_share_balances_t` state. `pool_cannot_exist_without_collateralized_shares` submits a real pool tx from an empty share balance and observes the live rejection as `InsufficientSharesForPool`.

`pool_reserves_not_counted_as_coin` and `lp_shares_not_counted_as_coin` snapshot the real pre-pool `EconomicState`, submit the pool tx, and call production `assert_total_ctf_conserved(&pre, &post, &[])` over the real post-state. `cargo test --test constitution_cpmm_pool` passed 4/4.

Q3: PASS - The `TypedTx::CpmmPool` arm in `src/state/sequencer.rs` enforces all five stated preconditions in order: stale parent rejection, non-zero seeds, symmetric seed equality, provider YES/NO inventory, and one-pool-per-event. It then applies the three atomic mutations: debits provider conditional shares, creates an `Active` `CpmmPool` with `lp_total_shares = seed_yes`, and credits provider LP shares.

The arm calls all three monetary invariants: `assert_no_post_init_mint`, `assert_total_ctf_conserved` with an empty exempt list, and `assert_complete_set_balanced`; it then advances the state root through `cpmm_pool_accept_state_root`. I did not find extra admission clauses beyond the packet's documented strict choices, nor missing clauses from the packet's five-stage admission contract.

Q4: PASS - The `assert_complete_set_balanced` extension correctly adds `cpmm_pools_t[event_id].pool_yes` and `.pool_no` into the YES/NO share totals. That is the right accounting for P-M4 because pool creation moves existing collateralized claims out of provider `conditional_share_balances_t` into pool reserves while leaving `conditional_collateral_t` unchanged.

The asymmetric branch remains the pre-existing `sum_yes.min(sum_no)` branch and is still guarded by the `CTF-MIN-SAFE` marker; the symmetric branch still requires strict equality against collateral. P-M4 itself cannot silently admit an unbalanced pool because the sequencer rejects `seed_yes != seed_no` before mutation, and the strict-equality lint passed: `cargo test --test constitution_economy_strict_equality` passed 8/8.

Q5: PASS - `assert_total_ctf_conserved` is called across pool creation with `&[]`, and the six Coin-holding sum remains bit-identical because P-M4 only moves conditional share claims from the provider to the pool reserve fields. Neither `cpmm_pools_t.pool_yes/no` nor `lp_share_balances_t` participates in `total_supply_micro`; `conditional_collateral_t` is unchanged. The `assert_no_post_init_mint` allow-list addition for `TypedTx::CpmmPool` is correct for this pure share migration.

Q6: PASS - F-DEFERRAL-2 is closed for P-M4: `tests/constitution_architect_verbatim_struct_binding.rs` contains the sibling `CpmmPoolSigningPayload` binding, marked `Landed`, with the expected six signing fields. F-DEFERRAL-1 is vacuously closed for this atom because no helper alias or cross-file reduction helper was introduced for the monetary invariant.

The parser hardening for path-qualified types is forward-looking and preserves the P-M2 binding behavior. The four self-checks named in the packet all passed as part of the 5-test binding suite.

Q7: PASS - `cpmm_pool_accept_state_root` is deterministic: SHA-256 over `b"turingosv4.cpmm_pool.accept.v1"`, the previous state root bytes, and canonical-encoded `TypedTx`. `TxKind::CpmmPool = 15` is present after `CompleteSetMerge = 14`. The seven trust-root files listed in the packet have hashes in `genesis_payload.toml` matching fresh `sha256sum` output for `q_state.rs`, `typed_tx.rs`, `sequencer.rs`, `transition_ledger.rs`, `monetary_invariant.rs`, `verify.rs`, and `run_summary.rs`.

`cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` passed 1/1. `EconomicState` is 13 to 15 with `#[serde(default)]` on `cpmm_pools_t` and `lp_share_balances_t`; the reconstruct/count tests reflect 15 fields. Full validation also matched the packet baselines: constitution gates reported 207 passed / 0 failed / 1 ignored, and `cargo test --workspace --no-fail-fast` produced 1340 passed / 0 failed / 151 ignored.

Q8: PASS - I do not see a P-M4 ship-blocking substrate defect. Pool creation establishes a well-defined initial constant product with `seed_yes > 0`, `seed_no > 0`, and `seed_yes == seed_no`, so future P-M5 swap code starts from `k > 0`. The future swap arm must still enforce `status == Active`, preserve or intentionally update `k`, and decide how pool status follows event resolution; those are P-M5/P-M6 responsibilities, not missing P-M4 mutations.

The symmetric-init constraint is conservative for creation and does not by itself prevent later asymmetric pool reserves from swaps or resolution handling. The main future-risk area is not this creation arm but the later transition from pre-resolution symmetric accounting into post-swap/post-resolution asymmetric accounting: P-M5/P-M6 must explicitly distinguish swap-induced asymmetry from the existing post-resolution `CTF-MIN-SAFE` branch and should keep the pool status gate load-bearing in tests.

## VERDICT: PASS
Conviction: high
Recommendation: PROCEED
Remediations: None.
