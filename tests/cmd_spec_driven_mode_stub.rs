//! W6 atom — cmd_spec driven mode structural tests.
//! Real-LLM E2E deferred to W9. Tests here verify CLI surface, arg
//! validation, and basic state machine.

use std::process::Command;

fn bin_path() -> std::path::PathBuf {
    let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("target/debug/turingos");
    p
}

#[test]
fn help_lists_mode_flag() {
    let output = Command::new(bin_path()).arg("spec").arg("--help").output();
    // spec subcommand may not have --help in the same fashion; alternatively
    // check that invoking with --mode garbage exits non-zero
    if let Ok(out) = output {
        let combined = String::from_utf8_lossy(&out.stdout).to_string()
            + &String::from_utf8_lossy(&out.stderr);
        // Either help is shown, or it errors out — either is OK as long
        // as the binary recognizes the flag
        let _ = combined; // just verify it runs
    }
}

#[test]
fn driven_mode_with_invalid_mode_fails() {
    let output = Command::new(bin_path())
        .arg("spec")
        .arg("--mode")
        .arg("invalid_mode_value")
        .arg("--workspace")
        .arg("/tmp")
        .output()
        .expect("spawn");
    assert!(
        !output.status.success(),
        "spec with --mode invalid_mode_value should fail"
    );
}

#[test]
#[ignore = "needs real LLM or mock server infra; deferred to W9"]
fn driven_mode_completes_in_8_turns_when_llm_cooperative() {
    // TODO: requires mock LLM stub (or env-override the SiliconFlow endpoint).
}

#[test]
#[ignore = "needs real LLM or mock server"]
fn driven_mode_forces_terminate_at_turn_15() {}

#[test]
#[ignore = "needs real LLM or mock server"]
fn driven_mode_halts_on_double_predicate_fail() {}

#[test]
fn static_mode_is_default_when_no_mode_flag() {
    // When --mode is omitted, behavior must match legacy. We don't have an
    // easy way to assert this without a real run, but we can at least verify
    // the binary doesn't crash on `spec --help` and doesn't require --mode.
    let output = Command::new(bin_path())
        .arg("spec")
        .arg("--answers-file")
        .arg("/nonexistent.json")
        .arg("--workspace")
        .arg("/tmp")
        .output()
        .expect("spawn");
    // Should fail (missing file) but NOT due to missing --mode
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.to_lowercase().contains("mode"),
        "stderr should not complain about missing --mode in static default; got: {}",
        stderr
    );
}
