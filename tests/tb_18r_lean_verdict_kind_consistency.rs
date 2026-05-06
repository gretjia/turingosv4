//! TB-18R Phase 2 — typed-verdict consistency contract witness.
//!
//! Exercises `LeanResult::is_verdict_kind_consistent`, the predicate that
//! `assert_45_lean_result_retrievable_from_cas` enforces over LeanResult CAS
//! objects (`src/runtime/audit_assertions.rs:assert_45_lean_result_retrievable_from_cas`).
//!
//! Per Phase 2 directive §6 + FC-first analysis §2.4: the four canonical
//! shapes must all PASS, and ANY drift between the typed `verdict_kind` and
//! the legacy `(verified, error_class, exit_code)` fields must be detected
//! by the consistency check. The consistency predicate is the single source
//! of truth that assert_45 calls; exercising the predicate directly
//! validates the contract without building a full LoadedTape.
//!
//! TRACE_MATRIX FC2-N34 (TB-18R Phase 2 NEW witness).

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::runtime::attempt_telemetry::{LeanErrorClass, LeanResult, LeanVerdictKind};
use turingosv4::state::q_state::TxId;

fn make_lr(
    exit_code: i32,
    verified: bool,
    error_class: Option<LeanErrorClass>,
    verdict_kind: LeanVerdictKind,
) -> LeanResult {
    let _ = Cid::from_content(b"placeholder"); // retain Cid in scope to silence unused-import drift
    LeanResult {
        attempt_id: TxId("att-test".into()),
        exit_code,
        verified,
        stderr_cid: None,
        stdout_cid: None,
        proof_artifact_cid: None,
        error_class,
        verdict_kind,
    }
}

#[test]
fn consistency_passes_on_canonical_verified() {
    let lr = make_lr(0, true, None, LeanVerdictKind::Verified);
    assert!(lr.is_verdict_kind_consistent());
}

#[test]
fn consistency_passes_on_canonical_failed_lean_failed() {
    let lr = make_lr(
        1,
        false,
        Some(LeanErrorClass::LeanFailed),
        LeanVerdictKind::Failed,
    );
    assert!(lr.is_verdict_kind_consistent());
}

#[test]
fn consistency_passes_on_canonical_failed_parse_failed() {
    // Failed-class admits any LeanErrorClass at exit_code≠0.
    let lr = make_lr(
        2,
        false,
        Some(LeanErrorClass::ParseFailed),
        LeanVerdictKind::Failed,
    );
    assert!(lr.is_verdict_kind_consistent());
}

#[test]
fn consistency_passes_on_canonical_failed_with_sorry_at_nonzero_exit() {
    // exit_code=1 + SorryBlocked classifies as Failed at the LeanResult level
    // (the sorry distinction lives in LeanErrorClass; verdict_kind = Failed).
    let lr = make_lr(
        1,
        false,
        Some(LeanErrorClass::SorryBlocked),
        LeanVerdictKind::Failed,
    );
    assert!(lr.is_verdict_kind_consistent());
}

#[test]
fn consistency_passes_on_canonical_partial_accepted() {
    // The state that round-1 VETO surfaced — Phase 2 makes it explicit.
    let lr = make_lr(0, false, None, LeanVerdictKind::PartialAccepted);
    assert!(lr.is_verdict_kind_consistent());
}

#[test]
fn consistency_passes_on_canonical_sorry_blocked() {
    let lr = make_lr(
        0,
        false,
        Some(LeanErrorClass::SorryBlocked),
        LeanVerdictKind::SorryBlocked,
    );
    assert!(lr.is_verdict_kind_consistent());
}

#[test]
fn consistency_fails_on_drift_verified_with_false_verified_flag() {
    // Drift: verdict_kind=Verified but verified=false. The Phase 2 typed
    // invariant catches this — the round-1 R8 form would have missed the
    // drift because it used implication (=>) not equality (==).
    let lr = make_lr(0, false, None, LeanVerdictKind::Verified);
    assert!(!lr.is_verdict_kind_consistent());
}

#[test]
fn consistency_fails_on_drift_verified_with_error_class_set() {
    let lr = make_lr(
        0,
        true,
        Some(LeanErrorClass::LeanFailed),
        LeanVerdictKind::Verified,
    );
    assert!(!lr.is_verdict_kind_consistent());
}

#[test]
fn consistency_fails_on_drift_partial_accepted_with_error_class_set() {
    // verdict_kind=PartialAccepted requires error_class=None; emitter passing
    // Some(_) by mistake is caught.
    let lr = make_lr(
        0,
        false,
        Some(LeanErrorClass::LeanFailed),
        LeanVerdictKind::PartialAccepted,
    );
    assert!(!lr.is_verdict_kind_consistent());
}

#[test]
fn consistency_fails_on_drift_failed_with_exit_code_zero() {
    // Failed kind requires exit_code≠0. exit_code=0 with Failed kind is
    // either a real partial-accept mis-classified, or a defect — must fail.
    let lr = make_lr(
        0,
        false,
        Some(LeanErrorClass::LeanFailed),
        LeanVerdictKind::Failed,
    );
    assert!(!lr.is_verdict_kind_consistent());
}

#[test]
fn consistency_fails_on_drift_failed_with_no_error_class() {
    // Failed kind requires error_class.is_some(). None at exit_code≠0 with
    // Failed kind would silently swallow a missing-classification defect.
    let lr = make_lr(1, false, None, LeanVerdictKind::Failed);
    assert!(!lr.is_verdict_kind_consistent());
}

#[test]
fn consistency_fails_on_drift_sorry_blocked_with_wrong_error_class() {
    // SorryBlocked verdict_kind requires error_class == Some(SorryBlocked);
    // any other LeanErrorClass is drift.
    let lr = make_lr(
        0,
        false,
        Some(LeanErrorClass::LeanFailed),
        LeanVerdictKind::SorryBlocked,
    );
    assert!(!lr.is_verdict_kind_consistent());
}

#[test]
fn consistency_fails_on_drift_sorry_blocked_with_no_error_class() {
    let lr = make_lr(0, false, None, LeanVerdictKind::SorryBlocked);
    assert!(!lr.is_verdict_kind_consistent());
}

#[test]
fn consistency_fails_on_drift_sorry_blocked_at_nonzero_exit() {
    // SorryBlocked verdict_kind requires exit_code=0; sorry at exit_code≠0
    // should be classified as Failed.
    let lr = make_lr(
        1,
        false,
        Some(LeanErrorClass::SorryBlocked),
        LeanVerdictKind::SorryBlocked,
    );
    assert!(!lr.is_verdict_kind_consistent());
}

#[test]
fn consistency_canonical_round_trip_via_canonical_codec() {
    // The canonical-encoded LeanResult round-trips byte-stable AND the
    // recovered record passes the consistency check. This is the durability
    // guarantee for v2 LeanResult records on Phase 3 evidence.
    use turingosv4::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};

    let canonical_shapes: [(i32, bool, Option<LeanErrorClass>, LeanVerdictKind); 4] = [
        (0, true, None, LeanVerdictKind::Verified),
        (1, false, Some(LeanErrorClass::LeanFailed), LeanVerdictKind::Failed),
        (0, false, None, LeanVerdictKind::PartialAccepted),
        (
            0,
            false,
            Some(LeanErrorClass::SorryBlocked),
            LeanVerdictKind::SorryBlocked,
        ),
    ];
    for (ec, v, errc, kind) in canonical_shapes {
        let lr = make_lr(ec, v, errc, kind);
        assert!(lr.is_verdict_kind_consistent());
        let bytes = canonical_encode(&lr).expect("encode");
        let decoded: LeanResult = canonical_decode(&bytes).expect("decode");
        assert_eq!(decoded, lr);
        assert!(decoded.is_verdict_kind_consistent());
    }
}
