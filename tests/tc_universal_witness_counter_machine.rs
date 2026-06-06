#[path = "support/tc_universal_witness.rs"]
mod tc_universal_witness;

use tc_universal_witness::{
    verify_universal_witness, TransitionWitness, UniversalWitnessKind, UniversalWitnessRun,
    WitnessError, WitnessExpectation,
};

#[test]
fn counter_machine_reconstructs_from_tape_prefix_and_cas() {
    let mut run = UniversalWitnessRun::new(
        "a12-counter",
        UniversalWitnessKind::CounterMachine,
        "git-head-a12",
        WitnessExpectation::CounterValue { counter: 3 },
    );
    run.transitions
        .push(TransitionWitness::accepted("t1", "counter:+1"));
    run.transitions
        .push(TransitionWitness::accepted("t2", "counter:+2"));
    run.cas_objects
        .push(tc_universal_witness::CasObjectWitness::new(
            "cid-counter-fixture",
            "counter-machine-fixture",
        ));

    let verified = verify_universal_witness(&run).expect("counter witness verifies");

    assert_eq!(verified.accepted_transitions, 2);
    assert_eq!(verified.rejected_transitions, 0);
    assert_eq!(verified.counter_value, Some(3));
    assert_eq!(verified.cas_objects_checked, 1);
}

#[test]
fn counter_machine_tamper_positive_control_fails() {
    let mut run = UniversalWitnessRun::new(
        "a12-counter-tamper",
        UniversalWitnessKind::CounterMachine,
        "git-head-a12",
        WitnessExpectation::CounterValue { counter: 3 },
    );
    let mut transition = TransitionWitness::accepted("t1", "counter:+1");
    transition.payload = "counter:+99".to_string();
    run.transitions.push(transition);

    let err = verify_universal_witness(&run).expect_err("tamper must fail");

    assert!(matches!(err, WitnessError::HashMismatch { .. }));
}
