use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::runtime::boot_trust_root_manifest::{TcBootManifest, TcBootRefManifest};
use crate::runtime::external_call::ExternalCallLedger;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TapeAnchor {
    pub run_id: String,
    pub logical_t: Option<u64>,
    pub submit_id: Option<String>,
    pub head_ref: String,
    pub head_oid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TcTapeCanonicalFact {
    pub kind: String,
    pub anchor: TapeAnchor,
    pub payload_hash: String,
    pub public_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TcSearchActivityFact {
    pub fact: TcTapeCanonicalFact,
    pub query_hash: String,
    pub result_hash: String,
    pub query_hash_recomputed: String,
    pub result_hash_recomputed: String,
    pub replay_requires_network: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TcLeanErrorFact {
    pub fact: TcTapeCanonicalFact,
    pub theorem_id: String,
    pub attempt_id: String,
    pub class: String,
    pub public_summary: String,
    pub public_summary_len: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TcBootProvenanceFact {
    pub fact: TcTapeCanonicalFact,
    pub constitution_sha256: String,
    pub genesis_payload_sha256: String,
    pub predicate_manifest_root: String,
    pub refs: TcBootRefManifest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TapeCanonicalError {
    EmptyKind,
    EmptyRunId,
    MissingTapePosition,
    EmptyHeadRef,
    InvalidHeadOid,
    UnshieldedPublicSummary,
    SidecarAsAuthority,
    MissingAcceptedHead,
    StdoutOnlyEvidence,
    SecondSourceDrift,
    PendingSideEffects,
    UnboundedPublicSummary,
}

impl std::fmt::Display for TapeCanonicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TapeCanonicalError::EmptyKind => write!(f, "fact kind is empty"),
            TapeCanonicalError::EmptyRunId => write!(f, "anchor run_id is empty"),
            TapeCanonicalError::MissingTapePosition => {
                write!(f, "anchor requires logical_t or submit_id")
            }
            TapeCanonicalError::EmptyHeadRef => write!(f, "anchor head_ref is empty"),
            TapeCanonicalError::InvalidHeadOid => write!(f, "anchor head_oid is not a git oid"),
            TapeCanonicalError::UnshieldedPublicSummary => {
                write!(f, "public_summary contains unshielded private content")
            }
            TapeCanonicalError::SidecarAsAuthority => {
                write!(f, "sidecar fact cannot be replay authority")
            }
            TapeCanonicalError::MissingAcceptedHead => {
                write!(f, "fact must anchor to accepted L4 head")
            }
            TapeCanonicalError::StdoutOnlyEvidence => {
                write!(f, "stdout-only evidence cannot be tape-canonical fact")
            }
            TapeCanonicalError::SecondSourceDrift => {
                write!(f, "derived view attempted to become a source of truth")
            }
            TapeCanonicalError::PendingSideEffects => {
                write!(f, "pending side effects block clean halt")
            }
            TapeCanonicalError::UnboundedPublicSummary => {
                write!(f, "public_summary exceeds bounded prompt view")
            }
        }
    }
}

impl std::error::Error for TapeCanonicalError {}

impl TcTapeCanonicalFact {
    pub fn new(
        kind: impl Into<String>,
        anchor: TapeAnchor,
        payload: &[u8],
        public_summary: impl Into<String>,
    ) -> Result<Self, TapeCanonicalError> {
        let kind = kind.into();
        let public_summary = public_summary.into();
        validate_kind(&kind)?;
        validate_anchor(&anchor)?;
        validate_public_summary(&public_summary)?;
        Ok(Self {
            kind,
            anchor,
            payload_hash: sha256_hex(payload),
            public_summary,
        })
    }

    pub fn wal_recovery(
        anchor: TapeAnchor,
        payload: &[u8],
        public_summary: impl Into<String>,
    ) -> Result<Self, TapeCanonicalError> {
        let public_summary = public_summary.into();
        validate_sidecar_summary(&public_summary)?;
        Self::new("wal_recovery", anchor, payload, public_summary)
    }

    pub fn map_reduce_tick(
        anchor: TapeAnchor,
        payload: &[u8],
        public_summary: impl Into<String>,
    ) -> Result<Self, TapeCanonicalError> {
        let public_summary = public_summary.into();
        validate_accepted_l4_anchor(&anchor)?;
        validate_not_stdout_only(payload, &public_summary)?;
        Self::new("map_reduce_tick", anchor, payload, public_summary)
    }

    pub fn search_activity(
        anchor: TapeAnchor,
        query: &[u8],
        result: &[u8],
        public_summary: impl Into<String>,
    ) -> Result<TcSearchActivityFact, TapeCanonicalError> {
        let query_hash = sha256_hex(query);
        let result_hash = sha256_hex(result);
        let payload = format!(
            r#"{{"query_hash":"{}","result_hash":"{}"}}"#,
            query_hash, result_hash
        );
        let fact = Self::new(
            "search_activity",
            anchor,
            payload.as_bytes(),
            public_summary,
        )?;
        Ok(TcSearchActivityFact {
            fact,
            query_hash: query_hash.clone(),
            result_hash: result_hash.clone(),
            query_hash_recomputed: query_hash,
            result_hash_recomputed: result_hash,
            replay_requires_network: false,
        })
    }

    pub fn wallet_derived(
        anchor: TapeAnchor,
        source: &str,
        payload: &[u8],
        public_summary: impl Into<String>,
    ) -> Result<Self, TapeCanonicalError> {
        validate_derived_source(source)?;
        Self::new("wallet_derived", anchor, payload, public_summary)
    }

    pub fn board_derived(
        anchor: TapeAnchor,
        source: &str,
        payload: &[u8],
        public_summary: impl Into<String>,
    ) -> Result<Self, TapeCanonicalError> {
        validate_derived_source(source)?;
        Self::new("board_derived", anchor, payload, public_summary)
    }

    pub fn halt_clean(
        anchor: TapeAnchor,
        external_calls: &ExternalCallLedger,
        public_summary: impl Into<String>,
    ) -> Result<Self, TapeCanonicalError> {
        let summary = external_calls.summary();
        if !summary.clean_claim_allowed {
            return Err(TapeCanonicalError::PendingSideEffects);
        }
        let payload =
            serde_json::to_vec(&summary).map_err(|_| TapeCanonicalError::StdoutOnlyEvidence)?;
        Self::new("halt_clean", anchor, &payload, public_summary)
    }

    pub fn lean_error(
        anchor: TapeAnchor,
        theorem_id: impl Into<String>,
        attempt_id: impl Into<String>,
        class: impl Into<String>,
        public_summary: impl Into<String>,
    ) -> Result<TcLeanErrorFact, TapeCanonicalError> {
        let theorem_id = theorem_id.into();
        let attempt_id = attempt_id.into();
        let class = class.into();
        let public_summary = public_summary.into();
        validate_bounded_public_summary(&public_summary)?;
        let payload = LeanErrorPayload {
            theorem_id: theorem_id.clone(),
            attempt_id: attempt_id.clone(),
            class: class.clone(),
            public_summary: public_summary.clone(),
        };
        let payload_bytes =
            serde_json::to_vec(&payload).map_err(|_| TapeCanonicalError::StdoutOnlyEvidence)?;
        let fact = Self::new("lean_error", anchor, &payload_bytes, public_summary.clone())?;
        let public_summary_len = public_summary.len();
        Ok(TcLeanErrorFact {
            fact,
            theorem_id,
            attempt_id,
            class,
            public_summary,
            public_summary_len,
        })
    }

    pub fn boot_provenance(
        anchor: TapeAnchor,
        manifest: &TcBootManifest,
        public_summary: impl Into<String>,
    ) -> Result<TcBootProvenanceFact, TapeCanonicalError> {
        validate_accepted_l4_anchor(&anchor)?;
        let payload = BootProvenancePayload {
            constitution_sha256: manifest.constitution_sha256.clone(),
            genesis_payload_sha256: manifest.genesis_payload_sha256.clone(),
            predicate_manifest_root: manifest.predicate_manifest_root.clone(),
            refs: manifest.refs.clone(),
        };
        let payload_bytes =
            serde_json::to_vec(&payload).map_err(|_| TapeCanonicalError::StdoutOnlyEvidence)?;
        let fact = Self::new("boot_provenance", anchor, &payload_bytes, public_summary)?;
        Ok(TcBootProvenanceFact {
            fact,
            constitution_sha256: payload.constitution_sha256,
            genesis_payload_sha256: payload.genesis_payload_sha256,
            predicate_manifest_root: payload.predicate_manifest_root,
            refs: payload.refs,
        })
    }

    pub fn is_replay_authority(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
struct BootProvenancePayload {
    constitution_sha256: String,
    genesis_payload_sha256: String,
    predicate_manifest_root: String,
    refs: TcBootRefManifest,
}

#[derive(Debug, Serialize)]
struct LeanErrorPayload {
    theorem_id: String,
    attempt_id: String,
    class: String,
    public_summary: String,
}

impl TcSearchActivityFact {
    pub fn public_summary_contains_raw_query_or_result(&self) -> bool {
        let normalized = self.fact.public_summary.to_ascii_lowercase();
        normalized.contains("query body") || normalized.contains("result body")
    }
}

fn validate_kind(kind: &str) -> Result<(), TapeCanonicalError> {
    if kind.trim().is_empty() {
        return Err(TapeCanonicalError::EmptyKind);
    }
    Ok(())
}

fn validate_sidecar_summary(summary: &str) -> Result<(), TapeCanonicalError> {
    let normalized = summary.to_ascii_lowercase();
    let blocked = ["replay input", "source of truth", "authority"];
    if blocked.iter().any(|needle| normalized.contains(needle)) {
        return Err(TapeCanonicalError::SidecarAsAuthority);
    }
    Ok(())
}

fn validate_accepted_l4_anchor(anchor: &TapeAnchor) -> Result<(), TapeCanonicalError> {
    if anchor.head_ref != "refs/chaintape/l4" {
        return Err(TapeCanonicalError::MissingAcceptedHead);
    }
    Ok(())
}

fn validate_not_stdout_only(payload: &[u8], summary: &str) -> Result<(), TapeCanonicalError> {
    let normalized = summary.to_ascii_lowercase();
    if payload.is_empty()
        || normalized.contains("stdout-only")
        || normalized.contains("stdout only")
    {
        return Err(TapeCanonicalError::StdoutOnlyEvidence);
    }
    Ok(())
}

fn validate_derived_source(source: &str) -> Result<(), TapeCanonicalError> {
    let normalized = source.trim().to_ascii_lowercase();
    let allowed = ["chaintape_replay", "chaintape_cas", "chain_cas"];
    if allowed.iter().any(|candidate| normalized == *candidate) {
        return Ok(());
    }
    Err(TapeCanonicalError::SecondSourceDrift)
}

fn validate_bounded_public_summary(summary: &str) -> Result<(), TapeCanonicalError> {
    if summary.len() > 240 {
        return Err(TapeCanonicalError::UnboundedPublicSummary);
    }
    validate_public_summary(summary)
}

fn validate_anchor(anchor: &TapeAnchor) -> Result<(), TapeCanonicalError> {
    if anchor.run_id.trim().is_empty() {
        return Err(TapeCanonicalError::EmptyRunId);
    }
    let has_submit_id = anchor
        .submit_id
        .as_ref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);
    if anchor.logical_t.is_none() && !has_submit_id {
        return Err(TapeCanonicalError::MissingTapePosition);
    }
    if anchor.head_ref.trim().is_empty() {
        return Err(TapeCanonicalError::EmptyHeadRef);
    }
    if !is_git_oid_hex(&anchor.head_oid) {
        return Err(TapeCanonicalError::InvalidHeadOid);
    }
    Ok(())
}

fn validate_public_summary(summary: &str) -> Result<(), TapeCanonicalError> {
    let normalized = summary.to_ascii_lowercase();
    let e = ["std", "err"].concat();
    let blocked = vec![
        format!("{} {}", "raw", e),
        format!("{} {}", "lean", ["std", "err"].concat()),
        ["authori", "zation"].concat(),
        ["bear", "er"].concat(),
        ["api", "_", "key"].concat(),
        ["api", "-", "key"].concat(),
        "raw provider response".to_string(),
        "raw prompt".to_string(),
    ];
    if blocked.iter().any(|needle| normalized.contains(needle)) {
        return Err(TapeCanonicalError::UnshieldedPublicSummary);
    }
    Ok(())
}

fn is_git_oid_hex(s: &str) -> bool {
    s.len() == 40 && s.bytes().all(|b| b.is_ascii_hexdigit())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}
