#[path = "support/tc_universal_witness.rs"]
mod tc_universal_witness;

use tc_universal_witness::{
    verify_universal_witness, TapeLane, TransitionWitness, UniversalWitnessKind,
    UniversalWitnessRun, WitnessError, WitnessExpectation,
};

#[test]
fn branching_witness_keeps_accepted_and_rejected_tape_visible() {
    let mut run = UniversalWitnessRun::new(
        "a12-branching",
        UniversalWitnessKind::BranchAndReject,
        "git-head-a12",
        WitnessExpectation::BranchAndReject {
            accepted_count: 1,
            rejected_count: 1,
        },
    );
    run.transitions.push(TransitionWitness::accepted(
        "accepted-branch",
        "branch:accepted",
    ));
    run.transitions.push(TransitionWitness::rejected(
        "rejected-branch",
        "branch:rejected",
    ));

    let verified = verify_universal_witness(&run).expect("branching witness verifies");

    assert_eq!(verified.accepted_transitions, 1);
    assert_eq!(verified.rejected_transitions, 1);
}

#[test]
fn branching_witness_rejects_missing_l4e_rejected_branch() {
    let mut run = UniversalWitnessRun::new(
        "a12-branching-negative",
        UniversalWitnessKind::BranchAndReject,
        "git-head-a12",
        WitnessExpectation::BranchAndReject {
            accepted_count: 1,
            rejected_count: 1,
        },
    );
    let mut rejected = TransitionWitness::rejected("rejected-branch", "branch:rejected");
    rejected.lane = TapeLane::Accepted;
    run.transitions.push(TransitionWitness::accepted(
        "accepted-branch",
        "branch:accepted",
    ));
    run.transitions.push(rejected);

    let err = verify_universal_witness(&run).expect_err("rejected branch must stay on L4.E");

    assert_eq!(
        err,
        WitnessError::ExpectationMismatch("expected 1 rejected transitions, got 0".to_string())
    );
}
