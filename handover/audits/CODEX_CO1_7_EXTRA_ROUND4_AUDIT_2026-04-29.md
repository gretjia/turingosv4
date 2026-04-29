# Codex CO1.7-extra Round-4 Audit
**Date**: 2026-04-29
**Target**: spec v1.2 (post round-3 mechanical fixes)
**HEAD**: 13bfb7ecc0a8df49478def9a984789038563385e
**Prompt size**: 106888 chars

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
session id: 019dd752-7242-72c1-b45f-4f01e1b11f55
--------
user
# Codex Adversarial Audit — CO1.7-extra v1.2 (Round 4; post round-3 mechanical fixes)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-4 (parallel).

**Mandate**: round 4 dual external audit on CO1.7-extra v1.2 — applied 4 mechanical patches (B1-B4) per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md`. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

**Round-3 disposition**: Codex r3 CHALLENGE/High (3 blockers + 1 non-blocking); Gemini r3 PASS/High ("model of post-audit closure"). Conservative-merged CHALLENGE; v1.2 fixes applied.

**Round-4 expectation**: PASS unless v1.2 introduced new defects or missed a closure. Patches are mechanical (1-line code/text fixes); no architectural surface change since v1.1.

## What changed since round-3 (v1.1 → v1.2)

- **B1**: § 1.1 stage-9 snippet `&**writer_w` → `&*writer_w` (compile error fix; `RwLockWriteGuard<dyn LedgerWriter>` cannot double-deref)
- **B2**: § 1.1 helper `pub(crate) fn advance_head_t` → `pub fn advance_head_t` (integration tests need `pub`); FC-trace doc-comment added
- **B3**: removed 2 stale Kernel references — preface line 14 single-sentence summary + § 6 pre-implementation gate file list
- **B4** (non-blocking): § 2.1 `#[serde(skip)]` made conditional with explicit comment; § 7 LoC vs patch log synced (200-280 → 210-300)

## Round 4 audit questions (tight)

**Q1. B1 closure**: spec § 1.1 stage-9 snippet uses `&*writer_w` (single deref). Verify:
- Compiles given `writer_w: RwLockWriteGuard<dyn LedgerWriter>` (per src/state/sequencer.rs:201, :363-368)
- Produces `&dyn LedgerWriter` matching `advance_head_t` signature `&dyn LedgerWriter`
- No residual `&**writer_w` anywhere in spec

**Q2. B2 closure**: spec § 1.1 declares `pub fn advance_head_t` (was `pub(crate)`). Verify:
- Integration test at § 3.3 (`turingosv4::state::sequencer::advance_head_t`) can now access it
- Doc-comment includes FC-trace `/// TRACE_MATRIX § 5 — L4 sequencer post-commit head_t wiring (Art 0.4)` per CLAUDE.md "Alignment Standard"
- The `pub` keyword change is the ONLY visibility change (no over-exposure)

**Q3. B3 closure**: stale Kernel references removed. Verify:
- Preface line 14 single-sentence summary now says "single-file STEP_B ceremony adding a Sequencer entry-point on TuringBus (Kernel UNTOUCHED)" — NO "combined" or "TuringBus + Kernel"
- § 6 pre-implementation gate file list does NOT include `src/kernel.rs`; explicitly states "Kernel UNTOUCHED"
- Any other residual Kernel references in v1.2 that should be removed?

**Q4. B4 closure**: non-blocking inconsistencies fixed. Verify:
- § 2.1 `#[serde(skip)]` now conditional with explicit comment about TuringBus serde derive state
- § 7 LoC says `~210-300`; patch log says `~210-300` (synced). Or is there still divergence?

**Q5. New defects in v1.2**: any new issues introduced by these 4 patches?
- Internal contradictions between sections?
- New compile blockers in code snippets?
- Stale references to round-3 / B-numbered items that should be cleaned up?

## Output format

# Codex CO1.7-extra Round-4 Audit
## Q1 B1 closure (stage-9 deref)
## Q2 B2 closure (advance_head_t pub)
## Q3 B3 closure (stale Kernel refs)
## Q4 B4 closure (serde-skip conditional + LoC sync)
## Q5 New defects in v1.2
## **VERDICT**: PASS / CHALLENGE / VETO
## Top issues (if CHALLENGE)
## Conviction (low/med/high)

Be rigorous. Cite spec line numbers when calling defects. Do NOT pass on principle; do NOT veto on principle.

---


# CO1.7-extra v1.2 (target of audit)

# CO1.7-extra: L4 head_t close + Sequencer entry-point wiring v1.2 (post round-3 audit patches)

**Status**: v1.2 DRAFT (2026-04-29; post round-3 dual external audit on v1.1 at HEAD `a3952cf`). Round-3 returned CHALLENGE/PASS (Codex/Gemini); conservative-merged CHALLENGE. v1.2 applies 4 mechanical patches (B1-B4 per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md`). Awaiting round-4 dual external audit.
**Author**: ArchitectAI (Claude); session 2026-04-29.
**Supersedes**: prior bundled `CO1_7_5_TRANSITION_BODIES_AND_RUNTIME_WIRING_v1_2026-04-29.md` (committed `334111a`; round-1 CHALLENGE/CHALLENGE; preserved in git history).
**Pre-implementation gate**: PASS/PASS dual external audit before any code lands. Per CLAUDE.md "Audit Standard".

**Companion specs (frozen, read first)**:
- `CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` v1.2 — round-3 PASS/PASS; freezes `LedgerWriter` trait + Sequencer 9-stage apply_one + `Git2LedgerWriter::head_commit_oid()`.
- `CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — frozen 7-variant TypedTx; not directly touched here.
- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — referenced for K3 v1.2 supersession authority only; transition bodies are out of scope for this atom.
- `handover/audits/CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md` — round-1 merged verdict that drove this scope split.

**Single sentence**: close the G-1 carry-forward `q.head_t = NodeId(commit_oid_hex)` after `Git2LedgerWriter.commit`, perform a single-file STEP_B ceremony adding a Sequencer entry-point on TuringBus (Kernel UNTOUCHED), and ship substrate-independent tests — leaving transition function bodies + replay byte-identity to a future CO1.7.5 atom that depends on the Wave-2 substrate (CO P2.x family).

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
///
/// TRACE_MATRIX § 5 — L4 sequencer post-commit head_t wiring (Art 0.4).
///
/// **Visibility** (round-3 B2): `pub` (NOT `pub(crate)`) so that flat
/// integration tests under `tests/co1_7_extra_*.rs` can call this helper
/// directly; needed for round-2 MF2 closure.
pub fn advance_head_t(q: &mut QState, writer: &dyn LedgerWriter) {
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
// (round-3 B1 fix: single deref of RwLockWriteGuard<dyn LedgerWriter> →
// &dyn LedgerWriter; double deref does not work on dyn.)
advance_head_t(&mut *q_w, &*writer_w);
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
    // `#[serde(skip)]` applied IFF TuringBus has Serialize/Deserialize
    // derives at implementation time. Current `pub struct TuringBus` at
    // src/bus.rs:53 has NO serde derives, so the attribute is omitted at
    // first landing. If a future atom adds serde derives to TuringBus, the
    // skip attribute MUST be added in the same patch (the Sequencer's
    // Arc-locked runtime state is not serializable Q_t data).
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

Round-2 MF7: the private module-test helper `entry_at` (at `transition_ledger.rs:813`; Codex r2 misidentified the name as `canonical_test_entry` but substantive finding holds — helper is private and unavailable to integration tests). Integration tests must construct `LedgerEntry` inline.

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

**Pre-implementation gate**: spec must reach PASS/PASS before any code in `src/state/sequencer.rs` (D2 helper + apply_one stage 9 patch), `src/bus.rs` (TuringBus field + constructor + forwarder; single-file STEP_B), or `src/bottom_white/ledger/transition_ledger.rs` (trait method + 2 impl declarations) is written. **Kernel UNTOUCHED**. Per CLAUDE.md "Audit Standard".

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
| S10 | `entry_at` (private module-test helper) is private (round-2 MF7) | `sed -n '810,820p' src/bottom_white/ledger/transition_ledger.rs` | `fn entry_at(...)` at line 813 inside `mod tests`; no `pub` qualifier |
| S11 | cargo baseline | `cargo check --workspace && cargo test --workspace --lib` | clean compile + 239 / 0 / 1 ignored |

---

**END v1 DRAFT body.**

## Pre-audit smoke results

### Round-2 smoke (HEAD `617f01e`; v1)

8/8 PASS — see prior commit log. v1 spec sent to round-2 dual external audit on this baseline.

### Round-3 smoke (HEAD `25564d7`; v1.1 patches commit)

| # | Claim | Result | Status |
|---|---|---|---|
| S1 | head_commit_oid signature | `pub fn head_commit_oid(&self) -> Option<git2::Oid>` (transition_ledger.rs:674) | ✅ PASS |
| S2 | TuringBus struct | `pub struct TuringBus` at bus.rs:53 | ✅ PASS |
| S3 | Kernel UNTOUCHED | `use crate::ledger::{Node, NodeId, Tape, TapeError}` at kernel.rs:8; **0 hits** of "sequencer" anywhere in kernel.rs (pure topology preserved per round-2 MF4) | ✅ PASS |
| S4 | Sequencer struct line | `pub struct Sequencer` at sequencer.rs:190 | ✅ PASS |
| S5 | Ed25519Keypair has NO Debug derive | `#[derive(Zeroize, ZeroizeOnDrop)]` precedes `pub struct Ed25519Keypair` (system_keypair.rs:282-284); no Debug → forces manual Sequencer Debug impl per MF6 | ✅ PASS |
| S6 | CasStore put + get | `pub fn put` at line 163, `pub fn get` at line 199 | ✅ PASS |
| S7 | wallet untouched | 0 hits in `src/sdk/tools/wallet.rs` | ✅ PASS |
| S8 | head_t type | `pub head_t: NodeId` (q_state.rs:311) — type matches new tuple-struct | ✅ PASS |
| S9 | stale comments confirmed | sequencer.rs:178-184 (doc on apply_one Sequencer) + :357-361 (in apply_one stage 9 inline comment) — both still say "deferred to CO1.7.5+"; will be patched by D2 implementation per atom landing checklist | ✅ PASS (with minor line cite refinement vs v1.1 spec's "180-184 + 359-361" — actual lines 178-184 + 357-361; spec patched in this commit) |
| S10 | private module-test helper exists | `fn entry_at` at transition_ledger.rs:813 inside `mod tests`; no `pub` qualifier (Codex r2 misidentified name as `canonical_test_entry` but substantive finding holds) | ✅ PASS (with helper-name correction) |
| S11 | cargo baseline | check pass; `239 passed; 0 failed; 1 ignored` (the ignored test is `sequencer_serial_replay_byte_identity`, deferred to future CO1.7.5 atom) | ✅ PASS |

**Smoke gate v1.1**: 11 / 11 PASS at HEAD `25564d7`. Spec v1.1 sent to round-3 dual external audit.

### Round-4 smoke (v1.2 HEAD; populated at audit launch)

| # | Status |
|---|---|
| S1-S11 | ⏳ pending (will re-run at v1.2 commit HEAD; expected unchanged from v1.1 since source code did not change between v1.1 and v1.2 — only spec text patches) |

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
- **MF7** `entry_at` private → tests construct LedgerEntry inline (§ 3.2)
- **MF8** stale Sequencer comments (sequencer.rs:178-184 + :357-361) added to atom landing checklist (§ 1.1 + § 8 ack #8)
- **MF9** atomicity wording: "post-commit non-failing best-effort head binding (Some path)" + "explicit no-op preservation (None path)" (§ 1.1)
- **MF10** LoC estimate: 150-230 → 210-300 (manual Debug + helper extraction + 3rd test + harness adjustments) (§ 7)

**v1.2 (round-3 driven; this revision)** — 4 mechanical patches per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md`:

- **B1** (Codex Q2): § 1.1 stage-9 snippet `&**writer_w` → `&*writer_w` (single deref of `RwLockWriteGuard<dyn LedgerWriter>` → `&dyn LedgerWriter`; double deref does not work on `dyn`).
- **B2** (Codex Q2): § 1.1 helper `pub(crate) fn advance_head_t` → `pub fn advance_head_t` (so flat integration tests under `tests/co1_7_extra_*.rs` per MF5 can call it). Added FC-trace doc-comment.
- **B3** (Codex Q4): removed stale Kernel references at preface line 14 (single-sentence summary now reads "single-file STEP_B ceremony adding a Sequencer entry-point on TuringBus (Kernel UNTOUCHED)") + § 6 pre-implementation gate (removed `src/kernel.rs` from gate file list; explicitly noted "Kernel UNTOUCHED").
- **B4** (Codex Q5+Q6, non-blocking): § 2.1 `#[serde(skip)]` made conditional with explicit comment ("applied IFF TuringBus has serde derives at implementation time"); § 7 vs patch log LoC sync (patch log "200-280" → "210-300" matches § 7).

**Round-3 Codex/Gemini disagreement summary**: Gemini PASS ("model of post-audit closure"; v1.1 architecturally sound). Codex CHALLENGE (3 concrete patch blockers + 1 non-blocking, all mechanical fixes). Conservative-merged CHALLENGE (per memory `feedback_dual_audit_conflict`); v1.2 patches B1-B4 mechanically; round-4 expected PASS/PASS.

### Awaiting

1. round-4 dual external audit on CO1.7-extra v1.2
2. expected PASS/PASS (only mechanical fixes need verification; no architectural surface change since v1.1)
3. then CO1.7-extra-impl (D2 helper extraction + apply_one patch + trait method + TuringBus single-file STEP_B + 3 tests + stale-comment update)
4. file STATE_TRANSITION_SPEC v1.5 housekeeping issue per § 0.4 commitment
5. spec future CO1.7.5 (transition bodies; gated on CO P2.x substrate atoms)


---

# XREF: round-3 merged verdict (drove the v1.2 patches)

# CO1.7-extra Dual External Audit — Round-3 Merged Verdict

**Date**: 2026-04-29
**Target**: `CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md` v1.1 at HEAD `a3952cf`
**Audits**: Codex r3 (`CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md`) + Gemini r3 (`GEMINI_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md`)

## Verdict matrix

| Audit | Verdict | Conviction |
|---|---|---|
| Codex | **CHALLENGE** | High (3 concrete patch blockers; "no foundational design flaw") |
| Gemini | **PASS** | High ("v1.1 spec is a model of post-audit closure") |

**Conservative-merged verdict** (per memory `feedback_dual_audit_conflict`, VETO > CHALLENGE > PASS): **CHALLENGE / High**. Codex's deeper source-level review caught 2 compile-blocking errors + 1 internal-contradiction blocker that Gemini's strategic angle did not surface.

## Round-2 must-fix closure (per round-3 review)

| R2 MF | Codex r3 status | Gemini r3 status |
|---|---|---|
| MF1 § 0.4 disposition table | ✅ PASS (factually correct) | ✅ PASS (Q5 sound) |
| MF2 advance_head_t helper | ❌ CHALLENGE (visibility + compile error) | ✅ PASS (Q4 testable + clean) |
| MF3 required trait method | ✅ PASS (compiler enforces; safety claims hold) | ✅ PASS (Q3 constitutionally sound) |
| MF4 Sequencer placement (TuringBus) | ⚠️ partial PASS — main rewrite correct; **stale Kernel references at spec line 14 + 395** | ✅ PASS (Q2 strict improvement) |
| MF5 flat test names | ✅ PASS | (covered in Q4) |
| MF6 manual Debug | ✅ PASS | (architectural verdict OK) |
| MF7 `entry_at` private | ✅ PASS | (architectural verdict OK) |
| MF8 stale comment cites | ✅ PASS (lines 180-184 + 359-361 match) | (architectural verdict OK) |
| MF9 atomicity wording | ✅ PASS | (architectural verdict OK) |
| MF10 LoC estimate | ⚠️ PARTIAL — § 7 says `210-300`, patch log says `200-280`; inconsistent | (out of scope for strategic review) |

## Round-3 new must-fix items (Codex-found)

### B1 — `&**writer_w` compile error (Codex Q2)

Spec § 1.1 stage-9 snippet:
```rust
advance_head_t(&mut *q_w, &**writer_w);
```

`writer_w` is `RwLockWriteGuard<dyn LedgerWriter>` per `src/state/sequencer.rs:201` + `:363-368`. `dyn LedgerWriter` cannot be double-dereferenced. The correct expression is `&*writer_w` (single deref turning the `RwLockWriteGuard<dyn LedgerWriter>` into `&dyn LedgerWriter`).

**Fix**: § 1.1 — `&**writer_w` → `&*writer_w`.

### B2 — `advance_head_t` visibility contradicts integration-test plan (Codex Q2)

Spec § 1.1 declares `pub(crate) fn advance_head_t(...)` but § 3.3 places the test in `tests/co1_7_extra_sequencer_head_t_advancement.rs` calling `turingosv4::state::sequencer::advance_head_t`. Integration tests live in a separate compilation unit; `pub(crate)` is not accessible from there. The MF5 flat-test choice forces this to be `pub`.

**Fix**: § 1.1 — `pub(crate) fn advance_head_t` → `pub fn advance_head_t`. Add doc-comment `/// TRACE_MATRIX § 5 — L4 sequencer post-commit head_t wiring (Art 0.4)` per FC-trace requirement.

### B3 — Stale MF4 references at spec line 14 + 395 (Codex Q4)

Spec body main § 2 correctly rewritten to TuringBus-only architecture, but two pre-existing v1 references survived the v1.1 patch round:
- **Line 14** (single-sentence summary, top of doc): still says "perform combined STEP_B ceremony adding a Sequencer entry-point on TuringBus + Kernel". MF4 made this single-file (TuringBus only); "combined" + "Kernel" are stale.
- **Line 395** (pre-implementation gate paragraph in § 6): still mentions `src/kernel.rs` field as part of the gate. MF4 removed that touch surface.

**Fix**: § preface line 14 + § 6 line 395 — remove "combined" + "TuringBus + Kernel" from line 14; remove `src/kernel.rs` reference from line 395.

### B4 — Non-blocking inconsistencies (Codex Q5 + Q6)

- **B4a `#[serde(skip)]` conditional**: § 2.1 shows `#[serde(skip)]` on the new `TuringBus.sequencer` field unconditionally, but `src/bus.rs:53` `pub struct TuringBus` has no `Serialize/Deserialize` derive currently — the literal `#[serde(skip)]` would not be invalid syntactically (serde-skip without serde just no-ops) but is misleading documentation. v1.1 spec already has a comment "applied if TuringBus has Serialize/Deserialize" (line ~167); Codex flags that this conditional should be more explicit at the code-comment level.
- **B4b LoC inconsistency**: § 7 says `~210-300 LoC`; patch log says `~200-280`. Mechanical sync.

## Where audits agreed (round-3)

- ✅ All round-2 MF1, MF3, MF5, MF6, MF7, MF8, MF9 fully closed by v1.1 patches
- ✅ Architectural changes (Sequencer → TuringBus) are sound (Gemini "strict improvement"; Codex "main rewrite correct")
- ✅ Required trait method (MF3) is the correct pattern for constitutional anchor
- ✅ Helper extraction (MF2) is the right design — Codex's only complaint is `pub(crate)` vs `pub` (one-keyword fix)
- ✅ No foundational design flaw

## Where audits disagreed (round-3)

- **MF2 helper visibility**: Gemini didn't notice that `pub(crate)` blocks integration-test access (MF5 flat-test placement); Codex caught it. Conservative-wins → fix.
- **MF4 stale references**: Gemini didn't find the line-14 and line-395 leftovers; Codex caught them. Mechanical fix.
- **B1 compile error**: pure source-level finding only Codex's deep-grep would catch.

The disagreements are NOT architectural — Gemini's PASS is architecturally sound; Codex's CHALLENGE is mechanical-correctness. Both verdicts are coherent at their respective angles.

## Conservative-merged decision (no further audit input needed)

ArchitectAI applies all 4 patches (B1-B4) directly to v1.1 spec → v1.2. Per "无损压缩即智能", these are mechanical fixes; no architectural decision points remain ambiguous. Round-4 audit budget after v1.2: ~$3-7 (1 round expected to PASS/PASS — only mechanical fixes need verification).

## Audit cost summary

- Codex r3: 140,559 tokens (smaller than r2's 158k)
- Gemini r3: prompt + candidates ~140k tokens
- Estimated round cost: ~$5-10
- Cumulative project audit spend: ~$194-310 / $890 mid-budget (~22-35%)

## Status going forward

1. **CO1.7-extra v1.2**: spec patched in place this session (4 patches: B1 compile fix, B2 visibility, B3 stale refs ×2, B4 small inconsistencies); awaiting round-4 dual audit
2. **CO1.7.5 (transition bodies)**: future atom (unchanged from r1+r2 verdicts)
3. **LATEST.md correction**: still pending (per r1 + r2 + r3 verdicts; ~30-40% Wave 6 #1 progress diagnosis confirmed three rounds running)


---

# XREF: shipped src/state/sequencer.rs (B1 + B2 target)

```rust
//! L4 Sequencer + dispatch_transition (CO1.7-impl A2 + A3).
//!
//! Spec authority:
//! - `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` § 3 (Sequencer
//!   pseudocode, K1 dual-counter, K3 head_t deferred, C3 sign API)
//! - `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` § 8
//!   (dispatch_transition exhaustive enum match; K5 Slash dropped)
//!
//! Single-writer per (runtime_repo, run_id). Per spec § 5.2.1.
//!
//! **Stub state (this atom)**: every per-kind transition returns
//! `TransitionError::NotYetImplemented`; CO1.7.5 (downstream atom) fills the
//! bodies. The structural correctness of the apply path (snapshot → dispatch →
//! CAS put → sign → root fold → commit → Q_t mutation) is locked by the
//! impl + tests here; what's left is per-kind transition logic.
//!
//! /// TRACE_MATRIX § 5.2.1 + § 8 — L4 sequencer single-writer + dispatch.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use crate::bottom_white::cas::schema::ObjectType;
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::bottom_white::ledger::system_keypair::{
    transition_ledger_emitter, Ed25519Keypair, KeypairError, SystemEpoch,
};
use crate::bottom_white::ledger::transition_ledger::{
    append, canonical_encode, LedgerEntry, LedgerEntrySigningPayload, LedgerWriter,
    LedgerWriterError,
};
use crate::bottom_white::tools::registry::ToolRegistry;
use crate::state::q_state::QState;
use crate::state::typed_tx::{SignalBundle, TransitionError, TypedTx};
use crate::top_white::predicates::registry::PredicateRegistry;

// ────────────────────────────────────────────────────────────────────────────
// § 8 dispatch_transition — exhaustive enum match (K5: NO Slash)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 8 — exhaustive dispatch over `TypedTx` variants.
///
/// **Stub state (CO1.7-impl A3)**: every variant returns
/// `TransitionError::NotYetImplemented`. CO1.7.5 fills each arm with the real
/// transition body per `STATE_TRANSITION_SPEC § 3.1-3.7`. The exhaustive match
/// itself is the contract: any future TypedTx variant addition triggers a
/// non-exhaustive-match compile error here, forcing explicit handling.
pub(crate) fn dispatch_transition(
    _q: &QState,
    tx: &TypedTx,
    _predicate_registry: &PredicateRegistry,
    _tool_registry: &ToolRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {
    match tx {
        TypedTx::Work(_) => Err(TransitionError::NotYetImplemented),
        TypedTx::Verify(_) => Err(TransitionError::NotYetImplemented),
        TypedTx::Challenge(_) => Err(TransitionError::NotYetImplemented),
        TypedTx::Reuse(_) => Err(TransitionError::NotYetImplemented),
        TypedTx::FinalizeReward(_) => Err(TransitionError::NotYetImplemented),
        TypedTx::TaskExpire(_) => Err(TransitionError::NotYetImplemented),
        TypedTx::TerminalSummary(_) => Err(TransitionError::NotYetImplemented),
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Submission types — K1 dual counter
// ────────────────────────────────────────────────────────────────────────────

/// Returned by `Sequencer::submit`. Carries `submit_id` (always assigned at
/// submit time) but **NOT** `logical_t` — logical_t is only assigned post-accept
/// per K1 (see spec § 3 + CO1.7 K1 closure).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubmissionReceipt {
    pub submit_id: u64,
}

#[derive(Debug)]
pub enum SubmitError {
    /// Bounded queue saturated (Q1/Q2 resolution: agent retries with backoff).
    QueueFull,
    /// Receiver dropped — sequencer no longer running.
    QueueClosed,
}

impl std::fmt::Display for SubmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueueFull => write!(f, "submission queue saturated"),
            Self::QueueClosed => write!(f, "submission queue closed"),
        }
    }
}
impl std::error::Error for SubmitError {}

/// Errors that can occur during `apply_one`. Spec § 3 implicitly assumes
/// `Result<_, TransitionError>` but the actual `?`-propagated error chain
/// crosses CAS, keypair, and ledger-writer boundaries — wrapper enum captures
/// all of these explicitly. **Implementation note vs. spec**: spec § 3 line
/// 307 writes the apply_one signature as `Result<LedgerEntry, TransitionError>`;
/// this implementation widens to `Result<LedgerEntry, ApplyError>` to preserve
/// distinct error provenance (TransitionError keeps its closed taxonomy +
/// additive-only invariant per CO1.1.4-pre1 § 7.2).
#[derive(Debug)]
pub enum ApplyError {
    /// Pure transition function rejected the tx.
    Transition(TransitionError),
    /// CAS payload put failed.
    Cas(CasError),
    /// System keypair sign failed.
    Keypair(KeypairError),
    /// Ledger writer commit failed.
    LedgerCommit(LedgerWriterError),
    /// Internal: canonical encoding of typed-tx payload failed (should never
    /// happen for serde-derive types; surfaced for completeness).
    PayloadEncode(String),
    /// `q.read()` / `q.write()` lock poisoned by panicking thread.
    QStateLockPoisoned,
}

impl std::fmt::Display for ApplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transition(e) => write!(f, "transition rejected: {e}"),
            Self::Cas(e) => write!(f, "cas put failed: {e}"),
            Self::Keypair(e) => write!(f, "keypair sign failed: {e:?}"),
            Self::LedgerCommit(e) => write!(f, "ledger commit failed: {e}"),
            Self::PayloadEncode(s) => write!(f, "payload encode failed: {s}"),
            Self::QStateLockPoisoned => write!(f, "q-state lock poisoned"),
        }
    }
}
impl std::error::Error for ApplyError {}

impl From<TransitionError> for ApplyError {
    fn from(e: TransitionError) -> Self {
        Self::Transition(e)
    }
}
impl From<CasError> for ApplyError {
    fn from(e: CasError) -> Self {
        Self::Cas(e)
    }
}
impl From<KeypairError> for ApplyError {
    fn from(e: KeypairError) -> Self {
        Self::Keypair(e)
    }
}
impl From<LedgerWriterError> for ApplyError {
    fn from(e: LedgerWriterError) -> Self {
        Self::LedgerCommit(e)
    }
}

#[derive(Debug)]
pub enum SequencerError {
    /// `run()` was called when the receiver had already been consumed.
    ReceiverAlreadyTaken,
}

impl std::fmt::Display for SequencerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReceiverAlreadyTaken => write!(f, "sequencer receiver already taken"),
        }
    }
}
impl std::error::Error for SequencerError {}

// ────────────────────────────────────────────────────────────────────────────
// Sequencer — single-writer per (runtime_repo, run_id)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 5.2.1 — L4 sequencer; single-writer per (runtime_repo, run_id).
///
/// **K1 dual counter**: `next_submit_id` advances at every `submit()` (used to
/// derive `SubmissionReceipt.submit_id`); `next_logical_t` advances ONLY at
/// commit time (rejected submissions never get a logical_t — preserves
/// `LedgerWriter`'s strict logical_t monotonicity invariant).
///
/// **K3 v1.2 (revised)**: Sequencer does NOT mutate `q.head_t` or
/// `q.state_root_t` directly; the transition function returns the new
/// `QState` and the sequencer accepts it as-is. `head_t` mutation defers to
/// CO1.7.5+ wiring (when `Git2LedgerWriter::commit` provides commit_sha
/// alongside Hash).
///
/// **C3 sign API**: signs through
/// `transition_ledger_emitter::sign_ledger_entry(keypair, digest_bytes)` —
/// the typed `CanonicalMessage::LedgerEntrySigning([u8;32])` extension closes
/// the C3 round-2 audit point.
pub struct Sequencer {
    /// K1: assigned at submit; never appears in LedgerEntry.
    next_submit_id: AtomicU64,
    /// K1: advances ONLY on commit; first accepted entry gets logical_t=1.
    next_logical_t: AtomicU64,

    queue_tx: tokio::sync::mpsc::Sender<TypedTx>,

    cas: Arc<RwLock<CasStore>>,
    keypair: Arc<Ed25519Keypair>,
    epoch: SystemEpoch,
    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,

    predicate_registry: Arc<PredicateRegistry>,
    tool_registry: Arc<ToolRegistry>,

    q: RwLock<QState>,
}

impl Sequencer {
    /// Construct. Returns the `Sequencer` plus the receiver half of the
    /// internal mpsc; pass the receiver to `run()` exactly once.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cas: Arc<RwLock<CasStore>>,
        keypair: Arc<Ed25519Keypair>,
        epoch: SystemEpoch,
        ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
        predicate_registry: Arc<PredicateRegistry>,
        tool_registry: Arc<ToolRegistry>,
        initial_q: QState,
        queue_capacity: usize,
    ) -> (Self, tokio::sync::mpsc::Receiver<TypedTx>) {
        let (queue_tx, queue_rx) = tokio::sync::mpsc::channel(queue_capacity);
        let seq = Self {
            next_submit_id: AtomicU64::new(1),
            next_logical_t: AtomicU64::new(0), // first accepted commit advances to 1
            queue_tx,
            cas,
            keypair,
            epoch,
            ledger_writer,
            predicate_registry,
            tool_registry,
            q: RwLock::new(initial_q),
        };
        (seq, queue_rx)
    }

    /// Submit a typed transition. Returns immediately with a receipt carrying
    /// `submit_id` (NOT `logical_t`). Per Q2 (back-pressure resolution): on
    /// queue saturation returns `Err(SubmitError::QueueFull)` and the agent is
    /// expected to retry with deterministic exponential backoff.
    pub async fn submit(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
        let submit_id = self.next_submit_id.fetch_add(1, Ordering::SeqCst);
        match self.queue_tx.try_send(tx) {
            Ok(()) => Ok(SubmissionReceipt { submit_id }),
            Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => Err(SubmitError::QueueFull),
            Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => Err(SubmitError::QueueClosed),
        }
    }

    /// Driver loop. Drains the queue and runs `apply_one` on each tx. Errors
    /// from individual `apply_one` calls are logged and skipped (per-tx
    /// rejection does NOT halt the sequencer). Returns when the queue is
    /// closed and drained.
    pub async fn run(
        &self,
        mut queue_rx: tokio::sync::mpsc::Receiver<TypedTx>,
    ) -> Result<(), SequencerError> {
        while let Some(tx) = queue_rx.recv().await {
            // Stub state: dispatch returns NotYetImplemented; apply_one
            // bubbles up. We log and continue per spec § 3 v1.2 ordering rule
            // (rejection does not consume a logical_t — see K1).
            if let Err(e) = self.apply_one(tx) {
                log::debug!("sequencer apply_one rejected: {e}");
            }
        }
        Ok(())
    }

    /// Per-tx critical section. Pure transition + CAS put + sign + commit +
    /// Q_t mutation. See spec § 3 stages 1-9.
    ///
    /// **v1.1 C-2 closure (Codex bundle Q-B)**: `next_logical_t` advances
    /// **only on commit success** — the original spec § 3 stage-4
    /// `fetch_add(1)` happened BEFORE sign + writer.commit, so any infra
    /// failure (sign / commit) left `next_logical_t` advanced past a
    /// logical_t that was never written to the ledger. The next accepted
    /// tx would then be assigned a logical_t the writer rejects forever
    /// (writer enforces strict `len + 1`). Fixed by `load → use → store
    /// after commit succeeds`. Single-writer per spec § 5.2.1 makes the
    /// load+store atomic enough; if multi-writer ever lands the AtomicU64
    /// can be upgraded to a `compare_exchange` reservation pattern.
    pub(crate) fn apply_one(&self, tx: TypedTx) -> Result<LedgerEntry, ApplyError> {
        // Stage 1: snapshot Q_t under read lock.
        let q_snapshot = {
            let g = self.q.read().map_err(|_| ApplyError::QStateLockPoisoned)?;
            g.clone()
        };

        // Stage 2: dispatch (pure). On reject (incl. NotYetImplemented stub),
        // EARLY RETURN. K1: no logical_t consumed.
        let (q_next, _signals) = dispatch_transition(
            &q_snapshot,
            &tx,
            &self.predicate_registry,
            &self.tool_registry,
        )?;

        // v1.1 C-2: TENTATIVE logical_t (do NOT fetch_add yet).
        let logical_t = self.next_logical_t.load(Ordering::SeqCst) + 1;

        // Stage 3: put payload to CAS. DIV-5 5-param put signature.
        let payload_bytes = canonical_encode(&tx)
            .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
        let payload_cid = {
            let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
            cas_w.put(
                &payload_bytes,
                ObjectType::ProposalPayload,
                &format!("sequencer-epoch-{}", self.epoch.get()),
                logical_t,
                Some("TypedTx.v1".to_string()),
            )?
        };

        // Stage 5: build LedgerEntrySigningPayload (v1.1 — stage 4 fetch_add
        // moved to AFTER stage 9 commit success).
        let signing_payload = LedgerEntrySigningPayload {
            logical_t,
            parent_state_root: q_snapshot.state_root_t,
            parent_ledger_root: q_snapshot.ledger_root_t,
            tx_kind: tx.tx_kind(),
            tx_payload_cid: payload_cid,
            resulting_state_root: q_next.state_root_t,
            timestamp_logical: logical_t,
            epoch: self.epoch,
            extensions: std::collections::BTreeMap::new(),
        };

        // Stage 6: C3 — sign via typed CanonicalMessage::LedgerEntrySigning.
        let signing_digest = signing_payload.canonical_digest();
        let system_signature = transition_ledger_emitter::sign_ledger_entry(
            &self.keypair,
            signing_digest.0,
        )?;

        // Stage 7: pure ledger-root fold (deterministic).
        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);

        // Stage 8: build LedgerEntry (the stored record).
        let entry = LedgerEntry {
            logical_t: signing_payload.logical_t,
            parent_state_root: signing_payload.parent_state_root,
            parent_ledger_root: signing_payload.parent_ledger_root,
            tx_kind: signing_payload.tx_kind,
            tx_payload_cid: signing_payload.tx_payload_cid,
            resulting_state_root: signing_payload.resulting_state_root,
            resulting_ledger_root,
            timestamp_logical: signing_payload.timestamp_logical,
            epoch: signing_payload.epoch,
            extensions: signing_payload.extensions,
            system_signature,
        };

        // Stage 9: commit + mutate Q_t under write lock.
        // v1.1 C-2: next_logical_t.store(logical_t) HAPPENS ONLY AFTER
        // writer.commit succeeds — preserves K1 under infra failure.
        // K3 v1.2 (revised): we set q.ledger_root_t but NOT q.head_t (head_t
        // mutation deferred to CO1.7.5+ when Git2LedgerWriter exposes
        // commit_sha alongside Hash). state_root_t comes from q_next as-is.
        {
            let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
            let mut writer_w = self
                .ledger_writer
                .write()
                .map_err(|_| ApplyError::QStateLockPoisoned)?;
            writer_w.commit(&entry)?; // ← may fail; if it does, fetch_add was NOT called
            // commit succeeded → safe to advance counter.
            self.next_logical_t.store(logical_t, Ordering::SeqCst);
            *q_w = q_next;
            q_w.ledger_root_t = entry.resulting_ledger_root;
        }

        Ok(entry)
    }

    /// Read-only accessor (testing + CO1.7.5+ wiring).
    pub fn q_snapshot(&self) -> Result<QState, ApplyError> {
        self.q
            .read()
            .map(|g| g.clone())
            .map_err(|_| ApplyError::QStateLockPoisoned)
    }

    pub fn next_submit_id_peek(&self) -> u64 {
        self.next_submit_id.load(Ordering::SeqCst)
    }

    pub fn next_logical_t_peek(&self) -> u64 {
        self.next_logical_t.load(Ordering::SeqCst)
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Tests — stub-mode coverage (CO1.7.5 fills real-transition tests)
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bottom_white::ledger::transition_ledger::InMemoryLedgerWriter;
    use crate::state::typed_tx::{
        AgentSignature, BoolWithProof, ChallengeTx, ClaimId, FinalizeRewardTx, PredicateId,
        PredicateResultsBundle, ReadKey, ReuseTx, RunId, RunOutcome, SafetyOrCreation,
        TaskExpireTx, TaskId, TerminalSummaryTx, ToolId, VerifyTx, VerifyVerdict, WorkTx,
        WriteKey,
    };
    use crate::state::q_state::{AgentId, TxId};
    use crate::economy::money::{MicroCoin, StakeMicroCoin};
    use crate::bottom_white::cas::schema::Cid;
    use crate::bottom_white::ledger::system_keypair::SystemSignature;
    use std::collections::{BTreeMap, BTreeSet};
    use tempfile::TempDir;

    fn fresh_sequencer() -> (
        TempDir,
        Sequencer,
        tokio::sync::mpsc::Receiver<TypedTx>,
    ) {
        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas open")));
        let keypair = Arc::new(
            Ed25519Keypair::generate_with_secure_entropy().expect("keypair gen"),
        );
        let epoch = SystemEpoch::new(1);
        let writer: Arc<RwLock<dyn LedgerWriter>> =
            Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
        let preds = Arc::new(PredicateRegistry::new());
        let tools = Arc::new(ToolRegistry::new());
        let q = QState::genesis();
        let (seq, rx) = Sequencer::new(cas, keypair, epoch, writer, preds, tools, q, 16);
        (tmp, seq, rx)
    }

    fn fixture_work_tx() -> WorkTx {
        let mut acceptance = BTreeMap::new();
        acceptance.insert(
            PredicateId("acc1".into()),
            BoolWithProof {
                value: true,
                proof_cid: None,
            },
        );
        WorkTx {
            tx_id: TxId("worktx-seq-fixture".into()),
            task_id: TaskId("task-seq-fixture".into()),
            parent_state_root: Default::default(),
            agent_id: AgentId("alice".into()),
            read_set: [ReadKey("k.read.a".into())].into_iter().collect::<BTreeSet<_>>(),
            write_set: [WriteKey("k.write.a".into())].into_iter().collect::<BTreeSet<_>>(),
            proposal_cid: Default::default(),
            predicate_results: PredicateResultsBundle {
                acceptance,
                settlement: BTreeMap::new(),
                safety_class: SafetyOrCreation::Safety,
            },
            stake: StakeMicroCoin::from_micro_units(1_000_000),
            signature: AgentSignature::from_bytes([0x77u8; 64]),
            timestamp_logical: 1,
        }
    }

    // 1. dispatch_transition: every variant returns NotYetImplemented (stub state).
    #[test]
    fn dispatch_transition_stubs_all_variants() {
        let q = QState::genesis();
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();

        let cases: Vec<TypedTx> = vec![
            TypedTx::Work(fixture_work_tx()),
            TypedTx::Verify(VerifyTx {
                tx_id: TxId("vt".into()),
                target_work_tx: TxId("wt".into()),
                verifier_agent: AgentId("v".into()),
                bond: StakeMicroCoin::from_micro_units(1),
                verdict: VerifyVerdict::Confirm,
                signature: AgentSignature::from_bytes([0; 64]),
                timestamp_logical: 1,
            }),
            TypedTx::Challenge(ChallengeTx {
                tx_id: TxId("ct".into()),
                target_work_tx: TxId("wt".into()),
                challenger_agent: AgentId("c".into()),
                stake: StakeMicroCoin::from_micro_units(1),
                counterexample_cid: Cid([0; 32]),
                signature: AgentSignature::from_bytes([0; 64]),
                timestamp_logical: 1,
            }),
            TypedTx::Reuse(ReuseTx {
                tx_id: TxId("rt".into()),
                reusing_work_tx: TxId("wt".into()),
                reused_tool_id: ToolId("tool".into()),
                reused_tool_creator: AgentId("a".into()),
                timestamp_logical: 1,
            }),
            TypedTx::FinalizeReward(FinalizeRewardTx {
                tx_id: TxId("ft".into()),
                claim_id: ClaimId::new("cl"),
                task_id: TaskId("t".into()),
                solver: AgentId("s".into()),
                reward: MicroCoin::from_micro_units(1),
                parent_state_root: Default::default(),
                epoch: SystemEpoch::new(1),
                timestamp_logical: 1,
                system_signature: SystemSignature::from_bytes([0; 64]),
            }),
            TypedTx::TaskExpire(TaskExpireTx {
                tx_id: TxId("et".into()),
                task_id: TaskId("t".into()),
                parent_state_root: Default::default(),
                bounty_refunded: MicroCoin::from_micro_units(1),
                epoch: SystemEpoch::new(1),
                timestamp_logical: 1,
                system_signature: SystemSignature::from_bytes([0; 64]),
            }),
            TypedTx::TerminalSummary(TerminalSummaryTx {
                tx_id: TxId("ts".into()),
                task_id: TaskId("t".into()),
                run_id: RunId("r".into()),
                run_outcome: RunOutcome::OmegaAccepted,
                total_attempts: 0,
                failure_class_histogram: BTreeMap::new(),
                last_logical_t: 0,
                system_signature: SystemSignature::from_bytes([0; 64]),
            }),
        ];

        for tx in cases {
            let result = dispatch_transition(&q, &tx, &preds, &tools);
            assert!(matches!(result, Err(TransitionError::NotYetImplemented)));
        }
    }

    // 2. K1 dual counter: submit advances submit_id but NOT logical_t.
    #[tokio::test]
    async fn submit_advances_submit_id_only() {
        let (_tmp, seq, _rx) = fresh_sequencer();
        assert_eq!(seq.next_submit_id_peek(), 1);
        assert_eq!(seq.next_logical_t_peek(), 0);

        let r1 = seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("submit 1");
        assert_eq!(r1.submit_id, 1);
        assert_eq!(seq.next_submit_id_peek(), 2);
        assert_eq!(seq.next_logical_t_peek(), 0, "logical_t MUST NOT advance at submit");

        let r2 = seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("submit 2");
        assert_eq!(r2.submit_id, 2);
        assert_eq!(seq.next_logical_t_peek(), 0);
    }

    // 3. apply_one in stub mode: returns Transition(NotYetImplemented); no
    //    logical_t consumed (K1 invariant: rejected submission never advances commit counter).
    #[test]
    fn apply_one_stub_does_not_consume_logical_t() {
        let (_tmp, seq, _rx) = fresh_sequencer();
        let pre = seq.next_logical_t_peek();
        let err = seq.apply_one(TypedTx::Work(fixture_work_tx())).unwrap_err();
        assert!(matches!(err, ApplyError::Transition(TransitionError::NotYetImplemented)));
        let post = seq.next_logical_t_peek();
        assert_eq!(pre, post, "logical_t MUST NOT advance on rejected apply_one");
    }

    // 4. Queue saturation: submit returns QueueFull (Q1/Q2 resolution).
    #[tokio::test]
    async fn submit_returns_queue_full_on_saturation() {
        // Capacity=2; receiver never drained.
        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
        let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("kp"));
        let writer: Arc<RwLock<dyn LedgerWriter>> =
            Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
        let preds = Arc::new(PredicateRegistry::new());
        let tools = Arc::new(ToolRegistry::new());
        let (seq, _rx) = Sequencer::new(
            cas,
            keypair,
            SystemEpoch::new(1),
            writer,
            preds,
            tools,
            QState::genesis(),
            2,
        );
        // Fill capacity.
        seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("1");
        seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("2");
        // Saturated.
        let err = seq.submit(TypedTx::Work(fixture_work_tx())).await.unwrap_err();
        assert!(matches!(err, SubmitError::QueueFull));
    }

    // 5. submit returns QueueClosed when receiver dropped.
    #[tokio::test]
    async fn submit_returns_queue_closed_after_rx_drop() {
        let (_tmp, seq, rx) = fresh_sequencer();
        drop(rx);
        let err = seq.submit(TypedTx::Work(fixture_work_tx())).await.unwrap_err();
        assert!(matches!(err, SubmitError::QueueClosed));
    }
}

```

---

# XREF: shipped src/bus.rs (B4 serde context — TuringBus has no serde derives)

```rust
// Tier 4: TSP Event Bus — SKILL lifecycle serial reactor
// Constitutional basis: Art. II (selective broadcast), Art. III (selective shielding)
// V3L-11: serial reactor for causal ordering (no concurrent pricing oscillation)
// V3L-21: one-step-per-node payload limits
// V3L-31: supervisor loop, never silent exit
// V3L-32: cascade failure protection

use crate::kernel::{Kernel, KernelError};
use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
use crate::sdk::tool::{BetDirection, ToolSignal, TuringTool};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Symbolic constants (V-01 ceremonial kill per D-VETO-7 ratified A) ──────────

/// TRACE_MATRIX FC1-Cost / FC3-Cost: placeholder until CO1.1.4 STEP_B propagates
/// real LLM completion tokens from `drivers::llm_http::LlmResponse` through to
/// `Node::completion_tokens`. CO1.1.4-pre1 ceremonial commit replaces the magic
/// literal `0` at `bus.rs:268` with this named constant; the value is unchanged
/// (still 0), but the literal is killed so the STEP_B refactor has a clear
/// rename target rather than an anonymous integer.
///
/// See `handover/architect-insights/PROJECT_DECISION_MAP_2026-04-27.md` § 2.2
/// D-VETO-7 for the ratified disposition.
pub(crate) const PENDING_COMPLETION_TOKENS_CO1_1_4: u32 = 0;

// ── Configuration ───────────────────────────────────────────────

/// Bus configuration. V3L-23: no hardcoded values, all configurable.
pub struct BusConfig {
    pub max_payload_chars: usize,
    pub max_payload_lines: usize,
    pub system_lp_amount: f64,
    pub forbidden_patterns: Vec<String>,
}

impl Default for BusConfig {
    fn default() -> Self {
        BusConfig {
            max_payload_chars: 1600,
            max_payload_lines: 24,
            system_lp_amount: 200.0,
            forbidden_patterns: Vec::new(),
        }
    }
}

// ── Core Bus ────────────────────────────────────────────────────

/// The serial event reactor.
/// V3L-11: ALL state mutations go through this single-threaded reactor.
/// No concurrent access to kernel/markets — causal ordering guaranteed.
pub struct TuringBus {
    pub kernel: Kernel,
    pub ledger: Ledger,
    pub tools: Vec<Box<dyn TuringTool>>,
    pub config: BusConfig,
    pub clock: u64,
    pub tx_count: u64,
    pub generation: u32,
    graveyard: HashMap<String, Vec<String>>,
    // Phase 1 (C-037 candidate): durable Q_t. None = legacy in-memory mode.
    wal: Option<crate::wal::Wal>,
}

/// Scope for recent_rejections query.
/// Step-B v3 Art. II.1 fix: enables global abstract-broadcast without violating C-022.
#[derive(Debug, Clone, Copy)]
pub enum RejectionScope {
    /// Legacy: per-author graveyard (before-fix behavior).
    PerAuthor,
    /// Flattened across all authors, chronological (may leak raw content — use with caution).
    Global,
    /// Art. II.1 compliant: counted + top-k class labels. Requires callers to record class labels.
    TopKClasses(usize),
}

/// Result of a bus append operation.
#[derive(Debug)]
pub enum BusResult {
    Appended { node_id: NodeId },
    Invested { node_id: NodeId, shares: f64 },
    Vetoed { reason: String },
}

impl TuringBus {
    pub fn new(kernel: Kernel, config: BusConfig) -> Self {
        TuringBus {
            kernel,
            ledger: Ledger::new(),
            tools: Vec::new(),
            config,
            clock: 0,
            tx_count: 0,
            generation: 0,
            graveyard: HashMap::new(),
            wal: None,
        }
    }

    /// Phase 1: open with WAL persistence. If the path exists, replay it to
    /// rebuild tape + ledger state (resume mode). If not, start fresh and append
    /// to the WAL going forward (durable mode). Either way, the Wal handle is
    /// retained and every successful tape.append / ledger.append persists.
    pub fn with_wal_path(
        kernel: Kernel,
        config: BusConfig,
        wal_path: impl Into<std::path::PathBuf>,
    ) -> Result<Self, std::io::Error> {
        let wal_path = wal_path.into();
        let mut bus = Self::new(kernel, config);
        // Replay first (if file exists), then open in append mode.
        let (nodes, events) = crate::wal::Wal::replay(&wal_path)?;
        let resumed_nodes = nodes.len();
        let resumed_events = events.len();
        for n in nodes {
            // Replay errors are tolerable — duplicates and dangling cites can
            // happen if the WAL was concurrently appended at a stale point. We
            // log and skip; the surviving prefix is canonical Q_t.
            if let Err(e) = bus.kernel.append(n.clone()) {
                eprintln!("[wal/replay] skip node {}: {}", n.id, e);
            }
        }
        for e in events {
            // Re-append events through the ledger so hash chain is recomputed
            // from this process's perspective. Original hashes are discarded.
            bus.ledger.append(e.event_type, e.node_id, e.agent, e.detail).ok();
        }
        if resumed_nodes > 0 || resumed_events > 0 {
            eprintln!("[wal/replay] resumed {} nodes, {} events from {:?}",
                      resumed_nodes, resumed_events, wal_path);
        }
        bus.wal = Some(crate::wal::Wal::open(&wal_path)?);
        Ok(bus)
    }

    /// Mount a tool into the bus. Tools execute in mount order.
    pub fn mount_tool(&mut self, tool: Box<dyn TuringTool>) {
        self.tools.push(tool);
    }

    /// Boot all tools.
    pub fn boot(&mut self) {
        for tool in &mut self.tools {
            tool.on_boot();
        }
    }

    /// Initialize all tools with agent list. Triggers GENESIS.
    pub fn init(&mut self, agent_ids: &[String]) {
        for tool in &mut self.tools {
            tool.on_init(agent_ids);
        }
        // Phase 3A (Hayek): open the bounty market at genesis if the feature
        // is enabled. Seed from ghost-liquidity pool (same exemption as per-
        // node markets — pre-committed LP, not a mint). BOUNTY_LP env tunable
        // for experimentation; constitutional default lands later.
        if std::env::var("HAYEK_BOUNTY").ok().as_deref() == Some("1") {
            let lp: f64 = std::env::var("BOUNTY_LP")
                .ok().and_then(|s| s.parse().ok())
                .unwrap_or(self.config.system_lp_amount);
            let _ = self.kernel.open_bounty_market(lp);
        }
        if let Ok(evt) = self.ledger.append(EventType::RunStart, None, None, None) {
            let evt_clone = evt.clone();
            if let Some(w) = self.wal.as_mut() {
                let _ = w.write_event(&evt_clone);
            }
        }
    }

    /// The main append pipeline — 6 phases.
    /// V3L-11: this runs serially, never concurrently.
    pub fn append(&mut self, author: &str, payload: &str,
                  parent_id: Option<&str>) -> Result<BusResult, String> {
        self.append_internal(author, payload, parent_id, /*oracle_blessed*/ false)
    }

    /// Phase 2.1 (C-043 candidate): bypass agent-facing gates for ∏p-blessed payloads.
    /// The forbidden_patterns list (C-011) exists to prevent agents from appending
    /// brute-force tactics (e.g. bare `decide`, `omega`, `native_decide`) as scratch
    /// work. Once the Lean oracle has accepted a full proof, those same tactics are
    /// by construction legitimate — re-rejecting at bus level would block the
    /// wtool write that Art. IV mandates. Only oracle-accepted payloads should
    /// take this path. Payload-size caps are also relaxed (proofs are longer than
    /// agent scratch steps).
    pub fn append_oracle_accepted(&mut self, author: &str, payload: &str,
                                   parent_id: Option<&str>) -> Result<BusResult, String> {
        self.append_internal(author, payload, parent_id, /*oracle_blessed*/ true)
    }

    fn append_internal(&mut self, author: &str, payload: &str,
                       parent_id: Option<&str>, oracle_blessed: bool) -> Result<BusResult, String> {
        // Phase 0: Forbidden pattern check — skipped for oracle-accepted payloads.
        if !oracle_blessed {
            for pattern in &self.config.forbidden_patterns {
                if payload.contains(pattern.as_str()) {
                    let reason = format!("Forbidden pattern: {}", pattern);
                    self.record_rejection(author, &reason);
                    return Ok(BusResult::Vetoed { reason });
                }
            }
        }

        // Phase 0b: Payload size limits (V3L-21). Skipped for oracle-accepted since
        // real proofs can legitimately exceed the per-step scratch budget.
        if !oracle_blessed {
            if payload.len() > self.config.max_payload_chars {
                let reason = format!("Payload too long: {} > {} chars",
                                     payload.len(), self.config.max_payload_chars);
                self.record_rejection(author, &reason);
                return Ok(BusResult::Vetoed { reason });
            }
            let line_count = payload.lines().count();
            if line_count > self.config.max_payload_lines {
                let reason = format!("Too many lines: {} > {}",
                                     line_count, self.config.max_payload_lines);
                self.record_rejection(author, &reason);
                return Ok(BusResult::Vetoed { reason });
            }
        }

        // Phase 1: Tool pre-append hooks
        let mut signal = ToolSignal::Pass;
        for tool in &mut self.tools {
            match tool.on_pre_append(author, payload) {
                ToolSignal::Veto(reason) => {
                    self.record_rejection(author, &reason);
                    return Ok(BusResult::Vetoed { reason });
                }
                ToolSignal::InvestOnly { target_node, amount, direction } => {
                    signal = ToolSignal::InvestOnly { target_node, amount, direction };
                    break;
                }
                ToolSignal::YieldReward { reward } => {
                    signal = ToolSignal::YieldReward { reward };
                }
                ToolSignal::Pass => {}
            }
        }

        // Phase 2: InvestOnly routing (skip append, buy shares)
        // Law 2: staking COSTS money — debit wallet before buying shares
        if let ToolSignal::InvestOnly { target_node, amount, direction } = signal {
            // Debit the agent's wallet BEFORE buying shares
            self.debit_wallet(author, amount)?;

            let shares = match direction {
                BetDirection::Long => self.kernel.buy_yes(&target_node, amount),
                BetDirection::Short => self.kernel.buy_no(&target_node, amount),
            }.map_err(|e| {
                // Refund on failure (Law 2: no silent burns)
                self.credit_wallet(author, amount);
                e.to_string()
            })?;

            if let Ok(evt) = self.ledger.append(EventType::Invest, Some(target_node.clone()),
                               Some(author.to_string()), None) {
                let evt_clone = evt.clone();
                if let Some(w) = self.wal.as_mut() {
                    let _ = w.write_event(&evt_clone);
                }
            }
            self.tx_count += 1;
            return Ok(BusResult::Invested { node_id: target_node, shares });
        }

        // Phase 3: Kernel append (topology validation)
        let node_id = format!("tx_{}_by_{}", self.tx_count, author);
        let citations = parent_id.map(|p| vec![p.to_string()]).unwrap_or_default();

        let node = Node {
            id: node_id.clone(),
            author: author.to_string(),
            payload: payload.to_string(),
            citations,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            completion_tokens: PENDING_COMPLETION_TOKENS_CO1_1_4,
        };

        self.kernel.append(node.clone()).map_err(|e| e.to_string())?;

        // Phase 1 WAL: persist node AFTER successful in-memory append, BEFORE
        // any downstream effects. At-most-one-loss-on-crash semantics: if the
        // process dies between in-memory insert and this write, the node is
        // lost on replay but every prior node survives. Log+continue on I/O
        // error rather than aborting the run (Q_t durability is best-effort
        // when disk is the failing component).
        if let Some(w) = self.wal.as_mut() {
            if let Err(e) = w.write_node(&node) {
                log::warn!("[wal] write_node({}) failed: {}", node.id, e);
            }
        }

        // Phase 4: System Market Maker (Magna Carta Rule #19 exemption)
        // System MM injects liquidity from a dedicated pool, NOT from agent wallets.
        // This is the only constitutional exception to Law 2 — MM impermanent loss
        // is an expected physical cost, not minting. See C-001/C-002 for history.
        self.kernel.create_market(&node_id, self.config.system_lp_amount)
            .ok(); // Market creation failure is non-fatal

        // Phase 2 (C-042 candidate): founder grant — the author of this tape
        // node auto-receives γ·system_lp YES shares. No Coin is minted: the
        // market's redeem math pays out at most `lp_coins` on the winning side,
        // so these shares draw from pre-committed ghost liquidity (same as how
        // agent `invest` does). Gated by TAPE_ECONOMY_V2=1; γ via
        // FOUNDER_GRANT_GAMMA env (experimental) → constitutional default at merge.
        if std::env::var("TAPE_ECONOMY_V2").ok().as_deref() == Some("1") {
            let gamma: f64 = std::env::var("FOUNDER_GRANT_GAMMA")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(0.05);
            let grant_shares = gamma * self.config.system_lp_amount;
            for tool in &mut self.tools {
                if tool.manifest() == "wallet" {
                    if let Some(wallet) = tool.as_any_mut()
                        .downcast_mut::<crate::sdk::tools::wallet::WalletTool>()
                    {
                        wallet.record_shares(author, &node_id, grant_shares, 0.0, 0.0);
                        break;
                    }
                }
            }
        }

        // Phase 5: Tool post-append hooks
        for tool in &mut self.tools {
            tool.on_post_append(author, &node_id);
        }

        if let Ok(evt) = self.ledger.append(EventType::Append, Some(node_id.clone()),
                                             Some(author.to_string()), None) {
            // Phase 1 WAL: persist ledger event for full hash-chain recovery.
            if let Some(w) = self.wal.as_mut() {
                let evt_clone = evt.clone();
                if let Err(e) = w.write_event(&evt_clone) {
                    log::warn!("[wal] write_event(Append) failed: {}", e);
                }
            }
        }
        self.tx_count += 1;
        self.clock += 1;

        Ok(BusResult::Appended { node_id })
    }

    /// Halt and settle — triggered by Oracle verification.
    pub fn halt_and_settle(&mut self, golden_path: &[NodeId]) -> Result<(), String> {
        // Resolve all markets
        self.kernel.resolve_all(golden_path).map_err(|e| e.to_string())?;

        // Phase 2: pay out every agent's YES/NO positions against resolved markets.
        // Shares redeem 1:1 against the winning side; losing shares redeem to 0.
        // Conservation: LP that backed the market flows to winners; total Coin
        // across the system is preserved (LP-side only). Gated behind the same
        // TAPE_ECONOMY_V2 toggle so baseline runs keep historical behaviour.
        if std::env::var("TAPE_ECONOMY_V2").ok().as_deref() == Some("1") {
            self.settle_portfolios();
        }

        // Phase 3A (Hayek): resolve the bounty market and distribute its
        // committed LP to GP-node authors by occurrence count. This creates
        // the cross-agent reward that makes appending a lemma EV-positive
        // independently of whether the lemma-author also closes the proof.
        if std::env::var("HAYEK_BOUNTY").ok().as_deref() == Some("1") {
            let gp_authors: Vec<String> = golden_path.iter()
                .filter_map(|nid| self.kernel.tape.get(nid).map(|n| n.author.clone()))
                .collect();
            let payouts = self.kernel.resolve_bounty(&gp_authors);
            for (agent, amount) in payouts {
                self.credit_wallet(&agent, amount);
            }
        }

        // Tool halt hooks
        let gp: Vec<String> = golden_path.to_vec();
        for tool in &mut self.tools {
            tool.on_halt(&gp);
        }

        if let Ok(evt) = self.ledger.append(EventType::RunEnd, None, None, None) {
            let evt_clone = evt.clone();
            if let Some(w) = self.wal.as_mut() {
                let _ = w.write_event(&evt_clone);
            }
        }
        Ok(())
    }

    /// Phase 2: redeem every agent's portfolio against resolved markets.
    /// Walks wallet.portfolios, finds matching resolved market, credits wallet
    /// with share count on the winning side (0 on the losing side). Resolved
    /// positions are zeroed to prevent double-redemption on a second call.
    /// Conservation: pays only from LP already committed at market creation.
    fn settle_portfolios(&mut self) {
        use crate::sdk::tools::wallet::WalletTool;
        // Snapshot resolved outcomes so we can borrow kernel + wallet disjointly.
        let outcomes: HashMap<String, bool> = self.kernel.markets.iter()
            .filter_map(|(id, m)| m.resolved.map(|w| (id.clone(), w)))
            .collect();
        let wallet: &mut WalletTool = match self.tools.iter_mut()
            .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
        {
            Some(w) => w,
            None => return,
        };
        let mut credits: Vec<(String, f64)> = Vec::new();
        for (agent, portfolio) in wallet.portfolios.iter_mut() {
            for (node_id, entry) in portfolio.iter_mut() {
                let (yes, no, _lp) = *entry;
                if let Some(yes_wins) = outcomes.get(node_id) {
                    let payout = if *yes_wins { yes } else { no };
                    if payout > 0.0 {
                        credits.push((agent.clone(), payout));
                    }
                    // Zero out settled positions to make settle_portfolios idempotent.
                    entry.0 = 0.0;
                    entry.1 = 0.0;
                }
            }
        }
        for (agent, amount) in credits {
            wallet.credit(&agent, amount);
        }
    }

    /// Debit an agent's wallet. Finds the WalletTool among mounted tools.
    fn debit_wallet(&mut self, agent: &str, amount: f64) -> Result<(), String> {
        for tool in &mut self.tools {
            if tool.manifest() == "wallet" {
                if let Some(wallet) = tool.as_any_mut().downcast_mut::<crate::sdk::tools::wallet::WalletTool>() {
                    return wallet.deduct(agent, amount);
                }
            }
        }
        Err("No wallet tool mounted".into())
    }

    /// Credit an agent's wallet (for refunds only — not new coins).
    fn credit_wallet(&mut self, agent: &str, amount: f64) {
        for tool in &mut self.tools {
            if tool.manifest() == "wallet" {
                if let Some(wallet) = tool.as_any_mut().downcast_mut::<crate::sdk::tools::wallet::WalletTool>() {
                    wallet.credit(agent, amount);
                    return;
                }
            }
        }
    }

    /// Record a rejection in the graveyard.
    /// Step-B v3: ALL stored entries are bounded class labels (C-022 shield enforced at write).
    /// If `reason` is already a valid class label (starts with "err:"), stored as-is.
    /// Otherwise normalized to a bus-level class via `bus_classify`.
    /// Exposed publicly so evaluator.rs can populate from OMEGA-reject and parse-fail.
    pub fn record_rejection(&mut self, author: &str, reason: &str) {
        let label = Self::bus_classify(reason);
        self.graveyard
            .entry(author.to_string())
            .or_default()
            .push(label.to_string());
    }

    /// Bus-level classifier: coerces any rejection reason to a bounded label.
    /// This is the write-side shield that enforces Art. II.1 end-to-end.
    /// The finite label set is the union of:
    ///   - "err:" prefixed labels from sdk::error_abstraction (caller-classified)
    ///   - "veto:forbidden", "veto:size", "veto:lines", "veto:wallet", "veto:tool_other"
    ///     (bus-internal veto classes)
    ///   - "err:other" catchall
    pub fn bus_classify(reason: &str) -> &'static str {
        // If caller already produced an "err:..." class label, trust it.
        // Validate prefix; the length is bounded because the enum of labels is finite.
        if reason.starts_with("err:") {
            // Accept as-is but intern to static slice where possible.
            // For simplicity we allocate a leaked &'static; safer: fixed mapping of known labels.
            // Here we collapse unknown "err:*" to err:other to preserve finite-set invariant.
            return match reason {
                "err:tactic_linarith" => "err:tactic_linarith",
                "err:tactic_simp_noprog" => "err:tactic_simp_noprog",
                "err:tactic_ring" => "err:tactic_ring",
                "err:tactic_norm_num" => "err:tactic_norm_num",
                "err:tactic_other" => "err:tactic_other",
                "err:unknown_const" => "err:unknown_const",
                "err:unsolved_goals" => "err:unsolved_goals",
                "err:unexpected_token" => "err:unexpected_token",
                "err:type_mismatch" => "err:type_mismatch",
                "err:rewrite_no_match" => "err:rewrite_no_match",
                "err:heartbeat" => "err:heartbeat",
                "err:other" => "err:other",
                _ => "err:other",
            };
        }
        // Bus internal veto reasons get their own bounded classes.
        if reason.starts_with("Forbidden") { return "veto:forbidden"; }
        if reason.starts_with("Payload too long") { return "veto:size"; }
        if reason.starts_with("Too many lines") { return "veto:lines"; }
        if reason.contains("wallet") || reason.contains("balance") { return "veto:wallet"; }
        if reason.starts_with("Tool") || reason.contains("tool") { return "veto:tool_other"; }
        "err:other"
    }

    /// Get recent rejections for an agent (Art. II.1: broadcast typical errors).
    /// v3 Step-B: default scope changed to TopKClasses(3) — globally abstract-and-broadcast.
    /// Call sites that explicitly want per-author scope use `recent_rejections_scoped`.
    pub fn recent_rejections(&self, author: &str, max: usize) -> Vec<String> {
        self.recent_rejections_scoped(author, max, RejectionScope::TopKClasses(3))
    }

    /// Scoped rejection query (Step-B v3 Art. II.1 fix).
    pub fn recent_rejections_scoped(
        &self,
        author: &str,
        max: usize,
        scope: RejectionScope,
    ) -> Vec<String> {
        match scope {
            RejectionScope::PerAuthor => {
                self.graveyard.get(author)
                    .map(|v| v.iter().rev().take(max).cloned().collect())
                    .unwrap_or_default()
            }
            RejectionScope::Global => {
                // Flatten all authors' recent; keep most recent `max` across swarm.
                let mut all: Vec<&String> = self.graveyard.values().flatten().collect();
                // Heuristic: assume push-order ~= time-order; take last `max` global entries.
                let start = all.len().saturating_sub(max);
                all.drain(..start);
                all.into_iter().cloned().collect()
            }
            RejectionScope::TopKClasses(k) => {
                // C-022 shield: broadcast abstracted CLASSES with COUNTS, not raw strings.
                // Expects reason strings to already be class labels (see error_abstraction).
                let mut counts: HashMap<String, u32> = HashMap::new();
                for v in self.graveyard.values() {
                    for r in v {
                        *counts.entry(r.clone()).or_insert(0) += 1;
                    }
                }
                let mut sorted: Vec<(String, u32)> = counts.into_iter().collect();
                // Sort: count DESC, then alphabetical (tiebreak stable).
                sorted.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
                sorted.truncate(k);
                // Emit as "label(count)" strings for prompt.
                sorted.into_iter()
                    .map(|(lbl, c)| format!("{}({})", lbl, c))
                    .take(max)
                    .collect()
            }
        }
    }

    /// Get a snapshot of the universe for agents to read.
    pub fn snapshot(&self) -> crate::sdk::snapshot::UniverseSnapshot {
        let markets: HashMap<NodeId, crate::sdk::snapshot::MarketSnapshot> =
            self.kernel.markets.iter()
                .map(|(id, m)| (id.clone(), crate::sdk::snapshot::MarketSnapshot {
                    yes_price: m.yes_price(),
                    no_price: m.no_price(),
                    yes_reserve: m.yes_reserve(),
                    no_reserve: m.no_reserve(),
                    resolved: m.resolved,
                }))
                .collect();

        // Extended ticker (Art. II.2 bidirectional price signal):
        //   up to 50 unresolved markets with YES/NO price + reserves.
        // Bigger cap reduces Matthew-effect; reserves let agents estimate
        // price impact before investing (market depth visibility).
        let ticker = self.kernel.market_ticker_full(50);
        let mut ticker_lines: Vec<String> = ticker.iter()
            .map(|(id, yes_p, no_p, yes_r, no_r)| {
                format!("{}: YES={:.1}% NO={:.1}% (Y={:.0} N={:.0})",
                    id, yes_p * 100.0, no_p * 100.0, yes_r, no_r)
            })
            .collect();
        // Phase 3A: surface the bounty price first so agents see the pre-
        // existing signal. No prose, no rule — just price-as-state (Hayek).
        if let Some(bp) = self.kernel.bounty_yes_price() {
            ticker_lines.insert(0,
                format!("__bounty__: {:.1}% (LP={:.0})", bp * 100.0,
                        self.kernel.bounty_lp_seed));
        }
        let ticker_str = ticker_lines.join(", ");

        crate::sdk::snapshot::UniverseSnapshot {
            tape: self.kernel.tape.clone(),
            balances: HashMap::new(), // filled by wallet tool query
            portfolios: HashMap::new(),
            markets,
            market_ticker: ticker_str,
            generation: self.generation,
            tx_count: self.tx_count,
        }
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sdk::tools::wallet::WalletTool;

    fn make_bus() -> TuringBus {
        let kernel = Kernel::new();
        let config = BusConfig {
            max_payload_chars: 200,
            max_payload_lines: 10,
            system_lp_amount: 200.0,
            forbidden_patterns: vec!["FORBIDDEN".to_string()],
        };
        let mut bus = TuringBus::new(kernel, config);
        bus.mount_tool(Box::new(WalletTool::new(10000.0)));
        bus.init(&["A0".into(), "A1".into()]);
        bus
    }

    #[test]
    fn test_bus_basic_append() {
        let mut bus = make_bus();
        match bus.append("A0", "step 1", None).unwrap() {
            BusResult::Appended { node_id } => {
                assert!(node_id.starts_with("tx_"));
                assert!(bus.kernel.tape.get(&node_id).is_some());
            }
            _ => panic!("Expected Appended"),
        }
    }

    #[test]
    fn test_bus_forbidden_pattern_veto() {
        let mut bus = make_bus();
        match bus.append("A0", "this is FORBIDDEN content", None).unwrap() {
            BusResult::Vetoed { reason } => {
                assert!(reason.contains("Forbidden"));
            }
            _ => panic!("Expected Vetoed"),
        }
    }

    #[test]
    fn test_bus_payload_too_long() {
        let mut bus = make_bus();
        let long_payload = "x".repeat(300);
        match bus.append("A0", &long_payload, None).unwrap() {
            BusResult::Vetoed { reason } => {
                assert!(reason.contains("too long"));
            }
            _ => panic!("Expected Vetoed"),
        }
    }

    #[test]
    fn test_bus_too_many_lines() {
        let mut bus = make_bus();
        let many_lines = (0..20).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");
        match bus.append("A0", &many_lines, None).unwrap() {
            BusResult::Vetoed { reason } => {
                assert!(reason.contains("many lines"));
            }
            _ => panic!("Expected Vetoed"),
        }
    }

    #[test]
    fn test_bus_unknown_agent_vetoed() {
        let mut bus = make_bus();
        match bus.append("unknown", "step", None).unwrap() {
            BusResult::Vetoed { reason } => {
                assert!(reason.contains("Unknown"));
            }
            _ => panic!("Expected Vetoed"),
        }
    }

    #[test]
    fn test_bus_creates_market_on_append() {
        let mut bus = make_bus();
        if let BusResult::Appended { node_id } = bus.append("A0", "step 1", None).unwrap() {
            assert!(bus.kernel.markets.contains_key(&node_id));
        }
    }

    #[test]
    fn test_bus_halt_and_settle() {
        let mut bus = make_bus();
        if let BusResult::Appended { node_id } = bus.append("A0", "step", None).unwrap() {
            bus.halt_and_settle(&[node_id.clone()]).unwrap();
            assert_eq!(bus.kernel.markets[&node_id].resolved, Some(true));
        }
    }

    #[test]
    fn test_bus_ledger_integrity() {
        let mut bus = make_bus();
        bus.append("A0", "step 1", None).unwrap();
        bus.append("A1", "step 2", None).unwrap();
        assert!(bus.ledger.verify().is_ok());
        assert!(bus.ledger.len() >= 3); // RunStart + 2 appends
    }

    #[test]
    fn test_bus_graveyard_feedback() {
        // Step-B v3: default recent_rejections() returns TopKClasses-abstracted labels.
        // Raw "Forbidden pattern: ..." strings are normalized to "veto:forbidden" via bus_classify.
        let mut bus = make_bus();
        bus.append("A0", "this is FORBIDDEN content", None).unwrap();
        // TopKClasses default: returns "label(count)"
        let rejections = bus.recent_rejections("A0", 5);
        assert_eq!(rejections.len(), 1);
        assert!(
            rejections[0].contains("veto:forbidden"),
            "expected abstracted class label, got: {:?}", rejections[0],
        );
        // Per-author scope still returns raw labels without count
        let per_author = bus.recent_rejections_scoped(
            "A0", 5, crate::bus::RejectionScope::PerAuthor,
        );
        assert_eq!(per_author, vec!["veto:forbidden".to_string()]);
    }

    #[test]
    fn test_bus_classify_bounded() {
        // Invariant: bus_classify never returns unbounded text.
        assert_eq!(TuringBus::bus_classify("Forbidden pattern: decide"), "veto:forbidden");
        assert_eq!(TuringBus::bus_classify("Payload too long: 9999 > 1000"), "veto:size");
        assert_eq!(TuringBus::bus_classify("Too many lines: 50 > 18"), "veto:lines");
        assert_eq!(TuringBus::bus_classify("err:tactic_linarith"), "err:tactic_linarith");
        assert_eq!(TuringBus::bus_classify("err:unknown_variant_we_dont_track"), "err:other");
        assert_eq!(TuringBus::bus_classify("some unprecedented garbage"), "err:other");
    }

    #[test]
    fn test_bus_snapshot() {
        let mut bus = make_bus();
        bus.append("A0", "step 1", None).unwrap();
        let snap = bus.snapshot();
        assert_eq!(snap.tx_count, 1);
        assert!(!snap.markets.is_empty());
    }

    #[test]
    fn test_bus_serial_ordering() {
        // V3L-11: tx_count must increment monotonically
        let mut bus = make_bus();
        for i in 0..5 {
            bus.append("A0", &format!("step {}", i), None).unwrap();
        }
        assert_eq!(bus.tx_count, 5);
        assert_eq!(bus.clock, 5);
    }
}

```

---

Now give your INDEPENDENT round-4 audit. Round-4 expects PASS unless v1.2 introduced regressions or missed a closure.

codex
I’ll audit against the supplied v1.2 text and the checked-out source where it helps validate type claims. I’m first locating the spec file so I can cite exact local line numbers instead of approximating from the prompt.
exec
/bin/bash -lc "rg --files -g 'AGENTS.md' -g 'CO1_7_EXTRA*' -g '*CO1_7_EXTRA*'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/audits/CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md
handover/audits/CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md
handover/audits/GEMINI_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md
handover/audits/CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md
handover/audits/CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md
handover/audits/GEMINI_CO1_7_EXTRA_ROUND4_AUDIT_2026-04-29.md
handover/audits/CODEX_CO1_7_EXTRA_ROUND4_AUDIT_2026-04-29.md
handover/audits/GEMINI_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md

exec
/bin/bash -lc 'pwd && git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4
 M rules/enforcement.log
?? handover/audits/CODEX_CO1_7_EXTRA_ROUND4_AUDIT_2026-04-29.md
?? handover/audits/GEMINI_CO1_7_EXTRA_ROUND4_AUDIT_2026-04-29.md
?? handover/audits/run_codex_co1_7_extra_round4_audit.sh
?? handover/audits/run_gemini_co1_7_extra_round4_audit.py

codex
The repo has a local v1.2 spec file and existing round-4 audit artifacts. I’ll avoid touching the dirty/untracked files and use read-only checks for the spec/source claims.
exec
/bin/bash -lc "nl -ba handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md | sed -n '1,460p'" in /home/zephryj/projects/turingosv4
 succeeded in 1ms:
     1	# CO1.7-extra: L4 head_t close + Sequencer entry-point wiring v1.2 (post round-3 audit patches)
     2	
     3	**Status**: v1.2 DRAFT (2026-04-29; post round-3 dual external audit on v1.1 at HEAD `a3952cf`). Round-3 returned CHALLENGE/PASS (Codex/Gemini); conservative-merged CHALLENGE. v1.2 applies 4 mechanical patches (B1-B4 per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md`). Awaiting round-4 dual external audit.
     4	**Author**: ArchitectAI (Claude); session 2026-04-29.
     5	**Supersedes**: prior bundled `CO1_7_5_TRANSITION_BODIES_AND_RUNTIME_WIRING_v1_2026-04-29.md` (committed `334111a`; round-1 CHALLENGE/CHALLENGE; preserved in git history).
     6	**Pre-implementation gate**: PASS/PASS dual external audit before any code lands. Per CLAUDE.md "Audit Standard".
     7	
     8	**Companion specs (frozen, read first)**:
     9	- `CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` v1.2 — round-3 PASS/PASS; freezes `LedgerWriter` trait + Sequencer 9-stage apply_one + `Git2LedgerWriter::head_commit_oid()`.
    10	- `CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — frozen 7-variant TypedTx; not directly touched here.
    11	- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — referenced for K3 v1.2 supersession authority only; transition bodies are out of scope for this atom.
    12	- `handover/audits/CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md` — round-1 merged verdict that drove this scope split.
    13	
    14	**Single sentence**: close the G-1 carry-forward `q.head_t = NodeId(commit_oid_hex)` after `Git2LedgerWriter.commit`, perform a single-file STEP_B ceremony adding a Sequencer entry-point on TuringBus (Kernel UNTOUCHED), and ship substrate-independent tests — leaving transition function bodies + replay byte-identity to a future CO1.7.5 atom that depends on the Wave-2 substrate (CO P2.x family).
    15	
    16	---
    17	
    18	## § 0 Scope decision (round-1 driven)
    19	
    20	### 0.1 Why this atom exists (Occam-driven scope split)
    21	
    22	Round-1 dual external audit on the prior bundled CO1.7.5 v1 spec (`334111a`) returned CHALLENGE/CHALLENGE. The conservative-merged verdict (`CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md`) found that the v1 bundling crossed Anti-Oreo three-layer boundaries: D1 transition bodies require FC1 top-white predicate execution methods + FC2 middle-black state schemas that don't exist in shipped code (CO P2.x family substrate not yet shipped per `PROJECT_DECISION_MAP § 3.4`).
    23	
    24	Per "无损压缩即智能" + Anti-Oreo + Occam:
    25	
    26	| Atom | Owns | Substrate dependency | Ships when |
    27	|---|---|---|---|
    28	| **CO1.7-extra (THIS spec)** | D2 head_t close + D3 Sequencer entry-point wiring + 1 substrate-independent test | None — uses only frozen `LedgerWriter` trait + `Git2LedgerWriter::head_commit_oid()` + existing `CasStore::put`/`get` | Now (post-PASS/PASS) |
    29	| **CO1.7.5 (future; restored to CO1.7 § 13 original meaning)** | D1 transition bodies (7) + 3 D4 tests + un-ignore `sequencer_serial_replay_byte_identity` | CO P2.1 / P2.2 / P2.3 / P2.5 / P2.6 / P2.7 / P2.9 + CO1.11 + (new) PredicateRegistry execution-methods atom | After substrate atoms reach individual PASS/PASS |
    30	
    31	The split uses the `CO1.4-extra` precedent (small bridge atom alongside larger primary atom). Zero new architectural concepts introduced.
    32	
    33	### 0.2 What this atom inherits (frozen)
    34	
    35	| Frozen by | Surface |
    36	|---|---|
    37	| CO1.7-impl A1 (commit `2461fe6`) | `LedgerEntry` 9-field signing surface + `Git2LedgerWriter` + `InMemoryLedgerWriter` + `head_commit_oid()` accessor |
    38	| CO1.7-impl A2 (commit `2461fe6`) | `Sequencer` 9-stage `apply_one` + `dispatch_transition` exhaustive match (variants stay `Err(NotYetImplemented)` post-CO1.7-extra; D1 transition bodies are out of scope) |
    39	| CO1.4-extra (commit `b6b7574`) | CAS sidecar JSONL index persistence (substrate for the cas_payload_round_trip test) |
    40	
    41	### 0.3 What this atom delivers (new)
    42	
    43	1. **D2** — `q.head_t = state::q_state::NodeId(commit_oid_hex)` after `writer.commit(&entry)` returns Ok; adds 1 trait method `LedgerWriter::head_commit_oid_hex` with mandatory-override design pattern (Q1 synthesis from round-1).
    44	2. **D3** — Single-file STEP_B ceremony adds `Option<Arc<Sequencer>>` field + `with_sequencer` constructor + `submit_typed_tx` forwarder method to `TuringBus` (note: type is `TuringBus`, not `Bus`, per `src/bus.rs:53`). Sequencer lives in TuringBus directly (not nested through Kernel) per round-2 MF4 — Kernel preserves "pure topology" doctrine and stays UNTOUCHED by this atom.
    45	3. **D4-substrate-independent** — One conformance test `tests/cas_payload_round_trip` (`CasStore::put` → `get` round-trip with CID stability post-CO1.4-extra). Other 3 D4 tests (replay state-root + system-signature canonical-message + un-ignore byte-identity) move to future CO1.7.5 atom because they require D1 transition bodies to actually commit.
    46	
    47	### 0.4 Process commitment (active reconciliation per round-1 Gemini MF1+MF3 + Codex Q-A; round-2 MF1 corrected)
    48	
    49	Two STATE_TRANSITION_SPEC § 3 supersessions previously declared in the prior bundled CO1.7.5 v1 spec divide differently across the new atom split (round-2 MF1 corrected the v1 wording):
    50	
    51	| Supersession | Authority chain | Disposition |
    52	|---|---|---|
    53	| **head_t = NodeId(commit_oid_hex) NOT NodeId::from_state_root** (CO1.7 K3 v1.2 round-3 PASS/PASS supersedes STATE v1.4 § 3 line 412) | CO1.7 v1.2 § 5 K3 | **Enacted in CO1.7-extra D2** (§ 1.1 below); takes effect at first apply_one commit |
    54	| **SignalBundle 4-variant SignalKind suffices** (CO1.1.4-pre1 supersedes STATE v1.4 § 3 BoolSignal/StatSignal richness) | CO1.1.4-pre1 § 7.2 | **Migrates to future CO1.7.5** (transition bodies); takes effect when D1 ships |
    55	
    56	**Asserted authority principle** (strengthened per round-1 Gemini MF3): a later, more specific, audited spec (CO1.7 v1.2 round-3 PASS/PASS; CO1.1.4-pre1 PASS/PASS) **legitimately supersedes** earlier general specs (STATE v1.4 round-4 PASS/PASS) within the layered boundary the later spec covers. This is consistent with the project's atom-decomposition pattern: each atom locks its own surface; downstream atoms refine via PASS/PASS audit, not by editing upstream artifacts.
    57	
    58	**Institutional debt acknowledged** (per round-1 Gemini MF1): as part of CO1.7-extra atom closure, ArchitectAI commits to filing a STATE_TRANSITION_SPEC v1.5 housekeeping issue (one paragraph noting both supersessions with backlinks) — NOT a re-audit, just an annotation pass that prevents future readers from being confused by the historical drafting language. Tracked in the § 9 awaiting list.
    59	
    60	---
    61	
    62	## § 1 D2 — head_t close
    63	
    64	### 1.1 Code change
    65	
    66	The D2 logic is extracted into a small helper `advance_head_t(q, writer)` callable from `apply_one` stage 9 AND directly testable by the new `tests/co1_7_extra_head_t_advancement.rs` integration test (round-2 MF2 closure). Helper extraction adds zero behavior change — `apply_one` still executes identical logic.
    67	
    68	```rust
    69	// src/state/sequencer.rs (NEW pub(crate) helper)
    70	/// Closes G-1 head_t carry-forward (Art 0.4 alignment per CO1.7 K3 v1.2).
    71	/// Best-effort head binding: when writer surfaces a commit OID (Git2LedgerWriter
    72	/// always; future writers may), advance head_t. When writer returns None
    73	/// (InMemoryLedgerWriter), leave head_t unchanged (no-op preservation).
    74	///
    75	/// Called from apply_one stage 9 AFTER writer.commit succeeds. Pure function
    76	/// (writer is &dyn so behavior depends only on writer's head_commit_oid_hex
    77	/// return + q's prior state).
    78	///
    79	/// TRACE_MATRIX § 5 — L4 sequencer post-commit head_t wiring (Art 0.4).
    80	///
    81	/// **Visibility** (round-3 B2): `pub` (NOT `pub(crate)`) so that flat
    82	/// integration tests under `tests/co1_7_extra_*.rs` can call this helper
    83	/// directly; needed for round-2 MF2 closure.
    84	pub fn advance_head_t(q: &mut QState, writer: &dyn LedgerWriter) {
    85	    if let Some(commit_oid_hex) = writer.head_commit_oid_hex() {
    86	        q.head_t = crate::state::q_state::NodeId(commit_oid_hex);
    87	    }
    88	}
    89	```
    90	
    91	```rust
    92	// src/state/sequencer.rs::apply_one stage 9 (currently lines 362-373; v1.1 patch)
    93	let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
    94	let mut writer_w = self.ledger_writer.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
    95	writer_w.commit(&entry)?;
    96	self.next_logical_t.store(logical_t, Ordering::SeqCst);
    97	*q_w = q_next;
    98	q_w.ledger_root_t = entry.resulting_ledger_root;
    99	// NEW (CO1.7-extra D2): close G-1 head_t carry-forward.
   100	// (round-3 B1 fix: single deref of RwLockWriteGuard<dyn LedgerWriter> →
   101	// &dyn LedgerWriter; double deref does not work on dyn.)
   102	advance_head_t(&mut *q_w, &*writer_w);
   103	```
   104	
   105	**Stale comments must be updated** (round-2 MF8 — Codex Q-8 finding): `src/state/sequencer.rs:180-184` + `:359-361` currently say "head_t mutation deferred to CO1.7.5+". CO1.7-extra implementation MUST update these comments to reflect "head_t closed by CO1.7-extra D2 via `advance_head_t` helper". Added to § 9 atom landing checklist.
   106	
   107	**NodeId disambiguation**: two `NodeId` types coexist — legacy `pub type NodeId = String` at `src/ledger.rs:13` (imported by TuringBus + Kernel for the legacy ledger event API) and new `pub struct NodeId(pub String)` at `src/state/q_state.rs:49`. `q.head_t` is typed as the new tuple-struct (`q_state.rs:311`); D2 constructs the new variant exclusively (legacy String alias is unused here).
   108	
   109	**Atomicity** (per Codex Q-B + round-2 MF9 wording correction): under acquired `q_w` + `writer_w` write locks, after `writer_w.commit(&entry)?` returns `Ok`, the remaining operations are an `AtomicU64::store` (infallible), a plain `*q_w = q_next` move (infallible), and `advance_head_t` (infallible). For writers whose `head_commit_oid_hex` returns `Some` (Git2LedgerWriter), this is a **post-commit non-failing best-effort head binding** — `q.head_t` advances atomically with `ledger_root_t` and `next_logical_t`. For writers returning `None` (InMemoryLedgerWriter), `advance_head_t` is **explicit no-op preservation** — `q.head_t` stays at its prior value (which equals `q_next.head_t` after the `*q_w = q_next` move because CO1.7 K3 v1.2 forbids transition bodies from mutating head_t).
   110	
   111	### 1.2 Trait method addition (round-2 MF3: REQUIRED, no default impl)
   112	
   113	`LedgerWriter` trait at `src/bottom_white/ledger/transition_ledger.rs` gains one **required** method (round-2 audits both converged on third option per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md` MF3):
   114	
   115	```rust
   116	pub trait LedgerWriter: Send + Sync {
   117	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
   118	    fn len(&self) -> u64;
   119	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;  // (existing; spec preserves)
   120	
   121	    /// NEW (CO1.7-extra D2): canonical 40-char lowercase hex commit OID of the
   122	    /// most recent appended entry, or None if the chain is empty / backend has
   123	    /// no commit-OID notion.
   124	    ///
   125	    /// **REQUIRED** (no default impl): Rust compiler enforces every
   126	    /// LedgerWriter implementation declares this method. This is the round-2
   127	    /// MF3 closure — both audits' safety arguments satisfied:
   128	    /// - **silent stagnation prevention** (Gemini r1+r2): impossible to inherit
   129	    ///   a default that silently leaves head_t stale; a missing impl is a
   130	    ///   compile error.
   131	    /// - **post-commit no-panic** (Codex r1): impl is free to return None at
   132	    ///   runtime if the backend has no OID notion; no panic risk.
   133	    fn head_commit_oid_hex(&self) -> Option<String>;
   134	}
   135	
   136	impl LedgerWriter for Git2LedgerWriter {
   137	    fn head_commit_oid_hex(&self) -> Option<String> {
   138	        self.head_commit_oid().map(|oid| oid.to_string())
   139	    }
   140	    // ... existing commit / len / read_at ...
   141	}
   142	
   143	impl LedgerWriter for InMemoryLedgerWriter {
   144	    /// InMemory has no git substrate → always None. Required by the trait
   145	    /// (no default to inherit) so the choice is explicit, not implicit.
   146	    fn head_commit_oid_hex(&self) -> Option<String> {
   147	        None
   148	    }
   149	    // ... existing ...
   150	}
   151	```
   152	
   153	This is a **breaking change** to any third-party `LedgerWriter` impl outside the workspace (would no longer compile). Inside the workspace, only Git2LedgerWriter and InMemoryLedgerWriter implement the trait; both get explicit declarations above. Forward-compat: any future LedgerWriter impl is forced to declare its OID semantics explicitly — a desirable property for a constitutional anchor field.
   154	
   155	---
   156	
   157	## § 2 D3 — Single-file STEP_B ceremony for TuringBus Sequencer entry-point
   158	
   159	### 2.1 Code change (round-2 MF4: Sequencer placement TuringBus, NOT Kernel)
   160	
   161	Round-2 Codex Q-7 + Gemini Q5 converged on placing Sequencer at TuringBus directly (not nested through Kernel). Rationale per round-2 MF4:
   162	- TuringBus already owns runtime orchestration (`src/bus.rs:53` + per CO1.7-impl). Sequencer is a runtime-orchestration peer of Kernel, not nested inside it.
   163	- Kernel `src/kernel.rs:5-6` has explicit warning against domain-specific terms; the documented "pure topology" role (`:15-17`) is preserved by NOT adding state-driver fields.
   164	- STEP_B Phase 0 less-invasive-alternative test: TuringBus-only is strictly simpler than TuringBus + Kernel coupled changes.
   165	
   166	`src/bus.rs` (note: actual struct name is **`TuringBus`** at `src/bus.rs:53`, NOT `Bus`):
   167	
   168	```rust
   169	// src/bus.rs (additive — TuringBus gets one field + one constructor variant + one method)
   170	pub struct TuringBus {
   171	    // ... existing fields including kernel: Kernel ...
   172	
   173	    /// NEW (CO1.7-extra D3): typed-tx Sequencer; None when bus runs in legacy
   174	    /// ledger-only mode (preserves back-compat with all existing tests).
   175	    /// Marked serde-skip if TuringBus has serde derives (Sequencer holds
   176	    /// Arc-locked runtime state that isn't serializable Q_t data).
   177	    // `#[serde(skip)]` applied IFF TuringBus has Serialize/Deserialize
   178	    // derives at implementation time. Current `pub struct TuringBus` at
   179	    // src/bus.rs:53 has NO serde derives, so the attribute is omitted at
   180	    // first landing. If a future atom adds serde derives to TuringBus, the
   181	    // skip attribute MUST be added in the same patch (the Sequencer's
   182	    // Arc-locked runtime state is not serializable Q_t data).
   183	    pub sequencer: Option<Arc<Sequencer>>,
   184	}
   185	
   186	impl TuringBus {
   187	    pub fn new(kernel: Kernel, config: BusConfig) -> Self {
   188	        Self { /* ...existing..., */ sequencer: None }
   189	    }
   190	
   191	    /// NEW: opt-in constructor that wires a typed-tx Sequencer alongside the legacy ledger.
   192	    pub fn with_sequencer(kernel: Kernel, config: BusConfig, sequencer: Arc<Sequencer>) -> Self {
   193	        Self { /* ...existing..., */ sequencer: Some(sequencer) }
   194	    }
   195	
   196	    /// NEW (CO1.7-extra D3): typed-tx submission path. Returns receipt
   197	    /// (submit_id) immediately; commit happens asynchronously in
   198	    /// Sequencer::run driver loop.
   199	    pub async fn submit_typed_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
   200	        match self.sequencer.as_ref() {
   201	            Some(seq) => seq.submit(tx).await,
   202	            None => Err(SubmitError::QueueClosed),
   203	        }
   204	    }
   205	}
   206	```
   207	
   208	`src/kernel.rs`: **UNTOUCHED** by CO1.7-extra. "Pure topology" doctrine preserved.
   209	
   210	`src/state/sequencer.rs` (round-2 MF6: manual Debug impl, NOT derive — Sequencer holds `Arc<Ed25519Keypair>` at line 199 and `Ed25519Keypair` intentionally has no Debug derive at `src/bottom_white/ledger/system_keypair.rs:282-284`; blanket derive fails to compile):
   211	
   212	```rust
   213	// src/state/sequencer.rs (additive — manual Debug impl for TuringBus.Debug propagation through Arc<Sequencer>)
   214	impl std::fmt::Debug for Sequencer {
   215	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   216	        // finish_non_exhaustive() — leaks no keypair / QState / CAS contents;
   217	        // satisfies Debug requirements for Arc<Sequencer> propagation.
   218	        f.debug_struct("Sequencer").finish_non_exhaustive()
   219	    }
   220	}
   221	```
   222	
   223	### 2.2 Single-file STEP_B ceremony (round-2 MF4 simplification)
   224	
   225	CO1.7-extra now touches a single STEP_B-restricted file: `src/bus.rs`. No combined-ceremony justification needed. Per `STEP_B_PROTOCOL.md` Phase 0, the change is "minimum sufficient version" and has no less-invasive alternative (the typed-tx submission path needs SOME entry-point in the runtime layer; TuringBus is the canonical orchestrator).
   226	
   227	**Ceremony procedure**:
   228	1. Branch A (`step-b-co1.7-extra-A`): edits `src/bus.rs` per § 2.1 (1 field + 1 constructor variant + 1 forwarder method). Also adds the manual `Debug` impl on `Sequencer` in `src/state/sequencer.rs` (NOT STEP_B-restricted; lands alongside for compile coherence).
   229	2. Branch B (`step-b-co1.7-extra-B`): independently re-derives the same edits from this spec (separate session / context).
   230	3. Byte-identity comparison: `diff src/bus.rs` between A and B. Identical → merge to `main`. Divergent → re-do with stricter spec.
   231	
   232	### 2.3 Forward-compat note (round-2 Gemini Q5 partial response)
   233	
   234	Gemini Q5 r2 noted "Kernel placement creates forward-compat hazard of Kernel bloat". The TuringBus placement avoids this hazard entirely — Kernel stays at "pure topology" role; future stateful runtime drivers (e.g., a hypothetical CO1.x event router) would land at TuringBus level alongside Sequencer, which is the natural runtime-orchestrator role for TuringBus to own. No further justification needed beyond Codex Q-7 + Gemini Q5 convergence.
   235	
   236	---
   237	
   238	## § 3 Test plan (substrate-independent; round-2 MF2 + MF5 + MF7 patches)
   239	
   240	Three tests, **flat-named in `tests/`** (round-2 MF5 — Cargo auto-discovery requires flat naming or a `tests/co1_7_extra/main.rs` harness; v1.1 chooses flat naming for simplicity):
   241	
   242	### 3.1 `tests/co1_7_extra_cas_payload_round_trip.rs`
   243	
   244	```rust
   245	//! CO1.7-extra D4: CAS payload round-trip + CID stability across restart.
   246	//! Verifies that CO1.4-extra sidecar persistence makes CasStore content
   247	//! reachable across cold-start, which is a precondition for CO1.7.5
   248	//! FullTransition replay (deferred; gated on substrate atoms).
   249	//! Substrate-independent: uses only CasStore + ObjectType (CO1.4 + CO1.4-extra
   250	//! shipped surfaces); does NOT depend on CO P2.x.
   251	
   252	#[test]
   253	fn cas_payload_round_trip_with_cid_stability_across_restart() {
   254	    let tmp = tempfile::TempDir::new().expect("tempdir");
   255	    let payload = b"co1.7-extra-deterministic-payload-v1";
   256	    let cid_first = {
   257	        let mut cas = CasStore::open(tmp.path()).expect("first open");
   258	        cas.put(payload, ObjectType::ProposalPayload, "test-epoch", 1, Some("CO1.7-extra".into()))
   259	            .expect("put")
   260	    };
   261	    // Drop CasStore handle; reopen (cold-start path).
   262	    let bytes = {
   263	        let cas = CasStore::open(tmp.path()).expect("reopen post-restart");
   264	        cas.get(&cid_first).expect("get post-restart")
   265	    };
   266	    assert_eq!(bytes.as_slice(), payload);
   267	}
   268	```
   269	
   270	### 3.2 `tests/co1_7_extra_git2_writer_head_oid_defense.rs`
   271	
   272	Round-2 MF7: the private module-test helper `entry_at` (at `transition_ledger.rs:813`; Codex r2 misidentified the name as `canonical_test_entry` but substantive finding holds — helper is private and unavailable to integration tests). Integration tests must construct `LedgerEntry` inline.
   273	
   274	```rust
   275	#[test]
   276	fn git2_writer_returns_some_after_commit() {
   277	    let tmp = tempfile::TempDir::new().expect("tempdir");
   278	    let mut writer = Git2LedgerWriter::open(tmp.path()).expect("open");
   279	
   280	    // Inline LedgerEntry construction (round-2 MF7) — uses public CO1.7-impl
   281	    // surfaces only.
   282	    let entry = LedgerEntry {
   283	        logical_t: 1,
   284	        parent_state_root: Hash::ZERO,
   285	        parent_ledger_root: Hash::ZERO,
   286	        tx_kind: TxKind::Work,
   287	        tx_payload_cid: Cid([0u8; 32]),
   288	        resulting_state_root: Hash::ZERO,
   289	        resulting_ledger_root: Hash([1u8; 32]),
   290	        timestamp_logical: 1,
   291	        epoch: SystemEpoch::new(1),
   292	        extensions: Default::default(),
   293	        system_signature: SystemSignature::from_bytes([0u8; 64]),
   294	    };
   295	
   296	    writer.commit(&entry).expect("commit");
   297	    // Defensive against silent head_t stagnation: if Git2LedgerWriter ever
   298	    // inherits a default behavior (impossible given round-2 MF3 — trait method
   299	    // is now required), this catches it. Belt-and-suspenders for the
   300	    // constitutional anchor.
   301	    assert!(
   302	        writer.head_commit_oid_hex().is_some(),
   303	        "Git2LedgerWriter MUST return Some after commit; constitutional anchor violation otherwise"
   304	    );
   305	}
   306	```
   307	
   308	### 3.3 `tests/co1_7_extra_sequencer_head_t_advancement.rs` (NEW — round-2 MF2 closure)
   309	
   310	Tests the actual D2 code path via the `advance_head_t` helper extraction:
   311	
   312	```rust
   313	//! CO1.7-extra D2: verifies advance_head_t correctly advances q.head_t
   314	//! when writer surfaces a commit OID, and preserves q.head_t when writer
   315	//! returns None. Substrate-independent: uses only LedgerWriter trait + QState.
   316	//! Closes round-2 MF2 (D2 code path was untested in v1).
   317	
   318	use std::sync::Mutex;
   319	
   320	/// Mock LedgerWriter that returns a configurable head_commit_oid_hex value.
   321	/// Stubs commit() to always succeed (returns dummy Hash).
   322	struct MockLedgerWriter {
   323	    head_oid: Mutex<Option<String>>,
   324	    len: u64,
   325	}
   326	
   327	impl LedgerWriter for MockLedgerWriter {
   328	    fn commit(&mut self, _entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
   329	        self.len += 1;
   330	        Ok(Hash([0xAB; 32]))
   331	    }
   332	    fn len(&self) -> u64 { self.len }
   333	    fn read_at(&self, _: u64) -> Result<LedgerEntry, LedgerWriterError> {
   334	        unimplemented!("test mock")
   335	    }
   336	    fn head_commit_oid_hex(&self) -> Option<String> {
   337	        self.head_oid.lock().expect("lock").clone()
   338	    }
   339	}
   340	
   341	#[test]
   342	fn advance_head_t_writes_node_id_when_writer_returns_some() {
   343	    let writer = MockLedgerWriter {
   344	        head_oid: Mutex::new(Some("a".repeat(40))),  // 40-hex literal
   345	        len: 0,
   346	    };
   347	    let mut q = QState::genesis();
   348	    let q_initial_head = q.head_t.clone();
   349	
   350	    turingosv4::state::sequencer::advance_head_t(&mut q, &writer);
   351	
   352	    // Post-condition: q.head_t = NodeId("aaaa...aaaa")
   353	    assert_eq!(q.head_t.0, "a".repeat(40));
   354	    assert_ne!(q.head_t, q_initial_head);
   355	}
   356	
   357	#[test]
   358	fn advance_head_t_preserves_node_id_when_writer_returns_none() {
   359	    let writer = MockLedgerWriter {
   360	        head_oid: Mutex::new(None),
   361	        len: 0,
   362	    };
   363	    let mut q = QState::genesis();
   364	    let q_initial_head = q.head_t.clone();
   365	
   366	    turingosv4::state::sequencer::advance_head_t(&mut q, &writer);
   367	
   368	    // Post-condition: q.head_t unchanged (no-op preservation per § 1.1).
   369	    assert_eq!(q.head_t, q_initial_head);
   370	}
   371	```
   372	
   373	Total: 3 tests across 3 flat-named integration test files.
   374	
   375	---
   376	
   377	## § 4 Out of scope (explicitly deferred)
   378	
   379	1. **D1 transition function bodies (7)** — moved to future CO1.7.5 atom; gated on CO P2.x substrate atoms (§ 0.1 table).
   380	2. **3 of 4 D4 tests** (`replay_full_transition_state_root`, `system_signature_verifies_via_canonical_message`, un-ignore `sequencer_serial_replay_byte_identity`) — all require D1 to actually commit; deferred with D1 to future CO1.7.5.
   381	3. **TransitionError 22-variant mapping table** — was over-claimed in prior bundled v1 (Codex Q-E); deferred with D1 to future CO1.7.5 spec.
   382	4. **RejectedAttemptSummary side-channel substantiation** — was overclaimed (Codex Q-E); deferred to future CO1.7.5 spec where it's actually relevant.
   383	5. **STATE_TRANSITION_SPEC v1.5 housekeeping issue filing** — committed to as a post-CO1.7-extra-PASS/PASS process item (§ 0.4); not gating implementation.
   384	6. **Legacy `src/ledger.rs` retirement** — CO1.1.5 atom; CO1.7-extra leaves the legacy WAL ledger fully running.
   385	7. **Materializer state_root computation** — CO1.8 (L5).
   386	
   387	---
   388	
   389	## § 5 Open questions (0 remain — all closed by round-2 audits)
   390	
   391	| Q | Round-2 resolution |
   392	|---|---|
   393	| Q1 `head_commit_oid_hex` default impl (round-1 open) | **Closed by round-2 MF3** — trait method is REQUIRED (no default); compiler enforces every impl declares (§ 1.2). Both audits' safety arguments satisfied. |
   394	| Q1' Sequencer Debug derive completeness (round-1 surfaced) | **Closed by round-2 MF6** — manual `impl Debug for Sequencer` with `f.debug_struct("Sequencer").finish_non_exhaustive()`; `#[derive(Debug)]` not viable because `Arc<Ed25519Keypair>` field has no Debug derive. Codex Q-5 confirms `finish_non_exhaustive()` leaks no keypair / QState / CAS contents. |
   395	
   396	CO1.7-extra v1.1 has zero open questions — round-3 audit verifies patch correctness only.
   397	
   398	---
   399	
   400	## § 6 Audit gates (round structure)
   401	
   402	| Round | Codex | Gemini | Conservative | Action |
   403	|---|---|---|---|---|
   404	| 1 (on prior bundled v1) | CHALLENGE / High | CHALLENGE / High | **CHALLENGE** | Atom rescoped via Occam scope-split (this v1) + small fixes |
   405	| 2 (on this spec) | ⏳ pending | ⏳ pending | TBD | re-audit on CO1.7-extra v1; 1 round expected (small, focused atom) |
   406	| 3+ if needed | … | … | … | iterate to PASS/PASS |
   407	
   408	**Pre-implementation gate**: spec must reach PASS/PASS before any code in `src/state/sequencer.rs` (D2 helper + apply_one stage 9 patch), `src/bus.rs` (TuringBus field + constructor + forwarder; single-file STEP_B), or `src/bottom_white/ledger/transition_ledger.rs` (trait method + 2 impl declarations) is written. **Kernel UNTOUCHED**. Per CLAUDE.md "Audit Standard".
   409	
   410	---
   411	
   412	## § 7 Estimated scope (round-2 MF10: revised upward)
   413	
   414	- **Spec rounds**: round-2 done (CHALLENGE/CHALLENGE → 10 patches in v1.1); round-3 expected to PASS/PASS (small atom, all r2 issues addressed systematically). Round-3 budget ~$5-10.
   415	- **Implementation scope** (post-PASS/PASS):
   416	  - D2 (head_t close: `advance_head_t` helper + apply_one stage 9 patch + required trait method + 2 impl declarations + stale-comment updates at sequencer.rs:180-184/:359-361): ~40-60 LoC
   417	  - D3 (TuringBus field + with_sequencer constructor + submit_typed_tx forwarder + manual Sequencer Debug impl): ~50-80 LoC across single STEP_B-restricted file (bus.rs) + 1 supporting file (sequencer.rs Debug impl)
   418	  - D4 (3 tests with mock LedgerWriter + inline LedgerEntry fixture): ~120-160 LoC
   419	- **Total atom budget**: ~210-300 LoC (revised up from v1's 150-230 per round-2 MF10 — manual Debug + helper extraction + 3rd test + harness adjustments). **Estimated calendar time**: 1-2 days.
   420	
   421	---
   422	
   423	## § 8 Honest acknowledgements (v1.1)
   424	
   425	1. **Scope split is round-1-driven**, not voluntary. Prior bundled CO1.7.5 v1 spec was found by Codex r1 Q-D/H/I to have heavyweight cross-layer substrate dependencies in D1. v1 reverts CO1.7.5 to its CO1.7 § 13 original meaning (transition bodies; future) and creates CO1.7-extra (this atom) as a new bridge for the substrate-independent wiring.
   426	2. **`head_commit_oid_hex` is a NEW REQUIRED trait method** (no default impl; round-2 MF3). Compiler enforces every LedgerWriter impl declares; both `Git2LedgerWriter` and `InMemoryLedgerWriter` get explicit declarations in § 1.2.
   427	3. **D2 logic is extracted into `advance_head_t` helper** (round-2 MF2 closure). The extraction adds zero behavior change but makes D2 directly testable via mock writer (without injecting dispatch_transition into Sequencer).
   428	4. **TuringBus owns Sequencer directly** (round-2 MF4) — not nested through Kernel. Kernel preserves "pure topology" doctrine (`src/kernel.rs:5-6`+`:15-17`) and stays UNTOUCHED by this atom. STEP_B becomes single-file ceremony on `src/bus.rs`.
   429	5. **Manual Sequencer Debug impl** (round-2 MF6) — `#[derive(Debug)]` fails because `Arc<Ed25519Keypair>` field has no Debug (system_keypair.rs:282-284 intentional); `finish_non_exhaustive()` is the safe replacement (Codex Q-5 confirmed no leak risk).
   430	6. **STATE_TRANSITION_SPEC v1.5 housekeeping issue filing is committed** (§ 0.4) per round-1 Gemini MF1 active-reconciliation requirement. Round-2 confirmed the directionally correct framing; v1.1 corrected the supersession-disposition table (head_t enacted HERE; SignalKind migrates to future CO1.7.5).
   431	7. **Most of CO1.1.4-pre1 ABI lock is irrelevant to this atom** — D1 (the part that uses TypedTx + TransitionError + SignalKind) is out of scope. CO1.7-extra only touches `LedgerWriter` trait + TuringBus wiring + Sequencer Debug impl; ABI lock untouched.
   432	8. **Stale Sequencer comments will be updated** during implementation (round-2 MF8): `src/state/sequencer.rs:180-184` + `:359-361` currently say "head_t deferred to CO1.7.5+"; CO1.7-extra implementation must update them to reflect the new D2 reality.
   433	9. **FC-trace requirements**: the new pub symbols introduced by CO1.7-extra implementation must carry doc-comment `/// TRACE_MATRIX <FC-id>: <role>` backlinks per CLAUDE.md "Alignment Standard". Set: `LedgerWriter::head_commit_oid_hex` + `advance_head_t` helper (→ § 5 L4 sequencer post-commit head_t wiring); `TuringBus.sequencer` field + `TuringBus::with_sequencer` + `TuringBus::submit_typed_tx` (→ § 5.2.1 single-writer entry-point).
   434	
   435	---
   436	
   437	## § 9 Pre-audit smoke test plan (v1.1; round-3 launch)
   438	
   439	Per memory `feedback_smoke_before_batch`. Smoke run before round-3 audit launch, at the v1.1 commit HEAD.
   440	
   441	| # | Claim | Smoke command | Pass criterion |
   442	|---|---|---|---|
   443	| S1 | `Git2LedgerWriter::head_commit_oid()` returns `Option<git2::Oid>` | `grep -A1 'pub fn head_commit_oid' src/bottom_white/ledger/transition_ledger.rs` | matches signature (line 674) |
   444	| S2 | Bus struct is named `TuringBus` | `grep -n 'pub struct TuringBus' src/bus.rs` | one hit at line 53 |
   445	| S3 | Kernel UNTOUCHED by this atom (round-2 MF4) | `grep -n 'use crate::ledger::' src/kernel.rs && grep -L 'sequencer' src/kernel.rs` | legacy ledger import present; no "sequencer" reference (Kernel stays at pure topology) |
   446	| S4 | Sequencer struct exists at sequencer.rs:190 | `grep -n 'pub struct Sequencer' src/state/sequencer.rs` | one hit at line 190 |
   447	| S5 | Ed25519Keypair has no Debug derive (forces manual Sequencer Debug impl per MF6) | `grep -B5 'pub struct Ed25519Keypair' src/bottom_white/ledger/system_keypair.rs` | no `#[derive(Debug` precedes struct line |
   448	| S6 | CasStore exposes `put` + `get` (CO1.4 + CO1.4-extra) | `grep -n 'pub fn put\|pub fn get' src/bottom_white/cas/store.rs` | both present |
   449	| S7 | Wallet (`src/sdk/tools/wallet.rs`) untouched | `grep -c 'transition_ledger\|state::sequencer\|TypedTx' src/sdk/tools/wallet.rs` | 0 hits |
   450	| S8 | QState.head_t is `state::q_state::NodeId` (tuple struct) | `grep -B1 -A1 'pub head_t' src/state/q_state.rs` | type matches |
   451	| S9 | Stale comment locations (round-2 MF8) | `grep -n 'CO1.7.5+\|deferred to CO1.7.5' src/state/sequencer.rs` | hits at lines 180-184 + 359-361 (to be updated by D2 implementation) |
   452	| S10 | `entry_at` (private module-test helper) is private (round-2 MF7) | `sed -n '810,820p' src/bottom_white/ledger/transition_ledger.rs` | `fn entry_at(...)` at line 813 inside `mod tests`; no `pub` qualifier |
   453	| S11 | cargo baseline | `cargo check --workspace && cargo test --workspace --lib` | clean compile + 239 / 0 / 1 ignored |
   454	
   455	---
   456	
   457	**END v1 DRAFT body.**
   458	
   459	## Pre-audit smoke results
   460	

exec
/bin/bash -lc "rg -n '&\\*\\*writer_w|&\\*writer_w|pub\\(crate\\) fn advance_head_t|pub fn advance_head_t|Kernel|serde\\(skip\\)|210-300|200-280|TRACE_MATRIX § 5' handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
14:**Single sentence**: close the G-1 carry-forward `q.head_t = NodeId(commit_oid_hex)` after `Git2LedgerWriter.commit`, perform a single-file STEP_B ceremony adding a Sequencer entry-point on TuringBus (Kernel UNTOUCHED), and ship substrate-independent tests — leaving transition function bodies + replay byte-identity to a future CO1.7.5 atom that depends on the Wave-2 substrate (CO P2.x family).
44:2. **D3** — Single-file STEP_B ceremony adds `Option<Arc<Sequencer>>` field + `with_sequencer` constructor + `submit_typed_tx` forwarder method to `TuringBus` (note: type is `TuringBus`, not `Bus`, per `src/bus.rs:53`). Sequencer lives in TuringBus directly (not nested through Kernel) per round-2 MF4 — Kernel preserves "pure topology" doctrine and stays UNTOUCHED by this atom.
79:/// TRACE_MATRIX § 5 — L4 sequencer post-commit head_t wiring (Art 0.4).
84:pub fn advance_head_t(q: &mut QState, writer: &dyn LedgerWriter) {
102:advance_head_t(&mut *q_w, &*writer_w);
107:**NodeId disambiguation**: two `NodeId` types coexist — legacy `pub type NodeId = String` at `src/ledger.rs:13` (imported by TuringBus + Kernel for the legacy ledger event API) and new `pub struct NodeId(pub String)` at `src/state/q_state.rs:49`. `q.head_t` is typed as the new tuple-struct (`q_state.rs:311`); D2 constructs the new variant exclusively (legacy String alias is unused here).
159:### 2.1 Code change (round-2 MF4: Sequencer placement TuringBus, NOT Kernel)
161:Round-2 Codex Q-7 + Gemini Q5 converged on placing Sequencer at TuringBus directly (not nested through Kernel). Rationale per round-2 MF4:
162:- TuringBus already owns runtime orchestration (`src/bus.rs:53` + per CO1.7-impl). Sequencer is a runtime-orchestration peer of Kernel, not nested inside it.
163:- Kernel `src/kernel.rs:5-6` has explicit warning against domain-specific terms; the documented "pure topology" role (`:15-17`) is preserved by NOT adding state-driver fields.
164:- STEP_B Phase 0 less-invasive-alternative test: TuringBus-only is strictly simpler than TuringBus + Kernel coupled changes.
171:    // ... existing fields including kernel: Kernel ...
177:    // `#[serde(skip)]` applied IFF TuringBus has Serialize/Deserialize
187:    pub fn new(kernel: Kernel, config: BusConfig) -> Self {
192:    pub fn with_sequencer(kernel: Kernel, config: BusConfig, sequencer: Arc<Sequencer>) -> Self {
234:Gemini Q5 r2 noted "Kernel placement creates forward-compat hazard of Kernel bloat". The TuringBus placement avoids this hazard entirely — Kernel stays at "pure topology" role; future stateful runtime drivers (e.g., a hypothetical CO1.x event router) would land at TuringBus level alongside Sequencer, which is the natural runtime-orchestrator role for TuringBus to own. No further justification needed beyond Codex Q-7 + Gemini Q5 convergence.
408:**Pre-implementation gate**: spec must reach PASS/PASS before any code in `src/state/sequencer.rs` (D2 helper + apply_one stage 9 patch), `src/bus.rs` (TuringBus field + constructor + forwarder; single-file STEP_B), or `src/bottom_white/ledger/transition_ledger.rs` (trait method + 2 impl declarations) is written. **Kernel UNTOUCHED**. Per CLAUDE.md "Audit Standard".
419:- **Total atom budget**: ~210-300 LoC (revised up from v1's 150-230 per round-2 MF10 — manual Debug + helper extraction + 3rd test + harness adjustments). **Estimated calendar time**: 1-2 days.
428:4. **TuringBus owns Sequencer directly** (round-2 MF4) — not nested through Kernel. Kernel preserves "pure topology" doctrine (`src/kernel.rs:5-6`+`:15-17`) and stays UNTOUCHED by this atom. STEP_B becomes single-file ceremony on `src/bus.rs`.
445:| S3 | Kernel UNTOUCHED by this atom (round-2 MF4) | `grep -n 'use crate::ledger::' src/kernel.rs && grep -L 'sequencer' src/kernel.rs` | legacy ledger import present; no "sequencer" reference (Kernel stays at pure topology) |
471:| S3 | Kernel UNTOUCHED | `use crate::ledger::{Node, NodeId, Tape, TapeError}` at kernel.rs:8; **0 hits** of "sequencer" anywhere in kernel.rs (pure topology preserved per round-2 MF4) | ✅ PASS |
499:- **MF4** Sequencer placement: TuringBus owns directly (NOT nested through Kernel). Kernel UNTOUCHED. STEP_B becomes single-file ceremony (§ 2.1 + § 2.2).
505:- **MF10** LoC estimate: 150-230 → 210-300 (manual Debug + helper extraction + 3rd test + harness adjustments) (§ 7)
509:- **B1** (Codex Q2): § 1.1 stage-9 snippet `&**writer_w` → `&*writer_w` (single deref of `RwLockWriteGuard<dyn LedgerWriter>` → `&dyn LedgerWriter`; double deref does not work on `dyn`).
510:- **B2** (Codex Q2): § 1.1 helper `pub(crate) fn advance_head_t` → `pub fn advance_head_t` (so flat integration tests under `tests/co1_7_extra_*.rs` per MF5 can call it). Added FC-trace doc-comment.
511:- **B3** (Codex Q4): removed stale Kernel references at preface line 14 (single-sentence summary now reads "single-file STEP_B ceremony adding a Sequencer entry-point on TuringBus (Kernel UNTOUCHED)") + § 6 pre-implementation gate (removed `src/kernel.rs` from gate file list; explicitly noted "Kernel UNTOUCHED").
512:- **B4** (Codex Q5+Q6, non-blocking): § 2.1 `#[serde(skip)]` made conditional with explicit comment ("applied IFF TuringBus has serde derives at implementation time"); § 7 vs patch log LoC sync (patch log "200-280" → "210-300" matches § 7).

exec
/bin/bash -lc "rg -n 'pub struct Sequencer|ledger_writer:|writer_w|pub struct TuringBus|derive\\(|pub fn head_commit_oid|fn head_commit_oid_hex|trait LedgerWriter|pub head_t' src/state/sequencer.rs src/bus.rs src/bottom_white/ledger/transition_ledger.rs src/state/q_state.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/q_state.rs:28:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/q_state.rs:48:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/q_state.rs:64:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/q_state.rs:68:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/q_state.rs:73:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/q_state.rs:82:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:90:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:105:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:117:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/q_state.rs:142:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:156:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:160:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:166:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/q_state.rs:181:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:185:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/q_state.rs:200:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:204:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/q_state.rs:219:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:223:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:229:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/q_state.rs:261:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:265:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:274:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:278:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/q_state.rs:295:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:306:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:311:    pub head_t: NodeId,
src/bottom_white/ledger/transition_ledger.rs:51:#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
src/bottom_white/ledger/transition_ledger.rs:68:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/bottom_white/ledger/transition_ledger.rs:107:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/bottom_white/ledger/transition_ledger.rs:189:pub trait LedgerWriter: Send + Sync {
src/bottom_white/ledger/transition_ledger.rs:195:#[derive(Debug)]
src/bottom_white/ledger/transition_ledger.rs:216:#[derive(Debug, Default)]
src/bottom_white/ledger/transition_ledger.rs:263:#[derive(Debug, Clone, Copy, PartialEq, Eq)]
src/bottom_white/ledger/transition_ledger.rs:269:#[derive(Debug)]
src/bottom_white/ledger/transition_ledger.rs:563:#[derive(Debug)]
src/bottom_white/ledger/transition_ledger.rs:674:    pub fn head_commit_oid(&self) -> Option<git2::Oid> {
src/bus.rs:53:pub struct TuringBus {
src/bus.rs:68:#[derive(Debug, Clone, Copy)]
src/bus.rs:79:#[derive(Debug)]
src/state/sequencer.rs:71:#[derive(Debug, Clone, Copy, PartialEq, Eq)]
src/state/sequencer.rs:76:#[derive(Debug)]
src/state/sequencer.rs:102:#[derive(Debug)]
src/state/sequencer.rs:154:#[derive(Debug)]
src/state/sequencer.rs:190:pub struct Sequencer {
src/state/sequencer.rs:201:    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
src/state/sequencer.rs:217:        ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
src/state/sequencer.rs:364:            let mut writer_w = self
src/state/sequencer.rs:368:            writer_w.commit(&entry)?; // ← may fail; if it does, fetch_add was NOT called

exec
/bin/bash -lc 'find .. -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
 succeeded in 730ms:
../turingos/AGENTS.md
../omega_kepler_discovery/AGENTS.md
../omega/AGENTS.md
../turingosv2/AGENTS.md
../turingclaw/AGENTS.md

codex
The spec file confirms the B1/B2/B3/B4 text is present at the expected places. I’m doing one more pass for residual bad strings and the Rust type coercion claim before writing the verdict.
exec
/bin/bash -lc "nl -ba handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md | sed -n '460,540p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   460	
   461	### Round-2 smoke (HEAD `617f01e`; v1)
   462	
   463	8/8 PASS — see prior commit log. v1 spec sent to round-2 dual external audit on this baseline.
   464	
   465	### Round-3 smoke (HEAD `25564d7`; v1.1 patches commit)
   466	
   467	| # | Claim | Result | Status |
   468	|---|---|---|---|
   469	| S1 | head_commit_oid signature | `pub fn head_commit_oid(&self) -> Option<git2::Oid>` (transition_ledger.rs:674) | ✅ PASS |
   470	| S2 | TuringBus struct | `pub struct TuringBus` at bus.rs:53 | ✅ PASS |
   471	| S3 | Kernel UNTOUCHED | `use crate::ledger::{Node, NodeId, Tape, TapeError}` at kernel.rs:8; **0 hits** of "sequencer" anywhere in kernel.rs (pure topology preserved per round-2 MF4) | ✅ PASS |
   472	| S4 | Sequencer struct line | `pub struct Sequencer` at sequencer.rs:190 | ✅ PASS |
   473	| S5 | Ed25519Keypair has NO Debug derive | `#[derive(Zeroize, ZeroizeOnDrop)]` precedes `pub struct Ed25519Keypair` (system_keypair.rs:282-284); no Debug → forces manual Sequencer Debug impl per MF6 | ✅ PASS |
   474	| S6 | CasStore put + get | `pub fn put` at line 163, `pub fn get` at line 199 | ✅ PASS |
   475	| S7 | wallet untouched | 0 hits in `src/sdk/tools/wallet.rs` | ✅ PASS |
   476	| S8 | head_t type | `pub head_t: NodeId` (q_state.rs:311) — type matches new tuple-struct | ✅ PASS |
   477	| S9 | stale comments confirmed | sequencer.rs:178-184 (doc on apply_one Sequencer) + :357-361 (in apply_one stage 9 inline comment) — both still say "deferred to CO1.7.5+"; will be patched by D2 implementation per atom landing checklist | ✅ PASS (with minor line cite refinement vs v1.1 spec's "180-184 + 359-361" — actual lines 178-184 + 357-361; spec patched in this commit) |
   478	| S10 | private module-test helper exists | `fn entry_at` at transition_ledger.rs:813 inside `mod tests`; no `pub` qualifier (Codex r2 misidentified name as `canonical_test_entry` but substantive finding holds) | ✅ PASS (with helper-name correction) |
   479	| S11 | cargo baseline | check pass; `239 passed; 0 failed; 1 ignored` (the ignored test is `sequencer_serial_replay_byte_identity`, deferred to future CO1.7.5 atom) | ✅ PASS |
   480	
   481	**Smoke gate v1.1**: 11 / 11 PASS at HEAD `25564d7`. Spec v1.1 sent to round-3 dual external audit.
   482	
   483	### Round-4 smoke (v1.2 HEAD; populated at audit launch)
   484	
   485	| # | Status |
   486	|---|---|
   487	| S1-S11 | ⏳ pending (will re-run at v1.2 commit HEAD; expected unchanged from v1.1 since source code did not change between v1.1 and v1.2 — only spec text patches) |
   488	
   489	### Patch log
   490	
   491	**v1 (round-1 scope split; commits `f7fc19f` + `617f01e`)**:
   492	- Scope split per round-1 Codex r1 Q-D/H/I + ArchitectAI Occam decision. D1 + 3 D4 tests + un-ignore migrated to future CO1.7.5 atom (gated on CO P2.x). v1 inherited round-1 fixes M3-M5 + § 0.4 active reconciliation.
   493	
   494	**v1.1 (round-2 driven; this revision)** — 10 patches per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md`:
   495	
   496	- **MF1** § 0.4 supersession-disposition table corrected: head_t supersession **enacted in CO1.7-extra D2**; SignalKind supersession migrates to future CO1.7.5 (was: both migrate)
   497	- **MF2** D2 testability: `advance_head_t(q, writer)` helper extracted from apply_one stage 9 → makes D2 directly testable via mock writer (§ 1.1 + new test § 3.3)
   498	- **MF3** trait method `head_commit_oid_hex` becomes REQUIRED (no default impl); Rust compiler enforces every LedgerWriter declares (§ 1.2). Both audits' safety arguments satisfied via this third-option synthesis.
   499	- **MF4** Sequencer placement: TuringBus owns directly (NOT nested through Kernel). Kernel UNTOUCHED. STEP_B becomes single-file ceremony (§ 2.1 + § 2.2).
   500	- **MF5** test harness: flat-named `tests/co1_7_extra_*.rs` for Cargo auto-discovery (§ 3 file paths)
   501	- **MF6** Sequencer Debug: manual `impl Debug` with `finish_non_exhaustive()` (Ed25519Keypair has no Debug derive at system_keypair.rs:282-284 — blanket derive fails) (§ 2.1)
   502	- **MF7** `entry_at` private → tests construct LedgerEntry inline (§ 3.2)
   503	- **MF8** stale Sequencer comments (sequencer.rs:178-184 + :357-361) added to atom landing checklist (§ 1.1 + § 8 ack #8)
   504	- **MF9** atomicity wording: "post-commit non-failing best-effort head binding (Some path)" + "explicit no-op preservation (None path)" (§ 1.1)
   505	- **MF10** LoC estimate: 150-230 → 210-300 (manual Debug + helper extraction + 3rd test + harness adjustments) (§ 7)
   506	
   507	**v1.2 (round-3 driven; this revision)** — 4 mechanical patches per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md`:
   508	
   509	- **B1** (Codex Q2): § 1.1 stage-9 snippet `&**writer_w` → `&*writer_w` (single deref of `RwLockWriteGuard<dyn LedgerWriter>` → `&dyn LedgerWriter`; double deref does not work on `dyn`).
   510	- **B2** (Codex Q2): § 1.1 helper `pub(crate) fn advance_head_t` → `pub fn advance_head_t` (so flat integration tests under `tests/co1_7_extra_*.rs` per MF5 can call it). Added FC-trace doc-comment.
   511	- **B3** (Codex Q4): removed stale Kernel references at preface line 14 (single-sentence summary now reads "single-file STEP_B ceremony adding a Sequencer entry-point on TuringBus (Kernel UNTOUCHED)") + § 6 pre-implementation gate (removed `src/kernel.rs` from gate file list; explicitly noted "Kernel UNTOUCHED").
   512	- **B4** (Codex Q5+Q6, non-blocking): § 2.1 `#[serde(skip)]` made conditional with explicit comment ("applied IFF TuringBus has serde derives at implementation time"); § 7 vs patch log LoC sync (patch log "200-280" → "210-300" matches § 7).
   513	
   514	**Round-3 Codex/Gemini disagreement summary**: Gemini PASS ("model of post-audit closure"; v1.1 architecturally sound). Codex CHALLENGE (3 concrete patch blockers + 1 non-blocking, all mechanical fixes). Conservative-merged CHALLENGE (per memory `feedback_dual_audit_conflict`); v1.2 patches B1-B4 mechanically; round-4 expected PASS/PASS.
   515	
   516	### Awaiting
   517	
   518	1. round-4 dual external audit on CO1.7-extra v1.2
   519	2. expected PASS/PASS (only mechanical fixes need verification; no architectural surface change since v1.1)
   520	3. then CO1.7-extra-impl (D2 helper extraction + apply_one patch + trait method + TuringBus single-file STEP_B + 3 tests + stale-comment update)
   521	4. file STATE_TRANSITION_SPEC v1.5 housekeeping issue per § 0.4 commitment
   522	5. spec future CO1.7.5 (transition bodies; gated on CO P2.x substrate atoms)

exec
/bin/bash -lc "rg -n '&\\*\\*writer_w|combined STEP_B|TuringBus \\+ Kernel|src/kernel\\.rs \\(field|src/kernel\\.rs\\).*written|200-280|serde\\(skip\\)|pub\\(crate\\) fn advance_head_t|NEW pub\\(crate\\) helper' handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
69:// src/state/sequencer.rs (NEW pub(crate) helper)
107:**NodeId disambiguation**: two `NodeId` types coexist — legacy `pub type NodeId = String` at `src/ledger.rs:13` (imported by TuringBus + Kernel for the legacy ledger event API) and new `pub struct NodeId(pub String)` at `src/state/q_state.rs:49`. `q.head_t` is typed as the new tuple-struct (`q_state.rs:311`); D2 constructs the new variant exclusively (legacy String alias is unused here).
164:- STEP_B Phase 0 less-invasive-alternative test: TuringBus-only is strictly simpler than TuringBus + Kernel coupled changes.
177:    // `#[serde(skip)]` applied IFF TuringBus has Serialize/Deserialize
509:- **B1** (Codex Q2): § 1.1 stage-9 snippet `&**writer_w` → `&*writer_w` (single deref of `RwLockWriteGuard<dyn LedgerWriter>` → `&dyn LedgerWriter`; double deref does not work on `dyn`).
510:- **B2** (Codex Q2): § 1.1 helper `pub(crate) fn advance_head_t` → `pub fn advance_head_t` (so flat integration tests under `tests/co1_7_extra_*.rs` per MF5 can call it). Added FC-trace doc-comment.
512:- **B4** (Codex Q5+Q6, non-blocking): § 2.1 `#[serde(skip)]` made conditional with explicit comment ("applied IFF TuringBus has serde derives at implementation time"); § 7 vs patch log LoC sync (patch log "200-280" → "210-300" matches § 7).

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '190,205p;360,372p' && nl -ba src/bus.rs | sed -n '45,60p' && nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '185,193p;668,676p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   190	pub struct Sequencer {
   191	    /// K1: assigned at submit; never appears in LedgerEntry.
   192	    next_submit_id: AtomicU64,
   193	    /// K1: advances ONLY on commit; first accepted entry gets logical_t=1.
   194	    next_logical_t: AtomicU64,
   195	
   196	    queue_tx: tokio::sync::mpsc::Sender<TypedTx>,
   197	
   198	    cas: Arc<RwLock<CasStore>>,
   199	    keypair: Arc<Ed25519Keypair>,
   200	    epoch: SystemEpoch,
   201	    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
   202	
   203	    predicate_registry: Arc<PredicateRegistry>,
   204	    tool_registry: Arc<ToolRegistry>,
   205	
   360	        // mutation deferred to CO1.7.5+ when Git2LedgerWriter exposes
   361	        // commit_sha alongside Hash). state_root_t comes from q_next as-is.
   362	        {
   363	            let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
   364	            let mut writer_w = self
   365	                .ledger_writer
   366	                .write()
   367	                .map_err(|_| ApplyError::QStateLockPoisoned)?;
   368	            writer_w.commit(&entry)?; // ← may fail; if it does, fetch_add was NOT called
   369	            // commit succeeded → safe to advance counter.
   370	            self.next_logical_t.store(logical_t, Ordering::SeqCst);
   371	            *q_w = q_next;
   372	            q_w.ledger_root_t = entry.resulting_ledger_root;
    45	    }
    46	}
    47	
    48	// ── Core Bus ────────────────────────────────────────────────────
    49	
    50	/// The serial event reactor.
    51	/// V3L-11: ALL state mutations go through this single-threaded reactor.
    52	/// No concurrent access to kernel/markets — causal ordering guaranteed.
    53	pub struct TuringBus {
    54	    pub kernel: Kernel,
    55	    pub ledger: Ledger,
    56	    pub tools: Vec<Box<dyn TuringTool>>,
    57	    pub config: BusConfig,
    58	    pub clock: u64,
    59	    pub tx_count: u64,
    60	    pub generation: u32,
   185	/// Test/skeleton impl is `InMemoryLedgerWriter` below.
   186	///
   187	/// **K4**: signature `commit(&mut self) → Hash` (NOT `&self → NodeId`); `iter_from`
   188	/// deferred to CO1.7.5+ (only used by FullTransition replay; not v1 deliverable).
   189	pub trait LedgerWriter: Send + Sync {
   190	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
   191	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
   192	    fn len(&self) -> u64;
   193	}
   668	        Repository::open(&self.repo_path)
   669	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("repo open: {e}")))
   670	    }
   671	
   672	    /// Commit OID of the most recent appended entry (None if chain is empty).
   673	    /// CO1.7.5+ `head_t` wiring uses this to surface commit_sha alongside Hash.
   674	    pub fn head_commit_oid(&self) -> Option<git2::Oid> {
   675	        self.head_oid
   676	    }

codex
One small stale code-fence label still says “pub(crate)” even though the actual signature is public; I’m treating that as an editorial cleanup note unless it changes implementation semantics. I’m checking module export visibility now so the integration-test access claim is complete.
exec
/bin/bash -lc "nl -ba src/lib.rs | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	pub mod boot;
     2	pub mod ledger;
     3	pub mod prediction_market;
     4	pub mod kernel;
     5	pub mod bus;
     6	pub mod sdk;
     7	pub mod drivers;
     8	pub mod wal;
     9	pub mod economy;
    10	pub mod top_white;
    11	pub mod bottom_white;
    12	pub mod state;

exec
/bin/bash -lc "nl -ba src/state/mod.rs | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TRACE_MATRIX Art 0.1: 四要素映射 (Tape / Input-Tape / Q / State).
     2	//! TRACE_MATRIX Art 0.4: Q_t version-controlled state vector.
     3	//! TRACE_MATRIX WP § 4: 9-component system state Q_t.
     4	//! TRACE_MATRIX WP § 0 axiom 1: state monotonicity.
     5	//!
     6	//! Atom: CO1.2 (Q_t struct) — implements `STATE_TRANSITION_SPEC v1.4 § 1.1`.
     7	//! All public re-exports below are surface for the same TRACE_MATRIX rows.
     8	
     9	/// TRACE_MATRIX Art 0.4 / WP § 4 — Q_t module: implements all 9 system state fields.
    10	pub mod q_state;
    11	
    12	/// TRACE_MATRIX FC2-Submit / CO1.1.4-pre1 — typed-tx ABI surface (TypedTx + per-kind structs).
    13	pub mod typed_tx;
    14	
    15	/// TRACE_MATRIX § 5.2.1 / CO1.7-impl A2+A3 — L4 sequencer + dispatch_transition.
    16	pub mod sequencer;
    17	
    18	pub use q_state::{
    19	    AgentId, AgentSwarmState, AgentVisibleProjection, BalancesIndex, BudgetSnapshot,
    20	    ChallengeCase, ChallengeCasesIndex, ClaimEntry, ClaimsIndex, EconomicState, EscrowEntry,
    21	    EscrowsIndex, Hash, NodeId, PerAgentState, PriceIndex, QState, Reputation, ReputationsIndex,
    22	    RoyaltyEdge, RoyaltyGraph, StakeEntry, StakesIndex, TaskMarketEntry, TaskMarketsIndex, TxId,
    23	};
    24	
    25	pub use typed_tx::{
    26	    AgentSignature, BoolWithProof, ChallengeSigningPayload, ChallengeTx, ClaimId,
    27	    FinalizeRewardSigningPayload, FinalizeRewardTx, HasSubmitter, PredicateId,
    28	    PredicateResultsBundle, ReadKey, RejectionClass, ReuseTx, RunId, RunOutcome,
    29	    SafetyOrCreation, SignalBundle, SignalKind, SlashEvidenceCid, TaskExpireSigningPayload,
    30	    TaskExpireTx, TaskId, TerminalSummarySigningPayload, TerminalSummaryTx, ToolId,
    31	    TransitionError, TxStatus, TypedTx, VerifySigningPayload, VerifyTx, VerifyVerdict,
    32	    WorkSigningPayload, WorkTx, WriteKey,
    33	};

exec
/bin/bash -lc "rg -n 'mod state|pub mod state|mod sequencer|pub mod sequencer|advance_head_t|&\\*\\*writer_w|&\\*writer_w' src tests handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:66:The D2 logic is extracted into a small helper `advance_head_t(q, writer)` callable from `apply_one` stage 9 AND directly testable by the new `tests/co1_7_extra_head_t_advancement.rs` integration test (round-2 MF2 closure). Helper extraction adds zero behavior change — `apply_one` still executes identical logic.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:84:pub fn advance_head_t(q: &mut QState, writer: &dyn LedgerWriter) {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:102:advance_head_t(&mut *q_w, &*writer_w);
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:105:**Stale comments must be updated** (round-2 MF8 — Codex Q-8 finding): `src/state/sequencer.rs:180-184` + `:359-361` currently say "head_t mutation deferred to CO1.7.5+". CO1.7-extra implementation MUST update these comments to reflect "head_t closed by CO1.7-extra D2 via `advance_head_t` helper". Added to § 9 atom landing checklist.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:109:**Atomicity** (per Codex Q-B + round-2 MF9 wording correction): under acquired `q_w` + `writer_w` write locks, after `writer_w.commit(&entry)?` returns `Ok`, the remaining operations are an `AtomicU64::store` (infallible), a plain `*q_w = q_next` move (infallible), and `advance_head_t` (infallible). For writers whose `head_commit_oid_hex` returns `Some` (Git2LedgerWriter), this is a **post-commit non-failing best-effort head binding** — `q.head_t` advances atomically with `ledger_root_t` and `next_logical_t`. For writers returning `None` (InMemoryLedgerWriter), `advance_head_t` is **explicit no-op preservation** — `q.head_t` stays at its prior value (which equals `q_next.head_t` after the `*q_w = q_next` move because CO1.7 K3 v1.2 forbids transition bodies from mutating head_t).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:310:Tests the actual D2 code path via the `advance_head_t` helper extraction:
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:313://! CO1.7-extra D2: verifies advance_head_t correctly advances q.head_t
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:342:fn advance_head_t_writes_node_id_when_writer_returns_some() {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:350:    turingosv4::state::sequencer::advance_head_t(&mut q, &writer);
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:358:fn advance_head_t_preserves_node_id_when_writer_returns_none() {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:366:    turingosv4::state::sequencer::advance_head_t(&mut q, &writer);
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:416:  - D2 (head_t close: `advance_head_t` helper + apply_one stage 9 patch + required trait method + 2 impl declarations + stale-comment updates at sequencer.rs:180-184/:359-361): ~40-60 LoC
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:427:3. **D2 logic is extracted into `advance_head_t` helper** (round-2 MF2 closure). The extraction adds zero behavior change but makes D2 directly testable via mock writer (without injecting dispatch_transition into Sequencer).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:433:9. **FC-trace requirements**: the new pub symbols introduced by CO1.7-extra implementation must carry doc-comment `/// TRACE_MATRIX <FC-id>: <role>` backlinks per CLAUDE.md "Alignment Standard". Set: `LedgerWriter::head_commit_oid_hex` + `advance_head_t` helper (→ § 5 L4 sequencer post-commit head_t wiring); `TuringBus.sequencer` field + `TuringBus::with_sequencer` + `TuringBus::submit_typed_tx` (→ § 5.2.1 single-writer entry-point).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:497:- **MF2** D2 testability: `advance_head_t(q, writer)` helper extracted from apply_one stage 9 → makes D2 directly testable via mock writer (§ 1.1 + new test § 3.3)
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:509:- **B1** (Codex Q2): § 1.1 stage-9 snippet `&**writer_w` → `&*writer_w` (single deref of `RwLockWriteGuard<dyn LedgerWriter>` → `&dyn LedgerWriter`; double deref does not work on `dyn`).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:510:- **B2** (Codex Q2): § 1.1 helper `pub(crate) fn advance_head_t` → `pub fn advance_head_t` (so flat integration tests under `tests/co1_7_extra_*.rs` per MF5 can call it). Added FC-trace doc-comment.
src/lib.rs:12:pub mod state;
src/state/mod.rs:16:pub mod sequencer;

codex
# Codex CO1.7-extra Round-4 Audit

## Q1 B1 closure (stage-9 deref)
PASS. Active §1.1 stage-9 snippet uses `advance_head_t(&mut *q_w, &*writer_w);` at spec line 102. Given `ledger_writer: Arc<RwLock<dyn LedgerWriter>>` in source line 201 and the write guard at lines 364-367, `&*writer_w` produces `&dyn LedgerWriter`, matching `advance_head_t(q: &mut QState, writer: &dyn LedgerWriter)` at spec line 84.

Only residual `&**writer_w` is historical patch-log provenance at spec line 509, not an active code snippet.

## Q2 B2 closure (advance_head_t pub)
PASS. Spec line 84 declares `pub fn advance_head_t`, and the integration-test calls at lines 350 and 366 are reachable because `state` and `state::sequencer` are public modules (`src/lib.rs:12`, `src/state/mod.rs:16`). FC trace is present at spec line 79.

No B2 over-exposure found. Minor editorial residue: line 69 still labels the snippet `NEW pub(crate) helper`, but lines 81-84 unambiguously require `pub`.

## Q3 B3 closure (stale Kernel refs)
PASS. Preface line 14 now says the required TuringBus-only single-file STEP_B wording with “Kernel UNTOUCHED”. §6 pre-implementation gate line 408 lists only `src/state/sequencer.rs`, `src/bus.rs`, and `src/bottom_white/ledger/transition_ledger.rs`, then explicitly says `Kernel UNTOUCHED`.

Residual Kernel mentions are contextual or protective, not stale touch-surface claims.

## Q4 B4 closure (serde-skip conditional + LoC sync)
PASS. §2.1 lines 175-182 make `#[serde(skip)]` conditional and state it is omitted at first landing because current `TuringBus` has no serde derives. Source confirms `TuringBus` has no derive at `src/bus.rs:53`.

LoC is synced: §7 line 419 says `~210-300`; patch log lines 505 and 512 also say `210-300`. No divergence.

## Q5 New defects in v1.2
No blocking new defects found.

Non-blocking editorial nits:
- Spec line 69 still says `NEW pub(crate) helper`; should be cleaned to avoid visual contradiction.
- Spec line 66 names `tests/co1_7_extra_head_t_advancement.rs`, while §3.3 line 308 uses `tests/co1_7_extra_sequencer_head_t_advancement.rs`.

## **VERDICT**: PASS

## Top issues (if CHALLENGE)
N/A.

## Conviction (low/med/high)
High.
2026-04-29T03:42:25.033794Z ERROR codex_core::session: failed to record rollout items: thread 019dd752-7242-72c1-b45f-4f01e1b11f55 not found
tokens used
30,269
# Codex CO1.7-extra Round-4 Audit

## Q1 B1 closure (stage-9 deref)
PASS. Active §1.1 stage-9 snippet uses `advance_head_t(&mut *q_w, &*writer_w);` at spec line 102. Given `ledger_writer: Arc<RwLock<dyn LedgerWriter>>` in source line 201 and the write guard at lines 364-367, `&*writer_w` produces `&dyn LedgerWriter`, matching `advance_head_t(q: &mut QState, writer: &dyn LedgerWriter)` at spec line 84.

Only residual `&**writer_w` is historical patch-log provenance at spec line 509, not an active code snippet.

## Q2 B2 closure (advance_head_t pub)
PASS. Spec line 84 declares `pub fn advance_head_t`, and the integration-test calls at lines 350 and 366 are reachable because `state` and `state::sequencer` are public modules (`src/lib.rs:12`, `src/state/mod.rs:16`). FC trace is present at spec line 79.

No B2 over-exposure found. Minor editorial residue: line 69 still labels the snippet `NEW pub(crate) helper`, but lines 81-84 unambiguously require `pub`.

## Q3 B3 closure (stale Kernel refs)
PASS. Preface line 14 now says the required TuringBus-only single-file STEP_B wording with “Kernel UNTOUCHED”. §6 pre-implementation gate line 408 lists only `src/state/sequencer.rs`, `src/bus.rs`, and `src/bottom_white/ledger/transition_ledger.rs`, then explicitly says `Kernel UNTOUCHED`.

Residual Kernel mentions are contextual or protective, not stale touch-surface claims.

## Q4 B4 closure (serde-skip conditional + LoC sync)
PASS. §2.1 lines 175-182 make `#[serde(skip)]` conditional and state it is omitted at first landing because current `TuringBus` has no serde derives. Source confirms `TuringBus` has no derive at `src/bus.rs:53`.

LoC is synced: §7 line 419 says `~210-300`; patch log lines 505 and 512 also say `210-300`. No divergence.

## Q5 New defects in v1.2
No blocking new defects found.

Non-blocking editorial nits:
- Spec line 69 still says `NEW pub(crate) helper`; should be cleaned to avoid visual contradiction.
- Spec line 66 names `tests/co1_7_extra_head_t_advancement.rs`, while §3.3 line 308 uses `tests/co1_7_extra_sequencer_head_t_advancement.rs`.

## **VERDICT**: PASS

## Top issues (if CHALLENGE)
N/A.

## Conviction (low/med/high)
High.
