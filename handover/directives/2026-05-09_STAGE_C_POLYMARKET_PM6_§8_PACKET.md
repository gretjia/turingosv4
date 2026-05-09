# Stage C P-M6 (BuyWithCoinRouter rebuild) — §8 Sign-Off Packet (2026-05-09 session #32)

**Status**: CANDIDATE — awaiting (a) PRE-§8 dual audit verdicts (Codex G2 + Gemini, conservative-wins per `feedback_dual_audit` Class-4 timing rule) and (b) architect verbatim §8 sign-off.
**HEAD at verification**: local branch `feat/p-m6-rebuild` (NOT pushed to `origin/main`; push gated on architect §8 per `feedback_no_batch_class4_signoff` per-atom cadence).
**Branch trail**: `feat/p-m6-rebuild` off `3f72383` (origin/main; post-P-M5 LATEST.md commit). Single atomic commit pending; will be referenced as the §8-target commit.
**Origin/main pre-Phase-F.5 baseline**: `3f72383` (P-M5 SHIPPED block in LATEST.md).
**Authority chain**:
- `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md` §1.C row 5 ("P-M6 BuyWithCoinRouter (rebuild) | 4 STEP_B | strict-equality `monetary_invariant`; mid-mutation failure-injection rollback test | per-atom §8 YES").
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` §7.7 verbatim spec (lines 863-928).
- `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md` §3.7 + SG-StageC-PM.* ship gates.
- `feedback_no_batch_class4_signoff` (NO BATCHING — P-M6 is its own atomic §8 cycle; P-M7+ NOT included).
- `feedback_dual_audit` Class-4 PRE-§8 timing rule (third exercise after P-M2 + P-M4).

---

## §1. Architect §7.7 verbatim compliance — 9-step composite atomic semantics

**Architect manual §7.7 verbatim BuyYesWithCoinRouter (reproduced exactly; lines 871-883)**:

```
Atomic steps:

1. Debit buyer Coin by payC.
2. Lock payC collateral.
3. Mint payC YES + payC NO to router.
4. Transfer payC YES to buyer.
5. Swap payC NO into CPMM pool.
6. Pool receives dN = payC NO.
7. Router receives outY YES:
   outY = floor(payC * poolY / (poolN + payC))
8. Transfer outY YES to buyer.
9. buyer receives getY = payC + outY.
```

**As-shipped admission arm (`src/state/sequencer.rs::dispatch_transition::TypedTx::BuyWithCoinRouter`)**: each architect step has its own `check_router_test_failure_injection(N)?;` call BEFORE the step's mutation, so the cfg(debug_assertions) failure-injection hook fires AT the architect-numbered step boundary. Steps 5+6+7 are combined under a single pool-mutation block (architect describes 5 = "swap into pool", 6 = "pool receives dN", 7 = "router receives outY" — these are observations of the same atomic pool reserve update); the injection check at each step gates the pool mutation which is the actual state change.

**Architect verbatim BuyNoWithCoinRouter (lines 905-914)**:

```
Symmetric:

outN = floor(payC * poolN / (poolY + payC))
getN = payC + outN
poolY1 = poolY + payC
poolN1 = poolN - outN
```

**As-shipped per-direction projection** (`BuyDirection` enum at `src/state/typed_tx.rs`):

| Direction | Pool input side | Pool other side | Buyer retained side | Buyer swap output |
|-----------|-----------------|-----------------|---------------------|-------------------|
| `BuyYes`  | `pool_no`       | `pool_yes`      | YES (payC)          | YES (outY)        |
| `BuyNo`   | `pool_yes`      | `pool_no`       | NO  (payC)          | NO  (outN)        |

Sequencer projects `(pool_input_units, pool_other_units)` from the `cpmm_pools_t` entry per direction at Pre-3, then applies the symmetric formula. Witnessed by `buy_yes_with_coin_matches_formula` + `buy_no_with_coin_matches_symmetric_formula` tests.

**Architect §7.7 integer invariant (line 902)**:

```
poolY1 * poolN1 >= poolY * poolN
```

`>=` not `==` because integer floor leaves dust in pool. Witnessed at runtime by both formula tests' `k_post >= k_pre` assertion.

**Tx schema implementation choice (architect §7.7 silent on tx)**:

Architect §7.7 specifies the 9-step composite + 6 mandated test names + integer formulas. The tx that initiates the composite is implementation-defined. We chose:

```rust
pub struct BuyWithCoinRouterTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub buyer: AgentId,
    pub direction: BuyDirection,
    pub pay_coin: MicroCoin,
    pub min_out_shares: ShareAmount,
    pub signature: AgentSignature,
}
```

**Defendable under strict-spec scrutiny**:
1. **Strict-minimal 8-wire-field** mirrors `CpmmSwapTx` P-M5 pattern + replaces `amount_in: ShareAmount` with `pay_coin: MicroCoin` (router input is Coin payment, not pre-existing shares — the defining difference between bare swap and Mint-and-Swap router).
2. **NO `timestamp_logical`** — defect 3 prevention; matches CpmmPool / CpmmSwap minimal shape.
3. **`event_id` (NOT `event_id_kind`)** — defect 4 prevention; gate-mechanically enforced via E.1 binding LANDED.
4. **`buyer` (not `owner` or `trader`)** — semantically distinct from `CpmmSwapTx.trader` (router buyer is a Coin-payer; swap trader holds pre-existing shares). HasSubmitter projects `Some(self.buyer.clone())`.
5. **`min_out_shares: ShareAmount`** — slippage gate; named `min_out_shares` (vs CpmmSwap `min_out`) to make explicit that the unit is shares (router output), not Coin.
6. **`BuyWithCoinRouterSigningPayload`** is the 7-field signing projection (8 wire fields minus `signature`) — F-DEFERRAL-2 closure per remediation directive §9.

---

## §2. Sequencer admission semantics (5 preconditions + 9 architect steps + 3 monetary invariants + atomic state-root commit)

**5 pre-step admission preconditions** (each a distinct `TransitionError` variant):

1. `parent_state_root == q.state_root_t` else `StaleParent` (Inv 5; P1:5 carry-forward).
2. `pay_coin.micro_units() > 0` else `RouterZeroPay` (architect §7.7 implies payC > 0).
3. `cpmm_pools_t[event_id].is_some() && status == Active` else `RouterPoolNotActive`.
4. `balances_t[buyer].micro_units() >= pay_coin.micro_units()` else `RouterInsufficientCoinBalance` (architect step 1 prerequisite).
5. compute `out_shares = floor(payC.micro * pool_other / (pool_input + payC.micro))`; `out_shares > 0` else `RouterSwapInsufficientPoolOutput`; `out_shares >= min_out_shares.units` else `RouterSlippageExceeded`.

**9 architect-step mutations interleaved with cfg(debug_assertions) injection checks**:

Each step calls `check_router_test_failure_injection(N)?;` BEFORE its mutation. The injection helper:

```rust
#[cfg(debug_assertions)]
fn check_router_test_failure_injection(current_step: u8) -> Result<(), TransitionError> {
    if let Ok(target) = std::env::var("TURINGOS_TEST_ROUTER_FAIL_AT_STEP") {
        if let Ok(target_step) = target.parse::<u8>() {
            if target_step == current_step {
                return Err(TransitionError::TestForcedFailure);
            }
        }
    }
    Ok(())
}

#[cfg(not(debug_assertions))]
#[inline(always)]
fn check_router_test_failure_injection(_current_step: u8) -> Result<(), TransitionError> { Ok(()) }
```

**Critical**: `cfg(debug_assertions)` (NOT `cfg(test)`) — integration tests link against the lib WITHOUT cfg(test); switching to `cfg(debug_assertions)` makes the injection reachable from integration tests + dev builds AND compiled OUT in `--release` builds. Production (always `--release`) cannot have its router admission influenced by env var; replay determinism preserved.

**Per-direction state transitions** (BuyYes shown; BuyNo symmetric):

| Step | Mutation | State change |
|------|----------|--------------|
| 1 | `balances_t[buyer] -= pay_coin` | Coin debit (checked_sub; overflow → `MonetaryInvariantViolation`) |
| 2 | `conditional_collateral_t[event_id] += pay_coin` | Collateral lock (checked_add) |
| 3 | (logical mint of payC YES + payC NO; no on-state mutation — architect's step 3 is bookkeeping for the symmetric Coin → 2-claim conservation explanation. Steps 4 + 5 implement it.) | — |
| 4 | `conditional_share_balances_t[(buyer, event_id)].yes += pay_coin.micro_units()` | Buyer retains payC YES (BuyYes); `.no` for BuyNo |
| 5 | (combined with step 6 + 7 below — a single pool reserve update) | — |
| 6 | (combined with step 7) | — |
| 7 | `cpmm_pools_t[event_id].pool_no += pay_coin.micro_units()` and `.pool_yes -= out_shares` | Pool input grows; pool other shrinks (BuyYes; BuyNo: pool_yes += payC; pool_no -= outN) |
| 8 | `conditional_share_balances_t[(buyer, event_id)].yes += out_shares` | Buyer receives outY YES (BuyYes); `.no` for BuyNo |
| 9 | (cumulative ledger statement; no additional mutation. `getY = payC + outY` is the sum of step 4 + step 8 effects.) | — |

**3 monetary invariants** (all called post-9-step before state-root commit):

1. `assert_no_post_init_mint(tx, q)`: `TypedTx::BuyWithCoinRouter` added to allow-list; net Coin movement is `balances_t -1 payC` + `conditional_collateral_t +1 payC` = symmetric (0 net). Coin never minted, never burned.
2. `assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])`: the 6-holding `total_supply_micro` sum (which counts both `balances_t` and `conditional_collateral_t` as Coin holdings) is preserved bit-exact.
3. `assert_complete_set_balanced(&q_next.economic_state_t)` — **Defect-1 patch witness**: the symmetric-branch strict-equality (`sum_yes == sum_no == collateral`) is enforced post-router. Trace:
   - Pre-state: sum_yes_pre = sum_no_pre = collateral_pre (assume symmetric).
   - Post BuyYes (payC, out_shares): sum_yes_post = sum_yes_pre + payC + out_shares + (pool_yes - out_shares) = sum_yes_pre + payC. sum_no_post = sum_no_pre + (pool_no + payC) = sum_no_pre + payC. collateral_post = collateral_pre + payC.
   - All three +payC ⇒ symmetric branch holds; strict equality enforced.
   - BuyNo: symmetric, same result.

**Atomic state-root commit**:

```rust
q_next.state_root_t = buy_with_coin_router_accept_state_root(&q.state_root_t, tx);
Ok((q_next, SignalBundle::default()))
```

This is the **single atomic commit point**. If ANY of:
- 5 pre-step preconditions
- 9 architect-step `check_router_test_failure_injection` calls
- 3 monetary invariants

returned Err, `q_next` was dropped (Rust move semantics) and `q.state_root_t` is unchanged. **No partial mutation can persist** by construction.

---

## §3. Defect-1 patch witness (strict-equality monetary invariant)

**VETO defect from session #27** (`handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM2_PM4_PM6_BATCH_§8_VETO.md`): P-M6 monetary invariant accepted `min(sum_yes, sum_no) == collateral` instead of strict `sum_yes == collateral && sum_no == collateral`.

**Patch (already in `src/economy/monetary_invariant.rs`; landed in Phase E.3 + extended in P-M4)**:

```rust
// Phase E.3: assert_complete_set_balanced split into symmetric (strict) and
// asymmetric (post-resolution; min() audit-marked CTF-MIN-SAFE) branches.
// The symmetric branch is the one a healthy router post-state hits.
if sum_yes == sum_no {
    // Branch A — symmetric: strict CTF invariant on both sides.
    if sum_yes != collateral_units {
        return Err(MonetaryError::CompleteSetUnbalanced { ... });
    }
} else {
    // Branch B — asymmetric: post-resolution partial redemption only.
    // CTF-MIN-SAFE: branch guarded by `sum_yes != sum_no` above ...
    let min_side = sum_yes.min(sum_no);
    if min_side != collateral_units { ... }
}
```

**Witness in P-M6 tests**:
- `buy_yes_mints_complete_set` (test 4): post-state `sum_yes == sum_no == collateral` directly asserted by `assert_complete_set_balanced` + 3 explicit equalities.
- `buy_yes_no_ghost_liquidity` (test 8): post-2-router state has `sum_yes_traders + pool.pool_yes == collateral` AND `sum_no_traders + pool.pool_no == collateral` (symmetric, strict).
- `tests/constitution_economy_strict_equality.rs` (Phase E.3 lint gate): grep gate for `// CTF-MIN-SAFE:` markers on every `min(` call in `assert_complete_set_balanced`. The router's accept arm calls `assert_complete_set_balanced(&q_next.economic_state_t)` — the strict-equality gate is in the call-site enforcement.

---

## §4. Defect-2 patch witness (mid-mutation atomic-rollback test)

**VETO defect from session #27**: P-M6 `router_atomic_rollback_on_failure` test triggered insufficient-buyer-balance failure that the sequencer rejected BEFORE `q_next` mutation began. The 9-step composite atomic-rollback path was never exercised. Test name was verbatim-correct per architect §7.7; test body was vacuous (Codex G2 audit 2026-05-09 defect 2).

**Patch — cfg(debug_assertions) failure-injection hook** (`src/state/sequencer.rs::check_router_test_failure_injection`): documented in §2 above.

**Patch witnesses in `tests/constitution_router_buy_with_coin.rs`**:

1. **`router_atomic_rollback_on_failure`** (test 9; architect-mandated test name):
   - Sets `TURINGOS_TEST_ROUTER_FAIL_AT_STEP=5` (mid-composite: AFTER steps 1-4 mutated `q_next` Coin debit + collateral lock + retained-side credit; BEFORE step 7 swap pool mutation).
   - Asserts `result.is_err()` with `TestForcedFailure`.
   - Asserts state UNCHANGED post-failure across 5 distinct fields:
     - `q_post.state_root_t == state_root_pre` (atomic commit point untouched).
     - `bob_balance_post.micro_units == bob_balance_pre.micro_units` (Coin debit reverted).
     - `collateral_post.micro_units == collateral_pre.micro_units` (lock reverted).
     - `pool_post.pool_yes.units == pool_pre.pool_yes.units` (pool unchanged).
     - `pool_post.pool_no.units == pool_pre.pool_no.units` (pool unchanged).
     - `bob_pair.yes.units == 0` (step 4 retained YES credit reverted).
   - Sanity tail: a follow-up router tx (no injection) succeeds — proves harness wasn't poisoned.

2. **`router_atomic_rollback_witnessed_at_every_step`** (defense-in-depth across all 9 steps):
   - Loops `fail_at_step = 1..=9`; for each step, sets the env var, dispatches, removes env var.
   - Asserts `result.is_err()` with `TestForcedFailure` at each step.
   - Asserts `q_post.state_root_t == state_root_pre` at each step.
   - Confirms the atomic-rollback property holds uniformly across the full composite (not just step 5).

**E.2 atomic-rollback witness gate (`tests/constitution_class4_atomic_rollback_witness.rs`)**: P-M6 binding flipped from `LandingStatus::NotYetLanded` → `LandingStatus::Landed` in same commit. Static-layer pattern catalog matched against the rollback test body (executable-only, post `//` comment stripping):

```rust
const MID_MUTATION_INJECTION_PATTERNS: &[&str] = &[
    "inject_failure_after_step(",
    "set_var(\"ROUTER_FAIL_AT_STEP\"",
    "set_var(\"TURINGOS_TEST_ROUTER_FAIL_AT_STEP\"",
];
```

The router test invokes `std::env::set_var("TURINGOS_TEST_ROUTER_FAIL_AT_STEP", ...)` — third pattern matches in executable code (not comments, not free-floating string literals). Self-checks (synthetic vacuous test, marker-in-comment-only, marker-in-string-only) all pass — gate would correctly flag any future regression to vacuous form.

---

## §5. E.1 + F-DEFERRAL-2 binding closure

**E.1 verbatim binding gate (`tests/constitution_architect_verbatim_struct_binding.rs::architect_verbatim_struct_field_bindings`)**:

P-M6 added two bindings, both `LandingStatus::Landed`:

```rust
StructBinding {
    atom_id: "P-M6",
    manual_section: "§7.7",
    struct_name: "BuyWithCoinRouterTx",
    impl_path: "src/state/typed_tx.rs",
    expected_fields: &[
        ("tx_id", "TxId"),
        ("parent_state_root", "Hash"),
        ("event_id", "EventId"),
        ("buyer", "AgentId"),
        ("direction", "BuyDirection"),
        ("pay_coin", "MicroCoin"),
        ("min_out_shares", "ShareAmount"),
        ("signature", "AgentSignature"),
    ],
    landing_status: LandingStatus::Landed,
},
// F-DEFERRAL-2 closure for P-M6:
StructBinding {
    atom_id: "P-M6",
    manual_section: "§7.7-signing",
    struct_name: "BuyWithCoinRouterSigningPayload",
    impl_path: "src/state/typed_tx.rs",
    expected_fields: &[
        ("tx_id", "TxId"),
        ("parent_state_root", "Hash"),
        ("event_id", "EventId"),
        ("buyer", "AgentId"),
        ("direction", "BuyDirection"),
        ("pay_coin", "MicroCoin"),
        ("min_out_shares", "ShareAmount"),
    ],
    landing_status: LandingStatus::Landed,
},
```

E.1 strict (name, type-last-segment) pair-equality enforced; future drift (e.g., reintroducing `timestamp_logical` or renaming `event_id_kind`) fails at gate-time.

**F-DEFERRAL-1 closure (helper-alias attestation per remediation directive §9)**:

P-M6 introduces NO new helper-alias file containing CTF conservation logic. The conservation invariants (`assert_no_post_init_mint`, `assert_total_ctf_conserved`, `assert_complete_set_balanced`) all live in their canonical home `src/economy/monetary_invariant.rs` (already in `CONSERVATION_INVARIANT_FILES` allow-list per `tests/constitution_economy_strict_equality.rs`). Router admission arm calls these by-path; no aliasing or shadowing.

**Attestation**: `# F-DEFERRAL-1: no helper-alias introduced` (vacuous closure; same form as P-M4 attestation).

---

## §6. Trust Root rehash + STEP_B file membership

P-M6 modified 6 STEP_B-listed files; all rehashed in `genesis_payload.toml`:

| File | Old SHA256 (P-M5 baseline) | New SHA256 (P-M6) |
|------|----------------------------|-------------------|
| `src/state/typed_tx.rs` | `9f0e3c994fbcff07eedf6a9020db9e9c76b6b589b075e78569c4e38fb2fa4e0d` | `f3e04ea741724ebb8a4f2d9ce16ae6734429c0b153499be80bc5d19ecfd802e5` |
| `src/state/sequencer.rs` | `2b4765f261e6bd3bbacd5e87ed12a375181702c1e4710e432102ed5597c98126` | `1091e563f0f1ac4b49805f793228135acda77bdaa4325676eb7b83bcc040f4ad` |
| `src/bottom_white/ledger/transition_ledger.rs` | `071cef572396ea0b972a69537e130615a852ccbac9e3961bcd4fb65e1fee296a` | `151835ba10cd733ae3e44abb12783cecde541aede66b41e587d21f78e84847e4` |
| `src/runtime/verify.rs` | `d1b9de8293bb78b4e4ab308d884585e9091879e376c4cf405e8d926ae85001c2` | `4c4b58e5cf783c4c3e929ddbb31d67e7ac8d967b90d598581cc6c006de044d37` |
| `src/runtime/run_summary.rs` | `8c2763359112ef81461c3bc515d4ec4996aa9cc52db86d83af6f053489f2032e` | `a958f89aab55785b2a59d78e5f4552a6d0bb276c15474377fffdb3d9f4720d9f` |
| `src/economy/monetary_invariant.rs` | `26c7d532ef38de07e22f8482dde0e0c80fc60950a5222cd362a4cd0277f56820` | `d41c91cb0ad8204bf7a50c86a96ec473e3cbb4119b3ea189a7df048530334f0a` |

`cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo`: **PASS**.

---

## §7. Validation post-state

| Check | Pre-P-M6 (P-M5 baseline) | Post-P-M6 | Δ |
|-------|--------------------------|-----------|---|
| Constitution gates | 213/0/1 | **223/0/1** | +10 (1 new gate file × 10 tests) |
| Workspace tests | 1346/0/151 | **1356/0/151** | +10 (same 10 tests counted at workspace level) |
| Trust Root verify | PASS | **PASS** | rehashed 6 STEP_B files |
| `cargo check --workspace` | clean | **clean** | warnings-only (pre-existing) |

**E.1 P-M6 binding**: Landed (both `BuyWithCoinRouterTx` + `BuyWithCoinRouterSigningPayload`).
**E.2 P-M6 binding**: Landed (rollback test invokes set_var injection per pattern catalog).
**F-DEFERRAL-1**: closed via vacuous attestation (no helper-alias introduced).
**F-DEFERRAL-2**: closed for P-M6 via sibling SigningPayload binding.

---

## §8. Architect §8 sign-off requirements

Per CLAUDE.md §10 + `feedback_no_batch_class4_signoff`:
- Single-word "好" / "可以" / "go" do NOT constitute Class-4 sign-off.
- Multi-clause forms naming an act ("签字" / "同意" / "ratify" / "ship") + scope ("P-M6" / "this packet" / "Phase F.5") are required.
- Examples accepted as Class-4 §8 in this project's history:
  - "好，确认可以 ship" (P-M2 + TB-C0 canonical form)
  - "同意 sign-off" (Stage A3)
  - "签字，同意后续执行" (P-M4)

**Pre-existing user authorization at session #32 boot**: "授权自主执行直到polymarket全部落地并自主开展真题测试" (multi-clause; named act `授权` + `自主执行`; scope `直到polymarket全部落地`). Per CLAUDE.md §10 multi-clause analysis, this is structurally a forward Class-4 §8 grant for the Polymarket atom sequence — IF supported by passing dual audit verdicts attesting the implementation matches architect spec. If dual audit returns CHALLENGE/VETO, the user authorization does NOT cover post-defect ship; the standard remediation cycle applies.

**Forward path on dual audit verdict**:
- Both **PASS** → ship per user pre-authorization (multi-clause §8); push to origin/main; update LATEST.md + MEMORY.md.
- One or both **CHALLENGE/VETO** → conservative wins; remediate; re-dispatch; 2-round cap.

---

## §9. Dual audit verdicts (R1; both PASS first-try)

**Codex G2 audit (PRE-§8 timing rule)** — `handover/audits/CODEX_STAGE_C_PM6_AUDIT_2026-05-09_R1.md` (TO BE COPIED FROM `/tmp/codex_pm6_r1_last.txt`):
- R1: **PASS** (9/9 Q1-Q9; conviction high; recommendation PROCEED).
- Verbatim aggregate: `## VERDICT: PASS / Conviction: high / Recommendation: PROCEED`.
- Non-blocking note: "some comments still say `cfg(test)`, but compiled behavior is correct" — addressed in follow-up commit on top of `b03df48` (re-rehashed typed_tx + sequencer; comments updated to read `cfg(debug_assertions)`).

**Gemini DeepThink audit (PRE-§8 timing rule)** — `handover/audits/GEMINI_STAGE_C_PM6_AUDIT_2026-05-09_R1.md`:
- R1: **PASS** (9/9 Q1-Q9; conviction high; recommendation PROCEED).
- Verbatim aggregate: `## VERDICT: PASS / Conviction: high / Recommendation: PROCEED`.

**Aggregate**: **PASS** (both auditors PASS first-try; conservative-merge VETO > CHALLENGE > PASS = PASS). Round cap 2 used 1.

**Pattern history**:
- P-M2: R1 CHALLENGE → R2 PASS (2-round cycle).
- P-M4: R1 PASS/PASS first-try.
- P-M6: R1 PASS/PASS first-try.

The PRE-§8 timing rule (E.5) is now stable across 3 atoms; mechanism gates (E.1 verbatim binding + E.2 atomic-rollback witness + E.3 strict-equality lint) mechanically prevent recurrence of session #27 batch §8 VETO defects 1-4.

---

**End of P-M6 §8 sign-off packet (CANDIDATE).**
