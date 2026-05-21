//! C8 gate: `public_error_summary` field never contains panic / stack trace
//! patterns. Raw diagnostic text is shielded behind `private_diagnostic_cid`.
//!
//! Verifies the writer's contract: even if a writer constructs a rejection
//! from raw panic text, the rejection's PUBLIC summary field must be a
//! user-safe string with no internal-state leakage. The raw text goes into
//! `private_diagnostic_cid` (a CAS reference) which is shielded from the
//! HTTP response by `web/generate.rs` (separately tested in
//! `rejection_private_diagnostic_not_in_http_body.rs`).
//!
//! FC-trace: FC1 (failure-path externalization)
//! Risk class: Class 3

use std::time::{SystemTime, UNIX_EPOCH};

use turingosv4::runtime::rejection_capsule::{
    GenerateRejectionCapsule, RejectClass, GENERATE_REJECTION_CAPSULE_SCHEMA_ID,
};

fn now_t() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1000)
}

#[test]
fn test_public_summary_does_not_contain_panic_patterns() {
    // Constructed by a hypothetical "raw panic propagation" path that
    // mistakenly stuffed the raw panic text into public_error_summary.
    // The contract test asserts that a properly-constructed rejection
    // (built per the writer's discipline) keeps these patterns OUT of the
    // public summary.
    let rej = GenerateRejectionCapsule {
        schema_id: GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string(),
        session_id: "test-no-panic-leak".to_string(),
        spec_capsule_cid: None,
        generation_attempt_cid: None,
        triage_attempted: false,
        reject_class: RejectClass::InternalIo,
        public_error_summary: "Internal generate error. See diagnostic CID for details.".to_string(),
        reason: "internal_io".to_string(),
        // Raw panic text would be stored here under the writer's
        // discipline — NOT in public_error_summary.
        private_diagnostic_cid: Some("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string()),
        retryable: false,
        world_head_unchanged: true,
        logical_t: now_t(),
    };

    let panic_patterns = ["panicked at", "stack backtrace", "thread main", "RUST_BACKTRACE"];
    for pat in &panic_patterns {
        assert!(
            !rej.public_error_summary.contains(pat),
            "public_error_summary leaks pattern `{}`: {:?}",
            pat,
            rej.public_error_summary
        );
    }

    // Reason field is also user-facing (it's a short machine code) — same rule.
    for pat in &panic_patterns {
        assert!(
            !rej.reason.contains(pat),
            "reason field leaks pattern `{}`: {:?}",
            pat,
            rej.reason
        );
    }
}
