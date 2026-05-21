//! Atom-K (NB3 fix) — welcome endpoint override surface test.
//!
//! Verifies that `turingos welcome` prints a ⚠ warning line when
//! TURINGOS_SILICONFLOW_ENDPOINT is set to a non-default value, and that
//! it stays silent when the env var is absent.
//!
//! TRACE_MATRIX FC2-N16: CLI boot adapter (welcome onboarding UX).
//! Risk class: 1 (additive UX only).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn turingos_bin() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let debug = PathBuf::from(format!("{manifest_dir}/target/debug/turingos"));
    let release = PathBuf::from(format!("{manifest_dir}/target/release/turingos"));
    if debug.exists() {
        return debug;
    }
    if release.exists() {
        return release;
    }
    panic!(
        "turingos binary not found at debug or release paths; \
         run `cargo build --bin turingos` first"
    );
}

/// Build a workspace that is init+llm-configured but spec not yet done,
/// so welcome enters the branch where env-var and endpoint checks run.
fn workspace_llm_configured(dir: &std::path::Path) -> std::path::PathBuf {
    let workspace = dir.join("ws");
    fs::create_dir_all(&workspace).expect("create workspace dir");

    // genesis_payload.toml + agent_pubkeys.json → init_done = true
    fs::write(workspace.join("genesis_payload.toml"), "# stub\n").expect("write genesis");
    fs::write(workspace.join("agent_pubkeys.json"), "{}\n").expect("write agent_pubkeys");

    // turingos.toml with llm.meta.model + llm.blackbox.model → llm_configured = true
    let config = "\
llm.meta.model = \"meta-model-stub\"\n\
llm.blackbox.model = \"blackbox-model-stub\"\n\
llm.meta.api_key_env = \"FAKE_KEY_META\"\n\
llm.blackbox.api_key_env = \"FAKE_KEY_WORKER\"\n";
    fs::write(workspace.join("turingos.toml"), config).expect("write turingos.toml");

    // No spec.md → spec_done = false → welcome enters the init+llm+!spec branch.

    workspace
}

const OVERRIDE_URL: &str = "https://api.example.com/v1/chat/completions";
const DEFAULT_URL: &str = "https://api.siliconflow.cn/v1/chat/completions";

#[test]
fn welcome_shows_warning_when_endpoint_overridden() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let workspace = workspace_llm_configured(tmp.path());

    let output = Command::new(turingos_bin())
        .arg("welcome")
        .arg("--workspace")
        .arg(&workspace)
        .env("TURINGOS_SILICONFLOW_ENDPOINT", OVERRIDE_URL)
        // Ensure the fake api key vars are unset so the warning path is reached.
        .env_remove("FAKE_KEY_META")
        .env_remove("FAKE_KEY_WORKER")
        .output()
        .expect("spawn turingos welcome");

    assert!(
        output.status.success(),
        "turingos welcome must exit 0; \
         exit={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(
        stdout.contains("TURINGOS_SILICONFLOW_ENDPOINT overridden"),
        "welcome stdout must contain 'TURINGOS_SILICONFLOW_ENDPOINT overridden'; got:\n{stdout}"
    );
    assert!(
        stdout.contains(OVERRIDE_URL),
        "welcome stdout must contain the override URL {OVERRIDE_URL}; got:\n{stdout}"
    );
    assert!(
        stdout.contains(DEFAULT_URL),
        "welcome stdout must contain the default URL {DEFAULT_URL}; got:\n{stdout}"
    );
}

#[test]
fn welcome_silent_when_endpoint_not_overridden() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let workspace = workspace_llm_configured(tmp.path());

    let output = Command::new(turingos_bin())
        .arg("welcome")
        .arg("--workspace")
        .arg(&workspace)
        // Remove the override so default is used.
        .env_remove("TURINGOS_SILICONFLOW_ENDPOINT")
        .env_remove("FAKE_KEY_META")
        .env_remove("FAKE_KEY_WORKER")
        .output()
        .expect("spawn turingos welcome");

    assert!(
        output.status.success(),
        "turingos welcome must exit 0; \
         exit={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(
        !stdout.contains("TURINGOS_SILICONFLOW_ENDPOINT overridden"),
        "welcome stdout must NOT contain endpoint override warning when env var is unset; got:\n{stdout}"
    );
}
