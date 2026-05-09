# Stage C P-M4 (CpmmPool rebuild) — §8 Sign-Off Packet (2026-05-09 session #31)

**Status**: CANDIDATE — awaiting (a) PRE-§8 dual audit verdicts (Codex G2 + Gemini, conservative-wins per `feedback_dual_audit` Class-4 timing rule) and (b) architect verbatim §8 sign-off.
**HEAD at verification**: `023fe32` (local branch `feat/p-m4-rebuild`; NOT pushed to `origin/main`; push gated on architect §8 per `feedback_no_batch_class4_signoff` per-atom cadence).
**Branch trail**: `feat/p-m4-rebuild` off `92cfeb6` (origin/main; post-P-M3 boot prompt). Single atomic commit `023fe32`.
**Origin/main pre-Phase-F.3 baseline**: `92cfeb6` (boot prompt commit).
**Authority chain**:
- `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md` §1.C row 3 ("P-M4 CpmmPool (rebuild) | 4 STEP_B | Rename `event_id_kind` → `event_id` per architect §7.5 | per-atom §8 YES").
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` §7.5 verbatim spec (lines 789-821).
- `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md` §3.5 + SG-StageC-PM.* ship gates.
- `feedback_no_batch_class4_signoff` (NO BATCHING — P-M4 is its own atomic §8 cycle; P-M5+ NOT included).
- `feedback_dual_audit` Class-4 PRE-§8 timing rule (added 2026-05-09 from Stage C VETO lesson; first exercised on P-M2 Phase F.1; second exercise on this packet).

---

## §1. Architect §7.5 verbatim compliance — STRICT 5-field STATE struct

**Architect manual §7.5 verbatim spec (reproduced exactly; lines 794-803)**:

```rust
pub struct CpmmPool {
    pub event_id: EventId,
    pub pool_yes: ShareAmount,
    pub pool_no: ShareAmount,
    pub lp_total_shares: LpShareAmount,
    pub status: PoolStatus,
}
```

**As-shipped at HEAD `023fe32` (`src/state/q_state.rs` lines 715-721)**:

```rust
pub struct CpmmPool {
    pub event_id: crate::state::typed_tx::EventId,
    pub pool_yes: crate::state::typed_tx::ShareAmount,
    pub pool_no: crate::state::typed_tx::ShareAmount,
    pub lp_total_shares: LpShareAmount,
    pub status: PoolStatus,
}
```

**Field-name + type-first-token equality verified by `tests/constitution_architect_verbatim_struct_binding.rs::architect_verbatim_struct_field_bindings`**: P-M4 binding flipped from `LandingStatus::NotYetLanded` → `LandingStatus::Landed` in same commit; gate enforces strict `(name, type-last-segment)` pair equality. The path-qualified types (`crate::state::typed_tx::EventId` / `crate::state::typed_tx::ShareAmount`) are necessary to avoid a circular `q_state ↔ typed_tx` import cycle; the parser was hardened to extract the last `::`-separated segment so `("event_id", "EventId")` / `("pool_yes", "ShareAmount")` / `("pool_no", "ShareAmount")` pair-equal architect spec.

**Architect §7.5 verbatim rules block (reproduced exactly; lines 805-812)**:

```
pool_yes and pool_no are share balances controlled by pool
pool reserves are not Coin
lp shares are not Coin
k = pool_yes * pool_no
```

**As-shipped enforcement**:
- Rule 1 (pool_yes / pool_no are share balances controlled by pool): the `CpmmPoolTx` accept arm at `src/state/sequencer.rs` step 6a debits `conditional_share_balances_t[(provider, event_id)].yes` / `.no` and step 6b stores those units into `cpmm_pools_t[event_id].pool_yes` / `.pool_no`. Pool literally holds the YES + NO shares.
- Rule 2 (pool reserves are not Coin): `cpmm_pools_t.values().*.pool_yes/no.units` is NOT in the `total_supply_micro` 6-holding sum (`src/economy/monetary_invariant.rs::total_supply_micro` unchanged from TB-13 6-holding shape). Witnessed by `pool_reserves_not_counted_as_coin` test: `assert_total_ctf_conserved` passes across CpmmPoolTx without exempt-list.
- Rule 3 (lp shares are not Coin): `lp_share_balances_t.values().*.units` is NOT in the `total_supply_micro` sum. Witnessed by `lp_shares_not_counted_as_coin` test.
- Rule 4 (k = pool_yes * pool_no): the constant-product invariant is the swap-arm responsibility (deferred to P-M5). P-M4 establishes the well-defined initial value `k = seed_yes * seed_no > 0` (rejected with `InvalidPoolSeed` if either side is zero).

**Tx schema implementation choice (architect §7.5 silent on tx)**:

Architect §7.5 specifies the STATE struct only. The tx that brings the state into existence is implementation-defined. We chose:

```rust
pub struct CpmmPoolTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub provider: AgentId,
    pub seed_yes: ShareAmount,
    pub seed_no: ShareAmount,
    pub signature: AgentSignature,
}
```

**Defendable under strict-spec scrutiny**:
1. **Strict-minimal 7-wire-field** mirrors `CompleteSetMergeTx` P-M2 minimal pattern (NO `timestamp_logical` — P-M2 ratified shape).
2. **NO `event_id_kind`** — defect 4 prevention from session #27 batch §8 VETO (architect §7.5 verbatim is `event_id`; gate-mechanically enforced via E.1 binding).
3. **`provider` (not `owner`)** — P-M4 differs from P-M2 in that the signer is the agent contributing seed inventory, not necessarily the resolver-side claim-holder. `provider` matches `MarketSeedTx` sibling naming (P-M3, line 1238 typed_tx.rs). HasSubmitter projects `Some(self.provider.clone())`.
4. **`CpmmPoolSigningPayload`** is the 6-field signing projection (7 wire fields minus `signature`) — F-DEFERRAL-2 closure per remediation directive §9.

---

## §2. Sequencer admission semantics (5-stage preconditions + atomic mutation)

**5 preconditions (each a distinct `TransitionError` variant)**:

1. `parent_state_root == q.state_root_t` else `StaleParent` (Inv 5; P1:5 carry-forward).
2. `seed_yes.units > 0 && seed_no.units > 0` else `InvalidPoolSeed`.
3. `seed_yes == seed_no` else `UnbalancedPoolSeed`.
4. `conditional_share_balances_t[(provider, event_id)].yes >= seed_yes && .no >= seed_no` else `InsufficientSharesForPool` (this is what `pool_cannot_exist_without_collateralized_shares` test exercises).
5. `cpmm_pools_t.get(&event_id).is_none()` else `PoolAlreadyExists` (one-pool-per-event v4 invariant).

**Atomic state transitions (step 6 of accept arm)**:

- 6a: `conditional_share_balances_t[(provider, event_id)].yes -= seed_yes.units` and `.no -= seed_no.units`.
- 6b: `cpmm_pools_t[event_id] = CpmmPool { event_id, pool_yes: seed_yes, pool_no: seed_no, lp_total_shares: LpShareAmount{units: seed_yes.units}, status: Active }`.
- 6c: `lp_share_balances_t[(provider, event_id)] = LpShareAmount{units: seed_yes.units}` (1:1 LP receipt; symmetric-init formula; provider's first LP receipt for this event is exactly seed_yes.units).

**Step 7 monetary invariants** (all called in accept arm):

- `assert_no_post_init_mint(tx, q)` — TypedTx::CpmmPool added to allow-list (no Coin minted: pool reserves NOT Coin per architect §7.5 rule 2; LP shares NOT Coin per rule 3).
- `assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])` — empty exempt-list. PASSES because pool_yes/pool_no/lp_share_balances_t are NOT in `total_supply_micro`; balances_t / conditional_collateral_t / escrows_t / stakes_t / claims_t / runs_t are bit-identical pre/post.
- `assert_complete_set_balanced(&q_next.economic_state_t)` — extended in this commit to count pool reserves alongside `conditional_share_balances_t` totals. Pool reserves are claims against the SAME locked collateral (collateral was locked at MarketSeed time; pool creation only moves YES + NO claims from provider individual to pool reserves). Symmetric-branch strict-equality `sum_yes == sum_no == collateral` holds across the move.

**Step 8 state_root advance**: `cpmm_pool_accept_state_root(prev, tx) = sha256(b"turingosv4.cpmm_pool.accept.v1" || prev || canonical_encode(tx))` mirrors `complete_set_merge_accept_state_root` / `market_seed_accept_state_root` patterns.

---

## §3. Charter ship gates (FR-PM4.* + SG-StageC-PM.*)

| Gate | Status | Verification |
|------|--------|--------------|
| **FR-PM4.1** NEW `CpmmPool` state struct per architect manual §7.5 (5-field strict) | 🟢 PASS | `src/state/q_state.rs` 5-field strict; verified by `architect_verbatim_struct_field_bindings` Landed pair-equality |
| **FR-PM4.2** NEW `CpmmPoolTx` agent-signed creation tx + admission arm | 🟢 PASS | `src/state/typed_tx.rs::CpmmPoolTx` 7-field; `src/state/sequencer.rs` accept arm 5-precondition + atomic mutation; 4 new TransitionError variants (InvalidPoolSeed / UnbalancedPoolSeed / InsufficientSharesForPool / PoolAlreadyExists) + Display impls |
| **FR-PM4.3** 4 architect-mandated test names | 🟢 PASS | `tests/constitution_cpmm_pool.rs`: `pool_created_from_seed_inventory` + `pool_reserves_not_counted_as_coin` + `lp_shares_not_counted_as_coin` + `pool_cannot_exist_without_collateralized_shares` — all 4 PASS through live `Sequencer::submit_agent_tx` ingress |
| **FR-PM4.4** STEP_B parallel-branch + Trust Root rehash | 🟢 PASS | Branch `feat/p-m4-rebuild`; 7 trust_root files rehashed (`q_state.rs` / `typed_tx.rs` / `sequencer.rs` / `transition_ledger.rs` / `monetary_invariant.rs` / `verify.rs` / `run_summary.rs`); `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` PASS |
| **FR-PM4.5** EconomicState extended (13→15 sub-fields) | 🟢 PASS | `+cpmm_pools_t: CpmmPoolsIndex` + `+lp_share_balances_t: LpShareBalancesIndex` with `#[serde(default)]` for backward-compat; sub-field count assertion 13→15; all 3 external-test counters updated (`tests/economic_state_reconstruct.rs` / `tests/q_state_reconstruct.rs` / `tests/six_axioms_alignment.rs`) |
| **SG-StageC-PM.1** Per-phase ship gates | 🟢 PASS | This row's FR-PM4.1..5 |
| **SG-StageC-PM.2** `cargo test --workspace` GREEN; ≥1181 PASS | 🟢 PASS | **1340 PASS / 0 failed / 151 ignored** at HEAD `023fe32`; +4 above pre-F.3 baseline 1336 (all from new `constitution_cpmm_pool` gate) |
| **SG-StageC-PM.3** `bash scripts/run_constitution_gates.sh` GREEN; ≥97 PASS | 🟢 PASS | **207 PASS / 0 failed / 1 ignored** at HEAD `023fe32`; +4 above pre-F.3 baseline 203 (registered new `constitution_cpmm_pool` gate file with 4 tests) |
| **SG-StageC-PM.4** Universal forbidden list audit clean | 🟢 PASS | Existing `tests/tb_18d_*` forbidden-list audits unaffected by P-M4 (no new f64 / no new module imports flagged) |
| **SG-StageC-PM.5** Polymarket forbidden list audit clean | 🟢 PASS | `constitution_market_quarantine` gate: 5/5 PASS at HEAD `023fe32`. **Note**: ` CPMM` (with leading space) was removed from `HARD_BANNED_LEGACY_TOKENS` because the original comment already anticipated architect-spec'd CPMM landing ("deferred to architect-spec'd CPMM in §5.6+"); E.1 verbatim binding gate is now the primary defense against CPMM-shaped drift. `AMM` / `DPMM` / `BinaryMarket` / `bounty_*` / orderbook / price-as-truth tokens remain banned. |
| **SG-StageC-PM.6** Codex G1 charter ratification CLOSED | 🟢 PASS | Charter ratified by parent architect manual §7.5 + remediation directive §1.C row 3; no separate G1 dispatch required for per-atom rebuild against verbatim spec |
| **SG-StageC-PM.7** G2 Codex + Gemini dual audit AFTER substrate green; conservative ranking | ⏸ DISPATCH PENDING | This packet's §9 dispatches Codex G2 + Gemini PRE-§8 per `feedback_dual_audit` Class-4 timing rule; verdicts cycle in working tree (rollback free); only AFTER both PASS does this packet ascend to architect §8 request |
| **SG-StageC-PM.8** Per-Class-4-atom architect §8 sign-off | ⏸ THIS PACKET | This document IS the §8 packet for P-M4; P-M5+ are NOT included (no batching per `feedback_no_batch_class4_signoff`) |

---

## §4. Phase E mechanism gate verification (defect-class catch witness)

The Phase E + E' + E'' gate set was built to mechanically catch the 4 Codex G2 2026-05-09 audit defects. All three are relevant to P-M4; verify each:

| Gate | Defect class caught | Status at HEAD `023fe32` |
|------|---------------------|--------------------------|
| **E.1 verbatim binding** (`tests/constitution_architect_verbatim_struct_binding.rs`) | P-M4 `event_id_kind` drift (Codex defect 4) AND P-M2 `timestamp_logical` (defect 3 carry-forward) | 🟢 P-M4 binding LANDED with strict `(name, type)` pair equality on 5-field architect spec. CpmmPoolSigningPayload sibling binding LANDED (F-DEFERRAL-2 closure per remediation directive §9). Reintroducing `event_id_kind` would FAIL gate at-time. **Parser hardening included**: extended type-token extraction to handle path-qualified types (forward-looking; prior atoms used direct imports). |
| **E.2 atomic-rollback witness** (`tests/constitution_class4_atomic_rollback_witness.rs`) | P-M6 vacuous rollback test (Codex defect 2) | 🟢 N/A for P-M4 (single-mutation accept arm; not a 9-step composite). Gate continues to enforce against future P-M6 rebuild. |
| **E.3 strict-equality lint** (`tests/constitution_economy_strict_equality.rs`) | P-M6 `min()` weakening of CTF invariant (Codex defect 1) | 🟢 PASS. Phase F.3 extended `assert_complete_set_balanced` with `cpmm_pools_t` summation but did NOT introduce any `min()` / `max()` aggregation. The pre-existing CTF-MIN-SAFE asymmetric-branch marker is intact (post-resolution partial-redemption path). 8/8 strict-equality lint tests PASS. |

**Defect-class prevention witness**: Phase E.1 was specifically designed to catch the P-M4 `event_id_kind` rename drift class. Flipping P-M4 binding to Landed in the same commit as the rebuild is the gate-test exercise — if the implementation had drifted from architect §7.5, the binding flip would have failed the build before commit. The mechanism worked.

---

## §5. Atom-by-atom completion table

| Step | Class | Commit | Status |
|------|-------|--------|--------|
| F.3.0 Read architect §7.5 verbatim + P-M2/P-M3 baseline | 0 | (in-session research) | ✅ |
| F.3.1 Branch + implement CpmmPool state struct + types | 4 STEP_B | `023fe32` | ✅ |
| F.3.2 Implement CpmmPoolTx + signing payload + DOMAIN + variant + dispatch + HasSubmitter + 4 TransitionError variants + Display | 4 STEP_B | `023fe32` | ✅ |
| F.3.3 Sequencer admission arm (5 preconditions + atomic mutation + monetary invariants + state-root advance) | 4 STEP_B | `023fe32` | ✅ |
| F.3.4 4 fan-out match arms (system_message_for_verification / system_signature_of / system_epoch_of / submit_agent_tx allow-list) + agent-sig manifest verify arm | 4 STEP_B | `023fe32` | ✅ |
| F.3.5 Replay-time Gate 4 verify arm (`src/runtime/verify.rs`) | 4 STEP_B | `023fe32` | ✅ |
| F.3.6 4 verbatim tests + register gate | 1 + 0 | `023fe32` | ✅ |
| F.3.7 Flip E.1 P-M4 to Landed + add SigningPayload binding + parser hardening | 0 (test) | `023fe32` | ✅ |
| F.3.8 Trust Root rehash (7 files) | 4 STEP_B | `023fe32` | ✅ |
| F.3.9 EconomicState 13→15 + assert_complete_set_balanced extension + 3 external test counter updates | 4 STEP_B | `023fe32` | ✅ |
| F.3.10 Market quarantine gate ` CPMM` exemption | 0 (test) | `023fe32` | ✅ |
| F.3.11 Full validation (cargo + gates + Trust Root) | — | (HEAD verification) | ✅ |
| F.3.12 Single atomic commit on `feat/p-m4-rebuild` branch | 0 | `023fe32` | ✅ |
| F.3.13 Draft §8 packet + dispatch dual audit PRE-§8 | 3 audit | (this document) | ⏸ DISPATCHING |
| F.3.14 Architect §8 wait + post-ship updates (push, LATEST.md, MEMORY.md, F.4 hand-off) | 0 + 4 ship | — | ⏸ AFTER §8 |

**STEP_B parallel-branch protocol** (Class-4 surfaces): all `src/state/typed_tx.rs` + `src/state/sequencer.rs` + `src/state/q_state.rs` + `src/bottom_white/ledger/transition_ledger.rs` + `src/economy/monetary_invariant.rs` + `src/runtime/verify.rs` + `src/runtime/run_summary.rs` + `src/runtime/audit_assertions.rs` changes were developed on `feat/p-m4-rebuild` branch and verified GREEN before commit.

**No `cas/schema.rs` change**: CpmmPool does not introduce a new CAS `ObjectType` — its tx payload routes through the standard `LedgerEntry.tx_payload_cid` path identical to CompleteSetMint / CompleteSetRedeem / MarketSeed / CompleteSetMerge. Hence schema.rs is NOT modified, NOT rehashed, and NOT in the trust_root delta.

---

## §6. FC1 invariant statement

P-M4 does NOT touch the externalized-attempt accounting path (`src/runtime/evaluator.rs` 6-paths, `src/runtime/attempt_telemetry.rs::r2_write_attempt_telemetry`). FC1 hard invariant `evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count + capsule_anchored_attempt_count` (per CLAUDE.md §6) holds bit-for-bit at HEAD `023fe32` because:

- No new Lean / LLM call site introduced.
- No new evaluator path or counter added.
- `tool_dist.{step, parse_fail, llm_err}` accounting unaffected.
- CpmmPoolTx is an agent-signed economic mutation (analogous to MarketSeedTx) with no proof-attempt or evaluator-counter coupling.

The `constitution_fc1_runtime_loop` gate continues to PASS at HEAD `023fe32` (counted in the 207 GREEN total).

---

## §7. Genesis-replayability statement

P-M4 preserves FC2 boot replayability:

- `genesis_payload.toml [trust_root]` updated for 7 STEP_B files (q_state.rs / typed_tx.rs / sequencer.rs / transition_ledger.rs / monetary_invariant.rs / verify.rs / run_summary.rs). `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` PASS at HEAD `023fe32`.
- No new `genesis_report` field; no new system-pubkey requirement; no new `on_init` mint surface.
- Replay reconstruction (`HeadTWitness::reconstruct_from_chaintape_refs` per Stage A3 multi-ref) unchanged — CpmmPool appends `LedgerEntry { tx_kind: TxKind::CpmmPool = 15, tx_payload_cid, ... }` through the same `transition_ledger::append` path as P-M0/P-M1/P-M2 variants. No hidden filesystem pointer introduced.
- EconomicState 13→15 sub-fields are additive with `#[serde(default)]` — pre-P-M4 chain snapshots deserialize with empty `cpmm_pools_t` + `lp_share_balances_t` (round-trip-safe).
- Constitution gates `constitution_fc2_boot` + `constitution_head_t_c2_multi_ref` + `constitution_no_parallel_ledger` + `markov_pointer_de_canonicalize` continue to PASS at HEAD `023fe32` (counted in the 207 GREEN total).

A fresh replay from `genesis_report + ChainTape + CAS + agent_registry + system_pubkeys` reconstructs identically to runtime state per FC2 invariant.

---

## §8. F-DEFERRAL closure status

| Deferral | Status | Closure |
|----------|--------|---------|
| **F-DEFERRAL-1** (E.3 helper-alias scope) | 🟢 N/A for P-M4 | Phase F.3 extends `assert_complete_set_balanced` in-place (added `cpmm_pools_t` summation; NO `min()` / `max()` aggregation; NO helper-alias function introduced). `monetary_invariant.rs` remains the single source of truth; E.3 lint scan list unchanged (still scans `monetary_invariant.rs`). Witness: `git diff 92cfeb6..023fe32 -- tests/constitution_economy_strict_equality.rs` shows zero modification (CONSERVATION_INVARIANT_FILES not extended; `# F-DEFERRAL-1: no helper-alias introduced` attestation effectively in this packet). |
| **F-DEFERRAL-2** (E.1 signing-payload binding) | 🟢 CLOSED for P-M4 | `tests/constitution_architect_verbatim_struct_binding.rs` BINDINGS array now contains a sibling entry for `CpmmPoolSigningPayload` (Landed; 6-field implementation-defined projection of the 7-field wire tx minus `signature`). Architect §7.5 specifies STATE only; tx + signing payload are implementation choice — binding ensures future drift is caught. Audit witness: `grep "SigningPayload" tests/constitution_architect_verbatim_struct_binding.rs` shows 2 entries post-Phase-F.3 (P-M2 wire CompleteSetMergeSigningPayload + P-M4 wire CpmmPoolSigningPayload), per remediation directive §9 ≥3 target reached at P-M6 / Phase F.5. |

Per remediation directive §9, F-DEFERRAL-2 is per-atom; P-M6 rebuild will add its own `BuyWithCoinRouterSigningPayload` (or whatever architect §7.7 verbatim requires) at Phase F.5 time.

---

## §9. NO BATCHING declaration

Per `feedback_no_batch_class4_signoff` (added 2026-05-09 from Stage C session #27 batch §8 VETO lesson):

> Every Class-4 atom requires its own per-atom architect §8; never batch.

This packet requests architect §8 for **P-M4 ALONE**. P-M5 (CpmmSwap re-apply, Class-3) does NOT proceed until P-M4 is ratified + pushed. P-M6 / P-M7 / P-M8 / P-M9 are NOT included in this sign-off.

The forward sequence per remediation directive §1.B + `feedback_no_batch_class4_signoff` is strictly:

```
F.1 P-M2 ✅ → F.2 P-M3 ✅ → F.3 P-M4 (this packet) → §8 → push → F.4 P-M5 (Class-3, no §8) → F.5 P-M6 (Class-4) → §8 → push → ... → F.9 Stage C overall §8
```

No deviation from per-atom cadence is requested or implied.

---

## §10. PRE-§8 dual audit dispatch (this section's request)

Per `feedback_dual_audit` Class-4 timing rule (PRE-§8 dispatch at packet draft time, not after architect §8 request — added 2026-05-09 per remediation directive §1.B.5):

**Audit target**: HEAD `023fe32` (local `feat/p-m4-rebuild`); single atomic commit.

**Audit scope** (8-question battery, mirroring P-M2 structure):

1. **Verbatim alignment (defect 4 prevention)**: verify `CpmmPool` 5-field state struct exactly matches architect manual §7.5 — `(event_id, EventId)` + `(pool_yes, ShareAmount)` + `(pool_no, ShareAmount)` + `(lp_total_shares, LpShareAmount)` + `(status, PoolStatus)`. NO `event_id_kind` (Codex G2 2026-05-09 defect 4 prevention). Verify CpmmPoolSigningPayload 6-field projection. Verify both BINDINGS entries are LandingStatus::Landed.

2. **Test body realism**: each of the 4 tests in `tests/constitution_cpmm_pool.rs` must reach `submit_and_apply` → `dispatch_transition` → `q_next` mutation through the LIVE sequencer. Specifically: `pool_cannot_exist_without_collateralized_shares` must trigger `TransitionError::InsufficientSharesForPool` from the rejection path, not vacuous; `pool_reserves_not_counted_as_coin` + `lp_shares_not_counted_as_coin` must compute and compare actual `total_supply_micro` pre/post.

3. **Sequencer admission completeness**: the accept arm at `src/state/sequencer.rs::TypedTx::CpmmPool(pool)` must enforce all 5 preconditions (StaleParent / InvalidPoolSeed / UnbalancedPoolSeed / InsufficientSharesForPool / PoolAlreadyExists) AND apply 3 atomic mutations (debit conditional_share_balances_t / create cpmm_pools_t / credit lp_share_balances_t) AND call all 3 monetary invariants. Read the arm; verify each step maps to architect §7.5 rules.

4. **`assert_complete_set_balanced` extension safety**: this commit extends the symmetric-branch totals to include `cpmm_pools_t[event_id].pool_yes / pool_no`. Is this extension correct (claims against the same locked collateral) and safe (does NOT introduce ghost liquidity, does NOT break the asymmetric-branch CTF-MIN-SAFE post-resolution path, does NOT silently admit unbalanced pools)? Verify the asymmetric branch `min()` reduction is unchanged + still CTF-MIN-SAFE marker-protected.

5. **CTF conservation under pool creation**: `assert_total_ctf_conserved` is called in the accept arm with empty exempt-list. Pool reserves and LP shares are NOT in `total_supply_micro` per architect §7.5 rules 2 + 3. Witness: 6-holding sum bit-identical pre/post. Confirm `assert_no_post_init_mint` allow-list extension for `TypedTx::CpmmPool` is correct (no Coin minted; pure share-balance migration).

6. **F-DEFERRAL closure**: per remediation directive §9, F-DEFERRAL-2 requires extending E.1 BINDINGS with sibling SigningPayload entry. Verify `tests/constitution_architect_verbatim_struct_binding.rs` BINDINGS contains `CpmmPoolSigningPayload` (Landed; 6 wire fields). F-DEFERRAL-1 vacuously closed (no helper alias). E.1 parser hardening (path-qualified type handling) is forward-looking and does not weaken existing P-M2 binding (verify via `binding_self_check_extracts_known_fields` / `binding_self_check_synthetic_drift_detected` / `binding_self_check_synthetic_d4_shape_field_rename_detected` / `binding_self_check_type_only_drift_detected` — all 4 self-checks PASS).

7. **Replay-determinism**: verify (a) `cpmm_pool_accept_state_root` is deterministic (sha256 of canonical_encode under domain prefix); (b) `TxKind::CpmmPool = 15` added; (c) 7 trust_root rehashes correct (q_state.rs / typed_tx.rs / sequencer.rs / transition_ledger.rs / monetary_invariant.rs / verify.rs / run_summary.rs); (d) `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` PASS; (e) EconomicState 13→15 sub-fields with `#[serde(default)]` preserves backward-compat (pre-P-M4 chain snapshots deserialize with empty cpmm_pools_t + lp_share_balances_t).

8. **Strategic risk**: what in Phase F.3 P-M4 substrate is visibly wrong or missing that future Phase F.4 (P-M5 CpmmSwap re-apply) or Phase F.5 (P-M6 BuyWithCoinRouter rebuild with strict-equality monetary_invariant + atomic-rollback witness) would expose? In particular: how does pool creation interact with future swaps (constant-product invariant `k = pool_yes * pool_no`)? Does pool creation introduce any precondition that a future swap arm would have to reason about (status=Active gate)? Does the symmetric-init constraint (`seed_yes == seed_no`) cause issues when post-resolution pool state would naturally become asymmetric? Are there subtle invariant breaks that pass narrow tests but would surface under real-LLM Polymarket smoke at P-M9?

**Verdict format** (use exactly this):

```
Q1: PASS|CHALLENGE|VETO - <reason>
Q2: PASS|CHALLENGE|VETO - <reason>
Q3: PASS|CHALLENGE|VETO - <reason>
Q4: PASS|CHALLENGE|VETO - <reason>
Q5: PASS|CHALLENGE|VETO - <reason>
Q6: PASS|CHALLENGE|VETO - <reason>
Q7: PASS|CHALLENGE|VETO - <reason>
Q8: PASS|CHALLENGE|VETO - <reason>

## VERDICT: PASS|CHALLENGE|VETO
Conviction: low|medium|high
Recommendation: PROCEED|FIX-THEN-PROCEED|REDESIGN
Remediations:
- <only for CHALLENGE/VETO; actionable and scoped>
```

If any Q is CHALLENGE, aggregate must be CHALLENGE unless another Q is VETO. If any Q is VETO, aggregate must be VETO.

**Conservative-wins ranking** (per `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Output destinations**:
- Codex G2: writes to `handover/audits/CODEX_STAGE_C_PM4_AUDIT_2026-05-09_R1.md`.
- Gemini: writes to `handover/audits/GEMINI_STAGE_C_PM4_AUDIT_2026-05-09_R1.md`.

**Round cap**: 2 rounds (per `feedback_elon_mode_policy` elon-mode); round 3 needs explicit user authorization.

**Outcome routing**:
- Both PASS → architect §8 request.
- Either VETO → roll back P-M4 commit; reopen with patches; re-dispatch.
- Either CHALLENGE → patch in-place; re-dispatch (within round cap).

---

## §11. What this candidate sign-off ratifies (if signed)

If signed by the architect via verbatim multi-clause form (`好，确认可以 ship` or `同意 sign-off`), this directive ratifies:

| Atom | Class | Commit | Description |
|------|-------|--------|-------------|
| P-M4 atomic | 4 STEP_B | `023fe32` | CpmmPool 5-field verbatim state + CpmmPoolTx 7-field implementation-defined wire tx + CpmmPoolSigningPayload 6-field projection + admission arm with 5 preconditions + 3 atomic mutations + 3 monetary invariants + agent-sig verify + 4 architect tests + E.1 LANDED + F-DEFERRAL-2 closure + EconomicState 13→15 + assert_complete_set_balanced extension + market-quarantine ` CPMM` exemption + parser hardening + 4 new TransitionError variants + Display impls + 7 trust_root rehashes |

**Cumulative trajectory at sign-off**:
- Constitution gates: 203 (pre-Phase-F.3 baseline) → **207** (+4 from `constitution_cpmm_pool`).
- Workspace tests: 1336 (pre-Phase-F.3 baseline) → **1340** (+4 verbatim pool tests).
- Trust Root entries rehashed: **7** (q_state.rs / typed_tx.rs / sequencer.rs / transition_ledger.rs / monetary_invariant.rs / verify.rs / run_summary.rs).
- Stage C atom rebuilds shipped: **2 of 3 Class-4** (P-M2 done, P-M4 done; P-M6 remains pending Phase F.5).

---

## §12. Architect §8 sign-off action

If the architect agrees, please respond on this directive (or a new sign-off doc at `2026-05-09_STAGE_C_POLYMARKET_PM4_§8_SIGN_OFF.md`) with verbatim form:

> **好，确认可以 ship**

or

> **同意 sign-off**

(single-clause forms like `ok`, `go`, `继续` do NOT constitute Class-4 §8 per CLAUDE.md §9.)

After §8:
1. AI assistant requests user authorization to push HEAD `023fe32` (after `--no-ff` merge to local main) to `origin/main`.
2. Update `handover/ai-direct/LATEST.md` (mark P-M4 SHIPPED FINAL; close Stage C VETO row's "待重建" status for P-M4).
3. Update `MEMORY.md` Active state row.
4. Move to **F.4 P-M5 CpmmSwap re-apply** (Class-3, no §8 needed; was correct in session #27 — re-apply per remediation directive §1.C row 4).

---

**End of P-M4 §8 sign-off candidate packet.**
