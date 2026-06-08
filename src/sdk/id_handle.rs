//! Shared content-hash ID-handle helper for the render/parse membrane.
//!
//! **Why this exists** (EXPLICIT_ID_HALLUCINATION_EXPOSURE_AUDIT_2026-06-08):
//! explicit/guessable ids (`worktx-…`, `Agent_0`, `true-suite-market-{run}`,
//! `slot_0`) interpolated into LLM-visible prompts are hallucination bait — a
//! model can fabricate or typo one and the parse layer may silently bind it.
//! The remedy is to show the agent an OPAQUE content-hash HANDLE (a sha256
//! short-prefix of the underlying canonical id/descriptor), and resolve the
//! agent's echoed handle back to the canonical id by EXACT membership in the
//! set the agent was shown — rejecting anything not in that set.
//!
//! **Scope**: render/parse membrane ONLY. The canonical identity types
//! (`TxId`/`AgentId`/`TaskId`/`EventId`), their minting in `q_state.rs`, the
//! typed-tx wire schema, sequencer admission, and signing payloads are
//! UNCHANGED. This helper never touches canonical state; callers keep keying
//! on the legacy strings internally and use a handle only at the render seam.
//!
//! **Handle form**: `sha256(domain || 0x1f || underlying)` hex, truncated to a
//! short display prefix. It is a HASH, never a slot index. Distinct underlying
//! ids ⇒ distinct handles; the same underlying id ⇒ the same handle
//! (deterministic, replay-stable). The optional `domain` tag scopes handles
//! per render surface (e.g. a self-identity handle vs a node handle) so the
//! same underlying string can yield surface-specific opaque tokens.

use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// TRACE_MATRIX FC1a-output_edge + FC1 rtool/wtool read-view shielding
/// (Art. III): default display length (hex chars) of a content-hash handle.
/// 16 hex chars = 64 bits of the sha256 digest — collision-safe for the small
/// per-run candidate sets rendered into a single prompt, while staying compact.
pub const HANDLE_PREFIX_LEN: usize = 16;

/// TRACE_MATRIX FC1a-output_edge + FC1 read-view shielding (Art. III): compute
/// a deterministic content-hash handle for an explicit id/descriptor at the
/// render membrane.
///
/// `domain` scopes the handle to a render surface (so a self-identity handle
/// and a node handle for the same string differ); pass `""` for an unscoped
/// handle. `underlying` is the canonical id string (e.g. a `TxId.0`,
/// `AgentId.0`, `TaskId.0`). The result is a `prefix_len`-char hex slice of
/// `sha256(domain || 0x1f || underlying)`.
///
/// This is a HASH, never a slot index. It is display/parse-membrane only and
/// does not mutate or re-key any canonical state.
pub fn id_handle(domain: &str, underlying: &str, prefix_len: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(domain.as_bytes());
    // 0x1f (unit separator) is not a valid id char in any canonical id form,
    // so domain||sep||underlying is injective in (domain, underlying).
    hasher.update([0x1fu8]);
    hasher.update(underlying.as_bytes());
    let digest = hasher.finalize();
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    let n = prefix_len.min(hex.len());
    hex[..n].to_string()
}

/// TRACE_MATRIX FC1a-output_edge: convenience wrapper over [`id_handle`] using
/// [`HANDLE_PREFIX_LEN`]. Use this for the common render-site case.
pub fn handle(domain: &str, underlying: &str) -> String {
    id_handle(domain, underlying, HANDLE_PREFIX_LEN)
}

/// TRACE_MATRIX FC1a-output_edge + FC1 read-view shielding (Art. III): a
/// render→resolve table for an "agent picks an id" membrane (audit fixes
/// 2/4/7/8).
///
/// Build it from the EXACT set of canonical ids the agent was shown; render
/// each id as its handle; then resolve the agent-echoed handle back to the
/// canonical id by EXACT-membership-or-REJECT. There is no fuzzy prefix match
/// and no `last()` fallback — an unknown handle returns `None` and the caller
/// must reject (drop the action / set parent to `None`), never silently bind a
/// wrong or default id.
#[derive(Debug, Clone, Default)]
pub struct HandleSet {
    domain: String,
    /// handle → canonical id string. BTreeMap for deterministic iteration.
    by_handle: BTreeMap<String, String>,
}

impl HandleSet {
    /// TRACE_MATRIX FC1a-output_edge: construct an empty handle set scoped to
    /// `domain`.
    pub fn new(domain: &str) -> Self {
        HandleSet {
            domain: domain.to_string(),
            by_handle: BTreeMap::new(),
        }
    }

    /// TRACE_MATRIX FC1a-output_edge: register a canonical id and return the
    /// handle the agent will see for it. Idempotent for the same id.
    pub fn insert(&mut self, canonical_id: &str) -> String {
        let h = handle(&self.domain, canonical_id);
        self.by_handle.insert(h.clone(), canonical_id.to_string());
        h
    }

    /// TRACE_MATRIX FC1a-output_edge: the handle for a canonical id WITHOUT
    /// registering it (pure display).
    pub fn handle_for(&self, canonical_id: &str) -> String {
        handle(&self.domain, canonical_id)
    }

    /// TRACE_MATRIX FC1a-output_edge + FC1 read-view shielding (Art. III):
    /// resolve an agent-echoed token back to a canonical id by
    /// EXACT-membership-or-REJECT.
    ///
    /// Accepts the token ONLY if it byte-equals a registered handle. As a
    /// convenience the agent may also echo the canonical id verbatim IF that
    /// exact id is a member of the rendered set (some prompts still show ids in
    /// other blocks); this is still exact membership, never a prefix or
    /// fallback. Anything else returns `None` and the caller MUST reject.
    pub fn resolve(&self, echoed: &str) -> Option<String> {
        if let Some(canonical) = self.by_handle.get(echoed) {
            return Some(canonical.clone());
        }
        // Exact canonical-id membership (NOT a prefix match): only if the
        // echoed string IS one of the registered canonical ids verbatim.
        if self.by_handle.values().any(|c| c == echoed) {
            return Some(echoed.to_string());
        }
        None
    }

    /// TRACE_MATRIX FC1a-output_edge: number of registered handles.
    pub fn len(&self) -> usize {
        self.by_handle.len()
    }

    /// TRACE_MATRIX FC1a-output_edge: whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.by_handle.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_is_deterministic_and_hex() {
        let a = handle("node", "worktx-abc123");
        let b = handle("node", "worktx-abc123");
        assert_eq!(a, b, "same id ⇒ same handle (replay-stable)");
        assert_eq!(a.len(), HANDLE_PREFIX_LEN);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()), "hex only");
    }

    #[test]
    fn distinct_ids_give_distinct_handles() {
        let a = handle("node", "worktx-aaa");
        let b = handle("node", "worktx-bbb");
        assert_ne!(a, b, "distinct ids ⇒ distinct handles");
    }

    #[test]
    fn handle_is_not_a_slot_index() {
        // A slot index like "node_0"/"slot_0" is an explicit guessable id; the
        // handle must NOT look like one (it is a hash hex string).
        let h = handle("node", "worktx-0");
        assert!(!h.starts_with("node_"));
        assert!(!h.starts_with("slot_"));
        assert_ne!(h, "0");
    }

    #[test]
    fn domain_scopes_the_handle() {
        let self_h = handle("self", "Agent_0");
        let node_h = handle("node", "Agent_0");
        assert_ne!(self_h, node_h, "domain scopes the handle for the same id");
    }

    #[test]
    fn handle_set_resolves_exact_handle_only() {
        let mut set = HandleSet::new("node");
        let h_a = set.insert("worktx-aaa");
        let h_b = set.insert("worktx-bbb");
        assert_eq!(set.resolve(&h_a).as_deref(), Some("worktx-aaa"));
        assert_eq!(set.resolve(&h_b).as_deref(), Some("worktx-bbb"));
    }

    #[test]
    fn handle_set_resolves_exact_canonical_id_membership() {
        let mut set = HandleSet::new("node");
        set.insert("worktx-aaa");
        // The agent may echo the canonical id verbatim IF it is a member.
        assert_eq!(set.resolve("worktx-aaa").as_deref(), Some("worktx-aaa"));
    }

    #[test]
    fn handle_set_rejects_unknown_handle_no_fallback() {
        let mut set = HandleSet::new("node");
        set.insert("worktx-aaa");
        set.insert("worktx-bbb");
        // A fabricated/typoed token resolves to None — NO last() fallback, NO
        // fuzzy prefix bind.
        assert_eq!(set.resolve("deadbeef"), None);
        assert_eq!(set.resolve("worktx"), None, "prefix must NOT match");
        assert_eq!(set.resolve("worktx-ccc"), None, "non-member id rejected");
    }

    #[test]
    fn handle_set_rejects_prefix_of_a_handle() {
        let mut set = HandleSet::new("node");
        let h = set.insert("worktx-aaa");
        let prefix = &h[..h.len() - 1];
        assert_eq!(set.resolve(prefix), None, "handle prefix must NOT match");
    }
}
