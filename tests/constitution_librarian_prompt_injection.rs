//! REAL-BCAST-1 — PromptCapsule injection gates.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::attempt_telemetry::{AttemptKind, AttemptOutcome, AttemptTelemetry};
use turingosv4::runtime::librarian_broadcast::{
    render_librarian_notices_section, validate_prompt_capsule_librarian_binding,
};
use turingosv4::runtime::prompt_capsule::{write_prompt_capsule_v2_to_cas, PromptCapsuleV2};
use turingosv4::runtime::proposal_telemetry::TokenCounts;
use turingosv4::runtime::real5_roles::AgentRole;
use turingosv4::state::q_state::{AgentId, Hash, TxId};

#[test]
fn prompt_capsule_read_set_and_visible_context_bind_digest() {
    let digest_cid = Cid::from_content(b"digest");
    let crop_cid = Cid::from_content(b"crop");
    let notices = render_librarian_notices_section(
        "cid:digest",
        &["err:type_mismatch count=2 trend=up".to_string()],
        8,
    )
    .unwrap();
    let prompt_bytes = format!("base prompt\n{notices}").into_bytes();
    let prompt_hash = Hash(Cid::from_content(&prompt_bytes).0);
    let capsule = PromptCapsuleV2 {
        prompt_context_hash: prompt_hash,
        agent_id: AgentId("Agent_0".into()),
        role: AgentRole::Solver,
        view_policy_id: "real5/solver_view/v1".into(),
        visible_context_cid: Cid::from_content(&prompt_bytes),
        read_set: vec![digest_cid, crop_cid],
        hidden_fields_redacted: vec!["raw prompt body".into(), "raw completion".into()],
        system_prompt_template_hash: Hash(Cid::from_content(b"template").0),
        model_assignment_cid: None,
    };

    validate_prompt_capsule_librarian_binding(&capsule, &prompt_bytes, digest_cid, crop_cid)
        .unwrap();

    let mut missing = capsule.clone();
    missing.read_set = vec![digest_cid];
    assert!(validate_prompt_capsule_librarian_binding(
        &missing,
        &prompt_bytes,
        digest_cid,
        crop_cid
    )
    .unwrap_err()
    .contains("role crop CID"));
}

#[test]
fn prompt_hash_changes_when_digest_notice_changes() {
    let a = render_librarian_notices_section("cid:a", &["err:a count=2".into()], 8).unwrap();
    let b = render_librarian_notices_section("cid:b", &["err:b count=2".into()], 8).unwrap();
    assert_ne!(
        Cid::from_content(a.as_bytes()),
        Cid::from_content(b.as_bytes())
    );
}

#[test]
fn attempt_telemetry_links_to_prompt_capsule_with_librarian_read_set() {
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let digest_cid = Cid::from_content(b"digest");
    let crop_cid = Cid::from_content(b"crop");
    let prompt = b"=== Librarian Notices ===\nsource: CAS/ChainTape-derived\n";
    let capsule = PromptCapsuleV2 {
        prompt_context_hash: Hash(Cid::from_content(prompt).0),
        agent_id: AgentId("Agent_0".into()),
        role: AgentRole::Solver,
        view_policy_id: "real5/solver_view/v1".into(),
        visible_context_cid: Cid::from_content(prompt),
        read_set: vec![digest_cid, crop_cid],
        hidden_fields_redacted: vec!["raw prompt body".into()],
        system_prompt_template_hash: Hash(Cid::from_content(b"template").0),
        model_assignment_cid: None,
    };
    let capsule_cid = write_prompt_capsule_v2_to_cas(&mut cas, &capsule, "test", 1).unwrap();
    assert_eq!(
        cas.metadata(&capsule_cid).unwrap().object_type,
        ObjectType::PromptCapsule
    );

    let attempt = AttemptTelemetry::new_root(
        TxId("attempt".into()),
        "run".into(),
        "task".into(),
        AgentId("Agent_0".into()),
        "branch".into(),
        capsule.prompt_context_hash,
        Cid::from_content(b"candidate"),
        AttemptKind::ExternalizedLlmCycle,
        AttemptOutcome::ParseFail,
        TokenCounts {
            prompt_tokens: 1,
            completion_tokens: 1,
            tool_tokens: 0,
        },
        "step".into(),
    )
    .with_prompt_capsule_cid(capsule_cid);
    assert_eq!(attempt.prompt_capsule_cid, Some(capsule_cid));
}
