//! TB-C0 Constitution Landing Gate — No Parallel Ledger fence
//!
//! Dedicated Art. 0.2 Tape Canonical fence. Asserts the repo never
//! re-introduces a global filesystem pointer or shadow-tape that would
//! function as a parallel ledger source-of-truth.
//!
//! Lineage:
//! - OBS_R022 (TB-16 2026-05-04): `LATEST_MARKOV_CAPSULE.txt` was a
//!   global pointer; architect ruled Option α — delete the file, switch
//!   to per-runtime `markov_tip.cid` + `--markov-capsule-cid <hex>` CLI.
//! - TB-C0 directive 2026-05-06 §4.2 + §5.4: codify the absence as a
//!   permanent test, so this regression cannot reappear.
//!
//! See: `handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md`
//!      `handover/directives/2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md`
//!
//! All tests in this file are real assertions — no `assert!(true)` stubs
//! per CR-C0.1.

use std::path::Path;

const LEGACY_GLOBAL_MARKOV_POINTER: &str = "handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt";

/// FC3-INV2 / Art. 0.2 — the legacy global Markov pointer file MUST NOT
/// exist. Reintroduction means an Art. 0.2 parallel-ledger violation
/// (OBS_R022 regression).
#[test]
fn no_global_markov_pointer() {
    assert!(
        !Path::new(LEGACY_GLOBAL_MARKOV_POINTER).exists(),
        "Art. 0.2 violation: {LEGACY_GLOBAL_MARKOV_POINTER} re-appeared. \
         Per OBS_R022 closure 2026-05-04, this filesystem-side global \
         pointer was deleted because it functioned as a parallel ledger \
         source-of-truth. Markov capsule MUST be derived from \
         ChainTape + CAS — never read from a global file. \
         If you need per-runtime inheritance, use \
         `--prior-chain-runtime-repo <path>` reading `<path>/markov_tip.cid` \
         OR `--markov-capsule-cid <hex>` CLI."
    );
}

/// Art. 0.2 — no production code path may write or read
/// `LATEST_MARKOV_CAPSULE.txt` as authoritative input/output. Per
/// OBS_R022 §A.5 closure, no `fs::write` / `File::create` / `.write_all`
/// / `read_to_string` site may target this file name.
///
/// String-literal mentions in user-facing help text (audit_dashboard
/// help strings explaining the de-canonicalization to operators) ARE
/// permitted — those are informational, not authoritative I/O.
#[test]
fn no_parallel_ledger_source_of_truth() {
    use std::process::Command;
    // Tight pattern: any I/O call (read OR write) targeting the name.
    let out = Command::new("grep")
        .args([
            "-rEn",
            "--include=*.rs",
            r#"(fs::(write|read|read_to_string|create)|File::(create|open)|\.write_all|\.write_str|read_to_string)\([^)]*LATEST_MARKOV_CAPSULE"#,
            "src/",
            "experiments/",
        ])
        .output()
        .expect("grep should be available");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.trim().is_empty(),
        "Art. 0.2 violation: production code performs I/O on \
         LATEST_MARKOV_CAPSULE — global pointer parallel-ledger \
         regression. Found:\n{stdout}"
    );
}

/// Tape-canonical — the canonical chain ID surface (`Hash` / state-root /
/// LedgerEntry) must exist as a single source. We assert that the
/// `transition_ledger` module exposes the canonical type and that no
/// rival "shadow" ledger module exists at top level. This catches a
/// future "FastTape" / "SnapshotTape" / "DashboardTape" parallel module
/// from sneaking in.
#[test]
fn no_shadow_tape_authoritative_parent() {
    let bottom_white =
        std::fs::read_dir("src/bottom_white/ledger").expect("src/bottom_white/ledger should exist");
    let mut suspicious = Vec::new();
    for entry in bottom_white.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy().to_string();
        // The canonical module is `transition_ledger.rs`. A second `*_ledger.rs`
        // module would be an authoritative-parent risk unless explicitly named
        // as a derived view.
        if name_str.ends_with("_ledger.rs")
            && name_str != "transition_ledger.rs"
            && !name_str.contains("derived")
            && !name_str.contains("view")
            && !name_str.contains("test")
        {
            suspicious.push(name_str);
        }
    }
    assert!(
        suspicious.is_empty(),
        "Art. 0.2 violation: a sibling `*_ledger.rs` module exists alongside \
         `transition_ledger.rs` without `derived` / `view` / `test` markers. \
         A second authoritative-parent ledger violates Tape Canonical. \
         Suspicious: {suspicious:?}"
    );
}

/// Tape-canonical — global mutable Markov pointers (file-backed
/// singletons) are forbidden in production code. We assert by grep
/// that production code does not invent new global-pointer file names.
#[test]
fn no_global_pointer_files_introduced() {
    use std::process::Command;
    let out = Command::new("grep")
        .args([
            "-rEn",
            "--include=*.rs",
            r#"\.write_all\(.*LATEST_|fs::write\(.*LATEST_|write_file.*LATEST_"#,
            "src/",
        ])
        .output()
        .expect("grep should be available");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.trim().is_empty(),
        "Art. 0.2 violation: production code writes a `LATEST_*` global pointer file:\n{stdout}"
    );
}

/// FC2 / FC3 — the on-disk markov_capsules dir must not contain a file
/// whose name implies "the latest" beyond the per-run `markov_tip.cid`
/// pattern. The repo's only legitimate latest-pointer location post-
/// OBS_R022 is per-runtime under each runtime_repo.
#[test]
fn markov_capsules_dir_has_no_global_latest_marker() {
    let dir = Path::new("handover/markov_capsules");
    if !dir.exists() {
        // dir not yet created — vacuously OK
        return;
    }
    let entries = std::fs::read_dir(dir).expect("dir readable");
    for entry in entries.flatten() {
        let name = entry.file_name();
        let n = name.to_string_lossy().to_string();
        // forbid: LATEST_*, GLOBAL_*, CURRENT_*, SOURCE_OF_TRUTH_*
        for forbidden_prefix in ["LATEST_", "GLOBAL_", "CURRENT_", "SOURCE_OF_TRUTH_"] {
            assert!(
                !n.starts_with(forbidden_prefix),
                "Art. 0.2 violation: handover/markov_capsules/{n} re-introduces \
                 a global-pointer convention forbidden by OBS_R022."
            );
        }
    }
}
