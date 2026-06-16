//! LIVE-FC1 — tape-canonical Verified-PPUT (VPPUT) reconstruction.
//!
//! The architect North Star (`PPUT_DRIVEN_FULL_PASS_2026-04-25`) is the
//! held-out **Verified PPUT**. This module reconstructs it FROM THE CANONICAL
//! TAPE ALONE (L4 accepted spine + L4.E rejection chain + CAS payloads) so that
//! efficiency becomes a gate-verifiable, OS-qualifying dimension — not a
//! sidecar JSON dashboard number.
//!
//! ── ARCHITECT PPUT DEFINITION (binding) ───────────────────────────────────
//! For a single task `i`:
//!
//! ```text
//!                  1[ GroundTruth(G_i) = 1 ]
//!     VPPUT_i  =  ───────────────────────────
//!                        C_i  ×  T_i
//! ```
//!
//! where
//!   * `progress_i ∈ {0, 1}` — **ground-truth gated** (Art.I.1). `1` ONLY when a
//!     VERIFIED GOLDEN PATH exists for the task: a `TerminalSummaryTx` with
//!     `run_outcome == OmegaAccepted` AND at least one accepted L4 `WorkTx` for
//!     the task whose `ProposalTelemetry.verification_result_cid` resolves to a
//!     CAS `VerificationResult { verified: true }` (the Lean-oracle witness).
//!     Predicate-pass ALONE is NOT progress without that ground-truth witness.
//!   * `C_i` — ALL token cost across ALL agents, ALL branches, ALL FAILED
//!     proposals, and ALL tool stdout. Reconstructed by summing
//!     `ProposalTelemetry.token_counts.total()` (prompt + completion + **tool**)
//!     over EVERY `WorkTx` for the task on the ACCEPTED L4 spine **and** on the
//!     L4.E rejection chain (failed branches MUST count). `tool_tokens` is part
//!     of `.total()`, so tool stdout is included.
//!   * `T_i` — wall-clock first-read→final-accept span. The canonical tape stores
//!     a monotonic **logical-t** counter (`LedgerEntry.logical_t`), NOT real
//!     milliseconds — wall-clock ms is explicitly NON-reconstructable from chain
//!     bytes (see the `chain_derived_run_facts.rs` module doc: wall time is
//!     non-deterministic across runs even when the chain is identical). We
//!     therefore reconstruct `T_i` as the **logical-tick span** of the task on
//!     L4: `max(logical_t) − min(logical_t) + 1` over all L4 entries that touch
//!     the task. This is the only first-read→final-accept duration that is
//!     byte-deterministic from ChainTape, so it is the canonical `T_i`.
//!
//! ── INTEGER-ONLY (canonical metric) ───────────────────────────────────────
//! The canonical metric is an integer **micro-unit** (reusing the existing
//! `verified_pput_micro: u64` shape from `market_performance_e4.rs`):
//!
//! ```text
//!     verified_pput_micro_i = (1_000_000 × progress_i) / (C_i × T_i)
//! ```
//!
//! computed with integer division and saturating arithmetic — there is NO `f64`
//! anywhere on the canonical path. A derived dashboard `f64` view is allowed
//! ELSEWHERE but is never the canonical value and is not produced here.
//!
//! ── TAPE-CANONICAL (Art.0.2) ──────────────────────────────────────────────
//! Every field is reconstructed from `LoadedTape` (`entries` = L4, `l4e_writer`
//! = L4.E, `cas` = CAS). There is NO sidecar-JSON input. Anything that is not
//! tape-reconstructable cannot enter the metric. ChainTape/CAS win on any
//! conflict.
//!
//! ── SHIELDED (Art.III.4 / Gate H) ─────────────────────────────────────────
//! VPPUT is a METRIC. It MUST NOT leak into any agent prompt. This module has no
//! prompt-builder dependency and is never imported by one; the value is a
//! SYSTEM/auditor witness only. Goodhart shield: do NOT add `verified_pput_micro`
//! to any read view an agent sees.
//!
//! ── OBSERVE-ONLY ──────────────────────────────────────────────────────────
//! Pure read. Takes `&LoadedTape` by shared reference; mutates nothing, advances
//! no head, changes no admission or predicate. Not a source of truth.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::bottom_white::ledger::transition_ledger::{canonical_decode, TxKind};
use crate::runtime::audit_assertions::LoadedTape;
use crate::runtime::proposal_telemetry::read_from_cas as read_proposal_telemetry;
use crate::runtime::verification_result::read_from_cas as read_verification_result;
use crate::state::typed_tx::{RunOutcome, TypedTx};

/// TRACE_MATRIX FC1-N14 + Art.I.1: micro-unit numerator. The canonical metric is
/// `verified_pput_micro = (PPUT_MICRO_SCALE × progress) / (cost × ticks)` — a
/// pure-integer micro-unit (reusing the `verified_pput_micro: u64` shape from
/// `market_performance_e4.rs`). NO `f64` ever touches this path.
pub const PPUT_MICRO_SCALE: u64 = 1_000_000;

/// TRACE_MATRIX FC1-N34: per-task tape-reconstructed VPPUT row.
///
/// Every field is derived PURELY from `LoadedTape` (L4 + L4.E + CAS). Integer
/// only. `progress` is GROUND-TRUTH gated (Art.I.1) — `1` only when a verified
/// golden path exists for the task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskVpput {
    /// TRACE_MATRIX: canonical `WorkTx.task_id` (the on-tape task identity).
    pub task_id: String,
    /// TRACE_MATRIX Art.I.1: `1` iff a VERIFIED GOLDEN PATH exists for the task
    /// (TerminalSummary OmegaAccepted AND a CAS `VerificationResult.verified`).
    /// Ground-truth gated — predicate-pass alone is NOT progress. `0` otherwise.
    pub progress: u64,
    /// TRACE_MATRIX: `C_i` — ALL tokens across accepted L4 AND L4.E-rejected
    /// WorkTx for the task, including tool stdout (`tool_tokens`). Failed
    /// branches counted.
    pub cost_tokens: u64,
    /// TRACE_MATRIX: `T_i` — logical-tick span of the task on L4
    /// (`max(logical_t) − min(logical_t) + 1`); the tape-canonical
    /// first-read→final-accept duration (wall-clock ms is non-reconstructable).
    pub wall_clock_ticks: u64,
    /// TRACE_MATRIX: integer count of WorkTx attempts (accepted + rejected) that
    /// contributed to `cost_tokens`. Bounded discriminator; not a metric leak.
    pub attempt_count: u64,
    /// TRACE_MATRIX: integer count of L4.E-rejected (failed-branch) WorkTx
    /// attempts for the task that the cost sum INCLUDED. Failed branches MUST
    /// count — this field witnesses that they did.
    pub failed_branch_attempt_count: u64,
    /// TRACE_MATRIX Art.0 (integer-only): the CANONICAL metric —
    /// `(PPUT_MICRO_SCALE × progress) / (cost_tokens × wall_clock_ticks)`,
    /// integer division, saturating. `0` when `progress == 0` or the denominator
    /// is `0`. NO `f64`.
    pub verified_pput_micro: u64,
}

impl TaskVpput {
    /// TRACE_MATRIX Art.0 (integer-only): compute the canonical micro-unit from
    /// the reconstructed `(progress, cost, ticks)` with integer math only. A
    /// zero denominator (no tokens or no ticks — i.e. the task never produced a
    /// costed externalized cycle) yields `0`, never a divide-by-zero.
    fn compute_micro(progress: u64, cost_tokens: u64, wall_clock_ticks: u64) -> u64 {
        if progress == 0 {
            return 0;
        }
        let denom = cost_tokens.saturating_mul(wall_clock_ticks);
        if denom == 0 {
            return 0;
        }
        // Integer division — pure u64; NO f64. Saturation is defensive (the
        // numerator is bounded by PPUT_MICRO_SCALE × 1).
        PPUT_MICRO_SCALE.saturating_mul(progress) / denom
    }
}

/// TRACE_MATRIX FC1-N34: the held-out VPPUT reconstruction.
///
/// A SCOPED, SHIELDED, tape-derived projection: per-task `TaskVpput` rows plus a
/// held-out aggregate (`H-VPPUT`) over a caller-supplied held-out task split.
/// The split membership is the held-out TAG; the task identities and all metric
/// inputs come from the tape. Not a source of truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VpputReconstruction {
    /// TRACE_MATRIX: per-task rows, ordered by `task_id` (BTreeMap iteration).
    pub tasks: Vec<TaskVpput>,
    /// TRACE_MATRIX: the held-out split tag — task_ids reserved as the held-out
    /// evaluation set. Membership is the only non-tape input; everything else is
    /// reconstructed. Empty = no held-out split declared.
    pub held_out_task_ids: Vec<String>,
    /// TRACE_MATRIX: integer count of held-out tasks that were actually present
    /// on the tape (held-out tag ∩ tape tasks). Held-out tags with no tape
    /// footprint do NOT enter the aggregate.
    pub held_out_task_count: u64,
    /// TRACE_MATRIX Art.0 (integer-only): the held-out aggregate **H-VPPUT** —
    /// the integer mean of `verified_pput_micro` over the held-out tasks that
    /// appear on the tape: `Σ verified_pput_micro_i / held_out_task_count`.
    /// `0` when the held-out split is empty / absent from the tape. NO `f64`.
    pub h_vpput_micro: u64,
    /// TRACE_MATRIX: integer count of L4 accepted entries on the reconstructed
    /// tape. Integer-only provenance counter.
    pub l4_entry_count: u64,
    /// TRACE_MATRIX: integer count of L4.E rejection rows. Integer-only.
    pub l4e_entry_count: u64,
    /// TRACE_MATRIX: always `true`. This report can never be canonical state, an
    /// admission/predicate input, or an agent-visible read view.
    pub observe_only: bool,
}

impl VpputReconstruction {
    /// TRACE_MATRIX Art.0 (integer-only): the held-out aggregate as the canonical
    /// micro-unit. Exposed so gates can assert on it without re-deriving.
    pub fn h_vpput_micro(&self) -> u64 {
        self.h_vpput_micro
    }

    /// TRACE_MATRIX FC1-N34: count of per-task rows whose `progress == 1` (a
    /// verified golden path exists). Integer comparison only — never an `f64`
    /// success rate on the canonical path.
    pub fn ground_truth_solved_count(&self) -> u64 {
        self.tasks.iter().filter(|t| t.progress == 1).count() as u64
    }

    /// TRACE_MATRIX FC1-N34: look up a per-task row by `task_id` (the on-tape
    /// identity). Read-only accessor over the reconstructed rows.
    pub fn task(&self, task_id: &str) -> Option<&TaskVpput> {
        self.tasks.iter().find(|t| t.task_id == task_id)
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Tape-derived per-task accumulation (pure; L4 + L4.E + CAS only)
// ─────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N34: per-task running accumulation during the single tape
/// pass. Every field is an integer count / boolean witness. No `f64`.
#[derive(Debug, Clone, Default)]
struct TaskAccum {
    cost_tokens: u64,
    attempt_count: u64,
    failed_branch_attempt_count: u64,
    /// Min/max L4 `logical_t` seen for the task → logical-tick span = `T_i`.
    min_logical_t: Option<u64>,
    max_logical_t: Option<u64>,
    /// True iff a `TerminalSummaryTx { run_outcome == OmegaAccepted }` named the
    /// task on the accepted spine.
    omega_terminal: bool,
    /// True iff at least one accepted L4 WorkTx for the task resolved a CAS
    /// `VerificationResult { verified: true }` (the Lean-oracle ground-truth
    /// witness for the verified golden path).
    oracle_verified: bool,
}

impl TaskAccum {
    /// Fold one L4 `logical_t` into the task's logical-tick span.
    fn observe_logical_t(&mut self, logical_t: u64) {
        self.min_logical_t = Some(match self.min_logical_t {
            Some(m) => m.min(logical_t),
            None => logical_t,
        });
        self.max_logical_t = Some(match self.max_logical_t {
            Some(m) => m.max(logical_t),
            None => logical_t,
        });
    }

    /// `T_i` — logical-tick span (`max − min + 1`), the tape-canonical
    /// first-read→final-accept duration. `0` when the task has no L4 footprint.
    fn wall_clock_ticks(&self) -> u64 {
        match (self.min_logical_t, self.max_logical_t) {
            (Some(lo), Some(hi)) => hi.saturating_sub(lo).saturating_add(1),
            _ => 0,
        }
    }

    /// Ground-truth gate (Art.I.1): a verified golden path exists ⇔ the task
    /// terminated `OmegaAccepted` AND a Lean-oracle `VerificationResult.verified`
    /// witnesses it. Predicate-pass alone is NOT enough.
    fn progress(&self) -> u64 {
        if self.omega_terminal && self.oracle_verified {
            1
        } else {
            0
        }
    }
}

/// TRACE_MATRIX FC1-N34: decode the `WorkTx` carried by an accepted L4 entry (or
/// an L4.E record) from CAS. Returns `None` on any CAS/decode miss (the row
/// still proves the attempt happened for counting, but its tokens/task can't be
/// attributed — handled by the caller).
fn decode_work_tx(
    cas: &crate::bottom_white::cas::store::CasStore,
    tx_payload_cid: &crate::bottom_white::cas::schema::Cid,
) -> Option<crate::state::typed_tx::WorkTx> {
    let bytes = cas.get(tx_payload_cid).ok()?;
    match canonical_decode::<TypedTx>(&bytes) {
        Ok(TypedTx::Work(work)) => Some(work),
        _ => None,
    }
}

/// TRACE_MATRIX FC1-N34: sum the token cost of a `WorkTx` from its
/// `ProposalTelemetry.token_counts.total()` (prompt + completion + **tool**).
/// Zero-CID legacy synthetic seeds and CAS misses contribute `0` (no tokens to
/// attribute), exactly like `chain_derived_run_facts.rs`. The boolean reports
/// whether the proposal also carried a verified Lean-oracle witness.
fn work_tx_cost_and_oracle(
    cas: &crate::bottom_white::cas::store::CasStore,
    work: &crate::state::typed_tx::WorkTx,
) -> (u64, bool) {
    if work.proposal_cid.0 == [0u8; 32] {
        return (0, false);
    }
    let Ok(tel) = read_proposal_telemetry(cas, &work.proposal_cid) else {
        return (0, false);
    };
    let cost = tel.token_counts.total();
    let oracle_verified = match tel.verification_result_cid {
        Some(vr_cid) => read_verification_result(cas, &vr_cid)
            .map(|vr| vr.verified)
            .unwrap_or(false),
        None => false,
    };
    (cost, oracle_verified)
}

/// TRACE_MATRIX FC1-N34: walk L4 + L4.E + CAS once and accumulate every task's
/// `(cost, ticks, progress)` inputs. Pure read; no mutation, no head advance.
fn accumulate_tasks(tape: &LoadedTape) -> BTreeMap<String, TaskAccum> {
    let mut by_task: BTreeMap<String, TaskAccum> = BTreeMap::new();

    // ── L4 accepted spine ──────────────────────────────────────────────────
    for entry in &tape.entries {
        let Ok(bytes) = tape.cas.get(&entry.tx_payload_cid) else {
            continue;
        };
        let Ok(typed) = canonical_decode::<TypedTx>(&bytes) else {
            continue;
        };
        match typed {
            TypedTx::Work(work) => {
                let task_id = work.task_id.0.clone();
                let (cost, oracle_verified) = work_tx_cost_and_oracle(&tape.cas, &work);
                let acc = by_task.entry(task_id).or_default();
                acc.cost_tokens = acc.cost_tokens.saturating_add(cost);
                acc.attempt_count = acc.attempt_count.saturating_add(1);
                acc.observe_logical_t(entry.logical_t);
                if oracle_verified {
                    acc.oracle_verified = true;
                }
            }
            TypedTx::TerminalSummary(summary) => {
                let task_id = summary.task_id.0.clone();
                let acc = by_task.entry(task_id).or_default();
                // The terminal is part of the task's lifespan on tape → fold its
                // logical_t into the first-read→final-accept span.
                acc.observe_logical_t(entry.logical_t);
                if summary.run_outcome == RunOutcome::OmegaAccepted {
                    acc.omega_terminal = true;
                }
            }
            _ => {}
        }
    }

    // ── L4.E rejection chain — FAILED BRANCHES MUST COUNT ──────────────────
    // Each rejected WorkTx's tokens still cost real budget; the architect C_i
    // definition counts ALL failed proposals. We decode the rejected payload,
    // attribute its task_id + telemetry tokens, and count it as a failed-branch
    // attempt. Rejections carry no logical_t (submit-side only), so they do NOT
    // extend the L4 logical-tick span — only the accepted spine bounds T_i.
    for rec in tape.l4e_writer.records() {
        if rec.tx_kind != TxKind::Work {
            continue;
        }
        let Some(work) = decode_work_tx(&tape.cas, &rec.tx_payload_cid) else {
            continue;
        };
        let task_id = work.task_id.0.clone();
        let (cost, oracle_verified) = work_tx_cost_and_oracle(&tape.cas, &work);
        let acc = by_task.entry(task_id).or_default();
        acc.cost_tokens = acc.cost_tokens.saturating_add(cost);
        acc.attempt_count = acc.attempt_count.saturating_add(1);
        acc.failed_branch_attempt_count = acc.failed_branch_attempt_count.saturating_add(1);
        // A rejected attempt would not normally carry a verified oracle witness,
        // but if one is structurally present we honor the tape rather than
        // hard-coding an assumption.
        if oracle_verified {
            acc.oracle_verified = true;
        }
    }

    by_task
}

// ─────────────────────────────────────────────────────────────────────────
// Public reconstruction API
// ─────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N34 + Art.I.1: reconstruct per-task VPPUT and the held-out
/// H-VPPUT aggregate PURELY from the loaded tape (L4 + L4.E + CAS) and a
/// held-out task split tag.
///
/// `held_out_task_ids` is the held-out split TAG (e.g. the `task_id` set reserved
/// for held-out evaluation). It is the ONLY non-tape input — every metric value
/// is reconstructed from the tape. A held-out tag with no tape footprint is
/// ignored (it cannot contribute a reconstructed number).
///
/// **Observe-only**, **tape-canonical**, **integer-only**, **shielded** — see
/// the module header. The returned `verified_pput_micro` / `h_vpput_micro` are
/// the CANONICAL metric values (`u64` micro-units); no `f64` is computed.
pub fn reconstruct_vpput_from_tape(
    tape: &LoadedTape,
    held_out_task_ids: &[String],
) -> VpputReconstruction {
    let by_task = accumulate_tasks(tape);

    let tasks: Vec<TaskVpput> = by_task
        .iter()
        .map(|(task_id, acc)| {
            let progress = acc.progress();
            let cost_tokens = acc.cost_tokens;
            let wall_clock_ticks = acc.wall_clock_ticks();
            TaskVpput {
                task_id: task_id.clone(),
                progress,
                cost_tokens,
                wall_clock_ticks,
                attempt_count: acc.attempt_count,
                failed_branch_attempt_count: acc.failed_branch_attempt_count,
                verified_pput_micro: TaskVpput::compute_micro(
                    progress,
                    cost_tokens,
                    wall_clock_ticks,
                ),
            }
        })
        .collect();

    // Held-out aggregate: integer mean of verified_pput_micro over held-out
    // tasks that actually appear on the tape. Pure-integer; NO f64.
    let held_out_set: BTreeSet<&str> = held_out_task_ids.iter().map(String::as_str).collect();
    let held_out_present: Vec<&TaskVpput> = tasks
        .iter()
        .filter(|t| held_out_set.contains(t.task_id.as_str()))
        .collect();
    let held_out_task_count = held_out_present.len() as u64;
    let h_vpput_micro = if held_out_task_count == 0 {
        0
    } else {
        let sum: u64 = held_out_present
            .iter()
            .fold(0u64, |a, t| a.saturating_add(t.verified_pput_micro));
        sum / held_out_task_count
    };

    VpputReconstruction {
        tasks,
        held_out_task_ids: held_out_task_ids.to_vec(),
        held_out_task_count,
        h_vpput_micro,
        l4_entry_count: tape.entries.len() as u64,
        l4e_entry_count: tape.l4e_writer.len() as u64,
        observe_only: true,
    }
}

/// TRACE_MATRIX FC1-N34: render a SHIELDED human/audit summary. Emits ONLY
/// bounded task ids + integer counts + integer micro-unit metric VALUES. This is
/// a SYSTEM/auditor witness surface — it is NOT an agent-visible read view, and
/// MUST NOT be wired into any prompt builder (Art.III.4 / Gate H). No `f64`
/// decimal point appears on any numeric surface.
pub fn render_vpput_summary(report: &VpputReconstruction) -> String {
    let mut out = String::new();
    out.push_str("VPPUT RECONSTRUCTION (observe-only, tape-derived, integer micro-units)\n");
    out.push_str(&format!(
        "  L4 entries: {}  L4.E entries: {}\n",
        report.l4_entry_count, report.l4e_entry_count
    ));
    out.push_str(&format!(
        "  held_out tasks (present on tape): {}  H-VPPUT (micro): {}\n",
        report.held_out_task_count, report.h_vpput_micro
    ));
    out.push_str(&format!(
        "  ground-truth solved tasks: {}\n",
        report.ground_truth_solved_count()
    ));
    out.push_str("  [per-task]\n");
    for t in &report.tasks {
        out.push_str(&format!(
            "    {:<28} progress={} cost_tokens={} ticks={} attempts={} failed={} vppu_micro={}\n",
            t.task_id,
            t.progress,
            t.cost_tokens,
            t.wall_clock_ticks,
            t.attempt_count,
            t.failed_branch_attempt_count,
            t.verified_pput_micro,
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The canonical micro-unit is integer-only and ground-truth gated:
    /// progress=0 → 0 regardless of cost/ticks; progress=1 → integer division.
    #[test]
    fn compute_micro_is_integer_and_ground_truth_gated() {
        // progress 0 → metric 0 even with cheap fast run.
        assert_eq!(TaskVpput::compute_micro(0, 1, 1), 0);
        // progress 1, cost=2, ticks=5 → 1_000_000 / 10 = 100_000.
        assert_eq!(TaskVpput::compute_micro(1, 2, 5), 100_000);
        // progress 1, cost=1, ticks=1 → 1_000_000 (max for a 1-token 1-tick solve).
        assert_eq!(TaskVpput::compute_micro(1, 1, 1), PPUT_MICRO_SCALE);
        // Zero denominator (no costed cycle) → 0, never a panic.
        assert_eq!(TaskVpput::compute_micro(1, 0, 5), 0);
        assert_eq!(TaskVpput::compute_micro(1, 7, 0), 0);
    }

    /// Cheaper / faster verified solve ⇒ strictly larger micro metric (the
    /// efficiency ordering the North Star wants).
    #[test]
    fn cheaper_or_faster_verified_solve_scores_higher() {
        let cheap = TaskVpput::compute_micro(1, 10, 2); // cost*ticks = 20
        let dear = TaskVpput::compute_micro(1, 100, 2); // cost*ticks = 200
        assert!(
            cheap > dear,
            "fewer tokens must score higher: {cheap} !> {dear}"
        );

        let fast = TaskVpput::compute_micro(1, 10, 1); // cost*ticks = 10
        let slow = TaskVpput::compute_micro(1, 10, 10); // cost*ticks = 100
        assert!(
            fast > slow,
            "fewer ticks must score higher: {fast} !> {slow}"
        );
    }

    /// Ground-truth gate: omega-terminal WITHOUT an oracle witness is NOT
    /// progress (predicate-pass alone is not enough); both required.
    #[test]
    fn progress_requires_omega_and_oracle_witness() {
        let mut acc = TaskAccum {
            omega_terminal: true,
            oracle_verified: false,
            ..TaskAccum::default()
        };
        assert_eq!(
            acc.progress(),
            0,
            "omega without oracle witness is not progress"
        );
        acc.oracle_verified = true;
        assert_eq!(
            acc.progress(),
            1,
            "omega + oracle witness is the verified golden path"
        );
        let mut oracle_only = TaskAccum {
            omega_terminal: false,
            oracle_verified: true,
            ..TaskAccum::default()
        };
        assert_eq!(
            oracle_only.progress(),
            0,
            "oracle witness without omega terminal is not progress"
        );
        oracle_only.omega_terminal = true;
        assert_eq!(oracle_only.progress(), 1);
    }

    /// T_i logical-tick span = max − min + 1 over observed L4 logical_t; a
    /// single-entry task spans 1 tick; no entries spans 0.
    #[test]
    fn wall_clock_ticks_is_logical_span() {
        let mut acc = TaskAccum::default();
        assert_eq!(acc.wall_clock_ticks(), 0);
        acc.observe_logical_t(7);
        assert_eq!(acc.wall_clock_ticks(), 1, "single entry spans 1 tick");
        acc.observe_logical_t(11);
        acc.observe_logical_t(9);
        assert_eq!(acc.wall_clock_ticks(), 5, "11 - 7 + 1 = 5");
    }

    /// Held-out aggregate is the integer mean over held-out tasks PRESENT on the
    /// tape; held-out tags absent from the tape are ignored; empty split → 0.
    #[test]
    fn h_vpput_is_integer_mean_over_present_held_out_tasks() {
        let tasks = vec![
            TaskVpput {
                task_id: "held_a".into(),
                progress: 1,
                cost_tokens: 1,
                wall_clock_ticks: 1,
                attempt_count: 1,
                failed_branch_attempt_count: 0,
                verified_pput_micro: 1_000_000,
            },
            TaskVpput {
                task_id: "held_b".into(),
                progress: 1,
                cost_tokens: 2,
                wall_clock_ticks: 1,
                attempt_count: 1,
                failed_branch_attempt_count: 0,
                verified_pput_micro: 500_000,
            },
            TaskVpput {
                task_id: "train_c".into(),
                progress: 1,
                cost_tokens: 1,
                wall_clock_ticks: 1,
                attempt_count: 1,
                failed_branch_attempt_count: 0,
                verified_pput_micro: 1_000_000,
            },
        ];
        // held-out = {held_a, held_b, ghost_d}; ghost_d not on tape → ignored.
        let held = vec![
            "held_a".to_string(),
            "held_b".to_string(),
            "ghost_d".to_string(),
        ];
        let held_set: BTreeSet<&str> = held.iter().map(String::as_str).collect();
        let present: Vec<&TaskVpput> = tasks
            .iter()
            .filter(|t| held_set.contains(t.task_id.as_str()))
            .collect();
        assert_eq!(present.len(), 2, "ghost_d has no tape footprint → excluded");
        let sum: u64 = present.iter().map(|t| t.verified_pput_micro).sum();
        assert_eq!(
            sum / present.len() as u64,
            750_000,
            "(1_000_000 + 500_000)/2"
        );
    }

    /// The shielded render emits status + integer counts + integer micro values
    /// and carries NO raw-diagnostic / decimal-point leak.
    #[test]
    fn render_is_shielded_and_integer_only() {
        let report = VpputReconstruction {
            tasks: vec![TaskVpput {
                task_id: "t1".into(),
                progress: 1,
                cost_tokens: 4,
                wall_clock_ticks: 2,
                attempt_count: 3,
                failed_branch_attempt_count: 2,
                verified_pput_micro: 125_000,
            }],
            held_out_task_ids: vec!["t1".into()],
            held_out_task_count: 1,
            h_vpput_micro: 125_000,
            l4_entry_count: 5,
            l4e_entry_count: 2,
            observe_only: true,
        };
        let s = render_vpput_summary(&report);
        assert!(s.contains("vppu_micro=125000"));
        assert!(s.contains("H-VPPUT (micro): 125000"));
        // No raw-diagnostic markers; no f64 decimal point on any NUMERIC
        // surface (a `<digit>.<digit>` pattern is the f64-leak signal; prose
        // periods are fine).
        assert!(!s.to_lowercase().contains("stderr"));
        assert!(!s.to_lowercase().contains("autopsy"));
        let bytes = s.as_bytes();
        let has_decimal_number = bytes
            .windows(3)
            .any(|w| w[1] == b'.' && w[0].is_ascii_digit() && w[2].is_ascii_digit());
        assert!(
            !has_decimal_number,
            "no f64 decimal point may appear on a numeric surface"
        );
    }
}
