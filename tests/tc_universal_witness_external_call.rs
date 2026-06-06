#[path = "support/tc_universal_witness.rs"]
mod tc_universal_witness;

use tc_universal_witness::{
    verify_universal_witness, ExternalCallTerminal, ExternalCallWitness, UniversalWitnessKind,
    UniversalWitnessRun, WitnessError, WitnessExpectation,
};

#[test]
fn external_call_replay_closes_intent_without_network() {
    let mut run = UniversalWitnessRun::new(
        "a12-external",
        UniversalWitnessKind::ExternalCallReplay,
        "git-head-a12",
        WitnessExpectation::ExternalCallClosed { intent_count: 1 },
    );
    run.external_calls.push(ExternalCallWitness {
        intent_id: "intent-1".to_string(),
        terminal: Some(ExternalCallTerminal::Completed {
            terminal_id: "terminal-1".to_string(),
        }),
        provider_called_during_replay: false,
    });

    let verified = verify_universal_witness(&run).expect("external-call witness verifies");

    assert_eq!(verified.external_intents_closed, 1);
    assert!(!verified.network_used);
}

#[test]
fn external_call_replay_fails_if_provider_is_called_or_terminal_missing() {
    let mut run = UniversalWitnessRun::new(
        "a12-external-negative",
        UniversalWitnessKind::ExternalCallReplay,
        "git-head-a12",
        WitnessExpectation::ExternalCallClosed { intent_count: 1 },
    );
    run.external_calls.push(ExternalCallWitness {
        intent_id: "intent-1".to_string(),
        terminal: None,
        provider_called_during_replay: true,
    });

    let err = verify_universal_witness(&run).expect_err("network replay must fail");

    assert_eq!(
        err,
        WitnessError::NetworkUsedDuringReplay("intent-1".to_string())
    );
}
