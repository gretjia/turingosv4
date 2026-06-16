//! Tier-1 SkillCapsule — SYSTEM-authored, AGENT-read-only distilled memory.
//!
//! A `SkillCapsule` is the system's distilled, reusable rule extracted from
//! failure/feedback evidence (the already-shielded, tape-derived
//! `TypicalErrorSummary` clusters produced by
//! `autopsy_capsule::cluster_autopsies`). The system *consolidates* a capsule:
//! reads failure evidence → distills one rule → writes it to CAS → the CAS
//! `put` advances `refs/chaintape/cas` (the canonical tape per Art. 0.2), so
//! the write IS the L4 anchor. Capsules chain via `previous_capsule_cid`,
//! exactly like `MarkovEvidenceCapsule`.
//!
//! ## Tier boundary (constitutional)
//!
//! - **Tier-1 (this module, within authority):** the SYSTEM authors a capsule
//!   from failure/feedback evidence and an AGENT reads a scoped, shielded
//!   projection. `author` is always `SYSTEM`. There is NO agent-callable
//!   mutate path — the only write entry point (`consolidate_skill_capsule`)
//!   takes already-shielded `TypicalErrorSummary` input (system-derived) and
//!   stamps `author = SkillAuthor::System`. The agent read surface
//!   (`project_for_agent`) returns an owned, immutable
//!   `AgentSkillProjection`; it cannot write back.
//! - **Tier-2 (NOT implemented here, needs separate §8):** an agent-writable
//!   memory path. This module deliberately exposes no such surface.
//!
//! ## Tape canonical (Art. 0.2)
//!
//! The canonical store is the TAPE/CAS, never the filesystem. This module
//! never calls `std::fs::write`; `consolidate_skill_capsule` persists ONLY
//! through `CasStore::put`, whose commit-chain (`refs/chaintape/cas`) is the
//! canonical evidence ledger. A capsule is fully reconstructable from CAS +
//! tape alone (`restore_skill_capsule_from_cas_bytes` over
//! `cas.get(&capsule_id)`).
//!
//! ## Schema without a pinned edit
//!
//! `cas/schema.rs` (the `ObjectType` enum) is a trust-root-pinned file. Rather
//! than add a new enum variant there, a SkillCapsule is stored as
//! `ObjectType::Generic` with the free-form `schema_id`
//! `SKILL_CAPSULE_SCHEMA_ID` (`"v1/skill_capsule"`). `schema_id` participates
//! in `CasObjectMetadata::canonical_hash`, so it is collision-distinct from
//! other `Generic` blobs and audit-discoverable via
//! `CasStore::list_cids_by_schema_id`.
//!
//! TRACE_MATRIX FC3-N43 (meta-architecture feedback → re-init: distilled
//! reusable rules feed the next session's boot context) + Art. 0.2 (Tape
//! Canonical: capsule canonical bytes ARE the CAS object referenced by
//! `capsule_id`; CAS commit-chain is the L4 anchor) + CR-15.5 (capsules are
//! evidence compression, not hidden source of truth — every field is derivable
//! from the chain + CAS) + Inv 10 (Goodhart shield: the agent read-projection
//! omits raw failure bytes / private detail).

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::transition_ledger::canonical_encode;
use crate::runtime::autopsy_capsule::TypicalErrorSummary;
use crate::state::q_state::Hash;

/// TRACE_MATRIX Art. 0.2 + CR-15.5: free-form CAS `schema_id` for a
/// SkillCapsule. Used in lieu of a (pinned) `ObjectType` enum variant; it is
/// part of `CasObjectMetadata::canonical_hash`, so it is collision-distinct
/// from other `ObjectType::Generic` blobs and discoverable via
/// `CasStore::list_cids_by_schema_id(SKILL_CAPSULE_SCHEMA_ID)`.
/// TRACE_MATRIX Art. 0.2: free-form CAS schema_id tag for a SkillCapsule object.
pub const SKILL_CAPSULE_SCHEMA_ID: &str = "v1/skill_capsule";

/// TRACE_MATRIX FC3-N43 + Inv 10 (Tier-1 boundary): authorship marker. A
/// SkillCapsule is ALWAYS system-authored. The enum carries a single variant
/// by design — there is intentionally no `Agent` variant, structurally
/// enforcing the Tier-1 "system-authored, agent-read-only" boundary. Adding an
/// agent-writable author would be a Tier-2 change requiring a separate §8.
/// TRACE_MATRIX FC3-N43 + Inv 10: capsule authorship marker (always System).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub enum SkillAuthor {
    /// The runtime/system distilled this capsule from failure/feedback
    /// evidence. The ONLY legal author.
    #[default]
    System,
}

impl SkillAuthor {
    /// Stable string tag for audit rendering.
    ///
    /// TRACE_MATRIX FC3-N43: authorship is a provenance field, not a
    /// behavior parameter.
    pub fn tag(&self) -> &str {
        match self {
            Self::System => "SYSTEM",
        }
    }
}

/// TRACE_MATRIX FC3-N43 + Art. 0.2 + CR-15.5: SYSTEM-authored distilled
/// memory capsule. CAS-resident; chained via `previous_capsule_cid` (mirrors
/// `MarkovEvidenceCapsule`).
///
/// **Provenance (CR-15.5)**: every field is derivable from the chain + CAS at
/// consolidation time. `source_failure_cids` are pre-existing public capsule
/// Cids (the `exemplar_capsule_cids` of the `TypicalErrorSummary` clusters);
/// the capsule does NOT mint or duplicate raw failure bytes.
///
/// **Shielding (Inv 10)**: the capsule carries only `distilled_rule` text +
/// provenance Cids. Raw stderr / autopsy / private-detail bytes are never
/// stored here. The agent read view (`AgentSkillProjection`) is a further
/// scoped projection of this struct.
/// TRACE_MATRIX FC3-N43 + Art. 0.2 + CR-15.5: system-authored distilled memory capsule, CAS-resident and chained.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillCapsule {
    /// CAS Cid of this capsule's canonical bytes (with `capsule_id` +
    /// `sha256` zeroed during the hash, per the markov R3 discipline).
    /// Computed by the writer.
    /// TRACE_MATRIX Art. 0.2: self-addressing CAS Cid (capsule canonical bytes).
    pub capsule_id: Cid,

    /// Cid of the previous SkillCapsule in the chain. `None` for the first
    /// ever capsule (genesis skill). This is the L4-chained lineage.
    /// TRACE_MATRIX FC3-N43: previous-capsule Cid forming the L4-chained skill lineage.
    pub previous_capsule_cid: Option<Cid>,

    /// SHA-256 of `constitution.md` bytes at consolidation time. Binds the
    /// distilled rule to the constitution version it was derived under
    /// (mirrors `MarkovEvidenceCapsule.constitution_hash`).
    /// TRACE_MATRIX FC3-N43: constitution-version binding for the distilled rule.
    pub constitution_hash: Hash,

    /// Tape head (e.g. L4 root / CAS root) the system read failure evidence
    /// from at consolidation time. Strictly informational provenance anchor;
    /// CR-15.5 — the capsule references the chain, it is not a second source
    /// of truth.
    /// TRACE_MATRIX CR-15.5 + Art. 0.2: provenance anchor for the tape head evidence was read from.
    pub source_run_head: Hash,

    /// Cids of the failure-evidence objects this rule was distilled from —
    /// the `exemplar_capsule_cids` of the contributing `TypicalErrorSummary`
    /// clusters. Pre-existing public CAS objects only; never raw private
    /// bytes (Inv 10).
    /// TRACE_MATRIX CR-15.5 + Inv 10: provenance Cids of the public failure exemplars distilled from.
    pub source_failure_cids: Vec<Cid>,

    /// The distilled, reusable rule text. Low-information, agent-surfaceable
    /// (this is the field that flows into `AgentSkillProjection`).
    /// TRACE_MATRIX FC3-N43 + Inv 10: the agent-surfaceable distilled rule text.
    pub distilled_rule: String,

    /// Scope tags this rule applies to (e.g. loss-reason-class tags, or a
    /// task/module label). The agent read view is filtered against this set,
    /// so an agent only sees rules applicable to its current scope.
    /// TRACE_MATRIX Art. III.2: scope tags gating which agents may read this rule.
    pub applicable_scope: Vec<String>,

    /// Integer confidence signal (count of distinct failure exemplars that
    /// support this rule). **Integer math only** — no `f64`/`f32` (money/
    /// signal-path rule). Higher = more corroborated.
    /// TRACE_MATRIX FC3-N43: integer corroboration count (no f64 in signal path).
    pub confidence_signal: u32,

    /// Authorship marker — ALWAYS `SkillAuthor::System`. Structural Tier-1
    /// boundary witness.
    /// TRACE_MATRIX Inv 10 (Tier-1 boundary): structural system-authored witness.
    pub author: SkillAuthor,

    /// SHA-256 of this capsule's canonical bytes (zeroed-field variant).
    /// Defense-in-depth duplicate of `capsule_id`.
    /// TRACE_MATRIX Art. 0.2: defense-in-depth hash of the zeroed-field canonical bytes.
    pub sha256: Hash,

    /// Logical time at consolidation (sequencer- or generator-supplied).
    /// TRACE_MATRIX FC3-N43: logical timestamp at consolidation.
    pub created_at_logical_t: u64,

    /// Schema tag — `SKILL_CAPSULE_SCHEMA_ID`. Strictly informational
    /// (the authoritative schema discriminator is the CAS metadata
    /// `schema_id`); duplicated in-struct for self-describing replay.
    /// TRACE_MATRIX Art. 0.2: in-struct schema tag for self-describing replay.
    pub schema_tag: String,
}

impl Default for SkillCapsule {
    fn default() -> Self {
        Self {
            capsule_id: Cid::default(),
            previous_capsule_cid: None,
            constitution_hash: Hash::ZERO,
            source_run_head: Hash::ZERO,
            source_failure_cids: Vec::new(),
            distilled_rule: String::new(),
            applicable_scope: Vec::new(),
            confidence_signal: 0,
            author: SkillAuthor::System,
            sha256: Hash::ZERO,
            created_at_logical_t: 0,
            schema_tag: SKILL_CAPSULE_SCHEMA_ID.to_string(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Consolidation error taxonomy
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-N43: consolidation error taxonomy (mirrors
/// `MarkovGenError`).
#[derive(Debug)]
pub enum SkillConsolidationError {
    Cas(crate::bottom_white::cas::store::CasError),
    Encode(String),
    InternalLockPoisoned,
}

impl std::fmt::Display for SkillConsolidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cas(e) => write!(f, "cas: {e}"),
            Self::Encode(s) => write!(f, "encode: {s}"),
            Self::InternalLockPoisoned => write!(f, "internal lock poisoned"),
        }
    }
}
impl std::error::Error for SkillConsolidationError {}

impl From<crate::bottom_white::cas::store::CasError> for SkillConsolidationError {
    fn from(e: crate::bottom_white::cas::store::CasError) -> Self {
        Self::Cas(e)
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Distillation (pure) + consolidation (CAS-writing, system-authored)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-N43 + Inv 10: PURE distillation of a `SkillCapsule` body
/// from already-shielded failure clusters. No CAS access, no env, no clock,
/// no `capsule_id`/`sha256` population. Used by `consolidate_skill_capsule`
/// (and directly testable for replay-determinism).
///
/// The distilled rule is a deterministic, low-information summary over the
/// input `TypicalErrorSummary` clusters: it joins each cluster's
/// `exemplar_public_summary` (which is itself the shielded `public_summary`
/// text per the autopsy privacy contract — NEVER private-detail bytes). The
/// `confidence_signal` is the integer sum of cluster counts; `applicable_scope`
/// is the de-duplicated set of cluster loss-reason-class tags;
/// `source_failure_cids` are the clusters' `exemplar_capsule_cids`.
///
/// **CR-15.5**: the body references existing public evidence; it mints no new
/// ground truth. **Inv 10**: only `public_summary`-derived text + capsule Cids
/// enter the body.
/// TRACE_MATRIX FC3-N43 + Inv 10: pure, deterministic distillation of a capsule body from shielded clusters.
pub fn distill_skill_capsule_body(
    previous_capsule_cid: Option<Cid>,
    constitution_hash: Hash,
    source_run_head: Hash,
    typical_errors: &[TypicalErrorSummary],
    created_at_logical_t: u64,
) -> SkillCapsule {
    use std::collections::BTreeSet;

    // De-duplicated, sorted scope tags for replay-determinism.
    let mut scope_set: BTreeSet<String> = BTreeSet::new();
    let mut source_failure_cids: Vec<Cid> = Vec::new();
    let mut rule_parts: Vec<String> = Vec::new();
    let mut confidence: u32 = 0;

    for cluster in typical_errors {
        let tag = cluster.loss_reason_class.tag().to_string();
        scope_set.insert(tag.clone());
        confidence = confidence.saturating_add(cluster.count);
        // Provenance: the cluster's contributing capsule Cids (public).
        source_failure_cids.extend(cluster.exemplar_capsule_cids.iter().copied());
        // Distilled rule fragment: shielded public summary only.
        rule_parts.push(format!(
            "[{}×{}] avoid: {}",
            tag, cluster.count, cluster.exemplar_public_summary
        ));
    }

    let distilled_rule = if rule_parts.is_empty() {
        "no distillable failure cluster (insufficient corroboration)".to_string()
    } else {
        rule_parts.join(" | ")
    };

    SkillCapsule {
        capsule_id: Cid::default(),
        previous_capsule_cid,
        constitution_hash,
        source_run_head,
        source_failure_cids,
        distilled_rule,
        applicable_scope: scope_set.into_iter().collect(),
        confidence_signal: confidence,
        author: SkillAuthor::System,
        sha256: Hash::ZERO,
        created_at_logical_t,
        schema_tag: SKILL_CAPSULE_SCHEMA_ID.to_string(),
    }
}

/// TRACE_MATRIX FC3-N43 + Art. 0.2 (SYSTEM consolidation path): read
/// failure/feedback evidence (already-shielded `TypicalErrorSummary`
/// clusters), distill a `SkillCapsule`, write it to CAS, and (via the CAS
/// commit-chain `refs/chaintape/cas`) anchor it on the tape. Chained via
/// `previous_capsule_cid`.
///
/// **System-authored, no agent-write path**: the signature takes
/// system-derived shielded input and stamps `author = SkillAuthor::System`.
/// There is NO variant that lets an agent write its own memory. `creator_str`
/// is the CAS metadata creator (use `"system"` for runtime emission).
///
/// **R3 self-addressing discipline** (cloned verbatim from
/// `write_markov_capsule`): the CAS-stored bytes have `capsule_id`/`sha256`
/// ZEROED, so `Cid::from_content(stored_bytes) == capsule_id` and
/// `cas.get(&capsule_id)` resolves. The returned in-memory struct has them
/// populated for ergonomic use.
///
/// **Tape canonical (Art. 0.2)**: persistence is ONLY via `CasStore::put` —
/// no `std::fs::write` anywhere. `put` advances `refs/chaintape/cas`
/// fail-closed; if the chain update fails, the capsule is not accepted.
/// TRACE_MATRIX FC3-N43 + Art. 0.2: system consolidation path — distill, CAS-write, tape-anchor.
#[allow(clippy::too_many_arguments)]
pub fn consolidate_skill_capsule(
    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
    previous_capsule_cid: Option<Cid>,
    constitution_hash: Hash,
    source_run_head: Hash,
    typical_errors: &[TypicalErrorSummary],
    creator_str: &str,
    created_at_logical_t: u64,
) -> Result<SkillCapsule, SkillConsolidationError> {
    let mut capsule = distill_skill_capsule_body(
        previous_capsule_cid,
        constitution_hash,
        source_run_head,
        typical_errors,
        created_at_logical_t,
    );

    let mut cas_w = cas
        .write()
        .map_err(|_| SkillConsolidationError::InternalLockPoisoned)?;

    // R3 closure: encode with capsule_id/sha256 zeroed; the stored bytes' Cid
    // IS the capsule_id, guaranteeing cas.get(&capsule_id) resolvability.
    let stored_bytes = canonical_encode(&capsule)
        .map_err(|e| SkillConsolidationError::Encode(format!("capsule canonical encode: {e:?}")))?;
    let cid = Cid::from_content(&stored_bytes);
    let cas_returned_cid = cas_w.put(
        &stored_bytes,
        ObjectType::Generic,
        creator_str,
        created_at_logical_t,
        Some(SKILL_CAPSULE_SCHEMA_ID.to_string()),
    )?;
    debug_assert_eq!(
        cas_returned_cid, cid,
        "CAS-returned cid must equal sha256(stored_bytes); CasStore::put contract"
    );
    capsule.capsule_id = cid;
    capsule.sha256 = Hash(cid.0);

    Ok(capsule)
}

/// TRACE_MATRIX Art. 0.2 (reconstructability): rebuild a `SkillCapsule` from
/// CAS-resident bytes. Caller supplies the bytes returned by
/// `cas.get(&capsule_id)`; this canonical-decodes them and re-derives
/// `capsule_id` + `sha256` from `Cid::from_content(&bytes)` — yielding the
/// ergonomic in-memory view identical to `consolidate_skill_capsule`'s return.
///
/// Invariant: for any capsule written by `consolidate_skill_capsule`,
/// `restore_skill_capsule_from_cas_bytes(cas.get(&cap.capsule_id)?) == cap`.
/// TRACE_MATRIX Art. 0.2 (reconstructability): rebuild a capsule from CAS-resident bytes.
pub fn restore_skill_capsule_from_cas_bytes(
    bytes: &[u8],
) -> Result<SkillCapsule, SkillConsolidationError> {
    use crate::bottom_white::ledger::transition_ledger::canonical_decode;
    let mut cap: SkillCapsule = canonical_decode(bytes)
        .map_err(|e| SkillConsolidationError::Encode(format!("capsule decode: {e:?}")))?;
    let cid = Cid::from_content(bytes);
    cap.capsule_id = cid;
    cap.sha256 = Hash(cid.0);
    Ok(cap)
}

// ────────────────────────────────────────────────────────────────────────────
// Agent read-projection (scoped + shielded, READ-ONLY)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX Art. III.2 (read-view scoping) + Inv 10 (Goodhart shield):
/// the agent-visible projection of one or more applicable `SkillCapsule`s.
///
/// Carries ONLY the distilled rule text + provenance Cids — never the raw
/// failure bytes, never the private-detail Cids, never the full capsule struct.
/// It is an owned, immutable value handed to the agent: the agent CANNOT write
/// back through it (no `&mut`, no CAS handle), structurally enforcing the
/// Tier-1 agent-read-only boundary.
/// TRACE_MATRIX Art. III.2 + Inv 10: scoped, shielded, read-only agent projection of capsule rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSkillProjection {
    /// Distilled rule texts applicable to the requested scope, in capsule
    /// order. Shielded — derived only from `SkillCapsule.distilled_rule`.
    /// TRACE_MATRIX Inv 10: shielded distilled-rule texts in scope, capsule order.
    pub applicable_rules: Vec<String>,
    /// Provenance: capsule Cids the rules came from (audit can fetch them).
    /// NEVER the `source_failure_cids` raw private bytes — only the capsule's
    /// own self-address.
    /// TRACE_MATRIX Inv 10: provenance capsule self-Cids only (never raw failure bytes).
    pub source_capsule_cids: Vec<Cid>,
}

/// TRACE_MATRIX Art. III.2 + Inv 10: project a single `SkillCapsule` into the
/// scoped, shielded agent read view IFF the capsule's `applicable_scope`
/// intersects the requested `scope`. Returns `None` when out of scope
/// (default-deny: an agent only sees rules applicable to its current scope).
///
/// **Shielding**: emits `distilled_rule` + `capsule_id` only. Does NOT emit
/// `source_failure_cids`, `source_run_head`, `constitution_hash`, or any raw
/// failure bytes — the agent gets the rule, not the underlying private
/// evidence.
/// TRACE_MATRIX Art. III.2 + Inv 10: scope-gated single-capsule projection (default-deny out of scope).
pub fn project_capsule_for_agent(
    capsule: &SkillCapsule,
    scope: &[String],
) -> Option<AgentSkillProjection> {
    let in_scope = scope
        .iter()
        .any(|s| capsule.applicable_scope.iter().any(|c| c == s));
    if !in_scope {
        return None;
    }
    Some(AgentSkillProjection {
        applicable_rules: vec![capsule.distilled_rule.clone()],
        source_capsule_cids: vec![capsule.capsule_id],
    })
}

/// TRACE_MATRIX Art. III.2 + Inv 10: project a set of `SkillCapsule`s into one
/// merged agent read view, filtered by `scope`. Out-of-scope capsules are
/// dropped (default-deny). Preserves input order for replay-determinism.
///
/// This is the surface a boot/read path hands an agent: a scoped, shielded,
/// READ-ONLY view of applicable distilled rules. No CAS handle, no `&mut`, no
/// write-back — the agent cannot author or mutate memory through it.
/// TRACE_MATRIX Art. III.2 + Inv 10: merged scope-filtered read-only projection over many capsules.
pub fn project_for_agent(capsules: &[SkillCapsule], scope: &[String]) -> AgentSkillProjection {
    let mut applicable_rules = Vec::new();
    let mut source_capsule_cids = Vec::new();
    for capsule in capsules {
        if let Some(p) = project_capsule_for_agent(capsule, scope) {
            applicable_rules.extend(p.applicable_rules);
            source_capsule_cids.extend(p.source_capsule_cids);
        }
    }
    AgentSkillProjection {
        applicable_rules,
        source_capsule_cids,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::autopsy_capsule::LossReasonClass;

    fn cluster(tag: LossReasonClass, count: u32, exemplar_byte: u8) -> TypicalErrorSummary {
        TypicalErrorSummary {
            loss_reason_class: tag,
            count,
            exemplar_public_summary: "agent=A lost 100μC reason=X".to_string(),
            exemplar_capsule_cids: vec![Cid([exemplar_byte; 32])],
        }
    }

    /// SkillCapsule default round-trips through canonical bytes.
    #[test]
    fn skill_capsule_default_round_trip() {
        use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
        let c = SkillCapsule::default();
        let bytes = canonical_encode(&c).expect("encode");
        let back: SkillCapsule = canonical_decode(&bytes).expect("decode");
        assert_eq!(c, back);
    }

    /// distill is pure + deterministic; confidence is integer sum of counts;
    /// scope is the de-duplicated class-tag set; author is System.
    #[test]
    fn distill_is_deterministic_integer_confidence_system_author() {
        let errors = vec![
            cluster(LossReasonClass::Bankruptcy, 3, 0xAA),
            cluster(LossReasonClass::SlashLoss, 4, 0xBB),
            cluster(LossReasonClass::Bankruptcy, 2, 0xCC), // same class again
        ];
        let a = distill_skill_capsule_body(None, Hash([0x01; 32]), Hash([0x02; 32]), &errors, 7);
        let b = distill_skill_capsule_body(None, Hash([0x01; 32]), Hash([0x02; 32]), &errors, 7);
        assert_eq!(a, b, "distill must be deterministic");
        assert_eq!(a.confidence_signal, 9, "3 + 4 + 2 = 9 (integer sum)");
        // De-duplicated class tags, sorted: Bankruptcy, SlashLoss.
        assert_eq!(
            a.applicable_scope,
            vec!["Bankruptcy".to_string(), "SlashLoss".to_string()]
        );
        assert_eq!(a.author, SkillAuthor::System);
        assert_eq!(a.source_failure_cids.len(), 3);
    }

    /// project_for_agent filters by scope (default-deny out of scope) and
    /// emits ONLY rule text + capsule Cid — never source_failure_cids.
    #[test]
    fn projection_is_scoped_and_shielded() {
        let mut cap = distill_skill_capsule_body(
            None,
            Hash([0x01; 32]),
            Hash([0x02; 32]),
            &[cluster(LossReasonClass::Bankruptcy, 3, 0xAA)],
            1,
        );
        cap.capsule_id = Cid([0x11; 32]);

        // In scope → rule surfaces.
        let p_in = project_for_agent(&[cap.clone()], &["Bankruptcy".to_string()]);
        assert_eq!(p_in.applicable_rules.len(), 1);
        assert_eq!(p_in.source_capsule_cids, vec![Cid([0x11; 32])]);

        // Out of scope → default-deny (empty).
        let p_out = project_for_agent(&[cap.clone()], &["SlashLoss".to_string()]);
        assert!(p_out.applicable_rules.is_empty());
        assert!(p_out.source_capsule_cids.is_empty());

        // Shielded: the source_failure_cid (0xAA run) MUST NOT appear in the
        // serialized projection.
        let bytes = serde_json::to_vec(&p_in).expect("serialize projection");
        let private_run = [0xAAu8; 32];
        for window in bytes.windows(32) {
            assert!(
                window != private_run,
                "projection must not embed source_failure_cid bytes (Inv 10 shield)"
            );
        }
    }
}
