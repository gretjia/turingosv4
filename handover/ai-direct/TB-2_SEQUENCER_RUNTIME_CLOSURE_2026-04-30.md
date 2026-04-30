# STEP_B Preflight — TB-2 Sequencer Runtime Closure

**Date**: 2026-04-30 (rev v2 same day, post Phase-0 r1 dual audit)
**TB**: TB-2 ("P1/P3 Runtime Boundary Closure + RSP-1")
**Charter**: `handover/tracer_bullets/TB-2_charter_2026-04-30.md`
**Protocol**: `handover/ai-direct/STEP_B_PROTOCOL.md`
**Audit history**:
- v1 (commit `3f06d51`) → Phase-0 r1 dual audit `handover/audits/DUAL_AUDIT_TB_2_PHASE0_VERDICT_R1_2026-04-30.md` → CHALLENGE / 5/5 (both Codex + Gemini).
- v2 (this revision) addresses 5 P0s + 5 P1s from r1 verdict. P0-B `TaskId` vs `TxId` resolution = **option (a) — bridge at lookup site** (user decision 2026-04-30).

---

## 0. Why STEP_B applies here

`STEP_B_PROTOCOL.md §0` scope (line 3, verbatim): *"any change to files in CLAUDE.md's restricted list (currently `src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`). Also applicable to any proposal that touches 'institution' per C-031."*

`src/state/sequencer.rs` qualifies under STEP_B line 3's institutional-touch clause:

- it is the **runtime wtool gate** — every accepted state transition is committed by `Sequencer::apply_one` (the only writer that mutates `state_root_t` / `ledger_root_t` / accepted `logical_t`);
- TB-2 changes its **error-path semantics** (rejected tx must now produce L4.E side effects), its **queue payload type** (`Sender<TypedTx>` → `Sender<SubmissionEnvelope>`), and its **constructor surface** (`Sequencer` gains a `rejection_writer` field — see §3 P0-A), all institutional-class changes;
- a regression here breaks the L4 / L4.E split that TB-1 paid 7 days to establish.

C-031 (`cases/C-031_institution_over_tuning.yaml:16,18`) provides policy support for the rule "Build right, then tune" / "先确认制度 (规则/约束/结构) 正确，再调参数" but is *not* itself a path-rule authorization — STEP_B line 3's catch-all is the operative trigger. Per Gemini r1 P1-A, `src/state/sequencer.rs` is also being added to CLAUDE.md's literal restricted list in this revision (see §11 hygiene patches) so future LLM agents do not need to re-derive applicability from case law.

Treat sequencer.rs edits with full STEP_B rigor. Disagreement → conservative verdict (require) per `feedback_dual_audit_conflict`.

---

## 1. Target

| File | Role | Touched by TB-2 |
|---|---|---|
| `src/state/sequencer.rs` | L4 sequencer + `dispatch_transition` + `apply_one` driver | **YES** (primary) — adds `rejection_writer` field (P0-A); fills `TypedTx::Work` arm of `dispatch_transition`; rewrites `apply_one` error path |
| `src/state/q_state.rs` | `QState` snapshot type | possibly (deterministic interim `state_root_t` mutation helper if it reduces sequencer.rs diff) |
| `src/state/typed_tx.rs` | `TypedTx` enum + `TransitionError` enum | **YES (limited)** (P0-D) — add `TransitionError::EscrowMissing` + `TransitionError::PostInitMint` + `TransitionError::StaleParentRoot` variants ONLY (NO new `TypedTx` variants — `task_open_tx` / `escrow_lock_tx` / `yes_stake_tx` remain TB-3 scope). New `TransitionError` variants are exhaustive-match additions only; no behaviour change for variants already wired. |
| `src/sdk/tools/wallet.rs` | wtool | **NO** (no widening) |
| `src/bus.rs` / `src/kernel.rs` | bus / kernel | **NO** |
| `src/economy/ledger.rs::AcceptedLedger` | TB-1 RSP-0 primitive wrapper | **NO** (stays a primitive; not used as production accepted spine) |
| `src/bottom_white/ledger/transition_ledger` + `LedgerWriter` | canonical L4 | invoked through existing API only (no signature change) |
| `src/bottom_white/ledger/rejection_evidence.rs` | L4.E writer | invoked through existing `append_rejected` only; `Sequencer` gains an `Arc<RejectionEvidenceWriter>` field (P0-A) |
| `src/economy/escrow_vault.rs` | task-keyed `EscrowVault` | **NO** (per P0-B option (a) — runtime uses `EconomicState.{escrows_t, task_markets_t}` via the in-arm bridge, NOT `EscrowVault` directly; `EscrowVault` remains the future TB-3+ truth source) |
| `tests/tb_2_runtime_boundary.rs` | TB-2 acceptance battery (integration-test surface) | **NEW** |
| `src/state/sequencer.rs` `#[cfg(test)] mod tb2_runtime_boundary` | TB-2 unit tests for `pub(crate)` API (P0-C) | **NEW** (in-crate tests for `apply_one` envelope plumbing + private-API checks) |

---

## 2. Why the change is necessary (Phase-0 brief for external audit)

**Observable behavior broken at HEAD `3f06d51`** (verified by Codex r1 line-by-line):

1. `Sequencer::dispatch_transition` returns `TransitionError::NotYetImplemented` for every `TypedTx` variant — match arms at `src/state/sequencer.rs:54-60`. No real WorkTx ever produces a `q_next`. The accepted L4 spine is therefore unexercised at runtime; the L4.E spine is unreachable from real submissions.
2. `Sequencer::apply_one` (`src/state/sequencer.rs:332`) calls `dispatch_transition(...)?` at `:339, :346` — any transition error propagates via the `?` operator and `apply_one` returns `Err(ApplyError::Transition(e))` without writing CAS, signing, committing, or appending L4.E. The `log::debug!("sequencer apply_one rejected: {e}")` happens in `Sequencer::run` at `:308, :313` AFTER `apply_one` returns the error — i.e. the rejection is logged once at the driver loop, not zero times, but the L4.E primitive is still untouched.
3. `Sequencer::submit` allocates `submit_id` at `src/state/sequencer.rs:292` and returns it in `SubmissionReceipt`, but `try_send` at `:293` carries only `tx`. `apply_one(:332)` receives `TypedTx`, not an envelope. The L4.E identity contract (rejected evidence is `submit_id`-keyed, never `logical_t`-keyed) is unenforceable end-to-end.
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

Day-1 of any production-code work must NOT exceed the six items below. Each item lists the v1 → v2 r1-driven amendments inline.

### 3.1 `SubmissionEnvelope` plumbing (Atom 2; queue payload type change)

```rust
// src/state/sequencer.rs (new)
#[derive(Debug)]
pub(crate) struct SubmissionEnvelope {
    pub submit_id: u64,
    pub tx: TypedTx,
}
```

- `Sequencer.queue_tx: Sender<TypedTx>` → `Sender<SubmissionEnvelope>` (`src/state/sequencer.rs:236`).
- `Sequencer::new` channel allocation (`:271`) updated; constructor signature unchanged for callers other than the queue type.
- `Sequencer::run(rx)` (`:304-:316`) takes `Receiver<SubmissionEnvelope>`; `apply_one(envelope)` (`:332`) is rewritten to consume the envelope.
- `Sequencer::submit(tx)` (`:291`) constructs `SubmissionEnvelope { submit_id, tx }` before `try_send` at `:293`. Public `submit()` signature unchanged — still `async fn submit(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError>`. The `submit_id` allocated by `fetch_add(:292)` is reused unchanged in the envelope (no second counter).

**P1-C — `SubmissionEnvelope` vs tuple `(u64, TypedTx)`**: a tuple changes the same channel/run/apply_one surface (`:236, :271, :304, :332`) so it is not a smaller diff. Named struct wins on (i) extensibility — TB-3 will likely add `submitter_id` / `timestamp_logical` / `epoch` fields without re-naming the type; (ii) clarity at every match site (no positional `.0 / .1` access); (iii) future ABI versioning (struct can `#[non_exhaustive]`; tuple cannot). No fields beyond `{submit_id, tx}` are added in TB-2.

**P1-D — submit-id concurrency contract**: `next_submit_id.fetch_add(1, SeqCst)` at `:292` happens BEFORE `try_send(:293)`. Under multi-producer contention a producer may allocate ID `n` and another may allocate ID `n+1` and `try_send` first; queue arrival order at `Sequencer::run` is **NOT** monotonic in `submit_id`. Tests MUST NOT assert "queue order = submit_id order". The receipt-side guarantee TB-2 establishes is: "the `submit_id` returned to the caller equals the `envelope.submit_id` that `apply_one` consumes for the same submission" — i.e. **per-submission identity preservation**, not cross-submission ordering. `submit_queue_full_consumes_submit_id` (battery test #14, see §5) further asserts that a failed `try_send` still burns its `submit_id` (no ID reuse), so monotonicity-over-allocations holds even when allocations-over-arrivals does not.

### 3.2 `Sequencer.rejection_writer` ownership (P0-A — writer ownership disclosed)

```rust
// src/state/sequencer.rs (new field)
pub struct Sequencer {
    next_submit_id: AtomicU64,
    next_logical_t: AtomicU64,
    queue_tx: tokio::sync::mpsc::Sender<SubmissionEnvelope>,
    cas: Arc<CasStore>,
    keypair: Arc<SystemKeypair>,
    epoch: u64,
    ledger_writer: Arc<dyn LedgerWriter>,
    rejection_writer: Arc<RejectionEvidenceWriter>,  // P0-A — NEW
    predicates: Arc<PredicateRegistry>,
    tools: Arc<ToolRegistry>,
    q: Arc<RwLock<QState>>,
}
```

- `Sequencer::new(...)` gains a `rejection_writer: Arc<RejectionEvidenceWriter>` constructor parameter, positioned immediately after `ledger_writer` to mirror their semantic pair (canonical L4 / L4.E).
- A `pub(crate) fn rejection_writer_for_test(&self) -> Arc<RejectionEvidenceWriter>` accessor is added so the new in-crate `#[cfg(test)] mod tb2_runtime_boundary` (see §5.1) can read row counts and reconstruct `PublicRejectionView`.
- All existing `Sequencer::new(...)` call sites are updated to pass an `Arc::new(RejectionEvidenceWriter::default())` (or a freshly-constructed in-memory writer) in tests; production call sites get the same shape — `RejectionEvidenceWriter` is currently in-memory per its own docs (`src/bottom_white/ledger/rejection_evidence.rs:30, 34`), so no new persistence wiring is incurred in TB-2. (Persistence semantics for L4.E are deferred per `rejection_evidence.rs` comments and CHL-S5 in r1 verdict.)

### 3.3 `dispatch_transition` `TypedTx::Work` arm — pure validation

The arm is filled with **pure** validation (no side effects, no I/O, no writer calls). On accept it returns `Ok((q_next, signals))`; on reject it returns `Err(TransitionError::<specific class>)`.

Validation steps (in order; first-failure short-circuits):

1. **Parent-root match**: `if tx.parent_state_root != q.state_root_t { return Err(TransitionError::StaleParentRoot); }`.
2. **Acceptance predicate bundle**: every entry in `tx.predicate_results.acceptance` is `true` (else `Err(TransitionError::PredicateFailed)`).
3. **Settlement predicate bundle (if applicable to RSP-1)**: every entry is `true` or empty (else `Err(TransitionError::PredicateFailed)`).
4. **YES stake gate (RSP-1)**: `tx.stake > 0` (else `Err(TransitionError::StakeInsufficient)` — variant already exists at `src/state/typed_tx.rs:717`).
5. **Escrow presence gate (RSP-1, P0-B option (a) — bridge at lookup site)**:

   ```rust
   // P0-B option (a): in-arm deterministic bridge from TaskId to TxId namespace.
   // TB-3 introduces formal task_open_tx / escrow_lock_tx / yes_stake_tx variants
   // that allocate proper TxIds at submission time; this bridge is then deleted.
   let lookup_tx_id = TxId(tx.task_id.0.clone());
   let has_escrow = q.economic_state_t.escrows_t.contains_key(&lookup_tx_id)
                  || q.economic_state_t.task_markets_t.contains_key(&lookup_tx_id);
   if !has_escrow {
       return Err(TransitionError::EscrowMissing);
   }
   ```

   **Rationale**: `WorkTx.task_id: TaskId` (`src/state/typed_tx.rs:225, src/state/typed_tx.rs:33-35`) but `EconomicState.{escrows_t, task_markets_t}: BTreeMap<TxId, ...>` (`src/state/q_state.rs:161, 224`). The bridge is a single line whose only failure mode is being deleted in TB-3 when real `task_open_tx` lands and produces `TxId` directly. The task-keyed `EscrowVault` (`src/economy/escrow_vault.rs`) is intentionally NOT used here — it is a separate truth source ("distinct from `state::q_state::EscrowEntry`" per its own docs at `:15, :53, :146, :168`); TB-2 keeps `EscrowVault` as the future TB-3+ unification target and reads only from `q.economic_state_t` at the runtime spine to preserve the single-truth-source contract.

6. **Monetary invariants** (all three; production call sites pass `&[]` exempt list per §3.5):
   - `monetary_invariant::assert_no_post_init_mint(tx, q)` → maps to `Err(TransitionError::PostInitMint)` on violation.
   - `monetary_invariant::assert_read_is_free(TxKind::Work, 0)` → infallible for `TxKind::Work` with zero read cost; included for symmetry / future-proofing.
   - `monetary_invariant::assert_total_ctf_conserved(q.economic_state_t, q_next.economic_state_t, &[])` → maps to `Err(TransitionError::InvariantViolation)` on violation.

### 3.4 `q_next.state_root_t` — interim domain-separated hash (P1-E — domain constant)

```rust
// src/state/sequencer.rs or src/state/q_state.rs (new constant)
/// TB-2 interim WorkTx-accept state-root domain. Real patch semantics land in P5.
/// MUST be registered in genesis_payload.toml [trust_root.domains] before
/// merging Phase-1c (per Trust Root manifest discipline).
pub(crate) const WORKTX_ACCEPT_DOMAIN_V1: &[u8] = b"turingosv4.worktx.accept.v1";
```

On accept:

```rust
let work_tx_digest = canonical_hash(tx);  // existing helper or sha256 of canonical bytes
q_next.state_root_t = sha256_concat(
    WORKTX_ACCEPT_DOMAIN_V1,
    q.state_root_t.as_bytes(),
    work_tx_digest.as_bytes(),
);
```

The TB-1 toy domain `turingosv4.l4_state_root.v1` (`src/economy/ledger.rs:350, 357`) belongs to `AcceptedLedger` and is NOT reused here — that would conflate the TB-1 primitive's hash domain with the production state-root mutator. Both domains coexist until `AcceptedLedger` retires.

`q_next.economic_state_t` reflects stake/escrow/balances delta as required by the monetary invariants (concrete delta semantics handled by existing `EconomicState` helpers — no new APIs).

### 3.5 `apply_one` rejection-writer error path

`apply_one(envelope)` semantics on `Err(e)` from `dispatch_transition`:

```rust
if let Err(e) = dispatch_result {
    // 1. CAS-put canonical-encoded tx payload (orphan-CAS note: this object is
    //    durable even if the next step fails — see §3.6).
    let tx_payload_cid = self.cas.put(canonical_encode(&envelope.tx))?;

    // 2. Optionally CAS-put diagnostic. raw_diagnostic_cid is structurally
    //    serde-shielded by TB-1 P0-3 on RejectedSubmissionRecord — but the
    //    runtime path's exposure is RE-CONFIRMED by battery test #15.
    let raw_diagnostic_cid = Some(self.cas.put(e.to_string().into_bytes())?);

    // 3. Append to L4.E. Currently in-memory + infallible; persistence deferred.
    self.rejection_writer.append_rejected(
        envelope.submit_id,
        q_snapshot.state_root_t,
        envelope.tx.submitter_id(),  // HasSubmitter trait, src/state/typed_tx.rs:631
        envelope.tx.tx_kind(),       // src/state/typed_tx.rs:620
        tx_payload_cid,
        rejection_class_for(&e),     // P0-D — see §3.7 mapping table
        raw_diagnostic_cid,
        public_summary_for(&e),      // small derived string, no raw payload
    );

    // 4. Return without advancing logical_t / state_root_t / ledger_root_t.
    return Err(ApplyError::Transition(e));
}
```

Accepted path stays on the **existing** `transition_ledger` + `LedgerWriter` flow already wired into `apply_one`'s success arm at `src/state/sequencer.rs:351, :377, :386, :423` — TB-2 does not introduce a new ledger writer and does not change the accepted-side commit sequence.

### 3.6 Orphan-CAS partial-write contract (P1-E)

CAS `put` is durable as soon as it returns `Ok` (`src/bottom_white/cas/store.rs:162, 195`). If a later step in `apply_one` fails (rejection: L4.E append; accepted: writer commit), the already-written CAS object becomes an orphan. TB-2 contract:

- Orphan-CAS objects are content-addressed; identical re-submission produces an identical CID and re-uses the existing object (no duplication).
- Orphan-CAS objects are tolerable in TB-2 because both rejection-path L4.E append (`src/bottom_white/ledger/rejection_evidence.rs:30, 34, 258, 268`) and accepted-path writer commit are currently in-memory or single-commit Git2 operations — partial-write windows are narrow and bounded.
- Orphan-CAS GC / reachability semantics are deferred to a later TB (likely co-with L4.E persistence). TB-2 does NOT add a GC pass.

### 3.7 `TransitionError → RejectionClass` mapping (P0-D)

Existing types:

- `TransitionError` variants today (`src/state/typed_tx.rs:717`): `StakeInsufficient`, `TaskNotFound`, `NotYetImplemented`, ... (verified in r1; full list audited).
- `RejectionEvidence::RejectionClass` variants today (`src/bottom_white/ledger/rejection_evidence.rs:56-67`): `PredicateFailed`, `PolicyViolation`, `EscrowMissing`, `InvariantViolation`.

TB-2 adds three `TransitionError` variants (only — no other typed_tx.rs changes):

- `TransitionError::StaleParentRoot` (new) — `tx.parent_state_root != q.state_root_t`.
- `TransitionError::EscrowMissing` (new) — escrow / task-market lookup missed.
- `TransitionError::PostInitMint` (new) — `assert_no_post_init_mint` violation surfaced as a transition error class.

Mapping table (`fn rejection_class_for(e: &TransitionError) -> RejectionClass`):

| `TransitionError` | `RejectionClass` |
|---|---|
| `StaleParentRoot` (new) | `PolicyViolation` |
| `PredicateFailed` (existing or alias) | `PredicateFailed` |
| `StakeInsufficient` (existing) | `PolicyViolation` |
| `EscrowMissing` (new) | `EscrowMissing` |
| `PostInitMint` (new) | `InvariantViolation` |
| `InvariantViolation` (existing or new alias) | `InvariantViolation` |
| `NotYetImplemented` (existing — not expected on WorkTx arm post-TB-2) | `PolicyViolation` |
| `TaskNotFound` (existing — not raised by WorkTx arm post-TB-2 because escrow gate fires first) | `PolicyViolation` |

The mapping is closed (no `_ =>` wildcard). Adding any new `TransitionError` variant in a future TB MUST extend this table and adjust the battery's expected names.

### 3.8 Untouched arms

`TypedTx::Verify`, `TypedTx::Challenge`, `TypedTx::Reuse`, `TypedTx::FinalizeReward`, `TypedTx::TaskExpire`, `TypedTx::TerminalSummary` all stay `Err(TransitionError::NotYetImplemented)` and are out of TB-2 scope.

**Scope creep guard**: any line of code outside §3.1–§3.7 (production-code §) constitutes a separate atom and must be extracted into TB-3+ unless the auditors explicitly approve it in Phase-1c. The only files touched by §3.1–§3.7 are `src/state/sequencer.rs` (primary), `src/state/typed_tx.rs` (3 new `TransitionError` variants only), and optionally `src/state/q_state.rs` (state-root domain constant, if not co-located in `sequencer.rs`).

---

## 4. Parallel-branch plan (Phase 1)

### A branch — `experiment/tb2-sequencer-runtime-closure`

```bash
git worktree add .claude/worktrees/stepb-tb2-sequencer-runtime-closure -b experiment/tb2-sequencer-runtime-closure
```

Implements §3 minimum-sufficient version. Acceptance battery is split between in-crate unit tests (`#[cfg(test)] mod tb2_runtime_boundary` inside `src/state/sequencer.rs` — for `pub(crate)` API checks) and integration tests (`tests/tb_2_runtime_boundary.rs` — for behaviour through `Sequencer::submit`). Total 16 tests across both surfaces (see §5). Each test added in red→green order; commit boundaries respect `feedback_phased_checkpoint` (paired N=20 not applicable — runtime spine is deterministic per submission, no LLM in the loop yet).

### B branch — baseline (control)

`main @ <last-PASS HEAD>` (currently `3f06d51` — TB-2 Day-1 docs commit). No code change. The acceptance battery is run on B as a control to confirm it produces zero rows of L4.E and zero `state_root_t` advance from real `Sequencer::submit` traffic (because no `WorkTx` survives `NotYetImplemented`). This is the "before" snapshot.

### Acceptance gate

A is merge-eligible only if all of:

1. `cargo check --workspace` clean on A.
2. `cargo test --workspace` green on A (including all pre-existing tests).
3. `cargo test --test tb_2_runtime_boundary` + the in-crate `tb2_runtime_boundary` mod in `sequencer.rs` together produce **16/16 green** on A.
4. Same combined battery produces **16/16 FAIL on B** (deterministic A/B asymmetry: every test depends on the WorkTx arm not returning `NotYetImplemented` and/or on the rejection writer being wired).
5. `cargo test --workspace` on B baseline-suite (pre-TB-2 tests) stays green.
6. Diff confined to §1 `Touched=YES` rows: `src/state/sequencer.rs` (primary), `src/state/typed_tx.rs` (3 new `TransitionError` variants ONLY — no new `TypedTx` variants), optionally `src/state/q_state.rs` (state-root domain constant ONLY if co-locating in sequencer.rs is awkward), `tests/tb_2_runtime_boundary.rs` (new). Any edit outside this surface fails Phase-1c review.
7. Dual external audit on diff (Phase-1c) returns PASS / PASS. VETO from either auditor blocks merge until addressed; CHALLENGE → conservative.
8. Two ship proofs (charter §8) demonstrable in fixture: predicate-failed WorkTx via `Sequencer::submit` → exactly one L4.E row + zero `state_root_t` / `ledger_root_t` / `logical_t` change; predicate-passing WorkTx with stake+escrow → `state_root_t` + `ledger_root_t` + accepted `logical_t` advance + zero L4.E rows. Plus the two replay proofs added in v2 §5 (test 16 split).

A FAIL on any of the above → branch abandoned or revised; charter must change before retry (TB methodology v2 no-same-charter-retry rule).

---

## 5. Acceptance battery (16 tests; split unit + integration per P0-C)

### 5.1 In-crate unit tests — `src/state/sequencer.rs` `#[cfg(test)] mod tb2_runtime_boundary`

These tests need access to `pub(crate)` API (`apply_one`, `dispatch_transition`, `SubmissionEnvelope`) and live inside the crate. They drive `apply_one` directly with constructed envelopes; they do NOT go through `Sequencer::submit + Sequencer::run`.

| # | Test | Asserts |
|---|---|---|
| **U1** | `apply_one_consumes_submission_envelope` | Compile-time check: `apply_one(envelope: SubmissionEnvelope)` is the canonical signature. Constructs a synthetic envelope and asserts the call typechecks. Replaces v1 Test 2. |
| **U2** | `apply_one_rejected_path_uses_envelope_submit_id` | Drive `apply_one` with an envelope carrying `submit_id = 42` and a WorkTx that fails predicate validation. Read `rejection_writer_for_test()` and assert the new L4.E row's `submit_id == 42`. |
| **U3** | `dispatch_transition_worktx_returns_state_root_via_domain_v1` | Call `dispatch_transition` directly with a predicate-passing WorkTx + stake>0 + seeded escrow. Assert `q_next.state_root_t == sha256(WORKTX_ACCEPT_DOMAIN_V1 ‖ q.state_root_t ‖ canonical_hash(tx))` exactly (proves the interim domain hash is what TB-2 ships, not a different scheme). |

### 5.2 Integration tests — `tests/tb_2_runtime_boundary.rs`

These tests go through `Sequencer::submit` (the public path) and exercise the full driver loop via a single-poll `Sequencer::run` step. They need only `pub` API.

#### Submit-id plumbing

| # | Test | Asserts |
|---|---|---|
| **I1** | `submit_returns_receipt_and_envelope_submit_id_matches` | The `submit_id` returned by `submit()` matches the `submit_id` keyed in the resulting L4 row (accept) or L4.E row (reject). Replaces v1 Test 1. |
| **I2** | `submit_queue_full_consumes_submit_id` | Saturate the queue (size known from `Sequencer::new` config), call `submit()` once more — expect `Err(SubmitError::QueueFull)`. Drain one slot, `submit()` again. Assert the successful `submit_id` is `n+2`, not `n+1` — the failed `try_send` still burned ID `n+1`. **Locks the contract that `submit_id` is allocated atomically before `try_send` and is NEVER reused, even on `try_send` failure.** Battery test #14 from r1 P0-E. |

#### Rejection spine (proof 1)

| # | Test | Asserts |
|---|---|---|
| **I3** | `runtime_predicate_failed_worktx_appends_l4e` | Submit a WorkTx whose `predicate_results.acceptance` contains a `false`. Expect: 1 L4.E row with matching `submit_id`, `rejection_class == PredicateFailed`. |
| **I4** | `runtime_stale_parent_worktx_appends_l4e` | Submit a WorkTx with `parent_state_root != q.state_root_t`. Expect: 1 L4.E row, `rejection_class == PolicyViolation` (mapped from `TransitionError::StaleParentRoot` per §3.7). Battery test #13 from r1 P0-E. |
| **I5** | `runtime_stakeless_worktx_appends_l4e` | Submit a WorkTx with `stake == 0`. Expect: 1 L4.E row, `rejection_class == PolicyViolation` (mapped from `StakeInsufficient`). |
| **I6** | `runtime_no_escrow_worktx_appends_l4e` | Submit a WorkTx for a `task_id` whose bridged `TxId(task_id.0.clone())` has no entry in either `q.economic_state_t.escrows_t` or `task_markets_t`. Expect: 1 L4.E row, `rejection_class == EscrowMissing`. |
| **I7** | `runtime_rejected_worktx_does_not_advance_logical_t_or_state_root` | Across I3-I6, accepted `logical_t` is unchanged AND `state_root_t` is unchanged AND `ledger_root_t` is unchanged. Merges v1 tests 7+8 since they're observed at the same site. |
| **I8** | `runtime_l4e_public_view_honors_serde_shield` | Drive an I3 rejection. Retrieve the L4.E record via `RejectionEvidenceWriter`'s public-view API. Assert `serde_json::to_string(&public_view)` does NOT contain the substring of `raw_diagnostic_cid`'s value (it's `#[serde(skip_serializing)]`-shielded per TB-1 P0-3). **Re-confirms TB-1 P0-3 at the runtime path, not just the primitive.** Battery test #15 from r1 P0-E. |

**Note on Test 6 (post-init mint via WorkTx) — DROPPED from runtime battery.** Per Codex r1 CHL-S3: WorkTx carries no economic-delta field; mint-via-WorkTx is not a representable transition. The post-init mint invariant is already proven at the primitive level by TB-1's `assert_no_post_init_mint` unit tests in `src/economy/monetary_invariant.rs::tests` and re-confirmed at runtime by I7's "no state advance on reject" property — any `q_next` that violates supply conservation would either (i) be impossible to construct (the WorkTx arm computes `q_next` itself; any path that would mint requires an attacker to supply a malformed `q_next` directly, which the runtime never accepts) or (ii) be caught by `assert_total_ctf_conserved(..., &[])` and routed to L4.E with `InvariantViolation`. If/when a future TB introduces a `TypedTx` variant that CAN carry a supply delta (e.g. RSP-2's `settlement_tx`), the runtime post-init mint test moves into THAT TB's battery.

#### Acceptance spine (proof 2)

| # | Test | Asserts |
|---|---|---|
| **I9** | `runtime_accepted_worktx_advances_state_root_via_domain_v1` | Submit a predicate-passing WorkTx with stake+escrow. `state_root_t` differs from the pre-submit snapshot AND equals `sha256(WORKTX_ACCEPT_DOMAIN_V1 ‖ prev_state_root ‖ canonical_hash(tx))` exactly (cross-checks U3 at the integration layer). |
| **I10** | `runtime_accepted_worktx_advances_ledger_root` | After I9, `ledger_root_t` differs (canonical `transition_ledger` advanced). |
| **I11** | `runtime_accepted_worktx_increments_logical_t` | After I9, accepted `logical_t == prev + 1`. |
| **I12** | `runtime_accepted_worktx_does_not_append_l4e` | After I9, L4.E row count is unchanged. |

#### Replay invariant (P1:8 — battery test #16 from r1 P0-E)

| # | Test | Asserts |
|---|---|---|
| **I13** | `runtime_replay_from_l4_only_ignores_l4e` | Submit one accepted WorkTx (I9-class) and one rejected WorkTx (I3-class) through the same `Sequencer`. Capture pre-replay `state_root_t`. Reconstruct `QState` from the canonical `transition_ledger` ONLY (using existing `replay_full_transition` machinery in `src/bottom_white/ledger/transition_ledger.rs:371, 389, 442, 486`). Assert reconstructed `state_root_t` equals the sequencer's post-submission `state_root_t` AND that no L4.E record influenced the reconstruction (replay ignores `RejectionEvidenceWriter` entirely). **Proves P1:8 / Art IV Boot — state.db is reconstructible from L4 alone.** |

### 5.3 Test fixtures

`tests/common/runtime_fixtures.rs` (or inline if helpers fit in one file):

- `seed_economic_state_with_escrow(task_id: TaskId, bounty: u64) -> EconomicState` — seeds `escrows_t.insert(TxId(task_id.0.clone()), ...)` per the §3.3 step-5 bridge so I3/I4/I5/I9 all share construction.
- `make_worktx(opts: WorkTxFixtureOpts) -> TypedTx::Work` — opts cover `parent_state_root` (test I4 sets a stale value; others use `q.state_root_t`), `predicate_results.acceptance` (I3 sets at least one `false`; others all-true), `stake` (I5 sets 0), `task_id` (I6 picks an unseeded id). Drops the `supply-delta-injection` field from v1 (no longer needed since Test 6 is dropped).
- `assert_l4e_row_matches(writer: &RejectionEvidenceWriter, submit_id: u64, expected_class: RejectionClass)` — single-row count + rejection_class match; uses `rejection_writer_for_test()` accessor.
- `assert_l4e_row_count(writer: &RejectionEvidenceWriter, expected: usize)` — for I7 and I12.
- `replay_state_root_from_l4(ledger_writer: &dyn LedgerWriter) -> StateRoot` — uses existing `transition_ledger::replay_full_transition` to reconstruct from L4 only; fixture for I13.

Helpers MUST be test-only; they MUST NOT leak into `src/`.

---

## 6. Frozen analyzer (Phase 2)

TB-2 is a runtime-spine correctness change, not a population-statistics A/B. The pre-registered decision rule is **deterministic 16/16 PASS** across the combined battery (3 in-crate unit + 13 integration tests; see §5.1 / §5.2), not a SolveRate delta. `frozen_analysis.py` is **not invoked** for TB-2 acceptance (no LLM-in-the-loop sample). The STEP_B Phase-2 `--control`/`--treatment` machinery is therefore replaced by:

- A: `cargo test --lib state::sequencer::tb2_runtime_boundary` (3/3 PASS) + `cargo test --test tb_2_runtime_boundary` (13/13 PASS) → 16/16 PASS overall.
- B (baseline): both commands → 16/16 FAIL (runtime path is `NotYetImplemented`; no `RejectionEvidenceWriter` field on `Sequencer`).

This A/B asymmetry is itself the empirical signal. Both runs pre-registered.

---

## 7. Verdict and merge path (Phase 3)

| Verdict | Action |
|---|---|
| A 16/16 PASS + B 16/16 FAIL + both auditors PASS on diff | `git merge experiment/tb2-sequencer-runtime-closure --no-ff` on `main`; update `TB_LOG.tsv` row TB-2 status `active → shipped` with `ship_commits` range; update `AUTO_RESEARCH_NOTEPAD.md`; update `ROADMAP_9_PHASE_2026-04-29.md` P1 Exit 5/6/9 + P3 Exit 3/5 to green. |
| A < 16/16 PASS or B not 16/16 FAIL | abandon branch (`git branch -D experiment/tb2-sequencer-runtime-closure` or archive); write `handover/alignment/OBS_TB-2_FAILED.md` per TB methodology v2; new charter required before retry. |
| Auditors split (PASS / VETO) | conservative wins → block. |
| Auditors split (PASS / CHALLENGE) | resolve CHALLENGE → re-audit. |
| Auditors agree CHALLENGE | merge CHALLENGE → re-audit; do not merge to main while CHALLENGE is open. |

Cleanup: `git worktree remove .claude/worktrees/stepb-tb2-sequencer-runtime-closure`. If branch archived: `git tag archive/tb2-sequencer-runtime-closure_2026-MM-DD experiment/tb2-sequencer-runtime-closure` then delete branch.

---

## 8. Forbidden in this STEP_B (red lines)

Per TB-2 charter §5, repeated here for the auditors:

1. No ledger I/O (CAS put, writer commit, ledger append) inside `dispatch_transition`. The function returns `(q_next, signals)` or `Err(TransitionError)` only.
2. No use of `economy::ledger::AcceptedLedger::append_accepted` in the production accepted spine. `AcceptedLedger` stays a TB-1 primitive / test wrapper.
3. No new `TypedTx` variants. `task_open_tx` / `escrow_lock_tx` / `yes_stake_tx` are reserved for TB-3. **Three new `TransitionError` variants are permitted** (`StaleParentRoot`, `EscrowMissing`, `PostInitMint`) per §3.7 mapping table — these are exhaustive-match-completeness additions, not new economic types.
4. No non-empty `exempt_tx_kinds` argument at the runtime call sites of `assert_total_ctf_conserved`. Production must pass `&[]`.
5. No widening of `WalletTool` mutation surface.
6. No P5/P6/h_vppu/capability-metric work inside this STEP_B branch.
7. No edits to `src/kernel.rs` / `src/bus.rs` / `src/sdk/tools/wallet.rs` (the formal STEP_B-restricted set per CLAUDE.md). If TB-2 implementation discovers a real need to touch any of those, halt and open a *separate* STEP_B preflight before continuing.
8. No use of `EscrowVault` (`src/economy/escrow_vault.rs`) inside the WorkTx-arm escrow lookup. Per P0-B option (a), runtime reads from `q.economic_state_t.{escrows_t, task_markets_t}` only, via the in-arm `TxId(tx.task_id.0.clone())` bridge. `EscrowVault` remains the TB-3+ unification target; a second escrow truth source on the runtime spine is forbidden in TB-2.
9. Bridge line `let lookup_tx_id = TxId(tx.task_id.0.clone())` MUST carry an inline `// TB-2 P0-B option (a): drop this when task_open_tx lands in TB-3` comment — the bridge is intentionally short-lived; failure to mark it for deletion creates exactly the kind of debt the audit flagged.

A diff that violates any of these auto-fails Phase-1c review even if `cargo test` is green.

---

## 9. Pointers

- TB-2 charter: `handover/tracer_bullets/TB-2_charter_2026-04-30.md`
- STEP_B protocol: `handover/ai-direct/STEP_B_PROTOCOL.md`
- Restricted-file path correction (path drift OBS): `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md`
- TB-1 ship row + narrowed claim: `handover/tracer_bullets/TB_LOG.tsv` (TB-1 row, ship_commits `063b003..ccb01fa`)
- TB-1 dual-audit verdict: `handover/audits/DUAL_AUDIT_TB_1_VERDICT_2026-04-29.md`
- TB-2 Phase-0 r1 dual-audit verdict: `handover/audits/DUAL_AUDIT_TB_2_PHASE0_VERDICT_R1_2026-04-30.md` (drove this v2 revision)
- TB-2 Phase-0 r1 individual audits: `handover/audits/CODEX_TB_2_PHASE0_AUDIT_2026-04-30.md` + `handover/audits/GEMINI_TB_2_PHASE0_AUDIT_2026-04-30.md`
- 9-phase canonical roadmap: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` (P1 / P3 Current state refreshed 2026-04-30)
- Memory: `feedback_step_b_protocol`, `feedback_dual_audit`, `feedback_dual_audit_conflict`, `feedback_phased_checkpoint`, `feedback_smoke_before_batch`, `feedback_no_fake_menus`, `feedback_session_label_codification`, `feedback_elon_mode_policy`.

---

## 10. v1 → v2 changelog (r1 audit response)

| r1 Finding | Resolution in v2 | Section(s) |
|---|---|---|
| **P0-A** Sequencer has no L4.E writer field | `Sequencer.rejection_writer: Arc<RejectionEvidenceWriter>` field declared; constructor parameter; `rejection_writer_for_test()` accessor for in-crate tests. | §3.2 |
| **P0-B** TaskId vs TxId mismatch | Option (a) chosen — inline bridge `TxId(tx.task_id.0.clone())` at the WorkTx-arm lookup site; `EscrowVault` not used; bridge is single-line and gets deleted in TB-3. Marked with deletion-target comment per §8 red line 9. | §3.3 step 5; §8 lines 8-9 |
| **P0-C** Battery not compile-expressible | Split into 3 in-crate unit tests (§5.1, for `pub(crate)` API access) + 13 integration tests (§5.2, through `Sequencer::submit`). Test 6 (post-init mint via WorkTx) DROPPED — WorkTx carries no economic-delta field, mint via WorkTx is not representable; primitive-level invariant remains green via TB-1 unit tests + I7 "no state advance on reject". | §5.1, §5.2 (Test 6 note) |
| **P0-D** Error / rejection-class mapping undefined | §3.7 mapping table added. Three new `TransitionError` variants (`StaleParentRoot`, `EscrowMissing`, `PostInitMint`) explicitly disclosed as `typed_tx.rs` edits in §1 + §8 line 3. Mapping is closed (no wildcard). | §1, §3.7, §8 |
| **P0-E** Battery missing 4 critical tests | I2 (`submit_queue_full_consumes_submit_id`), I4 (`runtime_stale_parent_worktx_appends_l4e`), I8 (`runtime_l4e_public_view_honors_serde_shield`), I13 (`runtime_replay_from_l4_only_ignores_l4e`) added. 12-test → 16-test battery; charter §8 ship proofs bumped to include I13 replay invariant. | §5.2 |
| **P1-A** sequencer.rs not in CLAUDE.md restricted list | Applied in same revision commit — see §11 hygiene patches. | §11 |
| **P1-B** §0 cited C-031 alone | §0 reworded to cite STEP_B line 3 directly; C-031 framed as policy support, not path authorization. | §0 |
| **P1-C** SubmissionEnvelope vs tuple rationale missing | §3.1 documents tuple equivalence + named-struct wins (extensibility, clarity, ABI versioning). | §3.1 |
| **P1-D** Concurrency note on submit_id ordering | §3.1 documents `fetch_add` precedes `try_send`; submit_id order is NOT arrival order; tests must not assert otherwise. I2 explicitly tests "failed try_send still burns submit_id". | §3.1, §5.2 (I2) |
| **P1-E** Unregistered state-root domain + orphan-CAS | §3.4 declares `WORKTX_ACCEPT_DOMAIN_V1` constant; §3.6 documents orphan-CAS partial-write contract. | §3.4, §3.6 |
| Cosmetic: HEAD reference `459c747` stale | Updated to `3f06d51` (TB-2 Day-1 docs commit) in §2 + §4-B. | §2, §4 |
| Cosmetic: "`apply_one ... log::debug!`s" wording | Corrected — `log::debug!` is in `Sequencer::run`, not `apply_one`. | §2 |

---

## 11. Hygiene patches applied alongside this v2 (P1-A)

Per Gemini r1 P1-A, `src/state/sequencer.rs` is added to CLAUDE.md's literal restricted-file list to prevent future LLM agents from re-deriving STEP_B applicability from C-031 case law:

```diff
 ## Code Standard (Art. I.1 + C-004 + C-027)
 - `cargo check` / `cargo test` 必过；`.env` 永不 commit
-- `src/{kernel,bus}.rs` + `src/sdk/tools/wallet.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
+- `src/{kernel,bus}.rs` + `src/sdk/tools/wallet.rs` + `src/state/sequencer.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
 - 任何影响行为的参数必须 env/config 可覆盖，不可硬编码
```

Equivalent edit applied to `handover/ai-direct/STEP_B_PROTOCOL.md` line 3 to keep the two restricted-file lists synchronized (per OBS path-drift policy at `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md`).

Both edits are committed in the same commit as this preflight v2 so the restricted-file list is current at the moment any TB-2 code work begins.
