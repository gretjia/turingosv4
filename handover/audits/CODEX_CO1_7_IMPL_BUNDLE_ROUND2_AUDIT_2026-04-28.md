# Codex CO1.7-impl Bundle Round-2 Narrow Audit
**Date**: 2026-04-28
**Target**: v1.1 closure (P1-P4)
**Prompt size**: 99693 chars

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
session id: 019dd469-3421-77b1-b15c-c2c091b58d3e
--------
user
# Codex Round-2 Narrow Audit — CO1.7-impl Bundle v1.1 (closure check)

**Role**: skeptical reviewer (round-2 narrow-scope closure). Round-1 returned CHALLENGE/CHALLENGE; v1.1 (commit `1a921e5`) closes 4 patches addressing Codex's R1 must-fix list.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## What changed v1.0 → v1.1 (your r1 must-fix items)

| Your r1 must-fix | v1.1 patch | Cite to verify |
|---|---|---|
| C-1: replay_full_transition accepts only roots; fabricates QState::genesis() | P1: signature changed to `replay_full_transition(genesis: &QState, ...) -> Result<QState, ReplayError>`; caller provides full QState; replay returns reconstructed final QState | transition_ledger.rs replay_full_transition signature + caller updates |
| C-2: K1 violated by infra failure — fetch_add before sign+commit | P2: apply_one uses `next_logical_t.load(SeqCst) + 1` for tentative; `next_logical_t.store(logical_t, SeqCst)` happens ONLY AFTER writer.commit succeeds; doc-comment justifies single-writer assumption | sequencer.rs apply_one stage 4-9 |
| C-3: replay never asserts tx_kind match | P3: NEW ReplayError::TxKindMismatch variant + stage 6.5 assertion + test replay_rejects_tx_kind_mismatch | transition_ledger.rs ReplayError enum + replay_full_transition stage 6.5 + tests |
| C-3-secondary: decode errors reported as CasMissing | P4: NEW ReplayError::PayloadDecode { at, reason } variant; replay stage 6 distinguishes decode from CAS-miss; test replay_rejects_payload_decode_failure | transition_ledger.rs ReplayError + stage 6 |

## Round-2 narrow questions

**Q1**: Does P1 (replay signature) actually return `Result<QState, ReplayError>` instead of two-roots-tuple? Does it correctly use `genesis: &QState` for state-root + ledger-root + downstream consumer fields?

**Q2**: Does P2 (logical_t ordering) correctly defer `next_logical_t.store(...)` to AFTER `writer.commit(&entry)?`? On commit Err, is fetch_add NOT called (preserving K1 under infra failure)?

**Q3**: Does P3 (tx_kind match) correctly reject the case where envelope claims one kind but CAS payload decodes as another? Test exercises the case with a re-signed tampered envelope?

**Q4**: Does P4 (PayloadDecode) cleanly separate decode failure from CAS miss? Test exercises decode failure with non-canonical bytes?

**Q5**: cargo test --lib still passes (237/0 + 1 ignored expected)?

**Q6**: Any OTHER stale references / patches missed? Specifically grep both files for "fetch_add(1, Ordering::SeqCst)" / "Result<(Hash, Hash)" / "genesis_state_root, genesis_ledger_root" residue.

## Output format

# Codex CO1.7-impl Bundle Round-2 Audit
## Q1 P1 replay signature
## Q2 P2 logical_t ordering
## Q3 P3 tx_kind match
## Q4 P4 PayloadDecode separation
## Q5 Test status
## Q6 Other stale residue
## **VERDICT**: PASS / CHALLENGE / VETO
## Top must-fix (if CHALLENGE)
## Conviction (low/med/high)

Be terse. Cite line numbers.

---



# Round-1 verdict (must-fix list this round verifies)

# CO1.7-impl Bundle Round-1 Dual External Audit — Merged Verdict

**Date**: 2026-04-28
**Target**: CO1.7-impl bundle (A1+A2+A3+A4) + CO1.4-extra — last commit `272fcf4`
**Auditors**: Codex (gpt-5-codex; 189,349 tokens) + Gemini 2.5 Pro (137,536 tokens)
**Conservative rule** (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

---

## § 1 Verdicts

| Auditor | Verdict | Conviction |
|---|---|---|
| **Codex** | **CHALLENGE** | High |
| **Gemini** | **CHALLENGE** | High |
| **Conservative merged** | **CHALLENGE** | High |

---

## § 2 Codex must-fix items (concrete patch-mechanical defects)

| ID | Item | Codex citation | Severity |
|---|---|---|---|
| **C-1** | `replay_full_transition` accepts only `genesis_state_root` + `genesis_ledger_root`; fabricates `QState::genesis()` and patches roots. **Drops budget / registries / balances / task_markets** — CO1.7.5 cannot reconstruct real state from this API | transition_ledger.rs:348+368 | Real defect |
| **C-2** | K1 invariant **violated under infra failure**: `next_logical_t.fetch_add(1)` happens BEFORE sign + writer.commit. If sign or commit fails, run() logs/skips; the next accepted tx gets a logical_t the writer rejects forever (`expected = len + 1` invariant broken) | sequencer.rs:307,324,357 | Real defect |
| **C-3** | Replay never asserts `decoded_typed_tx.tx_kind() == entry.tx_kind`. A signed envelope claiming `Work` could ride a CAS payload that decodes as `Verify`; "every byte sequencer writes gets verified" violated | transition_ledger.rs:410,413 | Real defect |
| **C-3-secondary** | Decode errors reported as `CasMissing` — conflates lookup-miss vs corrupt-payload | transition_ledger.rs:410 | Diagnostic gap |

## § 3 Gemini must-fix / risks

| ID | Item | Gemini citation | Type |
|---|---|---|---|
| **G-1** | `head_t` constitutional alignment (Art 0.4) — Sequencer never mutates head_t; QState::default empty string for entire CO1.7-impl runtime | Q4 | Carry-forward to CO1.7.5 (per K3 v1.2 deferred-by-design) |
| **G-2** | CO1.4-extra O(N) startup latency — long-term tech debt | Q3 | Risk (not blocker) |
| **G-3** | apply_one ↔ replay divergence hazard — two implementations of same conceptual process; future maintenance risk | Q9 | Engineering discipline |

## § 4 PASS items (both auditors)

- A1 Git2LedgerWriter tree-blob design (deterministic author time, byte-stable commit OIDs)
- A2 ApplyError vs spec § 3 line 307 — defensible deviation (TransitionError stays closed-taxonomy)
- A3 `Err(NotYetImplemented)` stub pattern over `unimplemented!()` macro (avoids panic poisoning sequencer)
- A4 staging order (sig before CAS — fail-close earlier on unauthenticated entries)
- A4 LedgerCasView trait — narrow + useful for testing + future backend separation
- CO1.4-extra append-before-memory + strict-mode parse — both correct
- CO1.4-extra durability gap absent (idempotent put short-circuits before sidecar write)
- A2↔A4 signing pre-image symmetry (apply builds, replay rebuilds — same bytes)

## § 5 v1.1 patches landed (this commit)

| Patch | Maps to | Change |
|---|---|---|
| **P1** | C-1 | `replay_full_transition` signature: `(genesis: &QState, ...)` instead of two roots; caller provides full QState including budget/registries/balances. Returns `Result<QState, ReplayError>`. |
| **P2** | C-2 | apply_one: `next_logical_t.load(SeqCst) + 1` for tentative use through stages 5-8; `next_logical_t.store(logical_t, SeqCst)` ONLY after writer.commit succeeds. Single-writer per spec § 5.2.1 makes load+store sufficient (no atomic reservation needed). Doc-comment justifies the pattern + flags upgrade-to-compare_exchange path if multi-writer ever lands. |
| **P3** | C-3 | `ReplayError::TxKindMismatch { at, envelope_kind, decoded_kind }` NEW variant; replay stage 6.5 asserts `decoded_typed_tx.tx_kind() == entry.tx_kind`. New test `replay_rejects_tx_kind_mismatch` exercises the case where the signed envelope claims `Verify` but CAS decodes as `Work`. |
| **P4** | C-3-secondary | `ReplayError::PayloadDecode { at, reason }` NEW variant; replay stage 6 distinguishes decode failure from CAS lookup miss. New test `replay_rejects_payload_decode_failure`. |

## § 6 Carry-forward items (NOT closed in this round)

| ID | Status | Rationale |
|---|---|---|
| **G-1** head_t Art 0.4 | DEFERRED to CO1.7.5 | Per CO1.7 spec K3 v1.2 (already PASS/PASS audited): head_t mutation deferred to CO1.7.5+ wiring when Git2LedgerWriter exposes commit_sha. Both auditors agree the spec-level deferral is acceptable for this gate; closure happens in CO1.7.5 atom. **Documented as a CO1.7.5 hard prerequisite** in the next atom's spec. |
| **G-2** CO1.4-extra O(N) | TRACKED, not blocking | Long-term tech debt; instrument + monitor in production; embedded-DB upgrade path open for post-Wave 6 scale. |
| **G-3** apply_one ↔ replay divergence | TRACKED, partially mitigated | C-3 (tx_kind match) closes one specific divergence vector. General discipline = engineering practice + future test harness sharing canonical apply/replay logic. |
| **D** apply_one panic safety | NOTED | Codex Q-D — non-fatal for this gate; commit-before-Q-mutation panic-window is theoretical. CO1.7.5 wiring may add `tokio::task::spawn_blocking` boundary to convert panics to errors. |

## § 7 Round structure forward

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 | CHALLENGE/high | CHALLENGE/high | **CHALLENGE** | v1.1 patch round (P1-P4 above) — this commit |
| 2 | ⏳ | ⏳ | TBD | re-audit on v1.1; expected PASS or 1-issue CHALLENGE on closure verification |

## § 8 Cumulative cost

| Round | Codex tokens | Gemini tokens | Estimated $ |
|---|---|---|---|
| Bundle r1 | 189,349 | 137,536 | ~$8-15 |

**CO1.7-impl bundle r1 cost**: ~$8-15. Cumulative project audit spend: ~$169-267 / $890 mid-budget (~19-30%).

---

## § 9 Sedimented lessons (this round)

1. **Bundling defensible despite 4 atoms in one audit**: both auditors gave concrete actionable findings. Bundling did NOT dilute focus. Net cost: 1× $8-15 vs 4× $8-15 = ~$24-60 per-atom = ~70% saving.

2. **K1 invariant requires "all-or-nothing" on counter advance**: spec § 3 wrote `fetch_add` at stage 4, but spec is pseudocode. Implementation must defer counter advance until after the LAST fallible step (writer.commit) succeeds. Sedimented: any monotonic counter that gates a load-bearing invariant MUST advance only on full critical-section success.

3. **Envelope-vs-payload integrity check is a real attack surface**: signing the envelope (LedgerEntry header fields including tx_kind) doesn't verify that the CAS payload bytes are the type the envelope claims. The check `decoded_payload.tx_kind() == entry.tx_kind` is a 5-LOC fix that closes a class of "envelope-payload swap" attacks. Sedimented: when an envelope binds a content-addressed reference, the dereferenced content MUST have its discriminator re-verified against the envelope's claim.

4. **API shape: replay must take FULL state, not subset**: a deceptively-small-feeling reduction (state_root + ledger_root) lost downstream-consumer capability (budget / registries). Sedimented: replay APIs are forward-locked once shipped; err on more-state-than-less for the first cut.

5. **Carry-forward CHALLENGEs are a real verdict pattern**: Gemini's #1 head_t is unfixable in this atom by spec design. The conservative-merge result is still CHALLENGE, but the action is "track + close in next atom" not "fix + re-audit". Sedimented: round-1 verdicts can have findings that close in a downstream atom; explicitly mark these as carry-forward in the merged verdict.

— ArchitectAI synthesis, 2026-04-28; Round-1 closure 2026-04-28; v1.1 patch round opens.


---

# Current src/bottom_white/ledger/transition_ledger.rs (v1.1)

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

# Current src/state/sequencer.rs (v1.1)

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

Now give your INDEPENDENT round-2 narrow audit.

codex
I’ll audit the checked-out repository, not the pasted excerpts, so the line citations match the workspace. I’m first loading the local instructions and the relevant git/file state.
exec
/bin/bash -lc 'git status --short && git rev-parse --short HEAD && git log -1 --oneline' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
?? handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md
?? handover/audits/run_codex_co1_7_impl_bundle_round2_audit.sh
1a921e5
1a921e5 CO1.7-impl bundle v1.1: round-1 audit closure (CHALLENGE/CHALLENGE → 4 patches)

exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
/home/zephryj/projects/turingosv4

codex
The checkout is at commit `1a921e5`; there are two untracked audit helper files already present, so I’ll leave them untouched. The first AGENTS search returned no repo-local file, and I’m checking ancestor locations before reading code.
exec
/bin/bash -lc 'ls /AGENTS.md /home/AGENTS.md /home/zephryj/AGENTS.md /home/zephryj/projects/AGENTS.md /home/zephryj/projects/turingosv4/AGENTS.md' in /home/zephryj/projects/turingosv4
 exited 2 in 0ms:
ls: cannot access '/AGENTS.md': No such file or directory
ls: cannot access '/home/AGENTS.md': No such file or directory
ls: cannot access '/home/zephryj/AGENTS.md': No such file or directory
ls: cannot access '/home/zephryj/projects/AGENTS.md': No such file or directory
ls: cannot access '/home/zephryj/projects/turingosv4/AGENTS.md': No such file or directory

exec
/bin/bash -lc 'find /home/zephryj/projects/turingosv4 -path /home/zephryj/projects/turingosv4/target -prune -o -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
codex
No scoped AGENTS.md exists in the repo or ancestors I checked. I’m starting the library test run now and will inspect the closure points while it runs.
exec
/bin/bash -lc "rg -n \"replay_full_transition|Result<\\(Hash, Hash\\)|genesis_state_root|genesis_ledger_root|fetch_add\\(1, Ordering::SeqCst\\)|TxKindMismatch|PayloadDecode|canonical_decode|next_logical_t\\.(load|store)|writer_w\\.commit|commit\\(&entry\\)\" src/bottom_white/ledger/transition_ledger.rs src/state/sequencer.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/sequencer.rs:244:        let submit_id = self.next_submit_id.fetch_add(1, Ordering::SeqCst);
src/state/sequencer.rs:301:        let logical_t = self.next_logical_t.load(Ordering::SeqCst) + 1;
src/state/sequencer.rs:357:        // v1.1 C-2: next_logical_t.store(logical_t) HAPPENS ONLY AFTER
src/state/sequencer.rs:368:            writer_w.commit(&entry)?; // ← may fail; if it does, fetch_add was NOT called
src/state/sequencer.rs:370:            self.next_logical_t.store(logical_t, Ordering::SeqCst);
src/state/sequencer.rs:391:        self.next_logical_t.load(Ordering::SeqCst)
src/bottom_white/ledger/transition_ledger.rs:289:    TxKindMismatch {
src/bottom_white/ledger/transition_ledger.rs:295:    /// retrieved from CAS but `canonical_decode` failed (corruption /
src/bottom_white/ledger/transition_ledger.rs:297:    PayloadDecode {
src/bottom_white/ledger/transition_ledger.rs:316:            Self::TxKindMismatch { at, envelope_kind, decoded_kind } => write!(
src/bottom_white/ledger/transition_ledger.rs:320:            Self::PayloadDecode { at, reason } => write!(f, "payload canonical_decode failed at index {at}: {reason}"),
src/bottom_white/ledger/transition_ledger.rs:327:// CO1.7-impl A4: LedgerCasView trait + replay_full_transition
src/bottom_white/ledger/transition_ledger.rs:331:/// needs. Decouples `replay_full_transition` from full `CasStore` (the
src/bottom_white/ledger/transition_ledger.rs:358:/// 6. canonical_decode of payload bytes → TypedTx
src/bottom_white/ledger/transition_ledger.rs:364:/// **v1.1 C-1 closure**: takes a full `genesis: &QState` (was `genesis_state_root`
src/bottom_white/ledger/transition_ledger.rs:365:/// + `genesis_ledger_root` only). Caller provides the complete genesis state
src/bottom_white/ledger/transition_ledger.rs:374:pub fn replay_full_transition(
src/bottom_white/ledger/transition_ledger.rs:426:        // Stage 6: canonical_decode → TypedTx (v1.1 C-3-secondary: distinct
src/bottom_white/ledger/transition_ledger.rs:429:            canonical_decode(&payload_bytes).map_err(|e| ReplayError::PayloadDecode {
src/bottom_white/ledger/transition_ledger.rs:440:            return Err(ReplayError::TxKindMismatch {
src/bottom_white/ledger/transition_ledger.rs:485:    genesis_state_root: Hash,
src/bottom_white/ledger/transition_ledger.rs:486:    genesis_ledger_root: Hash,
src/bottom_white/ledger/transition_ledger.rs:488:) -> Result<(Hash, Hash), ReplayError> {
src/bottom_white/ledger/transition_ledger.rs:489:    let mut prev_state_root = genesis_state_root;
src/bottom_white/ledger/transition_ledger.rs:490:    let mut prev_ledger_root = genesis_ledger_root;
src/bottom_white/ledger/transition_ledger.rs:548:pub fn canonical_decode<T: serde::de::DeserializeOwned>(
src/bottom_white/ledger/transition_ledger.rs:790:        canonical_decode::<LedgerEntry>(&bytes).map_err(|e| {
src/bottom_white/ledger/transition_ledger.rs:791:            LedgerWriterError::BackendCorruption(format!("canonical_decode at {logical_t}: {e}"))
src/bottom_white/ledger/transition_ledger.rs:1124:    // 15-18. CO1.7-impl A4 — replay_full_transition (THE I-DETHASH witness)
src/bottom_white/ledger/transition_ledger.rs:1237:    fn replay_full_transition_reaches_dispatch_then_stubs() {
src/bottom_white/ledger/transition_ledger.rs:1249:        let err = replay_full_transition(
src/bottom_white/ledger/transition_ledger.rs:1281:        let err = replay_full_transition(
src/bottom_white/ledger/transition_ledger.rs:1321:        let err = replay_full_transition(
src/bottom_white/ledger/transition_ledger.rs:1342:        // ledger_writer + replay_full_transition(...) → assert final state_root
src/bottom_white/ledger/transition_ledger.rs:1352:        let e1_back: LedgerEntry = canonical_decode(&bytes).expect("decode");
src/bottom_white/ledger/transition_ledger.rs:1362:        let err = canonical_decode::<LedgerEntry>(&bytes_extra).unwrap_err();

exec
/bin/bash -lc 'cargo test --lib' in /home/zephryj/projects/turingosv4
 succeeded in 1557ms:
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-337b37021c2192ab)

running 238 tests
test boot::tests::parse_errors_on_unquoted_key ... ok
test boot::tests::parse_errors_when_section_missing ... ok
test boot::tests::parse_strips_inline_comment_and_blanks ... ok
test boot::tests::verify_trust_root_detects_tamper_in_tempdir ... ok
test boot::tests::verify_trust_root_detects_child_manifest_tamper ... ok
test boot::tests::verify_trust_root_passes_when_hash_matches_in_tempdir ... ok
test bottom_white::cas::schema::tests::cid_differs_on_content ... ok
test bottom_white::cas::schema::tests::cid_deterministic ... ok
test bottom_white::cas::schema::tests::cid_from_empty_content ... ok
test bottom_white::cas::schema::tests::cid_display_format ... ok
test bottom_white::cas::schema::tests::metadata_canonical_hash_differs_on_object_type ... ok
test bottom_white::cas::schema::tests::metadata_canonical_hash_deterministic ... ok
test boot::tests::verify_trust_root_passes_with_matching_child_manifest ... ok
test bottom_white::cas::store::tests::cid_is_content_address ... ok
test bottom_white::cas::store::tests::cell_isolation_disjoint_cas ... ok
test bottom_white::cas::store::tests::corrupted_sidecar_line_returns_parse_error ... ok
test bottom_white::cas::store::tests::empty_store_root ... ok
test bottom_white::cas::store::tests::get_nonexistent_returns_error ... ok
test bottom_white::cas::store::tests::idempotent_put_does_not_duplicate_sidecar_line ... ok
test bottom_white::cas::store::tests::each_new_put_appends_one_line ... ok
test bottom_white::cas::store::tests::missing_sidecar_opens_fresh ... ok
test bottom_white::cas::store::tests::metadata_recorded ... ok
test bottom_white::cas::store::tests::put_get_round_trip_small ... ok
test bottom_white::cas::store::tests::merkle_root_deterministic_two_runs ... ok
test bottom_white::cas::store::tests::put_get_round_trip_large ... ok
test bottom_white::cas::store::tests::put_idempotent_same_content ... ok
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
test bottom_white::ledger::transition_ledger::tests::replay_rejects_bad_system_signature ... ok
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
test ledger::tests::test_tape_reject_dangling_citation ... ok
test ledger::tests::test_tape_reject_duplicate_id ... ok
test ledger::tests::test_tape_time_arrow_ordering ... ok
test ledger::tests::test_tape_trace_ancestors ... ok
test prediction_market::tests::test_assassin_profit ... ok
test prediction_market::tests::test_buy_no_increases_no_price ... ok
test prediction_market::tests::test_buy_yes_increases_yes_price ... ok
test prediction_market::tests::test_constant_product_invariant ... ok
test prediction_market::tests::test_create_market ... ok
test prediction_market::tests::test_ctf_conservation_1_coin_1_yes_1_no ... ok
test prediction_market::tests::test_initial_price_is_50_50 ... ok
test prediction_market::tests::test_multiple_traders_price_discovery ... ok
test prediction_market::tests::test_no_double_resolution ... ok
test prediction_market::tests::test_no_trading_after_resolution ... ok
test prediction_market::tests::test_pioneer_profit ... ok
test prediction_market::tests::test_prices_sum_to_one ... ok
test prediction_market::tests::test_redeem_requires_resolution ... ok
test prediction_market::tests::test_reject_zero_or_negative_amounts ... ok
test sdk::actor::tests::test_boltzmann_diversity_not_deterministic ... ok
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
test sdk::prompt::tests::test_prompt_surfaces_search_hits ... ok
test sdk::prompt::tests::test_prompt_surfaces_team_board ... ok
test sdk::prompt::tests::test_prompt_truncates_errors_to_3 ... ok
test sdk::prompt_guard::tests::test_case_insensitive_match - should panic ... ok
test sdk::prompt_guard::tests::test_clean_prompt_passes ... ok
test sdk::prompt_guard::tests::test_empty_prompt_passes ... ok
test sdk::prompt_guard::tests::test_h_vpput_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_assignment_pattern_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_m_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_runtime_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_substring_in_larger_text - should panic ... ok
test sdk::prompt_guard::tests::test_pput_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_wbcg_caught - should panic ... ok
test sdk::protocol::tests::test_deduct_negative_amount_rejected ... ok
test sdk::protocol::tests::test_malformed_action_tag_rejected_not_fallback ... ok
test sdk::protocol::tests::test_no_byte_repair_on_invalid_escape ... ok
test sdk::protocol::tests::test_parse_action_tag_valid ... ok
test sdk::protocol::tests::test_parse_action_tag_with_think_block ... ok
test sdk::protocol::tests::test_parse_bare_json_fallback ... ok
test sdk::protocol::tests::test_parse_invalid_json_returns_error ... ok
test sdk::protocol::tests::test_parse_no_action_returns_error ... ok
test sdk::protocol::tests::test_parse_with_invest_action ... ok
test sdk::protocol::tests::test_strip_multiple_think_blocks ... ok
test sdk::protocol::tests::test_strip_think_blocks ... ok
test bottom_white::ledger::transition_ledger::tests::signature_round_trip_and_transplant_defense ... ok
test sdk::protocol::tests::test_strip_unclosed_think_block ... ok
test boot::tests::verify_trust_root_passes_on_intact_repo ... ok
test sdk::sandbox::tests::test_sandbox_echo_command ... ok
test sdk::sandbox::tests::test_sandbox_captures_stderr ... ok
test sdk::snapshot::tests::test_snapshot_balance_query ... ok
test sdk::sandbox::tests::test_sandbox_nonzero_exit ... ok
test sdk::tools::librarian::tests::test_board_post_append ... ok
test sdk::tools::librarian::tests::test_build_compression_prompt ... ok
test sdk::tools::librarian::tests::test_board_write_read_roundtrip ... ok
test sdk::tools::librarian::tests::test_compress_interval ... ok
test sdk::tools::librarian::tests::test_zero_interval_never_compresses ... ok
test sdk::tools::search::tests::test_sanitize_query ... ok
test sdk::tools::search::tests::test_search_nonexistent_path ... ok
test sdk::tools::search::tests::test_search_empty_query ... ok
test sdk::tools::wallet::tests::test_append_is_free ... ok
test sdk::tools::wallet::tests::test_deduct_and_credit ... ok
test sdk::tools::wallet::tests::test_insufficient_balance_rejected ... ok
test sdk::tools::wallet::tests::test_genesis_allocation ... ok
test sdk::tools::wallet::tests::test_no_double_genesis ... ok
test sdk::tools::wallet::tests::test_negative_deduct_rejected ... ok
test sdk::tools::wallet::tests::test_portfolio_tracking ... ok
test sdk::tools::wallet::tests::test_query_balance ... ok
test sdk::tools::wallet::tests::test_query_unknown_key ... ok
test sdk::tools::wallet::tests::test_unknown_agent_vetoed ... ok
test sdk::tools::wallet::tests::test_zero_deduct_rejected ... ok
test state::q_state::tests::btreemap_insertion_order_independent_serialization ... ok
test state::q_state::tests::genesis_q_state_is_total_and_default ... ok
test state::q_state::tests::economic_state_has_nine_sub_fields ... ok
test state::q_state::tests::nine_field_count_via_serde_json ... ok
test state::q_state::tests::node_id_from_state_root_is_deterministic ... ok
test state::sequencer::tests::dispatch_transition_stubs_all_variants ... ok
test bottom_white::cas::store::tests::put_many_then_iterate_count ... ok
test state::sequencer::tests::submit_advances_submit_id_only ... ok
test state::sequencer::tests::apply_one_stub_does_not_consume_logical_t ... ok
test state::typed_tx::tests::golden_challenge_tx_digest ... ok
test state::typed_tx::tests::golden_finalize_reward_tx_digest ... ok
test state::typed_tx::tests::golden_reuse_tx_digest ... ok
test state::typed_tx::tests::golden_task_expire_tx_digest ... ok
test state::typed_tx::tests::golden_terminal_summary_tx_digest ... ok
test state::typed_tx::tests::golden_verify_tx_digest ... ok
test state::typed_tx::tests::golden_work_tx_digest ... ok
test state::typed_tx::tests::has_submitter_partitioning ... ok
test state::typed_tx::tests::signing_payload_domain_prefix_is_load_bearing ... ok
test state::typed_tx::tests::signing_payload_domains_are_distinct ... ok
test state::sequencer::tests::submit_returns_queue_closed_after_rx_drop ... ok
test state::typed_tx::tests::signing_payload_golden_digests ... ok
test state::typed_tx::tests::signing_payload_excludes_signature ... ok
test state::typed_tx::tests::typed_tx_btree_permutation_independence ... ok
test state::typed_tx::tests::typed_tx_btreemap_permutation_independence ... ok
test state::typed_tx::tests::typed_tx_byte_stability_across_calls ... ok
test state::typed_tx::tests::typed_tx_default_round_trip ... ok
test state::sequencer::tests::submit_returns_queue_full_on_saturation ... ok
test state::typed_tx::tests::typed_tx_cross_variant_non_collision ... ok
test state::typed_tx::tests::typed_tx_kind_projection ... ok
test top_white::predicates::registry::tests::agent_visible_view_filters_private ... ok
test top_white::predicates::registry::tests::duplicate_id_rejected ... ok
test state::typed_tx::tests::typed_tx_round_trip_all_variants ... ok
test top_white::predicates::registry::tests::empty_registry_root_is_sha256_empty ... ok
test top_white::predicates::registry::tests::empty_id_rejected ... ok
test top_white::predicates::registry::tests::merkle_root_changes_on_register ... ok
test top_white::predicates::registry::tests::merkle_root_deterministic_two_runs ... ok
test top_white::predicates::registry::tests::metadata_canonical_hash_deterministic ... ok
test top_white::predicates::registry::tests::metadata_canonical_hash_differs_on_visibility ... ok
test top_white::predicates::registry::tests::register_and_get_round_trip ... ok
test top_white::predicates::visibility::tests::commit_reveal_pre_reveal ... ok
test top_white::predicates::visibility::tests::private_never_content_visible ... ok
test top_white::predicates::visibility::tests::public_always_visible ... ok
test top_white::predicates::visibility::tests::serde_round_trip_public ... ok
test top_white::predicates::visibility::tests::serde_round_trip_commit_reveal ... ok
test wal::tests::test_wal_replay_missing_file_is_empty ... ok
test state::typed_tx::tests::typed_tx_round_trip_100_inputs ... ok
test wal::tests::test_wal_skip_malformed_line ... ok
test wal::tests::test_wal_roundtrip_nodes_only ... ok
test wal::tests::test_wal_roundtrip_mixed ... ok
test sdk::sandbox::tests::test_sandbox_timeout_kills_process ... ok

test result: ok. 237 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 1.31s


codex
The implementation paths are visible, but the first grep already shows one important audit wrinkle: the claimed new P3/P4 test names did not appear in the broad search output. I’m checking that directly before deciding whether this is just naming drift or a missed patch.
exec
/bin/bash -lc 'rg -n "replay_rejects_tx_kind_mismatch|replay_rejects_payload_decode_failure|TxKindMismatch|PayloadDecode|tampered envelope|non-canonical|canonical bytes|decode failure" src/bottom_white/ledger/transition_ledger.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
289:    TxKindMismatch {
296:    /// non-canonical bytes). Distinct from `CasMissing` (lookup failure).
297:    PayloadDecode {
316:            Self::TxKindMismatch { at, envelope_kind, decoded_kind } => write!(
320:            Self::PayloadDecode { at, reason } => write!(f, "payload canonical_decode failed at index {at}: {reason}"),
429:            canonical_decode(&payload_bytes).map_err(|e| ReplayError::PayloadDecode {
440:            return Err(ReplayError::TxKindMismatch {
1166:    /// with the typed_tx's canonical bytes stored in CAS. Mirrors

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '270,455p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '280,378p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '455,505p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '1228,1332p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1228	        let tools = ToolRegistry::new();
  1229	        (tmp, cas, kp, epoch, pinned, preds, tools)
  1230	    }
  1231	
  1232	    /// 15. CO1.7.5-stage: in stub mode, dispatch errors with NotYetImplemented;
  1233	    ///     replay correctly bubbles up `Transition { at: 0, inner: NotYetImplemented }`.
  1234	    ///     This proves stages 1-6 (chain + sig + CAS + decode) all PASS,
  1235	    ///     leaving stage 7 (dispatch) as the only gate. CO1.7.5 fills it.
  1236	    #[test]
  1237	    fn replay_full_transition_reaches_dispatch_then_stubs() {
  1238	        let (_tmp, mut cas, kp, epoch, pinned, preds, tools) = replay_test_setup();
  1239	        let entry = build_signed_entry(
  1240	            1,
  1241	            Hash::ZERO,
  1242	            Hash::ZERO,
  1243	            h(1), // resulting state_root (won't be reached due to dispatch stub)
  1244	            epoch,
  1245	            &kp,
  1246	            &mut cas,
  1247	            &dummy_typed_tx(),
  1248	        );
  1249	        let err = replay_full_transition(
  1250	            &crate::state::q_state::QState::genesis(),
  1251	            &[entry],
  1252	            &cas,
  1253	            &pinned,
  1254	            &preds,
  1255	            &tools,
  1256	        )
  1257	        .unwrap_err();
  1258	        assert!(
  1259	            matches!(err, ReplayError::Transition { at: 0, inner: crate::state::typed_tx::TransitionError::NotYetImplemented }),
  1260	            "expected Transition(NotYetImplemented at 0); got {err:?}"
  1261	        );
  1262	    }
  1263	
  1264	    /// 16. system_signature_verifies_via_canonical_message — tampering the
  1265	    ///     signature MUST fire BadSignature BEFORE dispatch is reached.
  1266	    #[test]
  1267	    fn replay_rejects_bad_system_signature() {
  1268	        let (_tmp, mut cas, kp, epoch, pinned, preds, tools) = replay_test_setup();
  1269	        let mut entry = build_signed_entry(
  1270	            1,
  1271	            Hash::ZERO,
  1272	            Hash::ZERO,
  1273	            h(1),
  1274	            epoch,
  1275	            &kp,
  1276	            &mut cas,
  1277	            &dummy_typed_tx(),
  1278	        );
  1279	        // Tamper signature.
  1280	        entry.system_signature = SystemSignature::from_bytes([0xff; 64]);
  1281	        let err = replay_full_transition(
  1282	            &crate::state::q_state::QState::genesis(),
  1283	            &[entry],
  1284	            &cas,
  1285	            &pinned,
  1286	            &preds,
  1287	            &tools,
  1288	        )
  1289	        .unwrap_err();
  1290	        assert!(matches!(err, ReplayError::BadSignature { at: 0 }));
  1291	    }
  1292	
  1293	    /// 17. cas_payload_round_trip — replay correctly fetches CAS bytes;
  1294	    ///     CO1.4-extra cold-restart capability test.
  1295	    #[test]
  1296	    fn replay_cas_payload_round_trip_after_reopen() {
  1297	        let tmp = TempDir::new().expect("tempdir");
  1298	        let kp = Ed25519Keypair::generate_with_secure_entropy().expect("kp");
  1299	        let epoch = SystemEpoch::new(1);
  1300	        let mut pinned = PinnedSystemPubkeys::new();
  1301	        pinned.insert(epoch, kp.public_key());
  1302	        let preds = PredicateRegistry::new();
  1303	        let tools = ToolRegistry::new();
  1304	
  1305	        let entry;
  1306	        {
  1307	            let mut cas = CasStore::open(tmp.path()).expect("cas");
  1308	            entry = build_signed_entry(
  1309	                1,
  1310	                Hash::ZERO,
  1311	                Hash::ZERO,
  1312	                h(1),
  1313	                epoch,
  1314	                &kp,
  1315	                &mut cas,
  1316	                &dummy_typed_tx(),
  1317	            );
  1318	        }
  1319	        // Reopen — CO1.4-extra sidecar replay restores the CAS index.
  1320	        let cas2 = CasStore::open(tmp.path()).expect("reopen");
  1321	        let err = replay_full_transition(
  1322	            &crate::state::q_state::QState::genesis(),
  1323	            &[entry],
  1324	            &cas2,
  1325	            &pinned,
  1326	            &preds,
  1327	            &tools,
  1328	        )
  1329	        .unwrap_err();
  1330	        // Stages 1-6.5 (incl. CAS lookup post-reopen + tx_kind match) PASS;
  1331	        // stage 7 stubs.
  1332	        assert!(matches!(err, ReplayError::Transition { at: 0, .. }));

codex
`cargo test --lib` passes with the expected `237 passed; 0 failed; 1 ignored`. The implementation has the tx-kind and decode error branches, but I’m verifying the caller set and stale-reference grep before the verdict because missing claimed tests may affect closure.
exec
/bin/bash -lc "rg -n \"replay_full_transition\\(\" src tests handover -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:4185:pub fn replay_full_transition(
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:4248:| **I-DETHASH** | replay_full_transition(genesis, entries) recovers live state_root | **Bound to FullTransition mode only** (C1); skeleton ChainOnly is necessary-but-not-sufficient |
handover/audits/run_codex_co1_7_impl_bundle_round2_audit.sh:21:| C-1: replay_full_transition accepts only roots; fabricates QState::genesis() | P1: signature changed to `replay_full_transition(genesis: &QState, ...) -> Result<QState, ReplayError>`; caller provides full QState; replay returns reconstructed final QState | transition_ledger.rs replay_full_transition signature + caller updates |
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:540:pub fn replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:603:| **I-DETHASH** | replay_full_transition(genesis, entries) recovers live state_root | **Bound to FullTransition mode only** (C1); skeleton ChainOnly is necessary-but-not-sufficient |
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:1505:pub fn replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:2374:        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:2407:        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:2448:        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:2469:        // ledger_writer + replay_full_transition(...) → assert final state_root
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:7020:   348	pub fn replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:8333:   348	pub fn replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:9522:  1217	        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:9555:  1250	        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:9596:  1291	        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:11852:handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:431:pub fn replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:11870:handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:494:| **I-DETHASH** | replay_full_transition(genesis, entries) recovers live state_root | **Bound to FullTransition mode only** (C1); skeleton ChainOnly is necessary-but-not-sufficient |
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:31:| C-1: replay_full_transition accepts only roots; fabricates QState::genesis() | P1: signature changed to `replay_full_transition(genesis: &QState, ...) -> Result<QState, ReplayError>`; caller provides full QState; replay returns reconstructed final QState | transition_ledger.rs replay_full_transition signature + caller updates |
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:546:pub fn replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:1421:        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:1453:        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:1493:        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:1514:        // ledger_writer + replay_full_transition(...) → assert final state_root
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2205:src/bottom_white/ledger/transition_ledger.rs:374:pub fn replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2219:src/bottom_white/ledger/transition_ledger.rs:1249:        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2220:src/bottom_white/ledger/transition_ledger.rs:1281:        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2221:src/bottom_white/ledger/transition_ledger.rs:1321:        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2222:src/bottom_white/ledger/transition_ledger.rs:1342:        // ledger_writer + replay_full_transition(...) → assert final state_root
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2701:   374	pub fn replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2966:  1249	        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2998:  1281	        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:3038:  1321	        let err = replay_full_transition(
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:524:pub fn replay_full_transition(
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:587:| **I-DETHASH** | replay_full_transition(genesis, entries) recovers live state_root | **Bound to FullTransition mode only** (C1); skeleton ChainOnly is necessary-but-not-sufficient |
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:11530:   401	pub fn replay_full_transition(
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:11593:   464	| **I-DETHASH** | replay_full_transition(genesis, entries) recovers live state_root | **Bound to FullTransition mode only** (C1); skeleton ChainOnly is necessary-but-not-sufficient |
src/bottom_white/ledger/transition_ledger.rs:374:pub fn replay_full_transition(
src/bottom_white/ledger/transition_ledger.rs:1249:        let err = replay_full_transition(
src/bottom_white/ledger/transition_ledger.rs:1281:        let err = replay_full_transition(
src/bottom_white/ledger/transition_ledger.rs:1321:        let err = replay_full_transition(
src/bottom_white/ledger/transition_ledger.rs:1342:        // ledger_writer + replay_full_transition(...) → assert final state_root
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:529:pub fn replay_full_transition(
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:592:| **I-DETHASH** | replay_full_transition(genesis, entries) recovers live state_root | **Bound to FullTransition mode only** (C1); skeleton ChainOnly is necessary-but-not-sufficient |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2770:pub fn replay_full_transition(
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2833:| **I-DETHASH** | replay_full_transition(genesis, entries) recovers live state_root | **Bound to FullTransition mode only** (C1); skeleton ChainOnly is necessary-but-not-sufficient |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:13776:   401	pub fn replay_full_transition(
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:13839:   464	| **I-DETHASH** | replay_full_transition(genesis, entries) recovers live state_root | **Bound to FullTransition mode only** (C1); skeleton ChainOnly is necessary-but-not-sufficient |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:16331:   431	pub fn replay_full_transition(
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:431:pub fn replay_full_transition(
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:494:| **I-DETHASH** | replay_full_transition(genesis, entries) recovers live state_root | **Bound to FullTransition mode only** (C1); skeleton ChainOnly is necessary-but-not-sufficient |

exec
/bin/bash -lc "rg -n \"Result<\\(Hash, Hash\\)|genesis_state_root|genesis_ledger_root|fetch_add\\(1, Ordering::SeqCst\\)\" src/bottom_white/ledger/transition_ledger.rs src/state/sequencer.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/sequencer.rs:244:        let submit_id = self.next_submit_id.fetch_add(1, Ordering::SeqCst);
src/bottom_white/ledger/transition_ledger.rs:364:/// **v1.1 C-1 closure**: takes a full `genesis: &QState` (was `genesis_state_root`
src/bottom_white/ledger/transition_ledger.rs:365:/// + `genesis_ledger_root` only). Caller provides the complete genesis state
src/bottom_white/ledger/transition_ledger.rs:485:    genesis_state_root: Hash,
src/bottom_white/ledger/transition_ledger.rs:486:    genesis_ledger_root: Hash,
src/bottom_white/ledger/transition_ledger.rs:488:) -> Result<(Hash, Hash), ReplayError> {
src/bottom_white/ledger/transition_ledger.rs:489:    let mut prev_state_root = genesis_state_root;
src/bottom_white/ledger/transition_ledger.rs:490:    let mut prev_ledger_root = genesis_ledger_root;

exec
/bin/bash -lc "rg -n \"replay_rejects|payload_decode|tx_kind_mismatch|TxKindMismatch|PayloadDecode\" src/bottom_white/ledger/transition_ledger.rs handover -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/ledger/transition_ledger.rs:289:    TxKindMismatch {
src/bottom_white/ledger/transition_ledger.rs:297:    PayloadDecode {
src/bottom_white/ledger/transition_ledger.rs:316:            Self::TxKindMismatch { at, envelope_kind, decoded_kind } => write!(
src/bottom_white/ledger/transition_ledger.rs:320:            Self::PayloadDecode { at, reason } => write!(f, "payload canonical_decode failed at index {at}: {reason}"),
src/bottom_white/ledger/transition_ledger.rs:429:            canonical_decode(&payload_bytes).map_err(|e| ReplayError::PayloadDecode {
src/bottom_white/ledger/transition_ledger.rs:440:            return Err(ReplayError::TxKindMismatch {
src/bottom_white/ledger/transition_ledger.rs:903:    fn replay_rejects_parent_state_tamper() {
src/bottom_white/ledger/transition_ledger.rs:913:    fn replay_rejects_parent_ledger_tamper() {
src/bottom_white/ledger/transition_ledger.rs:925:    fn replay_rejects_ledger_root_tamper() {
src/bottom_white/ledger/transition_ledger.rs:1267:    fn replay_rejects_bad_system_signature() {
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:4692:    fn replay_rejects_parent_state_tamper() {
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:4702:    fn replay_rejects_parent_ledger_tamper() {
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:4714:    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:8839:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:8840:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:8842:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
handover/audits/run_codex_co1_7_impl_bundle_round2_audit.sh:23:| C-3: replay never asserts tx_kind match | P3: NEW ReplayError::TxKindMismatch variant + stage 6.5 assertion + test replay_rejects_tx_kind_mismatch | transition_ledger.rs ReplayError enum + replay_full_transition stage 6.5 + tests |
handover/audits/run_codex_co1_7_impl_bundle_round2_audit.sh:24:| C-3-secondary: decode errors reported as CasMissing | P4: NEW ReplayError::PayloadDecode { at, reason } variant; replay stage 6 distinguishes decode from CAS-miss; test replay_rejects_payload_decode_failure | transition_ledger.rs ReplayError + stage 6 |
handover/audits/run_codex_co1_7_impl_bundle_round2_audit.sh:34:**Q4**: Does P4 (PayloadDecode) cleanly separate decode failure from CAS miss? Test exercises decode failure with non-canonical bytes?
handover/audits/run_codex_co1_7_impl_bundle_round2_audit.sh:46:## Q4 P4 PayloadDecode separation
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:926:    fn replay_rejects_parent_mismatch() {
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:938:    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5064:   377	    fn replay_rejects_parent_mismatch() {
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5076:   389	    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:6488:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:6489:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_mismatch ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND5_AUDIT_2026-04-28.md:4254:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND5_AUDIT_2026-04-28.md:4255:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND5_AUDIT_2026-04-28.md:4256:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND5_AUDIT_2026-04-28.md:7622:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND5_AUDIT_2026-04-28.md:7623:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND5_AUDIT_2026-04-28.md:7625:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND5_AUDIT_2026-04-28.md:10958:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND5_AUDIT_2026-04-28.md:10959:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND5_AUDIT_2026-04-28.md:10960:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:2028:    fn replay_rejects_parent_state_tamper() {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:2038:    fn replay_rejects_parent_ledger_tamper() {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:2050:    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:2393:    fn replay_rejects_bad_system_signature() {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:8856:   871	    fn replay_rejects_parent_state_tamper() {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:8866:   881	    fn replay_rejects_parent_ledger_tamper() {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:8878:   893	    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:9541:  1236	    fn replay_rejects_bad_system_signature() {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:11541:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:11542:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:11543:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:11583:test bottom_white::ledger::transition_ledger::tests::replay_rejects_bad_system_signature ... ok
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:67:**Q-A5** (K2 transplant): parent_ledger_root field added + bound in signing payload + new test. Verify: (a) skeleton tests/replay_rejects_parent_ledger_tamper actually exercises the transplant defense path; (b) signing payload digest includes parent_ledger_root before the signature is computed; (c) any other transplant-vector still open?
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:1219:    fn replay_rejects_parent_state_tamper() {
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:1229:    fn replay_rejects_parent_ledger_tamper() {
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:1241:    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:2211:    fn replay_rejects_parent_mismatch() {
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:2223:    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:6349:   377	    fn replay_rejects_parent_mismatch() {
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:6361:   389	    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:7773:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:7774:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_mismatch ... ok
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:11851:   447	    fn replay_rejects_parent_state_tamper() {
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:11861:   457	    fn replay_rejects_parent_ledger_tamper() {
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:11873:   469	    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:11961:   469	    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:12343:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:12344:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:12345:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND4_AUDIT_2026-04-28.md:4189:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND4_AUDIT_2026-04-28.md:4190:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND4_AUDIT_2026-04-28.md:4191:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND4_AUDIT_2026-04-28.md:7557:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND4_AUDIT_2026-04-28.md:7558:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND4_AUDIT_2026-04-28.md:7560:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
handover/audits/run_codex_co1_7_round2_audit.sh:58:**Q-A5** (K2 transplant): parent_ledger_root field added + bound in signing payload + new test. Verify: (a) skeleton tests/replay_rejects_parent_ledger_tamper actually exercises the transplant defense path; (b) signing payload digest includes parent_ledger_root before the signature is computed; (c) any other transplant-vector still open?
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:33:| C-3: replay never asserts tx_kind match | P3: NEW ReplayError::TxKindMismatch variant + stage 6.5 assertion + test replay_rejects_tx_kind_mismatch | transition_ledger.rs ReplayError enum + replay_full_transition stage 6.5 + tests |
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:34:| C-3-secondary: decode errors reported as CasMissing | P4: NEW ReplayError::PayloadDecode { at, reason } variant; replay stage 6 distinguishes decode from CAS-miss; test replay_rejects_payload_decode_failure | transition_ledger.rs ReplayError + stage 6 |
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:44:**Q4**: Does P4 (PayloadDecode) cleanly separate decode failure from CAS miss? Test exercises decode failure with non-canonical bytes?
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:56:## Q4 P4 PayloadDecode separation
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:124:| **P3** | C-3 | `ReplayError::TxKindMismatch { at, envelope_kind, decoded_kind }` NEW variant; replay stage 6.5 asserts `decoded_typed_tx.tx_kind() == entry.tx_kind`. New test `replay_rejects_tx_kind_mismatch` exercises the case where the signed envelope claims `Verify` but CAS decodes as `Work`. |
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:125:| **P4** | C-3-secondary | `ReplayError::PayloadDecode { at, reason }` NEW variant; replay stage 6 distinguishes decode failure from CAS lookup miss. New test `replay_rejects_payload_decode_failure`. |
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:461:    TxKindMismatch {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:469:    PayloadDecode {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:488:            Self::TxKindMismatch { at, envelope_kind, decoded_kind } => write!(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:492:            Self::PayloadDecode { at, reason } => write!(f, "payload canonical_decode failed at index {at}: {reason}"),
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:601:            canonical_decode(&payload_bytes).map_err(|e| ReplayError::PayloadDecode {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:612:            return Err(ReplayError::TxKindMismatch {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:1075:    fn replay_rejects_parent_state_tamper() {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:1085:    fn replay_rejects_parent_ledger_tamper() {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:1097:    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:1439:    fn replay_rejects_bad_system_signature() {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2187:/bin/bash -lc "rg -n \"replay_full_transition|Result<\\(Hash, Hash\\)|genesis_state_root|genesis_ledger_root|fetch_add\\(1, Ordering::SeqCst\\)|TxKindMismatch|PayloadDecode|canonical_decode|next_logical_t\\.(load|store)|writer_w\\.commit|commit\\(&entry\\)\" src/bottom_white/ledger/transition_ledger.rs src/state/sequencer.rs" in /home/zephryj/projects/turingosv4
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2195:src/bottom_white/ledger/transition_ledger.rs:289:    TxKindMismatch {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2197:src/bottom_white/ledger/transition_ledger.rs:297:    PayloadDecode {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2198:src/bottom_white/ledger/transition_ledger.rs:316:            Self::TxKindMismatch { at, envelope_kind, decoded_kind } => write!(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2199:src/bottom_white/ledger/transition_ledger.rs:320:            Self::PayloadDecode { at, reason } => write!(f, "payload canonical_decode failed at index {at}: {reason}"),
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2207:src/bottom_white/ledger/transition_ledger.rs:429:            canonical_decode(&payload_bytes).map_err(|e| ReplayError::PayloadDecode {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2208:src/bottom_white/ledger/transition_ledger.rs:440:            return Err(ReplayError::TxKindMismatch {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2378:test bottom_white::ledger::transition_ledger::tests::replay_rejects_bad_system_signature ... ok
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2379:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2380:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2381:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2583:/bin/bash -lc 'rg -n "replay_rejects_tx_kind_mismatch|replay_rejects_payload_decode_failure|TxKindMismatch|PayloadDecode|tampered envelope|non-canonical|canonical bytes|decode failure" src/bottom_white/ledger/transition_ledger.rs' in /home/zephryj/projects/turingosv4
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2585:289:    TxKindMismatch {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2587:297:    PayloadDecode {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2588:316:            Self::TxKindMismatch { at, envelope_kind, decoded_kind } => write!(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2589:320:            Self::PayloadDecode { at, reason } => write!(f, "payload canonical_decode failed at index {at}: {reason}"),
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2590:429:            canonical_decode(&payload_bytes).map_err(|e| ReplayError::PayloadDecode {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2591:440:            return Err(ReplayError::TxKindMismatch {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2616:   289	    TxKindMismatch {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2624:   297	    PayloadDecode {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2643:   316	            Self::TxKindMismatch { at, envelope_kind, decoded_kind } => write!(
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2647:   320	            Self::PayloadDecode { at, reason } => write!(f, "payload canonical_decode failed at index {at}: {reason}"),
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2756:   429	            canonical_decode(&payload_bytes).map_err(|e| ReplayError::PayloadDecode {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2767:   440	            return Err(ReplayError::TxKindMismatch {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md:2984:  1267	    fn replay_rejects_bad_system_signature() {
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:4128:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:4129:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:4130:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
handover/audits/CO1_7_IMPL_BUNDLE_DUAL_AUDIT_VERDICT_R1_2026-04-28.md:54:| **P3** | C-3 | `ReplayError::TxKindMismatch { at, envelope_kind, decoded_kind }` NEW variant; replay stage 6.5 asserts `decoded_typed_tx.tx_kind() == entry.tx_kind`. New test `replay_rejects_tx_kind_mismatch` exercises the case where the signed envelope claims `Verify` but CAS decodes as `Work`. |
handover/audits/CO1_7_IMPL_BUNDLE_DUAL_AUDIT_VERDICT_R1_2026-04-28.md:55:| **P4** | C-3-secondary | `ReplayError::PayloadDecode { at, reason }` NEW variant; replay stage 6 distinguishes decode failure from CAS lookup miss. New test `replay_rejects_payload_decode_failure`. |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:1224:    fn replay_rejects_parent_state_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:1234:    fn replay_rejects_parent_ledger_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:1246:    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2313:**Q-A5** (K2 transplant): parent_ledger_root field added + bound in signing payload + new test. Verify: (a) skeleton tests/replay_rejects_parent_ledger_tamper actually exercises the transplant defense path; (b) signing payload digest includes parent_ledger_root before the signature is computed; (c) any other transplant-vector still open?
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:3465:    fn replay_rejects_parent_state_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:3475:    fn replay_rejects_parent_ledger_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:3487:    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:4457:    fn replay_rejects_parent_mismatch() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:4469:    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:8595:   377	    fn replay_rejects_parent_mismatch() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:8607:   389	    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:10019:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:10020:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_mismatch ... ok
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:14097:   447	    fn replay_rejects_parent_state_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:14107:   457	    fn replay_rejects_parent_ledger_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:14119:   469	    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:14207:   469	    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:14589:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:14590:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:14591:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:15773:   447	    fn replay_rejects_parent_state_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:15783:   457	    fn replay_rejects_parent_ledger_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:15795:   469	    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:16510:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:16511:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:16512:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5163:    fn replay_rejects_parent_state_tamper() {
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5173:    fn replay_rejects_parent_ledger_tamper() {
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5185:    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5901:src/bottom_white/ledger/transition_ledger.rs:730:    fn replay_rejects_parent_state_tamper() {
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5902:src/bottom_white/ledger/transition_ledger.rs:740:    fn replay_rejects_parent_ledger_tamper() {
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5903:src/bottom_white/ledger/transition_ledger.rs:752:    fn replay_rejects_ledger_root_tamper() {
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:13216:test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:13217:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:13218:test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok

codex
# Codex CO1.7-impl Bundle Round-2 Audit

## Q1 P1 replay signature
Yes. `replay_full_transition` takes `genesis: &QState` and returns `Result<QState, ReplayError>` at `src/bottom_white/ledger/transition_ledger.rs:374-381`. It clones the full genesis state at `:387`, checks `q.state_root_t` / `q.ledger_root_t` at `:400-405`, passes full `q` to dispatch at `:448-450`, and returns final `q` at `:468`.

## Q2 P2 logical_t ordering
Yes. `apply_one` uses tentative `load + 1` at `src/state/sequencer.rs:300-301`. `writer_w.commit(&entry)?` happens at `:368`; `next_logical_t.store(...)` is only after that at `:370`. On commit error, `?` returns before store. No `next_logical_t.fetch_add` remains; only `next_submit_id.fetch_add` at `:244`.

## Q3 P3 tx_kind match
Implementation yes, test no. `TxKindMismatch` exists at `src/bottom_white/ledger/transition_ledger.rs:289-293`; replay decodes payload then rejects kind mismatch at `:426-444`. But no `replay_rejects_tx_kind_mismatch` test exists in `src`; grep only found existing replay tests, not the claimed mismatch test.

## Q4 P4 PayloadDecode separation
Implementation yes, test no. `PayloadDecode` exists at `src/bottom_white/ledger/transition_ledger.rs:297-300`; CAS lookup maps to `CasMissing` at `:421-424`, while decode failure maps to `PayloadDecode` at `:426-432`. No `replay_rejects_payload_decode_failure` / non-canonical-bytes test exists in `src`.

## Q5 Test status
`cargo test --lib` passes: `237 passed; 0 failed; 1 ignored`.

## Q6 Other stale residue
Residue grep:
`fetch_add(1, Ordering::SeqCst)` only remains for `next_submit_id` at `src/state/sequencer.rs:244`, acceptable.
`Result<(Hash, Hash)>`, `genesis_state_root`, `genesis_ledger_root` remain in `replay_chain_integrity` at `src/bottom_white/ledger/transition_ledger.rs:484-490`; not in `replay_full_transition`, but still literal residue.

## **VERDICT**: CHALLENGE

## Top must-fix (if CHALLENGE)
Add the two claimed tests: re-signed envelope kind mismatch and non-canonical payload decode failure. The code path appears fixed, but the v1.1 claimed test coverage is absent.

## Conviction (low/med/high)
High.
2026-04-28T14:07:45.370132Z ERROR codex_core::session: failed to record rollout items: thread 019dd469-3421-77b1-b15c-c2c091b58d3e not found
tokens used
67,977
# Codex CO1.7-impl Bundle Round-2 Audit

## Q1 P1 replay signature
Yes. `replay_full_transition` takes `genesis: &QState` and returns `Result<QState, ReplayError>` at `src/bottom_white/ledger/transition_ledger.rs:374-381`. It clones the full genesis state at `:387`, checks `q.state_root_t` / `q.ledger_root_t` at `:400-405`, passes full `q` to dispatch at `:448-450`, and returns final `q` at `:468`.

## Q2 P2 logical_t ordering
Yes. `apply_one` uses tentative `load + 1` at `src/state/sequencer.rs:300-301`. `writer_w.commit(&entry)?` happens at `:368`; `next_logical_t.store(...)` is only after that at `:370`. On commit error, `?` returns before store. No `next_logical_t.fetch_add` remains; only `next_submit_id.fetch_add` at `:244`.

## Q3 P3 tx_kind match
Implementation yes, test no. `TxKindMismatch` exists at `src/bottom_white/ledger/transition_ledger.rs:289-293`; replay decodes payload then rejects kind mismatch at `:426-444`. But no `replay_rejects_tx_kind_mismatch` test exists in `src`; grep only found existing replay tests, not the claimed mismatch test.

## Q4 P4 PayloadDecode separation
Implementation yes, test no. `PayloadDecode` exists at `src/bottom_white/ledger/transition_ledger.rs:297-300`; CAS lookup maps to `CasMissing` at `:421-424`, while decode failure maps to `PayloadDecode` at `:426-432`. No `replay_rejects_payload_decode_failure` / non-canonical-bytes test exists in `src`.

## Q5 Test status
`cargo test --lib` passes: `237 passed; 0 failed; 1 ignored`.

## Q6 Other stale residue
Residue grep:
`fetch_add(1, Ordering::SeqCst)` only remains for `next_submit_id` at `src/state/sequencer.rs:244`, acceptable.
`Result<(Hash, Hash)>`, `genesis_state_root`, `genesis_ledger_root` remain in `replay_chain_integrity` at `src/bottom_white/ledger/transition_ledger.rs:484-490`; not in `replay_full_transition`, but still literal residue.

## **VERDICT**: CHALLENGE

## Top must-fix (if CHALLENGE)
Add the two claimed tests: re-signed envelope kind mismatch and non-canonical payload decode failure. The code path appears fixed, but the v1.1 claimed test coverage is absent.

## Conviction (low/med/high)
High.
