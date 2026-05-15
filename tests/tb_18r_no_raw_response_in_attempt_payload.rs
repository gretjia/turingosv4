//! TB-18R R1 Integration Test — Privacy structural fence.
//!
//! Per CR-18R.4 v2 (Codex Q3 ratified): `candidate_payload_cid` MUST point
//! at parsed external candidate bytes only; raw LLM response containing
//! private chain-of-thought is NEVER stored in AttemptTelemetry CAS.
//!
//! This test demonstrates the structural fence: a candidate payload that
//! resembles a raw LLM response shape (with `role` / `content` / `thinking`
//! fields characteristic of provider SDK envelopes) is detectable via a
//! conservative pattern check. R2 evaluator hot path will use this same
//! check pre-CAS-write to refuse a raw-response-shaped payload.
//!
//! Note: this is a *structural fence*, not a semantic guarantee. A
//! sufficiently determined R2 implementer could bypass it by extracting
//! only the prose content (which legitimately could contain "thinking"
//! prose). The fence catches the common-case mistake of dumping the whole
//! provider SDK response object.
//!
//! Maps to TB-18R charter v2 CR-18R.4 v2 + FR-18R.1 v2 candidate_payload_cid
//! invariant.
//!
//! TRACE_MATRIX FC1-N41 (TB-18R R1 NEW witness; structural privacy fence).

use sha2::{Digest, Sha256};
use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::attempt_telemetry::{
    write_attempt_telemetry_to_cas, AttemptKind, AttemptOutcome, AttemptTelemetry,
};
use turingosv4::runtime::proposal_telemetry::TokenCounts;
use turingosv4::state::q_state::{AgentId, Hash, TxId};

fn hash_for(domain: &str) -> Hash {
    let mut h = Sha256::new();
    h.update(domain.as_bytes());
    Hash(h.finalize().into())
}

/// Conservative structural fence: detects payloads that look like raw LLM
/// SDK response envelopes (containing role / content / thinking fields).
/// The R2 evaluator hot path will use this same check before CAS write to
/// reject raw-response-shaped payloads.
///
/// Returns `true` if the bytes look like a parsed candidate (e.g. Lean
/// source, tactic body, proof prefix) — i.e. PASS the privacy fence.
/// Returns `false` if the bytes look like a raw LLM response envelope —
/// i.e. FAIL the privacy fence and must be rejected.
fn looks_like_parsed_candidate(bytes: &[u8]) -> bool {
    let s = match std::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => {
            // Non-UTF8 bytes pass: parsed candidates may be binary
            // (e.g. Lean compiled .olean fragment); we don't structure-check
            // those. R2 implementer is responsible for not stuffing JSON
            // into a binary slot.
            return true;
        }
    };
    // Reject payloads that look like JSON envelope objects with the
    // characteristic provider SDK fields. The check uses substring matching
    // on the `"field"` key form to avoid false positives on Lean source
    // that happens to mention these as identifiers.
    let envelope_markers = ["\"role\"", "\"content\"", "\"thinking\"", "\"reasoning\""];
    let envelope_hits = envelope_markers.iter().filter(|m| s.contains(*m)).count();
    // 2+ envelope-shape markers in a single payload → very likely a raw
    // response. Single occurrences could be incidental (e.g. theorem
    // statement mentions "role" as a noun), so we require at least 2.
    envelope_hits < 2
}

#[test]
fn fence_passes_for_parsed_lean_candidate() {
    let parsed_lean = b"theorem t : 1 + 1 = 2 := rfl\n";
    assert!(
        looks_like_parsed_candidate(parsed_lean),
        "a clean Lean source candidate must pass the privacy fence"
    );
}

#[test]
fn fence_rejects_raw_anthropic_response_envelope() {
    let raw_response = br#"{"role":"assistant","content":[{"type":"text","text":"theorem t : 1+1 = 2 := rfl"}],"thinking":"I should use rfl here because both sides reduce to 2."}"#;
    assert!(
        !looks_like_parsed_candidate(raw_response),
        "raw Anthropic response envelope must FAIL the privacy fence"
    );
}

#[test]
fn fence_rejects_raw_openai_response_envelope() {
    let raw_response = br#"{"role":"assistant","content":"theorem t : 1+1=2 := rfl","reasoning":"chain of thought here"}"#;
    assert!(
        !looks_like_parsed_candidate(raw_response),
        "raw OpenAI-shaped response envelope must FAIL the privacy fence"
    );
}

#[test]
fn fc1_n41_privacy_fence_in_action() {
    // End-to-end: simulate the R2 evaluator path. A correctly-implemented
    // R2 will run `looks_like_parsed_candidate` on the parsed bytes BEFORE
    // calling `write_attempt_telemetry_to_cas`, refusing to construct
    // AttemptTelemetry if the fence trips.
    let dir = TempDir::new().expect("tempdir");
    let mut cas = CasStore::open(dir.path()).expect("open cas");

    // (a) Clean parsed candidate → fence passes → AttemptTelemetry written.
    let parsed_clean = b"theorem t : 1 + 1 = 2 := rfl\n";
    assert!(
        looks_like_parsed_candidate(parsed_clean),
        "parsed candidate must pass the fence"
    );

    let attempt_clean = AttemptTelemetry::new_root(
        TxId("att-clean".into()),
        "test-run".into(),
        "minif2f.t1".into(),
        AgentId("agent_0".into()),
        "n1.b0".into(),
        hash_for("clean-ctx"),
        Cid::from_content(parsed_clean),
        AttemptKind::ExternalizedLlmCycle,
        AttemptOutcome::LeanPass,
        TokenCounts::default(),
        "omega_wtool".into(),
    );
    write_attempt_telemetry_to_cas(&mut cas, &attempt_clean, "evaluator", 1)
        .expect("write clean candidate");

    // (b) Raw response envelope → fence fails → R2 should refuse.
    let raw_response = br#"{"role":"assistant","content":"...","thinking":"..."}"#;
    assert!(
        !looks_like_parsed_candidate(raw_response),
        "fence must reject raw response envelope; R2 must not call write_attempt_telemetry_to_cas"
    );
    // Note: we deliberately do NOT write `raw_response` as a candidate
    // payload. The point of this test is the structural fence; R2 enforces
    // it pre-write.
}
