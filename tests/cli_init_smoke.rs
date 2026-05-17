//! TISR Phase 6.0/6.1 alpha — `turingos init` smoke / integration test.
//!
//! Per §8 packet `2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md`
//! Section 4 allowed path: `tests/cli_*.rs`.
//!
//! Class 1 verification: confirms the init subcommand produces the expected
//! workspace scaffold and rejects double-init without --force.
//!
//! Real-problem witness scope (per UNIFIED_CLI_SPEC §9): smallest possible
//! happy-path that future Phase 6.1+ subcommands extend.

use std::path::PathBuf;
use std::process::Command;

/// Resolve the compiled `turingos` binary path. Tries debug then release,
/// then falls back to `cargo run` shape.
fn turingos_bin() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let candidates = [
        format!("{manifest_dir}/target/debug/turingos"),
        format!("{manifest_dir}/target/release/turingos"),
    ];
    for candidate in candidates.iter() {
        let path = PathBuf::from(candidate);
        if path.exists() {
            return path;
        }
    }
    panic!(
        "turingos binary not found at debug or release target paths; \
         run `cargo build --bin turingos` before running this smoke test"
    );
}

#[test]
fn turingos_init_creates_workspace_scaffold() {
    let tmp = tempfile::TempDir::new().expect("create tempdir");
    let project_dir = tmp.path().join("test_init_proj");

    let output = Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&project_dir)
        .output()
        .expect("run turingos");

    assert!(
        output.status.success(),
        "turingos init failed: status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    assert!(project_dir.exists(), "project dir not created");
    assert!(
        project_dir.join("runtime_repo").is_dir(),
        "runtime_repo subdir missing"
    );
    assert!(project_dir.join("cas").is_dir(), "cas subdir missing");
    assert!(
        project_dir.join("genesis_payload.toml").is_file(),
        "genesis_payload.toml missing"
    );
    assert!(
        project_dir.join("agent_pubkeys.json").is_file(),
        "agent_pubkeys.json missing"
    );
}

#[test]
fn turingos_init_rejects_existing_dir_without_force() {
    let tmp = tempfile::TempDir::new().expect("create tempdir");
    let project_dir = tmp.path().join("preexisting");
    std::fs::create_dir_all(&project_dir).expect("preexisting dir");

    let output = Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&project_dir)
        .output()
        .expect("run turingos");

    assert!(
        !output.status.success(),
        "init should fail when directory already exists without --force"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("already exists"),
        "expected 'already exists' error, got stderr: {stderr}"
    );
}

#[test]
fn turingos_init_proof_template_is_default() {
    let tmp = tempfile::TempDir::new().expect("create tempdir");
    let project_dir = tmp.path().join("proof_default");

    let output = Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&project_dir)
        .output()
        .expect("run turingos");

    assert!(output.status.success(), "init should succeed");

    let genesis_content = std::fs::read_to_string(project_dir.join("genesis_payload.toml"))
        .expect("read genesis_payload.toml");
    assert!(
        genesis_content.contains("template = \"proof\""),
        "default template should be proof; got:\n{genesis_content}",
    );
    assert!(
        genesis_content.contains("lean_proof"),
        "proof template should mention lean_proof market type"
    );
}

#[test]
fn turingos_init_polymarket_template_writes_correct_marker() {
    let tmp = tempfile::TempDir::new().expect("create tempdir");
    let project_dir = tmp.path().join("polymarket_proj");

    Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&project_dir)
        .arg("--template")
        .arg("polymarket")
        .output()
        .expect("run turingos");

    let genesis_content = std::fs::read_to_string(project_dir.join("genesis_payload.toml"))
        .expect("read genesis_payload.toml");
    assert!(
        genesis_content.contains("template = \"polymarket\""),
        "polymarket template marker missing"
    );
    assert!(
        genesis_content.contains("polymarket_event"),
        "polymarket market type missing"
    );
}

#[test]
fn turingos_init_multi_agent_template_writes_correct_marker() {
    let tmp = tempfile::TempDir::new().expect("create tempdir");
    let project_dir = tmp.path().join("multi_agent_proj");

    Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&project_dir)
        .arg("--template")
        .arg("multi-agent")
        .output()
        .expect("run turingos");

    let genesis_content = std::fs::read_to_string(project_dir.join("genesis_payload.toml"))
        .expect("read genesis_payload.toml");
    assert!(
        genesis_content.contains("template = \"multi-agent\""),
        "multi-agent template marker missing"
    );
    assert!(
        genesis_content.contains("multi_agent_arena"),
        "multi-agent market type missing"
    );
    assert!(
        genesis_content.contains("BullTrader"),
        "multi-agent template should mention BullTrader role"
    );
}
