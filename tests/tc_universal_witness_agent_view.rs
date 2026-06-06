#[path = "support/tc_universal_witness.rs"]
mod tc_universal_witness;

use tc_universal_witness::{
    verify_universal_witness, AgentViewWitness, UniversalWitnessKind, UniversalWitnessRun,
    WitnessError, WitnessExpectation,
};

#[test]
fn agent_view_witness_shields_private_oracle_fragments() {
    let mut run = UniversalWitnessRun::new(
        "a12-agent-view",
        UniversalWitnessKind::AgentViewShielding,
        "git-head-a12",
        WitnessExpectation::AgentViewShielded,
    );
    run.agent_view = Some(AgentViewWitness {
        public_prompt: "public summary: verdict only".to_string(),
        private_fragments: vec!["raw_lean_stderr".to_string(), "hidden_oracle".to_string()],
        private_cids: vec!["cid-private-diagnostic".to_string()],
    });

    let verified = verify_universal_witness(&run).expect("agent view witness verifies");

    assert!(verified.agent_view_checked);
}

#[test]
fn agent_view_witness_rejects_private_fragment_leakage() {
    let mut run = UniversalWitnessRun::new(
        "a12-agent-view-negative",
        UniversalWitnessKind::AgentViewShielding,
        "git-head-a12",
        WitnessExpectation::AgentViewShielded,
    );
    run.agent_view = Some(AgentViewWitness {
        public_prompt: "public summary accidentally includes raw_lean_stderr".to_string(),
        private_fragments: vec!["raw_lean_stderr".to_string()],
        private_cids: vec![],
    });

    let err = verify_universal_witness(&run).expect_err("private fragment must not leak");

    assert_eq!(
        err,
        WitnessError::PrivateViewLeak("raw_lean_stderr".to_string())
    );
}
