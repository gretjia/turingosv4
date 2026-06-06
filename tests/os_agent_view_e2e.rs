//! A13 Agentic OS v0 E2E CLI: shielded agent view witness.
//!
//! FC-trace: FC1 scoped read view, FC2 replay/audit. Risk: Class 2 fixture only.

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

/// TRACE_MATRIX FC1 + FC2: audit proves scoped agent views do not expose private fixture fields.
#[test]
fn turingos_os_agent_view_audit_shields_private_fixture_data() {
    let tmp = tempfile::tempdir().expect("create tempdir");
    let run_dir = tmp.path().join("agent-view-run");
    run_fixture(&run_dir);

    let manifest = json_file(&run_dir.join("run_manifest.json"));
    let head = manifest["final_tape_head"]
        .as_str()
        .expect("manifest final_tape_head");
    let agent_view_audit = json_file(&run_dir.join("agent_view_audit.json"));

    assert_eq!(agent_view_audit["derived_from_tape_head"], head);
    assert_eq!(agent_view_audit["hidden_leak_count"], 0);
    assert_eq!(agent_view_audit["private_oracle_exposed"], false);
    assert_eq!(
        agent_view_audit["public_view"]["task_id"],
        "a13-hello-agentic-os"
    );
    assert_eq!(
        agent_view_audit["public_view"]["result"],
        "hello-agentic-os"
    );

    let serialized = serde_json::to_string_pretty(&agent_view_audit).expect("serialize audit");
    assert!(
        !serialized.contains("SHOULD_NOT_LEAK_A13_PRIVATE_ORACLE"),
        "agent view audit must not leak the private oracle fixture"
    );

    let audit = Command::new(turingos_bin())
        .args([
            "os",
            "audit",
            "--run-dir",
            run_dir.to_str().expect("run dir utf8"),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run turingos os audit");
    assert_success(&audit, "os audit");

    let stdout = String::from_utf8_lossy(&audit.stdout);
    assert!(
        stdout.contains("PREDICATES-GREEN"),
        "audit stdout should include predicate verdict; got:\n{stdout}"
    );
    assert!(
        !stdout.contains("SHOULD_NOT_LEAK_A13_PRIVATE_ORACLE"),
        "audit stdout must not leak private fixture data"
    );
}
