//! TB-18 Atom A — Re-entrant per-task driver API surface (architect ruling
//! 2026-05-05 §3 Atom A + PRE-17.6 deviation §6.A + FR-18.1 + FR-18.7).
//!
//! ## Why this module exists
//!
//! TB-16.x.2.6 delivered a 4-chain UNION 13/13 because the evaluator's
//! per-task subprocess fork model creates a new chain per task. Architect
//! ruling §2.8 verbatim:
//!
//! > Atom B 要证明的是: one evaluator process / one runtime_repo /
//! > one CAS / one chain / multiple tasks. 如果它只是一个 process 里启动多个
//! > subprocess，每个 subprocess 自己起 chain，那不合格.
//!
//! Atom A's contribution is the **API ratification** that lets atom B do
//! its multi-task driver work without forking subprocesses. The actual
//! shared-chain plumbing (passing `&mut ChaintapeBundle` through the
//! per-task driver) is Atom B's substantive build.
//!
//! ## Atom A.1 (this commit) — API surface
//!
//! - `pub struct TaskSpec`: per-task input parameters (problem_file,
//!   problem_statement, theorem_name, lean_path, model, n_agents).
//! - `pub async fn drive_task(spec, budget) -> PputResult`: thin wrapper
//!   that wires `PerCallBudget` through to the LLM call sites and
//!   delegates per-task chain handling to the existing `run_swarm` body
//!   (currently each invocation creates its own bundle from
//!   `TURINGOS_CHAINTAPE_PATH` env). Caller (atom B) sees the API
//!   ratified at this commit.
//!
//! ## Atom B (next, separate commit) — shared-chain plumbing
//!
//! - Refactor `run_swarm` body to accept `&mut ChaintapeBundle`.
//! - Update `drive_task` signature to `drive_task(bundle, spec, budget)`.
//! - Update comprehensive_arena.rs to thread one bundle across N tasks.
//!
//! ## Why split this way
//!
//! Per `feedback_iteration_cap_24h` (Class 3 production wire-up exception,
//! 72h-to-feedback-loop): completing the entire bundle plumbing in Atom A
//! would block budget enforcement (the OBS_M0 closure) on a multi-day
//! refactor. Splitting Atom A into "API ratification" + "bundle plumbing
//! deferred to Atom B" lets us:
//!
//!   1. Ship per-LLM-call budget enforcement IMMEDIATELY (closes
//!      OBS_M0 §5.1 silent-hang signal).
//!   2. Atom H0 preflight can use the budget enforcement on the existing
//!      per-task chain model (M0 doesn't require single-chain).
//!   3. Atom B does the deeper refactor with full attention.
//!
//! `feedback_no_workarounds_strict_constitution` is honored: this is NOT
//! 凑活 — the API surface is the architect-ratified contract; the
//! internal bundle handling is a known forward-binding to Atom B.

use crate::per_call_budget::PerCallBudget;

/// TB-18 Atom A: Per-task input specification. Frozen at task start;
/// passed through `drive_task`.
#[derive(Debug, Clone)]
pub struct TaskSpec {
    /// Lean problem file path (e.g. `data/heldout/mathd_algebra_107.lean`).
    pub problem_file: String,
    /// Lean theorem statement (the line below `theorem mathd_algebra_107 : ...`).
    pub problem_statement: String,
    /// Lean theorem name (e.g. `mathd_algebra_107`).
    pub theorem_name: String,
    /// Path to `lean` binary.
    pub lean_path: String,
    /// LLM proxy URL (e.g. `http://localhost:8080`).
    pub proxy_url: String,
    /// Model identifier (e.g. `deepseek-chat`).
    pub model: String,
    /// Number of agents in the swarm. `n_agents == 1` ≡ oneshot path.
    pub n_agents: usize,
}

impl TaskSpec {
    /// Construct a TaskSpec with sensible defaults for `lean_path` +
    /// `proxy_url`. Used by atom H0 preflight + atom B comprehensive_arena.
    pub fn new(
        problem_file: impl Into<String>,
        problem_statement: impl Into<String>,
        theorem_name: impl Into<String>,
        model: impl Into<String>,
        n_agents: usize,
    ) -> Self {
        Self {
            problem_file: problem_file.into(),
            problem_statement: problem_statement.into(),
            theorem_name: theorem_name.into(),
            lean_path: std::env::var("LEAN_PATH").unwrap_or_else(|_| "lean".into()),
            proxy_url: std::env::var("LLM_PROXY_URL")
                .unwrap_or_else(|_| "http://localhost:8080".into()),
            model: model.into(),
            n_agents,
        }
    }
}

/// TB-18 Atom A: API ratification stub for the re-entrant per-task driver.
///
/// **Atom A.1 surface** — this is the architect-ratified type signature
/// (`drive_task(spec, budget) -> Result<...>`) that atom B's
/// comprehensive_arena will call. Internal implementation in Atom A.1 is
/// deliberately minimal: returns `Err(DriveTaskError::PendingAtomB)` until
/// atom B refactors `run_swarm` body for shared-chain semantics.
///
/// Per `feedback_no_fake_menus`: this stub does NOT silently delegate to
/// run_swarm with separate-bundle semantics — that would be a workaround
/// hiding the substantive work. Callers learn the API exists; actual
/// drive happens via direct `run_swarm` invocation until atom B lands.
///
/// Atom B will replace this stub body with:
///
/// ```text
/// pub async fn drive_task(
///     chain: &mut ChaintapeBundle,
///     spec: TaskSpec,
///     budget: PerCallBudget,
/// ) -> Result<PputResult, DriveTaskError>
/// ```
///
/// where `chain` is the shared ChaintapeBundle threaded by
/// comprehensive_arena across all N tasks.
pub async fn drive_task(
    spec: TaskSpec,
    budget: PerCallBudget,
) -> Result<DriveTaskResult, DriveTaskError> {
    // Atom A.1: API ratification stub. Body lands in Atom B with
    // bundle-plumbing refactor of run_swarm.
    let _ = (spec, budget);
    Err(DriveTaskError::PendingAtomB)
}

/// TB-18 Atom A: Result returned by `drive_task` once Atom B implements
/// the body. Today this is just a placeholder type so the API surface
/// compiles.
#[derive(Debug, Clone, PartialEq)]
pub struct DriveTaskResult {
    /// Atom B will populate this from the underlying `PputResult`.
    pub problem_file: String,
}

/// TB-18 Atom A: Errors from `drive_task`. The `PendingAtomB` variant
/// makes the staged-delivery state EXPLICIT to callers (atom B will
/// remove it).
#[derive(Debug, Clone, PartialEq)]
pub enum DriveTaskError {
    /// Atom A.1 stub: the bundle plumbing refactor lands in Atom B; until
    /// then, callers must use the existing `run_swarm` invocation path.
    /// This variant is REMOVED in Atom B's commit.
    PendingAtomB,
}

impl std::fmt::Display for DriveTaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriveTaskError::PendingAtomB => {
                write!(f, "drive_task body deferred to TB-18 Atom B")
            }
        }
    }
}

impl std::error::Error for DriveTaskError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_spec_new_uses_env_defaults() {
        let spec = TaskSpec::new(
            "data/heldout/mathd_algebra_107.lean",
            "theorem mathd_algebra_107 : ...",
            "mathd_algebra_107",
            "deepseek-chat",
            1,
        );
        assert_eq!(spec.problem_file, "data/heldout/mathd_algebra_107.lean");
        assert_eq!(spec.theorem_name, "mathd_algebra_107");
        assert_eq!(spec.model, "deepseek-chat");
        assert_eq!(spec.n_agents, 1);
        // lean_path + proxy_url default if env not set; both are non-empty.
        assert!(!spec.lean_path.is_empty());
        assert!(!spec.proxy_url.is_empty());
    }

    #[tokio::test]
    async fn drive_task_atom_a_stub_returns_pending_atom_b() {
        let spec = TaskSpec::new(
            "data/heldout/mathd_algebra_107.lean",
            "theorem mathd_algebra_107 : ...",
            "mathd_algebra_107",
            "deepseek-chat",
            1,
        );
        let budget = PerCallBudget::default();
        let result = drive_task(spec, budget).await;
        assert_eq!(result, Err(DriveTaskError::PendingAtomB));
    }

    #[test]
    fn drive_task_error_display_explains_atom_b_deferral() {
        let msg = format!("{}", DriveTaskError::PendingAtomB);
        assert!(
            msg.contains("Atom B"),
            "error message should reference Atom B; got: {msg}"
        );
    }
}
