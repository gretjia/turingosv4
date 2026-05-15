use std::fs;

#[test]
fn real11_micro_probe_runner_rejects_live_real6b_and_scripted_attempt_prediction() {
    let script = fs::read_to_string("scripts/run_real11_e2_micro_probe.sh")
        .expect("REAL-11 E2 micro-probe runner exists");

    for required in [
        "TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION",
        "TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE",
        "TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS",
        "live REAL-6B is not authorized in REAL-11",
        "scripted AttemptPrediction fixture is forbidden in REAL-11 Atom 5",
        "scripted TaskOutcome buys are forbidden in REAL-11 Atom 5",
        "unset TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS",
        "attempt_prediction_fixture_count=0",
        "live_real6b_enabled=false",
    ] {
        assert!(
            script.contains(required),
            "micro-probe runner missing live REAL-6B fail-closed sentinel: {required}"
        );
    }
}

#[test]
fn real11_micro_probe_report_forbids_live_real6b_approval() {
    let report =
        fs::read_to_string("handover/reports/REAL11_E2_MICRO_PROBE_REPORT.md").unwrap_or_default();
    if report.is_empty() {
        return;
    }

    assert!(report.contains("live_real6b_enabled=false"));
    assert!(report.contains("attempt_prediction_fixture_count=0"));
    assert!(report.contains("No scripted buys in Atom 5"));
    assert!(!report.contains("live REAL-6B approved"));
}
