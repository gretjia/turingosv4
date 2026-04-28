# Codex CO1.7-impl Bundle Round-1 Audit
**Date**: 2026-04-28
**Target**: A1+A2+A3+A4 + CO1.4-extra (all mid-implementation commits)
**Prompt size**: 313941 chars

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
session id: 019dd45e-6254-7961-8665-9a63a24e5877
--------
user
# Codex Audit — CO1.7-impl BUNDLE Round-1 (Pre-CO1.7.5 implementation gate)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-1 (parallel).

**Mandate**: round-1 dual external audit on the **CO1.7-impl bundle** (A1 + A2 + A3 + A4) + **CO1.4-extra** atom, all mid-implementation commits. Per CLAUDE.md "Audit Standard" + spec § 12: this PASS/PASS gates entry to CO1.7.5 (per-kind transition function bodies + STEP_B wiring to bus.rs/kernel.rs).

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## Atoms in this bundle

| Atom | Commit | LoC | Tests | Description |
|---|---|---|---|---|
| **A1** Git2LedgerWriter | `a03cc52` | ~525 | 5 | git2-rs commit chain on `refs/transitions/main`; tree blobs (payload_cid / signature / entry_canonical); deterministic author time (logical_t, 0); reopen_recovers_chain test for cold-restart capability |
| **A2** Sequencer | `609d8d5` | ~290 (in 430 incl. tests) | 5 | K1 dual counter (next_submit_id / next_logical_t); tokio mpsc bounded queue; apply_one 9 stages per spec § 3; K3 head_t deferred to CO1.7.5+ wiring; ApplyError wrapper (impl deviation from spec § 3 line 307 — TransitionError stays closed-taxonomy + additive-only per CO1.1.4-pre1 § 7.2) |
| **A3** dispatch_transition | `609d8d5` | ~30 | 1 | Exhaustive 7-variant enum match; every variant returns NotYetImplemented stub (CO1.7.5 fills) |
| **A4** replay_full_transition | `272fcf4` | ~140 + LedgerCasView trait | 4 (1 #[ignore]) | 9-stage full-mode replay; new ReplayError::Transition variant; LedgerCasView trait (DIV-4 closure); tests prove stages 1-6 PASS, stage 7 stubs; #[ignore]'d test for full state_root reconstruction (CO1.7.5) |
| **CO1.4-extra** | `b6b7574` | ~110 + tests | 5 | Sidecar JSONL CAS index persistence; closes Art 0.2 cold-replay gate per CO1.1.4-pre1 § 0.1 cross-atom ordering |

**Total**: ~1100 LoC + 20 conformance tests + 1 #[ignore]'d. **234/0 → 237/0 lib PASS**.

## Companion specs (frozen, read first)

- `CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` v1.2 — pre-implementation PASS/PASS (rounds 1-3)
- `CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` v1.2.2 — ABI surface PASS/PASS (rounds 1-5)
- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — frozen invariants + § 3 transition pseudocode

## What is at stake

- **PASS/PASS** unblocks **CO1.7.5** (per-kind transition function bodies + STEP_B wiring to bus.rs/kernel.rs); CO1.7.5 is the final L4 atom
- **CHALLENGE** triggers v1.1 patch round on the bundle
- **VETO** would mean a foundational design issue requiring re-spec

## Round-1 audit questions

**Q-A. A1 (Git2LedgerWriter) integration discipline**: tree-blob choice (payload_cid + signature + entry_canonical) vs original commit-message-body approach (which had a binary normalization bug). Is the deterministic author time (`(logical_t as i64, 0)`) sufficient for byte-identical commit OIDs across runs? Are the 5 tests load-bearing (open_recovers_chain proves cold-restart; what else might miss)?

**Q-B. A2 Sequencer K1 invariant + K3 deferred head_t**: K1 — submit advances submit_id; commit advances logical_t. Verify rejected-submission path does NOT advance logical_t (test `apply_one_stub_does_not_consume_logical_t` covers stub mode; what about future real-transition rejection path)? K3 — head_t mutation deferred to CO1.7.5+. Is leaving `q.head_t` un-mutated (still at QState::default empty string) during the entire CO1.7-impl runtime ACCEPTABLE for Art 0.4 (Q_t = ⟨q_t, HEAD_t, tape_t⟩) compliance? Or is this a gap that should at least set head_t to a placeholder pointing at the latest commit OID?

**Q-C. A2 Sequencer ApplyError vs spec § 3 line 307 deviation**: spec writes apply_one signature as `Result<LedgerEntry, TransitionError>`; impl widens to `Result<LedgerEntry, ApplyError>` with TransitionError as a wrapped variant. **Defensible vs spec drift?** Argument for: TransitionError stays closed-taxonomy (CO1.1.4-pre1 § 7.2 additive-only commitment); infra errors (CasError / KeypairError / LedgerWriterError) shouldn't pollute it. Argument against: spec is authoritative, deviations need explicit closure.

**Q-D. A2 Sequencer concurrency / panic safety**: apply_one is sync and acquires multiple `RwLock` write guards (Q_t + writer). What happens if apply_one panics mid-way? Q_t lock poisoned (we map to ApplyError::QStateLockPoisoned but the sequencer task continues). Is this acceptable? Is there a runtime concern about run() being a long-running async task that loses progress on panic?

**Q-E. A3 dispatch_transition exhaustive match contract**: every variant returns NotYetImplemented. Is this the right pattern for "transition body not yet implemented", or should the stubs use unimplemented!() macro for clearer crash semantics? (Tests verify the Err return path; CO1.7.5 fills bodies preserving the match arms.)

**Q-F. A4 replay_full_transition staging**: 9 stages per spec § 4. Implementation order (1-2-3 chain checks → 4 sig verify → 5 CAS lookup → 6 decode → 7 dispatch → 8 state_root match → 9 ledger_root match) — defensible? Or should sig verify come AFTER CAS lookup (cheaper to fail-fast on missing payload)?

**Q-G. A4 LedgerCasView trait**: narrow read-only interface. Necessary for testability + future MetaCas backend, OR over-engineering since CasStore is the only impl?

**Q-H. CO1.4-extra sidecar JSONL discipline**: append BEFORE in-memory insert (so crash mid-write keeps runtime consistent — durable+memory both present, or neither). Strict mode on corrupted JSONL line (returns IndexParse error vs skip-and-warn). Both correct?

**Q-I. CO1.4-extra durability gap on idempotent put**: idempotent put short-circuits before sidecar write (correct — content is already durable from prior put). But what about a partial write that left the in-memory index updated but sidecar not flushed? Current code appends THEN inserts — addresses this. Confirm no other gap.

**Q-J. Cross-atom A2↔A4 consistency**: Sequencer.apply_one stages 5-9 (sign + ledger_root fold + commit) vs replay_full_transition stages 4-9 (verify + decode + dispatch + state_root + ledger_root). Are these symmetric — does every byte the sequencer writes get verified by replay? Specifically: signing payload digest pre-image must be EXACTLY what apply_one constructed at stage 5 vs what replay reconstructs at stage 4.

**Q-K. New defects independent of catalog**:
- Type errors that cargo check missed?
- Spec ↔ code parity drift?
- Test gaps: anything from STATE_TRANSITION_SPEC § 4 (27 invariants) that CO1.7-impl bundle should provide a witness for but doesn't?
- Doc-comment hygiene (lesson #11 from CO1.1.4-pre1)?

**Q-L. Implementation gating**: with bundle at 237/0 PASS, is CO1.7.5 (per-kind transition bodies + STEP_B bus.rs/kernel.rs wiring) implementable end-to-end against this bundle? Specific blockers to call out.

## Output format

# Codex CO1.7-impl Bundle Round-1 Audit
## Q-A A1 Git2LedgerWriter
## Q-B A2 K1 + K3 deferred head_t
## Q-C A2 ApplyError vs spec § 3 line 307
## Q-D A2 panic safety
## Q-E A3 stub pattern
## Q-F A4 staging order
## Q-G A4 LedgerCasView trait
## Q-H CO1.4-extra sidecar discipline
## Q-I CO1.4-extra durability gap
## Q-J cross-atom A2↔A4 symmetry
## Q-K New defects
## Q-L Implementation gating
## **VERDICT**: PASS / CHALLENGE / VETO
## Top must-fix (if CHALLENGE)
## Conviction (low/med/high)

Be rigorous. Cite line numbers. Real defects = CHALLENGE; foundational design flaw = VETO; clean bundle = PASS.

---



# CO1.7 spec v1.2 (already PASS/PASS)

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

# CO1.1.4-pre1 spec v1.2.2 (already PASS/PASS)

# CO1.1.4-pre1 — Typed Tx ABI Surface (v1.2)

**Status**: v1.2 — round-2 returned PASS (Gemini, high) + CHALLENGE (Codex, high; 4 patch-mechanical defects). Conservative merged CHALLENGE. v1.2 closes the 4 must-fix items (P11-P14 below) + 1 secondary (P15) + 3 Gemini recommendations (GR-1/2/3). Awaiting round-3.
**Status (v1.1)**: round-1 dual audit returned CHALLENGE/CHALLENGE; v1.1 closed 10 patches (P1-P10).
**Status (v1)**: v1 DRAFT, post-CO1.7 PASS/PASS gate (2026-04-28).
**Author**: ArchitectAI (Claude); session 2026-04-28 (continued).
**Round-1 verdicts**: `handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md` (CHALLENGE/high) + `handover/audits/GEMINI_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md` (CHALLENGE/high); merged in `handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md`.
**Round-2 verdicts**: `handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md` (CHALLENGE/high) + `handover/audits/GEMINI_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md` (PASS/high); merged in `handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R2_2026-04-28.md`.

## v1.2 patch log (vs. v1.1) — round-2 closure

| ID | v1.1 issue | v1.2 fix | Source |
|---|---|---|---|
| **P11** | `SignalKind::Finalize` and `SignalBundle::finalize` still used `TxId` (v1.1 P2 missed call site) | `SignalKind::Finalize.claim_id: ClaimId` + `SignalBundle::finalize(claim_id: ClaimId, ...)` | R2-1 (Codex Q-2) |
| **P12** | `FinalizeRewardTx.system_signature` and `TaskExpireTx.system_signature` retained but no `CanonicalMessage::FinalizeRewardSigning` / `::TaskExpireSigning` variants + no emitter fns. Dual-sign rationale (§ 4.2) not executable for 2 of 3 system txs | NEW `CanonicalMessage::FinalizeRewardSigning([u8; 32])` + `TaskExpireSigning([u8; 32])` variants (system_keypair.rs) + canonical_digest match arms + `transition_emitter::sign_finalize_reward` + `sign_task_expire` symmetric emitter fns | R2-2 (Codex Q-7) |
| **P13** | Spec drift: § 0 line 47 still said "TerminalSummaryTx in system_keypair.rs"; § 6 line 210 said "imported from system_keypair"; § 9 D-3 row still present despite v1.1 P7 claiming removal | All 3 stale references cleaned: § 0 lists TerminalSummaryTx in `state::typed_tx`; § 6 inline comment updated; § 9 D-3 row REMOVED (HTML comment marker placed in its slot) | R2-3 (Codex Q-3) |
| **P14** | Signing-payload tests not load-bearing: round-1 `signing_payload_domains_are_distinct` used different bodies (would pass even without domain prefix); `signing_payload_excludes_signature` only tested for WorkTx; no signing-payload golden hex | NEW `signing_payload_domain_prefix_is_load_bearing` test (identical 64-byte body across 6 domains → 6 pairwise-distinct digests; would FAIL without domain prefix). `signing_payload_excludes_signature` extended to all 6 signed tx kinds. NEW `signing_payload_golden_digests` test with locked SHA-256 hex per signing payload (6 EXPECTED_SIGNING_HEX_* constants). | R2-4 (Codex Q-9) |
| **P15** | BTreeMap permutation only covered BTreeSet (read_set); BTreeMap fields (predicate_results, failure_class_histogram) untested for permutation independence | NEW `typed_tx_btreemap_permutation_independence` test using `predicate_results.acceptance` (BTreeMap<PredicateId, BoolWithProof>) | Codex round-2 secondary (Q-5 caveat) |
| **GR-1** (Gemini PASS recommendation) | MetaTx domain prefix not reserved; v4.1 namespace might force domain rotation later | NEW `DOMAIN_AGENT_META_PROPOSAL: &[u8] = b"turingosv4.agent_sig.meta_proposal.v1"` constant (typed_tx.rs); marked `#[allow(dead_code)]` until v4.1 wires MetaTx | Gemini Q9 / GR-1 |
| **GR-2** (Gemini recommendation) | TransitionError additive-only commitment not stated | spec § 7.2 NEW: TransitionError variants in v4 are additive-only; never reorder; new variants append at the end | Gemini Q9 / GR-2 |
| **GR-3** (Gemini recommendation) | Domain-string rotation process not documented | spec § 7.3 NEW: domain rotation = new constant (`*.v2`) added in parallel; old `.v1` retained until all-replay window passes; bumping major version triggers a v2 spec round | Gemini Q9 / GR-3 |

5 must-fix patches + 1 secondary + 3 Gemini recommendations = **9 closures** integrated below.



## v1.1 patch log (vs. v1) — round-1 closure

| ID | v1 issue | v1.1 fix | Source |
|---|---|---|---|
| **P1** | AgentSignature reused 64-byte adapter without domain separation; comments implied "exclude signature" digest with no signing payload | NEW signing-payload structs (`WorkSigningPayload` / `VerifySigningPayload` / `ChallengeSigningPayload` / `FinalizeRewardSigningPayload` / `TaskExpireSigningPayload` / `TerminalSummarySigningPayload`) — each has explicit domain prefix (`b"turingosv4.<actor>.<purpose>.v1"`) prepended to bincode body bytes in `canonical_digest()`. Plus `to_signing_payload()` projection on each tx. | C-1 (Codex Q-E + Gemini Q7) |
| **P2** | `FinalizeRewardTx.claim_id: TxId` reused TxId, leaking ClaimsIndex impl into wire format | New `ClaimId(pub TxId)` newtype with `#[serde(transparent)]` (wire-identical to TxId; non-breaking); `FinalizeRewardTx.claim_id: ClaimId` now | C-3 (Codex Q-B) |
| **P3** | `TerminalSummaryTx` was 3-field placeholder living in `system_keypair.rs` (versus STATE § 1.5 8-field schema); locking the wrong shape into ABI | Migrated to `state::typed_tx::TerminalSummaryTx` with full 8-field STATE schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). `system_keypair` now signs an opaque `TerminalSummarySigning([u8; 32])` digest (same opaque-digest pattern as `LedgerEntrySigning`) — no `bottom_white ↔ state` circular dep. | C-3 (Codex Q-C must-fix-now) |
| **P4** | `TransitionError` had only 10 variants; STATE § 3 pseudocode invokes ~22 | Expanded to 22 variants: SignatureInvalid / StakeInsufficient / TargetWorkTxNotFound / TargetWorkTxNotVerifiable / ParentNotAcceptedYet / AcceptancePredicateFailed(PredicateId) / VerificationPredicateFailed(PredicateId) / SettlementPredicateFailed(PredicateId) / ChallengeWindowClosed / CounterexampleInsufficient / ToolNotInRegistry / ToolCreatorMismatch + 10 prior. Plus `NotYetImplemented` retained as explicit stub sentinel. | CX-1 (Codex Q-G) |
| **P5** | "Phase 1 record-only" golden fixture tests asserted only length=64 + self-stability, did NOT lock SHA-256 hex; `TerminalSummary` excluded from round-trip / kind / golden tests | Hardcoded SHA-256 hex constants for all 7 TypedTx fixture digests (Work / Verify / Challenge / Reuse / FinalizeReward / TaskExpire / TerminalSummary). NEW tests: cross-variant non-collision (7×7 pairwise distinct), BTreeSet permutation independence, default round-trip, signing-payload domain non-collision (6 distinct domain digests), signing-payload-excludes-signature (mutating tx.signature must NOT affect digest). All variants now in round-trip + kind-projection. Total typed_tx tests: 11 → 17. | C-2 (Codex Q-J + Gemini Q9) |
| **P6** | STATE § 2.5 wording wrong vs actual codec — claimed `#[repr(u8)]`-controlled enum discriminants; bincode-2 actually emits u32 BE for variants and u64 BE for lengths | This v1.1 spec § 2.5-bis explicitly documents the actual codec behavior + cross-references bincode-2 source (`bincode 2.0.1 src/features/serde/ser.rs:186`, `enc/impls.rs:68 + :128`). `#[repr(u8)]` is a Rust language attribute that does NOT control serde wire format. Recommendation accepted: keep u32 variants + u64 lengths (no codec change; spec language fixed). | CX-2 (Codex Q-D) |
| **P7** | D-3 TerminalSummaryTx field-set divergence | RESOLVED (P3 migrated to full schema). § 9 D-3 row removed. | C-3 followup |
| **P8** | FinalizeRewardTx had ambiguous {task_id, solver, reward, royalty} provenance + redundant system_signature unclear | This spec § 4 explicitly states {task_id, solver, reward} are **Q-DERIVED at replay** (re-fetched from ClaimsIndex by claim_id; wire fields are ledger summary, NOT trusted from wire); `system_signature` is RETAINED with explicit dual-sign rationale (this sig binds the tx-payload bytes; the L4 `LedgerEntrySigningPayload` sig binds the sequencer-stamped envelope; both are needed). | C-3 + GM-2 |
| **P9** | Cold-replay → Art 0.2 violation if CAS index not persisted | This spec § 0 NEW "Cross-Atom Ordering Gate": v1.1 PASS is contingent on CO1.4-extra (CAS index persistence) shipping BEFORE CO1.7-impl A4 (replay_full_transition). CO1.7-impl A2 (Sequencer apply path) and A3 (dispatch_transition stubs) may proceed; A4 BLOCKED on CO1.4-extra. | GM-1 (Gemini Q4) |
| **P10** | TaskId-vs-TxId QState index mismatch (typed_tx uses TaskId; QState `task_markets_t` / `escrows_t` / `stakes_t` keyed by TxId) | This spec § 9 NEW D-4 documents the forward-migration plan: CO P2.1 (TaskMarket atom) owns the QState retrofit; v1.1 records the migration debt + cross-atom dependency note. Does NOT perform the retrofit (out of CO1.1.4-pre1 scope; would touch q_state.rs which is its own atom). | CX-3 (Codex Q-J) |

10 patches integrated below.

---


**Why this atom exists**: spec § 2.5 of `STATE_TRANSITION_SPEC_v1_2026-04-27.md` explicitly deferred "full ABI surface for QState/SignalBundle/TransitionError" to CO1.7. CO1.7 spec § 0 places the per-kind tx schemas in `STATE_TRANSITION_SPEC § 1` ("frozen on paper, not yet in code"). When CO1.7-impl A1 (Git2LedgerWriter, commit `a03cc52`) shipped, downstream A2 (TypedTx + dispatch_transition) discovered ~30 supporting schema types are required but **none of them exist in code** — only `MicroCoin` is defined. This atom defines that ABI surface in isolation under its own dual-audit gate, per the project's per-atom audit principle (CLAUDE.md "Audit Standard").

**Companion**: `STATE_TRANSITION_SPEC_v1_2026-04-27.md` § 1 (typed schemas), § 2.5 (canonical serialization), § 3 (transition pseudocode — informs FinalizeRewardTx schema, see § 4 below).

**Single sentence**: define every supporting type + the 7 typed-tx variant payload structs + the `TypedTx` enum, with `Serialize/Deserialize` derives over the spec § 2.5 canonical encoding (bincode v2 BE + fixed_int), so that CO1.7-impl A2-A4 (Sequencer + dispatch_transition + replay_full_transition) can be implemented against a stable type surface.

---

## § 0 Scope

### In scope

1. **Identifier newtypes**: `TaskId`, `RunId`, `ToolId`, `PredicateId` (each opaque `String`).
2. **Read/Write set keys**: `ReadKey(String)`, `WriteKey(String)`.
3. **Agent signature**: `AgentSignature([u8; 64])` — Ed25519 detached signature, distinct from `SystemSignature` (system_keypair.rs).
4. **Predicate result types**: `BoolWithProof`, `PredicateResultsBundle`, `SafetyOrCreation`.
5. **Status / class enums**: `TxStatus`, `RejectionClass`, `VerifyVerdict`, `RunOutcome`.
6. **Slash evidence reference**: `SlashEvidenceCid(Cid)` newtype.
7. **Money newtype**: `StakeMicroCoin(MicroCoin)` (non-negative invariant enforced at business layer; type-level newtype prevents accidental mix with general `MicroCoin`).
8. **Typed-tx payload structs** (all in `state::typed_tx`): `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`, `TerminalSummaryTx` (8-field STATE § 1.5 schema; v1.1 P3 migrated from a 3-field placeholder previously in `system_keypair.rs`).
9. **Outer enum**: `pub enum TypedTx` with the 7 variants.
10. **Trait**: `pub trait HasSubmitter` per STATE spec § 3.6.5 v1.3.
11. **Conformance tests**: 1 golden fixture per main tx kind (input → known SHA-256 of canonical bytes) + 100-input round-trip + cross-call byte stability.

### Out of scope (explicit deferral)

- **MetaTx + ancillaries** (`PredicatePatch`, `ToolPatch`, `JudgeSignature`, `HumanSignature`, `ConstitutionCheckProof`, `ReversibilityPlan`) — STATE spec § 1.6 declares MetaTx is **v4.1 only**; v4 emits `MetaProposalDraft` to L3 CAS, not L4. ⏭ deferred.
- **Slash transition** — already deferred to CO P2.5 ChallengeCourt per CO1.7 spec K5.
- **Per-kind transition function bodies** (`step_transition`, `verify_transition`, `challenge_transition`, `reuse_transition`, `finalize_reward_transition`, `task_expire_transition`, `emit_terminal_summary_transition`) — these consume the ABI defined here; they belong to **CO1.7.5** (the body atom).
- **Sequencer + dispatch_transition + replay_full_transition** — these consume the ABI; they belong to CO1.7-impl **A2-A4** (post this atom).
- **`SignalBundle` typed shape** — STATE spec uses `SignalBundle::empty()` / `::finalize(...)` / `::task_expired(...)` / `::terminal_summary(...)` constructors. v1 of this atom emits a minimal typed `SignalBundle` (single enum-like discriminator + payload) sufficient for CO1.7-impl to compile; full event-stream design lands in CO1.9 L6 signal indices.
- **TransitionError full taxonomy** — v1 emits a minimal enum covering the variants invoked in spec § 3 pseudocode (`ClaimNotFound`, `ChallengeWindowStillOpen`, `AlreadySlashed`, `TaskNotFound`, `InvalidSystemSignature`, `StaleParent`, `TaskNotExpired`, `TaskHasOpenClaim`, `TerminalSummaryNotApplicable`, `NotYetImplemented`); per-stage enum proliferation is a CO1.7.5 concern.

### What this atom is NOT replacing

- `src/state/q_state.rs` (existing): keeps its existing types verbatim. CO1.1.4-pre1 only adds new types in `src/state/typed_tx.rs`.
- `src/economy/money.rs` (existing): unchanged. `StakeMicroCoin` is a **newtype on `MicroCoin`** living in `src/economy/money.rs` (additive).

### § 0.1 Cross-atom ordering gate (v1.1 NEW per Gemini Q4 round-1)

**Constitutional concern**: CO1.7 LedgerEntry stores typed-tx payloads in L3 CAS via `tx_payload_cid: Cid`. The current shipped `CasStore::open()` initializes an empty in-memory index (CO1.4 store.rs:67); after process restart the CAS bytes are unrecoverable until the index is repopulated. This means **cold-replay of L4 cannot reconstruct typed payloads** — a direct Art. 0.2 (tape canonicality) violation if uncorrected.

**Mitigation**: CAS index persistence is its own atom — **CO1.4-extra** — already named in CO1.7 spec § 0. CO1.4-extra adds index persistence (likely a sidecar JSONL or git-tag manifest) so cold-replay can recover payloads via `CasStore::get`.

**Hard ordering for v1.1 PASS**:
- CO1.7-impl A2 (Sequencer apply path) + A3 (dispatch_transition skeleton) may proceed against CO1.1.4-pre1 v1.1 PASS independently.
- **CO1.7-impl A4 (replay_full_transition) MUST NOT ship before CO1.4-extra**. Until then, FullTransition replay errors with `CasMissing` after process restart (already documented in CO1.7 spec § 4 / `ReplayError::CasMissing`).
- CO1.4-extra has its own dual-audit gate.

This ordering is a **necessary condition for CO1.1.4-pre1 PASS** per round-1 Gemini Q4; documented here so future audits cannot reinterpret silence as approval.

---

## § 1 Module layout

```
src/state/
├── mod.rs                       (existing; +pub mod typed_tx + re-exports)
├── q_state.rs                   (existing; unchanged)
└── typed_tx.rs                  (NEW; ~600-900 LoC; the ABI surface)

src/economy/
└── money.rs                     (existing; +pub struct StakeMicroCoin newtype + minimal impls)

src/bottom_white/ledger/
└── system_keypair.rs            (existing; serde_bytes_64 helper promoted to pub(crate)
                                  so AgentSignature can re-use the [u8; 64] adapter)
```

**Crate boundary**: `state::typed_tx` consumes (a) `state::q_state` types (Hash, AgentId, TxId, NodeId), (b) `economy::money::MicroCoin` + `StakeMicroCoin`, (c) `bottom_white::cas::schema::Cid`, (d) `bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature}`. No new outward dependencies; no circular dep risk.

---

## § 2 Identifier newtypes

```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct TaskId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct RunId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ToolId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct PredicateId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ReadKey(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct WriteKey(pub String);
```

All identifiers are opaque strings to Q_t (per existing `AgentId` / `TxId` pattern in q_state.rs). Concrete derivation rules (e.g. `TxId::derive(run_id, "terminal")` per STATE § 3.7) live at the call sites, not in the type.

---

## § 3 AgentSignature, StakeMicroCoin, SlashEvidenceCid

```rust
/// Detached Ed25519 signature over a per-tx canonical_digest.
/// Distinct from SystemSignature (system-keypair signatures) at type level —
/// agent-vs-system signature confusion would be a security hazard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSignature(#[serde(with = "system_keypair::serde_bytes_64")] [u8; 64]);

/// Newtype on MicroCoin for stake fields. Non-negative is a runtime invariant
/// (not a type invariant) per Inv 3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct StakeMicroCoin(pub MicroCoin);

/// L3 CAS handle to slash evidence. Kept as a newtype (not a bare Cid) so the
/// FinalizedSlash variant of TxStatus can't accidentally accept arbitrary CIDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct SlashEvidenceCid(pub Cid);
```

---

## § 4 FinalizeRewardTx — derived schema

**Spec gap**: STATE_TRANSITION_SPEC § 3.4 uses `FinalizeTx::from(claim_id, reward)` constructor pattern but provides no explicit struct. CO1.7 spec § 1 lists `TxKind::FinalizeReward = 4` but defers the struct to "frozen in STATE_TRANSITION_SPEC § 1" — which the STATE spec doesn't actually contain.

**v1 derivation** (from § 3.4 call sites + the TaskExpireTx pattern in § 3.6, system-emitted):

```rust
pub struct FinalizeRewardTx {
    pub tx_id: TxId,                       //  1
    pub claim_id: ClaimId,                 //  2  TYPED newtype (v1.1 P2)
    pub task_id: TaskId,                   //  3  Q-DERIVED at replay; wire = ledger summary
    pub solver: AgentId,                   //  4  Q-DERIVED at replay; wire = ledger summary
    pub reward: MicroCoin,                 //  5  Q-DERIVED at replay (SettlementEngine output); wire = ledger summary
    pub parent_state_root: Hash,           //  6  must equal q.state_root_t at submission
    pub epoch: SystemEpoch,                //  7  which keypair signed
    pub timestamp_logical: u64,            //  8  monotonic
    pub system_signature: SystemSignature, //  9  system-emitted, see § 4.1 dual-sign rationale
}
```

### § 4.1 Q-derived vs wire-only fields (v1.1 NEW per Codex Q-B + Gemini Q6)

For `FinalizeRewardTx`, fields {`task_id`, `solver`, `reward`} are recorded on the wire as a **ledger summary** (so a human reading L4 can see the finalize event semantics + downstream tools without Q_t access can render the event). At replay, however, **the AUTHORITATIVE values come from `Q_t` lookups by `claim_id`**:
- `task_id` = `q.economic_state_t.claims_t[claim_id].task_id` (or equivalent ClaimEntry field)
- `solver` = `q.economic_state_t.claims_t[claim_id].solver` (or claimant)
- `reward` = `SettlementEngine::finalize(claim, escrow, attribution, ...)` — recomputed from Q_t

If wire-stored values diverge from Q-derived values at replay, **replay rejects with `TransitionError::ClaimNotFound` or a stricter mismatch error** (CO1.7-impl A4 enforces this; CO1.7.5 transition body owns the comparison rule).

**Royalty edges**: NOT on wire. Replay walks `q.economic_state_t.royalty_graph_t.edges_from(claim.target_work_tx)` per STATE § 3.4 stage 3c. Eliminates wire-format bloat + prevents stale royalty snapshots from being trusted post-amendment.

### § 4.2 Dual-sign rationale (v1.1 NEW per Gemini Q6)

`FinalizeRewardTx.system_signature` is **NOT redundant** with the L4 envelope signature. They sign different bytes:
- `FinalizeRewardTx.system_signature` signs the **payload bytes** (`FinalizeRewardSigningPayload.canonical_digest()` via `b"turingosv4.system_sig.finalize_reward.v1"` domain prefix). Audit-relevant for: "this finalize event was emitted by a runtime keypair epoch X" (cross-cell trust + post-hoc forensics).
- L4 `LedgerEntry.system_signature` signs the **sequencer-stamped envelope** (`LedgerEntrySigningPayload.canonical_digest()` via `b"turingosv4.ledger_entry_signing.v1"` — CO1.7 spec § 1.2). Audit-relevant for: "this `(logical_t, parent_ledger_root, tx_payload_cid)` was committed by the sequencer".

A successful replay verifies BOTH: payload sig (this struct) confirms typed bytes integrity; envelope sig confirms sequencer commitment ordering.

---

## § 5 Other typed tx schemas (transcribed from STATE spec)

`WorkTx` (§ 1.2 — 12 fields), `VerifyTx` / `ChallengeTx` / `ReuseTx` (§ 1.3), `TaskExpireTx` (§ 3.6 v1.3 schema). Verbatim transcription; minor adjustments documented inline.

`TxStatus` includes a `Pending` variant (per STATE § 1.2) but in this v4 codebase `TxStatus` is **set BY the runner**, never serialized into the canonical transaction wire format. Therefore: `TxStatus` is **NOT a field of any TypedTx variant**; it is a runtime book-keeping enum exposed on the public API surface but not part of the canonical encoding. (CO1.7 spec § 1.2 puts `status: TxStatus` on WorkTx field 12; this atom **diverges**: status is tracked in `q_t.q_t.agents[id].last_accepted_tx` + ClaimsIndex, NOT on the wire. **Audit input**: confirm or push back.)

---

## § 6 TypedTx enum

```rust
pub enum TypedTx {
    Work(WorkTx),
    Verify(VerifyTx),
    Challenge(ChallengeTx),
    Reuse(ReuseTx),
    FinalizeReward(FinalizeRewardTx),
    TaskExpire(TaskExpireTx),
    TerminalSummary(TerminalSummaryTx),  // 8-field schema in state::typed_tx (v1.1 P3)
}

impl TypedTx {
    pub fn tx_kind(&self) -> TxKind {
        match self {
            Self::Work(_)            => TxKind::Work,
            Self::Verify(_)          => TxKind::Verify,
            Self::Challenge(_)       => TxKind::Challenge,
            Self::Reuse(_)           => TxKind::Reuse,
            Self::FinalizeReward(_)  => TxKind::FinalizeReward,
            Self::TaskExpire(_)      => TxKind::TaskExpire,
            Self::TerminalSummary(_) => TxKind::TerminalSummary,
        }
    }
}
```

The `TxKind` enum already exists in `transition_ledger.rs` with `#[repr(u8)]` and explicit discriminants. `TypedTx::tx_kind()` is the projection used by CO1.7 sequencer apply_one stage 5 (`tx_kind: TxKind::from_typed(&tx)` → renamed `TypedTx::tx_kind(&tx)` for ergonomics).

---

## § 7 Canonical serialization invariants

`canonical_encode` / `canonical_decode` (already shipped in `transition_ledger.rs` per CO1.7-impl A1) are reused as the wire codec:

- **I-CANON-A**: `canonical_encode(typed_tx)` returns deterministic bytes (BE + fixed_int + BTreeMap/BTreeSet lex order).
- **I-CANON-B**: `decode(encode(x)) == x` byte-identically for ALL variants (incl. zero-default).
- **I-CANON-C**: 2 independent encode calls on the same value produce identical bytes.
- **I-CANON-D**: per-variant golden fixture: every TypedTx variant (7 / 7) has a known SHA-256 of canonical bytes, hard-coded in tests (`EXPECTED_HEX_*`). Future serde-derive / codec change → fixture diff → audit-required (rotation commit).
- **I-CANON-E** (v1.1 NEW): cross-variant non-collision — pairwise digests over all 7 fixture variants are distinct.
- **I-CANON-F** (v1.1 NEW): BTreeMap / BTreeSet permutation independence — building the same struct via different insertion orders produces byte-identical bytes.
- **I-CANON-G** (v1.1 NEW per C-1): each agent-signed and system-emitted typed-tx has a paired `*SigningPayload` struct + `canonical_digest()` with explicit domain prefix `b"turingosv4.<actor>.<purpose>.v1"`. Domain prefix bytes are part of the SHA-256 input. 6 distinct domains (work / verify / challenge agent + finalize_reward / task_expire / terminal_summary system) yield pairwise-distinct digests.

### § 7.1 Codec wording fix (v1.1 P6 per Codex Q-D round-1)

STATE_TRANSITION_SPEC § 2.5 v1.4 wording is **inaccurate** for the actual codec; this v1.1 spec corrects:

| What § 2.5 said | What bincode-2 actually does |
|---|---|
| `Enum discriminant: u8 (variant index in declaration order)` | **u32 BE** ([bincode 2.0.1 src/features/serde/ser.rs:186](https://docs.rs/bincode/2.0.1/src/bincode/features/serde/ser.rs.html), [src/enc/impls.rs:68](https://docs.rs/bincode/2.0.1/src/bincode/enc/impls.rs.html)) under `with_fixed_int_encoding`. The variant index is encoded as `u32::to_be_bytes()`. |
| `Strings serialized as UTF-8 with explicit length prefix u32-BE` | **u64 BE** length prefix (bincode encodes `usize` as u64 under `with_fixed_int_encoding`; [src/enc/impls.rs:128](https://docs.rs/bincode/2.0.1/src/bincode/enc/impls.rs.html)). The same applies to BTreeMap / BTreeSet / Vec lengths. |
| `#[repr(u8)]` controls discriminant | **No** — `#[repr(u8)]` is a Rust language attribute affecting in-memory layout + raw cast (`as u8`) but does NOT control serde wire format. Codex caught this; spec language fixed. |

**v1.1 decision**: keep u32 variants + u64 lengths; do NOT introduce a custom serde adapter to force u8 discriminants (which would force re-encoding of all existing fixtures + complicate forward-compat for >256 variants). The locked golden fixtures in `EXPECTED_HEX_*` reflect the actual u32/u64 codec.

This wording fix is a **spec-only patch**; no code change required (the codec was already correct; only the description was wrong).

### § 7.2 TransitionError additive-only commitment (v1.2 GR-2 per Gemini round-2)

`TransitionError` variants in **v4 are additive-only**:
- New variants MUST be APPENDED to the existing list (no insertion that would shift downstream variant indices).
- Existing variants MUST NOT be reordered (bincode emits variant-index-as-u32-BE; reordering changes the wire format and invalidates locked golden fixtures).
- Variant removal is NOT permitted within v4; deprecated error classes get a doc-comment "deprecated; replaced by X" but the variant stays.
- Bumping the major version (v4 → v5) is the only path that allows non-additive changes; that triggers an ABI rotation cycle (re-audit + re-fixture + canonical re-encoding migration).

This rule applies symmetrically to other ABI enums frozen in this atom: `TxStatus` / `RejectionClass` / `VerifyVerdict` / `RunOutcome` / `SafetyOrCreation` / `SignalKind` / `CanonicalMessage` / `TxKind` (transition_ledger.rs).

### § 7.3 Domain-string rotation process (v1.2 GR-3 per Gemini round-2)

If a future audit finds a security defect in a domain prefix (`b"turingosv4.<actor>.<purpose>.v1"`), rotation follows this discipline:

1. **Add NEW constant** (`*.v2`) **in parallel**; do NOT delete the old `*.v1` constant.
2. **Old `*.v1` MUST remain reachable in code** until the runtime can prove no in-flight tx still uses it (replay-window quiescence).
3. **New transitions emit only `*.v2` digests**; the runtime accepts both digests during the rotation window.
4. **Bump the v4 spec minor version** with a "domain rotation" entry in the patch log.
5. **Lock new golden hex** for v2-domain digests; v1-domain digests stay locked too (so historical replay still verifies).

The `.v1` suffix on every current domain constant is the affordance that makes this protocol possible without ambiguity.

---

## § 8 HasSubmitter trait

```rust
pub trait HasSubmitter {
    fn submitter_id(&self) -> Option<AgentId>;
}

impl HasSubmitter for WorkTx       { fn submitter_id(&self) -> Option<AgentId> { Some(self.agent_id.clone()) } }
impl HasSubmitter for VerifyTx     { fn submitter_id(&self) -> Option<AgentId> { Some(self.verifier_agent.clone()) } }
impl HasSubmitter for ChallengeTx  { fn submitter_id(&self) -> Option<AgentId> { Some(self.challenger_agent.clone()) } }
impl HasSubmitter for ReuseTx      { fn submitter_id(&self) -> Option<AgentId> { None } }
// FinalizeRewardTx, TaskExpireTx, TerminalSummaryTx: system-emitted; submitter_id() = None
```

Implements STATE spec § 3.6.5 v1.3 directive verbatim.

---

## § 9 Acknowledged divergences from STATE_TRANSITION_SPEC

| ID | STATE spec | CO1.1.4-pre1 v1.1 | Reason |
|---|---|---|---|
| **D-1** | § 1.2 WorkTx field 12 = `status: TxStatus` | **dropped from wire** (Codex round-1 PASS with patch note) | TxStatus is runner book-keeping, not canonical wire data. STATE § 3 transition fns do NOT read `tx.status` from received tx; status is derived from accepted-tx history + ClaimsIndex. Codex Q-A round-1: PASS. |
| **D-2** | § 3.4 `FinalizeTx::from(claim_id, reward)` opaque constructor | **explicit `FinalizeRewardTx` struct** with Q-derived field discipline (§ 4.1) + dual-sign rationale (§ 4.2) | spec gap; derived schema. |
<!-- v1.2 (R2-3 closure): D-3 row removed. Migration is complete; no divergence remains. -->
| **D-4** (v1.1 NEW per Codex Q-J / CX-3) | QState `task_markets_t` / `escrows_t` / `stakes_t` keyed by `TxId` (q_state.rs:201/161/182) but typed_tx schemas use `TaskId` for the same task references | **NOT retrofit in this atom**. Migration owned by **CO P2.1 (TaskMarket atom)** which will rekey the QState indices to `TaskId`. CO1.1.4-pre1 documents the cross-atom debt; no wire-format consequence (the typed-tx schemas already use `TaskId` correctly per STATE § 1.2). |

---

## § 10 Audit gates

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 | CHALLENGE (high) | CHALLENGE (high) | **CHALLENGE** | v1.1 patch round (P1-P10) |
| 2 | CHALLENGE (high) | PASS (high) | **CHALLENGE** | v1.2 patch round (P11-P15 + GR-1/2/3) — this version |
| 3 | ⏳ pending | ⏳ pending | TBD | re-audit on v1.2; expected PASS or 1-issue CHALLENGE |
| 4+ | … | … | … | iterate to PASS/PASS |

**Pre-implementation gate** (for CO1.7-impl A2-A4): CO1.1.4-pre1 must reach `PASS/PASS` before A2 starts.

**Audit cost estimate**: ~$15-25 (smaller surface than CO1.7 spec @ $25-42; mostly type definitions + 2 plausibly-derived schemas).

---

## § 11 Estimated scope

- **Spec rounds**: 1-2 expected. The bulk is mechanical transcription; § 4 (FinalizeRewardTx derivation) + § 5 D-1 (TxStatus elision) are the only design decisions auditors are likely to test.
- **Implementation**: ~600-900 LoC (types) + ~150-250 LoC (golden fixture + round-trip tests). All in `src/state/typed_tx.rs` + minimal `src/economy/money.rs` extension.
- **Wall-clock**: 1-2 days.
- **Total atom budget**: ~1.5-2.5 days from spec draft to PASS/PASS.

---

## § 12 What this spec does NOT specify

1. **Field-level meaning beyond identifier types**: e.g. what `read_set` MUST contain for replay attribution to work — that's a CO1.7.5 + CO P2.4.0 concern.
2. **Encryption**: no field is encrypted. Predicate visibility is a Q_t projection (Inv 10), not a schema concern.
3. **Versioning**: `extensions: BTreeMap<String, Vec<u8>>` is on `LedgerEntry` (CO1.7); per-tx forward compat is via additive variants on `TypedTx` (e.g. `TypedTx::MetaTx(...)` lands in v4.1). No per-struct `version` field.
4. **CAS persistence of payloads**: `tx_payload_cid: Cid` is the CAS handle; the bytes lookup is L3 CAS (CO1.4). CAS index persistence is **CO1.4-extra** (separate atom).

---

— ArchitectAI synthesis, 2026-04-28; awaiting round-1 dual external audit.


---

# Implementation A1+A4: src/bottom_white/ledger/transition_ledger.rs

```rust
//! L4 Transition Ledger (CO1.7) — type skeleton + pure helpers.
//!
//! TRACE_MATRIX FC2-Append: canonical envelope appended to L4 once a transition is accepted.
//! TRACE_MATRIX WP § 5.L4: ChainTape Layer 4 spine; one LedgerEntry per accepted transition.
//! TRACE_MATRIX § 1 (CO1_7_TRANSITION_LEDGER_v1_2026-04-28 v1.1): schema + append() + replay_chain_integrity() pseudocode.
//!
//! **Status**: v1.1 type skeleton — round-1 dual audit returned CHALLENGE/CHALLENGE; this
//! version closes 11 must-fix items (C1/C2/C3 + K1-K7 + G1 + D1). Awaiting round-2.
//! All bodies that depend on yet-to-implement transition functions or CAS index
//! persistence are stubbed; full-mode replay is deferred to CO1.7.5+.
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
/// 7. dispatch_transition re-run produces (q_next, _signals)
/// 8. q_next.state_root_t matches entry.resulting_state_root
/// 9. resulting_ledger_root recomputed via append() matches stored
///
/// **Stub-state caveat (CO1.7.5 unblocks)**: while `dispatch_transition`
/// returns `NotYetImplemented` for every variant, replay errors at stage 7
/// for any non-empty chain. Conformance tests exercising stages 1-6
/// independently are `#[test]`-runnable now; full state_root reconstruction
/// gates on CO1.7.5.
pub fn replay_full_transition(
    genesis_state_root: crate::state::q_state::Hash,
    genesis_ledger_root: crate::state::q_state::Hash,
    entries: &[LedgerEntry],
    cas: &dyn LedgerCasView,
    pinned_pubkeys: &crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys,
    predicate_registry: &crate::top_white::predicates::registry::PredicateRegistry,
    tool_registry: &crate::bottom_white::tools::registry::ToolRegistry,
) -> Result<(crate::state::q_state::Hash, crate::state::q_state::Hash), ReplayError> {
    use crate::bottom_white::ledger::system_keypair::{
        verify_system_signature, CanonicalMessage,
    };
    use crate::state::q_state::QState;
    use crate::state::sequencer::dispatch_transition;

    let mut prev_state_root = genesis_state_root;
    let mut prev_ledger_root = genesis_ledger_root;
    // For dispatch we need a QState. Replay reconstructs it from genesis;
    // initial state has empty agent swarm + budget defaults. The state_root_t
    // and ledger_root_t are the load-bearing fields entries verify against.
    let mut q = QState::genesis();
    q.state_root_t = genesis_state_root;
    q.ledger_root_t = genesis_ledger_root;

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
        if entry.parent_state_root != prev_state_root {
            return Err(ReplayError::ParentStateMismatch { at: i });
        }
        // Stage 3
        if entry.parent_ledger_root != prev_ledger_root {
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

        // Stage 6: canonical_decode → TypedTx.
        let typed_tx: crate::state::typed_tx::TypedTx = canonical_decode(&payload_bytes)
            .map_err(|_| ReplayError::CasMissing { at: i })?;

        // Stage 7: re-run pure dispatch_transition.
        let (q_next, _signals) =
            dispatch_transition(&q, &typed_tx, predicate_registry, tool_registry)
                .map_err(|inner| ReplayError::Transition { at: i, inner })?;

        // Stage 8: state_root match.
        if q_next.state_root_t != entry.resulting_state_root {
            return Err(ReplayError::StateRootMismatch { at: i });
        }

        // Stage 9: ledger_root match (recompute via append).
        let recomputed_ledger_root = append(&prev_ledger_root, &signing_digest);
        if recomputed_ledger_root != entry.resulting_ledger_root {
            return Err(ReplayError::LedgerRootMismatch { at: i });
        }

        // Advance.
        q = q_next;
        q.ledger_root_t = entry.resulting_ledger_root;
        prev_state_root = entry.resulting_state_root;
        prev_ledger_root = entry.resulting_ledger_root;
    }

    Ok((prev_state_root, prev_ledger_root))
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
            Hash::ZERO,
            Hash::ZERO,
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
            Hash::ZERO,
            Hash::ZERO,
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
            Hash::ZERO,
            Hash::ZERO,
            &[entry],
            &cas2,
            &pinned,
            &preds,
            &tools,
        )
        .unwrap_err();
        // Stages 1-6 (incl. CAS lookup post-reopen) PASS; stage 7 stubs.
        assert!(matches!(err, ReplayError::Transition { at: 0, .. }));
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

# Implementation A2+A3: src/state/sequencer.rs

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

        // Stage 3: put payload to CAS. DIV-5 5-param put signature. The
        // `created_at_logical_t` is the TENTATIVE logical_t (current counter +
        // 1); the final commit logical_t is assigned at stage 4 atomically.
        let payload_bytes = canonical_encode(&tx)
            .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
        let tentative_logical_t = self.next_logical_t.load(Ordering::SeqCst) + 1;
        let payload_cid = {
            let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
            cas_w.put(
                &payload_bytes,
                ObjectType::ProposalPayload,
                &format!("sequencer-epoch-{}", self.epoch.get()),
                tentative_logical_t,
                Some("TypedTx.v1".to_string()),
            )?
        };

        // Stage 4: K1 — assign logical_t ONLY now (post-accept).
        let logical_t = self.next_logical_t.fetch_add(1, Ordering::SeqCst) + 1;

        // Stage 5: build LedgerEntrySigningPayload.
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
        // K3 v1.2 (revised): we set q.ledger_root_t but NOT q.head_t (head_t
        // mutation deferred to CO1.7.5+ when Git2LedgerWriter exposes
        // commit_sha alongside Hash). state_root_t comes from q_next as-is.
        {
            let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
            let mut writer_w = self
                .ledger_writer
                .write()
                .map_err(|_| ApplyError::QStateLockPoisoned)?;
            writer_w.commit(&entry)?;
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

# Implementation CO1.4-extra: src/bottom_white/cas/store.rs

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

# Supporting: src/state/typed_tx.rs (TypedTx ABI; PASS/PASS)

```rust
//! Typed transaction ABI surface — CO1.1.4-pre1.
//!
//! Spec authority:
//! - `handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — this atom
//! - `STATE_TRANSITION_SPEC_v1_2026-04-27.md` § 1 (typed schemas), § 2.5
//!   (canonical serialization), § 3 (transition pseudocode used to derive
//!   FinalizeRewardTx schema in spec § 4)
//!
//! Why this module exists: when CO1.7-impl A1 (Git2LedgerWriter) shipped, the
//! downstream A2 (Sequencer + `dispatch_transition`) needed a `TypedTx` enum
//! whose variants carry per-kind tx structs. Those structs and ~20 supporting
//! types (identifiers, signatures, predicate-result types, status enums) were
//! "frozen on paper" in STATE_TRANSITION_SPEC § 1 but had no Rust definition.
//! CO1.1.4-pre1 lands them in isolation under its own dual-audit gate,
//! per the project's per-atom audit principle (CLAUDE.md "Audit Standard").
//!
//! /// TRACE_MATRIX FC2-Submit + § 1 typed schemas: typed-tx ABI surface.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::ledger::system_keypair::{serde_bytes_64, SystemEpoch, SystemSignature};
use crate::economy::money::{MicroCoin, StakeMicroCoin};
use crate::state::q_state::{AgentId, Hash, TxId};

// ────────────────────────────────────────────────────────────────────────────
// § 2 Identifier newtypes (all opaque strings to Q_t)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 — task-market entry id; opaque string.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct TaskId(pub String);

/// TRACE_MATRIX § 1.5 — runtime run id (one run per `Sequencer` driver lifecycle).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct RunId(pub String);

/// TRACE_MATRIX STATE § 3.4 + § 4 I-FINALIZE-BATCH-ORDER — typed claim id used
/// in `FinalizeRewardTx.claim_id` and `ClaimsIndex` keying. Wraps `TxId`
/// (the underlying claim is recorded against the work_tx's TxId in
/// ClaimsIndex per current QState shape) but **prevents accidental mixing
/// of claim references with arbitrary transaction references** at the type
/// level (Codex round-1 Q-B CHALLENGE).
///
/// `#[serde(transparent)]` — wire-identical to TxId, so adoption is
/// non-breaking for canonical encoding.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct ClaimId(pub TxId);

impl ClaimId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(TxId(s.into()))
    }
    pub fn as_tx_id(&self) -> &TxId {
        &self.0
    }
}

/// TRACE_MATRIX § 1.3 ReuseTx + L2 Tool Registry — opaque tool identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ToolId(pub String);

/// TRACE_MATRIX § 1.2 PredicateResultsBundle + L1 Predicate Registry — opaque predicate id.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct PredicateId(pub String);

/// TRACE_MATRIX § 1.2 WorkTx field 5 — read-set key (DAG attribution / replay).
/// Kept as opaque string in v1; stricter typing (path / tape-coordinate) lands
/// in CO P2.4.0 attribution-engine spike.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ReadKey(pub String);

/// TRACE_MATRIX § 1.2 WorkTx field 6 — write-set key (DAG attribution / replay).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct WriteKey(pub String);

// ────────────────────────────────────────────────────────────────────────────
// § 3 AgentSignature (Ed25519 [u8;64], type-distinct from SystemSignature)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 WorkTx field 10 + I-SIG: agent-side detached Ed25519
/// signature over the per-tx canonical_digest. Distinct type from
/// `SystemSignature` to prevent accidental confusion at API boundaries
/// (Codex sec-arg: agent-vs-system signature mixing is a real hazard).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSignature(#[serde(with = "serde_bytes_64")] [u8; 64]);

impl AgentSignature {
    pub const fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }
    pub const fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }
}

impl Default for AgentSignature {
    fn default() -> Self {
        Self([0u8; 64])
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 3 SlashEvidenceCid (newtype; type-distinct slash-evidence reference)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 TxStatus::FinalizedSlash — typed reference to the
/// counter-example payload that justified the slash (lives in L3 CAS).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct SlashEvidenceCid(pub Cid);

// ────────────────────────────────────────────────────────────────────────────
// § 4 Predicate result types
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 PredicateResultsBundle — boolean predicate verdict
/// optionally accompanied by an L3 CAS reference to the proof object
/// (e.g. Lean witness, ZK proof bytes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BoolWithProof {
    pub value: bool,
    pub proof_cid: Option<Cid>,
}

/// TRACE_MATRIX § 1.2 PredicateResultsBundle — safety-class discriminator.
/// Determines fail-closed (Safety) vs fail-open-with-signal (Creation) behavior
/// when a predicate's evaluation errors. Frozen STATE spec § 1.2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum SafetyOrCreation {
    Safety = 0,
    Creation = 1,
}

impl Default for SafetyOrCreation {
    fn default() -> Self {
        // Safety bias by default: fail-closed if no class declared.
        Self::Safety
    }
}

/// TRACE_MATRIX § 1.2 WorkTx field 8 — runner-stamped predicate results
/// (acceptance + settlement gates) with explicit safety-class discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PredicateResultsBundle {
    pub acceptance: BTreeMap<PredicateId, BoolWithProof>,
    pub settlement: BTreeMap<PredicateId, BoolWithProof>,
    pub safety_class: SafetyOrCreation,
}

// ────────────────────────────────────────────────────────────────────────────
// § 5 Status / class enums (RejectionClass, VerifyVerdict, RunOutcome,
//                          and the runtime-only TxStatus per D-1)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.4 — classification of a rejected attempt.
/// Public predicates are classified concretely; private predicates surface as
/// `Opaque` (no information leakage to attacker).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RejectionClass {
    AcceptancePredicateFail(PredicateId),
    SettlementPredicateFail(PredicateId),
    StakeInsufficient,
    SignatureInvalid,
    StaleParentRoot,
    Opaque,
    BudgetExceeded,
}

/// TRACE_MATRIX § 1.3 VerifyTx field 5 — verifier verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum VerifyVerdict {
    Confirm = 0,
    Doubt = 1,
}

/// TRACE_MATRIX § 1.5 TerminalSummaryTx field 4 + Art. IV halt-reason taxonomy.
/// Five-way partition over how a run terminates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum RunOutcome {
    OmegaAccepted = 0,
    MaxTxExhausted = 1,
    WallClockCap = 2,
    ComputeCap = 3,
    ErrorHalt = 4,
}

/// TRACE_MATRIX § 1.2 TxStatus — **runtime book-keeping only** (D-1 divergence
/// from STATE spec): never serialized into a TypedTx variant's wire bytes.
/// Tracked in `q_t.q_t.agents[id].last_accepted_tx` + `ClaimsIndex`. Exposed
/// here as a public type for the runtime API surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TxStatus {
    Pending,
    Accepted,
    Rejected(RejectionClass),
    FinalizedReward(MicroCoin),
    FinalizedSlash(SlashEvidenceCid),
}

// ────────────────────────────────────────────────────────────────────────────
// § 5 (cont'd) — Typed tx structs (STATE spec § 1.2-1.6 + § 3.6)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 — agent-submitted work transaction (12-field schema;
/// **D-1 divergence**: field 12 `status: TxStatus` is excluded from canonical
/// wire bytes — TxStatus is runner book-keeping per CO1.1.4-pre1 spec § 5).
///
/// This is the per-tx struct that the CO1.7 sequencer hands to
/// `step_transition` (CO1.7.5 body atom). The `signature` is over
/// `WorkSigningPayload::canonical_digest()` — i.e. the projection produced by
/// `WorkTx::to_signing_payload()` (excludes the signature field itself; per
/// v1.1 P1 the digest pre-image carries the `b"turingosv4.agent_sig.work.v1"`
/// domain prefix).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WorkTx {
    pub tx_id: TxId,                                  //  1
    pub task_id: TaskId,                              //  2
    pub parent_state_root: Hash,                      //  3
    pub agent_id: AgentId,                            //  4
    pub read_set: BTreeSet<ReadKey>,                  //  5
    pub write_set: BTreeSet<WriteKey>,                //  6
    pub proposal_cid: Cid,                            //  7
    pub predicate_results: PredicateResultsBundle,    //  8 (runner-stamped)
    pub stake: StakeMicroCoin,                        //  9
    pub signature: AgentSignature,                    // 10
    pub timestamp_logical: u64,                       // 11
    // 12: TxStatus — D-1 elision; runtime-only.
}

/// TRACE_MATRIX § 1.3 — verifier verdict transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VerifyTx {
    pub tx_id: TxId,                       //  1
    pub target_work_tx: TxId,              //  2
    pub verifier_agent: AgentId,           //  3
    pub bond: StakeMicroCoin,              //  4
    pub verdict: VerifyVerdict,            //  5
    pub signature: AgentSignature,         //  6
    pub timestamp_logical: u64,            //  7
}

impl Default for VerifyVerdict {
    fn default() -> Self {
        Self::Confirm
    }
}

/// TRACE_MATRIX § 1.3 — challenge transaction (counter-example posted with
/// stake at risk).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChallengeTx {
    pub tx_id: TxId,                       //  1
    pub target_work_tx: TxId,              //  2
    pub challenger_agent: AgentId,         //  3
    pub stake: StakeMicroCoin,             //  4
    pub counterexample_cid: Cid,           //  5
    pub signature: AgentSignature,         //  6
    pub timestamp_logical: u64,            //  7
}

/// TRACE_MATRIX § 1.3 — fact-tx recording reuse of a tool created by a prior
/// agent (royalty graph edge). No submitting agent (per § 3.6.5 v1.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReuseTx {
    pub tx_id: TxId,                       //  1
    pub reusing_work_tx: TxId,             //  2
    pub reused_tool_id: ToolId,            //  3
    pub reused_tool_creator: AgentId,      //  4
    pub timestamp_logical: u64,            //  5
}

/// TRACE_MATRIX CO1.1.4-pre1 spec § 4 — derived schema (STATE spec § 3.4
/// uses opaque `FinalizeTx::from(claim_id, reward)` constructor without an
/// explicit struct definition).
///
/// **v1.1 round-1 audit closures**:
/// - **C-3 (Codex Q-B)**: `claim_id` is now a typed `ClaimId` newtype (was
///   bare `TxId`) — STATE § 4 I-FINALIZE-BATCH-ORDER speaks in claim_id;
///   reusing TxId leaked QState implementation into the wire format.
/// - **C-3 (Codex Q-B)**: `task_id` / `solver` / `reward` are documented as
///   **Q-DERIVED at replay** — replay (CO1.7-impl A4) re-fetches them from
///   ClaimsIndex by `claim_id`, NOT trusted from wire. Wire fields are kept
///   as a ledger summary (so a human reading L4 can see the finalize event
///   semantics) but the AUTHORITATIVE values come from Q_t.
/// - **C-3 / GM-2 followup**: `system_signature` is RETAINED for v1.1 — it
///   binds the system-emitted FinalizeRewardTx to a specific runtime keypair
///   epoch (auditability + cross-cell trust). The CO1.7 `LedgerEntry`
///   wraps this struct via CAS reference + signs the `LedgerEntrySigningPayload`
///   digest; the two sigs are NOT redundant: this one binds the tx-payload
///   bytes; the L4 envelope sig binds the (logical_t, parent_ledger_root, tx_payload_cid)
///   sequencer-stamped envelope. v1.1 spec § 4 makes the dual-sign rationale explicit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FinalizeRewardTx {
    pub tx_id: TxId,                       //  1
    pub claim_id: ClaimId,                 //  2 — typed (was TxId in v1)
    pub task_id: TaskId,                   //  3 — Q-derived authoritative; wire = ledger summary
    pub solver: AgentId,                   //  4 — Q-derived authoritative; wire = ledger summary
    pub reward: MicroCoin,                 //  5 — Q-derived authoritative (SettlementEngine output); wire = ledger summary
    pub parent_state_root: Hash,           //  6
    pub epoch: SystemEpoch,                //  7
    pub timestamp_logical: u64,            //  8
    pub system_signature: SystemSignature, //  9 — see doc-comment on dual-sign rationale
}

/// TRACE_MATRIX STATE spec § 3.6 v1.3 — system-emitted task-expiry tx
/// (refunds bounty + locked stakes when no claim finalized by deadline).
/// Verbatim transcription.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskExpireTx {
    pub tx_id: TxId,                       //  1
    pub task_id: TaskId,                   //  2
    pub parent_state_root: Hash,           //  3
    pub bounty_refunded: MicroCoin,        //  4 (computed by runtime; included for ledger summary)
    pub epoch: SystemEpoch,                //  5
    pub timestamp_logical: u64,            //  6
    pub system_signature: SystemSignature, //  7
}

/// TRACE_MATRIX STATE spec § 1.5 — system-emitted no-accept-run handler.
/// Emitted exactly once if a run terminates without any accepted work_tx, so
/// L6 reconstructibility (failure-class signal) is preserved on the tape
/// even when no work_tx ever passed.
///
/// **v1.1 round-1 audit closure (C-3 Codex Q-C must-fix-now)**: replaces the
/// 3-field placeholder previously living in `system_keypair.rs`. Full
/// 8-field schema per STATE § 1.5. The signer (`system_keypair`) now signs
/// an opaque `TerminalSummarySigning([u8; 32])` digest — same opaque-digest
/// pattern as `LedgerEntrySigning` — so the digest is computed here via
/// `TerminalSummaryTx::to_signing_payload().canonical_digest()` (with the
/// `b"turingosv4.system_sig.terminal_summary.v1"` domain prefix per v1.1 P1)
/// and `system_keypair` stays oblivious to the typed-tx schema (no circular
/// `bottom_white ↔ state` dependency).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TerminalSummaryTx {
    pub tx_id: TxId,                                          //  1
    pub task_id: TaskId,                                      //  2
    pub run_id: RunId,                                        //  3
    pub run_outcome: RunOutcome,                              //  4
    pub total_attempts: u32,                                  //  5
    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,// 6
    pub last_logical_t: u64,                                  //  7
    pub system_signature: SystemSignature,                    //  8
}

impl Default for RunOutcome {
    fn default() -> Self {
        Self::OmegaAccepted
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 7 Signing payloads (CO1.1.4-pre1 v1.1 round-1 closure C-1)
//
// Each agent-signed and system-emitted typed-tx has a paired `*SigningPayload`
// struct (subset of fields, EXCLUDES the signature itself) with a
// `canonical_digest()` method that **prepends a stable domain-separation
// prefix** before the bincode-canonical body bytes. This implements:
//
//   sig_input = sha256(b"turingosv4.<actor>.<purpose>.v1" || canonical_encode(payload))
//
// Property: even if two distinct payload TYPES happen to bincode-encode to
// identical bytes (extremely unlikely given distinct field shapes, but
// defensively guaranteed), the domain prefix ensures the SHA-256 inputs
// differ. Closes Codex Q-E + Gemini Q7: type-level distinction is necessary
// but not sufficient as a security boundary.
//
// **Forward dependency**: actual `verify_agent_signature(sig, payload, agent_pubkey)`
// + agent-pubkey-registry lookup is CO P2.x AgentRegistry territory; this
// atom only freezes the canonical_digest pre-image.
// ────────────────────────────────────────────────────────────────────────────

const DOMAIN_AGENT_WORK: &[u8] = b"turingosv4.agent_sig.work.v1";
const DOMAIN_AGENT_VERIFY: &[u8] = b"turingosv4.agent_sig.verify.v1";
const DOMAIN_AGENT_CHALLENGE: &[u8] = b"turingosv4.agent_sig.challenge.v1";
const DOMAIN_SYSTEM_FINALIZE_REWARD: &[u8] = b"turingosv4.system_sig.finalize_reward.v1";
const DOMAIN_SYSTEM_TASK_EXPIRE: &[u8] = b"turingosv4.system_sig.task_expire.v1";
const DOMAIN_SYSTEM_TERMINAL_SUMMARY: &[u8] = b"turingosv4.system_sig.terminal_summary.v1";

/// Reserved for v4.1 MetaTx (Gemini round-2 GR-1 recommendation).
/// Not used in v4 — namespace placeholder so v4.1 can introduce
/// `MetaSigningPayload` without re-rotating sibling domains. Marked
/// `#[allow(dead_code)]` because no v4 consumer references it.
#[allow(dead_code)]
const DOMAIN_AGENT_META_PROPOSAL: &[u8] = b"turingosv4.agent_sig.meta_proposal.v1";

fn domain_prefixed_digest<T: Serialize>(domain: &[u8], value: &T) -> [u8; 32] {
    use crate::bottom_white::ledger::transition_ledger::canonical_encode;
    let body = canonical_encode(value).expect("canonical_encode of signing payload");
    let mut h = Sha256::new();
    h.update(domain);
    h.update(&body);
    h.finalize().into()
}

/// Agent signing payload for `WorkTx` (12 fields → 11 fields; signature excluded).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WorkSigningPayload {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub parent_state_root: Hash,
    pub agent_id: AgentId,
    pub read_set: BTreeSet<ReadKey>,
    pub write_set: BTreeSet<WriteKey>,
    pub proposal_cid: Cid,
    pub predicate_results: PredicateResultsBundle,
    pub stake: StakeMicroCoin,
    pub timestamp_logical: u64,
}

impl WorkSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_AGENT_WORK, self)
    }
}

/// Agent signing payload for `VerifyTx` (7 fields → 6 fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VerifySigningPayload {
    pub tx_id: TxId,
    pub target_work_tx: TxId,
    pub verifier_agent: AgentId,
    pub bond: StakeMicroCoin,
    pub verdict: VerifyVerdict,
    pub timestamp_logical: u64,
}

impl VerifySigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_AGENT_VERIFY, self)
    }
}

/// Agent signing payload for `ChallengeTx` (7 fields → 6 fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChallengeSigningPayload {
    pub tx_id: TxId,
    pub target_work_tx: TxId,
    pub challenger_agent: AgentId,
    pub stake: StakeMicroCoin,
    pub counterexample_cid: Cid,
    pub timestamp_logical: u64,
}

impl ChallengeSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_AGENT_CHALLENGE, self)
    }
}

/// System signing payload for `FinalizeRewardTx` (9 fields → 8 fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FinalizeRewardSigningPayload {
    pub tx_id: TxId,
    pub claim_id: ClaimId,
    pub task_id: TaskId,
    pub solver: AgentId,
    pub reward: MicroCoin,
    pub parent_state_root: Hash,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
}

impl FinalizeRewardSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_SYSTEM_FINALIZE_REWARD, self)
    }
}

/// System signing payload for `TaskExpireTx` (7 fields → 6 fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskExpireSigningPayload {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub parent_state_root: Hash,
    pub bounty_refunded: MicroCoin,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
}

impl TaskExpireSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_SYSTEM_TASK_EXPIRE, self)
    }
}

/// System signing payload for `TerminalSummaryTx` (8 fields → 7 fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TerminalSummarySigningPayload {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub run_id: RunId,
    pub run_outcome: RunOutcome,
    pub total_attempts: u32,
    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,
    pub last_logical_t: u64,
}

impl TerminalSummarySigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_SYSTEM_TERMINAL_SUMMARY, self)
    }
}

// ── Projections: tx → signing payload ────────────────────────────────────

impl WorkTx {
    pub fn to_signing_payload(&self) -> WorkSigningPayload {
        WorkSigningPayload {
            tx_id: self.tx_id.clone(),
            task_id: self.task_id.clone(),
            parent_state_root: self.parent_state_root,
            agent_id: self.agent_id.clone(),
            read_set: self.read_set.clone(),
            write_set: self.write_set.clone(),
            proposal_cid: self.proposal_cid,
            predicate_results: self.predicate_results.clone(),
            stake: self.stake,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl VerifyTx {
    pub fn to_signing_payload(&self) -> VerifySigningPayload {
        VerifySigningPayload {
            tx_id: self.tx_id.clone(),
            target_work_tx: self.target_work_tx.clone(),
            verifier_agent: self.verifier_agent.clone(),
            bond: self.bond,
            verdict: self.verdict,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl ChallengeTx {
    pub fn to_signing_payload(&self) -> ChallengeSigningPayload {
        ChallengeSigningPayload {
            tx_id: self.tx_id.clone(),
            target_work_tx: self.target_work_tx.clone(),
            challenger_agent: self.challenger_agent.clone(),
            stake: self.stake,
            counterexample_cid: self.counterexample_cid,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl FinalizeRewardTx {
    pub fn to_signing_payload(&self) -> FinalizeRewardSigningPayload {
        FinalizeRewardSigningPayload {
            tx_id: self.tx_id.clone(),
            claim_id: self.claim_id.clone(),
            task_id: self.task_id.clone(),
            solver: self.solver.clone(),
            reward: self.reward,
            parent_state_root: self.parent_state_root,
            epoch: self.epoch,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl TaskExpireTx {
    pub fn to_signing_payload(&self) -> TaskExpireSigningPayload {
        TaskExpireSigningPayload {
            tx_id: self.tx_id.clone(),
            task_id: self.task_id.clone(),
            parent_state_root: self.parent_state_root,
            bounty_refunded: self.bounty_refunded,
            epoch: self.epoch,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl TerminalSummaryTx {
    pub fn to_signing_payload(&self) -> TerminalSummarySigningPayload {
        TerminalSummarySigningPayload {
            tx_id: self.tx_id.clone(),
            task_id: self.task_id.clone(),
            run_id: self.run_id.clone(),
            run_outcome: self.run_outcome,
            total_attempts: self.total_attempts,
            failure_class_histogram: self.failure_class_histogram.clone(),
            last_logical_t: self.last_logical_t,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 6 TypedTx outer enum
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 8 dispatch_transition — typed-tx outer enum.
/// 7 variants (K5 closed: NO `Slash`). All variants are defined in this
/// module (`state::typed_tx`); v1.1 P3 migrated `TerminalSummaryTx` here
/// from a 3-field placeholder previously in `system_keypair.rs`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypedTx {
    Work(WorkTx),
    Verify(VerifyTx),
    Challenge(ChallengeTx),
    Reuse(ReuseTx),
    FinalizeReward(FinalizeRewardTx),
    TaskExpire(TaskExpireTx),
    TerminalSummary(TerminalSummaryTx),
}

impl TypedTx {
    /// Project to the [`TxKind`] discriminator stored in `LedgerEntry.tx_kind`.
    pub fn tx_kind(&self) -> crate::bottom_white::ledger::transition_ledger::TxKind {
        use crate::bottom_white::ledger::transition_ledger::TxKind;
        match self {
            Self::Work(_) => TxKind::Work,
            Self::Verify(_) => TxKind::Verify,
            Self::Challenge(_) => TxKind::Challenge,
            Self::Reuse(_) => TxKind::Reuse,
            Self::FinalizeReward(_) => TxKind::FinalizeReward,
            Self::TaskExpire(_) => TxKind::TaskExpire,
            Self::TerminalSummary(_) => TxKind::TerminalSummary,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 8 HasSubmitter trait (STATE spec § 3.6.5 v1.3)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX STATE spec § 3.6.5 v1.3 — submitter resolution trait used
/// by the implicit-init step in agent-submitted transitions. System-emitted
/// transitions return `None` (no agent to init).
pub trait HasSubmitter {
    fn submitter_id(&self) -> Option<AgentId>;
}

impl HasSubmitter for WorkTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.agent_id.clone())
    }
}

impl HasSubmitter for VerifyTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.verifier_agent.clone())
    }
}

impl HasSubmitter for ChallengeTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.challenger_agent.clone())
    }
}

impl HasSubmitter for ReuseTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None
    }
}

impl HasSubmitter for FinalizeRewardTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None
    }
}

impl HasSubmitter for TaskExpireTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None
    }
}

impl HasSubmitter for TerminalSummaryTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None
    }
}

impl HasSubmitter for TypedTx {
    fn submitter_id(&self) -> Option<AgentId> {
        match self {
            Self::Work(t) => t.submitter_id(),
            Self::Verify(t) => t.submitter_id(),
            Self::Challenge(t) => t.submitter_id(),
            Self::Reuse(t) => t.submitter_id(),
            Self::FinalizeReward(t) => t.submitter_id(),
            Self::TaskExpire(t) => t.submitter_id(),
            Self::TerminalSummary(t) => t.submitter_id(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// TransitionError — minimal v1 taxonomy (CO1.1.4-pre1 spec § 0 out-of-scope
// note: full per-stage enum proliferation is CO1.7.5)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX STATE § 3 — transition-function error taxonomy. v1.1 covers
/// every variant invoked in STATE_TRANSITION_SPEC § 3.1-3.7 pseudocode +
/// `NotYetImplemented` for CO1.7.5 stub bodies (per Codex Q-G CHALLENGE).
///
/// **Why payloads are minimal**: the failed `PredicateId` (etc.) is a string
/// reference; richer context (PredicateResultsBundle, Cid of failed proof)
/// is attached by the runtime via separate book-keeping channels (rejected
/// summary stamping, bus rejection log). Keeping TransitionError serializable
/// with primitive payloads avoids forcing PredicateResultsBundle through
/// every error site.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionError {
    // ── Stale-parent & signature ───────────────────────────────────────────
    /// `parent_state_root` does not match `q.state_root_t` (any agent tx).
    StaleParent,
    /// Agent signature verify failed (work / verify / challenge tx).
    SignatureInvalid,
    /// System-keypair signature verify failed (system-emitted tx).
    InvalidSystemSignature,

    // ── Economy ────────────────────────────────────────────────────────────
    /// Submitter's available balance is below the declared stake / bond.
    /// Payload-rich variant (available + required) is intentionally elided
    /// in v1.1 to keep this enum primitive-payloads-only; runtime attaches
    /// context via the rejection log (per STATE § 1.4 RejectedAttemptSummary).
    StakeInsufficient,

    // ── Target lookup ──────────────────────────────────────────────────────
    /// VerifyTx / ChallengeTx / ReuseTx target work_tx not found in L4.
    TargetWorkTxNotFound,
    /// VerifyTx target is not in a verifiable status (e.g. already finalized).
    TargetWorkTxNotVerifiable,
    /// ReuseTx target work_tx exists but is not yet Accepted (parent must accept first).
    ParentNotAcceptedYet,

    // ── Predicate failures ─────────────────────────────────────────────────
    /// step_transition stage 4 — acceptance predicate denied. `PredicateId`
    /// is the public predicate that failed; private predicates surface as
    /// `RejectionClass::Opaque` in book-keeping (NOT here).
    AcceptancePredicateFailed(PredicateId),
    /// verify_transition stage 4 — verification predicate denied.
    VerificationPredicateFailed(PredicateId),
    /// finalize_reward / step_transition stage 5 — settlement predicate denied.
    SettlementPredicateFailed(PredicateId),

    // ── Challenge ──────────────────────────────────────────────────────────
    /// challenge_transition stage 1 — challenge filed after window closed.
    ChallengeWindowClosed,
    /// finalize_reward stage 1 — challenge window still open; cannot finalize.
    ChallengeWindowStillOpen,
    /// finalize_reward stage 1 — claim already slashed; cannot also reward.
    AlreadySlashed,
    /// challenge_transition stage 4 — counterexample failed predicate check.
    CounterexampleInsufficient,

    // ── Reuse ──────────────────────────────────────────────────────────────
    /// reuse_transition stage 1 — referenced tool not in L2 ToolRegistry.
    ToolNotInRegistry,
    /// reuse_transition stage 1 — declared tool creator does not match registry.
    ToolCreatorMismatch,

    // ── Finalize ───────────────────────────────────────────────────────────
    /// finalize_reward — no claim entry for the given claim_id.
    ClaimNotFound,

    // ── Task expire ────────────────────────────────────────────────────────
    /// task_expire — referenced TaskMarket entry not found.
    TaskNotFound,
    /// task_expire — deadline not yet reached.
    TaskNotExpired,
    /// task_expire — at least one open claim exists; cannot refund bounty.
    TaskHasOpenClaim,

    // ── Terminal summary ───────────────────────────────────────────────────
    /// emit_terminal_summary — run already has an accepted work_tx.
    TerminalSummaryNotApplicable,

    // ── Stub sentinel (CO1.7.5 fills) ──────────────────────────────────────
    /// Stub return value used by CO1.7.5 unimplemented bodies — preserves
    /// sequencer + dispatch correctness without forcing transition logic
    /// into this atom. Audit input: this is intentional, not a code smell.
    NotYetImplemented,
}

impl std::fmt::Display for TransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StaleParent => write!(f, "stale parent_state_root"),
            Self::SignatureInvalid => write!(f, "agent signature invalid"),
            Self::InvalidSystemSignature => write!(f, "invalid system signature"),
            Self::StakeInsufficient => write!(f, "stake / bond insufficient"),
            Self::TargetWorkTxNotFound => write!(f, "target work_tx not found"),
            Self::TargetWorkTxNotVerifiable => write!(f, "target work_tx not in a verifiable state"),
            Self::ParentNotAcceptedYet => write!(f, "parent work_tx not yet accepted"),
            Self::AcceptancePredicateFailed(p) => write!(f, "acceptance predicate failed: {p:?}"),
            Self::VerificationPredicateFailed(p) => write!(f, "verification predicate failed: {p:?}"),
            Self::SettlementPredicateFailed(p) => write!(f, "settlement predicate failed: {p:?}"),
            Self::ChallengeWindowClosed => write!(f, "challenge window closed"),
            Self::ChallengeWindowStillOpen => write!(f, "challenge window still open"),
            Self::AlreadySlashed => write!(f, "already slashed"),
            Self::CounterexampleInsufficient => write!(f, "counterexample insufficient"),
            Self::ToolNotInRegistry => write!(f, "reuse tool not in registry"),
            Self::ToolCreatorMismatch => write!(f, "reuse tool creator mismatch"),
            Self::ClaimNotFound => write!(f, "claim not found"),
            Self::TaskNotFound => write!(f, "task not found"),
            Self::TaskNotExpired => write!(f, "task deadline not yet reached"),
            Self::TaskHasOpenClaim => write!(f, "task has at least one open claim"),
            Self::TerminalSummaryNotApplicable => write!(f, "terminal summary not applicable"),
            Self::NotYetImplemented => write!(f, "transition body not yet implemented (CO1.7.5)"),
        }
    }
}
impl std::error::Error for TransitionError {}

// ────────────────────────────────────────────────────────────────────────────
// SignalBundle — minimal v1 typed shape (CO1.7.5 + CO1.9 enrich it later)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX STATE § 3 — tape-emitted signal bundle. v1 minimal: a single
/// enum variant per spec call site in § 3 pseudocode (`empty` /
/// `finalize` / `task_expired` / `terminal_summary`). Full L6 signal-stream
/// design is CO1.9. CO1.1.4-pre1 ships just enough shape for CO1.7-impl to
/// compile and for CO1.7.5 transition bodies to construct each variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SignalBundle {
    pub kind: SignalKind,
}

/// Discriminator over the spec § 3 pseudocode's `SignalBundle::*` constructors.
///
/// **v1.2 round-2 closure (R2-1)**: `Finalize.claim_id` is `ClaimId` (was `TxId`
/// in v1.1; round-2 caught the missed call site that leaked the old type
/// through `SignalBundle::finalize`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalKind {
    Empty,
    Finalize {
        claim_id: ClaimId,
        reward: MicroCoin,
    },
    TaskExpired {
        task_id: TaskId,
        bounty_refunded: MicroCoin,
    },
    TerminalSummary {
        run_id: RunId,
        outcome: RunOutcome,
    },
}

impl Default for SignalKind {
    fn default() -> Self {
        Self::Empty
    }
}

impl SignalBundle {
    pub fn empty() -> Self {
        Self {
            kind: SignalKind::Empty,
        }
    }
    pub fn finalize(claim_id: ClaimId, reward: MicroCoin) -> Self {
        Self {
            kind: SignalKind::Finalize { claim_id, reward },
        }
    }
    pub fn task_expired(task_id: TaskId, bounty_refunded: MicroCoin) -> Self {
        Self {
            kind: SignalKind::TaskExpired {
                task_id,
                bounty_refunded,
            },
        }
    }
    pub fn terminal_summary(run_id: RunId, outcome: RunOutcome) -> Self {
        Self {
            kind: SignalKind::TerminalSummary { run_id, outcome },
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Tests — round-trip (I-CANON-A/B/C) + golden fixtures (I-CANON-D)
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
    use sha2::{Digest, Sha256};

    fn h(byte: u8) -> Hash {
        Hash([byte; 32])
    }
    fn cid(byte: u8) -> Cid {
        Cid([byte; 32])
    }

    /// Helper: canonical bytes → SHA-256 hex string. Used to lock golden
    /// fixtures: any future change to the wire format causes the digest hex
    /// to diverge → audit-required.
    fn digest_hex<T: Serialize>(value: &T) -> String {
        let bytes = canonical_encode(value).expect("encode");
        let hash = Sha256::digest(&bytes);
        hex_lower(&hash)
    }
    fn hex_lower(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for b in bytes {
            s.push_str(&format!("{:02x}", b));
        }
        s
    }

    // ── I-CANON-A/B/C — round-trip + byte-stability ──────────────────────────

    fn fixture_work_tx() -> WorkTx {
        let mut acceptance = BTreeMap::new();
        acceptance.insert(
            PredicateId("acc1".into()),
            BoolWithProof {
                value: true,
                proof_cid: Some(cid(0x11)),
            },
        );
        let mut settlement = BTreeMap::new();
        settlement.insert(
            PredicateId("set1".into()),
            BoolWithProof {
                value: true,
                proof_cid: None,
            },
        );
        WorkTx {
            tx_id: TxId("worktx-fixture-01".into()),
            task_id: TaskId("task-fixture-01".into()),
            parent_state_root: h(0x42),
            agent_id: AgentId("alice".into()),
            read_set: [ReadKey("k.read.a".into()), ReadKey("k.read.b".into())]
                .into_iter()
                .collect(),
            write_set: [WriteKey("k.write.a".into())].into_iter().collect(),
            proposal_cid: cid(0x13),
            predicate_results: PredicateResultsBundle {
                acceptance,
                settlement,
                safety_class: SafetyOrCreation::Safety,
            },
            stake: StakeMicroCoin::from_micro_units(1_000_000),
            signature: AgentSignature::from_bytes([0x77u8; 64]),
            timestamp_logical: 7,
        }
    }

    fn fixture_verify_tx() -> VerifyTx {
        VerifyTx {
            tx_id: TxId("verifytx-fixture-01".into()),
            target_work_tx: TxId("worktx-fixture-01".into()),
            verifier_agent: AgentId("bob".into()),
            bond: StakeMicroCoin::from_micro_units(500_000),
            verdict: VerifyVerdict::Confirm,
            signature: AgentSignature::from_bytes([0x55u8; 64]),
            timestamp_logical: 8,
        }
    }

    fn fixture_challenge_tx() -> ChallengeTx {
        ChallengeTx {
            tx_id: TxId("challengetx-fixture-01".into()),
            target_work_tx: TxId("worktx-fixture-01".into()),
            challenger_agent: AgentId("carol".into()),
            stake: StakeMicroCoin::from_micro_units(2_000_000),
            counterexample_cid: cid(0x21),
            signature: AgentSignature::from_bytes([0x33u8; 64]),
            timestamp_logical: 9,
        }
    }

    fn fixture_reuse_tx() -> ReuseTx {
        ReuseTx {
            tx_id: TxId("reusetx-fixture-01".into()),
            reusing_work_tx: TxId("worktx-fixture-02".into()),
            reused_tool_id: ToolId("tool-001".into()),
            reused_tool_creator: AgentId("alice".into()),
            timestamp_logical: 10,
        }
    }

    fn fixture_finalize_reward_tx() -> FinalizeRewardTx {
        FinalizeRewardTx {
            tx_id: TxId("finalizetx-fixture-01".into()),
            claim_id: ClaimId::new("claim-001"),
            task_id: TaskId("task-fixture-01".into()),
            solver: AgentId("alice".into()),
            reward: MicroCoin::from_micro_units(5_000_000),
            parent_state_root: h(0x43),
            epoch: SystemEpoch::new(1),
            timestamp_logical: 11,
            system_signature: SystemSignature::from_bytes([0xaau8; 64]),
        }
    }

    fn fixture_task_expire_tx() -> TaskExpireTx {
        TaskExpireTx {
            tx_id: TxId("expiretx-fixture-01".into()),
            task_id: TaskId("task-fixture-02".into()),
            parent_state_root: h(0x44),
            bounty_refunded: MicroCoin::from_micro_units(3_000_000),
            epoch: SystemEpoch::new(1),
            timestamp_logical: 12,
            system_signature: SystemSignature::from_bytes([0xbbu8; 64]),
        }
    }

    fn fixture_terminal_summary_tx() -> TerminalSummaryTx {
        let mut hist = BTreeMap::new();
        hist.insert(RejectionClass::SignatureInvalid, 2);
        hist.insert(RejectionClass::StakeInsufficient, 1);
        hist.insert(
            RejectionClass::AcceptancePredicateFail(PredicateId("acc1".into())),
            5,
        );
        TerminalSummaryTx {
            tx_id: TxId("terminalsummary-fixture-01".into()),
            task_id: TaskId("task-fixture-03".into()),
            run_id: RunId("run-001".into()),
            run_outcome: RunOutcome::MaxTxExhausted,
            total_attempts: 8,
            failure_class_histogram: hist,
            last_logical_t: 13,
            system_signature: SystemSignature::from_bytes([0xccu8; 64]),
        }
    }

    /// Round-trip for every typed-tx variant.
    #[test]
    fn typed_tx_round_trip_all_variants() {
        let cases: Vec<TypedTx> = vec![
            TypedTx::Work(fixture_work_tx()),
            TypedTx::Verify(fixture_verify_tx()),
            TypedTx::Challenge(fixture_challenge_tx()),
            TypedTx::Reuse(fixture_reuse_tx()),
            TypedTx::FinalizeReward(fixture_finalize_reward_tx()),
            TypedTx::TaskExpire(fixture_task_expire_tx()),
            TypedTx::TerminalSummary(fixture_terminal_summary_tx()),
        ];
        for tx in cases {
            let bytes = canonical_encode(&tx).expect("encode");
            let decoded: TypedTx = canonical_decode(&bytes).expect("decode");
            assert_eq!(tx, decoded, "round-trip mismatch on {:?}", tx.tx_kind());
        }
    }

    /// Two encodes of the same value produce byte-identical bytes.
    #[test]
    fn typed_tx_byte_stability_across_calls() {
        let tx = TypedTx::Work(fixture_work_tx());
        let bytes_a = canonical_encode(&tx).expect("encode a");
        let bytes_b = canonical_encode(&tx).expect("encode b");
        assert_eq!(bytes_a, bytes_b);
    }

    /// 100-input round-trip: random-ish AgentSignature bytes + variant choice.
    #[test]
    fn typed_tx_round_trip_100_inputs() {
        let mut tx = fixture_work_tx();
        for i in 0u32..100 {
            tx.timestamp_logical = i as u64;
            tx.signature = AgentSignature::from_bytes([(i % 256) as u8; 64]);
            let outer = TypedTx::Work(tx.clone());
            let bytes = canonical_encode(&outer).expect("encode");
            let back: TypedTx = canonical_decode(&bytes).expect("decode");
            assert_eq!(outer, back);
        }
    }

    /// HasSubmitter — agent-submitted vs system-emitted partitioning.
    #[test]
    fn has_submitter_partitioning() {
        let alice = AgentId("alice".into());
        assert_eq!(
            TypedTx::Work(fixture_work_tx()).submitter_id(),
            Some(alice.clone())
        );
        assert_eq!(
            TypedTx::Verify(fixture_verify_tx()).submitter_id(),
            Some(AgentId("bob".into()))
        );
        assert_eq!(
            TypedTx::Challenge(fixture_challenge_tx()).submitter_id(),
            Some(AgentId("carol".into()))
        );
        assert_eq!(TypedTx::Reuse(fixture_reuse_tx()).submitter_id(), None);
        assert_eq!(
            TypedTx::FinalizeReward(fixture_finalize_reward_tx()).submitter_id(),
            None
        );
        assert_eq!(
            TypedTx::TaskExpire(fixture_task_expire_tx()).submitter_id(),
            None
        );
    }

    /// tx_kind matches the LedgerEntry TxKind enum variant.
    #[test]
    fn typed_tx_kind_projection() {
        use crate::bottom_white::ledger::transition_ledger::TxKind;
        assert_eq!(TypedTx::Work(fixture_work_tx()).tx_kind(), TxKind::Work);
        assert_eq!(
            TypedTx::Verify(fixture_verify_tx()).tx_kind(),
            TxKind::Verify
        );
        assert_eq!(
            TypedTx::Challenge(fixture_challenge_tx()).tx_kind(),
            TxKind::Challenge
        );
        assert_eq!(TypedTx::Reuse(fixture_reuse_tx()).tx_kind(), TxKind::Reuse);
        assert_eq!(
            TypedTx::FinalizeReward(fixture_finalize_reward_tx()).tx_kind(),
            TxKind::FinalizeReward
        );
        assert_eq!(
            TypedTx::TaskExpire(fixture_task_expire_tx()).tx_kind(),
            TxKind::TaskExpire
        );
        assert_eq!(
            TypedTx::TerminalSummary(fixture_terminal_summary_tx()).tx_kind(),
            TxKind::TerminalSummary,
        );
    }

    // ── v1.1 NEW: cross-variant non-collision (C-2 / Codex Q-J) ──────────────

    /// All 7 TypedTx variant fixtures encode to pairwise-distinct canonical bytes.
    /// (Different field shapes + bincode variant tags → ANY collision is a bincode
    /// regression that this test catches.)
    #[test]
    fn typed_tx_cross_variant_non_collision() {
        let variants: Vec<(&str, TypedTx)> = vec![
            ("Work", TypedTx::Work(fixture_work_tx())),
            ("Verify", TypedTx::Verify(fixture_verify_tx())),
            ("Challenge", TypedTx::Challenge(fixture_challenge_tx())),
            ("Reuse", TypedTx::Reuse(fixture_reuse_tx())),
            (
                "FinalizeReward",
                TypedTx::FinalizeReward(fixture_finalize_reward_tx()),
            ),
            ("TaskExpire", TypedTx::TaskExpire(fixture_task_expire_tx())),
            (
                "TerminalSummary",
                TypedTx::TerminalSummary(fixture_terminal_summary_tx()),
            ),
        ];
        let digests: Vec<(&str, String)> = variants
            .iter()
            .map(|(name, tx)| (*name, digest_hex(tx)))
            .collect();
        for i in 0..digests.len() {
            for j in (i + 1)..digests.len() {
                assert_ne!(
                    digests[i].1, digests[j].1,
                    "{} and {} have colliding canonical digests",
                    digests[i].0, digests[j].0
                );
            }
        }
    }

    // ── v1.1 NEW: BTreeMap / BTreeSet permutation independence (C-2 / Gemini Q9) ─

    /// Building the same WorkTx via different `BTreeSet` insertion orders produces
    /// byte-identical canonical bytes. (BTreeSet iterates in sorted order, but
    /// this test locks that bincode honors the iteration order — defensive against
    /// a future codec choice that uses HashMap-style hash-randomized iteration.)
    #[test]
    fn typed_tx_btree_permutation_independence() {
        let make_work_tx = |read_keys_in_order: &[&str]| -> WorkTx {
            let mut tx = fixture_work_tx();
            tx.read_set = BTreeSet::new();
            for k in read_keys_in_order {
                tx.read_set.insert(ReadKey((*k).into()));
            }
            tx
        };
        // Insert keys in different orders.
        let tx_a = make_work_tx(&["k.read.a", "k.read.b", "k.read.c"]);
        let tx_b = make_work_tx(&["k.read.c", "k.read.a", "k.read.b"]);
        let tx_c = make_work_tx(&["k.read.b", "k.read.c", "k.read.a"]);
        let bytes_a = canonical_encode(&tx_a).expect("encode a");
        let bytes_b = canonical_encode(&tx_b).expect("encode b");
        let bytes_c = canonical_encode(&tx_c).expect("encode c");
        assert_eq!(bytes_a, bytes_b);
        assert_eq!(bytes_a, bytes_c);
    }

    // ── v1.1 NEW: zero-default round-trip per main tx kind (Gemini Q9) ──────

    #[test]
    fn typed_tx_default_round_trip() {
        let cases: Vec<TypedTx> = vec![
            TypedTx::Work(WorkTx::default()),
            TypedTx::Verify(VerifyTx::default()),
            TypedTx::Challenge(ChallengeTx::default()),
            TypedTx::Reuse(ReuseTx::default()),
            TypedTx::FinalizeReward(FinalizeRewardTx::default()),
            TypedTx::TaskExpire(TaskExpireTx::default()),
            TypedTx::TerminalSummary(TerminalSummaryTx::default()),
        ];
        for tx in cases {
            let bytes = canonical_encode(&tx).expect("encode default");
            let back: TypedTx = canonical_decode(&bytes).expect("decode default");
            assert_eq!(tx, back, "default round-trip mismatch on {:?}", tx.tx_kind());
        }
    }

    // ── v1.1 NEW: signing-payload domain-prefix non-collision (C-1) ─────────

    /// 6 signing-payload digests (Work / Verify / Challenge agent + Finalize /
    /// TaskExpire / TerminalSummary system) all have distinct domain prefixes;
    /// even if their bincode bodies COULD overlap, the SHA-256 inputs differ.
    /// We don't construct bodies that overlap (different fields); the assertion
    /// is simply that all 6 distinct domain-prefixed digests are pairwise distinct
    /// — which is the property auditors flagged as essential.
    #[test]
    fn signing_payload_domains_are_distinct() {
        let digests: Vec<(&str, [u8; 32])> = vec![
            ("Work", fixture_work_tx().to_signing_payload().canonical_digest()),
            (
                "Verify",
                fixture_verify_tx().to_signing_payload().canonical_digest(),
            ),
            (
                "Challenge",
                fixture_challenge_tx().to_signing_payload().canonical_digest(),
            ),
            (
                "FinalizeReward",
                fixture_finalize_reward_tx()
                    .to_signing_payload()
                    .canonical_digest(),
            ),
            (
                "TaskExpire",
                fixture_task_expire_tx()
                    .to_signing_payload()
                    .canonical_digest(),
            ),
            (
                "TerminalSummary",
                fixture_terminal_summary_tx()
                    .to_signing_payload()
                    .canonical_digest(),
            ),
        ];
        for i in 0..digests.len() {
            for j in (i + 1)..digests.len() {
                assert_ne!(
                    digests[i].1, digests[j].1,
                    "{} and {} signing-payload digests collide",
                    digests[i].0, digests[j].0
                );
            }
        }
    }

    /// Excluding the signature: mutating `tx.signature` must NOT change the
    /// signing-payload digest (the signature is its own input — a canonical
    /// digest cycle prevention property).
    #[test]
    fn signing_payload_excludes_signature() {
        // WorkTx (agent-signed)
        let tx_clean = fixture_work_tx();
        let d_clean = tx_clean.to_signing_payload().canonical_digest();
        let mut tx_mut = tx_clean.clone();
        tx_mut.signature = AgentSignature::from_bytes([0xff; 64]);
        let d_mut_sig = tx_mut.to_signing_payload().canonical_digest();
        assert_eq!(d_clean, d_mut_sig, "Work: mutating signature must NOT affect digest");

        // VerifyTx (agent-signed)
        let v_clean = fixture_verify_tx();
        let dv_clean = v_clean.to_signing_payload().canonical_digest();
        let mut v_mut = v_clean.clone();
        v_mut.signature = AgentSignature::from_bytes([0xee; 64]);
        assert_eq!(
            dv_clean,
            v_mut.to_signing_payload().canonical_digest(),
            "Verify: mutating signature must NOT affect digest"
        );

        // ChallengeTx (agent-signed)
        let c_clean = fixture_challenge_tx();
        let dc_clean = c_clean.to_signing_payload().canonical_digest();
        let mut c_mut = c_clean.clone();
        c_mut.signature = AgentSignature::from_bytes([0xdd; 64]);
        assert_eq!(
            dc_clean,
            c_mut.to_signing_payload().canonical_digest(),
            "Challenge: mutating signature must NOT affect digest"
        );

        // FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx (system-signed)
        let f_clean = fixture_finalize_reward_tx();
        let df_clean = f_clean.to_signing_payload().canonical_digest();
        let mut f_mut = f_clean.clone();
        f_mut.system_signature = SystemSignature::from_bytes([0x11; 64]);
        assert_eq!(
            df_clean,
            f_mut.to_signing_payload().canonical_digest(),
            "FinalizeReward: mutating signature must NOT affect digest"
        );
        let t_clean = fixture_task_expire_tx();
        let dt_clean = t_clean.to_signing_payload().canonical_digest();
        let mut t_mut = t_clean.clone();
        t_mut.system_signature = SystemSignature::from_bytes([0x22; 64]);
        assert_eq!(
            dt_clean,
            t_mut.to_signing_payload().canonical_digest(),
            "TaskExpire: mutating signature must NOT affect digest"
        );
        let ts_clean = fixture_terminal_summary_tx();
        let dts_clean = ts_clean.to_signing_payload().canonical_digest();
        let mut ts_mut = ts_clean.clone();
        ts_mut.system_signature = SystemSignature::from_bytes([0x33; 64]);
        assert_eq!(
            dts_clean,
            ts_mut.to_signing_payload().canonical_digest(),
            "TerminalSummary: mutating signature must NOT affect digest"
        );

        // Sanity: mutating a SIGNED field DOES change digest.
        let mut tx_signed_change = tx_clean.clone();
        tx_signed_change.timestamp_logical = 9999;
        let d_signed = tx_signed_change.to_signing_payload().canonical_digest();
        assert_ne!(d_clean, d_signed);
    }

    // ── v1.2 NEW (R2-4 Codex round-2): LOAD-BEARING domain test ─────────────

    /// Hash the SAME body bytes with each of the 6 domain prefixes; assert all
    /// 6 results are pairwise distinct. Without the domain prefix, this test
    /// would FAIL — proving the prefix is load-bearing (the round-1 test
    /// `signing_payload_domains_are_distinct` used different bodies and
    /// would have passed even without domains).
    #[test]
    fn signing_payload_domain_prefix_is_load_bearing() {
        // Identical 64-byte body across all domains; the only thing that varies
        // is which domain prefix gets prepended before SHA-256.
        let body: Vec<u8> = (0..64u8).collect();
        let domains: &[&[u8]] = &[
            DOMAIN_AGENT_WORK,
            DOMAIN_AGENT_VERIFY,
            DOMAIN_AGENT_CHALLENGE,
            DOMAIN_SYSTEM_FINALIZE_REWARD,
            DOMAIN_SYSTEM_TASK_EXPIRE,
            DOMAIN_SYSTEM_TERMINAL_SUMMARY,
        ];
        let digests: Vec<[u8; 32]> = domains
            .iter()
            .map(|d| {
                let mut h = Sha256::new();
                h.update(d);
                h.update(&body);
                h.finalize().into()
            })
            .collect();
        for i in 0..digests.len() {
            for j in (i + 1)..digests.len() {
                assert_ne!(
                    digests[i], digests[j],
                    "domains {} and {} produced identical digests on identical body",
                    String::from_utf8_lossy(domains[i]),
                    String::from_utf8_lossy(domains[j])
                );
            }
        }
    }

    // ── v1.2 NEW (P15 Codex round-2 secondary): BTreeMap permutation ───────

    /// PredicateResultsBundle's `acceptance: BTreeMap<PredicateId, BoolWithProof>`
    /// must encode identically regardless of insertion order (matches the BTreeSet
    /// permutation test for read_set; closes round-2 caveat that BTreeMap
    /// fields weren't covered).
    #[test]
    fn typed_tx_btreemap_permutation_independence() {
        let make_work_tx = |insertion_order: &[(&str, bool)]| -> WorkTx {
            let mut tx = fixture_work_tx();
            tx.predicate_results.acceptance = BTreeMap::new();
            for (k, v) in insertion_order {
                tx.predicate_results.acceptance.insert(
                    PredicateId((*k).into()),
                    BoolWithProof {
                        value: *v,
                        proof_cid: None,
                    },
                );
            }
            tx
        };
        let tx_a = make_work_tx(&[("p_a", true), ("p_b", false), ("p_c", true)]);
        let tx_b = make_work_tx(&[("p_c", true), ("p_a", true), ("p_b", false)]);
        let tx_c = make_work_tx(&[("p_b", false), ("p_c", true), ("p_a", true)]);
        let bytes_a = canonical_encode(&tx_a).expect("encode a");
        let bytes_b = canonical_encode(&tx_b).expect("encode b");
        let bytes_c = canonical_encode(&tx_c).expect("encode c");
        assert_eq!(bytes_a, bytes_b);
        assert_eq!(bytes_a, bytes_c);
    }

    // ── v1.2 NEW (R2-4): signing-payload golden hex ────────────────────────

    fn signing_digest_hex(bytes: &[u8; 32]) -> String {
        hex_lower(bytes)
    }

    /// Lock SHA-256 hex of each signing-payload's canonical_digest. Any
    /// future codec / domain / projection change diffs one of these hex strings.
    /// Locked values captured 2026-04-28.
    #[test]
    fn signing_payload_golden_digests() {
        let tests: &[(&str, [u8; 32], &str)] = &[
            (
                "Work",
                fixture_work_tx().to_signing_payload().canonical_digest(),
                EXPECTED_SIGNING_HEX_WORK,
            ),
            (
                "Verify",
                fixture_verify_tx().to_signing_payload().canonical_digest(),
                EXPECTED_SIGNING_HEX_VERIFY,
            ),
            (
                "Challenge",
                fixture_challenge_tx().to_signing_payload().canonical_digest(),
                EXPECTED_SIGNING_HEX_CHALLENGE,
            ),
            (
                "FinalizeReward",
                fixture_finalize_reward_tx()
                    .to_signing_payload()
                    .canonical_digest(),
                EXPECTED_SIGNING_HEX_FINALIZE_REWARD,
            ),
            (
                "TaskExpire",
                fixture_task_expire_tx().to_signing_payload().canonical_digest(),
                EXPECTED_SIGNING_HEX_TASK_EXPIRE,
            ),
            (
                "TerminalSummary",
                fixture_terminal_summary_tx()
                    .to_signing_payload()
                    .canonical_digest(),
                EXPECTED_SIGNING_HEX_TERMINAL_SUMMARY,
            ),
        ];
        // Collect all mismatches before panicking — useful for capturing fresh
        // hex on first run (otherwise only the first failure prints).
        let mut mismatches: Vec<String> = Vec::new();
        for (name, actual, expected) in tests {
            let actual_hex = signing_digest_hex(actual);
            if &actual_hex != expected {
                mismatches.push(format!("{name}: actual={actual_hex} expected={expected}"));
            }
        }
        assert!(
            mismatches.is_empty(),
            "signing-payload digest mismatches:\n  {}",
            mismatches.join("\n  ")
        );
    }

    const EXPECTED_SIGNING_HEX_WORK: &str =
        "534d3cf26b7419a2741fa4eb2930b37095f982cc09c75ba2ee34396675a3d685";
    const EXPECTED_SIGNING_HEX_VERIFY: &str =
        "7c0f5ff4423bf204d39ff17c5f4d8d65a19861140ed15c59f304b2eda167fb95";
    const EXPECTED_SIGNING_HEX_CHALLENGE: &str =
        "64d190a2576ba0e4a1055a0d98a7763c35f817d914ce9eb2a3a49f614b704aa4";
    const EXPECTED_SIGNING_HEX_FINALIZE_REWARD: &str =
        "74fd6bfb730b9d3e9828e4ebf8c3edb24aabb755813a058583949f08fbf5654b";
    const EXPECTED_SIGNING_HEX_TASK_EXPIRE: &str =
        "d30fcf5fd45e32975e5547e266bcc4ef16353284205009d3feb4189e8b248def";
    const EXPECTED_SIGNING_HEX_TERMINAL_SUMMARY: &str =
        "71143e56cbd0fc3bdc4d8b764af9572564f8d66b2f4062d57d3678d4a311ac12";

    // ── I-CANON-D — golden fixtures (locked SHA-256 of canonical bytes) ──────
    //
    // **v1.1 round-1 closure (C-2 / Codex Q-J / Gemini Q9)**: hex values are
    // hardcoded — any future codec / schema change causes the assertion to
    // fail, forcing a deliberate "ABI golden fixture rotation" commit with
    // re-audit. To rotate:
    //   1. Run `cargo test --lib state::typed_tx::tests::golden_` with current code
    //   2. The assertion failure messages report the new hex in the `actual` slot
    //   3. Update each `EXPECTED_HEX` constant + cite the rotation rationale in commit message

    const EXPECTED_HEX_WORK: &str =
        "6ec94fa4910ef4cc108ca8f36c202647d2cf60426d13ca0bccf777efb07b4fef";
    const EXPECTED_HEX_VERIFY: &str =
        "425b9bd7e99c427b3b7934d45a00dee3d66fc346deed72ec307de01bb3f1db99";
    const EXPECTED_HEX_CHALLENGE: &str =
        "c90be7617e9aba5a70dc8d625e654c1c712403aaf47e7734497fc0e909e8f788";
    const EXPECTED_HEX_REUSE: &str =
        "8bb33232b7c20a63a206f505179b0f64fa50acb41061aaa471ba8e4435593aed";
    const EXPECTED_HEX_FINALIZE_REWARD: &str =
        "0f5e213ec919f8e61dc998b13a4dcd49ff6e81e473850725f2ca1f27c1d65a2d";
    const EXPECTED_HEX_TASK_EXPIRE: &str =
        "835cdec950a7fd09531e03b1ab2f571ccc9a7c05b3a3e04905f0dc77078c2d60";
    const EXPECTED_HEX_TERMINAL_SUMMARY: &str =
        "f05983df19cb2af951d79216d71a64aae6b1ae960d036022f90f28039b059208";

    #[test]
    fn golden_work_tx_digest() {
        let actual = digest_hex(&TypedTx::Work(fixture_work_tx()));
        assert_eq!(actual.len(), 64);
        assert_eq!(actual, EXPECTED_HEX_WORK, "Work canonical digest changed");
    }

    #[test]
    fn golden_verify_tx_digest() {
        let actual = digest_hex(&TypedTx::Verify(fixture_verify_tx()));
        assert_eq!(actual, EXPECTED_HEX_VERIFY);
    }

    #[test]
    fn golden_challenge_tx_digest() {
        let actual = digest_hex(&TypedTx::Challenge(fixture_challenge_tx()));
        assert_eq!(actual, EXPECTED_HEX_CHALLENGE);
    }

    #[test]
    fn golden_reuse_tx_digest() {
        let actual = digest_hex(&TypedTx::Reuse(fixture_reuse_tx()));
        assert_eq!(actual, EXPECTED_HEX_REUSE);
    }

    #[test]
    fn golden_finalize_reward_tx_digest() {
        let actual = digest_hex(&TypedTx::FinalizeReward(fixture_finalize_reward_tx()));
        assert_eq!(actual, EXPECTED_HEX_FINALIZE_REWARD);
    }

    #[test]
    fn golden_task_expire_tx_digest() {
        let actual = digest_hex(&TypedTx::TaskExpire(fixture_task_expire_tx()));
        assert_eq!(actual, EXPECTED_HEX_TASK_EXPIRE);
    }

    #[test]
    fn golden_terminal_summary_tx_digest() {
        let actual = digest_hex(&TypedTx::TerminalSummary(fixture_terminal_summary_tx()));
        assert_eq!(actual, EXPECTED_HEX_TERMINAL_SUMMARY);
    }
}

```

---

# Supporting: src/bottom_white/ledger/system_keypair.rs (signing primitives)

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

# Supporting: src/state/q_state.rs (QState shape + indices)

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

Now give your INDEPENDENT round-1 audit on the bundle. Cite line numbers.

codex
I’ll audit against the checked-out code rather than only the pasted snippets, so the line citations match the workspace. I’m first loading the local guidance and locating the relevant implementation files.
exec
/bin/bash -lc 'git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
?? handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md
?? handover/audits/GEMINI_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md
?? handover/audits/run_codex_co1_7_impl_bundle_round1_audit.sh
?? handover/audits/run_gemini_co1_7_impl_bundle_round1_audit.py

exec
/bin/bash -lc 'git rev-parse --show-toplevel' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4

exec
/bin/bash -lc "rg --files -g 'AGENTS.md' -g 'CLAUDE.md' -g 'CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md' -g 'CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md' -g 'STATE_TRANSITION_SPEC_v1_2026-04-27.md'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
CLAUDE.md
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md

codex
There’s no `AGENTS.md` under the repo from `rg`; I’m checking parent scopes and the project’s `CLAUDE.md` next because this audit explicitly invokes its standards.
exec
/bin/bash -lc 'for p in / /home /home/zephryj /home/zephryj/projects /home/zephryj/projects/turingosv4; do test -f "$p/AGENTS.md" && printf '"'%s\\n' \""'$p/AGENTS.md"; done' in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
exec
/bin/bash -lc "nl -ba CLAUDE.md | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
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
    14	- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
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
    56	| `docs/experiments.md` | 创建或运行实验时 |
    57	| `docs/rules.md` | 触发规则或修改规则时 |
    58	
    59	## User
    60	独狼研究员, 零编程基础 vibe coder. 中文为主, 技术术语英文可.

exec
/bin/bash -lc 'rg -n "Audit Standard|Code Standard|STEP_B|restricted|doc-comment|hardcoded|TRACE_MATRIX" CLAUDE.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
12:## Code Standard (Art. I.1 + C-004 + C-027)
14:- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
17:## Audit Standard (Art. V.1 + C-010 + C-023 + C-035)
35:- 权威对齐文件: `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md`
36:  (后续 rev: `TRACE_MATRIX_vN.md`)
38:  或 BLOCK merge。doc-comment backlink 格式: `/// TRACE_MATRIX <FC-id>: <role>`

codex
The local `CLAUDE.md` confirms the dual-audit gate and no special `AGENTS.md` applies. I’m now reading the exact implementation with line numbers and will run the relevant test suite after the first pass.
exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '1,760p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! L4 Transition Ledger (CO1.7) — type skeleton + pure helpers.
     2	//!
     3	//! TRACE_MATRIX FC2-Append: canonical envelope appended to L4 once a transition is accepted.
     4	//! TRACE_MATRIX WP § 5.L4: ChainTape Layer 4 spine; one LedgerEntry per accepted transition.
     5	//! TRACE_MATRIX § 1 (CO1_7_TRANSITION_LEDGER_v1_2026-04-28 v1.1): schema + append() + replay_chain_integrity() pseudocode.
     6	//!
     7	//! **Status**: v1.1 type skeleton — round-1 dual audit returned CHALLENGE/CHALLENGE; this
     8	//! version closes 11 must-fix items (C1/C2/C3 + K1-K7 + G1 + D1). Awaiting round-2.
     9	//! All bodies that depend on yet-to-implement transition functions or CAS index
    10	//! persistence are stubbed; full-mode replay is deferred to CO1.7.5+.
    11	//!
    12	//! v1 → v1.1 changes (smoke for round-2 dual audit):
    13	//! - C1: two-mode replay enum (ChainOnly v1; FullTransition CO1.7.5+); skeleton now
    14	//!   exposes `replay_chain_integrity` only (renamed for honesty).
    15	//! - K1: sequencer dual-counter design — documented in spec § 3; skeleton has no
    16	//!   sequencer code (deferred to CO1.7.5).
    17	//! - K2: `parent_ledger_root: Hash` field added + bound in signing payload (transplant
    18	//!   defense); new test asserts replay rejects parent_ledger_root tamper.
    19	//! - K3: L4/L5 boundary clarified — CO1.7 owns ledger_root + commit-chain head_t;
    20	//!   CO1.8 owns state_root mutation. Skeleton reflects boundary (no state_root mutation).
    21	//! - K5: `TxKind::Slash` DROPPED for v4 (deferred to CO P2.5).
    22	//! - K6: `#[repr(u8)]` + explicit discriminants on TxKind.
    23	//! - K7: +2 conformance tests (parent_ledger_root tamper, digest exclusion).
    24	//! - G1: `extensions: BTreeMap<String, Vec<u8>>` forward-compat field (empty in v1).
    25	//! - C3 / Q8: signing target is `LedgerEntrySigningPayload` (separate struct) ready to
    26	//!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
    27	//!   `system_keypair` (Wave 4-B additive extension). Skeleton has the payload struct
    28	//!   + canonical_digest method; the actual CanonicalMessage extension is deferred.
    29	//! - Q9: canonical_digest now lives on LedgerEntrySigningPayload, not LedgerEntry —
    30	//!   structurally enforces "derivatives excluded".
    31	//! - D1: epoch is bound in signing payload (Codex security wins over Gemini orthogonality).
    32	
    33	use std::collections::BTreeMap;
    34	use std::path::{Path, PathBuf};
    35	
    36	use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
    37	use serde::{Deserialize, Serialize};
    38	use sha2::{Digest, Sha256};
    39	
    40	use crate::bottom_white::cas::schema::Cid;
    41	use crate::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
    42	use crate::state::q_state::Hash;
    43	
    44	// ────────────────────────────────────────────────────────────────────────────
    45	// § 1 LedgerEntry — the stored record (11 fields per v1.1)
    46	// ────────────────────────────────────────────────────────────────────────────
    47	
    48	/// TRACE_MATRIX FC2-Append: discriminator for the typed payload behind a CAS Cid.
    49	/// **K6**: `#[repr(u8)]` + explicit discriminants for stable cast in canonical digest.
    50	/// **K5**: NO `Slash` variant — ChallengeCourt slash event deferred to CO P2.5 atom.
    51	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    52	#[repr(u8)]
    53	pub enum TxKind {
    54	    Work            = 0,
    55	    Verify          = 1,
    56	    Challenge       = 2,
    57	    Reuse           = 3,
    58	    FinalizeReward  = 4,
    59	    TaskExpire      = 5,
    60	    TerminalSummary = 6,
    61	}
    62	
    63	/// TRACE_MATRIX FC2-Append + WP § 5.L4: stored LedgerEntry record (11 fields).
    64	///
    65	/// Distinct from `LedgerEntrySigningPayload`: this is the FULL stored record
    66	/// (includes derivatives + signature); the signing payload is the subset that
    67	/// the system keypair attests.
    68	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    69	pub struct LedgerEntry {
    70	    /// **K1**: assigned ONLY at commit (sequencer dual-counter design); rejected
    71	    /// submissions never get a logical_t.
    72	    pub logical_t: u64,                          //  1
    73	    pub parent_state_root: Hash,                 //  2
    74	    /// **K2 NEW**: parent_ledger_root before fold; bound in signed payload to
    75	    /// prevent transplant attacks.
    76	    pub parent_ledger_root: Hash,                //  3
    77	    pub tx_kind: TxKind,                         //  4
    78	    /// CAS handle (CO1.4) to canonical-serialized payload (DIV-5 5-param put).
    79	    pub tx_payload_cid: Cid,                     //  5
    80	    /// Resulting state_root post-transition (NOT mutated by L4 — accepted as
    81	    /// returned by transition function per K3 boundary).
    82	    pub resulting_state_root: Hash,              //  6
    83	    /// Resulting ledger_root after fold. Derivative; NOT in signed digest.
    84	    pub resulting_ledger_root: Hash,             //  7
    85	    pub timestamp_logical: u64,                  //  8
    86	    /// **D1 / Q10**: epoch bound in signed payload (Codex security wins).
    87	    pub epoch: SystemEpoch,                      //  9
    88	    /// **G1 NEW**: forward-compat extension map. Empty in v1; reserved for v4.x.
    89	    /// Bound in signed payload (G1 cannot bypass signature).
    90	    pub extensions: BTreeMap<String, Vec<u8>>,   // 10
    91	    /// Detached system signature over `LedgerEntrySigningPayload.canonical_digest()`.
    92	    pub system_signature: SystemSignature,       // 11
    93	}
    94	
    95	// ────────────────────────────────────────────────────────────────────────────
    96	// § 1.1 LedgerEntrySigningPayload — the signed bytes (NEW per C3 / Q9)
    97	// ────────────────────────────────────────────────────────────────────────────
    98	
    99	/// TRACE_MATRIX FC2-Append C3: the bytes the system keypair actually signs.
   100	///
   101	/// **Excludes** (Q9 cycle prevention):
   102	/// - `resulting_ledger_root` (derivative; including → cycle)
   103	/// - `system_signature` (its own input)
   104	///
   105	/// **Includes** (9 non-derivative bound fields). Domain-separation prefix is
   106	/// part of the digest to prevent cross-namespace collision.
   107	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   108	pub struct LedgerEntrySigningPayload {
   109	    pub logical_t: u64,
   110	    pub parent_state_root: Hash,
   111	    pub parent_ledger_root: Hash,                  // K2
   112	    pub tx_kind: TxKind,
   113	    pub tx_payload_cid: Cid,
   114	    pub resulting_state_root: Hash,
   115	    pub timestamp_logical: u64,
   116	    pub epoch: SystemEpoch,                        // D1
   117	    pub extensions: BTreeMap<String, Vec<u8>>,     // G1
   118	}
   119	
   120	impl LedgerEntrySigningPayload {
   121	    /// Canonical SHA-256 digest. Stable wire format (NOT bincode/serde dependent).
   122	    pub fn canonical_digest(&self) -> Hash {
   123	        let mut h = Sha256::new();
   124	        h.update(b"turingosv4.ledger_entry_signing.v1");
   125	        h.update(self.logical_t.to_be_bytes());
   126	        h.update(self.parent_state_root.0);
   127	        h.update(self.parent_ledger_root.0);
   128	        h.update((self.tx_kind as u8).to_be_bytes()); // K6 #[repr(u8)] makes cast stable
   129	        h.update(self.tx_payload_cid.0);
   130	        h.update(self.resulting_state_root.0);
   131	        h.update(self.timestamp_logical.to_be_bytes());
   132	        h.update(self.epoch.get().to_be_bytes());
   133	        // Extensions: BTreeMap iterates in lex key order (deterministic);
   134	        // length-prefix every field to prevent ambiguity attacks.
   135	        h.update((self.extensions.len() as u64).to_be_bytes());
   136	        for (k, v) in &self.extensions {
   137	            h.update((k.len() as u64).to_be_bytes());
   138	            h.update(k.as_bytes());
   139	            h.update((v.len() as u64).to_be_bytes());
   140	            h.update(v);
   141	        }
   142	        Hash(h.finalize().into())
   143	    }
   144	}
   145	
   146	impl LedgerEntry {
   147	    /// Project the LedgerEntry's signed-fields-subset back into a signing payload.
   148	    /// Used by replay to recompute `signing_digest` and re-verify chain integrity.
   149	    pub fn to_signing_payload(&self) -> LedgerEntrySigningPayload {
   150	        LedgerEntrySigningPayload {
   151	            logical_t: self.logical_t,
   152	            parent_state_root: self.parent_state_root,
   153	            parent_ledger_root: self.parent_ledger_root,
   154	            tx_kind: self.tx_kind,
   155	            tx_payload_cid: self.tx_payload_cid,
   156	            resulting_state_root: self.resulting_state_root,
   157	            timestamp_logical: self.timestamp_logical,
   158	            epoch: self.epoch,
   159	            extensions: self.extensions.clone(),
   160	        }
   161	    }
   162	}
   163	
   164	// ────────────────────────────────────────────────────────────────────────────
   165	// § 4 append() — pure ledger-root fold
   166	// ────────────────────────────────────────────────────────────────────────────
   167	
   168	/// TRACE_MATRIX FC2-Append + spec § 4: pure ledger-root fold over signed digests.
   169	/// Same `(parent_root, signing_digest)` → byte-identical `new_root`.
   170	/// No I/O, no clock, no env. Witness for I-DET ledger axis.
   171	pub fn append(parent_root: &Hash, signing_digest: &Hash) -> Hash {
   172	    let mut h = Sha256::new();
   173	    h.update(b"turingosv4.ledger_root.v1");
   174	    h.update(parent_root.0);
   175	    h.update(signing_digest.0);
   176	    Hash(h.finalize().into())
   177	}
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
   286	}
   287	
   288	impl std::fmt::Display for ReplayError {
   289	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   290	        match self {
   291	            Self::LogicalTGap { at, expected, got } => {
   292	                write!(f, "logical_t gap at index {at}: expected {expected}, got {got}")
   293	            }
   294	            Self::ParentStateMismatch { at } => write!(f, "parent_state_root mismatch at index {at}"),
   295	            Self::ParentLedgerMismatch { at } => write!(f, "parent_ledger_root mismatch at index {at}"),
   296	            Self::LedgerRootMismatch { at } => write!(f, "ledger_root mismatch at index {at}"),
   297	            Self::BadSignature { at } => write!(f, "system_signature verify failed at index {at}"),
   298	            Self::CasMissing { at } => write!(f, "CAS payload not retrievable at index {at}"),
   299	            Self::StateRootMismatch { at } => write!(f, "resulting_state_root divergence at index {at}"),
   300	            Self::Transition { at, inner } => write!(f, "dispatch_transition rejected at index {at}: {inner}"),
   301	        }
   302	    }
   303	}
   304	impl std::error::Error for ReplayError {}
   305	
   306	// ────────────────────────────────────────────────────────────────────────────
   307	// CO1.7-impl A4: LedgerCasView trait + replay_full_transition
   308	// ────────────────────────────────────────────────────────────────────────────
   309	
   310	/// CO1.7 spec § 4 + DIV-4 closure: narrow read-only CAS trait that replay
   311	/// needs. Decouples `replay_full_transition` from full `CasStore` (the
   312	/// production impl). Anything that can hand back the bytes for a `Cid`
   313	/// satisfies this — testing can mock it; cold-replay uses CasStore directly.
   314	pub trait LedgerCasView {
   315	    fn get_typed_payload(
   316	        &self,
   317	        cid: &crate::bottom_white::cas::schema::Cid,
   318	    ) -> Result<Vec<u8>, ReplayError>;
   319	}
   320	
   321	impl LedgerCasView for crate::bottom_white::cas::store::CasStore {
   322	    fn get_typed_payload(
   323	        &self,
   324	        cid: &crate::bottom_white::cas::schema::Cid,
   325	    ) -> Result<Vec<u8>, ReplayError> {
   326	        self.get(cid).map_err(|_| ReplayError::CasMissing { at: 0 })
   327	    }
   328	}
   329	
   330	/// CO1.7-impl A4 — full-mode replay (THE I-DETHASH witness).
   331	///
   332	/// Validates **every** stage spec § 4 + § 6 promises:
   333	/// 1. logical_t monotonicity
   334	/// 2. parent_state_root chain
   335	/// 3. parent_ledger_root chain (K2 transplant defense)
   336	/// 4. system_signature verifies via CanonicalMessage::LedgerEntrySigning + pinned pubkeys
   337	/// 5. CAS lookup of tx_payload_cid succeeds (CO1.4-extra cold-replay capability)
   338	/// 6. canonical_decode of payload bytes → TypedTx
   339	/// 7. dispatch_transition re-run produces (q_next, _signals)
   340	/// 8. q_next.state_root_t matches entry.resulting_state_root
   341	/// 9. resulting_ledger_root recomputed via append() matches stored
   342	///
   343	/// **Stub-state caveat (CO1.7.5 unblocks)**: while `dispatch_transition`
   344	/// returns `NotYetImplemented` for every variant, replay errors at stage 7
   345	/// for any non-empty chain. Conformance tests exercising stages 1-6
   346	/// independently are `#[test]`-runnable now; full state_root reconstruction
   347	/// gates on CO1.7.5.
   348	pub fn replay_full_transition(
   349	    genesis_state_root: crate::state::q_state::Hash,
   350	    genesis_ledger_root: crate::state::q_state::Hash,
   351	    entries: &[LedgerEntry],
   352	    cas: &dyn LedgerCasView,
   353	    pinned_pubkeys: &crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys,
   354	    predicate_registry: &crate::top_white::predicates::registry::PredicateRegistry,
   355	    tool_registry: &crate::bottom_white::tools::registry::ToolRegistry,
   356	) -> Result<(crate::state::q_state::Hash, crate::state::q_state::Hash), ReplayError> {
   357	    use crate::bottom_white::ledger::system_keypair::{
   358	        verify_system_signature, CanonicalMessage,
   359	    };
   360	    use crate::state::q_state::QState;
   361	    use crate::state::sequencer::dispatch_transition;
   362	
   363	    let mut prev_state_root = genesis_state_root;
   364	    let mut prev_ledger_root = genesis_ledger_root;
   365	    // For dispatch we need a QState. Replay reconstructs it from genesis;
   366	    // initial state has empty agent swarm + budget defaults. The state_root_t
   367	    // and ledger_root_t are the load-bearing fields entries verify against.
   368	    let mut q = QState::genesis();
   369	    q.state_root_t = genesis_state_root;
   370	    q.ledger_root_t = genesis_ledger_root;
   371	
   372	    for (i, entry) in entries.iter().enumerate() {
   373	        // Stage 1
   374	        let expected_logical_t = (i as u64) + 1;
   375	        if entry.logical_t != expected_logical_t {
   376	            return Err(ReplayError::LogicalTGap {
   377	                at: i,
   378	                expected: expected_logical_t,
   379	                got: entry.logical_t,
   380	            });
   381	        }
   382	        // Stage 2
   383	        if entry.parent_state_root != prev_state_root {
   384	            return Err(ReplayError::ParentStateMismatch { at: i });
   385	        }
   386	        // Stage 3
   387	        if entry.parent_ledger_root != prev_ledger_root {
   388	            return Err(ReplayError::ParentLedgerMismatch { at: i });
   389	        }
   390	
   391	        // Stage 4: system_signature verify (FullTransition mode only).
   392	        let signing_payload = entry.to_signing_payload();
   393	        let signing_digest = signing_payload.canonical_digest();
   394	        let canonical_msg = CanonicalMessage::LedgerEntrySigning(signing_digest.0);
   395	        if !verify_system_signature(
   396	            &entry.system_signature,
   397	            &canonical_msg,
   398	            entry.epoch,
   399	            pinned_pubkeys,
   400	        ) {
   401	            return Err(ReplayError::BadSignature { at: i });
   402	        }
   403	
   404	        // Stage 5: CAS lookup.
   405	        let payload_bytes = cas
   406	            .get_typed_payload(&entry.tx_payload_cid)
   407	            .map_err(|_| ReplayError::CasMissing { at: i })?;
   408	
   409	        // Stage 6: canonical_decode → TypedTx.
   410	        let typed_tx: crate::state::typed_tx::TypedTx = canonical_decode(&payload_bytes)
   411	            .map_err(|_| ReplayError::CasMissing { at: i })?;
   412	
   413	        // Stage 7: re-run pure dispatch_transition.
   414	        let (q_next, _signals) =
   415	            dispatch_transition(&q, &typed_tx, predicate_registry, tool_registry)
   416	                .map_err(|inner| ReplayError::Transition { at: i, inner })?;
   417	
   418	        // Stage 8: state_root match.
   419	        if q_next.state_root_t != entry.resulting_state_root {
   420	            return Err(ReplayError::StateRootMismatch { at: i });
   421	        }
   422	
   423	        // Stage 9: ledger_root match (recompute via append).
   424	        let recomputed_ledger_root = append(&prev_ledger_root, &signing_digest);
   425	        if recomputed_ledger_root != entry.resulting_ledger_root {
   426	            return Err(ReplayError::LedgerRootMismatch { at: i });
   427	        }
   428	
   429	        // Advance.
   430	        q = q_next;
   431	        q.ledger_root_t = entry.resulting_ledger_root;
   432	        prev_state_root = entry.resulting_state_root;
   433	        prev_ledger_root = entry.resulting_ledger_root;
   434	    }
   435	
   436	    Ok((prev_state_root, prev_ledger_root))
   437	}
   438	
   439	/// Skeleton-stage entry point (v1.1).
   440	///
   441	/// Validates:
   442	/// 1. logical_t monotonicity (no gaps, no duplicates)
   443	/// 2. parent_state_root chain
   444	/// 3. parent_ledger_root chain (K2 transplant defense)
   445	/// 4. resulting_ledger_root recomputed via append(prev_ledger_root, signing_digest)
   446	///
   447	/// Does NOT verify:
   448	/// - system_signature (CO1.7.5+: requires CanonicalMessage extension wired through keypair)
   449	/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
   450	///
   451	/// Returns final (state_root, ledger_root) on success.
   452	pub fn replay_chain_integrity(
   453	    genesis_state_root: Hash,
   454	    genesis_ledger_root: Hash,
   455	    entries: &[LedgerEntry],
   456	) -> Result<(Hash, Hash), ReplayError> {
   457	    let mut prev_state_root = genesis_state_root;
   458	    let mut prev_ledger_root = genesis_ledger_root;
   459	
   460	    for (i, entry) in entries.iter().enumerate() {
   461	        let expected_logical_t = (i as u64) + 1;
   462	        if entry.logical_t != expected_logical_t {
   463	            return Err(ReplayError::LogicalTGap {
   464	                at: i,
   465	                expected: expected_logical_t,
   466	                got: entry.logical_t,
   467	            });
   468	        }
   469	        if entry.parent_state_root != prev_state_root {
   470	            return Err(ReplayError::ParentStateMismatch { at: i });
   471	        }
   472	        // K2 NEW: parent_ledger_root chain check
   473	        if entry.parent_ledger_root != prev_ledger_root {
   474	            return Err(ReplayError::ParentLedgerMismatch { at: i });
   475	        }
   476	        let signing_digest = entry.to_signing_payload().canonical_digest();
   477	        let recomputed = append(&prev_ledger_root, &signing_digest);
   478	        if recomputed != entry.resulting_ledger_root {
   479	            return Err(ReplayError::LedgerRootMismatch { at: i });
   480	        }
   481	        prev_state_root = entry.resulting_state_root;
   482	        prev_ledger_root = entry.resulting_ledger_root;
   483	    }
   484	
   485	    Ok((prev_state_root, prev_ledger_root))
   486	}
   487	
   488	// ────────────────────────────────────────────────────────────────────────────
   489	// § 2.5 Canonical serialization (bincode v2; STATE_TRANSITION_SPEC § 2.5)
   490	// ────────────────────────────────────────────────────────────────────────────
   491	
   492	/// `bincode::config` used for the canonical `LedgerEntry` wire format.
   493	///
   494	/// **Frozen choices** (per STATE_TRANSITION_SPEC § 2.5):
   495	/// - **Big-endian** byte order (network order; deterministic across platforms).
   496	/// - **Fixed-int encoding** (no varint; fixed-width for byte-stable round-trip).
   497	/// - **`BTreeMap` keys**: bincode iterates the map in serde-supplied order; we
   498	///   only ever encode `BTreeMap` (sorted by construction) so key order is lex.
   499	/// - **No padding, no implicit alignment.**
   500	fn bincode_canonical_config() -> impl bincode::config::Config {
   501	    bincode::config::standard()
   502	        .with_big_endian()
   503	        .with_fixed_int_encoding()
   504	}
   505	
   506	/// Canonical encode any serde-Serialize value to bytes (CO1.7 wire format).
   507	/// Used by `Git2LedgerWriter` for commit-message bodies and by future callers
   508	/// needing byte-stable signatures over typed payloads.
   509	pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalCodecError> {
   510	    bincode::serde::encode_to_vec(value, bincode_canonical_config())
   511	        .map_err(|e| CanonicalCodecError::Encode(e.to_string()))
   512	}
   513	
   514	/// Canonical decode the inverse of `canonical_encode`. Returns the value plus
   515	/// the number of bytes consumed (entire input must be consumed for a clean decode).
   516	pub fn canonical_decode<T: serde::de::DeserializeOwned>(
   517	    bytes: &[u8],
   518	) -> Result<T, CanonicalCodecError> {
   519	    let (value, consumed) =
   520	        bincode::serde::decode_from_slice::<T, _>(bytes, bincode_canonical_config())
   521	            .map_err(|e| CanonicalCodecError::Decode(e.to_string()))?;
   522	    if consumed != bytes.len() {
   523	        return Err(CanonicalCodecError::TrailingBytes {
   524	            consumed,
   525	            total: bytes.len(),
   526	        });
   527	    }
   528	    Ok(value)
   529	}
   530	
   531	#[derive(Debug)]
   532	pub enum CanonicalCodecError {
   533	    Encode(String),
   534	    Decode(String),
   535	    TrailingBytes { consumed: usize, total: usize },
   536	}
   537	
   538	impl std::fmt::Display for CanonicalCodecError {
   539	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   540	        match self {
   541	            Self::Encode(s) => write!(f, "canonical encode failed: {s}"),
   542	            Self::Decode(s) => write!(f, "canonical decode failed: {s}"),
   543	            Self::TrailingBytes { consumed, total } => {
   544	                write!(f, "trailing bytes after decode: consumed {consumed} of {total}")
   545	            }
   546	        }
   547	    }
   548	}
   549	impl std::error::Error for CanonicalCodecError {}
   550	
   551	// ────────────────────────────────────────────────────────────────────────────
   552	// § 5 Git2LedgerWriter — git2-rs commit chain on `refs/transitions/main`
   553	// ────────────────────────────────────────────────────────────────────────────
   554	
   555	/// Spec § 5 production storage backend.
   556	///
   557	/// **Mapping**:
   558	/// - One `LedgerEntry` = one git commit on `refs/transitions/main`.
   559	/// - **Commit tree** = three named blobs:
   560	///     - `payload_cid`     = entry.tx_payload_cid.0 (32 bytes)
   561	///     - `signature`       = entry.system_signature.as_bytes() (64 bytes)
   562	///     - `entry_canonical` = bincode v2 BE + fixed-int encoding of the full
   563	///       `LedgerEntry` (deterministic, byte-stable; this blob IS the
   564	///       canonical record — `read_at` decodes it directly).
   565	/// - **Commit message** = human-readable `"transition logical_t=<N>\n"` (the
   566	///   canonical record lives in the tree blob, not the message — git
   567	///   normalizes message bytes in ways that break round-trip).
   568	/// - **Parent**: `head_t-1` commit (or none at genesis).
   569	/// - **Author/committer identity**: fixed `("turingosv4 sequencer", "system@turingos")`
   570	///   with `time = (logical_t as i64, 0)` to keep commit OIDs deterministic. NO
   571	///   wall-clock leakage (`I-NOENV` + `I-LOGTIME`).
   572	///
   573	/// **K3 (revised v1.2)**: this writer surfaces `commit_oid` for callers that
   574	/// need it (CO1.7.5+ `head_t` wiring), but the `LedgerWriter::commit` trait
   575	/// returns only `Hash` (entry.resulting_ledger_root). Callers requesting the
   576	/// commit OID use [`Git2LedgerWriter::head_commit_oid`] post-commit.
   577	pub struct Git2LedgerWriter {
   578	    repo_path: PathBuf,
   579	    /// Last commit OID on `refs/transitions/main`; `None` at empty-chain genesis.
   580	    head_oid: Option<git2::Oid>,
   581	    /// Number of entries committed = highest assigned `logical_t` (0 at genesis).
   582	    len: u64,
   583	}
   584	
   585	const TRANSITIONS_REF: &str = "refs/transitions/main";
   586	const TREE_BLOB_PAYLOAD_CID: &str = "payload_cid";
   587	const TREE_BLOB_SIGNATURE: &str = "signature";
   588	const TREE_BLOB_ENTRY_CANONICAL: &str = "entry_canonical";
   589	
   590	impl Git2LedgerWriter {
   591	    /// Open or initialize a `Git2LedgerWriter` rooted at `repo_path`.
   592	    /// Creates the underlying git repo if it doesn't exist; resolves the
   593	    /// existing `refs/transitions/main` if present and seeds `head_oid` + `len`.
   594	    pub fn open(repo_path: &Path) -> Result<Self, LedgerWriterError> {
   595	        let repo_path = repo_path.to_path_buf();
   596	        let repo = match Repository::open(&repo_path) {
   597	            Ok(r) => r,
   598	            Err(_) => Repository::init(&repo_path).map_err(|e| {
   599	                LedgerWriterError::BackendCorruption(format!("repo init: {e}"))
   600	            })?,
   601	        };
   602	
   603	        // Resolve refs/transitions/main if it exists.
   604	        let (head_oid, len) = match repo.find_reference(TRANSITIONS_REF) {
   605	            Ok(reference) => {
   606	                let oid = reference
   607	                    .target()
   608	                    .ok_or_else(|| {
   609	                        LedgerWriterError::BackendCorruption(format!(
   610	                            "{TRANSITIONS_REF} has no direct target"
   611	                        ))
   612	                    })?;
   613	                // Walk parents to count chain length.
   614	                let mut n: u64 = 0;
   615	                let mut cursor = Some(oid);
   616	                while let Some(c) = cursor {
   617	                    n += 1;
   618	                    let commit = repo.find_commit(c).map_err(|e| {
   619	                        LedgerWriterError::BackendCorruption(format!("walk parent: {e}"))
   620	                    })?;
   621	                    cursor = commit.parent(0).ok().map(|p| p.id());
   622	                }
   623	                (Some(oid), n)
   624	            }
   625	            Err(_) => (None, 0),
   626	        };
   627	
   628	        Ok(Self {
   629	            repo_path,
   630	            head_oid,
   631	            len,
   632	        })
   633	    }
   634	
   635	    fn open_repo(&self) -> Result<Repository, LedgerWriterError> {
   636	        Repository::open(&self.repo_path)
   637	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("repo open: {e}")))
   638	    }
   639	
   640	    /// Commit OID of the most recent appended entry (None if chain is empty).
   641	    /// CO1.7.5+ `head_t` wiring uses this to surface commit_sha alongside Hash.
   642	    pub fn head_commit_oid(&self) -> Option<git2::Oid> {
   643	        self.head_oid
   644	    }
   645	
   646	    /// Read raw canonical-encoded `LedgerEntry` bytes (the `entry_canonical`
   647	    /// tree blob) for the entry at `logical_t`. `logical_t` is 1-indexed.
   648	    fn read_canonical_bytes(&self, logical_t: u64) -> Result<Vec<u8>, LedgerWriterError> {
   649	        if logical_t == 0 || logical_t > self.len {
   650	            return Err(LedgerWriterError::NotFound { logical_t });
   651	        }
   652	        let repo = self.open_repo()?;
   653	        // Walk back (len - logical_t) parents from head.
   654	        let mut cursor = self.head_oid.ok_or(LedgerWriterError::NotFound { logical_t })?;
   655	        let mut steps_back = self.len - logical_t;
   656	        while steps_back > 0 {
   657	            let commit = repo.find_commit(cursor).map_err(|e| {
   658	                LedgerWriterError::BackendCorruption(format!("find_commit: {e}"))
   659	            })?;
   660	            cursor = commit
   661	                .parent(0)
   662	                .map_err(|e| LedgerWriterError::BackendCorruption(format!("parent: {e}")))?
   663	                .id();
   664	            steps_back -= 1;
   665	        }
   666	        let commit = repo
   667	            .find_commit(cursor)
   668	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_commit: {e}")))?;
   669	        let tree = commit
   670	            .tree()
   671	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree: {e}")))?;
   672	        let entry_obj = tree
   673	            .get_name(TREE_BLOB_ENTRY_CANONICAL)
   674	            .ok_or_else(|| {
   675	                LedgerWriterError::BackendCorruption(format!(
   676	                    "missing {TREE_BLOB_ENTRY_CANONICAL} blob at logical_t={logical_t}"
   677	                ))
   678	            })?;
   679	        let blob = repo
   680	            .find_blob(entry_obj.id())
   681	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_blob: {e}")))?;
   682	        Ok(blob.content().to_vec())
   683	    }
   684	}
   685	
   686	impl LedgerWriter for Git2LedgerWriter {
   687	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
   688	        let expected = self.len + 1;
   689	        if entry.logical_t != expected {
   690	            return Err(LedgerWriterError::LogicalTGap {
   691	                expected,
   692	                got: entry.logical_t,
   693	            });
   694	        }
   695	
   696	        let repo = self.open_repo()?;
   697	        let canonical = canonical_encode(entry).map_err(|e| {
   698	            LedgerWriterError::BackendCorruption(format!("canonical_encode: {e}"))
   699	        })?;
   700	
   701	        let mut tb = repo
   702	            .treebuilder(None)
   703	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("treebuilder: {e}")))?;
   704	        let cid_blob = repo
   705	            .blob(&entry.tx_payload_cid.0)
   706	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("cid blob: {e}")))?;
   707	        tb.insert(TREE_BLOB_PAYLOAD_CID, cid_blob, 0o100644)
   708	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert cid: {e}")))?;
   709	        let sig_blob = repo
   710	            .blob(entry.system_signature.as_bytes())
   711	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("sig blob: {e}")))?;
   712	        tb.insert(TREE_BLOB_SIGNATURE, sig_blob, 0o100644)
   713	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert sig: {e}")))?;
   714	        let entry_blob = repo
   715	            .blob(&canonical)
   716	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("entry blob: {e}")))?;
   717	        tb.insert(TREE_BLOB_ENTRY_CANONICAL, entry_blob, 0o100644)
   718	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert entry: {e}")))?;
   719	        let tree_oid = tb
   720	            .write()
   721	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree write: {e}")))?;
   722	        let tree = repo
   723	            .find_tree(tree_oid)
   724	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_tree: {e}")))?;
   725	
   726	        // Determinism: time = (logical_t, 0). NO wall clock.
   727	        let time = git2::Time::new(entry.logical_t as i64, 0);
   728	        let author = GitSignature::new("turingosv4 sequencer", "system@turingos", &time)
   729	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("git sig: {e}")))?;
   730	        let committer = author.clone();
   731	
   732	        let parents: Vec<git2::Commit<'_>> = match self.head_oid {
   733	            Some(oid) => vec![repo.find_commit(oid).map_err(|e| {
   734	                LedgerWriterError::BackendCorruption(format!("parent commit: {e}"))
   735	            })?],
   736	            None => Vec::new(),
   737	        };
   738	        let parent_refs: Vec<&git2::Commit<'_>> = parents.iter().collect();
   739	        let message = format!("transition logical_t={}\n", entry.logical_t);
   740	        let new_oid = repo
   741	            .commit(
   742	                Some(TRANSITIONS_REF),
   743	                &author,
   744	                &committer,
   745	                &message,
   746	                &tree,
   747	                &parent_refs,
   748	            )
   749	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("commit: {e}")))?;
   750	
   751	        self.head_oid = Some(new_oid);
   752	        self.len += 1;
   753	        Ok(entry.resulting_ledger_root)
   754	    }
   755	
   756	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
   757	        let bytes = self.read_canonical_bytes(logical_t)?;
   758	        canonical_decode::<LedgerEntry>(&bytes).map_err(|e| {
   759	            LedgerWriterError::BackendCorruption(format!("canonical_decode at {logical_t}: {e}"))
   760	        })

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '1,520p'" in /home/zephryj/projects/turingosv4
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
   273	    pub(crate) fn apply_one(&self, tx: TypedTx) -> Result<LedgerEntry, ApplyError> {
   274	        // Stage 1: snapshot Q_t under read lock.
   275	        let q_snapshot = {
   276	            let g = self.q.read().map_err(|_| ApplyError::QStateLockPoisoned)?;
   277	            g.clone()
   278	        };
   279	
   280	        // Stage 2: dispatch (pure). On reject (incl. NotYetImplemented stub),
   281	        // EARLY RETURN. K1: no logical_t consumed.
   282	        let (q_next, _signals) = dispatch_transition(
   283	            &q_snapshot,
   284	            &tx,
   285	            &self.predicate_registry,
   286	            &self.tool_registry,
   287	        )?;
   288	
   289	        // Stage 3: put payload to CAS. DIV-5 5-param put signature. The
   290	        // `created_at_logical_t` is the TENTATIVE logical_t (current counter +
   291	        // 1); the final commit logical_t is assigned at stage 4 atomically.
   292	        let payload_bytes = canonical_encode(&tx)
   293	            .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
   294	        let tentative_logical_t = self.next_logical_t.load(Ordering::SeqCst) + 1;
   295	        let payload_cid = {
   296	            let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
   297	            cas_w.put(
   298	                &payload_bytes,
   299	                ObjectType::ProposalPayload,
   300	                &format!("sequencer-epoch-{}", self.epoch.get()),
   301	                tentative_logical_t,
   302	                Some("TypedTx.v1".to_string()),
   303	            )?
   304	        };
   305	
   306	        // Stage 4: K1 — assign logical_t ONLY now (post-accept).
   307	        let logical_t = self.next_logical_t.fetch_add(1, Ordering::SeqCst) + 1;
   308	
   309	        // Stage 5: build LedgerEntrySigningPayload.
   310	        let signing_payload = LedgerEntrySigningPayload {
   311	            logical_t,
   312	            parent_state_root: q_snapshot.state_root_t,
   313	            parent_ledger_root: q_snapshot.ledger_root_t,
   314	            tx_kind: tx.tx_kind(),
   315	            tx_payload_cid: payload_cid,
   316	            resulting_state_root: q_next.state_root_t,
   317	            timestamp_logical: logical_t,
   318	            epoch: self.epoch,
   319	            extensions: std::collections::BTreeMap::new(),
   320	        };
   321	
   322	        // Stage 6: C3 — sign via typed CanonicalMessage::LedgerEntrySigning.
   323	        let signing_digest = signing_payload.canonical_digest();
   324	        let system_signature = transition_ledger_emitter::sign_ledger_entry(
   325	            &self.keypair,
   326	            signing_digest.0,
   327	        )?;
   328	
   329	        // Stage 7: pure ledger-root fold (deterministic).
   330	        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);
   331	
   332	        // Stage 8: build LedgerEntry (the stored record).
   333	        let entry = LedgerEntry {
   334	            logical_t: signing_payload.logical_t,
   335	            parent_state_root: signing_payload.parent_state_root,
   336	            parent_ledger_root: signing_payload.parent_ledger_root,
   337	            tx_kind: signing_payload.tx_kind,
   338	            tx_payload_cid: signing_payload.tx_payload_cid,
   339	            resulting_state_root: signing_payload.resulting_state_root,
   340	            resulting_ledger_root,
   341	            timestamp_logical: signing_payload.timestamp_logical,
   342	            epoch: signing_payload.epoch,
   343	            extensions: signing_payload.extensions,
   344	            system_signature,
   345	        };
   346	
   347	        // Stage 9: commit + mutate Q_t under write lock.
   348	        // K3 v1.2 (revised): we set q.ledger_root_t but NOT q.head_t (head_t
   349	        // mutation deferred to CO1.7.5+ when Git2LedgerWriter exposes
   350	        // commit_sha alongside Hash). state_root_t comes from q_next as-is.
   351	        {
   352	            let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
   353	            let mut writer_w = self
   354	                .ledger_writer
   355	                .write()
   356	                .map_err(|_| ApplyError::QStateLockPoisoned)?;
   357	            writer_w.commit(&entry)?;
   358	            *q_w = q_next;
   359	            q_w.ledger_root_t = entry.resulting_ledger_root;
   360	        }
   361	
   362	        Ok(entry)
   363	    }
   364	
   365	    /// Read-only accessor (testing + CO1.7.5+ wiring).
   366	    pub fn q_snapshot(&self) -> Result<QState, ApplyError> {
   367	        self.q
   368	            .read()
   369	            .map(|g| g.clone())
   370	            .map_err(|_| ApplyError::QStateLockPoisoned)
   371	    }
   372	
   373	    pub fn next_submit_id_peek(&self) -> u64 {
   374	        self.next_submit_id.load(Ordering::SeqCst)
   375	    }
   376	
   377	    pub fn next_logical_t_peek(&self) -> u64 {
   378	        self.next_logical_t.load(Ordering::SeqCst)
   379	    }
   380	}
   381	
   382	// ────────────────────────────────────────────────────────────────────────────
   383	// Tests — stub-mode coverage (CO1.7.5 fills real-transition tests)
   384	// ────────────────────────────────────────────────────────────────────────────
   385	
   386	#[cfg(test)]
   387	mod tests {
   388	    use super::*;
   389	    use crate::bottom_white::ledger::transition_ledger::InMemoryLedgerWriter;
   390	    use crate::state::typed_tx::{
   391	        AgentSignature, BoolWithProof, ChallengeTx, ClaimId, FinalizeRewardTx, PredicateId,
   392	        PredicateResultsBundle, ReadKey, ReuseTx, RunId, RunOutcome, SafetyOrCreation,
   393	        TaskExpireTx, TaskId, TerminalSummaryTx, ToolId, VerifyTx, VerifyVerdict, WorkTx,
   394	        WriteKey,
   395	    };
   396	    use crate::state::q_state::{AgentId, TxId};
   397	    use crate::economy::money::{MicroCoin, StakeMicroCoin};
   398	    use crate::bottom_white::cas::schema::Cid;
   399	    use crate::bottom_white::ledger::system_keypair::SystemSignature;
   400	    use std::collections::{BTreeMap, BTreeSet};
   401	    use tempfile::TempDir;
   402	
   403	    fn fresh_sequencer() -> (
   404	        TempDir,
   405	        Sequencer,
   406	        tokio::sync::mpsc::Receiver<TypedTx>,
   407	    ) {
   408	        let tmp = TempDir::new().expect("tempdir");
   409	        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas open")));
   410	        let keypair = Arc::new(
   411	            Ed25519Keypair::generate_with_secure_entropy().expect("keypair gen"),
   412	        );
   413	        let epoch = SystemEpoch::new(1);
   414	        let writer: Arc<RwLock<dyn LedgerWriter>> =
   415	            Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
   416	        let preds = Arc::new(PredicateRegistry::new());
   417	        let tools = Arc::new(ToolRegistry::new());
   418	        let q = QState::genesis();
   419	        let (seq, rx) = Sequencer::new(cas, keypair, epoch, writer, preds, tools, q, 16);
   420	        (tmp, seq, rx)
   421	    }
   422	
   423	    fn fixture_work_tx() -> WorkTx {
   424	        let mut acceptance = BTreeMap::new();
   425	        acceptance.insert(
   426	            PredicateId("acc1".into()),
   427	            BoolWithProof {
   428	                value: true,
   429	                proof_cid: None,
   430	            },
   431	        );
   432	        WorkTx {
   433	            tx_id: TxId("worktx-seq-fixture".into()),
   434	            task_id: TaskId("task-seq-fixture".into()),
   435	            parent_state_root: Default::default(),
   436	            agent_id: AgentId("alice".into()),
   437	            read_set: [ReadKey("k.read.a".into())].into_iter().collect::<BTreeSet<_>>(),
   438	            write_set: [WriteKey("k.write.a".into())].into_iter().collect::<BTreeSet<_>>(),
   439	            proposal_cid: Default::default(),
   440	            predicate_results: PredicateResultsBundle {
   441	                acceptance,
   442	                settlement: BTreeMap::new(),
   443	                safety_class: SafetyOrCreation::Safety,
   444	            },
   445	            stake: StakeMicroCoin::from_micro_units(1_000_000),
   446	            signature: AgentSignature::from_bytes([0x77u8; 64]),
   447	            timestamp_logical: 1,
   448	        }
   449	    }
   450	
   451	    // 1. dispatch_transition: every variant returns NotYetImplemented (stub state).
   452	    #[test]
   453	    fn dispatch_transition_stubs_all_variants() {
   454	        let q = QState::genesis();
   455	        let preds = PredicateRegistry::new();
   456	        let tools = ToolRegistry::new();
   457	
   458	        let cases: Vec<TypedTx> = vec![
   459	            TypedTx::Work(fixture_work_tx()),
   460	            TypedTx::Verify(VerifyTx {
   461	                tx_id: TxId("vt".into()),
   462	                target_work_tx: TxId("wt".into()),
   463	                verifier_agent: AgentId("v".into()),
   464	                bond: StakeMicroCoin::from_micro_units(1),
   465	                verdict: VerifyVerdict::Confirm,
   466	                signature: AgentSignature::from_bytes([0; 64]),
   467	                timestamp_logical: 1,
   468	            }),
   469	            TypedTx::Challenge(ChallengeTx {
   470	                tx_id: TxId("ct".into()),
   471	                target_work_tx: TxId("wt".into()),
   472	                challenger_agent: AgentId("c".into()),
   473	                stake: StakeMicroCoin::from_micro_units(1),
   474	                counterexample_cid: Cid([0; 32]),
   475	                signature: AgentSignature::from_bytes([0; 64]),
   476	                timestamp_logical: 1,
   477	            }),
   478	            TypedTx::Reuse(ReuseTx {
   479	                tx_id: TxId("rt".into()),
   480	                reusing_work_tx: TxId("wt".into()),
   481	                reused_tool_id: ToolId("tool".into()),
   482	                reused_tool_creator: AgentId("a".into()),
   483	                timestamp_logical: 1,
   484	            }),
   485	            TypedTx::FinalizeReward(FinalizeRewardTx {
   486	                tx_id: TxId("ft".into()),
   487	                claim_id: ClaimId::new("cl"),
   488	                task_id: TaskId("t".into()),
   489	                solver: AgentId("s".into()),
   490	                reward: MicroCoin::from_micro_units(1),
   491	                parent_state_root: Default::default(),
   492	                epoch: SystemEpoch::new(1),
   493	                timestamp_logical: 1,
   494	                system_signature: SystemSignature::from_bytes([0; 64]),
   495	            }),
   496	            TypedTx::TaskExpire(TaskExpireTx {
   497	                tx_id: TxId("et".into()),
   498	                task_id: TaskId("t".into()),
   499	                parent_state_root: Default::default(),
   500	                bounty_refunded: MicroCoin::from_micro_units(1),
   501	                epoch: SystemEpoch::new(1),
   502	                timestamp_logical: 1,
   503	                system_signature: SystemSignature::from_bytes([0; 64]),
   504	            }),
   505	            TypedTx::TerminalSummary(TerminalSummaryTx {
   506	                tx_id: TxId("ts".into()),
   507	                task_id: TaskId("t".into()),
   508	                run_id: RunId("r".into()),
   509	                run_outcome: RunOutcome::OmegaAccepted,
   510	                total_attempts: 0,
   511	                failure_class_histogram: BTreeMap::new(),
   512	                last_logical_t: 0,
   513	                system_signature: SystemSignature::from_bytes([0; 64]),
   514	            }),
   515	        ];
   516	
   517	        for tx in cases {
   518	            let result = dispatch_transition(&q, &tx, &preds, &tools);
   519	            assert!(matches!(result, Err(TransitionError::NotYetImplemented)));
   520	        }

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/store.rs | sed -n '1,360p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! CAS store backed by git2-rs blob layer.
     2	//!
     3	//! Each runtime_repo (per spec § 5.2.2 cell isolation) has its own CasStore.
     4	//! Objects are content-addressed by `Cid` (sha256 of content); git's sha-1
     5	//! OID is recorded but not canonical.
     6	//!
     7	//! **CO1.4-extra (this atom)** adds index persistence: the `Cid → metadata`
     8	//! map is durably persisted to a sidecar JSONL file at
     9	//! `<repo_path>/.turingos_cas_index.jsonl`. On `CasStore::open()` the sidecar
    10	//! is replayed into an in-memory BTreeMap; on `CasStore::put()` (new entries
    11	//! only) one JSONL line is appended + flushed. This closes the Art 0.2
    12	//! tape-canonicality cold-replay gate that CO1.7 spec § 0 + CO1.1.4-pre1
    13	//! v1.1 § 0.1 declared a hard prerequisite for `replay_full_transition`
    14	//! (CO1.7-impl A4).
    15	//!
    16	//! **Design choice (sidecar JSONL)**: chosen over (b) git-tag manifest /
    17	//! (c) bincode index + WAL because (a) is the simplest deterministic
    18	//! append-only artifact, replayable from scratch, easy to audit by reading.
    19	//! Per "压缩即智能" — pick simplest correct shape; upgrade later if profiling
    20	//! shows O(N)-on-restart cost is real.
    21	//!
    22	//! /// TRACE_MATRIX WP-arch-§5.L3 + spec-§5.2.2 (cell isolation): CAS store
    23	//! /// TRACE_MATRIX CO1.7 spec § 0 + CO1.1.4-pre1 § 0.1 cross-atom ordering:
    24	//! /// CAS index persistence — required by `replay_full_transition` cold-restart.
    25	
    26	use git2::{ObjectType as Git2ObjectType, Repository};
    27	use sha2::{Digest, Sha256};
    28	use std::collections::BTreeMap;
    29	use std::fs::OpenOptions;
    30	use std::io::{self, Write};
    31	use std::path::{Path, PathBuf};
    32	
    33	use super::schema::{CasObjectMetadata, Cid, ObjectType};
    34	
    35	const CAS_INDEX_FILENAME: &str = ".turingos_cas_index.jsonl";
    36	
    37	#[derive(Debug)]
    38	pub enum CasError {
    39	    /// git2-rs underlying error.
    40	    Git2(git2::Error),
    41	    /// Cid not found in this CasStore's metadata index.
    42	    CidNotFound(Cid),
    43	    /// Content stored at git OID but Cid metadata absent (corrupted index).
    44	    MetadataMissing(Cid),
    45	    /// Content's sha256 doesn't match the asserted Cid (corruption).
    46	    CidMismatch { expected: Cid, computed: Cid },
    47	    /// I/O error reading or writing the CO1.4-extra sidecar index file.
    48	    IoError(io::Error),
    49	    /// JSON-deserialization error on a sidecar index line. Includes 1-based
    50	    /// line number for diagnostics.
    51	    IndexParse { line: usize, error: String },
    52	}
    53	
    54	impl std::fmt::Display for CasError {
    55	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    56	        match self {
    57	            Self::Git2(e) => write!(f, "git2 backend error: {e}"),
    58	            Self::CidNotFound(c) => write!(f, "{c} not found in CAS index"),
    59	            Self::MetadataMissing(c) => write!(f, "{c} metadata missing (index corrupted)"),
    60	            Self::CidMismatch { expected, computed } => write!(
    61	                f,
    62	                "CAS content corruption: expected {expected}, computed {computed}"
    63	            ),
    64	            Self::IoError(e) => write!(f, "cas index I/O error: {e}"),
    65	            Self::IndexParse { line, error } => {
    66	                write!(f, "cas index parse error at line {line}: {error}")
    67	            }
    68	        }
    69	    }
    70	}
    71	
    72	impl std::error::Error for CasError {}
    73	
    74	impl From<git2::Error> for CasError {
    75	    fn from(e: git2::Error) -> Self {
    76	        Self::Git2(e)
    77	    }
    78	}
    79	
    80	impl From<io::Error> for CasError {
    81	    fn from(e: io::Error) -> Self {
    82	        Self::IoError(e)
    83	    }
    84	}
    85	
    86	fn cas_index_path(repo_path: &Path) -> PathBuf {
    87	    repo_path.join(CAS_INDEX_FILENAME)
    88	}
    89	
    90	/// CO1.4-extra: read the sidecar JSONL into an in-memory index.
    91	/// Strict mode — any malformed line aborts the load (per Art 0.2: a
    92	/// corrupted index means the tape is non-canonical; abort + diagnose
    93	/// is more honest than skip-and-warn).
    94	fn load_index_from_sidecar(repo_path: &Path) -> Result<BTreeMap<Cid, CasObjectMetadata>, CasError> {
    95	    let path = cas_index_path(repo_path);
    96	    let mut index = BTreeMap::new();
    97	    if !path.exists() {
    98	        return Ok(index);
    99	    }
   100	    let content = std::fs::read_to_string(&path)?;
   101	    for (i, line) in content.lines().enumerate() {
   102	        if line.is_empty() {
   103	            continue;
   104	        }
   105	        let meta: CasObjectMetadata =
   106	            serde_json::from_str(line).map_err(|e| CasError::IndexParse {
   107	                line: i + 1,
   108	                error: e.to_string(),
   109	            })?;
   110	        index.insert(meta.cid, meta);
   111	    }
   112	    Ok(index)
   113	}
   114	
   115	/// CO1.4-extra: append a single JSONL line for a newly-created CAS object.
   116	/// Followed by `sync_data` for durability (single-writer per cell per spec
   117	/// § 5.2.2 — concurrent-writer atomicity is out of scope).
   118	fn append_to_sidecar(repo_path: &Path, meta: &CasObjectMetadata) -> Result<(), CasError> {
   119	    let path = cas_index_path(repo_path);
   120	    let serialized = serde_json::to_string(meta).map_err(|e| CasError::IndexParse {
   121	        line: 0,
   122	        error: format!("serialize: {e}"),
   123	    })?;
   124	    let mut f = OpenOptions::new()
   125	        .create(true)
   126	        .append(true)
   127	        .open(&path)?;
   128	    f.write_all(serialized.as_bytes())?;
   129	    f.write_all(b"\n")?;
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
   221	        Ok(content)
   222	    }
   223	
   224	    /// Get metadata only (no content fetch).
   225	    pub fn metadata(&self, cid: &Cid) -> Option<&CasObjectMetadata> {
   226	        self.index.get(cid)
   227	    }
   228	
   229	    pub fn len(&self) -> usize {
   230	        self.index.len()
   231	    }
   232	
   233	    pub fn is_empty(&self) -> bool {
   234	        self.index.is_empty()
   235	    }
   236	
   237	    /// Merkle root over all CAS object metadata; deterministic per BTreeMap order.
   238	    pub fn merkle_root(&self) -> [u8; 32] {
   239	        let mut h = Sha256::new();
   240	        for (_cid, meta) in &self.index {
   241	            h.update(meta.canonical_hash());
   242	        }
   243	        h.finalize().into()
   244	    }
   245	}
   246	
   247	#[cfg(test)]
   248	mod tests {
   249	    use super::*;
   250	    use tempfile::TempDir;
   251	
   252	    fn fresh_store() -> (TempDir, CasStore) {
   253	        let tmp = TempDir::new().unwrap();
   254	        let store = CasStore::open(tmp.path()).unwrap();
   255	        (tmp, store)
   256	    }
   257	
   258	    #[test]
   259	    fn put_get_round_trip_small() {
   260	        let (_tmp, mut s) = fresh_store();
   261	        let cid = s.put(b"hello world", ObjectType::ProposalPayload, "alice", 100, None).unwrap();
   262	        let content = s.get(&cid).unwrap();
   263	        assert_eq!(content, b"hello world");
   264	    }
   265	
   266	    #[test]
   267	    fn put_get_round_trip_large() {
   268	        let (_tmp, mut s) = fresh_store();
   269	        let big = vec![0xab; 65536];
   270	        let cid = s.put(&big, ObjectType::PredicateBytecode, "system", 0, Some("wasm".into())).unwrap();
   271	        let content = s.get(&cid).unwrap();
   272	        assert_eq!(content, big);
   273	    }
   274	
   275	    #[test]
   276	    fn put_idempotent_same_content() {
   277	        let (_tmp, mut s) = fresh_store();
   278	        let cid_a = s.put(b"x", ObjectType::Generic, "alice", 1, None).unwrap();
   279	        let cid_b = s.put(b"x", ObjectType::Generic, "bob", 2, None).unwrap();
   280	        assert_eq!(cid_a, cid_b, "same content → same Cid");
   281	        // Index size = 1 (idempotent)
   282	        assert_eq!(s.len(), 1);
   283	    }
   284	
   285	    #[test]
   286	    fn cid_is_content_address() {
   287	        let (_tmp, mut s) = fresh_store();
   288	        let cid = s.put(b"specific content", ObjectType::Generic, "system", 0, None).unwrap();
   289	        // Cid is sha256 of content; verifiable independently
   290	        let expected = Cid::from_content(b"specific content");
   291	        assert_eq!(cid, expected);
   292	    }
   293	
   294	    #[test]
   295	    fn get_nonexistent_returns_error() {
   296	        let (_tmp, s) = fresh_store();
   297	        let bogus = Cid([0u8; 32]);
   298	        match s.get(&bogus) {
   299	            Err(CasError::CidNotFound(c)) => assert_eq!(c, bogus),
   300	            other => panic!("expected CidNotFound, got {other:?}"),
   301	        }
   302	    }
   303	
   304	    #[test]
   305	    fn metadata_recorded() {
   306	        let (_tmp, mut s) = fresh_store();
   307	        let cid = s.put(b"meta test", ObjectType::CounterexamplePayload, "carol", 250, Some("v1".into())).unwrap();
   308	        let meta = s.metadata(&cid).unwrap();
   309	        assert_eq!(meta.cid, cid);
   310	        assert_eq!(meta.object_type, ObjectType::CounterexamplePayload);
   311	        assert_eq!(meta.creator, "carol");
   312	        assert_eq!(meta.created_at_logical_t, 250);
   313	        assert_eq!(meta.schema_id.as_deref(), Some("v1"));
   314	        assert_eq!(meta.size_bytes, 9);
   315	    }
   316	
   317	    #[test]
   318	    fn merkle_root_deterministic_two_runs() {
   319	        let (_tmp1, mut s1) = fresh_store();
   320	        let (_tmp2, mut s2) = fresh_store();
   321	        for content in [b"a".as_slice(), b"b".as_slice(), b"c".as_slice()] {
   322	            s1.put(content, ObjectType::Generic, "system", 0, None).unwrap();
   323	        }
   324	        // Different insertion order
   325	        for content in [b"c".as_slice(), b"b".as_slice(), b"a".as_slice()] {
   326	            s2.put(content, ObjectType::Generic, "system", 0, None).unwrap();
   327	        }
   328	        assert_eq!(s1.merkle_root(), s2.merkle_root(),
   329	            "BTreeMap-ordered: insertion order independent (I-DET)");
   330	    }
   331	
   332	    #[test]
   333	    fn empty_store_root() {
   334	        let (_tmp, s) = fresh_store();
   335	        let r = s.merkle_root();
   336	        let expected: [u8; 32] = Sha256::new().finalize().into();
   337	        assert_eq!(r, expected, "empty store root = sha256(empty)");
   338	    }
   339	
   340	    #[test]
   341	    fn cell_isolation_disjoint_cas() {
   342	        // Per spec § 5.2.2 cross-cell isolation: separate runtime_repo paths
   343	        // → completely disjoint CasStore instances.
   344	        let (_tmp_a, mut store_a) = fresh_store();
   345	        let (_tmp_b, mut store_b) = fresh_store();
   346	
   347	        let cid_a = store_a.put(b"only in a", ObjectType::Generic, "agent_a", 100, None).unwrap();
   348	        let cid_b = store_b.put(b"only in b", ObjectType::Generic, "agent_b", 100, None).unwrap();
   349	
   350	        // Each store has its own object only
   351	        assert!(store_a.get(&cid_a).is_ok(), "store_a has cid_a");
   352	        assert!(store_a.get(&cid_b).is_err(), "store_a lacks cid_b (isolated)");
   353	        assert!(store_b.get(&cid_b).is_ok(), "store_b has cid_b");
   354	        assert!(store_b.get(&cid_a).is_err(), "store_b lacks cid_a (isolated)");
   355	    }
   356	
   357	    #[test]
   358	    fn put_many_then_iterate_count() {
   359	        let (_tmp, mut s) = fresh_store();
   360	        for i in 0..50 {

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '340,1180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   340	/// 8. q_next.state_root_t matches entry.resulting_state_root
   341	/// 9. resulting_ledger_root recomputed via append() matches stored
   342	///
   343	/// **Stub-state caveat (CO1.7.5 unblocks)**: while `dispatch_transition`
   344	/// returns `NotYetImplemented` for every variant, replay errors at stage 7
   345	/// for any non-empty chain. Conformance tests exercising stages 1-6
   346	/// independently are `#[test]`-runnable now; full state_root reconstruction
   347	/// gates on CO1.7.5.
   348	pub fn replay_full_transition(
   349	    genesis_state_root: crate::state::q_state::Hash,
   350	    genesis_ledger_root: crate::state::q_state::Hash,
   351	    entries: &[LedgerEntry],
   352	    cas: &dyn LedgerCasView,
   353	    pinned_pubkeys: &crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys,
   354	    predicate_registry: &crate::top_white::predicates::registry::PredicateRegistry,
   355	    tool_registry: &crate::bottom_white::tools::registry::ToolRegistry,
   356	) -> Result<(crate::state::q_state::Hash, crate::state::q_state::Hash), ReplayError> {
   357	    use crate::bottom_white::ledger::system_keypair::{
   358	        verify_system_signature, CanonicalMessage,
   359	    };
   360	    use crate::state::q_state::QState;
   361	    use crate::state::sequencer::dispatch_transition;
   362	
   363	    let mut prev_state_root = genesis_state_root;
   364	    let mut prev_ledger_root = genesis_ledger_root;
   365	    // For dispatch we need a QState. Replay reconstructs it from genesis;
   366	    // initial state has empty agent swarm + budget defaults. The state_root_t
   367	    // and ledger_root_t are the load-bearing fields entries verify against.
   368	    let mut q = QState::genesis();
   369	    q.state_root_t = genesis_state_root;
   370	    q.ledger_root_t = genesis_ledger_root;
   371	
   372	    for (i, entry) in entries.iter().enumerate() {
   373	        // Stage 1
   374	        let expected_logical_t = (i as u64) + 1;
   375	        if entry.logical_t != expected_logical_t {
   376	            return Err(ReplayError::LogicalTGap {
   377	                at: i,
   378	                expected: expected_logical_t,
   379	                got: entry.logical_t,
   380	            });
   381	        }
   382	        // Stage 2
   383	        if entry.parent_state_root != prev_state_root {
   384	            return Err(ReplayError::ParentStateMismatch { at: i });
   385	        }
   386	        // Stage 3
   387	        if entry.parent_ledger_root != prev_ledger_root {
   388	            return Err(ReplayError::ParentLedgerMismatch { at: i });
   389	        }
   390	
   391	        // Stage 4: system_signature verify (FullTransition mode only).
   392	        let signing_payload = entry.to_signing_payload();
   393	        let signing_digest = signing_payload.canonical_digest();
   394	        let canonical_msg = CanonicalMessage::LedgerEntrySigning(signing_digest.0);
   395	        if !verify_system_signature(
   396	            &entry.system_signature,
   397	            &canonical_msg,
   398	            entry.epoch,
   399	            pinned_pubkeys,
   400	        ) {
   401	            return Err(ReplayError::BadSignature { at: i });
   402	        }
   403	
   404	        // Stage 5: CAS lookup.
   405	        let payload_bytes = cas
   406	            .get_typed_payload(&entry.tx_payload_cid)
   407	            .map_err(|_| ReplayError::CasMissing { at: i })?;
   408	
   409	        // Stage 6: canonical_decode → TypedTx.
   410	        let typed_tx: crate::state::typed_tx::TypedTx = canonical_decode(&payload_bytes)
   411	            .map_err(|_| ReplayError::CasMissing { at: i })?;
   412	
   413	        // Stage 7: re-run pure dispatch_transition.
   414	        let (q_next, _signals) =
   415	            dispatch_transition(&q, &typed_tx, predicate_registry, tool_registry)
   416	                .map_err(|inner| ReplayError::Transition { at: i, inner })?;
   417	
   418	        // Stage 8: state_root match.
   419	        if q_next.state_root_t != entry.resulting_state_root {
   420	            return Err(ReplayError::StateRootMismatch { at: i });
   421	        }
   422	
   423	        // Stage 9: ledger_root match (recompute via append).
   424	        let recomputed_ledger_root = append(&prev_ledger_root, &signing_digest);
   425	        if recomputed_ledger_root != entry.resulting_ledger_root {
   426	            return Err(ReplayError::LedgerRootMismatch { at: i });
   427	        }
   428	
   429	        // Advance.
   430	        q = q_next;
   431	        q.ledger_root_t = entry.resulting_ledger_root;
   432	        prev_state_root = entry.resulting_state_root;
   433	        prev_ledger_root = entry.resulting_ledger_root;
   434	    }
   435	
   436	    Ok((prev_state_root, prev_ledger_root))
   437	}
   438	
   439	/// Skeleton-stage entry point (v1.1).
   440	///
   441	/// Validates:
   442	/// 1. logical_t monotonicity (no gaps, no duplicates)
   443	/// 2. parent_state_root chain
   444	/// 3. parent_ledger_root chain (K2 transplant defense)
   445	/// 4. resulting_ledger_root recomputed via append(prev_ledger_root, signing_digest)
   446	///
   447	/// Does NOT verify:
   448	/// - system_signature (CO1.7.5+: requires CanonicalMessage extension wired through keypair)
   449	/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
   450	///
   451	/// Returns final (state_root, ledger_root) on success.
   452	pub fn replay_chain_integrity(
   453	    genesis_state_root: Hash,
   454	    genesis_ledger_root: Hash,
   455	    entries: &[LedgerEntry],
   456	) -> Result<(Hash, Hash), ReplayError> {
   457	    let mut prev_state_root = genesis_state_root;
   458	    let mut prev_ledger_root = genesis_ledger_root;
   459	
   460	    for (i, entry) in entries.iter().enumerate() {
   461	        let expected_logical_t = (i as u64) + 1;
   462	        if entry.logical_t != expected_logical_t {
   463	            return Err(ReplayError::LogicalTGap {
   464	                at: i,
   465	                expected: expected_logical_t,
   466	                got: entry.logical_t,
   467	            });
   468	        }
   469	        if entry.parent_state_root != prev_state_root {
   470	            return Err(ReplayError::ParentStateMismatch { at: i });
   471	        }
   472	        // K2 NEW: parent_ledger_root chain check
   473	        if entry.parent_ledger_root != prev_ledger_root {
   474	            return Err(ReplayError::ParentLedgerMismatch { at: i });
   475	        }
   476	        let signing_digest = entry.to_signing_payload().canonical_digest();
   477	        let recomputed = append(&prev_ledger_root, &signing_digest);
   478	        if recomputed != entry.resulting_ledger_root {
   479	            return Err(ReplayError::LedgerRootMismatch { at: i });
   480	        }
   481	        prev_state_root = entry.resulting_state_root;
   482	        prev_ledger_root = entry.resulting_ledger_root;
   483	    }
   484	
   485	    Ok((prev_state_root, prev_ledger_root))
   486	}
   487	
   488	// ────────────────────────────────────────────────────────────────────────────
   489	// § 2.5 Canonical serialization (bincode v2; STATE_TRANSITION_SPEC § 2.5)
   490	// ────────────────────────────────────────────────────────────────────────────
   491	
   492	/// `bincode::config` used for the canonical `LedgerEntry` wire format.
   493	///
   494	/// **Frozen choices** (per STATE_TRANSITION_SPEC § 2.5):
   495	/// - **Big-endian** byte order (network order; deterministic across platforms).
   496	/// - **Fixed-int encoding** (no varint; fixed-width for byte-stable round-trip).
   497	/// - **`BTreeMap` keys**: bincode iterates the map in serde-supplied order; we
   498	///   only ever encode `BTreeMap` (sorted by construction) so key order is lex.
   499	/// - **No padding, no implicit alignment.**
   500	fn bincode_canonical_config() -> impl bincode::config::Config {
   501	    bincode::config::standard()
   502	        .with_big_endian()
   503	        .with_fixed_int_encoding()
   504	}
   505	
   506	/// Canonical encode any serde-Serialize value to bytes (CO1.7 wire format).
   507	/// Used by `Git2LedgerWriter` for commit-message bodies and by future callers
   508	/// needing byte-stable signatures over typed payloads.
   509	pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalCodecError> {
   510	    bincode::serde::encode_to_vec(value, bincode_canonical_config())
   511	        .map_err(|e| CanonicalCodecError::Encode(e.to_string()))
   512	}
   513	
   514	/// Canonical decode the inverse of `canonical_encode`. Returns the value plus
   515	/// the number of bytes consumed (entire input must be consumed for a clean decode).
   516	pub fn canonical_decode<T: serde::de::DeserializeOwned>(
   517	    bytes: &[u8],
   518	) -> Result<T, CanonicalCodecError> {
   519	    let (value, consumed) =
   520	        bincode::serde::decode_from_slice::<T, _>(bytes, bincode_canonical_config())
   521	            .map_err(|e| CanonicalCodecError::Decode(e.to_string()))?;
   522	    if consumed != bytes.len() {
   523	        return Err(CanonicalCodecError::TrailingBytes {
   524	            consumed,
   525	            total: bytes.len(),
   526	        });
   527	    }
   528	    Ok(value)
   529	}
   530	
   531	#[derive(Debug)]
   532	pub enum CanonicalCodecError {
   533	    Encode(String),
   534	    Decode(String),
   535	    TrailingBytes { consumed: usize, total: usize },
   536	}
   537	
   538	impl std::fmt::Display for CanonicalCodecError {
   539	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   540	        match self {
   541	            Self::Encode(s) => write!(f, "canonical encode failed: {s}"),
   542	            Self::Decode(s) => write!(f, "canonical decode failed: {s}"),
   543	            Self::TrailingBytes { consumed, total } => {
   544	                write!(f, "trailing bytes after decode: consumed {consumed} of {total}")
   545	            }
   546	        }
   547	    }
   548	}
   549	impl std::error::Error for CanonicalCodecError {}
   550	
   551	// ────────────────────────────────────────────────────────────────────────────
   552	// § 5 Git2LedgerWriter — git2-rs commit chain on `refs/transitions/main`
   553	// ────────────────────────────────────────────────────────────────────────────
   554	
   555	/// Spec § 5 production storage backend.
   556	///
   557	/// **Mapping**:
   558	/// - One `LedgerEntry` = one git commit on `refs/transitions/main`.
   559	/// - **Commit tree** = three named blobs:
   560	///     - `payload_cid`     = entry.tx_payload_cid.0 (32 bytes)
   561	///     - `signature`       = entry.system_signature.as_bytes() (64 bytes)
   562	///     - `entry_canonical` = bincode v2 BE + fixed-int encoding of the full
   563	///       `LedgerEntry` (deterministic, byte-stable; this blob IS the
   564	///       canonical record — `read_at` decodes it directly).
   565	/// - **Commit message** = human-readable `"transition logical_t=<N>\n"` (the
   566	///   canonical record lives in the tree blob, not the message — git
   567	///   normalizes message bytes in ways that break round-trip).
   568	/// - **Parent**: `head_t-1` commit (or none at genesis).
   569	/// - **Author/committer identity**: fixed `("turingosv4 sequencer", "system@turingos")`
   570	///   with `time = (logical_t as i64, 0)` to keep commit OIDs deterministic. NO
   571	///   wall-clock leakage (`I-NOENV` + `I-LOGTIME`).
   572	///
   573	/// **K3 (revised v1.2)**: this writer surfaces `commit_oid` for callers that
   574	/// need it (CO1.7.5+ `head_t` wiring), but the `LedgerWriter::commit` trait
   575	/// returns only `Hash` (entry.resulting_ledger_root). Callers requesting the
   576	/// commit OID use [`Git2LedgerWriter::head_commit_oid`] post-commit.
   577	pub struct Git2LedgerWriter {
   578	    repo_path: PathBuf,
   579	    /// Last commit OID on `refs/transitions/main`; `None` at empty-chain genesis.
   580	    head_oid: Option<git2::Oid>,
   581	    /// Number of entries committed = highest assigned `logical_t` (0 at genesis).
   582	    len: u64,
   583	}
   584	
   585	const TRANSITIONS_REF: &str = "refs/transitions/main";
   586	const TREE_BLOB_PAYLOAD_CID: &str = "payload_cid";
   587	const TREE_BLOB_SIGNATURE: &str = "signature";
   588	const TREE_BLOB_ENTRY_CANONICAL: &str = "entry_canonical";
   589	
   590	impl Git2LedgerWriter {
   591	    /// Open or initialize a `Git2LedgerWriter` rooted at `repo_path`.
   592	    /// Creates the underlying git repo if it doesn't exist; resolves the
   593	    /// existing `refs/transitions/main` if present and seeds `head_oid` + `len`.
   594	    pub fn open(repo_path: &Path) -> Result<Self, LedgerWriterError> {
   595	        let repo_path = repo_path.to_path_buf();
   596	        let repo = match Repository::open(&repo_path) {
   597	            Ok(r) => r,
   598	            Err(_) => Repository::init(&repo_path).map_err(|e| {
   599	                LedgerWriterError::BackendCorruption(format!("repo init: {e}"))
   600	            })?,
   601	        };
   602	
   603	        // Resolve refs/transitions/main if it exists.
   604	        let (head_oid, len) = match repo.find_reference(TRANSITIONS_REF) {
   605	            Ok(reference) => {
   606	                let oid = reference
   607	                    .target()
   608	                    .ok_or_else(|| {
   609	                        LedgerWriterError::BackendCorruption(format!(
   610	                            "{TRANSITIONS_REF} has no direct target"
   611	                        ))
   612	                    })?;
   613	                // Walk parents to count chain length.
   614	                let mut n: u64 = 0;
   615	                let mut cursor = Some(oid);
   616	                while let Some(c) = cursor {
   617	                    n += 1;
   618	                    let commit = repo.find_commit(c).map_err(|e| {
   619	                        LedgerWriterError::BackendCorruption(format!("walk parent: {e}"))
   620	                    })?;
   621	                    cursor = commit.parent(0).ok().map(|p| p.id());
   622	                }
   623	                (Some(oid), n)
   624	            }
   625	            Err(_) => (None, 0),
   626	        };
   627	
   628	        Ok(Self {
   629	            repo_path,
   630	            head_oid,
   631	            len,
   632	        })
   633	    }
   634	
   635	    fn open_repo(&self) -> Result<Repository, LedgerWriterError> {
   636	        Repository::open(&self.repo_path)
   637	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("repo open: {e}")))
   638	    }
   639	
   640	    /// Commit OID of the most recent appended entry (None if chain is empty).
   641	    /// CO1.7.5+ `head_t` wiring uses this to surface commit_sha alongside Hash.
   642	    pub fn head_commit_oid(&self) -> Option<git2::Oid> {
   643	        self.head_oid
   644	    }
   645	
   646	    /// Read raw canonical-encoded `LedgerEntry` bytes (the `entry_canonical`
   647	    /// tree blob) for the entry at `logical_t`. `logical_t` is 1-indexed.
   648	    fn read_canonical_bytes(&self, logical_t: u64) -> Result<Vec<u8>, LedgerWriterError> {
   649	        if logical_t == 0 || logical_t > self.len {
   650	            return Err(LedgerWriterError::NotFound { logical_t });
   651	        }
   652	        let repo = self.open_repo()?;
   653	        // Walk back (len - logical_t) parents from head.
   654	        let mut cursor = self.head_oid.ok_or(LedgerWriterError::NotFound { logical_t })?;
   655	        let mut steps_back = self.len - logical_t;
   656	        while steps_back > 0 {
   657	            let commit = repo.find_commit(cursor).map_err(|e| {
   658	                LedgerWriterError::BackendCorruption(format!("find_commit: {e}"))
   659	            })?;
   660	            cursor = commit
   661	                .parent(0)
   662	                .map_err(|e| LedgerWriterError::BackendCorruption(format!("parent: {e}")))?
   663	                .id();
   664	            steps_back -= 1;
   665	        }
   666	        let commit = repo
   667	            .find_commit(cursor)
   668	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_commit: {e}")))?;
   669	        let tree = commit
   670	            .tree()
   671	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree: {e}")))?;
   672	        let entry_obj = tree
   673	            .get_name(TREE_BLOB_ENTRY_CANONICAL)
   674	            .ok_or_else(|| {
   675	                LedgerWriterError::BackendCorruption(format!(
   676	                    "missing {TREE_BLOB_ENTRY_CANONICAL} blob at logical_t={logical_t}"
   677	                ))
   678	            })?;
   679	        let blob = repo
   680	            .find_blob(entry_obj.id())
   681	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_blob: {e}")))?;
   682	        Ok(blob.content().to_vec())
   683	    }
   684	}
   685	
   686	impl LedgerWriter for Git2LedgerWriter {
   687	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
   688	        let expected = self.len + 1;
   689	        if entry.logical_t != expected {
   690	            return Err(LedgerWriterError::LogicalTGap {
   691	                expected,
   692	                got: entry.logical_t,
   693	            });
   694	        }
   695	
   696	        let repo = self.open_repo()?;
   697	        let canonical = canonical_encode(entry).map_err(|e| {
   698	            LedgerWriterError::BackendCorruption(format!("canonical_encode: {e}"))
   699	        })?;
   700	
   701	        let mut tb = repo
   702	            .treebuilder(None)
   703	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("treebuilder: {e}")))?;
   704	        let cid_blob = repo
   705	            .blob(&entry.tx_payload_cid.0)
   706	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("cid blob: {e}")))?;
   707	        tb.insert(TREE_BLOB_PAYLOAD_CID, cid_blob, 0o100644)
   708	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert cid: {e}")))?;
   709	        let sig_blob = repo
   710	            .blob(entry.system_signature.as_bytes())
   711	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("sig blob: {e}")))?;
   712	        tb.insert(TREE_BLOB_SIGNATURE, sig_blob, 0o100644)
   713	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert sig: {e}")))?;
   714	        let entry_blob = repo
   715	            .blob(&canonical)
   716	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("entry blob: {e}")))?;
   717	        tb.insert(TREE_BLOB_ENTRY_CANONICAL, entry_blob, 0o100644)
   718	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert entry: {e}")))?;
   719	        let tree_oid = tb
   720	            .write()
   721	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree write: {e}")))?;
   722	        let tree = repo
   723	            .find_tree(tree_oid)
   724	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_tree: {e}")))?;
   725	
   726	        // Determinism: time = (logical_t, 0). NO wall clock.
   727	        let time = git2::Time::new(entry.logical_t as i64, 0);
   728	        let author = GitSignature::new("turingosv4 sequencer", "system@turingos", &time)
   729	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("git sig: {e}")))?;
   730	        let committer = author.clone();
   731	
   732	        let parents: Vec<git2::Commit<'_>> = match self.head_oid {
   733	            Some(oid) => vec![repo.find_commit(oid).map_err(|e| {
   734	                LedgerWriterError::BackendCorruption(format!("parent commit: {e}"))
   735	            })?],
   736	            None => Vec::new(),
   737	        };
   738	        let parent_refs: Vec<&git2::Commit<'_>> = parents.iter().collect();
   739	        let message = format!("transition logical_t={}\n", entry.logical_t);
   740	        let new_oid = repo
   741	            .commit(
   742	                Some(TRANSITIONS_REF),
   743	                &author,
   744	                &committer,
   745	                &message,
   746	                &tree,
   747	                &parent_refs,
   748	            )
   749	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("commit: {e}")))?;
   750	
   751	        self.head_oid = Some(new_oid);
   752	        self.len += 1;
   753	        Ok(entry.resulting_ledger_root)
   754	    }
   755	
   756	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
   757	        let bytes = self.read_canonical_bytes(logical_t)?;
   758	        canonical_decode::<LedgerEntry>(&bytes).map_err(|e| {
   759	            LedgerWriterError::BackendCorruption(format!("canonical_decode at {logical_t}: {e}"))
   760	        })
   761	    }
   762	
   763	    fn len(&self) -> u64 {
   764	        self.len
   765	    }
   766	}
   767	
   768	// ────────────────────────────────────────────────────────────────────────────
   769	// Tests — 8 conformance items (4 NEW vs v1 skeleton: K2 / Q9 / repr(u8) / extensions)
   770	// ────────────────────────────────────────────────────────────────────────────
   771	
   772	#[cfg(test)]
   773	mod tests {
   774	    use super::*;
   775	
   776	    fn h(byte: u8) -> Hash {
   777	        Hash([byte; 32])
   778	    }
   779	
   780	    /// Build an entry that satisfies all chain invariants given the previous state.
   781	    fn entry_at(
   782	        logical_t: u64,
   783	        parent_state_root: Hash,
   784	        parent_ledger_root: Hash,
   785	        resulting_state_root: Hash,
   786	    ) -> LedgerEntry {
   787	        let signing = LedgerEntrySigningPayload {
   788	            logical_t,
   789	            parent_state_root,
   790	            parent_ledger_root,
   791	            tx_kind: TxKind::Work,
   792	            tx_payload_cid: Cid([0u8; 32]),
   793	            resulting_state_root,
   794	            timestamp_logical: logical_t,
   795	            epoch: SystemEpoch::new(1),
   796	            extensions: BTreeMap::new(),
   797	        };
   798	        let signing_digest = signing.canonical_digest();
   799	        let resulting_ledger_root = append(&parent_ledger_root, &signing_digest);
   800	        LedgerEntry {
   801	            logical_t: signing.logical_t,
   802	            parent_state_root: signing.parent_state_root,
   803	            parent_ledger_root: signing.parent_ledger_root,
   804	            tx_kind: signing.tx_kind,
   805	            tx_payload_cid: signing.tx_payload_cid,
   806	            resulting_state_root: signing.resulting_state_root,
   807	            resulting_ledger_root,
   808	            timestamp_logical: signing.timestamp_logical,
   809	            epoch: signing.epoch,
   810	            extensions: signing.extensions,
   811	            system_signature: SystemSignature::from_bytes([0u8; 64]),
   812	        }
   813	    }
   814	
   815	    // 1. append byte-stable (I-DET ledger axis)
   816	    #[test]
   817	    fn append_is_pure_and_byte_stable() {
   818	        let a = append(&Hash::ZERO, &h(1));
   819	        let b = append(&Hash::ZERO, &h(1));
   820	        assert_eq!(a, b);
   821	        let c = append(&Hash::ZERO, &h(2));
   822	        assert_ne!(a, c);
   823	    }
   824	
   825	    // 2. canonical_digest stable (#[repr(u8)] discriminant stable)
   826	    #[test]
   827	    fn canonical_digest_stable_across_clones() {
   828	        let p = LedgerEntrySigningPayload {
   829	            logical_t: 1,
   830	            parent_state_root: Hash::ZERO,
   831	            parent_ledger_root: Hash::ZERO,
   832	            tx_kind: TxKind::Work,
   833	            tx_payload_cid: Cid([7u8; 32]),
   834	            resulting_state_root: h(0xaa),
   835	            timestamp_logical: 1,
   836	            epoch: SystemEpoch::new(2),
   837	            extensions: BTreeMap::new(),
   838	        };
   839	        let d1 = p.canonical_digest();
   840	        let d2 = p.clone().canonical_digest();
   841	        assert_eq!(d1, d2);
   842	    }
   843	
   844	    // 3. InMemoryWriter enforces logical_t monotonic
   845	    #[test]
   846	    fn in_memory_writer_enforces_logical_t() {
   847	        let mut w = InMemoryLedgerWriter::new();
   848	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   849	        assert!(w.commit(&e1).is_ok());
   850	
   851	        let e_skip = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   852	        let err = w.commit(&e_skip).unwrap_err();
   853	        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
   854	    }
   855	
   856	    // 4. ChainOnly replay validates clean chain
   857	    #[test]
   858	    fn replay_chain_integrity_clean() {
   859	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   860	        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   861	        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
   862	        let (final_state, final_ledger) =
   863	            replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1.clone(), e2.clone(), e3.clone()])
   864	                .expect("clean chain replays");
   865	        assert_eq!(final_state, e3.resulting_state_root);
   866	        assert_eq!(final_ledger, e3.resulting_ledger_root);
   867	    }
   868	
   869	    // 5. ChainOnly replay rejects parent_state_root tamper
   870	    #[test]
   871	    fn replay_rejects_parent_state_tamper() {
   872	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   873	        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   874	        e2.parent_state_root = h(0xff);
   875	        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
   876	        assert!(matches!(err, ReplayError::ParentStateMismatch { at: 1 }));
   877	    }
   878	
   879	    // 6. K2 NEW: ChainOnly replay rejects parent_ledger_root tamper (transplant defense)
   880	    #[test]
   881	    fn replay_rejects_parent_ledger_tamper() {
   882	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   883	        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   884	        // Tamper with parent_ledger_root WITHOUT recomputing resulting_ledger_root —
   885	        // simulates an attacker transplanting an entry from a different ledger history.
   886	        e2.parent_ledger_root = h(0xff);
   887	        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
   888	        assert!(matches!(err, ReplayError::ParentLedgerMismatch { at: 1 }));
   889	    }
   890	
   891	    // 7. ChainOnly replay rejects ledger_root tamper
   892	    #[test]
   893	    fn replay_rejects_ledger_root_tamper() {
   894	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   895	        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   896	        e2.resulting_ledger_root = h(0xee);
   897	        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
   898	        assert!(matches!(err, ReplayError::LedgerRootMismatch { at: 1 }));
   899	    }
   900	
   901	    // 8. Q9 NEW: canonical_digest excludes derivatives
   902	    // Mutating `resulting_ledger_root` or `system_signature` of LedgerEntry must NOT
   903	    // change the signing payload digest (because they're not in LedgerEntrySigningPayload).
   904	    #[test]
   905	    fn canonical_digest_excludes_derivatives() {
   906	        let e_clean = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   907	        let digest_clean = e_clean.to_signing_payload().canonical_digest();
   908	
   909	        // Mutate resulting_ledger_root (a derivative; should NOT affect signing digest)
   910	        let mut e_tamper = e_clean.clone();
   911	        e_tamper.resulting_ledger_root = h(0xff);
   912	        let digest_after_root_tamper = e_tamper.to_signing_payload().canonical_digest();
   913	        assert_eq!(
   914	            digest_clean, digest_after_root_tamper,
   915	            "resulting_ledger_root MUST NOT affect signing digest (Q9 cycle prevention)"
   916	        );
   917	
   918	        // Mutate system_signature (signature is its own input; should NOT affect signing digest)
   919	        let mut e_tamper2 = e_clean.clone();
   920	        e_tamper2.system_signature = SystemSignature::from_bytes([0xffu8; 64]);
   921	        let digest_after_sig_tamper = e_tamper2.to_signing_payload().canonical_digest();
   922	        assert_eq!(digest_clean, digest_after_sig_tamper);
   923	
   924	        // Sanity: mutating a SIGNED field DOES change digest
   925	        let mut e_signed_change = e_clean.clone();
   926	        e_signed_change.epoch = SystemEpoch::new(99);
   927	        let digest_after_signed = e_signed_change.to_signing_payload().canonical_digest();
   928	        assert_ne!(digest_clean, digest_after_signed);
   929	    }
   930	
   931	    // 9. C3 closure (round-2): real signature roundtrip via system_keypair extension.
   932	    // Verifies: (a) typed sign API works; (b) signature verifies via CanonicalMessage::LedgerEntrySigning;
   933	    // (c) signature does NOT verify after mutating a signed field (parent_ledger_root — K2 transplant defense).
   934	    #[test]
   935	    fn signature_round_trip_and_transplant_defense() {
   936	        use crate::bottom_white::ledger::system_keypair::{
   937	            transition_ledger_emitter, CanonicalMessage, Ed25519Keypair, PinnedSystemPubkeys,
   938	            SystemEpoch, verify_system_signature,
   939	        };
   940	
   941	        let keypair = Ed25519Keypair::generate_with_secure_entropy().expect("keypair gen");
   942	        let epoch = SystemEpoch::new(1);
   943	        let mut pinned = PinnedSystemPubkeys::new();
   944	        pinned.insert(epoch, keypair.public_key());
   945	
   946	        // Build a clean signing payload (e1's worth)
   947	        let payload = LedgerEntrySigningPayload {
   948	            logical_t: 1,
   949	            parent_state_root: Hash::ZERO,
   950	            parent_ledger_root: Hash::ZERO,
   951	            tx_kind: TxKind::Work,
   952	            tx_payload_cid: Cid([42u8; 32]),
   953	            resulting_state_root: h(1),
   954	            timestamp_logical: 1,
   955	            epoch,
   956	            extensions: BTreeMap::new(),
   957	        };
   958	        let digest = payload.canonical_digest();
   959	
   960	        // Real sign through the typed CanonicalMessage extension
   961	        let sig = transition_ledger_emitter::sign_ledger_entry(&keypair, digest.0)
   962	            .expect("sign_ledger_entry");
   963	
   964	        // Verify (clean) — must succeed
   965	        let msg_clean = CanonicalMessage::LedgerEntrySigning(digest.0);
   966	        assert!(
   967	            verify_system_signature(&sig, &msg_clean, epoch, &pinned),
   968	            "clean signature must verify"
   969	        );
   970	
   971	        // Verify (tamper parent_ledger_root) — K2 transplant defense
   972	        let mut payload_tamper = payload.clone();
   973	        payload_tamper.parent_ledger_root = h(0xff);
   974	        let digest_tamper = payload_tamper.canonical_digest();
   975	        let msg_tamper = CanonicalMessage::LedgerEntrySigning(digest_tamper.0);
   976	        assert!(
   977	            !verify_system_signature(&sig, &msg_tamper, epoch, &pinned),
   978	            "transplanted parent_ledger_root MUST fail signature verify (K2)"
   979	        );
   980	
   981	        // Verify (cross-epoch transplant) — D1 defense via epoch IN payload digest.
   982	        // Attacker scenario: sig was made for payload with epoch=1; attacker forges a
   983	        // NEW payload claiming epoch=2 reusing the old sig. Since epoch is in the
   984	        // canonical digest, digest_v2 ≠ digest_v1, so the sig on digest_v1 cannot
   985	        // verify against digest_v2.
   986	        let mut payload_other_epoch = payload.clone();
   987	        payload_other_epoch.epoch = SystemEpoch::new(2);
   988	        let digest_other_epoch = payload_other_epoch.canonical_digest();
   989	        assert_ne!(digest, digest_other_epoch, "epoch is bound in canonical digest");
   990	        let msg_other_epoch = CanonicalMessage::LedgerEntrySigning(digest_other_epoch.0);
   991	        assert!(
   992	            !verify_system_signature(&sig, &msg_other_epoch, epoch, &pinned),
   993	            "cross-epoch transplant MUST fail signature verify (D1 epoch binding)"
   994	        );
   995	    }
   996	
   997	    // ──────────────────────────────────────────────────────────────────────
   998	    // 10–13. Git2LedgerWriter — git2-rs commit chain backend (§ 5)
   999	    // ──────────────────────────────────────────────────────────────────────
  1000	
  1001	    use tempfile::TempDir;
  1002	
  1003	    fn fresh_git_writer() -> (TempDir, Git2LedgerWriter) {
  1004	        let tmp = TempDir::new().expect("tempdir");
  1005	        let w = Git2LedgerWriter::open(tmp.path()).expect("open");
  1006	        (tmp, w)
  1007	    }
  1008	
  1009	    // 10. Empty repo: len()=0, head_commit_oid=None.
  1010	    #[test]
  1011	    fn git2_writer_empty_chain() {
  1012	        let (_tmp, w) = fresh_git_writer();
  1013	        assert_eq!(w.len(), 0);
  1014	        assert!(w.head_commit_oid().is_none());
  1015	    }
  1016	
  1017	    // 11. Append three entries; len + head_commit_oid advance per commit;
  1018	    //     read_at recovers each entry byte-identically (canonical encode/decode round-trip).
  1019	    #[test]
  1020	    fn git2_writer_append_and_read_back() {
  1021	        let (_tmp, mut w) = fresh_git_writer();
  1022	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
  1023	        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
  1024	        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
  1025	
  1026	        let r1 = w.commit(&e1).expect("commit 1");
  1027	        assert_eq!(r1, e1.resulting_ledger_root);
  1028	        assert_eq!(w.len(), 1);
  1029	        let oid_1 = w.head_commit_oid().expect("head after 1");
  1030	
  1031	        let r2 = w.commit(&e2).expect("commit 2");
  1032	        assert_eq!(r2, e2.resulting_ledger_root);
  1033	        assert_eq!(w.len(), 2);
  1034	        let oid_2 = w.head_commit_oid().expect("head after 2");
  1035	        assert_ne!(oid_1, oid_2, "head must advance after second commit");
  1036	
  1037	        w.commit(&e3).expect("commit 3");
  1038	        assert_eq!(w.len(), 3);
  1039	
  1040	        // read_at returns each entry byte-identically.
  1041	        assert_eq!(w.read_at(1).expect("read 1"), e1);
  1042	        assert_eq!(w.read_at(2).expect("read 2"), e2);
  1043	        assert_eq!(w.read_at(3).expect("read 3"), e3);
  1044	    }
  1045	
  1046	    // 12. Skipping a logical_t triggers LogicalTGap; chain state is unchanged.
  1047	    #[test]
  1048	    fn git2_writer_rejects_logical_t_gap() {
  1049	        let (_tmp, mut w) = fresh_git_writer();
  1050	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
  1051	        w.commit(&e1).expect("commit 1");
  1052	        let pre_oid = w.head_commit_oid();
  1053	
  1054	        // Try to commit a logical_t=3 entry (gap: expected 2)
  1055	        let e_skip = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
  1056	        let err = w.commit(&e_skip).unwrap_err();
  1057	        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
  1058	        // Chain unchanged.
  1059	        assert_eq!(w.len(), 1);
  1060	        assert_eq!(w.head_commit_oid(), pre_oid);
  1061	    }
  1062	
  1063	    // 13. Reopening the same repo path resurrects the chain (head + len recovered
  1064	    //     from refs/transitions/main). Crucial for runtime cold-restart.
  1065	    #[test]
  1066	    fn git2_writer_reopen_recovers_chain() {
  1067	        let tmp = TempDir::new().expect("tempdir");
  1068	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
  1069	        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
  1070	        let oid_after_two;
  1071	        {
  1072	            let mut w = Git2LedgerWriter::open(tmp.path()).expect("open");
  1073	            w.commit(&e1).expect("commit 1");
  1074	            w.commit(&e2).expect("commit 2");
  1075	            oid_after_two = w.head_commit_oid().expect("head");
  1076	        }
  1077	        // Reopen — fresh struct, same on-disk repo.
  1078	        let w2 = Git2LedgerWriter::open(tmp.path()).expect("reopen");
  1079	        assert_eq!(w2.len(), 2);
  1080	        assert_eq!(w2.head_commit_oid(), Some(oid_after_two));
  1081	        assert_eq!(w2.read_at(1).expect("read 1"), e1);
  1082	        assert_eq!(w2.read_at(2).expect("read 2"), e2);
  1083	
  1084	        // Continue chain after reopen.
  1085	        let mut w3 = Git2LedgerWriter::open(tmp.path()).expect("reopen 2");
  1086	        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
  1087	        w3.commit(&e3).expect("commit 3");
  1088	        assert_eq!(w3.len(), 3);
  1089	    }
  1090	
  1091	    // ──────────────────────────────────────────────────────────────────────
  1092	    // 15-18. CO1.7-impl A4 — replay_full_transition (THE I-DETHASH witness)
  1093	    // ──────────────────────────────────────────────────────────────────────
  1094	
  1095	    use crate::bottom_white::cas::schema::ObjectType;
  1096	    use crate::bottom_white::cas::store::CasStore;
  1097	    use crate::bottom_white::ledger::system_keypair::{
  1098	        transition_ledger_emitter, Ed25519Keypair, PinnedSystemPubkeys,
  1099	    };
  1100	    use crate::bottom_white::tools::registry::ToolRegistry;
  1101	    use crate::state::typed_tx::{
  1102	        AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey,
  1103	        SafetyOrCreation, TaskId, TypedTx, WorkTx, WriteKey,
  1104	    };
  1105	    use crate::state::q_state::{AgentId, TxId as QTxId};
  1106	    use crate::top_white::predicates::registry::PredicateRegistry;
  1107	
  1108	    fn dummy_typed_tx() -> TypedTx {
  1109	        let mut acceptance = std::collections::BTreeMap::new();
  1110	        acceptance.insert(
  1111	            PredicateId("acc1".into()),
  1112	            BoolWithProof { value: true, proof_cid: None },
  1113	        );
  1114	        TypedTx::Work(WorkTx {
  1115	            tx_id: QTxId("worktx-replay-fixture".into()),
  1116	            task_id: TaskId("task-replay".into()),
  1117	            parent_state_root: Hash::ZERO,
  1118	            agent_id: AgentId("alice".into()),
  1119	            read_set: [ReadKey("k.r".into())].into_iter().collect::<std::collections::BTreeSet<_>>(),
  1120	            write_set: [WriteKey("k.w".into())].into_iter().collect::<std::collections::BTreeSet<_>>(),
  1121	            proposal_cid: Cid([0; 32]),
  1122	            predicate_results: PredicateResultsBundle {
  1123	                acceptance,
  1124	                settlement: std::collections::BTreeMap::new(),
  1125	                safety_class: SafetyOrCreation::Safety,
  1126	            },
  1127	            stake: crate::economy::money::StakeMicroCoin::from_micro_units(1),
  1128	            signature: AgentSignature::from_bytes([0u8; 64]),
  1129	            timestamp_logical: 1,
  1130	        })
  1131	    }
  1132	
  1133	    /// Build a real signed LedgerEntry against the given keypair + epoch,
  1134	    /// with the typed_tx's canonical bytes stored in CAS. Mirrors
  1135	    /// `Sequencer::apply_one` stages 5-9 outside the runtime.
  1136	    fn build_signed_entry(
  1137	        logical_t: u64,
  1138	        parent_state_root: Hash,
  1139	        parent_ledger_root: Hash,
  1140	        resulting_state_root: Hash,
  1141	        epoch: SystemEpoch,
  1142	        keypair: &Ed25519Keypair,
  1143	        cas: &mut CasStore,
  1144	        typed_tx: &TypedTx,
  1145	    ) -> LedgerEntry {
  1146	        let bytes = canonical_encode(typed_tx).expect("encode");
  1147	        let cid = cas
  1148	            .put(&bytes, ObjectType::ProposalPayload, "test", logical_t, None)
  1149	            .expect("cas put");
  1150	        let signing = LedgerEntrySigningPayload {
  1151	            logical_t,
  1152	            parent_state_root,
  1153	            parent_ledger_root,
  1154	            tx_kind: typed_tx.tx_kind(),
  1155	            tx_payload_cid: cid,
  1156	            resulting_state_root,
  1157	            timestamp_logical: logical_t,
  1158	            epoch,
  1159	            extensions: BTreeMap::new(),
  1160	        };
  1161	        let digest = signing.canonical_digest();
  1162	        let sig = transition_ledger_emitter::sign_ledger_entry(keypair, digest.0)
  1163	            .expect("sign");
  1164	        let resulting_ledger_root = append(&parent_ledger_root, &digest);
  1165	        LedgerEntry {
  1166	            logical_t,
  1167	            parent_state_root,
  1168	            parent_ledger_root,
  1169	            tx_kind: typed_tx.tx_kind(),
  1170	            tx_payload_cid: cid,
  1171	            resulting_state_root,
  1172	            resulting_ledger_root,
  1173	            timestamp_logical: logical_t,
  1174	            epoch,
  1175	            extensions: BTreeMap::new(),
  1176	            system_signature: sig,
  1177	        }
  1178	    }
  1179	
  1180	    fn replay_test_setup() -> (

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '520,760p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   520	        }
   521	    }
   522	
   523	    // 2. K1 dual counter: submit advances submit_id but NOT logical_t.
   524	    #[tokio::test]
   525	    async fn submit_advances_submit_id_only() {
   526	        let (_tmp, seq, _rx) = fresh_sequencer();
   527	        assert_eq!(seq.next_submit_id_peek(), 1);
   528	        assert_eq!(seq.next_logical_t_peek(), 0);
   529	
   530	        let r1 = seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("submit 1");
   531	        assert_eq!(r1.submit_id, 1);
   532	        assert_eq!(seq.next_submit_id_peek(), 2);
   533	        assert_eq!(seq.next_logical_t_peek(), 0, "logical_t MUST NOT advance at submit");
   534	
   535	        let r2 = seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("submit 2");
   536	        assert_eq!(r2.submit_id, 2);
   537	        assert_eq!(seq.next_logical_t_peek(), 0);
   538	    }
   539	
   540	    // 3. apply_one in stub mode: returns Transition(NotYetImplemented); no
   541	    //    logical_t consumed (K1 invariant: rejected submission never advances commit counter).
   542	    #[test]
   543	    fn apply_one_stub_does_not_consume_logical_t() {
   544	        let (_tmp, seq, _rx) = fresh_sequencer();
   545	        let pre = seq.next_logical_t_peek();
   546	        let err = seq.apply_one(TypedTx::Work(fixture_work_tx())).unwrap_err();
   547	        assert!(matches!(err, ApplyError::Transition(TransitionError::NotYetImplemented)));
   548	        let post = seq.next_logical_t_peek();
   549	        assert_eq!(pre, post, "logical_t MUST NOT advance on rejected apply_one");
   550	    }
   551	
   552	    // 4. Queue saturation: submit returns QueueFull (Q1/Q2 resolution).
   553	    #[tokio::test]
   554	    async fn submit_returns_queue_full_on_saturation() {
   555	        // Capacity=2; receiver never drained.
   556	        let tmp = TempDir::new().expect("tempdir");
   557	        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
   558	        let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("kp"));
   559	        let writer: Arc<RwLock<dyn LedgerWriter>> =
   560	            Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
   561	        let preds = Arc::new(PredicateRegistry::new());
   562	        let tools = Arc::new(ToolRegistry::new());
   563	        let (seq, _rx) = Sequencer::new(
   564	            cas,
   565	            keypair,
   566	            SystemEpoch::new(1),
   567	            writer,
   568	            preds,
   569	            tools,
   570	            QState::genesis(),
   571	            2,
   572	        );
   573	        // Fill capacity.
   574	        seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("1");
   575	        seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("2");
   576	        // Saturated.
   577	        let err = seq.submit(TypedTx::Work(fixture_work_tx())).await.unwrap_err();
   578	        assert!(matches!(err, SubmitError::QueueFull));
   579	    }
   580	
   581	    // 5. submit returns QueueClosed when receiver dropped.
   582	    #[tokio::test]
   583	    async fn submit_returns_queue_closed_after_rx_drop() {
   584	        let (_tmp, seq, rx) = fresh_sequencer();
   585	        drop(rx);
   586	        let err = seq.submit(TypedTx::Work(fixture_work_tx())).await.unwrap_err();
   587	        assert!(matches!(err, SubmitError::QueueClosed));
   588	    }
   589	}

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/store.rs | sed -n '360,520p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   360	        for i in 0..50 {
   361	            s.put(
   362	                format!("content {i}").as_bytes(),
   363	                ObjectType::ProposalPayload,
   364	                "system",
   365	                i as u64,
   366	                None,
   367	            )
   368	            .unwrap();
   369	        }
   370	        assert_eq!(s.len(), 50);
   371	        assert!(!s.is_empty());
   372	    }
   373	
   374	    // ── CO1.4-extra: sidecar JSONL persistence tests ─────────────────────────
   375	
   376	    /// Cold-restart: reopen recovers all metadata; get() works post-reopen
   377	    /// (closes the Art 0.2 cold-replay gate that CO1.7-impl A4 needs).
   378	    #[test]
   379	    fn reopen_recovers_index_and_get_works() {
   380	        let tmp = TempDir::new().expect("tempdir");
   381	        let cid_a;
   382	        let cid_b;
   383	        {
   384	            let mut s = CasStore::open(tmp.path()).expect("open");
   385	            cid_a = s
   386	                .put(b"alpha", ObjectType::ProposalPayload, "alice", 1, None)
   387	                .unwrap();
   388	            cid_b = s
   389	                .put(b"beta", ObjectType::CounterexamplePayload, "bob", 2, Some("s.v1".into()))
   390	                .unwrap();
   391	        }
   392	        // Reopen: in-memory store is fresh; sidecar replay is the ONLY way
   393	        // metadata survives.
   394	        let s2 = CasStore::open(tmp.path()).expect("reopen");
   395	        assert_eq!(s2.len(), 2);
   396	        assert_eq!(s2.get(&cid_a).expect("get a"), b"alpha");
   397	        assert_eq!(s2.get(&cid_b).expect("get b"), b"beta");
   398	
   399	        let meta_b = s2.metadata(&cid_b).expect("metadata b");
   400	        assert_eq!(meta_b.creator, "bob");
   401	        assert_eq!(meta_b.created_at_logical_t, 2);
   402	        assert_eq!(meta_b.schema_id.as_deref(), Some("s.v1"));
   403	        assert_eq!(meta_b.object_type, ObjectType::CounterexamplePayload);
   404	    }
   405	
   406	    /// Idempotent put: same content twice → same Cid → only ONE sidecar line.
   407	    #[test]
   408	    fn idempotent_put_does_not_duplicate_sidecar_line() {
   409	        let tmp = TempDir::new().expect("tempdir");
   410	        let mut s = CasStore::open(tmp.path()).expect("open");
   411	        let _ = s
   412	            .put(b"content", ObjectType::Generic, "alice", 1, None)
   413	            .unwrap();
   414	        let _ = s
   415	            .put(b"content", ObjectType::Generic, "alice", 1, None)
   416	            .unwrap();
   417	        let path = cas_index_path(tmp.path());
   418	        let lines: Vec<&str> = std::fs::read_to_string(&path)
   419	            .unwrap()
   420	            .lines()
   421	            .filter(|l| !l.is_empty())
   422	            .map(|l| {
   423	                // own the str via leak — cheap for test
   424	                Box::leak(l.to_string().into_boxed_str()) as &str
   425	            })
   426	            .collect();
   427	        assert_eq!(lines.len(), 1, "idempotent put should produce 1 sidecar line, got {}", lines.len());
   428	    }
   429	
   430	    /// Append-only: each NEW put adds exactly ONE line.
   431	    #[test]
   432	    fn each_new_put_appends_one_line() {
   433	        let tmp = TempDir::new().expect("tempdir");
   434	        let mut s = CasStore::open(tmp.path()).expect("open");
   435	        for i in 0..5 {
   436	            s.put(
   437	                format!("c{i}").as_bytes(),
   438	                ObjectType::Generic,
   439	                "system",
   440	                i,
   441	                None,
   442	            )
   443	            .unwrap();
   444	        }
   445	        let path = cas_index_path(tmp.path());
   446	        let line_count = std::fs::read_to_string(&path)
   447	            .unwrap()
   448	            .lines()
   449	            .filter(|l| !l.is_empty())
   450	            .count();
   451	        assert_eq!(line_count, 5);
   452	    }
   453	
   454	    /// Corrupted JSONL → strict parse error with line number (not silent skip).
   455	    #[test]
   456	    fn corrupted_sidecar_line_returns_parse_error() {
   457	        let tmp = TempDir::new().expect("tempdir");
   458	        // Init repo + ONE valid put to get a known-good first line.
   459	        {
   460	            let mut s = CasStore::open(tmp.path()).expect("open");
   461	            s.put(b"hello", ObjectType::Generic, "alice", 1, None).unwrap();
   462	        }
   463	        // Corrupt: append a malformed line.
   464	        let path = cas_index_path(tmp.path());
   465	        let mut f = OpenOptions::new().append(true).open(&path).unwrap();
   466	        f.write_all(b"this is not valid json\n").unwrap();
   467	        f.sync_data().unwrap();
   468	
   469	        // Reopen MUST fail with a typed IndexParse error citing the line number.
   470	        let err = CasStore::open(tmp.path()).unwrap_err();
   471	        match err {
   472	            CasError::IndexParse { line, .. } => {
   473	                assert_eq!(line, 2, "expected line 2 to be flagged");
   474	            }
   475	            other => panic!("expected IndexParse, got {other:?}"),
   476	        }
   477	    }
   478	
   479	    /// Empty / non-existent sidecar → opens fresh with empty index.
   480	    #[test]
   481	    fn missing_sidecar_opens_fresh() {
   482	        let tmp = TempDir::new().expect("tempdir");
   483	        let s = CasStore::open(tmp.path()).expect("open");
   484	        assert_eq!(s.len(), 0);
   485	        assert!(s.is_empty());
   486	    }
   487	}

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '686,790p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   686	impl LedgerWriter for Git2LedgerWriter {
   687	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
   688	        let expected = self.len + 1;
   689	        if entry.logical_t != expected {
   690	            return Err(LedgerWriterError::LogicalTGap {
   691	                expected,
   692	                got: entry.logical_t,
   693	            });
   694	        }
   695	
   696	        let repo = self.open_repo()?;
   697	        let canonical = canonical_encode(entry).map_err(|e| {
   698	            LedgerWriterError::BackendCorruption(format!("canonical_encode: {e}"))
   699	        })?;
   700	
   701	        let mut tb = repo
   702	            .treebuilder(None)
   703	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("treebuilder: {e}")))?;
   704	        let cid_blob = repo
   705	            .blob(&entry.tx_payload_cid.0)
   706	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("cid blob: {e}")))?;
   707	        tb.insert(TREE_BLOB_PAYLOAD_CID, cid_blob, 0o100644)
   708	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert cid: {e}")))?;
   709	        let sig_blob = repo
   710	            .blob(entry.system_signature.as_bytes())
   711	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("sig blob: {e}")))?;
   712	        tb.insert(TREE_BLOB_SIGNATURE, sig_blob, 0o100644)
   713	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert sig: {e}")))?;
   714	        let entry_blob = repo
   715	            .blob(&canonical)
   716	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("entry blob: {e}")))?;
   717	        tb.insert(TREE_BLOB_ENTRY_CANONICAL, entry_blob, 0o100644)
   718	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert entry: {e}")))?;
   719	        let tree_oid = tb
   720	            .write()
   721	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree write: {e}")))?;
   722	        let tree = repo
   723	            .find_tree(tree_oid)
   724	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_tree: {e}")))?;
   725	
   726	        // Determinism: time = (logical_t, 0). NO wall clock.
   727	        let time = git2::Time::new(entry.logical_t as i64, 0);
   728	        let author = GitSignature::new("turingosv4 sequencer", "system@turingos", &time)
   729	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("git sig: {e}")))?;
   730	        let committer = author.clone();
   731	
   732	        let parents: Vec<git2::Commit<'_>> = match self.head_oid {
   733	            Some(oid) => vec![repo.find_commit(oid).map_err(|e| {
   734	                LedgerWriterError::BackendCorruption(format!("parent commit: {e}"))
   735	            })?],
   736	            None => Vec::new(),
   737	        };
   738	        let parent_refs: Vec<&git2::Commit<'_>> = parents.iter().collect();
   739	        let message = format!("transition logical_t={}\n", entry.logical_t);
   740	        let new_oid = repo
   741	            .commit(
   742	                Some(TRANSITIONS_REF),
   743	                &author,
   744	                &committer,
   745	                &message,
   746	                &tree,
   747	                &parent_refs,
   748	            )
   749	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("commit: {e}")))?;
   750	
   751	        self.head_oid = Some(new_oid);
   752	        self.len += 1;
   753	        Ok(entry.resulting_ledger_root)
   754	    }
   755	
   756	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
   757	        let bytes = self.read_canonical_bytes(logical_t)?;
   758	        canonical_decode::<LedgerEntry>(&bytes).map_err(|e| {
   759	            LedgerWriterError::BackendCorruption(format!("canonical_decode at {logical_t}: {e}"))
   760	        })
   761	    }
   762	
   763	    fn len(&self) -> u64 {
   764	        self.len
   765	    }
   766	}
   767	
   768	// ────────────────────────────────────────────────────────────────────────────
   769	// Tests — 8 conformance items (4 NEW vs v1 skeleton: K2 / Q9 / repr(u8) / extensions)
   770	// ────────────────────────────────────────────────────────────────────────────
   771	
   772	#[cfg(test)]
   773	mod tests {
   774	    use super::*;
   775	
   776	    fn h(byte: u8) -> Hash {
   777	        Hash([byte; 32])
   778	    }
   779	
   780	    /// Build an entry that satisfies all chain invariants given the previous state.
   781	    fn entry_at(
   782	        logical_t: u64,
   783	        parent_state_root: Hash,
   784	        parent_ledger_root: Hash,
   785	        resulting_state_root: Hash,
   786	    ) -> LedgerEntry {
   787	        let signing = LedgerEntrySigningPayload {
   788	            logical_t,
   789	            parent_state_root,
   790	            parent_ledger_root,

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '1180,1305p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1180	    fn replay_test_setup() -> (
  1181	        TempDir,
  1182	        CasStore,
  1183	        Ed25519Keypair,
  1184	        SystemEpoch,
  1185	        PinnedSystemPubkeys,
  1186	        PredicateRegistry,
  1187	        ToolRegistry,
  1188	    ) {
  1189	        let tmp = TempDir::new().expect("tempdir");
  1190	        let cas = CasStore::open(tmp.path()).expect("cas");
  1191	        let kp = Ed25519Keypair::generate_with_secure_entropy().expect("kp");
  1192	        let epoch = SystemEpoch::new(1);
  1193	        let mut pinned = PinnedSystemPubkeys::new();
  1194	        pinned.insert(epoch, kp.public_key());
  1195	        let preds = PredicateRegistry::new();
  1196	        let tools = ToolRegistry::new();
  1197	        (tmp, cas, kp, epoch, pinned, preds, tools)
  1198	    }
  1199	
  1200	    /// 15. CO1.7.5-stage: in stub mode, dispatch errors with NotYetImplemented;
  1201	    ///     replay correctly bubbles up `Transition { at: 0, inner: NotYetImplemented }`.
  1202	    ///     This proves stages 1-6 (chain + sig + CAS + decode) all PASS,
  1203	    ///     leaving stage 7 (dispatch) as the only gate. CO1.7.5 fills it.
  1204	    #[test]
  1205	    fn replay_full_transition_reaches_dispatch_then_stubs() {
  1206	        let (_tmp, mut cas, kp, epoch, pinned, preds, tools) = replay_test_setup();
  1207	        let entry = build_signed_entry(
  1208	            1,
  1209	            Hash::ZERO,
  1210	            Hash::ZERO,
  1211	            h(1), // resulting state_root (won't be reached due to dispatch stub)
  1212	            epoch,
  1213	            &kp,
  1214	            &mut cas,
  1215	            &dummy_typed_tx(),
  1216	        );
  1217	        let err = replay_full_transition(
  1218	            Hash::ZERO,
  1219	            Hash::ZERO,
  1220	            &[entry],
  1221	            &cas,
  1222	            &pinned,
  1223	            &preds,
  1224	            &tools,
  1225	        )
  1226	        .unwrap_err();
  1227	        assert!(
  1228	            matches!(err, ReplayError::Transition { at: 0, inner: crate::state::typed_tx::TransitionError::NotYetImplemented }),
  1229	            "expected Transition(NotYetImplemented at 0); got {err:?}"
  1230	        );
  1231	    }
  1232	
  1233	    /// 16. system_signature_verifies_via_canonical_message — tampering the
  1234	    ///     signature MUST fire BadSignature BEFORE dispatch is reached.
  1235	    #[test]
  1236	    fn replay_rejects_bad_system_signature() {
  1237	        let (_tmp, mut cas, kp, epoch, pinned, preds, tools) = replay_test_setup();
  1238	        let mut entry = build_signed_entry(
  1239	            1,
  1240	            Hash::ZERO,
  1241	            Hash::ZERO,
  1242	            h(1),
  1243	            epoch,
  1244	            &kp,
  1245	            &mut cas,
  1246	            &dummy_typed_tx(),
  1247	        );
  1248	        // Tamper signature.
  1249	        entry.system_signature = SystemSignature::from_bytes([0xff; 64]);
  1250	        let err = replay_full_transition(
  1251	            Hash::ZERO,
  1252	            Hash::ZERO,
  1253	            &[entry],
  1254	            &cas,
  1255	            &pinned,
  1256	            &preds,
  1257	            &tools,
  1258	        )
  1259	        .unwrap_err();
  1260	        assert!(matches!(err, ReplayError::BadSignature { at: 0 }));
  1261	    }
  1262	
  1263	    /// 17. cas_payload_round_trip — replay correctly fetches CAS bytes;
  1264	    ///     CO1.4-extra cold-restart capability test.
  1265	    #[test]
  1266	    fn replay_cas_payload_round_trip_after_reopen() {
  1267	        let tmp = TempDir::new().expect("tempdir");
  1268	        let kp = Ed25519Keypair::generate_with_secure_entropy().expect("kp");
  1269	        let epoch = SystemEpoch::new(1);
  1270	        let mut pinned = PinnedSystemPubkeys::new();
  1271	        pinned.insert(epoch, kp.public_key());
  1272	        let preds = PredicateRegistry::new();
  1273	        let tools = ToolRegistry::new();
  1274	
  1275	        let entry;
  1276	        {
  1277	            let mut cas = CasStore::open(tmp.path()).expect("cas");
  1278	            entry = build_signed_entry(
  1279	                1,
  1280	                Hash::ZERO,
  1281	                Hash::ZERO,
  1282	                h(1),
  1283	                epoch,
  1284	                &kp,
  1285	                &mut cas,
  1286	                &dummy_typed_tx(),
  1287	            );
  1288	        }
  1289	        // Reopen — CO1.4-extra sidecar replay restores the CAS index.
  1290	        let cas2 = CasStore::open(tmp.path()).expect("reopen");
  1291	        let err = replay_full_transition(
  1292	            Hash::ZERO,
  1293	            Hash::ZERO,
  1294	            &[entry],
  1295	            &cas2,
  1296	            &pinned,
  1297	            &preds,
  1298	            &tools,
  1299	        )
  1300	        .unwrap_err();
  1301	        // Stages 1-6 (incl. CAS lookup post-reopen) PASS; stage 7 stubs.
  1302	        assert!(matches!(err, ReplayError::Transition { at: 0, .. }));
  1303	    }
  1304	
  1305	    /// 18. sequencer_serial_replay_byte_identity — gated behind #[ignore]

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '150,330p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   150	
   151	/// TRACE_MATRIX FC1-Sig: typed rejection summary stamped by the predicate runner.
   152	#[derive(Debug, Clone, PartialEq, Eq)]
   153	pub struct RejectedAttemptSummary {
   154	    run_id: String,
   155	    attempt_id: String,
   156	    failure_class: String,
   157	    summary_hash: [u8; 32],
   158	}
   159	
   160	impl RejectedAttemptSummary {
   161	    /// TRACE_MATRIX FC1-Sig: construct a typed rejected-attempt summary, never a free-form sign blob.
   162	    pub fn new(
   163	        run_id: impl Into<String>,
   164	        attempt_id: impl Into<String>,
   165	        failure_class: impl Into<String>,
   166	        summary_hash: [u8; 32],
   167	    ) -> Self {
   168	        Self {
   169	            run_id: run_id.into(),
   170	            attempt_id: attempt_id.into(),
   171	            failure_class: failure_class.into(),
   172	            summary_hash,
   173	        }
   174	    }
   175	}
   176	
   177	// TRACE_MATRIX CO1.1.4-pre1 v1.1 round-1 closure (C-3 / Codex Q-C):
   178	// the typed `TerminalSummaryTx` struct (8-field per STATE § 1.5) now lives in
   179	// `state::typed_tx`. system_keypair signs an opaque digest via the
   180	// `CanonicalMessage::TerminalSummarySigning([u8; 32])` variant — same
   181	// opaque-digest pattern as `LedgerEntrySigning`, avoiding `bottom_white ↔ state`
   182	// circular dependency.
   183	
   184	/// TRACE_MATRIX FC3-Sig: typed continuity statement for system key rotation.
   185	#[derive(Debug, Clone, PartialEq, Eq)]
   186	pub struct EpochRotationProof {
   187	    old_epoch: SystemEpoch,
   188	    new_epoch: SystemEpoch,
   189	    old_pubkey: SystemPublicKey,
   190	    new_pubkey: SystemPublicKey,
   191	    signed_at_unix: u64,
   192	}
   193	
   194	impl EpochRotationProof {
   195	    /// TRACE_MATRIX FC3-Sig: construct a typed epoch-rotation continuity proof.
   196	    pub const fn new(
   197	        old_epoch: SystemEpoch,
   198	        new_epoch: SystemEpoch,
   199	        old_pubkey: SystemPublicKey,
   200	        new_pubkey: SystemPublicKey,
   201	        signed_at_unix: u64,
   202	    ) -> Self {
   203	        Self {
   204	            old_epoch,
   205	            new_epoch,
   206	            old_pubkey,
   207	            new_pubkey,
   208	            signed_at_unix,
   209	        }
   210	    }
   211	
   212	    /// TRACE_MATRIX FC3-Sig: old signing epoch certified by the rotation proof.
   213	    pub const fn old_epoch(&self) -> SystemEpoch {
   214	        self.old_epoch
   215	    }
   216	
   217	    /// TRACE_MATRIX FC3-Sig: new signing epoch certified by the rotation proof.
   218	    pub const fn new_epoch(&self) -> SystemEpoch {
   219	        self.new_epoch
   220	    }
   221	}
   222	
   223	/// TRACE_MATRIX FC1-Sig+FC3-Sig: only typed runtime messages may enter signature verification.
   224	#[derive(Debug, Clone, PartialEq, Eq)]
   225	pub enum CanonicalMessage {
   226	    /// TRACE_MATRIX FC1-Sig: predicate-runner rejection summary.
   227	    RejectedAttemptSummary(RejectedAttemptSummary),
   228	    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.1 closure C-3): terminal
   229	    /// summary signing-payload digest. Opaque `[u8; 32]` produced by
   230	    /// `state::typed_tx::TerminalSummarySigningPayload::canonical_digest()`
   231	    /// (a/k/a `TerminalSummaryTx::to_signing_payload().canonical_digest()`);
   232	    /// this variant only carries the 32-byte digest into the typed sign API.
   233	    /// Same opaque-digest pattern as `LedgerEntrySigning`; avoids a circular
   234	    /// `system_keypair ↔ state` module dependency.
   235	    TerminalSummarySigning([u8; 32]),
   236	    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): finalize
   237	    /// reward signing-payload digest. Opaque `[u8; 32]` produced by
   238	    /// `state::typed_tx::FinalizeRewardSigningPayload::canonical_digest()`.
   239	    FinalizeRewardSigning([u8; 32]),
   240	    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): task
   241	    /// expire signing-payload digest. Opaque `[u8; 32]` produced by
   242	    /// `state::typed_tx::TaskExpireSigningPayload::canonical_digest()`.
   243	    TaskExpireSigning([u8; 32]),
   244	    /// TRACE_MATRIX FC3-Sig: system key epoch continuity proof.
   245	    EpochRotationProof(EpochRotationProof),
   246	    /// TRACE_MATRIX FC2-Append (CO1.7 v1.2 round-2 closure C3): L4 transition_ledger
   247	    /// signing payload digest. Opaque [u8; 32] — full canonical_digest of
   248	    /// `LedgerEntrySigningPayload` is computed in `transition_ledger`; this variant
   249	    /// only carries the 32-byte digest into the typed sign API. Avoids a circular
   250	    /// `system_keypair ↔ transition_ledger` module dependency while preserving the
   251	    /// "all sign goes through CanonicalMessage" invariant.
   252	    LedgerEntrySigning([u8; 32]),
   253	}
   254	
   255	/// TRACE_MATRIX FC1-Sig+FC3-Sig: epoch-indexed public keys pinned by genesis and rotation history.
   256	#[derive(Debug, Clone, Default, PartialEq, Eq)]
   257	pub struct PinnedSystemPubkeys {
   258	    keys: BTreeMap<SystemEpoch, SystemPublicKey>,
   259	}
   260	
   261	impl PinnedSystemPubkeys {
   262	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: create an empty pinned system-key map.
   263	    pub fn new() -> Self {
   264	        Self::default()
   265	    }
   266	
   267	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: pin a public key for a system epoch.
   268	    pub fn insert(
   269	        &mut self,
   270	        epoch: SystemEpoch,
   271	        public_key: SystemPublicKey,
   272	    ) -> Option<SystemPublicKey> {
   273	        self.keys.insert(epoch, public_key)
   274	    }
   275	
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
   313	            return Err(KeypairError::InvalidFormat("bad plaintext length"));
   314	        }
   315	        let mut secret = [0u8; SECRET_KEY_LEN];
   316	        secret.copy_from_slice(&plaintext[..SECRET_KEY_LEN]);
   317	        let mut public = [0u8; PUBLIC_KEY_LEN];
   318	        public.copy_from_slice(&plaintext[SECRET_KEY_LEN..]);
   319	
   320	        let signing_key = SigningKey::from_bytes(&secret);
   321	        if signing_key.verifying_key().to_bytes() != public {
   322	            secret.zeroize();
   323	            return Err(KeypairError::InvalidFormat(
   324	                "public key does not match private key",
   325	            ));
   326	        }
   327	
   328	        let mut keypair = Self {
   329	            secret_key: Vec::from(secret).into_boxed_slice(),
   330	            public_key: SystemPublicKey::from_bytes(public),

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '360,560p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   360	    }
   361	
   362	    fn mlock_private_key_best_effort(&mut self) -> bool {
   363	        mlock_best_effort(self.secret_key.as_ptr(), self.secret_key.len())
   364	    }
   365	}
   366	
   367	/// TRACE_MATRIX FC1-Sig+FC3-Sig: system keypair lifecycle and crypto error taxonomy.
   368	#[derive(Debug)]
   369	pub enum KeypairError {
   370	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: filesystem operation failed.
   371	    Io(std::io::Error),
   372	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: secure operating-system entropy failed.
   373	    Entropy(getrandom::Error),
   374	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: KDF environment parameter was absent or invalid.
   375	    KdfParam(String),
   376	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: Argon2id key derivation failed.
   377	    Kdf(argon2::Error),
   378	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: ChaCha20-Poly1305 encryption or authentication failed.
   379	    Crypto(&'static str),
   380	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: encrypted keystore format was malformed.
   381	    InvalidFormat(&'static str),
   382	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: default keystore path could not be resolved.
   383	    HomeUnavailable,
   384	}
   385	
   386	impl fmt::Display for KeypairError {
   387	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   388	        match self {
   389	            Self::Io(err) => write!(f, "system keypair I/O failed: {err}"),
   390	            Self::Entropy(err) => write!(f, "system keypair entropy failed: {err}"),
   391	            Self::KdfParam(msg) => write!(f, "system keypair KDF parameter invalid: {msg}"),
   392	            Self::Kdf(err) => write!(f, "system keypair KDF failed: {err}"),
   393	            Self::Crypto(msg) => write!(f, "system keypair crypto failed: {msg}"),
   394	            Self::InvalidFormat(msg) => write!(f, "system keypair keystore invalid: {msg}"),
   395	            Self::HomeUnavailable => {
   396	                write!(f, "system keypair default keystore path requires HOME")
   397	            }
   398	        }
   399	    }
   400	}
   401	
   402	impl std::error::Error for KeypairError {}
   403	
   404	impl From<std::io::Error> for KeypairError {
   405	    fn from(value: std::io::Error) -> Self {
   406	        Self::Io(value)
   407	    }
   408	}
   409	
   410	/// TRACE_MATRIX FC1-Sig+FC3-Sig: resolve `~/.turingos/keystore/system_keypair_v{epoch}.enc`.
   411	///
   412	/// `TURINGOS_KEYSTORE_PATH` overrides the default path. The default never
   413	/// points into the repository, CAS, or ledger directories.
   414	pub fn default_system_keystore_path(epoch: SystemEpoch) -> Result<PathBuf, KeypairError> {
   415	    if let Ok(path) = env::var("TURINGOS_KEYSTORE_PATH") {
   416	        return Ok(PathBuf::from(path));
   417	    }
   418	    let home = env::var("HOME").map_err(|_| KeypairError::HomeUnavailable)?;
   419	    Ok(PathBuf::from(home)
   420	        .join(".turingos")
   421	        .join("keystore")
   422	        .join(format!("system_keypair_v{}.enc", epoch.get())))
   423	}
   424	
   425	/// TRACE_MATRIX FC1-Sig+FC3-Sig: first-boot generate-or-second-boot decrypt lifecycle entrypoint.
   426	pub fn generate_or_load_system_keypair(
   427	    keystore_path: &Path,
   428	    user_kdf_password: &SecretString,
   429	) -> Result<Ed25519Keypair, KeypairError> {
   430	    if keystore_path.exists() {
   431	        return load_existing_keypair(keystore_path, user_kdf_password);
   432	    }
   433	
   434	    let keypair = Ed25519Keypair::generate_with_secure_entropy()?;
   435	    let encrypted = encrypt_at_rest(&keypair, user_kdf_password)?;
   436	    write_keystore_0600(keystore_path, &encrypted)?;
   437	    Ok(keypair)
   438	}
   439	
   440	/// TRACE_MATRIX FC1-Sig+FC3-Sig: decrypt an existing encrypted system keypair keystore.
   441	pub fn load_existing_keypair(
   442	    keystore_path: &Path,
   443	    user_kdf_password: &SecretString,
   444	) -> Result<Ed25519Keypair, KeypairError> {
   445	    let bytes = fs::read(keystore_path)?;
   446	    let encoded = EncryptedKeypair::decode(&bytes)?;
   447	    let mut key = derive_key(user_kdf_password, &encoded.salt, encoded.kdf)?;
   448	    let cipher = ChaCha20Poly1305::new_from_slice(&key)
   449	        .map_err(|_| KeypairError::Crypto("bad cipher key"))?;
   450	    let plaintext = cipher
   451	        .decrypt(
   452	            Nonce::from_slice(&encoded.nonce),
   453	            encoded.ciphertext.as_ref(),
   454	        )
   455	        .map_err(|_| KeypairError::Crypto("keystore authentication failed"))?;
   456	    key.zeroize();
   457	    Ed25519Keypair::from_plaintext(&plaintext)
   458	}
   459	
   460	/// TRACE_MATRIX FC1-Sig+FC3-Sig: canonical SHA-256 digest for typed system messages.
   461	pub fn canonical_digest(message: &CanonicalMessage) -> [u8; 32] {
   462	    let mut h = Sha256::new();
   463	    h.update(b"turingosv4.system_keypair.v1");
   464	    match message {
   465	        CanonicalMessage::RejectedAttemptSummary(summary) => {
   466	            h.update(b"RejectedAttemptSummary");
   467	            update_len_prefixed(&mut h, summary.run_id.as_bytes());
   468	            update_len_prefixed(&mut h, summary.attempt_id.as_bytes());
   469	            update_len_prefixed(&mut h, summary.failure_class.as_bytes());
   470	            h.update(summary.summary_hash);
   471	        }
   472	        CanonicalMessage::TerminalSummarySigning(digest) => {
   473	            h.update(b"TerminalSummarySigning");
   474	            h.update(digest);
   475	        }
   476	        CanonicalMessage::FinalizeRewardSigning(digest) => {
   477	            h.update(b"FinalizeRewardSigning");
   478	            h.update(digest);
   479	        }
   480	        CanonicalMessage::TaskExpireSigning(digest) => {
   481	            h.update(b"TaskExpireSigning");
   482	            h.update(digest);
   483	        }
   484	        CanonicalMessage::EpochRotationProof(proof) => {
   485	            h.update(b"EpochRotationProof");
   486	            h.update(proof.old_epoch.get().to_be_bytes());
   487	            h.update(proof.new_epoch.get().to_be_bytes());
   488	            h.update(proof.old_pubkey.as_bytes());
   489	            h.update(proof.new_pubkey.as_bytes());
   490	            h.update(proof.signed_at_unix.to_be_bytes());
   491	        }
   492	        CanonicalMessage::LedgerEntrySigning(digest) => {
   493	            h.update(b"LedgerEntrySigning");
   494	            h.update(digest);
   495	        }
   496	    }
   497	    h.finalize().into()
   498	}
   499	
   500	/// TRACE_MATRIX FC1-Sig+FC3-Sig: public system signature verification against pinned epoch keys.
   501	pub fn verify_system_signature(
   502	    sig: &SystemSignature,
   503	    message: &CanonicalMessage,
   504	    epoch: SystemEpoch,
   505	    pinned_pubkeys: &PinnedSystemPubkeys,
   506	) -> bool {
   507	    let Some(pk) = pinned_pubkeys.get(epoch) else {
   508	        return false;
   509	    };
   510	    let Ok(verifying_key) = VerifyingKey::from_bytes(pk.as_bytes()) else {
   511	        return false;
   512	    };
   513	    let signature = Signature::from_bytes(sig.as_bytes());
   514	    verifying_key
   515	        .verify(&canonical_digest(message), &signature)
   516	        .is_ok()
   517	}
   518	
   519	/// TRACE_MATRIX FC3-Sig: verify old and new signatures over a rotation continuity proof.
   520	pub fn verify_epoch_rotation_proof(
   521	    proof: &EpochRotationProof,
   522	    old_signature: &SystemSignature,
   523	    new_signature: &SystemSignature,
   524	    pinned_pubkeys: &PinnedSystemPubkeys,
   525	) -> bool {
   526	    let message = CanonicalMessage::EpochRotationProof(proof.clone());
   527	    verify_system_signature(old_signature, &message, proof.old_epoch(), pinned_pubkeys)
   528	        && verify_system_signature(new_signature, &message, proof.new_epoch(), pinned_pubkeys)
   529	}
   530	
   531	/// TRACE_MATRIX FC3-Sig: boot extension stub for genesis `[system_pubkeys]` verification.
   532	pub fn verify_system_pubkeys(genesis_payload_toml: &str) -> Result<(), TrustRootError> {
   533	    if !has_toml_section(genesis_payload_toml, "system_pubkeys") {
   534	        return Ok(());
   535	    }
   536	    // TODO(CO1.7): parse genesis_payload.toml [system_pubkeys] entries and
   537	    // verify creator PGP signatures against the pinned creator public key.
   538	    Ok(())
   539	}
   540	
   541	/// TRACE_MATRIX FC1-Sig: crate-only signing surface for the predicate runner.
   542	pub(crate) mod predicate_runner {
   543	    use super::{
   544	        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, KeypairError,
   545	        RejectedAttemptSummary, SystemSignature,
   546	    };
   547	
   548	    /// TRACE_MATRIX FC1-Sig: sign only typed rejected-attempt summaries from the predicate runner.
   549	    pub(crate) fn sign_rejected_attempt_summary(
   550	        keypair: &Ed25519Keypair,
   551	        summary: &RejectedAttemptSummary,
   552	    ) -> Result<SystemSignature, KeypairError> {
   553	        sign_system_message_inner(
   554	            keypair,
   555	            &CanonicalMessage::RejectedAttemptSummary(summary.clone()),
   556	        )
   557	    }
   558	
   559	    /// TRACE_MATRIX FC1-Sig: sign only typed canonical messages within the predicate-runner scope.
   560	    pub(crate) fn sign_system_message(

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '1,520p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! Typed transaction ABI surface — CO1.1.4-pre1.
     2	//!
     3	//! Spec authority:
     4	//! - `handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — this atom
     5	//! - `STATE_TRANSITION_SPEC_v1_2026-04-27.md` § 1 (typed schemas), § 2.5
     6	//!   (canonical serialization), § 3 (transition pseudocode used to derive
     7	//!   FinalizeRewardTx schema in spec § 4)
     8	//!
     9	//! Why this module exists: when CO1.7-impl A1 (Git2LedgerWriter) shipped, the
    10	//! downstream A2 (Sequencer + `dispatch_transition`) needed a `TypedTx` enum
    11	//! whose variants carry per-kind tx structs. Those structs and ~20 supporting
    12	//! types (identifiers, signatures, predicate-result types, status enums) were
    13	//! "frozen on paper" in STATE_TRANSITION_SPEC § 1 but had no Rust definition.
    14	//! CO1.1.4-pre1 lands them in isolation under its own dual-audit gate,
    15	//! per the project's per-atom audit principle (CLAUDE.md "Audit Standard").
    16	//!
    17	//! /// TRACE_MATRIX FC2-Submit + § 1 typed schemas: typed-tx ABI surface.
    18	
    19	use serde::{Deserialize, Serialize};
    20	use std::collections::{BTreeMap, BTreeSet};
    21	
    22	use sha2::{Digest, Sha256};
    23	
    24	use crate::bottom_white::cas::schema::Cid;
    25	use crate::bottom_white::ledger::system_keypair::{serde_bytes_64, SystemEpoch, SystemSignature};
    26	use crate::economy::money::{MicroCoin, StakeMicroCoin};
    27	use crate::state::q_state::{AgentId, Hash, TxId};
    28	
    29	// ────────────────────────────────────────────────────────────────────────────
    30	// § 2 Identifier newtypes (all opaque strings to Q_t)
    31	// ────────────────────────────────────────────────────────────────────────────
    32	
    33	/// TRACE_MATRIX § 1.2 — task-market entry id; opaque string.
    34	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    35	pub struct TaskId(pub String);
    36	
    37	/// TRACE_MATRIX § 1.5 — runtime run id (one run per `Sequencer` driver lifecycle).
    38	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    39	pub struct RunId(pub String);
    40	
    41	/// TRACE_MATRIX STATE § 3.4 + § 4 I-FINALIZE-BATCH-ORDER — typed claim id used
    42	/// in `FinalizeRewardTx.claim_id` and `ClaimsIndex` keying. Wraps `TxId`
    43	/// (the underlying claim is recorded against the work_tx's TxId in
    44	/// ClaimsIndex per current QState shape) but **prevents accidental mixing
    45	/// of claim references with arbitrary transaction references** at the type
    46	/// level (Codex round-1 Q-B CHALLENGE).
    47	///
    48	/// `#[serde(transparent)]` — wire-identical to TxId, so adoption is
    49	/// non-breaking for canonical encoding.
    50	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    51	#[serde(transparent)]
    52	pub struct ClaimId(pub TxId);
    53	
    54	impl ClaimId {
    55	    pub fn new(s: impl Into<String>) -> Self {
    56	        Self(TxId(s.into()))
    57	    }
    58	    pub fn as_tx_id(&self) -> &TxId {
    59	        &self.0
    60	    }
    61	}
    62	
    63	/// TRACE_MATRIX § 1.3 ReuseTx + L2 Tool Registry — opaque tool identifier.
    64	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    65	pub struct ToolId(pub String);
    66	
    67	/// TRACE_MATRIX § 1.2 PredicateResultsBundle + L1 Predicate Registry — opaque predicate id.
    68	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    69	pub struct PredicateId(pub String);
    70	
    71	/// TRACE_MATRIX § 1.2 WorkTx field 5 — read-set key (DAG attribution / replay).
    72	/// Kept as opaque string in v1; stricter typing (path / tape-coordinate) lands
    73	/// in CO P2.4.0 attribution-engine spike.
    74	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    75	pub struct ReadKey(pub String);
    76	
    77	/// TRACE_MATRIX § 1.2 WorkTx field 6 — write-set key (DAG attribution / replay).
    78	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    79	pub struct WriteKey(pub String);
    80	
    81	// ────────────────────────────────────────────────────────────────────────────
    82	// § 3 AgentSignature (Ed25519 [u8;64], type-distinct from SystemSignature)
    83	// ────────────────────────────────────────────────────────────────────────────
    84	
    85	/// TRACE_MATRIX § 1.2 WorkTx field 10 + I-SIG: agent-side detached Ed25519
    86	/// signature over the per-tx canonical_digest. Distinct type from
    87	/// `SystemSignature` to prevent accidental confusion at API boundaries
    88	/// (Codex sec-arg: agent-vs-system signature mixing is a real hazard).
    89	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    90	pub struct AgentSignature(#[serde(with = "serde_bytes_64")] [u8; 64]);
    91	
    92	impl AgentSignature {
    93	    pub const fn from_bytes(bytes: [u8; 64]) -> Self {
    94	        Self(bytes)
    95	    }
    96	    pub const fn as_bytes(&self) -> &[u8; 64] {
    97	        &self.0
    98	    }
    99	}
   100	
   101	impl Default for AgentSignature {
   102	    fn default() -> Self {
   103	        Self([0u8; 64])
   104	    }
   105	}
   106	
   107	// ────────────────────────────────────────────────────────────────────────────
   108	// § 3 SlashEvidenceCid (newtype; type-distinct slash-evidence reference)
   109	// ────────────────────────────────────────────────────────────────────────────
   110	
   111	/// TRACE_MATRIX § 1.2 TxStatus::FinalizedSlash — typed reference to the
   112	/// counter-example payload that justified the slash (lives in L3 CAS).
   113	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
   114	#[serde(transparent)]
   115	pub struct SlashEvidenceCid(pub Cid);
   116	
   117	// ────────────────────────────────────────────────────────────────────────────
   118	// § 4 Predicate result types
   119	// ────────────────────────────────────────────────────────────────────────────
   120	
   121	/// TRACE_MATRIX § 1.2 PredicateResultsBundle — boolean predicate verdict
   122	/// optionally accompanied by an L3 CAS reference to the proof object
   123	/// (e.g. Lean witness, ZK proof bytes).
   124	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   125	pub struct BoolWithProof {
   126	    pub value: bool,
   127	    pub proof_cid: Option<Cid>,
   128	}
   129	
   130	/// TRACE_MATRIX § 1.2 PredicateResultsBundle — safety-class discriminator.
   131	/// Determines fail-closed (Safety) vs fail-open-with-signal (Creation) behavior
   132	/// when a predicate's evaluation errors. Frozen STATE spec § 1.2.
   133	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   134	#[repr(u8)]
   135	pub enum SafetyOrCreation {
   136	    Safety = 0,
   137	    Creation = 1,
   138	}
   139	
   140	impl Default for SafetyOrCreation {
   141	    fn default() -> Self {
   142	        // Safety bias by default: fail-closed if no class declared.
   143	        Self::Safety
   144	    }
   145	}
   146	
   147	/// TRACE_MATRIX § 1.2 WorkTx field 8 — runner-stamped predicate results
   148	/// (acceptance + settlement gates) with explicit safety-class discriminator.
   149	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   150	pub struct PredicateResultsBundle {
   151	    pub acceptance: BTreeMap<PredicateId, BoolWithProof>,
   152	    pub settlement: BTreeMap<PredicateId, BoolWithProof>,
   153	    pub safety_class: SafetyOrCreation,
   154	}
   155	
   156	// ────────────────────────────────────────────────────────────────────────────
   157	// § 5 Status / class enums (RejectionClass, VerifyVerdict, RunOutcome,
   158	//                          and the runtime-only TxStatus per D-1)
   159	// ────────────────────────────────────────────────────────────────────────────
   160	
   161	/// TRACE_MATRIX § 1.4 — classification of a rejected attempt.
   162	/// Public predicates are classified concretely; private predicates surface as
   163	/// `Opaque` (no information leakage to attacker).
   164	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   165	pub enum RejectionClass {
   166	    AcceptancePredicateFail(PredicateId),
   167	    SettlementPredicateFail(PredicateId),
   168	    StakeInsufficient,
   169	    SignatureInvalid,
   170	    StaleParentRoot,
   171	    Opaque,
   172	    BudgetExceeded,
   173	}
   174	
   175	/// TRACE_MATRIX § 1.3 VerifyTx field 5 — verifier verdict.
   176	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   177	#[repr(u8)]
   178	pub enum VerifyVerdict {
   179	    Confirm = 0,
   180	    Doubt = 1,
   181	}
   182	
   183	/// TRACE_MATRIX § 1.5 TerminalSummaryTx field 4 + Art. IV halt-reason taxonomy.
   184	/// Five-way partition over how a run terminates.
   185	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   186	#[repr(u8)]
   187	pub enum RunOutcome {
   188	    OmegaAccepted = 0,
   189	    MaxTxExhausted = 1,
   190	    WallClockCap = 2,
   191	    ComputeCap = 3,
   192	    ErrorHalt = 4,
   193	}
   194	
   195	/// TRACE_MATRIX § 1.2 TxStatus — **runtime book-keeping only** (D-1 divergence
   196	/// from STATE spec): never serialized into a TypedTx variant's wire bytes.
   197	/// Tracked in `q_t.q_t.agents[id].last_accepted_tx` + `ClaimsIndex`. Exposed
   198	/// here as a public type for the runtime API surface.
   199	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   200	pub enum TxStatus {
   201	    Pending,
   202	    Accepted,
   203	    Rejected(RejectionClass),
   204	    FinalizedReward(MicroCoin),
   205	    FinalizedSlash(SlashEvidenceCid),
   206	}
   207	
   208	// ────────────────────────────────────────────────────────────────────────────
   209	// § 5 (cont'd) — Typed tx structs (STATE spec § 1.2-1.6 + § 3.6)
   210	// ────────────────────────────────────────────────────────────────────────────
   211	
   212	/// TRACE_MATRIX § 1.2 — agent-submitted work transaction (12-field schema;
   213	/// **D-1 divergence**: field 12 `status: TxStatus` is excluded from canonical
   214	/// wire bytes — TxStatus is runner book-keeping per CO1.1.4-pre1 spec § 5).
   215	///
   216	/// This is the per-tx struct that the CO1.7 sequencer hands to
   217	/// `step_transition` (CO1.7.5 body atom). The `signature` is over
   218	/// `WorkSigningPayload::canonical_digest()` — i.e. the projection produced by
   219	/// `WorkTx::to_signing_payload()` (excludes the signature field itself; per
   220	/// v1.1 P1 the digest pre-image carries the `b"turingosv4.agent_sig.work.v1"`
   221	/// domain prefix).
   222	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   223	pub struct WorkTx {
   224	    pub tx_id: TxId,                                  //  1
   225	    pub task_id: TaskId,                              //  2
   226	    pub parent_state_root: Hash,                      //  3
   227	    pub agent_id: AgentId,                            //  4
   228	    pub read_set: BTreeSet<ReadKey>,                  //  5
   229	    pub write_set: BTreeSet<WriteKey>,                //  6
   230	    pub proposal_cid: Cid,                            //  7
   231	    pub predicate_results: PredicateResultsBundle,    //  8 (runner-stamped)
   232	    pub stake: StakeMicroCoin,                        //  9
   233	    pub signature: AgentSignature,                    // 10
   234	    pub timestamp_logical: u64,                       // 11
   235	    // 12: TxStatus — D-1 elision; runtime-only.
   236	}
   237	
   238	/// TRACE_MATRIX § 1.3 — verifier verdict transaction.
   239	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   240	pub struct VerifyTx {
   241	    pub tx_id: TxId,                       //  1
   242	    pub target_work_tx: TxId,              //  2
   243	    pub verifier_agent: AgentId,           //  3
   244	    pub bond: StakeMicroCoin,              //  4
   245	    pub verdict: VerifyVerdict,            //  5
   246	    pub signature: AgentSignature,         //  6
   247	    pub timestamp_logical: u64,            //  7
   248	}
   249	
   250	impl Default for VerifyVerdict {
   251	    fn default() -> Self {
   252	        Self::Confirm
   253	    }
   254	}
   255	
   256	/// TRACE_MATRIX § 1.3 — challenge transaction (counter-example posted with
   257	/// stake at risk).
   258	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   259	pub struct ChallengeTx {
   260	    pub tx_id: TxId,                       //  1
   261	    pub target_work_tx: TxId,              //  2
   262	    pub challenger_agent: AgentId,         //  3
   263	    pub stake: StakeMicroCoin,             //  4
   264	    pub counterexample_cid: Cid,           //  5
   265	    pub signature: AgentSignature,         //  6
   266	    pub timestamp_logical: u64,            //  7
   267	}
   268	
   269	/// TRACE_MATRIX § 1.3 — fact-tx recording reuse of a tool created by a prior
   270	/// agent (royalty graph edge). No submitting agent (per § 3.6.5 v1.3).
   271	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   272	pub struct ReuseTx {
   273	    pub tx_id: TxId,                       //  1
   274	    pub reusing_work_tx: TxId,             //  2
   275	    pub reused_tool_id: ToolId,            //  3
   276	    pub reused_tool_creator: AgentId,      //  4
   277	    pub timestamp_logical: u64,            //  5
   278	}
   279	
   280	/// TRACE_MATRIX CO1.1.4-pre1 spec § 4 — derived schema (STATE spec § 3.4
   281	/// uses opaque `FinalizeTx::from(claim_id, reward)` constructor without an
   282	/// explicit struct definition).
   283	///
   284	/// **v1.1 round-1 audit closures**:
   285	/// - **C-3 (Codex Q-B)**: `claim_id` is now a typed `ClaimId` newtype (was
   286	///   bare `TxId`) — STATE § 4 I-FINALIZE-BATCH-ORDER speaks in claim_id;
   287	///   reusing TxId leaked QState implementation into the wire format.
   288	/// - **C-3 (Codex Q-B)**: `task_id` / `solver` / `reward` are documented as
   289	///   **Q-DERIVED at replay** — replay (CO1.7-impl A4) re-fetches them from
   290	///   ClaimsIndex by `claim_id`, NOT trusted from wire. Wire fields are kept
   291	///   as a ledger summary (so a human reading L4 can see the finalize event
   292	///   semantics) but the AUTHORITATIVE values come from Q_t.
   293	/// - **C-3 / GM-2 followup**: `system_signature` is RETAINED for v1.1 — it
   294	///   binds the system-emitted FinalizeRewardTx to a specific runtime keypair
   295	///   epoch (auditability + cross-cell trust). The CO1.7 `LedgerEntry`
   296	///   wraps this struct via CAS reference + signs the `LedgerEntrySigningPayload`
   297	///   digest; the two sigs are NOT redundant: this one binds the tx-payload
   298	///   bytes; the L4 envelope sig binds the (logical_t, parent_ledger_root, tx_payload_cid)
   299	///   sequencer-stamped envelope. v1.1 spec § 4 makes the dual-sign rationale explicit.
   300	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   301	pub struct FinalizeRewardTx {
   302	    pub tx_id: TxId,                       //  1
   303	    pub claim_id: ClaimId,                 //  2 — typed (was TxId in v1)
   304	    pub task_id: TaskId,                   //  3 — Q-derived authoritative; wire = ledger summary
   305	    pub solver: AgentId,                   //  4 — Q-derived authoritative; wire = ledger summary
   306	    pub reward: MicroCoin,                 //  5 — Q-derived authoritative (SettlementEngine output); wire = ledger summary
   307	    pub parent_state_root: Hash,           //  6
   308	    pub epoch: SystemEpoch,                //  7
   309	    pub timestamp_logical: u64,            //  8
   310	    pub system_signature: SystemSignature, //  9 — see doc-comment on dual-sign rationale
   311	}
   312	
   313	/// TRACE_MATRIX STATE spec § 3.6 v1.3 — system-emitted task-expiry tx
   314	/// (refunds bounty + locked stakes when no claim finalized by deadline).
   315	/// Verbatim transcription.
   316	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   317	pub struct TaskExpireTx {
   318	    pub tx_id: TxId,                       //  1
   319	    pub task_id: TaskId,                   //  2
   320	    pub parent_state_root: Hash,           //  3
   321	    pub bounty_refunded: MicroCoin,        //  4 (computed by runtime; included for ledger summary)
   322	    pub epoch: SystemEpoch,                //  5
   323	    pub timestamp_logical: u64,            //  6
   324	    pub system_signature: SystemSignature, //  7
   325	}
   326	
   327	/// TRACE_MATRIX STATE spec § 1.5 — system-emitted no-accept-run handler.
   328	/// Emitted exactly once if a run terminates without any accepted work_tx, so
   329	/// L6 reconstructibility (failure-class signal) is preserved on the tape
   330	/// even when no work_tx ever passed.
   331	///
   332	/// **v1.1 round-1 audit closure (C-3 Codex Q-C must-fix-now)**: replaces the
   333	/// 3-field placeholder previously living in `system_keypair.rs`. Full
   334	/// 8-field schema per STATE § 1.5. The signer (`system_keypair`) now signs
   335	/// an opaque `TerminalSummarySigning([u8; 32])` digest — same opaque-digest
   336	/// pattern as `LedgerEntrySigning` — so the digest is computed here via
   337	/// `TerminalSummaryTx::to_signing_payload().canonical_digest()` (with the
   338	/// `b"turingosv4.system_sig.terminal_summary.v1"` domain prefix per v1.1 P1)
   339	/// and `system_keypair` stays oblivious to the typed-tx schema (no circular
   340	/// `bottom_white ↔ state` dependency).
   341	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   342	pub struct TerminalSummaryTx {
   343	    pub tx_id: TxId,                                          //  1
   344	    pub task_id: TaskId,                                      //  2
   345	    pub run_id: RunId,                                        //  3
   346	    pub run_outcome: RunOutcome,                              //  4
   347	    pub total_attempts: u32,                                  //  5
   348	    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,// 6
   349	    pub last_logical_t: u64,                                  //  7
   350	    pub system_signature: SystemSignature,                    //  8
   351	}
   352	
   353	impl Default for RunOutcome {
   354	    fn default() -> Self {
   355	        Self::OmegaAccepted
   356	    }
   357	}
   358	
   359	// ────────────────────────────────────────────────────────────────────────────
   360	// § 7 Signing payloads (CO1.1.4-pre1 v1.1 round-1 closure C-1)
   361	//
   362	// Each agent-signed and system-emitted typed-tx has a paired `*SigningPayload`
   363	// struct (subset of fields, EXCLUDES the signature itself) with a
   364	// `canonical_digest()` method that **prepends a stable domain-separation
   365	// prefix** before the bincode-canonical body bytes. This implements:
   366	//
   367	//   sig_input = sha256(b"turingosv4.<actor>.<purpose>.v1" || canonical_encode(payload))
   368	//
   369	// Property: even if two distinct payload TYPES happen to bincode-encode to
   370	// identical bytes (extremely unlikely given distinct field shapes, but
   371	// defensively guaranteed), the domain prefix ensures the SHA-256 inputs
   372	// differ. Closes Codex Q-E + Gemini Q7: type-level distinction is necessary
   373	// but not sufficient as a security boundary.
   374	//
   375	// **Forward dependency**: actual `verify_agent_signature(sig, payload, agent_pubkey)`
   376	// + agent-pubkey-registry lookup is CO P2.x AgentRegistry territory; this
   377	// atom only freezes the canonical_digest pre-image.
   378	// ────────────────────────────────────────────────────────────────────────────
   379	
   380	const DOMAIN_AGENT_WORK: &[u8] = b"turingosv4.agent_sig.work.v1";
   381	const DOMAIN_AGENT_VERIFY: &[u8] = b"turingosv4.agent_sig.verify.v1";
   382	const DOMAIN_AGENT_CHALLENGE: &[u8] = b"turingosv4.agent_sig.challenge.v1";
   383	const DOMAIN_SYSTEM_FINALIZE_REWARD: &[u8] = b"turingosv4.system_sig.finalize_reward.v1";
   384	const DOMAIN_SYSTEM_TASK_EXPIRE: &[u8] = b"turingosv4.system_sig.task_expire.v1";
   385	const DOMAIN_SYSTEM_TERMINAL_SUMMARY: &[u8] = b"turingosv4.system_sig.terminal_summary.v1";
   386	
   387	/// Reserved for v4.1 MetaTx (Gemini round-2 GR-1 recommendation).
   388	/// Not used in v4 — namespace placeholder so v4.1 can introduce
   389	/// `MetaSigningPayload` without re-rotating sibling domains. Marked
   390	/// `#[allow(dead_code)]` because no v4 consumer references it.
   391	#[allow(dead_code)]
   392	const DOMAIN_AGENT_META_PROPOSAL: &[u8] = b"turingosv4.agent_sig.meta_proposal.v1";
   393	
   394	fn domain_prefixed_digest<T: Serialize>(domain: &[u8], value: &T) -> [u8; 32] {
   395	    use crate::bottom_white::ledger::transition_ledger::canonical_encode;
   396	    let body = canonical_encode(value).expect("canonical_encode of signing payload");
   397	    let mut h = Sha256::new();
   398	    h.update(domain);
   399	    h.update(&body);
   400	    h.finalize().into()
   401	}
   402	
   403	/// Agent signing payload for `WorkTx` (12 fields → 11 fields; signature excluded).
   404	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   405	pub struct WorkSigningPayload {
   406	    pub tx_id: TxId,
   407	    pub task_id: TaskId,
   408	    pub parent_state_root: Hash,
   409	    pub agent_id: AgentId,
   410	    pub read_set: BTreeSet<ReadKey>,
   411	    pub write_set: BTreeSet<WriteKey>,
   412	    pub proposal_cid: Cid,
   413	    pub predicate_results: PredicateResultsBundle,
   414	    pub stake: StakeMicroCoin,
   415	    pub timestamp_logical: u64,
   416	}
   417	
   418	impl WorkSigningPayload {
   419	    pub fn canonical_digest(&self) -> [u8; 32] {
   420	        domain_prefixed_digest(DOMAIN_AGENT_WORK, self)
   421	    }
   422	}
   423	
   424	/// Agent signing payload for `VerifyTx` (7 fields → 6 fields).
   425	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   426	pub struct VerifySigningPayload {
   427	    pub tx_id: TxId,
   428	    pub target_work_tx: TxId,
   429	    pub verifier_agent: AgentId,
   430	    pub bond: StakeMicroCoin,
   431	    pub verdict: VerifyVerdict,
   432	    pub timestamp_logical: u64,
   433	}
   434	
   435	impl VerifySigningPayload {
   436	    pub fn canonical_digest(&self) -> [u8; 32] {
   437	        domain_prefixed_digest(DOMAIN_AGENT_VERIFY, self)
   438	    }
   439	}
   440	
   441	/// Agent signing payload for `ChallengeTx` (7 fields → 6 fields).
   442	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   443	pub struct ChallengeSigningPayload {
   444	    pub tx_id: TxId,
   445	    pub target_work_tx: TxId,
   446	    pub challenger_agent: AgentId,
   447	    pub stake: StakeMicroCoin,
   448	    pub counterexample_cid: Cid,
   449	    pub timestamp_logical: u64,
   450	}
   451	
   452	impl ChallengeSigningPayload {
   453	    pub fn canonical_digest(&self) -> [u8; 32] {
   454	        domain_prefixed_digest(DOMAIN_AGENT_CHALLENGE, self)
   455	    }
   456	}
   457	
   458	/// System signing payload for `FinalizeRewardTx` (9 fields → 8 fields).
   459	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   460	pub struct FinalizeRewardSigningPayload {
   461	    pub tx_id: TxId,
   462	    pub claim_id: ClaimId,
   463	    pub task_id: TaskId,
   464	    pub solver: AgentId,
   465	    pub reward: MicroCoin,
   466	    pub parent_state_root: Hash,
   467	    pub epoch: SystemEpoch,
   468	    pub timestamp_logical: u64,
   469	}
   470	
   471	impl FinalizeRewardSigningPayload {
   472	    pub fn canonical_digest(&self) -> [u8; 32] {
   473	        domain_prefixed_digest(DOMAIN_SYSTEM_FINALIZE_REWARD, self)
   474	    }
   475	}
   476	
   477	/// System signing payload for `TaskExpireTx` (7 fields → 6 fields).
   478	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   479	pub struct TaskExpireSigningPayload {
   480	    pub tx_id: TxId,
   481	    pub task_id: TaskId,
   482	    pub parent_state_root: Hash,
   483	    pub bounty_refunded: MicroCoin,
   484	    pub epoch: SystemEpoch,
   485	    pub timestamp_logical: u64,
   486	}
   487	
   488	impl TaskExpireSigningPayload {
   489	    pub fn canonical_digest(&self) -> [u8; 32] {
   490	        domain_prefixed_digest(DOMAIN_SYSTEM_TASK_EXPIRE, self)
   491	    }
   492	}
   493	
   494	/// System signing payload for `TerminalSummaryTx` (8 fields → 7 fields).
   495	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   496	pub struct TerminalSummarySigningPayload {
   497	    pub tx_id: TxId,
   498	    pub task_id: TaskId,
   499	    pub run_id: RunId,
   500	    pub run_outcome: RunOutcome,
   501	    pub total_attempts: u32,
   502	    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,
   503	    pub last_logical_t: u64,
   504	}
   505	
   506	impl TerminalSummarySigningPayload {
   507	    pub fn canonical_digest(&self) -> [u8; 32] {
   508	        domain_prefixed_digest(DOMAIN_SYSTEM_TERMINAL_SUMMARY, self)
   509	    }
   510	}
   511	
   512	// ── Projections: tx → signing payload ────────────────────────────────────
   513	
   514	impl WorkTx {
   515	    pub fn to_signing_payload(&self) -> WorkSigningPayload {
   516	        WorkSigningPayload {
   517	            tx_id: self.tx_id.clone(),
   518	            task_id: self.task_id.clone(),
   519	            parent_state_root: self.parent_state_root,
   520	            agent_id: self.agent_id.clone(),

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! Q_t — system state vector per `STATE_TRANSITION_SPEC v1.4 § 1.1`.
     2	//!
     3	//! TRACE_MATRIX Art 0.1 — 四要素映射: `QState` provides the tape/control mapping.
     4	//! TRACE_MATRIX Art 0.4 — Q_t version-controlled: `head_t` = git commit SHA in Path B substrate.
     5	//! TRACE_MATRIX Art IV — Boot: `QState::genesis` is the starting state of every runtime.
     6	//! TRACE_MATRIX WP § 0 axiom 1 — state monotonicity: Q_t evolves only via accepted transitions.
     7	//! TRACE_MATRIX WP § 4 — 9-component system state.
     8	//! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
     9	//!
    10	//! **BTreeMap, not HashMap, everywhere** (Inv determinism;
    11	//! Codex flagged `kernel.rs:187-204` HashMap nondeterminism in round-2).
    12	//!
    13	//! Sub-types whose entry shapes are scoped to later atoms (CO P2.x economic engine,
    14	//! CO1.7 transition_ledger) are intentionally minimal here — full schemas land per atom,
    15	//! but the *index typing* (BTreeMap newtype shells) freezes here so Q_t is total.
    16	
    17	use std::collections::BTreeMap;
    18	
    19	use serde::{Deserialize, Serialize};
    20	
    21	use crate::economy::money::MicroCoin;
    22	
    23	// ────────────────────────────────────────────────────────────────────────────
    24	// Newtype primitives — minimal, deterministic, serde-ready.
    25	// ────────────────────────────────────────────────────────────────────────────
    26	
    27	/// TRACE_MATRIX § 1.1 — generic 32-byte hash (sha256). State / ledger / registry roots.
    28	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    29	pub struct Hash(pub [u8; 32]);
    30	
    31	impl Hash {
    32	    /// TRACE_MATRIX § 1.1 — additive identity (genesis state-root, ledger-root, etc.).
    33	    pub const ZERO: Hash = Hash([0u8; 32]);
    34	
    35	    /// TRACE_MATRIX § 1.1 — construct from a 32-byte digest (sha256 output).
    36	    pub fn from_bytes(b: [u8; 32]) -> Self {
    37	        Hash(b)
    38	    }
    39	}
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
    71	/// TRACE_MATRIX § 1.1 — reputation snapshot. Signed i64 to permit negative reputation
    72	/// (e.g. post-slash); ledger-of-record lives in `ReputationsIndex` (CO P2.9).
    73	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    74	pub struct Reputation(pub i64);
    75	
    76	// ────────────────────────────────────────────────────────────────────────────
    77	// AgentSwarmState + PerAgentState — spec § 1.1 verbatim.
    78	// ────────────────────────────────────────────────────────────────────────────
    79	
    80	/// TRACE_MATRIX § 1.1 — agent swarm sub-state.
    81	/// MUST be reconstructible from L4 transition ledger replay.
    82	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    83	pub struct AgentSwarmState {
    84	    pub agents: BTreeMap<AgentId, PerAgentState>,
    85	    pub current_round: u64,
    86	}
    87	
    88	/// TRACE_MATRIX § 1.1 — per-agent runtime state.
    89	/// `retry_counter_for_current_task` resets on accept; persists across rejections.
    90	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    91	pub struct PerAgentState {
    92	    pub reputation_snapshot: Reputation,
    93	    pub last_accepted_tx: Option<TxId>,
    94	    pub retry_counter_for_current_task: u32,
    95	}
    96	
    97	// ────────────────────────────────────────────────────────────────────────────
    98	// AgentVisibleProjection — Inv 10 Goodhart shield (CO P2.7 visibility runtime).
    99	// ────────────────────────────────────────────────────────────────────────────
   100	
   101	/// TRACE_MATRIX § 1.1 — agent-visible projection of tape filtered by per-agent
   102	/// visibility policy (Inv 10 Goodhart shield; `top_white::predicates::visibility`).
   103	///
   104	/// `views`: per-agent filtered head pointer; full filtering machinery lands in CO P2.7.
   105	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   106	pub struct AgentVisibleProjection {
   107	    pub views: BTreeMap<AgentId, NodeId>,
   108	}
   109	
   110	// ────────────────────────────────────────────────────────────────────────────
   111	// BudgetSnapshot — global compute / cost / wall-clock budget.
   112	// ────────────────────────────────────────────────────────────────────────────
   113	
   114	/// TRACE_MATRIX § 1.1 — global budget snapshot:
   115	/// cost ceiling (MicroCoin), wall clock remaining (ms), compute cap remaining.
   116	/// Exhaustion → halt_reason ∈ {WallClockCap, ComputeCapViolated, MaxTxExhausted}.
   117	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   118	pub struct BudgetSnapshot {
   119	    pub cost_ceiling_microcoin: MicroCoin,
   120	    pub wall_clock_remaining_ms: u64,
   121	    pub compute_cap_remaining: u64,
   122	}
   123	
   124	impl Default for BudgetSnapshot {
   125	    fn default() -> Self {
   126	        Self {
   127	            cost_ceiling_microcoin: MicroCoin::zero(),
   128	            wall_clock_remaining_ms: 0,
   129	            compute_cap_remaining: 0,
   130	        }
   131	    }
   132	}
   133	
   134	// ────────────────────────────────────────────────────────────────────────────
   135	// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
   136	// ────────────────────────────────────────────────────────────────────────────
   137	
   138	/// TRACE_MATRIX WP § 2 economic — 9-sub-field economic state. Each sub-index
   139	/// is a BTreeMap newtype; entry shapes (Escrow / Stake / Claim / TaskMarket /
   140	/// RoyaltyEdge / ChallengeCase) are minimal-but-typed here and fully fleshed
   141	/// in the owning atoms (CO P2.1-2.6).
   142	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   143	pub struct EconomicState {
   144	    pub balances_t: BalancesIndex,
   145	    pub escrows_t: EscrowsIndex,
   146	    pub stakes_t: StakesIndex,
   147	    pub claims_t: ClaimsIndex,
   148	    pub reputations_t: ReputationsIndex,
   149	    pub task_markets_t: TaskMarketsIndex,
   150	    pub royalty_graph_t: RoyaltyGraph,
   151	    pub challenge_cases_t: ChallengeCasesIndex,
   152	    pub price_index_t: PriceIndex,
   153	}
   154	
   155	/// TRACE_MATRIX WP § 2 — agent → balance ledger. Concrete entry: `MicroCoin` (CO1.0a).
   156	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   157	pub struct BalancesIndex(pub BTreeMap<AgentId, MicroCoin>);
   158	
   159	/// TRACE_MATRIX WP § 2 — tx → escrow entry. Full schema lands CO P2.2 EscrowVault.
   160	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   161	pub struct EscrowsIndex(pub BTreeMap<TxId, EscrowEntry>);
   162	
   163	/// TRACE_MATRIX WP § 2 — escrow entry shape (stub). Full fields land CO P2.2.
   164	/// `#[serde(default)]` on each field gives forward-compat: future atoms can add
   165	/// fields without breaking deserialization of historical ledger rows.
   166	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   167	pub struct EscrowEntry {
   168	    #[serde(default = "MicroCoin::zero")]
   169	    pub amount: MicroCoin,
   170	    #[serde(default)]
   171	    pub depositor: AgentId,
   172	}
   173	
   174	impl Default for EscrowEntry {
   175	    fn default() -> Self {
   176	        Self { amount: MicroCoin::zero(), depositor: AgentId::default() }
   177	    }
   178	}
   179	
   180	/// TRACE_MATRIX WP § 2 — tx → stake entry. Full schema lands CO P2.5 ChallengeCourt.
   181	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   182	pub struct StakesIndex(pub BTreeMap<TxId, StakeEntry>);
   183	
   184	/// TRACE_MATRIX WP § 2 — stake entry shape (stub). Full fields land CO P2.5.
   185	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   186	pub struct StakeEntry {
   187	    #[serde(default = "MicroCoin::zero")]
   188	    pub amount: MicroCoin,
   189	    #[serde(default)]
   190	    pub staker: AgentId,
   191	}
   192	
   193	impl Default for StakeEntry {
   194	    fn default() -> Self {
   195	        Self { amount: MicroCoin::zero(), staker: AgentId::default() }
   196	    }
   197	}
   198	
   199	/// TRACE_MATRIX WP § 2 — tx → reward claim. Full schema lands CO P2.6 SettlementEngine.
   200	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   201	pub struct ClaimsIndex(pub BTreeMap<TxId, ClaimEntry>);
   202	
   203	/// TRACE_MATRIX WP § 2 — claim entry shape (stub). Full fields land CO P2.6.
   204	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   205	pub struct ClaimEntry {
   206	    #[serde(default = "MicroCoin::zero")]
   207	    pub amount: MicroCoin,
   208	    #[serde(default)]
   209	    pub claimant: AgentId,
   210	}
   211	
   212	impl Default for ClaimEntry {
   213	    fn default() -> Self {
   214	        Self { amount: MicroCoin::zero(), claimant: AgentId::default() }
   215	    }
   216	}
   217	
   218	/// TRACE_MATRIX WP § 2 — agent → reputation ledger.
   219	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   220	pub struct ReputationsIndex(pub BTreeMap<AgentId, Reputation>);
   221	
   222	/// TRACE_MATRIX WP § 2 — tx → task market. Full schema lands CO P2.1.
   223	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   224	pub struct TaskMarketsIndex(pub BTreeMap<TxId, TaskMarketEntry>);
   225	
   226	/// TRACE_MATRIX WP § 2 — task market entry shape (stub). Full fields land CO P2.1.
   227	/// Default values (verifier_quorum=1, max_reuse_royalty_fraction=0.10) match the
   228	/// PROJECT_DECISION_MAP § 2.3 spec gap defaults.
   229	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   230	pub struct TaskMarketEntry {
   231	    #[serde(default)]
   232	    pub publisher: AgentId,
   233	    #[serde(default = "MicroCoin::zero")]
   234	    pub bounty: MicroCoin,
   235	    #[serde(default = "task_market_default_quorum")]
   236	    pub verifier_quorum: u32,
   237	    #[serde(default = "task_market_default_royalty_bp")]
   238	    pub max_reuse_royalty_fraction_basis_points: u16,
   239	}
   240	
   241	fn task_market_default_quorum() -> u32 {
   242	    1
   243	}
   244	fn task_market_default_royalty_bp() -> u16 {
   245	    1000
   246	}
   247	
   248	impl Default for TaskMarketEntry {
   249	    fn default() -> Self {
   250	        Self {
   251	            publisher: AgentId::default(),
   252	            bounty: MicroCoin::zero(),
   253	            verifier_quorum: 1,
   254	            max_reuse_royalty_fraction_basis_points: 1000, // 0.10 per spec gap default
   255	        }
   256	    }
   257	}
   258	
   259	/// TRACE_MATRIX WP § 2 — directed royalty edges (reuse depth attribution).
   260	/// Full attribution algebra lands CO P2.4.

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '520,820p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   520	            agent_id: self.agent_id.clone(),
   521	            read_set: self.read_set.clone(),
   522	            write_set: self.write_set.clone(),
   523	            proposal_cid: self.proposal_cid,
   524	            predicate_results: self.predicate_results.clone(),
   525	            stake: self.stake,
   526	            timestamp_logical: self.timestamp_logical,
   527	        }
   528	    }
   529	}
   530	
   531	impl VerifyTx {
   532	    pub fn to_signing_payload(&self) -> VerifySigningPayload {
   533	        VerifySigningPayload {
   534	            tx_id: self.tx_id.clone(),
   535	            target_work_tx: self.target_work_tx.clone(),
   536	            verifier_agent: self.verifier_agent.clone(),
   537	            bond: self.bond,
   538	            verdict: self.verdict,
   539	            timestamp_logical: self.timestamp_logical,
   540	        }
   541	    }
   542	}
   543	
   544	impl ChallengeTx {
   545	    pub fn to_signing_payload(&self) -> ChallengeSigningPayload {
   546	        ChallengeSigningPayload {
   547	            tx_id: self.tx_id.clone(),
   548	            target_work_tx: self.target_work_tx.clone(),
   549	            challenger_agent: self.challenger_agent.clone(),
   550	            stake: self.stake,
   551	            counterexample_cid: self.counterexample_cid,
   552	            timestamp_logical: self.timestamp_logical,
   553	        }
   554	    }
   555	}
   556	
   557	impl FinalizeRewardTx {
   558	    pub fn to_signing_payload(&self) -> FinalizeRewardSigningPayload {
   559	        FinalizeRewardSigningPayload {
   560	            tx_id: self.tx_id.clone(),
   561	            claim_id: self.claim_id.clone(),
   562	            task_id: self.task_id.clone(),
   563	            solver: self.solver.clone(),
   564	            reward: self.reward,
   565	            parent_state_root: self.parent_state_root,
   566	            epoch: self.epoch,
   567	            timestamp_logical: self.timestamp_logical,
   568	        }
   569	    }
   570	}
   571	
   572	impl TaskExpireTx {
   573	    pub fn to_signing_payload(&self) -> TaskExpireSigningPayload {
   574	        TaskExpireSigningPayload {
   575	            tx_id: self.tx_id.clone(),
   576	            task_id: self.task_id.clone(),
   577	            parent_state_root: self.parent_state_root,
   578	            bounty_refunded: self.bounty_refunded,
   579	            epoch: self.epoch,
   580	            timestamp_logical: self.timestamp_logical,
   581	        }
   582	    }
   583	}
   584	
   585	impl TerminalSummaryTx {
   586	    pub fn to_signing_payload(&self) -> TerminalSummarySigningPayload {
   587	        TerminalSummarySigningPayload {
   588	            tx_id: self.tx_id.clone(),
   589	            task_id: self.task_id.clone(),
   590	            run_id: self.run_id.clone(),
   591	            run_outcome: self.run_outcome,
   592	            total_attempts: self.total_attempts,
   593	            failure_class_histogram: self.failure_class_histogram.clone(),
   594	            last_logical_t: self.last_logical_t,
   595	        }
   596	    }
   597	}
   598	
   599	// ────────────────────────────────────────────────────────────────────────────
   600	// § 6 TypedTx outer enum
   601	// ────────────────────────────────────────────────────────────────────────────
   602	
   603	/// TRACE_MATRIX § 8 dispatch_transition — typed-tx outer enum.
   604	/// 7 variants (K5 closed: NO `Slash`). All variants are defined in this
   605	/// module (`state::typed_tx`); v1.1 P3 migrated `TerminalSummaryTx` here
   606	/// from a 3-field placeholder previously in `system_keypair.rs`.
   607	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   608	pub enum TypedTx {
   609	    Work(WorkTx),
   610	    Verify(VerifyTx),
   611	    Challenge(ChallengeTx),
   612	    Reuse(ReuseTx),
   613	    FinalizeReward(FinalizeRewardTx),
   614	    TaskExpire(TaskExpireTx),
   615	    TerminalSummary(TerminalSummaryTx),
   616	}
   617	
   618	impl TypedTx {
   619	    /// Project to the [`TxKind`] discriminator stored in `LedgerEntry.tx_kind`.
   620	    pub fn tx_kind(&self) -> crate::bottom_white::ledger::transition_ledger::TxKind {
   621	        use crate::bottom_white::ledger::transition_ledger::TxKind;
   622	        match self {
   623	            Self::Work(_) => TxKind::Work,
   624	            Self::Verify(_) => TxKind::Verify,
   625	            Self::Challenge(_) => TxKind::Challenge,
   626	            Self::Reuse(_) => TxKind::Reuse,
   627	            Self::FinalizeReward(_) => TxKind::FinalizeReward,
   628	            Self::TaskExpire(_) => TxKind::TaskExpire,
   629	            Self::TerminalSummary(_) => TxKind::TerminalSummary,
   630	        }
   631	    }
   632	}
   633	
   634	// ────────────────────────────────────────────────────────────────────────────
   635	// § 8 HasSubmitter trait (STATE spec § 3.6.5 v1.3)
   636	// ────────────────────────────────────────────────────────────────────────────
   637	
   638	/// TRACE_MATRIX STATE spec § 3.6.5 v1.3 — submitter resolution trait used
   639	/// by the implicit-init step in agent-submitted transitions. System-emitted
   640	/// transitions return `None` (no agent to init).
   641	pub trait HasSubmitter {
   642	    fn submitter_id(&self) -> Option<AgentId>;
   643	}
   644	
   645	impl HasSubmitter for WorkTx {
   646	    fn submitter_id(&self) -> Option<AgentId> {
   647	        Some(self.agent_id.clone())
   648	    }
   649	}
   650	
   651	impl HasSubmitter for VerifyTx {
   652	    fn submitter_id(&self) -> Option<AgentId> {
   653	        Some(self.verifier_agent.clone())
   654	    }
   655	}
   656	
   657	impl HasSubmitter for ChallengeTx {
   658	    fn submitter_id(&self) -> Option<AgentId> {
   659	        Some(self.challenger_agent.clone())
   660	    }
   661	}
   662	
   663	impl HasSubmitter for ReuseTx {
   664	    fn submitter_id(&self) -> Option<AgentId> {
   665	        None
   666	    }
   667	}
   668	
   669	impl HasSubmitter for FinalizeRewardTx {
   670	    fn submitter_id(&self) -> Option<AgentId> {
   671	        None
   672	    }
   673	}
   674	
   675	impl HasSubmitter for TaskExpireTx {
   676	    fn submitter_id(&self) -> Option<AgentId> {
   677	        None
   678	    }
   679	}
   680	
   681	impl HasSubmitter for TerminalSummaryTx {
   682	    fn submitter_id(&self) -> Option<AgentId> {
   683	        None
   684	    }
   685	}
   686	
   687	impl HasSubmitter for TypedTx {
   688	    fn submitter_id(&self) -> Option<AgentId> {
   689	        match self {
   690	            Self::Work(t) => t.submitter_id(),
   691	            Self::Verify(t) => t.submitter_id(),
   692	            Self::Challenge(t) => t.submitter_id(),
   693	            Self::Reuse(t) => t.submitter_id(),
   694	            Self::FinalizeReward(t) => t.submitter_id(),
   695	            Self::TaskExpire(t) => t.submitter_id(),
   696	            Self::TerminalSummary(t) => t.submitter_id(),
   697	        }
   698	    }
   699	}
   700	
   701	// ────────────────────────────────────────────────────────────────────────────
   702	// TransitionError — minimal v1 taxonomy (CO1.1.4-pre1 spec § 0 out-of-scope
   703	// note: full per-stage enum proliferation is CO1.7.5)
   704	// ────────────────────────────────────────────────────────────────────────────
   705	
   706	/// TRACE_MATRIX STATE § 3 — transition-function error taxonomy. v1.1 covers
   707	/// every variant invoked in STATE_TRANSITION_SPEC § 3.1-3.7 pseudocode +
   708	/// `NotYetImplemented` for CO1.7.5 stub bodies (per Codex Q-G CHALLENGE).
   709	///
   710	/// **Why payloads are minimal**: the failed `PredicateId` (etc.) is a string
   711	/// reference; richer context (PredicateResultsBundle, Cid of failed proof)
   712	/// is attached by the runtime via separate book-keeping channels (rejected
   713	/// summary stamping, bus rejection log). Keeping TransitionError serializable
   714	/// with primitive payloads avoids forcing PredicateResultsBundle through
   715	/// every error site.
   716	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   717	pub enum TransitionError {
   718	    // ── Stale-parent & signature ───────────────────────────────────────────
   719	    /// `parent_state_root` does not match `q.state_root_t` (any agent tx).
   720	    StaleParent,
   721	    /// Agent signature verify failed (work / verify / challenge tx).
   722	    SignatureInvalid,
   723	    /// System-keypair signature verify failed (system-emitted tx).
   724	    InvalidSystemSignature,
   725	
   726	    // ── Economy ────────────────────────────────────────────────────────────
   727	    /// Submitter's available balance is below the declared stake / bond.
   728	    /// Payload-rich variant (available + required) is intentionally elided
   729	    /// in v1.1 to keep this enum primitive-payloads-only; runtime attaches
   730	    /// context via the rejection log (per STATE § 1.4 RejectedAttemptSummary).
   731	    StakeInsufficient,
   732	
   733	    // ── Target lookup ──────────────────────────────────────────────────────
   734	    /// VerifyTx / ChallengeTx / ReuseTx target work_tx not found in L4.
   735	    TargetWorkTxNotFound,
   736	    /// VerifyTx target is not in a verifiable status (e.g. already finalized).
   737	    TargetWorkTxNotVerifiable,
   738	    /// ReuseTx target work_tx exists but is not yet Accepted (parent must accept first).
   739	    ParentNotAcceptedYet,
   740	
   741	    // ── Predicate failures ─────────────────────────────────────────────────
   742	    /// step_transition stage 4 — acceptance predicate denied. `PredicateId`
   743	    /// is the public predicate that failed; private predicates surface as
   744	    /// `RejectionClass::Opaque` in book-keeping (NOT here).
   745	    AcceptancePredicateFailed(PredicateId),
   746	    /// verify_transition stage 4 — verification predicate denied.
   747	    VerificationPredicateFailed(PredicateId),
   748	    /// finalize_reward / step_transition stage 5 — settlement predicate denied.
   749	    SettlementPredicateFailed(PredicateId),
   750	
   751	    // ── Challenge ──────────────────────────────────────────────────────────
   752	    /// challenge_transition stage 1 — challenge filed after window closed.
   753	    ChallengeWindowClosed,
   754	    /// finalize_reward stage 1 — challenge window still open; cannot finalize.
   755	    ChallengeWindowStillOpen,
   756	    /// finalize_reward stage 1 — claim already slashed; cannot also reward.
   757	    AlreadySlashed,
   758	    /// challenge_transition stage 4 — counterexample failed predicate check.
   759	    CounterexampleInsufficient,
   760	
   761	    // ── Reuse ──────────────────────────────────────────────────────────────
   762	    /// reuse_transition stage 1 — referenced tool not in L2 ToolRegistry.
   763	    ToolNotInRegistry,
   764	    /// reuse_transition stage 1 — declared tool creator does not match registry.
   765	    ToolCreatorMismatch,
   766	
   767	    // ── Finalize ───────────────────────────────────────────────────────────
   768	    /// finalize_reward — no claim entry for the given claim_id.
   769	    ClaimNotFound,
   770	
   771	    // ── Task expire ────────────────────────────────────────────────────────
   772	    /// task_expire — referenced TaskMarket entry not found.
   773	    TaskNotFound,
   774	    /// task_expire — deadline not yet reached.
   775	    TaskNotExpired,
   776	    /// task_expire — at least one open claim exists; cannot refund bounty.
   777	    TaskHasOpenClaim,
   778	
   779	    // ── Terminal summary ───────────────────────────────────────────────────
   780	    /// emit_terminal_summary — run already has an accepted work_tx.
   781	    TerminalSummaryNotApplicable,
   782	
   783	    // ── Stub sentinel (CO1.7.5 fills) ──────────────────────────────────────
   784	    /// Stub return value used by CO1.7.5 unimplemented bodies — preserves
   785	    /// sequencer + dispatch correctness without forcing transition logic
   786	    /// into this atom. Audit input: this is intentional, not a code smell.
   787	    NotYetImplemented,
   788	}
   789	
   790	impl std::fmt::Display for TransitionError {
   791	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   792	        match self {
   793	            Self::StaleParent => write!(f, "stale parent_state_root"),
   794	            Self::SignatureInvalid => write!(f, "agent signature invalid"),
   795	            Self::InvalidSystemSignature => write!(f, "invalid system signature"),
   796	            Self::StakeInsufficient => write!(f, "stake / bond insufficient"),
   797	            Self::TargetWorkTxNotFound => write!(f, "target work_tx not found"),
   798	            Self::TargetWorkTxNotVerifiable => write!(f, "target work_tx not in a verifiable state"),
   799	            Self::ParentNotAcceptedYet => write!(f, "parent work_tx not yet accepted"),
   800	            Self::AcceptancePredicateFailed(p) => write!(f, "acceptance predicate failed: {p:?}"),
   801	            Self::VerificationPredicateFailed(p) => write!(f, "verification predicate failed: {p:?}"),
   802	            Self::SettlementPredicateFailed(p) => write!(f, "settlement predicate failed: {p:?}"),
   803	            Self::ChallengeWindowClosed => write!(f, "challenge window closed"),
   804	            Self::ChallengeWindowStillOpen => write!(f, "challenge window still open"),
   805	            Self::AlreadySlashed => write!(f, "already slashed"),
   806	            Self::CounterexampleInsufficient => write!(f, "counterexample insufficient"),
   807	            Self::ToolNotInRegistry => write!(f, "reuse tool not in registry"),
   808	            Self::ToolCreatorMismatch => write!(f, "reuse tool creator mismatch"),
   809	            Self::ClaimNotFound => write!(f, "claim not found"),
   810	            Self::TaskNotFound => write!(f, "task not found"),
   811	            Self::TaskNotExpired => write!(f, "task deadline not yet reached"),
   812	            Self::TaskHasOpenClaim => write!(f, "task has at least one open claim"),
   813	            Self::TerminalSummaryNotApplicable => write!(f, "terminal summary not applicable"),
   814	            Self::NotYetImplemented => write!(f, "transition body not yet implemented (CO1.7.5)"),
   815	        }
   816	    }
   817	}
   818	impl std::error::Error for TransitionError {}
   819	
   820	// ────────────────────────────────────────────────────────────────────────────

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '260,380p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   260	/// Full attribution algebra lands CO P2.4.
   261	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   262	pub struct RoyaltyGraph(pub BTreeMap<TxId, Vec<RoyaltyEdge>>);
   263	
   264	/// TRACE_MATRIX WP § 2 — single royalty edge (ancestor → reuse weight). Stub; CO P2.4.
   265	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   266	pub struct RoyaltyEdge {
   267	    #[serde(default)]
   268	    pub ancestor: TxId,
   269	    #[serde(default)]
   270	    pub fraction_basis_points: u16,
   271	}
   272	
   273	/// TRACE_MATRIX WP § 2 — tx → challenge case. Full schema lands CO P2.5.
   274	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   275	pub struct ChallengeCasesIndex(pub BTreeMap<TxId, ChallengeCase>);
   276	
   277	/// TRACE_MATRIX WP § 2 — challenge case shape (stub). Full fields land CO P2.5.
   278	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   279	pub struct ChallengeCase {
   280	    #[serde(default)]
   281	    pub challenger: AgentId,
   282	    #[serde(default = "MicroCoin::zero")]
   283	    pub bond: MicroCoin,
   284	    #[serde(default)]
   285	    pub opened_at_round: u64,
   286	}
   287	
   288	impl Default for ChallengeCase {
   289	    fn default() -> Self {
   290	        Self { challenger: AgentId::default(), bond: MicroCoin::zero(), opened_at_round: 0 }
   291	    }
   292	}
   293	
   294	/// TRACE_MATRIX WP § 2 — tx → posted price (last accepted price index).
   295	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   296	pub struct PriceIndex(pub BTreeMap<TxId, MicroCoin>);
   297	
   298	// ────────────────────────────────────────────────────────────────────────────
   299	// QState — § 1.1 verbatim, 9 fields.
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
   319	    pub predicate_registry_root_t: Hash,
   320	    /// L2 Tool Registry root.
   321	    pub tool_registry_root_t: Hash,
   322	    /// Economic state (WP § 2 amendment, 9 sub-fields).
   323	    pub economic_state_t: EconomicState,
   324	    /// Global budget snapshot.
   325	    pub budget_state_t: BudgetSnapshot,
   326	}
   327	
   328	impl QState {
   329	    /// TRACE_MATRIX Art IV Boot — genesis Q_t. All zero / empty;
   330	    /// roots populated by `boot::verify_trust_root` and the `state_root_t` published
   331	    /// in `genesis_payload.toml [constitution_root]`.
   332	    pub fn genesis() -> Self {
   333	        QState::default()
   334	    }
   335	}
   336	
   337	// ────────────────────────────────────────────────────────────────────────────
   338	// Inline determinism tests (round-trip + insertion-order independence).
   339	// Conformance tests proper live in tests/{four_element_mapping, q_state_reconstruct,
   340	// economic_state_reconstruct, six_axioms_alignment}.rs per TRACE_MATRIX_v3.
   341	// ────────────────────────────────────────────────────────────────────────────
   342	
   343	#[cfg(test)]
   344	mod tests {
   345	    use super::*;
   346	
   347	    #[test]
   348	    fn genesis_q_state_is_total_and_default() {
   349	        let g = QState::genesis();
   350	        assert_eq!(g, QState::default());
   351	        assert_eq!(g.q_t.current_round, 0);
   352	        assert!(g.q_t.agents.is_empty());
   353	        assert_eq!(g.head_t, NodeId::default());
   354	        assert_eq!(g.state_root_t, Hash::ZERO);
   355	    }
   356	
   357	    #[test]
   358	    fn nine_field_count_via_serde_json() {
   359	        // Sanity that QState has exactly 9 top-level fields.
   360	        let s = serde_json::to_value(QState::genesis()).unwrap();
   361	        let obj = s.as_object().expect("object");
   362	        assert_eq!(
   363	            obj.len(),
   364	            9,
   365	            "QState must have exactly 9 fields per WP § 4; got {}",
   366	            obj.len()
   367	        );
   368	        for k in &[
   369	            "q_t",
   370	            "head_t",
   371	            "state_root_t",
   372	            "tape_view_t",
   373	            "ledger_root_t",
   374	            "predicate_registry_root_t",
   375	            "tool_registry_root_t",
   376	            "economic_state_t",
   377	            "budget_state_t",
   378	        ] {
   379	            assert!(obj.contains_key(*k), "QState missing field {}", k);
   380	        }

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/schema.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! CAS object schema per WP architecture § 5.L3.
     2	//!
     3	//! /// TRACE_MATRIX WP-arch-§5.L3: CAS object schema
     4	
     5	use serde::{Deserialize, Serialize};
     6	use sha2::{Digest, Sha256};
     7	
     8	/// Content-addressed identifier — sha256 of payload bytes.
     9	///
    10	/// Distinct from git's SHA-1 OID (which is an internal storage detail of
    11	/// the git2-rs backend). `Cid` is the v4-canonical identifier; spec § 1.2
    12	/// `WorkTx.proposal_cid: Cid` references this.
    13	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    14	pub struct Cid(pub [u8; 32]);
    15	
    16	impl Cid {
    17	    /// Compute Cid from content bytes.
    18	    pub fn from_content(content: &[u8]) -> Self {
    19	        let mut h = Sha256::new();
    20	        h.update(content);
    21	        Self(h.finalize().into())
    22	    }
    23	
    24	    /// Hex-encoded representation (lowercase; 64 chars).
    25	    pub fn hex(&self) -> String {
    26	        let mut s = String::with_capacity(64);
    27	        for b in self.0 {
    28	            s.push_str(&format!("{:02x}", b));
    29	        }
    30	        s
    31	    }
    32	}
    33	
    34	impl std::fmt::Display for Cid {
    35	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    36	        write!(f, "cid:{}", self.hex())
    37	    }
    38	}
    39	
    40	/// Type tag for CAS objects (replaces inline string-typed kind).
    41	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    42	pub enum ObjectType {
    43	    /// Agent's work_tx proposal payload (Lean proof, code patch, etc.).
    44	    ProposalPayload,
    45	    /// Challenger's counterexample for a slashed claim.
    46	    CounterexamplePayload,
    47	    /// Predicate bytecode (Lean tactic, WASM module, Rust source bytes).
    48	    PredicateBytecode,
    49	    /// Tool bytecode.
    50	    ToolBytecode,
    51	    /// Constitution diff (for amendment proposals).
    52	    AmendmentDiff,
    53	    /// Reversibility plan attached to a meta_tx.
    54	    ReversibilityPlan,
    55	    /// Generic / unclassified blob.
    56	    Generic,
    57	}
    58	
    59	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    60	pub struct CasObjectMetadata {
    61	    /// Content-addressed identifier (sha256 of content).
    62	    pub cid: Cid,
    63	    /// Backend-specific OID (git sha-1 for git2-rs backend); informational only.
    64	    /// Different backends may have different OID schemes; Cid is canonical.
    65	    pub backend_oid_hex: String,
    66	    pub object_type: ObjectType,
    67	    /// Submitter / author. Use "system" for runtime-emitted objects.
    68	    pub creator: String,
    69	    /// Logical time at insertion (assigned by sequencer; not wall clock).
    70	    pub created_at_logical_t: u64,
    71	    /// Optional schema identifier (JSON Schema URI, type tag, etc.).
    72	    pub schema_id: Option<String>,
    73	    /// Size of content in bytes (informational; not part of canonical hash).
    74	    pub size_bytes: u64,
    75	}
    76	
    77	impl CasObjectMetadata {
    78	    /// Canonical hash of metadata for Merkle tree inclusion.
    79	    pub fn canonical_hash(&self) -> [u8; 32] {
    80	        let mut h = Sha256::new();
    81	        h.update(self.cid.0);
    82	        h.update(self.backend_oid_hex.as_bytes());
    83	        h.update(serde_json::to_vec(&self.object_type).expect("object_type serialize"));
    84	        h.update(self.creator.as_bytes());
    85	        h.update(self.created_at_logical_t.to_be_bytes());
    86	        if let Some(s) = &self.schema_id {
    87	            h.update(b"\x01");
    88	            h.update(s.as_bytes());
    89	        } else {
    90	            h.update(b"\x00");
    91	        }
    92	        h.update(self.size_bytes.to_be_bytes());
    93	        h.finalize().into()
    94	    }
    95	}
    96	
    97	#[cfg(test)]
    98	mod tests {
    99	    use super::*;
   100	
   101	    #[test]
   102	    fn cid_from_empty_content() {
   103	        let cid = Cid::from_content(b"");
   104	        // SHA-256 of empty input = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
   105	        assert_eq!(
   106	            cid.hex(),
   107	            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
   108	        );
   109	    }
   110	
   111	    #[test]
   112	    fn cid_deterministic() {
   113	        let cid_a = Cid::from_content(b"hello");
   114	        let cid_b = Cid::from_content(b"hello");
   115	        assert_eq!(cid_a, cid_b);
   116	    }
   117	
   118	    #[test]
   119	    fn cid_differs_on_content() {
   120	        let cid_a = Cid::from_content(b"hello");
   121	        let cid_b = Cid::from_content(b"world");
   122	        assert_ne!(cid_a, cid_b);
   123	    }
   124	
   125	    #[test]
   126	    fn cid_display_format() {
   127	        let cid = Cid::from_content(b"x");
   128	        let s = cid.to_string();
   129	        assert!(s.starts_with("cid:"));
   130	        assert_eq!(s.len(), 4 + 64);
   131	    }
   132	
   133	    #[test]
   134	    fn metadata_canonical_hash_deterministic() {
   135	        let m = CasObjectMetadata {
   136	            cid: Cid::from_content(b"x"),
   137	            backend_oid_hex: "abc123".to_string(),
   138	            object_type: ObjectType::ProposalPayload,
   139	            creator: "alice".to_string(),
   140	            created_at_logical_t: 100,
   141	            schema_id: Some("v1/proposal".to_string()),
   142	            size_bytes: 1,
   143	        };
   144	        assert_eq!(m.canonical_hash(), m.canonical_hash());
   145	    }
   146	
   147	    #[test]
   148	    fn metadata_canonical_hash_differs_on_object_type() {
   149	        let base = CasObjectMetadata {
   150	            cid: Cid::from_content(b"x"),
   151	            backend_oid_hex: "abc".to_string(),
   152	            object_type: ObjectType::ProposalPayload,
   153	            creator: "alice".to_string(),
   154	            created_at_logical_t: 100,
   155	            schema_id: None,
   156	            size_bytes: 1,
   157	        };
   158	        let mut variant = base.clone();
   159	        variant.object_type = ObjectType::CounterexamplePayload;
   160	        assert_ne!(base.canonical_hash(), variant.canonical_hash());
   161	    }
   162	}

codex
The first code pass found the core implementation paths and a few places that deserve adversarial testing: commit determinism, replay error precision, and sequencer atomicity. I’m running the relevant lib tests now so the verdict doesn’t rely on the claimed `237/0` alone.
exec
/bin/bash -lc 'cargo test --lib' in /home/zephryj/projects/turingosv4
 succeeded in 1638ms:
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

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:627:19
    |
627 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib test) generated 16 warnings (run `cargo fix --lib -p turingosv4 --tests` to apply 11 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-337b37021c2192ab)

running 238 tests
test boot::tests::parse_strips_inline_comment_and_blanks ... ok
test boot::tests::parse_errors_when_section_missing ... ok
test boot::tests::parse_errors_on_unquoted_key ... ok
test boot::tests::verify_trust_root_detects_tamper_in_tempdir ... ok
test boot::tests::verify_trust_root_detects_child_manifest_tamper ... ok
test bottom_white::cas::schema::tests::cid_deterministic ... ok
test boot::tests::verify_trust_root_passes_when_hash_matches_in_tempdir ... ok
test bottom_white::cas::schema::tests::cid_display_format ... ok
test boot::tests::verify_trust_root_passes_with_matching_child_manifest ... ok
test bottom_white::cas::schema::tests::cid_differs_on_content ... ok
test bottom_white::cas::schema::tests::metadata_canonical_hash_deterministic ... ok
test bottom_white::cas::schema::tests::cid_from_empty_content ... ok
test bottom_white::cas::schema::tests::metadata_canonical_hash_differs_on_object_type ... ok
test bottom_white::cas::store::tests::cid_is_content_address ... ok
test bottom_white::cas::store::tests::corrupted_sidecar_line_returns_parse_error ... ok
test bottom_white::cas::store::tests::cell_isolation_disjoint_cas ... ok
test bottom_white::cas::store::tests::empty_store_root ... ok
test bottom_white::cas::store::tests::get_nonexistent_returns_error ... ok
test bottom_white::cas::store::tests::idempotent_put_does_not_duplicate_sidecar_line ... ok
test bottom_white::cas::store::tests::each_new_put_appends_one_line ... ok
test bottom_white::cas::store::tests::missing_sidecar_opens_fresh ... ok
test bottom_white::cas::store::tests::metadata_recorded ... ok
test bottom_white::cas::store::tests::merkle_root_deterministic_two_runs ... ok
test bottom_white::cas::store::tests::put_get_round_trip_small ... ok
test bottom_white::cas::store::tests::put_idempotent_same_content ... ok
test bottom_white::cas::store::tests::put_get_round_trip_large ... ok
test bottom_white::ledger::system_keypair::tests::authorized_scope_signing_round_trip ... ok
test bottom_white::cas::store::tests::reopen_recovers_index_and_get_works ... ok
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
test boot::tests::verify_trust_root_passes_on_intact_repo ... ok
test bottom_white::ledger::transition_ledger::tests::replay_cas_payload_round_trip_after_reopen ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::sequencer_serial_replay_byte_identity ... ignored, CO1.7.5: requires real per-kind transition bodies
test bottom_white::ledger::transition_ledger::tests::replay_full_transition_reaches_dispatch_then_stubs ... ok
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
test bottom_white::ledger::transition_ledger::tests::replay_rejects_bad_system_signature ... ok
test economy::money::tests::serde_transparent_format ... ok
test kernel::tests::test_append_and_retrieve ... ok
test kernel::tests::test_golden_path_trace ... ok
test kernel::tests::test_market_lifecycle ... ok
test kernel::tests::test_market_ticker ... ok
test kernel::tests::test_no_duplicate_market ... ok
test kernel::tests::test_no_market_for_nonexistent_node ... ok
test kernel::tests::test_reject_dangling_citation ... ok
test kernel::tests::test_resolve_all_markets ... ok
test kernel::tests::test_reject_duplicate ... ok
test ledger::tests::test_ledger_hash_chain_integrity ... ok
test ledger::tests::test_ledger_append_and_verify ... ok
test ledger::tests::test_ledger_omega_vocabulary ... ok
test ledger::tests::test_ledger_sequence_monotonic ... ok
test ledger::tests::test_tape_append_root_node ... ok
test ledger::tests::test_ledger_tamper_detection ... ok
test ledger::tests::test_tape_dag_branching ... ok
test ledger::tests::test_tape_empty ... ok
test ledger::tests::test_tape_reject_dangling_citation ... ok
test ledger::tests::test_tape_reject_duplicate_id ... ok
test ledger::tests::test_tape_time_arrow_ordering ... ok
test ledger::tests::test_tape_trace_ancestors ... ok
test prediction_market::tests::test_assassin_profit ... ok
test prediction_market::tests::test_buy_no_increases_no_price ... ok
test prediction_market::tests::test_buy_yes_increases_yes_price ... ok
test prediction_market::tests::test_constant_product_invariant ... ok
test ledger::tests::test_tape_append_with_valid_citation ... ok
test prediction_market::tests::test_create_market ... ok
test prediction_market::tests::test_ctf_conservation_1_coin_1_yes_1_no ... ok
test prediction_market::tests::test_initial_price_is_50_50 ... ok
test prediction_market::tests::test_no_double_resolution ... ok
test prediction_market::tests::test_multiple_traders_price_discovery ... ok
test prediction_market::tests::test_pioneer_profit ... ok
test prediction_market::tests::test_no_trading_after_resolution ... ok
test prediction_market::tests::test_redeem_requires_resolution ... ok
test prediction_market::tests::test_prices_sum_to_one ... ok
test prediction_market::tests::test_reject_zero_or_negative_amounts ... ok
test sdk::actor::tests::test_boltzmann_never_returns_none_with_nodes ... ok
test sdk::actor::tests::test_boltzmann_returns_none_empty_tape ... ok
test sdk::actor::tests::test_frontier_detection_leaf ... ok
test sdk::actor::tests::test_frontier_detection_parent_with_child ... ok
test sdk::actor::tests::test_lineage_score_increases_with_depth ... ok
test sdk::error_abstraction::tests::classifier_version_is_stamped ... ok
test sdk::error_abstraction::tests::fixture_linarith_failed ... ok
test sdk::error_abstraction::tests::fixture_other_catchall ... ok
test sdk::error_abstraction::tests::fixture_rewrite_no_match ... ok
test sdk::error_abstraction::tests::fixture_simp_no_progress ... ok
test sdk::error_abstraction::tests::fixture_type_mismatch ... ok
test sdk::error_abstraction::tests::fixture_unexpected_token ... ok
test sdk::error_abstraction::tests::fixture_unknown_constant ... ok
test sdk::error_abstraction::tests::fixture_unsolved_goals ... ok
test sdk::error_abstraction::tests::labels_are_unique_and_stable ... ok
test sdk::prompt::tests::test_prompt_contains_no_example_values ... ok
test sdk::prompt::tests::test_prompt_includes_balance ... ok
test sdk::actor::tests::test_boltzmann_diversity_not_deterministic ... ok
test sdk::prompt::tests::test_prompt_surfaces_search_hits ... ok
test sdk::prompt::tests::test_prompt_truncates_errors_to_3 ... ok
test sdk::prompt_guard::tests::test_case_insensitive_match - should panic ... ok
test sdk::prompt_guard::tests::test_clean_prompt_passes ... ok
test sdk::prompt_guard::tests::test_empty_prompt_passes ... ok
test sdk::prompt::tests::test_prompt_surfaces_team_board ... ok
test sdk::prompt_guard::tests::test_h_vpput_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_assignment_pattern_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_m_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_runtime_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_substring_in_larger_text - should panic ... ok
test sdk::prompt_guard::tests::test_wbcg_caught - should panic ... ok
test sdk::protocol::tests::test_deduct_negative_amount_rejected ... ok
test sdk::protocol::tests::test_malformed_action_tag_rejected_not_fallback ... ok
test sdk::protocol::tests::test_parse_action_tag_valid ... ok
test sdk::protocol::tests::test_no_byte_repair_on_invalid_escape ... ok
test sdk::protocol::tests::test_parse_bare_json_fallback ... ok
test sdk::protocol::tests::test_parse_action_tag_with_think_block ... ok
test sdk::protocol::tests::test_parse_no_action_returns_error ... ok
test sdk::protocol::tests::test_parse_with_invest_action ... ok
test sdk::protocol::tests::test_parse_invalid_json_returns_error ... ok
test sdk::protocol::tests::test_strip_multiple_think_blocks ... ok
test sdk::protocol::tests::test_strip_think_blocks ... ok
test sdk::protocol::tests::test_strip_unclosed_think_block ... ok
test bottom_white::ledger::transition_ledger::tests::signature_round_trip_and_transplant_defense ... ok
test sdk::sandbox::tests::test_sandbox_echo_command ... ok
test sdk::sandbox::tests::test_sandbox_captures_stderr ... ok
test sdk::snapshot::tests::test_snapshot_balance_query ... ok
test sdk::tools::librarian::tests::test_board_post_append ... ok
test sdk::tools::librarian::tests::test_board_write_read_roundtrip ... ok
test sdk::tools::librarian::tests::test_build_compression_prompt ... ok
test sdk::tools::librarian::tests::test_compress_interval ... ok
test sdk::tools::librarian::tests::test_zero_interval_never_compresses ... ok
test sdk::tools::search::tests::test_sanitize_query ... ok
test sdk::tools::search::tests::test_search_empty_query ... ok
test sdk::tools::search::tests::test_search_nonexistent_path ... ok
test sdk::tools::wallet::tests::test_append_is_free ... ok
test sdk::tools::wallet::tests::test_deduct_and_credit ... ok
test sdk::tools::wallet::tests::test_genesis_allocation ... ok
test sdk::tools::wallet::tests::test_insufficient_balance_rejected ... ok
test sdk::tools::wallet::tests::test_negative_deduct_rejected ... ok
test sdk::tools::wallet::tests::test_no_double_genesis ... ok
test sdk::tools::wallet::tests::test_portfolio_tracking ... ok
test sdk::tools::wallet::tests::test_query_balance ... ok
test sdk::tools::wallet::tests::test_query_unknown_key ... ok
test sdk::tools::wallet::tests::test_unknown_agent_vetoed ... ok
test sdk::tools::wallet::tests::test_zero_deduct_rejected ... ok
test state::q_state::tests::btreemap_insertion_order_independent_serialization ... ok
test state::q_state::tests::economic_state_has_nine_sub_fields ... ok
test state::q_state::tests::genesis_q_state_is_total_and_default ... ok
test state::q_state::tests::nine_field_count_via_serde_json ... ok
test state::q_state::tests::node_id_from_state_root_is_deterministic ... ok
test bottom_white::cas::store::tests::put_many_then_iterate_count ... ok
test state::sequencer::tests::dispatch_transition_stubs_all_variants ... ok
test sdk::sandbox::tests::test_sandbox_nonzero_exit ... ok
test state::sequencer::tests::apply_one_stub_does_not_consume_logical_t ... ok
test state::sequencer::tests::submit_advances_submit_id_only ... ok
test state::typed_tx::tests::golden_challenge_tx_digest ... ok
test state::typed_tx::tests::golden_finalize_reward_tx_digest ... ok
test state::typed_tx::tests::golden_reuse_tx_digest ... ok
test state::typed_tx::tests::golden_task_expire_tx_digest ... ok
test state::typed_tx::tests::golden_terminal_summary_tx_digest ... ok
test state::typed_tx::tests::golden_verify_tx_digest ... ok
test state::typed_tx::tests::golden_work_tx_digest ... ok
test state::typed_tx::tests::has_submitter_partitioning ... ok
test state::typed_tx::tests::signing_payload_domain_prefix_is_load_bearing ... ok
test state::sequencer::tests::submit_returns_queue_closed_after_rx_drop ... ok
test state::typed_tx::tests::signing_payload_domains_are_distinct ... ok
test state::sequencer::tests::submit_returns_queue_full_on_saturation ... ok
test state::typed_tx::tests::signing_payload_excludes_signature ... ok
test state::typed_tx::tests::typed_tx_btree_permutation_independence ... ok
test state::typed_tx::tests::typed_tx_btreemap_permutation_independence ... ok
test state::typed_tx::tests::signing_payload_golden_digests ... ok
test state::typed_tx::tests::typed_tx_cross_variant_non_collision ... ok
test state::typed_tx::tests::typed_tx_byte_stability_across_calls ... ok
test state::typed_tx::tests::typed_tx_default_round_trip ... ok
test state::typed_tx::tests::typed_tx_kind_projection ... ok
test top_white::predicates::registry::tests::agent_visible_view_filters_private ... ok
test top_white::predicates::registry::tests::duplicate_id_rejected ... ok
test state::typed_tx::tests::typed_tx_round_trip_all_variants ... ok
test top_white::predicates::registry::tests::empty_id_rejected ... ok
test top_white::predicates::registry::tests::merkle_root_changes_on_register ... ok
test top_white::predicates::registry::tests::empty_registry_root_is_sha256_empty ... ok
test top_white::predicates::registry::tests::metadata_canonical_hash_deterministic ... ok
test top_white::predicates::registry::tests::merkle_root_deterministic_two_runs ... ok
test top_white::predicates::registry::tests::register_and_get_round_trip ... ok
test top_white::predicates::visibility::tests::commit_reveal_pre_reveal ... ok
test top_white::predicates::registry::tests::metadata_canonical_hash_differs_on_visibility ... ok
test top_white::predicates::visibility::tests::public_always_visible ... ok
test top_white::predicates::visibility::tests::private_never_content_visible ... ok
test top_white::predicates::visibility::tests::serde_round_trip_public ... ok
test top_white::predicates::visibility::tests::serde_round_trip_commit_reveal ... ok
test wal::tests::test_wal_replay_missing_file_is_empty ... ok
test state::typed_tx::tests::typed_tx_round_trip_100_inputs ... ok
test wal::tests::test_wal_skip_malformed_line ... ok
test wal::tests::test_wal_roundtrip_nodes_only ... ok
test wal::tests::test_wal_roundtrip_mixed ... ok
test sdk::sandbox::tests::test_sandbox_timeout_kills_process ... ok

test result: ok. 237 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 1.35s


codex
`cargo test --lib` passed locally as 237 passed, 1 ignored. I’m doing a final adversarial sweep for spec-to-code drift now, especially around replay symmetry and crash/restart semantics where unit tests can pass while the gate should still challenge.
exec
/bin/bash -lc "rg -n \"apply_one|head_t|LedgerWriter|ReplayError|replay_full_transition|append\\(|ObjectType::ProposalPayload|K3|ApplyError|FullTransition|Json|JSONL|sidecar|cold-replay|TransitionError\" handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:76:    pub head_t: NodeId,
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:270:| `self.graveyard: HashMap<String, Vec<String>>` | `bus.rs:48` | **ILLEGAL sidecar** (Art. 0.2 explicitly anti-patterned) | retire; replace with `RejectedAttemptSummary` stamped on next accepted tx + `TerminalSummaryTx` |
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:324:**Out of scope for v1.x spec** (deferred per Codex Q5/NEW-5 round-3 PARTIAL acknowledgment): full golden fixture corpus + differential fuzzing seed + complete runner ABI for QState/SignalBundle/TransitionError. v1.4 freezes the SERIALIZATION RULE (bincode v2 big-endian + BTreeMap lex); fixtures + ABI land in **CO1.1.4-pre1** (canonical fixture corpus) + **CO1.7** (full ABI surface). This is an **explicit deferral** — not unresolved spec ambiguity. STEP_B branch A and branch B both implement the SAME bincode rule; per-tx digest matching is mechanical from v1.4. Full corpus generation is a downstream code task, not spec scope.
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:338:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:342:        return Err(TransitionError::StaleParent {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:352:        return Err(TransitionError::SignatureInvalid);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:358:        return Err(TransitionError::StakeInsufficient { available: agent_balance, required: tx.stake });
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:366:            return Err(TransitionError::AcceptancePredicateFailed(acceptance_results));
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:371:            return Err(TransitionError::AcceptancePredicateFailed(acceptance_results));
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:395:    let new_ledger_root = ledger::append(&q.ledger_root_t, tx);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:412:    q_next.head_t = NodeId::from_state_root(new_state_root);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:434:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:438:        .ok_or(TransitionError::TargetWorkTxNotFound)?;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:440:        return Err(TransitionError::TargetWorkTxNotVerifiable);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:445:        return Err(TransitionError::SignatureInvalid);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:449:        return Err(TransitionError::StakeInsufficient);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:455:        return Err(TransitionError::VerificationPredicateFailed(verify_results));
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:465:    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:467:    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:485:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:489:        .ok_or(TransitionError::TargetWorkTxNotFound)?;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:491:        .ok_or(TransitionError::ChallengeWindowClosed)?;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:494:        return Err(TransitionError::ChallengeWindowClosed);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:499:        return Err(TransitionError::SignatureInvalid);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:503:        return Err(TransitionError::StakeInsufficient);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:510:        return Err(TransitionError::CounterexampleInsufficient(counter_check));
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:559:    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:561:    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:582:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:585:        .ok_or(TransitionError::ToolNotInRegistry)?;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:587:        return Err(TransitionError::ToolCreatorMismatch);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:592:        .ok_or(TransitionError::TargetWorkTxNotFound)?;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:594:        return Err(TransitionError::ParentNotAcceptedYet);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:623:    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:625:    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:640:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:642:        .ok_or(TransitionError::ClaimNotFound)?;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:650:            return Err(TransitionError::ChallengeWindowStillOpen);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:653:            return Err(TransitionError::AlreadySlashed);  // never finalize a slashed claim
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:699:    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &FinalizeTx::from(claim_id, reward));
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:701:    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:718:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:720:        .ok_or(TransitionError::TaskNotFound)?;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:724:        return Err(TransitionError::InvalidSystemSignature);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:729:        return Err(TransitionError::StaleParent);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:735:        return Err(TransitionError::TaskNotExpired);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:738:        return Err(TransitionError::TaskHasOpenClaim);    // refund only if NO claim exists at all
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:757:    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:759:    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:832:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:835:        return Err(TransitionError::TerminalSummaryNotApplicable);  // only emitted for no-accept runs
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:851:    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &summary);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:853:    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:878:| I-NOSIDECAR | no Vec/HashMap "graveyard"-like sidecar (Art. 0.2) | static analysis | `tests/no_rejection_sidecar.rs` |
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:1030:| Tool post-append hook | `src/bus.rs:312-318` `tool.on_post_append()` | **RETIRED**: tool hooks become explicit ToolInvocation field in `WorkTx.write_set` (read by predicate runner); no separate hook |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:20:| **GR-2** (Gemini recommendation) | TransitionError additive-only commitment not stated | spec § 7.2 NEW: TransitionError variants in v4 are additive-only; never reorder; new variants append at the end | Gemini Q9 / GR-2 |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:34:| **P4** | `TransitionError` had only 10 variants; STATE § 3 pseudocode invokes ~22 | Expanded to 22 variants: SignatureInvalid / StakeInsufficient / TargetWorkTxNotFound / TargetWorkTxNotVerifiable / ParentNotAcceptedYet / AcceptancePredicateFailed(PredicateId) / VerificationPredicateFailed(PredicateId) / SettlementPredicateFailed(PredicateId) / ChallengeWindowClosed / CounterexampleInsufficient / ToolNotInRegistry / ToolCreatorMismatch + 10 prior. Plus `NotYetImplemented` retained as explicit stub sentinel. | CX-1 (Codex Q-G) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:39:| **P9** | Cold-replay → Art 0.2 violation if CAS index not persisted | This spec § 0 NEW "Cross-Atom Ordering Gate": v1.1 PASS is contingent on CO1.4-extra (CAS index persistence) shipping BEFORE CO1.7-impl A4 (replay_full_transition). CO1.7-impl A2 (Sequencer apply path) and A3 (dispatch_transition stubs) may proceed; A4 BLOCKED on CO1.4-extra. | GM-1 (Gemini Q4) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:47:**Why this atom exists**: spec § 2.5 of `STATE_TRANSITION_SPEC_v1_2026-04-27.md` explicitly deferred "full ABI surface for QState/SignalBundle/TransitionError" to CO1.7. CO1.7 spec § 0 places the per-kind tx schemas in `STATE_TRANSITION_SPEC § 1` ("frozen on paper, not yet in code"). When CO1.7-impl A1 (Git2LedgerWriter, commit `a03cc52`) shipped, downstream A2 (TypedTx + dispatch_transition) discovered ~30 supporting schema types are required but **none of them exist in code** — only `MicroCoin` is defined. This atom defines that ABI surface in isolation under its own dual-audit gate, per the project's per-atom audit principle (CLAUDE.md "Audit Standard").
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:51:**Single sentence**: define every supporting type + the 7 typed-tx variant payload structs + the `TypedTx` enum, with `Serialize/Deserialize` derives over the spec § 2.5 canonical encoding (bincode v2 BE + fixed_int), so that CO1.7-impl A2-A4 (Sequencer + dispatch_transition + replay_full_transition) can be implemented against a stable type surface.
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:76:- **Sequencer + dispatch_transition + replay_full_transition** — these consume the ABI; they belong to CO1.7-impl **A2-A4** (post this atom).
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:78:- **TransitionError full taxonomy** — v1 emits a minimal enum covering the variants invoked in spec § 3 pseudocode (`ClaimNotFound`, `ChallengeWindowStillOpen`, `AlreadySlashed`, `TaskNotFound`, `InvalidSystemSignature`, `StaleParent`, `TaskNotExpired`, `TaskHasOpenClaim`, `TerminalSummaryNotApplicable`, `NotYetImplemented`); per-stage enum proliferation is a CO1.7.5 concern.
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:87:**Constitutional concern**: CO1.7 LedgerEntry stores typed-tx payloads in L3 CAS via `tx_payload_cid: Cid`. The current shipped `CasStore::open()` initializes an empty in-memory index (CO1.4 store.rs:67); after process restart the CAS bytes are unrecoverable until the index is repopulated. This means **cold-replay of L4 cannot reconstruct typed payloads** — a direct Art. 0.2 (tape canonicality) violation if uncorrected.
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:89:**Mitigation**: CAS index persistence is its own atom — **CO1.4-extra** — already named in CO1.7 spec § 0. CO1.4-extra adds index persistence (likely a sidecar JSONL or git-tag manifest) so cold-replay can recover payloads via `CasStore::get`.
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:93:- **CO1.7-impl A4 (replay_full_transition) MUST NOT ship before CO1.4-extra**. Until then, FullTransition replay errors with `CasMissing` after process restart (already documented in CO1.7 spec § 4 / `ReplayError::CasMissing`).
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:197:If wire-stored values diverge from Q-derived values at replay, **replay rejects with `TransitionError::ClaimNotFound` or a stricter mismatch error** (CO1.7-impl A4 enforces this; CO1.7.5 transition body owns the comparison rule).
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:247:The `TxKind` enum already exists in `transition_ledger.rs` with `#[repr(u8)]` and explicit discriminants. `TypedTx::tx_kind()` is the projection used by CO1.7 sequencer apply_one stage 5 (`tx_kind: TxKind::from_typed(&tx)` → renamed `TypedTx::tx_kind(&tx)` for ergonomics).
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:277:### § 7.2 TransitionError additive-only commitment (v1.2 GR-2 per Gemini round-2)
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:279:`TransitionError` variants in **v4 are additive-only**:
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:3:**Status**: v1.2 — round-2 returned PASS (Gemini, high) + CHALLENGE (Codex, high; 3 narrow patch blockers). Conservative merged CHALLENGE. v1.2 closes the 3 v1.1→v1.2 patches: (a) C3 actually wired in code (`CanonicalMessage::LedgerEntrySigning([u8;32])` + `transition_ledger_emitter::sign_ledger_entry`); (b) K3 head_t mutation explicitly deferred to CO1.7.5+ (no longer claimed in v1.x); (c) `ObjectType::Transition` replaced with shipped `ObjectType::ProposalPayload`. Plus typo fix and 1 new test. Awaiting round-3.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:16:**Single sentence**: implement the L4 transition_ledger module so that `ledger::append(parent_root, signing_digest) → new_root` (called from sequencer) is real code, the L4 sequencer (§ 5.2.1) is real code, and `Q_t.ledger_root_t` is no longer a placeholder.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:25:| **R2-K3** | Spec § 3 / § 5 said "CO1.7 owns head_t = NodeId(commit_sha)" but `LedgerWriter::commit` returns `Hash` not commit SHA; v1.1 InMemoryLedgerWriter has no commit_sha to return at all → contradiction | head_t mutation explicitly **deferred to CO1.7.5+** (when Git2LedgerWriter exists and can return both Hash + commit SHA). v1.x ledger owns `ledger_root_t` only; `head_t` continues to be set elsewhere (currently QState placeholder; CO1.7.5 wiring concern). Spec § 0 / § 3 / § 5 updated. | Codex round-2 must-fix #2 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:26:| **R2-C2-CAS** | Spec § 3 sequencer pseudocode used `ObjectType::Transition` but shipped `ObjectType` has no such variant | Spec § 3 changed to `ObjectType::ProposalPayload` (the existing variant for agent work_tx payloads — semantically correct, no schema extension needed) | Codex round-2 must-fix #3 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:37:| C1 | replay was single-mode; called "I-DETHASH witness" but skeleton only did chain check | Two-mode `ReplayMode::ChainOnly` (skeleton-stage) vs `ReplayMode::FullTransition` (CO1.7.5+; I-DETHASH witness only in this mode) | Codex Q-D + Gemini Q3 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:38:| C2 | spec did not acknowledge that shipped `CasStore::open()` initializes empty in-memory index → cold-replay impossible | § 0 + § 5 explicit dependency + mitigation: CasStore index persistence is deferred to **CO1.4-extra** (separate atom); v1 documents the gap | Codex Q-H + Gemini Q2 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:42:| K3 | L4/L5 head_t ownership inconsistent (spec line 194 vs 276 disagreed) | CO1.7 owns `ledger_root_t` + commit-chain `head_t = NodeId(commit_sha)` only; L5 (CO1.8) owns `state_root_t` mutation; sequencer drops `head_t = NodeId::from_state_root(...)` line | Codex Q-E |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:43:| K4 | spec `LedgerWriter::commit(&self) → NodeId` + `iter_from` did not match skeleton `commit(&mut self) → Hash` | Spec aligned to skeleton: `&mut self` + `Hash` return; `iter_from` deferred to CO1.7.5+ when needed for cold-replay | Codex Q-H |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:61:- **append(parent_root, signing_digest)**: pure function returning the new ledger_root.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:62:- **replay (two-mode)**: `ChainOnly` (chain integrity; skeleton-stage; v1) vs `FullTransition` (rerun pure transitions + verify state_root + verify signatures; CO1.7.5+; THE I-DETHASH witness).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:63:- **Storage backend**: git2-rs commit chain (built on CO1.4 CAS); each LedgerEntry = one git commit on `refs/transitions/main`. **R2-K3**: head_t mutation deferred to CO1.7.5+ — v1.x ledger does NOT mutate `Q_t.head_t` directly. Once `Git2LedgerWriter::commit` exists and returns commit_sha alongside Hash, CO1.7.5 wiring will set `head_t = NodeId(commit_sha)` outside the L4 sequencer apply path.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:70:- L5 materializer (state_root computation) — deferred to **CO1.8**. **K3 boundary (revised v1.2)**: CO1.7 owns `ledger_root_t` only; CO1.8 owns `state_root_t`; **head_t mutation is deferred to CO1.7.5+ wiring** (when `Git2LedgerWriter` exists). Sequencer does NOT mutate `state_root_t` or `head_t` directly; it accepts `q_next.state_root_t` as returned by the transition function and persists `ledger_root_t`.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:75:- **CAS index persistence (cold-replay enabler)** — `CasStore::open()` shipped at Wave 3 initializes empty in-memory index ([store.rs:67](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs)); cold-replay therefore cannot recover payloads via `CasStore::get` after restart. **CO1.4-extra** atom (NEW, scheduled post-CO1.7) adds index persistence (likely a sidecar JSONL or git-tag manifest). v1 ledger documents the dependency; full-mode replay is implementable once CO1.4-extra lands.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:115:    /// — accepted as-returned from the transition function (K3 boundary).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:250:└── transition_ledger.rs         (NEW; LedgerEntry, LedgerEntrySigningPayload, TxKind, append, replay_*, LedgerWriter)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:254:├── q_state.rs                   (existing; CO1.7 fills `ledger_root_t` placeholder; does NOT touch `state_root_t` per K3)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:262:## § 3 Sequencer (K1 dual-counter; K3 head_t ownership; C3 sign API)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:286:    /// Storage backend (in CO1.7.5+; skeleton uses InMemoryLedgerWriter).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:287:    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:307:    fn apply_one(&self, tx: TypedTx) -> Result<LedgerEntry, TransitionError> {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:320:            ObjectType::ProposalPayload,  // R2 fix: shipped CAS variant (NOT Transition)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:351:        // 7. Compute resulting_ledger_root via append() (pure)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:353:        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:377:        // **K3 (v1.2 revised)**: do NOT mutate q_w.head_t here. v1.x ledger owns
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:378:        // `ledger_root_t` only. head_t mutation is **deferred to CO1.7.5+ wiring**
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:379:        // (when `Git2LedgerWriter::commit` is implemented and can return commit_sha
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:380:        // alongside Hash). Until then, head_t remains at QState placeholder; replay
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:381:        // and chain-integrity tests do NOT depend on head_t.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:388:**Why dual counter (K1)**: rejection of a submission must NOT consume a logical_t, because (a) skeleton's `InMemoryLedgerWriter::commit` enforces `expected_logical_t = len + 1` and would reject a gap; (b) replay enforces `entry.logical_t == (i+1)` and would reject a gap. Submitter IDs (`submit_id`) are returned from `submit()` immediately for receipt; logical_t is observable only on the committed entry.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:390:**Why no head_t mutation in apply_one (K3, revised v1.2)**: v1.x CO1.7 owns `ledger_root_t` only. CO1.8 owns `state_root_t`. **head_t mutation deferred to CO1.7.5+** when `Git2LedgerWriter` provides a commit_sha return alongside Hash; the InMemoryLedgerWriter used by the v1 skeleton has no commit_sha to expose, so the trait keeps a single `Hash` return and head_t wiring is a separate downstream concern. Sequencer never calls `NodeId::from_state_root(...)`.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:392:**Q3 (Gemini)**: `Sequencer` vs `LedgerWriter + OrderingCoordinator` split — v1.1 keeps `Sequencer` as the abstraction; trait-segregation refactor is a v4.x consideration (the current single-writer constraint per § 5.2.1 makes the split synthetic for v1).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:396:## § 4 append() + replay() — two-mode (per C1)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:401:pub fn append(parent_root: &Hash, signing_digest: &Hash) -> Hash {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:418:    /// **THIS** is the I-DETHASH witness (I-DETHASH bound to FullTransition only).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:420:    FullTransition,
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:428:) -> Result<(Hash, Hash), ReplayError>;
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:431:pub fn replay_full_transition(
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:436:) -> Result<QState, ReplayError>;
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:439:**I-DETHASH witness (revised per C1)**: `replay_full_transition` is the I-DETHASH witness. `replay_chain_integrity` is necessary-but-not-sufficient — passing chain check does NOT prove transition determinism. v1 documents this explicitly to close trust ambiguity.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:441:**ReplayError enum** (skeleton already has 3 variants; v1.1 adds 4 more for FullTransition):
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:446:- `BadSignature { at }` (NEW; FullTransition only)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:447:- `CasMissing { at, cid }` (NEW; FullTransition only — fires if CO1.4-extra not yet landed)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:448:- `StateRootMismatch { at }` (NEW; FullTransition only)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:449:- `TransitionError { at, inner }` (NEW; wraps dispatch_transition errors)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:460:- Commit tree = `(payload_cid_blob, signature_blob)` (state_root NOT a tree blob — per K3, L5 owns state_root materialization).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:461:- **K3 (v1.2)**: `head_t = NodeId(commit_sha)` is the canonical convention WHEN head_t is wired (CO1.7.5+). v1.x sequencer does NOT mutate head_t — `Git2LedgerWriter` is needed to surface commit_sha. `NodeId::from_state_root(...)` is NOT used by L4 in any version.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:462:- **C2**: cold-replay availability requires `CasStore` index persistence; deferred to CO1.4-extra. Until then, full-mode replay errors with `CasMissing` if CAS state is not warm.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:465:**LedgerWriter trait (K4 reconciled to skeleton)**:
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:468:pub trait LedgerWriter: Send + Sync {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:471:    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:474:    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:479:    // K4: iter_from() deferred — used only by FullTransition replay; CO1.7.5+ adds it.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:483:**Implementation (CO1.7.5+)**: `Git2LedgerWriter` (built on existing CO1.4 CAS); skeleton `InMemoryLedgerWriter` for v1 testing.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:493:| **I-DET** | Same (Q_t, tx) → byte-identical (Q_{t+1}, signals) | sequencer.apply_one stages 2-7 (pure dispatch + deterministic append) |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:494:| **I-DETHASH** | replay_full_transition(genesis, entries) recovers live state_root | **Bound to FullTransition mode only** (C1); skeleton ChainOnly is necessary-but-not-sufficient |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:495:| **I-LOGTIME** | timestamp_logical strictly monotonic; no wall clock | sequencer apply_one stage 4; LedgerEntry has no wall-clock field |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:498:| **I-NOSIDE** | step_transition reads only (q, tx, registries) | append() and replay_* are pure; sequencer.apply_one isolates I/O to CAS put + writer commit |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:512:| `tests/inmemory_writer_logical_t` | skeleton (v1) | InMemoryLedgerWriter rejects logical_t gaps |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:518:| `tests/replay_full_transition_state_root` | CO1.7.5+ (post-CO1.4-extra) | FullTransition replay re-runs dispatch_transition; asserts state_root match (I-DETHASH witness) |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:535:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:575:6. **Cold-restart full replay** — depends on CO1.4-extra; until then, FullTransition mode errors with `CasMissing` after process restart. ChainOnly mode unaffected.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:593:| Q11 NEW open Qs | — | Codex round-1 listed: parent_ledger_root binding (now K2 / done), rejected-submission logical time (now K1 / done), CAS persistence (now C2 → CO1.4-extra), canonical fixtures (deferred to CO1.7.5+ test stubs), L4/L5 head_t ownership (now K3 / done). All addressed. |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:632:   - DIV-4 CasReader → LedgerCasView — **resolved**: narrow trait for replay_full_transition.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:633:   - DIV-5 CasStore::put 5-param — **resolved**: sequencer apply_one stage 3 builds full metadata.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:636:8. **K3 L4/L5 boundary blur** found by Codex round-1 — closed by spec § 0 + § 3 + § 5 boundary clarification.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:644:| `cargo check` on `src/bottom_white/ledger/transition_ledger.rs` | PASS | LedgerEntry / TxKind / append / replay_chain_integrity / InMemoryLedgerWriter all type-check against shipped CO1.4 + CO1.7.0a-f + Q_t types |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:657:| C1 replay two-mode | New `ReplayMode` enum + spec § 4 + I-DETHASH bound to FullTransition only | § 0, § 4, § 6 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:658:| C2 CAS cold-replay risk | New CO1.4-extra atom + § 0 explicit dependency note + ReplayError::CasMissing | § 0, § 5, § 13 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:662:| K3 L4/L5 head_t ownership | Boundary clarified: CO1.7 owns ledger_root + commit-chain head_t (NodeId(commit_sha)); CO1.8 owns state_root | § 0, § 3, § 5 |

exec
/bin/bash -lc 'rg -n "TODO|unimplemented!|panic!|unwrap'"\\(|expect\\(|NotYetImplemented|head_t|fetch_add|commit\\(|get_typed_payload|CasMissing|QStateLockPoisoned|ReceiverAlreadyTaken|ObjectType as Git2ObjectType\" src/bottom_white/ledger/transition_ledger.rs src/state/sequencer.rs src/bottom_white/cas/store.rs src/state/typed_tx.rs src/state/q_state.rs src/bottom_white/ledger/system_keypair.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/ledger/system_keypair.rs:536:    // TODO(CO1.7): parse genesis_payload.toml [system_pubkeys] entries and
src/bottom_white/ledger/system_keypair.rs:933:        let keypair = Ed25519Keypair::generate_with_secure_entropy().expect("keypair");
src/bottom_white/ledger/system_keypair.rs:936:            predicate_runner::sign_rejected_attempt_summary(&keypair, &summary).expect("sign");
src/bottom_white/ledger/system_keypair.rs:950:        let old = Ed25519Keypair::generate_with_secure_entropy().expect("old");
src/bottom_white/ledger/system_keypair.rs:951:        let new = Ed25519Keypair::generate_with_secure_entropy().expect("new");
src/bottom_white/ledger/system_keypair.rs:960:            terminal_summary_emitter::sign_epoch_rotation_proof(&old, &proof).expect("old sign");
src/bottom_white/ledger/system_keypair.rs:962:            terminal_summary_emitter::sign_epoch_rotation_proof(&new, &proof).expect("new sign");
src/state/q_state.rs:4://! TRACE_MATRIX Art 0.4 — Q_t version-controlled: `head_t` = git commit SHA in Path B substrate.
src/state/q_state.rs:47:/// TRACE_MATRIX Art 0.4 — `head_t` = git commit SHA in Path B substrate (40 hex chars).
src/state/q_state.rs:311:    pub head_t: NodeId,
src/state/q_state.rs:353:        assert_eq!(g.head_t, NodeId::default());
src/state/q_state.rs:360:        let s = serde_json::to_value(QState::genesis()).unwrap();
src/state/q_state.rs:361:        let obj = s.as_object().expect("object");
src/state/q_state.rs:370:            "head_t",
src/state/q_state.rs:386:        let s = serde_json::to_value(&e).unwrap();
src/state/q_state.rs:387:        let obj = s.as_object().unwrap();
src/state/q_state.rs:400:        a.0.insert(AgentId("alice".into()), MicroCoin::from_coin(10).unwrap());
src/state/q_state.rs:401:        a.0.insert(AgentId("bob".into()), MicroCoin::from_coin(20).unwrap());
src/state/q_state.rs:404:        b.0.insert(AgentId("bob".into()), MicroCoin::from_coin(20).unwrap());
src/state/q_state.rs:405:        b.0.insert(AgentId("alice".into()), MicroCoin::from_coin(10).unwrap());
src/state/q_state.rs:407:        let sa = serde_json::to_string(&a).unwrap();
src/state/q_state.rs:408:        let sb = serde_json::to_string(&b).unwrap();
src/state/typed_tx.rs:396:    let body = canonical_encode(value).expect("canonical_encode of signing payload");
src/state/typed_tx.rs:708:/// `NotYetImplemented` for CO1.7.5 stub bodies (per Codex Q-G CHALLENGE).
src/state/typed_tx.rs:787:    NotYetImplemented,
src/state/typed_tx.rs:814:            Self::NotYetImplemented => write!(f, "transition body not yet implemented (CO1.7.5)"),
src/state/typed_tx.rs:909:        let bytes = canonical_encode(value).expect("encode");
src/state/typed_tx.rs:1054:            let bytes = canonical_encode(&tx).expect("encode");
src/state/typed_tx.rs:1055:            let decoded: TypedTx = canonical_decode(&bytes).expect("decode");
src/state/typed_tx.rs:1064:        let bytes_a = canonical_encode(&tx).expect("encode a");
src/state/typed_tx.rs:1065:        let bytes_b = canonical_encode(&tx).expect("encode b");
src/state/typed_tx.rs:1077:            let bytes = canonical_encode(&outer).expect("encode");
src/state/typed_tx.rs:1078:            let back: TypedTx = canonical_decode(&bytes).expect("decode");
src/state/typed_tx.rs:1195:        let bytes_a = canonical_encode(&tx_a).expect("encode a");
src/state/typed_tx.rs:1196:        let bytes_b = canonical_encode(&tx_b).expect("encode b");
src/state/typed_tx.rs:1197:        let bytes_c = canonical_encode(&tx_c).expect("encode c");
src/state/typed_tx.rs:1216:            let bytes = canonical_encode(&tx).expect("encode default");
src/state/typed_tx.rs:1217:            let back: TypedTx = canonical_decode(&bytes).expect("decode default");
src/state/typed_tx.rs:1409:        let bytes_a = canonical_encode(&tx_a).expect("encode a");
src/state/typed_tx.rs:1410:        let bytes_b = canonical_encode(&tx_b).expect("encode b");
src/state/typed_tx.rs:1411:        let bytes_c = canonical_encode(&tx_c).expect("encode c");
src/bottom_white/cas/store.rs:26:use git2::{ObjectType as Git2ObjectType, Repository};
src/bottom_white/cas/store.rs:253:        let tmp = TempDir::new().unwrap();
src/bottom_white/cas/store.rs:254:        let store = CasStore::open(tmp.path()).unwrap();
src/bottom_white/cas/store.rs:261:        let cid = s.put(b"hello world", ObjectType::ProposalPayload, "alice", 100, None).unwrap();
src/bottom_white/cas/store.rs:262:        let content = s.get(&cid).unwrap();
src/bottom_white/cas/store.rs:270:        let cid = s.put(&big, ObjectType::PredicateBytecode, "system", 0, Some("wasm".into())).unwrap();
src/bottom_white/cas/store.rs:271:        let content = s.get(&cid).unwrap();
src/bottom_white/cas/store.rs:278:        let cid_a = s.put(b"x", ObjectType::Generic, "alice", 1, None).unwrap();
src/bottom_white/cas/store.rs:279:        let cid_b = s.put(b"x", ObjectType::Generic, "bob", 2, None).unwrap();
src/bottom_white/cas/store.rs:288:        let cid = s.put(b"specific content", ObjectType::Generic, "system", 0, None).unwrap();
src/bottom_white/cas/store.rs:300:            other => panic!("expected CidNotFound, got {other:?}"),
src/bottom_white/cas/store.rs:307:        let cid = s.put(b"meta test", ObjectType::CounterexamplePayload, "carol", 250, Some("v1".into())).unwrap();
src/bottom_white/cas/store.rs:308:        let meta = s.metadata(&cid).unwrap();
src/bottom_white/cas/store.rs:322:            s1.put(content, ObjectType::Generic, "system", 0, None).unwrap();
src/bottom_white/cas/store.rs:326:            s2.put(content, ObjectType::Generic, "system", 0, None).unwrap();
src/bottom_white/cas/store.rs:347:        let cid_a = store_a.put(b"only in a", ObjectType::Generic, "agent_a", 100, None).unwrap();
src/bottom_white/cas/store.rs:348:        let cid_b = store_b.put(b"only in b", ObjectType::Generic, "agent_b", 100, None).unwrap();
src/bottom_white/cas/store.rs:368:            .unwrap();
src/bottom_white/cas/store.rs:380:        let tmp = TempDir::new().expect("tempdir");
src/bottom_white/cas/store.rs:384:            let mut s = CasStore::open(tmp.path()).expect("open");
src/bottom_white/cas/store.rs:387:                .unwrap();
src/bottom_white/cas/store.rs:390:                .unwrap();
src/bottom_white/cas/store.rs:394:        let s2 = CasStore::open(tmp.path()).expect("reopen");
src/bottom_white/cas/store.rs:396:        assert_eq!(s2.get(&cid_a).expect("get a"), b"alpha");
src/bottom_white/cas/store.rs:397:        assert_eq!(s2.get(&cid_b).expect("get b"), b"beta");
src/bottom_white/cas/store.rs:399:        let meta_b = s2.metadata(&cid_b).expect("metadata b");
src/bottom_white/cas/store.rs:409:        let tmp = TempDir::new().expect("tempdir");
src/bottom_white/cas/store.rs:410:        let mut s = CasStore::open(tmp.path()).expect("open");
src/bottom_white/cas/store.rs:413:            .unwrap();
src/bottom_white/cas/store.rs:416:            .unwrap();
src/bottom_white/cas/store.rs:419:            .unwrap()
src/bottom_white/cas/store.rs:433:        let tmp = TempDir::new().expect("tempdir");
src/bottom_white/cas/store.rs:434:        let mut s = CasStore::open(tmp.path()).expect("open");
src/bottom_white/cas/store.rs:443:            .unwrap();
src/bottom_white/cas/store.rs:447:            .unwrap()
src/bottom_white/cas/store.rs:457:        let tmp = TempDir::new().expect("tempdir");
src/bottom_white/cas/store.rs:460:            let mut s = CasStore::open(tmp.path()).expect("open");
src/bottom_white/cas/store.rs:461:            s.put(b"hello", ObjectType::Generic, "alice", 1, None).unwrap();
src/bottom_white/cas/store.rs:465:        let mut f = OpenOptions::new().append(true).open(&path).unwrap();
src/bottom_white/cas/store.rs:466:        f.write_all(b"this is not valid json\n").unwrap();
src/bottom_white/cas/store.rs:467:        f.sync_data().unwrap();
src/bottom_white/cas/store.rs:475:            other => panic!("expected IndexParse, got {other:?}"),
src/bottom_white/cas/store.rs:482:        let tmp = TempDir::new().expect("tempdir");
src/bottom_white/cas/store.rs:483:        let s = CasStore::open(tmp.path()).expect("open");
src/state/sequencer.rs:5://!   pseudocode, K1 dual-counter, K3 head_t deferred, C3 sign API)
src/state/sequencer.rs:12://! `TransitionError::NotYetImplemented`; CO1.7.5 (downstream atom) fills the
src/state/sequencer.rs:43:/// `TransitionError::NotYetImplemented`. CO1.7.5 fills each arm with the real
src/state/sequencer.rs:54:        TypedTx::Work(_) => Err(TransitionError::NotYetImplemented),
src/state/sequencer.rs:55:        TypedTx::Verify(_) => Err(TransitionError::NotYetImplemented),
src/state/sequencer.rs:56:        TypedTx::Challenge(_) => Err(TransitionError::NotYetImplemented),
src/state/sequencer.rs:57:        TypedTx::Reuse(_) => Err(TransitionError::NotYetImplemented),
src/state/sequencer.rs:58:        TypedTx::FinalizeReward(_) => Err(TransitionError::NotYetImplemented),
src/state/sequencer.rs:59:        TypedTx::TaskExpire(_) => Err(TransitionError::NotYetImplemented),
src/state/sequencer.rs:60:        TypedTx::TerminalSummary(_) => Err(TransitionError::NotYetImplemented),
src/state/sequencer.rs:116:    QStateLockPoisoned,
src/state/sequencer.rs:127:            Self::QStateLockPoisoned => write!(f, "q-state lock poisoned"),
src/state/sequencer.rs:157:    ReceiverAlreadyTaken,
src/state/sequencer.rs:163:            Self::ReceiverAlreadyTaken => write!(f, "sequencer receiver already taken"),
src/state/sequencer.rs:180:/// **K3 v1.2 (revised)**: Sequencer does NOT mutate `q.head_t` or
src/state/sequencer.rs:182:/// `QState` and the sequencer accepts it as-is. `head_t` mutation defers to
src/state/sequencer.rs:244:        let submit_id = self.next_submit_id.fetch_add(1, Ordering::SeqCst);
src/state/sequencer.rs:261:            // Stub state: dispatch returns NotYetImplemented; apply_one
src/state/sequencer.rs:276:            let g = self.q.read().map_err(|_| ApplyError::QStateLockPoisoned)?;
src/state/sequencer.rs:280:        // Stage 2: dispatch (pure). On reject (incl. NotYetImplemented stub),
src/state/sequencer.rs:296:            let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
src/state/sequencer.rs:307:        let logical_t = self.next_logical_t.fetch_add(1, Ordering::SeqCst) + 1;
src/state/sequencer.rs:348:        // K3 v1.2 (revised): we set q.ledger_root_t but NOT q.head_t (head_t
src/state/sequencer.rs:352:            let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
src/state/sequencer.rs:356:                .map_err(|_| ApplyError::QStateLockPoisoned)?;
src/state/sequencer.rs:357:            writer_w.commit(&entry)?;
src/state/sequencer.rs:370:            .map_err(|_| ApplyError::QStateLockPoisoned)
src/state/sequencer.rs:408:        let tmp = TempDir::new().expect("tempdir");
src/state/sequencer.rs:409:        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas open")));
src/state/sequencer.rs:411:            Ed25519Keypair::generate_with_secure_entropy().expect("keypair gen"),
src/state/sequencer.rs:451:    // 1. dispatch_transition: every variant returns NotYetImplemented (stub state).
src/state/sequencer.rs:519:            assert!(matches!(result, Err(TransitionError::NotYetImplemented)));
src/state/sequencer.rs:530:        let r1 = seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("submit 1");
src/state/sequencer.rs:535:        let r2 = seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("submit 2");
src/state/sequencer.rs:540:    // 3. apply_one in stub mode: returns Transition(NotYetImplemented); no
src/state/sequencer.rs:547:        assert!(matches!(err, ApplyError::Transition(TransitionError::NotYetImplemented)));
src/state/sequencer.rs:556:        let tmp = TempDir::new().expect("tempdir");
src/state/sequencer.rs:557:        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
src/state/sequencer.rs:558:        let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("kp"));
src/state/sequencer.rs:574:        seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("1");
src/state/sequencer.rs:575:        seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("2");
src/bottom_white/ledger/transition_ledger.rs:19://! - K3: L4/L5 boundary clarified — CO1.7 owns ledger_root + commit-chain head_t;
src/bottom_white/ledger/transition_ledger.rs:36:use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
src/bottom_white/ledger/transition_ledger.rs:187:/// **K4**: signature `commit(&mut self) → Hash` (NOT `&self → NodeId`); `iter_from`
src/bottom_white/ledger/transition_ledger.rs:190:    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
src/bottom_white/ledger/transition_ledger.rs:228:    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
src/bottom_white/ledger/transition_ledger.rs:277:    CasMissing { at: usize },
src/bottom_white/ledger/transition_ledger.rs:281:    /// `inner = NotYetImplemented`.
src/bottom_white/ledger/transition_ledger.rs:298:            Self::CasMissing { at } => write!(f, "CAS payload not retrievable at index {at}"),
src/bottom_white/ledger/transition_ledger.rs:315:    fn get_typed_payload(
src/bottom_white/ledger/transition_ledger.rs:322:    fn get_typed_payload(
src/bottom_white/ledger/transition_ledger.rs:326:        self.get(cid).map_err(|_| ReplayError::CasMissing { at: 0 })
src/bottom_white/ledger/transition_ledger.rs:344:/// returns `NotYetImplemented` for every variant, replay errors at stage 7
src/bottom_white/ledger/transition_ledger.rs:406:            .get_typed_payload(&entry.tx_payload_cid)
src/bottom_white/ledger/transition_ledger.rs:407:            .map_err(|_| ReplayError::CasMissing { at: i })?;
src/bottom_white/ledger/transition_ledger.rs:411:            .map_err(|_| ReplayError::CasMissing { at: i })?;
src/bottom_white/ledger/transition_ledger.rs:568:/// - **Parent**: `head_t-1` commit (or none at genesis).
src/bottom_white/ledger/transition_ledger.rs:574:/// need it (CO1.7.5+ `head_t` wiring), but the `LedgerWriter::commit` trait
src/bottom_white/ledger/transition_ledger.rs:618:                    let commit = repo.find_commit(c).map_err(|e| {
src/bottom_white/ledger/transition_ledger.rs:641:    /// CO1.7.5+ `head_t` wiring uses this to surface commit_sha alongside Hash.
src/bottom_white/ledger/transition_ledger.rs:657:            let commit = repo.find_commit(cursor).map_err(|e| {
src/bottom_white/ledger/transition_ledger.rs:667:            .find_commit(cursor)
src/bottom_white/ledger/transition_ledger.rs:687:    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
src/bottom_white/ledger/transition_ledger.rs:733:            Some(oid) => vec![repo.find_commit(oid).map_err(|e| {
src/bottom_white/ledger/transition_ledger.rs:741:            .commit(
src/bottom_white/ledger/transition_ledger.rs:849:        assert!(w.commit(&e1).is_ok());
src/bottom_white/ledger/transition_ledger.rs:852:        let err = w.commit(&e_skip).unwrap_err();
src/bottom_white/ledger/transition_ledger.rs:864:                .expect("clean chain replays");
src/bottom_white/ledger/transition_ledger.rs:941:        let keypair = Ed25519Keypair::generate_with_secure_entropy().expect("keypair gen");
src/bottom_white/ledger/transition_ledger.rs:962:            .expect("sign_ledger_entry");
src/bottom_white/ledger/transition_ledger.rs:1004:        let tmp = TempDir::new().expect("tempdir");
src/bottom_white/ledger/transition_ledger.rs:1005:        let w = Git2LedgerWriter::open(tmp.path()).expect("open");
src/bottom_white/ledger/transition_ledger.rs:1026:        let r1 = w.commit(&e1).expect("commit 1");
src/bottom_white/ledger/transition_ledger.rs:1029:        let oid_1 = w.head_commit_oid().expect("head after 1");
src/bottom_white/ledger/transition_ledger.rs:1031:        let r2 = w.commit(&e2).expect("commit 2");
src/bottom_white/ledger/transition_ledger.rs:1034:        let oid_2 = w.head_commit_oid().expect("head after 2");
src/bottom_white/ledger/transition_ledger.rs:1037:        w.commit(&e3).expect("commit 3");
src/bottom_white/ledger/transition_ledger.rs:1041:        assert_eq!(w.read_at(1).expect("read 1"), e1);
src/bottom_white/ledger/transition_ledger.rs:1042:        assert_eq!(w.read_at(2).expect("read 2"), e2);
src/bottom_white/ledger/transition_ledger.rs:1043:        assert_eq!(w.read_at(3).expect("read 3"), e3);
src/bottom_white/ledger/transition_ledger.rs:1051:        w.commit(&e1).expect("commit 1");
src/bottom_white/ledger/transition_ledger.rs:1056:        let err = w.commit(&e_skip).unwrap_err();
src/bottom_white/ledger/transition_ledger.rs:1067:        let tmp = TempDir::new().expect("tempdir");
src/bottom_white/ledger/transition_ledger.rs:1072:            let mut w = Git2LedgerWriter::open(tmp.path()).expect("open");
src/bottom_white/ledger/transition_ledger.rs:1073:            w.commit(&e1).expect("commit 1");
src/bottom_white/ledger/transition_ledger.rs:1074:            w.commit(&e2).expect("commit 2");
src/bottom_white/ledger/transition_ledger.rs:1075:            oid_after_two = w.head_commit_oid().expect("head");
src/bottom_white/ledger/transition_ledger.rs:1078:        let w2 = Git2LedgerWriter::open(tmp.path()).expect("reopen");
src/bottom_white/ledger/transition_ledger.rs:1081:        assert_eq!(w2.read_at(1).expect("read 1"), e1);
src/bottom_white/ledger/transition_ledger.rs:1082:        assert_eq!(w2.read_at(2).expect("read 2"), e2);
src/bottom_white/ledger/transition_ledger.rs:1085:        let mut w3 = Git2LedgerWriter::open(tmp.path()).expect("reopen 2");
src/bottom_white/ledger/transition_ledger.rs:1087:        w3.commit(&e3).expect("commit 3");
src/bottom_white/ledger/transition_ledger.rs:1146:        let bytes = canonical_encode(typed_tx).expect("encode");
src/bottom_white/ledger/transition_ledger.rs:1149:            .expect("cas put");
src/bottom_white/ledger/transition_ledger.rs:1163:            .expect("sign");
src/bottom_white/ledger/transition_ledger.rs:1189:        let tmp = TempDir::new().expect("tempdir");
src/bottom_white/ledger/transition_ledger.rs:1190:        let cas = CasStore::open(tmp.path()).expect("cas");
src/bottom_white/ledger/transition_ledger.rs:1191:        let kp = Ed25519Keypair::generate_with_secure_entropy().expect("kp");
src/bottom_white/ledger/transition_ledger.rs:1200:    /// 15. CO1.7.5-stage: in stub mode, dispatch errors with NotYetImplemented;
src/bottom_white/ledger/transition_ledger.rs:1201:    ///     replay correctly bubbles up `Transition { at: 0, inner: NotYetImplemented }`.
src/bottom_white/ledger/transition_ledger.rs:1228:            matches!(err, ReplayError::Transition { at: 0, inner: crate::state::typed_tx::TransitionError::NotYetImplemented }),
src/bottom_white/ledger/transition_ledger.rs:1229:            "expected Transition(NotYetImplemented at 0); got {err:?}"
src/bottom_white/ledger/transition_ledger.rs:1267:        let tmp = TempDir::new().expect("tempdir");
src/bottom_white/ledger/transition_ledger.rs:1268:        let kp = Ed25519Keypair::generate_with_secure_entropy().expect("kp");
src/bottom_white/ledger/transition_ledger.rs:1277:            let mut cas = CasStore::open(tmp.path()).expect("cas");
src/bottom_white/ledger/transition_ledger.rs:1290:        let cas2 = CasStore::open(tmp.path()).expect("reopen");
src/bottom_white/ledger/transition_ledger.rs:1314:        // (q_next, _signals) — currently all NotYetImplemented.
src/bottom_white/ledger/transition_ledger.rs:1321:        let bytes = canonical_encode(&e1).expect("encode");
src/bottom_white/ledger/transition_ledger.rs:1322:        let e1_back: LedgerEntry = canonical_decode(&bytes).expect("decode");
src/bottom_white/ledger/transition_ledger.rs:1326:        let bytes_again = canonical_encode(&e1).expect("encode again");

exec
/bin/bash -lc 'git log --oneline --decorate -n 12' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
272fcf4 (HEAD -> main, origin/main) CO1.7-impl A4: replay_full_transition + LedgerCasView + 4 conformance tests
b6b7574 CO1.4-extra: CAS index persistence via sidecar JSONL
609d8d5 CO1.7-impl A2 + A3: Sequencer + dispatch_transition (stub state)
c1226e2 CO1.1.4-pre1 PASS/PASS gate cleared — Wave 6 #1 ABI atom COMPLETE ✅
4d917ac CO1.1.4-pre1 v1.2.2 + round-4 audit (CHALLENGE → 2 more doc fixes)
33e75b8 CO1.1.4-pre1 v1.2.1 + round-3 audit (PASS/CHALLENGE → 2 doc-only fixes)
f4649a9 CO1.1.4-pre1 v1.2 + round-2 audit closure (CHALLENGE/PASS → CHALLENGE)
e0e4565 CO1.1.4-pre1 v1.1: round-1 audit closure (10 patches integrated)
df548c5 CO1.1.4-pre1 round-1 dual audit returned: CHALLENGE/CHALLENGE → merged CHALLENGE
227de72 CO1.1.4-pre1 v1: Typed Tx ABI surface (spec + impl + tests)
a03cc52 CO1.7-impl A1: Git2LedgerWriter + canonical bincode codec
c3b00d0 CO1.7 PASS/PASS gate cleared — Wave 6 #1 spec phase COMPLETE ✅

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '580,635p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   580	        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, EpochRotationProof,
   581	        KeypairError, SystemSignature,
   582	    };
   583	
   584	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: sign an opaque 32-byte digest of a
   585	    /// terminal-summary signing payload (computed by typed_tx).
   586	    pub(crate) fn sign_terminal_summary(
   587	        keypair: &Ed25519Keypair,
   588	        digest: [u8; 32],
   589	    ) -> Result<SystemSignature, KeypairError> {
   590	        sign_system_message_inner(keypair, &CanonicalMessage::TerminalSummarySigning(digest))
   591	    }
   592	
   593	    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): sign an
   594	    /// opaque 32-byte digest of a `FinalizeRewardSigningPayload` (computed by
   595	    /// state::typed_tx). Symmetric to `sign_terminal_summary` and
   596	    /// `sign_task_expire`.
   597	    pub(crate) fn sign_finalize_reward(
   598	        keypair: &Ed25519Keypair,
   599	        digest: [u8; 32],
   600	    ) -> Result<SystemSignature, KeypairError> {
   601	        sign_system_message_inner(keypair, &CanonicalMessage::FinalizeRewardSigning(digest))
   602	    }
   603	
   604	    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): sign an
   605	    /// opaque 32-byte digest of a `TaskExpireSigningPayload` (computed by
   606	    /// state::typed_tx). Symmetric to `sign_terminal_summary` and
   607	    /// `sign_finalize_reward`.
   608	    pub(crate) fn sign_task_expire(
   609	        keypair: &Ed25519Keypair,
   610	        digest: [u8; 32],
   611	    ) -> Result<SystemSignature, KeypairError> {
   612	        sign_system_message_inner(keypair, &CanonicalMessage::TaskExpireSigning(digest))
   613	    }
   614	
   615	    /// TRACE_MATRIX FC3-Sig: sign only typed epoch rotation proofs.
   616	    pub(crate) fn sign_epoch_rotation_proof(
   617	        keypair: &Ed25519Keypair,
   618	        proof: &EpochRotationProof,
   619	    ) -> Result<SystemSignature, KeypairError> {
   620	        sign_system_message_inner(
   621	            keypair,
   622	            &CanonicalMessage::EpochRotationProof(proof.clone()),
   623	        )
   624	    }
   625	
   626	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: sign only typed canonical messages within terminal-summary scope.
   627	    pub(crate) fn sign_system_message(
   628	        keypair: &Ed25519Keypair,
   629	        message: &CanonicalMessage,
   630	    ) -> Result<SystemSignature, KeypairError> {
   631	        sign_system_message_inner(keypair, message)
   632	    }
   633	}
   634	
   635	/// TRACE_MATRIX FC2-Append + FC1-Sig: crate-only signing surface for the L4

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '635,660p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   635	/// TRACE_MATRIX FC2-Append + FC1-Sig: crate-only signing surface for the L4
   636	/// transition ledger sequencer (CO1.7 v1.2). Authorized emitter pattern per
   637	/// round-1 audit Q-G recommendation: the ledger sequencer calls
   638	/// `sign_ledger_entry` with the canonical digest of `LedgerEntrySigningPayload`
   639	/// and gets back a `SystemSignature` bound through `CanonicalMessage`. No raw
   640	/// digest signer escapes this module.
   641	pub(crate) mod transition_ledger_emitter {
   642	    use super::{
   643	        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, KeypairError, SystemSignature,
   644	    };
   645	
   646	    /// TRACE_MATRIX FC2-Append: sign only the canonical-digest of a
   647	    /// `LedgerEntrySigningPayload`. Caller (sequencer in CO1.7) is responsible
   648	    /// for computing the digest; this fn only wraps in the typed enum.
   649	    pub(crate) fn sign_ledger_entry(
   650	        keypair: &Ed25519Keypair,
   651	        signing_payload_digest: [u8; 32],
   652	    ) -> Result<SystemSignature, KeypairError> {
   653	        sign_system_message_inner(
   654	            keypair,
   655	            &CanonicalMessage::LedgerEntrySigning(signing_payload_digest),
   656	        )
   657	    }
   658	}
   659	
   660	fn sign_system_message_inner(

codex
# Codex CO1.7-impl Bundle Round-1 Audit

## Q-A A1 Git2LedgerWriter

Tree blobs are the right move. `entry_canonical` is the canonical record, with `payload_cid` and `signature` as inspectable blobs ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:557)). Fixed author/committer identity and `git2::Time::new(logical_t as i64, 0)` are sufficient for deterministic commit OIDs assuming same git object format and same parent chain ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:726)).

Test gap: no test asserts same entries in two fresh repos produce identical commit OIDs. Existing reopen coverage is useful but not that property ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:1066)).

## Q-B A2 K1 + K3 deferred head_t

K1 transition rejection is correct for current stub and future pure `TransitionError`: dispatch returns before `next_logical_t` advances ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:282), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:307)).

But infra failure violates K1: `next_logical_t.fetch_add` happens before signing and ledger commit ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:307), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:324), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:357)). If sign or commit fails, `run()` logs/skips ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:264)), leaving the next accepted tx with a skipped `logical_t`, which the writer rejects forever.

K3 deferral is acceptable for this gate because the spec explicitly defers `head_t`; do not set a placeholder commit OID. CO1.7.5 must wire it.

## Q-C A2 ApplyError vs spec § 3 line 307

Defensible. `ApplyError` preserves `TransitionError` as the closed transition taxonomy while keeping CAS/keypair/writer errors out of it ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:94)). This is a documented spec deviation, not a design flaw.

## Q-D A2 panic safety

Not fatal for this atom, but weak. Poisoned locks map to `QStateLockPoisoned` ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:115)), but actual panics inside `apply_one` are not caught by `run()`. More importantly, commit-before-Q mutation means a panic after writer commit and before Q update can leave live memory inconsistent ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:351)).

## Q-E A3 stub pattern

Returning `Err(NotYetImplemented)` is correct. `unimplemented!()` would crash the sequencer and poison locks; the exhaustive match already gives the compile-time contract ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:47)).

## Q-F A4 staging order

The order is defensible. Signature-before-CAS is not cheaper, but it fail-closes earlier on unauthenticated ledger entries ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:391)). No challenge.

## Q-G A4 LedgerCasView trait

Accept. It is narrow and useful for replay tests/future backend separation ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:314)).

## Q-H CO1.4-extra sidecar discipline

Append-before-memory is correct ([store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:190)). Strict parse is correct for tape canonicality ([store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:90)).

Minor durability caveat: `sync_data` is used ([store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:130)), but parent-directory fsync is absent for first file creation. I would not block the bundle on that.

## Q-I CO1.4-extra durability gap

No idempotent-put gap found. Duplicate content returns before sidecar append only if already indexed ([store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:175)). Since new puts append sidecar before index insert, the stated crash window is handled.

## Q-J cross-atom A2↔A4 symmetry

Signing preimage is symmetric: apply builds `LedgerEntrySigningPayload` then signs/folds it ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:310)); replay rebuilds from entry fields and verifies/folds the same digest ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:392)).

But replay never checks decoded payload kind against `entry.tx_kind` after `canonical_decode` ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:410), [transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:413)). That means full replay can verify a signed ledger envelope whose `tx_kind` disagrees with the CAS payload it dispatches. This violates “every byte sequencer writes gets verified.”

## Q-K New defects

Blocking: `replay_full_transition` does not accept a genesis `QState`; it accepts only two roots and fabricates `QState::genesis()` ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:348), [transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:368)). That drops all non-root genesis state: budget, registries, balances, task markets. CO1.7.5 cannot reconstruct real state from this API.

Secondary: decode errors are reported as `CasMissing` ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:410)), which conflates missing payload with corrupt/non-canonical payload.

`cargo test --lib`: PASS locally, 237 passed, 1 ignored.

## Q-L Implementation gating

CO1.7.5 is close, but not gate-ready. The transition bodies can be written, but full replay and sequencer commit semantics need the above patches first.

## **VERDICT**: CHALLENGE

## Top must-fix

1. Change `replay_full_transition` to accept/reconstruct from full genesis `QState`, not only roots.
2. Move `next_logical_t` advancement to after successful ledger commit, or add rollback/transactional reservation.
3. In replay, assert `typed_tx.tx_kind() == entry.tx_kind`; add a dedicated replay error/test.

## Conviction

High.
2026-04-28T13:59:00.712514Z ERROR codex_core::session: failed to record rollout items: thread 019dd45e-6254-7961-8665-9a63a24e5877 not found
tokens used
189,349
# Codex CO1.7-impl Bundle Round-1 Audit

## Q-A A1 Git2LedgerWriter

Tree blobs are the right move. `entry_canonical` is the canonical record, with `payload_cid` and `signature` as inspectable blobs ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:557)). Fixed author/committer identity and `git2::Time::new(logical_t as i64, 0)` are sufficient for deterministic commit OIDs assuming same git object format and same parent chain ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:726)).

Test gap: no test asserts same entries in two fresh repos produce identical commit OIDs. Existing reopen coverage is useful but not that property ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:1066)).

## Q-B A2 K1 + K3 deferred head_t

K1 transition rejection is correct for current stub and future pure `TransitionError`: dispatch returns before `next_logical_t` advances ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:282), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:307)).

But infra failure violates K1: `next_logical_t.fetch_add` happens before signing and ledger commit ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:307), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:324), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:357)). If sign or commit fails, `run()` logs/skips ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:264)), leaving the next accepted tx with a skipped `logical_t`, which the writer rejects forever.

K3 deferral is acceptable for this gate because the spec explicitly defers `head_t`; do not set a placeholder commit OID. CO1.7.5 must wire it.

## Q-C A2 ApplyError vs spec § 3 line 307

Defensible. `ApplyError` preserves `TransitionError` as the closed transition taxonomy while keeping CAS/keypair/writer errors out of it ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:94)). This is a documented spec deviation, not a design flaw.

## Q-D A2 panic safety

Not fatal for this atom, but weak. Poisoned locks map to `QStateLockPoisoned` ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:115)), but actual panics inside `apply_one` are not caught by `run()`. More importantly, commit-before-Q mutation means a panic after writer commit and before Q update can leave live memory inconsistent ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:351)).

## Q-E A3 stub pattern

Returning `Err(NotYetImplemented)` is correct. `unimplemented!()` would crash the sequencer and poison locks; the exhaustive match already gives the compile-time contract ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:47)).

## Q-F A4 staging order

The order is defensible. Signature-before-CAS is not cheaper, but it fail-closes earlier on unauthenticated ledger entries ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:391)). No challenge.

## Q-G A4 LedgerCasView trait

Accept. It is narrow and useful for replay tests/future backend separation ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:314)).

## Q-H CO1.4-extra sidecar discipline

Append-before-memory is correct ([store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:190)). Strict parse is correct for tape canonicality ([store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:90)).

Minor durability caveat: `sync_data` is used ([store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:130)), but parent-directory fsync is absent for first file creation. I would not block the bundle on that.

## Q-I CO1.4-extra durability gap

No idempotent-put gap found. Duplicate content returns before sidecar append only if already indexed ([store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:175)). Since new puts append sidecar before index insert, the stated crash window is handled.

## Q-J cross-atom A2↔A4 symmetry

Signing preimage is symmetric: apply builds `LedgerEntrySigningPayload` then signs/folds it ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:310)); replay rebuilds from entry fields and verifies/folds the same digest ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:392)).

But replay never checks decoded payload kind against `entry.tx_kind` after `canonical_decode` ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:410), [transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:413)). That means full replay can verify a signed ledger envelope whose `tx_kind` disagrees with the CAS payload it dispatches. This violates “every byte sequencer writes gets verified.”

## Q-K New defects

Blocking: `replay_full_transition` does not accept a genesis `QState`; it accepts only two roots and fabricates `QState::genesis()` ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:348), [transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:368)). That drops all non-root genesis state: budget, registries, balances, task markets. CO1.7.5 cannot reconstruct real state from this API.

Secondary: decode errors are reported as `CasMissing` ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:410)), which conflates missing payload with corrupt/non-canonical payload.

`cargo test --lib`: PASS locally, 237 passed, 1 ignored.

## Q-L Implementation gating

CO1.7.5 is close, but not gate-ready. The transition bodies can be written, but full replay and sequencer commit semantics need the above patches first.

## **VERDICT**: CHALLENGE

## Top must-fix

1. Change `replay_full_transition` to accept/reconstruct from full genesis `QState`, not only roots.
2. Move `next_logical_t` advancement to after successful ledger commit, or add rollback/transactional reservation.
3. In replay, assert `typed_tx.tx_kind() == entry.tx_kind`; add a dedicated replay error/test.

## Conviction

High.
