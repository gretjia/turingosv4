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
