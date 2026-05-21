//! Constitutional gate: Web layer must not fork off the CLI's canonical kernel.
//!
//! P7.z architectural invariant (post-2026-05-21 session #56): both binaries
//! (`turingos` CLI and `turingos_web`) must share the same `src/runtime/*`
//! kernel. The Web layer is a thin wrapper over the CLI:
//!   - LLM-bearing work (generate, spec grill, welcome, task open) → shell
//!     out to `turingos` CLI binary.
//!   - Read-only views (preview, build session view) → in-process library
//!     calls to `src/runtime/*` (the same library the CLI uses).
//!   - HTTP transport concerns (response shielding, WebSocket push) → web
//!     layer-only, but MUST NOT define new schema constants or call the LLM
//!     client directly.
//!
//! This test enforces two hard invariants via static text grep on the repo:
//!
//!   1. **Web never calls the LLM client directly.** All LLM dispatches must
//!      route through the CLI binary (which is the single canonical
//!      implementation that lives in `src/bin/turingos/cmd_*.rs`). If the web
//!      layer ever calls `chat_complete_blocking` or
//!      `siliconflow_client::require_api_key` itself, the kernel forks.
//!
//!   2. **Web never defines its own capsule schema constants.** All
//!      `pub const *_SCHEMA_ID` values must live in `src/runtime/*.rs`. If a
//!      file under `src/web/*.rs` declares its own schema constant, the
//!      schema namespace forks.
//!
//! These are mechanical checks; no human judgment. Any violation is an
//! architectural drift that must be repaired before ship.
//!
//! FC-trace: FC3 (canonical kernel boundary)
//! Risk class: Class 2 (production wire-up gate)

use std::path::Path;

fn walk_rust_files_in(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    if !dir.exists() {
        return out;
    }
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return out,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(walk_rust_files_in(&path));
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(path);
        }
    }
    out
}

#[test]
fn web_layer_never_calls_llm_client_directly() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let web_dir = root.join("src/web");

    if !web_dir.exists() {
        // If src/web/ doesn't exist, the invariant trivially holds.
        return;
    }

    // Tokens that, if found anywhere in `src/web/`, would indicate the web
    // layer has bypassed the CLI canonical path for an LLM call.
    let forbidden_tokens = [
        "chat_complete_blocking",
        "siliconflow_client::require_api_key",
        "siliconflow_client::chat_complete",
    ];

    let mut violations: Vec<(std::path::PathBuf, &str)> = Vec::new();

    for file in walk_rust_files_in(&web_dir) {
        let content = match std::fs::read_to_string(&file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for token in &forbidden_tokens {
            // Strip line comments to avoid false positives from rationale
            // discussions; keep block comments and code text. Cheap line filter.
            let code_only: String = content
                .lines()
                .filter(|line| !line.trim_start().starts_with("//"))
                .collect::<Vec<_>>()
                .join("\n");
            if code_only.contains(token) {
                violations.push((file.clone(), token));
            }
        }
    }

    if !violations.is_empty() {
        let detail: Vec<String> = violations
            .iter()
            .map(|(p, t)| format!("  {} contains forbidden token `{}`", p.display(), t))
            .collect();
        panic!(
            "WEB-CLI KERNEL INVARIANT VIOLATED: src/web/ must not call the LLM \
             client directly. All LLM-bearing work must shell out to the \
             `turingos` CLI binary (e.g. via `tokio::process::Command::new(\
             \"turingos\")`). Violations:\n{}",
            detail.join("\n")
        );
    }
}

#[test]
fn web_layer_never_defines_capsule_schema_ids() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let web_dir = root.join("src/web");

    if !web_dir.exists() {
        return;
    }

    let mut violations: Vec<(std::path::PathBuf, String)> = Vec::new();

    for file in walk_rust_files_in(&web_dir) {
        let content = match std::fs::read_to_string(&file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for (idx, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("//") {
                continue;
            }
            // Match `pub const <NAME>_SCHEMA_ID` patterns.
            // The canonical schema ids live in src/runtime/*.rs only.
            if (trimmed.contains("pub const ") || trimmed.contains("pub(crate) const "))
                && trimmed.contains("SCHEMA_ID")
                && trimmed.contains(":")
                && (trimmed.contains("&str") || trimmed.contains("String"))
            {
                violations.push((file.clone(), format!("L{}: {}", idx + 1, line.trim())));
            }
        }
    }

    if !violations.is_empty() {
        let detail: Vec<String> = violations
            .iter()
            .map(|(p, line)| format!("  {} {}", p.display(), line))
            .collect();
        panic!(
            "WEB-CLI KERNEL INVARIANT VIOLATED: src/web/ must not declare \
             `pub const *_SCHEMA_ID` constants. Schema namespace is owned by \
             `src/runtime/*` only. Import existing constants via \
             `use turingosv4::runtime::<module>::SCHEMA_ID;` instead. \
             Violations:\n{}",
            detail.join("\n")
        );
    }
}
