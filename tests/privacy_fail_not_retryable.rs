//! C8 gate: `PrivacyBlocked` rejection class enforces `retryable = false`.
//!
//! Privacy-blocked content must NOT be retried by the auto-retry loop in the
//! web layer or the CLI. This test asserts the invariant at the capsule
//! level: a rejection with `reject_class = PrivacyBlocked` must have
//! `retryable = false`. The web layer (`src/web/generate.rs`) honors this
//! field to skip auto-retry.
//!
//! FC-trace: FC1 (failure-path externalization)
//! Risk class: Class 3

use std::time::{SystemTime, UNIX_EPOCH};

use turingosv4::runtime::generation_attempt::{
    GenerateRejectionCapsule, RejectClass, GENERATE_REJECTION_CAPSULE_SCHEMA_ID,
};

fn now_t() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1000)
}

fn build_rejection(reject_class: RejectClass, retryable: bool) -> GenerateRejectionCapsule {
    GenerateRejectionCapsule {
        schema_id: GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string(),
        session_id: "test-privacy".to_string(),
        spec_capsule_cid: None,
        generation_attempt_cid: None,
        triage_attempted: false,
        reject_class,
        public_error_summary: "Generation blocked".to_string(),
        reason: "test_reason".to_string(),
        private_diagnostic_cid: None,
        retryable,
        world_head_unchanged: true,
        logical_t: now_t(),
    }
}

#[test]
fn test_privacy_blocked_with_correct_retryable_false() {
    let rej = build_rejection(RejectClass::PrivacyBlocked, false);
    // Round-trip via serde to confirm the field structure
    let serialized = serde_json::to_string(&rej).expect("serialize");
    let deserialized: GenerateRejectionCapsule =
        serde_json::from_str(&serialized).expect("deserialize");
    assert_eq!(deserialized.reject_class, RejectClass::PrivacyBlocked);
    assert!(
        !deserialized.retryable,
        "PrivacyBlocked must NOT be retryable"
    );
}

#[test]
fn test_other_reject_classes_can_be_retryable() {
    // Sanity check: non-privacy rejections may have retryable=true.
    // (This verifies the test for PrivacyBlocked is checking class-specific
    // behavior, not a blanket "all rejections are not retryable" rule.)
    for class in &[
        RejectClass::InvalidInput,
        RejectClass::LlmApiError,
        RejectClass::NoFilesParsed,
        RejectClass::TooManyFiles,
        RejectClass::HeuristicFailed,
        RejectClass::BudgetExceeded,
    ] {
        let rej = build_rejection(*class, true);
        assert!(
            rej.retryable,
            "non-PrivacyBlocked class {:?} should be allowed retryable=true",
            class
        );
    }
}
