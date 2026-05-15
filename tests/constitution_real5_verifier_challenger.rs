//! REAL-5 Atom 7 — Verifier / Challenger Bridge gates.

use turingosv4::runtime::real5_roles::{
    apply_verifier_reputation_delta, challenge_decision_trace, derive_role_view,
    verify_peer_fixture, AgentRole, ChallengeDecisionTrace, DerivedViewInput, DerivedViewRequest,
    NoChallengeReason, NoVerifyReason, VerifierTurnWitness,
};
use turingosv4::state::q_state::{AgentId, TaskId, TxId};

#[test]
fn sg_r5_7_verifier_fixture_and_no_verify_reason_are_tape_visible() {
    let verify = verify_peer_fixture(
        AgentId("Agent_verifier".into()),
        AgentId("Agent_solver".into()),
        TxId("work_tx_1".into()),
    );
    assert_eq!(verify.tx_kind, "VerifyTx");
    assert_ne!(verify.verifier_agent, verify.solver_agent);
    assert_eq!(apply_verifier_reputation_delta(10, true), 11);
    let no_verify = VerifierTurnWitness::NoVerify(NoVerifyReason {
        agent_id: AgentId("Agent_verifier".into()),
        reason: "no proof artifact".into(),
    });
    assert!(format!("{no_verify:?}").contains("no proof artifact"));
}

#[test]
fn sg_r5_7_challenge_trace_and_high_price_view_exist() {
    let trace: ChallengeDecisionTrace = challenge_decision_trace(
        AgentId("Agent_challenger".into()),
        Some(TxId("work_tx_high_price".into())),
        None::<NoChallengeReason>,
    );
    assert_eq!(trace.tx_kind.as_deref(), Some("ChallengeTx"));
    let view = derive_role_view(
        DerivedViewRequest {
            agent_id: AgentId("Agent_challenger".into()),
            role: AgentRole::Challenger,
            task_id: TaskId("task".into()),
            head_t: "HEAD1".into(),
        },
        DerivedViewInput::fixture(),
    )
    .unwrap();
    assert!(view
        .public_sections
        .iter()
        .any(|s| s.contains("high-price nodes")));
}
