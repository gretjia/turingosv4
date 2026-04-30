# STEP_B Preflight — TB-2 Sequencer Runtime Closure

**Date**: 2026-04-30
**TB**: TB-2 ("P1/P3 Runtime Boundary Closure + RSP-1")
**Charter**: `handover/tracer_bullets/TB-2_charter_2026-04-30.md`
**Protocol**: `handover/ai-direct/STEP_B_PROTOCOL.md`

---

## 0. Why STEP_B applies here

`STEP_B_PROTOCOL.md §0` scope: "any change to files in CLAUDE.md's restricted list (currently `src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`). Also applicable to any proposal that touches 'institution' per C-031."

`src/state/sequencer.rs` is not literally on the path list, but it satisfies the "touches institution" trigger:

- it is the **runtime wtool gate** — every accepted state transition is committed by `Sequencer::apply_one` (the only writer that mutates `state_root_t` / `ledger_root_t` / accepted `logical_t`);
- TB-2 changes its **error-path semantics** (rejected tx must now produce L4.E side effects) and its **queue payload type** (`Sender<TypedTx>` → `Sender<SubmissionEnvelope>`), both institutional-class changes;
- a regression here breaks the L4 / L4.E split that TB-1 paid 7 days to establish.

Treat sequencer.rs edits with full STEP_B rigor. If the dual external auditors disagree on whether STEP_B is required, conservative verdict (require) wins per `feedback_dual_audit_conflict`.

---

## 1. Target

| File | Role | Touched by TB-2 |
|---|---|---|
| `src/state/sequencer.rs` | L4 sequencer + `dispatch_transition` + `apply_one` driver | **YES** (primary) |
| `src/state/q_state.rs` | `QState` snapshot type | possibly (helper for deterministic interim `state_root_t` mutation) |
| `src/state/typed_tx.rs` | `TypedTx` enum | **NO** (no new variants Day-1 — TB-3 scope) |
| `src/sdk/tools/wallet.rs` | wtool | **NO** (no widening) |
| `src/bus.rs` / `src/kernel.rs` | bus / kernel | **NO** |
| `src/economy/ledger.rs::AcceptedLedger` | TB-1 RSP-0 primitive wrapper | **NO** (stays a primitive; not used as production accepted spine) |
| `src/bottom_white/ledger/transition_ledger` + `LedgerWriter` | canonical L4 | invoked through existing API only (no signature change) |
| `src/bottom_white/ledger/rejection_evidence.rs` | L4.E writer | invoked through existing `append_rejected` only |
| `tests/tb_2_runtime_boundary.rs` | TB-2 acceptance battery | **NEW** |

---

## 2. Why the change is necessary (Phase-0 brief for external audit)

**Observable behavior broken at HEAD `459c747`**:

1. `Sequencer::dispatch_transition` returns `TransitionError::NotYetImplemented` for every `TypedTx` variant. No real WorkTx ever produces a `q_next`. The accepted L4 spine is therefore unexercised at runtime; the L4.E spine is unreachable from real submissions.
2. `Sequencer::apply_one` early-returns on transition error and only `log::debug!`s the rejection. No L4.E row is ever written from the runtime path. The L4.E primitive built in TB-1 is dead-code from the runtime's perspective.
3. `Sequencer::submit` allocates a `submit_id` and returns it in `SubmissionReceipt`, but the queue carries `TypedTx` (no envelope), so `apply_one` cannot key any L4.E record by the same `submit_id` that the caller received. The L4.E identity contract (rejected evidence is `submit_id`-keyed, never `logical_t`-keyed) is unenforceable end-to-end.
4. P1 Exit 5 / 6 / 9 and P3 Exit 3 / 5 cannot be discharged at the runtime spine. P1 kill 1 / 2 + P3 kill 2 / 3 are only proven against synthetic Tier-A inputs (per TB-1 narrowed claim), not against `Sequencer::submit` traffic.

**Failure mode if we don't change**:

- TuringOS continues to hold "primitives required to honor the L4 / L4.E split" but cannot honestly claim the runtime kernel does so. P2 Agent Runtime stays blocked because role separation cannot be demonstrated without runtime stake/escrow gating. P4 Information Loom stays blocked because the clusterer has no real L4.E input. Every downstream phase (P5/P6 product line, P7 public, P8 autonomous) inherits the same blocker.

**Less-invasive alternatives considered and rejected**:

- *(Alt A)* Keep `dispatch_transition` `NotYetImplemented` and write L4.E from `apply_one`'s error path only. Rejected: the `NotYetImplemented` return signal is ambient, not specific — the rejection_class field of L4.E records would lose causal information (predicate-fail vs no-stake vs no-escrow vs monetary-violation are indistinguishable). Predicate gating must live in `dispatch_transition`.
- *(Alt B)* Move ledger writes inside `dispatch_transition` (the user's naive A from the audit). Rejected: violates the bottom-white separation between pure transition function and side-effecting commit. `dispatch_transition` is meant to be replayable from the ledger; putting writes in it would create a chicken-and-egg loop on replay.
- *(Alt C)* Swap `economy::ledger::AcceptedLedger` for `transition_ledger` only inside the WorkTx accept arm. Rejected: `AcceptedLedger` is a TB-1 RSP-0 primitive wrapper documented as not production-grade (in-memory `Vec`, no real `SystemSignature`, no `Git2LedgerWriter` chain). Promoting it to production would create a second accepted spine ("L4-A vs L4-B") that contradicts the ChainTape single-spine contract.

**Audit gate**: if both Codex and Gemini say "less-invasive alternative exists", take it. If both say "change as scoped is necessary", proceed to Phase 1. Disagreement → conservative verdict (block) per `feedback_dual_audit_conflict`.

---

## 3. Minimum sufficient version (scope ceiling)

Day-1 of any production-code work must NOT exceed:

1. `SubmissionEnvelope { submit_id, tx }` introduced; `Sequencer::queue_tx` and `Sequencer::run` rewritten to carry it; `Sequencer::submit` constructs the envelope before `try_send`. Public `submit()` signature unchanged (still returns `SubmissionReceipt`).
2. `dispatch_transition` `TypedTx::Work` arm filled with **pure** validation:
   - parent-root match (`tx.parent_state_root == q.state_root_t`, where `parent_state_root` is the WorkTx's declared parent);
   - all acceptance predicate results in `tx.predicate_results.acceptance` are `true`;
   - settlement predicate results, where applicable to RSP-1 stake/escrow, are `true` or empty;
   - `tx.stake > 0` (RSP-1 YES-stake proxy);
   - `q.economic_state_t` has an escrow / task-market entry for `tx.task_id` (RSP-1 escrow proxy);
   - `monetary_invariant::assert_no_post_init_mint(tx, q)`;
   - `monetary_invariant::assert_read_is_free(TxKind::Work, 0)`;
   - `monetary_invariant::assert_total_ctf_conserved(q.economic_state_t, q_next.economic_state_t, &[])`.

   On accept: returns `Ok((q_next, signals))` where `q_next.state_root_t = sha256("turingosv4.worktx.accept.v1" || q.state_root_t || work_tx_digest)` (interim domain-separated hash; real patch semantics are P5 scope) and `q_next.economic_state_t` reflects stake/escrow/balances delta. On reject: returns `Err(TransitionError::<specific class>)`.

3. `apply_one` rewritten so that on `Err(e)` from `dispatch_transition`:
   - CAS-put canonical-encoded tx payload, obtain `tx_payload_cid`;
   - optionally CAS-put `e.to_string()` as `raw_diagnostic_cid` (already serde-shielded by TB-1 P0-3);
   - `RejectionEvidenceWriter::append_rejected(submit_id = envelope.submit_id, parent_state_root = q_snapshot.state_root_t, agent_id = tx.submitter_id (or system fallback), tx_kind = tx.tx_kind(), tx_payload_cid, rejection_class = map(e), raw_diagnostic_cid, public_summary = derived);`
   - return `ApplyError::Transition(e)` without advancing `logical_t` / `state_root_t` / `ledger_root_t`.

4. Accepted path stays on the **existing** `transition_ledger` + `LedgerWriter` flow already wired into `apply_one`'s success arm — TB-2 does not introduce a new ledger writer.

Everything else (`TypedTx::Verify`, `TypedTx::Challenge`, `TypedTx::Reuse`, `TypedTx::FinalizeReward`, `TypedTx::TaskExpire`, `TypedTx::TerminalSummary`) stays `NotYetImplemented` and is out of TB-2 scope.

**Scope creep guard**: any line of code outside this list constitutes a separate atom and must be extracted into TB-3+ unless the auditors explicitly approve it in Phase-1c.

---

## 4. Parallel-branch plan (Phase 1)

### A branch — `experiment/tb2-sequencer-runtime-closure`

```bash
git worktree add .claude/worktrees/stepb-tb2-sequencer-runtime-closure -b experiment/tb2-sequencer-runtime-closure
```

Implements §3 minimum-sufficient version + `tests/tb_2_runtime_boundary.rs` (12 tests, listed in §5). Each test added in red→green order; commit boundaries respect `feedback_phased_checkpoint` (paired N=20 not applicable — runtime spine is deterministic per submission, no LLM in the loop yet).

### B branch — baseline (control)

`main @ <last-PASS HEAD>` (currently `459c747`). No code change. The acceptance battery is run on B as a control to confirm it produces zero rows of L4.E and zero `state_root_t` advance from real `Sequencer::submit` traffic (because no `WorkTx` survives `NotYetImplemented`). This is the "before" snapshot.

### Acceptance gate

A is merge-eligible only if all of:

1. `cargo check --workspace` clean on A.
2. `cargo test --workspace` green on A (including all pre-existing tests).
3. `tests/tb_2_runtime_boundary.rs` 12/12 green on A.
4. `cargo test --workspace` on B unchanged (baseline pre-TB-2 suite stays green; required to detect "did A break something elsewhere?").
5. Diff is confined to the §1 `Touched=YES` rows. Any edit outside that surface fails Phase-1c review.
6. Dual external audit on diff (Phase-1c) returns PASS / PASS. VETO from either auditor blocks merge until addressed; CHALLENGE → conservative.
7. Two ship proofs (charter §8) demonstrable in fixture: predicate-failed WorkTx via `Sequencer::submit` → exactly one L4.E row + zero `state_root_t` change; predicate-passing WorkTx → state_root + ledger_root + logical_t advance + zero L4.E rows.

A FAIL on any of the above → branch abandoned or revised; charter must change before retry (TB methodology v2 no-same-charter-retry rule).

---

## 5. Acceptance battery (`tests/tb_2_runtime_boundary.rs`)

### Submit-id plumbing

| # | Test | Asserts |
|---|---|---|
| 1 | `submit_returns_receipt_and_enqueues_same_submit_id` | The `submit_id` returned by `submit()` equals the `envelope.submit_id` `apply_one` operates on. |
| 2 | `apply_one_can_access_envelope_submit_id` | `apply_one` signature accepts `SubmissionEnvelope` (typecheck-level guarantee). |

### Rejection spine (proof 1)

| # | Test | Asserts |
|---|---|---|
| 3 | `runtime_predicate_failed_worktx_appends_l4e` | Submit a WorkTx whose `predicate_results.acceptance` contains a `false`. Expect: 1 L4.E row with matching `submit_id`, `rejection_class = PredicateFailed`. |
| 4 | `runtime_stakeless_worktx_appends_l4e` | Submit a WorkTx with `stake == 0`. Expect: 1 L4.E row, `rejection_class = StakeRequired` (or P3-named equivalent). |
| 5 | `runtime_no_escrow_worktx_appends_l4e` | Submit a WorkTx for a `task_id` with no escrow / no task-market entry. Expect: 1 L4.E row, `rejection_class = EscrowMissing`. |
| 6 | `runtime_post_init_mint_worktx_appends_l4e` | Submit a WorkTx whose `q_next.economic_state_t` total supply > `q.economic_state_t` total supply. Expect: 1 L4.E row, `rejection_class = PostInitMint`. |
| 7 | `runtime_rejected_worktx_does_not_advance_logical_t` | Across tests 3-6, accepted `logical_t` is unchanged. |
| 8 | `runtime_rejected_worktx_does_not_change_state_root` | Across tests 3-6, `state_root_t` is unchanged AND `ledger_root_t` is unchanged. |

### Acceptance spine (proof 2)

| # | Test | Asserts |
|---|---|---|
| 9 | `runtime_accepted_worktx_advances_state_root` | Submit a predicate-passing WorkTx with stake+escrow. `state_root_t` differs from the pre-submit snapshot (matches the interim domain-separated hash). |
| 10 | `runtime_accepted_worktx_advances_ledger_root` | After test 9, `ledger_root_t` differs (canonical `transition_ledger` advanced). |
| 11 | `runtime_accepted_worktx_increments_logical_t` | After test 9, accepted `logical_t == prev + 1`. |
| 12 | `runtime_accepted_worktx_does_not_append_l4e` | After test 9, L4.E row count is unchanged. |

### Test fixtures

A small helper module `tests/common/runtime_fixtures.rs` (or inline within the test file) provides:

- `seed_economic_state_with_escrow(task_id, bounty: u64) -> EconomicState`
- `make_worktx(opts: WorkTxFixtureOpts) -> TypedTx::Work` (parameterizes `parent_state_root` / `predicate_results.acceptance` / `stake` / `task_id` / supply-delta-injection so tests 3-6, 9 share construction)
- `assert_l4e_row_matches(submit_id: u64, rejection_class: ...) -> ()`
- `assert_l4e_row_count_unchanged(before: usize, writer: &Reader) -> ()`

Helpers MUST be test-only; they MUST NOT leak into `src/`.

---

## 6. Frozen analyzer (Phase 2)

TB-2 is a runtime-spine correctness change, not a population-statistics A/B. The pre-registered decision rule is **deterministic 12/12 PASS** on `tests/tb_2_runtime_boundary.rs`, not a SolveRate delta. `frozen_analysis.py` is **not invoked** for TB-2 acceptance (no LLM-in-the-loop sample). The STEP_B Phase-2 `--control`/`--treatment` machinery is therefore replaced by:

- `cargo test --test tb_2_runtime_boundary` on A → expect 12/12 PASS
- `cargo test --test tb_2_runtime_boundary` on B → expect 12/12 FAIL (because runtime path is `NotYetImplemented`)

This A/B asymmetry is itself the empirical signal. Both runs pre-registered.

---

## 7. Verdict and merge path (Phase 3)

| Verdict | Action |
|---|---|
| A 12/12 PASS + B 12/12 FAIL + both auditors PASS on diff | `git merge experiment/tb2-sequencer-runtime-closure --no-ff` on `main`; update `TB_LOG.tsv` row TB-2 status `active → shipped` with `ship_commits` range; update `AUTO_RESEARCH_NOTEPAD.md`; update `ROADMAP_9_PHASE_2026-04-29.md` P1 Exit 5/6/9 + P3 Exit 3/5 to green. |
| A < 12/12 PASS or B not 12/12 FAIL | abandon branch (`git branch -D experiment/tb2-sequencer-runtime-closure` or archive); write `handover/alignment/OBS_TB-2_FAILED.md` per TB methodology v2; new charter required before retry. |
| Auditors split (PASS / VETO) | conservative wins → block. |
| Auditors split (PASS / CHALLENGE) | resolve CHALLENGE → re-audit. |
| Auditors agree CHALLENGE | merge CHALLENGE → re-audit; do not merge to main while CHALLENGE is open. |

Cleanup: `git worktree remove .claude/worktrees/stepb-tb2-sequencer-runtime-closure`. If branch archived: `git tag archive/tb2-sequencer-runtime-closure_2026-MM-DD experiment/tb2-sequencer-runtime-closure` then delete branch.

---

## 8. Forbidden in this STEP_B (red lines)

Per TB-2 charter §5, repeated here for the auditors:

1. No ledger I/O (CAS put, writer commit, ledger append) inside `dispatch_transition`. The function returns `(q_next, signals)` or `Err(TransitionError)` only.
2. No use of `economy::ledger::AcceptedLedger::append_accepted` in the production accepted spine. `AcceptedLedger` stays a TB-1 primitive / test wrapper.
3. No new `TypedTx` variants. `task_open_tx` / `escrow_lock_tx` / `yes_stake_tx` are reserved for TB-3.
4. No non-empty `exempt_tx_kinds` argument at the runtime call sites of `assert_total_ctf_conserved`. Production must pass `&[]`.
5. No widening of `WalletTool` mutation surface.
6. No P5/P6/h_vppu/capability-metric work inside this STEP_B branch.
7. No edits to `src/kernel.rs` / `src/bus.rs` / `src/sdk/tools/wallet.rs` (the formal STEP_B-restricted set per CLAUDE.md). If TB-2 implementation discovers a real need to touch any of those, halt and open a *separate* STEP_B preflight before continuing.

A diff that violates any of these auto-fails Phase-1c review even if `cargo test` is green.

---

## 9. Pointers

- TB-2 charter: `handover/tracer_bullets/TB-2_charter_2026-04-30.md`
- STEP_B protocol: `handover/ai-direct/STEP_B_PROTOCOL.md`
- Restricted-file path correction (path drift OBS): `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md`
- TB-1 ship row + narrowed claim: `handover/tracer_bullets/TB_LOG.tsv` (TB-1 row, ship_commits `063b003..ccb01fa`)
- TB-1 dual-audit verdict: `handover/audits/DUAL_AUDIT_TB_1_VERDICT_2026-04-29.md`
- 9-phase canonical roadmap: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` (P1 / P3 Current state refreshed 2026-04-30)
- Memory: `feedback_step_b_protocol`, `feedback_dual_audit`, `feedback_dual_audit_conflict`, `feedback_phased_checkpoint`, `feedback_smoke_before_batch`, `feedback_no_fake_menus`, `feedback_session_label_codification`.
