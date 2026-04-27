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
use std::collections::BTreeMap;

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

/// L1 PredicateRegistry — a deterministic ordered store of predicate metadata.
///
/// Uses BTreeMap (not HashMap) per spec § 2 I-BTREE invariant.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PredicateRegistry {
    predicates: BTreeMap<String, PredicateMetadata>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegisterError {
    /// A predicate with this id already registered (use `replace` for explicit replacement).
    DuplicateId(String),
    /// Empty / malformed predicate_id.
    InvalidId(String),
}

impl PredicateRegistry {
    pub fn new() -> Self {
        Self {
            predicates: BTreeMap::new(),
        }
    }

    /// Register a NEW predicate. Returns DuplicateId if id exists.
    pub fn register(&mut self, meta: PredicateMetadata) -> Result<(), RegisterError> {
        if meta.predicate_id.is_empty() {
            return Err(RegisterError::InvalidId(meta.predicate_id));
        }
        if self.predicates.contains_key(&meta.predicate_id) {
            return Err(RegisterError::DuplicateId(meta.predicate_id));
        }
        self.predicates.insert(meta.predicate_id.clone(), meta);
        Ok(())
    }

    /// Lookup by predicate_id.
    pub fn get(&self, id: &str) -> Option<&PredicateMetadata> {
        self.predicates.get(id)
    }

    /// Total count of registered predicates.
    pub fn len(&self) -> usize {
        self.predicates.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.predicates.is_empty()
    }

    /// Compute Merkle-style root over all registered predicates' canonical hashes.
    /// Returns sha256 of empty bytes if registry is empty (matches spec § 5.L1 EMPTY_TREE_ROOT).
    pub fn merkle_root(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        // BTreeMap iterates in lexicographic key order — deterministic.
        for (_id, meta) in &self.predicates {
            h.update(meta.canonical_hash());
        }
        h.finalize().into()
    }

    /// Agent-visible projection of the registry (Goodhart shield per Inv 10).
    /// Returns a NEW registry containing only Public predicates + commit-reveal that have reveal-time passed.
    pub fn agent_visible_view(&self, now: u64) -> Self {
        Self {
            predicates: self
                .predicates
                .iter()
                .filter(|(_, m)| m.visibility.content_visible_to_agent(now))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        }
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
        let mut reg = PredicateRegistry::new();
        let m = sample_meta("lean4_oracle", Visibility::Public);
        reg.register(m.clone()).unwrap();
        assert_eq!(reg.get("lean4_oracle"), Some(&m));
    }

    #[test]
    fn duplicate_id_rejected() {
        let mut reg = PredicateRegistry::new();
        let m = sample_meta("dup", Visibility::Public);
        reg.register(m.clone()).unwrap();
        assert_eq!(
            reg.register(m),
            Err(RegisterError::DuplicateId("dup".to_string()))
        );
    }

    #[test]
    fn empty_id_rejected() {
        let mut reg = PredicateRegistry::new();
        let m = sample_meta("", Visibility::Public);
        assert_eq!(
            reg.register(m),
            Err(RegisterError::InvalidId("".to_string()))
        );
    }

    #[test]
    fn merkle_root_deterministic_two_runs() {
        let mut reg1 = PredicateRegistry::new();
        let mut reg2 = PredicateRegistry::new();
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
        let mut reg = PredicateRegistry::new();
        let r0 = reg.merkle_root();
        reg.register(sample_meta("p1", Visibility::Public)).unwrap();
        let r1 = reg.merkle_root();
        assert_ne!(r0, r1, "registering predicate must change root");
    }

    #[test]
    fn agent_visible_view_filters_private() {
        let mut reg = PredicateRegistry::new();
        reg.register(sample_meta("public_pred", Visibility::Public)).unwrap();
        reg.register(sample_meta("private_pred", Visibility::Private)).unwrap();
        reg.register(sample_meta(
            "future_pred",
            Visibility::CommitReveal {
                reveal_at_logical_t: 1000,
                predicate_hash: [0u8; 32],
            },
        )).unwrap();

        let view_now = reg.agent_visible_view(0);
        assert_eq!(view_now.len(), 1, "only public visible at now=0");
        assert!(view_now.get("public_pred").is_some());
        assert!(view_now.get("private_pred").is_none(), "private hidden");
        assert!(view_now.get("future_pred").is_none(), "commit-reveal pre-reveal hidden");

        let view_later = reg.agent_visible_view(1000);
        assert_eq!(view_later.len(), 2, "public + commit-reveal at reveal time");
        assert!(view_later.get("future_pred").is_some(), "commit-reveal now visible");
    }

    #[test]
    fn empty_registry_root_is_sha256_empty() {
        let reg = PredicateRegistry::new();
        let r = reg.merkle_root();
        let expected = {
            let mut h = Sha256::new();
            // empty input
            h.finalize()
        };
        assert_eq!(r, <[u8; 32]>::from(expected));
    }

    #[test]
    fn metadata_canonical_hash_deterministic() {
        let m1 = sample_meta("test", Visibility::Public);
        let m2 = sample_meta("test", Visibility::Public);
        assert_eq!(m1.canonical_hash(), m2.canonical_hash(),
            "same metadata → same canonical hash (I-DET)");
    }

    #[test]
    fn metadata_canonical_hash_differs_on_visibility() {
        let m_pub = sample_meta("p", Visibility::Public);
        let m_priv = sample_meta("p", Visibility::Private);
        assert_ne!(m_pub.canonical_hash(), m_priv.canonical_hash(),
            "visibility class is part of canonical hash");
    }
}
