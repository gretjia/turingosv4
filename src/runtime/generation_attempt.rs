use std::path::Path;
use crate::bottom_white::cas::schema::ObjectType;
use crate::bottom_white::cas::store::CasStore;
use crate::runtime::spec_capsule::{cas_path, CapsuleError};

/// TRACE_MATRIX FC1: Schema ID for LLM generation attempts.
pub const GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID: &str = "turingos-generation-attempt-v1";

/// TRACE_MATRIX FC1: Enum representing the outcome of a single generation attempt.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum AttemptOutcome {
    Success = 0,
    ParseFailed = 1,
    LlmApiError = 2,
    NoFilesParsed = 3,
    InternalIo = 4,
}

/// TRACE_MATRIX FC1 + FC3-N4: Capsule containing metadata for an individual generation attempt.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct GenerationAttemptCapsule {
    pub schema_id: String,                       // = "turingos-generation-attempt-v1"
    pub session_id: String,
    pub spec_capsule_cid: Option<String>,
    pub spec_source: String,                     // "cas_capsule" | "ondisk_spec_md"
    pub model_id: String,
    pub model_seed: Option<u64>,                 // when provider supports it; None otherwise
    pub prompt_hash: String,                     // hex sha256 of canonical prompt
    pub raw_output_cid: Option<String>,          // None if LlmApiError before any bytes returned
    pub usage_total_tokens: Option<u32>,
    pub retry_index: u32,                        // 0..MAX_GENERATE_ATTEMPTS-1
    pub parent_attempt_cid: Option<String>,      // previous retry in this session, ordering chain
    pub outcome: AttemptOutcome,
    pub parsed_file_count: usize,                // informational, never gating
    pub logical_t: u64,
}

/// TRACE_MATRIX FC3-N4: Writes the GenerationAttemptCapsule to CAS store.
pub fn write_generation_attempt_capsule(
    workspace: &Path,
    body: &GenerationAttemptCapsule,
) -> Result<String, CapsuleError> {
    let cas_dir = cas_path(workspace);
    std::fs::create_dir_all(&cas_dir)
        .map_err(|e| CapsuleError::Open(format!("create cas dir: {e}")))?;

    let mut store = CasStore::open(&cas_dir).map_err(|e| CapsuleError::Open(e.to_string()))?;

    let body_bytes =
        serde_json::to_vec(body).map_err(|e| CapsuleError::Put(format!("serialize body: {e}")))?;

    let cid = store
        .put(
            &body_bytes,
            ObjectType::EvidenceCapsule,
            "generate_system",
            body.logical_t,
            Some(GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string()),
        )
        .map_err(|e| CapsuleError::Put(e.to_string()))?;

    Ok(cid.hex())
}
<<<<<<< HEAD
=======

/// TRACE_MATRIX FC1: Schema ID for LLM generate rejections.
pub const GENERATE_REJECTION_CAPSULE_SCHEMA_ID: &str = "turingos-generate-rejection-v1";

/// TRACE_MATRIX FC1: Enum representing the classification of a generate rejection.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum RejectClass {
    InvalidInput = 0,
    SpecMissing = 1,
    LlmApiError = 2,
    NoFilesParsed = 3,
    TooManyFiles = 4,
    HeuristicFailed = 5,
    PrivacyBlocked = 6,
    BudgetExceeded = 7,
    InternalIo = 8,
}

/// TRACE_MATRIX FC1 + FC3-N4: Capsule containing metadata for a generate rejection event.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct GenerateRejectionCapsule {
    pub schema_id: String,                       // = "turingos-generate-rejection-v1"
    pub session_id: String,
    pub spec_capsule_cid: Option<String>,
    pub generation_attempt_cid: Option<String>,  // links to C2 capsule if attempt was made
    pub triage_attempted: bool,                  // false if rejected pre-LLM
    pub reject_class: RejectClass,
    pub public_error_summary: String,            // user-safe; no diagnostics
    pub reason: String,                          // short machine-readable reason code
    pub private_diagnostic_cid: Option<String>,  // raw bytes in CAS, SHIELDED
    pub retryable: bool,
    pub world_head_unchanged: bool,              // MUST be true (asserted)
    pub logical_t: u64,
}

/// TRACE_MATRIX FC3-N4: Writes the GenerateRejectionCapsule to CAS store.
pub fn write_generate_rejection_capsule(
    workspace: &Path,
    body: &GenerateRejectionCapsule,
) -> Result<String, CapsuleError> {
    let cas_dir = cas_path(workspace);
    std::fs::create_dir_all(&cas_dir)
        .map_err(|e| CapsuleError::Open(format!("create cas dir: {e}")))?;

    let mut store = CasStore::open(&cas_dir).map_err(|e| CapsuleError::Open(e.to_string()))?;

    let body_bytes =
        serde_json::to_vec(body).map_err(|e| CapsuleError::Put(format!("serialize body: {e}")))?;

    let cid = store
        .put(
            &body_bytes,
            ObjectType::EvidenceCapsule,
            "generate_system",
            body.logical_t,
            Some(GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string()),
        )
        .map_err(|e| CapsuleError::Put(e.to_string()))?;

    Ok(cid.hex())
}
>>>>>>> origin/charter-cak-c3
