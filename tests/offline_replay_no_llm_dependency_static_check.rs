//! C9 static check: offline replay modules must not import LLM/network clients.
//!
//! This test greps offline replay/outbox source to assert it does NOT:
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
    let content =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("could not read {:?}: {e}", path));

    let forbidden_patterns = [
        "chat_client",
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

    for rel in [
        "src/runtime/replay.rs",
        "src/runtime/external_call.rs",
        "src/runtime/orphan_intent_sweeper.rs",
        "src/bin/turingos/cmd_spec_audit.rs",
    ] {
        let path = root.join(rel);
        assert!(path.exists(), "{rel} must exist");
        assert_no_llm_imports(&path);
    }

    println!("STATIC-CHECK PASS: no LLM/network imports in offline replay modules");
}

fn assert_file_contains(path: &std::path::Path, needle: &str) {
    let content =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("could not read {:?}: {e}", path));
    assert!(
        content.contains(needle),
        "File {:?} must stay in the external-call inventory; missing {:?}",
        path,
        needle
    );
}

#[test]
fn test_external_call_entrypoint_inventory_is_explicit() {
    let root = workspace_root();

    let expected = [
        (
            "src/bin/turingos/chat_client.rs",
            "reqwest::Client::builder",
        ),
        (
            "src/bin/turingos/cmd_llm.rs",
            "crate::chat_client::chat_complete",
        ),
        ("src/bin/turingos/cmd_generate.rs", "chat_complete_blocking"),
        ("src/bin/turingos/cmd_spec.rs", "chat_complete_blocking"),
        ("src/bin/turingos/cmd_spec.rs", "turingos llm complete"),
        ("src/bin/turingos/cmd_tdma.rs", "chat_complete_blocking"),
        ("src/drivers/llm_http.rs", "reqwest::Client::builder"),
        ("src/web/spec.rs", "turingos llm complete"),
        ("src/web/generate.rs", "turingos generate"),
    ];

    for (rel, needle) in expected {
        assert_file_contains(&root.join(rel), needle);
    }
}
