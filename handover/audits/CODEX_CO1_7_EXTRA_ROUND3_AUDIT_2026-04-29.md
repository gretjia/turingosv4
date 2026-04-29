# Codex CO1.7-extra Round-3 Audit
**Date**: 2026-04-29
**Target**: spec v1.1 (post round-2 patches)
**HEAD**: a3952cf73e50b711d18eff55a6f426fb1bd82b2f
**Prompt size**: 276723 chars

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
session id: 019dd748-bfed-7dd1-961a-b559b987d39c
--------
user
# Codex Adversarial Audit — CO1.7-extra v1.1 (Round 3; post round-2 patches)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-3 (parallel).

**Mandate**: round 3 dual external audit on CO1.7-extra v1.1 — applied 10 patches (MF1-MF10) per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md`. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

**What changed since round-2**: ArchitectAI applied all 10 r2 must-fix items. v1.1 commits: `25564d7` (v1.1 patches + audit artifacts) + `a3952cf` (smoke 11/11 PASS footer + 2 minor refinements). Smoke verified at HEAD `a3952cf`.

Round-2 must-fix items disposition:
- **MF1** § 0.4 supersession-disposition table corrected (head_t enacted in CO1.7-extra D2; SignalKind migrates) — § 0.4 patched
- **MF2** D2 testability: `advance_head_t` helper extracted from apply_one stage 9 — § 1.1 + new test § 3.3
- **MF3** trait method `head_commit_oid_hex` becomes REQUIRED (no default impl) — § 1.2
- **MF4** Sequencer placement: TuringBus directly (not nested through Kernel); STEP_B becomes single-file — § 2.1 + § 2.2
- **MF5** test harness flat-named `tests/co1_7_extra_*.rs` — § 3
- **MF6** manual Sequencer Debug `finish_non_exhaustive()` (Ed25519Keypair has no Debug) — § 2.1
- **MF7** `entry_at` helper private; tests construct LedgerEntry inline — § 3.2 (Codex r2 misidentified name as `canonical_test_entry`; v1.1 corrected to `entry_at`)
- **MF8** stale Sequencer comments at sequencer.rs:178-184 + :357-361 (spec v1.1 corrected line cites from r2's 180-184/359-361) — listed in atom landing checklist § 1.1
- **MF9** atomicity wording refined — § 1.1
- **MF10** LoC estimate 150-230 → 200-280 — § 7

**Open Q count after v1.1**: 0. Q1 (round-1) and Q1' (round-1 surfaced) both closed.

## What you're reviewing

1. **Spec doc**: `handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md` v1.1 (~520 lines)
2. **Round-2 audit artifacts** (verify closure): `CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md`, `GEMINI_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md`, `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md`
3. **Frozen prior specs**: CO1.7 v1.2 + CO1.1.4-pre1
4. **Shipped code**: src/state/sequencer.rs + src/bottom_white/ledger/transition_ledger.rs + src/bus.rs + src/state/q_state.rs + src/bottom_white/cas/store.rs

## Round 3 audit questions

**Q1. MF1 § 0.4 disposition table closure**: spec § 0.4 now has explicit table — head_t enacted HERE (D2); SignalKind migrates to future CO1.7.5. Verify:
- Is the table content factually correct?
- Does the principle assertion ("downstream supersedes upstream within layered boundary") still hold post-correction?
- Any residual drift?

**Q2. MF2 advance_head_t helper extraction**: spec § 1.1 introduces `advance_head_t(q, writer)` helper called from apply_one stage 9. Verify:
- Helper signature is correct + testable in isolation
- apply_one stage 9 logic preserved byte-identically (pure refactor, zero behavior change)
- New test § 3.3 actually exercises the D2 code path (calls `advance_head_t` directly with mock writer)
- Mock LedgerWriter implementation in test correctly tests both Some and None paths

**Q3. MF3 required trait method**: spec § 1.2 makes `head_commit_oid_hex` required (no default). Verify:
- The trait definition is syntactically correct
- Both `Git2LedgerWriter` and `InMemoryLedgerWriter` implementations explicitly declare
- The "compiler enforces" claim holds (a missing impl would be E0046)
- Both safety arguments (silent stagnation prevention + no-panic) actually satisfied

**Q4. MF4 Sequencer placement (TuringBus, not Kernel)**: § 2.1 + § 2.2 rewritten. Verify:
- TuringBus (NOT Bus, NOT Kernel) gets the field + constructor + forwarder
- Kernel UNTOUCHED — does the spec consistently state this? Is "pure topology" doctrine preserved?
- STEP_B is now single-file (bus.rs only) — does § 2.2 ceremony procedure match? Is the "less invasive alternative" framing correct?
- Any residual references to Kernel.sequencer that should now be TuringBus.sequencer?

**Q5. MF5 + MF6 + MF7 + MF8 + MF9 + MF10 small fixes**: verify each:
- MF5: test paths use flat naming `tests/co1_7_extra_*.rs` (§ 3.1, § 3.2, § 3.3 file paths)
- MF6: manual Sequencer Debug with `finish_non_exhaustive()` (§ 2.1) — uses correct method
- MF7: `entry_at` helper at line 813 (Codex r2 misidentification corrected)
- MF8: stale comment line cites match real source (sequencer.rs:178-184 + :357-361)
- MF9: atomicity wording uses "non-failing best-effort head binding" / "explicit no-op preservation"
- MF10: LoC estimate 200-280 in § 7

**Q6. New defects in v1.1**: did the patches introduce any new issues?
- Any internal contradictions between sections (e.g., § 0.4 table vs § 1.1 helper code)?
- Implementation-blocking ambiguities in the new helper signature or test files?
- Anything in v1.1 that compiles in spec but won't compile when implemented?
- LoC estimate 200-280 still defensible given the new test mock?

**Q7. Implementation gating**: assuming v1.1 reaches PASS, is CO1.7-extra implementable end-to-end with no further blockers?

## Output format

# Codex CO1.7-extra Round-3 Audit
## Q1 MF1 § 0.4 disposition table closure
## Q2 MF2 advance_head_t helper extraction
## Q3 MF3 required trait method
## Q4 MF4 Sequencer placement (TuringBus)
## Q5 MF5-MF10 small fixes
## Q6 New defects in v1.1
## Q7 Implementation gating
## **VERDICT**: PASS / CHALLENGE / VETO
## Top issues (if CHALLENGE)
## Conviction (low/med/high)

Be rigorous. Cite spec line numbers + source file line numbers when calling defects. Do NOT pass on principle; do NOT veto on principle. Real defects = CHALLENGE; foundational design flaw = VETO; clean spec = PASS.

---


# CO1.7-extra v1.1 (target of audit)

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

**Smoke gate**: 11 / 11 PASS at HEAD `25564d7`. Spec v1.1 ready for round-3 dual external audit.

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
- **MF10** LoC estimate: 150-230 → 200-280 (manual Debug + helper extraction + 3rd test + harness adjustments) (§ 7)

### Awaiting

1. round-3 dual external audit on CO1.7-extra v1.1
2. expected PASS/PASS (small atom, all r2 issues addressed systematically)
3. then CO1.7-extra-impl (D2 helper extraction + apply_one patch + trait method + TuringBus single-file STEP_B + 3 tests + stale-comment update)
4. file STATE_TRANSITION_SPEC v1.5 housekeeping issue per § 0.4 commitment
5. spec future CO1.7.5 (transition bodies; gated on CO P2.x substrate atoms)


---

# XREF: round-2 merged verdict (the document driving v1.1 patches)

# CO1.7-extra Dual External Audit — Round-2 Merged Verdict

**Date**: 2026-04-29
**Target**: `CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md` at HEAD `617f01e`
**Audits**: Codex r2 (`CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md`) + Gemini r2 (`GEMINI_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md`)

## Verdict matrix

| Audit | Verdict | Conviction |
|---|---|---|
| Codex | **CHALLENGE** | High |
| Gemini | **CHALLENGE** | High |

**Conservative-merged verdict**: **CHALLENGE / High**. No VETO. Both audits explicitly note "no foundational design flaw"; v1 issues are "readily correctable" (Gemini) / "implementable after fixes" (Codex).

## Round-1 must-fix closure status (per round-2 review)

| R1 MF | Audit-2 status |
|---|---|
| M1 substrate gap | ✅ closed by scope split (both audits PASS Q1) |
| M2 D1 purity | ✅ closed by scope split (Codex Q2 PASS) |
| M3a TuringBus rename | ✅ closed (Codex Q3 PASS) |
| M3b Kernel serde-skip | ✅ partial — serde-skip sufficient; **but** Sequencer Debug derive insufficient (Codex Q3 — Ed25519Keypair has no Debug derive at `src/bottom_white/ledger/system_keypair.rs:282-284`; blanket derive fails) |
| M3c Sequencer placement | ❌ **rejected by Codex (Q3 + Q7)**; "papers over real ownership/dependency coupling"; Kernel `src/kernel.rs:5-6` has hard warning re: domain-specific terms; less-invasive alternative exists (TuringBus) |
| M4 § 0.4 active reconciliation | ⚠️ partial — principle assertion accepted; commitment language directionally correct; **but** § 0.4 contains **factual contradiction** about supersession migration (head_t enacted HERE, not migrated) |
| M5 Q1 head_commit_oid_hex synthesis | ❌ **rejected by both audits**; default-None is convention not compiler-enforced; both r2 verdicts converge on **third option** (no default) |

## Round-2 new must-fix items

### MF1 — § 0.4 factual contradiction (both audits agree)

§ 0.4 states "two STATE supersessions migrate intact to future CO1.7.5", but D2 (`q_w.head_t = state::q_state::NodeId(commit_oid_hex)` at spec lines 72-73) **enacts** the head_t supersession HERE in CO1.7-extra. Only the SignalKind supersession migrates to future CO1.7.5.

**Fix**: amend § 0.4 to clearly state "head_t supersession enacted in CO1.7-extra D2; SignalKind supersession migrates to future CO1.7.5".

### MF2 — D2 code path test gap (both audits agree)

Neither `cas_payload_round_trip` nor `git2_writer_returns_some_after_commit` exercises the new D2 code in `Sequencer::apply_one` stage 9 (the `q_w.head_t = NodeId(...)` assignment). If implementation omits these lines, both proposed tests still pass.

**Fix**: add a test that asserts `q.head_t` is correctly updated post-commit. Implementation challenge: dispatch_transition currently returns `Err(NotYetImplemented)` for all variants, blocking apply_one from reaching stage 9. **Resolution adopted (per ArchitectAI; in v1.1)**: extract D2 logic into helper `advance_head_t(q: &mut QState, writer: &dyn LedgerWriter)` callable from apply_one stage 9 AND directly testable via mock writer. The extraction adds zero behavior change but makes D2 unit-testable without injection of dispatch_transition. Test file: `tests/co1_7_extra/head_t_advancement.rs` (or flat-named `tests/co1_7_extra_head_t_advancement.rs` per Cargo discovery; resolved in MF5 below).

### MF3 — `head_commit_oid_hex` trait method becomes required (both audits converge to 3rd option)

Round-1: Gemini voted `unimplemented!()`; Codex voted `default { None }` + override + test. v1 synthesized to default-None + mandatory-override-by-convention + defensive test. Round-2 both audits **reject** this synthesis as fragile (mandate is convention, not compiler-enforced).

Both r2 audits converge on **third option**: remove the default impl entirely. Rust compiler then enforces every `LedgerWriter` impl declares `head_commit_oid_hex` (Gemini Q3 cleanest; Codex Q5 also).

**Fix**: amend § 1.2 trait definition:
```rust
pub trait LedgerWriter: Send + Sync {
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
    fn len(&self) -> u64;
    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
    fn head_commit_oid_hex(&self) -> Option<String>;  // REQUIRED — no default
}
```

Both `Git2LedgerWriter` and `InMemoryLedgerWriter` (and any future impl) must explicitly declare. Compiler-enforced. Both audits' safety arguments (silent stagnation prevention + post-commit no-panic) satisfied.

### MF4 — Sequencer placement: TuringBus, not Kernel (Codex Q-3 + Q-7)

Codex Q-7: STEP_B Phase 0 asks for less-invasive alternative; TuringBus already owns runtime orchestration and avoids Kernel serde/debug/topology debt. Codex Q-3 (M3c): saying Kernel "holds the driver, not the data" papers over Sequencer's heavy ownership of CAS + keypair + registries + writer + QState. Kernel `src/kernel.rs:5-6` explicit warning against domain-specific terms.

Gemini Q5 r2 framing: Kernel placement is "pragmatic but architecturally compromising"; argues for separate runtime layer in stricter cases. The TuringBus alternative addresses Gemini's concern directly.

**Fix**: rewrite § 2.1 + § 2.2 + § 2.3:
- TuringBus gets `Option<Arc<Sequencer>>` field + `with_sequencer` constructor + `submit_typed_tx` forwarder
- Kernel UNTOUCHED (preserves "pure topology" doctrine)
- STEP_B becomes single-file ceremony on `src/bus.rs` only
- Combined-ceremony justification (functional coupling) no longer needed; replaced with simpler "single restricted-file change" rationale

**Architectural side-benefit**: cleaner layering. TuringBus owns Kernel + Sequencer as peers. Kernel stays at pure-topology layer. Sequencer at runtime-orchestration layer. Single-file STEP_B ceremony is simpler than combined.

## Round-2 smaller findings (to be patched in v1.1)

| ID | Source | Fix |
|---|---|---|
| **MF5** test harness | Codex Q-8 | `tests/co1_7_extra/*.rs` not auto-discovered by Cargo without `tests/co1_7_extra/main.rs` harness OR flat-named `tests/co1_7_extra_*.rs`. Spec § 3 chooses **flat-named** (simplest; aligns with existing convention in `tests/`). |
| **MF6** Sequencer Debug impl | Codex Q-3 (M3b refinement) | Manual `impl Debug for Sequencer` using `f.debug_struct("Sequencer").finish_non_exhaustive()`. Cannot use `#[derive(Debug)]` because Sequencer holds `Arc<Ed25519Keypair>` and `Ed25519Keypair` intentionally has no Debug derive (system_keypair.rs:282-284). Spec § 2.1 patched. |
| **MF7** canonical_test_entry private | Codex Q-5 | Helper at `transition_ledger.rs:813` is private to module tests. v1.1 either makes a small `pub(crate) fn canonical_test_entry()` helper, OR the new test inlines `LedgerEntry { ... }` construction. Spec § 3.2 chooses inline construction (test doesn't need cross-module reuse). |
| **MF8** stale Sequencer comments | Codex Q-8 | sequencer.rs:180-184 + :359-361 say "head_t mutation deferred to CO1.7.5+". CO1.7-extra implementation must update these comments to reflect "head_t closed by CO1.7-extra D2". Spec § 1.1 adds to atom landing checklist. |
| **MF9** atomicity wording | Codex Q-6 | "post-commit non-failing best-effort head binding (Some path; Git2)" + "explicit no-op preservation (None path; InMemory)". Drop "atomic head close for all writers" framing. § 1.1 patched. |
| **MF10** LoC estimate | Codex Q-8 | 150-230 → 200-280 LoC (manual Debug impl + test harness + helper extraction + D2 head_t test add overhead). § 7 patched. |

## Where the audits agree (for the record)

- ✅ Scope split is constitutionally sound (Gemini Q1 + Codex Q1)
- ✅ Round-1 substrate gap MF closed (Gemini Q1 + Codex Q1)
- ✅ Round-1 D1 purity MF closed (Gemini implicit + Codex Q2)
- ✅ Round-1 § 0.4 process commitment principle (downstream supersedes upstream) is within ArchitectAI authority (Gemini Q2 + Codex Q4)
- ✅ STEP_B functional-coupling argument is stronger than minimum-sufficient-version invocation (Gemini Q4 PASS strong; Codex Q7 acknowledges-but-still-CHALLENGE because less-invasive alternative exists)
- ✅ Forward sustainability preserved (Gemini Q7 + Codex Q1)

## Where the audits disagree (none significantly)

Both round-2 verdicts converged on the same MF set with high agreement. Disagreements are at the level of severity weighting and small wording details, not fundamental directions. Codex performed deeper source-level verification (Sequencer field types, Ed25519Keypair Debug, Cargo test discovery); Gemini provided cleaner architectural framing (third-option trait design, scope-split soundness).

## Conservative-merged decision (no further audit input needed)

ArchitectAI applies all 10 patches (MF1-MF10) directly to v1 spec → v1.1. Per "无损压缩即智能", the patches are systematic application of audit findings; no architectural decision points remain ambiguous.

Round-3 audit budget after v1.1: ~$5-10 (1 round expected to PASS/PASS — small focused atom; r2 issues are correctable; no new architectural surface introduced).

## Audit cost summary

- Codex r2: 158,872 tokens
- Gemini r2: prompt=130,878 / candidates=2,968 / total=137,133 tokens
- Estimated round cost: ~$6-12
- Cumulative project audit spend: ~$189-300 / $890 mid-budget (~21-34%)

## Status going forward

1. **CO1.7-extra v1.1**: spec patched in place this session; awaiting round-3 dual audit
2. **CO1.7.5 (transition bodies)**: future atom (unchanged from r1 verdict)
3. **LATEST.md**: should be patched to reflect Wave 6 #1 ~30-40% diagnosis (per r1 verdict, reconfirmed in r2 Q1)
4. **PROJECT_DECISION_MAP**: should track CO1.7-extra as new bridge atom (was added to task #5 but should be sedimented in the canonical decision-map doc)


---

# XREF: CO1.7 v1.2 spec (frozen, round-3 PASS/PASS)

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

# XREF: shipped src/state/q_state.rs (NodeId + Q_t)

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

# XREF: shipped src/bottom_white/ledger/system_keypair.rs (Ed25519Keypair derives — for MF6 verification)

```rust
//! Runtime system keypair lifecycle per
//! `handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md`.
//!
//! The private key is generated from OS entropy via `getrandom(2)`, encrypted
//! at rest with ChaCha20-Poly1305, and protected by Argon2id using RFC 9106 /
//! OWASP-class defaults: m=64 MiB, t=3, p=4. The KDF parameters are read from
//! environment variables so deployments can ratchet cost without code churn.
//!
//! /// TRACE_MATRIX FC1-Sig+FC3-Sig: runtime attribution signature primitive

use crate::boot::TrustRootError;
use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Nonce};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::env;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use zeroize::{Zeroize, ZeroizeOnDrop};

const DEFAULT_KDF_MEMORY_KIB: u32 = 65_536;
const DEFAULT_KDF_ITER: u32 = 3;
const DEFAULT_KDF_LANES: u32 = 4;
const DERIVED_KEY_LEN: usize = 32;
const SECRET_KEY_LEN: usize = 32;
const PUBLIC_KEY_LEN: usize = 32;
const SIGNATURE_LEN: usize = 64;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const PLAINTEXT_LEN: usize = SECRET_KEY_LEN + PUBLIC_KEY_LEN;
const FORMAT_MAGIC: &[u8; 11] = b"TOS4SYSKEY1";
const FORMAT_VERSION: u8 = 1;

/// TRACE_MATRIX FC1-Sig+FC3-Sig: system signature epoch identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct SystemEpoch(u64);

impl SystemEpoch {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a system signature epoch.
    pub const fn new(epoch: u64) -> Self {
        Self(epoch)
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: expose the numeric epoch for canonical encoding.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl From<u64> for SystemEpoch {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: ed25519 public key pinned by epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemPublicKey([u8; PUBLIC_KEY_LEN]);

impl SystemPublicKey {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a system public key from raw ed25519 bytes.
    pub const fn from_bytes(bytes: [u8; PUBLIC_KEY_LEN]) -> Self {
        Self(bytes)
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: expose raw public key bytes for pinning and verification.
    pub const fn as_bytes(&self) -> &[u8; PUBLIC_KEY_LEN] {
        &self.0
    }

    /// TRACE_MATRIX FC3-Sig: stable SHA-256 fingerprint for audit logs and rotation records.
    pub fn fingerprint_sha256(&self) -> [u8; 32] {
        Sha256::digest(self.0).into()
    }
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: ed25519 detached signature over a canonical system message digest.
///
/// `[u8; 64]` serde via `serde_bytes_64` (serde-derive default doesn't support
/// arrays > 32). With `bincode` + `fixed_int_encoding` this writes 64 raw bytes —
/// deterministic, platform-stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemSignature(#[serde(with = "serde_bytes_64")] [u8; SIGNATURE_LEN]);

impl Default for SystemSignature {
    fn default() -> Self {
        Self([0u8; SIGNATURE_LEN])
    }
}

/// Serde adapter for `[u8; 64]`: serializes as a length-64 byte sequence
/// (deterministic under bincode `fixed_int_encoding` → 64 raw bytes; no length prefix
/// because the ARRAY type encodes its length statically).
///
/// `pub(crate)` so other in-crate types with `[u8; 64]` fields (e.g.
/// `state::typed_tx::AgentSignature`) can reuse the same adapter — keeps the
/// serde wire format byte-identical across all 64-byte signature types.
pub(crate) mod serde_bytes_64 {
    use serde::de::{SeqAccess, Visitor};
    use serde::ser::SerializeTuple;
    use serde::{Deserializer, Serializer};
    use std::fmt;

    pub fn serialize<S: Serializer>(bytes: &[u8; 64], s: S) -> Result<S::Ok, S::Error> {
        let mut tup = s.serialize_tuple(64)?;
        for b in bytes {
            tup.serialize_element(b)?;
        }
        tup.end()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 64], D::Error> {
        struct ArrVisitor;
        impl<'de> Visitor<'de> for ArrVisitor {
            type Value = [u8; 64];
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "byte array of length 64")
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut out = [0u8; 64];
                for (i, slot) in out.iter_mut().enumerate() {
                    *slot = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;
                }
                Ok(out)
            }
        }
        d.deserialize_tuple(64, ArrVisitor)
    }
}

impl SystemSignature {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a detached system signature from raw ed25519 bytes.
    pub const fn from_bytes(bytes: [u8; SIGNATURE_LEN]) -> Self {
        Self(bytes)
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: expose raw signature bytes for tape serialization.
    pub const fn as_bytes(&self) -> &[u8; SIGNATURE_LEN] {
        &self.0
    }
}

/// TRACE_MATRIX FC1-Sig: typed rejection summary stamped by the predicate runner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RejectedAttemptSummary {
    run_id: String,
    attempt_id: String,
    failure_class: String,
    summary_hash: [u8; 32],
}

impl RejectedAttemptSummary {
    /// TRACE_MATRIX FC1-Sig: construct a typed rejected-attempt summary, never a free-form sign blob.
    pub fn new(
        run_id: impl Into<String>,
        attempt_id: impl Into<String>,
        failure_class: impl Into<String>,
        summary_hash: [u8; 32],
    ) -> Self {
        Self {
            run_id: run_id.into(),
            attempt_id: attempt_id.into(),
            failure_class: failure_class.into(),
            summary_hash,
        }
    }
}

// TRACE_MATRIX CO1.1.4-pre1 v1.1 round-1 closure (C-3 / Codex Q-C):
// the typed `TerminalSummaryTx` struct (8-field per STATE § 1.5) now lives in
// `state::typed_tx`. system_keypair signs an opaque digest via the
// `CanonicalMessage::TerminalSummarySigning([u8; 32])` variant — same
// opaque-digest pattern as `LedgerEntrySigning`, avoiding `bottom_white ↔ state`
// circular dependency.

/// TRACE_MATRIX FC3-Sig: typed continuity statement for system key rotation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpochRotationProof {
    old_epoch: SystemEpoch,
    new_epoch: SystemEpoch,
    old_pubkey: SystemPublicKey,
    new_pubkey: SystemPublicKey,
    signed_at_unix: u64,
}

impl EpochRotationProof {
    /// TRACE_MATRIX FC3-Sig: construct a typed epoch-rotation continuity proof.
    pub const fn new(
        old_epoch: SystemEpoch,
        new_epoch: SystemEpoch,
        old_pubkey: SystemPublicKey,
        new_pubkey: SystemPublicKey,
        signed_at_unix: u64,
    ) -> Self {
        Self {
            old_epoch,
            new_epoch,
            old_pubkey,
            new_pubkey,
            signed_at_unix,
        }
    }

    /// TRACE_MATRIX FC3-Sig: old signing epoch certified by the rotation proof.
    pub const fn old_epoch(&self) -> SystemEpoch {
        self.old_epoch
    }

    /// TRACE_MATRIX FC3-Sig: new signing epoch certified by the rotation proof.
    pub const fn new_epoch(&self) -> SystemEpoch {
        self.new_epoch
    }
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: only typed runtime messages may enter signature verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalMessage {
    /// TRACE_MATRIX FC1-Sig: predicate-runner rejection summary.
    RejectedAttemptSummary(RejectedAttemptSummary),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.1 closure C-3): terminal
    /// summary signing-payload digest. Opaque `[u8; 32]` produced by
    /// `state::typed_tx::TerminalSummarySigningPayload::canonical_digest()`
    /// (a/k/a `TerminalSummaryTx::to_signing_payload().canonical_digest()`);
    /// this variant only carries the 32-byte digest into the typed sign API.
    /// Same opaque-digest pattern as `LedgerEntrySigning`; avoids a circular
    /// `system_keypair ↔ state` module dependency.
    TerminalSummarySigning([u8; 32]),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): finalize
    /// reward signing-payload digest. Opaque `[u8; 32]` produced by
    /// `state::typed_tx::FinalizeRewardSigningPayload::canonical_digest()`.
    FinalizeRewardSigning([u8; 32]),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): task
    /// expire signing-payload digest. Opaque `[u8; 32]` produced by
    /// `state::typed_tx::TaskExpireSigningPayload::canonical_digest()`.
    TaskExpireSigning([u8; 32]),
    /// TRACE_MATRIX FC3-Sig: system key epoch continuity proof.
    EpochRotationProof(EpochRotationProof),
    /// TRACE_MATRIX FC2-Append (CO1.7 v1.2 round-2 closure C3): L4 transition_ledger
    /// signing payload digest. Opaque [u8; 32] — full canonical_digest of
    /// `LedgerEntrySigningPayload` is computed in `transition_ledger`; this variant
    /// only carries the 32-byte digest into the typed sign API. Avoids a circular
    /// `system_keypair ↔ transition_ledger` module dependency while preserving the
    /// "all sign goes through CanonicalMessage" invariant.
    LedgerEntrySigning([u8; 32]),
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: epoch-indexed public keys pinned by genesis and rotation history.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PinnedSystemPubkeys {
    keys: BTreeMap<SystemEpoch, SystemPublicKey>,
}

impl PinnedSystemPubkeys {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: create an empty pinned system-key map.
    pub fn new() -> Self {
        Self::default()
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: pin a public key for a system epoch.
    pub fn insert(
        &mut self,
        epoch: SystemEpoch,
        public_key: SystemPublicKey,
    ) -> Option<SystemPublicKey> {
        self.keys.insert(epoch, public_key)
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: fetch the public key pinned for a system epoch.
    pub fn get(&self, epoch: SystemEpoch) -> Option<&SystemPublicKey> {
        self.keys.get(&epoch)
    }
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: in-memory ed25519 system keypair with zeroized private key on drop.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Ed25519Keypair {
    secret_key: Box<[u8]>,
    #[zeroize(skip)]
    public_key: SystemPublicKey,
}

impl Ed25519Keypair {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: generate ed25519 key material from `getrandom(2)` entropy.
    pub fn generate_with_secure_entropy() -> Result<Self, KeypairError> {
        let mut seed = [0u8; SECRET_KEY_LEN];
        getrandom::getrandom(&mut seed).map_err(KeypairError::Entropy)?;
        let signing_key = SigningKey::from_bytes(&seed);
        let public_key = SystemPublicKey::from_bytes(signing_key.verifying_key().to_bytes());
        let mut keypair = Self {
            secret_key: Vec::from(seed).into_boxed_slice(),
            public_key,
        };
        seed.zeroize();
        keypair.mlock_private_key_best_effort();
        Ok(keypair)
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: return the public half of the system keypair.
    pub const fn public_key(&self) -> SystemPublicKey {
        self.public_key
    }

    fn from_plaintext(plaintext: &[u8]) -> Result<Self, KeypairError> {
        if plaintext.len() != PLAINTEXT_LEN {
            return Err(KeypairError::InvalidFormat("bad plaintext length"));
        }
        let mut secret = [0u8; SECRET_KEY_LEN];
        secret.copy_from_slice(&plaintext[..SECRET_KEY_LEN]);
        let mut public = [0u8; PUBLIC_KEY_LEN];
        public.copy_from_slice(&plaintext[SECRET_KEY_LEN..]);

        let signing_key = SigningKey::from_bytes(&secret);
        if signing_key.verifying_key().to_bytes() != public {
            secret.zeroize();
            return Err(KeypairError::InvalidFormat(
                "public key does not match private key",
            ));
        }

        let mut keypair = Self {
            secret_key: Vec::from(secret).into_boxed_slice(),
            public_key: SystemPublicKey::from_bytes(public),
        };
        secret.zeroize();
        keypair.mlock_private_key_best_effort();
        Ok(keypair)
    }

    fn to_plaintext(&self) -> Result<[u8; PLAINTEXT_LEN], KeypairError> {
        let secret = self.secret_slice()?;
        let mut plaintext = [0u8; PLAINTEXT_LEN];
        plaintext[..SECRET_KEY_LEN].copy_from_slice(secret);
        plaintext[SECRET_KEY_LEN..].copy_from_slice(self.public_key.as_bytes());
        Ok(plaintext)
    }

    fn sign_digest(&self, digest: [u8; 32]) -> Result<SystemSignature, KeypairError> {
        let mut secret = [0u8; SECRET_KEY_LEN];
        secret.copy_from_slice(self.secret_slice()?);
        let signing_key = SigningKey::from_bytes(&secret);
        let signature = signing_key.sign(&digest);
        secret.zeroize();
        Ok(SystemSignature::from_bytes(signature.to_bytes()))
    }

    fn secret_slice(&self) -> Result<&[u8], KeypairError> {
        if self.secret_key.len() == SECRET_KEY_LEN {
            Ok(&self.secret_key)
        } else {
            Err(KeypairError::InvalidFormat("bad in-memory secret length"))
        }
    }

    fn mlock_private_key_best_effort(&mut self) -> bool {
        mlock_best_effort(self.secret_key.as_ptr(), self.secret_key.len())
    }
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: system keypair lifecycle and crypto error taxonomy.
#[derive(Debug)]
pub enum KeypairError {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: filesystem operation failed.
    Io(std::io::Error),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: secure operating-system entropy failed.
    Entropy(getrandom::Error),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: KDF environment parameter was absent or invalid.
    KdfParam(String),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: Argon2id key derivation failed.
    Kdf(argon2::Error),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: ChaCha20-Poly1305 encryption or authentication failed.
    Crypto(&'static str),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: encrypted keystore format was malformed.
    InvalidFormat(&'static str),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: default keystore path could not be resolved.
    HomeUnavailable,
}

impl fmt::Display for KeypairError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "system keypair I/O failed: {err}"),
            Self::Entropy(err) => write!(f, "system keypair entropy failed: {err}"),
            Self::KdfParam(msg) => write!(f, "system keypair KDF parameter invalid: {msg}"),
            Self::Kdf(err) => write!(f, "system keypair KDF failed: {err}"),
            Self::Crypto(msg) => write!(f, "system keypair crypto failed: {msg}"),
            Self::InvalidFormat(msg) => write!(f, "system keypair keystore invalid: {msg}"),
            Self::HomeUnavailable => {
                write!(f, "system keypair default keystore path requires HOME")
            }
        }
    }
}

impl std::error::Error for KeypairError {}

impl From<std::io::Error> for KeypairError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: resolve `~/.turingos/keystore/system_keypair_v{epoch}.enc`.
///
/// `TURINGOS_KEYSTORE_PATH` overrides the default path. The default never
/// points into the repository, CAS, or ledger directories.
pub fn default_system_keystore_path(epoch: SystemEpoch) -> Result<PathBuf, KeypairError> {
    if let Ok(path) = env::var("TURINGOS_KEYSTORE_PATH") {
        return Ok(PathBuf::from(path));
    }
    let home = env::var("HOME").map_err(|_| KeypairError::HomeUnavailable)?;
    Ok(PathBuf::from(home)
        .join(".turingos")
        .join("keystore")
        .join(format!("system_keypair_v{}.enc", epoch.get())))
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: first-boot generate-or-second-boot decrypt lifecycle entrypoint.
pub fn generate_or_load_system_keypair(
    keystore_path: &Path,
    user_kdf_password: &SecretString,
) -> Result<Ed25519Keypair, KeypairError> {
    if keystore_path.exists() {
        return load_existing_keypair(keystore_path, user_kdf_password);
    }

    let keypair = Ed25519Keypair::generate_with_secure_entropy()?;
    let encrypted = encrypt_at_rest(&keypair, user_kdf_password)?;
    write_keystore_0600(keystore_path, &encrypted)?;
    Ok(keypair)
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: decrypt an existing encrypted system keypair keystore.
pub fn load_existing_keypair(
    keystore_path: &Path,
    user_kdf_password: &SecretString,
) -> Result<Ed25519Keypair, KeypairError> {
    let bytes = fs::read(keystore_path)?;
    let encoded = EncryptedKeypair::decode(&bytes)?;
    let mut key = derive_key(user_kdf_password, &encoded.salt, encoded.kdf)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&key)
        .map_err(|_| KeypairError::Crypto("bad cipher key"))?;
    let plaintext = cipher
        .decrypt(
            Nonce::from_slice(&encoded.nonce),
            encoded.ciphertext.as_ref(),
        )
        .map_err(|_| KeypairError::Crypto("keystore authentication failed"))?;
    key.zeroize();
    Ed25519Keypair::from_plaintext(&plaintext)
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: canonical SHA-256 digest for typed system messages.
pub fn canonical_digest(message: &CanonicalMessage) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"turingosv4.system_keypair.v1");
    match message {
        CanonicalMessage::RejectedAttemptSummary(summary) => {
            h.update(b"RejectedAttemptSummary");
            update_len_prefixed(&mut h, summary.run_id.as_bytes());
            update_len_prefixed(&mut h, summary.attempt_id.as_bytes());
            update_len_prefixed(&mut h, summary.failure_class.as_bytes());
            h.update(summary.summary_hash);
        }
        CanonicalMessage::TerminalSummarySigning(digest) => {
            h.update(b"TerminalSummarySigning");
            h.update(digest);
        }
        CanonicalMessage::FinalizeRewardSigning(digest) => {
            h.update(b"FinalizeRewardSigning");
            h.update(digest);
        }
        CanonicalMessage::TaskExpireSigning(digest) => {
            h.update(b"TaskExpireSigning");
            h.update(digest);
        }
        CanonicalMessage::EpochRotationProof(proof) => {
            h.update(b"EpochRotationProof");
            h.update(proof.old_epoch.get().to_be_bytes());
            h.update(proof.new_epoch.get().to_be_bytes());
            h.update(proof.old_pubkey.as_bytes());
            h.update(proof.new_pubkey.as_bytes());
            h.update(proof.signed_at_unix.to_be_bytes());
        }
        CanonicalMessage::LedgerEntrySigning(digest) => {
            h.update(b"LedgerEntrySigning");
            h.update(digest);
        }
    }
    h.finalize().into()
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: public system signature verification against pinned epoch keys.
pub fn verify_system_signature(
    sig: &SystemSignature,
    message: &CanonicalMessage,
    epoch: SystemEpoch,
    pinned_pubkeys: &PinnedSystemPubkeys,
) -> bool {
    let Some(pk) = pinned_pubkeys.get(epoch) else {
        return false;
    };
    let Ok(verifying_key) = VerifyingKey::from_bytes(pk.as_bytes()) else {
        return false;
    };
    let signature = Signature::from_bytes(sig.as_bytes());
    verifying_key
        .verify(&canonical_digest(message), &signature)
        .is_ok()
}

/// TRACE_MATRIX FC3-Sig: verify old and new signatures over a rotation continuity proof.
pub fn verify_epoch_rotation_proof(
    proof: &EpochRotationProof,
    old_signature: &SystemSignature,
    new_signature: &SystemSignature,
    pinned_pubkeys: &PinnedSystemPubkeys,
) -> bool {
    let message = CanonicalMessage::EpochRotationProof(proof.clone());
    verify_system_signature(old_signature, &message, proof.old_epoch(), pinned_pubkeys)
        && verify_system_signature(new_signature, &message, proof.new_epoch(), pinned_pubkeys)
}

/// TRACE_MATRIX FC3-Sig: boot extension stub for genesis `[system_pubkeys]` verification.
pub fn verify_system_pubkeys(genesis_payload_toml: &str) -> Result<(), TrustRootError> {
    if !has_toml_section(genesis_payload_toml, "system_pubkeys") {
        return Ok(());
    }
    // TODO(CO1.7): parse genesis_payload.toml [system_pubkeys] entries and
    // verify creator PGP signatures against the pinned creator public key.
    Ok(())
}

/// TRACE_MATRIX FC1-Sig: crate-only signing surface for the predicate runner.
pub(crate) mod predicate_runner {
    use super::{
        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, KeypairError,
        RejectedAttemptSummary, SystemSignature,
    };

    /// TRACE_MATRIX FC1-Sig: sign only typed rejected-attempt summaries from the predicate runner.
    pub(crate) fn sign_rejected_attempt_summary(
        keypair: &Ed25519Keypair,
        summary: &RejectedAttemptSummary,
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(
            keypair,
            &CanonicalMessage::RejectedAttemptSummary(summary.clone()),
        )
    }

    /// TRACE_MATRIX FC1-Sig: sign only typed canonical messages within the predicate-runner scope.
    pub(crate) fn sign_system_message(
        keypair: &Ed25519Keypair,
        message: &CanonicalMessage,
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(keypair, message)
    }
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: crate-only signing surface for system-emitted
/// transition signatures (terminal summary, finalize reward, task expire).
///
/// **CO1.1.4-pre1 v1.1 round-1 closure (C-3) + v1.2 round-2 closure (R2-2)**:
/// signs an opaque `[u8; 32]` digest produced by
/// `state::typed_tx::TerminalSummaryTx::to_signing_payload().canonical_digest()`
/// (and the parallel paths via `FinalizeRewardSigningPayload::canonical_digest()` /
/// `TaskExpireSigningPayload::canonical_digest()`). Same opaque-digest pattern
/// as `transition_ledger_emitter::sign_ledger_entry` — keeps `system_keypair`
/// oblivious to the typed-tx schema; no `bottom_white ↔ state` circular dep.
pub(crate) mod terminal_summary_emitter {
    use super::{
        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, EpochRotationProof,
        KeypairError, SystemSignature,
    };

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: sign an opaque 32-byte digest of a
    /// terminal-summary signing payload (computed by typed_tx).
    pub(crate) fn sign_terminal_summary(
        keypair: &Ed25519Keypair,
        digest: [u8; 32],
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(keypair, &CanonicalMessage::TerminalSummarySigning(digest))
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): sign an
    /// opaque 32-byte digest of a `FinalizeRewardSigningPayload` (computed by
    /// state::typed_tx). Symmetric to `sign_terminal_summary` and
    /// `sign_task_expire`.
    pub(crate) fn sign_finalize_reward(
        keypair: &Ed25519Keypair,
        digest: [u8; 32],
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(keypair, &CanonicalMessage::FinalizeRewardSigning(digest))
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): sign an
    /// opaque 32-byte digest of a `TaskExpireSigningPayload` (computed by
    /// state::typed_tx). Symmetric to `sign_terminal_summary` and
    /// `sign_finalize_reward`.
    pub(crate) fn sign_task_expire(
        keypair: &Ed25519Keypair,
        digest: [u8; 32],
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(keypair, &CanonicalMessage::TaskExpireSigning(digest))
    }

    /// TRACE_MATRIX FC3-Sig: sign only typed epoch rotation proofs.
    pub(crate) fn sign_epoch_rotation_proof(
        keypair: &Ed25519Keypair,
        proof: &EpochRotationProof,
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(
            keypair,
            &CanonicalMessage::EpochRotationProof(proof.clone()),
        )
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: sign only typed canonical messages within terminal-summary scope.
    pub(crate) fn sign_system_message(
        keypair: &Ed25519Keypair,
        message: &CanonicalMessage,
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(keypair, message)
    }
}

/// TRACE_MATRIX FC2-Append + FC1-Sig: crate-only signing surface for the L4
/// transition ledger sequencer (CO1.7 v1.2). Authorized emitter pattern per
/// round-1 audit Q-G recommendation: the ledger sequencer calls
/// `sign_ledger_entry` with the canonical digest of `LedgerEntrySigningPayload`
/// and gets back a `SystemSignature` bound through `CanonicalMessage`. No raw
/// digest signer escapes this module.
pub(crate) mod transition_ledger_emitter {
    use super::{
        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, KeypairError, SystemSignature,
    };

    /// TRACE_MATRIX FC2-Append: sign only the canonical-digest of a
    /// `LedgerEntrySigningPayload`. Caller (sequencer in CO1.7) is responsible
    /// for computing the digest; this fn only wraps in the typed enum.
    pub(crate) fn sign_ledger_entry(
        keypair: &Ed25519Keypair,
        signing_payload_digest: [u8; 32],
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(
            keypair,
            &CanonicalMessage::LedgerEntrySigning(signing_payload_digest),
        )
    }
}

fn sign_system_message_inner(
    keypair: &Ed25519Keypair,
    message: &CanonicalMessage,
) -> Result<SystemSignature, KeypairError> {
    keypair.sign_digest(canonical_digest(message))
}

fn encrypt_at_rest(
    keypair: &Ed25519Keypair,
    user_kdf_password: &SecretString,
) -> Result<Vec<u8>, KeypairError> {
    let kdf = KdfParams::from_env()?;
    let mut salt = [0u8; SALT_LEN];
    let mut nonce = [0u8; NONCE_LEN];
    getrandom::getrandom(&mut salt).map_err(KeypairError::Entropy)?;
    getrandom::getrandom(&mut nonce).map_err(KeypairError::Entropy)?;

    let mut key = derive_key(user_kdf_password, &salt, kdf)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&key)
        .map_err(|_| KeypairError::Crypto("bad cipher key"))?;
    let mut plaintext = keypair.to_plaintext()?;
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
        .map_err(|_| KeypairError::Crypto("keystore encryption failed"))?;
    plaintext.zeroize();
    key.zeroize();

    EncryptedKeypair {
        kdf,
        salt,
        nonce,
        ciphertext,
    }
    .encode()
}

fn derive_key(
    user_kdf_password: &SecretString,
    salt: &[u8; SALT_LEN],
    kdf: KdfParams,
) -> Result<[u8; DERIVED_KEY_LEN], KeypairError> {
    let params = Params::new(
        kdf.memory_kib,
        kdf.iterations,
        kdf.lanes,
        Some(DERIVED_KEY_LEN),
    )
    .map_err(|err| KeypairError::KdfParam(err.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; DERIVED_KEY_LEN];
    argon2
        .hash_password_into(user_kdf_password.expose_secret().as_bytes(), salt, &mut key)
        .map_err(KeypairError::Kdf)?;
    Ok(key)
}

fn write_keystore_0600(path: &Path, bytes: &[u8]) -> Result<(), KeypairError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    set_open_options_mode_0600(&mut options);
    let mut file = options.open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    set_file_permissions_0600(path)?;
    Ok(())
}

#[cfg(unix)]
fn set_open_options_mode_0600(options: &mut OpenOptions) {
    use std::os::unix::fs::OpenOptionsExt;
    options.mode(0o600);
}

#[cfg(not(unix))]
fn set_open_options_mode_0600(_options: &mut OpenOptions) {}

#[cfg(unix)]
fn set_file_permissions_0600(path: &Path) -> Result<(), KeypairError> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).map_err(KeypairError::Io)
}

#[cfg(not(unix))]
fn set_file_permissions_0600(_path: &Path) -> Result<(), KeypairError> {
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct KdfParams {
    memory_kib: u32,
    iterations: u32,
    lanes: u32,
}

impl KdfParams {
    fn from_env() -> Result<Self, KeypairError> {
        Ok(Self {
            memory_kib: read_env_u32("TURINGOS_KDF_MEMORY_KIB", DEFAULT_KDF_MEMORY_KIB)?,
            iterations: read_env_u32("TURINGOS_KDF_ITER", DEFAULT_KDF_ITER)?,
            lanes: read_env_u32("TURINGOS_KDF_LANES", DEFAULT_KDF_LANES)?,
        })
    }
}

fn read_env_u32(name: &str, default: u32) -> Result<u32, KeypairError> {
    match env::var(name) {
        Ok(value) => {
            let parsed = value
                .parse::<u32>()
                .map_err(|_| KeypairError::KdfParam(format!("{name} must be u32")))?;
            if parsed == 0 {
                return Err(KeypairError::KdfParam(format!("{name} must be non-zero")));
            }
            Ok(parsed)
        }
        Err(env::VarError::NotPresent) => Ok(default),
        Err(env::VarError::NotUnicode(_)) => {
            Err(KeypairError::KdfParam(format!("{name} is not unicode")))
        }
    }
}

struct EncryptedKeypair {
    kdf: KdfParams,
    salt: [u8; SALT_LEN],
    nonce: [u8; NONCE_LEN],
    ciphertext: Vec<u8>,
}

impl EncryptedKeypair {
    fn encode(self) -> Result<Vec<u8>, KeypairError> {
        let ciphertext_len = u32::try_from(self.ciphertext.len())
            .map_err(|_| KeypairError::InvalidFormat("ciphertext too large"))?;
        let mut out = Vec::with_capacity(
            FORMAT_MAGIC.len() + 1 + 4 + 4 + 4 + SALT_LEN + NONCE_LEN + 4 + self.ciphertext.len(),
        );
        out.extend_from_slice(FORMAT_MAGIC);
        out.push(FORMAT_VERSION);
        out.extend_from_slice(&self.kdf.memory_kib.to_be_bytes());
        out.extend_from_slice(&self.kdf.iterations.to_be_bytes());
        out.extend_from_slice(&self.kdf.lanes.to_be_bytes());
        out.extend_from_slice(&self.salt);
        out.extend_from_slice(&self.nonce);
        out.extend_from_slice(&ciphertext_len.to_be_bytes());
        out.extend_from_slice(&self.ciphertext);
        Ok(out)
    }

    fn decode(bytes: &[u8]) -> Result<Self, KeypairError> {
        let mut cursor = Cursor::new(bytes);
        if cursor.read(FORMAT_MAGIC.len())? != FORMAT_MAGIC {
            return Err(KeypairError::InvalidFormat("bad magic"));
        }
        if cursor.read_u8()? != FORMAT_VERSION {
            return Err(KeypairError::InvalidFormat("bad version"));
        }
        let kdf = KdfParams {
            memory_kib: cursor.read_u32()?,
            iterations: cursor.read_u32()?,
            lanes: cursor.read_u32()?,
        };
        let mut salt = [0u8; SALT_LEN];
        salt.copy_from_slice(cursor.read(SALT_LEN)?);
        let mut nonce = [0u8; NONCE_LEN];
        nonce.copy_from_slice(cursor.read(NONCE_LEN)?);
        let ciphertext_len = cursor.read_u32()? as usize;
        let ciphertext = cursor.read(ciphertext_len)?.to_vec();
        if !cursor.is_finished() {
            return Err(KeypairError::InvalidFormat("trailing bytes"));
        }
        Ok(Self {
            kdf,
            salt,
            nonce,
            ciphertext,
        })
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    fn read(&mut self, len: usize) -> Result<&'a [u8], KeypairError> {
        let end = self
            .pos
            .checked_add(len)
            .ok_or(KeypairError::InvalidFormat("offset overflow"))?;
        if end > self.bytes.len() {
            return Err(KeypairError::InvalidFormat("truncated keystore"));
        }
        let slice = &self.bytes[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    fn read_u8(&mut self) -> Result<u8, KeypairError> {
        Ok(self.read(1)?[0])
    }

    fn read_u32(&mut self) -> Result<u32, KeypairError> {
        let mut out = [0u8; 4];
        out.copy_from_slice(self.read(4)?);
        Ok(u32::from_be_bytes(out))
    }

    fn is_finished(&self) -> bool {
        self.pos == self.bytes.len()
    }
}

fn update_len_prefixed(h: &mut Sha256, bytes: &[u8]) {
    h.update((bytes.len() as u64).to_be_bytes());
    h.update(bytes);
}

fn has_toml_section(text: &str, name: &str) -> bool {
    text.lines().any(|raw| {
        let line = strip_comment(raw).trim();
        line.strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .map(|header| header.trim() == name)
            .unwrap_or(false)
    })
}

fn strip_comment(line: &str) -> &str {
    let mut in_string = false;
    for (i, c) in line.char_indices() {
        match c {
            '"' => in_string = !in_string,
            '#' if !in_string => return &line[..i],
            _ => {}
        }
    }
    line
}

fn mlock_best_effort(ptr: *const u8, len: usize) -> bool {
    if ptr.is_null() || len == 0 {
        return false;
    }
    mlock_os_best_effort(ptr, len)
}

#[cfg(unix)]
fn mlock_os_best_effort(ptr: *const u8, len: usize) -> bool {
    // SAFETY: `ptr` and `len` come from a live boxed private-key byte slice in
    // `Ed25519Keypair`; mlock does not take ownership and failure is non-fatal.
    unsafe { libc::mlock(ptr.cast(), len) == 0 }
}

#[cfg(not(unix))]
fn mlock_os_best_effort(_ptr: *const u8, _len: usize) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorized_scope_signing_round_trip() {
        let keypair = Ed25519Keypair::generate_with_secure_entropy().expect("keypair");
        let summary = RejectedAttemptSummary::new("run", "attempt", "predicate_reject", [7u8; 32]);
        let sig =
            predicate_runner::sign_rejected_attempt_summary(&keypair, &summary).expect("sign");

        let mut pinned = PinnedSystemPubkeys::new();
        pinned.insert(SystemEpoch::new(1), keypair.public_key());
        assert!(verify_system_signature(
            &sig,
            &CanonicalMessage::RejectedAttemptSummary(summary),
            SystemEpoch::new(1),
            &pinned
        ));
    }

    #[test]
    fn terminal_scope_rotation_signing_round_trip() {
        let old = Ed25519Keypair::generate_with_secure_entropy().expect("old");
        let new = Ed25519Keypair::generate_with_secure_entropy().expect("new");
        let proof = EpochRotationProof::new(
            SystemEpoch::new(1),
            SystemEpoch::new(2),
            old.public_key(),
            new.public_key(),
            1_776_000_000,
        );
        let old_sig =
            terminal_summary_emitter::sign_epoch_rotation_proof(&old, &proof).expect("old sign");
        let new_sig =
            terminal_summary_emitter::sign_epoch_rotation_proof(&new, &proof).expect("new sign");

        let mut pinned = PinnedSystemPubkeys::new();
        pinned.insert(SystemEpoch::new(1), old.public_key());
        pinned.insert(SystemEpoch::new(2), new.public_key());
        assert!(verify_epoch_rotation_proof(
            &proof, &old_sig, &new_sig, &pinned
        ));
    }
}

```

---

Now give your INDEPENDENT round-3 audit. Cite spec § + line where possible. Round-3 expects PASS unless v1.1 missed a closure or introduced a regression.

codex
I’ll verify the v1.1 spec against the actual checked-out files and prior audit artifacts, then return a finding-led round-3 verdict with line citations. First I’m checking the repo instructions and locating the target documents.
exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g 'handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md' -g 'handover/audits/*CO1_7_EXTRA*ROUND2*' -g 'handover/audits/CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md
handover/audits/CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md
handover/audits/GEMINI_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md
handover/audits/CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md

exec
/bin/bash -lc 'find .. -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
 succeeded in 682ms:
../turingos/AGENTS.md
../omega_kepler_discovery/AGENTS.md
../omega/AGENTS.md
../turingosv2/AGENTS.md
../turingclaw/AGENTS.md

codex
There is no `AGENTS.md` under this repository or its parent `projects` directory, so there are no local repo-specific instructions governing this read-only audit. I’m now extracting the exact spec and source line ranges that the round-2 fixes touched.
exec
/bin/bash -lc "test -f /AGENTS.md && printf '/AGENTS.md\\n' || true; test -f /home/AGENTS.md && printf '/home/AGENTS.md\\n' || true; test -f /home/zephryj/AGENTS.md && printf '/home/zephryj/AGENTS.md\\n' || true; test -f /home/zephryj/projects/AGENTS.md && printf '/home/zephryj/projects/AGENTS.md\\n' || true; test -f /home/zephryj/projects/turingosv4/AGENTS.md && printf '/repo/AGENTS.md\\n' || true" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
exec
/bin/bash -lc "nl -ba handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# CO1.7-extra: L4 head_t close + Sequencer entry-point wiring v1.1 (post round-2 audit patches)
     2	
     3	**Status**: v1.1 DRAFT (2026-04-29; post round-2 dual external audit on v1 at HEAD `617f01e`). Round-2 returned CHALLENGE/CHALLENGE; v1.1 applies 10 patches (MF1-MF10 per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md`). Awaiting round-3 dual external audit.
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
    78	pub(crate) fn advance_head_t(q: &mut QState, writer: &dyn LedgerWriter) {
    79	    if let Some(commit_oid_hex) = writer.head_commit_oid_hex() {
    80	        q.head_t = crate::state::q_state::NodeId(commit_oid_hex);
    81	    }
    82	}
    83	```
    84	
    85	```rust
    86	// src/state/sequencer.rs::apply_one stage 9 (currently lines 362-373; v1.1 patch)
    87	let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
    88	let mut writer_w = self.ledger_writer.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
    89	writer_w.commit(&entry)?;
    90	self.next_logical_t.store(logical_t, Ordering::SeqCst);
    91	*q_w = q_next;
    92	q_w.ledger_root_t = entry.resulting_ledger_root;
    93	// NEW (CO1.7-extra D2): close G-1 head_t carry-forward.
    94	advance_head_t(&mut *q_w, &**writer_w);
    95	```
    96	
    97	**Stale comments must be updated** (round-2 MF8 — Codex Q-8 finding): `src/state/sequencer.rs:180-184` + `:359-361` currently say "head_t mutation deferred to CO1.7.5+". CO1.7-extra implementation MUST update these comments to reflect "head_t closed by CO1.7-extra D2 via `advance_head_t` helper". Added to § 9 atom landing checklist.
    98	
    99	**NodeId disambiguation**: two `NodeId` types coexist — legacy `pub type NodeId = String` at `src/ledger.rs:13` (imported by TuringBus + Kernel for the legacy ledger event API) and new `pub struct NodeId(pub String)` at `src/state/q_state.rs:49`. `q.head_t` is typed as the new tuple-struct (`q_state.rs:311`); D2 constructs the new variant exclusively (legacy String alias is unused here).
   100	
   101	**Atomicity** (per Codex Q-B + round-2 MF9 wording correction): under acquired `q_w` + `writer_w` write locks, after `writer_w.commit(&entry)?` returns `Ok`, the remaining operations are an `AtomicU64::store` (infallible), a plain `*q_w = q_next` move (infallible), and `advance_head_t` (infallible). For writers whose `head_commit_oid_hex` returns `Some` (Git2LedgerWriter), this is a **post-commit non-failing best-effort head binding** — `q.head_t` advances atomically with `ledger_root_t` and `next_logical_t`. For writers returning `None` (InMemoryLedgerWriter), `advance_head_t` is **explicit no-op preservation** — `q.head_t` stays at its prior value (which equals `q_next.head_t` after the `*q_w = q_next` move because CO1.7 K3 v1.2 forbids transition bodies from mutating head_t).
   102	
   103	### 1.2 Trait method addition (round-2 MF3: REQUIRED, no default impl)
   104	
   105	`LedgerWriter` trait at `src/bottom_white/ledger/transition_ledger.rs` gains one **required** method (round-2 audits both converged on third option per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md` MF3):
   106	
   107	```rust
   108	pub trait LedgerWriter: Send + Sync {
   109	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
   110	    fn len(&self) -> u64;
   111	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;  // (existing; spec preserves)
   112	
   113	    /// NEW (CO1.7-extra D2): canonical 40-char lowercase hex commit OID of the
   114	    /// most recent appended entry, or None if the chain is empty / backend has
   115	    /// no commit-OID notion.
   116	    ///
   117	    /// **REQUIRED** (no default impl): Rust compiler enforces every
   118	    /// LedgerWriter implementation declares this method. This is the round-2
   119	    /// MF3 closure — both audits' safety arguments satisfied:
   120	    /// - **silent stagnation prevention** (Gemini r1+r2): impossible to inherit
   121	    ///   a default that silently leaves head_t stale; a missing impl is a
   122	    ///   compile error.
   123	    /// - **post-commit no-panic** (Codex r1): impl is free to return None at
   124	    ///   runtime if the backend has no OID notion; no panic risk.
   125	    fn head_commit_oid_hex(&self) -> Option<String>;
   126	}
   127	
   128	impl LedgerWriter for Git2LedgerWriter {
   129	    fn head_commit_oid_hex(&self) -> Option<String> {
   130	        self.head_commit_oid().map(|oid| oid.to_string())
   131	    }
   132	    // ... existing commit / len / read_at ...
   133	}
   134	
   135	impl LedgerWriter for InMemoryLedgerWriter {
   136	    /// InMemory has no git substrate → always None. Required by the trait
   137	    /// (no default to inherit) so the choice is explicit, not implicit.
   138	    fn head_commit_oid_hex(&self) -> Option<String> {
   139	        None
   140	    }
   141	    // ... existing ...
   142	}
   143	```
   144	
   145	This is a **breaking change** to any third-party `LedgerWriter` impl outside the workspace (would no longer compile). Inside the workspace, only Git2LedgerWriter and InMemoryLedgerWriter implement the trait; both get explicit declarations above. Forward-compat: any future LedgerWriter impl is forced to declare its OID semantics explicitly — a desirable property for a constitutional anchor field.
   146	
   147	---
   148	
   149	## § 2 D3 — Single-file STEP_B ceremony for TuringBus Sequencer entry-point
   150	
   151	### 2.1 Code change (round-2 MF4: Sequencer placement TuringBus, NOT Kernel)
   152	
   153	Round-2 Codex Q-7 + Gemini Q5 converged on placing Sequencer at TuringBus directly (not nested through Kernel). Rationale per round-2 MF4:
   154	- TuringBus already owns runtime orchestration (`src/bus.rs:53` + per CO1.7-impl). Sequencer is a runtime-orchestration peer of Kernel, not nested inside it.
   155	- Kernel `src/kernel.rs:5-6` has explicit warning against domain-specific terms; the documented "pure topology" role (`:15-17`) is preserved by NOT adding state-driver fields.
   156	- STEP_B Phase 0 less-invasive-alternative test: TuringBus-only is strictly simpler than TuringBus + Kernel coupled changes.
   157	
   158	`src/bus.rs` (note: actual struct name is **`TuringBus`** at `src/bus.rs:53`, NOT `Bus`):
   159	
   160	```rust
   161	// src/bus.rs (additive — TuringBus gets one field + one constructor variant + one method)
   162	pub struct TuringBus {
   163	    // ... existing fields including kernel: Kernel ...
   164	
   165	    /// NEW (CO1.7-extra D3): typed-tx Sequencer; None when bus runs in legacy
   166	    /// ledger-only mode (preserves back-compat with all existing tests).
   167	    /// Marked serde-skip if TuringBus has serde derives (Sequencer holds
   168	    /// Arc-locked runtime state that isn't serializable Q_t data).
   169	    #[serde(skip)]  // applied if TuringBus has Serialize/Deserialize
   170	    pub sequencer: Option<Arc<Sequencer>>,
   171	}
   172	
   173	impl TuringBus {
   174	    pub fn new(kernel: Kernel, config: BusConfig) -> Self {
   175	        Self { /* ...existing..., */ sequencer: None }
   176	    }
   177	
   178	    /// NEW: opt-in constructor that wires a typed-tx Sequencer alongside the legacy ledger.
   179	    pub fn with_sequencer(kernel: Kernel, config: BusConfig, sequencer: Arc<Sequencer>) -> Self {
   180	        Self { /* ...existing..., */ sequencer: Some(sequencer) }
   181	    }
   182	
   183	    /// NEW (CO1.7-extra D3): typed-tx submission path. Returns receipt
   184	    /// (submit_id) immediately; commit happens asynchronously in
   185	    /// Sequencer::run driver loop.
   186	    pub async fn submit_typed_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
   187	        match self.sequencer.as_ref() {
   188	            Some(seq) => seq.submit(tx).await,
   189	            None => Err(SubmitError::QueueClosed),
   190	        }
   191	    }
   192	}
   193	```
   194	
   195	`src/kernel.rs`: **UNTOUCHED** by CO1.7-extra. "Pure topology" doctrine preserved.
   196	
   197	`src/state/sequencer.rs` (round-2 MF6: manual Debug impl, NOT derive — Sequencer holds `Arc<Ed25519Keypair>` at line 199 and `Ed25519Keypair` intentionally has no Debug derive at `src/bottom_white/ledger/system_keypair.rs:282-284`; blanket derive fails to compile):
   198	
   199	```rust
   200	// src/state/sequencer.rs (additive — manual Debug impl for TuringBus.Debug propagation through Arc<Sequencer>)
   201	impl std::fmt::Debug for Sequencer {
   202	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   203	        // finish_non_exhaustive() — leaks no keypair / QState / CAS contents;
   204	        // satisfies Debug requirements for Arc<Sequencer> propagation.
   205	        f.debug_struct("Sequencer").finish_non_exhaustive()
   206	    }
   207	}
   208	```
   209	
   210	### 2.2 Single-file STEP_B ceremony (round-2 MF4 simplification)
   211	
   212	CO1.7-extra now touches a single STEP_B-restricted file: `src/bus.rs`. No combined-ceremony justification needed. Per `STEP_B_PROTOCOL.md` Phase 0, the change is "minimum sufficient version" and has no less-invasive alternative (the typed-tx submission path needs SOME entry-point in the runtime layer; TuringBus is the canonical orchestrator).
   213	
   214	**Ceremony procedure**:
   215	1. Branch A (`step-b-co1.7-extra-A`): edits `src/bus.rs` per § 2.1 (1 field + 1 constructor variant + 1 forwarder method). Also adds the manual `Debug` impl on `Sequencer` in `src/state/sequencer.rs` (NOT STEP_B-restricted; lands alongside for compile coherence).
   216	2. Branch B (`step-b-co1.7-extra-B`): independently re-derives the same edits from this spec (separate session / context).
   217	3. Byte-identity comparison: `diff src/bus.rs` between A and B. Identical → merge to `main`. Divergent → re-do with stricter spec.
   218	
   219	### 2.3 Forward-compat note (round-2 Gemini Q5 partial response)
   220	
   221	Gemini Q5 r2 noted "Kernel placement creates forward-compat hazard of Kernel bloat". The TuringBus placement avoids this hazard entirely — Kernel stays at "pure topology" role; future stateful runtime drivers (e.g., a hypothetical CO1.x event router) would land at TuringBus level alongside Sequencer, which is the natural runtime-orchestrator role for TuringBus to own. No further justification needed beyond Codex Q-7 + Gemini Q5 convergence.
   222	
   223	---
   224	
   225	## § 3 Test plan (substrate-independent; round-2 MF2 + MF5 + MF7 patches)
   226	
   227	Three tests, **flat-named in `tests/`** (round-2 MF5 — Cargo auto-discovery requires flat naming or a `tests/co1_7_extra/main.rs` harness; v1.1 chooses flat naming for simplicity):
   228	
   229	### 3.1 `tests/co1_7_extra_cas_payload_round_trip.rs`
   230	
   231	```rust
   232	//! CO1.7-extra D4: CAS payload round-trip + CID stability across restart.
   233	//! Verifies that CO1.4-extra sidecar persistence makes CasStore content
   234	//! reachable across cold-start, which is a precondition for CO1.7.5
   235	//! FullTransition replay (deferred; gated on substrate atoms).
   236	//! Substrate-independent: uses only CasStore + ObjectType (CO1.4 + CO1.4-extra
   237	//! shipped surfaces); does NOT depend on CO P2.x.
   238	
   239	#[test]
   240	fn cas_payload_round_trip_with_cid_stability_across_restart() {
   241	    let tmp = tempfile::TempDir::new().expect("tempdir");
   242	    let payload = b"co1.7-extra-deterministic-payload-v1";
   243	    let cid_first = {
   244	        let mut cas = CasStore::open(tmp.path()).expect("first open");
   245	        cas.put(payload, ObjectType::ProposalPayload, "test-epoch", 1, Some("CO1.7-extra".into()))
   246	            .expect("put")
   247	    };
   248	    // Drop CasStore handle; reopen (cold-start path).
   249	    let bytes = {
   250	        let cas = CasStore::open(tmp.path()).expect("reopen post-restart");
   251	        cas.get(&cid_first).expect("get post-restart")
   252	    };
   253	    assert_eq!(bytes.as_slice(), payload);
   254	}
   255	```
   256	
   257	### 3.2 `tests/co1_7_extra_git2_writer_head_oid_defense.rs`
   258	
   259	Round-2 MF7: the private module-test helper `entry_at` (at `transition_ledger.rs:813`; Codex r2 misidentified the name as `canonical_test_entry` but substantive finding holds — helper is private and unavailable to integration tests). Integration tests must construct `LedgerEntry` inline.
   260	

exec
/bin/bash -lc "nl -ba handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md | sed -n '260,620p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   260	
   261	```rust
   262	#[test]
   263	fn git2_writer_returns_some_after_commit() {
   264	    let tmp = tempfile::TempDir::new().expect("tempdir");
   265	    let mut writer = Git2LedgerWriter::open(tmp.path()).expect("open");
   266	
   267	    // Inline LedgerEntry construction (round-2 MF7) — uses public CO1.7-impl
   268	    // surfaces only.
   269	    let entry = LedgerEntry {
   270	        logical_t: 1,
   271	        parent_state_root: Hash::ZERO,
   272	        parent_ledger_root: Hash::ZERO,
   273	        tx_kind: TxKind::Work,
   274	        tx_payload_cid: Cid([0u8; 32]),
   275	        resulting_state_root: Hash::ZERO,
   276	        resulting_ledger_root: Hash([1u8; 32]),
   277	        timestamp_logical: 1,
   278	        epoch: SystemEpoch::new(1),
   279	        extensions: Default::default(),
   280	        system_signature: SystemSignature::from_bytes([0u8; 64]),
   281	    };
   282	
   283	    writer.commit(&entry).expect("commit");
   284	    // Defensive against silent head_t stagnation: if Git2LedgerWriter ever
   285	    // inherits a default behavior (impossible given round-2 MF3 — trait method
   286	    // is now required), this catches it. Belt-and-suspenders for the
   287	    // constitutional anchor.
   288	    assert!(
   289	        writer.head_commit_oid_hex().is_some(),
   290	        "Git2LedgerWriter MUST return Some after commit; constitutional anchor violation otherwise"
   291	    );
   292	}
   293	```
   294	
   295	### 3.3 `tests/co1_7_extra_sequencer_head_t_advancement.rs` (NEW — round-2 MF2 closure)
   296	
   297	Tests the actual D2 code path via the `advance_head_t` helper extraction:
   298	
   299	```rust
   300	//! CO1.7-extra D2: verifies advance_head_t correctly advances q.head_t
   301	//! when writer surfaces a commit OID, and preserves q.head_t when writer
   302	//! returns None. Substrate-independent: uses only LedgerWriter trait + QState.
   303	//! Closes round-2 MF2 (D2 code path was untested in v1).
   304	
   305	use std::sync::Mutex;
   306	
   307	/// Mock LedgerWriter that returns a configurable head_commit_oid_hex value.
   308	/// Stubs commit() to always succeed (returns dummy Hash).
   309	struct MockLedgerWriter {
   310	    head_oid: Mutex<Option<String>>,
   311	    len: u64,
   312	}
   313	
   314	impl LedgerWriter for MockLedgerWriter {
   315	    fn commit(&mut self, _entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
   316	        self.len += 1;
   317	        Ok(Hash([0xAB; 32]))
   318	    }
   319	    fn len(&self) -> u64 { self.len }
   320	    fn read_at(&self, _: u64) -> Result<LedgerEntry, LedgerWriterError> {
   321	        unimplemented!("test mock")
   322	    }
   323	    fn head_commit_oid_hex(&self) -> Option<String> {
   324	        self.head_oid.lock().expect("lock").clone()
   325	    }
   326	}
   327	
   328	#[test]
   329	fn advance_head_t_writes_node_id_when_writer_returns_some() {
   330	    let writer = MockLedgerWriter {
   331	        head_oid: Mutex::new(Some("a".repeat(40))),  // 40-hex literal
   332	        len: 0,
   333	    };
   334	    let mut q = QState::genesis();
   335	    let q_initial_head = q.head_t.clone();
   336	
   337	    turingosv4::state::sequencer::advance_head_t(&mut q, &writer);
   338	
   339	    // Post-condition: q.head_t = NodeId("aaaa...aaaa")
   340	    assert_eq!(q.head_t.0, "a".repeat(40));
   341	    assert_ne!(q.head_t, q_initial_head);
   342	}
   343	
   344	#[test]
   345	fn advance_head_t_preserves_node_id_when_writer_returns_none() {
   346	    let writer = MockLedgerWriter {
   347	        head_oid: Mutex::new(None),
   348	        len: 0,
   349	    };
   350	    let mut q = QState::genesis();
   351	    let q_initial_head = q.head_t.clone();
   352	
   353	    turingosv4::state::sequencer::advance_head_t(&mut q, &writer);
   354	
   355	    // Post-condition: q.head_t unchanged (no-op preservation per § 1.1).
   356	    assert_eq!(q.head_t, q_initial_head);
   357	}
   358	```
   359	
   360	Total: 3 tests across 3 flat-named integration test files.
   361	
   362	---
   363	
   364	## § 4 Out of scope (explicitly deferred)
   365	
   366	1. **D1 transition function bodies (7)** — moved to future CO1.7.5 atom; gated on CO P2.x substrate atoms (§ 0.1 table).
   367	2. **3 of 4 D4 tests** (`replay_full_transition_state_root`, `system_signature_verifies_via_canonical_message`, un-ignore `sequencer_serial_replay_byte_identity`) — all require D1 to actually commit; deferred with D1 to future CO1.7.5.
   368	3. **TransitionError 22-variant mapping table** — was over-claimed in prior bundled v1 (Codex Q-E); deferred with D1 to future CO1.7.5 spec.
   369	4. **RejectedAttemptSummary side-channel substantiation** — was overclaimed (Codex Q-E); deferred to future CO1.7.5 spec where it's actually relevant.
   370	5. **STATE_TRANSITION_SPEC v1.5 housekeeping issue filing** — committed to as a post-CO1.7-extra-PASS/PASS process item (§ 0.4); not gating implementation.
   371	6. **Legacy `src/ledger.rs` retirement** — CO1.1.5 atom; CO1.7-extra leaves the legacy WAL ledger fully running.
   372	7. **Materializer state_root computation** — CO1.8 (L5).
   373	
   374	---
   375	
   376	## § 5 Open questions (0 remain — all closed by round-2 audits)
   377	
   378	| Q | Round-2 resolution |
   379	|---|---|
   380	| Q1 `head_commit_oid_hex` default impl (round-1 open) | **Closed by round-2 MF3** — trait method is REQUIRED (no default); compiler enforces every impl declares (§ 1.2). Both audits' safety arguments satisfied. |
   381	| Q1' Sequencer Debug derive completeness (round-1 surfaced) | **Closed by round-2 MF6** — manual `impl Debug for Sequencer` with `f.debug_struct("Sequencer").finish_non_exhaustive()`; `#[derive(Debug)]` not viable because `Arc<Ed25519Keypair>` field has no Debug derive. Codex Q-5 confirms `finish_non_exhaustive()` leaks no keypair / QState / CAS contents. |
   382	
   383	CO1.7-extra v1.1 has zero open questions — round-3 audit verifies patch correctness only.
   384	
   385	---
   386	
   387	## § 6 Audit gates (round structure)
   388	
   389	| Round | Codex | Gemini | Conservative | Action |
   390	|---|---|---|---|---|
   391	| 1 (on prior bundled v1) | CHALLENGE / High | CHALLENGE / High | **CHALLENGE** | Atom rescoped via Occam scope-split (this v1) + small fixes |
   392	| 2 (on this spec) | ⏳ pending | ⏳ pending | TBD | re-audit on CO1.7-extra v1; 1 round expected (small, focused atom) |
   393	| 3+ if needed | … | … | … | iterate to PASS/PASS |
   394	
   395	**Pre-implementation gate**: spec must reach PASS/PASS before any code in `src/state/sequencer.rs` D2 lines, `src/bus.rs` forwarder, `src/kernel.rs` field, or `src/bottom_white/ledger/transition_ledger.rs` trait method is written. Per CLAUDE.md "Audit Standard".
   396	
   397	---
   398	
   399	## § 7 Estimated scope (round-2 MF10: revised upward)
   400	
   401	- **Spec rounds**: round-2 done (CHALLENGE/CHALLENGE → 10 patches in v1.1); round-3 expected to PASS/PASS (small atom, all r2 issues addressed systematically). Round-3 budget ~$5-10.
   402	- **Implementation scope** (post-PASS/PASS):
   403	  - D2 (head_t close: `advance_head_t` helper + apply_one stage 9 patch + required trait method + 2 impl declarations + stale-comment updates at sequencer.rs:180-184/:359-361): ~40-60 LoC
   404	  - D3 (TuringBus field + with_sequencer constructor + submit_typed_tx forwarder + manual Sequencer Debug impl): ~50-80 LoC across single STEP_B-restricted file (bus.rs) + 1 supporting file (sequencer.rs Debug impl)
   405	  - D4 (3 tests with mock LedgerWriter + inline LedgerEntry fixture): ~120-160 LoC
   406	- **Total atom budget**: ~210-300 LoC (revised up from v1's 150-230 per round-2 MF10 — manual Debug + helper extraction + 3rd test + harness adjustments). **Estimated calendar time**: 1-2 days.
   407	
   408	---
   409	
   410	## § 8 Honest acknowledgements (v1.1)
   411	
   412	1. **Scope split is round-1-driven**, not voluntary. Prior bundled CO1.7.5 v1 spec was found by Codex r1 Q-D/H/I to have heavyweight cross-layer substrate dependencies in D1. v1 reverts CO1.7.5 to its CO1.7 § 13 original meaning (transition bodies; future) and creates CO1.7-extra (this atom) as a new bridge for the substrate-independent wiring.
   413	2. **`head_commit_oid_hex` is a NEW REQUIRED trait method** (no default impl; round-2 MF3). Compiler enforces every LedgerWriter impl declares; both `Git2LedgerWriter` and `InMemoryLedgerWriter` get explicit declarations in § 1.2.
   414	3. **D2 logic is extracted into `advance_head_t` helper** (round-2 MF2 closure). The extraction adds zero behavior change but makes D2 directly testable via mock writer (without injecting dispatch_transition into Sequencer).
   415	4. **TuringBus owns Sequencer directly** (round-2 MF4) — not nested through Kernel. Kernel preserves "pure topology" doctrine (`src/kernel.rs:5-6`+`:15-17`) and stays UNTOUCHED by this atom. STEP_B becomes single-file ceremony on `src/bus.rs`.
   416	5. **Manual Sequencer Debug impl** (round-2 MF6) — `#[derive(Debug)]` fails because `Arc<Ed25519Keypair>` field has no Debug (system_keypair.rs:282-284 intentional); `finish_non_exhaustive()` is the safe replacement (Codex Q-5 confirmed no leak risk).
   417	6. **STATE_TRANSITION_SPEC v1.5 housekeeping issue filing is committed** (§ 0.4) per round-1 Gemini MF1 active-reconciliation requirement. Round-2 confirmed the directionally correct framing; v1.1 corrected the supersession-disposition table (head_t enacted HERE; SignalKind migrates to future CO1.7.5).
   418	7. **Most of CO1.1.4-pre1 ABI lock is irrelevant to this atom** — D1 (the part that uses TypedTx + TransitionError + SignalKind) is out of scope. CO1.7-extra only touches `LedgerWriter` trait + TuringBus wiring + Sequencer Debug impl; ABI lock untouched.
   419	8. **Stale Sequencer comments will be updated** during implementation (round-2 MF8): `src/state/sequencer.rs:180-184` + `:359-361` currently say "head_t deferred to CO1.7.5+"; CO1.7-extra implementation must update them to reflect the new D2 reality.
   420	9. **FC-trace requirements**: the new pub symbols introduced by CO1.7-extra implementation must carry doc-comment `/// TRACE_MATRIX <FC-id>: <role>` backlinks per CLAUDE.md "Alignment Standard". Set: `LedgerWriter::head_commit_oid_hex` + `advance_head_t` helper (→ § 5 L4 sequencer post-commit head_t wiring); `TuringBus.sequencer` field + `TuringBus::with_sequencer` + `TuringBus::submit_typed_tx` (→ § 5.2.1 single-writer entry-point).
   421	
   422	---
   423	
   424	## § 9 Pre-audit smoke test plan (v1.1; round-3 launch)
   425	
   426	Per memory `feedback_smoke_before_batch`. Smoke run before round-3 audit launch, at the v1.1 commit HEAD.
   427	
   428	| # | Claim | Smoke command | Pass criterion |
   429	|---|---|---|---|
   430	| S1 | `Git2LedgerWriter::head_commit_oid()` returns `Option<git2::Oid>` | `grep -A1 'pub fn head_commit_oid' src/bottom_white/ledger/transition_ledger.rs` | matches signature (line 674) |
   431	| S2 | Bus struct is named `TuringBus` | `grep -n 'pub struct TuringBus' src/bus.rs` | one hit at line 53 |
   432	| S3 | Kernel UNTOUCHED by this atom (round-2 MF4) | `grep -n 'use crate::ledger::' src/kernel.rs && grep -L 'sequencer' src/kernel.rs` | legacy ledger import present; no "sequencer" reference (Kernel stays at pure topology) |
   433	| S4 | Sequencer struct exists at sequencer.rs:190 | `grep -n 'pub struct Sequencer' src/state/sequencer.rs` | one hit at line 190 |
   434	| S5 | Ed25519Keypair has no Debug derive (forces manual Sequencer Debug impl per MF6) | `grep -B5 'pub struct Ed25519Keypair' src/bottom_white/ledger/system_keypair.rs` | no `#[derive(Debug` precedes struct line |
   435	| S6 | CasStore exposes `put` + `get` (CO1.4 + CO1.4-extra) | `grep -n 'pub fn put\|pub fn get' src/bottom_white/cas/store.rs` | both present |
   436	| S7 | Wallet (`src/sdk/tools/wallet.rs`) untouched | `grep -c 'transition_ledger\|state::sequencer\|TypedTx' src/sdk/tools/wallet.rs` | 0 hits |
   437	| S8 | QState.head_t is `state::q_state::NodeId` (tuple struct) | `grep -B1 -A1 'pub head_t' src/state/q_state.rs` | type matches |
   438	| S9 | Stale comment locations (round-2 MF8) | `grep -n 'CO1.7.5+\|deferred to CO1.7.5' src/state/sequencer.rs` | hits at lines 180-184 + 359-361 (to be updated by D2 implementation) |
   439	| S10 | `entry_at` (private module-test helper) is private (round-2 MF7) | `sed -n '810,820p' src/bottom_white/ledger/transition_ledger.rs` | `fn entry_at(...)` at line 813 inside `mod tests`; no `pub` qualifier |
   440	| S11 | cargo baseline | `cargo check --workspace && cargo test --workspace --lib` | clean compile + 239 / 0 / 1 ignored |
   441	
   442	---
   443	
   444	**END v1 DRAFT body.**
   445	
   446	## Pre-audit smoke results
   447	
   448	### Round-2 smoke (HEAD `617f01e`; v1)
   449	
   450	8/8 PASS — see prior commit log. v1 spec sent to round-2 dual external audit on this baseline.
   451	
   452	### Round-3 smoke (HEAD `25564d7`; v1.1 patches commit)
   453	
   454	| # | Claim | Result | Status |
   455	|---|---|---|---|
   456	| S1 | head_commit_oid signature | `pub fn head_commit_oid(&self) -> Option<git2::Oid>` (transition_ledger.rs:674) | ✅ PASS |
   457	| S2 | TuringBus struct | `pub struct TuringBus` at bus.rs:53 | ✅ PASS |
   458	| S3 | Kernel UNTOUCHED | `use crate::ledger::{Node, NodeId, Tape, TapeError}` at kernel.rs:8; **0 hits** of "sequencer" anywhere in kernel.rs (pure topology preserved per round-2 MF4) | ✅ PASS |
   459	| S4 | Sequencer struct line | `pub struct Sequencer` at sequencer.rs:190 | ✅ PASS |
   460	| S5 | Ed25519Keypair has NO Debug derive | `#[derive(Zeroize, ZeroizeOnDrop)]` precedes `pub struct Ed25519Keypair` (system_keypair.rs:282-284); no Debug → forces manual Sequencer Debug impl per MF6 | ✅ PASS |
   461	| S6 | CasStore put + get | `pub fn put` at line 163, `pub fn get` at line 199 | ✅ PASS |
   462	| S7 | wallet untouched | 0 hits in `src/sdk/tools/wallet.rs` | ✅ PASS |
   463	| S8 | head_t type | `pub head_t: NodeId` (q_state.rs:311) — type matches new tuple-struct | ✅ PASS |
   464	| S9 | stale comments confirmed | sequencer.rs:178-184 (doc on apply_one Sequencer) + :357-361 (in apply_one stage 9 inline comment) — both still say "deferred to CO1.7.5+"; will be patched by D2 implementation per atom landing checklist | ✅ PASS (with minor line cite refinement vs v1.1 spec's "180-184 + 359-361" — actual lines 178-184 + 357-361; spec patched in this commit) |
   465	| S10 | private module-test helper exists | `fn entry_at` at transition_ledger.rs:813 inside `mod tests`; no `pub` qualifier (Codex r2 misidentified name as `canonical_test_entry` but substantive finding holds) | ✅ PASS (with helper-name correction) |
   466	| S11 | cargo baseline | check pass; `239 passed; 0 failed; 1 ignored` (the ignored test is `sequencer_serial_replay_byte_identity`, deferred to future CO1.7.5 atom) | ✅ PASS |
   467	
   468	**Smoke gate**: 11 / 11 PASS at HEAD `25564d7`. Spec v1.1 ready for round-3 dual external audit.
   469	
   470	### Patch log
   471	
   472	**v1 (round-1 scope split; commits `f7fc19f` + `617f01e`)**:
   473	- Scope split per round-1 Codex r1 Q-D/H/I + ArchitectAI Occam decision. D1 + 3 D4 tests + un-ignore migrated to future CO1.7.5 atom (gated on CO P2.x). v1 inherited round-1 fixes M3-M5 + § 0.4 active reconciliation.
   474	
   475	**v1.1 (round-2 driven; this revision)** — 10 patches per `CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md`:
   476	
   477	- **MF1** § 0.4 supersession-disposition table corrected: head_t supersession **enacted in CO1.7-extra D2**; SignalKind supersession migrates to future CO1.7.5 (was: both migrate)
   478	- **MF2** D2 testability: `advance_head_t(q, writer)` helper extracted from apply_one stage 9 → makes D2 directly testable via mock writer (§ 1.1 + new test § 3.3)
   479	- **MF3** trait method `head_commit_oid_hex` becomes REQUIRED (no default impl); Rust compiler enforces every LedgerWriter declares (§ 1.2). Both audits' safety arguments satisfied via this third-option synthesis.
   480	- **MF4** Sequencer placement: TuringBus owns directly (NOT nested through Kernel). Kernel UNTOUCHED. STEP_B becomes single-file ceremony (§ 2.1 + § 2.2).
   481	- **MF5** test harness: flat-named `tests/co1_7_extra_*.rs` for Cargo auto-discovery (§ 3 file paths)
   482	- **MF6** Sequencer Debug: manual `impl Debug` with `finish_non_exhaustive()` (Ed25519Keypair has no Debug derive at system_keypair.rs:282-284 — blanket derive fails) (§ 2.1)
   483	- **MF7** `entry_at` private → tests construct LedgerEntry inline (§ 3.2)
   484	- **MF8** stale Sequencer comments (sequencer.rs:178-184 + :357-361) added to atom landing checklist (§ 1.1 + § 8 ack #8)
   485	- **MF9** atomicity wording: "post-commit non-failing best-effort head binding (Some path)" + "explicit no-op preservation (None path)" (§ 1.1)
   486	- **MF10** LoC estimate: 150-230 → 200-280 (manual Debug + helper extraction + 3rd test + harness adjustments) (§ 7)
   487	
   488	### Awaiting
   489	
   490	1. round-3 dual external audit on CO1.7-extra v1.1
   491	2. expected PASS/PASS (small atom, all r2 issues addressed systematically)
   492	3. then CO1.7-extra-impl (D2 helper extraction + apply_one patch + trait method + TuringBus single-file STEP_B + 3 tests + stale-comment update)
   493	4. file STATE_TRANSITION_SPEC v1.5 housekeeping issue per § 0.4 commitment
   494	5. spec future CO1.7.5 (transition bodies; gated on CO P2.x substrate atoms)

codex
The main body is mostly patched, but I already found two stale top-level references that still mention Kernel wiring despite § 2 saying Kernel is untouched. I’m continuing the source-level checks to decide whether that is just editorial drift or a gating defect.
exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '160,390p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '250,360p;650,705p;790,825p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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
   806	    use super::*;
   807	
   808	    fn h(byte: u8) -> Hash {
   809	        Hash([byte; 32])
   810	    }
   811	
   812	    /// Build an entry that satisfies all chain invariants given the previous state.
   813	    fn entry_at(
   814	        logical_t: u64,
   815	        parent_state_root: Hash,
   816	        parent_ledger_root: Hash,
   817	        resulting_state_root: Hash,
   818	    ) -> LedgerEntry {
   819	        let signing = LedgerEntrySigningPayload {
   820	            logical_t,
   821	            parent_state_root,
   822	            parent_ledger_root,
   823	            tx_kind: TxKind::Work,
   824	            tx_payload_cid: Cid([0u8; 32]),
   825	            resulting_state_root,

exec
/bin/bash -lc "nl -ba src/bus.rs | sed -n '1,90p' && rg -n 'sequencer|Sequencer|submit_typed_tx|with_sequencer|Kernel|kernel\\.sequencer' handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md src/bus.rs src/kernel.rs" in /home/zephryj/projects/turingosv4
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
src/kernel.rs:19:pub struct Kernel {
src/kernel.rs:49:impl Kernel {
src/kernel.rs:51:        Kernel {
src/kernel.rs:63:    pub fn open_bounty_market(&mut self, lp_coins: f64) -> Result<(), KernelError> {
src/kernel.rs:65:            return Err(KernelError::MarketExists("__bounty__".to_string()));
src/kernel.rs:68:            .map_err(KernelError::Market)?;
src/kernel.rs:108:    pub fn append(&mut self, node: Node) -> Result<AppendResult, KernelError> {
src/kernel.rs:110:        self.tape.append(node).map_err(KernelError::Tape)?;
src/kernel.rs:115:    pub fn create_market(&mut self, node_id: &str, lp_coins: f64) -> Result<(), KernelError> {
src/kernel.rs:117:            return Err(KernelError::NodeNotFound(node_id.to_string()));
src/kernel.rs:120:            return Err(KernelError::MarketExists(node_id.to_string()));
src/kernel.rs:123:            .map_err(KernelError::Market)?;
src/kernel.rs:129:    pub fn buy_yes(&mut self, node_id: &str, coins: f64) -> Result<f64, KernelError> {
src/kernel.rs:131:            .ok_or_else(|| KernelError::MarketNotFound(node_id.to_string()))?;
src/kernel.rs:132:        let outcome = market.buy_yes(coins).map_err(KernelError::Market)?;
src/kernel.rs:137:    pub fn buy_no(&mut self, node_id: &str, coins: f64) -> Result<f64, KernelError> {
src/kernel.rs:139:            .ok_or_else(|| KernelError::MarketNotFound(node_id.to_string()))?;
src/kernel.rs:140:        let outcome = market.buy_no(coins).map_err(KernelError::Market)?;
src/kernel.rs:146:    pub fn trace_golden_path(&self, terminal_id: &str) -> Result<Vec<NodeId>, KernelError> {
src/kernel.rs:148:            return Err(KernelError::NodeNotFound(terminal_id.to_string()));
src/kernel.rs:159:    ) -> Result<ResolutionResult, KernelError> {
src/kernel.rs:170:            market.resolve(yes_wins).map_err(KernelError::Market)?;
src/kernel.rs:209:impl Default for Kernel {
src/kernel.rs:218:pub enum KernelError {
src/kernel.rs:226:impl std::fmt::Display for KernelError {
src/kernel.rs:229:            KernelError::Tape(e) => write!(f, "Tape error: {}", e),
src/kernel.rs:230:            KernelError::Market(e) => write!(f, "Market error: {}", e),
src/kernel.rs:231:            KernelError::NodeNotFound(id) => write!(f, "Node not found: {}", id),
src/kernel.rs:232:            KernelError::MarketNotFound(id) => write!(f, "Market not found for node: {}", id),
src/kernel.rs:233:            KernelError::MarketExists(id) => write!(f, "Market already exists for node: {}", id),
src/kernel.rs:238:impl std::error::Error for KernelError {}
src/kernel.rs:261:        let mut k = Kernel::new();
src/kernel.rs:268:        let mut k = Kernel::new();
src/kernel.rs:275:        let mut k = Kernel::new();
src/kernel.rs:281:        let mut k = Kernel::new();
src/kernel.rs:294:        let mut k = Kernel::new();
src/kernel.rs:300:        let mut k = Kernel::new();
src/kernel.rs:308:        let mut k = Kernel::new();
src/kernel.rs:319:        let mut k = Kernel::new();
src/kernel.rs:333:        let mut k = Kernel::new();
src/bus.rs:8:use crate::kernel::{Kernel, KernelError};
src/bus.rs:54:    pub kernel: Kernel,
src/bus.rs:87:    pub fn new(kernel: Kernel, config: BusConfig) -> Self {
src/bus.rs:106:        kernel: Kernel,
src/bus.rs:268:        // Phase 3: Kernel append (topology validation)
src/bus.rs:607:        let kernel = Kernel::new();
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:1:# CO1.7-extra: L4 head_t close + Sequencer entry-point wiring v1.1 (post round-2 audit patches)
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:9:- `CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` v1.2 — round-3 PASS/PASS; freezes `LedgerWriter` trait + Sequencer 9-stage apply_one + `Git2LedgerWriter::head_commit_oid()`.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:14:**Single sentence**: close the G-1 carry-forward `q.head_t = NodeId(commit_oid_hex)` after `Git2LedgerWriter.commit`, perform combined STEP_B ceremony adding a Sequencer entry-point on TuringBus + Kernel, and ship one substrate-independent CAS round-trip test — leaving transition function bodies + replay byte-identity to a future CO1.7.5 atom that depends on the Wave-2 substrate (CO P2.x family).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:28:| **CO1.7-extra (THIS spec)** | D2 head_t close + D3 Sequencer entry-point wiring + 1 substrate-independent test | None — uses only frozen `LedgerWriter` trait + `Git2LedgerWriter::head_commit_oid()` + existing `CasStore::put`/`get` | Now (post-PASS/PASS) |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:29:| **CO1.7.5 (future; restored to CO1.7 § 13 original meaning)** | D1 transition bodies (7) + 3 D4 tests + un-ignore `sequencer_serial_replay_byte_identity` | CO P2.1 / P2.2 / P2.3 / P2.5 / P2.6 / P2.7 / P2.9 + CO1.11 + (new) PredicateRegistry execution-methods atom | After substrate atoms reach individual PASS/PASS |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:38:| CO1.7-impl A2 (commit `2461fe6`) | `Sequencer` 9-stage `apply_one` + `dispatch_transition` exhaustive match (variants stay `Err(NotYetImplemented)` post-CO1.7-extra; D1 transition bodies are out of scope) |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:44:2. **D3** — Single-file STEP_B ceremony adds `Option<Arc<Sequencer>>` field + `with_sequencer` constructor + `submit_typed_tx` forwarder method to `TuringBus` (note: type is `TuringBus`, not `Bus`, per `src/bus.rs:53`). Sequencer lives in TuringBus directly (not nested through Kernel) per round-2 MF4 — Kernel preserves "pure topology" doctrine and stays UNTOUCHED by this atom.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:69:// src/state/sequencer.rs (NEW pub(crate) helper)
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:86:// src/state/sequencer.rs::apply_one stage 9 (currently lines 362-373; v1.1 patch)
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:97:**Stale comments must be updated** (round-2 MF8 — Codex Q-8 finding): `src/state/sequencer.rs:180-184` + `:359-361` currently say "head_t mutation deferred to CO1.7.5+". CO1.7-extra implementation MUST update these comments to reflect "head_t closed by CO1.7-extra D2 via `advance_head_t` helper". Added to § 9 atom landing checklist.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:99:**NodeId disambiguation**: two `NodeId` types coexist — legacy `pub type NodeId = String` at `src/ledger.rs:13` (imported by TuringBus + Kernel for the legacy ledger event API) and new `pub struct NodeId(pub String)` at `src/state/q_state.rs:49`. `q.head_t` is typed as the new tuple-struct (`q_state.rs:311`); D2 constructs the new variant exclusively (legacy String alias is unused here).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:149:## § 2 D3 — Single-file STEP_B ceremony for TuringBus Sequencer entry-point
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:151:### 2.1 Code change (round-2 MF4: Sequencer placement TuringBus, NOT Kernel)
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:153:Round-2 Codex Q-7 + Gemini Q5 converged on placing Sequencer at TuringBus directly (not nested through Kernel). Rationale per round-2 MF4:
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:154:- TuringBus already owns runtime orchestration (`src/bus.rs:53` + per CO1.7-impl). Sequencer is a runtime-orchestration peer of Kernel, not nested inside it.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:155:- Kernel `src/kernel.rs:5-6` has explicit warning against domain-specific terms; the documented "pure topology" role (`:15-17`) is preserved by NOT adding state-driver fields.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:156:- STEP_B Phase 0 less-invasive-alternative test: TuringBus-only is strictly simpler than TuringBus + Kernel coupled changes.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:163:    // ... existing fields including kernel: Kernel ...
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:165:    /// NEW (CO1.7-extra D3): typed-tx Sequencer; None when bus runs in legacy
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:167:    /// Marked serde-skip if TuringBus has serde derives (Sequencer holds
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:170:    pub sequencer: Option<Arc<Sequencer>>,
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:174:    pub fn new(kernel: Kernel, config: BusConfig) -> Self {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:175:        Self { /* ...existing..., */ sequencer: None }
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:178:    /// NEW: opt-in constructor that wires a typed-tx Sequencer alongside the legacy ledger.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:179:    pub fn with_sequencer(kernel: Kernel, config: BusConfig, sequencer: Arc<Sequencer>) -> Self {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:180:        Self { /* ...existing..., */ sequencer: Some(sequencer) }
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:185:    /// Sequencer::run driver loop.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:186:    pub async fn submit_typed_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:187:        match self.sequencer.as_ref() {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:197:`src/state/sequencer.rs` (round-2 MF6: manual Debug impl, NOT derive — Sequencer holds `Arc<Ed25519Keypair>` at line 199 and `Ed25519Keypair` intentionally has no Debug derive at `src/bottom_white/ledger/system_keypair.rs:282-284`; blanket derive fails to compile):
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:200:// src/state/sequencer.rs (additive — manual Debug impl for TuringBus.Debug propagation through Arc<Sequencer>)
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:201:impl std::fmt::Debug for Sequencer {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:204:        // satisfies Debug requirements for Arc<Sequencer> propagation.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:205:        f.debug_struct("Sequencer").finish_non_exhaustive()
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:215:1. Branch A (`step-b-co1.7-extra-A`): edits `src/bus.rs` per § 2.1 (1 field + 1 constructor variant + 1 forwarder method). Also adds the manual `Debug` impl on `Sequencer` in `src/state/sequencer.rs` (NOT STEP_B-restricted; lands alongside for compile coherence).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:221:Gemini Q5 r2 noted "Kernel placement creates forward-compat hazard of Kernel bloat". The TuringBus placement avoids this hazard entirely — Kernel stays at "pure topology" role; future stateful runtime drivers (e.g., a hypothetical CO1.x event router) would land at TuringBus level alongside Sequencer, which is the natural runtime-orchestrator role for TuringBus to own. No further justification needed beyond Codex Q-7 + Gemini Q5 convergence.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:295:### 3.3 `tests/co1_7_extra_sequencer_head_t_advancement.rs` (NEW — round-2 MF2 closure)
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:337:    turingosv4::state::sequencer::advance_head_t(&mut q, &writer);
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:353:    turingosv4::state::sequencer::advance_head_t(&mut q, &writer);
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:367:2. **3 of 4 D4 tests** (`replay_full_transition_state_root`, `system_signature_verifies_via_canonical_message`, un-ignore `sequencer_serial_replay_byte_identity`) — all require D1 to actually commit; deferred with D1 to future CO1.7.5.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:381:| Q1' Sequencer Debug derive completeness (round-1 surfaced) | **Closed by round-2 MF6** — manual `impl Debug for Sequencer` with `f.debug_struct("Sequencer").finish_non_exhaustive()`; `#[derive(Debug)]` not viable because `Arc<Ed25519Keypair>` field has no Debug derive. Codex Q-5 confirms `finish_non_exhaustive()` leaks no keypair / QState / CAS contents. |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:395:**Pre-implementation gate**: spec must reach PASS/PASS before any code in `src/state/sequencer.rs` D2 lines, `src/bus.rs` forwarder, `src/kernel.rs` field, or `src/bottom_white/ledger/transition_ledger.rs` trait method is written. Per CLAUDE.md "Audit Standard".
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:403:  - D2 (head_t close: `advance_head_t` helper + apply_one stage 9 patch + required trait method + 2 impl declarations + stale-comment updates at sequencer.rs:180-184/:359-361): ~40-60 LoC
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:404:  - D3 (TuringBus field + with_sequencer constructor + submit_typed_tx forwarder + manual Sequencer Debug impl): ~50-80 LoC across single STEP_B-restricted file (bus.rs) + 1 supporting file (sequencer.rs Debug impl)
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:414:3. **D2 logic is extracted into `advance_head_t` helper** (round-2 MF2 closure). The extraction adds zero behavior change but makes D2 directly testable via mock writer (without injecting dispatch_transition into Sequencer).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:415:4. **TuringBus owns Sequencer directly** (round-2 MF4) — not nested through Kernel. Kernel preserves "pure topology" doctrine (`src/kernel.rs:5-6`+`:15-17`) and stays UNTOUCHED by this atom. STEP_B becomes single-file ceremony on `src/bus.rs`.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:416:5. **Manual Sequencer Debug impl** (round-2 MF6) — `#[derive(Debug)]` fails because `Arc<Ed25519Keypair>` field has no Debug (system_keypair.rs:282-284 intentional); `finish_non_exhaustive()` is the safe replacement (Codex Q-5 confirmed no leak risk).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:418:7. **Most of CO1.1.4-pre1 ABI lock is irrelevant to this atom** — D1 (the part that uses TypedTx + TransitionError + SignalKind) is out of scope. CO1.7-extra only touches `LedgerWriter` trait + TuringBus wiring + Sequencer Debug impl; ABI lock untouched.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:419:8. **Stale Sequencer comments will be updated** during implementation (round-2 MF8): `src/state/sequencer.rs:180-184` + `:359-361` currently say "head_t deferred to CO1.7.5+"; CO1.7-extra implementation must update them to reflect the new D2 reality.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:420:9. **FC-trace requirements**: the new pub symbols introduced by CO1.7-extra implementation must carry doc-comment `/// TRACE_MATRIX <FC-id>: <role>` backlinks per CLAUDE.md "Alignment Standard". Set: `LedgerWriter::head_commit_oid_hex` + `advance_head_t` helper (→ § 5 L4 sequencer post-commit head_t wiring); `TuringBus.sequencer` field + `TuringBus::with_sequencer` + `TuringBus::submit_typed_tx` (→ § 5.2.1 single-writer entry-point).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:432:| S3 | Kernel UNTOUCHED by this atom (round-2 MF4) | `grep -n 'use crate::ledger::' src/kernel.rs && grep -L 'sequencer' src/kernel.rs` | legacy ledger import present; no "sequencer" reference (Kernel stays at pure topology) |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:433:| S4 | Sequencer struct exists at sequencer.rs:190 | `grep -n 'pub struct Sequencer' src/state/sequencer.rs` | one hit at line 190 |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:434:| S5 | Ed25519Keypair has no Debug derive (forces manual Sequencer Debug impl per MF6) | `grep -B5 'pub struct Ed25519Keypair' src/bottom_white/ledger/system_keypair.rs` | no `#[derive(Debug` precedes struct line |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:436:| S7 | Wallet (`src/sdk/tools/wallet.rs`) untouched | `grep -c 'transition_ledger\|state::sequencer\|TypedTx' src/sdk/tools/wallet.rs` | 0 hits |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:438:| S9 | Stale comment locations (round-2 MF8) | `grep -n 'CO1.7.5+\|deferred to CO1.7.5' src/state/sequencer.rs` | hits at lines 180-184 + 359-361 (to be updated by D2 implementation) |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:458:| S3 | Kernel UNTOUCHED | `use crate::ledger::{Node, NodeId, Tape, TapeError}` at kernel.rs:8; **0 hits** of "sequencer" anywhere in kernel.rs (pure topology preserved per round-2 MF4) | ✅ PASS |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:459:| S4 | Sequencer struct line | `pub struct Sequencer` at sequencer.rs:190 | ✅ PASS |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:460:| S5 | Ed25519Keypair has NO Debug derive | `#[derive(Zeroize, ZeroizeOnDrop)]` precedes `pub struct Ed25519Keypair` (system_keypair.rs:282-284); no Debug → forces manual Sequencer Debug impl per MF6 | ✅ PASS |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:464:| S9 | stale comments confirmed | sequencer.rs:178-184 (doc on apply_one Sequencer) + :357-361 (in apply_one stage 9 inline comment) — both still say "deferred to CO1.7.5+"; will be patched by D2 implementation per atom landing checklist | ✅ PASS (with minor line cite refinement vs v1.1 spec's "180-184 + 359-361" — actual lines 178-184 + 357-361; spec patched in this commit) |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:466:| S11 | cargo baseline | check pass; `239 passed; 0 failed; 1 ignored` (the ignored test is `sequencer_serial_replay_byte_identity`, deferred to future CO1.7.5 atom) | ✅ PASS |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:480:- **MF4** Sequencer placement: TuringBus owns directly (NOT nested through Kernel). Kernel UNTOUCHED. STEP_B becomes single-file ceremony (§ 2.1 + § 2.2).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:482:- **MF6** Sequencer Debug: manual `impl Debug` with `finish_non_exhaustive()` (Ed25519Keypair has no Debug derive at system_keypair.rs:282-284 — blanket derive fails) (§ 2.1)
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:484:- **MF8** stale Sequencer comments (sequencer.rs:178-184 + :357-361) added to atom landing checklist (§ 1.1 + § 8 ack #8)

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '43,58p;304,316p'; nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '276,288p'; nl -ba src/bottom_white/cas/store.rs | sed -n '155,205p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '190,250p;735,805p'" in /home/zephryj/projects/turingosv4
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
/bin/bash -lc "nl -ba src/kernel.rs | sed -n '1,70p'" in /home/zephryj/projects/turingosv4
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

exec
/bin/bash -lc "rg -n 'head_commit_oid_hex|advance_head_t|pub\\(crate\\) fn advance_head_t|impl LedgerWriter for|pub trait LedgerWriter|struct MockLedgerWriter|tests/co1_7_extra' handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md src tests || true" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:43:1. **D2** — `q.head_t = state::q_state::NodeId(commit_oid_hex)` after `writer.commit(&entry)` returns Ok; adds 1 trait method `LedgerWriter::head_commit_oid_hex` with mandatory-override design pattern (Q1 synthesis from round-1).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:66:The D2 logic is extracted into a small helper `advance_head_t(q, writer)` callable from `apply_one` stage 9 AND directly testable by the new `tests/co1_7_extra_head_t_advancement.rs` integration test (round-2 MF2 closure). Helper extraction adds zero behavior change — `apply_one` still executes identical logic.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:76:/// (writer is &dyn so behavior depends only on writer's head_commit_oid_hex
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:78:pub(crate) fn advance_head_t(q: &mut QState, writer: &dyn LedgerWriter) {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:79:    if let Some(commit_oid_hex) = writer.head_commit_oid_hex() {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:94:advance_head_t(&mut *q_w, &**writer_w);
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:97:**Stale comments must be updated** (round-2 MF8 — Codex Q-8 finding): `src/state/sequencer.rs:180-184` + `:359-361` currently say "head_t mutation deferred to CO1.7.5+". CO1.7-extra implementation MUST update these comments to reflect "head_t closed by CO1.7-extra D2 via `advance_head_t` helper". Added to § 9 atom landing checklist.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:101:**Atomicity** (per Codex Q-B + round-2 MF9 wording correction): under acquired `q_w` + `writer_w` write locks, after `writer_w.commit(&entry)?` returns `Ok`, the remaining operations are an `AtomicU64::store` (infallible), a plain `*q_w = q_next` move (infallible), and `advance_head_t` (infallible). For writers whose `head_commit_oid_hex` returns `Some` (Git2LedgerWriter), this is a **post-commit non-failing best-effort head binding** — `q.head_t` advances atomically with `ledger_root_t` and `next_logical_t`. For writers returning `None` (InMemoryLedgerWriter), `advance_head_t` is **explicit no-op preservation** — `q.head_t` stays at its prior value (which equals `q_next.head_t` after the `*q_w = q_next` move because CO1.7 K3 v1.2 forbids transition bodies from mutating head_t).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:108:pub trait LedgerWriter: Send + Sync {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:125:    fn head_commit_oid_hex(&self) -> Option<String>;
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:128:impl LedgerWriter for Git2LedgerWriter {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:129:    fn head_commit_oid_hex(&self) -> Option<String> {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:135:impl LedgerWriter for InMemoryLedgerWriter {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:138:    fn head_commit_oid_hex(&self) -> Option<String> {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:227:Three tests, **flat-named in `tests/`** (round-2 MF5 — Cargo auto-discovery requires flat naming or a `tests/co1_7_extra/main.rs` harness; v1.1 chooses flat naming for simplicity):
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:229:### 3.1 `tests/co1_7_extra_cas_payload_round_trip.rs`
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:257:### 3.2 `tests/co1_7_extra_git2_writer_head_oid_defense.rs`
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:289:        writer.head_commit_oid_hex().is_some(),
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:295:### 3.3 `tests/co1_7_extra_sequencer_head_t_advancement.rs` (NEW — round-2 MF2 closure)
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:297:Tests the actual D2 code path via the `advance_head_t` helper extraction:
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:300://! CO1.7-extra D2: verifies advance_head_t correctly advances q.head_t
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:307:/// Mock LedgerWriter that returns a configurable head_commit_oid_hex value.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:309:struct MockLedgerWriter {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:314:impl LedgerWriter for MockLedgerWriter {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:323:    fn head_commit_oid_hex(&self) -> Option<String> {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:329:fn advance_head_t_writes_node_id_when_writer_returns_some() {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:337:    turingosv4::state::sequencer::advance_head_t(&mut q, &writer);
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:345:fn advance_head_t_preserves_node_id_when_writer_returns_none() {
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:353:    turingosv4::state::sequencer::advance_head_t(&mut q, &writer);
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:380:| Q1 `head_commit_oid_hex` default impl (round-1 open) | **Closed by round-2 MF3** — trait method is REQUIRED (no default); compiler enforces every impl declares (§ 1.2). Both audits' safety arguments satisfied. |
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:403:  - D2 (head_t close: `advance_head_t` helper + apply_one stage 9 patch + required trait method + 2 impl declarations + stale-comment updates at sequencer.rs:180-184/:359-361): ~40-60 LoC
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:413:2. **`head_commit_oid_hex` is a NEW REQUIRED trait method** (no default impl; round-2 MF3). Compiler enforces every LedgerWriter impl declares; both `Git2LedgerWriter` and `InMemoryLedgerWriter` get explicit declarations in § 1.2.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:414:3. **D2 logic is extracted into `advance_head_t` helper** (round-2 MF2 closure). The extraction adds zero behavior change but makes D2 directly testable via mock writer (without injecting dispatch_transition into Sequencer).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:420:9. **FC-trace requirements**: the new pub symbols introduced by CO1.7-extra implementation must carry doc-comment `/// TRACE_MATRIX <FC-id>: <role>` backlinks per CLAUDE.md "Alignment Standard". Set: `LedgerWriter::head_commit_oid_hex` + `advance_head_t` helper (→ § 5 L4 sequencer post-commit head_t wiring); `TuringBus.sequencer` field + `TuringBus::with_sequencer` + `TuringBus::submit_typed_tx` (→ § 5.2.1 single-writer entry-point).
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:478:- **MF2** D2 testability: `advance_head_t(q, writer)` helper extracted from apply_one stage 9 → makes D2 directly testable via mock writer (§ 1.1 + new test § 3.3)
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:479:- **MF3** trait method `head_commit_oid_hex` becomes REQUIRED (no default impl); Rust compiler enforces every LedgerWriter declares (§ 1.2). Both audits' safety arguments satisfied via this third-option synthesis.
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:481:- **MF5** test harness: flat-named `tests/co1_7_extra_*.rs` for Cargo auto-discovery (§ 3 file paths)
src/bottom_white/ledger/transition_ledger.rs:189:pub trait LedgerWriter: Send + Sync {
src/bottom_white/ledger/transition_ledger.rs:227:impl LedgerWriter for InMemoryLedgerWriter {
src/bottom_white/ledger/transition_ledger.rs:718:impl LedgerWriter for Git2LedgerWriter {

codex
A stronger issue emerged: § 1.1 makes `advance_head_t` `pub(crate)`, but § 3.3 puts the test in `tests/` and calls it through the crate public path. Rust integration tests cannot access `pub(crate)` items, so that test as specified will not compile. I’m checking whether the module is publicly exported, because if it is, the fix is likely just changing the helper visibility to `pub`.
exec
/bin/bash -lc "nl -ba src/state/mod.rs | sed -n '1,80p'" in /home/zephryj/projects/turingosv4
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
/bin/bash -lc "rg -n 'pub mod state|mod state|pub mod sequencer|mod sequencer' src/lib.rs src/state/mod.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/mod.rs:16:pub mod sequencer;
src/lib.rs:12:pub mod state;

codex
I’m also checking a small Rust coercion detail in the stage-9 snippet. The spec calls `advance_head_t(&mut *q_w, &**writer_w)`; if that does not type-check against `RwLockWriteGuard<dyn LedgerWriter>`, it is another direct implementation blocker rather than just prose drift.
exec
/bin/bash -lc "rustc --edition=2021 - <<'RS'
use std::sync::{Arc, RwLock};
trait T {}
struct S;
impl T for S {}
fn f(_: &dyn T) {}
fn main() {
    let x: Arc<RwLock<dyn T>> = Arc::new(RwLock::new(S));
    let g = x.write().unwrap();
    f(&**g);
}
RS" in /home/zephryj/projects/turingosv4
 exited 1 in 204ms:
error[E0614]: type `dyn T` cannot be dereferenced
 --> <anon>:9:8
  |
9 |     f(&**g);
  |        ^^^ can't be dereferenced

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0614`.

exec
/bin/bash -lc "rustc --edition=2021 - <<'RS'
use std::sync::{Arc, RwLock};
trait T {}
struct S;
impl T for S {}
fn f(_: &dyn T) {}
fn main() {
    let x: Arc<RwLock<dyn T>> = Arc::new(RwLock::new(S));
    let g = x.write().unwrap();
    f(&*g);
}
RS" in /home/zephryj/projects/turingosv4
 succeeded in 251ms:
exec
/bin/bash -lc "rg -n '"'^name'"\\s*=|\\[package\\]' Cargo.toml" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
1:[package]
2:name = "turingosv4"

exec
/bin/bash -lc "rg -n 'combined STEP_B|TuringBus \\+ Kernel|Kernel.*field|kernel\\.sequencer|src/kernel\\.rs"'` field|one substrate-independent|3 tests|single-file|UNTOUCHED'"' handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
14:**Single sentence**: close the G-1 carry-forward `q.head_t = NodeId(commit_oid_hex)` after `Git2LedgerWriter.commit`, perform combined STEP_B ceremony adding a Sequencer entry-point on TuringBus + Kernel, and ship one substrate-independent CAS round-trip test — leaving transition function bodies + replay byte-identity to a future CO1.7.5 atom that depends on the Wave-2 substrate (CO P2.x family).
44:2. **D3** — Single-file STEP_B ceremony adds `Option<Arc<Sequencer>>` field + `with_sequencer` constructor + `submit_typed_tx` forwarder method to `TuringBus` (note: type is `TuringBus`, not `Bus`, per `src/bus.rs:53`). Sequencer lives in TuringBus directly (not nested through Kernel) per round-2 MF4 — Kernel preserves "pure topology" doctrine and stays UNTOUCHED by this atom.
99:**NodeId disambiguation**: two `NodeId` types coexist — legacy `pub type NodeId = String` at `src/ledger.rs:13` (imported by TuringBus + Kernel for the legacy ledger event API) and new `pub struct NodeId(pub String)` at `src/state/q_state.rs:49`. `q.head_t` is typed as the new tuple-struct (`q_state.rs:311`); D2 constructs the new variant exclusively (legacy String alias is unused here).
155:- Kernel `src/kernel.rs:5-6` has explicit warning against domain-specific terms; the documented "pure topology" role (`:15-17`) is preserved by NOT adding state-driver fields.
156:- STEP_B Phase 0 less-invasive-alternative test: TuringBus-only is strictly simpler than TuringBus + Kernel coupled changes.
195:`src/kernel.rs`: **UNTOUCHED** by CO1.7-extra. "Pure topology" doctrine preserved.
360:Total: 3 tests across 3 flat-named integration test files.
395:**Pre-implementation gate**: spec must reach PASS/PASS before any code in `src/state/sequencer.rs` D2 lines, `src/bus.rs` forwarder, `src/kernel.rs` field, or `src/bottom_white/ledger/transition_ledger.rs` trait method is written. Per CLAUDE.md "Audit Standard".
405:  - D4 (3 tests with mock LedgerWriter + inline LedgerEntry fixture): ~120-160 LoC
415:4. **TuringBus owns Sequencer directly** (round-2 MF4) — not nested through Kernel. Kernel preserves "pure topology" doctrine (`src/kernel.rs:5-6`+`:15-17`) and stays UNTOUCHED by this atom. STEP_B becomes single-file ceremony on `src/bus.rs`.
432:| S3 | Kernel UNTOUCHED by this atom (round-2 MF4) | `grep -n 'use crate::ledger::' src/kernel.rs && grep -L 'sequencer' src/kernel.rs` | legacy ledger import present; no "sequencer" reference (Kernel stays at pure topology) |
458:| S3 | Kernel UNTOUCHED | `use crate::ledger::{Node, NodeId, Tape, TapeError}` at kernel.rs:8; **0 hits** of "sequencer" anywhere in kernel.rs (pure topology preserved per round-2 MF4) | ✅ PASS |
480:- **MF4** Sequencer placement: TuringBus owns directly (NOT nested through Kernel). Kernel UNTOUCHED. STEP_B becomes single-file ceremony (§ 2.1 + § 2.2).
492:3. then CO1.7-extra-impl (D2 helper extraction + apply_one patch + trait method + TuringBus single-file STEP_B + 3 tests + stale-comment update)

exec
/bin/bash -lc "rg -n 'head_t = NodeId|SignalBundle 4-variant|supersedes STATE|head_t mutation deferred|commit_sha|head_commit_oid\\(|pub type NodeId' handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md src/ledger.rs src/state/q_state.rs src/bottom_white/ledger/transition_ledger.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/ledger/transition_ledger.rs:673:    /// CO1.7.5+ `head_t` wiring uses this to surface commit_sha alongside Hash.
src/bottom_white/ledger/transition_ledger.rs:674:    pub fn head_commit_oid(&self) -> Option<git2::Oid> {
src/bottom_white/ledger/transition_ledger.rs:1046:        assert!(w.head_commit_oid().is_none());
src/bottom_white/ledger/transition_ledger.rs:1061:        let oid_1 = w.head_commit_oid().expect("head after 1");
src/bottom_white/ledger/transition_ledger.rs:1066:        let oid_2 = w.head_commit_oid().expect("head after 2");
src/bottom_white/ledger/transition_ledger.rs:1084:        let pre_oid = w.head_commit_oid();
src/bottom_white/ledger/transition_ledger.rs:1092:        assert_eq!(w.head_commit_oid(), pre_oid);
src/bottom_white/ledger/transition_ledger.rs:1107:            oid_after_two = w.head_commit_oid().expect("head");
src/bottom_white/ledger/transition_ledger.rs:1112:        assert_eq!(w2.head_commit_oid(), Some(oid_after_two));
src/ledger.rs:13:pub type NodeId = String;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:412:    q_next.head_t = NodeId::from_state_root(new_state_root);
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:25:| **R2-K3** | Spec § 3 / § 5 said "CO1.7 owns head_t = NodeId(commit_sha)" but `LedgerWriter::commit` returns `Hash` not commit SHA; v1.1 InMemoryLedgerWriter has no commit_sha to return at all → contradiction | head_t mutation explicitly **deferred to CO1.7.5+** (when Git2LedgerWriter exists and can return both Hash + commit SHA). v1.x ledger owns `ledger_root_t` only; `head_t` continues to be set elsewhere (currently QState placeholder; CO1.7.5 wiring concern). Spec § 0 / § 3 / § 5 updated. | Codex round-2 must-fix #2 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:42:| K3 | L4/L5 head_t ownership inconsistent (spec line 194 vs 276 disagreed) | CO1.7 owns `ledger_root_t` + commit-chain `head_t = NodeId(commit_sha)` only; L5 (CO1.8) owns `state_root_t` mutation; sequencer drops `head_t = NodeId::from_state_root(...)` line | Codex Q-E |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:63:- **Storage backend**: git2-rs commit chain (built on CO1.4 CAS); each LedgerEntry = one git commit on `refs/transitions/main`. **R2-K3**: head_t mutation deferred to CO1.7.5+ — v1.x ledger does NOT mutate `Q_t.head_t` directly. Once `Git2LedgerWriter::commit` exists and returns commit_sha alongside Hash, CO1.7.5 wiring will set `head_t = NodeId(commit_sha)` outside the L4 sequencer apply path.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:379:        // (when `Git2LedgerWriter::commit` is implemented and can return commit_sha
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:390:**Why no head_t mutation in apply_one (K3, revised v1.2)**: v1.x CO1.7 owns `ledger_root_t` only. CO1.8 owns `state_root_t`. **head_t mutation deferred to CO1.7.5+** when `Git2LedgerWriter` provides a commit_sha return alongside Hash; the InMemoryLedgerWriter used by the v1 skeleton has no commit_sha to expose, so the trait keeps a single `Hash` return and head_t wiring is a separate downstream concern. Sequencer never calls `NodeId::from_state_root(...)`.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:461:- **K3 (v1.2)**: `head_t = NodeId(commit_sha)` is the canonical convention WHEN head_t is wired (CO1.7.5+). v1.x sequencer does NOT mutate head_t — `Git2LedgerWriter` is needed to surface commit_sha. `NodeId::from_state_root(...)` is NOT used by L4 in any version.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:662:| K3 L4/L5 head_t ownership | Boundary clarified: CO1.7 owns ledger_root + commit-chain head_t (NodeId(commit_sha)); CO1.8 owns state_root | § 0, § 3, § 5 |

exec
/bin/bash -lc "rg -n 'pub enum SignalKind|BoolSignal|StatSignal|SignalBundle' handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md src/state/typed_tx.rs handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:324:**Out of scope for v1.x spec** (deferred per Codex Q5/NEW-5 round-3 PARTIAL acknowledgment): full golden fixture corpus + differential fuzzing seed + complete runner ABI for QState/SignalBundle/TransitionError. v1.4 freezes the SERIALIZATION RULE (bincode v2 big-endian + BTreeMap lex); fixtures + ABI land in **CO1.1.4-pre1** (canonical fixture corpus) + **CO1.7** (full ABI surface). This is an **explicit deferral** — not unresolved spec ambiguity. STEP_B branch A and branch B both implement the SAME bincode rule; per-tx digest matching is mechanical from v1.4. Full corpus generation is a downstream code task, not spec scope.
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:338:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:403:    let signals = SignalBundle {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:404:        boolean: vec![Signal::Boolean(BoolSignal::AcceptedAt(tx.tx_id))],
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:406:            Signal::Statistical(StatSignal::PriceUpdate(price_for(tx.task_id, q_next.economic_state_t.price_index_t))),
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:407:            Signal::Statistical(StatSignal::ReputationDelta(tx.agent_id, +reputation_delta(tx))),
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:434:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:469:    let signals = SignalBundle {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:470:        boolean: vec![Signal::Boolean(BoolSignal::VerifiedAt(tx.tx_id))],
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:471:        statistical: vec![Signal::Statistical(StatSignal::ReputationDelta(tx.verifier_agent, +verify_reputation_delta(tx, target)))],
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:485:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:563:    let signals = SignalBundle {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:564:        boolean: vec![Signal::Boolean(BoolSignal::ChallengeUpheld(tx.tx_id))],
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:566:            Signal::Statistical(StatSignal::ReputationDelta(target.solver, -slash_reputation_delta())),
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:567:            Signal::Statistical(StatSignal::ReputationDelta(tx.challenger_agent, +challenge_reputation_delta())),
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:582:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:627:    Ok((q_next, SignalBundle::empty()))
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:640:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:703:    Ok((q_next, SignalBundle::finalize(claim_id, reward)))
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:718:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:761:    let signals = SignalBundle::task_expired(tx.task_id, bounty);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:832:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:855:    let signals = SignalBundle::terminal_summary(&summary);
src/state/typed_tx.rs:821:// SignalBundle — minimal v1 typed shape (CO1.7.5 + CO1.9 enrich it later)
src/state/typed_tx.rs:830:pub struct SignalBundle {
src/state/typed_tx.rs:834:/// Discriminator over the spec § 3 pseudocode's `SignalBundle::*` constructors.
src/state/typed_tx.rs:838:/// through `SignalBundle::finalize`).
src/state/typed_tx.rs:840:pub enum SignalKind {
src/state/typed_tx.rs:862:impl SignalBundle {
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:14:| **P11** | `SignalKind::Finalize` and `SignalBundle::finalize` still used `TxId` (v1.1 P2 missed call site) | `SignalKind::Finalize.claim_id: ClaimId` + `SignalBundle::finalize(claim_id: ClaimId, ...)` | R2-1 (Codex Q-2) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:47:**Why this atom exists**: spec § 2.5 of `STATE_TRANSITION_SPEC_v1_2026-04-27.md` explicitly deferred "full ABI surface for QState/SignalBundle/TransitionError" to CO1.7. CO1.7 spec § 0 places the per-kind tx schemas in `STATE_TRANSITION_SPEC § 1` ("frozen on paper, not yet in code"). When CO1.7-impl A1 (Git2LedgerWriter, commit `a03cc52`) shipped, downstream A2 (TypedTx + dispatch_transition) discovered ~30 supporting schema types are required but **none of them exist in code** — only `MicroCoin` is defined. This atom defines that ABI surface in isolation under its own dual-audit gate, per the project's per-atom audit principle (CLAUDE.md "Audit Standard").
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:77:- **`SignalBundle` typed shape** — STATE spec uses `SignalBundle::empty()` / `::finalize(...)` / `::task_expired(...)` / `::terminal_summary(...)` constructors. v1 of this atom emits a minimal typed `SignalBundle` (single enum-like discriminator + payload) sufficient for CO1.7-impl to compile; full event-stream design lands in CO1.9 L6 signal indices.

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '821,872p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   821	// SignalBundle — minimal v1 typed shape (CO1.7.5 + CO1.9 enrich it later)
   822	// ────────────────────────────────────────────────────────────────────────────
   823	
   824	/// TRACE_MATRIX STATE § 3 — tape-emitted signal bundle. v1 minimal: a single
   825	/// enum variant per spec call site in § 3 pseudocode (`empty` /
   826	/// `finalize` / `task_expired` / `terminal_summary`). Full L6 signal-stream
   827	/// design is CO1.9. CO1.1.4-pre1 ships just enough shape for CO1.7-impl to
   828	/// compile and for CO1.7.5 transition bodies to construct each variant.
   829	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   830	pub struct SignalBundle {
   831	    pub kind: SignalKind,
   832	}
   833	
   834	/// Discriminator over the spec § 3 pseudocode's `SignalBundle::*` constructors.
   835	///
   836	/// **v1.2 round-2 closure (R2-1)**: `Finalize.claim_id` is `ClaimId` (was `TxId`
   837	/// in v1.1; round-2 caught the missed call site that leaked the old type
   838	/// through `SignalBundle::finalize`).
   839	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   840	pub enum SignalKind {
   841	    Empty,
   842	    Finalize {
   843	        claim_id: ClaimId,
   844	        reward: MicroCoin,
   845	    },
   846	    TaskExpired {
   847	        task_id: TaskId,
   848	        bounty_refunded: MicroCoin,
   849	    },
   850	    TerminalSummary {
   851	        run_id: RunId,
   852	        outcome: RunOutcome,
   853	    },
   854	}
   855	
   856	impl Default for SignalKind {
   857	    fn default() -> Self {
   858	        Self::Empty
   859	    }
   860	}
   861	
   862	impl SignalBundle {
   863	    pub fn empty() -> Self {
   864	        Self {
   865	            kind: SignalKind::Empty,
   866	        }
   867	    }
   868	    pub fn finalize(claim_id: ClaimId, reward: MicroCoin) -> Self {
   869	        Self {
   870	            kind: SignalKind::Finalize { claim_id, reward },
   871	        }
   872	    }

exec
/bin/bash -lc "nl -ba handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md | sed -n '70,115p;545,590p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    70	
    71	### Out of scope (explicit deferral)
    72	
    73	- **MetaTx + ancillaries** (`PredicatePatch`, `ToolPatch`, `JudgeSignature`, `HumanSignature`, `ConstitutionCheckProof`, `ReversibilityPlan`) — STATE spec § 1.6 declares MetaTx is **v4.1 only**; v4 emits `MetaProposalDraft` to L3 CAS, not L4. ⏭ deferred.
    74	- **Slash transition** — already deferred to CO P2.5 ChallengeCourt per CO1.7 spec K5.
    75	- **Per-kind transition function bodies** (`step_transition`, `verify_transition`, `challenge_transition`, `reuse_transition`, `finalize_reward_transition`, `task_expire_transition`, `emit_terminal_summary_transition`) — these consume the ABI defined here; they belong to **CO1.7.5** (the body atom).
    76	- **Sequencer + dispatch_transition + replay_full_transition** — these consume the ABI; they belong to CO1.7-impl **A2-A4** (post this atom).
    77	- **`SignalBundle` typed shape** — STATE spec uses `SignalBundle::empty()` / `::finalize(...)` / `::task_expired(...)` / `::terminal_summary(...)` constructors. v1 of this atom emits a minimal typed `SignalBundle` (single enum-like discriminator + payload) sufficient for CO1.7-impl to compile; full event-stream design lands in CO1.9 L6 signal indices.
    78	- **TransitionError full taxonomy** — v1 emits a minimal enum covering the variants invoked in spec § 3 pseudocode (`ClaimNotFound`, `ChallengeWindowStillOpen`, `AlreadySlashed`, `TaskNotFound`, `InvalidSystemSignature`, `StaleParent`, `TaskNotExpired`, `TaskHasOpenClaim`, `TerminalSummaryNotApplicable`, `NotYetImplemented`); per-stage enum proliferation is a CO1.7.5 concern.
    79	
    80	### What this atom is NOT replacing
    81	
    82	- `src/state/q_state.rs` (existing): keeps its existing types verbatim. CO1.1.4-pre1 only adds new types in `src/state/typed_tx.rs`.
    83	- `src/economy/money.rs` (existing): unchanged. `StakeMicroCoin` is a **newtype on `MicroCoin`** living in `src/economy/money.rs` (additive).
    84	
    85	### § 0.1 Cross-atom ordering gate (v1.1 NEW per Gemini Q4 round-1)
    86	
    87	**Constitutional concern**: CO1.7 LedgerEntry stores typed-tx payloads in L3 CAS via `tx_payload_cid: Cid`. The current shipped `CasStore::open()` initializes an empty in-memory index (CO1.4 store.rs:67); after process restart the CAS bytes are unrecoverable until the index is repopulated. This means **cold-replay of L4 cannot reconstruct typed payloads** — a direct Art. 0.2 (tape canonicality) violation if uncorrected.
    88	
    89	**Mitigation**: CAS index persistence is its own atom — **CO1.4-extra** — already named in CO1.7 spec § 0. CO1.4-extra adds index persistence (likely a sidecar JSONL or git-tag manifest) so cold-replay can recover payloads via `CasStore::get`.
    90	
    91	**Hard ordering for v1.1 PASS**:
    92	- CO1.7-impl A2 (Sequencer apply path) + A3 (dispatch_transition skeleton) may proceed against CO1.1.4-pre1 v1.1 PASS independently.
    93	- **CO1.7-impl A4 (replay_full_transition) MUST NOT ship before CO1.4-extra**. Until then, FullTransition replay errors with `CasMissing` after process restart (already documented in CO1.7 spec § 4 / `ReplayError::CasMissing`).
    94	- CO1.4-extra has its own dual-audit gate.
    95	
    96	This ordering is a **necessary condition for CO1.1.4-pre1 PASS** per round-1 Gemini Q4; documented here so future audits cannot reinterpret silence as approval.
    97	
    98	---
    99	
   100	## § 1 Module layout
   101	
   102	```
   103	src/state/
   104	├── mod.rs                       (existing; +pub mod typed_tx + re-exports)
   105	├── q_state.rs                   (existing; unchanged)
   106	└── typed_tx.rs                  (NEW; ~600-900 LoC; the ABI surface)
   107	
   108	src/economy/
   109	└── money.rs                     (existing; +pub struct StakeMicroCoin newtype + minimal impls)
   110	
   111	src/bottom_white/ledger/
   112	└── system_keypair.rs            (existing; serde_bytes_64 helper promoted to pub(crate)
   113	                                  so AgentSignature can re-use the [u8; 64] adapter)
   114	```
   115	

exec
/bin/bash -lc "rg -n 'SignalKind|SignalBundle' handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
14:| **P11** | `SignalKind::Finalize` and `SignalBundle::finalize` still used `TxId` (v1.1 P2 missed call site) | `SignalKind::Finalize.claim_id: ClaimId` + `SignalBundle::finalize(claim_id: ClaimId, ...)` | R2-1 (Codex Q-2) |
47:**Why this atom exists**: spec § 2.5 of `STATE_TRANSITION_SPEC_v1_2026-04-27.md` explicitly deferred "full ABI surface for QState/SignalBundle/TransitionError" to CO1.7. CO1.7 spec § 0 places the per-kind tx schemas in `STATE_TRANSITION_SPEC § 1` ("frozen on paper, not yet in code"). When CO1.7-impl A1 (Git2LedgerWriter, commit `a03cc52`) shipped, downstream A2 (TypedTx + dispatch_transition) discovered ~30 supporting schema types are required but **none of them exist in code** — only `MicroCoin` is defined. This atom defines that ABI surface in isolation under its own dual-audit gate, per the project's per-atom audit principle (CLAUDE.md "Audit Standard").
77:- **`SignalBundle` typed shape** — STATE spec uses `SignalBundle::empty()` / `::finalize(...)` / `::task_expired(...)` / `::terminal_summary(...)` constructors. v1 of this atom emits a minimal typed `SignalBundle` (single enum-like discriminator + payload) sufficient for CO1.7-impl to compile; full event-stream design lands in CO1.9 L6 signal indices.
285:This rule applies symmetrically to other ABI enums frozen in this atom: `TxStatus` / `RejectionClass` / `VerifyVerdict` / `RunOutcome` / `SafetyOrCreation` / `SignalKind` / `CanonicalMessage` / `TxKind` (transition_ledger.rs).

exec
/bin/bash -lc "nl -ba handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md | sed -n '260,292p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   260	- **I-CANON-F** (v1.1 NEW): BTreeMap / BTreeSet permutation independence — building the same struct via different insertion orders produces byte-identical bytes.
   261	- **I-CANON-G** (v1.1 NEW per C-1): each agent-signed and system-emitted typed-tx has a paired `*SigningPayload` struct + `canonical_digest()` with explicit domain prefix `b"turingosv4.<actor>.<purpose>.v1"`. Domain prefix bytes are part of the SHA-256 input. 6 distinct domains (work / verify / challenge agent + finalize_reward / task_expire / terminal_summary system) yield pairwise-distinct digests.
   262	
   263	### § 7.1 Codec wording fix (v1.1 P6 per Codex Q-D round-1)
   264	
   265	STATE_TRANSITION_SPEC § 2.5 v1.4 wording is **inaccurate** for the actual codec; this v1.1 spec corrects:
   266	
   267	| What § 2.5 said | What bincode-2 actually does |
   268	|---|---|
   269	| `Enum discriminant: u8 (variant index in declaration order)` | **u32 BE** ([bincode 2.0.1 src/features/serde/ser.rs:186](https://docs.rs/bincode/2.0.1/src/bincode/features/serde/ser.rs.html), [src/enc/impls.rs:68](https://docs.rs/bincode/2.0.1/src/bincode/enc/impls.rs.html)) under `with_fixed_int_encoding`. The variant index is encoded as `u32::to_be_bytes()`. |
   270	| `Strings serialized as UTF-8 with explicit length prefix u32-BE` | **u64 BE** length prefix (bincode encodes `usize` as u64 under `with_fixed_int_encoding`; [src/enc/impls.rs:128](https://docs.rs/bincode/2.0.1/src/bincode/enc/impls.rs.html)). The same applies to BTreeMap / BTreeSet / Vec lengths. |
   271	| `#[repr(u8)]` controls discriminant | **No** — `#[repr(u8)]` is a Rust language attribute affecting in-memory layout + raw cast (`as u8`) but does NOT control serde wire format. Codex caught this; spec language fixed. |
   272	
   273	**v1.1 decision**: keep u32 variants + u64 lengths; do NOT introduce a custom serde adapter to force u8 discriminants (which would force re-encoding of all existing fixtures + complicate forward-compat for >256 variants). The locked golden fixtures in `EXPECTED_HEX_*` reflect the actual u32/u64 codec.
   274	
   275	This wording fix is a **spec-only patch**; no code change required (the codec was already correct; only the description was wrong).
   276	
   277	### § 7.2 TransitionError additive-only commitment (v1.2 GR-2 per Gemini round-2)
   278	
   279	`TransitionError` variants in **v4 are additive-only**:
   280	- New variants MUST be APPENDED to the existing list (no insertion that would shift downstream variant indices).
   281	- Existing variants MUST NOT be reordered (bincode emits variant-index-as-u32-BE; reordering changes the wire format and invalidates locked golden fixtures).
   282	- Variant removal is NOT permitted within v4; deprecated error classes get a doc-comment "deprecated; replaced by X" but the variant stays.
   283	- Bumping the major version (v4 → v5) is the only path that allows non-additive changes; that triggers an ABI rotation cycle (re-audit + re-fixture + canonical re-encoding migration).
   284	
   285	This rule applies symmetrically to other ABI enums frozen in this atom: `TxStatus` / `RejectionClass` / `VerifyVerdict` / `RunOutcome` / `SafetyOrCreation` / `SignalKind` / `CanonicalMessage` / `TxKind` (transition_ledger.rs).
   286	
   287	### § 7.3 Domain-string rotation process (v1.2 GR-3 per Gemini round-2)
   288	
   289	If a future audit finds a security defect in a domain prefix (`b"turingosv4.<actor>.<purpose>.v1"`), rotation follows this discipline:
   290	
   291	1. **Add NEW constant** (`*.v2`) **in parallel**; do NOT delete the old `*.v1` constant.
   292	2. **Old `*.v1` MUST remain reachable in code** until the runtime can prove no in-flight tx still uses it (replay-window quiescence).

codex
One more compile detail: § 2.1 says to put `#[serde(skip)]` on `TuringBus.sequencer` only “if TuringBus has serde derives.” Current `TuringBus` has no derive. I’m confirming that this conditional wording is necessary, because an unconditional helper attribute would be a compile error.
exec
/bin/bash -lc "rustc --edition=2021 - <<'RS'
struct S { #[serde(skip)] x: u8 }
fn main() {}
RS" in /home/zephryj/projects/turingosv4
 exited 1 in 101ms:
exec
/bin/bash -lc "rg -n 'impl\\s+LedgerWriter\\s+for' src tests" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/ledger/transition_ledger.rs:227:impl LedgerWriter for InMemoryLedgerWriter {
src/bottom_white/ledger/transition_ledger.rs:718:impl LedgerWriter for Git2LedgerWriter {

exec
/bin/bash -lc 'git rev-parse --short HEAD && git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
a3952cf
 M rules/enforcement.log
?? handover/audits/CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md
?? handover/audits/GEMINI_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md
?? handover/audits/run_codex_co1_7_extra_round3_audit.sh
?? handover/audits/run_gemini_co1_7_extra_round3_audit.py
?? rust_out

codex
The repository is at HEAD `a3952cf`; I also noticed the quick `rustc` probe emitted a `rust_out` binary in the worktree. That file is my temporary artifact, so I’m removing only that before finalizing the audit.
exec
/bin/bash -lc 'rm -f rust_out && git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
 M rules/enforcement.log
?? handover/audits/CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md
?? handover/audits/GEMINI_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md
?? handover/audits/run_codex_co1_7_extra_round3_audit.sh
?? handover/audits/run_gemini_co1_7_extra_round3_audit.py

codex
# Codex CO1.7-extra Round-3 Audit

## Q1 MF1 § 0.4 Disposition Table Closure

PASS. The § 0.4 table is now factually correct: head_t supersession is enacted in CO1.7-extra D2, while SignalKind migrates to future CO1.7.5 ([spec:51-54](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:51)). The authority principle also holds ([spec:56](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:56)).

Source cross-check: STATE still has `NodeId::from_state_root` at [STATE:412](/home/zephryj/projects/turingosv4/handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:412); shipped `QState.head_t` is the tuple struct at [q_state.rs:49](/home/zephryj/projects/turingosv4/src/state/q_state.rs:49) and [q_state.rs:311](/home/zephryj/projects/turingosv4/src/state/q_state.rs:311). Current `SignalKind` is the minimal 4-variant shape at [typed_tx.rs:840-854](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:840).

## Q2 MF2 `advance_head_t` Helper Extraction

CHALLENGE.

The helper shape is logically correct, but not testable as specified. § 1.1 declares it `pub(crate)` ([spec:78](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:78)), while § 3.3 is a flat integration test calling `turingosv4::state::sequencer::advance_head_t` ([spec:295](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:295), [spec:337](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:337)). Integration tests cannot access `pub(crate)` items. Fix: make the helper `pub` with the required TRACE doc, or move the test into `src/state/sequencer.rs`; MF5’s flat integration-test choice points to `pub`.

Second blocker: the stage-9 call `advance_head_t(&mut *q_w, &**writer_w)` will not compile with current `Arc<RwLock<dyn LedgerWriter>>` ([sequencer.rs:201](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:201), [sequencer.rs:363-368](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:363)). `dyn LedgerWriter` cannot be dereferenced twice. It should be `&*writer_w` ([spec:94](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:94)).

The mock itself covers both Some and None paths correctly ([spec:329-357](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:329)).

## Q3 MF3 Required Trait Method

PASS. The trait method is required, with no default ([spec:108-125](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:108)). Current workspace has only two impls to patch: `InMemoryLedgerWriter` and `Git2LedgerWriter` ([transition_ledger.rs:227](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:227), [transition_ledger.rs:718](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:718)). A missing impl will be Rust E0046. The safety claims hold: Git2 explicitly maps `head_commit_oid()` to hex, InMemory explicitly returns `None` ([spec:128-140](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:128)).

## Q4 MF4 Sequencer Placement

CHALLENGE for residual spec drift, not design.

The main § 2 rewrite is correct: TuringBus owns the field/methods, Kernel is untouched, and STEP_B is single-file for `src/bus.rs` ([spec:153-156](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:153), [spec:160-190](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:160), [spec:195](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:195), [spec:212-217](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:212)). Source supports this: `TuringBus` is at [bus.rs:53](/home/zephryj/projects/turingosv4/src/bus.rs:53); Kernel’s pure-topology warning is at [kernel.rs:5-17](/home/zephryj/projects/turingosv4/src/kernel.rs:5).

Residual contradiction: the top single-sentence summary still says “combined STEP_B ceremony” and “TuringBus + Kernel” ([spec:14](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:14)). The pre-implementation gate also still mentions a `src/kernel.rs` field ([spec:395](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:395)). Those are stale MF4 leftovers.

## Q5 MF5-MF10 Small Fixes

MF5 PASS: flat test names are present ([spec:229](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:229), [spec:257](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:257), [spec:295](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:295)).

MF6 PASS: manual `Debug` uses `finish_non_exhaustive()` ([spec:201-205](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:201)); `Ed25519Keypair` has no Debug derive ([system_keypair.rs:282-284](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:282)).

MF7 PASS: `entry_at` is private inside module tests ([transition_ledger.rs:813](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:813)).

MF8 PASS: stale comment cites match current source ([sequencer.rs:180-184](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:180), [sequencer.rs:359-361](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:359)).

MF9 PASS: atomicity wording is corrected ([spec:101](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:101)).

MF10 PARTIAL: § 7 now says `~210-300 LoC` ([spec:406](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:406)), while the patch log still says `200-280` ([spec:486](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:486)). Not blocking, but inconsistent.

## Q6 New Defects In v1.1

Yes: two compile blockers and one stale MF4 contradiction.

Blocking:
- `pub(crate)` helper cannot be called from the specified integration test.
- `&**writer_w` does not compile for `RwLockWriteGuard<dyn LedgerWriter>`.

Non-blocking but should patch:
- stale Kernel references at spec lines 14 and 395.
- `#[serde(skip)]` must remain conditional. Current `TuringBus` has no serde derive ([bus.rs:53](/home/zephryj/projects/turingosv4/src/bus.rs:53)); adding `#[serde(skip)]` literally would be invalid unless a serde derive is added.

## Q7 Implementation Gating

Not PASS-gated yet. The design is implementable end-to-end after the small v1.1 spec patch: make `advance_head_t` publicly testable or move the test, change `&**writer_w` to `&*writer_w`, remove stale Kernel wording, and clarify the serde-skip conditional.

## **VERDICT**: CHALLENGE

## Top Issues

1. `advance_head_t` visibility contradicts the flat integration test plan.
2. Stage-9 snippet has a literal Rust compile error: `&**writer_w`.
3. MF4 still has stale top-level Kernel wiring references.

## Conviction

High. No foundational design flaw; these are concrete patch blockers.
2026-04-29T03:33:22.456314Z ERROR codex_core::session: failed to record rollout items: thread 019dd748-bfed-7dd1-961a-b559b987d39c not found
tokens used
140,559
# Codex CO1.7-extra Round-3 Audit

## Q1 MF1 § 0.4 Disposition Table Closure

PASS. The § 0.4 table is now factually correct: head_t supersession is enacted in CO1.7-extra D2, while SignalKind migrates to future CO1.7.5 ([spec:51-54](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:51)). The authority principle also holds ([spec:56](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:56)).

Source cross-check: STATE still has `NodeId::from_state_root` at [STATE:412](/home/zephryj/projects/turingosv4/handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:412); shipped `QState.head_t` is the tuple struct at [q_state.rs:49](/home/zephryj/projects/turingosv4/src/state/q_state.rs:49) and [q_state.rs:311](/home/zephryj/projects/turingosv4/src/state/q_state.rs:311). Current `SignalKind` is the minimal 4-variant shape at [typed_tx.rs:840-854](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:840).

## Q2 MF2 `advance_head_t` Helper Extraction

CHALLENGE.

The helper shape is logically correct, but not testable as specified. § 1.1 declares it `pub(crate)` ([spec:78](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:78)), while § 3.3 is a flat integration test calling `turingosv4::state::sequencer::advance_head_t` ([spec:295](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:295), [spec:337](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:337)). Integration tests cannot access `pub(crate)` items. Fix: make the helper `pub` with the required TRACE doc, or move the test into `src/state/sequencer.rs`; MF5’s flat integration-test choice points to `pub`.

Second blocker: the stage-9 call `advance_head_t(&mut *q_w, &**writer_w)` will not compile with current `Arc<RwLock<dyn LedgerWriter>>` ([sequencer.rs:201](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:201), [sequencer.rs:363-368](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:363)). `dyn LedgerWriter` cannot be dereferenced twice. It should be `&*writer_w` ([spec:94](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:94)).

The mock itself covers both Some and None paths correctly ([spec:329-357](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:329)).

## Q3 MF3 Required Trait Method

PASS. The trait method is required, with no default ([spec:108-125](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:108)). Current workspace has only two impls to patch: `InMemoryLedgerWriter` and `Git2LedgerWriter` ([transition_ledger.rs:227](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:227), [transition_ledger.rs:718](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:718)). A missing impl will be Rust E0046. The safety claims hold: Git2 explicitly maps `head_commit_oid()` to hex, InMemory explicitly returns `None` ([spec:128-140](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:128)).

## Q4 MF4 Sequencer Placement

CHALLENGE for residual spec drift, not design.

The main § 2 rewrite is correct: TuringBus owns the field/methods, Kernel is untouched, and STEP_B is single-file for `src/bus.rs` ([spec:153-156](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:153), [spec:160-190](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:160), [spec:195](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:195), [spec:212-217](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:212)). Source supports this: `TuringBus` is at [bus.rs:53](/home/zephryj/projects/turingosv4/src/bus.rs:53); Kernel’s pure-topology warning is at [kernel.rs:5-17](/home/zephryj/projects/turingosv4/src/kernel.rs:5).

Residual contradiction: the top single-sentence summary still says “combined STEP_B ceremony” and “TuringBus + Kernel” ([spec:14](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:14)). The pre-implementation gate also still mentions a `src/kernel.rs` field ([spec:395](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:395)). Those are stale MF4 leftovers.

## Q5 MF5-MF10 Small Fixes

MF5 PASS: flat test names are present ([spec:229](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:229), [spec:257](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:257), [spec:295](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:295)).

MF6 PASS: manual `Debug` uses `finish_non_exhaustive()` ([spec:201-205](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:201)); `Ed25519Keypair` has no Debug derive ([system_keypair.rs:282-284](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:282)).

MF7 PASS: `entry_at` is private inside module tests ([transition_ledger.rs:813](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:813)).

MF8 PASS: stale comment cites match current source ([sequencer.rs:180-184](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:180), [sequencer.rs:359-361](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:359)).

MF9 PASS: atomicity wording is corrected ([spec:101](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:101)).

MF10 PARTIAL: § 7 now says `~210-300 LoC` ([spec:406](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:406)), while the patch log still says `200-280` ([spec:486](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md:486)). Not blocking, but inconsistent.

## Q6 New Defects In v1.1

Yes: two compile blockers and one stale MF4 contradiction.

Blocking:
- `pub(crate)` helper cannot be called from the specified integration test.
- `&**writer_w` does not compile for `RwLockWriteGuard<dyn LedgerWriter>`.

Non-blocking but should patch:
- stale Kernel references at spec lines 14 and 395.
- `#[serde(skip)]` must remain conditional. Current `TuringBus` has no serde derive ([bus.rs:53](/home/zephryj/projects/turingosv4/src/bus.rs:53)); adding `#[serde(skip)]` literally would be invalid unless a serde derive is added.

## Q7 Implementation Gating

Not PASS-gated yet. The design is implementable end-to-end after the small v1.1 spec patch: make `advance_head_t` publicly testable or move the test, change `&**writer_w` to `&*writer_w`, remove stale Kernel wording, and clarify the serde-skip conditional.

## **VERDICT**: CHALLENGE

## Top Issues

1. `advance_head_t` visibility contradicts the flat integration test plan.
2. Stage-9 snippet has a literal Rust compile error: `&**writer_w`.
3. MF4 still has stale top-level Kernel wiring references.

## Conviction

High. No foundational design flaw; these are concrete patch blockers.
