//! GA-6b — H-HET-2 ATTEMPT-LEVEL decision_source completeness.
//!
//! ## HONESTY HEADER (mandatory per task brief)
//!
//! `decision_source` is currently a BIN-only field on `AttemptNode` in
//! `src/bin/lean_market_agent.rs`. It is NOT tape-canonical (Art-0.2 gap): it
//! lives only in the in-process `AttemptNode` struct and the JSON manifest
//! emitted to stdout/file — it is NOT stored in any CAS record, ProposalTelemetry,
//! or ChainTape entry that a verifier could reconstruct.
//!
//! **This gate (GA-6b) is therefore a LOGIC/EXCLUSION WITNESS ONLY.**
//! It proves that IF decision_source is populated correctly AND the exclusion
//! predicate is applied, then forced-solve attempts (parse_fallback, llm_error,
//! forced_solve) are correctly excluded from the primary solve/coverage/PPUT
//! metrics.  It does NOT close the Art-0.2 gap.
//!
//! **REQUIRED before any paid confirmatory run (Class-4 blocked):**
//! Promoting `decision_source` onto a tape-canonical record
//! (ProposalTelemetry/AttemptTelemetry schema v-next) requires:
//!   - schema version bump
//!   - legacy-fallback decoder (§8-gated, Class 3)
//!   - replay-equivalence proof
//!   - Veto-AI PASS
//! Those are Class-4 trust-root-pinned files. **Do NOT make that schema change
//! in this gate.** Flag it here; a separate Class-4 atom is REQUIRED.
//!
//! ## What this gate protects
//!
//! Primary metrics (solve rate, coverage, PPUT) must exclude attempts whose
//! `decision_source` is `parse_fallback`, `llm_error`, or `forced_solve`.
//! Counting a forced-solve as a real agent choice inflates the treatment's
//! apparent solve rate, undermining the economic claim.
//!
//! ## Authority
//!
//! `handover/tracer_bullets/H_HET_2_PHASE2_GATE_DESIGN_2026-06-16.md` (GA-6b);
//! `src/bin/lean_market_agent.rs` lines ~295/329/1176–1210/2247.
//! Art 0.2 (tape-canonical); §4 (no untraced attempt in primary metric).
//!
//! ## FAILABLE tests
//!
//! - `none_decision_source_counted_in_primary_is_red`: an attempt with
//!   `decision_source = None` counted in primary → must fail the gate.
//! - `parse_fallback_counted_in_primary_is_red`: a `parse_fallback` attempt
//!   counted in primary → must fail the gate.

/// Minimal test-local fixture mirroring AttemptNode's relevant fields.
/// (AttemptNode is in a bin crate and is not importable by integration tests.)
#[derive(Debug, Clone)]
struct AttemptRecord {
    /// Mirrors `AttemptNode.action_source`.  `None` for non-autonomous policies;
    /// `Some("agent")` for real agent choice; `Some("parse_fallback")` /
    /// `Some("llm_error")` / `Some("forced_solve")` for harness-forced actions.
    decision_source: Option<&'static str>,
    /// Whether this attempt contributes to the primary solve/coverage/PPUT metric.
    counted_in_primary: bool,
    /// Whether this attempt produced a verified proof.
    solved: bool,
}

/// The set of decision_source values that MUST be excluded from primary metrics.
const FORCED_SOLVE_SOURCES: &[&str] = &["parse_fallback", "llm_error", "forced_solve"];

/// Returns true iff an attempt is properly excluded from primary metrics.
/// An attempt passes the exclusion predicate when:
///   (a) it has a populated (Some, non-empty) decision_source, AND
///   (b) if its decision_source is in FORCED_SOLVE_SOURCES, it is NOT counted in primary.
fn exclusion_predicate_ok(r: &AttemptRecord) -> bool {
    match r.decision_source {
        None => false, // missing decision_source is always a violation
        Some(s) if s.is_empty() => false, // empty string is treated as missing
        Some(s) => {
            if FORCED_SOLVE_SOURCES.contains(&s) {
                // forced-solve: must NOT be in primary
                !r.counted_in_primary
            } else {
                // genuine agent choice: no constraint from this gate
                true
            }
        }
    }
}

// ─── HAPPY-PATH TESTS ────────────────────────────────────────────────────────

#[test]
fn all_attempts_have_populated_decision_source() {
    let tape = vec![
        AttemptRecord { decision_source: Some("agent"), counted_in_primary: true,  solved: true  },
        AttemptRecord { decision_source: Some("agent"), counted_in_primary: true,  solved: false },
        AttemptRecord { decision_source: Some("parse_fallback"), counted_in_primary: false, solved: false },
        AttemptRecord { decision_source: Some("llm_error"),      counted_in_primary: false, solved: false },
    ];
    for (i, r) in tape.iter().enumerate() {
        let populated = r.decision_source.map(|s| !s.is_empty()).unwrap_or(false);
        assert!(
            populated,
            "attempt[{}] has missing or empty decision_source — every attempt must \
             carry a populated decision_source so the metric-exclusion predicate \
             can be applied (Art 0.2 logic gap flagged in GA-6b header)",
            i
        );
    }
}

#[test]
fn forced_solve_attempts_excluded_from_primary() {
    let tape = vec![
        AttemptRecord { decision_source: Some("agent"),          counted_in_primary: true,  solved: true  },
        AttemptRecord { decision_source: Some("parse_fallback"), counted_in_primary: false, solved: false },
        AttemptRecord { decision_source: Some("llm_error"),      counted_in_primary: false, solved: false },
        AttemptRecord { decision_source: Some("forced_solve"),   counted_in_primary: false, solved: false },
    ];
    let violations: Vec<usize> = tape
        .iter()
        .enumerate()
        .filter(|(_, r)| !exclusion_predicate_ok(r))
        .map(|(i, _)| i)
        .collect();
    assert!(
        violations.is_empty(),
        "GA-6b exclusion predicate failed on attempt indices {:?}: forced-solve \
         attempts (parse_fallback/llm_error/forced_solve) must NOT be counted in \
         the primary solve/coverage/PPUT metric",
        violations
    );
}

#[test]
fn agent_attempts_may_be_in_primary() {
    // An "agent" attempt that is counted in primary must pass the gate.
    let r = AttemptRecord {
        decision_source: Some("agent"),
        counted_in_primary: true,
        solved: true,
    };
    assert!(
        exclusion_predicate_ok(&r),
        "gate incorrectly rejects a genuine agent attempt counted in primary"
    );
}

// ─── FABILITY TESTS (these must catch the named anti-patterns) ───────────────

/// FAILABLE (a): an attempt with `decision_source = None` that is counted in
/// primary must be REJECTED by the exclusion predicate.
#[test]
fn none_decision_source_counted_in_primary_is_red() {
    let bad = AttemptRecord {
        decision_source: None,
        counted_in_primary: true,
        solved: false,
    };
    assert!(
        !exclusion_predicate_ok(&bad),
        "GA-6b FAILABLE TEST FAILED: an attempt with decision_source=None \
         counted in primary was ACCEPTED — this would silently let an untraced \
         attempt inflate the solve-rate metric (Art 0.2 violation)"
    );
}

/// FAILABLE (b): a `parse_fallback` attempt that is counted in primary must be
/// REJECTED by the exclusion predicate.
#[test]
fn parse_fallback_counted_in_primary_is_red() {
    let bad = AttemptRecord {
        decision_source: Some("parse_fallback"),
        counted_in_primary: true,
        solved: false,
    };
    assert!(
        !exclusion_predicate_ok(&bad),
        "GA-6b FAILABLE TEST FAILED: a parse_fallback attempt counted in primary \
         was ACCEPTED — forced-solve attempts must be excluded from the primary \
         solve/coverage/PPUT metric"
    );
}

/// FAILABLE (extra): an `llm_error` attempt counted in primary must also be rejected.
#[test]
fn llm_error_counted_in_primary_is_red() {
    let bad = AttemptRecord {
        decision_source: Some("llm_error"),
        counted_in_primary: true,
        solved: false,
    };
    assert!(
        !exclusion_predicate_ok(&bad),
        "GA-6b FAILABLE TEST FAILED: an llm_error attempt counted in primary \
         was ACCEPTED — forced-solve attempts must be excluded from the primary \
         solve/coverage/PPUT metric"
    );
}
