//! L1 Predicate Registry — typed metadata store per WP § 5.L1 + spec v1.4 § 1.5.
//!
//! Constitution authority:
//! - Inv 6 (predicate-gated transition): un-passed work_tx does NOT advance state
//! - Inv 10 (signal vs evaluator): private/commit-reveal predicates SHIELDED from agent view
//! - Const Art III.4: Goodhart shield via three visibility classes
//!
//! Spec authority:
//! - STATE_TRANSITION_SPEC v1.4 § 4 invariants I-PRED-GATE + I-NORANDOM bound to this registry
//! - § 2 hidden inputs: BTreeMap (not HashMap) for deterministic iteration order
//!
//! v4 first iteration: typed metadata + register/lookup + Merkle root computation.
//! Predicate EXECUTION (running the actual predicate code on a work_tx) lives in `runner` (future atom CO1.5.6).
//!
//! /// TRACE_MATRIX WP-arch-§5.L1 + Inv-6 + Inv-10: PredicateRegistry

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::sync::Arc;
use std::time::Duration;

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
use crate::economy::money::StakeMicroCoin;
use crate::sdk::sanitized_runner::{env_allowlist_from_current, run_sanitized, SanitizedCommand};
use crate::state::q_state::{AgentId, Hash, TaskId, TxId};
use crate::state::typed_tx::{BoolWithProof, PredicateId, ReadKey, WorkTx, WriteKey};

use super::visibility::Visibility;

/// Whether a predicate is fail-closed (Safety) or fail-open-with-signal (Creation).
/// Per WP § 7.2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafetyOrCreation {
    /// Fail-closed: rejected work_tx does NOT advance state_root.
    Safety,
    /// Fail-open-with-signal: rejected work_tx still produces a signal but does not advance state.
    /// (In v4, both behave identically at the state-transition level; difference matters at signal layer.)
    Creation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredicateMetadata {
    /// Stable identifier; e.g., "lean4_oracle".
    pub predicate_id: String,
    /// Schema/code-hash version.
    pub version: u32,
    /// SHA-256 of compiled bytecode or canonical source.
    pub code_hash: [u8; 32],
    /// JSON Schema (or type ID) describing input shape.
    pub input_schema: String,
    /// JSON Schema describing output shape.
    pub output_schema: String,
    /// Goodhart visibility class.
    pub visibility: Visibility,
    /// Owner (agent_id or "system").
    pub owner: String,
    /// SHA-256 of conformance test suite committed alongside.
    pub test_suite_hash: [u8; 32],
    /// Fail-closed (Safety) or fail-open-with-signal (Creation).
    pub safety_class: SafetyOrCreation,
}

impl PredicateMetadata {
    /// Canonical hash of this metadata for Merkle tree inclusion.
    /// Bincode-style; mirrors STATE_TRANSITION_SPEC § 2.5 canonical serialization rule
    /// (BTreeMap key order is irrelevant here since fields are fixed-order in struct).
    pub fn canonical_hash(&self) -> [u8; 32] {
        // Manual canonical serialization for v1; matches spec § 2.5 deterministic format.
        // (Avoiding bincode dep in lib for now; upgrade later if v1.4 conformance test demands.)
        let mut h = Sha256::new();
        h.update(self.predicate_id.as_bytes());
        h.update(self.version.to_be_bytes());
        h.update(self.code_hash);
        h.update(self.input_schema.as_bytes());
        h.update(self.output_schema.as_bytes());
        h.update(serde_json::to_vec(&self.visibility).expect("visibility serialize"));
        h.update(self.owner.as_bytes());
        h.update(self.test_suite_hash);
        h.update(serde_json::to_vec(&self.safety_class).expect("safety_class serialize"));
        h.finalize().into()
    }
}

/// TRACE_MATRIX FC1-N11 + FC1-N12: predicate-result map identity used by bound admission for acceptance and settlement bundles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PredicateBundleMap {
    Acceptance,
    Settlement,
}

/// TRACE_MATRIX FC1-N12 + FC2-N19: snapshot row binding predicate metadata to the bundle maps where it is required.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredicateSnapshotEntry {
    pub metadata: PredicateMetadata,
    pub required_in: BTreeSet<PredicateBundleMap>,
}

/// TRACE_MATRIX FC1-N11 + FC1-N12 + FC2-N19: CAS capsule for replayable predicate registry snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredicateRegistrySnapshotCapsule {
    pub schema_id: String,
    pub entries: Vec<PredicateSnapshotEntry>,
    pub merkle_root: Hash,
}

impl PredicateRegistrySnapshotCapsule {
    /// TRACE_MATRIX FC1-N11 + FC2-N19: schema id for predicate registry snapshot CAS objects.
    pub const SCHEMA_ID: &'static str = "turingos-predicate-registry-snapshot-v1";
}

/// TRACE_MATRIX FC1-N12: proof verification mode declared by each executable predicate implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredicateProofKind {
    ReExecute,
    LeanArtifact,
}

/// TRACE_MATRIX FC1-N11 + FC1-N12: CAS-resident proof artifact envelope verified by predicate implementations.
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
    pub proof_result_cid: Option<Cid>,
    pub proof_artifact_cid: Option<Cid>,
    pub proof_artifact_sha256: Option<Hash>,
}

impl PredicateProofCapsule {
    /// TRACE_MATRIX FC1-N11 + FC1-N12: schema id for predicate proof capsule CAS objects.
    pub const SCHEMA_ID: &'static str = "turingos-predicate-proof-capsule-v1";
}

/// TRACE_MATRIX FC1-N11 + FC2-N19: typed CAS read result used by predicate proof verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateCasObject {
    pub bytes: Vec<u8>,
    pub object_type: ObjectType,
    pub schema_id: Option<String>,
}

/// TRACE_MATRIX FC1-N11 + FC2-N19: deterministic error surface for predicate proof CAS reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredicateCasViewError {
    Missing(Cid),
    Backend(String),
}

impl fmt::Display for PredicateCasViewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(cid) => write!(f, "predicate CAS object missing: {cid}"),
            Self::Backend(msg) => write!(f, "predicate CAS backend error: {msg}"),
        }
    }
}

impl std::error::Error for PredicateCasViewError {}

impl From<CasError> for PredicateCasViewError {
    fn from(value: CasError) -> Self {
        match value {
            CasError::CidNotFound(cid) => Self::Missing(cid),
            other => Self::Backend(other.to_string()),
        }
    }
}

/// TRACE_MATRIX FC1-N11 + FC2-N19: minimal CAS read view required by admission and replay predicate verification.
pub trait PredicateCasView: Send + Sync {
    fn get_object(&self, cid: &Cid) -> Result<PredicateCasObject, PredicateCasViewError>;
}

impl PredicateCasView for CasStore {
    fn get_object(&self, cid: &Cid) -> Result<PredicateCasObject, PredicateCasViewError> {
        let metadata = self
            .metadata(cid)
            .cloned()
            .ok_or(PredicateCasViewError::Missing(*cid))?;
        let bytes = self.get(cid)?;
        Ok(PredicateCasObject {
            bytes,
            object_type: metadata.object_type,
            schema_id: metadata.schema_id,
        })
    }
}

/// TRACE_MATRIX FC1-N11 + FC2-N19: empty proof store preserving pre-activation legacy replay behavior.
#[derive(Debug, Default)]
pub struct EmptyPredicateCasView;

impl PredicateCasView for EmptyPredicateCasView {
    fn get_object(&self, cid: &Cid) -> Result<PredicateCasObject, PredicateCasViewError> {
        Err(PredicateCasViewError::Missing(*cid))
    }
}

/// TRACE_MATRIX FC1-N11 + FC1-N12: sequencer-owned WorkTx projection consumed by executable predicates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredicateWorkView {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub parent_state_root: Hash,
    pub agent_id: AgentId,
    pub read_set: BTreeSet<ReadKey>,
    pub write_set: BTreeSet<WriteKey>,
    pub proposal_cid: Cid,
    pub stake: StakeMicroCoin,
}

impl PredicateWorkView {
    /// TRACE_MATRIX FC1-N11 + FC1-N12: derive predicate context from sequencer-owned WorkTx fields.
    pub fn from_work_tx(work: &WorkTx) -> Self {
        Self {
            tx_id: work.tx_id.clone(),
            task_id: work.task_id.clone(),
            parent_state_root: work.parent_state_root,
            agent_id: work.agent_id.clone(),
            read_set: work.read_set.clone(),
            write_set: work.write_set.clone(),
            proposal_cid: work.proposal_cid,
            stake: work.stake,
        }
    }

    /// TRACE_MATRIX FC1-N11 + FC1-N12: stable context digest bound into proof capsules.
    pub fn context_hash(&self, predicate_registry_root: Hash) -> Hash {
        let payload = PredicateWorkContextDigestPayload {
            predicate_registry_root,
            tx_id: self.tx_id.clone(),
            task_id: self.task_id.clone(),
            parent_state_root: self.parent_state_root,
            agent_id: self.agent_id.clone(),
            read_set: self.read_set.clone(),
            write_set: self.write_set.clone(),
            proposal_cid: self.proposal_cid,
            stake: self.stake,
        };
        let mut h = Sha256::new();
        h.update(b"turingosv4.predicate_work_context.v1");
        h.update(canonical_encode(&payload).expect("PredicateWorkContextDigestPayload encodes"));
        Hash::from_bytes(h.finalize().into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PredicateWorkContextDigestPayload {
    predicate_registry_root: Hash,
    tx_id: TxId,
    task_id: TaskId,
    parent_state_root: Hash,
    agent_id: AgentId,
    read_set: BTreeSet<ReadKey>,
    write_set: BTreeSet<WriteKey>,
    proposal_cid: Cid,
    stake: StakeMicroCoin,
}

/// TRACE_MATRIX FC1-N11 + FC1-N12: runtime predicate verification context assembled by the sequencer.
pub struct PredicateContext<'a> {
    pub registry_root: Hash,
    pub work: PredicateWorkView,
    pub proof_store: &'a dyn PredicateCasView,
}

/// TRACE_MATRIX FC1-N11 + FC1-N12: executable predicate verification failure taxonomy surfaced through L4.E.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredicateVerifyError {
    MissingProofCid,
    Cas(PredicateCasViewError),
    ProofObjectType {
        expected: ObjectType,
        got: ObjectType,
    },
    ProofSchema {
        expected: &'static str,
        got: Option<String>,
    },
    Decode(String),
    PredicateIdMismatch {
        expected: PredicateId,
        got: PredicateId,
    },
    RegistryRootMismatch {
        expected: Hash,
        got: Hash,
    },
    CodeHashMismatch {
        expected: [u8; 32],
        got: [u8; 32],
    },
    ProposalCidMismatch {
        expected: Cid,
        got: Cid,
    },
    ClaimValueMismatch {
        expected: bool,
        got: bool,
    },
    ContextHashMismatch {
        expected: Hash,
        got: Hash,
    },
    ProofKindMismatch {
        expected: PredicateProofKind,
        got: PredicateProofKind,
    },
    ExpectedStatementHashMismatch {
        expected: Hash,
        got: Option<Hash>,
    },
    ProofArtifactHashMismatch,
    ProposalPayloadMissing,
    ProposalPayloadDecode(String),
    ForbiddenPattern(String),
    PayloadTooLarge {
        max_bytes: usize,
        got_bytes: usize,
    },
    PayloadTooManyLines {
        max_lines: usize,
        got_lines: usize,
    },
    LeanCheckerFailed(String),
    LeanCheckerUnavailable(String),
}

/// TRACE_MATRIX FC1-N11 + FC1-N12: executable ground-truth predicate contract for sequencer admission.
pub trait Predicate: Send + Sync {
    fn predicate_id(&self) -> &str;
    fn code_hash(&self) -> [u8; 32];
    fn evaluate(&self, ctx: &PredicateContext<'_>) -> BoolWithProof;
    fn verify_proof(
        &self,
        ctx: &PredicateContext<'_>,
        claim: &BoolWithProof,
    ) -> Result<bool, PredicateVerifyError>;
}

/// TRACE_MATRIX FC1-N11 + FC1-N12 + FC2-N19: registry row coupling metadata, executable implementation, and required bundle membership.
#[derive(Clone)]
pub struct RegistryEntry {
    pub metadata: PredicateMetadata,
    pub impl_arc: Arc<dyn Predicate>,
    pub required_in: BTreeSet<PredicateBundleMap>,
}

impl fmt::Debug for RegistryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegistryEntry")
            .field("metadata", &self.metadata)
            .field("required_in", &self.required_in)
            .finish_non_exhaustive()
    }
}

/// L1 PredicateRegistry — a deterministic ordered store of predicate metadata.
///
/// Uses BTreeMap (not HashMap) per spec § 2 I-BTREE invariant.
#[derive(Debug, Clone)]
pub struct PredicateRegistry {
    entries: BTreeMap<String, RegistryEntry>,
    required_acceptance: BTreeSet<PredicateId>,
    required_settlement: BTreeSet<PredicateId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterError {
    /// A predicate with this id already registered (use `replace` for explicit replacement).
    DuplicateId(String),
    /// Empty / malformed predicate_id.
    InvalidId(String),
    CodeHashMismatch {
        id: String,
    },
}

impl PredicateRegistry {
    fn empty_unregistered() -> Self {
        Self {
            entries: BTreeMap::new(),
            required_acceptance: BTreeSet::new(),
            required_settlement: BTreeSet::new(),
        }
    }

    /// TRACE_MATRIX FC3-INV6: module-private predicate registration surface for boot/snapshot construction only.
    pub(crate) fn register(&mut self, meta: PredicateMetadata) -> Result<(), RegisterError> {
        self.register_entry(
            meta.clone(),
            Arc::new(StaticPredicate::new(
                meta.predicate_id.clone(),
                meta.code_hash,
                true,
            )),
            BTreeSet::new(),
        )
    }

    /// TRACE_MATRIX FC3-INV6: module-private executable predicate registration with metadata/code-hash parity.
    pub(crate) fn register_entry(
        &mut self,
        meta: PredicateMetadata,
        impl_arc: Arc<dyn Predicate>,
        required_in: BTreeSet<PredicateBundleMap>,
    ) -> Result<(), RegisterError> {
        if meta.predicate_id.is_empty() {
            return Err(RegisterError::InvalidId(meta.predicate_id));
        }
        if self.entries.contains_key(&meta.predicate_id) {
            return Err(RegisterError::DuplicateId(meta.predicate_id));
        }
        if impl_arc.predicate_id() != meta.predicate_id || impl_arc.code_hash() != meta.code_hash {
            return Err(RegisterError::CodeHashMismatch {
                id: meta.predicate_id,
            });
        }
        let pid = PredicateId(meta.predicate_id.clone());
        if required_in.contains(&PredicateBundleMap::Acceptance) {
            self.required_acceptance.insert(pid.clone());
        }
        if required_in.contains(&PredicateBundleMap::Settlement) {
            self.required_settlement.insert(pid);
        }
        self.entries.insert(
            meta.predicate_id.clone(),
            RegistryEntry {
                metadata: meta,
                impl_arc,
                required_in,
            },
        );
        Ok(())
    }

    /// Lookup by predicate_id.
    pub fn get(&self, id: &str) -> Option<&PredicateMetadata> {
        self.entries.get(id).map(|entry| &entry.metadata)
    }

    /// TRACE_MATRIX FC1-N11 + FC1-N12: lookup executable predicate entry by typed predicate id.
    pub fn entry(&self, id: &PredicateId) -> Option<&RegistryEntry> {
        self.entries.get(&id.0)
    }

    /// TRACE_MATRIX FC1-N11 + FC1-N12: lookup registry-declared code hash for admission proof parity.
    pub fn code_hash_for(&self, id: &PredicateId) -> Option<[u8; 32]> {
        self.entry(id).map(|entry| entry.metadata.code_hash)
    }

    /// TRACE_MATRIX FC1-N11: required predicate key set for exact WorkTx bundle admission.
    pub fn required_predicates(&self, map: PredicateBundleMap) -> &BTreeSet<PredicateId> {
        match map {
            PredicateBundleMap::Acceptance => &self.required_acceptance,
            PredicateBundleMap::Settlement => &self.required_settlement,
        }
    }

    /// Total count of registered predicates.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Compute Merkle-style root over all registered predicates' canonical hashes.
    /// Returns sha256 of empty bytes if registry is empty (matches spec § 5.L1 EMPTY_TREE_ROOT).
    pub fn merkle_root(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        // BTreeMap iterates in lexicographic key order — deterministic.
        for (_id, entry) in &self.entries {
            h.update(entry.metadata.canonical_hash());
            h.update(
                canonical_encode(&entry.required_in)
                    .expect("PredicateBundleMap set is canonical-encodable"),
            );
        }
        h.finalize().into()
    }

    /// TRACE_MATRIX FC1-N11 + FC2-N19: registry Merkle root as canonical `Hash` for QState and LedgerEntry binding.
    pub fn merkle_root_hash(&self) -> Hash {
        Hash::from_bytes(self.merkle_root())
    }

    /// TRACE_MATRIX FC1-N11 + FC2-N19: produce replayable snapshot bytes for boot activation CAS anchoring.
    pub fn snapshot_capsule(&self) -> PredicateRegistrySnapshotCapsule {
        let entries = self
            .entries
            .values()
            .map(|entry| PredicateSnapshotEntry {
                metadata: entry.metadata.clone(),
                required_in: entry.required_in.clone(),
            })
            .collect();
        PredicateRegistrySnapshotCapsule {
            schema_id: PredicateRegistrySnapshotCapsule::SCHEMA_ID.to_string(),
            entries,
            merkle_root: self.merkle_root_hash(),
        }
    }

    /// TRACE_MATRIX FC2-N19 + FC3-INV6: construct the executable registry only from the boot manifest.
    pub fn from_boot_manifest(manifest: BootPredicateManifest) -> Result<Self, RegisterError> {
        let mut registry = PredicateRegistry::empty_unregistered();
        for spec in manifest.entries {
            let impl_arc = predicate_impl_from_spec(&spec);
            registry.register_entry(spec.metadata, impl_arc, spec.required_in)?;
        }
        Ok(registry)
    }

    /// TRACE_MATRIX FC1-N11 + FC2-N19: reconstruct the executable registry from snapshot bytes plus current binary implementations.
    pub fn from_snapshot_and_binary_impls(
        snapshot_bytes: &[u8],
        binary_impls: BTreeMap<String, Arc<dyn Predicate>>,
    ) -> Result<Self, SnapshotLoadError> {
        let snapshot: PredicateRegistrySnapshotCapsule = canonical_decode(snapshot_bytes)
            .map_err(|e| SnapshotLoadError::Decode(e.to_string()))?;
        if snapshot.schema_id != PredicateRegistrySnapshotCapsule::SCHEMA_ID {
            return Err(SnapshotLoadError::Schema {
                expected: PredicateRegistrySnapshotCapsule::SCHEMA_ID,
                got: snapshot.schema_id,
            });
        }
        let mut registry = PredicateRegistry::empty_unregistered();
        for entry in snapshot.entries {
            let id = entry.metadata.predicate_id.clone();
            let impl_arc = binary_impls
                .get(&id)
                .cloned()
                .ok_or_else(|| SnapshotLoadError::MissingBinaryImpl(id.clone()))?;
            if impl_arc.code_hash() != entry.metadata.code_hash {
                return Err(SnapshotLoadError::BinaryDriftFromSnapshot {
                    id,
                    expected: entry.metadata.code_hash,
                    got: impl_arc.code_hash(),
                });
            }
            registry
                .register_entry(entry.metadata, impl_arc, entry.required_in)
                .map_err(SnapshotLoadError::Register)?;
        }
        if registry.merkle_root_hash() != snapshot.merkle_root {
            return Err(SnapshotLoadError::RootMismatch {
                expected: snapshot.merkle_root,
                got: registry.merkle_root_hash(),
            });
        }
        Ok(registry)
    }

    /// TRACE_MATRIX FC2-N19: expose executable binary implementations for snapshot replay parity checks.
    pub fn binary_impls(&self) -> BTreeMap<String, Arc<dyn Predicate>> {
        self.entries
            .iter()
            .map(|(id, entry)| (id.clone(), entry.impl_arc.clone()))
            .collect()
    }

    /// Agent-visible projection of the registry (Goodhart shield per Inv 10).
    /// Returns a NEW registry containing only Public predicates + commit-reveal that have reveal-time passed.
    pub fn agent_visible_view(&self, now: u64) -> Self {
        Self {
            entries: self
                .entries
                .iter()
                .filter(|(_, entry)| entry.metadata.visibility.content_visible_to_agent(now))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            required_acceptance: self.required_acceptance.clone(),
            required_settlement: self.required_settlement.clone(),
        }
    }
}

/// TRACE_MATRIX FC2-N19 + FC3-INV6: boot-only source for constructing the executable predicate registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BootPredicateManifest {
    pub entries: Vec<BootPredicateSpec>,
}

impl BootPredicateManifest {
    /// TRACE_MATRIX FC2-N19: explicit empty manifest for legacy/pre-activation tests.
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// TRACE_MATRIX FC1-N11 + FC1-N12 + FC2-N19: production predicate catalog activated at fresh boot.
    pub fn v8_production() -> Self {
        let mut entries = Vec::new();

        // Backward-compatible authoritative replacement for the historical
        // runner-stamped `acc1` claim. Existing WorkTx builders already emit
        // the key; bound mode now recomputes the claim by resolving the
        // proposal payload from CAS instead of trusting the boolean.
        entries.push(BootPredicateSpec::new(
            "acc1",
            BootPredicateKind::ProposalPayloadNotEmpty,
            [PredicateBundleMap::Acceptance],
        ));

        // Executable v8 predicate catalog. These are shipped in the binary so
        // snapshots can be reconstructed with explicit binary parity. They are
        // not required by default until WorkTx runners migrate their exact
        // predicate-result key sets.
        entries.push(BootPredicateSpec::new(
            "forbidden_patterns_v1",
            BootPredicateKind::ForbiddenPatterns {
                patterns: vec![
                    "native_decide".to_string(),
                    "unsafe".to_string(),
                    "axiom ".to_string(),
                ],
            },
            [],
        ));
        entries.push(BootPredicateSpec::new(
            "sorry_free_v1",
            BootPredicateKind::SorryFree,
            [],
        ));
        entries.push(BootPredicateSpec::new(
            "payload_size_v1",
            BootPredicateKind::PayloadSize {
                max_bytes: 1_048_576,
                max_lines: 20_000,
            },
            [],
        ));
        entries.push(BootPredicateSpec::new(
            "lean_artifact_v1",
            BootPredicateKind::LeanArtifact,
            [],
        ));

        Self { entries }
    }
}

/// TRACE_MATRIX FC2-N19: manifest row pairing predicate metadata with executable boot implementation kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BootPredicateSpec {
    pub metadata: PredicateMetadata,
    pub required_in: BTreeSet<PredicateBundleMap>,
    pub kind: BootPredicateKind,
}

impl BootPredicateSpec {
    /// TRACE_MATRIX FC2-N19: construct a boot predicate spec with deterministic code-hash derivation.
    pub fn new<const N: usize>(
        predicate_id: &str,
        kind: BootPredicateKind,
        required_in: [PredicateBundleMap; N],
    ) -> Self {
        let code_hash = code_hash_for_boot_predicate(predicate_id, &kind);
        Self {
            metadata: PredicateMetadata {
                predicate_id: predicate_id.to_string(),
                version: 1,
                code_hash,
                input_schema: "PredicateContext.v1".to_string(),
                output_schema: "BoolWithProof.v1".to_string(),
                visibility: Visibility::Public,
                owner: "system".to_string(),
                test_suite_hash: code_hash_for_bytes(
                    b"turingosv4.predicate_registry_bind.v8.tests",
                ),
                safety_class: SafetyOrCreation::Safety,
            },
            required_in: required_in.into_iter().collect(),
            kind,
        }
    }
}

/// TRACE_MATRIX FC1-N12 + FC2-N19: binary predicate implementation catalog used for boot and replay reconstruction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BootPredicateKind {
    StaticTrue,
    StaticBool(bool),
    ProposalPayloadNotEmpty,
    ForbiddenPatterns { patterns: Vec<String> },
    SorryFree,
    PayloadSize { max_bytes: usize, max_lines: usize },
    LeanArtifact,
}

/// TRACE_MATRIX FC1-N11 + FC2-N19: deterministic failures while reconstructing a registry snapshot against binary implementations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotLoadError {
    Decode(String),
    Schema {
        expected: &'static str,
        got: String,
    },
    MissingBinaryImpl(String),
    BinaryDriftFromSnapshot {
        id: String,
        expected: [u8; 32],
        got: [u8; 32],
    },
    Register(RegisterError),
    RootMismatch {
        expected: Hash,
        got: Hash,
    },
}

/// TRACE_MATRIX FC1-N12: simple executable predicate used for tests and legacy metadata registration.
#[derive(Debug, Clone)]
pub struct StaticPredicate {
    id: String,
    code_hash: [u8; 32],
    result: bool,
}

impl StaticPredicate {
    /// TRACE_MATRIX FC1-N12: build a static predicate implementation with explicit id/code hash/result.
    pub fn new(id: impl Into<String>, code_hash: [u8; 32], result: bool) -> Self {
        Self {
            id: id.into(),
            code_hash,
            result,
        }
    }
}

impl Predicate for StaticPredicate {
    fn predicate_id(&self) -> &str {
        &self.id
    }

    fn code_hash(&self) -> [u8; 32] {
        self.code_hash
    }

    fn evaluate(&self, _ctx: &PredicateContext<'_>) -> BoolWithProof {
        BoolWithProof {
            value: self.result,
            proof_cid: None,
        }
    }

    fn verify_proof(
        &self,
        _ctx: &PredicateContext<'_>,
        _claim: &BoolWithProof,
    ) -> Result<bool, PredicateVerifyError> {
        Ok(self.result)
    }
}

#[derive(Debug, Clone)]
struct ProposalPayloadNotEmptyPredicate {
    id: String,
    code_hash: [u8; 32],
}

impl Predicate for ProposalPayloadNotEmptyPredicate {
    fn predicate_id(&self) -> &str {
        &self.id
    }

    fn code_hash(&self) -> [u8; 32] {
        self.code_hash
    }

    fn evaluate(&self, ctx: &PredicateContext<'_>) -> BoolWithProof {
        BoolWithProof {
            value: proposal_payload_bytes(ctx)
                .map(|bytes| !bytes.is_empty())
                .unwrap_or(false),
            proof_cid: None,
        }
    }

    fn verify_proof(
        &self,
        ctx: &PredicateContext<'_>,
        _claim: &BoolWithProof,
    ) -> Result<bool, PredicateVerifyError> {
        Ok(!proposal_payload_bytes(ctx)?.is_empty())
    }
}

#[derive(Debug, Clone)]
struct ForbiddenPatternsPredicate {
    id: String,
    code_hash: [u8; 32],
    patterns: Vec<String>,
}

impl Predicate for ForbiddenPatternsPredicate {
    fn predicate_id(&self) -> &str {
        &self.id
    }

    fn code_hash(&self) -> [u8; 32] {
        self.code_hash
    }

    fn evaluate(&self, ctx: &PredicateContext<'_>) -> BoolWithProof {
        BoolWithProof {
            value: self.verify_payload(ctx).unwrap_or(false),
            proof_cid: None,
        }
    }

    fn verify_proof(
        &self,
        ctx: &PredicateContext<'_>,
        _claim: &BoolWithProof,
    ) -> Result<bool, PredicateVerifyError> {
        self.verify_payload(ctx)
    }
}

impl ForbiddenPatternsPredicate {
    fn verify_payload(&self, ctx: &PredicateContext<'_>) -> Result<bool, PredicateVerifyError> {
        let bytes = proposal_payload_bytes(ctx)?;
        let text = String::from_utf8_lossy(&bytes);
        for pattern in &self.patterns {
            if text.contains(pattern) {
                return Err(PredicateVerifyError::ForbiddenPattern(pattern.clone()));
            }
        }
        Ok(true)
    }
}

#[derive(Debug, Clone)]
struct SorryFreePredicate {
    id: String,
    code_hash: [u8; 32],
}

impl Predicate for SorryFreePredicate {
    fn predicate_id(&self) -> &str {
        &self.id
    }

    fn code_hash(&self) -> [u8; 32] {
        self.code_hash
    }

    fn evaluate(&self, ctx: &PredicateContext<'_>) -> BoolWithProof {
        BoolWithProof {
            value: self.verify_payload(ctx).unwrap_or(false),
            proof_cid: None,
        }
    }

    fn verify_proof(
        &self,
        ctx: &PredicateContext<'_>,
        _claim: &BoolWithProof,
    ) -> Result<bool, PredicateVerifyError> {
        self.verify_payload(ctx)
    }
}

impl SorryFreePredicate {
    fn verify_payload(&self, ctx: &PredicateContext<'_>) -> Result<bool, PredicateVerifyError> {
        let bytes = proposal_payload_bytes(ctx)?;
        let text = String::from_utf8_lossy(&bytes);
        Ok(!text
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .any(|token| token == "sorry" || token == "admit"))
    }
}

#[derive(Debug, Clone)]
struct PayloadSizePredicate {
    id: String,
    code_hash: [u8; 32],
    max_bytes: usize,
    max_lines: usize,
}

impl Predicate for PayloadSizePredicate {
    fn predicate_id(&self) -> &str {
        &self.id
    }

    fn code_hash(&self) -> [u8; 32] {
        self.code_hash
    }

    fn evaluate(&self, ctx: &PredicateContext<'_>) -> BoolWithProof {
        BoolWithProof {
            value: self.verify_payload(ctx).unwrap_or(false),
            proof_cid: None,
        }
    }

    fn verify_proof(
        &self,
        ctx: &PredicateContext<'_>,
        _claim: &BoolWithProof,
    ) -> Result<bool, PredicateVerifyError> {
        self.verify_payload(ctx)
    }
}

impl PayloadSizePredicate {
    fn verify_payload(&self, ctx: &PredicateContext<'_>) -> Result<bool, PredicateVerifyError> {
        let bytes = proposal_payload_bytes(ctx)?;
        if bytes.len() > self.max_bytes {
            return Err(PredicateVerifyError::PayloadTooLarge {
                max_bytes: self.max_bytes,
                got_bytes: bytes.len(),
            });
        }
        let lines = bytes.iter().filter(|&&b| b == b'\n').count() + usize::from(!bytes.is_empty());
        if lines > self.max_lines {
            return Err(PredicateVerifyError::PayloadTooManyLines {
                max_lines: self.max_lines,
                got_lines: lines,
            });
        }
        Ok(true)
    }
}

#[derive(Debug, Clone)]
struct LeanArtifactPredicate {
    id: String,
    code_hash: [u8; 32],
}

impl Predicate for LeanArtifactPredicate {
    fn predicate_id(&self) -> &str {
        &self.id
    }

    fn code_hash(&self) -> [u8; 32] {
        self.code_hash
    }

    fn evaluate(&self, _ctx: &PredicateContext<'_>) -> BoolWithProof {
        BoolWithProof {
            value: false,
            proof_cid: None,
        }
    }

    fn verify_proof(
        &self,
        ctx: &PredicateContext<'_>,
        claim: &BoolWithProof,
    ) -> Result<bool, PredicateVerifyError> {
        let capsule = decode_and_validate_proof_capsule(
            ctx,
            PredicateId(self.id.clone()),
            self.code_hash,
            PredicateProofKind::LeanArtifact,
            claim,
        )?;
        let proposal_bytes = proposal_payload_bytes(ctx)?;
        let expected_statement_hash = lean_expected_statement_hash(
            &self.id,
            self.code_hash,
            ctx.registry_root,
            &ctx.work,
            &proposal_bytes,
        );
        if capsule.expected_statement_hash != Some(expected_statement_hash) {
            return Err(PredicateVerifyError::ExpectedStatementHashMismatch {
                expected: expected_statement_hash,
                got: capsule.expected_statement_hash,
            });
        }
        let proof_artifact_cid = capsule
            .proof_artifact_cid
            .ok_or(PredicateVerifyError::MissingProofCid)?;
        let proof_artifact_sha256 = capsule
            .proof_artifact_sha256
            .ok_or(PredicateVerifyError::ProofArtifactHashMismatch)?;
        let artifact = ctx
            .proof_store
            .get_object(&proof_artifact_cid)
            .map_err(PredicateVerifyError::Cas)?;
        let mut h = Sha256::new();
        h.update(&artifact.bytes);
        let digest = Hash::from_bytes(h.finalize().into());
        if digest != proof_artifact_sha256 {
            return Err(PredicateVerifyError::ProofArtifactHashMismatch);
        }
        run_lean_checker(&artifact.bytes)?;
        Ok(capsule.claimed_value)
    }
}

fn predicate_impl_from_spec(spec: &BootPredicateSpec) -> Arc<dyn Predicate> {
    match spec.kind {
        BootPredicateKind::StaticTrue => Arc::new(StaticPredicate::new(
            spec.metadata.predicate_id.clone(),
            spec.metadata.code_hash,
            true,
        )),
        BootPredicateKind::StaticBool(result) => Arc::new(StaticPredicate::new(
            spec.metadata.predicate_id.clone(),
            spec.metadata.code_hash,
            result,
        )),
        BootPredicateKind::ProposalPayloadNotEmpty => Arc::new(ProposalPayloadNotEmptyPredicate {
            id: spec.metadata.predicate_id.clone(),
            code_hash: spec.metadata.code_hash,
        }),
        BootPredicateKind::ForbiddenPatterns { ref patterns } => {
            Arc::new(ForbiddenPatternsPredicate {
                id: spec.metadata.predicate_id.clone(),
                code_hash: spec.metadata.code_hash,
                patterns: patterns.clone(),
            })
        }
        BootPredicateKind::SorryFree => Arc::new(SorryFreePredicate {
            id: spec.metadata.predicate_id.clone(),
            code_hash: spec.metadata.code_hash,
        }),
        BootPredicateKind::PayloadSize {
            max_bytes,
            max_lines,
        } => Arc::new(PayloadSizePredicate {
            id: spec.metadata.predicate_id.clone(),
            code_hash: spec.metadata.code_hash,
            max_bytes,
            max_lines,
        }),
        BootPredicateKind::LeanArtifact => Arc::new(LeanArtifactPredicate {
            id: spec.metadata.predicate_id.clone(),
            code_hash: spec.metadata.code_hash,
        }),
    }
}

/// TRACE_MATRIX FC1-N11 + FC1-N12: decode and cross-check a predicate proof capsule against sequencer-owned context.
pub fn decode_and_validate_proof_capsule(
    ctx: &PredicateContext<'_>,
    predicate_id: PredicateId,
    code_hash: [u8; 32],
    expected_kind: PredicateProofKind,
    claim: &BoolWithProof,
) -> Result<PredicateProofCapsule, PredicateVerifyError> {
    let proof_cid = claim
        .proof_cid
        .ok_or(PredicateVerifyError::MissingProofCid)?;
    let obj = ctx
        .proof_store
        .get_object(&proof_cid)
        .map_err(PredicateVerifyError::Cas)?;
    if obj.object_type != ObjectType::PredicateProofCapsule {
        return Err(PredicateVerifyError::ProofObjectType {
            expected: ObjectType::PredicateProofCapsule,
            got: obj.object_type,
        });
    }
    if obj.schema_id.as_deref() != Some(PredicateProofCapsule::SCHEMA_ID) {
        return Err(PredicateVerifyError::ProofSchema {
            expected: PredicateProofCapsule::SCHEMA_ID,
            got: obj.schema_id,
        });
    }
    let capsule: PredicateProofCapsule =
        canonical_decode(&obj.bytes).map_err(|e| PredicateVerifyError::Decode(e.to_string()))?;
    if capsule.schema_id != PredicateProofCapsule::SCHEMA_ID {
        return Err(PredicateVerifyError::ProofSchema {
            expected: PredicateProofCapsule::SCHEMA_ID,
            got: Some(capsule.schema_id),
        });
    }
    if capsule.predicate_id != predicate_id {
        return Err(PredicateVerifyError::PredicateIdMismatch {
            expected: predicate_id,
            got: capsule.predicate_id,
        });
    }
    if capsule.predicate_registry_root != ctx.registry_root {
        return Err(PredicateVerifyError::RegistryRootMismatch {
            expected: ctx.registry_root,
            got: capsule.predicate_registry_root,
        });
    }
    if capsule.predicate_code_hash != code_hash {
        return Err(PredicateVerifyError::CodeHashMismatch {
            expected: code_hash,
            got: capsule.predicate_code_hash,
        });
    }
    if capsule.proposal_cid != ctx.work.proposal_cid {
        return Err(PredicateVerifyError::ProposalCidMismatch {
            expected: ctx.work.proposal_cid,
            got: capsule.proposal_cid,
        });
    }
    if capsule.claimed_value != claim.value {
        return Err(PredicateVerifyError::ClaimValueMismatch {
            expected: claim.value,
            got: capsule.claimed_value,
        });
    }
    let expected_context = ctx.work.context_hash(ctx.registry_root);
    if capsule.work_context_hash != expected_context {
        return Err(PredicateVerifyError::ContextHashMismatch {
            expected: expected_context,
            got: capsule.work_context_hash,
        });
    }
    if capsule.proof_kind != expected_kind {
        return Err(PredicateVerifyError::ProofKindMismatch {
            expected: expected_kind,
            got: capsule.proof_kind,
        });
    }
    Ok(capsule)
}

fn code_hash_for_bytes(bytes: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().into()
}

fn code_hash_for_boot_predicate(id: &str, kind: &BootPredicateKind) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"turingosv4.predicate.binary.v8");
    h.update(id.as_bytes());
    h.update(canonical_encode(kind).expect("BootPredicateKind encodes"));
    h.finalize().into()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ProposalTelemetryRef {
    agent_id: AgentId,
    prompt_context_hash: Hash,
    proposal_artifact_cid: Cid,
    candidate_tactic: String,
    token_counts: ProposalTokenCountsRef,
    tool_calls: Vec<ProposalToolCallRecordRef>,
    branch_id: String,
    parent_tx: Option<TxId>,
    #[serde(default)]
    verification_result_cid: Option<Cid>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ProposalTokenCountsRef {
    prompt_tokens: u64,
    completion_tokens: u64,
    tool_tokens: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ProposalToolCallRecordRef {
    tool_id: String,
    args_hash: Hash,
    result_hash: Hash,
}

fn proposal_payload_bytes(ctx: &PredicateContext<'_>) -> Result<Vec<u8>, PredicateVerifyError> {
    let obj = ctx
        .proof_store
        .get_object(&ctx.work.proposal_cid)
        .map_err(PredicateVerifyError::Cas)?;
    if obj.object_type == ObjectType::ProposalPayload {
        return Ok(obj.bytes);
    }
    if obj.schema_id.as_deref() == Some("turingosv4.proposal_telemetry.v1") {
        let telemetry: ProposalTelemetryRef = canonical_decode(&obj.bytes)
            .map_err(|e| PredicateVerifyError::ProposalPayloadDecode(e.to_string()))?;
        let artifact = ctx
            .proof_store
            .get_object(&telemetry.proposal_artifact_cid)
            .map_err(PredicateVerifyError::Cas)?;
        if artifact.object_type != ObjectType::ProposalPayload
            && artifact.schema_id.as_deref() != Some("turingos-artifact-bundle-v1")
        {
            return Err(PredicateVerifyError::ProposalPayloadMissing);
        }
        return Ok(artifact.bytes);
    }
    Ok(obj.bytes)
}

/// TRACE_MATRIX FC1-N12: deterministic Lean expected-statement digest bound into Lean predicate proof capsules.
pub fn lean_expected_statement_hash(
    predicate_id: &str,
    code_hash: [u8; 32],
    registry_root: Hash,
    work: &PredicateWorkView,
    proposal_bytes: &[u8],
) -> Hash {
    let mut h = Sha256::new();
    h.update(b"turingosv4.predicate.lean.expected_statement.v1");
    h.update(predicate_id.as_bytes());
    h.update(code_hash);
    h.update(work.context_hash(registry_root).0);
    h.update(proposal_bytes);
    Hash::from_bytes(h.finalize().into())
}

fn run_lean_checker(bytes: &[u8]) -> Result<(), PredicateVerifyError> {
    let mut h = Sha256::new();
    h.update(bytes);
    let digest: [u8; 32] = h.finalize().into();
    let filename = format!(
        "turingos-predicate-lean-{}-{}.lean",
        std::process::id(),
        digest
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>()
    );
    let path = std::env::temp_dir().join(filename);
    fs::write(&path, bytes)
        .map_err(|e| PredicateVerifyError::LeanCheckerUnavailable(e.to_string()))?;
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::env::temp_dir());
    let output = run_sanitized(SanitizedCommand {
        program: "lean".into(),
        args: vec![path.to_string_lossy().into_owned()],
        cwd,
        env: env_allowlist_from_current(&["PATH"]),
        stdin: None,
        timeout: Duration::from_secs(30),
    })
    .map_err(|e| PredicateVerifyError::LeanCheckerUnavailable(e.to_string()));
    let _ = fs::remove_file(&path);
    let output = output?;
    if output.success() {
        Ok(())
    } else {
        Err(PredicateVerifyError::LeanCheckerFailed(format!(
            "exit_code={:?} timed_out={}",
            output.exit_code, output.timed_out
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_meta(id: &str, vis: Visibility) -> PredicateMetadata {
        PredicateMetadata {
            predicate_id: id.to_string(),
            version: 1,
            code_hash: [0xab; 32],
            input_schema: r#"{"type":"object"}"#.to_string(),
            output_schema: r#"{"type":"boolean"}"#.to_string(),
            visibility: vis,
            owner: "system".to_string(),
            test_suite_hash: [0xcd; 32],
            safety_class: SafetyOrCreation::Safety,
        }
    }

    #[test]
    fn register_and_get_round_trip() {
        let mut reg = PredicateRegistry::from_boot_manifest(
            crate::top_white::predicates::registry::BootPredicateManifest::empty(),
        )
        .expect("empty predicate manifest");
        let m = sample_meta("lean4_oracle", Visibility::Public);
        reg.register(m.clone()).unwrap();
        assert_eq!(reg.get("lean4_oracle"), Some(&m));
    }

    #[test]
    fn duplicate_id_rejected() {
        let mut reg = PredicateRegistry::from_boot_manifest(
            crate::top_white::predicates::registry::BootPredicateManifest::empty(),
        )
        .expect("empty predicate manifest");
        let m = sample_meta("dup", Visibility::Public);
        reg.register(m.clone()).unwrap();
        assert_eq!(
            reg.register(m),
            Err(RegisterError::DuplicateId("dup".to_string()))
        );
    }

    #[test]
    fn empty_id_rejected() {
        let mut reg = PredicateRegistry::from_boot_manifest(
            crate::top_white::predicates::registry::BootPredicateManifest::empty(),
        )
        .expect("empty predicate manifest");
        let m = sample_meta("", Visibility::Public);
        assert_eq!(
            reg.register(m),
            Err(RegisterError::InvalidId("".to_string()))
        );
    }

    #[test]
    fn merkle_root_deterministic_two_runs() {
        let mut reg1 = PredicateRegistry::from_boot_manifest(
            crate::top_white::predicates::registry::BootPredicateManifest::empty(),
        )
        .expect("empty predicate manifest");
        let mut reg2 = PredicateRegistry::from_boot_manifest(
            crate::top_white::predicates::registry::BootPredicateManifest::empty(),
        )
        .expect("empty predicate manifest");
        for id in &["b_pred", "a_pred", "c_pred"] {
            // Register in DIFFERENT orders; BTreeMap normalizes
            reg1.register(sample_meta(id, Visibility::Public)).unwrap();
        }
        for id in &["c_pred", "a_pred", "b_pred"] {
            reg2.register(sample_meta(id, Visibility::Public)).unwrap();
        }
        assert_eq!(
            reg1.merkle_root(),
            reg2.merkle_root(),
            "BTreeMap-ordered Merkle root is order-insensitive (I-DET)"
        );
    }

    #[test]
    fn merkle_root_changes_on_register() {
        let mut reg = PredicateRegistry::from_boot_manifest(
            crate::top_white::predicates::registry::BootPredicateManifest::empty(),
        )
        .expect("empty predicate manifest");
        let r0 = reg.merkle_root();
        reg.register(sample_meta("p1", Visibility::Public)).unwrap();
        let r1 = reg.merkle_root();
        assert_ne!(r0, r1, "registering predicate must change root");
    }

    #[test]
    fn agent_visible_view_filters_private() {
        let mut reg = PredicateRegistry::from_boot_manifest(
            crate::top_white::predicates::registry::BootPredicateManifest::empty(),
        )
        .expect("empty predicate manifest");
        reg.register(sample_meta("public_pred", Visibility::Public))
            .unwrap();
        reg.register(sample_meta("private_pred", Visibility::Private))
            .unwrap();
        reg.register(sample_meta(
            "future_pred",
            Visibility::CommitReveal {
                reveal_at_logical_t: 1000,
                predicate_hash: [0u8; 32],
            },
        ))
        .unwrap();

        let view_now = reg.agent_visible_view(0);
        assert_eq!(view_now.len(), 1, "only public visible at now=0");
        assert!(view_now.get("public_pred").is_some());
        assert!(view_now.get("private_pred").is_none(), "private hidden");
        assert!(
            view_now.get("future_pred").is_none(),
            "commit-reveal pre-reveal hidden"
        );

        let view_later = reg.agent_visible_view(1000);
        assert_eq!(view_later.len(), 2, "public + commit-reveal at reveal time");
        assert!(
            view_later.get("future_pred").is_some(),
            "commit-reveal now visible"
        );
    }

    #[test]
    fn empty_registry_root_is_sha256_empty() {
        let reg = PredicateRegistry::from_boot_manifest(
            crate::top_white::predicates::registry::BootPredicateManifest::empty(),
        )
        .expect("empty predicate manifest");
        let r = reg.merkle_root();
        let expected = {
            let h = Sha256::new();
            // empty input
            h.finalize()
        };
        assert_eq!(r, <[u8; 32]>::from(expected));
    }

    #[test]
    fn metadata_canonical_hash_deterministic() {
        let m1 = sample_meta("test", Visibility::Public);
        let m2 = sample_meta("test", Visibility::Public);
        assert_eq!(
            m1.canonical_hash(),
            m2.canonical_hash(),
            "same metadata → same canonical hash (I-DET)"
        );
    }

    #[test]
    fn metadata_canonical_hash_differs_on_visibility() {
        let m_pub = sample_meta("p", Visibility::Public);
        let m_priv = sample_meta("p", Visibility::Private);
        assert_ne!(
            m_pub.canonical_hash(),
            m_priv.canonical_hash(),
            "visibility class is part of canonical hash"
        );
    }
}
