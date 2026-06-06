#[path = "support/tc_universal_witness.rs"]
mod tc_universal_witness;

use tc_universal_witness::{
    verify_universal_witness, SelfBootstrapWitness, UniversalWitnessKind, UniversalWitnessRun,
    WitnessError, WitnessExpectation,
};

#[test]
fn self_bootstrap_witness_is_proposal_only() {
    let mut run = UniversalWitnessRun::new(
        "a12-self-bootstrap",
        UniversalWitnessKind::SelfBootstrapProposalOnly,
        "git-head-a12",
        WitnessExpectation::SelfBootstrapProposalOnly,
    );
    run.self_bootstrap = Some(SelfBootstrapWitness {
        proposal_cid: "cid-fc3-proposal".to_string(),
        runtime_authority_changed: false,
        claims_full_fc3_closure: false,
    });

    let verified = verify_universal_witness(&run).expect("self-bootstrap witness verifies");

    assert!(verified.self_bootstrap_proposal_only);
}

#[test]
fn self_bootstrap_witness_rejects_runtime_authority_or_full_fc3_claim() {
    let mut run = UniversalWitnessRun::new(
        "a12-self-bootstrap-negative",
        UniversalWitnessKind::SelfBootstrapProposalOnly,
        "git-head-a12",
        WitnessExpectation::SelfBootstrapProposalOnly,
    );
    run.self_bootstrap = Some(SelfBootstrapWitness {
        proposal_cid: "cid-fc3-proposal".to_string(),
        runtime_authority_changed: false,
        claims_full_fc3_closure: true,
    });

    let err = verify_universal_witness(&run).expect_err("full FC3 closure claim must fail");

    assert_eq!(err, WitnessError::SelfBootstrapClaimsFullFc3Closure);
}
