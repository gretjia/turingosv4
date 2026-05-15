//! Constitution gate — fail-open admission default lint for sequencer.
//!
//! Authority: Codex Stage C overall §8 R2 CHALLENGE Q10 (2026-05-09 session
//! #32). The R1 event-state gate added live `state == Open` checks to the
//! Polymarket admission arms (CpmmPool / CpmmSwap / BuyWithCoinRouter), but
//! the R1 implementation read state via
//!
//!   `task_markets_t.0.get(&event).map(|e| e.state).unwrap_or(TaskMarketState::Open)`
//!
//! That default admits txs against MISSING entries (malformed / pre-genesis
//! events) — a fail-OPEN admission semantic. Codex flagged this in R2; the
//! R3 fix replaced the `unwrap_or` with `.ok_or(TransitionError::EventNotOpen)?`
//! for fail-CLOSED admission semantics. See
//! `feedback_admission_fail_closed_default.md` for the recurring rule.
//!
//! This gate scans `src/state/sequencer.rs` for any same-line co-occurrence
//! of an `unwrap_or` family call and a state-machine `Open` / `Active`
//! variant — the textual shape of the R2 Q10 defect. The fix is to use
//! `.ok_or(<RejectError>)?` (preferred) or to replace the default with a
//! known-rejecting variant such as `TaskMarketState::Bankrupt`.
//!
//! Forbidden tokens (case-sensitive Rust enum-variant form, with or without
//! path qualification — substring scan picks up `crate::state::q_state::
//! TaskMarketState::Open` too):
//!   - `TaskMarketState::Open`  — Q10 root case
//!   - `ChallengeStatus::Open`  — TB-5 challenge admission
//!   - `ClaimStatus::Open`      — TB-3 claim admission
//!   - `PoolStatus::Active`     — P-M4 pool admission ("Active" is open-class)
//!   - `EventState::Open`       — forward-eligible naming
//!
//! Forbidden patterns (any of these on the same code line as an `Open`/
//! `Active` token above):
//!   - `.unwrap_or(`
//!   - `.unwrap_or_else(`
//!
//! Mirrors the source-grep gate pattern of
//! `tests/constitution_economy_strict_equality.rs` (Phase E.3) — same
//! comment-stripping logic to prevent disguising violations inside comments.

use std::path::PathBuf;

const SEQUENCER_FILE: &str = "src/state/sequencer.rs";

/// State-machine variants whose presence as an `unwrap_or` default admits a
/// missing-entry tx into a permissive ("trading allowed") state. Substring
/// scan catches both bare and path-qualified forms.
const FAIL_OPEN_DEFAULT_TOKENS: &[&str] = &[
    "TaskMarketState::Open",
    "ChallengeStatus::Open",
    "ClaimStatus::Open",
    "PoolStatus::Active",
    "EventState::Open",
];

/// Unwrap families that turn `Option::None` (= missing entry) into a default
/// value. Combined with a fail-open token on the same line this is the
/// documented R2 Q10 defect shape.
const UNWRAP_OR_PATTERNS: &[&str] = &[".unwrap_or(", ".unwrap_or_else("];

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_file(rel: &str) -> String {
    let path = workspace_root().join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("fail-open lint: failed to read {}: {}", path.display(), e))
}

fn is_doc_or_comment_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("///") || trimmed.starts_with("//!") || trimmed.starts_with("//")
}

/// Strip everything after `//` (line-comment) so a fail-open token mentioned
/// only in a comment cannot trigger a false positive, and an `unwrap_or`
/// call commented out cannot be falsely flagged.
fn strip_inline_comment(line: &str) -> &str {
    if let Some(idx) = line.find("//") {
        &line[..idx]
    } else {
        line
    }
}

/// Scan a Rust source string for same-line co-occurrence of any
/// `UNWRAP_OR_PATTERN` and any `FAIL_OPEN_DEFAULT_TOKEN`. Returns
/// (line_number_1_indexed, line_text) for each violation.
fn scan_fail_open_defaults(content: &str) -> Vec<(usize, String)> {
    let mut violations = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if is_doc_or_comment_line(line) {
            continue;
        }
        let code = strip_inline_comment(line);
        let has_unwrap = UNWRAP_OR_PATTERNS.iter().any(|p| code.contains(p));
        if !has_unwrap {
            continue;
        }
        let has_fail_open = FAIL_OPEN_DEFAULT_TOKENS.iter().any(|t| code.contains(t));
        if !has_fail_open {
            continue;
        }
        violations.push((i + 1, line.to_string()));
    }
    violations
}

#[test]
fn no_fail_open_default_in_sequencer_admission() {
    let content = read_file(SEQUENCER_FILE);
    let violations = scan_fail_open_defaults(&content);
    assert!(
        violations.is_empty(),
        "Stage C R2 Q10 lint failed in {}: {} fail-open default(s) found.\n\
         Each `.unwrap_or(...)` / `.unwrap_or_else(...)` whose same code line \
         mentions one of {:?} admits missing-entry txs into a permissive state — \
         the R2 Q10 fail-open class. Replace with `.ok_or(<RejectError>)?` for \
         fail-closed admission semantics (see `feedback_admission_fail_closed_default.md`).\n\
         Violations:\n{}",
        SEQUENCER_FILE,
        violations.len(),
        FAIL_OPEN_DEFAULT_TOKENS,
        violations
            .iter()
            .map(|(ln, l)| format!("  {}:{}  {}", SEQUENCER_FILE, ln, l.trim()))
            .collect::<Vec<_>>()
            .join("\n"),
    );
}

#[test]
fn lint_self_check_unwrap_or_task_market_open_flagged() {
    // Documented R2 Q10 defect shape (verbatim modulo identifier names):
    //   `.get(&id).map(|e| e.state).unwrap_or(TaskMarketState::Open)`
    // — must be flagged.
    let synthetic = r#"
fn admission_arm() -> Result<(), ()> {
    let market_state = task_markets_t.0
        .get(&event_id.0)
        .map(|e| e.state)
        .unwrap_or(TaskMarketState::Open);
    if market_state != TaskMarketState::Open {
        return Err(());
    }
    Ok(())
}
"#;
    let violations = scan_fail_open_defaults(synthetic);
    assert!(
        !violations.is_empty(),
        "Q10 lint self-check: synthetic `.unwrap_or(TaskMarketState::Open)` MUST be flagged; got 0 violations."
    );
    let (_ln, line) = &violations[0];
    assert!(
        line.contains("unwrap_or(TaskMarketState::Open)"),
        "Q10 lint self-check: violation should be on the unwrap_or line; got: {}",
        line,
    );
}

#[test]
fn lint_self_check_unwrap_or_else_pool_active_flagged() {
    // `.unwrap_or_else(|| PoolStatus::Active)` — closure form must be caught.
    let synthetic = r#"
fn pool_admission() {
    let status = cpmm_pools_t.0
        .get(&event_id)
        .map(|p| p.status)
        .unwrap_or_else(|| PoolStatus::Active);
    let _ = status;
}
"#;
    let violations = scan_fail_open_defaults(synthetic);
    assert!(
        !violations.is_empty(),
        "Q10 lint self-check: `.unwrap_or_else(|| PoolStatus::Active)` MUST be flagged."
    );
}

#[test]
fn lint_self_check_path_qualified_token_flagged() {
    // Path-qualified token form (mirrors how sequencer.rs writes
    // `crate::state::q_state::TaskMarketState::Open` in places).
    let synthetic = r#"
fn admission_path_qualified() {
    let s = q.economic_state_t.task_markets_t.0
        .get(&id)
        .map(|e| e.state)
        .unwrap_or(crate::state::q_state::TaskMarketState::Open);
    let _ = s;
}
"#;
    let violations = scan_fail_open_defaults(synthetic);
    assert!(
        !violations.is_empty(),
        "Q10 lint self-check: path-qualified `crate::state::q_state::TaskMarketState::Open` \
         in `.unwrap_or(...)` MUST be flagged."
    );
}

#[test]
fn lint_self_check_challenge_open_flagged() {
    // `ChallengeStatus::Open` is also fail-open class for challenge admission.
    let synthetic = r#"
fn challenge_admission() {
    let status = challenges_t.0
        .get(&id)
        .map(|c| c.status)
        .unwrap_or(ChallengeStatus::Open);
    let _ = status;
}
"#;
    let violations = scan_fail_open_defaults(synthetic);
    assert!(
        !violations.is_empty(),
        "Q10 lint self-check: `.unwrap_or(ChallengeStatus::Open)` MUST be flagged."
    );
}

#[test]
fn lint_self_check_ok_or_pattern_passes() {
    // `.ok_or(EventNotOpen)?` is the fail-closed remediation pattern — must
    // NOT be flagged. (No `unwrap_or` substring at all on the line.)
    let synthetic = r#"
fn admission_fail_closed() -> Result<(), &'static str> {
    let market_entry = task_markets_t.0
        .get(&event_id.0)
        .ok_or(TransitionError::EventNotOpen)?;
    if market_entry.state != TaskMarketState::Open {
        return Err("EventNotOpen");
    }
    Ok(())
}
"#;
    let violations = scan_fail_open_defaults(synthetic);
    assert!(
        violations.is_empty(),
        "Q10 lint self-check: fail-closed `.ok_or(...)?` pattern MUST NOT be flagged; got {} violations: {:?}",
        violations.len(),
        violations,
    );
}

#[test]
fn lint_self_check_money_default_passes() {
    // `MicroCoin::zero()` defaults are fail-closed (zero balance triggers
    // amount-insufficient reject downstream). Must NOT be flagged.
    let synthetic = r#"
fn money_default() {
    let bal = balances_t.0
        .get(&account)
        .copied()
        .unwrap_or(crate::economy::money::MicroCoin::zero());
    let _ = bal;
}
"#;
    let violations = scan_fail_open_defaults(synthetic);
    assert!(
        violations.is_empty(),
        "Q10 lint self-check: `MicroCoin::zero()` default is fail-closed; MUST NOT be flagged; got {} violations: {:?}",
        violations.len(),
        violations,
    );
}

#[test]
fn lint_self_check_token_in_comment_does_not_trigger_false_positive() {
    // A fail-open token mentioned only inside a `//` comment must NOT trigger
    // the gate (the `unwrap_or` is on a different code-only line). Mirrors
    // `constitution_economy_strict_equality::lint_self_check_marker_in_comment_*`
    // hardening idea.
    let synthetic = r#"
fn unrelated() {
    // historical context: prior code defaulted to TaskMarketState::Open here
    let bal = balances_t.0.get(&id).copied().unwrap_or(0u128);
    let _ = bal;
}
"#;
    let violations = scan_fail_open_defaults(synthetic);
    assert!(
        violations.is_empty(),
        "Q10 lint self-check: token-in-comment must NOT cause a false positive on an \
         unrelated `unwrap_or(0u128)`; got {} violations: {:?}",
        violations.len(),
        violations,
    );
}

#[test]
fn lint_self_check_commented_out_violation_not_flagged() {
    // A `.unwrap_or(TaskMarketState::Open)` that is fully inside a `//` comment
    // (i.e. the entire violation is commented out) MUST NOT be flagged. The
    // `is_doc_or_comment_line` early-skip catches this.
    let synthetic = r#"
fn commented_violation() {
    // let bad = m.get(&id).map(|e| e.state).unwrap_or(TaskMarketState::Open);
    let good = 1;
    let _ = good;
}
"#;
    let violations = scan_fail_open_defaults(synthetic);
    assert!(
        violations.is_empty(),
        "Q10 lint self-check: a fully-commented-out violation MUST NOT be flagged; \
         got {} violations: {:?}",
        violations.len(),
        violations,
    );
}
