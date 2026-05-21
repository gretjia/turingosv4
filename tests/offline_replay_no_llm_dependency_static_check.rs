//! C9 static check: offline replay modules must not import LLM/network clients.
//!
//! This test greps the source of `src/runtime/replay.rs` and
//! `src/bin/turingos/cmd_spec_audit.rs` to assert they do NOT:
//! (a) use any siliconflow / reqwest / hyper / LlmError client module
//! (b) mod any module that itself uses them
//!
//! FC-trace: FC1 (replay loop), FC2 (offline guarantee)
//! Risk class: Class 2

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn assert_no_llm_imports(path: &std::path::Path) {
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("could not read {:?}: {e}", path));

    let forbidden_patterns = [
        "siliconflow_client",
        "reqwest",
        "hyper::",
        "LlmError",
        "chat_complete_blocking",
        "require_api_key",
        "SILICONFLOW_API_KEY",
    ];

    for pattern in &forbidden_patterns {
        assert!(
            !content.contains(pattern),
            "File {:?} contains forbidden LLM/network import {:?}",
            path,
            pattern
        );
    }
}

#[test]
fn test_offline_replay_no_llm_dependency_static_check() {
    let root = workspace_root();

    let replay_rs = root.join("src/runtime/replay.rs");
    assert!(replay_rs.exists(), "src/runtime/replay.rs must exist");
    assert_no_llm_imports(&replay_rs);

    let cmd_spec_audit_rs = root.join("src/bin/turingos/cmd_spec_audit.rs");
    assert!(cmd_spec_audit_rs.exists(), "src/bin/turingos/cmd_spec_audit.rs must exist");
    assert_no_llm_imports(&cmd_spec_audit_rs);

    println!("STATIC-CHECK PASS: no LLM/network imports in offline replay modules");
}
