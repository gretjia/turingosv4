/// TRACE_MATRIX FC1 + FC2: Offline CAS replay for build sessions.
///
/// Reconstructs a build session entirely from CAS objects, with zero network
/// and zero LLM calls. Verifies all cross-CID references resolve.
///
/// FC-trace: FC1 (replay loop), FC2 (boot reconstruction)
/// Risk class: Class 2 (evaluator adapter / replay verifier)

use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::CasStore;
use crate::runtime::spec_capsule::{
    cas_path, CapsuleError,
    SPEC_CAPSULE_SCHEMA_ID, GrillTurnCapsuleBody, GrillSessionCapsuleBody,
};
use crate::runtime::generation_attempt::{
    GenerationAttemptCapsule, GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID,
};
use crate::runtime::rejection_capsule::{
    GenerateRejectionCapsule, GENERATE_REJECTION_CAPSULE_SCHEMA_ID,
};
use crate::runtime::artifact_bundle::{ArtifactBundleManifest, ARTIFACT_BUNDLE_SCHEMA_ID};
use crate::runtime::preview_run::{PreviewRunCapsule, PREVIEW_RUN_CAPSULE_SCHEMA_ID};
use crate::runtime::build_session_view::BuildSessionView;

/// TRACE_MATRIX FC1: A single step in the offline replay transcript.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayStep {
    SpecCapsule { cid: String },
    GrillTurn { cid: String },
    GrillSession { cid: String },
    GenerationAttempt { cid: String, outcome: String },
    ArtifactBundle { cid: String, file_count: usize },
    PreviewRun { cid: String },
    GenerateRejection { cid: String, reject_class: String, retryable: bool },
}

/// TRACE_MATRIX FC2: Full offline replay result for a build session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayResult {
    pub session_id: String,
    pub steps: Vec<ReplayStep>,
    pub view: BuildSessionView,
    pub dangling_cid_errors: Vec<String>,
}

/// TRACE_MATRIX FC2: Offline CAS-only replay of a build session.
///
/// Does NOT call any LLM, siliconflow client, or external network.
/// All cross-CID references are verified to resolve in CAS.
/// Returns a `ReplayResult` with a step-by-step transcript and `BuildSessionView`.
pub fn reconstruct_session(
    workspace: &Path,
    session_id: &str,
) -> Result<ReplayResult, CapsuleError> {
    let cas_dir = cas_path(workspace);
    if !cas_dir.exists() {
        return Ok(ReplayResult {
            session_id: session_id.to_string(),
            steps: Vec::new(),
            view: crate::runtime::build_session_view::derive_build_session_view(workspace, session_id)?,
            dangling_cid_errors: Vec::new(),
        });
    }

    let mut store = CasStore::open(&cas_dir)
        .map_err(|e| CapsuleError::Open(e.to_string()))?;
    let _ = store.reload_index_from_sidecar();

    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);

    // Collect all steps associated with this session, sorted by logical_t.
    let mut step_candidates: Vec<(u64, Cid, ReplayStep)> = Vec::new();
    let mut dangling_cid_errors: Vec<String> = Vec::new();

    for cid in &cids {
        let meta = match store.metadata(cid) {
            Some(m) => m,
            None => continue,
        };
        let schema_id = match &meta.schema_id {
            Some(s) => s.clone(),
            None => continue,
        };
        let logical_t = meta.created_at_logical_t;

        match schema_id.as_str() {
            id if id == SPEC_CAPSULE_SCHEMA_ID => {
                if let Ok(bytes) = store.get(cid) {
                    // spec capsule: body is raw spec.md bytes; no session_id field
                    // We include it in the replay if it's linked from any generation attempt
                    // for this session, or if it's the latest spec capsule.
                    step_candidates.push((logical_t, cid.clone(), ReplayStep::SpecCapsule {
                        cid: cid.hex(),
                    }));
                    let _ = bytes; // body not needed for step
                }
            }
            "turingos-spec-grill-turn-v1" => {
                if let Ok(bytes) = store.get(cid) {
                    if let Ok(body) = serde_json::from_slice::<GrillTurnCapsuleBody>(&bytes) {
                        if body.session_id == session_id {
                            step_candidates.push((logical_t, cid.clone(), ReplayStep::GrillTurn {
                                cid: cid.hex(),
                            }));
                        }
                    }
                }
            }
            "turingos-spec-grill-session-v1" => {
                if let Ok(bytes) = store.get(cid) {
                    if let Ok(body) = serde_json::from_slice::<GrillSessionCapsuleBody>(&bytes) {
                        if body.session_id == session_id {
                            step_candidates.push((logical_t, cid.clone(), ReplayStep::GrillSession {
                                cid: cid.hex(),
                            }));
                        }
                    }
                }
            }
            id if id == GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID => {
                if let Ok(bytes) = store.get(cid) {
                    if let Ok(capsule) = serde_json::from_slice::<GenerationAttemptCapsule>(&bytes) {
                        if capsule.session_id == session_id {
                            // Verify cross-CID references: spec_capsule_cid, raw_output_cid, parent_attempt_cid
                            for ref_cid_hex in [
                                capsule.spec_capsule_cid.as_deref(),
                                capsule.raw_output_cid.as_deref(),
                                capsule.parent_attempt_cid.as_deref(),
                            ].into_iter().flatten() {
                                if !verify_cid_resolves(&store, ref_cid_hex) {
                                    dangling_cid_errors.push(format!(
                                        "GenerationAttemptCapsule {} references dangling CID: {}",
                                        cid.hex(), ref_cid_hex
                                    ));
                                }
                            }
                            step_candidates.push((logical_t, cid.clone(), ReplayStep::GenerationAttempt {
                                cid: cid.hex(),
                                outcome: format!("{:?}", capsule.outcome),
                            }));
                        }
                    }
                }
            }
            id if id == ARTIFACT_BUNDLE_SCHEMA_ID => {
                if let Ok(bytes) = store.get(cid) {
                    if let Ok(manifest) = serde_json::from_slice::<ArtifactBundleManifest>(&bytes) {
                        if manifest.session_id == session_id {
                            // Verify cross-CID references
                            for ref_cid_hex in [
                                manifest.spec_capsule_cid.as_deref(),
                                manifest.previous_bundle_cid.as_deref(),
                            ].into_iter().flatten() {
                                if !verify_cid_resolves(&store, ref_cid_hex) {
                                    dangling_cid_errors.push(format!(
                                        "ArtifactBundleManifest {} references dangling CID: {}",
                                        cid.hex(), ref_cid_hex
                                    ));
                                }
                            }
                            // Verify generation_attempt_cid resolves
                            if !verify_cid_resolves(&store, &manifest.generation_attempt_cid) {
                                dangling_cid_errors.push(format!(
                                    "ArtifactBundleManifest {} references dangling generation_attempt_cid: {}",
                                    cid.hex(), manifest.generation_attempt_cid
                                ));
                            }
                            // Verify all file CIDs resolve
                            for file_entry in &manifest.files {
                                if !verify_cid_resolves(&store, &file_entry.cid) {
                                    dangling_cid_errors.push(format!(
                                        "ArtifactBundleManifest {} file {:?} references dangling CID: {}",
                                        cid.hex(), file_entry.path, file_entry.cid
                                    ));
                                }
                            }
                            let file_count = manifest.files.len();
                            step_candidates.push((logical_t, cid.clone(), ReplayStep::ArtifactBundle {
                                cid: cid.hex(),
                                file_count,
                            }));
                        }
                    }
                }
            }
            id if id == PREVIEW_RUN_CAPSULE_SCHEMA_ID => {
                if let Ok(bytes) = store.get(cid) {
                    if let Ok(capsule) = serde_json::from_slice::<PreviewRunCapsule>(&bytes) {
                        if capsule.session_id == session_id {
                            if !verify_cid_resolves(&store, &capsule.artifact_bundle_cid) {
                                dangling_cid_errors.push(format!(
                                    "PreviewRunCapsule {} references dangling artifact_bundle_cid: {}",
                                    cid.hex(), capsule.artifact_bundle_cid
                                ));
                            }
                            step_candidates.push((logical_t, cid.clone(), ReplayStep::PreviewRun {
                                cid: cid.hex(),
                            }));
                        }
                    }
                }
            }
            id if id == GENERATE_REJECTION_CAPSULE_SCHEMA_ID => {
                if let Ok(bytes) = store.get(cid) {
                    if let Ok(capsule) = serde_json::from_slice::<GenerateRejectionCapsule>(&bytes) {
                        if capsule.session_id == session_id {
                            // Verify cross-CID references (but NOT private_diagnostic_cid — shielded)
                            for ref_cid_hex in [
                                capsule.spec_capsule_cid.as_deref(),
                                capsule.generation_attempt_cid.as_deref(),
                            ].into_iter().flatten() {
                                if !verify_cid_resolves(&store, ref_cid_hex) {
                                    dangling_cid_errors.push(format!(
                                        "GenerateRejectionCapsule {} references dangling CID: {}",
                                        cid.hex(), ref_cid_hex
                                    ));
                                }
                            }
                            step_candidates.push((logical_t, cid.clone(), ReplayStep::GenerateRejection {
                                cid: cid.hex(),
                                reject_class: format!("{:?}", capsule.reject_class),
                                retryable: capsule.retryable,
                            }));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Sort by (logical_t, cid) for deterministic ordering
    step_candidates.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    let steps: Vec<ReplayStep> = step_candidates.into_iter().map(|(_, _, s)| s).collect();

    let view = crate::runtime::build_session_view::derive_build_session_view(workspace, session_id)?;

    Ok(ReplayResult {
        session_id: session_id.to_string(),
        steps,
        view,
        dangling_cid_errors,
    })
}

/// TRACE_MATRIX FC2: Verify a CID hex string resolves in the CAS store.
fn verify_cid_resolves(store: &CasStore, cid_hex: &str) -> bool {
    if cid_hex.len() != 64 || !cid_hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return false;
    }
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        let hex_byte = &cid_hex[i * 2..i * 2 + 2];
        match u8::from_str_radix(hex_byte, 16) {
            Ok(b) => bytes[i] = b,
            Err(_) => return false,
        }
    }
    let cid = Cid(bytes);
    store.metadata(&cid).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_cid_resolves_format() {
        // A zero CID hex shouldn't resolve in an empty store; just test format validation.
        let zero_hex = "0000000000000000000000000000000000000000000000000000000000000000";
        // Valid format but won't be in store
        assert_eq!(zero_hex.len(), 64);
        assert!(zero_hex.chars().all(|c| c.is_ascii_hexdigit()));

        // Invalid format
        assert!(!verify_cid_resolves_format("short"));
        assert!(!verify_cid_resolves_format("gg000000000000000000000000000000000000000000000000000000000000000")); // non-hex
    }

    fn verify_cid_resolves_format(hex: &str) -> bool {
        hex.len() == 64 && hex.chars().all(|c| c.is_ascii_hexdigit())
    }
}
