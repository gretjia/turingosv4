# CO1.7.5 Transition Bodies + Runtime Wiring v1 — DRAFT (self-audited; pre round-1)

**Status**: v1 DRAFT (2026-04-29). Smoke 8/8 PASS. Self-audit pass complete (8 patches; § 0.3 / § 1 D2-D3 / § 3 / § 5 / § 6 / § 0.2 rewritten — see footer patch log). Awaiting round-1 dual external audit.
**Author**: ArchitectAI (Claude); session 2026-04-29.
**Supersedes**: nothing (NEW atom; predicted by `CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` § 13).
**Pre-implementation gate**: this spec must reach **PASS/PASS** dual external audit before any code lands. Sedimented per CLAUDE.md "Audit Standard".

**Companion specs (frozen, read first)**:
- `CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` v1.2 — L4 transition ledger + Sequencer + dispatch_transition skeleton (round-3 PASS/PASS); freezes ABI + apply_one machinery.
- `CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — frozen 7-variant TypedTx + 13 locked golden hex + 22-variant TransitionError.
- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — pure transition pseudocode for 7 sub-sections § 3 / § 3.1 / § 3.2 / § 3.3 / § 3.4 / § 3.6 / § 3.7 (round-4 PASS/PASS).
- `META_TRANSITION_INTERFACE_v1_2026-04-27.md` — trait pattern (deferred runtime to v4.1).

**Single sentence**: implement the 7 per-kind transition function bodies (currently `Err(NotYetImplemented)` stubs), close the G-1 carry-forward `q.head_t = NodeId(commit_oid_hex)` wiring after `Git2LedgerWriter.commit`, perform STEP_B parallel-branch ceremony for the bus.rs / kernel.rs Sequencer entry-point, and un-ignore `sequencer_serial_replay_byte_identity` so the byte-identity I-DETHASH witness fires end-to-end.

---

## § 0 Status + dependency map

### 0.1 What this atom inherits (frozen)
| Frozen by | Surface | Why CO1.7.5 cannot change it |
|---|---|---|
| CO1.1.4-pre1 (commit `c1226e2`) | 7-variant `TypedTx` + 22-variant `TransitionError` (incl. `NotYetImplemented`) + 13 locked golden hex | ABI-locked; behavior change = re-audit + golden invalidation |
| CO1.7-impl A1 (commit `2461fe6`) | `LedgerEntry` / `LedgerEntrySigningPayload` 9-field signing surface; `Git2LedgerWriter` + `InMemoryLedgerWriter`; `head_commit_oid()` accessor; `transition_ledger_emitter::sign_ledger_entry` | C3 wired in code; head_commit_oid already exposed |
| CO1.7-impl A2+A3 (commit `2461fe6`) | `Sequencer` 9-stage `apply_one` + `dispatch_transition` exhaustive match | structural correctness locked; only per-variant arms change |
| CO1.7-impl A4 (commit `2461fe6`) | `replay_full_transition` 9-stage I-DETHASH witness | replay fixture path locked |
| CO1.4-extra (commit `b6b7574`) | CAS sidecar JSONL index persistence | cold-restart full-replay path unblocked |

### 0.2 What this atom delivers (new)
1. **D1 — 7 per-kind transition function bodies** translating `STATE_TRANSITION_SPEC § 3 / § 3.1-3.4 / § 3.6 / § 3.7` pseudocode into deterministic pure Rust, with two CO1.7-K3-v1.2 / CO1.1.4-pre1 supersessions carried forward (§ 0.3).
2. **D2 — G-1 head_t close** at Sequencer post-commit: `q.head_t = state::q_state::NodeId(commit_oid_hex)` after `writer.commit(&entry)` returns Ok. Requires one additive `LedgerWriter` trait method (`head_commit_oid_hex`).
3. **D3 — Combined STEP_B ceremony** for `src/kernel.rs` (Sequencer field) + `src/bus.rs` (forwarder method) as one A/B unit. Sequencer instance lives in Kernel only; Bus forwards via `self.kernel.sequencer`. Coexists with legacy `crate::ledger`; full retirement is CO1.1.5.
4. **D4 — Un-ignore `sequencer_serial_replay_byte_identity`** + 3 NEW CO1.7.5+ stage tests already declared in `CO1_7 § 7` table.

### 0.3 Two STATE_TRANSITION_SPEC § 3 supersessions adopted by CO1.7.5

CO1.7.5 inherits two prior minimization decisions that diverge from STATE § 3 v1.4 pseudocode. Both were ratified by downstream specs reaching dual-audit PASS/PASS; CO1.7.5 carries them forward unchanged. Re-audit gating for the STATE spec itself is **not** decided here — CO1.7.5 spec only documents the carry-forward; whether STATE_TRANSITION_SPEC needs a v1.5 housekeeping commit is a separate decision for the STATE spec curator.

#### 0.3.1 head_t mutation site
| Source | Says | Authority |
|---|---|---|
| STATE § 3 line 412 (and parallel lines in § 3.1 line 467, § 3.2 line 561) | `q_next.head_t = NodeId::from_state_root(new_state_root)` inside the pure transition body | STATE v1.4 round-4 PASS/PASS |
| CO1.7 K3 v1.2 § 5 | "`NodeId::from_state_root(...)` is NOT used by L4 in any version"; head_t mutation deferred to CO1.7.5+ Sequencer post-commit | CO1.7 v1.2 round-3 PASS/PASS |

**Resolution carried forward**: CO1.7.5 transition bodies **never** mutate `q_next.head_t`. Pure function returns `q_next` with `head_t == q.head_t`. `q.head_t = NodeId(commit_oid_hex)` happens exclusively in `Sequencer::apply_one` post-commit (§ 1 D2 below). All 7 transition bodies inherit this rule.

#### 0.3.2 SignalBundle shape
| Source | Says | Authority |
|---|---|---|
| STATE § 3 lines 403-409 (and parallels in § 3.1 / § 3.2) | `SignalBundle { boolean: vec![Signal::Boolean(BoolSignal::AcceptedAt(...))], statistical: vec![Signal::Statistical(StatSignal::PriceUpdate(...)), Signal::Statistical(StatSignal::ReputationDelta(...))] }` (two-axis vec with Bool/Stat sub-variants) | STATE v1.4 round-4 PASS/PASS |
| CO1.1.4-pre1 § 7.2 + shipped `src/state/typed_tx.rs:830-854` | `SignalBundle { kind: SignalKind }` with 4 variants only: `Empty / Finalize { claim_id, reward } / TaskExpired { task_id, bounty_refunded } / TerminalSummary { run_id, outcome }` | CO1.1.4-pre1 PASS/PASS |

**Resolution carried forward**: CO1.7.5 transition bodies emit `SignalBundle` matching the **shipped** 4-variant SignalKind only. Per the CO1.1.4-pre1 doc-comment "Full L6 signal-stream design is CO1.9", the BoolSignal/StatSignal richness is deferred. Shipped emit table:

| Transition body | Emits |
|---|---|
| step_transition (Work) | `SignalKind::Empty` |
| verify_transition | `SignalKind::Empty` |
| challenge_transition | `SignalKind::Empty` |
| reuse_transition | `SignalKind::Empty` |
| finalize_reward_transition | `SignalKind::Finalize { claim_id, reward }` |
| task_expire_transition | `SignalKind::TaskExpired { task_id, bounty_refunded }` |
| emit_terminal_summary_transition | `SignalKind::TerminalSummary { run_id, outcome }` |

CO1.9 will extend SignalKind with the deferred reputation / price-update / acceptance-event variants; CO1.7.5 makes **zero** SignalKind additions (preserves CO1.1.4-pre1 ABI lock).

### 0.4 Pre-implementation gate (from CO1.7 § 12)
> "CO1.7 must reach PASS/PASS before implementing CO1.7.5 (transition function bodies) + CO1.4-extra (CAS persistence)."

CO1.7-impl reached PASS/PASS-equivalent at commit `2461fe6` (per LATEST.md 2026-04-28 session-2 summary). CO1.4-extra shipped at commit `b6b7574`. **Pre-implementation gate is therefore satisfied**; CO1.7.5 spec is unblocked.

---

## § 1 Scope (4 deliverables; each is testable + audit-checkable)

### D1 — Per-kind transition function bodies (7)

Replace each `Err(TransitionError::NotYetImplemented)` arm in `src/state/sequencer.rs::dispatch_transition` (lines 53-61) with a real pure transition function. Each body lives in its OWN file under `src/state/transitions/` (NEW directory; one file per variant) and is called by `dispatch_transition` via a single-line forwarder. Authoritative pseudocode source: `STATE_TRANSITION_SPEC v1.4`.

| TypedTx variant | Function | Pseudocode source | Approx LoC | Key dependencies |
|---|---|---|---|---|
| `Work(WorkTx)` | `step_transition` | STATE § 3 (lines 328-419) | 100-150 | PredicateRegistry, ToolRegistry, EconomicState (balances/stakes/escrows/claims), ChallengeWindow.open |
| `Verify(VerifyTx)` | `verify_transition` | STATE § 3.1 (lines 423-477) | 60-90 | claim lookup, verifier quorum (default=1), reputation delta |
| `Challenge(ChallengeTx)` | `challenge_transition` | STATE § 3.2 (lines 478-574) | 80-120 | ChallengeWindow.is_open, predicate counterexample check, verifier_bond release policy, false-challenge reputation penalty (=0 v4) |
| `Reuse(ReuseTx)` | `reuse_transition` | STATE § 3.3 (lines 575-630) | 60-90 | ToolRegistry lookup, MAX_REUSE_ROYALTY_FRACTION (=0.10 default), edge weight, integer-floor royalty math |
| `FinalizeReward(FinalizeRewardTx)` | `finalize_reward_transition` | STATE § 3.4 (lines 631-706) | 80-120 | ChallengeWindow.is_open + MUST be CLOSED to fire; solver stake unlock + return; royalty integer floor |
| `TaskExpire(TaskExpireTx)` | `task_expire_transition` | STATE § 3.6 (lines 707-786) | 50-80 | TaskMarket lookup, deadline check, "no-claim-of-any-status" guard (Codex new-issue #2 fix), refund |
| `TerminalSummary(TerminalSummaryTx)` | `emit_terminal_summary_transition` | STATE § 3.7 (lines 825+) | 30-50 | run-state lookup, "no accepted work_tx exists" guard |

**Total**: ~460-700 LoC in 7 transition body files. Each gets a unit test module covering happy path + each `TransitionError` variant the function can return (per CO1.1.4-pre1 § 7.2 closed-taxonomy invariant).

**Purity contract** (STATE § 2 + § 3): every transition body
- takes `(&QState, &TxVariant, &PredicateRegistry, &ToolRegistry)` and returns `Result<(QState, SignalBundle), TransitionError>`;
- performs **no I/O**, **no env reads**, **no clock reads**, **no `HashMap` iteration**, **no `f64` arithmetic on monetary values** (per STATE § 2 expanded scope);
- mutates `q_next` cloned from `q`; returns `q_next` byte-identically deterministic across processes.

**Removal of legacy line**: STATE § 3 line 412 (`q_next.head_t = NodeId::from_state_root(...)`) is **excluded** from the Rust translation (per § 0.3 above). All other STATE § 3 stages 1-6 + 8 are translated faithfully.

### D2 — G-1 head_t carry-forward close

In `src/state/sequencer.rs::apply_one`, stage 9 currently mutates `q.ledger_root_t` from `entry.resulting_ledger_root` (lines 362-373) but leaves `q.head_t` unchanged (the K3 v1.2 deferral). CO1.7.5 closes this. Two `NodeId` types coexist in the codebase — the legacy `pub type NodeId = String` at `src/ledger.rs:13` and the new `pub struct NodeId(pub String)` at `src/state/q_state.rs:49`. `q.head_t` is typed as the **new** `state::q_state::NodeId` (tuple struct); CO1.7.5 D2 constructs the new variant exclusively. Legacy `crate::ledger::NodeId` is untouched by D2.

```rust
// Stage 9: commit + mutate Q_t under write lock.
let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
let mut writer_w = self.ledger_writer.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
writer_w.commit(&entry)?;
self.next_logical_t.store(logical_t, Ordering::SeqCst);
*q_w = q_next;
q_w.ledger_root_t = entry.resulting_ledger_root;
// NEW (CO1.7.5 D2): head_t = state::q_state::NodeId(commit_oid_hex)
if let Some(commit_oid_hex) = writer_w.head_commit_oid_hex() {
    q_w.head_t = crate::state::q_state::NodeId(commit_oid_hex);
}
```

**Atomicity** (extends v1.1 C-2 closure): under the acquired `q_w` + `writer_w` write locks, after `writer_w.commit(&entry)?` returns `Ok`, the remaining operations are an `AtomicU64::store` (infallible), a plain `*q_w = q_next` move (infallible), and two field assignments (infallible). There is no failure point between commit success and `head_t` advancement; `q.head_t` and `q.ledger_root_t` therefore advance atomically with `next_logical_t`. The single-writer per (runtime_repo, run_id) invariant from CO1.7 § 5.2.1 is preserved.

**Required trait extension**: the existing `LedgerWriter` trait does **NOT** expose `head_commit_oid()` (only `Git2LedgerWriter` has it). CO1.7.5 adds:

```rust
// In src/bottom_white/ledger/transition_ledger.rs
pub trait LedgerWriter: Send + Sync {
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
    fn len(&self) -> u64;

    /// NEW (CO1.7.5): canonical 40-char lowercase hex commit OID of the most
    /// recent appended entry, or None if the chain is empty / backend has no
    /// commit-OID notion (e.g. InMemoryLedgerWriter).
    ///
    /// Default impl returns None to preserve back-compat for any future
    /// non-git-backed writers; Git2LedgerWriter overrides with
    /// `self.head_commit_oid().map(|oid| oid.to_string())`.
    fn head_commit_oid_hex(&self) -> Option<String> {
        None
    }
}
```

**Rationale for `Option<String>` over `Option<git2::Oid>`**: `LedgerWriter` is the abstract trait; leaking `git2::Oid` across the trait boundary forces every consumer (incl. `InMemoryLedgerWriter`) to depend on git2-rs. Returning the **canonical hex string** keeps the trait backend-agnostic. The string is 40 lowercase hex chars (same as `git2::Oid::to_string()` default) — cheap to clone, easy to round-trip into `NodeId(String)`.

**InMemoryLedgerWriter behavior**: returns `None` (no git substrate → no commit OID → `q.head_t` stays at its prior value). This means tests using `InMemoryLedgerWriter` will see `head_t` NOT advance after commits — that's correct behavior, not a bug. The G-1 close only fires under `Git2LedgerWriter`.

**Why `if let Some(...)` instead of `unwrap()`**: tolerates `InMemoryLedgerWriter` deterministically. CO1.7.5+ never panics on `None`; it simply leaves `head_t` stale, which is acceptable for unit tests (they assert on `ledger_root_t` only, per the CO1.7-impl test suite).

### D3 — STEP_B ceremony for runtime Sequencer entry-point

`src/bus.rs:9` and `src/kernel.rs:8` both use legacy `crate::ledger::{Ledger, Node, NodeId, EventType, TapeError}` (retirement is CO1.1.5's job, not CO1.7.5's). `Bus::new(kernel: Kernel, config: BusConfig)` (bus.rs:87) shows Bus owns Kernel; the architecture below exploits this.

**Architecture (single owner; thin forwarder)**: Sequencer lives **inside Kernel** as `Option<Arc<Sequencer>>`. Bus does NOT hold a Sequencer field; it forwards `submit_typed_tx` calls to `self.kernel.sequencer`. This minimizes the Bus surface change (no new field — the spec-leaked struct shape stays put), keeps the typed-tx machinery as a kernel-level concern (state mutation lives in kernel), and reduces semver risk to one struct.

```rust
// src/kernel.rs (additive)
pub struct Kernel {
    // ... existing fields ...
    /// NEW (CO1.7.5 D3): None when kernel runs in legacy-only mode.
    pub sequencer: Option<Arc<Sequencer>>,
}

impl Kernel {
    pub fn new() -> Self {
        Self { /* ...existing..., */ sequencer: None }
    }
    /// NEW: opt-in constructor that wires a typed-tx Sequencer.
    pub fn with_sequencer(/* …existing args…, */ sequencer: Arc<Sequencer>) -> Self {
        Self { /* …existing…, */ sequencer: Some(sequencer) }
    }
}

// src/bus.rs (additive — NO new struct field)
impl Bus {
    /// NEW: typed-tx submission path. Forwards to kernel-owned Sequencer.
    /// Returns receipt (submit_id) immediately; commit happens asynchronously
    /// in Sequencer::run driver loop.
    pub async fn submit_typed_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
        match self.kernel.sequencer.as_ref() {
            Some(seq) => seq.submit(tx).await,
            None => Err(SubmitError::QueueClosed),
        }
    }
}
```

**STEP_B ceremony — combined (bus.rs + kernel.rs as one unit)**: per `handover/ai-direct/STEP_B_PROTOCOL.md` Phase 0 "minimum sufficient version", the bus.rs forwarder cannot be reasoned about without the kernel.rs field it forwards through. Splitting into two independent ceremonies would break the necessity-audit's "what observable behavior is broken now" question (each half is meaningless in isolation). Therefore:

1. Branch A (`step-b-co1.7.5-runtime-A`): edits BOTH `src/bus.rs` (add forwarder) AND `src/kernel.rs` (add field + constructor variant) to the spec-described target.
2. Branch B (`step-b-co1.7.5-runtime-B`): independently re-derives BOTH edits from this spec (separate session / context).
3. Byte-identity comparison: `diff src/bus.rs && diff src/kernel.rs` between branches A and B. **Both identical** → merge to `main`. **Either divergent** → re-do with stricter spec.

**Touched files**:
- `src/kernel.rs` (additive: 1 field + 1 constructor variant) — STEP_B-restricted; covered by combined ceremony
- `src/bus.rs` (additive: 1 forwarder method, NO struct field) — STEP_B-restricted; covered by combined ceremony
- `src/state/sequencer.rs` (D2 head_t close, D1 dispatch_transition forwarders) — additive; NOT STEP_B-restricted
- `src/state/transitions/{step,verify,challenge,reuse,finalize_reward,task_expire,terminal_summary}.rs` (NEW; 7 files) + `mod.rs` — NOT STEP_B-restricted
- `src/bottom_white/ledger/transition_ledger.rs` (D2 trait extension: 1 default method) — additive; NOT STEP_B-restricted
- `tests/transition_co1_7_5_*.rs` (NEW; 4 conformance tests + 7 transition body unit-test modules)

**`src/sdk/tools/wallet.rs` is NOT touched by CO1.7.5**. Per CO1.1.4-pre1, EconomicState mutations within transition bodies happen via `q.economic_state_t.{balances_t, stakes_t, escrows_t, claims_t}.method_call()` types under `src/economy/` (not the wallet tool). Smoke S5 verifies 0 hits. CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`) was sedimented and fixed in `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` during smoke phase; the STEP_B-restricted set now correctly reads `src/kernel.rs` + `src/bus.rs` + `src/sdk/tools/wallet.rs`.

### D4 — 4 CO1.7.5+ stage conformance tests + un-ignore

Per `CO1_7 § 7 table` (lines 510-521):

| Test | Stage | What it asserts |
|---|---|---|
| `tests/replay_full_transition_state_root` | NEW | FullTransition replay re-runs `dispatch_transition`; asserts state_root matches the entry's `resulting_state_root` byte-identically (I-DETHASH witness). |
| `tests/system_signature_verifies_via_canonical_message` | NEW | `LedgerEntry.system_signature` verifies through `verify_system_signature(&CanonicalMessage::LedgerEntrySigning(digest), epoch, pinned_pubkeys)`. |
| `tests/cas_payload_round_trip` | NEW | `CasStore::put` → `CasStore::get` round-trip; CID stable across runs (post-CO1.4-extra cold-restart). |
| `tests/sequencer_serial_replay_byte_identity` | un-ignore (currently `#[ignore = "CO1.7.5: requires real per-kind transition bodies"]` at `src/bottom_white/ledger/transition_ledger.rs:1451`) | Submit 100 TypedTx through Sequencer; replay → byte-identical `state_root`. |

Plus 7 transition-body unit-test modules (one per variant; each ≥ happy path + 1 rejection-error path per declared `TransitionError`).

---

## § 2 What this spec does NOT specify (out of scope)

1. **Legacy `src/ledger.rs` retirement** — CO1.1.5 atom (per `STATE_TRANSITION_SPEC § 5.3`). CO1.7.5 leaves the legacy WAL ledger fully running; bus.rs / kernel.rs continue to call `self.ledger.append(...)` for non-typed events.
2. **Multi-verifier quorum aggregation** — Q1.1 deferred to CO P2.7 atom (per STATE § 3.1 v1.1 note). v4 default `verifier_quorum_required = 1`.
3. **Slash transition** — K5 dropped per CO1.7 § 8. ChallengeCourt slashing event scheduled for CO P2.5 atom.
4. **Submission queue back-pressure tuning** — Q2 fixed at deterministic exponential backoff (Q2 v1.1 resolution); CO1.7.5 inherits it. Per-task fairness deferred to v4.x.
5. **STATE § 3 line 412 NodeId::from_state_root patch** — CO1.7.5 spec lands D1-D4. STATE_TRANSITION_SPEC v1.5 single-line clarification is a separate housekeeping commit (NOT re-audit; § 0.3 above).
6. **Materializer state_root computation** — CO1.8 atom (L5). CO1.7.5 transition bodies build `q_next.state_root_t` via the same `q_next.economic_state_t.derive_state_root()` accessor that CO1.7-impl tests already exercise; CO1.8 will replace this with a real merkleized materializer.

---

## § 3 Open questions (5 closed in smoke; 1 remains)

Per memory `feedback_smoke_before_batch` + the lossless-compression principle (close what smoke can answer; only outsource genuine design tradeoffs to audit).

### 3.1 Closed by smoke

| Q | Resolution | Closure rationale |
|---|---|---|
| **Q2 transition body file structure** | **CLOSED**: per-variant file under `src/state/transitions/{step,verify,challenge,reuse,finalize_reward,task_expire,terminal_summary}.rs` + `mod.rs`. | One file per pure transition matches the existing module-per-concern pattern in `src/state/`. Single-file-700-LoC variant rejected: harder to TRACE_MATRIX-annotate per variant. |
| **Q3 STEP_B per file vs combined** | **CLOSED**: combined ceremony covering bus.rs + kernel.rs as one A/B unit (§ 1 D3). | Per-file ceremony fails STEP_B Phase 0 "minimum sufficient version" — the bus.rs forwarder is meaningless without the kernel.rs field it forwards through; A/B byte-identity check is only meaningful on the coupled change. |
| **Q4 SignalBundle population** | **CLOSED**: shipped 4-variant `SignalKind` (Empty/Finalize/TaskExpired/TerminalSummary) suffices for all 7 transition bodies; full emit table in § 0.3.2 above. | Smoke read of `src/state/typed_tx.rs:830-854` confirmed CO1.1.4-pre1 minimization. STATE § 3 BoolSignal/StatSignal richness is CO1.9 scope; CO1.7.5 makes zero SignalKind additions. |
| **Q5 TransitionError 22-variant adequacy** | **CLOSED**: every STATE § 3.1-3.7 rejection path maps to an existing variant. Mapping table below. | Smoke verified the 22 variants in `src/state/typed_tx.rs:716-788` against the rejection paths in STATE § 3 / § 3.1 / § 3.2 / § 3.3 / § 3.4 / § 3.6 / § 3.7. Minimal-payload pattern (rich context flows via RejectedAttemptSummary side channel; per typed_tx.rs:710-715 doc-comment) is preserved. |
| **Q6 head_t test policy under InMemoryLedgerWriter** | **CLOSED**: InMemory tests assert `q.head_t` unchanged across commits (default impl `head_commit_oid_hex() == None`); Git2-backed tests (incl. un-ignored `sequencer_serial_replay_byte_identity`) assert `q.head_t` advances and matches `Git2LedgerWriter::head_commit_oid().to_string()`. | The split is the natural consequence of the trait's default-None contract; audit can dispute, but the alternative (faking a deterministic OID in InMemory) introduces test-doubles divergence with no benefit. |

#### Q5 mapping table (STATE § 3.x rejection path → shipped TransitionError variant)

| STATE pseudocode call | Variant used | Source |
|---|---|---|
| `Err(TransitionError::StaleParent { … })` (§ 3 stage 1) | `StaleParent` (no payload — context via RejectedAttemptSummary) | typed_tx.rs:720 |
| `Err(TransitionError::SignatureInvalid)` (§ 3 / § 3.1 / § 3.2 stage 2) | `SignatureInvalid` | typed_tx.rs:722 |
| `Err(TransitionError::StakeInsufficient { … })` (§ 3 / § 3.1 / § 3.2 stage 2/3) | `StakeInsufficient` (no payload) | typed_tx.rs:731 |
| `Err(TransitionError::AcceptancePredicateFailed(acceptance_results))` (§ 3 stage 4) | `AcceptancePredicateFailed(PredicateId)` (single failing PredicateId, not the full bundle) | typed_tx.rs:745 |
| `Err(TransitionError::TargetWorkTxNotFound)` (§ 3.1/3.2 stage 1) | `TargetWorkTxNotFound` | typed_tx.rs:735 |
| `Err(TransitionError::TargetWorkTxNotVerifiable)` (§ 3.1 stage 1) | `TargetWorkTxNotVerifiable` | typed_tx.rs:737 |
| `Err(TransitionError::VerificationPredicateFailed(verify_results))` (§ 3.1 stage 3) | `VerificationPredicateFailed(PredicateId)` | typed_tx.rs:747 |
| `Err(TransitionError::ChallengeWindowClosed)` (§ 3.2 stage 1) | `ChallengeWindowClosed` | typed_tx.rs:753 |
| `Err(TransitionError::CounterexampleInsufficient(counter_check))` (§ 3.2 stage 3) | `CounterexampleInsufficient` (no payload) | typed_tx.rs:759 |
| `Err(TransitionError::ToolNotInRegistry)` / `ToolCreatorMismatch` / `ParentNotAcceptedYet` (§ 3.3) | matching shipped variants | typed_tx.rs:739, 763, 765 |
| `Err(TransitionError::ChallengeWindowStillOpen)` / `AlreadySlashed` / `ClaimNotFound` (§ 3.4) | matching shipped variants | typed_tx.rs:755, 757, 769 |
| `Err(TransitionError::SettlementPredicateFailed(predicate_id))` (§ 3.4) | `SettlementPredicateFailed(PredicateId)` | typed_tx.rs:749 |
| `Err(TransitionError::TaskNotFound)` / `TaskNotExpired` / `TaskHasOpenClaim` (§ 3.6) | matching shipped variants | typed_tx.rs:773, 775, 777 |
| `Err(TransitionError::TerminalSummaryNotApplicable)` (§ 3.7) | `TerminalSummaryNotApplicable` | typed_tx.rs:781 |
| System-emitted tx signature failure | `InvalidSystemSignature` | typed_tx.rs:724 |

Coverage: 22 / 22 shipped variants accounted for (15 used + 7 indirectly: `NotYetImplemented` is the stub sentinel that disappears post-CO1.7.5; the others map 1-to-1). CO1.7.5 adds **zero** new variants.

### 3.2 Remains open for round-1 audit

| Q | Conservative resolution proposed | Audit input requested |
|---|---|---|
| **Q1 `head_commit_oid_hex` default impl** | `fn head_commit_oid_hex(&self) -> Option<String> { None }` default in trait (back-compat for any future non-git writer). | Should the default be `unimplemented!()` instead — forcing every impl to declare and preventing silent-None bugs? Tradeoff: stricter contract vs. forward-compat ergonomics. |

---

## § 4 Audit gates (round structure)

Following `CO1_7 § 12` precedent:

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 | ⏳ pending | ⏳ pending | TBD | dual external audit on v1 (this draft) |
| 2 | … | … | … | iterate per merged verdict |
| 3+ | … | … | … | iterate to PASS/PASS |

**Pre-audit smoke verification gate** (per memory `feedback_smoke_before_batch`): before sending v1 to dual audit, smoke-verify all spec claims against current code state. Smoke plan is § 7 below; results recorded in this same file's "Pre-audit smoke results" footer (added at audit-send time). Without smoke-verified baseline, any CHALLENGE on "X assumption is wrong" wastes a round.

**Pre-implementation gate**: spec must reach PASS/PASS before any code in `src/state/transitions/`, `src/bus.rs`, `src/kernel.rs`, or `src/state/sequencer.rs` D2 lines is written. Per CLAUDE.md "Audit Standard" + memory `feedback_dual_audit`.

---

## § 5 Estimated scope

- **Spec rounds**: 1-2 expected (CO1.7.5 inherits frozen ABI; 5 of 6 v1 open Qs already closed by smoke; only Q1 remains as audit input — substantively smaller surface than CO1.7 v1's 11 must-fix items).
- **Implementation scope** (post-PASS/PASS):
  - D1 transition bodies: ~460-700 LoC + 7 unit-test modules (~250-400 LoC tests)
  - D2 head_t wiring: ~20-40 LoC (single Sequencer apply_one patch + 1 trait method default)
  - D3 STEP_B combined ceremony (kernel.rs + bus.rs): ~30-60 LoC per branch × 2 A/B branches = ~60-120 LoC of total restricted-file delta
  - D4 4 conformance tests: ~200-300 LoC
- **Total atom budget**: ~1,000-1,560 LoC (down from initial 1,200-1,900 because Bus stays struct-shape-compatible after D3 simplification). **Estimated calendar time**: 4-7 days. Implementation may bundle as `CO1.7.5-impl A1+A2+A3+A4` like CO1.7-impl, OR ship as 4 sequential commits.
- **Audit cost**: ~$10-25 (1-2 rounds at ~$10-17 each).

---

## § 6 Honest acknowledgements (v1)

1. **Two STATE § 3 supersessions adopted, not introduced** (§ 0.3): NodeId-from-commit-OID + 4-variant SignalKind. Both are CO1.7-K3-v1.2 and CO1.1.4-pre1 minimization decisions that already cleared dual audit; CO1.7.5 carries them forward unchanged. Auditors should validate the authority chain (CO1.7 v1.2 → CO1.7.5 vs STATE v1.4) — that's the substantive review surface, not the resolutions themselves.
2. **`head_commit_oid_hex` is a NEW trait method**: additive (default impl returns `None`); no existing `LedgerWriter` impl breaks. Net new public API surface = 1 method. Default `None` vs `unimplemented!()` is the sole open audit-input question (Q1).
3. **`src/state/transitions/` is a NEW directory** (per Q2 closure): 7 transition body files + `mod.rs`. Each file gets a doc-comment `/// TRACE_MATRIX § 3.x — pure transition for <Variant>` mapping per CLAUDE.md "Alignment Standard" (every src/ pub symbol → flowchart element).
4. **Combined STEP_B ceremony** (per Q3 closure): one A/B unit covers `src/bus.rs` + `src/kernel.rs` together because the bus.rs forwarder is meaningless without kernel.rs's Sequencer field. Per-file ceremony was rejected for failing STEP_B Phase 0 "minimum sufficient version".
5. **`src/sdk/tools/wallet.rs` and `src/wal.rs` untouched**: smoke S5 verifies 0 hits in wallet; spec § 9 of CO1_7 v1.2 separately confirms wal.rs is not STEP_B-restricted. CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`) was sedimented + fixed in `OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` during smoke; the restricted set now reads `src/{kernel,bus}.rs + src/sdk/tools/wallet.rs`.
6. **FC-trace requirements** (per CLAUDE.md "Alignment Standard"): every NEW pub symbol introduced by CO1.7.5 must carry a doc-comment `/// TRACE_MATRIX <FC-id>: <role>` backlink. The set is: 7 transition body fns (each → STATE § 3.x); `LedgerWriter::head_commit_oid_hex` (→ § 5 L4 sequencer post-commit head_t wiring); `Kernel.sequencer` field + `Kernel::with_sequencer` + `Bus::submit_typed_tx` (→ § 5.2.1 single-writer entry-point). `tests/fc_alignment_conformance.rs` witness-test additions are scoped under the implementation atom (not this spec).

---

## § 7 Pre-audit smoke test plan (mandatory before round-1 audit)

Per memory `feedback_smoke_before_batch`. Each item is a runnable check; all must pass before the v1 spec is sent to Codex + Gemini.

| # | Claim in spec | Smoke command | Pass criterion |
|---|---|---|---|
| S1 | "every variant returns `Err(TransitionError::NotYetImplemented)`" (§ 0.1) | `grep -c 'NotYetImplemented' src/state/sequencer.rs` lines 53-61 | exactly 7 hits |
| S2 | "`sequencer_serial_replay_byte_identity` is currently `#[ignore]`" (§ 1 D4) | `grep -B1 'fn sequencer_serial_replay_byte_identity' src/bottom_white/ledger/transition_ledger.rs` | preceding line contains `#[ignore]` |
| S3 | "`Git2LedgerWriter::head_commit_oid()` is exposed and returns `Option<git2::Oid>`" (§ 1 D2) | `grep -A1 'fn head_commit_oid' src/bottom_white/ledger/transition_ledger.rs` | signature `pub fn head_commit_oid(&self) -> Option<git2::Oid>` |
| S4 | "bus.rs / kernel.rs use legacy `crate::ledger`" (§ 1 D3) | `grep -n 'use crate::ledger::' src/bus.rs src/kernel.rs` | both files have a `use crate::ledger::...` line |
| S5 | "wallet (at `src/sdk/tools/wallet.rs`) is untouched" (§ 1 D3) | `grep -c 'transition_ledger\|state::sequencer\|TypedTx' src/sdk/tools/wallet.rs` | 0 hits. Note: `src/wallet.rs` does NOT exist at HEAD `2f5093a` (CLAUDE.md "Code Standard" + STEP_B_PROTOCOL.md path is stale; logged as OBS post-PASS/PASS — see § 1 D3 closing paragraph). |
| S6 | "QState.head_t is `NodeId`" (§ 1 D2) | `grep -A2 'pub head_t' src/state/q_state.rs` | type is `NodeId` |
| S7 | "TransitionError has 22 variants" (§ 0.1) | `grep -c '^\s*[A-Z][a-zA-Z]*\(.*\)\?,$\|^\s*[A-Z][a-zA-Z]*$\|^\s*NotYetImplemented' src/state/typed_tx.rs` (within `enum TransitionError`) | 22 |
| S8 | baseline cargo state | `cargo check --workspace && cargo test --workspace --lib` | check passes; tests = 239/0/1 (the ignored is `sequencer_serial_replay_byte_identity`) |

If any of S1-S8 fails, the spec cite at issue is **wrong** and MUST be patched before round-1 audit (CHALLENGE-avoidance per `feedback_smoke_before_batch`).

---

**END v1 DRAFT body.**

---

## Pre-audit smoke results (footer; populated 2026-04-29 pre-round-1)

Smoke run at HEAD `2f5093a` (working tree clean except `rules/enforcement.log` hook-only WARN appendage).

| # | Claim | Smoke command | Result | Status |
|---|---|---|---|---|
| S1 | 7× NotYetImplemented in dispatch_transition lines 53-61 | `sed -n '53,61p' src/state/sequencer.rs \| grep -c NotYetImplemented` | `7` | ✅ PASS |
| S2 | `#[ignore]` precedes `sequencer_serial_replay_byte_identity` | `grep -B2 'fn sequencer_serial_replay_byte_identity' src/bottom_white/ledger/transition_ledger.rs` | `#[ignore = "CO1.7.5: requires real per-kind transition bodies"]` precedes `fn sequencer_serial_replay_byte_identity` | ✅ PASS (with reason string) |
| S3 | `Git2LedgerWriter::head_commit_oid` returns `Option<git2::Oid>` | `grep -A1 'pub fn head_commit_oid' src/bottom_white/ledger/transition_ledger.rs` | `pub fn head_commit_oid(&self) -> Option<git2::Oid>` | ✅ PASS |
| S4 | bus.rs + kernel.rs use legacy `crate::ledger::...` | `grep -n 'use crate::ledger::' src/bus.rs src/kernel.rs` | `src/bus.rs:9: use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};` + `src/kernel.rs:8: use crate::ledger::{Node, NodeId, Tape, TapeError};` | ✅ PASS |
| S5 | wallet untouched | `grep -c 'transition_ledger\|state::sequencer\|TypedTx' src/sdk/tools/wallet.rs` (NOT `src/wallet.rs` — that path does not exist) | `0` | ✅ PASS (with path correction; CLAUDE.md + STEP_B_PROTOCOL.md path drift sedimented as separate OBS post-PASS/PASS) |
| S6 | QState.head_t is `NodeId` | `grep -B1 -A1 'pub head_t' src/state/q_state.rs` | `pub head_t: NodeId,` (line 311) | ✅ PASS |
| S7 | TransitionError has 22 variants | `awk '/^pub enum TransitionError/,/^impl /' src/state/typed_tx.rs \| grep -cE '^\s*[A-Z][A-Za-z]+(\(.*\))?,?\s*$'` | `22` | ✅ PASS |
| S8 | cargo baseline | `cargo check --workspace && cargo test --workspace --lib` | check: 18 warnings (pre-existing), 0 errors. lib tests: `239 passed; 0 failed; 1 ignored` (the ignored test is `sequencer_serial_replay_byte_identity` per S2). | ✅ PASS |

**Smoke gate**: 8 / 8 PASS.

### Patch log (this session, pre round-1)

**Smoke phase (S5 + S2 findings)**:
- P1: § 1 D3 wallet wording — note `src/wallet.rs` does not exist; wallet at `src/sdk/tools/wallet.rs`; CLAUDE.md path drift observation
- P2: § 7 S5 smoke row corrected to point at `src/sdk/tools/wallet.rs`
- P3: § 6 ack #8 wallet path correction
- P4: § 1 D4 un-ignore reason string precision

**Self-audit phase (8 patches; principle: 无损压缩即智能 — close what smoke can answer; only outsource genuine design tradeoffs to audit)**:
- P5: § 0.3 restructured as **two** STATE supersessions (NodeId + SignalBundle); tightened "re-audit" framing (CO1.7.5 spec only documents carry-forward; STATE re-audit is the STATE curator's decision, not this spec's)
- P6: § 1 D2 added NodeId disambiguation (legacy `crate::ledger::NodeId` = String alias vs new `state::q_state::NodeId` tuple struct; D2 uses the new one only) + atomicity proof (no failure point between writer.commit success and head_t store under acquired lock)
- P7: § 1 D3 architecture simplification — Sequencer lives in Kernel only (not Bus); Bus exposes thin forwarder `submit_typed_tx` via `self.kernel.sequencer`; **combined STEP_B ceremony** (one A/B unit covers both files) replaces per-file ceremony — justified by STEP_B Phase 0 "minimum sufficient version" since the Bus forwarder is meaningless without the Kernel field
- P8: § 3 closed Q2 (per-variant file structure committed), Q3 (combined ceremony committed), Q4 (SignalKind 4-variant minimal sufficient + emit table), Q5 (TransitionError 22-variant adequacy proven by mapping table), Q6 (head_t test policy split between InMemory/Git2 backends decided) — only Q1 (`head_commit_oid_hex` default impl) remains as genuine design tradeoff for round-1 audit
- P9: § 6 dropped acknowledgements #5-7 that duplicated § 3; added FC-trace requirements (item #6) per CLAUDE.md "Alignment Standard"
- P10: § 5 estimated scope tightened — D2 LoC reduced (single trait method default), D3 LoC reduced (Bus stays struct-shape-compatible), audit cost halved (1-2 rounds expected vs 1-3)
- P11: § 0.2 deliverables tightened to one sentence per D-item, surfacing the architecture choices made in P5-P8
- P12: hygiene fix landed `CLAUDE.md:14` + `STEP_B_PROTOCOL.md:3` to correct path drift; `OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` records rationale

### Awaiting

1. round-1 dual external audit (Codex + Gemini)
2. iterate to PASS/PASS per `CO1_7 § 12` precedent (1-2 rounds expected given simplified surface)
3. then CO1.7.5-impl (D1 + D2 + D3 STEP_B + D4)
