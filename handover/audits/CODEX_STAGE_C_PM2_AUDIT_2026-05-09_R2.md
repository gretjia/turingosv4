# Codex Stage C P-M2 Audit R2

HEAD: `851364a`
Round: R2
Date: 2026-05-09
Scope: focused re-audit of R1 Q2 and Q3 remediation in `444c470`.

Fresh verification run:
- `cargo test --test constitution_completeset_merge` -> 5 passed / 0 failed
- `cargo test --test constitution_architect_verbatim_struct_binding` -> 5 passed / 0 failed
- `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` -> 1 passed / 0 failed
- `bash scripts/run_constitution_gates.sh 2>&1 | tail -5` -> Totals: 198 passed, 0 failed, 1 ignored; PASS
- `git show 444c470 -- src/state/sequencer.rs | head -80` and `git show 444c470 -- tests/constitution_completeset_merge.rs | head -60` confirm the remediation diff under review.
- `ls tests/tb_8_minimal_payout.rs tests/tb_11_*.rs` -> `tests/tb_8_minimal_payout.rs`, `tests/tb_11_epistemic_exhaust.rs`

Q2: PASS - `merge_requires_both_sides` is now fully live. The test starts from `genesis_with_balances_and_open_task`, drives a live `CompleteSetMintTx` through `submit_and_apply`, drives a live merge-all `CompleteSetMergeTx` through the same path, asserts both sides are zero, then drives the failing merge attempt through the live sequencer accept arm. I see no fixture-side state flip and no fresh-harness reseat in this test.

`merge_unavailable_after_final_redeem_if_shares_exhausted` now correctly bounds its claim: it explicitly labels the `TaskMarketState::Finalized` setup as fixture-side pre-condition staging, while preserving live redeem and live merge rejection through `submit_and_apply` -> `dispatch_transition`. The cross-reference targets exist. `tests/tb_8_minimal_payout.rs` contains live `emit_system_tx(SystemEmitCommand::FinalizeReward)` -> `try_apply_one` coverage and asserts `ClaimStatus::Finalized`; `tests/tb_11_epistemic_exhaust.rs` contains live `emit_system_tx(SystemEmitCommand::TaskBankruptcy)` -> `try_apply_one` coverage and asserts `TaskMarketState::Bankrupt`.

Q3: PASS - The active `TypedTx::CompleteSetMerge` accept arm no longer has an early `merge.amount.units == 0` rejection. The remaining `InsufficientSharesForMerge` exits are only the architect-mandated YES and NO balance preconditions. The new inline comment documents the strict architect section 7.3 reading: zero amount satisfies `>= amount`, the debit and credit steps are `-= 0` / `+= 0`, and the previous zero rejection was extra policy not ratified by the verbatim semantics block.

I do not see a new invariant edge from zero admission for a typical post-mint state. After mint, aggregate YES, aggregate NO, and event collateral are equal; a zero merge leaves all economic quantities unchanged before `assert_complete_set_balanced` runs, so the invariant remains on its symmetric strict-equality branch. The five architect-mandated merge tests pass against the modified accept arm.

Q1/Q4/Q5/Q6/Q7/Q8: PASS carried forward from R1. I saw no regression while rebasing the focused audit against HEAD `851364a`; the struct-binding gate, trust-root test, and full constitution gate tail are green.

## VERDICT: PASS
Conviction: high
Recommendation: PROCEED
Remediations: None.
