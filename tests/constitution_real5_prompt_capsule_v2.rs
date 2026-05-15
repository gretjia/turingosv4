//! REAL-5 Atom 3 — PromptCapsule role/view upgrade gates.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::attempt_telemetry::{
    read_attempt_telemetry_from_cas, write_attempt_telemetry_to_cas, AttemptKind, AttemptOutcome,
    AttemptTelemetry,
};
use turingosv4::runtime::prompt_capsule::{
    read_prompt_capsule_v2_from_cas, write_prompt_capsule_v2_to_cas, PromptCapsuleV2,
    PROMPT_CAPSULE_V2_SCHEMA_ID,
};
use turingosv4::runtime::proposal_telemetry::TokenCounts;
use turingosv4::runtime::real5_roles::{AgentRole, AgentRoleAssignment};
use turingosv4::state::q_state::{AgentId, Hash, TxId};

#[test]
fn sg_r5_3_prompt_capsule_v2_carries_role_view_and_no_raw_prompt() {
    let capsule = PromptCapsuleV2 {
        prompt_context_hash: Hash([1; 32]),
        agent_id: AgentId("Agent_0".into()),
        role: AgentRole::Trader,
        view_policy_id: "policy.real5.trader.v1".into(),
        visible_context_cid: Cid([2; 32]),
        read_set: vec![Cid([3; 32])],
        hidden_fields_redacted: vec!["raw_diagnostics".into(), "raw CoT".into()],
        system_prompt_template_hash: Hash([4; 32]),
        model_assignment_cid: Some(Cid([5; 32])),
    };
    let json = serde_json::to_string(&capsule).unwrap();
    assert!(json.contains("agent_id"));
    assert!(json.contains("role"));
    assert!(json.contains("view_policy_id"));
    assert!(!json.contains("raw_prompt"));
    assert!(!json.contains("completion"));
    assert!(!json.contains("chain_of_thought"));
    assert!(capsule.read_set_resolves(&[Cid([3; 32])]));
}

#[test]
fn sg_r5_3_prompt_capsule_role_matches_assignment() {
    let capsule = PromptCapsuleV2 {
        prompt_context_hash: Hash([1; 32]),
        agent_id: AgentId("Agent_0".into()),
        role: AgentRole::Verifier,
        view_policy_id: "policy.real5.verifier.v1".into(),
        visible_context_cid: Cid([2; 32]),
        read_set: vec![Cid([3; 32])],
        hidden_fields_redacted: vec!["raw CoT".into()],
        system_prompt_template_hash: Hash([4; 32]),
        model_assignment_cid: None,
    };
    let assignment = AgentRoleAssignment {
        agent_id: AgentId("Agent_0".into()),
        role: AgentRole::Verifier,
        role_objective_cid: Cid([9; 32]),
        allowed_tools: vec!["verify_peer".into()],
        risk_budget_micro: turingosv4::economy::money::MicroCoin::from_micro_units(1),
        view_policy_id: "policy.real5.verifier.v1".into(),
    };
    capsule
        .assert_matches_assignment(&assignment)
        .expect("matching assignment is accepted");
}

#[test]
fn sg_r5_3_prompt_capsule_v2_is_cas_anchored_and_readable() {
    let capsule = PromptCapsuleV2 {
        prompt_context_hash: Hash([7; 32]),
        agent_id: AgentId("Agent_3".into()),
        role: AgentRole::Challenger,
        view_policy_id: "policy.real5.challenger.v1".into(),
        visible_context_cid: Cid([8; 32]),
        read_set: vec![Cid([9; 32]), Cid([10; 32])],
        hidden_fields_redacted: vec!["raw_diagnostics".into()],
        system_prompt_template_hash: Hash([11; 32]),
        model_assignment_cid: Some(Cid([12; 32])),
    };
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let cid = write_prompt_capsule_v2_to_cas(&mut cas, &capsule, "real5-test", 5)
        .expect("v2 capsule writes to CAS");
    let decoded = read_prompt_capsule_v2_from_cas(&cas, &cid).expect("v2 capsule reads from CAS");
    assert_eq!(decoded, capsule);
    assert_eq!(PROMPT_CAPSULE_V2_SCHEMA_ID, "v2/prompt_capsule_role_view");
}

#[test]
fn sg_r5_3_externalized_attempt_references_prompt_capsule_v2_cid() {
    let prompt_hash = Hash([13; 32]);
    let capsule = PromptCapsuleV2 {
        prompt_context_hash: prompt_hash,
        agent_id: AgentId("Agent_4".into()),
        role: AgentRole::Trader,
        view_policy_id: "policy.real5.trader.v1".into(),
        visible_context_cid: Cid([14; 32]),
        read_set: vec![Cid([15; 32])],
        hidden_fields_redacted: vec!["raw CoT".into()],
        system_prompt_template_hash: Hash([16; 32]),
        model_assignment_cid: Some(Cid([17; 32])),
    };
    let tmp = TempDir::new().unwrap();
    let mut cas = CasStore::open(tmp.path()).unwrap();
    let capsule_cid = write_prompt_capsule_v2_to_cas(&mut cas, &capsule, "real5-test", 7).unwrap();
    let attempt = AttemptTelemetry::new_root(
        TxId("attempt-real5-v2".into()),
        "run-real5".into(),
        "task-real5".into(),
        AgentId("Agent_4".into()),
        "n4.b0".into(),
        prompt_hash,
        Cid([18; 32]),
        AttemptKind::ExternalizedLlmCycle,
        AttemptOutcome::ParseFail,
        TokenCounts {
            prompt_tokens: 1,
            completion_tokens: 1,
            tool_tokens: 0,
        },
        "real5_role_gateway".into(),
    )
    .with_prompt_capsule_cid(capsule_cid);
    let attempt_cid =
        write_attempt_telemetry_to_cas(&mut cas, &attempt, "real5-attempt", 8).unwrap();
    let decoded =
        read_attempt_telemetry_from_cas(&cas, &attempt_cid).expect("attempt telemetry reads");
    assert_eq!(decoded.prompt_capsule_cid, Some(capsule_cid));
    assert_eq!(decoded.prompt_context_hash, capsule.prompt_context_hash);
}
