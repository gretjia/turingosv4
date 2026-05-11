//! TB-G G1.2-3 (Option B+ orchestration ruling 2026-05-11; binding directive
//! `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md`):
//! batch-orchestration library that turns "subprocess can resume" (G1.1
//! precedent) into "batch is fact-grounded multi-task."
//!
//! Responsibilities:
//! - Build the per-task env-var set the subprocess will see
//! - At each task boundary (task_k>0): snapshot HEAD_t from the shared
//!   runtime_repo, build a `ResumeContract`, call
//!   `resume_preflight::check`, acquire a `ChainTapeLease`
//! - Maintain the incremental `BatchContinuationManifest` skeleton
//!   (G1.2-4 will harden the manifest schema; this module writes the
//!   stable shape today so the orchestrator can ship behind G1.2-3)
//! - On subprocess non-zero exit: halt-and-record (architect §3.5 — no
//!   automatic retry; halt is the safe default)
//!
//! FC-trace: FC2-Boot (chain-continuity safety primitive). The
//! orchestrator is the binding glue between G1.2-1 ResumePreflight,
//! G1.2-2 ChainTapeLease, and the existing per-problem `evaluator`
//! binary. It does NOT execute LLM-Lean cycles itself — those happen
//! inside the spawned `evaluator` subprocess.
//!
//! Constitutional Justification:
//! `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md`
//! §1 (Option B+ canonical orchestration) + §3.1 (ResumePreflight
//! mandate) + §3.2 (ChainTapeLease mandate) + §3.3
//! (BatchContinuationManifest mandate) + §3.5 (halt-and-record on
//! subprocess crash).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use turingosv4::runtime::chain_tape_lease::{self, LeaseGuard};
use turingosv4::runtime::resume_preflight::{
    check as preflight_check, snapshot_head_t, PreflightFailure, PreflightVerdict, ResumeContract,
};

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-3 2026-05-11; Option B+ §1 +
/// §3.1-§3.5): per-batch input shape. Owned by the orchestrator
/// binary; passed by reference to every helper. Constitutional
/// Justification: same OPTION_B_PLUS_RULING.
#[derive(Debug, Clone)]
pub struct BatchSpec {
    pub runtime_repo: PathBuf,
    pub cas_path: PathBuf,
    pub batch_id: String,
    pub model: String,
    pub n_agents: usize,
    pub out_dir: PathBuf,
    pub llm_proxy_url: String,
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-3 2026-05-11; Option B+ §3.3):
/// one row of the incremental manifest. Stable wire shape (Serialize)
/// so G1.2-4 can promote the surrounding container to CAS without
/// changing the row schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskOutcome {
    pub task_index: u64,
    pub problem_id: String,
    pub start_head_t_hex: String,
    pub end_head_t_hex: String,
    pub start_chain_length: u64,
    pub end_chain_length: u64,
    pub exit_code: i32,
    pub started_at_unix_s: i64,
    pub finished_at_unix_s: i64,
    pub terminal_marker: TerminalMarker,
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-3 2026-05-11; Option B+ §3.5):
/// halt-and-record taxonomy. `Completed` is the happy path;
/// `SubprocessCrashed` records the architect-named "subprocess crash
/// mid-task is not the same as a clean failure" mode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum TerminalMarker {
    Completed,
    SubprocessCrashed { exit_code: i32 },
    PreflightRejected { failure: String },
    LeaseUnavailable { reason: String },
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-3 2026-05-11; Option B+ §3.1 +
/// §3.2): boundary-prep helper. For `task_index > 0`: snapshots
/// HEAD_t from `spec.runtime_repo`, builds a `ResumeContract`, calls
/// `resume_preflight::check`, and acquires a `ChainTapeLease`. For
/// `task_index == 0`: returns `BoundaryPrep::FreshGenesis` (no
/// preflight; no lease — task_0 owns the genesis).
///
/// Returns the lease guard so the caller can drop it after the
/// subprocess completes. Constitutional Justification: same
/// OPTION_B_PLUS_RULING.
pub fn prepare_task_boundary(
    spec: &BatchSpec,
    task_index: u64,
    prior_outcome: Option<&TaskOutcome>,
) -> Result<BoundaryPrep, BoundaryError> {
    if task_index == 0 {
        eprintln!(
            "batch_orchestrator: task_index=0 BoundaryPrep::FreshGenesis (no preflight, no lease — task_0 owns genesis)"
        );
        return Ok(BoundaryPrep::FreshGenesis);
    }

    let prior = prior_outcome.ok_or(BoundaryError::PriorOutcomeMissing { task_index })?;

    // Acquire the chain-tape lease BEFORE preflight: the head we
    // snapshot below must be observed under lock, not racy.
    let lease = chain_tape_lease::acquire(
        &spec.runtime_repo,
        &spec.batch_id,
        &prior.end_head_t_hex,
    )
    .map_err(|e| BoundaryError::Lease(e.to_string()))?;
    eprintln!(
        "batch_orchestrator: task_index={task_index} ChainTapeLease ACQUIRED \
         (holder_pid={}, batch_id={}, start_head={})",
        std::process::id(),
        spec.batch_id,
        prior.end_head_t_hex,
    );

    // Re-snapshot under lock for the state-root claim that preflight
    // will verify.
    let (head_hex, state_root_hex, chain_length) =
        snapshot_head_t(&spec.runtime_repo).map_err(BoundaryError::Snapshot)?;

    let contract = ResumeContract {
        runtime_repo: spec.runtime_repo.clone(),
        cas_path: spec.cas_path.clone(),
        expected_head_t_hex: head_hex.clone(),
        expected_state_root_hex: state_root_hex.clone(),
        expected_chain_length: chain_length,
        batch_id: spec.batch_id.clone(),
        task_index,
        agent_pubkeys_path: spec.runtime_repo.join("agent_pubkeys.json"),
        pinned_pubkeys_path: spec.runtime_repo.join("pinned_pubkeys.json"),
        genesis_report_path: spec.runtime_repo.join("genesis_report.json"),
    };

    match preflight_check(&contract) {
        PreflightVerdict::Ok => {
            eprintln!(
                "batch_orchestrator: task_index={task_index} ResumePreflight::Ok \
                 (head={head_hex} state_root={state_root_hex} chain_length={chain_length}) \
                 → BoundaryPrep::Resume"
            );
            Ok(BoundaryPrep::Resume {
                lease,
                start_head_t_hex: head_hex,
                start_chain_length: chain_length,
            })
        }
        PreflightVerdict::Fail { failure } => {
            // Drop the lease before returning the error so the next
            // attempt sees a clean state.
            drop(lease);
            eprintln!(
                "batch_orchestrator: task_index={task_index} ResumePreflight::Fail \
                 → ChainTapeLease released; failure={}",
                format_failure(&failure)
            );
            Err(BoundaryError::Preflight {
                failure: format_failure(&failure),
            })
        }
    }
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-3 2026-05-11; Option B+ §3.1):
/// outcome of `prepare_task_boundary`.
#[derive(Debug)]
pub enum BoundaryPrep {
    /// task_index == 0: fresh genesis path. No lease acquired (no
    /// existing chain to protect); subprocess will create the
    /// runtime_repo + CAS + genesis_report on its own. Caller MUST
    /// NOT set `TURINGOS_CHAINTAPE_RESUME=1` in the subprocess env.
    FreshGenesis,
    /// task_index > 0: resume path. `lease` held until subprocess
    /// completes. `start_head_t_hex` is the head observed under the
    /// lease — caller can record it in `TaskOutcome.start_head_t_hex`.
    Resume {
        lease: LeaseGuard,
        start_head_t_hex: String,
        start_chain_length: u64,
    },
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-3 2026-05-11; Option B+ §3.1 +
/// §3.2 + §3.5): boundary-prep failure modes. Stable wire shape so
/// the manifest can serialize a halt reason.
#[derive(Debug)]
pub enum BoundaryError {
    /// Caller asked for task_index>0 without supplying the prior
    /// outcome — orchestrator wiring bug.
    PriorOutcomeMissing { task_index: u64 },
    /// `chain_tape_lease::acquire` returned an error.
    Lease(String),
    /// `resume_preflight::snapshot_head_t` returned an error.
    Snapshot(String),
    /// `resume_preflight::check` returned Fail.
    Preflight { failure: String },
}

impl std::fmt::Display for BoundaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PriorOutcomeMissing { task_index } => write!(
                f,
                "prior outcome missing for task_index={task_index} (orchestrator bug)"
            ),
            Self::Lease(s) => write!(f, "lease acquire failed: {s}"),
            Self::Snapshot(s) => write!(f, "snapshot head_t failed: {s}"),
            Self::Preflight { failure } => write!(f, "preflight failed: {failure}"),
        }
    }
}

impl std::error::Error for BoundaryError {}

fn format_failure(f: &PreflightFailure) -> String {
    serde_json::to_string(f).unwrap_or_else(|_| format!("{f:?}"))
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-3 2026-05-11; Option B+ §1 +
/// §2 Q2): build the subprocess env-var set. Idempotent / pure
/// function — given the same `spec` + `task_index` + `boundary`
/// returns the same vector. Tests assert the resume flag wiring
/// directly.
pub fn build_subprocess_env(
    spec: &BatchSpec,
    task_index: u64,
    boundary: &BoundaryPrep,
) -> Vec<(String, String)> {
    let mut env = vec![
        (
            "TURINGOS_CHAINTAPE_PATH".to_string(),
            spec.runtime_repo.to_string_lossy().into_owned(),
        ),
        (
            "TURINGOS_CAS_PATH".to_string(),
            spec.cas_path.to_string_lossy().into_owned(),
        ),
        ("ACTIVE_MODEL".to_string(), spec.model.clone()),
        (
            "LLM_PROXY_URL".to_string(),
            spec.llm_proxy_url.clone(),
        ),
        (
            "TURINGOS_RUN_ID".to_string(),
            format!("{}_t{:03}", spec.batch_id, task_index),
        ),
    ];
    // Architect §1: for task_0 the resume flag MUST be unset; for
    // task_k>0 it MUST be "1". Default-deny posture preserved.
    if matches!(boundary, BoundaryPrep::Resume { .. }) {
        env.push(("TURINGOS_CHAINTAPE_RESUME".to_string(), "1".to_string()));
    }
    env
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-3 2026-05-11; Option B+ §3.3 +
/// §3.5): write the per-batch manifest skeleton to disk using the
/// canonical `BatchContinuationManifest` schema (G1.2-4
/// `runtime::batch_continuation_manifest`). Reads parseable by the
/// canonical types so `tb_g_persistence_report` (G1.2-6 Codex Q6
/// closure) can ingest the manifest directly via
/// `serde_json::from_str::<BatchContinuationManifest>`.
pub fn write_manifest_skeleton(
    spec: &BatchSpec,
    outcomes: &[TaskOutcome],
    terminated_reason: Option<&str>,
) -> std::io::Result<PathBuf> {
    use turingosv4::runtime::batch_continuation_manifest::{
        BatchContinuationManifest, TaskContinuationEntry,
    };

    std::fs::create_dir_all(&spec.out_dir)?;
    let path = spec.out_dir.join("BatchContinuationManifest.json");

    let initial_head_t_hex = outcomes
        .first()
        .map(|o| o.start_head_t_hex.clone())
        .unwrap_or_default();

    let tasks: Vec<TaskContinuationEntry> = outcomes
        .iter()
        .map(|o| TaskContinuationEntry {
            task_index: o.task_index,
            problem_id: o.problem_id.clone(),
            start_head_t_hex: o.start_head_t_hex.clone(),
            end_head_t_hex: o.end_head_t_hex.clone(),
            start_chain_length: o.start_chain_length,
            end_chain_length: o.end_chain_length,
            subprocess_command_sha256: String::new(),
            run_summary_cid_hex: None,
            terminal_tx_id: None,
            exit_code: o.exit_code,
            started_at_unix_s: o.started_at_unix_s,
            finished_at_unix_s: o.finished_at_unix_s,
        })
        .collect();

    let manifest = BatchContinuationManifest {
        schema_version: "g1_2_v1".to_string(),
        batch_id: spec.batch_id.clone(),
        runtime_repo: spec.runtime_repo.to_string_lossy().into_owned(),
        cas_root: spec.cas_path.to_string_lossy().into_owned(),
        model: spec.model.clone(),
        n_agents: spec.n_agents,
        initial_head_t_hex,
        agent_registry_cid_hex: None,
        system_pubkeys_cid_hex: None,
        model_manifest_cid_hex: None,
        tasks,
        terminated_reason: terminated_reason.map(|s| s.to_string()),
    };
    let body = serde_json::to_string_pretty(&manifest)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    std::fs::write(&path, body)?;
    Ok(path)
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-3 2026-05-11; Option B+ §3.5):
/// chain-continuity verifier across a `Vec<TaskOutcome>`. Returns Ok
/// iff `tasks[k+1].start_head_t_hex == tasks[k].end_head_t_hex` for
/// every k (and the first task starts from empty / fresh-genesis).
pub fn verify_chain_continuity(outcomes: &[TaskOutcome]) -> Result<(), ContinuityError> {
    for (i, pair) in outcomes.windows(2).enumerate() {
        if pair[1].start_head_t_hex != pair[0].end_head_t_hex {
            return Err(ContinuityError::Gap {
                at_pair_index: i,
                task_k_index: pair[0].task_index,
                task_k_end: pair[0].end_head_t_hex.clone(),
                task_k_plus_1_index: pair[1].task_index,
                task_k_plus_1_start: pair[1].start_head_t_hex.clone(),
            });
        }
    }
    Ok(())
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-3 2026-05-11; Option B+ §3.5):
/// continuity failure carrier.
#[derive(Debug)]
pub enum ContinuityError {
    Gap {
        at_pair_index: usize,
        task_k_index: u64,
        task_k_end: String,
        task_k_plus_1_index: u64,
        task_k_plus_1_start: String,
    },
}

impl std::fmt::Display for ContinuityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gap {
                at_pair_index,
                task_k_index,
                task_k_end,
                task_k_plus_1_index,
                task_k_plus_1_start,
            } => write!(
                f,
                "chain continuity gap at pair index {at_pair_index}: \
                 task[{task_k_index}].end_head={task_k_end} != \
                 task[{task_k_plus_1_index}].start_head={task_k_plus_1_start}"
            ),
        }
    }
}

impl std::error::Error for ContinuityError {}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-3 2026-05-11; Option B+ §3.5):
/// post-subprocess snapshot helper. Reads the chain head + length
/// from the shared `runtime_repo` after the subprocess has exited,
/// returning `(end_head_hex, end_chain_length)`. Empty repo returns
/// `("", 0)`. Used by the orchestrator to populate
/// `TaskOutcome.end_head_t_hex` + `end_chain_length`.
pub fn snapshot_post_task(runtime_repo: &Path) -> Result<(String, u64), String> {
    let (head_hex, _state_root_hex, len) =
        snapshot_head_t(runtime_repo).map_err(|e| format!("snapshot_post_task: {e}"))?;
    Ok((head_hex, len))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn spec_for(tmp: &TempDir, batch_id: &str) -> BatchSpec {
        BatchSpec {
            runtime_repo: tmp.path().join("runtime_repo"),
            cas_path: tmp.path().join("cas"),
            batch_id: batch_id.into(),
            model: "test-model".into(),
            n_agents: 1,
            out_dir: tmp.path().join("out"),
            llm_proxy_url: "http://localhost:0".into(),
        }
    }

    #[test]
    fn unit_task_0_is_fresh_genesis_boundary() {
        let tmp = TempDir::new().expect("tempdir");
        let spec = spec_for(&tmp, "test_fresh");
        let bp = prepare_task_boundary(&spec, 0, None).expect("task_0 ok");
        assert!(matches!(bp, BoundaryPrep::FreshGenesis));
    }

    #[test]
    fn unit_env_contains_resume_flag_for_task_k_gt_0() {
        let tmp = TempDir::new().expect("tempdir");
        let spec = spec_for(&tmp, "test_env");
        let fake_resume = BoundaryPrep::Resume {
            lease: chain_tape_lease::acquire(
                &{
                    std::fs::create_dir_all(spec.runtime_repo.parent().unwrap()).unwrap();
                    std::fs::create_dir_all(&spec.runtime_repo).unwrap();
                    spec.runtime_repo.clone()
                },
                &spec.batch_id,
                "",
            )
            .expect("acquire test lease"),
            start_head_t_hex: "abc".into(),
            start_chain_length: 1,
        };
        let env = build_subprocess_env(&spec, 1, &fake_resume);
        assert!(env.iter().any(|(k, v)| k == "TURINGOS_CHAINTAPE_RESUME" && v == "1"));
        assert!(env.iter().any(|(k, _)| k == "TURINGOS_CHAINTAPE_PATH"));
    }

    #[test]
    fn unit_env_omits_resume_flag_for_task_0() {
        let tmp = TempDir::new().expect("tempdir");
        let spec = spec_for(&tmp, "test_env0");
        let env = build_subprocess_env(&spec, 0, &BoundaryPrep::FreshGenesis);
        assert!(!env.iter().any(|(k, _)| k == "TURINGOS_CHAINTAPE_RESUME"));
    }

    #[test]
    fn unit_continuity_verifier_catches_gap() {
        let outcomes = vec![
            TaskOutcome {
                task_index: 0,
                problem_id: "p0".into(),
                start_head_t_hex: "".into(),
                end_head_t_hex: "head0".into(),
                start_chain_length: 0,
                end_chain_length: 1,
                exit_code: 0,
                started_at_unix_s: 0,
                finished_at_unix_s: 1,
                terminal_marker: TerminalMarker::Completed,
            },
            TaskOutcome {
                task_index: 1,
                problem_id: "p1".into(),
                start_head_t_hex: "head_WRONG".into(),
                end_head_t_hex: "head1".into(),
                start_chain_length: 1,
                end_chain_length: 2,
                exit_code: 0,
                started_at_unix_s: 2,
                finished_at_unix_s: 3,
                terminal_marker: TerminalMarker::Completed,
            },
        ];
        match verify_chain_continuity(&outcomes) {
            Err(ContinuityError::Gap { .. }) => {}
            other => panic!("expected Gap, got {other:?}"),
        }
    }

    #[test]
    fn unit_continuity_verifier_accepts_continuous_chain() {
        let outcomes = vec![
            TaskOutcome {
                task_index: 0,
                problem_id: "p0".into(),
                start_head_t_hex: "".into(),
                end_head_t_hex: "headA".into(),
                start_chain_length: 0,
                end_chain_length: 1,
                exit_code: 0,
                started_at_unix_s: 0,
                finished_at_unix_s: 1,
                terminal_marker: TerminalMarker::Completed,
            },
            TaskOutcome {
                task_index: 1,
                problem_id: "p1".into(),
                start_head_t_hex: "headA".into(),
                end_head_t_hex: "headB".into(),
                start_chain_length: 1,
                end_chain_length: 2,
                exit_code: 0,
                started_at_unix_s: 2,
                finished_at_unix_s: 3,
                terminal_marker: TerminalMarker::Completed,
            },
        ];
        verify_chain_continuity(&outcomes).expect("continuous chain");
    }
}
