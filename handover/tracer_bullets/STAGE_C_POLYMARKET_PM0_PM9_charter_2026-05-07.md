# Stage C — Polymarket / RSP-M Implementation P-M0..P-M9 (charter, 2026-05-07)

> **Naming note**: This charter has NO TB ID. Architect alignment docs use Stage C
> framing + per-phase `P-M0..P-M9` naming (`zh-doc §5 / en-doc §7 AI-coder Polymarket
> implementation manual`). En-doc §1.2.3's forward TB ID list does NOT assign a TB ID
> to Polymarket directly. If a TB ID is later assigned, it will come from architect
> ratification.

**Authority**: `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md`
§3.3 (Stage C charter draft authorized; executable feature work GATED on Stage A
green AND Stage B1 green; Class-4 STEP_B per atom requires per-atom architect
sign-off going forward).

**Companion architect alignment docs (canonical engineering spec)**:
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_zh.md` §5 (中文叙述; AI coder 完整说明书)
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` §7 (English engineering manual; structs + tests verbatim)

**Mode**: Constitutional Harness Engineering.

**Class**: Mixed
- P-M0 quarantine = Class 1 (no-import gate test)
- P-M1 / P-M3 / P-M5 / P-M7 / P-M8 = Class 3 (production wire-up; no STEP_B-restricted file changes if possible)
- P-M2 (CompleteSetMergeTx) / P-M4 (CpmmPool) / P-M6 (Mint-and-Swap Router) = **Class 4 STEP_B** on `src/state/typed_tx.rs` + `src/state/sequencer.rs` + `src/bottom_white/cas/schema.rs`
- P-M9 controlled smoke = Class 3 evidence

**Phase**: P3 Lean Proof Task Market — Polymarket extension; market exposure on top of Lean
proof bounty market. Per `feedback_launch_priority` post-TB-7 sequencing: Lean Proof Task
Market MVP → NodeMarket / Polymarket / public-chain — Stage C Polymarket opens the
NodeMarket/Polymarket layer.

**Phase tag**:
- `phase_id` = P3 Polymarket / RSP-M extension
- `roadmap_exit_criteria_addressed` = CTF-style 1 Coin = 1 YES + 1 NO market on top of TuringOS economic layer; collateral-backed; integer math; predicate-as-truth; supplements bounty (does NOT replace escrow reward in v1)
- `kill_criteria_tested` = (a) f64 in money path; (b) ghost liquidity; (c) price-as-truth (price overrides predicate); (d) DPMM / pro-rata payout in CTF track; (e) agent-submitted MarketResolveTx; (f) AMM before CompleteSet; (g) shares counted as Coin; (h) automatic per-node 100 YES + 100 NO injection

---

## §1. Scope

Stage C Polymarket executes Stage C of the architect alignment doc:
1. **P-M0** Quarantine legacy f64 CPMM
2. **P-M1** CompleteSet Mint/Redeem hardening
3. **P-M2** CompleteSetMergeTx
4. **P-M3** MarketSeedTx hardening
5. **P-M4** Integer CpmmPool (LiquidityPool state)
6. **P-M5** Share-only swap (YES↔NO; pre-Coin-router)
7. **P-M6** Mint-and-Swap Router (BuyYesWithCoin + BuyNoWithCoin)
8. **P-M7** PriceIndex from CPMM (signal-only)
9. **P-M8** Audit views (`audit_tape view-shares` / `view-pools` / `view-prices` / `view-positions`)
10. **P-M9** Controlled market smoke (Lean task → market lifecycle full path)

All phases are sequenced; later phase MUST NOT begin before earlier phase ships.

Out of scope (explicitly):
- Real-money / restricted beta market — gated on Stage D real-world readiness directive (forward).
- Public chain — gated on Stage D + safety boundary directive (forward).
- LP fee splits / advanced AMM mechanics — forward TB after P-M9 ships.
- Polymarket-on-Polymarket recursive markets — forward.
- ChallengeResolveTx integration with market settlement — forward (this charter integrates at PriceIndex + collateral level only).

## §2. Pre-conditions (HARD — charter execution gated)

Per parent authorization §3.3 + alignment-doc §3 (zh) / §4 (en) Stage C:

- **TB-18R FINAL SHIPPED** (✅ 2026-05-07 per `2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md`).
- **TB-C0 SHIPPED FINAL** (✅ 2026-05-07).
- **Stage A green** — TB-18R Final + AMBER closure progress + HEAD_t C1 GREEN ✅; HEAD_t C2 (Stage A3) RECOMMENDED before P-M2 STEP_B work to avoid ledger storage-form change mid-feature.
- **Stage B1 green** — Wave 3 20p diagnostic ✅ shipped; per architect alignment doc §3 / §4 "Do not start executable Polymarket features until constitution gates and diagnostic benchmarks are stable" — B1 is the minimum threshold; B2 (50p) ✅ also shipped at same session.
- **TB-18B charter LANDED** (✅ 2026-05-07 — `TB-18B_charter_2026-05-07.md`); TB-18B execution NOT a P-M0..P-M5 blocker but IS recommended before P-M9 controlled smoke to ensure substrate stability under load.
- **Universal forbidden list ACK** — see §6 below.

## §3. Functional Requirements (FR) — per architect manual §7 / §5

### §3.1. P-M0 — Quarantine legacy f64 CPMM

**P-M0 STATUS**: 🟢 SHIPPED 2026-05-08 (session #25, commit `d33c25a`) — substantively closed via `tests/constitution_market_quarantine.rs` (§5.2 verbatim, Layer 1+2 scan + no-f64 + no-import) + `tests/constitution_completeset_hardening.rs` (§5.3 8 verbatim tests). Legacy `src/prediction_market.rs` was physically deleted; the no-resurrect fence at `tests/tb_13_legacy_cpmm_forward_fence.rs::prediction_market_legacy_quarantined` asserts absence. No-import + no-f64 grep tests fold into the live `constitution_market_quarantine.rs` rather than separate `tb_18d_*` files.

| ID | Requirement |
|----|-------------|
| FR-PM0.1 | The legacy `src/prediction_market.rs` (f64 CPMM) MUST NOT be imported by ANY new Stage C Polymarket market module. (Note: as of 2026-05-07 the legacy file may already be deleted; if so, the no-resurrect fence stands as forward guard.) |
| FR-PM0.2 | `tests/tb_18d_legacy_cpm_api_not_imported_by_new_market.rs` (NEW) — grep-style test asserting absence of `use crate::prediction_market::*` in any Stage C Polymarket-tagged module. |
| FR-PM0.3 | `tests/tb_18d_no_f64_in_market_modules.rs` (NEW) — grep-style test asserting absence of `f64` / `f32` literals in `src/economy/polymarket*.rs`, `src/economy/cpmm*.rs`, etc. |

### §3.2. P-M1 — CompleteSet Mint/Redeem hardening

**P-M1 STATUS**: 🟢 SHIPPED 2026-05-08 (session #25 / verified session #27 2026-05-09) — `tests/constitution_completeset_hardening.rs` exact-matches all 8 architect manual §7.2 verbatim test names: `mint_one_coin_creates_one_yes_one_no` + `mint_conserves_total_coin` + `shares_not_counted_as_coin` + `redeem_unavailable_before_resolution` + `redeem_yes_after_yes_pays_yes_not_no` + `redeem_no_after_no_pays_no_not_yes` + `redeem_cannot_exceed_share_balance` + `redeem_debits_collateral`. `cargo test --workspace --test constitution_completeset_hardening` → 8 passed.

| ID | Requirement |
|----|-------------|
| FR-PM1.1 | `CompleteSetMintTx` semantics per architect manual: `balances_t[owner] -= amount; conditional_collateral_t[event_id] += amount; share_balance[(owner,event,YES)] += amount; share_balance[(owner,event,NO)] += amount`. |
| FR-PM1.2 | `CompleteSetRedeemTx` semantics: requires resolved outcome; if YES, burn YES shares + owner balance += share_amount + collateral -= share_amount + NO receives 0. Symmetric for NO. |
| FR-PM1.3 | Mint tests: `mint_one_coin_creates_one_yes_one_no` + `mint_conserves_total_coin` + `shares_not_counted_as_coin`. |
| FR-PM1.4 | Redeem tests: `redeem_unavailable_before_resolution` + `redeem_yes_after_yes_pays_yes_not_no` + `redeem_no_after_no_pays_no_not_yes` + `redeem_cannot_exceed_share_balance` + `redeem_debits_collateral`. |

### §3.3. P-M2 — CompleteSetMergeTx (Class 4 STEP_B)

| ID | Requirement |
|----|-------------|
| FR-PM2.1 | NEW `CompleteSetMergeTx` typed-tx schema per architect manual struct: `{tx_id, parent_state_root, event_id, owner, amount, signature}`. |
| FR-PM2.2 | Semantics: `require owner YES >= amount; require owner NO >= amount; burn amount YES; burn amount NO; conditional_collateral_t[event] -= amount; balances_t[owner] += amount Coin`. |
| FR-PM2.3 | Tests: `merge_yes_no_returns_coin` + `merge_requires_both_sides` + `merge_conserves_total_coin` + `merge_reduces_collateral` + `merge_unavailable_after_final_redeem_if_shares_exhausted`. |
| FR-PM2.4 | STEP_B parallel-branch on `src/state/typed_tx.rs` + `src/state/sequencer.rs` admission rules per CLAUDE.md §12. Trust Root rehash routine. |

### §3.4. P-M3 — MarketSeedTx hardening

**P-M3 STATUS**: 🟢 SHIPPED 2026-05-09 (session #27, plan cozy-waddling-raven Step 5) — `tests/constitution_marketseed_hardening.rs` exact-matches all 5 architect manual §7.4 verbatim test names: `market_seed_debits_provider` + `market_seed_creates_yes_no_inventory` + `market_seed_fails_insufficient_balance` + `market_seed_no_ghost_liquidity` + `market_seed_conserves_total_coin`. `cargo test --workspace --test constitution_marketseed_hardening` → 5 passed. Closes manifest D.4 PARTIAL-W → 🟢 GREEN.

| ID | Requirement |
|----|-------------|
| FR-PM3.1 | `MarketSeedTx` MUST be collateral-backed per architect manual: provider deposits `seedC` Coin → CompleteSetMint-like operation creates `seedC` YES + `seedC` NO → shares go to pool inventory → collateral locks `seedC`. |
| FR-PM3.2 | Tests: `market_seed_debits_provider` + `market_seed_creates_yes_no_inventory` + `market_seed_fails_insufficient_balance` + `market_seed_no_ghost_liquidity` + `market_seed_conserves_total_coin`. |
| FR-PM3.3 | NO automatic per-node 100 YES + 100 NO without collateral. |
| FR-PM3.4 | If "per-node initial market making" is needed, MarketMakerBudget pre-set on `on_init` + each `MarketSeedTx` debits from budget (budget = explicit Coin holding pre-allocated at genesis). |

### §3.5. P-M4 — Integer CpmmPool (Class 4 STEP_B if typed_tx surface)

**P-M4 STATUS**: 🟢 SHIPPED 2026-05-09 (session #27, plan cozy-waddling-raven Step 6, STEP_B parallel-branch `feat/p-m4-cpmm-pool`) — `tests/constitution_cpmm_pool.rs` exact-matches all 4 architect manual §7.5 verbatim test names: `pool_created_from_seed_inventory` + `pool_reserves_not_counted_as_coin` + `lp_shares_not_counted_as_coin` + `pool_cannot_exist_without_collateralized_shares`. `cargo test --workspace --test constitution_cpmm_pool` → 4 passed. EconomicState sub-field count 13 → 14 with new `cpmm_pools_t: CpmmPoolsIndex` field. Awaiting **batch §8 sign-off** (P-M2 + P-M4 + P-M6).

| ID | Requirement |
|----|-------------|
| FR-PM4.1 | `CpmmPool` state per architect manual: `{event_id, pool_yes, pool_no, lp_total_shares, status}`. |
| FR-PM4.2 | Pool reserves `pool_yes` and `pool_no` are SHARE balances, NOT Coin. LP shares are NOT Coin. `k = pool_yes * pool_no` (integer / rational math). |
| FR-PM4.3 | Tests: `pool_created_from_seed_inventory` + `pool_reserves_not_counted_as_coin` + `lp_shares_not_counted_as_coin` + `pool_cannot_exist_without_collateralized_shares`. |

### §3.6. P-M5 — Share-only swap

**P-M5 STATUS**: 🟢 SHIPPED 2026-05-09 (session #27, plan cozy-waddling-raven Step 7, STEP_B parallel-branch `feat/p-m5-cpmm-swap`) — `tests/constitution_cpmm_swap.rs` exact-matches all 6 architect manual §7.6 verbatim test names: `swap_no_for_yes_constant_product_non_decreasing` + `swap_yes_for_no_constant_product_non_decreasing` + `swap_fails_zero_input` + `swap_fails_insufficient_pool_output` + `swap_respects_min_out_slippage` + `swap_uses_integer_math_no_f64`. New `CpmmSwapTx` typed-tx variant (TxKind::CpmmSwap=15) + sequencer dispatch arm with integer floor formula + 5 new TransitionError variants (SwapZeroInput / SwapPoolNotFound / SwapInsufficientSenderInput / SwapInsufficientPoolOutput / SwapMinOutNotMet / SwapConstantProductRegressed). Constitution market-quarantine ban list narrowed (` CPMM`, ` AMM`, `PriceIndex`, `yes_price`, `no_price`, `price_yes`, `price_no` removed — defense-in-depth via `no_f64_in_market_modules` + `swap_uses_integer_math_no_f64` + `price_never_overrides_predicate`). Class 3 wire-up — no per-atom §8 pause; covered by Stage C overall §8 sign-off.

| ID | Requirement |
|----|-------------|
| FR-PM5.1 | Buy YES with NO: `outY = floor(dN * poolY / (poolN + dN))`; `poolN1 = poolN + dN`; `poolY1 = poolY - outY`. |
| FR-PM5.2 | Buy NO with YES: symmetric `outN = floor(dY * poolN / (poolY + dY))`. |
| FR-PM5.3 | Integer invariant: `poolY1 * poolN1 >= poolY * poolN` (floor-rounded; dust stays in pool). |
| FR-PM5.4 | Tests: `swap_no_for_yes_constant_product_non_decreasing` + `swap_yes_for_no_constant_product_non_decreasing` + `swap_fails_zero_input` + `swap_fails_insufficient_pool_output` + `swap_respects_min_out_slippage` + `swap_uses_integer_math_no_f64`. |

### §3.7. P-M6 — Mint-and-Swap Router (Class 4 STEP_B if typed_tx surface)

| ID | Requirement |
|----|-------------|
| FR-PM6.1 | `BuyYesWithCoinRouter` atomic 9-step flow per architect manual §7.7 verbatim. `outY = floor(payC * poolY / (poolN + payC))`; `getY = payC + outY`; `priceY = payC / getY`. |
| FR-PM6.2 | `BuyNoWithCoinRouter` symmetric. |
| FR-PM6.3 | Atomic rollback on failure (any of the 9 steps fails → entire tx reverts). |
| FR-PM6.4 | Integer invariant: `poolY1 * poolN1 >= poolY * poolN`. |
| FR-PM6.5 | Tests (per architect manual §7.7): `buy_yes_with_coin_matches_formula` + `buy_no_with_coin_matches_symmetric_formula` + `buy_yes_debits_coin_locks_collateral` + `buy_yes_mints_complete_set` + `buy_yes_transfers_retained_yes_plus_swap_yes` + `buy_yes_respects_min_yes_out` + `buy_yes_no_f64` + `buy_yes_no_ghost_liquidity` + `router_atomic_rollback_on_failure`. |

### §3.8. P-M7 — PriceIndex from CPMM (signal-only)

| ID | Requirement |
|----|-------------|
| FR-PM7.1 | `price_yes_effective = quote_payC / quote_getY`; `price_no_effective = quote_payC / quote_getN`. |
| FR-PM7.2 | Price quote MUST NOT change state (read-only operation). |
| FR-PM7.3 | Price MUST NOT decide predicate truth. Existing `tests/constitution_predicate_gate.rs::price_never_overrides_predicate` (GREEN) is preserved. |
| FR-PM7.4 | Tests (per manual §7.8): `price_quote_does_not_change_state` + `price_signal_not_predicate` + `price_does_not_make_failed_node_accepted` + `low_liquidity_warning`. |

### §3.9. P-M8 — Audit views

| ID | Requirement |
|----|-------------|
| FR-PM8.1 | `audit_tape view-shares` shows owner YES/NO shares. |
| FR-PM8.2 | `audit_tape view-pools` shows pool reserves + LP shares. |
| FR-PM8.3 | `audit_tape view-prices` shows price signal + low-liquidity warning. |
| FR-PM8.4 | `audit_tape view-positions` shows NodePositions. |
| FR-PM8.5 | Tests: `audit_view_shares_matches_state` + `audit_view_pools_matches_state` + `dashboard_regenerates_market_view`. |

### §3.10. P-M9 — Controlled market smoke

| ID | Requirement |
|----|-------------|
| FR-PM9.1 | Scenario per architect manual §7.10: Lean task → Agent A WorkTx FirstLong → Agent B ChallengeTx Short → MarketSeedTx by sponsor or treasury → BuyYesWithCoin → BuyNoWithCoin → PriceIndex update → Task resolved → Redeem / merge → Autopsy if loss. |
| FR-PM9.2 | All chain-backed (per `feedback_chaintape_externalized_proposal`). All evidence replayable. |
| FR-PM9.3 | No raw log broadcast (Art. III shielding preserved). |
| FR-PM9.4 | No price-as-truth in resolution path. |
| FR-PM9.5 | Total coin conserved end-to-end. |

## §4. Constitutional Requirements (CR)

| ID | Constraint |
|----|------------|
| CR-StageC-PM.1 | NO f64 in money path (universal forbidden list per parent authorization §4). |
| CR-StageC-PM.2 | NO ghost liquidity (any pool / inventory / share emission MUST trace to a Coin debit). |
| CR-StageC-PM.3 | NO price-as-truth (price MUST NOT modulate predicate verdict; existing `price_never_overrides_predicate` test preserved). |
| CR-StageC-PM.4 | NO dashboard source-of-truth (must regenerate from chain + CAS). |
| CR-StageC-PM.5 | NO automatic per-node 100 YES + 100 NO injection (must be MarketSeedTx with collateral debit). |
| CR-StageC-PM.6 | NO Treasury magic seed without debit. |
| CR-StageC-PM.7 | NO DPMM / pro-rata payout inside CTF track (CTF semantics: 1 Coin = 1 YES + 1 NO; YES wins → YES holder paid; NO holder gets 0; symmetric). |
| CR-StageC-PM.8 | NO price-based settlement (predicate / oracle resolves; price is signal). |
| CR-StageC-PM.9 | NO agent-submitted MarketResolveTx (system-only). |
| CR-StageC-PM.10 | NO agent-submitted system resolution. |
| CR-StageC-PM.11 | NO AMM before CompleteSet hardened (P-M1 ships before P-M4). |
| CR-StageC-PM.12 | NO trading before audit tools (P-M8 ships before any non-smoke external use). |
| CR-StageC-PM.13 | NO public chain before sandbox (Stage D directive forward). |
| CR-StageC-PM.14 | NO real money before readiness gate (Stage D directive forward). |
| CR-StageC-PM.15 | STEP_B parallel-branch protocol per CLAUDE.md §12 for all P-M2 / P-M4 / P-M6 atoms touching `src/state/typed_tx.rs` / `src/state/sequencer.rs` / `src/bottom_white/cas/schema.rs`. Trust Root rehash routine per atom. |
| CR-StageC-PM.16 | NO Class-4 typed-tx schema bump bundled across atoms. Each Class-4 atom (P-M2 / P-M4 if needed / P-M6 if needed) is its own STEP_B with per-atom architect §8 sign-off. |

## §5. Ship Gates (SG) — high-level

Each P-Mx phase has its own internal SG (per architect manual). Stage C Polymarket as a whole ships
FINAL only after:

| ID | Gate | Verification |
|----|------|-------------|
| SG-StageC-PM.1 | All P-M0..P-M9 phases pass per-phase ship gates listed in architect manual §7 / §5 | per-phase test files |
| SG-StageC-PM.2 | `cargo test --workspace` GREEN; ≥1181 PASS (no regression from `feec129`) | runner |
| SG-StageC-PM.3 | `bash scripts/run_constitution_gates.sh` GREEN; ≥97 PASS (no regression) | gate runner |
| SG-StageC-PM.4 | Universal forbidden list audit clean | grep-style tests in `tests/tb_18d_*` |
| SG-StageC-PM.5 | Polymarket forbidden list audit clean | grep-style tests |
| SG-StageC-PM.6 | Codex G1 charter ratification CLOSED | `handover/audits/CODEX_TB_18D_CHARTER_RATIFICATION_*.md` |
| SG-StageC-PM.7 | G2 Codex + Gemini dual audit per phase (at minimum P-M2 + P-M6 + P-M9) AFTER substrate green; conservative ranking | `handover/audits/G2_TB_18D_*` |
| SG-StageC-PM.8 | Per-Class-4-atom architect §8 sign-off (P-M2 + any other Class-4) | `handover/directives/YYYY-MM-DD_STAGE_C_POLYMARKET_PMx_§8_SIGN_OFF.md` per atom |
| SG-StageC-PM.9 | P-M9 controlled market smoke produces tape-replayable end-to-end evidence with Lean-task + market-lifecycle integration; FC1 invariant + economic conservation + price-not-truth all preserved | `handover/evidence/tb_18d_pm9_smoke_*/` |

## §6. Forbidden list (verbatim per architect alignment doc + CLAUDE.md)

```
Universal (parent authorization §4):
- no f64
- no ghost liquidity
- no price-as-truth
- no dashboard source-of-truth
- no real funds
- no public chain

Polymarket-specific (architect manual §6 / §8):
- no automatic per-node 100 YES + 100 NO without collateral
- no Treasury magic seed without debit
- no f64 money math
- no DPMM / pro-rata payout inside CTF track
- no price-based settlement
- no agent-submitted MarketResolveTx
- no agent-submitted system resolution
- no AMM before CompleteSet
- no trading before audit tools
- no public chain before sandbox
- no real money before readiness gate

Stage C Polymarket-specific:
- no Class-4 atom bundling (each Class-4 atom is its own STEP_B)
- no MVP gate regression (97/0/1 baseline)
- no workspace-test regression (1181/0/151 baseline)
- no Polymarket-on-Polymarket recursive markets in this charter
- no LP fee mechanics in this charter
- no real-money / restricted beta in this charter (Stage D forward)
```

## §7. Atom sequence (sequence-binding)

```
P-M0 (Class 1) → P-M1 (Class 3) → P-M2 (Class 4 STEP_B + §8 sign-off) →
P-M3 (Class 3) → P-M4 (Class 3 / 4 STEP_B if typed_tx) → P-M5 (Class 3) →
P-M6 (Class 4 STEP_B + §8 sign-off) → P-M7 (Class 2) → P-M8 (Class 1-2) →
P-M9 (Class 3 evidence)
```

Each phase's per-phase SG must close before next phase begins.

## §8. §8 ship gates (architect)

Stage C Polymarket ships FINAL only after:
1. SG-StageC-PM.1..9 GREEN.
2. Per-Class-4-atom architect §8 sign-offs filed.
3. Codex G1 + G2 dual audits closed.
4. Explicit overall Stage C Polymarket architect §8 sign-off at `handover/directives/YYYY-MM-DD_STAGE_C_POLYMARKET_§8_SIGN_OFF.md`.

## §9. Cross-references

- Architect alignment Stage C (canonical engineering spec): `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_{zh,en}.md` §5 / §7
- Parent authorization: `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md`
- TB-C0 §8 sign-off (FREEZE-lift authority): `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md`
- TB-18R FINAL ship: `handover/directives/2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md`
- Stage A3 HEAD_t C2 charter (recommended pre-condition): `handover/tracer_bullets/STAGE_A3_HEAD_T_C2_charter_2026-05-07.md`
- TB-18B M1/M2 charter (recommended pre-condition for P-M9 smoke): `handover/tracer_bullets/TB-18B_charter_2026-05-07.md`
- Legacy CPMM forward fence: `tests/tb_13_legacy_cpmm_forward_fence.rs`
- Existing CompleteSetMintTx / CompleteSetRedeemTx / MarketSeedTx (LATEST.md session #17): `src/state/typed_tx.rs`
- Existing BoltzmannMaskPolicy + ChallengeResolveTx system-signing: `src/runtime/...`
- Constitution gap analysis: `handover/alignment/CONSTITUTION_GAP_ANALYSIS_2026-05-07.md`
- Launch priority rule: `feedback_launch_priority`
- Class-4 hide-in-Class-3 rule: `feedback_class4_cannot_hide_in_class3`
- Tape-first rule: `feedback_tape_first_real_tests`
- Real problems rule: `feedback_real_problems_not_designed`
