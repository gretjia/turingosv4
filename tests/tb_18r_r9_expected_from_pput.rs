//! Regression coverage for TB-18R R9 postprocess expected-count extraction.
//!
//! FC trace: FC1 attempt accounting and FC3 evidence feedback. The helper is a
//! materialized evaluator-side view; it must match the chain-side invariant
//! terms without counting synthetic preseed gates.

use std::io::Write;
use std::process::{Command, Stdio};

use serde_json::Value;

const HELPER: &str = "handover/tests/scripts/tb_18r_expected_from_pput.py";

#[test]
fn p01_solved_partial_counts_omega_reject_and_partial_without_preseed() {
    let out = run_helper(
        r#"{"solved":true,"hit_max_tx":false,"tool_dist":{"step_reject":2,"omega_wtool":1,"step_partial_ok":1,"step":4}}"#,
    );

    assert_eq!(out["expected_completed_attempts"], 4);
    assert_eq!(out["halt_class"], "OmegaAccepted");
    assert_eq!(out["components"]["step_reject"], 2);
    assert_eq!(out["components"]["omega"], 1);
    assert_eq!(out["components"]["step_partial_ok"], 1);
}

#[test]
fn p02_max_tx_partial_counts_capsule_anchored_partials_without_preseed() {
    let out = run_helper(
        r#"{"solved":false,"hit_max_tx":true,"tool_dist":{"step_partial_ok":9,"step":12,"step_reject":3}}"#,
    );

    assert_eq!(out["expected_completed_attempts"], 12);
    assert_eq!(out["halt_class"], "MaxTxExhausted");
    assert_eq!(out["components"]["step_reject"], 3);
    assert_eq!(out["components"]["omega"], 0);
    assert_eq!(out["components"]["step_partial_ok"], 9);
}

fn run_helper(pput_json: &str) -> Value {
    let mut child = Command::new("python3")
        .arg(HELPER)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn expected helper");
    child
        .stdin
        .as_mut()
        .expect("helper stdin")
        .write_all(pput_json.as_bytes())
        .expect("write pput json");
    let output = child.wait_with_output().expect("wait helper");
    assert!(
        output.status.success(),
        "helper failed: status={} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("helper stdout json")
}
