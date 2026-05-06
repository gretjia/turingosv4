//! TB-18R R3 Integration Test — `RejectionClass` enum tail-append stability.
//!
//! Maps to charter v2 SG-18R.5 + Codex Q8 ratified stable-repr-u8 invariant.
//! Verifies:
//! - Pre-R3 variants 0..5 unchanged repr (no renumbering).
//! - New R3 variants 6..9 (LeanFailed / ParseFailed / SorryBlocked / LlmError)
//!   tail-append with stable repr-u8.
//! - `From<LeanErrorClass> for RejectionClass` preserves the discriminator
//!   1:1 (R1 LeanErrorClass repr 6/7/8/9 ↔ R3 RejectionClass repr 6/7/8/9).
//! - Pre-R3 vs R3-new variants occupy disjoint u8 ranges. The
//!   `RejectedSubmissionRecord::compute_hash` byte stream depends on
//!   `rejection_class as u8`; disjoint ranges guarantee post-R3 source
//!   produces byte-identical canonical hashes for any pre-R3-variant
//!   record (no collision; no shifted discriminator).
//!
//! TRACE_MATRIX FC1-N42 (TB-18R R3 NEW witness on the rejection-class side).

use std::collections::HashSet;

use turingosv4::bottom_white::ledger::rejection_evidence::RejectionClass;
use turingosv4::runtime::attempt_telemetry::LeanErrorClass;

#[test]
fn rejection_class_repr_stable_with_new_variants() {
    // Pre-R3 variants 0..5 — byte-pinned for canonical-hash stability.
    assert_eq!(RejectionClass::PredicateFailed as u8, 0);
    assert_eq!(RejectionClass::PolicyViolation as u8, 1);
    assert_eq!(RejectionClass::EscrowMissing as u8, 2);
    assert_eq!(RejectionClass::InvariantViolation as u8, 3);
    assert_eq!(RejectionClass::MalformedPayload as u8, 4);
    assert_eq!(RejectionClass::InsufficientBalance as u8, 5);

    // R3 NEW variants 6..9 — tail-append per charter §0.A Q8 remediation.
    assert_eq!(RejectionClass::LeanFailed as u8, 6);
    assert_eq!(RejectionClass::ParseFailed as u8, 7);
    assert_eq!(RejectionClass::SorryBlocked as u8, 8);
    assert_eq!(RejectionClass::LlmError as u8, 9);
}

#[test]
fn lean_error_class_to_rejection_class_repr_preserved() {
    // Per preflight §3.3: From<LeanErrorClass> for RejectionClass is a
    // no-op-discriminator transcode. Repr u8 preserved 1:1.
    let cases = [
        (LeanErrorClass::LeanFailed,   RejectionClass::LeanFailed,   6u8),
        (LeanErrorClass::ParseFailed,  RejectionClass::ParseFailed,  7),
        (LeanErrorClass::SorryBlocked, RejectionClass::SorryBlocked, 8),
        (LeanErrorClass::LlmError,     RejectionClass::LlmError,     9),
    ];
    for (lec, expected_rc, expected_u8) in cases {
        let rc: RejectionClass = lec.into();
        assert_eq!(rc, expected_rc, "From<{lec:?}> mapped to wrong RejectionClass");
        assert_eq!(rc as u8, expected_u8, "discriminator drifted on From<{lec:?}>");
        assert_eq!(lec as u8, expected_u8, "LeanErrorClass discriminator drifted");
    }
}

#[test]
fn pre_r3_and_r3_new_variants_occupy_disjoint_u8_ranges() {
    // Charter §0.A Q8 stable-repr-u8 invariant: any pre-R3 record's
    // canonical hash depends on `rejection_class as u8`. R3 must not
    // shift any pre-R3 discriminator. Equivalent structural test:
    // pre-R3 u8s ∩ R3-new u8s = ∅.
    //
    // This is the structural fence on FR-18R.5 + the implicit
    // pre-R3-records-replay-byte-identical commitment.
    let pre_r3: HashSet<u8> = [
        RejectionClass::PredicateFailed,
        RejectionClass::PolicyViolation,
        RejectionClass::EscrowMissing,
        RejectionClass::InvariantViolation,
        RejectionClass::MalformedPayload,
        RejectionClass::InsufficientBalance,
    ]
    .iter()
    .map(|r| *r as u8)
    .collect();
    let r3_new: HashSet<u8> = [
        RejectionClass::LeanFailed,
        RejectionClass::ParseFailed,
        RejectionClass::SorryBlocked,
        RejectionClass::LlmError,
    ]
    .iter()
    .map(|r| *r as u8)
    .collect();
    let expected_pre: HashSet<u8> = (0u8..=5).collect();
    let expected_new: HashSet<u8> = (6u8..=9).collect();

    assert_eq!(pre_r3, expected_pre, "pre-R3 variants must occupy 0..=5");
    assert_eq!(r3_new, expected_new, "R3-new variants must occupy 6..=9");
    assert!(
        pre_r3.is_disjoint(&r3_new),
        "pre-R3 and R3-new u8 ranges must be disjoint to preserve canonical-hash stability"
    );
}
