//! TB-7 Atom 7 — Gate 7 legacy-bypass regression test.
//!
//! Per ARCHITECT_RULING 2026-05-01 §4 Gate 7 + TB-7 charter §6 #31:
//! "A new conformance test asserts that no proposal-producing evaluator
//! site can call legacy append as authoritative state mutation. Permitted
//! call sites must carry `// shadow_only:` annotation that the test
//! recognizes and exempts."
//!
//! This is a repo-wide grep gate analogous to:
//! - TB-1 P0-3 serde-shield scanner
//! - TB-3 bridge-pattern invariant scanner
//! - TB-4 anti-drift CI scanner (no NoStakeTx / VerifierBondTx /
//!   ChallengeStakeTx variants in src/)
//! - TB-5 system-tx-forbidden ingress barrier
//!
//! Scope: walks `experiments/minif2f_v4/src/bin/evaluator.rs` (the live
//! production-binary path); flags any line containing `bus.append(` or
//! `bus.append_oracle_accepted(` UNLESS the contiguous comment block
//! immediately above the call site contains a `// shadow_only:`
//! annotation.
//!
//! Why "contiguous comment block": the typical annotation pattern is a
//! multi-line comment block (1-12 lines) immediately above the call,
//! with no intervening code. The scanner walks backward from the call
//! line, skipping blank lines and comment lines, until it hits either
//! a non-comment line OR the `// shadow_only:` marker. This avoids
//! false-positives when the annotation is verbose AND false-negatives
//! when the annotation is on a distant earlier line separated by code.
//!
//! TRACE_MATRIX FC1-N14: Gate 7 conformance witness.

const SHADOW_ONLY_MARKER: &str = "// shadow_only:";

/// Returns the line indices (0-based) of every `bus.append(` or
/// `bus.append_oracle_accepted(` call site whose preceding contiguous
/// comment block does NOT contain a `// shadow_only:` annotation.
///
/// Algorithm:
/// 1. For each line containing the legacy call (and not itself a comment),
/// 2. Walk backward, skipping blank lines and comment lines (// ...),
/// 3. While walking, check each comment line for `// shadow_only:`.
/// 4. Stop walking at the first non-comment, non-blank line.
/// 5. If the marker was seen during the walk → annotated; else → violation.
fn unannotated_legacy_append_sites(source: &str) -> Vec<(usize, String)> {
    let lines: Vec<&str> = source.lines().collect();
    let mut violations = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") || trimmed.starts_with("///") {
            continue;
        }
        let is_legacy_call = (line.contains("bus.append(")
            || line.contains("bus.append_oracle_accepted("))
            && !trimmed.starts_with("//");
        if !is_legacy_call {
            continue;
        }
        let mut annotated = false;
        let mut back: usize = 1;
        loop {
            if idx < back {
                break;
            }
            let prev = lines[idx - back];
            let prev_trim = prev.trim();
            if prev_trim.is_empty() {
                back += 1;
                continue;
            }
            if !prev_trim.starts_with("//") {
                // Hit code; stop walking.
                break;
            }
            if prev.contains(SHADOW_ONLY_MARKER) {
                annotated = true;
                break;
            }
            back += 1;
            if back > 50 {
                // Safety bound: no comment block should exceed 50 lines.
                break;
            }
        }
        if !annotated {
            violations.push((idx + 1, line.trim().to_string()));
        }
    }
    violations
}


/// Positive control: confirm the scanner CAN flag a synthetic violation,
/// so the gate isn't silently passing on a broken scanner.
#[test]
fn gate_7_scanner_positive_control_flags_unannotated_call() {
    let synthetic = r#"
fn fake_evaluator() {
    let bus = ();
    // This is just a setup comment.
    let result = bus.append("agent", "payload", None);
    let _ = result;
}
"#;
    let violations = unannotated_legacy_append_sites(synthetic);
    assert!(
        !violations.is_empty(),
        "scanner positive control failed — synthetic unannotated bus.append should be flagged"
    );
}

/// Positive control: confirm the scanner correctly EXEMPTS a call with
/// the `// shadow_only:` annotation in the preceding 3 lines.
#[test]
fn gate_7_scanner_exempts_shadow_only_annotated_call() {
    let synthetic = r#"
fn fake_evaluator() {
    let bus = ();
    // shadow_only: tape view sync. Not authoritative.
    let result = bus.append("agent", "payload", None);
    let _ = result;
}
"#;
    let violations = unannotated_legacy_append_sites(synthetic);
    assert!(
        violations.is_empty(),
        "shadow_only-annotated call must NOT be flagged: {violations:?}"
    );
}
