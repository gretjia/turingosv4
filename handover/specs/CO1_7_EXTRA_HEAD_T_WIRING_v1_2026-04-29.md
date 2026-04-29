# CO1.7-extra: L4 head_t close + Sequencer entry-point wiring v1.1 (post round-2 audit patches)

**Status**: v1.1 DRAFT (2026-04-29; post round-2 dual external audit on v1 at HEAD `617f01e`). Round-2 returned CHALLENGE/CHALLENGE; v1.1 applies 10 patches (MF1-MF10 per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md`). Awaiting round-3 dual external audit.
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
2. **D3** — Single-file STEP_B ceremony adds `Option<Arc<Sequencer>>` field + `with_sequencer` constructor + `submit_typed_tx` forwarder method to `TuringBus` (note: type is `TuringBus`, not `Bus`, per `src/bus.rs:53`). Sequencer lives in TuringBus directly (not nested through Kernel) per round-2 MF4 — Kernel preserves "pure topology" doctrine and stays UNTOUCHED by this atom.
3. **D4-substrate-independent** — One conformance test `tests/cas_payload_round_trip` (`CasStore::put` → `get` round-trip with CID stability post-CO1.4-extra). Other 3 D4 tests (replay state-root + system-signature canonical-message + un-ignore byte-identity) move to future CO1.7.5 atom because they require D1 transition bodies to actually commit.

### 0.4 Process commitment (active reconciliation per round-1 Gemini MF1+MF3 + Codex Q-A; round-2 MF1 corrected)

Two STATE_TRANSITION_SPEC § 3 supersessions previously declared in the prior bundled CO1.7.5 v1 spec divide differently across the new atom split (round-2 MF1 corrected the v1 wording):

| Supersession | Authority chain | Disposition |
|---|---|---|
| **head_t = NodeId(commit_oid_hex) NOT NodeId::from_state_root** (CO1.7 K3 v1.2 round-3 PASS/PASS supersedes STATE v1.4 § 3 line 412) | CO1.7 v1.2 § 5 K3 | **Enacted in CO1.7-extra D2** (§ 1.1 below); takes effect at first apply_one commit |
| **SignalBundle 4-variant SignalKind suffices** (CO1.1.4-pre1 supersedes STATE v1.4 § 3 BoolSignal/StatSignal richness) | CO1.1.4-pre1 § 7.2 | **Migrates to future CO1.7.5** (transition bodies); takes effect when D1 ships |

**Asserted authority principle** (strengthened per round-1 Gemini MF3): a later, more specific, audited spec (CO1.7 v1.2 round-3 PASS/PASS; CO1.1.4-pre1 PASS/PASS) **legitimately supersedes** earlier general specs (STATE v1.4 round-4 PASS/PASS) within the layered boundary the later spec covers. This is consistent with the project's atom-decomposition pattern: each atom locks its own surface; downstream atoms refine via PASS/PASS audit, not by editing upstream artifacts.

**Institutional debt acknowledged** (per round-1 Gemini MF1): as part of CO1.7-extra atom closure, ArchitectAI commits to filing a STATE_TRANSITION_SPEC v1.5 housekeeping issue (one paragraph noting both supersessions with backlinks) — NOT a re-audit, just an annotation pass that prevents future readers from being confused by the historical drafting language. Tracked in the § 9 awaiting list.

---

## § 1 D2 — head_t close

### 1.1 Code change

The D2 logic is extracted into a small helper `advance_head_t(q, writer)` callable from `apply_one` stage 9 AND directly testable by the new `tests/co1_7_extra_head_t_advancement.rs` integration test (round-2 MF2 closure). Helper extraction adds zero behavior change — `apply_one` still executes identical logic.

```rust
// src/state/sequencer.rs (NEW pub(crate) helper)
/// Closes G-1 head_t carry-forward (Art 0.4 alignment per CO1.7 K3 v1.2).
/// Best-effort head binding: when writer surfaces a commit OID (Git2LedgerWriter
/// always; future writers may), advance head_t. When writer returns None
/// (InMemoryLedgerWriter), leave head_t unchanged (no-op preservation).
///
/// Called from apply_one stage 9 AFTER writer.commit succeeds. Pure function
/// (writer is &dyn so behavior depends only on writer's head_commit_oid_hex
/// return + q's prior state).
pub(crate) fn advance_head_t(q: &mut QState, writer: &dyn LedgerWriter) {
    if let Some(commit_oid_hex) = writer.head_commit_oid_hex() {
        q.head_t = crate::state::q_state::NodeId(commit_oid_hex);
    }
}
```

```rust
// src/state/sequencer.rs::apply_one stage 9 (currently lines 362-373; v1.1 patch)
let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
let mut writer_w = self.ledger_writer.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
writer_w.commit(&entry)?;
self.next_logical_t.store(logical_t, Ordering::SeqCst);
*q_w = q_next;
q_w.ledger_root_t = entry.resulting_ledger_root;
// NEW (CO1.7-extra D2): close G-1 head_t carry-forward.
advance_head_t(&mut *q_w, &**writer_w);
```

**Stale comments must be updated** (round-2 MF8 — Codex Q-8 finding): `src/state/sequencer.rs:180-184` + `:359-361` currently say "head_t mutation deferred to CO1.7.5+". CO1.7-extra implementation MUST update these comments to reflect "head_t closed by CO1.7-extra D2 via `advance_head_t` helper". Added to § 9 atom landing checklist.

**NodeId disambiguation**: two `NodeId` types coexist — legacy `pub type NodeId = String` at `src/ledger.rs:13` (imported by TuringBus + Kernel for the legacy ledger event API) and new `pub struct NodeId(pub String)` at `src/state/q_state.rs:49`. `q.head_t` is typed as the new tuple-struct (`q_state.rs:311`); D2 constructs the new variant exclusively (legacy String alias is unused here).

**Atomicity** (per Codex Q-B + round-2 MF9 wording correction): under acquired `q_w` + `writer_w` write locks, after `writer_w.commit(&entry)?` returns `Ok`, the remaining operations are an `AtomicU64::store` (infallible), a plain `*q_w = q_next` move (infallible), and `advance_head_t` (infallible). For writers whose `head_commit_oid_hex` returns `Some` (Git2LedgerWriter), this is a **post-commit non-failing best-effort head binding** — `q.head_t` advances atomically with `ledger_root_t` and `next_logical_t`. For writers returning `None` (InMemoryLedgerWriter), `advance_head_t` is **explicit no-op preservation** — `q.head_t` stays at its prior value (which equals `q_next.head_t` after the `*q_w = q_next` move because CO1.7 K3 v1.2 forbids transition bodies from mutating head_t).

### 1.2 Trait method addition (round-2 MF3: REQUIRED, no default impl)

`LedgerWriter` trait at `src/bottom_white/ledger/transition_ledger.rs` gains one **required** method (round-2 audits both converged on third option per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md` MF3):

```rust
pub trait LedgerWriter: Send + Sync {
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
    fn len(&self) -> u64;
    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;  // (existing; spec preserves)

    /// NEW (CO1.7-extra D2): canonical 40-char lowercase hex commit OID of the
    /// most recent appended entry, or None if the chain is empty / backend has
    /// no commit-OID notion.
    ///
    /// **REQUIRED** (no default impl): Rust compiler enforces every
    /// LedgerWriter implementation declares this method. This is the round-2
    /// MF3 closure — both audits' safety arguments satisfied:
    /// - **silent stagnation prevention** (Gemini r1+r2): impossible to inherit
    ///   a default that silently leaves head_t stale; a missing impl is a
    ///   compile error.
    /// - **post-commit no-panic** (Codex r1): impl is free to return None at
    ///   runtime if the backend has no OID notion; no panic risk.
    fn head_commit_oid_hex(&self) -> Option<String>;
}

impl LedgerWriter for Git2LedgerWriter {
    fn head_commit_oid_hex(&self) -> Option<String> {
        self.head_commit_oid().map(|oid| oid.to_string())
    }
    // ... existing commit / len / read_at ...
}

impl LedgerWriter for InMemoryLedgerWriter {
    /// InMemory has no git substrate → always None. Required by the trait
    /// (no default to inherit) so the choice is explicit, not implicit.
    fn head_commit_oid_hex(&self) -> Option<String> {
        None
    }
    // ... existing ...
}
```

This is a **breaking change** to any third-party `LedgerWriter` impl outside the workspace (would no longer compile). Inside the workspace, only Git2LedgerWriter and InMemoryLedgerWriter implement the trait; both get explicit declarations above. Forward-compat: any future LedgerWriter impl is forced to declare its OID semantics explicitly — a desirable property for a constitutional anchor field.

---

## § 2 D3 — Single-file STEP_B ceremony for TuringBus Sequencer entry-point

### 2.1 Code change (round-2 MF4: Sequencer placement TuringBus, NOT Kernel)

Round-2 Codex Q-7 + Gemini Q5 converged on placing Sequencer at TuringBus directly (not nested through Kernel). Rationale per round-2 MF4:
- TuringBus already owns runtime orchestration (`src/bus.rs:53` + per CO1.7-impl). Sequencer is a runtime-orchestration peer of Kernel, not nested inside it.
- Kernel `src/kernel.rs:5-6` has explicit warning against domain-specific terms; the documented "pure topology" role (`:15-17`) is preserved by NOT adding state-driver fields.
- STEP_B Phase 0 less-invasive-alternative test: TuringBus-only is strictly simpler than TuringBus + Kernel coupled changes.

`src/bus.rs` (note: actual struct name is **`TuringBus`** at `src/bus.rs:53`, NOT `Bus`):

```rust
// src/bus.rs (additive — TuringBus gets one field + one constructor variant + one method)
pub struct TuringBus {
    // ... existing fields including kernel: Kernel ...

    /// NEW (CO1.7-extra D3): typed-tx Sequencer; None when bus runs in legacy
    /// ledger-only mode (preserves back-compat with all existing tests).
    /// Marked serde-skip if TuringBus has serde derives (Sequencer holds
    /// Arc-locked runtime state that isn't serializable Q_t data).
    #[serde(skip)]  // applied if TuringBus has Serialize/Deserialize
    pub sequencer: Option<Arc<Sequencer>>,
}

impl TuringBus {
    pub fn new(kernel: Kernel, config: BusConfig) -> Self {
        Self { /* ...existing..., */ sequencer: None }
    }

    /// NEW: opt-in constructor that wires a typed-tx Sequencer alongside the legacy ledger.
    pub fn with_sequencer(kernel: Kernel, config: BusConfig, sequencer: Arc<Sequencer>) -> Self {
        Self { /* ...existing..., */ sequencer: Some(sequencer) }
    }

    /// NEW (CO1.7-extra D3): typed-tx submission path. Returns receipt
    /// (submit_id) immediately; commit happens asynchronously in
    /// Sequencer::run driver loop.
    pub async fn submit_typed_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
        match self.sequencer.as_ref() {
            Some(seq) => seq.submit(tx).await,
            None => Err(SubmitError::QueueClosed),
        }
    }
}
```

`src/kernel.rs`: **UNTOUCHED** by CO1.7-extra. "Pure topology" doctrine preserved.

`src/state/sequencer.rs` (round-2 MF6: manual Debug impl, NOT derive — Sequencer holds `Arc<Ed25519Keypair>` at line 199 and `Ed25519Keypair` intentionally has no Debug derive at `src/bottom_white/ledger/system_keypair.rs:282-284`; blanket derive fails to compile):

```rust
// src/state/sequencer.rs (additive — manual Debug impl for TuringBus.Debug propagation through Arc<Sequencer>)
impl std::fmt::Debug for Sequencer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // finish_non_exhaustive() — leaks no keypair / QState / CAS contents;
        // satisfies Debug requirements for Arc<Sequencer> propagation.
        f.debug_struct("Sequencer").finish_non_exhaustive()
    }
}
```

### 2.2 Single-file STEP_B ceremony (round-2 MF4 simplification)

CO1.7-extra now touches a single STEP_B-restricted file: `src/bus.rs`. No combined-ceremony justification needed. Per `STEP_B_PROTOCOL.md` Phase 0, the change is "minimum sufficient version" and has no less-invasive alternative (the typed-tx submission path needs SOME entry-point in the runtime layer; TuringBus is the canonical orchestrator).

**Ceremony procedure**:
1. Branch A (`step-b-co1.7-extra-A`): edits `src/bus.rs` per § 2.1 (1 field + 1 constructor variant + 1 forwarder method). Also adds the manual `Debug` impl on `Sequencer` in `src/state/sequencer.rs` (NOT STEP_B-restricted; lands alongside for compile coherence).
2. Branch B (`step-b-co1.7-extra-B`): independently re-derives the same edits from this spec (separate session / context).
3. Byte-identity comparison: `diff src/bus.rs` between A and B. Identical → merge to `main`. Divergent → re-do with stricter spec.

### 2.3 Forward-compat note (round-2 Gemini Q5 partial response)

Gemini Q5 r2 noted "Kernel placement creates forward-compat hazard of Kernel bloat". The TuringBus placement avoids this hazard entirely — Kernel stays at "pure topology" role; future stateful runtime drivers (e.g., a hypothetical CO1.x event router) would land at TuringBus level alongside Sequencer, which is the natural runtime-orchestrator role for TuringBus to own. No further justification needed beyond Codex Q-7 + Gemini Q5 convergence.

---

## § 3 Test plan (substrate-independent; round-2 MF2 + MF5 + MF7 patches)

Three tests, **flat-named in `tests/`** (round-2 MF5 — Cargo auto-discovery requires flat naming or a `tests/co1_7_extra/main.rs` harness; v1.1 chooses flat naming for simplicity):

### 3.1 `tests/co1_7_extra_cas_payload_round_trip.rs`

```rust
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

### 3.2 `tests/co1_7_extra_git2_writer_head_oid_defense.rs`

Round-2 MF7: `canonical_test_entry` is private to module tests at `transition_ledger.rs:813`; integration tests must construct LedgerEntry inline.

```rust
#[test]
fn git2_writer_returns_some_after_commit() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let mut writer = Git2LedgerWriter::open(tmp.path()).expect("open");

    // Inline LedgerEntry construction (round-2 MF7) — uses public CO1.7-impl
    // surfaces only.
    let entry = LedgerEntry {
        logical_t: 1,
        parent_state_root: Hash::ZERO,
        parent_ledger_root: Hash::ZERO,
        tx_kind: TxKind::Work,
        tx_payload_cid: Cid([0u8; 32]),
        resulting_state_root: Hash::ZERO,
        resulting_ledger_root: Hash([1u8; 32]),
        timestamp_logical: 1,
        epoch: SystemEpoch::new(1),
        extensions: Default::default(),
        system_signature: SystemSignature::from_bytes([0u8; 64]),
    };

    writer.commit(&entry).expect("commit");
    // Defensive against silent head_t stagnation: if Git2LedgerWriter ever
    // inherits a default behavior (impossible given round-2 MF3 — trait method
    // is now required), this catches it. Belt-and-suspenders for the
    // constitutional anchor.
    assert!(
        writer.head_commit_oid_hex().is_some(),
        "Git2LedgerWriter MUST return Some after commit; constitutional anchor violation otherwise"
    );
}
```

### 3.3 `tests/co1_7_extra_sequencer_head_t_advancement.rs` (NEW — round-2 MF2 closure)

Tests the actual D2 code path via the `advance_head_t` helper extraction:

```rust
//! CO1.7-extra D2: verifies advance_head_t correctly advances q.head_t
//! when writer surfaces a commit OID, and preserves q.head_t when writer
//! returns None. Substrate-independent: uses only LedgerWriter trait + QState.
//! Closes round-2 MF2 (D2 code path was untested in v1).

use std::sync::Mutex;

/// Mock LedgerWriter that returns a configurable head_commit_oid_hex value.
/// Stubs commit() to always succeed (returns dummy Hash).
struct MockLedgerWriter {
    head_oid: Mutex<Option<String>>,
    len: u64,
}

impl LedgerWriter for MockLedgerWriter {
    fn commit(&mut self, _entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
        self.len += 1;
        Ok(Hash([0xAB; 32]))
    }
    fn len(&self) -> u64 { self.len }
    fn read_at(&self, _: u64) -> Result<LedgerEntry, LedgerWriterError> {
        unimplemented!("test mock")
    }
    fn head_commit_oid_hex(&self) -> Option<String> {
        self.head_oid.lock().expect("lock").clone()
    }
}

#[test]
fn advance_head_t_writes_node_id_when_writer_returns_some() {
    let writer = MockLedgerWriter {
        head_oid: Mutex::new(Some("a".repeat(40))),  // 40-hex literal
        len: 0,
    };
    let mut q = QState::genesis();
    let q_initial_head = q.head_t.clone();

    turingosv4::state::sequencer::advance_head_t(&mut q, &writer);

    // Post-condition: q.head_t = NodeId("aaaa...aaaa")
    assert_eq!(q.head_t.0, "a".repeat(40));
    assert_ne!(q.head_t, q_initial_head);
}

#[test]
fn advance_head_t_preserves_node_id_when_writer_returns_none() {
    let writer = MockLedgerWriter {
        head_oid: Mutex::new(None),
        len: 0,
    };
    let mut q = QState::genesis();
    let q_initial_head = q.head_t.clone();

    turingosv4::state::sequencer::advance_head_t(&mut q, &writer);

    // Post-condition: q.head_t unchanged (no-op preservation per § 1.1).
    assert_eq!(q.head_t, q_initial_head);
}
```

Total: 3 tests across 3 flat-named integration test files.

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

## § 5 Open questions (0 remain — all closed by round-2 audits)

| Q | Round-2 resolution |
|---|---|
| Q1 `head_commit_oid_hex` default impl (round-1 open) | **Closed by round-2 MF3** — trait method is REQUIRED (no default); compiler enforces every impl declares (§ 1.2). Both audits' safety arguments satisfied. |
| Q1' Sequencer Debug derive completeness (round-1 surfaced) | **Closed by round-2 MF6** — manual `impl Debug for Sequencer` with `f.debug_struct("Sequencer").finish_non_exhaustive()`; `#[derive(Debug)]` not viable because `Arc<Ed25519Keypair>` field has no Debug derive. Codex Q-5 confirms `finish_non_exhaustive()` leaks no keypair / QState / CAS contents. |

CO1.7-extra v1.1 has zero open questions — round-3 audit verifies patch correctness only.

---

## § 6 Audit gates (round structure)

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 (on prior bundled v1) | CHALLENGE / High | CHALLENGE / High | **CHALLENGE** | Atom rescoped via Occam scope-split (this v1) + small fixes |
| 2 (on this spec) | ⏳ pending | ⏳ pending | TBD | re-audit on CO1.7-extra v1; 1 round expected (small, focused atom) |
| 3+ if needed | … | … | … | iterate to PASS/PASS |

**Pre-implementation gate**: spec must reach PASS/PASS before any code in `src/state/sequencer.rs` D2 lines, `src/bus.rs` forwarder, `src/kernel.rs` field, or `src/bottom_white/ledger/transition_ledger.rs` trait method is written. Per CLAUDE.md "Audit Standard".

---

## § 7 Estimated scope (round-2 MF10: revised upward)

- **Spec rounds**: round-2 done (CHALLENGE/CHALLENGE → 10 patches in v1.1); round-3 expected to PASS/PASS (small atom, all r2 issues addressed systematically). Round-3 budget ~$5-10.
- **Implementation scope** (post-PASS/PASS):
  - D2 (head_t close: `advance_head_t` helper + apply_one stage 9 patch + required trait method + 2 impl declarations + stale-comment updates at sequencer.rs:180-184/:359-361): ~40-60 LoC
  - D3 (TuringBus field + with_sequencer constructor + submit_typed_tx forwarder + manual Sequencer Debug impl): ~50-80 LoC across single STEP_B-restricted file (bus.rs) + 1 supporting file (sequencer.rs Debug impl)
  - D4 (3 tests with mock LedgerWriter + inline LedgerEntry fixture): ~120-160 LoC
- **Total atom budget**: ~210-300 LoC (revised up from v1's 150-230 per round-2 MF10 — manual Debug + helper extraction + 3rd test + harness adjustments). **Estimated calendar time**: 1-2 days.

---

## § 8 Honest acknowledgements (v1.1)

1. **Scope split is round-1-driven**, not voluntary. Prior bundled CO1.7.5 v1 spec was found by Codex r1 Q-D/H/I to have heavyweight cross-layer substrate dependencies in D1. v1 reverts CO1.7.5 to its CO1.7 § 13 original meaning (transition bodies; future) and creates CO1.7-extra (this atom) as a new bridge for the substrate-independent wiring.
2. **`head_commit_oid_hex` is a NEW REQUIRED trait method** (no default impl; round-2 MF3). Compiler enforces every LedgerWriter impl declares; both `Git2LedgerWriter` and `InMemoryLedgerWriter` get explicit declarations in § 1.2.
3. **D2 logic is extracted into `advance_head_t` helper** (round-2 MF2 closure). The extraction adds zero behavior change but makes D2 directly testable via mock writer (without injecting dispatch_transition into Sequencer).
4. **TuringBus owns Sequencer directly** (round-2 MF4) — not nested through Kernel. Kernel preserves "pure topology" doctrine (`src/kernel.rs:5-6`+`:15-17`) and stays UNTOUCHED by this atom. STEP_B becomes single-file ceremony on `src/bus.rs`.
5. **Manual Sequencer Debug impl** (round-2 MF6) — `#[derive(Debug)]` fails because `Arc<Ed25519Keypair>` field has no Debug (system_keypair.rs:282-284 intentional); `finish_non_exhaustive()` is the safe replacement (Codex Q-5 confirmed no leak risk).
6. **STATE_TRANSITION_SPEC v1.5 housekeeping issue filing is committed** (§ 0.4) per round-1 Gemini MF1 active-reconciliation requirement. Round-2 confirmed the directionally correct framing; v1.1 corrected the supersession-disposition table (head_t enacted HERE; SignalKind migrates to future CO1.7.5).
7. **Most of CO1.1.4-pre1 ABI lock is irrelevant to this atom** — D1 (the part that uses TypedTx + TransitionError + SignalKind) is out of scope. CO1.7-extra only touches `LedgerWriter` trait + TuringBus wiring + Sequencer Debug impl; ABI lock untouched.
8. **Stale Sequencer comments will be updated** during implementation (round-2 MF8): `src/state/sequencer.rs:180-184` + `:359-361` currently say "head_t deferred to CO1.7.5+"; CO1.7-extra implementation must update them to reflect the new D2 reality.
9. **FC-trace requirements**: the new pub symbols introduced by CO1.7-extra implementation must carry doc-comment `/// TRACE_MATRIX <FC-id>: <role>` backlinks per CLAUDE.md "Alignment Standard". Set: `LedgerWriter::head_commit_oid_hex` + `advance_head_t` helper (→ § 5 L4 sequencer post-commit head_t wiring); `TuringBus.sequencer` field + `TuringBus::with_sequencer` + `TuringBus::submit_typed_tx` (→ § 5.2.1 single-writer entry-point).

---

## § 9 Pre-audit smoke test plan (v1.1; round-3 launch)

Per memory `feedback_smoke_before_batch`. Smoke run before round-3 audit launch, at the v1.1 commit HEAD.

| # | Claim | Smoke command | Pass criterion |
|---|---|---|---|
| S1 | `Git2LedgerWriter::head_commit_oid()` returns `Option<git2::Oid>` | `grep -A1 'pub fn head_commit_oid' src/bottom_white/ledger/transition_ledger.rs` | matches signature (line 674) |
| S2 | Bus struct is named `TuringBus` | `grep -n 'pub struct TuringBus' src/bus.rs` | one hit at line 53 |
| S3 | Kernel UNTOUCHED by this atom (round-2 MF4) | `grep -n 'use crate::ledger::' src/kernel.rs && grep -L 'sequencer' src/kernel.rs` | legacy ledger import present; no "sequencer" reference (Kernel stays at pure topology) |
| S4 | Sequencer struct exists at sequencer.rs:190 | `grep -n 'pub struct Sequencer' src/state/sequencer.rs` | one hit at line 190 |
| S5 | Ed25519Keypair has no Debug derive (forces manual Sequencer Debug impl per MF6) | `grep -B5 'pub struct Ed25519Keypair' src/bottom_white/ledger/system_keypair.rs` | no `#[derive(Debug` precedes struct line |
| S6 | CasStore exposes `put` + `get` (CO1.4 + CO1.4-extra) | `grep -n 'pub fn put\|pub fn get' src/bottom_white/cas/store.rs` | both present |
| S7 | Wallet (`src/sdk/tools/wallet.rs`) untouched | `grep -c 'transition_ledger\|state::sequencer\|TypedTx' src/sdk/tools/wallet.rs` | 0 hits |
| S8 | QState.head_t is `state::q_state::NodeId` (tuple struct) | `grep -B1 -A1 'pub head_t' src/state/q_state.rs` | type matches |
| S9 | Stale comment locations (round-2 MF8) | `grep -n 'CO1.7.5+\|deferred to CO1.7.5' src/state/sequencer.rs` | hits at lines 180-184 + 359-361 (to be updated by D2 implementation) |
| S10 | `canonical_test_entry` is private (round-2 MF7) | `grep -n 'fn canonical_test_entry' src/bottom_white/ledger/transition_ledger.rs` | hit inside `mod tests` (line ~813); no `pub` |
| S11 | cargo baseline | `cargo check --workspace && cargo test --workspace --lib` | clean compile + 239 / 0 / 1 ignored |

---

**END v1 DRAFT body.**

## Pre-audit smoke results

### Round-2 smoke (HEAD `617f01e`; v1)

8/8 PASS — see prior commit log. v1 spec sent to round-2 dual external audit on this baseline.

### Round-3 smoke (v1.1 HEAD; populated at audit launch)

| # | Status |
|---|---|
| S1-S11 | ⏳ pending audit-launch smoke at v1.1 commit HEAD |

### Patch log

**v1 (round-1 scope split; commits `f7fc19f` + `617f01e`)**:
- Scope split per round-1 Codex r1 Q-D/H/I + ArchitectAI Occam decision. D1 + 3 D4 tests + un-ignore migrated to future CO1.7.5 atom (gated on CO P2.x). v1 inherited round-1 fixes M3-M5 + § 0.4 active reconciliation.

**v1.1 (round-2 driven; this revision)** — 10 patches per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md`:

- **MF1** § 0.4 supersession-disposition table corrected: head_t supersession **enacted in CO1.7-extra D2**; SignalKind supersession migrates to future CO1.7.5 (was: both migrate)
- **MF2** D2 testability: `advance_head_t(q, writer)` helper extracted from apply_one stage 9 → makes D2 directly testable via mock writer (§ 1.1 + new test § 3.3)
- **MF3** trait method `head_commit_oid_hex` becomes REQUIRED (no default impl); Rust compiler enforces every LedgerWriter declares (§ 1.2). Both audits' safety arguments satisfied via this third-option synthesis.
- **MF4** Sequencer placement: TuringBus owns directly (NOT nested through Kernel). Kernel UNTOUCHED. STEP_B becomes single-file ceremony (§ 2.1 + § 2.2).
- **MF5** test harness: flat-named `tests/co1_7_extra_*.rs` for Cargo auto-discovery (§ 3 file paths)
- **MF6** Sequencer Debug: manual `impl Debug` with `finish_non_exhaustive()` (Ed25519Keypair has no Debug derive at system_keypair.rs:282-284 — blanket derive fails) (§ 2.1)
- **MF7** `canonical_test_entry` private → tests construct LedgerEntry inline (§ 3.2)
- **MF8** stale Sequencer comments (sequencer.rs:180-184 + :359-361) added to atom landing checklist (§ 1.1 + § 8 ack #8)
- **MF9** atomicity wording: "post-commit non-failing best-effort head binding (Some path)" + "explicit no-op preservation (None path)" (§ 1.1)
- **MF10** LoC estimate: 150-230 → 200-280 (manual Debug + helper extraction + 3rd test + harness adjustments) (§ 7)

### Awaiting

1. round-3 dual external audit on CO1.7-extra v1.1
2. expected PASS/PASS (small atom, all r2 issues addressed systematically)
3. then CO1.7-extra-impl (D2 helper extraction + apply_one patch + trait method + TuringBus single-file STEP_B + 3 tests + stale-comment update)
4. file STATE_TRANSITION_SPEC v1.5 housekeeping issue per § 0.4 commitment
5. spec future CO1.7.5 (transition bodies; gated on CO P2.x substrate atoms)
