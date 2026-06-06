//! A13 Agentic OS v0 E2E CLI: network-off market projection witness.
//!
//! FC-trace: FC1 predicates/write, FC2 replay. Risk: Class 2 fixture projection.

use std::path::{Path, PathBuf};
use std::process::Command;

fn turingos_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_turingos"))
}

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/os/hello_agentic_task.json")
}

fn json_file(path: &Path) -> serde_json::Value {
    let raw = std::fs::read_to_string(path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn assert_success(output: &std::process::Output, label: &str) {
    assert!(
        output.status.success(),
        "{label} failed: status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn run_fixture(run_dir: &Path) {
    let output = Command::new(turingos_bin())
        .args([
            "os",
            "run",
            "--task",
            fixture_path().to_str().expect("fixture path utf8"),
            "--policy",
            "single_tree",
            "--market",
            "on",
            "--network",
            "off",
            "--out-dir",
            run_dir.to_str().expect("run dir utf8"),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run turingos os run");
    assert_success(&output, "os run");
}

fn assert_json_numbers_are_integral(value: &serde_json::Value) {
    match value {
        serde_json::Value::Number(number) => {
            assert!(
                number.as_i64().is_some() || number.as_u64().is_some(),
                "money/economy JSON must not use floating point numbers: {number}"
            );
        }
        serde_json::Value::Array(items) => {
            for item in items {
                assert_json_numbers_are_integral(item);
            }
        }
        serde_json::Value::Object(map) => {
            for item in map.values() {
                assert_json_numbers_are_integral(item);
            }
        }
        _ => {}
    }
}

/// TRACE_MATRIX FC1 + FC2: market projection is derived, conserved, and integer-only.
#[test]
fn turingos_os_market_projection_is_conserved_and_replay_anchored() {
    let tmp = tempfile::tempdir().expect("create tempdir");
    let run_dir = tmp.path().join("market-run");
    run_fixture(&run_dir);

    let manifest = json_file(&run_dir.join("run_manifest.json"));
    let head = manifest["final_tape_head"]
        .as_str()
        .expect("manifest final_tape_head");
    let economy = json_file(&run_dir.join("economy_projection.json"));

    assert_eq!(economy["derived_from_tape_head"], head);
    assert_eq!(economy["market_mode"], "on");
    assert_eq!(economy["settlement_kind"], "network_off_fixture_projection");
    assert_eq!(economy["conservation_ok"], true);
    assert_eq!(
        economy["initial_supply_microcredits"], economy["final_supply_microcredits"],
        "network-off projection must conserve integer microcredits"
    );
    assert_json_numbers_are_integral(&economy);

    let external_receipts = std::fs::read_to_string(run_dir.join("external_call_receipts.jsonl"))
        .expect("read external call receipts");
    assert!(
        external_receipts.contains("\"terminal_kind\":\"NetworkOffMocked\""),
        "network-off run should close its external intent with a deterministic terminal"
    );
    assert!(
        !external_receipts.contains("\"pending\":true"),
        "network-off run must not leave zombie external intents"
    );
}
