//! Patch P2 — dual-key no-fallback contract tests.
//!
//! Verifies that `read_meta_api_key_env` and `read_blackbox_api_key_env` do NOT
//! fall back to the other role's key when only one slot is configured.
//!
//! TRACE_MATRIX FC2-N16: LLM client boot adapter (dual-key isolation).
//! Risk class: 2 (production wire-up, additive error paths).
//!
//! Test strategy: subprocess-based. Each test writes a minimal `turingos.toml`
//! to a tempdir, then invokes the binary using a subcommand that triggers the
//! relevant key reader. We assert:
//!   - exit code is non-zero (error, not silent fallback)
//!   - stderr/stdout contains the role-specific "not configured" message
//!   - the OTHER role's env var name does NOT appear in the error (no leakage)

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

/// Write a minimal turingos.toml with only the meta key slot set.
/// Returns the tmp dir (kept alive for test duration).
fn workspace_with_only_meta_key(dir: &std::path::Path) -> std::path::PathBuf {
    let workspace = dir.join("ws");
    fs::create_dir_all(&workspace).expect("create workspace dir");
    // Minimal turingos.toml: only meta key configured, no blackbox key.
    let config = "llm.meta.api_key_env = \"FAKE_META_KEY\"\n\
                  llm.meta.model = \"meta-model-stub\"\n\
                  llm.blackbox.model = \"blackbox-model-stub\"\n";
    fs::write(workspace.join("turingos.toml"), config).expect("write turingos.toml");
    workspace
}

/// Write a minimal turingos.toml with only the blackbox key slot set.
fn workspace_with_only_blackbox_key(dir: &std::path::Path) -> std::path::PathBuf {
    let workspace = dir.join("ws");
    fs::create_dir_all(&workspace).expect("create workspace dir");
    // Minimal turingos.toml: only blackbox key configured, no meta key.
    let config = "llm.blackbox.api_key_env = \"FAKE_BLACKBOX_KEY\"\n\
                  llm.meta.model = \"meta-model-stub\"\n\
                  llm.blackbox.model = \"blackbox-model-stub\"\n";
    fs::write(workspace.join("turingos.toml"), config).expect("write turingos.toml");
    workspace
}

/// `turingos generate` reads the blackbox key. When only the meta key is
/// configured, it must fail loud with BlackboxKeyEnvNotConfigured — not
/// silently borrow FAKE_META_KEY and proceed.
#[test]
fn blackbox_reader_errors_when_only_meta_key_configured() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let workspace = workspace_with_only_meta_key(tmp.path());

    // generate requires a spec.md; write a minimal one so it reaches the key
    // reader before erroring on a missing spec.
    fs::write(
        workspace.join("spec.md"),
        "<!-- spec stub for test -->\n# Test spec\n",
    )
    .expect("write spec.md");

    let output = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&workspace)
        .output()
        .expect("spawn turingos generate");

    assert!(
        !output.status.success(),
        "generate must fail when llm.blackbox.api_key_env is missing; \
         exit={:?}",
        output.status
    );

    let combined = String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr);

    // Must mention the blackbox slot, not silently use FAKE_META_KEY.
    assert!(
        combined.contains("blackbox") || combined.contains("api_key_env"),
        "error output should reference the blackbox api_key_env slot; got:\n{combined}"
    );

    // Must NOT have attempted to use FAKE_META_KEY (the meta key) as the
    // blackbox key — that would be the silent-fallback bug.
    assert!(
        !combined.contains("FAKE_META_KEY"),
        "generate must not fall back to the meta key; got:\n{combined}"
    );
}

/// `turingos spec` reads the meta key. When only the blackbox key is
/// configured, it must fail loud with MetaKeyEnvNotConfigured — not
/// silently borrow FAKE_BLACKBOX_KEY and proceed.
#[test]
fn meta_reader_errors_when_only_blackbox_key_configured() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let workspace = workspace_with_only_blackbox_key(tmp.path());

    // Provide a valid 8-element JSON array answers file so spec reaches the
    // key reader (which is checked after answers are parsed).
    let answers_path = workspace.join("answers.json");
    fs::write(
        &answers_path,
        r#"["ans1","ans2","ans3","ans4","ans5","ans6","ans7","ans8"]"#,
    )
    .expect("write answers");

    let output = Command::new(turingos_bin())
        .arg("spec")
        .arg("--workspace")
        .arg(&workspace)
        .arg("--answers-file")
        .arg(&answers_path)
        .output()
        .expect("spawn turingos spec");

    assert!(
        !output.status.success(),
        "spec must fail when llm.meta.api_key_env is missing; \
         exit={:?}",
        output.status
    );

    let combined = String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr);

    // Must mention the meta slot.
    assert!(
        combined.contains("meta") || combined.contains("api_key_env"),
        "error output should reference the meta api_key_env slot; got:\n{combined}"
    );

    // Must NOT have attempted to use FAKE_BLACKBOX_KEY as the meta key.
    assert!(
        !combined.contains("FAKE_BLACKBOX_KEY"),
        "spec must not fall back to the blackbox key; got:\n{combined}"
    );
}
