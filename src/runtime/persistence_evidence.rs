//! TB-G G1.2-5 (Option B+ orchestration ruling 2026-05-11; binding directive
//! `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md` §3.4 +
//! charter §1 Module G1 G1.2-5): persistence-evidence binding library.
//!
//! Closes the architect-named gap: G1.1's 3-subprocess mini-smoke proved
//! that ChainTape resume **works**; G1.2 must prove it **persists agent
//! state** across task boundaries. Without an explicit binding for the
//! six architect-named persisted fields, "dashboard merely stitches
//! independent runs together" is the failure mode the architect §3.4
//! warned against.
//!
//! Charter §0 kill_criteria_tested #1 verbatim:
//!
//! > If post-G-Phase batch evidence on the 9-problem set shows
//! > per-problem genesis reset (balances reset, positions cleared,
//! > reputation zeroed) between problems, reject — G1 ship-gate
//! > violation.
//!
//! Six fields per charter §1 G1.2-5 ship-gate row:
//!  1. balances non-flat
//!  2. positions carry
//!  3. reputation accumulate
//!  4. PnL non-zero
//!  5. autopsy not reset
//!  6. model identity stable
//!
//! Each field is classified `Witnessed | Empty | Reset`:
//!  - `Witnessed` — field shows the expected cross-task persistence
//!    signature (non-flat / non-empty / monotone-add).
//!  - `Empty` — clean-negative permitted per
//!    `feedback_clean_negative_norm`; reported with a brief reason.
//!  - `Reset` — fail-closed: field was non-empty at some task boundary
//!    but reset to default at a later boundary (genesis-equivalent).
//!    This is the architect §3.4 + kill-criterion #1 condition.
//!
//! Inputs:
//!  - `initial_q` — QState seed for the batch (typically
//!    `QState::genesis()` plus any preseed; for the in-process test it
//!    comes from `genesis_with_balances`).
//!  - `task_end_snapshots` — `Vec<QState>` taken **after** each task's
//!    last accepted L4 entry (one snapshot per `TaskContinuationEntry`).
//!    In-process tests clone `Sequencer::q_snapshot()` after each
//!    `bundle.shutdown().await`; the forward orchestrator (G1.2-6/7)
//!    will derive these by `replay_full_transition` over the chain
//!    entries from genesis up to each task's `end_chain_length`.
//!  - `manifest` — the `BatchContinuationManifest` written by the
//!    orchestrator (G1.2-3).
//!
//! Output: `PersistenceBindingReport` with six `FieldVerdict`s plus a
//! per-task summary trace. The CI gate is `report.is_passing()` — every
//! verdict ∈ {Witnessed, Empty}.
//!
//! FC-trace: FC2-Boot (cross-task continuity binding) + FC3-Markov
//! (derived view from ChainTape + CAS; never LLM self-report).

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::system_keypair::{PinnedSystemPubkeys, SystemEpoch, SystemPublicKey};
use crate::bottom_white::ledger::transition_ledger::{
    replay_full_transition, Git2LedgerWriter, LedgerEntry, LedgerWriter,
};
use crate::bottom_white::tools::registry::ToolRegistry;
use crate::top_white::predicates::registry::PredicateRegistry;
use crate::state::q_state::QState;

use super::batch_continuation_manifest::BatchContinuationManifest;

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-5 2026-05-11; Option B+ §3.4):
/// per-field verdict. Three variants — `Witnessed` is the positive
/// witness, `Empty` is the architect-permitted clean-negative, `Reset`
/// is the kill-criterion violation.
/// Constitutional Justification:
/// `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md` §3.4 +
/// `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md` §0
/// kill_criteria_tested #1.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", content = "detail")]
pub enum FieldVerdict {
    Witnessed(String),
    Empty(String),
    Reset(String),
}

impl FieldVerdict {
    /// TRACE_MATRIX § 3 orphan (TB-G G1.2-5 2026-05-11; Option B+ §3.4):
    /// true iff the variant is `Witnessed`. Used by
    /// `PersistenceBindingReport::n_witnessed`.
    pub fn is_witnessed(&self) -> bool {
        matches!(self, Self::Witnessed(_))
    }
    /// TRACE_MATRIX § 3 orphan (TB-G G1.2-5 2026-05-11; Option B+ §3.4):
    /// true iff the variant is `Reset`. Used by
    /// `PersistenceBindingReport::is_passing` as the CI ship-gate
    /// disqualifier.
    pub fn is_reset(&self) -> bool {
        matches!(self, Self::Reset(_))
    }
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-5 2026-05-11; Option B+ §3.4):
/// per-task summary trace row. Carries the four counts the binding
/// inspects so a reader can replay the verdict without re-running the
/// binding.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskBoundaryTrace {
    pub task_index: u64,
    pub problem_id: String,
    pub balances_total_micro: i64,
    pub balances_distinct_agents: usize,
    pub node_positions_count: usize,
    pub reputations_count: usize,
    pub autopsy_event_count: usize,
    pub autopsy_capsule_total: usize,
    pub conditional_collateral_total_micro: i64,
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-5 2026-05-11; Option B+ §3.4):
/// full report. CAS-anchorable as a Generic object with schema_id
/// `persistence_evidence_g1_2_v1` (G1.2-6/7 will wire that anchor).
///
/// G1.2-7 R2 Codex Notes closure (2026-05-12): the auditor-convenience
/// fields `is_passing` and `n_witnessed` are now serialized directly
/// in the report (previously derived via `is_passing()` /
/// `n_witnessed()` methods; visible only via `tb_g_persistence_report`
/// stdout). `#[serde(default)]` preserves back-compat with R2 evidence
/// dirs that pre-date this field addition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PersistenceBindingReport {
    pub schema_version: String,
    pub batch_id: String,
    pub n_tasks: usize,
    #[serde(default)]
    pub is_passing: bool,
    #[serde(default)]
    pub n_witnessed: usize,
    pub balances: FieldVerdict,
    pub positions: FieldVerdict,
    pub reputation: FieldVerdict,
    pub pnl: FieldVerdict,
    pub autopsy: FieldVerdict,
    pub model_identity: FieldVerdict,
    pub per_task: Vec<TaskBoundaryTrace>,
}

impl PersistenceBindingReport {
    /// TRACE_MATRIX § 3 orphan (TB-G G1.2-5 2026-05-11; Option B+ §3.4):
    /// CI ship-gate predicate — every verdict must be `Witnessed` or
    /// `Empty`. Any `Reset` is the architect §3.4 / charter §0
    /// kill_criteria_tested #1 violation.
    pub fn is_passing(&self) -> bool {
        ![
            &self.balances,
            &self.positions,
            &self.reputation,
            &self.pnl,
            &self.autopsy,
            &self.model_identity,
        ]
        .iter()
        .any(|v| v.is_reset())
    }

    /// TRACE_MATRIX § 3 orphan (TB-G G1.2-5 2026-05-11; Option B+ §3.4):
    /// count of fields with a positive persistence witness. The
    /// SG-G1.2-5.6 gate asserts `n_witnessed >= 2` on a real 2-task
    /// batch.
    pub fn n_witnessed(&self) -> usize {
        [
            &self.balances,
            &self.positions,
            &self.reputation,
            &self.pnl,
            &self.autopsy,
            &self.model_identity,
        ]
        .iter()
        .filter(|v| v.is_witnessed())
        .count()
    }
}

fn total_balance_micro(q: &QState) -> i64 {
    q.economic_state_t
        .balances_t
        .0
        .values()
        .map(|c| c.micro_units())
        .fold(0i64, i64::saturating_add)
}

fn total_collateral_micro(q: &QState) -> i64 {
    q.economic_state_t
        .conditional_collateral_t
        .0
        .values()
        .map(|c| c.micro_units())
        .fold(0i64, i64::saturating_add)
}

fn autopsy_capsule_total(q: &QState) -> usize {
    q.economic_state_t
        .agent_autopsies_t
        .0
        .values()
        .map(|v| v.len())
        .sum()
}

fn build_task_trace(
    task_index: u64,
    problem_id: &str,
    q: &QState,
) -> TaskBoundaryTrace {
    TaskBoundaryTrace {
        task_index,
        problem_id: problem_id.to_string(),
        balances_total_micro: total_balance_micro(q),
        balances_distinct_agents: q.economic_state_t.balances_t.0.len(),
        node_positions_count: q.economic_state_t.node_positions_t.0.len(),
        reputations_count: q.economic_state_t.reputations_t.0.len(),
        autopsy_event_count: q.economic_state_t.agent_autopsies_t.0.len(),
        autopsy_capsule_total: autopsy_capsule_total(q),
        conditional_collateral_total_micro: total_collateral_micro(q),
    }
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-5 2026-05-11; Option B+ §3.4):
/// binding constructor. Classifies each of the six architect-required
/// persisted fields against the in-process or replay-derived `QState`
/// snapshots taken at each task boundary.
///
/// Reset semantics (architect §3.4 + charter §0 kill_criteria_tested #1):
///  - `balances`: Reset if any `task_end_snapshots[k].balances_t` is
///    empty AND a prior snapshot was non-empty.
///  - `positions`: Reset if any snapshot's `node_positions_t` is empty
///    AND a prior snapshot was non-empty.
///  - `reputation`: Reset if any snapshot's `reputations_t` is empty
///    AND a prior snapshot was non-empty.
///  - `pnl`: Reset if `(balances_total + collateral_total)` ever drops
///    back to `initial.balances_total + initial.collateral_total` after
///    a non-zero delta (i.e. the chain "unwound" its economic effect).
///  - `autopsy`: Reset if `autopsy_capsule_total` ever decreases
///    between consecutive snapshots (autopsy is monotone-add only).
///  - `model_identity`: Reset if `manifest.model` is empty AND the
///    batch has executed tasks (a "real" batch must name its model).
pub fn bind_persistence(
    initial_q: &QState,
    task_end_snapshots: &[QState],
    manifest: &BatchContinuationManifest,
) -> PersistenceBindingReport {
    let per_task: Vec<TaskBoundaryTrace> = manifest
        .tasks
        .iter()
        .zip(task_end_snapshots.iter())
        .map(|(entry, q)| build_task_trace(entry.task_index, &entry.problem_id, q))
        .collect();

    let initial_trace = build_task_trace(u64::MAX, "<initial>", initial_q);

    let balances = classify_balances(&initial_trace, &per_task);
    let positions = classify_positions(&initial_trace, &per_task);
    let reputation = classify_reputation(&initial_trace, &per_task);
    let pnl = classify_pnl(&initial_trace, &per_task);
    let autopsy = classify_autopsy(&initial_trace, &per_task);
    let model_identity = classify_model_identity(manifest, &per_task);

    let verdicts = [&balances, &positions, &reputation, &pnl, &autopsy, &model_identity];
    let is_passing = !verdicts.iter().any(|v| v.is_reset());
    let n_witnessed = verdicts.iter().filter(|v| v.is_witnessed()).count();

    PersistenceBindingReport {
        schema_version: "persistence_evidence_g1_2_v1".to_string(),
        batch_id: manifest.batch_id.clone(),
        n_tasks: manifest.tasks.len(),
        is_passing,
        n_witnessed,
        balances,
        positions,
        reputation,
        pnl,
        autopsy,
        model_identity,
        per_task,
    }
}

fn classify_balances(
    initial: &TaskBoundaryTrace,
    per_task: &[TaskBoundaryTrace],
) -> FieldVerdict {
    if per_task.is_empty() {
        return FieldVerdict::Empty(
            "no task end snapshots (batch contains zero tasks)".into(),
        );
    }
    let mut saw_non_empty = initial.balances_distinct_agents > 0;
    for (k, t) in per_task.iter().enumerate() {
        if t.balances_distinct_agents == 0 && saw_non_empty {
            return FieldVerdict::Reset(format!(
                "task[{k}] (problem={problem}) balances_t empty after a \
                 prior non-empty snapshot (kill-criterion #1)",
                problem = t.problem_id
            ));
        }
        if t.balances_distinct_agents > 0 {
            saw_non_empty = true;
        }
    }
    let final_total = per_task.last().map(|t| t.balances_total_micro).unwrap_or(0);
    if final_total != initial.balances_total_micro
        || per_task
            .last()
            .map(|t| t.balances_distinct_agents != initial.balances_distinct_agents)
            .unwrap_or(false)
    {
        FieldVerdict::Witnessed(format!(
            "balances_total_micro {initial_total} → {final_total} across {n} tasks; \
             distinct_agents {initial_n} → {final_n}",
            initial_total = initial.balances_total_micro,
            n = per_task.len(),
            initial_n = initial.balances_distinct_agents,
            final_n = per_task
                .last()
                .map(|t| t.balances_distinct_agents)
                .unwrap_or(0),
        ))
    } else {
        FieldVerdict::Empty(
            "balances_total + distinct_agents unchanged across batch".into(),
        )
    }
}

fn classify_positions(
    initial: &TaskBoundaryTrace,
    per_task: &[TaskBoundaryTrace],
) -> FieldVerdict {
    if per_task.is_empty() {
        return FieldVerdict::Empty("no task end snapshots".into());
    }
    let mut saw_non_empty = initial.node_positions_count > 0;
    for (k, t) in per_task.iter().enumerate() {
        if t.node_positions_count == 0 && saw_non_empty {
            return FieldVerdict::Reset(format!(
                "task[{k}] (problem={problem}) node_positions_t empty after \
                 a prior non-empty snapshot (kill-criterion #1)",
                problem = t.problem_id
            ));
        }
        if t.node_positions_count > 0 {
            saw_non_empty = true;
        }
    }
    let final_n = per_task
        .last()
        .map(|t| t.node_positions_count)
        .unwrap_or(0);
    if final_n > 0 {
        FieldVerdict::Witnessed(format!(
            "node_positions_t count {initial_n} → {final_n} across {n} tasks",
            initial_n = initial.node_positions_count,
            n = per_task.len(),
        ))
    } else {
        FieldVerdict::Empty(
            "no node_positions written by this batch (low market activity)".into(),
        )
    }
}

fn classify_reputation(
    initial: &TaskBoundaryTrace,
    per_task: &[TaskBoundaryTrace],
) -> FieldVerdict {
    if per_task.is_empty() {
        return FieldVerdict::Empty("no task end snapshots".into());
    }
    let mut saw_non_empty = initial.reputations_count > 0;
    for (k, t) in per_task.iter().enumerate() {
        if t.reputations_count == 0 && saw_non_empty {
            return FieldVerdict::Reset(format!(
                "task[{k}] (problem={problem}) reputations_t empty after a \
                 prior non-empty snapshot (kill-criterion #1)",
                problem = t.problem_id
            ));
        }
        if t.reputations_count > 0 {
            saw_non_empty = true;
        }
    }
    let final_n = per_task
        .last()
        .map(|t| t.reputations_count)
        .unwrap_or(0);
    if final_n > initial.reputations_count {
        FieldVerdict::Witnessed(format!(
            "reputations_t count {initial_n} → {final_n}",
            initial_n = initial.reputations_count,
        ))
    } else {
        FieldVerdict::Empty(
            "no reputation entries accumulated by this batch (no verify cycle)".into(),
        )
    }
}

fn classify_pnl(
    initial: &TaskBoundaryTrace,
    per_task: &[TaskBoundaryTrace],
) -> FieldVerdict {
    if per_task.is_empty() {
        return FieldVerdict::Empty("no task end snapshots".into());
    }
    let initial_econ = initial
        .balances_total_micro
        .saturating_add(initial.conditional_collateral_total_micro);
    let mut peak_delta: i64 = 0;
    let mut final_delta: i64 = 0;
    for t in per_task.iter() {
        let econ = t
            .balances_total_micro
            .saturating_add(t.conditional_collateral_total_micro);
        let delta = econ.saturating_sub(initial_econ);
        if delta.abs() > peak_delta.abs() {
            peak_delta = delta;
        }
        final_delta = delta;
    }
    if peak_delta != 0 && final_delta == 0 {
        return FieldVerdict::Reset(format!(
            "pnl peak_delta_micro={peak_delta} but final_delta_micro=0 — \
             economic effect unwound (kill-criterion #1)"
        ));
    }
    if final_delta != 0 {
        FieldVerdict::Witnessed(format!(
            "pnl final_delta_micro={final_delta} (balances+collateral vs initial)"
        ))
    } else {
        FieldVerdict::Empty("pnl total economic balance unchanged".into())
    }
}

fn classify_autopsy(
    initial: &TaskBoundaryTrace,
    per_task: &[TaskBoundaryTrace],
) -> FieldVerdict {
    if per_task.is_empty() {
        return FieldVerdict::Empty("no task end snapshots".into());
    }
    let mut prev_total = initial.autopsy_capsule_total;
    let mut saw_growth = false;
    for (k, t) in per_task.iter().enumerate() {
        if t.autopsy_capsule_total < prev_total {
            return FieldVerdict::Reset(format!(
                "task[{k}] (problem={problem}) autopsy_capsule_total \
                 {prev} → {curr} — monotonicity violated (kill-criterion #1)",
                problem = t.problem_id,
                prev = prev_total,
                curr = t.autopsy_capsule_total,
            ));
        }
        if t.autopsy_capsule_total > prev_total {
            saw_growth = true;
        }
        prev_total = t.autopsy_capsule_total;
    }
    if saw_growth {
        FieldVerdict::Witnessed(format!(
            "autopsy_capsule_total {initial_n} → {final_n} (monotone-add)",
            initial_n = initial.autopsy_capsule_total,
            final_n = prev_total,
        ))
    } else {
        FieldVerdict::Empty(
            "no autopsy capsules written by this batch (no event resolutions)".into(),
        )
    }
}

fn classify_model_identity(
    manifest: &BatchContinuationManifest,
    per_task: &[TaskBoundaryTrace],
) -> FieldVerdict {
    if per_task.is_empty() {
        return FieldVerdict::Empty("no tasks executed".into());
    }
    if manifest.model.trim().is_empty() {
        return FieldVerdict::Reset(
            "manifest.model is empty for a non-empty batch — orchestrator \
             must record model identity (kill-criterion #1)"
                .into(),
        );
    }
    FieldVerdict::Witnessed(format!(
        "manifest.model={model} stable across {n} tasks",
        model = manifest.model,
        n = per_task.len(),
    ))
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-5 2026-05-11; Option B+ §3.4):
/// `replay_task_end_snapshots_from_disk` — replays the on-disk
/// `runtime_repo` + `cas` for each task boundary recorded in
/// `manifest.tasks` and returns `(initial_q, per_task_end_q)` ready
/// for `bind_persistence`. Used by the `tb_g_persistence_report`
/// binary (G1.2-6/7 evidence enricher; Codex Q6 closure).
///
/// Reads `pinned_pubkeys.json` + `initial_q_state.json` via the same
/// FC2-Boot semantics as `verify_chaintape`. For each task k, slices
/// the chain entries to `[0..tasks[k].end_chain_length]` and re-runs
/// `replay_full_transition` to obtain the post-task `QState`. This
/// is O(N²) in worst case (one full replay per boundary) but
/// negligible for the G1.2 batch sizes (3-9 tasks).
pub fn replay_task_end_snapshots_from_disk(
    runtime_repo_path: &Path,
    cas_path: &Path,
    manifest: &BatchContinuationManifest,
) -> Result<(QState, Vec<QState>), ReplaySnapshotError> {
    // pinned pubkeys
    let pin_path = runtime_repo_path.join("pinned_pubkeys.json");
    let pin_json = std::fs::read_to_string(&pin_path)
        .map_err(|e| ReplaySnapshotError::Io(format!("read {pin_path:?}: {e}")))?;
    let pin_manifest: crate::runtime::PinnedPubkeyManifest = serde_json::from_str(&pin_json)
        .map_err(|e| ReplaySnapshotError::Parse(format!("pinned_pubkeys: {e}")))?;
    let mut pinned = PinnedSystemPubkeys::new();
    for entry in &pin_manifest.pubkeys {
        let bytes = decode_hex_32(&entry.pubkey_hex)?;
        let pubkey = SystemPublicKey::from_bytes(bytes);
        pinned.insert(SystemEpoch::new(entry.epoch), pubkey);
    }

    // initial QState (fall back to genesis if file absent — matches verify_chaintape)
    let initial_q_path = runtime_repo_path.join("initial_q_state.json");
    let initial_q = if initial_q_path.exists() {
        let s = std::fs::read_to_string(&initial_q_path)
            .map_err(|e| ReplaySnapshotError::Io(format!("read initial_q: {e}")))?;
        serde_json::from_str::<QState>(&s)
            .map_err(|e| ReplaySnapshotError::Parse(format!("initial_q_state: {e}")))?
    } else {
        QState::genesis()
    };

    // open ledger writer + CAS
    let writer = Git2LedgerWriter::open(runtime_repo_path)
        .map_err(|e| ReplaySnapshotError::Io(format!("open ledger: {e}")))?;
    let chain_len = writer.len();
    let cas_store = CasStore::open(cas_path)
        .map_err(|e| ReplaySnapshotError::Io(format!("open cas: {e}")))?;

    // pull entries [1..=chain_len] once
    let entries: Vec<LedgerEntry> = (1..=chain_len)
        .map(|t| writer.read_at(t))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| ReplaySnapshotError::Io(format!("read entries: {e}")))?;

    let predicate_registry = PredicateRegistry::new();
    let tool_registry = ToolRegistry::new();

    let mut per_task: Vec<QState> = Vec::with_capacity(manifest.tasks.len());
    for task in &manifest.tasks {
        let end_idx = task.end_chain_length as usize;
        if end_idx > entries.len() {
            return Err(ReplaySnapshotError::ManifestExceedsChain {
                task_index: task.task_index,
                end_chain_length: task.end_chain_length,
                actual_chain_length: chain_len,
            });
        }
        let prefix = &entries[..end_idx];
        let q = replay_full_transition(
            &initial_q,
            prefix,
            &cas_store,
            &pinned,
            &predicate_registry,
            &tool_registry,
        )
        .map_err(|e| ReplaySnapshotError::Replay {
            task_index: task.task_index,
            detail: format!("{e:?}"),
        })?;
        per_task.push(q);
    }

    Ok((initial_q, per_task))
}

fn decode_hex_32(hex: &str) -> Result<[u8; 32], ReplaySnapshotError> {
    if hex.len() != 64 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ReplaySnapshotError::Parse(format!(
            "pubkey hex must be 64 lowercase hex chars; got {} chars",
            hex.len()
        )));
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)
            .map_err(|_| ReplaySnapshotError::Parse("bad hex digit".into()))?;
    }
    Ok(out)
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-5 2026-05-11; Option B+ §3.4):
/// error carrier for `replay_task_end_snapshots_from_disk`.
#[derive(Debug)]
pub enum ReplaySnapshotError {
    Io(String),
    Parse(String),
    Replay {
        task_index: u64,
        detail: String,
    },
    ManifestExceedsChain {
        task_index: u64,
        end_chain_length: u64,
        actual_chain_length: u64,
    },
}

impl std::fmt::Display for ReplaySnapshotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(s) => write!(f, "io: {s}"),
            Self::Parse(s) => write!(f, "parse: {s}"),
            Self::Replay {
                task_index,
                detail,
            } => write!(f, "replay task {task_index}: {detail}"),
            Self::ManifestExceedsChain {
                task_index,
                end_chain_length,
                actual_chain_length,
            } => write!(
                f,
                "manifest task[{task_index}].end_chain_length={end_chain_length} \
                 exceeds actual chain length {actual_chain_length}"
            ),
        }
    }
}

impl std::error::Error for ReplaySnapshotError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::batch_continuation_manifest::TaskContinuationEntry;

    fn mk_entry(idx: u64) -> TaskContinuationEntry {
        TaskContinuationEntry {
            task_index: idx,
            problem_id: format!("p{idx}"),
            start_head_t_hex: String::new(),
            end_head_t_hex: String::new(),
            start_chain_length: idx,
            end_chain_length: idx + 1,
            subprocess_command_sha256: String::new(),
            run_summary_cid_hex: None,
            terminal_tx_id: None,
            exit_code: 0,
            started_at_unix_s: 0,
            finished_at_unix_s: 0,
        }
    }

    fn mk_manifest(n: u64, model: &str) -> BatchContinuationManifest {
        BatchContinuationManifest {
            schema_version: "g1_2_v1".into(),
            batch_id: "unit_batch".into(),
            runtime_repo: ".".into(),
            cas_root: ".".into(),
            model: model.into(),
            n_agents: 1,
            initial_head_t_hex: String::new(),
            agent_registry_cid_hex: None,
            system_pubkeys_cid_hex: None,
            model_manifest_cid_hex: None,
            tasks: (0..n).map(mk_entry).collect(),
            terminated_reason: None,
        }
    }

    #[test]
    fn unit_empty_batch_passes_with_all_empty() {
        let m = mk_manifest(0, "");
        let q = QState::genesis();
        let r = bind_persistence(&q, &[], &m);
        assert!(r.is_passing());
        assert_eq!(r.n_witnessed(), 0);
        assert!(matches!(r.balances, FieldVerdict::Empty(_)));
        assert!(matches!(r.model_identity, FieldVerdict::Empty(_)));
    }

    #[test]
    fn unit_non_empty_batch_missing_model_is_reset() {
        let m = mk_manifest(1, "  ");
        let q = QState::genesis();
        let r = bind_persistence(&q, &[q.clone()], &m);
        assert!(matches!(r.model_identity, FieldVerdict::Reset(_)));
        assert!(!r.is_passing());
    }

    #[test]
    fn unit_stable_model_is_witnessed() {
        let m = mk_manifest(2, "deepseek-chat");
        let q = QState::genesis();
        let r = bind_persistence(&q, &[q.clone(), q.clone()], &m);
        assert!(matches!(r.model_identity, FieldVerdict::Witnessed(_)));
    }
}
