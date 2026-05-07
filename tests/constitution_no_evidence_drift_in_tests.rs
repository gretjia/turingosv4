//! Constitution Landing Gate — No Evidence Drift in Tests
//!
//! `feedback_no_retroactive_evidence_rewrite` (memory) explicitly forbids
//! modifying old evidence dirs. Yet 2026-05-07 investigation found 3 cargo
//! tests writing to committed historical evidence dirs each run:
//!   - tests/tb_7_atom6_chain_backed_smoke.rs   → tb_7_chaintape_smoke_2026-05-01/
//!   - tests/tb_13_chaintape_smoke.rs           → tb_13_chaintape_smoke_2026-05-03/
//!   - tests/tb_14_chaintape_smoke.rs           → tb_14_chaintape_smoke_2026-05-03/
//!
//! Each `cargo test --workspace` silently produced 11 file diffs across these
//! committed roots. Diagnostic + resolution at:
//!   handover/alignment/OBS_EVIDENCE_DRIFT_ROOT_CAUSE_2026-05-07.md
//!
//! Resolution: writes are now gated behind TURINGOS_TEST_REGENERATE_EVIDENCE=1
//! (default = skip). This gate prevents regression: any new test writing to
//! handover/evidence/<dated-dir>/ without an env-gate must trip this test.
//!
//! `FC-trace: FC2-INV5` — protects evidence immutability invariant
//! (deep-history requires explicit override).

use std::fs;
use std::path::Path;

const TESTS_DIR: &str = "tests";
const EVIDENCE_PATH_LITERAL: &str = "handover/evidence/";
const ENV_GATE_TOKEN: &str = "TURINGOS_TEST_REGENERATE_EVIDENCE";

fn collect_test_files() -> Vec<std::path::PathBuf> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let dir = Path::new(project_root).join(TESTS_DIR);
    let mut files = Vec::new();
    let entries = fs::read_dir(&dir).unwrap_or_else(|e| {
        panic!("cannot read {}: {e}", dir.display());
    });
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    files
}

/// Tests known to READ from handover/evidence/ but NOT write there. They
/// either write to `std::env::temp_dir()` or use the evidence dir purely as a
/// read-only fixture source. Allowlisted to avoid false-positives in the
/// heuristic below; verify each entry's write-target is OUTSIDE handover/evidence/
/// before adding.
const READ_ONLY_FIXTURE_TESTS: &[&str] = &[
    // Reads tb_13_real_llm_smoke fixture; writes pointer + verdict to std::env::temp_dir().
    "markov_pointer_de_canonicalize.rs",
];

/// Heuristic: a test writes to committed evidence if its source contains a
/// pattern like `evidence_dir.join(...)` paired with `std::fs::{write,copy}`,
/// OR a `std::fs::create_dir_all(evidence_dir)` where evidence_dir is bound to
/// a "handover/evidence/<dated-dir>" literal. The 3 tests fixed 2026-05-07
/// all use this pattern. The marker `evidence_dir` (variable name) is
/// load-bearing — it's how we distinguish write-here from read-only-fixture.
fn looks_like_evidence_write_pattern(content: &str) -> bool {
    let has_evidence_path_literal =
        content.contains(EVIDENCE_PATH_LITERAL);
    if !has_evidence_path_literal {
        return false;
    }
    // The 3 tests bind a variable named `evidence_dir` to the evidence path
    // and then write/copy/create_dir_all using it. If we see `evidence_dir`
    // joined with a write call in the same file, treat as write-pattern.
    let has_evidence_dir_var = content.contains("evidence_dir");
    if !has_evidence_dir_var {
        return false;
    }
    let writes_to_evidence_dir = content.contains("evidence_dir.join")
        || content.contains("create_dir_all(evidence_dir)")
        || content.contains("create_dir_all(&evidence_dir)");
    writes_to_evidence_dir
}

/// For every `tests/*.rs` matching the evidence-write pattern, the same file
/// must contain the env-gate token. This catches new tests that copy/paste
/// the old write-by-default pattern without env-gating it.
#[test]
fn every_test_writing_to_committed_evidence_must_be_env_gated() {
    let files = collect_test_files();
    let mut violations: Vec<String> = Vec::new();

    for file in &files {
        let file_name = file
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if READ_ONLY_FIXTURE_TESTS.contains(&file_name) {
            continue;
        }

        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if looks_like_evidence_write_pattern(&content)
            && !content.contains(ENV_GATE_TOKEN)
        {
            violations.push(format!(
                "{}: writes to committed evidence dir but lacks '{}' env-gate. \
                 Per OBS_EVIDENCE_DRIFT_ROOT_CAUSE_2026-05-07.md, all test \
                 writes to committed evidence dirs must be opt-in via this env var. \
                 If this test is read-only, add it to READ_ONLY_FIXTURE_TESTS.",
                file.display(),
                ENV_GATE_TOKEN
            ));
        }
    }

    assert!(
        violations.is_empty(),
        "FC2-INV5 evidence immutability violation:\n  - {}",
        violations.join("\n  - ")
    );
}

/// Direct regression guard for the 3 specific tests fixed 2026-05-07. If
/// any of them loses its env-gate, this test fails loudly with a pointer
/// to the resolution OBS.
#[test]
fn the_three_2026_05_07_fixed_tests_retain_env_gate() {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let fixed_tests = [
        "tests/tb_7_atom6_chain_backed_smoke.rs",
        "tests/tb_13_chaintape_smoke.rs",
        "tests/tb_14_chaintape_smoke.rs",
    ];

    for rel in &fixed_tests {
        let path = Path::new(project_root).join(rel);
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        assert!(
            content.contains(ENV_GATE_TOKEN),
            "REGRESSION GUARD: {} lost its TURINGOS_TEST_REGENERATE_EVIDENCE \
             env-gate. This was the bug that caused 2 stash cycles in session \
             2026-05-07 (cargo test workspace silently overwriting committed \
             evidence dirs). Re-add the gate before merging.",
            rel
        );
    }
}
