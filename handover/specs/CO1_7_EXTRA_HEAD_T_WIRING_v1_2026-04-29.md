# CO1.7-extra: L4 head_t close + Sequencer entry-point wiring v1 (post round-1 scope split)

**Status**: v1 DRAFT (2026-04-29; post round-1 dual external audit on prior bundled CO1.7.5 spec). Scope-split executed by ArchitectAI per Occam + Anti-Oreo (see § 0.1). Awaiting round-2 dual external audit.
**Author**: ArchitectAI (Claude); session 2026-04-29.
**Supersedes**: prior bundled `CO1_7_5_TRANSITION_BODIES_AND_RUNTIME_WIRING_v1_2026-04-29.md` (committed `334111a`; round-1 CHALLENGE/CHALLENGE; preserved in git history).
**Pre-implementation gate**: PASS/PASS dual external audit before any code lands. Per CLAUDE.md "Audit Standard".

**Companion specs (frozen, read first)**:
- `CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` v1.2 — round-3 PASS/PASS; freezes `LedgerWriter` trait + Sequencer 9-stage apply_one + `Git2LedgerWriter::head_commit_oid()`.
- `CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — frozen 7-variant TypedTx; not directly touched here.
- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — referenced for K3 v1.2 supersession authority only; transition bodies are out of scope for this atom.
- `handover/audits/CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md` — round-1 merged verdict that drove this scope split.

**Single sentence**: close the G-1 carry-forward `q.head_t = NodeId(commit_oid_hex)` after `Git2LedgerWriter.commit`, perform combined STEP_B ceremony adding a Sequencer entry-point on TuringBus + Kernel, and ship one substrate-independent CAS round-trip test — leaving transition function bodies + replay byte-identity to a future CO1.7.5 atom that depends on the Wave-2 substrate (CO P2.x family).

---

## § 0 Scope decision (round-1 driven)

### 0.1 Why this atom exists (Occam-driven scope split)

Round-1 dual external audit on the prior bundled CO1.7.5 v1 spec (`334111a`) returned CHALLENGE/CHALLENGE. The conservative-merged verdict (`CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md`) found that the v1 bundling crossed Anti-Oreo three-layer boundaries: D1 transition bodies require FC1 top-white predicate execution methods + FC2 middle-black state schemas that don't exist in shipped code (CO P2.x family substrate not yet shipped per `PROJECT_DECISION_MAP § 3.4`).

Per "无损压缩即智能" + Anti-Oreo + Occam:

| Atom | Owns | Substrate dependency | Ships when |
|---|---|---|---|
| **CO1.7-extra (THIS spec)** | D2 head_t close + D3 Sequencer entry-point wiring + 1 substrate-independent test | None — uses only frozen `LedgerWriter` trait + `Git2LedgerWriter::head_commit_oid()` + existing `CasStore::put`/`get` | Now (post-PASS/PASS) |
| **CO1.7.5 (future; restored to CO1.7 § 13 original meaning)** | D1 transition bodies (7) + 3 D4 tests + un-ignore `sequencer_serial_replay_byte_identity` | CO P2.1 / P2.2 / P2.3 / P2.5 / P2.6 / P2.7 / P2.9 + CO1.11 + (new) PredicateRegistry execution-methods atom | After substrate atoms reach individual PASS/PASS |

The split uses the `CO1.4-extra` precedent (small bridge atom alongside larger primary atom). Zero new architectural concepts introduced.

### 0.2 What this atom inherits (frozen)

| Frozen by | Surface |
|---|---|
| CO1.7-impl A1 (commit `2461fe6`) | `LedgerEntry` 9-field signing surface + `Git2LedgerWriter` + `InMemoryLedgerWriter` + `head_commit_oid()` accessor |
| CO1.7-impl A2 (commit `2461fe6`) | `Sequencer` 9-stage `apply_one` + `dispatch_transition` exhaustive match (variants stay `Err(NotYetImplemented)` post-CO1.7-extra; D1 transition bodies are out of scope) |
| CO1.4-extra (commit `b6b7574`) | CAS sidecar JSONL index persistence (substrate for the cas_payload_round_trip test) |

### 0.3 What this atom delivers (new)

1. **D2** — `q.head_t = state::q_state::NodeId(commit_oid_hex)` after `writer.commit(&entry)` returns Ok; adds 1 trait method `LedgerWriter::head_commit_oid_hex` with mandatory-override design pattern (Q1 synthesis from round-1).
2. **D3** — Combined STEP_B ceremony adds `Option<Arc<Sequencer>>` field to `Kernel` + `submit_typed_tx` forwarder method on `TuringBus` (note: type is `TuringBus`, not `Bus`, per `src/bus.rs:53`). Sequencer instance lives in Kernel; TuringBus forwards via `self.kernel.sequencer`.
3. **D4-substrate-independent** — One conformance test `tests/cas_payload_round_trip` (`CasStore::put` → `get` round-trip with CID stability post-CO1.4-extra). Other 3 D4 tests (replay state-root + system-signature canonical-message + un-ignore byte-identity) move to future CO1.7.5 atom because they require D1 transition bodies to actually commit.

### 0.4 Process commitment (active reconciliation per Gemini MF1+MF3 + Codex Q-A v1.1 ask)

The two STATE_TRANSITION_SPEC § 3 supersessions previously declared in the prior CO1.7.5 v1 spec (NodeId head_t binding + SignalKind 4-variant minimization) **continue to apply** — but no longer in scope for CO1.7-extra (which doesn't contain transition bodies). They migrate intact to the future CO1.7.5 atom.

**Asserted authority principle** (strengthened per Gemini MF3): a later, more specific, audited spec (CO1.7 v1.2 round-3 PASS/PASS; CO1.1.4-pre1 PASS/PASS) **legitimately supersedes** earlier general specs (STATE v1.4 round-4 PASS/PASS) within the layered boundary the later spec covers. This is consistent with the project's atom-decomposition pattern: each atom locks its own surface; downstream atoms refine via PASS/PASS audit, not by editing upstream artifacts.

**Institutional debt acknowledged** (per Gemini MF1): as part of CO1.7-extra atom closure, ArchitectAI commits to filing a STATE_TRANSITION_SPEC v1.5 housekeeping issue (one paragraph noting the two supersessions from CO1.7 K3 v1.2 + CO1.1.4-pre1 with backlinks) — NOT a re-audit, just an annotation pass that prevents future readers from being confused by the historical drafting language. Tracked as part of the post-PASS/PASS landing checklist (§ 8 awaiting list).

---

## § 1 D2 — head_t close

### 1.1 Code change

In `src/state/sequencer.rs::apply_one` stage 9 (currently lines 362-373), one additional assignment after `writer_w.commit(&entry)?`:

```rust
// Stage 9 (CO1.7-extra D2): commit + mutate Q_t under write lock.
let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
let mut writer_w = self.ledger_writer.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
writer_w.commit(&entry)?;
self.next_logical_t.store(logical_t, Ordering::SeqCst);
*q_w = q_next;
q_w.ledger_root_t = entry.resulting_ledger_root;
// NEW (CO1.7-extra D2): close G-1 head_t carry-forward.
if let Some(commit_oid_hex) = writer_w.head_commit_oid_hex() {
    q_w.head_t = crate::state::q_state::NodeId(commit_oid_hex);
}
```

**NodeId disambiguation**: two `NodeId` types coexist — legacy `pub type NodeId = String` at `src/ledger.rs:13` (imported by TuringBus + Kernel for the legacy ledger event API) and new `pub struct NodeId(pub String)` at `src/state/q_state.rs:49`. `q.head_t` is typed as the new tuple-struct (`q_state.rs:311`); D2 constructs the new variant exclusively (legacy String alias is unused here).

**Atomicity** (per Codex Q-B finding, refined): under acquired `q_w` + `writer_w` write locks, after `writer_w.commit(&entry)?` returns `Ok`, the remaining operations are an `AtomicU64::store` (infallible), a plain `*q_w = q_next` move (infallible), and field assignments (infallible). The atomicity claim fully holds for writers whose `head_commit_oid_hex` returns `Some` (Git2LedgerWriter); writers returning `None` (InMemoryLedgerWriter) leave `q.head_t` unchanged from `q_next.head_t` (which equals `q.head_t` per CO1.7 K3 v1.2 — transition bodies don't mutate head_t even when they exist in CO1.7.5).

### 1.2 Trait method addition (Q1 synthesis: default None + mandatory override + defensive test)

`LedgerWriter` trait at `src/bottom_white/ledger/transition_ledger.rs` gains one method:

```rust
pub trait LedgerWriter: Send + Sync {
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
    fn len(&self) -> u64;
    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;  // (existing; spec preserves)

    /// NEW (CO1.7-extra D2): canonical 40-char lowercase hex commit OID of the
    /// most recent appended entry, or None if the chain is empty / backend has
    /// no commit-OID notion.
    ///
    /// **Q1 synthesis** (round-1 audit): default returns None to preserve
    /// post-commit no-failure goal (avoid panic-after-commit-success per Codex
    /// Q-B); BUT every shipped LedgerWriter impl MUST explicitly override this
    /// (Gemini Q8 silent-stagnation defense). Defensive test
    /// `git2_writer_returns_some_after_commit` (§ 3) asserts Git2LedgerWriter
    /// returns Some at commit time, catching silent stagnation bugs in CI.
    /// The default-None impl is intentionally dead code in production.
    fn head_commit_oid_hex(&self) -> Option<String> {
        None
    }
}

impl LedgerWriter for Git2LedgerWriter {
    fn head_commit_oid_hex(&self) -> Option<String> {
        self.head_commit_oid().map(|oid| oid.to_string())
    }
    // ... existing commit / len / read_at ...
}

impl LedgerWriter for InMemoryLedgerWriter {
    /// Explicit override (mandatory per Q1 synthesis). InMemory has no git
    /// substrate, so always None — but the override is required to make the
    /// "no implicit None" mandate enforceable (a missing override means the
    /// dead-default is reached, which the defensive test will fail-fast on for
    /// any code path that passes through Git2LedgerWriter).
    fn head_commit_oid_hex(&self) -> Option<String> {
        None
    }
    // ... existing ...
}
```

---

## § 2 D3 — Combined STEP_B ceremony for runtime entry-point

### 2.1 Code change

`src/kernel.rs` (currently `pub struct Kernel { ... }` with `Debug, Serialize, Deserialize` derives at line 18; documented as "pure topology" at line 15-17):

```rust
// src/kernel.rs (additive)
pub struct Kernel {
    // ... existing fields ...

    /// NEW (CO1.7-extra D3): typed-tx Sequencer; None when kernel runs in
    /// legacy-only mode (preserves back-compat with all existing tests).
    /// Marked serde-skip because Sequencer holds Arc-locked CAS / writer state
    /// that is constructed at runtime, not from on-disk Q_t snapshots.
    #[serde(skip)]
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
```

`src/state/sequencer.rs` (Sequencer struct currently at lines 190-207 has no derives per Codex Q-C):

```rust
// src/state/sequencer.rs (additive — Debug derive needed for Kernel.Debug propagation)
#[derive(Debug)]  // NEW (CO1.7-extra D3); Q1' open audit input
pub struct Sequencer { /* ... */ }
```

`#[derive(Debug)]` may not propagate cleanly across `Arc<RwLock<dyn LedgerWriter>>` (the trait object in field position). If blanket derive fails at compile, manual impl uses `f.debug_struct("Sequencer").finish_non_exhaustive()` (sole open question Q1' below).

`src/bus.rs` (note: actual struct name is **`TuringBus`** at `src/bus.rs:53`, NOT `Bus`):

```rust
// src/bus.rs (additive — NO new struct field)
impl TuringBus {
    /// NEW (CO1.7-extra D3): typed-tx submission path. Forwards to kernel-owned
    /// Sequencer. Returns receipt (submit_id) immediately; commit happens
    /// asynchronously in Sequencer::run driver loop.
    pub async fn submit_typed_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
        match self.kernel.sequencer.as_ref() {
            Some(seq) => seq.submit(tx).await,
            None => Err(SubmitError::QueueClosed),
        }
    }
}
```

### 2.2 Sequencer placement justification (per Codex Q-C concern)

`src/kernel.rs:15-17` doc says Kernel is "pure topology". Adding Sequencer as a new field appears to violate that descriptor at first glance. Resolution:

1. Kernel already holds `Tape` + `NodeId` from the legacy ledger (`src/kernel.rs:8`) — these are "topology" elements (DAG structure + node identity). Sequencer is the typed-tx topology element (submission queue + driver loop ordering); it parallels the existing Tape/NodeId pattern.
2. The actual state (`Q_t`) is owned by Sequencer, not Kernel. Kernel holds the *driver*, not the *data*.
3. As part of this atom landing, the kernel.rs doc-comment is patched to: "topology layer: holds Tape, NodeId, and (post-CO1.7-extra) the typed-tx Sequencer driver. State data lives in Q_t inside Sequencer or in the legacy WAL ledger; this layer does NOT hold raw user-state."

### 2.3 Combined ceremony justification (refined per Codex Q-C)

Per `STEP_B_PROTOCOL.md` Phase 0, "minimum sufficient version" is technically **advisory** language asking auditors to favor the smallest change that works. CO1.7-extra rests the combined-ceremony argument on **functional coupling** (a stronger criterion):

- The TuringBus forwarder reads `self.kernel.sequencer`; without the Kernel field, the forwarder fails to compile.
- The Kernel field has no observable effect without an external caller; without the TuringBus forwarder, the field is dead code.

Each half is a no-op without the other. A/B byte-identity testing each half independently would test two non-functional changes; combining them into one A/B unit tests the actual minimum-functional change. This is a **stronger** application of STEP_B's spirit than the per-file alternative.

**Ceremony procedure**:
1. Branch A (`step-b-co1.7-extra-A`): edits BOTH `src/bus.rs` (TuringBus forwarder) AND `src/kernel.rs` (Sequencer field + with_sequencer constructor) per § 2.1. Also adds the `#[derive(Debug)]` on Sequencer in `src/state/sequencer.rs` (NOT STEP_B-restricted; landed alongside the ceremony for compile coherence).
2. Branch B (`step-b-co1.7-extra-B`): independently re-derives the same edits from this spec (separate session / context).
3. Byte-identity comparison: `diff src/bus.rs && diff src/kernel.rs` between A and B. Both identical → merge to `main`. Either divergent → re-do the **whole** ceremony with stricter spec (no split-and-redo; coupled changes need coupled re-derivation).

---

## § 3 Test plan (substrate-independent)

Two tests in `tests/co1_7_extra/`:

### 3.1 `cas_payload_round_trip`

```rust
// tests/co1_7_extra/cas_payload_round_trip.rs (NEW)
//! CO1.7-extra D4: CAS payload round-trip + CID stability across restart.
//! Verifies that CO1.4-extra sidecar persistence makes CasStore content
//! reachable across cold-start, which is a precondition for CO1.7.5
//! FullTransition replay (deferred; gated on substrate atoms).
//! Substrate-independent: uses only CasStore + ObjectType (CO1.4 + CO1.4-extra
//! shipped surfaces); does NOT depend on CO P2.x.

#[test]
fn cas_payload_round_trip_with_cid_stability_across_restart() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let payload = b"co1.7-extra-deterministic-payload-v1";
    let cid_first = {
        let mut cas = CasStore::open(tmp.path()).expect("first open");
        cas.put(payload, ObjectType::ProposalPayload, "test-epoch", 1, Some("CO1.7-extra".into()))
            .expect("put")
    };
    // Drop CasStore handle; reopen (cold-start path).
    let bytes = {
        let cas = CasStore::open(tmp.path()).expect("reopen post-restart");
        cas.get(&cid_first).expect("get post-restart")
    };
    assert_eq!(bytes.as_slice(), payload);
}
```

### 3.2 `git2_writer_returns_some_after_commit` (Q1 synthesis defensive test)

```rust
// tests/co1_7_extra/git2_writer_head_oid_defense.rs (NEW)
#[test]
fn git2_writer_returns_some_after_commit() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let mut writer = Git2LedgerWriter::open(tmp.path()).expect("open");
    let entry = canonical_test_entry(1);
    writer.commit(&entry).expect("commit");
    // Defensive against silent head_t stagnation per Gemini Q8 concern.
    // If Git2LedgerWriter ever inherits the default-None impl by accident
    // (refactor regression / forgotten override), this fails fast in CI.
    assert!(
        writer.head_commit_oid_hex().is_some(),
        "Git2LedgerWriter MUST return Some after commit; default-None inheritance = constitutional anchor violation"
    );
}
```

Total: 2 tests.

---

## § 4 Out of scope (explicitly deferred)

1. **D1 transition function bodies (7)** — moved to future CO1.7.5 atom; gated on CO P2.x substrate atoms (§ 0.1 table).
2. **3 of 4 D4 tests** (`replay_full_transition_state_root`, `system_signature_verifies_via_canonical_message`, un-ignore `sequencer_serial_replay_byte_identity`) — all require D1 to actually commit; deferred with D1 to future CO1.7.5.
3. **TransitionError 22-variant mapping table** — was over-claimed in prior bundled v1 (Codex Q-E); deferred with D1 to future CO1.7.5 spec.
4. **RejectedAttemptSummary side-channel substantiation** — was overclaimed (Codex Q-E); deferred to future CO1.7.5 spec where it's actually relevant.
5. **STATE_TRANSITION_SPEC v1.5 housekeeping issue filing** — committed to as a post-CO1.7-extra-PASS/PASS process item (§ 0.4); not gating implementation.
6. **Legacy `src/ledger.rs` retirement** — CO1.1.5 atom; CO1.7-extra leaves the legacy WAL ledger fully running.
7. **Materializer state_root computation** — CO1.8 (L5).

---

## § 5 Open questions (1 remains)

| Q | Conservative resolution proposed | Audit input requested |
|---|---|---|
| **Q1' Sequencer Debug derive completeness** (NEW; surfaced by Codex Q-C) | `#[derive(Debug)]` on Sequencer struct; if blanket-derive fails on `Arc<RwLock<dyn LedgerWriter>>` field, fall back to manual impl with `f.debug_struct("Sequencer").finish_non_exhaustive()`. | Confirm: does `finish_non_exhaustive` leak any sensitive state, or is it safe for the Kernel-via-serde-skip path? Fallback: `PhantomData<()>` placeholder Debug? |

(The original v1 Q1 — `head_commit_oid_hex` default impl — is now resolved per Q1 synthesis in § 1.2: default `None` + mandatory override + defensive test.)

---

## § 6 Audit gates (round structure)

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 (on prior bundled v1) | CHALLENGE / High | CHALLENGE / High | **CHALLENGE** | Atom rescoped via Occam scope-split (this v1) + small fixes |
| 2 (on this spec) | ⏳ pending | ⏳ pending | TBD | re-audit on CO1.7-extra v1; 1 round expected (small, focused atom) |
| 3+ if needed | … | … | … | iterate to PASS/PASS |

**Pre-implementation gate**: spec must reach PASS/PASS before any code in `src/state/sequencer.rs` D2 lines, `src/bus.rs` forwarder, `src/kernel.rs` field, or `src/bottom_white/ledger/transition_ledger.rs` trait method is written. Per CLAUDE.md "Audit Standard".

---

## § 7 Estimated scope

- **Spec rounds**: 1 expected on CO1.7-extra (small atom; scope split addresses round-1 substantive findings; only fine-grained issues likely in round-2). Round-2 budget ~$5-10.
- **Implementation scope** (post-PASS/PASS):
  - D2 (head_t close + trait method + 2 impl overrides): ~30-50 LoC
  - D3 (TuringBus forwarder + Kernel field + serde-skip + Sequencer Debug derive): ~40-60 LoC across 2 STEP_B-coupled files
  - D4 (2 tests): ~80-120 LoC
- **Total atom budget**: ~150-230 LoC. **Estimated calendar time**: 1-2 days. Implementation may ship as one commit `CO1.7-extra A1+A2+A3` or 3 sequential.

---

## § 8 Honest acknowledgements (v1)

1. **Scope split is round-1-driven**, not voluntary. Prior bundled CO1.7.5 v1 spec was found by Codex Q-D/H/I to have heavyweight cross-layer substrate dependencies in D1. This v1 reverts CO1.7.5 to its CO1.7 § 13 original meaning (transition bodies; future) and creates CO1.7-extra (this atom) as a new bridge for the substrate-independent wiring.
2. **`head_commit_oid_hex` is a NEW trait method** with mandatory-override design (Q1 synthesis: default `None` + every impl overrides + defensive test).
3. **TuringBus is the actual struct name**; prior bundled v1 wrote "Bus" throughout (Codex Q-C catch). Fixed in § 2.1.
4. **Kernel needs `serde(skip)` on the new Sequencer field** because Sequencer holds Arc-locked runtime state that isn't serializable Q_t data (Codex Q-C).
5. **Combined STEP_B ceremony argument now rests on functional coupling** (each half is a compile-or-no-op-error without the other), not on `STEP_B_PROTOCOL.md` Phase 0 "minimum sufficient version" binding (which Codex Q-C correctly noted is advisory).
6. **STATE_TRANSITION_SPEC v1.5 housekeeping issue filing is committed** as part of CO1.7-extra atom closure (§ 0.4), per Gemini MF1 active-reconciliation requirement.
7. **Most of CO1.1.4-pre1 ABI lock is irrelevant to this atom** — D1 (the part that uses TypedTx + TransitionError + SignalKind) is out of scope. CO1.7-extra only touches `LedgerWriter` trait + Sequencer wiring; ABI lock untouched.
8. **FC-trace requirements**: the new pub symbols introduced by CO1.7-extra implementation must carry doc-comment `/// TRACE_MATRIX <FC-id>: <role>` backlinks per CLAUDE.md "Alignment Standard". Set: `LedgerWriter::head_commit_oid_hex` (→ § 5 L4 sequencer post-commit head_t wiring); `Kernel.sequencer` field + `Kernel::with_sequencer` + `TuringBus::submit_typed_tx` (→ § 5.2.1 single-writer entry-point).

---

## § 9 Pre-audit smoke test plan

Per memory `feedback_smoke_before_batch`. Smoke run before round-2 audit launch, at the v1.1 commit HEAD.

| # | Claim | Smoke command | Pass criterion |
|---|---|---|---|
| S1 | `Git2LedgerWriter::head_commit_oid()` returns `Option<git2::Oid>` | `grep -A1 'pub fn head_commit_oid' src/bottom_white/ledger/transition_ledger.rs` | matches signature (line 674) |
| S2 | Bus struct is named `TuringBus` | `grep -n 'pub struct TuringBus' src/bus.rs` | one hit at line 53 |
| S3 | Kernel derives `Debug, Serialize, Deserialize` | `grep -B1 'pub struct Kernel' src/kernel.rs` | derives present at line 18 |
| S4 | Sequencer struct exists | `grep -n 'pub struct Sequencer' src/state/sequencer.rs` | one hit |
| S5 | CasStore exposes `put` + `get` (CO1.4 + CO1.4-extra) | `grep -n 'pub fn put\|pub fn get' src/bottom_white/cas/store.rs` | both present |
| S6 | Wallet (`src/sdk/tools/wallet.rs`) untouched | `grep -c 'transition_ledger\|state::sequencer\|TypedTx' src/sdk/tools/wallet.rs` | 0 hits |
| S7 | QState.head_t is `state::q_state::NodeId` (tuple struct) | `grep -B1 -A1 'pub head_t' src/state/q_state.rs` | type matches |
| S8 | cargo baseline | `cargo check --workspace && cargo test --workspace --lib` | clean compile + 239 / 0 / 1 ignored |

---

**END v1 DRAFT body.**

## Pre-audit smoke results (footer; populated 2026-04-29 pre-round-2)

Smoke to be run at the v1 commit HEAD post-rename + content rewrite. Footer populated when round-2 audit launches.

| # | Status |
|---|---|
| S1 | ⏳ pending audit-launch smoke |
| S2 | ⏳ |
| S3 | ⏳ |
| S4 | ⏳ |
| S5 | ⏳ |
| S6 | ⏳ |
| S7 | ⏳ |
| S8 | ⏳ |

### Patch log (this session)

**Scope rewrite (round-1 driven; this v1)**:
- Q-D/H/I from Codex → prior bundled CO1.7.5 v1 was mis-scoped; D1 has cross-layer substrate dependencies. This atom rescoped to D2 + D3 + 1 substrate-independent D4 test only. D1 + 3 D4 tests + un-ignore migrated to future CO1.7.5 atom (gated on CO P2.x substrate).

**Round-1 fixes baked into this v1**:
- M3a (Codex Q-C): `Bus` → `TuringBus` everywhere (§ 2.1)
- M3b (Codex Q-C): Kernel field gets `#[serde(skip)]`; Sequencer struct gets `#[derive(Debug)]` (§ 2.1)
- M3c (Codex Q-C): Sequencer placement in Kernel justified by parallel to existing Tape/NodeId topology pattern + planned kernel.rs doc patch (§ 2.2)
- M4 (Gemini MF1+MF3 + Codex Q-A): § 0.4 commits to filing STATE_TRANSITION_SPEC v1.5 housekeeping issue + asserts downstream-supersession authority principle
- M5 (Gemini Q8 vs Codex Q-B synthesis): Q1 closed via default `None` + mandatory override + defensive `git2_writer_returns_some_after_commit` test (§ 1.2 + § 3.2)
- Combined-ceremony argument rebased onto functional coupling (Codex Q-C correction; § 2.3)

### Awaiting

1. round-2 dual external audit on CO1.7-extra v1
2. iterate to PASS/PASS (1 round expected; small focused atom)
3. then CO1.7-extra-impl (D2 + D3 STEP_B + 2 tests)
4. file STATE_TRANSITION_SPEC v1.5 housekeeping issue per § 0.4 commitment
5. spec future CO1.7.5 (transition bodies; gated on CO P2.x substrate atoms)
