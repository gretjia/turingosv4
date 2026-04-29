# Codex TB-1 Round-1 Dual External Audit
**Date**: 2026-04-29
**Target**: TB-1 Days 1-5 ship readiness (commits 063b003..6c04c26)
**HEAD**: 6c04c26a658b150e94b1d3148046531c65ef98ef
**Prompt size**: 154243 chars
**Mandate**: implementer-paranoid (Q1-Q8). Independent of Gemini r1 (parallel).

---

Reading prompt from stdin...
OpenAI Codex v0.125.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: danger-full-access
reasoning effort: xhigh
reasoning summaries: none
session id: 019dda7b-038f-7e50-8907-e75afde78628
--------
user
# Codex Audit — TB-1 Days 1-5 (P1 GitTape + P3 RSP-0 + P6 h_vppu instrumentation)

**Role**: skeptical adversarial reviewer (implementer-paranoid angle).
Independent of Gemini round-1 (running in parallel; strategic/architectural angle).

**Mandate**: verify TB-1 ship readiness. TB-1 ships when ALL Tier-A tests
(1-9) are green AND the dual external audit clears. The ship gate per CF-5
"lighter option" downgrades P6 (Tier-B) to non-blocking; only P1 + P3 RSP-0
correctness gates the ship.

**Per Elon-mode 2-round cap (memory `feedback_elon_mode_policy`)**: this is
round 1. Round 2 only runs if round 1 returns CHALLENGE/VETO. Disposition:

- PASS  → TB-1 ships Day-7 (no further audit before merge)
- CHALLENGE → specific patches required before ship
- VETO  → charter must change before retry (per `feedback_phased_checkpoint`;
          NOT permitted on Tier-A kill criteria themselves)

## What to verify

The CENTRAL claim under audit:

> "These 9 Tier-A tests, taken together, prove that:
>   (P1) the v4 GitTape kernel honors the L4 / L4.E split — accepted txs
>        advance state on L4; rejected txs go to L4.E only; deleting any row
>        on either chain breaks its hash chain; agent-facing read views are
>        structurally isolated from L4.E raw diagnostics; and state.db is
>        bit-equal reconstructable from L4 alone.
>   (P3 RSP-0) total CTF is conserved across redistribution sequences; reads
>        are free for all K5 TxKinds; any post-init mint surfaces as
>        MonetaryError::PostInitMint and routes to L4.E (not L4)."

Your audit answers: **does the test battery prove this claim, or does it have
adversarial gaps that let a malicious implementer pass all 9 tests while
breaking the invariant?**

## Round-1 audit questions (5; implementer-paranoid)

**Q1. (P1 kill 1) Is `test_p1_kill_1_no_wtool_bypass` a real bypass test?**
The test:
  1. Builds a clean L4 chain of 3 entries; persists.
  2. Tampers state.db on disk (overwrites last entry's resulting_state_root with 0xFF...FF).
  3. Asserts `load_from_path` either returns Err OR reconstructs to a state_root that differs from the canonical pre-tamper root.

Adversarial check: imagine an implementer "fixes" the test to pass by adding
`if reconstructed != canonical { reconstructed = canonical }` in
`load_from_path` (silent rebase). Would the test still pass? Or does the
test's structure prevent that bypass?

Sub-question: the test only checks one bypass shape (mutated last entry's
resulting_state_root). What about a bypass that:
  a. Inserts a fake row at position 0 (replaces genesis with a fake)?
  b. Reorders rows (swap row 1 and row 2)?
  c. Mutates parent_state_root only (chain-prev field, not chain-next)?
Are any of these caught structurally by load_from_path's verify_chain, or
would the test miss them?

**Q2. (P1 kill 2 + P3 kill 1) Is the L4 / L4.E disjointness ACTUALLY tested?**
Both tests construct L4 + L4.E side-by-side, append a rejection to L4.E only,
and assert L4 didn't advance. But these tests construct the two ledgers
independently in the test fixture — they don't exercise a real
`dispatch_transition` path that ROUTES based on predicate results.

Adversarial check: a buggy `dispatch_transition` that writes to BOTH L4 and
L4.E on rejection (or to L4 on rejection by mistake) would still let these
tests pass, because the tests don't exercise dispatch_transition. The recharter
explicitly defers WorkTx dispatch_transition body to TB-2 RSP-1 — but does
that mean TB-1 is shipping the L4/L4.E split as scaffolding only? Should
this be made loud in the disposition (e.g., "Tier-A tests prove the
PRIMITIVES; disjointness ENFORCEMENT requires TB-2 dispatch_transition")?

**Q3. (P1 kill 4) Is `test_p1_kill_4_rejected_log_isolated` real isolation?**
The test asserts the `PublicRejectionView` JSON omits raw_diagnostic_cid.
But isolation depends on every code path that materializes
RejectedSubmissionRecord into agent-facing views going through
`PublicRejectionView` — not through `RejectedSubmissionRecord` directly.

Adversarial check: grep the codebase for direct serialization of
`RejectedSubmissionRecord`. If any agent-facing path (e.g., a future
WAL stream, a future librarian board, a future bus.snapshot()) serializes
the record directly, raw_diagnostic_cid leaks. Is there a structural enforcer
(e.g., `#[serde(skip_serializing)]` on raw_diagnostic_cid, or a sealed-trait
projection-only escape hatch), or is the discipline purely convention?

**Q4. (P3 RSP-0 Exit 1) Does the 5-step redistribution sequence test the right thing?**
The test models a closed system: alice 100 → bob 30 → escrow → bob → carol → back to baseline. At every step, `assert_total_ctf_conserved(prev, next, &[])` must succeed.

Adversarial check: a bug where `total_supply_micro` undercounts a sub-index
(e.g., forgets `claims_t` or `task_markets_t.bounty`) might still pass this
test if the test sequence never touches that sub-index. Does the 5-step
sequence exercise ALL 6 sub-indices that `total_supply_micro` counts
(balances, escrows, stakes, claims, task_markets bounty, challenge_cases bond),
or only some? If only some, then a regression in the un-exercised sub-indices
would slip through.

Also verify: the sequence uses a closed-system roundtrip (end == start).
Does this test catch a bug where total_supply_micro is constant-zero (e.g.,
an empty sum)? Probably not — both sides would equal zero.

**Q5. (P3 RSP-0 Exit 2 + P3 kill 1) Is `assert_read_is_free` a real
structural guard, or just a function that's called nowhere?**

Read `src/economy/monetary_invariant.rs` Q3.b. If `assert_read_is_free` is
defined but NEVER called from a real dispatch_transition / settlement
path, then test 8 is testing the function's own logic, not the property
"reads are free in the K5 economy". A future read-tx could carry a fee
of 1, the function would correctly return ReadCharged, but the dispatch
path would never call the function — silent bypass.

Same question for `assert_no_post_init_mint` and
`assert_total_ctf_conserved`. Are these wired into a real dispatch path
today? If not, what's the call-site landing TB?

## Round-1 audit questions (3; charter-discipline)

**Q6. Is the P6 Tier-B downgrade defensible per the recharter ship gate?**
The recharter Day-5 (per CF-5 lighter option) says: "TB-1 ships when all
Tier-A tests (1-9) are green. Tier-B tests (10-13) are captured as
artifacts but do not gate ship." But Tier-B 10-13 are non-trivial:
- T10 (mathd_algebra_107 evaluator solve) — verified out-of-band by Day-4
  evidence (commit 50a1d67; both n3 runs solved=true).
- T11 (L4 entry per dispatched tx) — explicitly deferred to TB-2 RSP-1.
- T12 (h_vppu non-null on 2nd run) — verified by Day-4 commit 50a1d67;
  unit test stub registered as #[ignore] in tb_1_acceptance.rs (cannot
  import minif2f_v4 from turingosv4 integration tests).
- T13 (econ_balance_delta non-zero) — needs RSP-1 wiring.

Question: does the Day-4 live evidence actually anchor T10 + T12, or are
those screenshots / log files outside the canonical test harness? Is "out
of band evidence" a defensible substitute for an in-harness assertion?

**Q7. Did Day-3 cleanly absorb the Day-2 monetary_invariant + escrow_vault?**
Day 2 commit 451cc66 shipped monetary_invariant.rs + escrow_vault.rs but
did NOT wire them into a dispatch path; Day 3 commit 846279f shipped
ledger.rs + rejection_evidence.rs. The recharter Day-5 acceptance battery
exercises monetary_invariant via direct fn calls (good) and exercises the
L4/L4.E primitives via direct AcceptedLedger / RejectionEvidenceWriter API
calls (good). But escrow_vault is NOT exercised by any Tier-A test. Is
that intentional? Should there be a Tier-A test for escrow lock + release
+ overpayout-reject before TB-1 ships?

**Q8. Are there NEW defects introduced by Day-4 + Day-5?**
Day-4 wired h_vppu into evaluator.rs main(); re-hashed Trust Root.
Day-5 added tests/tb_1_acceptance.rs and removed tests/tb_1_p1_acceptance.rs.

- Is the Day-4 evaluator.rs wire-up Goodhart-safe? Specifically, does the
  ordering `load → query → stamp → record → save` correctly EXCLUDE the
  current run's pput_verified from the ratio (held-out semantics)? Read
  the unit test `test_record_before_query_does_not_self_reference` — does
  it actually verify the production wire-up's ordering, or just the
  HVppuHistory API in isolation?
- Day-5 deleted tb_1_p1_acceptance.rs and re-added the 6 P1 tests in
  tb_1_acceptance.rs. Are the test bodies BIT-EQUAL to the Day-3 versions,
  or did the consolidation introduce a regression?

## Verdict format

Section A: Verdict (PASS / CHALLENGE / VETO) with conviction level.
Section B: Per-Q1-Q8 disposition (one paragraph each).
Section C: P0 list (must-fix-before-ship, if any).
Section D: P1 list (should-fix; can ship-with-OBS per Elon-mode policy).
Section E: Final disposition recommendation.

Be concrete. Cite file:line. Run grep / read commands as needed (you have
filesystem access via `view` / `apply_patch` etc.). Refute or confirm each
claim against actual code, not the spec wording.

---

# XREF materials follow.


---

## XREF: TB-1 recharter (audit target spec) — `handover/tracer_bullets/TB-1_recharter_2026-04-29.md`

```
# TB-1 Re-Charter — Days 2-7 against P0-P9 phase model (2026-04-29)

**Authority**: architect directive 2026-04-29 (`handover/directives/2026-04-29_9_phase_roadmap.md`) + user `gretjia` chat authorization. Canonical roadmap: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`.

**Amended 2026-04-29 (post-audit)**: external auditor's CF-1 / CF-3 / CF-5 incorporated per `handover/audits/2026-04-29_external_audit.md` and user authorization on 2026-04-29. Specific amendments: Day-3 wording switched to L4 / L4.E split (rejected submissions go to L4.E rejection-evidence ledger, NOT to L4 with `status=rejected`); Day-2 framing sharpened (WalletTool = read-only projection of `EconomicState.balances_t`, not "legacy adapter"); Day-5 acceptance gate downgraded so P1/P3 are blocking and P6 artifacts are non-blocking until RSP-1.

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

**WalletTool framing (sharpened post-audit 2026-04-29 CF-3)**: `src/sdk/tools/wallet.rs` is **NOT a legacy mutable adapter** — it is a *read-only projection* of `QState.economic_state_t.balances_t`. Mutations to economic state happen exclusively through the canonical RSP path (`SettlementEngine` / `EscrowVault` / `StakeManager` / `monetary_invariant`). No new RSP code may depend on `WalletTool.credit()` or on `WalletTool` mutating its `HashMap<String, f64>` to represent canonical balance state. Existing `WalletTool` tests stay temporarily as legacy behavior tests; they get removed or rewritten as RSP-1/RSP-2 lands.

**Acceptance signal**: `cargo test -p turingosv4 economy::` ≥ 6 tests green; running 1 evaluator shot still produces JSONL row (no regression in P6 capability path).

### Day 3 — P1 GitTape Kernel hardening

**phase_id**: P1
**Exit addressed**: P1:5 (state_root advances on accept), P1:6 (state_root unchanged on reject), P1:7 (ledger hash chain), P1:8 (state.db reconstruction), P1:9 (rejected-log isolation)
**Kill tested**: P1:1 (no wtool bypass), P1:2 (rejected tx ≠ state_root advance), P1:3 (state.db reconstructable), P1:4 (no read-view pollution)

**Build** (post-audit 2026-04-29 CF-1: TWO ledgers, not one):
- `src/economy/ledger.rs` — minimum-viable accepted-only L4 wrapper around the existing `src/bottom_white/ledger/transition_ledger.rs`. Provides:
  - `pub fn append_accepted(tx: &TypedTx) -> Result<LedgerEntry, LedgerError>` — content-addressed, prev_hash chained, advances `logical_t`.
  - `pub fn verify_chain(start: usize, end: usize) -> Result<(), ChainError>` — L4 hash chain integrity.
  - `pub fn reconstruct_state(state_path: &Path) -> Result<QState, ReconstructError>` — replays L4 only (L4.E NOT consulted; rejections must not affect `state_root`).
- `src/bottom_white/ledger/rejection_evidence.rs` (NEW, post-audit 2026-04-29 CF-1): minimum-viable rejection-evidence ledger:
  - `RejectedSubmissionRecord` struct with `submit_id`, `parent_state_root`, `agent_id`, `tx_kind`, `tx_payload_cid`, `rejection_class`, `raw_diagnostic_cid` (Option), `public_summary` (Option), `prev_hash`, `hash`.
  - `RejectionEvidenceWriter::append_rejected()` returns the new chain hash.
  - `verify_chain()` returns `Err` on row deletion.
  - **No `logical_t`** — uses `submit_id` instead.
  - **No `state_root` advance** — `dispatch_transition` rejection path MUST NOT mutate `q.state_root_t` or `q.ledger_root_t`.
- 5 P1-kill acceptance tests (re-tagged post-audit):
  - `test_p1_kill_1_no_wtool_bypass`: any direct mutation to state.db without going through wtool→L4 panics or fails to round-trip via `reconstruct_state`.
  - `test_p1_kill_2_rejected_tx_no_state_advance`: simulate a tx that fails predicate; assert `state_root` unchanged; assert **L4 logical_t NOT incremented**; assert **L4.E `submit_id`-scoped record IS appended** (one record, raw_diagnostic_cid populated).
  - `test_p1_kill_3_ledger_reconstructable`: drop state.db; reconstruct from L4 only; bit-equal to pre-drop `state_root`. L4.E intentionally not consulted in reconstruction.
  - `test_p1_kill_4_rejected_log_isolated`: emit a rejected tx with diagnostic content; assert another Agent's materialized read view does NOT contain the raw diagnostic (only an aggregate counter or `public_summary` is permitted).
  - `test_p1_kill_4b_rejection_chain_breaks_on_row_deletion`: write 3 rejection-evidence records; delete row 2; `RejectionEvidenceWriter::verify_chain()` returns `Err(RejectionEvidenceError::HashMismatch { at: 2 })`.
- 1 L4 hash-chain acceptance test:
  - `test_p1_exit_7_l4_chain_breaks_on_row_deletion`: write 5 accepted L4 entries; delete row 3; `verify_chain(0, 5)` returns `Err(ChainError::HashMismatch { at_index: 3 })`.

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

**Tier A — BLOCKING (P1 + P3 RSP-0 correctness; TB-1 ship requires ALL Tier-A tests green):**

1. **(P1 kill 1)** `test_p1_kill_1_no_wtool_bypass` — direct state mutation outside wtool fails.
2. **(P1 kill 2)** `test_p1_kill_2_rejected_tx_no_state_advance` — `state_root` unchanged; L4 `logical_t` NOT incremented; L4.E `submit_id`-scoped record appended.
3. **(P1 kill 3)** `test_p1_kill_3_ledger_reconstructable` — drop state.db; reconstruct from L4 only; bit-equal pre-drop `state_root`.
4. **(P1 kill 4)** `test_p1_kill_4_rejected_log_isolated` — raw L4.E diagnostic NOT in another Agent's materialized view (only aggregate / public_summary).
5. **(P1 Exit 7)** `test_p1_exit_7_l4_chain_breaks_on_row_deletion`.
6. **(P1 kill 4b)** `test_p1_kill_4b_rejection_chain_breaks_on_row_deletion` — L4.E hash chain integrity.
7. **(P3 RSP-0 Exit 1)** `test_p3_rsp0_exit_1_on_init_total_invariant` — `total_coin(EconomicState)` sum invariant across N tx sequence.
8. **(P3 RSP-0 Exit 2)** `test_p3_rsp0_exit_2_read_is_free` — `assert_read_is_free(fee=0)` for `rtool` / `search` / `think`; non-zero fee returns `MonetaryError::ReadCharged`.
9. **(P3 kill 1)** `test_p3_kill_1_no_post_init_mint` — any post-`on_init` `mint_tx` returns `MonetaryError::PostInitMint`; rejection MUST be in L4.E, not L4.

**Tier B — NON-BLOCKING (P6 Epistemic Lab anchor evidence + future-RSP placeholders; TB-1 ship does NOT require these green; they are artifacts to capture, not gates):**

10. **(original AT-1, post-audit downgrade per CF-5)** evaluator runs n3 swarm on mathd_algebra_107 → solved=true. **Non-blocking**: this is P6 anchor smoke; TB-1's mission is P1/P3 correctness, not capability regression. If P6 regresses, file as a separate P6 anchor TB; it does NOT block TB-1 ship.
11. **(original AT-2, post-audit re-tag)** each tx in the run produces an L4 `LedgerEntry` committed via `Git2LedgerWriter` (or the new `src/economy/ledger.rs`); rejected proposals appear in L4.E. **Non-blocking** for TB-1 ship — until WorkTx `dispatch_transition` body lands (TB-1 Day-3 minimum WorkTx slice or later TB), the evaluator path may continue using its own legacy emit path. The L4 / L4.E assertions are evaluated against synthetic test inputs in Tier-A tests #2/3/5, not against the live evaluator run.
12. **(original AT-3, post-audit re-tag)** `PputResult.h_vppu` non-null on a 2nd-run row. **Non-blocking**: P6 instrumentation; useful artifact but not a P1/P3 gate.
13. **(original AT-4, post-audit downgrade per CF-3)** `PputResult.econ_balance_delta` non-zero. **Non-blocking until RSP-1**: RSP-0 (Day-2) only proves the conservation invariant + scaffolds escrow/balances structures; actual non-zero deltas require RSP-1's `escrow_lock_tx` + `yes_stake_tx` to fire, which lives in TB-2.
14. ~~**(original AT-5)**~~ **DESCOPED**: "second attempt of same problem in same session uses 1st attempt's winning tactic in prompt context" properly belongs to P5 MetaTape v1 (ArchitectAI proposal flow). Filed for a future TB after P3 RSP-3 green. Not part of TB-1 ship gate.

**Acceptance signal** (post-audit 2026-04-29 CF-5 lighter option): TB-1 ships when **all Tier-A tests (1-9) green**. Tier-B tests (10-13) are captured as artifacts but do not gate ship. If any Tier-A kill test goes RED → STOP TB-1; write `OBS_TB-1_FAILED_2026-04-29.md`; charter must change before retry. Kill-with-OBS NOT permitted on Tier-A.

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

```


---

## XREF: L4 vs L4.E decision record — `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`

```
# Decision Record — Rejection Evidence Ledger separate from Accepted Transition Ledger

**Date**: 2026-04-29
**Status**: ACCEPTED
**Driver**: external audit 2026-04-29 CF-1 (`handover/audits/2026-04-29_external_audit.md`)
**Authority**: user `gretjia` chat authorization on 2026-04-29 ("授权 6 个 items 全部执行")
**Decision class**: architectural boundary, P1 GitTape Kernel ledger semantics
**Reversibility**: in-principle reversible only via constitution amendment (Layer-1 architectural commitment)

---

## Decision

**Rejected submissions never enter the L4 accepted-transition ledger.**

The two ledgers are distinct, with distinct identifiers, distinct hash chains, and distinct contribution to `Q_t`:

```text
L4   Accepted Transition Ledger  (= existing src/bottom_white/ledger/transition_ledger.rs)
     - identifier:        logical_t (monotone-increasing)
     - hash chain:        ledger_root chain over accepted entries
     - Q_t contribution:  advances state_root_t and ledger_root_t
     - replay use:        replay_full_transition reconstructs Q_t from L4 only
     - source of truth:   accepted state-machine transitions

L4.E Rejection Evidence Ledger   (= new src/bottom_white/ledger/rejection_evidence.rs)
     - identifier:        submit_id (assigned at submit time, monotone but
                          orthogonal to logical_t — not all submit_ids become
                          logical_ts)
     - hash chain:        rejection_root chain over rejected records
     - Q_t contribution:  NONE — does NOT mutate state_root_t or ledger_root_t
     - read view:         raw_diagnostic_cid is private/shielded; only
                          aggregate counter and public_summary may surface
                          in another Agent's materialized view
     - consumers:         P4 ErrorClusterer, P3 ChallengeCourt, P3 ReputationIndex,
                          P5 ArchitectAI proposal flow (post-RSP-3)
```

## Reasoning

Three reasons, in order of severity:

### 1. Replay semantics

If rejected entries shared `logical_t` with accepted entries, the invariant "every `logical_t` is an accepted state transition; replay produces bit-equal `state_root`" would break. Then `replay_full_transition` would either need to skip `logical_t`s (defeats the monotone-time abstraction) or apply rejected entries as no-ops (defeats the auditability of why a `logical_t` exists at all).

Splitting the ledgers means:
- `replay_full_transition` consumes L4 only and produces a deterministic `state_root`.
- L4.E is auditable independently — useful for forensic / clusterer / reputation queries — but does not interfere with state reconstruction.

### 2. Goodhart shield

Accepted transitions are public constitutional state by definition. Rejected diagnostics are *failure detail*: stack traces, predicate-error strings, raw LLM output. If both lived in the same ledger, every Agent's materialized view would have to choose between (a) seeing all of L4 plus rejected diagnostics — context contamination + Goodhart attack surface, or (b) filtering at read time — which puts privacy logic in `materializer.py` instead of in the ledger schema, where it belongs.

Splitting means:
- L4 entries are accepted state — Agents' read views see them per the standard `read_set_authorized` predicate.
- L4.E entries default to private (raw_diagnostic_cid hidden); only the aggregate counter / public_summary surface unless an Agent has a privileged role (P3 ChallengeCourt, P5 ArchitectAI sandboxed read).

### 3. Context contamination prevention

The constitutional "selective broadcasting / selective shielding" principle (Art. III.4) requires that one Agent's failure does not pollute another Agent's prompt. A single-ledger model would force every read view to scrub rejected entries — an opt-out model that is fragile against future code paths.

Splitting means: by default, L4.E is shielded; access requires explicit role grant; policy violation is detectable as a schema-level read-set-authorization violation, not a content-level filtering bug.

## Implementation contract

### Sequencer behavior

```rust
// src/state/sequencer.rs (CF-4 minimum WorkTx dispatch lands at TB-1 Day-3+)

while let Some(tx) = queue_rx.recv().await {
    let submit_id = /* assigned at submit time */;
    match self.apply_one(tx.clone()) {
        Ok(entry) => {
            // L4 path: accepted transition
            self.transition_ledger.append(entry)?;
            // logical_t increments here; state_root + ledger_root advance
        }
        Err(ApplyError::Transition(inner)) => {
            // L4.E path: rejection evidence
            self.rejection_writer.append_rejected(RejectedSubmissionRecord {
                submit_id,
                parent_state_root: q_snapshot.state_root_t,
                agent_id: tx.agent_id(),
                tx_kind: tx.kind(),
                tx_payload_cid: tx.payload_cid(),
                rejection_class: classify(&inner),
                raw_diagnostic_cid: Some(cas_put(inner.raw_diagnostic())),
                public_summary: Some(inner.public_summary()),
                prev_hash: self.rejection_writer.head_hash(),
                hash: /* computed */,
                timestamp_logical_submit: now(),
            })?;
            // CRITICAL: do NOT mutate q.state_root_t or q.ledger_root_t.
            // Specifically: do NOT call self.transition_ledger.append(...) here.
        }
        Err(ApplyError::Infrastructure(inner)) => {
            // separate class: not L4, not L4.E by default; may halt or
            // route to operator log per OBS_BOOT_FAIL_NOT_HALT-style policy.
        }
    }
}
```

### Read-view shielding

`materializer.py` (or its Rust equivalent) MUST NOT include `raw_diagnostic_cid` content in any Agent's read view by default. Permitted L4.E projections in a default read view:

- aggregate counters (`rejection_count_by_class`, `rejection_count_by_agent`)
- `public_summary` (the predicate's intentionally-broadcast short string)
- the `submit_id` itself (not the payload)

Privileged roles (challenge court, architect sandbox) may opt in to richer projections via explicit role-grant, audited.

### Tests required (TB-1 Day-3 Tier-A)

- `test_p1_kill_2_rejected_tx_no_state_advance`: L4 `logical_t` NOT incremented; L4.E record IS appended.
- `test_p1_kill_4_rejected_log_isolated`: another Agent's view has aggregate counter only; raw_diagnostic_cid content NOT visible.
- `test_p1_kill_4b_rejection_chain_breaks_on_row_deletion`: L4.E hash chain integrity.

## Out-of-scope (this decision does not say)

- Whether L4.E is content-addressed (CAS) under-the-hood. Recommendation: yes (each `raw_diagnostic` and each `RejectedSubmissionRecord` payload becomes a CID), but the decision binds only the ledger-separation semantics, not the storage backend.
- Whether L4.E entries can be garbage-collected after a retention window. Recommendation: yes, per Art. III.4 selective broadcasting; specific retention policy is a P4 Information Loom design decision.
- Whether L4.E is replicated across multi-org peers in P6. Recommendation: yes for `public_summary` + aggregate counters; raw diagnostics are local-only by default. Specific multi-org policy is P6 work.

## Cross-references

- Audit driving this decision: `handover/audits/2026-04-29_external_audit.md` § CF-1
- ROADMAP P1 amendment: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` § P1 Build / Exit 6 / Exit 9
- TB-1 Day-3 charter: `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-3 (Tier-A tests 1-6)
- Memory hint: `~/.claude/.../memory/feedback_rejection_evidence_separate.md`
- Existing transition ledger code (= L4): `src/bottom_white/ledger/transition_ledger.rs`
- New rejection evidence module home (= L4.E): `src/bottom_white/ledger/rejection_evidence.rs` (to be created at TB-1 Day-3)

```


---

## XREF: Tier-A acceptance battery (Day-5 final) — `tests/tb_1_acceptance.rs`

```
//! TB-1 Day-5 final acceptance battery — Tier-A 9 BLOCKING + Tier-B 4 NON-BLOCKING.
//!
//! Charter: `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` § Day-5.
//! Tier discipline (audit CF-5 "lighter option"): TB-1 ships when ALL Tier-A
//! tests are green. Tier-B tests are captured as artifacts but DO NOT gate
//! ship; if a Tier-B test goes red, file as a follow-up TB rather than
//! blocking TB-1's P1/P3 RSP-0 deliverable.
//!
//! Tier-A (BLOCKING — P1 + P3 RSP-0 correctness):
//!   1. test_p1_kill_1_no_wtool_bypass                       (P1 kill 1)
//!   2. test_p1_kill_2_rejected_tx_no_state_advance          (P1 kill 2)
//!   3. test_p1_kill_3_ledger_reconstructable                (P1 kill 3)
//!   4. test_p1_kill_4_rejected_log_isolated                 (P1 kill 4)
//!   5. test_p1_exit_7_l4_chain_breaks_on_row_deletion       (P1 Exit 7)
//!   6. test_p1_kill_4b_rejection_chain_breaks_on_row_deletion (P1 kill 4b)
//!   7. test_p3_rsp0_exit_1_on_init_total_invariant          (P3 RSP-0 Exit 1)
//!   8. test_p3_rsp0_exit_2_read_is_free                     (P3 RSP-0 Exit 2)
//!   9. test_p3_kill_1_no_post_init_mint                     (P3 kill 1)
//!
//! Tier-B (NON-BLOCKING — P6 anchor evidence + future-RSP placeholders):
//!  10. test_at1_evaluator_solves_mathd_algebra_107_n3       (#[ignore]: live LLM)
//!  11. test_at2_l4_entry_per_dispatched_tx                  (#[ignore]: WorkTx dispatch
//!                                                           body lands TB-2 RSP-1)
//!  12. test_at3_h_vppu_non_null_on_second_run               (UNIT form; live form
//!                                                           verified by Day-4 evidence)
//!  13. test_at4_econ_balance_delta_non_zero                 (#[ignore]: RSP-1)
//!
//! AT-5 (winning-tactic-in-prompt-context) is DESCOPED per recharter — moves
//! to a future P5 MetaTape v1 TB after P3 RSP-3 lands.

use std::collections::{BTreeMap, BTreeSet};

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    PublicRejectionView, RejectionClass, RejectionEvidenceError, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::transition_ledger::TxKind;
use turingosv4::economy::ledger::{AcceptedLedger, LedgerError};
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin, MICRO_PER_COIN};
use turingosv4::economy::monetary_invariant::{
    assert_read_is_free, assert_total_ctf_conserved, MonetaryError,
};
use turingosv4::state::q_state::{AgentId, EconomicState, Hash, TxId};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey,
    SafetyOrCreation, TaskId, TypedTx, WorkTx, WriteKey,
};

// ────────────────────────────────────────────────────────────────────────────
// Fixtures
// ────────────────────────────────────────────────────────────────────────────

fn fixture_work_tx(suffix: u32) -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId(format!("acc-{}", suffix)),
        BoolWithProof {
            value: true,
            proof_cid: Some(Cid([0x11; 32])),
        },
    );
    let mut settlement = BTreeMap::new();
    settlement.insert(
        PredicateId(format!("set-{}", suffix)),
        BoolWithProof {
            value: true,
            proof_cid: None,
        },
    );
    let mut read_set = BTreeSet::new();
    read_set.insert(ReadKey(format!("k.r.{}", suffix)));
    let mut write_set = BTreeSet::new();
    write_set.insert(WriteKey(format!("k.w.{}", suffix)));
    TypedTx::Work(WorkTx {
        tx_id: TxId(format!("worktx-{}", suffix)),
        task_id: TaskId(format!("task-{}", suffix)),
        parent_state_root: Hash::ZERO,
        agent_id: AgentId("alice".into()),
        read_set,
        write_set,
        proposal_cid: Cid([0x13; 32]),
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement,
            safety_class: SafetyOrCreation::Safety,
        },
        stake: StakeMicroCoin::from_micro_units(1_000_000),
        signature: AgentSignature::from_bytes([0x77u8; 64]),
        timestamp_logical: suffix as u64,
    })
}

fn cid(byte: u8) -> Cid {
    Cid([byte; 32])
}

fn agent(s: &str) -> AgentId {
    AgentId(s.to_string())
}

fn coin(n: i64) -> MicroCoin {
    MicroCoin::from_coin(n).unwrap()
}

// ════════════════════════════════════════════════════════════════════════════
// Tier-A — BLOCKING
// ════════════════════════════════════════════════════════════════════════════

// ────────────────────────────────────────────────────────────────────────────
// (1) P1 kill 1 — no wtool bypass
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_1_no_wtool_bypass() {
    let mut l = AcceptedLedger::new();
    for i in 1..=3 {
        l.append_accepted(&fixture_work_tx(i)).unwrap();
    }
    let canonical_root = l.current_state_root();

    let tmp = tempfile::NamedTempFile::new().unwrap();
    l.persist(tmp.path()).unwrap();

    // Bypass: directly overwrite state.db without going through L4.
    let raw = std::fs::read(tmp.path()).unwrap();
    let mut tampered: Vec<turingosv4::economy::ledger::AcceptedEntry> =
        serde_json::from_slice(&raw).unwrap();
    tampered.last_mut().unwrap().resulting_state_root = Hash([0xFF; 32]);
    let bytes = serde_json::to_vec(&tampered).unwrap();
    std::fs::write(tmp.path(), bytes).unwrap();

    // Reconstruction MUST fail: explicit error OR diverged root.
    match AcceptedLedger::load_from_path(tmp.path()) {
        Err(_) => {} // bypass detected — expected
        Ok((_, reconstructed)) => assert_ne!(
            reconstructed, canonical_root,
            "bypass mutation must not survive a round-trip through reconstruct_state"
        ),
    }
}

// ────────────────────────────────────────────────────────────────────────────
// (2) P1 kill 2 — rejected tx does not advance state
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_2_rejected_tx_no_state_advance() {
    let mut l4 = AcceptedLedger::new();
    let mut l4e = RejectionEvidenceWriter::new();

    l4.append_accepted(&fixture_work_tx(1)).unwrap();
    let baseline_root = l4.current_state_root();
    let baseline_logical_t = l4.len();

    l4e.append_rejected(
        42,
        baseline_root,
        agent("alice"),
        TxKind::Work,
        cid(0x20),
        RejectionClass::PredicateFailed,
        Some(cid(0xAA)),
        None,
    );

    assert_eq!(
        l4.current_state_root(),
        baseline_root,
        "rejected tx must NOT advance L4 state_root"
    );
    assert_eq!(
        l4.len(),
        baseline_logical_t,
        "rejected tx must NOT advance L4 logical_t"
    );

    assert_eq!(l4e.len(), 1, "rejection produces exactly one L4.E record");
    let r = &l4e.records()[0];
    assert_eq!(r.submit_id, 42);
    assert!(r.raw_diagnostic_cid.is_some());
    assert!(l4e.verify_chain().is_ok());
}

// ────────────────────────────────────────────────────────────────────────────
// (3) P1 kill 3 — ledger reconstructable
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_3_ledger_reconstructable() {
    let mut l = AcceptedLedger::new();
    for i in 1..=4 {
        l.append_accepted(&fixture_work_tx(i)).unwrap();
    }
    let pre_drop_root = l.current_state_root();

    let tmp = tempfile::NamedTempFile::new().unwrap();
    l.persist(tmp.path()).unwrap();

    drop(l);

    let (l_reborn, reconstructed_root) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
    assert_eq!(
        reconstructed_root, pre_drop_root,
        "reconstructed state_root must be bit-equal to pre-drop state_root"
    );
    assert_eq!(l_reborn.len(), 4);
    assert!(l_reborn.verify_chain(0, 4).is_ok());
}

// ────────────────────────────────────────────────────────────────────────────
// (4) P1 kill 4 — rejected log is isolated from agent-facing read view
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_4_rejected_log_isolated() {
    let mut l4e = RejectionEvidenceWriter::new();
    l4e.append_rejected(
        7,
        Hash::ZERO,
        agent("alice"),
        TxKind::Work,
        cid(0x10),
        RejectionClass::PredicateFailed,
        Some(cid(0xBE)),
        Some("predicate acceptance failed for acc-7".into()),
    );

    let view: Vec<PublicRejectionView> = l4e.public_view();
    assert_eq!(view.len(), 1);

    let json = serde_json::to_value(&view[0]).unwrap();
    let obj = json.as_object().expect("PublicRejectionView serializes as object");
    assert!(
        !obj.contains_key("raw_diagnostic_cid"),
        "raw_diagnostic_cid must NOT appear in agent-facing public view"
    );
    assert_eq!(
        obj.get("public_summary").and_then(|v| v.as_str()),
        Some("predicate acceptance failed for acc-7")
    );

    assert!(
        l4e.records()[0].raw_diagnostic_cid.is_some(),
        "L4.E forensic record must retain raw_diagnostic_cid (shielding is structural, not destructive)"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// (5) P1 Exit 7 — L4 hash chain breaks on row deletion
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_exit_7_l4_chain_breaks_on_row_deletion() {
    let mut l = AcceptedLedger::new();
    for i in 1..=5 {
        l.append_accepted(&fixture_work_tx(i)).unwrap();
    }
    assert!(l.verify_chain(0, 5).is_ok());

    l.tamper_remove_entry(2);

    let r = l.verify_chain(0, 4);
    match r {
        Err(LedgerError::LogicalTGap { at_index: 2, .. })
        | Err(LedgerError::HashMismatch { at_index: 2 }) => {}
        other => panic!(
            "deleting an L4 row must break the chain at index 2; got {:?}",
            other
        ),
    }
}

// ────────────────────────────────────────────────────────────────────────────
// (6) P1 kill 4b — L4.E hash chain breaks on row deletion
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_4b_rejection_chain_breaks_on_row_deletion() {
    let mut l4e = RejectionEvidenceWriter::new();
    for i in 1..=3u64 {
        l4e.append_rejected(
            i,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            None,
            None,
        );
    }
    assert!(l4e.verify_chain().is_ok());

    l4e.tamper_remove_record(1);
    let r = l4e.verify_chain();
    assert!(
        matches!(r, Err(RejectionEvidenceError::HashMismatch { at: 1 })),
        "deleting row 1 must surface as HashMismatch at the new index 1; got {:?}",
        r
    );
}

// ────────────────────────────────────────────────────────────────────────────
// (7) P3 RSP-0 Exit 1 — on_init total invariant across N tx sequence
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p3_rsp0_exit_1_on_init_total_invariant() {
    // After on_init mint, total_coin must be invariant across an arbitrary
    // sequence of redistribution txs (no further mints, no burns). We model
    // a 5-step alice/bob/escrow shuffle and assert assert_total_ctf_conserved
    // succeeds at each step with an empty exempt list.
    let mut s = EconomicState::default();
    s.balances_t.0.insert(agent("alice"), coin(100));
    let baseline = s.clone();

    // Step 1: alice → bob 30
    let mut s1 = EconomicState::default();
    s1.balances_t.0.insert(agent("alice"), coin(70));
    s1.balances_t.0.insert(agent("bob"), coin(30));
    assert_eq!(assert_total_ctf_conserved(&baseline, &s1, &[]), Ok(()));

    // Step 2: bob 30 → escrow
    use turingosv4::state::q_state::EscrowEntry;
    let mut s2 = EconomicState::default();
    s2.balances_t.0.insert(agent("alice"), coin(70));
    s2.escrows_t.0.insert(
        TxId("e-1".into()),
        EscrowEntry { amount: coin(30), depositor: agent("bob") },
    );
    assert_eq!(assert_total_ctf_conserved(&s1, &s2, &[]), Ok(()));

    // Step 3: escrow back to bob
    let mut s3 = EconomicState::default();
    s3.balances_t.0.insert(agent("alice"), coin(70));
    s3.balances_t.0.insert(agent("bob"), coin(30));
    assert_eq!(assert_total_ctf_conserved(&s2, &s3, &[]), Ok(()));

    // Step 4: alice 70 → carol
    let mut s4 = EconomicState::default();
    s4.balances_t.0.insert(agent("carol"), coin(70));
    s4.balances_t.0.insert(agent("bob"), coin(30));
    assert_eq!(assert_total_ctf_conserved(&s3, &s4, &[]), Ok(()));

    // Step 5: full round-trip back to baseline
    let mut s5 = EconomicState::default();
    s5.balances_t.0.insert(agent("alice"), coin(100));
    assert_eq!(assert_total_ctf_conserved(&s4, &s5, &[]), Ok(()));

    // Final cross-check: end == start (closed-system loop).
    assert_eq!(
        s5.balances_t.0.get(&agent("alice")),
        baseline.balances_t.0.get(&agent("alice")),
        "round-trip must restore baseline"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// (8) P3 RSP-0 Exit 2 — read-is-free (rtool / search / think MUST have fee=0)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p3_rsp0_exit_2_read_is_free() {
    // K5 has no dedicated read-tx variants today; the structural guard runs
    // against ALL TxKinds. assert_read_is_free(kind, fee=0) must succeed for
    // every variant; non-zero fee on ANY variant must surface as ReadCharged.
    for kind in [
        TxKind::Work,
        TxKind::Verify,
        TxKind::Challenge,
        TxKind::Reuse,
        TxKind::FinalizeReward,
        TxKind::TaskExpire,
        TxKind::TerminalSummary,
    ] {
        assert_eq!(
            assert_read_is_free(kind, 0),
            Ok(()),
            "fee=0 must pass for TxKind={:?}",
            kind
        );
    }

    // Anti-Goodhart: any non-zero fee at any kind is structurally rejected.
    assert_eq!(
        assert_read_is_free(TxKind::Reuse, 1),
        Err(MonetaryError::ReadCharged {
            tx_kind: TxKind::Reuse,
            fee: 1
        })
    );
    assert_eq!(
        assert_read_is_free(TxKind::Work, 9999),
        Err(MonetaryError::ReadCharged {
            tx_kind: TxKind::Work,
            fee: 9999
        })
    );
}

// ────────────────────────────────────────────────────────────────────────────
// (9) P3 kill 1 — no post-init mint (rejected route MUST go to L4.E, not L4)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p3_kill_1_no_post_init_mint() {
    // The numeric kill: any post-init mint surfaces as MonetaryError::PostInitMint.
    let before = EconomicState::default();
    let mut after = EconomicState::default();
    after.balances_t.0.insert(agent("alice"), coin(100));
    let r = assert_total_ctf_conserved(&before, &after, &[]);
    assert_eq!(
        r,
        Err(MonetaryError::PostInitMint {
            delta_micro: 100 * MICRO_PER_COIN
        }),
        "any non-exempt supply increase must surface as PostInitMint"
    );

    // The structural kill: L4.E (NOT L4) is the home for the rejection record.
    // Simulate the dispatch_transition rejection path by writing the rejection
    // ONLY to L4.E and asserting L4 is untouched.
    let l4 = AcceptedLedger::new();
    let pre_root = l4.current_state_root();
    let pre_logical_t = l4.len();

    let mut l4e = RejectionEvidenceWriter::new();
    l4e.append_rejected(
        99,
        pre_root,
        agent("alice"),
        TxKind::Work,
        cid(0x30),
        RejectionClass::InvariantViolation,
        Some(cid(0xC0)),
        Some("PostInitMint: delta_micro=100000000".into()),
    );

    // L4 untouched; L4.E has the record.
    assert_eq!(l4.current_state_root(), pre_root);
    assert_eq!(l4.len(), pre_logical_t);
    assert_eq!(l4e.len(), 1);
    assert!(matches!(
        l4e.records()[0].rejection_class,
        RejectionClass::InvariantViolation
    ));
}

// ════════════════════════════════════════════════════════════════════════════
// Tier-B — NON-BLOCKING (artifacts; do not gate ship)
// ════════════════════════════════════════════════════════════════════════════

// (10) AT-1 P6 anchor — evaluator solves mathd_algebra_107 in n3 mode.
//
// Verified out-of-band by the Day-4 live runs documented in commit 50a1d67:
// RUN 1 + RUN 2 both produced solved=true with gp_payload=nlinarith. Capturing
// this here as an #[ignore] live integration test so the assertion is REGISTERED
// in the harness even though it requires a running LLM proxy + DEEPSEEK_API_KEY
// to execute. Run manually with:
//   cargo test test_at1_evaluator_solves_mathd_algebra_107_n3 -- --ignored
#[test]
#[ignore = "Tier-B P6 anchor: requires live LLM proxy + DEEPSEEK_API_KEY; verified Day-4 (commit 50a1d67)"]
fn test_at1_evaluator_solves_mathd_algebra_107_n3() {
    // Live form would shell out to target/release/evaluator with CONDITION=n3
    // ACTIVE_MODEL=deepseek-chat MAX_TRANSACTIONS=10 and assert solved=true on
    // the JSONL row. Body intentionally empty — TB-1 ship gate is the manually-
    // observable Day-4 evidence in /tmp/tb1_day4_smoke_v2/run{1,2}.jsonl.
}

// (11) AT-2 — each tx in evaluator run produces an L4 LedgerEntry.
// Non-blocking until WorkTx dispatch_transition body lands at TB-2 RSP-1.
#[test]
#[ignore = "Tier-B: WorkTx dispatch_transition body lands TB-2 RSP-1; evaluator currently uses legacy emit path"]
fn test_at2_l4_entry_per_dispatched_tx() {
    // When TB-2 RSP-1 wires the WorkTx → AcceptedLedger::append_accepted path,
    // un-ignore this test and assert: for every successful evaluator tx, exactly
    // one L4 entry is appended; the entry's tx_payload_hash equals the tx's
    // canonical hash; verify_chain(0, n) succeeds at the end of the run.
}

// (12) AT-3 — h_vppu non-null on a 2nd-run row.
//
// The live form (2 evaluator invocations producing JSONL rows) is verified by
// the Day-4 evidence at /tmp/tb1_day4_smoke_v2/run2.jsonl (commit 50a1d67):
// run 2 carried `h_vppu=6.215891726697228`. The unit-level CONTRACT (capacity-3
// rolling history; record-then-query semantics; persistence round-trip) is
// covered by the 9 unit tests inside `minif2f_v4::h_vppu_history` itself
// (cargo test -p minif2f_v4 --lib h_vppu_history → 9/9 PASS).
//
// This file is a top-level integration test for the `turingosv4` crate and
// cannot import the `minif2f_v4` experiments crate (asymmetric path dep).
// Registering the AT-3 contract here as a `#[ignore]` documentation stub so
// the harness lists it explicitly; un-ignore + relocate to
// `experiments/minif2f_v4/tests/` if/when minif2f_v4 grows an integration
// test directory.
#[test]
#[ignore = "Tier-B AT-3: covered by minif2f_v4 lib tests + Day-4 live evidence (commit 50a1d67); cannot import minif2f_v4 from turingosv4 integration tests"]
fn test_at3_h_vppu_non_null_on_second_run() {
    // No body — see ignore reason above.
}

// (13) AT-4 — PputResult.econ_balance_delta non-zero.
// Non-blocking until TB-2 RSP-1's escrow_lock_tx + yes_stake_tx fire. RSP-0
// (Day-2) only proves the conservation invariant + scaffolds escrow/balances
// structures; actual non-zero deltas need the RSP-1 wiring.
#[test]
#[ignore = "Tier-B: needs RSP-1 escrow_lock_tx + yes_stake_tx wiring (TB-2)"]
fn test_at4_econ_balance_delta_non_zero() {
    // When TB-2 RSP-1 lands, un-ignore and assert: for an evaluator run that
    // exercises an escrow_lock_tx, PputResult.econ_balance_delta is Some(non-zero).
    // RSP-0 today only ships the conservation invariant + escrow_vault scaffolding.
}

```


---

## XREF: monetary_invariant.rs (Day-2) — `src/economy/monetary_invariant.rs`

```
//! Monetary invariant guards — TB-1 Day-2 P3 RSP-0.
//!
//! Charter authority:
//! - `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-2.
//! - ROADMAP P3 Exit 1, 2, 5 (`on_init` total Coin invariant; rtool/think
//!   don't deduct; escrow required for market admission).
//! - `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`:
//!   `MonetaryError` returns drive L4.E (rejection-evidence) entries, not L4.
//!
//! Constitutional authority:
//! - 基本法 1 (Coin 守恒): monetary conservation MUST be exact post-genesis.
//! - Inv 4 (no post-init mint): only `on_init` may inject coins; any other
//!   path that increases total CTF supply is a constitutional violation.
//! - Art. III.4 (selective shielding): rejection diagnostics route to L4.E.
//!
//! Scope (RSP-0 micro-version):
//! - Three pure assertion functions; no I/O, no state mutation.
//! - Wired into `dispatch_transition` rejection path in TB-1 Day-3.
//! - Tool-level read-is-free for `rtool` / `search` / `think` is enforced
//!   at the SDK boundary in a later RSP atom; this module covers the
//!   tx-level guarantee (no K5 `TypedTx` carries a per-tx fee).
//!
//! /// TRACE_MATRIX 基本法 1 + Inv 4 + ROADMAP P3:1/P3:2: monetary guards.

use crate::bottom_white::ledger::transition_ledger::TxKind;
use crate::economy::money::{MicroCoin, MICRO_PER_COIN};
use crate::state::q_state::{EconomicState, Hash, QState};
use crate::state::typed_tx::TypedTx;

// ────────────────────────────────────────────────────────────────────────────
// MonetaryError — invariant-violation taxonomy
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P3 RSP-0 — taxonomy of monetary invariant violations.
///
/// Variants are surfaced to the sequencer's rejection path; per the
/// L4 / L4.E split decision (`DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`)
/// they cause the offending transition to land in L4.E, NOT L4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonetaryError {
    /// Total CTF supply increased post-genesis. Reported by
    /// [`assert_total_ctf_conserved`] when `delta_micro > 0` and no
    /// exempting tx kind was declared.
    PostInitMint { delta_micro: i64 },
    /// Total CTF supply decreased without an exempting tx kind. Burns
    /// are not permitted in v1; this variant is reserved so a future
    /// RSP can opt in via `exempt_tx_kinds`.
    TotalCtfBurn { delta_micro: i64 },
    /// A K5 `TxKind` was assigned a non-zero per-tx fee. K5 has no
    /// fee field on any variant; only stake / bond exist (locked, not
    /// consumed). A non-zero fee is a structural constitutional bug.
    ReadCharged { tx_kind: TxKind, fee: u64 },
    /// Arithmetic overflow while summing economic state (i64).
    Overflow,
}

impl std::fmt::Display for MonetaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PostInitMint { delta_micro } => {
                write!(f, "post-init mint: total CTF supply increased by {} micro", delta_micro)
            }
            Self::TotalCtfBurn { delta_micro } => {
                write!(f, "unauthorized burn: total CTF supply decreased by {} micro", delta_micro)
            }
            Self::ReadCharged { tx_kind, fee } => {
                write!(f, "read charged: tx_kind={:?} carries fee={} (must be 0)", tx_kind, fee)
            }
            Self::Overflow => write!(f, "i64 overflow while summing economic state"),
        }
    }
}

impl std::error::Error for MonetaryError {}

// ────────────────────────────────────────────────────────────────────────────
// total_supply — sum of all coin-holding fields in EconomicState
// ────────────────────────────────────────────────────────────────────────────

/// Sum of every coin-holding sub-index in `EconomicState`, in micro-units.
///
/// Counted (each contributes its `MicroCoin` directly):
/// - `balances_t` (agent-held)
/// - `escrows_t` (locked under task)
/// - `stakes_t` (locked under tx)
/// - `claims_t` (pending payout)
/// - `task_markets_t.bounty` (sponsor-locked under task)
/// - `challenge_cases_t.bond` (challenger-locked under case)
///
/// NOT counted (not a holding):
/// - `reputations_t` (signed reputation, not coin)
/// - `royalty_graph_t` (edges, no coin)
/// - `price_index_t` (market data, not held)
fn total_supply_micro(s: &EconomicState) -> Result<i64, MonetaryError> {
    let mut total: i64 = 0;
    for v in s.balances_t.0.values() {
        total = total.checked_add(v.micro_units()).ok_or(MonetaryError::Overflow)?;
    }
    for e in s.escrows_t.0.values() {
        total = total.checked_add(e.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
    }
    for e in s.stakes_t.0.values() {
        total = total.checked_add(e.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
    }
    for c in s.claims_t.0.values() {
        total = total.checked_add(c.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
    }
    for m in s.task_markets_t.0.values() {
        total = total.checked_add(m.bounty.micro_units()).ok_or(MonetaryError::Overflow)?;
    }
    for c in s.challenge_cases_t.0.values() {
        total = total.checked_add(c.bond.micro_units()).ok_or(MonetaryError::Overflow)?;
    }
    Ok(total)
}

// ────────────────────────────────────────────────────────────────────────────
// assert_no_post_init_mint — structural guard at the tx layer
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P3:1 (kill 1) — structural guard against post-genesis mint
/// at the `TypedTx` layer.
///
/// **Today, K5 has no `Mint` variant** — none of the 7 `TypedTx` variants
/// directly inject coins. Genesis allocation happens in `on_init` outside
/// the K5 transition surface. Therefore, on a non-genesis `q`, this fn
/// returns `Ok(())` for every well-formed `TypedTx`.
///
/// **Why the function exists anyway**: it is a forward-compat barrier.
/// If a future RSP atom adds a `Mint` (or `SystemReward`-class) variant,
/// it MUST be added to the match below AND rejected here when
/// `q.state_root_t != Hash::ZERO`. Numeric conservation is enforced by
/// [`assert_total_ctf_conserved`] separately.
pub fn assert_no_post_init_mint(tx: &TypedTx, q: &QState) -> Result<(), MonetaryError> {
    let is_post_init = q.state_root_t != Hash::ZERO;
    if !is_post_init {
        return Ok(());
    }
    match tx {
        TypedTx::Work(_)
        | TypedTx::Verify(_)
        | TypedTx::Challenge(_)
        | TypedTx::Reuse(_)
        | TypedTx::FinalizeReward(_)
        | TypedTx::TaskExpire(_)
        | TypedTx::TerminalSummary(_) => Ok(()),
    }
}

// ────────────────────────────────────────────────────────────────────────────
// assert_total_ctf_conserved — numeric conservation across a transition
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX 基本法 1 + P3:1 — conservation of total CTF across a
/// transition `before → after`.
///
/// Mints (`delta > 0`) and burns (`delta < 0`) are both rejected unless
/// `exempt_tx_kinds` is non-empty. The exempt list is the explicit opt-out
/// for legitimate supply-changing operations (e.g., genesis init,
/// system-emitted rewards in a future RSP); RSP-0 never populates it
/// at runtime.
///
/// Caller convention: pass `&[]` for normal agent-submitted transitions.
/// Pass `&[TxKind::FinalizeReward]` (etc.) only when a system-emitted
/// supply-changing tx is being processed AND the RSP semantics for that
/// kind have been ratified. RSP-0 does not ratify any.
pub fn assert_total_ctf_conserved(
    before: &EconomicState,
    after: &EconomicState,
    exempt_tx_kinds: &[TxKind],
) -> Result<(), MonetaryError> {
    let total_before = total_supply_micro(before)?;
    let total_after = total_supply_micro(after)?;
    let delta = total_after
        .checked_sub(total_before)
        .ok_or(MonetaryError::Overflow)?;
    if !exempt_tx_kinds.is_empty() {
        return Ok(());
    }
    if delta > 0 {
        return Err(MonetaryError::PostInitMint { delta_micro: delta });
    }
    if delta < 0 {
        return Err(MonetaryError::TotalCtfBurn { delta_micro: delta });
    }
    Ok(())
}

// ────────────────────────────────────────────────────────────────────────────
// assert_read_is_free — tx-level no-fee guard
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P3:2 — assert that no K5 `TxKind` carries a per-tx fee.
///
/// K5 spec: every `TypedTx` variant has stake / bond fields (locked but
/// not consumed) but NO fee field. A non-zero `fee` is a structural bug
/// in whichever caller computed it; this fn is the barrier.
///
/// Note: tool-level read-is-free for `rtool` / `search` / `think` is
/// enforced at the SDK boundary in a later RSP atom (out of scope for
/// RSP-0). This fn covers the tx-level invariant only.
pub fn assert_read_is_free(tx_kind: TxKind, fee: u64) -> Result<(), MonetaryError> {
    if fee != 0 {
        return Err(MonetaryError::ReadCharged { tx_kind, fee });
    }
    Ok(())
}

// ────────────────────────────────────────────────────────────────────────────
// Tests
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::q_state::{AgentId, ClaimEntry, EscrowEntry, StakeEntry, TaskMarketEntry, TxId};
    use crate::state::typed_tx::{TaskId, WorkTx};

    fn agent(s: &str) -> AgentId {
        AgentId(s.to_string())
    }

    fn tx(s: &str) -> TxId {
        TxId(s.to_string())
    }

    fn task(s: &str) -> TaskId {
        TaskId(s.to_string())
    }

    fn coin(n: i64) -> MicroCoin {
        MicroCoin::from_coin(n).unwrap()
    }

    fn state_with_balance(holder: &str, amount_coin: i64) -> EconomicState {
        let mut s = EconomicState::default();
        s.balances_t.0.insert(agent(holder), coin(amount_coin));
        s
    }

    fn post_init_q() -> QState {
        let mut q = QState::default();
        // Any non-zero state_root counts as post-init.
        q.state_root_t = Hash::from_bytes([7u8; 32]);
        q
    }

    fn genesis_q() -> QState {
        QState::default()
    }

    // ── assert_no_post_init_mint ────────────────────────────────────────────

    #[test]
    fn no_post_init_mint_passes_on_genesis() {
        let q = genesis_q();
        let work = TypedTx::Work(WorkTx::default());
        assert_eq!(assert_no_post_init_mint(&work, &q), Ok(()));
    }

    #[test]
    fn no_post_init_mint_passes_for_all_k5_variants_post_init() {
        use crate::state::typed_tx::{
            ChallengeTx, FinalizeRewardTx, ReuseTx, TaskExpireTx, TerminalSummaryTx, VerifyTx,
        };
        let q = post_init_q();
        let cases: Vec<TypedTx> = vec![
            TypedTx::Work(WorkTx::default()),
            TypedTx::Verify(VerifyTx::default()),
            TypedTx::Challenge(ChallengeTx::default()),
            TypedTx::Reuse(ReuseTx::default()),
            TypedTx::FinalizeReward(FinalizeRewardTx::default()),
            TypedTx::TaskExpire(TaskExpireTx::default()),
            TypedTx::TerminalSummary(TerminalSummaryTx::default()),
        ];
        for t in cases {
            assert_eq!(assert_no_post_init_mint(&t, &q), Ok(()),
                "structural guard must pass for all K5 variants today");
        }
    }

    // ── assert_total_ctf_conserved ──────────────────────────────────────────

    #[test]
    fn ctf_conserved_balanced_transfer() {
        // Alice 100 → Bob 30 = 70/30 split; total unchanged.
        let mut before = EconomicState::default();
        before.balances_t.0.insert(agent("alice"), coin(100));
        let mut after = EconomicState::default();
        after.balances_t.0.insert(agent("alice"), coin(70));
        after.balances_t.0.insert(agent("bob"), coin(30));
        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
    }

    #[test]
    fn ctf_post_init_mint_rejected() {
        // P3 kill 1 (Day-2 unit form): supply increased without exempt.
        let before = state_with_balance("alice", 100);
        let mut after = before.clone();
        after.balances_t.0.insert(agent("alice"), coin(150));
        let r = assert_total_ctf_conserved(&before, &after, &[]);
        assert_eq!(
            r,
            Err(MonetaryError::PostInitMint { delta_micro: 50 * MICRO_PER_COIN })
        );
    }

    #[test]
    fn ctf_unauthorized_burn_rejected() {
        let before = state_with_balance("alice", 100);
        let mut after = before.clone();
        after.balances_t.0.insert(agent("alice"), coin(40));
        let r = assert_total_ctf_conserved(&before, &after, &[]);
        assert_eq!(
            r,
            Err(MonetaryError::TotalCtfBurn { delta_micro: -60 * MICRO_PER_COIN })
        );
    }

    #[test]
    fn ctf_exempt_short_circuits() {
        // With a non-empty exempt list (e.g., genesis init), supply may change.
        let before = EconomicState::default();
        let after = state_with_balance("alice", 1_000);
        assert_eq!(
            assert_total_ctf_conserved(&before, &after, &[TxKind::FinalizeReward]),
            Ok(())
        );
    }

    #[test]
    fn ctf_conserved_across_subindexes() {
        // 100 in balances → 60 in balances + 40 in escrow; total unchanged.
        let mut before = EconomicState::default();
        before.balances_t.0.insert(agent("alice"), coin(100));
        let mut after = EconomicState::default();
        after.balances_t.0.insert(agent("alice"), coin(60));
        after.escrows_t.0.insert(
            tx("work-1"),
            EscrowEntry { amount: coin(40), depositor: agent("alice") },
        );
        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
    }

    #[test]
    fn ctf_conserved_across_n10_random_sequence() {
        // Charter Day-2 unit: "total CTF conserved across N=10 random tx sequences".
        // We model 10 deterministic-but-varied conservative redistributions
        // (Alice/Bob/Carol; balances ↔ escrow ↔ stake ↔ claim ↔ market ↔ challenge).
        // Each step is a closed transfer; total supply is invariant.
        let mut s = EconomicState::default();
        s.balances_t.0.insert(agent("alice"), coin(100));
        s.balances_t.0.insert(agent("bob"), coin(50));
        s.balances_t.0.insert(agent("carol"), coin(25));
        let total0 = total_supply_micro(&s).unwrap();

        let steps: [(&str, i64); 10] = [
            ("alice->bob", 5),
            ("bob->escrow:t1", 10),
            ("alice->stake:tx1", 7),
            ("escrow:t1->claim:tx1", 3),
            ("alice->market:t2", 20),
            ("market:t2->balance:carol", 15),
            ("stake:tx1->challenge:case1", 4),
            ("challenge:case1->balance:bob", 2),
            ("claim:tx1->balance:alice", 3),
            ("balance:carol->escrow:t3", 6),
        ];

        let total_each = vec![total0; 10];
        for (i, (label, _amt)) in steps.iter().enumerate() {
            // Apply a small redistribution: move `_amt` micro_per_coin
            // between two slots. We just re-shuffle existing supply.
            // (Concrete redistribution mechanics live in SettlementEngine;
            // the invariant under test is: any closed redistribution leaves
            // total_supply_micro unchanged.)
            let amt_micro = (i as i64 + 1) * 1_000; // small, deterministic
            // Move `amt_micro` from alice's balance into a synthetic stake.
            let alice_bal = s.balances_t.0.get(&agent("alice"))
                .copied().unwrap_or(MicroCoin::zero());
            if alice_bal.micro_units() >= amt_micro {
                s.balances_t.0.insert(
                    agent("alice"),
                    MicroCoin::from_micro_units(alice_bal.micro_units() - amt_micro),
                );
                let key = tx(&format!("stake-step-{}", i));
                s.stakes_t.0.insert(
                    key,
                    StakeEntry { amount: MicroCoin::from_micro_units(amt_micro), staker: agent("alice") },
                );
            }
            let total_now = total_supply_micro(&s).unwrap();
            assert_eq!(
                total_now, total_each[i],
                "step {} ({}): conservation broke",
                i, label
            );
        }
        // Final cross-check.
        assert_eq!(total_supply_micro(&s).unwrap(), total0);
    }

    #[test]
    fn ctf_counts_all_six_holding_subindexes() {
        // Make sure we sum balances + escrows + stakes + claims + bounty + bond.
        let mut s = EconomicState::default();
        s.balances_t.0.insert(agent("a"), coin(1));
        s.escrows_t.0.insert(
            tx("e"),
            EscrowEntry { amount: coin(2), depositor: agent("a") },
        );
        s.stakes_t.0.insert(
            tx("s"),
            StakeEntry { amount: coin(4), staker: agent("a") },
        );
        s.claims_t.0.insert(
            tx("c"),
            ClaimEntry { amount: coin(8), claimant: agent("a") },
        );
        s.task_markets_t.0.insert(
            tx("m"),
            TaskMarketEntry {
                publisher: agent("a"),
                bounty: coin(16),
                ..Default::default()
            },
        );
        // challenge_cases_t bond
        let mut cc = crate::state::q_state::ChallengeCase::default();
        cc.bond = coin(32);
        cc.challenger = agent("a");
        s.challenge_cases_t.0.insert(tx("ch"), cc);

        // Each power of two distinct => sum = 63 base coin = 63_000_000 micro.
        assert_eq!(total_supply_micro(&s).unwrap(), 63 * MICRO_PER_COIN);
    }

    // ── assert_read_is_free ─────────────────────────────────────────────────

    #[test]
    fn read_is_free_zero_fee_passes_for_all_kinds() {
        for k in [
            TxKind::Work,
            TxKind::Verify,
            TxKind::Challenge,
            TxKind::Reuse,
            TxKind::FinalizeReward,
            TxKind::TaskExpire,
            TxKind::TerminalSummary,
        ] {
            assert_eq!(assert_read_is_free(k, 0), Ok(()));
        }
    }

    #[test]
    fn read_is_free_nonzero_fee_rejected() {
        // P3:2 — any per-tx fee on a K5 TxKind is a structural bug.
        let r = assert_read_is_free(TxKind::Reuse, 1);
        assert_eq!(r, Err(MonetaryError::ReadCharged { tx_kind: TxKind::Reuse, fee: 1 }));
        let r = assert_read_is_free(TxKind::Work, 9999);
        assert_eq!(r, Err(MonetaryError::ReadCharged { tx_kind: TxKind::Work, fee: 9999 }));
    }
}

```


---

## XREF: escrow_vault.rs (Day-2) — `src/economy/escrow_vault.rs`

```
//! Escrow vault — TB-1 Day-2 P3 RSP-0.
//!
//! Charter authority:
//! - `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-2.
//! - ROADMAP P3 Exit 5: an escrow lock is taken before `work_tx` is admitted.
//! - ROADMAP P3 Exit 6, 8: acceptance produces only `provisional_accept`,
//!   not full payout; `settlement_tx.payout_sum ≤ escrow_pool`.
//!
//! Constitutional authority:
//! - 基本法 1 (Coin 守恒): payouts come from pre-locked escrow; sum may
//!   not exceed locked amount; residual returns to sponsor.
//! - Inv 3 (escrow only): write-side mutations require prior escrow lock.
//!
//! Scope (RSP-0 micro-version):
//! - In-memory `BTreeMap<TaskId, VaultEntry>` keyed by task.
//! - `lock_escrow` records sponsor-locked supply for one task.
//! - `release_escrow` distributes payouts, asserts `Σ payouts ≤ locked`,
//!   computes residual-to-sponsor.
//! - No I/O, no L4 emission. Wiring into `dispatch_transition` /
//!   `SettlementEngine` lands in TB-1 Day-3 / TB-2 (RSP-1).
//!
//! Out of scope (deferred):
//! - Per-claim sub-vault accounting (RSP-2 SettlementEngine).
//! - Multi-sponsor co-escrow (RSP-3 ChallengeCourt).
//! - Persistence to L4 / state.db (TB-1 Day-3 wiring).
//!
//! /// TRACE_MATRIX 基本法 1 + Inv 3 + ROADMAP P3:5/P3:6/P3:8: escrow vault.

use std::collections::BTreeMap;

use crate::economy::money::MicroCoin;
use crate::state::q_state::AgentId;
use crate::state::typed_tx::TaskId;

// ────────────────────────────────────────────────────────────────────────────
// VaultEntry — one task's escrow record (status + payout log)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P3:5 — the lifecycle status of a task's escrow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscrowStatus {
    /// Funds locked under the task; awaiting release.
    Locked,
    /// Funds distributed; payout log is final; residual already
    /// computed and surfaced via [`VaultEntry::residual_to_sponsor`].
    Released,
}

/// TRACE_MATRIX P3:5 — single task's escrow record.
///
/// Distinct from `state::q_state::EscrowEntry` (which is the L4-state
/// tx-keyed projection). This vault is the operational task-keyed working
/// view used by the sequencer / SettlementEngine before flushing to L4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VaultEntry {
    pub task_id: TaskId,
    pub sponsor: AgentId,
    pub locked_amount: MicroCoin,
    pub status: EscrowStatus,
    /// Per-recipient payouts, populated on release.
    pub payouts: BTreeMap<AgentId, MicroCoin>,
    /// `locked_amount − Σ payouts`, populated on release.
    pub residual_to_sponsor: MicroCoin,
}

/// TRACE_MATRIX P3:5 — receipt returned by [`EscrowVault::lock_escrow`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EscrowReceipt {
    pub task_id: TaskId,
    pub sponsor: AgentId,
    pub locked_amount: MicroCoin,
}

/// TRACE_MATRIX P3:6/P3:8 — outcome returned by [`EscrowVault::release_escrow`].
///
/// Captures the per-recipient distribution and the residual that returns
/// to the sponsor. Caller is responsible for crediting the residual back
/// to the sponsor's `BalancesIndex` entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseOutcome {
    pub task_id: TaskId,
    pub sponsor: AgentId,
    pub paid_total: MicroCoin,
    pub residual_to_sponsor: MicroCoin,
}

// ────────────────────────────────────────────────────────────────────────────
// EscrowError — vault-operation errors
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P3 RSP-0 — errors returned by `EscrowVault` operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscrowError {
    /// `lock_escrow` called twice for the same `task_id` without an
    /// intervening release; rejected to keep escrow per-task unique.
    AlreadyLocked { task_id: TaskId },
    /// `release_escrow` called for a `task_id` with no prior lock.
    NotFound { task_id: TaskId },
    /// `release_escrow` called for a `task_id` already released.
    AlreadyReleased { task_id: TaskId },
    /// Σ payouts exceeds `locked_amount` (Inv 3 violation).
    PayoutExceedsLocked { task_id: TaskId, locked_micro: i64, requested_micro: i64 },
    /// A negative amount was passed (lock or payout); rejected at the
    /// vault layer to keep monetary math non-negative-by-default.
    NegativeAmount,
    /// i64 overflow while summing payouts or computing residual.
    Overflow,
}

impl std::fmt::Display for EscrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyLocked { task_id } => {
                write!(f, "escrow already locked for task {:?}", task_id.0)
            }
            Self::NotFound { task_id } => {
                write!(f, "no escrow lock for task {:?}", task_id.0)
            }
            Self::AlreadyReleased { task_id } => {
                write!(f, "escrow already released for task {:?}", task_id.0)
            }
            Self::PayoutExceedsLocked { task_id, locked_micro, requested_micro } => {
                write!(
                    f,
                    "payout exceeds locked for task {:?}: locked={} micro, requested={} micro",
                    task_id.0, locked_micro, requested_micro
                )
            }
            Self::NegativeAmount => write!(f, "negative monetary amount rejected"),
            Self::Overflow => write!(f, "i64 overflow in escrow arithmetic"),
        }
    }
}

impl std::error::Error for EscrowError {}

// ────────────────────────────────────────────────────────────────────────────
// EscrowVault — task-keyed in-memory vault (RSP-0)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P3 RSP-0 — minimum-viable in-memory escrow vault.
///
/// One entry per `TaskId`. Locking is one-shot per task in RSP-0
/// (multi-sponsor co-escrow is RSP-3). Release is one-shot and final.
#[derive(Debug, Clone, Default)]
pub struct EscrowVault {
    entries: BTreeMap<TaskId, VaultEntry>,
}

impl EscrowVault {
    /// TRACE_MATRIX P3 RSP-0 — empty vault constructor.
    pub fn new() -> Self {
        Self { entries: BTreeMap::new() }
    }

    /// TRACE_MATRIX P3 RSP-0 — number of recorded tasks (diagnostics-only accessor).
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// TRACE_MATRIX P3 RSP-0 — emptiness predicate (diagnostics-only accessor).
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// TRACE_MATRIX P3 RSP-0 — read-only entry lookup; backs residual / payout assertions.
    pub fn get(&self, task_id: &TaskId) -> Option<&VaultEntry> {
        self.entries.get(task_id)
    }

    /// TRACE_MATRIX P3:5 — record sponsor-locked supply for one task.
    ///
    /// Returns `EscrowError::AlreadyLocked` if a prior lock exists for the
    /// same `task_id` (whether `Locked` or `Released`); a released task
    /// cannot be re-locked under RSP-0 semantics.
    /// Returns `EscrowError::NegativeAmount` for negative `amount`.
    pub fn lock_escrow(
        &mut self,
        task_id: TaskId,
        sponsor: AgentId,
        amount: MicroCoin,
    ) -> Result<EscrowReceipt, EscrowError> {
        if amount.is_negative() {
            return Err(EscrowError::NegativeAmount);
        }
        if self.entries.contains_key(&task_id) {
            return Err(EscrowError::AlreadyLocked { task_id });
        }
        let entry = VaultEntry {
            task_id: task_id.clone(),
            sponsor: sponsor.clone(),
            locked_amount: amount,
            status: EscrowStatus::Locked,
            payouts: BTreeMap::new(),
            residual_to_sponsor: MicroCoin::zero(),
        };
        self.entries.insert(task_id.clone(), entry);
        Ok(EscrowReceipt { task_id, sponsor, locked_amount: amount })
    }

    /// TRACE_MATRIX P3:6/P3:8 — distribute payouts, assert `Σ payouts ≤ locked`,
    /// compute residual-to-sponsor, mark task `Released`.
    ///
    /// Empty `payouts` is permitted (full residual returns to sponsor — the
    /// `TaskExpire` shape).
    /// Negative payout amounts are rejected (`NegativeAmount`).
    /// `Σ payouts > locked` → `PayoutExceedsLocked` and the entry is left
    /// in `Locked` status (operation atomic-rejected).
    pub fn release_escrow(
        &mut self,
        task_id: &TaskId,
        payouts: &BTreeMap<AgentId, MicroCoin>,
    ) -> Result<ReleaseOutcome, EscrowError> {
        let entry = self
            .entries
            .get_mut(task_id)
            .ok_or_else(|| EscrowError::NotFound { task_id: task_id.clone() })?;

        if entry.status == EscrowStatus::Released {
            return Err(EscrowError::AlreadyReleased { task_id: task_id.clone() });
        }

        let mut total_paid = MicroCoin::zero();
        for amt in payouts.values() {
            if amt.is_negative() {
                return Err(EscrowError::NegativeAmount);
            }
            total_paid = total_paid
                .checked_add(*amt)
                .ok_or(EscrowError::Overflow)?;
        }

        if total_paid.micro_units() > entry.locked_amount.micro_units() {
            return Err(EscrowError::PayoutExceedsLocked {
                task_id: task_id.clone(),
                locked_micro: entry.locked_amount.micro_units(),
                requested_micro: total_paid.micro_units(),
            });
        }

        let residual = entry
            .locked_amount
            .checked_sub(total_paid)
            .ok_or(EscrowError::Overflow)?;

        entry.payouts = payouts.clone();
        entry.residual_to_sponsor = residual;
        entry.status = EscrowStatus::Released;

        Ok(ReleaseOutcome {
            task_id: task_id.clone(),
            sponsor: entry.sponsor.clone(),
            paid_total: total_paid,
            residual_to_sponsor: residual,
        })
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Tests
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn agent(s: &str) -> AgentId {
        AgentId(s.to_string())
    }

    fn task(s: &str) -> TaskId {
        TaskId(s.to_string())
    }

    fn coin(n: i64) -> MicroCoin {
        MicroCoin::from_coin(n).unwrap()
    }

    // ── lock ────────────────────────────────────────────────────────────────

    #[test]
    fn lock_records_entry_and_returns_receipt() {
        let mut v = EscrowVault::new();
        let r = v
            .lock_escrow(task("t1"), agent("alice"), coin(100))
            .unwrap();
        assert_eq!(r.task_id, task("t1"));
        assert_eq!(r.sponsor, agent("alice"));
        assert_eq!(r.locked_amount, coin(100));
        let e = v.get(&task("t1")).unwrap();
        assert_eq!(e.status, EscrowStatus::Locked);
        assert_eq!(e.locked_amount, coin(100));
        assert_eq!(e.residual_to_sponsor, MicroCoin::zero());
        assert!(e.payouts.is_empty());
    }

    #[test]
    fn lock_rejects_double_lock_same_task() {
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
        let r = v.lock_escrow(task("t1"), agent("bob"), coin(50));
        assert_eq!(r, Err(EscrowError::AlreadyLocked { task_id: task("t1") }));
    }

    #[test]
    fn lock_rejects_negative_amount() {
        let mut v = EscrowVault::new();
        let r = v.lock_escrow(
            task("t1"),
            agent("alice"),
            MicroCoin::from_micro_units(-1),
        );
        assert_eq!(r, Err(EscrowError::NegativeAmount));
    }

    // ── release ─────────────────────────────────────────────────────────────

    #[test]
    fn release_overpayout_rejected() {
        // Charter Day-2 unit: "escrow over-payout rejected".
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
        let mut payouts = BTreeMap::new();
        payouts.insert(agent("solver"), coin(60));
        payouts.insert(agent("verifier"), coin(50));
        let r = v.release_escrow(&task("t1"), &payouts);
        assert!(matches!(r, Err(EscrowError::PayoutExceedsLocked { .. })));
        // Atomic rejection: entry still Locked, no payouts recorded.
        let e = v.get(&task("t1")).unwrap();
        assert_eq!(e.status, EscrowStatus::Locked);
        assert!(e.payouts.is_empty());
    }

    #[test]
    fn release_underpayout_residual_returns_to_sponsor() {
        // Charter Day-2 unit: "escrow under-payout accepted (residual returns to sponsor)".
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
        let mut payouts = BTreeMap::new();
        payouts.insert(agent("solver"), coin(60));
        payouts.insert(agent("verifier"), coin(10));
        let outcome = v.release_escrow(&task("t1"), &payouts).unwrap();
        assert_eq!(outcome.paid_total, coin(70));
        assert_eq!(outcome.residual_to_sponsor, coin(30));
        assert_eq!(outcome.sponsor, agent("alice"));
        let e = v.get(&task("t1")).unwrap();
        assert_eq!(e.status, EscrowStatus::Released);
        assert_eq!(e.residual_to_sponsor, coin(30));
        assert_eq!(e.payouts.len(), 2);
    }

    #[test]
    fn release_exact_payout_zero_residual() {
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
        let mut payouts = BTreeMap::new();
        payouts.insert(agent("solver"), coin(70));
        payouts.insert(agent("verifier"), coin(30));
        let outcome = v.release_escrow(&task("t1"), &payouts).unwrap();
        assert_eq!(outcome.paid_total, coin(100));
        assert_eq!(outcome.residual_to_sponsor, MicroCoin::zero());
    }

    #[test]
    fn release_empty_payouts_full_residual_to_sponsor() {
        // TaskExpire shape: deadline lapsed, no winners; full bounty refunds.
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
        let payouts = BTreeMap::new();
        let outcome = v.release_escrow(&task("t1"), &payouts).unwrap();
        assert_eq!(outcome.paid_total, MicroCoin::zero());
        assert_eq!(outcome.residual_to_sponsor, coin(100));
    }

    #[test]
    fn release_unknown_task_rejected() {
        let mut v = EscrowVault::new();
        let r = v.release_escrow(&task("t1"), &BTreeMap::new());
        assert_eq!(r, Err(EscrowError::NotFound { task_id: task("t1") }));
    }

    #[test]
    fn release_after_release_rejected() {
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
        v.release_escrow(&task("t1"), &BTreeMap::new()).unwrap();
        let r = v.release_escrow(&task("t1"), &BTreeMap::new());
        assert_eq!(r, Err(EscrowError::AlreadyReleased { task_id: task("t1") }));
    }

    #[test]
    fn release_negative_payout_rejected() {
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
        let mut payouts = BTreeMap::new();
        payouts.insert(agent("solver"), MicroCoin::from_micro_units(-1));
        let r = v.release_escrow(&task("t1"), &payouts);
        assert_eq!(r, Err(EscrowError::NegativeAmount));
        // Atomic-reject: still Locked.
        assert_eq!(v.get(&task("t1")).unwrap().status, EscrowStatus::Locked);
    }

    // ── multi-task isolation ────────────────────────────────────────────────

    #[test]
    fn multi_task_independent() {
        let mut v = EscrowVault::new();
        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
        v.lock_escrow(task("t2"), agent("bob"), coin(50)).unwrap();
        assert_eq!(v.len(), 2);
        let mut p1 = BTreeMap::new();
        p1.insert(agent("solver1"), coin(40));
        v.release_escrow(&task("t1"), &p1).unwrap();
        // t2 must still be locked.
        assert_eq!(v.get(&task("t2")).unwrap().status, EscrowStatus::Locked);
        assert_eq!(v.get(&task("t1")).unwrap().status, EscrowStatus::Released);
    }
}

```


---

## XREF: ledger.rs (L4; Day-3) — `src/economy/ledger.rs`

```
//! L4 accepted-only ledger wrapper — TB-1 Day-3 P1.
//!
//! Charter authority:
//! - `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-3.
//! - ROADMAP P1 Exit 5 (state_root advances on accept), Exit 6 (state_root
//!   unchanged on reject), Exit 7 (ledger hash chain), Exit 8 (state.db
//!   reconstructable from chaintape).
//! - `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`:
//!   accepted transitions ONLY land here; rejected submissions go to L4.E
//!   (`bottom_white::ledger::rejection_evidence`).
//!
//! Constitutional authority:
//! - WP § 5.L4 — ChainTape Layer 4 spine; one entry per accepted transition.
//! - Art IV (Boot) — every Q_t field MUST be reconstructible by replaying L4.
//! - Inv 7 (no rejection on the accepted spine) — rejections never advance
//!   `state_root_t` / `ledger_root_t`.
//!
//! Scope (RSP-0 minimum-viable wrapper):
//! - Self-contained accepted-only hash chain over `TypedTx` canonical bytes.
//! - `append_accepted` advances `logical_t` and chains `prev_hash`.
//! - `verify_chain(start, end)` walks the hash chain over `[start, end)`.
//! - `reconstruct_state` replays L4 only and returns the canonical
//!   `state_root_t` (L4.E is intentionally NOT consulted).
//! - Persistence helpers (`persist` / `load_from_path`) provide the
//!   "drop state.db; reconstruct from L4" round-trip used by P1 kill
//!   acceptance tests.
//!
//! Out of scope (deferred to CO1.7.5+):
//! - `SystemSignature` attachment (full signing payload + epoch binding).
//! - `dispatch_transition` re-run (state_root mutation requires CO1.8).
//! - Real `Git2LedgerWriter` commit chain — that's the production backend
//!   over `refs/transitions/main`; this RSP-0 wrapper uses an in-memory Vec.
//!
//! /// TRACE_MATRIX WP § 5.L4 + Art IV + ROADMAP P1:5/P1:6/P1:7/P1:8: L4 accepted-only ledger.

use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bottom_white::ledger::transition_ledger::{canonical_encode, TxKind};
use crate::state::q_state::Hash;
use crate::state::typed_tx::TypedTx;

// ────────────────────────────────────────────────────────────────────────────
// AcceptedEntry — one row on the L4 accepted-only chain
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:5 — one accepted-only L4 row.
///
/// All seven fields enter the hash; tampering any single field breaks
/// `verify_chain` at the affected index. The `tx_payload_hash` is the
/// SHA-256 over the bincode-canonical encoding of the source `TypedTx`,
/// re-using the lower-level `canonical_encode` from `transition_ledger`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcceptedEntry {
    /// 1-based monotonic counter; advances ONLY on accept (not on reject —
    /// rejections take a `submit_id` on L4.E instead, per the L4/L4.E split).
    pub logical_t: u64,
    /// Hash of the immediately-preceding entry; `Hash::ZERO` for the first row.
    pub prev_hash: Hash,
    /// Discriminator over the source `TypedTx` variant.
    pub tx_kind: TxKind,
    /// SHA-256 of `canonical_encode(tx)` — content-address of the payload.
    pub tx_payload_hash: Hash,
    /// State-root before this entry was applied.
    pub parent_state_root: Hash,
    /// State-root after this entry was applied. Computed by `next_state_root`
    /// (the RSP-0 toy mutator); a real `dispatch_transition` lands in CO1.7.5.
    pub resulting_state_root: Hash,
    /// SHA-256 over the six fields above plus a domain-separation prefix.
    pub hash: Hash,
}

impl AcceptedEntry {
    fn compute_hash(
        logical_t: u64,
        prev_hash: &Hash,
        tx_kind: TxKind,
        tx_payload_hash: &Hash,
        parent_state_root: &Hash,
        resulting_state_root: &Hash,
    ) -> Hash {
        let mut h = Sha256::new();
        h.update(b"turingosv4.l4_accepted.v1");
        h.update(logical_t.to_be_bytes());
        h.update(prev_hash.0);
        h.update((tx_kind as u8).to_be_bytes());
        h.update(tx_payload_hash.0);
        h.update(parent_state_root.0);
        h.update(resulting_state_root.0);
        Hash(h.finalize().into())
    }
}

// ────────────────────────────────────────────────────────────────────────────
// LedgerError — shared error taxonomy for append / verify / reconstruct
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:5/P1:6/P1:7/P1:8 — error taxonomy for the L4 wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    /// `verify_chain` walked off the end of `entries`.
    OutOfBounds { len: usize, requested_end: usize },
    /// Hash mismatch at the given chain index (prev_hash break OR entry hash break).
    HashMismatch { at_index: usize },
    /// `logical_t` is not the expected `index + 1` value.
    LogicalTGap { at_index: usize, expected: u64, got: u64 },
    /// `parent_state_root` doesn't match the running replay state.
    ParentStateMismatch { at_index: usize },
    /// `canonical_encode` of the source `TypedTx` failed.
    Encode(String),
    /// File system or JSON serialization error during persist / load.
    Io(String),
}

impl std::fmt::Display for LedgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBounds { len, requested_end } => {
                write!(f, "verify_chain end={} exceeds chain len={}", requested_end, len)
            }
            Self::HashMismatch { at_index } => {
                write!(f, "L4 hash chain break at index {}", at_index)
            }
            Self::LogicalTGap { at_index, expected, got } => write!(
                f,
                "logical_t gap at index {}: expected {}, got {}",
                at_index, expected, got
            ),
            Self::ParentStateMismatch { at_index } => {
                write!(f, "parent_state_root mismatch at index {}", at_index)
            }
            Self::Encode(e) => write!(f, "canonical_encode failed: {}", e),
            Self::Io(e) => write!(f, "persistence I/O failed: {}", e),
        }
    }
}

impl std::error::Error for LedgerError {}

/// TRACE_MATRIX P1:7 — `verify_chain` failure alias; kept distinct from
/// `ReconstructError` so callers can pattern-match on chain-walk vs replay.
pub type ChainError = LedgerError;
/// TRACE_MATRIX P1:8 — `reconstruct_state` / `load_from_path` failure alias;
/// distinct from `ChainError` so replay errors are syntactically separable.
pub type ReconstructError = LedgerError;

// ────────────────────────────────────────────────────────────────────────────
// AcceptedLedger — the wrapper itself
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:5/P1:6/P1:7/P1:8 — accepted-only L4 hash chain (RSP-0).
///
/// Single source of truth for the accepted spine. Rejected transitions
/// MUST NOT touch this struct; they take a `submit_id` on L4.E
/// (`bottom_white::ledger::rejection_evidence`).
#[derive(Debug, Clone, Default)]
pub struct AcceptedLedger {
    entries: Vec<AcceptedEntry>,
    current_state_root: Hash,
}

impl AcceptedLedger {
    /// TRACE_MATRIX Art IV Boot — empty L4 (genesis state_root is `Hash::ZERO`).
    pub fn new() -> Self {
        Self::default()
    }

    /// TRACE_MATRIX P1:5 — count of accepted rows.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// TRACE_MATRIX P1:5 — empty predicate.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// TRACE_MATRIX P1:5 — current canonical `state_root_t`.
    pub fn current_state_root(&self) -> Hash {
        self.current_state_root
    }

    /// TRACE_MATRIX P1:5 — append-accepted entry; advances `logical_t` by 1.
    ///
    /// Advances `current_state_root` via the toy mutator `next_state_root`.
    /// Returns the freshly-built `AcceptedEntry` (clone of what was pushed).
    pub fn append_accepted(&mut self, tx: &TypedTx) -> Result<AcceptedEntry, LedgerError> {
        let bytes = canonical_encode(tx).map_err(|e| LedgerError::Encode(e.to_string()))?;
        let tx_payload_hash = sha256_of(&bytes);
        let prev_hash = self.entries.last().map(|e| e.hash).unwrap_or(Hash::ZERO);
        let logical_t = (self.entries.len() as u64) + 1;
        let parent_state_root = self.current_state_root;
        let tx_kind = tx.tx_kind();
        let resulting_state_root = next_state_root(&parent_state_root, &tx_payload_hash);
        let hash = AcceptedEntry::compute_hash(
            logical_t,
            &prev_hash,
            tx_kind,
            &tx_payload_hash,
            &parent_state_root,
            &resulting_state_root,
        );
        let entry = AcceptedEntry {
            logical_t,
            prev_hash,
            tx_kind,
            tx_payload_hash,
            parent_state_root,
            resulting_state_root,
            hash,
        };
        self.entries.push(entry.clone());
        self.current_state_root = resulting_state_root;
        Ok(entry)
    }

    /// TRACE_MATRIX P1:7 — verify hash-chain integrity over `[start, end)`.
    ///
    /// Returns `Err(HashMismatch)` if any single field (logical_t, prev_hash,
    /// tx_payload_hash, parent_state_root, resulting_state_root, tx_kind, or
    /// the entry hash itself) was tampered.
    pub fn verify_chain(&self, start: usize, end: usize) -> Result<(), ChainError> {
        if end > self.entries.len() {
            return Err(LedgerError::OutOfBounds {
                len: self.entries.len(),
                requested_end: end,
            });
        }
        if start > end {
            return Err(LedgerError::OutOfBounds {
                len: self.entries.len(),
                requested_end: start,
            });
        }
        let mut prev = if start == 0 {
            Hash::ZERO
        } else {
            self.entries[start - 1].hash
        };
        for i in start..end {
            let e = &self.entries[i];
            let expected_logical_t = (i as u64) + 1;
            if e.logical_t != expected_logical_t {
                return Err(LedgerError::LogicalTGap {
                    at_index: i,
                    expected: expected_logical_t,
                    got: e.logical_t,
                });
            }
            if e.prev_hash != prev {
                return Err(LedgerError::HashMismatch { at_index: i });
            }
            let recomputed = AcceptedEntry::compute_hash(
                e.logical_t,
                &e.prev_hash,
                e.tx_kind,
                &e.tx_payload_hash,
                &e.parent_state_root,
                &e.resulting_state_root,
            );
            if recomputed != e.hash {
                return Err(LedgerError::HashMismatch { at_index: i });
            }
            prev = e.hash;
        }
        Ok(())
    }

    /// TRACE_MATRIX P1:8 — replay L4 only; recompute the canonical `state_root_t`.
    ///
    /// L4.E is intentionally NOT consulted: rejected submissions never affect
    /// `state_root_t` (Inv 7).
    pub fn reconstruct_state(&self) -> Result<Hash, ReconstructError> {
        let mut s = Hash::ZERO;
        for (i, e) in self.entries.iter().enumerate() {
            if e.parent_state_root != s {
                return Err(LedgerError::ParentStateMismatch { at_index: i });
            }
            let expected = next_state_root(&s, &e.tx_payload_hash);
            if e.resulting_state_root != expected {
                return Err(LedgerError::HashMismatch { at_index: i });
            }
            s = e.resulting_state_root;
        }
        Ok(s)
    }

    /// TRACE_MATRIX P1:8 — persist entries to `state_path` for cold restart.
    pub fn persist(&self, state_path: &Path) -> Result<(), LedgerError> {
        let bytes = serde_json::to_vec(&self.entries).map_err(|e| LedgerError::Io(e.to_string()))?;
        std::fs::write(state_path, bytes).map_err(|e| LedgerError::Io(e.to_string()))?;
        Ok(())
    }

    /// TRACE_MATRIX P1:8 — load entries from `state_path` and recompute the
    /// canonical `state_root_t`. Used by the "drop state.db; reconstruct from L4"
    /// kill test: any direct mutation that bypassed the L4 path is washed out.
    pub fn load_from_path(state_path: &Path) -> Result<(Self, Hash), ReconstructError> {
        let bytes = std::fs::read(state_path).map_err(|e| LedgerError::Io(e.to_string()))?;
        let entries: Vec<AcceptedEntry> =
            serde_json::from_slice(&bytes).map_err(|e| LedgerError::Io(e.to_string()))?;
        let mut l = Self {
            entries,
            current_state_root: Hash::ZERO,
        };
        let s = l.reconstruct_state()?;
        l.current_state_root = s;
        Ok((l, s))
    }

    /// TRACE_MATRIX P1:7 — read-only entry slice (for replay / debug / external
    /// tooling that wants to inspect the chain without mutating it).
    pub fn entries(&self) -> &[AcceptedEntry] {
        &self.entries
    }

    /// TRACE_MATRIX P1:7 — TAMPER-ONLY hook used by kill-criteria integration
    /// tests to simulate adversarial row deletion. The `tamper_` prefix and
    /// `#[doc(hidden)]` mark this as not part of the supported API; production
    /// callers MUST NOT use it. Kept `pub` (rather than `cfg(test)`) only so
    /// integration tests in `tests/` can reach it; integration tests link
    /// against the lib without `cfg(test)` enabled.
    #[doc(hidden)]
    pub fn tamper_remove_entry(&mut self, idx: usize) {
        self.entries.remove(idx);
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Helpers
// ────────────────────────────────────────────────────────────────────────────

fn sha256_of(bytes: &[u8]) -> Hash {
    let mut h = Sha256::new();
    h.update(bytes);
    Hash(h.finalize().into())
}

/// RSP-0 toy state mutator: `next = SHA-256(domain || prev_state_root || tx_payload_hash)`.
///
/// This is a minimum-viable demonstration of the state-root-advances-on-accept
/// invariant. The real `dispatch_transition`-driven state_root mutation lands
/// in CO1.7.5 / CO1.8 (proper economic + agent-swarm state evolution).
fn next_state_root(prev: &Hash, tx_payload_hash: &Hash) -> Hash {
    let mut h = Sha256::new();
    h.update(b"turingosv4.l4_state_root.v1");
    h.update(prev.0);
    h.update(tx_payload_hash.0);
    Hash(h.finalize().into())
}

// ────────────────────────────────────────────────────────────────────────────
// Inline correctness tests (round-trip + tamper detection on every field).
// Cross-cutting P1 kill acceptance tests live in `tests/tb_1_p1_acceptance.rs`.
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::q_state::{AgentId, TxId};
    use crate::state::typed_tx::{
        AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey,
        SafetyOrCreation, TaskId, TypedTx, WorkTx, WriteKey,
    };
    use crate::bottom_white::cas::schema::Cid;
    use crate::economy::money::StakeMicroCoin;
    use std::collections::{BTreeMap, BTreeSet};

    fn fixture_work_tx(suffix: u32) -> TypedTx {
        let mut acceptance = BTreeMap::new();
        acceptance.insert(
            PredicateId(format!("acc-{}", suffix)),
            BoolWithProof { value: true, proof_cid: Some(Cid([0x11; 32])) },
        );
        let mut settlement = BTreeMap::new();
        settlement.insert(
            PredicateId(format!("set-{}", suffix)),
            BoolWithProof { value: true, proof_cid: None },
        );
        let mut read_set = BTreeSet::new();
        read_set.insert(ReadKey(format!("k.r.{}", suffix)));
        let mut write_set = BTreeSet::new();
        write_set.insert(WriteKey(format!("k.w.{}", suffix)));
        TypedTx::Work(WorkTx {
            tx_id: TxId(format!("worktx-{}", suffix)),
            task_id: TaskId(format!("task-{}", suffix)),
            parent_state_root: Hash::ZERO,
            agent_id: AgentId("alice".into()),
            read_set,
            write_set,
            proposal_cid: Cid([0x13; 32]),
            predicate_results: PredicateResultsBundle {
                acceptance,
                settlement,
                safety_class: SafetyOrCreation::Safety,
            },
            stake: StakeMicroCoin::from_micro_units(1_000_000),
            signature: AgentSignature::from_bytes([0x77u8; 64]),
            timestamp_logical: suffix as u64,
        })
    }

    #[test]
    fn append_advances_logical_t_and_state_root() {
        let mut l = AcceptedLedger::new();
        assert_eq!(l.len(), 0);
        assert_eq!(l.current_state_root(), Hash::ZERO);

        let e1 = l.append_accepted(&fixture_work_tx(1)).unwrap();
        assert_eq!(e1.logical_t, 1);
        assert_eq!(e1.prev_hash, Hash::ZERO);
        assert_eq!(e1.parent_state_root, Hash::ZERO);
        assert_ne!(e1.resulting_state_root, Hash::ZERO);
        assert_eq!(l.current_state_root(), e1.resulting_state_root);

        let e2 = l.append_accepted(&fixture_work_tx(2)).unwrap();
        assert_eq!(e2.logical_t, 2);
        assert_eq!(e2.prev_hash, e1.hash);
        assert_eq!(e2.parent_state_root, e1.resulting_state_root);
    }

    #[test]
    fn verify_chain_passes_on_clean_chain() {
        let mut l = AcceptedLedger::new();
        for i in 1..=5 {
            l.append_accepted(&fixture_work_tx(i)).unwrap();
        }
        assert!(l.verify_chain(0, 5).is_ok());
        assert!(l.verify_chain(0, 0).is_ok());
        assert!(l.verify_chain(2, 4).is_ok());
    }

    #[test]
    fn verify_chain_out_of_bounds_rejected() {
        let mut l = AcceptedLedger::new();
        l.append_accepted(&fixture_work_tx(1)).unwrap();
        let r = l.verify_chain(0, 5);
        assert!(matches!(r, Err(LedgerError::OutOfBounds { .. })));
    }

    #[test]
    fn reconstruct_state_round_trip() {
        let mut l = AcceptedLedger::new();
        for i in 1..=4 {
            l.append_accepted(&fixture_work_tx(i)).unwrap();
        }
        let pre = l.current_state_root();
        let reconstructed = l.reconstruct_state().unwrap();
        assert_eq!(pre, reconstructed);
    }

    #[test]
    fn persist_and_load_round_trip() {
        let mut l = AcceptedLedger::new();
        for i in 1..=3 {
            l.append_accepted(&fixture_work_tx(i)).unwrap();
        }
        let pre = l.current_state_root();

        let tmp = tempfile::NamedTempFile::new().unwrap();
        l.persist(tmp.path()).unwrap();
        let (l2, post) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
        assert_eq!(pre, post);
        assert_eq!(l2.len(), 3);
    }
}

```


---

## XREF: rejection_evidence.rs (L4.E; Day-3) — `src/bottom_white/ledger/rejection_evidence.rs`

```
//! L4.E rejection-evidence ledger — TB-1 Day-3 P1.
//!
//! Charter authority:
//! - `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-3.
//! - `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`
//!   (architectural commitment to L4 / L4.E split, post external audit
//!   2026-04-29 CF-1).
//! - ROADMAP P1 Exit 6 (rejected tx ≠ state_root advance), Exit 9
//!   (rejected log not visible in another agent's read view).
//!
//! Constitutional authority:
//! - Inv 7 — accepted spine and rejection-evidence are disjoint ledgers;
//!   rejections never mutate `state_root_t` / `ledger_root_t`.
//! - Inv 10 (Goodhart shield) — raw rejection diagnostics are isolated
//!   from agent-facing materialized views; only `public_summary` is
//!   permitted to cross the agent boundary.
//! - Art. III.4 (selective shielding) — rejection raw content is shielded
//!   by default; explicit opt-in via `public_summary`.
//!
//! Scope (RSP-0 minimum-viable):
//! - In-memory `Vec<RejectedSubmissionRecord>` chained via `prev_hash`.
//! - `submit_id` (NOT `logical_t`) keys each record per the L4 / L4.E split:
//!   accepted spine takes the canonical counter; rejection-evidence carries
//!   an independent submit-side counter from `Sequencer::next_submit_id`.
//! - `raw_diagnostic_cid` is a CAS handle to the raw error bytes; the
//!   `PublicRejectionView` projection (used to materialize agent-facing
//!   read views) DOES NOT carry that field — structural shielding rather
//!   than runtime access-control.
//!
//! Out of scope (deferred):
//! - Persistence backend (Git2 commit chain on `refs/rejections/main` —
//!   future RSP / TB).
//! - SystemSignature attestation per record (CO1.7.5+ when system_keypair
//!   gets a `CanonicalMessage::RejectionEvidence` variant).
//! - Cross-agent visibility policy machinery (CO P2.7).
//!
//! /// TRACE_MATRIX Inv 7 + Inv 10 + ROADMAP P1:6/P1:9: L4.E rejection-evidence ledger.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::ledger::transition_ledger::TxKind;
use crate::state::q_state::{AgentId, Hash};

// ────────────────────────────────────────────────────────────────────────────
// RejectionClass — taxonomy of why a submission was rejected
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:6 — coarse rejection-class discriminator.
///
/// Stable byte-encoding via `#[repr(u8)]` so the discriminator can ride into
/// the canonical hash deterministically across compiler versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RejectionClass {
    /// A `top_white::predicates` acceptance gate returned `false`.
    PredicateFailed = 0,
    /// A higher-level policy gate (visibility / quorum / quota) said no.
    PolicyViolation = 1,
    /// `Inv 3` escrow-lock missing for a write-side mutation.
    EscrowMissing = 2,
    /// `monetary_invariant` (Inv 4 / 基本法 1) flagged a conservation break.
    InvariantViolation = 3,
    /// `canonical_decode` of the submitted bytes failed.
    MalformedPayload = 4,
}

// ────────────────────────────────────────────────────────────────────────────
// RejectedSubmissionRecord — one row on the L4.E chain
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:6/P1:9 — one rejection-evidence row.
///
/// Distinguished from `LedgerEntry` (the L4 accepted spine):
/// - keyed by `submit_id` (not `logical_t`);
/// - records `parent_state_root` for the snapshot-at-submit but never a
///   `resulting_state_root` (rejection MUST NOT advance state);
/// - `raw_diagnostic_cid` holds the raw error content shielded behind a CAS
///   handle (not exposed in agent-facing views);
/// - `public_summary` is the ONLY field permitted to cross the agent boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectedSubmissionRecord {
    /// Independent submit-side counter from `Sequencer::next_submit_id`.
    pub submit_id: u64,
    /// State-root snapshot at submit time — recorded for forensics; NEVER
    /// advanced by rejection (Inv 7).
    pub parent_state_root: Hash,
    /// Submitter agent (opaque string).
    pub agent_id: AgentId,
    /// Discriminator over the submitted (now-rejected) `TypedTx` variant.
    pub tx_kind: TxKind,
    /// CAS handle to the canonical-encoded source `TypedTx`.
    pub tx_payload_cid: Cid,
    /// Coarse why-class (one of `RejectionClass`).
    pub rejection_class: RejectionClass,
    /// CAS handle to the raw diagnostic bytes (e.g. predicate counter-example).
    /// `None` when no raw payload is captured. NEVER exposed via `PublicRejectionView`.
    pub raw_diagnostic_cid: Option<Cid>,
    /// Agent-facing summary string. `None` when no public summary is permitted
    /// (raw-diagnostic-only mode). The ONLY field that crosses the agent boundary.
    pub public_summary: Option<String>,
    /// Hash of the immediately-preceding rejection record; `Hash::ZERO` for the first.
    pub prev_hash: Hash,
    /// SHA-256 over the nine fields above plus a domain-separation prefix.
    pub hash: Hash,
}

impl RejectedSubmissionRecord {
    fn compute_hash(
        submit_id: u64,
        parent_state_root: &Hash,
        agent_id: &AgentId,
        tx_kind: TxKind,
        tx_payload_cid: &Cid,
        rejection_class: RejectionClass,
        raw_diagnostic_cid: &Option<Cid>,
        public_summary: &Option<String>,
        prev_hash: &Hash,
    ) -> Hash {
        let mut h = Sha256::new();
        h.update(b"turingosv4.l4e_rejection_evidence.v1");
        h.update(submit_id.to_be_bytes());
        h.update(parent_state_root.0);
        h.update((agent_id.0.len() as u64).to_be_bytes());
        h.update(agent_id.0.as_bytes());
        h.update((tx_kind as u8).to_be_bytes());
        h.update(tx_payload_cid.0);
        h.update((rejection_class as u8).to_be_bytes());
        match raw_diagnostic_cid {
            Some(c) => {
                h.update([1u8]);
                h.update(c.0);
            }
            None => h.update([0u8]),
        }
        match public_summary {
            Some(s) => {
                h.update([1u8]);
                h.update((s.len() as u64).to_be_bytes());
                h.update(s.as_bytes());
            }
            None => h.update([0u8]),
        }
        h.update(prev_hash.0);
        Hash(h.finalize().into())
    }
}

// ────────────────────────────────────────────────────────────────────────────
// PublicRejectionView — agent-facing projection (Inv 10 Goodhart shield)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX Inv 10 + ROADMAP P1:9 — agent-facing projection.
///
/// **Structural** isolation: the type itself does not carry
/// `raw_diagnostic_cid`. Materializing this view from a
/// `RejectedSubmissionRecord` cannot accidentally leak the raw diagnostic
/// because there is no field to write it into.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicRejectionView {
    pub submit_id: u64,
    pub parent_state_root: Hash,
    pub agent_id: AgentId,
    pub tx_kind: TxKind,
    pub rejection_class: RejectionClass,
    pub public_summary: Option<String>,
}

impl From<&RejectedSubmissionRecord> for PublicRejectionView {
    fn from(r: &RejectedSubmissionRecord) -> Self {
        Self {
            submit_id: r.submit_id,
            parent_state_root: r.parent_state_root,
            agent_id: r.agent_id.clone(),
            tx_kind: r.tx_kind,
            rejection_class: r.rejection_class,
            public_summary: r.public_summary.clone(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// RejectionEvidenceError — chain-walk failure taxonomy
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:6 — error returned by `RejectionEvidenceWriter::verify_chain`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectionEvidenceError {
    /// `prev_hash` chain or per-record hash diverged at the given index
    /// (covers row deletion, field tampering, and reordering).
    HashMismatch { at: usize },
}

impl std::fmt::Display for RejectionEvidenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HashMismatch { at } => write!(f, "rejection-evidence chain break at index {}", at),
        }
    }
}

impl std::error::Error for RejectionEvidenceError {}

// ────────────────────────────────────────────────────────────────────────────
// RejectionEvidenceWriter — append + verify + project-to-public
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:6/P1:9 — RSP-0 in-memory rejection-evidence writer.
///
/// One `Vec<RejectedSubmissionRecord>`; `prev_hash` chained; `submit_id`
/// monotonicity is the caller's responsibility (the writer trusts the
/// `Sequencer::next_submit_id` issuer). No `logical_t` field — accepted
/// spine and rejection-evidence are intentionally disjoint per the L4 / L4.E
/// split (`DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`).
#[derive(Debug, Clone, Default)]
pub struct RejectionEvidenceWriter {
    records: Vec<RejectedSubmissionRecord>,
}

impl RejectionEvidenceWriter {
    /// TRACE_MATRIX P1:6 — empty writer.
    pub fn new() -> Self {
        Self::default()
    }

    /// TRACE_MATRIX P1:6 — count of recorded rejections.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// TRACE_MATRIX P1:6 — empty predicate.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// TRACE_MATRIX P1:6 — last record's hash, or `Hash::ZERO` for empty chain.
    pub fn last_hash(&self) -> Hash {
        self.records.last().map(|r| r.hash).unwrap_or(Hash::ZERO)
    }

    /// TRACE_MATRIX P1:6/P1:9 — append a rejection record; returns the new chain hash.
    ///
    /// CRITICAL: this method MUST NOT be called from the L4 (accepted) write
    /// path — Inv 7 forbids state-root advance on rejection. The caller's
    /// dispatch logic decides which ledger receives the record.
    #[allow(clippy::too_many_arguments)]
    pub fn append_rejected(
        &mut self,
        submit_id: u64,
        parent_state_root: Hash,
        agent_id: AgentId,
        tx_kind: TxKind,
        tx_payload_cid: Cid,
        rejection_class: RejectionClass,
        raw_diagnostic_cid: Option<Cid>,
        public_summary: Option<String>,
    ) -> Hash {
        let prev_hash = self.last_hash();
        let hash = RejectedSubmissionRecord::compute_hash(
            submit_id,
            &parent_state_root,
            &agent_id,
            tx_kind,
            &tx_payload_cid,
            rejection_class,
            &raw_diagnostic_cid,
            &public_summary,
            &prev_hash,
        );
        let record = RejectedSubmissionRecord {
            submit_id,
            parent_state_root,
            agent_id,
            tx_kind,
            tx_payload_cid,
            rejection_class,
            raw_diagnostic_cid,
            public_summary,
            prev_hash,
            hash,
        };
        self.records.push(record);
        hash
    }

    /// TRACE_MATRIX P1:6 — verify the rejection-evidence chain end-to-end.
    ///
    /// Returns `Err(HashMismatch)` if any single field of any record was
    /// tampered, or if a row was deleted (the surviving row's `prev_hash`
    /// no longer matches its predecessor's `hash`).
    pub fn verify_chain(&self) -> Result<(), RejectionEvidenceError> {
        let mut prev = Hash::ZERO;
        for (i, r) in self.records.iter().enumerate() {
            if r.prev_hash != prev {
                return Err(RejectionEvidenceError::HashMismatch { at: i });
            }
            let recomputed = RejectedSubmissionRecord::compute_hash(
                r.submit_id,
                &r.parent_state_root,
                &r.agent_id,
                r.tx_kind,
                &r.tx_payload_cid,
                r.rejection_class,
                &r.raw_diagnostic_cid,
                &r.public_summary,
                &r.prev_hash,
            );
            if recomputed != r.hash {
                return Err(RejectionEvidenceError::HashMismatch { at: i });
            }
            prev = r.hash;
        }
        Ok(())
    }

    /// TRACE_MATRIX P1:9 — read-only record slice (for L4.E forensics; full
    /// records carry `raw_diagnostic_cid` and MUST NOT be exposed across the
    /// agent boundary; use `public_view` for that).
    pub fn records(&self) -> &[RejectedSubmissionRecord] {
        &self.records
    }

    /// TRACE_MATRIX Inv 10 + P1:9 — agent-facing projection.
    ///
    /// `PublicRejectionView` does not carry `raw_diagnostic_cid` by type
    /// construction; this method's output is safe to materialize into another
    /// agent's read view.
    pub fn public_view(&self) -> Vec<PublicRejectionView> {
        self.records.iter().map(PublicRejectionView::from).collect()
    }

    /// TRACE_MATRIX P1:6 — TAMPER-ONLY hook used by kill-criteria integration
    /// tests (`test_p1_kill_4b_rejection_chain_breaks_on_row_deletion`).
    /// `#[doc(hidden)]` + `tamper_` prefix flags any production use as a
    /// reviewable violation; kept `pub` only so integration tests in `tests/`
    /// can reach it (they link against the lib without `cfg(test)` enabled).
    #[doc(hidden)]
    pub fn tamper_remove_record(&mut self, idx: usize) {
        self.records.remove(idx);
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Inline correctness tests; cross-cutting P1 kill acceptance tests live in
// `tests/tb_1_p1_acceptance.rs`.
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn cid(byte: u8) -> Cid {
        Cid([byte; 32])
    }
    fn agent(s: &str) -> AgentId {
        AgentId(s.to_string())
    }

    #[test]
    fn append_records_and_chains() {
        let mut w = RejectionEvidenceWriter::new();
        let h1 = w.append_rejected(
            1,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            Some(cid(0xAA)),
            Some("predicate acc1 returned false".into()),
        );
        let h2 = w.append_rejected(
            2,
            Hash::ZERO,
            agent("bob"),
            TxKind::Verify,
            cid(0x11),
            RejectionClass::PolicyViolation,
            None,
            None,
        );
        assert_eq!(w.len(), 2);
        assert_ne!(h1, Hash::ZERO);
        assert_ne!(h2, Hash::ZERO);
        assert_eq!(w.records()[1].prev_hash, h1);
        assert_eq!(w.last_hash(), h2);
        assert!(w.verify_chain().is_ok());
    }

    #[test]
    fn public_view_omits_raw_diagnostic_cid() {
        let mut w = RejectionEvidenceWriter::new();
        w.append_rejected(
            1,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            Some(cid(0xAA)), // raw diagnostic bytes
            Some("acc1 false".into()),
        );
        let view = w.public_view();
        assert_eq!(view.len(), 1);
        // Structural isolation: `PublicRejectionView` doesn't have a
        // `raw_diagnostic_cid` field. Round-trip via JSON to assert the
        // serialized form also omits it.
        let json = serde_json::to_value(&view[0]).unwrap();
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("raw_diagnostic_cid"));
        assert_eq!(obj.get("public_summary").unwrap(), "acc1 false");
    }

    #[test]
    fn verify_detects_field_tamper() {
        let mut w = RejectionEvidenceWriter::new();
        w.append_rejected(
            1,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            None,
            Some("ok".into()),
        );
        w.append_rejected(
            2,
            Hash::ZERO,
            agent("bob"),
            TxKind::Verify,
            cid(0x11),
            RejectionClass::PolicyViolation,
            None,
            None,
        );
        // Tamper public_summary on record 0; per-record hash should now
        // disagree with its computed value.
        w.records[0].public_summary = Some("tampered".into());
        let r = w.verify_chain();
        assert!(matches!(r, Err(RejectionEvidenceError::HashMismatch { at: 0 })));
    }
}

```


---

## XREF: h_vppu_history.rs (Day-4) — `experiments/minif2f_v4/src/h_vppu_history.rs`

```
// TB-1 Day-4 (2026-04-29): per-problem rolling history of pput_verified,
// used to compute the H-VPPUT North Star metric per PREREG § 5.
//
// Authoritative spec: handover/tracer_bullets/TB-1_recharter_2026-04-29.md
// (Day 4 — P6 instrumentation: h_vppu computation).
//
// Semantics (per recharter § 2 Day 4):
//   h_vppu = current_pput_verified / mean(history N=1..3)
//
// Returns None when:
//   - no prior runs exist for this problem (first run; no signal),
//   - all prior pput_verified values sum to 0 (mean=0; ratio undefined).
//
// History is capped at 3 prior runs per problem (rolling window; oldest
// drops on push). This matches the recharter spec and keeps the per-
// problem signal fresh without unbounded growth.
//
// Persistence: caller passes a path to load_from / save_to. The store
// is JSON-encoded (one HashMap<String, VecDeque<f64>>); a missing or
// unreadable file degrades to an empty store so the first ever run
// against any environment never panics.
//
// FC-trace: FC1-N11 (∏p decision diversity) — h_vppu measures
// per-problem regression vs prior runs; runs that re-attempt with no
// learning produce h_vppu = current/mean ≈ 1.0 or below. Step-4
// Capability Compilation should drive h_vppu > 1 on heldout.

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

const HISTORY_CAPACITY: usize = 3;

/// TRACE_MATRIX orphan (P6 instrumentation; PREREG_PPUT_CCL_2026-04-26.md § 5
/// H-VPPUT North Star): per-problem rolling history of `pput_verified`
/// values used to compute the held-out verified PPUT regression ratio
/// emitted on `PputResult.h_vppu`. Not a constitutional flowchart node;
/// justified as an Epistemic Lab v0 product-line metric per the 9-phase
/// roadmap (`handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HVppuHistory {
    /// problem_id → rolling history of pput_verified values from prior runs.
    /// Newest pushed at back; oldest popped at front when len > capacity.
    by_problem: HashMap<String, VecDeque<f64>>,
}

impl HVppuHistory {
    /// TRACE_MATRIX orphan (P6 instrumentation): empty constructor;
    /// callers prefer load_from for persisted history.
    pub fn new() -> Self {
        Self::default()
    }

    /// TRACE_MATRIX orphan (P6 instrumentation; PREREG § 5):
    /// load from disk. Returns Self::default() on missing or unreadable
    /// file (graceful degradation; H-VPPUT is a non-blocking P6 metric per
    /// recharter Day 5 Tier-B). A corrupt store logs to stderr and starts
    /// fresh rather than panicking — instrumentation must not block runs.
    pub fn load_from(path: &Path) -> Self {
        match fs::read_to_string(path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|e| {
                eprintln!(
                    "[h_vppu_history] corrupt store at {:?} ({}); starting fresh",
                    path, e
                );
                Self::default()
            }),
            Err(_) => Self::default(),
        }
    }

    /// TRACE_MATRIX orphan (P6 instrumentation; PREREG § 5):
    /// save to disk (atomic-ish: write to tmp then rename). Returns io::Error
    /// on failure; caller decides fail-loud vs log-and-continue. P6
    /// instrumentation should not block the ship path on a missing disk.
    pub fn save_to(&self, path: &Path) -> io::Result<()> {
        let serialized = serde_json::to_string_pretty(self).map_err(io::Error::other)?;
        let tmp_path = path.with_extension("json.tmp");
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::write(&tmp_path, serialized)?;
        fs::rename(&tmp_path, path)
    }

    /// TRACE_MATRIX orphan (P6 instrumentation; PREREG § 5):
    /// append the current run's pput_verified to the per-problem history.
    /// Trims to HISTORY_CAPACITY (3) — newest at back, oldest dropped at
    /// front. Idempotent only with respect to identical values; callers
    /// that want at-most-once semantics across retries must dedupe.
    pub fn record(&mut self, problem_id: &str, pput_verified: f64) {
        let entry = self
            .by_problem
            .entry(problem_id.to_string())
            .or_default();
        entry.push_back(pput_verified);
        while entry.len() > HISTORY_CAPACITY {
            entry.pop_front();
        }
    }

    /// TRACE_MATRIX orphan (P6 instrumentation; PREREG_PPUT_CCL_2026-04-26.md § 5
    /// H-VPPUT definition): compute h_vppu = current / mean(history) when
    /// there is at least one prior run AND that mean is non-zero. The
    /// current run's value is NOT included in the mean — h_vppu measures
    /// improvement against a held-out baseline, not against itself.
    ///
    /// Returns None when:
    ///   - no history exists for this problem (first run);
    ///   - the prior history mean is 0 (all prior runs failed; ratio
    ///     undefined — None preserves "no signal" semantics rather than
    ///     emitting NaN/inf into the JSONL row).
    pub fn h_vppu_for(&self, problem_id: &str, current_pput_verified: f64) -> Option<f64> {
        let entry = self.by_problem.get(problem_id)?;
        if entry.is_empty() {
            return None;
        }
        let n = entry.len() as f64;
        let sum: f64 = entry.iter().sum();
        let mean = sum / n;
        if mean == 0.0 {
            return None;
        }
        Some(current_pput_verified / mean)
    }

    /// TRACE_MATRIX orphan (P6 instrumentation; PREREG § 5):
    /// number of prior runs stored for a given problem. Exposed for
    /// tests + downstream auditors that want to assert capacity-3
    /// invariants without round-tripping through JSON.
    pub fn history_len(&self, problem_id: &str) -> usize {
        self.by_problem
            .get(problem_id)
            .map(|v| v.len())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Generate a unique tmp path under std::env::temp_dir() without
    /// pulling in the `tempfile` crate. Each test gets its own.
    fn unique_tmp_path(label: &str) -> std::path::PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "h_vppu_history_{}_{}_{}.json",
            label, nanos, seq
        ))
    }

    #[test]
    fn test_first_run_returns_none() {
        let history = HVppuHistory::new();
        assert_eq!(
            history.h_vppu_for("mathd_algebra_107", 0.5),
            None,
            "no prior runs → None (no signal)"
        );
    }

    #[test]
    fn test_second_run_returns_ratio() {
        let mut history = HVppuHistory::new();
        history.record("mathd_algebra_107", 0.4);
        // Current run pput_verified = 0.6 → h_vppu = 0.6/0.4 = 1.5
        let h = history
            .h_vppu_for("mathd_algebra_107", 0.6)
            .expect("one prior run gives a ratio");
        assert!(
            (h - 1.5).abs() < 1e-12,
            "h_vppu = current/mean = 0.6/0.4 = 1.5, got {}",
            h
        );
    }

    #[test]
    fn test_capacity_3_rolling_window() {
        let mut history = HVppuHistory::new();
        for v in [0.1, 0.2, 0.3, 0.4, 0.5] {
            history.record("p1", v);
        }
        assert_eq!(
            history.history_len("p1"),
            HISTORY_CAPACITY,
            "rolling window keeps only last 3"
        );
        // Only the last 3 (0.3, 0.4, 0.5) survive; mean = 0.4
        let h = history.h_vppu_for("p1", 0.4).unwrap();
        assert!(
            (h - 1.0).abs() < 1e-12,
            "0.4 / mean(0.3,0.4,0.5) = 0.4/0.4 = 1.0, got {}",
            h
        );
    }

    #[test]
    fn test_zero_mean_returns_none() {
        let mut history = HVppuHistory::new();
        history.record("p1", 0.0);
        history.record("p1", 0.0);
        // mean=0 → ratio undefined → None (anti-Goodhart: never emit NaN/inf)
        assert_eq!(history.h_vppu_for("p1", 0.5), None);
    }

    #[test]
    fn test_per_problem_isolation() {
        let mut history = HVppuHistory::new();
        history.record("p1", 0.2);
        history.record("p2", 0.8);
        // p1's history must NOT pollute p2's ratio.
        let h_p1 = history.h_vppu_for("p1", 0.4).unwrap(); // 0.4 / 0.2 = 2.0
        let h_p2 = history.h_vppu_for("p2", 0.8).unwrap(); // 0.8 / 0.8 = 1.0
        assert!((h_p1 - 2.0).abs() < 1e-12);
        assert!((h_p2 - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_persistence_roundtrip() {
        let path = unique_tmp_path("roundtrip");
        let mut h1 = HVppuHistory::new();
        h1.record("p1", 0.4);
        h1.record("p2", 0.7);
        h1.save_to(&path).expect("save");

        let h2 = HVppuHistory::load_from(&path);
        assert!((h2.h_vppu_for("p1", 0.6).unwrap() - 1.5).abs() < 1e-12);
        assert!((h2.h_vppu_for("p2", 1.4).unwrap() - 2.0).abs() < 1e-12);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_load_missing_file_default() {
        let path = unique_tmp_path("missing");
        // Path does not exist; load must degrade to empty.
        let h = HVppuHistory::load_from(&path);
        assert_eq!(h.h_vppu_for("any", 1.0), None);
    }

    #[test]
    fn test_corrupt_file_degrades_to_default() {
        let path = unique_tmp_path("corrupt");
        fs::write(&path, "{not valid json").unwrap();
        let h = HVppuHistory::load_from(&path);
        assert_eq!(
            h.h_vppu_for("any", 1.0),
            None,
            "corrupt store must not panic; returns empty"
        );
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_record_before_query_does_not_self_reference() {
        // Verify h_vppu_for does NOT include the in-flight value when
        // the caller passes (current, then record). This ordering is
        // load → query → record → save (per evaluator wire site).
        let mut history = HVppuHistory::new();
        history.record("p1", 0.4);
        let h_before_record = history.h_vppu_for("p1", 0.6).unwrap();
        history.record("p1", 0.6);
        let h_after_record = history.h_vppu_for("p1", 0.6).unwrap();
        // Before record: 0.6 / 0.4 = 1.5
        // After record: 0.6 / mean(0.4, 0.6) = 0.6 / 0.5 = 1.2
        assert!((h_before_record - 1.5).abs() < 1e-12);
        assert!((h_after_record - 1.2).abs() < 1e-12);
    }
}

```


---

## XREF: Day-4 evaluator.rs main() wire-up (lines ~322-385)

```rust
          problem_file, condition, model, resolved_mode.label());

    let mut result = match condition.as_str() {
        "oneshot" => {
            run_oneshot(problem_file, &problem_statement, &theorem_name,
                       &lean_path, &proxy_url, &model).await
        }
        // Generic nN: parse any "n<digits>" → run_swarm with N agents.
        // Supports N-scaling experiment (percolation curve mapping).
        // **swarm_N=1** (CONDITION=n1) is the critical baseline for the
        // 2026-04-25 N-experiments arc: same code path as n3/n8 swarm
        // but with a single agent. NOT the same as `oneshot` (which
        // skips the swarm loop, tape, mr ticks, ∏p product, etc.).
        // Per Plan-agent NEXT-1 / Codex E0 / Gemini E1-Prime: every
        // N-curve experiment MUST use n1 (not oneshot) as the N=1
        // baseline to avoid code-path confound. Validated by unit
        // test below: parse_swarm_condition_n("n1") == Some(1).
        c if parse_swarm_condition_n(c).is_some() => {
            let n = parse_swarm_condition_n(c).unwrap();
            run_swarm(problem_file, &problem_statement, &theorem_name,
                     &lean_path, &proxy_url, &model, n).await
        }
        "hybrid_v1" => {
            // Mid-term audit P0-D fix 2026-04-25: hybrid_v1 was a Paper 1 era
            // condition that ran run_oneshot, then on failure ran run_swarm,
            // and merged via `..r2` field-spread. Codex flagged that the spread
            // dropped the failed oneshot's C_i (failed_branch_count and
            // total_run_token_count from r1 were silently discarded). PPUT-CCL
            // arc does NOT use hybrid_v1 — it operates exclusively on `oneshot`
            // and `n<N>` conditions per PREREG. Disabling here forces any
            // pipeline that ships a stale hybrid_v1 invocation to surface the
            // deprecation immediately rather than emit a corrupt C_i.
            eprintln!("hybrid_v1 condition is deprecated for PPUT-CCL arc and was \
                       disabled in mid-term audit P0-D fix 2026-04-25. The prior \
                       implementation dropped the failed oneshot leg's C_i via a \
                       `..r2` field-spread, corrupting full-run cost accounting. \
                       Use `oneshot` or `n<N>` instead.");
            std::process::exit(1);
        }
        other => { eprintln!("Unknown condition: {}", other); std::process::exit(1); }
    };

    // TB-1 Day-4 (2026-04-29): stamp h_vppu by querying the persisted
    // per-problem rolling history of pput_verified, then record the
    // current run's pput_verified for future invocations. Order is
    // load → query (excluding current) → stamp → record → save, so
    // the current run does NOT self-reference its own value when
    // computing the ratio.
    //
    // Storage: $EXPERIMENT_DIR/h_vppu_history.json (or cwd if unset).
    // Failure to load/save degrades quietly — h_vppu is a P6 non-
    // blocking metric per recharter Day-5 Tier-B. Saving failure logs
    // a warning but never aborts the run.
    let h_vppu_path = std::path::PathBuf::from(
        std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into()),
    )
    .join("h_vppu_history.json");
    let mut h_vppu_history =
        minif2f_v4::h_vppu_history::HVppuHistory::load_from(&h_vppu_path);
    result.h_vppu = h_vppu_history.h_vppu_for(&result.problem_id, result.pput_verified);
    h_vppu_history.record(&result.problem_id, result.pput_verified);
    if let Err(e) = h_vppu_history.save_to(&h_vppu_path) {
        log::warn!(
            "[h_vppu_history] save to {:?} failed: {}; next run will start without prior history",
            h_vppu_path, e
        );
    }

    // Output PPUT result as JSON (machine-readable for batch runner)
    let json = serde_json::to_string(&result).unwrap();
    println!("PPUT_RESULT:{}", json);

    if result.has_golden_path {
        info!("PPUT = {:.2}%/s (GP: {} nodes, {} tokens, {:.1}s)",
              result.pput, result.gp_node_count, result.gp_token_count, result.time_secs);
    } else {

```


---

## XREF: Day-4 live wire-up evidence (TB-1 acceptance signal)

### RUN 1 JSONL (cold; no h_vppu — first run, omitted via Option::is_none):
```json
PPUT_RESULT:{"schema_version":"v2.0","run_id":"n3_mathd_algebra_107_1777485949022","problem_id":"mathd_algebra_107","solved":true,"split":"adaptation","verified":true,"golden_path_token_count":12,"total_run_token_count":448,"total_wall_time_ms":56950,"progress":1,"pput_runtime":0.00003919478239056817,"pput_verified":0.00003919478239056817,"pput_m_verified":39.19478239056817,"failed_branch_count":0,"rollback_count":0,"hit_max_tx":false,"tactic_diversity":1.0,"verifier_wait_ms":56121,"budget_regime":"total_proposal","budget_max_transactions":10,"far":0.0,"err":0.0,"iac":0.0,"cpr":0.0,"model_snapshot":"deepseek-chat","git_sha":"","binary_sha256":"","mode":"full","problem":"/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_107.lean","condition":"n3","model":"deepseek-chat","has_golden_path":true,"time_secs":56.950916981,"pput":1.7558979784884248,"gp_token_count":12,"gp_node_count":1,"tx_count":1,"classifier_version":"v1_2026-04-16-a","tool_dist":{"step":1,"omega_wtool":1},"gp_payload":"nlinarith","gp_path":"per_tactic","gp_proof_file":"proofs/mathd_algebra_107_1777486005_73ee91ba.lean"}

```

### RUN 2 JSONL (warm; h_vppu=6.2159...):
```json
PPUT_RESULT:{"schema_version":"v2.0","run_id":"n3_mathd_algebra_107_1777486023970","problem_id":"mathd_algebra_107","solved":true,"split":"adaptation","verified":true,"golden_path_token_count":12,"total_run_token_count":448,"total_wall_time_ms":9162,"progress":1,"pput_runtime":0.00024363052359123086,"pput_verified":0.00024363052359123086,"pput_m_verified":243.63052359123085,"failed_branch_count":0,"rollback_count":0,"hit_max_tx":false,"tactic_diversity":1.0,"verifier_wait_ms":8175,"budget_regime":"total_proposal","budget_max_transactions":10,"far":0.0,"err":0.0,"iac":0.0,"cpr":0.0,"model_snapshot":"deepseek-chat","git_sha":"","binary_sha256":"","mode":"full","problem":"/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_107.lean","condition":"n3","model":"deepseek-chat","has_golden_path":true,"time_secs":9.162645877,"pput":10.913878080895739,"gp_token_count":12,"gp_node_count":1,"tx_count":1,"classifier_version":"v1_2026-04-16-a","tool_dist":{"step":1,"omega_wtool":1},"gp_payload":"nlinarith","gp_path":"per_tactic","gp_proof_file":"proofs/mathd_algebra_107_1777486033_73ee91ba.lean","h_vppu":6.215891726697228}

```

### h_vppu_history.json after run 2:
```json
{
  "by_problem": {
    "mathd_algebra_107": [
      0.00003919478239056817,
      0.00024363052359123086
    ]
  }
}
```


---

## XREF: Recent commit log (TB-1 Days 1-5)

```
6c04c26 TB-1 Day-5: Tier-A 9 acceptance battery (consolidated; 9/9 PASS)
50a1d67 TB-1 Day-4: P6 h_vppu_history instrumentation (NEW file)
846279f TB-1 Day-3: P1 GitTape ledger.rs + rejection_evidence.rs (NEW files)
451cc66 TB-1 Day-2: P3 RSP-0 monetary_invariant + escrow_vault (NEW files)
947e67a TB_LOG.tsv: TB-1 row Tier-A/B annotation; primary-secondary-tertiary phase tag (audit-ingestion wave A #6)
c82db19 DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29: L4 vs L4.E architectural commitment (audit-ingestion wave A #5)
edef868 TB-1 re-charter amendment: L4/L4.E split + WalletTool projection + Tier-A/B (audit-ingestion wave A #4)
538b697 ROADMAP amendment: P0.R + L4/L4.E split + P3 forbidden + dependency graph (audit-ingestion wave A #3)
6b100eb docs/economics.md: rewrite as RSP-0/RSP-1 ground rules (audit-ingestion wave A #2)
0709819 External audit 2026-04-29: archive verbatim (audit-ingestion wave A #1)
d29218e TB-1 re-charter (Days 2-7): P1 + P3 RSP-0 + P6 instrumentation
b433dfd TB_LOG.tsv schema v2: add phase_id + Exit/kill columns; backfill TB-0/TB-1
b983c4a TB methodology v2: P0-P9 phase tagging + kill-criteria gate
1435769 Architect directive 2026-04-29: archive 9-phase roadmap + canonical P0-P9 doc
063b003 TB-1 Day-1 spike: prompt_context_hash + h_vppu fields land; step-4 plumbing exists end-to-end on mathd_algebra_107
```

codex
I’ll audit the actual tree rather than the pasted excerpts first: local line numbers, grep evidence for call sites/serialization, and the Day-3 to Day-5 test consolidation diff.
exec
/bin/bash -lc 'git status --short && git log --oneline -n 18' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
?? handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md
?? handover/audits/run_codex_tb_1_audit_2026-04-29.sh
?? handover/audits/run_gemini_tb_1_audit_2026-04-29.py
6c04c26 TB-1 Day-5: Tier-A 9 acceptance battery (consolidated; 9/9 PASS)
50a1d67 TB-1 Day-4: P6 h_vppu_history instrumentation (NEW file)
846279f TB-1 Day-3: P1 GitTape ledger.rs + rejection_evidence.rs (NEW files)
451cc66 TB-1 Day-2: P3 RSP-0 monetary_invariant + escrow_vault (NEW files)
947e67a TB_LOG.tsv: TB-1 row Tier-A/B annotation; primary-secondary-tertiary phase tag (audit-ingestion wave A #6)
c82db19 DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29: L4 vs L4.E architectural commitment (audit-ingestion wave A #5)
edef868 TB-1 re-charter amendment: L4/L4.E split + WalletTool projection + Tier-A/B (audit-ingestion wave A #4)
538b697 ROADMAP amendment: P0.R + L4/L4.E split + P3 forbidden + dependency graph (audit-ingestion wave A #3)
6b100eb docs/economics.md: rewrite as RSP-0/RSP-1 ground rules (audit-ingestion wave A #2)
0709819 External audit 2026-04-29: archive verbatim (audit-ingestion wave A #1)
d29218e TB-1 re-charter (Days 2-7): P1 + P3 RSP-0 + P6 instrumentation
b433dfd TB_LOG.tsv schema v2: add phase_id + Exit/kill columns; backfill TB-0/TB-1
b983c4a TB methodology v2: P0-P9 phase tagging + kill-criteria gate
1435769 Architect directive 2026-04-29: archive 9-phase roadmap + canonical P0-P9 doc
063b003 TB-1 Day-1 spike: prompt_context_hash + h_vppu fields land; step-4 plumbing exists end-to-end on mathd_algebra_107
4ecb708 TB methodology install (Karpathy-minimal): TB_LOG.tsv + alignment_coverage.py + NOTEPAD section + TB-1 charter
f0b659f First v4-native MiniF2F solve: mathd_algebra_107 SOLVED (10s, nlinarith) at HEAD a906886
a906886 Session-3 capability-first pivot: CO1.8 v1 DEFERRED + iteration-cap policy

exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g '"'!target'"' -g '"'!node_modules'"'" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
/home/zephryj/projects/turingosv4

codex
The worktree already has untracked audit helper files; I’m not going to touch them. I didn’t find an `AGENTS.md` under this repo with the first scan, so the prompt-level AGENTS guidance is the active project instruction for this audit.
exec
/bin/bash -lc 'find .. -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'rg -n "RejectedSubmissionRecord|PublicRejectionView|raw_diagnostic_cid|public_view|append_rejected|RejectionEvidenceWriter" -S src tests experiments handover | head -n 200' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'rg -n "assert_read_is_free|assert_no_post_init_mint|assert_total_ctf_conserved|MonetaryError|EscrowVault|lock_escrow|release_escrow|dispatch_transition|apply_one|SettlementEngine" -S src tests experiments | head -n 240' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/economy/escrow_vault.rs:16://! - `lock_escrow` records sponsor-locked supply for one task.
src/economy/escrow_vault.rs:17://! - `release_escrow` distributes payouts, asserts `Σ payouts ≤ locked`,
src/economy/escrow_vault.rs:19://! - No I/O, no L4 emission. Wiring into `dispatch_transition` /
src/economy/escrow_vault.rs:20://!   `SettlementEngine` lands in TB-1 Day-3 / TB-2 (RSP-1).
src/economy/escrow_vault.rs:23://! - Per-claim sub-vault accounting (RSP-2 SettlementEngine).
src/economy/escrow_vault.rs:53:/// view used by the sequencer / SettlementEngine before flushing to L4.
src/economy/escrow_vault.rs:66:/// TRACE_MATRIX P3:5 — receipt returned by [`EscrowVault::lock_escrow`].
src/economy/escrow_vault.rs:74:/// TRACE_MATRIX P3:6/P3:8 — outcome returned by [`EscrowVault::release_escrow`].
src/economy/escrow_vault.rs:91:/// TRACE_MATRIX P3 RSP-0 — errors returned by `EscrowVault` operations.
src/economy/escrow_vault.rs:94:    /// `lock_escrow` called twice for the same `task_id` without an
src/economy/escrow_vault.rs:97:    /// `release_escrow` called for a `task_id` with no prior lock.
src/economy/escrow_vault.rs:99:    /// `release_escrow` called for a `task_id` already released.
src/economy/escrow_vault.rs:138:// EscrowVault — task-keyed in-memory vault (RSP-0)
src/economy/escrow_vault.rs:146:pub struct EscrowVault {
src/economy/escrow_vault.rs:150:impl EscrowVault {
src/economy/escrow_vault.rs:177:    pub fn lock_escrow(
src/economy/escrow_vault.rs:209:    pub fn release_escrow(
src/economy/escrow_vault.rs:283:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:285:            .lock_escrow(task("t1"), agent("alice"), coin(100))
src/economy/escrow_vault.rs:299:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:300:        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
src/economy/escrow_vault.rs:301:        let r = v.lock_escrow(task("t1"), agent("bob"), coin(50));
src/economy/escrow_vault.rs:307:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:308:        let r = v.lock_escrow(
src/economy/escrow_vault.rs:321:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:322:        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
src/economy/escrow_vault.rs:326:        let r = v.release_escrow(&task("t1"), &payouts);
src/economy/escrow_vault.rs:337:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:338:        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
src/economy/escrow_vault.rs:342:        let outcome = v.release_escrow(&task("t1"), &payouts).unwrap();
src/economy/escrow_vault.rs:354:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:355:        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
src/economy/escrow_vault.rs:359:        let outcome = v.release_escrow(&task("t1"), &payouts).unwrap();
src/economy/escrow_vault.rs:367:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:368:        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
src/economy/escrow_vault.rs:370:        let outcome = v.release_escrow(&task("t1"), &payouts).unwrap();
src/economy/escrow_vault.rs:377:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:378:        let r = v.release_escrow(&task("t1"), &BTreeMap::new());
src/economy/escrow_vault.rs:384:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:385:        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
src/economy/escrow_vault.rs:386:        v.release_escrow(&task("t1"), &BTreeMap::new()).unwrap();
src/economy/escrow_vault.rs:387:        let r = v.release_escrow(&task("t1"), &BTreeMap::new());
src/economy/escrow_vault.rs:393:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:394:        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
src/economy/escrow_vault.rs:397:        let r = v.release_escrow(&task("t1"), &payouts);
src/economy/escrow_vault.rs:407:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:408:        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
src/economy/escrow_vault.rs:409:        v.lock_escrow(task("t2"), agent("bob"), coin(50)).unwrap();
src/economy/escrow_vault.rs:413:        v.release_escrow(&task("t1"), &p1).unwrap();
src/economy/ledger.rs:30://! - `dispatch_transition` re-run (state_root mutation requires CO1.8).
src/economy/ledger.rs:69:    /// (the RSP-0 toy mutator); a real `dispatch_transition` lands in CO1.7.5.
src/economy/ledger.rs:344:/// invariant. The real `dispatch_transition`-driven state_root mutation lands
src/state/q_state.rs:159:/// TRACE_MATRIX WP § 2 — tx → escrow entry. Full schema lands CO P2.2 EscrowVault.
src/state/q_state.rs:199:/// TRACE_MATRIX WP § 2 — tx → reward claim. Full schema lands CO P2.6 SettlementEngine.
src/state/mod.rs:15:/// TRACE_MATRIX § 5.2.1 / CO1.7-impl A2+A3 — L4 sequencer + dispatch_transition.
src/state/sequencer.rs:1://! L4 Sequencer + dispatch_transition (CO1.7-impl A2 + A3).
src/state/sequencer.rs:7://!   (dispatch_transition exhaustive enum match; K5 Slash dropped)
src/state/sequencer.rs:37:// § 8 dispatch_transition — exhaustive enum match (K5: NO Slash)
src/state/sequencer.rs:47:pub(crate) fn dispatch_transition(
src/state/sequencer.rs:75:/// Called from `apply_one` stage 9 AFTER `writer.commit` succeeds. Pure
src/state/sequencer.rs:83:/// **Atomicity** (CO1.7-extra round-2 MF9): in apply_one, called under the
src/state/sequencer.rs:127:/// Errors that can occur during `apply_one`. Spec § 3 implicitly assumes
src/state/sequencer.rs:131:/// 307 writes the apply_one signature as `Result<LedgerEntry, TransitionError>`;
src/state/sequencer.rs:300:    /// Driver loop. Drains the queue and runs `apply_one` on each tx. Errors
src/state/sequencer.rs:301:    /// from individual `apply_one` calls are logged and skipped (per-tx
src/state/sequencer.rs:309:            // Stub state: dispatch returns NotYetImplemented; apply_one
src/state/sequencer.rs:312:            if let Err(e) = self.apply_one(tx) {
src/state/sequencer.rs:313:                log::debug!("sequencer apply_one rejected: {e}");
src/state/sequencer.rs:332:    pub(crate) fn apply_one(&self, tx: TypedTx) -> Result<LedgerEntry, ApplyError> {
src/state/sequencer.rs:341:        let (q_next, _signals) = dispatch_transition(
src/state/sequencer.rs:515:    // 1. dispatch_transition: every variant returns NotYetImplemented (stub state).
src/state/sequencer.rs:517:    fn dispatch_transition_stubs_all_variants() {
src/state/sequencer.rs:582:            let result = dispatch_transition(&q, &tx, &preds, &tools);
src/state/sequencer.rs:604:    // 3. apply_one in stub mode: returns Transition(NotYetImplemented); no
src/state/sequencer.rs:607:    fn apply_one_stub_does_not_consume_logical_t() {
src/state/sequencer.rs:610:        let err = seq.apply_one(TypedTx::Work(fixture_work_tx())).unwrap_err();
src/state/sequencer.rs:613:        assert_eq!(pre, post, "logical_t MUST NOT advance on rejected apply_one");
src/state/typed_tx.rs:10://! downstream A2 (Sequencer + `dispatch_transition`) needed a `TypedTx` enum
src/state/typed_tx.rs:306:    pub reward: MicroCoin,                 //  5 — Q-derived authoritative (SettlementEngine output); wire = ledger summary
src/state/typed_tx.rs:603:/// TRACE_MATRIX § 8 dispatch_transition — typed-tx outer enum.
src/economy/monetary_invariant.rs:8://!   `MonetaryError` returns drive L4.E (rejection-evidence) entries, not L4.
src/economy/monetary_invariant.rs:18://! - Wired into `dispatch_transition` rejection path in TB-1 Day-3.
src/economy/monetary_invariant.rs:31:// MonetaryError — invariant-violation taxonomy
src/economy/monetary_invariant.rs:40:pub enum MonetaryError {
src/economy/monetary_invariant.rs:42:    /// [`assert_total_ctf_conserved`] when `delta_micro > 0` and no
src/economy/monetary_invariant.rs:57:impl std::fmt::Display for MonetaryError {
src/economy/monetary_invariant.rs:74:impl std::error::Error for MonetaryError {}
src/economy/monetary_invariant.rs:94:fn total_supply_micro(s: &EconomicState) -> Result<i64, MonetaryError> {
src/economy/monetary_invariant.rs:97:        total = total.checked_add(v.micro_units()).ok_or(MonetaryError::Overflow)?;
src/economy/monetary_invariant.rs:100:        total = total.checked_add(e.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
src/economy/monetary_invariant.rs:103:        total = total.checked_add(e.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
src/economy/monetary_invariant.rs:106:        total = total.checked_add(c.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
src/economy/monetary_invariant.rs:109:        total = total.checked_add(m.bounty.micro_units()).ok_or(MonetaryError::Overflow)?;
src/economy/monetary_invariant.rs:112:        total = total.checked_add(c.bond.micro_units()).ok_or(MonetaryError::Overflow)?;
src/economy/monetary_invariant.rs:118:// assert_no_post_init_mint — structural guard at the tx layer
src/economy/monetary_invariant.rs:133:/// [`assert_total_ctf_conserved`] separately.
src/economy/monetary_invariant.rs:134:pub fn assert_no_post_init_mint(tx: &TypedTx, q: &QState) -> Result<(), MonetaryError> {
src/economy/monetary_invariant.rs:151:// assert_total_ctf_conserved — numeric conservation across a transition
src/economy/monetary_invariant.rs:167:pub fn assert_total_ctf_conserved(
src/economy/monetary_invariant.rs:171:) -> Result<(), MonetaryError> {
src/economy/monetary_invariant.rs:176:        .ok_or(MonetaryError::Overflow)?;
src/economy/monetary_invariant.rs:181:        return Err(MonetaryError::PostInitMint { delta_micro: delta });
src/economy/monetary_invariant.rs:184:        return Err(MonetaryError::TotalCtfBurn { delta_micro: delta });
src/economy/monetary_invariant.rs:190:// assert_read_is_free — tx-level no-fee guard
src/economy/monetary_invariant.rs:202:pub fn assert_read_is_free(tx_kind: TxKind, fee: u64) -> Result<(), MonetaryError> {
src/economy/monetary_invariant.rs:204:        return Err(MonetaryError::ReadCharged { tx_kind, fee });
src/economy/monetary_invariant.rs:252:    // ── assert_no_post_init_mint ────────────────────────────────────────────
src/economy/monetary_invariant.rs:258:        assert_eq!(assert_no_post_init_mint(&work, &q), Ok(()));
src/economy/monetary_invariant.rs:277:            assert_eq!(assert_no_post_init_mint(&t, &q), Ok(()),
src/economy/monetary_invariant.rs:282:    // ── assert_total_ctf_conserved ──────────────────────────────────────────
src/economy/monetary_invariant.rs:292:        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
src/economy/monetary_invariant.rs:301:        let r = assert_total_ctf_conserved(&before, &after, &[]);
src/economy/monetary_invariant.rs:304:            Err(MonetaryError::PostInitMint { delta_micro: 50 * MICRO_PER_COIN })
src/economy/monetary_invariant.rs:313:        let r = assert_total_ctf_conserved(&before, &after, &[]);
src/economy/monetary_invariant.rs:316:            Err(MonetaryError::TotalCtfBurn { delta_micro: -60 * MICRO_PER_COIN })
src/economy/monetary_invariant.rs:326:            assert_total_ctf_conserved(&before, &after, &[TxKind::FinalizeReward]),
src/economy/monetary_invariant.rs:342:        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
src/economy/monetary_invariant.rs:374:            // (Concrete redistribution mechanics live in SettlementEngine;
src/economy/monetary_invariant.rs:438:    // ── assert_read_is_free ─────────────────────────────────────────────────
src/economy/monetary_invariant.rs:451:            assert_eq!(assert_read_is_free(k, 0), Ok(()));
src/economy/monetary_invariant.rs:458:        let r = assert_read_is_free(TxKind::Reuse, 1);
src/economy/monetary_invariant.rs:459:        assert_eq!(r, Err(MonetaryError::ReadCharged { tx_kind: TxKind::Reuse, fee: 1 }));
src/economy/monetary_invariant.rs:460:        let r = assert_read_is_free(TxKind::Work, 9999);
src/economy/monetary_invariant.rs:461:        assert_eq!(r, Err(MonetaryError::ReadCharged { tx_kind: TxKind::Work, fee: 9999 }));
src/bottom_white/ledger/transition_ledger.rs:300:    /// CO1.7-impl A4: dispatch_transition rejected the re-run. In stub state
src/bottom_white/ledger/transition_ledger.rs:336:            Self::Transition { at, inner } => write!(f, "dispatch_transition rejected at index {at}: {inner}"),
src/bottom_white/ledger/transition_ledger.rs:381:/// 7. dispatch_transition re-run produces (q_next, _signals)
src/bottom_white/ledger/transition_ledger.rs:387:/// so dispatch_transition can read budget / registries / balances / task markets
src/bottom_white/ledger/transition_ledger.rs:390:/// **Stub-state caveat (CO1.7.5 unblocks)**: while `dispatch_transition`
src/bottom_white/ledger/transition_ledger.rs:406:    use crate::state::sequencer::dispatch_transition;
src/bottom_white/ledger/transition_ledger.rs:468:        // Stage 7: re-run pure dispatch_transition.
src/bottom_white/ledger/transition_ledger.rs:470:            dispatch_transition(&q, &typed_tx, predicate_registry, tool_registry)
src/bottom_white/ledger/transition_ledger.rs:502:/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
src/bottom_white/ledger/transition_ledger.rs:1195:    /// `Sequencer::apply_one` stages 5-9 outside the runtime.
tests/conformance_stubs.rs:142:#[test] #[ignore] fn economic_invariant_inv2_no_direct_collect() { unimplemented!("CO P2.6 — only SettlementEngine pays") }
tests/tb_1_acceptance.rs:41:    assert_read_is_free, assert_total_ctf_conserved, MonetaryError,
tests/tb_1_acceptance.rs:311:    // a 5-step alice/bob/escrow shuffle and assert assert_total_ctf_conserved
tests/tb_1_acceptance.rs:321:    assert_eq!(assert_total_ctf_conserved(&baseline, &s1, &[]), Ok(()));
tests/tb_1_acceptance.rs:331:    assert_eq!(assert_total_ctf_conserved(&s1, &s2, &[]), Ok(()));
tests/tb_1_acceptance.rs:337:    assert_eq!(assert_total_ctf_conserved(&s2, &s3, &[]), Ok(()));
tests/tb_1_acceptance.rs:343:    assert_eq!(assert_total_ctf_conserved(&s3, &s4, &[]), Ok(()));
tests/tb_1_acceptance.rs:348:    assert_eq!(assert_total_ctf_conserved(&s4, &s5, &[]), Ok(()));
tests/tb_1_acceptance.rs:365:    // against ALL TxKinds. assert_read_is_free(kind, fee=0) must succeed for
tests/tb_1_acceptance.rs:377:            assert_read_is_free(kind, 0),
tests/tb_1_acceptance.rs:386:        assert_read_is_free(TxKind::Reuse, 1),
tests/tb_1_acceptance.rs:387:        Err(MonetaryError::ReadCharged {
tests/tb_1_acceptance.rs:393:        assert_read_is_free(TxKind::Work, 9999),
tests/tb_1_acceptance.rs:394:        Err(MonetaryError::ReadCharged {
tests/tb_1_acceptance.rs:407:    // The numeric kill: any post-init mint surfaces as MonetaryError::PostInitMint.
tests/tb_1_acceptance.rs:411:    let r = assert_total_ctf_conserved(&before, &after, &[]);
tests/tb_1_acceptance.rs:414:        Err(MonetaryError::PostInitMint {
tests/tb_1_acceptance.rs:421:    // Simulate the dispatch_transition rejection path by writing the rejection
tests/tb_1_acceptance.rs:471:// Non-blocking until WorkTx dispatch_transition body lands at TB-2 RSP-1.
tests/tb_1_acceptance.rs:473:#[ignore = "Tier-B: WorkTx dispatch_transition body lands TB-2 RSP-1; evaluator currently uses legacy emit path"]

 succeeded in 167ms:
cord>`; `prev_hash` chained; `submit_id`
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2645:pub struct RejectionEvidenceWriter {
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2646:    records: Vec<RejectedSubmissionRecord>,
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2649:impl RejectionEvidenceWriter {
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2676:    pub fn append_rejected(
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2684:        raw_diagnostic_cid: Option<Cid>,
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2688:        let hash = RejectedSubmissionRecord::compute_hash(
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2695:            &raw_diagnostic_cid,
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2699:        let record = RejectedSubmissionRecord {
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2706:            raw_diagnostic_cid,
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2726:            let recomputed = RejectedSubmissionRecord::compute_hash(
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2733:                &r.raw_diagnostic_cid,
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2746:    /// records carry `raw_diagnostic_cid` and MUST NOT be exposed across the
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2747:    /// agent boundary; use `public_view` for that).
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2748:    pub fn records(&self) -> &[RejectedSubmissionRecord] {
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2754:    /// `PublicRejectionView` does not carry `raw_diagnostic_cid` by type
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2757:    pub fn public_view(&self) -> Vec<PublicRejectionView> {
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2758:        self.records.iter().map(PublicRejectionView::from).collect()
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2790:        let mut w = RejectionEvidenceWriter::new();
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2791:        let h1 = w.append_rejected(
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2801:        let h2 = w.append_rejected(
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2820:    fn public_view_omits_raw_diagnostic_cid() {
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2821:        let mut w = RejectionEvidenceWriter::new();
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2822:        w.append_rejected(
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2832:        let view = w.public_view();
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2834:        // Structural isolation: `PublicRejectionView` doesn't have a
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2835:        // `raw_diagnostic_cid` field. Round-trip via JSON to assert the
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2839:        assert!(!obj.contains_key("raw_diagnostic_cid"));
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2845:        let mut w = RejectionEvidenceWriter::new();
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2846:        w.append_rejected(
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md:2856:        w.append_rejected(
handover/audits/2026-04-29_external_audit.md:191:pub struct RejectedSubmissionRecord {
handover/audits/2026-04-29_external_audit.md:198:    pub raw_diagnostic_cid: Option<Cid>,
handover/audits/2026-04-29_external_audit.md:214:            rejection_writer.append_rejected(submit_id, tx, inner, q_snapshot.state_root_t)?;
handover/audits/2026-04-29_external_audit.md:830:ReadViewCompiler filters raw_diagnostic_cid
handover/audits/2026-04-29_external_audit.md:1194:pub struct RejectedSubmissionRecord {
handover/audits/2026-04-29_external_audit.md:1201:    pub raw_diagnostic_cid: Option<Cid>,
handover/audits/2026-04-29_external_audit.md:1208:pub trait RejectionEvidenceWriter {
handover/audits/2026-04-29_external_audit.md:1209:    fn append_rejected(&mut self, record: RejectedSubmissionRecord) -> Result<Hash, RejectionEvidenceError>;
handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md:32:     - read view:         raw_diagnostic_cid is private/shielded; only
handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md:57:- L4.E entries default to private (raw_diagnostic_cid hidden); only the aggregate counter / public_summary surface unless an Agent has a privileged role (P3 ChallengeCourt, P5 ArchitectAI sandboxed read).
handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md:82:            self.rejection_writer.append_rejected(RejectedSubmissionRecord {
handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md:89:                raw_diagnostic_cid: Some(cas_put(inner.raw_diagnostic())),
handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md:108:`materializer.py` (or its Rust equivalent) MUST NOT include `raw_diagnostic_cid` content in any Agent's read view by default. Permitted L4.E projections in a default read view:
handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md:119:- `test_p1_kill_4_rejected_log_isolated`: another Agent's view has aggregate counter only; raw_diagnostic_cid content NOT visible.
handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md:124:- Whether L4.E is content-addressed (CAS) under-the-hood. Recommendation: yes (each `raw_diagnostic` and each `RejectedSubmissionRecord` payload becomes a CID), but the decision binds only the ledger-separation semantics, not the storage backend.
handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:1153:        ledger.append_rejected(tx, results, Q_t)
handover/tracer_bullets/TB-1_recharter_2026-04-29.md:68:  - `RejectedSubmissionRecord` struct with `submit_id`, `parent_state_root`, `agent_id`, `tx_kind`, `tx_payload_cid`, `rejection_class`, `raw_diagnostic_cid` (Option), `public_summary` (Option), `prev_hash`, `hash`.
handover/tracer_bullets/TB-1_recharter_2026-04-29.md:69:  - `RejectionEvidenceWriter::append_rejected()` returns the new chain hash.
handover/tracer_bullets/TB-1_recharter_2026-04-29.md:75:  - `test_p1_kill_2_rejected_tx_no_state_advance`: simulate a tx that fails predicate; assert `state_root` unchanged; assert **L4 logical_t NOT incremented**; assert **L4.E `submit_id`-scoped record IS appended** (one record, raw_diagnostic_cid populated).
handover/tracer_bullets/TB-1_recharter_2026-04-29.md:78:  - `test_p1_kill_4b_rejection_chain_breaks_on_row_deletion`: write 3 rejection-evidence records; delete row 2; `RejectionEvidenceWriter::verify_chain()` returns `Err(RejectionEvidenceError::HashMismatch { at: 2 })`.
src/bottom_white/ledger/rejection_evidence.rs:21://! - In-memory `Vec<RejectedSubmissionRecord>` chained via `prev_hash`.
src/bottom_white/ledger/rejection_evidence.rs:25://! - `raw_diagnostic_cid` is a CAS handle to the raw error bytes; the
src/bottom_white/ledger/rejection_evidence.rs:26://!   `PublicRejectionView` projection (used to materialize agent-facing
src/bottom_white/ledger/rejection_evidence.rs:70:// RejectedSubmissionRecord — one row on the L4.E chain
src/bottom_white/ledger/rejection_evidence.rs:79:/// - `raw_diagnostic_cid` holds the raw error content shielded behind a CAS
src/bottom_white/ledger/rejection_evidence.rs:83:pub struct RejectedSubmissionRecord {
src/bottom_white/ledger/rejection_evidence.rs:98:    /// `None` when no raw payload is captured. NEVER exposed via `PublicRejectionView`.
src/bottom_white/ledger/rejection_evidence.rs:99:    pub raw_diagnostic_cid: Option<Cid>,
src/bottom_white/ledger/rejection_evidence.rs:109:impl RejectedSubmissionRecord {
src/bottom_white/ledger/rejection_evidence.rs:117:        raw_diagnostic_cid: &Option<Cid>,
src/bottom_white/ledger/rejection_evidence.rs:130:        match raw_diagnostic_cid {
src/bottom_white/ledger/rejection_evidence.rs:151:// PublicRejectionView — agent-facing projection (Inv 10 Goodhart shield)
src/bottom_white/ledger/rejection_evidence.rs:157:/// `raw_diagnostic_cid`. Materializing this view from a
src/bottom_white/ledger/rejection_evidence.rs:158:/// `RejectedSubmissionRecord` cannot accidentally leak the raw diagnostic
src/bottom_white/ledger/rejection_evidence.rs:161:pub struct PublicRejectionView {
src/bottom_white/ledger/rejection_evidence.rs:170:impl From<&RejectedSubmissionRecord> for PublicRejectionView {
src/bottom_white/ledger/rejection_evidence.rs:171:    fn from(r: &RejectedSubmissionRecord) -> Self {
src/bottom_white/ledger/rejection_evidence.rs:187:/// TRACE_MATRIX P1:6 — error returned by `RejectionEvidenceWriter::verify_chain`.
src/bottom_white/ledger/rejection_evidence.rs:206:// RejectionEvidenceWriter — append + verify + project-to-public
src/bottom_white/ledger/rejection_evidence.rs:211:/// One `Vec<RejectedSubmissionRecord>`; `prev_hash` chained; `submit_id`
src/bottom_white/ledger/rejection_evidence.rs:217:pub struct RejectionEvidenceWriter {
src/bottom_white/ledger/rejection_evidence.rs:218:    records: Vec<RejectedSubmissionRecord>,
src/bottom_white/ledger/rejection_evidence.rs:221:impl RejectionEvidenceWriter {
src/bottom_white/ledger/rejection_evidence.rs:248:    pub fn append_rejected(
src/bottom_white/ledger/rejection_evidence.rs:256:        raw_diagnostic_cid: Option<Cid>,
src/bottom_white/ledger/rejection_evidence.rs:260:        let hash = RejectedSubmissionRecord::compute_hash(
src/bottom_white/ledger/rejection_evidence.rs:267:            &raw_diagnostic_cid,
src/bottom_white/ledger/rejection_evidence.rs:271:        let record = RejectedSubmissionRecord {
src/bottom_white/ledger/rejection_evidence.rs:278:            raw_diagnostic_cid,
src/bottom_white/ledger/rejection_evidence.rs:298:            let recomputed = RejectedSubmissionRecord::compute_hash(
src/bottom_white/ledger/rejection_evidence.rs:305:                &r.raw_diagnostic_cid,
src/bottom_white/ledger/rejection_evidence.rs:318:    /// records carry `raw_diagnostic_cid` and MUST NOT be exposed across the
src/bottom_white/ledger/rejection_evidence.rs:319:    /// agent boundary; use `public_view` for that).
src/bottom_white/ledger/rejection_evidence.rs:320:    pub fn records(&self) -> &[RejectedSubmissionRecord] {
src/bottom_white/ledger/rejection_evidence.rs:326:    /// `PublicRejectionView` does not carry `raw_diagnostic_cid` by type
src/bottom_white/ledger/rejection_evidence.rs:329:    pub fn public_view(&self) -> Vec<PublicRejectionView> {
src/bottom_white/ledger/rejection_evidence.rs:330:        self.records.iter().map(PublicRejectionView::from).collect()
src/bottom_white/ledger/rejection_evidence.rs:362:        let mut w = RejectionEvidenceWriter::new();
src/bottom_white/ledger/rejection_evidence.rs:363:        let h1 = w.append_rejected(
src/bottom_white/ledger/rejection_evidence.rs:373:        let h2 = w.append_rejected(
src/bottom_white/ledger/rejection_evidence.rs:392:    fn public_view_omits_raw_diagnostic_cid() {
src/bottom_white/ledger/rejection_evidence.rs:393:        let mut w = RejectionEvidenceWriter::new();
src/bottom_white/ledger/rejection_evidence.rs:394:        w.append_rejected(
src/bottom_white/ledger/rejection_evidence.rs:404:        let view = w.public_view();
src/bottom_white/ledger/rejection_evidence.rs:406:        // Structural isolation: `PublicRejectionView` doesn't have a
src/bottom_white/ledger/rejection_evidence.rs:407:        // `raw_diagnostic_cid` field. Round-trip via JSON to assert the
src/bottom_white/ledger/rejection_evidence.rs:411:        assert!(!obj.contains_key("raw_diagnostic_cid"));
src/bottom_white/ledger/rejection_evidence.rs:417:        let mut w = RejectionEvidenceWriter::new();
src/bottom_white/ledger/rejection_evidence.rs:418:        w.append_rejected(
src/bottom_white/ledger/rejection_evidence.rs:428:        w.append_rejected(
tests/tb_1_acceptance.rs:35:    PublicRejectionView, RejectionClass, RejectionEvidenceError, RejectionEvidenceWriter,
tests/tb_1_acceptance.rs:149:    let mut l4e = RejectionEvidenceWriter::new();
tests/tb_1_acceptance.rs:155:    l4e.append_rejected(
tests/tb_1_acceptance.rs:180:    assert!(r.raw_diagnostic_cid.is_some());
tests/tb_1_acceptance.rs:216:    let mut l4e = RejectionEvidenceWriter::new();
tests/tb_1_acceptance.rs:217:    l4e.append_rejected(
tests/tb_1_acceptance.rs:228:    let view: Vec<PublicRejectionView> = l4e.public_view();
tests/tb_1_acceptance.rs:232:    let obj = json.as_object().expect("PublicRejectionView serializes as object");
tests/tb_1_acceptance.rs:234:        !obj.contains_key("raw_diagnostic_cid"),
tests/tb_1_acceptance.rs:235:        "raw_diagnostic_cid must NOT appear in agent-facing public view"
tests/tb_1_acceptance.rs:243:        l4e.records()[0].raw_diagnostic_cid.is_some(),
tests/tb_1_acceptance.rs:244:        "L4.E forensic record must retain raw_diagnostic_cid (shielding is structural, not destructive)"
tests/tb_1_acceptance.rs:279:    let mut l4e = RejectionEvidenceWriter::new();
tests/tb_1_acceptance.rs:281:        l4e.append_rejected(
tests/tb_1_acceptance.rs:427:    let mut l4e = RejectionEvidenceWriter::new();
tests/tb_1_acceptance.rs:428:    l4e.append_rejected(

 succeeded in 1263ms:
../turingos/AGENTS.md
../omega_kepler_discovery/AGENTS.md
../omega/AGENTS.md
../turingosv2/AGENTS.md
../turingclaw/AGENTS.md

codex
The first pass shows the acceptance tests are mostly exercising new pure/in-memory APIs. I’m drilling into the real sequencer/ledger call paths now, because that’s where a malicious implementer could bypass direct-function tests.
exec
/bin/bash -lc "nl -ba src/economy/ledger.rs | sed -n '1,240p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! L4 accepted-only ledger wrapper — TB-1 Day-3 P1.
     2	//!
     3	//! Charter authority:
     4	//! - `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-3.
     5	//! - ROADMAP P1 Exit 5 (state_root advances on accept), Exit 6 (state_root
     6	//!   unchanged on reject), Exit 7 (ledger hash chain), Exit 8 (state.db
     7	//!   reconstructable from chaintape).
     8	//! - `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`:
     9	//!   accepted transitions ONLY land here; rejected submissions go to L4.E
    10	//!   (`bottom_white::ledger::rejection_evidence`).
    11	//!
    12	//! Constitutional authority:
    13	//! - WP § 5.L4 — ChainTape Layer 4 spine; one entry per accepted transition.
    14	//! - Art IV (Boot) — every Q_t field MUST be reconstructible by replaying L4.
    15	//! - Inv 7 (no rejection on the accepted spine) — rejections never advance
    16	//!   `state_root_t` / `ledger_root_t`.
    17	//!
    18	//! Scope (RSP-0 minimum-viable wrapper):
    19	//! - Self-contained accepted-only hash chain over `TypedTx` canonical bytes.
    20	//! - `append_accepted` advances `logical_t` and chains `prev_hash`.
    21	//! - `verify_chain(start, end)` walks the hash chain over `[start, end)`.
    22	//! - `reconstruct_state` replays L4 only and returns the canonical
    23	//!   `state_root_t` (L4.E is intentionally NOT consulted).
    24	//! - Persistence helpers (`persist` / `load_from_path`) provide the
    25	//!   "drop state.db; reconstruct from L4" round-trip used by P1 kill
    26	//!   acceptance tests.
    27	//!
    28	//! Out of scope (deferred to CO1.7.5+):
    29	//! - `SystemSignature` attachment (full signing payload + epoch binding).
    30	//! - `dispatch_transition` re-run (state_root mutation requires CO1.8).
    31	//! - Real `Git2LedgerWriter` commit chain — that's the production backend
    32	//!   over `refs/transitions/main`; this RSP-0 wrapper uses an in-memory Vec.
    33	//!
    34	//! /// TRACE_MATRIX WP § 5.L4 + Art IV + ROADMAP P1:5/P1:6/P1:7/P1:8: L4 accepted-only ledger.
    35	
    36	use std::path::Path;
    37	
    38	use serde::{Deserialize, Serialize};
    39	use sha2::{Digest, Sha256};
    40	
    41	use crate::bottom_white::ledger::transition_ledger::{canonical_encode, TxKind};
    42	use crate::state::q_state::Hash;
    43	use crate::state::typed_tx::TypedTx;
    44	
    45	// ────────────────────────────────────────────────────────────────────────────
    46	// AcceptedEntry — one row on the L4 accepted-only chain
    47	// ────────────────────────────────────────────────────────────────────────────
    48	
    49	/// TRACE_MATRIX P1:5 — one accepted-only L4 row.
    50	///
    51	/// All seven fields enter the hash; tampering any single field breaks
    52	/// `verify_chain` at the affected index. The `tx_payload_hash` is the
    53	/// SHA-256 over the bincode-canonical encoding of the source `TypedTx`,
    54	/// re-using the lower-level `canonical_encode` from `transition_ledger`.
    55	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    56	pub struct AcceptedEntry {
    57	    /// 1-based monotonic counter; advances ONLY on accept (not on reject —
    58	    /// rejections take a `submit_id` on L4.E instead, per the L4/L4.E split).
    59	    pub logical_t: u64,
    60	    /// Hash of the immediately-preceding entry; `Hash::ZERO` for the first row.
    61	    pub prev_hash: Hash,
    62	    /// Discriminator over the source `TypedTx` variant.
    63	    pub tx_kind: TxKind,
    64	    /// SHA-256 of `canonical_encode(tx)` — content-address of the payload.
    65	    pub tx_payload_hash: Hash,
    66	    /// State-root before this entry was applied.
    67	    pub parent_state_root: Hash,
    68	    /// State-root after this entry was applied. Computed by `next_state_root`
    69	    /// (the RSP-0 toy mutator); a real `dispatch_transition` lands in CO1.7.5.
    70	    pub resulting_state_root: Hash,
    71	    /// SHA-256 over the six fields above plus a domain-separation prefix.
    72	    pub hash: Hash,
    73	}
    74	
    75	impl AcceptedEntry {
    76	    fn compute_hash(
    77	        logical_t: u64,
    78	        prev_hash: &Hash,
    79	        tx_kind: TxKind,
    80	        tx_payload_hash: &Hash,
    81	        parent_state_root: &Hash,
    82	        resulting_state_root: &Hash,
    83	    ) -> Hash {
    84	        let mut h = Sha256::new();
    85	        h.update(b"turingosv4.l4_accepted.v1");
    86	        h.update(logical_t.to_be_bytes());
    87	        h.update(prev_hash.0);
    88	        h.update((tx_kind as u8).to_be_bytes());
    89	        h.update(tx_payload_hash.0);
    90	        h.update(parent_state_root.0);
    91	        h.update(resulting_state_root.0);
    92	        Hash(h.finalize().into())
    93	    }
    94	}
    95	
    96	// ────────────────────────────────────────────────────────────────────────────
    97	// LedgerError — shared error taxonomy for append / verify / reconstruct
    98	// ────────────────────────────────────────────────────────────────────────────
    99	
   100	/// TRACE_MATRIX P1:5/P1:6/P1:7/P1:8 — error taxonomy for the L4 wrapper.
   101	#[derive(Debug, Clone, PartialEq, Eq)]
   102	pub enum LedgerError {
   103	    /// `verify_chain` walked off the end of `entries`.
   104	    OutOfBounds { len: usize, requested_end: usize },
   105	    /// Hash mismatch at the given chain index (prev_hash break OR entry hash break).
   106	    HashMismatch { at_index: usize },
   107	    /// `logical_t` is not the expected `index + 1` value.
   108	    LogicalTGap { at_index: usize, expected: u64, got: u64 },
   109	    /// `parent_state_root` doesn't match the running replay state.
   110	    ParentStateMismatch { at_index: usize },
   111	    /// `canonical_encode` of the source `TypedTx` failed.
   112	    Encode(String),
   113	    /// File system or JSON serialization error during persist / load.
   114	    Io(String),
   115	}
   116	
   117	impl std::fmt::Display for LedgerError {
   118	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   119	        match self {
   120	            Self::OutOfBounds { len, requested_end } => {
   121	                write!(f, "verify_chain end={} exceeds chain len={}", requested_end, len)
   122	            }
   123	            Self::HashMismatch { at_index } => {
   124	                write!(f, "L4 hash chain break at index {}", at_index)
   125	            }
   126	            Self::LogicalTGap { at_index, expected, got } => write!(
   127	                f,
   128	                "logical_t gap at index {}: expected {}, got {}",
   129	                at_index, expected, got
   130	            ),
   131	            Self::ParentStateMismatch { at_index } => {
   132	                write!(f, "parent_state_root mismatch at index {}", at_index)
   133	            }
   134	            Self::Encode(e) => write!(f, "canonical_encode failed: {}", e),
   135	            Self::Io(e) => write!(f, "persistence I/O failed: {}", e),
   136	        }
   137	    }
   138	}
   139	
   140	impl std::error::Error for LedgerError {}
   141	
   142	/// TRACE_MATRIX P1:7 — `verify_chain` failure alias; kept distinct from
   143	/// `ReconstructError` so callers can pattern-match on chain-walk vs replay.
   144	pub type ChainError = LedgerError;
   145	/// TRACE_MATRIX P1:8 — `reconstruct_state` / `load_from_path` failure alias;
   146	/// distinct from `ChainError` so replay errors are syntactically separable.
   147	pub type ReconstructError = LedgerError;
   148	
   149	// ────────────────────────────────────────────────────────────────────────────
   150	// AcceptedLedger — the wrapper itself
   151	// ────────────────────────────────────────────────────────────────────────────
   152	
   153	/// TRACE_MATRIX P1:5/P1:6/P1:7/P1:8 — accepted-only L4 hash chain (RSP-0).
   154	///
   155	/// Single source of truth for the accepted spine. Rejected transitions
   156	/// MUST NOT touch this struct; they take a `submit_id` on L4.E
   157	/// (`bottom_white::ledger::rejection_evidence`).
   158	#[derive(Debug, Clone, Default)]
   159	pub struct AcceptedLedger {
   160	    entries: Vec<AcceptedEntry>,
   161	    current_state_root: Hash,
   162	}
   163	
   164	impl AcceptedLedger {
   165	    /// TRACE_MATRIX Art IV Boot — empty L4 (genesis state_root is `Hash::ZERO`).
   166	    pub fn new() -> Self {
   167	        Self::default()
   168	    }
   169	
   170	    /// TRACE_MATRIX P1:5 — count of accepted rows.
   171	    pub fn len(&self) -> usize {
   172	        self.entries.len()
   173	    }
   174	
   175	    /// TRACE_MATRIX P1:5 — empty predicate.
   176	    pub fn is_empty(&self) -> bool {
   177	        self.entries.is_empty()
   178	    }
   179	
   180	    /// TRACE_MATRIX P1:5 — current canonical `state_root_t`.
   181	    pub fn current_state_root(&self) -> Hash {
   182	        self.current_state_root
   183	    }
   184	
   185	    /// TRACE_MATRIX P1:5 — append-accepted entry; advances `logical_t` by 1.
   186	    ///
   187	    /// Advances `current_state_root` via the toy mutator `next_state_root`.
   188	    /// Returns the freshly-built `AcceptedEntry` (clone of what was pushed).
   189	    pub fn append_accepted(&mut self, tx: &TypedTx) -> Result<AcceptedEntry, LedgerError> {
   190	        let bytes = canonical_encode(tx).map_err(|e| LedgerError::Encode(e.to_string()))?;
   191	        let tx_payload_hash = sha256_of(&bytes);
   192	        let prev_hash = self.entries.last().map(|e| e.hash).unwrap_or(Hash::ZERO);
   193	        let logical_t = (self.entries.len() as u64) + 1;
   194	        let parent_state_root = self.current_state_root;
   195	        let tx_kind = tx.tx_kind();
   196	        let resulting_state_root = next_state_root(&parent_state_root, &tx_payload_hash);
   197	        let hash = AcceptedEntry::compute_hash(
   198	            logical_t,
   199	            &prev_hash,
   200	            tx_kind,
   201	            &tx_payload_hash,
   202	            &parent_state_root,
   203	            &resulting_state_root,
   204	        );
   205	        let entry = AcceptedEntry {
   206	            logical_t,
   207	            prev_hash,
   208	            tx_kind,
   209	            tx_payload_hash,
   210	            parent_state_root,
   211	            resulting_state_root,
   212	            hash,
   213	        };
   214	        self.entries.push(entry.clone());
   215	        self.current_state_root = resulting_state_root;
   216	        Ok(entry)
   217	    }
   218	
   219	    /// TRACE_MATRIX P1:7 — verify hash-chain integrity over `[start, end)`.
   220	    ///
   221	    /// Returns `Err(HashMismatch)` if any single field (logical_t, prev_hash,
   222	    /// tx_payload_hash, parent_state_root, resulting_state_root, tx_kind, or
   223	    /// the entry hash itself) was tampered.
   224	    pub fn verify_chain(&self, start: usize, end: usize) -> Result<(), ChainError> {
   225	        if end > self.entries.len() {
   226	            return Err(LedgerError::OutOfBounds {
   227	                len: self.entries.len(),
   228	                requested_end: end,
   229	            });
   230	        }
   231	        if start > end {
   232	            return Err(LedgerError::OutOfBounds {
   233	                len: self.entries.len(),
   234	                requested_end: start,
   235	            });
   236	        }
   237	        let mut prev = if start == 0 {
   238	            Hash::ZERO
   239	        } else {
   240	            self.entries[start - 1].hash

exec
/bin/bash -lc "nl -ba src/economy/ledger.rs | sed -n '240,380p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   240	            self.entries[start - 1].hash
   241	        };
   242	        for i in start..end {
   243	            let e = &self.entries[i];
   244	            let expected_logical_t = (i as u64) + 1;
   245	            if e.logical_t != expected_logical_t {
   246	                return Err(LedgerError::LogicalTGap {
   247	                    at_index: i,
   248	                    expected: expected_logical_t,
   249	                    got: e.logical_t,
   250	                });
   251	            }
   252	            if e.prev_hash != prev {
   253	                return Err(LedgerError::HashMismatch { at_index: i });
   254	            }
   255	            let recomputed = AcceptedEntry::compute_hash(
   256	                e.logical_t,
   257	                &e.prev_hash,
   258	                e.tx_kind,
   259	                &e.tx_payload_hash,
   260	                &e.parent_state_root,
   261	                &e.resulting_state_root,
   262	            );
   263	            if recomputed != e.hash {
   264	                return Err(LedgerError::HashMismatch { at_index: i });
   265	            }
   266	            prev = e.hash;
   267	        }
   268	        Ok(())
   269	    }
   270	
   271	    /// TRACE_MATRIX P1:8 — replay L4 only; recompute the canonical `state_root_t`.
   272	    ///
   273	    /// L4.E is intentionally NOT consulted: rejected submissions never affect
   274	    /// `state_root_t` (Inv 7).
   275	    pub fn reconstruct_state(&self) -> Result<Hash, ReconstructError> {
   276	        let mut s = Hash::ZERO;
   277	        for (i, e) in self.entries.iter().enumerate() {
   278	            if e.parent_state_root != s {
   279	                return Err(LedgerError::ParentStateMismatch { at_index: i });
   280	            }
   281	            let expected = next_state_root(&s, &e.tx_payload_hash);
   282	            if e.resulting_state_root != expected {
   283	                return Err(LedgerError::HashMismatch { at_index: i });
   284	            }
   285	            s = e.resulting_state_root;
   286	        }
   287	        Ok(s)
   288	    }
   289	
   290	    /// TRACE_MATRIX P1:8 — persist entries to `state_path` for cold restart.
   291	    pub fn persist(&self, state_path: &Path) -> Result<(), LedgerError> {
   292	        let bytes = serde_json::to_vec(&self.entries).map_err(|e| LedgerError::Io(e.to_string()))?;
   293	        std::fs::write(state_path, bytes).map_err(|e| LedgerError::Io(e.to_string()))?;
   294	        Ok(())
   295	    }
   296	
   297	    /// TRACE_MATRIX P1:8 — load entries from `state_path` and recompute the
   298	    /// canonical `state_root_t`. Used by the "drop state.db; reconstruct from L4"
   299	    /// kill test: any direct mutation that bypassed the L4 path is washed out.
   300	    pub fn load_from_path(state_path: &Path) -> Result<(Self, Hash), ReconstructError> {
   301	        let bytes = std::fs::read(state_path).map_err(|e| LedgerError::Io(e.to_string()))?;
   302	        let entries: Vec<AcceptedEntry> =
   303	            serde_json::from_slice(&bytes).map_err(|e| LedgerError::Io(e.to_string()))?;
   304	        let mut l = Self {
   305	            entries,
   306	            current_state_root: Hash::ZERO,
   307	        };
   308	        let s = l.reconstruct_state()?;
   309	        l.current_state_root = s;
   310	        Ok((l, s))
   311	    }
   312	
   313	    /// TRACE_MATRIX P1:7 — read-only entry slice (for replay / debug / external
   314	    /// tooling that wants to inspect the chain without mutating it).
   315	    pub fn entries(&self) -> &[AcceptedEntry] {
   316	        &self.entries
   317	    }
   318	
   319	    /// TRACE_MATRIX P1:7 — TAMPER-ONLY hook used by kill-criteria integration
   320	    /// tests to simulate adversarial row deletion. The `tamper_` prefix and
   321	    /// `#[doc(hidden)]` mark this as not part of the supported API; production
   322	    /// callers MUST NOT use it. Kept `pub` (rather than `cfg(test)`) only so
   323	    /// integration tests in `tests/` can reach it; integration tests link
   324	    /// against the lib without `cfg(test)` enabled.
   325	    #[doc(hidden)]
   326	    pub fn tamper_remove_entry(&mut self, idx: usize) {
   327	        self.entries.remove(idx);
   328	    }
   329	}
   330	
   331	// ────────────────────────────────────────────────────────────────────────────
   332	// Helpers
   333	// ────────────────────────────────────────────────────────────────────────────
   334	
   335	fn sha256_of(bytes: &[u8]) -> Hash {
   336	    let mut h = Sha256::new();
   337	    h.update(bytes);
   338	    Hash(h.finalize().into())
   339	}
   340	
   341	/// RSP-0 toy state mutator: `next = SHA-256(domain || prev_state_root || tx_payload_hash)`.
   342	///
   343	/// This is a minimum-viable demonstration of the state-root-advances-on-accept
   344	/// invariant. The real `dispatch_transition`-driven state_root mutation lands
   345	/// in CO1.7.5 / CO1.8 (proper economic + agent-swarm state evolution).
   346	fn next_state_root(prev: &Hash, tx_payload_hash: &Hash) -> Hash {
   347	    let mut h = Sha256::new();
   348	    h.update(b"turingosv4.l4_state_root.v1");
   349	    h.update(prev.0);
   350	    h.update(tx_payload_hash.0);
   351	    Hash(h.finalize().into())
   352	}
   353	
   354	// ────────────────────────────────────────────────────────────────────────────
   355	// Inline correctness tests (round-trip + tamper detection on every field).
   356	// Cross-cutting P1 kill acceptance tests live in `tests/tb_1_p1_acceptance.rs`.
   357	// ────────────────────────────────────────────────────────────────────────────
   358	
   359	#[cfg(test)]
   360	mod tests {
   361	    use super::*;
   362	    use crate::state::q_state::{AgentId, TxId};
   363	    use crate::state::typed_tx::{
   364	        AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey,
   365	        SafetyOrCreation, TaskId, TypedTx, WorkTx, WriteKey,
   366	    };
   367	    use crate::bottom_white::cas::schema::Cid;
   368	    use crate::economy::money::StakeMicroCoin;
   369	    use std::collections::{BTreeMap, BTreeSet};
   370	
   371	    fn fixture_work_tx(suffix: u32) -> TypedTx {
   372	        let mut acceptance = BTreeMap::new();
   373	        acceptance.insert(
   374	            PredicateId(format!("acc-{}", suffix)),
   375	            BoolWithProof { value: true, proof_cid: Some(Cid([0x11; 32])) },
   376	        );
   377	        let mut settlement = BTreeMap::new();
   378	        settlement.insert(
   379	            PredicateId(format!("set-{}", suffix)),
   380	            BoolWithProof { value: true, proof_cid: None },

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/rejection_evidence.rs | sed -n '70,340p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    70	// RejectedSubmissionRecord — one row on the L4.E chain
    71	// ────────────────────────────────────────────────────────────────────────────
    72	
    73	/// TRACE_MATRIX P1:6/P1:9 — one rejection-evidence row.
    74	///
    75	/// Distinguished from `LedgerEntry` (the L4 accepted spine):
    76	/// - keyed by `submit_id` (not `logical_t`);
    77	/// - records `parent_state_root` for the snapshot-at-submit but never a
    78	///   `resulting_state_root` (rejection MUST NOT advance state);
    79	/// - `raw_diagnostic_cid` holds the raw error content shielded behind a CAS
    80	///   handle (not exposed in agent-facing views);
    81	/// - `public_summary` is the ONLY field permitted to cross the agent boundary.
    82	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    83	pub struct RejectedSubmissionRecord {
    84	    /// Independent submit-side counter from `Sequencer::next_submit_id`.
    85	    pub submit_id: u64,
    86	    /// State-root snapshot at submit time — recorded for forensics; NEVER
    87	    /// advanced by rejection (Inv 7).
    88	    pub parent_state_root: Hash,
    89	    /// Submitter agent (opaque string).
    90	    pub agent_id: AgentId,
    91	    /// Discriminator over the submitted (now-rejected) `TypedTx` variant.
    92	    pub tx_kind: TxKind,
    93	    /// CAS handle to the canonical-encoded source `TypedTx`.
    94	    pub tx_payload_cid: Cid,
    95	    /// Coarse why-class (one of `RejectionClass`).
    96	    pub rejection_class: RejectionClass,
    97	    /// CAS handle to the raw diagnostic bytes (e.g. predicate counter-example).
    98	    /// `None` when no raw payload is captured. NEVER exposed via `PublicRejectionView`.
    99	    pub raw_diagnostic_cid: Option<Cid>,
   100	    /// Agent-facing summary string. `None` when no public summary is permitted
   101	    /// (raw-diagnostic-only mode). The ONLY field that crosses the agent boundary.
   102	    pub public_summary: Option<String>,
   103	    /// Hash of the immediately-preceding rejection record; `Hash::ZERO` for the first.
   104	    pub prev_hash: Hash,
   105	    /// SHA-256 over the nine fields above plus a domain-separation prefix.
   106	    pub hash: Hash,
   107	}
   108	
   109	impl RejectedSubmissionRecord {
   110	    fn compute_hash(
   111	        submit_id: u64,
   112	        parent_state_root: &Hash,
   113	        agent_id: &AgentId,
   114	        tx_kind: TxKind,
   115	        tx_payload_cid: &Cid,
   116	        rejection_class: RejectionClass,
   117	        raw_diagnostic_cid: &Option<Cid>,
   118	        public_summary: &Option<String>,
   119	        prev_hash: &Hash,
   120	    ) -> Hash {
   121	        let mut h = Sha256::new();
   122	        h.update(b"turingosv4.l4e_rejection_evidence.v1");
   123	        h.update(submit_id.to_be_bytes());
   124	        h.update(parent_state_root.0);
   125	        h.update((agent_id.0.len() as u64).to_be_bytes());
   126	        h.update(agent_id.0.as_bytes());
   127	        h.update((tx_kind as u8).to_be_bytes());
   128	        h.update(tx_payload_cid.0);
   129	        h.update((rejection_class as u8).to_be_bytes());
   130	        match raw_diagnostic_cid {
   131	            Some(c) => {
   132	                h.update([1u8]);
   133	                h.update(c.0);
   134	            }
   135	            None => h.update([0u8]),
   136	        }
   137	        match public_summary {
   138	            Some(s) => {
   139	                h.update([1u8]);
   140	                h.update((s.len() as u64).to_be_bytes());
   141	                h.update(s.as_bytes());
   142	            }
   143	            None => h.update([0u8]),
   144	        }
   145	        h.update(prev_hash.0);
   146	        Hash(h.finalize().into())
   147	    }
   148	}
   149	
   150	// ────────────────────────────────────────────────────────────────────────────
   151	// PublicRejectionView — agent-facing projection (Inv 10 Goodhart shield)
   152	// ────────────────────────────────────────────────────────────────────────────
   153	
   154	/// TRACE_MATRIX Inv 10 + ROADMAP P1:9 — agent-facing projection.
   155	///
   156	/// **Structural** isolation: the type itself does not carry
   157	/// `raw_diagnostic_cid`. Materializing this view from a
   158	/// `RejectedSubmissionRecord` cannot accidentally leak the raw diagnostic
   159	/// because there is no field to write it into.
   160	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   161	pub struct PublicRejectionView {
   162	    pub submit_id: u64,
   163	    pub parent_state_root: Hash,
   164	    pub agent_id: AgentId,
   165	    pub tx_kind: TxKind,
   166	    pub rejection_class: RejectionClass,
   167	    pub public_summary: Option<String>,
   168	}
   169	
   170	impl From<&RejectedSubmissionRecord> for PublicRejectionView {
   171	    fn from(r: &RejectedSubmissionRecord) -> Self {
   172	        Self {
   173	            submit_id: r.submit_id,
   174	            parent_state_root: r.parent_state_root,
   175	            agent_id: r.agent_id.clone(),
   176	            tx_kind: r.tx_kind,
   177	            rejection_class: r.rejection_class,
   178	            public_summary: r.public_summary.clone(),
   179	        }
   180	    }
   181	}
   182	
   183	// ────────────────────────────────────────────────────────────────────────────
   184	// RejectionEvidenceError — chain-walk failure taxonomy
   185	// ────────────────────────────────────────────────────────────────────────────
   186	
   187	/// TRACE_MATRIX P1:6 — error returned by `RejectionEvidenceWriter::verify_chain`.
   188	#[derive(Debug, Clone, PartialEq, Eq)]
   189	pub enum RejectionEvidenceError {
   190	    /// `prev_hash` chain or per-record hash diverged at the given index
   191	    /// (covers row deletion, field tampering, and reordering).
   192	    HashMismatch { at: usize },
   193	}
   194	
   195	impl std::fmt::Display for RejectionEvidenceError {
   196	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   197	        match self {
   198	            Self::HashMismatch { at } => write!(f, "rejection-evidence chain break at index {}", at),
   199	        }
   200	    }
   201	}
   202	
   203	impl std::error::Error for RejectionEvidenceError {}
   204	
   205	// ────────────────────────────────────────────────────────────────────────────
   206	// RejectionEvidenceWriter — append + verify + project-to-public
   207	// ────────────────────────────────────────────────────────────────────────────
   208	
   209	/// TRACE_MATRIX P1:6/P1:9 — RSP-0 in-memory rejection-evidence writer.
   210	///
   211	/// One `Vec<RejectedSubmissionRecord>`; `prev_hash` chained; `submit_id`
   212	/// monotonicity is the caller's responsibility (the writer trusts the
   213	/// `Sequencer::next_submit_id` issuer). No `logical_t` field — accepted
   214	/// spine and rejection-evidence are intentionally disjoint per the L4 / L4.E
   215	/// split (`DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`).
   216	#[derive(Debug, Clone, Default)]
   217	pub struct RejectionEvidenceWriter {
   218	    records: Vec<RejectedSubmissionRecord>,
   219	}
   220	
   221	impl RejectionEvidenceWriter {
   222	    /// TRACE_MATRIX P1:6 — empty writer.
   223	    pub fn new() -> Self {
   224	        Self::default()
   225	    }
   226	
   227	    /// TRACE_MATRIX P1:6 — count of recorded rejections.
   228	    pub fn len(&self) -> usize {
   229	        self.records.len()
   230	    }
   231	
   232	    /// TRACE_MATRIX P1:6 — empty predicate.
   233	    pub fn is_empty(&self) -> bool {
   234	        self.records.is_empty()
   235	    }
   236	
   237	    /// TRACE_MATRIX P1:6 — last record's hash, or `Hash::ZERO` for empty chain.
   238	    pub fn last_hash(&self) -> Hash {
   239	        self.records.last().map(|r| r.hash).unwrap_or(Hash::ZERO)
   240	    }
   241	
   242	    /// TRACE_MATRIX P1:6/P1:9 — append a rejection record; returns the new chain hash.
   243	    ///
   244	    /// CRITICAL: this method MUST NOT be called from the L4 (accepted) write
   245	    /// path — Inv 7 forbids state-root advance on rejection. The caller's
   246	    /// dispatch logic decides which ledger receives the record.
   247	    #[allow(clippy::too_many_arguments)]
   248	    pub fn append_rejected(
   249	        &mut self,
   250	        submit_id: u64,
   251	        parent_state_root: Hash,
   252	        agent_id: AgentId,
   253	        tx_kind: TxKind,
   254	        tx_payload_cid: Cid,
   255	        rejection_class: RejectionClass,
   256	        raw_diagnostic_cid: Option<Cid>,
   257	        public_summary: Option<String>,
   258	    ) -> Hash {
   259	        let prev_hash = self.last_hash();
   260	        let hash = RejectedSubmissionRecord::compute_hash(
   261	            submit_id,
   262	            &parent_state_root,
   263	            &agent_id,
   264	            tx_kind,
   265	            &tx_payload_cid,
   266	            rejection_class,
   267	            &raw_diagnostic_cid,
   268	            &public_summary,
   269	            &prev_hash,
   270	        );
   271	        let record = RejectedSubmissionRecord {
   272	            submit_id,
   273	            parent_state_root,
   274	            agent_id,
   275	            tx_kind,
   276	            tx_payload_cid,
   277	            rejection_class,
   278	            raw_diagnostic_cid,
   279	            public_summary,
   280	            prev_hash,
   281	            hash,
   282	        };
   283	        self.records.push(record);
   284	        hash
   285	    }
   286	
   287	    /// TRACE_MATRIX P1:6 — verify the rejection-evidence chain end-to-end.
   288	    ///
   289	    /// Returns `Err(HashMismatch)` if any single field of any record was
   290	    /// tampered, or if a row was deleted (the surviving row's `prev_hash`
   291	    /// no longer matches its predecessor's `hash`).
   292	    pub fn verify_chain(&self) -> Result<(), RejectionEvidenceError> {
   293	        let mut prev = Hash::ZERO;
   294	        for (i, r) in self.records.iter().enumerate() {
   295	            if r.prev_hash != prev {
   296	                return Err(RejectionEvidenceError::HashMismatch { at: i });
   297	            }
   298	            let recomputed = RejectedSubmissionRecord::compute_hash(
   299	                r.submit_id,
   300	                &r.parent_state_root,
   301	                &r.agent_id,
   302	                r.tx_kind,
   303	                &r.tx_payload_cid,
   304	                r.rejection_class,
   305	                &r.raw_diagnostic_cid,
   306	                &r.public_summary,
   307	                &r.prev_hash,
   308	            );
   309	            if recomputed != r.hash {
   310	                return Err(RejectionEvidenceError::HashMismatch { at: i });
   311	            }
   312	            prev = r.hash;
   313	        }
   314	        Ok(())
   315	    }
   316	
   317	    /// TRACE_MATRIX P1:9 — read-only record slice (for L4.E forensics; full
   318	    /// records carry `raw_diagnostic_cid` and MUST NOT be exposed across the
   319	    /// agent boundary; use `public_view` for that).
   320	    pub fn records(&self) -> &[RejectedSubmissionRecord] {
   321	        &self.records
   322	    }
   323	
   324	    /// TRACE_MATRIX Inv 10 + P1:9 — agent-facing projection.
   325	    ///
   326	    /// `PublicRejectionView` does not carry `raw_diagnostic_cid` by type
   327	    /// construction; this method's output is safe to materialize into another
   328	    /// agent's read view.
   329	    pub fn public_view(&self) -> Vec<PublicRejectionView> {
   330	        self.records.iter().map(PublicRejectionView::from).collect()
   331	    }
   332	
   333	    /// TRACE_MATRIX P1:6 — TAMPER-ONLY hook used by kill-criteria integration
   334	    /// tests (`test_p1_kill_4b_rejection_chain_breaks_on_row_deletion`).
   335	    /// `#[doc(hidden)]` + `tamper_` prefix flags any production use as a
   336	    /// reviewable violation; kept `pub` only so integration tests in `tests/`
   337	    /// can reach it (they link against the lib without `cfg(test)` enabled).
   338	    #[doc(hidden)]
   339	    pub fn tamper_remove_record(&mut self, idx: usize) {
   340	        self.records.remove(idx);

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '37,380p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    37	// § 8 dispatch_transition — exhaustive enum match (K5: NO Slash)
    38	// ────────────────────────────────────────────────────────────────────────────
    39	
    40	/// TRACE_MATRIX § 8 — exhaustive dispatch over `TypedTx` variants.
    41	///
    42	/// **Stub state (CO1.7-impl A3)**: every variant returns
    43	/// `TransitionError::NotYetImplemented`. CO1.7.5 fills each arm with the real
    44	/// transition body per `STATE_TRANSITION_SPEC § 3.1-3.7`. The exhaustive match
    45	/// itself is the contract: any future TypedTx variant addition triggers a
    46	/// non-exhaustive-match compile error here, forcing explicit handling.
    47	pub(crate) fn dispatch_transition(
    48	    _q: &QState,
    49	    tx: &TypedTx,
    50	    _predicate_registry: &PredicateRegistry,
    51	    _tool_registry: &ToolRegistry,
    52	) -> Result<(QState, SignalBundle), TransitionError> {
    53	    match tx {
    54	        TypedTx::Work(_) => Err(TransitionError::NotYetImplemented),
    55	        TypedTx::Verify(_) => Err(TransitionError::NotYetImplemented),
    56	        TypedTx::Challenge(_) => Err(TransitionError::NotYetImplemented),
    57	        TypedTx::Reuse(_) => Err(TransitionError::NotYetImplemented),
    58	        TypedTx::FinalizeReward(_) => Err(TransitionError::NotYetImplemented),
    59	        TypedTx::TaskExpire(_) => Err(TransitionError::NotYetImplemented),
    60	        TypedTx::TerminalSummary(_) => Err(TransitionError::NotYetImplemented),
    61	    }
    62	}
    63	
    64	// ────────────────────────────────────────────────────────────────────────────
    65	// CO1.7-extra D2: advance_head_t — post-commit head_t close (Art 0.4)
    66	// ────────────────────────────────────────────────────────────────────────────
    67	
    68	/// TRACE_MATRIX § 5 — L4 sequencer post-commit head_t wiring (Art 0.4).
    69	///
    70	/// Closes the G-1 carry-forward: when `writer` surfaces a commit OID hex
    71	/// (Git2LedgerWriter), advance `q.head_t = state::q_state::NodeId(hex)`;
    72	/// when `writer` returns None (InMemoryLedgerWriter), leave `q.head_t`
    73	/// unchanged (no-op preservation).
    74	///
    75	/// Called from `apply_one` stage 9 AFTER `writer.commit` succeeds. Pure
    76	/// function (writer is `&dyn` so behavior depends only on writer's
    77	/// `head_commit_oid_hex` return + q's prior state).
    78	///
    79	/// **Visibility** (CO1.7-extra round-3 B2): `pub` (NOT `pub(crate)`) so that
    80	/// flat integration tests under `tests/co1_7_extra_*.rs` per round-2 MF5 can
    81	/// call this helper directly.
    82	///
    83	/// **Atomicity** (CO1.7-extra round-2 MF9): in apply_one, called under the
    84	/// `q_w` write lock immediately after `writer.commit` returns Ok. For Git2
    85	/// (Some path), this is post-commit non-failing best-effort head binding —
    86	/// `q.head_t`, `q.ledger_root_t`, and `next_logical_t` advance atomically.
    87	/// For InMemory (None path), this is explicit no-op preservation —
    88	/// `q.head_t` stays at the value `*q_w = q_next` left it (which equals the
    89	/// prior value because pure transition bodies never mutate head_t per
    90	/// CO1.7 K3 v1.2).
    91	pub fn advance_head_t(q: &mut QState, writer: &dyn LedgerWriter) {
    92	    if let Some(commit_oid_hex) = writer.head_commit_oid_hex() {
    93	        q.head_t = crate::state::q_state::NodeId(commit_oid_hex);
    94	    }
    95	}
    96	
    97	// ────────────────────────────────────────────────────────────────────────────
    98	// Submission types — K1 dual counter
    99	// ────────────────────────────────────────────────────────────────────────────
   100	
   101	/// Returned by `Sequencer::submit`. Carries `submit_id` (always assigned at
   102	/// submit time) but **NOT** `logical_t` — logical_t is only assigned post-accept
   103	/// per K1 (see spec § 3 + CO1.7 K1 closure).
   104	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
   105	pub struct SubmissionReceipt {
   106	    pub submit_id: u64,
   107	}
   108	
   109	#[derive(Debug)]
   110	pub enum SubmitError {
   111	    /// Bounded queue saturated (Q1/Q2 resolution: agent retries with backoff).
   112	    QueueFull,
   113	    /// Receiver dropped — sequencer no longer running.
   114	    QueueClosed,
   115	}
   116	
   117	impl std::fmt::Display for SubmitError {
   118	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   119	        match self {
   120	            Self::QueueFull => write!(f, "submission queue saturated"),
   121	            Self::QueueClosed => write!(f, "submission queue closed"),
   122	        }
   123	    }
   124	}
   125	impl std::error::Error for SubmitError {}
   126	
   127	/// Errors that can occur during `apply_one`. Spec § 3 implicitly assumes
   128	/// `Result<_, TransitionError>` but the actual `?`-propagated error chain
   129	/// crosses CAS, keypair, and ledger-writer boundaries — wrapper enum captures
   130	/// all of these explicitly. **Implementation note vs. spec**: spec § 3 line
   131	/// 307 writes the apply_one signature as `Result<LedgerEntry, TransitionError>`;
   132	/// this implementation widens to `Result<LedgerEntry, ApplyError>` to preserve
   133	/// distinct error provenance (TransitionError keeps its closed taxonomy +
   134	/// additive-only invariant per CO1.1.4-pre1 § 7.2).
   135	#[derive(Debug)]
   136	pub enum ApplyError {
   137	    /// Pure transition function rejected the tx.
   138	    Transition(TransitionError),
   139	    /// CAS payload put failed.
   140	    Cas(CasError),
   141	    /// System keypair sign failed.
   142	    Keypair(KeypairError),
   143	    /// Ledger writer commit failed.
   144	    LedgerCommit(LedgerWriterError),
   145	    /// Internal: canonical encoding of typed-tx payload failed (should never
   146	    /// happen for serde-derive types; surfaced for completeness).
   147	    PayloadEncode(String),
   148	    /// `q.read()` / `q.write()` lock poisoned by panicking thread.
   149	    QStateLockPoisoned,
   150	}
   151	
   152	impl std::fmt::Display for ApplyError {
   153	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   154	        match self {
   155	            Self::Transition(e) => write!(f, "transition rejected: {e}"),
   156	            Self::Cas(e) => write!(f, "cas put failed: {e}"),
   157	            Self::Keypair(e) => write!(f, "keypair sign failed: {e:?}"),
   158	            Self::LedgerCommit(e) => write!(f, "ledger commit failed: {e}"),
   159	            Self::PayloadEncode(s) => write!(f, "payload encode failed: {s}"),
   160	            Self::QStateLockPoisoned => write!(f, "q-state lock poisoned"),
   161	        }
   162	    }
   163	}
   164	impl std::error::Error for ApplyError {}
   165	
   166	impl From<TransitionError> for ApplyError {
   167	    fn from(e: TransitionError) -> Self {
   168	        Self::Transition(e)
   169	    }
   170	}
   171	impl From<CasError> for ApplyError {
   172	    fn from(e: CasError) -> Self {
   173	        Self::Cas(e)
   174	    }
   175	}
   176	impl From<KeypairError> for ApplyError {
   177	    fn from(e: KeypairError) -> Self {
   178	        Self::Keypair(e)
   179	    }
   180	}
   181	impl From<LedgerWriterError> for ApplyError {
   182	    fn from(e: LedgerWriterError) -> Self {
   183	        Self::LedgerCommit(e)
   184	    }
   185	}
   186	
   187	#[derive(Debug)]
   188	pub enum SequencerError {
   189	    /// `run()` was called when the receiver had already been consumed.
   190	    ReceiverAlreadyTaken,
   191	}
   192	
   193	impl std::fmt::Display for SequencerError {
   194	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   195	        match self {
   196	            Self::ReceiverAlreadyTaken => write!(f, "sequencer receiver already taken"),
   197	        }
   198	    }
   199	}
   200	impl std::error::Error for SequencerError {}
   201	
   202	// ────────────────────────────────────────────────────────────────────────────
   203	// Sequencer — single-writer per (runtime_repo, run_id)
   204	// ────────────────────────────────────────────────────────────────────────────
   205	
   206	/// TRACE_MATRIX § 5.2.1 — L4 sequencer; single-writer per (runtime_repo, run_id).
   207	///
   208	/// **K1 dual counter**: `next_submit_id` advances at every `submit()` (used to
   209	/// derive `SubmissionReceipt.submit_id`); `next_logical_t` advances ONLY at
   210	/// commit time (rejected submissions never get a logical_t — preserves
   211	/// `LedgerWriter`'s strict logical_t monotonicity invariant).
   212	///
   213	/// **K3 v1.2 + CO1.7-extra D2 (revised)**: the pure transition function does
   214	/// NOT mutate `q.head_t` or `q.state_root_t`; it returns the new `QState`
   215	/// and the sequencer accepts it as-is. `head_t` mutation now happens
   216	/// post-commit via `advance_head_t()` (CO1.7-extra D2): when
   217	/// `LedgerWriter::head_commit_oid_hex()` returns Some (Git2LedgerWriter),
   218	/// the sequencer writes `q.head_t = NodeId(commit_oid_hex)`; when None
   219	/// (InMemoryLedgerWriter), `head_t` is left unchanged (no-op preservation).
   220	///
   221	/// **C3 sign API**: signs through
   222	/// `transition_ledger_emitter::sign_ledger_entry(keypair, digest_bytes)` —
   223	/// the typed `CanonicalMessage::LedgerEntrySigning([u8;32])` extension closes
   224	/// the C3 round-2 audit point.
   225	/// **CO1.7-extra D3 (round-2 MF6)**: manual `Debug` impl below — `#[derive(Debug)]`
   226	/// fails because `Arc<Ed25519Keypair>` field has no Debug derive (intentional;
   227	/// `Ed25519Keypair` derives only `Zeroize, ZeroizeOnDrop` for secret-handling).
   228	/// `finish_non_exhaustive()` leaks no keypair / QState / CAS contents and
   229	/// satisfies Debug propagation through `Arc<Sequencer>` for `TuringBus.Debug`.
   230	pub struct Sequencer {
   231	    /// K1: assigned at submit; never appears in LedgerEntry.
   232	    next_submit_id: AtomicU64,
   233	    /// K1: advances ONLY on commit; first accepted entry gets logical_t=1.
   234	    next_logical_t: AtomicU64,
   235	
   236	    queue_tx: tokio::sync::mpsc::Sender<TypedTx>,
   237	
   238	    cas: Arc<RwLock<CasStore>>,
   239	    keypair: Arc<Ed25519Keypair>,
   240	    epoch: SystemEpoch,
   241	    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
   242	
   243	    predicate_registry: Arc<PredicateRegistry>,
   244	    tool_registry: Arc<ToolRegistry>,
   245	
   246	    q: RwLock<QState>,
   247	}
   248	
   249	/// CO1.7-extra D3 (round-2 MF6): manual Debug impl. Uses `finish_non_exhaustive()`
   250	/// to satisfy the Debug trait without exposing keypair / QState / CAS internals.
   251	impl std::fmt::Debug for Sequencer {
   252	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   253	        f.debug_struct("Sequencer").finish_non_exhaustive()
   254	    }
   255	}
   256	
   257	impl Sequencer {
   258	    /// Construct. Returns the `Sequencer` plus the receiver half of the
   259	    /// internal mpsc; pass the receiver to `run()` exactly once.
   260	    #[allow(clippy::too_many_arguments)]
   261	    pub fn new(
   262	        cas: Arc<RwLock<CasStore>>,
   263	        keypair: Arc<Ed25519Keypair>,
   264	        epoch: SystemEpoch,
   265	        ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
   266	        predicate_registry: Arc<PredicateRegistry>,
   267	        tool_registry: Arc<ToolRegistry>,
   268	        initial_q: QState,
   269	        queue_capacity: usize,
   270	    ) -> (Self, tokio::sync::mpsc::Receiver<TypedTx>) {
   271	        let (queue_tx, queue_rx) = tokio::sync::mpsc::channel(queue_capacity);
   272	        let seq = Self {
   273	            next_submit_id: AtomicU64::new(1),
   274	            next_logical_t: AtomicU64::new(0), // first accepted commit advances to 1
   275	            queue_tx,
   276	            cas,
   277	            keypair,
   278	            epoch,
   279	            ledger_writer,
   280	            predicate_registry,
   281	            tool_registry,
   282	            q: RwLock::new(initial_q),
   283	        };
   284	        (seq, queue_rx)
   285	    }
   286	
   287	    /// Submit a typed transition. Returns immediately with a receipt carrying
   288	    /// `submit_id` (NOT `logical_t`). Per Q2 (back-pressure resolution): on
   289	    /// queue saturation returns `Err(SubmitError::QueueFull)` and the agent is
   290	    /// expected to retry with deterministic exponential backoff.
   291	    pub async fn submit(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
   292	        let submit_id = self.next_submit_id.fetch_add(1, Ordering::SeqCst);
   293	        match self.queue_tx.try_send(tx) {
   294	            Ok(()) => Ok(SubmissionReceipt { submit_id }),
   295	            Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => Err(SubmitError::QueueFull),
   296	            Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => Err(SubmitError::QueueClosed),
   297	        }
   298	    }
   299	
   300	    /// Driver loop. Drains the queue and runs `apply_one` on each tx. Errors
   301	    /// from individual `apply_one` calls are logged and skipped (per-tx
   302	    /// rejection does NOT halt the sequencer). Returns when the queue is
   303	    /// closed and drained.
   304	    pub async fn run(
   305	        &self,
   306	        mut queue_rx: tokio::sync::mpsc::Receiver<TypedTx>,
   307	    ) -> Result<(), SequencerError> {
   308	        while let Some(tx) = queue_rx.recv().await {
   309	            // Stub state: dispatch returns NotYetImplemented; apply_one
   310	            // bubbles up. We log and continue per spec § 3 v1.2 ordering rule
   311	            // (rejection does not consume a logical_t — see K1).
   312	            if let Err(e) = self.apply_one(tx) {
   313	                log::debug!("sequencer apply_one rejected: {e}");
   314	            }
   315	        }
   316	        Ok(())
   317	    }
   318	
   319	    /// Per-tx critical section. Pure transition + CAS put + sign + commit +
   320	    /// Q_t mutation. See spec § 3 stages 1-9.
   321	    ///
   322	    /// **v1.1 C-2 closure (Codex bundle Q-B)**: `next_logical_t` advances
   323	    /// **only on commit success** — the original spec § 3 stage-4
   324	    /// `fetch_add(1)` happened BEFORE sign + writer.commit, so any infra
   325	    /// failure (sign / commit) left `next_logical_t` advanced past a
   326	    /// logical_t that was never written to the ledger. The next accepted
   327	    /// tx would then be assigned a logical_t the writer rejects forever
   328	    /// (writer enforces strict `len + 1`). Fixed by `load → use → store
   329	    /// after commit succeeds`. Single-writer per spec § 5.2.1 makes the
   330	    /// load+store atomic enough; if multi-writer ever lands the AtomicU64
   331	    /// can be upgraded to a `compare_exchange` reservation pattern.
   332	    pub(crate) fn apply_one(&self, tx: TypedTx) -> Result<LedgerEntry, ApplyError> {
   333	        // Stage 1: snapshot Q_t under read lock.
   334	        let q_snapshot = {
   335	            let g = self.q.read().map_err(|_| ApplyError::QStateLockPoisoned)?;
   336	            g.clone()
   337	        };
   338	
   339	        // Stage 2: dispatch (pure). On reject (incl. NotYetImplemented stub),
   340	        // EARLY RETURN. K1: no logical_t consumed.
   341	        let (q_next, _signals) = dispatch_transition(
   342	            &q_snapshot,
   343	            &tx,
   344	            &self.predicate_registry,
   345	            &self.tool_registry,
   346	        )?;
   347	
   348	        // v1.1 C-2: TENTATIVE logical_t (do NOT fetch_add yet).
   349	        let logical_t = self.next_logical_t.load(Ordering::SeqCst) + 1;
   350	
   351	        // Stage 3: put payload to CAS. DIV-5 5-param put signature.
   352	        let payload_bytes = canonical_encode(&tx)
   353	            .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
   354	        let payload_cid = {
   355	            let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
   356	            cas_w.put(
   357	                &payload_bytes,
   358	                ObjectType::ProposalPayload,
   359	                &format!("sequencer-epoch-{}", self.epoch.get()),
   360	                logical_t,
   361	                Some("TypedTx.v1".to_string()),
   362	            )?
   363	        };
   364	
   365	        // Stage 5: build LedgerEntrySigningPayload (v1.1 — stage 4 fetch_add
   366	        // moved to AFTER stage 9 commit success).
   367	        let signing_payload = LedgerEntrySigningPayload {
   368	            logical_t,
   369	            parent_state_root: q_snapshot.state_root_t,
   370	            parent_ledger_root: q_snapshot.ledger_root_t,
   371	            tx_kind: tx.tx_kind(),
   372	            tx_payload_cid: payload_cid,
   373	            resulting_state_root: q_next.state_root_t,
   374	            timestamp_logical: logical_t,
   375	            epoch: self.epoch,
   376	            extensions: std::collections::BTreeMap::new(),
   377	        };
   378	
   379	        // Stage 6: C3 — sign via typed CanonicalMessage::LedgerEntrySigning.
   380	        let signing_digest = signing_payload.canonical_digest();

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '380,650p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   380	        let signing_digest = signing_payload.canonical_digest();
   381	        let system_signature = transition_ledger_emitter::sign_ledger_entry(
   382	            &self.keypair,
   383	            signing_digest.0,
   384	        )?;
   385	
   386	        // Stage 7: pure ledger-root fold (deterministic).
   387	        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);
   388	
   389	        // Stage 8: build LedgerEntry (the stored record).
   390	        let entry = LedgerEntry {
   391	            logical_t: signing_payload.logical_t,
   392	            parent_state_root: signing_payload.parent_state_root,
   393	            parent_ledger_root: signing_payload.parent_ledger_root,
   394	            tx_kind: signing_payload.tx_kind,
   395	            tx_payload_cid: signing_payload.tx_payload_cid,
   396	            resulting_state_root: signing_payload.resulting_state_root,
   397	            resulting_ledger_root,
   398	            timestamp_logical: signing_payload.timestamp_logical,
   399	            epoch: signing_payload.epoch,
   400	            extensions: signing_payload.extensions,
   401	            system_signature,
   402	        };
   403	
   404	        // Stage 9: commit + mutate Q_t under write lock.
   405	        // v1.1 C-2: next_logical_t.store(logical_t) HAPPENS ONLY AFTER
   406	        // writer.commit succeeds — preserves K1 under infra failure.
   407	        // CO1.7-extra D2: q.head_t = NodeId(commit_oid_hex) via advance_head_t
   408	        // when writer surfaces a commit OID (Git2 path); no-op preservation
   409	        // for writers that return None (InMemory path). state_root_t comes
   410	        // from q_next as-is per K3 v1.2.
   411	        {
   412	            let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
   413	            let mut writer_w = self
   414	                .ledger_writer
   415	                .write()
   416	                .map_err(|_| ApplyError::QStateLockPoisoned)?;
   417	            writer_w.commit(&entry)?; // ← may fail; if it does, fetch_add was NOT called
   418	            // commit succeeded → safe to advance counter.
   419	            self.next_logical_t.store(logical_t, Ordering::SeqCst);
   420	            *q_w = q_next;
   421	            q_w.ledger_root_t = entry.resulting_ledger_root;
   422	            // CO1.7-extra D2: close G-1 head_t carry-forward (Art 0.4).
   423	            advance_head_t(&mut *q_w, &*writer_w);
   424	        }
   425	
   426	        Ok(entry)
   427	    }
   428	
   429	    /// Read-only accessor (testing + CO1.7.5+ wiring).
   430	    pub fn q_snapshot(&self) -> Result<QState, ApplyError> {
   431	        self.q
   432	            .read()
   433	            .map(|g| g.clone())
   434	            .map_err(|_| ApplyError::QStateLockPoisoned)
   435	    }
   436	
   437	    pub fn next_submit_id_peek(&self) -> u64 {
   438	        self.next_submit_id.load(Ordering::SeqCst)
   439	    }
   440	
   441	    pub fn next_logical_t_peek(&self) -> u64 {
   442	        self.next_logical_t.load(Ordering::SeqCst)
   443	    }
   444	}
   445	
   446	// ────────────────────────────────────────────────────────────────────────────
   447	// Tests — stub-mode coverage (CO1.7.5 fills real-transition tests)
   448	// ────────────────────────────────────────────────────────────────────────────
   449	
   450	#[cfg(test)]
   451	mod tests {
   452	    use super::*;
   453	    use crate::bottom_white::ledger::transition_ledger::InMemoryLedgerWriter;
   454	    use crate::state::typed_tx::{
   455	        AgentSignature, BoolWithProof, ChallengeTx, ClaimId, FinalizeRewardTx, PredicateId,
   456	        PredicateResultsBundle, ReadKey, ReuseTx, RunId, RunOutcome, SafetyOrCreation,
   457	        TaskExpireTx, TaskId, TerminalSummaryTx, ToolId, VerifyTx, VerifyVerdict, WorkTx,
   458	        WriteKey,
   459	    };
   460	    use crate::state::q_state::{AgentId, TxId};
   461	    use crate::economy::money::{MicroCoin, StakeMicroCoin};
   462	    use crate::bottom_white::cas::schema::Cid;
   463	    use crate::bottom_white::ledger::system_keypair::SystemSignature;
   464	    use std::collections::{BTreeMap, BTreeSet};
   465	    use tempfile::TempDir;
   466	
   467	    fn fresh_sequencer() -> (
   468	        TempDir,
   469	        Sequencer,
   470	        tokio::sync::mpsc::Receiver<TypedTx>,
   471	    ) {
   472	        let tmp = TempDir::new().expect("tempdir");
   473	        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas open")));
   474	        let keypair = Arc::new(
   475	            Ed25519Keypair::generate_with_secure_entropy().expect("keypair gen"),
   476	        );
   477	        let epoch = SystemEpoch::new(1);
   478	        let writer: Arc<RwLock<dyn LedgerWriter>> =
   479	            Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
   480	        let preds = Arc::new(PredicateRegistry::new());
   481	        let tools = Arc::new(ToolRegistry::new());
   482	        let q = QState::genesis();
   483	        let (seq, rx) = Sequencer::new(cas, keypair, epoch, writer, preds, tools, q, 16);
   484	        (tmp, seq, rx)
   485	    }
   486	
   487	    fn fixture_work_tx() -> WorkTx {
   488	        let mut acceptance = BTreeMap::new();
   489	        acceptance.insert(
   490	            PredicateId("acc1".into()),
   491	            BoolWithProof {
   492	                value: true,
   493	                proof_cid: None,
   494	            },
   495	        );
   496	        WorkTx {
   497	            tx_id: TxId("worktx-seq-fixture".into()),
   498	            task_id: TaskId("task-seq-fixture".into()),
   499	            parent_state_root: Default::default(),
   500	            agent_id: AgentId("alice".into()),
   501	            read_set: [ReadKey("k.read.a".into())].into_iter().collect::<BTreeSet<_>>(),
   502	            write_set: [WriteKey("k.write.a".into())].into_iter().collect::<BTreeSet<_>>(),
   503	            proposal_cid: Default::default(),
   504	            predicate_results: PredicateResultsBundle {
   505	                acceptance,
   506	                settlement: BTreeMap::new(),
   507	                safety_class: SafetyOrCreation::Safety,
   508	            },
   509	            stake: StakeMicroCoin::from_micro_units(1_000_000),
   510	            signature: AgentSignature::from_bytes([0x77u8; 64]),
   511	            timestamp_logical: 1,
   512	        }
   513	    }
   514	
   515	    // 1. dispatch_transition: every variant returns NotYetImplemented (stub state).
   516	    #[test]
   517	    fn dispatch_transition_stubs_all_variants() {
   518	        let q = QState::genesis();
   519	        let preds = PredicateRegistry::new();
   520	        let tools = ToolRegistry::new();
   521	
   522	        let cases: Vec<TypedTx> = vec![
   523	            TypedTx::Work(fixture_work_tx()),
   524	            TypedTx::Verify(VerifyTx {
   525	                tx_id: TxId("vt".into()),
   526	                target_work_tx: TxId("wt".into()),
   527	                verifier_agent: AgentId("v".into()),
   528	                bond: StakeMicroCoin::from_micro_units(1),
   529	                verdict: VerifyVerdict::Confirm,
   530	                signature: AgentSignature::from_bytes([0; 64]),
   531	                timestamp_logical: 1,
   532	            }),
   533	            TypedTx::Challenge(ChallengeTx {
   534	                tx_id: TxId("ct".into()),
   535	                target_work_tx: TxId("wt".into()),
   536	                challenger_agent: AgentId("c".into()),
   537	                stake: StakeMicroCoin::from_micro_units(1),
   538	                counterexample_cid: Cid([0; 32]),
   539	                signature: AgentSignature::from_bytes([0; 64]),
   540	                timestamp_logical: 1,
   541	            }),
   542	            TypedTx::Reuse(ReuseTx {
   543	                tx_id: TxId("rt".into()),
   544	                reusing_work_tx: TxId("wt".into()),
   545	                reused_tool_id: ToolId("tool".into()),
   546	                reused_tool_creator: AgentId("a".into()),
   547	                timestamp_logical: 1,
   548	            }),
   549	            TypedTx::FinalizeReward(FinalizeRewardTx {
   550	                tx_id: TxId("ft".into()),
   551	                claim_id: ClaimId::new("cl"),
   552	                task_id: TaskId("t".into()),
   553	                solver: AgentId("s".into()),
   554	                reward: MicroCoin::from_micro_units(1),
   555	                parent_state_root: Default::default(),
   556	                epoch: SystemEpoch::new(1),
   557	                timestamp_logical: 1,
   558	                system_signature: SystemSignature::from_bytes([0; 64]),
   559	            }),
   560	            TypedTx::TaskExpire(TaskExpireTx {
   561	                tx_id: TxId("et".into()),
   562	                task_id: TaskId("t".into()),
   563	                parent_state_root: Default::default(),
   564	                bounty_refunded: MicroCoin::from_micro_units(1),
   565	                epoch: SystemEpoch::new(1),
   566	                timestamp_logical: 1,
   567	                system_signature: SystemSignature::from_bytes([0; 64]),
   568	            }),
   569	            TypedTx::TerminalSummary(TerminalSummaryTx {
   570	                tx_id: TxId("ts".into()),
   571	                task_id: TaskId("t".into()),
   572	                run_id: RunId("r".into()),
   573	                run_outcome: RunOutcome::OmegaAccepted,
   574	                total_attempts: 0,
   575	                failure_class_histogram: BTreeMap::new(),
   576	                last_logical_t: 0,
   577	                system_signature: SystemSignature::from_bytes([0; 64]),
   578	            }),
   579	        ];
   580	
   581	        for tx in cases {
   582	            let result = dispatch_transition(&q, &tx, &preds, &tools);
   583	            assert!(matches!(result, Err(TransitionError::NotYetImplemented)));
   584	        }
   585	    }
   586	
   587	    // 2. K1 dual counter: submit advances submit_id but NOT logical_t.
   588	    #[tokio::test]
   589	    async fn submit_advances_submit_id_only() {
   590	        let (_tmp, seq, _rx) = fresh_sequencer();
   591	        assert_eq!(seq.next_submit_id_peek(), 1);
   592	        assert_eq!(seq.next_logical_t_peek(), 0);
   593	
   594	        let r1 = seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("submit 1");
   595	        assert_eq!(r1.submit_id, 1);
   596	        assert_eq!(seq.next_submit_id_peek(), 2);
   597	        assert_eq!(seq.next_logical_t_peek(), 0, "logical_t MUST NOT advance at submit");
   598	
   599	        let r2 = seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("submit 2");
   600	        assert_eq!(r2.submit_id, 2);
   601	        assert_eq!(seq.next_logical_t_peek(), 0);
   602	    }
   603	
   604	    // 3. apply_one in stub mode: returns Transition(NotYetImplemented); no
   605	    //    logical_t consumed (K1 invariant: rejected submission never advances commit counter).
   606	    #[test]
   607	    fn apply_one_stub_does_not_consume_logical_t() {
   608	        let (_tmp, seq, _rx) = fresh_sequencer();
   609	        let pre = seq.next_logical_t_peek();
   610	        let err = seq.apply_one(TypedTx::Work(fixture_work_tx())).unwrap_err();
   611	        assert!(matches!(err, ApplyError::Transition(TransitionError::NotYetImplemented)));
   612	        let post = seq.next_logical_t_peek();
   613	        assert_eq!(pre, post, "logical_t MUST NOT advance on rejected apply_one");
   614	    }
   615	
   616	    // 4. Queue saturation: submit returns QueueFull (Q1/Q2 resolution).
   617	    #[tokio::test]
   618	    async fn submit_returns_queue_full_on_saturation() {
   619	        // Capacity=2; receiver never drained.
   620	        let tmp = TempDir::new().expect("tempdir");
   621	        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
   622	        let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("kp"));
   623	        let writer: Arc<RwLock<dyn LedgerWriter>> =
   624	            Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
   625	        let preds = Arc::new(PredicateRegistry::new());
   626	        let tools = Arc::new(ToolRegistry::new());
   627	        let (seq, _rx) = Sequencer::new(
   628	            cas,
   629	            keypair,
   630	            SystemEpoch::new(1),
   631	            writer,
   632	            preds,
   633	            tools,
   634	            QState::genesis(),
   635	            2,
   636	        );
   637	        // Fill capacity.
   638	        seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("1");
   639	        seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("2");
   640	        // Saturated.
   641	        let err = seq.submit(TypedTx::Work(fixture_work_tx())).await.unwrap_err();
   642	        assert!(matches!(err, SubmitError::QueueFull));
   643	    }
   644	
   645	    // 5. submit returns QueueClosed when receiver dropped.
   646	    #[tokio::test]
   647	    async fn submit_returns_queue_closed_after_rx_drop() {
   648	        let (_tmp, seq, rx) = fresh_sequencer();
   649	        drop(rx);
   650	        let err = seq.submit(TypedTx::Work(fixture_work_tx())).await.unwrap_err();

exec
/bin/bash -lc "nl -ba tests/tb_1_acceptance.rs | sed -n '90,460p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    90	    })
    91	}
    92	
    93	fn cid(byte: u8) -> Cid {
    94	    Cid([byte; 32])
    95	}
    96	
    97	fn agent(s: &str) -> AgentId {
    98	    AgentId(s.to_string())
    99	}
   100	
   101	fn coin(n: i64) -> MicroCoin {
   102	    MicroCoin::from_coin(n).unwrap()
   103	}
   104	
   105	// ════════════════════════════════════════════════════════════════════════════
   106	// Tier-A — BLOCKING
   107	// ════════════════════════════════════════════════════════════════════════════
   108	
   109	// ────────────────────────────────────────────────────────────────────────────
   110	// (1) P1 kill 1 — no wtool bypass
   111	// ────────────────────────────────────────────────────────────────────────────
   112	
   113	#[test]
   114	fn test_p1_kill_1_no_wtool_bypass() {
   115	    let mut l = AcceptedLedger::new();
   116	    for i in 1..=3 {
   117	        l.append_accepted(&fixture_work_tx(i)).unwrap();
   118	    }
   119	    let canonical_root = l.current_state_root();
   120	
   121	    let tmp = tempfile::NamedTempFile::new().unwrap();
   122	    l.persist(tmp.path()).unwrap();
   123	
   124	    // Bypass: directly overwrite state.db without going through L4.
   125	    let raw = std::fs::read(tmp.path()).unwrap();
   126	    let mut tampered: Vec<turingosv4::economy::ledger::AcceptedEntry> =
   127	        serde_json::from_slice(&raw).unwrap();
   128	    tampered.last_mut().unwrap().resulting_state_root = Hash([0xFF; 32]);
   129	    let bytes = serde_json::to_vec(&tampered).unwrap();
   130	    std::fs::write(tmp.path(), bytes).unwrap();
   131	
   132	    // Reconstruction MUST fail: explicit error OR diverged root.
   133	    match AcceptedLedger::load_from_path(tmp.path()) {
   134	        Err(_) => {} // bypass detected — expected
   135	        Ok((_, reconstructed)) => assert_ne!(
   136	            reconstructed, canonical_root,
   137	            "bypass mutation must not survive a round-trip through reconstruct_state"
   138	        ),
   139	    }
   140	}
   141	
   142	// ────────────────────────────────────────────────────────────────────────────
   143	// (2) P1 kill 2 — rejected tx does not advance state
   144	// ────────────────────────────────────────────────────────────────────────────
   145	
   146	#[test]
   147	fn test_p1_kill_2_rejected_tx_no_state_advance() {
   148	    let mut l4 = AcceptedLedger::new();
   149	    let mut l4e = RejectionEvidenceWriter::new();
   150	
   151	    l4.append_accepted(&fixture_work_tx(1)).unwrap();
   152	    let baseline_root = l4.current_state_root();
   153	    let baseline_logical_t = l4.len();
   154	
   155	    l4e.append_rejected(
   156	        42,
   157	        baseline_root,
   158	        agent("alice"),
   159	        TxKind::Work,
   160	        cid(0x20),
   161	        RejectionClass::PredicateFailed,
   162	        Some(cid(0xAA)),
   163	        None,
   164	    );
   165	
   166	    assert_eq!(
   167	        l4.current_state_root(),
   168	        baseline_root,
   169	        "rejected tx must NOT advance L4 state_root"
   170	    );
   171	    assert_eq!(
   172	        l4.len(),
   173	        baseline_logical_t,
   174	        "rejected tx must NOT advance L4 logical_t"
   175	    );
   176	
   177	    assert_eq!(l4e.len(), 1, "rejection produces exactly one L4.E record");
   178	    let r = &l4e.records()[0];
   179	    assert_eq!(r.submit_id, 42);
   180	    assert!(r.raw_diagnostic_cid.is_some());
   181	    assert!(l4e.verify_chain().is_ok());
   182	}
   183	
   184	// ────────────────────────────────────────────────────────────────────────────
   185	// (3) P1 kill 3 — ledger reconstructable
   186	// ────────────────────────────────────────────────────────────────────────────
   187	
   188	#[test]
   189	fn test_p1_kill_3_ledger_reconstructable() {
   190	    let mut l = AcceptedLedger::new();
   191	    for i in 1..=4 {
   192	        l.append_accepted(&fixture_work_tx(i)).unwrap();
   193	    }
   194	    let pre_drop_root = l.current_state_root();
   195	
   196	    let tmp = tempfile::NamedTempFile::new().unwrap();
   197	    l.persist(tmp.path()).unwrap();
   198	
   199	    drop(l);
   200	
   201	    let (l_reborn, reconstructed_root) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
   202	    assert_eq!(
   203	        reconstructed_root, pre_drop_root,
   204	        "reconstructed state_root must be bit-equal to pre-drop state_root"
   205	    );
   206	    assert_eq!(l_reborn.len(), 4);
   207	    assert!(l_reborn.verify_chain(0, 4).is_ok());
   208	}
   209	
   210	// ────────────────────────────────────────────────────────────────────────────
   211	// (4) P1 kill 4 — rejected log is isolated from agent-facing read view
   212	// ────────────────────────────────────────────────────────────────────────────
   213	
   214	#[test]
   215	fn test_p1_kill_4_rejected_log_isolated() {
   216	    let mut l4e = RejectionEvidenceWriter::new();
   217	    l4e.append_rejected(
   218	        7,
   219	        Hash::ZERO,
   220	        agent("alice"),
   221	        TxKind::Work,
   222	        cid(0x10),
   223	        RejectionClass::PredicateFailed,
   224	        Some(cid(0xBE)),
   225	        Some("predicate acceptance failed for acc-7".into()),
   226	    );
   227	
   228	    let view: Vec<PublicRejectionView> = l4e.public_view();
   229	    assert_eq!(view.len(), 1);
   230	
   231	    let json = serde_json::to_value(&view[0]).unwrap();
   232	    let obj = json.as_object().expect("PublicRejectionView serializes as object");
   233	    assert!(
   234	        !obj.contains_key("raw_diagnostic_cid"),
   235	        "raw_diagnostic_cid must NOT appear in agent-facing public view"
   236	    );
   237	    assert_eq!(
   238	        obj.get("public_summary").and_then(|v| v.as_str()),
   239	        Some("predicate acceptance failed for acc-7")
   240	    );
   241	
   242	    assert!(
   243	        l4e.records()[0].raw_diagnostic_cid.is_some(),
   244	        "L4.E forensic record must retain raw_diagnostic_cid (shielding is structural, not destructive)"
   245	    );
   246	}
   247	
   248	// ────────────────────────────────────────────────────────────────────────────
   249	// (5) P1 Exit 7 — L4 hash chain breaks on row deletion
   250	// ────────────────────────────────────────────────────────────────────────────
   251	
   252	#[test]
   253	fn test_p1_exit_7_l4_chain_breaks_on_row_deletion() {
   254	    let mut l = AcceptedLedger::new();
   255	    for i in 1..=5 {
   256	        l.append_accepted(&fixture_work_tx(i)).unwrap();
   257	    }
   258	    assert!(l.verify_chain(0, 5).is_ok());
   259	
   260	    l.tamper_remove_entry(2);
   261	
   262	    let r = l.verify_chain(0, 4);
   263	    match r {
   264	        Err(LedgerError::LogicalTGap { at_index: 2, .. })
   265	        | Err(LedgerError::HashMismatch { at_index: 2 }) => {}
   266	        other => panic!(
   267	            "deleting an L4 row must break the chain at index 2; got {:?}",
   268	            other
   269	        ),
   270	    }
   271	}
   272	
   273	// ────────────────────────────────────────────────────────────────────────────
   274	// (6) P1 kill 4b — L4.E hash chain breaks on row deletion
   275	// ────────────────────────────────────────────────────────────────────────────
   276	
   277	#[test]
   278	fn test_p1_kill_4b_rejection_chain_breaks_on_row_deletion() {
   279	    let mut l4e = RejectionEvidenceWriter::new();
   280	    for i in 1..=3u64 {
   281	        l4e.append_rejected(
   282	            i,
   283	            Hash::ZERO,
   284	            agent("alice"),
   285	            TxKind::Work,
   286	            cid(0x10),
   287	            RejectionClass::PredicateFailed,
   288	            None,
   289	            None,
   290	        );
   291	    }
   292	    assert!(l4e.verify_chain().is_ok());
   293	
   294	    l4e.tamper_remove_record(1);
   295	    let r = l4e.verify_chain();
   296	    assert!(
   297	        matches!(r, Err(RejectionEvidenceError::HashMismatch { at: 1 })),
   298	        "deleting row 1 must surface as HashMismatch at the new index 1; got {:?}",
   299	        r
   300	    );
   301	}
   302	
   303	// ────────────────────────────────────────────────────────────────────────────
   304	// (7) P3 RSP-0 Exit 1 — on_init total invariant across N tx sequence
   305	// ────────────────────────────────────────────────────────────────────────────
   306	
   307	#[test]
   308	fn test_p3_rsp0_exit_1_on_init_total_invariant() {
   309	    // After on_init mint, total_coin must be invariant across an arbitrary
   310	    // sequence of redistribution txs (no further mints, no burns). We model
   311	    // a 5-step alice/bob/escrow shuffle and assert assert_total_ctf_conserved
   312	    // succeeds at each step with an empty exempt list.
   313	    let mut s = EconomicState::default();
   314	    s.balances_t.0.insert(agent("alice"), coin(100));
   315	    let baseline = s.clone();
   316	
   317	    // Step 1: alice → bob 30
   318	    let mut s1 = EconomicState::default();
   319	    s1.balances_t.0.insert(agent("alice"), coin(70));
   320	    s1.balances_t.0.insert(agent("bob"), coin(30));
   321	    assert_eq!(assert_total_ctf_conserved(&baseline, &s1, &[]), Ok(()));
   322	
   323	    // Step 2: bob 30 → escrow
   324	    use turingosv4::state::q_state::EscrowEntry;
   325	    let mut s2 = EconomicState::default();
   326	    s2.balances_t.0.insert(agent("alice"), coin(70));
   327	    s2.escrows_t.0.insert(
   328	        TxId("e-1".into()),
   329	        EscrowEntry { amount: coin(30), depositor: agent("bob") },
   330	    );
   331	    assert_eq!(assert_total_ctf_conserved(&s1, &s2, &[]), Ok(()));
   332	
   333	    // Step 3: escrow back to bob
   334	    let mut s3 = EconomicState::default();
   335	    s3.balances_t.0.insert(agent("alice"), coin(70));
   336	    s3.balances_t.0.insert(agent("bob"), coin(30));
   337	    assert_eq!(assert_total_ctf_conserved(&s2, &s3, &[]), Ok(()));
   338	
   339	    // Step 4: alice 70 → carol
   340	    let mut s4 = EconomicState::default();
   341	    s4.balances_t.0.insert(agent("carol"), coin(70));
   342	    s4.balances_t.0.insert(agent("bob"), coin(30));
   343	    assert_eq!(assert_total_ctf_conserved(&s3, &s4, &[]), Ok(()));
   344	
   345	    // Step 5: full round-trip back to baseline
   346	    let mut s5 = EconomicState::default();
   347	    s5.balances_t.0.insert(agent("alice"), coin(100));
   348	    assert_eq!(assert_total_ctf_conserved(&s4, &s5, &[]), Ok(()));
   349	
   350	    // Final cross-check: end == start (closed-system loop).
   351	    assert_eq!(
   352	        s5.balances_t.0.get(&agent("alice")),
   353	        baseline.balances_t.0.get(&agent("alice")),
   354	        "round-trip must restore baseline"
   355	    );
   356	}
   357	
   358	// ────────────────────────────────────────────────────────────────────────────
   359	// (8) P3 RSP-0 Exit 2 — read-is-free (rtool / search / think MUST have fee=0)
   360	// ────────────────────────────────────────────────────────────────────────────
   361	
   362	#[test]
   363	fn test_p3_rsp0_exit_2_read_is_free() {
   364	    // K5 has no dedicated read-tx variants today; the structural guard runs
   365	    // against ALL TxKinds. assert_read_is_free(kind, fee=0) must succeed for
   366	    // every variant; non-zero fee on ANY variant must surface as ReadCharged.
   367	    for kind in [
   368	        TxKind::Work,
   369	        TxKind::Verify,
   370	        TxKind::Challenge,
   371	        TxKind::Reuse,
   372	        TxKind::FinalizeReward,
   373	        TxKind::TaskExpire,
   374	        TxKind::TerminalSummary,
   375	    ] {
   376	        assert_eq!(
   377	            assert_read_is_free(kind, 0),
   378	            Ok(()),
   379	            "fee=0 must pass for TxKind={:?}",
   380	            kind
   381	        );
   382	    }
   383	
   384	    // Anti-Goodhart: any non-zero fee at any kind is structurally rejected.
   385	    assert_eq!(
   386	        assert_read_is_free(TxKind::Reuse, 1),
   387	        Err(MonetaryError::ReadCharged {
   388	            tx_kind: TxKind::Reuse,
   389	            fee: 1
   390	        })
   391	    );
   392	    assert_eq!(
   393	        assert_read_is_free(TxKind::Work, 9999),
   394	        Err(MonetaryError::ReadCharged {
   395	            tx_kind: TxKind::Work,
   396	            fee: 9999
   397	        })
   398	    );
   399	}
   400	
   401	// ────────────────────────────────────────────────────────────────────────────
   402	// (9) P3 kill 1 — no post-init mint (rejected route MUST go to L4.E, not L4)
   403	// ────────────────────────────────────────────────────────────────────────────
   404	
   405	#[test]
   406	fn test_p3_kill_1_no_post_init_mint() {
   407	    // The numeric kill: any post-init mint surfaces as MonetaryError::PostInitMint.
   408	    let before = EconomicState::default();
   409	    let mut after = EconomicState::default();
   410	    after.balances_t.0.insert(agent("alice"), coin(100));
   411	    let r = assert_total_ctf_conserved(&before, &after, &[]);
   412	    assert_eq!(
   413	        r,
   414	        Err(MonetaryError::PostInitMint {
   415	            delta_micro: 100 * MICRO_PER_COIN
   416	        }),
   417	        "any non-exempt supply increase must surface as PostInitMint"
   418	    );
   419	
   420	    // The structural kill: L4.E (NOT L4) is the home for the rejection record.
   421	    // Simulate the dispatch_transition rejection path by writing the rejection
   422	    // ONLY to L4.E and asserting L4 is untouched.
   423	    let l4 = AcceptedLedger::new();
   424	    let pre_root = l4.current_state_root();
   425	    let pre_logical_t = l4.len();
   426	
   427	    let mut l4e = RejectionEvidenceWriter::new();
   428	    l4e.append_rejected(
   429	        99,
   430	        pre_root,
   431	        agent("alice"),
   432	        TxKind::Work,
   433	        cid(0x30),
   434	        RejectionClass::InvariantViolation,
   435	        Some(cid(0xC0)),
   436	        Some("PostInitMint: delta_micro=100000000".into()),
   437	    );
   438	
   439	    // L4 untouched; L4.E has the record.
   440	    assert_eq!(l4.current_state_root(), pre_root);
   441	    assert_eq!(l4.len(), pre_logical_t);
   442	    assert_eq!(l4e.len(), 1);
   443	    assert!(matches!(
   444	        l4e.records()[0].rejection_class,
   445	        RejectionClass::InvariantViolation
   446	    ));
   447	}
   448	
   449	// ════════════════════════════════════════════════════════════════════════════
   450	// Tier-B — NON-BLOCKING (artifacts; do not gate ship)
   451	// ════════════════════════════════════════════════════════════════════════════
   452	
   453	// (10) AT-1 P6 anchor — evaluator solves mathd_algebra_107 in n3 mode.
   454	//
   455	// Verified out-of-band by the Day-4 live runs documented in commit 50a1d67:
   456	// RUN 1 + RUN 2 both produced solved=true with gp_payload=nlinarith. Capturing
   457	// this here as an #[ignore] live integration test so the assertion is REGISTERED
   458	// in the harness even though it requires a running LLM proxy + DEEPSEEK_API_KEY
   459	// to execute. Run manually with:
   460	//   cargo test test_at1_evaluator_solves_mathd_algebra_107_n3 -- --ignored

exec
/bin/bash -lc "nl -ba tests/tb_1_acceptance.rs | sed -n '460,530p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   460	//   cargo test test_at1_evaluator_solves_mathd_algebra_107_n3 -- --ignored
   461	#[test]
   462	#[ignore = "Tier-B P6 anchor: requires live LLM proxy + DEEPSEEK_API_KEY; verified Day-4 (commit 50a1d67)"]
   463	fn test_at1_evaluator_solves_mathd_algebra_107_n3() {
   464	    // Live form would shell out to target/release/evaluator with CONDITION=n3
   465	    // ACTIVE_MODEL=deepseek-chat MAX_TRANSACTIONS=10 and assert solved=true on
   466	    // the JSONL row. Body intentionally empty — TB-1 ship gate is the manually-
   467	    // observable Day-4 evidence in /tmp/tb1_day4_smoke_v2/run{1,2}.jsonl.
   468	}
   469	
   470	// (11) AT-2 — each tx in evaluator run produces an L4 LedgerEntry.
   471	// Non-blocking until WorkTx dispatch_transition body lands at TB-2 RSP-1.
   472	#[test]
   473	#[ignore = "Tier-B: WorkTx dispatch_transition body lands TB-2 RSP-1; evaluator currently uses legacy emit path"]
   474	fn test_at2_l4_entry_per_dispatched_tx() {
   475	    // When TB-2 RSP-1 wires the WorkTx → AcceptedLedger::append_accepted path,
   476	    // un-ignore this test and assert: for every successful evaluator tx, exactly
   477	    // one L4 entry is appended; the entry's tx_payload_hash equals the tx's
   478	    // canonical hash; verify_chain(0, n) succeeds at the end of the run.
   479	}
   480	
   481	// (12) AT-3 — h_vppu non-null on a 2nd-run row.
   482	//
   483	// The live form (2 evaluator invocations producing JSONL rows) is verified by
   484	// the Day-4 evidence at /tmp/tb1_day4_smoke_v2/run2.jsonl (commit 50a1d67):
   485	// run 2 carried `h_vppu=6.215891726697228`. The unit-level CONTRACT (capacity-3
   486	// rolling history; record-then-query semantics; persistence round-trip) is
   487	// covered by the 9 unit tests inside `minif2f_v4::h_vppu_history` itself
   488	// (cargo test -p minif2f_v4 --lib h_vppu_history → 9/9 PASS).
   489	//
   490	// This file is a top-level integration test for the `turingosv4` crate and
   491	// cannot import the `minif2f_v4` experiments crate (asymmetric path dep).
   492	// Registering the AT-3 contract here as a `#[ignore]` documentation stub so
   493	// the harness lists it explicitly; un-ignore + relocate to
   494	// `experiments/minif2f_v4/tests/` if/when minif2f_v4 grows an integration
   495	// test directory.
   496	#[test]
   497	#[ignore = "Tier-B AT-3: covered by minif2f_v4 lib tests + Day-4 live evidence (commit 50a1d67); cannot import minif2f_v4 from turingosv4 integration tests"]
   498	fn test_at3_h_vppu_non_null_on_second_run() {
   499	    // No body — see ignore reason above.
   500	}
   501	
   502	// (13) AT-4 — PputResult.econ_balance_delta non-zero.
   503	// Non-blocking until TB-2 RSP-1's escrow_lock_tx + yes_stake_tx fire. RSP-0
   504	// (Day-2) only proves the conservation invariant + scaffolds escrow/balances
   505	// structures; actual non-zero deltas need the RSP-1 wiring.
   506	#[test]
   507	#[ignore = "Tier-B: needs RSP-1 escrow_lock_tx + yes_stake_tx wiring (TB-2)"]
   508	fn test_at4_econ_balance_delta_non_zero() {
   509	    // When TB-2 RSP-1 lands, un-ignore and assert: for an evaluator run that
   510	    // exercises an escrow_lock_tx, PputResult.econ_balance_delta is Some(non-zero).
   511	    // RSP-0 today only ships the conservation invariant + escrow_vault scaffolding.
   512	}

exec
/bin/bash -lc "nl -ba src/economy/monetary_invariant.rs | sed -n '80,210p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    80	/// Sum of every coin-holding sub-index in `EconomicState`, in micro-units.
    81	///
    82	/// Counted (each contributes its `MicroCoin` directly):
    83	/// - `balances_t` (agent-held)
    84	/// - `escrows_t` (locked under task)
    85	/// - `stakes_t` (locked under tx)
    86	/// - `claims_t` (pending payout)
    87	/// - `task_markets_t.bounty` (sponsor-locked under task)
    88	/// - `challenge_cases_t.bond` (challenger-locked under case)
    89	///
    90	/// NOT counted (not a holding):
    91	/// - `reputations_t` (signed reputation, not coin)
    92	/// - `royalty_graph_t` (edges, no coin)
    93	/// - `price_index_t` (market data, not held)
    94	fn total_supply_micro(s: &EconomicState) -> Result<i64, MonetaryError> {
    95	    let mut total: i64 = 0;
    96	    for v in s.balances_t.0.values() {
    97	        total = total.checked_add(v.micro_units()).ok_or(MonetaryError::Overflow)?;
    98	    }
    99	    for e in s.escrows_t.0.values() {
   100	        total = total.checked_add(e.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
   101	    }
   102	    for e in s.stakes_t.0.values() {
   103	        total = total.checked_add(e.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
   104	    }
   105	    for c in s.claims_t.0.values() {
   106	        total = total.checked_add(c.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
   107	    }
   108	    for m in s.task_markets_t.0.values() {
   109	        total = total.checked_add(m.bounty.micro_units()).ok_or(MonetaryError::Overflow)?;
   110	    }
   111	    for c in s.challenge_cases_t.0.values() {
   112	        total = total.checked_add(c.bond.micro_units()).ok_or(MonetaryError::Overflow)?;
   113	    }
   114	    Ok(total)
   115	}
   116	
   117	// ────────────────────────────────────────────────────────────────────────────
   118	// assert_no_post_init_mint — structural guard at the tx layer
   119	// ────────────────────────────────────────────────────────────────────────────
   120	
   121	/// TRACE_MATRIX P3:1 (kill 1) — structural guard against post-genesis mint
   122	/// at the `TypedTx` layer.
   123	///
   124	/// **Today, K5 has no `Mint` variant** — none of the 7 `TypedTx` variants
   125	/// directly inject coins. Genesis allocation happens in `on_init` outside
   126	/// the K5 transition surface. Therefore, on a non-genesis `q`, this fn
   127	/// returns `Ok(())` for every well-formed `TypedTx`.
   128	///
   129	/// **Why the function exists anyway**: it is a forward-compat barrier.
   130	/// If a future RSP atom adds a `Mint` (or `SystemReward`-class) variant,
   131	/// it MUST be added to the match below AND rejected here when
   132	/// `q.state_root_t != Hash::ZERO`. Numeric conservation is enforced by
   133	/// [`assert_total_ctf_conserved`] separately.
   134	pub fn assert_no_post_init_mint(tx: &TypedTx, q: &QState) -> Result<(), MonetaryError> {
   135	    let is_post_init = q.state_root_t != Hash::ZERO;
   136	    if !is_post_init {
   137	        return Ok(());
   138	    }
   139	    match tx {
   140	        TypedTx::Work(_)
   141	        | TypedTx::Verify(_)
   142	        | TypedTx::Challenge(_)
   143	        | TypedTx::Reuse(_)
   144	        | TypedTx::FinalizeReward(_)
   145	        | TypedTx::TaskExpire(_)
   146	        | TypedTx::TerminalSummary(_) => Ok(()),
   147	    }
   148	}
   149	
   150	// ────────────────────────────────────────────────────────────────────────────
   151	// assert_total_ctf_conserved — numeric conservation across a transition
   152	// ────────────────────────────────────────────────────────────────────────────
   153	
   154	/// TRACE_MATRIX 基本法 1 + P3:1 — conservation of total CTF across a
   155	/// transition `before → after`.
   156	///
   157	/// Mints (`delta > 0`) and burns (`delta < 0`) are both rejected unless
   158	/// `exempt_tx_kinds` is non-empty. The exempt list is the explicit opt-out
   159	/// for legitimate supply-changing operations (e.g., genesis init,
   160	/// system-emitted rewards in a future RSP); RSP-0 never populates it
   161	/// at runtime.
   162	///
   163	/// Caller convention: pass `&[]` for normal agent-submitted transitions.
   164	/// Pass `&[TxKind::FinalizeReward]` (etc.) only when a system-emitted
   165	/// supply-changing tx is being processed AND the RSP semantics for that
   166	/// kind have been ratified. RSP-0 does not ratify any.
   167	pub fn assert_total_ctf_conserved(
   168	    before: &EconomicState,
   169	    after: &EconomicState,
   170	    exempt_tx_kinds: &[TxKind],
   171	) -> Result<(), MonetaryError> {
   172	    let total_before = total_supply_micro(before)?;
   173	    let total_after = total_supply_micro(after)?;
   174	    let delta = total_after
   175	        .checked_sub(total_before)
   176	        .ok_or(MonetaryError::Overflow)?;
   177	    if !exempt_tx_kinds.is_empty() {
   178	        return Ok(());
   179	    }
   180	    if delta > 0 {
   181	        return Err(MonetaryError::PostInitMint { delta_micro: delta });
   182	    }
   183	    if delta < 0 {
   184	        return Err(MonetaryError::TotalCtfBurn { delta_micro: delta });
   185	    }
   186	    Ok(())
   187	}
   188	
   189	// ────────────────────────────────────────────────────────────────────────────
   190	// assert_read_is_free — tx-level no-fee guard
   191	// ────────────────────────────────────────────────────────────────────────────
   192	
   193	/// TRACE_MATRIX P3:2 — assert that no K5 `TxKind` carries a per-tx fee.
   194	///
   195	/// K5 spec: every `TypedTx` variant has stake / bond fields (locked but
   196	/// not consumed) but NO fee field. A non-zero `fee` is a structural bug
   197	/// in whichever caller computed it; this fn is the barrier.
   198	///
   199	/// Note: tool-level read-is-free for `rtool` / `search` / `think` is
   200	/// enforced at the SDK boundary in a later RSP atom (out of scope for
   201	/// RSP-0). This fn covers the tx-level invariant only.
   202	pub fn assert_read_is_free(tx_kind: TxKind, fee: u64) -> Result<(), MonetaryError> {
   203	    if fee != 0 {
   204	        return Err(MonetaryError::ReadCharged { tx_kind, fee });
   205	    }
   206	    Ok(())
   207	}
   208	
   209	// ────────────────────────────────────────────────────────────────────────────
   210	// Tests

exec
/bin/bash -lc "nl -ba src/economy/monetary_invariant.rs | sed -n '330,430p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   330	
   331	    #[test]
   332	    fn ctf_conserved_across_subindexes() {
   333	        // 100 in balances → 60 in balances + 40 in escrow; total unchanged.
   334	        let mut before = EconomicState::default();
   335	        before.balances_t.0.insert(agent("alice"), coin(100));
   336	        let mut after = EconomicState::default();
   337	        after.balances_t.0.insert(agent("alice"), coin(60));
   338	        after.escrows_t.0.insert(
   339	            tx("work-1"),
   340	            EscrowEntry { amount: coin(40), depositor: agent("alice") },
   341	        );
   342	        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
   343	    }
   344	
   345	    #[test]
   346	    fn ctf_conserved_across_n10_random_sequence() {
   347	        // Charter Day-2 unit: "total CTF conserved across N=10 random tx sequences".
   348	        // We model 10 deterministic-but-varied conservative redistributions
   349	        // (Alice/Bob/Carol; balances ↔ escrow ↔ stake ↔ claim ↔ market ↔ challenge).
   350	        // Each step is a closed transfer; total supply is invariant.
   351	        let mut s = EconomicState::default();
   352	        s.balances_t.0.insert(agent("alice"), coin(100));
   353	        s.balances_t.0.insert(agent("bob"), coin(50));
   354	        s.balances_t.0.insert(agent("carol"), coin(25));
   355	        let total0 = total_supply_micro(&s).unwrap();
   356	
   357	        let steps: [(&str, i64); 10] = [
   358	            ("alice->bob", 5),
   359	            ("bob->escrow:t1", 10),
   360	            ("alice->stake:tx1", 7),
   361	            ("escrow:t1->claim:tx1", 3),
   362	            ("alice->market:t2", 20),
   363	            ("market:t2->balance:carol", 15),
   364	            ("stake:tx1->challenge:case1", 4),
   365	            ("challenge:case1->balance:bob", 2),
   366	            ("claim:tx1->balance:alice", 3),
   367	            ("balance:carol->escrow:t3", 6),
   368	        ];
   369	
   370	        let total_each = vec![total0; 10];
   371	        for (i, (label, _amt)) in steps.iter().enumerate() {
   372	            // Apply a small redistribution: move `_amt` micro_per_coin
   373	            // between two slots. We just re-shuffle existing supply.
   374	            // (Concrete redistribution mechanics live in SettlementEngine;
   375	            // the invariant under test is: any closed redistribution leaves
   376	            // total_supply_micro unchanged.)
   377	            let amt_micro = (i as i64 + 1) * 1_000; // small, deterministic
   378	            // Move `amt_micro` from alice's balance into a synthetic stake.
   379	            let alice_bal = s.balances_t.0.get(&agent("alice"))
   380	                .copied().unwrap_or(MicroCoin::zero());
   381	            if alice_bal.micro_units() >= amt_micro {
   382	                s.balances_t.0.insert(
   383	                    agent("alice"),
   384	                    MicroCoin::from_micro_units(alice_bal.micro_units() - amt_micro),
   385	                );
   386	                let key = tx(&format!("stake-step-{}", i));
   387	                s.stakes_t.0.insert(
   388	                    key,
   389	                    StakeEntry { amount: MicroCoin::from_micro_units(amt_micro), staker: agent("alice") },
   390	                );
   391	            }
   392	            let total_now = total_supply_micro(&s).unwrap();
   393	            assert_eq!(
   394	                total_now, total_each[i],
   395	                "step {} ({}): conservation broke",
   396	                i, label
   397	            );
   398	        }
   399	        // Final cross-check.
   400	        assert_eq!(total_supply_micro(&s).unwrap(), total0);
   401	    }
   402	
   403	    #[test]
   404	    fn ctf_counts_all_six_holding_subindexes() {
   405	        // Make sure we sum balances + escrows + stakes + claims + bounty + bond.
   406	        let mut s = EconomicState::default();
   407	        s.balances_t.0.insert(agent("a"), coin(1));
   408	        s.escrows_t.0.insert(
   409	            tx("e"),
   410	            EscrowEntry { amount: coin(2), depositor: agent("a") },
   411	        );
   412	        s.stakes_t.0.insert(
   413	            tx("s"),
   414	            StakeEntry { amount: coin(4), staker: agent("a") },
   415	        );
   416	        s.claims_t.0.insert(
   417	            tx("c"),
   418	            ClaimEntry { amount: coin(8), claimant: agent("a") },
   419	        );
   420	        s.task_markets_t.0.insert(
   421	            tx("m"),
   422	            TaskMarketEntry {
   423	                publisher: agent("a"),
   424	                bounty: coin(16),
   425	                ..Default::default()
   426	            },
   427	        );
   428	        // challenge_cases_t bond
   429	        let mut cc = crate::state::q_state::ChallengeCase::default();
   430	        cc.bond = coin(32);

exec
/bin/bash -lc "nl -ba src/economy/monetary_invariant.rs | sed -n '403,470p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   403	    #[test]
   404	    fn ctf_counts_all_six_holding_subindexes() {
   405	        // Make sure we sum balances + escrows + stakes + claims + bounty + bond.
   406	        let mut s = EconomicState::default();
   407	        s.balances_t.0.insert(agent("a"), coin(1));
   408	        s.escrows_t.0.insert(
   409	            tx("e"),
   410	            EscrowEntry { amount: coin(2), depositor: agent("a") },
   411	        );
   412	        s.stakes_t.0.insert(
   413	            tx("s"),
   414	            StakeEntry { amount: coin(4), staker: agent("a") },
   415	        );
   416	        s.claims_t.0.insert(
   417	            tx("c"),
   418	            ClaimEntry { amount: coin(8), claimant: agent("a") },
   419	        );
   420	        s.task_markets_t.0.insert(
   421	            tx("m"),
   422	            TaskMarketEntry {
   423	                publisher: agent("a"),
   424	                bounty: coin(16),
   425	                ..Default::default()
   426	            },
   427	        );
   428	        // challenge_cases_t bond
   429	        let mut cc = crate::state::q_state::ChallengeCase::default();
   430	        cc.bond = coin(32);
   431	        cc.challenger = agent("a");
   432	        s.challenge_cases_t.0.insert(tx("ch"), cc);
   433	
   434	        // Each power of two distinct => sum = 63 base coin = 63_000_000 micro.
   435	        assert_eq!(total_supply_micro(&s).unwrap(), 63 * MICRO_PER_COIN);
   436	    }
   437	
   438	    // ── assert_read_is_free ─────────────────────────────────────────────────
   439	
   440	    #[test]
   441	    fn read_is_free_zero_fee_passes_for_all_kinds() {
   442	        for k in [
   443	            TxKind::Work,
   444	            TxKind::Verify,
   445	            TxKind::Challenge,
   446	            TxKind::Reuse,
   447	            TxKind::FinalizeReward,
   448	            TxKind::TaskExpire,
   449	            TxKind::TerminalSummary,
   450	        ] {
   451	            assert_eq!(assert_read_is_free(k, 0), Ok(()));
   452	        }
   453	    }
   454	
   455	    #[test]
   456	    fn read_is_free_nonzero_fee_rejected() {
   457	        // P3:2 — any per-tx fee on a K5 TxKind is a structural bug.
   458	        let r = assert_read_is_free(TxKind::Reuse, 1);
   459	        assert_eq!(r, Err(MonetaryError::ReadCharged { tx_kind: TxKind::Reuse, fee: 1 }));
   460	        let r = assert_read_is_free(TxKind::Work, 9999);
   461	        assert_eq!(r, Err(MonetaryError::ReadCharged { tx_kind: TxKind::Work, fee: 9999 }));
   462	    }
   463	}

exec
/bin/bash -lc "rg -n \"load → query|h_vppu_history|result\\.h_vppu|record_before_query|test_record_before_query\" -S experiments/minif2f_v4/src" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/lib.rs:16:pub mod h_vppu_history;
experiments/minif2f_v4/src/bin/evaluator.rs:365:    // load → query (excluding current) → stamp → record → save, so
experiments/minif2f_v4/src/bin/evaluator.rs:369:    // Storage: $EXPERIMENT_DIR/h_vppu_history.json (or cwd if unset).
experiments/minif2f_v4/src/bin/evaluator.rs:376:    .join("h_vppu_history.json");
experiments/minif2f_v4/src/bin/evaluator.rs:377:    let mut h_vppu_history =
experiments/minif2f_v4/src/bin/evaluator.rs:378:        minif2f_v4::h_vppu_history::HVppuHistory::load_from(&h_vppu_path);
experiments/minif2f_v4/src/bin/evaluator.rs:379:    result.h_vppu = h_vppu_history.h_vppu_for(&result.problem_id, result.pput_verified);
experiments/minif2f_v4/src/bin/evaluator.rs:380:    h_vppu_history.record(&result.problem_id, result.pput_verified);
experiments/minif2f_v4/src/bin/evaluator.rs:381:    if let Err(e) = h_vppu_history.save_to(&h_vppu_path) {
experiments/minif2f_v4/src/bin/evaluator.rs:383:            "[h_vppu_history] save to {:?} failed: {}; next run will start without prior history",
experiments/minif2f_v4/src/h_vppu_history.rs:40:/// emitted on `PputResult.h_vppu`. Not a constitutional flowchart node;
experiments/minif2f_v4/src/h_vppu_history.rs:66:                    "[h_vppu_history] corrupt store at {:?} ({}); starting fresh",
experiments/minif2f_v4/src/h_vppu_history.rs:160:            "h_vppu_history_{}_{}_{}.json",
experiments/minif2f_v4/src/h_vppu_history.rs:267:    fn test_record_before_query_does_not_self_reference() {
experiments/minif2f_v4/src/h_vppu_history.rs:270:        // load → query → record → save (per evaluator wire site).

exec
/bin/bash -lc 'rg --files experiments/minif2f_v4/src | sort' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/agent_models.rs
experiments/minif2f_v4/src/bin/evaluator.rs
experiments/minif2f_v4/src/budget_regime.rs
experiments/minif2f_v4/src/cost_aggregator.rs
experiments/minif2f_v4/src/experiment_mode.rs
experiments/minif2f_v4/src/fc_trace.rs
experiments/minif2f_v4/src/h_vppu_history.rs
experiments/minif2f_v4/src/jsonl_schema.rs
experiments/minif2f_v4/src/lean4_oracle.rs
experiments/minif2f_v4/src/lib.rs
experiments/minif2f_v4/src/post_hoc_verifier.rs
experiments/minif2f_v4/src/rollback_sim.rs
experiments/minif2f_v4/src/run_id.rs
experiments/minif2f_v4/src/wall_clock.rs

exec
/bin/bash -lc 'rg -n "h_vppu" -S Cargo.toml experiments/minif2f_v4/Cargo.toml tests src experiments | head -n 120' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/lib.rs:16:pub mod h_vppu_history;
experiments/minif2f_v4/src/h_vppu_history.rs:5:// (Day 4 — P6 instrumentation: h_vppu computation).
experiments/minif2f_v4/src/h_vppu_history.rs:8://   h_vppu = current_pput_verified / mean(history N=1..3)
experiments/minif2f_v4/src/h_vppu_history.rs:23:// FC-trace: FC1-N11 (∏p decision diversity) — h_vppu measures
experiments/minif2f_v4/src/h_vppu_history.rs:25:// learning produce h_vppu = current/mean ≈ 1.0 or below. Step-4
experiments/minif2f_v4/src/h_vppu_history.rs:26:// Capability Compilation should drive h_vppu > 1 on heldout.
experiments/minif2f_v4/src/h_vppu_history.rs:40:/// emitted on `PputResult.h_vppu`. Not a constitutional flowchart node;
experiments/minif2f_v4/src/h_vppu_history.rs:66:                    "[h_vppu_history] corrupt store at {:?} ({}); starting fresh",
experiments/minif2f_v4/src/h_vppu_history.rs:108:    /// H-VPPUT definition): compute h_vppu = current / mean(history) when
experiments/minif2f_v4/src/h_vppu_history.rs:110:    /// current run's value is NOT included in the mean — h_vppu measures
experiments/minif2f_v4/src/h_vppu_history.rs:118:    pub fn h_vppu_for(&self, problem_id: &str, current_pput_verified: f64) -> Option<f64> {
experiments/minif2f_v4/src/h_vppu_history.rs:160:            "h_vppu_history_{}_{}_{}.json",
experiments/minif2f_v4/src/h_vppu_history.rs:169:            history.h_vppu_for("mathd_algebra_107", 0.5),
experiments/minif2f_v4/src/h_vppu_history.rs:179:        // Current run pput_verified = 0.6 → h_vppu = 0.6/0.4 = 1.5
experiments/minif2f_v4/src/h_vppu_history.rs:181:            .h_vppu_for("mathd_algebra_107", 0.6)
experiments/minif2f_v4/src/h_vppu_history.rs:185:            "h_vppu = current/mean = 0.6/0.4 = 1.5, got {}",
experiments/minif2f_v4/src/h_vppu_history.rs:202:        let h = history.h_vppu_for("p1", 0.4).unwrap();
experiments/minif2f_v4/src/h_vppu_history.rs:216:        assert_eq!(history.h_vppu_for("p1", 0.5), None);
experiments/minif2f_v4/src/h_vppu_history.rs:225:        let h_p1 = history.h_vppu_for("p1", 0.4).unwrap(); // 0.4 / 0.2 = 2.0
experiments/minif2f_v4/src/h_vppu_history.rs:226:        let h_p2 = history.h_vppu_for("p2", 0.8).unwrap(); // 0.8 / 0.8 = 1.0
experiments/minif2f_v4/src/h_vppu_history.rs:240:        assert!((h2.h_vppu_for("p1", 0.6).unwrap() - 1.5).abs() < 1e-12);
experiments/minif2f_v4/src/h_vppu_history.rs:241:        assert!((h2.h_vppu_for("p2", 1.4).unwrap() - 2.0).abs() < 1e-12);
experiments/minif2f_v4/src/h_vppu_history.rs:250:        assert_eq!(h.h_vppu_for("any", 1.0), None);
experiments/minif2f_v4/src/h_vppu_history.rs:259:            h.h_vppu_for("any", 1.0),
experiments/minif2f_v4/src/h_vppu_history.rs:268:        // Verify h_vppu_for does NOT include the in-flight value when
experiments/minif2f_v4/src/h_vppu_history.rs:273:        let h_before_record = history.h_vppu_for("p1", 0.6).unwrap();
experiments/minif2f_v4/src/h_vppu_history.rs:275:        let h_after_record = history.h_vppu_for("p1", 0.6).unwrap();
experiments/minif2f_v4/src/bin/evaluator.rs:230:    h_vppu: Option<f64>,
experiments/minif2f_v4/src/bin/evaluator.rs:362:    // TB-1 Day-4 (2026-04-29): stamp h_vppu by querying the persisted
experiments/minif2f_v4/src/bin/evaluator.rs:369:    // Storage: $EXPERIMENT_DIR/h_vppu_history.json (or cwd if unset).
experiments/minif2f_v4/src/bin/evaluator.rs:370:    // Failure to load/save degrades quietly — h_vppu is a P6 non-
experiments/minif2f_v4/src/bin/evaluator.rs:373:    let h_vppu_path = std::path::PathBuf::from(
experiments/minif2f_v4/src/bin/evaluator.rs:376:    .join("h_vppu_history.json");
experiments/minif2f_v4/src/bin/evaluator.rs:377:    let mut h_vppu_history =
experiments/minif2f_v4/src/bin/evaluator.rs:378:        minif2f_v4::h_vppu_history::HVppuHistory::load_from(&h_vppu_path);
experiments/minif2f_v4/src/bin/evaluator.rs:379:    result.h_vppu = h_vppu_history.h_vppu_for(&result.problem_id, result.pput_verified);
experiments/minif2f_v4/src/bin/evaluator.rs:380:    h_vppu_history.record(&result.problem_id, result.pput_verified);
experiments/minif2f_v4/src/bin/evaluator.rs:381:    if let Err(e) = h_vppu_history.save_to(&h_vppu_path) {
experiments/minif2f_v4/src/bin/evaluator.rs:383:            "[h_vppu_history] save to {:?} failed: {}; next run will start without prior history",
experiments/minif2f_v4/src/bin/evaluator.rs:384:            h_vppu_path, e
experiments/minif2f_v4/src/bin/evaluator.rs:1836:        h_vppu: None,
src/sdk/prompt_guard.rs:110:    fn test_h_vpput_caught() {
tests/tb_1_acceptance.rs:24://!  12. test_at3_h_vppu_non_null_on_second_run               (UNIT form; live form
tests/tb_1_acceptance.rs:481:// (12) AT-3 — h_vppu non-null on a 2nd-run row.
tests/tb_1_acceptance.rs:485:// run 2 carried `h_vppu=6.215891726697228`. The unit-level CONTRACT (capacity-3
tests/tb_1_acceptance.rs:487:// covered by the 9 unit tests inside `minif2f_v4::h_vppu_history` itself
tests/tb_1_acceptance.rs:488:// (cargo test -p minif2f_v4 --lib h_vppu_history → 9/9 PASS).
tests/tb_1_acceptance.rs:498:fn test_at3_h_vppu_non_null_on_second_run() {

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '340,390p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   340	                     &lean_path, &proxy_url, &model, n).await
   341	        }
   342	        "hybrid_v1" => {
   343	            // Mid-term audit P0-D fix 2026-04-25: hybrid_v1 was a Paper 1 era
   344	            // condition that ran run_oneshot, then on failure ran run_swarm,
   345	            // and merged via `..r2` field-spread. Codex flagged that the spread
   346	            // dropped the failed oneshot's C_i (failed_branch_count and
   347	            // total_run_token_count from r1 were silently discarded). PPUT-CCL
   348	            // arc does NOT use hybrid_v1 — it operates exclusively on `oneshot`
   349	            // and `n<N>` conditions per PREREG. Disabling here forces any
   350	            // pipeline that ships a stale hybrid_v1 invocation to surface the
   351	            // deprecation immediately rather than emit a corrupt C_i.
   352	            eprintln!("hybrid_v1 condition is deprecated for PPUT-CCL arc and was \
   353	                       disabled in mid-term audit P0-D fix 2026-04-25. The prior \
   354	                       implementation dropped the failed oneshot leg's C_i via a \
   355	                       `..r2` field-spread, corrupting full-run cost accounting. \
   356	                       Use `oneshot` or `n<N>` instead.");
   357	            std::process::exit(1);
   358	        }
   359	        other => { eprintln!("Unknown condition: {}", other); std::process::exit(1); }
   360	    };
   361	
   362	    // TB-1 Day-4 (2026-04-29): stamp h_vppu by querying the persisted
   363	    // per-problem rolling history of pput_verified, then record the
   364	    // current run's pput_verified for future invocations. Order is
   365	    // load → query (excluding current) → stamp → record → save, so
   366	    // the current run does NOT self-reference its own value when
   367	    // computing the ratio.
   368	    //
   369	    // Storage: $EXPERIMENT_DIR/h_vppu_history.json (or cwd if unset).
   370	    // Failure to load/save degrades quietly — h_vppu is a P6 non-
   371	    // blocking metric per recharter Day-5 Tier-B. Saving failure logs
   372	    // a warning but never aborts the run.
   373	    let h_vppu_path = std::path::PathBuf::from(
   374	        std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into()),
   375	    )
   376	    .join("h_vppu_history.json");
   377	    let mut h_vppu_history =
   378	        minif2f_v4::h_vppu_history::HVppuHistory::load_from(&h_vppu_path);
   379	    result.h_vppu = h_vppu_history.h_vppu_for(&result.problem_id, result.pput_verified);
   380	    h_vppu_history.record(&result.problem_id, result.pput_verified);
   381	    if let Err(e) = h_vppu_history.save_to(&h_vppu_path) {
   382	        log::warn!(
   383	            "[h_vppu_history] save to {:?} failed: {}; next run will start without prior history",
   384	            h_vppu_path, e
   385	        );
   386	    }
   387	
   388	    // Output PPUT result as JSON (machine-readable for batch runner)
   389	    let json = serde_json::to_string(&result).unwrap();
   390	    println!("PPUT_RESULT:{}", json);

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/h_vppu_history.rs | sed -n '100,135p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   100	            .or_default();
   101	        entry.push_back(pput_verified);
   102	        while entry.len() > HISTORY_CAPACITY {
   103	            entry.pop_front();
   104	        }
   105	    }
   106	
   107	    /// TRACE_MATRIX orphan (P6 instrumentation; PREREG_PPUT_CCL_2026-04-26.md § 5
   108	    /// H-VPPUT definition): compute h_vppu = current / mean(history) when
   109	    /// there is at least one prior run AND that mean is non-zero. The
   110	    /// current run's value is NOT included in the mean — h_vppu measures
   111	    /// improvement against a held-out baseline, not against itself.
   112	    ///
   113	    /// Returns None when:
   114	    ///   - no history exists for this problem (first run);
   115	    ///   - the prior history mean is 0 (all prior runs failed; ratio
   116	    ///     undefined — None preserves "no signal" semantics rather than
   117	    ///     emitting NaN/inf into the JSONL row).
   118	    pub fn h_vppu_for(&self, problem_id: &str, current_pput_verified: f64) -> Option<f64> {
   119	        let entry = self.by_problem.get(problem_id)?;
   120	        if entry.is_empty() {
   121	            return None;
   122	        }
   123	        let n = entry.len() as f64;
   124	        let sum: f64 = entry.iter().sum();
   125	        let mean = sum / n;
   126	        if mean == 0.0 {
   127	            return None;
   128	        }
   129	        Some(current_pput_verified / mean)
   130	    }
   131	
   132	    /// TRACE_MATRIX orphan (P6 instrumentation; PREREG § 5):
   133	    /// number of prior runs stored for a given problem. Exposed for
   134	    /// tests + downstream auditors that want to assert capacity-3
   135	    /// invariants without round-tripping through JSON.

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/h_vppu_history.rs | sed -n '260,285p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   260	            None,
   261	            "corrupt store must not panic; returns empty"
   262	        );
   263	        let _ = fs::remove_file(&path);
   264	    }
   265	
   266	    #[test]
   267	    fn test_record_before_query_does_not_self_reference() {
   268	        // Verify h_vppu_for does NOT include the in-flight value when
   269	        // the caller passes (current, then record). This ordering is
   270	        // load → query → record → save (per evaluator wire site).
   271	        let mut history = HVppuHistory::new();
   272	        history.record("p1", 0.4);
   273	        let h_before_record = history.h_vppu_for("p1", 0.6).unwrap();
   274	        history.record("p1", 0.6);
   275	        let h_after_record = history.h_vppu_for("p1", 0.6).unwrap();
   276	        // Before record: 0.6 / 0.4 = 1.5
   277	        // After record: 0.6 / mean(0.4, 0.6) = 0.6 / 0.5 = 1.2
   278	        assert!((h_before_record - 1.5).abs() < 1e-12);
   279	        assert!((h_after_record - 1.2).abs() < 1e-12);
   280	    }
   281	}

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1818,1845p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1818	        gp_node_count: gp_nodes,
  1819	        tx_count,
  1820	        build_sha,
  1821	        classifier_version,
  1822	        boltzmann_seed,
  1823	        tool_dist,
  1824	        unique_payload_ratio,
  1825	        gp_payload,
  1826	        gp_path,
  1827	        gp_proof_file,
  1828	        // B7-extra: only the calibration-treatment short-circuit site mutates
  1829	        // this to Some(true). Default = None (most callers).
  1830	        synthetic_short_circuit: None,
  1831	        // TB-1 Day-1: stamped post-construction at the prompt-build site
  1832	        // (run_oneshot today). Default None lets non-prompt-stamping
  1833	        // callers (tests, error-path returns before prompt build) round-trip.
  1834	        prompt_context_hash: None,
  1835	        // TB-1 Day-1: declared field; computation lands TB-1 Day 4.
  1836	        h_vppu: None,
  1837	    }
  1838	}
  1839	
  1840	/// Phase 0 (C-039 candidate): persist a self-contained, re-verifiable proof artifact.
  1841	/// Writes <EXPERIMENT_DIR>/proofs/<theorem>_<timestamp>_<short_hash>.lean containing
  1842	/// the exact code that the Lean oracle accepted. An external verifier can run
  1843	/// `lean --stdin < <file>` with the matching toolchain + Mathlib and reproduce the result.
  1844	/// Returns the relative path (for embedding in PputResult) or None on I/O failure.
  1845	fn persist_proof_artifact(

exec
/bin/bash -lc "rg -n \"serde_json::to_(string|vec|value)|serde_json::from_|records\\(\\)|public_view\\(\\)|raw_diagnostic_cid\" -S src tests experiments | head -n 220" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/tb_1_acceptance.rs:127:        serde_json::from_slice(&raw).unwrap();
tests/tb_1_acceptance.rs:129:    let bytes = serde_json::to_vec(&tampered).unwrap();
tests/tb_1_acceptance.rs:178:    let r = &l4e.records()[0];
tests/tb_1_acceptance.rs:180:    assert!(r.raw_diagnostic_cid.is_some());
tests/tb_1_acceptance.rs:228:    let view: Vec<PublicRejectionView> = l4e.public_view();
tests/tb_1_acceptance.rs:231:    let json = serde_json::to_value(&view[0]).unwrap();
tests/tb_1_acceptance.rs:234:        !obj.contains_key("raw_diagnostic_cid"),
tests/tb_1_acceptance.rs:235:        "raw_diagnostic_cid must NOT appear in agent-facing public view"
tests/tb_1_acceptance.rs:243:        l4e.records()[0].raw_diagnostic_cid.is_some(),
tests/tb_1_acceptance.rs:244:        "L4.E forensic record must retain raw_diagnostic_cid (shielding is structural, not destructive)"
tests/tb_1_acceptance.rs:444:        l4e.records()[0].rejection_class,
tests/q_state_reconstruct.rs:17:    let s = serde_json::to_string(&g).unwrap();
tests/q_state_reconstruct.rs:18:    let back: QState = serde_json::from_str(&s).unwrap();
tests/q_state_reconstruct.rs:41:    let s = serde_json::to_string(&q).unwrap();
tests/q_state_reconstruct.rs:42:    let back: QState = serde_json::from_str(&s).unwrap();
tests/q_state_reconstruct.rs:60:    assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap());
tests/q_state_reconstruct.rs:65:    let v = serde_json::to_value(QState::genesis()).unwrap();
tests/q_state_reconstruct.rs:87:    let v = serde_json::to_value(&e).unwrap();
src/economy/money.rs:294:        let s = serde_json::to_string(&m).unwrap();
src/economy/money.rs:295:        let m2: MicroCoin = serde_json::from_str(&s).unwrap();
src/economy/money.rs:303:        let s = serde_json::to_string(&m).unwrap();
src/economy/ledger.rs:292:        let bytes = serde_json::to_vec(&self.entries).map_err(|e| LedgerError::Io(e.to_string()))?;
src/economy/ledger.rs:303:            serde_json::from_slice(&bytes).map_err(|e| LedgerError::Io(e.to_string()))?;
src/top_white/predicates/visibility.rs:77:        let s = serde_json::to_string(&v).unwrap();
src/top_white/predicates/visibility.rs:78:        let v2: Visibility = serde_json::from_str(&s).unwrap();
src/top_white/predicates/visibility.rs:88:        let s = serde_json::to_string(&v).unwrap();
src/top_white/predicates/visibility.rs:89:        let v2: Visibility = serde_json::from_str(&s).unwrap();
src/top_white/predicates/registry.rs:69:        h.update(serde_json::to_vec(&self.visibility).expect("visibility serialize"));
src/top_white/predicates/registry.rs:72:        h.update(serde_json::to_vec(&self.safety_class).expect("safety_class serialize"));
src/bottom_white/tools/registry.rs:93:        h.update(serde_json::to_vec(&self.capability).expect("capability serialize"));
src/bottom_white/tools/registry.rs:94:        h.update(serde_json::to_vec(&self.permission_policy).expect("perm serialize"));
src/bottom_white/tools/registry.rs:95:        h.update(serde_json::to_vec(&self.determinism_class).expect("det serialize"));
src/bottom_white/tools/registry.rs:96:        h.update(serde_json::to_vec(&self.side_effect_class).expect("se serialize"));
experiments/minif2f_v4/tests/fc_trace_smoke.rs:76:        serde_json::from_str(line).expect("emitted line must be valid JSON");
tests/economic_state_reconstruct.rs:15:    let v = serde_json::to_value(&e).unwrap();
tests/economic_state_reconstruct.rs:74:    let s = serde_json::to_string(&e).unwrap();
tests/economic_state_reconstruct.rs:75:    let back: EconomicState = serde_json::from_str(&s).unwrap();
tests/economic_state_reconstruct.rs:91:    assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap());
tests/economic_state_reconstruct.rs:96:    assert_eq!(serde_json::to_string(&BalancesIndex::default()).unwrap(), "{}");
tests/economic_state_reconstruct.rs:97:    assert_eq!(serde_json::to_string(&EscrowsIndex::default()).unwrap(), "{}");
tests/economic_state_reconstruct.rs:98:    assert_eq!(serde_json::to_string(&StakesIndex::default()).unwrap(), "{}");
tests/economic_state_reconstruct.rs:99:    assert_eq!(serde_json::to_string(&ClaimsIndex::default()).unwrap(), "{}");
tests/economic_state_reconstruct.rs:100:    assert_eq!(serde_json::to_string(&ReputationsIndex::default()).unwrap(), "{}");
tests/economic_state_reconstruct.rs:101:    assert_eq!(serde_json::to_string(&TaskMarketsIndex::default()).unwrap(), "{}");
tests/economic_state_reconstruct.rs:102:    assert_eq!(serde_json::to_string(&RoyaltyGraph::default()).unwrap(), "{}");
tests/economic_state_reconstruct.rs:103:    assert_eq!(serde_json::to_string(&ChallengeCasesIndex::default()).unwrap(), "{}");
tests/economic_state_reconstruct.rs:104:    assert_eq!(serde_json::to_string(&PriceIndex::default()).unwrap(), "{}");
tests/six_axioms_alignment.rs:45:    let v = serde_json::to_value(&g).unwrap();
tests/six_axioms_alignment.rs:53:    let e = serde_json::to_value(&g.economic_state_t).unwrap();
tests/six_axioms_alignment.rs:61:    let v = serde_json::to_value(&g).unwrap();
tests/six_axioms_alignment.rs:70:    let v = serde_json::to_value(&g).unwrap();
tests/six_axioms_alignment.rs:80:    let v = serde_json::to_value(&g).unwrap();
tests/six_axioms_alignment.rs:99:    let _ = serde_json::to_string(&q).unwrap();
experiments/minif2f_v4/src/jsonl_schema.rs:227:        let v: serde_json::Value = serde_json::from_str(line)?;
experiments/minif2f_v4/src/jsonl_schema.rs:233:            Ok(RunRecord::V2(serde_json::from_value(v)?))
experiments/minif2f_v4/src/jsonl_schema.rs:235:            Ok(RunRecord::Legacy(serde_json::from_value(v)?))
experiments/minif2f_v4/src/jsonl_schema.rs:277:        let line = serde_json::to_string(&original).expect("serialize");
experiments/minif2f_v4/src/jsonl_schema.rs:278:        let parsed: RunAggregate = serde_json::from_str(&line).expect("deserialize");
experiments/minif2f_v4/src/jsonl_schema.rs:326:        let v2_line = serde_json::to_string(&sample_run()).unwrap();
experiments/minif2f_v4/src/jsonl_schema.rs:344:        let line = serde_json::to_string(&r).unwrap();
experiments/minif2f_v4/src/jsonl_schema.rs:348:        let parsed: RunAggregate = serde_json::from_str(&line).unwrap();
experiments/minif2f_v4/src/jsonl_schema.rs:375:        let line = serde_json::to_string(&r).unwrap();
experiments/minif2f_v4/src/jsonl_schema.rs:378:        let parsed: RunAggregate = serde_json::from_str(&line).unwrap();
experiments/minif2f_v4/src/h_vppu_history.rs:64:            Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|e| {
experiments/minif2f_v4/src/h_vppu_history.rs:80:        let serialized = serde_json::to_string_pretty(self).map_err(io::Error::other)?;
experiments/minif2f_v4/src/bin/evaluator.rs:389:    let json = serde_json::to_string(&result).unwrap();
experiments/minif2f_v4/src/bin/evaluator.rs:1999:        let line = serde_json::to_string(&result).expect("serialize PputResult");
experiments/minif2f_v4/src/bin/evaluator.rs:2106:        let line = serde_json::to_string(&result).expect("serialize PputResult");
experiments/minif2f_v4/src/bin/evaluator.rs:2164:        let line = serde_json::to_string(&result).expect("serialize PputResult");
src/bottom_white/cas/schema.rs:83:        h.update(serde_json::to_vec(&self.object_type).expect("object_type serialize"));
src/bottom_white/cas/store.rs:106:            serde_json::from_str(line).map_err(|e| CasError::IndexParse {
src/bottom_white/cas/store.rs:120:    let serialized = serde_json::to_string(meta).map_err(|e| CasError::IndexParse {
src/state/q_state.rs:360:        let s = serde_json::to_value(QState::genesis()).unwrap();
src/state/q_state.rs:386:        let s = serde_json::to_value(&e).unwrap();
src/state/q_state.rs:407:        let sa = serde_json::to_string(&a).unwrap();
src/state/q_state.rs:408:        let sb = serde_json::to_string(&b).unwrap();
src/sdk/tools/wallet.rs:87:        let json = serde_json::to_string_pretty(self)
src/sdk/tools/wallet.rs:96:        serde_json::from_str(&raw).ok()
src/sdk/protocol.rs:148:    serde_json::from_str::<AgentAction>(json_str)
src/drivers/llm_http.rs:183:        let json = serde_json::to_string(&req).unwrap();
src/wal.rs:59:        let line = serde_json::to_string(rec)?;
src/wal.rs:85:            match serde_json::from_str::<WalRecord>(&line) {
src/bottom_white/ledger/rejection_evidence.rs:25://! - `raw_diagnostic_cid` is a CAS handle to the raw error bytes; the
src/bottom_white/ledger/rejection_evidence.rs:79:/// - `raw_diagnostic_cid` holds the raw error content shielded behind a CAS
src/bottom_white/ledger/rejection_evidence.rs:99:    pub raw_diagnostic_cid: Option<Cid>,
src/bottom_white/ledger/rejection_evidence.rs:117:        raw_diagnostic_cid: &Option<Cid>,
src/bottom_white/ledger/rejection_evidence.rs:130:        match raw_diagnostic_cid {
src/bottom_white/ledger/rejection_evidence.rs:157:/// `raw_diagnostic_cid`. Materializing this view from a
src/bottom_white/ledger/rejection_evidence.rs:256:        raw_diagnostic_cid: Option<Cid>,
src/bottom_white/ledger/rejection_evidence.rs:267:            &raw_diagnostic_cid,
src/bottom_white/ledger/rejection_evidence.rs:278:            raw_diagnostic_cid,
src/bottom_white/ledger/rejection_evidence.rs:305:                &r.raw_diagnostic_cid,
src/bottom_white/ledger/rejection_evidence.rs:318:    /// records carry `raw_diagnostic_cid` and MUST NOT be exposed across the
src/bottom_white/ledger/rejection_evidence.rs:326:    /// `PublicRejectionView` does not carry `raw_diagnostic_cid` by type
src/bottom_white/ledger/rejection_evidence.rs:386:        assert_eq!(w.records()[1].prev_hash, h1);
src/bottom_white/ledger/rejection_evidence.rs:392:    fn public_view_omits_raw_diagnostic_cid() {
src/bottom_white/ledger/rejection_evidence.rs:404:        let view = w.public_view();
src/bottom_white/ledger/rejection_evidence.rs:407:        // `raw_diagnostic_cid` field. Round-trip via JSON to assert the
src/bottom_white/ledger/rejection_evidence.rs:409:        let json = serde_json::to_value(&view[0]).unwrap();
src/bottom_white/ledger/rejection_evidence.rs:411:        assert!(!obj.contains_key("raw_diagnostic_cid"));

exec
/bin/bash -lc "rg -n \"RejectedSubmissionRecord|RejectionEvidenceWriter|raw_diagnostic_cid|PublicRejectionView\" -S src tests experiments --glob '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/tb_1_acceptance.rs:35:    PublicRejectionView, RejectionClass, RejectionEvidenceError, RejectionEvidenceWriter,
tests/tb_1_acceptance.rs:149:    let mut l4e = RejectionEvidenceWriter::new();
tests/tb_1_acceptance.rs:180:    assert!(r.raw_diagnostic_cid.is_some());
tests/tb_1_acceptance.rs:216:    let mut l4e = RejectionEvidenceWriter::new();
tests/tb_1_acceptance.rs:228:    let view: Vec<PublicRejectionView> = l4e.public_view();
tests/tb_1_acceptance.rs:232:    let obj = json.as_object().expect("PublicRejectionView serializes as object");
tests/tb_1_acceptance.rs:234:        !obj.contains_key("raw_diagnostic_cid"),
tests/tb_1_acceptance.rs:235:        "raw_diagnostic_cid must NOT appear in agent-facing public view"
tests/tb_1_acceptance.rs:243:        l4e.records()[0].raw_diagnostic_cid.is_some(),
tests/tb_1_acceptance.rs:244:        "L4.E forensic record must retain raw_diagnostic_cid (shielding is structural, not destructive)"
tests/tb_1_acceptance.rs:279:    let mut l4e = RejectionEvidenceWriter::new();
tests/tb_1_acceptance.rs:427:    let mut l4e = RejectionEvidenceWriter::new();
src/bottom_white/ledger/rejection_evidence.rs:21://! - In-memory `Vec<RejectedSubmissionRecord>` chained via `prev_hash`.
src/bottom_white/ledger/rejection_evidence.rs:25://! - `raw_diagnostic_cid` is a CAS handle to the raw error bytes; the
src/bottom_white/ledger/rejection_evidence.rs:26://!   `PublicRejectionView` projection (used to materialize agent-facing
src/bottom_white/ledger/rejection_evidence.rs:70:// RejectedSubmissionRecord — one row on the L4.E chain
src/bottom_white/ledger/rejection_evidence.rs:79:/// - `raw_diagnostic_cid` holds the raw error content shielded behind a CAS
src/bottom_white/ledger/rejection_evidence.rs:83:pub struct RejectedSubmissionRecord {
src/bottom_white/ledger/rejection_evidence.rs:98:    /// `None` when no raw payload is captured. NEVER exposed via `PublicRejectionView`.
src/bottom_white/ledger/rejection_evidence.rs:99:    pub raw_diagnostic_cid: Option<Cid>,
src/bottom_white/ledger/rejection_evidence.rs:109:impl RejectedSubmissionRecord {
src/bottom_white/ledger/rejection_evidence.rs:117:        raw_diagnostic_cid: &Option<Cid>,
src/bottom_white/ledger/rejection_evidence.rs:130:        match raw_diagnostic_cid {
src/bottom_white/ledger/rejection_evidence.rs:151:// PublicRejectionView — agent-facing projection (Inv 10 Goodhart shield)
src/bottom_white/ledger/rejection_evidence.rs:157:/// `raw_diagnostic_cid`. Materializing this view from a
src/bottom_white/ledger/rejection_evidence.rs:158:/// `RejectedSubmissionRecord` cannot accidentally leak the raw diagnostic
src/bottom_white/ledger/rejection_evidence.rs:161:pub struct PublicRejectionView {
src/bottom_white/ledger/rejection_evidence.rs:170:impl From<&RejectedSubmissionRecord> for PublicRejectionView {
src/bottom_white/ledger/rejection_evidence.rs:171:    fn from(r: &RejectedSubmissionRecord) -> Self {
src/bottom_white/ledger/rejection_evidence.rs:187:/// TRACE_MATRIX P1:6 — error returned by `RejectionEvidenceWriter::verify_chain`.
src/bottom_white/ledger/rejection_evidence.rs:206:// RejectionEvidenceWriter — append + verify + project-to-public
src/bottom_white/ledger/rejection_evidence.rs:211:/// One `Vec<RejectedSubmissionRecord>`; `prev_hash` chained; `submit_id`
src/bottom_white/ledger/rejection_evidence.rs:217:pub struct RejectionEvidenceWriter {
src/bottom_white/ledger/rejection_evidence.rs:218:    records: Vec<RejectedSubmissionRecord>,
src/bottom_white/ledger/rejection_evidence.rs:221:impl RejectionEvidenceWriter {
src/bottom_white/ledger/rejection_evidence.rs:256:        raw_diagnostic_cid: Option<Cid>,
src/bottom_white/ledger/rejection_evidence.rs:260:        let hash = RejectedSubmissionRecord::compute_hash(
src/bottom_white/ledger/rejection_evidence.rs:267:            &raw_diagnostic_cid,
src/bottom_white/ledger/rejection_evidence.rs:271:        let record = RejectedSubmissionRecord {
src/bottom_white/ledger/rejection_evidence.rs:278:            raw_diagnostic_cid,
src/bottom_white/ledger/rejection_evidence.rs:298:            let recomputed = RejectedSubmissionRecord::compute_hash(
src/bottom_white/ledger/rejection_evidence.rs:305:                &r.raw_diagnostic_cid,
src/bottom_white/ledger/rejection_evidence.rs:318:    /// records carry `raw_diagnostic_cid` and MUST NOT be exposed across the
src/bottom_white/ledger/rejection_evidence.rs:320:    pub fn records(&self) -> &[RejectedSubmissionRecord] {
src/bottom_white/ledger/rejection_evidence.rs:326:    /// `PublicRejectionView` does not carry `raw_diagnostic_cid` by type
src/bottom_white/ledger/rejection_evidence.rs:329:    pub fn public_view(&self) -> Vec<PublicRejectionView> {
src/bottom_white/ledger/rejection_evidence.rs:330:        self.records.iter().map(PublicRejectionView::from).collect()
src/bottom_white/ledger/rejection_evidence.rs:362:        let mut w = RejectionEvidenceWriter::new();
src/bottom_white/ledger/rejection_evidence.rs:392:    fn public_view_omits_raw_diagnostic_cid() {
src/bottom_white/ledger/rejection_evidence.rs:393:        let mut w = RejectionEvidenceWriter::new();
src/bottom_white/ledger/rejection_evidence.rs:406:        // Structural isolation: `PublicRejectionView` doesn't have a
src/bottom_white/ledger/rejection_evidence.rs:407:        // `raw_diagnostic_cid` field. Round-trip via JSON to assert the
src/bottom_white/ledger/rejection_evidence.rs:411:        assert!(!obj.contains_key("raw_diagnostic_cid"));
src/bottom_white/ledger/rejection_evidence.rs:417:        let mut w = RejectionEvidenceWriter::new();

exec
/bin/bash -lc "rg -n \"assert_no_post_init_mint\\(|assert_total_ctf_conserved\\(|assert_read_is_free\\(\" -S src tests experiments --glob '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/economy/monetary_invariant.rs:134:pub fn assert_no_post_init_mint(tx: &TypedTx, q: &QState) -> Result<(), MonetaryError> {
src/economy/monetary_invariant.rs:167:pub fn assert_total_ctf_conserved(
src/economy/monetary_invariant.rs:202:pub fn assert_read_is_free(tx_kind: TxKind, fee: u64) -> Result<(), MonetaryError> {
src/economy/monetary_invariant.rs:258:        assert_eq!(assert_no_post_init_mint(&work, &q), Ok(()));
src/economy/monetary_invariant.rs:277:            assert_eq!(assert_no_post_init_mint(&t, &q), Ok(()),
src/economy/monetary_invariant.rs:292:        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
src/economy/monetary_invariant.rs:301:        let r = assert_total_ctf_conserved(&before, &after, &[]);
src/economy/monetary_invariant.rs:313:        let r = assert_total_ctf_conserved(&before, &after, &[]);
src/economy/monetary_invariant.rs:326:            assert_total_ctf_conserved(&before, &after, &[TxKind::FinalizeReward]),
src/economy/monetary_invariant.rs:342:        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
src/economy/monetary_invariant.rs:451:            assert_eq!(assert_read_is_free(k, 0), Ok(()));
src/economy/monetary_invariant.rs:458:        let r = assert_read_is_free(TxKind::Reuse, 1);
src/economy/monetary_invariant.rs:460:        let r = assert_read_is_free(TxKind::Work, 9999);
tests/tb_1_acceptance.rs:321:    assert_eq!(assert_total_ctf_conserved(&baseline, &s1, &[]), Ok(()));
tests/tb_1_acceptance.rs:331:    assert_eq!(assert_total_ctf_conserved(&s1, &s2, &[]), Ok(()));
tests/tb_1_acceptance.rs:337:    assert_eq!(assert_total_ctf_conserved(&s2, &s3, &[]), Ok(()));
tests/tb_1_acceptance.rs:343:    assert_eq!(assert_total_ctf_conserved(&s3, &s4, &[]), Ok(()));
tests/tb_1_acceptance.rs:348:    assert_eq!(assert_total_ctf_conserved(&s4, &s5, &[]), Ok(()));
tests/tb_1_acceptance.rs:365:    // against ALL TxKinds. assert_read_is_free(kind, fee=0) must succeed for
tests/tb_1_acceptance.rs:377:            assert_read_is_free(kind, 0),
tests/tb_1_acceptance.rs:386:        assert_read_is_free(TxKind::Reuse, 1),
tests/tb_1_acceptance.rs:393:        assert_read_is_free(TxKind::Work, 9999),
tests/tb_1_acceptance.rs:411:    let r = assert_total_ctf_conserved(&before, &after, &[]);

codex
So far, the evidence is pointing to a wording/coverage problem, not a failing implementation primitive: `dispatch_transition` is still a stub and the monetary guards have no non-test call sites. I’m running the relevant tests and checking the historical consolidation now.
exec
/bin/bash -lc 'git show --name-status --oneline 846279f -- tests/tb_1_p1_acceptance.rs tests/tb_1_acceptance.rs && git show --name-status --oneline 6c04c26 -- tests/tb_1_p1_acceptance.rs tests/tb_1_acceptance.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
846279f TB-1 Day-3: P1 GitTape ledger.rs + rejection_evidence.rs (NEW files)
A	tests/tb_1_p1_acceptance.rs
6c04c26 TB-1 Day-5: Tier-A 9 acceptance battery (consolidated; 9/9 PASS)
A	tests/tb_1_acceptance.rs
D	tests/tb_1_p1_acceptance.rs

exec
/bin/bash -lc 'cargo test --test tb_1_acceptance' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'cargo test -p minif2f_v4 --lib h_vppu_history' in /home/zephryj/projects/turingosv4
 succeeded in 17334ms:
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling ring v0.17.14
   Compiling rustls v0.23.38
   Compiling rustls-webpki v0.103.11
   Compiling tokio-rustls v0.26.4
   Compiling hyper-rustls v0.27.8
   Compiling reqwest v0.12.28
   Compiling turingosv4 v0.1.0 (/home/zephryj/projects/turingosv4)
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:13:13
   |
13 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::HashSet`
 --> src/sdk/actor.rs:9:5
  |
9 | use std::collections::HashSet;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused imports: `MICRO_PER_COIN` and `MicroCoin`
  --> src/economy/monetary_invariant.rs:26:29
   |
26 | use crate::economy::money::{MicroCoin, MICRO_PER_COIN};
   |                             ^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/cas/store.rs:26:12
   |
26 | use git2::{ObjectType as Git2ObjectType, Repository};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/ledger/transition_ledger.rs:36:12
   |
36 | use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_rejected_attempt_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:549:19
    |
549 |     pub(crate) fn sign_rejected_attempt_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:560:19
    |
560 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: function `sign_terminal_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:586:19
    |
586 |     pub(crate) fn sign_terminal_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_finalize_reward` is never used
   --> src/bottom_white/ledger/system_keypair.rs:597:19
    |
597 |     pub(crate) fn sign_finalize_reward(
    |                   ^^^^^^^^^^^^^^^^^^^^

warning: function `sign_task_expire` is never used
   --> src/bottom_white/ledger/system_keypair.rs:608:19
    |
608 |     pub(crate) fn sign_task_expire(
    |                   ^^^^^^^^^^^^^^^^

warning: function `sign_epoch_rotation_proof` is never used
   --> src/bottom_white/ledger/system_keypair.rs:616:19
    |
616 |     pub(crate) fn sign_epoch_rotation_proof(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:627:19
    |
627 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib) generated 19 warnings (run `cargo fix --lib -p turingosv4` to apply 12 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 17.23s
     Running tests/tb_1_acceptance.rs (target/debug/deps/tb_1_acceptance-9b2fe0f6ff43f57a)

running 13 tests
test test_at1_evaluator_solves_mathd_algebra_107_n3 ... ignored, Tier-B P6 anchor: requires live LLM proxy + DEEPSEEK_API_KEY; verified Day-4 (commit 50a1d67)
test test_at2_l4_entry_per_dispatched_tx ... ignored, Tier-B: WorkTx dispatch_transition body lands TB-2 RSP-1; evaluator currently uses legacy emit path
test test_at3_h_vppu_non_null_on_second_run ... ignored, Tier-B AT-3: covered by minif2f_v4 lib tests + Day-4 live evidence (commit 50a1d67); cannot import minif2f_v4 from turingosv4 integration tests
test test_at4_econ_balance_delta_non_zero ... ignored, Tier-B: needs RSP-1 escrow_lock_tx + yes_stake_tx wiring (TB-2)
test test_p1_kill_2_rejected_tx_no_state_advance ... ok
test test_p1_kill_4_rejected_log_isolated ... ok
test test_p1_kill_4b_rejection_chain_breaks_on_row_deletion ... ok
test test_p3_kill_1_no_post_init_mint ... ok
test test_p3_rsp0_exit_1_on_init_total_invariant ... ok
test test_p3_rsp0_exit_2_read_is_free ... ok
test test_p1_exit_7_l4_chain_breaks_on_row_deletion ... ok
test test_p1_kill_1_no_wtool_bypass ... ok
test test_p1_kill_3_ledger_reconstructable ... ok

test result: ok. 9 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out; finished in 0.01s


 succeeded in 24344ms:
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on artifact directory
   Compiling tokio-rustls v0.26.4
   Compiling hyper-rustls v0.27.8
   Compiling reqwest v0.12.28
   Compiling turingosv4 v0.1.0 (/home/zephryj/projects/turingosv4)
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:13:13
   |
13 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::HashSet`
 --> src/sdk/actor.rs:9:5
  |
9 | use std::collections::HashSet;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused imports: `MICRO_PER_COIN` and `MicroCoin`
  --> src/economy/monetary_invariant.rs:26:29
   |
26 | use crate::economy::money::{MicroCoin, MICRO_PER_COIN};
   |                             ^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/cas/store.rs:26:12
   |
26 | use git2::{ObjectType as Git2ObjectType, Repository};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/ledger/transition_ledger.rs:36:12
   |
36 | use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_rejected_attempt_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:549:19
    |
549 |     pub(crate) fn sign_rejected_attempt_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:560:19
    |
560 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: function `sign_terminal_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:586:19
    |
586 |     pub(crate) fn sign_terminal_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_finalize_reward` is never used
   --> src/bottom_white/ledger/system_keypair.rs:597:19
    |
597 |     pub(crate) fn sign_finalize_reward(
    |                   ^^^^^^^^^^^^^^^^^^^^

warning: function `sign_task_expire` is never used
   --> src/bottom_white/ledger/system_keypair.rs:608:19
    |
608 |     pub(crate) fn sign_task_expire(
    |                   ^^^^^^^^^^^^^^^^

warning: function `sign_epoch_rotation_proof` is never used
   --> src/bottom_white/ledger/system_keypair.rs:616:19
    |
616 |     pub(crate) fn sign_epoch_rotation_proof(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:627:19
    |
627 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib) generated 19 warnings (run `cargo fix --lib -p turingosv4` to apply 12 suggestions)
   Compiling minif2f_v4 v0.1.0 (/home/zephryj/projects/turingosv4/experiments/minif2f_v4)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 24.25s
     Running unittests src/lib.rs (target/debug/deps/minif2f_v4-33a072f940fb77c3)

running 9 tests
test h_vppu_history::tests::test_capacity_3_rolling_window ... ok
test h_vppu_history::tests::test_first_run_returns_none ... ok
test h_vppu_history::tests::test_corrupt_file_degrades_to_default ... ok
test h_vppu_history::tests::test_load_missing_file_default ... ok
test h_vppu_history::tests::test_per_problem_isolation ... ok
test h_vppu_history::tests::test_second_run_returns_ratio ... ok
test h_vppu_history::tests::test_record_before_query_does_not_self_reference ... ok
test h_vppu_history::tests::test_persistence_roundtrip ... ok
test h_vppu_history::tests::test_zero_mean_returns_none ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 104 filtered out; finished in 0.00s


exec
/bin/bash -lc 'git show 846279f:tests/tb_1_p1_acceptance.rs | rg -n "fn test_p1|let canonical_root|resulting_state_root|public_view|tamper_remove"' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
99:fn test_p1_kill_1_no_wtool_bypass() {
108:    let canonical_root = l.current_state_root();
114:    // state_root by replacing one entry's resulting_state_root JSON-side.
118:    // Tamper the last entry's resulting_state_root (a wtool-bypass would have
120:    tampered.last_mut().unwrap().resulting_state_root = Hash([0xFF; 32]);
143:fn test_p1_kill_2_rejected_tx_no_state_advance() {
192:fn test_p1_kill_3_ledger_reconstructable() {
219:fn test_p1_kill_4_rejected_log_isolated() {
233:    let view: Vec<PublicRejectionView> = l4e.public_view();
263:fn test_p1_kill_4b_rejection_chain_breaks_on_row_deletion() {
281:    l4e.tamper_remove_record(1);
295:fn test_p1_exit_7_l4_chain_breaks_on_row_deletion() {
303:    l.tamper_remove_entry(2);

exec
/bin/bash -lc 'git show 6c04c26:tests/tb_1_acceptance.rs | rg -n "fn test_p1|let canonical_root|resulting_state_root|public_view|tamper_remove"' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
114:fn test_p1_kill_1_no_wtool_bypass() {
119:    let canonical_root = l.current_state_root();
128:    tampered.last_mut().unwrap().resulting_state_root = Hash([0xFF; 32]);
147:fn test_p1_kill_2_rejected_tx_no_state_advance() {
189:fn test_p1_kill_3_ledger_reconstructable() {
215:fn test_p1_kill_4_rejected_log_isolated() {
228:    let view: Vec<PublicRejectionView> = l4e.public_view();
253:fn test_p1_exit_7_l4_chain_breaks_on_row_deletion() {
260:    l.tamper_remove_entry(2);
278:fn test_p1_kill_4b_rejection_chain_breaks_on_row_deletion() {
294:    l4e.tamper_remove_record(1);

exec
/bin/bash -lc "diff -u <(git show 846279f:tests/tb_1_p1_acceptance.rs | sed -n '1,305p') <(git show 6c04c26:tests/tb_1_acceptance.rs | sed -n '1,301p')" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
--- /dev/fd/63	2026-04-29 18:25:14.559264378 +0000
+++ /dev/fd/62	2026-04-29 18:25:14.567265117 +0000
@@ -1,28 +1,32 @@
-//! TB-1 Day-3 P1 acceptance battery (6 tests).
+//! TB-1 Day-5 final acceptance battery — Tier-A 9 BLOCKING + Tier-B 4 NON-BLOCKING.
 //!
-//! Charter: `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` § Day-3.
+//! Charter: `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` § Day-5.
+//! Tier discipline (audit CF-5 "lighter option"): TB-1 ships when ALL Tier-A
+//! tests are green. Tier-B tests are captured as artifacts but DO NOT gate
+//! ship; if a Tier-B test goes red, file as a follow-up TB rather than
+//! blocking TB-1's P1/P3 RSP-0 deliverable.
 //!
-//! Six tests cover P1 kill criteria + L4 hash-chain Exit:
-//!   1. test_p1_kill_1_no_wtool_bypass
-//!      — direct mutation to state.db that bypassed L4 fails to round-trip via
-//!        `AcceptedLedger::reconstruct_state`. Persisted state.db can ALWAYS be
-//!        re-derived from the L4 chain; any bypass write is washed out.
-//!   2. test_p1_kill_2_rejected_tx_no_state_advance
-//!      — a rejected tx leaves `state_root` unchanged, L4 `logical_t` NOT
-//!        incremented, and L4.E records exactly one `submit_id`-scoped record
-//!        with `raw_diagnostic_cid` populated.
-//!   3. test_p1_kill_3_ledger_reconstructable
-//!      — drop state.db; reconstruct from L4 only; bit-equal to pre-drop
-//!        `state_root`. L4.E intentionally NOT consulted.
-//!   4. test_p1_kill_4_rejected_log_isolated
-//!      — raw L4.E diagnostic NOT in another agent's materialized read view;
-//!        only `public_summary` (when explicitly set) crosses the boundary.
-//!   5. test_p1_kill_4b_rejection_chain_breaks_on_row_deletion
-//!      — write 3 rejection-evidence records; delete row 2;
-//!        `RejectionEvidenceWriter::verify_chain()` returns Err(HashMismatch).
-//!   6. test_p1_exit_7_l4_chain_breaks_on_row_deletion
-//!      — write 5 accepted L4 entries; delete row 3;
-//!        `AcceptedLedger::verify_chain(0, 4)` returns Err(HashMismatch).
+//! Tier-A (BLOCKING — P1 + P3 RSP-0 correctness):
+//!   1. test_p1_kill_1_no_wtool_bypass                       (P1 kill 1)
+//!   2. test_p1_kill_2_rejected_tx_no_state_advance          (P1 kill 2)
+//!   3. test_p1_kill_3_ledger_reconstructable                (P1 kill 3)
+//!   4. test_p1_kill_4_rejected_log_isolated                 (P1 kill 4)
+//!   5. test_p1_exit_7_l4_chain_breaks_on_row_deletion       (P1 Exit 7)
+//!   6. test_p1_kill_4b_rejection_chain_breaks_on_row_deletion (P1 kill 4b)
+//!   7. test_p3_rsp0_exit_1_on_init_total_invariant          (P3 RSP-0 Exit 1)
+//!   8. test_p3_rsp0_exit_2_read_is_free                     (P3 RSP-0 Exit 2)
+//!   9. test_p3_kill_1_no_post_init_mint                     (P3 kill 1)
+//!
+//! Tier-B (NON-BLOCKING — P6 anchor evidence + future-RSP placeholders):
+//!  10. test_at1_evaluator_solves_mathd_algebra_107_n3       (#[ignore]: live LLM)
+//!  11. test_at2_l4_entry_per_dispatched_tx                  (#[ignore]: WorkTx dispatch
+//!                                                           body lands TB-2 RSP-1)
+//!  12. test_at3_h_vppu_non_null_on_second_run               (UNIT form; live form
+//!                                                           verified by Day-4 evidence)
+//!  13. test_at4_econ_balance_delta_non_zero                 (#[ignore]: RSP-1)
+//!
+//! AT-5 (winning-tactic-in-prompt-context) is DESCOPED per recharter — moves
+//! to a future P5 MetaTape v1 TB after P3 RSP-3 lands.
 
 use std::collections::{BTreeMap, BTreeSet};
 
@@ -32,8 +36,11 @@
 };
 use turingosv4::bottom_white::ledger::transition_ledger::TxKind;
 use turingosv4::economy::ledger::{AcceptedLedger, LedgerError};
-use turingosv4::economy::money::StakeMicroCoin;
-use turingosv4::state::q_state::{AgentId, Hash, TxId};
+use turingosv4::economy::money::{MicroCoin, StakeMicroCoin, MICRO_PER_COIN};
+use turingosv4::economy::monetary_invariant::{
+    assert_read_is_free, assert_total_ctf_conserved, MonetaryError,
+};
+use turingosv4::state::q_state::{AgentId, EconomicState, Hash, TxId};
 use turingosv4::state::typed_tx::{
     AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey,
     SafetyOrCreation, TaskId, TypedTx, WorkTx, WriteKey,
@@ -91,16 +98,20 @@
     AgentId(s.to_string())
 }
 
+fn coin(n: i64) -> MicroCoin {
+    MicroCoin::from_coin(n).unwrap()
+}
+
+// ════════════════════════════════════════════════════════════════════════════
+// Tier-A — BLOCKING
+// ════════════════════════════════════════════════════════════════════════════
+
 // ────────────────────────────────────────────────────────────────────────────
 // (1) P1 kill 1 — no wtool bypass
 // ────────────────────────────────────────────────────────────────────────────
 
 #[test]
 fn test_p1_kill_1_no_wtool_bypass() {
-    // Build an accepted chain, persist, then simulate a "bypass" mutation that
-    // edits state.db on disk WITHOUT going through `append_accepted` → L4.
-    // Reconstruction from the L4 chain washes the bypass out: the
-    // reconstructed `state_root` does not match the (corrupted) on-disk view.
     let mut l = AcceptedLedger::new();
     for i in 1..=3 {
         l.append_accepted(&fixture_work_tx(i)).unwrap();
@@ -110,28 +121,21 @@
     let tmp = tempfile::NamedTempFile::new().unwrap();
     l.persist(tmp.path()).unwrap();
 
-    // Bypass: directly overwrite state.db with garbage that claims a different
-    // state_root by replacing one entry's resulting_state_root JSON-side.
+    // Bypass: directly overwrite state.db without going through L4.
     let raw = std::fs::read(tmp.path()).unwrap();
     let mut tampered: Vec<turingosv4::economy::ledger::AcceptedEntry> =
         serde_json::from_slice(&raw).unwrap();
-    // Tamper the last entry's resulting_state_root (a wtool-bypass would have
-    // mutated state without re-deriving the chain hash).
     tampered.last_mut().unwrap().resulting_state_root = Hash([0xFF; 32]);
     let bytes = serde_json::to_vec(&tampered).unwrap();
     std::fs::write(tmp.path(), bytes).unwrap();
 
-    // Reconstruction MUST fail to round-trip: either an explicit error, or a
-    // reconstructed root that no longer matches the canonical root.
-    let result = AcceptedLedger::load_from_path(tmp.path());
-    match result {
-        Err(_) => {} // bypass detected via integrity error — expected.
-        Ok((_, reconstructed)) => {
-            assert_ne!(
-                reconstructed, canonical_root,
-                "bypass mutation must not survive a round-trip through reconstruct_state"
-            );
-        }
+    // Reconstruction MUST fail: explicit error OR diverged root.
+    match AcceptedLedger::load_from_path(tmp.path()) {
+        Err(_) => {} // bypass detected — expected
+        Ok((_, reconstructed)) => assert_ne!(
+            reconstructed, canonical_root,
+            "bypass mutation must not survive a round-trip through reconstruct_state"
+        ),
     }
 }
 
@@ -144,12 +148,10 @@
     let mut l4 = AcceptedLedger::new();
     let mut l4e = RejectionEvidenceWriter::new();
 
-    // One accepted tx — sets a non-zero baseline state_root.
     l4.append_accepted(&fixture_work_tx(1)).unwrap();
     let baseline_root = l4.current_state_root();
     let baseline_logical_t = l4.len();
 
-    // Simulate predicate-failed dispatch: tx routes to L4.E, NOT L4.
     l4e.append_rejected(
         42,
         baseline_root,
@@ -157,11 +159,10 @@
         TxKind::Work,
         cid(0x20),
         RejectionClass::PredicateFailed,
-        Some(cid(0xAA)), // raw diagnostic populated
+        Some(cid(0xAA)),
         None,
     );
 
-    // L4: state_root unchanged, logical_t unchanged.
     assert_eq!(
         l4.current_state_root(),
         baseline_root,
@@ -173,14 +174,10 @@
         "rejected tx must NOT advance L4 logical_t"
     );
 
-    // L4.E: exactly one record, raw_diagnostic_cid populated.
     assert_eq!(l4e.len(), 1, "rejection produces exactly one L4.E record");
     let r = &l4e.records()[0];
     assert_eq!(r.submit_id, 42);
-    assert!(
-        r.raw_diagnostic_cid.is_some(),
-        "L4.E record must carry raw_diagnostic_cid"
-    );
+    assert!(r.raw_diagnostic_cid.is_some());
     assert!(l4e.verify_chain().is_ok());
 }
 
@@ -196,11 +193,10 @@
     }
     let pre_drop_root = l.current_state_root();
 
-    // Persist, drop in-memory state.db, reconstruct from chaintape.jsonl.
     let tmp = tempfile::NamedTempFile::new().unwrap();
     l.persist(tmp.path()).unwrap();
 
-    drop(l); // simulate full state-vector drop.
+    drop(l);
 
     let (l_reborn, reconstructed_root) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
     assert_eq!(
@@ -225,30 +221,24 @@
         TxKind::Work,
         cid(0x10),
         RejectionClass::PredicateFailed,
-        Some(cid(0xBE)), // raw diagnostic CID — NEVER materialized to other agents
+        Some(cid(0xBE)),
         Some("predicate acceptance failed for acc-7".into()),
     );
 
-    // The view another agent's materializer sees:
     let view: Vec<PublicRejectionView> = l4e.public_view();
     assert_eq!(view.len(), 1);
 
-    // Structural isolation: the type does not carry raw_diagnostic_cid.
-    // Round-trip via JSON to assert the wire form omits it too.
     let json = serde_json::to_value(&view[0]).unwrap();
     let obj = json.as_object().expect("PublicRejectionView serializes as object");
     assert!(
         !obj.contains_key("raw_diagnostic_cid"),
         "raw_diagnostic_cid must NOT appear in agent-facing public view"
     );
-    // Only public_summary crosses the boundary.
     assert_eq!(
         obj.get("public_summary").and_then(|v| v.as_str()),
         Some("predicate acceptance failed for acc-7")
     );
 
-    // Sanity: the underlying L4.E forensic record DID carry the raw cid
-    // (so the writer is not silently dropping it; it is shielded structurally).
     assert!(
         l4e.records()[0].raw_diagnostic_cid.is_some(),
         "L4.E forensic record must retain raw_diagnostic_cid (shielding is structural, not destructive)"
@@ -256,7 +246,32 @@
 }
 
 // ────────────────────────────────────────────────────────────────────────────
-// (5) P1 kill 4b — L4.E hash chain breaks on row deletion
+// (5) P1 Exit 7 — L4 hash chain breaks on row deletion
+// ────────────────────────────────────────────────────────────────────────────
+
+#[test]
+fn test_p1_exit_7_l4_chain_breaks_on_row_deletion() {
+    let mut l = AcceptedLedger::new();
+    for i in 1..=5 {
+        l.append_accepted(&fixture_work_tx(i)).unwrap();
+    }
+    assert!(l.verify_chain(0, 5).is_ok());
+
+    l.tamper_remove_entry(2);
+
+    let r = l.verify_chain(0, 4);
+    match r {
+        Err(LedgerError::LogicalTGap { at_index: 2, .. })
+        | Err(LedgerError::HashMismatch { at_index: 2 }) => {}
+        other => panic!(
+            "deleting an L4 row must break the chain at index 2; got {:?}",
+            other
+        ),
+    }
+}
+
+// ────────────────────────────────────────────────────────────────────────────
+// (6) P1 kill 4b — L4.E hash chain breaks on row deletion
 // ────────────────────────────────────────────────────────────────────────────
 
 #[test]
@@ -276,8 +291,6 @@
     }
     assert!(l4e.verify_chain().is_ok());
 
-    // Delete the middle row — surviving row's prev_hash now disagrees with its
-    // (new) predecessor's hash.
     l4e.tamper_remove_record(1);
     let r = l4e.verify_chain();
     assert!(
@@ -286,20 +299,3 @@
         r
     );
 }
-
-// ────────────────────────────────────────────────────────────────────────────
-// (6) P1 Exit 7 — L4 hash chain breaks on row deletion
-// ────────────────────────────────────────────────────────────────────────────
-
-#[test]
-fn test_p1_exit_7_l4_chain_breaks_on_row_deletion() {
-    let mut l = AcceptedLedger::new();
-    for i in 1..=5 {
-        l.append_accepted(&fixture_work_tx(i)).unwrap();
-    }
-    assert!(l.verify_chain(0, 5).is_ok());
-
-    // Delete row 2 (was logical_t=3); surviving rows are now [t=1, t=2, t=4, t=5].
-    l.tamper_remove_entry(2);
-
-    // Chain length is now 4. Verifying [0, 4):

exec
/bin/bash -lc 'ls -la /tmp/tb1_day4_smoke_v2' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
total 176
drwxr-xr-x  3 zephryj zephryj   4096 Apr 29 18:07 .
drwxrwxrwt 88 root    root    139264 Apr 29 18:25 ..
-rw-r--r--  1 zephryj zephryj    117 Apr 29 18:07 h_vppu_history.json
drwxr-xr-x  2 zephryj zephryj   4096 Apr 29 18:07 proofs
-rw-r--r--  1 zephryj zephryj    135 Apr 29 18:06 run1.err
-rw-r--r--  1 zephryj zephryj   1142 Apr 29 18:06 run1.jsonl
-rw-r--r--  1 zephryj zephryj   1142 Apr 29 18:06 run1.stdout
-rw-r--r--  1 zephryj zephryj    135 Apr 29 18:07 run2.err
-rw-r--r--  1 zephryj zephryj   1167 Apr 29 18:07 run2.jsonl
-rw-r--r--  1 zephryj zephryj   1167 Apr 29 18:07 run2.stdout

exec
/bin/bash -lc 'rg -n "PPUT_RESULT|h_vppu|solved|gp_payload" /tmp/tb1_day4_smoke_v2 -S' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/tmp/tb1_day4_smoke_v2/run1.jsonl:1:PPUT_RESULT:{"schema_version":"v2.0","run_id":"n3_mathd_algebra_107_1777485949022","problem_id":"mathd_algebra_107","solved":true,"split":"adaptation","verified":true,"golden_path_token_count":12,"total_run_token_count":448,"total_wall_time_ms":56950,"progress":1,"pput_runtime":0.00003919478239056817,"pput_verified":0.00003919478239056817,"pput_m_verified":39.19478239056817,"failed_branch_count":0,"rollback_count":0,"hit_max_tx":false,"tactic_diversity":1.0,"verifier_wait_ms":56121,"budget_regime":"total_proposal","budget_max_transactions":10,"far":0.0,"err":0.0,"iac":0.0,"cpr":0.0,"model_snapshot":"deepseek-chat","git_sha":"","binary_sha256":"","mode":"full","problem":"/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_107.lean","condition":"n3","model":"deepseek-chat","has_golden_path":true,"time_secs":56.950916981,"pput":1.7558979784884248,"gp_token_count":12,"gp_node_count":1,"tx_count":1,"classifier_version":"v1_2026-04-16-a","tool_dist":{"step":1,"omega_wtool":1},"gp_payload":"nlinarith","gp_path":"per_tactic","gp_proof_file":"proofs/mathd_algebra_107_1777486005_73ee91ba.lean"}
/tmp/tb1_day4_smoke_v2/run2.jsonl:1:PPUT_RESULT:{"schema_version":"v2.0","run_id":"n3_mathd_algebra_107_1777486023970","problem_id":"mathd_algebra_107","solved":true,"split":"adaptation","verified":true,"golden_path_token_count":12,"total_run_token_count":448,"total_wall_time_ms":9162,"progress":1,"pput_runtime":0.00024363052359123086,"pput_verified":0.00024363052359123086,"pput_m_verified":243.63052359123085,"failed_branch_count":0,"rollback_count":0,"hit_max_tx":false,"tactic_diversity":1.0,"verifier_wait_ms":8175,"budget_regime":"total_proposal","budget_max_transactions":10,"far":0.0,"err":0.0,"iac":0.0,"cpr":0.0,"model_snapshot":"deepseek-chat","git_sha":"","binary_sha256":"","mode":"full","problem":"/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_107.lean","condition":"n3","model":"deepseek-chat","has_golden_path":true,"time_secs":9.162645877,"pput":10.913878080895739,"gp_token_count":12,"gp_node_count":1,"tx_count":1,"classifier_version":"v1_2026-04-16-a","tool_dist":{"step":1,"omega_wtool":1},"gp_payload":"nlinarith","gp_path":"per_tactic","gp_proof_file":"proofs/mathd_algebra_107_1777486033_73ee91ba.lean","h_vppu":6.215891726697228}
/tmp/tb1_day4_smoke_v2/run2.stdout:1:PPUT_RESULT:{"schema_version":"v2.0","run_id":"n3_mathd_algebra_107_1777486023970","problem_id":"mathd_algebra_107","solved":true,"split":"adaptation","verified":true,"golden_path_token_count":12,"total_run_token_count":448,"total_wall_time_ms":9162,"progress":1,"pput_runtime":0.00024363052359123086,"pput_verified":0.00024363052359123086,"pput_m_verified":243.63052359123085,"failed_branch_count":0,"rollback_count":0,"hit_max_tx":false,"tactic_diversity":1.0,"verifier_wait_ms":8175,"budget_regime":"total_proposal","budget_max_transactions":10,"far":0.0,"err":0.0,"iac":0.0,"cpr":0.0,"model_snapshot":"deepseek-chat","git_sha":"","binary_sha256":"","mode":"full","problem":"/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_107.lean","condition":"n3","model":"deepseek-chat","has_golden_path":true,"time_secs":9.162645877,"pput":10.913878080895739,"gp_token_count":12,"gp_node_count":1,"tx_count":1,"classifier_version":"v1_2026-04-16-a","tool_dist":{"step":1,"omega_wtool":1},"gp_payload":"nlinarith","gp_path":"per_tactic","gp_proof_file":"proofs/mathd_algebra_107_1777486033_73ee91ba.lean","h_vppu":6.215891726697228}
/tmp/tb1_day4_smoke_v2/run1.stdout:1:PPUT_RESULT:{"schema_version":"v2.0","run_id":"n3_mathd_algebra_107_1777485949022","problem_id":"mathd_algebra_107","solved":true,"split":"adaptation","verified":true,"golden_path_token_count":12,"total_run_token_count":448,"total_wall_time_ms":56950,"progress":1,"pput_runtime":0.00003919478239056817,"pput_verified":0.00003919478239056817,"pput_m_verified":39.19478239056817,"failed_branch_count":0,"rollback_count":0,"hit_max_tx":false,"tactic_diversity":1.0,"verifier_wait_ms":56121,"budget_regime":"total_proposal","budget_max_transactions":10,"far":0.0,"err":0.0,"iac":0.0,"cpr":0.0,"model_snapshot":"deepseek-chat","git_sha":"","binary_sha256":"","mode":"full","problem":"/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_107.lean","condition":"n3","model":"deepseek-chat","has_golden_path":true,"time_secs":56.950916981,"pput":1.7558979784884248,"gp_token_count":12,"gp_node_count":1,"tx_count":1,"classifier_version":"v1_2026-04-16-a","tool_dist":{"step":1,"omega_wtool":1},"gp_payload":"nlinarith","gp_path":"per_tactic","gp_proof_file":"proofs/mathd_algebra_107_1777486005_73ee91ba.lean"}

exec
/bin/bash -lc 'find handover -maxdepth 3 -type f | rg "day4|tb1|TB-1|h_vppu|smoke|evidence|audit"' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/tracer_bullets/TB-1_day1_spike_2026-04-29.md
handover/tracer_bullets/TB-1_day1_oneshot.jsonl
handover/tracer_bullets/TB-1_recharter_2026-04-29.md
handover/ai-direct/EXT_AUDIT_2026-04-21/run_audit.py
handover/evidence/e1_proofs/imo_1962_p2_1776951132_82f02b76.lean
handover/evidence/e1_proofs/imo_1962_p2_1776958185_b7329f9c.lean
handover/evidence/e1_proofs/imo_1962_p2_1776869602_faeefc7.lean
handover/evidence/e1_proofs/mathd_algebra_332_1776994561_16871d44.lean
handover/evidence/e1_proofs/mathd_algebra_44_1776942255_6188d1b1.lean
handover/evidence/e1_proofs/mathd_algebra_44_1776956522_f7af89d8.lean
handover/evidence/e1_proofs/mathd_algebra_332_1776999060_e57b2dca.lean
handover/evidence/e1_proofs/mathd_algebra_44_1776999097_6277f804.lean
handover/evidence/e1_proofs/mathd_algebra_332_1776864088_c8854dcf.lean
handover/evidence/e1_proofs/mathd_algebra_44_1776873440_7c6d7e9.lean
handover/evidence/e1_proofs/mathd_algebra_44_1776994690_510aa55a.lean
handover/evidence/e1_proofs/mathd_algebra_332_1776999212_2d5c2db8.lean
handover/evidence/e1_proofs/imo_1962_p2_1776996434_6277f804.lean
handover/evidence/e1_proofs/mathd_algebra_44_1776869774_7c6d7e9.lean
handover/evidence/e1_proofs/mathd_algebra_44_1776864287_f1beef26.lean
handover/evidence/e1_proofs/algebra_apbon2pownleqapownpbpowon2_1777010656_fed75b88.lean
handover/evidence/README.md
handover/evidence/e1_jsonl/E1_A_homogeneous_n8_20260423T144314.jsonl
handover/evidence/e1_jsonl/E1_B_heterogeneous_n8_20260423T144327.jsonl
handover/evidence/e1_jsonl/E1_B_seed31415_n8_20260424T012605.jsonl
handover/evidence/e1_jsonl/E1_A_seed2718_n8_20260424T024005.jsonl
handover/evidence/e1_jsonl/E1_A_seed31415_n8_20260424T012605.jsonl
handover/evidence/e1_jsonl/E1_A_easy_ctrl_n8_20260424T024005.jsonl
handover/evidence/e1_jsonl/E1_B_seed2357_n8_20260424T045345.jsonl
handover/evidence/e1_jsonl/E1_B_seed2718_n8_20260424T024005.jsonl
handover/evidence/e1_jsonl/E1_ablation_no_meta_seed141421_n8_20260424T045345.jsonl
handover/evidence/e1_jsonl/E1_B_easy_ctrl_n8_20260424T024005.jsonl
handover/evidence/e1_jsonl/E1_A_seed2357_n8_20260424T045345.jsonl
handover/evidence/ADVERSARIAL_AUDIT_2026-04-23.md
handover/evidence/v2/E1v2_A_s2357_n8_20260424T151424.jsonl
handover/evidence/v2/E1v2_B_s2357_n8_20260424T151426.jsonl
handover/evidence/v2/E1v2_B_s2718_n8_20260424T140411.jsonl
handover/evidence/v2/E1v2_B_s31415_n8_20260424T112916.jsonl
handover/evidence/v2/E1v2_A_s141421_n8_20260424T084857.jsonl
handover/evidence/v2/E1v2_Abl_s2718_n8_20260424T140413.jsonl
handover/evidence/v2/E1v2_A_s31415_n8_20260424T112914.jsonl
handover/evidence/v2/E1v2_A_s2718_n8_20260424T124629.jsonl
handover/evidence/v2/E1v2_Abl_s31415_n8_20260424T124627.jsonl
handover/evidence/v2/E1v2_Abl_s141421_n8_20260424T102222.jsonl
handover/evidence/v2/E1v2_Abl_s2357_n8_20260424T163508.jsonl
handover/evidence/v2/E1v2_B_s141421_n8_20260424T102222.jsonl
handover/evidence/sample_E1_easy10.txt
handover/evidence/phase9a_jsonl/phase9a_seed2718_n8_20260423T041018.jsonl
handover/evidence/phase9a_jsonl/phase9a_c027fixed_seed31415_n8_20260422T220558.jsonl
handover/evidence/phase9a_jsonl/phase9a_seed141421_n8_20260423T101303.jsonl
handover/evidence/sample_E1_hard10.txt
handover/evidence/first_v4_solve_2026-04-29/run.err
handover/evidence/first_v4_solve_2026-04-29/README.md
handover/evidence/first_v4_solve_2026-04-29/mathd_algebra_107.lean
handover/evidence/first_v4_solve_2026-04-29/pput_result.jsonl
handover/audits/EXT_GEMINI_PHASE_2_AB_2026-04-22.md
handover/audits/GEMINI_PAPER1_AUDIT_2026-04-23.md
handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R2_2026-04-28.md
handover/audits/EXT_GEMINI_PHASE_8A_2026-04-22.md
handover/audits/run_gemini_phase_a0_exit_audit.py
handover/audits/GEMINI_INV8_DAG_AUDIT_2026-04-27.md
handover/audits/run_gemini_pput_ccl_audit.py
handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md
handover/audits/run_gemini_phase_a8_exit_audit.py
handover/audits/GEMINI_SPEC_V14_ROUND4_AUDIT_2026-04-27.md
handover/audits/GEMINI_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md
handover/audits/run_gemini_spec_freeze_audit.py
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R9.md
handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md
handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md
handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R10.md
handover/audits/run_gemini_inv8_audit.py
handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md
handover/audits/EXT_CODEX_PHASE_8_R1ALPHA_2026-04-22.md
handover/audits/EXT_CODEX_PHASE_8_BATCH_2026-04-22.md
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md
handover/audits/CO1_7_IMPL_BUNDLE_DUAL_AUDIT_VERDICT_FINAL_2026-04-28.md
handover/audits/CODEX_SPEC_V12_REAUDIT_2026-04-27.md
handover/audits/EXT_CODEX_2026-04-22.md
handover/audits/2026-04-29_external_audit.md
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R10.md
handover/audits/GEMINI_V32_REVIEW_2026-04-27.md
handover/audits/EXT_CODEX_PHASE_2_AB_2026-04-22.md
handover/audits/EXT_CODEX_PHASE_8A_2026-04-22.md
handover/audits/CODEX_T_S_REVIEW_2026-04-27.md
handover/audits/CODEX_PAPER1_AUDIT_2026-04-23.md
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md
handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R7.md
handover/audits/GEMINI_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md
handover/audits/run_codex_b7_extra_round4_audit.sh
handover/audits/GEMINI_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md
handover/audits/CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md
handover/audits/CODEX_CO1_13_ROUND2_AUDIT_2026-04-29.md
handover/audits/run_gemini_paper1_v2_1_audit.py
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md
handover/audits/run_codex_co1_7_extra_round2_audit.sh
handover/audits/run_gemini_co1_1_4_pre1_round1_audit.py
handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md
handover/audits/B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md
handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_ROUND4_2026-04-26.md
handover/audits/run_codex_co1_1_4_pre1_round4_audit.sh
handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md
handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md
handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md
handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_FINAL_2026-04-28.md
handover/audits/run_gemini_pput_ccl_audit_round3.py
handover/audits/run_codex_co1_1_4_pre1_round1_audit.sh
handover/audits/CO1_7_DUAL_AUDIT_VERDICT_R3_2026-04-28.md
handover/audits/AUDITOR_TFR_PLAN_AUDIT_2026-04-26.md
handover/audits/pput_scan.py
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md
handover/audits/CO1_7_IMPL_BUNDLE_DUAL_AUDIT_VERDICT_R1_2026-04-28.md
handover/audits/run_codex_co1_7_impl_bundle_round3_audit.sh
handover/audits/CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md
handover/audits/run_gemini_paper1_v2_audit.py
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md
handover/audits/GEMINI_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md
handover/audits/run_codex_pput_ccl_audit_round2.sh
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md
handover/audits/run_codex_co1_7_5_round1_audit.sh
handover/audits/GEMINI_CO1_8_ROUND1_AUDIT_2026-04-29.md
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R11.md
handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R8.md
handover/audits/run_gemini_paper1_audit.py
handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R9.md
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py
handover/audits/run_gemini_tb_1_audit_2026-04-29.py
handover/audits/GEMINI_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md
handover/audits/run_gemini_co_p0_audit.sh
handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md
handover/audits/run_gemini_co1_13_round2_audit.py
handover/audits/GEMINI_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md
handover/audits/run_gemini_co1_7_extra_round4_audit.py
handover/audits/GEMINI_CO_P0_AUDIT_2026-04-26.md
handover/audits/run_gemini_co1_7_extra_round3_audit.py
handover/audits/GEMINI_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md
handover/audits/DUAL_AUDIT_PAPER1_VERDICT_2026-04-23.md
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND3_AUDIT_2026-04-28.md
handover/audits/GEMINI_PPUT_CCL_AUDIT_2026-04-26.md
handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md
handover/audits/run_codex_pput_ccl_audit_round3.sh
handover/audits/run_gemini_co1_1_4_pre1_round2_audit.py
handover/audits/DUAL_AUDIT_V2_1_VERDICT_2026-04-25.md
handover/audits/GEMINI_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md
handover/audits/PPUT_REFRAME_2026-04-22.md
handover/audits/run_codex_co1_7_extra_round4_audit.sh
handover/audits/AUDIT_LEDGER.md
handover/audits/run_gemini_spec_v14_round4_audit.py
handover/audits/GEMINI_WHITEPAPER_V2_AUDIT_2026-04-27_R3.md
handover/audits/run_gemini_wp_revision_audit.py
handover/audits/SYNTHESIS_2026-04-22.md
handover/audits/GEMINI_WHITEPAPER_V2_AUDIT_2026-04-27.md
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md
handover/audits/run_gemini_co_p0_audit.py
handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md
handover/audits/run_codex_co1_7_round2_audit.sh
handover/audits/EXT_GEMINI_2026-04-21.md
handover/audits/CODEX_CO1_8_ROUND1_AUDIT_2026-04-29.md
handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26.md
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md
handover/audits/CODEX_CO1_13_ROUND1_AUDIT_2026-04-29.md
handover/audits/CODEX_INV8_DAG_AUDIT_2026-04-27.md
handover/audits/GEMINI_SPEC_V12_REAUDIT_2026-04-27.md
handover/audits/run_gemini_co1_1_4_pre1_round3_audit.py
handover/audits/CODEX_CO1_2_QSTATE_AUDIT_2026-04-27.md
handover/audits/DUAL_AUDIT_V2_VERDICT_2026-04-24.md
handover/audits/PPUT_HISTORICAL_AUDIT_2026-04-22.md
handover/audits/run_codex_co1_1_4_pre1_round2_audit.sh
handover/audits/A8_EXIT_PACKET_2026-04-26.md
handover/audits/run_codex_phase_a8_exit_audit.sh
handover/audits/CODEX_SPEC_FREEZE_AUDIT_2026-04-27.md
handover/audits/CODEX_WHITEPAPER_V2_AUDIT_2026-04-27.md
handover/audits/run_codex_phase_a0_exit_audit.sh
handover/audits/CODEX_WHITEPAPER_V2_AUDIT_2026-04-27_R3.md
handover/audits/run_gemini_pput_ccl_audit_round4.py
handover/audits/CODEX_CO1_1_4_PRE1_ROUND4_AUDIT_2026-04-28.md
handover/audits/GEMINI_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md
handover/audits/run_gemini_co1_7_5_round1_audit.py
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md
handover/audits/CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md
handover/audits/run_codex_pput_ccl_audit_round4.sh
handover/audits/GEMINI_CO1_13_ROUND1_AUDIT_2026-04-29.md
handover/audits/run_gemini_b7_extra_reaudit.py
handover/audits/GEMINI_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md
handover/audits/run_gemini_co1_7_round2_audit.py
handover/audits/run_gemini_spec_v12_reaudit.py
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md
handover/audits/CODEX_PAPER1_V2_1_AUDIT_2026-04-25.md
handover/audits/CODEX_SPEC_V13_REAUDIT_2026-04-27.md
handover/audits/run_gemini_co1_8_round1_audit.py
handover/audits/run_codex_co1_13_round1_audit.sh
handover/audits/EXT_GEMINI_PHASE_8_R1ALPHA_2026-04-22.md
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R12.md
handover/audits/run_gemini_pput_ccl_audit_round2.py
handover/audits/run_gemini_v32_review.py
handover/audits/CODEX_WHITEPAPER_V2_AUDIT_2026-04-27_R2.md
handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_2026-04-26.md
handover/audits/run_codex_co1_13_round2_audit.sh
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md
handover/audits/run_codex_co1_7_round1_audit.sh
handover/audits/run_codex_co1_8_round1_audit.sh
handover/audits/run_gemini_co1_7_impl_bundle_round1_audit.py
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md
handover/audits/GEMINI_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md
handover/audits/CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R4_2026-04-29.md
handover/audits/GEMINI_CO1_13_ROUND2_AUDIT_2026-04-29.md
handover/audits/run_gemini_co1_13_round1_audit.py
handover/audits/CLAUDE_AUDITOR_CO1_7_0AF_KEYPAIR_2026-04-27.md
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md
handover/audits/run_codex_b7_extra_reaudit.sh
handover/audits/CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md
handover/audits/GEMINI_WP_REVISION_AUDIT_2026-04-27.md
handover/audits/run_codex_b7_extra_prebatch_audit.sh
handover/audits/GEMINI_CO1_7_ROUND3_AUDIT_2026-04-28.md
handover/audits/GEMINI_PAPER1_V2_AUDIT_2026-04-24.md
handover/audits/EXT_GEMINI_PHASE_8_R1R8_2026-04-22.md
handover/audits/CODEX_CO1_1_4_PRE1_ROUND5_AUDIT_2026-04-28.md
handover/audits/run_codex_tb_1_audit_2026-04-29.sh
handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R13.md
handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R11.md
handover/audits/EXT_DEEPSEEK_2026-04-21.md
handover/audits/EXT_GEMINI_PHASE_8_BATCH_2026-04-22.md
handover/audits/run_codex_co1_1_4_pre1_round5_audit.sh
handover/audits/PPUT_RAW_DATA_2026-04-22.md
handover/audits/run_codex_co1_7_impl_bundle_round1_audit.sh
handover/audits/run_gemini_b7_extra_round3_audit.py
handover/audits/GEMINI_CO1_7_EXTRA_ROUND4_AUDIT_2026-04-29.md
handover/audits/run_gemini_whitepaper_v2_audit.py
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R7.md
handover/audits/run_codex_co1_1_4_pre1_round3_audit.sh
handover/audits/run_codex_co1_7_impl_bundle_round2_audit.sh
handover/audits/CO1_7_DUAL_AUDIT_VERDICT_R1_2026-04-28.md
handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R12.md
handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md
handover/audits/run_codex_co1_7_round3_audit.sh
handover/audits/CODEX_CO1_7_EXTRA_ROUND4_AUDIT_2026-04-29.md
handover/audits/run_codex_co1_7_extra_round3_audit.sh
handover/audits/run_gemini_co1_7_round1_audit.py
handover/audits/run_gemini_co1_7_round3_audit.py
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R8.md
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh
handover/audits/CODEX_TFR_PLAN_AUDIT_2026-04-26.md
handover/audits/GEMINI_PAPER1_V2_1_AUDIT_2026-04-25.md
handover/audits/CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md
handover/audits/run_gemini_co1_7_extra_round2_audit.py
handover/audits/run_codex_pput_ccl_audit.sh
handover/audits/run_codex_b7_extra_round3_audit.sh
handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md
handover/audits/GEMINI_SPEC_FREEZE_AUDIT_2026-04-27.md
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md
handover/audits/CODEX_PAPER1_V2_AUDIT_2026-04-24.md
handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md
handover/audits/run_gemini_b7_extra_prebatch_audit.py
handover/audits/CODEX_SPEC_V14_ROUND4_AUDIT_2026-04-27.md
handover/audits/run_gemini_b7_extra_round4_audit.py
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md
handover/audits/CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md
handover/audits/GEMINI_TB_1_AUDIT_2026-04-29.md
handover/audits/GEMINI_CO1_7_ROUND1_AUDIT_2026-04-28.md
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R13.md
handover/audits/GEMINI_B7_EXTRA_REAUDIT_2026-04-25.md
handover/audits/GEMINI_CO1_7_ROUND2_AUDIT_2026-04-28.md
handover/audits/GEMINI_WHITEPAPER_V2_AUDIT_2026-04-27_R2.md

codex
The Tier-A harness is green: 9 passed, 4 ignored. The h_vppu history unit tests are also green. I’m checking Day-2 economy unit coverage now, especially escrow, because it is absent from Tier-A.
exec
/bin/bash -lc "nl -ba src/economy/escrow_vault.rs | sed -n '138,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   138	// EscrowVault — task-keyed in-memory vault (RSP-0)
   139	// ────────────────────────────────────────────────────────────────────────────
   140	
   141	/// TRACE_MATRIX P3 RSP-0 — minimum-viable in-memory escrow vault.
   142	///
   143	/// One entry per `TaskId`. Locking is one-shot per task in RSP-0
   144	/// (multi-sponsor co-escrow is RSP-3). Release is one-shot and final.
   145	#[derive(Debug, Clone, Default)]
   146	pub struct EscrowVault {
   147	    entries: BTreeMap<TaskId, VaultEntry>,
   148	}
   149	
   150	impl EscrowVault {
   151	    /// TRACE_MATRIX P3 RSP-0 — empty vault constructor.
   152	    pub fn new() -> Self {
   153	        Self { entries: BTreeMap::new() }
   154	    }
   155	
   156	    /// TRACE_MATRIX P3 RSP-0 — number of recorded tasks (diagnostics-only accessor).
   157	    pub fn len(&self) -> usize {
   158	        self.entries.len()
   159	    }
   160	
   161	    /// TRACE_MATRIX P3 RSP-0 — emptiness predicate (diagnostics-only accessor).
   162	    pub fn is_empty(&self) -> bool {
   163	        self.entries.is_empty()
   164	    }
   165	
   166	    /// TRACE_MATRIX P3 RSP-0 — read-only entry lookup; backs residual / payout assertions.
   167	    pub fn get(&self, task_id: &TaskId) -> Option<&VaultEntry> {
   168	        self.entries.get(task_id)
   169	    }
   170	
   171	    /// TRACE_MATRIX P3:5 — record sponsor-locked supply for one task.
   172	    ///
   173	    /// Returns `EscrowError::AlreadyLocked` if a prior lock exists for the
   174	    /// same `task_id` (whether `Locked` or `Released`); a released task
   175	    /// cannot be re-locked under RSP-0 semantics.
   176	    /// Returns `EscrowError::NegativeAmount` for negative `amount`.
   177	    pub fn lock_escrow(
   178	        &mut self,
   179	        task_id: TaskId,
   180	        sponsor: AgentId,
   181	        amount: MicroCoin,
   182	    ) -> Result<EscrowReceipt, EscrowError> {
   183	        if amount.is_negative() {
   184	            return Err(EscrowError::NegativeAmount);
   185	        }
   186	        if self.entries.contains_key(&task_id) {
   187	            return Err(EscrowError::AlreadyLocked { task_id });
   188	        }
   189	        let entry = VaultEntry {
   190	            task_id: task_id.clone(),
   191	            sponsor: sponsor.clone(),
   192	            locked_amount: amount,
   193	            status: EscrowStatus::Locked,
   194	            payouts: BTreeMap::new(),
   195	            residual_to_sponsor: MicroCoin::zero(),
   196	        };
   197	        self.entries.insert(task_id.clone(), entry);
   198	        Ok(EscrowReceipt { task_id, sponsor, locked_amount: amount })
   199	    }
   200	
   201	    /// TRACE_MATRIX P3:6/P3:8 — distribute payouts, assert `Σ payouts ≤ locked`,
   202	    /// compute residual-to-sponsor, mark task `Released`.
   203	    ///
   204	    /// Empty `payouts` is permitted (full residual returns to sponsor — the
   205	    /// `TaskExpire` shape).
   206	    /// Negative payout amounts are rejected (`NegativeAmount`).
   207	    /// `Σ payouts > locked` → `PayoutExceedsLocked` and the entry is left
   208	    /// in `Locked` status (operation atomic-rejected).
   209	    pub fn release_escrow(
   210	        &mut self,
   211	        task_id: &TaskId,
   212	        payouts: &BTreeMap<AgentId, MicroCoin>,
   213	    ) -> Result<ReleaseOutcome, EscrowError> {
   214	        let entry = self
   215	            .entries
   216	            .get_mut(task_id)
   217	            .ok_or_else(|| EscrowError::NotFound { task_id: task_id.clone() })?;
   218	
   219	        if entry.status == EscrowStatus::Released {
   220	            return Err(EscrowError::AlreadyReleased { task_id: task_id.clone() });
   221	        }
   222	
   223	        let mut total_paid = MicroCoin::zero();
   224	        for amt in payouts.values() {
   225	            if amt.is_negative() {
   226	                return Err(EscrowError::NegativeAmount);
   227	            }
   228	            total_paid = total_paid
   229	                .checked_add(*amt)
   230	                .ok_or(EscrowError::Overflow)?;
   231	        }
   232	
   233	        if total_paid.micro_units() > entry.locked_amount.micro_units() {
   234	            return Err(EscrowError::PayoutExceedsLocked {
   235	                task_id: task_id.clone(),
   236	                locked_micro: entry.locked_amount.micro_units(),
   237	                requested_micro: total_paid.micro_units(),
   238	            });
   239	        }
   240	
   241	        let residual = entry
   242	            .locked_amount
   243	            .checked_sub(total_paid)
   244	            .ok_or(EscrowError::Overflow)?;
   245	
   246	        entry.payouts = payouts.clone();
   247	        entry.residual_to_sponsor = residual;
   248	        entry.status = EscrowStatus::Released;
   249	
   250	        Ok(ReleaseOutcome {
   251	            task_id: task_id.clone(),
   252	            sponsor: entry.sponsor.clone(),
   253	            paid_total: total_paid,
   254	            residual_to_sponsor: residual,
   255	        })
   256	    }
   257	}
   258	
   259	// ────────────────────────────────────────────────────────────────────────────
   260	// Tests

exec
/bin/bash -lc "nl -ba src/economy/escrow_vault.rs | sed -n '280,420p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   280	
   281	    #[test]
   282	    fn lock_records_entry_and_returns_receipt() {
   283	        let mut v = EscrowVault::new();
   284	        let r = v
   285	            .lock_escrow(task("t1"), agent("alice"), coin(100))
   286	            .unwrap();
   287	        assert_eq!(r.task_id, task("t1"));
   288	        assert_eq!(r.sponsor, agent("alice"));
   289	        assert_eq!(r.locked_amount, coin(100));
   290	        let e = v.get(&task("t1")).unwrap();
   291	        assert_eq!(e.status, EscrowStatus::Locked);
   292	        assert_eq!(e.locked_amount, coin(100));
   293	        assert_eq!(e.residual_to_sponsor, MicroCoin::zero());
   294	        assert!(e.payouts.is_empty());
   295	    }
   296	
   297	    #[test]
   298	    fn lock_rejects_double_lock_same_task() {
   299	        let mut v = EscrowVault::new();
   300	        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
   301	        let r = v.lock_escrow(task("t1"), agent("bob"), coin(50));
   302	        assert_eq!(r, Err(EscrowError::AlreadyLocked { task_id: task("t1") }));
   303	    }
   304	
   305	    #[test]
   306	    fn lock_rejects_negative_amount() {
   307	        let mut v = EscrowVault::new();
   308	        let r = v.lock_escrow(
   309	            task("t1"),
   310	            agent("alice"),
   311	            MicroCoin::from_micro_units(-1),
   312	        );
   313	        assert_eq!(r, Err(EscrowError::NegativeAmount));
   314	    }
   315	
   316	    // ── release ─────────────────────────────────────────────────────────────
   317	
   318	    #[test]
   319	    fn release_overpayout_rejected() {
   320	        // Charter Day-2 unit: "escrow over-payout rejected".
   321	        let mut v = EscrowVault::new();
   322	        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
   323	        let mut payouts = BTreeMap::new();
   324	        payouts.insert(agent("solver"), coin(60));
   325	        payouts.insert(agent("verifier"), coin(50));
   326	        let r = v.release_escrow(&task("t1"), &payouts);
   327	        assert!(matches!(r, Err(EscrowError::PayoutExceedsLocked { .. })));
   328	        // Atomic rejection: entry still Locked, no payouts recorded.
   329	        let e = v.get(&task("t1")).unwrap();
   330	        assert_eq!(e.status, EscrowStatus::Locked);
   331	        assert!(e.payouts.is_empty());
   332	    }
   333	
   334	    #[test]
   335	    fn release_underpayout_residual_returns_to_sponsor() {
   336	        // Charter Day-2 unit: "escrow under-payout accepted (residual returns to sponsor)".
   337	        let mut v = EscrowVault::new();
   338	        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
   339	        let mut payouts = BTreeMap::new();
   340	        payouts.insert(agent("solver"), coin(60));
   341	        payouts.insert(agent("verifier"), coin(10));
   342	        let outcome = v.release_escrow(&task("t1"), &payouts).unwrap();
   343	        assert_eq!(outcome.paid_total, coin(70));
   344	        assert_eq!(outcome.residual_to_sponsor, coin(30));
   345	        assert_eq!(outcome.sponsor, agent("alice"));
   346	        let e = v.get(&task("t1")).unwrap();
   347	        assert_eq!(e.status, EscrowStatus::Released);
   348	        assert_eq!(e.residual_to_sponsor, coin(30));
   349	        assert_eq!(e.payouts.len(), 2);
   350	    }
   351	
   352	    #[test]
   353	    fn release_exact_payout_zero_residual() {
   354	        let mut v = EscrowVault::new();
   355	        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
   356	        let mut payouts = BTreeMap::new();
   357	        payouts.insert(agent("solver"), coin(70));
   358	        payouts.insert(agent("verifier"), coin(30));
   359	        let outcome = v.release_escrow(&task("t1"), &payouts).unwrap();
   360	        assert_eq!(outcome.paid_total, coin(100));
   361	        assert_eq!(outcome.residual_to_sponsor, MicroCoin::zero());
   362	    }
   363	
   364	    #[test]
   365	    fn release_empty_payouts_full_residual_to_sponsor() {
   366	        // TaskExpire shape: deadline lapsed, no winners; full bounty refunds.
   367	        let mut v = EscrowVault::new();
   368	        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
   369	        let payouts = BTreeMap::new();
   370	        let outcome = v.release_escrow(&task("t1"), &payouts).unwrap();
   371	        assert_eq!(outcome.paid_total, MicroCoin::zero());
   372	        assert_eq!(outcome.residual_to_sponsor, coin(100));
   373	    }
   374	
   375	    #[test]
   376	    fn release_unknown_task_rejected() {
   377	        let mut v = EscrowVault::new();
   378	        let r = v.release_escrow(&task("t1"), &BTreeMap::new());
   379	        assert_eq!(r, Err(EscrowError::NotFound { task_id: task("t1") }));
   380	    }
   381	
   382	    #[test]
   383	    fn release_after_release_rejected() {
   384	        let mut v = EscrowVault::new();
   385	        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
   386	        v.release_escrow(&task("t1"), &BTreeMap::new()).unwrap();
   387	        let r = v.release_escrow(&task("t1"), &BTreeMap::new());
   388	        assert_eq!(r, Err(EscrowError::AlreadyReleased { task_id: task("t1") }));
   389	    }
   390	
   391	    #[test]
   392	    fn release_negative_payout_rejected() {
   393	        let mut v = EscrowVault::new();
   394	        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
   395	        let mut payouts = BTreeMap::new();
   396	        payouts.insert(agent("solver"), MicroCoin::from_micro_units(-1));
   397	        let r = v.release_escrow(&task("t1"), &payouts);
   398	        assert_eq!(r, Err(EscrowError::NegativeAmount));
   399	        // Atomic-reject: still Locked.
   400	        assert_eq!(v.get(&task("t1")).unwrap().status, EscrowStatus::Locked);
   401	    }
   402	
   403	    // ── multi-task isolation ────────────────────────────────────────────────
   404	
   405	    #[test]
   406	    fn multi_task_independent() {
   407	        let mut v = EscrowVault::new();
   408	        v.lock_escrow(task("t1"), agent("alice"), coin(100)).unwrap();
   409	        v.lock_escrow(task("t2"), agent("bob"), coin(50)).unwrap();
   410	        assert_eq!(v.len(), 2);
   411	        let mut p1 = BTreeMap::new();
   412	        p1.insert(agent("solver1"), coin(40));
   413	        v.release_escrow(&task("t1"), &p1).unwrap();
   414	        // t2 must still be locked.
   415	        assert_eq!(v.get(&task("t2")).unwrap().status, EscrowStatus::Locked);
   416	        assert_eq!(v.get(&task("t1")).unwrap().status, EscrowStatus::Released);
   417	    }
   418	}

exec
/bin/bash -lc 'cargo test -p turingosv4 economy::escrow_vault' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'cargo test -p turingosv4 economy::monetary_invariant::tests::ctf_counts_all_six_holding_subindexes' in /home/zephryj/projects/turingosv4
 succeeded in 9578ms:
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:13:13
   |
13 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::HashSet`
 --> src/sdk/actor.rs:9:5
  |
9 | use std::collections::HashSet;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused imports: `MICRO_PER_COIN` and `MicroCoin`
  --> src/economy/monetary_invariant.rs:26:29
   |
26 | use crate::economy::money::{MicroCoin, MICRO_PER_COIN};
   |                             ^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/cas/store.rs:26:12
   |
26 | use git2::{ObjectType as Git2ObjectType, Repository};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/ledger/transition_ledger.rs:36:12
   |
36 | use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_rejected_attempt_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:549:19
    |
549 |     pub(crate) fn sign_rejected_attempt_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:560:19
    |
560 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: function `sign_terminal_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:586:19
    |
586 |     pub(crate) fn sign_terminal_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_finalize_reward` is never used
   --> src/bottom_white/ledger/system_keypair.rs:597:19
    |
597 |     pub(crate) fn sign_finalize_reward(
    |                   ^^^^^^^^^^^^^^^^^^^^

warning: function `sign_task_expire` is never used
   --> src/bottom_white/ledger/system_keypair.rs:608:19
    |
608 |     pub(crate) fn sign_task_expire(
    |                   ^^^^^^^^^^^^^^^^

warning: function `sign_epoch_rotation_proof` is never used
   --> src/bottom_white/ledger/system_keypair.rs:616:19
    |
616 |     pub(crate) fn sign_epoch_rotation_proof(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:627:19
    |
627 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib) generated 19 warnings (run `cargo fix --lib -p turingosv4` to apply 12 suggestions)
   Compiling turingosv4 v0.1.0 (/home/zephryj/projects/turingosv4)
warning: useless assignment of field of type `turingosv4::state::Hash` to itself
  --> tests/six_axioms_alignment.rs:90:5
   |
90 |     q.predicate_registry_root_t = q.predicate_registry_root_t; // axiom 2
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: useless assignment of field of type `turingosv4::state::Hash` to itself
  --> tests/six_axioms_alignment.rs:95:5
   |
95 |     q.tool_registry_root_t = q.tool_registry_root_t; // axiom 4
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `MICRO_PER_COIN`
  --> tests/walkthrough_inv3_conservation.rs:18:45
   |
18 | use turingosv4::economy::money::{MicroCoin, MICRO_PER_COIN};
   |                                             ^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: field `royalty_edges` is never read
  --> tests/walkthrough_inv3_conservation.rs:26:5
   |
22 | struct EconStateMinimal {
   |        ---------------- field in this struct
...
26 |     royalty_edges: Vec<(String, String, i64)>,     // (creator, beneficiary_work_tx, weight_micro)
   |     ^^^^^^^^^^^^^
   |
   = note: `EconStateMinimal` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: variable does not need to be mutable
   --> src/top_white/predicates/registry.rs:255:17
    |
255 |             let mut h = Sha256::new();
    |                 ----^
    |                 |
    |                 help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: function `task` is never used
   --> src/economy/monetary_invariant.rs:227:8
    |
227 |     fn task(s: &str) -> TaskId {
    |        ^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (test "six_axioms_alignment") generated 2 warnings
warning: `turingosv4` (test "walkthrough_inv3_conservation") generated 2 warnings (run `cargo fix --test "walkthrough_inv3_conservation" -p turingosv4` to apply 1 suggestion)
warning: `turingosv4` (lib test) generated 17 warnings (15 duplicates) (run `cargo fix --lib -p turingosv4 --tests` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 9.40s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-337b37021c2192ab)

running 11 tests
test economy::escrow_vault::tests::lock_rejects_double_lock_same_task ... ok
test economy::escrow_vault::tests::multi_task_independent ... ok
test economy::escrow_vault::tests::lock_records_entry_and_returns_receipt ... ok
test economy::escrow_vault::tests::lock_rejects_negative_amount ... ok
test economy::escrow_vault::tests::release_after_release_rejected ... ok
test economy::escrow_vault::tests::release_negative_payout_rejected ... ok
test economy::escrow_vault::tests::release_empty_payouts_full_residual_to_sponsor ... ok
test economy::escrow_vault::tests::release_exact_payout_zero_residual ... ok
test economy::escrow_vault::tests::release_underpayout_residual_returns_to_sponsor ... ok
test economy::escrow_vault::tests::release_unknown_task_rejected ... ok
test economy::escrow_vault::tests::release_overpayout_rejected ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 259 filtered out; finished in 0.01s

     Running unittests src/main.rs (target/debug/deps/turingosv4-c93a21f17f9da5e3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/co1_7_extra_cas_payload_round_trip.rs (target/debug/deps/co1_7_extra_cas_payload_round_trip-dd02ebe65ae314d3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/co1_7_extra_git2_writer_head_oid_defense.rs (target/debug/deps/co1_7_extra_git2_writer_head_oid_defense-8f24f337d31cde76)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/co1_7_extra_sequencer_head_t_advancement.rs (target/debug/deps/co1_7_extra_sequencer_head_t_advancement-dedb149f85249bd2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/conformance_stubs.rs (target/debug/deps/conformance_stubs-838f70d88220d2ca)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 117 filtered out; finished in 0.00s

     Running tests/economic_state_reconstruct.rs (target/debug/deps/economic_state_reconstruct-dcd9523dc1aa1566)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/fc_alignment_conformance.rs (target/debug/deps/fc_alignment_conformance-14dd2473f73dd408)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/four_element_mapping.rs (target/debug/deps/four_element_mapping-5b315f6baf23d713)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/q_state_reconstruct.rs (target/debug/deps/q_state_reconstruct-af1d0ad4dcee8ec5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/r_022_integration_orchestrator.rs (target/debug/deps/r_022_integration_orchestrator-3a0e788a04afa895)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/reward_pull_conservation.rs (target/debug/deps/reward_pull_conservation-90fc50cc5689d0e3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/six_axioms_alignment.rs (target/debug/deps/six_axioms_alignment-a70e5d8781d5c342)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/system_keypair_generation.rs (target/debug/deps/system_keypair_generation-ec212f6e09008269)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/system_keypair_load_and_decrypt.rs (target/debug/deps/system_keypair_load_and_decrypt-aba048fc986f1726)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/system_keypair_rotation_proof.rs (target/debug/deps/system_keypair_rotation_proof-3a4c5ca7a4e54ff0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/system_keypair_sign_only_from_runner.rs (target/debug/deps/system_keypair_sign_only_from_runner-30f12ce13eb682f5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/system_keypair_verify_correctness.rs (target/debug/deps/system_keypair_verify_correctness-2ad7e8bc77293838)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tb_1_acceptance.rs (target/debug/deps/tb_1_acceptance-9b2fe0f6ff43f57a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/wal_resume.rs (target/debug/deps/wal_resume-74c341ba32faa63d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/walkthrough_inv3_conservation.rs (target/debug/deps/walkthrough_inv3_conservation-b920165ef7c34a29)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s


 succeeded in 9599ms:
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on artifact directory
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:13:13
   |
13 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::HashSet`
 --> src/sdk/actor.rs:9:5
  |
9 | use std::collections::HashSet;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused imports: `MICRO_PER_COIN` and `MicroCoin`
  --> src/economy/monetary_invariant.rs:26:29
   |
26 | use crate::economy::money::{MicroCoin, MICRO_PER_COIN};
   |                             ^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/cas/store.rs:26:12
   |
26 | use git2::{ObjectType as Git2ObjectType, Repository};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/ledger/transition_ledger.rs:36:12
   |
36 | use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_rejected_attempt_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:549:19
    |
549 |     pub(crate) fn sign_rejected_attempt_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:560:19
    |
560 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: function `sign_terminal_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:586:19
    |
586 |     pub(crate) fn sign_terminal_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_finalize_reward` is never used
   --> src/bottom_white/ledger/system_keypair.rs:597:19
    |
597 |     pub(crate) fn sign_finalize_reward(
    |                   ^^^^^^^^^^^^^^^^^^^^

warning: function `sign_task_expire` is never used
   --> src/bottom_white/ledger/system_keypair.rs:608:19
    |
608 |     pub(crate) fn sign_task_expire(
    |                   ^^^^^^^^^^^^^^^^

warning: function `sign_epoch_rotation_proof` is never used
   --> src/bottom_white/ledger/system_keypair.rs:616:19
    |
616 |     pub(crate) fn sign_epoch_rotation_proof(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:627:19
    |
627 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib) generated 19 warnings (run `cargo fix --lib -p turingosv4` to apply 12 suggestions)
warning: useless assignment of field of type `turingosv4::state::Hash` to itself
  --> tests/six_axioms_alignment.rs:90:5
   |
90 |     q.predicate_registry_root_t = q.predicate_registry_root_t; // axiom 2
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: useless assignment of field of type `turingosv4::state::Hash` to itself
  --> tests/six_axioms_alignment.rs:95:5
   |
95 |     q.tool_registry_root_t = q.tool_registry_root_t; // axiom 4
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (test "six_axioms_alignment") generated 2 warnings
warning: variable does not need to be mutable
   --> src/top_white/predicates/registry.rs:255:17
    |
255 |             let mut h = Sha256::new();
    |                 ----^
    |                 |
    |                 help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: function `task` is never used
   --> src/economy/monetary_invariant.rs:227:8
    |
227 |     fn task(s: &str) -> TaskId {
    |        ^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (lib test) generated 17 warnings (15 duplicates) (run `cargo fix --lib -p turingosv4 --tests` to apply 1 suggestion)
warning: unused import: `MICRO_PER_COIN`
  --> tests/walkthrough_inv3_conservation.rs:18:45
   |
18 | use turingosv4::economy::money::{MicroCoin, MICRO_PER_COIN};
   |                                             ^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: field `royalty_edges` is never read
  --> tests/walkthrough_inv3_conservation.rs:26:5
   |
22 | struct EconStateMinimal {
   |        ---------------- field in this struct
...
26 |     royalty_edges: Vec<(String, String, i64)>,     // (creator, beneficiary_work_tx, weight_micro)
   |     ^^^^^^^^^^^^^
   |
   = note: `EconStateMinimal` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (test "walkthrough_inv3_conservation") generated 2 warnings (run `cargo fix --test "walkthrough_inv3_conservation" -p turingosv4` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 9.47s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-337b37021c2192ab)

running 1 test
test economy::monetary_invariant::tests::ctf_counts_all_six_holding_subindexes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 269 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/turingosv4-c93a21f17f9da5e3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/co1_7_extra_cas_payload_round_trip.rs (target/debug/deps/co1_7_extra_cas_payload_round_trip-dd02ebe65ae314d3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/co1_7_extra_git2_writer_head_oid_defense.rs (target/debug/deps/co1_7_extra_git2_writer_head_oid_defense-8f24f337d31cde76)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/co1_7_extra_sequencer_head_t_advancement.rs (target/debug/deps/co1_7_extra_sequencer_head_t_advancement-dedb149f85249bd2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/conformance_stubs.rs (target/debug/deps/conformance_stubs-838f70d88220d2ca)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 117 filtered out; finished in 0.00s

     Running tests/economic_state_reconstruct.rs (target/debug/deps/economic_state_reconstruct-dcd9523dc1aa1566)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/fc_alignment_conformance.rs (target/debug/deps/fc_alignment_conformance-14dd2473f73dd408)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/four_element_mapping.rs (target/debug/deps/four_element_mapping-5b315f6baf23d713)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/q_state_reconstruct.rs (target/debug/deps/q_state_reconstruct-af1d0ad4dcee8ec5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/r_022_integration_orchestrator.rs (target/debug/deps/r_022_integration_orchestrator-3a0e788a04afa895)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/reward_pull_conservation.rs (target/debug/deps/reward_pull_conservation-90fc50cc5689d0e3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/six_axioms_alignment.rs (target/debug/deps/six_axioms_alignment-a70e5d8781d5c342)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/system_keypair_generation.rs (target/debug/deps/system_keypair_generation-ec212f6e09008269)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/system_keypair_load_and_decrypt.rs (target/debug/deps/system_keypair_load_and_decrypt-aba048fc986f1726)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/system_keypair_rotation_proof.rs (target/debug/deps/system_keypair_rotation_proof-3a4c5ca7a4e54ff0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/system_keypair_sign_only_from_runner.rs (target/debug/deps/system_keypair_sign_only_from_runner-30f12ce13eb682f5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/system_keypair_verify_correctness.rs (target/debug/deps/system_keypair_verify_correctness-2ad7e8bc77293838)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tb_1_acceptance.rs (target/debug/deps/tb_1_acceptance-9b2fe0f6ff43f57a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/wal_resume.rs (target/debug/deps/wal_resume-74c341ba32faa63d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/walkthrough_inv3_conservation.rs (target/debug/deps/walkthrough_inv3_conservation-b920165ef7c34a29)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s


exec
/bin/bash -lc "rg -n \"AcceptedLedger|append_accepted|economy::ledger\" -S src tests experiments --glob '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/economy/ledger.rs:20://! - `append_accepted` advances `logical_t` and chains `prev_hash`.
src/economy/ledger.rs:150:// AcceptedLedger — the wrapper itself
src/economy/ledger.rs:159:pub struct AcceptedLedger {
src/economy/ledger.rs:164:impl AcceptedLedger {
src/economy/ledger.rs:189:    pub fn append_accepted(&mut self, tx: &TypedTx) -> Result<AcceptedEntry, LedgerError> {
src/economy/ledger.rs:407:        let mut l = AcceptedLedger::new();
src/economy/ledger.rs:411:        let e1 = l.append_accepted(&fixture_work_tx(1)).unwrap();
src/economy/ledger.rs:418:        let e2 = l.append_accepted(&fixture_work_tx(2)).unwrap();
src/economy/ledger.rs:426:        let mut l = AcceptedLedger::new();
src/economy/ledger.rs:428:            l.append_accepted(&fixture_work_tx(i)).unwrap();
src/economy/ledger.rs:437:        let mut l = AcceptedLedger::new();
src/economy/ledger.rs:438:        l.append_accepted(&fixture_work_tx(1)).unwrap();
src/economy/ledger.rs:445:        let mut l = AcceptedLedger::new();
src/economy/ledger.rs:447:            l.append_accepted(&fixture_work_tx(i)).unwrap();
src/economy/ledger.rs:456:        let mut l = AcceptedLedger::new();
src/economy/ledger.rs:458:            l.append_accepted(&fixture_work_tx(i)).unwrap();
src/economy/ledger.rs:464:        let (l2, post) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
tests/tb_1_acceptance.rs:38:use turingosv4::economy::ledger::{AcceptedLedger, LedgerError};
tests/tb_1_acceptance.rs:115:    let mut l = AcceptedLedger::new();
tests/tb_1_acceptance.rs:117:        l.append_accepted(&fixture_work_tx(i)).unwrap();
tests/tb_1_acceptance.rs:126:    let mut tampered: Vec<turingosv4::economy::ledger::AcceptedEntry> =
tests/tb_1_acceptance.rs:133:    match AcceptedLedger::load_from_path(tmp.path()) {
tests/tb_1_acceptance.rs:148:    let mut l4 = AcceptedLedger::new();
tests/tb_1_acceptance.rs:151:    l4.append_accepted(&fixture_work_tx(1)).unwrap();
tests/tb_1_acceptance.rs:190:    let mut l = AcceptedLedger::new();
tests/tb_1_acceptance.rs:192:        l.append_accepted(&fixture_work_tx(i)).unwrap();
tests/tb_1_acceptance.rs:201:    let (l_reborn, reconstructed_root) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
tests/tb_1_acceptance.rs:254:    let mut l = AcceptedLedger::new();
tests/tb_1_acceptance.rs:256:        l.append_accepted(&fixture_work_tx(i)).unwrap();
tests/tb_1_acceptance.rs:423:    let l4 = AcceptedLedger::new();
tests/tb_1_acceptance.rs:475:    // When TB-2 RSP-1 wires the WorkTx → AcceptedLedger::append_accepted path,

exec
/bin/bash -lc "rg -n \"RejectionEvidenceWriter|append_rejected|public_view\" -S src tests experiments --glob '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/ledger/rejection_evidence.rs:187:/// TRACE_MATRIX P1:6 — error returned by `RejectionEvidenceWriter::verify_chain`.
src/bottom_white/ledger/rejection_evidence.rs:206:// RejectionEvidenceWriter — append + verify + project-to-public
src/bottom_white/ledger/rejection_evidence.rs:217:pub struct RejectionEvidenceWriter {
src/bottom_white/ledger/rejection_evidence.rs:221:impl RejectionEvidenceWriter {
src/bottom_white/ledger/rejection_evidence.rs:248:    pub fn append_rejected(
src/bottom_white/ledger/rejection_evidence.rs:319:    /// agent boundary; use `public_view` for that).
src/bottom_white/ledger/rejection_evidence.rs:329:    pub fn public_view(&self) -> Vec<PublicRejectionView> {
src/bottom_white/ledger/rejection_evidence.rs:362:        let mut w = RejectionEvidenceWriter::new();
src/bottom_white/ledger/rejection_evidence.rs:363:        let h1 = w.append_rejected(
src/bottom_white/ledger/rejection_evidence.rs:373:        let h2 = w.append_rejected(
src/bottom_white/ledger/rejection_evidence.rs:392:    fn public_view_omits_raw_diagnostic_cid() {
src/bottom_white/ledger/rejection_evidence.rs:393:        let mut w = RejectionEvidenceWriter::new();
src/bottom_white/ledger/rejection_evidence.rs:394:        w.append_rejected(
src/bottom_white/ledger/rejection_evidence.rs:404:        let view = w.public_view();
src/bottom_white/ledger/rejection_evidence.rs:417:        let mut w = RejectionEvidenceWriter::new();
src/bottom_white/ledger/rejection_evidence.rs:418:        w.append_rejected(
src/bottom_white/ledger/rejection_evidence.rs:428:        w.append_rejected(
tests/tb_1_acceptance.rs:35:    PublicRejectionView, RejectionClass, RejectionEvidenceError, RejectionEvidenceWriter,
tests/tb_1_acceptance.rs:149:    let mut l4e = RejectionEvidenceWriter::new();
tests/tb_1_acceptance.rs:155:    l4e.append_rejected(
tests/tb_1_acceptance.rs:216:    let mut l4e = RejectionEvidenceWriter::new();
tests/tb_1_acceptance.rs:217:    l4e.append_rejected(
tests/tb_1_acceptance.rs:228:    let view: Vec<PublicRejectionView> = l4e.public_view();
tests/tb_1_acceptance.rs:279:    let mut l4e = RejectionEvidenceWriter::new();
tests/tb_1_acceptance.rs:281:        l4e.append_rejected(
tests/tb_1_acceptance.rs:427:    let mut l4e = RejectionEvidenceWriter::new();
tests/tb_1_acceptance.rs:428:    l4e.append_rejected(

exec
/bin/bash -lc "rg -n \"EscrowVault|monetary_invariant|assert_total_ctf_conserved|assert_read_is_free|assert_no_post_init_mint\" -S src tests experiments --glob '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/economy/mod.rs:13:pub mod monetary_invariant;
src/economy/escrow_vault.rs:66:/// TRACE_MATRIX P3:5 — receipt returned by [`EscrowVault::lock_escrow`].
src/economy/escrow_vault.rs:74:/// TRACE_MATRIX P3:6/P3:8 — outcome returned by [`EscrowVault::release_escrow`].
src/economy/escrow_vault.rs:91:/// TRACE_MATRIX P3 RSP-0 — errors returned by `EscrowVault` operations.
src/economy/escrow_vault.rs:138:// EscrowVault — task-keyed in-memory vault (RSP-0)
src/economy/escrow_vault.rs:146:pub struct EscrowVault {
src/economy/escrow_vault.rs:150:impl EscrowVault {
src/economy/escrow_vault.rs:283:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:299:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:307:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:321:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:337:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:354:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:367:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:377:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:384:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:393:        let mut v = EscrowVault::new();
src/economy/escrow_vault.rs:407:        let mut v = EscrowVault::new();
src/economy/monetary_invariant.rs:42:    /// [`assert_total_ctf_conserved`] when `delta_micro > 0` and no
src/economy/monetary_invariant.rs:118:// assert_no_post_init_mint — structural guard at the tx layer
src/economy/monetary_invariant.rs:133:/// [`assert_total_ctf_conserved`] separately.
src/economy/monetary_invariant.rs:134:pub fn assert_no_post_init_mint(tx: &TypedTx, q: &QState) -> Result<(), MonetaryError> {
src/economy/monetary_invariant.rs:151:// assert_total_ctf_conserved — numeric conservation across a transition
src/economy/monetary_invariant.rs:167:pub fn assert_total_ctf_conserved(
src/economy/monetary_invariant.rs:190:// assert_read_is_free — tx-level no-fee guard
src/economy/monetary_invariant.rs:202:pub fn assert_read_is_free(tx_kind: TxKind, fee: u64) -> Result<(), MonetaryError> {
src/economy/monetary_invariant.rs:252:    // ── assert_no_post_init_mint ────────────────────────────────────────────
src/economy/monetary_invariant.rs:258:        assert_eq!(assert_no_post_init_mint(&work, &q), Ok(()));
src/economy/monetary_invariant.rs:277:            assert_eq!(assert_no_post_init_mint(&t, &q), Ok(()),
src/economy/monetary_invariant.rs:282:    // ── assert_total_ctf_conserved ──────────────────────────────────────────
src/economy/monetary_invariant.rs:292:        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
src/economy/monetary_invariant.rs:301:        let r = assert_total_ctf_conserved(&before, &after, &[]);
src/economy/monetary_invariant.rs:313:        let r = assert_total_ctf_conserved(&before, &after, &[]);
src/economy/monetary_invariant.rs:326:            assert_total_ctf_conserved(&before, &after, &[TxKind::FinalizeReward]),
src/economy/monetary_invariant.rs:342:        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
src/economy/monetary_invariant.rs:438:    // ── assert_read_is_free ─────────────────────────────────────────────────
src/economy/monetary_invariant.rs:451:            assert_eq!(assert_read_is_free(k, 0), Ok(()));
src/economy/monetary_invariant.rs:458:        let r = assert_read_is_free(TxKind::Reuse, 1);
src/economy/monetary_invariant.rs:460:        let r = assert_read_is_free(TxKind::Work, 9999);
src/bottom_white/ledger/rejection_evidence.rs:63:    /// `monetary_invariant` (Inv 4 / 基本法 1) flagged a conservation break.
src/state/q_state.rs:159:/// TRACE_MATRIX WP § 2 — tx → escrow entry. Full schema lands CO P2.2 EscrowVault.
tests/tb_1_acceptance.rs:40:use turingosv4::economy::monetary_invariant::{
tests/tb_1_acceptance.rs:41:    assert_read_is_free, assert_total_ctf_conserved, MonetaryError,
tests/tb_1_acceptance.rs:311:    // a 5-step alice/bob/escrow shuffle and assert assert_total_ctf_conserved
tests/tb_1_acceptance.rs:321:    assert_eq!(assert_total_ctf_conserved(&baseline, &s1, &[]), Ok(()));
tests/tb_1_acceptance.rs:331:    assert_eq!(assert_total_ctf_conserved(&s1, &s2, &[]), Ok(()));
tests/tb_1_acceptance.rs:337:    assert_eq!(assert_total_ctf_conserved(&s2, &s3, &[]), Ok(()));
tests/tb_1_acceptance.rs:343:    assert_eq!(assert_total_ctf_conserved(&s3, &s4, &[]), Ok(()));
tests/tb_1_acceptance.rs:348:    assert_eq!(assert_total_ctf_conserved(&s4, &s5, &[]), Ok(()));
tests/tb_1_acceptance.rs:365:    // against ALL TxKinds. assert_read_is_free(kind, fee=0) must succeed for
tests/tb_1_acceptance.rs:377:            assert_read_is_free(kind, 0),
tests/tb_1_acceptance.rs:386:        assert_read_is_free(TxKind::Reuse, 1),
tests/tb_1_acceptance.rs:393:        assert_read_is_free(TxKind::Work, 9999),
tests/tb_1_acceptance.rs:411:    let r = assert_total_ctf_conserved(&before, &after, &[]);

exec
/bin/bash -lc "rg -n \"state\\.db|chaintape|state_path|reconstruct_state|load_from_path\" -S src tests handover/tracer_bullets/TB-1_recharter_2026-04-29.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/tracer_bullets/TB-1_recharter_2026-04-29.md:24:> 3. (P1 Exit 8) state.db can be reconstructed from chaintape.jsonl;
handover/tracer_bullets/TB-1_recharter_2026-04-29.md:59:**Exit addressed**: P1:5 (state_root advances on accept), P1:6 (state_root unchanged on reject), P1:7 (ledger hash chain), P1:8 (state.db reconstruction), P1:9 (rejected-log isolation)
handover/tracer_bullets/TB-1_recharter_2026-04-29.md:60:**Kill tested**: P1:1 (no wtool bypass), P1:2 (rejected tx ≠ state_root advance), P1:3 (state.db reconstructable), P1:4 (no read-view pollution)
handover/tracer_bullets/TB-1_recharter_2026-04-29.md:66:  - `pub fn reconstruct_state(state_path: &Path) -> Result<QState, ReconstructError>` — replays L4 only (L4.E NOT consulted; rejections must not affect `state_root`).
handover/tracer_bullets/TB-1_recharter_2026-04-29.md:74:  - `test_p1_kill_1_no_wtool_bypass`: any direct mutation to state.db without going through wtool→L4 panics or fails to round-trip via `reconstruct_state`.
handover/tracer_bullets/TB-1_recharter_2026-04-29.md:76:  - `test_p1_kill_3_ledger_reconstructable`: drop state.db; reconstruct from L4 only; bit-equal to pre-drop `state_root`. L4.E intentionally not consulted in reconstruction.
handover/tracer_bullets/TB-1_recharter_2026-04-29.md:116:3. **(P1 kill 3)** `test_p1_kill_3_ledger_reconstructable` — drop state.db; reconstruct from L4 only; bit-equal pre-drop `state_root`.
src/economy/escrow_vault.rs:25://! - Persistence to L4 / state.db (TB-1 Day-3 wiring).
src/economy/ledger.rs:6://!   unchanged on reject), Exit 7 (ledger hash chain), Exit 8 (state.db
src/economy/ledger.rs:7://!   reconstructable from chaintape).
src/economy/ledger.rs:13://! - WP § 5.L4 — ChainTape Layer 4 spine; one entry per accepted transition.
src/economy/ledger.rs:22://! - `reconstruct_state` replays L4 only and returns the canonical
src/economy/ledger.rs:24://! - Persistence helpers (`persist` / `load_from_path`) provide the
src/economy/ledger.rs:25://!   "drop state.db; reconstruct from L4" round-trip used by P1 kill
src/economy/ledger.rs:145:/// TRACE_MATRIX P1:8 — `reconstruct_state` / `load_from_path` failure alias;
src/economy/ledger.rs:275:    pub fn reconstruct_state(&self) -> Result<Hash, ReconstructError> {
src/economy/ledger.rs:290:    /// TRACE_MATRIX P1:8 — persist entries to `state_path` for cold restart.
src/economy/ledger.rs:291:    pub fn persist(&self, state_path: &Path) -> Result<(), LedgerError> {
src/economy/ledger.rs:293:        std::fs::write(state_path, bytes).map_err(|e| LedgerError::Io(e.to_string()))?;
src/economy/ledger.rs:297:    /// TRACE_MATRIX P1:8 — load entries from `state_path` and recompute the
src/economy/ledger.rs:298:    /// canonical `state_root_t`. Used by the "drop state.db; reconstruct from L4"
src/economy/ledger.rs:300:    pub fn load_from_path(state_path: &Path) -> Result<(Self, Hash), ReconstructError> {
src/economy/ledger.rs:301:        let bytes = std::fs::read(state_path).map_err(|e| LedgerError::Io(e.to_string()))?;
src/economy/ledger.rs:308:        let s = l.reconstruct_state()?;
src/economy/ledger.rs:444:    fn reconstruct_state_round_trip() {
src/economy/ledger.rs:450:        let reconstructed = l.reconstruct_state().unwrap();
src/economy/ledger.rs:464:        let (l2, post) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
src/state/q_state.rs:310:    /// Current ChainTape head pointer = git commit SHA in Path B substrate.
src/bottom_white/ledger/transition_ledger.rs:4://! TRACE_MATRIX WP § 5.L4: ChainTape Layer 4 spine; one LedgerEntry per accepted transition.
tests/tb_1_acceptance.rs:124:    // Bypass: directly overwrite state.db without going through L4.
tests/tb_1_acceptance.rs:133:    match AcceptedLedger::load_from_path(tmp.path()) {
tests/tb_1_acceptance.rs:137:            "bypass mutation must not survive a round-trip through reconstruct_state"
tests/tb_1_acceptance.rs:201:    let (l_reborn, reconstructed_root) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
tests/conformance_stubs.rs:64:// ChainTape layers (CO1.0-1.9)

codex
**Section A: Verdict**

CHALLENGE, high conviction. The 9 Tier-A tests are green, but they prove the new primitives, not the full central claim. A malicious implementer can still pass all 9 while bypassing real dispatch routing, monetary guard call sites, and raw rejection-record shielding.

**Section B: Q1-Q8**

Q1. The exact “silent rebase to canonical” bypass would not pass: `test_p1_kill_1_no_wtool_bypass` expects `Err` or `reconstructed != canonical_root` [tests/tb_1_acceptance.rs:132](/home/zephryj/projects/turingosv4/tests/tb_1_acceptance.rs:132). But the test mutates the serialized L4 entries, not an independent `state.db`, and it treats `Ok(divergent_root)` as acceptable. Also `load_from_path` calls `reconstruct_state` only, not `verify_chain` [src/economy/ledger.rs:300](/home/zephryj/projects/turingosv4/src/economy/ledger.rs:300). Simple fake-genesis/reorder/parent-state-root tamper is caught by replay or `verify_chain`, but `prev_hash`/`hash`/`logical_t`-only tamper can load successfully unless the caller separately verifies.

Q2. No, L4/L4.E disjointness is not actually tested through a real route. Tests manually instantiate `AcceptedLedger` and `RejectionEvidenceWriter` and append only to L4.E [tests/tb_1_acceptance.rs:147](/home/zephryj/projects/turingosv4/tests/tb_1_acceptance.rs:147). Production `dispatch_transition` is still all-variants `NotYetImplemented` [src/state/sequencer.rs:47](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:47), and `apply_one` early-returns on transition error before any L4.E append [src/state/sequencer.rs:339](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:339). TB-1 must say “primitives only”; enforcement is TB-2.

Q3. The isolation test is only a projection test. `PublicRejectionView` omits `raw_diagnostic_cid` [src/bottom_white/ledger/rejection_evidence.rs:160](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/rejection_evidence.rs:160), but `RejectedSubmissionRecord` is public, derives `Serialize`, and exposes `pub raw_diagnostic_cid` [src/bottom_white/ledger/rejection_evidence.rs:82](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/rejection_evidence.rs:82). `records()` returns full raw records [src/bottom_white/ledger/rejection_evidence.rs:320](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/rejection_evidence.rs:320). Grep found no current production direct serialization, but the structural guard is convention, not enforced.

Q4. Tier-A’s 5-step redistribution touches balances and escrows only [tests/tb_1_acceptance.rs:307](/home/zephryj/projects/turingosv4/tests/tb_1_acceptance.rs:307). The implementation does sum all six subindices [src/economy/monetary_invariant.rs:94](/home/zephryj/projects/turingosv4/src/economy/monetary_invariant.rs:94), and a unit test covers all six [src/economy/monetary_invariant.rs:403](/home/zephryj/projects/turingosv4/src/economy/monetary_invariant.rs:403). But that is not one of the 9 Tier-A tests. Constant-zero supply would be caught by Tier-A post-init mint [tests/tb_1_acceptance.rs:405](/home/zephryj/projects/turingosv4/tests/tb_1_acceptance.rs:405), but undercounting claims/stakes/task bounty/challenge bond can pass Tier-A.

Q5. The monetary guards are not wired into a real dispatch/settlement path. `rg` finds `assert_read_is_free`, `assert_total_ctf_conserved`, and `assert_no_post_init_mint` only in their module tests and `tb_1_acceptance`. `assert_no_post_init_mint` currently returns `Ok(())` for every K5 variant post-init because there is no Mint variant [src/economy/monetary_invariant.rs:134](/home/zephryj/projects/turingosv4/src/economy/monetary_invariant.rs:134). Landing point is TB-2/RSP-1 per the ignored AT-2/AT-4 comments [tests/tb_1_acceptance.rs:470](/home/zephryj/projects/turingosv4/tests/tb_1_acceptance.rs:470).

Q6. The P6 downgrade is defensible only because it is explicitly non-blocking. T10/T12 are registered as ignored or empty harness stubs [tests/tb_1_acceptance.rs:461](/home/zephryj/projects/turingosv4/tests/tb_1_acceptance.rs:461), [tests/tb_1_acceptance.rs:496](/home/zephryj/projects/turingosv4/tests/tb_1_acceptance.rs:496). I verified `/tmp/tb1_day4_smoke_v2/run2.jsonl` has `solved=true` and `h_vppu=6.215891726697228`, and `cargo test -p minif2f_v4 --lib h_vppu_history` passed 9/9. That is useful evidence, not a canonical in-harness assertion.

Q7. Day-3 absorbed monetary invariant only at direct-function level, not dispatch level. `escrow_vault` has good unit coverage, and `cargo test -p turingosv4 economy::escrow_vault` passed 11/11; overpayout and underpayout are tested [src/economy/escrow_vault.rs:318](/home/zephryj/projects/turingosv4/src/economy/escrow_vault.rs:318), [src/economy/escrow_vault.rs:334](/home/zephryj/projects/turingosv4/src/economy/escrow_vault.rs:334). But escrow is absent from Tier-A and has no call site. If TB-1 claims escrow admission/payout semantics, add Tier-A; if not, mark it scaffolded for TB-2.

Q8. Day-4 production ordering is currently correct: load, query/stamp, record, save [experiments/minif2f_v4/src/bin/evaluator.rs:373](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:373). The unit test proves the API behavior, not the production wire site [experiments/minif2f_v4/src/h_vppu_history.rs:266](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/h_vppu_history.rs:266). Day-5 P1 tests are not bit-equal to Day-3: comments changed and L4/L4.E deletion test order changed, but I do not see a behavioral regression.

**Section C: P0 Must-Fix Before Unqualified Ship**

1. Narrow the TB-1 ship claim to “P1/P3 primitives scaffolded” or add real sequencer-level tests/wiring for rejection routing and monetary guard enforcement.
2. Promote all-six economic subindex coverage into Tier-A, or the Tier-A battery does not prove total CTF conservation over the whole `EconomicState`.
3. Add structural shielding for `RejectedSubmissionRecord`: no default raw serialization, or hide full records behind privileged/test-only APIs.
4. Make `AcceptedLedger::load_from_path` verify the hash chain, then add tamper tests for fake genesis, reorder, parent-state-root, prev-hash/hash-only mutation.

**Section D: P1 Should-Fix**

Escrow should either be promoted into Tier-A or explicitly documented as RSP-1-only. P6 evidence should move from `/tmp`/ignored stubs into a reproducible non-default harness when it becomes blocking. Add an integration test around the evaluator h_vppu wire site, not just the `HVppuHistory` API.

**Section E: Recommendation**

Do not ship with the current central claim. Ship is acceptable after the claim is narrowed and the P0 test/structural gaps are patched; otherwise this deserves round 2 after fixes.
2026-04-29T18:28:40.292284Z ERROR codex_core::session: failed to record rollout items: thread 019dda7b-038f-7e50-8907-e75afde78628 not found
tokens used
154,768
**Section A: Verdict**

CHALLENGE, high conviction. The 9 Tier-A tests are green, but they prove the new primitives, not the full central claim. A malicious implementer can still pass all 9 while bypassing real dispatch routing, monetary guard call sites, and raw rejection-record shielding.

**Section B: Q1-Q8**

Q1. The exact “silent rebase to canonical” bypass would not pass: `test_p1_kill_1_no_wtool_bypass` expects `Err` or `reconstructed != canonical_root` [tests/tb_1_acceptance.rs:132](/home/zephryj/projects/turingosv4/tests/tb_1_acceptance.rs:132). But the test mutates the serialized L4 entries, not an independent `state.db`, and it treats `Ok(divergent_root)` as acceptable. Also `load_from_path` calls `reconstruct_state` only, not `verify_chain` [src/economy/ledger.rs:300](/home/zephryj/projects/turingosv4/src/economy/ledger.rs:300). Simple fake-genesis/reorder/parent-state-root tamper is caught by replay or `verify_chain`, but `prev_hash`/`hash`/`logical_t`-only tamper can load successfully unless the caller separately verifies.

Q2. No, L4/L4.E disjointness is not actually tested through a real route. Tests manually instantiate `AcceptedLedger` and `RejectionEvidenceWriter` and append only to L4.E [tests/tb_1_acceptance.rs:147](/home/zephryj/projects/turingosv4/tests/tb_1_acceptance.rs:147). Production `dispatch_transition` is still all-variants `NotYetImplemented` [src/state/sequencer.rs:47](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:47), and `apply_one` early-returns on transition error before any L4.E append [src/state/sequencer.rs:339](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:339). TB-1 must say “primitives only”; enforcement is TB-2.

Q3. The isolation test is only a projection test. `PublicRejectionView` omits `raw_diagnostic_cid` [src/bottom_white/ledger/rejection_evidence.rs:160](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/rejection_evidence.rs:160), but `RejectedSubmissionRecord` is public, derives `Serialize`, and exposes `pub raw_diagnostic_cid` [src/bottom_white/ledger/rejection_evidence.rs:82](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/rejection_evidence.rs:82). `records()` returns full raw records [src/bottom_white/ledger/rejection_evidence.rs:320](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/rejection_evidence.rs:320). Grep found no current production direct serialization, but the structural guard is convention, not enforced.

Q4. Tier-A’s 5-step redistribution touches balances and escrows only [tests/tb_1_acceptance.rs:307](/home/zephryj/projects/turingosv4/tests/tb_1_acceptance.rs:307). The implementation does sum all six subindices [src/economy/monetary_invariant.rs:94](/home/zephryj/projects/turingosv4/src/economy/monetary_invariant.rs:94), and a unit test covers all six [src/economy/monetary_invariant.rs:403](/home/zephryj/projects/turingosv4/src/economy/monetary_invariant.rs:403). But that is not one of the 9 Tier-A tests. Constant-zero supply would be caught by Tier-A post-init mint [tests/tb_1_acceptance.rs:405](/home/zephryj/projects/turingosv4/tests/tb_1_acceptance.rs:405), but undercounting claims/stakes/task bounty/challenge bond can pass Tier-A.

Q5. The monetary guards are not wired into a real dispatch/settlement path. `rg` finds `assert_read_is_free`, `assert_total_ctf_conserved`, and `assert_no_post_init_mint` only in their module tests and `tb_1_acceptance`. `assert_no_post_init_mint` currently returns `Ok(())` for every K5 variant post-init because there is no Mint variant [src/economy/monetary_invariant.rs:134](/home/zephryj/projects/turingosv4/src/economy/monetary_invariant.rs:134). Landing point is TB-2/RSP-1 per the ignored AT-2/AT-4 comments [tests/tb_1_acceptance.rs:470](/home/zephryj/projects/turingosv4/tests/tb_1_acceptance.rs:470).

Q6. The P6 downgrade is defensible only because it is explicitly non-blocking. T10/T12 are registered as ignored or empty harness stubs [tests/tb_1_acceptance.rs:461](/home/zephryj/projects/turingosv4/tests/tb_1_acceptance.rs:461), [tests/tb_1_acceptance.rs:496](/home/zephryj/projects/turingosv4/tests/tb_1_acceptance.rs:496). I verified `/tmp/tb1_day4_smoke_v2/run2.jsonl` has `solved=true` and `h_vppu=6.215891726697228`, and `cargo test -p minif2f_v4 --lib h_vppu_history` passed 9/9. That is useful evidence, not a canonical in-harness assertion.

Q7. Day-3 absorbed monetary invariant only at direct-function level, not dispatch level. `escrow_vault` has good unit coverage, and `cargo test -p turingosv4 economy::escrow_vault` passed 11/11; overpayout and underpayout are tested [src/economy/escrow_vault.rs:318](/home/zephryj/projects/turingosv4/src/economy/escrow_vault.rs:318), [src/economy/escrow_vault.rs:334](/home/zephryj/projects/turingosv4/src/economy/escrow_vault.rs:334). But escrow is absent from Tier-A and has no call site. If TB-1 claims escrow admission/payout semantics, add Tier-A; if not, mark it scaffolded for TB-2.

Q8. Day-4 production ordering is currently correct: load, query/stamp, record, save [experiments/minif2f_v4/src/bin/evaluator.rs:373](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:373). The unit test proves the API behavior, not the production wire site [experiments/minif2f_v4/src/h_vppu_history.rs:266](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/h_vppu_history.rs:266). Day-5 P1 tests are not bit-equal to Day-3: comments changed and L4/L4.E deletion test order changed, but I do not see a behavioral regression.

**Section C: P0 Must-Fix Before Unqualified Ship**

1. Narrow the TB-1 ship claim to “P1/P3 primitives scaffolded” or add real sequencer-level tests/wiring for rejection routing and monetary guard enforcement.
2. Promote all-six economic subindex coverage into Tier-A, or the Tier-A battery does not prove total CTF conservation over the whole `EconomicState`.
3. Add structural shielding for `RejectedSubmissionRecord`: no default raw serialization, or hide full records behind privileged/test-only APIs.
4. Make `AcceptedLedger::load_from_path` verify the hash chain, then add tamper tests for fake genesis, reorder, parent-state-root, prev-hash/hash-only mutation.

**Section D: P1 Should-Fix**

Escrow should either be promoted into Tier-A or explicitly documented as RSP-1-only. P6 evidence should move from `/tmp`/ignored stubs into a reproducible non-default harness when it becomes blocking. Add an integration test around the evaluator h_vppu wire site, not just the `HVppuHistory` API.

**Section E: Recommendation**

Do not ship with the current central claim. Ship is acceptable after the claim is narrowed and the P0 test/structural gaps are patched; otherwise this deserves round 2 after fixes.
