/// TRACE_MATRIX FC2 + FC3: Prompt promotion receipt schema and runtime guard.
///
/// C10: Promote v1 → v2 prompts ONLY via a CAS-anchored `PromptPromotionReceipt`.
/// The runtime guard checks that any canonical prompt CID change has a matching
/// receipt with `promotion_decision == Promote` before the LLM can start.
///
/// **No env-var bypass is honored. TURINGOS_BYPASS_PROMOTION_GUARD has no effect.**
///
/// FC-trace: FC2 (prompt boot), FC3 (eval evidence binding)
/// Risk class: Class 3

use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::bottom_white::cas::schema::ObjectType;
use crate::bottom_white::cas::store::CasStore;
use crate::runtime::spec_capsule::cas_path;

/// TRACE_MATRIX FC3: Schema ID for prompt promotion receipts.
pub const PROMPT_PROMOTION_RECEIPT_SCHEMA_ID: &str = "turingos-prompt-promotion-v1";

/// TRACE_MATRIX FC2 + FC3: Promotion decision.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PromotionDecision {
    Promote,
    Reject,
}

/// TRACE_MATRIX FC2 + FC3: CAS-anchored prompt promotion receipt.
///
/// Written to CAS when `turingos llm prompt-eval --from <v1> --to <v2> --eval-set <cid>`
/// runs both prompts against an anchored eval set and records a decision.
///
/// `eval_set_cid` is REQUIRED — receipt without it is invalid.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptPromotionReceipt {
    pub schema_id: String,           // = PROMPT_PROMOTION_RECEIPT_SCHEMA_ID
    pub from_prompt_cid: String,     // SHA-256 hex of the baseline prompt bytes in CAS
    pub to_prompt_cid: String,       // SHA-256 hex of the candidate prompt bytes in CAS
    pub eval_set_cid: String,        // anchors which eval set was used — required
    pub eval_before_cid: String,     // CAS CID of eval transcript for from_prompt
    pub eval_after_cid: String,      // CAS CID of eval transcript for to_prompt
    pub promotion_decision: PromotionDecision,
    pub logical_t: u64,
}

// ---------------------------------------------------------------------------
// Promotion guard — Class 3 boundary
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC2: error types for the promotion guard.
#[derive(Debug)]
pub enum PromotionGuardError {
    NoCasStore(String),
    PromptNotInCas(String),
    NoReceiptFound,
    ReceiptDecisionReject,
    InvalidReceipt(String),
}

impl std::fmt::Display for PromotionGuardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCasStore(e) => write!(f, "CAS store unavailable: {e}"),
            Self::PromptNotInCas(cid) => write!(f, "prompt CID not in CAS: {cid}"),
            Self::NoReceiptFound => write!(
                f,
                "promotion guard: no PromptPromotionReceipt found for this prompt CID \
                 — run `turingos llm prompt-eval --from <v1> --to <v2> --eval-set <cid>` first"
            ),
            Self::ReceiptDecisionReject => write!(
                f,
                "promotion guard: matching receipt has decision=reject — cannot start LLM"
            ),
            Self::InvalidReceipt(e) => write!(f, "promotion guard: invalid receipt: {e}"),
        }
    }
}

/// TRACE_MATRIX FC2: Check that a canonical prompt (as a CAS CID hex) has an
/// approved `PromptPromotionReceipt` in the workspace CAS.
///
/// **IMPORTANT**: This function ignores the `TURINGOS_BYPASS_PROMOTION_GUARD`
/// environment variable unconditionally. No bypass path exists by design.
///
/// Returns `Ok(())` if a Promote receipt is found for this prompt CID.
/// Returns `Err(...)` on any failure — callers MUST refuse LLM startup on error.
pub fn check_promotion_guard(
    workspace: &Path,
    prompt_cid_hex: &str,
) -> Result<(), PromotionGuardError> {
    // NOTE: TURINGOS_BYPASS_PROMOTION_GUARD is intentionally NOT read here.
    // This is a hard architectural requirement (C10 kill criterion).

    let cas_dir = cas_path(workspace);
    if !cas_dir.exists() {
        return Err(PromotionGuardError::NoCasStore(
            format!("CAS dir does not exist: {:?}", cas_dir)
        ));
    }
    let mut store = CasStore::open(&cas_dir)
        .map_err(|e| PromotionGuardError::NoCasStore(e.to_string()))?;
    let _ = store.reload_index_from_sidecar();

    // Search all EvidenceCapsules for a matching receipt.
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    for cid in cids {
        let meta = match store.metadata(&cid) {
            Some(m) => m,
            None => continue,
        };
        if meta.schema_id.as_deref() != Some(PROMPT_PROMOTION_RECEIPT_SCHEMA_ID) {
            continue;
        }
        let bytes = match store.get(&cid) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let receipt: PromptPromotionReceipt = match serde_json::from_slice(&bytes) {
            Ok(r) => r,
            Err(e) => return Err(PromotionGuardError::InvalidReceipt(e.to_string())),
        };
        // The receipt must match the new prompt CID as to_prompt_cid.
        if receipt.to_prompt_cid != prompt_cid_hex {
            continue;
        }
        // Receipt must have eval_set_cid (non-empty — no anonymous eval).
        if receipt.eval_set_cid.is_empty() {
            return Err(PromotionGuardError::InvalidReceipt(
                "eval_set_cid is empty — anonymous eval not accepted".to_string()
            ));
        }
        // Check decision.
        return match receipt.promotion_decision {
            PromotionDecision::Promote => Ok(()),
            PromotionDecision::Reject => Err(PromotionGuardError::ReceiptDecisionReject),
        };
    }

    Err(PromotionGuardError::NoReceiptFound)
}

/// TRACE_MATRIX FC3: Write a `PromptPromotionReceipt` to CAS and return its CID hex.
///
/// `eval_set_cid` must be non-empty. Errors if empty.
pub fn write_promotion_receipt(
    workspace: &Path,
    receipt: &PromptPromotionReceipt,
    logical_t: u64,
) -> Result<String, crate::runtime::spec_capsule::CapsuleError> {
    if receipt.eval_set_cid.is_empty() {
        return Err(crate::runtime::spec_capsule::CapsuleError::Open(
            "eval_set_cid must not be empty".to_string()
        ));
    }

    let cas_dir = cas_path(workspace);
    std::fs::create_dir_all(&cas_dir)
        .map_err(|e| crate::runtime::spec_capsule::CapsuleError::Open(e.to_string()))?;
    let mut store = CasStore::open(&cas_dir)
        .map_err(|e| crate::runtime::spec_capsule::CapsuleError::Open(e.to_string()))?;

    let bytes = serde_json::to_vec(receipt)
        .map_err(|e| crate::runtime::spec_capsule::CapsuleError::Open(e.to_string()))?;

    let cid = store.put(
        &bytes,
        ObjectType::EvidenceCapsule,
        "promotion_guard",
        logical_t,
        Some(PROMPT_PROMOTION_RECEIPT_SCHEMA_ID.to_string()),
    ).map_err(|e| crate::runtime::spec_capsule::CapsuleError::Open(e.to_string()))?;

    Ok(cid.hex())
}

// ---------------------------------------------------------------------------
// Helper: sha256 of prompt bytes (used by cmd_llm for `--from`/`--to` PIDs)
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC3: Compute the CAS-style SHA-256 hex of raw prompt bytes.
///
/// This is the canonical "prompt CID hex" used in `PromptPromotionReceipt.from_prompt_cid`
/// and `to_prompt_cid`. Not a CAS CID object — just the sha256 content hash used
/// as a stable identifier for a prompt version.
pub fn sha256_hex_of_prompt(prompt_bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(prompt_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_promotion_decision_serialize_roundtrip() {
        let d = PromotionDecision::Promote;
        let json = serde_json::to_string(&d).expect("serialize");
        assert_eq!(json, "\"promote\"");
        let back: PromotionDecision = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, PromotionDecision::Promote);
    }

    #[test]
    fn test_receipt_has_eval_set_cid_required() {
        let receipt = PromptPromotionReceipt {
            schema_id: PROMPT_PROMOTION_RECEIPT_SCHEMA_ID.to_string(),
            from_prompt_cid: "a".repeat(64),
            to_prompt_cid: "b".repeat(64),
            eval_set_cid: String::new(), // EMPTY — must be rejected
            eval_before_cid: "c".repeat(64),
            eval_after_cid: "d".repeat(64),
            promotion_decision: PromotionDecision::Promote,
            logical_t: 1000,
        };
        let ws = tempfile::TempDir::new().expect("tempdir");
        let result = write_promotion_receipt(ws.path(), &receipt, 1000);
        assert!(result.is_err(), "empty eval_set_cid must be rejected");
    }

    #[test]
    fn test_no_bypass_env_var() {
        // Verify the guard does NOT read TURINGOS_BYPASS_PROMOTION_GUARD.
        // We set the env var, call the guard on an empty workspace, and
        // confirm it still returns Err (not Ok).
        std::env::set_var("TURINGOS_BYPASS_PROMOTION_GUARD", "1");
        let ws = tempfile::TempDir::new().expect("tempdir");
        let result = check_promotion_guard(ws.path(), &"a".repeat(64));
        std::env::remove_var("TURINGOS_BYPASS_PROMOTION_GUARD");
        assert!(
            result.is_err(),
            "guard must reject even with TURINGOS_BYPASS_PROMOTION_GUARD=1"
        );
    }

    #[test]
    fn test_sha256_hex_of_prompt_stable() {
        let h1 = sha256_hex_of_prompt(b"hello");
        let h2 = sha256_hex_of_prompt(b"hello");
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
        let h3 = sha256_hex_of_prompt(b"world");
        assert_ne!(h1, h3);
    }
}
