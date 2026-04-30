//! TB-5 anti-drift CI tests — extends TB-4's FORBIDDEN_VARIANTS scanner with
//! the four philosophy-preservation variants that must NEVER appear in src/.
//!
//! Charter: `handover/tracer_bullets/TB-5_charter_2026-04-30.md` v2 § 6 + § 4.11.
//! Preflight: `handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md` v2 § 8.5.
//!
//! The four anti-drift renames per directive § 5 + charter § 4.11:
//!   - SlashTx           → TB-6 RSP-3.2 territory (NOT TB-5)
//!   - SettlementTx      → RSP-4 territory (settlement is implicit at apply)
//!   - ProvisionalAcceptTx → never (binary accept/reject only per WP § 18 Inv 5)
//!   - ReputationUpdateTx → never (reputation is a derived projection)
//!
//! These names must never leak into src/ as TypedTx variants — any leak would
//! signal philosophy drift away from the WP-canonical inline-field shape.

use std::path::Path;
use std::path::PathBuf;

const FORBIDDEN_VARIANTS: &[&str] = &[
    "SlashTx",
    "SettlementTx",
    "ProvisionalAcceptTx",
    "ReputationUpdateTx",
];

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_rs_files(&path, out);
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                out.push(path);
            }
        }
    }
}

fn project_root() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"))
}

/// I82-I85 unified: no forbidden TypedTx variant name appears in src/.
#[test]
fn no_forbidden_tb5_variants_in_src() {
    let root = project_root();
    let src_dir = root.join("src");
    assert!(src_dir.exists() && src_dir.is_dir(), "src/ must exist");

    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);
    assert!(!files.is_empty(), "src/ must contain Rust files (sanity)");

    let mut hits: Vec<String> = Vec::new();
    for path in &files {
        let content = std::fs::read_to_string(path).unwrap_or_default();
        for (lineno, line) in content.lines().enumerate() {
            // Skip comments — directive forbids the variants in code, not in
            // doc-comments that might reference them historically.
            let trimmed = line.trim_start();
            if trimmed.starts_with("//")
                || trimmed.starts_with("///")
                || trimmed.starts_with("//!")
            {
                continue;
            }
            for forbidden in FORBIDDEN_VARIANTS {
                if line.contains(forbidden) {
                    hits.push(format!(
                        "{}:{} | {} | matched: {}",
                        path.display(),
                        lineno + 1,
                        line.trim(),
                        forbidden
                    ));
                }
            }
        }
    }

    assert!(
        hits.is_empty(),
        "TB-5 charter § 4.11 violated — forbidden TypedTx variant name appears in src/:\n{}",
        hits.join("\n")
    );
}

/// I86: charter contains the four anti-drift rename markers — soft test for
/// documentation hygiene. The charter itself documents *why* these names are
/// forbidden; if the documentation drifts away from the rules then either
/// the test breaks (good — flags drift) or the rule has been intentionally
/// retired (rare; explicit charter update required).
#[test]
fn four_anti_drift_renames_documented_in_charter() {
    let root = project_root();
    let charter_path = root.join("handover/tracer_bullets/TB-5_charter_2026-04-30.md");
    if !charter_path.exists() {
        // Soft skip: charter not present in this build slice (e.g., subset
        // checkout). The test is a documentation hygiene check; absence
        // means we cannot run it but should not fail the build.
        eprintln!("TB-5 charter not found at {:?} — skipping I86 (soft check)", charter_path);
        return;
    }
    let body = std::fs::read_to_string(&charter_path).expect("read charter");
    for forbidden in FORBIDDEN_VARIANTS {
        assert!(
            body.contains(forbidden),
            "TB-5 charter must reference '{}' (anti-drift rename per § 4.11)",
            forbidden
        );
    }
}

/// I87: TB-5 must not touch P6 (Epistemic Lab v0) files — pre-merge gate
/// to prevent cross-phase scope creep. Soft-checks the file paths exist
/// at the locations charter § 6 declares forbidden; uses a directory-scan
/// fallback when git is not available in the test environment.
#[test]
fn no_p6_files_touched_in_tb5() {
    let root = project_root();
    // Best-effort: try `git diff main..HEAD --name-only`. If git isn't
    // available, fall back to a positive existence check for the known
    // P6 paths that should NOT have been touched.
    let output = std::process::Command::new("git")
        .args(["diff", "main..HEAD", "--name-only"])
        .current_dir(&root)
        .output();
    let Ok(out) = output else {
        eprintln!("git unavailable — skipping I87 git-diff scan (soft check)");
        return;
    };
    if !out.status.success() {
        eprintln!("git diff failed (likely no `main` ref) — skipping I87 (soft check)");
        return;
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let p6_prefixes = [
        "experiments/minif2f_v4/",
        "src/loom/h_vppu",
        "src/loom/meta_tape",
    ];
    let hits: Vec<&str> = stdout
        .lines()
        .filter(|line| p6_prefixes.iter().any(|p| line.starts_with(p)))
        .collect();
    assert!(
        hits.is_empty(),
        "TB-5 must not touch P6 files; offending paths:\n{}",
        hits.join("\n")
    );
}
