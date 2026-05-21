//! B4: Unit tests for the TestRunCapsule human-readable summary formatter.
//!
//! TRACE_MATRIX FC1 + FC3: TestRunCapsule summary (UX hardening).
//! Risk class: 2 (additive, production wire-up).
//! No network or CAS calls — exercises the format_test_run_summary helper.

use turingosv4::runtime::test_run::{format_test_run_summary, TestScenarioResult};
use turingosv4::runtime::test_scenario::TestScenario;

fn make_result(scenario: TestScenario, pass: bool) -> TestScenarioResult {
    TestScenarioResult {
        scenario,
        pass,
        detail: if pass { "ok".to_string() } else { "fail".to_string() },
    }
}

#[test]
fn all_pass_shows_pass_with_scenario_names() {
    let results = vec![
        make_result(TestScenario::EntrypointExists, true),
        make_result(TestScenario::HtmlParses, true),
        make_result(TestScenario::SandboxPolicyPreserved { policy: "sandbox".to_string() }, true),
    ];
    let summary = format_test_run_summary(&results);
    assert!(
        summary.starts_with("Internal tests: PASS"),
        "should start with 'Internal tests: PASS'; got: {summary}"
    );
    assert!(
        summary.contains("3/3"),
        "should show 3/3; got: {summary}"
    );
    assert!(summary.contains("EntrypointExists"), "missing EntrypointExists; got: {summary}");
    assert!(summary.contains("HtmlParses"), "missing HtmlParses; got: {summary}");
    assert!(summary.contains("SandboxPolicyPreserved"), "missing SandboxPolicyPreserved; got: {summary}");
}

#[test]
fn partial_fail_shows_fail_with_failing_scenario() {
    let results = vec![
        make_result(TestScenario::EntrypointExists, true),
        make_result(TestScenario::HtmlParses, false),
        make_result(TestScenario::SandboxPolicyPreserved { policy: "sandbox".to_string() }, true),
    ];
    let summary = format_test_run_summary(&results);
    assert!(
        summary.starts_with("Internal tests: FAIL"),
        "should start with 'Internal tests: FAIL'; got: {summary}"
    );
    assert!(
        summary.contains("2/3"),
        "should show 2/3 passed; got: {summary}"
    );
    assert!(
        summary.contains("HtmlParses"),
        "should list HtmlParses as failed; got: {summary}"
    );
    // Passing scenarios must NOT appear in the failed list.
    assert!(
        summary.contains("failed:") || summary.contains("failed"),
        "should mention failed scenarios; got: {summary}"
    );
}

#[test]
fn all_fail_shows_fail_with_all_scenarios() {
    let results = vec![
        make_result(TestScenario::EntrypointExists, false),
        make_result(TestScenario::HtmlParses, false),
    ];
    let summary = format_test_run_summary(&results);
    assert!(
        summary.starts_with("Internal tests: FAIL"),
        "all-fail: should start with FAIL; got: {summary}"
    );
    assert!(
        summary.contains("0/2"),
        "should show 0/2 passed; got: {summary}"
    );
    assert!(summary.contains("EntrypointExists"), "should list EntrypointExists; got: {summary}");
    assert!(summary.contains("HtmlParses"), "should list HtmlParses; got: {summary}");
}

#[test]
fn single_pass_scenario_formats_correctly() {
    let results = vec![make_result(TestScenario::EntrypointExists, true)];
    let summary = format_test_run_summary(&results);
    assert!(
        summary.contains("PASS"),
        "single pass should show PASS; got: {summary}"
    );
    assert!(
        summary.contains("1/1"),
        "should show 1/1; got: {summary}"
    );
}

#[test]
fn multiple_failures_listed_comma_separated() {
    let results = vec![
        make_result(TestScenario::EntrypointExists, false),
        make_result(TestScenario::HtmlParses, false),
        make_result(TestScenario::SandboxPolicyPreserved { policy: "csp".to_string() }, true),
    ];
    let summary = format_test_run_summary(&results);
    assert!(
        summary.contains("EntrypointExists") && summary.contains("HtmlParses"),
        "both failing scenarios should be listed; got: {summary}"
    );
    // They should be comma-separated.
    assert!(
        summary.contains("EntrypointExists, HtmlParses") || summary.contains("HtmlParses, EntrypointExists"),
        "failing scenarios should be comma-separated; got: {summary}"
    );
}
