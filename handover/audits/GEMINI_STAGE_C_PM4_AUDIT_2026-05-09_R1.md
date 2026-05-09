# Gemini Audit Report: Stage C P-M4 Rebuild (R1)

- **HEAD**: `023fe32`
- **Date**: 2026-05-09
- **Round**: 1
- **Aggregate Verdict**: PASS

---

Q1: **Verbatim alignment (defect 4 prevention)**: Does `CpmmPool` exactly match architect §7.5 5-field spec — `(event_id, EventId)` + `(pool_yes, ShareAmount)` + `(pool_no, ShareAmount)` + `(lp_total_shares, LpShareAmount)` + `(status, PoolStatus)`? Does `CpmmPoolSigningPayload` mirror as 6-field projection (signature excluded)? Verify NO `event_id_kind`, NO `timestamp_logical`. Verify both BINDINGS entries are `LandingStatus::Landed`.

Q1: PASS - The `CpmmPool` struct in `src/state/q_state.rs` is a verbatim match for the 5-field architect spec, correctly using `event_id: ...::EventId`. The `CpmmPoolSigningPayload` in `src/state/typed_tx.rs` is the correct 6-field projection of the 7-field `CpmmPoolTx`, excluding the signature. Neither struct contains `event_id_kind` or `timestamp_logical`, preventing the defects from the prior VETO. The `constitution_architect_verbatim_struct_binding.rs` test confirms both bindings are `Landed` and enforces this alignment mechanically.

Q2: **Test body realism**: Each of the 4 tests in `tests/constitution_cpmm_pool.rs` must reach `submit_and_apply` → `dispatch_transition` → `q_next` mutation through the LIVE sequencer, not fixture-forge. Specifically: does `pool_cannot_exist_without_collateralized_shares` actually trigger `TransitionError::InsufficientSharesForPool` from the rejection path? Do `pool_reserves_not_counted_as_coin` + `lp_shares_not_counted_as_coin` compute and compare actual `total_supply_micro` pre/post via `assert_total_ctf_conserved`?

Q2: PASS - The four tests in `constitution_cpmm_pool.rs` correctly use a test harness that invokes the live sequencer via `submit_and_apply`. `pool_cannot_exist_without_collateralized_shares` successfully asserts that the sequencer rejects the transaction with `InsufficientSharesForPool`. The `pool_reserves_not_counted_as_coin` and `lp_shares_not_counted_as_coin` tests correctly use `assert_total_ctf_conserved` to witness that total coin supply is unchanged, proving that neither pool reserves nor LP shares are counted as Coin.

Q3: **Sequencer admission completeness**: The accept arm at `src/state/sequencer.rs::TypedTx::CpmmPool` must enforce all 5 preconditions, apply 3 atomic mutations, call all 3 monetary invariants, and advance state_root. Read the arm; verify each step. Are any clauses missing or extra clauses introduced beyond the architect §7.5 rules + the strict-spec implementation choices documented in the packet (symmetric-init invariant; one-pool-per-event)?

Q3: PASS - The `TypedTx::CpmmPool` arm in `src/state/sequencer.rs` is complete. It correctly enforces all 5 specified preconditions (StaleParent, InvalidPoolSeed, UnbalancedPoolSeed, InsufficientSharesForPool, PoolAlreadyExists), then applies the 3 atomic mutations (debit shares, create pool, credit LP shares), calls all 3 required monetary invariants, and advances the state root using the deterministic `cpmm_pool_accept_state_root` helper. No steps are missing or extraneous.

Q4: **`assert_complete_set_balanced` extension safety**: This commit extends the symmetric-branch totals to include `cpmm_pools_t[event_id].pool_yes / pool_no`. Is this extension correct (claims against the same locked collateral) and safe (does NOT introduce ghost liquidity, does NOT break the asymmetric-branch CTF-MIN-SAFE post-resolution path, does NOT silently admit unbalanced pools)? Verify the asymmetric branch `min()` reduction is unchanged + still CTF-MIN-SAFE marker-protected. Verify `tests/constitution_economy_strict_equality.rs` 8/8 PASS.

Q4: PASS - The extension is correct and safe. `assert_complete_set_balanced` in `src/economy/monetary_invariant.rs` now correctly includes pool reserves in the total share count, recognizing them as claims against the same collateral. The logic for the asymmetric branch, including the `min()` reduction and its `CTF-MIN-SAFE` audit marker, is unchanged. The `constitution_economy_strict_equality` test suite passes, confirming that no dangerous `min()` calls were introduced into the symmetric path.

Q5: **CTF conservation under pool creation**: `assert_total_ctf_conserved` is called in the accept arm with empty exempt-list. Pool reserves and LP shares are NOT in `total_supply_micro` per architect §7.5 rules 2 + 3. Verify the 6-holding sum is bit-identical pre/post (witness: P-M4 only moves YES + NO claims from `conditional_share_balances_t` to `cpmm_pools_t.pool_yes/no`, neither of which is a Coin holding; conditional_collateral_t unchanged). Confirm `assert_no_post_init_mint` allow-list extension for `TypedTx::CpmmPool` is correct (no Coin minted).

Q5: PASS - The implementation correctly ensures CTF conservation. The `total_supply_micro` function does not count `cpmm_pools_t` or `lp_share_balances_t` as Coin holdings. The `CpmmPoolTx` transaction is a pure share migration and does not alter any of the six canonical Coin holdings. The call to `assert_total_ctf_conserved` with an empty exempt list passes, and `assert_no_post_init_mint` is correctly extended for this transaction type, confirming no Coin is minted.

Q6: **F-DEFERRAL closure**: Per remediation directive §9, F-DEFERRAL-2 requires extending E.1 BINDINGS with sibling SigningPayload entry. Verify `tests/constitution_architect_verbatim_struct_binding.rs` BINDINGS contains `CpmmPoolSigningPayload` (Landed; 6 wire fields). F-DEFERRAL-1 vacuously closed (no helper alias). E.1 parser hardening (path-qualified type handling) is forward-looking and does not weaken existing P-M2 binding (verify via 4 self-check tests all PASS).

Q6: PASS - Both deferrals are closed. `constitution_architect_verbatim_struct_binding.rs` contains the required `CpmmPoolSigningPayload` binding marked as `Landed`, satisfying F-DEFERRAL-2. F-DEFERRAL-1 is vacuously closed as no helper aliases were introduced. The passing self-check tests confirm the E.1 parser was successfully hardened to handle path-qualified types without weakening existing bindings.

Q7: **Replay-determinism**: Verify (a) `cpmm_pool_accept_state_root` is deterministic (sha256 of canonical_encode under domain prefix `b"turingosv4.cpmm_pool.accept.v1"`); (b) `TxKind::CpmmPool = 15` added; (c) 7 trust_root rehashes correct (q_state.rs / typed_tx.rs / sequencer.rs / transition_ledger.rs / monetary_invariant.rs / verify.rs / run_summary.rs); (d) `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` PASS at HEAD `023fe32`; (e) EconomicState 13→15 sub-fields with `#[serde(default)]` preserves backward-compat.

Q7: PASS - All conditions for replay-determinism are met. The state root function is deterministic, `TxKind::CpmmPool` is correctly set to 15, the 7 required files are rehashed in `genesis_payload.toml`, the `verify_trust_root_passes_on_intact_repo` test passes, and the new `EconomicState` fields use `#[serde(default)]` to ensure backward compatibility.

Q8: **Strategic risk**: What in Phase F.3 P-M4 substrate is visibly wrong or missing that future Phase F.4 (P-M5 CpmmSwap) or F.5 (P-M6 BuyWithCoinRouter rebuild with strict-equality monetary_invariant + atomic-rollback witness) would expose? In particular: how does pool creation interact with future swaps (constant-product invariant `k = pool_yes * pool_no`)? Does pool creation introduce any precondition that a future swap arm would have to reason about (status=Active gate)? Does the symmetric-init constraint (`seed_yes == seed_no`) cause issues when post-resolution pool state would naturally become asymmetric? Are there subtle invariant breaks that pass narrow tests but would surface under real-LLM Polymarket smoke at P-M9?

Q8: PASS - Within the defined scope of P-M4, no significant strategic risks are introduced. The substrate is sound for future phases. The creation logic correctly establishes `k > 0` and `status=Active`, which are necessary preconditions for swaps. The symmetric-init constraint is a reasonable simplification for this phase and does not prevent the pool from becoming asymmetric through future swaps or resolution. The known risk regarding the monetary invariant's asymmetric branch (`min()` usage) is a P-M6 concern and is not a flaw in this P-M4 commit.

## VERDICT: PASS
Conviction: high
Recommendation: PROCEED
Remediations:
- None.
