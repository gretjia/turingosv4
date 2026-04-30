# TB-2 Phase-0 Dual External Audit — Round-1 Merged Verdict

**Date**: 2026-04-30
**Target**: STEP_B preflight `handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md` (rev at HEAD `3f06d51`)
**Charter**: `handover/tracer_bullets/TB-2_charter_2026-04-30.md`
**Audits**: Codex r1 (`CODEX_TB_2_PHASE0_AUDIT_2026-04-30.md`) + Gemini r1 (`GEMINI_TB_2_PHASE0_AUDIT_2026-04-30.md`)
**Merge rule**: VETO > CHALLENGE > PASS (per memory `feedback_dual_audit_conflict`)

---

## Verdict matrix

| Audit | Verdict | Conviction | Q breakdown |
|---|---|---|---|
| **Codex** (implementer-paranoid; verified vs HEAD `3f06d51`) | **CHALLENGE** | 5/5 | Q1 PASS-w/-caveat, Q2 PASS-w/-minor-overstatement, Q3 **CHALLENGE (substrate)**, Q4 PASS, Q5 **CHALLENGE (concurrency note + ABI rationale missing)**, Q6 PASS, Q7 **CHALLENGE (battery not compile-expressible)**, Q8 **CHALLENGE (4 adversarial-sweep risks)** |
| **Gemini** (strategic / architectural / constitutional) | **CHALLENGE** | 5/5 | Q1 PASS, Q2 PASS, Q3 PASS, Q4 PASS, Q5 PASS, Q6 PASS, Q7 **CHALLENGE (4 missing tests)**, Q8 PASS |

**Conservative-merged verdict**: **CHALLENGE / 5/5**. No VETO from either auditor — both endorse the architectural framing and the runtime-gap diagnosis. Both reject Phase-1 proceed against the current preflight text.

---

## Where the audits agree (both PASS)

1. **Q2 — broken-behavior diagnosis is verifiable.** Codex line-by-line confirms: `dispatch_transition` is all-`NotYetImplemented` at `src/state/sequencer.rs:54-60`; `apply_one` early-returns via `?` on transition error at `src/state/sequencer.rs:339, 346`; `submit_id` is allocated at `src/state/sequencer.rs:292` but `try_send` at `:293` carries only `tx`, and `apply_one(:332)` receives `TypedTx` not envelope. Gemini concurs the gap is real. Minor wording: preflight §2 says "`apply_one ... log::debug!`s" — actually the `log::debug!` happens in `Sequencer::run` at `:308, :313` AFTER `apply_one` returns the error. Cosmetic; not a blocker.
2. **Q3 (architectural ruling) — no ledger I/O inside `dispatch_transition`.** Both: architecturally mandatory. Gemini: "If ledger I/O were placed inside it, replaying the ledger (which is required by Art IV Boot and P1:8) would re-trigger those side effects, destroying determinism and creating a catastrophic chicken-and-egg loop." Codex confirms accepted path is already on `transition_ledger` + `LedgerWriter` at `:351, :377, :386, :423`.
3. **Q4 — AcceptedLedger as TB-1 primitive, not production L4.** Both PASS. Codex: "Current runtime `apply_one` uses `transition_ledger` and not `AcceptedLedger`" (`:27, :30, :351, :423`); `append_accepted` call sites are definition/tests/TB-1 integration only. The red line is correct future-proofing, not correcting an existing drift.
4. **Q6 — `exempt_tx_kinds` fence.** Both PASS. Codex confirms implementation short-circuits non-empty exempt at `src/economy/monetary_invariant.rs:177, 179`; only test-only call site is `ctf_exempt_short_circuits` at `:320, :328`; TB-1 integration calls all pass `&[]` at `tests/tb_1_acceptance.rs:355, 382, 445, 540`.

## Where they CONVERGE on CHALLENGE — Q7 acceptance battery

- **Gemini-Q7**: 12-test battery is missing 4 critical tests:
  1. `runtime_stale_parent_worktx_appends_l4e` (StaleParentRoot rejection class).
  2. `submit_queue_full_consumes_submit_id` (failed `try_send` still burns ID — must be asserted to prevent future ID-reuse "fixes").
  3. `runtime_l4e_public_view_honors_serde_shield` (re-confirm TB-1 P0-3 at runtime path).
  4. `runtime_replay_ignores_l4e` (state.db rebuild from canonical L4 only — proves P1:8).

- **Codex-Q7 + Q8**: 12-test battery is not compile-expressible as written:
  - Tests 1-2 want `apply_one(envelope)` typecheck, but `apply_one` is `pub(crate)` and integration tests under `tests/` cannot call it.
  - L4.E row counting requires runtime-connected `RejectionEvidenceWriter`, but `Sequencer` has no rejection writer field today (`src/state/sequencer.rs:230, 247`).
  - Test 6 (`runtime_post_init_mint_worktx_appends_l4e`) is structurally blocked: WorkTx carries no `supply-delta-injection` field; mint-via-WorkTx is not a representable transition.
  - Test 9's interim hash `turingosv4.worktx.accept.v1` is unregistered — only `turingosv4.l4_state_root.v1` exists in source (`src/economy/ledger.rs:350, 357`).

The two CHALLENGEs are complementary — Gemini's are blind-spots in coverage; Codex's are blind-spots in expressibility. Both must be addressed in preflight r2.

## Codex-only CHALLENGE findings (substrate / API mismatches not visible to strategic angle)

These are file:line-grounded blockers that revise the preflight scope, not just its prose:

### CHL-S1 — Sequencer has no L4.E writer field (P0-1)

`Sequencer` fields: `next_submit_id`, `next_logical_t`, `queue_tx`, CAS, keypair, epoch, `ledger_writer`, registries, `q` (`src/state/sequencer.rs:230, 247`). **No `rejection_writer` field, no constructor parameter, no test accessor.** `RejectionEvidenceWriter::append_rejected` exists and is callable (`src/bottom_white/ledger/rejection_evidence.rs:258, 268`) but it's disconnected from the runtime spine. The preflight's §3 minimum-sufficient version doesn't disclose how the writer arrives.

### CHL-S2 — TaskId vs TxId economic-lookup mismatch (P0-2)

`WorkTx.task_id: TaskId` (`src/state/typed_tx.rs:225, src/state/typed_tx.rs:33-35`) but `EconomicState.escrows_t` and `task_markets_t` are both `BTreeMap<TxId, ...>` (`src/state/q_state.rs:161, 224`). The task-keyed `EscrowVault` exists separately but is "distinct from `state::q_state::EscrowEntry`" per its own doc-comments (`src/economy/escrow_vault.rs:15, 53, 146, 168`). The preflight says "`q.economic_state_t` has an escrow / task-market entry for `tx.task_id`" — which is currently NOT a one-line lookup; it needs an explicit bridge.

Three options Codex enumerates (none chosen by audit; this is an architectural decision):

| Option | Cost | Risk |
|---|---|---|
| (a) Deterministic bridge `TxId(tx.task_id.0.clone())` at lookup site | 0.5 day; sequencer-only | Conflates two namespaces; future TB-3/TB-4 may need to undo |
| (b) Add task-keyed index to `q_state.rs` | 1 day; touches `q_state.rs` (possibly outside sequencer.rs scope) | Cleanest semantically; widens TB-2 surface |
| (c) Make `EscrowVault` a `Sequencer` dependency | 1+ day; touches sequencer construction | Doubles the escrow truth source until L4-state projection unifies |

**Audit does not pick** — flagged for explicit decision in r2.

### CHL-S3 — Battery not compile-expressible (P0-3)

- `apply_one` and `dispatch_transition` are `pub(crate)`. Tests 1-2 (envelope plumbing) cannot live in `tests/tb_2_runtime_boundary.rs` as-is.
- Test 6 has no representable transition (post-init mint via WorkTx).

Remediation: split private-API tests into `sequencer.rs` unit tests (in-crate); expose only what's essential as `pub(crate)` test accessors; rewrite Test 6 as a monetary-invariant unit test or drop it.

### CHL-S4 — Error / rejection-class mapping undefined (P0-4)

- `TransitionError` has `StakeInsufficient`, `TaskNotFound`, `NotYetImplemented` (`src/state/typed_tx.rs:717, 787`) but **no `EscrowMissing` or `PostInitMint` variants**.
- `RejectionEvidence::RejectionClass` has `PredicateFailed`, `PolicyViolation`, `EscrowMissing`, `InvariantViolation` (`src/bottom_white/ledger/rejection_evidence.rs:56, 67`) — names don't match the battery's `StakeRequired`/`PostInitMint`.

Remediation: pre-register a `TransitionError → RejectionClass` mapping table in the preflight; add new `TransitionError` variants if needed (note: `typed_tx.rs` edit is outside `sequencer.rs` scope — must be disclosed).

### CHL-S5 — Adversarial sweep (Codex-Q8; mostly P1)

- Multi-producer ordering: `fetch_add` precedes `try_send`, so submit-ID monotonicity is NOT an arrival-order guarantee. Tests must NOT assert "queue order = submit_id order".
- Replay determinism: `apply_one` ignores `_signals` from `dispatch_transition` (`src/state/sequencer.rs:341, 346`). Any TB-2 signal side-channel is UNVERIFIED for replay unless ledgered or explicitly out-of-scope.
- Orphan-CAS on partial-write: rejection path adds CAS write + L4.E append; partial-write rollback contract not specified. (`append_rejected` is currently in-memory and infallible per `:30, :34, :258, :268`, so immediate risk is bounded; but persistence semantics are deferred.)

---

## Gemini-only finding (Section D / P1)

**G-P1**: add `src/state/sequencer.rs` to CLAUDE.md's explicit restricted-file list. C-031 catch-all is fine for *this* audit but is "vulnerable to future LLM agents bypassing STEP_B due to context-window truncation of case law". 2-min hygiene fix.

---

## Merged remediation list (R2 input)

### P0 — must-fix before STEP_B Phase-1 starts (5 items)

| # | Origin | Fix | Effort | Determinate? |
|---|---|---|---|---|
| **P0-A** | Codex S1 | Disclose `Sequencer.rejection_writer` field + constructor parameter + test accessor in preflight §3. | 0.5 day | YES — surgical |
| **P0-B** | Codex S2 | Pick one of (a)/(b)/(c) for `TaskId` vs `TxId` resolution. **Decision needed — not surgical.** | 0.5–1 day | **NO — architectural decision** |
| **P0-C** | Codex S3 | Split battery: unit tests in `sequencer.rs` for private-API checks; integration tests in `tests/tb_2_runtime_boundary.rs` for behaviour through `Sequencer::submit`. Drop or re-route Test 6. | 0.5–1 day | YES — determinate split |
| **P0-D** | Codex S4 | Pre-register `TransitionError → RejectionClass` mapping in preflight. Add new `TransitionError` variants only if mapping requires (and disclose `typed_tx.rs` edit). | 0.25–0.5 day | YES — determinate |
| **P0-E** | Gemini Q7 | Expand battery 12 → 16 tests: add `StaleParentRoot`, `submit_queue_full_consumes_submit_id`, `runtime_l4e_public_view_honors_serde_shield`, `runtime_replay_ignores_l4e`. Update charter §8 ship proofs accordingly. | 0.5 day | YES — determinate |

### P1 — should-fix; can proceed-with-OBS (5 items, ~80 min total)

| # | Origin | Fix | Effort |
|---|---|---|---|
| P1-A | Gemini D-1 | Add `src/state/sequencer.rs` to CLAUDE.md "Code Standard" restricted list. | 2 min |
| P1-B | Codex D-1 | Reword preflight §0 to cite STEP_B line 3 directly (not C-031 alone). | 10 min |
| P1-C | Codex D-2 | Justify `SubmissionEnvelope` over tuple `(u64, TypedTx)` (named struct = clearer + extensible; same surface change either way). | 15 min |
| P1-D | Codex D-3 | Add concurrency note: `submit_id` allocation order is NOT receiver arrival order under multi-producer scheduling. Tests must not assert otherwise. | 20 min |
| P1-E | Codex D-4 + D-5 | Name interim state-root domain as a registered constant; document orphan-CAS partial-write semantics. | 15 + 30 min |

---

## Recommendation

**Revise preflight (R2) before any Phase-1 code work.** Both auditors explicitly say: do NOT reject TB-2 charter — runtime gap is real, accepted-path machinery exists, AcceptedLedger red line is correct. But do NOT proceed to Phase-1 until P0-A through P0-E are resolved in the preflight.

**Process**: per memory `feedback_elon_mode_policy` (round-cap=2), R2 is the budgeted second audit round. P0-A / P0-C / P0-D / P0-E are determinate-best surgical patches that can be applied without a second audit (auto-execute exception per same memory). **P0-B (`TaskId` vs `TxId`) is an architectural decision** — three named options, each with different downstream cost — and MUST be surfaced for explicit user decision before R2 audit launches.

**Suggested execution order**:

1. Surface P0-B options to user; get decision.
2. Apply P0-A / P0-C / P0-D / P0-E + the chosen P0-B variant + P1-A through P1-E to the preflight (single revision).
3. Bump preflight to v2; bump charter §5 acceptance battery to 16 tests; bump charter §8 ship proofs.
4. Optionally launch R2 dual audit on the revised preflight (recommended given conviction 5/5 from both r1 audits) — OR exercise the auto-execute exception if P0-B + P0-C resolutions are surgically clean.
5. Only then enter STEP_B Phase-1 (`git worktree add .claude/worktrees/stepb-tb2-sequencer-runtime-closure -b experiment/tb2-sequencer-runtime-closure`).

**Do NOT bypass.** Both audits at conviction 5/5 means the conservative-merged signal is unambiguous.
