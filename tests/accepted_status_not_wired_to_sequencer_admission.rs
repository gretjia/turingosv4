//! C11 gate: BuildStatus::Accepted must NOT be wired into sequencer admission.
//!
//! Static grep test: src/state/sequencer.rs must not reference BuildStatus::Accepted
//! or any accepted_delivery path in admission logic.
//!
//! FC-trace: FC1 (anti-wire invariant), FC3 (test evidence)
//! Risk class: Class 3

#[test]
fn test_accepted_status_not_wired_to_sequencer_admission() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sequencer_path = root.join("src/state/sequencer.rs");

    if !sequencer_path.exists() {
        // If sequencer.rs doesn't exist, there's nothing to wire — pass.
        println!("test: sequencer.rs not found — anti-wire invariant trivially holds");
        return;
    }

    let content = std::fs::read_to_string(&sequencer_path)
        .expect("cannot read src/state/sequencer.rs");

    let forbidden = [
        "BuildStatus",       // C11 enum lives in runtime::build_session_view — not sequencer
        "accepted_delivery", // C11 field
        "TestRunCapsule",    // C11 test run
        "overall_pass",      // C11 test run field
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

#[test]
fn test_accepted_status_serializes_correctly() {
    use turingosv4::runtime::build_session_view::BuildStatus;
    let s = BuildStatus::Accepted;
    let json = serde_json::to_string(&s).expect("serialize");
    assert_eq!(json, "\"accepted\"");
    let back: BuildStatus = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, BuildStatus::Accepted);
}

#[test]
fn test_build_session_view_accepted_delivery_defaults_to_false() {
    // Verify that accepted_delivery defaults to false when deserialized from
    // old JSON that doesn't have the field (backward compat via #[serde(default)]).
    let old_json = r#"{
        "session_id": "test",
        "spec_capsule_cid": null,
        "generation_attempts": [],
        "artifact_versions": [],
        "preview_runs": [],
        "rejection_events": [],
        "current_status": "generated"
    }"#;
    let view: turingosv4::runtime::build_session_view::BuildSessionView =
        serde_json::from_str(old_json).expect("deserialize old format");
    assert!(!view.accepted_delivery, "accepted_delivery must default to false");
}
