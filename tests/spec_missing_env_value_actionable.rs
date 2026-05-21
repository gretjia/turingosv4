//! Patch P4 — spec missing env value actionable error test.
//!
//! Verifies that `turingos spec` fails with a non-zero exit code AND prints
//! both the env var NAME and the literal string "export " when the configured
//! api_key_env var is set in turingos.toml but not in the shell environment.
//!
//! TRACE_MATRIX FC2-N16: CLI boot adapter (spec env-var error UX).
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

/// Workspace with both api_key_env slots set to the same fake name.
fn workspace_with_unset_key(dir: &std::path::Path) -> (std::path::PathBuf, std::path::PathBuf) {
    let workspace = dir.join("ws");
    fs::create_dir_all(&workspace).expect("create workspace dir");

    let config = "\
llm.meta.model = \"meta-model-stub\"\n\
llm.blackbox.model = \"blackbox-model-stub\"\n\
llm.meta.api_key_env = \"FAKE_KEY_UNSET_NAME\"\n\
llm.blackbox.api_key_env = \"FAKE_KEY_UNSET_NAME\"\n";
    fs::write(workspace.join("turingos.toml"), config).expect("write turingos.toml");

    // Provide a valid 8-element answers file so spec reaches the key reader.
    let answers_path = workspace.join("answers.json");
    fs::write(
        &answers_path,
        r#"["ans1","ans2","ans3","ans4","ans5","ans6","ans7","ans8"]"#,
    )
    .expect("write answers.json");

    (workspace, answers_path)
}

#[test]
fn spec_fails_with_actionable_export_hint_when_env_var_unset() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let (workspace, answers_path) = workspace_with_unset_key(tmp.path());

    let output = Command::new(turingos_bin())
        .arg("spec")
        .arg("--workspace")
        .arg(&workspace)
        .arg("--answers-file")
        .arg(&answers_path)
        .env_remove("FAKE_KEY_UNSET_NAME")
        .output()
        .expect("spawn turingos spec");

    assert!(
        !output.status.success(),
        "turingos spec must fail (non-zero exit) when env var is unset; \
         exit={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let combined = String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr);

    assert!(
        combined.contains("FAKE_KEY_UNSET_NAME"),
        "error output must contain the env var name 'FAKE_KEY_UNSET_NAME'; got:\n{combined}"
    );

    assert!(
        combined.contains("export "),
        "error output must contain 'export ' as a shell hint; got:\n{combined}"
    );
}
