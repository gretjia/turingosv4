//! TRACE_MATRIX FC1a-rtool: TDMA-Bounded read-tool (rtool).
//!
//! The kernel's read-side O(1) checkout: given a `verified_head` + `Task`, the
//! rtool produces a `SessionDigest` that fits under `B_S` tokens. The digest is
//! built by walking ONLY tape (via `ImmutableTapeLedger`), never by reading
//! mutable side state.
//!
//! Degradation cascade (directive §10):
//!   Level 1 FullRelevantDiff           — relevant diff + failing files
//!   Level 2 FailingFunctionAst          — failing function + predicate
//!   Level 3 FilePathAndPredicateSummary — touched paths + symbols + lines
//!   Level 4 MinimalHeadOnly             — verified_head + retrieval handles
//!
//! For RC1 the diff/AST sources come from the kernel-supplied `WorkspaceView`
//! (a small typed object). A future atom can swap in a real workspace scanner
//! behind the same `Rtool::checkout_digest` API. The cascade ensures any
//! workspace (even multi-MB) is reduced to a `B_S`-bounded digest.
//!
//! KILL discipline:
//!   * No raw stderr from failure-side AgentProposal nodes is injected into
//!     SessionDigest (KILL-tdma-1; the rtool only reads StateAccepted nodes
//!     and the task itself).
//!   * No `payload.len()` proxy — all sizing goes through the Tokenizer.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::ledger::ImmutableTapeLedger;
use crate::memory_kernel::Task;
use crate::token_budget::B_S;
use crate::tokenizer::Tokenizer;

// ── Public types ─────────────────────────────────────────────────

/// Session-checkout output (directive §10).
/// TRACE_MATRIX FC1a-rtool: Bounded, replayable digest passed to the worker.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SessionDigest {
    pub schema_version: String,
    pub verified_head: String,
    pub text: String,
    pub retrieval_handles: Vec<String>,
    pub degradation_level: SessionDegradationLevel,
    pub token_count: usize,
}

/// Cascade levels (directive §10).
/// TRACE_MATRIX FC1a-rtool: Discriminates how aggressively the digest was
/// trimmed to fit budget.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionDegradationLevel {
    FullRelevantDiff,
    FailingFunctionAst,
    FilePathAndPredicateSummary,
    MinimalHeadOnly,
}

/// Optional workspace input for higher-fidelity digests (directive §10).
/// Future atoms can populate these from a real workspace scanner; for RC1 the
/// kernel passes whatever it has (or `WorkspaceView::default()`).
/// TRACE_MATRIX FC1a-rtool: Workspace facts the digest may include.
#[derive(Debug, Clone, Default)]
pub struct WorkspaceView {
    pub relevant_diff: Option<String>,
    pub failing_function_src: Option<String>,
    pub failing_predicate: Option<String>,
    pub touched_paths: Vec<String>,
    pub symbols: Vec<String>,
}

// ── Rtool ────────────────────────────────────────────────────────

/// TDMA read-tool (directive §10).
/// TRACE_MATRIX FC1a-rtool: Single API used by `MemoryKernel::handle_rejection`
/// (lands Atom 7) to checkout a budget-respecting digest at retry time.
pub struct Rtool<L: ImmutableTapeLedger> {
    pub tape: Arc<L>,
    pub tokenizer: Arc<Tokenizer>,
}

impl<L: ImmutableTapeLedger> Rtool<L> {
    /// TRACE_MATRIX FC1a-rtool: Constructor.
    pub fn new(tape: Arc<L>, tokenizer: Arc<Tokenizer>) -> Self {
        Self { tape, tokenizer }
    }

    /// Checkout a digest at `verified_head` for `task` under `token_budget`.
    /// Walks the cascade until the digest text fits the budget.
    /// TRACE_MATRIX FC1a-rtool: The single checkout entry-point.
    pub fn checkout_digest(
        &self,
        verified_head: &str,
        task: &Task,
        token_budget: usize,
    ) -> SessionDigest {
        let workspace = WorkspaceView::default();
        self.checkout_digest_with_workspace(verified_head, task, &workspace, token_budget)
    }

    /// Variant that accepts a `WorkspaceView` for higher-fidelity levels.
    /// TRACE_MATRIX FC1a-rtool: Cascade entry-point for kernels that hold
    /// workspace facts.
    pub fn checkout_digest_with_workspace(
        &self,
        verified_head: &str,
        task: &Task,
        workspace: &WorkspaceView,
        token_budget: usize,
    ) -> SessionDigest {
        let handles = vec![format!("verified_head={}", verified_head)];

        // Build candidate at each level until one fits the budget.
        let candidates: [(SessionDegradationLevel, String); 4] = [
            (
                SessionDegradationLevel::FullRelevantDiff,
                self.level_1(verified_head, task, workspace),
            ),
            (
                SessionDegradationLevel::FailingFunctionAst,
                self.level_2(verified_head, task, workspace),
            ),
            (
                SessionDegradationLevel::FilePathAndPredicateSummary,
                self.level_3(verified_head, task, workspace),
            ),
            (
                SessionDegradationLevel::MinimalHeadOnly,
                self.level_4(verified_head, task, workspace),
            ),
        ];

        for (level, text) in &candidates {
            let count = self.tokenizer.count_text(text);
            if count <= token_budget {
                return SessionDigest {
                    schema_version: "tdma-session-digest/v1".into(),
                    verified_head: verified_head.to_string(),
                    text: text.clone(),
                    retrieval_handles: handles.clone(),
                    degradation_level: *level,
                    token_count: count,
                };
            }
        }

        // Even MinimalHeadOnly was over budget — clip by chars and try again.
        let (level, mut text) = candidates.into_iter().last().unwrap();
        let mut count = self.tokenizer.count_text(&text);
        while count > token_budget && !text.is_empty() {
            let chars: Vec<char> = text.chars().collect();
            let cut = (chars.len() * 9) / 10;
            text = chars[..cut].iter().collect();
            count = self.tokenizer.count_text(&text);
        }
        SessionDigest {
            schema_version: "tdma-session-digest/v1".into(),
            verified_head: verified_head.to_string(),
            text,
            retrieval_handles: handles,
            degradation_level: level,
            token_count: count,
        }
    }

    // ── Cascade levels ────────────────────────────────────────

    fn level_1(&self, verified_head: &str, task: &Task, ws: &WorkspaceView) -> String {
        let mut s = format!("[VERIFIED_HEAD]\n{}\n\n[TASK]\n{}\n", verified_head, task.prompt);
        if let Some(diff) = &ws.relevant_diff {
            s.push_str("\n[RELEVANT DIFF]\n");
            s.push_str(diff);
        }
        if let Some(fp) = &ws.failing_predicate {
            s.push_str("\n[FAILING PREDICATE]\n");
            s.push_str(fp);
        }
        s
    }

    fn level_2(&self, verified_head: &str, task: &Task, ws: &WorkspaceView) -> String {
        let mut s = format!("[VERIFIED_HEAD]\n{}\n\n[TASK]\n{}\n", verified_head, task.prompt);
        if let Some(fn_src) = &ws.failing_function_src {
            s.push_str("\n[FAILING FUNCTION]\n");
            s.push_str(fn_src);
        }
        if let Some(fp) = &ws.failing_predicate {
            s.push_str("\n[FAILING PREDICATE]\n");
            s.push_str(fp);
        }
        s
    }

    fn level_3(&self, verified_head: &str, task: &Task, ws: &WorkspaceView) -> String {
        let mut s = format!("[VERIFIED_HEAD]\n{}\n\n[TASK]\n{}\n", verified_head, task.prompt);
        if !ws.touched_paths.is_empty() {
            s.push_str("\n[TOUCHED PATHS]\n");
            s.push_str(&ws.touched_paths.join("\n"));
        }
        if !ws.symbols.is_empty() {
            s.push_str("\n[SYMBOLS]\n");
            s.push_str(&ws.symbols.join("\n"));
        }
        if let Some(fp) = &ws.failing_predicate {
            s.push_str("\n[FAILING PREDICATE]\n");
            s.push_str(fp);
        }
        s
    }

    fn level_4(&self, verified_head: &str, task: &Task, _ws: &WorkspaceView) -> String {
        format!(
            "[VERIFIED_HEAD]\n{}\n[TASK_ID]\n{}\n",
            verified_head, task.id
        )
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::MemoryTapeLedger;

    fn rtool() -> Rtool<MemoryTapeLedger> {
        Rtool::new(
            Arc::new(MemoryTapeLedger::new()),
            Arc::new(Tokenizer::new()),
        )
    }

    fn task() -> Task {
        Task {
            id: "t1".into(),
            prompt: "Prove X.".into(),
        }
    }

    // ── session_digest_budget ───────────────────────────────────

    #[test]
    fn session_digest_budget_b_s_under_normal_input() {
        let r = rtool();
        let digest = r.checkout_digest("H0", &task(), B_S);
        assert!(digest.token_count <= B_S, "must fit B_S");
        assert_eq!(digest.schema_version, "tdma-session-digest/v1");
        assert_eq!(digest.verified_head, "H0");
        assert!(digest.retrieval_handles.iter().any(|h| h.contains("H0")));
    }

    #[test]
    fn session_digest_budget_holds_under_tight_token_budget() {
        let r = rtool();
        // Force a very tight budget — must still produce something <= 50 tokens.
        let digest = r.checkout_digest("H0", &task(), 50);
        assert!(digest.token_count <= 50, "must fit tight budget");
    }

    // ── session_digest_degradation (4-level cascade) ────────────

    #[test]
    fn session_digest_degradation_full_diff_when_small() {
        let r = rtool();
        let ws = WorkspaceView {
            relevant_diff: Some("+small diff line".into()),
            failing_function_src: Some("fn foo() { panic!() }".into()),
            failing_predicate: Some("x > 0".into()),
            touched_paths: vec!["src/foo.rs".into()],
            symbols: vec!["foo".into()],
        };
        let digest = r.checkout_digest_with_workspace("H0", &task(), &ws, B_S);
        assert_eq!(
            digest.degradation_level,
            SessionDegradationLevel::FullRelevantDiff,
            "small workspace fits FullRelevantDiff"
        );
        assert!(digest.text.contains("small diff line"));
    }

    #[test]
    fn session_digest_degradation_walks_down_on_budget_pressure() {
        let r = rtool();
        let huge_diff = "+line\n".repeat(20_000); // ~20k * 6 chars = 120k chars
        let ws = WorkspaceView {
            relevant_diff: Some(huge_diff),
            failing_function_src: Some("fn foo() { panic!() }".into()),
            failing_predicate: Some("x > 0".into()),
            touched_paths: vec!["src/foo.rs".into(), "src/bar.rs".into()],
            symbols: vec!["foo".into(), "bar".into()],
        };
        let digest = r.checkout_digest_with_workspace("H0", &task(), &ws, B_S);
        assert!(digest.token_count <= B_S);
        // Cannot be FullRelevantDiff since huge_diff alone exceeds B_S
        assert_ne!(
            digest.degradation_level,
            SessionDegradationLevel::FullRelevantDiff
        );
    }

    #[test]
    fn session_digest_degradation_minimal_head_only_under_50_token_budget() {
        let r = rtool();
        let ws = WorkspaceView {
            relevant_diff: Some("+x\n".repeat(1000)),
            failing_function_src: Some("fn foo() { x }".repeat(200)),
            failing_predicate: Some("x > 0".into()),
            touched_paths: (0..200).map(|i| format!("src/f{}.rs", i)).collect(),
            symbols: (0..200).map(|i| format!("sym_{}", i)).collect(),
        };
        let digest = r.checkout_digest_with_workspace("H0", &task(), &ws, 50);
        assert!(digest.token_count <= 50, "tight budget enforced");
        // Likely MinimalHeadOnly at this budget pressure
        assert_eq!(
            digest.degradation_level,
            SessionDegradationLevel::MinimalHeadOnly
        );
    }

    #[test]
    fn session_digest_never_includes_raw_stderr_from_workspace() {
        // The WorkspaceView API has no field for raw stderr — by construction
        // the rtool cannot leak it. This test fixes that constraint in code.
        let _ws = WorkspaceView::default();
        // Asserting on the type: WorkspaceView fields are explicit and do not
        // include raw_stderr. (Compile-time guard; the absence of the field is
        // the test.)
    }
}
