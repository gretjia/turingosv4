use std::fs;
use std::path::{Path, PathBuf};

const COMMAND_PATTERNS: &[&str] = &[
    "Command::new",
    "std::process::Command::new",
    "tokio::process::Command::new",
];

const ALLOWED_EXCEPTIONS: &[(&str, &str)] = &[
    (
        "src/sdk/sanitized_runner.rs",
        "single process-hygiene boundary that owns child process creation",
    ),
    (
        "src/sdk/sandbox.rs",
        "legacy LocalProcessSandbox dead-code adapter retained until removed",
    ),
    (
        "src/bin/turingos_dev.rs",
        "dev-only harness shell-out path, not product runtime",
    ),
    (
        "src/runtime/dev_harness.rs",
        "dev-only harness shell-out path, not product runtime",
    ),
    (
        "src/bin/turingos/cmd_wizard.rs",
        "OS-interactive local UX helpers: opener/stty are explicit exceptions",
    ),
];

#[test]
fn production_shellouts_must_use_sanitized_runner() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let src = repo.join("src");
    let mut violations = Vec::new();

    visit_rs_files(&src, &mut |path| {
        let rel = path.strip_prefix(&repo).expect("path under repo");
        let rel = rel.to_string_lossy().replace('\\', "/");
        let text = fs::read_to_string(path).expect("source file is readable");

        for (idx, line) in text.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("//") {
                continue;
            }
            if !COMMAND_PATTERNS
                .iter()
                .any(|pattern| line.contains(pattern))
            {
                continue;
            }
            if ALLOWED_EXCEPTIONS
                .iter()
                .any(|(allowed, _)| *allowed == rel)
            {
                continue;
            }
            violations.push(format!("{}:{}: {}", rel, idx + 1, line.trim()));
        }
    });

    assert!(
        violations.is_empty(),
        "production shell-outs must route through sanitized_runner; violations:\n{}",
        violations.join("\n")
    );
}

fn visit_rs_files(dir: &Path, f: &mut dyn FnMut(&Path)) {
    for entry in fs::read_dir(dir).expect("source dir is readable") {
        let entry = entry.expect("dir entry is readable");
        let path = entry.path();
        if path.is_dir() {
            visit_rs_files(&path, f);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            f(&path);
        }
    }
}
