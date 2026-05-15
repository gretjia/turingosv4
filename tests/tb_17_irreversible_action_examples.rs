//! TB-17 SG-17.8 enforcement — `IRREVERSIBLE_ACTION_POLICY.md` well-formedness.
//!
//! Per 2026-05-05 architect verdict §B.7 SG-17.8 ("at least 8 candidate
//! actions: allow / deny / require-human / require-delay") + Q6.2 verbatim
//! 8-subtype list, this file enforces:
//!
//! - Atom 6 doc exists.
//! - §2 enumerates ≥8 named subtypes (matching the architect's verbatim list).
//! - §5 candidate-action matrix has ≥8 rows.
//! - All four verdict classes (allow / deny / require-human / require-delay)
//!   are exercised in the matrix.
//!
//! TRACE_MATRIX FC3-N44 fixture — atom 6 doc well-formedness.

use std::path::Path;

const ATOM6_PATH: &str = "handover/whitepapers/IRREVERSIBLE_ACTION_POLICY.md";

fn read_atom6() -> String {
    let path = Path::new(ATOM6_PATH);
    assert!(
        path.exists(),
        "IRREVERSIBLE_ACTION_POLICY.md not found at {} — SG-17.8 + atom 6 \
         require this file",
        ATOM6_PATH
    );
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read {}: {}", ATOM6_PATH, e))
}

#[test]
fn atom6_doc_exists() {
    let _ = read_atom6();
}

#[test]
fn atom6_enumerates_eight_architect_subtypes() {
    let body = read_atom6();
    let lower = body.to_lowercase();
    // Per 2026-05-05 architect Q6.2 verbatim list:
    let expected: [&str; 8] = [
        "external_api_write",
        "payment",
        "publication",
        "message_sending",
        "physical_actuation",
        "deletion",
        "legal_medical_financial",
        "credential_key_rotation",
    ];
    for subtype in expected.iter() {
        assert!(
            lower.contains(subtype),
            "atom 6 doc missing architect Q6.2 subtype: {}",
            subtype
        );
    }
}

#[test]
fn atom6_lists_eight_or_more_candidate_action_rows() {
    // Conservative: count `| #` table cells in §5 region. The doc uses a
    // markdown table with leading numeric cells `| 1 |`, `| 2 |`, etc.
    let body = read_atom6();
    let mut count = 0u32;
    for n in 1..=20 {
        // match `| <n> |` exactly (the leading cell of each table row in §5).
        let needle = format!("| {} |", n);
        if body.contains(&needle) {
            count += 1;
        }
    }
    assert!(
        count >= 8,
        "atom 6 §5 must list ≥8 candidate-action rows; found {}",
        count
    );
}

#[test]
fn atom6_exercises_all_four_verdict_classes() {
    let body = read_atom6();
    let lower = body.to_lowercase();
    // Each verdict class must appear as a verdict in §5.
    assert!(
        lower.contains("**allow**"),
        "atom 6 §5 must exercise verdict class 'allow'"
    );
    assert!(
        lower.contains("**deny**"),
        "atom 6 §5 must exercise verdict class 'deny'"
    );
    assert!(
        lower.contains("**require-human**"),
        "atom 6 §5 must exercise verdict class 'require-human'"
    );
    assert!(
        lower.contains("**require-delay**"),
        "atom 6 §5 must exercise verdict class 'require-delay'"
    );
}

#[test]
fn atom6_cites_cr_17_3() {
    let body = read_atom6();
    assert!(
        body.contains("CR-17.3"),
        "atom 6 doc must cite CR-17.3 (no irreversible external action)"
    );
}
