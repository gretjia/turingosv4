//! TB-18R Phase 2 — `AttemptOutcome::PartialAccepted` tail-additive u8
//! representation stability witness.
//!
//! Per `handover/directives/2026-05-06_TB18R_PHASE_2_REMEDIATION_DIRECTIVE.md`
//! §3.2: the new `AttemptOutcome::PartialAccepted = 6` variant is tail-additive
//! relative to the R1-ratified discriminants `LeanPass=0, LeanFail=1,
//! ParseFail=2, SorryBlock=3, LlmErr=4, Aborted=5`. Existing variants must
//! NOT renumber; only PartialAccepted=6 is added.
//!
//! TRACE_MATRIX FC1-N41 (TB-18R Phase 2 NEW witness).

use turingosv4::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
use turingosv4::runtime::attempt_telemetry::AttemptOutcome;

#[test]
fn attempt_outcome_existing_discriminants_unchanged_post_phase_2() {
    // Per CLAUDE.md Code Standard + R1 ratification: pre-Phase-2 variants
    // 0..5 must NOT renumber. Phase 2 only tail-adds PartialAccepted=6.
    assert_eq!(AttemptOutcome::LeanPass as u8, 0);
    assert_eq!(AttemptOutcome::LeanFail as u8, 1);
    assert_eq!(AttemptOutcome::ParseFail as u8, 2);
    assert_eq!(AttemptOutcome::SorryBlock as u8, 3);
    assert_eq!(AttemptOutcome::LlmErr as u8, 4);
    assert_eq!(AttemptOutcome::Aborted as u8, 5);
}

#[test]
fn attempt_outcome_partial_accepted_discriminant_is_six() {
    // Tail-additive, next-available: 6.
    assert_eq!(AttemptOutcome::PartialAccepted as u8, 6);
}

#[test]
fn attempt_outcome_serde_round_trip_all_variants() {
    // bincode v2 BE+fixed-int round-trip for all seven variants. Each
    // discriminator rides into the canonical hash via `#[repr(u8)]`.
    for outcome in [
        AttemptOutcome::LeanPass,
        AttemptOutcome::LeanFail,
        AttemptOutcome::ParseFail,
        AttemptOutcome::SorryBlock,
        AttemptOutcome::LlmErr,
        AttemptOutcome::Aborted,
        AttemptOutcome::PartialAccepted,
    ] {
        let bytes = canonical_encode(&outcome).expect("encode");
        let decoded: AttemptOutcome = canonical_decode(&bytes).expect("decode");
        assert_eq!(decoded, outcome, "round-trip mismatch for {:?}", outcome);
    }
}

#[test]
fn attempt_outcome_partial_accepted_canonical_byte_sequence() {
    // bincode v2 + fixed-int + BigEndian (canonical_encode config) encodes
    // serde enum variant tags as u32 BE (4 bytes). Lock-in the byte
    // sequence for PartialAccepted = 6 → [0, 0, 0, 6].
    assert_eq!(
        canonical_encode(&AttemptOutcome::PartialAccepted).unwrap(),
        vec![0u8, 0, 0, 6]
    );
}

#[test]
fn attempt_outcome_partial_accepted_default_unchanged() {
    // R1 ratified: AttemptOutcome::default() = LeanPass. Phase 2 tail-add
    // does NOT change the default. (Test-time impl Default for AttemptOutcome.)
    assert_eq!(AttemptOutcome::default(), AttemptOutcome::LeanPass);
}
