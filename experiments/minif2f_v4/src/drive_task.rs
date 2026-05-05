//! TB-18 Atom A + B Phase 3 — Re-entrant per-task driver API surface
//! (architect ruling 2026-05-05 §3 Atom A + Atom B + PRE-17.6 deviation §6.A
//! + FR-18.1 + FR-18.7 + SG-18.6).
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
//! ## Atom A.1 (TB-18 atom A) — API surface (frozen 2026-05-05)
//!
//! - `pub struct TaskSpec`: per-task input parameters.
//! - `pub async fn drive_task(chain, spec, budget) -> Result<...>`:
//!   re-entrant per-task scaffolder.
//! - `PendingAtomB` error variant.
//!
//! ## Atom B Phase 3 (this commit) — substantive body
//!
//! Replaces the Atom A.1 stub with a real implementation that:
//!
//! 1. Mints `task-{theorem_name}` task_id from `spec`
//! 2. Submits a real-signed `TaskOpenTx` (sponsor: `tb7-7-sponsor` by
//!    default — uses the preseed sponsor balance from
//!    `default_pput_preseed_pairs`)
//! 3. Awaits TaskOpen commit (state_root advance; 5s poll budget)
//! 4. Submits a real-signed `EscrowLockTx` (1_000_000 micro-coin escrow
//!    by default; chain-level configurable via env-var future)
//! 5. Awaits EscrowLock commit
//! 6. Returns `DriveTaskResult` with the post-commit state_root_hex so
//!    the caller (`comprehensive_arena`) can use it as
//!    `parent_state_root` when composing subsequent task-specific txs
//!    (WorkTx, VerifyTx, ChallengeTx, TaskBankruptcy, etc.).
//!
//! `DriveTaskError::PendingAtomB` variant is REMOVED in this commit per
//! the Atom A.1 forward-binding promise.
//!
//! ## Why minimum-viable (TaskOpen + EscrowLock only)
//!
//! Per `feedback_chaintape_externalized_proposal`: the chain records what
//! the system externalized via `submit_typed_tx`, not LLM internals.
//! Subsequent task-specific txs (WorkTx for proof attempt, VerifyTx for
//! OMEGA-Confirm, ChallengeTx for dispute, MarketSeed/CompleteSetMint for
//! market lifecycle, TaskBankruptcy for failure-mode close, TaskExpire for
//! timeout) are emitted by `comprehensive_arena` via direct
//! `chain.bus.submit_typed_tx` and `chain.chaintape_bundle.sequencer
//! .emit_system_tx` calls AFTER `drive_task` returns. This keeps
//! `drive_task` as the universal "task scaffold" operation; per-task
//! lifecycle composition is the comprehensive_arena multi-task driver's
//! responsibility (architect §3 Atom B "≥6 engineered Lean tasks").
//!
//! Per `feedback_no_workarounds_strict_constitution`: this is NOT 凑活 —
//! drive_task is the architect-ratified (atom A.1) re-entrant primitive;
//! the lifecycle composition layer (which 13/13 tx kinds get emitted in
//! what order per task) is comprehensive_arena's substantive scope.
//!
//! ## TRACE_MATRIX
//!
//! `FC-trace: FC1-N12 + FC1-N14` — proposal-side oracle scope for the
//! per-task scaffolder; TaskOpen + EscrowLock are the chain-level
//! pipeline-liveness anchors per task.

use crate::chain_runtime::SharedChain;
use crate::per_call_budget::PerCallBudget;

/// TB-18 Atom A: Per-task input specification. Frozen at task start;
/// passed through `drive_task`.
#[derive(Debug, Clone)]
pub struct TaskSpec {
    /// Lean problem file path (e.g. `data/heldout/mathd_algebra_107.lean`).
    pub problem_file: String,
    /// Lean theorem statement (the line below `theorem mathd_algebra_107 : ...`).
    pub problem_statement: String,
    /// Lean theorem name (e.g. `mathd_algebra_107`). Used as the
    /// `task-{theorem_name}` chain task_id by `drive_task`.
    pub theorem_name: String,
    /// Path to `lean` binary.
    pub lean_path: String,
    /// LLM proxy URL (e.g. `http://localhost:8080`).
    pub proxy_url: String,
    /// Model identifier (e.g. `deepseek-chat`).
    pub model: String,
    /// Number of agents in the swarm. `n_agents == 1` ≡ oneshot path.
    pub n_agents: usize,
    /// TB-18 Atom B Phase 3: sponsor agent_id whose preseeded balance
    /// pays for `TaskOpenTx` + `EscrowLockTx`. Defaults to
    /// `tb7-7-sponsor` (10_000_000 micro per
    /// `default_pput_preseed_pairs`). `comprehensive_arena` may override
    /// to differentiate per-task sponsorship across the 6-task set.
    pub sponsor_agent: String,
    /// TB-18 Atom B Phase 3: amount (in micro-coin) to lock in escrow at
    /// task open. Defaults to `1_000_000` (1 token; matches typical
    /// TB-13/14/16 smoke parameters). Sponsor must have ≥ this amount in
    /// the chain's current QState `balances_t` for `EscrowLockTx` to
    /// admit; insufficient balance produces an `L4.E` rejection rather
    /// than a panic.
    pub escrow_amount_micro: i64,
}

impl TaskSpec {
    /// Construct a TaskSpec with sensible defaults for `lean_path` +
    /// `proxy_url` + `sponsor_agent` + `escrow_amount_micro`. Used by
    /// atom H0 preflight + atom B comprehensive_arena.
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
            sponsor_agent: std::env::var("TB18_TASK_SPONSOR")
                .unwrap_or_else(|_| "tb7-7-sponsor".into()),
            escrow_amount_micro: std::env::var("TB18_TASK_ESCROW_MICRO")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1_000_000),
        }
    }
}

/// TB-18 Atom B Phase 3: re-entrant per-task scaffolder.
///
/// **Atom A.1 stub body REPLACED** — this is the substantive
/// implementation per architect §3 Atom B. Submits the universal per-task
/// pair (`TaskOpenTx` + `EscrowLockTx`) against the shared chain, awaits
/// both commits, and returns the post-commit state_root for downstream
/// task-specific tx composition by `comprehensive_arena`.
///
/// **Re-entrancy contract**: `drive_task` is callable N times against the
/// same `&mut SharedChain` to scaffold N tasks in ONE chain. Each call
/// uses `spec.theorem_name` as the per-task identifier suffix
/// (`task-{theorem_name}`); collisions across calls would produce
/// `TaskAlreadyOpen` rejections (caller's responsibility to ensure unique
/// `theorem_name` per call within a chain — `comprehensive_arena`
/// satisfies this trivially because each engineered task has a distinct
/// theorem).
///
/// **Pre-condition**: `chain` was constructed via
/// `SharedChain::from_env(...)` and (when chaintape mode is enabled)
/// includes a populated `agent_keypairs` registry.
///
/// **Post-condition** on success: chain contains 1 new accepted L4
/// `TaskOpenTx` for `task-{theorem_name}` + 1 new accepted L4
/// `EscrowLockTx` locking `spec.escrow_amount_micro` from
/// `spec.sponsor_agent` into the task's escrow.
///
/// **Failure modes**:
/// - `chain.chaintape_bundle = None` (legacy mode) →
///   `Err(DriveTaskError::ChaintapeRequired)` (drive_task is chain-only;
///   legacy WAL_DIR / in-memory paths are not supported).
/// - `chain.agent_keypairs = None` →
///   `Err(DriveTaskError::AgentKeypairsRequired)`.
/// - Real-signature construction fails →
///   `Err(DriveTaskError::SigningFailed { stage, source })`.
/// - tx submit fails or commit-await budget expires →
///   `Err(DriveTaskError::SubmitFailed { stage, .. })`.
///
/// `_budget` is currently unused by the task scaffolder (no LLM calls in
/// this body); kept in the signature per architect-ratified atom A.1
/// contract — Phase 3+ may thread it through if drive_task gains an
/// optional inline LLM path (NOT in scope today).
pub async fn drive_task(
    chain: &mut SharedChain,
    spec: &TaskSpec,
    _budget: PerCallBudget,
) -> Result<DriveTaskResult, DriveTaskError> {
    use turingosv4::runtime::adapter::{
        make_real_escrow_lock_signed_by, make_real_task_open_signed_by,
        tb8_await_state_root_advance,
    };
    use turingosv4::state::q_state::Hash;

    let bundle = chain
        .chaintape_bundle
        .as_ref()
        .ok_or(DriveTaskError::ChaintapeRequired)?;
    let keypairs_arc = chain
        .agent_keypairs
        .as_ref()
        .ok_or(DriveTaskError::AgentKeypairsRequired)?
        .clone();

    let task_id_str = format!("task-{}", spec.theorem_name);
    let pre_open_root = bundle
        .sequencer
        .q_snapshot()
        .map(|q| q.state_root_t)
        .unwrap_or(Hash::ZERO);

    // Build + submit TaskOpen (real-signed by sponsor).
    let task_open = {
        let mut reg = keypairs_arc
            .lock()
            .map_err(|_| DriveTaskError::SigningFailed {
                stage: "task_open_lock",
                detail: "agent_keypairs registry mutex poisoned".into(),
            })?;
        make_real_task_open_signed_by(
            &mut reg,
            &task_id_str,
            &spec.sponsor_agent,
            pre_open_root,
            "tb18-drive-task-open",
            1,
        )
        .map_err(|e| DriveTaskError::SigningFailed {
            stage: "task_open_sign",
            detail: format!("{e:?}"),
        })?
    };
    let task_open_tx_id_str = format!("taskopen-{}-tb18-drive-task-open", task_id_str);
    chain
        .bus
        .submit_typed_tx(task_open)
        .await
        .map_err(|e| DriveTaskError::SubmitFailed {
            stage: "task_open_submit",
            detail: format!("{e:?}"),
        })?;
    let post_open_root = tb8_await_state_root_advance(bundle.sequencer.as_ref(), pre_open_root, 5000)
        .await
        .map_err(|_| DriveTaskError::SubmitFailed {
            stage: "task_open_commit_await",
            detail: "5s state_root advance budget expired".into(),
        })?;

    // Build + submit EscrowLock (real-signed by sponsor).
    let escrow_lock = {
        let mut reg = keypairs_arc
            .lock()
            .map_err(|_| DriveTaskError::SigningFailed {
                stage: "escrow_lock_lock",
                detail: "agent_keypairs registry mutex poisoned".into(),
            })?;
        make_real_escrow_lock_signed_by(
            &mut reg,
            &task_id_str,
            &spec.sponsor_agent,
            spec.escrow_amount_micro,
            post_open_root,
            "tb18-drive-escrow-lock",
            2,
        )
        .map_err(|e| DriveTaskError::SigningFailed {
            stage: "escrow_lock_sign",
            detail: format!("{e:?}"),
        })?
    };
    let escrow_lock_tx_id_str = format!("escrowlock-{}-tb18-drive-escrow-lock", task_id_str);
    chain
        .bus
        .submit_typed_tx(escrow_lock)
        .await
        .map_err(|e| DriveTaskError::SubmitFailed {
            stage: "escrow_lock_submit",
            detail: format!("{e:?}"),
        })?;
    let post_lock_root = tb8_await_state_root_advance(bundle.sequencer.as_ref(), post_open_root, 5000)
        .await
        .map_err(|_| DriveTaskError::SubmitFailed {
            stage: "escrow_lock_commit_await",
            detail: "5s state_root advance budget expired".into(),
        })?;

    Ok(DriveTaskResult {
        problem_file: spec.problem_file.clone(),
        task_id: task_id_str,
        task_open_tx_id: task_open_tx_id_str,
        escrow_lock_tx_id: escrow_lock_tx_id_str,
        post_open_lock_state_root_hex: hex_lower(&post_lock_root),
    })
}

fn hex_lower(h: &turingosv4::state::q_state::Hash) -> String {
    h.0.iter().map(|b| format!("{:02x}", b)).collect()
}

/// TB-18 Atom B Phase 3: result returned by `drive_task`.
///
/// Carries the per-task task_id + tx_ids + post-commit state_root so the
/// `comprehensive_arena` multi-task driver can compose downstream
/// task-specific txs (WorkTx, VerifyTx, etc.) with the correct
/// `parent_state_root` references. The state_root is hex-encoded for
/// log-friendly emission; callers convert back to `Hash` via
/// `Hash::from_hex` when constructing tx envelopes.
#[derive(Debug, Clone, PartialEq)]
pub struct DriveTaskResult {
    /// Mirrors `TaskSpec.problem_file`. Carried so multi-task drivers can
    /// correlate per-task output to source problems.
    pub problem_file: String,
    /// `task-{theorem_name}` — the chain task_id this scaffold created.
    pub task_id: String,
    /// `taskopen-{task_id}-tb18-drive-task-open` — the per-task TaskOpenTx
    /// tx_id (mirrors `make_real_task_open_signed_by` suffix convention).
    pub task_open_tx_id: String,
    /// `escrowlock-{task_id}-tb18-drive-escrow-lock` — the per-task
    /// EscrowLockTx tx_id.
    pub escrow_lock_tx_id: String,
    /// Hex-encoded chain state_root_t observed AFTER both TaskOpen and
    /// EscrowLock commits. Use this as `parent_state_root` for the next
    /// task-specific tx the comprehensive_arena driver emits against
    /// this task.
    pub post_open_lock_state_root_hex: String,
}

/// TB-18 Atom B Phase 3: errors from `drive_task`.
///
/// Atom A.1's `PendingAtomB` variant is REMOVED in this commit per the
/// stub's forward-binding promise.
#[derive(Debug, Clone, PartialEq)]
pub enum DriveTaskError {
    /// `drive_task` requires `TURINGOS_CHAINTAPE_PATH` to be set so
    /// `chain.chaintape_bundle = Some(_)`. Legacy WAL_DIR / in-memory
    /// modes are not supported (no on-disk chain → no architect §2.8
    /// "one runtime_repo + one CAS" semantics).
    ChaintapeRequired,
    /// `drive_task` requires the durable agent keypair registry to have
    /// been initialized at chain construction (TB-9 Atom 2). When this
    /// is `None`, real-signature constructors cannot run.
    AgentKeypairsRequired,
    /// Real-signature construction failed (Ed25519 keypair init, mutex
    /// poison, canonical-digest serialization, etc.).
    SigningFailed {
        /// Pipeline stage where signing failed (e.g. `task_open_sign`,
        /// `escrow_lock_sign`).
        stage: &'static str,
        /// Free-form failure detail; not parsed programmatically.
        detail: String,
    },
    /// `bus.submit_typed_tx` returned `Err` OR the post-submit
    /// state_root advance poll budget expired.
    SubmitFailed {
        /// Pipeline stage where submission failed (e.g. `task_open_submit`,
        /// `task_open_commit_await`).
        stage: &'static str,
        /// Free-form failure detail; not parsed programmatically.
        detail: String,
    },
}

impl std::fmt::Display for DriveTaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriveTaskError::ChaintapeRequired => write!(
                f,
                "drive_task requires TURINGOS_CHAINTAPE_PATH (legacy WAL_DIR / in-memory \
                 modes not supported per architect §2.8 one-chain mandate)"
            ),
            DriveTaskError::AgentKeypairsRequired => write!(
                f,
                "drive_task requires durable agent keypair registry (TB-9 Atom 2; was not \
                 initialized at SharedChain::from_env time — non-chaintape mode?)"
            ),
            DriveTaskError::SigningFailed { stage, detail } => {
                write!(f, "drive_task signing failed at stage={stage}: {detail}")
            }
            DriveTaskError::SubmitFailed { stage, detail } => {
                write!(f, "drive_task submit failed at stage={stage}: {detail}")
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
        assert!(!spec.lean_path.is_empty());
        assert!(!spec.proxy_url.is_empty());
        // Phase 3 additions: sponsor + escrow defaults.
        assert!(!spec.sponsor_agent.is_empty());
        assert!(spec.escrow_amount_micro > 0);
    }

    /// TB-18 Atom B Phase 3: legacy in-memory mode (no TURINGOS_CHAINTAPE_PATH)
    /// returns `Err(ChaintapeRequired)`. Per architect §2.8 one-chain mandate,
    /// drive_task is chaintape-only.
    #[tokio::test]
    async fn drive_task_legacy_mode_returns_chaintape_required() {
        // Skip if any of the chaintape/wal env vars is set — concurrent test
        // race; this test only validates the legacy no-env-set branch.
        if std::env::var("TURINGOS_CHAINTAPE_PATH").is_ok()
            || std::env::var("TURINGOS_CHAINTAPE_PRESEED").is_ok()
            || std::env::var("WAL_DIR").is_ok()
        {
            eprintln!(
                "[drive_task_legacy_test] skipped (concurrent env-var writer in test pool); \
                 ChaintapeRequired-branch verified by smoke probes downstream"
            );
            return;
        }
        let mut chain = SharedChain::from_env("data/heldout/mathd_algebra_107.lean");
        let spec = TaskSpec::new(
            "data/heldout/mathd_algebra_107.lean",
            "theorem mathd_algebra_107 : ...",
            "mathd_algebra_107",
            "deepseek-chat",
            1,
        );
        let budget = PerCallBudget::default();
        let result = drive_task(&mut chain, &spec, budget).await;
        assert_eq!(result, Err(DriveTaskError::ChaintapeRequired));
    }

    #[test]
    fn drive_task_error_display_explains_each_variant() {
        let chaintape_msg = format!("{}", DriveTaskError::ChaintapeRequired);
        assert!(chaintape_msg.contains("TURINGOS_CHAINTAPE_PATH"));

        let keypairs_msg = format!("{}", DriveTaskError::AgentKeypairsRequired);
        assert!(keypairs_msg.contains("agent keypair registry"));

        let signing_msg = format!(
            "{}",
            DriveTaskError::SigningFailed {
                stage: "task_open_sign",
                detail: "test detail".into(),
            }
        );
        assert!(signing_msg.contains("task_open_sign") && signing_msg.contains("test detail"));

        let submit_msg = format!(
            "{}",
            DriveTaskError::SubmitFailed {
                stage: "escrow_lock_submit",
                detail: "test detail".into(),
            }
        );
        assert!(submit_msg.contains("escrow_lock_submit") && submit_msg.contains("test detail"));
    }
}
