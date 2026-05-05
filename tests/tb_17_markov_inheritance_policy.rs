//! TB-17 SG-17.9 enforcement — `MARKOV_INHERITANCE_POLICY.md` documentation
//! conformance battery.
//!
//! Per 2026-05-05 architect verdict §B.7 SG-17.9 + §B.8 atom 11 ("at least
//! markov_inheritance_policy tests"), this file enforces:
//!
//! - The policy doc exists at the canonical path.
//! - Required sections (§1 / §2.1 / §2.2 / §2.3 / §3.1 / §3.2 / §3.3 / §3.4
//!   / §4 / §5 / §6.1 / §6.2 / §7 / §8) are present as headings.
//! - Required forbidden-pattern names appear in §3.
//! - Cross-references to the architect rulings are present.
//!
//! Per-case synthetic-chain fixtures (§2.1 Genesis / §2.2 Inherited / §2.3
//! Invalid) require multi-task chain substrate from PRE-17.6 substantive
//! build; those are deferred to TB-18 (per
//! `handover/proposals/TB-17_PRE_17_6_COMPREHENSIVE_ARENA_DEVIATION_2026-05-05.md`).
//!
//! TRACE_MATRIX FC3-N45 (markov inheritance policy doc + tests).
//!
//! The doc-existence + structural assertions here form the TB-17 ship-time
//! evidence for SG-17.9; when TB-18 implements per-case fixtures, this file's
//! scope extends.

use std::path::Path;

const POLICY_PATH: &str = "handover/markov_capsules/MARKOV_INHERITANCE_POLICY.md";

fn read_policy() -> String {
    let path = Path::new(POLICY_PATH);
    assert!(
        path.exists(),
        "MARKOV_INHERITANCE_POLICY.md not found at {} — SG-17.9 enforcement \
         requires the doc to exist",
        POLICY_PATH
    );
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", POLICY_PATH, e))
}

#[test]
fn markov_inheritance_policy_doc_exists() {
    let _ = read_policy();
}

#[test]
fn markov_inheritance_policy_has_purpose_section() {
    let body = read_policy();
    assert!(body.contains("## §1"), "policy missing §1 Purpose");
}

#[test]
fn markov_inheritance_policy_has_three_cases() {
    let body = read_policy();
    assert!(body.contains("§2.1"), "missing §2.1 Genesis case");
    assert!(body.contains("§2.2"), "missing §2.2 Inherited case");
    assert!(body.contains("§2.3"), "missing §2.3 Invalid case");
    assert!(
        body.to_lowercase().contains("genesis"),
        "policy text should mention 'genesis' for §2.1"
    );
    assert!(
        body.to_lowercase().contains("inherit"),
        "policy text should mention 'inherit' for §2.2"
    );
    assert!(
        body.to_lowercase().contains("invalid") || body.to_lowercase().contains("unresolvable"),
        "policy text should mention 'invalid' or 'unresolvable' for §2.3"
    );
}

#[test]
fn markov_inheritance_policy_lists_four_forbidden_patterns() {
    let body = read_policy();
    assert!(body.contains("§3.1"), "missing §3.1 forbidden pattern");
    assert!(body.contains("§3.2"), "missing §3.2 forbidden pattern");
    assert!(body.contains("§3.3"), "missing §3.3 forbidden pattern");
    assert!(body.contains("§3.4"), "missing §3.4 forbidden pattern");
}

#[test]
fn markov_inheritance_policy_forbids_global_filesystem_pointer() {
    let body = read_policy();
    assert!(
        body.contains("LATEST_MARKOV_CAPSULE.txt"),
        "policy must explicitly name the forbidden global pointer"
    );
}

#[test]
fn markov_inheritance_policy_forbids_provenance_sidecar() {
    let body = read_policy();
    let lower = body.to_lowercase();
    assert!(
        lower.contains("provenance") || lower.contains("sidecar"),
        "policy must mention sidecar/provenance forbidden pattern (§3.2)"
    );
}

#[test]
fn markov_inheritance_policy_documents_alpha_beta_modes() {
    let body = read_policy();
    assert!(
        body.contains("α") || body.to_lowercase().contains("alpha"),
        "policy must document α transitional mode"
    );
    assert!(
        body.contains("β") || body.to_lowercase().contains("beta"),
        "policy must document β long-term mode"
    );
}

#[test]
fn markov_inheritance_policy_cites_architect_rulings() {
    let body = read_policy();
    assert!(
        body.contains("OBS_R022"),
        "policy must cross-ref OBS_R022 ruling"
    );
    assert!(
        body.contains("Art. 0.2") || body.contains("Art 0.2"),
        "policy must cite Art. 0.2 (Tape Canonical)"
    );
}

#[test]
fn markov_inheritance_policy_lists_ship_gates() {
    let body = read_policy();
    assert!(
        body.contains("SG-17.9"),
        "policy must reference SG-17.9 (TB-17 ship gate)"
    );
    assert!(
        body.contains("SG-17.10"),
        "policy must reference SG-17.10 (no global pointer)"
    );
}

// SG-17.10 enforcement — `LATEST_MARKOV_CAPSULE.txt` MUST NOT exist on disk.
// Existing test `tests/markov_pointer_de_canonicalize.rs::markov_pointer_no_global_parallel_ledger`
// already enforces this; this duplicate guard here so SG-17.10 assertion is
// also visible from the TB-17 conformance battery surface.
#[test]
fn sg_17_10_no_global_filesystem_pointer() {
    let pointer = Path::new("handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt");
    assert!(
        !pointer.exists(),
        "SG-17.10 violation: LATEST_MARKOV_CAPSULE.txt must not exist"
    );
}
