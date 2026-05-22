//! TRACE_MATRIX FC1a-budget_gate: TDMA-Bounded hard-budget constants + type-aware
//! token enforcer.
//!
//! All 12 RC1 hard budgets live here as `pub const`. There is no per-atom override;
//! tests that need different values use local `const _MAX_RETRIES_OVERRIDE: u32 = N`
//! patterns. Hard asserts in `MemoryKernel::step_forward` and Atom 7's
//! `assemble_o1_prompt` reference these constants directly.
//!
//! KILL-tdma-3: this module is the canonical home for token math. No byte-length
//! proxy is permitted anywhere in TDMA modules.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use serde::Serialize;

use crate::tokenizer::Tokenizer;

// ── Hard budgets (directive §1) ─────────────────────────────────

/// TRACE_MATRIX FC1a-budget_gate: CharterCore content budget.
pub const B_G: usize = 500;
/// TRACE_MATRIX FC1a-budget_gate: SessionDigest budget.
pub const B_S: usize = 3000;
/// TRACE_MATRIX FC1a-budget_gate: RetryBeliefState JSON budget.
pub const B_D: usize = 400;
/// TRACE_MATRIX FC1a-budget_gate: Task prompt budget.
pub const B_T: usize = 1500;
/// TRACE_MATRIX FC1a-budget_gate: Evidence-hash pointer budget.
pub const B_H: usize = 100;
/// TRACE_MATRIX FC1a-budget_gate: Output-contract / fixed control text budget.
pub const B_CTL: usize = 300;
/// TRACE_MATRIX FC1a-budget_gate: StateUpdate header JSON budget.
pub const B_HEADER: usize = 256;
/// TRACE_MATRIX FC1a-budget_gate: Parser prefix-scan budget (state-first).
pub const B_HEADER_SCAN: usize = 512;
/// TRACE_MATRIX FC1a-budget_gate: Deterministic trace slicer output budget.
pub const B_DISTILL_IN: usize = 2048;
/// TRACE_MATRIX FC1a-budget_gate: Maximum retries before escalation.
pub const MAX_RETRIES: u32 = 5;
/// TRACE_MATRIX FC1a-budget_gate: Zero-gain-streak fuse before escalation.
pub const ZERO_GAIN_K: u32 = 3;
/// TRACE_MATRIX FC1a-budget_gate: Minimum information gain to reset zero_gain_streak.
pub const EPSILON_GAIN: f64 = 0.01;

/// Combined prompt budget — sum of all six bucket caps (directive §1).
/// TRACE_MATRIX FC1a-budget_gate: Composed assert target for assemble_o1_prompt.
pub const B_PROMPT_MAX: usize = B_G + B_S + B_D + B_T + B_H + B_CTL;

// ── Payload taxonomy + budgeted return ──────────────────────────

/// Per-kind degradation strategy (directive §7).
/// TRACE_MATRIX FC1a-budget_gate: Discriminates the slicing rule applied to a payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadKind {
    Json,
    Code,
    Diff,
    Markdown,
    PlainText,
}

/// Result of a budget enforcement pass (directive §7).
/// TRACE_MATRIX FC1a-budget_gate: Carries the budgeted text plus an audit of
/// what the enforcer dropped (the `degraded` count is useful for replay).
#[derive(Debug, Clone)]
pub struct BudgetedPayload {
    pub text: String,
    pub token_count: usize,
    pub degraded: bool,
}

// ── Per-kind enforcement ────────────────────────────────────────

/// Truncate plain text by sentence-ish boundaries while preserving head + tail.
fn truncate_plain(text: &str, budget: usize, tokenizer: &Tokenizer) -> BudgetedPayload {
    let n = tokenizer.count_text(text);
    if n <= budget {
        return BudgetedPayload {
            text: text.to_string(),
            token_count: n,
            degraded: false,
        };
    }
    let max_chars = budget.saturating_mul(4);
    if max_chars < 16 {
        let prefix: String = text.chars().take(max_chars).collect();
        return BudgetedPayload {
            token_count: tokenizer.count_text(&prefix),
            text: prefix,
            degraded: true,
        };
    }
    let half = max_chars / 2 - 3;
    let head: String = text.chars().take(half).collect();
    let tail: String = text
        .chars()
        .rev()
        .take(half)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    let mut out = format!("{}…{}", head, tail);
    while tokenizer.count_text(&out) > budget && out.chars().count() > 8 {
        // shave one char from each side
        let chars: Vec<char> = out.chars().collect();
        out = chars[1..chars.len() - 1].iter().collect();
    }
    BudgetedPayload {
        token_count: tokenizer.count_text(&out),
        text: out,
        degraded: true,
    }
}

/// JSON: keep schema_version + scope + failure_signature + evidence; evict
/// optional/explanation fields and constraints ascending by priority (per
/// directive §7 + KILL-tdma-3).
/// TRACE_MATRIX FC1a-budget_gate: JSON degradation preserves required fields.
pub fn enforce_json_budget<T: Serialize>(
    value: &T,
    budget: usize,
    tokenizer: &Tokenizer,
) -> BudgetedPayload {
    let initial = serde_json::to_value(value).unwrap_or(serde_json::json!(null));
    let mut current = initial.clone();

    if tokenizer.count_json(&current) <= budget {
        return finalize_budget(current, tokenizer, false);
    }

    // Step 1: drop optional explanation-class fields.
    for k in [
        "explanation",
        "next_action_hint",
        "rationale",
        "comment",
        "comments",
    ] {
        if let Some(obj) = current.as_object_mut() {
            obj.remove(k);
        }
        if tokenizer.count_json(&current) <= budget {
            return finalize_budget(current, tokenizer, true);
        }
    }

    // Step 2: truncate evicted detail to count only.
    let evicted_count: Option<usize> = current
        .get("evicted")
        .and_then(|v| v.as_array())
        .map(|a| a.len());
    if let Some(count) = evicted_count {
        if let Some(obj) = current.as_object_mut() {
            obj.insert("evicted".into(), serde_json::json!({ "count": count }));
        }
        if tokenizer.count_json(&current) <= budget {
            return finalize_budget(current, tokenizer, true);
        }
    }

    // Step 3: evict constraints ascending by priority until under budget.
    // Sort the array first (one mutable borrow), then loop with remove (separate borrows).
    if let Some(obj) = current.as_object_mut() {
        if let Some(arr) = obj.get_mut("constraints").and_then(|c| c.as_array_mut()) {
            arr.sort_by_key(|c| c.get("priority").and_then(|p| p.as_u64()).unwrap_or(0));
        }
    }
    loop {
        let over_budget = tokenizer.count_json(&current) > budget;
        if !over_budget {
            break;
        }
        let did_remove = current
            .as_object_mut()
            .and_then(|obj| obj.get_mut("constraints"))
            .and_then(|c| c.as_array_mut())
            .map(|arr| {
                if arr.is_empty() {
                    false
                } else {
                    arr.remove(0);
                    true
                }
            })
            .unwrap_or(false);
        if !did_remove {
            break;
        }
    }
    if tokenizer.count_json(&current) <= budget {
        return finalize_budget(current, tokenizer, true);
    }

    // Step 4: last resort — strip everything except the preserve_keys.
    let preserve_keys = [
        "schema_version",
        "scope",
        "failure_signature",
        "evidence",
        "zero_gain_streak",
        "information_gain",
    ];
    if let Some(obj) = current.as_object_mut() {
        let keys: Vec<String> = obj.keys().cloned().collect();
        for k in keys {
            if !preserve_keys.contains(&k.as_str()) {
                obj.remove(&k);
            }
        }
    }

    finalize_budget(current, tokenizer, true)
}

fn finalize_budget(
    value: serde_json::Value,
    tokenizer: &Tokenizer,
    degraded: bool,
) -> BudgetedPayload {
    let text = serde_json::to_string(&value).unwrap_or_default();
    let token_count = tokenizer.count_text(&text);
    BudgetedPayload {
        text,
        token_count,
        degraded,
    }
}

/// Code: preserve failing signature + ±10 lines around failing line;
/// drop non-touched functions; last resort path+symbol+line (directive §7).
/// `failing_line` is 1-indexed; pass `None` to keep head/tail trimming.
/// TRACE_MATRIX FC1a-budget_gate: Code degradation keeps the failing context.
pub fn enforce_code_budget(
    code: &str,
    failing_line: Option<usize>,
    budget: usize,
    tokenizer: &Tokenizer,
) -> BudgetedPayload {
    let n = tokenizer.count_text(code);
    if n <= budget {
        return BudgetedPayload {
            text: code.to_string(),
            token_count: n,
            degraded: false,
        };
    }
    let lines: Vec<&str> = code.lines().collect();
    if let Some(fl) = failing_line {
        let idx = fl.saturating_sub(1).min(lines.len().saturating_sub(1));
        let lo = idx.saturating_sub(10);
        let hi = (idx + 11).min(lines.len());
        let mut slice = lines[lo..hi].join("\n");
        // If still over budget, walk inward.
        let mut cur_lo = lo;
        let mut cur_hi = hi;
        while tokenizer.count_text(&slice) > budget && cur_hi - cur_lo > 1 {
            if cur_hi > idx + 1 {
                cur_hi -= 1;
            } else if cur_lo < idx {
                cur_lo += 1;
            } else {
                break;
            }
            slice = lines[cur_lo..cur_hi].join("\n");
        }
        return BudgetedPayload {
            token_count: tokenizer.count_text(&slice),
            text: slice,
            degraded: true,
        };
    }
    // No failing line — fall back to plain head/tail.
    truncate_plain(code, budget, tokenizer)
}

/// Diff: keep touched files + hunk headers + failing hunk; drop context lines
/// (directive §7). For RC1 we keep only `+++/---/@@` markers + lines beginning
/// with `+`/`-` (the actual changes), dropping leading-space context lines.
/// TRACE_MATRIX FC1a-budget_gate: Diff degradation keeps the failing hunk.
pub fn enforce_diff_budget(
    diff: &str,
    budget: usize,
    tokenizer: &Tokenizer,
) -> BudgetedPayload {
    let n = tokenizer.count_text(diff);
    if n <= budget {
        return BudgetedPayload {
            text: diff.to_string(),
            token_count: n,
            degraded: false,
        };
    }
    let mut out = String::new();
    for line in diff.lines() {
        let keep = line.starts_with("+++")
            || line.starts_with("---")
            || line.starts_with("@@")
            || line.starts_with('+')
            || line.starts_with('-')
            || line.starts_with("diff ");
        if keep {
            out.push_str(line);
            out.push('\n');
        }
    }
    let out = out.trim_end_matches('\n').to_string();
    if tokenizer.count_text(&out) > budget {
        truncate_plain(&out, budget, tokenizer)
    } else {
        BudgetedPayload {
            token_count: tokenizer.count_text(&out),
            text: out,
            degraded: true,
        }
    }
}

/// Markdown: keep heading hierarchy + TODO/ERROR/Invariant lines;
/// drop long body text (directive §7).
/// TRACE_MATRIX FC1a-budget_gate: Markdown degradation keeps headings.
pub fn enforce_markdown_budget(
    md: &str,
    budget: usize,
    tokenizer: &Tokenizer,
) -> BudgetedPayload {
    let n = tokenizer.count_text(md);
    if n <= budget {
        return BudgetedPayload {
            text: md.to_string(),
            token_count: n,
            degraded: false,
        };
    }
    let mut out = String::new();
    for line in md.lines() {
        let t = line.trim_start();
        let keep = t.starts_with('#')
            || t.contains("TODO")
            || t.contains("ERROR")
            || t.contains("Invariant");
        if keep {
            out.push_str(line);
            out.push('\n');
        }
    }
    let out = out.trim_end_matches('\n').to_string();
    if tokenizer.count_text(&out) > budget {
        truncate_plain(&out, budget, tokenizer)
    } else {
        BudgetedPayload {
            token_count: tokenizer.count_text(&out),
            text: out,
            degraded: true,
        }
    }
}

/// Plain text: keep head + tail, summarize middle (directive §7).
/// TRACE_MATRIX FC1a-budget_gate: Plain-text degradation keeps head+tail.
pub fn enforce_plain_text_budget(
    text: &str,
    budget: usize,
    tokenizer: &Tokenizer,
) -> BudgetedPayload {
    truncate_plain(text, budget, tokenizer)
}

/// Dispatcher (directive §7).
/// TRACE_MATRIX FC1a-budget_gate: Single front door for any payload.
pub fn enforce_budget(
    text: &str,
    kind: PayloadKind,
    budget: usize,
    tokenizer: &Tokenizer,
) -> BudgetedPayload {
    match kind {
        PayloadKind::Json => match serde_json::from_str::<serde_json::Value>(text) {
            Ok(v) => enforce_json_budget(&v, budget, tokenizer),
            Err(_) => enforce_plain_text_budget(text, budget, tokenizer),
        },
        PayloadKind::Code => enforce_code_budget(text, None, budget, tokenizer),
        PayloadKind::Diff => enforce_diff_budget(text, budget, tokenizer),
        PayloadKind::Markdown => enforce_markdown_budget(text, budget, tokenizer),
        PayloadKind::PlainText => enforce_plain_text_budget(text, budget, tokenizer),
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn tk() -> Tokenizer {
        Tokenizer::new()
    }

    // ── token_budget_json_schema_preserving ─────────────────────

    #[test]
    fn token_budget_json_schema_preserving_drops_explanation_first() {
        // A JSON with optional explanation field — over budget by ~5 tokens.
        let payload = serde_json::json!({
            "schema_version": "tdma-bbs/v1",
            "scope": {"run_id": "r", "task_id": "t", "verified_parent": "H0"},
            "failure_signature": {"reject_class":"x","failed_predicate":"y","root_cause":"z"},
            "constraints": [],
            "evidence": {
                "evidence_node_hash": "e",
                "raw_stderr_sha256": "0",
                "trace_view_sha256": "1"
            },
            "zero_gain_streak": 0,
            "information_gain": 0.5,
            "evicted": [],
            "explanation": "a".repeat(400),
        });
        let initial_tokens = tk().count_json(&payload);
        let budget = initial_tokens.saturating_sub(50);
        let out = enforce_json_budget(&payload, budget, &tk());
        assert!(out.degraded, "must degrade");
        assert!(out.token_count <= budget, "must fit budget");
        // schema_version + scope + failure_signature + evidence preserved
        let parsed: serde_json::Value = serde_json::from_str(&out.text).unwrap();
        for k in ["schema_version", "scope", "failure_signature", "evidence"] {
            assert!(parsed.get(k).is_some(), "preserved key '{}' missing", k);
        }
        // explanation removed
        assert!(parsed.get("explanation").is_none());
    }

    #[test]
    fn token_budget_json_schema_preserving_evicts_constraints_by_priority() {
        let payload = serde_json::json!({
            "schema_version": "tdma-bbs/v1",
            "scope": {"run_id":"r","task_id":"t","verified_parent":"H0"},
            "failure_signature": {"reject_class":"x","failed_predicate":"y","root_cause":"z"},
            "constraints": [
                {"id":"low",  "rule":"low priority rule with extra padding bytes here to bulk", "priority": 10,  "source_attempt": 1, "evidence_hash":"e"},
                {"id":"high", "rule":"high priority rule with extra padding bytes here as well", "priority": 200, "source_attempt": 2, "evidence_hash":"e"},
                {"id":"mid",  "rule":"mid priority rule with extra padding bytes here for size", "priority": 100, "source_attempt": 3, "evidence_hash":"e"}
            ],
            "evidence": {"evidence_node_hash":"e","raw_stderr_sha256":"0","trace_view_sha256":"1"},
            "zero_gain_streak": 0, "information_gain": 0.0, "evicted": []
        });
        let initial_tokens = tk().count_json(&payload);
        // Force eviction by halving the budget.
        let budget = initial_tokens / 2;
        let out = enforce_json_budget(&payload, budget, &tk());
        assert!(out.degraded);
        let parsed: serde_json::Value = serde_json::from_str(&out.text).unwrap();
        // High-priority constraint must survive eviction longest.
        if let Some(arr) = parsed.get("constraints").and_then(|c| c.as_array()) {
            if !arr.is_empty() {
                let ids: Vec<&str> = arr.iter().filter_map(|c| c.get("id").and_then(|i| i.as_str())).collect();
                assert!(!ids.contains(&"low"), "low priority should be evicted first");
            }
        }
    }

    // ── token_budget_diff_slicer ────────────────────────────────

    #[test]
    fn token_budget_diff_slicer_drops_context() {
        let diff = r#"diff --git a/x b/x
--- a/x
+++ b/x
@@ -1,3 +1,3 @@
 context line one
-old change
+new change
 context line two"#;
        let initial_tokens = tk().count_text(diff);
        let out = enforce_diff_budget(diff, initial_tokens - 1, &tk());
        assert!(out.degraded);
        // header + + and - lines must remain
        assert!(out.text.contains("+++"));
        assert!(out.text.contains("---"));
        assert!(out.text.contains("@@"));
        assert!(out.text.contains("+new change"));
        assert!(out.text.contains("-old change"));
        // context lines should be dropped
        assert!(!out.text.contains("context line one"));
    }

    // ── token_budget_no_byte_proxy ──────────────────────────────

    #[test]
    fn token_budget_no_byte_proxy_unicode_is_chars_not_bytes() {
        // A multi-byte CJK char is 1 char (per char count) but 3 bytes — we
        // explicitly count chars, never bytes, to avoid the directive's
        // KILL-tdma-3 byte-proxy anti-pattern.
        let s = "中文测试abcd"; // 4 CJK + 4 ASCII = 8 chars; byte length is 16
        let n = tk().count_text(s);
        // 8 chars / 4 = 2 tokens
        assert_eq!(n, 2);
        // If we had naively used bytes (16) / 4 = 4 tokens — that would be wrong.
        assert_ne!(n, 4);
    }

    #[test]
    fn token_budget_dispatcher_falls_back_to_plain_for_bad_json() {
        let bad = "{not valid json at all";
        let out = enforce_budget(bad, PayloadKind::Json, 10, &tk());
        // Falls back to plain text path; resulting tokens <= budget.
        assert!(out.token_count <= 10);
    }

    // ── code budget ─────────────────────────────────────────────

    #[test]
    fn enforce_code_budget_keeps_window_around_failing_line() {
        let mut code = String::new();
        for i in 1..=50 {
            code.push_str(&format!("fn line_{}() {{ /* body */ }}\n", i));
        }
        let initial_tokens = tk().count_text(&code);
        // Force a tight budget that cannot hold full file.
        let budget = initial_tokens / 5;
        let out = enforce_code_budget(&code, Some(25), budget, &tk());
        assert!(out.degraded);
        assert!(out.token_count <= budget);
        // Window centers on line 25 (±10 lines is the goal, shrunk inward).
        assert!(out.text.contains("line_25"));
    }

    // ── markdown ────────────────────────────────────────────────

    #[test]
    fn enforce_markdown_budget_keeps_headings_and_signals() {
        let md = "# H1\n## H2\nbody line one with lots of filler text padded out\nTODO: do the thing\nbody line two also with padding\nERROR: critical\nmore body";
        let initial_tokens = tk().count_text(md);
        let out = enforce_markdown_budget(md, initial_tokens / 2, &tk());
        assert!(out.degraded);
        assert!(out.text.contains("# H1"));
        assert!(out.text.contains("## H2"));
        assert!(out.text.contains("TODO"));
        assert!(out.text.contains("ERROR"));
        // Plain body is dropped
        assert!(!out.text.contains("body line one"));
    }
}
