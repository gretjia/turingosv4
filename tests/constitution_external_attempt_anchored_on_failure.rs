//! LIVE CONSTITUTION GATE (conformance #2) — evidence-cas-anchor witness:
//! every external-LLM runner must anchor its completed attempt in CAS BEFORE any
//! parse/guard abort.
//!
//! ── WHAT THIS GATE ENFORCES (the always-anchor invariant) ──────────────────
//! Once an external LLM call completes, the run's tokens are spent and the
//! response IS the externalized attempt. A parse/guard failure must NOT throw
//! the attempt away unrecorded: the attempt-evidence capsule must be written to
//! CAS FIRST (capturing the parse error in a field, swebench-style), and only
//! THEN may the runner abort. Otherwise the failure branch leaves zero CAS
//! object, zero L4 WorkTx, and zero L4.E — the spent attempt is unreconstructable
//! from tape (FC1 / Art. "tape-first" — no tape, no run).
//!
//! The invariant is structural and completeness-shaped: it ENUMERATES every
//! `src/bin/*_current_kernel.rs` runner that makes an external LLM call (the
//! response-hash anchor `*_response_sha256 = sha256_hex(`), and for EACH asserts
//! that the span from that completion anchor to the first subsequent
//! `put_json(` (the attempt-evidence write) contains NO `parse_…(…)?`
//! error-propagating call and NO `return Err(`. swebench
//! (`parse_patch_claim` without `?`, capturing `parse_error`, always writing the
//! claim capsule) is the passing reference; the market binary (formerly
//! `parse_decision(..)?` + direction-mismatch `return Err` before the first
//! `put_json`) was the RED site this gate was written to catch. A future
//! parallel runner that early-returns on a parse/guard failure before anchoring
//! turns this gate RED. It is a SOURCE-STRUCTURAL witness (same family as
//! `tests/constitution_single_admission_contract.rs`): it greps the canonical
//! runner sources, so it cannot be satisfied by a vacuous `assert!(true)`.
//!
//! ── TRIPLE-COUPLING ──────────────────────────────────────────────────────
//! Registered in `scripts/constitution_gates.manifest.toml`
//! (`constitution_external_attempt_anchored_on_failure`) and referenced in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`. Discovered by the flat
//! `ls tests/constitution_*.rs` glob in `scripts/run_constitution_gates.sh`.

use std::fs;
use std::path::PathBuf;

/// The externalized-LLM-completion anchor: right after the call returns, every
/// runner hashes the response. This is the point past which tokens are spent.
const COMPLETION_ANCHOR: &str = "_response_sha256 = sha256_hex(";
/// The attempt-evidence write that must close the span.
const PUT_JSON: &str = "put_json(";

/// Enumerate every `src/bin/*_current_kernel.rs` runner that makes an external
/// LLM call (identified by the response-hash completion anchor). This is the
/// class S over which the always-anchor invariant must hold.
fn external_llm_runners() -> Vec<(PathBuf, String)> {
    let mut out = Vec::new();
    let dir = std::path::Path::new("src/bin");
    for entry in fs::read_dir(dir).expect("src/bin readable") {
        let path = entry.expect("dir entry").path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !name.ends_with("_current_kernel.rs") {
            continue;
        }
        let src = fs::read_to_string(&path).expect("runner source readable");
        if src.contains(COMPLETION_ANCHOR) {
            out.push((path, src));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// The class is non-empty and includes the named passing reference (swebench)
/// and the formerly-RED site (market). If either disappears, the gate must be
/// re-pointed, not silently satisfied.
#[test]
fn external_llm_runner_class_is_populated_and_includes_reference_sites() {
    let runners = external_llm_runners();
    assert!(
        runners.len() >= 2,
        "evidence-cas-anchor gate: fewer than 2 external-LLM runners found via \
         the completion anchor `{COMPLETION_ANCHOR}` in src/bin/*_current_kernel.rs. \
         If the anchor pattern changed, re-point this gate — do NOT let the \
         completeness witness collapse to zero sites."
    );
    let names: Vec<String> = runners
        .iter()
        .map(|(p, _)| p.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    assert!(
        names
            .iter()
            .any(|n| n == "swebench_live_coding_repair_current_kernel.rs"),
        "evidence-cas-anchor gate: swebench runner (the passing reference) not \
         in the enumerated class. Found: {names:?}"
    );
    assert!(
        names
            .iter()
            .any(|n| n == "market_external_agent_current_kernel.rs"),
        "evidence-cas-anchor gate: market runner (the confirmed #2 site) not in \
         the enumerated class. Found: {names:?}"
    );
}

/// ENUMERATE-ALL-SITES completeness: for every external-LLM runner, the span
/// from the completion anchor to the FIRST subsequent `put_json(` must contain
/// no error-propagating parse call (`parse_…(…)?`) and no `return Err(`.
#[test]
fn no_parse_or_return_err_between_llm_completion_and_first_evidence_put_json() {
    let runners = external_llm_runners();
    assert!(
        !runners.is_empty(),
        "evidence-cas-anchor gate: zero external-LLM runners to scan."
    );

    for (path, src) in &runners {
        let name = path.file_name().unwrap().to_string_lossy();

        // Each runner may complete more than one LLM call (e.g. a multi-trade
        // loop). Check EVERY completion anchor occurrence.
        let mut from = 0usize;
        let mut anchors_checked = 0usize;
        while let Some(rel) = src[from..].find(COMPLETION_ANCHOR) {
            let anchor_at = from + rel;
            from = anchor_at + COMPLETION_ANCHOR.len();

            // The span ends at the first put_json AFTER the anchor (the
            // attempt-evidence write). If there is none after this anchor, the
            // anchor belongs to a later call already covered by a put_json
            // earlier in iteration; skip (no span to police).
            let put_rel = match src[anchor_at..].find(PUT_JSON) {
                Some(r) => r,
                None => continue,
            };
            let span = &src[anchor_at..anchor_at + put_rel];

            // Strip `//`-to-EOL line comments first: the always-anchor fix's own
            // doc-comments legitimately MENTION the old `parse_decision(..)?`
            // shape, and those mentions must not trip the structural scan.
            let code = strip_line_comments(span);

            // Forbidden A: an error-propagating parse call `parse…(…)?`.
            assert!(
                !span_has_error_propagating_parse(&code),
                "evidence-cas-anchor violation (conformance #2): in src/bin/{name} \
                 an error-propagating `parse_…(…)?` appears BETWEEN the completed \
                 LLM call and the first attempt-evidence `put_json(`. The spent \
                 attempt would be lost on the parse-failure branch. Capture the \
                 parse error into a capsule field and ALWAYS `put_json` the claim \
                 capsule first (swebench `parse_patch_claim` is the reference), \
                 then abort AFTER anchoring. Offending span:\n{code}"
            );

            // Forbidden B: a `return Err(` (direction/guard abort) before the
            // evidence write. Scanned on comment-stripped code so the fix's own
            // explanatory comments do not trip it.
            assert!(
                !code.contains("return Err("),
                "evidence-cas-anchor violation (conformance #2): in src/bin/{name} \
                 a `return Err(` appears BETWEEN the completed LLM call and the \
                 first attempt-evidence `put_json(`. The completed attempt must be \
                 anchored in CAS BEFORE any guard abort. Move the guard's \
                 `return Err` to AFTER the `put_json` and record the failure in a \
                 capsule field. Offending span:\n{code}"
            );

            anchors_checked += 1;
        }

        assert!(
            anchors_checked >= 1,
            "evidence-cas-anchor gate: src/bin/{name} matched the runner class \
             but yielded zero policed spans. If its evidence-write shape changed, \
             re-point this gate."
        );
    }
}

/// Drop `//`-to-end-of-line comments from a code span so doc/explanatory
/// comments that legitimately MENTION the forbidden `parse…(..)?` shape do not
/// trip the structural scan. (Block comments are not used in the policed spans;
/// if they ever are, extend this.)
fn strip_line_comments(span: &str) -> String {
    span.lines()
        .map(|line| match line.find("//") {
            Some(i) => &line[..i],
            None => line,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Detect an error-propagating parse CALL inside a (comment-stripped) span: an
/// identifier beginning with `parse` that starts a real call and whose call
/// expression is terminated by the `?` operator before the statement's `;`. The
/// always-anchor pattern binds the parse Result (`let x = parse_…(…);` WITHOUT
/// `?`), so a `parse…(…)?` is exactly the short-circuit the invariant forbids.
fn span_has_error_propagating_parse(code: &str) -> bool {
    let bytes = code.as_bytes();
    let mut from = 0usize;
    while let Some(rel) = code[from..].find("parse") {
        let at = from + rel;
        from = at + 5;
        // Require `parse` to START an identifier (preceded by a non-identifier
        // char), so it is a call name, not a substring of another word.
        if at > 0 {
            let prev = bytes[at - 1];
            if prev == b'_' || prev.is_ascii_alphanumeric() {
                continue;
            }
        }
        // The identifier must be a call: somewhere after `parse…` an open paren.
        // Scan the statement (up to `;`) and require a `)?` error-propagation
        // marker to count it as a short-circuiting parse.
        let stmt_end = code[at..].find(';').map(|e| at + e).unwrap_or(code.len());
        let stmt = &code[at..stmt_end];
        if stmt.contains('(') && stmt.contains(")?") {
            return true;
        }
    }
    false
}
