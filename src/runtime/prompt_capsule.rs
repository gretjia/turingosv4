//! Constitution Landing First 2026-05-07 (HARNESS.md §3 G-016/G-019/G-021/
//! G-028; architect ruling 2026-05-07): Class-3 `PromptCapsule` CAS schema.
//!
//! ## Why
//!
//! Art. III (selective shielding) plus the prompt-persistence gates demand a
//! tape-resident witness that the prompt context delivered to the agent was
//!
//!   - derivable from a fixed `read_set`,
//!   - shaped by a versioned redaction policy,
//!   - rendered against a known system-prompt template,
//!
//! WITHOUT putting verbatim prompt bytes onto canonical tape. The architect's
//! CLAUDE.md §4.3 ruling pins the schema: verbatim prompt may exist only as
//! an encrypted / audit-only Class-4 artifact requiring explicit ratification.
//! `PromptCapsule` is the Class-3 default that lets tape audit answer "what
//! did the agent see?" without leaking hidden fields.
//!
//! ## Schema (architect-pinned 2026-05-07)
//!
//! ```text
//! PromptCapsule {
//!   prompt_context_hash,         // sha256 of canonical visible context bytes
//!   read_set,                    // ReadKey set the agent could observe
//!   policy_version,              // opaque redaction-policy version
//!   hidden_fields_redacted,      // invariant assertion (must be true)
//!   visible_context_cid,         // CAS CID of the visible context bytes
//!   system_prompt_template_hash, // sha256 of system-prompt template bytes
//!   agent_view_manifest_cid,     // CAS CID of per-key view manifest JSON
//! }
//! ```
//!
//! `AttemptTelemetry.prompt_context_hash` MUST equal
//! `PromptCapsule.prompt_context_hash`; the constitution gate
//! `prompt_capsule_referenced_by_attempt_telemetry` pins this identity.
//!
//! `FC-trace: FC1-INV1 + Art-III + G-016/G-019/G-021/G-028`.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::bottom_white::ledger::transition_ledger::{
    canonical_decode, canonical_encode, CanonicalCodecError,
};
use crate::state::q_state::Hash;
use crate::state::typed_tx::ReadKey;

/// TRACE_MATRIX FC1-N44: schema id constant for `PromptCapsule` CAS objects.
///
/// Schema version discrimination lives in this string + the
/// `ObjectType::PromptCapsule` CAS metadata sidecar — NOT inside the
/// canonical 7-field shape (architect §4.3 ruling: the capsule's payload
/// contains exactly 7 named fields, no implementation-side `schema_version`
/// discriminator).
pub const PROMPT_CAPSULE_SCHEMA_ID: &str = "v1/prompt_capsule";

/// TRACE_MATRIX FC1-N44: tape-resident prompt capsule (architect schema,
/// 2026-05-07 ruling §4.3).
///
/// Architect-pinned 7-field shape. Adding any field — including a
/// `schema_version` discriminator — breaks the shape gate
/// `prompt_capsule_struct_field_count_is_exactly_seven`. Schema-version
/// discrimination lives in `PROMPT_CAPSULE_SCHEMA_ID` + the CAS sidecar
/// metadata `schema_id` slot, not in the canonical payload.
///
/// Privacy invariant: this object MUST NOT carry verbatim prompt bytes. The
/// constructor is the only public way to build a capsule and asserts
/// `hidden_fields_redacted == true`. Verbatim bytes belong in a separate
/// Class-4 AuditOnly artifact with explicit ratification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptCapsule {
    /// SHA-256 of the canonical-encoded visible context bytes delivered to
    /// the LLM (after redaction). Matches
    /// `AttemptTelemetry.prompt_context_hash`.
    pub prompt_context_hash: Hash,
    /// Set of keys the agent could read at construction time (mirrors
    /// `WorkTx.read_set`). Empty set is legal (agent saw only system prompt).
    pub read_set: BTreeSet<ReadKey>,
    /// Opaque version identifier of the redaction / visibility policy.
    /// Bump when the policy mutates so audit can replay the right policy.
    pub policy_version: String,
    /// Invariant assertion: all fields the policy classifies as "hidden"
    /// were redacted before computing `prompt_context_hash`. Constructor
    /// rejects `false`.
    pub hidden_fields_redacted: bool,
    /// CID of the canonical-encoded visible context bytes (the bytes whose
    /// sha256 is `prompt_context_hash`). Lets audit re-derive the context.
    pub visible_context_cid: Cid,
    /// SHA-256 of the system-prompt template bytes (separate from the
    /// dynamic visible context). Pins template-version drift.
    pub system_prompt_template_hash: Hash,
    /// CID of a per-key JSON manifest enumerating the agent's view at
    /// construction time. Audit re-walks the manifest to verify the
    /// `read_set` reflects on-tape state at that point.
    pub agent_view_manifest_cid: Cid,
}

/// TRACE_MATRIX FC1-N44: construction / CAS / codec errors for `PromptCapsule`.
#[derive(Debug)]
pub enum PromptCapsuleError {
    /// Constructor refused to build a capsule with `hidden_fields_redacted=false`.
    /// Per architect §4.3: redaction is mandatory before tape persistence.
    HiddenFieldsNotRedacted,
    Codec(String),
    Cas(CasError),
}

impl std::fmt::Display for PromptCapsuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HiddenFieldsNotRedacted => write!(
                f,
                "PromptCapsule constructor refused: hidden_fields_redacted must be true \
                 (architect ruling 2026-05-07 §4.3 — verbatim prompt is Class-4 audit-only)"
            ),
            Self::Codec(e) => write!(f, "PromptCapsule canonical_codec failed: {e}"),
            Self::Cas(e) => write!(f, "PromptCapsule CAS error: {e}"),
        }
    }
}

impl std::error::Error for PromptCapsuleError {}

impl From<CanonicalCodecError> for PromptCapsuleError {
    fn from(e: CanonicalCodecError) -> Self {
        Self::Codec(e.to_string())
    }
}

impl From<CasError> for PromptCapsuleError {
    fn from(e: CasError) -> Self {
        Self::Cas(e)
    }
}

impl PromptCapsule {
    /// TRACE_MATRIX FC1-N44: construct a capsule. Refuses
    /// `hidden_fields_redacted == false` per architect §4.3 — this is the
    /// only privacy-invariant gate at construction time.
    ///
    /// Caller is expected to compute `prompt_context_hash` from the same
    /// canonical bytes whose CID is `visible_context_cid`. The constructor
    /// does not verify the bytes against the hash because that requires CAS
    /// access; the gate `prompt_capsule_hash_stable` exercises the consistent
    /// path on round-trip.
    pub fn new(
        prompt_context_hash: Hash,
        read_set: BTreeSet<ReadKey>,
        policy_version: impl Into<String>,
        hidden_fields_redacted: bool,
        visible_context_cid: Cid,
        system_prompt_template_hash: Hash,
        agent_view_manifest_cid: Cid,
    ) -> Result<Self, PromptCapsuleError> {
        if !hidden_fields_redacted {
            return Err(PromptCapsuleError::HiddenFieldsNotRedacted);
        }
        Ok(Self {
            prompt_context_hash,
            read_set,
            policy_version: policy_version.into(),
            hidden_fields_redacted,
            visible_context_cid,
            system_prompt_template_hash,
            agent_view_manifest_cid,
        })
    }

    /// TRACE_MATRIX FC1-N44: canonical hash of the capsule (sha256 of its
    /// canonical encoding). Idempotent on identical input → identical bytes;
    /// `prompt_capsule_hash_stable` gate relies on this.
    pub fn canonical_hash(&self) -> Result<Hash, PromptCapsuleError> {
        let bytes = canonical_encode(self)?;
        Ok(Hash(Cid::from_content(&bytes).0))
    }
}

/// TRACE_MATRIX FC1-N44: canonical-encode a `PromptCapsule` and put into CAS.
/// Returns the CID of the encoded capsule (idempotent: same record → same CID).
pub fn write_prompt_capsule_to_cas(
    cas: &mut CasStore,
    record: &PromptCapsule,
    creator: &str,
    logical_t: u64,
) -> Result<Cid, PromptCapsuleError> {
    let bytes = canonical_encode(record)?;
    let cid = cas.put(
        &bytes,
        ObjectType::PromptCapsule,
        creator,
        logical_t,
        Some(PROMPT_CAPSULE_SCHEMA_ID.to_string()),
    )?;
    Ok(cid)
}

/// TRACE_MATRIX FC1-N44: CAS fetch + canonical-decode a `PromptCapsule`.
pub fn read_prompt_capsule_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<PromptCapsule, PromptCapsuleError> {
    let bytes = cas.get(cid)?;
    canonical_decode::<PromptCapsule>(&bytes).map_err(PromptCapsuleError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn fixture_capsule() -> PromptCapsule {
        let read_set: BTreeSet<ReadKey> = [
            ReadKey("agent_view".into()),
            ReadKey("market_ticker".into()),
        ]
        .into_iter()
        .collect();
        PromptCapsule::new(
            Hash([0xAA; 32]),
            read_set,
            "policy_v1",
            true,
            Cid([0xBB; 32]),
            Hash([0xCC; 32]),
            Cid([0xDD; 32]),
        )
        .expect("constructor accepts redacted=true")
    }

    #[test]
    fn constructor_rejects_unredacted() {
        let read_set: BTreeSet<ReadKey> = BTreeSet::new();
        let result = PromptCapsule::new(
            Hash([0; 32]),
            read_set,
            "policy_v1",
            false, // <-- forbidden
            Cid([0; 32]),
            Hash([0; 32]),
            Cid([0; 32]),
        );
        assert!(matches!(
            result,
            Err(PromptCapsuleError::HiddenFieldsNotRedacted)
        ));
    }

    #[test]
    fn canonical_hash_is_deterministic() {
        let a = fixture_capsule();
        let b = fixture_capsule();
        let ha = a.canonical_hash().expect("hash a");
        let hb = b.canonical_hash().expect("hash b");
        assert_eq!(ha, hb, "identical capsules must produce identical canonical hashes");
    }

    #[test]
    fn cas_round_trip_preserves_capsule() {
        let dir = TempDir::new().expect("tempdir");
        let mut cas = CasStore::open(dir.path()).expect("cas open");
        let cap = fixture_capsule();
        let cid = write_prompt_capsule_to_cas(&mut cas, &cap, "test-creator", 0)
            .expect("write capsule");
        let read_back = read_prompt_capsule_from_cas(&cas, &cid).expect("read capsule");
        assert_eq!(read_back, cap);
    }
}
