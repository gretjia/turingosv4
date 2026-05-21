//! Patch P4 — welcome env-var guidance test.
//!
//! Verifies that `turingos welcome` prints the configured env-var NAME and
//! the literal string "export " when init + llm config are done but the
//! configured env vars are unset in the shell.
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

/// Set up a workspace that looks fully init+llm-configured but has fake env
/// var names that are guaranteed not to be set in the test process's environment.
fn workspace_with_fake_env_vars(dir: &std::path::Path) -> std::path::PathBuf {
    let workspace = dir.join("ws");
    fs::create_dir_all(&workspace).expect("create workspace dir");

    // genesis_payload.toml + agent_pubkeys.json → init_done = true
    fs::write(workspace.join("genesis_payload.toml"), "# stub\n").expect("write genesis");
    fs::write(workspace.join("agent_pubkeys.json"), "{}\n").expect("write agent_pubkeys");

    // turingos.toml with llm.meta.model + llm.blackbox.model → llm_configured = true
    // Both api_key_env slots set to fake names that are never in the environment.
    let config = "\
llm.meta.model = \"meta-model-stub\"\n\
llm.blackbox.model = \"blackbox-model-stub\"\n\
llm.meta.api_key_env = \"FAKE_NEVER_SET_META\"\n\
llm.blackbox.api_key_env = \"FAKE_NEVER_SET_WORKER\"\n";
    fs::write(workspace.join("turingos.toml"), config).expect("write turingos.toml");

    // No spec.md and no cas/ dir → spec_done = false → welcome enters the
    // init+llm+!spec branch where env-var guidance is printed.

    workspace
}

#[test]
fn welcome_prints_env_var_name_and_export_hint() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let workspace = workspace_with_fake_env_vars(tmp.path());

    // Spawn with FAKE vars explicitly removed so we know they're unset.
    let output = Command::new(turingos_bin())
        .arg("welcome")
        .arg("--workspace")
        .arg(&workspace)
        .env_remove("FAKE_NEVER_SET_META")
        .env_remove("FAKE_NEVER_SET_WORKER")
        .output()
        .expect("spawn turingos welcome");

    assert!(
        output.status.success(),
        "turingos welcome must exit 0 (informational, not failing); \
         exit={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(
        stdout.contains("FAKE_NEVER_SET_META"),
        "welcome stdout must contain the configured meta env var name; got:\n{stdout}"
    );

    assert!(
        stdout.contains("export "),
        "welcome stdout must contain 'export ' as a shell hint; got:\n{stdout}"
    );
}
