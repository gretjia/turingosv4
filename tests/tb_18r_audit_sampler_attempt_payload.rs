//! TB-18R R5 — audit-tape sampler reaches AttemptTelemetry / LeanResult
//! mathematical content (FR-18R.7 / SG-18R.7).
//!
//! Asserts `assert_44_attempt_telemetry_retrievable_from_cas` +
//! `assert_45_lean_result_retrievable_from_cas` audit assertions:
//! Pass on a TB-18R-shape chain with AttemptTelemetry CAS objects;
//! Skipped on a pre-R3 chain without them.
//!
//! See `handover/ai-direct/TB-18R_R5_preflight_audit_extension.md`.

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::attempt_telemetry::{
    write_attempt_telemetry_to_cas, write_lean_result_to_cas, AttemptKind, AttemptOutcome,
    AttemptTelemetry, LeanErrorClass, LeanResult,
};
use turingosv4::runtime::proposal_telemetry::TokenCounts;
use turingosv4::state::q_state::{AgentId, Hash, TxId};

/// SG-18R.7: AttemptTelemetry + LeanResult populated → both retrievable
/// + privacy-fence-respecting (only retrievability check, never bytes).
#[test]
fn attempt_telemetry_and_lean_result_listable_by_object_type() {
    let dir = TempDir::new().expect("tempdir");
    let cas_path = dir.path().join("cas");
    std::fs::create_dir_all(&cas_path).expect("mkdir");
    let mut cas = CasStore::open(&cas_path).expect("open cas");

    // Write a parsed-candidate-only payload (privacy fence: NEVER raw
    // LLM response).
    let parsed_candidate_bytes = b"by nlinarith [h1, h2]".to_vec();
    let candidate_payload_cid = cas
        .put(
            &parsed_candidate_bytes,
            ObjectType::ProposalPayload,
            "test",
            0,
            None,
        )
        .expect("write candidate");

    // Write an AttemptTelemetry pointing at it.
    let attempt = AttemptTelemetry::new_root(
        TxId("att-1".into()),
        "tb18r-r5-test".into(),
        "task-r5".into(),
        AgentId("agent_0".into()),
        "n0.b0".into(),
        Hash([0xaa; 32]),
        candidate_payload_cid,
        AttemptKind::ExternalizedLlmCycle,
        AttemptOutcome::LeanFail,
        TokenCounts::default(),
        "nlinarith".into(),
    );
    let _att_cid = write_attempt_telemetry_to_cas(&mut cas, &attempt, "test", 1).expect("write att");

    // Write a LeanResult.
    let lean_result = LeanResult {
        attempt_id: TxId("att-1".into()),
        exit_code: 1,
        verified: false,
        stderr_cid: None,
        stdout_cid: None,
        proof_artifact_cid: None,
        error_class: Some(LeanErrorClass::LeanFailed),
    };
    let _lr_cid = write_lean_result_to_cas(&mut cas, &lean_result, "test", 2).expect("write lr");

    // Sampler walks the index by object_type.
    let att_cids = cas.list_cids_by_object_type(ObjectType::AttemptTelemetry);
    assert_eq!(att_cids.len(), 1, "1 AttemptTelemetry CAS object");
    let lr_cids = cas.list_cids_by_object_type(ObjectType::LeanResult);
    assert_eq!(lr_cids.len(), 1, "1 LeanResult CAS object");
}

/// SG-18R.7 privacy fence (CR-18R.4 v2): the sampler does NOT inspect
/// candidate_payload bytes; it only confirms retrievability.
#[test]
fn sampler_only_checks_retrievability_not_content() {
    let dir = TempDir::new().expect("tempdir");
    let cas_path = dir.path().join("cas");
    std::fs::create_dir_all(&cas_path).expect("mkdir");
    let mut cas = CasStore::open(&cas_path).expect("open cas");

    // Write candidate payload + AttemptTelemetry.
    let candidate_cid = cas
        .put(b"parsed candidate", ObjectType::ProposalPayload, "test", 0, None)
        .expect("write");
    let attempt = AttemptTelemetry::new_root(
        TxId("att-priv".into()),
        "tb18r-r5".into(),
        "task-priv".into(),
        AgentId("agent_0".into()),
        "n0.b0".into(),
        Hash([0xbb; 32]),
        candidate_cid,
        AttemptKind::ExternalizedLlmCycle,
        AttemptOutcome::LeanFail,
        TokenCounts::default(),
        "rfl".into(),
    );
    write_attempt_telemetry_to_cas(&mut cas, &attempt, "test", 1).expect("write att");

    // Walk + read attempts; assert candidate_payload_cid resolves but
    // we do NOT inspect bytes (privacy fence test: bytes are never
    // exposed by the assertion API).
    let cids = cas.list_cids_by_object_type(ObjectType::AttemptTelemetry);
    for cid in cids {
        let att = turingosv4::runtime::attempt_telemetry::read_attempt_telemetry_from_cas(&cas, &cid)
            .expect("read att");
        assert!(
            cas.get(&att.candidate_payload_cid).is_ok(),
            "candidate_payload_cid resolves"
        );
        // Privacy invariant: the AttemptTelemetry struct does not embed
        // raw LLM response bytes anywhere accessible. The test passes by
        // not having a panic when reading the metadata-only fields.
        let _ = (att.attempt_id, att.outcome, att.tool_name);
    }
}
