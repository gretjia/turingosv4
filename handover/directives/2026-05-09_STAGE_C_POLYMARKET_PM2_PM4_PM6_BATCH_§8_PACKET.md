# Stage C Polymarket — P-M2 + P-M4 + P-M6 Batch §8 Sign-Off Packet

**Date**: 2026-05-09 session #27
**HEAD at packet draft**: `7395aaa` (P-M6 merge commit)
**Plan**: `/home/zephryj/.claude/plans/cozy-waddling-raven.md` Step 9
**Cadence**: User-confirmed batch (vs per-atom) — accepted §8 veto cascade
risk in exchange for ~3-4 weeks shorter wall-clock to Stage C ship.

---

## §1. Authority Chain

- User verbatim 2026-05-09: "把 polymarket 前的所有 gate 全部完成，尽快推进 polymarket 代码落地" + plan-mode approval of `cozy-waddling-raven.md` + auto-mode authorization for continuous execution.
- Charter: `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md` §3.3 / §3.5 / §3.7 + CR-StageC-PM.16 (per-Class-4-atom STEP_B).
- Architect alignment manual: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` §7.3 + §7.5 + §7.7 (verbatim spec for the 3 atoms).
- Parent §8 sign-off authority: `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md` (TB-C0 Constitution Landing freeze-lift; Stage C eligible).

---

## §2. Atoms in this batch

### P-M2 — CompleteSetMergeTx (commit `bac20ba` merge of `8bf5352`)
- **Class**: 4 STEP_B
- **Branch**: `feat/p-m2-completeset-merge` → main via `--no-ff` merge
- **Surface**: typed_tx schema bump + sequencer admission arm
- **Architect spec**: §7.3 verbatim
- **New tests** (5 verbatim names, all GREEN):
  - `merge_yes_no_returns_coin`
  - `merge_requires_both_sides`
  - `merge_conserves_total_coin`
  - `merge_reduces_collateral`
  - `merge_unavailable_after_final_redeem_if_shares_exhausted`
- **Trust Root rehash**: 5 files (typed_tx.rs / sequencer.rs / transition_ledger.rs / monetary_invariant.rs / run_summary.rs)

### P-M4 — Integer CpmmPool state (commit `8c74034` merge of `8e9149a`)
- **Class**: 4 STEP_B (q_state additive)
- **Branch**: `feat/p-m4-cpmm-pool` → main via `--no-ff` merge
- **Surface**: q_state EconomicState 14th sub-field; NEW types CpmmPool / CpmmPoolsIndex / LpShareAmount / PoolStatus / PoolEventKind
- **Architect spec**: §7.5 verbatim
- **New tests** (4 verbatim names, all GREEN):
  - `pool_created_from_seed_inventory`
  - `pool_reserves_not_counted_as_coin`
  - `lp_shares_not_counted_as_coin`
  - `pool_cannot_exist_without_collateralized_shares`
- **Trust Root rehash**: q_state.rs only

### P-M6 — Mint-and-Swap Router (commit `7395aaa` merge of `d6f699f`)
- **Class**: 4 STEP_B
- **Branch**: `feat/p-m6-router` → main via `--no-ff` merge
- **Surface**: typed_tx schema bump + sequencer admission arm + monetary invariant extension
- **Architect spec**: §7.7 verbatim (atomic 9-step composite over CompleteSetMint + CpmmSwap)
- **New tests** (9 verbatim names, all GREEN):
  - `buy_yes_with_coin_matches_formula`
  - `buy_no_with_coin_matches_symmetric_formula`
  - `buy_yes_debits_coin_locks_collateral`
  - `buy_yes_mints_complete_set`
  - `buy_yes_transfers_retained_yes_plus_swap_yes`
  - `buy_yes_respects_min_yes_out`
  - `buy_yes_no_f64`
  - `buy_yes_no_ghost_liquidity`
  - `router_atomic_rollback_on_failure`
- **Trust Root rehash**: 5 files (typed_tx.rs / sequencer.rs / transition_ledger.rs / monetary_invariant.rs / run_summary.rs)
- **Notable**: `assert_complete_set_balanced` extended to include `cpmm_pools_t` reserves in Σ_yes / Σ_no sums (architect §7.5 "share balances controlled by pool" semantics).

---

## §3. Constitution-Gate Progression

| Stage | Gates GREEN | Delta |
|-------|-------------|-------|
| HEAD `b468140` (Manifest baseline) | 175 | — |
| Step 1 tech-debt log distribution | 175 | +0 (doc-only) |
| Step 2 P-M0 charter label | 175 | +0 (doc-only) |
| Step 3 P-M1 verify | 175 | +0 (existing) |
| Step 4 P-M2 ship | 180 | +5 |
| Step 5 P-M3 ship | 185 | +5 |
| Step 6 P-M4 ship | 189 | +4 |
| Step 7 P-M5 ship | 195 | +6 |
| **Step 8 P-M6 ship** | **204** | **+9** |

Total gate count growth: **+29** new constitution gates landed at HEAD `7395aaa`.

`bash scripts/run_constitution_gates.sh` at HEAD `7395aaa`: **204 PASS / 0 FAIL / 1 ignored** (PASS — all gates GREEN).

---

## §4. Workspace Test Continuity

| Atom | `cargo test --workspace --test <gate>` | Status |
|------|----------------------------------------|--------|
| P-M2 | constitution_completeset_merge | 5/5 GREEN |
| P-M3 | constitution_marketseed_hardening | 5/5 GREEN |
| P-M4 | constitution_cpmm_pool | 4/4 GREEN |
| P-M5 | constitution_cpmm_swap | 6/6 GREEN |
| P-M6 | constitution_router_buy_with_coin | 9/9 GREEN |

`cargo test --workspace --no-fail-fast` at HEAD `7395aaa`: pre-existing env-var test flake under concurrency (`chain_runtime::tests::shared_chain_from_env_no_env_vars_set_legacy_mode`; passes isolated; tracked in `feedback_env_var_test_lock`); 1313+ passed otherwise; **0 P-M-attributable regressions**.

---

## §5. STEP_B Parallel-Branch Evidence (per-atom)

Per CLAUDE.md §12 + `feedback_step_b_protocol`:

| Atom | Branch | Merge commit | A/B isolation | TR rehash |
|------|--------|--------------|---------------|-----------|
| P-M2 | `feat/p-m2-completeset-merge` | `bac20ba` | branch tested green before merge | ✅ 5 files |
| P-M4 | `feat/p-m4-cpmm-pool` | `8c74034` | branch tested green before merge | ✅ q_state.rs |
| P-M6 | `feat/p-m6-router` | `7395aaa` | branch tested green before merge | ✅ 5 files |

Each branch was fully tested (`cargo test --workspace` GREEN + `bash scripts/run_constitution_gates.sh` GREEN + `cargo test --lib verify_trust_root_passes_on_intact_repo` PASS) before `--no-ff` merge to main.

---

## §6. CR-StageC-PM.16 Deviation Note (batch over per-atom)

Charter CR-StageC-PM.16 verbatim:
> NO Class-4 typed-tx schema bump bundled across atoms. Each Class-4 atom (P-M2 / P-M4 if needed / P-M6 if needed) is its own STEP_B with per-atom architect §8 sign-off.

**This packet deviates from the per-atom §8 cadence**:
- Each atom IS its own STEP_B (3 separate feature branches; per-atom Trust Root rehash; per-atom merge commit; per-atom test gate). The STEP_B isolation requirement is satisfied verbatim.
- The deviation is the §8 cadence: rather than 3 separate architect §8 sign-offs (P-M2 → wait → ship; P-M4 → wait → ship; P-M6 → wait → ship), this packet bundles the 3 ratifications into one composite §8 review.

**Risk acceptance**: §8 veto on any of the 3 atoms cascades to all 3 (rollback to `bac20ba^` and reopen at the vetoed atom). User explicitly accepted this risk on 2026-05-09 in exchange for ~3-4 week wall-clock reduction to Stage C ship.

**Mitigation**: intra-batch self-audit between atoms — each atom's commit was preceded by `cargo test --workspace` GREEN + constitution gates GREEN + STEP_B branch isolation. Veto cascade probability bounded by intra-batch quality.

---

## §7. Genesis-Replayability Statement

**genesis_payload.toml** rehash entries (per Trust Root manifest):
- `src/state/typed_tx.rs`: 213251db → 35213911 (P-M2) → f22b0cef (P-M5) → 32d1ae1f (P-M6)
- `src/state/sequencer.rs`: 48452658 → 4238c4ee (P-M2) → aeef7505 (P-M5) → 9bd375f4 (P-M6)
- `src/state/q_state.rs`: c23cc95d → 52c93bc4 (P-M4)
- `src/bottom_white/ledger/transition_ledger.rs`: 3928cd3f → f730ea5f (P-M2) → 44242cba (P-M5) → 3a5f7332 (P-M6)
- `src/economy/monetary_invariant.rs`: 91f66421 → 96c8a3ce (P-M2) → 525c2488 (P-M5) → 24df59c5 (P-M6)
- `src/runtime/run_summary.rs`: defc4697 → 6c6f7a8f (P-M2) → e094fc87 (P-M5) → 32fbe356 (P-M6)

No new field added to `genesis_payload.toml [trust_root]` outside the 6 listed paths. No deletion. Forward-compat: pre-Stage-C chain snapshots deserialize cleanly via `#[serde(default)]` on the new `cpmm_pools_t` field.

`cargo test --lib verify_trust_root_passes_on_intact_repo` PASS at HEAD `7395aaa`.

---

## §8. FC1 Hard Invariant Statement

Per CLAUDE.md §6:
> evaluator_reported_completed_llm_calls = l4_work_attempt_count + l4e_work_attempt_count + capsule_anchored_attempt_count

**P-M2 / P-M4 / P-M6 atoms do NOT touch the externalized-attempt accounting surface**. They are state-mutation typed transactions (Polymarket market mechanics) — NOT LLM-Lean cycle. FC1 hard invariant is structurally unaffected.

Cumulative-109-cell substrate-stable evidence (per `handover/decisions/2026-05-09_M2_KILL_AND_SUBSTRATE_STABLE_DECLARATION.md`) had `chain_invariant Ok delta=0` on every cell pre-Stage-C. Post-Stage-C, FC1 invariant continues to hold (no LLM-Lean path changes).

---

## §9. CR-StageC-PM.16 Forward Bind

If architect §8 review prefers strict per-atom cadence going forward, re-cadence applies to **future Class-4 atoms only** (post-Stage-C if any). Stage C P-M2/P-M4/P-M6 are already shipped as a batch under user-accepted batch cadence; rolling them back to per-atom cadence would require git revert + 3 separate re-merges, which is rework cost the user explicitly asked to avoid.

If §8 vetoes ALL 3 atoms: roll back to `bac20ba^` (the commit before P-M2 merge); rework P-M2 in isolation per stricter cadence; then revisit P-M4 and P-M6 sequentially.

If §8 partial-vetoes (e.g., one atom passes, two veto): roll back the youngest 2 merges; preserve the 1 acceptable atom; rework the 2 vetoed ones.

---

## §10. Architect §8 Sign-Off Request

Architect: please review §1-§9 above + the per-atom commits (`bac20ba`, `8c74034`, `7395aaa`) + the per-atom verbatim test files. If the batch is acceptable, sign off in canonical form (e.g., `好，确认可以 ship` or `同意 sign-off` per multi-clause analysis precedent — see `handover/directives/2026-05-08_STAGE_A3_§8_SIGN_OFF.md`).

If §8 sign-off lands, this packet is closed; Stage C P-M7 / P-M8 / P-M9 (Class 1-3, no per-atom §8) proceed per plan `cozy-waddling-raven.md` Steps 10-12; Stage C overall §8 packet (Step 13) lands after P-M9 controlled smoke evidence.

---

**Status at draft**: 🔵 GATED — awaiting architect §8 ratification.
**Plan-side flow**: blocks Stage C overall §8 (Step 13); does NOT block P-M7 (Step 10) / P-M8 (Step 11) / P-M9 (Step 12) since those are non-Class-4.
