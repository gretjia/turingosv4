//! TISR Phase 6.1 W2.1 — `turingos config` smoke tests.

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

#[test]
fn turingos_config_help_shows_description() {
    let output = Command::new(turingos_bin())
        .arg("config")
        .arg("--help")
        .output()
        .expect("run turingos");
    assert!(output.status.success(), "expected --help to succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("config") && (stdout.contains("set") || stdout.contains("get")),
        "help missing expected description; got: {stdout}"
    );
}

#[test]
fn turingos_config_set_writes_toml() {
    let tmp = tempfile::TempDir::new().expect("tmpdir");
    let output = Command::new(turingos_bin())
        .arg("config")
        .arg("set")
        .arg("foo")
        .arg("bar")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("run");
    assert!(
        output.status.success(),
        "config set failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let path = tmp.path().join("turingos.toml");
    assert!(path.is_file(), "turingos.toml not created");
    let content = std::fs::read_to_string(&path).expect("read");
    assert!(
        content.contains("foo = \"bar\""),
        "config content unexpected: {content}"
    );
}

#[test]
fn turingos_config_get_reads_value() {
    let tmp = tempfile::TempDir::new().expect("tmpdir");
    // First set
    Command::new(turingos_bin())
        .arg("config")
        .arg("set")
        .arg("name")
        .arg("phase6")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("set");
    // Then get
    let output = Command::new(turingos_bin())
        .arg("config")
        .arg("get")
        .arg("name")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("get");
    assert!(output.status.success(), "config get failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("phase6"),
        "expected 'phase6' in output; got: {stdout}"
    );
}

#[test]
fn turingos_config_list_shows_entries() {
    let tmp = tempfile::TempDir::new().expect("tmpdir");
    Command::new(turingos_bin())
        .arg("config")
        .arg("set")
        .arg("k1")
        .arg("v1")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("set k1");
    Command::new(turingos_bin())
        .arg("config")
        .arg("set")
        .arg("k2")
        .arg("v2")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("set k2");
    let output = Command::new(turingos_bin())
        .arg("config")
        .arg("list")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("list");
    assert!(output.status.success(), "config list failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("k1") && stdout.contains("k2"),
        "list missing entries; got: {stdout}"
    );
}

#[test]
fn turingos_config_get_missing_returns_nonzero() {
    let tmp = tempfile::TempDir::new().expect("tmpdir");
    let output = Command::new(turingos_bin())
        .arg("config")
        .arg("get")
        .arg("nonexistent_key")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("get");
    assert!(
        !output.status.success(),
        "expected non-zero exit on missing key"
    );
}

#[test]
fn turingos_config_bogus_action_returns_nonzero() {
    let tmp = tempfile::TempDir::new().expect("tmpdir");
    let output = Command::new(turingos_bin())
        .arg("config")
        .arg("zzz-bogus-action")
        .arg("--workspace")
        .arg(tmp.path())
        .output()
        .expect("bogus");
    assert!(
        !output.status.success(),
        "expected non-zero exit on bogus action"
    );
}
