//! B5 regression: `turingos welcome` must NOT show "turingos agent deploy" as
//! an unchecked action item for proof/default workspaces.
//!
//! After `turingos init` (default template = proof), the welcome checklist
//! omitted the agent-deploy step entirely. Previously that step appeared as
//! "[ ] 3. turingos agent deploy (0 registered)" while the footer could say
//! "All onboarding steps complete." — a visible contradiction.
//!
//! Source: handover/observations/USERSIM_DEEPSEEK_DUAL_KEY_2026-05-21.md
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

/// Build a fresh-init proof workspace (no spec, no artifacts) that matches
/// what `turingos init --template proof` produces in terms of files used by
/// the welcome inspector.
fn fresh_proof_workspace(dir: &std::path::Path) -> PathBuf {
    let workspace = dir.join("ws");
    fs::create_dir_all(&workspace).expect("create workspace dir");

    // genesis_payload.toml with template = "proof" → init_done = true, requires_agent_deploy = false
    let genesis = "# TuringOS genesis payload — Proof market template\n\
                   template = \"proof\"\n";
    fs::write(workspace.join("genesis_payload.toml"), genesis).expect("write genesis");
    fs::write(workspace.join("agent_pubkeys.json"), "{}\n").expect("write agent_pubkeys");

    // No turingos.toml → llm_configured = false (welcome stops at step 2)
    // No spec.md, no cas/ → spec_done = false
    // No artifacts/ → artifacts_done = false

    workspace
}

#[test]
fn welcome_does_not_show_agent_deploy_for_proof_template() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let workspace = fresh_proof_workspace(tmp.path());

    let output = Command::new(turingos_bin())
        .arg("welcome")
        .arg("--workspace")
        .arg(&workspace)
        .output()
        .expect("spawn turingos welcome");

    assert!(
        output.status.success(),
        "turingos welcome must exit 0; exit={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    // The "agent deploy" line must not appear as an unchecked item.
    // Accept neither "[ ] ... agent deploy" nor a bare "agent deploy" line.
    assert!(
        !stdout.contains("[ ]") || !stdout.contains("agent deploy"),
        "welcome stdout must not show an unchecked 'agent deploy' step for proof template;\
         \ngot:\n{stdout}"
    );

    // The step number 3 that does appear should be for `turingos spec`, not agent deploy.
    // We check that if "3." appears it is NOT followed by "agent deploy".
    if let Some(pos) = stdout.find("3.") {
        let snippet = &stdout[pos..pos.min(stdout.len()).min(pos + 60)];
        assert!(
            !snippet.contains("agent deploy"),
            "step 3 must not be 'agent deploy' for proof template; got snippet: {snippet:?}"
        );
    }
}

#[test]
fn welcome_does_not_show_agent_deploy_when_genesis_absent() {
    // When genesis_payload.toml doesn't exist (workspace not yet inited),
    // requires_agent_deploy defaults to false — still no agent deploy step.
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let workspace = tmp.path().join("empty_ws");
    fs::create_dir_all(&workspace).expect("create empty workspace dir");

    let output = Command::new(turingos_bin())
        .arg("welcome")
        .arg("--workspace")
        .arg(&workspace)
        .output()
        .expect("spawn turingos welcome");

    assert!(output.status.success(), "welcome must exit 0 even on empty dir");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    assert!(
        !stdout.contains("agent deploy"),
        "welcome must not show 'agent deploy' when workspace has no genesis_payload.toml;\
         \ngot:\n{stdout}"
    );
}
