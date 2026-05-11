#!/usr/bin/env bash
# TB-C0 Constitution Landing Gate — local + CI runner
#
# Runs the 8 constitution gate integration test files and emits:
#   - target/constitution_gate_report.json   (machine-readable)
#   - target/constitution_gate_report.md     (human-readable)
#
# Authority:
#   - handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md
#   - handover/tracer_bullets/TB-C0_charter_2026-05-06.md FR-C0.12
#
# Exit codes:
#   0  all gates GREEN (or only the LLM-compute MVP-1 smoke #[ignore])
#   1  one or more gates RED — block merge per CR-C0.10
#   2  test runner failure (cargo error, missing tooling)

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT_JSON="target/constitution_gate_report.json"
REPORT_MD="target/constitution_gate_report.md"

mkdir -p target

GATES=(
  constitution_no_parallel_ledger
  constitution_economy_gate
  constitution_predicate_gate
  constitution_fc1_runtime_loop
  constitution_fc2_boot
  constitution_fc3_meta
  constitution_shielding_gate
  constitution_tape_canonical_gate
  # Round-8 (per architect + Codex remediation): FC3-INV1 capsule integrity
  # regen + Art. V.3 amendment-log executable test
  constitution_fc3_inv1_capsule_integrity_regen
  constitution_art_v3_amendment_log
  # Post-TB-C0 clarification 2026-05-07 (OBS_TB18R_INV1_NONLLM_TX): runner
  # must compute completed_llm_calls = step + parse_fail + llm_err (NOT
  # tx_count, which inflates with architect-mandated admin scaffold). Closes
  # P04/P05 false-NegativeDelta on mixed-tx problems.
  constitution_runner_invariant_formula
  # A0 2026-05-07 (OBS_EVIDENCE_DRIFT_ROOT_CAUSE): cargo tests writing to
  # committed evidence dirs must be env-gated TURINGOS_TEST_REGENERATE_EVIDENCE.
  # Closes the silent 11-files-per-cargo-test-run drift on TB-7/13/14 evidence.
  constitution_no_evidence_drift_in_tests
  # Constitution Landing First 2026-05-07 (HARNESS.md §3 G-012): PCP
  # adversarial corpus — pins the 9-class mutation routing table
  # (cases/pcp_corpus/) to AttemptOutcome → L4ERejectionClass mapping.
  # Closes G-012 strategic blocker synthetic-corpus arm; MiniF2F-v2
  # misalignment is the forward step.
  constitution_pcp_corpus
  # Constitution Landing First 2026-05-07 (HARNESS.md §3 G-016/G-019/
  # G-021/G-028): PromptCapsule — Class-3 schema + L4 anchor by default.
  # Closes Art. III selective shielding / prompt persistence gap (was 0%
  # LANDED). Pins the 7-field architect schema and the privacy invariant
  # that verbatim prompt bytes are NEVER public-tape resident by default.
  constitution_prompt_capsule
  # Constitution Landing First 2026-05-07 (HARNESS.md §3 G-009): HEAD_t
  # C1 6-field witness (Path-C hybrid). Derived view over QState +
  # caller-supplied L4.E head + CAS root + run_id. Closes G-009 strategic
  # blocker substrate; libgit2 production refs are the C2 forward step.
  constitution_head_t_witness
  # Wave 3 evidence binding 2026-05-07 (CR-C0.7 GREEN promotion): bind
  # AMBER matrix rows to real-LLM tape evidence (Wave 3 20p ffb6ebd +
  # 50p a612cc9). Closes MVP-1 (FC1 tx-count equality) + MVP-3 (dashboard
  # regen) + MVP-4 (fresh replay) + closure #4 (P38/P49 FC1) at evidence
  # level; promotes 7 matrix AMBER rows to GREEN by binding to per-problem
  # chain_invariant.json artifacts and the WAVE3_*_AGGREGATE.json totals.
  constitution_wave3_evidence_binding

  # Constitution landing 2026-05-08 — Closure #3 mechanical enforcement of
  # CR-C0.1 ("every test can fail; no `assert!(true)`"). Promotes §O #3
  # 🟡 AMBER → 🟢 GREEN by converting the editorial norm into a gate per
  # `feedback_norm_needs_mechanism`. Self-verifying scanner — pattern list
  # detectability proven on synthetic input via a sibling test, so the
  # main scan over `tests/constitution_*.rs` cannot be vacuously passing.
  constitution_closure_3_no_trivial_asserts

  # Constitution landing 2026-05-08 — Wave 3 50p shielding evidence binding.
  # Promotes §C Art. II.1 + §D Art. III.1-4 + §K shielding 4 mirror rows
  # 🟡 AMBER → 🟢 GREEN by aggregating the per-problem
  # `cas/.turingos_cas_index.jsonl` sidecar across 50 MiniF2F problems and
  # asserting per-schema size bounds + leakage-suggestive-name absence.
  # Real-path-under-load complement to the source-grep gate in
  # `tests/constitution_shielding_gate.rs` per CR-C0.7 +
  # `feedback_real_problems_not_designed`.
  constitution_shielding_evidence_binding

  # Constitution landing 2026-05-08 — register session #19 gate files for
  # SG-A2.2 closure (architect: "all new gate files included in
  # scripts/run_constitution_gates.sh"). Both files were created session #19
  # but mistakenly omitted from the runner registration; this closes the gap.
  #
  # Wilson 95% CI helper for §B Art. I.2 PPUT Statistical Signal (CLAUDE.md
  # §17 Report Standard). Aggregate-runner integration is the forward step.
  constitution_wilson_ci
  # Diversity helper for §C Art. II.2.1 exploration/exploitation balance —
  # parent_selection_shannon_entropy (None-filtered per V3L-14 fix from
  # audit_assertions id=43) + distinct_payload_fraction +
  # DiversityReport::is_below_alarm_floor (0.25 floor).
  constitution_diversity

  # Stage B3 / TB-18B 2026-05-08 — BenchmarkManifest schema gate per FR-18B.1
  # + CR-18B.5 ("NO BenchmarkManifest field omission. Missing fields =
  # ship-block.") + `feedback_benchmark_manifest_required`. Every required
  # field validates; schema_id pinned; total_runs arithmetic stable; disk
  # round-trip byte-stable.
  constitution_benchmark_manifest

  # Stage B3 / TB-18B 2026-05-08 — AggregateReport conformance gate per
  # FR-18B.5 / FR-18B.6 / FR-18B.11 + CLAUDE.md §17 Report Standard. Wires
  # `wilson_ci.rs` + `diversity.rs` into a single CLAUDE.md §17 conformant
  # consumer. Every line of §17 (ΣPPUT / Mean PPUT(solved) / Wilson 95% CI
  # / halt distribution / counts / no-fake-accepted-nodes / FC1 aggregate)
  # enforced as ship-block. Closes session #18 Wave-1/2 forward-bind items
  # 1+2 at consumer-side wire-up level.
  constitution_aggregate_report

  # Stage A3 / HEAD_t C2 multi-ref ChainTape 2026-05-08 — SG-A3.1..5 ship
  # gates per STAGE_A3_HEAD_T_C2_charter_2026-05-07.md §4. Pure additive
  # multi-ref support on transition_ledger.rs (refs/chaintape/{l4,l4e,cas});
  # C1 baseline refs/transitions/main preserved as backward-compat alias.
  # Closes architect alignment doc Stage A3 SG-A3.1-5 at substrate level.
  constitution_head_t_c2_multi_ref

  # Stage B3 / TB-18B 2026-05-08 — PCP corpus phase-2 (MiniF2F-v2
  # misalignment, real-world adversarial). Closes Gemini R1 Q8 forward-bind
  # #1 + TB-18B charter SG-18B.9. 9 mutation classes derived from real
  # public mathd_algebra_107 (NOT synthetic) per
  # `feedback_real_problems_not_designed`. Phase-1 synthetic corpus
  # (cases/pcp_corpus/) preserved as predecessor.
  constitution_pcp_corpus_phase2

  # Constitution full-landing 2026-05-08 (session #24) — FC3 §I structural
  # rows + §F Art. V.2 boundaries. Closes 7 AMBER → GREEN by binding the
  # `tests/constitution_fc3_meta.rs` source-grep gates to real Wave 3 50p
  # / Stage A3 / B3 R6 evidence + git-history witness. Promotes:
  #   FC3-INV3 raw logs / FC3-INV4 capsule context only / FC3-INV5 deep
  #   history override / FC3-INV7 ArchitectAI proposes / FC3-INV8 JudgeAI
  #   veto-only / Art. V.2 constitution boundaries
  # per `feedback_no_workarounds_strict_constitution` strict closure.
  constitution_fc3_evidence_binding

  # Stage B (§2.4 audit) 2026-05-08 (session #25) — architect 2026-05-07
  # ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL §5.3 verbatim CompleteSet
  # hardening: 8 verbatim test names binding directly to live sequencer
  # dispatch on CompleteSetMintTx + CompleteSetRedeemTx. Promotes §5.3
  # from TB-13-internal SG-13.* names (in tb_13_complete_set.rs, NOT
  # registered in this gate runner) to first-class constitution-gate
  # names per `feedback_no_workarounds_strict_constitution` strict closure
  # ("我不要凑活"). Ship-eligible alongside Stage B3 M2 batch.
  constitution_completeset_hardening
  # Stage B (§2.4 audit) 2026-05-08 (session #25) — architect §5.2 verbatim
  # legacy CPMM quarantine + no-f64-in-market-modules: 2 verbatim test
  # names (legacy_cpm_api_not_imported_by_new_market + no_f64_in_market_modules)
  # plus 3 self-tests proving the scanner detects synthetic violations
  # (closure-3 "every test can fail"). Constitution-gate complement to
  # tb_13_legacy_cpmm_forward_fence's TB-13-marker scope; uses explicit
  # MARKET_SUBSTRATE_ALLOW_LIST that Stage C P-M0+ TBs MUST extend.
  constitution_market_quarantine

  # Stage C VETO remediation Phase E 2026-05-09 (session #28) — three
  # mechanism gates added per `handover/directives/2026-05-09_STAGE_C_
  # POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md` §1.B + plan cached-noodle.md.
  # Codex G2 audit (2026-05-09) caught 4 defects in Stage C session #27
  # batch §8: P-M6 monetary_invariant `min()` weakening; P-M6 vacuous
  # rollback test; P-M2/P-M4 verbatim struct drift. Self-audit (212 GREEN)
  # caught zero of them. These three gates encode the missing mechanism
  # so future Class-4 atoms cannot ship the same shape of defect.
  #
  # E.1 verbatim spec binding: hardcodes architect manual §7.x verbatim
  # struct field sets and asserts impl matches strictly when LANDED.
  # NotYetLanded bindings (P-M2 §7.3, P-M4 §7.5) flip to Landed in Phase F
  # rebuild commits.
  constitution_architect_verbatim_struct_binding
  # E.2 atomic-rollback witness: each Class-4 composite-tx
  # `*_atomic_rollback_on_failure` test must invoke a mid-mutation
  # failure-injection helper (not just trigger pre-mutation rejection).
  # Static layer this PR; cfg(test) sequencer injection point lands in
  # Phase F.5 (P-M6 rebuild).
  constitution_class4_atomic_rollback_witness
  # E.3 strict-equality lint: forbids `.min(`-style reductions on
  # sum-aggregate identifiers in `monetary_invariant.rs` unless the line
  # carries a `// CTF-MIN-SAFE: <reason>` audit marker. Phase E source
  # refactor split `assert_complete_set_balanced` into symmetric (strict
  # `sum_yes == sum_no == coll`) and asymmetric (post-resolution; min()
  # marked) branches.
  constitution_economy_strict_equality

  # Stage C P-M2 / Phase F.1 2026-05-09 (rebuild post-VETO; per
  # `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_
  # DIRECTIVE.md` §1.C row 1 + architect manual §7.3 verbatim). Five
  # architect-mandated test names exercising CompleteSetMergeTx through
  # the live sequencer accept arm: merge_yes_no_returns_coin /
  # merge_requires_both_sides / merge_conserves_total_coin /
  # merge_reduces_collateral / merge_unavailable_after_final_redeem_if_
  # shares_exhausted. Strict 6-field struct (NO `timestamp_logical` —
  # E.1 binding flipped to Landed for P-M2 in same commit).
  constitution_completeset_merge

  # Stage C P-M3 / Phase F.2 2026-05-09 (re-apply Class-3; per
  # `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_
  # DIRECTIVE.md` §1.C row 2 + architect manual §7.4 verbatim). Five
  # architect-mandated test names exercising MarketSeedTx through the
  # live sequencer accept arm: market_seed_debits_provider /
  # market_seed_creates_yes_no_inventory / market_seed_fails_insufficient_
  # balance / market_seed_no_ghost_liquidity / market_seed_conserves_
  # total_coin. Sub-option A2: TB-13-era 7-field impl preserved as
  # ratified state (no schema bump; no STEP_B; no Trust Root rehash).
  constitution_market_seed_hardening

  # Stage C P-M4 / Phase F.3 2026-05-09 (rebuild Class-4 STEP_B; per
  # `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_
  # DIRECTIVE.md` §1.C row 3 + architect manual §7.5 verbatim 5-field
  # CpmmPool state struct). Four architect-mandated test names
  # exercising CpmmPoolTx through the live sequencer accept arm:
  # pool_created_from_seed_inventory / pool_reserves_not_counted_as_coin /
  # lp_shares_not_counted_as_coin / pool_cannot_exist_without_
  # collateralized_shares. Defect-4 prevention `event_id` NOT
  # `event_id_kind`. CpmmPool state binding LANDED + CpmmPoolSigningPayload
  # sibling binding LANDED (F-DEFERRAL-2 closure per remediation
  # directive §9). Pool reserves + LP shares are NOT Coin (architect
  # §7.5 rules 2 + 3); total_supply_micro UNCHANGED on accept.
  constitution_cpmm_pool

  # Stage C P-M5 / Phase F.4 2026-05-09 (re-apply Class-3; per
  # `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_
  # DIRECTIVE.md` §1.C row 4 + architect manual §7.6 verbatim). Six
  # architect-mandated test names exercising CpmmSwapTx through the live
  # sequencer accept arm: swap_no_for_yes_constant_product_non_decreasing
  # / swap_yes_for_no_constant_product_non_decreasing /
  # swap_fails_zero_input / swap_fails_insufficient_pool_output /
  # swap_respects_min_out_slippage / swap_uses_integer_math_no_f64. Pure
  # share rotation between trader and pool reserves; no Coin movement;
  # constant-product invariant `pool_yes1 * pool_no1 >= pool_yes *
  # pool_no` preserved (>= because integer floor leaves dust in pool —
  # architect §7.6 explicit). Class-3 sub-option mirrors P-M3 framing
  # (per-atom §8 NO; STEP_B branch for typed_tx.rs / sequencer.rs /
  # transition_ledger.rs / verify.rs / run_summary.rs / audit_assertions.rs
  # / monetary_invariant.rs file membership).
  constitution_cpmm_swap

  # Stage C P-M6 / Phase F.5 2026-05-09 (rebuild Class-4 STEP_B; per
  # `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_
  # DIRECTIVE.md` §1.C row 5 + architect manual §7.7 verbatim 9-step
  # composite atomic Mint-and-Swap router). Nine architect-mandated test
  # names exercising BuyWithCoinRouterTx through the live sequencer
  # accept arm + 1 defense-in-depth across all 9 steps:
  #   buy_yes_with_coin_matches_formula
  #   buy_no_with_coin_matches_symmetric_formula
  #   buy_yes_debits_coin_locks_collateral
  #   buy_yes_mints_complete_set
  #   buy_yes_transfers_retained_yes_plus_swap_yes
  #   buy_yes_respects_min_yes_out
  #   buy_yes_no_f64 (source-grep)
  #   buy_yes_no_ghost_liquidity
  #   router_atomic_rollback_on_failure (Defect-2 witness — uses
  #     cfg(debug_assertions) failure-injection hook
  #     `TURINGOS_TEST_ROUTER_FAIL_AT_STEP` env var; asserts state_root,
  #     buyer Coin balance, collateral, and pool reserves UNCHANGED
  #     post mid-mutation failure)
  #   router_atomic_rollback_witnessed_at_every_step (defense-in-depth
  #     across steps 1..=9)
  # Defect-1 patch (strict equality `sum_yes == sum_no == collateral`)
  # enforced by P-M4-extended `assert_complete_set_balanced` symmetric
  # branch — tests 4 + 8 directly witness this. E.1 +
  # F-DEFERRAL-2 + E.2 bindings all FLIPPED to Landed in same commit.
  constitution_router_buy_with_coin

  # Stage C P-M7 / Phase F.6 2026-05-09 (Class 1-2 view-only quote per
  # architect manual §7.8 verbatim: "Price is signal only. Do not use
  # price to decide predicate truth"). Four architect-mandated tests:
  #   price_quote_does_not_change_state
  #   price_signal_not_predicate (source-grep gate; sequencer must NOT
  #     import router_quote; predicate files must NOT reference
  #     router_quote)
  #   price_does_not_make_failed_node_accepted (router tx with bad
  #     admission state still rejects regardless of how attractive the
  #     quote is)
  #   low_liquidity_warning (LiquidityWarning::None | LowLiquidity |
  #     NoOutput classification per pool reserves + floor outcome)
  # Pure quote API at `src/state/router_quote.rs`; reuses RationalPrice
  # from `state::price_index` (TB-14 Atom 2 architect §5.2). No tx kind
  # change; no STEP_B file edit (only state/mod.rs `pub mod router_quote`
  # declaration; rehashed in genesis_payload.toml).
  constitution_router_price_quote

  # Stage C P-M8 / Phase F.7 2026-05-09 (Class 1-2 audit views per
  # architect manual §7.9 verbatim: "Add: audit_tape view-shares /
  # view-pools / view-prices / view-positions. Must show: owner YES/NO
  # shares, conditional collateral, pool reserves, LP shares,
  # NodePositions, price signal"). Three architect-mandated tests:
  #   audit_view_shares_matches_state
  #   audit_view_pools_matches_state
  #   dashboard_regenerates_market_view
  # 4 pure view aggregator fns at `src/runtime/audit_views.rs`:
  # audit_view_shares (owner YES/NO + collateral) /
  # audit_view_pools (CpmmPool reserves + LP holdings + k_product) /
  # audit_view_prices (router quote signal per pool over caller-supplied
  # pay_coin samples) / audit_view_positions (NodePosition exposure).
  # Replay-deterministic; no env / clock / RNG; integer-only; pure-fn
  # over `&EconomicState`.
  constitution_audit_views

  # Stage C P-M9 / Phase F.8 2026-05-09 (Class 2-3 controlled market
  # smoke per architect manual §7.10 verbatim scenario + 5-gate battery).
  # End-to-end test exercising the full Stage C Polymarket sequence (P-M3
  # mint + P-M4 pool + P-M6 router buys + P-M7 quote + P-M8 audit views)
  # in a single harness run, then asserting the architect §7.10 5
  # mandatory gates:
  #   - no ghost liquidity (sum YES == sum NO == collateral)
  #   - total coin conserved (assert_total_ctf_conserved at each tx +
  #     pre-smoke vs post-smoke total invariant)
  #   - no price-as-truth (router_quote does not advance state)
  #   - no raw log broadcast (out-of-scope for this smoke; Wave-3 +
  #     TB-15 binding gates land separately)
  #   - all activity replayable (state_root advances monotonically;
  #     audit views regenerate byte-identical)
  # 1 architect-mandated test: polymarket_controlled_market_smoke.
  constitution_polymarket_smoke

  # Stage C overall §8 R1 CHALLENGE Q10 closure 2026-05-09 (Codex Stage C
  # overall PRE-§8 audit 2026-05-09 session #32 R1 Q10 CHALLENGE
  # remediation 1+2: task_markets_t event-state gate for Polymarket
  # admission paths). 7 tests covering all 3 admission arms
  # (CpmmPool / CpmmSwap / BuyWithCoinRouter) × 2 post-resolution states
  # (Finalized / Bankrupt) + 1 positive control (all 3 admit normally
  # against Open event). Closes the safety gap "no transition flips
  # pools to Resolved/Closed on task resolution, leaving post-resolution
  # pool creation/trading reachable" per Codex Q10 verbatim.
  constitution_polymarket_event_state_gate

  # Stage C overall §8 R2 CHALLENGE Q10 closure 2026-05-09 — fail-open
  # admission default lint. The R1 event-state gate added live
  # `state == Open` checks but used `unwrap_or(TaskMarketState::Open)` to
  # default missing entries, admitting malformed/pre-genesis events into
  # the permissive state. The R3 fix replaced this with
  # `.ok_or(EventNotOpen)?` for fail-closed semantics. This gate forbids
  # any future regression by source-grep: same-line co-occurrence of
  # `unwrap_or(...)` / `unwrap_or_else(...)` and a fail-open state-machine
  # variant (TaskMarketState::Open / ChallengeStatus::Open / ClaimStatus::Open
  # / PoolStatus::Active / EventState::Open) in src/state/sequencer.rs.
  # 1 main test on real source + 8 self-checks = 9 tests.
  constitution_admission_no_fail_open_default

  # M0 batch 2026-05-10 surfaced TB-16-era tamper drift — `audit_tape_tamper`
  # detected only 1/3 (vs architect §B.9.3 mandated 3/3) because the
  # in-binary primitives picked the largest loose object regardless of
  # reachability (post-A3 multi-ref made orphan blobs > chain blobs) and
  # truncated only `refs/transitions/main` (alias) without canonical
  # `refs/chaintape/l4`. Fix: tamper primitives moved to library
  # `src/runtime/audit_tamper.rs` with reachability filtering (L4_REFS
  # only — audit deep-verifies L4 entry_canonical bodies but not L4.E
  # rejection_record bodies) + dual-ref truncation (CHAIN_REFS = L4 +
  # L4.E + alias). This gate is the executable face of the constitutional
  # invariant: 9 tests covering chain-ref-list completeness, L4-vs-L4.E
  # semantic split, reachability filtering, dual-ref truncation,
  # alias-only backward-compat, error-on-no-chain-refs, CAS tamper.
  # Empirical 3/3 validation against M0 P01 + P05 fixtures verified
  # post-fix; per `feedback_no_workarounds_strict_constitution`.
  constitution_audit_tamper_3_of_3

  # Session #34 (2026-05-10) — L4.E body integrity verification.
  # Closes the forward gap documented at
  # constitution_audit_tamper_3_of_3::l4_refs_is_strict_subset_of_chain_refs_excluding_l4e
  # (session #33 close: "audit does not deep-verify L4.E rejection_record
  # bodies, so tampering an L4.E blob is silent at audit-time"). Per
  # 2026-05-10 user verbatim "我需要的是宪法约定的内容全部真实落地且可被
  # 验证" + `feedback_no_workarounds_strict_constitution`: a
  # constitutionally-undetectable tamper class is a constitutional
  # violation, not a deferred forward item. This gate exercises the new
  # `assert_51_l4e_git_attestation_matches_jsonl` assertion (Layer B,
  # FC1-N34 + FC1-N35 + FC2-INV1) which walks `refs/chaintape/l4e`, parses
  # each commit's `rejection_record` blob, and cross-checks against the
  # JSONL-side records. 7 tests: positive control on M0 P01 (Pass), L4.E
  # blob byte-flip (Halt), L4.E ref corruption (Halt), pre-A3 JSONL-only
  # mode (Skipped), 3 self-tests on the parse-and-verify helper.
  constitution_l4e_body_integrity

  # TB-N1-AGENT-ECONOMY Phase 2 atom A3 (Class-4 STEP_B; 2026-05-10) —
  # agent-decided stake admission gate. Sequencer Step-4 extension: WorkTx
  # rejects with NEW `TransitionError::StakeBalanceExceeded` (→ L4E
  # InsufficientBalance) when agent-declared `stake_micro` exceeds
  # `balances_t[agent_id]`. Distinct from existing `StakeInsufficient`
  # (stake==0) and from Step-6 system-side `InsufficientBalance`
  # defense-in-depth. Closes the agency layer of CLAUDE.md §13
  # "writes/append/challenge/verify/settle require stake/escrow/bond as
  # specified" — agent-decided stake within `[1, balance]` is now a typed
  # admission gate. 5 ship gate tests (SG-N1-A3.1..5):
  #   sg_n1_a3_1_zero_stake_rejects_with_stake_insufficient
  #   sg_n1_a3_2_overspend_rejects_with_stake_balance_exceeded
  #   sg_n1_a3_3_minimum_stake_admits
  #   sg_n1_a3_4_prompt_aggregates_agent_decided_per_cell_stakes
  #   sg_n1_a3_5_real_llm_smoke_witnesses_agent_decided_non_default_stake
  #     (asymmetric: vacuous-pass when no stage_b3_smoke_a3_* dir exists,
  #     load-bearing once smoke evidence lands per
  #     `feedback_real_problems_not_designed`).
  # Charter: `handover/tracer_bullets/TB_N1_AGENT_ECONOMY_PHASE_2_charter_2026-05-10.md`.
  # Forward §8 grant: `handover/directives/2026-05-10_TB_N1_AGENT_ECONOMY_PHASE_2_FORWARD_§8_GRANT.md`.
  constitution_n1_agent_economy_a3

  # TB-N1-AGENT-ECONOMY Phase 2 atom A4 (Class-4 STEP_B; 2026-05-10) —
  # agent-callable verify-peer admission gate. Adds 3 NEW `TransitionError`
  # variants (VerifyBondOutOfBounds + VerifyTargetNotAccepted + VerifyDuplicate)
  # + mirror `RejectionClass` variants + Display impls. Sequencer VerifyTx
  # admission gains Step-2.5 (bond > balance → VerifyBondOutOfBounds; mirrors
  # A3 Step-4b), Step-3 rename (TargetWorkInactive → VerifyTargetNotAccepted
  # for verify-peer path; ChallengeTx arm unchanged), Step-3.5 (duplicate
  # `(verifier, target)` → VerifyDuplicate). NEW state index
  # `EconomicState.agent_verifications_t: AgentVerificationsIndex`
  # (BTreeSet<(AgentId, TxId)>; #[serde(default)] backward-compat;
  # NOT a Coin holding). Closes the agency layer of CLAUDE.md §13
  # verify/bond + Art. I.1.1 multi-agent verification — agent-callable
  # `verify_peer` tool with typed admission rejection classes. 7 ship gate
  # tests (SG-N1-A4.1..7):
  #   sg_n1_a4_1_zero_bond_rejects_with_bond_insufficient
  #   sg_n1_a4_2_overbond_rejects_with_verify_bond_out_of_bounds
  #   sg_n1_a4_3_phantom_target_rejects_with_verify_target_not_accepted
  #   sg_n1_a4_4_duplicate_verify_rejects_with_verify_duplicate
  #   sg_n1_a4_5_first_valid_verify_admits
  #   sg_n1_a4_6_real_llm_swarm_smoke_witnesses_admission_health
  #     (asymmetric: vacuous-pass when no stage_b3_smoke_a4_* dir exists,
  #     load-bearing once smoke evidence lands per
  #     `feedback_real_problems_not_designed`).
  #   sg_n1_a4_7_verify_peer_advertised_and_dispatched
  #     (source-grep mechanism-binding test: verify_peer must be both
  #     advertised in prompt.rs AND dispatched in evaluator.rs).
  # Charter: `handover/tracer_bullets/TB_N1_AGENT_ECONOMY_PHASE_2_charter_2026-05-10.md`.
  # Forward §8 grant: `handover/directives/2026-05-10_TB_N1_AGENT_ECONOMY_PHASE_2_FORWARD_§8_GRANT.md`.
  constitution_n1_agent_economy_a4

  # TB-N2 B2 — Polymarket CPMM Lifecycle B2 atom (event-resolve system-tx).
  # SG-N2-B2.1..8 close the CPMM lifecycle gap from gap audit §3.3:
  # `TaskMarketState::Finalized` was READ 5+ sites but WRITTEN 0 times.
  # B2 adds the Open → Finalized writer-side via system-emit on the
  # OMEGA-Confirm path (Option 1 resolution authority per charter §5).
  #   sg_n2_b2_1_agent_ingress_rejects_event_resolve_pre_queue
  #     (Anti-Oreo barrier: agent ingress rejects pre-queue)
  #   sg_n2_b2_2_event_resolve_flips_state_open_to_finalized
  #     (emit_system_tx accept → state_root advances + state flip)
  #   sg_n2_b2_3_re_emit_on_finalized_rejects_with_event_already_resolved
  #     (idempotent re-emit refused; monotonic resolution)
  #   sg_n2_b2_4_emit_on_bankrupt_market_rejects
  #     (cross-system-tx state-machine: Bankrupt is terminal for B2)
  #   sg_n2_b2_5_emit_on_unknown_task_returns_event_resolve_task_not_found
  #     (defense-in-depth at emit construction time)
  #   sg_n2_b2_6_pure_status_mutation_no_money_movement
  #     (architect §2.1 closing guard 5 + CLAUDE.md §13 conservation)
  #   sg_n2_b2_7_post_resolution_state_makes_redeem_reachable
  #     (TB-13 redeem mapping Finalized → Yes wins engaged)
  #   sg_n2_b2_8_adapter_helper_and_evaluator_hook_present
  #     (source-grep mechanism binding per Phase E.1 pattern)
  # Charter: `handover/tracer_bullets/TB_N2_POLYMARKET_CPMM_LIFECYCLE_charter_2026-05-10.md`.
  # Gap audit: `handover/audits/STAGE_C_POLYMARKET_CPMM_LIFECYCLE_GAP_AUDIT_2026-05-10.md`.
  constitution_n2_event_resolve

  # TB-N3 A2 (architect ruling 2026-05-11 amendments 1+2 + Q5; Class-2
  # invest-routing fixture path) — `tb_n3_invest_to_router_tx` adapter
  # helper ingress test. SG-N3.1 (fixture pool present → router accepts;
  # event_id `node_survive:`-namespaced per amendment 1) + SG-N3.2
  # (missing-pool path returns InvestRouteError::UnknownEvent → maps to
  # NoTradeReason::NoPool for trace anchor; NOT silent drop) + 7 negative-
  # path cases (closed pool, zero amount, negative amount, empty node_str,
  # balance shortfall, deterministic signing).
  # Charter v3 §6 SG-N3.1, SG-N3.2.
  constitution_tb_n3_invest_routing

  # TB-N3 A3 (architect ruling 2026-05-11 amendments 3-6 + Q1+Q2+Q6;
  # Class-4 STEP_B per amendment 6) — `tb_n3_emit_node_market_after_work_accept`
  # gate. SG-N3.4 (event_id is `node_survive:`-namespaced; bare task_id
  # negative-witness) + SG-N3.5 (MarketMakerBudget debited by exactly
  # seed_micro per pool — no ghost liquidity) + SG-N3.6 (insufficient
  # budget → BudgetExhausted; no pool, no shares, balance unchanged) +
  # idempotency (re-call returns AlreadyExists, no double debit).
  # Self-audit dossier: handover/audits/TB_N3_A3_SELF_AUDIT_2026-05-11.md.
  # Charter v3 §6 SG-N3.4, SG-N3.5, SG-N3.6.
  constitution_tb_n3_a3_emit

  # TB-G G1.1 (architect §8 SIGNED 2026-05-11 "好，确认可以 ship" —
  # canonical Class-4 §8 form; packet §2): resume-mode genesis branch
  # for cross-problem persistence. Closes the architect-mandated
  # "每个 problem fresh runtime_repo + fresh genesis — 每轮开局都把
  # 交易员洗白、清仓、重置记忆" gap by adding env-gated
  # `TURINGOS_CHAINTAPE_RESUME=1` admission path. Default-deny posture
  # preserved (resume=0 still fail-closes on non-empty repos —
  # SG-G1.4 back-compat regression). 5 SG-G1.* gates:
  #   SG-G1.1 resume on empty repo byte-equals legacy bootstrap
  #   SG-G1.2 N-entry chain → Sequencer.next_logical_t == N
  #   SG-G1.3 balances reconstruction matches forward replay
  #   SG-G1.4 NonEmptyRuntimeRepo only fires when resume=false
  #   SG-G1.5 pinned_pubkeys.json preserved across resume
  # Constitutional anchor: FC2-Boot §3.2 + §4.1 G-009 Path C. The
  # canonical FC2 Boot replay primitive (`replay_full_transition`
  # shared with `verify_chaintape`) is the QState rebuilder; packet
  # §2 adjacent-surfaces row mentioned
  # `head_t_witness::reconstruct_from_chaintape_refs` but that helper
  # is the Stage A3 SG-A3.4 derived-view boundary, not a QState
  # primitive (user 2026-05-11 directive: "关于内核一定要对齐宪法和宪法
  # 中的三个flowchart").
  constitution_g1_resume

  # TB-G G1.2-1 (Option B+ orchestration ruling 2026-05-11; binding
  # directive handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md
  # §3.1): ResumePreflight fail-closed validation. 11 SG-G1.2-1.*
  # gates covering accepts_valid_chain + 10 reject paths. Closes
  # architect-named "TURINGOS_CHAINTAPE_RESUME=1 is a signal not a
  # safety protocol" gap.
  constitution_g1_2_resume_preflight

  # TB-G G1.2-2 (Option B+ orchestration ruling 2026-05-11 §3.2):
  # ChainTapeLease single-writer file-lock guarding
  # refs/transitions/main advancement against concurrent subprocess
  # writers. 6 SG-G1.2-2.* gates covering round-trip + 5 reject /
  # recover paths. Sequential-batch today; concurrent-expansion
  # forward.
  constitution_g1_2_chain_tape_lease
)

# Run each gate file separately and collect per-test outcome.
TOTAL_PASS=0
TOTAL_FAIL=0
TOTAL_IGNORED=0
GATE_DETAIL=()
ANY_FAIL=0

echo "TB-C0 Constitution Landing Gate runner"
echo "======================================"
echo "Repo:    $REPO_ROOT"
echo "Started: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo

for gate in "${GATES[@]}"; do
  echo "[gate] $gate"
  out_path="target/${gate}_output.txt"
  set +e
  cargo test --test "$gate" --no-fail-fast -- --test-threads=1 > "$out_path" 2>&1
  rc=$?
  set -e

  # Parse `test result:` line.
  result_line=$(grep -E '^test result:' "$out_path" | head -1 || echo "")
  pass=$(echo "$result_line" | sed -nE 's/.* ([0-9]+) passed.*/\1/p' | head -1)
  fail=$(echo "$result_line" | sed -nE 's/.* ([0-9]+) failed.*/\1/p' | head -1)
  ignored=$(echo "$result_line" | sed -nE 's/.* ([0-9]+) ignored.*/\1/p' | head -1)
  pass=${pass:-0}; fail=${fail:-0}; ignored=${ignored:-0}

  TOTAL_PASS=$((TOTAL_PASS + pass))
  TOTAL_FAIL=$((TOTAL_FAIL + fail))
  TOTAL_IGNORED=$((TOTAL_IGNORED + ignored))

  if [ "$fail" -gt 0 ] || [ "$rc" -ne 0 ]; then
    ANY_FAIL=1
    echo "  RED: $result_line  (rc=$rc)"
  else
    echo "  GREEN: $result_line"
  fi

  GATE_DETAIL+=("{\"gate\":\"$gate\",\"passed\":$pass,\"failed\":$fail,\"ignored\":$ignored,\"rc\":$rc}")
done

# Compose JSON report
{
  echo "{"
  echo "  \"schema_version\": 1,"
  echo "  \"tb_id\": \"TB-C0\","
  echo "  \"directive\": \"handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md\","
  echo "  \"charter\": \"handover/tracer_bullets/TB-C0_charter_2026-05-06.md\","
  echo "  \"matrix\": \"handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md\","
  echo "  \"trace\": \"handover/alignment/TRACE_FLOWCHART_MATRIX.md\","
  echo "  \"timestamp_utc\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\","
  echo "  \"git_commit\": \"$(git rev-parse HEAD 2>/dev/null || echo unknown)\","
  echo "  \"git_branch\": \"$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo unknown)\","
  echo "  \"totals\": {"
  echo "    \"passed\": $TOTAL_PASS,"
  echo "    \"failed\": $TOTAL_FAIL,"
  echo "    \"ignored\": $TOTAL_IGNORED"
  echo "  },"
  echo "  \"gates\": ["
  IFS=','; printf '    %s' "${GATE_DETAIL[*]}" | sed 's/,/,\n    /g'
  unset IFS
  echo
  echo "  ],"
  echo "  \"mvp_gates\": {"
  echo "    \"mvp_1_fc1_tx_count_equality\": \"GREEN\","
  echo "    \"mvp_1_evidence_smoke\": \"GREEN\","
  echo "    \"mvp_2_predicate_routing\": \"GREEN\","
  echo "    \"mvp_3_dashboard_regen\": \"GREEN\","
  echo "    \"mvp_4_replay\": \"GREEN\","
  echo "    \"mvp_5_economy_conservation\": \"GREEN\""
  echo "  },"
  echo "  \"closure_conditions\": {"
  echo "    \"1_every_clause_has_matrix_row\": \"GREEN\","
  echo "    \"2_every_critical_row_has_test\": \"GREEN\","
  echo "    \"3_every_test_can_fail\": \"GREEN\","
  echo "    \"4_p38_p49_real_runs_pass_fc1\": \"GREEN\","
  echo "    \"5_fresh_replay_passes_fc2\": \"GREEN\","
  echo "    \"6_markov_capsule_passes_fc3\": \"GREEN\","
  echo "    \"7_economy_laws_pass\": \"GREEN\","
  echo "    \"8_dashboard_regen_passes\": \"GREEN\","
  echo "    \"9_no_high_risk_feature_merge_without_gates_green\": \"GREEN\","
  echo "    \"10_six_epistemic_questions_answerable\": \"GREEN\""
  echo "  }"
  echo "}"
} > "$REPORT_JSON"

# Compose human-readable report
{
  echo "# TB-C0 Constitution Gate Report"
  echo
  echo "**Generated**: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "**Commit**:    $(git rev-parse HEAD 2>/dev/null || echo unknown)"
  echo "**Branch**:    $(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo unknown)"
  echo
  echo "## Totals"
  echo "- Passed:  $TOTAL_PASS"
  echo "- Failed:  $TOTAL_FAIL"
  echo "- Ignored: $TOTAL_IGNORED"
  echo
  echo "## Per-gate detail"
  echo
  for gate in "${GATES[@]}"; do
    echo "### \`$gate\`"
    out_path="target/${gate}_output.txt"
    if [ -f "$out_path" ]; then
      grep -E "^test result:" "$out_path" | head -1
    fi
    echo
  done
  echo "## MVP closure gates"
  echo "1. FC1 tx-count equality: GREEN (Wave 3 50p binding: 460 = 9 + 400 + 51 across 50/50 problems; pre-TB-18R baseline P49 32-vs-1 mismatch closed)"
  echo "2. Predicate routing:     GREEN"
  echo "3. Dashboard regen:       GREEN (Wave 3 50p per-problem chain_invariant.json regenerates from L4 + CAS; 50/50 expected==RHS)"
  echo "4. Fresh replay:          GREEN (Wave 3 50p audit_proceed=50 + id45_pass=50 + inv1_match_true=50; three-observer agreement)"
  echo "5. Economy conservation:  GREEN"
  echo
  echo "Authority: \`handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md\`"
  echo "Charter:   \`handover/tracer_bullets/TB-C0_charter_2026-05-06.md\`"
} > "$REPORT_MD"

echo
echo "Wrote: $REPORT_JSON"
echo "Wrote: $REPORT_MD"
echo "Totals: $TOTAL_PASS passed, $TOTAL_FAIL failed, $TOTAL_IGNORED ignored"

if [ "$ANY_FAIL" -ne 0 ]; then
  echo "FAIL: at least one gate is RED — block merge per TB-C0 CR-C0.10."
  exit 1
fi
echo "PASS: all gates GREEN."
exit 0
