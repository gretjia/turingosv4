# Codex CO1.7 transition_ledger Round-3 Audit
**Date**: 2026-04-28
**Target**: spec v1.2 + skeleton v1.2 + system_keypair.rs (closure check)
**Prompt size**: 831196 chars

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
session id: 019dd313-cb6a-7c02-8185-ab3d2d2d4d19
--------
user
# Codex Round-3 Audit — CO1.7 transition_ledger v1.2 (narrow closure check)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-3.

**Mandate**: round-3 closure-only check. v1.2 claims to close the 3 specific must-fix items from your round-2 CHALLENGE plus 1 typo. PASS/PASS unblocks CO1.7 implementation.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## Your round-2 verdict (the items v1.2 must close)

You returned **CHALLENGE** with 3 specific must-fix items:
1. **R2-C3**: actually close C3 in code or stop claiming it is closed. Add `CanonicalMessage::LedgerEntrySigning`, `canonical_digest` arm, and `sign_ledger_entry` to `system_keypair.rs`. Plus skeleton test verifying signature rejection after `parent_ledger_root` mutation.
2. **R2-K3**: fix the head_t/commit return contradiction. Either make `LedgerWriter::commit` return commit `NodeId`, or explicitly defer `head_t` mutation and remove the "CO1.7 owns head_t" claim from spec § 0/§ 3/§ 5.
3. **R2-C2-CAS**: fix the CAS object type mismatch. Add `ObjectType::Transition` to schema.rs, OR change spec § 3 to use an existing CAS `ObjectType`.

You also flagged: typo "8-field" → should be "9-field" in spec.

## What v1.2 ships (per patch log at top of spec)

| Item | v1.2 fix |
|---|---|
| R2-C3 | Wave 4-B additive extension shipped: `CanonicalMessage::LedgerEntrySigning([u8;32])` opaque-digest variant + canonical_digest match arm + new `mod transition_ledger_emitter::sign_ledger_entry`. Skeleton test 9 (`signature_round_trip_and_transplant_defense`) generates real Ed25519 keypair, signs via emitter, verifies; asserts clean verify + transplant fail (K2) + cross-epoch fail (D1). |
| R2-K3 | head_t mutation explicitly deferred to CO1.7.5+ when `Git2LedgerWriter` exists. v1.x ledger owns `ledger_root_t` only. `LedgerWriter::commit` keeps `Hash` return. Spec § 0/§ 3/§ 5 updated; "CO1.7 owns head_t" claim removed. |
| R2-C2-CAS | Spec § 3 changed `ObjectType::Transition` → `ObjectType::ProposalPayload` (existing variant — semantically correct for agent work_tx payloads; no CAS schema extension). |
| R2-typo | Spec § 0 / § 1.1 "8-field" → "9-field". |

## Round-3 closure-only questions

For each of your round-2 must-fix items, judge: **CLOSED / PARTIAL / REGRESSED / NEW-ISSUE**.

**Q-1 (R2-C3 closure)**: verify in code:
- `src/bottom_white/ledger/system_keypair.rs` has `CanonicalMessage::LedgerEntrySigning([u8; 32])` variant
- `canonical_digest()` has match arm `b"LedgerEntrySigning" + digest`
- `pub(crate) mod transition_ledger_emitter` exposes `sign_ledger_entry(keypair, signing_payload_digest)` returning `Result<SystemSignature, KeypairError>`
- Skeleton test 9 (`signature_round_trip_and_transplant_defense`) actually exercises sign + verify roundtrip and the K2/D1 defenses

**Q-2 (R2-K3 closure)**: verify in spec:
- § 0 no longer claims "CO1.7 owns head_t = NodeId(commit_sha)"
- § 3 sequencer pseudocode explicitly does NOT mutate q_w.head_t (deferral comment present)
- § 5 storage backend says head_t deferred to CO1.7.5+
- LedgerWriter trait keeps `Hash` return (not `NodeId`)

**Q-3 (R2-C2-CAS closure)**: verify in spec § 3 sequencer:
- `cas.put(...)` call uses `ObjectType::ProposalPayload` (NOT `ObjectType::Transition`)
- Comment notes the R2 fix

**Q-4 (R2-typo closure)**: verify spec § 0 says "9-field" not "8-field".

**Q-5 (NEW issues from v1.2 patches)**: did v1.2 introduce any new defects?
- Opaque-digest variant in CanonicalMessage — does it correctly preserve typed-sign-only invariant?
- transition_ledger_emitter module is `pub(crate) mod` — is the visibility appropriate?
- Skeleton test cross-epoch defense uses `payload.epoch = SystemEpoch::new(2)` then computes new digest — does this correctly model the threat?
- TR manifest hashes refreshed for system_keypair.rs + transition_ledger.rs + spec — anything else needed?

**Q-6**: independent verification —  `cargo test --lib` reports 199/0 PASS (was 198 pre-v1.2; +1 new test). Verify the structural correctness of test #9.

## Output format

# Codex CO1.7 Round-3 Audit
## Q-1 R2-C3 closure
## Q-2 R2-K3 closure
## Q-3 R2-C2-CAS closure
## Q-4 R2-typo closure
## Q-5 NEW v1.2 issues
## Q-6 Independent verification
## **VERDICT**: PASS / CHALLENGE / VETO
## Top must-fix (if CHALLENGE)
## Conviction (low/med/high)

You MUST PASS if v1.2 closes round-2 blockers without new ones. Round-3 is a closure check, NOT a fresh exploration.

If CHALLENGE: list exactly what to patch in v1.3.

---


# CO1.7 spec v1.2 (current target of round-3)

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

# Type-skeleton v1.2 (current target of round-3)

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
        }
    }
}
impl std::error::Error for ReplayError {}

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
}

```

---

# system_keypair.rs (v1.2 with CanonicalMessage extension + transition_ledger_emitter — verify R2-C3)

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemSignature([u8; SIGNATURE_LEN]);

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

/// TRACE_MATRIX FC1-Sig+FC3-Sig: typed terminal summary transaction emitted on no-accept runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalSummaryTx {
    run_id: String,
    terminal_state_root: [u8; 32],
    rejected_attempt_count: u64,
}

impl TerminalSummaryTx {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a typed terminal summary transaction.
    pub fn new(
        run_id: impl Into<String>,
        terminal_state_root: [u8; 32],
        rejected_attempt_count: u64,
    ) -> Self {
        Self {
            run_id: run_id.into(),
            terminal_state_root,
            rejected_attempt_count,
        }
    }
}

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
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: terminal summary transaction.
    TerminalSummaryTx(TerminalSummaryTx),
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
        CanonicalMessage::TerminalSummaryTx(tx) => {
            h.update(b"TerminalSummaryTx");
            update_len_prefixed(&mut h, tx.run_id.as_bytes());
            h.update(tx.terminal_state_root);
            h.update(tx.rejected_attempt_count.to_be_bytes());
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

/// TRACE_MATRIX FC1-Sig+FC3-Sig: crate-only signing surface for terminal summary emission.
pub(crate) mod terminal_summary_emitter {
    use super::{
        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, EpochRotationProof,
        KeypairError, SystemSignature, TerminalSummaryTx,
    };

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: sign only typed terminal summary transactions.
    pub(crate) fn sign_terminal_summary_tx(
        keypair: &Ed25519Keypair,
        tx: &TerminalSummaryTx,
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(keypair, &CanonicalMessage::TerminalSummaryTx(tx.clone()))
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

# Your round-2 audit (the must-fix items v1.2 must close)

# Codex CO1.7 transition_ledger Round-2 Audit
**Date**: 2026-04-28
**Target**: spec v1.1 + skeleton v1.1 joint artifact
**Prompt size**: 580463 chars

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
session id: 019dd309-cb98-7702-84d0-b2a3aa23606d
--------
user
# Codex Round-2 Audit — CO1.7 transition_ledger v1.1 (post round-1 CHALLENGE)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-2 (running in parallel).

**Mandate**: round 2 closure-verification audit on v1.1 joint artifact (spec + skeleton). Round-1 returned CHALLENGE/CHALLENGE; v1.1 claims to close 11 must-fix + 1 disagreement (D1).

Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## What changed v1 → v1.1 (per patch log at top of spec)

| ID | v1 issue | v1.1 fix claim |
|---|---|---|
| C1 | replay single-mode | Two-mode `ReplayMode::ChainOnly` (skeleton) vs `FullTransition` (CO1.7.5+); I-DETHASH bound to Full only |
| C2 | shipped CasStore::open inits empty in-memory index → cold-replay impossible | New CO1.4-extra atom (separate); v1 doc + ReplayError::CasMissing |
| C3 | signing primitive integration unspecified | LedgerEntrySigningPayload struct + CanonicalMessage::LedgerEntrySigning(_) variant + sign_ledger_entry API |
| K1 | sequencer fetch_add before accept skips logical_t | Dual counter: next_submit_id (submit) + next_logical_t (commit only) |
| K2 | signature did not bind parent_ledger_root | Added field + bound in signing payload + new test |
| K3 | L4/L5 head_t ownership inconsistent | CO1.7 owns ledger_root + commit-chain head_t; CO1.8 owns state_root |
| K4 | spec/skeleton trait mismatch | Spec aligned to skeleton: `commit(&mut self) -> Hash`; iter_from deferred |
| K5 | TxKind::Slash dispatch gap | Slash DROPPED for v4 (CO P2.5 atom) |
| K6 | tx_kind cast without #[repr(u8)] | #[repr(u8)] + explicit discriminants |
| K7 | spec promised 8 tests, skeleton had 6 | 8 tests with stage marker (4 skeleton + 4 CO1.7.5+) |
| G1 | LedgerEntry rigid; no forward-compat slot | extensions: BTreeMap<String, Vec<u8>> bound in signing payload |
| D1 | epoch binding disagreement | Conservative: bound in signing payload (Codex security wins) |

## Your previous (round-1) verdict

You returned **CHALLENGE** with high conviction. Your top 3 must-fix:
1. Sequencer logical time: no skipped accepted-entry; clear submit-vs-accept ordering
2. Redesign ledger signing payload to bind epoch + parent ledger root; exclude only derived root and signature
3. Replay honestly two-mode + ensure full replay can recover CAS payloads cold

You also raised: parent_ledger_root binding (NEW), CAS cold-replay (CasStore index empty), spec/skeleton trait mismatch, TxKind::Slash dispatch gap, #[repr(u8)] missing, conformance test gap.

## Round-2 audit questions

For each round-1 finding, judge **CLOSED / PARTIAL / REGRESSED / NEW-ISSUE**:

**Q-A1** (C1 replay two-mode): Does v1.1 ReplayMode enum + replay_chain_integrity rename + I-DETHASH binding to FullTransition only fully close the trust ambiguity? Or is documentation insufficient?

**Q-A2** (C2 CAS cold-replay): Is CO1.4-extra atom plan acceptable, or does spec need to ship CAS persistence in CO1.7 itself? Is `ReplayError::CasMissing` adequate when the dependent atom hasn't shipped?

**Q-A3** (C3 signing integration): Does `LedgerEntrySigningPayload` + `CanonicalMessage::LedgerEntrySigning` extension correctly close DIV-1? Verify: (a) the canonical_digest method binds the right 9 fields with deterministic byte layout; (b) the sign API extension is additive (Wave 4-B not breaking); (c) the forward-compat clause for future ledger-side variants is sound.

**Q-A4** (K1 sequencer): Is dual-counter design (next_submit_id at submit, next_logical_t at commit) correct? Verify spec § 3 apply_one ordering: stage 4 logical_t assignment AFTER stage 2 dispatch_transition success. Does anything else break (e.g., does submit() returning submit_id without logical_t still satisfy I-LOGTIME ordering for the agent's perspective)?

**Q-A5** (K2 transplant): parent_ledger_root field added + bound in signing payload + new test. Verify: (a) skeleton tests/replay_rejects_parent_ledger_tamper actually exercises the transplant defense path; (b) signing payload digest includes parent_ledger_root before the signature is computed; (c) any other transplant-vector still open?

**Q-A6** (K3 L4/L5 boundary): Verify spec § 3 apply_one no longer mutates head_t via from_state_root. Verify spec § 5 says head_t = NodeId(commit_sha) only. Is the boundary documentation sufficient, or does CO1.8 need a stub trait reference now?

**Q-A7** (K4 trait): commit(&mut self) -> Hash matches skeleton. iter_from deferred. Does this leave a hole for FullTransition replay? (it does — but flagged as CO1.7.5+ stage, OK?)

**Q-A8** (K5 Slash drop): Verify TxKind has 7 variants (no Slash); dispatch enum-match has no Slash arm. Is "drop until CO P2.5" cleaner than "stub now + impl later"? Any forward-compat hazard?

**Q-A9** (K6 #[repr(u8)]): Verify enum has #[repr(u8)] + Work=0..TerminalSummary=6. Verify canonical_digest uses `tx_kind as u8` safely. Any related serde-derive concern (TxKind has no serde derive in v1.1)?

**Q-A10** (K7 conformance): 8 tests now (4 skeleton + 4 CO1.7.5+). Verify the 4 skeleton-stage tests are actually present in skeleton (not the spec only). The CO1.7.5+ stage tests are deferred — is that OK, or should v1.1 ship at least stubbed unimplemented!() test functions?

**Q-A11** (G1 extensions): BTreeMap<String, Vec<u8>> field added; bound in signing payload via length-prefix iteration. Is the canonical_digest extension-binding correct (prevents collision attacks via length-prefix)? Forward-compat: when a v4.x feature populates extensions, will old verifiers reject the new entries? (yes — by design — but is this safe for upgrade?)

**Q-A12** (D1 resolution): epoch bound in signing payload (your security argument won). Verify spec § 1.1 + skeleton both have epoch in canonical_digest. Any cross-epoch transplant residual still open? (e.g., what if old-epoch private key compromised AND attacker can re-sign with current pubkey claim?)

## NEW v1.1 issues (independent of round-1)

**Q-B1**: review the new sequencer apply_one ordering in spec § 3. Specifically:
- Stage 3 puts payload to CAS BEFORE stage 4 assigns logical_t. The tentative CAS metadata `created_at_logical_t = self.next_logical_t.load() + 1` is racy — what if multiple submitters succeed before any commit?
- The CAS put's `creator` field is `format!("sequencer-{}", self.epoch.get())` — does this leak the epoch identity to CAS metadata? Is that intentional?

**Q-B2**: § 1.2 forward-compat clause says "future ledger-side variants add new CanonicalMessage::*". But CanonicalMessage is in shipped Wave 4-B code; does v4 vs v4.1 boundary policy permit any module to add variants, or only ledger module?

**Q-B3**: § 4 ReplayError has 8 variants now (4 ChainOnly + 4 FullTransition). Are the FullTransition-mode-only variants reachable from ChainOnly mode? If so → invalid state. Should ReplayError be split into 2 enums per mode?

**Q-B4**: skeleton's `to_signing_payload(&self) -> LedgerEntrySigningPayload` clones `extensions`. For very large extensions (multi-KB blobs), this is O(N) work per replay step. Is this an acceptable cost, or should `to_signing_payload` return a borrowed view?

## Independent verification

**Q-C1**: review `cargo test --lib bottom_white::ledger::transition_ledger::` output (8 tests PASS reported in commit message). Are the test names + assertions structurally consistent with the closures claimed?

**Q-C2**: spot-check the spec § 1 LedgerEntry struct vs skeleton struct: are field counts (11), order, types all matching?

**Q-C3**: TR manifest update — refreshed hashes for spec + skeleton. Anything else needs TR tracking that wasn't?

## Output format

# Codex CO1.7 Round-2 Audit
## Q-A1..A12 Round-1 closure judgments (CLOSED / PARTIAL / REGRESSED / NEW-ISSUE)
## Q-B1..B4 New v1.1 issues
## Q-C1..C3 Independent verification
## **VERDICT**: PASS / CHALLENGE / VETO
## Top must-fix (if CHALLENGE) — be specific; cite spec § + skeleton line
## Conviction (low/med/high)

You MUST PASS if v1.1 closes round-1 blockers without new ones. The audit gate exists to catch real defects, not to refuse on principle. Do NOT challenge on issues already addressed or on style preferences.

If CHALLENGE: list exactly what to patch in v1.2.
If VETO: explain why a foundational design flaw remains (round-1 issued no VETO, so this would be a regression).
If PASS: explicit GO for CO1.7 implementation start (CO1.7.5 etc.).

---


# CO1.7 spec v1.1 (current target of round-2 audit)

# CO1.7 Transition Ledger v1.1 — Round-1 closure

**Status**: v1.1 — round-1 dual external audit (Codex + Gemini) returned CHALLENGE/CHALLENGE; this version closes 11 must-fix items, awaiting round-2.
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
- **LedgerEntrySigningPayload**: the 8-field bytes-on-the-wire that the system keypair actually signs (distinct from LedgerEntry-the-stored-record).
- **LedgerRoot computation**: deterministic Merkle accumulation over signed digests; this is the value of `Q_t.ledger_root_t`.
- **Sequencer**: per-(runtime_repo, run_id) single-writer instance enforcing § 5.2.1 (dual-counter `submit_id`/`logical_t`, submission-order serialization, post-commit `logical_t` assignment).
- **append(parent_root, signing_digest)**: pure function returning the new ledger_root.
- **replay (two-mode)**: `ChainOnly` (chain integrity; skeleton-stage; v1) vs `FullTransition` (rerun pure transitions + verify state_root + verify signatures; CO1.7.5+; THE I-DETHASH witness).
- **Storage backend**: git2-rs commit chain (built on CO1.4 CAS); each LedgerEntry = one git commit on `refs/transitions/main`; commit_sha is the canonical `head_t`.
- **CanonicalMessage extension**: extends shipped `CanonicalMessage` enum with `LedgerEntrySigning(LedgerEntrySigningPayload)` variant; new sign API `keypair.sign_ledger_entry(payload)`.

### Out of scope (handled by other atoms)
- WorkTx / VerifyTx / ChallengeTx / ReuseTx / FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx schemas — frozen in `STATE_TRANSITION_SPEC § 1`.
- step_transition / verify_transition / challenge_transition logic — frozen in `STATE_TRANSITION_SPEC § 3`.
- system_keypair signing primitives — done @ CO1.7.0a-f; CO1.7 only adds a typed extension.
- L5 materializer (state_root computation) — deferred to **CO1.8**. **K3 boundary**: CO1.7 owns `ledger_root_t` + `head_t`; CO1.8 owns `state_root_t`. Sequencer does NOT mutate `state_root_t` directly; it accepts `q_next.state_root_t` as returned by the transition function.
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

### § 1.2 CanonicalMessage extension (per C3)

CO1.7 extends shipped `system_keypair::CanonicalMessage` with one new variant:

```rust
// In src/bottom_white/ledger/system_keypair.rs (additive Wave 4-B extension):
pub enum CanonicalMessage {
    RejectedAttemptSummary(RejectedAttemptSummary),  // existing
    TerminalSummaryTx(TerminalSummaryTx),            // existing
    EpochRotationProof(EpochRotationProof),          // existing
    LedgerEntrySigning(LedgerEntrySigningPayload),   // NEW (C3)
}

// canonical_digest() in system_keypair.rs adds match arm:
//   CanonicalMessage::LedgerEntrySigning(payload) => {
//       h.update(b"LedgerEntrySigning");
//       h.update(payload.canonical_digest().0);
//   }

// New typed sign API (added to Ed25519Keypair impl block):
impl Ed25519Keypair {
    pub fn sign_ledger_entry(&self, payload: &LedgerEntrySigningPayload, epoch: SystemEpoch) -> SystemSignature;
}
```

**Forward-compat clause** (per Gemini Q4 + the audit response): if v4.x adds new ledger-side message variants, they MUST add new `CanonicalMessage::*` variants (NOT extend `LedgerEntrySigningPayload` in-place). v4-shipped extensions go in the `LedgerEntry::extensions` BTreeMap (G1) which IS bound in this signed digest.

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
            ObjectType::Transition,
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

        // 6. **C3 NEW SIGN API**: typed sign through CanonicalMessage extension
        let system_signature = self.keypair.sign_ledger_entry(&signing_payload, self.epoch);

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
        // **K3**: do NOT mutate q_w.head_t here. CO1.7's ownership is `ledger_root_t` only;
        // `head_t = NodeId(commit_sha)` is set by `Git2LedgerWriter::commit` returning the
        // commit_sha through the writer's `Hash` return + sequencer assigning to head_t
        // under the same write lock. (Skeleton uses InMemoryLedgerWriter; head_t mutation
        // path is CO1.7.5+ wiring concern.)

        Ok(entry)
    }
}
```

**Why dual counter (K1)**: rejection of a submission must NOT consume a logical_t, because (a) skeleton's `InMemoryLedgerWriter::commit` enforces `expected_logical_t = len + 1` and would reject a gap; (b) replay enforces `entry.logical_t == (i+1)` and would reject a gap. Submitter IDs (`submit_id`) are returned from `submit()` immediately for receipt; logical_t is observable only on the committed entry.

**Why no head_t mutation in apply_one (K3)**: CO1.7 owns `ledger_root_t` and the commit-chain `head_t`; CO1.8 (L5 materializer) owns `state_root_t` mutation. Sequencer accepts `q_next.state_root_t` as the transition function returns it; sequencer does NOT call `NodeId::from_state_root(...)`.

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
- **K3**: `head_t = NodeId(commit_sha)` is the canonical convention. `NodeId::from_state_root(...)` is NOT used by L4. (q_state.rs:54 keeps the helper for cross-reference but L4 sequencer does NOT call it.)
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

# Type-skeleton v1.1 (current target of round-2 audit)

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
        }
    }
}
impl std::error::Error for ReplayError {}

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
}

```

---

# Your round-1 audit verdict (the must-fix items v1.1 must close)

# Codex CO1.7 transition_ledger Round-1 Audit
**Date**: 2026-04-28
**Target**: spec v1 + type-skeleton joint artifact
**Prompt size**: 175543 chars

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
session id: 019dd2f6-3efc-70d1-8ad0-5ff9f9fe8988
--------
user
# Codex Adversarial Audit — CO1.7 transition_ledger spec v1 + type-skeleton (Round 1)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-1 (running in parallel).

**Mandate**: round 1 dual external audit on the **joint artifact** (spec v1 + Rust type-skeleton). Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator (Claude generated; you review). Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

**Why a joint artifact, not paper-only**: per memory `feedback_smoke_before_batch`, plan audit ≠ runtime check. Skeleton smoke already caught 1 spec bug + 5 DIV cataloged before this audit. Paper-review-only would have missed those.

## What you're reviewing

1. **Spec doc**: `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` (~440 lines, 15 sections)
2. **Type skeleton**: `src/bottom_white/ledger/transition_ledger.rs` (~370 lines incl. 6 inline tests, 196/0 lib PASS, TR-tracked)
3. **Companion specs (frozen)**: STATE_TRANSITION_SPEC v1.4, SYSTEM_KEYPAIR_SECURITY_v1, WP v2.2 § 5.L4

## What is at stake

- **PASS** unblocks CO1.7.5 (step_transition body) + CO1.8 (L5 materializer) + CO1.9 (L6 signal indices) + Git runtime_repo wiring + economy 6 transition fn instantiation
- **CHALLENGE** triggers spec v1.1 patch round
- **VETO** blocks CO1.7 implementation entirely until major rework

## Round 1 audit questions

**Q-A. DIV-1..5 validity**: are the 5 cataloged spec ↔ shipped-code divergences real, or did the skeleton mis-read the actual API?
- DIV-1 LedgerEntry not in CanonicalMessage 3-variant enum → real or strawman?
- DIV-2 Q_t mutation API absent → does CO P2.x economy genuinely block? Workaround?
- DIV-3 epoch field missing in spec v1 § 1 → skeleton fix correct? Anything else missing?
- DIV-4 CasReader trait → narrowed to LedgerCasView; trait segregation defensible?
- DIV-5 CasStore::put 5-param signature → sequencer responsibility correctly placed?

**Q-B. Q9 spec bug closure**: the smoke caught canonical_digest_unsigned circular dependency on resulting_ledger_root. Skeleton's fix (digest covers 7 fields, EXCLUDES resulting_ledger_root AND system_signature). Is the exclusion list complete? Should `epoch` also be excluded if it's set by sequencer not the agent? What about `tx_payload_cid` if CAS put is non-deterministic across implementations?

**Q-C. Sequencer correctness**: spec § 3 sequencer pseudocode (apply_one). Issues to check:
1. RwLock pattern — read snapshot, compute pure, then exclusive write — correct? Concurrent readers between snapshot and commit could observe inconsistent (Q_t, ledger_root_t) — race?
2. AtomicU64::fetch_add(1, SeqCst) for logical_t — sufficient for I-LOGTIME monotonicity? Failure cases?
3. `keypair.sign_entry(entry)` is a placeholder; the actual sign primitive needs to ride CanonicalMessage (DIV-1). What's the correct API to expose?
4. apply_one builds LedgerEntry with resulting_ledger_root computed BEFORE knowing system_signature; is this ordering forced by Q9, and does it open any attack vector (e.g., signature does not bind ledger_root)?

**Q-D. Replay completeness**: skeleton replay_chain_integrity does parent_state_root + ledger_root chain check only. It does NOT re-run pure transitions to independently verify resulting_state_root (deferred per DIV-2 / CO1.7.5+).
- Is partial replay an acceptable v1 deliverable, OR is it a I-DETHASH violation by construction?
- If partial: is the spec sufficiently honest about what replay v1 does NOT verify?
- Should v1.1 add a "trust mode" flag distinguishing chain-only-replay vs full-transition-replay?

**Q-E. Atom scope creep**: spec § 0 says "out of scope: L5 materializer (CO1.8), L6 signal indices (CO1.9), MetaTx full schema (v4.1)". But § 3 sequencer.apply_one already references state_root_t materialization (`q_w.head_t = NodeId::from_state_root(...)`). Where is the L5 boundary?
- Is the L4/L5 line drawn correctly or does CO1.7 implicitly need a L5 stub?
- Does this create a CO1.7 ↔ CO1.8 ordering hazard (each blocks the other)?

**Q-F. STEP_B disposition**: spec § 9 claims "no STEP_B parallel-branch ceremony required" because transition_ledger.rs + sequencer.rs are NEW files. Verify against `STEP_B_PROTOCOL.md` + memory `feedback_step_b_protocol`. Are there indirect modifications (e.g., changes to bus.rs/kernel.rs/wal.rs that the sequencer integration WILL force later)?

**Q-G. Open Q1-Q7 (original spec) + Q8-Q11 (skeleton-found) judgment**:
- Q1 SubmissionQueue type (tokio mpsc / crossbeam / std mpsc) — your recommendation?
- Q4 system_signature placement (inside struct vs sidecar tuple) — your recommendation?
- Q5 dispatch via enum-match vs MetaTransitionInterface trait — your recommendation?
- Q7 genesis ledger_root_t (Hash::ZERO vs sha256 of genesis_payload.toml) — your recommendation?
- Q8 CanonicalMessage extension (a) vs sibling sign primitive (b) — your recommendation?
- Q11 (you may add): are there NEW open Qs not in spec § 11 that should be?

**Q-H. New defects**: independent of catalog, what does the joint artifact still get wrong?
- Type errors that cargo check missed (rare but possible if behind a cfg)?
- Spec ↔ skeleton inconsistencies?
- Missing invariants — anything from STATE_TRANSITION_SPEC § 4 (27 invariants) that CO1.7 should enforce but skeleton/spec doesn't?
- Conformance tests: skeleton has 6; spec § 7 promises 8; what's the gap?

**Q-I. Implementation gating**: assuming all your CHALLENGEs are addressed in v1.1, is the joint artifact implementable end-to-end (i.e., `cargo test --lib transition_ledger` will pass with stubs filled in by CO1.7.5+)? Specific blockers to call out.

## Output format

# Codex CO1.7 Round-1 Audit
## Q-A DIV-1..5 validity
## Q-B Q9 spec bug closure
## Q-C Sequencer correctness
## Q-D Replay completeness
## Q-E Atom scope creep
## Q-F STEP_B disposition
## Q-G Open Q recommendations
## Q-H New defects
## Q-I Implementation gating
## **VERDICT**: PASS / CHALLENGE / VETO
## Top 3 must-fix (if CHALLENGE)
## Conviction (low/med/high)

Be rigorous. Cite spec line numbers + skeleton line numbers when calling defects. Do NOT pass on principle; do NOT veto on principle. Real defects = CHALLENGE; foundational design flaw = VETO; clean joint artifact = PASS.

---


# CO1.7 spec v1 (target of audit)

# CO1.7 Transition Ledger v1 — DRAFT outline

**Status**: DRAFT outline awaiting round-1 dual external audit (Codex + Gemini).
**Author**: ArchitectAI (Claude); session 2026-04-28.
**Supersedes**: none (first cut).
**Companion specs** (frozen, read first):
- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — typed schemas + step_transition pseudocode + 27 invariants (round-4 PASS/PASS)
- `SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md` — runtime keypair lifecycle (CO1.7.0a-f, done @ Wave 4-B)
- `META_TRANSITION_INTERFACE_v1_2026-04-27.md` — trait pattern for L4 acceptance (deferred runtime to v4.1)
- `TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md` § 5.L4 (line 365-389) — ChainTape Layer 4 axioms

**Single sentence**: implement the L4 transition_ledger module so that `ledger::append(parent_root, tx) → new_root` (called from § 3 transition pseudocode) is real code, the L4 sequencer (§ 5.2.1) is real code, and `Q_t.ledger_root_t` is no longer a placeholder.

---

## § 0 Scope

### In scope
- **LedgerEntry schema**: the canonical envelope wrapping each typed transition (WorkTx / VerifyTx / ChallengeTx / ReuseTx / FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx / SlashTx) before it is appended to L4
- **LedgerRoot computation**: deterministic Merkle accumulation over the entry sequence; this is the value of `Q_t.ledger_root_t`
- **Sequencer**: per-(runtime_repo, run_id) single-writer instance enforcing § 5.2.1 (atomic logical_t, submission-order serialization, post-step_transition commit)
- **append(parent_root, ledger_entry)**: pure function returning the new ledger_root (no I/O at this layer; storage commit is sequencer's job)
- **replay(genesis_root, [ledger_entry])**: deterministic replay producing final state_root; the witness for I-DETHASH
- **Storage backend**: git2-rs commit chain (built on CO1.4 CAS); each LedgerEntry = one git commit on `refs/transitions/main`

### Out of scope (handled by other atoms)
- WorkTx / VerifyTx / ChallengeTx schemas — frozen in `STATE_TRANSITION_SPEC § 1`
- step_transition / verify_transition / challenge_transition logic — frozen in `STATE_TRANSITION_SPEC § 3`
- system_keypair signing — done @ CO1.7.0a-f
- L5 materializer (state_root computation) — deferred to **CO1.8** (separate atom)
- L6 signal indices — deferred to **CO1.9**
- AttributionEngine DAG — deferred to CO P2.4.0 spike (Inv 8 design)
- MetaTx full schema — v4.1 only; v4 emits `MetaProposalDraft` to L3 CAS, not L4

### What this spec is NOT replacing
- `src/ledger.rs` (legacy, top-level) is retired in **CO1.1.5 (kernel.rs split)**; CO1.7 lives at `src/bottom_white/ledger/transition_ledger.rs` (NEW). No STEP_B parallel-branch ceremony required (new module, not restricted file).

---

## § 1 LedgerEntry schema

```rust
/// TRACE_MATRIX FC2-Append (FC2 transition machinery):
///   canonical envelope appended to L4 once step_transition succeeds.
///
/// One LedgerEntry per accepted transition, regardless of TxKind.
/// Genesis state has zero LedgerEntries; ledger_root_t = Hash::ZERO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntry {
    /// Monotonic counter from sequencer; starts at 1 per genesis.
    /// Sequencer guarantees: distinct entries have distinct logical_t (§ 5.2.6).
    pub logical_t: u64,                          //  1

    /// Parent state_root before this transition. MUST equal the
    /// resulting_state_root of the entry at logical_t-1 (or Hash::ZERO at logical_t=1).
    pub parent_state_root: Hash,                 //  2

    /// Discriminator; payload schema depends on this.
    pub tx_kind: TxKind,                         //  3

    /// CAS handle (CO1.4) to canonical-serialized payload (WorkTx / VerifyTx / ...).
    /// Payload itself is NOT inlined — kept in CO1.4 CAS to bound LedgerEntry size.
    pub tx_payload_cid: Cid,                     //  4

    /// Resulting state_root after step_transition applied.
    /// Used by I-DETHASH replay test.
    pub resulting_state_root: Hash,              //  5

    /// Resulting ledger_root after this entry is folded in.
    /// Convention: ledger_root_{t+1} = sha256(ledger_root_t || canonical_digest(LedgerEntry_t))
    pub resulting_ledger_root: Hash,             //  6

    /// Wall-clock-free timestamp; derived from sequencer logical_t (NOT system time).
    /// Bound to logical_t at sequencer commit; runtime layer does NOT mutate this field.
    pub timestamp_logical: u64,                  //  7

    /// System runtime keypair signature over canonical_digest of fields 1-7.
    /// Distinct from the agent_signature inside tx_payload (§ 1, agent self-sign).
    /// System signature attests "sequencer accepted this entry at this logical_t".
    pub system_signature: SystemSignature,       //  8
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TxKind {
    Work,              // WorkTx    (§ 1.2)
    Verify,            // VerifyTx  (§ 1.3)
    Challenge,         // ChallengeTx (§ 1.3)
    Reuse,             // ReuseTx   (§ 1.3)
    FinalizeReward,    // claim window expired clean → reward + stake return (§ 3.4)
    TaskExpire,        // task deadline reached unsolved → bounty refund (§ 3.6)
    TerminalSummary,   // run end without acceptance (§ 1.5 + § 3.7)
    Slash,             // (post-CO P2.5) ChallengeCourt slashing event
}
```

**Why an envelope (vs. inlining payload)**:
1. **Bounded entry size**: payloads vary widely (12-field WorkTx vs. 6-field ReuseTx). CAS handle keeps LedgerEntry ~200B regardless.
2. **Storage backend reuse**: CO1.4 CAS already provides addressable blob storage; no second blob layer needed.
3. **Replay separation**: replay reads only LedgerEntry chain to validate I-DETHASH; full payload retrieval is on-demand.

---

## § 2 Module layout

```
src/bottom_white/ledger/
├── mod.rs                       (re-exports; existing — extends with `pub mod transition_ledger`)
├── system_keypair.rs            (existing, CO1.7.0a-f, Wave 4-B)
└── transition_ledger.rs         (NEW, this atom)

src/state/
├── mod.rs                       (existing)
├── q_state.rs                   (existing; ledger_root_t field present at line 317 — CO1.7 fills the placeholder)
└── sequencer.rs                 (NEW, this atom)
```

**Crate boundary**: `transition_ledger` is in `bottom_white::ledger` because it is a tool layer (storage); `sequencer` is in `state::` because it touches Q_t mutation. Sequencer DEPENDS ON ledger; ledger does NOT depend on sequencer (DAG: state → bottom_white::ledger → CO1.4 CAS).

---

## § 3 Sequencer

```rust
/// TRACE_MATRIX § 5.2.1 — L4 sequencer; single-writer per (runtime_repo, run_id).
pub struct Sequencer {
    /// Atomic monotonic counter (§ 5.2.6 tie-break canonical source).
    next_logical_t: AtomicU64,

    /// Submission queue; mpsc-style. Submission order = arrival order at the queue head.
    /// Async completion order does NOT matter (§ 5.2.1 step 4).
    queue: SubmissionQueue<TypedTx>,

    /// Reference to ledger writer (storage backend).
    ledger_writer: Arc<dyn LedgerWriter>,

    /// Reference to system keypair for entry signing (CO1.7.0a-f).
    keypair: Arc<SystemKeyPair>,

    /// Reference to predicate + tool registries (read-only at this layer).
    predicate_registry: Arc<PredicateRegistry>,
    tool_registry: Arc<ToolRegistry>,

    /// Current Q_t snapshot. Held under exclusive write-lock during transition apply.
    q: RwLock<QState>,
}

impl Sequencer {
    /// External entry point for any agent / runtime caller.
    /// Returns the submitted tx's logical_t + tx_id (deterministic from logical_t, agent_id, payload_hash).
    pub fn submit(&self, tx: TypedTx) -> SubmissionReceipt;

    /// Driver loop: drain queue, run transition, append entry. Single-threaded internally.
    /// Executor is implementation-detail (tokio task / std thread); spec does NOT mandate.
    pub async fn run(&self) -> Result<(), SequencerError>;

    /// Per-tx critical section (called by run()):
    fn apply_one(&self, tx: TypedTx) -> Result<LedgerEntry, TransitionError> {
        // 1. Assign logical_t (atomic increment)
        let logical_t = self.next_logical_t.fetch_add(1, Ordering::SeqCst);

        // 2. Snapshot Q_t under read lock (no mutation yet)
        let q_snapshot = self.q.read().clone();

        // 3. Dispatch to the correct pure transition function (§ 3, § 3.1, § 3.2, ...)
        let (q_next, signals) = match tx {
            TypedTx::Work(work_tx)        => step_transition(&q_snapshot, &work_tx, &self.predicate_registry, &self.tool_registry)?,
            TypedTx::Verify(verify_tx)    => verify_transition(&q_snapshot, &verify_tx, &self.predicate_registry)?,
            TypedTx::Challenge(chal_tx)   => challenge_transition(&q_snapshot, &chal_tx, &self.predicate_registry)?,
            TypedTx::Reuse(reuse_tx)      => reuse_transition(&q_snapshot, &reuse_tx, &self.tool_registry)?,
            TypedTx::FinalizeReward(_)    => finalize_reward_transition(/* … */)?,
            TypedTx::TaskExpire(_)        => task_expire_transition(/* … */)?,
            TypedTx::TerminalSummary(_)   => emit_terminal_summary(/* … */)?,
        };

        // 4. Compute ledger_root via append()
        let payload_cid = self.cas.put_canonical(&tx)?;
        let entry = LedgerEntry {
            logical_t,
            parent_state_root: q_snapshot.state_root_t,
            tx_kind: TxKind::from_typed(&tx),
            tx_payload_cid: payload_cid,
            resulting_state_root: q_next.state_root_t,
            resulting_ledger_root: append(&q_snapshot.ledger_root_t, /* unsigned-stub */),
            timestamp_logical: logical_t,
            system_signature: SystemSignature::placeholder(),  // filled in step 5
        };
        let signed_entry = self.keypair.sign_entry(entry);

        // 5. Acquire write lock; commit to storage; mutate Q_t
        let mut q_w = self.q.write();
        self.ledger_writer.commit(&signed_entry)?;
        *q_w = q_next;
        q_w.ledger_root_t = signed_entry.resulting_ledger_root;
        q_w.head_t = NodeId::from_state_root(q_w.state_root_t);

        Ok(signed_entry)
    }
}
```

**Why a single sequencer**: enforces I-DET, I-LOGTIME, I-FINALIZE-BATCH-ORDER, I-FINALIZE-EXCLUSIVE without needing per-transition synchronization. Submission concurrency is handled by the queue; execution concurrency is zero (serial).

**What § 5.2.7 leaves to implementation**: queue type (mpsc / lock-free / mutex+VecDeque), executor (tokio / std::thread), back-pressure policy. CO1.7 v1 picks tokio mpsc (matches existing kernel runtime). Round-1 audit may push back.

---

## § 4 append() + replay()

```rust
/// Pure. Same (parent_root, entry) → byte-identical new_root.
/// No I/O, no clock, no env.
pub fn append(parent_root: &Hash, entry_digest: &Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(parent_root.0);
    hasher.update(entry_digest.0);
    Hash::from_bytes(hasher.finalize().into())
}

/// Replay a sequence of LedgerEntries from genesis. Returns final (state_root, ledger_root).
/// Used by I-DETHASH conformance test + cold-boot recovery.
pub fn replay(
    genesis: &QState,
    entries: &[LedgerEntry],
    payloads: &dyn CasReader,
) -> Result<QState, ReplayError> {
    let mut q = genesis.clone();
    for (i, entry) in entries.iter().enumerate() {
        // 1. Validate parent_state_root chain
        if entry.parent_state_root != q.state_root_t {
            return Err(ReplayError::ParentMismatch { at: i });
        }

        // 2. Validate logical_t monotonicity
        if entry.logical_t != (i as u64) + 1 {
            return Err(ReplayError::LogicalTGap { at: i, expected: (i as u64) + 1, got: entry.logical_t });
        }

        // 3. Verify system_signature (rejects forgeries from non-runtime sources)
        if !verify_system_signature(&entry.system_signature, &entry.canonical_digest_unsigned()) {
            return Err(ReplayError::BadSignature { at: i });
        }

        // 4. Re-fetch payload from CAS, re-run pure transition, compare result
        let payload = payloads.get(&entry.tx_payload_cid)?;
        let typed_tx = TypedTx::deserialize_canonical(&payload)?;
        let (q_next, _) = dispatch_transition(&q, &typed_tx)?;
        if q_next.state_root_t != entry.resulting_state_root {
            return Err(ReplayError::StateRootMismatch { at: i });
        }

        // 5. Re-fold ledger_root, compare
        let recomputed_ledger_root = append(&q.ledger_root_t, &entry.canonical_digest_unsigned());
        if recomputed_ledger_root != entry.resulting_ledger_root {
            return Err(ReplayError::LedgerRootMismatch { at: i });
        }

        q = q_next;
        q.ledger_root_t = entry.resulting_ledger_root;
    }
    Ok(q)
}
```

**Replay is the I-DETHASH witness**: any cold restart MUST be able to call `replay(genesis, ledger_entries, cas) → q` and get the same state_root the live system has. If it diverges, either (a) the spec was implemented non-deterministically, or (b) the ledger was tampered with — both are I-DETHASH violations.

---

## § 5 Storage backend

**Choice**: git2-rs commit chain (Path B substrate, ratified per Const Art 0.4 + WP § 5.L4).

**Mapping**:
- One `LedgerEntry` = one git commit on `refs/transitions/main`
- Commit message = canonical-serialized `LedgerEntry` (bincode v2)
- Commit tree = `(payload_cid_blob, state_root_marker, signature_blob)`
- `head_t = NodeId(commit_sha)` (Q_t § 1.1 line 47-49 already implements `NodeId::from_state_root`)
- Genesis: `refs/transitions/main` is created at the empty-tree commit corresponding to `genesis_payload.toml` (CO1.0)

**LedgerWriter trait**:

```rust
pub trait LedgerWriter: Send + Sync {
    /// Commit a signed LedgerEntry to storage. Atomic: either commit + ref update both succeed,
    /// or neither does (git2-rs txn semantics).
    fn commit(&self, entry: &LedgerEntry) -> Result<NodeId, LedgerWriterError>;

    /// Read entry at a specific logical_t (1-indexed).
    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;

    /// Iterate entries in logical_t order from `from` (inclusive).
    fn iter_from(&self, from: u64) -> Box<dyn Iterator<Item = Result<LedgerEntry, LedgerWriterError>> + '_>;
}
```

**Implementation**: `Git2LedgerWriter` (built on existing CO1.4 `git2-rs` CAS layer). Uses `repo.commit(...)` with parents = [previous head]. Ref update via `repo.reference("refs/transitions/main", new_oid, force=false, log_msg)`.

**Why git2-rs not gix**: Const Art 0.4 ratified path B (gix→git2-rs pivot per CO1.3.1 spike 8/8 PASS).

---

## § 6 Invariants enforced by CO1.7

| ID | Invariant | Enforced where in CO1.7 |
|---|---|---|
| **I-DET** | Same (Q_t, tx) → byte-identical (Q_{t+1}, signals) | sequencer.apply_one stages 3-4 (pure step_transition + deterministic append) |
| **I-DETHASH** | replay(genesis, ledger_entries) recovers live state_root | replay() + conformance `tests/q_state_reconstruct.rs` |
| **I-LOGTIME** | timestamp_logical strictly monotonic; no wall clock | sequencer.apply_one stage 1 (atomic fetch_add); LedgerEntry has no wall-clock field |
| **I-FINALIZE-BATCH-ORDER** | When N claims expire same logical_t, finalize order = `(expires_at_logical ASC, claim_id ASC)` | sequencer enqueues finalize tx in this order before resuming work tx; per § 5.2.3 |
| **I-FINALIZE-EXCLUSIVE** | finalize_reward_tx and slash_tx mutually exclusive per claim | sequencer's serial dispatch (no concurrent finalize possible) |
| **I-NOSIDE** | step_transition reads only (q, tx, registries) | append() and replay() are pure; sequencer.apply_one isolates I/O to step 5 (commit) |
| **I-NOENV** | step_transition dependency tree has no `std::env` access | grep test in CO1.7 module — already enforced by CLAUDE.md hardcoded-config rule (C-027) |
| **I-NORANDOM** | tx consuming randomness MUST seed PRNG from `(tx.tx_id, q.state_root_t)` | LedgerEntry.system_signature uses keypair (deterministic given private key); no entropy in append/replay |

CO1.7 does NOT introduce new invariants — it provides the machine-checkable witness for 8 of the 27 frozen invariants.

---

## § 7 Conformance tests

| Test | What it asserts |
|---|---|
| `tests/transition_determinism.rs` | step_transition(q, tx) called twice → byte-identical Q_{t+1}; ledger_root_t identical (CO1.7 append() witness) |
| `tests/q_state_reconstruct.rs` | Run N transitions live → snapshot Q_t. Cold-restart, call replay(genesis, [entries]) → assert state_root + ledger_root match snapshot. (CO1.7 replay() witness) |
| `tests/l4_sequencer_serialization.rs` | Submit 100 tx concurrently from 8 threads; assert (logical_t, tx_id) is strict total order; replay produces deterministic state_root (CO1.7 sequencer witness) |
| `tests/finalize_batch_order.rs` | 3 claims expire same tick; assert ordering by (expires_at, claim_id); 2 runs byte-identical (CO1.7 sequencer + § 5.2.3 witness) |
| `tests/no_wall_clock_in_tx.rs` | LedgerEntry has no wall-clock field; sequencer.apply_one has no `SystemTime::now()` call (grep test) |
| `tests/ledger_root_chain_integrity.rs` | NEW, CO1.7-specific: tamper with one LedgerEntry's resulting_ledger_root; replay must FAIL with LedgerRootMismatch at that index |
| `tests/cas_payload_recovery.rs` | NEW, CO1.7-specific: serialize a WorkTx → CAS put → LedgerEntry references CID → CAS get → byte-identical WorkTx |
| `tests/system_signature_verifies.rs` | NEW, CO1.7-specific: every committed LedgerEntry's system_signature verifies against the committed system_keypair public key |

**Total CO1.7-specific tests**: 3 NEW + 5 referenced from spec § 4 = 8 conformance tests.

---

## § 8 Integration with step_transition family

CO1.7 publishes a single function `dispatch_transition(q, typed_tx) -> (q_next, signals)` that the sequencer's `apply_one` calls. Existing transition functions in `STATE_TRANSITION_SPEC § 3-3.7` are wired into this dispatch:

```rust
pub(crate) fn dispatch_transition(q: &QState, tx: &TypedTx) -> Result<(QState, SignalBundle), TransitionError> {
    match tx {
        TypedTx::Work(t)             => step_transition(q, t, &q.predicate_registry, &q.tool_registry),
        TypedTx::Verify(t)           => verify_transition(q, t, &q.predicate_registry),
        TypedTx::Challenge(t)        => challenge_transition(q, t, &q.predicate_registry),
        TypedTx::Reuse(t)            => reuse_transition(q, t, &q.tool_registry),
        TypedTx::FinalizeReward(t)   => finalize_reward_transition(q, t),
        TypedTx::TaskExpire(t)       => task_expire_transition(q, t),
        TypedTx::TerminalSummary(t)  => emit_terminal_summary(q, t),
    }
}
```

**Where the transition function bodies live**: this is decided per-atom downstream (CO1.7.5 implements `step_transition`; CO1.7.6 implements verify/challenge/etc. — see Plan v3.2 § 3.4 atoms). CO1.7 itself only ships the dispatch + sequencer + ledger writer; the transition function bodies are stubs (`unimplemented!()`) that downstream atoms fill.

---

## § 9 STEP_B disposition

CO1.7 lives in NEW files (`src/bottom_white/ledger/transition_ledger.rs`, `src/state/sequencer.rs`). It does NOT modify `src/bus.rs` / `src/kernel.rs` / `src/wal.rs` (the STEP_B-restricted files). Therefore: **no STEP_B parallel-branch ceremony required**. Direct edit on `main` is per CLAUDE.md "Code Standard".

The retirement of `src/ledger.rs` (legacy top-level) is **NOT in CO1.7 scope** — it is in CO1.1.5 (kernel.rs split) per `STATE_TRANSITION_SPEC § 5.3` Legacy Economic Tx Disposition table.

---

## § 10 What this spec does NOT specify

1. **Garbage collection of finalized claims** — claims are finalized in-place via finalize_reward_transition; no L4 entry deletion ever (append-only is constitutional, Art 0.2). CO1.8 materialized-state may compact L5 indices, but L4 stays whole.
2. **Cross-cell sharing** — § 5.2.2 mandates disjoint runtime_repo per cell. Multi-tenant deployments are a v4.x extension.
3. **Recovery from corrupted git history** — out of scope for v1; if `git fsck` fails, runtime aborts (fail-closed). Backup/restore strategy is operational, not specified.
4. **Performance tuning** — no SLO commitments. Round-1 audit may request rough wall-clock budget.

---

## § 11 Open questions for round-1 audit

The following are deliberately under-specified; round-1 audit input requested:

- **Q1** (Codex/Gemini both): SubmissionQueue type — `tokio::sync::mpsc::UnboundedReceiver` (current proposal), `crossbeam::channel`, or `std::sync::mpsc`? Trade-off is back-pressure semantics + dep weight.
- **Q2** (Codex preferred): how to surface sequencer back-pressure to agent submissions when queue is full? Async wait vs. immediate Err? Affects multi-agent fairness.
- **Q3** (Gemini preferred): is `Sequencer` the right abstraction boundary, or should it be split into `LedgerWriter` (storage) + `OrderingCoordinator` (sequencer logic)? Trait segregation argument.
- **Q4** (Codex): system_signature placement — inside LedgerEntry struct (current proposal, signed-entry is the canonical artifact) vs. a sidecar `(LedgerEntry, SystemSignature)` tuple. The sidecar form makes the canonical_digest computation simpler but adds a pairing concern.
- **Q5** (Gemini): is the `dispatch_transition` enum-match pattern the right shape, or should we use the `MetaTransitionInterface` trait pattern (CO P3-prep.5)? Trade-off is v4/v4.1 boundary cleanliness.
- **Q6** (Codex): `replay` rejects on first error (current). Should it instead collect all errors for diagnostic completeness? Trade-off is error-mode complexity.
- **Q7** (Gemini): genesis ledger_root_t — `Hash::ZERO` (current) or sha256 of the genesis_payload.toml content? The latter binds replay to a specific genesis; the former is simpler but loses that anchor.
- **Q8** (BOTH; surfaced post type-skeleton smoke 2026-04-28): existing `system_keypair::CanonicalMessage` has 3 fixed variants (RejectedAttemptSummary / TerminalSummaryTx / EpochRotationProof). LedgerEntry is NOT among them. Two paths: (a) extend `CanonicalMessage` enum with `LedgerEntry(LedgerEntry)` variant — touches Wave 4-B shipped code (additive, not breaking); (b) introduce a sibling sign primitive specifically for LedgerEntry that does not go through `CanonicalMessage`. Trade-off: (a) preserves single-canonical-digest principle but couples ledger to the enum; (b) decouples but introduces a second signing pathway with parallel canonical digest discipline.
- **Q9** (BOTH; surfaced post type-skeleton smoke 2026-04-28): spec v1 § 1 said `canonical_digest_unsigned` "covers fields 1-7 (excludes signature)" but did NOT explicitly state that `resulting_ledger_root` (field 6) must ALSO be excluded. Skeleton's first replay test failed immediately — including `resulting_ledger_root` creates a circular dependency (`ledger_root_t+1 = append(ledger_root_t, digest)` where `digest ⊃ ledger_root_t+1`). Skeleton fixed: digest now covers `{logical_t, parent_state_root, tx_kind, tx_payload_cid, resulting_state_root, timestamp_logical, epoch}` — 7 fields, NOT including `resulting_ledger_root` and NOT including `system_signature`. Spec v1.1 must make this exclusion explicit at § 1.
- **Q10** (BOTH; surfaced post smoke): spec missed `epoch: SystemEpoch` field on LedgerEntry. Without it, `verify_system_signature(sig, msg, epoch, pinned_pubkeys)` cannot resolve the pubkey to use. Skeleton added it (now field 7 of 8). Spec v1.1 must add this field.

---

## § 12 Audit gates (round structure mirrors INV8 / spec v1.4 / system_keypair)

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 | ⏳ pending | ⏳ pending | TBD | initial review of this draft |
| 2+ | … | … | … | iterate to PASS/PASS |

**Pre-implementation gate**: CO1.7 v1 must reach `PASS/PASS` from Codex + Gemini before any `src/bottom_white/ledger/transition_ledger.rs` or `src/state/sequencer.rs` code is written. Sedimented per CLAUDE.md "Audit Standard" (Generator ≠ Evaluator) + memory `feedback_dual_audit`.

---

## § 13 Estimated scope

- **Spec rounds**: 2-4 round dual audit (per system_keypair + spec v1.4 history)
- **Implementation**: ~600-900 LoC + 8 conformance tests, est. 3-5 days post-PASS
- **Total atom budget**: ~1.5-2 weeks (matches LATEST line 92 estimate)

---

## § 14 Honest acknowledgements

1. ~~This spec presumes CO1.4 CAS layer's API surface~~ — verified post type-skeleton smoke 2026-04-28: `CasStore::get(&Cid) → Result<Vec<u8>, CasError>` matches; `CasStore::put` has wider signature than expected (5 params: `content`, `object_type`, `creator`, `created_at_logical_t`, `schema_id`) — sequencer must build full CAS metadata. **DIV-5** flagged.
2. The SubmissionQueue type is a tokio choice; if the project pivots to a different async runtime, § 3 Sequencer.run() rewrites.
3. § 11 Q4 + Q7 + Q8 + Q9 + Q10 are real design forks; round-1 audit settles them.
4. ~~system_signature integration relies on CO1.7.0a-f's API exactly as shipped~~ — verified post smoke: `SystemSignature::from_bytes`, `SystemEpoch::new/get`, `verify_system_signature(sig, msg, epoch, pinned_pubkeys)` all public. The actual `CanonicalMessage` enum has 3 fixed variants, LedgerEntry is NOT among them. **Q8** (NEW) surfaced.
5. **Spec ↔ skeleton divergences sedimented** (post 2026-04-28 smoke):
   - **DIV-1**: `CanonicalMessage` enum integration → Q8 (NEW)
   - **DIV-2**: Q_t mutation API not yet present → state-mutation paths in skeleton are `unimplemented!()` until CO P2.x economy atoms
   - **DIV-3**: missing `epoch: SystemEpoch` field → Q10 (NEW); skeleton already added
   - **DIV-4**: `CasReader` trait → narrowed to `LedgerCasView` (CasStore impls in CO1.7.5+)
   - **DIV-5**: `CasStore::put` 5-param signature → sequencer responsibility documented in § 1
6. **Spec v1 bug found by skeleton smoke** (Q9, NEW): `canonical_digest_unsigned` must EXCLUDE `resulting_ledger_root`, not just `system_signature`. Spec v1 § 1 wording was ambiguous; first replay test caught the cycle. Skeleton fixed; spec v1.1 must explicit.

## § 15 Pre-audit smoke verification (2026-04-28)

| Smoke item | Result | What it proved |
|---|---|---|
| `cargo check` on `src/bottom_white/ledger/transition_ledger.rs` | PASS | LedgerEntry / TxKind / append / replay_chain_integrity / InMemoryLedgerWriter all type-check against existing `Cid` (CO1.4) + `SystemSignature`/`SystemEpoch` (CO1.7.0a-f) + `Hash` (Q_t) |
| `cargo test --lib bottom_white::ledger::transition_ledger::` | 6/6 PASS | append byte-stable; canonical_digest stable across clones; in-memory writer enforces logical_t monotonic; replay validates parent chain; replay rejects parent_state_root tamper; replay rejects ledger_root tamper |
| `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` | PASS post TR refresh | new file `transition_ledger.rs` + modified `mod.rs` added to `genesis_payload.toml [trust_root]` |
| `cargo test --lib` (full workspace) | 196/0 PASS | no regression in 190 pre-existing tests |

**Audit-ready artifact set**: spec v1 (this file) + skeleton (`src/bottom_white/ledger/transition_ledger.rs`, ~370 lines incl. 6 inline tests) + 5 cataloged divergences + 4 new round-1 audit Qs (Q8/Q9/Q10/Q11). Round-1 audit has both paper + code to inspect — higher signal density than spec-only review.

— ArchitectAI, session 2026-04-28; smoke-verified 2026-04-28.


---

# Type-skeleton (target of audit, joint with spec above)

```rust
//! L4 Transition Ledger (CO1.7) — type skeleton + pure helpers.
//!
//! TRACE_MATRIX FC2-Append: canonical envelope appended to L4 once a transition is accepted.
//! TRACE_MATRIX WP § 5.L4: ChainTape Layer 4 spine; one LedgerEntry per accepted transition.
//! TRACE_MATRIX § 1 (CO1_7_TRANSITION_LEDGER_v1_2026-04-28): schema + append() + replay() pseudocode.
//!
//! **Status**: type skeleton ahead of round-1 dual external audit.
//! All bodies that depend on yet-to-implement transition functions are
//! `unimplemented!()` and gated by `cfg(any())` to keep `cargo check` clean.
//!
//! **Spec ↔ code divergences flagged for round-1 audit** (NOT silently absorbed):
//! - **DIV-1** `LedgerEntry` cannot ride existing `CanonicalMessage` enum
//!   (3 variants: RejectedAttemptSummary / TerminalSummaryTx / EpochRotationProof).
//!   Either extend the enum OR introduce a sibling sign primitive. v1 spec
//!   chose neither — round-1 audit Q8 (NEW).
//! - **DIV-2** `Sequencer` integration with `Q_t.economic_state_t.balances_t`
//!   etc. requires those indices to expose mutation API; current `q_state.rs`
//!   only stores the BTreeMap shells. Mutation API arrives at CO P2.x economy
//!   atoms; skeleton uses `unimplemented!()` for state mutation paths.
//! - **DIV-3** Spec § 1 missed `epoch: SystemEpoch` field. Added here. Without
//!   it, signature verification cannot resolve which pinned pubkey to use.
//! - **DIV-4** Spec § 4 used a `CasReader` trait; actual code uses concrete
//!   `CasStore` struct. Skeleton uses a narrower trait `LedgerCasView` that
//!   `CasStore` will impl in CO1.7.5+; keeps test seams open.
//! - **DIV-5** Spec § 1 `tx_payload_cid: Cid`. CAS `put` requires
//!   `(content, object_type, creator, created_at_logical_t, schema_id)` —
//!   five fields, not just bytes. Sequencer must build full metadata when
//!   storing; skeleton documents this requirement.

use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
use crate::state::q_state::Hash;

// ────────────────────────────────────────────────────────────────────────────
// § 1 LedgerEntry schema (skeleton; round-1-audit-pending)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append: discriminator for the typed payload behind a CAS Cid.
/// **Skeleton note**: serde derives deferred — bincode v2 canonical shape is
/// round-1 audit Q5 / spec § 2.5; premature derive would lock the shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TxKind {
    /// WorkTx (STATE_TRANSITION_SPEC § 1.2; 12 fields).
    Work,
    /// VerifyTx (§ 1.3).
    Verify,
    /// ChallengeTx (§ 1.3).
    Challenge,
    /// ReuseTx (§ 1.3).
    Reuse,
    /// Claim window expired clean → reward + stake return (§ 3.4).
    FinalizeReward,
    /// Task deadline reached unsolved → bounty refund (§ 3.6).
    TaskExpire,
    /// Run end without acceptance (§ 1.5 + § 3.7).
    TerminalSummary,
    /// Post-CO P2.5 ChallengeCourt slashing event.
    Slash,
}

/// TRACE_MATRIX FC2-Append + WP § 5.L4 (12-field WorkTx envelope wrapper):
/// canonical envelope stamped by the L4 sequencer once `dispatch_transition` succeeds.
///
/// **DIV-3** vs spec v1: added `epoch: SystemEpoch` to bind signature verification
/// to a specific pinned pubkey (per `system_keypair::verify_system_signature`).
///
/// **Skeleton note**: serde derives deferred per round-1 audit Q5 (canonical shape
/// for cross-impl byte parity is a real spec choice, not a default).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerEntry {
    /// Monotonic counter from sequencer; starts at 1 per genesis.
    pub logical_t: u64,

    /// Parent state_root before this transition. Equals `prev.resulting_state_root`
    /// (or `Hash::ZERO` at logical_t=1).
    pub parent_state_root: Hash,

    /// Discriminator; payload schema depends on this.
    pub tx_kind: TxKind,

    /// CAS handle (CO1.4) to canonically-serialized payload.
    /// Sequencer is responsible for building full CAS metadata
    /// (object_type / creator / created_at_logical_t / schema_id) per **DIV-5**.
    pub tx_payload_cid: Cid,

    /// Resulting state_root after `dispatch_transition` applied.
    /// Used by I-DETHASH replay test.
    pub resulting_state_root: Hash,

    /// Resulting ledger_root after this entry is folded in.
    /// Convention: `ledger_root_{t+1} = sha256(ledger_root_t || canonical_digest_unsigned(LedgerEntry_t))`
    pub resulting_ledger_root: Hash,

    /// Wall-clock-free timestamp (= `logical_t`).
    /// Runtime layer does NOT mutate this field after sequencer commit.
    pub timestamp_logical: u64,

    /// **DIV-3**: which pinned epoch pubkey signed this entry.
    pub epoch: SystemEpoch,

    /// System runtime keypair signature over `canonical_digest_unsigned`.
    /// Distinct from the `agent_signature` inside payload (agent self-sign).
    /// System signature attests "sequencer accepted this entry at this logical_t".
    ///
    /// **DIV-1**: how this is computed is round-1 audit Q8. Skeleton stores it
    /// but does not yet derive it through `CanonicalMessage` (which currently has
    /// 3 fixed variants and does NOT include LedgerEntry).
    pub system_signature: SystemSignature,
}

impl LedgerEntry {
    /// TRACE_MATRIX FC2-Append: canonical digest of the 7 fields the system
    /// signature attests. **Excludes** `resulting_ledger_root` AND `system_signature`:
    /// - `system_signature` (8) is excluded because the digest is its input.
    /// - `resulting_ledger_root` (6) is excluded because it is *derived* via
    ///   `append(prev_ledger_root, digest)` — including it would create a
    ///   circular dependency (ledger_root ⊃ digest ⊃ ledger_root).
    ///
    /// **Spec finding**: this exclusion was NOT explicit in spec v1 § 1.
    /// Skeleton smoke caught the cycle immediately on first replay-test run.
    /// To be sedimented into spec v1.1 round-1 audit Q9 (NEW).
    ///
    /// **DIV-1**: this digest is what the system_signature must sign once the
    /// `CanonicalMessage` integration question is resolved at round-1.
    pub fn canonical_digest_unsigned(&self) -> Hash {
        let mut h = Sha256::new();
        h.update(b"turingosv4.ledger_entry.v1");
        h.update(self.logical_t.to_be_bytes());
        h.update(self.parent_state_root.0);
        h.update((self.tx_kind as u8).to_be_bytes());
        h.update(self.tx_payload_cid.0);
        h.update(self.resulting_state_root.0);
        // EXCLUDED: self.resulting_ledger_root.0 — derivative of this digest.
        h.update(self.timestamp_logical.to_be_bytes());
        h.update(self.epoch.get().to_be_bytes());
        Hash(h.finalize().into())
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 4 append() — pure ledger-root fold
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append + spec § 4: pure ledger-root fold.
/// Same `(parent_root, entry_digest)` → byte-identical `new_root`.
/// No I/O, no clock, no env. Witness for I-DET / I-DETHASH ledger axis.
pub fn append(parent_root: &Hash, entry_digest: &Hash) -> Hash {
    let mut h = Sha256::new();
    h.update(b"turingosv4.ledger_root.v1");
    h.update(parent_root.0);
    h.update(entry_digest.0);
    Hash(h.finalize().into())
}

// ────────────────────────────────────────────────────────────────────────────
// LedgerWriter trait + in-memory test impl
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append: storage abstraction for L4.
/// Production impl is `Git2LedgerWriter` (CO1.7.5; refs/transitions/main commit chain).
/// Test/skeleton impl is `InMemoryLedgerWriter` below.
pub trait LedgerWriter: Send + Sync {
    /// Commit a signed entry. Atomic: either the entry lands at the next logical_t
    /// or no state change. Returns the entry's `resulting_ledger_root` on success.
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;

    /// Read the entry at a specific 1-indexed `logical_t`.
    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;

    /// Total number of accepted entries (highest assigned `logical_t`; 0 at genesis).
    fn len(&self) -> u64;
}

/// TRACE_MATRIX FC2-Append: error surface for storage layer.
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

/// TRACE_MATRIX FC2-Append: in-memory test/skeleton writer.
/// Vec backing → strict logical_t ordering enforced at commit.
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
// § 4 replay() — chain-integrity skeleton (full transition dispatch deferred)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX I-DETHASH (chain-integrity axis only at this iteration).
/// **Skeleton**: validates parent_state_root + ledger_root chain only.
/// Full transition dispatch (re-running each tx pure-function-side) lands when
/// CO1.7.5+ implements the actual transition function bodies.
#[derive(Debug)]
pub enum ReplayError {
    LogicalTGap { at: usize, expected: u64, got: u64 },
    ParentMismatch { at: usize },
    LedgerRootMismatch { at: usize },
}

impl std::fmt::Display for ReplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LogicalTGap { at, expected, got } => {
                write!(f, "logical_t gap at index {at}: expected {expected}, got {got}")
            }
            Self::ParentMismatch { at } => write!(f, "parent_state_root mismatch at index {at}"),
            Self::LedgerRootMismatch { at } => write!(f, "ledger_root mismatch at index {at}"),
        }
    }
}
impl std::error::Error for ReplayError {}

/// Replay chain integrity. Returns final `(state_root, ledger_root)` after replaying
/// `entries` from a given (genesis_state_root, genesis_ledger_root) start.
///
/// Per **DIV-2**: this skeleton does NOT yet re-run pure transition functions to
/// independently verify `entry.resulting_state_root`. That step lands in CO1.7.5
/// once `dispatch_transition` is implementable.
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
            return Err(ReplayError::ParentMismatch { at: i });
        }
        let recomputed = append(&prev_ledger_root, &entry.canonical_digest_unsigned());
        if recomputed != entry.resulting_ledger_root {
            return Err(ReplayError::LedgerRootMismatch { at: i });
        }
        prev_state_root = entry.resulting_state_root;
        prev_ledger_root = entry.resulting_ledger_root;
    }

    Ok((prev_state_root, prev_ledger_root))
}

// ────────────────────────────────────────────────────────────────────────────
// Tests — only the genuinely-pure paths are exercised at skeleton stage
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn h(byte: u8) -> Hash {
        Hash([byte; 32])
    }

    fn entry_at(
        logical_t: u64,
        parent_state_root: Hash,
        resulting_state_root: Hash,
        prev_ledger_root: Hash,
    ) -> LedgerEntry {
        let mut entry = LedgerEntry {
            logical_t,
            parent_state_root,
            tx_kind: TxKind::Work,
            tx_payload_cid: Cid([0u8; 32]),
            resulting_state_root,
            resulting_ledger_root: Hash::ZERO, // patched below
            timestamp_logical: logical_t,
            epoch: SystemEpoch::new(1),
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        };
        entry.resulting_ledger_root = append(&prev_ledger_root, &entry.canonical_digest_unsigned());
        entry
    }

    #[test]
    fn append_is_pure_and_byte_stable() {
        let a = append(&Hash::ZERO, &h(1));
        let b = append(&Hash::ZERO, &h(1));
        assert_eq!(a, b, "I-DET witness on append()");
        let c = append(&Hash::ZERO, &h(2));
        assert_ne!(a, c, "different entry digests must produce different roots");
    }

    #[test]
    fn canonical_digest_byte_stable_across_clones() {
        let e = entry_at(1, Hash::ZERO, h(0xaa), Hash::ZERO);
        let d1 = e.canonical_digest_unsigned();
        let e2 = e.clone();
        let d2 = e2.canonical_digest_unsigned();
        assert_eq!(d1, d2);
    }

    #[test]
    fn in_memory_writer_enforces_logical_t() {
        let mut w = InMemoryLedgerWriter::new();
        let e1 = entry_at(1, Hash::ZERO, h(1), Hash::ZERO);
        assert!(w.commit(&e1).is_ok());
        assert_eq!(w.len(), 1);

        let e_skip = entry_at(3, e1.resulting_state_root, h(2), e1.resulting_ledger_root);
        let err = w.commit(&e_skip).unwrap_err();
        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
    }

    #[test]
    fn replay_validates_parent_chain() {
        let e1 = entry_at(1, Hash::ZERO, h(1), Hash::ZERO);
        let e2 = entry_at(2, e1.resulting_state_root, h(2), e1.resulting_ledger_root);
        let e3 = entry_at(3, e2.resulting_state_root, h(3), e2.resulting_ledger_root);
        let (final_state, final_ledger) =
            replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1.clone(), e2.clone(), e3.clone()])
                .expect("clean chain replays");
        assert_eq!(final_state, e3.resulting_state_root);
        assert_eq!(final_ledger, e3.resulting_ledger_root);
    }

    #[test]
    fn replay_rejects_parent_mismatch() {
        let e1 = entry_at(1, Hash::ZERO, h(1), Hash::ZERO);
        // e2 lies about parent_state_root
        let mut e2 = entry_at(2, e1.resulting_state_root, h(2), e1.resulting_ledger_root);
        e2.parent_state_root = h(0xff); // tamper
        e2.resulting_ledger_root = append(&e1.resulting_ledger_root, &e2.canonical_digest_unsigned());

        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
        assert!(matches!(err, ReplayError::ParentMismatch { at: 1 }));
    }

    #[test]
    fn replay_rejects_ledger_root_tamper() {
        let e1 = entry_at(1, Hash::ZERO, h(1), Hash::ZERO);
        let mut e2 = entry_at(2, e1.resulting_state_root, h(2), e1.resulting_ledger_root);
        // Don't recompute resulting_ledger_root — tamper directly
        e2.resulting_ledger_root = h(0xee);

        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
        assert!(matches!(err, ReplayError::LedgerRootMismatch { at: 1 }));
    }
}

```

---

# XREF: STATE_TRANSITION_SPEC v1.4 (frozen, round-4 PASS/PASS)

# State Transition Specification v1.4

> **Date**: 2026-04-27 (v1.4 closes 4 cosmetic Codex round-3 PARTIAL items)
>
> **Patch v1.3 → v1.4 changes** (per Codex round-3 re-audit at `handover/audits/CODEX_SPEC_V13_REAUDIT_2026-04-27.md`):
> - **§ 5.3 grep list cleanup** (Q1.1): patch log no longer claims `TaskMarketPublishTx` is RETIRED; it's a NEW transition deferred to CO P2.1. Conformance test grep includes only actually-retired symbols.
> - **§ 3.2 challenge_transition + § 3.4 finalize_reward** (Q2.4): both now invoke `ChallengeWindow::is_open(now)` method (defined § 5.2.5 NEW); pseudocode no longer hand-codes the inequality.
> - **§ 5.2.1 sequencer tie-break** (Q6): `next_logical_t()` is atomic; assigned `logical_t` IS the canonical tie-break for concurrent submissions; explicit prose added.
> - **§ 2.5 + § 7 fixture corpus defer-ack** (Q5/NEW-5): canonical serialization RULE frozen v1.4; full golden fixture corpus + differential fuzz seed lands in CO1.1.4-pre1 + CO1.7 atoms (not v1.x spec scope).
>
> **Patch v1.2 → v1.3 changes** (per Codex re-audit verdict CHALLENGE/NO-GO at `handover/audits/CODEX_SPEC_V12_REAUDIT_2026-04-27.md`):
> - **§ 3.6 task_expire_transition refactored** — removed runtime side effects from pure transition; runtime constructs+signs `TaskExpireTx` BEFORE pure entry; restores § 2 + § 3 pure-boundary discipline (Codex new-issue #1 fix)
> - **§ 3.6 stage 3 expiry guard broadened** — refund only if NO claim of ANY status exists for task; prevents race with Pending/Provisional claims (Codex new-issue #2 fix)
> - **§ 3.6.5 agent_implicit_init refactored** — introduce `HasSubmitter` trait with per-tx `submitter_id()` methods; resolves WorkTx vs VerifyTx vs ChallengeTx vs ReuseTx field-name divergence; `ReuseTx` returns None (intentional; reuse facts have no submitter) (Codex new-issue #3 fix)
> - **I-FINALIZE-BATCH-ORDER + § 5.2.3 + test all use `claim_id`** consistently (was 3-way contradiction with `target_work_tx`) (Codex new-issue #4 fix)
> - **I-CHALLENGE-WINDOW-EDGE binding** — `is_open(now)` defined as `now < opens_at + duration_ticks`; both challenge_transition AND finalize_reward MUST use same `is_open()` rule (Codex Q2.4 fix)
> - **§ 5.1 false-challenge prose cleanup** — removed "User can override any default" generality where 11.1 is in fact NOT overridable in v4
> - **§ 6 (NEW) Legacy economic tx disposition** — InvestTx / TaskMarketPublishTx / MarketCreateTx / MarketResolveTx explicitly retired in CO1.1.4 atom (Codex Q1.1 NOT-CLOSED fix)
>
> **Patch v1.1 → v1.2 changes** (per Codex+Gemini CO1.SPEC.0.5 dual audit, 2026-04-27):
> - **§ 2 hidden-input table EXPANDED** — added HAYEK_BOUNTY, BOUNTY_LP, Boltzmann params, BOLTZMANN_SEED, async ordering boundary, WAL/git commit boundary, full HashMap scope, f64 royalty math
> - **§ 2.5 (NEW) canonical serialization** — defines byte-level format for all signed tx + state roots
> - **§ 3.4 finalize_reward** — added stage 3a (solver stake unlock + return); royalty math now uses integer floor rule
> - **§ 3.6 (NEW) task_expire_transition** — handles unsolved task bounty refund
> - **§ 3.7 (NEW) agent_register implicit-init** — first appearance in L4 = default reputation 0
> - **§ 5.1 false-challenge resolution** — fixed to "v4 default 0, NOT configurable" (resolves prose-vs-pseudocode contradiction)
> - **§ 4 invariants** — 22 → 27 (added I-STAKE-RETURN, I-BOUNTY-REFUND, I-FINALIZE-BATCH-ORDER, I-CHALLENGE-WINDOW-EDGE, I-AGENT-INIT)
> - **§ 6.1 (NEW) concurrency rule** — L4 sequencer per (runtime_repo, run_id); deterministic ordering key
> - **§ 8 count fix** — "16 invariants" → "27 invariants"
>
> **Patch v1 → v1.1 changes** (per SPEC_WALKTHROUGH gap fixes, 2026-04-27):
> - § 3.2 (challenge_transition) stage 4e ADDED: verifier_bond release policy (default = return to verifier; configurable)
> - § 3.3 (reuse_transition) stage 3 AMENDED: edge weight bounded by `MAX_REUSE_ROYALTY_FRACTION` config (default = 0.10)
> - § 3.2 (challenge_transition) stage 4d AMENDED: false-challenge reputation penalty (v1.3 update: **fixed to 0 in v4; NOT configurable**; previous v1.1 patch log saying "configurable" is OBSOLETE)
> - § 3.1 (verify_transition) note ADDED: quorum-aggregation rule placeholder (default = 1; configurable)
> - § 4 invariants ADDED: I-VBOND-RELEASE / I-ROYALTY-CAP
> - § 11 (Found Inconsistencies) — promoted from SPEC_WALKTHROUGH § 11
>
> All 4 walk-through gaps now have either (a) machine-checkable default applied, or (b) explicit deferral with target atom.
>
> **Purpose**: D-VETO-1 binding form. Defines `step_transition: (Q_t, tx_i) → (Q_{t+1}, signals_t)` with typed schemas, deterministic pseudocode, named invariants, conformance test list. Gates CO1.1.4/CO1.1.5 bus.rs/kernel.rs split (per Plan v3.2 atom CO1.SPEC.0).
>
> **Authority**: Constitution Art. 0–0.4 + white paper architecture § 3-7 + economic § 2/§ 6/§ 18-21. Where this spec disagrees with white paper, **white paper wins** and this spec must be amended.
>
> **Audit**: Codex CO P0.7 T+S review (2026-04-27) demanded binding spec form before refactor. This document is the response.

---

## § 0 Scope

**In scope**:
- The single-step state transition function `step_transition` for object-level work_tx
- Typed `QState`, `WorkTx`, `VerifyTx`, `ChallengeTx`, `RejectedAttemptSummary`, `TerminalSummaryTx` schemas
- Hidden-input classification: which existing `bus.rs`/`kernel.rs` inputs are `Q_t`, which are `tx_i`, which are illegal side effects
- Named invariants enforceable mechanically
- Conformance test list generated from the spec

**Out of scope** (handled separately):
- `MetaTx` schema for runtime meta-transitions — defined as **stub only** here; full schema deferred to v4.1 per D-VETO-4 = B (defer, not abandon)
- AttributionEngine DAG construction algorithm — deferred to CO2.4.0 spike (Inv 8 design)
- Full predicate visibility air-gap proof — deferred to CO P1.5 (Goodhart shield design)

---

## § 1 Typed Schemas

### 1.1 QState (white paper § 4 + economic § 2 amendment, 9 fields)

```rust
pub struct QState {
    /// Agent swarm sub-state: tape head per agent, per-agent reputation snapshots, etc.
    /// MUST be reconstructible from L4 transition ledger replay.
    pub q_t: AgentSwarmState,

    /// Current ChainTape head pointer = git commit SHA in Path B substrate.
    pub head_t: NodeId,

    /// Materialized state Merkle root (git tree root in Path B).
    pub state_root_t: Hash,

    /// Agent-visible projection of tape filtered by per-agent visibility policy
    /// (Inv 10 Goodhart shield). Derived from L1 PredicateRegistry visibility tags.
    pub tape_view_t: AgentVisibleProjection,

    /// L4 Transition Ledger root (Merkle root of all accepted tx so far).
    pub ledger_root_t: Hash,

    /// L1 Predicate Registry root.
    pub predicate_registry_root_t: Hash,

    /// L2 Tool Registry root.
    pub tool_registry_root_t: Hash,

    /// Economic state (economic § 2 amendment, 9 sub-fields).
    pub economic_state_t: EconomicState,

    /// Global budget snapshot: cost ceiling, wall clock, compute cap.
    pub budget_state_t: BudgetSnapshot,
}

pub struct AgentSwarmState {
    pub agents: BTreeMap<AgentId, PerAgentState>,
    pub current_round: u64,
}

pub struct PerAgentState {
    pub reputation_snapshot: Reputation,
    pub last_accepted_tx: Option<TxId>,
    pub retry_counter_for_current_task: u32,  // resets on accept; persists across rejections
}

pub struct EconomicState {
    pub balances_t:       BalancesIndex,
    pub escrows_t:        EscrowsIndex,
    pub stakes_t:         StakesIndex,
    pub claims_t:         ClaimsIndex,
    pub reputations_t:    ReputationsIndex,
    pub task_markets_t:   TaskMarketsIndex,
    pub royalty_graph_t:  RoyaltyGraph,
    pub challenge_cases_t: ChallengeCasesIndex,
    pub price_index_t:    PriceIndex,
}
```

**BTreeMap, not HashMap, everywhere**: deterministic iteration order for replay byte-identity (Codex flagged kernel.rs:187-204 HashMap nondeterminism).

### 1.2 WorkTx (12 fields per WP § 5.L4)

```rust
pub struct WorkTx {
    pub tx_id: TxId,                              //  1
    pub task_id: TaskId,                          //  2  links to TaskMarket entry
    pub parent_state_root: Hash,                  //  3  must equal Q_t.state_root_t at submission
    pub agent_id: AgentId,                        //  4
    pub read_set: BTreeSet<ReadKey>,              //  5  agent MUST declare read deps (DAG attribution)
    pub write_set: BTreeSet<WriteKey>,            //  6  agent MUST declare write targets
    pub proposal_cid: Cid,                        //  7  L3 CAS handle to payload (not raw payload)
    pub predicate_results: PredicateResultsBundle,//  8  filled BY runner, not by agent
    pub stake: StakeMicroCoin,                    //  9  YES_E stake, i64 micro-coin units
    pub signature: AgentSignature,                // 10
    pub timestamp_logical: u64,                   // 11  monotonic counter from runtime, NOT wall clock
    pub status: TxStatus,                         // 12  Pending | Accepted | Rejected(class) | Finalized
}

pub enum TxStatus {
    Pending,
    Accepted,
    Rejected(RejectionClass),
    FinalizedReward(MicroCoin),
    FinalizedSlash(SlashEvidenceCid),
}

pub struct PredicateResultsBundle {
    pub acceptance: BTreeMap<PredicateId, BoolWithProof>,
    pub settlement: BTreeMap<PredicateId, BoolWithProof>,
    pub safety_class: SafetyOrCreation,  // determines fail-closed vs fail-open-with-signal
}
```

### 1.3 VerifyTx, ChallengeTx, ReuseTx (economic § 13)

```rust
pub struct VerifyTx {
    pub tx_id: TxId,
    pub target_work_tx: TxId,         // the work_tx being verified
    pub verifier_agent: AgentId,
    pub bond: StakeMicroCoin,         // verifier reputation/bond stake
    pub verdict: VerifyVerdict,       // Confirm | Doubt
    pub signature: AgentSignature,
    pub timestamp_logical: u64,
}

pub struct ChallengeTx {
    pub tx_id: TxId,
    pub target_work_tx: TxId,
    pub challenger_agent: AgentId,
    pub stake: StakeMicroCoin,        // NO_E stake, i64 micro-coin
    pub counterexample_cid: Cid,      // L3 CAS handle to counterexample
    pub signature: AgentSignature,
    pub timestamp_logical: u64,
}

pub struct ReuseTx {
    pub tx_id: TxId,
    pub reusing_work_tx: TxId,        // the work_tx that triggered the reuse
    pub reused_tool_id: ToolId,       // L2 Tool Registry handle
    pub reused_tool_creator: AgentId, // royalty recipient
    pub timestamp_logical: u64,
}
```

### 1.4 RejectedAttemptSummary (D-VETO-6 system-stamped, NOT agent self-report)

```rust
pub struct RejectedAttemptSummary {
    pub failed_attempts_since_last_accept: u32,           // bounded, capped at u32::MAX
    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,  // counts only, no payloads
    pub first_failure_logical_t: Option<u64>,             // for time-to-first-fail signal
    pub last_failure_logical_t: Option<u64>,              // for recency signal
    // NO raw error strings, NO agent payload contents, NO predicate internal traces
}

pub enum RejectionClass {
    AcceptancePredicateFail(PredicateId),     // public predicates only; private predicates → Opaque
    SettlementPredicateFail(PredicateId),
    StakeInsufficient,
    SignatureInvalid,
    StaleParentRoot,                          // Q_t advanced; agent's view stale
    Opaque,                                   // private predicate failure; classification withheld
    BudgetExceeded,
}
```

`RejectedAttemptSummary` is stamped **by the white-box predicate runner** onto the next accepted `WorkTx`. Trust boundary: the runner generates this summary; the agent does NOT self-report. Verified at conformance test level.

### 1.5 TerminalSummaryTx (no-accept run handler)

```rust
pub struct TerminalSummaryTx {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub run_id: RunId,
    pub run_outcome: RunOutcome,           // OmegaAccepted | MaxTxExhausted | WallClockCap | ComputeCap | ErrorHalt
    pub total_attempts: u32,
    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,
    pub last_logical_t: u64,
    pub system_signature: SystemSignature,  // signed by runtime keypair, not by any agent
}
```

If a run terminates without any accepted work_tx, the runtime emits exactly one `TerminalSummaryTx` to L4. This preserves L6 reconstructibility: error class signal is derivable from tape even if no work_tx ever passed.

### 1.6 MetaTx (stub for v4.1; v4 only emits `MetaProposalDraft` to L3 CAS, not L4)

```rust
pub struct MetaTx {
    pub tx_id: TxId,
    pub parent_architecture_root: Hash,
    pub proposed_predicate_patches: Vec<PredicatePatch>,
    pub proposed_tool_patches:      Vec<ToolPatch>,
    pub log_evidence_cids:           Vec<Cid>,
    pub reversibility_plan_cid:      Cid,
    pub constitution_check:          ConstitutionCheckProof,
    pub judge_signatures:            Vec<JudgeSignature>,
    pub human_signature_required:    bool,
    pub human_signature:             Option<HumanSignature>,
}
```

**v4 status**: MetaTx schema reserved; runtime ArchitectAI/JudgeAI **NOT implemented**. v4 produces `MetaProposalDraft` (a CAS object) only, written when ArchitectAI proposes architecture amendments via the cp workflow. v4.1 implements the runtime actor + L4 acceptance.

This is the D-VETO-4 = B (defer, not abandon) implementation.

---

## § 2 Hidden-Input Classification (Codex § C demanded)

The current `src/bus.rs` and `src/kernel.rs` mix four categories of inputs. The spec must classify each:

| Input | Current source | T+S classification | New home in step_transition |
|---|---|---|---|
| `created_at` (wall clock seconds) | `bus.rs:264-268` `SystemTime::now()` | **ILLEGAL hidden side effect** | retire; replace with `timestamp_logical: u64` from runtime monotonic counter |
| `completion_tokens: 0` literal | `bus.rs:268` | **ILLEGAL hidden zero** | kill in CO1.1.4-pre1; read real value from LLM `usage.completion_tokens` |
| `TAPE_ECONOMY_V2` env var | `bus.rs:298, 345` | **`Q_t.budget_state_t.feature_flags`** | promote to typed field; tx must reference flag value at parent_state_root |
| `FOUNDER_GRANT_GAMMA` env var | `bus.rs:307` | **`Q_t.economic_state_t.task_markets_t.config.founder_grant_gamma`** | promote to typed field; bound at task creation, not env at runtime |
| `self.config.system_lp_amount` | `bus.rs:340` | **`Q_t.economic_state_t.task_markets_t.config.system_lp_amount`** | promote |
| `self.clock` counter | `bus.rs:42` | **`Q_t.q_t.current_round` derived** | derive from L4 ledger length; not separately tracked |
| `self.tx_count` | `bus.rs:42` | **`Q_t.q_t.current_round` derived** | derive |
| `self.generation` | `bus.rs:42` | **`Q_t.q_t.generation` typed field** | promote |
| `self.graveyard: HashMap<String, Vec<String>>` | `bus.rs:48` | **ILLEGAL sidecar** (Art. 0.2 explicitly anti-patterned) | retire; replace with `RejectedAttemptSummary` stamped on next accepted tx + `TerminalSummaryTx` |
| Tool list iteration order | `bus.rs:312-319` Vec | **`Q_t.tool_registry_root_t` derived** | runner queries L2 in deterministic order |
| Wallet "magic search" | `bus.rs:312-319` `manifest() == "wallet"` | **EXPLICIT capability lookup** | runner queries L2 by `Capability::EconomicWallet` tag, not by string match |
| `HAYEK_BOUNTY` env var (v1.2 added per Codex Q3) | `src/bus.rs:141-150` (init), `src/bus.rs:349-360` (settle) | **`Q_t.economic_state_t.task_markets_t.config.hayek_bounty_enabled`** | promote to typed task config; bound at task creation |
| `BOUNTY_LP` env var (v1.2 added per Codex Q3) | `src/bus.rs:141-150`, `src/bus.rs:349-360` | **`Q_t.economic_state_t.task_markets_t.config.bounty_lp_seed: MicroCoin`** | promote to typed task config |
| `BOLTZMANN_TEMP` / `FRONTIER_CAP` / `DEPTH_WEIGHT` / `PRICE_GATE_ALPHA` / `BOLTZMANN_SEED` env (v1.2 added per Codex Q3) | `src/sdk/actor.rs:22-39` (params), `experiments/.../bin/evaluator.rs:693-697` (seed) | **OFF-TAPE proposal-generation only**; NOT part of `Q_t`; routing seed visible in `proposal_cid` payload (CAS); transition pseudocode does NOT consume these | classified as "agent-side proposal entropy"; the SAMPLED outcome is on tape via proposal_cid; the sampling RNG state is NOT |
| HashMap iteration order broadly (v1.2 added per Codex Q3) | `src/kernel.rs:19-21` (markets), `src/kernel.rs:165-204` (resolve + ticker), any new code | **BANNED in any module reachable from `step_transition` call tree** | runtime test grep extends to ALL `src/` files reachable transitively; not just modules containing "q_state" or "transition" |
| Async tokio task completion ordering (v1.2 added per Codex Q3 + Q6) | `experiments/.../bin/evaluator.rs:192-193` (#[tokio::main]) | **L4 sequencer (§ 6.1) defines deterministic ordering key (logical_t, tx_id)**; async completion order is NOT used | sequencer enforces serialization point per (runtime_repo, run_id); see § 6.1 |
| WAL / git commit filesystem effects (v1.2 added per Codex Q3) | `src/bus.rs:279-282` (WAL Node), `src/bus.rs:319-327` (WAL event) | **explicit boundary: pure `step_transition(q, tx)` returns `(q', signals)` PURELY; runtime layer commits side effects to WAL/git AFTER pure result** | step_transition is pure function of (q, tx); commit is runtime concern; § 6.1 specifies commit point |
| `f64` arithmetic in monetary / royalty math (v1.2 added per Codex Q3 + Q10) | `src/prediction_market.rs:21-27,87-133` (reserves, trades) + spec § 3.3 royalty `reward * edge.weight` | **i64 MicroCoin only; royalty rounding rule = integer floor (`micro_reward * weight_micro / 1_000_000`)** | promote `prediction_market.rs` to MicroCoin; spec § 3.3 stage 3b adds explicit rounding |
| Future tokio::spawn introduction (v1.2 hypothesis per Codex Q3) | (none currently) | **BANNED in `src/transition/*` and `src/economy/*` call trees** | cargo-deny rule + transitive grep |

After this classification, every step_transition input is either part of `Q_t`, part of `tx_i`, or part of the runtime config bound at genesis (which is itself in `Q_t`).

**Conformance test for § 2** (`tests/no_hidden_inputs.rs`):
- grep src/ for `SystemTime::now()` → must return 0 hits in non-runtime-bootstrap code
- grep src/ for `std::env::var(` → must return 0 hits in step_transition path **AND** in any module transitively reachable from `transition::*`, `economy::*`, `top_white::predicates::*` (v1.2 expanded scope per Codex Q3)
- grep src/ for `HashMap` → must return 0 hits in **ALL modules reachable from `step_transition` call tree** (v1.2 expanded scope; was: only "q_state" or "transition" modules; new scope: full transitive reach)
- assert all monetary fields are typed `MicroCoin` (a newtype around `i64`), no `f64` — **including `src/prediction_market.rs` and any RSP module**
- grep src/ for `tokio::spawn` → must return 0 hits in `src/{transition,economy,top_white::predicates}/*` (v1.2 added per Codex Q3 hypothesis)

## § 2.5 Canonical Serialization (v1.2 NEW per Codex Q5)

> **Required because**: `tx.canonical_digest()` is called in spec § 3 stages 2 of WorkTx / VerifyTx / ChallengeTx, but byte-level format is undefined. STEP_B branch A vs branch B may pick different serialization (JSON sorted keys vs bincode vs Rust derive order) → cross-branch signature verification fails. Mandatory canonical format closes this.

**Format**: **bincode v2** (`bincode::serde`) with the following constraints:
- **Big-endian byte order** for all multi-byte integers (network order; deterministic across platforms)
- **`BTreeMap` keys serialized in lexicographic byte order** (this is bincode default; verified by test)
- **Strings serialized as UTF-8 with explicit length prefix u32-BE**
- **Optional fields: `0x00` prefix for `None`, `0x01` + value for `Some`**
- **Enum discriminant: u8 (variant index in declaration order)**
- **No padding bytes; no implicit alignment**

**Application**:
```rust
pub fn canonical_digest<T: Serialize>(value: &T) -> [u8; 32] {
    let bytes = bincode::serde::encode_to_vec(value, bincode_canonical_config()).expect("serialize");
    sha256(&bytes)
}

fn bincode_canonical_config() -> bincode::config::Configuration {
    bincode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding()    // no varint; fixed-width for determinism
}
```

**Conformance**: `tests/canonical_serialization.rs` MUST verify:
- 1 golden tx fixture per tx type (WorkTx / VerifyTx / ChallengeTx / ReuseTx / TerminalSummaryTx); each has known input → known SHA-256 output
- Round-trip: `decode(encode(x)) == x` byte-identical for 100 random inputs
- Stability: 2 independent runs on same input → same bytes

**STEP_B implication**: branches A and B both use this exact `bincode_canonical_config`; signature verification works cross-branch by construction.

**Out of scope for v1.x spec** (deferred per Codex Q5/NEW-5 round-3 PARTIAL acknowledgment): full golden fixture corpus + differential fuzzing seed + complete runner ABI for QState/SignalBundle/TransitionError. v1.4 freezes the SERIALIZATION RULE (bincode v2 big-endian + BTreeMap lex); fixtures + ABI land in **CO1.1.4-pre1** (canonical fixture corpus) + **CO1.7** (full ABI surface). This is an **explicit deferral** — not unresolved spec ambiguity. STEP_B branch A and branch B both implement the SAME bincode rule; per-tx digest matching is mechanical from v1.4. Full corpus generation is a downstream code task, not spec scope.

---

## § 3 step_transition (Deterministic Pseudocode)

```rust
/// Pure function. Same (Q_t, tx_i) → byte-identical (Q_{t+1}, signals_t).
/// No I/O. No env reads. No clock reads. No randomness without seed in tx_i.
pub fn step_transition(
    q: &QState,
    tx: &WorkTx,
    registry: &PredicateRegistry,
    tool_registry: &ToolRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {

    // STAGE 1: parent_state_root match (stale view rejection)
    if tx.parent_state_root != q.state_root_t {
        return Err(TransitionError::StaleParent {
            expected: q.state_root_t,
            got:      tx.parent_state_root,
        });
        // NB: rejection here does NOT change Q_t; runner stamps RejectedAttemptSummary
        // onto the NEXT accepted tx (or onto TerminalSummaryTx if run ends without accept)
    }

    // STAGE 2: signature verification
    if !verify_signature(&tx.signature, tx.canonical_digest()) {
        return Err(TransitionError::SignatureInvalid);
    }

    // STAGE 3: stake availability (Inv 5 — YES_E event-bound)
    let agent_balance = q.economic_state_t.balances_t.get(&tx.agent_id);
    if agent_balance < tx.stake {
        return Err(TransitionError::StakeInsufficient { available: agent_balance, required: tx.stake });
    }

    // STAGE 4: predicate gate (Inv 6 — predicate-gated transition)
    let acceptance_results = registry.run_acceptance(tx, q)?;
    let safety_class = registry.classify(tx);
    match (safety_class, acceptance_results.all_passed()) {
        (SafetyOrCreation::Safety, false) => {
            return Err(TransitionError::AcceptancePredicateFailed(acceptance_results));
            // fail-closed for Safety (WP § 7.2)
        }
        (SafetyOrCreation::Creation, false) => {
            // fail-open-with-signal: still reject, but emit informational signal (no Q_t change)
            return Err(TransitionError::AcceptancePredicateFailed(acceptance_results));
        }
        _ => {}  // passed; continue
    }

    // STAGE 5: provisional reward issue (Inv 7 — provisional then final)
    let claim = ClaimId::derive(tx.tx_id);
    let provisional_reward = SettlementEngine::issue_provisional(
        claim,
        &q.economic_state_t.escrows_t,
        tx.task_id,
    )?;

    // STAGE 6: state transition apply (deterministic)
    let mut q_next = q.clone();
    q_next.economic_state_t.claims_t.insert(claim, provisional_reward);
    q_next.economic_state_t.stakes_t.lock(tx.agent_id, tx.task_id, tx.stake);
    q_next.economic_state_t.balances_t.debit(tx.agent_id, tx.stake);
    q_next.q_t.update_per_agent(tx.agent_id, |s| {
        s.last_accepted_tx = Some(tx.tx_id);
        s.retry_counter_for_current_task = 0;  // reset on accept
    });

    // L4 append
    let new_ledger_root = ledger::append(&q.ledger_root_t, tx);
    q_next.ledger_root_t = new_ledger_root;

    // L5 materialize
    let new_state_root = materializer::apply(&q.state_root_t, tx);
    q_next.state_root_t = new_state_root;

    // L6 signal emit (broadcast price + reputation; NOT evaluator internals — Inv 10)
    let signals = SignalBundle {
        boolean: vec![Signal::Boolean(BoolSignal::AcceptedAt(tx.tx_id))],
        statistical: vec![
            Signal::Statistical(StatSignal::PriceUpdate(price_for(tx.task_id, q_next.economic_state_t.price_index_t))),
            Signal::Statistical(StatSignal::ReputationDelta(tx.agent_id, +reputation_delta(tx))),
        ],
    };

    // STAGE 7: head advance
    q_next.head_t = NodeId::from_state_root(new_state_root);

    // STAGE 8: challenge window open (Inv 7 — finalization is deferred)
    q_next.economic_state_t.challenge_cases_t.open(claim, tx.timestamp_logical, CHALLENGE_WINDOW_TICKS);

    Ok((q_next, signals))
}
```

**No wall-clock, no env-var, no HashMap iteration**. Every input is either `q`, `tx`, or registries (themselves in `q.predicate_registry_root_t` / `q.tool_registry_root_t`).

### 3.1 verify_transition (VerifyTx)

Per Gemini v3.2 review Q10 VETO — extending pseudocode to all state-mutating tx types.

> **v1.1 note (gap 11.4)**: this pseudocode handles ONE verifier per tx. Multi-verifier quorum aggregation is a TaskMarket config (`verifier_quorum_required: usize` default = 1). When N>1 verifiers each submit verify_tx for the same target_work_tx, claim transitions to `Pending → ApprovedByVerifiers` only after `verifier_quorum_required` distinct verifiers have submitted `Confirm`. Aggregation rule deferred to CO P2.7 atom (Verifier role detail). For v4 default (quorum=1), each verify_tx independently advances claim to ApprovedByVerifiers.

```rust
pub fn verify_transition(
    q: &QState,
    tx: &VerifyTx,
    registry: &PredicateRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {

    // STAGE 1: target work_tx must exist + be in Pending or Provisional state
    let target = q.economic_state_t.claims_t.get(&tx.target_work_tx)
        .ok_or(TransitionError::TargetWorkTxNotFound)?;
    if !target.status.allows_verification() {
        return Err(TransitionError::TargetWorkTxNotVerifiable);
    }

    // STAGE 2: signature + bond
    if !verify_signature(&tx.signature, tx.canonical_digest()) {
        return Err(TransitionError::SignatureInvalid);
    }
    let verifier_balance = q.economic_state_t.balances_t.get(&tx.verifier_agent);
    if verifier_balance < tx.bond {
        return Err(TransitionError::StakeInsufficient);
    }

    // STAGE 3: predicate gate (verifier predicate, NOT same as work_tx acceptance)
    let verify_results = registry.run_verification(tx, target, q)?;
    if !verify_results.all_passed() {
        return Err(TransitionError::VerificationPredicateFailed(verify_results));
    }

    // STAGE 4: state transition
    let mut q_next = q.clone();
    q_next.economic_state_t.balances_t.debit(tx.verifier_agent, tx.bond);
    q_next.economic_state_t.stakes_t.lock_verifier_bond(tx.verifier_agent, tx.target_work_tx, tx.bond);
    q_next.economic_state_t.claims_t.add_verification(tx.target_work_tx, tx.verifier_agent, tx.verdict);

    // STAGE 5: append + materialize + signals
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle {
        boolean: vec![Signal::Boolean(BoolSignal::VerifiedAt(tx.tx_id))],
        statistical: vec![Signal::Statistical(StatSignal::ReputationDelta(tx.verifier_agent, +verify_reputation_delta(tx, target)))],
    };

    Ok((q_next, signals))
}
```

### 3.2 challenge_transition (ChallengeTx)

```rust
pub fn challenge_transition(
    q: &QState,
    tx: &ChallengeTx,
    registry: &PredicateRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {

    // STAGE 1: target work_tx must exist + still in challenge window
    let target = q.economic_state_t.claims_t.get(&tx.target_work_tx)
        .ok_or(TransitionError::TargetWorkTxNotFound)?;
    let window = q.economic_state_t.challenge_cases_t.get(tx.target_work_tx)
        .ok_or(TransitionError::ChallengeWindowClosed)?;
    // v1.4: use ChallengeWindow::is_open(now) per § 5.2.5; same rule used by finalize_reward
    if !window.is_open(tx.timestamp_logical) {
        return Err(TransitionError::ChallengeWindowClosed);
    }

    // STAGE 2: signature + NO_E stake
    if !verify_signature(&tx.signature, tx.canonical_digest()) {
        return Err(TransitionError::SignatureInvalid);
    }
    let challenger_balance = q.economic_state_t.balances_t.get(&tx.challenger_agent);
    if challenger_balance < tx.stake {
        return Err(TransitionError::StakeInsufficient);
    }

    // STAGE 3: counterexample acceptance predicate (the BURDEN OF PROOF predicate, Inv 7)
    let counterexample = cas::get(&tx.counterexample_cid)?;
    let counter_check = registry.run_counterexample_check(target, &counterexample, q)?;
    if !counter_check.proves_violation() {
        return Err(TransitionError::CounterexampleInsufficient(counter_check));
    }

    // STAGE 4: state transition — ROLLBACK target work_tx + slash original solver + reward challenger
    let mut q_next = q.clone();
    q_next.economic_state_t.balances_t.debit(tx.challenger_agent, tx.stake);

    // 4a: rollback target's provisional reward
    let rollback_amount = q.economic_state_t.claims_t.provisional_amount(tx.target_work_tx);
    q_next.economic_state_t.claims_t.mark_slashed(tx.target_work_tx, tx.tx_id);

    // 4b: slash original solver's stake → reward pool for challenger
    let solver_stake = q.economic_state_t.stakes_t.get(target.solver, target.task_id);
    q_next.economic_state_t.stakes_t.slash(target.solver, target.task_id);
    q_next.economic_state_t.escrows_t.deposit_from_slash(tx.challenger_agent, solver_stake);

    // 4c: challenger gets back NO_E stake + slashed solver stake
    q_next.economic_state_t.balances_t.credit(tx.challenger_agent, tx.stake + solver_stake);

    // 4d: solver reputation -= delta; challenger reputation += delta (Inv 9 immutable but we update via formula not transfer)
    q_next.economic_state_t.reputations_t.adjust(target.solver, -slash_reputation_delta());
    q_next.economic_state_t.reputations_t.adjust(tx.challenger_agent, +challenge_reputation_delta());

    // 4e: verifier_bond release per task config (gap 11.2 fix; default = return to good-faith verifier)
    //   Rationale: when Carol slashes Alice via challenge, Bob (the verifier) was duped but acted in good faith.
    //   Slashing Bob's bond would discourage future verification. Configurable per TaskMarket.
    //   Applies to ALL verifiers who voted Confirm on the slashed work_tx.
    let bond_release_policy = q.economic_state_t.task_markets_t
        .get(target.task_id)
        .map(|tm| tm.config.verifier_bond_on_slash)
        .unwrap_or(VerifierBondPolicy::ReturnToVerifier);
    for (verifier, bond) in q.economic_state_t.stakes_t.verifier_bonds_for(tx.target_work_tx) {
        match bond_release_policy {
            VerifierBondPolicy::ReturnToVerifier => {
                q_next.economic_state_t.balances_t.credit(verifier, bond);
                q_next.economic_state_t.stakes_t.release_verifier_bond(verifier, tx.target_work_tx);
            }
            VerifierBondPolicy::SlashedToChallenger => {
                q_next.economic_state_t.balances_t.credit(tx.challenger_agent, bond);
                q_next.economic_state_t.stakes_t.slash_verifier_bond(verifier, tx.target_work_tx);
                q_next.economic_state_t.reputations_t.adjust(verifier, -verifier_slash_delta());
            }
        }
    }

    // STAGE 5: close challenge window
    q_next.economic_state_t.challenge_cases_t.close(tx.target_work_tx, ChallengeOutcome::Slashed(tx.tx_id));

    // STAGE 6: append + materialize + signals
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle {
        boolean: vec![Signal::Boolean(BoolSignal::ChallengeUpheld(tx.tx_id))],
        statistical: vec![
            Signal::Statistical(StatSignal::ReputationDelta(target.solver, -slash_reputation_delta())),
            Signal::Statistical(StatSignal::ReputationDelta(tx.challenger_agent, +challenge_reputation_delta())),
        ],
    };

    Ok((q_next, signals))
}
```

### 3.3 reuse_transition (ReuseTx)

```rust
pub fn reuse_transition(
    q: &QState,
    tx: &ReuseTx,
    tool_registry: &ToolRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {
    // STAGE 1: tool must be registered + still active in L2
    let tool = tool_registry.get(tx.reused_tool_id)
        .ok_or(TransitionError::ToolNotInRegistry)?;
    if tool.creator != tx.reused_tool_creator {
        return Err(TransitionError::ToolCreatorMismatch);
    }

    // STAGE 2: parent reusing_work_tx must exist + Accepted
    let parent = q.economic_state_t.claims_t.get(&tx.reusing_work_tx)
        .ok_or(TransitionError::TargetWorkTxNotFound)?;
    if !parent.status.is_accepted_or_finalized() {
        return Err(TransitionError::ParentNotAcceptedYet);
    }

    // STAGE 3: state transition — add edge to royalty graph
    //   gap 11.3 fix: weight bounded by MAX_REUSE_ROYALTY_FRACTION = 0.10 default
    //   Rationale: 10% upper bound protects solver's primary reward. Builders earn via creating
    //   widely-reusable tools, not via single high-percentage extractions. Configurable per TaskMarket
    //   for cases where user wants to override (e.g., creator-economy experiments).
    let max_royalty = q.economic_state_t.task_markets_t
        .get(parent.task_id)
        .and_then(|tm| tm.config.max_reuse_royalty_fraction)
        .unwrap_or(MAX_REUSE_ROYALTY_FRACTION_DEFAULT);  // = 0.10 in micro-coin fractional repr (10000 / 100000)
    let bounded_weight = tool.reuse_royalty_share.min(max_royalty);
    if tool.reuse_royalty_share > max_royalty {
        log::warn!(
            "reuse_tx {}: tool {} declared royalty {} > max {}; clamping to {}",
            tx.tx_id, tx.reused_tool_id, tool.reuse_royalty_share, max_royalty, bounded_weight
        );
    }

    let mut q_next = q.clone();
    q_next.economic_state_t.royalty_graph_t.add_edge(
        from: tx.reusing_work_tx,
        to:   tx.reused_tool_id,
        creator: tx.reused_tool_creator,
        weight: bounded_weight,    // clamped per gap 11.3
    );

    // STAGE 4: append + materialize (no signals; royalty paid at finalize_reward time)
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    Ok((q_next, SignalBundle::empty()))
}
```

### 3.4 finalize_reward (challenge window expiry)

Triggered by tick (no agent submits this; runtime emits when challenge window expires for any provisional claim).

```rust
pub fn finalize_reward_transition(
    q: &QState,
    claim_id: ClaimId,
    settlement_engine: &SettlementEngine,
) -> Result<(QState, SignalBundle), TransitionError> {
    let claim = q.economic_state_t.claims_t.get(&claim_id)
        .ok_or(TransitionError::ClaimNotFound)?;
    let window = q.economic_state_t.challenge_cases_t.get(claim.target_work_tx);

    // STAGE 1: window must be expired AND no open slash
    // v1.4: invoke ChallengeWindow::is_open(now) per § 5.2.5 with explicit `now` arg;
    // same rule as challenge_transition stage 1
    if let Some(w) = window {
        if w.is_open(q.q_t.current_round) {
            return Err(TransitionError::ChallengeWindowStillOpen);
        }
        if w.outcome == Some(ChallengeOutcome::Slashed(_)) {
            return Err(TransitionError::AlreadySlashed);  // never finalize a slashed claim
        }
    }

    // STAGE 2: compute reward per Economic § 21 final formula
    let reward = settlement_engine.finalize(
        claim,
        Escrow::lookup(q, claim.task_id),
        Attribution::lookup(q, claim.target_work_tx),
        Survival::full,  // window expired without slash
        Utility::lookup(q, claim.target_work_tx),
        Constitution::check(q),
    )?;

    // STAGE 3: state transition
    let mut q_next = q.clone();
    let target = claim.target_work_tx_data;

    // 3a (v1.2 NEW; gap 11.A per Gemini + Codex Q2): unlock + return solver's stake
    // Without this, every successful solver permanently loses their stake → Inv 3 violation.
    let solver_stake_locked = q.economic_state_t.stakes_t.get(target.solver, target.task_id);
    q_next.economic_state_t.stakes_t.unlock(target.solver, target.task_id);
    q_next.economic_state_t.balances_t.credit(target.solver, solver_stake_locked);

    // 3b: credit reward + finalize claim + debit escrow
    q_next.economic_state_t.balances_t.credit(target.solver, reward);
    q_next.economic_state_t.claims_t.finalize(claim_id, reward);
    q_next.economic_state_t.escrows_t.debit(claim.task_id, reward);

    // 3c: pay royalties along royalty_graph_t edges (v1.2 explicit rounding rule per Codex Q3 + Q10)
    // Royalty math uses i64 micro-coin throughout; rounding = integer floor (round-down) to preserve Inv 3.
    // No f64; no implicit casts. weight stored as MicroFraction (i64 in 1_000_000 units representing 0.0..1.0).
    let reward_micro = reward.to_micro_units();    // i64
    for edge in q.economic_state_t.royalty_graph_t.edges_from(claim.target_work_tx) {
        let royalty_micro = reward_micro
            .checked_mul(edge.weight.micro_units())
            .expect("overflow")
            / 1_000_000;    // integer floor; deterministic across platforms
        let royalty = MicroCoin::from_micro_units(royalty_micro);
        q_next.economic_state_t.balances_t.credit(edge.creator, royalty);
        q_next.economic_state_t.balances_t.debit(target.solver, royalty);  // royalty comes from solver's reward, not extra mint (Inv 4)
    }
    // Note: integer floor means total royalty payments may be < `reward × Σ weights` by up to `n` micro-units (1 per edge);
    // the dust remains in solver's balance. This is intentional and consistent with Bitcoin satoshi rounding.

    // STAGE 4: emit terminal signals
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &FinalizeTx::from(claim_id, reward));
    q_next.state_root_t  = materializer::apply(&q.state_root_t, &FinalizeTx::from(claim_id, reward));
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    Ok((q_next, SignalBundle::finalize(claim_id, reward)))
}
```

### 3.6 task_expire_transition (v1.3 refactored: pure boundary preserved per Codex re-audit)

**Why**: a TaskMarket entry has a deadline; if no work_tx is accepted by deadline, the bounty MUST refund to task creator (otherwise Inv 3 monetary conservation broken: bounty trapped in escrow forever).

**v1.3 fix**: split runtime side effects (signing, logical time assignment) from pure transition. Runtime constructs `TaskExpireTx` BEFORE entering pure transition; pure `task_expire_transition` takes already-signed tx as argument. This restores § 2 + § 3 pure-boundary discipline (Codex Q1.3 + new-issue #1 fix).

```rust
// PURE transition (used by both branch A and branch B in STEP_B)
pub fn task_expire_transition(
    q: &QState,
    tx: &TaskExpireTx,    // v1.3: already-signed by runtime BEFORE entry
) -> Result<(QState, SignalBundle), TransitionError> {
    let task = q.economic_state_t.task_markets_t.get(tx.task_id)
        .ok_or(TransitionError::TaskNotFound)?;

    // STAGE 1: signature verification (system signature; not agent)
    if !verify_system_signature(&tx.system_signature, &tx, q.system_pubkey_at_epoch(tx.epoch)) {
        return Err(TransitionError::InvalidSystemSignature);
    }

    // STAGE 2: parent_state_root match (stale view rejection)
    if tx.parent_state_root != q.state_root_t {
        return Err(TransitionError::StaleParent);
    }

    // STAGE 3: expiry check — task must be expired AND have NO Pending OR Provisional OR Finalized claim
    // v1.3 fix (Codex new-issue #2): broaden race-protection from "Finalized only" to all claim statuses
    if task.deadline_logical_t > q.q_t.current_round {
        return Err(TransitionError::TaskNotExpired);
    }
    if q.economic_state_t.claims_t.any_claim_for_task(tx.task_id) {
        return Err(TransitionError::TaskHasOpenClaim);    // refund only if NO claim exists at all
    }

    // STAGE 4: refund bounty from escrow to task creator
    let mut q_next = q.clone();
    let bounty = q.economic_state_t.escrows_t.get(tx.task_id);
    q_next.economic_state_t.escrows_t.refund(tx.task_id);
    q_next.economic_state_t.balances_t.credit(task.creator, bounty);

    // STAGE 5: refund any solver stakes still locked on expired task
    for (agent, locked_stake) in q.economic_state_t.stakes_t.all_locked_for_task(tx.task_id) {
        q_next.economic_state_t.stakes_t.unlock(agent, tx.task_id);
        q_next.economic_state_t.balances_t.credit(agent, locked_stake);
    }

    // STAGE 6: remove task from active markets
    q_next.economic_state_t.task_markets_t.remove(tx.task_id);

    // STAGE 7: append + materialize + signal (purely on tx, q)
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle::task_expired(tx.task_id, bounty);

    Ok((q_next, signals))
}

// TaskExpireTx schema (v1.3 NEW typed schema):
pub struct TaskExpireTx {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub parent_state_root: Hash,
    pub bounty_refunded: MicroCoin,         // for ledger summary; runtime computes from q
    pub epoch: SystemEpoch,                  // which keypair signed
    pub timestamp_logical: u64,              // assigned by runtime BEFORE pure transition
    pub system_signature: SystemSignature,   // computed by runtime BEFORE pure transition
}
```

**Trigger**: runtime tick scans for expired tasks; for each, runtime:
1. Calls `runtime.next_logical_t()` to get next logical_t
2. Constructs `TaskExpireTx` with current `q.state_root_t` as parent
3. Signs `TaskExpireTx` via `runtime.system_keypair().sign(canonical_digest(&tx))`
4. Submits signed tx to L4 sequencer (§ 5.2.1)
5. Sequencer calls pure `task_expire_transition(q, &tx)`

This split is identical to how `WorkTx` is constructed by agent BEFORE submitting to pure `step_transition`. Agents construct + sign; runtime constructs + sign for system tx. Pure transition fn is `(q, tx) → (q', signals)` in BOTH cases.

### 3.6.5 Agent Implicit Init (v1.3 fixed: trait-based submitter resolution per Codex re-audit Q1.4)

**Where**: applies to ALL agent-submitted transitions (work_transition / verify_transition / challenge_transition / reuse_transition). Inline at stage 4 of each, before user-state mutations.

**v1.3 fix**: WorkTx has `agent_id`; VerifyTx has `verifier_agent`; ChallengeTx has `challenger_agent`; ReuseTx has no submitting-agent field (it's a fact-tx). Introduce a `Tx::submitter_id() -> Option<AgentId>` trait method that each tx implements explicitly:

```rust
pub trait HasSubmitter {
    fn submitter_id(&self) -> Option<AgentId>;
}

impl HasSubmitter for WorkTx       { fn submitter_id(&self) -> Option<AgentId> { Some(self.agent_id.clone()) } }
impl HasSubmitter for VerifyTx     { fn submitter_id(&self) -> Option<AgentId> { Some(self.verifier_agent.clone()) } }
impl HasSubmitter for ChallengeTx  { fn submitter_id(&self) -> Option<AgentId> { Some(self.challenger_agent.clone()) } }
impl HasSubmitter for ReuseTx      { fn submitter_id(&self) -> Option<AgentId> { None }    // ReuseTx has no submitting agent; reuse facts derive from L4 read_set }

// In each agent-submitted transition's stage 4, INLINE this snippet:
fn implicit_init_agent_if_new(q_next: &mut QState, tx: &impl HasSubmitter) {
    if let Some(submitter) = tx.submitter_id() {
        if !q_next.q_t.agents.contains_key(&submitter) {
            q_next.q_t.agents.insert(submitter, PerAgentState {
                reputation_snapshot: Reputation::default_initial(),    // = 0
                last_accepted_tx: None,
                retry_counter_for_current_task: 0,
            });
        }
    }
}
```

**Rule**: each transition function MUST call `implicit_init_agent_if_new(&mut q_next, tx)` as the FIRST statement of stage 4 (after stage 3 predicate gate, before any user-state mutation). For `ReuseTx`, `submitter_id()` returns None; no init happens; that's intentional (ReuseTx has no submitting agent to init).

**Why implicit (not explicit `register_agent_transition`)**:
- Satoshi parallel: Bitcoin addresses are implicitly created at first use; no separate register step
- Avoids gatekeeping: any agent submitting a valid signed tx joins the system
- v4 single-user friendly: gretjia + Codex/Gemini auto-discoverable
- v4.1+: if needed, can add explicit `agent_register_tx` later WITHOUT breaking implicit-init (new tx is purely additive)

### 3.7 emit_terminal_summary (run-end without acceptance)

```rust
pub fn emit_terminal_summary_transition(
    q: &QState,
    run_id: RunId,
    runtime: &Runtime,
) -> Result<(QState, SignalBundle), TransitionError> {
    let run = runtime.run_state(run_id)?;
    if run.has_accepted_work_tx() {
        return Err(TransitionError::TerminalSummaryNotApplicable);  // only emitted for no-accept runs
    }

    let summary = TerminalSummaryTx {
        tx_id: TxId::derive(run_id, "terminal"),
        task_id: run.task_id,
        run_id,
        run_outcome: run.outcome(),
        total_attempts: run.attempt_counter(),
        failure_class_histogram: run.failure_histogram(),
        last_logical_t: run.last_logical_t(),
        system_signature: runtime.system_keypair().sign(canonical_digest_terminal(run)),
    };

    // STAGE: append; materialize; emit failure-class signals to L6
    let mut q_next = q.clone();
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &summary);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, &summary);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle::terminal_summary(&summary);

    Ok((q_next, signals))
}
```

---

## § 4 Named Invariants (machine-checkable)

| ID | Invariant | Enforced at | Conformance test |
|---|---|---|---|
| I-DET | Same (Q_t, tx) → byte-identical (Q_{t+1}, signals) | step_transition stage 6-8 | `tests/transition_determinism.rs` |
| I-DETHASH | `state_root_t` after replay from genesis matches authoritative state | replay test | `tests/q_state_reconstruct.rs` |
| I-NOSIDE | step_transition reads only (q, tx, registries); no I/O | static analysis grep + cargo-deny | `tests/no_hidden_inputs.rs` |
| I-PARENT | tx.parent_state_root must equal q.state_root_t | stage 1 | `tests/stale_parent_rejection.rs` |
| I-SIG | tx.signature verifies against tx.canonical_digest() | stage 2 | `tests/signature_verification.rs` |
| I-STAKE | tx.stake ≤ q.balances_t[tx.agent_id]; debit atomic | stage 3, 6 | `tests/stake_atomicity.rs` |
| I-PRED-GATE | rejected work_tx does NOT advance state_root_t | stage 4 | `tests/economic_invariant_INV6_predicate_gated.rs` |
| I-PROV | accepted work_tx → provisional claim, NOT finalized reward | stage 5 | `tests/economic_invariant_INV7_provisional_then_final.rs` |
| I-LOGTIME | timestamp_logical strictly monotonic per-tx; no wall clock | stage 6 | `tests/no_wall_clock_in_tx.rs` |
| I-MICROCOIN | all monetary fields are MicroCoin (i64 newtype) | type system | compile-time + `tests/no_f64_money.rs` |
| I-BTREE | Q_t indices use BTreeMap, not HashMap (deterministic order) | type system | `tests/q_state_uses_btree.rs` |
| I-NOSIDECAR | no Vec/HashMap "graveyard"-like sidecar (Art. 0.2) | static analysis | `tests/no_rejection_sidecar.rs` |
| I-RETRY | RejectedAttemptSummary stamped by runner, not agent | stamp call site | `tests/retry_summary_runner_signed.rs` |
| I-TERMINAL | every run terminates with at least one of: accepted work_tx OR TerminalSummaryTx | run finalize hook | `tests/run_terminal_invariant.rs` |
| I-NOENV | step_transition dependency tree contains no `std::env` access | cargo-deny + grep | `tests/no_env_in_transition.rs` |
| I-FREEZE-CONFIG | TAPE_ECONOMY_V2 + FOUNDER_GRANT_GAMMA + system_lp_amount frozen at task creation, not at tx submission | TaskMarket::publish | `tests/task_config_frozen_at_publish.rs` |
| **I-NORANDOM** (added per Gemini v3.2 review Q1) | Any tx that consumes randomness MUST seed PRNG from `(tx.tx_id, q.state_root_t)`; no system entropy in step_transition path | step_transition stages 1-7 | `tests/no_runtime_entropy.rs` |
| **I-VERIFY-LIVE** (added per Gemini v3.2 review Q10) | VerifyTx targets MUST be in Pending or Provisional state; cannot verify Accepted-and-finalized or Slashed | verify_transition stage 1 | `tests/verify_target_liveness.rs` |
| **I-CHAL-WINDOW** (added per Gemini v3.2 review Q10) | ChallengeTx must be received within target's challenge_cases_t window; no challenges after window close | challenge_transition stage 1 | `tests/challenge_window_enforced.rs` |
| **I-FINALIZE-EXCLUSIVE** (added) | FinalizeRewardTx and SlashTx are mutually exclusive per claim_id; system runtime serializes | finalize_reward_transition stage 2 | `tests/finalize_or_slash_exclusive.rs` |
| **I-VBOND-RELEASE** (v1.1, gap 11.2 fix) | Verifier bond release on slashed work_tx follows TaskMarket.config.verifier_bond_on_slash policy; default = `ReturnToVerifier`; verifier reputation NOT adjusted under default policy | challenge_transition stage 4e | `tests/verifier_bond_release.rs` |
| **I-ROYALTY-CAP** (v1.1, gap 11.3 fix) | reuse_tx edge weight ≤ TaskMarket.config.max_reuse_royalty_fraction (default 0.10); excess clamped + warning logged | reuse_transition stage 3 | `tests/royalty_cap_enforced.rs` |
| **I-STAKE-RETURN** (v1.2 NEW per Gemini Q2 + Codex Q2) | Successful unchallenged finalize_reward returns + unlocks solver's locked stake exactly once (in addition to reward credit). Test attempts double-claim. | finalize_reward_transition stage 3a | `tests/stake_return_on_finalize.rs` |
| **I-BOUNTY-REFUND** (v1.2 NEW per Gemini Q2 + Codex Q2) | task_expire_transition refunds full bounty to creator + refunds any locked solver stakes when no claim finalized by deadline | task_expire_transition stages 2-3 | `tests/bounty_refund_on_expire.rs` |
| **I-FINALIZE-BATCH-ORDER** (v1.3 corrected: single key throughout) | When N claims become finalizable at the same logical_t, finalize_tx emit order is `(expires_at_logical ASC, claim_id ASC)` — `claim_id` (NOT `target_work_tx`) used everywhere: invariant + § 5.2.3 + conformance test all consistent. | runtime finalize loop + § 5.2.3 | `tests/finalize_batch_order.rs` |
| **I-CHALLENGE-WINDOW-EDGE** (v1.3 finalize binding fixed) | Challenge window is `[opens_at, opens_at + duration_ticks)` — left-inclusive, right-exclusive. `is_open(now)` defined as `now < opens_at + duration_ticks`. **Both** challenge_transition stage 1 AND finalize_reward stage 1 MUST use `is_open(q.q_t.current_round)` (NOT a different rule). | challenge_transition + finalize_reward_transition | `tests/challenge_window_edge.rs` |
| **I-AGENT-INIT** (v1.2 NEW per Gemini Q2) | First appearance of agent in L4 transition tx implicitly initializes q_t.agents[id] with reputation=0; subsequent appearances do not re-initialize | work/verify/challenge/reuse_transition stage 4 | `tests/agent_implicit_init.rs` |

**Total: 27 invariants → 27 tests** (was 22 in v1.1; +5 in v1.2). Every transition test must pass before CO1.1.4 (bus.rs split) starts. STEP_B implementation comparison is "branch X conforms to spec" / "branch Y conforms to spec", not "branch X looks like branch Y".

---

## § 5 Optional TLA+ Skeleton (deferred to spec-gate audit)

For ordering + replay invariants (I-DET, I-DETHASH, I-LOGTIME), Codex suggested TLA+/PlusCal. ArchitectAI agrees with the suggestion but does NOT include the full model in v1 of this spec — it would balloon the doc. Skeleton:

```tla
EXTENDS Naturals, Sequences

VARIABLES q, ledger, signals

Init == /\ q = GenesisQState
        /\ ledger = <<>>
        /\ signals = <<>>

Step(tx) == /\ ValidParent(tx, q)
            /\ ValidSignature(tx)
            /\ StakeAvailable(tx, q)
            /\ AcceptancePredicates(tx, q)
            /\ q' = Apply(q, tx)
            /\ ledger' = Append(ledger, tx)
            /\ signals' = EmitSignals(q, tx, q')

Spec == Init /\ [][\E tx \in WorkTx : Step(tx)]_<<q, ledger, signals>>

\* Determinism: same input sequence → same final state
DeterminismProperty == \A seq1, seq2 \in Seq(WorkTx) :
    (seq1 = seq2) => (Replay(seq1) = Replay(seq2))
```

If CO P1 audit demands stronger guarantees, the TLA+ model is upgraded to a full PlusCal program with TLC model checking. For v4 scope, the type-level + conformance-test combination is deemed sufficient by Codex.

---

## § 5.1 v1.1 Walk-Through Gap Resolutions

Per `SPEC_WALKTHROUGH_v1_2026-04-27.md` § 11, four spec gaps were found. Resolution status:

| Gap | Issue | v1.2 Resolution | User-overridable |
|---|---|---|---|
| 11.1 | False-challenge reputation penalty undefined | **v1.2 (Codex Q10 fix)**: false_challenge_reputation_penalty is **fixed to 0 in v4** (NOT configurable). Pseudocode resolves contradiction: challenge_transition stage 3 returns `Err(CounterexampleInsufficient)` BEFORE any state mutation; no executable path for nonzero penalty → "configurable" prose retired. v4.1+ MAY add explicit `failed_challenge_penalty_transition` if needed. | NO (v4) |
| 11.2 | Verifier bond release policy on slashed claim | spec § 3.2 stage 4e ADDED with `VerifierBondPolicy::ReturnToVerifier` default | yes — `verifier_bond_on_slash` config |
| 11.3 | Royalty edge weight bound | spec § 3.3 stage 3 ADDED with `MAX_REUSE_ROYALTY_FRACTION_DEFAULT = 0.10` | yes — `max_reuse_royalty_fraction` config |
| 11.4 | Multi-verifier quorum aggregation | spec § 3.1 note ADDED with `verifier_quorum_required: usize = 1` default; full multi-verifier impl deferred to CO P2.7 | yes — set per TaskMarket |

All 4 gaps now have machine-checkable defaults. User can override 11.2/11.3/11.4 defaults via TaskMarket.config when creating tasks; the default applies if config field is missing. **11.1 (false-challenge penalty) is NOT user-overridable in v4** (fixed to 0; v4.1+ may introduce a separate `failed_challenge_penalty_transition` if needed).

---

## § 5.2 Concurrency Rules (v1.2 NEW per Codex Q6)

**Why**: spec § 3 pseudocode is single-threaded; CO P1 may parallelize Phase C 5 modes × N seeds. Without explicit serialization rule, two work_tx can race on same parent_state_root, both pass `I-PARENT`, but produce different (logical_t, tx_id) sequences across STEP_B branches → state_root divergence.

### 5.2.1 L4 Sequencer

**Per (runtime_repo, run_id)** there is exactly ONE L4 sequencer instance. The sequencer:
1. **Receives** tx submissions in any order (concurrent-safe queue)
2. **Assigns** monotonic `(logical_t, tx_id)` ordering key:
   - `logical_t = sequencer.next_logical_t()` (atomic counter; starts at 1 per genesis)
   - `tx_id = TxId::derive(logical_t, agent_id, payload_hash)` (deterministic from above)
3. **Serializes** transition execution: takes 1 tx at a time from queue in submission order; calls pure `step_transition`
4. **Commits** result to L4 (WAL write + git commit) BEFORE accepting next tx

**Async completion order is NEVER an ordering source**. Even if async tasks finish out-of-order, sequencer enforces submission-order ingestion.

### 5.2.2 Cross-Cell Isolation

**Phase C 5-mode × 10-problem × N-seed cells** (per `CO1_3_1_GIX_SPIKE_PREFLIGHT § 1` C4) MUST use:
- **Disjoint `runtime_repo`** (different filesystem path; no shared state)
- **Disjoint `QState`** (each cell has its own genesis_payload + Q_t replay)
- **No shared L4 sequencer** (each cell has its own)

If a future deployment shares runtime_repo across cells (e.g., multi-tenant): MUST add **ref locks** (gix branch refs serve as atomic guards) + **deterministic retry semantics** (failed lock → wait 100ms × n_attempts; deterministic seed from `(run_id, tx_id)`).

### 5.2.3 Finalize Batch Order

When N claims expire at the same `logical_t`:
- Order = `(claim.expires_at_logical ASC, claim.claim_id ASC)` (stable, deterministic) — v1.3 fix: uses `claim_id` consistently (NOT `target_work_tx`) to align with `I-FINALIZE-BATCH-ORDER` invariant + conformance test
- Sequencer emits `finalize_reward_transition` ONE AT A TIME in this order
- Each finalize advances state_root before next finalize starts

### 5.2.4 Conformance Tests

- `tests/l4_sequencer_serialization.rs` — concurrent submit; assert single-threaded execution by sequencer; same input order → same state_root
- `tests/cross_cell_isolation.rs` — 5 cells run; assert disjoint state_roots; no cross-contamination
- `tests/finalize_batch_order.rs` — 3 claims expire same tick; assert ordering by (expires_at, claim_id); 2 runs byte-identical

### 5.2.5 ChallengeWindow::is_open (v1.4 NEW per Codex Q2.4)

```rust
impl ChallengeWindow {
    /// Half-open interval `[opens_at, opens_at + duration_ticks)`.
    /// Both challenge_transition stage 1 AND finalize_reward stage 1 MUST invoke this method
    /// (NOT hand-code the inequality) to guarantee consistent edge semantics.
    pub fn is_open(&self, now: u64) -> bool {
        now >= self.opens_at && now < self.opens_at + self.duration_ticks
    }
}
```

**Invariant binding**: `I-CHALLENGE-WINDOW-EDGE` enforces that BOTH transition functions call `is_open(now)` rather than hand-coding the boundary check. STEP_B branch A vs branch B both implement the same `is_open()`; cross-branch comparison verifies identical results for all (opens_at, duration_ticks, now) triples.

### 5.2.6 Sequencer Tie-Break (v1.4 NEW per Codex Q6)

When multiple agent threads concurrently call sequencer's `submit(tx)`, the sequencer's atomic `next_logical_t()` (§ 5.2.1 step 2) provides the **canonical tie-breaker**:

- `logical_t` assignments are produced by atomic increment (e.g., `AtomicU64::fetch_add`)
- The order in which threads receive their `logical_t` values IS the canonical ordering
- "Submission order" = the order of `logical_t` assignment, NOT wall-clock arrival order
- For two `tx` arriving at the same nanosecond on different threads, whichever thread wins the atomic gets the lower `logical_t`; the other gets the next higher

This means: STEP_B branch A and branch B may serialize threads differently (depending on OS scheduler), but as long as both branches use atomic logical_t assignment + replay from the SAME logical_t sequence, they produce byte-identical state_roots.

**Conformance test addition** (extends `tests/l4_sequencer_serialization.rs`): submit 100 tx concurrently from 8 threads; assert `(logical_t, tx_id_hash)` is a strict total order; replay produces deterministic state_root regardless of thread interleaving.

### 5.2.7 What This Does NOT Specify

- Async runtime choice (tokio vs std::thread): runtime concern, not spec; spec only requires sequencer property
- Sequencer implementation: lock-free queue, mutex, channel — implementation detail
- Cross-cell sharing pattern (post-v4): future v4.x extension

## § 5.3 Legacy Economic Tx Disposition (v1.3 NEW per Codex Q1.1)

The current pre-CO-P1 codebase contains economic mutation surfaces in `src/bus.rs` and `src/kernel.rs` that have NO direct equivalent in v1.x typed transitions:

| Legacy mutation | Current location | v4 disposition |
|---|---|---|
| `Invest` event (agent stakes Coin to YES/NO market position) | `src/bus.rs:229-252,285-290` `handle_invest_only` + market interactions | **RETIRED in CO1.1.4** — agent staking now goes through `WorkTx.stake` (YES_E) or `ChallengeTx.stake` (NO_E); no separate InvestTx. |
| `TaskMarketPublish` (task creator publishes new task) | implicit in current code; tasks hardcoded | **NEW v1 transition (deferred to CO P2.1)** — `TaskMarketPublishTx` lands in CO P2.1 atom; v1.x spec stubs the schema only |
| `MarketCreate` (per-node market on each tape append) | `src/bus.rs:285-290` + `src/kernel.rs:114-126` `Kernel::create_market` | **RETIRED in CO1.1.5** — per-node markets are an artifact of the Phase A "every node = market" pattern; CO P2.1 TaskMarket replaces with per-task markets only |
| `MarketResolve` (settle markets at OMEGA accept) | `src/kernel.rs:156-206` `Kernel::resolve_all` | **RETIRED in CO1.1.5** — market resolution becomes part of `finalize_reward_transition` (per-task, per-claim); no separate market-resolve event |
| `RunEnd` / `halt_and_settle` (run-level settlement) | `src/bus.rs:355-375` `TuringBus::halt_and_settle` | **RETIRED in CO1.1.4** — run-end becomes implicit via `TerminalSummaryTx` (§ 3.7) for no-accept runs OR `finalize_reward_transition` for accepted runs |
| WAL append side effect | `src/bus.rs:273-282` + `:319-327` | **MOVED to runtime layer**, not transition: spec § 5.2.1 sequencer commits L4 entries AFTER pure `step_transition` returns |
| Tool post-append hook | `src/bus.rs:312-318` `tool.on_post_append()` | **RETIRED**: tool hooks become explicit ToolInvocation field in `WorkTx.write_set` (read by predicate runner); no separate hook |

**Conformance test**: `tests/legacy_economic_tx_retired.rs` greps post-CO1.1.4/CO1.1.5 codebase for: `Invest` event variant, `Kernel::create_market`, `Kernel::resolve_all`, `halt_and_settle`, `tool.on_post_append`. Each must return 0 hits in the new `src/{top_white,middle_black,bottom_white,economy,state,transition}/*` dirs (matches in old `src/{bus,kernel}.rs` ARE expected if those files still exist as legacy markers; CO1.1.4 atom retires them).

**Why retired-not-renamed**: each legacy operation is either (a) absorbed into a v1.x typed transition (Invest → WorkTx.stake; Resolve → finalize_reward) OR (b) moved to runtime layer (WAL append; tool hook). Direct rename would preserve the old monolithic semantics.

## § 6 What This Spec DOES NOT Specify

Listed for honesty:

1. **MetaTx full schema** — only stub here; v4.1 atom defines.
2. **AttributionEngine deterministic DAG construction** — CO2.4.0 spike (separate doc).
3. **Predicate visibility leak channels** — covered at CO P1.5 design (Goodhart shield); this spec only declares `BoolWithProof.proof_visibility_class`, not the leak-proof proof format.
4. **gix Path B substrate-specific operations** — CO1.3.1 spike validates; this spec is substrate-agnostic.
5. **Retry metadata bound on `failed_attempts_since_last_accept`** — must be finite for tape size containment, but exact bound (e.g., u32::MAX vs cap-at-1000) is CO P1.7 design choice.
6. **Verifier verdict aggregation rule** — when N verifiers vote, how to combine? CO2.7 design.
7. **Challenge window length** — `CHALLENGE_WINDOW_TICKS` is a TaskMarket config bound at publish, but the default value + bounds are CO2.5 design.

These deferrals are **explicit and named**. Future atoms reference this list to resolve them.

---

## § 7 Pre-CO P1 Gate Procedure

1. ArchitectAI commits this spec v1
2. Codex independent review: confirm that every WP § 4-7 + economic § 2/§ 6 / § 18-21 concept maps to a typed field or invariant here
3. Gemini cross-review: confirm spec respects ENTIRE white paper (not just cited §)
4. Both PASS → spec frozen as v1 (any change requires re-audit)
5. **Then** Plan v3.2 atom CO1.SPEC.0 marked complete; CO1.0 / CO1.1.* / CO1.2.* atoms cleared to start
6. STEP_B implementation: Claude implements branch A against spec; Codex implements branch B against spec; comparison metric = "spec conformance", not "code similarity"

---

## § 8 Honest Acknowledgements

What this spec is:
- A typed, deterministic, side-effect-free state transition definition
- A binding contract for STEP_B branch A/B comparison
- A list of **27 named invariants** (was 16 in v1; 22 in v1.1; +5 in v1.2: I-STAKE-RETURN / I-BOUNTY-REFUND / I-FINALIZE-BATCH-ORDER / I-CHALLENGE-WINDOW-EDGE / I-AGENT-INIT) each backed by a conformance test path

What this spec is NOT:
- A full formal proof (no Lean/Coq)
- A complete TLA+ model (skeleton only)
- A substitute for code review (still required per Protocol Hard rule 1+2)
- A guarantee that branches A/B will produce identical Rust code (only spec-equivalent code)

What this spec does NOT yet include and the user must decide:
- Whether to run full TLA+ TLC model check (~3-5 day effort) or stop at type+test level (Codex suggested optional)
- Whether `RejectionClass::Opaque` aggregation respects Goodhart shield in practice (deferred to CO P1.5)
- Whether to embed Art 0.2 mini-amendment (see `ART_0_2_REINTERPRETATION_2026-04-27.md`) BEFORE running this spec, or AFTER (depends on rejection-on-tape constitutional reading)

— ArchitectAI, 2026-04-27


---

# XREF: SYSTEM_KEYPAIR_SECURITY_v1 (frozen, CO1.7.0a-f)

# System Keypair Security Spec v1

> **Date**: 2026-04-27
> **Purpose**: Gemini v3.2 review Q9 VETO — runtime system keypair (signs `RejectedAttemptSummary` + `TerminalSummaryTx`) lifecycle was unspecified; this doc defines generation, storage, rotation, threat model, and audit gates.
> **Authority**: Constitution Art. 0–0.4 (Tape Canonical) + Art. V.1 (separation of powers).
> **Audit**: Gemini v3.2 cross-review (2026-04-27) marked Plan v3.2 VETO until this exists.

---

## § 1 Why a system keypair exists

D-VETO-6 / Codex CHALLENGE: failure metadata cannot be agent self-reported (wrong trust boundary). The runtime's white-box predicate runner stamps `RejectedAttemptSummary` onto next accepted `WorkTx`, and emits `TerminalSummaryTx` on no-accept runs. These stamps must be **cryptographically bound to the runtime instance**, not forgeable by any agent.

Hence: a runtime-level `system_keypair` distinct from any agent keypair.

---

## § 2 Threat Model

### Adversary capabilities
- A1: malicious agent inside swarm — can submit work_tx / verify_tx / challenge_tx with arbitrary contents but **cannot sign as system**
- A2: external attacker without runtime access — can read public tape; cannot inject tx
- A3: external attacker with read access to runtime host filesystem — can attempt to extract keypair from disk
- A4: external attacker with full runtime host compromise — can read memory, sign arbitrary system tx until detected
- A5: malicious code path inside runtime (e.g., compromised dependency) — can call sign API arbitrarily

### What the keypair MUST defend against
- A1 forging system signatures → **defended by signature verification at append time**
- A2 reading retry metadata for Goodhart attack → **partially defended by L1 visibility filter**; system signature is orthogonal
- A3 extracting key from rest disk → **defended by encrypted-at-rest with user-derived KDF**

### What the keypair CANNOT defend against
- A4 full host compromise → **acknowledged loss**; mitigation = epoch rotation + L4 anchor proves PAST tx still valid
- A5 internal call abuse → **partially mitigated** by sign API only callable from `predicate_runner` + `terminal_summary_emitter` modules; static analysis enforces

### Recovery requirement
If A4/A5 detected: user can rotate to a new system keypair via Art V.3 amendment + signed git tag. Old key remains pinned in L4 history for replay verification but is invalidated for new tx. Future tx use new key.

---

## § 3 Keypair Lifecycle

### 3.1 Generation

**When**: at runtime first boot (post-genesis verification).

**How**:
```rust
// src/bottom_white/ledger/system_keypair.rs (NEW per CO1.7.0b)
pub fn generate_or_load_system_keypair(
    keystore_path: &Path,
    user_kdf_password: &SecretString,
) -> Result<Ed25519Keypair, KeypairError> {
    if keystore_path.exists() {
        return load_existing_keypair(keystore_path, user_kdf_password);
    }

    // First boot: generate ed25519 keypair
    let keypair = Ed25519Keypair::generate_with_secure_entropy()?;

    // Encrypt with user-derived key (Argon2id KDF on user_kdf_password)
    let encrypted = encrypt_at_rest(&keypair, user_kdf_password)?;
    fs::write(keystore_path, encrypted)?;
    set_file_permissions(keystore_path, 0o600)?;

    Ok(keypair)
}
```

**Algorithm**: ed25519 (small, fast, deterministic signatures, side-channel resistant).
**Entropy**: from `getrandom(2)` (Linux) / `SecRandomCopyBytes` (macOS); never from agent input or PRNG seeded by tape.

### 3.2 Storage

**At rest**:
- Path: `~/.turingos/keystore/system_keypair_v{epoch}.enc` (NOT in repo, NOT in any cas/ledger directory)
- Permissions: 0o600 (user read/write only)
- Encryption: ChaCha20-Poly1305 with key derived from user password via Argon2id (32-byte derived key)
- KDF parameters: Argon2id with m=64MB, t=3, p=4 (current OWASP recommendation as of 2026-04)
- Salt: stored alongside encrypted blob; randomly generated on first encryption

**In memory**:
- Loaded once at boot; held in `Arc<Ed25519Keypair>` inside `Runtime` struct
- Memory-locked via `mlock(2)` to prevent swap to disk
- Zeroized on Runtime drop (`zeroize` crate)
- NEVER serialized to ledger/CAS/log
- NEVER passed to agent code or LLM payload

### 3.3 Sign API contract

```rust
// Only callable from these two paths, enforced by `pub(restricted)` + cargo-deny
pub(restricted = predicate_runner, terminal_summary_emitter)
fn sign_system_message(
    keypair: &Ed25519Keypair,
    message: &CanonicalMessage,
) -> SystemSignature {
    keypair.sign(canonical_digest(message))
}
```

`CanonicalMessage` is a typed enum — `RejectedAttemptSummary | TerminalSummaryTx | EpochRotationProof`. No free-form message signing exposed.

### 3.4 Verification (public)

```rust
pub fn verify_system_signature(
    sig: &SystemSignature,
    message: &CanonicalMessage,
    epoch: SystemEpoch,
    pinned_pubkeys: &PinnedSystemPubkeys,
) -> bool {
    let pk = pinned_pubkeys.get(epoch).expect("epoch pubkey missing");
    pk.verify(canonical_digest(message), sig)
}
```

`pinned_pubkeys` is loaded from `genesis_payload.toml` `[system_pubkeys]` section (NEW per this spec). Each epoch has a public key entry; private key is in encrypted keystore.

### 3.5 Rotation

**Trigger conditions**:
- Suspected compromise (A4/A5)
- Scheduled rotation: every 12 months from epoch start (long enough for v4 timeline)
- Architecture amendment that changes signature algorithm

**Procedure**:
1. User runs `cargo run --bin rotate-system-keypair -- --new-epoch=<N+1>`
2. Tool generates fresh ed25519 keypair
3. Tool emits `EpochRotationProof` signed by **both** old (epoch N) and new (epoch N+1) keys, certifying continuity
4. User PGP-signs the rotation: `git tag -s v4-syskey-rotate-N-to-N+1`
5. `genesis_payload.toml` `[system_pubkeys]` updated to add new pubkey
6. AUDIT_LEDGER row added with: rotation timestamp, old/new fingerprint, rotation tag fingerprint, user signature verification

**After rotation**:
- All NEW tx signed with epoch N+1 key
- Old epoch N key retained in `[system_pubkeys]` for verifying historical L4 entries
- Old encrypted keystore moved to `~/.turingos/keystore/archive/system_keypair_v{N}.enc.archived`

### 3.6 Compromise response

If A4 host compromise detected (e.g., user notices unauthorized tx):
1. **STOP runtime immediately** (kill process, prevent further sign calls)
2. Verify via `git log` whether unauthorized commits exist; if so, trigger Art V.3 amendment
3. Run `cargo run --bin emergency-rotate-system-keypair`
4. Future runtime starts use new key
5. AUDIT_LEDGER + LATEST.md document the event
6. Constitution Art V.3 amendment if compromise window changes any L4 invariants

**Note**: post-compromise, OLD tx signed by compromised key remain VERIFIABLE (their pubkey still pinned). What's lost: confidence that those tx were emitted by legitimate runtime. User can choose to mass-mark them as "post-compromise quarantine" via amendment, but that's a policy decision, not a cryptographic one.

---

## § 4 Conformance Tests (5 new)

```
tests/system_keypair_generation.rs           — first-boot keypair generated; encrypted at rest; correct permissions
tests/system_keypair_load_and_decrypt.rs     — second-boot load with correct password succeeds; wrong password fails
tests/system_keypair_sign_only_from_runner.rs — static check: sign API not exported beyond predicate_runner + terminal_summary_emitter
tests/system_keypair_verify_correctness.rs   — round-trip: sign then verify, with correct epoch pubkey lookup
tests/system_keypair_rotation_proof.rs       — EpochRotationProof signed by BOTH old + new key; verifies continuity
```

---

## § 5 New `[system_pubkeys]` Section in genesis_payload.toml

Extend `GENESIS_MINIMAL_WITH_ANCHOR_v1` schema:

```toml
[constitution_root]
... (8 fields from genesis spec)

[system_pubkeys]
epoch_1 = "<base64 ed25519 pubkey>"
epoch_1_signed_at = "2026-04-27T..."
epoch_1_creator_pgp_sig = "<PGP sig of pubkey covering creator authorization>"
# Future epochs added here on rotation.
```

Boot extension:
```rust
pub fn verify_system_pubkeys(manifest: &GenesisPayload) -> Result<(), TrustRootError> {
    for (epoch, pk_entry) in &manifest.system_pubkeys {
        verify_pgp_signature(&pk_entry.creator_pgp_sig, &pk_entry.pubkey, &PINNED_CREATOR_PUBKEY)?;
    }
    Ok(())
}
```

---

## § 6 Interaction with B-1 PGP Tag Governance

Every system keypair rotation produces a PGP-signed git tag. This tag goes into AUDIT_LEDGER per the existing B-1 governance protocol. Net result: every system signature ever produced has a complete provenance chain:

```
runtime tx → system signature (epoch N)
            ↓
            epoch N pubkey (in genesis_payload.toml [system_pubkeys])
            ↓
            creator_pgp_sig (user authorized this epoch's pubkey)
            ↓
            user PGP key (pinned in boot.rs as PINNED_CREATOR_PUBKEY)
            ↓
            git tag v4-syskey-rotate-{from}-to-{to} (history of all rotations)
```

A complete tamper requires compromising user's PGP key, which is out of scope for runtime defense (user responsibility to protect personal cryptographic identity).

---

## § 7 Out of Scope

- **HSM (Hardware Security Module)**: v4 is software-only; v4.1+ may add HSM backing
- **Multi-party threshold signatures (FROST/MuSig)**: only relevant when multiple human architects join (currently solo); v5
- **Post-quantum signatures**: ed25519 ample for v4 horizon (5-10 years); revisit when post-quantum standards stabilize
- **TPM remote attestation**: cloud-deployment scenario; v5

---

## § 8 Honest Acknowledgements

What this spec achieves:
- Closes Gemini v3.2 Q9 VETO on undefined keypair lifecycle
- Defines threat model honestly (A1-A5)
- Acknowledges A4 host compromise as out-of-scope-for-cryptography
- Provides concrete rotation procedure + emergency response

What this spec is honest about:
- v4 keypair security is **as strong as user's password + filesystem permissions + memory hygiene** — not stronger
- Single-instance, software-only, single-user system; multi-party operations are v5
- A4 = full host compromise is a known unfixable hole; the design limits damage to "future tx forgery from compromise time forward", not retroactive

What this spec adds to v4 atom count:
- CO1.7.0c (system keypair lifecycle implementation): 5 atoms (gen + load + sign API + rotation tool + emergency rotation tool)
- 5 new conformance tests
- New `[system_pubkeys]` section in genesis_payload.toml (extends CO1.0)

— ArchitectAI, 2026-04-27


---

# XREF: WP v2.2 § 5.L4 + § 6 transition protocol

encryption_policy
```

而不是记录全部内容。

#### Layer 4：Transition Ledger

追加式状态转移账本。

```
tx_id
parent_state_root
agent_id
task_id
read_set
write_set
proposal_cid
predicate_results
stake
signature
timestamp
status
```

这层可以是：

- local hash chain
- append-only JSONL
- Git commit graph
- permissioned blockchain
- public-chain settlement proof

#### Layer 5：Materialized State and Agent Read View

物化状态层。

```
current_state_db
task_index
agent_reputation_index
error_taxonomy_index
price_signal_index
permission_view
```

Agent 实际读取的是这一层，而不是底层完整账本。

```
read_tool(agent_i, task_j, Q_t)
  -> minimal sufficient context
```

这正是选择性屏蔽与渐进式披露。

#### Layer 6：Signal Indices

信号索引层。

```
boolean pass/fail history
typical error clusters
price signals
reputation counters
resource scarcity indicators
exploration/exploitation statistics
```

> **L5 vs L6 区分 (Art 0.2 重建性合规)**: Layer 5 是**当前状态的物化视图**（agent 直接 read_tool 读到的最小充分上下文，含 `agent_reputation_index` / `price_signal_index`）。Layer 6 是**统计聚合 / 趋势压缩**（`reputation counters` 是窗口内增量统计，`price signals` 是市场微观结构压缩）。**L6 严格 derivable from L4 + L5**：删掉 L6 不破坏 Art 0.2 重建性，因为 L4 (transition ledger) 是 ground truth，L5 是 L4 物化，L6 是 L5 统计变换。L6 存在的唯一目的是**降低顶层白盒访问成本**（从 O(账本扫描) 降到 O(预聚合查询)）。

这层让顶层白盒能够持续进行量化、广播、屏蔽。

---

## 6. 状态转移协议

一次标准计算循环如下。

### 6.1 Read

```
input_i = rtool(Q_t, agent_i, task_j)
```

read tool 只返回当前任务需要的最小充分上下文。

```
input_i = <q_i, s_i, budget_view_i, allowed_tools_i>
```

其中：

```
q_i = 当前控制状态摘要
s_i = 局部 tape view
```

### 6.2 Generate

```
output_i = δ_i(input_i)
```


---

# XREF: shipped CO1.7.0a-f system_keypair source (for DIV-1 ground truth)

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemSignature([u8; SIGNATURE_LEN]);

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

/// TRACE_MATRIX FC1-Sig+FC3-Sig: typed terminal summary transaction emitted on no-accept runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalSummaryTx {
    run_id: String,
    terminal_state_root: [u8; 32],
    rejected_attempt_count: u64,
}

impl TerminalSummaryTx {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a typed terminal summary transaction.
    pub fn new(
        run_id: impl Into<String>,
        terminal_state_root: [u8; 32],
        rejected_attempt_count: u64,
    ) -> Self {
        Self {
            run_id: run_id.into(),
            terminal_state_root,
            rejected_attempt_count,
        }
    }
}

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
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: terminal summary transaction.
    TerminalSummaryTx(TerminalSummaryTx),
    /// TRACE_MATRIX FC3-Sig: system key epoch continuity proof.
    EpochRotationProof(EpochRotationProof),
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
        CanonicalMessage::TerminalSummaryTx(tx) => {
            h.update(b"TerminalSummaryTx");
            update_len_prefixed(&mut h, tx.run_id.as_bytes());
            h.update(tx.terminal_state_root);
            h.update(tx.rejected_attempt_count.to_be_bytes());
        }
        CanonicalMessage::EpochRotationProof(proof) => {
            h.update(b"EpochRotationProof");
            h.update(proof.old_epoch.get().to_be_bytes());
            h.update(proof.new_epoch.get().to_be_bytes());
            h.update(proof.old_pubkey.as_bytes());
            h.update(proof.new_pubkey.as_bytes());
            h.update(proof.signed_at_unix.to_be_bytes());
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

/// TRACE_MATRIX FC1-Sig+FC3-Sig: crate-only signing surface for terminal summary emission.
pub(crate) mod terminal_summary_emitter {
    use super::{
        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, EpochRotationProof,
        KeypairError, SystemSignature, TerminalSummaryTx,
    };

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: sign only typed terminal summary transactions.
    pub(crate) fn sign_terminal_summary_tx(
        keypair: &Ed25519Keypair,
        tx: &TerminalSummaryTx,
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(keypair, &CanonicalMessage::TerminalSummaryTx(tx.clone()))
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

# XREF: shipped CO1.4 CAS source (for DIV-4/5 ground truth)

```rust
//! CAS object schema per WP architecture § 5.L3.
//!
//! /// TRACE_MATRIX WP-arch-§5.L3: CAS object schema

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Content-addressed identifier — sha256 of payload bytes.
///
/// Distinct from git's SHA-1 OID (which is an internal storage detail of
/// the git2-rs backend). `Cid` is the v4-canonical identifier; spec § 1.2
/// `WorkTx.proposal_cid: Cid` references this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Cid(pub [u8; 32]);

impl Cid {
    /// Compute Cid from content bytes.
    pub fn from_content(content: &[u8]) -> Self {
        let mut h = Sha256::new();
        h.update(content);
        Self(h.finalize().into())
    }

    /// Hex-encoded representation (lowercase; 64 chars).
    pub fn hex(&self) -> String {
        let mut s = String::with_capacity(64);
        for b in self.0 {
            s.push_str(&format!("{:02x}", b));
        }
        s
    }
}

impl std::fmt::Display for Cid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cid:{}", self.hex())
    }
}

/// Type tag for CAS objects (replaces inline string-typed kind).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectType {
    /// Agent's work_tx proposal payload (Lean proof, code patch, etc.).
    ProposalPayload,
    /// Challenger's counterexample for a slashed claim.
    CounterexamplePayload,
    /// Predicate bytecode (Lean tactic, WASM module, Rust source bytes).
    PredicateBytecode,
    /// Tool bytecode.
    ToolBytecode,
    /// Constitution diff (for amendment proposals).
    AmendmentDiff,
    /// Reversibility plan attached to a meta_tx.
    ReversibilityPlan,
    /// Generic / unclassified blob.
    Generic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CasObjectMetadata {
    /// Content-addressed identifier (sha256 of content).
    pub cid: Cid,
    /// Backend-specific OID (git sha-1 for git2-rs backend); informational only.
    /// Different backends may have different OID schemes; Cid is canonical.
    pub backend_oid_hex: String,
    pub object_type: ObjectType,
    /// Submitter / author. Use "system" for runtime-emitted objects.
    pub creator: String,
    /// Logical time at insertion (assigned by sequencer; not wall clock).
    pub created_at_logical_t: u64,
    /// Optional schema identifier (JSON Schema URI, type tag, etc.).
    pub schema_id: Option<String>,
    /// Size of content in bytes (informational; not part of canonical hash).
    pub size_bytes: u64,
}

impl CasObjectMetadata {
    /// Canonical hash of metadata for Merkle tree inclusion.
    pub fn canonical_hash(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(self.cid.0);
        h.update(self.backend_oid_hex.as_bytes());
        h.update(serde_json::to_vec(&self.object_type).expect("object_type serialize"));
        h.update(self.creator.as_bytes());
        h.update(self.created_at_logical_t.to_be_bytes());
        if let Some(s) = &self.schema_id {
            h.update(b"\x01");
            h.update(s.as_bytes());
        } else {
            h.update(b"\x00");
        }
        h.update(self.size_bytes.to_be_bytes());
        h.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cid_from_empty_content() {
        let cid = Cid::from_content(b"");
        // SHA-256 of empty input = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        assert_eq!(
            cid.hex(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn cid_deterministic() {
        let cid_a = Cid::from_content(b"hello");
        let cid_b = Cid::from_content(b"hello");
        assert_eq!(cid_a, cid_b);
    }

    #[test]
    fn cid_differs_on_content() {
        let cid_a = Cid::from_content(b"hello");
        let cid_b = Cid::from_content(b"world");
        assert_ne!(cid_a, cid_b);
    }

    #[test]
    fn cid_display_format() {
        let cid = Cid::from_content(b"x");
        let s = cid.to_string();
        assert!(s.starts_with("cid:"));
        assert_eq!(s.len(), 4 + 64);
    }

    #[test]
    fn metadata_canonical_hash_deterministic() {
        let m = CasObjectMetadata {
            cid: Cid::from_content(b"x"),
            backend_oid_hex: "abc123".to_string(),
            object_type: ObjectType::ProposalPayload,
            creator: "alice".to_string(),
            created_at_logical_t: 100,
            schema_id: Some("v1/proposal".to_string()),
            size_bytes: 1,
        };
        assert_eq!(m.canonical_hash(), m.canonical_hash());
    }

    #[test]
    fn metadata_canonical_hash_differs_on_object_type() {
        let base = CasObjectMetadata {
            cid: Cid::from_content(b"x"),
            backend_oid_hex: "abc".to_string(),
            object_type: ObjectType::ProposalPayload,
            creator: "alice".to_string(),
            created_at_logical_t: 100,
            schema_id: None,
            size_bytes: 1,
        };
        let mut variant = base.clone();
        variant.object_type = ObjectType::CounterexamplePayload;
        assert_ne!(base.canonical_hash(), variant.canonical_hash());
    }
}
//! CAS store backed by git2-rs blob layer.
//!
//! Each runtime_repo (per spec § 5.2.2 cell isolation) has its own CasStore.
//! Objects are content-addressed by `Cid` (sha256 of content); git's sha-1
//! OID is recorded but not canonical.
//!
//! /// TRACE_MATRIX WP-arch-§5.L3 + spec-§5.2.2 (cell isolation): CAS store

use git2::{ObjectType as Git2ObjectType, Repository};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::schema::{CasObjectMetadata, Cid, ObjectType};

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
        }
    }
}

impl std::error::Error for CasError {}

impl From<git2::Error> for CasError {
    fn from(e: git2::Error) -> Self {
        Self::Git2(e)
    }
}

/// Content-addressable store backed by git's blob object database.
pub struct CasStore {
    repo_path: PathBuf,
    /// Cid → metadata index. BTreeMap per spec § 2 I-BTREE.
    index: BTreeMap<Cid, CasObjectMetadata>,
}

impl CasStore {
    /// Open or initialize a CAS store at the given runtime_repo path.
    /// Creates the git repo if it doesn't exist.
    pub fn open(repo_path: &Path) -> Result<Self, CasError> {
        let repo_path = repo_path.to_path_buf();
        // Init or open
        let _repo = match Repository::open(&repo_path) {
            Ok(r) => r,
            Err(_) => Repository::init(&repo_path)?,
        };
        Ok(Self {
            repo_path,
            index: BTreeMap::new(),
        })
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
}

```

---

Now give your INDEPENDENT round-1 audit. Cite spec § + line where possible.

codex
I’ll inspect the checked-in spec, skeleton, and local governance docs so the audit references the actual repository lines rather than only the pasted excerpt. After that I’ll compare the claims against the shipped CAS/keypair APIs and the frozen transition spec.
exec
/bin/bash -lc 'pwd && git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4
 M rules/enforcement.log
?? handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md
?? handover/audits/GEMINI_CO1_7_ROUND1_AUDIT_2026-04-28.md
?? handover/audits/run_codex_co1_7_round1_audit.sh
?? handover/audits/run_gemini_co1_7_round1_audit.py

exec
/bin/bash -lc "rg --files -g 'AGENTS.md' -g 'CLAUDE.md' -g 'STEP_B_PROTOCOL.md' -g '*TRANSITION_LEDGER*' -g 'transition_ledger.rs' -g 'system_keypair.rs' -g 'store.rs' -g 'schema.rs'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/cas/schema.rs
src/bottom_white/ledger/system_keypair.rs
src/bottom_white/cas/store.rs
src/bottom_white/ledger/transition_ledger.rs
CLAUDE.md
handover/ai-direct/STEP_B_PROTOCOL.md
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md

codex
The worktree already has audit artifacts and enforcement log changes unrelated to this review; I’m leaving them untouched. Next I’m pulling line-numbered slices around the schema, sequencer, replay, STEP_B, and shipped API definitions.
exec
/bin/bash -lc "nl -ba handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# CO1.7 Transition Ledger v1 — DRAFT outline
     2	
     3	**Status**: DRAFT outline awaiting round-1 dual external audit (Codex + Gemini).
     4	**Author**: ArchitectAI (Claude); session 2026-04-28.
     5	**Supersedes**: none (first cut).
     6	**Companion specs** (frozen, read first):
     7	- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — typed schemas + step_transition pseudocode + 27 invariants (round-4 PASS/PASS)
     8	- `SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md` — runtime keypair lifecycle (CO1.7.0a-f, done @ Wave 4-B)
     9	- `META_TRANSITION_INTERFACE_v1_2026-04-27.md` — trait pattern for L4 acceptance (deferred runtime to v4.1)
    10	- `TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md` § 5.L4 (line 365-389) — ChainTape Layer 4 axioms
    11	
    12	**Single sentence**: implement the L4 transition_ledger module so that `ledger::append(parent_root, tx) → new_root` (called from § 3 transition pseudocode) is real code, the L4 sequencer (§ 5.2.1) is real code, and `Q_t.ledger_root_t` is no longer a placeholder.
    13	
    14	---
    15	
    16	## § 0 Scope
    17	
    18	### In scope
    19	- **LedgerEntry schema**: the canonical envelope wrapping each typed transition (WorkTx / VerifyTx / ChallengeTx / ReuseTx / FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx / SlashTx) before it is appended to L4
    20	- **LedgerRoot computation**: deterministic Merkle accumulation over the entry sequence; this is the value of `Q_t.ledger_root_t`
    21	- **Sequencer**: per-(runtime_repo, run_id) single-writer instance enforcing § 5.2.1 (atomic logical_t, submission-order serialization, post-step_transition commit)
    22	- **append(parent_root, ledger_entry)**: pure function returning the new ledger_root (no I/O at this layer; storage commit is sequencer's job)
    23	- **replay(genesis_root, [ledger_entry])**: deterministic replay producing final state_root; the witness for I-DETHASH
    24	- **Storage backend**: git2-rs commit chain (built on CO1.4 CAS); each LedgerEntry = one git commit on `refs/transitions/main`
    25	
    26	### Out of scope (handled by other atoms)
    27	- WorkTx / VerifyTx / ChallengeTx schemas — frozen in `STATE_TRANSITION_SPEC § 1`
    28	- step_transition / verify_transition / challenge_transition logic — frozen in `STATE_TRANSITION_SPEC § 3`
    29	- system_keypair signing — done @ CO1.7.0a-f
    30	- L5 materializer (state_root computation) — deferred to **CO1.8** (separate atom)
    31	- L6 signal indices — deferred to **CO1.9**
    32	- AttributionEngine DAG — deferred to CO P2.4.0 spike (Inv 8 design)
    33	- MetaTx full schema — v4.1 only; v4 emits `MetaProposalDraft` to L3 CAS, not L4
    34	
    35	### What this spec is NOT replacing
    36	- `src/ledger.rs` (legacy, top-level) is retired in **CO1.1.5 (kernel.rs split)**; CO1.7 lives at `src/bottom_white/ledger/transition_ledger.rs` (NEW). No STEP_B parallel-branch ceremony required (new module, not restricted file).
    37	
    38	---
    39	
    40	## § 1 LedgerEntry schema
    41	
    42	```rust
    43	/// TRACE_MATRIX FC2-Append (FC2 transition machinery):
    44	///   canonical envelope appended to L4 once step_transition succeeds.
    45	///
    46	/// One LedgerEntry per accepted transition, regardless of TxKind.
    47	/// Genesis state has zero LedgerEntries; ledger_root_t = Hash::ZERO.
    48	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    49	pub struct LedgerEntry {
    50	    /// Monotonic counter from sequencer; starts at 1 per genesis.
    51	    /// Sequencer guarantees: distinct entries have distinct logical_t (§ 5.2.6).
    52	    pub logical_t: u64,                          //  1
    53	
    54	    /// Parent state_root before this transition. MUST equal the
    55	    /// resulting_state_root of the entry at logical_t-1 (or Hash::ZERO at logical_t=1).
    56	    pub parent_state_root: Hash,                 //  2
    57	
    58	    /// Discriminator; payload schema depends on this.
    59	    pub tx_kind: TxKind,                         //  3
    60	
    61	    /// CAS handle (CO1.4) to canonical-serialized payload (WorkTx / VerifyTx / ...).
    62	    /// Payload itself is NOT inlined — kept in CO1.4 CAS to bound LedgerEntry size.
    63	    pub tx_payload_cid: Cid,                     //  4
    64	
    65	    /// Resulting state_root after step_transition applied.
    66	    /// Used by I-DETHASH replay test.
    67	    pub resulting_state_root: Hash,              //  5
    68	
    69	    /// Resulting ledger_root after this entry is folded in.
    70	    /// Convention: ledger_root_{t+1} = sha256(ledger_root_t || canonical_digest(LedgerEntry_t))
    71	    pub resulting_ledger_root: Hash,             //  6
    72	
    73	    /// Wall-clock-free timestamp; derived from sequencer logical_t (NOT system time).
    74	    /// Bound to logical_t at sequencer commit; runtime layer does NOT mutate this field.
    75	    pub timestamp_logical: u64,                  //  7
    76	
    77	    /// System runtime keypair signature over canonical_digest of fields 1-7.
    78	    /// Distinct from the agent_signature inside tx_payload (§ 1, agent self-sign).
    79	    /// System signature attests "sequencer accepted this entry at this logical_t".
    80	    pub system_signature: SystemSignature,       //  8
    81	}
    82	
    83	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    84	pub enum TxKind {
    85	    Work,              // WorkTx    (§ 1.2)
    86	    Verify,            // VerifyTx  (§ 1.3)
    87	    Challenge,         // ChallengeTx (§ 1.3)
    88	    Reuse,             // ReuseTx   (§ 1.3)
    89	    FinalizeReward,    // claim window expired clean → reward + stake return (§ 3.4)
    90	    TaskExpire,        // task deadline reached unsolved → bounty refund (§ 3.6)
    91	    TerminalSummary,   // run end without acceptance (§ 1.5 + § 3.7)
    92	    Slash,             // (post-CO P2.5) ChallengeCourt slashing event
    93	}
    94	```
    95	
    96	**Why an envelope (vs. inlining payload)**:
    97	1. **Bounded entry size**: payloads vary widely (12-field WorkTx vs. 6-field ReuseTx). CAS handle keeps LedgerEntry ~200B regardless.
    98	2. **Storage backend reuse**: CO1.4 CAS already provides addressable blob storage; no second blob layer needed.
    99	3. **Replay separation**: replay reads only LedgerEntry chain to validate I-DETHASH; full payload retrieval is on-demand.
   100	
   101	---
   102	
   103	## § 2 Module layout
   104	
   105	```
   106	src/bottom_white/ledger/
   107	├── mod.rs                       (re-exports; existing — extends with `pub mod transition_ledger`)
   108	├── system_keypair.rs            (existing, CO1.7.0a-f, Wave 4-B)
   109	└── transition_ledger.rs         (NEW, this atom)
   110	
   111	src/state/
   112	├── mod.rs                       (existing)
   113	├── q_state.rs                   (existing; ledger_root_t field present at line 317 — CO1.7 fills the placeholder)
   114	└── sequencer.rs                 (NEW, this atom)
   115	```
   116	
   117	**Crate boundary**: `transition_ledger` is in `bottom_white::ledger` because it is a tool layer (storage); `sequencer` is in `state::` because it touches Q_t mutation. Sequencer DEPENDS ON ledger; ledger does NOT depend on sequencer (DAG: state → bottom_white::ledger → CO1.4 CAS).
   118	
   119	---
   120	
   121	## § 3 Sequencer
   122	
   123	```rust
   124	/// TRACE_MATRIX § 5.2.1 — L4 sequencer; single-writer per (runtime_repo, run_id).
   125	pub struct Sequencer {
   126	    /// Atomic monotonic counter (§ 5.2.6 tie-break canonical source).
   127	    next_logical_t: AtomicU64,
   128	
   129	    /// Submission queue; mpsc-style. Submission order = arrival order at the queue head.
   130	    /// Async completion order does NOT matter (§ 5.2.1 step 4).
   131	    queue: SubmissionQueue<TypedTx>,
   132	
   133	    /// Reference to ledger writer (storage backend).
   134	    ledger_writer: Arc<dyn LedgerWriter>,
   135	
   136	    /// Reference to system keypair for entry signing (CO1.7.0a-f).
   137	    keypair: Arc<SystemKeyPair>,
   138	
   139	    /// Reference to predicate + tool registries (read-only at this layer).
   140	    predicate_registry: Arc<PredicateRegistry>,
   141	    tool_registry: Arc<ToolRegistry>,
   142	
   143	    /// Current Q_t snapshot. Held under exclusive write-lock during transition apply.
   144	    q: RwLock<QState>,
   145	}
   146	
   147	impl Sequencer {
   148	    /// External entry point for any agent / runtime caller.
   149	    /// Returns the submitted tx's logical_t + tx_id (deterministic from logical_t, agent_id, payload_hash).
   150	    pub fn submit(&self, tx: TypedTx) -> SubmissionReceipt;
   151	
   152	    /// Driver loop: drain queue, run transition, append entry. Single-threaded internally.
   153	    /// Executor is implementation-detail (tokio task / std thread); spec does NOT mandate.
   154	    pub async fn run(&self) -> Result<(), SequencerError>;
   155	
   156	    /// Per-tx critical section (called by run()):
   157	    fn apply_one(&self, tx: TypedTx) -> Result<LedgerEntry, TransitionError> {
   158	        // 1. Assign logical_t (atomic increment)
   159	        let logical_t = self.next_logical_t.fetch_add(1, Ordering::SeqCst);
   160	
   161	        // 2. Snapshot Q_t under read lock (no mutation yet)
   162	        let q_snapshot = self.q.read().clone();
   163	
   164	        // 3. Dispatch to the correct pure transition function (§ 3, § 3.1, § 3.2, ...)
   165	        let (q_next, signals) = match tx {
   166	            TypedTx::Work(work_tx)        => step_transition(&q_snapshot, &work_tx, &self.predicate_registry, &self.tool_registry)?,
   167	            TypedTx::Verify(verify_tx)    => verify_transition(&q_snapshot, &verify_tx, &self.predicate_registry)?,
   168	            TypedTx::Challenge(chal_tx)   => challenge_transition(&q_snapshot, &chal_tx, &self.predicate_registry)?,
   169	            TypedTx::Reuse(reuse_tx)      => reuse_transition(&q_snapshot, &reuse_tx, &self.tool_registry)?,
   170	            TypedTx::FinalizeReward(_)    => finalize_reward_transition(/* … */)?,
   171	            TypedTx::TaskExpire(_)        => task_expire_transition(/* … */)?,
   172	            TypedTx::TerminalSummary(_)   => emit_terminal_summary(/* … */)?,
   173	        };
   174	
   175	        // 4. Compute ledger_root via append()
   176	        let payload_cid = self.cas.put_canonical(&tx)?;
   177	        let entry = LedgerEntry {
   178	            logical_t,
   179	            parent_state_root: q_snapshot.state_root_t,
   180	            tx_kind: TxKind::from_typed(&tx),
   181	            tx_payload_cid: payload_cid,
   182	            resulting_state_root: q_next.state_root_t,
   183	            resulting_ledger_root: append(&q_snapshot.ledger_root_t, /* unsigned-stub */),
   184	            timestamp_logical: logical_t,
   185	            system_signature: SystemSignature::placeholder(),  // filled in step 5
   186	        };
   187	        let signed_entry = self.keypair.sign_entry(entry);
   188	
   189	        // 5. Acquire write lock; commit to storage; mutate Q_t
   190	        let mut q_w = self.q.write();
   191	        self.ledger_writer.commit(&signed_entry)?;
   192	        *q_w = q_next;
   193	        q_w.ledger_root_t = signed_entry.resulting_ledger_root;
   194	        q_w.head_t = NodeId::from_state_root(q_w.state_root_t);
   195	
   196	        Ok(signed_entry)
   197	    }
   198	}
   199	```
   200	
   201	**Why a single sequencer**: enforces I-DET, I-LOGTIME, I-FINALIZE-BATCH-ORDER, I-FINALIZE-EXCLUSIVE without needing per-transition synchronization. Submission concurrency is handled by the queue; execution concurrency is zero (serial).
   202	
   203	**What § 5.2.7 leaves to implementation**: queue type (mpsc / lock-free / mutex+VecDeque), executor (tokio / std::thread), back-pressure policy. CO1.7 v1 picks tokio mpsc (matches existing kernel runtime). Round-1 audit may push back.
   204	
   205	---
   206	
   207	## § 4 append() + replay()
   208	
   209	```rust
   210	/// Pure. Same (parent_root, entry) → byte-identical new_root.
   211	/// No I/O, no clock, no env.
   212	pub fn append(parent_root: &Hash, entry_digest: &Hash) -> Hash {
   213	    let mut hasher = Sha256::new();
   214	    hasher.update(parent_root.0);
   215	    hasher.update(entry_digest.0);
   216	    Hash::from_bytes(hasher.finalize().into())
   217	}
   218	
   219	/// Replay a sequence of LedgerEntries from genesis. Returns final (state_root, ledger_root).
   220	/// Used by I-DETHASH conformance test + cold-boot recovery.
   221	pub fn replay(
   222	    genesis: &QState,
   223	    entries: &[LedgerEntry],
   224	    payloads: &dyn CasReader,
   225	) -> Result<QState, ReplayError> {
   226	    let mut q = genesis.clone();
   227	    for (i, entry) in entries.iter().enumerate() {
   228	        // 1. Validate parent_state_root chain
   229	        if entry.parent_state_root != q.state_root_t {
   230	            return Err(ReplayError::ParentMismatch { at: i });
   231	        }
   232	
   233	        // 2. Validate logical_t monotonicity
   234	        if entry.logical_t != (i as u64) + 1 {
   235	            return Err(ReplayError::LogicalTGap { at: i, expected: (i as u64) + 1, got: entry.logical_t });
   236	        }
   237	
   238	        // 3. Verify system_signature (rejects forgeries from non-runtime sources)
   239	        if !verify_system_signature(&entry.system_signature, &entry.canonical_digest_unsigned()) {
   240	            return Err(ReplayError::BadSignature { at: i });
   241	        }
   242	
   243	        // 4. Re-fetch payload from CAS, re-run pure transition, compare result
   244	        let payload = payloads.get(&entry.tx_payload_cid)?;
   245	        let typed_tx = TypedTx::deserialize_canonical(&payload)?;
   246	        let (q_next, _) = dispatch_transition(&q, &typed_tx)?;
   247	        if q_next.state_root_t != entry.resulting_state_root {
   248	            return Err(ReplayError::StateRootMismatch { at: i });
   249	        }
   250	
   251	        // 5. Re-fold ledger_root, compare
   252	        let recomputed_ledger_root = append(&q.ledger_root_t, &entry.canonical_digest_unsigned());
   253	        if recomputed_ledger_root != entry.resulting_ledger_root {
   254	            return Err(ReplayError::LedgerRootMismatch { at: i });
   255	        }
   256	
   257	        q = q_next;
   258	        q.ledger_root_t = entry.resulting_ledger_root;
   259	    }
   260	    Ok(q)

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! L4 Transition Ledger (CO1.7) — type skeleton + pure helpers.
     2	//!
     3	//! TRACE_MATRIX FC2-Append: canonical envelope appended to L4 once a transition is accepted.
     4	//! TRACE_MATRIX WP § 5.L4: ChainTape Layer 4 spine; one LedgerEntry per accepted transition.
     5	//! TRACE_MATRIX § 1 (CO1_7_TRANSITION_LEDGER_v1_2026-04-28): schema + append() + replay() pseudocode.
     6	//!
     7	//! **Status**: type skeleton ahead of round-1 dual external audit.
     8	//! All bodies that depend on yet-to-implement transition functions are
     9	//! `unimplemented!()` and gated by `cfg(any())` to keep `cargo check` clean.
    10	//!
    11	//! **Spec ↔ code divergences flagged for round-1 audit** (NOT silently absorbed):
    12	//! - **DIV-1** `LedgerEntry` cannot ride existing `CanonicalMessage` enum
    13	//!   (3 variants: RejectedAttemptSummary / TerminalSummaryTx / EpochRotationProof).
    14	//!   Either extend the enum OR introduce a sibling sign primitive. v1 spec
    15	//!   chose neither — round-1 audit Q8 (NEW).
    16	//! - **DIV-2** `Sequencer` integration with `Q_t.economic_state_t.balances_t`
    17	//!   etc. requires those indices to expose mutation API; current `q_state.rs`
    18	//!   only stores the BTreeMap shells. Mutation API arrives at CO P2.x economy
    19	//!   atoms; skeleton uses `unimplemented!()` for state mutation paths.
    20	//! - **DIV-3** Spec § 1 missed `epoch: SystemEpoch` field. Added here. Without
    21	//!   it, signature verification cannot resolve which pinned pubkey to use.
    22	//! - **DIV-4** Spec § 4 used a `CasReader` trait; actual code uses concrete
    23	//!   `CasStore` struct. Skeleton uses a narrower trait `LedgerCasView` that
    24	//!   `CasStore` will impl in CO1.7.5+; keeps test seams open.
    25	//! - **DIV-5** Spec § 1 `tx_payload_cid: Cid`. CAS `put` requires
    26	//!   `(content, object_type, creator, created_at_logical_t, schema_id)` —
    27	//!   five fields, not just bytes. Sequencer must build full metadata when
    28	//!   storing; skeleton documents this requirement.
    29	
    30	use sha2::{Digest, Sha256};
    31	
    32	use crate::bottom_white::cas::schema::Cid;
    33	use crate::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
    34	use crate::state::q_state::Hash;
    35	
    36	// ────────────────────────────────────────────────────────────────────────────
    37	// § 1 LedgerEntry schema (skeleton; round-1-audit-pending)
    38	// ────────────────────────────────────────────────────────────────────────────
    39	
    40	/// TRACE_MATRIX FC2-Append: discriminator for the typed payload behind a CAS Cid.
    41	/// **Skeleton note**: serde derives deferred — bincode v2 canonical shape is
    42	/// round-1 audit Q5 / spec § 2.5; premature derive would lock the shape.
    43	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    44	pub enum TxKind {
    45	    /// WorkTx (STATE_TRANSITION_SPEC § 1.2; 12 fields).
    46	    Work,
    47	    /// VerifyTx (§ 1.3).
    48	    Verify,
    49	    /// ChallengeTx (§ 1.3).
    50	    Challenge,
    51	    /// ReuseTx (§ 1.3).
    52	    Reuse,
    53	    /// Claim window expired clean → reward + stake return (§ 3.4).
    54	    FinalizeReward,
    55	    /// Task deadline reached unsolved → bounty refund (§ 3.6).
    56	    TaskExpire,
    57	    /// Run end without acceptance (§ 1.5 + § 3.7).
    58	    TerminalSummary,
    59	    /// Post-CO P2.5 ChallengeCourt slashing event.
    60	    Slash,
    61	}
    62	
    63	/// TRACE_MATRIX FC2-Append + WP § 5.L4 (12-field WorkTx envelope wrapper):
    64	/// canonical envelope stamped by the L4 sequencer once `dispatch_transition` succeeds.
    65	///
    66	/// **DIV-3** vs spec v1: added `epoch: SystemEpoch` to bind signature verification
    67	/// to a specific pinned pubkey (per `system_keypair::verify_system_signature`).
    68	///
    69	/// **Skeleton note**: serde derives deferred per round-1 audit Q5 (canonical shape
    70	/// for cross-impl byte parity is a real spec choice, not a default).
    71	#[derive(Debug, Clone, PartialEq, Eq)]
    72	pub struct LedgerEntry {
    73	    /// Monotonic counter from sequencer; starts at 1 per genesis.
    74	    pub logical_t: u64,
    75	
    76	    /// Parent state_root before this transition. Equals `prev.resulting_state_root`
    77	    /// (or `Hash::ZERO` at logical_t=1).
    78	    pub parent_state_root: Hash,
    79	
    80	    /// Discriminator; payload schema depends on this.
    81	    pub tx_kind: TxKind,
    82	
    83	    /// CAS handle (CO1.4) to canonically-serialized payload.
    84	    /// Sequencer is responsible for building full CAS metadata
    85	    /// (object_type / creator / created_at_logical_t / schema_id) per **DIV-5**.
    86	    pub tx_payload_cid: Cid,
    87	
    88	    /// Resulting state_root after `dispatch_transition` applied.
    89	    /// Used by I-DETHASH replay test.
    90	    pub resulting_state_root: Hash,
    91	
    92	    /// Resulting ledger_root after this entry is folded in.
    93	    /// Convention: `ledger_root_{t+1} = sha256(ledger_root_t || canonical_digest_unsigned(LedgerEntry_t))`
    94	    pub resulting_ledger_root: Hash,
    95	
    96	    /// Wall-clock-free timestamp (= `logical_t`).
    97	    /// Runtime layer does NOT mutate this field after sequencer commit.
    98	    pub timestamp_logical: u64,
    99	
   100	    /// **DIV-3**: which pinned epoch pubkey signed this entry.
   101	    pub epoch: SystemEpoch,
   102	
   103	    /// System runtime keypair signature over `canonical_digest_unsigned`.
   104	    /// Distinct from the `agent_signature` inside payload (agent self-sign).
   105	    /// System signature attests "sequencer accepted this entry at this logical_t".
   106	    ///
   107	    /// **DIV-1**: how this is computed is round-1 audit Q8. Skeleton stores it
   108	    /// but does not yet derive it through `CanonicalMessage` (which currently has
   109	    /// 3 fixed variants and does NOT include LedgerEntry).
   110	    pub system_signature: SystemSignature,
   111	}
   112	
   113	impl LedgerEntry {
   114	    /// TRACE_MATRIX FC2-Append: canonical digest of the 7 fields the system
   115	    /// signature attests. **Excludes** `resulting_ledger_root` AND `system_signature`:
   116	    /// - `system_signature` (8) is excluded because the digest is its input.
   117	    /// - `resulting_ledger_root` (6) is excluded because it is *derived* via
   118	    ///   `append(prev_ledger_root, digest)` — including it would create a
   119	    ///   circular dependency (ledger_root ⊃ digest ⊃ ledger_root).
   120	    ///
   121	    /// **Spec finding**: this exclusion was NOT explicit in spec v1 § 1.
   122	    /// Skeleton smoke caught the cycle immediately on first replay-test run.
   123	    /// To be sedimented into spec v1.1 round-1 audit Q9 (NEW).
   124	    ///
   125	    /// **DIV-1**: this digest is what the system_signature must sign once the
   126	    /// `CanonicalMessage` integration question is resolved at round-1.
   127	    pub fn canonical_digest_unsigned(&self) -> Hash {
   128	        let mut h = Sha256::new();
   129	        h.update(b"turingosv4.ledger_entry.v1");
   130	        h.update(self.logical_t.to_be_bytes());
   131	        h.update(self.parent_state_root.0);
   132	        h.update((self.tx_kind as u8).to_be_bytes());
   133	        h.update(self.tx_payload_cid.0);
   134	        h.update(self.resulting_state_root.0);
   135	        // EXCLUDED: self.resulting_ledger_root.0 — derivative of this digest.
   136	        h.update(self.timestamp_logical.to_be_bytes());
   137	        h.update(self.epoch.get().to_be_bytes());
   138	        Hash(h.finalize().into())
   139	    }
   140	}
   141	
   142	// ────────────────────────────────────────────────────────────────────────────
   143	// § 4 append() — pure ledger-root fold
   144	// ────────────────────────────────────────────────────────────────────────────
   145	
   146	/// TRACE_MATRIX FC2-Append + spec § 4: pure ledger-root fold.
   147	/// Same `(parent_root, entry_digest)` → byte-identical `new_root`.
   148	/// No I/O, no clock, no env. Witness for I-DET / I-DETHASH ledger axis.
   149	pub fn append(parent_root: &Hash, entry_digest: &Hash) -> Hash {
   150	    let mut h = Sha256::new();
   151	    h.update(b"turingosv4.ledger_root.v1");
   152	    h.update(parent_root.0);
   153	    h.update(entry_digest.0);
   154	    Hash(h.finalize().into())
   155	}
   156	
   157	// ────────────────────────────────────────────────────────────────────────────
   158	// LedgerWriter trait + in-memory test impl
   159	// ────────────────────────────────────────────────────────────────────────────
   160	
   161	/// TRACE_MATRIX FC2-Append: storage abstraction for L4.
   162	/// Production impl is `Git2LedgerWriter` (CO1.7.5; refs/transitions/main commit chain).
   163	/// Test/skeleton impl is `InMemoryLedgerWriter` below.
   164	pub trait LedgerWriter: Send + Sync {
   165	    /// Commit a signed entry. Atomic: either the entry lands at the next logical_t
   166	    /// or no state change. Returns the entry's `resulting_ledger_root` on success.
   167	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
   168	
   169	    /// Read the entry at a specific 1-indexed `logical_t`.
   170	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
   171	
   172	    /// Total number of accepted entries (highest assigned `logical_t`; 0 at genesis).
   173	    fn len(&self) -> u64;
   174	}
   175	
   176	/// TRACE_MATRIX FC2-Append: error surface for storage layer.
   177	#[derive(Debug)]
   178	pub enum LedgerWriterError {
   179	    LogicalTGap { expected: u64, got: u64 },
   180	    NotFound { logical_t: u64 },
   181	    BackendCorruption(String),
   182	}
   183	
   184	impl std::fmt::Display for LedgerWriterError {
   185	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   186	        match self {
   187	            Self::LogicalTGap { expected, got } => {
   188	                write!(f, "logical_t gap: expected {expected}, got {got}")
   189	            }
   190	            Self::NotFound { logical_t } => write!(f, "no entry at logical_t={logical_t}"),
   191	            Self::BackendCorruption(msg) => write!(f, "backend corruption: {msg}"),
   192	        }
   193	    }
   194	}
   195	impl std::error::Error for LedgerWriterError {}
   196	
   197	/// TRACE_MATRIX FC2-Append: in-memory test/skeleton writer.
   198	/// Vec backing → strict logical_t ordering enforced at commit.
   199	#[derive(Debug, Default)]
   200	pub struct InMemoryLedgerWriter {
   201	    entries: Vec<LedgerEntry>,
   202	}
   203	
   204	impl InMemoryLedgerWriter {
   205	    pub fn new() -> Self {
   206	        Self::default()
   207	    }
   208	}
   209	
   210	impl LedgerWriter for InMemoryLedgerWriter {
   211	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
   212	        let expected = (self.entries.len() as u64) + 1;
   213	        if entry.logical_t != expected {
   214	            return Err(LedgerWriterError::LogicalTGap {
   215	                expected,
   216	                got: entry.logical_t,
   217	            });
   218	        }
   219	        let root = entry.resulting_ledger_root;
   220	        self.entries.push(entry.clone());
   221	        Ok(root)
   222	    }
   223	
   224	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
   225	        if logical_t == 0 || logical_t > self.entries.len() as u64 {
   226	            return Err(LedgerWriterError::NotFound { logical_t });
   227	        }
   228	        Ok(self.entries[(logical_t - 1) as usize].clone())
   229	    }
   230	
   231	    fn len(&self) -> u64 {
   232	        self.entries.len() as u64
   233	    }
   234	}
   235	
   236	// ────────────────────────────────────────────────────────────────────────────
   237	// § 4 replay() — chain-integrity skeleton (full transition dispatch deferred)
   238	// ────────────────────────────────────────────────────────────────────────────
   239	
   240	/// TRACE_MATRIX I-DETHASH (chain-integrity axis only at this iteration).
   241	/// **Skeleton**: validates parent_state_root + ledger_root chain only.
   242	/// Full transition dispatch (re-running each tx pure-function-side) lands when
   243	/// CO1.7.5+ implements the actual transition function bodies.
   244	#[derive(Debug)]
   245	pub enum ReplayError {
   246	    LogicalTGap { at: usize, expected: u64, got: u64 },
   247	    ParentMismatch { at: usize },
   248	    LedgerRootMismatch { at: usize },
   249	}
   250	
   251	impl std::fmt::Display for ReplayError {
   252	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   253	        match self {
   254	            Self::LogicalTGap { at, expected, got } => {
   255	                write!(f, "logical_t gap at index {at}: expected {expected}, got {got}")
   256	            }
   257	            Self::ParentMismatch { at } => write!(f, "parent_state_root mismatch at index {at}"),
   258	            Self::LedgerRootMismatch { at } => write!(f, "ledger_root mismatch at index {at}"),
   259	        }
   260	    }

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '1,280p'" in /home/zephryj/projects/turingosv4
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
    17	use sha2::{Digest, Sha256};
    18	use std::collections::BTreeMap;
    19	use std::env;
    20	use std::fmt;
    21	use std::fs::{self, OpenOptions};
    22	use std::io::Write;
    23	use std::path::{Path, PathBuf};
    24	use zeroize::{Zeroize, ZeroizeOnDrop};
    25	
    26	const DEFAULT_KDF_MEMORY_KIB: u32 = 65_536;
    27	const DEFAULT_KDF_ITER: u32 = 3;
    28	const DEFAULT_KDF_LANES: u32 = 4;
    29	const DERIVED_KEY_LEN: usize = 32;
    30	const SECRET_KEY_LEN: usize = 32;
    31	const PUBLIC_KEY_LEN: usize = 32;
    32	const SIGNATURE_LEN: usize = 64;
    33	const SALT_LEN: usize = 16;
    34	const NONCE_LEN: usize = 12;
    35	const PLAINTEXT_LEN: usize = SECRET_KEY_LEN + PUBLIC_KEY_LEN;
    36	const FORMAT_MAGIC: &[u8; 11] = b"TOS4SYSKEY1";
    37	const FORMAT_VERSION: u8 = 1;
    38	
    39	/// TRACE_MATRIX FC1-Sig+FC3-Sig: system signature epoch identifier.
    40	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    41	pub struct SystemEpoch(u64);
    42	
    43	impl SystemEpoch {
    44	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a system signature epoch.
    45	    pub const fn new(epoch: u64) -> Self {
    46	        Self(epoch)
    47	    }
    48	
    49	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: expose the numeric epoch for canonical encoding.
    50	    pub const fn get(self) -> u64 {
    51	        self.0
    52	    }
    53	}
    54	
    55	impl From<u64> for SystemEpoch {
    56	    fn from(value: u64) -> Self {
    57	        Self(value)
    58	    }
    59	}
    60	
    61	/// TRACE_MATRIX FC1-Sig+FC3-Sig: ed25519 public key pinned by epoch.
    62	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    63	pub struct SystemPublicKey([u8; PUBLIC_KEY_LEN]);
    64	
    65	impl SystemPublicKey {
    66	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a system public key from raw ed25519 bytes.
    67	    pub const fn from_bytes(bytes: [u8; PUBLIC_KEY_LEN]) -> Self {
    68	        Self(bytes)
    69	    }
    70	
    71	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: expose raw public key bytes for pinning and verification.
    72	    pub const fn as_bytes(&self) -> &[u8; PUBLIC_KEY_LEN] {
    73	        &self.0
    74	    }
    75	
    76	    /// TRACE_MATRIX FC3-Sig: stable SHA-256 fingerprint for audit logs and rotation records.
    77	    pub fn fingerprint_sha256(&self) -> [u8; 32] {
    78	        Sha256::digest(self.0).into()
    79	    }
    80	}
    81	
    82	/// TRACE_MATRIX FC1-Sig+FC3-Sig: ed25519 detached signature over a canonical system message digest.
    83	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
    84	pub struct SystemSignature([u8; SIGNATURE_LEN]);
    85	
    86	impl SystemSignature {
    87	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a detached system signature from raw ed25519 bytes.
    88	    pub const fn from_bytes(bytes: [u8; SIGNATURE_LEN]) -> Self {
    89	        Self(bytes)
    90	    }
    91	
    92	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: expose raw signature bytes for tape serialization.
    93	    pub const fn as_bytes(&self) -> &[u8; SIGNATURE_LEN] {
    94	        &self.0
    95	    }
    96	}
    97	
    98	/// TRACE_MATRIX FC1-Sig: typed rejection summary stamped by the predicate runner.
    99	#[derive(Debug, Clone, PartialEq, Eq)]
   100	pub struct RejectedAttemptSummary {
   101	    run_id: String,
   102	    attempt_id: String,
   103	    failure_class: String,
   104	    summary_hash: [u8; 32],
   105	}
   106	
   107	impl RejectedAttemptSummary {
   108	    /// TRACE_MATRIX FC1-Sig: construct a typed rejected-attempt summary, never a free-form sign blob.
   109	    pub fn new(
   110	        run_id: impl Into<String>,
   111	        attempt_id: impl Into<String>,
   112	        failure_class: impl Into<String>,
   113	        summary_hash: [u8; 32],
   114	    ) -> Self {
   115	        Self {
   116	            run_id: run_id.into(),
   117	            attempt_id: attempt_id.into(),
   118	            failure_class: failure_class.into(),
   119	            summary_hash,
   120	        }
   121	    }
   122	}
   123	
   124	/// TRACE_MATRIX FC1-Sig+FC3-Sig: typed terminal summary transaction emitted on no-accept runs.
   125	#[derive(Debug, Clone, PartialEq, Eq)]
   126	pub struct TerminalSummaryTx {
   127	    run_id: String,
   128	    terminal_state_root: [u8; 32],
   129	    rejected_attempt_count: u64,
   130	}
   131	
   132	impl TerminalSummaryTx {
   133	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a typed terminal summary transaction.
   134	    pub fn new(
   135	        run_id: impl Into<String>,
   136	        terminal_state_root: [u8; 32],
   137	        rejected_attempt_count: u64,
   138	    ) -> Self {
   139	        Self {
   140	            run_id: run_id.into(),
   141	            terminal_state_root,
   142	            rejected_attempt_count,
   143	        }
   144	    }
   145	}
   146	
   147	/// TRACE_MATRIX FC3-Sig: typed continuity statement for system key rotation.
   148	#[derive(Debug, Clone, PartialEq, Eq)]
   149	pub struct EpochRotationProof {
   150	    old_epoch: SystemEpoch,
   151	    new_epoch: SystemEpoch,
   152	    old_pubkey: SystemPublicKey,
   153	    new_pubkey: SystemPublicKey,
   154	    signed_at_unix: u64,
   155	}
   156	
   157	impl EpochRotationProof {
   158	    /// TRACE_MATRIX FC3-Sig: construct a typed epoch-rotation continuity proof.
   159	    pub const fn new(
   160	        old_epoch: SystemEpoch,
   161	        new_epoch: SystemEpoch,
   162	        old_pubkey: SystemPublicKey,
   163	        new_pubkey: SystemPublicKey,
   164	        signed_at_unix: u64,
   165	    ) -> Self {
   166	        Self {
   167	            old_epoch,
   168	            new_epoch,
   169	            old_pubkey,
   170	            new_pubkey,
   171	            signed_at_unix,
   172	        }
   173	    }
   174	
   175	    /// TRACE_MATRIX FC3-Sig: old signing epoch certified by the rotation proof.
   176	    pub const fn old_epoch(&self) -> SystemEpoch {
   177	        self.old_epoch
   178	    }
   179	
   180	    /// TRACE_MATRIX FC3-Sig: new signing epoch certified by the rotation proof.
   181	    pub const fn new_epoch(&self) -> SystemEpoch {
   182	        self.new_epoch
   183	    }
   184	}
   185	
   186	/// TRACE_MATRIX FC1-Sig+FC3-Sig: only typed runtime messages may enter signature verification.
   187	#[derive(Debug, Clone, PartialEq, Eq)]
   188	pub enum CanonicalMessage {
   189	    /// TRACE_MATRIX FC1-Sig: predicate-runner rejection summary.
   190	    RejectedAttemptSummary(RejectedAttemptSummary),
   191	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: terminal summary transaction.
   192	    TerminalSummaryTx(TerminalSummaryTx),
   193	    /// TRACE_MATRIX FC3-Sig: system key epoch continuity proof.
   194	    EpochRotationProof(EpochRotationProof),
   195	}
   196	
   197	/// TRACE_MATRIX FC1-Sig+FC3-Sig: epoch-indexed public keys pinned by genesis and rotation history.
   198	#[derive(Debug, Clone, Default, PartialEq, Eq)]
   199	pub struct PinnedSystemPubkeys {
   200	    keys: BTreeMap<SystemEpoch, SystemPublicKey>,
   201	}
   202	
   203	impl PinnedSystemPubkeys {
   204	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: create an empty pinned system-key map.
   205	    pub fn new() -> Self {
   206	        Self::default()
   207	    }
   208	
   209	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: pin a public key for a system epoch.
   210	    pub fn insert(
   211	        &mut self,
   212	        epoch: SystemEpoch,
   213	        public_key: SystemPublicKey,
   214	    ) -> Option<SystemPublicKey> {
   215	        self.keys.insert(epoch, public_key)
   216	    }
   217	
   218	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: fetch the public key pinned for a system epoch.
   219	    pub fn get(&self, epoch: SystemEpoch) -> Option<&SystemPublicKey> {
   220	        self.keys.get(&epoch)
   221	    }
   222	}
   223	
   224	/// TRACE_MATRIX FC1-Sig+FC3-Sig: in-memory ed25519 system keypair with zeroized private key on drop.
   225	#[derive(Zeroize, ZeroizeOnDrop)]
   226	pub struct Ed25519Keypair {
   227	    secret_key: Box<[u8]>,
   228	    #[zeroize(skip)]
   229	    public_key: SystemPublicKey,
   230	}
   231	
   232	impl Ed25519Keypair {
   233	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: generate ed25519 key material from `getrandom(2)` entropy.
   234	    pub fn generate_with_secure_entropy() -> Result<Self, KeypairError> {
   235	        let mut seed = [0u8; SECRET_KEY_LEN];
   236	        getrandom::getrandom(&mut seed).map_err(KeypairError::Entropy)?;
   237	        let signing_key = SigningKey::from_bytes(&seed);
   238	        let public_key = SystemPublicKey::from_bytes(signing_key.verifying_key().to_bytes());
   239	        let mut keypair = Self {
   240	            secret_key: Vec::from(seed).into_boxed_slice(),
   241	            public_key,
   242	        };
   243	        seed.zeroize();
   244	        keypair.mlock_private_key_best_effort();
   245	        Ok(keypair)
   246	    }
   247	
   248	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: return the public half of the system keypair.
   249	    pub const fn public_key(&self) -> SystemPublicKey {
   250	        self.public_key
   251	    }
   252	
   253	    fn from_plaintext(plaintext: &[u8]) -> Result<Self, KeypairError> {
   254	        if plaintext.len() != PLAINTEXT_LEN {
   255	            return Err(KeypairError::InvalidFormat("bad plaintext length"));
   256	        }
   257	        let mut secret = [0u8; SECRET_KEY_LEN];
   258	        secret.copy_from_slice(&plaintext[..SECRET_KEY_LEN]);
   259	        let mut public = [0u8; PUBLIC_KEY_LEN];
   260	        public.copy_from_slice(&plaintext[SECRET_KEY_LEN..]);
   261	
   262	        let signing_key = SigningKey::from_bytes(&secret);
   263	        if signing_key.verifying_key().to_bytes() != public {
   264	            secret.zeroize();
   265	            return Err(KeypairError::InvalidFormat(
   266	                "public key does not match private key",
   267	            ));
   268	        }
   269	
   270	        let mut keypair = Self {
   271	            secret_key: Vec::from(secret).into_boxed_slice(),
   272	            public_key: SystemPublicKey::from_bytes(public),
   273	        };
   274	        secret.zeroize();
   275	        keypair.mlock_private_key_best_effort();
   276	        Ok(keypair)
   277	    }
   278	
   279	    fn to_plaintext(&self) -> Result<[u8; PLAINTEXT_LEN], KeypairError> {
   280	        let secret = self.secret_slice()?;

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/store.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! CAS store backed by git2-rs blob layer.
     2	//!
     3	//! Each runtime_repo (per spec § 5.2.2 cell isolation) has its own CasStore.
     4	//! Objects are content-addressed by `Cid` (sha256 of content); git's sha-1
     5	//! OID is recorded but not canonical.
     6	//!
     7	//! /// TRACE_MATRIX WP-arch-§5.L3 + spec-§5.2.2 (cell isolation): CAS store
     8	
     9	use git2::{ObjectType as Git2ObjectType, Repository};
    10	use sha2::{Digest, Sha256};
    11	use std::collections::BTreeMap;
    12	use std::path::{Path, PathBuf};
    13	
    14	use super::schema::{CasObjectMetadata, Cid, ObjectType};
    15	
    16	#[derive(Debug)]
    17	pub enum CasError {
    18	    /// git2-rs underlying error.
    19	    Git2(git2::Error),
    20	    /// Cid not found in this CasStore's metadata index.
    21	    CidNotFound(Cid),
    22	    /// Content stored at git OID but Cid metadata absent (corrupted index).
    23	    MetadataMissing(Cid),
    24	    /// Content's sha256 doesn't match the asserted Cid (corruption).
    25	    CidMismatch { expected: Cid, computed: Cid },
    26	}
    27	
    28	impl std::fmt::Display for CasError {
    29	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    30	        match self {
    31	            Self::Git2(e) => write!(f, "git2 backend error: {e}"),
    32	            Self::CidNotFound(c) => write!(f, "{c} not found in CAS index"),
    33	            Self::MetadataMissing(c) => write!(f, "{c} metadata missing (index corrupted)"),
    34	            Self::CidMismatch { expected, computed } => write!(
    35	                f,
    36	                "CAS content corruption: expected {expected}, computed {computed}"
    37	            ),
    38	        }
    39	    }
    40	}
    41	
    42	impl std::error::Error for CasError {}
    43	
    44	impl From<git2::Error> for CasError {
    45	    fn from(e: git2::Error) -> Self {
    46	        Self::Git2(e)
    47	    }
    48	}
    49	
    50	/// Content-addressable store backed by git's blob object database.
    51	pub struct CasStore {
    52	    repo_path: PathBuf,
    53	    /// Cid → metadata index. BTreeMap per spec § 2 I-BTREE.
    54	    index: BTreeMap<Cid, CasObjectMetadata>,
    55	}
    56	
    57	impl CasStore {
    58	    /// Open or initialize a CAS store at the given runtime_repo path.
    59	    /// Creates the git repo if it doesn't exist.
    60	    pub fn open(repo_path: &Path) -> Result<Self, CasError> {
    61	        let repo_path = repo_path.to_path_buf();
    62	        // Init or open
    63	        let _repo = match Repository::open(&repo_path) {
    64	            Ok(r) => r,
    65	            Err(_) => Repository::init(&repo_path)?,
    66	        };
    67	        Ok(Self {
    68	            repo_path,
    69	            index: BTreeMap::new(),
    70	        })
    71	    }
    72	
    73	    fn open_repo(&self) -> Result<Repository, CasError> {
    74	        Repository::open(&self.repo_path).map_err(CasError::from)
    75	    }
    76	
    77	    /// Store content; returns its Cid. Idempotent — same content → same Cid.
    78	    pub fn put(
    79	        &mut self,
    80	        content: &[u8],
    81	        object_type: ObjectType,
    82	        creator: &str,
    83	        created_at_logical_t: u64,
    84	        schema_id: Option<String>,
    85	    ) -> Result<Cid, CasError> {
    86	        let cid = Cid::from_content(content);
    87	        let repo = self.open_repo()?;
    88	        let git_oid = repo.blob(content)?;
    89	
    90	        // If already in index, idempotent: just return Cid (content addressing
    91	        // guarantees same content → same Cid → already present)
    92	        if self.index.contains_key(&cid) {
    93	            return Ok(cid);
    94	        }
    95	
    96	        let metadata = CasObjectMetadata {
    97	            cid,
    98	            backend_oid_hex: git_oid.to_string(),
    99	            object_type,
   100	            creator: creator.to_string(),
   101	            created_at_logical_t,
   102	            schema_id,
   103	            size_bytes: content.len() as u64,
   104	        };
   105	        self.index.insert(cid, metadata);
   106	        Ok(cid)
   107	    }
   108	
   109	    /// Retrieve content by Cid. Verifies content sha256 matches Cid (corruption check).
   110	    pub fn get(&self, cid: &Cid) -> Result<Vec<u8>, CasError> {
   111	        let metadata = self
   112	            .index
   113	            .get(cid)
   114	            .ok_or(CasError::CidNotFound(*cid))?;
   115	        let repo = self.open_repo()?;
   116	        let git_oid = git2::Oid::from_str(&metadata.backend_oid_hex)
   117	            .map_err(CasError::Git2)?;
   118	        let blob = repo.find_blob(git_oid)?;
   119	        let content = blob.content().to_vec();
   120	
   121	        // Verify content sha256 matches Cid (defense against corruption).
   122	        let mut h = Sha256::new();
   123	        h.update(&content);
   124	        let computed = Cid(h.finalize().into());
   125	        if &computed != cid {
   126	            return Err(CasError::CidMismatch {
   127	                expected: *cid,
   128	                computed,
   129	            });
   130	        }
   131	
   132	        Ok(content)
   133	    }
   134	
   135	    /// Get metadata only (no content fetch).
   136	    pub fn metadata(&self, cid: &Cid) -> Option<&CasObjectMetadata> {
   137	        self.index.get(cid)
   138	    }
   139	
   140	    pub fn len(&self) -> usize {
   141	        self.index.len()
   142	    }
   143	
   144	    pub fn is_empty(&self) -> bool {
   145	        self.index.is_empty()
   146	    }
   147	
   148	    /// Merkle root over all CAS object metadata; deterministic per BTreeMap order.
   149	    pub fn merkle_root(&self) -> [u8; 32] {
   150	        let mut h = Sha256::new();
   151	        for (_cid, meta) in &self.index {
   152	            h.update(meta.canonical_hash());
   153	        }
   154	        h.finalize().into()
   155	    }
   156	}
   157	
   158	#[cfg(test)]
   159	mod tests {
   160	    use super::*;
   161	    use tempfile::TempDir;
   162	
   163	    fn fresh_store() -> (TempDir, CasStore) {
   164	        let tmp = TempDir::new().unwrap();
   165	        let store = CasStore::open(tmp.path()).unwrap();
   166	        (tmp, store)
   167	    }
   168	
   169	    #[test]
   170	    fn put_get_round_trip_small() {
   171	        let (_tmp, mut s) = fresh_store();
   172	        let cid = s.put(b"hello world", ObjectType::ProposalPayload, "alice", 100, None).unwrap();
   173	        let content = s.get(&cid).unwrap();
   174	        assert_eq!(content, b"hello world");
   175	    }
   176	
   177	    #[test]
   178	    fn put_get_round_trip_large() {
   179	        let (_tmp, mut s) = fresh_store();
   180	        let big = vec![0xab; 65536];
   181	        let cid = s.put(&big, ObjectType::PredicateBytecode, "system", 0, Some("wasm".into())).unwrap();
   182	        let content = s.get(&cid).unwrap();
   183	        assert_eq!(content, big);
   184	    }
   185	
   186	    #[test]
   187	    fn put_idempotent_same_content() {
   188	        let (_tmp, mut s) = fresh_store();
   189	        let cid_a = s.put(b"x", ObjectType::Generic, "alice", 1, None).unwrap();
   190	        let cid_b = s.put(b"x", ObjectType::Generic, "bob", 2, None).unwrap();
   191	        assert_eq!(cid_a, cid_b, "same content → same Cid");
   192	        // Index size = 1 (idempotent)
   193	        assert_eq!(s.len(), 1);
   194	    }
   195	
   196	    #[test]
   197	    fn cid_is_content_address() {
   198	        let (_tmp, mut s) = fresh_store();
   199	        let cid = s.put(b"specific content", ObjectType::Generic, "system", 0, None).unwrap();
   200	        // Cid is sha256 of content; verifiable independently
   201	        let expected = Cid::from_content(b"specific content");
   202	        assert_eq!(cid, expected);
   203	    }
   204	
   205	    #[test]
   206	    fn get_nonexistent_returns_error() {
   207	        let (_tmp, s) = fresh_store();
   208	        let bogus = Cid([0u8; 32]);
   209	        match s.get(&bogus) {
   210	            Err(CasError::CidNotFound(c)) => assert_eq!(c, bogus),
   211	            other => panic!("expected CidNotFound, got {other:?}"),
   212	        }
   213	    }
   214	
   215	    #[test]
   216	    fn metadata_recorded() {
   217	        let (_tmp, mut s) = fresh_store();
   218	        let cid = s.put(b"meta test", ObjectType::CounterexamplePayload, "carol", 250, Some("v1".into())).unwrap();
   219	        let meta = s.metadata(&cid).unwrap();
   220	        assert_eq!(meta.cid, cid);

exec
/bin/bash -lc "nl -ba handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md | sed -n '260,520p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   260	    Ok(q)
   261	}
   262	```
   263	
   264	**Replay is the I-DETHASH witness**: any cold restart MUST be able to call `replay(genesis, ledger_entries, cas) → q` and get the same state_root the live system has. If it diverges, either (a) the spec was implemented non-deterministically, or (b) the ledger was tampered with — both are I-DETHASH violations.
   265	
   266	---
   267	
   268	## § 5 Storage backend
   269	
   270	**Choice**: git2-rs commit chain (Path B substrate, ratified per Const Art 0.4 + WP § 5.L4).
   271	
   272	**Mapping**:
   273	- One `LedgerEntry` = one git commit on `refs/transitions/main`
   274	- Commit message = canonical-serialized `LedgerEntry` (bincode v2)
   275	- Commit tree = `(payload_cid_blob, state_root_marker, signature_blob)`
   276	- `head_t = NodeId(commit_sha)` (Q_t § 1.1 line 47-49 already implements `NodeId::from_state_root`)
   277	- Genesis: `refs/transitions/main` is created at the empty-tree commit corresponding to `genesis_payload.toml` (CO1.0)
   278	
   279	**LedgerWriter trait**:
   280	
   281	```rust
   282	pub trait LedgerWriter: Send + Sync {
   283	    /// Commit a signed LedgerEntry to storage. Atomic: either commit + ref update both succeed,
   284	    /// or neither does (git2-rs txn semantics).
   285	    fn commit(&self, entry: &LedgerEntry) -> Result<NodeId, LedgerWriterError>;
   286	
   287	    /// Read entry at a specific logical_t (1-indexed).
   288	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
   289	
   290	    /// Iterate entries in logical_t order from `from` (inclusive).
   291	    fn iter_from(&self, from: u64) -> Box<dyn Iterator<Item = Result<LedgerEntry, LedgerWriterError>> + '_>;
   292	}
   293	```
   294	
   295	**Implementation**: `Git2LedgerWriter` (built on existing CO1.4 `git2-rs` CAS layer). Uses `repo.commit(...)` with parents = [previous head]. Ref update via `repo.reference("refs/transitions/main", new_oid, force=false, log_msg)`.
   296	
   297	**Why git2-rs not gix**: Const Art 0.4 ratified path B (gix→git2-rs pivot per CO1.3.1 spike 8/8 PASS).
   298	
   299	---
   300	
   301	## § 6 Invariants enforced by CO1.7
   302	
   303	| ID | Invariant | Enforced where in CO1.7 |
   304	|---|---|---|
   305	| **I-DET** | Same (Q_t, tx) → byte-identical (Q_{t+1}, signals) | sequencer.apply_one stages 3-4 (pure step_transition + deterministic append) |
   306	| **I-DETHASH** | replay(genesis, ledger_entries) recovers live state_root | replay() + conformance `tests/q_state_reconstruct.rs` |
   307	| **I-LOGTIME** | timestamp_logical strictly monotonic; no wall clock | sequencer.apply_one stage 1 (atomic fetch_add); LedgerEntry has no wall-clock field |
   308	| **I-FINALIZE-BATCH-ORDER** | When N claims expire same logical_t, finalize order = `(expires_at_logical ASC, claim_id ASC)` | sequencer enqueues finalize tx in this order before resuming work tx; per § 5.2.3 |
   309	| **I-FINALIZE-EXCLUSIVE** | finalize_reward_tx and slash_tx mutually exclusive per claim | sequencer's serial dispatch (no concurrent finalize possible) |
   310	| **I-NOSIDE** | step_transition reads only (q, tx, registries) | append() and replay() are pure; sequencer.apply_one isolates I/O to step 5 (commit) |
   311	| **I-NOENV** | step_transition dependency tree has no `std::env` access | grep test in CO1.7 module — already enforced by CLAUDE.md hardcoded-config rule (C-027) |
   312	| **I-NORANDOM** | tx consuming randomness MUST seed PRNG from `(tx.tx_id, q.state_root_t)` | LedgerEntry.system_signature uses keypair (deterministic given private key); no entropy in append/replay |
   313	
   314	CO1.7 does NOT introduce new invariants — it provides the machine-checkable witness for 8 of the 27 frozen invariants.
   315	
   316	---
   317	
   318	## § 7 Conformance tests
   319	
   320	| Test | What it asserts |
   321	|---|---|
   322	| `tests/transition_determinism.rs` | step_transition(q, tx) called twice → byte-identical Q_{t+1}; ledger_root_t identical (CO1.7 append() witness) |
   323	| `tests/q_state_reconstruct.rs` | Run N transitions live → snapshot Q_t. Cold-restart, call replay(genesis, [entries]) → assert state_root + ledger_root match snapshot. (CO1.7 replay() witness) |
   324	| `tests/l4_sequencer_serialization.rs` | Submit 100 tx concurrently from 8 threads; assert (logical_t, tx_id) is strict total order; replay produces deterministic state_root (CO1.7 sequencer witness) |
   325	| `tests/finalize_batch_order.rs` | 3 claims expire same tick; assert ordering by (expires_at, claim_id); 2 runs byte-identical (CO1.7 sequencer + § 5.2.3 witness) |
   326	| `tests/no_wall_clock_in_tx.rs` | LedgerEntry has no wall-clock field; sequencer.apply_one has no `SystemTime::now()` call (grep test) |
   327	| `tests/ledger_root_chain_integrity.rs` | NEW, CO1.7-specific: tamper with one LedgerEntry's resulting_ledger_root; replay must FAIL with LedgerRootMismatch at that index |
   328	| `tests/cas_payload_recovery.rs` | NEW, CO1.7-specific: serialize a WorkTx → CAS put → LedgerEntry references CID → CAS get → byte-identical WorkTx |
   329	| `tests/system_signature_verifies.rs` | NEW, CO1.7-specific: every committed LedgerEntry's system_signature verifies against the committed system_keypair public key |
   330	
   331	**Total CO1.7-specific tests**: 3 NEW + 5 referenced from spec § 4 = 8 conformance tests.
   332	
   333	---
   334	
   335	## § 8 Integration with step_transition family
   336	
   337	CO1.7 publishes a single function `dispatch_transition(q, typed_tx) -> (q_next, signals)` that the sequencer's `apply_one` calls. Existing transition functions in `STATE_TRANSITION_SPEC § 3-3.7` are wired into this dispatch:
   338	
   339	```rust
   340	pub(crate) fn dispatch_transition(q: &QState, tx: &TypedTx) -> Result<(QState, SignalBundle), TransitionError> {
   341	    match tx {
   342	        TypedTx::Work(t)             => step_transition(q, t, &q.predicate_registry, &q.tool_registry),
   343	        TypedTx::Verify(t)           => verify_transition(q, t, &q.predicate_registry),
   344	        TypedTx::Challenge(t)        => challenge_transition(q, t, &q.predicate_registry),
   345	        TypedTx::Reuse(t)            => reuse_transition(q, t, &q.tool_registry),
   346	        TypedTx::FinalizeReward(t)   => finalize_reward_transition(q, t),
   347	        TypedTx::TaskExpire(t)       => task_expire_transition(q, t),
   348	        TypedTx::TerminalSummary(t)  => emit_terminal_summary(q, t),
   349	    }
   350	}
   351	```
   352	
   353	**Where the transition function bodies live**: this is decided per-atom downstream (CO1.7.5 implements `step_transition`; CO1.7.6 implements verify/challenge/etc. — see Plan v3.2 § 3.4 atoms). CO1.7 itself only ships the dispatch + sequencer + ledger writer; the transition function bodies are stubs (`unimplemented!()`) that downstream atoms fill.
   354	
   355	---
   356	
   357	## § 9 STEP_B disposition
   358	
   359	CO1.7 lives in NEW files (`src/bottom_white/ledger/transition_ledger.rs`, `src/state/sequencer.rs`). It does NOT modify `src/bus.rs` / `src/kernel.rs` / `src/wal.rs` (the STEP_B-restricted files). Therefore: **no STEP_B parallel-branch ceremony required**. Direct edit on `main` is per CLAUDE.md "Code Standard".
   360	
   361	The retirement of `src/ledger.rs` (legacy top-level) is **NOT in CO1.7 scope** — it is in CO1.1.5 (kernel.rs split) per `STATE_TRANSITION_SPEC § 5.3` Legacy Economic Tx Disposition table.
   362	
   363	---
   364	
   365	## § 10 What this spec does NOT specify
   366	
   367	1. **Garbage collection of finalized claims** — claims are finalized in-place via finalize_reward_transition; no L4 entry deletion ever (append-only is constitutional, Art 0.2). CO1.8 materialized-state may compact L5 indices, but L4 stays whole.
   368	2. **Cross-cell sharing** — § 5.2.2 mandates disjoint runtime_repo per cell. Multi-tenant deployments are a v4.x extension.
   369	3. **Recovery from corrupted git history** — out of scope for v1; if `git fsck` fails, runtime aborts (fail-closed). Backup/restore strategy is operational, not specified.
   370	4. **Performance tuning** — no SLO commitments. Round-1 audit may request rough wall-clock budget.
   371	
   372	---
   373	
   374	## § 11 Open questions for round-1 audit
   375	
   376	The following are deliberately under-specified; round-1 audit input requested:
   377	
   378	- **Q1** (Codex/Gemini both): SubmissionQueue type — `tokio::sync::mpsc::UnboundedReceiver` (current proposal), `crossbeam::channel`, or `std::sync::mpsc`? Trade-off is back-pressure semantics + dep weight.
   379	- **Q2** (Codex preferred): how to surface sequencer back-pressure to agent submissions when queue is full? Async wait vs. immediate Err? Affects multi-agent fairness.
   380	- **Q3** (Gemini preferred): is `Sequencer` the right abstraction boundary, or should it be split into `LedgerWriter` (storage) + `OrderingCoordinator` (sequencer logic)? Trait segregation argument.
   381	- **Q4** (Codex): system_signature placement — inside LedgerEntry struct (current proposal, signed-entry is the canonical artifact) vs. a sidecar `(LedgerEntry, SystemSignature)` tuple. The sidecar form makes the canonical_digest computation simpler but adds a pairing concern.
   382	- **Q5** (Gemini): is the `dispatch_transition` enum-match pattern the right shape, or should we use the `MetaTransitionInterface` trait pattern (CO P3-prep.5)? Trade-off is v4/v4.1 boundary cleanliness.
   383	- **Q6** (Codex): `replay` rejects on first error (current). Should it instead collect all errors for diagnostic completeness? Trade-off is error-mode complexity.
   384	- **Q7** (Gemini): genesis ledger_root_t — `Hash::ZERO` (current) or sha256 of the genesis_payload.toml content? The latter binds replay to a specific genesis; the former is simpler but loses that anchor.
   385	- **Q8** (BOTH; surfaced post type-skeleton smoke 2026-04-28): existing `system_keypair::CanonicalMessage` has 3 fixed variants (RejectedAttemptSummary / TerminalSummaryTx / EpochRotationProof). LedgerEntry is NOT among them. Two paths: (a) extend `CanonicalMessage` enum with `LedgerEntry(LedgerEntry)` variant — touches Wave 4-B shipped code (additive, not breaking); (b) introduce a sibling sign primitive specifically for LedgerEntry that does not go through `CanonicalMessage`. Trade-off: (a) preserves single-canonical-digest principle but couples ledger to the enum; (b) decouples but introduces a second signing pathway with parallel canonical digest discipline.
   386	- **Q9** (BOTH; surfaced post type-skeleton smoke 2026-04-28): spec v1 § 1 said `canonical_digest_unsigned` "covers fields 1-7 (excludes signature)" but did NOT explicitly state that `resulting_ledger_root` (field 6) must ALSO be excluded. Skeleton's first replay test failed immediately — including `resulting_ledger_root` creates a circular dependency (`ledger_root_t+1 = append(ledger_root_t, digest)` where `digest ⊃ ledger_root_t+1`). Skeleton fixed: digest now covers `{logical_t, parent_state_root, tx_kind, tx_payload_cid, resulting_state_root, timestamp_logical, epoch}` — 7 fields, NOT including `resulting_ledger_root` and NOT including `system_signature`. Spec v1.1 must make this exclusion explicit at § 1.
   387	- **Q10** (BOTH; surfaced post smoke): spec missed `epoch: SystemEpoch` field on LedgerEntry. Without it, `verify_system_signature(sig, msg, epoch, pinned_pubkeys)` cannot resolve the pubkey to use. Skeleton added it (now field 7 of 8). Spec v1.1 must add this field.
   388	
   389	---
   390	
   391	## § 12 Audit gates (round structure mirrors INV8 / spec v1.4 / system_keypair)
   392	
   393	| Round | Codex | Gemini | Conservative | Action |
   394	|---|---|---|---|---|
   395	| 1 | ⏳ pending | ⏳ pending | TBD | initial review of this draft |
   396	| 2+ | … | … | … | iterate to PASS/PASS |
   397	
   398	**Pre-implementation gate**: CO1.7 v1 must reach `PASS/PASS` from Codex + Gemini before any `src/bottom_white/ledger/transition_ledger.rs` or `src/state/sequencer.rs` code is written. Sedimented per CLAUDE.md "Audit Standard" (Generator ≠ Evaluator) + memory `feedback_dual_audit`.
   399	
   400	---
   401	
   402	## § 13 Estimated scope
   403	
   404	- **Spec rounds**: 2-4 round dual audit (per system_keypair + spec v1.4 history)
   405	- **Implementation**: ~600-900 LoC + 8 conformance tests, est. 3-5 days post-PASS
   406	- **Total atom budget**: ~1.5-2 weeks (matches LATEST line 92 estimate)
   407	
   408	---
   409	
   410	## § 14 Honest acknowledgements
   411	
   412	1. ~~This spec presumes CO1.4 CAS layer's API surface~~ — verified post type-skeleton smoke 2026-04-28: `CasStore::get(&Cid) → Result<Vec<u8>, CasError>` matches; `CasStore::put` has wider signature than expected (5 params: `content`, `object_type`, `creator`, `created_at_logical_t`, `schema_id`) — sequencer must build full CAS metadata. **DIV-5** flagged.
   413	2. The SubmissionQueue type is a tokio choice; if the project pivots to a different async runtime, § 3 Sequencer.run() rewrites.
   414	3. § 11 Q4 + Q7 + Q8 + Q9 + Q10 are real design forks; round-1 audit settles them.
   415	4. ~~system_signature integration relies on CO1.7.0a-f's API exactly as shipped~~ — verified post smoke: `SystemSignature::from_bytes`, `SystemEpoch::new/get`, `verify_system_signature(sig, msg, epoch, pinned_pubkeys)` all public. The actual `CanonicalMessage` enum has 3 fixed variants, LedgerEntry is NOT among them. **Q8** (NEW) surfaced.
   416	5. **Spec ↔ skeleton divergences sedimented** (post 2026-04-28 smoke):
   417	   - **DIV-1**: `CanonicalMessage` enum integration → Q8 (NEW)
   418	   - **DIV-2**: Q_t mutation API not yet present → state-mutation paths in skeleton are `unimplemented!()` until CO P2.x economy atoms
   419	   - **DIV-3**: missing `epoch: SystemEpoch` field → Q10 (NEW); skeleton already added
   420	   - **DIV-4**: `CasReader` trait → narrowed to `LedgerCasView` (CasStore impls in CO1.7.5+)
   421	   - **DIV-5**: `CasStore::put` 5-param signature → sequencer responsibility documented in § 1
   422	6. **Spec v1 bug found by skeleton smoke** (Q9, NEW): `canonical_digest_unsigned` must EXCLUDE `resulting_ledger_root`, not just `system_signature`. Spec v1 § 1 wording was ambiguous; first replay test caught the cycle. Skeleton fixed; spec v1.1 must explicit.
   423	
   424	## § 15 Pre-audit smoke verification (2026-04-28)
   425	
   426	| Smoke item | Result | What it proved |
   427	|---|---|---|
   428	| `cargo check` on `src/bottom_white/ledger/transition_ledger.rs` | PASS | LedgerEntry / TxKind / append / replay_chain_integrity / InMemoryLedgerWriter all type-check against existing `Cid` (CO1.4) + `SystemSignature`/`SystemEpoch` (CO1.7.0a-f) + `Hash` (Q_t) |
   429	| `cargo test --lib bottom_white::ledger::transition_ledger::` | 6/6 PASS | append byte-stable; canonical_digest stable across clones; in-memory writer enforces logical_t monotonic; replay validates parent chain; replay rejects parent_state_root tamper; replay rejects ledger_root tamper |
   430	| `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` | PASS post TR refresh | new file `transition_ledger.rs` + modified `mod.rs` added to `genesis_payload.toml [trust_root]` |
   431	| `cargo test --lib` (full workspace) | 196/0 PASS | no regression in 190 pre-existing tests |
   432	
   433	**Audit-ready artifact set**: spec v1 (this file) + skeleton (`src/bottom_white/ledger/transition_ledger.rs`, ~370 lines incl. 6 inline tests) + 5 cataloged divergences + 4 new round-1 audit Qs (Q8/Q9/Q10/Q11). Round-1 audit has both paper + code to inspect — higher signal density than spec-only review.
   434	
   435	— ArchitectAI, session 2026-04-28; smoke-verified 2026-04-28.

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '260,460p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   260	    }
   261	}
   262	impl std::error::Error for ReplayError {}
   263	
   264	/// Replay chain integrity. Returns final `(state_root, ledger_root)` after replaying
   265	/// `entries` from a given (genesis_state_root, genesis_ledger_root) start.
   266	///
   267	/// Per **DIV-2**: this skeleton does NOT yet re-run pure transition functions to
   268	/// independently verify `entry.resulting_state_root`. That step lands in CO1.7.5
   269	/// once `dispatch_transition` is implementable.
   270	pub fn replay_chain_integrity(
   271	    genesis_state_root: Hash,
   272	    genesis_ledger_root: Hash,
   273	    entries: &[LedgerEntry],
   274	) -> Result<(Hash, Hash), ReplayError> {
   275	    let mut prev_state_root = genesis_state_root;
   276	    let mut prev_ledger_root = genesis_ledger_root;
   277	
   278	    for (i, entry) in entries.iter().enumerate() {
   279	        let expected_logical_t = (i as u64) + 1;
   280	        if entry.logical_t != expected_logical_t {
   281	            return Err(ReplayError::LogicalTGap {
   282	                at: i,
   283	                expected: expected_logical_t,
   284	                got: entry.logical_t,
   285	            });
   286	        }
   287	        if entry.parent_state_root != prev_state_root {
   288	            return Err(ReplayError::ParentMismatch { at: i });
   289	        }
   290	        let recomputed = append(&prev_ledger_root, &entry.canonical_digest_unsigned());
   291	        if recomputed != entry.resulting_ledger_root {
   292	            return Err(ReplayError::LedgerRootMismatch { at: i });
   293	        }
   294	        prev_state_root = entry.resulting_state_root;
   295	        prev_ledger_root = entry.resulting_ledger_root;
   296	    }
   297	
   298	    Ok((prev_state_root, prev_ledger_root))
   299	}
   300	
   301	// ────────────────────────────────────────────────────────────────────────────
   302	// Tests — only the genuinely-pure paths are exercised at skeleton stage
   303	// ────────────────────────────────────────────────────────────────────────────
   304	
   305	#[cfg(test)]
   306	mod tests {
   307	    use super::*;
   308	
   309	    fn h(byte: u8) -> Hash {
   310	        Hash([byte; 32])
   311	    }
   312	
   313	    fn entry_at(
   314	        logical_t: u64,
   315	        parent_state_root: Hash,
   316	        resulting_state_root: Hash,
   317	        prev_ledger_root: Hash,
   318	    ) -> LedgerEntry {
   319	        let mut entry = LedgerEntry {
   320	            logical_t,
   321	            parent_state_root,
   322	            tx_kind: TxKind::Work,
   323	            tx_payload_cid: Cid([0u8; 32]),
   324	            resulting_state_root,
   325	            resulting_ledger_root: Hash::ZERO, // patched below
   326	            timestamp_logical: logical_t,
   327	            epoch: SystemEpoch::new(1),
   328	            system_signature: SystemSignature::from_bytes([0u8; 64]),
   329	        };
   330	        entry.resulting_ledger_root = append(&prev_ledger_root, &entry.canonical_digest_unsigned());
   331	        entry
   332	    }
   333	
   334	    #[test]
   335	    fn append_is_pure_and_byte_stable() {
   336	        let a = append(&Hash::ZERO, &h(1));
   337	        let b = append(&Hash::ZERO, &h(1));
   338	        assert_eq!(a, b, "I-DET witness on append()");
   339	        let c = append(&Hash::ZERO, &h(2));
   340	        assert_ne!(a, c, "different entry digests must produce different roots");
   341	    }
   342	
   343	    #[test]
   344	    fn canonical_digest_byte_stable_across_clones() {
   345	        let e = entry_at(1, Hash::ZERO, h(0xaa), Hash::ZERO);
   346	        let d1 = e.canonical_digest_unsigned();
   347	        let e2 = e.clone();
   348	        let d2 = e2.canonical_digest_unsigned();
   349	        assert_eq!(d1, d2);
   350	    }
   351	
   352	    #[test]
   353	    fn in_memory_writer_enforces_logical_t() {
   354	        let mut w = InMemoryLedgerWriter::new();
   355	        let e1 = entry_at(1, Hash::ZERO, h(1), Hash::ZERO);
   356	        assert!(w.commit(&e1).is_ok());
   357	        assert_eq!(w.len(), 1);
   358	
   359	        let e_skip = entry_at(3, e1.resulting_state_root, h(2), e1.resulting_ledger_root);
   360	        let err = w.commit(&e_skip).unwrap_err();
   361	        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
   362	    }
   363	
   364	    #[test]
   365	    fn replay_validates_parent_chain() {
   366	        let e1 = entry_at(1, Hash::ZERO, h(1), Hash::ZERO);
   367	        let e2 = entry_at(2, e1.resulting_state_root, h(2), e1.resulting_ledger_root);
   368	        let e3 = entry_at(3, e2.resulting_state_root, h(3), e2.resulting_ledger_root);
   369	        let (final_state, final_ledger) =
   370	            replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1.clone(), e2.clone(), e3.clone()])
   371	                .expect("clean chain replays");
   372	        assert_eq!(final_state, e3.resulting_state_root);
   373	        assert_eq!(final_ledger, e3.resulting_ledger_root);
   374	    }
   375	
   376	    #[test]
   377	    fn replay_rejects_parent_mismatch() {
   378	        let e1 = entry_at(1, Hash::ZERO, h(1), Hash::ZERO);
   379	        // e2 lies about parent_state_root
   380	        let mut e2 = entry_at(2, e1.resulting_state_root, h(2), e1.resulting_ledger_root);
   381	        e2.parent_state_root = h(0xff); // tamper
   382	        e2.resulting_ledger_root = append(&e1.resulting_ledger_root, &e2.canonical_digest_unsigned());
   383	
   384	        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
   385	        assert!(matches!(err, ReplayError::ParentMismatch { at: 1 }));
   386	    }
   387	
   388	    #[test]
   389	    fn replay_rejects_ledger_root_tamper() {
   390	        let e1 = entry_at(1, Hash::ZERO, h(1), Hash::ZERO);
   391	        let mut e2 = entry_at(2, e1.resulting_state_root, h(2), e1.resulting_ledger_root);
   392	        // Don't recompute resulting_ledger_root — tamper directly
   393	        e2.resulting_ledger_root = h(0xee);
   394	
   395	        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
   396	        assert!(matches!(err, ReplayError::LedgerRootMismatch { at: 1 }));
   397	    }
   398	}

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '280,520p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   280	        let secret = self.secret_slice()?;
   281	        let mut plaintext = [0u8; PLAINTEXT_LEN];
   282	        plaintext[..SECRET_KEY_LEN].copy_from_slice(secret);
   283	        plaintext[SECRET_KEY_LEN..].copy_from_slice(self.public_key.as_bytes());
   284	        Ok(plaintext)
   285	    }
   286	
   287	    fn sign_digest(&self, digest: [u8; 32]) -> Result<SystemSignature, KeypairError> {
   288	        let mut secret = [0u8; SECRET_KEY_LEN];
   289	        secret.copy_from_slice(self.secret_slice()?);
   290	        let signing_key = SigningKey::from_bytes(&secret);
   291	        let signature = signing_key.sign(&digest);
   292	        secret.zeroize();
   293	        Ok(SystemSignature::from_bytes(signature.to_bytes()))
   294	    }
   295	
   296	    fn secret_slice(&self) -> Result<&[u8], KeypairError> {
   297	        if self.secret_key.len() == SECRET_KEY_LEN {
   298	            Ok(&self.secret_key)
   299	        } else {
   300	            Err(KeypairError::InvalidFormat("bad in-memory secret length"))
   301	        }
   302	    }
   303	
   304	    fn mlock_private_key_best_effort(&mut self) -> bool {
   305	        mlock_best_effort(self.secret_key.as_ptr(), self.secret_key.len())
   306	    }
   307	}
   308	
   309	/// TRACE_MATRIX FC1-Sig+FC3-Sig: system keypair lifecycle and crypto error taxonomy.
   310	#[derive(Debug)]
   311	pub enum KeypairError {
   312	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: filesystem operation failed.
   313	    Io(std::io::Error),
   314	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: secure operating-system entropy failed.
   315	    Entropy(getrandom::Error),
   316	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: KDF environment parameter was absent or invalid.
   317	    KdfParam(String),
   318	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: Argon2id key derivation failed.
   319	    Kdf(argon2::Error),
   320	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: ChaCha20-Poly1305 encryption or authentication failed.
   321	    Crypto(&'static str),
   322	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: encrypted keystore format was malformed.
   323	    InvalidFormat(&'static str),
   324	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: default keystore path could not be resolved.
   325	    HomeUnavailable,
   326	}
   327	
   328	impl fmt::Display for KeypairError {
   329	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   330	        match self {
   331	            Self::Io(err) => write!(f, "system keypair I/O failed: {err}"),
   332	            Self::Entropy(err) => write!(f, "system keypair entropy failed: {err}"),
   333	            Self::KdfParam(msg) => write!(f, "system keypair KDF parameter invalid: {msg}"),
   334	            Self::Kdf(err) => write!(f, "system keypair KDF failed: {err}"),
   335	            Self::Crypto(msg) => write!(f, "system keypair crypto failed: {msg}"),
   336	            Self::InvalidFormat(msg) => write!(f, "system keypair keystore invalid: {msg}"),
   337	            Self::HomeUnavailable => {
   338	                write!(f, "system keypair default keystore path requires HOME")
   339	            }
   340	        }
   341	    }
   342	}
   343	
   344	impl std::error::Error for KeypairError {}
   345	
   346	impl From<std::io::Error> for KeypairError {
   347	    fn from(value: std::io::Error) -> Self {
   348	        Self::Io(value)
   349	    }
   350	}
   351	
   352	/// TRACE_MATRIX FC1-Sig+FC3-Sig: resolve `~/.turingos/keystore/system_keypair_v{epoch}.enc`.
   353	///
   354	/// `TURINGOS_KEYSTORE_PATH` overrides the default path. The default never
   355	/// points into the repository, CAS, or ledger directories.
   356	pub fn default_system_keystore_path(epoch: SystemEpoch) -> Result<PathBuf, KeypairError> {
   357	    if let Ok(path) = env::var("TURINGOS_KEYSTORE_PATH") {
   358	        return Ok(PathBuf::from(path));
   359	    }
   360	    let home = env::var("HOME").map_err(|_| KeypairError::HomeUnavailable)?;
   361	    Ok(PathBuf::from(home)
   362	        .join(".turingos")
   363	        .join("keystore")
   364	        .join(format!("system_keypair_v{}.enc", epoch.get())))
   365	}
   366	
   367	/// TRACE_MATRIX FC1-Sig+FC3-Sig: first-boot generate-or-second-boot decrypt lifecycle entrypoint.
   368	pub fn generate_or_load_system_keypair(
   369	    keystore_path: &Path,
   370	    user_kdf_password: &SecretString,
   371	) -> Result<Ed25519Keypair, KeypairError> {
   372	    if keystore_path.exists() {
   373	        return load_existing_keypair(keystore_path, user_kdf_password);
   374	    }
   375	
   376	    let keypair = Ed25519Keypair::generate_with_secure_entropy()?;
   377	    let encrypted = encrypt_at_rest(&keypair, user_kdf_password)?;
   378	    write_keystore_0600(keystore_path, &encrypted)?;
   379	    Ok(keypair)
   380	}
   381	
   382	/// TRACE_MATRIX FC1-Sig+FC3-Sig: decrypt an existing encrypted system keypair keystore.
   383	pub fn load_existing_keypair(
   384	    keystore_path: &Path,
   385	    user_kdf_password: &SecretString,
   386	) -> Result<Ed25519Keypair, KeypairError> {
   387	    let bytes = fs::read(keystore_path)?;
   388	    let encoded = EncryptedKeypair::decode(&bytes)?;
   389	    let mut key = derive_key(user_kdf_password, &encoded.salt, encoded.kdf)?;
   390	    let cipher = ChaCha20Poly1305::new_from_slice(&key)
   391	        .map_err(|_| KeypairError::Crypto("bad cipher key"))?;
   392	    let plaintext = cipher
   393	        .decrypt(
   394	            Nonce::from_slice(&encoded.nonce),
   395	            encoded.ciphertext.as_ref(),
   396	        )
   397	        .map_err(|_| KeypairError::Crypto("keystore authentication failed"))?;
   398	    key.zeroize();
   399	    Ed25519Keypair::from_plaintext(&plaintext)
   400	}
   401	
   402	/// TRACE_MATRIX FC1-Sig+FC3-Sig: canonical SHA-256 digest for typed system messages.
   403	pub fn canonical_digest(message: &CanonicalMessage) -> [u8; 32] {
   404	    let mut h = Sha256::new();
   405	    h.update(b"turingosv4.system_keypair.v1");
   406	    match message {
   407	        CanonicalMessage::RejectedAttemptSummary(summary) => {
   408	            h.update(b"RejectedAttemptSummary");
   409	            update_len_prefixed(&mut h, summary.run_id.as_bytes());
   410	            update_len_prefixed(&mut h, summary.attempt_id.as_bytes());
   411	            update_len_prefixed(&mut h, summary.failure_class.as_bytes());
   412	            h.update(summary.summary_hash);
   413	        }
   414	        CanonicalMessage::TerminalSummaryTx(tx) => {
   415	            h.update(b"TerminalSummaryTx");
   416	            update_len_prefixed(&mut h, tx.run_id.as_bytes());
   417	            h.update(tx.terminal_state_root);
   418	            h.update(tx.rejected_attempt_count.to_be_bytes());
   419	        }
   420	        CanonicalMessage::EpochRotationProof(proof) => {
   421	            h.update(b"EpochRotationProof");
   422	            h.update(proof.old_epoch.get().to_be_bytes());
   423	            h.update(proof.new_epoch.get().to_be_bytes());
   424	            h.update(proof.old_pubkey.as_bytes());
   425	            h.update(proof.new_pubkey.as_bytes());
   426	            h.update(proof.signed_at_unix.to_be_bytes());
   427	        }
   428	    }
   429	    h.finalize().into()
   430	}
   431	
   432	/// TRACE_MATRIX FC1-Sig+FC3-Sig: public system signature verification against pinned epoch keys.
   433	pub fn verify_system_signature(
   434	    sig: &SystemSignature,
   435	    message: &CanonicalMessage,
   436	    epoch: SystemEpoch,
   437	    pinned_pubkeys: &PinnedSystemPubkeys,
   438	) -> bool {
   439	    let Some(pk) = pinned_pubkeys.get(epoch) else {
   440	        return false;
   441	    };
   442	    let Ok(verifying_key) = VerifyingKey::from_bytes(pk.as_bytes()) else {
   443	        return false;
   444	    };
   445	    let signature = Signature::from_bytes(sig.as_bytes());
   446	    verifying_key
   447	        .verify(&canonical_digest(message), &signature)
   448	        .is_ok()
   449	}
   450	
   451	/// TRACE_MATRIX FC3-Sig: verify old and new signatures over a rotation continuity proof.
   452	pub fn verify_epoch_rotation_proof(
   453	    proof: &EpochRotationProof,
   454	    old_signature: &SystemSignature,
   455	    new_signature: &SystemSignature,
   456	    pinned_pubkeys: &PinnedSystemPubkeys,
   457	) -> bool {
   458	    let message = CanonicalMessage::EpochRotationProof(proof.clone());
   459	    verify_system_signature(old_signature, &message, proof.old_epoch(), pinned_pubkeys)
   460	        && verify_system_signature(new_signature, &message, proof.new_epoch(), pinned_pubkeys)
   461	}
   462	
   463	/// TRACE_MATRIX FC3-Sig: boot extension stub for genesis `[system_pubkeys]` verification.
   464	pub fn verify_system_pubkeys(genesis_payload_toml: &str) -> Result<(), TrustRootError> {
   465	    if !has_toml_section(genesis_payload_toml, "system_pubkeys") {
   466	        return Ok(());
   467	    }
   468	    // TODO(CO1.7): parse genesis_payload.toml [system_pubkeys] entries and
   469	    // verify creator PGP signatures against the pinned creator public key.
   470	    Ok(())
   471	}
   472	
   473	/// TRACE_MATRIX FC1-Sig: crate-only signing surface for the predicate runner.
   474	pub(crate) mod predicate_runner {
   475	    use super::{
   476	        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, KeypairError,
   477	        RejectedAttemptSummary, SystemSignature,
   478	    };
   479	
   480	    /// TRACE_MATRIX FC1-Sig: sign only typed rejected-attempt summaries from the predicate runner.
   481	    pub(crate) fn sign_rejected_attempt_summary(
   482	        keypair: &Ed25519Keypair,
   483	        summary: &RejectedAttemptSummary,
   484	    ) -> Result<SystemSignature, KeypairError> {
   485	        sign_system_message_inner(
   486	            keypair,
   487	            &CanonicalMessage::RejectedAttemptSummary(summary.clone()),
   488	        )
   489	    }
   490	
   491	    /// TRACE_MATRIX FC1-Sig: sign only typed canonical messages within the predicate-runner scope.
   492	    pub(crate) fn sign_system_message(
   493	        keypair: &Ed25519Keypair,
   494	        message: &CanonicalMessage,
   495	    ) -> Result<SystemSignature, KeypairError> {
   496	        sign_system_message_inner(keypair, message)
   497	    }
   498	}
   499	
   500	/// TRACE_MATRIX FC1-Sig+FC3-Sig: crate-only signing surface for terminal summary emission.
   501	pub(crate) mod terminal_summary_emitter {
   502	    use super::{
   503	        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, EpochRotationProof,
   504	        KeypairError, SystemSignature, TerminalSummaryTx,
   505	    };
   506	
   507	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: sign only typed terminal summary transactions.
   508	    pub(crate) fn sign_terminal_summary_tx(
   509	        keypair: &Ed25519Keypair,
   510	        tx: &TerminalSummaryTx,
   511	    ) -> Result<SystemSignature, KeypairError> {
   512	        sign_system_message_inner(keypair, &CanonicalMessage::TerminalSummaryTx(tx.clone()))
   513	    }
   514	
   515	    /// TRACE_MATRIX FC3-Sig: sign only typed epoch rotation proofs.
   516	    pub(crate) fn sign_epoch_rotation_proof(
   517	        keypair: &Ed25519Keypair,
   518	        proof: &EpochRotationProof,
   519	    ) -> Result<SystemSignature, KeypairError> {
   520	        sign_system_message_inner(

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/schema.rs | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
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
    13	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

exec
/bin/bash -lc 'rg -n "STEP_B|restricted|bus.rs|kernel.rs|wal.rs|transition_ledger|src/state/sequencer|new files|conflict" handover/ai-direct/STEP_B_PROTOCOL.md CLAUDE.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
CLAUDE.md:14:- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
handover/ai-direct/STEP_B_PROTOCOL.md:3:**Scope**: any change to files in CLAUDE.md's restricted list (currently `kernel.rs`, `bus.rs`, `wallet.rs`). Also applicable to any proposal that touches "institution" per C-031.
handover/ai-direct/STEP_B_PROTOCOL.md:13:- Is a less-invasive alternative available? (same effect, no restricted-file touch)
handover/ai-direct/STEP_B_PROTOCOL.md:17:**Gate**: if both auditors say "less-invasive alternative exists", take that path instead. If both say "change is necessary", proceed to Phase 1. If they disagree, take the conservative verdict per `feedback_dual_audit_conflict`.
handover/ai-direct/STEP_B_PROTOCOL.md:107:1. **C-010**: external audit at 3 junctures (necessity, diff, verdict). Researcher cannot self-approve a restricted change.
handover/ai-direct/STEP_B_PROTOCOL.md:136:**`src/bus.rs recent_rejections` Art. II.1 fix** (notepad §5, F-2026-04-15-02):

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '520,580p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   520	        sign_system_message_inner(
   521	            keypair,
   522	            &CanonicalMessage::EpochRotationProof(proof.clone()),
   523	        )
   524	    }
   525	
   526	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: sign only typed canonical messages within terminal-summary scope.
   527	    pub(crate) fn sign_system_message(
   528	        keypair: &Ed25519Keypair,
   529	        message: &CanonicalMessage,
   530	    ) -> Result<SystemSignature, KeypairError> {
   531	        sign_system_message_inner(keypair, message)
   532	    }
   533	}
   534	
   535	fn sign_system_message_inner(
   536	    keypair: &Ed25519Keypair,
   537	    message: &CanonicalMessage,
   538	) -> Result<SystemSignature, KeypairError> {
   539	    keypair.sign_digest(canonical_digest(message))
   540	}
   541	
   542	fn encrypt_at_rest(
   543	    keypair: &Ed25519Keypair,
   544	    user_kdf_password: &SecretString,
   545	) -> Result<Vec<u8>, KeypairError> {
   546	    let kdf = KdfParams::from_env()?;
   547	    let mut salt = [0u8; SALT_LEN];
   548	    let mut nonce = [0u8; NONCE_LEN];
   549	    getrandom::getrandom(&mut salt).map_err(KeypairError::Entropy)?;
   550	    getrandom::getrandom(&mut nonce).map_err(KeypairError::Entropy)?;
   551	
   552	    let mut key = derive_key(user_kdf_password, &salt, kdf)?;
   553	    let cipher = ChaCha20Poly1305::new_from_slice(&key)
   554	        .map_err(|_| KeypairError::Crypto("bad cipher key"))?;
   555	    let mut plaintext = keypair.to_plaintext()?;
   556	    let ciphertext = cipher
   557	        .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
   558	        .map_err(|_| KeypairError::Crypto("keystore encryption failed"))?;
   559	    plaintext.zeroize();
   560	    key.zeroize();
   561	
   562	    EncryptedKeypair {
   563	        kdf,
   564	        salt,
   565	        nonce,
   566	        ciphertext,
   567	    }
   568	    .encode()
   569	}
   570	
   571	fn derive_key(
   572	    user_kdf_password: &SecretString,
   573	    salt: &[u8; SALT_LEN],
   574	    kdf: KdfParams,
   575	) -> Result<[u8; DERIVED_KEY_LEN], KeypairError> {
   576	    let params = Params::new(
   577	        kdf.memory_kib,
   578	        kdf.iterations,
   579	        kdf.lanes,
   580	        Some(DERIVED_KEY_LEN),

exec
/bin/bash -lc 'rg -n "struct QState|ledger_root_t|state_root_t|EconomicState|BalancesIndex|ClaimsIndex|impl .*Index|type .*Index|BTreeMap" src/state/q_state.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
8://! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
10://! **BTreeMap, not HashMap, everywhere** (Inv determinism;
15://! but the *index typing* (BTreeMap newtype shells) freezes here so Q_t is total.
17:use std::collections::BTreeMap;
84:    pub agents: BTreeMap<AgentId, PerAgentState>,
107:    pub views: BTreeMap<AgentId, NodeId>,
135:// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
139:/// is a BTreeMap newtype; entry shapes (Escrow / Stake / Claim / TaskMarket /
143:pub struct EconomicState {
144:    pub balances_t: BalancesIndex,
147:    pub claims_t: ClaimsIndex,
157:pub struct BalancesIndex(pub BTreeMap<AgentId, MicroCoin>);
161:pub struct EscrowsIndex(pub BTreeMap<TxId, EscrowEntry>);
182:pub struct StakesIndex(pub BTreeMap<TxId, StakeEntry>);
201:pub struct ClaimsIndex(pub BTreeMap<TxId, ClaimEntry>);
220:pub struct ReputationsIndex(pub BTreeMap<AgentId, Reputation>);
224:pub struct TaskMarketsIndex(pub BTreeMap<TxId, TaskMarketEntry>);
262:pub struct RoyaltyGraph(pub BTreeMap<TxId, Vec<RoyaltyEdge>>);
275:pub struct ChallengeCasesIndex(pub BTreeMap<TxId, ChallengeCase>);
296:pub struct PriceIndex(pub BTreeMap<TxId, MicroCoin>);
307:pub struct QState {
313:    pub state_root_t: Hash,
317:    pub ledger_root_t: Hash,
323:    pub economic_state_t: EconomicState,
330:    /// roots populated by `boot::verify_trust_root` and the `state_root_t` published
354:        assert_eq!(g.state_root_t, Hash::ZERO);
371:            "state_root_t",
373:            "ledger_root_t",
385:        let e = EconomicState::default();
391:            "EconomicState must have 9 sub-fields per WP § 2; got {}",
399:        let mut a = BalancesIndex::default();
403:        let mut b = BalancesIndex::default();
409:        assert_eq!(sa, sb, "BTreeMap must yield identical bytes regardless of insertion order");

exec
/bin/bash -lc "nl -ba CLAUDE.md | sed -n '1,80p' && nl -ba handover/ai-direct/STEP_B_PROTOCOL.md | sed -n '1,40p'" in /home/zephryj/projects/turingosv4
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
     1	# Step-B Protocol: Restricted-File Changes via Parallel Branch Experiment
     2	
     3	**Scope**: any change to files in CLAUDE.md's restricted list (currently `kernel.rs`, `bus.rs`, `wallet.rs`). Also applicable to any proposal that touches "institution" per C-031.
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

exec
/bin/bash -lc "rg --files handover/specs | rg 'STATE_TRANSITION|SYSTEM_KEYPAIR|WHITEPAPER|META_TRANSITION'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md
handover/specs/STATE_TRANSITION_SPEC_TLA_2026-04-27.tla
handover/specs/META_TRANSITION_INTERFACE_v1_2026-04-27.md

codex
The shipped keypair API confirms the skeleton’s DIV-1 premise: verification is typed through `CanonicalMessage`, and there is no public raw digest signing/verification path for ledger entries. I’m checking the transition spec and QState now for invariant coverage and the Q mutation boundary.
exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '1,340p'" in /home/zephryj/projects/turingosv4
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

exec
/bin/bash -lc 'rg -n "§ 2.5 Canonical|§ 4 Named|I-DETHASH|I-PARENT|I-SIG|I-LOGTIME|I-FINALIZE-BATCH-ORDER|5.2.1 L4 Sequencer|5.2.6 Sequencer Tie-Break|Genesis|ledger_root|bincode|with_big_endian|Hash::ZERO|ChallengeWindow" handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
7:> - **§ 3.2 challenge_transition + § 3.4 finalize_reward** (Q2.4): both now invoke `ChallengeWindow::is_open(now)` method (defined § 5.2.5 NEW); pseudocode no longer hand-codes the inequality.
15:> - **I-FINALIZE-BATCH-ORDER + § 5.2.3 + test all use `claim_id`** consistently (was 3-way contradiction with `target_work_tx`) (Codex new-issue #4 fix)
27:> - **§ 4 invariants** — 22 → 27 (added I-STAKE-RETURN, I-BOUNTY-REFUND, I-FINALIZE-BATCH-ORDER, I-CHALLENGE-WINDOW-EDGE, I-AGENT-INIT)
86:    pub ledger_root_t: Hash,
291:## § 2.5 Canonical Serialization (v1.2 NEW per Codex Q5)
293:> **Required because**: `tx.canonical_digest()` is called in spec § 3 stages 2 of WorkTx / VerifyTx / ChallengeTx, but byte-level format is undefined. STEP_B branch A vs branch B may pick different serialization (JSON sorted keys vs bincode vs Rust derive order) → cross-branch signature verification fails. Mandatory canonical format closes this.
295:**Format**: **bincode v2** (`bincode::serde`) with the following constraints:
297:- **`BTreeMap` keys serialized in lexicographic byte order** (this is bincode default; verified by test)
306:    let bytes = bincode::serde::encode_to_vec(value, bincode_canonical_config()).expect("serialize");
310:fn bincode_canonical_config() -> bincode::config::Configuration {
311:    bincode::config::standard()
312:        .with_big_endian()
322:**STEP_B implication**: branches A and B both use this exact `bincode_canonical_config`; signature verification works cross-branch by construction.
324:**Out of scope for v1.x spec** (deferred per Codex Q5/NEW-5 round-3 PARTIAL acknowledgment): full golden fixture corpus + differential fuzzing seed + complete runner ABI for QState/SignalBundle/TransitionError. v1.4 freezes the SERIALIZATION RULE (bincode v2 big-endian + BTreeMap lex); fixtures + ABI land in **CO1.1.4-pre1** (canonical fixture corpus) + **CO1.7** (full ABI surface). This is an **explicit deferral** — not unresolved spec ambiguity. STEP_B branch A and branch B both implement the SAME bincode rule; per-tx digest matching is mechanical from v1.4. Full corpus generation is a downstream code task, not spec scope.
395:    let new_ledger_root = ledger::append(&q.ledger_root_t, tx);
396:    q_next.ledger_root_t = new_ledger_root;
465:    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
491:        .ok_or(TransitionError::ChallengeWindowClosed)?;
492:    // v1.4: use ChallengeWindow::is_open(now) per § 5.2.5; same rule used by finalize_reward
494:        return Err(TransitionError::ChallengeWindowClosed);
559:    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
623:    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
646:    // v1.4: invoke ChallengeWindow::is_open(now) per § 5.2.5 with explicit `now` arg;
650:            return Err(TransitionError::ChallengeWindowStillOpen);
699:    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &FinalizeTx::from(claim_id, reward));
757:    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
851:    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &summary);
863:## § 4 Named Invariants (machine-checkable)
868:| I-DETHASH | `state_root_t` after replay from genesis matches authoritative state | replay test | `tests/q_state_reconstruct.rs` |
870:| I-PARENT | tx.parent_state_root must equal q.state_root_t | stage 1 | `tests/stale_parent_rejection.rs` |
871:| I-SIG | tx.signature verifies against tx.canonical_digest() | stage 2 | `tests/signature_verification.rs` |
875:| I-LOGTIME | timestamp_logical strictly monotonic per-tx; no wall clock | stage 6 | `tests/no_wall_clock_in_tx.rs` |
891:| **I-FINALIZE-BATCH-ORDER** (v1.3 corrected: single key throughout) | When N claims become finalizable at the same logical_t, finalize_tx emit order is `(expires_at_logical ASC, claim_id ASC)` — `claim_id` (NOT `target_work_tx`) used everywhere: invariant + § 5.2.3 + conformance test all consistent. | runtime finalize loop + § 5.2.3 | `tests/finalize_batch_order.rs` |
901:For ordering + replay invariants (I-DET, I-DETHASH, I-LOGTIME), Codex suggested TLA+/PlusCal. ArchitectAI agrees with the suggestion but does NOT include the full model in v1 of this spec — it would balloon the doc. Skeleton:
908:Init == /\ q = GenesisQState
948:**Why**: spec § 3 pseudocode is single-threaded; CO P1 may parallelize Phase C 5 modes × N seeds. Without explicit serialization rule, two work_tx can race on same parent_state_root, both pass `I-PARENT`, but produce different (logical_t, tx_id) sequences across STEP_B branches → state_root divergence.
950:### 5.2.1 L4 Sequencer
974:- Order = `(claim.expires_at_logical ASC, claim.claim_id ASC)` (stable, deterministic) — v1.3 fix: uses `claim_id` consistently (NOT `target_work_tx`) to align with `I-FINALIZE-BATCH-ORDER` invariant + conformance test
984:### 5.2.5 ChallengeWindow::is_open (v1.4 NEW per Codex Q2.4)
987:impl ChallengeWindow {
999:### 5.2.6 Sequencer Tie-Break (v1.4 NEW per Codex Q6)
1068:- A list of **27 named invariants** (was 16 in v1; 22 in v1.1; +5 in v1.2: I-STAKE-RETURN / I-BOUNTY-REFUND / I-FINALIZE-BATCH-ORDER / I-CHALLENGE-WINDOW-EDGE / I-AGENT-INIT) each backed by a conformance test path

exec
/bin/bash -lc "nl -ba handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md | sed -n '1,160p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# System Keypair Security Spec v1
     2	
     3	> **Date**: 2026-04-27
     4	> **Purpose**: Gemini v3.2 review Q9 VETO — runtime system keypair (signs `RejectedAttemptSummary` + `TerminalSummaryTx`) lifecycle was unspecified; this doc defines generation, storage, rotation, threat model, and audit gates.
     5	> **Authority**: Constitution Art. 0–0.4 (Tape Canonical) + Art. V.1 (separation of powers).
     6	> **Audit**: Gemini v3.2 cross-review (2026-04-27) marked Plan v3.2 VETO until this exists.
     7	
     8	---
     9	
    10	## § 1 Why a system keypair exists
    11	
    12	D-VETO-6 / Codex CHALLENGE: failure metadata cannot be agent self-reported (wrong trust boundary). The runtime's white-box predicate runner stamps `RejectedAttemptSummary` onto next accepted `WorkTx`, and emits `TerminalSummaryTx` on no-accept runs. These stamps must be **cryptographically bound to the runtime instance**, not forgeable by any agent.
    13	
    14	Hence: a runtime-level `system_keypair` distinct from any agent keypair.
    15	
    16	---
    17	
    18	## § 2 Threat Model
    19	
    20	### Adversary capabilities
    21	- A1: malicious agent inside swarm — can submit work_tx / verify_tx / challenge_tx with arbitrary contents but **cannot sign as system**
    22	- A2: external attacker without runtime access — can read public tape; cannot inject tx
    23	- A3: external attacker with read access to runtime host filesystem — can attempt to extract keypair from disk
    24	- A4: external attacker with full runtime host compromise — can read memory, sign arbitrary system tx until detected
    25	- A5: malicious code path inside runtime (e.g., compromised dependency) — can call sign API arbitrarily
    26	
    27	### What the keypair MUST defend against
    28	- A1 forging system signatures → **defended by signature verification at append time**
    29	- A2 reading retry metadata for Goodhart attack → **partially defended by L1 visibility filter**; system signature is orthogonal
    30	- A3 extracting key from rest disk → **defended by encrypted-at-rest with user-derived KDF**
    31	
    32	### What the keypair CANNOT defend against
    33	- A4 full host compromise → **acknowledged loss**; mitigation = epoch rotation + L4 anchor proves PAST tx still valid
    34	- A5 internal call abuse → **partially mitigated** by sign API only callable from `predicate_runner` + `terminal_summary_emitter` modules; static analysis enforces
    35	
    36	### Recovery requirement
    37	If A4/A5 detected: user can rotate to a new system keypair via Art V.3 amendment + signed git tag. Old key remains pinned in L4 history for replay verification but is invalidated for new tx. Future tx use new key.
    38	
    39	---
    40	
    41	## § 3 Keypair Lifecycle
    42	
    43	### 3.1 Generation
    44	
    45	**When**: at runtime first boot (post-genesis verification).
    46	
    47	**How**:
    48	```rust
    49	// src/bottom_white/ledger/system_keypair.rs (NEW per CO1.7.0b)
    50	pub fn generate_or_load_system_keypair(
    51	    keystore_path: &Path,
    52	    user_kdf_password: &SecretString,
    53	) -> Result<Ed25519Keypair, KeypairError> {
    54	    if keystore_path.exists() {
    55	        return load_existing_keypair(keystore_path, user_kdf_password);
    56	    }
    57	
    58	    // First boot: generate ed25519 keypair
    59	    let keypair = Ed25519Keypair::generate_with_secure_entropy()?;
    60	
    61	    // Encrypt with user-derived key (Argon2id KDF on user_kdf_password)
    62	    let encrypted = encrypt_at_rest(&keypair, user_kdf_password)?;
    63	    fs::write(keystore_path, encrypted)?;
    64	    set_file_permissions(keystore_path, 0o600)?;
    65	
    66	    Ok(keypair)
    67	}
    68	```
    69	
    70	**Algorithm**: ed25519 (small, fast, deterministic signatures, side-channel resistant).
    71	**Entropy**: from `getrandom(2)` (Linux) / `SecRandomCopyBytes` (macOS); never from agent input or PRNG seeded by tape.
    72	
    73	### 3.2 Storage
    74	
    75	**At rest**:
    76	- Path: `~/.turingos/keystore/system_keypair_v{epoch}.enc` (NOT in repo, NOT in any cas/ledger directory)
    77	- Permissions: 0o600 (user read/write only)
    78	- Encryption: ChaCha20-Poly1305 with key derived from user password via Argon2id (32-byte derived key)
    79	- KDF parameters: Argon2id with m=64MB, t=3, p=4 (current OWASP recommendation as of 2026-04)
    80	- Salt: stored alongside encrypted blob; randomly generated on first encryption
    81	
    82	**In memory**:
    83	- Loaded once at boot; held in `Arc<Ed25519Keypair>` inside `Runtime` struct
    84	- Memory-locked via `mlock(2)` to prevent swap to disk
    85	- Zeroized on Runtime drop (`zeroize` crate)
    86	- NEVER serialized to ledger/CAS/log
    87	- NEVER passed to agent code or LLM payload
    88	
    89	### 3.3 Sign API contract
    90	
    91	```rust
    92	// Only callable from these two paths, enforced by `pub(restricted)` + cargo-deny
    93	pub(restricted = predicate_runner, terminal_summary_emitter)
    94	fn sign_system_message(
    95	    keypair: &Ed25519Keypair,
    96	    message: &CanonicalMessage,
    97	) -> SystemSignature {
    98	    keypair.sign(canonical_digest(message))
    99	}
   100	```
   101	
   102	`CanonicalMessage` is a typed enum — `RejectedAttemptSummary | TerminalSummaryTx | EpochRotationProof`. No free-form message signing exposed.
   103	
   104	### 3.4 Verification (public)
   105	
   106	```rust
   107	pub fn verify_system_signature(
   108	    sig: &SystemSignature,
   109	    message: &CanonicalMessage,
   110	    epoch: SystemEpoch,
   111	    pinned_pubkeys: &PinnedSystemPubkeys,
   112	) -> bool {
   113	    let pk = pinned_pubkeys.get(epoch).expect("epoch pubkey missing");
   114	    pk.verify(canonical_digest(message), sig)
   115	}
   116	```
   117	
   118	`pinned_pubkeys` is loaded from `genesis_payload.toml` `[system_pubkeys]` section (NEW per this spec). Each epoch has a public key entry; private key is in encrypted keystore.
   119	
   120	### 3.5 Rotation
   121	
   122	**Trigger conditions**:
   123	- Suspected compromise (A4/A5)
   124	- Scheduled rotation: every 12 months from epoch start (long enough for v4 timeline)
   125	- Architecture amendment that changes signature algorithm
   126	
   127	**Procedure**:
   128	1. User runs `cargo run --bin rotate-system-keypair -- --new-epoch=<N+1>`
   129	2. Tool generates fresh ed25519 keypair
   130	3. Tool emits `EpochRotationProof` signed by **both** old (epoch N) and new (epoch N+1) keys, certifying continuity
   131	4. User PGP-signs the rotation: `git tag -s v4-syskey-rotate-N-to-N+1`
   132	5. `genesis_payload.toml` `[system_pubkeys]` updated to add new pubkey
   133	6. AUDIT_LEDGER row added with: rotation timestamp, old/new fingerprint, rotation tag fingerprint, user signature verification
   134	
   135	**After rotation**:
   136	- All NEW tx signed with epoch N+1 key
   137	- Old epoch N key retained in `[system_pubkeys]` for verifying historical L4 entries
   138	- Old encrypted keystore moved to `~/.turingos/keystore/archive/system_keypair_v{N}.enc.archived`
   139	
   140	### 3.6 Compromise response
   141	
   142	If A4 host compromise detected (e.g., user notices unauthorized tx):
   143	1. **STOP runtime immediately** (kill process, prevent further sign calls)
   144	2. Verify via `git log` whether unauthorized commits exist; if so, trigger Art V.3 amendment
   145	3. Run `cargo run --bin emergency-rotate-system-keypair`
   146	4. Future runtime starts use new key
   147	5. AUDIT_LEDGER + LATEST.md document the event
   148	6. Constitution Art V.3 amendment if compromise window changes any L4 invariants
   149	
   150	**Note**: post-compromise, OLD tx signed by compromised key remain VERIFIABLE (their pubkey still pinned). What's lost: confidence that those tx were emitted by legitimate runtime. User can choose to mass-mark them as "post-compromise quarantine" via amendment, but that's a policy decision, not a cryptographic one.
   151	
   152	---
   153	
   154	## § 4 Conformance Tests (5 new)
   155	
   156	```
   157	tests/system_keypair_generation.rs           — first-boot keypair generated; encrypted at rest; correct permissions
   158	tests/system_keypair_load_and_decrypt.rs     — second-boot load with correct password succeeds; wrong password fails
   159	tests/system_keypair_sign_only_from_runner.rs — static check: sign API not exported beyond predicate_runner + terminal_summary_emitter
   160	tests/system_keypair_verify_correctness.rs   — round-trip: sign then verify, with correct epoch pubkey lookup

exec
/bin/bash -lc 'rg -n "pub mod transition_ledger|transition_ledger" src/bottom_white/ledger/mod.rs genesis_payload.toml .' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
genesis_payload.toml:250:# 2026-04-28 Wave 6 #1 — CO1.7 transition_ledger spec v1 DRAFT outline.
genesis_payload.toml:284:"src/bottom_white/ledger/transition_ledger.rs" = "6a0aa95f269bdbb121e7d99478ae9ce8a964c877e3ce44b56e2b85bf2b24b534"
src/bottom_white/ledger/mod.rs:10:pub mod transition_ledger;
./tests/conformance_stubs.rs:71:#[test] #[ignore] fn chain_tape_l4_transition_ledger() { unimplemented!("CO1.7 — L4 12-field TransitionTx schema") }
./src/bottom_white/ledger/mod.rs:10:pub mod transition_ledger;
./src/state/q_state.rs:14://! CO1.7 transition_ledger) are intentionally minimal here — full schemas land per atom,
./src/state/q_state.rs:53:    /// Concrete derivation (commit-tree-of-state-root) lands in CO1.7 transition_ledger.
./genesis_payload.toml:250:# 2026-04-28 Wave 6 #1 — CO1.7 transition_ledger spec v1 DRAFT outline.
./genesis_payload.toml:284:"src/bottom_white/ledger/transition_ledger.rs" = "6a0aa95f269bdbb121e7d99478ae9ce8a964c877e3ce44b56e2b85bf2b24b534"
./rules/enforcement.log:294:[2026-04-28 07:10:15] WARN R-014 | /home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs | R-014 (FC3-S3 Trust Root): if this file IS in genesis_payload.toml [trust_root] (24 entries as of A0d), recompute sha256sum + update manifest + cargo test boot::verify_trust_root_passes_on_intact_repo. Otherwise next boot panics TRUST_ROOT_TAMPERED.
./rules/enforcement.log:295:[2026-04-28 07:10:15] WARN R-015 | /home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs | REMINDER (R-015 / Alignment Standard): edit touches a pub symbol in a Rust file. New OR modified pub items MUST carry `/// TRACE_MATRIX FC?-N?: <role>` doc-comment AND have an entry in handover/alignment/TRACE_MATRIX_v?.md (current: v1_2026-04-25). If this is genuinely orphan, file under TRACE_MATRIX § 3 with explicit Constitutional Justification (cases/Cxxx or PREREG-§n.m). Untraced pub symbols cause silent constitutional drift.
./rules/enforcement.log:296:[2026-04-28 07:10:45] WARN R-014 | /home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs | R-014 (FC3-S3 Trust Root): if this file IS in genesis_payload.toml [trust_root] (24 entries as of A0d), recompute sha256sum + update manifest + cargo test boot::verify_trust_root_passes_on_intact_repo. Otherwise next boot panics TRUST_ROOT_TAMPERED.
./rules/enforcement.log:297:[2026-04-28 07:10:49] WARN R-014 | /home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs | R-014 (FC3-S3 Trust Root): if this file IS in genesis_payload.toml [trust_root] (24 entries as of A0d), recompute sha256sum + update manifest + cargo test boot::verify_trust_root_passes_on_intact_repo. Otherwise next boot panics TRUST_ROOT_TAMPERED.
./rules/enforcement.log:298:[2026-04-28 07:10:49] WARN R-015 | /home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs | REMINDER (R-015 / Alignment Standard): edit touches a pub symbol in a Rust file. New OR modified pub items MUST carry `/// TRACE_MATRIX FC?-N?: <role>` doc-comment AND have an entry in handover/alignment/TRACE_MATRIX_v?.md (current: v1_2026-04-25). If this is genuinely orphan, file under TRACE_MATRIX § 3 with explicit Constitutional Justification (cases/Cxxx or PREREG-§n.m). Untraced pub symbols cause silent constitutional drift.
./rules/enforcement.log:299:[2026-04-28 07:10:54] WARN R-014 | /home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs | R-014 (FC3-S3 Trust Root): if this file IS in genesis_payload.toml [trust_root] (24 entries as of A0d), recompute sha256sum + update manifest + cargo test boot::verify_trust_root_passes_on_intact_repo. Otherwise next boot panics TRUST_ROOT_TAMPERED.
./rules/enforcement.log:300:[2026-04-28 07:10:54] WARN R-015 | /home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs | REMINDER (R-015 / Alignment Standard): edit touches a pub symbol in a Rust file. New OR modified pub items MUST carry `/// TRACE_MATRIX FC?-N?: <role>` doc-comment AND have an entry in handover/alignment/TRACE_MATRIX_v?.md (current: v1_2026-04-25). If this is genuinely orphan, file under TRACE_MATRIX § 3 with explicit Constitutional Justification (cases/Cxxx or PREREG-§n.m). Untraced pub symbols cause silent constitutional drift.
./rules/enforcement.log:303:[2026-04-28 07:11:46] WARN R-014 | /home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs | R-014 (FC3-S3 Trust Root): if this file IS in genesis_payload.toml [trust_root] (24 entries as of A0d), recompute sha256sum + update manifest + cargo test boot::verify_trust_root_passes_on_intact_repo. Otherwise next boot panics TRUST_ROOT_TAMPERED.
./rules/enforcement.log:304:[2026-04-28 07:11:46] WARN R-015 | /home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs | REMINDER (R-015 / Alignment Standard): edit touches a pub symbol in a Rust file. New OR modified pub items MUST carry `/// TRACE_MATRIX FC?-N?: <role>` doc-comment AND have an entry in handover/alignment/TRACE_MATRIX_v?.md (current: v1_2026-04-25). If this is genuinely orphan, file under TRACE_MATRIX § 3 with explicit Constitutional Justification (cases/Cxxx or PREREG-§n.m). Untraced pub symbols cause silent constitutional drift.
./handover/audits/run_gemini_co1_7_round1_audit.py:2:"""Gemini round-1 audit on CO1.7 transition_ledger spec v1 + type-skeleton.
./handover/audits/run_gemini_co1_7_round1_audit.py:28:This is a **round-1 dual external audit** on CO1.7 transition_ledger spec v1 + type-skeleton joint artifact. Codex is running an independent round-1 in parallel (implementer-review angle); your angle is **strategic / architectural / constitutional**.
./handover/audits/run_gemini_co1_7_round1_audit.py:108:    ("DOC: type-skeleton (joint artifact target)", "src/bottom_white/ledger/transition_ledger.rs"),
./handover/audits/run_gemini_co1_7_round1_audit.py:129:    f.write(f"# Gemini CO1.7 transition_ledger Round-1 Audit Run\n")
./handover/audits/GEMINI_WHITEPAPER_V2_AUDIT_2026-04-27_R2.md:53:**Reasoning**: The 6-layer structure of ChainTape is logically clean, with clear separation of concerns. The progression from rules (L1/L2) to data (L3) to history (L4) to current state (L5) to statistical summary (L6) is sound. There is no obvious overlap or missing layer. The explicit note in `v2 § 5.1 Layer 6` confirming that L6 is derivable from L4+L5 correctly addresses `constitution.md Art 0.2` compliance. The appendix (`v2 附录 A`) confirms that Layer 4 correctly subsumes the `CO1.7 transition_ledger` atom.
./handover/audits/GEMINI_CO1_7_ROUND1_AUDIT_2026-04-28.md:1:# Gemini CO1.7 transition_ledger Round-1 Audit Run
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:1:# Codex CO1.7 transition_ledger Round-1 Audit
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:21:# Codex Adversarial Audit — CO1.7 transition_ledger spec v1 + type-skeleton (Round 1)
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:32:2. **Type skeleton**: `src/bottom_white/ledger/transition_ledger.rs` (~370 lines incl. 6 inline tests, 196/0 lib PASS, TR-tracked)
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:67:**Q-F. STEP_B disposition**: spec § 9 claims "no STEP_B parallel-branch ceremony required" because transition_ledger.rs + sequencer.rs are NEW files. Verify against `STEP_B_PROTOCOL.md` + memory `feedback_step_b_protocol`. Are there indirect modifications (e.g., changes to bus.rs/kernel.rs/wal.rs that the sequencer integration WILL force later)?
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:83:**Q-I. Implementation gating**: assuming all your CHALLENGEs are addressed in v1.1, is the joint artifact implementable end-to-end (i.e., `cargo test --lib transition_ledger` will pass with stubs filled in by CO1.7.5+)? Specific blockers to call out.
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:119:**Single sentence**: implement the L4 transition_ledger module so that `ledger::append(parent_root, tx) → new_root` (called from § 3 transition pseudocode) is real code, the L4 sequencer (§ 5.2.1) is real code, and `Q_t.ledger_root_t` is no longer a placeholder.
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:143:- `src/ledger.rs` (legacy, top-level) is retired in **CO1.1.5 (kernel.rs split)**; CO1.7 lives at `src/bottom_white/ledger/transition_ledger.rs` (NEW). No STEP_B parallel-branch ceremony required (new module, not restricted file).
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:214:├── mod.rs                       (re-exports; existing — extends with `pub mod transition_ledger`)
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:216:└── transition_ledger.rs         (NEW, this atom)
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:224:**Crate boundary**: `transition_ledger` is in `bottom_white::ledger` because it is a tool layer (storage); `sequencer` is in `state::` because it touches Q_t mutation. Sequencer DEPENDS ON ledger; ledger does NOT depend on sequencer (DAG: state → bottom_white::ledger → CO1.4 CAS).
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:466:CO1.7 lives in NEW files (`src/bottom_white/ledger/transition_ledger.rs`, `src/state/sequencer.rs`). It does NOT modify `src/bus.rs` / `src/kernel.rs` / `src/wal.rs` (the STEP_B-restricted files). Therefore: **no STEP_B parallel-branch ceremony required**. Direct edit on `main` is per CLAUDE.md "Code Standard".
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:505:**Pre-implementation gate**: CO1.7 v1 must reach `PASS/PASS` from Codex + Gemini before any `src/bottom_white/ledger/transition_ledger.rs` or `src/state/sequencer.rs` code is written. Sedimented per CLAUDE.md "Audit Standard" (Generator ≠ Evaluator) + memory `feedback_dual_audit`.
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:535:| `cargo check` on `src/bottom_white/ledger/transition_ledger.rs` | PASS | LedgerEntry / TxKind / append / replay_chain_integrity / InMemoryLedgerWriter all type-check against existing `Cid` (CO1.4) + `SystemSignature`/`SystemEpoch` (CO1.7.0a-f) + `Hash` (Q_t) |
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:536:| `cargo test --lib bottom_white::ledger::transition_ledger::` | 6/6 PASS | append byte-stable; canonical_digest stable across clones; in-memory writer enforces logical_t monotonic; replay validates parent chain; replay rejects parent_state_root tamper; replay rejects ledger_root tamper |
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:537:| `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` | PASS post TR refresh | new file `transition_ledger.rs` + modified `mod.rs` added to `genesis_payload.toml [trust_root]` |
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:540:**Audit-ready artifact set**: spec v1 (this file) + skeleton (`src/bottom_white/ledger/transition_ledger.rs`, ~370 lines incl. 6 inline tests) + 5 cataloged divergences + 4 new round-1 audit Qs (Q8/Q9/Q10/Q11). Round-1 audit has both paper + code to inspect — higher signal density than spec-only review.
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:3716:/bin/bash -lc "rg --files -g 'AGENTS.md' -g 'CLAUDE.md' -g 'STEP_B_PROTOCOL.md' -g '*TRANSITION_LEDGER*' -g 'transition_ledger.rs' -g 'system_keypair.rs' -g 'store.rs' -g 'schema.rs'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:3721:src/bottom_white/ledger/transition_ledger.rs
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:3742:    12	**Single sentence**: implement the L4 transition_ledger module so that `ledger::append(parent_root, tx) → new_root` (called from § 3 transition pseudocode) is real code, the L4 sequencer (§ 5.2.1) is real code, and `Q_t.ledger_root_t` is no longer a placeholder.
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:3766:    36	- `src/ledger.rs` (legacy, top-level) is retired in **CO1.1.5 (kernel.rs split)**; CO1.7 lives at `src/bottom_white/ledger/transition_ledger.rs` (NEW). No STEP_B parallel-branch ceremony required (new module, not restricted file).
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:3837:   107	├── mod.rs                       (re-exports; existing — extends with `pub mod transition_ledger`)
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:3839:   109	└── transition_ledger.rs         (NEW, this atom)
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:3847:   117	**Crate boundary**: `transition_ledger` is in `bottom_white::ledger` because it is a tool layer (storage); `sequencer` is in `state::` because it touches Q_t mutation. Sequencer DEPENDS ON ledger; ledger does NOT depend on sequencer (DAG: state → bottom_white::ledger → CO1.4 CAS).
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:3993:/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:4866:   359	CO1.7 lives in NEW files (`src/bottom_white/ledger/transition_ledger.rs`, `src/state/sequencer.rs`). It does NOT modify `src/bus.rs` / `src/kernel.rs` / `src/wal.rs` (the STEP_B-restricted files). Therefore: **no STEP_B parallel-branch ceremony required**. Direct edit on `main` is per CLAUDE.md "Code Standard".
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:4905:   398	**Pre-implementation gate**: CO1.7 v1 must reach `PASS/PASS` from Codex + Gemini before any `src/bottom_white/ledger/transition_ledger.rs` or `src/state/sequencer.rs` code is written. Sedimented per CLAUDE.md "Audit Standard" (Generator ≠ Evaluator) + memory `feedback_dual_audit`.
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:4935:   428	| `cargo check` on `src/bottom_white/ledger/transition_ledger.rs` | PASS | LedgerEntry / TxKind / append / replay_chain_integrity / InMemoryLedgerWriter all type-check against existing `Cid` (CO1.4) + `SystemSignature`/`SystemEpoch` (CO1.7.0a-f) + `Hash` (Q_t) |
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:4936:   429	| `cargo test --lib bottom_white::ledger::transition_ledger::` | 6/6 PASS | append byte-stable; canonical_digest stable across clones; in-memory writer enforces logical_t monotonic; replay validates parent chain; replay rejects parent_state_root tamper; replay rejects ledger_root tamper |
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:4937:   430	| `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` | PASS post TR refresh | new file `transition_ledger.rs` + modified `mod.rs` added to `genesis_payload.toml [trust_root]` |
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:4940:   433	**Audit-ready artifact set**: spec v1 (this file) + skeleton (`src/bottom_white/ledger/transition_ledger.rs`, ~370 lines incl. 6 inline tests) + 5 cataloged divergences + 4 new round-1 audit Qs (Q8/Q9/Q10/Q11). Round-1 audit has both paper + code to inspect — higher signal density than spec-only review.
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:4945:/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '260,460p'" in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5499:/bin/bash -lc 'rg -n "STEP_B|restricted|bus.rs|kernel.rs|wal.rs|transition_ledger|src/state/sequencer|new files|conflict" handover/ai-direct/STEP_B_PROTOCOL.md CLAUDE.md' in /home/zephryj/projects/turingosv4
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5740:    14	//! CO1.7 transition_ledger) are intentionally minimal here — full schemas land per atom,
./handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5779:    53	    /// Concrete derivation (commit-tree-of-state-root) lands in CO1.7 transition_ledger.
./handover/audits/run_codex_co1_7_round1_audit.sh:2:# Codex round-1 audit on CO1.7 transition_ledger spec v1 + type-skeleton smoke artifact.
./handover/audits/run_codex_co1_7_round1_audit.sh:14:# Codex Adversarial Audit — CO1.7 transition_ledger spec v1 + type-skeleton (Round 1)
./handover/audits/run_codex_co1_7_round1_audit.sh:25:2. **Type skeleton**: `src/bottom_white/ledger/transition_ledger.rs` (~370 lines incl. 6 inline tests, 196/0 lib PASS, TR-tracked)
./handover/audits/run_codex_co1_7_round1_audit.sh:60:**Q-F. STEP_B disposition**: spec § 9 claims "no STEP_B parallel-branch ceremony required" because transition_ledger.rs + sequencer.rs are NEW files. Verify against `STEP_B_PROTOCOL.md` + memory `feedback_step_b_protocol`. Are there indirect modifications (e.g., changes to bus.rs/kernel.rs/wal.rs that the sequencer integration WILL force later)?
./handover/audits/run_codex_co1_7_round1_audit.sh:76:**Q-I. Implementation gating**: assuming all your CHALLENGEs are addressed in v1.1, is the joint artifact implementable end-to-end (i.e., `cargo test --lib transition_ledger` will pass with stubs filled in by CO1.7.5+)? Specific blockers to call out.
./handover/audits/run_codex_co1_7_round1_audit.sh:105:cat "${ROOT}/src/bottom_white/ledger/transition_ledger.rs" >> "$TMP_PROMPT"
./handover/audits/run_codex_co1_7_round1_audit.sh:123:  printf '# Codex CO1.7 transition_ledger Round-1 Audit\n'
./handover/audits/CODEX_WHITEPAPER_V2_AUDIT_2026-04-27.md:43:Layer 4 correctly points at the deferred transition-ledger atom: v2 defines Transition Ledger as an append-only state-transition ledger with tx fields (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:359-385`), and Appendix A maps §5.1 Layer 4 to CO1.7 transition_ledger (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:1327-1333`).
./handover/audits/GEMINI_WHITEPAPER_V2_AUDIT_2026-04-27.md:56:The 6-layer structure of ChainTape is logically clean and comprehensive. The progression from Layer 0 (Constitution Root) to Layer 6 (Signal Indices) follows a clear dependency path from trust anchor to derived analytics. There is no significant overlap between layers. Layer 4 (Transition Ledger) correctly serves as the append-only source of truth from which Layer 5 (Materialized State) and Layer 6 are derived. The appendix (`附录 A`) confirms that Layer 4 is intended to subsume the `CO1.7 transition_ledger` atom.
./handover/audits/GEMINI_WHITEPAPER_V2_AUDIT_2026-04-27_R3.md:62:- **Subsumption**: The appendix explicitly maps `§ 5.1 Layer 4 Transition Ledger` to the `CO1.7 transition_ledger` atom, confirming it correctly subsumes the planned work.
./handover/audits/AUDIT_LEDGER.md:86:| 2026-04-27 | **Whitepaper v2 ratification cumulative summary** | n/a | 3 rounds dual audit + 7 must-fix patches + 1 v2.2 patch + 4 OBS sediment files + tactical alignment note (governance § 9 + § 10 added). | est **~$20** total (R1 ~$8.50 + R2 ~$8.50 + R3 ~$3.50) | n/a | 9 doc files committed atomically; tests 246/246 baseline preserved (zero src/ touched). Cumulative project ~$100-150 of $890 (~11-17%). Wave 6 priorities re-confirmed under ChainTape lens: CO1.7 transition_ledger (L4) → CO1.1.4-pre1.b fixtures → INV8 spec v2 → STEP_B → F ceremonies. **Orphan finding**: `test_trust_root_simulated_write_aborts` at `experiments/minif2f_v4/tests/trust_root_immutability.rs:74` is pre-existing failure at clean HEAD `fb63053`; unchanged by v2 work; deserves separate Wave 6 sweep atom (likely test assertion outdated post-A8e13 boot.rs `Tampered` → `SectionMissing` enum split). |
./handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:139:| CO1.7.6 | Conformance test for L4 | `tests/chain_tape_L4_transition_ledger.rs` |
./handover/architect-insights/PROJECT_DECISION_MAP_2026-04-27.md:182:| Wave 5 (next) | TBD | A=D INV8 spike / B=CO1.7 transition_ledger / C=CO1.1.4-pre1 (V-01 kill) / D=CO1.1.4 bus split / E=CO1.1.5 kernel split / F=ceremonies | ⏳ user picks |
./handover/architect-insights/PROJECT_DECISION_MAP_2026-04-27.md:225:| **CO1.7 transition_ledger** | 🟢 **NEWLY UNBLOCKED** | needs CO1.1.4+1.1.5 + system_keypair (now done) |
./handover/architect-insights/PROJECT_DECISION_MAP_2026-04-27.md:443:> v4 has finished CO P0 + Wave 1-4 + Wave 5-C (V-01 ceremonial kill); 246/0 tests PASS; constitutional + WP chain ratified; spec round-4 PASS unblocks CO1.1.4 / 1.1.5 / 1.7 STEP_B; **Wave 5-A INV8 dual audit RESULT: Codex VETO / Gemini PASS → conservative VETO** (4 VETO: concurrent parent tie-break SILENT / multi-parent weight contradiction / assert_acyclic broken / not implement-ready). INV8 spec v2 revision required before CO P2.4 implementation. Wave 5-B CO1.7 transition_ledger NOT started this session (deferred Wave 6 with D/E STEP_B). Budget ~$70-110 of $890 cumulative.
./handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:12:**Single sentence**: implement the L4 transition_ledger module so that `ledger::append(parent_root, tx) → new_root` (called from § 3 transition pseudocode) is real code, the L4 sequencer (§ 5.2.1) is real code, and `Q_t.ledger_root_t` is no longer a placeholder.
./handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:36:- `src/ledger.rs` (legacy, top-level) is retired in **CO1.1.5 (kernel.rs split)**; CO1.7 lives at `src/bottom_white/ledger/transition_ledger.rs` (NEW). No STEP_B parallel-branch ceremony required (new module, not restricted file).
./handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:107:├── mod.rs                       (re-exports; existing — extends with `pub mod transition_ledger`)
./handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:109:└── transition_ledger.rs         (NEW, this atom)
./handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:117:**Crate boundary**: `transition_ledger` is in `bottom_white::ledger` because it is a tool layer (storage); `sequencer` is in `state::` because it touches Q_t mutation. Sequencer DEPENDS ON ledger; ledger does NOT depend on sequencer (DAG: state → bottom_white::ledger → CO1.4 CAS).
./handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:359:CO1.7 lives in NEW files (`src/bottom_white/ledger/transition_ledger.rs`, `src/state/sequencer.rs`). It does NOT modify `src/bus.rs` / `src/kernel.rs` / `src/wal.rs` (the STEP_B-restricted files). Therefore: **no STEP_B parallel-branch ceremony required**. Direct edit on `main` is per CLAUDE.md "Code Standard".
./handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:398:**Pre-implementation gate**: CO1.7 v1 must reach `PASS/PASS` from Codex + Gemini before any `src/bottom_white/ledger/transition_ledger.rs` or `src/state/sequencer.rs` code is written. Sedimented per CLAUDE.md "Audit Standard" (Generator ≠ Evaluator) + memory `feedback_dual_audit`.
./handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:428:| `cargo check` on `src/bottom_white/ledger/transition_ledger.rs` | PASS | LedgerEntry / TxKind / append / replay_chain_integrity / InMemoryLedgerWriter all type-check against existing `Cid` (CO1.4) + `SystemSignature`/`SystemEpoch` (CO1.7.0a-f) + `Hash` (Q_t) |
./handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:429:| `cargo test --lib bottom_white::ledger::transition_ledger::` | 6/6 PASS | append byte-stable; canonical_digest stable across clones; in-memory writer enforces logical_t monotonic; replay validates parent chain; replay rejects parent_state_root tamper; replay rejects ledger_root tamper |
./handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:430:| `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` | PASS post TR refresh | new file `transition_ledger.rs` + modified `mod.rs` added to `genesis_payload.toml [trust_root]` |
./handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:433:**Audit-ready artifact set**: spec v1 (this file) + skeleton (`src/bottom_white/ledger/transition_ledger.rs`, ~370 lines incl. 6 inline tests) + 5 cataloged divergences + 4 new round-1 audit Qs (Q8/Q9/Q10/Q11). Round-1 audit has both paper + code to inspect — higher signal density than spec-only review.
./handover/ai-direct/LATEST.md:30:1. **CO1.7 transition_ledger** (Layer 4) — promoted: central artifact connecting agents → state
./handover/ai-direct/LATEST.md:59:**Updated**: 2026-04-28 — **Wave 6 #1 spec landed** (`330f239`): CO1.7 transition_ledger v1 DRAFT outline (~330 lines, 14 sections, 7 open questions for round-1 audit). + harness hygiene closeout (`c40b891`): backbone-language unification (NOTEPAD F-2026-04-26-01 RESOLVED + HANDOVER § 3 banner) + new memory `feedback_session_label_codification` (P0-loss prevention).
./handover/ai-direct/LATEST.md:63:**Next-session entry**: 🔥 **CO1.7 spec v1 round-1 dual audit** — gate is PASS/PASS before any `src/bottom_white/ledger/transition_ledger.rs` or `src/state/sequencer.rs` code. Spec at `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md`. Cost estimate per § 12: ~$5-15 round-1 (system_keypair was 1 round PASS; spec v1.4 took 4 rounds; CO1.7 size between). Working tree clean.
./handover/ai-direct/LATEST.md:69:> **Two parallel tracks** (re-confirmed): **CO refactor** (kernel architectural rewrite) and **PPUT-CCL experiment** (real minif2f benchmark on heldout-49). Per PREREG, neither blocks the other; CO1.7 transition_ledger does NOT block minif2f experiment runs.
./handover/ai-direct/LATEST.md:75:| **ChainTape (L0–L6)** | **48%** | L0 Trust Anchor 95% (待 ratification 签名) / L3 CAS 90% / L1 PredicateRegistry 60% / L2 ToolRegistry 50% | L4 transition_ledger **10%** (spec v1.4 PASS, code = CO1.7 未起草) → 直接卡 L5/L6 |
./handover/ai-direct/LATEST.md:79:### Single-point bottleneck: **CO1.7 transition_ledger**
./handover/ai-direct/LATEST.md:116:- 🔄 **5-B CO1.7 transition_ledger** (large atom; deserves dedicated session)
./handover/ai-direct/LATEST.md:135:- CO1.7 transition_ledger
./handover/ai-direct/LATEST.md:150:- CO1.7 transition_ledger
./handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:218:tests/chain_tape_L4_transition_ledger.rs
./handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:1333:> 例如：CO1.7 transition_ledger atom 在 Plan v3.2 中保持原 CO-id 不变，但在 § 17 narrative 中归入 **Phase 2 LedgerTape**。这是**标签**关系，不是**重命名**关系。
./handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:1370:| § 5.1 Layer 4 Transition Ledger | CO1.7 transition_ledger | 🔄 Wave 6 计划 |
./handover/alignment/OBS_WHITEPAPER_V2_QT_FIVE_ROOT_EXTENSION_2026-04-27.md:62:| `ledger_root_t` | placeholder | Layer 4 transition_ledger (CO1.7 — Wave 6) |
./handover/alignment/OBS_WHITEPAPER_V2_QT_FIVE_ROOT_EXTENSION_2026-04-27.md:74:2. **CO1.7 transition_ledger** (Wave 6) — `ledger_root_t` wired to actual append
./handover/alignment/WHITEPAPER_v2_TACTICAL_ALIGNMENT_2026-04-27.md:77:2. CO1.7 transition_ledger
./handover/alignment/WHITEPAPER_v2_TACTICAL_ALIGNMENT_2026-04-27.md:84:1. **CO1.7 transition_ledger** (Layer 4) — **promoted** because v2 makes Layer 4 the central artifact connecting agents to state
./handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:66:| § 5.L4 Transition Ledger | 12 fields | N | `bottom_white::ledger::transition::TransitionTx` (12 fields incl task_id) | `tests/chain_tape_L4_transition_ledger.rs` + `tests/transition_tx_12_fields.rs` | CO1.7 |
./handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:189:tests/chain_tape_L4_transition_ledger.rs

codex
The QState file supports DIV-2 in the narrow sense: the economic indices are typed BTreeMap shells, not transition-ready domain APIs. I’m going to run the narrow transition-ledger test target once so the verdict can separate smoke coverage from spec adequacy.
exec
/bin/bash -lc 'cargo test --lib bottom_white::ledger::transition_ledger::' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'cargo check' in /home/zephryj/projects/turingosv4
 succeeded in 403ms:
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
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
 --> src/bottom_white/cas/store.rs:9:12
  |
9 | use git2::{ObjectType as Git2ObjectType, Repository};
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
   --> src/bottom_white/ledger/system_keypair.rs:492:19
    |
492 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_terminal_summary_tx` is never used
   --> src/bottom_white/ledger/system_keypair.rs:508:19
    |
508 |     pub(crate) fn sign_terminal_summary_tx(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:527:19
    |
527 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib test) generated 13 warnings (run `cargo fix --lib -p turingosv4 --tests` to apply 10 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-128d20df5fbbc885)

running 6 tests
test bottom_white::ledger::transition_ledger::tests::append_is_pure_and_byte_stable ... ok
test bottom_white::ledger::transition_ledger::tests::in_memory_writer_enforces_logical_t ... ok
test bottom_white::ledger::transition_ledger::tests::canonical_digest_byte_stable_across_clones ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_mismatch ... ok
test bottom_white::ledger::transition_ledger::tests::replay_validates_parent_chain ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 190 filtered out; finished in 0.00s


 succeeded in 2244ms:
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on build directory
    Checking turingosv4 v0.1.0 (/home/zephryj/projects/turingosv4)
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
 --> src/bottom_white/cas/store.rs:9:12
  |
9 | use git2::{ObjectType as Git2ObjectType, Repository};
  |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `sign_digest` is never used
   --> src/bottom_white/ledger/system_keypair.rs:287:8
    |
232 | impl Ed25519Keypair {
    | ------------------- method in this implementation
...
287 |     fn sign_digest(&self, digest: [u8; 32]) -> Result<SystemSignature, KeypairError> {
    |        ^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message_inner` is never used
   --> src/bottom_white/ledger/system_keypair.rs:535:4
    |
535 | fn sign_system_message_inner(
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_rejected_attempt_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:481:19
    |
481 |     pub(crate) fn sign_rejected_attempt_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:492:19
    |
492 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: function `sign_terminal_summary_tx` is never used
   --> src/bottom_white/ledger/system_keypair.rs:508:19
    |
508 |     pub(crate) fn sign_terminal_summary_tx(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_epoch_rotation_proof` is never used
   --> src/bottom_white/ledger/system_keypair.rs:516:19
    |
516 |     pub(crate) fn sign_epoch_rotation_proof(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:527:19
    |
527 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib) generated 17 warnings (run `cargo fix --lib -p turingosv4` to apply 10 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.18s

exec
/bin/bash -lc 'rg --files tests | sort' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/conformance_stubs.rs
tests/economic_state_reconstruct.rs
tests/fc_alignment_conformance.rs
tests/four_element_mapping.rs
tests/harness_validation.py
tests/harness_validation.sh
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

exec
/bin/bash -lc "nl -ba handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md | sed -n '291,325p'" in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "nl -ba handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md | sed -n '863,895p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   291	## § 2.5 Canonical Serialization (v1.2 NEW per Codex Q5)
   292	
   293	> **Required because**: `tx.canonical_digest()` is called in spec § 3 stages 2 of WorkTx / VerifyTx / ChallengeTx, but byte-level format is undefined. STEP_B branch A vs branch B may pick different serialization (JSON sorted keys vs bincode vs Rust derive order) → cross-branch signature verification fails. Mandatory canonical format closes this.
   294	
   295	**Format**: **bincode v2** (`bincode::serde`) with the following constraints:
   296	- **Big-endian byte order** for all multi-byte integers (network order; deterministic across platforms)
   297	- **`BTreeMap` keys serialized in lexicographic byte order** (this is bincode default; verified by test)
   298	- **Strings serialized as UTF-8 with explicit length prefix u32-BE**
   299	- **Optional fields: `0x00` prefix for `None`, `0x01` + value for `Some`**
   300	- **Enum discriminant: u8 (variant index in declaration order)**
   301	- **No padding bytes; no implicit alignment**
   302	
   303	**Application**:
   304	```rust
   305	pub fn canonical_digest<T: Serialize>(value: &T) -> [u8; 32] {
   306	    let bytes = bincode::serde::encode_to_vec(value, bincode_canonical_config()).expect("serialize");
   307	    sha256(&bytes)
   308	}
   309	
   310	fn bincode_canonical_config() -> bincode::config::Configuration {
   311	    bincode::config::standard()
   312	        .with_big_endian()
   313	        .with_fixed_int_encoding()    // no varint; fixed-width for determinism
   314	}
   315	```
   316	
   317	**Conformance**: `tests/canonical_serialization.rs` MUST verify:
   318	- 1 golden tx fixture per tx type (WorkTx / VerifyTx / ChallengeTx / ReuseTx / TerminalSummaryTx); each has known input → known SHA-256 output
   319	- Round-trip: `decode(encode(x)) == x` byte-identical for 100 random inputs
   320	- Stability: 2 independent runs on same input → same bytes
   321	
   322	**STEP_B implication**: branches A and B both use this exact `bincode_canonical_config`; signature verification works cross-branch by construction.
   323	
   324	**Out of scope for v1.x spec** (deferred per Codex Q5/NEW-5 round-3 PARTIAL acknowledgment): full golden fixture corpus + differential fuzzing seed + complete runner ABI for QState/SignalBundle/TransitionError. v1.4 freezes the SERIALIZATION RULE (bincode v2 big-endian + BTreeMap lex); fixtures + ABI land in **CO1.1.4-pre1** (canonical fixture corpus) + **CO1.7** (full ABI surface). This is an **explicit deferral** — not unresolved spec ambiguity. STEP_B branch A and branch B both implement the SAME bincode rule; per-tx digest matching is mechanical from v1.4. Full corpus generation is a downstream code task, not spec scope.
   325	

 succeeded in 0ms:
   863	## § 4 Named Invariants (machine-checkable)
   864	
   865	| ID | Invariant | Enforced at | Conformance test |
   866	|---|---|---|---|
   867	| I-DET | Same (Q_t, tx) → byte-identical (Q_{t+1}, signals) | step_transition stage 6-8 | `tests/transition_determinism.rs` |
   868	| I-DETHASH | `state_root_t` after replay from genesis matches authoritative state | replay test | `tests/q_state_reconstruct.rs` |
   869	| I-NOSIDE | step_transition reads only (q, tx, registries); no I/O | static analysis grep + cargo-deny | `tests/no_hidden_inputs.rs` |
   870	| I-PARENT | tx.parent_state_root must equal q.state_root_t | stage 1 | `tests/stale_parent_rejection.rs` |
   871	| I-SIG | tx.signature verifies against tx.canonical_digest() | stage 2 | `tests/signature_verification.rs` |
   872	| I-STAKE | tx.stake ≤ q.balances_t[tx.agent_id]; debit atomic | stage 3, 6 | `tests/stake_atomicity.rs` |
   873	| I-PRED-GATE | rejected work_tx does NOT advance state_root_t | stage 4 | `tests/economic_invariant_INV6_predicate_gated.rs` |
   874	| I-PROV | accepted work_tx → provisional claim, NOT finalized reward | stage 5 | `tests/economic_invariant_INV7_provisional_then_final.rs` |
   875	| I-LOGTIME | timestamp_logical strictly monotonic per-tx; no wall clock | stage 6 | `tests/no_wall_clock_in_tx.rs` |
   876	| I-MICROCOIN | all monetary fields are MicroCoin (i64 newtype) | type system | compile-time + `tests/no_f64_money.rs` |
   877	| I-BTREE | Q_t indices use BTreeMap, not HashMap (deterministic order) | type system | `tests/q_state_uses_btree.rs` |
   878	| I-NOSIDECAR | no Vec/HashMap "graveyard"-like sidecar (Art. 0.2) | static analysis | `tests/no_rejection_sidecar.rs` |
   879	| I-RETRY | RejectedAttemptSummary stamped by runner, not agent | stamp call site | `tests/retry_summary_runner_signed.rs` |
   880	| I-TERMINAL | every run terminates with at least one of: accepted work_tx OR TerminalSummaryTx | run finalize hook | `tests/run_terminal_invariant.rs` |
   881	| I-NOENV | step_transition dependency tree contains no `std::env` access | cargo-deny + grep | `tests/no_env_in_transition.rs` |
   882	| I-FREEZE-CONFIG | TAPE_ECONOMY_V2 + FOUNDER_GRANT_GAMMA + system_lp_amount frozen at task creation, not at tx submission | TaskMarket::publish | `tests/task_config_frozen_at_publish.rs` |
   883	| **I-NORANDOM** (added per Gemini v3.2 review Q1) | Any tx that consumes randomness MUST seed PRNG from `(tx.tx_id, q.state_root_t)`; no system entropy in step_transition path | step_transition stages 1-7 | `tests/no_runtime_entropy.rs` |
   884	| **I-VERIFY-LIVE** (added per Gemini v3.2 review Q10) | VerifyTx targets MUST be in Pending or Provisional state; cannot verify Accepted-and-finalized or Slashed | verify_transition stage 1 | `tests/verify_target_liveness.rs` |
   885	| **I-CHAL-WINDOW** (added per Gemini v3.2 review Q10) | ChallengeTx must be received within target's challenge_cases_t window; no challenges after window close | challenge_transition stage 1 | `tests/challenge_window_enforced.rs` |
   886	| **I-FINALIZE-EXCLUSIVE** (added) | FinalizeRewardTx and SlashTx are mutually exclusive per claim_id; system runtime serializes | finalize_reward_transition stage 2 | `tests/finalize_or_slash_exclusive.rs` |
   887	| **I-VBOND-RELEASE** (v1.1, gap 11.2 fix) | Verifier bond release on slashed work_tx follows TaskMarket.config.verifier_bond_on_slash policy; default = `ReturnToVerifier`; verifier reputation NOT adjusted under default policy | challenge_transition stage 4e | `tests/verifier_bond_release.rs` |
   888	| **I-ROYALTY-CAP** (v1.1, gap 11.3 fix) | reuse_tx edge weight ≤ TaskMarket.config.max_reuse_royalty_fraction (default 0.10); excess clamped + warning logged | reuse_transition stage 3 | `tests/royalty_cap_enforced.rs` |
   889	| **I-STAKE-RETURN** (v1.2 NEW per Gemini Q2 + Codex Q2) | Successful unchallenged finalize_reward returns + unlocks solver's locked stake exactly once (in addition to reward credit). Test attempts double-claim. | finalize_reward_transition stage 3a | `tests/stake_return_on_finalize.rs` |
   890	| **I-BOUNTY-REFUND** (v1.2 NEW per Gemini Q2 + Codex Q2) | task_expire_transition refunds full bounty to creator + refunds any locked solver stakes when no claim finalized by deadline | task_expire_transition stages 2-3 | `tests/bounty_refund_on_expire.rs` |
   891	| **I-FINALIZE-BATCH-ORDER** (v1.3 corrected: single key throughout) | When N claims become finalizable at the same logical_t, finalize_tx emit order is `(expires_at_logical ASC, claim_id ASC)` — `claim_id` (NOT `target_work_tx`) used everywhere: invariant + § 5.2.3 + conformance test all consistent. | runtime finalize loop + § 5.2.3 | `tests/finalize_batch_order.rs` |
   892	| **I-CHALLENGE-WINDOW-EDGE** (v1.3 finalize binding fixed) | Challenge window is `[opens_at, opens_at + duration_ticks)` — left-inclusive, right-exclusive. `is_open(now)` defined as `now < opens_at + duration_ticks`. **Both** challenge_transition stage 1 AND finalize_reward stage 1 MUST use `is_open(q.q_t.current_round)` (NOT a different rule). | challenge_transition + finalize_reward_transition | `tests/challenge_window_edge.rs` |
   893	| **I-AGENT-INIT** (v1.2 NEW per Gemini Q2) | First appearance of agent in L4 transition tx implicitly initializes q_t.agents[id] with reputation=0; subsequent appearances do not re-initialize | work/verify/challenge/reuse_transition stage 4 | `tests/agent_implicit_init.rs` |
   894	
   895	**Total: 27 invariants → 27 tests** (was 22 in v1.1; +5 in v1.2). Every transition test must pass before CO1.1.4 (bus.rs split) starts. STEP_B implementation comparison is "branch X conforms to spec" / "branch Y conforms to spec", not "branch X looks like branch Y".

exec
/bin/bash -lc "nl -ba handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md | sed -n '948,1016p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   948	**Why**: spec § 3 pseudocode is single-threaded; CO P1 may parallelize Phase C 5 modes × N seeds. Without explicit serialization rule, two work_tx can race on same parent_state_root, both pass `I-PARENT`, but produce different (logical_t, tx_id) sequences across STEP_B branches → state_root divergence.
   949	
   950	### 5.2.1 L4 Sequencer
   951	
   952	**Per (runtime_repo, run_id)** there is exactly ONE L4 sequencer instance. The sequencer:
   953	1. **Receives** tx submissions in any order (concurrent-safe queue)
   954	2. **Assigns** monotonic `(logical_t, tx_id)` ordering key:
   955	   - `logical_t = sequencer.next_logical_t()` (atomic counter; starts at 1 per genesis)
   956	   - `tx_id = TxId::derive(logical_t, agent_id, payload_hash)` (deterministic from above)
   957	3. **Serializes** transition execution: takes 1 tx at a time from queue in submission order; calls pure `step_transition`
   958	4. **Commits** result to L4 (WAL write + git commit) BEFORE accepting next tx
   959	
   960	**Async completion order is NEVER an ordering source**. Even if async tasks finish out-of-order, sequencer enforces submission-order ingestion.
   961	
   962	### 5.2.2 Cross-Cell Isolation
   963	
   964	**Phase C 5-mode × 10-problem × N-seed cells** (per `CO1_3_1_GIX_SPIKE_PREFLIGHT § 1` C4) MUST use:
   965	- **Disjoint `runtime_repo`** (different filesystem path; no shared state)
   966	- **Disjoint `QState`** (each cell has its own genesis_payload + Q_t replay)
   967	- **No shared L4 sequencer** (each cell has its own)
   968	
   969	If a future deployment shares runtime_repo across cells (e.g., multi-tenant): MUST add **ref locks** (gix branch refs serve as atomic guards) + **deterministic retry semantics** (failed lock → wait 100ms × n_attempts; deterministic seed from `(run_id, tx_id)`).
   970	
   971	### 5.2.3 Finalize Batch Order
   972	
   973	When N claims expire at the same `logical_t`:
   974	- Order = `(claim.expires_at_logical ASC, claim.claim_id ASC)` (stable, deterministic) — v1.3 fix: uses `claim_id` consistently (NOT `target_work_tx`) to align with `I-FINALIZE-BATCH-ORDER` invariant + conformance test
   975	- Sequencer emits `finalize_reward_transition` ONE AT A TIME in this order
   976	- Each finalize advances state_root before next finalize starts
   977	
   978	### 5.2.4 Conformance Tests
   979	
   980	- `tests/l4_sequencer_serialization.rs` — concurrent submit; assert single-threaded execution by sequencer; same input order → same state_root
   981	- `tests/cross_cell_isolation.rs` — 5 cells run; assert disjoint state_roots; no cross-contamination
   982	- `tests/finalize_batch_order.rs` — 3 claims expire same tick; assert ordering by (expires_at, claim_id); 2 runs byte-identical
   983	
   984	### 5.2.5 ChallengeWindow::is_open (v1.4 NEW per Codex Q2.4)
   985	
   986	```rust
   987	impl ChallengeWindow {
   988	    /// Half-open interval `[opens_at, opens_at + duration_ticks)`.
   989	    /// Both challenge_transition stage 1 AND finalize_reward stage 1 MUST invoke this method
   990	    /// (NOT hand-code the inequality) to guarantee consistent edge semantics.
   991	    pub fn is_open(&self, now: u64) -> bool {
   992	        now >= self.opens_at && now < self.opens_at + self.duration_ticks
   993	    }
   994	}
   995	```
   996	
   997	**Invariant binding**: `I-CHALLENGE-WINDOW-EDGE` enforces that BOTH transition functions call `is_open(now)` rather than hand-coding the boundary check. STEP_B branch A vs branch B both implement the same `is_open()`; cross-branch comparison verifies identical results for all (opens_at, duration_ticks, now) triples.
   998	
   999	### 5.2.6 Sequencer Tie-Break (v1.4 NEW per Codex Q6)
  1000	
  1001	When multiple agent threads concurrently call sequencer's `submit(tx)`, the sequencer's atomic `next_logical_t()` (§ 5.2.1 step 2) provides the **canonical tie-breaker**:
  1002	
  1003	- `logical_t` assignments are produced by atomic increment (e.g., `AtomicU64::fetch_add`)
  1004	- The order in which threads receive their `logical_t` values IS the canonical ordering
  1005	- "Submission order" = the order of `logical_t` assignment, NOT wall-clock arrival order
  1006	- For two `tx` arriving at the same nanosecond on different threads, whichever thread wins the atomic gets the lower `logical_t`; the other gets the next higher
  1007	
  1008	This means: STEP_B branch A and branch B may serialize threads differently (depending on OS scheduler), but as long as both branches use atomic logical_t assignment + replay from the SAME logical_t sequence, they produce byte-identical state_roots.
  1009	
  1010	**Conformance test addition** (extends `tests/l4_sequencer_serialization.rs`): submit 100 tx concurrently from 8 threads; assert `(logical_t, tx_id_hash)` is a strict total order; replay produces deterministic state_root regardless of thread interleaving.
  1011	
  1012	### 5.2.7 What This Does NOT Specify
  1013	
  1014	- Async runtime choice (tokio vs std::thread): runtime concern, not spec; spec only requires sequencer property
  1015	- Sequencer implementation: lock-free queue, mutex, channel — implementation detail
  1016	- Cross-cell sharing pattern (post-v4): future v4.x extension

exec
/bin/bash -lc "nl -ba tests/conformance_stubs.rs | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! Conformance Test Stubs (CO P1 preparation)
     2	//!
     3	//! Per `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md` § H: this file declares
     4	//! every conformance test path required for v4 ship as `#[ignore]`d stubs. They
     5	//! compile but are skipped by default (`cargo test` ignores them; `cargo test --
     6	//! --include-ignored` runs them but each `unimplemented!()` panics).
     7	//!
     8	//! As CO P1 + CO P2 atoms land, each stub here is REPLACED by a real test in
     9	//! its own dedicated `tests/<name>.rs` file (or expanded inline here). The stub
    10	//! is the contract: by v4 ship, all 80+ stubs must be replaced + each replacing
    11	//! test must PASS without `#[ignore]`.
    12	//!
    13	//! Tracking: `cargo test --tests -- --list 2>&1 | grep -c conformance_stub`
    14	//! gives current stub count; v4 ship gate = 0 remaining stubs.
    15	//!
    16	//! Authority: TRACE_MATRIX_v3 § H + STATE_TRANSITION_SPEC_v1 § 4.
    17	
    18	// =============================================================================
    19	// Anti-Oreo + Q_t + Tape Canonical (CO1.1, CO1.2, CO1.5-1.9)
    20	// =============================================================================
    21	
    22	#[test] #[ignore = "stub: replace at CO1.1.6"]
    23	fn anti_oreo_layer_audit() { unimplemented!("CO1.1.6 — verify no cross-layer imports between top_white/middle_black/bottom_white/economy") }
    24	
    25	#[test] #[ignore = "stub: replace at CO1.2.4"]
    26	fn q_state_reconstruct() { unimplemented!("CO1.2.4 — replay tape from genesis; assert byte-identical QState") }
    27	
    28	#[test] #[ignore = "stub: replace at CO1.2.4"]
    29	fn economic_state_reconstruct() { unimplemented!("CO1.2.4 — replay; assert byte-identical EconomicState 9 sub-fields") }
    30	
    31	#[test] #[ignore = "stub: replace at CO1.0"]
    32	fn four_element_mapping() { unimplemented!("CO1.0 — Const Art 0.1 four-element mapping (tape/control/memory/alphabet) → code symbols") }
    33	
    34	#[test] #[ignore = "stub: replace at CO1.0"]
    35	fn turing_fundamentalism() { unimplemented!("CO1.0 — Const Art 0 four-element grounding") }
    36	
    37	// 24 tape canonical V-violations
    38	#[test] #[ignore] fn tape_canonical_v01_completion_tokens() { unimplemented!() }
    39	#[test] #[ignore] fn tape_canonical_v02_runcost_accumulator() { unimplemented!() }
    40	#[test] #[ignore] fn tape_canonical_v03_wallet_state() { unimplemented!() }
    41	#[test] #[ignore] fn tape_canonical_v04_payload_byte_hack() { unimplemented!() }
    42	#[test] #[ignore] fn tape_canonical_v05_event_detail_none() { unimplemented!() }
    43	#[test] #[ignore] fn tape_canonical_v06_runwallclock() { unimplemented!() }
    44	#[test] #[ignore] fn tape_canonical_v07_search_cache() { unimplemented!() }
    45	#[test] #[ignore] fn tape_canonical_v08_librarian_board() { unimplemented!() }
    46	#[test] #[ignore] fn tape_canonical_v09_fc_trace() { unimplemented!() }
    47	#[test] #[ignore] fn tape_canonical_v10_market_create_emit() { unimplemented!() }
    48	#[test] #[ignore] fn tape_canonical_v11_market_resolve_emit() { unimplemented!() }
    49	#[test] #[ignore] fn tape_canonical_v12_lean_error_drop() { unimplemented!() }
    50	#[test] #[ignore] fn tape_canonical_v13_boltzmann_provenance() { unimplemented!() }
    51	#[test] #[ignore] fn tape_canonical_v14_mr_tick_provenance() { unimplemented!() }
    52	#[test] #[ignore] fn tape_canonical_v15_wal_optin() { unimplemented!() }
    53	#[test] #[ignore] fn tape_canonical_v16_wal_no_hashchain() { unimplemented!() }
    54	#[test] #[ignore] fn tape_canonical_v17_node_no_hash_field() { unimplemented!() }
    55	#[test] #[ignore] fn tape_canonical_v18_mr_tick_to_stderr() { unimplemented!() }
    56	#[test] #[ignore] fn tape_canonical_v19_graveyard_sidecar() { unimplemented!() }
    57	#[test] #[ignore] fn tape_canonical_v20_stale_view_window() { unimplemented!() }
    58	#[test] #[ignore] fn tape_canonical_v21_settle_provisional() { unimplemented!() }
    59	#[test] #[ignore] fn tape_canonical_v22_reputation_alias() { unimplemented!() }
    60	#[test] #[ignore] fn tape_canonical_v23_economy_default_off() { unimplemented!() }
    61	#[test] #[ignore] fn tape_canonical_v24_audit_guard_provenance() { unimplemented!() }
    62	
    63	// =============================================================================
    64	// ChainTape layers (CO1.0-1.9)
    65	// =============================================================================
    66	
    67	#[test] #[ignore] fn chain_tape_l0_constitution_root() { unimplemented!("CO1.0 — L0 constitution_root verifies all 8 fields") }
    68	#[test] #[ignore] fn chain_tape_l1_predicate_registry() { unimplemented!("CO1.5 — L1 predicate_id + visibility schema") }
    69	#[test] #[ignore] fn chain_tape_l2_tool_registry() { unimplemented!("CO1.6 — L2 capability + permission classification") }
    70	#[test] #[ignore] fn chain_tape_l3_cas() { unimplemented!("CO1.4 — L3 CAS object schema + retrieval") }
    71	#[test] #[ignore] fn chain_tape_l4_transition_ledger() { unimplemented!("CO1.7 — L4 12-field TransitionTx schema") }
    72	#[test] #[ignore] fn chain_tape_l5_materialized_state() { unimplemented!("CO1.8 — L5 materialized state DB + indices") }
    73	#[test] #[ignore] fn chain_tape_l6_signal_indices() { unimplemented!("CO1.9 — L6 boolean + statistical indices") }
    74	
    75	// =============================================================================
    76	// State transition spec invariants I-1..I-22 (per STATE_TRANSITION_SPEC v1.1)
    77	// =============================================================================
    78	
    79	#[test] #[ignore] fn transition_determinism() { unimplemented!("I-DET; CO1.SPEC.0.6") }
    80	#[test] #[ignore] fn no_hidden_inputs() { unimplemented!("I-NOSIDE; CO1.SPEC.0.6 + grep audit") }
    81	#[test] #[ignore] fn stale_parent_rejection() { unimplemented!("I-PARENT; CO1.7.5 stage 1") }
    82	#[test] #[ignore] fn signature_verification() { unimplemented!("I-SIG; CO1.7.5 stage 2") }
    83	#[test] #[ignore] fn stake_atomicity() { unimplemented!("I-STAKE; CO1.7.5 stage 3+6") }
    84	#[test] #[ignore] fn no_wall_clock_in_tx() { unimplemented!("I-LOGTIME; CO1.7.5 stage 6") }
    85	#[test] #[ignore] fn no_f64_money() { unimplemented!("I-MICROCOIN; CO P2.0a") }
    86	#[test] #[ignore] fn q_state_uses_btree() { unimplemented!("I-BTREE; CO1.2") }
    87	#[test] #[ignore] fn no_rejection_sidecar() { unimplemented!("I-NOSIDECAR; static analysis post-CO1.1.4-pre1") }
    88	#[test] #[ignore] fn retry_summary_runner_signed() { unimplemented!("I-RETRY; CO1.7.0") }
    89	#[test] #[ignore] fn run_terminal_invariant() { unimplemented!("I-TERMINAL; CO1.7.0 + runtime hook") }
    90	#[test] #[ignore] fn no_env_in_transition() { unimplemented!("I-NOENV; cargo-deny rule") }
    91	#[test] #[ignore] fn task_config_frozen_at_publish() { unimplemented!("I-FREEZE-CONFIG; CO P2.1") }
    92	#[test] #[ignore] fn no_runtime_entropy() { unimplemented!("I-NORANDOM; CO1.7.5 + cargo-deny") }
    93	#[test] #[ignore] fn verify_target_liveness() { unimplemented!("I-VERIFY-LIVE; verify_transition stage 1") }
    94	#[test] #[ignore] fn challenge_window_enforced() { unimplemented!("I-CHAL-WINDOW; challenge_transition stage 1") }
    95	#[test] #[ignore] fn finalize_or_slash_exclusive() { unimplemented!("I-FINALIZE-EXCLUSIVE; finalize_reward_transition stage 2") }
    96	#[test] #[ignore] fn verifier_bond_release() { unimplemented!("I-VBOND-RELEASE (v1.1); challenge_transition stage 4e; default = ReturnToVerifier") }
    97	#[test] #[ignore] fn royalty_cap_enforced() { unimplemented!("I-ROYALTY-CAP (v1.1); reuse_transition stage 3; default = 0.10") }
    98	
    99	// =============================================================================
   100	// Genesis (CO1.0)
   101	// =============================================================================
   102	
   103	#[test] #[ignore] fn genesis_constitution_root_verify() { unimplemented!("CO1.0.4 — boot::verify_constitution_root passes 5 sub-checks") }
   104	#[test] #[ignore] fn genesis_amendment_predicate_resolves() { unimplemented!("CO1.0.4 — amendment_predicate_hash exists in L3 CAS") }
   105	#[test] #[ignore] fn genesis_initial_registry_empty() { unimplemented!("CO1.0.4 — initial registries = EMPTY_TREE_ROOT for v4 genesis") }
   106	#[test] #[ignore] fn genesis_boot_attestation_self_referential() { unimplemented!("CO1.0.4 — self-referential hash matches recompute") }
   107	#[test] #[ignore] fn genesis_creator_signature_verifies() { unimplemented!("CO1.0.4 — gretjia PGP/SSH sig over constitution.md verifies") }
   108	
   109	// =============================================================================
   110	// Predicates + Visibility (CO1.5, CO1.11)
   111	// =============================================================================
   112	
   113	#[test] #[ignore] fn safety_creation_dichotomy() { unimplemented!("CO1.11; § 7.2 fail-policy") }
   114	#[test] #[ignore] fn private_predicate_error_no_leak() { unimplemented!("CO1.5.7 Goodhart shield airgap") }
   115	#[test] #[ignore] fn agent_view_filters_internals() { unimplemented!("CO1.8.6 visibility filter") }
   116	#[test] #[ignore] fn agent_view_minimal_context() { unimplemented!("CO1.8.7 prompt_builder reads only agent_view") }
   117	#[test] #[ignore] fn goodhart_shield() { unimplemented!("CO1.5.2 visibility policy enforcement") }
   118	
   119	// =============================================================================
   120	// Signals (CO1.9, CO1.10)

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/mod.rs | sed -n '1,40p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! L4 ledger support modules for Bottom White.
     2	//!
     3	//! /// TRACE_MATRIX FC1-Sig+FC3-Sig: ledger crypto support root
     4	
     5	/// TRACE_MATRIX FC1-Sig+FC3-Sig: system runtime signature key lifecycle.
     6	pub mod system_keypair;
     7	
     8	/// TRACE_MATRIX FC2-Append + WP § 5.L4: L4 transition ledger (CO1.7 type skeleton).
     9	/// Status: pre-audit type skeleton; bodies that need transition functions are deferred.
    10	pub mod transition_ledger;

exec
/bin/bash -lc 'git ls-files src/bottom_white/ledger/transition_ledger.rs src/state/sequencer.rs src/bottom_white/ledger/mod.rs src/bus.rs src/kernel.rs src/wal.rs src/wallet.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/ledger/mod.rs
src/bottom_white/ledger/transition_ledger.rs
src/bus.rs
src/kernel.rs
src/wal.rs

exec
/bin/bash -lc "rg --files src | rg '("'^|/)sequencer'"\\.rs"'$|(''^|/)wal'"\\.rs"'$|(''^|/)wallet'"\\.rs"'$|(''^|/)kernel'"\\.rs"'$|(''^|/)bus'"\\.rs"'$'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/kernel.rs
src/wal.rs
src/bus.rs
src/sdk/tools/wallet.rs

codex
# Codex CO1.7 Round-1 Audit

## Q-A DIV-1..5 validity
DIV-1 real. `CanonicalMessage` has only 3 variants in shipped code, no `LedgerEntry` path ([system_keypair.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:188)), and verification requires `CanonicalMessage + epoch + pinned_pubkeys` ([system_keypair.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:433)). Spec v1 still calls a nonexistent digest-form verifier at spec lines 238-240.

DIV-2 real but overstated as an L4 blocker. `QState` economic fields are currently BTreeMap newtype shells, not domain mutation APIs ([q_state.rs](/home/zephryj/projects/turingosv4/src/state/q_state.rs:143), [q_state.rs](/home/zephryj/projects/turingosv4/src/state/q_state.rs:157)). CO1.7 can still implement ledger append/storage and chain replay; full economic transition bodies remain blocked.

DIV-3 real. Spec schema lacks `epoch` at lines 49-80; skeleton adds it at lines 100-101. Correct fix: include `epoch` in the signed payload, not exclude it.

DIV-4 real and defensible. Spec invents `CasReader` at line 224; shipped API is concrete `CasStore::get(&Cid)` ([store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:110)). A narrow read trait is fine for replay seams.

DIV-5 real. `CasStore::put` has metadata parameters ([store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:78)). Sequencer responsibility is correctly placed, but spec pseudocode uses `self.cas.put_canonical` without a `cas` field in `Sequencer` lines 123-145.

## Q-B Q9 spec bug closure
Skeleton’s exclusion of `resulting_ledger_root` and `system_signature` is necessary ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:113)). Do not exclude `epoch`; it is sequencer-set, but the signature must bind which pinned key epoch accepted the entry.

`tx_payload_cid` should remain included. `Cid` is sha256(content), independent of CAS metadata ([schema.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/schema.rs:8)). If canonical payload bytes differ across implementations, the CID should differ.

New Q9 gap: the signature does not bind the parent ledger root. Since `resulting_ledger_root` is unsigned and derived, an attacker can transplant a valid signed entry onto another history with the same `logical_t/parent_state_root` and recompute `resulting_ledger_root`. Add `parent_ledger_root` as a stored field or make `canonical_digest_unsigned(parent_ledger_root)` sign the previous ledger root.

## Q-C Sequencer correctness
Current pseudocode is not correct enough to implement.

The biggest bug: `fetch_add` happens before transition acceptance at lines 158-173. If `step_transition` rejects, no entry is committed, but the next accepted entry gets a skipped `logical_t`; skeleton writer and replay reject gaps ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:211), [transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:278)). Spec needs separate counters for submissions/rejections vs accepted ledger entries, or rejection entries must be first-class L4 entries.

There is also an order contradiction: spec says `submit()` returns `logical_t` lines 147-150, but pseudocode assigns it in `apply_one` line 159. STATE spec says tie-break is assignment order at submit/concurrent call time ([STATE spec](/home/zephryj/projects/turingosv4/handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:999)).

RwLock pattern is acceptable only if there is exactly one `run()` driver and ledger readers do not observe storage independently of `QState`. Spec should state that. `SeqCst` is sufficient but must define initial value = 1 and overflow fail-closed. Signing API should be `sign_ledger_entry(keypair, LedgerEntrySigningPayload)` in a new authorized keypair submodule; merely adding an enum variant is not enough for the security boundary.

## Q-D Replay completeness
Partial replay is acceptable only as skeleton smoke, not as a v1 CO1.7 deliverable. Spec says replay is the I-DETHASH witness at lines 219-264 and invariant witness at lines 305-306. Skeleton explicitly does chain-only replay and does not verify signatures, payload recovery, or transition recomputation ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:240), [transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:270)).

Add `ReplayMode::ChainOnly` vs `ReplayMode::FullTransition`. Only full mode may satisfy I-DETHASH.

## Q-E Atom scope creep
The L4/L5 boundary is currently blurred. Spec declares L5 materializer out of scope at line 30, but sequencer sets `head_t = NodeId::from_state_root(...)` at line 194, while storage mapping says `head_t = NodeId(commit_sha)` at line 276. Those are inconsistent. `NodeId::from_state_root` is currently only a hex-state-root placeholder ([q_state.rs](/home/zephryj/projects/turingosv4/src/state/q_state.rs:51)).

CO1.7 can own ledger roots and commit-chain heads. L5 must own state_root materialization. Spec v1.1 should define a stub `StateRootProvider`/materializer boundary or defer `head_t` mutation out of CO1.7.

## Q-F STEP_B disposition
No STEP_B ceremony is required for the current skeleton file itself. STEP_B scope is `kernel.rs`, `bus.rs`, `wallet.rs` per CLAUDE and protocol ([CLAUDE.md](/home/zephryj/projects/turingosv4/CLAUDE.md:14), [STEP_B_PROTOCOL.md](/home/zephryj/projects/turingosv4/handover/ai-direct/STEP_B_PROTOCOL.md:3)). Spec line 359 incorrectly names `wal.rs` instead of `wallet.rs`, and understates that `mod.rs` and `genesis_payload.toml` were touched. Future runtime wiring into `bus.rs`/`kernel.rs` will need STEP_B.

## Q-G Open Q recommendations
Q1: bounded `tokio::sync::mpsc`, not unbounded. Return `QueueFull` or provide `submit_async`.

Q4: keep `system_signature` inside `LedgerEntry`, but sign a separate `LedgerEntrySigningPayload`.

Q5: enum-match dispatch for v4. It is exhaustive and deterministic. Defer `MetaTransitionInterface` until v4.1 dynamic MetaTx.

Q7: prefer domain-separated `ledger_root_0 = sha256(genesis_payload canonical digest)`, not `Hash::ZERO`, unless `Hash::ZERO` is only the empty accumulator under a separate genesis anchor.

Q8: extend the typed system-keypair path with a ledger-specific signing payload and authorized `transition_ledger_emitter`; avoid a raw sibling digest signer.

Q11 new: parent ledger root binding, rejected-submission logical time, CAS persistence for cold replay, LedgerEntry canonical serialization fixtures, L4/L5 `head_t` ownership.

## Q-H New defects
CAS cold replay is not actually supported by shipped `CasStore`: `open()` initializes an empty in-memory index ([store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:67)), and `get()` requires that index ([store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:111)). CO1.7 replay cannot recover payloads after restart unless CAS metadata is persisted or ledger commits include enough blob lookup data.

Spec/skeleton trait mismatch: spec `LedgerWriter::commit(&self) -> NodeId` and `iter_from` lines 282-291; skeleton `commit(&mut self) -> Hash`, no iterator lines 164-173.

Skeleton manual digest uses `tx_kind as u8` without `#[repr(u8)]` ([transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:132)). Make discriminants explicit.

`TxKind::Slash` exists, but § 8 dispatch omits `TypedTx::Slash` lines 340-349.

Conformance gap is large: spec promises 8 CO1.7 tests lines 318-331; skeleton has 6 inline unit tests and none for signature, CAS recovery, sequencer serialization, full replay, or canonical fixtures.

## Q-I Implementation gating
After v1.1 patches, it is implementable. Current blockers: logical_t gap semantics, ledger-entry signing API, parent ledger root binding, CAS persistence for cold replay, L4/L5 boundary, and canonical serialization/discriminants. I verified `cargo check` passes and `cargo test --lib bottom_white::ledger::transition_ledger::` passes 6/6, but that smoke only covers pure chain helpers.

## **VERDICT**: CHALLENGE

## Top 3 must-fix
1. Fix sequencer logical time: no skipped accepted-entry `logical_t`, clear submit-vs-accept ordering.
2. Redesign ledger signing payload to bind `epoch` and parent ledger root while excluding only derived root and signature.
3. Make replay honestly two-mode and ensure full replay can recover CAS payloads after cold restart.

## Conviction
High.
2026-04-28T07:24:27.972633Z ERROR codex_core::session: failed to record rollout items: thread 019dd2f6-3efc-70d1-8ad0-5ff9f9fe8988 not found


---

# Gemini round-1 audit verdict (parallel; convergent flagged 3 same items)

# Gemini CO1.7 transition_ledger Round-1 Audit Run
- Model: gemini-2.5-pro
- Packet chars: 250471
- Started: 2026-04-28T07:20:45+00:00

---

# Gemini CO1.7 Round-1 Audit

## Q1 Constitutional alignment

The CO1.7 design demonstrates strong alignment with the TuringOS Constitution and the Anti-Oreo architecture.

-   **Art 0.1 四要素映射 (Tape / Input / Q / State)**: **PASS**. The design correctly maps the append-only ledger (`refs/transitions/main` git commit chain per spec § 5) to the constitutional `tape_t`. The `Sequencer` (spec § 3) provides the "strict discipline" by serializing transitions and stamping them with a `system_signature`.

-   **Art 0.2 Tape Canonical 公理**: **PASS**. The design upholds this axiom.
    -   *No rejection sidecar*: The spec correctly notes that rejected transactions do not advance the ledger state, and failure metadata is handled by `RejectedAttemptSummary` or `TerminalSummaryTx` (per `STATE_TRANSITION_SPEC § 1.4, 1.5`), which are themselves accepted ledger entries. This avoids the anti-pattern of a parallel "graveyard" ledger.
    -   *System signature attests semantics*: `LedgerEntry.system_signature` (spec § 1) is explicitly defined as the sequencer's attestation of acceptance, distinct from the agent's signature within the payload. This is the correct trust boundary.
    -   *Rejected tx do NOT advance ledger*: Per `STATE_TRANSITION_SPEC § 3`, predicate failures and other rejections occur *before* the `ledger::append` step, ensuring `Q_{t+1} = Q_t` for rejected proposals.

-   **Art 0.4 Q_t version-controlled**: **PASS**. The design directly implements this.
    -   `head_t = git commit SHA`: Spec § 5 explicitly maps `head_t` to the git commit SHA of the `LedgerEntry`.
    -   `ledger_root_t = Merkle root`: Spec § 1 and § 4 define the `resulting_ledger_root` as a chained hash, forming a Merkle accumulation, which is the value stored in `Q_t.ledger_root_t`.

-   **Anti-Oreo 三层结构**: **PASS**. The ledger is correctly positioned as a foundational tool. Spec § 2 places the module in `src/bottom_white/ledger`, confirming its role as a deterministic, auditable tool that records state transitions but does not generate or validate them. The `Sequencer` in `src/state/` acts as the interface between the middle-black (agent submissions) and the bottom-white (ledger writing), governed by rules from the top-white (predicates).

The design is constitutionally sound.

## Q2 WP § 5.L4 conformance

The envelope-and-payload design is a valid architectural choice that conforms to the whitepaper's axioms, but it introduces an availability risk that must be addressed.

-   **Preserve WP § 5.L4 axioms?**: **PASS**. The whitepaper defines the logical content of a `WorkTx`. The CO1.7 `LedgerEntry` is an envelope that contains a content-addressed identifier (`tx_payload_cid`) pointing to the canonical `WorkTx` payload. All 12 fields specified in WP § 5.L4 are preserved within the CAS-stored payload. The envelope adds sequencer-level metadata (`logical_t`, `system_signature`, etc.), which is a correct separation of concerns between agent-proposed state and system-accepted state.

-   **Create a 2-step indirection vulnerability?**: **CHALLENGE**. Yes. This is a significant architectural trade-off. By separating the ledger entry from its payload, the system's ability to fully interpret or replay the ledger becomes dependent on the availability of two systems: the L4 Transition Ledger store (git repo) and the L3 CAS. If the CAS is unavailable or a payload object is lost (e.g., due to failed garbage collection or backend corruption), the corresponding `LedgerEntry` becomes an un-reconstructible pointer, violating the spirit of Art 0.2. This availability dependency is a strategic risk that must be acknowledged and have a mitigation plan (e.g., replication, bundled archival).

-   **Correctly map "agent_signature" vs "system_signature"?**: **PASS**. The structure is correct and constitutionally sound. The agent's `WorkTx.signature` is inside the payload, attesting to the agent's intent. The `LedgerEntry.system_signature` is applied by the sequencer, attesting that the system has validated and accepted the transition at a specific logical time. This correctly separates the roles of proposal (middle-black agent) and disciplined acceptance (white-box system).

## Q3 Reconstructibility (Art 0.2)

The v1 replay implementation represents a significant, albeit temporary, deviation from the full guarantee of Art 0.2 and creates a trust model ambiguity.

-   **A defensible v1 deliverable?**: **CHALLENGE**. The skeleton's `replay_chain_integrity` function validates the hash chains of `parent_state_root` and `ledger_root`. This is a necessary condition for reconstructibility but is far from sufficient. It proves the *integrity of the record* but explicitly defers proving the *correctness of the recorded state transitions*. While this phased approach is a common engineering practice, it is only defensible if the resulting trust model is made explicit and the naming is not misleading.

-   **A partial Art 0.2 implementation that should NOT be called "I-DETHASH witness" until full?**: **CHALLENGE**. Correct. Calling the current chain-integrity check a full "I-DETHASH witness" is an overstatement. It is a witness for *chain integrity*, not for *deterministic state transition*. The name should be qualified, for example, as an "I-DETHASH chain-integrity witness," until the full replay (spec § 4 `replay` function, including re-running pure transitions) is implemented in CO1.7.5+.

-   **Creating a "trust mode" ambiguity?**: **CHALLENGE**. Yes. This is the core strategic issue. This design implicitly creates two modes of trust:
    1.  **Chain-Trust Mode (v1)**: Trust that the sequencer which produced the ledger was compliant, as we only verify its hash chain, not its work. This is akin to a light client in a blockchain.
    2.  **Full-Replay-Trust Mode (v-future)**: Trust nothing but the genesis state and the code. This is akin to a full node.

This ambiguity is a critical architectural detail. The system MUST be explicit about which trust model is currently in effect. Failure to do so undermines the verifiability guarantees of the entire system.

## Q4 CanonicalMessage extension

This question (surfaced as Q8/DIV-1) reveals a critical integration point with a shipped security primitive. The choice has constitutional implications for simplicity and auditability.

-   **Constitutional argument for/against each?**:
    -   **(a) Extend enum**: This path aligns with a core constitutional principle of simplicity and having a single, unified system for critical operations. A single `CanonicalMessage` enum and a single `verify_system_signature` function create a minimal, highly-auditable surface area for system-level cryptographic attestations. This is strongly preferable. The main drawback is coupling the ledger schema to the `system_keypair` module.
    -   **(b) Introduce sibling primitive**: This path creates a second, parallel signing and verification pathway. This doubles the surface area for audit and potential implementation drift or bugs. It violates the principle of minimizing cryptographic primitives. It is constitutionally weaker.

-   **Is "single canonical sign primitive" a property worth preserving?**: **PASS**. Yes, absolutely. Analogous to the "single tape" axiom (Art 0.2), a "single system signing primitive" is a powerful architectural property. It ensures that all system-level attestations are created and verified through one, and only one, well-understood and heavily scrutinized code path.

-   **If (a): does it create a forward-compat hazard?**: **CHALLENGE**. Yes. This is the primary risk of path (a). As new system-signed messages are added, the `CanonicalMessage` enum will grow. If `LedgerEntry` schema changes, the enum definition must be updated. This creates a maintenance burden and a risk of breaking changes if not managed carefully. The `bincode` serialization of enums can be brittle. This hazard must be mitigated by defining a clear, forward-compatible serialization strategy for the enum itself (e.g., explicit discriminants, versioning within variants).

**Recommendation**: Pursue path (a) for its constitutional simplicity and auditability, but explicitly specify and test a forward-compatible serialization strategy for the `CanonicalMessage` enum to mitigate the identified hazard.

## Q5 INV8 interaction

The CO1.7 design is neutral with respect to the final resolution of INV8 (DAG determinism).

-   The core of the CO1.7 design is the `Sequencer`, which imposes a **strict, linear, total ordering** on all incoming transactions (spec § 3, § 5.2.1). It serializes concurrent submissions into a single, unambiguous sequence based on `logical_t`.
-   INV8 concerns the deterministic resolution of dependencies in a directed acyclic graph (DAG) of transactions.
-   By enforcing a linear order, the sequencer effectively collapses any potential DAG into a simple chain. This design does not pre-commit to any particular DAG resolution logic (e.g., how to handle read/write set conflicts) because it prevents such conflicts from being processed concurrently in the first place. The problem of DAG determinism is pushed outside the boundary of the sequencer itself.

The design is **INV8-neutral**. **PASS**.

## Q6 Sequencer cell explosion

The spec's mandate for one sequencer and one disjoint `runtime_repo` per cell presents a potential operational scaling challenge.

-   The spec (§ 5.2.1, § 5.2.2) requires `N` sequencers and `N` git repos for `N` concurrent cells. For Phase C, N=100.
-   This is an O(N) resource cost, not O(N²). The cost is in process/thread handles, file descriptors, and inodes.
-   For N=100, this is likely tractable on modern hardware, but it is not elegant and may not scale to thousands of cells. It introduces significant operational complexity for setup, teardown, and monitoring.
-   The design choice prioritizes simplicity and absolute isolation over resource efficiency. This is a valid choice for an early-stage system where correctness and isolation are paramount. However, it represents an architectural pressure point that will need to be addressed in future versions for large-scale deployments.

This is not a design flaw that warrants a veto, but it is a strategic limitation. **CHALLENGE**. The operational implications and scaling limits of this "one repo per cell" model should be documented.

## Q7 v4 / v4.1 boundary

The spec and skeleton correctly implement the D-VETO-4=B decision, but the forward-compatibility of the `TxKind` enum is a concern.

-   **Correct boundary?**: **PASS**. The spec (§ 0) and skeleton (`TxKind` enum) correctly defer the `MetaTx` to v4.1. The v4 behavior of emitting a `MetaProposalDraft` to L3 CAS is explicitly stated. This perfectly reflects the ratified decision.

-   **Will adding `TxKind::Meta` in v4.1 break binary compat?**: **CHALLENGE**. Potentially, yes. Adding a variant to a Rust enum is a breaking change for serialization formats like `bincode` unless a forward-compatible representation is explicitly chosen (e.g., `#[repr(u8)]` with reserved space, or a format that is inherently extensible). Since the `LedgerEntry` is part of the canonical, on-disk format, this is a critical detail. The spec must mandate a forward-compatible serialization scheme for `TxKind` to ensure a smooth upgrade to v4.1.

## Q8 Open Q recommendations

-   **Q1 (SubmissionQueue type)**: **tokio mpsc**. The strategic argument is **dependency consistency**. The existing kernel runtime already uses tokio. Introducing crossbeam or std mpsc would add a new dependency and potentially a new async/threading model to bridge, increasing complexity for no clear strategic gain.

-   **Q3 (Sequencer-vs-Writer split)**: **Split**. The strategic argument is **separation of concerns and testability**. A `Sequencer` that orchestrates logic (ordering, dispatching transitions) is a distinct role from a `LedgerWriter` that handles the physical storage mechanics (git commits). Separating them into an `OrderingCoordinator` and a `LedgerWriter` trait allows for independent testing (e.g., testing ordering logic with an in-memory writer) and implementation (e.g., swapping the git writer for a database writer without changing the ordering logic). This is a cleaner, more robust abstraction.

-   **Q5 (enum-match vs MetaTransitionInterface trait)**: **enum-match for v4**. The strategic argument is **YAGNI (You Ain't Gonna Need It)**. The `MetaTransitionInterface` trait is designed for the complex, extensible world of v4.1 meta-governance. The set of v4 object-level transitions is small, fixed, and performance-sensitive. An enum-match is simpler, more performant (static vs. dynamic dispatch), and makes the complete set of possible transitions explicit at the call site. Applying the trait pattern here is over-engineering for the v4 scope.

-   **Q7 (genesis ledger_root_t)**: **sha256 of genesis_payload.toml**. The constitutional argument is **verifiable anchoring**. `Hash::ZERO` is an arbitrary convention. Hashing the genesis payload creates a cryptographic commitment that binds the entire ledger history to the specific configuration that birthed it. Any attempt to replay the ledger against a different genesis will fail verification immediately. This provides a much stronger, non-repudiable anchor for the entire system's history.

-   **Q10 (epoch field binding)**: **Do not bind to ledger_root_t**. The `epoch` field is metadata for signature verification, binding a specific entry to a specific system key. The `ledger_root_t` is a hash of the historical sequence of state transitions. These are orthogonal concerns. The `system_signature` already binds the epoch to the entry's content digest. Including the epoch in the ledger root hash as well would be redundant and would undesirably couple the ledger's historical hash to the key rotation schedule.

## Q9 Forward sustainability

The v1 design is functional but lacks explicit affordances for future extension, which is a strategic risk for a system intended for long-term evolution.

-   The envelope/payload design is a major implicit affordance, as it allows payload types to evolve independently.
-   However, the `LedgerEntry` struct itself is rigid. If an on-chain settlement proof, ZK predicate, or other metadata is needed for a public market, there is no designated place for it. This would require a breaking schema change.
-   **Recommendation**: To enhance forward sustainability, v1 should reserve an explicit extension point. A simple and effective pattern is to add a field like `extensions: BTreeMap<String, Vec<u8>>` to the `LedgerEntry` struct. In v1, this map would be empty, incurring minimal overhead. In future versions, new features could populate this map without requiring a breaking change to the top-level struct, allowing for graceful evolution.

The lack of such an extension point is a strategic shortcoming. **CHALLENGE**.

## Q10 **VERDICT**: CHALLENGE

The CO1.7 design is constitutionally aligned and architecturally sound in its core principles. However, the audit has identified several significant strategic risks, ambiguities, and integration failures that must be resolved before implementation proceeds beyond the skeleton. The design prioritizes v1 simplicity at the cost of creating a temporary trust model ambiguity (Q3) and lacks explicit forward-compatibility affordances (Q7, Q9). The indirection to CAS (Q2) creates an availability dependency that must be managed. The integration with the existing signing primitive (Q4) is unresolved.

## Top 3 must-fix

1.  **Reconstructibility & Trust Ambiguity (Q3)**: The v1 `replay` function must be clearly named and documented as a "chain-integrity check," not a full I-DETHASH witness. The documentation must explicitly state that v1 operates in a "trust-the-sequencer" mode until full transition replay is implemented, resolving the trust model ambiguity.
2.  **Canonical Signing Primitive Integration (Q4/DIV-1)**: A definitive path for signing `LedgerEntry` must be chosen and specified. The recommended path is to extend the existing `CanonicalMessage` enum (Path A), and the spec must define a forward-compatible serialization strategy for this enum to mitigate future breakage.
3.  **Indirection & Availability Risk (Q2)**: The spec must acknowledge the availability risk created by the L4 ledger's dependency on the L3 CAS. A mitigation strategy must be outlined, even if its implementation is deferred (e.g., "v4.x will introduce CAS replication," or "archival nodes must bundle ledger entries with their payloads").

## Conviction

High. The provided artifacts are comprehensive, allowing for a deep, cross-referenced analysis. The identified issues are grounded in specific constitutional articles, whitepaper sections, and code-level realities, representing clear architectural and strategic risks.

---
## Usage: prompt=82763 candidates=3855 total=90232 thoughts=3614
- Finished: 2026-04-28T07:21:58+00:00


---

# Merged round-1 verdict + v1.1 patch list (the architect synthesis)

# CO1.7 Round-1 Dual External Audit — Merged Verdict

**Date**: 2026-04-28
**Target**: spec v1 (`handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md`) + type-skeleton (`src/bottom_white/ledger/transition_ledger.rs`)
**Auditors**: Codex (gpt-5-codex; 129k tokens) + Gemini 2.5 Pro (90k tokens)
**Conservative rule** (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS

---

## § 1 Verdicts

| Auditor | Verdict | Conviction | Top 3 must-fix |
|---|---|---|---|
| Codex | **CHALLENGE** | High | (1) sequencer logical_t skip race; (2) signing payload binds epoch + parent_ledger_root; (3) replay two-mode + CAS cold-replay |
| Gemini | **CHALLENGE** | High | (1) replay trust ambiguity → name "chain-integrity check"; (2) Q4/DIV-1 pick Path A + forward-compat serde; (3) Q2 CAS availability risk + mitigation |
| **Conservative merged** | **CHALLENGE** | High | merge below |

**Both NOT VETO**. Both accept "after v1.1 patches the joint artifact is implementable" (Codex Q-I explicit; Gemini implicit).

---

## § 2 Convergent must-fix (both flagged)

| # | Topic | Codex § | Gemini § | v1.1 patch direction |
|---|---|---|---|---|
| **C1** | **Replay completeness / trust mode** — chain-only is not v1 deliverable; rename + two-mode | Q-D | Q3 | Add `ReplayMode::ChainOnly` vs `ReplayMode::FullTransition`; spec rename current method; declare which mode is I-DETHASH witness |
| **C2** | **CAS availability / cold-replay risk** — shipped `CasStore::open()` initializes empty in-memory index → no payload recovery after restart | Q-H | Q2 | Spec acknowledges L4 → L3 dependency; either (a) defer cold-replay to v4.x with mitigation outline, or (b) v1.1 requires CasStore index persistence (separate atom) |
| **C3** | **Signing primitive integration (DIV-1 / Q8)** — extend typed CanonicalMessage path | Q-G | Q4 | **Path A** (extend) + sign a separate `LedgerEntrySigningPayload` (NOT the LedgerEntry itself); add forward-compat serialization clause |

---

## § 3 Codex-only must-fix (Codex high-conviction; Gemini didn't probe these)

| # | Topic | Codex § | v1.1 patch direction |
|---|---|---|---|
| **K1** | **Sequencer logical_t skip race** — `fetch_add` before accept skips on rejection; skeleton + replay reject gaps | Q-C top-1 | Either (a) separate counters for submissions vs accepted entries (assign logical_t at COMMIT time, not submit), or (b) rejection entries become first-class L4 entries with logical_t. Recommend (a) for v4 minimalism. |
| **K2** | **Signature does NOT bind parent_ledger_root** — transplant attack vector | Q-B (NEW) | Add `parent_ledger_root` to `LedgerEntrySigningPayload`. |
| **K3** | **L4/L5 head_t ownership inconsistent** — spec line 194 (`from_state_root`) vs line 276 (`commit_sha`) disagree | Q-E | CO1.7 owns `ledger_root_t` + commit-chain head; L5 (CO1.8) owns `state_root_t` materialization. Spec v1.1 explicit boundary; defer `head_t` mutation OR define `StateRootProvider` stub. |
| **K4** | **Spec/skeleton trait mismatch** — spec `LedgerWriter::commit(&self) → NodeId` + `iter_from`; skeleton `commit(&mut self) → Hash`, no iter | Q-H | Reconcile in v1.1; keep skeleton signature (already cargo-check verified) and update spec, OR extend skeleton trait. |
| **K5** | **TxKind::Slash dispatch gap** — enum has Slash; § 8 dispatch omits | Q-H | Either remove `Slash` from TxKind for v1 (defer to CO P2.5) OR add to dispatch. Recommend remove for v4 minimalism. |
| **K6** | **`#[repr(u8)]` on TxKind** — discriminant fragility (default Rust enum repr is unspecified for cast) | Q-H | Add `#[repr(u8)]` + explicit discriminants (e.g. `Work = 0, Verify = 1, ...`). |
| **K7** | **Conformance test gap** — spec promises 8; skeleton has 6 (none for signature, CAS recovery, sequencer serialization, full replay, canonical fixtures) | Q-H | Either lower spec promise (defer 2 tests to CO1.7.5+ along with body impls) OR add stubbed tests now. |

---

## § 4 Gemini-only must-fix

| # | Topic | Gemini § | v1.1 patch direction |
|---|---|---|---|
| **G1** | **Forward-compat extensions** — LedgerEntry struct rigid; future ZK predicate / settlement proof / public-market metadata has no place | Q9 | Add `extensions: BTreeMap<String, Vec<u8>>` field (empty in v1; reserved for v4.x without breaking schema). |

---

## § 5 Disagreement (conservative resolution required)

| # | Topic | Codex | Gemini | Conservative |
|---|---|---|---|---|
| **D1** | Should `epoch` be bound into canonical signed digest? | YES (security: signature must bind which key signed, otherwise an old-epoch key could be replayed against new-epoch key holders) | NO (Q10 — ledger_root and epoch are "orthogonal concerns"; binding is "redundant") | **Codex wins** — concrete security argument trumps aesthetic orthogonality. Without epoch binding, an attacker with leaked old-epoch key could forge entries that verify against current pinned key (per Codex Q-B "transplant attack"). Skeleton already includes epoch; v1.1 retains. |

---

## § 6 Open Q recommendations (broadly aligned)

| Q | Recommendation | Source |
|---|---|---|
| Q1 SubmissionQueue type | bounded `tokio::sync::mpsc` (NOT unbounded); `QueueFull` error or `submit_async` | Codex Q-G |
| Q4 signature placement | keep `system_signature` inside `LedgerEntry`; sign separate `LedgerEntrySigningPayload` struct | Codex Q-G + Gemini Q4 |
| Q5 dispatch | enum-match for v4; defer `MetaTransitionInterface` to v4.1 | Codex Q-G |
| Q7 genesis ledger_root_0 | sha256(canonical digest of genesis_payload.toml) — NOT `Hash::ZERO` | Codex + Gemini agree |
| Q8 CanonicalMessage extension | Path A: extend enum AND introduce `LedgerEntrySigningPayload` (NOT raw sibling digest) | Codex + Gemini agree |
| Q10 epoch binding | bind in signed payload; do NOT bind in `ledger_root_t` fold (these are different axes — see D1 resolution) | Codex (security) overrides Gemini (orthogonality) |

---

## § 7 v1.1 Patch list (consolidated, ranked by effort/impact)

### Tier 1 — Spec-only edits (~30-60 min)
1. **C1** — replay two-mode rename + ReplayMode enum spec; declare full-mode = I-DETHASH witness (skeleton can stay; v1 doc updates)
2. **C2** — § 0 + § 5 acknowledge CAS cold-replay risk; defer CasStore index persistence to dedicated atom (likely CO1.4-extra)
3. **C3** — § 1 + § 11 Q8 close: extend `CanonicalMessage::LedgerEntrySigning(LedgerEntrySigningPayload)`; spec the new struct
4. **K3** — § 0 + § 5 explicit L4/L5 boundary (CO1.7 owns ledger_root + head_t; CO1.8 owns state_root); update sequencer pseudocode `head_t` derivation
5. **K4** — reconcile `LedgerWriter` trait with skeleton (drop `iter_from` for v1; keep `&mut self` + `Hash` return)
6. **K5** — drop `TxKind::Slash` for v4; defer to CO P2.5 atom
7. **K6** — add `#[repr(u8)]` + explicit discriminants in spec § 1 schema box
8. **G1** — add `extensions: BTreeMap<String, Vec<u8>>` to LedgerEntry schema

### Tier 2 — Spec-only design upgrades (~30-45 min)
9. **K1** — sequencer dual-counter design (next_logical_t advances only on commit; submit_id is separate); update § 3 pseudocode
10. **K2** — `LedgerEntrySigningPayload` includes `parent_ledger_root` (transplant defense); update spec § 1
11. **D1** — explicit "epoch IS in signed payload" clause (closes Codex/Gemini disagreement; Codex security wins)
12. Q1/Q4/Q5/Q7/Q8/Q10 fold from § 11 Open into spec body resolutions

### Tier 3 — Skeleton patches (~30-45 min)
13. Add `#[repr(u8)]` + discriminants on TxKind
14. Drop `TxKind::Slash` (or keep + add dispatch path — pick per K5)
15. Add `extensions: BTreeMap<String, Vec<u8>>` field
16. Update `canonical_digest_unsigned`: now signs `LedgerEntrySigningPayload` (with parent_ledger_root + epoch + tx_payload_cid + resulting_state_root + tx_kind + logical_t + timestamp_logical) — NOT including resulting_ledger_root or signature
17. Add 2 new test stubs (signature binding + transplant defense) to lift count from 6 → 8

### Tier 4 — Round-2 dual audit re-launch (~$5-15)
18. Same prompt structure; updated joint artifact attached; Codex + Gemini in parallel

**Total v1.1 estimated**: 1.5-2.5 hr Claude work + ~$5-15 round-2 audit cost.

---

## § 8 Decision tree

| Path | Description | Trade-off |
|---|---|---|
| **A** | Execute v1.1 patches now (Tier 1+2+3) → round-2 audit | ~3-4 hr total to PASS/PASS gate; recommended |
| **B** | Stop at v1 + skeleton; defer round-2 to a future session | Saves immediate cost; loses momentum on Wave 6 #1 |
| **C** | Reduced v1.1 (Tier 1 only — convergent C1/C2/C3 + low-cost K3/K4/K5/K6/G1) → round-2 | ~1.5 hr; only addresses 8 of 11 must-fix; round-2 likely CHALLENGE again |

**Recommended**: **A**. Both auditors high-conviction CHALLENGE = real defects, not principle. v1.1 is a single concentrated patch round; skipping items will cost more in round-3+.

---

## § 9 Cost ledger entry

| Audit | Tokens | Estimated $ |
|---|---|---|
| Codex round-1 | 129,132 (combined input/output) | ~$5-8 |
| Gemini round-1 | 90,232 (prompt 82,763 + output 3,855 + thoughts 3,614) | ~$2-4 |
| **Round-1 sub-total** | — | **~$7-12** |

Cumulative project audit spend (per AUDIT_LEDGER): now ~$110-160 / $890 mid-budget (~12-18%).

— ArchitectAI synthesis, 2026-04-28.


---

# XREF: STATE_TRANSITION_SPEC v1.4 (frozen)

# State Transition Specification v1.4

> **Date**: 2026-04-27 (v1.4 closes 4 cosmetic Codex round-3 PARTIAL items)
>
> **Patch v1.3 → v1.4 changes** (per Codex round-3 re-audit at `handover/audits/CODEX_SPEC_V13_REAUDIT_2026-04-27.md`):
> - **§ 5.3 grep list cleanup** (Q1.1): patch log no longer claims `TaskMarketPublishTx` is RETIRED; it's a NEW transition deferred to CO P2.1. Conformance test grep includes only actually-retired symbols.
> - **§ 3.2 challenge_transition + § 3.4 finalize_reward** (Q2.4): both now invoke `ChallengeWindow::is_open(now)` method (defined § 5.2.5 NEW); pseudocode no longer hand-codes the inequality.
> - **§ 5.2.1 sequencer tie-break** (Q6): `next_logical_t()` is atomic; assigned `logical_t` IS the canonical tie-break for concurrent submissions; explicit prose added.
> - **§ 2.5 + § 7 fixture corpus defer-ack** (Q5/NEW-5): canonical serialization RULE frozen v1.4; full golden fixture corpus + differential fuzz seed lands in CO1.1.4-pre1 + CO1.7 atoms (not v1.x spec scope).
>
> **Patch v1.2 → v1.3 changes** (per Codex re-audit verdict CHALLENGE/NO-GO at `handover/audits/CODEX_SPEC_V12_REAUDIT_2026-04-27.md`):
> - **§ 3.6 task_expire_transition refactored** — removed runtime side effects from pure transition; runtime constructs+signs `TaskExpireTx` BEFORE pure entry; restores § 2 + § 3 pure-boundary discipline (Codex new-issue #1 fix)
> - **§ 3.6 stage 3 expiry guard broadened** — refund only if NO claim of ANY status exists for task; prevents race with Pending/Provisional claims (Codex new-issue #2 fix)
> - **§ 3.6.5 agent_implicit_init refactored** — introduce `HasSubmitter` trait with per-tx `submitter_id()` methods; resolves WorkTx vs VerifyTx vs ChallengeTx vs ReuseTx field-name divergence; `ReuseTx` returns None (intentional; reuse facts have no submitter) (Codex new-issue #3 fix)
> - **I-FINALIZE-BATCH-ORDER + § 5.2.3 + test all use `claim_id`** consistently (was 3-way contradiction with `target_work_tx`) (Codex new-issue #4 fix)
> - **I-CHALLENGE-WINDOW-EDGE binding** — `is_open(now)` defined as `now < opens_at + duration_ticks`; both challenge_transition AND finalize_reward MUST use same `is_open()` rule (Codex Q2.4 fix)
> - **§ 5.1 false-challenge prose cleanup** — removed "User can override any default" generality where 11.1 is in fact NOT overridable in v4
> - **§ 6 (NEW) Legacy economic tx disposition** — InvestTx / TaskMarketPublishTx / MarketCreateTx / MarketResolveTx explicitly retired in CO1.1.4 atom (Codex Q1.1 NOT-CLOSED fix)
>
> **Patch v1.1 → v1.2 changes** (per Codex+Gemini CO1.SPEC.0.5 dual audit, 2026-04-27):
> - **§ 2 hidden-input table EXPANDED** — added HAYEK_BOUNTY, BOUNTY_LP, Boltzmann params, BOLTZMANN_SEED, async ordering boundary, WAL/git commit boundary, full HashMap scope, f64 royalty math
> - **§ 2.5 (NEW) canonical serialization** — defines byte-level format for all signed tx + state roots
> - **§ 3.4 finalize_reward** — added stage 3a (solver stake unlock + return); royalty math now uses integer floor rule
> - **§ 3.6 (NEW) task_expire_transition** — handles unsolved task bounty refund
> - **§ 3.7 (NEW) agent_register implicit-init** — first appearance in L4 = default reputation 0
> - **§ 5.1 false-challenge resolution** — fixed to "v4 default 0, NOT configurable" (resolves prose-vs-pseudocode contradiction)
> - **§ 4 invariants** — 22 → 27 (added I-STAKE-RETURN, I-BOUNTY-REFUND, I-FINALIZE-BATCH-ORDER, I-CHALLENGE-WINDOW-EDGE, I-AGENT-INIT)
> - **§ 6.1 (NEW) concurrency rule** — L4 sequencer per (runtime_repo, run_id); deterministic ordering key
> - **§ 8 count fix** — "16 invariants" → "27 invariants"
>
> **Patch v1 → v1.1 changes** (per SPEC_WALKTHROUGH gap fixes, 2026-04-27):
> - § 3.2 (challenge_transition) stage 4e ADDED: verifier_bond release policy (default = return to verifier; configurable)
> - § 3.3 (reuse_transition) stage 3 AMENDED: edge weight bounded by `MAX_REUSE_ROYALTY_FRACTION` config (default = 0.10)
> - § 3.2 (challenge_transition) stage 4d AMENDED: false-challenge reputation penalty (v1.3 update: **fixed to 0 in v4; NOT configurable**; previous v1.1 patch log saying "configurable" is OBSOLETE)
> - § 3.1 (verify_transition) note ADDED: quorum-aggregation rule placeholder (default = 1; configurable)
> - § 4 invariants ADDED: I-VBOND-RELEASE / I-ROYALTY-CAP
> - § 11 (Found Inconsistencies) — promoted from SPEC_WALKTHROUGH § 11
>
> All 4 walk-through gaps now have either (a) machine-checkable default applied, or (b) explicit deferral with target atom.
>
> **Purpose**: D-VETO-1 binding form. Defines `step_transition: (Q_t, tx_i) → (Q_{t+1}, signals_t)` with typed schemas, deterministic pseudocode, named invariants, conformance test list. Gates CO1.1.4/CO1.1.5 bus.rs/kernel.rs split (per Plan v3.2 atom CO1.SPEC.0).
>
> **Authority**: Constitution Art. 0–0.4 + white paper architecture § 3-7 + economic § 2/§ 6/§ 18-21. Where this spec disagrees with white paper, **white paper wins** and this spec must be amended.
>
> **Audit**: Codex CO P0.7 T+S review (2026-04-27) demanded binding spec form before refactor. This document is the response.

---

## § 0 Scope

**In scope**:
- The single-step state transition function `step_transition` for object-level work_tx
- Typed `QState`, `WorkTx`, `VerifyTx`, `ChallengeTx`, `RejectedAttemptSummary`, `TerminalSummaryTx` schemas
- Hidden-input classification: which existing `bus.rs`/`kernel.rs` inputs are `Q_t`, which are `tx_i`, which are illegal side effects
- Named invariants enforceable mechanically
- Conformance test list generated from the spec

**Out of scope** (handled separately):
- `MetaTx` schema for runtime meta-transitions — defined as **stub only** here; full schema deferred to v4.1 per D-VETO-4 = B (defer, not abandon)
- AttributionEngine DAG construction algorithm — deferred to CO2.4.0 spike (Inv 8 design)
- Full predicate visibility air-gap proof — deferred to CO P1.5 (Goodhart shield design)

---

## § 1 Typed Schemas

### 1.1 QState (white paper § 4 + economic § 2 amendment, 9 fields)

```rust
pub struct QState {
    /// Agent swarm sub-state: tape head per agent, per-agent reputation snapshots, etc.
    /// MUST be reconstructible from L4 transition ledger replay.
    pub q_t: AgentSwarmState,

    /// Current ChainTape head pointer = git commit SHA in Path B substrate.
    pub head_t: NodeId,

    /// Materialized state Merkle root (git tree root in Path B).
    pub state_root_t: Hash,

    /// Agent-visible projection of tape filtered by per-agent visibility policy
    /// (Inv 10 Goodhart shield). Derived from L1 PredicateRegistry visibility tags.
    pub tape_view_t: AgentVisibleProjection,

    /// L4 Transition Ledger root (Merkle root of all accepted tx so far).
    pub ledger_root_t: Hash,

    /// L1 Predicate Registry root.
    pub predicate_registry_root_t: Hash,

    /// L2 Tool Registry root.
    pub tool_registry_root_t: Hash,

    /// Economic state (economic § 2 amendment, 9 sub-fields).
    pub economic_state_t: EconomicState,

    /// Global budget snapshot: cost ceiling, wall clock, compute cap.
    pub budget_state_t: BudgetSnapshot,
}

pub struct AgentSwarmState {
    pub agents: BTreeMap<AgentId, PerAgentState>,
    pub current_round: u64,
}

pub struct PerAgentState {
    pub reputation_snapshot: Reputation,
    pub last_accepted_tx: Option<TxId>,
    pub retry_counter_for_current_task: u32,  // resets on accept; persists across rejections
}

pub struct EconomicState {
    pub balances_t:       BalancesIndex,
    pub escrows_t:        EscrowsIndex,
    pub stakes_t:         StakesIndex,
    pub claims_t:         ClaimsIndex,
    pub reputations_t:    ReputationsIndex,
    pub task_markets_t:   TaskMarketsIndex,
    pub royalty_graph_t:  RoyaltyGraph,
    pub challenge_cases_t: ChallengeCasesIndex,
    pub price_index_t:    PriceIndex,
}
```

**BTreeMap, not HashMap, everywhere**: deterministic iteration order for replay byte-identity (Codex flagged kernel.rs:187-204 HashMap nondeterminism).

### 1.2 WorkTx (12 fields per WP § 5.L4)

```rust
pub struct WorkTx {
    pub tx_id: TxId,                              //  1
    pub task_id: TaskId,                          //  2  links to TaskMarket entry
    pub parent_state_root: Hash,                  //  3  must equal Q_t.state_root_t at submission
    pub agent_id: AgentId,                        //  4
    pub read_set: BTreeSet<ReadKey>,              //  5  agent MUST declare read deps (DAG attribution)
    pub write_set: BTreeSet<WriteKey>,            //  6  agent MUST declare write targets
    pub proposal_cid: Cid,                        //  7  L3 CAS handle to payload (not raw payload)
    pub predicate_results: PredicateResultsBundle,//  8  filled BY runner, not by agent
    pub stake: StakeMicroCoin,                    //  9  YES_E stake, i64 micro-coin units
    pub signature: AgentSignature,                // 10
    pub timestamp_logical: u64,                   // 11  monotonic counter from runtime, NOT wall clock
    pub status: TxStatus,                         // 12  Pending | Accepted | Rejected(class) | Finalized
}

pub enum TxStatus {
    Pending,
    Accepted,
    Rejected(RejectionClass),
    FinalizedReward(MicroCoin),
    FinalizedSlash(SlashEvidenceCid),
}

pub struct PredicateResultsBundle {
    pub acceptance: BTreeMap<PredicateId, BoolWithProof>,
    pub settlement: BTreeMap<PredicateId, BoolWithProof>,
    pub safety_class: SafetyOrCreation,  // determines fail-closed vs fail-open-with-signal
}
```

### 1.3 VerifyTx, ChallengeTx, ReuseTx (economic § 13)

```rust
pub struct VerifyTx {
    pub tx_id: TxId,
    pub target_work_tx: TxId,         // the work_tx being verified
    pub verifier_agent: AgentId,
    pub bond: StakeMicroCoin,         // verifier reputation/bond stake
    pub verdict: VerifyVerdict,       // Confirm | Doubt
    pub signature: AgentSignature,
    pub timestamp_logical: u64,
}

pub struct ChallengeTx {
    pub tx_id: TxId,
    pub target_work_tx: TxId,
    pub challenger_agent: AgentId,
    pub stake: StakeMicroCoin,        // NO_E stake, i64 micro-coin
    pub counterexample_cid: Cid,      // L3 CAS handle to counterexample
    pub signature: AgentSignature,
    pub timestamp_logical: u64,
}

pub struct ReuseTx {
    pub tx_id: TxId,
    pub reusing_work_tx: TxId,        // the work_tx that triggered the reuse
    pub reused_tool_id: ToolId,       // L2 Tool Registry handle
    pub reused_tool_creator: AgentId, // royalty recipient
    pub timestamp_logical: u64,
}
```

### 1.4 RejectedAttemptSummary (D-VETO-6 system-stamped, NOT agent self-report)

```rust
pub struct RejectedAttemptSummary {
    pub failed_attempts_since_last_accept: u32,           // bounded, capped at u32::MAX
    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,  // counts only, no payloads
    pub first_failure_logical_t: Option<u64>,             // for time-to-first-fail signal
    pub last_failure_logical_t: Option<u64>,              // for recency signal
    // NO raw error strings, NO agent payload contents, NO predicate internal traces
}

pub enum RejectionClass {
    AcceptancePredicateFail(PredicateId),     // public predicates only; private predicates → Opaque
    SettlementPredicateFail(PredicateId),
    StakeInsufficient,
    SignatureInvalid,
    StaleParentRoot,                          // Q_t advanced; agent's view stale
    Opaque,                                   // private predicate failure; classification withheld
    BudgetExceeded,
}
```

`RejectedAttemptSummary` is stamped **by the white-box predicate runner** onto the next accepted `WorkTx`. Trust boundary: the runner generates this summary; the agent does NOT self-report. Verified at conformance test level.

### 1.5 TerminalSummaryTx (no-accept run handler)

```rust
pub struct TerminalSummaryTx {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub run_id: RunId,
    pub run_outcome: RunOutcome,           // OmegaAccepted | MaxTxExhausted | WallClockCap | ComputeCap | ErrorHalt
    pub total_attempts: u32,
    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,
    pub last_logical_t: u64,
    pub system_signature: SystemSignature,  // signed by runtime keypair, not by any agent
}
```

If a run terminates without any accepted work_tx, the runtime emits exactly one `TerminalSummaryTx` to L4. This preserves L6 reconstructibility: error class signal is derivable from tape even if no work_tx ever passed.

### 1.6 MetaTx (stub for v4.1; v4 only emits `MetaProposalDraft` to L3 CAS, not L4)

```rust
pub struct MetaTx {
    pub tx_id: TxId,
    pub parent_architecture_root: Hash,
    pub proposed_predicate_patches: Vec<PredicatePatch>,
    pub proposed_tool_patches:      Vec<ToolPatch>,
    pub log_evidence_cids:           Vec<Cid>,
    pub reversibility_plan_cid:      Cid,
    pub constitution_check:          ConstitutionCheckProof,
    pub judge_signatures:            Vec<JudgeSignature>,
    pub human_signature_required:    bool,
    pub human_signature:             Option<HumanSignature>,
}
```

**v4 status**: MetaTx schema reserved; runtime ArchitectAI/JudgeAI **NOT implemented**. v4 produces `MetaProposalDraft` (a CAS object) only, written when ArchitectAI proposes architecture amendments via the cp workflow. v4.1 implements the runtime actor + L4 acceptance.

This is the D-VETO-4 = B (defer, not abandon) implementation.

---

## § 2 Hidden-Input Classification (Codex § C demanded)

The current `src/bus.rs` and `src/kernel.rs` mix four categories of inputs. The spec must classify each:

| Input | Current source | T+S classification | New home in step_transition |
|---|---|---|---|
| `created_at` (wall clock seconds) | `bus.rs:264-268` `SystemTime::now()` | **ILLEGAL hidden side effect** | retire; replace with `timestamp_logical: u64` from runtime monotonic counter |
| `completion_tokens: 0` literal | `bus.rs:268` | **ILLEGAL hidden zero** | kill in CO1.1.4-pre1; read real value from LLM `usage.completion_tokens` |
| `TAPE_ECONOMY_V2` env var | `bus.rs:298, 345` | **`Q_t.budget_state_t.feature_flags`** | promote to typed field; tx must reference flag value at parent_state_root |
| `FOUNDER_GRANT_GAMMA` env var | `bus.rs:307` | **`Q_t.economic_state_t.task_markets_t.config.founder_grant_gamma`** | promote to typed field; bound at task creation, not env at runtime |
| `self.config.system_lp_amount` | `bus.rs:340` | **`Q_t.economic_state_t.task_markets_t.config.system_lp_amount`** | promote |
| `self.clock` counter | `bus.rs:42` | **`Q_t.q_t.current_round` derived** | derive from L4 ledger length; not separately tracked |
| `self.tx_count` | `bus.rs:42` | **`Q_t.q_t.current_round` derived** | derive |
| `self.generation` | `bus.rs:42` | **`Q_t.q_t.generation` typed field** | promote |
| `self.graveyard: HashMap<String, Vec<String>>` | `bus.rs:48` | **ILLEGAL sidecar** (Art. 0.2 explicitly anti-patterned) | retire; replace with `RejectedAttemptSummary` stamped on next accepted tx + `TerminalSummaryTx` |
| Tool list iteration order | `bus.rs:312-319` Vec | **`Q_t.tool_registry_root_t` derived** | runner queries L2 in deterministic order |
| Wallet "magic search" | `bus.rs:312-319` `manifest() == "wallet"` | **EXPLICIT capability lookup** | runner queries L2 by `Capability::EconomicWallet` tag, not by string match |
| `HAYEK_BOUNTY` env var (v1.2 added per Codex Q3) | `src/bus.rs:141-150` (init), `src/bus.rs:349-360` (settle) | **`Q_t.economic_state_t.task_markets_t.config.hayek_bounty_enabled`** | promote to typed task config; bound at task creation |
| `BOUNTY_LP` env var (v1.2 added per Codex Q3) | `src/bus.rs:141-150`, `src/bus.rs:349-360` | **`Q_t.economic_state_t.task_markets_t.config.bounty_lp_seed: MicroCoin`** | promote to typed task config |
| `BOLTZMANN_TEMP` / `FRONTIER_CAP` / `DEPTH_WEIGHT` / `PRICE_GATE_ALPHA` / `BOLTZMANN_SEED` env (v1.2 added per Codex Q3) | `src/sdk/actor.rs:22-39` (params), `experiments/.../bin/evaluator.rs:693-697` (seed) | **OFF-TAPE proposal-generation only**; NOT part of `Q_t`; routing seed visible in `proposal_cid` payload (CAS); transition pseudocode does NOT consume these | classified as "agent-side proposal entropy"; the SAMPLED outcome is on tape via proposal_cid; the sampling RNG state is NOT |
| HashMap iteration order broadly (v1.2 added per Codex Q3) | `src/kernel.rs:19-21` (markets), `src/kernel.rs:165-204` (resolve + ticker), any new code | **BANNED in any module reachable from `step_transition` call tree** | runtime test grep extends to ALL `src/` files reachable transitively; not just modules containing "q_state" or "transition" |
| Async tokio task completion ordering (v1.2 added per Codex Q3 + Q6) | `experiments/.../bin/evaluator.rs:192-193` (#[tokio::main]) | **L4 sequencer (§ 6.1) defines deterministic ordering key (logical_t, tx_id)**; async completion order is NOT used | sequencer enforces serialization point per (runtime_repo, run_id); see § 6.1 |
| WAL / git commit filesystem effects (v1.2 added per Codex Q3) | `src/bus.rs:279-282` (WAL Node), `src/bus.rs:319-327` (WAL event) | **explicit boundary: pure `step_transition(q, tx)` returns `(q', signals)` PURELY; runtime layer commits side effects to WAL/git AFTER pure result** | step_transition is pure function of (q, tx); commit is runtime concern; § 6.1 specifies commit point |
| `f64` arithmetic in monetary / royalty math (v1.2 added per Codex Q3 + Q10) | `src/prediction_market.rs:21-27,87-133` (reserves, trades) + spec § 3.3 royalty `reward * edge.weight` | **i64 MicroCoin only; royalty rounding rule = integer floor (`micro_reward * weight_micro / 1_000_000`)** | promote `prediction_market.rs` to MicroCoin; spec § 3.3 stage 3b adds explicit rounding |
| Future tokio::spawn introduction (v1.2 hypothesis per Codex Q3) | (none currently) | **BANNED in `src/transition/*` and `src/economy/*` call trees** | cargo-deny rule + transitive grep |

After this classification, every step_transition input is either part of `Q_t`, part of `tx_i`, or part of the runtime config bound at genesis (which is itself in `Q_t`).

**Conformance test for § 2** (`tests/no_hidden_inputs.rs`):
- grep src/ for `SystemTime::now()` → must return 0 hits in non-runtime-bootstrap code
- grep src/ for `std::env::var(` → must return 0 hits in step_transition path **AND** in any module transitively reachable from `transition::*`, `economy::*`, `top_white::predicates::*` (v1.2 expanded scope per Codex Q3)
- grep src/ for `HashMap` → must return 0 hits in **ALL modules reachable from `step_transition` call tree** (v1.2 expanded scope; was: only "q_state" or "transition" modules; new scope: full transitive reach)
- assert all monetary fields are typed `MicroCoin` (a newtype around `i64`), no `f64` — **including `src/prediction_market.rs` and any RSP module**
- grep src/ for `tokio::spawn` → must return 0 hits in `src/{transition,economy,top_white::predicates}/*` (v1.2 added per Codex Q3 hypothesis)

## § 2.5 Canonical Serialization (v1.2 NEW per Codex Q5)

> **Required because**: `tx.canonical_digest()` is called in spec § 3 stages 2 of WorkTx / VerifyTx / ChallengeTx, but byte-level format is undefined. STEP_B branch A vs branch B may pick different serialization (JSON sorted keys vs bincode vs Rust derive order) → cross-branch signature verification fails. Mandatory canonical format closes this.

**Format**: **bincode v2** (`bincode::serde`) with the following constraints:
- **Big-endian byte order** for all multi-byte integers (network order; deterministic across platforms)
- **`BTreeMap` keys serialized in lexicographic byte order** (this is bincode default; verified by test)
- **Strings serialized as UTF-8 with explicit length prefix u32-BE**
- **Optional fields: `0x00` prefix for `None`, `0x01` + value for `Some`**
- **Enum discriminant: u8 (variant index in declaration order)**
- **No padding bytes; no implicit alignment**

**Application**:
```rust
pub fn canonical_digest<T: Serialize>(value: &T) -> [u8; 32] {
    let bytes = bincode::serde::encode_to_vec(value, bincode_canonical_config()).expect("serialize");
    sha256(&bytes)
}

fn bincode_canonical_config() -> bincode::config::Configuration {
    bincode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding()    // no varint; fixed-width for determinism
}
```

**Conformance**: `tests/canonical_serialization.rs` MUST verify:
- 1 golden tx fixture per tx type (WorkTx / VerifyTx / ChallengeTx / ReuseTx / TerminalSummaryTx); each has known input → known SHA-256 output
- Round-trip: `decode(encode(x)) == x` byte-identical for 100 random inputs
- Stability: 2 independent runs on same input → same bytes

**STEP_B implication**: branches A and B both use this exact `bincode_canonical_config`; signature verification works cross-branch by construction.

**Out of scope for v1.x spec** (deferred per Codex Q5/NEW-5 round-3 PARTIAL acknowledgment): full golden fixture corpus + differential fuzzing seed + complete runner ABI for QState/SignalBundle/TransitionError. v1.4 freezes the SERIALIZATION RULE (bincode v2 big-endian + BTreeMap lex); fixtures + ABI land in **CO1.1.4-pre1** (canonical fixture corpus) + **CO1.7** (full ABI surface). This is an **explicit deferral** — not unresolved spec ambiguity. STEP_B branch A and branch B both implement the SAME bincode rule; per-tx digest matching is mechanical from v1.4. Full corpus generation is a downstream code task, not spec scope.

---

## § 3 step_transition (Deterministic Pseudocode)

```rust
/// Pure function. Same (Q_t, tx_i) → byte-identical (Q_{t+1}, signals_t).
/// No I/O. No env reads. No clock reads. No randomness without seed in tx_i.
pub fn step_transition(
    q: &QState,
    tx: &WorkTx,
    registry: &PredicateRegistry,
    tool_registry: &ToolRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {

    // STAGE 1: parent_state_root match (stale view rejection)
    if tx.parent_state_root != q.state_root_t {
        return Err(TransitionError::StaleParent {
            expected: q.state_root_t,
            got:      tx.parent_state_root,
        });
        // NB: rejection here does NOT change Q_t; runner stamps RejectedAttemptSummary
        // onto the NEXT accepted tx (or onto TerminalSummaryTx if run ends without accept)
    }

    // STAGE 2: signature verification
    if !verify_signature(&tx.signature, tx.canonical_digest()) {
        return Err(TransitionError::SignatureInvalid);
    }

    // STAGE 3: stake availability (Inv 5 — YES_E event-bound)
    let agent_balance = q.economic_state_t.balances_t.get(&tx.agent_id);
    if agent_balance < tx.stake {
        return Err(TransitionError::StakeInsufficient { available: agent_balance, required: tx.stake });
    }

    // STAGE 4: predicate gate (Inv 6 — predicate-gated transition)
    let acceptance_results = registry.run_acceptance(tx, q)?;
    let safety_class = registry.classify(tx);
    match (safety_class, acceptance_results.all_passed()) {
        (SafetyOrCreation::Safety, false) => {
            return Err(TransitionError::AcceptancePredicateFailed(acceptance_results));
            // fail-closed for Safety (WP § 7.2)
        }
        (SafetyOrCreation::Creation, false) => {
            // fail-open-with-signal: still reject, but emit informational signal (no Q_t change)
            return Err(TransitionError::AcceptancePredicateFailed(acceptance_results));
        }
        _ => {}  // passed; continue
    }

    // STAGE 5: provisional reward issue (Inv 7 — provisional then final)
    let claim = ClaimId::derive(tx.tx_id);
    let provisional_reward = SettlementEngine::issue_provisional(
        claim,
        &q.economic_state_t.escrows_t,
        tx.task_id,
    )?;

    // STAGE 6: state transition apply (deterministic)
    let mut q_next = q.clone();
    q_next.economic_state_t.claims_t.insert(claim, provisional_reward);
    q_next.economic_state_t.stakes_t.lock(tx.agent_id, tx.task_id, tx.stake);
    q_next.economic_state_t.balances_t.debit(tx.agent_id, tx.stake);
    q_next.q_t.update_per_agent(tx.agent_id, |s| {
        s.last_accepted_tx = Some(tx.tx_id);
        s.retry_counter_for_current_task = 0;  // reset on accept
    });

    // L4 append
    let new_ledger_root = ledger::append(&q.ledger_root_t, tx);
    q_next.ledger_root_t = new_ledger_root;

    // L5 materialize
    let new_state_root = materializer::apply(&q.state_root_t, tx);
    q_next.state_root_t = new_state_root;

    // L6 signal emit (broadcast price + reputation; NOT evaluator internals — Inv 10)
    let signals = SignalBundle {
        boolean: vec![Signal::Boolean(BoolSignal::AcceptedAt(tx.tx_id))],
        statistical: vec![
            Signal::Statistical(StatSignal::PriceUpdate(price_for(tx.task_id, q_next.economic_state_t.price_index_t))),
            Signal::Statistical(StatSignal::ReputationDelta(tx.agent_id, +reputation_delta(tx))),
        ],
    };

    // STAGE 7: head advance
    q_next.head_t = NodeId::from_state_root(new_state_root);

    // STAGE 8: challenge window open (Inv 7 — finalization is deferred)
    q_next.economic_state_t.challenge_cases_t.open(claim, tx.timestamp_logical, CHALLENGE_WINDOW_TICKS);

    Ok((q_next, signals))
}
```

**No wall-clock, no env-var, no HashMap iteration**. Every input is either `q`, `tx`, or registries (themselves in `q.predicate_registry_root_t` / `q.tool_registry_root_t`).

### 3.1 verify_transition (VerifyTx)

Per Gemini v3.2 review Q10 VETO — extending pseudocode to all state-mutating tx types.

> **v1.1 note (gap 11.4)**: this pseudocode handles ONE verifier per tx. Multi-verifier quorum aggregation is a TaskMarket config (`verifier_quorum_required: usize` default = 1). When N>1 verifiers each submit verify_tx for the same target_work_tx, claim transitions to `Pending → ApprovedByVerifiers` only after `verifier_quorum_required` distinct verifiers have submitted `Confirm`. Aggregation rule deferred to CO P2.7 atom (Verifier role detail). For v4 default (quorum=1), each verify_tx independently advances claim to ApprovedByVerifiers.

```rust
pub fn verify_transition(
    q: &QState,
    tx: &VerifyTx,
    registry: &PredicateRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {

    // STAGE 1: target work_tx must exist + be in Pending or Provisional state
    let target = q.economic_state_t.claims_t.get(&tx.target_work_tx)
        .ok_or(TransitionError::TargetWorkTxNotFound)?;
    if !target.status.allows_verification() {
        return Err(TransitionError::TargetWorkTxNotVerifiable);
    }

    // STAGE 2: signature + bond
    if !verify_signature(&tx.signature, tx.canonical_digest()) {
        return Err(TransitionError::SignatureInvalid);
    }
    let verifier_balance = q.economic_state_t.balances_t.get(&tx.verifier_agent);
    if verifier_balance < tx.bond {
        return Err(TransitionError::StakeInsufficient);
    }

    // STAGE 3: predicate gate (verifier predicate, NOT same as work_tx acceptance)
    let verify_results = registry.run_verification(tx, target, q)?;
    if !verify_results.all_passed() {
        return Err(TransitionError::VerificationPredicateFailed(verify_results));
    }

    // STAGE 4: state transition
    let mut q_next = q.clone();
    q_next.economic_state_t.balances_t.debit(tx.verifier_agent, tx.bond);
    q_next.economic_state_t.stakes_t.lock_verifier_bond(tx.verifier_agent, tx.target_work_tx, tx.bond);
    q_next.economic_state_t.claims_t.add_verification(tx.target_work_tx, tx.verifier_agent, tx.verdict);

    // STAGE 5: append + materialize + signals
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle {
        boolean: vec![Signal::Boolean(BoolSignal::VerifiedAt(tx.tx_id))],
        statistical: vec![Signal::Statistical(StatSignal::ReputationDelta(tx.verifier_agent, +verify_reputation_delta(tx, target)))],
    };

    Ok((q_next, signals))
}
```

### 3.2 challenge_transition (ChallengeTx)

```rust
pub fn challenge_transition(
    q: &QState,
    tx: &ChallengeTx,
    registry: &PredicateRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {

    // STAGE 1: target work_tx must exist + still in challenge window
    let target = q.economic_state_t.claims_t.get(&tx.target_work_tx)
        .ok_or(TransitionError::TargetWorkTxNotFound)?;
    let window = q.economic_state_t.challenge_cases_t.get(tx.target_work_tx)
        .ok_or(TransitionError::ChallengeWindowClosed)?;
    // v1.4: use ChallengeWindow::is_open(now) per § 5.2.5; same rule used by finalize_reward
    if !window.is_open(tx.timestamp_logical) {
        return Err(TransitionError::ChallengeWindowClosed);
    }

    // STAGE 2: signature + NO_E stake
    if !verify_signature(&tx.signature, tx.canonical_digest()) {
        return Err(TransitionError::SignatureInvalid);
    }
    let challenger_balance = q.economic_state_t.balances_t.get(&tx.challenger_agent);
    if challenger_balance < tx.stake {
        return Err(TransitionError::StakeInsufficient);
    }

    // STAGE 3: counterexample acceptance predicate (the BURDEN OF PROOF predicate, Inv 7)
    let counterexample = cas::get(&tx.counterexample_cid)?;
    let counter_check = registry.run_counterexample_check(target, &counterexample, q)?;
    if !counter_check.proves_violation() {
        return Err(TransitionError::CounterexampleInsufficient(counter_check));
    }

    // STAGE 4: state transition — ROLLBACK target work_tx + slash original solver + reward challenger
    let mut q_next = q.clone();
    q_next.economic_state_t.balances_t.debit(tx.challenger_agent, tx.stake);

    // 4a: rollback target's provisional reward
    let rollback_amount = q.economic_state_t.claims_t.provisional_amount(tx.target_work_tx);
    q_next.economic_state_t.claims_t.mark_slashed(tx.target_work_tx, tx.tx_id);

    // 4b: slash original solver's stake → reward pool for challenger
    let solver_stake = q.economic_state_t.stakes_t.get(target.solver, target.task_id);
    q_next.economic_state_t.stakes_t.slash(target.solver, target.task_id);
    q_next.economic_state_t.escrows_t.deposit_from_slash(tx.challenger_agent, solver_stake);

    // 4c: challenger gets back NO_E stake + slashed solver stake
    q_next.economic_state_t.balances_t.credit(tx.challenger_agent, tx.stake + solver_stake);

    // 4d: solver reputation -= delta; challenger reputation += delta (Inv 9 immutable but we update via formula not transfer)
    q_next.economic_state_t.reputations_t.adjust(target.solver, -slash_reputation_delta());
    q_next.economic_state_t.reputations_t.adjust(tx.challenger_agent, +challenge_reputation_delta());

    // 4e: verifier_bond release per task config (gap 11.2 fix; default = return to good-faith verifier)
    //   Rationale: when Carol slashes Alice via challenge, Bob (the verifier) was duped but acted in good faith.
    //   Slashing Bob's bond would discourage future verification. Configurable per TaskMarket.
    //   Applies to ALL verifiers who voted Confirm on the slashed work_tx.
    let bond_release_policy = q.economic_state_t.task_markets_t
        .get(target.task_id)
        .map(|tm| tm.config.verifier_bond_on_slash)
        .unwrap_or(VerifierBondPolicy::ReturnToVerifier);
    for (verifier, bond) in q.economic_state_t.stakes_t.verifier_bonds_for(tx.target_work_tx) {
        match bond_release_policy {
            VerifierBondPolicy::ReturnToVerifier => {
                q_next.economic_state_t.balances_t.credit(verifier, bond);
                q_next.economic_state_t.stakes_t.release_verifier_bond(verifier, tx.target_work_tx);
            }
            VerifierBondPolicy::SlashedToChallenger => {
                q_next.economic_state_t.balances_t.credit(tx.challenger_agent, bond);
                q_next.economic_state_t.stakes_t.slash_verifier_bond(verifier, tx.target_work_tx);
                q_next.economic_state_t.reputations_t.adjust(verifier, -verifier_slash_delta());
            }
        }
    }

    // STAGE 5: close challenge window
    q_next.economic_state_t.challenge_cases_t.close(tx.target_work_tx, ChallengeOutcome::Slashed(tx.tx_id));

    // STAGE 6: append + materialize + signals
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle {
        boolean: vec![Signal::Boolean(BoolSignal::ChallengeUpheld(tx.tx_id))],
        statistical: vec![
            Signal::Statistical(StatSignal::ReputationDelta(target.solver, -slash_reputation_delta())),
            Signal::Statistical(StatSignal::ReputationDelta(tx.challenger_agent, +challenge_reputation_delta())),
        ],
    };

    Ok((q_next, signals))
}
```

### 3.3 reuse_transition (ReuseTx)

```rust
pub fn reuse_transition(
    q: &QState,
    tx: &ReuseTx,
    tool_registry: &ToolRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {
    // STAGE 1: tool must be registered + still active in L2
    let tool = tool_registry.get(tx.reused_tool_id)
        .ok_or(TransitionError::ToolNotInRegistry)?;
    if tool.creator != tx.reused_tool_creator {
        return Err(TransitionError::ToolCreatorMismatch);
    }

    // STAGE 2: parent reusing_work_tx must exist + Accepted
    let parent = q.economic_state_t.claims_t.get(&tx.reusing_work_tx)
        .ok_or(TransitionError::TargetWorkTxNotFound)?;
    if !parent.status.is_accepted_or_finalized() {
        return Err(TransitionError::ParentNotAcceptedYet);
    }

    // STAGE 3: state transition — add edge to royalty graph
    //   gap 11.3 fix: weight bounded by MAX_REUSE_ROYALTY_FRACTION = 0.10 default
    //   Rationale: 10% upper bound protects solver's primary reward. Builders earn via creating
    //   widely-reusable tools, not via single high-percentage extractions. Configurable per TaskMarket
    //   for cases where user wants to override (e.g., creator-economy experiments).
    let max_royalty = q.economic_state_t.task_markets_t
        .get(parent.task_id)
        .and_then(|tm| tm.config.max_reuse_royalty_fraction)
        .unwrap_or(MAX_REUSE_ROYALTY_FRACTION_DEFAULT);  // = 0.10 in micro-coin fractional repr (10000 / 100000)
    let bounded_weight = tool.reuse_royalty_share.min(max_royalty);
    if tool.reuse_royalty_share > max_royalty {
        log::warn!(
            "reuse_tx {}: tool {} declared royalty {} > max {}; clamping to {}",
            tx.tx_id, tx.reused_tool_id, tool.reuse_royalty_share, max_royalty, bounded_weight
        );
    }

    let mut q_next = q.clone();
    q_next.economic_state_t.royalty_graph_t.add_edge(
        from: tx.reusing_work_tx,
        to:   tx.reused_tool_id,
        creator: tx.reused_tool_creator,
        weight: bounded_weight,    // clamped per gap 11.3
    );

    // STAGE 4: append + materialize (no signals; royalty paid at finalize_reward time)
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    Ok((q_next, SignalBundle::empty()))
}
```

### 3.4 finalize_reward (challenge window expiry)

Triggered by tick (no agent submits this; runtime emits when challenge window expires for any provisional claim).

```rust
pub fn finalize_reward_transition(
    q: &QState,
    claim_id: ClaimId,
    settlement_engine: &SettlementEngine,
) -> Result<(QState, SignalBundle), TransitionError> {
    let claim = q.economic_state_t.claims_t.get(&claim_id)
        .ok_or(TransitionError::ClaimNotFound)?;
    let window = q.economic_state_t.challenge_cases_t.get(claim.target_work_tx);

    // STAGE 1: window must be expired AND no open slash
    // v1.4: invoke ChallengeWindow::is_open(now) per § 5.2.5 with explicit `now` arg;
    // same rule as challenge_transition stage 1
    if let Some(w) = window {
        if w.is_open(q.q_t.current_round) {
            return Err(TransitionError::ChallengeWindowStillOpen);
        }
        if w.outcome == Some(ChallengeOutcome::Slashed(_)) {
            return Err(TransitionError::AlreadySlashed);  // never finalize a slashed claim
        }
    }

    // STAGE 2: compute reward per Economic § 21 final formula
    let reward = settlement_engine.finalize(
        claim,
        Escrow::lookup(q, claim.task_id),
        Attribution::lookup(q, claim.target_work_tx),
        Survival::full,  // window expired without slash
        Utility::lookup(q, claim.target_work_tx),
        Constitution::check(q),
    )?;

    // STAGE 3: state transition
    let mut q_next = q.clone();
    let target = claim.target_work_tx_data;

    // 3a (v1.2 NEW; gap 11.A per Gemini + Codex Q2): unlock + return solver's stake
    // Without this, every successful solver permanently loses their stake → Inv 3 violation.
    let solver_stake_locked = q.economic_state_t.stakes_t.get(target.solver, target.task_id);
    q_next.economic_state_t.stakes_t.unlock(target.solver, target.task_id);
    q_next.economic_state_t.balances_t.credit(target.solver, solver_stake_locked);

    // 3b: credit reward + finalize claim + debit escrow
    q_next.economic_state_t.balances_t.credit(target.solver, reward);
    q_next.economic_state_t.claims_t.finalize(claim_id, reward);
    q_next.economic_state_t.escrows_t.debit(claim.task_id, reward);

    // 3c: pay royalties along royalty_graph_t edges (v1.2 explicit rounding rule per Codex Q3 + Q10)
    // Royalty math uses i64 micro-coin throughout; rounding = integer floor (round-down) to preserve Inv 3.
    // No f64; no implicit casts. weight stored as MicroFraction (i64 in 1_000_000 units representing 0.0..1.0).
    let reward_micro = reward.to_micro_units();    // i64
    for edge in q.economic_state_t.royalty_graph_t.edges_from(claim.target_work_tx) {
        let royalty_micro = reward_micro
            .checked_mul(edge.weight.micro_units())
            .expect("overflow")
            / 1_000_000;    // integer floor; deterministic across platforms
        let royalty = MicroCoin::from_micro_units(royalty_micro);
        q_next.economic_state_t.balances_t.credit(edge.creator, royalty);
        q_next.economic_state_t.balances_t.debit(target.solver, royalty);  // royalty comes from solver's reward, not extra mint (Inv 4)
    }
    // Note: integer floor means total royalty payments may be < `reward × Σ weights` by up to `n` micro-units (1 per edge);
    // the dust remains in solver's balance. This is intentional and consistent with Bitcoin satoshi rounding.

    // STAGE 4: emit terminal signals
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &FinalizeTx::from(claim_id, reward));
    q_next.state_root_t  = materializer::apply(&q.state_root_t, &FinalizeTx::from(claim_id, reward));
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    Ok((q_next, SignalBundle::finalize(claim_id, reward)))
}
```

### 3.6 task_expire_transition (v1.3 refactored: pure boundary preserved per Codex re-audit)

**Why**: a TaskMarket entry has a deadline; if no work_tx is accepted by deadline, the bounty MUST refund to task creator (otherwise Inv 3 monetary conservation broken: bounty trapped in escrow forever).

**v1.3 fix**: split runtime side effects (signing, logical time assignment) from pure transition. Runtime constructs `TaskExpireTx` BEFORE entering pure transition; pure `task_expire_transition` takes already-signed tx as argument. This restores § 2 + § 3 pure-boundary discipline (Codex Q1.3 + new-issue #1 fix).

```rust
// PURE transition (used by both branch A and branch B in STEP_B)
pub fn task_expire_transition(
    q: &QState,
    tx: &TaskExpireTx,    // v1.3: already-signed by runtime BEFORE entry
) -> Result<(QState, SignalBundle), TransitionError> {
    let task = q.economic_state_t.task_markets_t.get(tx.task_id)
        .ok_or(TransitionError::TaskNotFound)?;

    // STAGE 1: signature verification (system signature; not agent)
    if !verify_system_signature(&tx.system_signature, &tx, q.system_pubkey_at_epoch(tx.epoch)) {
        return Err(TransitionError::InvalidSystemSignature);
    }

    // STAGE 2: parent_state_root match (stale view rejection)
    if tx.parent_state_root != q.state_root_t {
        return Err(TransitionError::StaleParent);
    }

    // STAGE 3: expiry check — task must be expired AND have NO Pending OR Provisional OR Finalized claim
    // v1.3 fix (Codex new-issue #2): broaden race-protection from "Finalized only" to all claim statuses
    if task.deadline_logical_t > q.q_t.current_round {
        return Err(TransitionError::TaskNotExpired);
    }
    if q.economic_state_t.claims_t.any_claim_for_task(tx.task_id) {
        return Err(TransitionError::TaskHasOpenClaim);    // refund only if NO claim exists at all
    }

    // STAGE 4: refund bounty from escrow to task creator
    let mut q_next = q.clone();
    let bounty = q.economic_state_t.escrows_t.get(tx.task_id);
    q_next.economic_state_t.escrows_t.refund(tx.task_id);
    q_next.economic_state_t.balances_t.credit(task.creator, bounty);

    // STAGE 5: refund any solver stakes still locked on expired task
    for (agent, locked_stake) in q.economic_state_t.stakes_t.all_locked_for_task(tx.task_id) {
        q_next.economic_state_t.stakes_t.unlock(agent, tx.task_id);
        q_next.economic_state_t.balances_t.credit(agent, locked_stake);
    }

    // STAGE 6: remove task from active markets
    q_next.economic_state_t.task_markets_t.remove(tx.task_id);

    // STAGE 7: append + materialize + signal (purely on tx, q)
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle::task_expired(tx.task_id, bounty);

    Ok((q_next, signals))
}

// TaskExpireTx schema (v1.3 NEW typed schema):
pub struct TaskExpireTx {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub parent_state_root: Hash,
    pub bounty_refunded: MicroCoin,         // for ledger summary; runtime computes from q
    pub epoch: SystemEpoch,                  // which keypair signed
    pub timestamp_logical: u64,              // assigned by runtime BEFORE pure transition
    pub system_signature: SystemSignature,   // computed by runtime BEFORE pure transition
}
```

**Trigger**: runtime tick scans for expired tasks; for each, runtime:
1. Calls `runtime.next_logical_t()` to get next logical_t
2. Constructs `TaskExpireTx` with current `q.state_root_t` as parent
3. Signs `TaskExpireTx` via `runtime.system_keypair().sign(canonical_digest(&tx))`
4. Submits signed tx to L4 sequencer (§ 5.2.1)
5. Sequencer calls pure `task_expire_transition(q, &tx)`

This split is identical to how `WorkTx` is constructed by agent BEFORE submitting to pure `step_transition`. Agents construct + sign; runtime constructs + sign for system tx. Pure transition fn is `(q, tx) → (q', signals)` in BOTH cases.

### 3.6.5 Agent Implicit Init (v1.3 fixed: trait-based submitter resolution per Codex re-audit Q1.4)

**Where**: applies to ALL agent-submitted transitions (work_transition / verify_transition / challenge_transition / reuse_transition). Inline at stage 4 of each, before user-state mutations.

**v1.3 fix**: WorkTx has `agent_id`; VerifyTx has `verifier_agent`; ChallengeTx has `challenger_agent`; ReuseTx has no submitting-agent field (it's a fact-tx). Introduce a `Tx::submitter_id() -> Option<AgentId>` trait method that each tx implements explicitly:

```rust
pub trait HasSubmitter {
    fn submitter_id(&self) -> Option<AgentId>;
}

impl HasSubmitter for WorkTx       { fn submitter_id(&self) -> Option<AgentId> { Some(self.agent_id.clone()) } }
impl HasSubmitter for VerifyTx     { fn submitter_id(&self) -> Option<AgentId> { Some(self.verifier_agent.clone()) } }
impl HasSubmitter for ChallengeTx  { fn submitter_id(&self) -> Option<AgentId> { Some(self.challenger_agent.clone()) } }
impl HasSubmitter for ReuseTx      { fn submitter_id(&self) -> Option<AgentId> { None }    // ReuseTx has no submitting agent; reuse facts derive from L4 read_set }

// In each agent-submitted transition's stage 4, INLINE this snippet:
fn implicit_init_agent_if_new(q_next: &mut QState, tx: &impl HasSubmitter) {
    if let Some(submitter) = tx.submitter_id() {
        if !q_next.q_t.agents.contains_key(&submitter) {
            q_next.q_t.agents.insert(submitter, PerAgentState {
                reputation_snapshot: Reputation::default_initial(),    // = 0
                last_accepted_tx: None,
                retry_counter_for_current_task: 0,
            });
        }
    }
}
```

**Rule**: each transition function MUST call `implicit_init_agent_if_new(&mut q_next, tx)` as the FIRST statement of stage 4 (after stage 3 predicate gate, before any user-state mutation). For `ReuseTx`, `submitter_id()` returns None; no init happens; that's intentional (ReuseTx has no submitting agent to init).

**Why implicit (not explicit `register_agent_transition`)**:
- Satoshi parallel: Bitcoin addresses are implicitly created at first use; no separate register step
- Avoids gatekeeping: any agent submitting a valid signed tx joins the system
- v4 single-user friendly: gretjia + Codex/Gemini auto-discoverable
- v4.1+: if needed, can add explicit `agent_register_tx` later WITHOUT breaking implicit-init (new tx is purely additive)

### 3.7 emit_terminal_summary (run-end without acceptance)

```rust
pub fn emit_terminal_summary_transition(
    q: &QState,
    run_id: RunId,
    runtime: &Runtime,
) -> Result<(QState, SignalBundle), TransitionError> {
    let run = runtime.run_state(run_id)?;
    if run.has_accepted_work_tx() {
        return Err(TransitionError::TerminalSummaryNotApplicable);  // only emitted for no-accept runs
    }

    let summary = TerminalSummaryTx {
        tx_id: TxId::derive(run_id, "terminal"),
        task_id: run.task_id,
        run_id,
        run_outcome: run.outcome(),
        total_attempts: run.attempt_counter(),
        failure_class_histogram: run.failure_histogram(),
        last_logical_t: run.last_logical_t(),
        system_signature: runtime.system_keypair().sign(canonical_digest_terminal(run)),
    };

    // STAGE: append; materialize; emit failure-class signals to L6
    let mut q_next = q.clone();
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &summary);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, &summary);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle::terminal_summary(&summary);

    Ok((q_next, signals))
}
```

---

## § 4 Named Invariants (machine-checkable)

| ID | Invariant | Enforced at | Conformance test |
|---|---|---|---|
| I-DET | Same (Q_t, tx) → byte-identical (Q_{t+1}, signals) | step_transition stage 6-8 | `tests/transition_determinism.rs` |
| I-DETHASH | `state_root_t` after replay from genesis matches authoritative state | replay test | `tests/q_state_reconstruct.rs` |
| I-NOSIDE | step_transition reads only (q, tx, registries); no I/O | static analysis grep + cargo-deny | `tests/no_hidden_inputs.rs` |
| I-PARENT | tx.parent_state_root must equal q.state_root_t | stage 1 | `tests/stale_parent_rejection.rs` |
| I-SIG | tx.signature verifies against tx.canonical_digest() | stage 2 | `tests/signature_verification.rs` |
| I-STAKE | tx.stake ≤ q.balances_t[tx.agent_id]; debit atomic | stage 3, 6 | `tests/stake_atomicity.rs` |
| I-PRED-GATE | rejected work_tx does NOT advance state_root_t | stage 4 | `tests/economic_invariant_INV6_predicate_gated.rs` |
| I-PROV | accepted work_tx → provisional claim, NOT finalized reward | stage 5 | `tests/economic_invariant_INV7_provisional_then_final.rs` |
| I-LOGTIME | timestamp_logical strictly monotonic per-tx; no wall clock | stage 6 | `tests/no_wall_clock_in_tx.rs` |
| I-MICROCOIN | all monetary fields are MicroCoin (i64 newtype) | type system | compile-time + `tests/no_f64_money.rs` |
| I-BTREE | Q_t indices use BTreeMap, not HashMap (deterministic order) | type system | `tests/q_state_uses_btree.rs` |
| I-NOSIDECAR | no Vec/HashMap "graveyard"-like sidecar (Art. 0.2) | static analysis | `tests/no_rejection_sidecar.rs` |
| I-RETRY | RejectedAttemptSummary stamped by runner, not agent | stamp call site | `tests/retry_summary_runner_signed.rs` |
| I-TERMINAL | every run terminates with at least one of: accepted work_tx OR TerminalSummaryTx | run finalize hook | `tests/run_terminal_invariant.rs` |
| I-NOENV | step_transition dependency tree contains no `std::env` access | cargo-deny + grep | `tests/no_env_in_transition.rs` |
| I-FREEZE-CONFIG | TAPE_ECONOMY_V2 + FOUNDER_GRANT_GAMMA + system_lp_amount frozen at task creation, not at tx submission | TaskMarket::publish | `tests/task_config_frozen_at_publish.rs` |
| **I-NORANDOM** (added per Gemini v3.2 review Q1) | Any tx that consumes randomness MUST seed PRNG from `(tx.tx_id, q.state_root_t)`; no system entropy in step_transition path | step_transition stages 1-7 | `tests/no_runtime_entropy.rs` |
| **I-VERIFY-LIVE** (added per Gemini v3.2 review Q10) | VerifyTx targets MUST be in Pending or Provisional state; cannot verify Accepted-and-finalized or Slashed | verify_transition stage 1 | `tests/verify_target_liveness.rs` |
| **I-CHAL-WINDOW** (added per Gemini v3.2 review Q10) | ChallengeTx must be received within target's challenge_cases_t window; no challenges after window close | challenge_transition stage 1 | `tests/challenge_window_enforced.rs` |
| **I-FINALIZE-EXCLUSIVE** (added) | FinalizeRewardTx and SlashTx are mutually exclusive per claim_id; system runtime serializes | finalize_reward_transition stage 2 | `tests/finalize_or_slash_exclusive.rs` |
| **I-VBOND-RELEASE** (v1.1, gap 11.2 fix) | Verifier bond release on slashed work_tx follows TaskMarket.config.verifier_bond_on_slash policy; default = `ReturnToVerifier`; verifier reputation NOT adjusted under default policy | challenge_transition stage 4e | `tests/verifier_bond_release.rs` |
| **I-ROYALTY-CAP** (v1.1, gap 11.3 fix) | reuse_tx edge weight ≤ TaskMarket.config.max_reuse_royalty_fraction (default 0.10); excess clamped + warning logged | reuse_transition stage 3 | `tests/royalty_cap_enforced.rs` |
| **I-STAKE-RETURN** (v1.2 NEW per Gemini Q2 + Codex Q2) | Successful unchallenged finalize_reward returns + unlocks solver's locked stake exactly once (in addition to reward credit). Test attempts double-claim. | finalize_reward_transition stage 3a | `tests/stake_return_on_finalize.rs` |
| **I-BOUNTY-REFUND** (v1.2 NEW per Gemini Q2 + Codex Q2) | task_expire_transition refunds full bounty to creator + refunds any locked solver stakes when no claim finalized by deadline | task_expire_transition stages 2-3 | `tests/bounty_refund_on_expire.rs` |
| **I-FINALIZE-BATCH-ORDER** (v1.3 corrected: single key throughout) | When N claims become finalizable at the same logical_t, finalize_tx emit order is `(expires_at_logical ASC, claim_id ASC)` — `claim_id` (NOT `target_work_tx`) used everywhere: invariant + § 5.2.3 + conformance test all consistent. | runtime finalize loop + § 5.2.3 | `tests/finalize_batch_order.rs` |
| **I-CHALLENGE-WINDOW-EDGE** (v1.3 finalize binding fixed) | Challenge window is `[opens_at, opens_at + duration_ticks)` — left-inclusive, right-exclusive. `is_open(now)` defined as `now < opens_at + duration_ticks`. **Both** challenge_transition stage 1 AND finalize_reward stage 1 MUST use `is_open(q.q_t.current_round)` (NOT a different rule). | challenge_transition + finalize_reward_transition | `tests/challenge_window_edge.rs` |
| **I-AGENT-INIT** (v1.2 NEW per Gemini Q2) | First appearance of agent in L4 transition tx implicitly initializes q_t.agents[id] with reputation=0; subsequent appearances do not re-initialize | work/verify/challenge/reuse_transition stage 4 | `tests/agent_implicit_init.rs` |

**Total: 27 invariants → 27 tests** (was 22 in v1.1; +5 in v1.2). Every transition test must pass before CO1.1.4 (bus.rs split) starts. STEP_B implementation comparison is "branch X conforms to spec" / "branch Y conforms to spec", not "branch X looks like branch Y".

---

## § 5 Optional TLA+ Skeleton (deferred to spec-gate audit)

For ordering + replay invariants (I-DET, I-DETHASH, I-LOGTIME), Codex suggested TLA+/PlusCal. ArchitectAI agrees with the suggestion but does NOT include the full model in v1 of this spec — it would balloon the doc. Skeleton:

```tla
EXTENDS Naturals, Sequences

VARIABLES q, ledger, signals

Init == /\ q = GenesisQState
        /\ ledger = <<>>
        /\ signals = <<>>

Step(tx) == /\ ValidParent(tx, q)
            /\ ValidSignature(tx)
            /\ StakeAvailable(tx, q)
            /\ AcceptancePredicates(tx, q)
            /\ q' = Apply(q, tx)
            /\ ledger' = Append(ledger, tx)
            /\ signals' = EmitSignals(q, tx, q')

Spec == Init /\ [][\E tx \in WorkTx : Step(tx)]_<<q, ledger, signals>>

\* Determinism: same input sequence → same final state
DeterminismProperty == \A seq1, seq2 \in Seq(WorkTx) :
    (seq1 = seq2) => (Replay(seq1) = Replay(seq2))
```

If CO P1 audit demands stronger guarantees, the TLA+ model is upgraded to a full PlusCal program with TLC model checking. For v4 scope, the type-level + conformance-test combination is deemed sufficient by Codex.

---

## § 5.1 v1.1 Walk-Through Gap Resolutions

Per `SPEC_WALKTHROUGH_v1_2026-04-27.md` § 11, four spec gaps were found. Resolution status:

| Gap | Issue | v1.2 Resolution | User-overridable |
|---|---|---|---|
| 11.1 | False-challenge reputation penalty undefined | **v1.2 (Codex Q10 fix)**: false_challenge_reputation_penalty is **fixed to 0 in v4** (NOT configurable). Pseudocode resolves contradiction: challenge_transition stage 3 returns `Err(CounterexampleInsufficient)` BEFORE any state mutation; no executable path for nonzero penalty → "configurable" prose retired. v4.1+ MAY add explicit `failed_challenge_penalty_transition` if needed. | NO (v4) |
| 11.2 | Verifier bond release policy on slashed claim | spec § 3.2 stage 4e ADDED with `VerifierBondPolicy::ReturnToVerifier` default | yes — `verifier_bond_on_slash` config |
| 11.3 | Royalty edge weight bound | spec § 3.3 stage 3 ADDED with `MAX_REUSE_ROYALTY_FRACTION_DEFAULT = 0.10` | yes — `max_reuse_royalty_fraction` config |
| 11.4 | Multi-verifier quorum aggregation | spec § 3.1 note ADDED with `verifier_quorum_required: usize = 1` default; full multi-verifier impl deferred to CO P2.7 | yes — set per TaskMarket |

All 4 gaps now have machine-checkable defaults. User can override 11.2/11.3/11.4 defaults via TaskMarket.config when creating tasks; the default applies if config field is missing. **11.1 (false-challenge penalty) is NOT user-overridable in v4** (fixed to 0; v4.1+ may introduce a separate `failed_challenge_penalty_transition` if needed).

---

## § 5.2 Concurrency Rules (v1.2 NEW per Codex Q6)

**Why**: spec § 3 pseudocode is single-threaded; CO P1 may parallelize Phase C 5 modes × N seeds. Without explicit serialization rule, two work_tx can race on same parent_state_root, both pass `I-PARENT`, but produce different (logical_t, tx_id) sequences across STEP_B branches → state_root divergence.

### 5.2.1 L4 Sequencer

**Per (runtime_repo, run_id)** there is exactly ONE L4 sequencer instance. The sequencer:
1. **Receives** tx submissions in any order (concurrent-safe queue)
2. **Assigns** monotonic `(logical_t, tx_id)` ordering key:
   - `logical_t = sequencer.next_logical_t()` (atomic counter; starts at 1 per genesis)
   - `tx_id = TxId::derive(logical_t, agent_id, payload_hash)` (deterministic from above)
3. **Serializes** transition execution: takes 1 tx at a time from queue in submission order; calls pure `step_transition`
4. **Commits** result to L4 (WAL write + git commit) BEFORE accepting next tx

**Async completion order is NEVER an ordering source**. Even if async tasks finish out-of-order, sequencer enforces submission-order ingestion.

### 5.2.2 Cross-Cell Isolation

**Phase C 5-mode × 10-problem × N-seed cells** (per `CO1_3_1_GIX_SPIKE_PREFLIGHT § 1` C4) MUST use:
- **Disjoint `runtime_repo`** (different filesystem path; no shared state)
- **Disjoint `QState`** (each cell has its own genesis_payload + Q_t replay)
- **No shared L4 sequencer** (each cell has its own)

If a future deployment shares runtime_repo across cells (e.g., multi-tenant): MUST add **ref locks** (gix branch refs serve as atomic guards) + **deterministic retry semantics** (failed lock → wait 100ms × n_attempts; deterministic seed from `(run_id, tx_id)`).

### 5.2.3 Finalize Batch Order

When N claims expire at the same `logical_t`:
- Order = `(claim.expires_at_logical ASC, claim.claim_id ASC)` (stable, deterministic) — v1.3 fix: uses `claim_id` consistently (NOT `target_work_tx`) to align with `I-FINALIZE-BATCH-ORDER` invariant + conformance test
- Sequencer emits `finalize_reward_transition` ONE AT A TIME in this order
- Each finalize advances state_root before next finalize starts

### 5.2.4 Conformance Tests

- `tests/l4_sequencer_serialization.rs` — concurrent submit; assert single-threaded execution by sequencer; same input order → same state_root
- `tests/cross_cell_isolation.rs` — 5 cells run; assert disjoint state_roots; no cross-contamination
- `tests/finalize_batch_order.rs` — 3 claims expire same tick; assert ordering by (expires_at, claim_id); 2 runs byte-identical

### 5.2.5 ChallengeWindow::is_open (v1.4 NEW per Codex Q2.4)

```rust
impl ChallengeWindow {
    /// Half-open interval `[opens_at, opens_at + duration_ticks)`.
    /// Both challenge_transition stage 1 AND finalize_reward stage 1 MUST invoke this method
    /// (NOT hand-code the inequality) to guarantee consistent edge semantics.
    pub fn is_open(&self, now: u64) -> bool {
        now >= self.opens_at && now < self.opens_at + self.duration_ticks
    }
}
```

**Invariant binding**: `I-CHALLENGE-WINDOW-EDGE` enforces that BOTH transition functions call `is_open(now)` rather than hand-coding the boundary check. STEP_B branch A vs branch B both implement the same `is_open()`; cross-branch comparison verifies identical results for all (opens_at, duration_ticks, now) triples.

### 5.2.6 Sequencer Tie-Break (v1.4 NEW per Codex Q6)

When multiple agent threads concurrently call sequencer's `submit(tx)`, the sequencer's atomic `next_logical_t()` (§ 5.2.1 step 2) provides the **canonical tie-breaker**:

- `logical_t` assignments are produced by atomic increment (e.g., `AtomicU64::fetch_add`)
- The order in which threads receive their `logical_t` values IS the canonical ordering
- "Submission order" = the order of `logical_t` assignment, NOT wall-clock arrival order
- For two `tx` arriving at the same nanosecond on different threads, whichever thread wins the atomic gets the lower `logical_t`; the other gets the next higher

This means: STEP_B branch A and branch B may serialize threads differently (depending on OS scheduler), but as long as both branches use atomic logical_t assignment + replay from the SAME logical_t sequence, they produce byte-identical state_roots.

**Conformance test addition** (extends `tests/l4_sequencer_serialization.rs`): submit 100 tx concurrently from 8 threads; assert `(logical_t, tx_id_hash)` is a strict total order; replay produces deterministic state_root regardless of thread interleaving.

### 5.2.7 What This Does NOT Specify

- Async runtime choice (tokio vs std::thread): runtime concern, not spec; spec only requires sequencer property
- Sequencer implementation: lock-free queue, mutex, channel — implementation detail
- Cross-cell sharing pattern (post-v4): future v4.x extension

## § 5.3 Legacy Economic Tx Disposition (v1.3 NEW per Codex Q1.1)

The current pre-CO-P1 codebase contains economic mutation surfaces in `src/bus.rs` and `src/kernel.rs` that have NO direct equivalent in v1.x typed transitions:

| Legacy mutation | Current location | v4 disposition |
|---|---|---|
| `Invest` event (agent stakes Coin to YES/NO market position) | `src/bus.rs:229-252,285-290` `handle_invest_only` + market interactions | **RETIRED in CO1.1.4** — agent staking now goes through `WorkTx.stake` (YES_E) or `ChallengeTx.stake` (NO_E); no separate InvestTx. |
| `TaskMarketPublish` (task creator publishes new task) | implicit in current code; tasks hardcoded | **NEW v1 transition (deferred to CO P2.1)** — `TaskMarketPublishTx` lands in CO P2.1 atom; v1.x spec stubs the schema only |
| `MarketCreate` (per-node market on each tape append) | `src/bus.rs:285-290` + `src/kernel.rs:114-126` `Kernel::create_market` | **RETIRED in CO1.1.5** — per-node markets are an artifact of the Phase A "every node = market" pattern; CO P2.1 TaskMarket replaces with per-task markets only |
| `MarketResolve` (settle markets at OMEGA accept) | `src/kernel.rs:156-206` `Kernel::resolve_all` | **RETIRED in CO1.1.5** — market resolution becomes part of `finalize_reward_transition` (per-task, per-claim); no separate market-resolve event |
| `RunEnd` / `halt_and_settle` (run-level settlement) | `src/bus.rs:355-375` `TuringBus::halt_and_settle` | **RETIRED in CO1.1.4** — run-end becomes implicit via `TerminalSummaryTx` (§ 3.7) for no-accept runs OR `finalize_reward_transition` for accepted runs |
| WAL append side effect | `src/bus.rs:273-282` + `:319-327` | **MOVED to runtime layer**, not transition: spec § 5.2.1 sequencer commits L4 entries AFTER pure `step_transition` returns |
| Tool post-append hook | `src/bus.rs:312-318` `tool.on_post_append()` | **RETIRED**: tool hooks become explicit ToolInvocation field in `WorkTx.write_set` (read by predicate runner); no separate hook |

**Conformance test**: `tests/legacy_economic_tx_retired.rs` greps post-CO1.1.4/CO1.1.5 codebase for: `Invest` event variant, `Kernel::create_market`, `Kernel::resolve_all`, `halt_and_settle`, `tool.on_post_append`. Each must return 0 hits in the new `src/{top_white,middle_black,bottom_white,economy,state,transition}/*` dirs (matches in old `src/{bus,kernel}.rs` ARE expected if those files still exist as legacy markers; CO1.1.4 atom retires them).

**Why retired-not-renamed**: each legacy operation is either (a) absorbed into a v1.x typed transition (Invest → WorkTx.stake; Resolve → finalize_reward) OR (b) moved to runtime layer (WAL append; tool hook). Direct rename would preserve the old monolithic semantics.

## § 6 What This Spec DOES NOT Specify

Listed for honesty:

1. **MetaTx full schema** — only stub here; v4.1 atom defines.
2. **AttributionEngine deterministic DAG construction** — CO2.4.0 spike (separate doc).
3. **Predicate visibility leak channels** — covered at CO P1.5 design (Goodhart shield); this spec only declares `BoolWithProof.proof_visibility_class`, not the leak-proof proof format.
4. **gix Path B substrate-specific operations** — CO1.3.1 spike validates; this spec is substrate-agnostic.
5. **Retry metadata bound on `failed_attempts_since_last_accept`** — must be finite for tape size containment, but exact bound (e.g., u32::MAX vs cap-at-1000) is CO P1.7 design choice.
6. **Verifier verdict aggregation rule** — when N verifiers vote, how to combine? CO2.7 design.
7. **Challenge window length** — `CHALLENGE_WINDOW_TICKS` is a TaskMarket config bound at publish, but the default value + bounds are CO2.5 design.

These deferrals are **explicit and named**. Future atoms reference this list to resolve them.

---

## § 7 Pre-CO P1 Gate Procedure

1. ArchitectAI commits this spec v1
2. Codex independent review: confirm that every WP § 4-7 + economic § 2/§ 6 / § 18-21 concept maps to a typed field or invariant here
3. Gemini cross-review: confirm spec respects ENTIRE white paper (not just cited §)
4. Both PASS → spec frozen as v1 (any change requires re-audit)
5. **Then** Plan v3.2 atom CO1.SPEC.0 marked complete; CO1.0 / CO1.1.* / CO1.2.* atoms cleared to start
6. STEP_B implementation: Claude implements branch A against spec; Codex implements branch B against spec; comparison metric = "spec conformance", not "code similarity"

---

## § 8 Honest Acknowledgements

What this spec is:
- A typed, deterministic, side-effect-free state transition definition
- A binding contract for STEP_B branch A/B comparison
- A list of **27 named invariants** (was 16 in v1; 22 in v1.1; +5 in v1.2: I-STAKE-RETURN / I-BOUNTY-REFUND / I-FINALIZE-BATCH-ORDER / I-CHALLENGE-WINDOW-EDGE / I-AGENT-INIT) each backed by a conformance test path

What this spec is NOT:
- A full formal proof (no Lean/Coq)
- A complete TLA+ model (skeleton only)
- A substitute for code review (still required per Protocol Hard rule 1+2)
- A guarantee that branches A/B will produce identical Rust code (only spec-equivalent code)

What this spec does NOT yet include and the user must decide:
- Whether to run full TLA+ TLC model check (~3-5 day effort) or stop at type+test level (Codex suggested optional)
- Whether `RejectionClass::Opaque` aggregation respects Goodhart shield in practice (deferred to CO P1.5)
- Whether to embed Art 0.2 mini-amendment (see `ART_0_2_REINTERPRETATION_2026-04-27.md`) BEFORE running this spec, or AFTER (depends on rejection-on-tape constitutional reading)

— ArchitectAI, 2026-04-27


---

# XREF: shipped system_keypair.rs (round-1 ground truth)

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemSignature([u8; SIGNATURE_LEN]);

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

/// TRACE_MATRIX FC1-Sig+FC3-Sig: typed terminal summary transaction emitted on no-accept runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalSummaryTx {
    run_id: String,
    terminal_state_root: [u8; 32],
    rejected_attempt_count: u64,
}

impl TerminalSummaryTx {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a typed terminal summary transaction.
    pub fn new(
        run_id: impl Into<String>,
        terminal_state_root: [u8; 32],
        rejected_attempt_count: u64,
    ) -> Self {
        Self {
            run_id: run_id.into(),
            terminal_state_root,
            rejected_attempt_count,
        }
    }
}

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
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: terminal summary transaction.
    TerminalSummaryTx(TerminalSummaryTx),
    /// TRACE_MATRIX FC3-Sig: system key epoch continuity proof.
    EpochRotationProof(EpochRotationProof),
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
        CanonicalMessage::TerminalSummaryTx(tx) => {
            h.update(b"TerminalSummaryTx");
            update_len_prefixed(&mut h, tx.run_id.as_bytes());
            h.update(tx.terminal_state_root);
            h.update(tx.rejected_attempt_count.to_be_bytes());
        }
        CanonicalMessage::EpochRotationProof(proof) => {
            h.update(b"EpochRotationProof");
            h.update(proof.old_epoch.get().to_be_bytes());
            h.update(proof.new_epoch.get().to_be_bytes());
            h.update(proof.old_pubkey.as_bytes());
            h.update(proof.new_pubkey.as_bytes());
            h.update(proof.signed_at_unix.to_be_bytes());
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

/// TRACE_MATRIX FC1-Sig+FC3-Sig: crate-only signing surface for terminal summary emission.
pub(crate) mod terminal_summary_emitter {
    use super::{
        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, EpochRotationProof,
        KeypairError, SystemSignature, TerminalSummaryTx,
    };

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: sign only typed terminal summary transactions.
    pub(crate) fn sign_terminal_summary_tx(
        keypair: &Ed25519Keypair,
        tx: &TerminalSummaryTx,
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(keypair, &CanonicalMessage::TerminalSummaryTx(tx.clone()))
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

Now give your INDEPENDENT round-2 audit. For each of your round-1 must-fix items, explicitly judge CLOSED/PARTIAL/REGRESSED.

codex
I’ll verify the pasted v1.1 against the workspace version and run the narrow ledger test target so the audit is anchored in code, not just the prompt. Then I’ll map each round-1 finding to CLOSED/PARTIAL/REGRESSED and call only real blockers.
exec
/bin/bash -lc 'pwd && git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4
 M rules/enforcement.log
?? handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md
?? handover/audits/GEMINI_CO1_7_ROUND2_AUDIT_2026-04-28.md
?? handover/audits/run_codex_co1_7_round2_audit.sh
?? handover/audits/run_gemini_co1_7_round2_audit.py

exec
/bin/bash -lc "rg --files -g 'AGENTS.md' -g 'CLAUDE.md' -g '*CO1_7_TRANSITION_LEDGER*' -g 'transition_ledger.rs' -g 'system_keypair.rs' -g 'store.rs' -g 'schema.rs' -g 'genesis_payload.toml'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/cas/schema.rs
src/bottom_white/cas/store.rs
src/bottom_white/ledger/transition_ledger.rs
src/bottom_white/ledger/system_keypair.rs
genesis_payload.toml
CLAUDE.md
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md

codex
The worktree has untracked round-2 audit artifacts and an enforcement log change; I’m treating those as pre-existing and not modifying them. I don’t see an `AGENTS.md` file under the repo from the file search, so the user-provided instructions are the active AGENTS guidance here.
exec
/bin/bash -lc "nl -ba handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# CO1.7 Transition Ledger v1.1 — Round-1 closure
     2	
     3	**Status**: v1.1 — round-1 dual external audit (Codex + Gemini) returned CHALLENGE/CHALLENGE; this version closes 11 must-fix items, awaiting round-2.
     4	**Author**: ArchitectAI (Claude); session 2026-04-28.
     5	**Supersedes**: v1 (2026-04-28 morning DRAFT outline).
     6	**Round-1 verdicts**: `handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md` + `handover/audits/GEMINI_CO1_7_ROUND1_AUDIT_2026-04-28.md`; merged in `handover/audits/CO1_7_DUAL_AUDIT_VERDICT_R1_2026-04-28.md`.
     7	
     8	**Companion specs** (frozen, read first):
     9	- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — typed schemas + step_transition pseudocode + 27 invariants (round-4 PASS/PASS)
    10	- `SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md` — runtime keypair lifecycle (CO1.7.0a-f, done @ Wave 4-B)
    11	- `META_TRANSITION_INTERFACE_v1_2026-04-27.md` — trait pattern for L4 acceptance (deferred runtime to v4.1)
    12	- `TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md` § 5.L4 — ChainTape Layer 4 axioms
    13	
    14	**Single sentence**: implement the L4 transition_ledger module so that `ledger::append(parent_root, signing_digest) → new_root` (called from sequencer) is real code, the L4 sequencer (§ 5.2.1) is real code, and `Q_t.ledger_root_t` is no longer a placeholder.
    15	
    16	---
    17	
    18	## v1.1 patch log (vs. v1)
    19	
    20	| ID | v1 issue | v1.1 fix | Source |
    21	|---|---|---|---|
    22	| C1 | replay was single-mode; called "I-DETHASH witness" but skeleton only did chain check | Two-mode `ReplayMode::ChainOnly` (skeleton-stage) vs `ReplayMode::FullTransition` (CO1.7.5+; I-DETHASH witness only in this mode) | Codex Q-D + Gemini Q3 |
    23	| C2 | spec did not acknowledge that shipped `CasStore::open()` initializes empty in-memory index → cold-replay impossible | § 0 + § 5 explicit dependency + mitigation: CasStore index persistence is deferred to **CO1.4-extra** (separate atom); v1 documents the gap | Codex Q-H + Gemini Q2 |
    24	| C3 | signing primitive integration via `CanonicalMessage` enum was unspecified; spec called nonexistent digest-form verifier | Path A: extend `CanonicalMessage::LedgerEntrySigning(LedgerEntrySigningPayload)`; sign separate signing payload (NOT raw `LedgerEntry`); new API `keypair.sign_ledger_entry(payload) → SystemSignature` | Codex Q-G + Gemini Q4 |
    25	| K1 | sequencer `next_logical_t.fetch_add(1)` happens BEFORE accept; rejection skips `logical_t`, replay rejects gaps | Dual counter design: `next_submit_id` advances at submit; `next_logical_t` advances ONLY on commit | Codex Q-C |
    26	| K2 | signature did NOT bind `parent_ledger_root` → transplant attack | `LedgerEntrySigningPayload` includes `parent_ledger_root` field | Codex Q-B (NEW) |
    27	| K3 | L4/L5 head_t ownership inconsistent (spec line 194 vs 276 disagreed) | CO1.7 owns `ledger_root_t` + commit-chain `head_t = NodeId(commit_sha)` only; L5 (CO1.8) owns `state_root_t` mutation; sequencer drops `head_t = NodeId::from_state_root(...)` line | Codex Q-E |
    28	| K4 | spec `LedgerWriter::commit(&self) → NodeId` + `iter_from` did not match skeleton `commit(&mut self) → Hash` | Spec aligned to skeleton: `&mut self` + `Hash` return; `iter_from` deferred to CO1.7.5+ when needed for cold-replay | Codex Q-H |
    29	| K5 | `TxKind::Slash` enum variant present but `dispatch_transition` omitted it | Drop `TxKind::Slash` for v4; ChallengeCourt slashing event scheduled for CO P2.5 atom | Codex Q-H |
    30	| K6 | `tx_kind as u8` cast without `#[repr(u8)]` → fragile discriminant | `#[repr(u8)]` + explicit discriminants (`Work = 0, Verify = 1, ...`) | Codex Q-H |
    31	| K7 | spec promised 8 conformance tests; skeleton has 6 | Explicit list of 8 tests with skeleton-stage vs CO1.7.5-stage marker; unimplemented stubs now stage-marked | Codex Q-H |
    32	| G1 | `LedgerEntry` struct rigid; future ZK / settlement proof had no place | Add `extensions: BTreeMap<String, Vec<u8>>` (empty in v1; reserved for v4.x without breaking schema) | Gemini Q9 |
    33	| D1 | epoch binding disagreement (Codex bind YES; Gemini bind NO) | Conservative resolution: epoch IS bound in `LedgerEntrySigningPayload`; epoch NOT separately folded into `ledger_root_t` (Codex security wins; Gemini orthogonality preserved at the ledger_root axis) | merged verdict § 5 |
    34	
    35	11 must-fix + 1 disagreement resolution = **12 closures** integrated below.
    36	
    37	---
    38	
    39	## § 0 Scope
    40	
    41	### In scope (CO1.7 atom)
    42	- **LedgerEntry schema**: canonical envelope wrapping each typed transition (WorkTx / VerifyTx / ChallengeTx / ReuseTx / FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx) before append to L4. **Note**: `Slash` is NOT in v4 (deferred to CO P2.5 ChallengeCourt atom — K5).
    43	- **LedgerEntrySigningPayload**: the 8-field bytes-on-the-wire that the system keypair actually signs (distinct from LedgerEntry-the-stored-record).
    44	- **LedgerRoot computation**: deterministic Merkle accumulation over signed digests; this is the value of `Q_t.ledger_root_t`.
    45	- **Sequencer**: per-(runtime_repo, run_id) single-writer instance enforcing § 5.2.1 (dual-counter `submit_id`/`logical_t`, submission-order serialization, post-commit `logical_t` assignment).
    46	- **append(parent_root, signing_digest)**: pure function returning the new ledger_root.
    47	- **replay (two-mode)**: `ChainOnly` (chain integrity; skeleton-stage; v1) vs `FullTransition` (rerun pure transitions + verify state_root + verify signatures; CO1.7.5+; THE I-DETHASH witness).
    48	- **Storage backend**: git2-rs commit chain (built on CO1.4 CAS); each LedgerEntry = one git commit on `refs/transitions/main`; commit_sha is the canonical `head_t`.
    49	- **CanonicalMessage extension**: extends shipped `CanonicalMessage` enum with `LedgerEntrySigning(LedgerEntrySigningPayload)` variant; new sign API `keypair.sign_ledger_entry(payload)`.
    50	
    51	### Out of scope (handled by other atoms)
    52	- WorkTx / VerifyTx / ChallengeTx / ReuseTx / FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx schemas — frozen in `STATE_TRANSITION_SPEC § 1`.
    53	- step_transition / verify_transition / challenge_transition logic — frozen in `STATE_TRANSITION_SPEC § 3`.
    54	- system_keypair signing primitives — done @ CO1.7.0a-f; CO1.7 only adds a typed extension.
    55	- L5 materializer (state_root computation) — deferred to **CO1.8**. **K3 boundary**: CO1.7 owns `ledger_root_t` + `head_t`; CO1.8 owns `state_root_t`. Sequencer does NOT mutate `state_root_t` directly; it accepts `q_next.state_root_t` as returned by the transition function.
    56	- L6 signal indices — deferred to **CO1.9**.
    57	- AttributionEngine DAG — deferred to CO P2.4.0 spike (Inv 8 design).
    58	- MetaTx full schema — v4.1 only; v4 emits `MetaProposalDraft` to L3 CAS, not L4.
    59	- **Slash transition** — deferred to CO P2.5 (ChallengeCourt) atom; v4 ledger has no `TxKind::Slash`.
    60	- **CAS index persistence (cold-replay enabler)** — `CasStore::open()` shipped at Wave 3 initializes empty in-memory index ([store.rs:67](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs)); cold-replay therefore cannot recover payloads via `CasStore::get` after restart. **CO1.4-extra** atom (NEW, scheduled post-CO1.7) adds index persistence (likely a sidecar JSONL or git-tag manifest). v1 ledger documents the dependency; full-mode replay is implementable once CO1.4-extra lands.
    61	
    62	### What this spec is NOT replacing
    63	- `src/ledger.rs` (legacy, top-level) is retired in **CO1.1.5 (kernel.rs split)**; CO1.7 lives at `src/bottom_white/ledger/transition_ledger.rs` (NEW). No STEP_B parallel-branch ceremony required (new module, not restricted file); restricted files per CLAUDE.md "Code Standard" are `src/{kernel,bus,wallet}.rs` (corrected from v1's incorrect `wal.rs` per K6 tail).
    64	
    65	---
    66	
    67	## § 1 LedgerEntry schema (the stored record)
    68	
    69	```rust
    70	use std::collections::BTreeMap;
    71	
    72	/// TRACE_MATRIX FC2-Append (FC2 transition machinery): canonical record
    73	/// stored at L4. One LedgerEntry per accepted transition. Genesis state has
    74	/// zero LedgerEntries; ledger_root_t = genesis_ledger_root_t (per § 5).
    75	#[derive(Debug, Clone, PartialEq, Eq)]
    76	pub struct LedgerEntry {
    77	    /// Monotonic counter from sequencer; starts at 1 at first accept.
    78	    /// **K1**: assigned ONLY at commit; rejected submissions never get a logical_t.
    79	    pub logical_t: u64,                          //  1
    80	
    81	    /// Parent state_root before this transition. Equals the resulting_state_root
    82	    /// of the entry at logical_t-1 (or genesis state_root at logical_t=1).
    83	    pub parent_state_root: Hash,                 //  2
    84	
    85	    /// **K2 NEW**: parent_ledger_root before this entry is folded in.
    86	    /// Bound by signature (transplant attack defense). Equals the
    87	    /// resulting_ledger_root of entry at logical_t-1 (or genesis_ledger_root at logical_t=1).
    88	    pub parent_ledger_root: Hash,                //  3
    89	
    90	    /// Discriminator; payload schema depends on this. **K6**: `#[repr(u8)]` for stable
    91	    /// discriminant in canonical digest computation.
    92	    pub tx_kind: TxKind,                         //  4
    93	
    94	    /// CAS handle (CO1.4) to canonically-serialized payload. Sequencer puts payload
    95	    /// to CAS via `CasStore::put(content, object_type, creator, created_at_logical_t, schema_id)`
    96	    /// (DIV-5: 5-param signature). cid = sha256(content).
    97	    pub tx_payload_cid: Cid,                     //  5
    98	
    99	    /// Resulting state_root after `dispatch_transition` applied. NOT mutated by L4
   100	    /// — accepted as-returned from the transition function (K3 boundary).
   101	    pub resulting_state_root: Hash,              //  6
   102	
   103	    /// Resulting ledger_root after this entry is folded in.
   104	    /// Convention: ledger_root_{t+1} = sha256(domain_sep || parent_ledger_root || signing_digest_t)
   105	    /// where signing_digest_t = canonical_digest(LedgerEntrySigningPayload at logical_t).
   106	    /// **NOT signed** — derivative; including it in signed digest creates a cycle (Q9).
   107	    pub resulting_ledger_root: Hash,             //  7
   108	
   109	    /// Wall-clock-free timestamp; equal to `logical_t` post-commit (no separate clock).
   110	    /// Field retained for symmetry with STATE_TRANSITION_SPEC § 1.2 WorkTx.
   111	    pub timestamp_logical: u64,                  //  8
   112	
   113	    /// **DIV-3 / Q10**: which pinned epoch pubkey signed this entry. Required by
   114	    /// `system_keypair::verify_system_signature(sig, msg, epoch, pinned_pubkeys)`.
   115	    /// Bound in signed payload (Codex security argument; **D1** resolved).
   116	    pub epoch: SystemEpoch,                      //  9
   117	
   118	    /// **G1 NEW**: forward-compatibility extension map. Empty in v1; reserved for
   119	    /// v4.x additions (e.g. ZK predicate proofs, settlement proofs, public-market metadata).
   120	    /// Bound in signed payload (so additions cannot bypass signature).
   121	    pub extensions: BTreeMap<String, Vec<u8>>,   // 10
   122	
   123	    /// Detached system signature over canonical_digest of LedgerEntrySigningPayload.
   124	    /// Distinct from agent-signature inside payload. NOT included in signed digest.
   125	    pub system_signature: SystemSignature,       // 11
   126	}
   127	
   128	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
   129	#[repr(u8)]
   130	pub enum TxKind {
   131	    Work            = 0,   // WorkTx       (STATE spec § 1.2)
   132	    Verify          = 1,   // VerifyTx     (STATE spec § 1.3)
   133	    Challenge       = 2,   // ChallengeTx  (STATE spec § 1.3)
   134	    Reuse           = 3,   // ReuseTx      (STATE spec § 1.3)
   135	    FinalizeReward  = 4,   // FinalizeRewardTx (STATE spec § 3.4)
   136	    TaskExpire      = 5,   // TaskExpireTx (STATE spec § 3.6)
   137	    TerminalSummary = 6,   // TerminalSummaryTx (STATE spec § 1.5 + § 3.7)
   138	    // K5: NO `Slash` — ChallengeCourt slash event is in CO P2.5; v4 ledger has no Slash variant.
   139	}
   140	```
   141	
   142	### § 1.1 LedgerEntrySigningPayload (NEW per C3)
   143	
   144	The system signature signs a **separate struct**, not the LedgerEntry directly. This:
   145	1. **Excludes derivatives**: `resulting_ledger_root` (cycle: ledger_root ⊃ digest ⊃ ledger_root) and `system_signature` (its own input) are NOT in the signing payload.
   146	2. **Binds non-derivatives**: `parent_ledger_root` (K2 transplant defense) + `epoch` (D1 + Q10) + `extensions` (G1 forward compat is signed).
   147	3. **Has stable wire format**: explicit byte layout (see canonical_digest below), independent of bincode/serde choices.
   148	
   149	```rust
   150	#[derive(Debug, Clone, PartialEq, Eq)]
   151	pub struct LedgerEntrySigningPayload {
   152	    pub logical_t: u64,                         // 1
   153	    pub parent_state_root: Hash,                // 2
   154	    pub parent_ledger_root: Hash,               // 3 (K2 transplant defense)
   155	    pub tx_kind: TxKind,                        // 4 (#[repr(u8)] discriminant cast safe)
   156	    pub tx_payload_cid: Cid,                    // 5
   157	    pub resulting_state_root: Hash,             // 6
   158	    pub timestamp_logical: u64,                 // 7
   159	    pub epoch: SystemEpoch,                     // 8 (D1)
   160	    pub extensions: BTreeMap<String, Vec<u8>>,  // 9 (G1; empty-map case yields empty bytes)
   161	    // EXCLUDED: resulting_ledger_root (cycle); system_signature (its input).
   162	}
   163	
   164	impl LedgerEntrySigningPayload {
   165	    pub fn canonical_digest(&self) -> Hash {
   166	        let mut h = Sha256::new();
   167	        h.update(b"turingosv4.ledger_entry_signing.v1");      // domain separation
   168	        h.update(self.logical_t.to_be_bytes());
   169	        h.update(self.parent_state_root.0);
   170	        h.update(self.parent_ledger_root.0);
   171	        h.update((self.tx_kind as u8).to_be_bytes());          // K6 #[repr(u8)] makes this stable
   172	        h.update(self.tx_payload_cid.0);
   173	        h.update(self.resulting_state_root.0);
   174	        h.update(self.timestamp_logical.to_be_bytes());
   175	        h.update(self.epoch.get().to_be_bytes());
   176	        // extensions: BTreeMap iteration is sorted by key (deterministic);
   177	        // length-prefix each (key, value) pair to prevent ambiguity.
   178	        h.update((self.extensions.len() as u64).to_be_bytes());
   179	        for (k, v) in &self.extensions {                        // BTreeMap = lex order
   180	            h.update((k.len() as u64).to_be_bytes());
   181	            h.update(k.as_bytes());
   182	            h.update((v.len() as u64).to_be_bytes());
   183	            h.update(v);
   184	        }
   185	        Hash(h.finalize().into())
   186	    }
   187	}
   188	```
   189	
   190	### § 1.2 CanonicalMessage extension (per C3)
   191	
   192	CO1.7 extends shipped `system_keypair::CanonicalMessage` with one new variant:
   193	
   194	```rust
   195	// In src/bottom_white/ledger/system_keypair.rs (additive Wave 4-B extension):
   196	pub enum CanonicalMessage {
   197	    RejectedAttemptSummary(RejectedAttemptSummary),  // existing
   198	    TerminalSummaryTx(TerminalSummaryTx),            // existing
   199	    EpochRotationProof(EpochRotationProof),          // existing
   200	    LedgerEntrySigning(LedgerEntrySigningPayload),   // NEW (C3)
   201	}
   202	
   203	// canonical_digest() in system_keypair.rs adds match arm:
   204	//   CanonicalMessage::LedgerEntrySigning(payload) => {
   205	//       h.update(b"LedgerEntrySigning");
   206	//       h.update(payload.canonical_digest().0);
   207	//   }
   208	
   209	// New typed sign API (added to Ed25519Keypair impl block):
   210	impl Ed25519Keypair {
   211	    pub fn sign_ledger_entry(&self, payload: &LedgerEntrySigningPayload, epoch: SystemEpoch) -> SystemSignature;
   212	}
   213	```
   214	
   215	**Forward-compat clause** (per Gemini Q4 + the audit response): if v4.x adds new ledger-side message variants, they MUST add new `CanonicalMessage::*` variants (NOT extend `LedgerEntrySigningPayload` in-place). v4-shipped extensions go in the `LedgerEntry::extensions` BTreeMap (G1) which IS bound in this signed digest.
   216	
   217	---
   218	
   219	## § 2 Module layout
   220	
   221	```
   222	src/bottom_white/ledger/
   223	├── mod.rs                       (existing; v1.1 wires `pub mod transition_ledger`)
   224	├── system_keypair.rs            (existing CO1.7.0a-f; CO1.7 adds 1 enum variant + 1 typed sign fn — additive)
   225	└── transition_ledger.rs         (NEW; LedgerEntry, LedgerEntrySigningPayload, TxKind, append, replay_*, LedgerWriter)
   226	
   227	src/state/
   228	├── mod.rs                       (existing)
   229	├── q_state.rs                   (existing; CO1.7 fills `ledger_root_t` placeholder; does NOT touch `state_root_t` per K3)
   230	└── sequencer.rs                 (NEW; deferred to CO1.7.5; pre-audit type stub may land in v1.1 if useful)
   231	```
   232	
   233	**Crate boundary**: `transition_ledger` in `bottom_white::ledger` (tool layer); `sequencer` in `state::` (touches Q_t mutation). Sequencer DEPENDS ON ledger; ledger does NOT depend on sequencer (DAG: state → bottom_white::ledger → CO1.4 CAS → CO1.7.0a-f keypair).
   234	
   235	---
   236	
   237	## § 3 Sequencer (K1 dual-counter; K3 head_t ownership; C3 sign API)
   238	
   239	```rust
   240	/// TRACE_MATRIX § 5.2.1 — L4 sequencer; single-writer per (runtime_repo, run_id).
   241	pub struct Sequencer {
   242	    /// **K1 NEW**: separate counter for submissions (independent of accept).
   243	    /// Used to derive submit_id for SubmissionReceipt; never appears in LedgerEntry.
   244	    next_submit_id: AtomicU64,
   245	
   246	    /// **K1 changed semantics**: advances ONLY on commit, NOT on submit.
   247	    /// Genesis = 0; first accepted entry gets logical_t=1.
   248	    next_logical_t: AtomicU64,
   249	
   250	    /// **Q1 resolution**: bounded `tokio::sync::mpsc::Sender` (NOT unbounded).
   251	    /// Submit returns `QueueFull` Err on saturation; agents handle backoff.
   252	    queue_tx: tokio::sync::mpsc::Sender<TypedTx>,
   253	
   254	    /// CAS handle for payload storage.
   255	    cas: Arc<RwLock<CasStore>>,
   256	
   257	    /// **C3**: signing key handle (CO1.7.0a-f).
   258	    keypair: Arc<Ed25519Keypair>,
   259	    epoch: SystemEpoch,                   // current signing epoch
   260	

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
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
    34	
    35	use sha2::{Digest, Sha256};
    36	
    37	use crate::bottom_white::cas::schema::Cid;
    38	use crate::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
    39	use crate::state::q_state::Hash;
    40	
    41	// ────────────────────────────────────────────────────────────────────────────
    42	// § 1 LedgerEntry — the stored record (11 fields per v1.1)
    43	// ────────────────────────────────────────────────────────────────────────────
    44	
    45	/// TRACE_MATRIX FC2-Append: discriminator for the typed payload behind a CAS Cid.
    46	/// **K6**: `#[repr(u8)]` + explicit discriminants for stable cast in canonical digest.
    47	/// **K5**: NO `Slash` variant — ChallengeCourt slash event deferred to CO P2.5 atom.
    48	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    49	#[repr(u8)]
    50	pub enum TxKind {
    51	    Work            = 0,
    52	    Verify          = 1,
    53	    Challenge       = 2,
    54	    Reuse           = 3,
    55	    FinalizeReward  = 4,
    56	    TaskExpire      = 5,
    57	    TerminalSummary = 6,
    58	}
    59	
    60	/// TRACE_MATRIX FC2-Append + WP § 5.L4: stored LedgerEntry record (11 fields).
    61	///
    62	/// Distinct from `LedgerEntrySigningPayload`: this is the FULL stored record
    63	/// (includes derivatives + signature); the signing payload is the subset that
    64	/// the system keypair attests.
    65	#[derive(Debug, Clone, PartialEq, Eq)]
    66	pub struct LedgerEntry {
    67	    /// **K1**: assigned ONLY at commit (sequencer dual-counter design); rejected
    68	    /// submissions never get a logical_t.
    69	    pub logical_t: u64,                          //  1
    70	    pub parent_state_root: Hash,                 //  2
    71	    /// **K2 NEW**: parent_ledger_root before fold; bound in signed payload to
    72	    /// prevent transplant attacks.
    73	    pub parent_ledger_root: Hash,                //  3
    74	    pub tx_kind: TxKind,                         //  4
    75	    /// CAS handle (CO1.4) to canonical-serialized payload (DIV-5 5-param put).
    76	    pub tx_payload_cid: Cid,                     //  5
    77	    /// Resulting state_root post-transition (NOT mutated by L4 — accepted as
    78	    /// returned by transition function per K3 boundary).
    79	    pub resulting_state_root: Hash,              //  6
    80	    /// Resulting ledger_root after fold. Derivative; NOT in signed digest.
    81	    pub resulting_ledger_root: Hash,             //  7
    82	    pub timestamp_logical: u64,                  //  8
    83	    /// **D1 / Q10**: epoch bound in signed payload (Codex security wins).
    84	    pub epoch: SystemEpoch,                      //  9
    85	    /// **G1 NEW**: forward-compat extension map. Empty in v1; reserved for v4.x.
    86	    /// Bound in signed payload (G1 cannot bypass signature).
    87	    pub extensions: BTreeMap<String, Vec<u8>>,   // 10
    88	    /// Detached system signature over `LedgerEntrySigningPayload.canonical_digest()`.
    89	    pub system_signature: SystemSignature,       // 11
    90	}
    91	
    92	// ────────────────────────────────────────────────────────────────────────────
    93	// § 1.1 LedgerEntrySigningPayload — the signed bytes (NEW per C3 / Q9)
    94	// ────────────────────────────────────────────────────────────────────────────
    95	
    96	/// TRACE_MATRIX FC2-Append C3: the bytes the system keypair actually signs.
    97	///
    98	/// **Excludes** (Q9 cycle prevention):
    99	/// - `resulting_ledger_root` (derivative; including → cycle)
   100	/// - `system_signature` (its own input)
   101	///
   102	/// **Includes** (9 non-derivative bound fields). Domain-separation prefix is
   103	/// part of the digest to prevent cross-namespace collision.
   104	#[derive(Debug, Clone, PartialEq, Eq)]
   105	pub struct LedgerEntrySigningPayload {
   106	    pub logical_t: u64,
   107	    pub parent_state_root: Hash,
   108	    pub parent_ledger_root: Hash,                  // K2
   109	    pub tx_kind: TxKind,
   110	    pub tx_payload_cid: Cid,
   111	    pub resulting_state_root: Hash,
   112	    pub timestamp_logical: u64,
   113	    pub epoch: SystemEpoch,                        // D1
   114	    pub extensions: BTreeMap<String, Vec<u8>>,     // G1
   115	}
   116	
   117	impl LedgerEntrySigningPayload {
   118	    /// Canonical SHA-256 digest. Stable wire format (NOT bincode/serde dependent).
   119	    pub fn canonical_digest(&self) -> Hash {
   120	        let mut h = Sha256::new();
   121	        h.update(b"turingosv4.ledger_entry_signing.v1");
   122	        h.update(self.logical_t.to_be_bytes());
   123	        h.update(self.parent_state_root.0);
   124	        h.update(self.parent_ledger_root.0);
   125	        h.update((self.tx_kind as u8).to_be_bytes()); // K6 #[repr(u8)] makes cast stable
   126	        h.update(self.tx_payload_cid.0);
   127	        h.update(self.resulting_state_root.0);
   128	        h.update(self.timestamp_logical.to_be_bytes());
   129	        h.update(self.epoch.get().to_be_bytes());
   130	        // Extensions: BTreeMap iterates in lex key order (deterministic);
   131	        // length-prefix every field to prevent ambiguity attacks.
   132	        h.update((self.extensions.len() as u64).to_be_bytes());
   133	        for (k, v) in &self.extensions {
   134	            h.update((k.len() as u64).to_be_bytes());
   135	            h.update(k.as_bytes());
   136	            h.update((v.len() as u64).to_be_bytes());
   137	            h.update(v);
   138	        }
   139	        Hash(h.finalize().into())
   140	    }
   141	}
   142	
   143	impl LedgerEntry {
   144	    /// Project the LedgerEntry's signed-fields-subset back into a signing payload.
   145	    /// Used by replay to recompute `signing_digest` and re-verify chain integrity.
   146	    pub fn to_signing_payload(&self) -> LedgerEntrySigningPayload {
   147	        LedgerEntrySigningPayload {
   148	            logical_t: self.logical_t,
   149	            parent_state_root: self.parent_state_root,
   150	            parent_ledger_root: self.parent_ledger_root,
   151	            tx_kind: self.tx_kind,
   152	            tx_payload_cid: self.tx_payload_cid,
   153	            resulting_state_root: self.resulting_state_root,
   154	            timestamp_logical: self.timestamp_logical,
   155	            epoch: self.epoch,
   156	            extensions: self.extensions.clone(),
   157	        }
   158	    }
   159	}
   160	
   161	// ────────────────────────────────────────────────────────────────────────────
   162	// § 4 append() — pure ledger-root fold
   163	// ────────────────────────────────────────────────────────────────────────────
   164	
   165	/// TRACE_MATRIX FC2-Append + spec § 4: pure ledger-root fold over signed digests.
   166	/// Same `(parent_root, signing_digest)` → byte-identical `new_root`.
   167	/// No I/O, no clock, no env. Witness for I-DET ledger axis.
   168	pub fn append(parent_root: &Hash, signing_digest: &Hash) -> Hash {
   169	    let mut h = Sha256::new();
   170	    h.update(b"turingosv4.ledger_root.v1");
   171	    h.update(parent_root.0);
   172	    h.update(signing_digest.0);
   173	    Hash(h.finalize().into())
   174	}
   175	
   176	// ────────────────────────────────────────────────────────────────────────────
   177	// LedgerWriter trait (K4 reconciled to skeleton signature)
   178	// ────────────────────────────────────────────────────────────────────────────
   179	
   180	/// TRACE_MATRIX FC2-Append: storage abstraction for L4.
   181	/// Production impl is `Git2LedgerWriter` (CO1.7.5+; refs/transitions/main commit chain).
   182	/// Test/skeleton impl is `InMemoryLedgerWriter` below.
   183	///
   184	/// **K4**: signature `commit(&mut self) → Hash` (NOT `&self → NodeId`); `iter_from`
   185	/// deferred to CO1.7.5+ (only used by FullTransition replay; not v1 deliverable).
   186	pub trait LedgerWriter: Send + Sync {
   187	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
   188	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
   189	    fn len(&self) -> u64;
   190	}
   191	
   192	#[derive(Debug)]
   193	pub enum LedgerWriterError {
   194	    LogicalTGap { expected: u64, got: u64 },
   195	    NotFound { logical_t: u64 },
   196	    BackendCorruption(String),
   197	}
   198	
   199	impl std::fmt::Display for LedgerWriterError {
   200	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   201	        match self {
   202	            Self::LogicalTGap { expected, got } => {
   203	                write!(f, "logical_t gap: expected {expected}, got {got}")
   204	            }
   205	            Self::NotFound { logical_t } => write!(f, "no entry at logical_t={logical_t}"),
   206	            Self::BackendCorruption(msg) => write!(f, "backend corruption: {msg}"),
   207	        }
   208	    }
   209	}
   210	impl std::error::Error for LedgerWriterError {}
   211	
   212	/// In-memory test/skeleton writer; Vec backing strict logical_t enforced at commit.
   213	#[derive(Debug, Default)]
   214	pub struct InMemoryLedgerWriter {
   215	    entries: Vec<LedgerEntry>,
   216	}
   217	
   218	impl InMemoryLedgerWriter {
   219	    pub fn new() -> Self {
   220	        Self::default()
   221	    }
   222	}
   223	
   224	impl LedgerWriter for InMemoryLedgerWriter {
   225	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
   226	        let expected = (self.entries.len() as u64) + 1;
   227	        if entry.logical_t != expected {
   228	            return Err(LedgerWriterError::LogicalTGap {
   229	                expected,
   230	                got: entry.logical_t,
   231	            });
   232	        }
   233	        let root = entry.resulting_ledger_root;
   234	        self.entries.push(entry.clone());
   235	        Ok(root)
   236	    }
   237	
   238	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
   239	        if logical_t == 0 || logical_t > self.entries.len() as u64 {
   240	            return Err(LedgerWriterError::NotFound { logical_t });
   241	        }
   242	        Ok(self.entries[(logical_t - 1) as usize].clone())
   243	    }
   244	
   245	    fn len(&self) -> u64 {
   246	        self.entries.len() as u64
   247	    }
   248	}
   249	
   250	// ────────────────────────────────────────────────────────────────────────────
   251	// § 4 replay — TWO-MODE per C1
   252	// ────────────────────────────────────────────────────────────────────────────
   253	
   254	/// **C1 NEW**: replay mode discriminator.
   255	/// - `ChainOnly`: skeleton-stage; chain integrity only (parent_state_root +
   256	///   parent_ledger_root + ledger_root chain). NOT the I-DETHASH witness.
   257	/// - `FullTransition`: CO1.7.5+ stage; verifies signatures + re-fetches payloads
   258	///   from CAS + re-runs pure transitions + asserts state_root match. THE
   259	///   I-DETHASH witness; requires CO1.4-extra (CAS index persistence).
   260	#[derive(Debug, Clone, Copy, PartialEq, Eq)]

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '180,460p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   180	    /// TRACE_MATRIX FC3-Sig: new signing epoch certified by the rotation proof.
   181	    pub const fn new_epoch(&self) -> SystemEpoch {
   182	        self.new_epoch
   183	    }
   184	}
   185	
   186	/// TRACE_MATRIX FC1-Sig+FC3-Sig: only typed runtime messages may enter signature verification.
   187	#[derive(Debug, Clone, PartialEq, Eq)]
   188	pub enum CanonicalMessage {
   189	    /// TRACE_MATRIX FC1-Sig: predicate-runner rejection summary.
   190	    RejectedAttemptSummary(RejectedAttemptSummary),
   191	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: terminal summary transaction.
   192	    TerminalSummaryTx(TerminalSummaryTx),
   193	    /// TRACE_MATRIX FC3-Sig: system key epoch continuity proof.
   194	    EpochRotationProof(EpochRotationProof),
   195	}
   196	
   197	/// TRACE_MATRIX FC1-Sig+FC3-Sig: epoch-indexed public keys pinned by genesis and rotation history.
   198	#[derive(Debug, Clone, Default, PartialEq, Eq)]
   199	pub struct PinnedSystemPubkeys {
   200	    keys: BTreeMap<SystemEpoch, SystemPublicKey>,
   201	}
   202	
   203	impl PinnedSystemPubkeys {
   204	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: create an empty pinned system-key map.
   205	    pub fn new() -> Self {
   206	        Self::default()
   207	    }
   208	
   209	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: pin a public key for a system epoch.
   210	    pub fn insert(
   211	        &mut self,
   212	        epoch: SystemEpoch,
   213	        public_key: SystemPublicKey,
   214	    ) -> Option<SystemPublicKey> {
   215	        self.keys.insert(epoch, public_key)
   216	    }
   217	
   218	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: fetch the public key pinned for a system epoch.
   219	    pub fn get(&self, epoch: SystemEpoch) -> Option<&SystemPublicKey> {
   220	        self.keys.get(&epoch)
   221	    }
   222	}
   223	
   224	/// TRACE_MATRIX FC1-Sig+FC3-Sig: in-memory ed25519 system keypair with zeroized private key on drop.
   225	#[derive(Zeroize, ZeroizeOnDrop)]
   226	pub struct Ed25519Keypair {
   227	    secret_key: Box<[u8]>,
   228	    #[zeroize(skip)]
   229	    public_key: SystemPublicKey,
   230	}
   231	
   232	impl Ed25519Keypair {
   233	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: generate ed25519 key material from `getrandom(2)` entropy.
   234	    pub fn generate_with_secure_entropy() -> Result<Self, KeypairError> {
   235	        let mut seed = [0u8; SECRET_KEY_LEN];
   236	        getrandom::getrandom(&mut seed).map_err(KeypairError::Entropy)?;
   237	        let signing_key = SigningKey::from_bytes(&seed);
   238	        let public_key = SystemPublicKey::from_bytes(signing_key.verifying_key().to_bytes());
   239	        let mut keypair = Self {
   240	            secret_key: Vec::from(seed).into_boxed_slice(),
   241	            public_key,
   242	        };
   243	        seed.zeroize();
   244	        keypair.mlock_private_key_best_effort();
   245	        Ok(keypair)
   246	    }
   247	
   248	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: return the public half of the system keypair.
   249	    pub const fn public_key(&self) -> SystemPublicKey {
   250	        self.public_key
   251	    }
   252	
   253	    fn from_plaintext(plaintext: &[u8]) -> Result<Self, KeypairError> {
   254	        if plaintext.len() != PLAINTEXT_LEN {
   255	            return Err(KeypairError::InvalidFormat("bad plaintext length"));
   256	        }
   257	        let mut secret = [0u8; SECRET_KEY_LEN];
   258	        secret.copy_from_slice(&plaintext[..SECRET_KEY_LEN]);
   259	        let mut public = [0u8; PUBLIC_KEY_LEN];
   260	        public.copy_from_slice(&plaintext[SECRET_KEY_LEN..]);
   261	
   262	        let signing_key = SigningKey::from_bytes(&secret);
   263	        if signing_key.verifying_key().to_bytes() != public {
   264	            secret.zeroize();
   265	            return Err(KeypairError::InvalidFormat(
   266	                "public key does not match private key",
   267	            ));
   268	        }
   269	
   270	        let mut keypair = Self {
   271	            secret_key: Vec::from(secret).into_boxed_slice(),
   272	            public_key: SystemPublicKey::from_bytes(public),
   273	        };
   274	        secret.zeroize();
   275	        keypair.mlock_private_key_best_effort();
   276	        Ok(keypair)
   277	    }
   278	
   279	    fn to_plaintext(&self) -> Result<[u8; PLAINTEXT_LEN], KeypairError> {
   280	        let secret = self.secret_slice()?;
   281	        let mut plaintext = [0u8; PLAINTEXT_LEN];
   282	        plaintext[..SECRET_KEY_LEN].copy_from_slice(secret);
   283	        plaintext[SECRET_KEY_LEN..].copy_from_slice(self.public_key.as_bytes());
   284	        Ok(plaintext)
   285	    }
   286	
   287	    fn sign_digest(&self, digest: [u8; 32]) -> Result<SystemSignature, KeypairError> {
   288	        let mut secret = [0u8; SECRET_KEY_LEN];
   289	        secret.copy_from_slice(self.secret_slice()?);
   290	        let signing_key = SigningKey::from_bytes(&secret);
   291	        let signature = signing_key.sign(&digest);
   292	        secret.zeroize();
   293	        Ok(SystemSignature::from_bytes(signature.to_bytes()))
   294	    }
   295	
   296	    fn secret_slice(&self) -> Result<&[u8], KeypairError> {
   297	        if self.secret_key.len() == SECRET_KEY_LEN {
   298	            Ok(&self.secret_key)
   299	        } else {
   300	            Err(KeypairError::InvalidFormat("bad in-memory secret length"))
   301	        }
   302	    }
   303	
   304	    fn mlock_private_key_best_effort(&mut self) -> bool {
   305	        mlock_best_effort(self.secret_key.as_ptr(), self.secret_key.len())
   306	    }
   307	}
   308	
   309	/// TRACE_MATRIX FC1-Sig+FC3-Sig: system keypair lifecycle and crypto error taxonomy.
   310	#[derive(Debug)]
   311	pub enum KeypairError {
   312	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: filesystem operation failed.
   313	    Io(std::io::Error),
   314	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: secure operating-system entropy failed.
   315	    Entropy(getrandom::Error),
   316	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: KDF environment parameter was absent or invalid.
   317	    KdfParam(String),
   318	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: Argon2id key derivation failed.
   319	    Kdf(argon2::Error),
   320	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: ChaCha20-Poly1305 encryption or authentication failed.
   321	    Crypto(&'static str),
   322	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: encrypted keystore format was malformed.
   323	    InvalidFormat(&'static str),
   324	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: default keystore path could not be resolved.
   325	    HomeUnavailable,
   326	}
   327	
   328	impl fmt::Display for KeypairError {
   329	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   330	        match self {
   331	            Self::Io(err) => write!(f, "system keypair I/O failed: {err}"),
   332	            Self::Entropy(err) => write!(f, "system keypair entropy failed: {err}"),
   333	            Self::KdfParam(msg) => write!(f, "system keypair KDF parameter invalid: {msg}"),
   334	            Self::Kdf(err) => write!(f, "system keypair KDF failed: {err}"),
   335	            Self::Crypto(msg) => write!(f, "system keypair crypto failed: {msg}"),
   336	            Self::InvalidFormat(msg) => write!(f, "system keypair keystore invalid: {msg}"),
   337	            Self::HomeUnavailable => {
   338	                write!(f, "system keypair default keystore path requires HOME")
   339	            }
   340	        }
   341	    }
   342	}
   343	
   344	impl std::error::Error for KeypairError {}
   345	
   346	impl From<std::io::Error> for KeypairError {
   347	    fn from(value: std::io::Error) -> Self {
   348	        Self::Io(value)
   349	    }
   350	}
   351	
   352	/// TRACE_MATRIX FC1-Sig+FC3-Sig: resolve `~/.turingos/keystore/system_keypair_v{epoch}.enc`.
   353	///
   354	/// `TURINGOS_KEYSTORE_PATH` overrides the default path. The default never
   355	/// points into the repository, CAS, or ledger directories.
   356	pub fn default_system_keystore_path(epoch: SystemEpoch) -> Result<PathBuf, KeypairError> {
   357	    if let Ok(path) = env::var("TURINGOS_KEYSTORE_PATH") {
   358	        return Ok(PathBuf::from(path));
   359	    }
   360	    let home = env::var("HOME").map_err(|_| KeypairError::HomeUnavailable)?;
   361	    Ok(PathBuf::from(home)
   362	        .join(".turingos")
   363	        .join("keystore")
   364	        .join(format!("system_keypair_v{}.enc", epoch.get())))
   365	}
   366	
   367	/// TRACE_MATRIX FC1-Sig+FC3-Sig: first-boot generate-or-second-boot decrypt lifecycle entrypoint.
   368	pub fn generate_or_load_system_keypair(
   369	    keystore_path: &Path,
   370	    user_kdf_password: &SecretString,
   371	) -> Result<Ed25519Keypair, KeypairError> {
   372	    if keystore_path.exists() {
   373	        return load_existing_keypair(keystore_path, user_kdf_password);
   374	    }
   375	
   376	    let keypair = Ed25519Keypair::generate_with_secure_entropy()?;
   377	    let encrypted = encrypt_at_rest(&keypair, user_kdf_password)?;
   378	    write_keystore_0600(keystore_path, &encrypted)?;
   379	    Ok(keypair)
   380	}
   381	
   382	/// TRACE_MATRIX FC1-Sig+FC3-Sig: decrypt an existing encrypted system keypair keystore.
   383	pub fn load_existing_keypair(
   384	    keystore_path: &Path,
   385	    user_kdf_password: &SecretString,
   386	) -> Result<Ed25519Keypair, KeypairError> {
   387	    let bytes = fs::read(keystore_path)?;
   388	    let encoded = EncryptedKeypair::decode(&bytes)?;
   389	    let mut key = derive_key(user_kdf_password, &encoded.salt, encoded.kdf)?;
   390	    let cipher = ChaCha20Poly1305::new_from_slice(&key)
   391	        .map_err(|_| KeypairError::Crypto("bad cipher key"))?;
   392	    let plaintext = cipher
   393	        .decrypt(
   394	            Nonce::from_slice(&encoded.nonce),
   395	            encoded.ciphertext.as_ref(),
   396	        )
   397	        .map_err(|_| KeypairError::Crypto("keystore authentication failed"))?;
   398	    key.zeroize();
   399	    Ed25519Keypair::from_plaintext(&plaintext)
   400	}
   401	
   402	/// TRACE_MATRIX FC1-Sig+FC3-Sig: canonical SHA-256 digest for typed system messages.
   403	pub fn canonical_digest(message: &CanonicalMessage) -> [u8; 32] {
   404	    let mut h = Sha256::new();
   405	    h.update(b"turingosv4.system_keypair.v1");
   406	    match message {
   407	        CanonicalMessage::RejectedAttemptSummary(summary) => {
   408	            h.update(b"RejectedAttemptSummary");
   409	            update_len_prefixed(&mut h, summary.run_id.as_bytes());
   410	            update_len_prefixed(&mut h, summary.attempt_id.as_bytes());
   411	            update_len_prefixed(&mut h, summary.failure_class.as_bytes());
   412	            h.update(summary.summary_hash);
   413	        }
   414	        CanonicalMessage::TerminalSummaryTx(tx) => {
   415	            h.update(b"TerminalSummaryTx");
   416	            update_len_prefixed(&mut h, tx.run_id.as_bytes());
   417	            h.update(tx.terminal_state_root);
   418	            h.update(tx.rejected_attempt_count.to_be_bytes());
   419	        }
   420	        CanonicalMessage::EpochRotationProof(proof) => {
   421	            h.update(b"EpochRotationProof");
   422	            h.update(proof.old_epoch.get().to_be_bytes());
   423	            h.update(proof.new_epoch.get().to_be_bytes());
   424	            h.update(proof.old_pubkey.as_bytes());
   425	            h.update(proof.new_pubkey.as_bytes());
   426	            h.update(proof.signed_at_unix.to_be_bytes());
   427	        }
   428	    }
   429	    h.finalize().into()
   430	}
   431	
   432	/// TRACE_MATRIX FC1-Sig+FC3-Sig: public system signature verification against pinned epoch keys.
   433	pub fn verify_system_signature(
   434	    sig: &SystemSignature,
   435	    message: &CanonicalMessage,
   436	    epoch: SystemEpoch,
   437	    pinned_pubkeys: &PinnedSystemPubkeys,
   438	) -> bool {
   439	    let Some(pk) = pinned_pubkeys.get(epoch) else {
   440	        return false;
   441	    };
   442	    let Ok(verifying_key) = VerifyingKey::from_bytes(pk.as_bytes()) else {
   443	        return false;
   444	    };
   445	    let signature = Signature::from_bytes(sig.as_bytes());
   446	    verifying_key
   447	        .verify(&canonical_digest(message), &signature)
   448	        .is_ok()
   449	}
   450	
   451	/// TRACE_MATRIX FC3-Sig: verify old and new signatures over a rotation continuity proof.
   452	pub fn verify_epoch_rotation_proof(
   453	    proof: &EpochRotationProof,
   454	    old_signature: &SystemSignature,
   455	    new_signature: &SystemSignature,
   456	    pinned_pubkeys: &PinnedSystemPubkeys,
   457	) -> bool {
   458	    let message = CanonicalMessage::EpochRotationProof(proof.clone());
   459	    verify_system_signature(old_signature, &message, proof.old_epoch(), pinned_pubkeys)
   460	        && verify_system_signature(new_signature, &message, proof.new_epoch(), pinned_pubkeys)

codex
The local v1.1 spec and skeleton match the prompt at the main schema level. One important local fact: `system_keypair.rs` itself is still the old 3-variant enum, while the v1.1 skeleton says the actual `CanonicalMessage` extension is deferred. I’ll account for that as a spec/skeleton closure issue rather than assuming it landed.
exec
/bin/bash -lc "nl -ba handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md | sed -n '237,520p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   237	## § 3 Sequencer (K1 dual-counter; K3 head_t ownership; C3 sign API)
   238	
   239	```rust
   240	/// TRACE_MATRIX § 5.2.1 — L4 sequencer; single-writer per (runtime_repo, run_id).
   241	pub struct Sequencer {
   242	    /// **K1 NEW**: separate counter for submissions (independent of accept).
   243	    /// Used to derive submit_id for SubmissionReceipt; never appears in LedgerEntry.
   244	    next_submit_id: AtomicU64,
   245	
   246	    /// **K1 changed semantics**: advances ONLY on commit, NOT on submit.
   247	    /// Genesis = 0; first accepted entry gets logical_t=1.
   248	    next_logical_t: AtomicU64,
   249	
   250	    /// **Q1 resolution**: bounded `tokio::sync::mpsc::Sender` (NOT unbounded).
   251	    /// Submit returns `QueueFull` Err on saturation; agents handle backoff.
   252	    queue_tx: tokio::sync::mpsc::Sender<TypedTx>,
   253	
   254	    /// CAS handle for payload storage.
   255	    cas: Arc<RwLock<CasStore>>,
   256	
   257	    /// **C3**: signing key handle (CO1.7.0a-f).
   258	    keypair: Arc<Ed25519Keypair>,
   259	    epoch: SystemEpoch,                   // current signing epoch
   260	
   261	    /// Storage backend (in CO1.7.5+; skeleton uses InMemoryLedgerWriter).
   262	    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
   263	
   264	    /// Predicate + tool registries (read-only).
   265	    predicate_registry: Arc<PredicateRegistry>,
   266	    tool_registry: Arc<ToolRegistry>,
   267	
   268	    /// Current Q_t snapshot.
   269	    q: RwLock<QState>,
   270	}
   271	
   272	impl Sequencer {
   273	    /// Submit a typed transition for processing. Returns immediately with a
   274	    /// SubmissionReceipt carrying `submit_id` (NOT logical_t — submit_id is
   275	    /// always assigned; logical_t only assigned post-accept).
   276	    pub async fn submit(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError>;
   277	
   278	    /// Driver loop: drain queue, run transition, commit on accept. Single-thread internal.
   279	    pub async fn run(&self) -> Result<(), SequencerError>;
   280	
   281	    /// Per-tx critical section.
   282	    fn apply_one(&self, tx: TypedTx) -> Result<LedgerEntry, TransitionError> {
   283	        // 1. Snapshot Q_t under read lock
   284	        let q_snapshot = self.q.read().clone();
   285	
   286	        // 2. Dispatch (pure)
   287	        let (q_next, _signals) = dispatch_transition(&q_snapshot, &tx, &self.predicate_registry, &self.tool_registry)?;
   288	        // **K1**: if step returns Err, EARLY RETURN — no logical_t assigned, no entry committed.
   289	
   290	        // 3. Put payload to CAS (DIV-5 5-param signature)
   291	        let mut cas_w = self.cas.write();
   292	        let cas_bytes = canonical_serialize(&tx);  // bincode v2 per § 2.5 of STATE spec
   293	        let payload_cid = cas_w.put(
   294	            &cas_bytes,
   295	            ObjectType::Transition,
   296	            &format!("sequencer-{}", self.epoch.get()),
   297	            self.next_logical_t.load(Ordering::SeqCst) + 1,  // tentative; final below
   298	            Some("LedgerEntrySigningPayload.v1".to_string()),
   299	        )?;
   300	        drop(cas_w);
   301	
   302	        // 4. **K1**: assign logical_t ONLY now (post-accept)
   303	        let logical_t = self.next_logical_t.fetch_add(1, Ordering::SeqCst) + 1;
   304	
   305	        // 5. Build LedgerEntrySigningPayload
   306	        let signing_payload = LedgerEntrySigningPayload {
   307	            logical_t,
   308	            parent_state_root: q_snapshot.state_root_t,
   309	            parent_ledger_root: q_snapshot.ledger_root_t,   // K2 transplant defense
   310	            tx_kind: TxKind::from_typed(&tx),
   311	            tx_payload_cid: payload_cid,
   312	            resulting_state_root: q_next.state_root_t,
   313	            timestamp_logical: logical_t,
   314	            epoch: self.epoch,
   315	            extensions: BTreeMap::new(),                     // G1 empty in v1
   316	        };
   317	
   318	        // 6. **C3 NEW SIGN API**: typed sign through CanonicalMessage extension
   319	        let system_signature = self.keypair.sign_ledger_entry(&signing_payload, self.epoch);
   320	
   321	        // 7. Compute resulting_ledger_root via append() (pure)
   322	        let signing_digest = signing_payload.canonical_digest();
   323	        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);
   324	
   325	        // 8. Build LedgerEntry (the stored record)
   326	        let entry = LedgerEntry {
   327	            logical_t: signing_payload.logical_t,
   328	            parent_state_root: signing_payload.parent_state_root,
   329	            parent_ledger_root: signing_payload.parent_ledger_root,
   330	            tx_kind: signing_payload.tx_kind,
   331	            tx_payload_cid: signing_payload.tx_payload_cid,
   332	            resulting_state_root: signing_payload.resulting_state_root,
   333	            resulting_ledger_root,                            // derived; not in signed digest
   334	            timestamp_logical: signing_payload.timestamp_logical,
   335	            epoch: signing_payload.epoch,
   336	            extensions: signing_payload.extensions,
   337	            system_signature,
   338	        };
   339	
   340	        // 9. Commit + mutate Q_t under write lock (atomic)
   341	        let mut q_w = self.q.write();
   342	        let mut writer_w = self.ledger_writer.write();
   343	        writer_w.commit(&entry)?;                              // K4 returns Hash; matches skeleton
   344	        drop(writer_w);
   345	        *q_w = q_next;
   346	        q_w.ledger_root_t = entry.resulting_ledger_root;
   347	        // **K3**: do NOT mutate q_w.head_t here. CO1.7's ownership is `ledger_root_t` only;
   348	        // `head_t = NodeId(commit_sha)` is set by `Git2LedgerWriter::commit` returning the
   349	        // commit_sha through the writer's `Hash` return + sequencer assigning to head_t
   350	        // under the same write lock. (Skeleton uses InMemoryLedgerWriter; head_t mutation
   351	        // path is CO1.7.5+ wiring concern.)
   352	
   353	        Ok(entry)
   354	    }
   355	}
   356	```
   357	
   358	**Why dual counter (K1)**: rejection of a submission must NOT consume a logical_t, because (a) skeleton's `InMemoryLedgerWriter::commit` enforces `expected_logical_t = len + 1` and would reject a gap; (b) replay enforces `entry.logical_t == (i+1)` and would reject a gap. Submitter IDs (`submit_id`) are returned from `submit()` immediately for receipt; logical_t is observable only on the committed entry.
   359	
   360	**Why no head_t mutation in apply_one (K3)**: CO1.7 owns `ledger_root_t` and the commit-chain `head_t`; CO1.8 (L5 materializer) owns `state_root_t` mutation. Sequencer accepts `q_next.state_root_t` as the transition function returns it; sequencer does NOT call `NodeId::from_state_root(...)`.
   361	
   362	**Q3 (Gemini)**: `Sequencer` vs `LedgerWriter + OrderingCoordinator` split — v1.1 keeps `Sequencer` as the abstraction; trait-segregation refactor is a v4.x consideration (the current single-writer constraint per § 5.2.1 makes the split synthetic for v1).
   363	
   364	---
   365	
   366	## § 4 append() + replay() — two-mode (per C1)
   367	
   368	```rust
   369	/// Pure. Same (parent_root, signing_digest) → byte-identical new_root.
   370	/// No I/O, no clock, no env. Witness for I-DET / I-DETHASH ledger axis.
   371	pub fn append(parent_root: &Hash, signing_digest: &Hash) -> Hash {
   372	    let mut h = Sha256::new();
   373	    h.update(b"turingosv4.ledger_root.v1");      // domain separation
   374	    h.update(parent_root.0);
   375	    h.update(signing_digest.0);
   376	    Hash(h.finalize().into())
   377	}
   378	
   379	/// Replay mode (C1).
   380	pub enum ReplayMode {
   381	    /// Skeleton-stage: validates parent_state_root + parent_ledger_root + ledger_root chain.
   382	    /// Does NOT verify signatures, re-fetch payloads, or re-run pure transitions.
   383	    /// Trust mode: "trust the sequencer". v1 deliverable.
   384	    ChainOnly,
   385	    /// CO1.7.5+ stage: full re-execution.
   386	    /// Verifies signatures via CanonicalMessage; fetches payloads from CAS;
   387	    /// re-runs pure dispatch_transition; compares resulting_state_root.
   388	    /// **THIS** is the I-DETHASH witness (I-DETHASH bound to FullTransition only).
   389	    /// Requires CO1.4-extra (CAS index persistence) for cold-restart.
   390	    FullTransition,
   391	}
   392	
   393	/// Skeleton-stage entry point (v1).
   394	pub fn replay_chain_integrity(
   395	    genesis_state_root: Hash,
   396	    genesis_ledger_root: Hash,
   397	    entries: &[LedgerEntry],
   398	) -> Result<(Hash, Hash), ReplayError>;
   399	
   400	/// CO1.7.5+ stage entry point (v1.1 spec only; impl deferred).
   401	pub fn replay_full_transition(
   402	    genesis: &QState,
   403	    entries: &[LedgerEntry],
   404	    cas: &dyn LedgerCasView,
   405	    pinned_pubkeys: &PinnedSystemPubkeys,
   406	) -> Result<QState, ReplayError>;
   407	```
   408	
   409	**I-DETHASH witness (revised per C1)**: `replay_full_transition` is the I-DETHASH witness. `replay_chain_integrity` is necessary-but-not-sufficient — passing chain check does NOT prove transition determinism. v1 documents this explicitly to close trust ambiguity.
   410	
   411	**ReplayError enum** (skeleton already has 3 variants; v1.1 adds 4 more for FullTransition):
   412	- `LogicalTGap { at, expected, got }` (existing)
   413	- `ParentMismatch { at }` (existing; covers parent_state_root)
   414	- `LedgerRootMismatch { at }` (existing)
   415	- `ParentLedgerRootMismatch { at }` (NEW K2)
   416	- `BadSignature { at }` (NEW; FullTransition only)
   417	- `CasMissing { at, cid }` (NEW; FullTransition only — fires if CO1.4-extra not yet landed)
   418	- `StateRootMismatch { at }` (NEW; FullTransition only)
   419	- `TransitionError { at, inner }` (NEW; wraps dispatch_transition errors)
   420	
   421	---
   422	
   423	## § 5 Storage backend
   424	
   425	**Choice**: git2-rs commit chain (Path B substrate, ratified per Const Art 0.4 + WP § 5.L4).
   426	
   427	**Mapping**:
   428	- One `LedgerEntry` = one git commit on `refs/transitions/main`.
   429	- Commit message = canonical-serialized `LedgerEntry` (bincode v2 per `STATE_TRANSITION_SPEC § 2.5`).
   430	- Commit tree = `(payload_cid_blob, signature_blob)` (state_root NOT a tree blob — per K3, L5 owns state_root materialization).
   431	- **K3**: `head_t = NodeId(commit_sha)` is the canonical convention. `NodeId::from_state_root(...)` is NOT used by L4. (q_state.rs:54 keeps the helper for cross-reference but L4 sequencer does NOT call it.)
   432	- **C2**: cold-replay availability requires `CasStore` index persistence; deferred to CO1.4-extra. Until then, full-mode replay errors with `CasMissing` if CAS state is not warm.
   433	- Genesis: `refs/transitions/main` is created at the empty-tree commit corresponding to `genesis_payload.toml` (CO1.0). `genesis_ledger_root_t = sha256("turingosv4.ledger_root.v1.genesis" || sha256(genesis_payload.toml))` — **Q7 resolution** (NOT `Hash::ZERO`; both auditors agreed).
   434	
   435	**LedgerWriter trait (K4 reconciled to skeleton)**:
   436	
   437	```rust
   438	pub trait LedgerWriter: Send + Sync {
   439	    /// Commit a signed LedgerEntry. K4: `&mut self` + `Hash` return matches skeleton.
   440	    /// Returns the entry's `resulting_ledger_root`.
   441	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
   442	
   443	    /// Read entry at a specific 1-indexed `logical_t`.
   444	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
   445	
   446	    /// Total accepted entries (highest assigned logical_t; 0 at genesis).
   447	    fn len(&self) -> u64;
   448	
   449	    // K4: iter_from() deferred — used only by FullTransition replay; CO1.7.5+ adds it.
   450	}
   451	```
   452	
   453	**Implementation (CO1.7.5+)**: `Git2LedgerWriter` (built on existing CO1.4 CAS); skeleton `InMemoryLedgerWriter` for v1 testing.
   454	
   455	**Why git2-rs not gix**: Const Art 0.4 ratified path B (gix→git2-rs pivot per CO1.3.1 spike 8/8 PASS).
   456	
   457	---
   458	
   459	## § 6 Invariants enforced by CO1.7
   460	
   461	| ID | Invariant | Enforced where in CO1.7 |
   462	|---|---|---|
   463	| **I-DET** | Same (Q_t, tx) → byte-identical (Q_{t+1}, signals) | sequencer.apply_one stages 2-7 (pure dispatch + deterministic append) |
   464	| **I-DETHASH** | replay_full_transition(genesis, entries) recovers live state_root | **Bound to FullTransition mode only** (C1); skeleton ChainOnly is necessary-but-not-sufficient |
   465	| **I-LOGTIME** | timestamp_logical strictly monotonic; no wall clock | sequencer apply_one stage 4; LedgerEntry has no wall-clock field |
   466	| **I-FINALIZE-BATCH-ORDER** | When N claims expire same logical_t, finalize order = `(expires_at_logical ASC, claim_id ASC)` | sequencer enqueues finalize tx in order before resuming work tx; per § 5.2.3 |
   467	| **I-FINALIZE-EXCLUSIVE** | finalize_reward and slash mutually exclusive per claim | **v4 has no Slash** (K5); invariant trivially holds via TxKind enum |
   468	| **I-NOSIDE** | step_transition reads only (q, tx, registries) | append() and replay_* are pure; sequencer.apply_one isolates I/O to CAS put + writer commit |
   469	| **I-NOENV** | step_transition dependency tree has no `std::env` access | grep test in CO1.7 module — already enforced by CLAUDE.md hardcoded-config rule (C-027) |
   470	| **I-NORANDOM** | tx consuming randomness MUST seed PRNG from `(tx.tx_id, q.state_root_t)` | LedgerEntry.system_signature uses keypair (deterministic); no entropy in append/replay |
   471	
   472	CO1.7 does NOT introduce new invariants — provides machine-checkable witnesses for 8 of the 27 frozen invariants.
   473	
   474	---
   475	
   476	## § 7 Conformance tests (K7 staged)
   477	
   478	| Test | Stage | What it asserts |
   479	|---|---|---|
   480	| `tests/append_byte_stable` | skeleton (v1) | append byte-stable across calls (I-DET ledger axis) |
   481	| `tests/canonical_digest_stable` | skeleton (v1) | LedgerEntrySigningPayload digest stable across clones; #[repr(u8)] discriminant stable |
   482	| `tests/inmemory_writer_logical_t` | skeleton (v1) | InMemoryLedgerWriter rejects logical_t gaps |
   483	| `tests/replay_chain_integrity_clean` | skeleton (v1) | clean ChainOnly replay returns final state_root + ledger_root |
   484	| `tests/replay_chain_rejects_parent_state_root_tamper` | skeleton (v1) | ChainOnly replay rejects parent_state_root tamper |
   485	| `tests/replay_chain_rejects_parent_ledger_root_tamper` | **K2 NEW** skeleton (v1) | ChainOnly replay rejects parent_ledger_root tamper (transplant defense) |
   486	| `tests/replay_chain_rejects_ledger_root_tamper` | skeleton (v1) | ChainOnly replay rejects resulting_ledger_root tamper |
   487	| `tests/canonical_digest_excludes_derivatives` | **Q9 NEW** skeleton (v1) | LedgerEntrySigningPayload.canonical_digest excludes resulting_ledger_root + system_signature; mutation of either does NOT change digest |
   488	| `tests/replay_full_transition_state_root` | CO1.7.5+ (post-CO1.4-extra) | FullTransition replay re-runs dispatch_transition; asserts state_root match (I-DETHASH witness) |
   489	| `tests/system_signature_verifies_via_canonical_message` | CO1.7.5+ | LedgerEntry.system_signature verifies through `verify_system_signature(&CanonicalMessage::LedgerEntrySigning(...), epoch, pinned_pubkeys)` |
   490	| `tests/cas_payload_round_trip` | CO1.7.5+ (after CO1.4-extra) | put→get round trip; CID stability across runs |
   491	| `tests/sequencer_serial_replay_byte_identity` | CO1.7.5+ | submit 100 tx; replay → byte-identical state_root |
   492	
   493	**v1 stage (skeleton)**: 8 tests (6 already in skeleton + 2 NEW K2/Q9). **CO1.7.5+ stage**: 4 more.
   494	
   495	---
   496	
   497	## § 8 dispatch_transition (K5 Slash dropped)
   498	
   499	```rust
   500	pub(crate) fn dispatch_transition(
   501	    q: &QState,
   502	    tx: &TypedTx,
   503	    predicate_registry: &PredicateRegistry,
   504	    tool_registry: &ToolRegistry,
   505	) -> Result<(QState, SignalBundle), TransitionError> {
   506	    match tx {
   507	        TypedTx::Work(t)             => step_transition(q, t, predicate_registry, tool_registry),
   508	        TypedTx::Verify(t)           => verify_transition(q, t, predicate_registry),
   509	        TypedTx::Challenge(t)        => challenge_transition(q, t, predicate_registry),
   510	        TypedTx::Reuse(t)            => reuse_transition(q, t, tool_registry),
   511	        TypedTx::FinalizeReward(t)   => finalize_reward_transition(q, t),
   512	        TypedTx::TaskExpire(t)       => task_expire_transition(q, t),
   513	        TypedTx::TerminalSummary(t)  => emit_terminal_summary(q, t),
   514	        // K5: NO `TypedTx::Slash` — v4 has no slash transition.
   515	    }
   516	}
   517	```
   518	
   519	**Q5 resolution** (Gemini): enum-match for v4 (exhaustive, deterministic, simple); defer `MetaTransitionInterface` trait pattern to v4.1 dynamic MetaTx.
   520	

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '250,470p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   250	// ────────────────────────────────────────────────────────────────────────────
   251	// § 4 replay — TWO-MODE per C1
   252	// ────────────────────────────────────────────────────────────────────────────
   253	
   254	/// **C1 NEW**: replay mode discriminator.
   255	/// - `ChainOnly`: skeleton-stage; chain integrity only (parent_state_root +
   256	///   parent_ledger_root + ledger_root chain). NOT the I-DETHASH witness.
   257	/// - `FullTransition`: CO1.7.5+ stage; verifies signatures + re-fetches payloads
   258	///   from CAS + re-runs pure transitions + asserts state_root match. THE
   259	///   I-DETHASH witness; requires CO1.4-extra (CAS index persistence).
   260	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
   261	pub enum ReplayMode {
   262	    ChainOnly,
   263	    FullTransition,
   264	}
   265	
   266	#[derive(Debug)]
   267	pub enum ReplayError {
   268	    LogicalTGap { at: usize, expected: u64, got: u64 },
   269	    ParentStateMismatch { at: usize },
   270	    ParentLedgerMismatch { at: usize }, // K2 NEW
   271	    LedgerRootMismatch { at: usize },
   272	    // FullTransition-mode-only (CO1.7.5+):
   273	    BadSignature { at: usize },
   274	    CasMissing { at: usize },
   275	    StateRootMismatch { at: usize },
   276	}
   277	
   278	impl std::fmt::Display for ReplayError {
   279	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   280	        match self {
   281	            Self::LogicalTGap { at, expected, got } => {
   282	                write!(f, "logical_t gap at index {at}: expected {expected}, got {got}")
   283	            }
   284	            Self::ParentStateMismatch { at } => write!(f, "parent_state_root mismatch at index {at}"),
   285	            Self::ParentLedgerMismatch { at } => write!(f, "parent_ledger_root mismatch at index {at}"),
   286	            Self::LedgerRootMismatch { at } => write!(f, "ledger_root mismatch at index {at}"),
   287	            Self::BadSignature { at } => write!(f, "system_signature verify failed at index {at}"),
   288	            Self::CasMissing { at } => write!(f, "CAS payload not retrievable at index {at}"),
   289	            Self::StateRootMismatch { at } => write!(f, "resulting_state_root divergence at index {at}"),
   290	        }
   291	    }
   292	}
   293	impl std::error::Error for ReplayError {}
   294	
   295	/// Skeleton-stage entry point (v1.1).
   296	///
   297	/// Validates:
   298	/// 1. logical_t monotonicity (no gaps, no duplicates)
   299	/// 2. parent_state_root chain
   300	/// 3. parent_ledger_root chain (K2 transplant defense)
   301	/// 4. resulting_ledger_root recomputed via append(prev_ledger_root, signing_digest)
   302	///
   303	/// Does NOT verify:
   304	/// - system_signature (CO1.7.5+: requires CanonicalMessage extension wired through keypair)
   305	/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
   306	///
   307	/// Returns final (state_root, ledger_root) on success.
   308	pub fn replay_chain_integrity(
   309	    genesis_state_root: Hash,
   310	    genesis_ledger_root: Hash,
   311	    entries: &[LedgerEntry],
   312	) -> Result<(Hash, Hash), ReplayError> {
   313	    let mut prev_state_root = genesis_state_root;
   314	    let mut prev_ledger_root = genesis_ledger_root;
   315	
   316	    for (i, entry) in entries.iter().enumerate() {
   317	        let expected_logical_t = (i as u64) + 1;
   318	        if entry.logical_t != expected_logical_t {
   319	            return Err(ReplayError::LogicalTGap {
   320	                at: i,
   321	                expected: expected_logical_t,
   322	                got: entry.logical_t,
   323	            });
   324	        }
   325	        if entry.parent_state_root != prev_state_root {
   326	            return Err(ReplayError::ParentStateMismatch { at: i });
   327	        }
   328	        // K2 NEW: parent_ledger_root chain check
   329	        if entry.parent_ledger_root != prev_ledger_root {
   330	            return Err(ReplayError::ParentLedgerMismatch { at: i });
   331	        }
   332	        let signing_digest = entry.to_signing_payload().canonical_digest();
   333	        let recomputed = append(&prev_ledger_root, &signing_digest);
   334	        if recomputed != entry.resulting_ledger_root {
   335	            return Err(ReplayError::LedgerRootMismatch { at: i });
   336	        }
   337	        prev_state_root = entry.resulting_state_root;
   338	        prev_ledger_root = entry.resulting_ledger_root;
   339	    }
   340	
   341	    Ok((prev_state_root, prev_ledger_root))
   342	}
   343	
   344	// ────────────────────────────────────────────────────────────────────────────
   345	// Tests — 8 conformance items (4 NEW vs v1 skeleton: K2 / Q9 / repr(u8) / extensions)
   346	// ────────────────────────────────────────────────────────────────────────────
   347	
   348	#[cfg(test)]
   349	mod tests {
   350	    use super::*;
   351	
   352	    fn h(byte: u8) -> Hash {
   353	        Hash([byte; 32])
   354	    }
   355	
   356	    /// Build an entry that satisfies all chain invariants given the previous state.
   357	    fn entry_at(
   358	        logical_t: u64,
   359	        parent_state_root: Hash,
   360	        parent_ledger_root: Hash,
   361	        resulting_state_root: Hash,
   362	    ) -> LedgerEntry {
   363	        let signing = LedgerEntrySigningPayload {
   364	            logical_t,
   365	            parent_state_root,
   366	            parent_ledger_root,
   367	            tx_kind: TxKind::Work,
   368	            tx_payload_cid: Cid([0u8; 32]),
   369	            resulting_state_root,
   370	            timestamp_logical: logical_t,
   371	            epoch: SystemEpoch::new(1),
   372	            extensions: BTreeMap::new(),
   373	        };
   374	        let signing_digest = signing.canonical_digest();
   375	        let resulting_ledger_root = append(&parent_ledger_root, &signing_digest);
   376	        LedgerEntry {
   377	            logical_t: signing.logical_t,
   378	            parent_state_root: signing.parent_state_root,
   379	            parent_ledger_root: signing.parent_ledger_root,
   380	            tx_kind: signing.tx_kind,
   381	            tx_payload_cid: signing.tx_payload_cid,
   382	            resulting_state_root: signing.resulting_state_root,
   383	            resulting_ledger_root,
   384	            timestamp_logical: signing.timestamp_logical,
   385	            epoch: signing.epoch,
   386	            extensions: signing.extensions,
   387	            system_signature: SystemSignature::from_bytes([0u8; 64]),
   388	        }
   389	    }
   390	
   391	    // 1. append byte-stable (I-DET ledger axis)
   392	    #[test]
   393	    fn append_is_pure_and_byte_stable() {
   394	        let a = append(&Hash::ZERO, &h(1));
   395	        let b = append(&Hash::ZERO, &h(1));
   396	        assert_eq!(a, b);
   397	        let c = append(&Hash::ZERO, &h(2));
   398	        assert_ne!(a, c);
   399	    }
   400	
   401	    // 2. canonical_digest stable (#[repr(u8)] discriminant stable)
   402	    #[test]
   403	    fn canonical_digest_stable_across_clones() {
   404	        let p = LedgerEntrySigningPayload {
   405	            logical_t: 1,
   406	            parent_state_root: Hash::ZERO,
   407	            parent_ledger_root: Hash::ZERO,
   408	            tx_kind: TxKind::Work,
   409	            tx_payload_cid: Cid([7u8; 32]),
   410	            resulting_state_root: h(0xaa),
   411	            timestamp_logical: 1,
   412	            epoch: SystemEpoch::new(2),
   413	            extensions: BTreeMap::new(),
   414	        };
   415	        let d1 = p.canonical_digest();
   416	        let d2 = p.clone().canonical_digest();
   417	        assert_eq!(d1, d2);
   418	    }
   419	
   420	    // 3. InMemoryWriter enforces logical_t monotonic
   421	    #[test]
   422	    fn in_memory_writer_enforces_logical_t() {
   423	        let mut w = InMemoryLedgerWriter::new();
   424	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   425	        assert!(w.commit(&e1).is_ok());
   426	
   427	        let e_skip = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   428	        let err = w.commit(&e_skip).unwrap_err();
   429	        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
   430	    }
   431	
   432	    // 4. ChainOnly replay validates clean chain
   433	    #[test]
   434	    fn replay_chain_integrity_clean() {
   435	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   436	        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   437	        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
   438	        let (final_state, final_ledger) =
   439	            replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1.clone(), e2.clone(), e3.clone()])
   440	                .expect("clean chain replays");
   441	        assert_eq!(final_state, e3.resulting_state_root);
   442	        assert_eq!(final_ledger, e3.resulting_ledger_root);
   443	    }
   444	
   445	    // 5. ChainOnly replay rejects parent_state_root tamper
   446	    #[test]
   447	    fn replay_rejects_parent_state_tamper() {
   448	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   449	        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   450	        e2.parent_state_root = h(0xff);
   451	        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
   452	        assert!(matches!(err, ReplayError::ParentStateMismatch { at: 1 }));
   453	    }
   454	
   455	    // 6. K2 NEW: ChainOnly replay rejects parent_ledger_root tamper (transplant defense)
   456	    #[test]
   457	    fn replay_rejects_parent_ledger_tamper() {
   458	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   459	        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   460	        // Tamper with parent_ledger_root WITHOUT recomputing resulting_ledger_root —
   461	        // simulates an attacker transplanting an entry from a different ledger history.
   462	        e2.parent_ledger_root = h(0xff);
   463	        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
   464	        assert!(matches!(err, ReplayError::ParentLedgerMismatch { at: 1 }));
   465	    }
   466	
   467	    // 7. ChainOnly replay rejects ledger_root tamper
   468	    #[test]
   469	    fn replay_rejects_ledger_root_tamper() {
   470	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/store.rs | sed -n '50,125p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    50	/// Content-addressable store backed by git's blob object database.
    51	pub struct CasStore {
    52	    repo_path: PathBuf,
    53	    /// Cid → metadata index. BTreeMap per spec § 2 I-BTREE.
    54	    index: BTreeMap<Cid, CasObjectMetadata>,
    55	}
    56	
    57	impl CasStore {
    58	    /// Open or initialize a CAS store at the given runtime_repo path.
    59	    /// Creates the git repo if it doesn't exist.
    60	    pub fn open(repo_path: &Path) -> Result<Self, CasError> {
    61	        let repo_path = repo_path.to_path_buf();
    62	        // Init or open
    63	        let _repo = match Repository::open(&repo_path) {
    64	            Ok(r) => r,
    65	            Err(_) => Repository::init(&repo_path)?,
    66	        };
    67	        Ok(Self {
    68	            repo_path,
    69	            index: BTreeMap::new(),
    70	        })
    71	    }
    72	
    73	    fn open_repo(&self) -> Result<Repository, CasError> {
    74	        Repository::open(&self.repo_path).map_err(CasError::from)
    75	    }
    76	
    77	    /// Store content; returns its Cid. Idempotent — same content → same Cid.
    78	    pub fn put(
    79	        &mut self,
    80	        content: &[u8],
    81	        object_type: ObjectType,
    82	        creator: &str,
    83	        created_at_logical_t: u64,
    84	        schema_id: Option<String>,
    85	    ) -> Result<Cid, CasError> {
    86	        let cid = Cid::from_content(content);
    87	        let repo = self.open_repo()?;
    88	        let git_oid = repo.blob(content)?;
    89	
    90	        // If already in index, idempotent: just return Cid (content addressing
    91	        // guarantees same content → same Cid → already present)
    92	        if self.index.contains_key(&cid) {
    93	            return Ok(cid);
    94	        }
    95	
    96	        let metadata = CasObjectMetadata {
    97	            cid,
    98	            backend_oid_hex: git_oid.to_string(),
    99	            object_type,
   100	            creator: creator.to_string(),
   101	            created_at_logical_t,
   102	            schema_id,
   103	            size_bytes: content.len() as u64,
   104	        };
   105	        self.index.insert(cid, metadata);
   106	        Ok(cid)
   107	    }
   108	
   109	    /// Retrieve content by Cid. Verifies content sha256 matches Cid (corruption check).
   110	    pub fn get(&self, cid: &Cid) -> Result<Vec<u8>, CasError> {
   111	        let metadata = self
   112	            .index
   113	            .get(cid)
   114	            .ok_or(CasError::CidNotFound(*cid))?;
   115	        let repo = self.open_repo()?;
   116	        let git_oid = git2::Oid::from_str(&metadata.backend_oid_hex)
   117	            .map_err(CasError::Git2)?;
   118	        let blob = repo.find_blob(git_oid)?;
   119	        let content = blob.content().to_vec();
   120	
   121	        // Verify content sha256 matches Cid (defense against corruption).
   122	        let mut h = Sha256::new();
   123	        h.update(&content);
   124	        let computed = Cid(h.finalize().into());
   125	        if &computed != cid {

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '467,560p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   467	    // 7. ChainOnly replay rejects ledger_root tamper
   468	    #[test]
   469	    fn replay_rejects_ledger_root_tamper() {
   470	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   471	        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   472	        e2.resulting_ledger_root = h(0xee);
   473	        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
   474	        assert!(matches!(err, ReplayError::LedgerRootMismatch { at: 1 }));
   475	    }
   476	
   477	    // 8. Q9 NEW: canonical_digest excludes derivatives
   478	    // Mutating `resulting_ledger_root` or `system_signature` of LedgerEntry must NOT
   479	    // change the signing payload digest (because they're not in LedgerEntrySigningPayload).
   480	    #[test]
   481	    fn canonical_digest_excludes_derivatives() {
   482	        let e_clean = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   483	        let digest_clean = e_clean.to_signing_payload().canonical_digest();
   484	
   485	        // Mutate resulting_ledger_root (a derivative; should NOT affect signing digest)
   486	        let mut e_tamper = e_clean.clone();
   487	        e_tamper.resulting_ledger_root = h(0xff);
   488	        let digest_after_root_tamper = e_tamper.to_signing_payload().canonical_digest();
   489	        assert_eq!(
   490	            digest_clean, digest_after_root_tamper,
   491	            "resulting_ledger_root MUST NOT affect signing digest (Q9 cycle prevention)"
   492	        );
   493	
   494	        // Mutate system_signature (signature is its own input; should NOT affect signing digest)
   495	        let mut e_tamper2 = e_clean.clone();
   496	        e_tamper2.system_signature = SystemSignature::from_bytes([0xffu8; 64]);
   497	        let digest_after_sig_tamper = e_tamper2.to_signing_payload().canonical_digest();
   498	        assert_eq!(digest_clean, digest_after_sig_tamper);
   499	
   500	        // Sanity: mutating a SIGNED field DOES change digest
   501	        let mut e_signed_change = e_clean.clone();
   502	        e_signed_change.epoch = SystemEpoch::new(99);
   503	        let digest_after_signed = e_signed_change.to_signing_payload().canonical_digest();
   504	        assert_ne!(digest_clean, digest_after_signed);
   505	    }
   506	}

exec
/bin/bash -lc "nl -ba handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md | sed -n '520,760p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   520	
   521	---
   522	
   523	## § 9 STEP_B disposition (K4 corrected typo)
   524	
   525	CO1.7 lives in NEW files (`src/bottom_white/ledger/transition_ledger.rs`, future `src/state/sequencer.rs`). It does NOT modify `src/bus.rs` / `src/kernel.rs` / `src/wallet.rs` (the STEP_B-restricted files per CLAUDE.md "Code Standard"; v1 incorrectly listed `wal.rs`). Therefore: **no STEP_B parallel-branch ceremony required** for the CO1.7 atom itself.
   526	
   527	**Touched files** (skeleton commit, additive only):
   528	- `src/bottom_white/ledger/transition_ledger.rs` (NEW, ~370 lines)
   529	- `src/bottom_white/ledger/mod.rs` (existing, +1 `pub mod` line — additive)
   530	- `genesis_payload.toml` (TR manifest +1 entry, refreshed mod.rs hash — TR governance, not code edit)
   531	
   532	Future runtime wiring (CO1.7.5+) into `bus.rs`/`kernel.rs` WILL need STEP_B — that's a separate atom. The retirement of `src/ledger.rs` (legacy top-level) is in CO1.1.5 per `STATE_TRANSITION_SPEC § 5.3`.
   533	
   534	CO1.7 also extends `src/bottom_white/ledger/system_keypair.rs` (CanonicalMessage variant + sign API); that file is NOT STEP_B-restricted and the change is additive (no behavior change to existing 3 variants).
   535	
   536	---
   537	
   538	## § 10 What this spec does NOT specify
   539	
   540	1. **Garbage collection** — append-only constitutional (Art 0.2); L4 entry deletion never happens.
   541	2. **Cross-cell sharing** — § 5.2.2 mandates disjoint runtime_repo per cell; multi-tenant deployments are v4.x.
   542	3. **Recovery from corrupted git history** — out of scope; if `git fsck` fails, runtime aborts (fail-closed).
   543	4. **Performance tuning** — no SLO commitments. Sequencer single-writer property + CAS metadata write per entry sets approximate throughput floor; rough budget is "≥ 10 tx/sec per cell on 4-core hardware".
   544	5. **CAS index persistence** — deferred to **CO1.4-extra** (NEW atom; not yet planned in Plan v3.2). C2 mitigation route.
   545	6. **Cold-restart full replay** — depends on CO1.4-extra; until then, FullTransition mode errors with `CasMissing` after process restart. ChainOnly mode unaffected.
   546	
   547	---
   548	
   549	## § 11 Open questions resolution (post-round-1)
   550	
   551	| Q | v1 status | v1.1 resolution | Source |
   552	|---|---|---|---|
   553	| Q1 SubmissionQueue type | open | **Bounded `tokio::sync::mpsc::Sender`** with `QueueFull` Err on saturation (Codex Q-G). Async wait variant `submit_async` may be added if multi-agent fairness becomes an issue (deferred). |
   554	| Q2 back-pressure | open | Returns `Err(SubmitError::QueueFull)` immediately (Codex). Agent retry with deterministic exponential backoff (seed from `(agent_id, attempt_count)`). |
   555	| Q3 Sequencer abstraction split | open | **Keep monolithic `Sequencer`** for v1.1 (single-writer constraint makes split synthetic). Trait segregation = v4.x consideration. |
   556	| Q4 signature placement | open | **Inside `LedgerEntry`** (v1 design); BUT signing target is `LedgerEntrySigningPayload` (separate struct, distinct fields). Both auditors agreed. |
   557	| Q5 enum-match vs trait dispatch | open | **Enum-match** for v4 (exhaustive, simple). `MetaTransitionInterface` trait deferred to v4.1 dynamic MetaTx. |
   558	| Q6 replay error mode | open | **Reject on first error** (current). Diagnostic-collection mode is a v4.x extension; first-error simplicity matches CO1.7.5 implementation budget. |
   559	| Q7 genesis ledger_root_t | open | **`sha256("turingosv4.ledger_root.v1.genesis" || sha256(genesis_payload.toml))`** — domain-separated anchor (both auditors agreed). NOT `Hash::ZERO`. |
   560	| Q8 CanonicalMessage extension | open | **Path A**: extend enum with `LedgerEntrySigning(LedgerEntrySigningPayload)` variant; new typed sign API `sign_ledger_entry(payload, epoch)`. Forward-compat clause: future ledger-side message variants add new `CanonicalMessage::*` variants (NOT in-place edits). Both auditors agreed. |
   561	| Q9 canonical_digest exclusion | open | **Excludes**: `resulting_ledger_root` (cycle), `system_signature` (its input). **Includes**: 9 fields explicit in `LedgerEntrySigningPayload`. Spec § 1.1 explicit. |
   562	| Q10 epoch field | open | **Added** to `LedgerEntry` field 9; **bound** in `LedgerEntrySigningPayload` (D1 conservative resolution: Codex security argument). NOT separately folded into ledger_root. |
   563	| Q11 NEW open Qs | — | Codex round-1 listed: parent_ledger_root binding (now K2 / done), rejected-submission logical time (now K1 / done), CAS persistence (now C2 → CO1.4-extra), canonical fixtures (deferred to CO1.7.5+ test stubs), L4/L5 head_t ownership (now K3 / done). All addressed. |
   564	
   565	**v1.1 closed all 11 open Qs from v1**. Round-2 audit's open-Q section starts empty.
   566	
   567	---
   568	
   569	## § 12 Audit gates (round structure)
   570	
   571	| Round | Codex | Gemini | Conservative | Action |
   572	|---|---|---|---|---|
   573	| 1 | CHALLENGE (high) | CHALLENGE (high) | **CHALLENGE** | v1.1 patch round (this version) |
   574	| 2 | ⏳ pending | ⏳ pending | TBD | re-audit on v1.1; expected PASS or 1-issue CHALLENGE |
   575	| 3+ | … | … | … | iterate to PASS/PASS |
   576	
   577	**Pre-implementation gate**: CO1.7 must reach `PASS/PASS` before implementing CO1.7.5 (transition function bodies) + CO1.4-extra (CAS persistence). Sedimented per CLAUDE.md "Audit Standard" (Generator ≠ Evaluator) + memory `feedback_dual_audit`.
   578	
   579	---
   580	
   581	## § 13 Estimated scope (revised)
   582	
   583	- **Spec rounds**: round-1 done; round-2 expected; possible round-3 if Codex finds new edge cases.
   584	- **Implementation scope** (post-PASS/PASS):
   585	  - CO1.7-impl proper: ~600-900 LoC + 8 conformance tests (4 skeleton-stage + 4 CO1.7.5-stage)
   586	  - CO1.4-extra (CAS index persistence): ~150-300 LoC + 3-4 tests (NEW atom; budget add)
   587	  - CO1.7.5 (transition function bodies): separate downstream atom
   588	- **Total atom budget**: ~1.5-2.5 weeks (slight expansion due to CO1.4-extra; matches LATEST line 92 estimate).
   589	
   590	---
   591	
   592	## § 14 Honest acknowledgements (v1.1)
   593	
   594	1. CO1.4 CAS API surface verified via type-skeleton smoke 2026-04-28: `CasStore::get(&Cid) → Result<Vec<u8>, CasError>` matches; `CasStore::put` 5-param signature documented (DIV-5).
   595	2. SubmissionQueue choice = bounded tokio::sync::mpsc per Q1 resolution; pivot to different runtime would rewrite `Sequencer.run()`.
   596	3. system_keypair extension (CanonicalMessage variant + sign_ledger_entry API) is additive Wave 4-B extension; non-breaking; verified via type-skeleton (sign API call site is concrete, not unimplemented).
   597	4. **v1.1 patches incorporate 11 round-1 must-fix items + 1 disagreement resolution** (D1: epoch security wins). Detailed in patch log header.
   598	5. **Spec ↔ skeleton divergences cataloged in v1; resolution status v1.1**:
   599	   - DIV-1 CanonicalMessage integration — **resolved** via Q8 (extend enum, Path A) + § 1.2.
   600	   - DIV-2 Q_t mutation API — unchanged status (CO P2.x economy atoms still unblock); skeleton stays unimplemented!() for state mutation.
   601	   - DIV-3 epoch field — **resolved**: in LedgerEntry + bound in LedgerEntrySigningPayload.
   602	   - DIV-4 CasReader → LedgerCasView — **resolved**: narrow trait for replay_full_transition.
   603	   - DIV-5 CasStore::put 5-param — **resolved**: sequencer apply_one stage 3 builds full metadata.
   604	6. **Q9 spec bug found by skeleton smoke** — closed by spec § 1.1: explicit `LedgerEntrySigningPayload` separate struct excludes derivatives.
   605	7. **K2 transplant attack vector** found by Codex round-1 — closed by binding `parent_ledger_root` in signing payload.
   606	8. **K3 L4/L5 boundary blur** found by Codex round-1 — closed by spec § 0 + § 3 + § 5 boundary clarification.
   607	
   608	---
   609	
   610	## § 15 Pre-audit smoke verification (2026-04-28)
   611	
   612	| Smoke item | Result | What it proved |
   613	|---|---|---|
   614	| `cargo check` on `src/bottom_white/ledger/transition_ledger.rs` | PASS | LedgerEntry / TxKind / append / replay_chain_integrity / InMemoryLedgerWriter all type-check against shipped CO1.4 + CO1.7.0a-f + Q_t types |
   615	| `cargo test --lib bottom_white::ledger::transition_ledger::` | 6/6 PASS | append byte-stable; canonical_digest stable; in-memory writer enforces logical_t monotonic; ChainOnly replay validates parent chain + rejects 2 tamper modes |
   616	| `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` | PASS | TR manifest aligned with skeleton + spec |
   617	| `cargo test --lib` (full workspace) | 196/0 PASS | no regression in 190 pre-existing tests |
   618	
   619	**v1.1 skeleton update plan** (separate commit): apply K6 (#[repr(u8)]), K5 (drop Slash), G1 (extensions field), C3 (sign through CanonicalMessage extension), K2 (parent_ledger_root field + new sign payload struct), Q9 (canonical_digest moves to LedgerEntrySigningPayload), K7 (add 2 new tests for parent_ledger_root tamper + digest exclusion). Target: 8 skeleton tests PASS, full workspace still 196+/0 PASS.
   620	
   621	---
   622	
   623	## § 16 Round-1 audit closure verification
   624	
   625	| Audit finding | Closure mechanism | v1.1 location |
   626	|---|---|---|
   627	| C1 replay two-mode | New `ReplayMode` enum + spec § 4 + I-DETHASH bound to FullTransition only | § 0, § 4, § 6 |
   628	| C2 CAS cold-replay risk | New CO1.4-extra atom + § 0 explicit dependency note + ReplayError::CasMissing | § 0, § 5, § 13 |
   629	| C3 signing primitive integration | LedgerEntrySigningPayload struct + CanonicalMessage extension + sign_ledger_entry API | § 1.1, § 1.2 |
   630	| K1 sequencer logical_t skip race | Dual counter design (next_submit_id, next_logical_t) | § 3 |
   631	| K2 parent_ledger_root binding | Field added + bound in signing payload + new test | § 1, § 1.1, § 7 |
   632	| K3 L4/L5 head_t ownership | Boundary clarified: CO1.7 owns ledger_root + commit-chain head_t (NodeId(commit_sha)); CO1.8 owns state_root | § 0, § 3, § 5 |
   633	| K4 trait mismatch | Spec aligned to skeleton: `&mut self` + `Hash` return; iter_from deferred | § 5 |
   634	| K5 Slash dispatch gap | Slash variant DROPPED for v4; deferred to CO P2.5 | § 1, § 6, § 8 |
   635	| K6 #[repr(u8)] | Added with explicit discriminants | § 1 |
   636	| K7 conformance test gap | Explicit 8 tests (4 skeleton + 4 CO1.7.5+ stage) | § 7 |
   637	| G1 forward-compat extensions | `extensions: BTreeMap<String, Vec<u8>>` in LedgerEntry; bound in signing payload | § 1, § 1.1 |
   638	| D1 epoch binding (Codex/Gemini disagree) | Conservative resolution: bound in signing payload (Codex security wins) | § 1.1 |
   639	
   640	**12 closures** (11 must-fix + 1 disagreement) integrated. Round-2 audit input minimal: residual issues only.
   641	
   642	— ArchitectAI, session 2026-04-28; round-1 closure 2026-04-28.

exec
/bin/bash -lc 'rg -n "LedgerEntrySigning|sign_ledger_entry|CanonicalMessage::LedgerEntrySigning|ObjectType::Transition|ReplayError|TransitionError|CasMissing|extensions|Slash" src handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md genesis_payload.toml' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:24:| C3 | signing primitive integration via `CanonicalMessage` enum was unspecified; spec called nonexistent digest-form verifier | Path A: extend `CanonicalMessage::LedgerEntrySigning(LedgerEntrySigningPayload)`; sign separate signing payload (NOT raw `LedgerEntry`); new API `keypair.sign_ledger_entry(payload) → SystemSignature` | Codex Q-G + Gemini Q4 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:26:| K2 | signature did NOT bind `parent_ledger_root` → transplant attack | `LedgerEntrySigningPayload` includes `parent_ledger_root` field | Codex Q-B (NEW) |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:29:| K5 | `TxKind::Slash` enum variant present but `dispatch_transition` omitted it | Drop `TxKind::Slash` for v4; ChallengeCourt slashing event scheduled for CO P2.5 atom | Codex Q-H |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:32:| G1 | `LedgerEntry` struct rigid; future ZK / settlement proof had no place | Add `extensions: BTreeMap<String, Vec<u8>>` (empty in v1; reserved for v4.x without breaking schema) | Gemini Q9 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:33:| D1 | epoch binding disagreement (Codex bind YES; Gemini bind NO) | Conservative resolution: epoch IS bound in `LedgerEntrySigningPayload`; epoch NOT separately folded into `ledger_root_t` (Codex security wins; Gemini orthogonality preserved at the ledger_root axis) | merged verdict § 5 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:42:- **LedgerEntry schema**: canonical envelope wrapping each typed transition (WorkTx / VerifyTx / ChallengeTx / ReuseTx / FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx) before append to L4. **Note**: `Slash` is NOT in v4 (deferred to CO P2.5 ChallengeCourt atom — K5).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:43:- **LedgerEntrySigningPayload**: the 8-field bytes-on-the-wire that the system keypair actually signs (distinct from LedgerEntry-the-stored-record).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:49:- **CanonicalMessage extension**: extends shipped `CanonicalMessage` enum with `LedgerEntrySigning(LedgerEntrySigningPayload)` variant; new sign API `keypair.sign_ledger_entry(payload)`.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:59:- **Slash transition** — deferred to CO P2.5 (ChallengeCourt) atom; v4 ledger has no `TxKind::Slash`.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:105:    /// where signing_digest_t = canonical_digest(LedgerEntrySigningPayload at logical_t).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:121:    pub extensions: BTreeMap<String, Vec<u8>>,   // 10
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:123:    /// Detached system signature over canonical_digest of LedgerEntrySigningPayload.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:138:    // K5: NO `Slash` — ChallengeCourt slash event is in CO P2.5; v4 ledger has no Slash variant.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:142:### § 1.1 LedgerEntrySigningPayload (NEW per C3)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:146:2. **Binds non-derivatives**: `parent_ledger_root` (K2 transplant defense) + `epoch` (D1 + Q10) + `extensions` (G1 forward compat is signed).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:151:pub struct LedgerEntrySigningPayload {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:160:    pub extensions: BTreeMap<String, Vec<u8>>,  // 9 (G1; empty-map case yields empty bytes)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:164:impl LedgerEntrySigningPayload {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:176:        // extensions: BTreeMap iteration is sorted by key (deterministic);
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:178:        h.update((self.extensions.len() as u64).to_be_bytes());
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:179:        for (k, v) in &self.extensions {                        // BTreeMap = lex order
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:200:    LedgerEntrySigning(LedgerEntrySigningPayload),   // NEW (C3)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:204://   CanonicalMessage::LedgerEntrySigning(payload) => {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:205://       h.update(b"LedgerEntrySigning");
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:211:    pub fn sign_ledger_entry(&self, payload: &LedgerEntrySigningPayload, epoch: SystemEpoch) -> SystemSignature;
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:215:**Forward-compat clause** (per Gemini Q4 + the audit response): if v4.x adds new ledger-side message variants, they MUST add new `CanonicalMessage::*` variants (NOT extend `LedgerEntrySigningPayload` in-place). v4-shipped extensions go in the `LedgerEntry::extensions` BTreeMap (G1) which IS bound in this signed digest.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:225:└── transition_ledger.rs         (NEW; LedgerEntry, LedgerEntrySigningPayload, TxKind, append, replay_*, LedgerWriter)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:282:    fn apply_one(&self, tx: TypedTx) -> Result<LedgerEntry, TransitionError> {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:295:            ObjectType::Transition,
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:298:            Some("LedgerEntrySigningPayload.v1".to_string()),
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:305:        // 5. Build LedgerEntrySigningPayload
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:306:        let signing_payload = LedgerEntrySigningPayload {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:315:            extensions: BTreeMap::new(),                     // G1 empty in v1
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:319:        let system_signature = self.keypair.sign_ledger_entry(&signing_payload, self.epoch);
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:336:            extensions: signing_payload.extensions,
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:398:) -> Result<(Hash, Hash), ReplayError>;
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:406:) -> Result<QState, ReplayError>;
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:411:**ReplayError enum** (skeleton already has 3 variants; v1.1 adds 4 more for FullTransition):
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:417:- `CasMissing { at, cid }` (NEW; FullTransition only — fires if CO1.4-extra not yet landed)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:419:- `TransitionError { at, inner }` (NEW; wraps dispatch_transition errors)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:432:- **C2**: cold-replay availability requires `CasStore` index persistence; deferred to CO1.4-extra. Until then, full-mode replay errors with `CasMissing` if CAS state is not warm.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:467:| **I-FINALIZE-EXCLUSIVE** | finalize_reward and slash mutually exclusive per claim | **v4 has no Slash** (K5); invariant trivially holds via TxKind enum |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:481:| `tests/canonical_digest_stable` | skeleton (v1) | LedgerEntrySigningPayload digest stable across clones; #[repr(u8)] discriminant stable |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:487:| `tests/canonical_digest_excludes_derivatives` | **Q9 NEW** skeleton (v1) | LedgerEntrySigningPayload.canonical_digest excludes resulting_ledger_root + system_signature; mutation of either does NOT change digest |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:489:| `tests/system_signature_verifies_via_canonical_message` | CO1.7.5+ | LedgerEntry.system_signature verifies through `verify_system_signature(&CanonicalMessage::LedgerEntrySigning(...), epoch, pinned_pubkeys)` |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:497:## § 8 dispatch_transition (K5 Slash dropped)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:505:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:514:        // K5: NO `TypedTx::Slash` — v4 has no slash transition.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:545:6. **Cold-restart full replay** — depends on CO1.4-extra; until then, FullTransition mode errors with `CasMissing` after process restart. ChainOnly mode unaffected.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:556:| Q4 signature placement | open | **Inside `LedgerEntry`** (v1 design); BUT signing target is `LedgerEntrySigningPayload` (separate struct, distinct fields). Both auditors agreed. |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:560:| Q8 CanonicalMessage extension | open | **Path A**: extend enum with `LedgerEntrySigning(LedgerEntrySigningPayload)` variant; new typed sign API `sign_ledger_entry(payload, epoch)`. Forward-compat clause: future ledger-side message variants add new `CanonicalMessage::*` variants (NOT in-place edits). Both auditors agreed. |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:561:| Q9 canonical_digest exclusion | open | **Excludes**: `resulting_ledger_root` (cycle), `system_signature` (its input). **Includes**: 9 fields explicit in `LedgerEntrySigningPayload`. Spec § 1.1 explicit. |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:562:| Q10 epoch field | open | **Added** to `LedgerEntry` field 9; **bound** in `LedgerEntrySigningPayload` (D1 conservative resolution: Codex security argument). NOT separately folded into ledger_root. |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:596:3. system_keypair extension (CanonicalMessage variant + sign_ledger_entry API) is additive Wave 4-B extension; non-breaking; verified via type-skeleton (sign API call site is concrete, not unimplemented).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:601:   - DIV-3 epoch field — **resolved**: in LedgerEntry + bound in LedgerEntrySigningPayload.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:604:6. **Q9 spec bug found by skeleton smoke** — closed by spec § 1.1: explicit `LedgerEntrySigningPayload` separate struct excludes derivatives.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:619:**v1.1 skeleton update plan** (separate commit): apply K6 (#[repr(u8)]), K5 (drop Slash), G1 (extensions field), C3 (sign through CanonicalMessage extension), K2 (parent_ledger_root field + new sign payload struct), Q9 (canonical_digest moves to LedgerEntrySigningPayload), K7 (add 2 new tests for parent_ledger_root tamper + digest exclusion). Target: 8 skeleton tests PASS, full workspace still 196+/0 PASS.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:628:| C2 CAS cold-replay risk | New CO1.4-extra atom + § 0 explicit dependency note + ReplayError::CasMissing | § 0, § 5, § 13 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:629:| C3 signing primitive integration | LedgerEntrySigningPayload struct + CanonicalMessage extension + sign_ledger_entry API | § 1.1, § 1.2 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:634:| K5 Slash dispatch gap | Slash variant DROPPED for v4; deferred to CO P2.5 | § 1, § 6, § 8 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:637:| G1 forward-compat extensions | `extensions: BTreeMap<String, Vec<u8>>` in LedgerEntry; bound in signing payload | § 1, § 1.1 |
src/bottom_white/ledger/transition_ledger.rs:21://! - K5: `TxKind::Slash` DROPPED for v4 (deferred to CO P2.5).
src/bottom_white/ledger/transition_ledger.rs:24://! - G1: `extensions: BTreeMap<String, Vec<u8>>` forward-compat field (empty in v1).
src/bottom_white/ledger/transition_ledger.rs:25://! - C3 / Q8: signing target is `LedgerEntrySigningPayload` (separate struct) ready to
src/bottom_white/ledger/transition_ledger.rs:26://!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
src/bottom_white/ledger/transition_ledger.rs:29://! - Q9: canonical_digest now lives on LedgerEntrySigningPayload, not LedgerEntry —
src/bottom_white/ledger/transition_ledger.rs:47:/// **K5**: NO `Slash` variant — ChallengeCourt slash event deferred to CO P2.5 atom.
src/bottom_white/ledger/transition_ledger.rs:62:/// Distinct from `LedgerEntrySigningPayload`: this is the FULL stored record
src/bottom_white/ledger/transition_ledger.rs:87:    pub extensions: BTreeMap<String, Vec<u8>>,   // 10
src/bottom_white/ledger/transition_ledger.rs:88:    /// Detached system signature over `LedgerEntrySigningPayload.canonical_digest()`.
src/bottom_white/ledger/transition_ledger.rs:93:// § 1.1 LedgerEntrySigningPayload — the signed bytes (NEW per C3 / Q9)
src/bottom_white/ledger/transition_ledger.rs:105:pub struct LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:114:    pub extensions: BTreeMap<String, Vec<u8>>,     // G1
src/bottom_white/ledger/transition_ledger.rs:117:impl LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:132:        h.update((self.extensions.len() as u64).to_be_bytes());
src/bottom_white/ledger/transition_ledger.rs:133:        for (k, v) in &self.extensions {
src/bottom_white/ledger/transition_ledger.rs:146:    pub fn to_signing_payload(&self) -> LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:147:        LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:156:            extensions: self.extensions.clone(),
src/bottom_white/ledger/transition_ledger.rs:267:pub enum ReplayError {
src/bottom_white/ledger/transition_ledger.rs:274:    CasMissing { at: usize },
src/bottom_white/ledger/transition_ledger.rs:278:impl std::fmt::Display for ReplayError {
src/bottom_white/ledger/transition_ledger.rs:288:            Self::CasMissing { at } => write!(f, "CAS payload not retrievable at index {at}"),
src/bottom_white/ledger/transition_ledger.rs:293:impl std::error::Error for ReplayError {}
src/bottom_white/ledger/transition_ledger.rs:312:) -> Result<(Hash, Hash), ReplayError> {
src/bottom_white/ledger/transition_ledger.rs:319:            return Err(ReplayError::LogicalTGap {
src/bottom_white/ledger/transition_ledger.rs:326:            return Err(ReplayError::ParentStateMismatch { at: i });
src/bottom_white/ledger/transition_ledger.rs:330:            return Err(ReplayError::ParentLedgerMismatch { at: i });
src/bottom_white/ledger/transition_ledger.rs:335:            return Err(ReplayError::LedgerRootMismatch { at: i });
src/bottom_white/ledger/transition_ledger.rs:345:// Tests — 8 conformance items (4 NEW vs v1 skeleton: K2 / Q9 / repr(u8) / extensions)
src/bottom_white/ledger/transition_ledger.rs:363:        let signing = LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:372:            extensions: BTreeMap::new(),
src/bottom_white/ledger/transition_ledger.rs:386:            extensions: signing.extensions,
src/bottom_white/ledger/transition_ledger.rs:404:        let p = LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:413:            extensions: BTreeMap::new(),
src/bottom_white/ledger/transition_ledger.rs:452:        assert!(matches!(err, ReplayError::ParentStateMismatch { at: 1 }));
src/bottom_white/ledger/transition_ledger.rs:464:        assert!(matches!(err, ReplayError::ParentLedgerMismatch { at: 1 }));
src/bottom_white/ledger/transition_ledger.rs:474:        assert!(matches!(err, ReplayError::LedgerRootMismatch { at: 1 }));
src/bottom_white/ledger/transition_ledger.rs:479:    // change the signing payload digest (because they're not in LedgerEntrySigningPayload).
src/drivers/llm_proxy.py:250:        # Slash-form like "deepseek-ai/..." won't reach here per K2.

codex
The spec has one concrete pseudocode bug emerging in §3 before final verdict: it uses `ObjectType::Transition`, but the shipped CAS `ObjectType` has no `Transition` variant. I’m running the target tests and trust-root check now to separate skeleton health from spec compile-readiness.
exec
/bin/bash -lc 'sha256sum handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md src/bottom_white/ledger/transition_ledger.rs src/bottom_white/ledger/mod.rs genesis_payload.toml && rg -n "CO1_7_TRANSITION|transition_ledger.rs|bottom_white/ledger/mod.rs|system_keypair.rs" genesis_payload.toml' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
1c9fba2cf54ee6f08bf869f592a60518df87ed6756d806adc6fd0cd3ade32d9f  handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md
7ce0f403db06f6fa437e0e94228e100ba2f4295a90cb3d22023f26b3ff50771e  src/bottom_white/ledger/transition_ledger.rs
edb4cda94b80dceb85a414de58139fde4f6f4635867141e31c14720ab06148ea  src/bottom_white/ledger/mod.rs
a184938838f5ad68e6bd569da80b994965e441c7526973bb2aa2305ecf40bce8  genesis_payload.toml
252:"handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md" = "1c9fba2cf54ee6f08bf869f592a60518df87ed6756d806adc6fd0cd3ade32d9f"
281:"src/bottom_white/ledger/mod.rs" = "edb4cda94b80dceb85a414de58139fde4f6f4635867141e31c14720ab06148ea"
282:"src/bottom_white/ledger/system_keypair.rs" = "d68e1d3403128b26f6fe6d8f4b3686b77b13cb4a6cb360965909a638d653bc8a"
284:"src/bottom_white/ledger/transition_ledger.rs" = "7ce0f403db06f6fa437e0e94228e100ba2f4295a90cb3d22023f26b3ff50771e"

exec
/bin/bash -lc 'cargo test --lib bottom_white::ledger::transition_ledger::' in /home/zephryj/projects/turingosv4
 succeeded in 273ms:
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
 --> src/bottom_white/cas/store.rs:9:12
  |
9 | use git2::{ObjectType as Git2ObjectType, Repository};
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
   --> src/bottom_white/ledger/system_keypair.rs:492:19
    |
492 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_terminal_summary_tx` is never used
   --> src/bottom_white/ledger/system_keypair.rs:508:19
    |
508 |     pub(crate) fn sign_terminal_summary_tx(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:527:19
    |
527 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib test) generated 13 warnings (run `cargo fix --lib -p turingosv4 --tests` to apply 10 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-128d20df5fbbc885)

running 8 tests
test bottom_white::ledger::transition_ledger::tests::append_is_pure_and_byte_stable ... ok
test bottom_white::ledger::transition_ledger::tests::in_memory_writer_enforces_logical_t ... ok
test bottom_white::ledger::transition_ledger::tests::canonical_digest_stable_across_clones ... ok
test bottom_white::ledger::transition_ledger::tests::canonical_digest_excludes_derivatives ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::replay_chain_integrity_clean ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 190 filtered out; finished in 0.00s


exec
/bin/bash -lc 'cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo' in /home/zephryj/projects/turingosv4
 succeeded in 396ms:
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
 --> src/bottom_white/cas/store.rs:9:12
  |
9 | use git2::{ObjectType as Git2ObjectType, Repository};
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
   --> src/bottom_white/ledger/system_keypair.rs:492:19
    |
492 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_terminal_summary_tx` is never used
   --> src/bottom_white/ledger/system_keypair.rs:508:19
    |
508 |     pub(crate) fn sign_terminal_summary_tx(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:527:19
    |
527 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib test) generated 13 warnings (run `cargo fix --lib -p turingosv4 --tests` to apply 10 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-128d20df5fbbc885)

running 1 test
test boot::tests::verify_trust_root_passes_on_intact_repo ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 197 filtered out; finished in 0.15s


exec
/bin/bash -lc "sha256sum src/bottom_white/ledger/system_keypair.rs src/bottom_white/cas/schema.rs src/bottom_white/cas/store.rs && nl -ba src/bottom_white/cas/schema.rs | sed -n '40,60p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
d68e1d3403128b26f6fe6d8f4b3686b77b13cb4a6cb360965909a638d653bc8a  src/bottom_white/ledger/system_keypair.rs
75ffc356c7abb50f397ed6f160786cdbfd3ed9cfadbf8f20d931efe6d49d185c  src/bottom_white/cas/schema.rs
c77976fec2bc8d2d3b2b5e93e3b9ac39339942eae3c8fd1c9822ee4c6fe07ad6  src/bottom_white/cas/store.rs
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

exec
/bin/bash -lc 'rg -n "pub enum CanonicalMessage|LedgerEntrySigning|sign_ledger_entry|ObjectType::Transition|pub enum ObjectType|Transition," src/bottom_white/ledger/system_keypair.rs src/bottom_white/cas/schema.rs handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md src/bottom_white/ledger/transition_ledger.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/ledger/transition_ledger.rs:25://! - C3 / Q8: signing target is `LedgerEntrySigningPayload` (separate struct) ready to
src/bottom_white/ledger/transition_ledger.rs:26://!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
src/bottom_white/ledger/transition_ledger.rs:29://! - Q9: canonical_digest now lives on LedgerEntrySigningPayload, not LedgerEntry —
src/bottom_white/ledger/transition_ledger.rs:62:/// Distinct from `LedgerEntrySigningPayload`: this is the FULL stored record
src/bottom_white/ledger/transition_ledger.rs:88:    /// Detached system signature over `LedgerEntrySigningPayload.canonical_digest()`.
src/bottom_white/ledger/transition_ledger.rs:93:// § 1.1 LedgerEntrySigningPayload — the signed bytes (NEW per C3 / Q9)
src/bottom_white/ledger/transition_ledger.rs:105:pub struct LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:117:impl LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:146:    pub fn to_signing_payload(&self) -> LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:147:        LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:263:    FullTransition,
src/bottom_white/ledger/transition_ledger.rs:363:        let signing = LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:404:        let p = LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:479:    // change the signing payload digest (because they're not in LedgerEntrySigningPayload).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:24:| C3 | signing primitive integration via `CanonicalMessage` enum was unspecified; spec called nonexistent digest-form verifier | Path A: extend `CanonicalMessage::LedgerEntrySigning(LedgerEntrySigningPayload)`; sign separate signing payload (NOT raw `LedgerEntry`); new API `keypair.sign_ledger_entry(payload) → SystemSignature` | Codex Q-G + Gemini Q4 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:26:| K2 | signature did NOT bind `parent_ledger_root` → transplant attack | `LedgerEntrySigningPayload` includes `parent_ledger_root` field | Codex Q-B (NEW) |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:33:| D1 | epoch binding disagreement (Codex bind YES; Gemini bind NO) | Conservative resolution: epoch IS bound in `LedgerEntrySigningPayload`; epoch NOT separately folded into `ledger_root_t` (Codex security wins; Gemini orthogonality preserved at the ledger_root axis) | merged verdict § 5 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:43:- **LedgerEntrySigningPayload**: the 8-field bytes-on-the-wire that the system keypair actually signs (distinct from LedgerEntry-the-stored-record).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:49:- **CanonicalMessage extension**: extends shipped `CanonicalMessage` enum with `LedgerEntrySigning(LedgerEntrySigningPayload)` variant; new sign API `keypair.sign_ledger_entry(payload)`.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:105:    /// where signing_digest_t = canonical_digest(LedgerEntrySigningPayload at logical_t).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:123:    /// Detached system signature over canonical_digest of LedgerEntrySigningPayload.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:142:### § 1.1 LedgerEntrySigningPayload (NEW per C3)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:151:pub struct LedgerEntrySigningPayload {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:164:impl LedgerEntrySigningPayload {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:196:pub enum CanonicalMessage {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:200:    LedgerEntrySigning(LedgerEntrySigningPayload),   // NEW (C3)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:204://   CanonicalMessage::LedgerEntrySigning(payload) => {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:205://       h.update(b"LedgerEntrySigning");
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:211:    pub fn sign_ledger_entry(&self, payload: &LedgerEntrySigningPayload, epoch: SystemEpoch) -> SystemSignature;
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:215:**Forward-compat clause** (per Gemini Q4 + the audit response): if v4.x adds new ledger-side message variants, they MUST add new `CanonicalMessage::*` variants (NOT extend `LedgerEntrySigningPayload` in-place). v4-shipped extensions go in the `LedgerEntry::extensions` BTreeMap (G1) which IS bound in this signed digest.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:225:└── transition_ledger.rs         (NEW; LedgerEntry, LedgerEntrySigningPayload, TxKind, append, replay_*, LedgerWriter)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:295:            ObjectType::Transition,
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:298:            Some("LedgerEntrySigningPayload.v1".to_string()),
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:305:        // 5. Build LedgerEntrySigningPayload
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:306:        let signing_payload = LedgerEntrySigningPayload {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:319:        let system_signature = self.keypair.sign_ledger_entry(&signing_payload, self.epoch);
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:390:    FullTransition,
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:481:| `tests/canonical_digest_stable` | skeleton (v1) | LedgerEntrySigningPayload digest stable across clones; #[repr(u8)] discriminant stable |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:487:| `tests/canonical_digest_excludes_derivatives` | **Q9 NEW** skeleton (v1) | LedgerEntrySigningPayload.canonical_digest excludes resulting_ledger_root + system_signature; mutation of either does NOT change digest |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:489:| `tests/system_signature_verifies_via_canonical_message` | CO1.7.5+ | LedgerEntry.system_signature verifies through `verify_system_signature(&CanonicalMessage::LedgerEntrySigning(...), epoch, pinned_pubkeys)` |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:556:| Q4 signature placement | open | **Inside `LedgerEntry`** (v1 design); BUT signing target is `LedgerEntrySigningPayload` (separate struct, distinct fields). Both auditors agreed. |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:560:| Q8 CanonicalMessage extension | open | **Path A**: extend enum with `LedgerEntrySigning(LedgerEntrySigningPayload)` variant; new typed sign API `sign_ledger_entry(payload, epoch)`. Forward-compat clause: future ledger-side message variants add new `CanonicalMessage::*` variants (NOT in-place edits). Both auditors agreed. |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:561:| Q9 canonical_digest exclusion | open | **Excludes**: `resulting_ledger_root` (cycle), `system_signature` (its input). **Includes**: 9 fields explicit in `LedgerEntrySigningPayload`. Spec § 1.1 explicit. |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:562:| Q10 epoch field | open | **Added** to `LedgerEntry` field 9; **bound** in `LedgerEntrySigningPayload` (D1 conservative resolution: Codex security argument). NOT separately folded into ledger_root. |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:596:3. system_keypair extension (CanonicalMessage variant + sign_ledger_entry API) is additive Wave 4-B extension; non-breaking; verified via type-skeleton (sign API call site is concrete, not unimplemented).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:601:   - DIV-3 epoch field — **resolved**: in LedgerEntry + bound in LedgerEntrySigningPayload.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:604:6. **Q9 spec bug found by skeleton smoke** — closed by spec § 1.1: explicit `LedgerEntrySigningPayload` separate struct excludes derivatives.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:619:**v1.1 skeleton update plan** (separate commit): apply K6 (#[repr(u8)]), K5 (drop Slash), G1 (extensions field), C3 (sign through CanonicalMessage extension), K2 (parent_ledger_root field + new sign payload struct), Q9 (canonical_digest moves to LedgerEntrySigningPayload), K7 (add 2 new tests for parent_ledger_root tamper + digest exclusion). Target: 8 skeleton tests PASS, full workspace still 196+/0 PASS.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:629:| C3 signing primitive integration | LedgerEntrySigningPayload struct + CanonicalMessage extension + sign_ledger_entry API | § 1.1, § 1.2 |
src/bottom_white/cas/schema.rs:42:pub enum ObjectType {
src/bottom_white/ledger/system_keypair.rs:188:pub enum CanonicalMessage {

codex
# Codex CO1.7 Round-2 Audit

## Q-A1..A12 Round-1 Closure Judgments
- **Q-A1 C1 replay two-mode: CLOSED.** `ChainOnly` vs `FullTransition` is explicit, and I-DETHASH is bound only to full replay.
- **Q-A2 C2 CAS cold-replay: CLOSED.** CO1.4-extra is an acceptable dependency split; `CasMissing` is adequate until full replay ships.
- **Q-A3 C3 signing integration: PARTIAL.** The signing payload digest is structurally right, but shipped [system_keypair.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:188) still has only 3 `CanonicalMessage` variants and no `sign_ledger_entry`; skeleton also says this is deferred in [transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:25).
- **Q-A4 K1 sequencer: CLOSED.** Dual counter design fixes skipped accepted `logical_t`, assuming single-driver `apply_one`.
- **Q-A5 K2 transplant: PARTIAL.** `parent_ledger_root` is in the digest before signing, but the skeleton test only checks chain mismatch, not the signature transplant path.
- **Q-A6 K3 L4/L5 boundary: PARTIAL.** `from_state_root` is removed, but §3 still says `head_t = NodeId(commit_sha)` while `LedgerWriter::commit` returns `Hash`, not commit SHA; see spec [§3](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:347) and [§5](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:438).
- **Q-A7 K4 trait: CLOSED.** `commit(&mut self) -> Hash` matches skeleton; `iter_from` deferral is OK for CO1.7.5+.
- **Q-A8 K5 Slash drop: CLOSED.** No `Slash` variant in skeleton; deferring ChallengeCourt slash to CO P2.5 is cleaner.
- **Q-A9 K6 repr: CLOSED.** `#[repr(u8)]` + explicit discriminants are present.
- **Q-A10 K7 tests: CLOSED.** 8 skeleton tests are present and pass.
- **Q-A11 G1 extensions: CLOSED.** `BTreeMap` + length prefixes are collision-safe. Upgrade policy is conservative: new extension semantics require version-aware verifiers.
- **Q-A12 D1 epoch binding: CLOSED.** Epoch is bound in spec and skeleton digest. Old-epoch key compromise cannot verify as current epoch without current private key or poisoned pinned pubkeys.

## Q-B1..B4 New v1.1 Issues
- **Q-B1: NEW-ISSUE.** The CAS metadata timing is not racy under the stated single-driver sequencer, but spec §3 uses `ObjectType::Transition` at [line 295](/home/zephryj/projects/turingosv4/handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:293); shipped `ObjectType` has no such variant in [schema.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/schema.rs:42). Epoch in `creator` is not a material leak because epoch is public in the entry.
- **Q-B2: PARTIAL.** Adding `CanonicalMessage` variants is acceptable, but ownership should be explicit: variants are added in `system_keypair`, not ad hoc by arbitrary modules.
- **Q-B3: CLOSED.** Full-only `ReplayError` variants are not reachable from `ChainOnly` today. Splitting enums is optional.
- **Q-B4: CLOSED.** Cloning multi-KB extensions is acceptable at this stage; digesting is already O(N). Borrowed view can be a later optimization.

## Q-C1..C3 Independent Verification
- **Q-C1:** `cargo test --lib bottom_white::ledger::transition_ledger::` passed: 8/8 tests. The tests structurally support chain integrity, but not full signature verification.
- **Q-C2:** LedgerEntry spec and skeleton match on 11 fields, order, and types. Minor doc typo: spec says “8-field bytes-on-the-wire” while `LedgerEntrySigningPayload` has 9 fields.
- **Q-C3:** TR manifest is aligned. Hashes for spec, skeleton, `mod.rs`, and existing `system_keypair.rs` match `genesis_payload.toml`; `boot::tests::verify_trust_root_passes_on_intact_repo` passed.

## **VERDICT**: CHALLENGE

No VETO. The design is fundamentally sound, but v1.1 has three concrete patch blockers before PASS.

## Top Must-Fix For v1.2
1. **Actually close C3 in code or stop claiming it is closed.** Add `CanonicalMessage::LedgerEntrySigning`, `canonical_digest` arm, and `sign_ledger_entry` to [system_keypair.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:188), plus a skeleton test that verifies signature rejection after `parent_ledger_root` mutation.
2. **Fix the K3 head_t/commit return contradiction.** Either make `LedgerWriter::commit` return a commit `NodeId`/commit result, or explicitly defer `head_t` mutation and remove the “CO1.7 owns head_t now” claim from spec §0/§3/§5.
3. **Fix the CAS object type mismatch.** Add `ObjectType::Transition` to [schema.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/schema.rs:42), or change spec §3 to use an existing CAS `ObjectType`.

## Conviction
High.
2026-04-28T07:45:05.742566Z ERROR codex_core::session: failed to record rollout items: thread 019dd309-cb98-7702-84d0-b2a3aa23606d not found


---

# CAS schema (verify R2-C2-CAS — ObjectType::ProposalPayload exists)

```rust
//! CAS object schema per WP architecture § 5.L3.
//!
//! /// TRACE_MATRIX WP-arch-§5.L3: CAS object schema

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Content-addressed identifier — sha256 of payload bytes.
///
/// Distinct from git's SHA-1 OID (which is an internal storage detail of
/// the git2-rs backend). `Cid` is the v4-canonical identifier; spec § 1.2
/// `WorkTx.proposal_cid: Cid` references this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Cid(pub [u8; 32]);

impl Cid {
    /// Compute Cid from content bytes.
    pub fn from_content(content: &[u8]) -> Self {
        let mut h = Sha256::new();
        h.update(content);
        Self(h.finalize().into())
    }

    /// Hex-encoded representation (lowercase; 64 chars).
    pub fn hex(&self) -> String {
        let mut s = String::with_capacity(64);
        for b in self.0 {
            s.push_str(&format!("{:02x}", b));
        }
        s
    }
}

impl std::fmt::Display for Cid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cid:{}", self.hex())
    }
}

/// Type tag for CAS objects (replaces inline string-typed kind).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectType {
    /// Agent's work_tx proposal payload (Lean proof, code patch, etc.).
    ProposalPayload,
    /// Challenger's counterexample for a slashed claim.
    CounterexamplePayload,
    /// Predicate bytecode (Lean tactic, WASM module, Rust source bytes).
    PredicateBytecode,
    /// Tool bytecode.
    ToolBytecode,
    /// Constitution diff (for amendment proposals).
    AmendmentDiff,
    /// Reversibility plan attached to a meta_tx.
    ReversibilityPlan,
    /// Generic / unclassified blob.
    Generic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CasObjectMetadata {
    /// Content-addressed identifier (sha256 of content).
    pub cid: Cid,
    /// Backend-specific OID (git sha-1 for git2-rs backend); informational only.
    /// Different backends may have different OID schemes; Cid is canonical.
    pub backend_oid_hex: String,
    pub object_type: ObjectType,
    /// Submitter / author. Use "system" for runtime-emitted objects.
    pub creator: String,
    /// Logical time at insertion (assigned by sequencer; not wall clock).
    pub created_at_logical_t: u64,
    /// Optional schema identifier (JSON Schema URI, type tag, etc.).
    pub schema_id: Option<String>,
    /// Size of content in bytes (informational; not part of canonical hash).
    pub size_bytes: u64,
}

impl CasObjectMetadata {
    /// Canonical hash of metadata for Merkle tree inclusion.
    pub fn canonical_hash(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(self.cid.0);
        h.update(self.backend_oid_hex.as_bytes());
        h.update(serde_json::to_vec(&self.object_type).expect("object_type serialize"));
        h.update(self.creator.as_bytes());
        h.update(self.created_at_logical_t.to_be_bytes());
        if let Some(s) = &self.schema_id {
            h.update(b"\x01");
            h.update(s.as_bytes());
        } else {
            h.update(b"\x00");
        }
        h.update(self.size_bytes.to_be_bytes());
        h.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cid_from_empty_content() {
        let cid = Cid::from_content(b"");
        // SHA-256 of empty input = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        assert_eq!(
            cid.hex(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn cid_deterministic() {
        let cid_a = Cid::from_content(b"hello");
        let cid_b = Cid::from_content(b"hello");
        assert_eq!(cid_a, cid_b);
    }

    #[test]
    fn cid_differs_on_content() {
        let cid_a = Cid::from_content(b"hello");
        let cid_b = Cid::from_content(b"world");
        assert_ne!(cid_a, cid_b);
    }

    #[test]
    fn cid_display_format() {
        let cid = Cid::from_content(b"x");
        let s = cid.to_string();
        assert!(s.starts_with("cid:"));
        assert_eq!(s.len(), 4 + 64);
    }

    #[test]
    fn metadata_canonical_hash_deterministic() {
        let m = CasObjectMetadata {
            cid: Cid::from_content(b"x"),
            backend_oid_hex: "abc123".to_string(),
            object_type: ObjectType::ProposalPayload,
            creator: "alice".to_string(),
            created_at_logical_t: 100,
            schema_id: Some("v1/proposal".to_string()),
            size_bytes: 1,
        };
        assert_eq!(m.canonical_hash(), m.canonical_hash());
    }

    #[test]
    fn metadata_canonical_hash_differs_on_object_type() {
        let base = CasObjectMetadata {
            cid: Cid::from_content(b"x"),
            backend_oid_hex: "abc".to_string(),
            object_type: ObjectType::ProposalPayload,
            creator: "alice".to_string(),
            created_at_logical_t: 100,
            schema_id: None,
            size_bytes: 1,
        };
        let mut variant = base.clone();
        variant.object_type = ObjectType::CounterexamplePayload;
        assert_ne!(base.canonical_hash(), variant.canonical_hash());
    }
}

```

---

Now give your INDEPENDENT round-3 closure check.

codex
I’ll treat this as a narrow closure audit and verify the three previous blockers against the checked-in files, not just the pasted packet. I’ll run the relevant tests after checking the exact code/spec points.
exec
/bin/bash -lc 'pwd && git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4
 M rules/enforcement.log
?? handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md
?? handover/audits/GEMINI_CO1_7_ROUND3_AUDIT_2026-04-28.md

exec
/bin/bash -lc "rg --files -g 'AGENTS.md' -g 'CLAUDE.md' -g '*CO1_7_TRANSITION_LEDGER*' -g 'transition_ledger.rs' -g 'system_keypair.rs' -g 'schema.rs' -g 'genesis_payload.toml'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/cas/schema.rs
src/bottom_white/ledger/system_keypair.rs
src/bottom_white/ledger/transition_ledger.rs
genesis_payload.toml
CLAUDE.md
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md

codex
The repo has pre-existing audit artifacts and an enforcement log modification; I’m leaving them alone. No `AGENTS.md` was found in the repo path search, so I’m applying the user-provided instructions plus `CLAUDE.md`.
exec
/bin/bash -lc "rg -n \"v1\\.2|LedgerEntrySigning|transition_ledger_emitter|sign_ledger_entry|ObjectType::ProposalPayload|ObjectType::Transition|head_t|8-field|9-field|signature_round_trip_and_transplant_defense\" handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md src/bottom_white/ledger/system_keypair.rs src/bottom_white/ledger/transition_ledger.rs src/bottom_white/cas/schema.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/ledger/transition_ledger.rs:19://! - K3: L4/L5 boundary clarified — CO1.7 owns ledger_root + commit-chain head_t;
src/bottom_white/ledger/transition_ledger.rs:25://! - C3 / Q8: signing target is `LedgerEntrySigningPayload` (separate struct) ready to
src/bottom_white/ledger/transition_ledger.rs:26://!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
src/bottom_white/ledger/transition_ledger.rs:29://! - Q9: canonical_digest now lives on LedgerEntrySigningPayload, not LedgerEntry —
src/bottom_white/ledger/transition_ledger.rs:62:/// Distinct from `LedgerEntrySigningPayload`: this is the FULL stored record
src/bottom_white/ledger/transition_ledger.rs:88:    /// Detached system signature over `LedgerEntrySigningPayload.canonical_digest()`.
src/bottom_white/ledger/transition_ledger.rs:93:// § 1.1 LedgerEntrySigningPayload — the signed bytes (NEW per C3 / Q9)
src/bottom_white/ledger/transition_ledger.rs:105:pub struct LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:117:impl LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:146:    pub fn to_signing_payload(&self) -> LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:147:        LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:363:        let signing = LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:404:        let p = LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:479:    // change the signing payload digest (because they're not in LedgerEntrySigningPayload).
src/bottom_white/ledger/transition_ledger.rs:508:    // Verifies: (a) typed sign API works; (b) signature verifies via CanonicalMessage::LedgerEntrySigning;
src/bottom_white/ledger/transition_ledger.rs:511:    fn signature_round_trip_and_transplant_defense() {
src/bottom_white/ledger/transition_ledger.rs:513:            transition_ledger_emitter, CanonicalMessage, Ed25519Keypair, PinnedSystemPubkeys,
src/bottom_white/ledger/transition_ledger.rs:523:        let payload = LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:537:        let sig = transition_ledger_emitter::sign_ledger_entry(&keypair, digest.0)
src/bottom_white/ledger/transition_ledger.rs:538:            .expect("sign_ledger_entry");
src/bottom_white/ledger/transition_ledger.rs:541:        let msg_clean = CanonicalMessage::LedgerEntrySigning(digest.0);
src/bottom_white/ledger/transition_ledger.rs:551:        let msg_tamper = CanonicalMessage::LedgerEntrySigning(digest_tamper.0);
src/bottom_white/ledger/transition_ledger.rs:566:        let msg_other_epoch = CanonicalMessage::LedgerEntrySigning(digest_other_epoch.0);
src/bottom_white/ledger/system_keypair.rs:195:    /// TRACE_MATRIX FC2-Append (CO1.7 v1.2 round-2 closure C3): L4 transition_ledger
src/bottom_white/ledger/system_keypair.rs:197:    /// `LedgerEntrySigningPayload` is computed in `transition_ledger`; this variant
src/bottom_white/ledger/system_keypair.rs:201:    LedgerEntrySigning([u8; 32]),
src/bottom_white/ledger/system_keypair.rs:435:        CanonicalMessage::LedgerEntrySigning(digest) => {
src/bottom_white/ledger/system_keypair.rs:436:            h.update(b"LedgerEntrySigning");
src/bottom_white/ledger/system_keypair.rs:547:/// transition ledger sequencer (CO1.7 v1.2). Authorized emitter pattern per
src/bottom_white/ledger/system_keypair.rs:549:/// `sign_ledger_entry` with the canonical digest of `LedgerEntrySigningPayload`
src/bottom_white/ledger/system_keypair.rs:552:pub(crate) mod transition_ledger_emitter {
src/bottom_white/ledger/system_keypair.rs:558:    /// `LedgerEntrySigningPayload`. Caller (sequencer in CO1.7) is responsible
src/bottom_white/ledger/system_keypair.rs:560:    pub(crate) fn sign_ledger_entry(
src/bottom_white/ledger/system_keypair.rs:566:            &CanonicalMessage::LedgerEntrySigning(signing_payload_digest),
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:1:# CO1.7 Transition Ledger v1.2 — Round-2 closure
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:3:**Status**: v1.2 — round-2 returned PASS (Gemini, high) + CHALLENGE (Codex, high; 3 narrow patch blockers). Conservative merged CHALLENGE. v1.2 closes the 3 v1.1→v1.2 patches: (a) C3 actually wired in code (`CanonicalMessage::LedgerEntrySigning([u8;32])` + `transition_ledger_emitter::sign_ledger_entry`); (b) K3 head_t mutation explicitly deferred to CO1.7.5+ (no longer claimed in v1.x); (c) `ObjectType::Transition` replaced with shipped `ObjectType::ProposalPayload`. Plus typo fix and 1 new test. Awaiting round-3.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:20:## v1.2 patch log (vs. v1.1) — round-2 closure
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:22:| ID | v1.1 issue | v1.2 fix | Source |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:24:| **R2-C3** | Spec claimed "C3 CLOSED" but `system_keypair.rs` had no LedgerEntry path; skeleton itself said "deferred to CO1.7.5+" | Wave 4-B additive extension shipped: `CanonicalMessage::LedgerEntrySigning([u8;32])` (opaque digest variant; avoids transition_ledger ↔ system_keypair circular dep) + `canonical_digest` match arm + new `pub(crate) mod transition_ledger_emitter` with `sign_ledger_entry(keypair, digest)`. Skeleton test 9 (`signature_round_trip_and_transplant_defense`) now exercises the real roundtrip + K2 + D1 defenses. | Codex round-2 must-fix #1 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:25:| **R2-K3** | Spec § 3 / § 5 said "CO1.7 owns head_t = NodeId(commit_sha)" but `LedgerWriter::commit` returns `Hash` not commit SHA; v1.1 InMemoryLedgerWriter has no commit_sha to return at all → contradiction | head_t mutation explicitly **deferred to CO1.7.5+** (when Git2LedgerWriter exists and can return both Hash + commit SHA). v1.x ledger owns `ledger_root_t` only; `head_t` continues to be set elsewhere (currently QState placeholder; CO1.7.5 wiring concern). Spec § 0 / § 3 / § 5 updated. | Codex round-2 must-fix #2 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:26:| **R2-C2-CAS** | Spec § 3 sequencer pseudocode used `ObjectType::Transition` but shipped `ObjectType` has no such variant | Spec § 3 changed to `ObjectType::ProposalPayload` (the existing variant for agent work_tx payloads — semantically correct, no schema extension needed) | Codex round-2 must-fix #3 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:27:| **R2-typo** | Spec § 1.1 said "8-field bytes-on-the-wire" but `LedgerEntrySigningPayload` actually has 9 fields | Updated to "9-field" | Codex Q-C2 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:39:| C3 | signing primitive integration via `CanonicalMessage` enum was unspecified; spec called nonexistent digest-form verifier | Path A: extend `CanonicalMessage::LedgerEntrySigning(LedgerEntrySigningPayload)`; sign separate signing payload (NOT raw `LedgerEntry`); new API `keypair.sign_ledger_entry(payload) → SystemSignature` | Codex Q-G + Gemini Q4 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:41:| K2 | signature did NOT bind `parent_ledger_root` → transplant attack | `LedgerEntrySigningPayload` includes `parent_ledger_root` field | Codex Q-B (NEW) |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:42:| K3 | L4/L5 head_t ownership inconsistent (spec line 194 vs 276 disagreed) | CO1.7 owns `ledger_root_t` + commit-chain `head_t = NodeId(commit_sha)` only; L5 (CO1.8) owns `state_root_t` mutation; sequencer drops `head_t = NodeId::from_state_root(...)` line | Codex Q-E |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:48:| D1 | epoch binding disagreement (Codex bind YES; Gemini bind NO) | Conservative resolution: epoch IS bound in `LedgerEntrySigningPayload`; epoch NOT separately folded into `ledger_root_t` (Codex security wins; Gemini orthogonality preserved at the ledger_root axis) | merged verdict § 5 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:58:- **LedgerEntrySigningPayload**: the 9-field bytes-on-the-wire that the system keypair actually signs (distinct from LedgerEntry-the-stored-record).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:63:- **Storage backend**: git2-rs commit chain (built on CO1.4 CAS); each LedgerEntry = one git commit on `refs/transitions/main`. **R2-K3**: head_t mutation deferred to CO1.7.5+ — v1.x ledger does NOT mutate `Q_t.head_t` directly. Once `Git2LedgerWriter::commit` exists and returns commit_sha alongside Hash, CO1.7.5 wiring will set `head_t = NodeId(commit_sha)` outside the L4 sequencer apply path.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:64:- **CanonicalMessage extension**: extends shipped `CanonicalMessage` enum with `LedgerEntrySigning([u8; 32])` opaque-digest variant (R2 design — avoids circular module dep); new typed sign API `transition_ledger_emitter::sign_ledger_entry(keypair, signing_payload_digest)`.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:70:- L5 materializer (state_root computation) — deferred to **CO1.8**. **K3 boundary (revised v1.2)**: CO1.7 owns `ledger_root_t` only; CO1.8 owns `state_root_t`; **head_t mutation is deferred to CO1.7.5+ wiring** (when `Git2LedgerWriter` exists). Sequencer does NOT mutate `state_root_t` or `head_t` directly; it accepts `q_next.state_root_t` as returned by the transition function and persists `ledger_root_t`.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:120:    /// where signing_digest_t = canonical_digest(LedgerEntrySigningPayload at logical_t).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:138:    /// Detached system signature over canonical_digest of LedgerEntrySigningPayload.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:157:### § 1.1 LedgerEntrySigningPayload (NEW per C3)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:166:pub struct LedgerEntrySigningPayload {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:179:impl LedgerEntrySigningPayload {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:205:### § 1.2 CanonicalMessage extension (per C3; **shipped in v1.2**)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:207:CO1.7 extends shipped `system_keypair::CanonicalMessage` with one new variant. **R2-C3 design choice**: variant carries the **opaque 32-byte canonical_digest** of `LedgerEntrySigningPayload`, NOT the full payload struct. This avoids a circular `system_keypair ↔ transition_ledger` module dependency (the payload struct needs `Cid` from CAS module + `SystemEpoch` from system_keypair; carrying the precomputed digest sidesteps the cycle entirely). The signature still binds the full payload because `canonical_digest()` is deterministic in `transition_ledger`.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:210:// In src/bottom_white/ledger/system_keypair.rs (additive Wave 4-B extension; SHIPPED v1.2):
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:215:    LedgerEntrySigning([u8; 32]),                     // NEW v1.2 (C3) — opaque digest
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:218:// canonical_digest() match arm (SHIPPED v1.2):
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:219://   CanonicalMessage::LedgerEntrySigning(digest) => {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:220://       h.update(b"LedgerEntrySigning");
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:224:// Authorized emitter module (SHIPPED v1.2):
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:225:pub(crate) mod transition_ledger_emitter {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:226:    pub(crate) fn sign_ledger_entry(
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:237:let sig = transition_ledger_emitter::sign_ledger_entry(&keypair, digest.0)?;
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:240:**Forward-compat clause**: if v4.x adds new ledger-side message variants, they MUST add new `CanonicalMessage::*` variants (NOT extend the LedgerEntrySigning variant in-place; opaque digest is committed to `[u8; 32]`). v4-shipped extensions go in the `LedgerEntry::extensions` BTreeMap (G1) which IS bound in `LedgerEntrySigningPayload::canonical_digest()`.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:250:└── transition_ledger.rs         (NEW; LedgerEntry, LedgerEntrySigningPayload, TxKind, append, replay_*, LedgerWriter)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:262:## § 3 Sequencer (K1 dual-counter; K3 head_t ownership; C3 sign API)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:320:            ObjectType::ProposalPayload,  // R2 fix: shipped CAS variant (NOT Transition)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:323:            Some("LedgerEntrySigningPayload.v1".to_string()),
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:330:        // 5. Build LedgerEntrySigningPayload
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:331:        let signing_payload = LedgerEntrySigningPayload {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:343:        // 6. **C3 NEW SIGN API (v1.2)**: typed sign through CanonicalMessage extension.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:346:        let system_signature = transition_ledger_emitter::sign_ledger_entry(
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:377:        // **K3 (v1.2 revised)**: do NOT mutate q_w.head_t here. v1.x ledger owns
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:378:        // `ledger_root_t` only. head_t mutation is **deferred to CO1.7.5+ wiring**
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:380:        // alongside Hash). Until then, head_t remains at QState placeholder; replay
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:381:        // and chain-integrity tests do NOT depend on head_t.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:390:**Why no head_t mutation in apply_one (K3, revised v1.2)**: v1.x CO1.7 owns `ledger_root_t` only. CO1.8 owns `state_root_t`. **head_t mutation deferred to CO1.7.5+** when `Git2LedgerWriter` provides a commit_sha return alongside Hash; the InMemoryLedgerWriter used by the v1 skeleton has no commit_sha to expose, so the trait keeps a single `Hash` return and head_t wiring is a separate downstream concern. Sequencer never calls `NodeId::from_state_root(...)`.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:461:- **K3 (v1.2)**: `head_t = NodeId(commit_sha)` is the canonical convention WHEN head_t is wired (CO1.7.5+). v1.x sequencer does NOT mutate head_t — `Git2LedgerWriter` is needed to surface commit_sha. `NodeId::from_state_root(...)` is NOT used by L4 in any version.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:511:| `tests/canonical_digest_stable` | skeleton (v1) | LedgerEntrySigningPayload digest stable across clones; #[repr(u8)] discriminant stable |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:517:| `tests/canonical_digest_excludes_derivatives` | **Q9 NEW** skeleton (v1) | LedgerEntrySigningPayload.canonical_digest excludes resulting_ledger_root + system_signature; mutation of either does NOT change digest |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:519:| `tests/system_signature_verifies_via_canonical_message` | CO1.7.5+ | LedgerEntry.system_signature verifies through `verify_system_signature(&CanonicalMessage::LedgerEntrySigning(...), epoch, pinned_pubkeys)` |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:586:| Q4 signature placement | open | **Inside `LedgerEntry`** (v1 design); BUT signing target is `LedgerEntrySigningPayload` (separate struct, distinct fields). Both auditors agreed. |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:590:| Q8 CanonicalMessage extension | open | **Path A**: extend enum with `LedgerEntrySigning(LedgerEntrySigningPayload)` variant; new typed sign API `sign_ledger_entry(payload, epoch)`. Forward-compat clause: future ledger-side message variants add new `CanonicalMessage::*` variants (NOT in-place edits). Both auditors agreed. |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:591:| Q9 canonical_digest exclusion | open | **Excludes**: `resulting_ledger_root` (cycle), `system_signature` (its input). **Includes**: 9 fields explicit in `LedgerEntrySigningPayload`. Spec § 1.1 explicit. |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:592:| Q10 epoch field | open | **Added** to `LedgerEntry` field 9; **bound** in `LedgerEntrySigningPayload` (D1 conservative resolution: Codex security argument). NOT separately folded into ledger_root. |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:593:| Q11 NEW open Qs | — | Codex round-1 listed: parent_ledger_root binding (now K2 / done), rejected-submission logical time (now K1 / done), CAS persistence (now C2 → CO1.4-extra), canonical fixtures (deferred to CO1.7.5+ test stubs), L4/L5 head_t ownership (now K3 / done). All addressed. |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:626:3. system_keypair extension (CanonicalMessage variant + sign_ledger_entry API) is additive Wave 4-B extension; non-breaking; verified via type-skeleton (sign API call site is concrete, not unimplemented).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:631:   - DIV-3 epoch field — **resolved**: in LedgerEntry + bound in LedgerEntrySigningPayload.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:634:6. **Q9 spec bug found by skeleton smoke** — closed by spec § 1.1: explicit `LedgerEntrySigningPayload` separate struct excludes derivatives.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:649:**v1.1 skeleton update plan** (separate commit): apply K6 (#[repr(u8)]), K5 (drop Slash), G1 (extensions field), C3 (sign through CanonicalMessage extension), K2 (parent_ledger_root field + new sign payload struct), Q9 (canonical_digest moves to LedgerEntrySigningPayload), K7 (add 2 new tests for parent_ledger_root tamper + digest exclusion). Target: 8 skeleton tests PASS, full workspace still 196+/0 PASS.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:659:| C3 signing primitive integration | LedgerEntrySigningPayload struct + CanonicalMessage extension + sign_ledger_entry API | § 1.1, § 1.2 |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:662:| K3 L4/L5 head_t ownership | Boundary clarified: CO1.7 owns ledger_root + commit-chain head_t (NodeId(commit_sha)); CO1.8 owns state_root | § 0, § 3, § 5 |
src/bottom_white/cas/schema.rs:138:            object_type: ObjectType::ProposalPayload,
src/bottom_white/cas/schema.rs:152:            object_type: ObjectType::ProposalPayload,

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '180,470p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   180	    /// TRACE_MATRIX FC3-Sig: new signing epoch certified by the rotation proof.
   181	    pub const fn new_epoch(&self) -> SystemEpoch {
   182	        self.new_epoch
   183	    }
   184	}
   185	
   186	/// TRACE_MATRIX FC1-Sig+FC3-Sig: only typed runtime messages may enter signature verification.
   187	#[derive(Debug, Clone, PartialEq, Eq)]
   188	pub enum CanonicalMessage {
   189	    /// TRACE_MATRIX FC1-Sig: predicate-runner rejection summary.
   190	    RejectedAttemptSummary(RejectedAttemptSummary),
   191	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: terminal summary transaction.
   192	    TerminalSummaryTx(TerminalSummaryTx),
   193	    /// TRACE_MATRIX FC3-Sig: system key epoch continuity proof.
   194	    EpochRotationProof(EpochRotationProof),
   195	    /// TRACE_MATRIX FC2-Append (CO1.7 v1.2 round-2 closure C3): L4 transition_ledger
   196	    /// signing payload digest. Opaque [u8; 32] — full canonical_digest of
   197	    /// `LedgerEntrySigningPayload` is computed in `transition_ledger`; this variant
   198	    /// only carries the 32-byte digest into the typed sign API. Avoids a circular
   199	    /// `system_keypair ↔ transition_ledger` module dependency while preserving the
   200	    /// "all sign goes through CanonicalMessage" invariant.
   201	    LedgerEntrySigning([u8; 32]),
   202	}
   203	
   204	/// TRACE_MATRIX FC1-Sig+FC3-Sig: epoch-indexed public keys pinned by genesis and rotation history.
   205	#[derive(Debug, Clone, Default, PartialEq, Eq)]
   206	pub struct PinnedSystemPubkeys {
   207	    keys: BTreeMap<SystemEpoch, SystemPublicKey>,
   208	}
   209	
   210	impl PinnedSystemPubkeys {
   211	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: create an empty pinned system-key map.
   212	    pub fn new() -> Self {
   213	        Self::default()
   214	    }
   215	
   216	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: pin a public key for a system epoch.
   217	    pub fn insert(
   218	        &mut self,
   219	        epoch: SystemEpoch,
   220	        public_key: SystemPublicKey,
   221	    ) -> Option<SystemPublicKey> {
   222	        self.keys.insert(epoch, public_key)
   223	    }
   224	
   225	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: fetch the public key pinned for a system epoch.
   226	    pub fn get(&self, epoch: SystemEpoch) -> Option<&SystemPublicKey> {
   227	        self.keys.get(&epoch)
   228	    }
   229	}
   230	
   231	/// TRACE_MATRIX FC1-Sig+FC3-Sig: in-memory ed25519 system keypair with zeroized private key on drop.
   232	#[derive(Zeroize, ZeroizeOnDrop)]
   233	pub struct Ed25519Keypair {
   234	    secret_key: Box<[u8]>,
   235	    #[zeroize(skip)]
   236	    public_key: SystemPublicKey,
   237	}
   238	
   239	impl Ed25519Keypair {
   240	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: generate ed25519 key material from `getrandom(2)` entropy.
   241	    pub fn generate_with_secure_entropy() -> Result<Self, KeypairError> {
   242	        let mut seed = [0u8; SECRET_KEY_LEN];
   243	        getrandom::getrandom(&mut seed).map_err(KeypairError::Entropy)?;
   244	        let signing_key = SigningKey::from_bytes(&seed);
   245	        let public_key = SystemPublicKey::from_bytes(signing_key.verifying_key().to_bytes());
   246	        let mut keypair = Self {
   247	            secret_key: Vec::from(seed).into_boxed_slice(),
   248	            public_key,
   249	        };
   250	        seed.zeroize();
   251	        keypair.mlock_private_key_best_effort();
   252	        Ok(keypair)
   253	    }
   254	
   255	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: return the public half of the system keypair.
   256	    pub const fn public_key(&self) -> SystemPublicKey {
   257	        self.public_key
   258	    }
   259	
   260	    fn from_plaintext(plaintext: &[u8]) -> Result<Self, KeypairError> {
   261	        if plaintext.len() != PLAINTEXT_LEN {
   262	            return Err(KeypairError::InvalidFormat("bad plaintext length"));
   263	        }
   264	        let mut secret = [0u8; SECRET_KEY_LEN];
   265	        secret.copy_from_slice(&plaintext[..SECRET_KEY_LEN]);
   266	        let mut public = [0u8; PUBLIC_KEY_LEN];
   267	        public.copy_from_slice(&plaintext[SECRET_KEY_LEN..]);
   268	
   269	        let signing_key = SigningKey::from_bytes(&secret);
   270	        if signing_key.verifying_key().to_bytes() != public {
   271	            secret.zeroize();
   272	            return Err(KeypairError::InvalidFormat(
   273	                "public key does not match private key",
   274	            ));
   275	        }
   276	
   277	        let mut keypair = Self {
   278	            secret_key: Vec::from(secret).into_boxed_slice(),
   279	            public_key: SystemPublicKey::from_bytes(public),
   280	        };
   281	        secret.zeroize();
   282	        keypair.mlock_private_key_best_effort();
   283	        Ok(keypair)
   284	    }
   285	
   286	    fn to_plaintext(&self) -> Result<[u8; PLAINTEXT_LEN], KeypairError> {
   287	        let secret = self.secret_slice()?;
   288	        let mut plaintext = [0u8; PLAINTEXT_LEN];
   289	        plaintext[..SECRET_KEY_LEN].copy_from_slice(secret);
   290	        plaintext[SECRET_KEY_LEN..].copy_from_slice(self.public_key.as_bytes());
   291	        Ok(plaintext)
   292	    }
   293	
   294	    fn sign_digest(&self, digest: [u8; 32]) -> Result<SystemSignature, KeypairError> {
   295	        let mut secret = [0u8; SECRET_KEY_LEN];
   296	        secret.copy_from_slice(self.secret_slice()?);
   297	        let signing_key = SigningKey::from_bytes(&secret);
   298	        let signature = signing_key.sign(&digest);
   299	        secret.zeroize();
   300	        Ok(SystemSignature::from_bytes(signature.to_bytes()))
   301	    }
   302	
   303	    fn secret_slice(&self) -> Result<&[u8], KeypairError> {
   304	        if self.secret_key.len() == SECRET_KEY_LEN {
   305	            Ok(&self.secret_key)
   306	        } else {
   307	            Err(KeypairError::InvalidFormat("bad in-memory secret length"))
   308	        }
   309	    }
   310	
   311	    fn mlock_private_key_best_effort(&mut self) -> bool {
   312	        mlock_best_effort(self.secret_key.as_ptr(), self.secret_key.len())
   313	    }
   314	}
   315	
   316	/// TRACE_MATRIX FC1-Sig+FC3-Sig: system keypair lifecycle and crypto error taxonomy.
   317	#[derive(Debug)]
   318	pub enum KeypairError {
   319	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: filesystem operation failed.
   320	    Io(std::io::Error),
   321	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: secure operating-system entropy failed.
   322	    Entropy(getrandom::Error),
   323	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: KDF environment parameter was absent or invalid.
   324	    KdfParam(String),
   325	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: Argon2id key derivation failed.
   326	    Kdf(argon2::Error),
   327	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: ChaCha20-Poly1305 encryption or authentication failed.
   328	    Crypto(&'static str),
   329	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: encrypted keystore format was malformed.
   330	    InvalidFormat(&'static str),
   331	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: default keystore path could not be resolved.
   332	    HomeUnavailable,
   333	}
   334	
   335	impl fmt::Display for KeypairError {
   336	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   337	        match self {
   338	            Self::Io(err) => write!(f, "system keypair I/O failed: {err}"),
   339	            Self::Entropy(err) => write!(f, "system keypair entropy failed: {err}"),
   340	            Self::KdfParam(msg) => write!(f, "system keypair KDF parameter invalid: {msg}"),
   341	            Self::Kdf(err) => write!(f, "system keypair KDF failed: {err}"),
   342	            Self::Crypto(msg) => write!(f, "system keypair crypto failed: {msg}"),
   343	            Self::InvalidFormat(msg) => write!(f, "system keypair keystore invalid: {msg}"),
   344	            Self::HomeUnavailable => {
   345	                write!(f, "system keypair default keystore path requires HOME")
   346	            }
   347	        }
   348	    }
   349	}
   350	
   351	impl std::error::Error for KeypairError {}
   352	
   353	impl From<std::io::Error> for KeypairError {
   354	    fn from(value: std::io::Error) -> Self {
   355	        Self::Io(value)
   356	    }
   357	}
   358	
   359	/// TRACE_MATRIX FC1-Sig+FC3-Sig: resolve `~/.turingos/keystore/system_keypair_v{epoch}.enc`.
   360	///
   361	/// `TURINGOS_KEYSTORE_PATH` overrides the default path. The default never
   362	/// points into the repository, CAS, or ledger directories.
   363	pub fn default_system_keystore_path(epoch: SystemEpoch) -> Result<PathBuf, KeypairError> {
   364	    if let Ok(path) = env::var("TURINGOS_KEYSTORE_PATH") {
   365	        return Ok(PathBuf::from(path));
   366	    }
   367	    let home = env::var("HOME").map_err(|_| KeypairError::HomeUnavailable)?;
   368	    Ok(PathBuf::from(home)
   369	        .join(".turingos")
   370	        .join("keystore")
   371	        .join(format!("system_keypair_v{}.enc", epoch.get())))
   372	}
   373	
   374	/// TRACE_MATRIX FC1-Sig+FC3-Sig: first-boot generate-or-second-boot decrypt lifecycle entrypoint.
   375	pub fn generate_or_load_system_keypair(
   376	    keystore_path: &Path,
   377	    user_kdf_password: &SecretString,
   378	) -> Result<Ed25519Keypair, KeypairError> {
   379	    if keystore_path.exists() {
   380	        return load_existing_keypair(keystore_path, user_kdf_password);
   381	    }
   382	
   383	    let keypair = Ed25519Keypair::generate_with_secure_entropy()?;
   384	    let encrypted = encrypt_at_rest(&keypair, user_kdf_password)?;
   385	    write_keystore_0600(keystore_path, &encrypted)?;
   386	    Ok(keypair)
   387	}
   388	
   389	/// TRACE_MATRIX FC1-Sig+FC3-Sig: decrypt an existing encrypted system keypair keystore.
   390	pub fn load_existing_keypair(
   391	    keystore_path: &Path,
   392	    user_kdf_password: &SecretString,
   393	) -> Result<Ed25519Keypair, KeypairError> {
   394	    let bytes = fs::read(keystore_path)?;
   395	    let encoded = EncryptedKeypair::decode(&bytes)?;
   396	    let mut key = derive_key(user_kdf_password, &encoded.salt, encoded.kdf)?;
   397	    let cipher = ChaCha20Poly1305::new_from_slice(&key)
   398	        .map_err(|_| KeypairError::Crypto("bad cipher key"))?;
   399	    let plaintext = cipher
   400	        .decrypt(
   401	            Nonce::from_slice(&encoded.nonce),
   402	            encoded.ciphertext.as_ref(),
   403	        )
   404	        .map_err(|_| KeypairError::Crypto("keystore authentication failed"))?;
   405	    key.zeroize();
   406	    Ed25519Keypair::from_plaintext(&plaintext)
   407	}
   408	
   409	/// TRACE_MATRIX FC1-Sig+FC3-Sig: canonical SHA-256 digest for typed system messages.
   410	pub fn canonical_digest(message: &CanonicalMessage) -> [u8; 32] {
   411	    let mut h = Sha256::new();
   412	    h.update(b"turingosv4.system_keypair.v1");
   413	    match message {
   414	        CanonicalMessage::RejectedAttemptSummary(summary) => {
   415	            h.update(b"RejectedAttemptSummary");
   416	            update_len_prefixed(&mut h, summary.run_id.as_bytes());
   417	            update_len_prefixed(&mut h, summary.attempt_id.as_bytes());
   418	            update_len_prefixed(&mut h, summary.failure_class.as_bytes());
   419	            h.update(summary.summary_hash);
   420	        }
   421	        CanonicalMessage::TerminalSummaryTx(tx) => {
   422	            h.update(b"TerminalSummaryTx");
   423	            update_len_prefixed(&mut h, tx.run_id.as_bytes());
   424	            h.update(tx.terminal_state_root);
   425	            h.update(tx.rejected_attempt_count.to_be_bytes());
   426	        }
   427	        CanonicalMessage::EpochRotationProof(proof) => {
   428	            h.update(b"EpochRotationProof");
   429	            h.update(proof.old_epoch.get().to_be_bytes());
   430	            h.update(proof.new_epoch.get().to_be_bytes());
   431	            h.update(proof.old_pubkey.as_bytes());
   432	            h.update(proof.new_pubkey.as_bytes());
   433	            h.update(proof.signed_at_unix.to_be_bytes());
   434	        }
   435	        CanonicalMessage::LedgerEntrySigning(digest) => {
   436	            h.update(b"LedgerEntrySigning");
   437	            h.update(digest);
   438	        }
   439	    }
   440	    h.finalize().into()
   441	}
   442	
   443	/// TRACE_MATRIX FC1-Sig+FC3-Sig: public system signature verification against pinned epoch keys.
   444	pub fn verify_system_signature(
   445	    sig: &SystemSignature,
   446	    message: &CanonicalMessage,
   447	    epoch: SystemEpoch,
   448	    pinned_pubkeys: &PinnedSystemPubkeys,
   449	) -> bool {
   450	    let Some(pk) = pinned_pubkeys.get(epoch) else {
   451	        return false;
   452	    };
   453	    let Ok(verifying_key) = VerifyingKey::from_bytes(pk.as_bytes()) else {
   454	        return false;
   455	    };
   456	    let signature = Signature::from_bytes(sig.as_bytes());
   457	    verifying_key
   458	        .verify(&canonical_digest(message), &signature)
   459	        .is_ok()
   460	}
   461	
   462	/// TRACE_MATRIX FC3-Sig: verify old and new signatures over a rotation continuity proof.
   463	pub fn verify_epoch_rotation_proof(
   464	    proof: &EpochRotationProof,
   465	    old_signature: &SystemSignature,
   466	    new_signature: &SystemSignature,
   467	    pinned_pubkeys: &PinnedSystemPubkeys,
   468	) -> bool {
   469	    let message = CanonicalMessage::EpochRotationProof(proof.clone());
   470	    verify_system_signature(old_signature, &message, proof.old_epoch(), pinned_pubkeys)

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '90,570p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    90	}
    91	
    92	// ────────────────────────────────────────────────────────────────────────────
    93	// § 1.1 LedgerEntrySigningPayload — the signed bytes (NEW per C3 / Q9)
    94	// ────────────────────────────────────────────────────────────────────────────
    95	
    96	/// TRACE_MATRIX FC2-Append C3: the bytes the system keypair actually signs.
    97	///
    98	/// **Excludes** (Q9 cycle prevention):
    99	/// - `resulting_ledger_root` (derivative; including → cycle)
   100	/// - `system_signature` (its own input)
   101	///
   102	/// **Includes** (9 non-derivative bound fields). Domain-separation prefix is
   103	/// part of the digest to prevent cross-namespace collision.
   104	#[derive(Debug, Clone, PartialEq, Eq)]
   105	pub struct LedgerEntrySigningPayload {
   106	    pub logical_t: u64,
   107	    pub parent_state_root: Hash,
   108	    pub parent_ledger_root: Hash,                  // K2
   109	    pub tx_kind: TxKind,
   110	    pub tx_payload_cid: Cid,
   111	    pub resulting_state_root: Hash,
   112	    pub timestamp_logical: u64,
   113	    pub epoch: SystemEpoch,                        // D1
   114	    pub extensions: BTreeMap<String, Vec<u8>>,     // G1
   115	}
   116	
   117	impl LedgerEntrySigningPayload {
   118	    /// Canonical SHA-256 digest. Stable wire format (NOT bincode/serde dependent).
   119	    pub fn canonical_digest(&self) -> Hash {
   120	        let mut h = Sha256::new();
   121	        h.update(b"turingosv4.ledger_entry_signing.v1");
   122	        h.update(self.logical_t.to_be_bytes());
   123	        h.update(self.parent_state_root.0);
   124	        h.update(self.parent_ledger_root.0);
   125	        h.update((self.tx_kind as u8).to_be_bytes()); // K6 #[repr(u8)] makes cast stable
   126	        h.update(self.tx_payload_cid.0);
   127	        h.update(self.resulting_state_root.0);
   128	        h.update(self.timestamp_logical.to_be_bytes());
   129	        h.update(self.epoch.get().to_be_bytes());
   130	        // Extensions: BTreeMap iterates in lex key order (deterministic);
   131	        // length-prefix every field to prevent ambiguity attacks.
   132	        h.update((self.extensions.len() as u64).to_be_bytes());
   133	        for (k, v) in &self.extensions {
   134	            h.update((k.len() as u64).to_be_bytes());
   135	            h.update(k.as_bytes());
   136	            h.update((v.len() as u64).to_be_bytes());
   137	            h.update(v);
   138	        }
   139	        Hash(h.finalize().into())
   140	    }
   141	}
   142	
   143	impl LedgerEntry {
   144	    /// Project the LedgerEntry's signed-fields-subset back into a signing payload.
   145	    /// Used by replay to recompute `signing_digest` and re-verify chain integrity.
   146	    pub fn to_signing_payload(&self) -> LedgerEntrySigningPayload {
   147	        LedgerEntrySigningPayload {
   148	            logical_t: self.logical_t,
   149	            parent_state_root: self.parent_state_root,
   150	            parent_ledger_root: self.parent_ledger_root,
   151	            tx_kind: self.tx_kind,
   152	            tx_payload_cid: self.tx_payload_cid,
   153	            resulting_state_root: self.resulting_state_root,
   154	            timestamp_logical: self.timestamp_logical,
   155	            epoch: self.epoch,
   156	            extensions: self.extensions.clone(),
   157	        }
   158	    }
   159	}
   160	
   161	// ────────────────────────────────────────────────────────────────────────────
   162	// § 4 append() — pure ledger-root fold
   163	// ────────────────────────────────────────────────────────────────────────────
   164	
   165	/// TRACE_MATRIX FC2-Append + spec § 4: pure ledger-root fold over signed digests.
   166	/// Same `(parent_root, signing_digest)` → byte-identical `new_root`.
   167	/// No I/O, no clock, no env. Witness for I-DET ledger axis.
   168	pub fn append(parent_root: &Hash, signing_digest: &Hash) -> Hash {
   169	    let mut h = Sha256::new();
   170	    h.update(b"turingosv4.ledger_root.v1");
   171	    h.update(parent_root.0);
   172	    h.update(signing_digest.0);
   173	    Hash(h.finalize().into())
   174	}
   175	
   176	// ────────────────────────────────────────────────────────────────────────────
   177	// LedgerWriter trait (K4 reconciled to skeleton signature)
   178	// ────────────────────────────────────────────────────────────────────────────
   179	
   180	/// TRACE_MATRIX FC2-Append: storage abstraction for L4.
   181	/// Production impl is `Git2LedgerWriter` (CO1.7.5+; refs/transitions/main commit chain).
   182	/// Test/skeleton impl is `InMemoryLedgerWriter` below.
   183	///
   184	/// **K4**: signature `commit(&mut self) → Hash` (NOT `&self → NodeId`); `iter_from`
   185	/// deferred to CO1.7.5+ (only used by FullTransition replay; not v1 deliverable).
   186	pub trait LedgerWriter: Send + Sync {
   187	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
   188	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
   189	    fn len(&self) -> u64;
   190	}
   191	
   192	#[derive(Debug)]
   193	pub enum LedgerWriterError {
   194	    LogicalTGap { expected: u64, got: u64 },
   195	    NotFound { logical_t: u64 },
   196	    BackendCorruption(String),
   197	}
   198	
   199	impl std::fmt::Display for LedgerWriterError {
   200	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   201	        match self {
   202	            Self::LogicalTGap { expected, got } => {
   203	                write!(f, "logical_t gap: expected {expected}, got {got}")
   204	            }
   205	            Self::NotFound { logical_t } => write!(f, "no entry at logical_t={logical_t}"),
   206	            Self::BackendCorruption(msg) => write!(f, "backend corruption: {msg}"),
   207	        }
   208	    }
   209	}
   210	impl std::error::Error for LedgerWriterError {}
   211	
   212	/// In-memory test/skeleton writer; Vec backing strict logical_t enforced at commit.
   213	#[derive(Debug, Default)]
   214	pub struct InMemoryLedgerWriter {
   215	    entries: Vec<LedgerEntry>,
   216	}
   217	
   218	impl InMemoryLedgerWriter {
   219	    pub fn new() -> Self {
   220	        Self::default()
   221	    }
   222	}
   223	
   224	impl LedgerWriter for InMemoryLedgerWriter {
   225	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
   226	        let expected = (self.entries.len() as u64) + 1;
   227	        if entry.logical_t != expected {
   228	            return Err(LedgerWriterError::LogicalTGap {
   229	                expected,
   230	                got: entry.logical_t,
   231	            });
   232	        }
   233	        let root = entry.resulting_ledger_root;
   234	        self.entries.push(entry.clone());
   235	        Ok(root)
   236	    }
   237	
   238	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
   239	        if logical_t == 0 || logical_t > self.entries.len() as u64 {
   240	            return Err(LedgerWriterError::NotFound { logical_t });
   241	        }
   242	        Ok(self.entries[(logical_t - 1) as usize].clone())
   243	    }
   244	
   245	    fn len(&self) -> u64 {
   246	        self.entries.len() as u64
   247	    }
   248	}
   249	
   250	// ────────────────────────────────────────────────────────────────────────────
   251	// § 4 replay — TWO-MODE per C1
   252	// ────────────────────────────────────────────────────────────────────────────
   253	
   254	/// **C1 NEW**: replay mode discriminator.
   255	/// - `ChainOnly`: skeleton-stage; chain integrity only (parent_state_root +
   256	///   parent_ledger_root + ledger_root chain). NOT the I-DETHASH witness.
   257	/// - `FullTransition`: CO1.7.5+ stage; verifies signatures + re-fetches payloads
   258	///   from CAS + re-runs pure transitions + asserts state_root match. THE
   259	///   I-DETHASH witness; requires CO1.4-extra (CAS index persistence).
   260	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
   261	pub enum ReplayMode {
   262	    ChainOnly,
   263	    FullTransition,
   264	}
   265	
   266	#[derive(Debug)]
   267	pub enum ReplayError {
   268	    LogicalTGap { at: usize, expected: u64, got: u64 },
   269	    ParentStateMismatch { at: usize },
   270	    ParentLedgerMismatch { at: usize }, // K2 NEW
   271	    LedgerRootMismatch { at: usize },
   272	    // FullTransition-mode-only (CO1.7.5+):
   273	    BadSignature { at: usize },
   274	    CasMissing { at: usize },
   275	    StateRootMismatch { at: usize },
   276	}
   277	
   278	impl std::fmt::Display for ReplayError {
   279	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   280	        match self {
   281	            Self::LogicalTGap { at, expected, got } => {
   282	                write!(f, "logical_t gap at index {at}: expected {expected}, got {got}")
   283	            }
   284	            Self::ParentStateMismatch { at } => write!(f, "parent_state_root mismatch at index {at}"),
   285	            Self::ParentLedgerMismatch { at } => write!(f, "parent_ledger_root mismatch at index {at}"),
   286	            Self::LedgerRootMismatch { at } => write!(f, "ledger_root mismatch at index {at}"),
   287	            Self::BadSignature { at } => write!(f, "system_signature verify failed at index {at}"),
   288	            Self::CasMissing { at } => write!(f, "CAS payload not retrievable at index {at}"),
   289	            Self::StateRootMismatch { at } => write!(f, "resulting_state_root divergence at index {at}"),
   290	        }
   291	    }
   292	}
   293	impl std::error::Error for ReplayError {}
   294	
   295	/// Skeleton-stage entry point (v1.1).
   296	///
   297	/// Validates:
   298	/// 1. logical_t monotonicity (no gaps, no duplicates)
   299	/// 2. parent_state_root chain
   300	/// 3. parent_ledger_root chain (K2 transplant defense)
   301	/// 4. resulting_ledger_root recomputed via append(prev_ledger_root, signing_digest)
   302	///
   303	/// Does NOT verify:
   304	/// - system_signature (CO1.7.5+: requires CanonicalMessage extension wired through keypair)
   305	/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
   306	///
   307	/// Returns final (state_root, ledger_root) on success.
   308	pub fn replay_chain_integrity(
   309	    genesis_state_root: Hash,
   310	    genesis_ledger_root: Hash,
   311	    entries: &[LedgerEntry],
   312	) -> Result<(Hash, Hash), ReplayError> {
   313	    let mut prev_state_root = genesis_state_root;
   314	    let mut prev_ledger_root = genesis_ledger_root;
   315	
   316	    for (i, entry) in entries.iter().enumerate() {
   317	        let expected_logical_t = (i as u64) + 1;
   318	        if entry.logical_t != expected_logical_t {
   319	            return Err(ReplayError::LogicalTGap {
   320	                at: i,
   321	                expected: expected_logical_t,
   322	                got: entry.logical_t,
   323	            });
   324	        }
   325	        if entry.parent_state_root != prev_state_root {
   326	            return Err(ReplayError::ParentStateMismatch { at: i });
   327	        }
   328	        // K2 NEW: parent_ledger_root chain check
   329	        if entry.parent_ledger_root != prev_ledger_root {
   330	            return Err(ReplayError::ParentLedgerMismatch { at: i });
   331	        }
   332	        let signing_digest = entry.to_signing_payload().canonical_digest();
   333	        let recomputed = append(&prev_ledger_root, &signing_digest);
   334	        if recomputed != entry.resulting_ledger_root {
   335	            return Err(ReplayError::LedgerRootMismatch { at: i });
   336	        }
   337	        prev_state_root = entry.resulting_state_root;
   338	        prev_ledger_root = entry.resulting_ledger_root;
   339	    }
   340	
   341	    Ok((prev_state_root, prev_ledger_root))
   342	}
   343	
   344	// ────────────────────────────────────────────────────────────────────────────
   345	// Tests — 8 conformance items (4 NEW vs v1 skeleton: K2 / Q9 / repr(u8) / extensions)
   346	// ────────────────────────────────────────────────────────────────────────────
   347	
   348	#[cfg(test)]
   349	mod tests {
   350	    use super::*;
   351	
   352	    fn h(byte: u8) -> Hash {
   353	        Hash([byte; 32])
   354	    }
   355	
   356	    /// Build an entry that satisfies all chain invariants given the previous state.
   357	    fn entry_at(
   358	        logical_t: u64,
   359	        parent_state_root: Hash,
   360	        parent_ledger_root: Hash,
   361	        resulting_state_root: Hash,
   362	    ) -> LedgerEntry {
   363	        let signing = LedgerEntrySigningPayload {
   364	            logical_t,
   365	            parent_state_root,
   366	            parent_ledger_root,
   367	            tx_kind: TxKind::Work,
   368	            tx_payload_cid: Cid([0u8; 32]),
   369	            resulting_state_root,
   370	            timestamp_logical: logical_t,
   371	            epoch: SystemEpoch::new(1),
   372	            extensions: BTreeMap::new(),
   373	        };
   374	        let signing_digest = signing.canonical_digest();
   375	        let resulting_ledger_root = append(&parent_ledger_root, &signing_digest);
   376	        LedgerEntry {
   377	            logical_t: signing.logical_t,
   378	            parent_state_root: signing.parent_state_root,
   379	            parent_ledger_root: signing.parent_ledger_root,
   380	            tx_kind: signing.tx_kind,
   381	            tx_payload_cid: signing.tx_payload_cid,
   382	            resulting_state_root: signing.resulting_state_root,
   383	            resulting_ledger_root,
   384	            timestamp_logical: signing.timestamp_logical,
   385	            epoch: signing.epoch,
   386	            extensions: signing.extensions,
   387	            system_signature: SystemSignature::from_bytes([0u8; 64]),
   388	        }
   389	    }
   390	
   391	    // 1. append byte-stable (I-DET ledger axis)
   392	    #[test]
   393	    fn append_is_pure_and_byte_stable() {
   394	        let a = append(&Hash::ZERO, &h(1));
   395	        let b = append(&Hash::ZERO, &h(1));
   396	        assert_eq!(a, b);
   397	        let c = append(&Hash::ZERO, &h(2));
   398	        assert_ne!(a, c);
   399	    }
   400	
   401	    // 2. canonical_digest stable (#[repr(u8)] discriminant stable)
   402	    #[test]
   403	    fn canonical_digest_stable_across_clones() {
   404	        let p = LedgerEntrySigningPayload {
   405	            logical_t: 1,
   406	            parent_state_root: Hash::ZERO,
   407	            parent_ledger_root: Hash::ZERO,
   408	            tx_kind: TxKind::Work,
   409	            tx_payload_cid: Cid([7u8; 32]),
   410	            resulting_state_root: h(0xaa),
   411	            timestamp_logical: 1,
   412	            epoch: SystemEpoch::new(2),
   413	            extensions: BTreeMap::new(),
   414	        };
   415	        let d1 = p.canonical_digest();
   416	        let d2 = p.clone().canonical_digest();
   417	        assert_eq!(d1, d2);
   418	    }
   419	
   420	    // 3. InMemoryWriter enforces logical_t monotonic
   421	    #[test]
   422	    fn in_memory_writer_enforces_logical_t() {
   423	        let mut w = InMemoryLedgerWriter::new();
   424	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   425	        assert!(w.commit(&e1).is_ok());
   426	
   427	        let e_skip = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   428	        let err = w.commit(&e_skip).unwrap_err();
   429	        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
   430	    }
   431	
   432	    // 4. ChainOnly replay validates clean chain
   433	    #[test]
   434	    fn replay_chain_integrity_clean() {
   435	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   436	        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   437	        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
   438	        let (final_state, final_ledger) =
   439	            replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1.clone(), e2.clone(), e3.clone()])
   440	                .expect("clean chain replays");
   441	        assert_eq!(final_state, e3.resulting_state_root);
   442	        assert_eq!(final_ledger, e3.resulting_ledger_root);
   443	    }
   444	
   445	    // 5. ChainOnly replay rejects parent_state_root tamper
   446	    #[test]
   447	    fn replay_rejects_parent_state_tamper() {
   448	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   449	        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   450	        e2.parent_state_root = h(0xff);
   451	        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
   452	        assert!(matches!(err, ReplayError::ParentStateMismatch { at: 1 }));
   453	    }
   454	
   455	    // 6. K2 NEW: ChainOnly replay rejects parent_ledger_root tamper (transplant defense)
   456	    #[test]
   457	    fn replay_rejects_parent_ledger_tamper() {
   458	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   459	        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   460	        // Tamper with parent_ledger_root WITHOUT recomputing resulting_ledger_root —
   461	        // simulates an attacker transplanting an entry from a different ledger history.
   462	        e2.parent_ledger_root = h(0xff);
   463	        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
   464	        assert!(matches!(err, ReplayError::ParentLedgerMismatch { at: 1 }));
   465	    }
   466	
   467	    // 7. ChainOnly replay rejects ledger_root tamper
   468	    #[test]
   469	    fn replay_rejects_ledger_root_tamper() {
   470	        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   471	        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
   472	        e2.resulting_ledger_root = h(0xee);
   473	        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
   474	        assert!(matches!(err, ReplayError::LedgerRootMismatch { at: 1 }));
   475	    }
   476	
   477	    // 8. Q9 NEW: canonical_digest excludes derivatives
   478	    // Mutating `resulting_ledger_root` or `system_signature` of LedgerEntry must NOT
   479	    // change the signing payload digest (because they're not in LedgerEntrySigningPayload).
   480	    #[test]
   481	    fn canonical_digest_excludes_derivatives() {
   482	        let e_clean = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
   483	        let digest_clean = e_clean.to_signing_payload().canonical_digest();
   484	
   485	        // Mutate resulting_ledger_root (a derivative; should NOT affect signing digest)
   486	        let mut e_tamper = e_clean.clone();
   487	        e_tamper.resulting_ledger_root = h(0xff);
   488	        let digest_after_root_tamper = e_tamper.to_signing_payload().canonical_digest();
   489	        assert_eq!(
   490	            digest_clean, digest_after_root_tamper,
   491	            "resulting_ledger_root MUST NOT affect signing digest (Q9 cycle prevention)"
   492	        );
   493	
   494	        // Mutate system_signature (signature is its own input; should NOT affect signing digest)
   495	        let mut e_tamper2 = e_clean.clone();
   496	        e_tamper2.system_signature = SystemSignature::from_bytes([0xffu8; 64]);
   497	        let digest_after_sig_tamper = e_tamper2.to_signing_payload().canonical_digest();
   498	        assert_eq!(digest_clean, digest_after_sig_tamper);
   499	
   500	        // Sanity: mutating a SIGNED field DOES change digest
   501	        let mut e_signed_change = e_clean.clone();
   502	        e_signed_change.epoch = SystemEpoch::new(99);
   503	        let digest_after_signed = e_signed_change.to_signing_payload().canonical_digest();
   504	        assert_ne!(digest_clean, digest_after_signed);
   505	    }
   506	
   507	    // 9. C3 closure (round-2): real signature roundtrip via system_keypair extension.
   508	    // Verifies: (a) typed sign API works; (b) signature verifies via CanonicalMessage::LedgerEntrySigning;
   509	    // (c) signature does NOT verify after mutating a signed field (parent_ledger_root — K2 transplant defense).
   510	    #[test]
   511	    fn signature_round_trip_and_transplant_defense() {
   512	        use crate::bottom_white::ledger::system_keypair::{
   513	            transition_ledger_emitter, CanonicalMessage, Ed25519Keypair, PinnedSystemPubkeys,
   514	            SystemEpoch, verify_system_signature,
   515	        };
   516	
   517	        let keypair = Ed25519Keypair::generate_with_secure_entropy().expect("keypair gen");
   518	        let epoch = SystemEpoch::new(1);
   519	        let mut pinned = PinnedSystemPubkeys::new();
   520	        pinned.insert(epoch, keypair.public_key());
   521	
   522	        // Build a clean signing payload (e1's worth)
   523	        let payload = LedgerEntrySigningPayload {
   524	            logical_t: 1,
   525	            parent_state_root: Hash::ZERO,
   526	            parent_ledger_root: Hash::ZERO,
   527	            tx_kind: TxKind::Work,
   528	            tx_payload_cid: Cid([42u8; 32]),
   529	            resulting_state_root: h(1),
   530	            timestamp_logical: 1,
   531	            epoch,
   532	            extensions: BTreeMap::new(),
   533	        };
   534	        let digest = payload.canonical_digest();
   535	
   536	        // Real sign through the typed CanonicalMessage extension
   537	        let sig = transition_ledger_emitter::sign_ledger_entry(&keypair, digest.0)
   538	            .expect("sign_ledger_entry");
   539	
   540	        // Verify (clean) — must succeed
   541	        let msg_clean = CanonicalMessage::LedgerEntrySigning(digest.0);
   542	        assert!(
   543	            verify_system_signature(&sig, &msg_clean, epoch, &pinned),
   544	            "clean signature must verify"
   545	        );
   546	
   547	        // Verify (tamper parent_ledger_root) — K2 transplant defense
   548	        let mut payload_tamper = payload.clone();
   549	        payload_tamper.parent_ledger_root = h(0xff);
   550	        let digest_tamper = payload_tamper.canonical_digest();
   551	        let msg_tamper = CanonicalMessage::LedgerEntrySigning(digest_tamper.0);
   552	        assert!(
   553	            !verify_system_signature(&sig, &msg_tamper, epoch, &pinned),
   554	            "transplanted parent_ledger_root MUST fail signature verify (K2)"
   555	        );
   556	
   557	        // Verify (cross-epoch transplant) — D1 defense via epoch IN payload digest.
   558	        // Attacker scenario: sig was made for payload with epoch=1; attacker forges a
   559	        // NEW payload claiming epoch=2 reusing the old sig. Since epoch is in the
   560	        // canonical digest, digest_v2 ≠ digest_v1, so the sig on digest_v1 cannot
   561	        // verify against digest_v2.
   562	        let mut payload_other_epoch = payload.clone();
   563	        payload_other_epoch.epoch = SystemEpoch::new(2);
   564	        let digest_other_epoch = payload_other_epoch.canonical_digest();
   565	        assert_ne!(digest, digest_other_epoch, "epoch is bound in canonical digest");
   566	        let msg_other_epoch = CanonicalMessage::LedgerEntrySigning(digest_other_epoch.0);
   567	        assert!(
   568	            !verify_system_signature(&sig, &msg_other_epoch, epoch, &pinned),
   569	            "cross-epoch transplant MUST fail signature verify (D1 epoch binding)"
   570	        );

exec
/bin/bash -lc "nl -ba handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md | sed -n '1,470p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# CO1.7 Transition Ledger v1.2 — Round-2 closure
     2	
     3	**Status**: v1.2 — round-2 returned PASS (Gemini, high) + CHALLENGE (Codex, high; 3 narrow patch blockers). Conservative merged CHALLENGE. v1.2 closes the 3 v1.1→v1.2 patches: (a) C3 actually wired in code (`CanonicalMessage::LedgerEntrySigning([u8;32])` + `transition_ledger_emitter::sign_ledger_entry`); (b) K3 head_t mutation explicitly deferred to CO1.7.5+ (no longer claimed in v1.x); (c) `ObjectType::Transition` replaced with shipped `ObjectType::ProposalPayload`. Plus typo fix and 1 new test. Awaiting round-3.
     4	
     5	**Status (v1.1)**: v1.1 — round-1 dual external audit (Codex + Gemini) returned CHALLENGE/CHALLENGE; this version closes 11 must-fix items, awaiting round-2.
     6	**Author**: ArchitectAI (Claude); session 2026-04-28.
     7	**Supersedes**: v1 (2026-04-28 morning DRAFT outline).
     8	**Round-1 verdicts**: `handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md` + `handover/audits/GEMINI_CO1_7_ROUND1_AUDIT_2026-04-28.md`; merged in `handover/audits/CO1_7_DUAL_AUDIT_VERDICT_R1_2026-04-28.md`.
     9	
    10	**Companion specs** (frozen, read first):
    11	- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — typed schemas + step_transition pseudocode + 27 invariants (round-4 PASS/PASS)
    12	- `SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md` — runtime keypair lifecycle (CO1.7.0a-f, done @ Wave 4-B)
    13	- `META_TRANSITION_INTERFACE_v1_2026-04-27.md` — trait pattern for L4 acceptance (deferred runtime to v4.1)
    14	- `TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md` § 5.L4 — ChainTape Layer 4 axioms
    15	
    16	**Single sentence**: implement the L4 transition_ledger module so that `ledger::append(parent_root, signing_digest) → new_root` (called from sequencer) is real code, the L4 sequencer (§ 5.2.1) is real code, and `Q_t.ledger_root_t` is no longer a placeholder.
    17	
    18	---
    19	
    20	## v1.2 patch log (vs. v1.1) — round-2 closure
    21	
    22	| ID | v1.1 issue | v1.2 fix | Source |
    23	|---|---|---|---|
    24	| **R2-C3** | Spec claimed "C3 CLOSED" but `system_keypair.rs` had no LedgerEntry path; skeleton itself said "deferred to CO1.7.5+" | Wave 4-B additive extension shipped: `CanonicalMessage::LedgerEntrySigning([u8;32])` (opaque digest variant; avoids transition_ledger ↔ system_keypair circular dep) + `canonical_digest` match arm + new `pub(crate) mod transition_ledger_emitter` with `sign_ledger_entry(keypair, digest)`. Skeleton test 9 (`signature_round_trip_and_transplant_defense`) now exercises the real roundtrip + K2 + D1 defenses. | Codex round-2 must-fix #1 |
    25	| **R2-K3** | Spec § 3 / § 5 said "CO1.7 owns head_t = NodeId(commit_sha)" but `LedgerWriter::commit` returns `Hash` not commit SHA; v1.1 InMemoryLedgerWriter has no commit_sha to return at all → contradiction | head_t mutation explicitly **deferred to CO1.7.5+** (when Git2LedgerWriter exists and can return both Hash + commit SHA). v1.x ledger owns `ledger_root_t` only; `head_t` continues to be set elsewhere (currently QState placeholder; CO1.7.5 wiring concern). Spec § 0 / § 3 / § 5 updated. | Codex round-2 must-fix #2 |
    26	| **R2-C2-CAS** | Spec § 3 sequencer pseudocode used `ObjectType::Transition` but shipped `ObjectType` has no such variant | Spec § 3 changed to `ObjectType::ProposalPayload` (the existing variant for agent work_tx payloads — semantically correct, no schema extension needed) | Codex round-2 must-fix #3 |
    27	| **R2-typo** | Spec § 1.1 said "8-field bytes-on-the-wire" but `LedgerEntrySigningPayload` actually has 9 fields | Updated to "9-field" | Codex Q-C2 |
    28	
    29	3 must-fix + 1 typo = **4 closures** integrated.
    30	
    31	---
    32	
    33	## v1.1 patch log (vs. v1)
    34	
    35	| ID | v1 issue | v1.1 fix | Source |
    36	|---|---|---|---|
    37	| C1 | replay was single-mode; called "I-DETHASH witness" but skeleton only did chain check | Two-mode `ReplayMode::ChainOnly` (skeleton-stage) vs `ReplayMode::FullTransition` (CO1.7.5+; I-DETHASH witness only in this mode) | Codex Q-D + Gemini Q3 |
    38	| C2 | spec did not acknowledge that shipped `CasStore::open()` initializes empty in-memory index → cold-replay impossible | § 0 + § 5 explicit dependency + mitigation: CasStore index persistence is deferred to **CO1.4-extra** (separate atom); v1 documents the gap | Codex Q-H + Gemini Q2 |
    39	| C3 | signing primitive integration via `CanonicalMessage` enum was unspecified; spec called nonexistent digest-form verifier | Path A: extend `CanonicalMessage::LedgerEntrySigning(LedgerEntrySigningPayload)`; sign separate signing payload (NOT raw `LedgerEntry`); new API `keypair.sign_ledger_entry(payload) → SystemSignature` | Codex Q-G + Gemini Q4 |
    40	| K1 | sequencer `next_logical_t.fetch_add(1)` happens BEFORE accept; rejection skips `logical_t`, replay rejects gaps | Dual counter design: `next_submit_id` advances at submit; `next_logical_t` advances ONLY on commit | Codex Q-C |
    41	| K2 | signature did NOT bind `parent_ledger_root` → transplant attack | `LedgerEntrySigningPayload` includes `parent_ledger_root` field | Codex Q-B (NEW) |
    42	| K3 | L4/L5 head_t ownership inconsistent (spec line 194 vs 276 disagreed) | CO1.7 owns `ledger_root_t` + commit-chain `head_t = NodeId(commit_sha)` only; L5 (CO1.8) owns `state_root_t` mutation; sequencer drops `head_t = NodeId::from_state_root(...)` line | Codex Q-E |
    43	| K4 | spec `LedgerWriter::commit(&self) → NodeId` + `iter_from` did not match skeleton `commit(&mut self) → Hash` | Spec aligned to skeleton: `&mut self` + `Hash` return; `iter_from` deferred to CO1.7.5+ when needed for cold-replay | Codex Q-H |
    44	| K5 | `TxKind::Slash` enum variant present but `dispatch_transition` omitted it | Drop `TxKind::Slash` for v4; ChallengeCourt slashing event scheduled for CO P2.5 atom | Codex Q-H |
    45	| K6 | `tx_kind as u8` cast without `#[repr(u8)]` → fragile discriminant | `#[repr(u8)]` + explicit discriminants (`Work = 0, Verify = 1, ...`) | Codex Q-H |
    46	| K7 | spec promised 8 conformance tests; skeleton has 6 | Explicit list of 8 tests with skeleton-stage vs CO1.7.5-stage marker; unimplemented stubs now stage-marked | Codex Q-H |
    47	| G1 | `LedgerEntry` struct rigid; future ZK / settlement proof had no place | Add `extensions: BTreeMap<String, Vec<u8>>` (empty in v1; reserved for v4.x without breaking schema) | Gemini Q9 |
    48	| D1 | epoch binding disagreement (Codex bind YES; Gemini bind NO) | Conservative resolution: epoch IS bound in `LedgerEntrySigningPayload`; epoch NOT separately folded into `ledger_root_t` (Codex security wins; Gemini orthogonality preserved at the ledger_root axis) | merged verdict § 5 |
    49	
    50	11 must-fix + 1 disagreement resolution = **12 closures** integrated below.
    51	
    52	---
    53	
    54	## § 0 Scope
    55	
    56	### In scope (CO1.7 atom)
    57	- **LedgerEntry schema**: canonical envelope wrapping each typed transition (WorkTx / VerifyTx / ChallengeTx / ReuseTx / FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx) before append to L4. **Note**: `Slash` is NOT in v4 (deferred to CO P2.5 ChallengeCourt atom — K5).
    58	- **LedgerEntrySigningPayload**: the 9-field bytes-on-the-wire that the system keypair actually signs (distinct from LedgerEntry-the-stored-record).
    59	- **LedgerRoot computation**: deterministic Merkle accumulation over signed digests; this is the value of `Q_t.ledger_root_t`.
    60	- **Sequencer**: per-(runtime_repo, run_id) single-writer instance enforcing § 5.2.1 (dual-counter `submit_id`/`logical_t`, submission-order serialization, post-commit `logical_t` assignment).
    61	- **append(parent_root, signing_digest)**: pure function returning the new ledger_root.
    62	- **replay (two-mode)**: `ChainOnly` (chain integrity; skeleton-stage; v1) vs `FullTransition` (rerun pure transitions + verify state_root + verify signatures; CO1.7.5+; THE I-DETHASH witness).
    63	- **Storage backend**: git2-rs commit chain (built on CO1.4 CAS); each LedgerEntry = one git commit on `refs/transitions/main`. **R2-K3**: head_t mutation deferred to CO1.7.5+ — v1.x ledger does NOT mutate `Q_t.head_t` directly. Once `Git2LedgerWriter::commit` exists and returns commit_sha alongside Hash, CO1.7.5 wiring will set `head_t = NodeId(commit_sha)` outside the L4 sequencer apply path.
    64	- **CanonicalMessage extension**: extends shipped `CanonicalMessage` enum with `LedgerEntrySigning([u8; 32])` opaque-digest variant (R2 design — avoids circular module dep); new typed sign API `transition_ledger_emitter::sign_ledger_entry(keypair, signing_payload_digest)`.
    65	
    66	### Out of scope (handled by other atoms)
    67	- WorkTx / VerifyTx / ChallengeTx / ReuseTx / FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx schemas — frozen in `STATE_TRANSITION_SPEC § 1`.
    68	- step_transition / verify_transition / challenge_transition logic — frozen in `STATE_TRANSITION_SPEC § 3`.
    69	- system_keypair signing primitives — done @ CO1.7.0a-f; CO1.7 only adds a typed extension.
    70	- L5 materializer (state_root computation) — deferred to **CO1.8**. **K3 boundary (revised v1.2)**: CO1.7 owns `ledger_root_t` only; CO1.8 owns `state_root_t`; **head_t mutation is deferred to CO1.7.5+ wiring** (when `Git2LedgerWriter` exists). Sequencer does NOT mutate `state_root_t` or `head_t` directly; it accepts `q_next.state_root_t` as returned by the transition function and persists `ledger_root_t`.
    71	- L6 signal indices — deferred to **CO1.9**.
    72	- AttributionEngine DAG — deferred to CO P2.4.0 spike (Inv 8 design).
    73	- MetaTx full schema — v4.1 only; v4 emits `MetaProposalDraft` to L3 CAS, not L4.
    74	- **Slash transition** — deferred to CO P2.5 (ChallengeCourt) atom; v4 ledger has no `TxKind::Slash`.
    75	- **CAS index persistence (cold-replay enabler)** — `CasStore::open()` shipped at Wave 3 initializes empty in-memory index ([store.rs:67](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs)); cold-replay therefore cannot recover payloads via `CasStore::get` after restart. **CO1.4-extra** atom (NEW, scheduled post-CO1.7) adds index persistence (likely a sidecar JSONL or git-tag manifest). v1 ledger documents the dependency; full-mode replay is implementable once CO1.4-extra lands.
    76	
    77	### What this spec is NOT replacing
    78	- `src/ledger.rs` (legacy, top-level) is retired in **CO1.1.5 (kernel.rs split)**; CO1.7 lives at `src/bottom_white/ledger/transition_ledger.rs` (NEW). No STEP_B parallel-branch ceremony required (new module, not restricted file); restricted files per CLAUDE.md "Code Standard" are `src/{kernel,bus,wallet}.rs` (corrected from v1's incorrect `wal.rs` per K6 tail).
    79	
    80	---
    81	
    82	## § 1 LedgerEntry schema (the stored record)
    83	
    84	```rust
    85	use std::collections::BTreeMap;
    86	
    87	/// TRACE_MATRIX FC2-Append (FC2 transition machinery): canonical record
    88	/// stored at L4. One LedgerEntry per accepted transition. Genesis state has
    89	/// zero LedgerEntries; ledger_root_t = genesis_ledger_root_t (per § 5).
    90	#[derive(Debug, Clone, PartialEq, Eq)]
    91	pub struct LedgerEntry {
    92	    /// Monotonic counter from sequencer; starts at 1 at first accept.
    93	    /// **K1**: assigned ONLY at commit; rejected submissions never get a logical_t.
    94	    pub logical_t: u64,                          //  1
    95	
    96	    /// Parent state_root before this transition. Equals the resulting_state_root
    97	    /// of the entry at logical_t-1 (or genesis state_root at logical_t=1).
    98	    pub parent_state_root: Hash,                 //  2
    99	
   100	    /// **K2 NEW**: parent_ledger_root before this entry is folded in.
   101	    /// Bound by signature (transplant attack defense). Equals the
   102	    /// resulting_ledger_root of entry at logical_t-1 (or genesis_ledger_root at logical_t=1).
   103	    pub parent_ledger_root: Hash,                //  3
   104	
   105	    /// Discriminator; payload schema depends on this. **K6**: `#[repr(u8)]` for stable
   106	    /// discriminant in canonical digest computation.
   107	    pub tx_kind: TxKind,                         //  4
   108	
   109	    /// CAS handle (CO1.4) to canonically-serialized payload. Sequencer puts payload
   110	    /// to CAS via `CasStore::put(content, object_type, creator, created_at_logical_t, schema_id)`
   111	    /// (DIV-5: 5-param signature). cid = sha256(content).
   112	    pub tx_payload_cid: Cid,                     //  5
   113	
   114	    /// Resulting state_root after `dispatch_transition` applied. NOT mutated by L4
   115	    /// — accepted as-returned from the transition function (K3 boundary).
   116	    pub resulting_state_root: Hash,              //  6
   117	
   118	    /// Resulting ledger_root after this entry is folded in.
   119	    /// Convention: ledger_root_{t+1} = sha256(domain_sep || parent_ledger_root || signing_digest_t)
   120	    /// where signing_digest_t = canonical_digest(LedgerEntrySigningPayload at logical_t).
   121	    /// **NOT signed** — derivative; including it in signed digest creates a cycle (Q9).
   122	    pub resulting_ledger_root: Hash,             //  7
   123	
   124	    /// Wall-clock-free timestamp; equal to `logical_t` post-commit (no separate clock).
   125	    /// Field retained for symmetry with STATE_TRANSITION_SPEC § 1.2 WorkTx.
   126	    pub timestamp_logical: u64,                  //  8
   127	
   128	    /// **DIV-3 / Q10**: which pinned epoch pubkey signed this entry. Required by
   129	    /// `system_keypair::verify_system_signature(sig, msg, epoch, pinned_pubkeys)`.
   130	    /// Bound in signed payload (Codex security argument; **D1** resolved).
   131	    pub epoch: SystemEpoch,                      //  9
   132	
   133	    /// **G1 NEW**: forward-compatibility extension map. Empty in v1; reserved for
   134	    /// v4.x additions (e.g. ZK predicate proofs, settlement proofs, public-market metadata).
   135	    /// Bound in signed payload (so additions cannot bypass signature).
   136	    pub extensions: BTreeMap<String, Vec<u8>>,   // 10
   137	
   138	    /// Detached system signature over canonical_digest of LedgerEntrySigningPayload.
   139	    /// Distinct from agent-signature inside payload. NOT included in signed digest.
   140	    pub system_signature: SystemSignature,       // 11
   141	}
   142	
   143	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
   144	#[repr(u8)]
   145	pub enum TxKind {
   146	    Work            = 0,   // WorkTx       (STATE spec § 1.2)
   147	    Verify          = 1,   // VerifyTx     (STATE spec § 1.3)
   148	    Challenge       = 2,   // ChallengeTx  (STATE spec § 1.3)
   149	    Reuse           = 3,   // ReuseTx      (STATE spec § 1.3)
   150	    FinalizeReward  = 4,   // FinalizeRewardTx (STATE spec § 3.4)
   151	    TaskExpire      = 5,   // TaskExpireTx (STATE spec § 3.6)
   152	    TerminalSummary = 6,   // TerminalSummaryTx (STATE spec § 1.5 + § 3.7)
   153	    // K5: NO `Slash` — ChallengeCourt slash event is in CO P2.5; v4 ledger has no Slash variant.
   154	}
   155	```
   156	
   157	### § 1.1 LedgerEntrySigningPayload (NEW per C3)
   158	
   159	The system signature signs a **separate struct**, not the LedgerEntry directly. This:
   160	1. **Excludes derivatives**: `resulting_ledger_root` (cycle: ledger_root ⊃ digest ⊃ ledger_root) and `system_signature` (its own input) are NOT in the signing payload.
   161	2. **Binds non-derivatives**: `parent_ledger_root` (K2 transplant defense) + `epoch` (D1 + Q10) + `extensions` (G1 forward compat is signed).
   162	3. **Has stable wire format**: explicit byte layout (see canonical_digest below), independent of bincode/serde choices.
   163	
   164	```rust
   165	#[derive(Debug, Clone, PartialEq, Eq)]
   166	pub struct LedgerEntrySigningPayload {
   167	    pub logical_t: u64,                         // 1
   168	    pub parent_state_root: Hash,                // 2
   169	    pub parent_ledger_root: Hash,               // 3 (K2 transplant defense)
   170	    pub tx_kind: TxKind,                        // 4 (#[repr(u8)] discriminant cast safe)
   171	    pub tx_payload_cid: Cid,                    // 5
   172	    pub resulting_state_root: Hash,             // 6
   173	    pub timestamp_logical: u64,                 // 7
   174	    pub epoch: SystemEpoch,                     // 8 (D1)
   175	    pub extensions: BTreeMap<String, Vec<u8>>,  // 9 (G1; empty-map case yields empty bytes)
   176	    // EXCLUDED: resulting_ledger_root (cycle); system_signature (its input).
   177	}
   178	
   179	impl LedgerEntrySigningPayload {
   180	    pub fn canonical_digest(&self) -> Hash {
   181	        let mut h = Sha256::new();
   182	        h.update(b"turingosv4.ledger_entry_signing.v1");      // domain separation
   183	        h.update(self.logical_t.to_be_bytes());
   184	        h.update(self.parent_state_root.0);
   185	        h.update(self.parent_ledger_root.0);
   186	        h.update((self.tx_kind as u8).to_be_bytes());          // K6 #[repr(u8)] makes this stable
   187	        h.update(self.tx_payload_cid.0);
   188	        h.update(self.resulting_state_root.0);
   189	        h.update(self.timestamp_logical.to_be_bytes());
   190	        h.update(self.epoch.get().to_be_bytes());
   191	        // extensions: BTreeMap iteration is sorted by key (deterministic);
   192	        // length-prefix each (key, value) pair to prevent ambiguity.
   193	        h.update((self.extensions.len() as u64).to_be_bytes());
   194	        for (k, v) in &self.extensions {                        // BTreeMap = lex order
   195	            h.update((k.len() as u64).to_be_bytes());
   196	            h.update(k.as_bytes());
   197	            h.update((v.len() as u64).to_be_bytes());
   198	            h.update(v);
   199	        }
   200	        Hash(h.finalize().into())
   201	    }
   202	}
   203	```
   204	
   205	### § 1.2 CanonicalMessage extension (per C3; **shipped in v1.2**)
   206	
   207	CO1.7 extends shipped `system_keypair::CanonicalMessage` with one new variant. **R2-C3 design choice**: variant carries the **opaque 32-byte canonical_digest** of `LedgerEntrySigningPayload`, NOT the full payload struct. This avoids a circular `system_keypair ↔ transition_ledger` module dependency (the payload struct needs `Cid` from CAS module + `SystemEpoch` from system_keypair; carrying the precomputed digest sidesteps the cycle entirely). The signature still binds the full payload because `canonical_digest()` is deterministic in `transition_ledger`.
   208	
   209	```rust
   210	// In src/bottom_white/ledger/system_keypair.rs (additive Wave 4-B extension; SHIPPED v1.2):
   211	pub enum CanonicalMessage {
   212	    RejectedAttemptSummary(RejectedAttemptSummary),  // existing
   213	    TerminalSummaryTx(TerminalSummaryTx),            // existing
   214	    EpochRotationProof(EpochRotationProof),          // existing
   215	    LedgerEntrySigning([u8; 32]),                     // NEW v1.2 (C3) — opaque digest
   216	}
   217	
   218	// canonical_digest() match arm (SHIPPED v1.2):
   219	//   CanonicalMessage::LedgerEntrySigning(digest) => {
   220	//       h.update(b"LedgerEntrySigning");
   221	//       h.update(digest);
   222	//   }
   223	
   224	// Authorized emitter module (SHIPPED v1.2):
   225	pub(crate) mod transition_ledger_emitter {
   226	    pub(crate) fn sign_ledger_entry(
   227	        keypair: &Ed25519Keypair,
   228	        signing_payload_digest: [u8; 32],
   229	    ) -> Result<SystemSignature, KeypairError>;
   230	}
   231	```
   232	
   233	**Sequencer call site** (in transition_ledger.rs, illustrative):
   234	
   235	```rust
   236	let digest = signing_payload.canonical_digest();
   237	let sig = transition_ledger_emitter::sign_ledger_entry(&keypair, digest.0)?;
   238	```
   239	
   240	**Forward-compat clause**: if v4.x adds new ledger-side message variants, they MUST add new `CanonicalMessage::*` variants (NOT extend the LedgerEntrySigning variant in-place; opaque digest is committed to `[u8; 32]`). v4-shipped extensions go in the `LedgerEntry::extensions` BTreeMap (G1) which IS bound in `LedgerEntrySigningPayload::canonical_digest()`.
   241	
   242	---
   243	
   244	## § 2 Module layout
   245	
   246	```
   247	src/bottom_white/ledger/
   248	├── mod.rs                       (existing; v1.1 wires `pub mod transition_ledger`)
   249	├── system_keypair.rs            (existing CO1.7.0a-f; CO1.7 adds 1 enum variant + 1 typed sign fn — additive)
   250	└── transition_ledger.rs         (NEW; LedgerEntry, LedgerEntrySigningPayload, TxKind, append, replay_*, LedgerWriter)
   251	
   252	src/state/
   253	├── mod.rs                       (existing)
   254	├── q_state.rs                   (existing; CO1.7 fills `ledger_root_t` placeholder; does NOT touch `state_root_t` per K3)
   255	└── sequencer.rs                 (NEW; deferred to CO1.7.5; pre-audit type stub may land in v1.1 if useful)
   256	```
   257	
   258	**Crate boundary**: `transition_ledger` in `bottom_white::ledger` (tool layer); `sequencer` in `state::` (touches Q_t mutation). Sequencer DEPENDS ON ledger; ledger does NOT depend on sequencer (DAG: state → bottom_white::ledger → CO1.4 CAS → CO1.7.0a-f keypair).
   259	
   260	---
   261	
   262	## § 3 Sequencer (K1 dual-counter; K3 head_t ownership; C3 sign API)
   263	
   264	```rust
   265	/// TRACE_MATRIX § 5.2.1 — L4 sequencer; single-writer per (runtime_repo, run_id).
   266	pub struct Sequencer {
   267	    /// **K1 NEW**: separate counter for submissions (independent of accept).
   268	    /// Used to derive submit_id for SubmissionReceipt; never appears in LedgerEntry.
   269	    next_submit_id: AtomicU64,
   270	
   271	    /// **K1 changed semantics**: advances ONLY on commit, NOT on submit.
   272	    /// Genesis = 0; first accepted entry gets logical_t=1.
   273	    next_logical_t: AtomicU64,
   274	
   275	    /// **Q1 resolution**: bounded `tokio::sync::mpsc::Sender` (NOT unbounded).
   276	    /// Submit returns `QueueFull` Err on saturation; agents handle backoff.
   277	    queue_tx: tokio::sync::mpsc::Sender<TypedTx>,
   278	
   279	    /// CAS handle for payload storage.
   280	    cas: Arc<RwLock<CasStore>>,
   281	
   282	    /// **C3**: signing key handle (CO1.7.0a-f).
   283	    keypair: Arc<Ed25519Keypair>,
   284	    epoch: SystemEpoch,                   // current signing epoch
   285	
   286	    /// Storage backend (in CO1.7.5+; skeleton uses InMemoryLedgerWriter).
   287	    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
   288	
   289	    /// Predicate + tool registries (read-only).
   290	    predicate_registry: Arc<PredicateRegistry>,
   291	    tool_registry: Arc<ToolRegistry>,
   292	
   293	    /// Current Q_t snapshot.
   294	    q: RwLock<QState>,
   295	}
   296	
   297	impl Sequencer {
   298	    /// Submit a typed transition for processing. Returns immediately with a
   299	    /// SubmissionReceipt carrying `submit_id` (NOT logical_t — submit_id is
   300	    /// always assigned; logical_t only assigned post-accept).
   301	    pub async fn submit(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError>;
   302	
   303	    /// Driver loop: drain queue, run transition, commit on accept. Single-thread internal.
   304	    pub async fn run(&self) -> Result<(), SequencerError>;
   305	
   306	    /// Per-tx critical section.
   307	    fn apply_one(&self, tx: TypedTx) -> Result<LedgerEntry, TransitionError> {
   308	        // 1. Snapshot Q_t under read lock
   309	        let q_snapshot = self.q.read().clone();
   310	
   311	        // 2. Dispatch (pure)
   312	        let (q_next, _signals) = dispatch_transition(&q_snapshot, &tx, &self.predicate_registry, &self.tool_registry)?;
   313	        // **K1**: if step returns Err, EARLY RETURN — no logical_t assigned, no entry committed.
   314	
   315	        // 3. Put payload to CAS (DIV-5 5-param signature)
   316	        let mut cas_w = self.cas.write();
   317	        let cas_bytes = canonical_serialize(&tx);  // bincode v2 per § 2.5 of STATE spec
   318	        let payload_cid = cas_w.put(
   319	            &cas_bytes,
   320	            ObjectType::ProposalPayload,  // R2 fix: shipped CAS variant (NOT Transition)
   321	            &format!("sequencer-{}", self.epoch.get()),
   322	            self.next_logical_t.load(Ordering::SeqCst) + 1,  // tentative; final below
   323	            Some("LedgerEntrySigningPayload.v1".to_string()),
   324	        )?;
   325	        drop(cas_w);
   326	
   327	        // 4. **K1**: assign logical_t ONLY now (post-accept)
   328	        let logical_t = self.next_logical_t.fetch_add(1, Ordering::SeqCst) + 1;
   329	
   330	        // 5. Build LedgerEntrySigningPayload
   331	        let signing_payload = LedgerEntrySigningPayload {
   332	            logical_t,
   333	            parent_state_root: q_snapshot.state_root_t,
   334	            parent_ledger_root: q_snapshot.ledger_root_t,   // K2 transplant defense
   335	            tx_kind: TxKind::from_typed(&tx),
   336	            tx_payload_cid: payload_cid,
   337	            resulting_state_root: q_next.state_root_t,
   338	            timestamp_logical: logical_t,
   339	            epoch: self.epoch,
   340	            extensions: BTreeMap::new(),                     // G1 empty in v1
   341	        };
   342	
   343	        // 6. **C3 NEW SIGN API (v1.2)**: typed sign through CanonicalMessage extension.
   344	        // Compute payload digest in transition_ledger; pass opaque [u8; 32] to emitter.
   345	        let signing_payload_digest = signing_payload.canonical_digest();
   346	        let system_signature = transition_ledger_emitter::sign_ledger_entry(
   347	            &self.keypair,
   348	            signing_payload_digest.0,
   349	        )?;
   350	
   351	        // 7. Compute resulting_ledger_root via append() (pure)
   352	        let signing_digest = signing_payload.canonical_digest();
   353	        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);
   354	
   355	        // 8. Build LedgerEntry (the stored record)
   356	        let entry = LedgerEntry {
   357	            logical_t: signing_payload.logical_t,
   358	            parent_state_root: signing_payload.parent_state_root,
   359	            parent_ledger_root: signing_payload.parent_ledger_root,
   360	            tx_kind: signing_payload.tx_kind,
   361	            tx_payload_cid: signing_payload.tx_payload_cid,
   362	            resulting_state_root: signing_payload.resulting_state_root,
   363	            resulting_ledger_root,                            // derived; not in signed digest
   364	            timestamp_logical: signing_payload.timestamp_logical,
   365	            epoch: signing_payload.epoch,
   366	            extensions: signing_payload.extensions,
   367	            system_signature,
   368	        };
   369	
   370	        // 9. Commit + mutate Q_t under write lock (atomic)
   371	        let mut q_w = self.q.write();
   372	        let mut writer_w = self.ledger_writer.write();
   373	        writer_w.commit(&entry)?;                              // K4 returns Hash; matches skeleton
   374	        drop(writer_w);
   375	        *q_w = q_next;
   376	        q_w.ledger_root_t = entry.resulting_ledger_root;
   377	        // **K3 (v1.2 revised)**: do NOT mutate q_w.head_t here. v1.x ledger owns
   378	        // `ledger_root_t` only. head_t mutation is **deferred to CO1.7.5+ wiring**
   379	        // (when `Git2LedgerWriter::commit` is implemented and can return commit_sha
   380	        // alongside Hash). Until then, head_t remains at QState placeholder; replay
   381	        // and chain-integrity tests do NOT depend on head_t.
   382	
   383	        Ok(entry)
   384	    }
   385	}
   386	```
   387	
   388	**Why dual counter (K1)**: rejection of a submission must NOT consume a logical_t, because (a) skeleton's `InMemoryLedgerWriter::commit` enforces `expected_logical_t = len + 1` and would reject a gap; (b) replay enforces `entry.logical_t == (i+1)` and would reject a gap. Submitter IDs (`submit_id`) are returned from `submit()` immediately for receipt; logical_t is observable only on the committed entry.
   389	
   390	**Why no head_t mutation in apply_one (K3, revised v1.2)**: v1.x CO1.7 owns `ledger_root_t` only. CO1.8 owns `state_root_t`. **head_t mutation deferred to CO1.7.5+** when `Git2LedgerWriter` provides a commit_sha return alongside Hash; the InMemoryLedgerWriter used by the v1 skeleton has no commit_sha to expose, so the trait keeps a single `Hash` return and head_t wiring is a separate downstream concern. Sequencer never calls `NodeId::from_state_root(...)`.
   391	
   392	**Q3 (Gemini)**: `Sequencer` vs `LedgerWriter + OrderingCoordinator` split — v1.1 keeps `Sequencer` as the abstraction; trait-segregation refactor is a v4.x consideration (the current single-writer constraint per § 5.2.1 makes the split synthetic for v1).
   393	
   394	---
   395	
   396	## § 4 append() + replay() — two-mode (per C1)
   397	
   398	```rust
   399	/// Pure. Same (parent_root, signing_digest) → byte-identical new_root.
   400	/// No I/O, no clock, no env. Witness for I-DET / I-DETHASH ledger axis.
   401	pub fn append(parent_root: &Hash, signing_digest: &Hash) -> Hash {
   402	    let mut h = Sha256::new();
   403	    h.update(b"turingosv4.ledger_root.v1");      // domain separation
   404	    h.update(parent_root.0);
   405	    h.update(signing_digest.0);
   406	    Hash(h.finalize().into())
   407	}
   408	
   409	/// Replay mode (C1).
   410	pub enum ReplayMode {
   411	    /// Skeleton-stage: validates parent_state_root + parent_ledger_root + ledger_root chain.
   412	    /// Does NOT verify signatures, re-fetch payloads, or re-run pure transitions.
   413	    /// Trust mode: "trust the sequencer". v1 deliverable.
   414	    ChainOnly,
   415	    /// CO1.7.5+ stage: full re-execution.
   416	    /// Verifies signatures via CanonicalMessage; fetches payloads from CAS;
   417	    /// re-runs pure dispatch_transition; compares resulting_state_root.
   418	    /// **THIS** is the I-DETHASH witness (I-DETHASH bound to FullTransition only).
   419	    /// Requires CO1.4-extra (CAS index persistence) for cold-restart.
   420	    FullTransition,
   421	}
   422	
   423	/// Skeleton-stage entry point (v1).
   424	pub fn replay_chain_integrity(
   425	    genesis_state_root: Hash,
   426	    genesis_ledger_root: Hash,
   427	    entries: &[LedgerEntry],
   428	) -> Result<(Hash, Hash), ReplayError>;
   429	
   430	/// CO1.7.5+ stage entry point (v1.1 spec only; impl deferred).
   431	pub fn replay_full_transition(
   432	    genesis: &QState,
   433	    entries: &[LedgerEntry],
   434	    cas: &dyn LedgerCasView,
   435	    pinned_pubkeys: &PinnedSystemPubkeys,
   436	) -> Result<QState, ReplayError>;
   437	```
   438	
   439	**I-DETHASH witness (revised per C1)**: `replay_full_transition` is the I-DETHASH witness. `replay_chain_integrity` is necessary-but-not-sufficient — passing chain check does NOT prove transition determinism. v1 documents this explicitly to close trust ambiguity.
   440	
   441	**ReplayError enum** (skeleton already has 3 variants; v1.1 adds 4 more for FullTransition):
   442	- `LogicalTGap { at, expected, got }` (existing)
   443	- `ParentMismatch { at }` (existing; covers parent_state_root)
   444	- `LedgerRootMismatch { at }` (existing)
   445	- `ParentLedgerRootMismatch { at }` (NEW K2)
   446	- `BadSignature { at }` (NEW; FullTransition only)
   447	- `CasMissing { at, cid }` (NEW; FullTransition only — fires if CO1.4-extra not yet landed)
   448	- `StateRootMismatch { at }` (NEW; FullTransition only)
   449	- `TransitionError { at, inner }` (NEW; wraps dispatch_transition errors)
   450	
   451	---
   452	
   453	## § 5 Storage backend
   454	
   455	**Choice**: git2-rs commit chain (Path B substrate, ratified per Const Art 0.4 + WP § 5.L4).
   456	
   457	**Mapping**:
   458	- One `LedgerEntry` = one git commit on `refs/transitions/main`.
   459	- Commit message = canonical-serialized `LedgerEntry` (bincode v2 per `STATE_TRANSITION_SPEC § 2.5`).
   460	- Commit tree = `(payload_cid_blob, signature_blob)` (state_root NOT a tree blob — per K3, L5 owns state_root materialization).
   461	- **K3 (v1.2)**: `head_t = NodeId(commit_sha)` is the canonical convention WHEN head_t is wired (CO1.7.5+). v1.x sequencer does NOT mutate head_t — `Git2LedgerWriter` is needed to surface commit_sha. `NodeId::from_state_root(...)` is NOT used by L4 in any version.
   462	- **C2**: cold-replay availability requires `CasStore` index persistence; deferred to CO1.4-extra. Until then, full-mode replay errors with `CasMissing` if CAS state is not warm.
   463	- Genesis: `refs/transitions/main` is created at the empty-tree commit corresponding to `genesis_payload.toml` (CO1.0). `genesis_ledger_root_t = sha256("turingosv4.ledger_root.v1.genesis" || sha256(genesis_payload.toml))` — **Q7 resolution** (NOT `Hash::ZERO`; both auditors agreed).
   464	
   465	**LedgerWriter trait (K4 reconciled to skeleton)**:
   466	
   467	```rust
   468	pub trait LedgerWriter: Send + Sync {
   469	    /// Commit a signed LedgerEntry. K4: `&mut self` + `Hash` return matches skeleton.
   470	    /// Returns the entry's `resulting_ledger_root`.

context compacted
codex
I’ve completed the structural closure checks from the source and spec. I’m running the full library test pass now because Q-6 asks for the reported 199/0 result specifically.
exec
/bin/bash -lc "sha256sum handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md src/bottom_white/ledger/system_keypair.rs src/bottom_white/ledger/transition_ledger.rs src/bottom_white/ledger/mod.rs genesis_payload.toml && rg -n \"CO1_7_TRANSITION_LEDGER|src/bottom_white/ledger/(system_keypair|transition_ledger|mod)\\.rs\" genesis_payload.toml" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
00ad743c162bdef2bc9c30ccd6ee7673522214b6be8e33f748e0f8fba625772a  handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md
ab0a63ca9fad9b9f931e8a3082c70ce6b4ff6432ae96013b03a19bda7c180c82  src/bottom_white/ledger/system_keypair.rs
4b1a15546e7d33d85e45b74b61063d21af63594efc6c60ee3ed783393ab108d1  src/bottom_white/ledger/transition_ledger.rs
edb4cda94b80dceb85a414de58139fde4f6f4635867141e31c14720ab06148ea  src/bottom_white/ledger/mod.rs
e8381b25b9c6b0cbf87b361fc90454fa15ce158bb583dd431f1bfdc7d3954408  genesis_payload.toml
252:"handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md" = "00ad743c162bdef2bc9c30ccd6ee7673522214b6be8e33f748e0f8fba625772a"
281:"src/bottom_white/ledger/mod.rs" = "edb4cda94b80dceb85a414de58139fde4f6f4635867141e31c14720ab06148ea"
282:"src/bottom_white/ledger/system_keypair.rs" = "ab0a63ca9fad9b9f931e8a3082c70ce6b4ff6432ae96013b03a19bda7c180c82"
284:"src/bottom_white/ledger/transition_ledger.rs" = "4b1a15546e7d33d85e45b74b61063d21af63594efc6c60ee3ed783393ab108d1"

exec
/bin/bash -lc 'cargo test --lib' in /home/zephryj/projects/turingosv4
 succeeded in 1453ms:
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
 --> src/bottom_white/cas/store.rs:9:12
  |
9 | use git2::{ObjectType as Git2ObjectType, Repository};
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
   --> src/bottom_white/ledger/system_keypair.rs:503:19
    |
503 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_terminal_summary_tx` is never used
   --> src/bottom_white/ledger/system_keypair.rs:519:19
    |
519 |     pub(crate) fn sign_terminal_summary_tx(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:538:19
    |
538 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib test) generated 13 warnings (run `cargo fix --lib -p turingosv4 --tests` to apply 10 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-128d20df5fbbc885)

running 199 tests
test boot::tests::parse_errors_on_unquoted_key ... ok
test boot::tests::parse_errors_when_section_missing ... ok
test boot::tests::parse_strips_inline_comment_and_blanks ... ok
test boot::tests::verify_trust_root_detects_tamper_in_tempdir ... ok
test boot::tests::verify_trust_root_detects_child_manifest_tamper ... ok
test boot::tests::verify_trust_root_passes_when_hash_matches_in_tempdir ... ok
test bottom_white::cas::schema::tests::cid_deterministic ... ok
test bottom_white::cas::schema::tests::cid_differs_on_content ... ok
test boot::tests::verify_trust_root_passes_with_matching_child_manifest ... ok
test bottom_white::cas::schema::tests::cid_display_format ... ok
test bottom_white::cas::schema::tests::metadata_canonical_hash_deterministic ... ok
test bottom_white::cas::schema::tests::cid_from_empty_content ... ok
test bottom_white::cas::schema::tests::metadata_canonical_hash_differs_on_object_type ... ok
test bottom_white::cas::store::tests::cid_is_content_address ... ok
test bottom_white::cas::store::tests::empty_store_root ... ok
test bottom_white::cas::store::tests::cell_isolation_disjoint_cas ... ok
test bottom_white::cas::store::tests::get_nonexistent_returns_error ... ok
test bottom_white::cas::store::tests::metadata_recorded ... ok
test bottom_white::cas::store::tests::merkle_root_deterministic_two_runs ... ok
test bottom_white::cas::store::tests::put_get_round_trip_small ... ok
test bottom_white::cas::store::tests::put_idempotent_same_content ... ok
test bottom_white::ledger::system_keypair::tests::authorized_scope_signing_round_trip ... ok
test bottom_white::cas::store::tests::put_get_round_trip_large ... ok
test bottom_white::ledger::transition_ledger::tests::append_is_pure_and_byte_stable ... ok
test bottom_white::ledger::transition_ledger::tests::canonical_digest_excludes_derivatives ... ok
test bottom_white::ledger::transition_ledger::tests::canonical_digest_stable_across_clones ... ok
test bottom_white::ledger::transition_ledger::tests::in_memory_writer_enforces_logical_t ... ok
test bottom_white::ledger::transition_ledger::tests::replay_chain_integrity_clean ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
test bottom_white::ledger::system_keypair::tests::terminal_scope_rotation_signing_round_trip ... ok
test bottom_white::tools::registry::tests::duplicate_id_rejected ... ok
test bottom_white::tools::registry::tests::empty_id_rejected ... ok
test bottom_white::tools::registry::tests::empty_registry ... ok
test bottom_white::tools::registry::tests::find_by_capability_replaces_magic_string ... ok
test bottom_white::tools::registry::tests::merkle_root_deterministic ... ok
test bottom_white::tools::registry::tests::non_idempotent_rejected ... ok
test bottom_white::tools::registry::tests::register_and_get_round_trip ... ok
test bus::tests::test_bus_basic_append ... ok
test bus::tests::test_bus_classify_bounded ... ok
test bottom_white::cas::store::tests::put_many_then_iterate_count ... ok
test bus::tests::test_bus_forbidden_pattern_veto ... ok
test bus::tests::test_bus_creates_market_on_append ... ok
test bus::tests::test_bus_halt_and_settle ... ok
test bus::tests::test_bus_graveyard_feedback ... ok
test bus::tests::test_bus_payload_too_long ... ok
test bus::tests::test_bus_ledger_integrity ... ok
test bus::tests::test_bus_serial_ordering ... ok
test bus::tests::test_bus_snapshot ... ok
test bus::tests::test_bus_too_many_lines ... ok
test drivers::llm_http::tests::test_client_creation ... ok
test bus::tests::test_bus_unknown_agent_vetoed ... ok
test drivers::llm_http::tests::test_driver_error_display ... ok
test economy::money::tests::checked_add_normal ... ok
test drivers::llm_http::tests::test_generate_request_serialization ... ok
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
test economy::money::tests::royalty_rejects_weight_above_1 ... ok
test economy::money::tests::royalty_rejects_negative ... ok
test economy::money::tests::serde_transparent_format ... ok
test economy::money::tests::serde_round_trip_json ... ok
test kernel::tests::test_append_and_retrieve ... ok
test kernel::tests::test_market_lifecycle ... ok
test kernel::tests::test_market_ticker ... ok
test kernel::tests::test_golden_path_trace ... ok
test kernel::tests::test_no_duplicate_market ... ok
test kernel::tests::test_reject_dangling_citation ... ok
test kernel::tests::test_no_market_for_nonexistent_node ... ok
test kernel::tests::test_reject_duplicate ... ok
test kernel::tests::test_resolve_all_markets ... ok
test ledger::tests::test_ledger_hash_chain_integrity ... ok
test ledger::tests::test_ledger_append_and_verify ... ok
test ledger::tests::test_ledger_omega_vocabulary ... ok
test ledger::tests::test_ledger_tamper_detection ... ok
test ledger::tests::test_ledger_sequence_monotonic ... ok
test ledger::tests::test_tape_append_root_node ... ok
test ledger::tests::test_tape_append_with_valid_citation ... ok
test ledger::tests::test_tape_empty ... ok
test ledger::tests::test_tape_dag_branching ... ok
test ledger::tests::test_tape_reject_dangling_citation ... ok
test ledger::tests::test_tape_reject_duplicate_id ... ok
test ledger::tests::test_tape_time_arrow_ordering ... ok
test ledger::tests::test_tape_trace_ancestors ... ok
test prediction_market::tests::test_assassin_profit ... ok
test prediction_market::tests::test_buy_yes_increases_yes_price ... ok
test prediction_market::tests::test_constant_product_invariant ... ok
test prediction_market::tests::test_buy_no_increases_no_price ... ok
test prediction_market::tests::test_create_market ... ok
test prediction_market::tests::test_ctf_conservation_1_coin_1_yes_1_no ... ok
test prediction_market::tests::test_multiple_traders_price_discovery ... ok
test prediction_market::tests::test_initial_price_is_50_50 ... ok
test prediction_market::tests::test_no_double_resolution ... ok
test prediction_market::tests::test_no_trading_after_resolution ... ok
test prediction_market::tests::test_pioneer_profit ... ok
test prediction_market::tests::test_redeem_requires_resolution ... ok
test prediction_market::tests::test_reject_zero_or_negative_amounts ... ok
test prediction_market::tests::test_prices_sum_to_one ... ok
test sdk::actor::tests::test_boltzmann_never_returns_none_with_nodes ... ok
test sdk::actor::tests::test_boltzmann_returns_none_empty_tape ... ok
test sdk::actor::tests::test_frontier_detection_leaf ... ok
test sdk::actor::tests::test_frontier_detection_parent_with_child ... ok
test sdk::actor::tests::test_boltzmann_diversity_not_deterministic ... ok
test sdk::actor::tests::test_lineage_score_increases_with_depth ... ok
test sdk::error_abstraction::tests::fixture_linarith_failed ... ok
test sdk::error_abstraction::tests::classifier_version_is_stamped ... ok
test sdk::error_abstraction::tests::fixture_rewrite_no_match ... ok
test sdk::error_abstraction::tests::fixture_simp_no_progress ... ok
test sdk::error_abstraction::tests::fixture_other_catchall ... ok
test sdk::error_abstraction::tests::fixture_type_mismatch ... ok
test sdk::error_abstraction::tests::fixture_unexpected_token ... ok
test sdk::error_abstraction::tests::fixture_unsolved_goals ... ok
test sdk::error_abstraction::tests::fixture_unknown_constant ... ok
test sdk::error_abstraction::tests::labels_are_unique_and_stable ... ok
test sdk::prompt::tests::test_prompt_contains_no_example_values ... ok
test sdk::prompt::tests::test_prompt_includes_balance ... ok
test sdk::prompt::tests::test_prompt_surfaces_team_board ... ok
test sdk::prompt::tests::test_prompt_surfaces_search_hits ... ok
test sdk::prompt::tests::test_prompt_truncates_errors_to_3 ... ok
test sdk::prompt_guard::tests::test_case_insensitive_match - should panic ... ok
test sdk::prompt_guard::tests::test_clean_prompt_passes ... ok
test sdk::prompt_guard::tests::test_empty_prompt_passes ... ok
test sdk::prompt_guard::tests::test_h_vpput_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_assignment_pattern_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_runtime_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_m_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_substring_in_larger_text - should panic ... ok
test sdk::prompt_guard::tests::test_wbcg_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_verified_caught - should panic ... ok
test sdk::protocol::tests::test_deduct_negative_amount_rejected ... ok
test sdk::protocol::tests::test_malformed_action_tag_rejected_not_fallback ... ok
test sdk::protocol::tests::test_parse_action_tag_valid ... ok
test sdk::protocol::tests::test_parse_action_tag_with_think_block ... ok
test sdk::protocol::tests::test_no_byte_repair_on_invalid_escape ... ok
test sdk::protocol::tests::test_parse_bare_json_fallback ... ok
test sdk::protocol::tests::test_parse_invalid_json_returns_error ... ok
test sdk::protocol::tests::test_parse_no_action_returns_error ... ok
test sdk::protocol::tests::test_strip_multiple_think_blocks ... ok
test sdk::protocol::tests::test_parse_with_invest_action ... ok
test sdk::protocol::tests::test_strip_unclosed_think_block ... ok
test sdk::protocol::tests::test_strip_think_blocks ... ok
test bottom_white::ledger::transition_ledger::tests::signature_round_trip_and_transplant_defense ... ok
test sdk::sandbox::tests::test_sandbox_captures_stderr ... ok
test sdk::sandbox::tests::test_sandbox_nonzero_exit ... ok
test sdk::sandbox::tests::test_sandbox_echo_command ... ok
test sdk::snapshot::tests::test_snapshot_balance_query ... ok
test sdk::tools::librarian::tests::test_board_post_append ... ok
test sdk::tools::librarian::tests::test_board_write_read_roundtrip ... ok
test sdk::tools::librarian::tests::test_build_compression_prompt ... ok
test sdk::tools::librarian::tests::test_zero_interval_never_compresses ... ok
test sdk::tools::librarian::tests::test_compress_interval ... ok
test sdk::tools::search::tests::test_sanitize_query ... ok
test sdk::tools::search::tests::test_search_nonexistent_path ... ok
test sdk::tools::search::tests::test_search_empty_query ... ok
test sdk::tools::wallet::tests::test_append_is_free ... ok
test sdk::tools::wallet::tests::test_deduct_and_credit ... ok
test sdk::tools::wallet::tests::test_genesis_allocation ... ok
test sdk::tools::wallet::tests::test_insufficient_balance_rejected ... ok
test sdk::tools::wallet::tests::test_negative_deduct_rejected ... ok
test sdk::tools::wallet::tests::test_portfolio_tracking ... ok
test sdk::tools::wallet::tests::test_no_double_genesis ... ok
test sdk::tools::wallet::tests::test_query_unknown_key ... ok
test sdk::tools::wallet::tests::test_query_balance ... ok
test sdk::tools::wallet::tests::test_unknown_agent_vetoed ... ok
test sdk::tools::wallet::tests::test_zero_deduct_rejected ... ok
test state::q_state::tests::btreemap_insertion_order_independent_serialization ... ok
test state::q_state::tests::economic_state_has_nine_sub_fields ... ok
test state::q_state::tests::genesis_q_state_is_total_and_default ... ok
test state::q_state::tests::node_id_from_state_root_is_deterministic ... ok
test state::q_state::tests::nine_field_count_via_serde_json ... ok
test top_white::predicates::registry::tests::agent_visible_view_filters_private ... ok
test top_white::predicates::registry::tests::empty_id_rejected ... ok
test top_white::predicates::registry::tests::duplicate_id_rejected ... ok
test top_white::predicates::registry::tests::empty_registry_root_is_sha256_empty ... ok
test top_white::predicates::registry::tests::merkle_root_changes_on_register ... ok
test top_white::predicates::registry::tests::metadata_canonical_hash_deterministic ... ok
test top_white::predicates::registry::tests::merkle_root_deterministic_two_runs ... ok
test top_white::predicates::registry::tests::metadata_canonical_hash_differs_on_visibility ... ok
test top_white::predicates::registry::tests::register_and_get_round_trip ... ok
test top_white::predicates::visibility::tests::commit_reveal_pre_reveal ... ok
test top_white::predicates::visibility::tests::private_never_content_visible ... ok
test top_white::predicates::visibility::tests::public_always_visible ... ok
test top_white::predicates::visibility::tests::serde_round_trip_public ... ok
test top_white::predicates::visibility::tests::serde_round_trip_commit_reveal ... ok
test wal::tests::test_wal_replay_missing_file_is_empty ... ok
test wal::tests::test_wal_roundtrip_nodes_only ... ok
test wal::tests::test_wal_skip_malformed_line ... ok
test wal::tests::test_wal_roundtrip_mixed ... ok
test boot::tests::verify_trust_root_passes_on_intact_repo ... ok
test sdk::sandbox::tests::test_sandbox_timeout_kills_process ... ok

test result: ok. 199 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.17s


codex
# Codex CO1.7 Round-3 Audit

## Q-1 R2-C3 closure
**CLOSED.** [system_keypair.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:188) now has `CanonicalMessage::LedgerEntrySigning([u8; 32])`; `canonical_digest()` adds `b"LedgerEntrySigning"` plus digest; `transition_ledger_emitter::sign_ledger_entry(...)` exists and wraps the digest through `CanonicalMessage`.

Test #9 in [transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:507) uses a real Ed25519 keypair, signs via the emitter, verifies clean, rejects `parent_ledger_root` mutation, and rejects epoch mutation.

## Q-2 R2-K3 closure
**CLOSED.** Spec §0/§3/§5 now defer `head_t` mutation to CO1.7.5+ and state v1.x owns `ledger_root_t` only. The sequencer pseudocode explicitly does not mutate `q_w.head_t`. `LedgerWriter::commit` still returns `Hash`.

## Q-3 R2-C2-CAS closure
**CLOSED.** Spec §3 uses `ObjectType::ProposalPayload`, with an R2 fix comment, not `ObjectType::Transition`. `ProposalPayload` exists in [schema.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/schema.rs:42).

## Q-4 R2-typo closure
**CLOSED.** Spec §0 says `LedgerEntrySigningPayload` is the **9-field** bytes-on-the-wire.

## Q-5 NEW v1.2 issues
**No new blocking issue.**

The opaque digest variant preserves the typed-sign-only invariant at the system-keypair layer: signatures still go through `CanonicalMessage` and the raw `sign_digest` path remains private. `pub(crate)` visibility is appropriate because the future sequencer lives outside `system_keypair` but still inside the crate. The cross-epoch test correctly models the D1 payload-binding threat by proving `epoch` changes the signed digest. TR manifest entries are refreshed for the spec, `system_keypair.rs`, `transition_ledger.rs`, and `mod.rs`.

## Q-6 Independent verification
`cargo test --lib bottom_white::ledger::transition_ledger::`: **9 passed / 0 failed**.

`cargo test --lib`: **199 passed / 0 failed**.

## **VERDICT**: PASS

## Top must-fix (if CHALLENGE)
None.

## Conviction
High.
2026-04-28T07:55:15.295537Z ERROR codex_core::session: failed to record rollout items: thread 019dd313-cb6a-7c02-8185-ab3d2d2d4d19 not found
tokens used
320,113
# Codex CO1.7 Round-3 Audit

## Q-1 R2-C3 closure
**CLOSED.** [system_keypair.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:188) now has `CanonicalMessage::LedgerEntrySigning([u8; 32])`; `canonical_digest()` adds `b"LedgerEntrySigning"` plus digest; `transition_ledger_emitter::sign_ledger_entry(...)` exists and wraps the digest through `CanonicalMessage`.

Test #9 in [transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:507) uses a real Ed25519 keypair, signs via the emitter, verifies clean, rejects `parent_ledger_root` mutation, and rejects epoch mutation.

## Q-2 R2-K3 closure
**CLOSED.** Spec §0/§3/§5 now defer `head_t` mutation to CO1.7.5+ and state v1.x owns `ledger_root_t` only. The sequencer pseudocode explicitly does not mutate `q_w.head_t`. `LedgerWriter::commit` still returns `Hash`.

## Q-3 R2-C2-CAS closure
**CLOSED.** Spec §3 uses `ObjectType::ProposalPayload`, with an R2 fix comment, not `ObjectType::Transition`. `ProposalPayload` exists in [schema.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/schema.rs:42).

## Q-4 R2-typo closure
**CLOSED.** Spec §0 says `LedgerEntrySigningPayload` is the **9-field** bytes-on-the-wire.

## Q-5 NEW v1.2 issues
**No new blocking issue.**

The opaque digest variant preserves the typed-sign-only invariant at the system-keypair layer: signatures still go through `CanonicalMessage` and the raw `sign_digest` path remains private. `pub(crate)` visibility is appropriate because the future sequencer lives outside `system_keypair` but still inside the crate. The cross-epoch test correctly models the D1 payload-binding threat by proving `epoch` changes the signed digest. TR manifest entries are refreshed for the spec, `system_keypair.rs`, `transition_ledger.rs`, and `mod.rs`.

## Q-6 Independent verification
`cargo test --lib bottom_white::ledger::transition_ledger::`: **9 passed / 0 failed**.

`cargo test --lib`: **199 passed / 0 failed**.

## **VERDICT**: PASS

## Top must-fix (if CHALLENGE)
None.

## Conviction
High.
