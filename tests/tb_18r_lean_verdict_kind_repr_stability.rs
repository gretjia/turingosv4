//! TB-18R Phase 2 — `LeanVerdictKind` u8 representation stability witness.
//!
//! Per `handover/directives/2026-05-06_TB18R_PHASE_2_REMEDIATION_DIRECTIVE.md`
//! §2.2 + §6: the typed verdict discriminator is part of the canonical hash
//! (CAS objects are content-addressed by their canonical bytes), so the u8
//! values for `LeanVerdictKind::{Verified, Failed, PartialAccepted, SorryBlocked}`
//! must be locked-in for the v2 LeanResult schema. Tail-additive only.
//!
//! TRACE_MATRIX FC1-N41 (TB-18R Phase 2 NEW witness).

use turingosv4::runtime::attempt_telemetry::{LeanErrorClass, LeanResult, LeanVerdictKind};

#[test]
fn lean_verdict_kind_u8_discriminants_locked() {
    // The LeanVerdictKind enum is the typed verdict classifier introduced in
    // TB-18R Phase 2 (2026-05-06). Discriminant values MUST stay stable post-
    // ratification — they ride into the LeanResult canonical bytes via
    // bincode + #[repr(u8)] and into the chain content-address space.
    assert_eq!(LeanVerdictKind::Verified as u8, 0);
    assert_eq!(LeanVerdictKind::Failed as u8, 1);
    assert_eq!(LeanVerdictKind::PartialAccepted as u8, 2);
    assert_eq!(LeanVerdictKind::SorryBlocked as u8, 3);
}

#[test]
fn lean_verdict_kind_default_is_failed() {
    // Phase 2 directive §2.2: default = Failed (safest fallback). Should
    // never fire in practice since every emitter migrated post-Phase-2 sets
    // verdict_kind explicitly. False-positive on partial-accept = visible
    // assert_45 FAIL (good); false-negative on real failure would silently
    // swallow a defect (bad), so we choose the visible-FAIL side.
    assert_eq!(LeanVerdictKind::default(), LeanVerdictKind::Failed);
}

#[test]
fn lean_verdict_kind_legacy_field_derivation_canonical_shapes() {
    // The four canonical (exit_code, verified, error_class) shapes derive
    // their kinds correctly. Used by `r2_write_attempt_telemetry` for
    // emitter callsites that haven't migrated to explicit verdict_kind.
    //
    // Verified: (0, true, None)
    assert_eq!(
        LeanResult::derive_verdict_kind_from_legacy_fields(0, true, None),
        Some(LeanVerdictKind::Verified)
    );
    // Failed: (≠0, false, Some(_)) for any error class
    assert_eq!(
        LeanResult::derive_verdict_kind_from_legacy_fields(
            1,
            false,
            Some(LeanErrorClass::LeanFailed)
        ),
        Some(LeanVerdictKind::Failed)
    );
    assert_eq!(
        LeanResult::derive_verdict_kind_from_legacy_fields(
            2,
            false,
            Some(LeanErrorClass::ParseFailed)
        ),
        Some(LeanVerdictKind::Failed)
    );
    // PartialAccepted: (0, false, None) — step_partial_ok shape
    assert_eq!(
        LeanResult::derive_verdict_kind_from_legacy_fields(0, false, None),
        Some(LeanVerdictKind::PartialAccepted)
    );
    // SorryBlocked: (0, false, Some(SorryBlocked)) — distinguishable from
    // Failed because exit_code=0 (Lean compiled but flagged sorry)
    assert_eq!(
        LeanResult::derive_verdict_kind_from_legacy_fields(
            0,
            false,
            Some(LeanErrorClass::SorryBlocked)
        ),
        Some(LeanVerdictKind::SorryBlocked)
    );
}

#[test]
fn lean_verdict_kind_legacy_field_derivation_out_of_canonical_returns_none() {
    // Any tuple that doesn't match the four canonical shapes returns None.
    // The caller (currently `r2_write_attempt_telemetry`) treats None as
    // a derive failure and falls back to LeanVerdictKind::default()
    // (Failed). assert_45 will then surface the drift as a typed-invariant
    // violation — the defect is visible, not silent.

    // verified=true with exit_code≠0 — impossible in practice
    assert_eq!(
        LeanResult::derive_verdict_kind_from_legacy_fields(1, true, None),
        None
    );
    // verified=true with error_class set — internally inconsistent
    assert_eq!(
        LeanResult::derive_verdict_kind_from_legacy_fields(
            0,
            true,
            Some(LeanErrorClass::LeanFailed)
        ),
        None
    );
    // !verified, exit_code≠0, error_class=None — should be classified
    assert_eq!(
        LeanResult::derive_verdict_kind_from_legacy_fields(1, false, None),
        None
    );
    // SorryBlocked-class but exit_code≠0 — sorry-block at exit_code=1 is
    // covered by the Failed arm (the canonical shape requires exit_code=0).
    // Verifying that the SorryBlocked arm specifically requires exit_code=0:
    assert_eq!(
        LeanResult::derive_verdict_kind_from_legacy_fields(
            1,
            false,
            Some(LeanErrorClass::SorryBlocked)
        ),
        Some(LeanVerdictKind::Failed),
        "exit_code≠0 with SorryBlocked classifies as Failed at the LeanResult \
         shape level; the sorry distinction lives in the LeanErrorClass field"
    );
}

#[test]
fn lean_verdict_kind_serde_round_trip_via_canonical_codec() {
    // bincode v2 BE+fixed-int round-trip for the typed verdict discriminator.
    // Each variant must encode + decode byte-stable.
    use turingosv4::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};

    for kind in [
        LeanVerdictKind::Verified,
        LeanVerdictKind::Failed,
        LeanVerdictKind::PartialAccepted,
        LeanVerdictKind::SorryBlocked,
    ] {
        let bytes = canonical_encode(&kind).expect("encode");
        let decoded: LeanVerdictKind = canonical_decode(&bytes).expect("decode");
        assert_eq!(decoded, kind);
    }
}

#[test]
fn lean_verdict_kind_byte_position_matches_repr_u8_via_bincode_be_fixed_int() {
    // bincode v2 + fixed-int + BigEndian (canonical_encode config) encodes
    // serde enum variant tags as u32 BE (4 bytes), NOT u8 — even when the
    // Rust enum carries `#[repr(u8)]`. The repr only affects in-memory
    // layout; the wire format follows serde's variant-index convention.
    // Locking in the BE u32 byte sequence for each discriminator value.
    use turingosv4::bottom_white::ledger::transition_ledger::canonical_encode;

    assert_eq!(
        canonical_encode(&LeanVerdictKind::Verified).unwrap(),
        vec![0u8, 0, 0, 0]
    );
    assert_eq!(
        canonical_encode(&LeanVerdictKind::Failed).unwrap(),
        vec![0u8, 0, 0, 1]
    );
    assert_eq!(
        canonical_encode(&LeanVerdictKind::PartialAccepted).unwrap(),
        vec![0u8, 0, 0, 2]
    );
    assert_eq!(
        canonical_encode(&LeanVerdictKind::SorryBlocked).unwrap(),
        vec![0u8, 0, 0, 3]
    );
}
