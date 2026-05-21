use std::path::{Path, PathBuf};
use std::time::Duration;

use turingosv4::sdk::sanitized_runner::{
    env_allowlist_from_current, run_sanitized, SanitizedCommand, SanitizedOutput,
};

#[test]
fn real_task_regression_two_phase() {
    let tmp = tempfile::tempdir().expect("tempdir");

    let base = r#"
pub fn init_balance() -> i32 { 1 }
#[test]
fn wallet_zero_balance_initializes_to_zero() {
    assert_eq!(init_balance(), 0);
}
"#;
    let good = r#"
pub fn init_balance() -> i32 { 0 }
#[test]
fn wallet_zero_balance_initializes_to_zero() {
    assert_eq!(init_balance(), 0);
}
"#;
    let bad_delete_assert = r#"
pub fn init_balance() -> i32 { 1 }
#[test]
fn wallet_zero_balance_initializes_to_zero() {
    let _ = init_balance();
}
"#;
    let bad_tautology = r#"
pub fn init_balance() -> i32 { 1 }
#[test]
fn wallet_zero_balance_initializes_to_zero() {
    assert!(true);
}
"#;

    let base_run = compile_and_run_test(tmp.path(), "base", base);
    let good_run = compile_and_run_test(tmp.path(), "good", good);
    let bad_delete_run = compile_and_run_test(tmp.path(), "bad_delete", bad_delete_assert);
    let bad_tautology_run = compile_and_run_test(tmp.path(), "bad_tautology", bad_tautology);

    assert!(!base_run.success(), "negative phase: base must fail");
    assert!(good_run.success(), "positive phase: good candidate passes");
    assert_eq!(
        decision(base_run.success(), good_run.success(), good),
        "accept"
    );
    assert_eq!(
        decision(
            base_run.success(),
            bad_delete_run.success(),
            bad_delete_assert
        ),
        "reject_missing_assertion"
    );
    assert_eq!(
        decision(
            base_run.success(),
            bad_tautology_run.success(),
            bad_tautology
        ),
        "reject_tautology"
    );
}

fn compile_and_run_test(cwd: &Path, name: &str, source: &str) -> SanitizedOutput {
    let source_path = cwd.join(format!("{name}.rs"));
    let bin_path = cwd.join(format!("{name}_test"));
    std::fs::write(&source_path, source).expect("write source");
    let compile = run_sanitized(SanitizedCommand {
        program: PathBuf::from("rustc"),
        args: vec![
            "--test".into(),
            source_path.to_string_lossy().into_owned(),
            "-o".into(),
            bin_path.to_string_lossy().into_owned(),
        ],
        cwd: cwd.to_path_buf(),
        env: env_allowlist_from_current(&["PATH"]),
        stdin: None,
        timeout: Duration::from_secs(20),
    })
    .expect("compile test");
    if !compile.success() {
        return compile;
    }
    run_sanitized(SanitizedCommand {
        program: bin_path,
        args: vec!["--nocapture".into()],
        cwd: cwd.to_path_buf(),
        env: env_allowlist_from_current(&["PATH"]),
        stdin: None,
        timeout: Duration::from_secs(20),
    })
    .expect("run compiled test")
}

fn decision(base_passed: bool, candidate_passed: bool, source: &str) -> &'static str {
    if base_passed {
        return "reject_base_did_not_fail";
    }
    if source.contains("assert!(true)") {
        return "reject_tautology";
    }
    if !source.contains("assert_eq!(init_balance(), 0)") {
        return "reject_missing_assertion";
    }
    if candidate_passed {
        "accept"
    } else {
        "reject_candidate_failed"
    }
}
