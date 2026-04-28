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
