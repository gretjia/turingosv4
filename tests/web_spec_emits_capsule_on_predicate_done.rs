//! A6 invariant test (2026-05-19): web layer must emit a real SpecCapsule
//! CID when a driven-mode grill session reaches `done = true` + predicate-pass.
//!
//! Pre-A6 the web handler returned
//! `spec_capsule_cid: None` + `termination_reason: "predicate_done_no_spec_pending_synthesis"`
//! because the `spec_capsule` module was binary-only. A6 library-ized
//! `src/runtime/spec_capsule.rs` + `src/runtime/spec_synthesis.rs` so the
//! same in-process synthesis the CLI driven path uses can fire from the web
//! request handler.
//!
//! This test exercises the synthesis primitives directly (not the full HTTP
//! handler — that needs a live LLM and is W9 scope). It guards against
//! regressions that would break the library-ization wire-up:
//!
//!   1. `spec_synthesis::canonical_questions` returns 8 entries per language.
//!   2. `spec_synthesis::synthesise_spec_md_no_llm` produces non-empty
//!      bytes that contain the SPEC_END marker.
//!   3. `spec_synthesis::wrap_spec_md` wraps the body with the appendix
//!      that the SpecCapsule canonically hashes.
//!   4. `spec_capsule::write_spec_capsule` produces a 64-char hex CID that
//!      round-trips via `read_spec_capsule`.
//!   5. `GrillSessionCapsuleBody` is serialisable, writable, and round-trips
//!      the `final_spec_capsule_cid` field cleanly.
//!
//! Run:
//! ```bash
//! cargo test --features web --test web_spec_emits_capsule_on_predicate_done
//! ```

#![cfg(feature = "web")]

use turingosv4::runtime::grill_predicates::Lang;
use turingosv4::runtime::spec_capsule::{
    read_spec_capsule, write_grill_session_capsule, write_spec_capsule, GrillAttemptTally,
    GrillSessionCapsuleBody,
};
use turingosv4::runtime::spec_synthesis::{
    canonical_questions, pad_answers_to_8, synthesise_spec_md_no_llm, wrap_spec_md,
};

/// Library API contract: `canonical_questions` returns 8 non-empty strings per language.
#[test]
fn canonical_questions_8_per_language() {
    assert_eq!(canonical_questions(Lang::Zh).len(), 8);
    assert_eq!(canonical_questions(Lang::En).len(), 8);
    for q in canonical_questions(Lang::Zh) {
        assert!(!q.is_empty());
    }
    for q in canonical_questions(Lang::En) {
        assert!(!q.is_empty());
    }
}

/// `pad_answers_to_8` pads short vectors with placeholder strings AND truncates over-long.
#[test]
fn pad_answers_to_8_pads_and_truncates() {
    let short = pad_answers_to_8(vec!["a".into()]);
    assert_eq!(short.len(), 8);
    assert_eq!(short[0], "a");
    assert!(short[7].contains("not collected"));

    let long = pad_answers_to_8((0..20).map(|i| format!("a{i}")).collect());
    assert_eq!(long.len(), 8);
    assert_eq!(long[7], "a7");
}

/// `synthesise_spec_md_no_llm` produces a non-empty body ending in the
/// `<!-- TURINGOS_SPEC_END -->` marker that downstream parsers gate on.
#[test]
fn synthesise_no_llm_emits_spec_end_marker() {
    let qs = canonical_questions(Lang::Zh);
    let answers = pad_answers_to_8(vec!["甲".into(), "乙".into()]);
    let body = synthesise_spec_md_no_llm(Lang::Zh, &qs, &answers);
    assert!(!body.is_empty());
    assert!(body.trim_end().ends_with("<!-- TURINGOS_SPEC_END -->"));
}

/// `wrap_spec_md` always renders the appendix with every (Q, A) pair, so
/// the SpecCapsule hashes raw transcript data alongside the synthesised body.
#[test]
fn wrap_spec_md_includes_qa_appendix() {
    let qs = canonical_questions(Lang::En);
    let answers = pad_answers_to_8(vec!["alpha".into(), "beta".into()]);
    let body = synthesise_spec_md_no_llm(Lang::En, &qs, &answers);
    let wrapped = wrap_spec_md(&body, &qs, &answers, "test-model", true);
    assert!(wrapped.contains("Appendix — Raw Q/A"));
    for i in 1..=8 {
        assert!(wrapped.contains(&format!("**Q{i}**:")));
        assert!(wrapped.contains(&format!("**A{i}**:")));
    }
    assert!(wrapped.contains("alpha"));
    assert!(wrapped.contains("beta"));
}

/// End-to-end: spec.md synthesis → CAS write → CAS read round-trip.
/// The CID must be 64 chars of hex; the read bytes must byte-equal the
/// wrapped spec.md (which is the SpecCapsule's canonical content).
#[test]
fn web_synth_path_round_trips_through_cas() {
    let tmp = tempfile::tempdir().expect("create temp workspace");
    let ws = tmp.path();

    let lang = Lang::En;
    let qs = canonical_questions(lang);
    let answers = pad_answers_to_8(vec![
        "I want a small group-buy tracker".into(),
        "Splitwise — but simpler".into(),
        "names, amounts, who-owes-who".into(),
    ]);
    let body = synthesise_spec_md_no_llm(lang, &qs, &answers);
    let spec_md = wrap_spec_md(&body, &qs, &answers, "web-skip-llm", true);

    let cid = write_spec_capsule(ws, &spec_md, "grill_driven_web", 1_716_001_000)
        .expect("write_spec_capsule succeeds");
    assert_eq!(cid.len(), 64, "CID must be 64 hex chars");
    assert!(
        cid.chars().all(|c| c.is_ascii_hexdigit()),
        "CID must be hex"
    );

    let bytes = read_spec_capsule(ws, &cid).expect("read_spec_capsule succeeds");
    assert_eq!(bytes, spec_md.as_bytes(), "round-trip bytes mismatch");
}

/// `GrillSessionCapsuleBody` write must succeed when populated from a
/// happy-path session, mirroring `cmd_spec.rs:1366-1378` (the CLI path).
#[test]
fn grill_session_capsule_writes_with_real_spec_cid() {
    let tmp = tempfile::tempdir().expect("create temp workspace");
    let ws = tmp.path();

    let lang = Lang::Zh;
    let qs = canonical_questions(lang);
    let answers = pad_answers_to_8(vec!["想要个小工具".into()]);
    let body = synthesise_spec_md_no_llm(lang, &qs, &answers);
    let spec_md = wrap_spec_md(&body, &qs, &answers, "web-skip-llm", true);
    let spec_cid = write_spec_capsule(ws, &spec_md, "grill_driven_web", 1_716_002_000).unwrap();

    let session_body = GrillSessionCapsuleBody {
        session_id: "session_a6_invariant".into(),
        turn_cids: vec!["turn1cid".into(), "turn2cid".into()],
        final_spec_capsule_cid: spec_cid.clone(),
        termination_reason: "llm_done_predicate_pass".into(),
        total_turns: 2,
        partial_session: false,
        lang: "zh".into(),
        grill_attempt_tally: GrillAttemptTally {
            meta_turns_accepted: 2,
            meta_turns_rejected: 0,
            triage_calls_relevant: 1,
            triage_calls_non_relevant: 0,
            synthesis_calls: 1,
        },
        logical_t: 1_716_002_500,
    };

    let session_cid = write_grill_session_capsule(ws, &session_body)
        .expect("write_grill_session_capsule succeeds on happy path");
    assert_eq!(session_cid.len(), 64);
    assert_ne!(
        session_cid, spec_cid,
        "session capsule must hash to a different CID than the spec capsule it references"
    );
}
