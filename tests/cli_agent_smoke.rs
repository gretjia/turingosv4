//! TISR Phase 6.1 W2.2 — `turingos agent` smoke tests.

use std::path::PathBuf;
use std::process::Command;

fn turingos_bin() -> PathBuf {
    let mut path = std::env::current_exe()
        .expect("current_exe")
        .parent()
        .expect("exe parent")
        .to_path_buf();
    path.pop();
    path.push("turingos");
    if !path.exists() {
        path.pop();
        path.pop();
        path.push("release");
        path.push("turingos");
    }
    assert!(
        path.exists(),
        "turingos binary not found at {}",
        path.display()
    );
    path
}

const VALID_PUBKEY: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

#[test]
fn turingos_agent_help_shows_description() {
    let output = Command::new(turingos_bin())
        .arg("agent")
        .arg("--help")
        .output()
        .expect("run");
    assert!(output.status.success(), "agent --help failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("agent") && (stdout.contains("deploy") || stdout.contains("AgentRole")),
        "help missing expected description; got: {stdout}"
    );
}

#[test]
fn turingos_agent_deploy_writes_json() {
    let tmp = tempfile::TempDir::new().expect("tmpdir");
    let output = Command::new(turingos_bin())
        .arg("agent")
        .arg("deploy")
        .arg("--id")
        .arg("agent_001")
        .arg("--pubkey")
        .arg(VALID_PUBKEY)
        .arg("--role")
        .arg("Solver")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("deploy");
    assert!(
        output.status.success(),
        "deploy failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json_path = tmp.path().join("agent_pubkeys.json");
    assert!(json_path.is_file(), "agent_pubkeys.json not created");
    let content = std::fs::read_to_string(&json_path).expect("read");
    assert!(
        content.contains("agent_001") && content.contains("Solver"),
        "json missing fields: {content}"
    );
}

#[test]
fn turingos_agent_list_shows_deployed() {
    let tmp = tempfile::TempDir::new().expect("tmpdir");
    Command::new(turingos_bin())
        .arg("agent")
        .arg("deploy")
        .arg("--id")
        .arg("agent_001")
        .arg("--pubkey")
        .arg(VALID_PUBKEY)
        .arg("--role")
        .arg("Solver")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("deploy");
    let output = Command::new(turingos_bin())
        .arg("agent")
        .arg("list")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("list");
    assert!(output.status.success(), "list failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("agent_001") && stdout.contains("Solver"),
        "list missing entry; got: {stdout}"
    );
}

#[test]
fn turingos_agent_view_shows_pubkey() {
    let tmp = tempfile::TempDir::new().expect("tmpdir");
    Command::new(turingos_bin())
        .arg("agent")
        .arg("deploy")
        .arg("--id")
        .arg("agent_007")
        .arg("--pubkey")
        .arg(VALID_PUBKEY)
        .arg("--role")
        .arg("Architect")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("deploy");
    let output = Command::new(turingos_bin())
        .arg("agent")
        .arg("view")
        .arg("--id")
        .arg("agent_007")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("view");
    assert!(output.status.success(), "view failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Architect") && stdout.contains(VALID_PUBKEY),
        "view missing pubkey/role; got: {stdout}"
    );
}

#[test]
fn turingos_agent_invalid_pubkey_length_rejected() {
    let tmp = tempfile::TempDir::new().expect("tmpdir");
    let output = Command::new(turingos_bin())
        .arg("agent")
        .arg("deploy")
        .arg("--id")
        .arg("agent_002")
        .arg("--pubkey")
        .arg("0123456789abcdef0123456789abcdef") // 32 chars; too short
        .arg("--role")
        .arg("Solver")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("deploy");
    assert!(
        !output.status.success(),
        "expected non-zero exit on invalid pubkey"
    );
}

#[test]
fn turingos_agent_invalid_role_rejected() {
    let tmp = tempfile::TempDir::new().expect("tmpdir");
    let output = Command::new(turingos_bin())
        .arg("agent")
        .arg("deploy")
        .arg("--id")
        .arg("agent_002")
        .arg("--pubkey")
        .arg(VALID_PUBKEY)
        .arg("--role")
        .arg("NotARealRole")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("deploy");
    assert!(
        !output.status.success(),
        "expected non-zero exit on invalid role"
    );
}

// ───────────────────────────────────────────────────────────────────────────
// R3 finding: empty-list deploy hint must shell-quote workspace paths so
// copy-paste survives whitespace.
// ───────────────────────────────────────────────────────────────────────────

#[test]
fn turingos_agent_list_empty_hint_quotes_whitespace_path() {
    // Create a workspace whose path contains a space, run `agent list`
    // against the freshly-init'd dir (no agents registered yet), and
    // verify the deploy hint quotes the workspace path single-quote-style.
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let space_path = tmp.path().join("ws with space");
    std::fs::create_dir(&space_path).expect("mkdir space-path");

    let output = Command::new(turingos_bin())
        .arg("agent")
        .arg("list")
        .arg("--workspace")
        .arg(&space_path)
        .output()
        .expect("run turingos agent list");

    assert!(
        output.status.success(),
        "agent list on a clean workspace should exit 0; status={:?}",
        output.status,
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let space_str = space_path.to_string_lossy().to_string();
    let quoted = format!("'{}'", space_str.replace('\'', r"'\''"));
    assert!(
        stdout.contains(&quoted),
        "deploy hint should contain shell-quoted workspace path {quoted:?}; \
         got: {stdout:?}",
    );
}
