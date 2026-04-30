# STEP_B Preflight — TB-2 Sequencer Runtime Closure

**Date**: 2026-04-30 (rev v3 same day, post Phase-0 r2 narrowed Codex audit)
**TB**: TB-2 ("P1/P3 Runtime Boundary Closure + RSP-1")
**Charter**: `handover/tracer_bullets/TB-2_charter_2026-04-30.md`
**Protocol**: `handover/ai-direct/STEP_B_PROTOCOL.md`
**Audit history**:
- v1 (commit `3f06d51`) → Phase-0 r1 dual audit `handover/audits/DUAL_AUDIT_TB_2_PHASE0_VERDICT_R1_2026-04-30.md` → CHALLENGE / 5/5 (both Codex + Gemini).
- v2 (commit `c5059a5`) addressed 5 P0s + 5 P1s from r1 verdict. P0-B `TaskId` vs `TxId` resolution = **option (a) — bridge at lookup site** (user decision 2026-04-30).
- v2 → narrowed Codex r2 (`handover/audits/CODEX_TB_2_PHASE0_R2_AUDIT_2026-04-30.md`) → CHALLENGE / 5/5: 6 substrate compile-shape blockers in v2's snippets (writer ownership type, `TaskId→TxId` map access, integration-test visibility, `TransitionError` enum mismatch, `submitter_id()` `Option<AgentId>` mismatch, CAS API 5-arg form). Gemini r2 NOT run — Gemini r1 was strategic-PASS on 7/8 questions; remaining live risks are substrate-class.
- v3 (this revision) addresses all 6 r2 P0s + 5 P1s with **API shapes verified against source at HEAD `c5059a5`**. Per memory `feedback_elon_mode_policy` round-cap=2 (now used), v3 takes the auto-execute exception: snippets are line-ref-grounded surgical patches; cargo check inside the STEP_B Phase-1 worktree is the operative verification.

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
| File | Role | Touched by TB-2 |
| --- | --- | --- |
| `src/state/sequencer.rs` | L4 sequencer + `dispatch_transition` + `apply_one` driver | **YES** (primary). Adds: `SubmissionEnvelope` struct, `WORKTX_ACCEPT_DOMAIN_V1` + `SYSTEM_AGENT_ID_STR` consts, `worktx_canonical_hash` / `rejection_class_for` / `public_summary_for` helper fns, `rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>` field, `pub fn try_apply_one(...)` test driver. Modifies: `Sequencer::new` signature (one new param + receiver type), `Sequencer::run` receiver type, `Sequencer::submit` queue payload, `dispatch_transition` WorkTx arm body, `apply_one` queue type + error path. The two existing `Sequencer::new` call sites at `:483, :627` get the new parameter. |
| `src/state/typed_tx.rs` | `TransitionError` enum + Display impl | **YES (limited)** (P0-D r1 + P0-4 r2). Adds **2** new `TransitionError` variants only: `EscrowMissing` + `MonetaryInvariantViolation`. Adds 2 corresponding Display arms in the exhaustive `Display` impl at `:790-816`. NO new `TypedTx` variants (revised down from r1's 3 — `StaleParent` already exists at `:720`). `task_open_tx` / `escrow_lock_tx` / `yes_stake_tx` remain TB-3 scope. |
| `src/state/q_state.rs` | `QState` snapshot type | **NO** (P1-1 r2 removed v2's "optionally q_state.rs"; constant home pinned to sequencer.rs in §3.4). |
| `src/sdk/tools/wallet.rs` | wtool | **NO** (no widening). |
| `src/bus.rs` / `src/kernel.rs` | bus / kernel | **NO**. |
| `src/economy/ledger.rs::AcceptedLedger` | TB-1 RSP-0 primitive wrapper | **NO** (stays a primitive; not used as production accepted spine). |
| `src/economy/monetary_invariant.rs` | RSP-0 invariants | **NO** (calls only — existing API; `&[]` exempt list per §3.5). |
| `src/bottom_white/ledger/transition_ledger.rs` + `LedgerWriter` | canonical L4 | invoked through existing API only (no signature change). I13 uses existing `pub fn replay_full_transition` at `:371`. |
| `src/bottom_white/ledger/rejection_evidence.rs` | L4.E writer | invoked through existing `append_rejected` (`&mut self` per `:258`); `Sequencer` gains an `Arc<RwLock<RejectionEvidenceWriter>>` field (P0-A r1 + P0-1 r2). |
| `src/bottom_white/cas/store.rs` | CAS | invoked through existing 5-arg `CasStore::put` at `:163-:170` (`&mut self`). |
| `src/economy/escrow_vault.rs` | task-keyed `EscrowVault` | **NO** (per P0-B option (a) — runtime uses `EconomicState.{escrows_t, task_markets_t}.0` via the in-arm bridge, NOT `EscrowVault` directly; `EscrowVault` remains the future TB-3+ truth source. Red line confirmed enforceable by r2 Q6: zero existing `src/state/` call sites). |
| `tests/tb_2_runtime_boundary.rs` | TB-2 integration acceptance battery (13 tests) | **NEW** (uses only `pub` API; retains `Arc<RwLock<RejectionEvidenceWriter>>` clone passed to `Sequencer::new` for L4.E observation). |
| `src/state/sequencer.rs` `#[cfg(test)] mod tb2_runtime_boundary` | TB-2 in-crate unit tests (3 tests) | **NEW** (`pub(crate)` API access for envelope/dispatch_transition signature checks). |

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

**P1-3 r2 — integration-test driver**: `Sequencer::run(rx)` (`src/state/sequencer.rs:304-:316`) is a `pub async fn` that loops until the receiver closes. There is no single-poll API today, but I3-I13 integration tests need to drive a single submission through `apply_one` to observe its effect synchronously. v3 adds:

```rust
// src/state/sequencer.rs (new pub method — small, intentional public test driver)
/// Drain at most one envelope from the queue and run `apply_one` on it.
/// Returns `None` if the queue is empty. Intended for tests / single-step
/// drivers; production code should use `Sequencer::run(rx)`.
pub fn try_apply_one(&self, rx: &mut Receiver<SubmissionEnvelope>) -> Option<Result<LedgerEntry, ApplyError>> {
    match rx.try_recv() {
        Ok(envelope) => Some(self.apply_one(envelope)),
        Err(_) => None,
    }
}
```

Receiver ownership note: `Sequencer::new` returns `(Sequencer, Receiver<SubmissionEnvelope>)`. Production passes the receiver into `run`; tests retain it and pass to `try_apply_one`. The pre-v3 receiver-on-`Sequencer-self` shape is verified at `src/state/sequencer.rs:271, :304` — receiver is moved into `run` at `:304`, so it's not stored on `Sequencer` itself; v3 exposes the same external receiver back to tests instead.

### 3.2 `Sequencer.rejection_writer` ownership (P0-A r1 + P0-1 r2 — writer ownership disclosed with verified shape)

**Verified shape** (r2 audit: `RejectionEvidenceWriter::append_rejected` is `&mut self` per `src/bottom_white/ledger/rejection_evidence.rs:226, 258`; existing `Sequencer` already wraps mutable shared state in `Arc<RwLock<...>>` at `src/state/sequencer.rs:238, 241`). `Arc<RejectionEvidenceWriter>` would NOT compile. The matching shape is:

```rust
// src/state/sequencer.rs — accurate shape post-v3 (existing fields verified vs
// constructor signature at :260-:285; field layout at :230-:247):
pub struct Sequencer {
    next_submit_id: AtomicU64,                                // existing
    next_logical_t: AtomicU64,                                // existing
    queue_tx: tokio::sync::mpsc::Sender<SubmissionEnvelope>,  // queue payload changes (§3.1)
    cas: Arc<RwLock<CasStore>>,                               // existing — RwLock pattern
    keypair: Arc<Ed25519Keypair>,                             // existing — actual type
    epoch: SystemEpoch,                                        // existing — actual type
    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,             // existing — RwLock pattern
    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,   // **NEW — matches pattern**
    predicate_registry: Arc<PredicateRegistry>,               // existing — actual field name
    tool_registry: Arc<ToolRegistry>,                         // existing — actual field name
    q: RwLock<QState>,                                         // existing — NO Arc wrap (q is
                                                               // accessed only through &Sequencer)
}
```

**Constructor signature change** (the only `Sequencer::new` signature delta):

```rust
pub fn new(
    cas: Arc<RwLock<CasStore>>,
    keypair: Arc<Ed25519Keypair>,
    epoch: SystemEpoch,
    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,  // **NEW — positioned after ledger_writer**
    predicate_registry: Arc<PredicateRegistry>,
    tool_registry: Arc<ToolRegistry>,
    initial_q: QState,
    queue_capacity: usize,
) -> (Self, tokio::sync::mpsc::Receiver<SubmissionEnvelope>) {  // Receiver type changes per §3.1
    // ... existing body ...
}
```

- `Sequencer::new(...)` gains a `rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>` constructor parameter immediately after `ledger_writer` (mirrors L4 / L4.E pair).
- **All existing `Sequencer::new(...)` call sites in repo (verified via r2 audit): exactly TWO** — `src/state/sequencer.rs:483` and `:627`. Both are inside the existing `#[cfg(test)] mod` and need the new parameter passed as `Arc::new(RwLock::new(RejectionEvidenceWriter::default()))`. (`RejectionEvidenceWriter` derives `Default` per its declaration at `src/bottom_white/ledger/rejection_evidence.rs:20`.) No production call site updates are needed because no production code currently constructs a `Sequencer` outside test fixtures.
- **Integration-test visibility (P0-5 r2)**: the in-crate `pub(crate) fn rejection_writer_for_test()` accessor described in v2 is INSUFFICIENT — it's invisible to `tests/tb_2_runtime_boundary.rs` (a separate crate). Replace with a constructor-injection pattern: integration tests construct their own `let writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()))`, pass `writer.clone()` to `Sequencer::new`, then read via the retained handle using `RejectionEvidenceWriter::records()` and `public_view()` (both `pub` per `src/bottom_white/ledger/rejection_evidence.rs:327, 340`). No new `Sequencer` accessor is needed.
- **In-crate unit tests (U1-U3)**: still use the same constructor-injection pattern internally (the `pub(crate)` accessor is dropped to avoid visibility-mismatch confusion).
- L4.E persistence semantics remain deferred — writer is in-memory per `src/bottom_white/ledger/rejection_evidence.rs:30, 34`. TB-2 incurs no new persistence wiring.

### 3.3 `dispatch_transition` `TypedTx::Work` arm — pure validation

The arm is filled with **pure** validation (no side effects, no I/O, no writer calls). On accept it returns `Ok((q_next, signals))`; on reject it returns `Err(TransitionError::<specific class>)`.

Validation steps (in order; first-failure short-circuits):

1. **Parent-root match**: `if tx.parent_state_root != q.state_root_t { return Err(TransitionError::StaleParentRoot); }`.
2. **Acceptance predicate bundle**: every entry in `tx.predicate_results.acceptance` is `true` (else `Err(TransitionError::PredicateFailed)`).
3. **Settlement predicate bundle (if applicable to RSP-1)**: every entry is `true` or empty (else `Err(TransitionError::PredicateFailed)`).
4. **YES stake gate (RSP-1, P0-3 r2 — verified `StakeMicroCoin` comparison)**: `tx.stake.micro_units() > 0` (else `Err(TransitionError::StakeInsufficient)` — variant already exists at `src/state/typed_tx.rs:731`). `WorkTx.stake: StakeMicroCoin` (`src/state/typed_tx.rs:232`); `StakeMicroCoin::micro_units(self) -> i64` is `const` (`src/economy/money.rs:168`); `StakeMicroCoin > 0` integer comparison would NOT compile (the newtype intentionally prevents arithmetic mixing with raw integers per the type's TRACE_MATRIX I-STAKE rationale at `src/economy/money.rs:142-150`).
5. **Escrow presence gate (RSP-1, P0-B option (a) — bridge at lookup site; P0-3 r2 — verified `.0` newtype-map access)**:

   ```rust
   // P0-B option (a): in-arm deterministic bridge from TaskId to TxId namespace.
   // TB-3 introduces formal task_open_tx / escrow_lock_tx / yes_stake_tx variants
   // that allocate proper TxIds at submission time; this bridge is then DELETED.
   let lookup_tx_id = TxId(tx.task_id.0.clone());
   // r2 finding P0-3: EscrowsIndex / TaskMarketsIndex are pub-tuple-struct newtypes
   // (`pub struct EscrowsIndex(pub BTreeMap<TxId, EscrowEntry>)` at q_state.rs:159-161).
   // `.contains_key(...)` is on the inner map, accessed via .0 — direct contains_key
   // on the newtype wrapper would not compile.
   let has_escrow = q.economic_state_t.escrows_t.0.contains_key(&lookup_tx_id)
                  || q.economic_state_t.task_markets_t.0.contains_key(&lookup_tx_id);
   if !has_escrow {
       return Err(TransitionError::EscrowMissing);  // NEW variant — see §3.7
   }
   ```

   **Rationale**: `WorkTx.task_id: TaskId` and `TaskId(pub String)` / `TxId(pub String)` (`src/state/typed_tx.rs:33, 35, 225`); `EconomicState.{escrows_t: EscrowsIndex, task_markets_t: TaskMarketsIndex}` newtypes wrap `BTreeMap<TxId, ...>` (`src/state/q_state.rs:159, 161, 222, 224`). The `.0` access is intentional — preserves the namespace-conflation cost as visible to readers and makes the deletion site obvious for TB-3 cleanup. The task-keyed `EscrowVault` (`src/economy/escrow_vault.rs`) is intentionally NOT used here — it is a separate truth source ("distinct from `state::q_state::EscrowEntry`" per its own docs at `:15, :53, :146, :168`); TB-2 keeps `EscrowVault` as the future TB-3+ unification target and reads only from `q.economic_state_t` at the runtime spine to preserve the single-truth-source contract. `EscrowVault` non-use red line confirmed enforceable by r2 Q6: zero existing call sites in `src/state/`.

6. **Monetary invariants** (production call sites pass `&[]` exempt list per §3.5):
   - `monetary_invariant::assert_no_post_init_mint(tx, q)` → on violation, map to `Err(TransitionError::MonetaryInvariantViolation)` (NEW variant — see §3.7).
   - `monetary_invariant::assert_read_is_free(TxKind::Work, 0)` → infallible for `TxKind::Work` with zero read cost; included for symmetry / future-proofing.
   - `monetary_invariant::assert_total_ctf_conserved(q.economic_state_t, q_next.economic_state_t, &[])` → on violation, map to `Err(TransitionError::MonetaryInvariantViolation)` (same NEW variant).

### 3.4 `q_next.state_root_t` — interim domain-separated hash (P1-E r1 + P1-1/P1-2 r2 — domain constant location pinned + canonical hash helper defined)

**Location pinned** (P1-1 r2): `src/state/sequencer.rs` (alongside the other state machinery; `q_state.rs` is data-shape only). Removed the v2 ambiguity ("`sequencer.rs` or `q_state.rs`").

```rust
// src/state/sequencer.rs (new constant — top of file, near other domain constants)
/// TB-2 interim WorkTx-accept state-root domain. Real patch semantics land in P5.
/// Distinct from the TB-1 toy domain `b"turingosv4.l4_state_root.v1"` used by
/// `AcceptedLedger` at `src/economy/ledger.rs:350, 357` (do not reuse — TB-1
/// primitive vs production state-root mutator separation).
pub(crate) const WORKTX_ACCEPT_DOMAIN_V1: &[u8] = b"turingosv4.worktx.accept.v1";

/// TB-2 canonical hash helper for a `TypedTx` (P1-2 r2 — `canonical_hash` is
/// NOT an existing src/state helper; the available primitive at the bottom_white
/// layer is `canonical_encode`). Defined locally to avoid a typed_tx.rs touch.
fn worktx_canonical_hash(tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(b"turingosv4.worktx.canonical_hash.v1");  // domain separation
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    Hash(h.finalize().into())
}
```

On accept (in `dispatch_transition`'s WorkTx arm, after all validation passes):

```rust
let work_tx_digest = worktx_canonical_hash(&TypedTx::Work(tx.clone()));
let mut h = Sha256::new();
h.update(WORKTX_ACCEPT_DOMAIN_V1);
h.update(q.state_root_t.0);  // Hash(pub [u8; 32]); access raw bytes via .0
h.update(work_tx_digest.0);
q_next.state_root_t = Hash(h.finalize().into());
```

`q_next.economic_state_t` reflects stake/escrow/balances delta as required by the monetary invariants (concrete delta semantics handled by existing `EconomicState` helpers — no new APIs added by TB-2 to that module).

### 3.5 `apply_one` rejection-writer error path (P0-2 + P0-6 r2 — verified `submitter_id() -> Option<AgentId>` + 5-arg `CasStore::put` shape)

`apply_one(envelope)` semantics on `Err(e)` from `dispatch_transition`. Uses the **same lock/encode/put pattern** as the accepted path's `apply_one` success arm (lines `:351, :363, :377, :386, :417, :423` per r2 verification):

```rust
if let Err(e) = dispatch_result {
    // 1. CAS-put canonical-encoded tx payload.
    //    P0-6 r2: CasStore::put is `&mut self` + 5 args (bytes, ObjectType, creator,
    //    created_at_logical_t, schema_id) per src/bottom_white/cas/store.rs:163-170.
    //    cas is `Arc<RwLock<CasStore>>`, so the call needs a write lock.
    let tx_payload_bytes = canonical_encode(&envelope.tx)
        .expect("TypedTx is canonical-encodable");
    let tx_payload_cid = {
        let mut cas_w = self.cas.write().await;
        cas_w.put(
            &tx_payload_bytes,
            ObjectType::TypedTxPayload,           // existing variant; if not, use the
                                                    // accepted-path's choice here verbatim
            "sequencer.rejection_path.tb2",       // creator string, mirrors accepted-path
            self.next_logical_t.load(Ordering::SeqCst), // pre-advance logical_t snapshot
            None,                                  // schema_id: None for TB-2; future TBs
                                                    // populate when typed schema lands
        )?
    };

    // 2. CAS-put diagnostic (always Some at runtime; raw_diagnostic_cid is
    //    structurally serde-shielded on RejectedSubmissionRecord per TB-1 P0-3
    //    at src/bottom_white/ledger/rejection_evidence.rs:108. I8 re-confirms.).
    let raw_diagnostic_cid = {
        let mut cas_w = self.cas.write().await;
        Some(cas_w.put(
            e.to_string().as_bytes(),
            ObjectType::RejectionDiagnostic,      // or fallback to existing variant
            "sequencer.rejection_path.tb2",
            self.next_logical_t.load(Ordering::SeqCst),
            None,
        )?)
    };

    // 3. P0-2 r2: HasSubmitter::submitter_id() returns Option<AgentId>
    //    (src/state/typed_tx.rs:642). RejectedSubmissionRecord.agent_id is AgentId
    //    (NOT Option) per src/bottom_white/ledger/rejection_evidence.rs:90.
    //    For TB-2's WorkTx-only arm, WorkTx::submitter_id() always returns Some
    //    (it clones tx.agent_id at typed_tx.rs:646-648). For future variants whose
    //    submitter_id() is None (system-emitted), we substitute the SYSTEM_AGENT_ID
    //    sentinel — keeps L4.E's single-truth-source `agent_id: AgentId` schema
    //    intact without forcing an Option<AgentId> schema migration.
    let agent_id = envelope.tx
        .submitter_id()
        .unwrap_or_else(|| AgentId(SYSTEM_AGENT_ID_STR.to_string()));

    // 4. Append to L4.E. Currently in-memory + infallible; persistence deferred.
    //    P0-1 r2: rejection_writer is Arc<RwLock<RejectionEvidenceWriter>>;
    //    append_rejected is &mut self per rejection_evidence.rs:258.
    {
        let mut writer_w = self.rejection_writer.write().await;
        writer_w.append_rejected(
            envelope.submit_id,
            q_snapshot.state_root_t,
            agent_id,
            envelope.tx.tx_kind(),       // TypedTx::tx_kind() at typed_tx.rs:620
            tx_payload_cid,
            rejection_class_for(&e),     // P0-D r1 + P0-4 r2 — see §3.7 mapping
            raw_diagnostic_cid,
            public_summary_for(&e),      // see §3.5b below
        );
    }

    // 5. Return without advancing logical_t / state_root_t / ledger_root_t.
    return Err(ApplyError::Transition(e));
}
```

`SYSTEM_AGENT_ID_STR` is a `pub(crate) const &str = "__system__"` declared near `WORKTX_ACCEPT_DOMAIN_V1` in `src/state/sequencer.rs`. The string content is internal-only; it never crosses the agent boundary (only `public_summary` does, per `RejectedSubmissionRecord.agent_id` doc-comment at `rejection_evidence.rs:89-90`).

#### 3.5b `public_summary_for(&e)` helper

Defined locally in `sequencer.rs` alongside `rejection_class_for`:

```rust
/// Agent-facing summary string for an L4.E record. Returns a small, predicate-id-
/// stripped class label so private predicate identities never leak (TB-1 §1.4
/// "Opaque" discipline). `None` for variants with no agent-safe summary.
fn public_summary_for(e: &TransitionError) -> Option<String> {
    match e {
        TransitionError::StaleParent => Some("stale_parent_root".into()),
        TransitionError::StakeInsufficient => Some("stake_insufficient".into()),
        TransitionError::EscrowMissing => Some("escrow_missing".into()),
        TransitionError::MonetaryInvariantViolation => Some("monetary_invariant".into()),
        TransitionError::AcceptancePredicateFailed(_)
        | TransitionError::SettlementPredicateFailed(_) => Some("predicate_failed".into()),
        // Out-of-WorkTx-arm variants — should not occur in TB-2; conservative
        // sentinel preserves L4.E append correctness if they do.
        _ => Some("policy_violation".into()),
    }
}
```

The wildcard arm is intentional and matches the §3.7 mapping policy. Codex r2 P0-4 explicitly permits "wildcard mapping for out-of-WorkTx variants" provided it's documented; the `_ =>` here is the documented sentinel.

Accepted path stays on the **existing** `transition_ledger` + `LedgerWriter` flow already wired into `apply_one`'s success arm at `src/state/sequencer.rs:351, :377, :386, :423` — TB-2 does not introduce a new ledger writer and does not change the accepted-side commit sequence.

### 3.6 Orphan-CAS partial-write contract (P1-E)

CAS `put` is durable as soon as it returns `Ok` (`src/bottom_white/cas/store.rs:162, 195`). If a later step in `apply_one` fails (rejection: L4.E append; accepted: writer commit), the already-written CAS object becomes an orphan. TB-2 contract:

- Orphan-CAS objects are content-addressed; identical re-submission produces an identical CID and re-uses the existing object (no duplication).
- Orphan-CAS objects are tolerable in TB-2 because both rejection-path L4.E append (`src/bottom_white/ledger/rejection_evidence.rs:30, 34, 258, 268`) and accepted-path writer commit are currently in-memory or single-commit Git2 operations — partial-write windows are narrow and bounded.
- Orphan-CAS GC / reachability semantics are deferred to a later TB (likely co-with L4.E persistence). TB-2 does NOT add a GC pass.

### 3.7 `TransitionError → l4e::RejectionClass` mapping (P0-D r1 + P0-4 r2 + P1-4 r2 — closed against verified 22-variant enum + disambiguated which `RejectionClass`)

**Disambiguation (P1-4 r2)**: there are TWO `RejectionClass` enums in the repo. TB-2 uses the **L4.E ledger one**:

```rust
use crate::bottom_white::ledger::rejection_evidence::RejectionClass as L4ERejectionClass;
```

(NOT `crate::state::typed_tx::RejectionClass` at `:165`, which is for `WorkOutcome::Rejected(...)` book-keeping.) The L4.E enum at `src/bottom_white/ledger/rejection_evidence.rs:56-67` has 5 variants: `PredicateFailed = 0`, `PolicyViolation = 1`, `EscrowMissing = 2`, `InvariantViolation = 3`, `MalformedPayload = 4`.

**Existing `TransitionError` variants** (`src/state/typed_tx.rs:717-788`, verified — 22 total):

`StaleParent`, `SignatureInvalid`, `InvalidSystemSignature`, `StakeInsufficient`, `TargetWorkTxNotFound`, `TargetWorkTxNotVerifiable`, `ParentNotAcceptedYet`, `AcceptancePredicateFailed(PredicateId)`, `VerificationPredicateFailed(PredicateId)`, `SettlementPredicateFailed(PredicateId)`, `ChallengeWindowClosed`, `ChallengeWindowStillOpen`, `AlreadySlashed`, `CounterexampleInsufficient`, `ToolNotInRegistry`, `ToolCreatorMismatch`, `ClaimNotFound`, `TaskNotFound`, `TaskNotExpired`, `TaskHasOpenClaim`, `TerminalSummaryNotApplicable`, `NotYetImplemented`. Enum is NOT `#[non_exhaustive]`.

**TB-2 adds TWO new `TransitionError` variants** (revised down from r1's three — `StaleParent` already exists, no need to add `StaleParentRoot`):

- `TransitionError::EscrowMissing` (NEW) — escrow / task-market lookup miss in the WorkTx arm bridge (§3.3 step 5).
- `TransitionError::MonetaryInvariantViolation` (NEW) — `assert_no_post_init_mint` OR `assert_total_ctf_conserved` violation in the WorkTx arm (§3.3 step 6).

**`Display` impl is exhaustive** (`src/state/typed_tx.rs:790-816`); the two new variants must add Display arms. Estimated typed_tx.rs delta: ~6 lines (2 enum variants + 2 Display arms + 2 doc-comments).

**Mapping (`fn rejection_class_for(e: &TransitionError) -> L4ERejectionClass` — closed match per Codex r2 P0-4)**:

| `TransitionError` (file:line) | `L4ERejectionClass` | WorkTx-arm reachable? |
|---|---|---|
| `StaleParent` (`:720`) | `PolicyViolation` | YES (parent-root mismatch) |
| `SignatureInvalid` (`:722`) | `PolicyViolation` | not in TB-2 (sig check is upstream) |
| `InvalidSystemSignature` (`:724`) | `PolicyViolation` | not in TB-2 |
| `StakeInsufficient` (`:731`) | `PolicyViolation` | YES (stake gate) |
| `TargetWorkTxNotFound` (`:735`) | `PolicyViolation` | not in TB-2 (Verify/Challenge/Reuse arms only) |
| `TargetWorkTxNotVerifiable` (`:737`) | `PolicyViolation` | not in TB-2 |
| `ParentNotAcceptedYet` (`:739`) | `PolicyViolation` | not in TB-2 |
| `AcceptancePredicateFailed(_)` (`:745`) | `PredicateFailed` | YES (predicate bundle) |
| `VerificationPredicateFailed(_)` (`:747`) | `PredicateFailed` | not in TB-2 |
| `SettlementPredicateFailed(_)` (`:749`) | `PredicateFailed` | YES (settlement bundle) |
| `ChallengeWindowClosed` (`:753`) | `PolicyViolation` | not in TB-2 |
| `ChallengeWindowStillOpen` (`:755`) | `PolicyViolation` | not in TB-2 |
| `AlreadySlashed` (`:757`) | `PolicyViolation` | not in TB-2 |
| `CounterexampleInsufficient` (`:759`) | `PolicyViolation` | not in TB-2 |
| `ToolNotInRegistry` (`:763`) | `PolicyViolation` | not in TB-2 |
| `ToolCreatorMismatch` (`:765`) | `PolicyViolation` | not in TB-2 |
| `ClaimNotFound` (`:769`) | `PolicyViolation` | not in TB-2 |
| `TaskNotFound` (`:773`) | `PolicyViolation` | not in TB-2 (escrow gate fires first in WorkTx arm) |
| `TaskNotExpired` (`:775`) | `PolicyViolation` | not in TB-2 |
| `TaskHasOpenClaim` (`:777`) | `PolicyViolation` | not in TB-2 |
| `TerminalSummaryNotApplicable` (`:781`) | `PolicyViolation` | not in TB-2 |
| `NotYetImplemented` (`:787`) | `PolicyViolation` | not in TB-2 (WorkTx arm now non-stub) |
| `EscrowMissing` (NEW) | `EscrowMissing` | YES (escrow gate) |
| `MonetaryInvariantViolation` (NEW) | `InvariantViolation` | YES (monetary invariants) |

Implementation:

```rust
fn rejection_class_for(e: &TransitionError) -> L4ERejectionClass {
    use TransitionError as TE;
    use L4ERejectionClass as RC;
    match e {
        TE::AcceptancePredicateFailed(_)
        | TE::VerificationPredicateFailed(_)
        | TE::SettlementPredicateFailed(_) => RC::PredicateFailed,
        TE::EscrowMissing => RC::EscrowMissing,
        TE::MonetaryInvariantViolation => RC::InvariantViolation,
        // All other 19 variants (including the new EscrowMissing/MonetaryInvariantViolation
        // are already covered above; this wildcard catches the 19 PolicyViolation entries
        // explicitly enumerated in the table). r2 P0-4 sanctions wildcard for non-WorkTx-arm
        // variants given the documented enumeration.
        _ => RC::PolicyViolation,
    }
}
```

**Adding any new `TransitionError` variant in a future TB MUST extend this table, the `match` above, AND the `Display` impl at `typed_tx.rs:790-816`. The mapping is closed by enumeration even though the `match` uses `_` for the 19-variant tail — the table is the source of truth.**

### 3.8 Untouched arms

`TypedTx::Verify`, `TypedTx::Challenge`, `TypedTx::Reuse`, `TypedTx::FinalizeReward`, `TypedTx::TaskExpire`, `TypedTx::TerminalSummary` all stay `Err(TransitionError::NotYetImplemented)` and are out of TB-2 scope.

**Scope creep guard**: any line of code outside §3.1–§3.7 (production-code §) constitutes a separate atom and must be extracted into TB-3+ unless the auditors explicitly approve it in Phase-1c. The only files touched by §3.1–§3.7 are `src/state/sequencer.rs` (primary — adds `SubmissionEnvelope`, `WORKTX_ACCEPT_DOMAIN_V1`, `SYSTEM_AGENT_ID_STR`, `worktx_canonical_hash`, `rejection_class_for`, `public_summary_for`, `try_apply_one`, `rejection_writer` field, fills WorkTx arm of `dispatch_transition`, rewrites `apply_one` error path) and `src/state/typed_tx.rs` (TWO new `TransitionError` variants: `EscrowMissing` + `MonetaryInvariantViolation`; corresponding 2 lines in the `Display` impl). `q_state.rs` constant home was rejected per §3.4 P1-1 r2.

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
| **U2** | `apply_one_rejected_path_uses_envelope_submit_id` | Drive `apply_one` with an envelope carrying `submit_id = 42` and a WorkTx that fails predicate validation. Read the **constructor-injected** `Arc<RwLock<RejectionEvidenceWriter>>` clone (held by the test) and assert via `RejectionEvidenceWriter::records()` that the new L4.E row's `submit_id == 42`. |
| **U3** | `dispatch_transition_worktx_returns_state_root_via_domain_v1` | Call `dispatch_transition` directly with a predicate-passing WorkTx + `stake.micro_units() > 0` + seeded escrow. Assert `q_next.state_root_t == Hash(sha256(WORKTX_ACCEPT_DOMAIN_V1 ‖ q.state_root_t.0 ‖ worktx_canonical_hash(&TypedTx::Work(tx)).0))` exactly (proves the interim domain hash is what TB-2 ships, not a different scheme). |

### 5.2 Integration tests — `tests/tb_2_runtime_boundary.rs`

These tests go through `Sequencer::submit` (the public path) and drive a single submission via the new `pub fn try_apply_one(rx: &mut Receiver<SubmissionEnvelope>)` helper (§3.1 P1-3 r2). They need only `pub` API. They observe L4.E by retaining a clone of the `Arc<RwLock<RejectionEvidenceWriter>>` they injected into `Sequencer::new` — no `pub(crate)` accessor is required (P0-5 r2 fix).

#### Submit-id plumbing

| # | Test | Asserts |
|---|---|---|
| **I1** | `submit_returns_receipt_and_envelope_submit_id_matches` | The `submit_id` returned by `submit()` matches the `submit_id` keyed in the resulting L4 row (accept) or L4.E row (reject). Replaces v1 Test 1. |
| **I2** | `submit_queue_full_consumes_submit_id` | Saturate the queue (size known from `Sequencer::new` config), call `submit()` once more — expect `Err(SubmitError::QueueFull)`. Drain one slot, `submit()` again. Assert the successful `submit_id` is `n+2`, not `n+1` — the failed `try_send` still burned ID `n+1`. **Locks the contract that `submit_id` is allocated atomically before `try_send` and is NEVER reused, even on `try_send` failure.** Battery test #14 from r1 P0-E. |

#### Rejection spine (proof 1)

| # | Test | Asserts |
|---|---|---|
| **I3** | `runtime_predicate_failed_worktx_appends_l4e` | Submit a WorkTx whose `predicate_results.acceptance` contains a `false`. Expect: 1 L4.E row with matching `submit_id`, `rejection_class == PredicateFailed`. |
| **I4** | `runtime_stale_parent_worktx_appends_l4e` | Submit a WorkTx with `parent_state_root != q.state_root_t`. Expect: 1 L4.E row, `rejection_class == PolicyViolation` (mapped from existing `TransitionError::StaleParent` per §3.7 — note: existing variant, NOT a new `StaleParentRoot`). Battery test #13 from r1 P0-E. |
| **I5** | `runtime_stakeless_worktx_appends_l4e` | Submit a WorkTx with `stake == StakeMicroCoin::zero()`. Expect: 1 L4.E row, `rejection_class == PolicyViolation` (mapped from existing `TransitionError::StakeInsufficient`). |
| **I6** | `runtime_no_escrow_worktx_appends_l4e` | Submit a WorkTx for a `task_id` whose bridged `TxId(task_id.0.clone())` has no entry in either `q.economic_state_t.escrows_t.0` or `task_markets_t.0`. Expect: 1 L4.E row, `rejection_class == EscrowMissing` (mapped from new `TransitionError::EscrowMissing`). |
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
3. No new `TypedTx` variants. `task_open_tx` / `escrow_lock_tx` / `yes_stake_tx` are reserved for TB-3. **TWO new `TransitionError` variants are permitted** (`EscrowMissing`, `MonetaryInvariantViolation`) per §3.7 mapping table + 2 new `Display` arms — these are exhaustive-match-completeness additions, not new economic types. **`StaleParent` already exists** at `src/state/typed_tx.rs:720`; do NOT add a `StaleParentRoot` (revised down from r1).
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
- TB-2 Phase-0 r1 dual-audit verdict: `handover/audits/DUAL_AUDIT_TB_2_PHASE0_VERDICT_R1_2026-04-30.md` (drove v1 → v2 revision)
- TB-2 Phase-0 r1 individual audits: `handover/audits/CODEX_TB_2_PHASE0_AUDIT_2026-04-30.md` + `handover/audits/GEMINI_TB_2_PHASE0_AUDIT_2026-04-30.md`
- TB-2 Phase-0 r2 narrowed Codex audit: `handover/audits/CODEX_TB_2_PHASE0_R2_AUDIT_2026-04-30.md` (drove v2 → v3 revision; CHALLENGE / 5/5 on substrate compile-shape)
- 9-phase canonical roadmap: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` (P1 / P3 Current state refreshed 2026-04-30)
- Memory: `feedback_step_b_protocol`, `feedback_dual_audit`, `feedback_dual_audit_conflict`, `feedback_phased_checkpoint`, `feedback_smoke_before_batch`, `feedback_no_fake_menus`, `feedback_session_label_codification`, `feedback_elon_mode_policy`.

---

## 10. v2 → v3 changelog (Codex r2 narrowed audit response)

| r2 Finding | Resolution in v3 | Section(s) |
|---|---|---|
| **P0-1** `Arc<RejectionEvidenceWriter>` won't compile (`append_rejected` is `&mut self`) | Changed to `Arc<RwLock<RejectionEvidenceWriter>>` matching existing `cas` / `ledger_writer` pattern. Two `Sequencer::new` call sites (`:483`, `:627`) get `Arc::new(RwLock::new(RejectionEvidenceWriter::default()))`. | §3.2 |
| **P0-2** `submitter_id() -> Option<AgentId>` mismatched with `RejectedSubmissionRecord.agent_id: AgentId` | Added `SYSTEM_AGENT_ID_STR` const + `unwrap_or_else(|| AgentId(...))` policy. WorkTx always returns Some so the unwrap arm is theoretical for TB-2 but gives future arms a typed answer. | §3.5 |
| **P0-3** `.contains_key` on `EscrowsIndex`/`TaskMarketsIndex` newtype + `tx.stake > 0` | Bridge uses `.escrows_t.0.contains_key(...)` and `.task_markets_t.0.contains_key(...)`. Stake gate uses `tx.stake.micro_units() > 0` per `src/economy/money.rs:168`. | §3.3 step 4-5 |
| **P0-4** Mapping table not closed against actual 22-variant `TransitionError`; names invented | Mapping rebuilt: `StaleParent` (existing) NOT `StaleParentRoot`; only `EscrowMissing` + `MonetaryInvariantViolation` are new variants (was 3, now 2). 22-row table enumerated; `match` uses documented wildcard for non-WorkTx-arm variants per Codex r2 P0-4 sanction. Display impl gets 2 new arms. | §3.7 + §1 |
| **P0-5** `pub(crate) rejection_writer_for_test()` invisible to `tests/` | Removed. Integration tests retain a clone of `Arc<RwLock<RejectionEvidenceWriter>>` they injected into `Sequencer::new` and read via existing `pub` `records()` / `public_view()` API at `rejection_evidence.rs:327, 340`. No new `Sequencer` accessor. | §3.2, §5.2 |
| **P0-6** CAS API mismatch — 5-arg `&mut self` not 1-arg `Arc<CasStore>` | Rejection-path CAS-puts mirror accepted-path: `let mut cas_w = self.cas.write().await; cas_w.put(bytes, ObjectType, creator, logical_t, schema_id)`. Same pattern in two places (tx_payload, raw_diagnostic). | §3.5 |
| **P1-1** `WORKTX_ACCEPT_DOMAIN_V1` location ambiguous | Pinned to `src/state/sequencer.rs`; q_state.rs option dropped from §1 + §3.4. | §1, §3.4 |
| **P1-2** `canonical_hash(tx)` not an existing helper | Defined locally as `fn worktx_canonical_hash(tx: &TypedTx) -> Hash` using existing `canonical_encode` + `Sha256` with explicit domain separation. | §3.4 |
| **P1-3** No single-poll `Sequencer::run` API for tests | Added `pub fn try_apply_one(&self, rx: &mut Receiver<SubmissionEnvelope>) -> Option<Result<...>>` — small intentional public driver. | §3.1 |
| **P1-4** Two `RejectionClass` enums (typed_tx vs rejection_evidence); v2 didn't disambiguate | Mapping table imports `crate::bottom_white::ledger::rejection_evidence::RejectionClass as L4ERejectionClass`. typed_tx::RejectionClass at `:165` is for `WorkOutcome::Rejected` book-keeping, NOT used by L4.E. | §3.7 |
| **P1-5** Stale `Sequencer` struct excerpt — wrong types in v2 | Struct excerpt + constructor signature corrected to verified types: `cas: Arc<RwLock<CasStore>>`, `keypair: Arc<Ed25519Keypair>`, `epoch: SystemEpoch`, `ledger_writer: Arc<RwLock<dyn LedgerWriter>>`, `q: RwLock<QState>` (no `Arc` wrap), `predicate_registry`/`tool_registry` field names. | §3.2 |

---

## 10b. v1 → v2 changelog (r1 audit response)

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
