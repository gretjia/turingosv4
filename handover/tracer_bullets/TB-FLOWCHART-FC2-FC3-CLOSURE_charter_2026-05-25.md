# TB-FLOWCHART-FC3-CLOSURE — Class 4 Charter v2

Date: 2026-05-25
Risk class: Class 4
Status: DRAFT — FC3-only follow-up, not yet §8-ratified
Source of truth: `constitution.md` only
User obligation: OBL-005
Baseline: `origin/main` after PR #142 FC2 tick and PR #143 FC1 rtool/input

This v2 supersedes the v1 FC2+FC3 combined draft. FC2 map-reduce tick is now
LIVE on main through `MapReduceTickTx`, boot emission, and replay prefix
verification. The only remaining constitutional flowchart liveness blockers
are FC3:

- `logs -> feedback -> architectAI`
- `init -> error -> re-init -> boot`

This charter does not change `constitution.md`, does not change canonical
flowchart hashes, does not import old session practices as authority, and does
not claim that REAL-5 role scaffolding, Markov capsules, handover docs, or
external audit practice already satisfy FC3.

## 1. Constitutional Anchors

FC3 authority is `constitution.md:826-870`:

- `constitution` and `logs` are read-only ground truth for the init/meta layer.
- `tools ==>|write| log` and `log ====>|archive| logs`.
- `logs -->|feedback| architectAI`.
- `init ==> error ==========>|re-init| boot`.
- `constitution -->|abide| vetoAI & architectAI`.
- `vetoAI -->|veto| architectAI`.

Derived matrices, old extracted FC element files, dashboards, handover docs,
and historical trace files are evidence or audit aids only. They are not FC3
topology authority.

## 2. Current Production Reality

The current mainline has strong substrate:

- L4 accepted ChainTape (`LedgerEntry`)
- L4.E rejection evidence
- CAS content-addressed payload and metadata index
- `verify_chaintape` / `replay_full_transition_with_predicate_binding`
- system-only tx emission and signing
- agent ingress rejection for system txs
- FC2 boot-visible `PredicateBindingActivate + MapReduceTick`

The current mainline does not have FC3 typed transitions:

- no `LogFeedbackArchiveTx` / `MetaFeedbackTx` / equivalent typed tx
- no `ReinitRequestTx` / `ReinitBootTx`
- no `TxKind` discriminants after `MapReduceTick = 20`
- no `SystemEmitCommand` for FC3 feedback or re-init
- no replay validation of FC3 feedback/re-init prefix roots
- ignored tests still reserve ArchitectAI/Veto-AI runtime and in-process
  re-init as deferred placeholders

REAL-5 `AgentRole::Architect` and `AgentRole::Veto` are useful product
workload scaffolding. They are not FC3 constitutional closure until their
outputs are bound to typed ChainTape/CAS facts under this charter.

## 3. Closure Strategy

The closure is tape-first and minimal:

- FC3 facts are system-only typed L4 transactions.
- Agent ingress must reject every FC3 system tx before queueing.
- Live apply and replay must recompute the same prefix roots and CAS bindings.
- `dispatch_transition` stays pure: no I/O, wall-clock, env, RNG, or writer
  side effects.
- Ordinary agent read views must not receive raw logs, raw diagnostics, raw
  autopsy detail, or deep history because FC3 feedback exists.
- ArchitectAI feedback is proposal/input evidence, not direct authority to
  mutate constitution, QState, predicate registry, or tools.

The atom should add exactly the typed surfaces needed for two edges:

1. **FC3-FEEDBACK** — log archive feedback becomes a typed, replayable input
   to ArchitectAI.
2. **FC3-REINIT** — error-triggered re-init request and boot acknowledgement
   become typed, replayable facts.

If implementation needs a new `QState` top-level field or a new CAS
`ObjectType`, stop and revise this charter before editing those surfaces.

## 4. Locked Interface Contract

Field order may be adjusted only by a charter revision before §8 sign-off.
Existing typed-tx discriminants and signing payloads must not be reordered.

### 4.1 FC3 Feedback

```rust
pub enum TypedTx {
    // Existing variants unchanged.
    LogFeedbackArchive(LogFeedbackArchiveTx),
}

#[repr(u8)]
pub enum TxKind {
    // Existing discriminants unchanged.
    LogFeedbackArchive = 21,
}

#[repr(u8)]
pub enum MetaRoleMode {
    ExternalOnly = 0,
    Runtime = 1,
}

#[repr(u8)]
pub enum VetoVerdict {
    Pass = 0,
    Veto = 1,
}

pub struct LogFeedbackArchiveTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub source_ledger_root: Hash,
    pub source_l4_len: u64,
    pub source_l4e_root: Hash,
    pub source_l4e_len: u64,
    pub cas_metadata_root: Hash,
    pub constitution_hash: Hash,
    pub feedback_capsule_cid: Cid,
    pub feedback_root: Hash,
    pub previous_feedback_cid: Option<Cid>,
    pub role_mode: MetaRoleMode,
    pub veto_verdict: VetoVerdict,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
    pub system_signature: SystemSignature,
}

pub struct LogFeedbackArchiveSigningPayload {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub source_ledger_root: Hash,
    pub source_l4_len: u64,
    pub source_l4e_root: Hash,
    pub source_l4e_len: u64,
    pub cas_metadata_root: Hash,
    pub constitution_hash: Hash,
    pub feedback_capsule_cid: Cid,
    pub feedback_root: Hash,
    pub previous_feedback_cid: Option<Cid>,
    pub role_mode: MetaRoleMode,
    pub veto_verdict: VetoVerdict,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
}

pub struct ArchitectFeedbackCapsule {
    pub schema_version: String, // exactly "fc3.architect_feedback.v1"
    pub source_ledger_root: Hash,
    pub source_l4e_root: Hash,
    pub cas_metadata_root: Hash,
    pub constitution_hash: Hash,
    pub public_summary: String,
    pub private_detail_cid: Option<Cid>,
}
```

Rules:

- The capsule is stored as `ObjectType::Generic` with
  `schema_id = "fc3.architect_feedback.v1"`.
- The implementation must not add a new CAS `ObjectType` unless this charter
  is revised.
- `feedback_root` is recomputed from the canonical capsule bytes and the
  pre-candidate L4/L4.E/CAS/constitution roots.
- The prefix boundary excludes the candidate `LogFeedbackArchiveTx` payload
  itself.
- `role_mode = ExternalOnly` is allowed for this atom, but it must be honest:
  it means the tx makes the FC3 edge tape-visible while the actual language
  model role remains outside the Rust runtime.

### 4.2 FC3 Re-init

```rust
pub enum TypedTx {
    // Existing variants unchanged.
    ReinitRequest(ReinitRequestTx),
    ReinitBoot(ReinitBootTx),
}

#[repr(u8)]
pub enum TxKind {
    // Existing discriminants unchanged.
    ReinitRequest = 22,
    ReinitBoot = 23,
}

#[repr(u8)]
pub enum ReinitReason {
    TerminalErrorHalt = 0,
    ReplayFailure = 1,
    RuntimeInvariantViolation = 2,
}

pub struct BootProfileId(pub String);

pub struct ReinitRequestTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub trigger_entry: u64,
    pub error_evidence_cid: Cid,
    pub reason: ReinitReason,
    pub source_ledger_root: Hash,
    pub source_l4_len: u64,
    pub source_l4e_root: Hash,
    pub source_l4e_len: u64,
    pub cas_metadata_root: Hash,
    pub target_boot_profile: BootProfileId,
    pub role_mode: MetaRoleMode,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
    pub system_signature: SystemSignature,
}

pub struct ReinitBootTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub request_tx_id: TxId,
    pub replayed_state_root: Hash,
    pub boot_profile: BootProfileId,
    pub role_mode: MetaRoleMode,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
    pub system_signature: SystemSignature,
}

pub struct ReinitReasonCapsule {
    pub schema_version: String, // exactly "fc3.reinit_reason.v1"
    pub trigger_entry: u64,
    pub reason: ReinitReason,
    pub public_summary: String,
    pub private_detail_cid: Option<Cid>,
}
```

Rules:

- `ReinitReasonCapsule` is stored as `ObjectType::Generic` with
  `schema_id = "fc3.reinit_reason.v1"`.
- `trigger_entry` initially points to an accepted
  `TerminalSummaryTx { run_outcome: ErrorHalt }`.
- Pre-trust-root boot failure is out of scope: if the constitution/trust-root
  cannot be verified before the sequencer exists, this atom must fail closed
  rather than invent tape.
- `ReinitBootTx.request_tx_id` must point to a prior accepted
  `ReinitRequestTx`.
- `ReinitBootTx.replayed_state_root` must be recomputed by live code and
  replay. It must not be trusted from caller input.
- Re-init never rewrites old genesis, old L4, old L4.E, old CAS, or old
  trust-root history.

### 4.3 System Emit Contract

```rust
pub enum SystemEmitCommand {
    LogFeedbackArchive {
        feedback_capsule_cid: Cid,
        role_mode: MetaRoleMode,
        veto_verdict: VetoVerdict,
    },
    ReinitRequest {
        trigger_entry: u64,
        error_evidence_cid: Cid,
        reason: ReinitReason,
        target_boot_profile: BootProfileId,
        role_mode: MetaRoleMode,
    },
    ReinitBoot {
        request_tx_id: TxId,
        boot_profile: BootProfileId,
        role_mode: MetaRoleMode,
    },
}
```

The sequencer derives all roots, lengths, `parent_state_root`, logical time,
epoch, replayed root, and signatures internally.

## 5. Required Implementation Touchpoints

Restricted/Class 4 surfaces:

- `src/state/typed_tx.rs`
- `src/state/sequencer.rs`
- `src/bottom_white/ledger/transition_ledger.rs`
- `src/bottom_white/ledger/system_keypair.rs`

Non-restricted or lower-risk surfaces likely touched:

- `src/runtime/mod.rs` or narrow runtime helpers
- `src/runtime/verify.rs`
- `tests/constitution_fc3_closure.rs`
- `tests/constitution_flowchart_livenow.rs`
- `tests/constitution_flowchart_source_alignment.rs`
- `tests/fc_alignment_conformance.rs`
- liveness/audit docs and obligation ledger

Implementation must add:

- tail `TxKind` discriminants 21, 22, 23 only
- `TypedTx` variants and `tx_kind()` projections
- signing payloads and canonical digest tests
- `CanonicalMessage` variants and system signer helpers
- `TransitionError` variants for invalid capsule/schema, prefix mismatch,
  logical-time mismatch, request lookup failure, and replayed-root mismatch
- `submit_agent_tx` forbidden-system-tx coverage
- `SystemEmitCommand` construction and signature verification
- live `apply_one` prefix/CAS verification
- replay verification using the same prefix/CAS formulas

## 6. Required Tests

Add active, non-ignored gates. Ignored tests and grep-only tests are not closure
evidence.

Minimum FC3 closure tests:

```bash
cargo test --test constitution_fc3_closure
cargo test --test constitution_flowchart_livenow
cargo test --test constitution_flowchart_source_alignment
cargo test --test fc_alignment_conformance
cargo test --test constitution_matrix_drift
bash scripts/run_constitution_gates.sh
cargo test --workspace --no-fail-fast
```

Required scenarios:

- `fc3_logs_feedback_to_architect_ai_is_tape_cas_bound`
- `fc3_meta_feedback_replay_recomputes_source_log_root`
- `fc3_agent_ingress_rejects_meta_txs`
- `fc3_architect_feedback_is_not_plain_handover_or_latest`
- `fc3_architect_feedback_read_view_is_shielded`
- `fc3_error_reinit_request_links_errorhalt_to_next_boot`
- `fc3_reinit_boot_recomputes_replayed_state_root`
- `fc3_reinit_no_rewrite_old_evidence`

## 7. Kill Criteria

Abort and revise if any of these become necessary:

- using `handover/ai-direct/LATEST.md`, dashboards, old trace matrices,
  stdout, wall clock, env, RNG, or old sessions as canonical runtime input
- implementing FC3 feedback as CAS-only without an accepted L4 typed tx
- accepting agent-submitted FC3 meta txs
- exposing raw logs/private diagnostics to ordinary agent read views
- letting ArchitectAI txs directly mutate constitution, QState, predicate
  registry, tools, or trust roots
- changing existing tx discriminants or old signing payloads
- adding a `QState` top-level field without a revised §8
- adding a CAS `ObjectType` without a revised §8
- claiming pre-trust-root boot failure is on tape
- rewriting old genesis, L4, L4.E, CAS, or trust-root history

## 8. Old Session Practice Ingestion Rule

The `codex://threads/019e55e5-e059-7013-b8a9-14eab5824d81` real-world testing
practice may be used later as product workload input only after the kernel
closure is live. It is not imported into this atom as a source of truth.

Any useful practice from that thread must pass through the same filter:

1. constitutional node or extra-module classification
2. ChainTape/CAS/replay evidence path
3. no reliance on a pre-closure broken kernel behavior
4. no global latest pointer or derived-file authority

## 9. §8 Ratification Options

The previous user selection `APPROVED-SPLIT-FC2-FIRST` is fully consumed by
PR #142. It does not authorize this FC3 atom.

The user/architect must sign exactly one option before implementation:

```text
[ ] APPROVED-FC3-v2-ALL-IN-ONE — implement FC3-FEEDBACK + FC3-REINIT in one Class 4 PR.
[ ] APPROVED-FC3-v2-FEEDBACK-FIRST — implement LogFeedbackArchive first; re-init remains blocked.
[ ] APPROVED-FC3-v2-REINIT-FIRST — implement ReinitRequest/ReinitBoot first; feedback remains blocked.
[ ] DEFER — keep OBL-005 blocked; no FC3 implementation.
[ ] REVISE-v3 — return with modifications before §8.
```

Recommended option: `APPROVED-FC3-v2-ALL-IN-ONE`.

Rationale: FC3 feedback and re-init share the same roots, CAS capsule checks,
system-only signing, and replay validation pattern. Splitting them creates two
rounds of restricted-surface churn while leaving OBL-005 blocked between PRs.

Sign-off line:

```text
Option selected: ______________________________
Signed by: ____________________________________
Date signed: __________________________________
```

## 10. Read-Only Research Witnesses

Three independent read-only researchers checked this revision direction:

- typed-tx/sequencer/signing researcher: FC3 needs tail typed system txs and
  must not add QState/CAS schema without revised §8
- runtime/replay/CAS researcher: existing substrate is reusable, but no
  FC3 typed transition exists today
- gate/docs researcher: ignored FC3 tests and Markov/deep-history support
  invariants cannot count as FC3 closure

All three used `constitution.md:826-870` as authority and treated derived
alignment files as evidence only.
