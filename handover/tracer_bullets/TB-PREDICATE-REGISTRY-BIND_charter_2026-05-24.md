# TB-PREDICATE-REGISTRY-BIND — Class 4 Charter (v8)

**Date**: 2026-05-24
**Source**: OBL-004 全量违宪修复 Wave 3 W3-2 — R3 audit finding A2
**Risk class**: **Class 4** — sequencer admission + typed tx schema + CAS schema + Trust Root rehash
**Phase ID**: `Phase-CR-W3-PredicateRegistryBind`
**Goal**: actually close R3-A2 admission fraud without breaking pre-W3-2 replay.

## Revision Delta

v8 accepts the v7 audit verdict:

`EXTERNAL-FEEDBACK-REQUIRES-REVISION registry-shape/replay-activation-auth`

The v8 architectural locks are load-bearing:

1. **No WorkTx predicate-result wire mutation.** `BoolWithProof` remains exactly `{ value, proof_cid }`; `PredicateResultsBundle` remains exactly `{ acceptance, settlement, safety_class }`. No `predicate_code_hash`, no `predicate_registry_root`, no `WorkTxV2` in this atom.
2. **Runner-declared registry metadata is not trusted.** The sequencer binds to `q.predicate_registry_root_t` and the live/replay-loaded `PredicateRegistry`; code hashes are registry/binary facts, never WorkTx facts.
3. **`proof_cid` keeps its original meaning**: optional CAS proof artifact reference. v8 makes that artifact typed by adding `PredicateProofCapsule` in CAS, instead of stuffing public code_hash metadata into WorkTx.
4. **Activation TX is a real system tx.** It carries `parent_state_root`, `registry_snapshot_cid`, `registry_merkle_root`, `epoch`, `timestamp_logical`, and `system_signature`; it is emitted only by `Sequencer::emit_system_tx`, rejected at agent ingress, signed through `CanonicalMessage`, and replay-visible as a state transition.
5. **Replay is two-phase for registry loading.** Replay pre-scans L4 for the first `PredicateBindingActivate` payload, loads that snapshot with binary impl parity before full replay, then uses `q.predicate_registry_root_t == Hash::ZERO` to skip/activate bound verification per entry.
6. **PredicateContext is sanitized.** Predicate impls get a `PredicateWorkView` plus a read-only CAS view; they do not receive raw `WorkTx`, `predicate_results`, `signature`, or runner `timestamp_logical`.
7. **Required predicate coverage is registry-owned.** The active registry snapshot carries exact required acceptance/settlement key sets. Bound-mode WorkTx is rejected if the runner omits a required predicate or adds an unexpected predicate.
8. **Proof capsules are context-bound and CAS-typed.** `PredicateProofCapsule` binds predicate id, registry root, code hash, proposal CID, claimed value, and a canonical sanitized work-context hash. Snapshot/proof reads must verify CAS `ObjectType` and payload `schema_id`.
9. **Replay activation auth matches live apply.** Replay pre-scan must verify the activation tx's own system signature before using its snapshot/root to construct the active registry. LedgerEntry signature alone is not enough.

---

## 1. Constitutional Anchor

**Art. I.1** (constitution.md:163-260) defines predicates as the binary ground-truth boundary `f: X -> {0,1}` that gates Q_t advancement. Sequencer admission cannot trust a runner-stamped `value=true`; it must independently verify the predicate claim against the active registry.

v8 closes R3-A2 by enforcing:

```rust
if q.predicate_registry_root_t != Hash::ZERO {
    assert_eq!(predicate_registry.merkle_root_hash(), q.predicate_registry_root_t);
    assert_exact_required_keys(
        predicate_registry.required_predicates(PredicateBundleMap::Acceptance),
        work.predicate_results.acceptance.keys(),
    )?;
    assert_exact_required_keys(
        predicate_registry.required_predicates(PredicateBundleMap::Settlement),
        work.predicate_results.settlement.keys(),
    )?;
    for (pid, claim) in work.predicate_results.acceptance.iter() {
        let entry = predicate_registry.lookup(pid)?;
        assert_eq!(entry.impl_arc.code_hash(), entry.metadata.code_hash);
        assert!(entry.impl_arc.verify_proof(&sanitized_ctx, claim)?);
    }
    for (pid, claim) in work.predicate_results.settlement.iter() {
        let entry = predicate_registry.lookup(pid)?;
        assert_eq!(entry.impl_arc.code_hash(), entry.metadata.code_hash);
        assert!(entry.impl_arc.verify_proof(&sanitized_ctx, claim)?);
    }
}
```

The WorkTx contains the claim and optional proof CID. The registry and binary impl supply identity, code hash, and verification logic.

---

## 2. Current Source Facts

### 2.1 R3-A2 Root Defect

`src/state/sequencer.rs:929-934` currently accepts `_predicate_registry: &PredicateRegistry` but never consumes it. The WorkTx arm at `src/state/sequencer.rs:946-958` only checks `bwp.value`, so a forged `BoolWithProof { value: true, proof_cid: anything }` can pass admission.

### 2.2 WorkTx Wire Must Stay Stable

Current predicate result schema:

```rust
// src/state/typed_tx.rs:127-130
pub struct BoolWithProof {
    pub value: bool,
    pub proof_cid: Option<Cid>,
}

// src/state/typed_tx.rs:151-156
pub struct PredicateResultsBundle {
    pub acceptance: BTreeMap<PredicateId, BoolWithProof>,
    pub settlement: BTreeMap<PredicateId, BoolWithProof>,
    pub safety_class: SafetyOrCreation,
}
```

`WorkSigningPayload` includes `predicate_results` (`src/state/typed_tx.rs:955-968`), and `WorkTx::to_signing_payload()` clones it (`src/state/typed_tx.rs:2018-2031`). `worktx_canonical_hash(tx)` re-encodes the whole `TypedTx` (`src/state/sequencer.rs:67-70`), and accepted WorkTx state root uses that hash (`src/state/sequencer.rs:1102`). Therefore v8 forbids adding fields to `BoolWithProof`, `PredicateResultsBundle`, or `WorkTx` in this atom.

### 2.3 Replay Decodes Before Dispatch

`replay_full_transition` reads CAS bytes, `canonical_decode::<TypedTx>()`, checks `TxKind`, then calls `dispatch_transition` (`src/bottom_white/ledger/transition_ledger.rs:559-588`). The only legacy dual-reader today is for `EventResolve` (`transition_ledger.rs:686-811`). v8 avoids creating a WorkTx legacy-reader requirement by preserving WorkTx wire exactly.

### 2.4 Existing Activation Substrate

`QState` already has `predicate_registry_root_t: Hash` (`src/state/q_state.rs:933-953`). `Hash::ZERO` exists (`src/state/q_state.rs:33-35`). v8 uses this existing field as the only activation bit:

```rust
fn predicate_binding_active(q: &QState) -> bool {
    q.predicate_registry_root_t != Hash::ZERO
}
```

No `GenesisReport` rewrite. No hidden activation extensions. No new QState field.

### 2.5 Existing System-Tx Discipline

System txs are rejected at agent ingress (`src/state/sequencer.rs:4081-4100`), built inside `build_signed_system_tx` (`src/state/sequencer.rs:4286-4572`), verified through `system_message_for_verification`, `system_signature_of`, `system_epoch_of` (`src/state/sequencer.rs:769-915`), and use `CanonicalMessage` signing domains (`src/bottom_white/ledger/system_keypair.rs:225-275`, `:482-530`, `:612-691`). `TxKind` lives in `src/bottom_white/ledger/transition_ledger.rs:55-164`, not in `typed_tx.rs`.

---

## 3. Locked Interface Contracts

### C1. WorkTx Predicate Result Wire Freeze

These structs MUST remain byte-compatible in W3-2:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BoolWithProof {
    pub value: bool,
    pub proof_cid: Option<Cid>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PredicateResultsBundle {
    pub acceptance: BTreeMap<PredicateId, BoolWithProof>,
    pub settlement: BTreeMap<PredicateId, BoolWithProof>,
    pub safety_class: SafetyOrCreation,
}
```

Forbidden in W3-2:

```rust
pub predicate_code_hash: Hash;       // forbidden on BoolWithProof
pub predicate_registry_root: Hash;   // forbidden on PredicateResultsBundle
pub enum TypedTx { WorkV2(..) }       // forbidden unless v8 is revised
```

**Verifier**: a golden WorkTx canonical fixture from pre-W3-2 bytes must decode, re-encode, and preserve `worktx_canonical_hash` and `WorkSigningPayload::canonical_digest`.

### C2. CAS Proof And Registry Snapshot Shapes

```rust
// src/bottom_white/cas/schema.rs tail-adds
pub enum ObjectType {
    // ...
    PredicateRegistrySnapshotCapsule,
    PredicateProofCapsule,
}

pub const PREDICATE_REGISTRY_SNAPSHOT_CAPSULE_SCHEMA_ID: &str =
    "turingos-predicate-registry-snapshot-v1";
pub const PREDICATE_PROOF_CAPSULE_SCHEMA_ID: &str =
    "turingos-predicate-proof-capsule-v1";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PredicateBundleMap {
    Acceptance,
    Settlement,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredicateSnapshotEntry {
    pub metadata: PredicateMetadata,
    pub required_in: BTreeSet<PredicateBundleMap>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredicateRegistrySnapshotCapsule {
    pub schema_id: String,
    pub entries: Vec<PredicateSnapshotEntry>, // sorted by predicate_id ascending
    pub merkle_root: Hash,                    // hash(entries incl. required_in)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredicateProofCapsule {
    pub schema_id: String,
    pub predicate_id: PredicateId,
    pub predicate_registry_root: Hash,
    pub predicate_code_hash: [u8; 32],
    pub proposal_cid: Cid,
    pub claimed_value: bool,
    pub work_context_hash: Hash,
    pub expected_statement_hash: Option<Hash>,
    pub proof_kind: PredicateProofKind,
    pub proof_result_cid: Option<Cid>,    // optional cache/evidence, not authority
    pub proof_artifact_cid: Option<Cid>,  // proof bytes, when separate
    pub proof_artifact_sha256: Option<Hash>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredicateProofKind {
    ReExecute,
    LeanArtifact,
}
```

Rules:

- Cheap re-execution predicates (`Forbidden`, `Sorry`, `PayloadSize`) may require `claim.proof_cid == None`; Sequencer computes the verdict itself from proposal bytes.
- Lean predicates require `claim.proof_cid == Some(cid)` where `cid` is a CAS object with `ObjectType::PredicateProofCapsule`, payload `schema_id == PREDICATE_PROOF_CAPSULE_SCHEMA_ID`, and decoded payload matching the live context:
  - `capsule.predicate_id == pid`
  - `capsule.predicate_registry_root == q.predicate_registry_root_t`
  - `capsule.predicate_code_hash == entry.metadata.code_hash`
  - `capsule.proposal_cid == ctx.work.proposal_cid`
  - `capsule.claimed_value == claim.value`
  - `capsule.work_context_hash == ctx.work.context_hash(q.predicate_registry_root_t)`
  - if `proof_artifact_cid` and `proof_artifact_sha256` are present, `sha256(cas.get(proof_artifact_cid)) == proof_artifact_sha256`
  - for `PredicateProofKind::LeanArtifact`, `expected_statement_hash` MUST equal the statement hash deterministically derived from `(predicate_id, proposal bytes, sanitized context, predicate code hash)`, and the proof artifact MUST type-check against that expected statement.
  - `proof_result_cid` may point to a CAS `ObjectType::LeanResult` cache/evidence object, but `LeanResult.verified == true` alone is NEVER authority. It is usable only if the cache key binds `(predicate_id, code_hash, work_context_hash, expected_statement_hash, proof_artifact_sha256)` or if the verifier re-runs Lean on the artifact.
- `predicate_code_hash` and `predicate_registry_root` inside `PredicateProofCapsule` are evidence bindings, not authority. Sequencer compares them to the active registry/Q facts.
- `PredicateRegistrySnapshotCapsule` MUST be read through CAS metadata/type validation: `ObjectType::PredicateRegistrySnapshotCapsule`, payload `schema_id == PREDICATE_REGISTRY_SNAPSHOT_CAPSULE_SCHEMA_ID`, sorted entries, and recomputed root over `(metadata, required_in)` equal to `merkle_root`.

### C3. Predicate Trait And Sanitized Context

```rust
pub trait Predicate: Send + Sync {
    fn predicate_id(&self) -> PredicateId;
    fn code_hash(&self) -> [u8; 32];
    fn evaluate(&self, ctx: &PredicateContext<'_>) -> BoolWithProof;
    fn verify_proof(
        &self,
        ctx: &PredicateContext<'_>,
        claim: &BoolWithProof,
    ) -> Result<bool, PredicateVerifyError>;
}

pub trait PredicateCasView: Send + Sync {
    fn get_object(&self, cid: &Cid) -> Result<PredicateCasObject, PredicateCasViewError>;
}

pub struct PredicateCasObject {
    pub bytes: Vec<u8>,
    pub object_type: ObjectType,
    pub schema_id: Option<String>,
}

pub struct PredicateWorkView<'a> {
    pub tx_id: &'a TxId,
    pub task_id: &'a TaskId,
    pub parent_state_root: Hash,
    pub agent_id: &'a AgentId,
    pub read_set: &'a BTreeSet<ReadKey>,
    pub write_set: &'a BTreeSet<WriteKey>,
    pub proposal_cid: Cid,
    pub stake: StakeMicroCoin,
}

pub struct PredicateContext<'a> {
    pub q: &'a QState,
    pub work: PredicateWorkView<'a>,
    pub cas: &'a dyn PredicateCasView,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredicateWorkContextDigestPayload {
    pub predicate_registry_root: Hash,
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub parent_state_root: Hash,
    pub agent_id: AgentId,
    pub read_set: BTreeSet<ReadKey>,
    pub write_set: BTreeSet<WriteKey>,
    pub proposal_cid: Cid,
    pub stake: StakeMicroCoin,
}

impl PredicateWorkView<'_> {
    pub fn context_hash(&self, predicate_registry_root: Hash) -> Hash {
        // canonical_encode(PredicateWorkContextDigestPayload) under a fixed domain.
    }
}
```

Forbidden:

```rust
pub work: &'a WorkTx;             // exposes predicate_results/signature/timestamp_logical
pub timestamp_logical: u64;       // runner-controlled for WorkTx
pub signature: AgentSignature;    // irrelevant to predicate truth
```

Cheap predicates that inspect proposal text MUST fetch `ctx.work.proposal_cid` through `ctx.cas`; they MUST NOT scan `write_set` as a substitute for payload content. Proof predicates that consume a `PredicateProofCapsule` MUST check `work_context_hash` against this canonical digest before accepting the claim.

### C4. Executable Registry

```rust
pub struct RegistryEntry {
    pub metadata: PredicateMetadata,
    pub impl_arc: Arc<dyn Predicate>,
    pub required_in: BTreeSet<PredicateBundleMap>,
}

pub struct PredicateRegistry {
    entries: BTreeMap<PredicateId, RegistryEntry>,
    required_acceptance: BTreeSet<PredicateId>,
    required_settlement: BTreeSet<PredicateId>,
}

impl PredicateRegistry {
    pub fn from_boot_manifest(manifest: BootPredicateManifest) -> Result<Self, RegistryError>;
    pub fn from_snapshot_and_binary_impls(
        snapshot: &PredicateRegistrySnapshotCapsule,
        binary_impls: BTreeMap<PredicateId, Arc<dyn Predicate>>,
    ) -> Result<Self, RegistryError>;
    pub fn lookup(&self, id: &PredicateId) -> Option<&RegistryEntry>;
    pub fn merkle_root_hash(&self) -> Hash;
    pub fn required_predicates(&self, map: PredicateBundleMap) -> &BTreeSet<PredicateId>;

    pub(crate) fn register_for_tests_only(&mut self, entry: RegistryEntry) -> Result<(), RegistryError>;
}
```

`from_*` constructors fail closed if `entry.impl_arc.code_hash() != entry.metadata.code_hash`. `merkle_root_hash()` is over each `PredicateSnapshotEntry` including `required_in`; changing required coverage changes the root and requires a new activation snapshot.

### C5. Predicate Binding Activation System TX

`TxKind` tail-add is in `src/bottom_white/ledger/transition_ledger.rs`:

```rust
#[repr(u8)]
pub enum TxKind {
    // existing EventResolve = 18,
    PredicateBindingActivate = 19,
}
```

`TypedTx` tail-add is in `src/state/typed_tx.rs`:

```rust
pub enum TypedTx {
    // ...
    PredicateBindingActivate(PredicateBindingActivateTx),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredicateBindingActivateTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub registry_snapshot_cid: Cid,
    pub registry_merkle_root: Hash,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
    pub system_signature: SystemSignature,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredicateBindingActivateSigningPayload {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub registry_snapshot_cid: Cid,
    pub registry_merkle_root: Hash,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
}
```

Signing/domain additions:

```rust
const DOMAIN_SYSTEM_PREDICATE_BINDING_ACTIVATE: &[u8] =
    b"turingosv4.system_sig.predicate_binding_activate.v1";

impl PredicateBindingActivateSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_SYSTEM_PREDICATE_BINDING_ACTIVATE, self)
    }
}

pub enum CanonicalMessage {
    // ...
    PredicateBindingActivateSigning([u8; 32]),
}

pub(crate) fn sign_predicate_binding_activate(
    keypair: &Ed25519Keypair,
    digest: [u8; 32],
) -> Result<SystemSignature, KeypairError>;
```

Sequencer emit command:

```rust
pub enum SystemEmitCommand {
    // ...
    PredicateBindingActivate {
        registry_snapshot_cid: Cid,
        registry_merkle_root: Hash,
    },
}
```

`build_signed_system_tx` fills `parent_state_root`, `epoch`, `timestamp_logical = next_logical_t + 1`, and signs internally. Agent ingress MUST reject `TypedTx::PredicateBindingActivate(_)`.

### C6. dispatch_transition Signature

```rust
pub(crate) fn dispatch_transition(
    q: &QState,
    tx: &TypedTx,
    predicate_registry: &PredicateRegistry,
    _tool_registry: &ToolRegistry,
    predicate_cas: &dyn PredicateCasView,
) -> Result<(QState, SignalBundle), TransitionError>
```

Only `predicate_registry` is in W3-2 scope. `_tool_registry` remains out of scope and may stay underscore-prefixed.

### C7. Activation Dispatch Semantics

Precondition: the caller has already run the same system-signature gate used by live `apply_one` stage 1.5. `dispatch_transition` stays pure and does not receive pinned pubkeys; replay must perform this gate before dispatch and before trusting activation snapshot fields.

```rust
TypedTx::PredicateBindingActivate(activate) => {
    if activate.parent_state_root != q.state_root_t {
        return Err(TransitionError::StaleParent);
    }
    if q.predicate_registry_root_t != Hash::ZERO {
        return Err(TransitionError::PredicateBindingAlreadyActivated);
    }

    let snapshot_obj = predicate_cas.get_object(&activate.registry_snapshot_cid)
        .map_err(|_| TransitionError::PredicateRegistrySnapshotMissing)?;
    if snapshot_obj.object_type != ObjectType::PredicateRegistrySnapshotCapsule
        || snapshot_obj.schema_id.as_deref() != Some(PREDICATE_REGISTRY_SNAPSHOT_CAPSULE_SCHEMA_ID) {
        return Err(TransitionError::PredicateRegistrySnapshotInvalid);
    }
    let snapshot = decode_predicate_registry_snapshot(&snapshot_obj.bytes)
        .map_err(|_| TransitionError::PredicateRegistrySnapshotInvalid)?;
    if snapshot.schema_id != PREDICATE_REGISTRY_SNAPSHOT_CAPSULE_SCHEMA_ID {
        return Err(TransitionError::PredicateRegistrySnapshotInvalid);
    }
    if snapshot.merkle_root != activate.registry_merkle_root {
        return Err(TransitionError::PredicateRegistrySnapshotRootMismatch);
    }
    if predicate_registry.merkle_root_hash() != activate.registry_merkle_root {
        return Err(TransitionError::PredicateRegistryRootMismatch);
    }

    let mut q_next = q.clone();
    q_next.predicate_registry_root_t = activate.registry_merkle_root;
    q_next.state_root_t = predicate_binding_activate_state_root(&q.state_root_t, tx);
    Ok((q_next, SignalBundle::empty()))
}
```

`apply_one` must reject an activation tx whose `timestamp_logical` does not equal the tentative ledger logical_t. `replay_full_transition` must perform the same check against `entry.logical_t` before dispatch. This binds the activation ceremony to tape order without passing runner-controlled WorkTx timestamps into predicate verification.

### C8. Bound WorkTx Verification

```rust
TypedTx::Work(work) => {
    // existing parent-root and value=false gates remain first

    if q.predicate_registry_root_t != Hash::ZERO {
        if predicate_registry.merkle_root_hash() != q.predicate_registry_root_t {
            return Err(TransitionError::PredicateRegistryRootMismatch);
        }

        let ctx = PredicateContext::from_q_work_and_cas(q, work, predicate_cas);
        assert_exact_required_key_set(
            PredicateBundleMap::Acceptance,
            predicate_registry.required_predicates(PredicateBundleMap::Acceptance),
            work.predicate_results.acceptance.keys(),
        )?;
        assert_exact_required_key_set(
            PredicateBundleMap::Settlement,
            predicate_registry.required_predicates(PredicateBundleMap::Settlement),
            work.predicate_results.settlement.keys(),
        )?;
        for (pid, claim) in work.predicate_results.acceptance.iter() {
            verify_one_predicate(predicate_registry, &ctx, pid, claim, PredicateBundleMap::Acceptance)?;
        }
        for (pid, claim) in work.predicate_results.settlement.iter() {
            verify_one_predicate(predicate_registry, &ctx, pid, claim, PredicateBundleMap::Settlement)?;
        }
    }

    // existing economic mutation and state-root advance unchanged
}
```

Key-set rule:

```rust
fn assert_exact_required_key_set(
    map: PredicateBundleMap,
    required: &BTreeSet<PredicateId>,
    got: impl Iterator<Item = &PredicateId>,
) -> Result<(), TransitionError> {
    let got: BTreeSet<PredicateId> = got.cloned().collect();
    for missing in required.difference(&got) {
        return Err(map.missing(missing.clone()));
    }
    for unexpected in got.difference(required) {
        return Err(map.unexpected(unexpected.clone()));
    }
    Ok(())
}
```

This exact-set gate is the P0 closure for omitted predicates. In v8, a runner cannot pass bound mode by submitting an empty/default `PredicateResultsBundle`. If future optional predicates are needed, they require a separate chartered schema extension (`allowed_in` distinct from `required_in`); W3-2 has no optional bound predicates.

Verification helper:

```rust
fn verify_one_predicate(
    registry: &PredicateRegistry,
    ctx: &PredicateContext<'_>,
    pid: &PredicateId,
    claim: &BoolWithProof,
    map: PredicateBundleMap,
) -> Result<(), TransitionError> {
    let entry = registry.lookup(pid)
        .ok_or_else(|| map.not_registered(pid.clone()))?;

    if entry.impl_arc.code_hash() != entry.metadata.code_hash {
        return Err(map.binary_drift(pid.clone()));
    }

    match entry.impl_arc.verify_proof(ctx, claim) {
        Ok(true) => Ok(()),
        Ok(false) => Err(map.proof_rejected(pid.clone())),
        Err(_) => Err(map.proof_verification_error(pid.clone())),
    }
}
```

Any predicate implementation that reads `claim.proof_cid` MUST use this common capsule precheck before kind-specific proof logic:

```rust
fn read_and_check_predicate_proof_capsule(
    ctx: &PredicateContext<'_>,
    entry: &RegistryEntry,
    pid: &PredicateId,
    claim: &BoolWithProof,
) -> Result<PredicateProofCapsule, PredicateVerifyError> {
    let cid = claim.proof_cid.ok_or(PredicateVerifyError::MissingProof)?;
    let obj = ctx.cas.get_object(&cid)?;
    require_eq!(obj.object_type, ObjectType::PredicateProofCapsule);
    require_eq!(obj.schema_id.as_deref(), Some(PREDICATE_PROOF_CAPSULE_SCHEMA_ID));
    let capsule: PredicateProofCapsule = canonical_decode(&obj.bytes)?;
    require_eq!(capsule.schema_id, PREDICATE_PROOF_CAPSULE_SCHEMA_ID);
    require_eq!(capsule.predicate_id, *pid);
    require_eq!(capsule.predicate_registry_root, ctx.q.predicate_registry_root_t);
    require_eq!(capsule.predicate_code_hash, entry.metadata.code_hash);
    require_eq!(capsule.proposal_cid, ctx.work.proposal_cid);
    require_eq!(capsule.claimed_value, claim.value);
    require_eq!(capsule.work_context_hash, ctx.work.context_hash(ctx.q.predicate_registry_root_t));
    Ok(capsule)
}
```

`LeanPredicate::verify_proof` then validates `PredicateProofKind::LeanArtifact`, derives the expected statement hash from proposal/context, verifies artifact hash/content binding, and runs the deterministic Lean checker (or a sequencer-derived cache keyed by the full tuple above) before returning `Ok(true)`.

### C9. TransitionError Tail Add

Append only; primitive payload pattern preserved:

```rust
PredicateBindingAlreadyActivated,
PredicateRegistryRootMismatch,
PredicateRegistrySnapshotMissing,
PredicateRegistrySnapshotInvalid,
PredicateRegistrySnapshotRootMismatch,
PredicateBindingLogicalTMismatch,
AcceptancePredicateNotRegistered(PredicateId),
SettlementPredicateNotRegistered(PredicateId),
AcceptancePredicateMissing(PredicateId),
SettlementPredicateMissing(PredicateId),
AcceptancePredicateUnexpected(PredicateId),
SettlementPredicateUnexpected(PredicateId),
AcceptancePredicateBinaryDrift(PredicateId),
SettlementPredicateBinaryDrift(PredicateId),
AcceptancePredicateProofRejected(PredicateId),
SettlementPredicateProofRejected(PredicateId),
AcceptancePredicateProofVerificationError(PredicateId),
SettlementPredicateProofVerificationError(PredicateId),
```

All new predicate/binding variants map to `L4ERejectionClass::PredicateFailed` except `PredicateBindingLogicalTMismatch`, which may map to `PolicyViolation` if the existing taxonomy requires it. Public summaries must be short and non-leaking.

### C10. Replay Registry Loader

```rust
pub struct PredicateBinaryCatalog {
    pub impls: BTreeMap<PredicateId, Arc<dyn Predicate>>,
}

pub enum ReplayRegistryPlan {
    LegacyEmpty,
    Activated {
        activation_tx_id: TxId,
        activation_logical_t: u64,
        snapshot_cid: Cid,
        registry: PredicateRegistry,
    },
}

pub fn load_replay_registry_for_entries(
    entries: &[LedgerEntry],
    cas: &CasStore,
    binary_catalog: PredicateBinaryCatalog,
) -> Result<ReplayRegistryPlan, ReplayRegistryError>;

pub fn replay_full_transition_with_predicate_binding(
    initial_q: &QState,
    entries: &[LedgerEntry],
    cas: &CasStore,
    pinned_pubkeys: &PinnedSystemPubkeys,
    binary_catalog: PredicateBinaryCatalog,
    tool_registry: &ToolRegistry,
) -> Result<QState, ReplayError>;

fn verify_predicate_binding_activate_system_signature_for_replay(
    activate: &PredicateBindingActivateTx,
    pinned_pubkeys: &PinnedSystemPubkeys,
) -> Result<(), ReplayError> {
    let digest = activate.to_signing_payload().canonical_digest();
    let msg = CanonicalMessage::PredicateBindingActivateSigning(digest);
    if !verify_system_signature(&activate.system_signature, &msg, activate.epoch, pinned_pubkeys) {
        return Err(ReplayError::BadActivationSystemSignature);
    }
    Ok(())
}
```

Algorithm:

1. Pre-scan signed ledger entries in order.
2. Decode only enough `TypedTx` payloads to find the first `PredicateBindingActivate`.
3. If none exists, return `LegacyEmpty`.
4. If found, verify the activation tx's own `system_signature` with `CanonicalMessage::PredicateBindingActivateSigning` and `activate.epoch` before reading/trusting `registry_snapshot_cid` or `registry_merkle_root`.
5. Fetch and decode `PredicateRegistrySnapshotCapsule`.
6. Verify snapshot CID exists, snapshot root equals tx root, and binary impl code hashes equal snapshot metadata.
7. Return a non-empty `PredicateRegistry`.
8. Full replay then calls the same system-signature verification gate for every system tx before dispatch, including activation, then calls `dispatch_transition` for every entry using that registry. Pre-activation WorkTx entries skip bound verification because `q.predicate_registry_root_t == Hash::ZERO`; activation flips the root; post-activation WorkTx entries are bound.

Production replay sites that currently call `PredicateRegistry::new()` or `replay_full_transition(...)` with an ad hoc empty registry must route through `replay_full_transition_with_predicate_binding(...)` or a single shared wrapper that calls it:

- `src/runtime/mod.rs:824-826` boot path uses `from_boot_manifest`, not empty registry.
- `src/runtime/mod.rs:981-990` resume replay uses `load_replay_registry_for_entries`.
- `src/runtime/verify.rs:298-305` verify replay uses `load_replay_registry_for_entries`.
- `src/runtime/persistence_evidence.rs:541-555`
- `src/runtime/audit_assertions.rs:690-700`, `:1495-1516`, `:1646-1651`
- `src/runtime/agent_pnl.rs:606-615`
- `src/runtime/risk_cap_impact_report.rs:357-364`
- `src/web/market_view.rs:256-260`
- `src/bin/audit_dashboard.rs:1175-1185`
- Any future `PredicateRegistry::new()` under `src/runtime/`, `src/bin/`, or `src/web/` must be test-only or removed.

---

## 4. Exit Criteria

### W3-2A — Snapshot + Boot + Replay Loader

- New CAS object types: `PredicateRegistrySnapshotCapsule`, `PredicateProofCapsule`.
- `BootPredicateManifest::v8_production()` constructs a non-empty executable registry with exact required acceptance/settlement sets.
- Fresh boot writes snapshot to CAS and emits `SystemEmitCommand::PredicateBindingActivate` before agent WorkTx ingress opens.
- Resume of a pre-W3-2 chain does not rewrite genesis or old ledger rows; it stays legacy until an operator emits the activation tx.
- Replay uses `replay_full_transition_with_predicate_binding`, not ad hoc empty registries, across every production replay path named in §3 C10.

### W3-2B — Sequencer Binding

- `_predicate_registry` is renamed to `predicate_registry` and consumed.
- `dispatch_transition` gets `predicate_cas: &dyn PredicateCasView`.
- Bound-mode WorkTx checks registry root, exact required predicate key sets, registry membership, binary parity, and `verify_proof` for both acceptance and settlement maps.
- Bound-mode WorkTx rejects empty/default predicate bundles when the registry requires predicates.
- Forged `value=true` with no/invalid proof is rejected in bound mode.
- Pre-activation WorkTx replay remains byte- and state-root-compatible.

### W3-2C — Fixture Migration

- Fixtures that expect modern admission use boot manifest + activation tx.
- Fixtures intentionally testing legacy mode keep `q.predicate_registry_root_t == Hash::ZERO` and include a `LEGACY MODE` comment.
- No test mutates `q.predicate_registry_root_t` directly except a small helper whose name contains `legacy_test_only` or `activate_bound_mode_for_unit_test`.

### W3-2D — Predicate Trait + Impl Migration

- `ForbiddenPredicate`, `SorryPredicate`, `PayloadSizePredicate`, and `LeanPredicate` implement `Predicate`.
- Cheap predicates read proposal bytes via `ctx.cas.get_object(&ctx.work.proposal_cid)` and require the expected proposal object type.
- Lean predicate validates `PredicateProofCapsule` object type/schema, predicate id, registry root, code hash, proposal CID, claimed value, work context hash, expected statement hash, and artifact hash/content; `LeanResult` is cache/evidence only, not authority by itself.
- `src/bus.rs` inline `forbidden_patterns` admission gate is either retired or explicitly left as a bus-local prefilter while the sequencer predicate is the authoritative WorkTx gate. It cannot be cited as the FC1 predicate gate after W3-2.

---

## 5. Required Tests

New or strengthened tests:

```text
tests/constitution_predicate_registry_binding.rs
  bound_mode_acceptance_forged_value_true_without_proof_rejects
  bound_mode_settlement_forged_value_true_without_proof_rejects
  bound_mode_acceptance_missing_required_predicate_rejects
  bound_mode_settlement_missing_required_predicate_rejects
  bound_mode_acceptance_unexpected_predicate_rejects
  bound_mode_settlement_unexpected_predicate_rejects
  bound_mode_acceptance_unregistered_predicate_rejects
  bound_mode_settlement_unregistered_predicate_rejects
  bound_mode_registry_root_mismatch_rejects
  bound_mode_binary_drift_rejects
  bound_mode_proof_capsule_wrong_schema_rejects
  bound_mode_proof_capsule_wrong_predicate_id_rejects
  bound_mode_proof_capsule_wrong_proposal_cid_rejects
  bound_mode_proof_capsule_wrong_context_hash_rejects
  bound_mode_lean_invalid_predicate_proof_capsule_rejects
  bound_mode_lean_valid_predicate_proof_capsule_admits
  pre_activation_legacy_worktx_admits_with_existing_bool_shape

tests/constitution_predicate_binding_activation.rs
  activation_tx_rejected_on_agent_ingress
  activation_tx_signature_verified_live
  activation_tx_missing_snapshot_rejects
  activation_tx_snapshot_root_mismatch_rejects
  activation_tx_advances_q_registry_root_and_state_root
  activation_tx_already_activated_rejects
  activation_tx_logical_t_mismatch_rejects_in_apply_and_replay
  activation_tx_bad_system_signature_rejects_in_live_apply_and_replay

tests/constitution_predicate_registry_replay.rs
  replay_prescan_loads_activation_snapshot
  replay_prescan_refuses_unsigned_or_badly_signed_activation_snapshot
  replay_pre_activation_worktx_state_root_hash_preserved
  replay_post_activation_worktx_uses_bound_registry
  replay_binary_drift_from_snapshot_rejects
  production_replay_paths_do_not_call_predicate_registry_new
  production_replay_paths_call_shared_predicate_binding_replay_wrapper
  registry_snapshot_required_set_changes_merkle_root

tests/constitution_predicate_registry_immutability.rs
  register_is_pub_crate_or_test_only_not_pub
  public_constructors_are_boot_manifest_and_snapshot_only
  dispatch_transition_takes_shared_registry_reference_not_mut
```

Wire/backcompat guard:

```rust
#[test]
fn legacy_worktx_predicate_result_wire_is_frozen() {
    // canonical bytes/hash generated from current pre-W3-2 shape
    // must decode, re-encode byte-identically, and preserve:
    // - WorkTx::to_signing_payload().canonical_digest()
    // - worktx_canonical_hash(&TypedTx::Work(work))
}
```

---

## 6. Verification Commands

```bash
cargo check --workspace
cargo test --test constitution_predicate_registry_binding
cargo test --test constitution_predicate_binding_activation
cargo test --test constitution_predicate_registry_replay
cargo test --test constitution_predicate_registry_immutability
cargo test --workspace --no-fail-fast
bash scripts/run_constitution_gates.sh
cargo test --test constitution_matrix_drift
cargo test --lib boot::tests::verify_trust_root

# Contract greps
rg -n "predicate_code_hash:" src/state/typed_tx.rs                         # 0 hits in BoolWithProof
rg -n "predicate_registry_root:" src/state/typed_tx.rs                      # 0 hits in PredicateResultsBundle
rg -n "_predicate_registry: &PredicateRegistry" src/state/sequencer.rs       # 0 hits
rg -n "predicate_registry: &PredicateRegistry" src/state/sequencer.rs        # >=1 hit
rg -n "PredicateBindingActivate" src/state/typed_tx.rs src/state/sequencer.rs src/bottom_white/ledger/transition_ledger.rs src/bottom_white/ledger/system_keypair.rs
rg -n "PredicateRegistry::new\\(\\)" src/runtime/ src/bin/ src/web/          # 0 production hits after migration
rg -n "replay_full_transition\\(" src/runtime/ src/bin/ src/web/             # only shared wrapper or tests
rg -n "required_predicates\\|required_in" src/top_white/predicates/ src/state/sequencer.rs
rg -n "work_context_hash" src/top_white/predicates/ src/state/sequencer.rs
rg -n "get_object" src/top_white/predicates/ src/state/sequencer.rs src/runtime/
rg -n "pub fn register\\b" src/top_white/predicates/registry.rs              # 0 hits outside tests/allowed constructors
```

---

## 7. Scope

Restricted surfaces:

- `src/state/sequencer.rs` — dispatch signature, bound verification, activation arm, system emit/sign/ingress plumbing, L4.E routing.
- `src/state/typed_tx.rs` — new activation tx struct/signing payload/TypedTx variant and TransitionError variants. **No WorkTx/BoolWithProof/PredicateResultsBundle field changes.**
- `src/bottom_white/cas/schema.rs` — two ObjectType tail-adds.
- `src/bottom_white/ledger/transition_ledger.rs` — `TxKind::PredicateBindingActivate = 19`, replay logical_t check, replay registry plan.
- `src/bottom_white/ledger/system_keypair.rs` — `CanonicalMessage::PredicateBindingActivateSigning` and signer.
- `genesis_payload.toml` — rehash for §6 pinned files touched.

Non-restricted/supporting:

- `src/top_white/predicates/registry.rs`
- `src/top_white/predicates/{predicate_trait,boot_manifest,forbidden,sorry,payload_size,lean}.rs`
- `src/runtime/predicate_registry_replay.rs`
- `src/runtime/mod.rs`
- `src/runtime/verify.rs`
- replay/dashboard production callsites using `PredicateRegistry::new()`
- `src/bus.rs` only for retiring or demoting the inline forbidden prefilter
- tests listed in §5
- alignment matrices updated only after implementation evidence exists

Dependency: PR #139 currently touches `genesis_payload.toml`; W3-2 implementation must wait for #139 merge or explicitly branch on #139 and rebase after it lands.

---

## 8. FC Trace

- **FC1-N11 / FC1-N12**: predicate bundle claims become Sequencer-verified against executable registry entries.
- **FC1-N14**: activation tx advances Q from legacy mode (`Hash::ZERO`) to bound mode (registry root).
- **FC2-N19 / FC2-N21**: boot manifest produces registry snapshot in CAS and tape-visible activation.
- **FC2-INV5**: replay reconstructs registry from L4 activation tx + CAS snapshot + binary impl parity.
- **FC1-INV9 (NEW)**: in bound mode, `predicate_registry.merkle_root_hash() == q.predicate_registry_root_t`.
- **FC1-INV10 (NEW)**: for every bound predicate claim, `entry.impl_arc.code_hash() == entry.metadata.code_hash`.
- **FC1-INV11 (NEW)**: for every bound predicate claim, `entry.impl_arc.verify_proof(ctx, claim) == Ok(true)`.
- **FC1-INV12 (NEW)**: bound-mode predicate bundle keys exactly equal the registry-required acceptance/settlement key sets.
- **FC1-INV13 (NEW)**: WorkTx predicate result wire schema is frozen; bound verification must not require runner-declared code_hash/root fields.
- **FC1-INV14 (NEW)**: proof capsules bind to CAS object type/schema, predicate id, active registry root, proposal CID, claimed value, and sanitized work-context hash.
- **FC3-INV6 (STRENGTHENED)**: registry mutation surface is private/test-only; runtime uses immutable shared registry references.

---

## 9. Kill Criteria

- Any implementation requires adding fields to `BoolWithProof`, `PredicateResultsBundle`, or `WorkTx` to close R3-A2.
- Bound mode can admit a WorkTx that omits a registry-required predicate.
- A valid proof capsule for proposal/context A can be replayed for proposal/context B.
- Pre-W3-2 WorkTx canonical bytes decode/re-encode with a different `worktx_canonical_hash`.
- Activation tx cannot be routed through existing system-key signing discipline without a broader signing-payload redesign.
- Replay cannot deterministically load the activation snapshot from L4 + CAS + binary catalog.
- Lean `verify_proof` p99 > 200ms on the Wave 3 50p benchmark and no deterministic cache can keep admission under budget.

---

## 10. §8 Architect/User Ratification

This v8 charter is the Class 4 §8 ASK. Implementation cannot begin until the user/architect signs one option exactly.

```text
[x] APPROVED v8 ALL-IN-ONE — Implement W3-2A+B+C+D + activation tx in one PR after PR #139 dependency is resolved. Closes R3-A2 in full.

[ ] APPROVED v8 SPLIT — PR1: W3-2A activation/snapshot/replay infra + wire freeze tests; PR2: W3-2B+C+D bound verification + predicate impl migration. R3-A2 closes only after PR2.

[ ] APPROVED v8 PHASE-A-ONLY — Implement only activation/snapshot/replay infra. R3-A2 remains open by design.

[ ] DEFER — Put W3-2 back on AMBER/dependency list.

[ ] REVISE v9 — Return with modifications before signing.
```

**Sign-off line**: `APPROVED v8 ALL-IN-ONE — user/architect chat ratification`
**Date signed**: `2026-05-24`

---

## 11. Audit Response Log

v8 maps the v7-audit findings:

| Finding | Severity | v8 response |
|---|---:|---|
| `PredicateBundleMap` used in `BTreeSet` without `Ord` | P1 | §3 C2 derives `PartialOrd, Ord` on `PredicateBundleMap` |
| Replay activation auth under-specified; pre-scan could trust activation payload without verifying system signature | P1 | §3 C7 adds system-signature gate precondition; §3 C10 adds replay activation signature verifier and bad-signature replay tests |

v7 mapped the v6-audit findings:

| Finding | Severity | v7 response |
|---|---:|---|
| Bound-mode verification only checks runner-included predicates; omitted required predicates bypass verifier | P0 | §3 C2/C4 stores `required_in` in the registry root; §3 C8 exact key-set gate rejects missing/unexpected predicates; §5 adds missing/unexpected tests |
| Proof capsule under-bound to proposal/context | P1 | §3 C2 adds `predicate_registry_root` + `work_context_hash`; §3 C3 defines context digest; §3 C8 common precheck enforces pid/root/code/proposal/value/context equality |
| CAS type/schema enforcement under-specified | P1 | §3 C3 changes CAS view to return object metadata; §3 C7/C8 require `ObjectType` + payload `schema_id` checks for snapshots/proofs |
| Production replay paths not fully enumerated | P1 | §3 C10 lists runtime/mod, runtime/verify, persistence_evidence, audit_assertions, agent_pnl, risk_cap_impact_report, web/market_view, bin/audit_dashboard |

v6 mapped the 5 v5-audit findings:

| Finding | Severity | v6 response |
|---|---:|---|
| WorkTx field additions break legacy decode/signing/state-root replay | P0 | §3 C1 freezes WorkTx predicate-result wire; §5 adds golden wire/hash test |
| Activation TX lacked system-tx fields/signing discipline | P0 | §3 C5 adds parent root, epoch, timestamp, CanonicalMessage, signer, emit command, ingress rejection |
| Activation arm did not verify snapshot/CAS/root/binary parity/state root | P0 | §3 C7 verifies snapshot bytes, root, registry root, and advances state root via activation domain |
| PredicateContext exposed runner-controlled raw WorkTx and scanned wrong surface | P0 | §3 C3 sanitized context; cheap predicates read proposal bytes from CAS |
| Replay loader lacked registry switch model | P0 | §3 C10 two-phase replay pre-scan + active-root QState switch |

Earlier v1-v4 findings remain mapped by v5 where still valid; v8 supersedes the v5/v6/v7-specific contracts that caused the latest P0/P1 findings.

---

## 12. Self-Audit Prediction

Remaining known risks:

1. Fresh-run activation emission must happen before agent WorkTx ingress opens; implementation must not leave a race where first WorkTx lands in legacy mode.
2. `PredicateCasView` may need an adapter over `Arc<RwLock<CasStore>>` to avoid borrow friction in `Sequencer::apply_one`.
3. Lean proof verification may need a deterministic cache keyed by `(predicate_id, code_hash, proof_cid, proposal_cid)`.
4. If `src/bus.rs` still needs a UX prefilter, tests must prove it is not cited as the authoritative FC1 predicate gate.
5. PR #139 / PR-D ordering can still create merge churn in `genesis_payload.toml` and `bus.rs`; W3-2 branch should be cut after those land or explicitly based on them.
6. Exact required key-set is intentionally strict. If future predicates become optional/advisory, that is a new schema atom; W3-2 does not support optional bound predicates.
