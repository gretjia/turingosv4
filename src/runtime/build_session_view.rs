/// TRACE_MATRIX FC2-N16: Build session view reconstructed from CAS.
///
/// FC-trace: FC2 (derived state reconstruction), FC3 (CAS evidence)
/// Risk class: Class 2.

use std::path::Path;
use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::CasStore;
use crate::runtime::spec_capsule::{cas_path, GrillSessionCapsuleBody};
use crate::runtime::generation_attempt::{GenerationAttemptCapsule, GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID};
use crate::runtime::artifact_bundle::{ArtifactBundleManifest, ARTIFACT_BUNDLE_SCHEMA_ID};
use crate::runtime::preview_run::{PreviewRunCapsule, PREVIEW_RUN_CAPSULE_SCHEMA_ID};

/// TRACE_MATRIX FC2-N16: build status enum of a build session.
///
/// NOTE: `Accepted` is derived from `TestRunCapsule.overall_pass == true`.
/// This status MUST NOT flow into any `src/state/sequencer.rs` admission rule.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BuildStatus {
    SpecPending,
    SpecDone,
    Generating,
    Generated,
    Rejected,
    /// C11: Delivery accepted — `TestRunCapsule.overall_pass == true`.
    /// Anti-wire: this variant MUST NOT be read by `src/state/sequencer.rs`.
    Accepted,
}

/// TRACE_MATRIX FC2-N16: build session view struct containing all session event CIDs.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BuildSessionView {
    pub session_id: String,
    pub spec_capsule_cid: Option<String>,
    pub generation_attempts: Vec<String>,
    pub artifact_versions: Vec<String>,
    pub preview_runs: Vec<String>,
    pub rejection_events: Vec<String>,
    pub current_status: BuildStatus,
    /// C11: true iff the latest TestRunCapsule for this session has overall_pass=true.
    /// Anti-wire: MUST NOT be consumed by sequencer admission logic.
    #[serde(default)]
    pub accepted_delivery: bool,
}

/// TRACE_MATRIX FC2-N16: reconstructs the BuildSessionView from CAS objects.
pub fn derive_build_session_view(
    workspace: &Path,
    session_id: &str,
) -> Result<BuildSessionView, crate::runtime::spec_capsule::CapsuleError> {
    let cas_dir = cas_path(workspace);
    if !cas_dir.exists() {
        return Ok(BuildSessionView {
            session_id: session_id.to_string(),
            spec_capsule_cid: None,
            generation_attempts: Vec::new(),
            artifact_versions: Vec::new(),
            preview_runs: Vec::new(),
            rejection_events: Vec::new(),
            current_status: BuildStatus::SpecPending,
            accepted_delivery: false,
        });
    }

    let mut store = match CasStore::open(&cas_dir) {
        Ok(s) => s,
        Err(_) => {
            return Ok(BuildSessionView {
                session_id: session_id.to_string(),
                spec_capsule_cid: None,
                generation_attempts: Vec::new(),
                artifact_versions: Vec::new(),
                preview_runs: Vec::new(),
                rejection_events: Vec::new(),
                current_status: BuildStatus::SpecPending,
                accepted_delivery: false,
            });
        }
    };

    // Reload index from sidecar to get any changes written since store opened
    let _ = store.reload_index_from_sidecar();

    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);

    let mut spec_grill_sessions: Vec<(u64, Cid)> = Vec::new();
    let mut generation_attempts: Vec<(u64, Cid)> = Vec::new();
    let mut artifact_bundles: Vec<(u64, Cid)> = Vec::new();
    let mut preview_runs: Vec<(u64, Cid)> = Vec::new();
    let mut rejection_events: Vec<(u64, Cid)> = Vec::new();

    for cid in cids {
        let meta = match store.metadata(&cid) {
            Some(m) => m,
            None => continue,
        };
        let schema_id = match &meta.schema_id {
            Some(s) => s,
            None => continue,
        };

        if schema_id == "turingos-spec-grill-session-v1" {
            if let Ok(bytes) = store.get(&cid) {
                if let Ok(body) = serde_json::from_slice::<GrillSessionCapsuleBody>(&bytes) {
                    if body.session_id == session_id {
                        spec_grill_sessions.push((meta.created_at_logical_t, cid));
                    }
                }
            }
        } else if schema_id == GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID {
            if let Ok(bytes) = store.get(&cid) {
                if let Ok(body) = serde_json::from_slice::<GenerationAttemptCapsule>(&bytes) {
                    if body.session_id == session_id {
                        generation_attempts.push((meta.created_at_logical_t, cid));
                    }
                }
            }
        } else if schema_id == ARTIFACT_BUNDLE_SCHEMA_ID {
            if let Ok(bytes) = store.get(&cid) {
                if let Ok(body) = serde_json::from_slice::<ArtifactBundleManifest>(&bytes) {
                    if body.session_id == session_id {
                        artifact_bundles.push((meta.created_at_logical_t, cid));
                    }
                }
            }
        } else if schema_id == PREVIEW_RUN_CAPSULE_SCHEMA_ID {
            if let Ok(bytes) = store.get(&cid) {
                if let Ok(body) = serde_json::from_slice::<PreviewRunCapsule>(&bytes) {
                    if body.session_id == session_id {
                        preview_runs.push((meta.created_at_logical_t, cid));
                    }
                }
            }
        } else if schema_id == "turingos-generate-rejection-v1" {
            if let Ok(bytes) = store.get(&cid) {
                if let Ok(body) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                    if let Some(s_id) = body.get("session_id").and_then(|v| v.as_str()) {
                        if s_id == session_id {
                            rejection_events.push((meta.created_at_logical_t, cid));
                        }
                    }
                }
            }
        }
    }

    // Deterministic ordering by (logical_t, cid)
    spec_grill_sessions.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    generation_attempts.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    artifact_bundles.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    preview_runs.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    rejection_events.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    let generation_attempts_hex: Vec<String> = generation_attempts.iter().map(|(_, cid)| cid.hex()).collect();
    let artifact_versions_hex: Vec<String> = artifact_bundles.iter().map(|(_, cid)| cid.hex()).collect();
    let preview_runs_hex: Vec<String> = preview_runs.iter().map(|(_, cid)| cid.hex()).collect();
    let rejection_events_hex: Vec<String> = rejection_events.iter().map(|(_, cid)| cid.hex()).collect();

    let mut spec_capsule_cid: Option<String> = None;

    // 1. Try spec_grill_sessions
    if let Some(&(_, session_cid)) = spec_grill_sessions.last() {
        if let Ok(bytes) = store.get(&session_cid) {
            if let Ok(body) = serde_json::from_slice::<GrillSessionCapsuleBody>(&bytes) {
                if !body.final_spec_capsule_cid.is_empty() {
                    spec_capsule_cid = Some(body.final_spec_capsule_cid.clone());
                }
            }
        }
    }

    // 2. Try generation_attempts
    if spec_capsule_cid.is_none() {
        for &(_, attempt_cid) in generation_attempts.iter().rev() {
            if let Ok(bytes) = store.get(&attempt_cid) {
                if let Ok(body) = serde_json::from_slice::<GenerationAttemptCapsule>(&bytes) {
                    if let Some(ref cid) = body.spec_capsule_cid {
                        if !cid.is_empty() {
                            spec_capsule_cid = Some(cid.clone());
                            break;
                        }
                    }
                }
            }
        }
    }

    // 3. Try artifact_bundles
    if spec_capsule_cid.is_none() {
        for &(_, bundle_cid) in artifact_bundles.iter().rev() {
            if let Ok(bytes) = store.get(&bundle_cid) {
                if let Ok(body) = serde_json::from_slice::<ArtifactBundleManifest>(&bytes) {
                    if let Some(ref cid) = body.spec_capsule_cid {
                        if !cid.is_empty() {
                            spec_capsule_cid = Some(cid.clone());
                            break;
                        }
                    }
                }
            }
        }
    }

    // Determine current status
    let current_status = if spec_capsule_cid.is_none() {
        BuildStatus::SpecPending
    } else if generation_attempts.is_empty() {
        BuildStatus::SpecDone
    } else {
        let latest_rejection_t = rejection_events.last().map(|&(t, _)| t);
        let latest_artifact_t = artifact_bundles.last().map(|&(t, _)| t);

        match (latest_rejection_t, latest_artifact_t) {
            (Some(rej_t), Some(art_t)) => {
                if rej_t >= art_t {
                    BuildStatus::Rejected
                } else {
                    BuildStatus::Generated
                }
            }
            (Some(_), None) => BuildStatus::Rejected,
            (None, Some(_)) => BuildStatus::Generated,
            (None, None) => BuildStatus::Generating,
        }
    };

    // C11: check for passing TestRunCapsule to derive accepted_delivery.
    // Accepted = latest artifact bundle has a TestRunCapsule with overall_pass=true.
    let accepted_delivery = if let Some(&(_, ref latest_bundle_cid)) = artifact_bundles.last() {
        let bundle_cid_hex = latest_bundle_cid.hex();
        let test_run = crate::runtime::test_run::latest_test_run_for_bundle(
            workspace,
            &bundle_cid_hex,
        );
        test_run.map(|r| r.overall_pass).unwrap_or(false)
    } else {
        false
    };

    // If accepted, upgrade status to Accepted.
    let current_status = if accepted_delivery && current_status == BuildStatus::Generated {
        BuildStatus::Accepted
    } else {
        current_status
    };

    Ok(BuildSessionView {
        session_id: session_id.to_string(),
        spec_capsule_cid,
        generation_attempts: generation_attempts_hex,
        artifact_versions: artifact_versions_hex,
        preview_runs: preview_runs_hex,
        rejection_events: rejection_events_hex,
        current_status,
        accepted_delivery,
    })
}
