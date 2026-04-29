# TB-1 Re-Charter — Days 2-7 against P0-P9 phase model (2026-04-29)

**Authority**: architect directive 2026-04-29 (`handover/directives/2026-04-29_9_phase_roadmap.md`) + user `gretjia` chat authorization. Canonical roadmap: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`.

**Original charter**: commit `4ecb708` body. Original GOAL was *"One MiniF2F adaptation problem solved end-to-end at HEAD with the full v4 5-step compile loop active per-tx + economy hooks firing per-tx + L4 ledger commits per-tx + h_vppu computed in PputResult."* That goal bundled four different layer-jumps (P1 ledger, P3 economy, P5 capability compilation, P6 metric) into one 7-day TB.

**Re-charter (this doc)**: keeps Day 1 (already shipped at `063b003`); re-tags Days 2-7 against the 9-phase model; descopes one acceptance test (AT-5) that properly belongs to a P5 MetaTape TB after P3 is green.

**Charter scope**: Days 2-7 only. Day 1 is shipped and final.
**Active TB**: TB-1.
**phase_id**: P1+P3+P6 (P1 primary; P3 RSP-0 secondary; P6 instrumentation tertiary).
**Budget**: remaining of original 7 days × ≤$30 API.

---

## 1. Re-tagged GOAL

> Discharge the **first slice of P1 + P3 RSP-0** by demonstrating, on a single MiniF2F problem run:
>
> 1. (P1 Exit 5,6) ledger advances on accept; ledger does NOT advance on reject;
> 2. (P1 Exit 7) deleting any ledger row breaks the hash chain;
> 3. (P1 Exit 8) state.db can be reconstructed from chaintape.jsonl;
> 4. (P1 Exit 9) rejected tx logs do NOT appear in another Agent's read view;
> 5. (P3 RSP-0 Exit 1,2,5) on_init mint is unique; rtool/think do not deduct CTF; an escrow lock is taken before work_tx is admitted;
> 6. (P3 RSP-0 Exit 6,8) acceptance produces only `provisional_accept`, not full payout; `settlement_tx.payout_sum ≤ escrow_pool`;
> 7. (P6 instrumentation) `h_vppu` field present and non-null on at least one row.

This replaces the previous *"5-step compile loop active per-tx"* goal — step 4 (Capability Compilation) is **out of TB-1 scope** (it's P5 MetaTape work that requires a green P3).

## 2. Days 2-7 schedule (revised)

### Day 2 — P3 RSP-0: monetary invariant + on_init unique mint

**phase_id**: P3 (RSP-0 micro-version)
**Exit addressed**: P3:1, P3:2, P3:5 (`on_init` total Coin invariant; rtool/think don't deduct; escrow required for market admission)
**Kill tested**: P3:1 (post-init mint MUST fail), P3:2 (stakeless write MUST fail)

**Build**:
- `src/economy/monetary_invariant.rs` — module exposing:
  - `pub fn assert_no_post_init_mint(tx: &TypedTx, q: &QState) -> Result<(), MonetaryError>`
  - `pub fn assert_total_ctf_conserved(before: &EconomicState, after: &EconomicState, exempt_tx_kinds: &[TxKind]) -> Result<(), MonetaryError>`
  - `pub fn assert_read_is_free(tx_kind: TxKind, fee: u64) -> Result<(), MonetaryError>` (rtool/search/think MUST have fee=0)
- `src/economy/escrow_vault.rs` — minimum-viable BTreeMap<TaskId, EscrowEntry>:
  - `pub fn lock_escrow(task_id, sponsor, amount) -> EscrowReceipt`
  - `pub fn release_escrow(task_id, payout_map) -> Result<(), EscrowError>` (asserts sum ≤ amount before release)
- Unit tests: post-init mint rejected; total CTF conserved across N=10 random tx sequences; escrow over-payout rejected; escrow under-payout accepted (residual returns to sponsor).

**FROZEN today**: `src/sdk/tools/wallet.rs` (STEP_B-protected); `kernel.rs`; `bus.rs`; `genesis_payload.toml [trust_root]` constitution_root entry.

**Acceptance signal**: `cargo test -p turingosv4 economy::` ≥ 6 tests green; running 1 evaluator shot still produces JSONL row (no regression in P6 capability path).

### Day 3 — P1 GitTape Kernel hardening

**phase_id**: P1
**Exit addressed**: P1:5 (state_root advances on accept), P1:6 (state_root unchanged on reject), P1:7 (ledger hash chain), P1:8 (state.db reconstruction), P1:9 (rejected-log isolation)
**Kill tested**: P1:1 (no wtool bypass), P1:2 (rejected tx ≠ state_root advance), P1:3 (state.db reconstructable), P1:4 (no read-view pollution)

**Build**:
- `src/economy/ledger.rs` (the file the original charter named) — minimum-viable append-only ledger with:
  - `pub fn append(tx: &TypedTx) -> Result<LedgerEntry, LedgerError>` — content-addressed, prev_hash chained
  - `pub fn verify_chain(start: usize, end: usize) -> Result<(), ChainError>` — hash chain integrity
  - `pub fn reconstruct_state(state_path: &Path) -> Result<QState, ReconstructError>`
- 4 P1-kill acceptance tests:
  - `test_p1_kill_1_no_wtool_bypass`: any direct mutation to state.db without going through wtool→ledger panics or fails to round-trip via reconstruct_state.
  - `test_p1_kill_2_rejected_tx_no_state_advance`: simulate a tx that fails predicate; assert state_root unchanged; assert ledger entry IS appended (with status=rejected) but tx is not applied.
  - `test_p1_kill_3_ledger_reconstructable`: drop state.db; reconstruct from ledger; bit-equal to pre-drop state_root.
  - `test_p1_kill_4_rejected_log_isolated`: emit a rejected tx with diagnostic content; assert another Agent's read view does NOT contain the diagnostic substring (only an aggregate counter).
- 1 hash-chain acceptance test:
  - `test_p1_exit_7_chain_breaks_on_row_deletion`: write 5 ledger entries; delete row 3; `verify_chain(0, 5)` returns `Err(ChainError::HashMismatch { at_index: 3 })`.

**FROZEN**: same as Day 2 + the new monetary_invariant.rs (no further edits today).

**Acceptance signal**: 5 new tests green; running 1 evaluator shot now writes ≥1 ledger row per tx; verify the hash chain holds across the run.

### Day 4 — P6 instrumentation: h_vppu computation

**phase_id**: P6 (Epistemic Lab v0 product-line metric)
**Exit addressed**: P6:7 (falsification-tracking metric: h_vppu reflects per-problem repeated-attempt regression; runs that re-attempt a problem with no learning have h_vppu=0)
**Kill tested**: none directly — P6 product-line metric only

**Build**:
- `experiments/minif2f_v4/src/h_vppu_history.rs` (new) — minimum-viable per-problem rolling history:
  - `pub struct HVppuHistory { /* problem_id → VecDeque<f64> with capacity 3 */ }`
  - `pub fn record(problem_id, pput_verified) -> ()`
  - `pub fn h_vppu_for(problem_id, current_pput_verified) -> Option<f64>` (returns `current / mean(history N=1..3)` if at least 1 prior run; else None)
- Wire into `make_pput`: pass history reference; stamp `h_vppu` field on result.
- (Optional, time permitting) upgrade `prompt_context_hash` from DefaultHasher 16-char to SHA-256 64-char; same commit re-hashes Trust Root manifest entry. **Out of TB-1 scope if Day 4 budget tight**; defer to TB-2 cleanup.

**FROZEN**: same as Days 2-3 (P3 monetary_invariant, P1 ledger, all STEP_B files).

**Acceptance signal**: 2 new evaluator runs of mathd_algebra_107 in n3 mode produce JSONL rows where the second row has `h_vppu` ≠ None; `cargo test` h_vppu_history unit tests ≥ 3 green.

### Day 5 — Acceptance test battery (5 original + 6 new)

**phase_id**: P1+P3 (battery integration)
**Exit addressed**: cumulative — every Exit listed in Days 2-3
**Kill tested**: cumulative — every Kill listed in Days 2-3

**Build** — `tests/tb_1_acceptance.rs` (new):
1. **(original AT-1)** evaluator runs n3 swarm on mathd_algebra_107 → solved=true (regression baseline vs `f0b659f`); phase=P6.
2. **(original AT-2)** each tx in the run produces a `LedgerEntry` committed via `Git2LedgerWriter` (or the new `src/economy/ledger.rs`); phase=P1 Exit 5,6,7.
3. **(original AT-3)** PputResult.h_vppu non-null on a 2nd-run row; phase=P6.
4. **(original AT-4)** PputResult.econ_balance_delta non-zero; agent's CTF balance changed by escrow + release; reputation counter +1 on accepted Verify-tx; phase=P3 Exit 3,5.
5. ~~**(original AT-5)**~~ **DESCOPED**: "second attempt of same problem in same session uses 1st attempt's winning tactic in prompt context" properly belongs to P5 MetaTape v1 (ArchitectAI proposal flow). Filed for a future TB after P3 RSP-3 green. Not part of TB-1 ship gate.
6. **(NEW P1 kill 1)** `test_p1_kill_1_no_wtool_bypass` — direct state mutation outside wtool fails.
7. **(NEW P1 kill 2)** `test_p1_kill_2_rejected_tx_no_state_advance`.
8. **(NEW P1 kill 3)** `test_p1_kill_3_ledger_reconstructable`.
9. **(NEW P1 kill 4)** `test_p1_kill_4_rejected_log_isolated`.
10. **(NEW P3 RSP-0 Exit 1)** `test_p3_rsp0_exit_1_on_init_total_invariant` — sum of CTF balances after on_init = sum of CTF balances after N work_tx + verify_tx + settlement_tx sequence.
11. **(NEW P3 RSP-0 Exit 6,8)** `test_p3_rsp0_exit_6_8_provisional_then_payout_capped` — accept produces only provisional accept; settlement_tx.payout_sum ≤ escrow_pool.

**Acceptance signal**: all 10 tests green (10 = 4 originals retained + 4 P1 kill + 2 P3 Exit; AT-5 descoped). If any kill test goes RED → STOP TB-1; write `OBS_TB-1_FAILED_2026-04-29.md`; charter must change before retry. Kill-with-OBS NOT permitted.

### Day 6 — Dual external audit (unchanged)

**Codex + Gemini parallel** with focus = "do these 10 tests prove the claimed P1/P3 RSP-0 properties?" — not spec wording. Apply VETO > CHALLENGE > PASS conservatism per `feedback_dual_audit_conflict`. Patches accepted as-is.

### Day 7 — Ship

If Day-6 dual audit returns PASS/PASS or CHALLENGE/PASS with all challenges addressed:

- TB_LOG.tsv: TB-1 row → status=`shipped`; capability_metric updated with measured `h_vppu` value (or "deferred to TB-2" if the SHA-256 upgrade was deferred); ship_commits range filled.
- Post a TB-2 candidate to user. Default candidate per directive ordering = **TB-2 = P3 RSP-1** (task escrow + work_tx + yes_stake; advances RSP-0 → RSP-1, addresses P3 Exit 3,5; tests P3 kill 2 fully green).

If Day-6 returns VETO:

- Write `handover/alignment/OBS_TB-1_FAILED_2026-04-29.md` with diagnosis layer (P1 / P3 / P6 instrumentation / charter scope).
- Revert OR keep-with-OBS (NOT for kill criteria; only for Exit-criteria coverage gaps).
- Charter MUST change before retry.

## 3. Out-of-scope items moved to future TBs

| Original AT | Reason | Future home |
|---|---|---|
| AT-5 (winning-tactic in prompt context) | Step-4 Capability Compilation = P5 MetaTape, requires P3 RSP-3 green first | TB-N (P5 MetaTape v1; post-P3-RSP-3) |
| SHA-256 upgrade for prompt_context_hash | Touches Cargo.lock + Trust Root re-hash; cleanest in a dedicated cleanup TB | TB-2 cleanup or TB-3 P5 prep |
| Per-tx FC events for every economy mutation | Belongs in P4 Information Loom signal-routing TB | TB-N (P4 v0) |

## 4. Things this re-charter does NOT change

- Day 1 (shipped at `063b003`) — final, no rewind.
- TB-1 budget — same 7 days × ≤$30 API as original charter.
- The 5 frozen files per TB-1 ship surface (`evaluator.rs`, `jsonl_schema.rs`, `src/economy/ledger.rs` [new], `tests/tb_1_acceptance.rs` [new], TB_LOG.tsv) — same surface; Day 2 adds `src/economy/monetary_invariant.rs` + `src/economy/escrow_vault.rs` to the surface, both new files (no STEP_B file edited).
- 24h iteration cap (memory `feedback_iteration_cap_24h`) — every Day must produce evaluator pass/fail signal within 24h.
- Trust Root protocol (R-014 + R-018) — unchanged; any new file going into the manifest follows the established hash-update protocol.

## 5. Acceptance for re-charter itself

The re-charter ships when:

- This doc is committed.
- TB_LOG.tsv reflects the new column schema with TB-1 phase_id correctly tagged.
- AUTO_RESEARCH_NOTEPAD.md TB methodology v2 references this doc.
- Day 2 work begins with the new `src/economy/monetary_invariant.rs` skeleton.

The re-charter is reverted if any of:

- User retracts the P0-P9 ordering authorization (no expected trigger).
- Day 2 monetary_invariant tests reveal that `on_init` mint-only is silently bypassed by an existing code path (would be a P0 ALREADY-FAILED kill criterion → escalate before continuing).
