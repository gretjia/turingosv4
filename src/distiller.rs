//! TRACE_MATRIX FC1a-rtool_input + FC3-replay: TDMA-Bounded distiller.
//!
//! The distiller has TWO jobs (directive §6):
//!
//!  1. `deterministic_trace_slicer(raw_stderr, header, B_DISTILL_IN) -> TraceView`
//!     A PURE pre-LLM gate. Extracts structured features from raw stderr
//!     (reject_class, failed_predicate, stack frames, touched paths, stderr tail)
//!     into a hard-bounded `TraceView` JSON. Raw stderr is NEVER returned —
//!     only its sha256. The kernel cannot leak raw stderr into a prompt because
//!     the LLM distiller call signature takes `TraceView`, NOT `&str`
//!     (KILL-tdma-1 enforced at the type system).
//!
//!  2. `compress_belief_state(prev, trace_view, evidence_hash, scope, B_D) -> RetryBeliefState`
//!     A deterministic BBS compressor. Computes information_gain via Jaccard
//!     delta + bonuses, updates zero_gain_streak per directive §6.2, evicts
//!     lowest-priority constraints until under budget. Returns a tape-canonical
//!     `RetryBeliefState` (kind=RetryBeliefState payload). NEVER mutates a
//!     sidecar (KILL-tdma-2).
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;

use crate::ledger::{
    AttemptScope, EvictedConstraint, EvidencePointer, FailureSignature, RetryBeliefState,
    RetryConstraint,
};
use crate::state_update::StateUpdate;
use crate::token_budget::EPSILON_GAIN;
use crate::tokenizer::Tokenizer;

// ── TraceView ───────────────────────────────────────────────────

/// Slim, deterministic view of a raw stderr trace (directive §6.1).
/// TRACE_MATRIX FC1a-rtool_input + KILL-tdma-1: Holds only structured fields
/// + sha256 of raw stderr; the raw bytes are NEVER carried forward into the
/// distiller LLM call or any active prompt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceView {
    pub schema_version: String,
    pub reject_class: String,
    pub failed_predicate: String,
    pub top_frames: Vec<String>,
    pub bottom_frames: Vec<String>,
    pub touched_paths: Vec<String>,
    pub stderr_tail: String,
    pub raw_stderr_sha256: String,
}

// ── Deterministic helpers ───────────────────────────────────────

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

/// Extract `<file>:<line>` or `at <file>:<line>` style stack-frame mentions.
/// TRACE_MATRIX FC1a-rtool_input: Heuristic frame extraction; deterministic.
pub fn extract_stack_frames(raw: &str) -> Vec<String> {
    let mut frames = Vec::new();
    for line in raw.lines() {
        let l = line.trim();
        if l.starts_with("at ") || l.contains(".rs:") || l.contains(".py:") || l.contains(".lean:") {
            frames.push(l.to_string());
        }
    }
    frames
}

/// Extract `path/to/file.ext` style references from a trace.
/// TRACE_MATRIX FC1a-rtool_input: Deterministic path extraction.
pub fn extract_file_paths(raw: &str) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut paths = Vec::new();
    for chunk in raw.split(|c: char| c.is_whitespace() || c == '(' || c == ')' || c == ',') {
        let trimmed = chunk.trim_end_matches(|c: char| c == '.' || c == ':' || c == ';');
        if trimmed.contains('/') && trimmed.contains('.') && trimmed.len() < 200 {
            if seen.insert(trimmed.to_string()) {
                paths.push(trimmed.to_string());
            }
        }
    }
    paths
}

/// Last `n` non-empty lines of stderr.
/// TRACE_MATRIX FC1a-rtool_input: Deterministic stderr tail extraction.
pub fn take_last_lines(raw: &str, n: usize) -> String {
    let lines: Vec<&str> = raw.lines().filter(|l| !l.trim().is_empty()).collect();
    let start = lines.len().saturating_sub(n);
    lines[start..].join("\n")
}

/// Fallback `reject_class` derivation when the header didn't provide one.
/// TRACE_MATRIX FC1a-rtool_input: Reject-class heuristic (deterministic).
pub fn classify_reject(raw: &str) -> String {
    let lower = raw.to_lowercase();
    if lower.contains("schema") || lower.contains("malformed") {
        "schema-fail".into()
    } else if lower.contains("timeout") {
        "timeout".into()
    } else if lower.contains("permission") {
        "permission-denied".into()
    } else if lower.contains("syntax") || lower.contains("parse") {
        "syntax-error".into()
    } else if lower.contains("not found") || lower.contains("no such") {
        "not-found".into()
    } else {
        "unclassified".into()
    }
}

/// Fallback failed-predicate from stderr when header didn't say.
/// TRACE_MATRIX FC1a-rtool_input: Predicate extraction heuristic.
pub fn extract_first_failed_predicate(raw: &str) -> String {
    for line in raw.lines() {
        let l = line.trim();
        if l.starts_with("assertion failed") || l.starts_with("AssertionError") {
            return l.to_string();
        }
        if l.to_lowercase().contains("predicate") {
            return l.to_string();
        }
    }
    "unknown-predicate".into()
}

// ── deterministic_trace_slicer ──────────────────────────────────

/// Hard pre-LLM budget gate (directive §6.1).
/// TRACE_MATRIX FC1a-rtool_input + KILL-tdma-1: Bounded, deterministic, pure
/// function. The LLM distiller can only consume what this function returns.
pub fn deterministic_trace_slicer(
    raw_stderr: &str,
    header: &StateUpdate,
    budget: usize,
    tokenizer: &Tokenizer,
) -> TraceView {
    let raw_stderr_sha256 = sha256_hex(raw_stderr.as_bytes());

    let reject_class = header
        .reject_class
        .clone()
        .unwrap_or_else(|| classify_reject(raw_stderr));
    let failed_predicate = header
        .failed_predicate
        .clone()
        .unwrap_or_else(|| extract_first_failed_predicate(raw_stderr));

    let all_frames = extract_stack_frames(raw_stderr);
    let top_frames: Vec<String> = all_frames.iter().take(5).cloned().collect();
    let bottom_frames: Vec<String> = all_frames.iter().rev().take(5).cloned().collect();
    // Cap touched_paths upfront to keep the trim loop bounded on huge stderr.
    let touched_paths: Vec<String> = extract_file_paths(raw_stderr).into_iter().take(20).collect();
    // Cap stderr tail aggressively: 40 lines, then 4 KB max to prevent the
    // trim loop from halving a multi-megabyte string repeatedly.
    let mut stderr_tail = take_last_lines(raw_stderr, 40);
    if stderr_tail.len() > 4096 {
        let cut = stderr_tail.len() - 4096;
        stderr_tail = stderr_tail.split_off(cut);
    }

    let mut view = TraceView {
        schema_version: "tdma-trace-view/v1".into(),
        reject_class,
        failed_predicate,
        top_frames,
        bottom_frames,
        touched_paths,
        stderr_tail,
        raw_stderr_sha256,
    };

    // Enforce budget by progressively trimming low-priority fields.
    while tokenizer.count_json(&view) > budget {
        if !view.touched_paths.is_empty() {
            view.touched_paths.pop();
            continue;
        }
        if view.top_frames.len() > 1 {
            view.top_frames.pop();
            continue;
        }
        if view.bottom_frames.len() > 1 {
            view.bottom_frames.pop();
            continue;
        }
        if !view.stderr_tail.is_empty() {
            let new_len = view.stderr_tail.len() / 2;
            view.stderr_tail = view.stderr_tail.chars().take(new_len.max(1)).collect();
            if view.stderr_tail.chars().count() <= 8 {
                view.stderr_tail.clear();
            }
            continue;
        }
        if !view.failed_predicate.is_empty() && view.failed_predicate.len() > 32 {
            view.failed_predicate = view
                .failed_predicate
                .chars()
                .take(view.failed_predicate.len() / 2)
                .collect();
            continue;
        }
        // Already at minimal shape — break.
        break;
    }

    view
}

// ── information_gain + compress_belief_state ────────────────────

/// Jaccard delta between two sets of rule strings + bonus terms (directive §6.2).
/// TRACE_MATRIX FC1a-rtool_input: Deterministic information-gain proxy.
pub fn information_gain(
    prev: Option<&RetryBeliefState>,
    new_signature: &FailureSignature,
    new_predicate: &str,
    new_touched_paths: &[String],
    new_rules: &[String],
) -> f64 {
    let prev_rules: HashSet<&str> = prev
        .map(|p| p.constraints.iter().map(|c| c.rule.as_str()).collect())
        .unwrap_or_default();
    let new_rules_set: HashSet<&str> = new_rules.iter().map(|s| s.as_str()).collect();
    let union: HashSet<&&str> = prev_rules.union(&new_rules_set).collect();
    let inter: HashSet<&&str> = prev_rules.intersection(&new_rules_set).collect();
    let jaccard_delta = if union.is_empty() {
        0.0
    } else {
        1.0 - (inter.len() as f64 / union.len() as f64)
    };

    let signature_change_bonus = match prev {
        Some(p) if &p.failure_signature == new_signature => 0.0,
        Some(_) => 0.5,
        None => 0.5,
    };

    let new_predicate_bonus = match prev {
        Some(p) if p.failure_signature.failed_predicate == new_predicate => 0.0,
        _ => 0.25,
    };

    // new_touched_path_bonus: BBS does not carry prev touched_paths (not in the
    // directive schema). For the first attempt (prev=None) any non-empty paths
    // earn the bonus once; subsequent attempts under the same signature get 0,
    // because we can't differentiate "new path" from "same path" without the
    // history. The intent of zero_gain detection is "are we learning anything
    // new?", and `signature_change_bonus` + `new_predicate_bonus` already
    // capture the signal we need on subsequent calls.
    let new_paths_bonus = match prev {
        None if !new_touched_paths.is_empty() => 0.25,
        _ => 0.0,
    };

    jaccard_delta + signature_change_bonus + new_predicate_bonus + new_paths_bonus
}

/// Build a minimal fallback BBS when schema-valid extraction fails.
/// TRACE_MATRIX FC1a-rtool_input: Deterministic floor for replay continuity.
pub fn fallback_regex_bbs(
    prev: Option<&RetryBeliefState>,
    trace: &TraceView,
    evidence_hash: &str,
    scope: &AttemptScope,
) -> RetryBeliefState {
    RetryBeliefState {
        schema_version: "tdma-bbs/v1".into(),
        scope: scope.clone(),
        failure_signature: FailureSignature {
            reject_class: trace.reject_class.clone(),
            failed_predicate: trace.failed_predicate.clone(),
            root_cause: trace
                .top_frames
                .first()
                .cloned()
                .unwrap_or_else(|| "unknown".into()),
        },
        constraints: prev
            .map(|p| p.constraints.clone())
            .unwrap_or_default(),
        evidence: EvidencePointer {
            evidence_node_hash: evidence_hash.into(),
            raw_stderr_sha256: trace.raw_stderr_sha256.clone(),
            trace_view_sha256: sha256_hex(
                serde_json::to_string(trace).unwrap_or_default().as_bytes(),
            ),
        },
        zero_gain_streak: 0,
        information_gain: 0.0,
        evicted: prev.map(|p| p.evicted.clone()).unwrap_or_default(),
    }
}

/// Compress a candidate BBS to fit within `budget` tokens by evicting
/// lowest-priority constraints first (directive §6.2).
/// TRACE_MATRIX FC1a-rtool_input: Budget-respecting BBS reduction.
pub fn evict_lowest_priority_rules_until_budget(
    mut bbs: RetryBeliefState,
    budget: usize,
    tokenizer: &Tokenizer,
) -> RetryBeliefState {
    if tokenizer.count_json(&bbs) <= budget {
        return bbs;
    }

    bbs.constraints.sort_by_key(|c| c.priority);
    while !bbs.constraints.is_empty() && tokenizer.count_json(&bbs) > budget {
        let dropped = bbs.constraints.remove(0);
        bbs.evicted.push(EvictedConstraint {
            id: dropped.id,
            priority: dropped.priority,
            reason: "budget".into(),
        });
    }

    bbs
}

/// Default `compress_belief_state` entry-point (directive §6.2).
/// `new_rules` is the candidate ruleset the kernel proposes for this attempt;
/// callers may pass an empty slice and let the fallback path produce a minimal
/// BBS from the TraceView alone.
/// TRACE_MATRIX FC1a-rtool_input + KILL-tdma-2: Deterministic, append-only
/// compression. The function NEVER mutates a sidecar — it produces a fresh
/// `RetryBeliefState` value that the kernel will commit to tape with
/// kind=RetryBeliefState.
pub fn compress_belief_state(
    prev: Option<&RetryBeliefState>,
    trace: &TraceView,
    new_rules: &[RetryConstraint],
    evidence_hash: &str,
    scope: &AttemptScope,
    budget: usize,
    tokenizer: &Tokenizer,
) -> RetryBeliefState {
    // Build the candidate by merging prev constraints + new rules.
    let mut constraints: Vec<RetryConstraint> = prev
        .map(|p| p.constraints.clone())
        .unwrap_or_default();
    for rule in new_rules {
        if !constraints.iter().any(|c| c.id == rule.id) {
            constraints.push(rule.clone());
        }
    }

    let signature = FailureSignature {
        reject_class: trace.reject_class.clone(),
        failed_predicate: trace.failed_predicate.clone(),
        root_cause: trace
            .top_frames
            .first()
            .cloned()
            .unwrap_or_else(|| "unknown".into()),
    };

    let rule_strings: Vec<String> = constraints.iter().map(|c| c.rule.clone()).collect();
    let gain = information_gain(
        prev,
        &signature,
        &trace.failed_predicate,
        &trace.touched_paths,
        &rule_strings,
    );

    let zero_gain_streak = match prev {
        Some(p) if p.failure_signature == signature && gain < EPSILON_GAIN => {
            p.zero_gain_streak + 1
        }
        _ => 0,
    };

    let evidence = EvidencePointer {
        evidence_node_hash: evidence_hash.into(),
        raw_stderr_sha256: trace.raw_stderr_sha256.clone(),
        trace_view_sha256: sha256_hex(
            serde_json::to_string(trace).unwrap_or_default().as_bytes(),
        ),
    };

    let candidate = RetryBeliefState {
        schema_version: "tdma-bbs/v1".into(),
        scope: scope.clone(),
        failure_signature: signature,
        constraints,
        evidence,
        zero_gain_streak,
        information_gain: gain,
        evicted: prev.map(|p| p.evicted.clone()).unwrap_or_default(),
    };

    // Apply budget; if even fallback won't fit, use fallback_regex_bbs as floor.
    let after_budget = evict_lowest_priority_rules_until_budget(candidate, budget, tokenizer);
    if tokenizer.count_json(&after_budget) > budget {
        // Still too big — use minimal fallback.
        fallback_regex_bbs(prev, trace, evidence_hash, scope)
    } else {
        after_budget
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_update::{StateStatus, StateUpdate};
    use crate::token_budget::{B_D, B_DISTILL_IN, ZERO_GAIN_K};

    fn header_with(reject: Option<&str>, predicate: Option<&str>) -> StateUpdate {
        StateUpdate {
            schema_version: "tdma-state-update/v1".into(),
            status: StateStatus::Retry,
            task_id: "t".into(),
            action: "RETRY".into(),
            failed_predicate: predicate.map(|s| s.into()),
            reject_class: reject.map(|s| s.into()),
            next_action_hint: None,
            evidence_hash: None,
        }
    }

    fn scope() -> AttemptScope {
        AttemptScope {
            run_id: "run".into(),
            task_id: "t".into(),
            verified_parent: "H0".into(),
        }
    }

    fn tk() -> Tokenizer {
        Tokenizer::new()
    }

    // ── distiller_in_budget_200k_trace ──────────────────────────

    #[test]
    fn distiller_in_budget_200k_trace() {
        // Build a synthetic stderr >= 50_000 tokens (~200_000 chars) — the
        // directive's "200k trace" name refers to a generously oversized blob;
        // any input large enough to far-exceed B_DISTILL_IN proves the gate.
        let mut raw = String::new();
        for i in 0..30_000 {
            raw.push_str(&format!(
                "at src/foo.rs:{} in fn_{}\n   error context line {} with extra padding bytes\n",
                i, i, i
            ));
        }
        assert!(
            tk().count_text(&raw) > 50_000,
            "synthetic stderr should be oversized (got {})", tk().count_text(&raw)
        );
        let header = header_with(Some("schema-fail"), Some("x.y"));
        let view = deterministic_trace_slicer(&raw, &header, B_DISTILL_IN, &tk());
        let budget_tokens = tk().count_json(&view);
        assert!(
            budget_tokens <= B_DISTILL_IN,
            "trace view must fit B_DISTILL_IN, got {}",
            budget_tokens
        );
        // raw_stderr never returned — only its sha
        let json = serde_json::to_string(&view).unwrap();
        assert!(!json.contains("error context line 4999"));
        assert_eq!(view.raw_stderr_sha256.len(), 64); // sha256 hex
    }

    // ── zero_gain_circuit_breaker ───────────────────────────────

    #[test]
    fn zero_gain_circuit_breaker_increments_on_same_signature_no_new_rules() {
        let header = header_with(Some("schema-fail"), Some("x.y"));
        let raw = "schema error at src/foo.rs:10\n".to_string();
        let view = deterministic_trace_slicer(&raw, &header, B_DISTILL_IN, &tk());

        let bbs0 = compress_belief_state(None, &view, &[], "ev0", &scope(), B_D, &tk());
        assert_eq!(bbs0.zero_gain_streak, 0);

        let bbs1 = compress_belief_state(Some(&bbs0), &view, &[], "ev1", &scope(), B_D, &tk());
        // Same signature, no new rules => streak increments.
        assert_eq!(bbs1.zero_gain_streak, 1);

        let bbs2 = compress_belief_state(Some(&bbs1), &view, &[], "ev2", &scope(), B_D, &tk());
        let bbs3 = compress_belief_state(Some(&bbs2), &view, &[], "ev3", &scope(), B_D, &tk());
        assert_eq!(bbs3.zero_gain_streak, 3);
        // ZERO_GAIN_K threshold reached
        assert!(bbs3.zero_gain_streak >= ZERO_GAIN_K);
    }

    #[test]
    fn zero_gain_resets_when_signature_changes() {
        let header_a = header_with(Some("schema-fail"), Some("x.y"));
        let view_a = deterministic_trace_slicer(
            "schema error at src/foo.rs:10\n",
            &header_a,
            B_DISTILL_IN,
            &tk(),
        );
        let header_b = header_with(Some("timeout"), Some("x.z"));
        let view_b = deterministic_trace_slicer(
            "timeout at src/foo.rs:20\n",
            &header_b,
            B_DISTILL_IN,
            &tk(),
        );

        let bbs0 = compress_belief_state(None, &view_a, &[], "ev0", &scope(), B_D, &tk());
        let bbs1 = compress_belief_state(Some(&bbs0), &view_a, &[], "ev1", &scope(), B_D, &tk());
        assert_eq!(bbs1.zero_gain_streak, 1);
        let bbs2 = compress_belief_state(Some(&bbs1), &view_b, &[], "ev2", &scope(), B_D, &tk());
        // Different failure signature => streak resets to 0.
        assert_eq!(bbs2.zero_gain_streak, 0);
    }

    // ── orthogonal_memory_retention ─────────────────────────────

    #[test]
    fn orthogonal_memory_retention_three_distinct_constraints() {
        let raw = "assertion failed: foo\nat src/bar.rs:42\n";
        let header = header_with(Some("schema-fail"), Some("foo"));
        let trace = deterministic_trace_slicer(raw, &header, B_DISTILL_IN, &tk());

        let r1 = RetryConstraint {
            id: "must-include-schema_version".into(),
            rule: "always include schema_version".into(),
            priority: 250,
            source_attempt: 1,
            evidence_hash: "ev1".into(),
        };
        let r2 = RetryConstraint {
            id: "use-correct-path".into(),
            rule: "use src/correct/path.rs".into(),
            priority: 230,
            source_attempt: 2,
            evidence_hash: "ev2".into(),
        };
        let r3 = RetryConstraint {
            id: "predicate-foo-needs-x".into(),
            rule: "predicate foo requires x>0".into(),
            priority: 200,
            source_attempt: 3,
            evidence_hash: "ev3".into(),
        };

        let bbs0 =
            compress_belief_state(None, &trace, std::slice::from_ref(&r1), "ev1", &scope(), B_D, &tk());
        let bbs1 = compress_belief_state(
            Some(&bbs0),
            &trace,
            std::slice::from_ref(&r2),
            "ev2",
            &scope(),
            B_D,
            &tk(),
        );
        let bbs2 = compress_belief_state(
            Some(&bbs1),
            &trace,
            std::slice::from_ref(&r3),
            "ev3",
            &scope(),
            B_D,
            &tk(),
        );

        // All three constraint ids retained
        let ids: Vec<&str> = bbs2.constraints.iter().map(|c| c.id.as_str()).collect();
        assert!(ids.contains(&"must-include-schema_version"));
        assert!(ids.contains(&"use-correct-path"));
        assert!(ids.contains(&"predicate-foo-needs-x"));
        // BBS fits within B_D budget
        assert!(tk().count_json(&bbs2) <= B_D);
    }

    // ── stack frame + path extraction sanity ────────────────────

    #[test]
    fn extract_stack_frames_picks_up_rs_and_py_lines() {
        let raw = "panic at src/foo.rs:42\n  at handler.py:10\n  unrelated";
        let frames = extract_stack_frames(raw);
        assert!(frames.iter().any(|f| f.contains("src/foo.rs:42")));
        assert!(frames.iter().any(|f| f.contains("handler.py:10")));
    }

    #[test]
    fn extract_file_paths_dedupes() {
        let raw = "src/a/b.rs and again src/a/b.rs plus src/c/d.rs";
        let paths = extract_file_paths(raw);
        // dedup
        assert_eq!(
            paths.iter().filter(|p| *p == "src/a/b.rs").count(),
            1
        );
        assert!(paths.contains(&"src/c/d.rs".to_string()));
    }
}
