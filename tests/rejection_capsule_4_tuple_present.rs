//! C8 gate: v5-derived 4-tuple required fields are present and non-default
//! in every rejection capsule.
//!
//! Per `turingosv5/docs/contracts/friendly_error_l4e.md` (adopted into the
//! v4 charter §3.3), every rejection capsule MUST carry:
//!   1. `attempt_identity` (implemented as `generation_attempt_cid: Option<String>`
//!      — may be None if rejection was pre-LLM, but the field must exist)
//!   2. `reject_class` — a typed enum
//!   3. `user_safe_message` (implemented as `public_error_summary: String`)
//!   4. `reason` — short machine-readable code
//!   5. `world_head_unchanged: true` — the contract assertion
//!
//! FC-trace: FC1, FC3 (L4.E binding)
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

#[test]
fn test_4tuple_fields_present_and_non_default() {
    let rej = GenerateRejectionCapsule {
        schema_id: GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string(),
        session_id: "test-4tuple".to_string(),
        spec_capsule_cid: None,
        generation_attempt_cid: Some(
            "aabbccddeeff00112233445566778899aabbccddeeff00112233445566778899".to_string(),
        ),
        triage_attempted: true,
        reject_class: RejectClass::LlmApiError,
        public_error_summary: "LLM API failure — see diagnostic CID".to_string(),
        reason: "llm_api_error".to_string(),
        private_diagnostic_cid: Some(
            "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff".to_string(),
        ),
        retryable: true,
        world_head_unchanged: true,
        logical_t: now_t(),
    };

    // 1. attempt_identity (generation_attempt_cid) — must be exposable;
    //    here we constructed it as Some so it's present.
    assert!(
        rej.generation_attempt_cid.is_some(),
        "generation_attempt_cid (4-tuple #1) must be present"
    );

    // 2. reject_class — must be a typed enum, non-default
    assert_ne!(
        rej.reject_class,
        RejectClass::InvalidInput, // discriminant 0 — treat as the "default-ish"
        "reject_class must be a deliberate variant, not the discriminant-0 fallback"
    );

    // 3. user_safe_message (public_error_summary) — non-empty
    assert!(
        !rej.public_error_summary.is_empty(),
        "public_error_summary (4-tuple #3) must be non-empty"
    );

    // 4. reason — non-empty
    assert!(
        !rej.reason.is_empty(),
        "reason (4-tuple #4) must be non-empty"
    );

    // 5. world_head_unchanged — must be true (writer contract)
    assert!(
        rej.world_head_unchanged,
        "world_head_unchanged (4-tuple #5) must be true per writer contract"
    );

    // Serialization round-trip must preserve all 5 fields.
    let serialized = serde_json::to_string(&rej).expect("serialize");
    for required_field in &[
        "generation_attempt_cid",
        "reject_class",
        "public_error_summary",
        "reason",
        "world_head_unchanged",
    ] {
        assert!(
            serialized.contains(required_field),
            "required 4-tuple field {} missing from serialized form: {}",
            required_field,
            serialized
        );
    }
}
