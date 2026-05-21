//! C11 gate: hidden-oracle — scenario set CID must not appear in BuildSessionView
//! in a way that would leak it to the generation prompt.
//!
//! This test verifies that `BuildSessionView` does NOT contain a
//! `test_scenario_set_cid` field. The only test-related field is
//! `accepted_delivery` (bool) — the actual scenario set CID stays shielded.
//!
//! FC-trace: FC1 (hidden-oracle invariant), FC3 (test evidence)
//! Risk class: Class 3

use turingosv4::runtime::build_session_view::BuildSessionView;

#[test]
fn test_build_session_view_has_no_scenario_set_cid_field() {
    // Verify by serializing a BuildSessionView and checking JSON keys.
    // BuildSessionView must NOT have a "test_scenario_set_cid" field.
    let view = BuildSessionView {
        session_id: "test".to_string(),
        spec_capsule_cid: None,
        generation_attempts: vec![],
        artifact_versions: vec![],
        preview_runs: vec![],
        rejection_events: vec![],
        current_status: turingosv4::runtime::build_session_view::BuildStatus::Generated,
        accepted_delivery: false,
    };

    let json = serde_json::to_string(&view).expect("serialize");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse");

    assert!(
        parsed.get("test_scenario_set_cid").is_none(),
        "BuildSessionView must NOT expose test_scenario_set_cid (hidden-oracle): {json}"
    );
}

#[test]
fn test_build_session_view_accepted_delivery_is_bool_not_cid() {
    // accepted_delivery must be a bool, not a CID string (which could leak oracle info)
    let view = BuildSessionView {
        session_id: "test-bool".to_string(),
        spec_capsule_cid: None,
        generation_attempts: vec![],
        artifact_versions: vec![],
        preview_runs: vec![],
        rejection_events: vec![],
        current_status: turingosv4::runtime::build_session_view::BuildStatus::Accepted,
        accepted_delivery: true,
    };

    let json = serde_json::to_string(&view).expect("serialize");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse");

    let ad = parsed.get("accepted_delivery").expect("accepted_delivery must exist");
    assert!(ad.is_boolean(), "accepted_delivery must be a bool: {}", ad);
    assert_eq!(ad.as_bool(), Some(true));
    assert_eq!(parsed["current_status"], "accepted");
}

#[test]
fn test_static_sequencer_has_no_accepted_status_reference() {
    // Static grep: src/state/sequencer.rs must not reference C11 BuildStatus::Accepted
    // or accepted_delivery/TestRunCapsule/overall_pass in any admission rule.
    // NOTE: The sequencer has its own "Accepted" terminology (AcceptedLedger,
    // VerifyTargetNotAccepted, PartialAccepted, OmegaAccepted) that are NOT related
    // to the C11 BuildStatus::Accepted. Only check for C11-specific symbols.
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sequencer_path = root.join("src/state/sequencer.rs");

    if !sequencer_path.exists() {
        println!("test: sequencer.rs not found — anti-wire invariant trivially holds");
        return;
    }

    let content = std::fs::read_to_string(&sequencer_path)
        .expect("cannot read src/state/sequencer.rs");

    // Only check for C11-specific symbols that would indicate wiring of
    // BuildStatus::Accepted into sequencer admission:
    let forbidden = [
        "BuildStatus",      // C11 enum lives in runtime::build_session_view — not sequencer
        "accepted_delivery", // C11 field — not a sequencer concept
        "TestRunCapsule",   // C11 test run — not a sequencer concept
        "overall_pass",     // C11 test run field — not a sequencer concept
        "turingos-test-run-v1", // C11 schema ID
    ];

    for pattern in &forbidden {
        assert!(
            !content.contains(pattern),
            "ANTI-WIRE VIOLATION: {:?} found in src/state/sequencer.rs. \
             C11 kill criterion: BuildStatus::Accepted must NEVER flow into sequencer admission.",
            pattern
        );
    }
}
