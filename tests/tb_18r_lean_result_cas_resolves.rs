//! TB-18R R1 Integration Test — `VerifierResult` CAS round-trip with shielded
//! stderr / stdout / proof_artifact CIDs.
//!
//! Verifies that VerifierResult written to CAS is byte-identical when read back,
//! AND that stderr / stdout bytes stay shielded behind their own CAS objects
//! (VerifierResult only carries Cid references, never raw bytes).
//!
//! Maps to TB-18R charter v2 SG-18R.5 (== SG-TAPE-5) + CR-18R.8 (no public
//! broadcast of raw Lean stderr).
//!
//! TRACE_MATRIX FC1-N41 (TB-18R R1 NEW witness).

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::attempt_telemetry::{
    read_verifier_result_from_cas, write_verifier_result_to_cas, VerifierErrorClass, VerifierResult,
    VerifierVerdictKind,
};
use turingosv4::state::q_state::TxId;

#[test]
fn fc1_n41_lean_result_verified_pass_round_trip() {
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");

    // Pre-store the proof artifact bytes as a separate CAS object so the
    // VerifierResult.proof_artifact_cid references real content. This mirrors
    // the production R2 flow.
    let proof_bytes = b"theorem t : 1 + 1 = 2 := rfl\n";
    let proof_cid = cas
        .put(
            proof_bytes,
            ObjectType::ProposalPayload,
            "lean",
            10,
            Some("turingosv4.proposal_payload.v1".into()),
        )
        .expect("put proof artifact");

    let original = VerifierResult {
        attempt_id: TxId("att-pass".into()),
        exit_code: 0,
        verified: true,
        stderr_cid: None,
        stdout_cid: None,
        proof_artifact_cid: Some(proof_cid),
        error_class: None,
        verdict_kind: VerifierVerdictKind::Verified,
    };

    let cid = write_verifier_result_to_cas(&mut cas, &original, "evaluator", 11)
        .expect("write VerifierResult to cas");
    let recovered = read_verifier_result_from_cas(&cas, &cid).expect("read VerifierResult from cas");

    assert_eq!(original, recovered);
    assert!(recovered.verified);
    assert_eq!(recovered.exit_code, 0);
    assert!(recovered.proof_artifact_cid.is_some());

    // Shielded proof bytes are recoverable from CAS via the cid; VerifierResult
    // itself does NOT carry raw bytes inline.
    let recovered_proof = cas
        .get(&recovered.proof_artifact_cid.expect("cid"))
        .expect("read proof bytes");
    assert_eq!(recovered_proof, proof_bytes);
}

#[test]
fn fc1_n41_lean_result_failure_path_with_shielded_stderr() {
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");

    // Lean stderr from a failed tactic — goes into a CAS object so the
    // VerifierResult only carries the Cid (per CR-18R.8: no public broadcast
    // of raw Lean stderr). This mirrors how R2 will store stderr.
    let stderr_bytes = b"error: tactic 'nlinarith' failed at line 5\n  unsolved goals: x > 0\n";
    let stderr_cid = cas
        .put(
            stderr_bytes,
            ObjectType::CompressedRunLog,
            "lean",
            20,
            Some("turingosv4.lean_stderr.v1".into()),
        )
        .expect("put lean stderr");

    let original = VerifierResult {
        attempt_id: TxId("att-fail".into()),
        exit_code: 1,
        verified: false,
        stderr_cid: Some(stderr_cid),
        stdout_cid: None,
        proof_artifact_cid: None,
        error_class: Some(VerifierErrorClass::VerifierFailed),
        verdict_kind: VerifierVerdictKind::Failed,
    };

    let cid = write_verifier_result_to_cas(&mut cas, &original, "evaluator", 21).expect("write");
    let recovered = read_verifier_result_from_cas(&cas, &cid).expect("read");

    assert_eq!(original, recovered);
    assert!(!recovered.verified);
    assert_eq!(recovered.error_class, Some(VerifierErrorClass::VerifierFailed));
    // Stderr is recoverable from CAS via the shielded Cid.
    let recovered_stderr = cas
        .get(&recovered.stderr_cid.expect("cid"))
        .expect("read stderr bytes");
    assert_eq!(recovered_stderr, stderr_bytes);
}

#[test]
fn fc1_n41_lean_result_all_error_classes_round_trip() {
    // Per Codex Q8: VerifierErrorClass mirrors R3 RejectionClass tail-append
    // values 6..9. All four variants must round-trip via canonical encode.
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");

    for (i, ec) in [
        VerifierErrorClass::VerifierFailed,
        VerifierErrorClass::ParseFailed,
        VerifierErrorClass::IncompleteProofBlocked,
        VerifierErrorClass::LlmError,
    ]
    .iter()
    .enumerate()
    {
        let original = VerifierResult {
            attempt_id: TxId(format!("att-{}", i)),
            exit_code: 1,
            verified: false,
            stderr_cid: None,
            stdout_cid: None,
            proof_artifact_cid: None,
            error_class: Some(*ec),
            // exit_code=1 + verified=false + error_class=Some(_) = Failed shape.
            // SorryBlocked-class records at exit_code=1 also classify as Failed
            // at the VerifierResult shape level (the sorry distinction lives in the
            // VerifierErrorClass field).
            verdict_kind: VerifierVerdictKind::Failed,
        };
        let cid = write_verifier_result_to_cas(&mut cas, &original, "evaluator", 30 + i as u64)
            .expect("write");
        let recovered = read_verifier_result_from_cas(&cas, &cid).expect("read");
        assert_eq!(recovered.error_class, Some(*ec));
    }
}
