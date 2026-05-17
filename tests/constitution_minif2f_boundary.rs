//! Constitution gate boundary: MiniF2F is a development benchmark corpus, not
//! a fixed TuringOS kernel or OS-level constitution gate.
//!
//! The core constitution gate runner may test ChainTape/CAS/replay/evidence
//! invariants that were originally exercised with MiniF2F evidence, but it must
//! not directly invoke the `minif2f_v4` experiment package as a required merge
//! gate. Experiment package checks belong in explicit development validation.

use std::fs;

#[test]
fn core_constitution_runner_does_not_invoke_minif2f_package() {
    let runner_path = "scripts/run_constitution_gates.sh";
    let runner = fs::read_to_string(runner_path).expect("read constitution gate runner");

    let active_minif2f_lines: Vec<_> = runner
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let trimmed = line.trim_start();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                return None;
            }
            if line.contains("minif2f_v4") || line.contains("experiments/minif2f_v4") {
                return Some((idx + 1, line.to_string()));
            }
            None
        })
        .collect();

    assert!(
        active_minif2f_lines.is_empty(),
        "core constitution runner must not invoke the MiniF2F experiment package as a fixed gate: {active_minif2f_lines:?}"
    );
}

#[test]
fn root_workspace_does_not_include_minif2f_package() {
    let manifest = fs::read_to_string("Cargo.toml").expect("read root Cargo.toml");
    let workspace = manifest
        .split("[workspace]")
        .nth(1)
        .expect("root Cargo.toml has workspace section");
    let members_line = workspace
        .lines()
        .find(|line| line.trim_start().starts_with("members"))
        .expect("workspace members line");

    assert!(
        !members_line.contains("experiments/minif2f_v4"),
        "root workspace members must not include the MiniF2F experiment package"
    );
    assert!(
        workspace.contains("exclude = [\"experiments/minif2f_v4\"]"),
        "root workspace must explicitly exclude the MiniF2F experiment package so cargo test --workspace stays core-scoped"
    );

    let minif2f_manifest =
        fs::read_to_string("experiments/minif2f_v4/Cargo.toml").expect("read MiniF2F Cargo.toml");
    assert!(
        minif2f_manifest.contains("[workspace]"),
        "MiniF2F must be its own explicit opt-in workspace for manifest-path development runs"
    );
}
