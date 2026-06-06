//! A13 Agentic OS v0 E2E CLI: boot -> run -> replay.
//!
//! FC-trace: FC1 runtime loop, FC2 boot/replay, FC3 audit feedback archive.
//! Risk: Class 2, network-off deterministic fixture only.

use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

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

fn write_json_file(path: &Path, value: &serde_json::Value) {
    let bytes = serde_json::to_vec_pretty(value).expect("serialize json");
    std::fs::write(path, [bytes, b"\n".to_vec()].concat())
        .unwrap_or_else(|err| panic!("write {path:?}: {err}"));
}

fn sha256_cid(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(71);
    out.push_str("sha256:");
    for byte in digest {
        out.push_str(&format!("{byte:02x}"));
    }
    out
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

/// TRACE_MATRIX FC2: boot manifest verification is available before an OS run.
#[test]
fn turingos_boot_verify_manifest_exits_zero() {
    let output = Command::new(turingos_bin())
        .arg("boot")
        .arg("--verify-manifest")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run turingos boot --verify-manifest");

    assert_success(&output, "boot verify-manifest");
}

/// TRACE_MATRIX FC1 + FC2 + FC3: network-off OS run produces replayable artifacts.
#[test]
fn turingos_os_run_produces_replayable_artifacts() {
    let tmp = tempfile::tempdir().expect("create tempdir");
    let run_dir = tmp.path().join("a13-run");

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

    for artifact in [
        "run_manifest.json",
        "replay_report.json",
        "predicate_receipts.jsonl",
        "external_call_receipts.jsonl",
        "economy_projection.json",
        "agent_view_audit.json",
    ] {
        assert!(
            run_dir.join(artifact).is_file(),
            "missing OS run artifact: {artifact}"
        );
    }
    assert!(
        run_dir.join("git_tape_repo/.git").is_dir(),
        "run must include a fsck-able git_tape_repo"
    );

    let fsck = Command::new("git")
        .arg("-C")
        .arg(run_dir.join("git_tape_repo"))
        .arg("fsck")
        .arg("--full")
        .output()
        .expect("git fsck --full");
    assert_success(&fsck, "git fsck");

    let manifest = json_file(&run_dir.join("run_manifest.json"));
    assert_eq!(manifest["network_policy"], "off");
    assert_eq!(manifest["policy"], "single_tree");
    assert_eq!(manifest["market_mode"], "on");
    let task_fixture_cid = manifest["task_fixture_cid"]
        .as_str()
        .expect("manifest task_fixture_cid");
    let task_fixture_hex = task_fixture_cid
        .strip_prefix("sha256:")
        .expect("fixture cid must be sha256");
    assert!(
        run_dir
            .join("cas/objects/sha256")
            .join(task_fixture_hex)
            .is_file(),
        "run must include the task fixture CAS object used by replay"
    );
    let head = manifest["final_tape_head"]
        .as_str()
        .expect("manifest final_tape_head");
    assert_eq!(head.len(), 40, "git tape head should be a git object id");
    assert!(
        manifest["derived_artifacts"]
            .as_array()
            .expect("derived artifacts array")
            .len()
            >= 5,
        "manifest should name the derived artifacts and hashes"
    );

    let replay = Command::new(turingos_bin())
        .args([
            "os",
            "replay",
            "--run-dir",
            run_dir.to_str().expect("run dir utf8"),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run turingos os replay");
    assert_success(&replay, "os replay");

    let replay_report = json_file(&run_dir.join("replay_report.json"));
    assert_eq!(replay_report["derived_from_tape_head"], head);
    assert_eq!(replay_report["deterministic_replay_ok"], true);
    assert_eq!(replay_report["pending_external_intents"], 0);
    assert_eq!(replay_report["unsupported_success_claims"], 0);
}

/// TRACE_MATRIX FC2: replay rejects manifests that omit required derived views.
#[test]
fn turingos_os_replay_rejects_manifest_missing_required_artifacts() {
    let tmp = tempfile::tempdir().expect("create tempdir");
    let run_dir = tmp.path().join("missing-artifact-run");

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

    let manifest_path = run_dir.join("run_manifest.json");
    let mut manifest = json_file(&manifest_path);
    manifest["derived_artifacts"]
        .as_array_mut()
        .expect("derived artifacts array")
        .retain(|artifact| artifact["path"] != "economy_projection.json");
    write_json_file(&manifest_path, &manifest);

    let replay = Command::new(turingos_bin())
        .args([
            "os",
            "replay",
            "--run-dir",
            run_dir.to_str().expect("run dir utf8"),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run turingos os replay");

    assert!(
        !replay.status.success(),
        "replay must fail when a required derived artifact is removed"
    );
    let stderr = String::from_utf8_lossy(&replay.stderr);
    assert!(
        stderr.contains("run manifest missing derived artifact economy_projection.json"),
        "unexpected stderr:\n{stderr}"
    );
}

/// TRACE_MATRIX FC2: replay rejects manifest paths that escape the run directory.
#[test]
fn turingos_os_replay_rejects_unsafe_manifest_paths() {
    let tmp = tempfile::tempdir().expect("create tempdir");
    let run_dir = tmp.path().join("unsafe-path-run");

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

    let manifest_path = run_dir.join("run_manifest.json");
    let mut manifest = json_file(&manifest_path);
    manifest["derived_artifacts"][0]["path"] = serde_json::json!("../escape.json");
    write_json_file(&manifest_path, &manifest);

    let replay = Command::new(turingos_bin())
        .args([
            "os",
            "replay",
            "--run-dir",
            run_dir.to_str().expect("run dir utf8"),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run turingos os replay");

    assert!(
        !replay.status.success(),
        "replay must fail when a manifest path escapes the run directory"
    );
    let stderr = String::from_utf8_lossy(&replay.stderr);
    assert!(
        stderr.contains("derived artifact path is unsafe: ../escape.json"),
        "unexpected stderr:\n{stderr}"
    );
}

/// TRACE_MATRIX FC2: replay reconstructs derived artifacts from GitTape/CAS, not manifest hashes.
#[test]
fn turingos_os_replay_rejects_manifest_hash_that_matches_tampered_derived_view() {
    let tmp = tempfile::tempdir().expect("create tempdir");
    let run_dir = tmp.path().join("tampered-derived-run");

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

    let economy_path = run_dir.join("economy_projection.json");
    let mut economy = json_file(&economy_path);
    economy["final_supply_microcredits"] = serde_json::json!(777);
    economy["conservation_ok"] = serde_json::json!(false);
    write_json_file(&economy_path, &economy);
    let tampered_hash = sha256_cid(
        &std::fs::read(&economy_path)
            .unwrap_or_else(|err| panic!("read tampered economy projection: {err}")),
    );

    let manifest_path = run_dir.join("run_manifest.json");
    let mut manifest = json_file(&manifest_path);
    for artifact in manifest["derived_artifacts"]
        .as_array_mut()
        .expect("derived artifacts array")
    {
        if artifact["path"] == "economy_projection.json" {
            artifact["content_hash_or_cid"] = serde_json::json!(tampered_hash);
        }
    }
    write_json_file(&manifest_path, &manifest);

    let replay = Command::new(turingos_bin())
        .args([
            "os",
            "replay",
            "--run-dir",
            run_dir.to_str().expect("run dir utf8"),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run turingos os replay");

    assert!(
        !replay.status.success(),
        "replay must fail even when a tampered derived artifact matches its tampered manifest hash"
    );
    let stderr = String::from_utf8_lossy(&replay.stderr);
    assert!(
        stderr.contains("artifact manifest hash is not the GitTape/CAS reconstruction for economy_projection.json"),
        "unexpected stderr:\n{stderr}"
    );
}

/// TRACE_MATRIX FC2: replay rejects CAS mutation instead of trusting derived artifacts.
#[test]
fn turingos_os_replay_rejects_task_fixture_cas_tamper() {
    let tmp = tempfile::tempdir().expect("create tempdir");
    let run_dir = tmp.path().join("tampered-cas-run");

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

    let manifest = json_file(&run_dir.join("run_manifest.json"));
    let task_fixture_cid = manifest["task_fixture_cid"]
        .as_str()
        .expect("manifest task_fixture_cid");
    let task_fixture_hex = task_fixture_cid
        .strip_prefix("sha256:")
        .expect("fixture cid must be sha256");
    std::fs::write(
        run_dir.join("cas/objects/sha256").join(task_fixture_hex),
        b"{\"schema\":\"tampered\"}\n",
    )
    .expect("tamper CAS object");

    let replay = Command::new(turingos_bin())
        .args([
            "os",
            "replay",
            "--run-dir",
            run_dir.to_str().expect("run dir utf8"),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run turingos os replay");

    assert!(
        !replay.status.success(),
        "replay must fail when the task fixture CAS object is mutated"
    );
    let stderr = String::from_utf8_lossy(&replay.stderr);
    assert!(
        stderr.contains("CAS object hash mismatch"),
        "unexpected stderr:\n{stderr}"
    );
}
