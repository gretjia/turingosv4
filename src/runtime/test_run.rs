/// TRACE_MATRIX FC1 + FC3: TestRunCapsule schema and runner.
///
/// C11: Runs the TestScenarioSet against the just-generated ArtifactBundle
/// (reading files from CAS by bundle CID, NOT from filesystem artifacts/).
/// Writes a TestRunCapsule to CAS referencing both artifact_bundle_cid and
/// test_scenario_set_cid.
///
/// FC-trace: FC1 (test loop), FC3 (test evidence)
/// Risk class: Class 3

use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::runtime::test_scenario::{TestScenario, TestScenarioSet, TEST_SCENARIO_SET_SCHEMA_ID};
use crate::runtime::artifact_bundle::{ArtifactBundleManifest, ARTIFACT_BUNDLE_SCHEMA_ID};
use crate::bottom_white::cas::schema::ObjectType;
use crate::bottom_white::cas::store::CasStore;
use crate::runtime::spec_capsule::cas_path;

/// TRACE_MATRIX FC3: Schema ID for TestRunCapsule.
pub const TEST_RUN_CAPSULE_SCHEMA_ID: &str = "turingos-test-run-v1";

/// TRACE_MATRIX FC1: Per-scenario result.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestScenarioResult {
    pub scenario: TestScenario,
    pub pass: bool,
    pub detail: String,
}

/// TRACE_MATRIX FC1 + FC3: CAS-anchored test run capsule.
///
/// NOTE: No self-cid field — per C11 kill criteria.
/// `test_scenario_set_cid` is separate from `artifact_bundle_cid` to enforce
/// the hidden-oracle pattern.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestRunCapsule {
    pub schema_id: String,              // = TEST_RUN_CAPSULE_SCHEMA_ID
    pub artifact_bundle_cid: String,
    pub test_scenario_set_cid: String,  // separate CID (hidden-oracle shielding)
    pub results: Vec<TestScenarioResult>,
    pub overall_pass: bool,             // = all results.pass
    pub logical_t: u64,
}

/// TRACE_MATRIX FC1: error types for test runner.
#[derive(Debug)]
pub enum TestRunError {
    CasOpen(String),
    BundleNotFound(String),
    BundleDeserialize(String),
    ScenarioSetNotFound,
}

impl std::fmt::Display for TestRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CasOpen(e) => write!(f, "CAS open error: {e}"),
            Self::BundleNotFound(cid) => write!(f, "artifact bundle not found in CAS: {cid}"),
            Self::BundleDeserialize(e) => write!(f, "artifact bundle deserialize error: {e}"),
            Self::ScenarioSetNotFound => write!(f, "scenario set not found in CAS"),
        }
    }
}

// ---------------------------------------------------------------------------
// Test runner — reads artifact files from CAS (NOT filesystem artifacts/)
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1: Run TestScenarioSet against an ArtifactBundle (by CAS CID).
///
/// Reads artifact files from CAS by bundle CID — does NOT touch the filesystem
/// `artifacts/` directory. Returns a TestRunCapsule (not yet written to CAS).
pub fn run_test_scenario_set(
    workspace: &Path,
    artifact_bundle_cid_hex: &str,
    scenario_set: &TestScenarioSet,
) -> Result<TestRunCapsule, TestRunError> {
    let cas_dir = cas_path(workspace);
    let mut store = CasStore::open(&cas_dir)
        .map_err(|e| TestRunError::CasOpen(e.to_string()))?;
    let _ = store.reload_index_from_sidecar();

    // Load the ArtifactBundleManifest from CAS.
    let bundle_cid = parse_cid_hex(artifact_bundle_cid_hex)
        .ok_or_else(|| TestRunError::BundleNotFound(artifact_bundle_cid_hex.to_string()))?;
    let bundle_bytes = store
        .get(&bundle_cid)
        .map_err(|_| TestRunError::BundleNotFound(artifact_bundle_cid_hex.to_string()))?;
    let manifest: ArtifactBundleManifest = serde_json::from_slice(&bundle_bytes)
        .map_err(|e| TestRunError::BundleDeserialize(e.to_string()))?;

    // Run each scenario against the manifest + CAS content.
    let mut results = Vec::new();
    for scenario in &scenario_set.scenarios {
        let result = run_one_scenario(scenario, &manifest, &store);
        results.push(result);
    }

    let overall_pass = results.iter().all(|r| r.pass);

    let logical_t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1);

    Ok(TestRunCapsule {
        schema_id: TEST_RUN_CAPSULE_SCHEMA_ID.to_string(),
        artifact_bundle_cid: artifact_bundle_cid_hex.to_string(),
        test_scenario_set_cid: String::new(), // filled by caller after writing set to CAS
        results,
        overall_pass,
        logical_t,
    })
}

fn run_one_scenario(
    scenario: &TestScenario,
    manifest: &ArtifactBundleManifest,
    store: &CasStore,
) -> TestScenarioResult {
    match scenario {
        TestScenario::EntrypointExists => {
            let entrypoint = &manifest.entrypoint;
            let exists = manifest.files.iter().any(|f| &f.path == entrypoint);
            TestScenarioResult {
                scenario: scenario.clone(),
                pass: exists,
                detail: if exists {
                    format!("entrypoint {:?} found in bundle", entrypoint)
                } else {
                    format!("entrypoint {:?} NOT found in bundle (files: {:?})",
                        entrypoint,
                        manifest.files.iter().map(|f| &f.path).collect::<Vec<_>>()
                    )
                },
            }
        }
        TestScenario::HtmlParses => {
            // Read entrypoint HTML bytes from CAS and verify it has DOCTYPE + <html.
            let entrypoint = &manifest.entrypoint;
            let file_entry = manifest.files.iter().find(|f| &f.path == entrypoint);
            match file_entry {
                None => TestScenarioResult {
                    scenario: scenario.clone(),
                    pass: false,
                    detail: format!("entrypoint {:?} not in bundle", entrypoint),
                },
                Some(fe) => {
                    match parse_cid_hex(&fe.cid).and_then(|cid| store.get(&cid).ok()) {
                        None => TestScenarioResult {
                            scenario: scenario.clone(),
                            pass: false,
                            detail: format!("entrypoint CAS content not found for CID {}", fe.cid),
                        },
                        Some(bytes) => {
                            let html = String::from_utf8_lossy(&bytes).to_ascii_lowercase();
                            let has_doctype = html.contains("<!doctype html") || html.contains("<!doctype");
                            let has_html_tag = html.contains("<html");
                            let pass = has_doctype && has_html_tag;
                            TestScenarioResult {
                                scenario: scenario.clone(),
                                pass,
                                detail: if pass {
                                    "HTML structure valid (DOCTYPE + <html present)".to_string()
                                } else {
                                    format!("HTML structure invalid: doctype={}, html_tag={}", has_doctype, has_html_tag)
                                },
                            }
                        }
                    }
                }
            }
        }
        TestScenario::SandboxPolicyPreserved { policy } => {
            // Verify the policy attribute appears in the entrypoint or any file.
            let found = manifest.files.iter().any(|fe| {
                parse_cid_hex(&fe.cid)
                    .and_then(|cid| store.get(&cid).ok())
                    .map(|bytes| {
                        let content = String::from_utf8_lossy(&bytes).to_ascii_lowercase();
                        content.contains(policy.as_str())
                    })
                    .unwrap_or(false)
            });
            TestScenarioResult {
                scenario: scenario.clone(),
                pass: found,
                detail: if found {
                    format!("sandbox policy {:?} found in artifact", policy)
                } else {
                    format!("sandbox policy {:?} NOT found in any artifact file", policy)
                },
            }
        }
    }
}

/// TRACE_MATRIX FC3: Write a TestScenarioSet to CAS and return its CID hex.
pub fn write_scenario_set(
    workspace: &Path,
    scenario_set: &TestScenarioSet,
) -> Result<String, String> {
    let cas_dir = cas_path(workspace);
    std::fs::create_dir_all(&cas_dir).map_err(|e| e.to_string())?;
    let mut store = CasStore::open(&cas_dir).map_err(|e| e.to_string())?;

    let bytes = serde_json::to_vec(scenario_set).map_err(|e| e.to_string())?;
    let cid = store.put(
        &bytes,
        ObjectType::EvidenceCapsule,
        "test_runner",
        scenario_set.logical_t,
        Some(TEST_SCENARIO_SET_SCHEMA_ID.to_string()),
    ).map_err(|e| e.to_string())?;

    Ok(cid.hex())
}

/// TRACE_MATRIX FC3: Write a TestRunCapsule to CAS and return its CID hex.
pub fn write_test_run_capsule(
    workspace: &Path,
    capsule: &TestRunCapsule,
) -> Result<String, String> {
    let cas_dir = cas_path(workspace);
    std::fs::create_dir_all(&cas_dir).map_err(|e| e.to_string())?;
    let mut store = CasStore::open(&cas_dir).map_err(|e| e.to_string())?;

    let bytes = serde_json::to_vec(capsule).map_err(|e| e.to_string())?;
    let cid = store.put(
        &bytes,
        ObjectType::EvidenceCapsule,
        "test_runner",
        capsule.logical_t,
        Some(TEST_RUN_CAPSULE_SCHEMA_ID.to_string()),
    ).map_err(|e| e.to_string())?;

    Ok(cid.hex())
}

// ---------------------------------------------------------------------------
// Helper: read TestRunCapsule from CAS for a session (latest by logical_t)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// C11 producer pipeline — called from cmd_generate post-bundle
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1 + FC3: Full C11 producer pipeline.
///
/// Derives a TestScenarioSet from `spec_bytes`, writes it to CAS (hidden — CID
/// not returned to caller), runs scenarios against the artifact bundle in CAS,
/// writes the TestRunCapsule to CAS, and returns `(test_run_cid, overall_pass)`.
///
/// **Hidden-oracle contract**: the scenario set CID is intentionally NOT returned
/// to the caller and must NOT appear in any generation prompt or public response.
/// Only `test_run_cid` and `overall_pass` are surfaced.
pub fn run_and_write_test_pipeline(
    workspace: &Path,
    spec_bytes: &[u8],
    spec_capsule_cid: &str,
    artifact_bundle_cid_hex: &str,
    logical_t: u64,
) -> Result<(String, bool), String> {
    use crate::runtime::test_scenario::derive_scenario_set_from_spec;

    // Derive scenario set — bytes stay in this call, never reach LLM prompt.
    let scenario_set = derive_scenario_set_from_spec(spec_bytes, spec_capsule_cid, logical_t);

    // Write scenario set to CAS (hidden CID — not propagated to callers).
    let scenario_set_cid = write_scenario_set(workspace, &scenario_set)?;

    // Run scenarios against the artifact bundle.
    let mut capsule = run_test_scenario_set(workspace, artifact_bundle_cid_hex, &scenario_set)
        .map_err(|e| e.to_string())?;

    // Attach the scenario set CID (stays inside TestRunCapsule only).
    capsule.test_scenario_set_cid = scenario_set_cid;

    // Write capsule and return its CID + overall result.
    let run_cid = write_test_run_capsule(workspace, &capsule)?;
    Ok((run_cid, capsule.overall_pass))
}

/// TRACE_MATRIX FC1: Find the latest TestRunCapsule for the given artifact_bundle_cid.
///
/// Returns None if no TestRunCapsule exists for the bundle.
pub fn latest_test_run_for_bundle(
    workspace: &Path,
    artifact_bundle_cid_hex: &str,
) -> Option<TestRunCapsule> {
    let cas_dir = cas_path(workspace);
    if !cas_dir.exists() { return None; }
    let mut store = CasStore::open(&cas_dir).ok()?;
    let _ = store.reload_index_from_sidecar();

    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    let mut best: Option<(u64, TestRunCapsule)> = None;

    for cid in cids {
        let meta = store.metadata(&cid)?;
        if meta.schema_id.as_deref() != Some(TEST_RUN_CAPSULE_SCHEMA_ID) { continue; }
        let bytes = store.get(&cid).ok()?;
        let capsule: TestRunCapsule = serde_json::from_slice(&bytes).ok()?;
        if capsule.artifact_bundle_cid == artifact_bundle_cid_hex {
            match &best {
                None => best = Some((capsule.logical_t, capsule)),
                Some((t, _)) if capsule.logical_t > *t => best = Some((capsule.logical_t, capsule)),
                _ => {}
            }
        }
    }

    best.map(|(_, c)| c)
}

/// Parse a 64-char hex CID into a CAS Cid. Returns None on bad format.
fn parse_cid_hex(hex: &str) -> Option<crate::bottom_white::cas::schema::Cid> {
    if hex.len() != 64 { return None; }
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        bytes[i] = u8::from_str_radix(&hex[i*2..i*2+2], 16).ok()?;
    }
    Some(crate::bottom_white::cas::schema::Cid(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capsule_serialization_roundtrip() {
        let cap = TestRunCapsule {
            schema_id: TEST_RUN_CAPSULE_SCHEMA_ID.to_string(),
            artifact_bundle_cid: "a".repeat(64),
            test_scenario_set_cid: "b".repeat(64),
            results: vec![TestScenarioResult {
                scenario: TestScenario::EntrypointExists,
                pass: true,
                detail: "ok".to_string(),
            }],
            overall_pass: true,
            logical_t: 1000,
        };
        let json = serde_json::to_string(&cap).expect("serialize");
        let back: TestRunCapsule = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(cap, back);
    }

    #[test]
    fn test_overall_pass_requires_all_pass() {
        let results = vec![
            TestScenarioResult { scenario: TestScenario::EntrypointExists, pass: true, detail: "ok".into() },
            TestScenarioResult { scenario: TestScenario::HtmlParses, pass: false, detail: "fail".into() },
        ];
        let overall = results.iter().all(|r| r.pass);
        assert!(!overall, "overall_pass must be false when any result fails");
    }
}
