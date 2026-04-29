# Codex CO1.7-extra Round-2 Audit
**Date**: 2026-04-29
**Target**: spec v1 (post round-1 scope split)
**HEAD**: 617f01e739253f2c2bee59a9ed44c2e6edb5fd4c
**Prompt size**: 267955 chars

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
session id: 019dd736-08c2-7e42-9dba-ce6df583a777
--------
user
# Codex Adversarial Audit — CO1.7-extra v1 (Round 2; post round-1 scope split)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-2 (parallel).

**Mandate**: round 2 dual external audit on CO1.7-extra v1 — a SCOPE-SPLIT atom carved out of the round-1-CHALLENGED bundled CO1.7.5 v1. Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

**What changed since round-1**: ArchitectAI executed Occam-driven scope split (B2: split by dependency profile) per user's "无损压缩即智能" principle. The previous bundled atom is now TWO atoms:
- **CO1.7-extra (this audit target)**: D2 head_t close + D3 Sequencer entry-point wiring + 1 substrate-independent test. **No FC1/FC2 substrate dependency**.
- **CO1.7.5 (future)**: D1 transition bodies + 3 D4 tests + un-ignore replay test. Gated on CO P2.x substrate atoms.

Round-1 must-fix items disposition:
- M1 (substrate gap) → D1 deferred to future CO1.7.5; CO1.7-extra has zero substrate dep
- M2 (purity violations) → moved with D1; not in CO1.7-extra
- M3 (compile defects: TuringBus / Kernel derives / Sequencer placement) → fixed in v1
- M4 (§ 0.3 process passive) → § 0.4 active reconciliation commitment + STATE v1.5 issue filing committed
- M5 (Q1 head_commit_oid_hex) → synthesis: default None + mandatory override + defensive test
- M6/M7 (mapping table + RejectedAttemptSummary) → moved with D1
- M8 (smoke staleness) → footer re-run at f7fc19f, 8/8 PASS

## What you're reviewing

1. **Spec doc**: `handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md` (~378 lines)
2. **Round-1 audit artifacts** (context): `handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md`, `GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md`, `CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md`
3. **Frozen prior specs**: CO1.7 v1.2 (round-3 PASS/PASS — supplies LedgerWriter trait + Sequencer machinery + head_commit_oid)
4. **Shipped code (target of D2/D3)**: src/state/sequencer.rs + src/bottom_white/ledger/transition_ledger.rs (D2 trait method addition + apply_one stage 9 patch); src/bus.rs (TuringBus forwarder) + src/kernel.rs (Kernel field) (D3 STEP_B target)
5. **Reference**: src/state/q_state.rs (NodeId types); src/bottom_white/cas/store.rs (CasStore put/get for D4 test)

## Round 2 audit questions

**Q1. Round-1 M1 closure (substrate gap)**: spec § 0.1 declares D1 transition bodies + 3 D4 tests moved to future CO1.7.5 atom; CO1.7-extra has "no substrate dependency". Verify:
- The 3 deliverables actually in CO1.7-extra (D2 + D3 + 1 D4 test) — are any of them transitively substrate-dependent? Specifically: does D2's `q_w.head_t = NodeId(commit_oid_hex)` write require ANY economic_state_t / predicate / tool registry method that doesn't exist? Does D3's TuringBus/Kernel wiring touch substrate?
- The cas_payload_round_trip test — uses only `CasStore::put` + `get`. Both exist (CO1.4 + CO1.4-extra shipped). Confirm no hidden substrate dep.
- Is the scope split a **valid** application of Anti-Oreo three-layer separation, or did the spec smuggle FC2/FC1 work into CO1.7-extra under a different name?

**Q2. Round-1 M2 closure (D1 purity violations)**: D1 is now out of scope. Confirm:
- Spec § 4 explicitly defers transition bodies + their purity contract concerns to future CO1.7.5
- Spec does NOT contain residual D1 specifications that could mislead implementation

**Q3. Round-1 M3 closure (compile defects)**: verify each fix:
- M3a: spec § 2.1 uses `TuringBus` everywhere (NOT `Bus`). Grep the spec body — any residual `Bus` references that should be `TuringBus`?
- M3b: Kernel field annotated `#[serde(skip)]` (spec § 2.1). Sequencer struct gets `#[derive(Debug)]` (spec § 2.1). Are these necessary AND sufficient? Specifically: does Sequencer's `Arc<RwLock<dyn LedgerWriter>>` field allow blanket `Debug` derive, or is manual impl actually required (per Q1' open)?
- M3c: Sequencer placement justification (spec § 2.2) — three arguments: (1) parallel to existing Tape/NodeId pattern, (2) state lives in Q_t not Kernel, (3) doc-comment patch. Does this hold up under scrutiny? Or does it just paper over the layering concern?

**Q4. Round-1 M4 closure (§ 0.4 active reconciliation)**: spec § 0.4 commits to filing STATE_TRANSITION_SPEC v1.5 housekeeping issue. Verify:
- Is "filing an issue" sufficient, or does the spec need to actually DRAFT the v1.5 patch text inline?
- Spec asserts "downstream-spec supersession authority principle" — is this assertion within ArchitectAI's authority, or does it require a constitution-level amendment?
- Two carry-forward supersessions (NodeId head_t binding + SignalKind 4-variant) migrate to future CO1.7.5 — but they actually take effect HERE (D2 sets head_t = NodeId(commit_oid_hex), not NodeId::from_state_root). Is the migration framing correct?

**Q5. Round-1 M5 closure (Q1 synthesis)**: spec § 1.2 + § 3.2 implement default `None` + mandatory override + defensive `git2_writer_returns_some_after_commit` test. Verify:
- The defensive test is sufficient: does it actually catch silent stagnation, or only the most obvious failure mode?
- "Mandatory override" — is this enforced by the language (compiler) or only by spec/convention? If only by convention, what guarantees future LedgerWriter impls don't inherit the default-None silently?
- Q1' (NEW open): Sequencer Debug derive completeness. Spec proposes `finish_non_exhaustive()` fallback if blanket-derive fails. Is `finish_non_exhaustive` safe/sufficient? Any leak risk?

**Q6. Atomicity claim refinement (Q-B from round-1, refined here)**: spec § 1.1 says under acquired locks, `writer.commit() Ok → AtomicU64::store → field assignments` is infallible. The refined claim acknowledges this only fully holds when `head_commit_oid_hex` returns Some (Git2). Is this refinement now correct, or does it still overclaim?

**Q7. STEP_B ceremony argument** (Q-C from round-1, rebased): spec § 2.3 rebased the combined-ceremony argument from "Phase 0 minimum sufficient version is binding" (Codex r1 said advisory) to "functional coupling" (each half compile-error or no-op without other). Is functional-coupling a stronger criterion that justifies combined ceremony, or is it an alternative way to phrase the same advisory request?

**Q8. New defects in v1**: the scope split changed the atom shape. Any new defects introduced by the rewrite that weren't in the bundled v1?
- Spec body coherence: any internal contradictions / dangling references to D1/D4-3-tests that should now be removed?
- Test coverage: 2 tests (cas_payload_round_trip + git2_writer_returns_some_after_commit) — sufficient for D2 + D3 + 1 D4 test scope, or should v1.1 add a head_t-advancement integration test that exercises the actual D2 code path?
- LoC estimate (~150-230 LoC): defensible? Or does Sequencer Debug + serde-skip + with_sequencer constructor + 2 trait method overrides come out higher?

**Q9. Implementation gating**: assuming all your CHALLENGEs are addressed in v1.1, is CO1.7-extra implementable end-to-end (cargo test --workspace passing with the 2 new tests)? Specific blockers — any v4-blocking deps that aren't in the inheritance list (CO1.7-impl + CO1.4-extra)?

## Output format

# Codex CO1.7-extra Round-2 Audit
## Q1 Round-1 M1 closure (substrate gap)
## Q2 Round-1 M2 closure (D1 purity)
## Q3 Round-1 M3 closure (compile defects)
## Q4 Round-1 M4 closure (§ 0.4 reconciliation)
## Q5 Round-1 M5 closure (Q1 synthesis) + Q1' Sequencer Debug
## Q6 Atomicity claim refinement
## Q7 STEP_B functional-coupling argument
## Q8 New defects in v1
## Q9 Implementation gating
## **VERDICT**: PASS / CHALLENGE / VETO
## Top 3 must-fix (if CHALLENGE)
## Conviction (low/med/high)

Be rigorous. Cite spec line numbers + source file line numbers when calling defects. Do NOT pass on principle; do NOT veto on principle. Real defects = CHALLENGE; foundational design flaw = VETO; clean spec = PASS.

---


# CO1.7-extra v1 (target of audit)

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

Smoke run at HEAD `f7fc19f` (CO1.7-extra v1 spec rewrite commit).

| # | Claim | Result | Status |
|---|---|---|---|
| S1 | Git2LedgerWriter::head_commit_oid returns Option<git2::Oid> | `pub fn head_commit_oid(&self) -> Option<git2::Oid>` (transition_ledger.rs:674) | ✅ PASS |
| S2 | Bus struct named TuringBus | `pub struct TuringBus` at bus.rs:53 | ✅ PASS |
| S3 | Kernel derives Debug, Serialize, Deserialize | `#[derive(Debug, Serialize, Deserialize)]` precedes `pub struct Kernel` (kernel.rs:18) | ✅ PASS |
| S4 | Sequencer struct exists | `pub struct Sequencer` at sequencer.rs:190 | ✅ PASS |
| S5 | CasStore put + get exposed | `pub fn put` at store.rs:163; `pub fn get` at store.rs:199 | ✅ PASS |
| S6 | wallet (sdk/tools/wallet.rs) untouched | 0 hits of `transition_ledger\|state::sequencer\|TypedTx` | ✅ PASS |
| S7 | QState.head_t is state::q_state::NodeId tuple struct | `pub head_t: NodeId` (q_state.rs:311) — type matches | ✅ PASS |
| S8 | cargo baseline | check pass + `239 passed; 0 failed; 1 ignored` (the ignored is `sequencer_serial_replay_byte_identity`, intentionally deferred to future CO1.7.5 atom) | ✅ PASS |

**Smoke gate**: 8 / 8 PASS at HEAD `f7fc19f`. Spec ready for round-2 dual external audit.

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


---

# XREF: round-1 merged verdict (the document driving this scope split)

# CO1.7.5 Dual External Audit — Round-1 Merged Verdict

**Date**: 2026-04-29
**Target**: `CO1_7_5_TRANSITION_BODIES_AND_RUNTIME_WIRING_v1_2026-04-29.md` at HEAD `334111a`
**Audits**: Codex r1 (`CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md`) + Gemini r1 (`GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md`)

## Verdict matrix

| Audit | Verdict | Conviction | Q breakdown |
|---|---|---|---|
| Codex | **CHALLENGE** | High | Q-A pass-with-v1.1-ask, Q-B pass-with-overclaim-fix, Q-C CHALLENGE (compile defects), Q-D **CHALLENGE (purity violations)**, Q-E **CHALLENGE (mapping table overclaims)**, Q-F PASS, Q-G CHALLENGE (smoke/patch staleness), Q-H **CHALLENGE (substrate missing)**, Q-I CHALLENGE (impl gating) |
| Gemini | **CHALLENGE** | High | Q1 PASS, Q2 **CHALLENGE (process passive)**, Q3 PASS, Q4 PASS, Q5 PASS, Q6 PASS, Q7 PASS, Q8 vote=`unimplemented!()` |

**Conservative-merged verdict** (per memory `feedback_dual_audit_conflict`, VETO > CHALLENGE > PASS): **CHALLENGE / High**. No VETO from either audit ("no foundational design flaw" — both explicit).

## Where the audits agree

1. § 0.3 STATE supersession framing must take **active** reconciliation responsibility (not delegate to STATE curator). Gemini MF1+MF3 + Codex Q-A v1.1 ask converge.
2. SignalKind 4-variant minimization is a **safe deferral**, not a hazard (Gemini Q5 PASS + Codex Q-F PASS).
3. Combined STEP_B ceremony (one A/B unit covering bus.rs + kernel.rs) is **strategically sound** (Gemini Q4 PASS); Codex agrees the combination is permissible but flags overclaim about Phase 0 binding-vs-advisory (Codex Q-C).
4. Hygiene OBS handling appropriate — inline fix on CLAUDE.md is correct since it's project instructions, not constitution.md (Gemini Q6 PASS).

## Where the audits disagree

**Q1 (`head_commit_oid_hex` default impl)**:
- Gemini: `unimplemented!()` (silent stagnation worse — head_t is constitutional anchor)
- Codex: `default { None }` + mandate Git2 override + Git2-backed test (panic-after-commit-success worse)

**Synthesis** (preserves both safety arguments): default `None` (Codex no-panic) + spec mandates every shipped LedgerWriter override head_commit_oid_hex + add a test that fails if Git2LedgerWriter returns None at commit time (Gemini silent-stagnation defense). The default is then dead code in production, never reached.

## Where Codex went deeper than Gemini (substantive must-fix)

Codex performed source-level verification at depth Gemini did not (Codex's prompt was implementer-paranoid; Gemini's was strategic). Findings unique to Codex:

### M1 (Q-D + Q-H + Q-I — the heavyweight finding): substrate missing

Codex verified each STATE § 3.x call site against shipped APIs and found the spec assumes infrastructure that **does not exist**:

| STATE pseudocode reference | Shipped reality | Gap |
|---|---|---|
| `q.economic_state_t.claims_t.get(&tx.target_work_tx).status.allows_verification()` | `ClaimsIndex` = `BTreeMap<TxId, ClaimEntry>` with only `amount` + `claimant` | No `status`, `solver`, `task_id` fields |
| `q.economic_state_t.task_markets_t.get(target.task_id).config.verifier_bond_on_slash` | `TaskMarketEntry` has no `deadline` / `creator` / `config` fields | No config substrate |
| `window.is_open(tx.timestamp_logical)` | `ChallengeCase` lacks `duration` / `outcome` field + `is_open` method | No challenge-window machinery |
| `registry.run_acceptance(tx, q)?` / `run_verification` / `run_counterexample_check` | `PredicateRegistry` exposes only `register/get/root/view` | No execution methods |
| `q_next.economic_state_t.derive_state_root()` | Method does not exist on EconomicState | No state-root derivation |

These are FC1 (top-white predicate execution) + FC2 (middle-black state-mutation schemas) responsibilities. Putting them inside an FC3 (bottom-white L4 ledger) atom violates Anti-Oreo 三层 separation.

Per PROJECT_DECISION_MAP § 3.4, the prerequisite substrate is the planned **CO P2.x family** (currently in "Pending CO P2 (after CO P1 exit)"):
- CO P2.1 TaskMarket
- CO P2.2 EscrowVault
- CO P2.3 ContributionLedger
- CO P2.5 ChallengeCourt (challenge-window machinery)
- CO P2.6 SettlementEngine (`issue_provisional`, settlement formula)
- CO P2.7 Agent roles
- CO P2.9 ReputationIndex (`reputations_t.adjust`)
- CO1.11 Safety vs Creation (uses PredicateRegistry — likely supplies execution methods)

### M2 (Q-D — purity boundary violations)

Spec § 1 D1 promises 4-arg signature `(&QState, &TxVariant, &PredicateRegistry, &ToolRegistry)` + "no I/O". STATE pseudocode violates this:
- `challenge_transition` reads CAS inside transition (`cas::get(&tx.counterexample_cid)?`) — needs CAS arg
- `emit_terminal_summary_transition` takes `&Runtime`, reads run state, signs inside transition — needs runtime + keypair args
- System signature verification needs `PinnedSystemPubkeys` — not in 4-arg sig

### M3 (Q-C — D3 compile defects)

- Bus type is **`TuringBus`** (`src/bus.rs:53`), not `Bus` — spec全文写错
- Kernel derives `Debug, Serialize, Deserialize` (`src/kernel.rs:18`); adding `Option<Arc<Sequencer>>` requires `serde(skip)` + Debug handling; Sequencer has no derives
- Kernel docs as "pure topology" (`src/kernel.rs:15-17`) — Sequencer placement needs stronger justification or move to a runtime layer

### M6 (Q-E — TransitionError mapping table overclaims)

Spec Q5 mapping table missed:
- CAS lookup failure in `challenge_transition` (no mapped variant)
- `SettlementEngine::issue_provisional` failure in Work (no mapped variant)
- Runtime / system-signature validation paths for FinalizeRewardTx + TerminalSummaryTx
- Some stale-parent checks for system tx

### M7 (Q-E — RejectedAttemptSummary side channel not real)

Spec asserts a side channel for rich rejection context. Codex finds:
- A type at `src/bottom_white/ledger/system_keypair.rs:151-158` exists, but does NOT match STATE shape (`STATE:192-214`)
- Sequencer rejection currently only logs and skips (`src/state/sequencer.rs:252-266`); no rejected-summary stamping path is wired

### M8 (Q-G — smoke/patch staleness)

- Footer says smoke ran at `2f5093a` — should be current HEAD `334111a` (smoke ran pre-commit; the spec was committed after, became HEAD)
- Spec § 1 D4 cites `transition_ledger.rs:1451` for the `#[ignore]`; actual location is line `1455` (`1451` is the doc-comment)
- S8 says "18 warnings"; full workspace also emits 1 `gix_capability_spike` warning → "19"
- P3 references "§ 6 ack #8" but § 6 has only 6 items after self-audit dropped duplicates

## Occam-driven scope decision (executed without further audit input)

The audit findings reveal the v1 spec was **mis-scoped** by my session: D1 transition bodies were bundled with D2+D3+D4 wiring, but D1 has heavyweight cross-layer substrate dependencies that D2+D3 do not.

**Decision** (per "无损压缩即智能" + Anti-Oreo + Occam, applied by ArchitectAI without further audit input):

Split the atom by dependency profile, using existing `CO1.4-extra` pattern as precedent:

| Atom | Scope | Substrate dependency | Ships when |
|---|---|---|---|
| **CO1.7-extra** (NEW; bridge atom) | D2 head_t close + D3 Sequencer entry-point + 1 D4 test (`cas_payload_round_trip`, substrate-independent) | None — uses only frozen LedgerWriter trait + Sequencer machinery + existing CasStore | Now (small atom; v1.1 fixes M3-M8) |
| **CO1.7.5** (reverts to CO1.7 § 13 original meaning) | D1 transition bodies + 3 D4 tests + un-ignore `sequencer_serial_replay_byte_identity` | CO P2.1 / 2.2 / 2.3 / 2.5 / 2.6 / 2.7 / 2.9 + CO1.11 + (new) PredicateRegistry execution-methods atom | After substrate atoms PASS/PASS |

### Why this beats the 3 user-presented options under Occam

| Option | Description-length cost | Anti-Oreo | WP § 5.L4 | Verdict |
|---|---|---|---|---|
| A: CO1.7.5 owns substrate | NEW concept "L4 atom owns FC1/FC2 schemas" | ❌ violates 三层 | ❌ exceeds L4 boundary | NO |
| B (raw): declare blocker; spec stays bundled | NEW concept "all-or-nothing implementation gate" | OK | OK but inefficient | suboptimal |
| C: atom-internal phasing D5 | NEW concept "atom-internal heterogeneous phases" | ❌ if D5 cross-layer | ❌ same | NO |
| **B2 (executed)**: split by dep profile | **0 new concepts** (CO1.4-extra precedent + Anti-Oreo + atom-decomposition) | ✅ each atom in its layer | ✅ L4 atom contains only L4 work | **YES** |

### What this reveals about LATEST.md

LATEST.md (commit `2f5093a`) claims "Wave 6 #1 80% complete; CO1.7.5 single critical path". This is **false-precision**. True state:
- L4 wiring (D2+D3): **shipping now via CO1.7-extra v1.1** post round-2 PASS/PASS (~80% → ~85%)
- L4 transition bodies (CO1.7.5 per CO1.7 § 13): **gated on 7+ substrate atoms** in the CO P2.x family
- Wave 6 #1 actual closure: requires CO1.7-extra + CO1.7.5 + CO P2.x family → far from "single critical path"

LATEST.md should be patched in the same session-cluster to reflect this audit-derived reality.

## v1.1 patch plan (rolled into CO1.7-extra v1, applied this session)

| ID | Source | Fix |
|---|---|---|
| **M1** scope | Codex Q-D/H/I + Occam | Atom rescoped to D2+D3 + 1 substrate-independent D4 test. D1 + 3 D4 tests + un-ignore moved to future CO1.7.5 atom (gated). |
| **M2** purity | Codex Q-D | Now N/A for CO1.7-extra (no transition bodies in scope). Will be addressed by future CO1.7.5 spec. |
| **M3a** TuringBus | Codex Q-C | Spec body uses `TuringBus` per `src/bus.rs:53`. |
| **M3b** Kernel derives | Codex Q-C | Spec specifies `#[serde(skip)]` on Kernel.sequencer field + manual Debug; Sequencer.rs adds `#[derive(Debug)]` minimal. |
| **M3c** Sequencer placement | Codex Q-C | v1.1 keeps Sequencer in Kernel; clarifies "pure topology" doc to acknowledge Sequencer as the typed-tx topology element (matching legacy Tape/NodeId pattern). |
| **M4** § 0.3 active reconciliation | Gemini MF1+MF3 + Codex Q-A | § 0.3 commits to filing STATE_TRANSITION_SPEC v1.5 housekeeping issue as part of CO1.7-extra atom closure; asserts downstream-spec supersession principle explicitly. |
| **M5** Q1 synthesis | Both | Default `None` + spec mandates every shipped LedgerWriter override + add test asserting Git2LedgerWriter returns Some at commit time. |
| **M6** mapping table | Codex Q-E | Now N/A (TransitionError mapping is a transition-bodies concern; future CO1.7.5 owns it). |
| **M7** RejectedAttemptSummary claim | Codex Q-E | Now N/A (transition-bodies concern). |
| **M8a** smoke commit cite | Codex Q-G | Footer updated to current HEAD (TBD post-rewrite). |
| **M8b** ignore line cite | Codex Q-G | `1451` → `1455` (doc-comment vs actual `#[ignore]`). |
| **M8c** warning count | Codex Q-G | "18" → "19" (gix_capability_spike spike adds one warning at workspace level). |
| **M8d** P3 stale wording | Codex Q-G | "§ 6 ack #8" → "§ 6 ack" (no #8 after self-audit drop). |

## Not addressed in v1.1 (out of scope for CO1.7-extra atom)

- M2 (purity boundary) — gated to future CO1.7.5 spec
- M6 (mapping table completeness) — gated to future CO1.7.5 spec
- M7 (RejectedAttemptSummary substantiation) — gated to future CO1.7.5 spec

These migrate with D1 to the future CO1.7.5 atom, where they belong.

## Audit cost summary

- Codex r1: 254,013 tokens used (size matches deep source-level review)
- Gemini r1: prompt=144,441 / candidates=3,107 / total=150,915 tokens
- Estimated round cost: ~$8-15 (single round both audits)
- Cumulative project audit spend: ~$183-288 / $890 mid-budget (~21-32%)

## Status going forward

1. **CO1.7-extra v1.1**: spec rewritten in place this session; awaiting round-2 dual audit
2. **CO1.7.5 (transition bodies)**: future atom; spec to be drafted **after** CO P2.x substrate atoms reach individual PASS/PASS
3. **LATEST.md**: should be patched to reflect audit-derived true state of Wave 6 #1 (~30-40%, not 80%)
4. **PROJECT_DECISION_MAP**: should track CO1.7-extra as new bridge atom; CO1.7.5 dependency declared


---

# XREF: CO1.7 v1.2 spec (frozen, round-3 PASS/PASS) — supplies inheritance

# CO1.7 Transition Ledger v1.2 — Round-2 closure

**Status**: v1.2 — round-2 returned PASS (Gemini, high) + CHALLENGE (Codex, high; 3 narrow patch blockers). Conservative merged CHALLENGE. v1.2 closes the 3 v1.1→v1.2 patches: (a) C3 actually wired in code (`CanonicalMessage::LedgerEntrySigning([u8;32])` + `transition_ledger_emitter::sign_ledger_entry`); (b) K3 head_t mutation explicitly deferred to CO1.7.5+ (no longer claimed in v1.x); (c) `ObjectType::Transition` replaced with shipped `ObjectType::ProposalPayload`. Plus typo fix and 1 new test. Awaiting round-3.

**Status (v1.1)**: v1.1 — round-1 dual external audit (Codex + Gemini) returned CHALLENGE/CHALLENGE; this version closes 11 must-fix items, awaiting round-2.
**Author**: ArchitectAI (Claude); session 2026-04-28.
**Supersedes**: v1 (2026-04-28 morning DRAFT outline).
**Round-1 verdicts**: `handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md` + `handover/audits/GEMINI_CO1_7_ROUND1_AUDIT_2026-04-28.md`; merged in `handover/audits/CO1_7_DUAL_AUDIT_VERDICT_R1_2026-04-28.md`.

**Companion specs** (frozen, read first):
- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — typed schemas + step_transition pseudocode + 27 invariants (round-4 PASS/PASS)
- `SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md` — runtime keypair lifecycle (CO1.7.0a-f, done @ Wave 4-B)
- `META_TRANSITION_INTERFACE_v1_2026-04-27.md` — trait pattern for L4 acceptance (deferred runtime to v4.1)
- `TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md` § 5.L4 — ChainTape Layer 4 axioms

**Single sentence**: implement the L4 transition_ledger module so that `ledger::append(parent_root, signing_digest) → new_root` (called from sequencer) is real code, the L4 sequencer (§ 5.2.1) is real code, and `Q_t.ledger_root_t` is no longer a placeholder.

---

## v1.2 patch log (vs. v1.1) — round-2 closure

| ID | v1.1 issue | v1.2 fix | Source |
|---|---|---|---|
| **R2-C3** | Spec claimed "C3 CLOSED" but `system_keypair.rs` had no LedgerEntry path; skeleton itself said "deferred to CO1.7.5+" | Wave 4-B additive extension shipped: `CanonicalMessage::LedgerEntrySigning([u8;32])` (opaque digest variant; avoids transition_ledger ↔ system_keypair circular dep) + `canonical_digest` match arm + new `pub(crate) mod transition_ledger_emitter` with `sign_ledger_entry(keypair, digest)`. Skeleton test 9 (`signature_round_trip_and_transplant_defense`) now exercises the real roundtrip + K2 + D1 defenses. | Codex round-2 must-fix #1 |
| **R2-K3** | Spec § 3 / § 5 said "CO1.7 owns head_t = NodeId(commit_sha)" but `LedgerWriter::commit` returns `Hash` not commit SHA; v1.1 InMemoryLedgerWriter has no commit_sha to return at all → contradiction | head_t mutation explicitly **deferred to CO1.7.5+** (when Git2LedgerWriter exists and can return both Hash + commit SHA). v1.x ledger owns `ledger_root_t` only; `head_t` continues to be set elsewhere (currently QState placeholder; CO1.7.5 wiring concern). Spec § 0 / § 3 / § 5 updated. | Codex round-2 must-fix #2 |
| **R2-C2-CAS** | Spec § 3 sequencer pseudocode used `ObjectType::Transition` but shipped `ObjectType` has no such variant | Spec § 3 changed to `ObjectType::ProposalPayload` (the existing variant for agent work_tx payloads — semantically correct, no schema extension needed) | Codex round-2 must-fix #3 |
| **R2-typo** | Spec § 1.1 said "8-field bytes-on-the-wire" but `LedgerEntrySigningPayload` actually has 9 fields | Updated to "9-field" | Codex Q-C2 |

3 must-fix + 1 typo = **4 closures** integrated.

---

## v1.1 patch log (vs. v1)

| ID | v1 issue | v1.1 fix | Source |
|---|---|---|---|
| C1 | replay was single-mode; called "I-DETHASH witness" but skeleton only did chain check | Two-mode `ReplayMode::ChainOnly` (skeleton-stage) vs `ReplayMode::FullTransition` (CO1.7.5+; I-DETHASH witness only in this mode) | Codex Q-D + Gemini Q3 |
| C2 | spec did not acknowledge that shipped `CasStore::open()` initializes empty in-memory index → cold-replay impossible | § 0 + § 5 explicit dependency + mitigation: CasStore index persistence is deferred to **CO1.4-extra** (separate atom); v1 documents the gap | Codex Q-H + Gemini Q2 |
| C3 | signing primitive integration via `CanonicalMessage` enum was unspecified; spec called nonexistent digest-form verifier | Path A: extend `CanonicalMessage::LedgerEntrySigning(LedgerEntrySigningPayload)`; sign separate signing payload (NOT raw `LedgerEntry`); new API `keypair.sign_ledger_entry(payload) → SystemSignature` | Codex Q-G + Gemini Q4 |
| K1 | sequencer `next_logical_t.fetch_add(1)` happens BEFORE accept; rejection skips `logical_t`, replay rejects gaps | Dual counter design: `next_submit_id` advances at submit; `next_logical_t` advances ONLY on commit | Codex Q-C |
| K2 | signature did NOT bind `parent_ledger_root` → transplant attack | `LedgerEntrySigningPayload` includes `parent_ledger_root` field | Codex Q-B (NEW) |
| K3 | L4/L5 head_t ownership inconsistent (spec line 194 vs 276 disagreed) | CO1.7 owns `ledger_root_t` + commit-chain `head_t = NodeId(commit_sha)` only; L5 (CO1.8) owns `state_root_t` mutation; sequencer drops `head_t = NodeId::from_state_root(...)` line | Codex Q-E |
| K4 | spec `LedgerWriter::commit(&self) → NodeId` + `iter_from` did not match skeleton `commit(&mut self) → Hash` | Spec aligned to skeleton: `&mut self` + `Hash` return; `iter_from` deferred to CO1.7.5+ when needed for cold-replay | Codex Q-H |
| K5 | `TxKind::Slash` enum variant present but `dispatch_transition` omitted it | Drop `TxKind::Slash` for v4; ChallengeCourt slashing event scheduled for CO P2.5 atom | Codex Q-H |
| K6 | `tx_kind as u8` cast without `#[repr(u8)]` → fragile discriminant | `#[repr(u8)]` + explicit discriminants (`Work = 0, Verify = 1, ...`) | Codex Q-H |
| K7 | spec promised 8 conformance tests; skeleton has 6 | Explicit list of 8 tests with skeleton-stage vs CO1.7.5-stage marker; unimplemented stubs now stage-marked | Codex Q-H |
| G1 | `LedgerEntry` struct rigid; future ZK / settlement proof had no place | Add `extensions: BTreeMap<String, Vec<u8>>` (empty in v1; reserved for v4.x without breaking schema) | Gemini Q9 |
| D1 | epoch binding disagreement (Codex bind YES; Gemini bind NO) | Conservative resolution: epoch IS bound in `LedgerEntrySigningPayload`; epoch NOT separately folded into `ledger_root_t` (Codex security wins; Gemini orthogonality preserved at the ledger_root axis) | merged verdict § 5 |

11 must-fix + 1 disagreement resolution = **12 closures** integrated below.

---

## § 0 Scope

### In scope (CO1.7 atom)
- **LedgerEntry schema**: canonical envelope wrapping each typed transition (WorkTx / VerifyTx / ChallengeTx / ReuseTx / FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx) before append to L4. **Note**: `Slash` is NOT in v4 (deferred to CO P2.5 ChallengeCourt atom — K5).
- **LedgerEntrySigningPayload**: the 9-field bytes-on-the-wire that the system keypair actually signs (distinct from LedgerEntry-the-stored-record).
- **LedgerRoot computation**: deterministic Merkle accumulation over signed digests; this is the value of `Q_t.ledger_root_t`.
- **Sequencer**: per-(runtime_repo, run_id) single-writer instance enforcing § 5.2.1 (dual-counter `submit_id`/`logical_t`, submission-order serialization, post-commit `logical_t` assignment).
- **append(parent_root, signing_digest)**: pure function returning the new ledger_root.
- **replay (two-mode)**: `ChainOnly` (chain integrity; skeleton-stage; v1) vs `FullTransition` (rerun pure transitions + verify state_root + verify signatures; CO1.7.5+; THE I-DETHASH witness).
- **Storage backend**: git2-rs commit chain (built on CO1.4 CAS); each LedgerEntry = one git commit on `refs/transitions/main`. **R2-K3**: head_t mutation deferred to CO1.7.5+ — v1.x ledger does NOT mutate `Q_t.head_t` directly. Once `Git2LedgerWriter::commit` exists and returns commit_sha alongside Hash, CO1.7.5 wiring will set `head_t = NodeId(commit_sha)` outside the L4 sequencer apply path.
- **CanonicalMessage extension**: extends shipped `CanonicalMessage` enum with `LedgerEntrySigning([u8; 32])` opaque-digest variant (R2 design — avoids circular module dep); new typed sign API `transition_ledger_emitter::sign_ledger_entry(keypair, signing_payload_digest)`.

### Out of scope (handled by other atoms)
- WorkTx / VerifyTx / ChallengeTx / ReuseTx / FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx schemas — frozen in `STATE_TRANSITION_SPEC § 1`.
- step_transition / verify_transition / challenge_transition logic — frozen in `STATE_TRANSITION_SPEC § 3`.
- system_keypair signing primitives — done @ CO1.7.0a-f; CO1.7 only adds a typed extension.
- L5 materializer (state_root computation) — deferred to **CO1.8**. **K3 boundary (revised v1.2)**: CO1.7 owns `ledger_root_t` only; CO1.8 owns `state_root_t`; **head_t mutation is deferred to CO1.7.5+ wiring** (when `Git2LedgerWriter` exists). Sequencer does NOT mutate `state_root_t` or `head_t` directly; it accepts `q_next.state_root_t` as returned by the transition function and persists `ledger_root_t`.
- L6 signal indices — deferred to **CO1.9**.
- AttributionEngine DAG — deferred to CO P2.4.0 spike (Inv 8 design).
- MetaTx full schema — v4.1 only; v4 emits `MetaProposalDraft` to L3 CAS, not L4.
- **Slash transition** — deferred to CO P2.5 (ChallengeCourt) atom; v4 ledger has no `TxKind::Slash`.
- **CAS index persistence (cold-replay enabler)** — `CasStore::open()` shipped at Wave 3 initializes empty in-memory index ([store.rs:67](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs)); cold-replay therefore cannot recover payloads via `CasStore::get` after restart. **CO1.4-extra** atom (NEW, scheduled post-CO1.7) adds index persistence (likely a sidecar JSONL or git-tag manifest). v1 ledger documents the dependency; full-mode replay is implementable once CO1.4-extra lands.

### What this spec is NOT replacing
- `src/ledger.rs` (legacy, top-level) is retired in **CO1.1.5 (kernel.rs split)**; CO1.7 lives at `src/bottom_white/ledger/transition_ledger.rs` (NEW). No STEP_B parallel-branch ceremony required (new module, not restricted file); restricted files per CLAUDE.md "Code Standard" are `src/{kernel,bus,wallet}.rs` (corrected from v1's incorrect `wal.rs` per K6 tail).

---

## § 1 LedgerEntry schema (the stored record)

```rust
use std::collections::BTreeMap;

/// TRACE_MATRIX FC2-Append (FC2 transition machinery): canonical record
/// stored at L4. One LedgerEntry per accepted transition. Genesis state has
/// zero LedgerEntries; ledger_root_t = genesis_ledger_root_t (per § 5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerEntry {
    /// Monotonic counter from sequencer; starts at 1 at first accept.
    /// **K1**: assigned ONLY at commit; rejected submissions never get a logical_t.
    pub logical_t: u64,                          //  1

    /// Parent state_root before this transition. Equals the resulting_state_root
    /// of the entry at logical_t-1 (or genesis state_root at logical_t=1).
    pub parent_state_root: Hash,                 //  2

    /// **K2 NEW**: parent_ledger_root before this entry is folded in.
    /// Bound by signature (transplant attack defense). Equals the
    /// resulting_ledger_root of entry at logical_t-1 (or genesis_ledger_root at logical_t=1).
    pub parent_ledger_root: Hash,                //  3

    /// Discriminator; payload schema depends on this. **K6**: `#[repr(u8)]` for stable
    /// discriminant in canonical digest computation.
    pub tx_kind: TxKind,                         //  4

    /// CAS handle (CO1.4) to canonically-serialized payload. Sequencer puts payload
    /// to CAS via `CasStore::put(content, object_type, creator, created_at_logical_t, schema_id)`
    /// (DIV-5: 5-param signature). cid = sha256(content).
    pub tx_payload_cid: Cid,                     //  5

    /// Resulting state_root after `dispatch_transition` applied. NOT mutated by L4
    /// — accepted as-returned from the transition function (K3 boundary).
    pub resulting_state_root: Hash,              //  6

    /// Resulting ledger_root after this entry is folded in.
    /// Convention: ledger_root_{t+1} = sha256(domain_sep || parent_ledger_root || signing_digest_t)
    /// where signing_digest_t = canonical_digest(LedgerEntrySigningPayload at logical_t).
    /// **NOT signed** — derivative; including it in signed digest creates a cycle (Q9).
    pub resulting_ledger_root: Hash,             //  7

    /// Wall-clock-free timestamp; equal to `logical_t` post-commit (no separate clock).
    /// Field retained for symmetry with STATE_TRANSITION_SPEC § 1.2 WorkTx.
    pub timestamp_logical: u64,                  //  8

    /// **DIV-3 / Q10**: which pinned epoch pubkey signed this entry. Required by
    /// `system_keypair::verify_system_signature(sig, msg, epoch, pinned_pubkeys)`.
    /// Bound in signed payload (Codex security argument; **D1** resolved).
    pub epoch: SystemEpoch,                      //  9

    /// **G1 NEW**: forward-compatibility extension map. Empty in v1; reserved for
    /// v4.x additions (e.g. ZK predicate proofs, settlement proofs, public-market metadata).
    /// Bound in signed payload (so additions cannot bypass signature).
    pub extensions: BTreeMap<String, Vec<u8>>,   // 10

    /// Detached system signature over canonical_digest of LedgerEntrySigningPayload.
    /// Distinct from agent-signature inside payload. NOT included in signed digest.
    pub system_signature: SystemSignature,       // 11
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TxKind {
    Work            = 0,   // WorkTx       (STATE spec § 1.2)
    Verify          = 1,   // VerifyTx     (STATE spec § 1.3)
    Challenge       = 2,   // ChallengeTx  (STATE spec § 1.3)
    Reuse           = 3,   // ReuseTx      (STATE spec § 1.3)
    FinalizeReward  = 4,   // FinalizeRewardTx (STATE spec § 3.4)
    TaskExpire      = 5,   // TaskExpireTx (STATE spec § 3.6)
    TerminalSummary = 6,   // TerminalSummaryTx (STATE spec § 1.5 + § 3.7)
    // K5: NO `Slash` — ChallengeCourt slash event is in CO P2.5; v4 ledger has no Slash variant.
}
```

### § 1.1 LedgerEntrySigningPayload (NEW per C3)

The system signature signs a **separate struct**, not the LedgerEntry directly. This:
1. **Excludes derivatives**: `resulting_ledger_root` (cycle: ledger_root ⊃ digest ⊃ ledger_root) and `system_signature` (its own input) are NOT in the signing payload.
2. **Binds non-derivatives**: `parent_ledger_root` (K2 transplant defense) + `epoch` (D1 + Q10) + `extensions` (G1 forward compat is signed).
3. **Has stable wire format**: explicit byte layout (see canonical_digest below), independent of bincode/serde choices.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerEntrySigningPayload {
    pub logical_t: u64,                         // 1
    pub parent_state_root: Hash,                // 2
    pub parent_ledger_root: Hash,               // 3 (K2 transplant defense)
    pub tx_kind: TxKind,                        // 4 (#[repr(u8)] discriminant cast safe)
    pub tx_payload_cid: Cid,                    // 5
    pub resulting_state_root: Hash,             // 6
    pub timestamp_logical: u64,                 // 7
    pub epoch: SystemEpoch,                     // 8 (D1)
    pub extensions: BTreeMap<String, Vec<u8>>,  // 9 (G1; empty-map case yields empty bytes)
    // EXCLUDED: resulting_ledger_root (cycle); system_signature (its input).
}

impl LedgerEntrySigningPayload {
    pub fn canonical_digest(&self) -> Hash {
        let mut h = Sha256::new();
        h.update(b"turingosv4.ledger_entry_signing.v1");      // domain separation
        h.update(self.logical_t.to_be_bytes());
        h.update(self.parent_state_root.0);
        h.update(self.parent_ledger_root.0);
        h.update((self.tx_kind as u8).to_be_bytes());          // K6 #[repr(u8)] makes this stable
        h.update(self.tx_payload_cid.0);
        h.update(self.resulting_state_root.0);
        h.update(self.timestamp_logical.to_be_bytes());
        h.update(self.epoch.get().to_be_bytes());
        // extensions: BTreeMap iteration is sorted by key (deterministic);
        // length-prefix each (key, value) pair to prevent ambiguity.
        h.update((self.extensions.len() as u64).to_be_bytes());
        for (k, v) in &self.extensions {                        // BTreeMap = lex order
            h.update((k.len() as u64).to_be_bytes());
            h.update(k.as_bytes());
            h.update((v.len() as u64).to_be_bytes());
            h.update(v);
        }
        Hash(h.finalize().into())
    }
}
```

### § 1.2 CanonicalMessage extension (per C3; **shipped in v1.2**)

CO1.7 extends shipped `system_keypair::CanonicalMessage` with one new variant. **R2-C3 design choice**: variant carries the **opaque 32-byte canonical_digest** of `LedgerEntrySigningPayload`, NOT the full payload struct. This avoids a circular `system_keypair ↔ transition_ledger` module dependency (the payload struct needs `Cid` from CAS module + `SystemEpoch` from system_keypair; carrying the precomputed digest sidesteps the cycle entirely). The signature still binds the full payload because `canonical_digest()` is deterministic in `transition_ledger`.

```rust
// In src/bottom_white/ledger/system_keypair.rs (additive Wave 4-B extension; SHIPPED v1.2):
pub enum CanonicalMessage {
    RejectedAttemptSummary(RejectedAttemptSummary),  // existing
    TerminalSummaryTx(TerminalSummaryTx),            // existing
    EpochRotationProof(EpochRotationProof),          // existing
    LedgerEntrySigning([u8; 32]),                     // NEW v1.2 (C3) — opaque digest
}

// canonical_digest() match arm (SHIPPED v1.2):
//   CanonicalMessage::LedgerEntrySigning(digest) => {
//       h.update(b"LedgerEntrySigning");
//       h.update(digest);
//   }

// Authorized emitter module (SHIPPED v1.2):
pub(crate) mod transition_ledger_emitter {
    pub(crate) fn sign_ledger_entry(
        keypair: &Ed25519Keypair,
        signing_payload_digest: [u8; 32],
    ) -> Result<SystemSignature, KeypairError>;
}
```

**Sequencer call site** (in transition_ledger.rs, illustrative):

```rust
let digest = signing_payload.canonical_digest();
let sig = transition_ledger_emitter::sign_ledger_entry(&keypair, digest.0)?;
```

**Forward-compat clause**: if v4.x adds new ledger-side message variants, they MUST add new `CanonicalMessage::*` variants (NOT extend the LedgerEntrySigning variant in-place; opaque digest is committed to `[u8; 32]`). v4-shipped extensions go in the `LedgerEntry::extensions` BTreeMap (G1) which IS bound in `LedgerEntrySigningPayload::canonical_digest()`.

---

## § 2 Module layout

```
src/bottom_white/ledger/
├── mod.rs                       (existing; v1.1 wires `pub mod transition_ledger`)
├── system_keypair.rs            (existing CO1.7.0a-f; CO1.7 adds 1 enum variant + 1 typed sign fn — additive)
└── transition_ledger.rs         (NEW; LedgerEntry, LedgerEntrySigningPayload, TxKind, append, replay_*, LedgerWriter)

src/state/
├── mod.rs                       (existing)
├── q_state.rs                   (existing; CO1.7 fills `ledger_root_t` placeholder; does NOT touch `state_root_t` per K3)
└── sequencer.rs                 (NEW; deferred to CO1.7.5; pre-audit type stub may land in v1.1 if useful)
```

**Crate boundary**: `transition_ledger` in `bottom_white::ledger` (tool layer); `sequencer` in `state::` (touches Q_t mutation). Sequencer DEPENDS ON ledger; ledger does NOT depend on sequencer (DAG: state → bottom_white::ledger → CO1.4 CAS → CO1.7.0a-f keypair).

---

## § 3 Sequencer (K1 dual-counter; K3 head_t ownership; C3 sign API)

```rust
/// TRACE_MATRIX § 5.2.1 — L4 sequencer; single-writer per (runtime_repo, run_id).
pub struct Sequencer {
    /// **K1 NEW**: separate counter for submissions (independent of accept).
    /// Used to derive submit_id for SubmissionReceipt; never appears in LedgerEntry.
    next_submit_id: AtomicU64,

    /// **K1 changed semantics**: advances ONLY on commit, NOT on submit.
    /// Genesis = 0; first accepted entry gets logical_t=1.
    next_logical_t: AtomicU64,

    /// **Q1 resolution**: bounded `tokio::sync::mpsc::Sender` (NOT unbounded).
    /// Submit returns `QueueFull` Err on saturation; agents handle backoff.
    queue_tx: tokio::sync::mpsc::Sender<TypedTx>,

    /// CAS handle for payload storage.
    cas: Arc<RwLock<CasStore>>,

    /// **C3**: signing key handle (CO1.7.0a-f).
    keypair: Arc<Ed25519Keypair>,
    epoch: SystemEpoch,                   // current signing epoch

    /// Storage backend (in CO1.7.5+; skeleton uses InMemoryLedgerWriter).
    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,

    /// Predicate + tool registries (read-only).
    predicate_registry: Arc<PredicateRegistry>,
    tool_registry: Arc<ToolRegistry>,

    /// Current Q_t snapshot.
    q: RwLock<QState>,
}

impl Sequencer {
    /// Submit a typed transition for processing. Returns immediately with a
    /// SubmissionReceipt carrying `submit_id` (NOT logical_t — submit_id is
    /// always assigned; logical_t only assigned post-accept).
    pub async fn submit(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError>;

    /// Driver loop: drain queue, run transition, commit on accept. Single-thread internal.
    pub async fn run(&self) -> Result<(), SequencerError>;

    /// Per-tx critical section.
    fn apply_one(&self, tx: TypedTx) -> Result<LedgerEntry, TransitionError> {
        // 1. Snapshot Q_t under read lock
        let q_snapshot = self.q.read().clone();

        // 2. Dispatch (pure)
        let (q_next, _signals) = dispatch_transition(&q_snapshot, &tx, &self.predicate_registry, &self.tool_registry)?;
        // **K1**: if step returns Err, EARLY RETURN — no logical_t assigned, no entry committed.

        // 3. Put payload to CAS (DIV-5 5-param signature)
        let mut cas_w = self.cas.write();
        let cas_bytes = canonical_serialize(&tx);  // bincode v2 per § 2.5 of STATE spec
        let payload_cid = cas_w.put(
            &cas_bytes,
            ObjectType::ProposalPayload,  // R2 fix: shipped CAS variant (NOT Transition)
            &format!("sequencer-{}", self.epoch.get()),
            self.next_logical_t.load(Ordering::SeqCst) + 1,  // tentative; final below
            Some("LedgerEntrySigningPayload.v1".to_string()),
        )?;
        drop(cas_w);

        // 4. **K1**: assign logical_t ONLY now (post-accept)
        let logical_t = self.next_logical_t.fetch_add(1, Ordering::SeqCst) + 1;

        // 5. Build LedgerEntrySigningPayload
        let signing_payload = LedgerEntrySigningPayload {
            logical_t,
            parent_state_root: q_snapshot.state_root_t,
            parent_ledger_root: q_snapshot.ledger_root_t,   // K2 transplant defense
            tx_kind: TxKind::from_typed(&tx),
            tx_payload_cid: payload_cid,
            resulting_state_root: q_next.state_root_t,
            timestamp_logical: logical_t,
            epoch: self.epoch,
            extensions: BTreeMap::new(),                     // G1 empty in v1
        };

        // 6. **C3 NEW SIGN API (v1.2)**: typed sign through CanonicalMessage extension.
        // Compute payload digest in transition_ledger; pass opaque [u8; 32] to emitter.
        let signing_payload_digest = signing_payload.canonical_digest();
        let system_signature = transition_ledger_emitter::sign_ledger_entry(
            &self.keypair,
            signing_payload_digest.0,
        )?;

        // 7. Compute resulting_ledger_root via append() (pure)
        let signing_digest = signing_payload.canonical_digest();
        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);

        // 8. Build LedgerEntry (the stored record)
        let entry = LedgerEntry {
            logical_t: signing_payload.logical_t,
            parent_state_root: signing_payload.parent_state_root,
            parent_ledger_root: signing_payload.parent_ledger_root,
            tx_kind: signing_payload.tx_kind,
            tx_payload_cid: signing_payload.tx_payload_cid,
            resulting_state_root: signing_payload.resulting_state_root,
            resulting_ledger_root,                            // derived; not in signed digest
            timestamp_logical: signing_payload.timestamp_logical,
            epoch: signing_payload.epoch,
            extensions: signing_payload.extensions,
            system_signature,
        };

        // 9. Commit + mutate Q_t under write lock (atomic)
        let mut q_w = self.q.write();
        let mut writer_w = self.ledger_writer.write();
        writer_w.commit(&entry)?;                              // K4 returns Hash; matches skeleton
        drop(writer_w);
        *q_w = q_next;
        q_w.ledger_root_t = entry.resulting_ledger_root;
        // **K3 (v1.2 revised)**: do NOT mutate q_w.head_t here. v1.x ledger owns
        // `ledger_root_t` only. head_t mutation is **deferred to CO1.7.5+ wiring**
        // (when `Git2LedgerWriter::commit` is implemented and can return commit_sha
        // alongside Hash). Until then, head_t remains at QState placeholder; replay
        // and chain-integrity tests do NOT depend on head_t.

        Ok(entry)
    }
}
```

**Why dual counter (K1)**: rejection of a submission must NOT consume a logical_t, because (a) skeleton's `InMemoryLedgerWriter::commit` enforces `expected_logical_t = len + 1` and would reject a gap; (b) replay enforces `entry.logical_t == (i+1)` and would reject a gap. Submitter IDs (`submit_id`) are returned from `submit()` immediately for receipt; logical_t is observable only on the committed entry.

**Why no head_t mutation in apply_one (K3, revised v1.2)**: v1.x CO1.7 owns `ledger_root_t` only. CO1.8 owns `state_root_t`. **head_t mutation deferred to CO1.7.5+** when `Git2LedgerWriter` provides a commit_sha return alongside Hash; the InMemoryLedgerWriter used by the v1 skeleton has no commit_sha to expose, so the trait keeps a single `Hash` return and head_t wiring is a separate downstream concern. Sequencer never calls `NodeId::from_state_root(...)`.

**Q3 (Gemini)**: `Sequencer` vs `LedgerWriter + OrderingCoordinator` split — v1.1 keeps `Sequencer` as the abstraction; trait-segregation refactor is a v4.x consideration (the current single-writer constraint per § 5.2.1 makes the split synthetic for v1).

---

## § 4 append() + replay() — two-mode (per C1)

```rust
/// Pure. Same (parent_root, signing_digest) → byte-identical new_root.
/// No I/O, no clock, no env. Witness for I-DET / I-DETHASH ledger axis.
pub fn append(parent_root: &Hash, signing_digest: &Hash) -> Hash {
    let mut h = Sha256::new();
    h.update(b"turingosv4.ledger_root.v1");      // domain separation
    h.update(parent_root.0);
    h.update(signing_digest.0);
    Hash(h.finalize().into())
}

/// Replay mode (C1).
pub enum ReplayMode {
    /// Skeleton-stage: validates parent_state_root + parent_ledger_root + ledger_root chain.
    /// Does NOT verify signatures, re-fetch payloads, or re-run pure transitions.
    /// Trust mode: "trust the sequencer". v1 deliverable.
    ChainOnly,
    /// CO1.7.5+ stage: full re-execution.
    /// Verifies signatures via CanonicalMessage; fetches payloads from CAS;
    /// re-runs pure dispatch_transition; compares resulting_state_root.
    /// **THIS** is the I-DETHASH witness (I-DETHASH bound to FullTransition only).
    /// Requires CO1.4-extra (CAS index persistence) for cold-restart.
    FullTransition,
}

/// Skeleton-stage entry point (v1).
pub fn replay_chain_integrity(
    genesis_state_root: Hash,
    genesis_ledger_root: Hash,
    entries: &[LedgerEntry],
) -> Result<(Hash, Hash), ReplayError>;

/// CO1.7.5+ stage entry point (v1.1 spec only; impl deferred).
pub fn replay_full_transition(
    genesis: &QState,
    entries: &[LedgerEntry],
    cas: &dyn LedgerCasView,
    pinned_pubkeys: &PinnedSystemPubkeys,
) -> Result<QState, ReplayError>;
```

**I-DETHASH witness (revised per C1)**: `replay_full_transition` is the I-DETHASH witness. `replay_chain_integrity` is necessary-but-not-sufficient — passing chain check does NOT prove transition determinism. v1 documents this explicitly to close trust ambiguity.

**ReplayError enum** (skeleton already has 3 variants; v1.1 adds 4 more for FullTransition):
- `LogicalTGap { at, expected, got }` (existing)
- `ParentMismatch { at }` (existing; covers parent_state_root)
- `LedgerRootMismatch { at }` (existing)
- `ParentLedgerRootMismatch { at }` (NEW K2)
- `BadSignature { at }` (NEW; FullTransition only)
- `CasMissing { at, cid }` (NEW; FullTransition only — fires if CO1.4-extra not yet landed)
- `StateRootMismatch { at }` (NEW; FullTransition only)
- `TransitionError { at, inner }` (NEW; wraps dispatch_transition errors)

---

## § 5 Storage backend

**Choice**: git2-rs commit chain (Path B substrate, ratified per Const Art 0.4 + WP § 5.L4).

**Mapping**:
- One `LedgerEntry` = one git commit on `refs/transitions/main`.
- Commit message = canonical-serialized `LedgerEntry` (bincode v2 per `STATE_TRANSITION_SPEC § 2.5`).
- Commit tree = `(payload_cid_blob, signature_blob)` (state_root NOT a tree blob — per K3, L5 owns state_root materialization).
- **K3 (v1.2)**: `head_t = NodeId(commit_sha)` is the canonical convention WHEN head_t is wired (CO1.7.5+). v1.x sequencer does NOT mutate head_t — `Git2LedgerWriter` is needed to surface commit_sha. `NodeId::from_state_root(...)` is NOT used by L4 in any version.
- **C2**: cold-replay availability requires `CasStore` index persistence; deferred to CO1.4-extra. Until then, full-mode replay errors with `CasMissing` if CAS state is not warm.
- Genesis: `refs/transitions/main` is created at the empty-tree commit corresponding to `genesis_payload.toml` (CO1.0). `genesis_ledger_root_t = sha256("turingosv4.ledger_root.v1.genesis" || sha256(genesis_payload.toml))` — **Q7 resolution** (NOT `Hash::ZERO`; both auditors agreed).

**LedgerWriter trait (K4 reconciled to skeleton)**:

```rust
pub trait LedgerWriter: Send + Sync {
    /// Commit a signed LedgerEntry. K4: `&mut self` + `Hash` return matches skeleton.
    /// Returns the entry's `resulting_ledger_root`.
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;

    /// Read entry at a specific 1-indexed `logical_t`.
    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;

    /// Total accepted entries (highest assigned logical_t; 0 at genesis).
    fn len(&self) -> u64;

    // K4: iter_from() deferred — used only by FullTransition replay; CO1.7.5+ adds it.
}
```

**Implementation (CO1.7.5+)**: `Git2LedgerWriter` (built on existing CO1.4 CAS); skeleton `InMemoryLedgerWriter` for v1 testing.

**Why git2-rs not gix**: Const Art 0.4 ratified path B (gix→git2-rs pivot per CO1.3.1 spike 8/8 PASS).

---

## § 6 Invariants enforced by CO1.7

| ID | Invariant | Enforced where in CO1.7 |
|---|---|---|
| **I-DET** | Same (Q_t, tx) → byte-identical (Q_{t+1}, signals) | sequencer.apply_one stages 2-7 (pure dispatch + deterministic append) |
| **I-DETHASH** | replay_full_transition(genesis, entries) recovers live state_root | **Bound to FullTransition mode only** (C1); skeleton ChainOnly is necessary-but-not-sufficient |
| **I-LOGTIME** | timestamp_logical strictly monotonic; no wall clock | sequencer apply_one stage 4; LedgerEntry has no wall-clock field |
| **I-FINALIZE-BATCH-ORDER** | When N claims expire same logical_t, finalize order = `(expires_at_logical ASC, claim_id ASC)` | sequencer enqueues finalize tx in order before resuming work tx; per § 5.2.3 |
| **I-FINALIZE-EXCLUSIVE** | finalize_reward and slash mutually exclusive per claim | **v4 has no Slash** (K5); invariant trivially holds via TxKind enum |
| **I-NOSIDE** | step_transition reads only (q, tx, registries) | append() and replay_* are pure; sequencer.apply_one isolates I/O to CAS put + writer commit |
| **I-NOENV** | step_transition dependency tree has no `std::env` access | grep test in CO1.7 module — already enforced by CLAUDE.md hardcoded-config rule (C-027) |
| **I-NORANDOM** | tx consuming randomness MUST seed PRNG from `(tx.tx_id, q.state_root_t)` | LedgerEntry.system_signature uses keypair (deterministic); no entropy in append/replay |

CO1.7 does NOT introduce new invariants — provides machine-checkable witnesses for 8 of the 27 frozen invariants.

---

## § 7 Conformance tests (K7 staged)

| Test | Stage | What it asserts |
|---|---|---|
| `tests/append_byte_stable` | skeleton (v1) | append byte-stable across calls (I-DET ledger axis) |
| `tests/canonical_digest_stable` | skeleton (v1) | LedgerEntrySigningPayload digest stable across clones; #[repr(u8)] discriminant stable |
| `tests/inmemory_writer_logical_t` | skeleton (v1) | InMemoryLedgerWriter rejects logical_t gaps |
| `tests/replay_chain_integrity_clean` | skeleton (v1) | clean ChainOnly replay returns final state_root + ledger_root |
| `tests/replay_chain_rejects_parent_state_root_tamper` | skeleton (v1) | ChainOnly replay rejects parent_state_root tamper |
| `tests/replay_chain_rejects_parent_ledger_root_tamper` | **K2 NEW** skeleton (v1) | ChainOnly replay rejects parent_ledger_root tamper (transplant defense) |
| `tests/replay_chain_rejects_ledger_root_tamper` | skeleton (v1) | ChainOnly replay rejects resulting_ledger_root tamper |
| `tests/canonical_digest_excludes_derivatives` | **Q9 NEW** skeleton (v1) | LedgerEntrySigningPayload.canonical_digest excludes resulting_ledger_root + system_signature; mutation of either does NOT change digest |
| `tests/replay_full_transition_state_root` | CO1.7.5+ (post-CO1.4-extra) | FullTransition replay re-runs dispatch_transition; asserts state_root match (I-DETHASH witness) |
| `tests/system_signature_verifies_via_canonical_message` | CO1.7.5+ | LedgerEntry.system_signature verifies through `verify_system_signature(&CanonicalMessage::LedgerEntrySigning(...), epoch, pinned_pubkeys)` |
| `tests/cas_payload_round_trip` | CO1.7.5+ (after CO1.4-extra) | put→get round trip; CID stability across runs |
| `tests/sequencer_serial_replay_byte_identity` | CO1.7.5+ | submit 100 tx; replay → byte-identical state_root |

**v1 stage (skeleton)**: 8 tests (6 already in skeleton + 2 NEW K2/Q9). **CO1.7.5+ stage**: 4 more.

---

## § 8 dispatch_transition (K5 Slash dropped)

```rust
pub(crate) fn dispatch_transition(
    q: &QState,
    tx: &TypedTx,
    predicate_registry: &PredicateRegistry,
    tool_registry: &ToolRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {
    match tx {
        TypedTx::Work(t)             => step_transition(q, t, predicate_registry, tool_registry),
        TypedTx::Verify(t)           => verify_transition(q, t, predicate_registry),
        TypedTx::Challenge(t)        => challenge_transition(q, t, predicate_registry),
        TypedTx::Reuse(t)            => reuse_transition(q, t, tool_registry),
        TypedTx::FinalizeReward(t)   => finalize_reward_transition(q, t),
        TypedTx::TaskExpire(t)       => task_expire_transition(q, t),
        TypedTx::TerminalSummary(t)  => emit_terminal_summary(q, t),
        // K5: NO `TypedTx::Slash` — v4 has no slash transition.
    }
}
```

**Q5 resolution** (Gemini): enum-match for v4 (exhaustive, deterministic, simple); defer `MetaTransitionInterface` trait pattern to v4.1 dynamic MetaTx.

---

## § 9 STEP_B disposition (K4 corrected typo)

CO1.7 lives in NEW files (`src/bottom_white/ledger/transition_ledger.rs`, future `src/state/sequencer.rs`). It does NOT modify `src/bus.rs` / `src/kernel.rs` / `src/wallet.rs` (the STEP_B-restricted files per CLAUDE.md "Code Standard"; v1 incorrectly listed `wal.rs`). Therefore: **no STEP_B parallel-branch ceremony required** for the CO1.7 atom itself.

**Touched files** (skeleton commit, additive only):
- `src/bottom_white/ledger/transition_ledger.rs` (NEW, ~370 lines)
- `src/bottom_white/ledger/mod.rs` (existing, +1 `pub mod` line — additive)
- `genesis_payload.toml` (TR manifest +1 entry, refreshed mod.rs hash — TR governance, not code edit)

Future runtime wiring (CO1.7.5+) into `bus.rs`/`kernel.rs` WILL need STEP_B — that's a separate atom. The retirement of `src/ledger.rs` (legacy top-level) is in CO1.1.5 per `STATE_TRANSITION_SPEC § 5.3`.

CO1.7 also extends `src/bottom_white/ledger/system_keypair.rs` (CanonicalMessage variant + sign API); that file is NOT STEP_B-restricted and the change is additive (no behavior change to existing 3 variants).

---

## § 10 What this spec does NOT specify

1. **Garbage collection** — append-only constitutional (Art 0.2); L4 entry deletion never happens.
2. **Cross-cell sharing** — § 5.2.2 mandates disjoint runtime_repo per cell; multi-tenant deployments are v4.x.
3. **Recovery from corrupted git history** — out of scope; if `git fsck` fails, runtime aborts (fail-closed).
4. **Performance tuning** — no SLO commitments. Sequencer single-writer property + CAS metadata write per entry sets approximate throughput floor; rough budget is "≥ 10 tx/sec per cell on 4-core hardware".
5. **CAS index persistence** — deferred to **CO1.4-extra** (NEW atom; not yet planned in Plan v3.2). C2 mitigation route.
6. **Cold-restart full replay** — depends on CO1.4-extra; until then, FullTransition mode errors with `CasMissing` after process restart. ChainOnly mode unaffected.

---

## § 11 Open questions resolution (post-round-1)

| Q | v1 status | v1.1 resolution | Source |
|---|---|---|---|
| Q1 SubmissionQueue type | open | **Bounded `tokio::sync::mpsc::Sender`** with `QueueFull` Err on saturation (Codex Q-G). Async wait variant `submit_async` may be added if multi-agent fairness becomes an issue (deferred). |
| Q2 back-pressure | open | Returns `Err(SubmitError::QueueFull)` immediately (Codex). Agent retry with deterministic exponential backoff (seed from `(agent_id, attempt_count)`). |
| Q3 Sequencer abstraction split | open | **Keep monolithic `Sequencer`** for v1.1 (single-writer constraint makes split synthetic). Trait segregation = v4.x consideration. |
| Q4 signature placement | open | **Inside `LedgerEntry`** (v1 design); BUT signing target is `LedgerEntrySigningPayload` (separate struct, distinct fields). Both auditors agreed. |
| Q5 enum-match vs trait dispatch | open | **Enum-match** for v4 (exhaustive, simple). `MetaTransitionInterface` trait deferred to v4.1 dynamic MetaTx. |
| Q6 replay error mode | open | **Reject on first error** (current). Diagnostic-collection mode is a v4.x extension; first-error simplicity matches CO1.7.5 implementation budget. |
| Q7 genesis ledger_root_t | open | **`sha256("turingosv4.ledger_root.v1.genesis" || sha256(genesis_payload.toml))`** — domain-separated anchor (both auditors agreed). NOT `Hash::ZERO`. |
| Q8 CanonicalMessage extension | open | **Path A**: extend enum with `LedgerEntrySigning(LedgerEntrySigningPayload)` variant; new typed sign API `sign_ledger_entry(payload, epoch)`. Forward-compat clause: future ledger-side message variants add new `CanonicalMessage::*` variants (NOT in-place edits). Both auditors agreed. |
| Q9 canonical_digest exclusion | open | **Excludes**: `resulting_ledger_root` (cycle), `system_signature` (its input). **Includes**: 9 fields explicit in `LedgerEntrySigningPayload`. Spec § 1.1 explicit. |
| Q10 epoch field | open | **Added** to `LedgerEntry` field 9; **bound** in `LedgerEntrySigningPayload` (D1 conservative resolution: Codex security argument). NOT separately folded into ledger_root. |
| Q11 NEW open Qs | — | Codex round-1 listed: parent_ledger_root binding (now K2 / done), rejected-submission logical time (now K1 / done), CAS persistence (now C2 → CO1.4-extra), canonical fixtures (deferred to CO1.7.5+ test stubs), L4/L5 head_t ownership (now K3 / done). All addressed. |

**v1.1 closed all 11 open Qs from v1**. Round-2 audit's open-Q section starts empty.

---

## § 12 Audit gates (round structure)

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 | CHALLENGE (high) | CHALLENGE (high) | **CHALLENGE** | v1.1 patch round (this version) |
| 2 | ⏳ pending | ⏳ pending | TBD | re-audit on v1.1; expected PASS or 1-issue CHALLENGE |
| 3+ | … | … | … | iterate to PASS/PASS |

**Pre-implementation gate**: CO1.7 must reach `PASS/PASS` before implementing CO1.7.5 (transition function bodies) + CO1.4-extra (CAS persistence). Sedimented per CLAUDE.md "Audit Standard" (Generator ≠ Evaluator) + memory `feedback_dual_audit`.

---

## § 13 Estimated scope (revised)

- **Spec rounds**: round-1 done; round-2 expected; possible round-3 if Codex finds new edge cases.
- **Implementation scope** (post-PASS/PASS):
  - CO1.7-impl proper: ~600-900 LoC + 8 conformance tests (4 skeleton-stage + 4 CO1.7.5-stage)
  - CO1.4-extra (CAS index persistence): ~150-300 LoC + 3-4 tests (NEW atom; budget add)
  - CO1.7.5 (transition function bodies): separate downstream atom
- **Total atom budget**: ~1.5-2.5 weeks (slight expansion due to CO1.4-extra; matches LATEST line 92 estimate).

---

## § 14 Honest acknowledgements (v1.1)

1. CO1.4 CAS API surface verified via type-skeleton smoke 2026-04-28: `CasStore::get(&Cid) → Result<Vec<u8>, CasError>` matches; `CasStore::put` 5-param signature documented (DIV-5).
2. SubmissionQueue choice = bounded tokio::sync::mpsc per Q1 resolution; pivot to different runtime would rewrite `Sequencer.run()`.
3. system_keypair extension (CanonicalMessage variant + sign_ledger_entry API) is additive Wave 4-B extension; non-breaking; verified via type-skeleton (sign API call site is concrete, not unimplemented).
4. **v1.1 patches incorporate 11 round-1 must-fix items + 1 disagreement resolution** (D1: epoch security wins). Detailed in patch log header.
5. **Spec ↔ skeleton divergences cataloged in v1; resolution status v1.1**:
   - DIV-1 CanonicalMessage integration — **resolved** via Q8 (extend enum, Path A) + § 1.2.
   - DIV-2 Q_t mutation API — unchanged status (CO P2.x economy atoms still unblock); skeleton stays unimplemented!() for state mutation.
   - DIV-3 epoch field — **resolved**: in LedgerEntry + bound in LedgerEntrySigningPayload.
   - DIV-4 CasReader → LedgerCasView — **resolved**: narrow trait for replay_full_transition.
   - DIV-5 CasStore::put 5-param — **resolved**: sequencer apply_one stage 3 builds full metadata.
6. **Q9 spec bug found by skeleton smoke** — closed by spec § 1.1: explicit `LedgerEntrySigningPayload` separate struct excludes derivatives.
7. **K2 transplant attack vector** found by Codex round-1 — closed by binding `parent_ledger_root` in signing payload.
8. **K3 L4/L5 boundary blur** found by Codex round-1 — closed by spec § 0 + § 3 + § 5 boundary clarification.

---

## § 15 Pre-audit smoke verification (2026-04-28)

| Smoke item | Result | What it proved |
|---|---|---|
| `cargo check` on `src/bottom_white/ledger/transition_ledger.rs` | PASS | LedgerEntry / TxKind / append / replay_chain_integrity / InMemoryLedgerWriter all type-check against shipped CO1.4 + CO1.7.0a-f + Q_t types |
| `cargo test --lib bottom_white::ledger::transition_ledger::` | 6/6 PASS | append byte-stable; canonical_digest stable; in-memory writer enforces logical_t monotonic; ChainOnly replay validates parent chain + rejects 2 tamper modes |
| `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` | PASS | TR manifest aligned with skeleton + spec |
| `cargo test --lib` (full workspace) | 196/0 PASS | no regression in 190 pre-existing tests |

**v1.1 skeleton update plan** (separate commit): apply K6 (#[repr(u8)]), K5 (drop Slash), G1 (extensions field), C3 (sign through CanonicalMessage extension), K2 (parent_ledger_root field + new sign payload struct), Q9 (canonical_digest moves to LedgerEntrySigningPayload), K7 (add 2 new tests for parent_ledger_root tamper + digest exclusion). Target: 8 skeleton tests PASS, full workspace still 196+/0 PASS.

---

## § 16 Round-1 audit closure verification

| Audit finding | Closure mechanism | v1.1 location |
|---|---|---|
| C1 replay two-mode | New `ReplayMode` enum + spec § 4 + I-DETHASH bound to FullTransition only | § 0, § 4, § 6 |
| C2 CAS cold-replay risk | New CO1.4-extra atom + § 0 explicit dependency note + ReplayError::CasMissing | § 0, § 5, § 13 |
| C3 signing primitive integration | LedgerEntrySigningPayload struct + CanonicalMessage extension + sign_ledger_entry API | § 1.1, § 1.2 |
| K1 sequencer logical_t skip race | Dual counter design (next_submit_id, next_logical_t) | § 3 |
| K2 parent_ledger_root binding | Field added + bound in signing payload + new test | § 1, § 1.1, § 7 |
| K3 L4/L5 head_t ownership | Boundary clarified: CO1.7 owns ledger_root + commit-chain head_t (NodeId(commit_sha)); CO1.8 owns state_root | § 0, § 3, § 5 |
| K4 trait mismatch | Spec aligned to skeleton: `&mut self` + `Hash` return; iter_from deferred | § 5 |
| K5 Slash dispatch gap | Slash variant DROPPED for v4; deferred to CO P2.5 | § 1, § 6, § 8 |
| K6 #[repr(u8)] | Added with explicit discriminants | § 1 |
| K7 conformance test gap | Explicit 8 tests (4 skeleton + 4 CO1.7.5+ stage) | § 7 |
| G1 forward-compat extensions | `extensions: BTreeMap<String, Vec<u8>>` in LedgerEntry; bound in signing payload | § 1, § 1.1 |
| D1 epoch binding (Codex/Gemini disagree) | Conservative resolution: bound in signing payload (Codex security wins) | § 1.1 |

**12 closures** (11 must-fix + 1 disagreement) integrated. Round-2 audit input minimal: residual issues only.

— ArchitectAI, session 2026-04-28; round-1 closure 2026-04-28.


---

# XREF: shipped src/state/sequencer.rs (D2 target)

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

# XREF: shipped src/bottom_white/ledger/transition_ledger.rs (D2 trait target)

```rust
//! L4 Transition Ledger (CO1.7) — implementation atom.
//!
//! TRACE_MATRIX FC2-Append: canonical envelope appended to L4 once a transition is accepted.
//! TRACE_MATRIX WP § 5.L4: ChainTape Layer 4 spine; one LedgerEntry per accepted transition.
//! TRACE_MATRIX § 1-§ 8 (CO1_7_TRANSITION_LEDGER_v1_2026-04-28 v1.2): schema +
//! append() + replay_chain_integrity() + replay_full_transition() + Git2LedgerWriter.
//!
//! **Status**: CO1.7 spec PASS/PASS (3 rounds) + CO1.7-impl bundle PASS/PASS
//! (3 rounds: A1+A2+A3+A4 + CO1.4-extra). Per-kind transition function bodies
//! deferred to CO1.7.5 (NotYetImplemented stubs in `src/state/sequencer.rs`).
//!
//! v1 → v1.1 changes (smoke for round-2 dual audit):
//! - C1: two-mode replay enum (ChainOnly v1; FullTransition CO1.7.5+); skeleton now
//!   exposes `replay_chain_integrity` only (renamed for honesty).
//! - K1: sequencer dual-counter design — documented in spec § 3; skeleton has no
//!   sequencer code (deferred to CO1.7.5).
//! - K2: `parent_ledger_root: Hash` field added + bound in signing payload (transplant
//!   defense); new test asserts replay rejects parent_ledger_root tamper.
//! - K3: L4/L5 boundary clarified — CO1.7 owns ledger_root + commit-chain head_t;
//!   CO1.8 owns state_root mutation. Skeleton reflects boundary (no state_root mutation).
//! - K5: `TxKind::Slash` DROPPED for v4 (deferred to CO P2.5).
//! - K6: `#[repr(u8)]` + explicit discriminants on TxKind.
//! - K7: +2 conformance tests (parent_ledger_root tamper, digest exclusion).
//! - G1: `extensions: BTreeMap<String, Vec<u8>>` forward-compat field (empty in v1).
//! - C3 / Q8: signing target is `LedgerEntrySigningPayload` (separate struct) ready to
//!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
//!   `system_keypair` (Wave 4-B additive extension). Skeleton has the payload struct
//!   + canonical_digest method; the actual CanonicalMessage extension is deferred.
//! - Q9: canonical_digest now lives on LedgerEntrySigningPayload, not LedgerEntry —
//!   structurally enforces "derivatives excluded".
//! - D1: epoch is bound in signing payload (Codex security wins over Gemini orthogonality).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
use crate::state::q_state::Hash;

// ────────────────────────────────────────────────────────────────────────────
// § 1 LedgerEntry — the stored record (11 fields per v1.1)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append: discriminator for the typed payload behind a CAS Cid.
/// **K6**: `#[repr(u8)]` + explicit discriminants for stable cast in canonical digest.
/// **K5**: NO `Slash` variant — ChallengeCourt slash event deferred to CO P2.5 atom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TxKind {
    Work            = 0,
    Verify          = 1,
    Challenge       = 2,
    Reuse           = 3,
    FinalizeReward  = 4,
    TaskExpire      = 5,
    TerminalSummary = 6,
}

/// TRACE_MATRIX FC2-Append + WP § 5.L4: stored LedgerEntry record (11 fields).
///
/// Distinct from `LedgerEntrySigningPayload`: this is the FULL stored record
/// (includes derivatives + signature); the signing payload is the subset that
/// the system keypair attests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntry {
    /// **K1**: assigned ONLY at commit (sequencer dual-counter design); rejected
    /// submissions never get a logical_t.
    pub logical_t: u64,                          //  1
    pub parent_state_root: Hash,                 //  2
    /// **K2 NEW**: parent_ledger_root before fold; bound in signed payload to
    /// prevent transplant attacks.
    pub parent_ledger_root: Hash,                //  3
    pub tx_kind: TxKind,                         //  4
    /// CAS handle (CO1.4) to canonical-serialized payload (DIV-5 5-param put).
    pub tx_payload_cid: Cid,                     //  5
    /// Resulting state_root post-transition (NOT mutated by L4 — accepted as
    /// returned by transition function per K3 boundary).
    pub resulting_state_root: Hash,              //  6
    /// Resulting ledger_root after fold. Derivative; NOT in signed digest.
    pub resulting_ledger_root: Hash,             //  7
    pub timestamp_logical: u64,                  //  8
    /// **D1 / Q10**: epoch bound in signed payload (Codex security wins).
    pub epoch: SystemEpoch,                      //  9
    /// **G1 NEW**: forward-compat extension map. Empty in v1; reserved for v4.x.
    /// Bound in signed payload (G1 cannot bypass signature).
    pub extensions: BTreeMap<String, Vec<u8>>,   // 10
    /// Detached system signature over `LedgerEntrySigningPayload.canonical_digest()`.
    pub system_signature: SystemSignature,       // 11
}

// ────────────────────────────────────────────────────────────────────────────
// § 1.1 LedgerEntrySigningPayload — the signed bytes (NEW per C3 / Q9)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append C3: the bytes the system keypair actually signs.
///
/// **Excludes** (Q9 cycle prevention):
/// - `resulting_ledger_root` (derivative; including → cycle)
/// - `system_signature` (its own input)
///
/// **Includes** (9 non-derivative bound fields). Domain-separation prefix is
/// part of the digest to prevent cross-namespace collision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntrySigningPayload {
    pub logical_t: u64,
    pub parent_state_root: Hash,
    pub parent_ledger_root: Hash,                  // K2
    pub tx_kind: TxKind,
    pub tx_payload_cid: Cid,
    pub resulting_state_root: Hash,
    pub timestamp_logical: u64,
    pub epoch: SystemEpoch,                        // D1
    pub extensions: BTreeMap<String, Vec<u8>>,     // G1
}

impl LedgerEntrySigningPayload {
    /// Canonical SHA-256 digest. Stable wire format (NOT bincode/serde dependent).
    pub fn canonical_digest(&self) -> Hash {
        let mut h = Sha256::new();
        h.update(b"turingosv4.ledger_entry_signing.v1");
        h.update(self.logical_t.to_be_bytes());
        h.update(self.parent_state_root.0);
        h.update(self.parent_ledger_root.0);
        h.update((self.tx_kind as u8).to_be_bytes()); // K6 #[repr(u8)] makes cast stable
        h.update(self.tx_payload_cid.0);
        h.update(self.resulting_state_root.0);
        h.update(self.timestamp_logical.to_be_bytes());
        h.update(self.epoch.get().to_be_bytes());
        // Extensions: BTreeMap iterates in lex key order (deterministic);
        // length-prefix every field to prevent ambiguity attacks.
        h.update((self.extensions.len() as u64).to_be_bytes());
        for (k, v) in &self.extensions {
            h.update((k.len() as u64).to_be_bytes());
            h.update(k.as_bytes());
            h.update((v.len() as u64).to_be_bytes());
            h.update(v);
        }
        Hash(h.finalize().into())
    }
}

impl LedgerEntry {
    /// Project the LedgerEntry's signed-fields-subset back into a signing payload.
    /// Used by replay to recompute `signing_digest` and re-verify chain integrity.
    pub fn to_signing_payload(&self) -> LedgerEntrySigningPayload {
        LedgerEntrySigningPayload {
            logical_t: self.logical_t,
            parent_state_root: self.parent_state_root,
            parent_ledger_root: self.parent_ledger_root,
            tx_kind: self.tx_kind,
            tx_payload_cid: self.tx_payload_cid,
            resulting_state_root: self.resulting_state_root,
            timestamp_logical: self.timestamp_logical,
            epoch: self.epoch,
            extensions: self.extensions.clone(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 4 append() — pure ledger-root fold
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append + spec § 4: pure ledger-root fold over signed digests.
/// Same `(parent_root, signing_digest)` → byte-identical `new_root`.
/// No I/O, no clock, no env. Witness for I-DET ledger axis.
pub fn append(parent_root: &Hash, signing_digest: &Hash) -> Hash {
    let mut h = Sha256::new();
    h.update(b"turingosv4.ledger_root.v1");
    h.update(parent_root.0);
    h.update(signing_digest.0);
    Hash(h.finalize().into())
}

// ────────────────────────────────────────────────────────────────────────────
// LedgerWriter trait (K4 reconciled to skeleton signature)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append: storage abstraction for L4.
/// Production impl is `Git2LedgerWriter` (CO1.7.5+; refs/transitions/main commit chain).
/// Test/skeleton impl is `InMemoryLedgerWriter` below.
///
/// **K4**: signature `commit(&mut self) → Hash` (NOT `&self → NodeId`); `iter_from`
/// deferred to CO1.7.5+ (only used by FullTransition replay; not v1 deliverable).
pub trait LedgerWriter: Send + Sync {
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
    fn len(&self) -> u64;
}

#[derive(Debug)]
pub enum LedgerWriterError {
    LogicalTGap { expected: u64, got: u64 },
    NotFound { logical_t: u64 },
    BackendCorruption(String),
}

impl std::fmt::Display for LedgerWriterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LogicalTGap { expected, got } => {
                write!(f, "logical_t gap: expected {expected}, got {got}")
            }
            Self::NotFound { logical_t } => write!(f, "no entry at logical_t={logical_t}"),
            Self::BackendCorruption(msg) => write!(f, "backend corruption: {msg}"),
        }
    }
}
impl std::error::Error for LedgerWriterError {}

/// In-memory test/skeleton writer; Vec backing strict logical_t enforced at commit.
#[derive(Debug, Default)]
pub struct InMemoryLedgerWriter {
    entries: Vec<LedgerEntry>,
}

impl InMemoryLedgerWriter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl LedgerWriter for InMemoryLedgerWriter {
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
        let expected = (self.entries.len() as u64) + 1;
        if entry.logical_t != expected {
            return Err(LedgerWriterError::LogicalTGap {
                expected,
                got: entry.logical_t,
            });
        }
        let root = entry.resulting_ledger_root;
        self.entries.push(entry.clone());
        Ok(root)
    }

    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
        if logical_t == 0 || logical_t > self.entries.len() as u64 {
            return Err(LedgerWriterError::NotFound { logical_t });
        }
        Ok(self.entries[(logical_t - 1) as usize].clone())
    }

    fn len(&self) -> u64 {
        self.entries.len() as u64
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 4 replay — TWO-MODE per C1
// ────────────────────────────────────────────────────────────────────────────

/// **C1 NEW**: replay mode discriminator.
/// - `ChainOnly`: skeleton-stage; chain integrity only (parent_state_root +
///   parent_ledger_root + ledger_root chain). NOT the I-DETHASH witness.
/// - `FullTransition`: CO1.7.5+ stage; verifies signatures + re-fetches payloads
///   from CAS + re-runs pure transitions + asserts state_root match. THE
///   I-DETHASH witness; requires CO1.4-extra (CAS index persistence).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayMode {
    ChainOnly,
    FullTransition,
}

#[derive(Debug)]
pub enum ReplayError {
    LogicalTGap { at: usize, expected: u64, got: u64 },
    ParentStateMismatch { at: usize },
    ParentLedgerMismatch { at: usize }, // K2 NEW
    LedgerRootMismatch { at: usize },
    // FullTransition-mode-only (CO1.7.5+):
    BadSignature { at: usize },
    CasMissing { at: usize },
    StateRootMismatch { at: usize },
    /// CO1.7-impl A4: dispatch_transition rejected the re-run. In stub state
    /// (CO1.7.5 not yet shipped), this fires on every replay step with
    /// `inner = NotYetImplemented`.
    Transition {
        at: usize,
        inner: crate::state::typed_tx::TransitionError,
    },
    /// CO1.7-impl A4 v1.1 (Codex bundle Q-J / C-3): the canonical-decoded
    /// `TypedTx` variant disagrees with the entry's `tx_kind` discriminator.
    /// Signed envelope claims one kind; CAS payload is another.
    TxKindMismatch {
        at: usize,
        envelope_kind: TxKind,
        decoded_kind: TxKind,
    },
    /// CO1.7-impl A4 v1.1 (Codex bundle Q-K secondary): payload bytes
    /// retrieved from CAS but `canonical_decode` failed (corruption /
    /// non-canonical bytes). Distinct from `CasMissing` (lookup failure).
    PayloadDecode {
        at: usize,
        reason: String,
    },
}

impl std::fmt::Display for ReplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LogicalTGap { at, expected, got } => {
                write!(f, "logical_t gap at index {at}: expected {expected}, got {got}")
            }
            Self::ParentStateMismatch { at } => write!(f, "parent_state_root mismatch at index {at}"),
            Self::ParentLedgerMismatch { at } => write!(f, "parent_ledger_root mismatch at index {at}"),
            Self::LedgerRootMismatch { at } => write!(f, "ledger_root mismatch at index {at}"),
            Self::BadSignature { at } => write!(f, "system_signature verify failed at index {at}"),
            Self::CasMissing { at } => write!(f, "CAS payload not retrievable at index {at}"),
            Self::StateRootMismatch { at } => write!(f, "resulting_state_root divergence at index {at}"),
            Self::Transition { at, inner } => write!(f, "dispatch_transition rejected at index {at}: {inner}"),
            Self::TxKindMismatch { at, envelope_kind, decoded_kind } => write!(
                f,
                "tx_kind mismatch at index {at}: envelope claims {envelope_kind:?} but CAS payload decoded as {decoded_kind:?}"
            ),
            Self::PayloadDecode { at, reason } => write!(f, "payload canonical_decode failed at index {at}: {reason}"),
        }
    }
}
impl std::error::Error for ReplayError {}

// ────────────────────────────────────────────────────────────────────────────
// CO1.7-impl A4: LedgerCasView trait + replay_full_transition
// ────────────────────────────────────────────────────────────────────────────

/// CO1.7 spec § 4 + DIV-4 closure: narrow read-only CAS trait that replay
/// needs. Decouples `replay_full_transition` from full `CasStore` (the
/// production impl). Anything that can hand back the bytes for a `Cid`
/// satisfies this — testing can mock it; cold-replay uses CasStore directly.
pub trait LedgerCasView {
    fn get_typed_payload(
        &self,
        cid: &crate::bottom_white::cas::schema::Cid,
    ) -> Result<Vec<u8>, ReplayError>;
}

impl LedgerCasView for crate::bottom_white::cas::store::CasStore {
    fn get_typed_payload(
        &self,
        cid: &crate::bottom_white::cas::schema::Cid,
    ) -> Result<Vec<u8>, ReplayError> {
        self.get(cid).map_err(|_| ReplayError::CasMissing { at: 0 })
    }
}

/// CO1.7-impl A4 — full-mode replay (THE I-DETHASH witness).
///
/// Validates **every** stage spec § 4 + § 6 promises:
/// 1. logical_t monotonicity
/// 2. parent_state_root chain
/// 3. parent_ledger_root chain (K2 transplant defense)
/// 4. system_signature verifies via CanonicalMessage::LedgerEntrySigning + pinned pubkeys
/// 5. CAS lookup of tx_payload_cid succeeds (CO1.4-extra cold-replay capability)
/// 6. canonical_decode of payload bytes → TypedTx
/// 6.5 (v1.1 C-3): decoded_typed_tx.tx_kind() MUST equal entry.tx_kind
/// 7. dispatch_transition re-run produces (q_next, _signals)
/// 8. q_next.state_root_t matches entry.resulting_state_root
/// 9. resulting_ledger_root recomputed via append() matches stored
///
/// **v1.1 C-1 closure**: takes a full `genesis: &QState` (was `genesis_state_root`
/// + `genesis_ledger_root` only). Caller provides the complete genesis state
/// so dispatch_transition can read budget / registries / balances / task markets
/// — fabricating `QState::genesis()` was dropping these fields.
///
/// **Stub-state caveat (CO1.7.5 unblocks)**: while `dispatch_transition`
/// returns `NotYetImplemented` for every variant, replay errors at stage 7
/// for any non-empty chain. Conformance tests exercising stages 1-6.5
/// independently are `#[test]`-runnable now; full state_root reconstruction
/// gates on CO1.7.5.
pub fn replay_full_transition(
    genesis: &crate::state::q_state::QState,
    entries: &[LedgerEntry],
    cas: &dyn LedgerCasView,
    pinned_pubkeys: &crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys,
    predicate_registry: &crate::top_white::predicates::registry::PredicateRegistry,
    tool_registry: &crate::bottom_white::tools::registry::ToolRegistry,
) -> Result<crate::state::q_state::QState, ReplayError> {
    use crate::bottom_white::ledger::system_keypair::{
        verify_system_signature, CanonicalMessage,
    };
    use crate::state::sequencer::dispatch_transition;

    let mut q = genesis.clone();

    for (i, entry) in entries.iter().enumerate() {
        // Stage 1
        let expected_logical_t = (i as u64) + 1;
        if entry.logical_t != expected_logical_t {
            return Err(ReplayError::LogicalTGap {
                at: i,
                expected: expected_logical_t,
                got: entry.logical_t,
            });
        }
        // Stage 2
        if entry.parent_state_root != q.state_root_t {
            return Err(ReplayError::ParentStateMismatch { at: i });
        }
        // Stage 3
        if entry.parent_ledger_root != q.ledger_root_t {
            return Err(ReplayError::ParentLedgerMismatch { at: i });
        }

        // Stage 4: system_signature verify (FullTransition mode only).
        let signing_payload = entry.to_signing_payload();
        let signing_digest = signing_payload.canonical_digest();
        let canonical_msg = CanonicalMessage::LedgerEntrySigning(signing_digest.0);
        if !verify_system_signature(
            &entry.system_signature,
            &canonical_msg,
            entry.epoch,
            pinned_pubkeys,
        ) {
            return Err(ReplayError::BadSignature { at: i });
        }

        // Stage 5: CAS lookup.
        let payload_bytes = cas
            .get_typed_payload(&entry.tx_payload_cid)
            .map_err(|_| ReplayError::CasMissing { at: i })?;

        // Stage 6: canonical_decode → TypedTx (v1.1 C-3-secondary: distinct
        // error from CasMissing).
        let typed_tx: crate::state::typed_tx::TypedTx =
            canonical_decode(&payload_bytes).map_err(|e| ReplayError::PayloadDecode {
                at: i,
                reason: e.to_string(),
            })?;

        // Stage 6.5 (v1.1 C-3): tx_kind envelope vs decoded payload kind MUST match.
        // Otherwise a signed envelope claiming `Work` could ride a CAS payload
        // that decodes as `Verify` — sequencer would have written that
        // mismatch but replay would have silently accepted it pre-v1.1.
        let decoded_kind = typed_tx.tx_kind();
        if decoded_kind != entry.tx_kind {
            return Err(ReplayError::TxKindMismatch {
                at: i,
                envelope_kind: entry.tx_kind,
                decoded_kind,
            });
        }

        // Stage 7: re-run pure dispatch_transition.
        let (q_next, _signals) =
            dispatch_transition(&q, &typed_tx, predicate_registry, tool_registry)
                .map_err(|inner| ReplayError::Transition { at: i, inner })?;

        // Stage 8: state_root match.
        if q_next.state_root_t != entry.resulting_state_root {
            return Err(ReplayError::StateRootMismatch { at: i });
        }

        // Stage 9: ledger_root match (recompute via append).
        let recomputed_ledger_root = append(&q.ledger_root_t, &signing_digest);
        if recomputed_ledger_root != entry.resulting_ledger_root {
            return Err(ReplayError::LedgerRootMismatch { at: i });
        }

        // Advance.
        q = q_next;
        q.ledger_root_t = entry.resulting_ledger_root;
    }

    Ok(q)
}

/// Skeleton-stage entry point (v1.1).
///
/// Validates:
/// 1. logical_t monotonicity (no gaps, no duplicates)
/// 2. parent_state_root chain
/// 3. parent_ledger_root chain (K2 transplant defense)
/// 4. resulting_ledger_root recomputed via append(prev_ledger_root, signing_digest)
///
/// Does NOT verify:
/// - system_signature (CO1.7.5+: requires CanonicalMessage extension wired through keypair)
/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
///
/// Returns final (state_root, ledger_root) on success.
pub fn replay_chain_integrity(
    genesis_state_root: Hash,
    genesis_ledger_root: Hash,
    entries: &[LedgerEntry],
) -> Result<(Hash, Hash), ReplayError> {
    let mut prev_state_root = genesis_state_root;
    let mut prev_ledger_root = genesis_ledger_root;

    for (i, entry) in entries.iter().enumerate() {
        let expected_logical_t = (i as u64) + 1;
        if entry.logical_t != expected_logical_t {
            return Err(ReplayError::LogicalTGap {
                at: i,
                expected: expected_logical_t,
                got: entry.logical_t,
            });
        }
        if entry.parent_state_root != prev_state_root {
            return Err(ReplayError::ParentStateMismatch { at: i });
        }
        // K2 NEW: parent_ledger_root chain check
        if entry.parent_ledger_root != prev_ledger_root {
            return Err(ReplayError::ParentLedgerMismatch { at: i });
        }
        let signing_digest = entry.to_signing_payload().canonical_digest();
        let recomputed = append(&prev_ledger_root, &signing_digest);
        if recomputed != entry.resulting_ledger_root {
            return Err(ReplayError::LedgerRootMismatch { at: i });
        }
        prev_state_root = entry.resulting_state_root;
        prev_ledger_root = entry.resulting_ledger_root;
    }

    Ok((prev_state_root, prev_ledger_root))
}

// ────────────────────────────────────────────────────────────────────────────
// § 2.5 Canonical serialization (bincode v2; STATE_TRANSITION_SPEC § 2.5)
// ────────────────────────────────────────────────────────────────────────────

/// `bincode::config` used for the canonical `LedgerEntry` wire format.
///
/// **Frozen choices** (per STATE_TRANSITION_SPEC § 2.5):
/// - **Big-endian** byte order (network order; deterministic across platforms).
/// - **Fixed-int encoding** (no varint; fixed-width for byte-stable round-trip).
/// - **`BTreeMap` keys**: bincode iterates the map in serde-supplied order; we
///   only ever encode `BTreeMap` (sorted by construction) so key order is lex.
/// - **No padding, no implicit alignment.**
fn bincode_canonical_config() -> impl bincode::config::Config {
    bincode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding()
}

/// Canonical encode any serde-Serialize value to bytes (CO1.7 wire format).
/// Used by `Git2LedgerWriter` for commit-message bodies and by future callers
/// needing byte-stable signatures over typed payloads.
pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalCodecError> {
    bincode::serde::encode_to_vec(value, bincode_canonical_config())
        .map_err(|e| CanonicalCodecError::Encode(e.to_string()))
}

/// Canonical decode the inverse of `canonical_encode`. Returns the value plus
/// the number of bytes consumed (entire input must be consumed for a clean decode).
pub fn canonical_decode<T: serde::de::DeserializeOwned>(
    bytes: &[u8],
) -> Result<T, CanonicalCodecError> {
    let (value, consumed) =
        bincode::serde::decode_from_slice::<T, _>(bytes, bincode_canonical_config())
            .map_err(|e| CanonicalCodecError::Decode(e.to_string()))?;
    if consumed != bytes.len() {
        return Err(CanonicalCodecError::TrailingBytes {
            consumed,
            total: bytes.len(),
        });
    }
    Ok(value)
}

#[derive(Debug)]
pub enum CanonicalCodecError {
    Encode(String),
    Decode(String),
    TrailingBytes { consumed: usize, total: usize },
}

impl std::fmt::Display for CanonicalCodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encode(s) => write!(f, "canonical encode failed: {s}"),
            Self::Decode(s) => write!(f, "canonical decode failed: {s}"),
            Self::TrailingBytes { consumed, total } => {
                write!(f, "trailing bytes after decode: consumed {consumed} of {total}")
            }
        }
    }
}
impl std::error::Error for CanonicalCodecError {}

// ────────────────────────────────────────────────────────────────────────────
// § 5 Git2LedgerWriter — git2-rs commit chain on `refs/transitions/main`
// ────────────────────────────────────────────────────────────────────────────

/// Spec § 5 production storage backend.
///
/// **Mapping**:
/// - One `LedgerEntry` = one git commit on `refs/transitions/main`.
/// - **Commit tree** = three named blobs:
///     - `payload_cid`     = entry.tx_payload_cid.0 (32 bytes)
///     - `signature`       = entry.system_signature.as_bytes() (64 bytes)
///     - `entry_canonical` = bincode v2 BE + fixed-int encoding of the full
///       `LedgerEntry` (deterministic, byte-stable; this blob IS the
///       canonical record — `read_at` decodes it directly).
/// - **Commit message** = human-readable `"transition logical_t=<N>\n"` (the
///   canonical record lives in the tree blob, not the message — git
///   normalizes message bytes in ways that break round-trip).
/// - **Parent**: `head_t-1` commit (or none at genesis).
/// - **Author/committer identity**: fixed `("turingosv4 sequencer", "system@turingos")`
///   with `time = (logical_t as i64, 0)` to keep commit OIDs deterministic. NO
///   wall-clock leakage (`I-NOENV` + `I-LOGTIME`).
///
/// **K3 (revised v1.2)**: this writer surfaces `commit_oid` for callers that
/// need it (CO1.7.5+ `head_t` wiring), but the `LedgerWriter::commit` trait
/// returns only `Hash` (entry.resulting_ledger_root). Callers requesting the
/// commit OID use [`Git2LedgerWriter::head_commit_oid`] post-commit.
pub struct Git2LedgerWriter {
    repo_path: PathBuf,
    /// Last commit OID on `refs/transitions/main`; `None` at empty-chain genesis.
    head_oid: Option<git2::Oid>,
    /// Number of entries committed = highest assigned `logical_t` (0 at genesis).
    len: u64,
}

const TRANSITIONS_REF: &str = "refs/transitions/main";
const TREE_BLOB_PAYLOAD_CID: &str = "payload_cid";
const TREE_BLOB_SIGNATURE: &str = "signature";
const TREE_BLOB_ENTRY_CANONICAL: &str = "entry_canonical";

impl Git2LedgerWriter {
    /// Open or initialize a `Git2LedgerWriter` rooted at `repo_path`.
    /// Creates the underlying git repo if it doesn't exist; resolves the
    /// existing `refs/transitions/main` if present and seeds `head_oid` + `len`.
    pub fn open(repo_path: &Path) -> Result<Self, LedgerWriterError> {
        let repo_path = repo_path.to_path_buf();
        let repo = match Repository::open(&repo_path) {
            Ok(r) => r,
            Err(_) => Repository::init(&repo_path).map_err(|e| {
                LedgerWriterError::BackendCorruption(format!("repo init: {e}"))
            })?,
        };

        // Resolve refs/transitions/main if it exists.
        let (head_oid, len) = match repo.find_reference(TRANSITIONS_REF) {
            Ok(reference) => {
                let oid = reference
                    .target()
                    .ok_or_else(|| {
                        LedgerWriterError::BackendCorruption(format!(
                            "{TRANSITIONS_REF} has no direct target"
                        ))
                    })?;
                // Walk parents to count chain length.
                let mut n: u64 = 0;
                let mut cursor = Some(oid);
                while let Some(c) = cursor {
                    n += 1;
                    let commit = repo.find_commit(c).map_err(|e| {
                        LedgerWriterError::BackendCorruption(format!("walk parent: {e}"))
                    })?;
                    cursor = commit.parent(0).ok().map(|p| p.id());
                }
                (Some(oid), n)
            }
            Err(_) => (None, 0),
        };

        Ok(Self {
            repo_path,
            head_oid,
            len,
        })
    }

    fn open_repo(&self) -> Result<Repository, LedgerWriterError> {
        Repository::open(&self.repo_path)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("repo open: {e}")))
    }

    /// Commit OID of the most recent appended entry (None if chain is empty).
    /// CO1.7.5+ `head_t` wiring uses this to surface commit_sha alongside Hash.
    pub fn head_commit_oid(&self) -> Option<git2::Oid> {
        self.head_oid
    }

    /// Read raw canonical-encoded `LedgerEntry` bytes (the `entry_canonical`
    /// tree blob) for the entry at `logical_t`. `logical_t` is 1-indexed.
    fn read_canonical_bytes(&self, logical_t: u64) -> Result<Vec<u8>, LedgerWriterError> {
        if logical_t == 0 || logical_t > self.len {
            return Err(LedgerWriterError::NotFound { logical_t });
        }
        let repo = self.open_repo()?;
        // Walk back (len - logical_t) parents from head.
        let mut cursor = self.head_oid.ok_or(LedgerWriterError::NotFound { logical_t })?;
        let mut steps_back = self.len - logical_t;
        while steps_back > 0 {
            let commit = repo.find_commit(cursor).map_err(|e| {
                LedgerWriterError::BackendCorruption(format!("find_commit: {e}"))
            })?;
            cursor = commit
                .parent(0)
                .map_err(|e| LedgerWriterError::BackendCorruption(format!("parent: {e}")))?
                .id();
            steps_back -= 1;
        }
        let commit = repo
            .find_commit(cursor)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_commit: {e}")))?;
        let tree = commit
            .tree()
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree: {e}")))?;
        let entry_obj = tree
            .get_name(TREE_BLOB_ENTRY_CANONICAL)
            .ok_or_else(|| {
                LedgerWriterError::BackendCorruption(format!(
                    "missing {TREE_BLOB_ENTRY_CANONICAL} blob at logical_t={logical_t}"
                ))
            })?;
        let blob = repo
            .find_blob(entry_obj.id())
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_blob: {e}")))?;
        Ok(blob.content().to_vec())
    }
}

impl LedgerWriter for Git2LedgerWriter {
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
        let expected = self.len + 1;
        if entry.logical_t != expected {
            return Err(LedgerWriterError::LogicalTGap {
                expected,
                got: entry.logical_t,
            });
        }

        let repo = self.open_repo()?;
        let canonical = canonical_encode(entry).map_err(|e| {
            LedgerWriterError::BackendCorruption(format!("canonical_encode: {e}"))
        })?;

        let mut tb = repo
            .treebuilder(None)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("treebuilder: {e}")))?;
        let cid_blob = repo
            .blob(&entry.tx_payload_cid.0)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("cid blob: {e}")))?;
        tb.insert(TREE_BLOB_PAYLOAD_CID, cid_blob, 0o100644)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert cid: {e}")))?;
        let sig_blob = repo
            .blob(entry.system_signature.as_bytes())
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("sig blob: {e}")))?;
        tb.insert(TREE_BLOB_SIGNATURE, sig_blob, 0o100644)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert sig: {e}")))?;
        let entry_blob = repo
            .blob(&canonical)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("entry blob: {e}")))?;
        tb.insert(TREE_BLOB_ENTRY_CANONICAL, entry_blob, 0o100644)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert entry: {e}")))?;
        let tree_oid = tb
            .write()
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree write: {e}")))?;
        let tree = repo
            .find_tree(tree_oid)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_tree: {e}")))?;

        // Determinism: time = (logical_t, 0). NO wall clock.
        let time = git2::Time::new(entry.logical_t as i64, 0);
        let author = GitSignature::new("turingosv4 sequencer", "system@turingos", &time)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("git sig: {e}")))?;
        let committer = author.clone();

        let parents: Vec<git2::Commit<'_>> = match self.head_oid {
            Some(oid) => vec![repo.find_commit(oid).map_err(|e| {
                LedgerWriterError::BackendCorruption(format!("parent commit: {e}"))
            })?],
            None => Vec::new(),
        };
        let parent_refs: Vec<&git2::Commit<'_>> = parents.iter().collect();
        let message = format!("transition logical_t={}\n", entry.logical_t);
        let new_oid = repo
            .commit(
                Some(TRANSITIONS_REF),
                &author,
                &committer,
                &message,
                &tree,
                &parent_refs,
            )
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("commit: {e}")))?;

        self.head_oid = Some(new_oid);
        self.len += 1;
        Ok(entry.resulting_ledger_root)
    }

    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
        let bytes = self.read_canonical_bytes(logical_t)?;
        canonical_decode::<LedgerEntry>(&bytes).map_err(|e| {
            LedgerWriterError::BackendCorruption(format!("canonical_decode at {logical_t}: {e}"))
        })
    }

    fn len(&self) -> u64 {
        self.len
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Tests — 8 conformance items (4 NEW vs v1 skeleton: K2 / Q9 / repr(u8) / extensions)
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn h(byte: u8) -> Hash {
        Hash([byte; 32])
    }

    /// Build an entry that satisfies all chain invariants given the previous state.
    fn entry_at(
        logical_t: u64,
        parent_state_root: Hash,
        parent_ledger_root: Hash,
        resulting_state_root: Hash,
    ) -> LedgerEntry {
        let signing = LedgerEntrySigningPayload {
            logical_t,
            parent_state_root,
            parent_ledger_root,
            tx_kind: TxKind::Work,
            tx_payload_cid: Cid([0u8; 32]),
            resulting_state_root,
            timestamp_logical: logical_t,
            epoch: SystemEpoch::new(1),
            extensions: BTreeMap::new(),
        };
        let signing_digest = signing.canonical_digest();
        let resulting_ledger_root = append(&parent_ledger_root, &signing_digest);
        LedgerEntry {
            logical_t: signing.logical_t,
            parent_state_root: signing.parent_state_root,
            parent_ledger_root: signing.parent_ledger_root,
            tx_kind: signing.tx_kind,
            tx_payload_cid: signing.tx_payload_cid,
            resulting_state_root: signing.resulting_state_root,
            resulting_ledger_root,
            timestamp_logical: signing.timestamp_logical,
            epoch: signing.epoch,
            extensions: signing.extensions,
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        }
    }

    // 1. append byte-stable (I-DET ledger axis)
    #[test]
    fn append_is_pure_and_byte_stable() {
        let a = append(&Hash::ZERO, &h(1));
        let b = append(&Hash::ZERO, &h(1));
        assert_eq!(a, b);
        let c = append(&Hash::ZERO, &h(2));
        assert_ne!(a, c);
    }

    // 2. canonical_digest stable (#[repr(u8)] discriminant stable)
    #[test]
    fn canonical_digest_stable_across_clones() {
        let p = LedgerEntrySigningPayload {
            logical_t: 1,
            parent_state_root: Hash::ZERO,
            parent_ledger_root: Hash::ZERO,
            tx_kind: TxKind::Work,
            tx_payload_cid: Cid([7u8; 32]),
            resulting_state_root: h(0xaa),
            timestamp_logical: 1,
            epoch: SystemEpoch::new(2),
            extensions: BTreeMap::new(),
        };
        let d1 = p.canonical_digest();
        let d2 = p.clone().canonical_digest();
        assert_eq!(d1, d2);
    }

    // 3. InMemoryWriter enforces logical_t monotonic
    #[test]
    fn in_memory_writer_enforces_logical_t() {
        let mut w = InMemoryLedgerWriter::new();
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        assert!(w.commit(&e1).is_ok());

        let e_skip = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let err = w.commit(&e_skip).unwrap_err();
        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
    }

    // 4. ChainOnly replay validates clean chain
    #[test]
    fn replay_chain_integrity_clean() {
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
        let (final_state, final_ledger) =
            replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1.clone(), e2.clone(), e3.clone()])
                .expect("clean chain replays");
        assert_eq!(final_state, e3.resulting_state_root);
        assert_eq!(final_ledger, e3.resulting_ledger_root);
    }

    // 5. ChainOnly replay rejects parent_state_root tamper
    #[test]
    fn replay_rejects_parent_state_tamper() {
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        e2.parent_state_root = h(0xff);
        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
        assert!(matches!(err, ReplayError::ParentStateMismatch { at: 1 }));
    }

    // 6. K2 NEW: ChainOnly replay rejects parent_ledger_root tamper (transplant defense)
    #[test]
    fn replay_rejects_parent_ledger_tamper() {
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        // Tamper with parent_ledger_root WITHOUT recomputing resulting_ledger_root —
        // simulates an attacker transplanting an entry from a different ledger history.
        e2.parent_ledger_root = h(0xff);
        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
        assert!(matches!(err, ReplayError::ParentLedgerMismatch { at: 1 }));
    }

    // 7. ChainOnly replay rejects ledger_root tamper
    #[test]
    fn replay_rejects_ledger_root_tamper() {
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        e2.resulting_ledger_root = h(0xee);
        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
        assert!(matches!(err, ReplayError::LedgerRootMismatch { at: 1 }));
    }

    // 8. Q9 NEW: canonical_digest excludes derivatives
    // Mutating `resulting_ledger_root` or `system_signature` of LedgerEntry must NOT
    // change the signing payload digest (because they're not in LedgerEntrySigningPayload).
    #[test]
    fn canonical_digest_excludes_derivatives() {
        let e_clean = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let digest_clean = e_clean.to_signing_payload().canonical_digest();

        // Mutate resulting_ledger_root (a derivative; should NOT affect signing digest)
        let mut e_tamper = e_clean.clone();
        e_tamper.resulting_ledger_root = h(0xff);
        let digest_after_root_tamper = e_tamper.to_signing_payload().canonical_digest();
        assert_eq!(
            digest_clean, digest_after_root_tamper,
            "resulting_ledger_root MUST NOT affect signing digest (Q9 cycle prevention)"
        );

        // Mutate system_signature (signature is its own input; should NOT affect signing digest)
        let mut e_tamper2 = e_clean.clone();
        e_tamper2.system_signature = SystemSignature::from_bytes([0xffu8; 64]);
        let digest_after_sig_tamper = e_tamper2.to_signing_payload().canonical_digest();
        assert_eq!(digest_clean, digest_after_sig_tamper);

        // Sanity: mutating a SIGNED field DOES change digest
        let mut e_signed_change = e_clean.clone();
        e_signed_change.epoch = SystemEpoch::new(99);
        let digest_after_signed = e_signed_change.to_signing_payload().canonical_digest();
        assert_ne!(digest_clean, digest_after_signed);
    }

    // 9. C3 closure (round-2): real signature roundtrip via system_keypair extension.
    // Verifies: (a) typed sign API works; (b) signature verifies via CanonicalMessage::LedgerEntrySigning;
    // (c) signature does NOT verify after mutating a signed field (parent_ledger_root — K2 transplant defense).
    #[test]
    fn signature_round_trip_and_transplant_defense() {
        use crate::bottom_white::ledger::system_keypair::{
            transition_ledger_emitter, CanonicalMessage, Ed25519Keypair, PinnedSystemPubkeys,
            SystemEpoch, verify_system_signature,
        };

        let keypair = Ed25519Keypair::generate_with_secure_entropy().expect("keypair gen");
        let epoch = SystemEpoch::new(1);
        let mut pinned = PinnedSystemPubkeys::new();
        pinned.insert(epoch, keypair.public_key());

        // Build a clean signing payload (e1's worth)
        let payload = LedgerEntrySigningPayload {
            logical_t: 1,
            parent_state_root: Hash::ZERO,
            parent_ledger_root: Hash::ZERO,
            tx_kind: TxKind::Work,
            tx_payload_cid: Cid([42u8; 32]),
            resulting_state_root: h(1),
            timestamp_logical: 1,
            epoch,
            extensions: BTreeMap::new(),
        };
        let digest = payload.canonical_digest();

        // Real sign through the typed CanonicalMessage extension
        let sig = transition_ledger_emitter::sign_ledger_entry(&keypair, digest.0)
            .expect("sign_ledger_entry");

        // Verify (clean) — must succeed
        let msg_clean = CanonicalMessage::LedgerEntrySigning(digest.0);
        assert!(
            verify_system_signature(&sig, &msg_clean, epoch, &pinned),
            "clean signature must verify"
        );

        // Verify (tamper parent_ledger_root) — K2 transplant defense
        let mut payload_tamper = payload.clone();
        payload_tamper.parent_ledger_root = h(0xff);
        let digest_tamper = payload_tamper.canonical_digest();
        let msg_tamper = CanonicalMessage::LedgerEntrySigning(digest_tamper.0);
        assert!(
            !verify_system_signature(&sig, &msg_tamper, epoch, &pinned),
            "transplanted parent_ledger_root MUST fail signature verify (K2)"
        );

        // Verify (cross-epoch transplant) — D1 defense via epoch IN payload digest.
        // Attacker scenario: sig was made for payload with epoch=1; attacker forges a
        // NEW payload claiming epoch=2 reusing the old sig. Since epoch is in the
        // canonical digest, digest_v2 ≠ digest_v1, so the sig on digest_v1 cannot
        // verify against digest_v2.
        let mut payload_other_epoch = payload.clone();
        payload_other_epoch.epoch = SystemEpoch::new(2);
        let digest_other_epoch = payload_other_epoch.canonical_digest();
        assert_ne!(digest, digest_other_epoch, "epoch is bound in canonical digest");
        let msg_other_epoch = CanonicalMessage::LedgerEntrySigning(digest_other_epoch.0);
        assert!(
            !verify_system_signature(&sig, &msg_other_epoch, epoch, &pinned),
            "cross-epoch transplant MUST fail signature verify (D1 epoch binding)"
        );
    }

    // ──────────────────────────────────────────────────────────────────────
    // 10–13. Git2LedgerWriter — git2-rs commit chain backend (§ 5)
    // ──────────────────────────────────────────────────────────────────────

    use tempfile::TempDir;

    fn fresh_git_writer() -> (TempDir, Git2LedgerWriter) {
        let tmp = TempDir::new().expect("tempdir");
        let w = Git2LedgerWriter::open(tmp.path()).expect("open");
        (tmp, w)
    }

    // 10. Empty repo: len()=0, head_commit_oid=None.
    #[test]
    fn git2_writer_empty_chain() {
        let (_tmp, w) = fresh_git_writer();
        assert_eq!(w.len(), 0);
        assert!(w.head_commit_oid().is_none());
    }

    // 11. Append three entries; len + head_commit_oid advance per commit;
    //     read_at recovers each entry byte-identically (canonical encode/decode round-trip).
    #[test]
    fn git2_writer_append_and_read_back() {
        let (_tmp, mut w) = fresh_git_writer();
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));

        let r1 = w.commit(&e1).expect("commit 1");
        assert_eq!(r1, e1.resulting_ledger_root);
        assert_eq!(w.len(), 1);
        let oid_1 = w.head_commit_oid().expect("head after 1");

        let r2 = w.commit(&e2).expect("commit 2");
        assert_eq!(r2, e2.resulting_ledger_root);
        assert_eq!(w.len(), 2);
        let oid_2 = w.head_commit_oid().expect("head after 2");
        assert_ne!(oid_1, oid_2, "head must advance after second commit");

        w.commit(&e3).expect("commit 3");
        assert_eq!(w.len(), 3);

        // read_at returns each entry byte-identically.
        assert_eq!(w.read_at(1).expect("read 1"), e1);
        assert_eq!(w.read_at(2).expect("read 2"), e2);
        assert_eq!(w.read_at(3).expect("read 3"), e3);
    }

    // 12. Skipping a logical_t triggers LogicalTGap; chain state is unchanged.
    #[test]
    fn git2_writer_rejects_logical_t_gap() {
        let (_tmp, mut w) = fresh_git_writer();
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        w.commit(&e1).expect("commit 1");
        let pre_oid = w.head_commit_oid();

        // Try to commit a logical_t=3 entry (gap: expected 2)
        let e_skip = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let err = w.commit(&e_skip).unwrap_err();
        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
        // Chain unchanged.
        assert_eq!(w.len(), 1);
        assert_eq!(w.head_commit_oid(), pre_oid);
    }

    // 13. Reopening the same repo path resurrects the chain (head + len recovered
    //     from refs/transitions/main). Crucial for runtime cold-restart.
    #[test]
    fn git2_writer_reopen_recovers_chain() {
        let tmp = TempDir::new().expect("tempdir");
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let oid_after_two;
        {
            let mut w = Git2LedgerWriter::open(tmp.path()).expect("open");
            w.commit(&e1).expect("commit 1");
            w.commit(&e2).expect("commit 2");
            oid_after_two = w.head_commit_oid().expect("head");
        }
        // Reopen — fresh struct, same on-disk repo.
        let w2 = Git2LedgerWriter::open(tmp.path()).expect("reopen");
        assert_eq!(w2.len(), 2);
        assert_eq!(w2.head_commit_oid(), Some(oid_after_two));
        assert_eq!(w2.read_at(1).expect("read 1"), e1);
        assert_eq!(w2.read_at(2).expect("read 2"), e2);

        // Continue chain after reopen.
        let mut w3 = Git2LedgerWriter::open(tmp.path()).expect("reopen 2");
        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
        w3.commit(&e3).expect("commit 3");
        assert_eq!(w3.len(), 3);
    }

    // ──────────────────────────────────────────────────────────────────────
    // 15-18. CO1.7-impl A4 — replay_full_transition (THE I-DETHASH witness)
    // ──────────────────────────────────────────────────────────────────────

    use crate::bottom_white::cas::schema::ObjectType;
    use crate::bottom_white::cas::store::CasStore;
    use crate::bottom_white::ledger::system_keypair::{
        transition_ledger_emitter, Ed25519Keypair, PinnedSystemPubkeys,
    };
    use crate::bottom_white::tools::registry::ToolRegistry;
    use crate::state::typed_tx::{
        AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey,
        SafetyOrCreation, TaskId, TypedTx, WorkTx, WriteKey,
    };
    use crate::state::q_state::{AgentId, TxId as QTxId};
    use crate::top_white::predicates::registry::PredicateRegistry;

    fn dummy_typed_tx() -> TypedTx {
        let mut acceptance = std::collections::BTreeMap::new();
        acceptance.insert(
            PredicateId("acc1".into()),
            BoolWithProof { value: true, proof_cid: None },
        );
        TypedTx::Work(WorkTx {
            tx_id: QTxId("worktx-replay-fixture".into()),
            task_id: TaskId("task-replay".into()),
            parent_state_root: Hash::ZERO,
            agent_id: AgentId("alice".into()),
            read_set: [ReadKey("k.r".into())].into_iter().collect::<std::collections::BTreeSet<_>>(),
            write_set: [WriteKey("k.w".into())].into_iter().collect::<std::collections::BTreeSet<_>>(),
            proposal_cid: Cid([0; 32]),
            predicate_results: PredicateResultsBundle {
                acceptance,
                settlement: std::collections::BTreeMap::new(),
                safety_class: SafetyOrCreation::Safety,
            },
            stake: crate::economy::money::StakeMicroCoin::from_micro_units(1),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 1,
        })
    }

    /// Build a real signed LedgerEntry against the given keypair + epoch,
    /// with the typed_tx's canonical bytes stored in CAS. Mirrors
    /// `Sequencer::apply_one` stages 5-9 outside the runtime.
    fn build_signed_entry(
        logical_t: u64,
        parent_state_root: Hash,
        parent_ledger_root: Hash,
        resulting_state_root: Hash,
        epoch: SystemEpoch,
        keypair: &Ed25519Keypair,
        cas: &mut CasStore,
        typed_tx: &TypedTx,
    ) -> LedgerEntry {
        let bytes = canonical_encode(typed_tx).expect("encode");
        let cid = cas
            .put(&bytes, ObjectType::ProposalPayload, "test", logical_t, None)
            .expect("cas put");
        let signing = LedgerEntrySigningPayload {
            logical_t,
            parent_state_root,
            parent_ledger_root,
            tx_kind: typed_tx.tx_kind(),
            tx_payload_cid: cid,
            resulting_state_root,
            timestamp_logical: logical_t,
            epoch,
            extensions: BTreeMap::new(),
        };
        let digest = signing.canonical_digest();
        let sig = transition_ledger_emitter::sign_ledger_entry(keypair, digest.0)
            .expect("sign");
        let resulting_ledger_root = append(&parent_ledger_root, &digest);
        LedgerEntry {
            logical_t,
            parent_state_root,
            parent_ledger_root,
            tx_kind: typed_tx.tx_kind(),
            tx_payload_cid: cid,
            resulting_state_root,
            resulting_ledger_root,
            timestamp_logical: logical_t,
            epoch,
            extensions: BTreeMap::new(),
            system_signature: sig,
        }
    }

    fn replay_test_setup() -> (
        TempDir,
        CasStore,
        Ed25519Keypair,
        SystemEpoch,
        PinnedSystemPubkeys,
        PredicateRegistry,
        ToolRegistry,
    ) {
        let tmp = TempDir::new().expect("tempdir");
        let cas = CasStore::open(tmp.path()).expect("cas");
        let kp = Ed25519Keypair::generate_with_secure_entropy().expect("kp");
        let epoch = SystemEpoch::new(1);
        let mut pinned = PinnedSystemPubkeys::new();
        pinned.insert(epoch, kp.public_key());
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        (tmp, cas, kp, epoch, pinned, preds, tools)
    }

    /// 15. CO1.7.5-stage: in stub mode, dispatch errors with NotYetImplemented;
    ///     replay correctly bubbles up `Transition { at: 0, inner: NotYetImplemented }`.
    ///     This proves stages 1-6 (chain + sig + CAS + decode) all PASS,
    ///     leaving stage 7 (dispatch) as the only gate. CO1.7.5 fills it.
    #[test]
    fn replay_full_transition_reaches_dispatch_then_stubs() {
        let (_tmp, mut cas, kp, epoch, pinned, preds, tools) = replay_test_setup();
        let entry = build_signed_entry(
            1,
            Hash::ZERO,
            Hash::ZERO,
            h(1), // resulting state_root (won't be reached due to dispatch stub)
            epoch,
            &kp,
            &mut cas,
            &dummy_typed_tx(),
        );
        let err = replay_full_transition(
            &crate::state::q_state::QState::genesis(),
            &[entry],
            &cas,
            &pinned,
            &preds,
            &tools,
        )
        .unwrap_err();
        assert!(
            matches!(err, ReplayError::Transition { at: 0, inner: crate::state::typed_tx::TransitionError::NotYetImplemented }),
            "expected Transition(NotYetImplemented at 0); got {err:?}"
        );
    }

    /// 16. system_signature_verifies_via_canonical_message — tampering the
    ///     signature MUST fire BadSignature BEFORE dispatch is reached.
    #[test]
    fn replay_rejects_bad_system_signature() {
        let (_tmp, mut cas, kp, epoch, pinned, preds, tools) = replay_test_setup();
        let mut entry = build_signed_entry(
            1,
            Hash::ZERO,
            Hash::ZERO,
            h(1),
            epoch,
            &kp,
            &mut cas,
            &dummy_typed_tx(),
        );
        // Tamper signature.
        entry.system_signature = SystemSignature::from_bytes([0xff; 64]);
        let err = replay_full_transition(
            &crate::state::q_state::QState::genesis(),
            &[entry],
            &cas,
            &pinned,
            &preds,
            &tools,
        )
        .unwrap_err();
        assert!(matches!(err, ReplayError::BadSignature { at: 0 }));
    }

    /// 17. cas_payload_round_trip — replay correctly fetches CAS bytes;
    ///     CO1.4-extra cold-restart capability test.
    #[test]
    fn replay_cas_payload_round_trip_after_reopen() {
        let tmp = TempDir::new().expect("tempdir");
        let kp = Ed25519Keypair::generate_with_secure_entropy().expect("kp");
        let epoch = SystemEpoch::new(1);
        let mut pinned = PinnedSystemPubkeys::new();
        pinned.insert(epoch, kp.public_key());
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();

        let entry;
        {
            let mut cas = CasStore::open(tmp.path()).expect("cas");
            entry = build_signed_entry(
                1,
                Hash::ZERO,
                Hash::ZERO,
                h(1),
                epoch,
                &kp,
                &mut cas,
                &dummy_typed_tx(),
            );
        }
        // Reopen — CO1.4-extra sidecar replay restores the CAS index.
        let cas2 = CasStore::open(tmp.path()).expect("reopen");
        let err = replay_full_transition(
            &crate::state::q_state::QState::genesis(),
            &[entry],
            &cas2,
            &pinned,
            &preds,
            &tools,
        )
        .unwrap_err();
        // Stages 1-6.5 (incl. CAS lookup post-reopen + tx_kind match) PASS;
        // stage 7 stubs.
        assert!(matches!(err, ReplayError::Transition { at: 0, .. }));
    }

    /// 18b. v1.1 C-3 closure: tx_kind mismatch — envelope claims one variant,
    ///      CAS payload decodes as another. Replay MUST reject before stage 7.
    #[test]
    fn replay_rejects_tx_kind_mismatch() {
        let (_tmp, mut cas, kp, epoch, pinned, preds, tools) = replay_test_setup();
        // Build a real entry whose envelope tx_kind matches the payload (Work).
        let mut entry = build_signed_entry(
            1,
            Hash::ZERO,
            Hash::ZERO,
            h(1),
            epoch,
            &kp,
            &mut cas,
            &dummy_typed_tx(),
        );
        // Tamper: claim a different tx_kind on the envelope, RE-SIGN with the
        // tampered envelope so signature still verifies.
        let tampered_signing = LedgerEntrySigningPayload {
            logical_t: entry.logical_t,
            parent_state_root: entry.parent_state_root,
            parent_ledger_root: entry.parent_ledger_root,
            tx_kind: TxKind::Verify, // ← lies about the payload kind
            tx_payload_cid: entry.tx_payload_cid,
            resulting_state_root: entry.resulting_state_root,
            timestamp_logical: entry.timestamp_logical,
            epoch: entry.epoch,
            extensions: entry.extensions.clone(),
        };
        let tampered_digest = tampered_signing.canonical_digest();
        let tampered_sig =
            transition_ledger_emitter::sign_ledger_entry(&kp, tampered_digest.0).expect("sign");
        entry.tx_kind = TxKind::Verify;
        entry.system_signature = tampered_sig;
        // Recompute resulting_ledger_root with the tampered signing digest so
        // chain check (stage 9) wouldn't be the failure path.
        entry.resulting_ledger_root = append(&Hash::ZERO, &tampered_digest);

        let err = replay_full_transition(
            &crate::state::q_state::QState::genesis(),
            &[entry],
            &cas,
            &pinned,
            &preds,
            &tools,
        )
        .unwrap_err();
        assert!(
            matches!(
                err,
                ReplayError::TxKindMismatch {
                    at: 0,
                    envelope_kind: TxKind::Verify,
                    decoded_kind: TxKind::Work
                }
            ),
            "expected TxKindMismatch(Verify vs Work), got {err:?}"
        );
    }

    /// 18c. v1.1 closure (Codex Q-K secondary): payload decode failure
    ///      reports as PayloadDecode (NOT CasMissing).
    #[test]
    fn replay_rejects_payload_decode_failure() {
        let (_tmp, mut cas, kp, epoch, pinned, preds, tools) = replay_test_setup();

        // Manually put NON-canonical bytes into CAS, then build an entry
        // pointing at them. Signature verifies because envelope binds the
        // cid, not the cid's contents.
        let bad_bytes = b"\xff\xff this is not a valid bincode TypedTx";
        let bad_cid = cas
            .put(bad_bytes, ObjectType::ProposalPayload, "test", 1, None)
            .expect("cas put");
        let signing = LedgerEntrySigningPayload {
            logical_t: 1,
            parent_state_root: Hash::ZERO,
            parent_ledger_root: Hash::ZERO,
            tx_kind: TxKind::Work,
            tx_payload_cid: bad_cid,
            resulting_state_root: h(1),
            timestamp_logical: 1,
            epoch,
            extensions: BTreeMap::new(),
        };
        let digest = signing.canonical_digest();
        let sig =
            transition_ledger_emitter::sign_ledger_entry(&kp, digest.0).expect("sign");
        let entry = LedgerEntry {
            logical_t: 1,
            parent_state_root: Hash::ZERO,
            parent_ledger_root: Hash::ZERO,
            tx_kind: TxKind::Work,
            tx_payload_cid: bad_cid,
            resulting_state_root: h(1),
            resulting_ledger_root: append(&Hash::ZERO, &digest),
            timestamp_logical: 1,
            epoch,
            extensions: BTreeMap::new(),
            system_signature: sig,
        };

        let err = replay_full_transition(
            &crate::state::q_state::QState::genesis(),
            &[entry],
            &cas,
            &pinned,
            &preds,
            &tools,
        )
        .unwrap_err();
        assert!(
            matches!(err, ReplayError::PayloadDecode { at: 0, .. }),
            "expected PayloadDecode at 0, got {err:?}"
        );
    }

    /// 18. sequencer_serial_replay_byte_identity — gated behind #[ignore]
    ///     until CO1.7.5 fills dispatch bodies. The skeleton of the
    ///     test is here so CO1.7.5 just removes the #[ignore].
    #[test]
    #[ignore = "CO1.7.5: requires real per-kind transition bodies"]
    fn sequencer_serial_replay_byte_identity() {
        // CO1.7.5 plan: submit N tx through Sequencer + collect entries from
        // ledger_writer + replay_full_transition(...) → assert final state_root
        // matches sequencer's q.state_root_t. Dispatch must produce real
        // (q_next, _signals) — currently all NotYetImplemented.
    }

    // 14. canonical_encode/decode round-trip for LedgerEntry (foundation of read_at).
    #[test]
    fn canonical_codec_round_trip() {
        let e1 = entry_at(7, h(0xaa), h(0xbb), h(0xcc));
        let bytes = canonical_encode(&e1).expect("encode");
        let e1_back: LedgerEntry = canonical_decode(&bytes).expect("decode");
        assert_eq!(e1, e1_back);

        // Two encodes of the same value must produce byte-identical bytes (I-DET).
        let bytes_again = canonical_encode(&e1).expect("encode again");
        assert_eq!(bytes, bytes_again);

        // Trailing garbage rejected.
        let mut bytes_extra = bytes.clone();
        bytes_extra.push(0xff);
        let err = canonical_decode::<LedgerEntry>(&bytes_extra).unwrap_err();
        assert!(matches!(err, CanonicalCodecError::TrailingBytes { .. }));
    }
}

```

---

# XREF: shipped src/bus.rs (D3 STEP_B target — TuringBus)

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

# XREF: shipped src/kernel.rs (D3 STEP_B target — Kernel)

```rust
// Tier 1: Pure topology (DAG) + zero-profit treasury + settlement
// Constitutional basis: Law 1 (zero domain knowledge)
// V3L-45: no domain strings. V3L-23: no hardcoded params.
//
// CRITICAL: This module must NEVER contain domain-specific terms.
// R-001 enforced by judge.sh — any edit is scanned.

use crate::ledger::{Node, NodeId, Tape, TapeError};
use crate::prediction_market::{BinaryMarket, MarketError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Core types ──────────────────────────────────────────────────

/// The pure topology manager.
/// It knows about nodes, edges (citations), and markets.
/// It does NOT know what the nodes contain or what domain they belong to.
#[derive(Debug, Serialize, Deserialize)]
pub struct Kernel {
    pub tape: Tape,
    pub markets: HashMap<NodeId, BinaryMarket>,
    /// Phase 3A (Hayek): bounty market opened at run start, seeded with
    /// pre-committed LP from the same ghost-liquidity pool as per-node markets.
    /// Liquid from tx 0 → gives agents a price signal BEFORE any behaviour.
    /// Resolves YES if golden path exists; pool distributed to GP-node authors.
    #[serde(default)]
    pub bounty_market: Option<BinaryMarket>,
    /// Seed LP committed to the bounty market at open time (separate from
    /// BinaryMarket's internal CPMM book). Used for payout distribution.
    #[serde(default)]
    pub bounty_lp_seed: f64,
}

/// Result of an append operation.
#[derive(Debug)]
pub struct AppendResult {
    pub node_id: NodeId,
}

/// Result of a resolution operation.
#[derive(Debug)]
pub struct ResolutionResult {
    pub golden_path: Vec<NodeId>,
    pub markets_resolved: usize,
}

// ── Implementation ──────────────────────────────────────────────

impl Kernel {
    pub fn new() -> Self {
        Kernel {
            tape: Tape::new(),
            markets: HashMap::new(),
            bounty_market: None,
            bounty_lp_seed: 0.0,
        }
    }

    /// Phase 3A (Hayek): open a run-level bounty market seeded with `lp_coins`.
    /// Agents see its YES price from tx 0; price pre-exists behaviour,
    /// breaking the Phase 2.5 bootstrap deadlock where no signal existed until
    /// some agent had already acted.
    pub fn open_bounty_market(&mut self, lp_coins: f64) -> Result<(), KernelError> {
        if self.bounty_market.is_some() {
            return Err(KernelError::MarketExists("__bounty__".to_string()));
        }
        let market = BinaryMarket::create("__bounty__".to_string(), lp_coins)
            .map_err(KernelError::Market)?;
        self.bounty_market = Some(market);
        self.bounty_lp_seed = lp_coins;
        Ok(())
    }

    pub fn bounty_yes_price(&self) -> Option<f64> {
        self.bounty_market.as_ref().map(|m| m.yes_price())
    }

    /// Resolve the bounty market. `gp_authors` lists the author of each node
    /// on the golden path (duplicates allowed — occurrences proxy contribution
    /// count). Empty list → YES loses, seed returned to ghost pool, no payout.
    /// Non-empty → YES wins, LP distributed equally across entries (so an
    /// author with 2 GP nodes gets twice the share of one with 1).
    pub fn resolve_bounty(&mut self, gp_authors: &[String]) -> HashMap<String, f64> {
        let mut payouts: HashMap<String, f64> = HashMap::new();
        let market = match self.bounty_market.as_mut() {
            Some(m) => m,
            None => return payouts,
        };
        if market.resolved.is_some() {
            return payouts;
        }
        let yes_wins = !gp_authors.is_empty();
        let _ = market.resolve(yes_wins);
        if !yes_wins {
            return payouts;
        }
        let lp = self.bounty_lp_seed;
        let n = gp_authors.len() as f64;
        for a in gp_authors {
            *payouts.entry(a.clone()).or_insert(0.0) += lp / n;
        }
        payouts
    }

    /// Append a node to the tape.
    /// Only checks structural validity (topology).
    /// Content validation is NOT this module's job (engine separation, C-003).
    pub fn append(&mut self, node: Node) -> Result<AppendResult, KernelError> {
        let node_id = node.id.clone();
        self.tape.append(node).map_err(KernelError::Tape)?;
        Ok(AppendResult { node_id })
    }

    /// Create a prediction market for a node.
    pub fn create_market(&mut self, node_id: &str, lp_coins: f64) -> Result<(), KernelError> {
        if !self.tape.nodes().contains_key(node_id) {
            return Err(KernelError::NodeNotFound(node_id.to_string()));
        }
        if self.markets.contains_key(node_id) {
            return Err(KernelError::MarketExists(node_id.to_string()));
        }
        let market = BinaryMarket::create(node_id.to_string(), lp_coins)
            .map_err(KernelError::Market)?;
        self.markets.insert(node_id.to_string(), market);
        Ok(())
    }

    /// Buy YES shares on a node's market.
    pub fn buy_yes(&mut self, node_id: &str, coins: f64) -> Result<f64, KernelError> {
        let market = self.markets.get_mut(node_id)
            .ok_or_else(|| KernelError::MarketNotFound(node_id.to_string()))?;
        let outcome = market.buy_yes(coins).map_err(KernelError::Market)?;
        Ok(outcome.shares_received)
    }

    /// Buy NO shares on a node's market.
    pub fn buy_no(&mut self, node_id: &str, coins: f64) -> Result<f64, KernelError> {
        let market = self.markets.get_mut(node_id)
            .ok_or_else(|| KernelError::MarketNotFound(node_id.to_string()))?;
        let outcome = market.buy_no(coins).map_err(KernelError::Market)?;
        Ok(outcome.shares_received)
    }

    /// Trace ancestors from a terminal node back to root(s).
    /// Pure topology — path validity is determined externally.
    pub fn trace_golden_path(&self, terminal_id: &str) -> Result<Vec<NodeId>, KernelError> {
        if !self.tape.nodes().contains_key(terminal_id) {
            return Err(KernelError::NodeNotFound(terminal_id.to_string()));
        }
        Ok(self.tape.trace_ancestors(terminal_id))
    }

    /// Resolve all markets after external settlement.
    /// `golden_path_ids`: nodes on the verified path (YES wins).
    /// All other markets resolve NO.
    pub fn resolve_all(
        &mut self,
        golden_path_ids: &[NodeId],
    ) -> Result<ResolutionResult, KernelError> {
        let gp_set: std::collections::HashSet<&str> =
            golden_path_ids.iter().map(|s| s.as_str()).collect();

        let mut resolved_count = 0;

        for (node_id, market) in self.markets.iter_mut() {
            if market.resolved.is_some() {
                continue;
            }
            let yes_wins = gp_set.contains(node_id.as_str());
            market.resolve(yes_wins).map_err(KernelError::Market)?;
            resolved_count += 1;
        }

        Ok(ResolutionResult {
            golden_path: golden_path_ids.to_vec(),
            markets_resolved: resolved_count,
        })
    }

    /// Get the current YES price for a node's market.
    pub fn yes_price(&self, node_id: &str) -> Option<f64> {
        self.markets.get(node_id).map(|m| m.yes_price())
    }

    /// Get top N nodes by YES price (highest first). Unresolved markets only.
    pub fn market_ticker(&self, top_n: usize) -> Vec<(NodeId, f64)> {
        let mut prices: Vec<(NodeId, f64)> = self.markets.iter()
            .filter(|(_, m)| m.resolved.is_none())
            .map(|(id, m)| (id.clone(), m.yes_price()))
            .collect();
        prices.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        prices.truncate(top_n);
        prices
    }

    /// Extended ticker with both sides + reserves, enabling bidirectional price signals
    /// (Art. II.2) and market-depth visibility. Sorted by YES price descending.
    pub fn market_ticker_full(&self, top_n: usize) -> Vec<(NodeId, f64, f64, f64, f64)> {
        let mut rows: Vec<(NodeId, f64, f64, f64, f64)> = self.markets.iter()
            .filter(|(_, m)| m.resolved.is_none())
            .map(|(id, m)| (id.clone(), m.yes_price(), m.no_price(), m.yes_reserve(), m.no_reserve()))
            .collect();
        rows.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        rows.truncate(top_n);
        rows
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}

// ── Errors ──────────────────────────────────────────────────────

#[derive(Debug)]
pub enum KernelError {
    Tape(TapeError),
    Market(MarketError),
    NodeNotFound(String),
    MarketNotFound(String),
    MarketExists(String),
}

impl std::fmt::Display for KernelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KernelError::Tape(e) => write!(f, "Tape error: {}", e),
            KernelError::Market(e) => write!(f, "Market error: {}", e),
            KernelError::NodeNotFound(id) => write!(f, "Node not found: {}", id),
            KernelError::MarketNotFound(id) => write!(f, "Market not found for node: {}", id),
            KernelError::MarketExists(id) => write!(f, "Market already exists for node: {}", id),
        }
    }
}

impl std::error::Error for KernelError {}

// ── Tests ───────────────────────────────────────────────────────
// NOTE: Domain-purity test lives in tests/kernel_purity.rs (outside this file)
// because R-001 forbids domain terms even as test strings in kernel.rs.

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(id: &str, author: &str, payload: &str, citations: Vec<&str>) -> Node {
        Node {
            id: id.to_string(),
            author: author.to_string(),
            payload: payload.to_string(),
            citations: citations.into_iter().map(|s| s.to_string()).collect(),
            created_at: 0,
            completion_tokens: 0,
        }
    }

    #[test]
    fn test_append_and_retrieve() {
        let mut k = Kernel::new();
        k.append(make_node("n1", "A0", "step 1", vec![])).unwrap();
        assert!(k.tape.get("n1").is_some());
    }

    #[test]
    fn test_reject_duplicate() {
        let mut k = Kernel::new();
        k.append(make_node("n1", "A0", "step 1", vec![])).unwrap();
        assert!(k.append(make_node("n1", "A1", "step 2", vec![])).is_err());
    }

    #[test]
    fn test_reject_dangling_citation() {
        let mut k = Kernel::new();
        assert!(k.append(make_node("n1", "A0", "step 1", vec!["ghost"])).is_err());
    }

    #[test]
    fn test_market_lifecycle() {
        let mut k = Kernel::new();
        k.append(make_node("n1", "A0", "step 1", vec![])).unwrap();
        k.create_market("n1", 2000.0).unwrap();

        let shares = k.buy_yes("n1", 100.0).unwrap();
        assert!(shares > 0.0);

        let price = k.yes_price("n1").unwrap();
        assert!(price > 0.5);
    }

    #[test]
    fn test_no_market_for_nonexistent_node() {
        let mut k = Kernel::new();
        assert!(k.create_market("ghost", 2000.0).is_err());
    }

    #[test]
    fn test_no_duplicate_market() {
        let mut k = Kernel::new();
        k.append(make_node("n1", "A0", "step", vec![])).unwrap();
        k.create_market("n1", 2000.0).unwrap();
        assert!(k.create_market("n1", 2000.0).is_err());
    }

    #[test]
    fn test_golden_path_trace() {
        let mut k = Kernel::new();
        k.append(make_node("root", "A0", "root", vec![])).unwrap();
        k.append(make_node("mid", "A1", "mid", vec!["root"])).unwrap();
        k.append(make_node("leaf", "A0", "leaf", vec!["mid"])).unwrap();

        let path = k.trace_golden_path("leaf").unwrap();
        assert_eq!(path, vec!["root", "mid", "leaf"]);
    }

    #[test]
    fn test_resolve_all_markets() {
        let mut k = Kernel::new();
        k.append(make_node("n1", "A0", "good", vec![])).unwrap();
        k.append(make_node("n2", "A1", "bad", vec![])).unwrap();
        k.create_market("n1", 2000.0).unwrap();
        k.create_market("n2", 2000.0).unwrap();

        let result = k.resolve_all(&["n1".to_string()]).unwrap();
        assert_eq!(result.markets_resolved, 2);
        assert_eq!(k.markets["n1"].resolved, Some(true));
        assert_eq!(k.markets["n2"].resolved, Some(false));
    }

    #[test]
    fn test_market_ticker() {
        let mut k = Kernel::new();
        k.append(make_node("n1", "A0", "a", vec![])).unwrap();
        k.append(make_node("n2", "A1", "b", vec![])).unwrap();
        k.create_market("n1", 2000.0).unwrap();
        k.create_market("n2", 2000.0).unwrap();
        k.buy_yes("n1", 100.0).unwrap();

        let ticker = k.market_ticker(10);
        assert_eq!(ticker.len(), 2);
        assert_eq!(ticker[0].0, "n1");
    }
}

```

---

# XREF: shipped src/state/q_state.rs (NodeId tuple struct + Q_t)

```rust
//! Q_t — system state vector per `STATE_TRANSITION_SPEC v1.4 § 1.1`.
//!
//! TRACE_MATRIX Art 0.1 — 四要素映射: `QState` provides the tape/control mapping.
//! TRACE_MATRIX Art 0.4 — Q_t version-controlled: `head_t` = git commit SHA in Path B substrate.
//! TRACE_MATRIX Art IV — Boot: `QState::genesis` is the starting state of every runtime.
//! TRACE_MATRIX WP § 0 axiom 1 — state monotonicity: Q_t evolves only via accepted transitions.
//! TRACE_MATRIX WP § 4 — 9-component system state.
//! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
//!
//! **BTreeMap, not HashMap, everywhere** (Inv determinism;
//! Codex flagged `kernel.rs:187-204` HashMap nondeterminism in round-2).
//!
//! Sub-types whose entry shapes are scoped to later atoms (CO P2.x economic engine,
//! CO1.7 transition_ledger) are intentionally minimal here — full schemas land per atom,
//! but the *index typing* (BTreeMap newtype shells) freezes here so Q_t is total.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::economy::money::MicroCoin;

// ────────────────────────────────────────────────────────────────────────────
// Newtype primitives — minimal, deterministic, serde-ready.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — generic 32-byte hash (sha256). State / ledger / registry roots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    /// TRACE_MATRIX § 1.1 — additive identity (genesis state-root, ledger-root, etc.).
    pub const ZERO: Hash = Hash([0u8; 32]);

    /// TRACE_MATRIX § 1.1 — construct from a 32-byte digest (sha256 output).
    pub fn from_bytes(b: [u8; 32]) -> Self {
        Hash(b)
    }
}

impl Default for Hash {
    fn default() -> Self {
        Hash::ZERO
    }
}

/// TRACE_MATRIX Art 0.4 — `head_t` = git commit SHA in Path B substrate (40 hex chars).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct NodeId(pub String);

impl NodeId {
    /// TRACE_MATRIX § 3 — pseudocode `NodeId::from_state_root(state_root)` constructor.
    /// Concrete derivation (commit-tree-of-state-root) lands in CO1.7 transition_ledger.
    pub fn from_state_root(state_root: Hash) -> Self {
        let mut s = String::with_capacity(64);
        for byte in state_root.0.iter() {
            s.push_str(&format!("{:02x}", byte));
        }
        NodeId(s)
    }
}

/// TRACE_MATRIX § 1.1 — agent identity (string, opaque to Q_t).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct AgentId(pub String);

/// TRACE_MATRIX § 1.1 — accepted-transaction id (string, opaque to Q_t).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct TxId(pub String);

/// TRACE_MATRIX § 1.1 — reputation snapshot. Signed i64 to permit negative reputation
/// (e.g. post-slash); ledger-of-record lives in `ReputationsIndex` (CO P2.9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct Reputation(pub i64);

// ────────────────────────────────────────────────────────────────────────────
// AgentSwarmState + PerAgentState — spec § 1.1 verbatim.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — agent swarm sub-state.
/// MUST be reconstructible from L4 transition ledger replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AgentSwarmState {
    pub agents: BTreeMap<AgentId, PerAgentState>,
    pub current_round: u64,
}

/// TRACE_MATRIX § 1.1 — per-agent runtime state.
/// `retry_counter_for_current_task` resets on accept; persists across rejections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PerAgentState {
    pub reputation_snapshot: Reputation,
    pub last_accepted_tx: Option<TxId>,
    pub retry_counter_for_current_task: u32,
}

// ────────────────────────────────────────────────────────────────────────────
// AgentVisibleProjection — Inv 10 Goodhart shield (CO P2.7 visibility runtime).
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — agent-visible projection of tape filtered by per-agent
/// visibility policy (Inv 10 Goodhart shield; `top_white::predicates::visibility`).
///
/// `views`: per-agent filtered head pointer; full filtering machinery lands in CO P2.7.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AgentVisibleProjection {
    pub views: BTreeMap<AgentId, NodeId>,
}

// ────────────────────────────────────────────────────────────────────────────
// BudgetSnapshot — global compute / cost / wall-clock budget.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — global budget snapshot:
/// cost ceiling (MicroCoin), wall clock remaining (ms), compute cap remaining.
/// Exhaustion → halt_reason ∈ {WallClockCap, ComputeCapViolated, MaxTxExhausted}.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetSnapshot {
    pub cost_ceiling_microcoin: MicroCoin,
    pub wall_clock_remaining_ms: u64,
    pub compute_cap_remaining: u64,
}

impl Default for BudgetSnapshot {
    fn default() -> Self {
        Self {
            cost_ceiling_microcoin: MicroCoin::zero(),
            wall_clock_remaining_ms: 0,
            compute_cap_remaining: 0,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX WP § 2 economic — 9-sub-field economic state. Each sub-index
/// is a BTreeMap newtype; entry shapes (Escrow / Stake / Claim / TaskMarket /
/// RoyaltyEdge / ChallengeCase) are minimal-but-typed here and fully fleshed
/// in the owning atoms (CO P2.1-2.6).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EconomicState {
    pub balances_t: BalancesIndex,
    pub escrows_t: EscrowsIndex,
    pub stakes_t: StakesIndex,
    pub claims_t: ClaimsIndex,
    pub reputations_t: ReputationsIndex,
    pub task_markets_t: TaskMarketsIndex,
    pub royalty_graph_t: RoyaltyGraph,
    pub challenge_cases_t: ChallengeCasesIndex,
    pub price_index_t: PriceIndex,
}

/// TRACE_MATRIX WP § 2 — agent → balance ledger. Concrete entry: `MicroCoin` (CO1.0a).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BalancesIndex(pub BTreeMap<AgentId, MicroCoin>);

/// TRACE_MATRIX WP § 2 — tx → escrow entry. Full schema lands CO P2.2 EscrowVault.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EscrowsIndex(pub BTreeMap<TxId, EscrowEntry>);

/// TRACE_MATRIX WP § 2 — escrow entry shape (stub). Full fields land CO P2.2.
/// `#[serde(default)]` on each field gives forward-compat: future atoms can add
/// fields without breaking deserialization of historical ledger rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EscrowEntry {
    #[serde(default = "MicroCoin::zero")]
    pub amount: MicroCoin,
    #[serde(default)]
    pub depositor: AgentId,
}

impl Default for EscrowEntry {
    fn default() -> Self {
        Self { amount: MicroCoin::zero(), depositor: AgentId::default() }
    }
}

/// TRACE_MATRIX WP § 2 — tx → stake entry. Full schema lands CO P2.5 ChallengeCourt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StakesIndex(pub BTreeMap<TxId, StakeEntry>);

/// TRACE_MATRIX WP § 2 — stake entry shape (stub). Full fields land CO P2.5.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StakeEntry {
    #[serde(default = "MicroCoin::zero")]
    pub amount: MicroCoin,
    #[serde(default)]
    pub staker: AgentId,
}

impl Default for StakeEntry {
    fn default() -> Self {
        Self { amount: MicroCoin::zero(), staker: AgentId::default() }
    }
}

/// TRACE_MATRIX WP § 2 — tx → reward claim. Full schema lands CO P2.6 SettlementEngine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ClaimsIndex(pub BTreeMap<TxId, ClaimEntry>);

/// TRACE_MATRIX WP § 2 — claim entry shape (stub). Full fields land CO P2.6.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimEntry {
    #[serde(default = "MicroCoin::zero")]
    pub amount: MicroCoin,
    #[serde(default)]
    pub claimant: AgentId,
}

impl Default for ClaimEntry {
    fn default() -> Self {
        Self { amount: MicroCoin::zero(), claimant: AgentId::default() }
    }
}

/// TRACE_MATRIX WP § 2 — agent → reputation ledger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReputationsIndex(pub BTreeMap<AgentId, Reputation>);

/// TRACE_MATRIX WP § 2 — tx → task market. Full schema lands CO P2.1.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskMarketsIndex(pub BTreeMap<TxId, TaskMarketEntry>);

/// TRACE_MATRIX WP § 2 — task market entry shape (stub). Full fields land CO P2.1.
/// Default values (verifier_quorum=1, max_reuse_royalty_fraction=0.10) match the
/// PROJECT_DECISION_MAP § 2.3 spec gap defaults.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskMarketEntry {
    #[serde(default)]
    pub publisher: AgentId,
    #[serde(default = "MicroCoin::zero")]
    pub bounty: MicroCoin,
    #[serde(default = "task_market_default_quorum")]
    pub verifier_quorum: u32,
    #[serde(default = "task_market_default_royalty_bp")]
    pub max_reuse_royalty_fraction_basis_points: u16,
}

fn task_market_default_quorum() -> u32 {
    1
}
fn task_market_default_royalty_bp() -> u16 {
    1000
}

impl Default for TaskMarketEntry {
    fn default() -> Self {
        Self {
            publisher: AgentId::default(),
            bounty: MicroCoin::zero(),
            verifier_quorum: 1,
            max_reuse_royalty_fraction_basis_points: 1000, // 0.10 per spec gap default
        }
    }
}

/// TRACE_MATRIX WP § 2 — directed royalty edges (reuse depth attribution).
/// Full attribution algebra lands CO P2.4.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RoyaltyGraph(pub BTreeMap<TxId, Vec<RoyaltyEdge>>);

/// TRACE_MATRIX WP § 2 — single royalty edge (ancestor → reuse weight). Stub; CO P2.4.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RoyaltyEdge {
    #[serde(default)]
    pub ancestor: TxId,
    #[serde(default)]
    pub fraction_basis_points: u16,
}

/// TRACE_MATRIX WP § 2 — tx → challenge case. Full schema lands CO P2.5.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChallengeCasesIndex(pub BTreeMap<TxId, ChallengeCase>);

/// TRACE_MATRIX WP § 2 — challenge case shape (stub). Full fields land CO P2.5.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeCase {
    #[serde(default)]
    pub challenger: AgentId,
    #[serde(default = "MicroCoin::zero")]
    pub bond: MicroCoin,
    #[serde(default)]
    pub opened_at_round: u64,
}

impl Default for ChallengeCase {
    fn default() -> Self {
        Self { challenger: AgentId::default(), bond: MicroCoin::zero(), opened_at_round: 0 }
    }
}

/// TRACE_MATRIX WP § 2 — tx → posted price (last accepted price index).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PriceIndex(pub BTreeMap<TxId, MicroCoin>);

// ────────────────────────────────────────────────────────────────────────────
// QState — § 1.1 verbatim, 9 fields.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — system state Q_t. 9 fields per WP § 4 + economic § 2 amendment.
///
/// Reconstructibility: every field is derivable from L4 transition ledger replay
/// (Art IV Boot 公理).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QState {
    /// Agent swarm sub-state (tape head per agent + per-agent reputation snapshots).
    pub q_t: AgentSwarmState,
    /// Current ChainTape head pointer = git commit SHA in Path B substrate.
    pub head_t: NodeId,
    /// Materialized state Merkle root (git tree root in Path B).
    pub state_root_t: Hash,
    /// Agent-visible projection of tape filtered by per-agent visibility policy.
    pub tape_view_t: AgentVisibleProjection,
    /// L4 Transition Ledger root (Merkle root of all accepted tx so far).
    pub ledger_root_t: Hash,
    /// L1 Predicate Registry root.
    pub predicate_registry_root_t: Hash,
    /// L2 Tool Registry root.
    pub tool_registry_root_t: Hash,
    /// Economic state (WP § 2 amendment, 9 sub-fields).
    pub economic_state_t: EconomicState,
    /// Global budget snapshot.
    pub budget_state_t: BudgetSnapshot,
}

impl QState {
    /// TRACE_MATRIX Art IV Boot — genesis Q_t. All zero / empty;
    /// roots populated by `boot::verify_trust_root` and the `state_root_t` published
    /// in `genesis_payload.toml [constitution_root]`.
    pub fn genesis() -> Self {
        QState::default()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Inline determinism tests (round-trip + insertion-order independence).
// Conformance tests proper live in tests/{four_element_mapping, q_state_reconstruct,
// economic_state_reconstruct, six_axioms_alignment}.rs per TRACE_MATRIX_v3.
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_q_state_is_total_and_default() {
        let g = QState::genesis();
        assert_eq!(g, QState::default());
        assert_eq!(g.q_t.current_round, 0);
        assert!(g.q_t.agents.is_empty());
        assert_eq!(g.head_t, NodeId::default());
        assert_eq!(g.state_root_t, Hash::ZERO);
    }

    #[test]
    fn nine_field_count_via_serde_json() {
        // Sanity that QState has exactly 9 top-level fields.
        let s = serde_json::to_value(QState::genesis()).unwrap();
        let obj = s.as_object().expect("object");
        assert_eq!(
            obj.len(),
            9,
            "QState must have exactly 9 fields per WP § 4; got {}",
            obj.len()
        );
        for k in &[
            "q_t",
            "head_t",
            "state_root_t",
            "tape_view_t",
            "ledger_root_t",
            "predicate_registry_root_t",
            "tool_registry_root_t",
            "economic_state_t",
            "budget_state_t",
        ] {
            assert!(obj.contains_key(*k), "QState missing field {}", k);
        }
    }

    #[test]
    fn economic_state_has_nine_sub_fields() {
        let e = EconomicState::default();
        let s = serde_json::to_value(&e).unwrap();
        let obj = s.as_object().unwrap();
        assert_eq!(
            obj.len(),
            9,
            "EconomicState must have 9 sub-fields per WP § 2; got {}",
            obj.len()
        );
    }

    #[test]
    fn btreemap_insertion_order_independent_serialization() {
        // Insertion-order independence (Inv determinism).
        let mut a = BalancesIndex::default();
        a.0.insert(AgentId("alice".into()), MicroCoin::from_coin(10).unwrap());
        a.0.insert(AgentId("bob".into()), MicroCoin::from_coin(20).unwrap());

        let mut b = BalancesIndex::default();
        b.0.insert(AgentId("bob".into()), MicroCoin::from_coin(20).unwrap());
        b.0.insert(AgentId("alice".into()), MicroCoin::from_coin(10).unwrap());

        let sa = serde_json::to_string(&a).unwrap();
        let sb = serde_json::to_string(&b).unwrap();
        assert_eq!(sa, sb, "BTreeMap must yield identical bytes regardless of insertion order");
    }

    #[test]
    fn node_id_from_state_root_is_deterministic() {
        let r = Hash::from_bytes([0xAB; 32]);
        let n1 = NodeId::from_state_root(r);
        let n2 = NodeId::from_state_root(r);
        assert_eq!(n1, n2);
        assert_eq!(n1.0.len(), 64, "40-byte git SHA hex form would be 40; we use full 32-byte sha256 hex = 64");
    }
}

```

---

# XREF: shipped src/bottom_white/cas/store.rs (D4 test surfaces)

```rust
//! CAS store backed by git2-rs blob layer.
//!
//! Each runtime_repo (per spec § 5.2.2 cell isolation) has its own CasStore.
//! Objects are content-addressed by `Cid` (sha256 of content); git's sha-1
//! OID is recorded but not canonical.
//!
//! **CO1.4-extra (this atom)** adds index persistence: the `Cid → metadata`
//! map is durably persisted to a sidecar JSONL file at
//! `<repo_path>/.turingos_cas_index.jsonl`. On `CasStore::open()` the sidecar
//! is replayed into an in-memory BTreeMap; on `CasStore::put()` (new entries
//! only) one JSONL line is appended + flushed. This closes the Art 0.2
//! tape-canonicality cold-replay gate that CO1.7 spec § 0 + CO1.1.4-pre1
//! v1.1 § 0.1 declared a hard prerequisite for `replay_full_transition`
//! (CO1.7-impl A4).
//!
//! **Design choice (sidecar JSONL)**: chosen over (b) git-tag manifest /
//! (c) bincode index + WAL because (a) is the simplest deterministic
//! append-only artifact, replayable from scratch, easy to audit by reading.
//! Per "压缩即智能" — pick simplest correct shape; upgrade later if profiling
//! shows O(N)-on-restart cost is real.
//!
//! /// TRACE_MATRIX WP-arch-§5.L3 + spec-§5.2.2 (cell isolation): CAS store
//! /// TRACE_MATRIX CO1.7 spec § 0 + CO1.1.4-pre1 § 0.1 cross-atom ordering:
//! /// CAS index persistence — required by `replay_full_transition` cold-restart.

use git2::{ObjectType as Git2ObjectType, Repository};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use super::schema::{CasObjectMetadata, Cid, ObjectType};

const CAS_INDEX_FILENAME: &str = ".turingos_cas_index.jsonl";

#[derive(Debug)]
pub enum CasError {
    /// git2-rs underlying error.
    Git2(git2::Error),
    /// Cid not found in this CasStore's metadata index.
    CidNotFound(Cid),
    /// Content stored at git OID but Cid metadata absent (corrupted index).
    MetadataMissing(Cid),
    /// Content's sha256 doesn't match the asserted Cid (corruption).
    CidMismatch { expected: Cid, computed: Cid },
    /// I/O error reading or writing the CO1.4-extra sidecar index file.
    IoError(io::Error),
    /// JSON-deserialization error on a sidecar index line. Includes 1-based
    /// line number for diagnostics.
    IndexParse { line: usize, error: String },
}

impl std::fmt::Display for CasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Git2(e) => write!(f, "git2 backend error: {e}"),
            Self::CidNotFound(c) => write!(f, "{c} not found in CAS index"),
            Self::MetadataMissing(c) => write!(f, "{c} metadata missing (index corrupted)"),
            Self::CidMismatch { expected, computed } => write!(
                f,
                "CAS content corruption: expected {expected}, computed {computed}"
            ),
            Self::IoError(e) => write!(f, "cas index I/O error: {e}"),
            Self::IndexParse { line, error } => {
                write!(f, "cas index parse error at line {line}: {error}")
            }
        }
    }
}

impl std::error::Error for CasError {}

impl From<git2::Error> for CasError {
    fn from(e: git2::Error) -> Self {
        Self::Git2(e)
    }
}

impl From<io::Error> for CasError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

fn cas_index_path(repo_path: &Path) -> PathBuf {
    repo_path.join(CAS_INDEX_FILENAME)
}

/// CO1.4-extra: read the sidecar JSONL into an in-memory index.
/// Strict mode — any malformed line aborts the load (per Art 0.2: a
/// corrupted index means the tape is non-canonical; abort + diagnose
/// is more honest than skip-and-warn).
fn load_index_from_sidecar(repo_path: &Path) -> Result<BTreeMap<Cid, CasObjectMetadata>, CasError> {
    let path = cas_index_path(repo_path);
    let mut index = BTreeMap::new();
    if !path.exists() {
        return Ok(index);
    }
    let content = std::fs::read_to_string(&path)?;
    for (i, line) in content.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let meta: CasObjectMetadata =
            serde_json::from_str(line).map_err(|e| CasError::IndexParse {
                line: i + 1,
                error: e.to_string(),
            })?;
        index.insert(meta.cid, meta);
    }
    Ok(index)
}

/// CO1.4-extra: append a single JSONL line for a newly-created CAS object.
/// Followed by `sync_data` for durability (single-writer per cell per spec
/// § 5.2.2 — concurrent-writer atomicity is out of scope).
fn append_to_sidecar(repo_path: &Path, meta: &CasObjectMetadata) -> Result<(), CasError> {
    let path = cas_index_path(repo_path);
    let serialized = serde_json::to_string(meta).map_err(|e| CasError::IndexParse {
        line: 0,
        error: format!("serialize: {e}"),
    })?;
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    f.write_all(serialized.as_bytes())?;
    f.write_all(b"\n")?;
    f.sync_data()?;
    Ok(())
}

/// Content-addressable store backed by git's blob object database.
#[derive(Debug)]
pub struct CasStore {
    repo_path: PathBuf,
    /// Cid → metadata index. BTreeMap per spec § 2 I-BTREE.
    index: BTreeMap<Cid, CasObjectMetadata>,
}

impl CasStore {
    /// Open or initialize a CAS store at the given runtime_repo path.
    /// Creates the git repo if it doesn't exist. **CO1.4-extra**: replays
    /// the sidecar `.turingos_cas_index.jsonl` (if any) into the in-memory
    /// index, restoring all metadata that was durably appended in prior
    /// sessions.
    pub fn open(repo_path: &Path) -> Result<Self, CasError> {
        let repo_path = repo_path.to_path_buf();
        let _repo = match Repository::open(&repo_path) {
            Ok(r) => r,
            Err(_) => Repository::init(&repo_path)?,
        };
        let index = load_index_from_sidecar(&repo_path)?;
        Ok(Self { repo_path, index })
    }

    fn open_repo(&self) -> Result<Repository, CasError> {
        Repository::open(&self.repo_path).map_err(CasError::from)
    }

    /// Store content; returns its Cid. Idempotent — same content → same Cid.
    pub fn put(
        &mut self,
        content: &[u8],
        object_type: ObjectType,
        creator: &str,
        created_at_logical_t: u64,
        schema_id: Option<String>,
    ) -> Result<Cid, CasError> {
        let cid = Cid::from_content(content);
        let repo = self.open_repo()?;
        let git_oid = repo.blob(content)?;

        // If already in index, idempotent: just return Cid (content addressing
        // guarantees same content → same Cid → already present)
        if self.index.contains_key(&cid) {
            return Ok(cid);
        }

        let metadata = CasObjectMetadata {
            cid,
            backend_oid_hex: git_oid.to_string(),
            object_type,
            creator: creator.to_string(),
            created_at_logical_t,
            schema_id,
            size_bytes: content.len() as u64,
        };
        // CO1.4-extra: durably append BEFORE inserting into in-memory index
        // (so a crash mid-write leaves the runtime in a consistent state —
        // either the entry is durably recorded AND in-memory, or neither).
        append_to_sidecar(&self.repo_path, &metadata)?;
        self.index.insert(cid, metadata);
        Ok(cid)
    }

    /// Retrieve content by Cid. Verifies content sha256 matches Cid (corruption check).
    pub fn get(&self, cid: &Cid) -> Result<Vec<u8>, CasError> {
        let metadata = self
            .index
            .get(cid)
            .ok_or(CasError::CidNotFound(*cid))?;
        let repo = self.open_repo()?;
        let git_oid = git2::Oid::from_str(&metadata.backend_oid_hex)
            .map_err(CasError::Git2)?;
        let blob = repo.find_blob(git_oid)?;
        let content = blob.content().to_vec();

        // Verify content sha256 matches Cid (defense against corruption).
        let mut h = Sha256::new();
        h.update(&content);
        let computed = Cid(h.finalize().into());
        if &computed != cid {
            return Err(CasError::CidMismatch {
                expected: *cid,
                computed,
            });
        }

        Ok(content)
    }

    /// Get metadata only (no content fetch).
    pub fn metadata(&self, cid: &Cid) -> Option<&CasObjectMetadata> {
        self.index.get(cid)
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Merkle root over all CAS object metadata; deterministic per BTreeMap order.
    pub fn merkle_root(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        for (_cid, meta) in &self.index {
            h.update(meta.canonical_hash());
        }
        h.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn fresh_store() -> (TempDir, CasStore) {
        let tmp = TempDir::new().unwrap();
        let store = CasStore::open(tmp.path()).unwrap();
        (tmp, store)
    }

    #[test]
    fn put_get_round_trip_small() {
        let (_tmp, mut s) = fresh_store();
        let cid = s.put(b"hello world", ObjectType::ProposalPayload, "alice", 100, None).unwrap();
        let content = s.get(&cid).unwrap();
        assert_eq!(content, b"hello world");
    }

    #[test]
    fn put_get_round_trip_large() {
        let (_tmp, mut s) = fresh_store();
        let big = vec![0xab; 65536];
        let cid = s.put(&big, ObjectType::PredicateBytecode, "system", 0, Some("wasm".into())).unwrap();
        let content = s.get(&cid).unwrap();
        assert_eq!(content, big);
    }

    #[test]
    fn put_idempotent_same_content() {
        let (_tmp, mut s) = fresh_store();
        let cid_a = s.put(b"x", ObjectType::Generic, "alice", 1, None).unwrap();
        let cid_b = s.put(b"x", ObjectType::Generic, "bob", 2, None).unwrap();
        assert_eq!(cid_a, cid_b, "same content → same Cid");
        // Index size = 1 (idempotent)
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn cid_is_content_address() {
        let (_tmp, mut s) = fresh_store();
        let cid = s.put(b"specific content", ObjectType::Generic, "system", 0, None).unwrap();
        // Cid is sha256 of content; verifiable independently
        let expected = Cid::from_content(b"specific content");
        assert_eq!(cid, expected);
    }

    #[test]
    fn get_nonexistent_returns_error() {
        let (_tmp, s) = fresh_store();
        let bogus = Cid([0u8; 32]);
        match s.get(&bogus) {
            Err(CasError::CidNotFound(c)) => assert_eq!(c, bogus),
            other => panic!("expected CidNotFound, got {other:?}"),
        }
    }

    #[test]
    fn metadata_recorded() {
        let (_tmp, mut s) = fresh_store();
        let cid = s.put(b"meta test", ObjectType::CounterexamplePayload, "carol", 250, Some("v1".into())).unwrap();
        let meta = s.metadata(&cid).unwrap();
        assert_eq!(meta.cid, cid);
        assert_eq!(meta.object_type, ObjectType::CounterexamplePayload);
        assert_eq!(meta.creator, "carol");
        assert_eq!(meta.created_at_logical_t, 250);
        assert_eq!(meta.schema_id.as_deref(), Some("v1"));
        assert_eq!(meta.size_bytes, 9);
    }

    #[test]
    fn merkle_root_deterministic_two_runs() {
        let (_tmp1, mut s1) = fresh_store();
        let (_tmp2, mut s2) = fresh_store();
        for content in [b"a".as_slice(), b"b".as_slice(), b"c".as_slice()] {
            s1.put(content, ObjectType::Generic, "system", 0, None).unwrap();
        }
        // Different insertion order
        for content in [b"c".as_slice(), b"b".as_slice(), b"a".as_slice()] {
            s2.put(content, ObjectType::Generic, "system", 0, None).unwrap();
        }
        assert_eq!(s1.merkle_root(), s2.merkle_root(),
            "BTreeMap-ordered: insertion order independent (I-DET)");
    }

    #[test]
    fn empty_store_root() {
        let (_tmp, s) = fresh_store();
        let r = s.merkle_root();
        let expected: [u8; 32] = Sha256::new().finalize().into();
        assert_eq!(r, expected, "empty store root = sha256(empty)");
    }

    #[test]
    fn cell_isolation_disjoint_cas() {
        // Per spec § 5.2.2 cross-cell isolation: separate runtime_repo paths
        // → completely disjoint CasStore instances.
        let (_tmp_a, mut store_a) = fresh_store();
        let (_tmp_b, mut store_b) = fresh_store();

        let cid_a = store_a.put(b"only in a", ObjectType::Generic, "agent_a", 100, None).unwrap();
        let cid_b = store_b.put(b"only in b", ObjectType::Generic, "agent_b", 100, None).unwrap();

        // Each store has its own object only
        assert!(store_a.get(&cid_a).is_ok(), "store_a has cid_a");
        assert!(store_a.get(&cid_b).is_err(), "store_a lacks cid_b (isolated)");
        assert!(store_b.get(&cid_b).is_ok(), "store_b has cid_b");
        assert!(store_b.get(&cid_a).is_err(), "store_b lacks cid_a (isolated)");
    }

    #[test]
    fn put_many_then_iterate_count() {
        let (_tmp, mut s) = fresh_store();
        for i in 0..50 {
            s.put(
                format!("content {i}").as_bytes(),
                ObjectType::ProposalPayload,
                "system",
                i as u64,
                None,
            )
            .unwrap();
        }
        assert_eq!(s.len(), 50);
        assert!(!s.is_empty());
    }

    // ── CO1.4-extra: sidecar JSONL persistence tests ─────────────────────────

    /// Cold-restart: reopen recovers all metadata; get() works post-reopen
    /// (closes the Art 0.2 cold-replay gate that CO1.7-impl A4 needs).
    #[test]
    fn reopen_recovers_index_and_get_works() {
        let tmp = TempDir::new().expect("tempdir");
        let cid_a;
        let cid_b;
        {
            let mut s = CasStore::open(tmp.path()).expect("open");
            cid_a = s
                .put(b"alpha", ObjectType::ProposalPayload, "alice", 1, None)
                .unwrap();
            cid_b = s
                .put(b"beta", ObjectType::CounterexamplePayload, "bob", 2, Some("s.v1".into()))
                .unwrap();
        }
        // Reopen: in-memory store is fresh; sidecar replay is the ONLY way
        // metadata survives.
        let s2 = CasStore::open(tmp.path()).expect("reopen");
        assert_eq!(s2.len(), 2);
        assert_eq!(s2.get(&cid_a).expect("get a"), b"alpha");
        assert_eq!(s2.get(&cid_b).expect("get b"), b"beta");

        let meta_b = s2.metadata(&cid_b).expect("metadata b");
        assert_eq!(meta_b.creator, "bob");
        assert_eq!(meta_b.created_at_logical_t, 2);
        assert_eq!(meta_b.schema_id.as_deref(), Some("s.v1"));
        assert_eq!(meta_b.object_type, ObjectType::CounterexamplePayload);
    }

    /// Idempotent put: same content twice → same Cid → only ONE sidecar line.
    #[test]
    fn idempotent_put_does_not_duplicate_sidecar_line() {
        let tmp = TempDir::new().expect("tempdir");
        let mut s = CasStore::open(tmp.path()).expect("open");
        let _ = s
            .put(b"content", ObjectType::Generic, "alice", 1, None)
            .unwrap();
        let _ = s
            .put(b"content", ObjectType::Generic, "alice", 1, None)
            .unwrap();
        let path = cas_index_path(tmp.path());
        let lines: Vec<&str> = std::fs::read_to_string(&path)
            .unwrap()
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                // own the str via leak — cheap for test
                Box::leak(l.to_string().into_boxed_str()) as &str
            })
            .collect();
        assert_eq!(lines.len(), 1, "idempotent put should produce 1 sidecar line, got {}", lines.len());
    }

    /// Append-only: each NEW put adds exactly ONE line.
    #[test]
    fn each_new_put_appends_one_line() {
        let tmp = TempDir::new().expect("tempdir");
        let mut s = CasStore::open(tmp.path()).expect("open");
        for i in 0..5 {
            s.put(
                format!("c{i}").as_bytes(),
                ObjectType::Generic,
                "system",
                i,
                None,
            )
            .unwrap();
        }
        let path = cas_index_path(tmp.path());
        let line_count = std::fs::read_to_string(&path)
            .unwrap()
            .lines()
            .filter(|l| !l.is_empty())
            .count();
        assert_eq!(line_count, 5);
    }

    /// Corrupted JSONL → strict parse error with line number (not silent skip).
    #[test]
    fn corrupted_sidecar_line_returns_parse_error() {
        let tmp = TempDir::new().expect("tempdir");
        // Init repo + ONE valid put to get a known-good first line.
        {
            let mut s = CasStore::open(tmp.path()).expect("open");
            s.put(b"hello", ObjectType::Generic, "alice", 1, None).unwrap();
        }
        // Corrupt: append a malformed line.
        let path = cas_index_path(tmp.path());
        let mut f = OpenOptions::new().append(true).open(&path).unwrap();
        f.write_all(b"this is not valid json\n").unwrap();
        f.sync_data().unwrap();

        // Reopen MUST fail with a typed IndexParse error citing the line number.
        let err = CasStore::open(tmp.path()).unwrap_err();
        match err {
            CasError::IndexParse { line, .. } => {
                assert_eq!(line, 2, "expected line 2 to be flagged");
            }
            other => panic!("expected IndexParse, got {other:?}"),
        }
    }

    /// Empty / non-existent sidecar → opens fresh with empty index.
    #[test]
    fn missing_sidecar_opens_fresh() {
        let tmp = TempDir::new().expect("tempdir");
        let s = CasStore::open(tmp.path()).expect("open");
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
    }
}

```

---

Now give your INDEPENDENT round-2 audit. Cite spec § + line where possible.

codex
I’ll verify the spec against the checked-out files rather than relying only on the pasted excerpt, then I’ll answer each audit question with concrete line references.
exec
/bin/bash -lc "rg --files | rg '("'^|/)CLAUDE.md$|handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md|src/(state/sequencer.rs|bottom_white/ledger/transition_ledger.rs|bus.rs|kernel.rs|state/q_state.rs|bottom_white/cas/store.rs)$'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bus.rs
src/state/q_state.rs
src/state/sequencer.rs
src/bottom_white/ledger/transition_ledger.rs
src/bottom_white/cas/store.rs
src/kernel.rs
CLAUDE.md
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md

exec
/bin/bash -lc 'pwd && git rev-parse --short HEAD && git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4
617f01e
 M rules/enforcement.log
?? handover/audits/CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md
?? handover/audits/GEMINI_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md
?? handover/audits/run_codex_co1_7_extra_round2_audit.sh
?? handover/audits/run_gemini_co1_7_extra_round2_audit.py

exec
/bin/bash -lc 'find .. -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
 succeeded in 1320ms:
../turingos/AGENTS.md
../omega_kepler_discovery/AGENTS.md
../omega/AGENTS.md
../turingosv2/AGENTS.md
../turingclaw/AGENTS.md

codex
The workspace has untracked audit artifacts already, but I’m not using those for this pass. I’m reading the target spec and source surfaces directly so the verdict stays independent.
exec
/bin/bash -lc "nl -ba handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# CO1.7-extra: L4 head_t close + Sequencer entry-point wiring v1 (post round-1 scope split)
     2	
     3	**Status**: v1 DRAFT (2026-04-29; post round-1 dual external audit on prior bundled CO1.7.5 spec). Scope-split executed by ArchitectAI per Occam + Anti-Oreo (see § 0.1). Awaiting round-2 dual external audit.
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
    14	**Single sentence**: close the G-1 carry-forward `q.head_t = NodeId(commit_oid_hex)` after `Git2LedgerWriter.commit`, perform combined STEP_B ceremony adding a Sequencer entry-point on TuringBus + Kernel, and ship one substrate-independent CAS round-trip test — leaving transition function bodies + replay byte-identity to a future CO1.7.5 atom that depends on the Wave-2 substrate (CO P2.x family).
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
    44	2. **D3** — Combined STEP_B ceremony adds `Option<Arc<Sequencer>>` field to `Kernel` + `submit_typed_tx` forwarder method on `TuringBus` (note: type is `TuringBus`, not `Bus`, per `src/bus.rs:53`). Sequencer instance lives in Kernel; TuringBus forwards via `self.kernel.sequencer`.
    45	3. **D4-substrate-independent** — One conformance test `tests/cas_payload_round_trip` (`CasStore::put` → `get` round-trip with CID stability post-CO1.4-extra). Other 3 D4 tests (replay state-root + system-signature canonical-message + un-ignore byte-identity) move to future CO1.7.5 atom because they require D1 transition bodies to actually commit.
    46	
    47	### 0.4 Process commitment (active reconciliation per Gemini MF1+MF3 + Codex Q-A v1.1 ask)
    48	
    49	The two STATE_TRANSITION_SPEC § 3 supersessions previously declared in the prior CO1.7.5 v1 spec (NodeId head_t binding + SignalKind 4-variant minimization) **continue to apply** — but no longer in scope for CO1.7-extra (which doesn't contain transition bodies). They migrate intact to the future CO1.7.5 atom.
    50	
    51	**Asserted authority principle** (strengthened per Gemini MF3): a later, more specific, audited spec (CO1.7 v1.2 round-3 PASS/PASS; CO1.1.4-pre1 PASS/PASS) **legitimately supersedes** earlier general specs (STATE v1.4 round-4 PASS/PASS) within the layered boundary the later spec covers. This is consistent with the project's atom-decomposition pattern: each atom locks its own surface; downstream atoms refine via PASS/PASS audit, not by editing upstream artifacts.
    52	
    53	**Institutional debt acknowledged** (per Gemini MF1): as part of CO1.7-extra atom closure, ArchitectAI commits to filing a STATE_TRANSITION_SPEC v1.5 housekeeping issue (one paragraph noting the two supersessions from CO1.7 K3 v1.2 + CO1.1.4-pre1 with backlinks) — NOT a re-audit, just an annotation pass that prevents future readers from being confused by the historical drafting language. Tracked as part of the post-PASS/PASS landing checklist (§ 8 awaiting list).
    54	
    55	---
    56	
    57	## § 1 D2 — head_t close
    58	
    59	### 1.1 Code change
    60	
    61	In `src/state/sequencer.rs::apply_one` stage 9 (currently lines 362-373), one additional assignment after `writer_w.commit(&entry)?`:
    62	
    63	```rust
    64	// Stage 9 (CO1.7-extra D2): commit + mutate Q_t under write lock.
    65	let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
    66	let mut writer_w = self.ledger_writer.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
    67	writer_w.commit(&entry)?;
    68	self.next_logical_t.store(logical_t, Ordering::SeqCst);
    69	*q_w = q_next;
    70	q_w.ledger_root_t = entry.resulting_ledger_root;
    71	// NEW (CO1.7-extra D2): close G-1 head_t carry-forward.
    72	if let Some(commit_oid_hex) = writer_w.head_commit_oid_hex() {
    73	    q_w.head_t = crate::state::q_state::NodeId(commit_oid_hex);
    74	}
    75	```
    76	
    77	**NodeId disambiguation**: two `NodeId` types coexist — legacy `pub type NodeId = String` at `src/ledger.rs:13` (imported by TuringBus + Kernel for the legacy ledger event API) and new `pub struct NodeId(pub String)` at `src/state/q_state.rs:49`. `q.head_t` is typed as the new tuple-struct (`q_state.rs:311`); D2 constructs the new variant exclusively (legacy String alias is unused here).
    78	
    79	**Atomicity** (per Codex Q-B finding, refined): under acquired `q_w` + `writer_w` write locks, after `writer_w.commit(&entry)?` returns `Ok`, the remaining operations are an `AtomicU64::store` (infallible), a plain `*q_w = q_next` move (infallible), and field assignments (infallible). The atomicity claim fully holds for writers whose `head_commit_oid_hex` returns `Some` (Git2LedgerWriter); writers returning `None` (InMemoryLedgerWriter) leave `q.head_t` unchanged from `q_next.head_t` (which equals `q.head_t` per CO1.7 K3 v1.2 — transition bodies don't mutate head_t even when they exist in CO1.7.5).
    80	
    81	### 1.2 Trait method addition (Q1 synthesis: default None + mandatory override + defensive test)
    82	
    83	`LedgerWriter` trait at `src/bottom_white/ledger/transition_ledger.rs` gains one method:
    84	
    85	```rust
    86	pub trait LedgerWriter: Send + Sync {
    87	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
    88	    fn len(&self) -> u64;
    89	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;  // (existing; spec preserves)
    90	
    91	    /// NEW (CO1.7-extra D2): canonical 40-char lowercase hex commit OID of the
    92	    /// most recent appended entry, or None if the chain is empty / backend has
    93	    /// no commit-OID notion.
    94	    ///
    95	    /// **Q1 synthesis** (round-1 audit): default returns None to preserve
    96	    /// post-commit no-failure goal (avoid panic-after-commit-success per Codex
    97	    /// Q-B); BUT every shipped LedgerWriter impl MUST explicitly override this
    98	    /// (Gemini Q8 silent-stagnation defense). Defensive test
    99	    /// `git2_writer_returns_some_after_commit` (§ 3) asserts Git2LedgerWriter
   100	    /// returns Some at commit time, catching silent stagnation bugs in CI.
   101	    /// The default-None impl is intentionally dead code in production.
   102	    fn head_commit_oid_hex(&self) -> Option<String> {
   103	        None
   104	    }
   105	}
   106	
   107	impl LedgerWriter for Git2LedgerWriter {
   108	    fn head_commit_oid_hex(&self) -> Option<String> {
   109	        self.head_commit_oid().map(|oid| oid.to_string())
   110	    }
   111	    // ... existing commit / len / read_at ...
   112	}
   113	
   114	impl LedgerWriter for InMemoryLedgerWriter {
   115	    /// Explicit override (mandatory per Q1 synthesis). InMemory has no git
   116	    /// substrate, so always None — but the override is required to make the
   117	    /// "no implicit None" mandate enforceable (a missing override means the
   118	    /// dead-default is reached, which the defensive test will fail-fast on for
   119	    /// any code path that passes through Git2LedgerWriter).
   120	    fn head_commit_oid_hex(&self) -> Option<String> {
   121	        None
   122	    }
   123	    // ... existing ...
   124	}
   125	```
   126	
   127	---
   128	
   129	## § 2 D3 — Combined STEP_B ceremony for runtime entry-point
   130	
   131	### 2.1 Code change
   132	
   133	`src/kernel.rs` (currently `pub struct Kernel { ... }` with `Debug, Serialize, Deserialize` derives at line 18; documented as "pure topology" at line 15-17):
   134	
   135	```rust
   136	// src/kernel.rs (additive)
   137	pub struct Kernel {
   138	    // ... existing fields ...
   139	
   140	    /// NEW (CO1.7-extra D3): typed-tx Sequencer; None when kernel runs in
   141	    /// legacy-only mode (preserves back-compat with all existing tests).
   142	    /// Marked serde-skip because Sequencer holds Arc-locked CAS / writer state
   143	    /// that is constructed at runtime, not from on-disk Q_t snapshots.
   144	    #[serde(skip)]
   145	    pub sequencer: Option<Arc<Sequencer>>,
   146	}
   147	
   148	impl Kernel {
   149	    pub fn new() -> Self {
   150	        Self { /* ...existing..., */ sequencer: None }
   151	    }
   152	    /// NEW: opt-in constructor that wires a typed-tx Sequencer.
   153	    pub fn with_sequencer(/* …existing args…, */ sequencer: Arc<Sequencer>) -> Self {
   154	        Self { /* …existing…, */ sequencer: Some(sequencer) }
   155	    }
   156	}
   157	```
   158	
   159	`src/state/sequencer.rs` (Sequencer struct currently at lines 190-207 has no derives per Codex Q-C):
   160	
   161	```rust
   162	// src/state/sequencer.rs (additive — Debug derive needed for Kernel.Debug propagation)
   163	#[derive(Debug)]  // NEW (CO1.7-extra D3); Q1' open audit input
   164	pub struct Sequencer { /* ... */ }
   165	```
   166	
   167	`#[derive(Debug)]` may not propagate cleanly across `Arc<RwLock<dyn LedgerWriter>>` (the trait object in field position). If blanket derive fails at compile, manual impl uses `f.debug_struct("Sequencer").finish_non_exhaustive()` (sole open question Q1' below).
   168	
   169	`src/bus.rs` (note: actual struct name is **`TuringBus`** at `src/bus.rs:53`, NOT `Bus`):
   170	
   171	```rust
   172	// src/bus.rs (additive — NO new struct field)
   173	impl TuringBus {
   174	    /// NEW (CO1.7-extra D3): typed-tx submission path. Forwards to kernel-owned
   175	    /// Sequencer. Returns receipt (submit_id) immediately; commit happens
   176	    /// asynchronously in Sequencer::run driver loop.
   177	    pub async fn submit_typed_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
   178	        match self.kernel.sequencer.as_ref() {
   179	            Some(seq) => seq.submit(tx).await,
   180	            None => Err(SubmitError::QueueClosed),
   181	        }
   182	    }
   183	}
   184	```
   185	
   186	### 2.2 Sequencer placement justification (per Codex Q-C concern)
   187	
   188	`src/kernel.rs:15-17` doc says Kernel is "pure topology". Adding Sequencer as a new field appears to violate that descriptor at first glance. Resolution:
   189	
   190	1. Kernel already holds `Tape` + `NodeId` from the legacy ledger (`src/kernel.rs:8`) — these are "topology" elements (DAG structure + node identity). Sequencer is the typed-tx topology element (submission queue + driver loop ordering); it parallels the existing Tape/NodeId pattern.
   191	2. The actual state (`Q_t`) is owned by Sequencer, not Kernel. Kernel holds the *driver*, not the *data*.
   192	3. As part of this atom landing, the kernel.rs doc-comment is patched to: "topology layer: holds Tape, NodeId, and (post-CO1.7-extra) the typed-tx Sequencer driver. State data lives in Q_t inside Sequencer or in the legacy WAL ledger; this layer does NOT hold raw user-state."
   193	
   194	### 2.3 Combined ceremony justification (refined per Codex Q-C)
   195	
   196	Per `STEP_B_PROTOCOL.md` Phase 0, "minimum sufficient version" is technically **advisory** language asking auditors to favor the smallest change that works. CO1.7-extra rests the combined-ceremony argument on **functional coupling** (a stronger criterion):
   197	
   198	- The TuringBus forwarder reads `self.kernel.sequencer`; without the Kernel field, the forwarder fails to compile.
   199	- The Kernel field has no observable effect without an external caller; without the TuringBus forwarder, the field is dead code.
   200	
   201	Each half is a no-op without the other. A/B byte-identity testing each half independently would test two non-functional changes; combining them into one A/B unit tests the actual minimum-functional change. This is a **stronger** application of STEP_B's spirit than the per-file alternative.
   202	
   203	**Ceremony procedure**:
   204	1. Branch A (`step-b-co1.7-extra-A`): edits BOTH `src/bus.rs` (TuringBus forwarder) AND `src/kernel.rs` (Sequencer field + with_sequencer constructor) per § 2.1. Also adds the `#[derive(Debug)]` on Sequencer in `src/state/sequencer.rs` (NOT STEP_B-restricted; landed alongside the ceremony for compile coherence).
   205	2. Branch B (`step-b-co1.7-extra-B`): independently re-derives the same edits from this spec (separate session / context).
   206	3. Byte-identity comparison: `diff src/bus.rs && diff src/kernel.rs` between A and B. Both identical → merge to `main`. Either divergent → re-do the **whole** ceremony with stricter spec (no split-and-redo; coupled changes need coupled re-derivation).
   207	
   208	---
   209	
   210	## § 3 Test plan (substrate-independent)
   211	
   212	Two tests in `tests/co1_7_extra/`:
   213	
   214	### 3.1 `cas_payload_round_trip`
   215	
   216	```rust
   217	// tests/co1_7_extra/cas_payload_round_trip.rs (NEW)
   218	//! CO1.7-extra D4: CAS payload round-trip + CID stability across restart.
   219	//! Verifies that CO1.4-extra sidecar persistence makes CasStore content
   220	//! reachable across cold-start, which is a precondition for CO1.7.5

exec
/bin/bash -lc "nl -ba handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md | sed -n '221,460p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   221	//! FullTransition replay (deferred; gated on substrate atoms).
   222	//! Substrate-independent: uses only CasStore + ObjectType (CO1.4 + CO1.4-extra
   223	//! shipped surfaces); does NOT depend on CO P2.x.
   224	
   225	#[test]
   226	fn cas_payload_round_trip_with_cid_stability_across_restart() {
   227	    let tmp = tempfile::TempDir::new().expect("tempdir");
   228	    let payload = b"co1.7-extra-deterministic-payload-v1";
   229	    let cid_first = {
   230	        let mut cas = CasStore::open(tmp.path()).expect("first open");
   231	        cas.put(payload, ObjectType::ProposalPayload, "test-epoch", 1, Some("CO1.7-extra".into()))
   232	            .expect("put")
   233	    };
   234	    // Drop CasStore handle; reopen (cold-start path).
   235	    let bytes = {
   236	        let cas = CasStore::open(tmp.path()).expect("reopen post-restart");
   237	        cas.get(&cid_first).expect("get post-restart")
   238	    };
   239	    assert_eq!(bytes.as_slice(), payload);
   240	}
   241	```
   242	
   243	### 3.2 `git2_writer_returns_some_after_commit` (Q1 synthesis defensive test)
   244	
   245	```rust
   246	// tests/co1_7_extra/git2_writer_head_oid_defense.rs (NEW)
   247	#[test]
   248	fn git2_writer_returns_some_after_commit() {
   249	    let tmp = tempfile::TempDir::new().expect("tempdir");
   250	    let mut writer = Git2LedgerWriter::open(tmp.path()).expect("open");
   251	    let entry = canonical_test_entry(1);
   252	    writer.commit(&entry).expect("commit");
   253	    // Defensive against silent head_t stagnation per Gemini Q8 concern.
   254	    // If Git2LedgerWriter ever inherits the default-None impl by accident
   255	    // (refactor regression / forgotten override), this fails fast in CI.
   256	    assert!(
   257	        writer.head_commit_oid_hex().is_some(),
   258	        "Git2LedgerWriter MUST return Some after commit; default-None inheritance = constitutional anchor violation"
   259	    );
   260	}
   261	```
   262	
   263	Total: 2 tests.
   264	
   265	---
   266	
   267	## § 4 Out of scope (explicitly deferred)
   268	
   269	1. **D1 transition function bodies (7)** — moved to future CO1.7.5 atom; gated on CO P2.x substrate atoms (§ 0.1 table).
   270	2. **3 of 4 D4 tests** (`replay_full_transition_state_root`, `system_signature_verifies_via_canonical_message`, un-ignore `sequencer_serial_replay_byte_identity`) — all require D1 to actually commit; deferred with D1 to future CO1.7.5.
   271	3. **TransitionError 22-variant mapping table** — was over-claimed in prior bundled v1 (Codex Q-E); deferred with D1 to future CO1.7.5 spec.
   272	4. **RejectedAttemptSummary side-channel substantiation** — was overclaimed (Codex Q-E); deferred to future CO1.7.5 spec where it's actually relevant.
   273	5. **STATE_TRANSITION_SPEC v1.5 housekeeping issue filing** — committed to as a post-CO1.7-extra-PASS/PASS process item (§ 0.4); not gating implementation.
   274	6. **Legacy `src/ledger.rs` retirement** — CO1.1.5 atom; CO1.7-extra leaves the legacy WAL ledger fully running.
   275	7. **Materializer state_root computation** — CO1.8 (L5).
   276	
   277	---
   278	
   279	## § 5 Open questions (1 remains)
   280	
   281	| Q | Conservative resolution proposed | Audit input requested |
   282	|---|---|---|
   283	| **Q1' Sequencer Debug derive completeness** (NEW; surfaced by Codex Q-C) | `#[derive(Debug)]` on Sequencer struct; if blanket-derive fails on `Arc<RwLock<dyn LedgerWriter>>` field, fall back to manual impl with `f.debug_struct("Sequencer").finish_non_exhaustive()`. | Confirm: does `finish_non_exhaustive` leak any sensitive state, or is it safe for the Kernel-via-serde-skip path? Fallback: `PhantomData<()>` placeholder Debug? |
   284	
   285	(The original v1 Q1 — `head_commit_oid_hex` default impl — is now resolved per Q1 synthesis in § 1.2: default `None` + mandatory override + defensive test.)
   286	
   287	---
   288	
   289	## § 6 Audit gates (round structure)
   290	
   291	| Round | Codex | Gemini | Conservative | Action |
   292	|---|---|---|---|---|
   293	| 1 (on prior bundled v1) | CHALLENGE / High | CHALLENGE / High | **CHALLENGE** | Atom rescoped via Occam scope-split (this v1) + small fixes |
   294	| 2 (on this spec) | ⏳ pending | ⏳ pending | TBD | re-audit on CO1.7-extra v1; 1 round expected (small, focused atom) |
   295	| 3+ if needed | … | … | … | iterate to PASS/PASS |
   296	
   297	**Pre-implementation gate**: spec must reach PASS/PASS before any code in `src/state/sequencer.rs` D2 lines, `src/bus.rs` forwarder, `src/kernel.rs` field, or `src/bottom_white/ledger/transition_ledger.rs` trait method is written. Per CLAUDE.md "Audit Standard".
   298	
   299	---
   300	
   301	## § 7 Estimated scope
   302	
   303	- **Spec rounds**: 1 expected on CO1.7-extra (small atom; scope split addresses round-1 substantive findings; only fine-grained issues likely in round-2). Round-2 budget ~$5-10.
   304	- **Implementation scope** (post-PASS/PASS):
   305	  - D2 (head_t close + trait method + 2 impl overrides): ~30-50 LoC
   306	  - D3 (TuringBus forwarder + Kernel field + serde-skip + Sequencer Debug derive): ~40-60 LoC across 2 STEP_B-coupled files
   307	  - D4 (2 tests): ~80-120 LoC
   308	- **Total atom budget**: ~150-230 LoC. **Estimated calendar time**: 1-2 days. Implementation may ship as one commit `CO1.7-extra A1+A2+A3` or 3 sequential.
   309	
   310	---
   311	
   312	## § 8 Honest acknowledgements (v1)
   313	
   314	1. **Scope split is round-1-driven**, not voluntary. Prior bundled CO1.7.5 v1 spec was found by Codex Q-D/H/I to have heavyweight cross-layer substrate dependencies in D1. This v1 reverts CO1.7.5 to its CO1.7 § 13 original meaning (transition bodies; future) and creates CO1.7-extra (this atom) as a new bridge for the substrate-independent wiring.
   315	2. **`head_commit_oid_hex` is a NEW trait method** with mandatory-override design (Q1 synthesis: default `None` + every impl overrides + defensive test).
   316	3. **TuringBus is the actual struct name**; prior bundled v1 wrote "Bus" throughout (Codex Q-C catch). Fixed in § 2.1.
   317	4. **Kernel needs `serde(skip)` on the new Sequencer field** because Sequencer holds Arc-locked runtime state that isn't serializable Q_t data (Codex Q-C).
   318	5. **Combined STEP_B ceremony argument now rests on functional coupling** (each half is a compile-or-no-op-error without the other), not on `STEP_B_PROTOCOL.md` Phase 0 "minimum sufficient version" binding (which Codex Q-C correctly noted is advisory).
   319	6. **STATE_TRANSITION_SPEC v1.5 housekeeping issue filing is committed** as part of CO1.7-extra atom closure (§ 0.4), per Gemini MF1 active-reconciliation requirement.
   320	7. **Most of CO1.1.4-pre1 ABI lock is irrelevant to this atom** — D1 (the part that uses TypedTx + TransitionError + SignalKind) is out of scope. CO1.7-extra only touches `LedgerWriter` trait + Sequencer wiring; ABI lock untouched.
   321	8. **FC-trace requirements**: the new pub symbols introduced by CO1.7-extra implementation must carry doc-comment `/// TRACE_MATRIX <FC-id>: <role>` backlinks per CLAUDE.md "Alignment Standard". Set: `LedgerWriter::head_commit_oid_hex` (→ § 5 L4 sequencer post-commit head_t wiring); `Kernel.sequencer` field + `Kernel::with_sequencer` + `TuringBus::submit_typed_tx` (→ § 5.2.1 single-writer entry-point).
   322	
   323	---
   324	
   325	## § 9 Pre-audit smoke test plan
   326	
   327	Per memory `feedback_smoke_before_batch`. Smoke run before round-2 audit launch, at the v1.1 commit HEAD.
   328	
   329	| # | Claim | Smoke command | Pass criterion |
   330	|---|---|---|---|
   331	| S1 | `Git2LedgerWriter::head_commit_oid()` returns `Option<git2::Oid>` | `grep -A1 'pub fn head_commit_oid' src/bottom_white/ledger/transition_ledger.rs` | matches signature (line 674) |
   332	| S2 | Bus struct is named `TuringBus` | `grep -n 'pub struct TuringBus' src/bus.rs` | one hit at line 53 |
   333	| S3 | Kernel derives `Debug, Serialize, Deserialize` | `grep -B1 'pub struct Kernel' src/kernel.rs` | derives present at line 18 |
   334	| S4 | Sequencer struct exists | `grep -n 'pub struct Sequencer' src/state/sequencer.rs` | one hit |
   335	| S5 | CasStore exposes `put` + `get` (CO1.4 + CO1.4-extra) | `grep -n 'pub fn put\|pub fn get' src/bottom_white/cas/store.rs` | both present |
   336	| S6 | Wallet (`src/sdk/tools/wallet.rs`) untouched | `grep -c 'transition_ledger\|state::sequencer\|TypedTx' src/sdk/tools/wallet.rs` | 0 hits |
   337	| S7 | QState.head_t is `state::q_state::NodeId` (tuple struct) | `grep -B1 -A1 'pub head_t' src/state/q_state.rs` | type matches |
   338	| S8 | cargo baseline | `cargo check --workspace && cargo test --workspace --lib` | clean compile + 239 / 0 / 1 ignored |
   339	
   340	---
   341	
   342	**END v1 DRAFT body.**
   343	
   344	## Pre-audit smoke results (footer; populated 2026-04-29 pre-round-2)
   345	
   346	Smoke run at HEAD `f7fc19f` (CO1.7-extra v1 spec rewrite commit).
   347	
   348	| # | Claim | Result | Status |
   349	|---|---|---|---|
   350	| S1 | Git2LedgerWriter::head_commit_oid returns Option<git2::Oid> | `pub fn head_commit_oid(&self) -> Option<git2::Oid>` (transition_ledger.rs:674) | ✅ PASS |
   351	| S2 | Bus struct named TuringBus | `pub struct TuringBus` at bus.rs:53 | ✅ PASS |
   352	| S3 | Kernel derives Debug, Serialize, Deserialize | `#[derive(Debug, Serialize, Deserialize)]` precedes `pub struct Kernel` (kernel.rs:18) | ✅ PASS |
   353	| S4 | Sequencer struct exists | `pub struct Sequencer` at sequencer.rs:190 | ✅ PASS |
   354	| S5 | CasStore put + get exposed | `pub fn put` at store.rs:163; `pub fn get` at store.rs:199 | ✅ PASS |
   355	| S6 | wallet (sdk/tools/wallet.rs) untouched | 0 hits of `transition_ledger\|state::sequencer\|TypedTx` | ✅ PASS |
   356	| S7 | QState.head_t is state::q_state::NodeId tuple struct | `pub head_t: NodeId` (q_state.rs:311) — type matches | ✅ PASS |
   357	| S8 | cargo baseline | check pass + `239 passed; 0 failed; 1 ignored` (the ignored is `sequencer_serial_replay_byte_identity`, intentionally deferred to future CO1.7.5 atom) | ✅ PASS |
   358	
   359	**Smoke gate**: 8 / 8 PASS at HEAD `f7fc19f`. Spec ready for round-2 dual external audit.
   360	
   361	### Patch log (this session)
   362	
   363	**Scope rewrite (round-1 driven; this v1)**:
   364	- Q-D/H/I from Codex → prior bundled CO1.7.5 v1 was mis-scoped; D1 has cross-layer substrate dependencies. This atom rescoped to D2 + D3 + 1 substrate-independent D4 test only. D1 + 3 D4 tests + un-ignore migrated to future CO1.7.5 atom (gated on CO P2.x substrate).
   365	
   366	**Round-1 fixes baked into this v1**:
   367	- M3a (Codex Q-C): `Bus` → `TuringBus` everywhere (§ 2.1)
   368	- M3b (Codex Q-C): Kernel field gets `#[serde(skip)]`; Sequencer struct gets `#[derive(Debug)]` (§ 2.1)
   369	- M3c (Codex Q-C): Sequencer placement in Kernel justified by parallel to existing Tape/NodeId topology pattern + planned kernel.rs doc patch (§ 2.2)
   370	- M4 (Gemini MF1+MF3 + Codex Q-A): § 0.4 commits to filing STATE_TRANSITION_SPEC v1.5 housekeeping issue + asserts downstream-supersession authority principle
   371	- M5 (Gemini Q8 vs Codex Q-B synthesis): Q1 closed via default `None` + mandatory override + defensive `git2_writer_returns_some_after_commit` test (§ 1.2 + § 3.2)
   372	- Combined-ceremony argument rebased onto functional coupling (Codex Q-C correction; § 2.3)
   373	
   374	### Awaiting
   375	
   376	1. round-2 dual external audit on CO1.7-extra v1
   377	2. iterate to PASS/PASS (1 round expected; small focused atom)
   378	3. then CO1.7-extra-impl (D2 + D3 STEP_B + 2 tests)
   379	4. file STATE_TRANSITION_SPEC v1.5 housekeeping issue per § 0.4 commitment
   380	5. spec future CO1.7.5 (transition bodies; gated on CO P2.x substrate atoms)

exec
/bin/bash -lc "rg -n '\\bBus\\b|TuringBus|D1|transition function|transition bodies|3 of 4 D4|cas_payload|head_commit_oid_hex|finish_non_exhaustive|STATE_TRANSITION|downstream-spec|STEP_B|serde\\(skip\\)|derive\\(Debug\\)' handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
11:- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — referenced for K3 v1.2 supersession authority only; transition bodies are out of scope for this atom.
14:**Single sentence**: close the G-1 carry-forward `q.head_t = NodeId(commit_oid_hex)` after `Git2LedgerWriter.commit`, perform combined STEP_B ceremony adding a Sequencer entry-point on TuringBus + Kernel, and ship one substrate-independent CAS round-trip test — leaving transition function bodies + replay byte-identity to a future CO1.7.5 atom that depends on the Wave-2 substrate (CO P2.x family).
22:Round-1 dual external audit on the prior bundled CO1.7.5 v1 spec (`334111a`) returned CHALLENGE/CHALLENGE. The conservative-merged verdict (`CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md`) found that the v1 bundling crossed Anti-Oreo three-layer boundaries: D1 transition bodies require FC1 top-white predicate execution methods + FC2 middle-black state schemas that don't exist in shipped code (CO P2.x family substrate not yet shipped per `PROJECT_DECISION_MAP § 3.4`).
29:| **CO1.7.5 (future; restored to CO1.7 § 13 original meaning)** | D1 transition bodies (7) + 3 D4 tests + un-ignore `sequencer_serial_replay_byte_identity` | CO P2.1 / P2.2 / P2.3 / P2.5 / P2.6 / P2.7 / P2.9 + CO1.11 + (new) PredicateRegistry execution-methods atom | After substrate atoms reach individual PASS/PASS |
38:| CO1.7-impl A2 (commit `2461fe6`) | `Sequencer` 9-stage `apply_one` + `dispatch_transition` exhaustive match (variants stay `Err(NotYetImplemented)` post-CO1.7-extra; D1 transition bodies are out of scope) |
39:| CO1.4-extra (commit `b6b7574`) | CAS sidecar JSONL index persistence (substrate for the cas_payload_round_trip test) |
43:1. **D2** — `q.head_t = state::q_state::NodeId(commit_oid_hex)` after `writer.commit(&entry)` returns Ok; adds 1 trait method `LedgerWriter::head_commit_oid_hex` with mandatory-override design pattern (Q1 synthesis from round-1).
44:2. **D3** — Combined STEP_B ceremony adds `Option<Arc<Sequencer>>` field to `Kernel` + `submit_typed_tx` forwarder method on `TuringBus` (note: type is `TuringBus`, not `Bus`, per `src/bus.rs:53`). Sequencer instance lives in Kernel; TuringBus forwards via `self.kernel.sequencer`.
45:3. **D4-substrate-independent** — One conformance test `tests/cas_payload_round_trip` (`CasStore::put` → `get` round-trip with CID stability post-CO1.4-extra). Other 3 D4 tests (replay state-root + system-signature canonical-message + un-ignore byte-identity) move to future CO1.7.5 atom because they require D1 transition bodies to actually commit.
49:The two STATE_TRANSITION_SPEC § 3 supersessions previously declared in the prior CO1.7.5 v1 spec (NodeId head_t binding + SignalKind 4-variant minimization) **continue to apply** — but no longer in scope for CO1.7-extra (which doesn't contain transition bodies). They migrate intact to the future CO1.7.5 atom.
53:**Institutional debt acknowledged** (per Gemini MF1): as part of CO1.7-extra atom closure, ArchitectAI commits to filing a STATE_TRANSITION_SPEC v1.5 housekeeping issue (one paragraph noting the two supersessions from CO1.7 K3 v1.2 + CO1.1.4-pre1 with backlinks) — NOT a re-audit, just an annotation pass that prevents future readers from being confused by the historical drafting language. Tracked as part of the post-PASS/PASS landing checklist (§ 8 awaiting list).
72:if let Some(commit_oid_hex) = writer_w.head_commit_oid_hex() {
77:**NodeId disambiguation**: two `NodeId` types coexist — legacy `pub type NodeId = String` at `src/ledger.rs:13` (imported by TuringBus + Kernel for the legacy ledger event API) and new `pub struct NodeId(pub String)` at `src/state/q_state.rs:49`. `q.head_t` is typed as the new tuple-struct (`q_state.rs:311`); D2 constructs the new variant exclusively (legacy String alias is unused here).
79:**Atomicity** (per Codex Q-B finding, refined): under acquired `q_w` + `writer_w` write locks, after `writer_w.commit(&entry)?` returns `Ok`, the remaining operations are an `AtomicU64::store` (infallible), a plain `*q_w = q_next` move (infallible), and field assignments (infallible). The atomicity claim fully holds for writers whose `head_commit_oid_hex` returns `Some` (Git2LedgerWriter); writers returning `None` (InMemoryLedgerWriter) leave `q.head_t` unchanged from `q_next.head_t` (which equals `q.head_t` per CO1.7 K3 v1.2 — transition bodies don't mutate head_t even when they exist in CO1.7.5).
102:    fn head_commit_oid_hex(&self) -> Option<String> {
108:    fn head_commit_oid_hex(&self) -> Option<String> {
120:    fn head_commit_oid_hex(&self) -> Option<String> {
129:## § 2 D3 — Combined STEP_B ceremony for runtime entry-point
144:    #[serde(skip)]
163:#[derive(Debug)]  // NEW (CO1.7-extra D3); Q1' open audit input
167:`#[derive(Debug)]` may not propagate cleanly across `Arc<RwLock<dyn LedgerWriter>>` (the trait object in field position). If blanket derive fails at compile, manual impl uses `f.debug_struct("Sequencer").finish_non_exhaustive()` (sole open question Q1' below).
169:`src/bus.rs` (note: actual struct name is **`TuringBus`** at `src/bus.rs:53`, NOT `Bus`):
173:impl TuringBus {
196:Per `STEP_B_PROTOCOL.md` Phase 0, "minimum sufficient version" is technically **advisory** language asking auditors to favor the smallest change that works. CO1.7-extra rests the combined-ceremony argument on **functional coupling** (a stronger criterion):
198:- The TuringBus forwarder reads `self.kernel.sequencer`; without the Kernel field, the forwarder fails to compile.
199:- The Kernel field has no observable effect without an external caller; without the TuringBus forwarder, the field is dead code.
201:Each half is a no-op without the other. A/B byte-identity testing each half independently would test two non-functional changes; combining them into one A/B unit tests the actual minimum-functional change. This is a **stronger** application of STEP_B's spirit than the per-file alternative.
204:1. Branch A (`step-b-co1.7-extra-A`): edits BOTH `src/bus.rs` (TuringBus forwarder) AND `src/kernel.rs` (Sequencer field + with_sequencer constructor) per § 2.1. Also adds the `#[derive(Debug)]` on Sequencer in `src/state/sequencer.rs` (NOT STEP_B-restricted; landed alongside the ceremony for compile coherence).
214:### 3.1 `cas_payload_round_trip`
217:// tests/co1_7_extra/cas_payload_round_trip.rs (NEW)
226:fn cas_payload_round_trip_with_cid_stability_across_restart() {
257:        writer.head_commit_oid_hex().is_some(),
269:1. **D1 transition function bodies (7)** — moved to future CO1.7.5 atom; gated on CO P2.x substrate atoms (§ 0.1 table).
270:2. **3 of 4 D4 tests** (`replay_full_transition_state_root`, `system_signature_verifies_via_canonical_message`, un-ignore `sequencer_serial_replay_byte_identity`) — all require D1 to actually commit; deferred with D1 to future CO1.7.5.
271:3. **TransitionError 22-variant mapping table** — was over-claimed in prior bundled v1 (Codex Q-E); deferred with D1 to future CO1.7.5 spec.
273:5. **STATE_TRANSITION_SPEC v1.5 housekeeping issue filing** — committed to as a post-CO1.7-extra-PASS/PASS process item (§ 0.4); not gating implementation.
283:| **Q1' Sequencer Debug derive completeness** (NEW; surfaced by Codex Q-C) | `#[derive(Debug)]` on Sequencer struct; if blanket-derive fails on `Arc<RwLock<dyn LedgerWriter>>` field, fall back to manual impl with `f.debug_struct("Sequencer").finish_non_exhaustive()`. | Confirm: does `finish_non_exhaustive` leak any sensitive state, or is it safe for the Kernel-via-serde-skip path? Fallback: `PhantomData<()>` placeholder Debug? |
285:(The original v1 Q1 — `head_commit_oid_hex` default impl — is now resolved per Q1 synthesis in § 1.2: default `None` + mandatory override + defensive test.)
306:  - D3 (TuringBus forwarder + Kernel field + serde-skip + Sequencer Debug derive): ~40-60 LoC across 2 STEP_B-coupled files
314:1. **Scope split is round-1-driven**, not voluntary. Prior bundled CO1.7.5 v1 spec was found by Codex Q-D/H/I to have heavyweight cross-layer substrate dependencies in D1. This v1 reverts CO1.7.5 to its CO1.7 § 13 original meaning (transition bodies; future) and creates CO1.7-extra (this atom) as a new bridge for the substrate-independent wiring.
315:2. **`head_commit_oid_hex` is a NEW trait method** with mandatory-override design (Q1 synthesis: default `None` + every impl overrides + defensive test).
316:3. **TuringBus is the actual struct name**; prior bundled v1 wrote "Bus" throughout (Codex Q-C catch). Fixed in § 2.1.
317:4. **Kernel needs `serde(skip)` on the new Sequencer field** because Sequencer holds Arc-locked runtime state that isn't serializable Q_t data (Codex Q-C).
318:5. **Combined STEP_B ceremony argument now rests on functional coupling** (each half is a compile-or-no-op-error without the other), not on `STEP_B_PROTOCOL.md` Phase 0 "minimum sufficient version" binding (which Codex Q-C correctly noted is advisory).
319:6. **STATE_TRANSITION_SPEC v1.5 housekeeping issue filing is committed** as part of CO1.7-extra atom closure (§ 0.4), per Gemini MF1 active-reconciliation requirement.
320:7. **Most of CO1.1.4-pre1 ABI lock is irrelevant to this atom** — D1 (the part that uses TypedTx + TransitionError + SignalKind) is out of scope. CO1.7-extra only touches `LedgerWriter` trait + Sequencer wiring; ABI lock untouched.
321:8. **FC-trace requirements**: the new pub symbols introduced by CO1.7-extra implementation must carry doc-comment `/// TRACE_MATRIX <FC-id>: <role>` backlinks per CLAUDE.md "Alignment Standard". Set: `LedgerWriter::head_commit_oid_hex` (→ § 5 L4 sequencer post-commit head_t wiring); `Kernel.sequencer` field + `Kernel::with_sequencer` + `TuringBus::submit_typed_tx` (→ § 5.2.1 single-writer entry-point).
332:| S2 | Bus struct is named `TuringBus` | `grep -n 'pub struct TuringBus' src/bus.rs` | one hit at line 53 |
351:| S2 | Bus struct named TuringBus | `pub struct TuringBus` at bus.rs:53 | ✅ PASS |
364:- Q-D/H/I from Codex → prior bundled CO1.7.5 v1 was mis-scoped; D1 has cross-layer substrate dependencies. This atom rescoped to D2 + D3 + 1 substrate-independent D4 test only. D1 + 3 D4 tests + un-ignore migrated to future CO1.7.5 atom (gated on CO P2.x substrate).
367:- M3a (Codex Q-C): `Bus` → `TuringBus` everywhere (§ 2.1)
368:- M3b (Codex Q-C): Kernel field gets `#[serde(skip)]`; Sequencer struct gets `#[derive(Debug)]` (§ 2.1)
370:- M4 (Gemini MF1+MF3 + Codex Q-A): § 0.4 commits to filing STATE_TRANSITION_SPEC v1.5 housekeeping issue + asserts downstream-supersession authority principle
378:3. then CO1.7-extra-impl (D2 + D3 STEP_B + 2 tests)
379:4. file STATE_TRANSITION_SPEC v1.5 housekeeping issue per § 0.4 commitment
380:5. spec future CO1.7.5 (transition bodies; gated on CO P2.x substrate atoms)

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '1,430p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! L4 Sequencer + dispatch_transition (CO1.7-impl A2 + A3).
     2	//!
     3	//! Spec authority:
     4	//! - `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` § 3 (Sequencer
     5	//!   pseudocode, K1 dual-counter, K3 head_t deferred, C3 sign API)
     6	//! - `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` § 8
     7	//!   (dispatch_transition exhaustive enum match; K5 Slash dropped)
     8	//!
     9	//! Single-writer per (runtime_repo, run_id). Per spec § 5.2.1.
    10	//!
    11	//! **Stub state (this atom)**: every per-kind transition returns
    12	//! `TransitionError::NotYetImplemented`; CO1.7.5 (downstream atom) fills the
    13	//! bodies. The structural correctness of the apply path (snapshot → dispatch →
    14	//! CAS put → sign → root fold → commit → Q_t mutation) is locked by the
    15	//! impl + tests here; what's left is per-kind transition logic.
    16	//!
    17	//! /// TRACE_MATRIX § 5.2.1 + § 8 — L4 sequencer single-writer + dispatch.
    18	
    19	use std::sync::atomic::{AtomicU64, Ordering};
    20	use std::sync::{Arc, RwLock};
    21	
    22	use crate::bottom_white::cas::schema::ObjectType;
    23	use crate::bottom_white::cas::store::{CasError, CasStore};
    24	use crate::bottom_white::ledger::system_keypair::{
    25	    transition_ledger_emitter, Ed25519Keypair, KeypairError, SystemEpoch,
    26	};
    27	use crate::bottom_white::ledger::transition_ledger::{
    28	    append, canonical_encode, LedgerEntry, LedgerEntrySigningPayload, LedgerWriter,
    29	    LedgerWriterError,
    30	};
    31	use crate::bottom_white::tools::registry::ToolRegistry;
    32	use crate::state::q_state::QState;
    33	use crate::state::typed_tx::{SignalBundle, TransitionError, TypedTx};
    34	use crate::top_white::predicates::registry::PredicateRegistry;
    35	
    36	// ────────────────────────────────────────────────────────────────────────────
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
    65	// Submission types — K1 dual counter
    66	// ────────────────────────────────────────────────────────────────────────────
    67	
    68	/// Returned by `Sequencer::submit`. Carries `submit_id` (always assigned at
    69	/// submit time) but **NOT** `logical_t` — logical_t is only assigned post-accept
    70	/// per K1 (see spec § 3 + CO1.7 K1 closure).
    71	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
    72	pub struct SubmissionReceipt {
    73	    pub submit_id: u64,
    74	}
    75	
    76	#[derive(Debug)]
    77	pub enum SubmitError {
    78	    /// Bounded queue saturated (Q1/Q2 resolution: agent retries with backoff).
    79	    QueueFull,
    80	    /// Receiver dropped — sequencer no longer running.
    81	    QueueClosed,
    82	}
    83	
    84	impl std::fmt::Display for SubmitError {
    85	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    86	        match self {
    87	            Self::QueueFull => write!(f, "submission queue saturated"),
    88	            Self::QueueClosed => write!(f, "submission queue closed"),
    89	        }
    90	    }
    91	}
    92	impl std::error::Error for SubmitError {}
    93	
    94	/// Errors that can occur during `apply_one`. Spec § 3 implicitly assumes
    95	/// `Result<_, TransitionError>` but the actual `?`-propagated error chain
    96	/// crosses CAS, keypair, and ledger-writer boundaries — wrapper enum captures
    97	/// all of these explicitly. **Implementation note vs. spec**: spec § 3 line
    98	/// 307 writes the apply_one signature as `Result<LedgerEntry, TransitionError>`;
    99	/// this implementation widens to `Result<LedgerEntry, ApplyError>` to preserve
   100	/// distinct error provenance (TransitionError keeps its closed taxonomy +
   101	/// additive-only invariant per CO1.1.4-pre1 § 7.2).
   102	#[derive(Debug)]
   103	pub enum ApplyError {
   104	    /// Pure transition function rejected the tx.
   105	    Transition(TransitionError),
   106	    /// CAS payload put failed.
   107	    Cas(CasError),
   108	    /// System keypair sign failed.
   109	    Keypair(KeypairError),
   110	    /// Ledger writer commit failed.
   111	    LedgerCommit(LedgerWriterError),
   112	    /// Internal: canonical encoding of typed-tx payload failed (should never
   113	    /// happen for serde-derive types; surfaced for completeness).
   114	    PayloadEncode(String),
   115	    /// `q.read()` / `q.write()` lock poisoned by panicking thread.
   116	    QStateLockPoisoned,
   117	}
   118	
   119	impl std::fmt::Display for ApplyError {
   120	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   121	        match self {
   122	            Self::Transition(e) => write!(f, "transition rejected: {e}"),
   123	            Self::Cas(e) => write!(f, "cas put failed: {e}"),
   124	            Self::Keypair(e) => write!(f, "keypair sign failed: {e:?}"),
   125	            Self::LedgerCommit(e) => write!(f, "ledger commit failed: {e}"),
   126	            Self::PayloadEncode(s) => write!(f, "payload encode failed: {s}"),
   127	            Self::QStateLockPoisoned => write!(f, "q-state lock poisoned"),
   128	        }
   129	    }
   130	}
   131	impl std::error::Error for ApplyError {}
   132	
   133	impl From<TransitionError> for ApplyError {
   134	    fn from(e: TransitionError) -> Self {
   135	        Self::Transition(e)
   136	    }
   137	}
   138	impl From<CasError> for ApplyError {
   139	    fn from(e: CasError) -> Self {
   140	        Self::Cas(e)
   141	    }
   142	}
   143	impl From<KeypairError> for ApplyError {
   144	    fn from(e: KeypairError) -> Self {
   145	        Self::Keypair(e)
   146	    }
   147	}
   148	impl From<LedgerWriterError> for ApplyError {
   149	    fn from(e: LedgerWriterError) -> Self {
   150	        Self::LedgerCommit(e)
   151	    }
   152	}
   153	
   154	#[derive(Debug)]
   155	pub enum SequencerError {
   156	    /// `run()` was called when the receiver had already been consumed.
   157	    ReceiverAlreadyTaken,
   158	}
   159	
   160	impl std::fmt::Display for SequencerError {
   161	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   162	        match self {
   163	            Self::ReceiverAlreadyTaken => write!(f, "sequencer receiver already taken"),
   164	        }
   165	    }
   166	}
   167	impl std::error::Error for SequencerError {}
   168	
   169	// ────────────────────────────────────────────────────────────────────────────
   170	// Sequencer — single-writer per (runtime_repo, run_id)
   171	// ────────────────────────────────────────────────────────────────────────────
   172	
   173	/// TRACE_MATRIX § 5.2.1 — L4 sequencer; single-writer per (runtime_repo, run_id).
   174	///
   175	/// **K1 dual counter**: `next_submit_id` advances at every `submit()` (used to
   176	/// derive `SubmissionReceipt.submit_id`); `next_logical_t` advances ONLY at
   177	/// commit time (rejected submissions never get a logical_t — preserves
   178	/// `LedgerWriter`'s strict logical_t monotonicity invariant).
   179	///
   180	/// **K3 v1.2 (revised)**: Sequencer does NOT mutate `q.head_t` or
   181	/// `q.state_root_t` directly; the transition function returns the new
   182	/// `QState` and the sequencer accepts it as-is. `head_t` mutation defers to
   183	/// CO1.7.5+ wiring (when `Git2LedgerWriter::commit` provides commit_sha
   184	/// alongside Hash).
   185	///
   186	/// **C3 sign API**: signs through
   187	/// `transition_ledger_emitter::sign_ledger_entry(keypair, digest_bytes)` —
   188	/// the typed `CanonicalMessage::LedgerEntrySigning([u8;32])` extension closes
   189	/// the C3 round-2 audit point.
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
   206	    q: RwLock<QState>,
   207	}
   208	
   209	impl Sequencer {
   210	    /// Construct. Returns the `Sequencer` plus the receiver half of the
   211	    /// internal mpsc; pass the receiver to `run()` exactly once.
   212	    #[allow(clippy::too_many_arguments)]
   213	    pub fn new(
   214	        cas: Arc<RwLock<CasStore>>,
   215	        keypair: Arc<Ed25519Keypair>,
   216	        epoch: SystemEpoch,
   217	        ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
   218	        predicate_registry: Arc<PredicateRegistry>,
   219	        tool_registry: Arc<ToolRegistry>,
   220	        initial_q: QState,
   221	        queue_capacity: usize,
   222	    ) -> (Self, tokio::sync::mpsc::Receiver<TypedTx>) {
   223	        let (queue_tx, queue_rx) = tokio::sync::mpsc::channel(queue_capacity);
   224	        let seq = Self {
   225	            next_submit_id: AtomicU64::new(1),
   226	            next_logical_t: AtomicU64::new(0), // first accepted commit advances to 1
   227	            queue_tx,
   228	            cas,
   229	            keypair,
   230	            epoch,
   231	            ledger_writer,
   232	            predicate_registry,
   233	            tool_registry,
   234	            q: RwLock::new(initial_q),
   235	        };
   236	        (seq, queue_rx)
   237	    }
   238	
   239	    /// Submit a typed transition. Returns immediately with a receipt carrying
   240	    /// `submit_id` (NOT `logical_t`). Per Q2 (back-pressure resolution): on
   241	    /// queue saturation returns `Err(SubmitError::QueueFull)` and the agent is
   242	    /// expected to retry with deterministic exponential backoff.
   243	    pub async fn submit(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
   244	        let submit_id = self.next_submit_id.fetch_add(1, Ordering::SeqCst);
   245	        match self.queue_tx.try_send(tx) {
   246	            Ok(()) => Ok(SubmissionReceipt { submit_id }),
   247	            Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => Err(SubmitError::QueueFull),
   248	            Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => Err(SubmitError::QueueClosed),
   249	        }
   250	    }
   251	
   252	    /// Driver loop. Drains the queue and runs `apply_one` on each tx. Errors
   253	    /// from individual `apply_one` calls are logged and skipped (per-tx
   254	    /// rejection does NOT halt the sequencer). Returns when the queue is
   255	    /// closed and drained.
   256	    pub async fn run(
   257	        &self,
   258	        mut queue_rx: tokio::sync::mpsc::Receiver<TypedTx>,
   259	    ) -> Result<(), SequencerError> {
   260	        while let Some(tx) = queue_rx.recv().await {
   261	            // Stub state: dispatch returns NotYetImplemented; apply_one
   262	            // bubbles up. We log and continue per spec § 3 v1.2 ordering rule
   263	            // (rejection does not consume a logical_t — see K1).
   264	            if let Err(e) = self.apply_one(tx) {
   265	                log::debug!("sequencer apply_one rejected: {e}");
   266	            }
   267	        }
   268	        Ok(())
   269	    }
   270	
   271	    /// Per-tx critical section. Pure transition + CAS put + sign + commit +
   272	    /// Q_t mutation. See spec § 3 stages 1-9.
   273	    ///
   274	    /// **v1.1 C-2 closure (Codex bundle Q-B)**: `next_logical_t` advances
   275	    /// **only on commit success** — the original spec § 3 stage-4
   276	    /// `fetch_add(1)` happened BEFORE sign + writer.commit, so any infra
   277	    /// failure (sign / commit) left `next_logical_t` advanced past a
   278	    /// logical_t that was never written to the ledger. The next accepted
   279	    /// tx would then be assigned a logical_t the writer rejects forever
   280	    /// (writer enforces strict `len + 1`). Fixed by `load → use → store
   281	    /// after commit succeeds`. Single-writer per spec § 5.2.1 makes the
   282	    /// load+store atomic enough; if multi-writer ever lands the AtomicU64
   283	    /// can be upgraded to a `compare_exchange` reservation pattern.
   284	    pub(crate) fn apply_one(&self, tx: TypedTx) -> Result<LedgerEntry, ApplyError> {
   285	        // Stage 1: snapshot Q_t under read lock.
   286	        let q_snapshot = {
   287	            let g = self.q.read().map_err(|_| ApplyError::QStateLockPoisoned)?;
   288	            g.clone()
   289	        };
   290	
   291	        // Stage 2: dispatch (pure). On reject (incl. NotYetImplemented stub),
   292	        // EARLY RETURN. K1: no logical_t consumed.
   293	        let (q_next, _signals) = dispatch_transition(
   294	            &q_snapshot,
   295	            &tx,
   296	            &self.predicate_registry,
   297	            &self.tool_registry,
   298	        )?;
   299	
   300	        // v1.1 C-2: TENTATIVE logical_t (do NOT fetch_add yet).
   301	        let logical_t = self.next_logical_t.load(Ordering::SeqCst) + 1;
   302	
   303	        // Stage 3: put payload to CAS. DIV-5 5-param put signature.
   304	        let payload_bytes = canonical_encode(&tx)
   305	            .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
   306	        let payload_cid = {
   307	            let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
   308	            cas_w.put(
   309	                &payload_bytes,
   310	                ObjectType::ProposalPayload,
   311	                &format!("sequencer-epoch-{}", self.epoch.get()),
   312	                logical_t,
   313	                Some("TypedTx.v1".to_string()),
   314	            )?
   315	        };
   316	
   317	        // Stage 5: build LedgerEntrySigningPayload (v1.1 — stage 4 fetch_add
   318	        // moved to AFTER stage 9 commit success).
   319	        let signing_payload = LedgerEntrySigningPayload {
   320	            logical_t,
   321	            parent_state_root: q_snapshot.state_root_t,
   322	            parent_ledger_root: q_snapshot.ledger_root_t,
   323	            tx_kind: tx.tx_kind(),
   324	            tx_payload_cid: payload_cid,
   325	            resulting_state_root: q_next.state_root_t,
   326	            timestamp_logical: logical_t,
   327	            epoch: self.epoch,
   328	            extensions: std::collections::BTreeMap::new(),
   329	        };
   330	
   331	        // Stage 6: C3 — sign via typed CanonicalMessage::LedgerEntrySigning.
   332	        let signing_digest = signing_payload.canonical_digest();
   333	        let system_signature = transition_ledger_emitter::sign_ledger_entry(
   334	            &self.keypair,
   335	            signing_digest.0,
   336	        )?;
   337	
   338	        // Stage 7: pure ledger-root fold (deterministic).
   339	        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);
   340	
   341	        // Stage 8: build LedgerEntry (the stored record).
   342	        let entry = LedgerEntry {
   343	            logical_t: signing_payload.logical_t,
   344	            parent_state_root: signing_payload.parent_state_root,
   345	            parent_ledger_root: signing_payload.parent_ledger_root,
   346	            tx_kind: signing_payload.tx_kind,
   347	            tx_payload_cid: signing_payload.tx_payload_cid,
   348	            resulting_state_root: signing_payload.resulting_state_root,
   349	            resulting_ledger_root,
   350	            timestamp_logical: signing_payload.timestamp_logical,
   351	            epoch: signing_payload.epoch,
   352	            extensions: signing_payload.extensions,
   353	            system_signature,
   354	        };
   355	
   356	        // Stage 9: commit + mutate Q_t under write lock.
   357	        // v1.1 C-2: next_logical_t.store(logical_t) HAPPENS ONLY AFTER
   358	        // writer.commit succeeds — preserves K1 under infra failure.
   359	        // K3 v1.2 (revised): we set q.ledger_root_t but NOT q.head_t (head_t
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
   373	        }
   374	
   375	        Ok(entry)
   376	    }
   377	
   378	    /// Read-only accessor (testing + CO1.7.5+ wiring).
   379	    pub fn q_snapshot(&self) -> Result<QState, ApplyError> {
   380	        self.q
   381	            .read()
   382	            .map(|g| g.clone())
   383	            .map_err(|_| ApplyError::QStateLockPoisoned)
   384	    }
   385	
   386	    pub fn next_submit_id_peek(&self) -> u64 {
   387	        self.next_submit_id.load(Ordering::SeqCst)
   388	    }
   389	
   390	    pub fn next_logical_t_peek(&self) -> u64 {
   391	        self.next_logical_t.load(Ordering::SeqCst)
   392	    }
   393	}
   394	
   395	// ────────────────────────────────────────────────────────────────────────────
   396	// Tests — stub-mode coverage (CO1.7.5 fills real-transition tests)
   397	// ────────────────────────────────────────────────────────────────────────────
   398	
   399	#[cfg(test)]
   400	mod tests {
   401	    use super::*;
   402	    use crate::bottom_white::ledger::transition_ledger::InMemoryLedgerWriter;
   403	    use crate::state::typed_tx::{
   404	        AgentSignature, BoolWithProof, ChallengeTx, ClaimId, FinalizeRewardTx, PredicateId,
   405	        PredicateResultsBundle, ReadKey, ReuseTx, RunId, RunOutcome, SafetyOrCreation,
   406	        TaskExpireTx, TaskId, TerminalSummaryTx, ToolId, VerifyTx, VerifyVerdict, WorkTx,
   407	        WriteKey,
   408	    };
   409	    use crate::state::q_state::{AgentId, TxId};
   410	    use crate::economy::money::{MicroCoin, StakeMicroCoin};
   411	    use crate::bottom_white::cas::schema::Cid;
   412	    use crate::bottom_white::ledger::system_keypair::SystemSignature;
   413	    use std::collections::{BTreeMap, BTreeSet};
   414	    use tempfile::TempDir;
   415	
   416	    fn fresh_sequencer() -> (
   417	        TempDir,
   418	        Sequencer,
   419	        tokio::sync::mpsc::Receiver<TypedTx>,
   420	    ) {
   421	        let tmp = TempDir::new().expect("tempdir");
   422	        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas open")));
   423	        let keypair = Arc::new(
   424	            Ed25519Keypair::generate_with_secure_entropy().expect("keypair gen"),
   425	        );
   426	        let epoch = SystemEpoch::new(1);
   427	        let writer: Arc<RwLock<dyn LedgerWriter>> =
   428	            Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
   429	        let preds = Arc::new(PredicateRegistry::new());
   430	        let tools = Arc::new(ToolRegistry::new());

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '190,760p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   190	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
   191	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
   192	    fn len(&self) -> u64;
   193	}
   194	
   195	#[derive(Debug)]
   196	pub enum LedgerWriterError {
   197	    LogicalTGap { expected: u64, got: u64 },
   198	    NotFound { logical_t: u64 },
   199	    BackendCorruption(String),
   200	}
   201	
   202	impl std::fmt::Display for LedgerWriterError {
   203	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   204	        match self {
   205	            Self::LogicalTGap { expected, got } => {
   206	                write!(f, "logical_t gap: expected {expected}, got {got}")
   207	            }
   208	            Self::NotFound { logical_t } => write!(f, "no entry at logical_t={logical_t}"),
   209	            Self::BackendCorruption(msg) => write!(f, "backend corruption: {msg}"),
   210	        }
   211	    }
   212	}
   213	impl std::error::Error for LedgerWriterError {}
   214	
   215	/// In-memory test/skeleton writer; Vec backing strict logical_t enforced at commit.
   216	#[derive(Debug, Default)]
   217	pub struct InMemoryLedgerWriter {
   218	    entries: Vec<LedgerEntry>,
   219	}
   220	
   221	impl InMemoryLedgerWriter {
   222	    pub fn new() -> Self {
   223	        Self::default()
   224	    }
   225	}
   226	
   227	impl LedgerWriter for InMemoryLedgerWriter {
   228	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
   229	        let expected = (self.entries.len() as u64) + 1;
   230	        if entry.logical_t != expected {
   231	            return Err(LedgerWriterError::LogicalTGap {
   232	                expected,
   233	                got: entry.logical_t,
   234	            });
   235	        }
   236	        let root = entry.resulting_ledger_root;
   237	        self.entries.push(entry.clone());
   238	        Ok(root)
   239	    }
   240	
   241	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
   242	        if logical_t == 0 || logical_t > self.entries.len() as u64 {
   243	            return Err(LedgerWriterError::NotFound { logical_t });
   244	        }
   245	        Ok(self.entries[(logical_t - 1) as usize].clone())
   246	    }
   247	
   248	    fn len(&self) -> u64 {
   249	        self.entries.len() as u64
   250	    }
   251	}
   252	
   253	// ────────────────────────────────────────────────────────────────────────────
   254	// § 4 replay — TWO-MODE per C1
   255	// ────────────────────────────────────────────────────────────────────────────
   256	
   257	/// **C1 NEW**: replay mode discriminator.
   258	/// - `ChainOnly`: skeleton-stage; chain integrity only (parent_state_root +
   259	///   parent_ledger_root + ledger_root chain). NOT the I-DETHASH witness.
   260	/// - `FullTransition`: CO1.7.5+ stage; verifies signatures + re-fetches payloads
   261	///   from CAS + re-runs pure transitions + asserts state_root match. THE
   262	///   I-DETHASH witness; requires CO1.4-extra (CAS index persistence).
   263	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
   264	pub enum ReplayMode {
   265	    ChainOnly,
   266	    FullTransition,
   267	}
   268	
   269	#[derive(Debug)]
   270	pub enum ReplayError {
   271	    LogicalTGap { at: usize, expected: u64, got: u64 },
   272	    ParentStateMismatch { at: usize },
   273	    ParentLedgerMismatch { at: usize }, // K2 NEW
   274	    LedgerRootMismatch { at: usize },
   275	    // FullTransition-mode-only (CO1.7.5+):
   276	    BadSignature { at: usize },
   277	    CasMissing { at: usize },
   278	    StateRootMismatch { at: usize },
   279	    /// CO1.7-impl A4: dispatch_transition rejected the re-run. In stub state
   280	    /// (CO1.7.5 not yet shipped), this fires on every replay step with
   281	    /// `inner = NotYetImplemented`.
   282	    Transition {
   283	        at: usize,
   284	        inner: crate::state::typed_tx::TransitionError,
   285	    },
   286	    /// CO1.7-impl A4 v1.1 (Codex bundle Q-J / C-3): the canonical-decoded
   287	    /// `TypedTx` variant disagrees with the entry's `tx_kind` discriminator.
   288	    /// Signed envelope claims one kind; CAS payload is another.
   289	    TxKindMismatch {
   290	        at: usize,
   291	        envelope_kind: TxKind,
   292	        decoded_kind: TxKind,
   293	    },
   294	    /// CO1.7-impl A4 v1.1 (Codex bundle Q-K secondary): payload bytes
   295	    /// retrieved from CAS but `canonical_decode` failed (corruption /
   296	    /// non-canonical bytes). Distinct from `CasMissing` (lookup failure).
   297	    PayloadDecode {
   298	        at: usize,
   299	        reason: String,
   300	    },
   301	}
   302	
   303	impl std::fmt::Display for ReplayError {
   304	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   305	        match self {
   306	            Self::LogicalTGap { at, expected, got } => {
   307	                write!(f, "logical_t gap at index {at}: expected {expected}, got {got}")
   308	            }
   309	            Self::ParentStateMismatch { at } => write!(f, "parent_state_root mismatch at index {at}"),
   310	            Self::ParentLedgerMismatch { at } => write!(f, "parent_ledger_root mismatch at index {at}"),
   311	            Self::LedgerRootMismatch { at } => write!(f, "ledger_root mismatch at index {at}"),
   312	            Self::BadSignature { at } => write!(f, "system_signature verify failed at index {at}"),
   313	            Self::CasMissing { at } => write!(f, "CAS payload not retrievable at index {at}"),
   314	            Self::StateRootMismatch { at } => write!(f, "resulting_state_root divergence at index {at}"),
   315	            Self::Transition { at, inner } => write!(f, "dispatch_transition rejected at index {at}: {inner}"),
   316	            Self::TxKindMismatch { at, envelope_kind, decoded_kind } => write!(
   317	                f,
   318	                "tx_kind mismatch at index {at}: envelope claims {envelope_kind:?} but CAS payload decoded as {decoded_kind:?}"
   319	            ),
   320	            Self::PayloadDecode { at, reason } => write!(f, "payload canonical_decode failed at index {at}: {reason}"),
   321	        }
   322	    }
   323	}
   324	impl std::error::Error for ReplayError {}
   325	
   326	// ────────────────────────────────────────────────────────────────────────────
   327	// CO1.7-impl A4: LedgerCasView trait + replay_full_transition
   328	// ────────────────────────────────────────────────────────────────────────────
   329	
   330	/// CO1.7 spec § 4 + DIV-4 closure: narrow read-only CAS trait that replay
   331	/// needs. Decouples `replay_full_transition` from full `CasStore` (the
   332	/// production impl). Anything that can hand back the bytes for a `Cid`
   333	/// satisfies this — testing can mock it; cold-replay uses CasStore directly.
   334	pub trait LedgerCasView {
   335	    fn get_typed_payload(
   336	        &self,
   337	        cid: &crate::bottom_white::cas::schema::Cid,
   338	    ) -> Result<Vec<u8>, ReplayError>;
   339	}
   340	
   341	impl LedgerCasView for crate::bottom_white::cas::store::CasStore {
   342	    fn get_typed_payload(
   343	        &self,
   344	        cid: &crate::bottom_white::cas::schema::Cid,
   345	    ) -> Result<Vec<u8>, ReplayError> {
   346	        self.get(cid).map_err(|_| ReplayError::CasMissing { at: 0 })
   347	    }
   348	}
   349	
   350	/// CO1.7-impl A4 — full-mode replay (THE I-DETHASH witness).
   351	///
   352	/// Validates **every** stage spec § 4 + § 6 promises:
   353	/// 1. logical_t monotonicity
   354	/// 2. parent_state_root chain
   355	/// 3. parent_ledger_root chain (K2 transplant defense)
   356	/// 4. system_signature verifies via CanonicalMessage::LedgerEntrySigning + pinned pubkeys
   357	/// 5. CAS lookup of tx_payload_cid succeeds (CO1.4-extra cold-replay capability)
   358	/// 6. canonical_decode of payload bytes → TypedTx
   359	/// 6.5 (v1.1 C-3): decoded_typed_tx.tx_kind() MUST equal entry.tx_kind
   360	/// 7. dispatch_transition re-run produces (q_next, _signals)
   361	/// 8. q_next.state_root_t matches entry.resulting_state_root
   362	/// 9. resulting_ledger_root recomputed via append() matches stored
   363	///
   364	/// **v1.1 C-1 closure**: takes a full `genesis: &QState` (was `genesis_state_root`
   365	/// + `genesis_ledger_root` only). Caller provides the complete genesis state
   366	/// so dispatch_transition can read budget / registries / balances / task markets
   367	/// — fabricating `QState::genesis()` was dropping these fields.
   368	///
   369	/// **Stub-state caveat (CO1.7.5 unblocks)**: while `dispatch_transition`
   370	/// returns `NotYetImplemented` for every variant, replay errors at stage 7
   371	/// for any non-empty chain. Conformance tests exercising stages 1-6.5
   372	/// independently are `#[test]`-runnable now; full state_root reconstruction
   373	/// gates on CO1.7.5.
   374	pub fn replay_full_transition(
   375	    genesis: &crate::state::q_state::QState,
   376	    entries: &[LedgerEntry],
   377	    cas: &dyn LedgerCasView,
   378	    pinned_pubkeys: &crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys,
   379	    predicate_registry: &crate::top_white::predicates::registry::PredicateRegistry,
   380	    tool_registry: &crate::bottom_white::tools::registry::ToolRegistry,
   381	) -> Result<crate::state::q_state::QState, ReplayError> {
   382	    use crate::bottom_white::ledger::system_keypair::{
   383	        verify_system_signature, CanonicalMessage,
   384	    };
   385	    use crate::state::sequencer::dispatch_transition;
   386	
   387	    let mut q = genesis.clone();
   388	
   389	    for (i, entry) in entries.iter().enumerate() {
   390	        // Stage 1
   391	        let expected_logical_t = (i as u64) + 1;
   392	        if entry.logical_t != expected_logical_t {
   393	            return Err(ReplayError::LogicalTGap {
   394	                at: i,
   395	                expected: expected_logical_t,
   396	                got: entry.logical_t,
   397	            });
   398	        }
   399	        // Stage 2
   400	        if entry.parent_state_root != q.state_root_t {
   401	            return Err(ReplayError::ParentStateMismatch { at: i });
   402	        }
   403	        // Stage 3
   404	        if entry.parent_ledger_root != q.ledger_root_t {
   405	            return Err(ReplayError::ParentLedgerMismatch { at: i });
   406	        }
   407	
   408	        // Stage 4: system_signature verify (FullTransition mode only).
   409	        let signing_payload = entry.to_signing_payload();
   410	        let signing_digest = signing_payload.canonical_digest();
   411	        let canonical_msg = CanonicalMessage::LedgerEntrySigning(signing_digest.0);
   412	        if !verify_system_signature(
   413	            &entry.system_signature,
   414	            &canonical_msg,
   415	            entry.epoch,
   416	            pinned_pubkeys,
   417	        ) {
   418	            return Err(ReplayError::BadSignature { at: i });
   419	        }
   420	
   421	        // Stage 5: CAS lookup.
   422	        let payload_bytes = cas
   423	            .get_typed_payload(&entry.tx_payload_cid)
   424	            .map_err(|_| ReplayError::CasMissing { at: i })?;
   425	
   426	        // Stage 6: canonical_decode → TypedTx (v1.1 C-3-secondary: distinct
   427	        // error from CasMissing).
   428	        let typed_tx: crate::state::typed_tx::TypedTx =
   429	            canonical_decode(&payload_bytes).map_err(|e| ReplayError::PayloadDecode {
   430	                at: i,
   431	                reason: e.to_string(),
   432	            })?;
   433	
   434	        // Stage 6.5 (v1.1 C-3): tx_kind envelope vs decoded payload kind MUST match.
   435	        // Otherwise a signed envelope claiming `Work` could ride a CAS payload
   436	        // that decodes as `Verify` — sequencer would have written that
   437	        // mismatch but replay would have silently accepted it pre-v1.1.
   438	        let decoded_kind = typed_tx.tx_kind();
   439	        if decoded_kind != entry.tx_kind {
   440	            return Err(ReplayError::TxKindMismatch {
   441	                at: i,
   442	                envelope_kind: entry.tx_kind,
   443	                decoded_kind,
   444	            });
   445	        }
   446	
   447	        // Stage 7: re-run pure dispatch_transition.
   448	        let (q_next, _signals) =
   449	            dispatch_transition(&q, &typed_tx, predicate_registry, tool_registry)
   450	                .map_err(|inner| ReplayError::Transition { at: i, inner })?;
   451	
   452	        // Stage 8: state_root match.
   453	        if q_next.state_root_t != entry.resulting_state_root {
   454	            return Err(ReplayError::StateRootMismatch { at: i });
   455	        }
   456	
   457	        // Stage 9: ledger_root match (recompute via append).
   458	        let recomputed_ledger_root = append(&q.ledger_root_t, &signing_digest);
   459	        if recomputed_ledger_root != entry.resulting_ledger_root {
   460	            return Err(ReplayError::LedgerRootMismatch { at: i });
   461	        }
   462	
   463	        // Advance.
   464	        q = q_next;
   465	        q.ledger_root_t = entry.resulting_ledger_root;
   466	    }
   467	
   468	    Ok(q)
   469	}
   470	
   471	/// Skeleton-stage entry point (v1.1).
   472	///
   473	/// Validates:
   474	/// 1. logical_t monotonicity (no gaps, no duplicates)
   475	/// 2. parent_state_root chain
   476	/// 3. parent_ledger_root chain (K2 transplant defense)
   477	/// 4. resulting_ledger_root recomputed via append(prev_ledger_root, signing_digest)
   478	///
   479	/// Does NOT verify:
   480	/// - system_signature (CO1.7.5+: requires CanonicalMessage extension wired through keypair)
   481	/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
   482	///
   483	/// Returns final (state_root, ledger_root) on success.
   484	pub fn replay_chain_integrity(
   485	    genesis_state_root: Hash,
   486	    genesis_ledger_root: Hash,
   487	    entries: &[LedgerEntry],
   488	) -> Result<(Hash, Hash), ReplayError> {
   489	    let mut prev_state_root = genesis_state_root;
   490	    let mut prev_ledger_root = genesis_ledger_root;
   491	
   492	    for (i, entry) in entries.iter().enumerate() {
   493	        let expected_logical_t = (i as u64) + 1;
   494	        if entry.logical_t != expected_logical_t {
   495	            return Err(ReplayError::LogicalTGap {
   496	                at: i,
   497	                expected: expected_logical_t,
   498	                got: entry.logical_t,
   499	            });
   500	        }
   501	        if entry.parent_state_root != prev_state_root {
   502	            return Err(ReplayError::ParentStateMismatch { at: i });
   503	        }
   504	        // K2 NEW: parent_ledger_root chain check
   505	        if entry.parent_ledger_root != prev_ledger_root {
   506	            return Err(ReplayError::ParentLedgerMismatch { at: i });
   507	        }
   508	        let signing_digest = entry.to_signing_payload().canonical_digest();
   509	        let recomputed = append(&prev_ledger_root, &signing_digest);
   510	        if recomputed != entry.resulting_ledger_root {
   511	            return Err(ReplayError::LedgerRootMismatch { at: i });
   512	        }
   513	        prev_state_root = entry.resulting_state_root;
   514	        prev_ledger_root = entry.resulting_ledger_root;
   515	    }
   516	
   517	    Ok((prev_state_root, prev_ledger_root))
   518	}
   519	
   520	// ────────────────────────────────────────────────────────────────────────────
   521	// § 2.5 Canonical serialization (bincode v2; STATE_TRANSITION_SPEC § 2.5)
   522	// ────────────────────────────────────────────────────────────────────────────
   523	
   524	/// `bincode::config` used for the canonical `LedgerEntry` wire format.
   525	///
   526	/// **Frozen choices** (per STATE_TRANSITION_SPEC § 2.5):
   527	/// - **Big-endian** byte order (network order; deterministic across platforms).
   528	/// - **Fixed-int encoding** (no varint; fixed-width for byte-stable round-trip).
   529	/// - **`BTreeMap` keys**: bincode iterates the map in serde-supplied order; we
   530	///   only ever encode `BTreeMap` (sorted by construction) so key order is lex.
   531	/// - **No padding, no implicit alignment.**
   532	fn bincode_canonical_config() -> impl bincode::config::Config {
   533	    bincode::config::standard()
   534	        .with_big_endian()
   535	        .with_fixed_int_encoding()
   536	}
   537	
   538	/// Canonical encode any serde-Serialize value to bytes (CO1.7 wire format).
   539	/// Used by `Git2LedgerWriter` for commit-message bodies and by future callers
   540	/// needing byte-stable signatures over typed payloads.
   541	pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalCodecError> {
   542	    bincode::serde::encode_to_vec(value, bincode_canonical_config())
   543	        .map_err(|e| CanonicalCodecError::Encode(e.to_string()))
   544	}
   545	
   546	/// Canonical decode the inverse of `canonical_encode`. Returns the value plus
   547	/// the number of bytes consumed (entire input must be consumed for a clean decode).
   548	pub fn canonical_decode<T: serde::de::DeserializeOwned>(
   549	    bytes: &[u8],
   550	) -> Result<T, CanonicalCodecError> {
   551	    let (value, consumed) =
   552	        bincode::serde::decode_from_slice::<T, _>(bytes, bincode_canonical_config())
   553	            .map_err(|e| CanonicalCodecError::Decode(e.to_string()))?;
   554	    if consumed != bytes.len() {
   555	        return Err(CanonicalCodecError::TrailingBytes {
   556	            consumed,
   557	            total: bytes.len(),
   558	        });
   559	    }
   560	    Ok(value)
   561	}
   562	
   563	#[derive(Debug)]
   564	pub enum CanonicalCodecError {
   565	    Encode(String),
   566	    Decode(String),
   567	    TrailingBytes { consumed: usize, total: usize },
   568	}
   569	
   570	impl std::fmt::Display for CanonicalCodecError {
   571	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   572	        match self {
   573	            Self::Encode(s) => write!(f, "canonical encode failed: {s}"),
   574	            Self::Decode(s) => write!(f, "canonical decode failed: {s}"),
   575	            Self::TrailingBytes { consumed, total } => {
   576	                write!(f, "trailing bytes after decode: consumed {consumed} of {total}")
   577	            }
   578	        }
   579	    }
   580	}
   581	impl std::error::Error for CanonicalCodecError {}
   582	
   583	// ────────────────────────────────────────────────────────────────────────────
   584	// § 5 Git2LedgerWriter — git2-rs commit chain on `refs/transitions/main`
   585	// ────────────────────────────────────────────────────────────────────────────
   586	
   587	/// Spec § 5 production storage backend.
   588	///
   589	/// **Mapping**:
   590	/// - One `LedgerEntry` = one git commit on `refs/transitions/main`.
   591	/// - **Commit tree** = three named blobs:
   592	///     - `payload_cid`     = entry.tx_payload_cid.0 (32 bytes)
   593	///     - `signature`       = entry.system_signature.as_bytes() (64 bytes)
   594	///     - `entry_canonical` = bincode v2 BE + fixed-int encoding of the full
   595	///       `LedgerEntry` (deterministic, byte-stable; this blob IS the
   596	///       canonical record — `read_at` decodes it directly).
   597	/// - **Commit message** = human-readable `"transition logical_t=<N>\n"` (the
   598	///   canonical record lives in the tree blob, not the message — git
   599	///   normalizes message bytes in ways that break round-trip).
   600	/// - **Parent**: `head_t-1` commit (or none at genesis).
   601	/// - **Author/committer identity**: fixed `("turingosv4 sequencer", "system@turingos")`
   602	///   with `time = (logical_t as i64, 0)` to keep commit OIDs deterministic. NO
   603	///   wall-clock leakage (`I-NOENV` + `I-LOGTIME`).
   604	///
   605	/// **K3 (revised v1.2)**: this writer surfaces `commit_oid` for callers that
   606	/// need it (CO1.7.5+ `head_t` wiring), but the `LedgerWriter::commit` trait
   607	/// returns only `Hash` (entry.resulting_ledger_root). Callers requesting the
   608	/// commit OID use [`Git2LedgerWriter::head_commit_oid`] post-commit.
   609	pub struct Git2LedgerWriter {
   610	    repo_path: PathBuf,
   611	    /// Last commit OID on `refs/transitions/main`; `None` at empty-chain genesis.
   612	    head_oid: Option<git2::Oid>,
   613	    /// Number of entries committed = highest assigned `logical_t` (0 at genesis).
   614	    len: u64,
   615	}
   616	
   617	const TRANSITIONS_REF: &str = "refs/transitions/main";
   618	const TREE_BLOB_PAYLOAD_CID: &str = "payload_cid";
   619	const TREE_BLOB_SIGNATURE: &str = "signature";
   620	const TREE_BLOB_ENTRY_CANONICAL: &str = "entry_canonical";
   621	
   622	impl Git2LedgerWriter {
   623	    /// Open or initialize a `Git2LedgerWriter` rooted at `repo_path`.
   624	    /// Creates the underlying git repo if it doesn't exist; resolves the
   625	    /// existing `refs/transitions/main` if present and seeds `head_oid` + `len`.
   626	    pub fn open(repo_path: &Path) -> Result<Self, LedgerWriterError> {
   627	        let repo_path = repo_path.to_path_buf();
   628	        let repo = match Repository::open(&repo_path) {
   629	            Ok(r) => r,
   630	            Err(_) => Repository::init(&repo_path).map_err(|e| {
   631	                LedgerWriterError::BackendCorruption(format!("repo init: {e}"))
   632	            })?,
   633	        };
   634	
   635	        // Resolve refs/transitions/main if it exists.
   636	        let (head_oid, len) = match repo.find_reference(TRANSITIONS_REF) {
   637	            Ok(reference) => {
   638	                let oid = reference
   639	                    .target()
   640	                    .ok_or_else(|| {
   641	                        LedgerWriterError::BackendCorruption(format!(
   642	                            "{TRANSITIONS_REF} has no direct target"
   643	                        ))
   644	                    })?;
   645	                // Walk parents to count chain length.
   646	                let mut n: u64 = 0;
   647	                let mut cursor = Some(oid);
   648	                while let Some(c) = cursor {
   649	                    n += 1;
   650	                    let commit = repo.find_commit(c).map_err(|e| {
   651	                        LedgerWriterError::BackendCorruption(format!("walk parent: {e}"))
   652	                    })?;
   653	                    cursor = commit.parent(0).ok().map(|p| p.id());
   654	                }
   655	                (Some(oid), n)
   656	            }
   657	            Err(_) => (None, 0),
   658	        };
   659	
   660	        Ok(Self {
   661	            repo_path,
   662	            head_oid,
   663	            len,
   664	        })
   665	    }
   666	
   667	    fn open_repo(&self) -> Result<Repository, LedgerWriterError> {
   668	        Repository::open(&self.repo_path)
   669	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("repo open: {e}")))
   670	    }
   671	
   672	    /// Commit OID of the most recent appended entry (None if chain is empty).
   673	    /// CO1.7.5+ `head_t` wiring uses this to surface commit_sha alongside Hash.
   674	    pub fn head_commit_oid(&self) -> Option<git2::Oid> {
   675	        self.head_oid
   676	    }
   677	
   678	    /// Read raw canonical-encoded `LedgerEntry` bytes (the `entry_canonical`
   679	    /// tree blob) for the entry at `logical_t`. `logical_t` is 1-indexed.
   680	    fn read_canonical_bytes(&self, logical_t: u64) -> Result<Vec<u8>, LedgerWriterError> {
   681	        if logical_t == 0 || logical_t > self.len {
   682	            return Err(LedgerWriterError::NotFound { logical_t });
   683	        }
   684	        let repo = self.open_repo()?;
   685	        // Walk back (len - logical_t) parents from head.
   686	        let mut cursor = self.head_oid.ok_or(LedgerWriterError::NotFound { logical_t })?;
   687	        let mut steps_back = self.len - logical_t;
   688	        while steps_back > 0 {
   689	            let commit = repo.find_commit(cursor).map_err(|e| {
   690	                LedgerWriterError::BackendCorruption(format!("find_commit: {e}"))
   691	            })?;
   692	            cursor = commit
   693	                .parent(0)
   694	                .map_err(|e| LedgerWriterError::BackendCorruption(format!("parent: {e}")))?
   695	                .id();
   696	            steps_back -= 1;
   697	        }
   698	        let commit = repo
   699	            .find_commit(cursor)
   700	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_commit: {e}")))?;
   701	        let tree = commit
   702	            .tree()
   703	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree: {e}")))?;
   704	        let entry_obj = tree
   705	            .get_name(TREE_BLOB_ENTRY_CANONICAL)
   706	            .ok_or_else(|| {
   707	                LedgerWriterError::BackendCorruption(format!(
   708	                    "missing {TREE_BLOB_ENTRY_CANONICAL} blob at logical_t={logical_t}"
   709	                ))
   710	            })?;
   711	        let blob = repo
   712	            .find_blob(entry_obj.id())
   713	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_blob: {e}")))?;
   714	        Ok(blob.content().to_vec())
   715	    }
   716	}
   717	
   718	impl LedgerWriter for Git2LedgerWriter {
   719	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
   720	        let expected = self.len + 1;
   721	        if entry.logical_t != expected {
   722	            return Err(LedgerWriterError::LogicalTGap {
   723	                expected,
   724	                got: entry.logical_t,
   725	            });
   726	        }
   727	
   728	        let repo = self.open_repo()?;
   729	        let canonical = canonical_encode(entry).map_err(|e| {
   730	            LedgerWriterError::BackendCorruption(format!("canonical_encode: {e}"))
   731	        })?;
   732	
   733	        let mut tb = repo
   734	            .treebuilder(None)
   735	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("treebuilder: {e}")))?;
   736	        let cid_blob = repo
   737	            .blob(&entry.tx_payload_cid.0)
   738	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("cid blob: {e}")))?;
   739	        tb.insert(TREE_BLOB_PAYLOAD_CID, cid_blob, 0o100644)
   740	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert cid: {e}")))?;
   741	        let sig_blob = repo
   742	            .blob(entry.system_signature.as_bytes())
   743	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("sig blob: {e}")))?;
   744	        tb.insert(TREE_BLOB_SIGNATURE, sig_blob, 0o100644)
   745	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert sig: {e}")))?;
   746	        let entry_blob = repo
   747	            .blob(&canonical)
   748	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("entry blob: {e}")))?;
   749	        tb.insert(TREE_BLOB_ENTRY_CANONICAL, entry_blob, 0o100644)
   750	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert entry: {e}")))?;
   751	        let tree_oid = tb
   752	            .write()
   753	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree write: {e}")))?;
   754	        let tree = repo
   755	            .find_tree(tree_oid)
   756	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_tree: {e}")))?;
   757	
   758	        // Determinism: time = (logical_t, 0). NO wall clock.
   759	        let time = git2::Time::new(entry.logical_t as i64, 0);
   760	        let author = GitSignature::new("turingosv4 sequencer", "system@turingos", &time)

exec
/bin/bash -lc "nl -ba src/kernel.rs | sed -n '1,280p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// Tier 1: Pure topology (DAG) + zero-profit treasury + settlement
     2	// Constitutional basis: Law 1 (zero domain knowledge)
     3	// V3L-45: no domain strings. V3L-23: no hardcoded params.
     4	//
     5	// CRITICAL: This module must NEVER contain domain-specific terms.
     6	// R-001 enforced by judge.sh — any edit is scanned.
     7	
     8	use crate::ledger::{Node, NodeId, Tape, TapeError};
     9	use crate::prediction_market::{BinaryMarket, MarketError};
    10	use serde::{Deserialize, Serialize};
    11	use std::collections::HashMap;
    12	
    13	// ── Core types ──────────────────────────────────────────────────
    14	
    15	/// The pure topology manager.
    16	/// It knows about nodes, edges (citations), and markets.
    17	/// It does NOT know what the nodes contain or what domain they belong to.
    18	#[derive(Debug, Serialize, Deserialize)]
    19	pub struct Kernel {
    20	    pub tape: Tape,
    21	    pub markets: HashMap<NodeId, BinaryMarket>,
    22	    /// Phase 3A (Hayek): bounty market opened at run start, seeded with
    23	    /// pre-committed LP from the same ghost-liquidity pool as per-node markets.
    24	    /// Liquid from tx 0 → gives agents a price signal BEFORE any behaviour.
    25	    /// Resolves YES if golden path exists; pool distributed to GP-node authors.
    26	    #[serde(default)]
    27	    pub bounty_market: Option<BinaryMarket>,
    28	    /// Seed LP committed to the bounty market at open time (separate from
    29	    /// BinaryMarket's internal CPMM book). Used for payout distribution.
    30	    #[serde(default)]
    31	    pub bounty_lp_seed: f64,
    32	}
    33	
    34	/// Result of an append operation.
    35	#[derive(Debug)]
    36	pub struct AppendResult {
    37	    pub node_id: NodeId,
    38	}
    39	
    40	/// Result of a resolution operation.
    41	#[derive(Debug)]
    42	pub struct ResolutionResult {
    43	    pub golden_path: Vec<NodeId>,
    44	    pub markets_resolved: usize,
    45	}
    46	
    47	// ── Implementation ──────────────────────────────────────────────
    48	
    49	impl Kernel {
    50	    pub fn new() -> Self {
    51	        Kernel {
    52	            tape: Tape::new(),
    53	            markets: HashMap::new(),
    54	            bounty_market: None,
    55	            bounty_lp_seed: 0.0,
    56	        }
    57	    }
    58	
    59	    /// Phase 3A (Hayek): open a run-level bounty market seeded with `lp_coins`.
    60	    /// Agents see its YES price from tx 0; price pre-exists behaviour,
    61	    /// breaking the Phase 2.5 bootstrap deadlock where no signal existed until
    62	    /// some agent had already acted.
    63	    pub fn open_bounty_market(&mut self, lp_coins: f64) -> Result<(), KernelError> {
    64	        if self.bounty_market.is_some() {
    65	            return Err(KernelError::MarketExists("__bounty__".to_string()));
    66	        }
    67	        let market = BinaryMarket::create("__bounty__".to_string(), lp_coins)
    68	            .map_err(KernelError::Market)?;
    69	        self.bounty_market = Some(market);
    70	        self.bounty_lp_seed = lp_coins;
    71	        Ok(())
    72	    }
    73	
    74	    pub fn bounty_yes_price(&self) -> Option<f64> {
    75	        self.bounty_market.as_ref().map(|m| m.yes_price())
    76	    }
    77	
    78	    /// Resolve the bounty market. `gp_authors` lists the author of each node
    79	    /// on the golden path (duplicates allowed — occurrences proxy contribution
    80	    /// count). Empty list → YES loses, seed returned to ghost pool, no payout.
    81	    /// Non-empty → YES wins, LP distributed equally across entries (so an
    82	    /// author with 2 GP nodes gets twice the share of one with 1).
    83	    pub fn resolve_bounty(&mut self, gp_authors: &[String]) -> HashMap<String, f64> {
    84	        let mut payouts: HashMap<String, f64> = HashMap::new();
    85	        let market = match self.bounty_market.as_mut() {
    86	            Some(m) => m,
    87	            None => return payouts,
    88	        };
    89	        if market.resolved.is_some() {
    90	            return payouts;
    91	        }
    92	        let yes_wins = !gp_authors.is_empty();
    93	        let _ = market.resolve(yes_wins);
    94	        if !yes_wins {
    95	            return payouts;
    96	        }
    97	        let lp = self.bounty_lp_seed;
    98	        let n = gp_authors.len() as f64;
    99	        for a in gp_authors {
   100	            *payouts.entry(a.clone()).or_insert(0.0) += lp / n;
   101	        }
   102	        payouts
   103	    }
   104	
   105	    /// Append a node to the tape.
   106	    /// Only checks structural validity (topology).
   107	    /// Content validation is NOT this module's job (engine separation, C-003).
   108	    pub fn append(&mut self, node: Node) -> Result<AppendResult, KernelError> {
   109	        let node_id = node.id.clone();
   110	        self.tape.append(node).map_err(KernelError::Tape)?;
   111	        Ok(AppendResult { node_id })
   112	    }
   113	
   114	    /// Create a prediction market for a node.
   115	    pub fn create_market(&mut self, node_id: &str, lp_coins: f64) -> Result<(), KernelError> {
   116	        if !self.tape.nodes().contains_key(node_id) {
   117	            return Err(KernelError::NodeNotFound(node_id.to_string()));
   118	        }
   119	        if self.markets.contains_key(node_id) {
   120	            return Err(KernelError::MarketExists(node_id.to_string()));
   121	        }
   122	        let market = BinaryMarket::create(node_id.to_string(), lp_coins)
   123	            .map_err(KernelError::Market)?;
   124	        self.markets.insert(node_id.to_string(), market);
   125	        Ok(())
   126	    }
   127	
   128	    /// Buy YES shares on a node's market.
   129	    pub fn buy_yes(&mut self, node_id: &str, coins: f64) -> Result<f64, KernelError> {
   130	        let market = self.markets.get_mut(node_id)
   131	            .ok_or_else(|| KernelError::MarketNotFound(node_id.to_string()))?;
   132	        let outcome = market.buy_yes(coins).map_err(KernelError::Market)?;
   133	        Ok(outcome.shares_received)
   134	    }
   135	
   136	    /// Buy NO shares on a node's market.
   137	    pub fn buy_no(&mut self, node_id: &str, coins: f64) -> Result<f64, KernelError> {
   138	        let market = self.markets.get_mut(node_id)
   139	            .ok_or_else(|| KernelError::MarketNotFound(node_id.to_string()))?;
   140	        let outcome = market.buy_no(coins).map_err(KernelError::Market)?;
   141	        Ok(outcome.shares_received)
   142	    }
   143	
   144	    /// Trace ancestors from a terminal node back to root(s).
   145	    /// Pure topology — path validity is determined externally.
   146	    pub fn trace_golden_path(&self, terminal_id: &str) -> Result<Vec<NodeId>, KernelError> {
   147	        if !self.tape.nodes().contains_key(terminal_id) {
   148	            return Err(KernelError::NodeNotFound(terminal_id.to_string()));
   149	        }
   150	        Ok(self.tape.trace_ancestors(terminal_id))
   151	    }
   152	
   153	    /// Resolve all markets after external settlement.
   154	    /// `golden_path_ids`: nodes on the verified path (YES wins).
   155	    /// All other markets resolve NO.
   156	    pub fn resolve_all(
   157	        &mut self,
   158	        golden_path_ids: &[NodeId],
   159	    ) -> Result<ResolutionResult, KernelError> {
   160	        let gp_set: std::collections::HashSet<&str> =
   161	            golden_path_ids.iter().map(|s| s.as_str()).collect();
   162	
   163	        let mut resolved_count = 0;
   164	
   165	        for (node_id, market) in self.markets.iter_mut() {
   166	            if market.resolved.is_some() {
   167	                continue;
   168	            }
   169	            let yes_wins = gp_set.contains(node_id.as_str());
   170	            market.resolve(yes_wins).map_err(KernelError::Market)?;
   171	            resolved_count += 1;
   172	        }
   173	
   174	        Ok(ResolutionResult {
   175	            golden_path: golden_path_ids.to_vec(),
   176	            markets_resolved: resolved_count,
   177	        })
   178	    }
   179	
   180	    /// Get the current YES price for a node's market.
   181	    pub fn yes_price(&self, node_id: &str) -> Option<f64> {
   182	        self.markets.get(node_id).map(|m| m.yes_price())
   183	    }
   184	
   185	    /// Get top N nodes by YES price (highest first). Unresolved markets only.
   186	    pub fn market_ticker(&self, top_n: usize) -> Vec<(NodeId, f64)> {
   187	        let mut prices: Vec<(NodeId, f64)> = self.markets.iter()
   188	            .filter(|(_, m)| m.resolved.is_none())
   189	            .map(|(id, m)| (id.clone(), m.yes_price()))
   190	            .collect();
   191	        prices.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
   192	        prices.truncate(top_n);
   193	        prices
   194	    }
   195	
   196	    /// Extended ticker with both sides + reserves, enabling bidirectional price signals
   197	    /// (Art. II.2) and market-depth visibility. Sorted by YES price descending.
   198	    pub fn market_ticker_full(&self, top_n: usize) -> Vec<(NodeId, f64, f64, f64, f64)> {
   199	        let mut rows: Vec<(NodeId, f64, f64, f64, f64)> = self.markets.iter()
   200	            .filter(|(_, m)| m.resolved.is_none())
   201	            .map(|(id, m)| (id.clone(), m.yes_price(), m.no_price(), m.yes_reserve(), m.no_reserve()))
   202	            .collect();
   203	        rows.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
   204	        rows.truncate(top_n);
   205	        rows
   206	    }
   207	}
   208	
   209	impl Default for Kernel {
   210	    fn default() -> Self {
   211	        Self::new()
   212	    }
   213	}
   214	
   215	// ── Errors ──────────────────────────────────────────────────────
   216	
   217	#[derive(Debug)]
   218	pub enum KernelError {
   219	    Tape(TapeError),
   220	    Market(MarketError),
   221	    NodeNotFound(String),
   222	    MarketNotFound(String),
   223	    MarketExists(String),
   224	}
   225	
   226	impl std::fmt::Display for KernelError {
   227	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   228	        match self {
   229	            KernelError::Tape(e) => write!(f, "Tape error: {}", e),
   230	            KernelError::Market(e) => write!(f, "Market error: {}", e),
   231	            KernelError::NodeNotFound(id) => write!(f, "Node not found: {}", id),
   232	            KernelError::MarketNotFound(id) => write!(f, "Market not found for node: {}", id),
   233	            KernelError::MarketExists(id) => write!(f, "Market already exists for node: {}", id),
   234	        }
   235	    }
   236	}
   237	
   238	impl std::error::Error for KernelError {}
   239	
   240	// ── Tests ───────────────────────────────────────────────────────
   241	// NOTE: Domain-purity test lives in tests/kernel_purity.rs (outside this file)
   242	// because R-001 forbids domain terms even as test strings in kernel.rs.
   243	
   244	#[cfg(test)]
   245	mod tests {
   246	    use super::*;
   247	
   248	    fn make_node(id: &str, author: &str, payload: &str, citations: Vec<&str>) -> Node {
   249	        Node {
   250	            id: id.to_string(),
   251	            author: author.to_string(),
   252	            payload: payload.to_string(),
   253	            citations: citations.into_iter().map(|s| s.to_string()).collect(),
   254	            created_at: 0,
   255	            completion_tokens: 0,
   256	        }
   257	    }
   258	
   259	    #[test]
   260	    fn test_append_and_retrieve() {
   261	        let mut k = Kernel::new();
   262	        k.append(make_node("n1", "A0", "step 1", vec![])).unwrap();
   263	        assert!(k.tape.get("n1").is_some());
   264	    }
   265	
   266	    #[test]
   267	    fn test_reject_duplicate() {
   268	        let mut k = Kernel::new();
   269	        k.append(make_node("n1", "A0", "step 1", vec![])).unwrap();
   270	        assert!(k.append(make_node("n1", "A1", "step 2", vec![])).is_err());
   271	    }
   272	
   273	    #[test]
   274	    fn test_reject_dangling_citation() {
   275	        let mut k = Kernel::new();
   276	        assert!(k.append(make_node("n1", "A0", "step 1", vec!["ghost"])).is_err());
   277	    }
   278	
   279	    #[test]
   280	    fn test_market_lifecycle() {

exec
/bin/bash -lc "nl -ba src/bus.rs | sed -n '1,130p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// Tier 4: TSP Event Bus — SKILL lifecycle serial reactor
     2	// Constitutional basis: Art. II (selective broadcast), Art. III (selective shielding)
     3	// V3L-11: serial reactor for causal ordering (no concurrent pricing oscillation)
     4	// V3L-21: one-step-per-node payload limits
     5	// V3L-31: supervisor loop, never silent exit
     6	// V3L-32: cascade failure protection
     7	
     8	use crate::kernel::{Kernel, KernelError};
     9	use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
    10	use crate::sdk::tool::{BetDirection, ToolSignal, TuringTool};
    11	use serde::{Deserialize, Serialize};
    12	use std::collections::HashMap;
    13	
    14	// ── Symbolic constants (V-01 ceremonial kill per D-VETO-7 ratified A) ──────────
    15	
    16	/// TRACE_MATRIX FC1-Cost / FC3-Cost: placeholder until CO1.1.4 STEP_B propagates
    17	/// real LLM completion tokens from `drivers::llm_http::LlmResponse` through to
    18	/// `Node::completion_tokens`. CO1.1.4-pre1 ceremonial commit replaces the magic
    19	/// literal `0` at `bus.rs:268` with this named constant; the value is unchanged
    20	/// (still 0), but the literal is killed so the STEP_B refactor has a clear
    21	/// rename target rather than an anonymous integer.
    22	///
    23	/// See `handover/architect-insights/PROJECT_DECISION_MAP_2026-04-27.md` § 2.2
    24	/// D-VETO-7 for the ratified disposition.
    25	pub(crate) const PENDING_COMPLETION_TOKENS_CO1_1_4: u32 = 0;
    26	
    27	// ── Configuration ───────────────────────────────────────────────
    28	
    29	/// Bus configuration. V3L-23: no hardcoded values, all configurable.
    30	pub struct BusConfig {
    31	    pub max_payload_chars: usize,
    32	    pub max_payload_lines: usize,
    33	    pub system_lp_amount: f64,
    34	    pub forbidden_patterns: Vec<String>,
    35	}
    36	
    37	impl Default for BusConfig {
    38	    fn default() -> Self {
    39	        BusConfig {
    40	            max_payload_chars: 1600,
    41	            max_payload_lines: 24,
    42	            system_lp_amount: 200.0,
    43	            forbidden_patterns: Vec::new(),
    44	        }
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
    61	    graveyard: HashMap<String, Vec<String>>,
    62	    // Phase 1 (C-037 candidate): durable Q_t. None = legacy in-memory mode.
    63	    wal: Option<crate::wal::Wal>,
    64	}
    65	
    66	/// Scope for recent_rejections query.
    67	/// Step-B v3 Art. II.1 fix: enables global abstract-broadcast without violating C-022.
    68	#[derive(Debug, Clone, Copy)]
    69	pub enum RejectionScope {
    70	    /// Legacy: per-author graveyard (before-fix behavior).
    71	    PerAuthor,
    72	    /// Flattened across all authors, chronological (may leak raw content — use with caution).
    73	    Global,
    74	    /// Art. II.1 compliant: counted + top-k class labels. Requires callers to record class labels.
    75	    TopKClasses(usize),
    76	}
    77	
    78	/// Result of a bus append operation.
    79	#[derive(Debug)]
    80	pub enum BusResult {
    81	    Appended { node_id: NodeId },
    82	    Invested { node_id: NodeId, shares: f64 },
    83	    Vetoed { reason: String },
    84	}
    85	
    86	impl TuringBus {
    87	    pub fn new(kernel: Kernel, config: BusConfig) -> Self {
    88	        TuringBus {
    89	            kernel,
    90	            ledger: Ledger::new(),
    91	            tools: Vec::new(),
    92	            config,
    93	            clock: 0,
    94	            tx_count: 0,
    95	            generation: 0,
    96	            graveyard: HashMap::new(),
    97	            wal: None,
    98	        }
    99	    }
   100	
   101	    /// Phase 1: open with WAL persistence. If the path exists, replay it to
   102	    /// rebuild tape + ledger state (resume mode). If not, start fresh and append
   103	    /// to the WAL going forward (durable mode). Either way, the Wal handle is
   104	    /// retained and every successful tape.append / ledger.append persists.
   105	    pub fn with_wal_path(
   106	        kernel: Kernel,
   107	        config: BusConfig,
   108	        wal_path: impl Into<std::path::PathBuf>,
   109	    ) -> Result<Self, std::io::Error> {
   110	        let wal_path = wal_path.into();
   111	        let mut bus = Self::new(kernel, config);
   112	        // Replay first (if file exists), then open in append mode.
   113	        let (nodes, events) = crate::wal::Wal::replay(&wal_path)?;
   114	        let resumed_nodes = nodes.len();
   115	        let resumed_events = events.len();
   116	        for n in nodes {
   117	            // Replay errors are tolerable — duplicates and dangling cites can
   118	            // happen if the WAL was concurrently appended at a stale point. We
   119	            // log and skip; the surviving prefix is canonical Q_t.
   120	            if let Err(e) = bus.kernel.append(n.clone()) {
   121	                eprintln!("[wal/replay] skip node {}: {}", n.id, e);
   122	            }
   123	        }
   124	        for e in events {
   125	            // Re-append events through the ledger so hash chain is recomputed
   126	            // from this process's perspective. Original hashes are discarded.
   127	            bus.ledger.append(e.event_type, e.node_id, e.agent, e.detail).ok();
   128	        }
   129	        if resumed_nodes > 0 || resumed_events > 0 {
   130	            eprintln!("[wal/replay] resumed {} nodes, {} events from {:?}",

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '40,70p;300,318p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    40	
    41	impl Default for Hash {
    42	    fn default() -> Self {
    43	        Hash::ZERO
    44	    }
    45	}
    46	
    47	/// TRACE_MATRIX Art 0.4 — `head_t` = git commit SHA in Path B substrate (40 hex chars).
    48	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    49	pub struct NodeId(pub String);
    50	
    51	impl NodeId {
    52	    /// TRACE_MATRIX § 3 — pseudocode `NodeId::from_state_root(state_root)` constructor.
    53	    /// Concrete derivation (commit-tree-of-state-root) lands in CO1.7 transition_ledger.
    54	    pub fn from_state_root(state_root: Hash) -> Self {
    55	        let mut s = String::with_capacity(64);
    56	        for byte in state_root.0.iter() {
    57	            s.push_str(&format!("{:02x}", byte));
    58	        }
    59	        NodeId(s)
    60	    }
    61	}
    62	
    63	/// TRACE_MATRIX § 1.1 — agent identity (string, opaque to Q_t).
    64	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    65	pub struct AgentId(pub String);
    66	
    67	/// TRACE_MATRIX § 1.1 — accepted-transaction id (string, opaque to Q_t).
    68	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    69	pub struct TxId(pub String);
    70	
   300	// ────────────────────────────────────────────────────────────────────────────
   301	
   302	/// TRACE_MATRIX § 1.1 — system state Q_t. 9 fields per WP § 4 + economic § 2 amendment.
   303	///
   304	/// Reconstructibility: every field is derivable from L4 transition ledger replay
   305	/// (Art IV Boot 公理).
   306	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   307	pub struct QState {
   308	    /// Agent swarm sub-state (tape head per agent + per-agent reputation snapshots).
   309	    pub q_t: AgentSwarmState,
   310	    /// Current ChainTape head pointer = git commit SHA in Path B substrate.
   311	    pub head_t: NodeId,
   312	    /// Materialized state Merkle root (git tree root in Path B).
   313	    pub state_root_t: Hash,
   314	    /// Agent-visible projection of tape filtered by per-agent visibility policy.
   315	    pub tape_view_t: AgentVisibleProjection,
   316	    /// L4 Transition Ledger root (Merkle root of all accepted tx so far).
   317	    pub ledger_root_t: Hash,
   318	    /// L1 Predicate Registry root.

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/store.rs | sed -n '130,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   130	    f.sync_data()?;
   131	    Ok(())
   132	}
   133	
   134	/// Content-addressable store backed by git's blob object database.
   135	#[derive(Debug)]
   136	pub struct CasStore {
   137	    repo_path: PathBuf,
   138	    /// Cid → metadata index. BTreeMap per spec § 2 I-BTREE.
   139	    index: BTreeMap<Cid, CasObjectMetadata>,
   140	}
   141	
   142	impl CasStore {
   143	    /// Open or initialize a CAS store at the given runtime_repo path.
   144	    /// Creates the git repo if it doesn't exist. **CO1.4-extra**: replays
   145	    /// the sidecar `.turingos_cas_index.jsonl` (if any) into the in-memory
   146	    /// index, restoring all metadata that was durably appended in prior
   147	    /// sessions.
   148	    pub fn open(repo_path: &Path) -> Result<Self, CasError> {
   149	        let repo_path = repo_path.to_path_buf();
   150	        let _repo = match Repository::open(&repo_path) {
   151	            Ok(r) => r,
   152	            Err(_) => Repository::init(&repo_path)?,
   153	        };
   154	        let index = load_index_from_sidecar(&repo_path)?;
   155	        Ok(Self { repo_path, index })
   156	    }
   157	
   158	    fn open_repo(&self) -> Result<Repository, CasError> {
   159	        Repository::open(&self.repo_path).map_err(CasError::from)
   160	    }
   161	
   162	    /// Store content; returns its Cid. Idempotent — same content → same Cid.
   163	    pub fn put(
   164	        &mut self,
   165	        content: &[u8],
   166	        object_type: ObjectType,
   167	        creator: &str,
   168	        created_at_logical_t: u64,
   169	        schema_id: Option<String>,
   170	    ) -> Result<Cid, CasError> {
   171	        let cid = Cid::from_content(content);
   172	        let repo = self.open_repo()?;
   173	        let git_oid = repo.blob(content)?;
   174	
   175	        // If already in index, idempotent: just return Cid (content addressing
   176	        // guarantees same content → same Cid → already present)
   177	        if self.index.contains_key(&cid) {
   178	            return Ok(cid);
   179	        }
   180	
   181	        let metadata = CasObjectMetadata {
   182	            cid,
   183	            backend_oid_hex: git_oid.to_string(),
   184	            object_type,
   185	            creator: creator.to_string(),
   186	            created_at_logical_t,
   187	            schema_id,
   188	            size_bytes: content.len() as u64,
   189	        };
   190	        // CO1.4-extra: durably append BEFORE inserting into in-memory index
   191	        // (so a crash mid-write leaves the runtime in a consistent state —
   192	        // either the entry is durably recorded AND in-memory, or neither).
   193	        append_to_sidecar(&self.repo_path, &metadata)?;
   194	        self.index.insert(cid, metadata);
   195	        Ok(cid)
   196	    }
   197	
   198	    /// Retrieve content by Cid. Verifies content sha256 matches Cid (corruption check).
   199	    pub fn get(&self, cid: &Cid) -> Result<Vec<u8>, CasError> {
   200	        let metadata = self
   201	            .index
   202	            .get(cid)
   203	            .ok_or(CasError::CidNotFound(*cid))?;
   204	        let repo = self.open_repo()?;
   205	        let git_oid = git2::Oid::from_str(&metadata.backend_oid_hex)
   206	            .map_err(CasError::Git2)?;
   207	        let blob = repo.find_blob(git_oid)?;
   208	        let content = blob.content().to_vec();
   209	
   210	        // Verify content sha256 matches Cid (defense against corruption).
   211	        let mut h = Sha256::new();
   212	        h.update(&content);
   213	        let computed = Cid(h.finalize().into());
   214	        if &computed != cid {
   215	            return Err(CasError::CidMismatch {
   216	                expected: *cid,
   217	                computed,
   218	            });
   219	        }
   220	

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '178,195p;718,805p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   178	
   179	// ────────────────────────────────────────────────────────────────────────────
   180	// LedgerWriter trait (K4 reconciled to skeleton signature)
   181	// ────────────────────────────────────────────────────────────────────────────
   182	
   183	/// TRACE_MATRIX FC2-Append: storage abstraction for L4.
   184	/// Production impl is `Git2LedgerWriter` (CO1.7.5+; refs/transitions/main commit chain).
   185	/// Test/skeleton impl is `InMemoryLedgerWriter` below.
   186	///
   187	/// **K4**: signature `commit(&mut self) → Hash` (NOT `&self → NodeId`); `iter_from`
   188	/// deferred to CO1.7.5+ (only used by FullTransition replay; not v1 deliverable).
   189	pub trait LedgerWriter: Send + Sync {
   190	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
   191	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
   192	    fn len(&self) -> u64;
   193	}
   194	
   195	#[derive(Debug)]
   718	impl LedgerWriter for Git2LedgerWriter {
   719	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
   720	        let expected = self.len + 1;
   721	        if entry.logical_t != expected {
   722	            return Err(LedgerWriterError::LogicalTGap {
   723	                expected,
   724	                got: entry.logical_t,
   725	            });
   726	        }
   727	
   728	        let repo = self.open_repo()?;
   729	        let canonical = canonical_encode(entry).map_err(|e| {
   730	            LedgerWriterError::BackendCorruption(format!("canonical_encode: {e}"))
   731	        })?;
   732	
   733	        let mut tb = repo
   734	            .treebuilder(None)
   735	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("treebuilder: {e}")))?;
   736	        let cid_blob = repo
   737	            .blob(&entry.tx_payload_cid.0)
   738	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("cid blob: {e}")))?;
   739	        tb.insert(TREE_BLOB_PAYLOAD_CID, cid_blob, 0o100644)
   740	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert cid: {e}")))?;
   741	        let sig_blob = repo
   742	            .blob(entry.system_signature.as_bytes())
   743	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("sig blob: {e}")))?;
   744	        tb.insert(TREE_BLOB_SIGNATURE, sig_blob, 0o100644)
   745	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert sig: {e}")))?;
   746	        let entry_blob = repo
   747	            .blob(&canonical)
   748	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("entry blob: {e}")))?;
   749	        tb.insert(TREE_BLOB_ENTRY_CANONICAL, entry_blob, 0o100644)
   750	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert entry: {e}")))?;
   751	        let tree_oid = tb
   752	            .write()
   753	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree write: {e}")))?;
   754	        let tree = repo
   755	            .find_tree(tree_oid)
   756	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_tree: {e}")))?;
   757	
   758	        // Determinism: time = (logical_t, 0). NO wall clock.
   759	        let time = git2::Time::new(entry.logical_t as i64, 0);
   760	        let author = GitSignature::new("turingosv4 sequencer", "system@turingos", &time)
   761	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("git sig: {e}")))?;
   762	        let committer = author.clone();
   763	
   764	        let parents: Vec<git2::Commit<'_>> = match self.head_oid {
   765	            Some(oid) => vec![repo.find_commit(oid).map_err(|e| {
   766	                LedgerWriterError::BackendCorruption(format!("parent commit: {e}"))
   767	            })?],
   768	            None => Vec::new(),
   769	        };
   770	        let parent_refs: Vec<&git2::Commit<'_>> = parents.iter().collect();
   771	        let message = format!("transition logical_t={}\n", entry.logical_t);
   772	        let new_oid = repo
   773	            .commit(
   774	                Some(TRANSITIONS_REF),
   775	                &author,
   776	                &committer,
   777	                &message,
   778	                &tree,
   779	                &parent_refs,
   780	            )
   781	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("commit: {e}")))?;
   782	
   783	        self.head_oid = Some(new_oid);
   784	        self.len += 1;
   785	        Ok(entry.resulting_ledger_root)
   786	    }
   787	
   788	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
   789	        let bytes = self.read_canonical_bytes(logical_t)?;
   790	        canonical_decode::<LedgerEntry>(&bytes).map_err(|e| {
   791	            LedgerWriterError::BackendCorruption(format!("canonical_decode at {logical_t}: {e}"))
   792	        })
   793	    }
   794	
   795	    fn len(&self) -> u64 {
   796	        self.len
   797	    }
   798	}
   799	
   800	// ────────────────────────────────────────────────────────────────────────────
   801	// Tests — 8 conformance items (4 NEW vs v1 skeleton: K2 / Q9 / repr(u8) / extensions)
   802	// ────────────────────────────────────────────────────────────────────────────
   803	
   804	#[cfg(test)]
   805	mod tests {

exec
/bin/bash -lc "rg -n \"struct Ed25519Keypair|enum KeypairError|struct PredicateRegistry|struct ToolRegistry|derive\\(Debug\" src/bottom_white/ledger/system_keypair.rs src/top_white/predicates/registry.rs src/bottom_white/tools/registry.rs src/state/typed_tx.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/typed_tx.rs:34:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:38:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:50:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:64:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:68:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:74:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:78:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:89:#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
src/state/typed_tx.rs:113:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:124:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:133:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:149:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:164:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:176:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:185:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:199:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/typed_tx.rs:222:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:239:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:258:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:271:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:300:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:316:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:341:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:404:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:425:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:442:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:459:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:478:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:495:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:607:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/typed_tx.rs:716:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/typed_tx.rs:829:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:839:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/bottom_white/tools/registry.rs:22:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/bottom_white/tools/registry.rs:36:#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
src/bottom_white/tools/registry.rs:44:#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
src/bottom_white/tools/registry.rs:56:#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
src/bottom_white/tools/registry.rs:70:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/bottom_white/tools/registry.rs:106:#[derive(Debug, Clone, Default, Serialize, Deserialize)]
src/bottom_white/tools/registry.rs:107:pub struct ToolRegistry {
src/bottom_white/tools/registry.rs:111:#[derive(Debug, PartialEq, Eq)]
src/top_white/predicates/registry.rs:25:#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
src/top_white/predicates/registry.rs:34:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/top_white/predicates/registry.rs:80:#[derive(Debug, Clone, Default, Serialize, Deserialize)]
src/top_white/predicates/registry.rs:81:pub struct PredicateRegistry {
src/top_white/predicates/registry.rs:85:#[derive(Debug, PartialEq, Eq)]
src/bottom_white/ledger/system_keypair.rs:41:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/bottom_white/ledger/system_keypair.rs:63:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
src/bottom_white/ledger/system_keypair.rs:88:#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
src/bottom_white/ledger/system_keypair.rs:152:#[derive(Debug, Clone, PartialEq, Eq)]
src/bottom_white/ledger/system_keypair.rs:185:#[derive(Debug, Clone, PartialEq, Eq)]
src/bottom_white/ledger/system_keypair.rs:224:#[derive(Debug, Clone, PartialEq, Eq)]
src/bottom_white/ledger/system_keypair.rs:256:#[derive(Debug, Clone, Default, PartialEq, Eq)]
src/bottom_white/ledger/system_keypair.rs:284:pub struct Ed25519Keypair {
src/bottom_white/ledger/system_keypair.rs:368:#[derive(Debug)]
src/bottom_white/ledger/system_keypair.rs:369:pub enum KeypairError {
src/bottom_white/ledger/system_keypair.rs:751:#[derive(Debug, Clone, Copy)]

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! Runtime system keypair lifecycle per
     2	//! `handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md`.
     3	//!
     4	//! The private key is generated from OS entropy via `getrandom(2)`, encrypted
     5	//! at rest with ChaCha20-Poly1305, and protected by Argon2id using RFC 9106 /
     6	//! OWASP-class defaults: m=64 MiB, t=3, p=4. The KDF parameters are read from
     7	//! environment variables so deployments can ratchet cost without code churn.
     8	//!
     9	//! /// TRACE_MATRIX FC1-Sig+FC3-Sig: runtime attribution signature primitive
    10	
    11	use crate::boot::TrustRootError;
    12	use argon2::{Algorithm, Argon2, Params, Version};
    13	use chacha20poly1305::aead::{Aead, KeyInit};
    14	use chacha20poly1305::{ChaCha20Poly1305, Nonce};
    15	use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
    16	use secrecy::{ExposeSecret, SecretString};
    17	use serde::{Deserialize, Serialize};
    18	use sha2::{Digest, Sha256};
    19	use std::collections::BTreeMap;
    20	use std::env;
    21	use std::fmt;
    22	use std::fs::{self, OpenOptions};
    23	use std::io::Write;
    24	use std::path::{Path, PathBuf};
    25	use zeroize::{Zeroize, ZeroizeOnDrop};
    26	
    27	const DEFAULT_KDF_MEMORY_KIB: u32 = 65_536;
    28	const DEFAULT_KDF_ITER: u32 = 3;
    29	const DEFAULT_KDF_LANES: u32 = 4;
    30	const DERIVED_KEY_LEN: usize = 32;
    31	const SECRET_KEY_LEN: usize = 32;
    32	const PUBLIC_KEY_LEN: usize = 32;
    33	const SIGNATURE_LEN: usize = 64;
    34	const SALT_LEN: usize = 16;
    35	const NONCE_LEN: usize = 12;
    36	const PLAINTEXT_LEN: usize = SECRET_KEY_LEN + PUBLIC_KEY_LEN;
    37	const FORMAT_MAGIC: &[u8; 11] = b"TOS4SYSKEY1";
    38	const FORMAT_VERSION: u8 = 1;
    39	
    40	/// TRACE_MATRIX FC1-Sig+FC3-Sig: system signature epoch identifier.
    41	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    42	pub struct SystemEpoch(u64);
    43	
    44	impl SystemEpoch {
    45	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a system signature epoch.
    46	    pub const fn new(epoch: u64) -> Self {
    47	        Self(epoch)
    48	    }
    49	
    50	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: expose the numeric epoch for canonical encoding.
    51	    pub const fn get(self) -> u64 {
    52	        self.0
    53	    }
    54	}
    55	
    56	impl From<u64> for SystemEpoch {
    57	    fn from(value: u64) -> Self {
    58	        Self(value)
    59	    }
    60	}
    61	
    62	/// TRACE_MATRIX FC1-Sig+FC3-Sig: ed25519 public key pinned by epoch.
    63	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    64	pub struct SystemPublicKey([u8; PUBLIC_KEY_LEN]);
    65	
    66	impl SystemPublicKey {
    67	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a system public key from raw ed25519 bytes.
    68	    pub const fn from_bytes(bytes: [u8; PUBLIC_KEY_LEN]) -> Self {
    69	        Self(bytes)
    70	    }
    71	
    72	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: expose raw public key bytes for pinning and verification.
    73	    pub const fn as_bytes(&self) -> &[u8; PUBLIC_KEY_LEN] {
    74	        &self.0
    75	    }
    76	
    77	    /// TRACE_MATRIX FC3-Sig: stable SHA-256 fingerprint for audit logs and rotation records.
    78	    pub fn fingerprint_sha256(&self) -> [u8; 32] {
    79	        Sha256::digest(self.0).into()
    80	    }
    81	}
    82	
    83	/// TRACE_MATRIX FC1-Sig+FC3-Sig: ed25519 detached signature over a canonical system message digest.
    84	///
    85	/// `[u8; 64]` serde via `serde_bytes_64` (serde-derive default doesn't support
    86	/// arrays > 32). With `bincode` + `fixed_int_encoding` this writes 64 raw bytes —
    87	/// deterministic, platform-stable.
    88	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    89	pub struct SystemSignature(#[serde(with = "serde_bytes_64")] [u8; SIGNATURE_LEN]);
    90	
    91	impl Default for SystemSignature {
    92	    fn default() -> Self {
    93	        Self([0u8; SIGNATURE_LEN])
    94	    }
    95	}
    96	
    97	/// Serde adapter for `[u8; 64]`: serializes as a length-64 byte sequence
    98	/// (deterministic under bincode `fixed_int_encoding` → 64 raw bytes; no length prefix
    99	/// because the ARRAY type encodes its length statically).
   100	///
   101	/// `pub(crate)` so other in-crate types with `[u8; 64]` fields (e.g.
   102	/// `state::typed_tx::AgentSignature`) can reuse the same adapter — keeps the
   103	/// serde wire format byte-identical across all 64-byte signature types.
   104	pub(crate) mod serde_bytes_64 {
   105	    use serde::de::{SeqAccess, Visitor};
   106	    use serde::ser::SerializeTuple;
   107	    use serde::{Deserializer, Serializer};
   108	    use std::fmt;
   109	
   110	    pub fn serialize<S: Serializer>(bytes: &[u8; 64], s: S) -> Result<S::Ok, S::Error> {
   111	        let mut tup = s.serialize_tuple(64)?;
   112	        for b in bytes {
   113	            tup.serialize_element(b)?;
   114	        }
   115	        tup.end()
   116	    }
   117	
   118	    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 64], D::Error> {
   119	        struct ArrVisitor;
   120	        impl<'de> Visitor<'de> for ArrVisitor {

exec
/bin/bash -lc "nl -ba src/top_white/predicates/registry.rs | sed -n '1,120p'; nl -ba src/bottom_white/tools/registry.rs | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! L1 Predicate Registry — typed metadata store per WP § 5.L1 + spec v1.4 § 1.5.
     2	//!
     3	//! Constitution authority:
     4	//! - Inv 6 (predicate-gated transition): un-passed work_tx does NOT advance state
     5	//! - Inv 10 (signal vs evaluator): private/commit-reveal predicates SHIELDED from agent view
     6	//! - Const Art III.4: Goodhart shield via three visibility classes
     7	//!
     8	//! Spec authority:
     9	//! - STATE_TRANSITION_SPEC v1.4 § 4 invariants I-PRED-GATE + I-NORANDOM bound to this registry
    10	//! - § 2 hidden inputs: BTreeMap (not HashMap) for deterministic iteration order
    11	//!
    12	//! v4 first iteration: typed metadata + register/lookup + Merkle root computation.
    13	//! Predicate EXECUTION (running the actual predicate code on a work_tx) lives in `runner` (future atom CO1.5.6).
    14	//!
    15	//! /// TRACE_MATRIX WP-arch-§5.L1 + Inv-6 + Inv-10: PredicateRegistry
    16	
    17	use serde::{Deserialize, Serialize};
    18	use sha2::{Digest, Sha256};
    19	use std::collections::BTreeMap;
    20	
    21	use super::visibility::Visibility;
    22	
    23	/// Whether a predicate is fail-closed (Safety) or fail-open-with-signal (Creation).
    24	/// Per WP § 7.2.
    25	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    26	pub enum SafetyOrCreation {
    27	    /// Fail-closed: rejected work_tx does NOT advance state_root.
    28	    Safety,
    29	    /// Fail-open-with-signal: rejected work_tx still produces a signal but does not advance state.
    30	    /// (In v4, both behave identically at the state-transition level; difference matters at signal layer.)
    31	    Creation,
    32	}
    33	
    34	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    35	pub struct PredicateMetadata {
    36	    /// Stable identifier; e.g., "lean4_oracle".
    37	    pub predicate_id: String,
    38	    /// Schema/code-hash version.
    39	    pub version: u32,
    40	    /// SHA-256 of compiled bytecode or canonical source.
    41	    pub code_hash: [u8; 32],
    42	    /// JSON Schema (or type ID) describing input shape.
    43	    pub input_schema: String,
    44	    /// JSON Schema describing output shape.
    45	    pub output_schema: String,
    46	    /// Goodhart visibility class.
    47	    pub visibility: Visibility,
    48	    /// Owner (agent_id or "system").
    49	    pub owner: String,
    50	    /// SHA-256 of conformance test suite committed alongside.
    51	    pub test_suite_hash: [u8; 32],
    52	    /// Fail-closed (Safety) or fail-open-with-signal (Creation).
    53	    pub safety_class: SafetyOrCreation,
    54	}
    55	
    56	impl PredicateMetadata {
    57	    /// Canonical hash of this metadata for Merkle tree inclusion.
    58	    /// Bincode-style; mirrors STATE_TRANSITION_SPEC § 2.5 canonical serialization rule
    59	    /// (BTreeMap key order is irrelevant here since fields are fixed-order in struct).
    60	    pub fn canonical_hash(&self) -> [u8; 32] {
    61	        // Manual canonical serialization for v1; matches spec § 2.5 deterministic format.
    62	        // (Avoiding bincode dep in lib for now; upgrade later if v1.4 conformance test demands.)
    63	        let mut h = Sha256::new();
    64	        h.update(self.predicate_id.as_bytes());
    65	        h.update(self.version.to_be_bytes());
    66	        h.update(self.code_hash);
    67	        h.update(self.input_schema.as_bytes());
    68	        h.update(self.output_schema.as_bytes());
    69	        h.update(serde_json::to_vec(&self.visibility).expect("visibility serialize"));
    70	        h.update(self.owner.as_bytes());
    71	        h.update(self.test_suite_hash);
    72	        h.update(serde_json::to_vec(&self.safety_class).expect("safety_class serialize"));
    73	        h.finalize().into()
    74	    }
    75	}
    76	
    77	/// L1 PredicateRegistry — a deterministic ordered store of predicate metadata.
    78	///
    79	/// Uses BTreeMap (not HashMap) per spec § 2 I-BTREE invariant.
    80	#[derive(Debug, Clone, Default, Serialize, Deserialize)]
    81	pub struct PredicateRegistry {
    82	    predicates: BTreeMap<String, PredicateMetadata>,
    83	}
    84	
    85	#[derive(Debug, PartialEq, Eq)]
    86	pub enum RegisterError {
    87	    /// A predicate with this id already registered (use `replace` for explicit replacement).
    88	    DuplicateId(String),
    89	    /// Empty / malformed predicate_id.
    90	    InvalidId(String),
    91	}
    92	
    93	impl PredicateRegistry {
    94	    pub fn new() -> Self {
    95	        Self {
    96	            predicates: BTreeMap::new(),
    97	        }
    98	    }
    99	
   100	    /// Register a NEW predicate. Returns DuplicateId if id exists.
   101	    pub fn register(&mut self, meta: PredicateMetadata) -> Result<(), RegisterError> {
   102	        if meta.predicate_id.is_empty() {
   103	            return Err(RegisterError::InvalidId(meta.predicate_id));
   104	        }
   105	        if self.predicates.contains_key(&meta.predicate_id) {
   106	            return Err(RegisterError::DuplicateId(meta.predicate_id));
   107	        }
   108	        self.predicates.insert(meta.predicate_id.clone(), meta);
   109	        Ok(())
   110	    }
   111	
   112	    /// Lookup by predicate_id.
   113	    pub fn get(&self, id: &str) -> Option<&PredicateMetadata> {
   114	        self.predicates.get(id)
   115	    }
   116	
   117	    /// Total count of registered predicates.
   118	    pub fn len(&self) -> usize {
   119	        self.predicates.len()
   120	    }
     1	//! L2 Tool Registry per WP architecture § 5.L2.
     2	//!
     3	//! Tools are registered with a typed `Capability` (NOT magic string match like
     4	//! `bus.rs:312-319` `manifest() == "wallet"` per spec § 2 hidden-input table).
     5	//!
     6	//! Constitution authority:
     7	//! - WP arch § 5.L2: tool_id + capability + permission + determinism + side_effect schema
     8	//! - Spec § 2 hidden-input table: explicit capability lookup; not string match
     9	//!
    10	//! v4 first iteration: typed metadata + register/lookup by Capability.
    11	//! Tool EXECUTION still happens via existing `TuringTool` trait in `src/sdk/tool.rs`;
    12	//! this registry is a META-LAYER mapping `Capability → ToolId → metadata`.
    13	//!
    14	//! /// TRACE_MATRIX WP-arch-§5.L2: ToolRegistry
    15	
    16	use serde::{Deserialize, Serialize};
    17	use sha2::{Digest, Sha256};
    18	use std::collections::BTreeMap;
    19	
    20	/// Tool capability — typed enum replacing magic string match in legacy code.
    21	/// Per spec § 2 hidden-input table: bus.rs `manifest() == "wallet"` retired in CO1.1.4.
    22	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    23	pub enum Capability {
    24	    EconomicWallet,
    25	    ProofValidator,
    26	    NetworkClient,
    27	    LeanOracle,
    28	    LibrarianBoard,
    29	    SearchTool,
    30	    SandboxedExec,
    31	    /// Custom capability with stable string ID (for v4.1+ extensibility).
    32	    /// Use sparingly; prefer adding a new variant when capability is well-known.
    33	    Custom(String),
    34	}
    35	
    36	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    37	pub enum PermissionPolicy {
    38	    /// Any agent may invoke.
    39	    Open,
    40	    /// Only the system runtime may invoke (e.g., system_keypair sign API).
    41	    SystemOnly,
    42	}
    43	
    44	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    45	pub enum DeterminismClass {
    46	    /// Same input → same output; no side effects. Safe in step_transition.
    47	    Pure,
    48	    /// Reads from external state but no writes. Safe-ish; result depends on read-time.
    49	    ReadOnly,
    50	    /// Idempotent writes (replay-safe).
    51	    IdempotentWrite,
    52	    /// Non-idempotent writes. FORBIDDEN in step_transition path (per spec § 2 I-NOSIDE).
    53	    NonIdempotent,
    54	}
    55	
    56	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    57	pub enum SideEffectClass {
    58	    /// No side effects.
    59	    None,
    60	    /// Reads filesystem.
    61	    FilesystemRead,
    62	    /// Writes filesystem.
    63	    FilesystemWrite,
    64	    /// Network access.
    65	    Network,
    66	    /// Spawns subprocess.
    67	    Subprocess,
    68	}
    69	
    70	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    71	pub struct ToolMetadata {
    72	    pub tool_id: String,
    73	    pub version: u32,
    74	    pub capability: Capability,
    75	    pub permission_policy: PermissionPolicy,
    76	    pub determinism_class: DeterminismClass,
    77	    pub side_effect_class: SideEffectClass,
    78	    pub schema: String,
    79	    pub creator: String,
    80	    /// SHA-256 of bytecode/source.
    81	    pub code_hash: [u8; 32],
    82	    /// SHA-256 of conformance test suite.
    83	    pub test_suite_hash: [u8; 32],
    84	    /// Reuse royalty share for AttributionEngine (CO P2.4 spike). Stored as MicroFraction.
    85	    pub reuse_royalty_share_micro: i64,
    86	}
    87	
    88	impl ToolMetadata {
    89	    pub fn canonical_hash(&self) -> [u8; 32] {
    90	        let mut h = Sha256::new();
    91	        h.update(self.tool_id.as_bytes());
    92	        h.update(self.version.to_be_bytes());
    93	        h.update(serde_json::to_vec(&self.capability).expect("capability serialize"));
    94	        h.update(serde_json::to_vec(&self.permission_policy).expect("perm serialize"));
    95	        h.update(serde_json::to_vec(&self.determinism_class).expect("det serialize"));
    96	        h.update(serde_json::to_vec(&self.side_effect_class).expect("se serialize"));
    97	        h.update(self.schema.as_bytes());
    98	        h.update(self.creator.as_bytes());
    99	        h.update(self.code_hash);
   100	        h.update(self.test_suite_hash);
   101	        h.update(self.reuse_royalty_share_micro.to_be_bytes());
   102	        h.finalize().into()
   103	    }
   104	}
   105	
   106	#[derive(Debug, Clone, Default, Serialize, Deserialize)]
   107	pub struct ToolRegistry {
   108	    tools: BTreeMap<String, ToolMetadata>,
   109	}
   110	
   111	#[derive(Debug, PartialEq, Eq)]
   112	pub enum RegisterError {
   113	    DuplicateId(String),
   114	    InvalidId(String),
   115	    /// `step_transition`-reachable code attempted to register a tool with NonIdempotent
   116	    /// determinism class. Forbidden by spec § 2 I-NOSIDE.
   117	    NonIdempotentNotAllowed(String),
   118	}
   119	
   120	impl ToolRegistry {

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '276,312p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   276	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: fetch the public key pinned for a system epoch.
   277	    pub fn get(&self, epoch: SystemEpoch) -> Option<&SystemPublicKey> {
   278	        self.keys.get(&epoch)
   279	    }
   280	}
   281	
   282	/// TRACE_MATRIX FC1-Sig+FC3-Sig: in-memory ed25519 system keypair with zeroized private key on drop.
   283	#[derive(Zeroize, ZeroizeOnDrop)]
   284	pub struct Ed25519Keypair {
   285	    secret_key: Box<[u8]>,
   286	    #[zeroize(skip)]
   287	    public_key: SystemPublicKey,
   288	}
   289	
   290	impl Ed25519Keypair {
   291	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: generate ed25519 key material from `getrandom(2)` entropy.
   292	    pub fn generate_with_secure_entropy() -> Result<Self, KeypairError> {
   293	        let mut seed = [0u8; SECRET_KEY_LEN];
   294	        getrandom::getrandom(&mut seed).map_err(KeypairError::Entropy)?;
   295	        let signing_key = SigningKey::from_bytes(&seed);
   296	        let public_key = SystemPublicKey::from_bytes(signing_key.verifying_key().to_bytes());
   297	        let mut keypair = Self {
   298	            secret_key: Vec::from(seed).into_boxed_slice(),
   299	            public_key,
   300	        };
   301	        seed.zeroize();
   302	        keypair.mlock_private_key_best_effort();
   303	        Ok(keypair)
   304	    }
   305	
   306	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: return the public half of the system keypair.
   307	    pub const fn public_key(&self) -> SystemPublicKey {
   308	        self.public_key
   309	    }
   310	
   311	    fn from_plaintext(plaintext: &[u8]) -> Result<Self, KeypairError> {
   312	        if plaintext.len() != PLAINTEXT_LEN {

codex
A few substantive issues are emerging: the split itself is mostly clean, but the spec still leaves the `head_t` write effectively untested and the proposed `Debug` strategy is not compile-complete as written. I’m checking whether those are isolated spec patches or deeper blockers.
exec
/bin/bash -lc 'git log --oneline -5 --decorate' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
617f01e (HEAD -> main) CO1.7-extra v1 footer: smoke 8/8 PASS at HEAD f7fc19f
f7fc19f CO1.7-extra v1 spec rewrite: scope split per round-1 Occam decision
e05a1ec CO1.7.5 round-1 dual external audit: CHALLENGE/CHALLENGE — substrate gap surfaced
334111a CO1.7.5 spec v1 DRAFT: transition bodies + runtime wiring (smoke 8/8 + self-audit 12 patches)
b2036aa hygiene: fix STEP_B-restricted file path drift (wallet.rs → sdk/tools/wallet.rs)

exec
/bin/bash -lc "rg -n \"fn canonical_test_entry|entry_at\\(|Git2LedgerWriter::open|head_commit_oid_hex|cas_payload_round_trip|co1_7_extra\" src tests handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:39:| CO1.4-extra (commit `b6b7574`) | CAS sidecar JSONL index persistence (substrate for the cas_payload_round_trip test) |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:43:1. **D2** — `q.head_t = state::q_state::NodeId(commit_oid_hex)` after `writer.commit(&entry)` returns Ok; adds 1 trait method `LedgerWriter::head_commit_oid_hex` with mandatory-override design pattern (Q1 synthesis from round-1).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:45:3. **D4-substrate-independent** — One conformance test `tests/cas_payload_round_trip` (`CasStore::put` → `get` round-trip with CID stability post-CO1.4-extra). Other 3 D4 tests (replay state-root + system-signature canonical-message + un-ignore byte-identity) move to future CO1.7.5 atom because they require D1 transition bodies to actually commit.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:72:if let Some(commit_oid_hex) = writer_w.head_commit_oid_hex() {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:79:**Atomicity** (per Codex Q-B finding, refined): under acquired `q_w` + `writer_w` write locks, after `writer_w.commit(&entry)?` returns `Ok`, the remaining operations are an `AtomicU64::store` (infallible), a plain `*q_w = q_next` move (infallible), and field assignments (infallible). The atomicity claim fully holds for writers whose `head_commit_oid_hex` returns `Some` (Git2LedgerWriter); writers returning `None` (InMemoryLedgerWriter) leave `q.head_t` unchanged from `q_next.head_t` (which equals `q.head_t` per CO1.7 K3 v1.2 — transition bodies don't mutate head_t even when they exist in CO1.7.5).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:102:    fn head_commit_oid_hex(&self) -> Option<String> {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:108:    fn head_commit_oid_hex(&self) -> Option<String> {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:120:    fn head_commit_oid_hex(&self) -> Option<String> {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:212:Two tests in `tests/co1_7_extra/`:
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:214:### 3.1 `cas_payload_round_trip`
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:217:// tests/co1_7_extra/cas_payload_round_trip.rs (NEW)
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:226:fn cas_payload_round_trip_with_cid_stability_across_restart() {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:246:// tests/co1_7_extra/git2_writer_head_oid_defense.rs (NEW)
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:250:    let mut writer = Git2LedgerWriter::open(tmp.path()).expect("open");
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:257:        writer.head_commit_oid_hex().is_some(),
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:285:(The original v1 Q1 — `head_commit_oid_hex` default impl — is now resolved per Q1 synthesis in § 1.2: default `None` + mandatory override + defensive test.)
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:315:2. **`head_commit_oid_hex` is a NEW trait method** with mandatory-override design (Q1 synthesis: default `None` + every impl overrides + defensive test).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:321:8. **FC-trace requirements**: the new pub symbols introduced by CO1.7-extra implementation must carry doc-comment `/// TRACE_MATRIX <FC-id>: <role>` backlinks per CLAUDE.md "Alignment Standard". Set: `LedgerWriter::head_commit_oid_hex` (→ § 5 L4 sequencer post-commit head_t wiring); `Kernel.sequencer` field + `Kernel::with_sequencer` + `TuringBus::submit_typed_tx` (→ § 5.2.1 single-writer entry-point).
src/bottom_white/ledger/transition_ledger.rs:813:    fn entry_at(
src/bottom_white/ledger/transition_ledger.rs:880:        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
src/bottom_white/ledger/transition_ledger.rs:883:        let e_skip = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:891:        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
src/bottom_white/ledger/transition_ledger.rs:892:        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:893:        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
src/bottom_white/ledger/transition_ledger.rs:904:        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
src/bottom_white/ledger/transition_ledger.rs:905:        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:914:        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
src/bottom_white/ledger/transition_ledger.rs:915:        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:926:        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
src/bottom_white/ledger/transition_ledger.rs:927:        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:938:        let e_clean = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
src/bottom_white/ledger/transition_ledger.rs:1037:        let w = Git2LedgerWriter::open(tmp.path()).expect("open");
src/bottom_white/ledger/transition_ledger.rs:1054:        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
src/bottom_white/ledger/transition_ledger.rs:1055:        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:1056:        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
src/bottom_white/ledger/transition_ledger.rs:1082:        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
src/bottom_white/ledger/transition_ledger.rs:1087:        let e_skip = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:1100:        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
src/bottom_white/ledger/transition_ledger.rs:1101:        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:1104:            let mut w = Git2LedgerWriter::open(tmp.path()).expect("open");
src/bottom_white/ledger/transition_ledger.rs:1110:        let w2 = Git2LedgerWriter::open(tmp.path()).expect("reopen");
src/bottom_white/ledger/transition_ledger.rs:1117:        let mut w3 = Git2LedgerWriter::open(tmp.path()).expect("reopen 2");
src/bottom_white/ledger/transition_ledger.rs:1118:        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
src/bottom_white/ledger/transition_ledger.rs:1293:    /// 17. cas_payload_round_trip — replay correctly fetches CAS bytes;
src/bottom_white/ledger/transition_ledger.rs:1296:    fn replay_cas_payload_round_trip_after_reopen() {
src/bottom_white/ledger/transition_ledger.rs:1466:        let e1 = entry_at(7, h(0xaa), h(0xbb), h(0xcc));

exec
/bin/bash -lc 'rg -n "submit_typed_tx|sequencer:|with_sequencer|Arc<Sequencer>|state::sequencer|head_commit_oid_hex" src' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/ledger/transition_ledger.rs:385:    use crate::state::sequencer::dispatch_transition;

exec
/bin/bash -lc "rg -n \"Audit Standard|Alignment Standard|Code Standard|STEP_B|TRACE_MATRIX|R-001|kernel.rs\" CLAUDE.md handover -g 'CLAUDE.md' -g 'STEP_B_PROTOCOL.md'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
CLAUDE.md:12:## Code Standard (Art. I.1 + C-004 + C-027)
CLAUDE.md:14:- `src/{kernel,bus}.rs` + `src/sdk/tools/wallet.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
CLAUDE.md:17:## Audit Standard (Art. V.1 + C-010 + C-023 + C-035)
CLAUDE.md:34:## Alignment Standard (Art. IV + C-069)
CLAUDE.md:35:- 权威对齐文件: `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md`
CLAUDE.md:36:  (后续 rev: `TRACE_MATRIX_vN.md`)
CLAUDE.md:38:  或 BLOCK merge。doc-comment backlink 格式: `/// TRACE_MATRIX <FC-id>: <role>`
handover/ai-direct/STEP_B_PROTOCOL.md:3:**Scope**: any change to files in CLAUDE.md's restricted list (currently `src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`). Also applicable to any proposal that touches "institution" per C-031.

exec
/bin/bash -lc 'rg -n "struct NodeId|type NodeId" src/ledger.rs src/state/q_state.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/q_state.rs:49:pub struct NodeId(pub String);
src/ledger.rs:13:pub type NodeId = String;

exec
/bin/bash -lc "rg -n \"sequencer_serial_replay_byte_identity|#\\[ignore\" src/bottom_white/ledger/transition_ledger.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
1451:    /// 18. sequencer_serial_replay_byte_identity — gated behind #[ignore]
1453:    ///     test is here so CO1.7.5 just removes the #[ignore].
1455:    #[ignore = "CO1.7.5: requires real per-kind transition bodies"]
1456:    fn sequencer_serial_replay_byte_identity() {

exec
/bin/bash -lc "nl -ba CLAUDE.md | sed -n '1,55p'; nl -ba handover/ai-direct/STEP_B_PROTOCOL.md | sed -n '1,90p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TuringOS v4
     2	
     3	## What
     4	Silicon-Native Microkernel for LLM Formal Verification Swarm.
     5	Rust 2021, tokio, serde_json. Mission: MiniF2F Lean 4.
     6	
     7	## Why
     8	- 唯一对齐文档: `constitution.md` (反奥利奥架构)
     9	- 压缩即智能: 抽象原则进宪法 / 具体情境进 `cases/`
    10	- 机制 > 参数 > 提示 (Art. V + C-021/C-031/C-034/C-043)
    11	
    12	## Code Standard (Art. I.1 + C-004 + C-027)
    13	- `cargo check` / `cargo test` 必过；`.env` 永不 commit
    14	- `src/{kernel,bus}.rs` + `src/sdk/tools/wallet.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
    15	- 任何影响行为的参数必须 env/config 可覆盖，不可硬编码
    16	
    17	## Audit Standard (Art. V.1 + C-010 + C-023 + C-035)
    18	- Generator ≠ Evaluator：代码作者不可是唯一审计者
    19	- 所有 merge / phase 决策双外审（Codex + Gemini）；VETO > CHALLENGE > PASS
    20	- 宪法违规立即 BLOCKER，不可延期、不可"可接受"
    21	
    22	## Report Standard (Art. I.2 + Art. II.2.1 + Art. IV 强制, C-052 + C-053 + C-057 + C-059 + C-061)
    23	- **主指标**（每报必填）: ΣPPUT + Mean PPUT (solved) + 95% CI (Wilson)
    24	- Art. I.2 三大统计信号不可缺: **信誉** (reputation_distribution p50/p90/max) + 效用 (PPUT) + 共识 (如适用)
    25	- Art. IV 终态区分: `halt_reason_distribution` {OmegaAccepted, MaxTxExhausted, WallClockCap, ComputeCapViolated, ErrorHalt}
    26	- 多 agent (n≥2) 专用: `parent_selection_entropy` + `pairwise_payload_diversity_mean`；任一 < 0.25 = Art. II.2.1 告警
    27	- solve count 不可独立陈述，必须配对 PPUT；以 solve count 起头 = 违宪
    28	
    29	## Reproducibility Standard (Art. I + C-012/C-016/C-032/C-039)
    30	- OMEGA accept 必留 self-contained artifact (`proofs/*.lean` + `gp_payload`)
    31	- 度量工具上线即冻结；Oracle 参数冻结；实验禁混 Oracle 模式
    32	- 中间件若修改数学内容 → 是 ArchitectAI 贡献，不是 swarm 涌现（C-023）
    33	
    34	## Alignment Standard (Art. IV + C-069)
    35	- 权威对齐文件: `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md`
    36	  (后续 rev: `TRACE_MATRIX_vN.md`)
    37	- 每个 src/ pub 符号必须映射到宪法 flowchart 元素、标 orphan+justification、
    38	  或 BLOCK merge。doc-comment backlink 格式: `/// TRACE_MATRIX <FC-id>: <role>`
    39	- Conformance tests: `tests/fc_alignment_conformance.rs` — 每个 ✅ 行 ≥1
    40	  witness test；`#[ignore]` stub 覆盖 📅 deferred rows
    41	- 宪法 flowchart 修改仅 human architect 可触发，需重跑 Phase Z′ 6-stage
    42	- constitution.md hygiene 观察登记到 `handover/alignment/OBS_*.md`，不改宪法
    43	
    44	## Common Law (宪法 + 判例)
    45	宪法高度压缩，具体裁决查 `cases/C-xxx.yaml` (facts → ruling → precedent)
    46	- 按条款查: `grep -l "Art. I.1" cases/*.yaml`
    47	- 映射：`cases/V3_LESSONS.md` (50 v3 教训 → 现行判例)
    48	- 编号跳号：C-038 / C-042 为 reserved（见 C-041/C-043 预引用）
    49	
    50	## Docs (按需加载)
    51	| 文档 | 何时加载 |
    52	|------|---------|
    53	| `docs/architecture.md` | 修改 src/ 核心模块时 |
    54	| `docs/economics.md` | 修改经济引擎 (wallet/market) 时 |
    55	| `docs/hardware.md` | SSH/部署/远程操作时 |
     1	# Step-B Protocol: Restricted-File Changes via Parallel Branch Experiment
     2	
     3	**Scope**: any change to files in CLAUDE.md's restricted list (currently `src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`). Also applicable to any proposal that touches "institution" per C-031.
     4	
     5	**Principle**: real data decides, not opinion. Parallel branch lets us test change empirically before merging.
     6	
     7	---
     8	
     9	## Phase 0 — Necessity audit (external, before any code)
    10	
    11	External auditors (**Codex + Gemini**) answer:
    12	- Is the change *necessary*? What observable behavior is broken now?
    13	- Is a less-invasive alternative available? (same effect, no restricted-file touch)
    14	- What's the *minimum sufficient* version? (avoid scope creep)
    15	- What's the failure mode if we don't change?
    16	
    17	**Gate**: if both auditors say "less-invasive alternative exists", take that path instead. If both say "change is necessary", proceed to Phase 1. If they disagree, take the conservative verdict per `feedback_dual_audit_conflict`.
    18	
    19	**My role**: write the audit brief; do not pre-advocate. If I agree with the verdict after reading both, proceed. If I disagree, record dissent in notepad §7 before proceeding.
    20	
    21	---
    22	
    23	## Phase 1 — Parallel branch creation
    24	
    25	### 1a. Worktree spawn
    26	```bash
    27	git worktree add .claude/worktrees/stepb-<slug> -b experiment/<slug>
    28	```
    29	
    30	Or via Agent tool with `isolation: worktree` frontmatter — for short experiments (<2h). Agent returns branch name + diff summary.
    31	
    32	### 1b. Implementation in isolation
    33	- Work in the experiment branch only.
    34	- Main branch stays at the last audited-PASS state.
    35	- Add fixture tests covering the change.
    36	- Run `cargo test` on experiment branch → must be green.
    37	- Commit with message `experiment/<slug>: <change>` (not merged).
    38	
    39	### 1c. Implementation audit
    40	External auditors (same two) review the **diff only**:
    41	- Is the change minimal?
    42	- Are tests sufficient?
    43	- Does it introduce new constitutional debt?
    44	- Any risk the diff itself is a Trojan (side-effects beyond scope)?
    45	
    46	**Gate**: both PASS on diff → proceed to Phase 2. Any VETO → abandon branch or revise.
    47	
    48	---
    49	
    50	## Phase 2 — Statistical A/B test (pre-registered)
    51	
    52	### 2a. Pre-register experimental design
    53	Before any batch run, write a locked spec to `handover/ai-direct/AB_<slug>_<date>.md`:
    54	
    55	- **Null hypothesis (H0)**: p(treatment solves) = p(control solves) on frozen sample
    56	- **Primary metric**: SolveRate delta (per `metrics.yaml`)
    57	- **Secondary metrics**: Aggregate_PPUT, mean wall time
    58	- **Sample**: identical to M4 (seed=74677, N=50, fingerprint=796ead6c40351ae9)
    59	- **Sample size justification**: binomial approximation — a 3-solve difference (6 pp on N=50) is ~2σ above null; 6-solve difference (12 pp) is ~4σ. Pre-register **strict-win threshold = ΔSolveRate ≥ 3** (consistent with v3.1 decision rule).
    60	- **Statistical test**: McNemar's test for paired nominal data. Report p-value but do not use as primary (SolveRate delta is).
    61	- **Decision rule**:
    62	  - `ΔSolveRate ≥ 3`: treatment strict win → merge candidate
    63	  - `-1 ≤ Δ ≤ 2`: inconclusive → either expand N (same seed family) or abandon
    64	  - `ΔSolveRate ≤ -1`: treatment regresses → abandon branch
    65	- **Interleaving**: run both conditions on each problem alternately (or in parallel if possible) to neutralize API drift (C-033).
    66	- **Abort gate**: per-condition 20% / 30% (same as v3.1).
    67	
    68	### 2b. Execute on both branches
    69	- **Control branch** (main or last-PASS HEAD): run `run_interleaved.sh` with `TREATMENT_LABEL=control`
    70	- **Treatment branch** (experiment/<slug>): run same script with `TREATMENT_LABEL=treatment`
    71	- **Cost**: 2× the single-arm experiment. Pre-registered budget before spending.
    72	
    73	### 2c. Freeze analyzer
    74	`frozen_analysis.py` for A/B extension:
    75	- Must support `--control <jsonl>` and `--treatment <jsonl>` flags
    76	- Outputs: paired-comparison table, McNemar p, ΔSolveRate, discordant pairs list
    77	- Must be fixture-tested before A/B run (C-012 mandatory freeze).
    78	
    79	---
    80	
    81	## Phase 3 — Verdict + commit path
    82	
    83	### 3a. Read the data
    84	After both branches finish, run `frozen_analysis.py --control ... --treatment ...`.
    85	
    86	### 3b. Audit the verdict (again)
    87	External auditors see **data only** (no researcher interpretation). They apply the pre-registered decision rule and return PASS / FAIL / INCONCLUSIVE.
    88	
    89	### 3c. Merge or abandon
    90	- **Treatment win (audits PASS)**: 

exec
/bin/bash -lc 'cargo check --workspace' in /home/zephryj/projects/turingosv4
 succeeded in 286ms:
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
  --> src/bus.rs:11:13
   |
11 | use serde::{Deserialize, Serialize};
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

warning: `turingosv4` (lib) generated 18 warnings (run `cargo fix --lib -p turingosv4` to apply 11 suggestions)
warning: variable does not need to be mutable
  --> spike/gix_capability/src/c1_init.rs:21:13
   |
21 |         let mut index = repo.index()?;
   |             ----^^^^^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: `gix_capability_spike` (lib) generated 1 warning (run `cargo fix --lib -p gix_capability_spike` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s

codex
Baseline `cargo check --workspace` is clean apart from existing warnings, so the remaining concerns are spec/implementation-plan issues rather than current-tree breakage.
exec
/bin/bash -lc 'cargo test --workspace --lib' in /home/zephryj/projects/turingosv4
 succeeded in 1569ms:
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
  --> src/bus.rs:11:13
   |
11 | use serde::{Deserialize, Serialize};
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

warning: variable does not need to be mutable
  --> spike/gix_capability/src/c1_init.rs:21:13
   |
21 |         let mut index = repo.index()?;
   |             ----^^^^^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (lib) generated 18 warnings (run `cargo fix --lib -p turingosv4` to apply 11 suggestions)
warning: `gix_capability_spike` (lib test) generated 1 warning (run `cargo fix --lib -p gix_capability_spike --tests` to apply 1 suggestion)
warning: variable does not need to be mutable
   --> src/top_white/predicates/registry.rs:255:17
    |
255 |             let mut h = Sha256::new();
    |                 ----^
    |                 |
    |                 help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:560:19
    |
560 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (lib test) generated 16 warnings (14 duplicates) (run `cargo fix --lib -p turingosv4 --tests` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/gix_capability_spike-758eb611b9b651aa)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/minif2f_v4-1dd6619c480d44e3)

running 104 tests
test agent_models::tests::csv_entries_trimmed ... ok
test agent_models::tests::empty_env_parses_to_empty_vec ... ok
test agent_models::tests::empty_csv_slot_rejected ... ok
test agent_models::tests::empty_parsed_broadcasts_global_model ... ok
test agent_models::tests::heterogeneous_without_gate_rejected ... ok
test agent_models::tests::positional_length_match_passes ... ok
test agent_models::tests::heterogeneous_with_gate_passes ... ok
test agent_models::tests::length_mismatch_rejected ... ok
test agent_models::tests::single_entry_broadcasts ... ok
test agent_models::tests::single_entry_parses ... ok
test agent_models::tests::uniform_length_n_does_not_trip_hetero_gate ... ok
test budget_regime::tests::effective_per_agent_scales_linearly_in_n ... ok
test budget_regime::tests::effective_per_agent_overflow_rejected ... ok
test budget_regime::tests::effective_token_total_unimplemented ... ok
test budget_regime::tests::effective_total_proposal_invariant_under_n ... ok
test budget_regime::tests::effective_wall_clock_unimplemented ... ok
test budget_regime::tests::label_strings_are_stable ... ok
test budget_regime::tests::n_agents_zero_does_not_panic ... ok
test budget_regime::tests::parse_max_transactions_empty_defaults_to_200 ... ok
test budget_regime::tests::parse_max_transactions_garbage_rejected ... ok
test budget_regime::tests::parse_max_transactions_negative_rejected ... ok
test budget_regime::tests::parse_max_transactions_valid ... ok
test budget_regime::tests::parse_max_transactions_zero_rejected ... ok
test budget_regime::tests::parse_regime_empty_defaults_to_total_proposal ... ok
test budget_regime::tests::parse_regime_known_values ... ok
test budget_regime::tests::parse_regime_unknown_rejected ... ok
test budget_regime::tests::resolve_budget_default_preserves_phase_b_baseline ... ok
test budget_regime::tests::resolve_budget_per_agent_via_env ... ok
test budget_regime::tests::resolve_budget_token_total_startup_fatal ... ok
test cost_aggregator::tests::test_empty_accumulator_zero_total ... ok
test cost_aggregator::tests::test_failed_branches_counted_in_total_cost ... ok
test cost_aggregator::tests::test_tool_stdout_chars_div_4_approximation ... ok
test experiment_mode::tests::apply_full_is_passthrough ... ok
test cost_aggregator::tests::test_flip_underflow_panics - should panic ... ok
test budget_regime::tests::resolve_budget_unknown_regime_via_env ... ok
test experiment_mode::tests::apply_idempotent_for_all_modes ... ok
test experiment_mode::tests::apply_soft_law_forces_runtime_accept ... ok
test experiment_mode::tests::apply_soft_law_preserves_post_hoc_verified ... ok
test experiment_mode::tests::apply_other_modes_passthrough_pre_implementation ... ok
test experiment_mode::tests::ensure_implemented_post_c1e_all_modes_pass ... ok
test experiment_mode::tests::extract_after_positional ... ok
test experiment_mode::tests::extract_dangling_flag_returns_empty_string ... ok
test experiment_mode::tests::extract_last_wins_for_repeated_flag ... ok
test experiment_mode::tests::extract_no_flag_returns_none ... ok
test experiment_mode::tests::extract_equals_form ... ok
test experiment_mode::tests::extract_space_form ... ok
test experiment_mode::tests::is_panopticon_predicate ... ok
test experiment_mode::tests::parse_empty_defaults_to_full ... ok
test experiment_mode::tests::is_amnesia_predicate ... ok
test experiment_mode::tests::label_strings_are_stable ... ok
test experiment_mode::tests::parse_case_sensitive ... ok
test experiment_mode::tests::parse_known_values ... ok
test experiment_mode::tests::parse_unknown_rejected ... ok
test experiment_mode::tests::predicates_are_mutually_exclusive ... ok
test experiment_mode::tests::resolve_cli_empty_aborts_no_default_fallback ... ok
test experiment_mode::tests::resolve_cli_overrides_env ... ok
test experiment_mode::tests::resolve_cli_unknown_aborts ... ok
test experiment_mode::tests::resolve_default_no_cli_no_env_is_full ... ok
test experiment_mode::tests::resolve_env_all_modes_implemented_post_c1e ... ok
test experiment_mode::tests::resolve_env_full_explicit ... ok
test experiment_mode::tests::skill_idx_homogeneous_always_zero ... ok
test experiment_mode::tests::skill_idx_full_cycles_modulo ... ok
test experiment_mode::tests::skill_idx_other_modes_passthrough ... ok
test experiment_mode::tests::skill_idx_zero_n_returns_zero ... ok
test fc_trace::tests::emit_event_with_full_payload_does_not_panic ... ok
test fc_trace::tests::emit_event_with_no_kv_or_agent_does_not_panic ... ok
test fc_trace::tests::emit_is_no_op_when_disabled ... ok
test fc_trace::tests::fc_id_display_matches_as_str ... ok
test fc_trace::tests::fc_id_strings_are_stable ... ok
test fc_trace::tests::json_str_escapes_required_chars ... ok
test jsonl_schema::tests::test_a4_tactic_diversity_helper ... ok
test jsonl_schema::tests::test_a4_verifier_wait_bounded_by_total_wall_time ... ok
test jsonl_schema::tests::test_a4_decomposed_metrics_round_trip ... ok
test jsonl_schema::tests::test_a5_budget_regime_round_trip ... ok
test jsonl_schema::tests::test_jsonl_schema_v2_round_trip ... ok
test lean4_oracle::tests::test_clean_tactic_accepted ... ok
test lean4_oracle::tests::test_correct_theorem_name_accepted ... ok
test jsonl_schema::tests::test_pput_verified_zero_when_progress_zero ... ok
test lean4_oracle::tests::test_decide_tactic_permitted ... ok
test lean4_oracle::tests::test_forbidden_native_decide ... ok
test jsonl_schema::tests::test_legacy_jsonl_still_readable ... ok
test lean4_oracle::tests::test_identity_theft_rejected ... ok
test lean4_oracle::tests::test_sorry_in_word_not_rejected ... ok
test lean4_oracle::tests::test_forbidden_io_process ... ok
test lean4_oracle::tests::test_sorry_rejected ... ok
test lean4_oracle::tests::test_word_boundary_function ... ok
test post_hoc_verifier::tests::test_no_runtime_accept_zeros_both_pput ... ok
test post_hoc_verifier::tests::test_post_hoc_verified_without_runtime_still_zero_progress ... ok
test post_hoc_verifier::tests::test_pput_verified_matches_runtime_when_both_accept ... ok
test rollback_sim::tests::does_not_fire_after_threshold ... ok
test post_hoc_verifier::tests::test_pput_verified_zero_when_lean_rejects ... ok
test rollback_sim::tests::does_not_fire_before_threshold ... ok
test rollback_sim::tests::env_var_name_matches_prereg ... ok
test rollback_sim::tests::never_fires_when_disabled ... ok
test rollback_sim::tests::fires_at_threshold_when_enabled ... ok
test rollback_sim::tests::threshold_constant_matches_prereg ... ok
test run_id::tests::handles_path_with_no_stem ... ok
test run_id::tests::distinguishes_conditions_for_same_problem ... ok
test run_id::tests::shape_matches_legacy_make_pput_format ... ok
test wall_clock::tests::test_wall_clock_first_read_to_final_accept ... ok
test wall_clock::tests::test_wall_clock_unmarked_returns_none ... ok
test wall_clock::tests::test_wall_clock_final_accept_overwrites ... ok
test wall_clock::tests::test_wall_clock_first_read_idempotent ... ok
test wall_clock::tests::test_wall_clock_no_final_accept_uses_now ... ok

test result: ok. 104 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running unittests src/lib.rs (target/debug/deps/turingosv4-02347eed360488fd)

running 240 tests
test boot::tests::parse_errors_when_section_missing ... ok
test boot::tests::parse_errors_on_unquoted_key ... ok
test boot::tests::parse_strips_inline_comment_and_blanks ... ok
test boot::tests::verify_trust_root_detects_child_manifest_tamper ... ok
test boot::tests::verify_trust_root_detects_tamper_in_tempdir ... ok
test boot::tests::verify_trust_root_passes_with_matching_child_manifest ... ok
test boot::tests::verify_trust_root_passes_when_hash_matches_in_tempdir ... ok
test bottom_white::cas::schema::tests::cid_differs_on_content ... ok
test bottom_white::cas::schema::tests::cid_deterministic ... ok
test bottom_white::cas::schema::tests::cid_from_empty_content ... ok
test bottom_white::cas::schema::tests::cid_display_format ... ok
test bottom_white::cas::schema::tests::metadata_canonical_hash_differs_on_object_type ... ok
test bottom_white::cas::schema::tests::metadata_canonical_hash_deterministic ... ok
test bottom_white::cas::store::tests::cid_is_content_address ... ok
test bottom_white::cas::store::tests::corrupted_sidecar_line_returns_parse_error ... ok
test bottom_white::cas::store::tests::cell_isolation_disjoint_cas ... ok
test bottom_white::cas::store::tests::empty_store_root ... ok
test bottom_white::cas::store::tests::get_nonexistent_returns_error ... ok
test bottom_white::cas::store::tests::idempotent_put_does_not_duplicate_sidecar_line ... ok
test bottom_white::cas::store::tests::each_new_put_appends_one_line ... ok
test bottom_white::cas::store::tests::metadata_recorded ... ok
test bottom_white::cas::store::tests::missing_sidecar_opens_fresh ... ok
test bottom_white::cas::store::tests::put_get_round_trip_small ... ok
test bottom_white::cas::store::tests::merkle_root_deterministic_two_runs ... ok
test bottom_white::cas::store::tests::put_idempotent_same_content ... ok
test bottom_white::cas::store::tests::put_get_round_trip_large ... ok
test bottom_white::cas::store::tests::reopen_recovers_index_and_get_works ... ok
test bottom_white::ledger::system_keypair::tests::authorized_scope_signing_round_trip ... ok
test bottom_white::ledger::transition_ledger::tests::append_is_pure_and_byte_stable ... ok
test bottom_white::ledger::transition_ledger::tests::canonical_codec_round_trip ... ok
test bottom_white::ledger::transition_ledger::tests::canonical_digest_excludes_derivatives ... ok
test bottom_white::ledger::transition_ledger::tests::canonical_digest_stable_across_clones ... ok
test bottom_white::ledger::transition_ledger::tests::git2_writer_append_and_read_back ... ok
test bottom_white::ledger::system_keypair::tests::terminal_scope_rotation_signing_round_trip ... ok
test bottom_white::ledger::transition_ledger::tests::git2_writer_empty_chain ... ok
test bottom_white::ledger::transition_ledger::tests::git2_writer_rejects_logical_t_gap ... ok
test bottom_white::ledger::transition_ledger::tests::in_memory_writer_enforces_logical_t ... ok
test bottom_white::ledger::transition_ledger::tests::git2_writer_reopen_recovers_chain ... ok
test bottom_white::ledger::transition_ledger::tests::replay_chain_integrity_clean ... ok
test bottom_white::ledger::transition_ledger::tests::replay_cas_payload_round_trip_after_reopen ... ok
test bottom_white::ledger::transition_ledger::tests::replay_full_transition_reaches_dispatch_then_stubs ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_bad_system_signature ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_payload_decode_failure ... ok
test bottom_white::ledger::transition_ledger::tests::sequencer_serial_replay_byte_identity ... ignored, CO1.7.5: requires real per-kind transition bodies
test bottom_white::ledger::transition_ledger::tests::replay_rejects_tx_kind_mismatch ... ok
test bottom_white::tools::registry::tests::duplicate_id_rejected ... ok
test bottom_white::tools::registry::tests::empty_id_rejected ... ok
test bottom_white::tools::registry::tests::empty_registry ... ok
test bottom_white::tools::registry::tests::find_by_capability_replaces_magic_string ... ok
test bottom_white::tools::registry::tests::merkle_root_deterministic ... ok
test bottom_white::tools::registry::tests::non_idempotent_rejected ... ok
test bottom_white::tools::registry::tests::register_and_get_round_trip ... ok
test bus::tests::test_bus_basic_append ... ok
test bus::tests::test_bus_classify_bounded ... ok
test bus::tests::test_bus_creates_market_on_append ... ok
test bus::tests::test_bus_forbidden_pattern_veto ... ok
test bus::tests::test_bus_graveyard_feedback ... ok
test bus::tests::test_bus_halt_and_settle ... ok
test bus::tests::test_bus_ledger_integrity ... ok
test bus::tests::test_bus_payload_too_long ... ok
test bus::tests::test_bus_serial_ordering ... ok
test bus::tests::test_bus_snapshot ... ok
test bus::tests::test_bus_too_many_lines ... ok
test bus::tests::test_bus_unknown_agent_vetoed ... ok
test drivers::llm_http::tests::test_client_creation ... ok
test drivers::llm_http::tests::test_driver_error_display ... ok
test drivers::llm_http::tests::test_generate_request_serialization ... ok
test economy::money::tests::checked_add_normal ... ok
test economy::money::tests::checked_add_overflow_returns_none ... ok
test economy::money::tests::checked_sub_normal ... ok
test economy::money::tests::conservation_law_basic ... ok
test economy::money::tests::display_positive ... ok
test economy::money::tests::display_zero ... ok
test economy::money::tests::from_coin_overflow_returns_none ... ok
test economy::money::tests::from_coin_round_trip ... ok
test economy::money::tests::from_micro_units_zero ... ok
test economy::money::tests::ordering_for_btreemap ... ok
test economy::money::tests::royalty_10_percent_rounds_down ... ok
test economy::money::tests::royalty_floor_dust ... ok
test economy::money::tests::royalty_rejects_negative ... ok
test economy::money::tests::royalty_rejects_weight_above_1 ... ok
test economy::money::tests::serde_round_trip_json ... ok
test economy::money::tests::serde_transparent_format ... ok
test kernel::tests::test_append_and_retrieve ... ok
test kernel::tests::test_golden_path_trace ... ok
test kernel::tests::test_market_lifecycle ... ok
test kernel::tests::test_market_ticker ... ok
test kernel::tests::test_no_duplicate_market ... ok
test kernel::tests::test_no_market_for_nonexistent_node ... ok
test kernel::tests::test_reject_dangling_citation ... ok
test kernel::tests::test_reject_duplicate ... ok
test kernel::tests::test_resolve_all_markets ... ok
test ledger::tests::test_ledger_append_and_verify ... ok
test ledger::tests::test_ledger_hash_chain_integrity ... ok
test ledger::tests::test_ledger_omega_vocabulary ... ok
test ledger::tests::test_ledger_sequence_monotonic ... ok
test ledger::tests::test_ledger_tamper_detection ... ok
test ledger::tests::test_tape_append_root_node ... ok
test ledger::tests::test_tape_append_with_valid_citation ... ok
test ledger::tests::test_tape_dag_branching ... ok
test ledger::tests::test_tape_empty ... ok
test boot::tests::verify_trust_root_passes_on_intact_repo ... ok
test ledger::tests::test_tape_reject_duplicate_id ... ok
test ledger::tests::test_tape_time_arrow_ordering ... ok
test ledger::tests::test_tape_trace_ancestors ... ok
test prediction_market::tests::test_assassin_profit ... ok
test ledger::tests::test_tape_reject_dangling_citation ... ok
test prediction_market::tests::test_buy_no_increases_no_price ... ok
test prediction_market::tests::test_buy_yes_increases_yes_price ... ok
test prediction_market::tests::test_constant_product_invariant ... ok
test prediction_market::tests::test_create_market ... ok
test prediction_market::tests::test_initial_price_is_50_50 ... ok
test prediction_market::tests::test_ctf_conservation_1_coin_1_yes_1_no ... ok
test prediction_market::tests::test_no_double_resolution ... ok
test prediction_market::tests::test_multiple_traders_price_discovery ... ok
test prediction_market::tests::test_pioneer_profit ... ok
test prediction_market::tests::test_no_trading_after_resolution ... ok
test prediction_market::tests::test_prices_sum_to_one ... ok
test prediction_market::tests::test_redeem_requires_resolution ... ok
test prediction_market::tests::test_reject_zero_or_negative_amounts ... ok
test sdk::actor::tests::test_boltzmann_never_returns_none_with_nodes ... ok
test sdk::actor::tests::test_boltzmann_returns_none_empty_tape ... ok
test sdk::actor::tests::test_frontier_detection_leaf ... ok
test sdk::actor::tests::test_frontier_detection_parent_with_child ... ok
test sdk::actor::tests::test_lineage_score_increases_with_depth ... ok
test sdk::error_abstraction::tests::classifier_version_is_stamped ... ok
test sdk::error_abstraction::tests::fixture_linarith_failed ... ok
test sdk::error_abstraction::tests::fixture_other_catchall ... ok
test sdk::actor::tests::test_boltzmann_diversity_not_deterministic ... ok
test sdk::error_abstraction::tests::fixture_rewrite_no_match ... ok
test sdk::error_abstraction::tests::fixture_simp_no_progress ... ok
test sdk::error_abstraction::tests::fixture_unexpected_token ... ok
test sdk::error_abstraction::tests::fixture_type_mismatch ... ok
test sdk::error_abstraction::tests::fixture_unsolved_goals ... ok
test sdk::error_abstraction::tests::fixture_unknown_constant ... ok
test sdk::prompt::tests::test_prompt_contains_no_example_values ... ok
test sdk::error_abstraction::tests::labels_are_unique_and_stable ... ok
test sdk::prompt::tests::test_prompt_includes_balance ... ok
test sdk::prompt::tests::test_prompt_surfaces_search_hits ... ok
test sdk::prompt::tests::test_prompt_surfaces_team_board ... ok
test sdk::prompt::tests::test_prompt_truncates_errors_to_3 ... ok
test sdk::prompt_guard::tests::test_case_insensitive_match - should panic ... ok
test sdk::prompt_guard::tests::test_empty_prompt_passes ... ok
test sdk::prompt_guard::tests::test_h_vpput_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_assignment_pattern_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_m_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_runtime_caught - should panic ... ok
test sdk::prompt_guard::tests::test_clean_prompt_passes ... ok
test sdk::prompt_guard::tests::test_pput_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_wbcg_caught - should panic ... ok
test sdk::protocol::tests::test_deduct_negative_amount_rejected ... ok
test sdk::prompt_guard::tests::test_pput_substring_in_larger_text - should panic ... ok
test sdk::protocol::tests::test_malformed_action_tag_rejected_not_fallback ... ok
test sdk::protocol::tests::test_parse_action_tag_valid ... ok
test sdk::protocol::tests::test_no_byte_repair_on_invalid_escape ... ok
test sdk::protocol::tests::test_parse_action_tag_with_think_block ... ok
test sdk::protocol::tests::test_parse_bare_json_fallback ... ok
test sdk::protocol::tests::test_parse_invalid_json_returns_error ... ok
test sdk::protocol::tests::test_parse_no_action_returns_error ... ok
test sdk::protocol::tests::test_parse_with_invest_action ... ok
test sdk::protocol::tests::test_strip_multiple_think_blocks ... ok
test sdk::protocol::tests::test_strip_think_blocks ... ok
test sdk::protocol::tests::test_strip_unclosed_think_block ... ok
test bottom_white::ledger::transition_ledger::tests::signature_round_trip_and_transplant_defense ... ok
test bottom_white::cas::store::tests::put_many_then_iterate_count ... ok
test sdk::sandbox::tests::test_sandbox_captures_stderr ... ok
test sdk::snapshot::tests::test_snapshot_balance_query ... ok
test sdk::tools::librarian::tests::test_board_post_append ... ok
test sdk::sandbox::tests::test_sandbox_echo_command ... ok
test sdk::tools::librarian::tests::test_build_compression_prompt ... ok
test sdk::tools::librarian::tests::test_compress_interval ... ok
test sdk::tools::librarian::tests::test_board_write_read_roundtrip ... ok
test sdk::tools::search::tests::test_sanitize_query ... ok
test sdk::tools::librarian::tests::test_zero_interval_never_compresses ... ok
test sdk::tools::search::tests::test_search_empty_query ... ok
test sdk::tools::search::tests::test_search_nonexistent_path ... ok
test sdk::tools::wallet::tests::test_append_is_free ... ok
test sdk::tools::wallet::tests::test_deduct_and_credit ... ok
test sdk::tools::wallet::tests::test_genesis_allocation ... ok
test sdk::tools::wallet::tests::test_insufficient_balance_rejected ... ok
test sdk::tools::wallet::tests::test_negative_deduct_rejected ... ok
test sdk::tools::wallet::tests::test_no_double_genesis ... ok
test sdk::tools::wallet::tests::test_portfolio_tracking ... ok
test sdk::tools::wallet::tests::test_query_unknown_key ... ok
test sdk::tools::wallet::tests::test_query_balance ... ok
test sdk::tools::wallet::tests::test_zero_deduct_rejected ... ok
test sdk::tools::wallet::tests::test_unknown_agent_vetoed ... ok
test state::q_state::tests::btreemap_insertion_order_independent_serialization ... ok
test state::q_state::tests::economic_state_has_nine_sub_fields ... ok
test state::q_state::tests::genesis_q_state_is_total_and_default ... ok
test state::q_state::tests::nine_field_count_via_serde_json ... ok
test sdk::sandbox::tests::test_sandbox_nonzero_exit ... ok
test state::sequencer::tests::dispatch_transition_stubs_all_variants ... ok
test state::q_state::tests::node_id_from_state_root_is_deterministic ... ok
test state::sequencer::tests::apply_one_stub_does_not_consume_logical_t ... ok
test state::sequencer::tests::submit_advances_submit_id_only ... ok
test state::sequencer::tests::submit_returns_queue_closed_after_rx_drop ... ok
test state::typed_tx::tests::golden_challenge_tx_digest ... ok
test state::typed_tx::tests::golden_finalize_reward_tx_digest ... ok
test state::typed_tx::tests::golden_task_expire_tx_digest ... ok
test state::typed_tx::tests::golden_reuse_tx_digest ... ok
test state::typed_tx::tests::golden_terminal_summary_tx_digest ... ok
test state::typed_tx::tests::golden_verify_tx_digest ... ok
test state::typed_tx::tests::has_submitter_partitioning ... ok
test state::typed_tx::tests::golden_work_tx_digest ... ok
test state::typed_tx::tests::signing_payload_domain_prefix_is_load_bearing ... ok
test state::typed_tx::tests::signing_payload_domains_are_distinct ... ok
test state::sequencer::tests::submit_returns_queue_full_on_saturation ... ok
test state::typed_tx::tests::signing_payload_excludes_signature ... ok
test state::typed_tx::tests::typed_tx_btree_permutation_independence ... ok
test state::typed_tx::tests::signing_payload_golden_digests ... ok
test state::typed_tx::tests::typed_tx_btreemap_permutation_independence ... ok
test state::typed_tx::tests::typed_tx_byte_stability_across_calls ... ok
test state::typed_tx::tests::typed_tx_kind_projection ... ok
test state::typed_tx::tests::typed_tx_default_round_trip ... ok
test state::typed_tx::tests::typed_tx_cross_variant_non_collision ... ok
test top_white::predicates::registry::tests::agent_visible_view_filters_private ... ok
test top_white::predicates::registry::tests::duplicate_id_rejected ... ok
test top_white::predicates::registry::tests::empty_id_rejected ... ok
test state::typed_tx::tests::typed_tx_round_trip_all_variants ... ok
test top_white::predicates::registry::tests::empty_registry_root_is_sha256_empty ... ok
test top_white::predicates::registry::tests::merkle_root_changes_on_register ... ok
test top_white::predicates::registry::tests::metadata_canonical_hash_deterministic ... ok
test top_white::predicates::registry::tests::metadata_canonical_hash_differs_on_visibility ... ok
test top_white::predicates::registry::tests::register_and_get_round_trip ... ok
test top_white::predicates::registry::tests::merkle_root_deterministic_two_runs ... ok
test top_white::predicates::visibility::tests::private_never_content_visible ... ok
test top_white::predicates::visibility::tests::commit_reveal_pre_reveal ... ok
test top_white::predicates::visibility::tests::public_always_visible ... ok
test top_white::predicates::visibility::tests::serde_round_trip_commit_reveal ... ok
test wal::tests::test_wal_replay_missing_file_is_empty ... ok
test top_white::predicates::visibility::tests::serde_round_trip_public ... ok
test state::typed_tx::tests::typed_tx_round_trip_100_inputs ... ok
test wal::tests::test_wal_skip_malformed_line ... ok
test wal::tests::test_wal_roundtrip_nodes_only ... ok
test wal::tests::test_wal_roundtrip_mixed ... ok
test sdk::sandbox::tests::test_sandbox_timeout_kills_process ... ok

test result: ok. 239 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 1.30s


exec
/bin/bash -lc "find tests -maxdepth 3 -type f -name '*.rs' -print | sort | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/conformance_stubs.rs
tests/economic_state_reconstruct.rs
tests/fc_alignment_conformance.rs
tests/four_element_mapping.rs
tests/q_state_reconstruct.rs
tests/reward_pull_conservation.rs
tests/six_axioms_alignment.rs
tests/system_keypair_generation.rs
tests/system_keypair_load_and_decrypt.rs
tests/system_keypair_rotation_proof.rs
tests/system_keypair_sign_only_from_runner.rs
tests/system_keypair_verify_correctness.rs
tests/wal_resume.rs
tests/walkthrough_inv3_conservation.rs

codex
# Codex CO1.7-extra Round-2 Audit

## Q1 Round-1 M1 closure (substrate gap)

Mostly closed. The split in spec lines 28-29 keeps D1 and CO P2.x substrate work out. D2 only writes `QState.head_t` as `q_state::NodeId` after commit (spec lines 61-74; source `QState.head_t` at `src/state/q_state.rs:311`, tuple `NodeId` at `:49`). No economic/predicate/tool execution method is needed.

D3 wiring calls existing `Sequencer::submit` and does not require transition bodies. CAS test uses only `CasStore::put`/`get` (`src/bottom_white/cas/store.rs:163`, `:199`). No FC1/FC2 substrate is smuggled, but D3’s Kernel placement is a separate layering problem, addressed below.

## Q2 Round-1 M2 closure (D1 purity)

Closed. Spec §4 explicitly defers D1 transition bodies and related mapping/rejection concerns (lines 269-272). The shipped dispatch still returns `NotYetImplemented` for all variants (`src/state/sequencer.rs:47-61`), so no residual D1 behavior is being specified for this atom.

## Q3 Round-1 M3 closure (compile defects)

M3a: closed. The spec uses `TuringBus`; residual `Bus` hits are explanatory, not wrong. Source confirms `pub struct TuringBus` at `src/bus.rs:53`.

M3b: partially closed. `#[serde(skip)]` is necessary and sufficient for Kernel serde because `Kernel` derives serde at `src/kernel.rs:18`. But `#[derive(Debug)]` on `Sequencer` is not sufficient: `Sequencer` contains `Arc<RwLock<dyn LedgerWriter>>` (`src/state/sequencer.rs:201`) and `Arc<Ed25519Keypair>` (`:199`), while `Ed25519Keypair` intentionally has no `Debug` derive (`src/bottom_white/ledger/system_keypair.rs:282-284`). Spec lines 163 and 167 should be changed to require a manual `Debug` impl, not derive-first fallback.

M3c: challenged. Kernel is explicitly “pure topology” (`src/kernel.rs:15-17`) and has a hard warning that the module must avoid domain-specific terms (`:5-6`). `Sequencer` owns CAS, keypair, registries, writer, and `QState` (`src/state/sequencer.rs:198-206`). Saying Kernel holds only the driver papers over real ownership/dependency coupling.

## Q4 Round-1 M4 closure (§ 0.4 reconciliation)

Filing a STATE v1.5 housekeeping issue is sufficient; drafting the full patch inline is not required for this atom.

The downstream supersession principle is acceptable within audited atom scope; I do not see a constitution amendment requirement. But spec line 49 is internally wrong for the NodeId supersession: `head_t = NodeId(commit_oid_hex)` takes effect in this atom at lines 72-73. Only SignalKind migration belongs fully to future CO1.7.5. Fix the wording.

## Q5 Round-1 M5 closure (Q1 synthesis) + Q1' Sequencer Debug

Challenge. “Mandatory override” is not enforced because the trait method has a default `None` (spec lines 95-103). Rust will allow future `LedgerWriter` impls to silently inherit it. Make `head_commit_oid_hex` a required trait method returning `Option<String>`; that preserves post-commit no-failure without silent inheritance.

The defensive test only catches Git2 returning `None` after commit (spec lines 247-259). It does not catch a missing Sequencer `q_w.head_t` assignment. Also, `canonical_test_entry(1)` is undefined in the integration-test scope (spec line 251); the existing helper is private inside module tests at `src/bottom_white/ledger/transition_ledger.rs:813`.

Manual `finish_non_exhaustive()` is safe and preferable. It leaks no keypair/QState/CAS contents and satisfies `Kernel.Debug` through `Arc<Sequencer>`.

## Q6 Atomicity claim refinement

Mostly correct but still needs sharper wording. Spec line 79 is correct only for writers returning `Some`. For `None`, the commit has succeeded but `head_t` is intentionally unchanged. Call it “post-commit non-failing best-effort head binding for non-Git writers,” not atomic head close for all writers.

## Q7 STEP_B functional-coupling argument

Challenge. Functional coupling is stronger than the previous advisory wording, but STEP_B Phase 0 asks whether a less-invasive alternative exists (`handover/ai-direct/STEP_B_PROTOCOL.md:11-17`). One exists: put `Option<Arc<Sequencer>>` on `TuringBus`, which already owns runtime orchestration and avoids Kernel serde/debug/topology debt. The spec must either move the field to TuringBus or explicitly defeat that alternative.

## Q8 New defects in v1

New defects:

- Test files under `tests/co1_7_extra/*.rs` (spec lines 212-217, 246) will not be auto-discovered by standard Cargo integration-test discovery unless a top-level harness or `Cargo.toml` target is added.
- The core D2 behavior is untested. If the implementation omits lines 72-73, the two proposed tests still pass.
- Existing Sequencer comments will become stale: they still say head_t mutation is deferred to CO1.7.5+ (`src/state/sequencer.rs:180-184`, `:359-361`).
- LoC estimate is optimistic once manual Debug, imports, a real test harness, fixture construction, and a D2 head_t test are included. Expect closer to 200-280 LoC.

## Q9 Implementation gating

After the above fixes, CO1.7-extra is implementable with no CO P2.x blocker. Current baseline passes: `cargo check --workspace` and `cargo test --workspace --lib` both pass; lib tests report `239 passed; 0 failed; 1 ignored`.

## **VERDICT**: CHALLENGE

## Top 3 must-fix

1. Resolve D3 placement: move Sequencer handle to `TuringBus`, or provide a concrete audited argument why Kernel must own it despite the less-invasive alternative.
2. Make `head_commit_oid_hex` a required trait method and add a test that actually fails if D2 does not update `q.head_t`.
3. Make the spec compile/run as written: manual `Sequencer` Debug, discoverable test harness/files, and a real `LedgerEntry` fixture instead of undefined `canonical_test_entry`.

## Conviction

High.
2026-04-29T03:13:17.048373Z ERROR codex_core::session: failed to record rollout items: thread 019dd736-08c2-7e42-9dba-ce6df583a777 not found
tokens used
158,872
# Codex CO1.7-extra Round-2 Audit

## Q1 Round-1 M1 closure (substrate gap)

Mostly closed. The split in spec lines 28-29 keeps D1 and CO P2.x substrate work out. D2 only writes `QState.head_t` as `q_state::NodeId` after commit (spec lines 61-74; source `QState.head_t` at `src/state/q_state.rs:311`, tuple `NodeId` at `:49`). No economic/predicate/tool execution method is needed.

D3 wiring calls existing `Sequencer::submit` and does not require transition bodies. CAS test uses only `CasStore::put`/`get` (`src/bottom_white/cas/store.rs:163`, `:199`). No FC1/FC2 substrate is smuggled, but D3’s Kernel placement is a separate layering problem, addressed below.

## Q2 Round-1 M2 closure (D1 purity)

Closed. Spec §4 explicitly defers D1 transition bodies and related mapping/rejection concerns (lines 269-272). The shipped dispatch still returns `NotYetImplemented` for all variants (`src/state/sequencer.rs:47-61`), so no residual D1 behavior is being specified for this atom.

## Q3 Round-1 M3 closure (compile defects)

M3a: closed. The spec uses `TuringBus`; residual `Bus` hits are explanatory, not wrong. Source confirms `pub struct TuringBus` at `src/bus.rs:53`.

M3b: partially closed. `#[serde(skip)]` is necessary and sufficient for Kernel serde because `Kernel` derives serde at `src/kernel.rs:18`. But `#[derive(Debug)]` on `Sequencer` is not sufficient: `Sequencer` contains `Arc<RwLock<dyn LedgerWriter>>` (`src/state/sequencer.rs:201`) and `Arc<Ed25519Keypair>` (`:199`), while `Ed25519Keypair` intentionally has no `Debug` derive (`src/bottom_white/ledger/system_keypair.rs:282-284`). Spec lines 163 and 167 should be changed to require a manual `Debug` impl, not derive-first fallback.

M3c: challenged. Kernel is explicitly “pure topology” (`src/kernel.rs:15-17`) and has a hard warning that the module must avoid domain-specific terms (`:5-6`). `Sequencer` owns CAS, keypair, registries, writer, and `QState` (`src/state/sequencer.rs:198-206`). Saying Kernel holds only the driver papers over real ownership/dependency coupling.

## Q4 Round-1 M4 closure (§ 0.4 reconciliation)

Filing a STATE v1.5 housekeeping issue is sufficient; drafting the full patch inline is not required for this atom.

The downstream supersession principle is acceptable within audited atom scope; I do not see a constitution amendment requirement. But spec line 49 is internally wrong for the NodeId supersession: `head_t = NodeId(commit_oid_hex)` takes effect in this atom at lines 72-73. Only SignalKind migration belongs fully to future CO1.7.5. Fix the wording.

## Q5 Round-1 M5 closure (Q1 synthesis) + Q1' Sequencer Debug

Challenge. “Mandatory override” is not enforced because the trait method has a default `None` (spec lines 95-103). Rust will allow future `LedgerWriter` impls to silently inherit it. Make `head_commit_oid_hex` a required trait method returning `Option<String>`; that preserves post-commit no-failure without silent inheritance.

The defensive test only catches Git2 returning `None` after commit (spec lines 247-259). It does not catch a missing Sequencer `q_w.head_t` assignment. Also, `canonical_test_entry(1)` is undefined in the integration-test scope (spec line 251); the existing helper is private inside module tests at `src/bottom_white/ledger/transition_ledger.rs:813`.

Manual `finish_non_exhaustive()` is safe and preferable. It leaks no keypair/QState/CAS contents and satisfies `Kernel.Debug` through `Arc<Sequencer>`.

## Q6 Atomicity claim refinement

Mostly correct but still needs sharper wording. Spec line 79 is correct only for writers returning `Some`. For `None`, the commit has succeeded but `head_t` is intentionally unchanged. Call it “post-commit non-failing best-effort head binding for non-Git writers,” not atomic head close for all writers.

## Q7 STEP_B functional-coupling argument

Challenge. Functional coupling is stronger than the previous advisory wording, but STEP_B Phase 0 asks whether a less-invasive alternative exists (`handover/ai-direct/STEP_B_PROTOCOL.md:11-17`). One exists: put `Option<Arc<Sequencer>>` on `TuringBus`, which already owns runtime orchestration and avoids Kernel serde/debug/topology debt. The spec must either move the field to TuringBus or explicitly defeat that alternative.

## Q8 New defects in v1

New defects:

- Test files under `tests/co1_7_extra/*.rs` (spec lines 212-217, 246) will not be auto-discovered by standard Cargo integration-test discovery unless a top-level harness or `Cargo.toml` target is added.
- The core D2 behavior is untested. If the implementation omits lines 72-73, the two proposed tests still pass.
- Existing Sequencer comments will become stale: they still say head_t mutation is deferred to CO1.7.5+ (`src/state/sequencer.rs:180-184`, `:359-361`).
- LoC estimate is optimistic once manual Debug, imports, a real test harness, fixture construction, and a D2 head_t test are included. Expect closer to 200-280 LoC.

## Q9 Implementation gating

After the above fixes, CO1.7-extra is implementable with no CO P2.x blocker. Current baseline passes: `cargo check --workspace` and `cargo test --workspace --lib` both pass; lib tests report `239 passed; 0 failed; 1 ignored`.

## **VERDICT**: CHALLENGE

## Top 3 must-fix

1. Resolve D3 placement: move Sequencer handle to `TuringBus`, or provide a concrete audited argument why Kernel must own it despite the less-invasive alternative.
2. Make `head_commit_oid_hex` a required trait method and add a test that actually fails if D2 does not update `q.head_t`.
3. Make the spec compile/run as written: manual `Sequencer` Debug, discoverable test harness/files, and a real `LedgerEntry` fixture instead of undefined `canonical_test_entry`.

## Conviction

High.
