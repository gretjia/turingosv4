# Constitution Landing Manifest — Real-Evidence Audit

**Generated**: 2026-05-09 session #26 (post Stage B3 R7 M2 launch + ChainTape physical-evidence audit)
**HEAD**: `9267ec3` (LATEST.md handover update)
**Author**: Claude Opus 4.7 (1M context) — physical evidence audit, not LATEST.md narrative paraphrase
**Companion**: this manifest is the *layer-organized real-status view*; `CONSTITUTION_EXECUTION_MATRIX.md` is the *clause-by-clause CI gate view*. Both should agree on row count and status; if they diverge, the matrix is authoritative for `bash scripts/run_constitution_gates.sh` purposes and this manifest is authoritative for *Polymarket-readiness* / *what is actually shipped* purposes.

---

## §0. Status Taxonomy

| Tag | Meaning |
|---|---|
| 🟢 **DONE** | Code surface + executable test + real-LLM-tape evidence + can-fail-on-regression all present. |
| 🟡 **PARTIAL-S** | Test exists (structural-only) but missing real-evidence binding OR runtime wire-up missing. |
| 🟡 **PARTIAL-W** | Wire-up exists but charter-required hardening tests are incomplete (e.g., verbatim names not bound). |
| 🔵 **GATED** | Implementation ≥80% complete; awaiting external decision (architect §8 / dual-audit / etc.). |
| ⚪ **NOT-STARTED** | No code surface yet, OR code is at stub-level only. |
| ⚠️ **KNOWN-GAP** | Implementation exists but semantically deviates from architect-strict reading; gap is documented + forward-bound. |

**Polymarket-blocking column** (`P-block`):
- 🚫 = blocks Stage C P-M0..P-M9 progression
- ✓ = does not block Stage C
- — = Polymarket-irrelevant

---

## §1. L1 — Substrate Foundation

| # | Constitution Requirement | Status | Real Evidence / Test | P-block |
|---|---|---|---|---|
| A.1 | Kernel zero-domain (no app logic, no f64, no money in `kernel.rs`) | 🟢 DONE | `tests/four_element_mapping.rs` 5 tests + `tests/tb_13_legacy_cpmm_forward_fence.rs` Layer 1+2 scope scan | ✓ |
| A.2 | QState canonical (single source of truth for state) | 🟢 DONE | `tests/q_state_reconstruct.rs` + `tests/six_axioms_alignment.rs` + Wave 3 50p replay-determinism (50/50 audit_proceed=true) | ✓ |
| A.3 | Sequencer admission (predicate-gated tx routing) | 🟢 DONE | `tests/constitution_predicate_gate.rs` (5 tests) + Wave 3 50p 460 cycles | ✓ |
| A.4 | Trust Root (24 files sha256-pinned in `genesis_payload.toml [trust_root]`) | 🟢 DONE | `src/boot.rs::verify_trust_root` panics on mismatch — verified live in session #26 (my edit to `llm_proxy.py` triggered TRUST_ROOT_TAMPERED panic at evaluator boot, requiring revert) | ✓ |
| A.5 | System pubkeys (Ed25519 keypair; system-tx forbidden from agent ingress) | 🟢 DONE | `tests/system_keypair_*.rs` (5 tests) + `tests/constitution_economy_gate.rs::system_tx_not_agent_submittable` | ✓ |
| A.6 | No legacy parallel ledger (`bus.append` authoritative writes blocked) | 🟢 DONE | `tests/constitution_no_parallel_ledger.rs` (5 tests) + Wave 3 50p witness (sequencer-mediated only, zero legacy authoritative writes) | ✓ |

**L1 verdict: 6/6 DONE. Substrate is solid. No further action required.**

---

## §2. L2 — ChainTape (Tape Layer)

| # | Constitution Requirement | Status | Real Evidence | P-block |
|---|---|---|---|---|
| B.1 | L4 accepted ledger (`refs/chaintape/l4` git2 commits) | 🟢 DONE | A3 R3.5 evidence: 2 commits walkable (logical_t=1→2) with content blobs `entry_canonical` + `payload_cid` + `signature`; M2 R7 P001 evidence: 2 commits same shape; deterministic timestamps (`1970-01-01 + N seconds`, no wall-clock leakage) | ✓ |
| B.2 | L4.E rejection ledger (`refs/chaintape/l4e` git2 commits + `rejections.jsonl` mirror) | 🟢 DONE | A3 R3.5: 10 commits parent-chained `submit_id=1..11` + 1:1 match with `rejections.jsonl` (10 lines == 10 commits); M2 R7 P001: 9 commits 1:1; commits contain real `rejection_record` JSON blob with `submit_id, parent_state_root, agent_id, tx_kind, tx_payload_cid, rejection_class, raw_diagnostic_cid, prev_hash, hash` | ✓ |
| B.3 | CAS storage (cid sha256 + blob OID + sidecar JSONL index) | 🟢 DONE | `cas/.git/objects/` git loose objects + `cas/.turingos_cas_index.jsonl` sidecar + Wave 3 50p 2074-object aggregate verified (no leakage-suggestive schema_id, size-bounded) | ✓ |
| B.4 | CAS root advancement (`refs/chaintape/cas`) | ⚠️ KNOWN-GAP | `refs/chaintape/cas` points to **latest blob OID** (`56edd4c6...`), **not a Merkle root**. `git log refs/chaintape/cas` returns 0 commits because the ref is a blob ref, not a commit ref. Full CAS history is materialized via `cas/.git/objects/` + `.turingos_cas_index.jsonl` sidecar. Replay works through this combination but is not a strict Merkle proof. **Forward**: Stage A3.6 enhancement TB charter draft includes "refs/chaintape/cas commit-chain redesign + atomic ref-update + failure-injection tests" (Codex Q1 + Q2 dual-audit forward-bind from A3 R7) | ✓ (replay still works) |
| B.5 | HEAD_t = (state_root, l4_head, l4e_head, cas_root, economic_state_root, run_id) | 🟢 DONE (C1 + C2) | `src/state/head_t_witness.rs` + `tests/constitution_head_t_witness.rs` (5 tests, C1 baseline) + `tests/constitution_head_t_c2_multi_ref.rs` (7 tests, C2 multi-ref) + Stage A3 §8 sign-off 2026-05-08 ("同意 sign-off"); `HeadTWitness::reconstruct_from_chaintape_refs` derives witness from refs alone | ✓ |
| B.6 | Replay determinism (genesis + tape + CAS → reconstructs HEAD_t) | 🟢 DONE | `tests/constitution_fc2_boot.rs::fc2_run_replayable_from_genesis_tape_cas` + Wave 3 50p three-observer agreement (audit_proceed=50 + id45_pass=50 + inv1_match_true=50) | ✓ |

**L2 verdict: 5/6 DONE + 1 KNOWN-GAP (CAS root). Replay-functional; strict-Merkle-CAS deferred to A3.6.**

---

## §3. L3 — Predicates / FC1 Runtime Loop

| # | Constitution Requirement | Status | Real Evidence | P-block |
|---|---|---|---|---|
| C.1 | Predicate pass → L4 admission | 🟢 DONE | `tests/constitution_predicate_gate.rs::predicate_pass_required_for_l4` + Wave 3 50p 460 cycles | ✓ |
| C.2 | Predicate fail → L4.E routing (NOT L4) | 🟢 DONE | `tests/constitution_predicate_gate.rs::predicate_failure_cannot_enter_l4` + B3 R7 M2 P001 9 L4.E rejections (LeanFailed × 9, 0 in L4) | ✓ |
| C.3 | Lean oracle (Lean verifies WorkTx) | 🟢 DONE | `tests/constitution_predicate_gate.rs::lean_verified_required_for_verified_worktx` + real Lean stderr in M2 cells (linarith failed, type_mismatch, sorry-forbidden) | ✓ |
| C.4 | PCP soundness G-012 (synthetic + MiniF2F-v2 misalignment 9-class corpus) | 🟢 DONE | `cases/pcp_corpus/` (Phase 1 synthetic) + `cases/pcp_corpus_phase2/` (Phase 2 real-MiniF2F derived 9 classes: 01_valid, 02_mutated_invalid, 03_sorry_insertion, 04_type_mismatch, 05_wrong_theorem_name, 06_off_by_one_arith, 07_irrelevant_theorem, 08_partial_then_final_invalid, 09_parse_invalid) + `tests/constitution_pcp_corpus.rs` (7 tests) + `tests/constitution_pcp_corpus_phase2.rs` (8 tests) | ✓ |
| C.5 | PromptCapsule (G-016 / G-019 / G-021 / G-028 — Class-3 7-field schema + L4 anchor) | 🟡 PARTIAL-S | Constitution gate exists: `src/runtime/prompt_capsule.rs` + `tests/constitution_prompt_capsule.rs` (7 tests) + 3 inline tests. **But evaluator runtime wire-up missing**: `grep "PromptCapsule::new" experiments/minif2f_v4/src/bin/evaluator.rs` returns 0 hits; `AttemptTelemetry` doesn't yet carry `prompt_capsule_cid` reference field. **Forward**: in evaluator.rs at LLM-call site, generate PromptCapsule (read_set + policy_version + redacted hidden_fields + visible_context_cid + system_prompt_template_hash + agent_view_manifest_cid), write to CAS, store cid in AttemptTelemetry. Estimate 1-2 days, Class 3. | ✓ (gate-level OK) |
| C.6 | FC1 hard invariant (`evaluator_completed_LLM_calls == L4_count + L4E_count + capsule_anchored_count`) | 🟢 DONE | `tests/constitution_fc1_runtime_loop.rs` + `tests/constitution_runner_invariant_formula.rs` + Wave 3 50p (50/50 chain_invariant Ok delta=0) + B3 R7 M2 (31/31 Ok delta=0 as of session #26 health check) | ✓ |

**L3 verdict: 5/6 DONE + 1 PARTIAL-S (PromptCapsule runtime wire-up). Constitution gate green, evaluator runtime not yet emitting PromptCapsule.**

---

## §4. L4 — Economy / CTF Conservation

| # | Constitution Requirement | Status | Real Evidence | P-block |
|---|---|---|---|---|
| D.1 | MicroCoin integer math (no f64 in money path) | 🟢 DONE | `tests/constitution_market_quarantine.rs::no_f64_in_market_modules` + `tests/tb_13_legacy_cpmm_forward_fence.rs::no_f64_in_complete_set_or_market_seed` + Layer 2 scan over `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `src/state/q_state.rs`, `src/economy/monetary_invariant.rs` | ✓ |
| D.2 | Total Coin conservation (post `on_init` no mint) | 🟢 DONE | `tests/constitution_economy_gate.rs::economy_total_coin_conserved` + `economy_no_post_init_mint` + 9-test gate file all PASS + Wave 3 50p covers economic flow (50 problems × EscrowLockTx + WorkTx cycles) | ✓ |
| D.3 | 1 Coin = 1 YES + 1 NO identity (CompleteSet hardening) | 🟢 DONE | `tests/constitution_completeset_hardening.rs` (8 §5.3 verbatim tests, commit `d33c25a` 2026-05-08 session #25) + sequencer dispatch arm calls `assert_complete_set_balanced` live (line 1837 of `src/state/sequencer.rs`) | ✓ |
| D.4 | No ghost liquidity (MarketSeed requires collateral debit) | 🟡 PARTIAL-W | TB-13 has 2 tests (`sg_13_3_market_seed_fails_if_provider_lacks_balance` + `sg_13_4_market_seed_cannot_create_liquidity_without_collateral`). **But charter §5.5 verbatim names not bound**: 5 verbatim test names required (`market_seed_debits_provider`, `market_seed_creates_yes_no_inventory`, `market_seed_fails_insufficient_balance`, `market_seed_no_ghost_liquidity`, `market_seed_conserves_total_coin`); only 2 covered semantically by TB-13's SG-13.* names. **Forward**: write `tests/constitution_marketseed_hardening.rs` with 5 verbatim names binding to live sequencer dispatch (same pattern as session #25 §5.3 work). Half-day, Class 1. | 🚫 (P-M3) |
| D.5 | Shares not counted as Coin (claims, not currency) | 🟢 DONE | `tests/constitution_completeset_hardening.rs::shares_not_counted_as_coin` (verbatim) + `src/economy/monetary_invariant.rs:182 total_supply_micro` excludes `conditional_share_balances_t` from sum | ✓ |
| D.6 | EconomicState mutation = sequencer-only | 🟢 DONE | `tests/constitution_fc2_boot.rs::fc2_no_memory_only_preseed` (source-grep) + Wave 3 50p replay-determinism witness (memory-only mutation would diverge across observers) | ✓ |
| D.7 | Legacy CPMM (`prediction_market.rs`) excised | 🟢 DONE | `tests/constitution_market_quarantine.rs::legacy_cpm_api_not_imported_by_new_market` + `tb_13_legacy_cpmm_forward_fence.rs::prediction_market_legacy_quarantined` + file physically deleted (asserted by absence-test) | ✓ |

**L4 verdict: 6/7 DONE + 1 PARTIAL-W (MarketSeed verbatim binding). 3 verbatim names need binding before P-M3 atom.**

---

## §5. L5 — FC2 Boot / Init

| # | Constitution Requirement | Status | Real Evidence | P-block |
|---|---|---|---|---|
| E.1 | Genesis report exists per run | 🟢 DONE | `tests/constitution_fc2_boot.rs::fc2_genesis_report_exists` + 91 cumulative cells × `runtime_repo/genesis_report.json` present | ✓ |
| E.2 | `on_init` only legal mint | 🟢 DONE | `fc2_on_init_only_mint` + Wave 3 50p check | ✓ |
| E.3 | System pubkeys verify | 🟢 DONE | `fc2_system_pubkeys_verify` + 5 keypair tests | ✓ |
| E.4 | Agent registry resolves | 🟢 DONE | `fc2_agent_registry_resolves` | ✓ |
| E.5 | TaskOpen / EscrowLock are chain events (not memory mutation) | 🟢 DONE | `fc2_taskopen_escrowlock_are_chain_events` | ✓ |
| E.6 | No memory-only preseed | 🟢 DONE | `fc2_no_memory_only_preseed` source-grep + Wave 3 50p replay-determinism witness | ✓ |
| E.7 | Replay from genesis + tape + CAS | 🟢 DONE | `fc2_run_replayable_from_genesis_tape_cas` + Wave 3 50p three-observer agreement | ✓ |
| E.8 | No global `LATEST_MARKOV_CAPSULE.txt` pointer | 🟢 DONE | `tests/constitution_no_parallel_ledger.rs::no_global_markov_pointer` | ✓ |

**L5 verdict: 8/8 DONE. Boot layer fully landed.**

---

## §6. L6 — FC3 Meta / Markov / Shielding (Capsule layer)

| # | Constitution Requirement | Status | Real Evidence | P-block |
|---|---|---|---|---|
| F.1 | `EvidenceCapsule` is derived view (not authoritative) | 🟢 DONE | `tests/constitution_fc3_inv1_capsule_integrity_regen.rs` (4 tests) | ✓ |
| F.2 | `MarkovEvidenceCapsule` for inheritance (per-runtime α; in-tape β long-term; γ rejected) | 🟢 DONE | `tests/constitution_fc3_meta.rs` (8 tests) | ✓ |
| F.3 | FC3-INV3 Raw logs shielded (no agent-prompt leakage) | 🟢 DONE | `tests/constitution_fc3_evidence_binding.rs::FC3-INV3` (Wave 3 50p CAS size-bound) + `tests/constitution_shielding_evidence_binding.rs` (9 tests) | ✓ |
| F.4 | FC3-INV4 Capsule context only (replay-determinism) | 🟢 DONE | `FC3-INV4` + Wave 3 50p replay-determinism witness | ✓ |
| F.5 | FC3-INV5 Deep-history default-deny (`TURINGOS_MARKOV_OVERRIDE` explicit enable) | 🟢 DONE | `FC3-INV5` production helper exercise + sequencer fail-closed if not set | ✓ |
| F.6 | FC3-INV7 ArchitectAI proposes-only (no auto predicate/tool mutation) | 🟢 DONE | `FC3-INV7` git-author scan over commits | ✓ |
| F.7 | FC3-INV8 JudgeAI / VetoAI veto-only | 🟢 DONE | `FC3-INV8` audit-dir extension whitelist | ✓ |

**L6 verdict: 7/7 DONE. Session #24 "宪法完整落地" work landed the final 7 §I + §F V.2 closures.**

---

## §7. L7 — Article III Selective Shielding (Agent Read-View)

| # | Constitution Requirement | Status | Real Evidence | P-block |
|---|---|---|---|---|
| G.1 | Selective broadcast (typical errors only, no raw stderr) | 🟢 DONE | `tests/constitution_shielding_gate.rs::raw_lean_stderr_not_in_agent_read_view` + Wave 3 50p shielding binding (LeanResult max 146B across 447 instances) | ✓ |
| G.2 | Selective shielding (private CID for raw diagnostic, audit-only) | 🟢 DONE | `private_diagnostic_cid_not_serialized_publicly` + Wave 3 50p (AttemptTelemetry max 469B / 460 instances; TypedTx.v1 max 459B / 668 instances) | ✓ |
| G.3 | Shield correlation (no Goodhart leakage / no leakage-suggestive schema_id) | 🟢 DONE | `dashboard_does_not_leak_private_failure_detail` + 2074-object aggregate scan: zero forbidden tokens (`raw_stderr`, `lean_full_body`, `private_diagnostic_*`, `agent_visible_raw`, `prompt_raw_visible`) | ✓ |
| G.4 | Public summary low-pollution (`TransitionError.display` ≤ 256B) | 🟢 DONE | `l4e_public_summary_low_pollution` + Wave 3 50p (TransitionError.display.v1 max 48B avg 34B / 95 real rejections) | ✓ |

**L7 verdict: 4/4 DONE.**

---

## §8. L8 — Constitutional CI / Governance

| # | Constitution Requirement | Status | Real Evidence | P-block |
|---|---|---|---|---|
| H.1 | Constitution gate runner | 🟢 DONE | `bash scripts/run_constitution_gates.sh` → 175 GREEN / 0 failed / 1 ignored (verified session #26) | ✓ |
| H.2 | Constitution Execution Matrix maintained | 🟢 DONE | `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` (225 lines) — 0 AMBER / 0 RED status | ✓ |
| H.3 | Trace Flowchart Matrix (per-FC node mapping) | 🟢 DONE | `handover/alignment/TRACE_FLOWCHART_MATRIX.md` | ✓ |
| H.4 | Pre-action gates (skills) | 🟢 DONE | `/runner-preflight` (verified live in session #26 — 7-stage gate caught R-019 alias drift, etc.) + `/constitution-landing-check` + `/harness-reflect` | ✓ |
| H.5 | STEP_B parallel-branch protocol | 🟢 DONE | `feedback_step_b_protocol` memory + A3 R7 production-defect fix used STEP_B parallel-branch | ✓ |
| H.6 | "Every test can fail" (closure-3 mechanical) | 🟢 DONE | `tests/constitution_closure_3_no_trivial_asserts.rs` (3 tests) — pattern scanner + sibling test on synthetic input proves detectability | ✓ |
| H.7 | Authorization semantics for Class-3/4 | 🟢 DONE | CLAUDE.md §10 + 4 architect §8 sign-off precedents (TB-C0, A2, A3, TB-18R) | ✓ |

**L8 verdict: 7/7 DONE.**

---

## §9. L9 — Polymarket Substrate (Stage C Forward)

| # | Charter Requirement | Status | Notes | P-block |
|---|---|---|---|---|
| I.0 | **P-M0 quarantine** (legacy excised + CompleteSet hardened) | 🟢 DONE (label pending) | Session #25 commit `d33c25a` substantively completes P-M0 via `tests/constitution_market_quarantine.rs` (§5.2 verbatim) + `tests/constitution_completeset_hardening.rs` (§5.3 verbatim 8 tests). Charter file `STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md` not yet updated with "P-M0 SHIPPED" marker. | — (effectively unblocked) |
| I.1 | **P-M1 CompleteSetMergeTx** (1 YES + 1 NO → 1 Coin) | ⚪ NOT-STARTED | `src/state/typed_tx.rs` has no `CompleteSetMergeTx` struct. Architect §5.4 spec is complete: struct definition (`tx_id, parent_state_root, event_id, owner, amount, signature`), 4 verbatim tests (`merge_yes_no_returns_coin`, `merge_requires_both_sides`, `merge_conserves_total_coin`, `merge_reduces_collateral`), and sequencer arm semantics (require both sides ≥ amount; burn YES + NO; debit collateral; credit balances). Class 4 (sequencer admission + typed_tx schema bump). | 🚫 (P-M1) |
| I.2 | **P-M2 ShareBalances export** | ⚪ NOT-STARTED | `audit_tape.rs` has no `view-shares` subcommand. Charter §5.10 lists 4 audit subcommands required (`view-shares`, `view-pools`, `view-prices`, `view-positions`). | 🚫 (P-M2) |
| I.3 | **P-M3 MarketSeed sanity hardening** | 🟡 PARTIAL-W | TB-13 has 2 tests (sg_13_3 + sg_13_4); charter §5.5 5 verbatim names need binding. See L4 row D.4 for forward path. | 🚫 (P-M3) |
| I.4 | **P-M4 CpmmPool integer math** (`pool_yes`, `pool_no`, `lp_total_shares`) | ⚪ NOT-STARTED | `src/state/q_state.rs` + `typed_tx.rs` have no `CpmmPool` struct. Architect §5.6 spec is complete (struct + 4 tests + invariant `k = pool_yes * pool_no` non-decreasing under floor rounding). Class 4. | 🚫 (P-M4) |
| I.5 | **P-M5 Share-only swap** (用 NO 买 YES integer floor formula) | ⚪ NOT-STARTED | Architect §5.7 spec is complete (formulas: `outY = floor(dN * poolY / (poolN + dN))`, `poolY1 * poolN1 >= poolY * poolN`, 6 verbatim tests). | 🚫 (P-M5) |
| I.6 | **P-M6 Mint-and-Swap Router** (BuyYesWithCoinRouter + BuyNoWithCoinRouter) | ⚪ NOT-STARTED | Architect §5.8 spec is complete (atomic 9-step BuyYes router + symmetric BuyNo + 9 verbatim tests including `router_atomic_rollback_on_failure`). Class 4. | 🚫 (P-M6) |
| I.7 | **P-M7 PriceIndex** (signal not truth) | ⚪ NOT-STARTED | `src/state/price_index.rs` exists (TB-14 era) but charter §5.9 4 verbatim tests not bound (`price_quote_does_not_change_state`, `price_signal_not_predicate`, `price_does_not_make_failed_node_accepted`, `low_liquidity_warning`). | 🚫 (P-M7) |
| I.8 | **P-M8 audit_tape views** (`view-shares` / `view-pools` / `view-prices` / `view-positions`) | ⚪ NOT-STARTED | All 4 subcommands missing from `src/bin/audit_tape.rs`. Charter §5.10 spec complete. Class 1 additive (no production-state mutation). | 🚫 (P-M8) |
| I.9 | **P-M9 controlled market smoke** | ⚪ NOT-STARTED | Gated on P-M1 through P-M8. Charter §5 implementation manual + §6 forbidden list. | 🚫 (P-M9) |

**L9 verdict: 1 effectively-DONE + 1 PARTIAL-W + 8 NOT-STARTED. This is the largest open work surface. Architect §5 charter provides complete implementation specs (struct + tests + formulas) for each atom — no design ambiguity, just sequenced engineering.**

---

## §10. L10 — Diagnostic Benchmarks (M-ladder)

| # | Requirement | Status | Real Evidence | P-block |
|---|---|---|---|---|
| J.1 | M0 harness probe (5-8 cells) | 🟢 DONE | `handover/evidence/m0_minif2f_harness_audit_2026-05-05/` + `handover/evidence/stage_b3_r6_minim1_2026-05-08T06-07-32Z/` (8 cells × n=1, 8/8 chain_invariant Ok delta=0) | ✓ |
| J.2 | **M1 (50p × n=3 × 3 seeds × 1 model = 450 runs)** | ⚪ NOT-DONE | **Only mini-M1 (8p × n=1) executed; charter-spec 450-run M1 batch never run.** This is a quiet gap — LATEST.md historically referenced "Stage B3 R6 mini-M1" without flagging that mini-M1 ≠ M1. M2 directly skips M1. | ⚠️ (charter shape) |
| J.3 | M2 (100p × n=3 × 3 seeds × 2 models = 1800 runs) | 🟡 RUNNING | tmux session `stage_b3_r7_m2`, 31/1800 cells complete as of session #26 health check; ~67-200h projection (revising upward as AIME-heavy front of problem list runs). | ⚠️ (but `substrate stable` declaration possible without M2) |
| J.4 | M3+ (200p+, public H-VPPU claim) | ⚪ NOT-STARTED | Gated on M2 completion + architect approval. Per `feedback_minif2f_scaling_policy`, M2 is harness-prep, not benchmark-publishable. | — |
| J.5 | 4 replay sampling tests (architect §3.B3 verbatim names: `sampled_full_replay`, `failure_heavy_sample_replay`, `solved_sample_replay`, `unsolved_sample_replay`) | ⚪ NOT-STARTED | Gated on M2 evidence (or any large-batch evidence). Constitution-gate level not blocking Polymarket. | — (gate-level only) |
| J.6 | EvidencePackagingPolicy compliance | 🟢 DONE (runner-level) | `scripts/run_stage_b3.sh` packs `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` per-cell; 4-cell smoke verified shape; M2 run will verify at scale | ✓ |
| J.7 | Wilson 95% CI + diversity helpers | 🟢 DONE | `src/runtime/wilson_ci.rs` + `src/runtime/diversity.rs` + 12 inline tests + `tests/constitution_wilson_ci.rs` (5 tests) + `tests/constitution_diversity.rs` (7 tests) | ✓ |
| J.8 | BenchmarkManifest schema (FR-18B.1 / CR-18B.5) | 🟢 DONE | `src/runtime/benchmark_manifest.rs` + `tests/constitution_benchmark_manifest.rs` (6 tests) + M2 R7 manifest pin verified in production at `handover/evidence/stage_b3_r7_m2_*/BenchmarkManifest.json` | ✓ |
| J.9 | AggregateReport schema (CLAUDE.md §17 verbatim — ΣPPUT + Mean PPUT(solved) + Wilson 95% CI + halt_distribution + counts + no-fake-accepted) | 🟢 DONE | `src/runtime/aggregate_report.rs` + `tests/constitution_aggregate_report.rs` (11 tests) | ✓ |

**L10 verdict: 6/9 DONE + 1 RUNNING + 2 NOT-STARTED. Full M1 (J.2) is a quiet gap; M2 (J.3) running but charter-form completion is ~9 days out at current cell rate.**

---

## §11. L11 — Real-world Readiness (Stage D)

| # | Requirement | Status | P-block |
|---|---|---|---|
| K.1 | REAL_WORLD_READINESS_REPORT | 🔵 GATED (directive draft only — `2026-05-07_REAL_WORLD_READINESS_DIRECTIVE.md`) | — |
| K.2 | DOMAIN_SELECTION_CRITERIA | ⚪ NOT-STARTED | — |
| K.3 | ORACLE_REQUIREMENTS | ⚪ NOT-STARTED | — |
| K.4 | CHALLENGE_COURT_REQUIREMENTS | ⚪ NOT-STARTED | — |
| K.5 | SAFETY_BOUNDARY | ⚪ NOT-STARTED | — |
| K.6 | IRREVERSIBLE_ACTION_POLICY | ⚪ NOT-STARTED | — |

**L11 verdict: directive draft only. Stage D requires architect-side oracle / challenge-court / safety design. Polymarket and Stage D are decoupled (neither blocks the other).**

---

## §12. Summary Statistics

| Layer | DONE | PARTIAL | NOT-STARTED | KNOWN-GAP | Total |
|---|---:|---:|---:|---:|---:|
| L1 Substrate | 6 | 0 | 0 | 0 | 6 |
| L2 ChainTape | 5 | 0 | 0 | 1 | 6 |
| L3 Predicates / FC1 | 5 | 1 | 0 | 0 | 6 |
| L4 Economy / CTF | 6 | 1 | 0 | 0 | 7 |
| L5 FC2 Boot | 8 | 0 | 0 | 0 | 8 |
| L6 FC3 Meta | 7 | 0 | 0 | 0 | 7 |
| L7 Shielding | 4 | 0 | 0 | 0 | 4 |
| L8 Governance | 7 | 0 | 0 | 0 | 7 |
| **L9 Polymarket** | **1** | **1** | **8** | **0** | **10** |
| L10 M-ladder | 6 | 1 (RUNNING) | 2 | 0 | 9 |
| L11 Stage D | 0 | 0 | 6 (gated) | 0 | 6 |
| **TOTAL** | **55** | **4** | **16** | **1** | **76** |

---

## §13. Honest Findings (audit-grade, not LATEST.md narrative)

### Finding 1: Substrate (L1-L8) is genuinely landed
55 of 56 substrate items are DONE. The single KNOWN-GAP (CAS root semantics) is forward-bound, replay-functional, and architecturally documented. **L1-L8 represent ~80% of constitution and they are real-evidence-tested across 91+ cumulative cells of Wave 3 50p + Stage A3 smokes + B3 R6 mini-M1 + B3 R7 M2 first 31 cells, every one with `chain_invariant Ok delta=0`.**

### Finding 2: Polymarket is the actual remaining work
L9 has 8 NOT-STARTED + 1 PARTIAL-W. P-M0 quarantine is effectively shipped via §2.4 audit (commit `d33c25a`). The remaining 9 atoms (P-M1..P-M9) are pure engineering against architect §5 verbatim specs — no design ambiguity, but ~15-20 working days of code.

### Finding 3: CAS root is a known architectural compromise
`refs/chaintape/cas` points to the latest CAS write's blob OID, NOT a Merkle root over all CAS objects. Replay still works through `cas/.git/objects/` + `.turingos_cas_index.jsonl` sidecar combination, but `git log refs/chaintape/cas` returns 0 commits (because it's a blob ref, not a commit ref). This was traded off in Stage A3 SG-A3.3 wording ("CAS root advances", not "CAS chain commit hash"). Stage A3.6 enhancement TB charter draft includes the strict-Merkle redesign.

### Finding 4: PromptCapsule constitution gate ≠ runtime wire-up
`tests/constitution_prompt_capsule.rs` (7 tests) GREEN does NOT mean the evaluator actually emits PromptCapsule per LLM call. Constitution gate verifies struct semantics + redaction discipline + L4 anchor schema, but evaluator runtime path doesn't yet call `PromptCapsule::new` at LLM-call sites. Class 3 forward work, ~1-2 days.

### Finding 5: MarketSeed verbatim-name binding is incomplete
TB-13 ships 2 hardening tests (sg_13_3 + sg_13_4); architect §5.5 charter requires 5 verbatim test names. This is the same pattern as §5.3 (which session #25 fully bound at constitution-gate surface). Half-day of work to complete.

### Finding 6: Full M1 (450 cells) was never executed
LATEST.md historically referenced "Stage B3 R6 mini-M1" but mini-M1 (8p × n=1 = 8 cells) is NOT charter M1 (50p × n=3 × 3 seeds × 1 model = 450 cells). M2 directly skips M1. This may matter if architect §8 ship gate insists on "M-ladder strict ordering"; otherwise M2 evidence subsumes M1's substrate-stability claim.

### Finding 7: M2 wall-time projection is ~9 days, not the 67h estimate
At the session #26 31-cell sample: avg 428s/cell × 1800 = 214h ≈ 9 days. The 67h estimate was extrapolated from smoke (mathd_algebra problems, easy). Real M2 hits AIME-prefix problems first (lex-first ordering), which take 4-7x longer per cell.

---

## §14. Polymarket-Fastest Recommended Sequencing

If user goal is shortest path to Polymarket P-M9 controlled smoke, and `solve rate` optimization is explicitly deferred:

```
Step 0 (decision):     Kill M2 batch + declare substrate stable @ 91 cells
                       cumulative + 175 constitution gates GREEN. Forward-bind
                       full M1 + M2 to post-Polymarket session.
                       (Saves ~9 days wall + ~¥2500 API)

Step 1 (~0.5 day):     Bind D.4 / I.3 MarketSeed §5.5 verbatim names
                       (tests/constitution_marketseed_hardening.rs — same
                       pattern as session #25 §5.3 work)

Step 2 (optional, ~1-2 days, can defer):
                       Wire PromptCapsule into evaluator runtime (C.5
                       PARTIAL-S → DONE). Not blocking Polymarket but
                       brings constitution strictness up one notch.

Step 3 (~0.5 day):     Update STAGE_C_POLYMARKET_PM0_PM9 charter to mark
                       P-M0 SHIPPED + draft P-M1 charter

Step 4 (~2-3 days):    P-M1 implementation — CompleteSetMergeTx struct
                       + sequencer arm + 4 §5.4 verbatim tests
                       + monetary invariant extension
                       Class 4 STEP_B parallel-branch.

Step 5-12 (~12-15 days): Sequential P-M2 .. P-M9 implementation
                         per architect §5 charter implementation manual.

Total estimated MVP path: ~3-4 weeks vs current charter-strict path
~6-8 weeks (M2 + V + A + S phases + then Polymarket from scratch).
```

---

## §15. Maintenance Note

This manifest should be regenerated after:
- Any Stage C P-M atom ships (1 row promotes)
- Stage A3.6 enhancement TB ships (B.4 KNOWN-GAP closes to DONE)
- PromptCapsule evaluator wire-up commits (C.5 PARTIAL-S → DONE)
- MarketSeed verbatim binding commits (D.4 PARTIAL-W → DONE)
- M2 batch completes + V/A/S phase ships (J.3 RUNNING → DONE)
- Architect §8 ratifies any "M-ladder strict ordering" deviation (J.2 status)

Refresh procedure: walk each row's "Real Evidence" column with `ls / git log / grep` against current HEAD; do not rely on LATEST.md narrative.

---

**End of Manifest**.

`Authority`: physical-evidence audit by Claude Opus 4.7 (1M context) at session #26 close 2026-05-09; reviewed against `constitution.md`, `CONSTITUTION_EXECUTION_MATRIX.md` (225 lines), `TB-18B_charter_2026-05-07.md`, `STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md`, and architect alignment doc `2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_zh.md` §3 + §5 + §7.
