//! L2 Tool Registry per WP architecture § 5.L2.
//!
//! Tools are registered with a typed `Capability` (NOT magic string match like
//! `bus.rs:312-319` `manifest() == "wallet"` per spec § 2 hidden-input table).
//!
//! Constitution authority:
//! - WP arch § 5.L2: tool_id + capability + permission + determinism + side_effect schema
//! - Spec § 2 hidden-input table: explicit capability lookup; not string match
//!
//! v4 first iteration: typed metadata + register/lookup by Capability.
//! Tool EXECUTION still happens via existing `TuringTool` trait in `src/sdk/tool.rs`;
//! this registry is a META-LAYER mapping `Capability → ToolId → metadata`.
//!
//! /// TRACE_MATRIX WP-arch-§5.L2: ToolRegistry

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Tool capability — typed enum replacing magic string match in legacy code.
/// Per spec § 2 hidden-input table: bus.rs `manifest() == "wallet"` retired in CO1.1.4.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Capability {
    EconomicWallet,
    ProofValidator,
    NetworkClient,
    LeanOracle,
    LibrarianBoard,
    SearchTool,
    SandboxedExec,
    /// Custom capability with stable string ID (for v4.1+ extensibility).
    /// Use sparingly; prefer adding a new variant when capability is well-known.
    Custom(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionPolicy {
    /// Any agent may invoke.
    Open,
    /// Only the system runtime may invoke (e.g., system_keypair sign API).
    SystemOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeterminismClass {
    /// Same input → same output; no side effects. Safe in step_transition.
    Pure,
    /// Reads from external state but no writes. Safe-ish; result depends on read-time.
    ReadOnly,
    /// Idempotent writes (replay-safe).
    IdempotentWrite,
    /// Non-idempotent writes. FORBIDDEN in step_transition path (per spec § 2 I-NOSIDE).
    NonIdempotent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SideEffectClass {
    /// No side effects.
    None,
    /// Reads filesystem.
    FilesystemRead,
    /// Writes filesystem.
    FilesystemWrite,
    /// Network access.
    Network,
    /// Spawns subprocess.
    Subprocess,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub tool_id: String,
    pub version: u32,
    pub capability: Capability,
    pub permission_policy: PermissionPolicy,
    pub determinism_class: DeterminismClass,
    pub side_effect_class: SideEffectClass,
    pub schema: String,
    pub creator: String,
    /// SHA-256 of bytecode/source.
    pub code_hash: [u8; 32],
    /// SHA-256 of conformance test suite.
    pub test_suite_hash: [u8; 32],
    /// Reuse royalty share for AttributionEngine (CO P2.4 spike). Stored as MicroFraction.
    pub reuse_royalty_share_micro: i64,
}

impl ToolMetadata {
    pub fn canonical_hash(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(self.tool_id.as_bytes());
        h.update(self.version.to_be_bytes());
        h.update(serde_json::to_vec(&self.capability).expect("capability serialize"));
        h.update(serde_json::to_vec(&self.permission_policy).expect("perm serialize"));
        h.update(serde_json::to_vec(&self.determinism_class).expect("det serialize"));
        h.update(serde_json::to_vec(&self.side_effect_class).expect("se serialize"));
        h.update(self.schema.as_bytes());
        h.update(self.creator.as_bytes());
        h.update(self.code_hash);
        h.update(self.test_suite_hash);
        h.update(self.reuse_royalty_share_micro.to_be_bytes());
        h.finalize().into()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolRegistry {
    tools: BTreeMap<String, ToolMetadata>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegisterError {
    DuplicateId(String),
    InvalidId(String),
    /// `step_transition`-reachable code attempted to register a tool with NonIdempotent
    /// determinism class. Forbidden by spec § 2 I-NOSIDE.
    NonIdempotentNotAllowed(String),
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, meta: ToolMetadata) -> Result<(), RegisterError> {
        if meta.tool_id.is_empty() {
            return Err(RegisterError::InvalidId(meta.tool_id));
        }
        if self.tools.contains_key(&meta.tool_id) {
            return Err(RegisterError::DuplicateId(meta.tool_id));
        }
        // I-NOSIDE prerequisite: NonIdempotent tools cannot enter v4 registry.
        // (Future: relaxed for v4.1+ when we have explicit isolation; for now hard-block.)
        if meta.determinism_class == DeterminismClass::NonIdempotent {
            return Err(RegisterError::NonIdempotentNotAllowed(meta.tool_id));
        }
        self.tools.insert(meta.tool_id.clone(), meta);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&ToolMetadata> {
        self.tools.get(id)
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    /// Find tools matching a capability (replaces `manifest() == "wallet"` magic string).
    pub fn find_by_capability(&self, cap: &Capability) -> Vec<&ToolMetadata> {
        self.tools.values().filter(|m| &m.capability == cap).collect()
    }

    pub fn merkle_root(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        for (_id, meta) in &self.tools {
            h.update(meta.canonical_hash());
        }
        h.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_meta(id: &str, cap: Capability) -> ToolMetadata {
        ToolMetadata {
            tool_id: id.to_string(),
            version: 1,
            capability: cap,
            permission_policy: PermissionPolicy::Open,
            determinism_class: DeterminismClass::Pure,
            side_effect_class: SideEffectClass::None,
            schema: r#"{"input":"any","output":"any"}"#.to_string(),
            creator: "system".to_string(),
            code_hash: [0xcc; 32],
            test_suite_hash: [0xdd; 32],
            reuse_royalty_share_micro: 50_000, // 0.05 (5%)
        }
    }

    #[test]
    fn register_and_get_round_trip() {
        let mut reg = ToolRegistry::new();
        let m = sample_meta("wallet_v1", Capability::EconomicWallet);
        reg.register(m.clone()).unwrap();
        assert_eq!(reg.get("wallet_v1"), Some(&m));
    }

    #[test]
    fn duplicate_id_rejected() {
        let mut reg = ToolRegistry::new();
        let m = sample_meta("dup", Capability::EconomicWallet);
        reg.register(m.clone()).unwrap();
        assert_eq!(
            reg.register(m),
            Err(RegisterError::DuplicateId("dup".to_string()))
        );
    }

    #[test]
    fn empty_id_rejected() {
        let mut reg = ToolRegistry::new();
        let m = sample_meta("", Capability::EconomicWallet);
        assert_eq!(
            reg.register(m),
            Err(RegisterError::InvalidId("".to_string()))
        );
    }

    #[test]
    fn non_idempotent_rejected() {
        let mut reg = ToolRegistry::new();
        let mut m = sample_meta("bad_tool", Capability::Custom("test".into()));
        m.determinism_class = DeterminismClass::NonIdempotent;
        assert_eq!(
            reg.register(m),
            Err(RegisterError::NonIdempotentNotAllowed("bad_tool".to_string()))
        );
    }

    #[test]
    fn find_by_capability_replaces_magic_string() {
        let mut reg = ToolRegistry::new();
        reg.register(sample_meta("wallet_a", Capability::EconomicWallet)).unwrap();
        reg.register(sample_meta("wallet_b", Capability::EconomicWallet)).unwrap();
        reg.register(sample_meta("oracle", Capability::LeanOracle)).unwrap();

        let wallets = reg.find_by_capability(&Capability::EconomicWallet);
        assert_eq!(wallets.len(), 2, "two wallets registered");

        let oracles = reg.find_by_capability(&Capability::LeanOracle);
        assert_eq!(oracles.len(), 1);

        let nones = reg.find_by_capability(&Capability::NetworkClient);
        assert_eq!(nones.len(), 0);
    }

    #[test]
    fn merkle_root_deterministic() {
        let mut reg1 = ToolRegistry::new();
        let mut reg2 = ToolRegistry::new();
        for id in &["b", "a", "c"] {
            reg1.register(sample_meta(id, Capability::EconomicWallet)).unwrap();
        }
        for id in &["c", "a", "b"] {
            reg2.register(sample_meta(id, Capability::EconomicWallet)).unwrap();
        }
        assert_eq!(reg1.merkle_root(), reg2.merkle_root(),
            "BTreeMap-ordered; insertion order independent");
    }

    #[test]
    fn empty_registry() {
        let reg = ToolRegistry::new();
        assert!(reg.is_empty());
        assert_eq!(reg.len(), 0);
    }
}
