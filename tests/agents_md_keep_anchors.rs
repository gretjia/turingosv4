//! Phase 4 (harness platform-agnostic unification, 2026-05-29) — KEEP-anchor
//! survival gate for the slimmed `AGENTS.md`.
//!
//! `AGENTS.md` is read by every platform every session, so Phase 4 slimmed it
//! by the irreducibility rule: remove a line only if removing it loses no
//! governance. The danger of that operation is *silent* governance loss — a
//! dedup or deletion that also drops a load-bearing rule. (Phase 2 already did
//! this once: deleting the `turingos_dev` §10 silently removed the only
//! `fail-closed` mention; this gate exists partly so that never recurs unseen.)
//!
//! This test makes "zero loss of load-bearing governance" FALSIFIABLE: it
//! asserts each KEEP-list anchor is still literally present in `AGENTS.md`. If
//! a future edit removes or renames any anchor, this test goes RED — exactly
//! the §7 "gate tests must be able to fail" property. `keep_anchor_checker_can_fail`
//! proves the matcher is not vacuously green.
//!
//! These are CONTENT anchors (does the contract still say the rule), distinct
//! from `cli_entry_files_redirect_to_agents.rs`, which checks the thin CLI
//! files point here without narrowing. Anchors are matched as substrings, not
//! exact lines, so benign rewording is allowed; deletion is not.

use std::fs;

/// Each KEEP-list anchor: a human label plus the substrings that ALL must be
/// present in `AGENTS.md` for the anchor to count as surviving. Substrings are
/// chosen to be load-bearing tokens unlikely to change under benign editing.
const KEEP_ANCHORS: &[(&str, &[&str])] = &[
    ("PR-only workflow (§14a)", &["PR-only"]),
    (
        "Restricted surfaces §6 (all six src/ surfaces)",
        &[
            "## 6. Restricted Surfaces",
            "src/kernel.rs",
            "src/bus.rs",
            "src/sdk/tools/wallet.rs",
            "src/state/sequencer.rs",
            "src/state/typed_tx.rs",
            "src/bottom_white/cas/schema.rs",
        ],
    ),
    ("Integer-only money math", &["integer math"]),
    ("Obligation ledger", &["OBLIGATIONS.md"]),
    ("No retroactive evidence rewrite", &["retroactively rewrite"]),
    (
        "Class-4 per-atom §8 ratification",
        &["Class 4 requires explicit per-atom"],
    ),
    ("Tape-first evidence", &["tape-first"]),
    ("Fail-closed admission/gates", &["fail-closed"]),
    ("FC-trace (state touched FC nodes)", &["FC nodes"]),
    (
        "Karpathy §13 skill references",
        &["KARPATHY_ARCHITECT.md", "KARPATHY_SIMPLE_CODE.md"],
    ),
    (
        "Karpathy cold-start read-order slot #6",
        &["6. Key Coding Principles"],
    ),
    (
        "§14 structural anti-pattern greps (type families + trait single-impl)",
        &[
            "Manager",
            "Factory",
            "Engine",
            "Platform",
            "Framework",
            "非-idiomatic impl",
        ],
    ),
];

/// Read `AGENTS.md` from the package root (where `cargo test` runs). Mirrors the
/// proven relative-path read in `cli_entry_files_redirect_to_agents.rs`.
fn agents_md() -> String {
    fs::read_to_string("AGENTS.md").expect("AGENTS.md readable from package root")
}

/// Return the labels of anchors whose required substrings are NOT all present.
fn missing_anchors(text: &str) -> Vec<String> {
    let mut missing = Vec::new();
    for (label, needles) in KEEP_ANCHORS {
        let absent: Vec<&str> = needles
            .iter()
            .copied()
            .filter(|n| !text.contains(n))
            .collect();
        if !absent.is_empty() {
            missing.push(format!("{label} — missing substrings: {absent:?}"));
        }
    }
    missing
}

#[test]
fn agents_md_retains_all_keep_list_anchors() {
    let text = agents_md();
    let missing = missing_anchors(&text);
    assert!(
        missing.is_empty(),
        "AGENTS.md slim dropped load-bearing governance anchor(s):\n{}",
        missing.join("\n")
    );
}

/// Gate-can-fail control (§7): feed the checker a document that is missing an
/// anchor and prove it reports the miss. Without this, an inverted `contains`
/// or a skipped loop would make `agents_md_retains_all_keep_list_anchors`
/// vacuously green.
#[test]
fn keep_anchor_checker_reports_a_real_miss() {
    // A document that satisfies no anchor at all → every anchor must be missing.
    let empty_doc = "this document contains none of the governance anchors\n";
    let missing = missing_anchors(empty_doc);
    assert_eq!(
        missing.len(),
        KEEP_ANCHORS.len(),
        "checker must flag every anchor as missing in a document that has none; got: {missing:?}"
    );
}
