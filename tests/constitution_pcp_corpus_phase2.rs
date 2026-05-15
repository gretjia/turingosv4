//! Constitution gate — PCP corpus phase-2 (MiniF2F-v2 misalignment, real-world adversarial).
//!
//! Per Gemini R1 Q8 forward-bind + TB-18B charter SG-18B.9 + CLAUDE.md §4.2
//! G-012: 9-class adversarial corpus must be derived from a real public
//! MiniF2F problem, not synthesized one-line theorems. Phase-1 synthetic
//! corpus (`cases/pcp_corpus/`) is preserved as predecessor; phase-2 is the
//! `feedback_real_problems_not_designed` upgrade required before TB-18B M2.
//!
//! These tests are the executable face of TB-18B charter SG-18B.9:
//! "Gemini Q8 phase-2 PCP corpus minimum 9-class landed at
//!  cases/pcp_corpus_phase2/ + tests/constitution_pcp_corpus_phase2.rs".
//!
//! `FC-trace: G-012 PCP soundness phase-2 + Art-I.1.1 PCP innocent-until-proven`.

use serde_json::Value;
use std::fs;
use std::path::Path;

const PHASE2_DIR: &str = "cases/pcp_corpus_phase2";
const PHASE2_MANIFEST: &str = "cases/pcp_corpus_phase2/MANIFEST.json";

fn read_manifest() -> Value {
    let body =
        fs::read_to_string(PHASE2_MANIFEST).unwrap_or_else(|e| panic!("read MANIFEST.json: {e}"));
    serde_json::from_str(&body).unwrap_or_else(|e| panic!("parse MANIFEST.json: {e}"))
}

/// SG-18B.9 — manifest is parseable + has 9-class minimum.
#[test]
fn pcp_corpus_phase2_manifest_is_parseable_and_has_9_classes() {
    let m = read_manifest();
    assert_eq!(m["schema_version"], "v1/pcp_corpus_phase2_manifest");
    let corpus = m["corpus"].as_array().expect("corpus is array");
    assert_eq!(
        corpus.len(),
        9,
        "PCP corpus phase-2 MUST have exactly 9 classes per CLAUDE.md §4.2 G-012; got {}",
        corpus.len()
    );
}

/// SG-18B.9 — every class entry has the required fields per phase-1 schema parity.
#[test]
fn pcp_corpus_phase2_every_entry_has_required_fields() {
    let m = read_manifest();
    let corpus = m["corpus"].as_array().expect("corpus is array");
    let required = [
        "id",
        "class",
        "fixture",
        "mutation",
        "attempt_outcome",
        "lean_verdict_kind",
        "rejection_class",
        "routes_to",
    ];
    for entry in corpus {
        for field in &required {
            assert!(
                entry.get(field).is_some(),
                "PCP corpus phase-2 entry missing field `{field}`: {entry}"
            );
        }
    }
}

/// SG-18B.9 — every fixture file referenced by manifest exists on disk.
#[test]
fn pcp_corpus_phase2_every_fixture_file_present() {
    let m = read_manifest();
    let corpus = m["corpus"].as_array().expect("corpus is array");
    for entry in corpus {
        let fixture = entry["fixture"].as_str().expect("fixture field is string");
        let path = Path::new(PHASE2_DIR).join(fixture);
        assert!(
            path.exists(),
            "PCP corpus phase-2 fixture `{fixture}` missing on disk at {}",
            path.display()
        );
        let body =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        assert!(
            !body.trim().is_empty(),
            "PCP corpus phase-2 fixture `{fixture}` is empty"
        );
    }
}

/// SG-18B.9 — every fixture is BASED on the mathd_algebra_107 canonical
/// statement (real-public-problem witness per `feedback_real_problems_not_designed`).
/// Each Lean fixture must reference the canonical theorem name root or signature.
#[test]
fn pcp_corpus_phase2_every_fixture_is_minif2f_derived() {
    let m = read_manifest();
    let corpus = m["corpus"].as_array().expect("corpus is array");
    for entry in corpus {
        let fixture = entry["fixture"].as_str().unwrap();
        let body = fs::read_to_string(Path::new(PHASE2_DIR).join(fixture)).expect("read fixture");
        // Must reference mathd_algebra_107 (theorem name) AND the canonical
        // (x y : ℝ) signature so the corpus cannot drift to designed shapes.
        assert!(
            body.contains("mathd_algebra_107"),
            "fixture `{fixture}` does not reference mathd_algebra_107 \
             (per `feedback_real_problems_not_designed` real-problem witness)"
        );
        assert!(
            body.contains("(x y : ℝ)"),
            "fixture `{fixture}` does not preserve the canonical (x y : ℝ) signature"
        );
    }
}

/// SG-18B.9 — invalid mutations route to L4.E; only the canonical valid
/// proof routes to L4_accepted. Per CLAUDE.md §4.2 G-012:
/// "Invalid proofs never enter L4 accepted."
#[test]
fn pcp_corpus_phase2_only_valid_routes_to_l4_accepted() {
    let m = read_manifest();
    let corpus = m["corpus"].as_array().expect("corpus is array");
    let mut accepted_count = 0usize;
    let mut l4e_count = 0usize;
    for entry in corpus {
        let id = entry["id"].as_str().unwrap();
        let routes_to = entry["routes_to"].as_str().unwrap();
        match routes_to {
            "L4_accepted" => {
                accepted_count += 1;
                assert_eq!(
                    id, "01_valid",
                    "Only id=01_valid may route to L4_accepted; got id={id}"
                );
            }
            "L4_E" => l4e_count += 1,
            other => panic!("unexpected routes_to value `{other}` on id={id}"),
        }
    }
    assert_eq!(
        accepted_count, 1,
        "exactly 1 entry routes to L4_accepted; got {accepted_count}"
    );
    assert_eq!(
        l4e_count, 8,
        "exactly 8 entries route to L4_E; got {l4e_count}"
    );
}

/// SG-18B.9 — rejection_class_u8 mapping is consistent with phase-1 schema.
/// LeanFailed=6, ParseFailed=7, SorryBlocked=8 per
/// `src/state/sequencer.rs::RejectionClass` discriminants.
#[test]
fn pcp_corpus_phase2_rejection_class_u8_consistent() {
    let m = read_manifest();
    let corpus = m["corpus"].as_array().expect("corpus is array");
    for entry in corpus {
        let id = entry["id"].as_str().unwrap();
        if entry["routes_to"] == "L4_accepted" {
            assert!(
                entry["rejection_class"].is_null(),
                "L4_accepted entry should have null rejection_class; id={id}"
            );
            continue;
        }
        let rc = entry["rejection_class"].as_str().unwrap();
        let rc_u8 = entry["rejection_class_u8"].as_u64().unwrap();
        match rc {
            "LeanFailed" => assert_eq!(rc_u8, 6, "LeanFailed must be u8=6; id={id}"),
            "ParseFailed" => assert_eq!(rc_u8, 7, "ParseFailed must be u8=7; id={id}"),
            "SorryBlocked" => assert_eq!(rc_u8, 8, "SorryBlocked must be u8=8; id={id}"),
            other => panic!("unknown rejection_class `{other}` on id={id}"),
        }
    }
}

/// SG-18B.9 — phase-2 covers all 9 G-012 mutation classes (CLAUDE.md §4.2).
/// The class set is pinned so a future drift fires this gate.
#[test]
fn pcp_corpus_phase2_covers_all_9_g_012_classes() {
    let m = read_manifest();
    let corpus = m["corpus"].as_array().expect("corpus is array");
    let mut classes: Vec<String> = corpus
        .iter()
        .map(|e| e["class"].as_str().unwrap().to_string())
        .collect();
    classes.sort();
    let mut expected: Vec<&str> = vec![
        "irrelevant_theorem",
        "mutated_invalid_proof",
        "off_by_one_arithmetic",
        "parse_invalid",
        "partial_then_final_invalid",
        "sorry_insertion",
        "type_mismatch",
        "valid_proof",
        "wrong_theorem_name",
    ];
    expected.sort();
    let observed: Vec<&str> = classes.iter().map(|s| s.as_str()).collect();
    assert_eq!(
        observed, expected,
        "PCP corpus phase-2 class coverage drift; observed != G-012 9-class set"
    );
}

/// SG-18B.9 — phase-2 does NOT silently replace phase-1 (predecessor preserved).
#[test]
fn pcp_corpus_phase2_predecessor_phase1_still_present() {
    assert!(
        Path::new("cases/pcp_corpus/MANIFEST.json").exists(),
        "phase-1 corpus MANIFEST.json missing — phase-2 must NOT replace phase-1; \
         CLAUDE.md §4.2 G-012 requires both as overlapping witnesses."
    );
    assert!(
        Path::new("cases/pcp_corpus_phase2/MANIFEST.json").exists(),
        "phase-2 corpus MANIFEST.json missing"
    );
}
