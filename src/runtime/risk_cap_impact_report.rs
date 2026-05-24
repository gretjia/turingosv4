//! TB-G G3.2 (charter §1 Module G3; 2026-05-12): audit views over the
//! canonical chain for risk-cap-rejection attribution + FinalizeRewardTx
//! payout breakdown.
//!
//! **Architect §7.1**: `RiskCapImpactReport` — per-rejection rows with
//! `risk_cap_rejections + agent_id + balance_before + risk_cap + tx_kind
//! + task_id + whether_another_agent_continued + solve_outcome`. Wired
//! into audit dashboard so post-G3.2 solve-rate analysis can attribute
//! regression to risk-cap suppression vs other causes.
//!
//! **Architect §7.5**: `FinalizeRewardPayoutBreakdown` — separates
//! `solver_reward_delta` + `verifier_bond_return_delta` (+
//! `other_settlement_delta` if present) so audit traceability can verify
//! `payout_sum <= escrow + bond_return` with no double credit.
//!
//! **Pure** — no CAS writes, no env access, no clock. Replay-deterministic:
//! identical chain inputs produce byte-identical outputs.
//!
//! **CLAUDE.md §13 no-f64**: all math in `i64` micro-units; no floating
//! point in any path.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::bottom_white::ledger::transition_ledger::TxKind;
use crate::state::q_state::{AgentId, EconomicState, Hash, QState, TaskId, TxId};
use crate::state::typed_tx::{
    FinalizeRewardTx, RejectionClass, RunOutcome, TransitionError, TypedTx,
};

/// TRACE_MATRIX FC1-N43 (TB-G G3.2 §7.1; 2026-05-12): one row per
/// `BankruptcyRiskCapExceeded` admission rejection on the chain. Columns
/// match architect §7.1 verbatim field list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskCapRejectionRow {
    /// Agent whose tx was rejected for risk-cap.
    pub agent_id: AgentId,
    /// Agent's balance at admission time, in `μC`.
    pub balance_before_micro: i64,
    /// Risk-cap threshold at admission time, in `μC`
    /// (= `initial_balance_micro / 10` per architect Q1).
    pub risk_cap_micro: i64,
    /// Wire tx-class discriminator (string form, e.g. `"work"`,
    /// `"verify"`, `"challenge"`, `"buy_with_coin_router"`).
    pub tx_kind: String,
    /// Task scope this rejection occurred under (None for system-wide
    /// or non-task-scoped txs; per-arm tx-kind dictates).
    pub task_id: Option<TaskId>,
    /// Did at least one OTHER agent (not the rejected one) successfully
    /// submit a work-class tx for the same task after this rejection?
    /// Diagnoses whether the rejection BLOCKED progress on the task or
    /// just one agent's attempt.
    pub another_agent_continued: bool,
    /// Final solve outcome for the task this rejection occurred under
    /// (None if task did not produce a `TerminalSummaryTx` within the
    /// chain segment analyzed).
    pub solve_outcome: Option<RunOutcome>,
}

/// TRACE_MATRIX FC1-N43 (TB-G G3.2 §7.1; 2026-05-12): aggregate report
/// over a chain — total counts + per-rejection rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RiskCapImpactReport {
    pub total_rejections: usize,
    pub rows: Vec<RiskCapRejectionRow>,
}

impl RiskCapImpactReport {
    /// TRACE_MATRIX FC1-N43 (TB-G G3.2 §7.1; 2026-05-12): render the
    /// `audit_dashboard --run-report` section for bankruptcy-risk-cap
    /// attribution. The dashboard is a materialized view, not a truth
    /// source; each row is derived from L4.E + CAS + replayed QState.
    pub fn render_section_g_2(&self) -> String {
        let mut out = String::new();
        out.push_str("\n## §G.2 RiskCapImpactReport\n");
        out.push_str("  (bankruptcy risk-cap admission rejections; derived from L4.E + CAS + replayed QState)\n");
        out.push_str(&format!(
            "  risk_cap_rejections: {}\n",
            self.total_rejections
        ));
        if self.rows.is_empty() {
            out.push_str("  (no BankruptcyRiskCapExceeded L4.E rows in this run)\n");
            return out;
        }
        out.push_str("  agent_id | balance_before_micro | risk_cap_micro | tx_kind | task_id | another_agent_continued | solve_outcome\n");
        for row in &self.rows {
            out.push_str(&format!(
                "  - {} | {} | {} | {} | {} | {} | {}\n",
                row.agent_id.0,
                row.balance_before_micro,
                row.risk_cap_micro,
                row.tx_kind,
                row.task_id.as_ref().map(|t| t.0.as_str()).unwrap_or("-"),
                row.another_agent_continued,
                row.solve_outcome
                    .map(|o| format!("{o:?}"))
                    .unwrap_or_else(|| "-".to_string()),
            ));
        }
        out
    }
}

/// TRACE_MATRIX § 3 orphan — TB-G G3.2 §7.1 path-based report walker
/// error. Kept string-light for dashboard display while preserving the
/// failing subsystem.
#[derive(Debug)]
pub enum RiskCapImpactReportError {
    L4eOpen(String),
    LedgerOpen(String),
    LedgerRead(String),
    CasOpen(String),
    CasRead(String),
    Decode(String),
    PinnedPubkeysIo(String),
    PinnedPubkeysParse(String),
    InitialQStateIo(String),
    InitialQStateParse(String),
    HexDecode(String),
    Replay(String),
    StateRootNotFound(String),
}

impl std::fmt::Display for RiskCapImpactReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::L4eOpen(e) => write!(f, "open L4.E rejections: {e}"),
            Self::LedgerOpen(e) => write!(f, "open L4 ledger: {e}"),
            Self::LedgerRead(e) => write!(f, "read L4 ledger: {e}"),
            Self::CasOpen(e) => write!(f, "open CAS: {e}"),
            Self::CasRead(e) => write!(f, "read CAS: {e}"),
            Self::Decode(e) => write!(f, "decode typed tx: {e}"),
            Self::PinnedPubkeysIo(e) => write!(f, "read pinned pubkeys: {e}"),
            Self::PinnedPubkeysParse(e) => write!(f, "parse pinned pubkeys: {e}"),
            Self::InitialQStateIo(e) => write!(f, "read initial_q_state: {e}"),
            Self::InitialQStateParse(e) => write!(f, "parse initial_q_state: {e}"),
            Self::HexDecode(e) => write!(f, "hex decode: {e}"),
            Self::Replay(e) => write!(f, "replay: {e}"),
            Self::StateRootNotFound(e) => write!(f, "state root not found: {e}"),
        }
    }
}

impl std::error::Error for RiskCapImpactReportError {}

/// TRACE_MATRIX FC1-N43 + FC2-Boot (TB-G G3.2 §7.1; 2026-05-12):
/// path-based walker for `audit_dashboard --run-report`.
///
/// The report is derived from:
/// - `<runtime_repo>/rejections.jsonl` for L4.E risk-cap rows;
/// - CAS `tx_payload_cid` for the rejected typed tx body;
/// - L4 replay from `initial_q_state.json` + `pinned_pubkeys.json` to
///   recover the exact `balance_before_micro` at `parent_state_root`.
pub fn compute_risk_cap_impact_report_from_paths(
    runtime_repo_path: &Path,
    cas_path: &Path,
) -> Result<RiskCapImpactReport, RiskCapImpactReportError> {
    use crate::bottom_white::cas::store::CasStore;
    use crate::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
    use crate::bottom_white::ledger::transition_ledger::{
        canonical_decode, Git2LedgerWriter, LedgerEntry, LedgerWriter,
    };

    let rejections_path = runtime_repo_path.join("rejections.jsonl");
    let l4e_writer = if rejections_path.exists() {
        RejectionEvidenceWriter::open_jsonl(rejections_path)
            .map_err(|e| RiskCapImpactReportError::L4eOpen(e.to_string()))?
    } else {
        RejectionEvidenceWriter::new()
    };
    let risk_records: Vec<_> = l4e_writer
        .records()
        .iter()
        .filter(|r| r.public_summary.as_deref() == Some("bankruptcy_risk_cap_exceeded"))
        .cloned()
        .collect();
    if risk_records.is_empty() {
        return Ok(RiskCapImpactReport::default());
    }

    let cas =
        CasStore::open(cas_path).map_err(|e| RiskCapImpactReportError::CasOpen(e.to_string()))?;
    let writer = Git2LedgerWriter::open(runtime_repo_path)
        .map_err(|e| RiskCapImpactReportError::LedgerOpen(format!("{e:?}")))?;
    let chain_len = writer.len();
    let mut entries: Vec<LedgerEntry> = Vec::with_capacity(chain_len as usize);
    for t in 1..=chain_len {
        entries.push(
            writer
                .read_at(t)
                .map_err(|e| RiskCapImpactReportError::LedgerRead(format!("{e:?}")))?,
        );
    }

    let mut accepted_work: Vec<(usize, TxId, TaskId, AgentId)> = Vec::new();
    let mut task_outcomes: BTreeMap<TaskId, RunOutcome> = BTreeMap::new();
    for (idx, entry) in entries.iter().enumerate() {
        let payload = match cas.get(&entry.tx_payload_cid) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let typed_tx: TypedTx = match canonical_decode(&payload) {
            Ok(t) => t,
            Err(_) => continue,
        };
        match typed_tx {
            TypedTx::Work(w) => {
                accepted_work.push((idx + 1, w.tx_id, w.task_id, w.agent_id));
            }
            TypedTx::TerminalSummary(ts) => {
                task_outcomes.insert(ts.task_id, ts.run_outcome);
            }
            _ => {}
        }
    }
    let work_task_by_tx: BTreeMap<TxId, TaskId> = accepted_work
        .iter()
        .map(|(_, tx, task, _)| (tx.clone(), task.clone()))
        .collect();

    let replay = ReplayInputs::load(runtime_repo_path, entries)?;
    let mut rows = Vec::new();
    for record in risk_records {
        let payload = cas
            .get(&record.tx_payload_cid)
            .map_err(|e| RiskCapImpactReportError::CasRead(e.to_string()))?;
        let typed_tx: TypedTx = canonical_decode(&payload)
            .map_err(|e| RiskCapImpactReportError::Decode(format!("{e:?}")))?;
        let task_id = task_id_for_rejected_tx(&typed_tx, &work_task_by_tx);
        let q_before = replay.q_at_state_root(record.parent_state_root, &cas)?;
        let balance_before_micro = q_before
            .economic_state_t
            .balances_t
            .0
            .get(&record.agent_id)
            .map(|b| b.micro_units())
            .unwrap_or(0);
        let risk_cap_micro =
            crate::runtime::agent_pnl::bankruptcy_risk_cap_micro(&record.agent_id, &q_before);
        let root_pos = replay.state_root_position(record.parent_state_root)?;
        let another_agent_continued = task_id.as_ref().map_or(false, |task| {
            accepted_work.iter().any(|(idx, _, work_task, work_agent)| {
                *idx > root_pos && work_task == task && work_agent != &record.agent_id
            })
        });
        let solve_outcome = task_id
            .as_ref()
            .and_then(|task| task_outcomes.get(task).copied());

        rows.push(RiskCapRejectionRow {
            agent_id: record.agent_id,
            balance_before_micro,
            risk_cap_micro,
            tx_kind: risk_cap_tx_kind_label(record.tx_kind).to_string(),
            task_id,
            another_agent_continued,
            solve_outcome,
        });
    }

    Ok(RiskCapImpactReport {
        total_rejections: rows.len(),
        rows,
    })
}

struct ReplayInputs {
    initial_q: QState,
    entries: Vec<crate::bottom_white::ledger::transition_ledger::LedgerEntry>,
    pinned: crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys,
}

impl ReplayInputs {
    fn load(
        runtime_repo_path: &Path,
        entries: Vec<crate::bottom_white::ledger::transition_ledger::LedgerEntry>,
    ) -> Result<Self, RiskCapImpactReportError> {
        use crate::bottom_white::ledger::system_keypair::{
            PinnedSystemPubkeys, SystemEpoch, SystemPublicKey,
        };

        let pinned_path = runtime_repo_path.join("pinned_pubkeys.json");
        let pinned_text = std::fs::read_to_string(&pinned_path).map_err(|e| {
            RiskCapImpactReportError::PinnedPubkeysIo(format!("{pinned_path:?}: {e}"))
        })?;
        let pinned_manifest: crate::runtime::PinnedPubkeyManifest =
            serde_json::from_str(&pinned_text)
                .map_err(|e| RiskCapImpactReportError::PinnedPubkeysParse(e.to_string()))?;
        let mut pinned = PinnedSystemPubkeys::new();
        for entry in &pinned_manifest.pubkeys {
            let bytes =
                decode_hex_32(&entry.pubkey_hex).map_err(RiskCapImpactReportError::HexDecode)?;
            pinned.insert(
                SystemEpoch::new(entry.epoch),
                SystemPublicKey::from_bytes(bytes),
            );
        }

        let initial_q_path = runtime_repo_path.join("initial_q_state.json");
        let initial_q = if initial_q_path.exists() {
            let text = std::fs::read_to_string(&initial_q_path)
                .map_err(|e| RiskCapImpactReportError::InitialQStateIo(e.to_string()))?;
            serde_json::from_str(&text)
                .map_err(|e| RiskCapImpactReportError::InitialQStateParse(e.to_string()))?
        } else {
            QState::genesis()
        };

        Ok(Self {
            initial_q,
            entries,
            pinned,
        })
    }

    fn state_root_position(&self, root: Hash) -> Result<usize, RiskCapImpactReportError> {
        if root == self.initial_q.state_root_t {
            return Ok(0);
        }
        self.entries
            .iter()
            .position(|entry| entry.resulting_state_root == root)
            .map(|idx| idx + 1)
            .ok_or_else(|| RiskCapImpactReportError::StateRootNotFound(hex_hash(root)))
    }

    fn q_at_state_root(
        &self,
        root: Hash,
        cas: &crate::bottom_white::cas::store::CasStore,
    ) -> Result<QState, RiskCapImpactReportError> {
        let pos = self.state_root_position(root)?;
        if pos == 0 {
            return Ok(self.initial_q.clone());
        }

        use crate::bottom_white::ledger::transition_ledger::{
            replay_full_transition_with_predicate_binding, LedgerCasView, ReplayError,
        };
        use crate::bottom_white::tools::registry::ToolRegistry;

        struct CasRef<'a>(&'a crate::bottom_white::cas::store::CasStore);
        impl<'a> LedgerCasView for CasRef<'a> {
            fn get_typed_payload(
                &self,
                cid: &crate::bottom_white::cas::schema::Cid,
            ) -> Result<Vec<u8>, ReplayError> {
                self.0
                    .get(cid)
                    .map_err(|_| ReplayError::CasMissing { at: 0 })
            }
        }

        replay_full_transition_with_predicate_binding(
            &self.initial_q,
            &self.entries[..pos],
            &CasRef(cas),
            cas,
            &self.pinned,
            &crate::runtime::predicate_registry_loader::load_replay_registry(),
            &ToolRegistry::new(),
        )
        .map_err(|e| RiskCapImpactReportError::Replay(format!("{e:?}")))
    }
}

fn task_id_for_rejected_tx(
    tx: &TypedTx,
    work_task_by_tx: &BTreeMap<TxId, TaskId>,
) -> Option<TaskId> {
    match tx {
        TypedTx::Work(w) => Some(w.task_id.clone()),
        TypedTx::Verify(v) => work_task_by_tx.get(&v.target_work_tx).cloned(),
        TypedTx::Challenge(c) => work_task_by_tx.get(&c.target_work_tx).cloned(),
        TypedTx::BuyWithCoinRouter(router) => Some(router.event_id.0.clone()),
        _ => None,
    }
}

fn risk_cap_tx_kind_label(kind: TxKind) -> &'static str {
    match kind {
        TxKind::Work => "work",
        TxKind::Verify => "verify",
        TxKind::Challenge => "challenge",
        TxKind::BuyWithCoinRouter => "buy_with_coin_router",
        _ => "other",
    }
}

fn decode_hex_32(hex: &str) -> Result<[u8; 32], String> {
    let h = hex.trim();
    if h.len() != 64 {
        return Err(format!("expected 64 hex chars, got {}", h.len()));
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        let byte = u8::from_str_radix(&h[2 * i..2 * i + 2], 16)
            .map_err(|e| format!("hex parse at {i}: {e}"))?;
        out[i] = byte;
    }
    Ok(out)
}

fn hex_hash(hash: Hash) -> String {
    hash.0.iter().map(|b| format!("{b:02x}")).collect()
}

/// TRACE_MATRIX FC1-N43 (TB-G G3.2 §7.1; 2026-05-12): tx-kind string for
/// chain projection. Stable wire shape across release builds.
pub fn tx_kind_label_for_risk_cap_rejection(tx_kind_id: u16) -> &'static str {
    // Mirror src/bottom_white/ledger/transition_ledger.rs TxKind ids.
    // Risk-cap fires in 4 admission arms: WorkTx (1), VerifyTx (2),
    // ChallengeTx (3), BuyWithCoinRouter (15 per P-M4 ordering).
    match tx_kind_id {
        1 => "work",
        2 => "verify",
        3 => "challenge",
        15 => "buy_with_coin_router",
        _ => "other",
    }
}

/// TRACE_MATRIX FC1-N43 (TB-G G3.2 §7.1; 2026-05-12): predicate for
/// `RejectionClass::BankruptcyRiskCapExceeded` — used by audit walkers
/// to filter rejected attempts to the risk-cap subset.
pub fn is_bankruptcy_risk_cap_rejection_class(rc: &RejectionClass) -> bool {
    matches!(rc, RejectionClass::BankruptcyRiskCapExceeded)
}

/// TRACE_MATRIX FC1-N43 (TB-G G3.2 §7.1; 2026-05-12): predicate for
/// `TransitionError::BankruptcyRiskCapExceeded` — used by sequencer-side
/// walkers to count risk-cap admission failures.
pub fn is_bankruptcy_risk_cap_transition_error(te: &TransitionError) -> bool {
    matches!(te, TransitionError::BankruptcyRiskCapExceeded)
}

// ────────────────────────────────────────────────────────────────────────────
// FinalizeRewardTx payout breakdown (architect §7.5; 2026-05-12)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N44 (TB-G G3.2 §7.5; 2026-05-12): payout breakdown
/// for one `FinalizeRewardTx` dispatch. Separates the 3 economic deltas
/// architect §7.5 verbatim: `solver_reward_delta` (escrow → solver,
/// from claim.amount) / `verifier_bond_return_delta` (stakes_t →
/// verifiers, via Step 7c-bis) / `other_settlement_delta` (reserved
/// for future TBs; always 0 in G3.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FinalizeRewardPayoutBreakdown {
    pub claim_id: String,
    pub task_id: TaskId,
    pub solver: AgentId,
    /// Solver-side payout delta in `μC` = escrow debited → solver balance
    /// credited. Matches `fr.reward.micro_units()` for the accepted dispatch.
    pub solver_reward_delta_micro: i64,
    /// Verifier bond-return delta in `μC` = sum of stakes_t entries
    /// removed for verifiers of this WorkTx, credited back to their
    /// balances.
    pub verifier_bond_return_delta_micro: i64,
    /// Reserved for future settlement deltas (G3.2 always 0).
    pub other_settlement_delta_micro: i64,
    /// Sum of all 3 deltas — must satisfy
    /// `total_payout <= escrow_at_pre + verifier_bond_at_pre`.
    pub total_payout_delta_micro: i64,
    /// Sum of escrow balance at PRE-dispatch + verifier bonds at PRE-
    /// dispatch — the structural upper bound on payout.
    pub escrow_plus_bonds_at_pre_micro: i64,
}

/// TRACE_MATRIX FC1-N44 (TB-G G3.2 §7.5; 2026-05-12): pure-deterministic
/// computation of the payout breakdown for one accepted `FinalizeRewardTx`.
///
/// Inputs: `q_pre.economic_state_t` (immediately before the dispatch) and
/// the `FinalizeRewardTx` itself. The breakdown is derived from:
///
/// - solver delta = `fr.reward` (= `claim.amount` at apply-time)
/// - verifier delta = `sum(stakes_t entries where task_id == claim.task_id
///   AND tx_id != claim.work_tx_id)` — same filter the sequencer uses at
///   FinalizeRewardTx Step 7c-bis
///
/// The post-dispatch `q_post` is NOT consulted here — the breakdown is
/// derivable from the pre-state + the wire tx alone, which is the
/// constitutional Information Loom contract for audit traceability.
pub fn compute_finalize_reward_payout_breakdown(
    q_pre: &EconomicState,
    fr: &FinalizeRewardTx,
) -> FinalizeRewardPayoutBreakdown {
    let claim_id = format!("{:?}", fr.claim_id);

    // Solver-side delta = reward (= escrow → solver transfer per Step 7a/7b).
    let solver_reward_delta_micro = fr.reward.micro_units();

    // Verifier-side delta = sum of stakes_t entries matching the same
    // filter used at sequencer Step 7c-bis: task_id == claim.task_id AND
    // tx_id != claim.work_tx_id. Q-derive `claim.work_tx_id` via claims_t
    // lookup.
    let mut verifier_bond_return_delta_micro: i64 = 0;
    let mut escrow_at_pre_micro: i64 = 0;
    let mut verifier_bonds_at_pre_micro: i64 = 0;

    if let Some(claim) = q_pre.claims_t.0.get(fr.claim_id.as_tx_id()) {
        for (tx_id, e) in q_pre.stakes_t.0.iter() {
            if e.task_id == claim.task_id && *tx_id != claim.work_tx_id {
                let amt = e.amount.micro_units();
                verifier_bond_return_delta_micro =
                    verifier_bond_return_delta_micro.saturating_add(amt);
                verifier_bonds_at_pre_micro = verifier_bonds_at_pre_micro.saturating_add(amt);
            }
        }
        // Escrow at pre = escrows_t[claim.escrow_lock_tx_id].amount.
        if let Some(esc) = q_pre.escrows_t.0.get(&claim.escrow_lock_tx_id) {
            escrow_at_pre_micro = esc.amount.micro_units();
        }
    }

    let total_payout_delta_micro =
        solver_reward_delta_micro.saturating_add(verifier_bond_return_delta_micro);
    let escrow_plus_bonds_at_pre_micro =
        escrow_at_pre_micro.saturating_add(verifier_bonds_at_pre_micro);

    FinalizeRewardPayoutBreakdown {
        claim_id,
        task_id: fr.task_id.clone(),
        solver: fr.solver.clone(),
        solver_reward_delta_micro,
        verifier_bond_return_delta_micro,
        other_settlement_delta_micro: 0,
        total_payout_delta_micro,
        escrow_plus_bonds_at_pre_micro,
    }
}

/// TRACE_MATRIX FC1-N44 (TB-G G3.2 §7.5; 2026-05-12): invariant for the
/// payout breakdown — `total_payout_delta <= escrow_plus_bonds_at_pre`.
/// Returns `Ok(())` on PASS; `Err` lists the violation cause.
pub fn assert_finalize_reward_payout_bounded(
    b: &FinalizeRewardPayoutBreakdown,
) -> Result<(), String> {
    if b.total_payout_delta_micro > b.escrow_plus_bonds_at_pre_micro {
        return Err(format!(
            "payout breakdown invariant violated: total_payout_delta ({}) > \
             escrow_plus_bonds_at_pre ({}); solver_reward_delta={} \
             verifier_bond_return_delta={}",
            b.total_payout_delta_micro,
            b.escrow_plus_bonds_at_pre_micro,
            b.solver_reward_delta_micro,
            b.verifier_bond_return_delta_micro,
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn risk_cap_rejection_class_predicate_matches() {
        assert!(is_bankruptcy_risk_cap_rejection_class(
            &RejectionClass::BankruptcyRiskCapExceeded
        ));
        assert!(!is_bankruptcy_risk_cap_rejection_class(
            &RejectionClass::StakeBalanceExceeded
        ));
        assert!(!is_bankruptcy_risk_cap_rejection_class(
            &RejectionClass::Opaque
        ));
    }

    #[test]
    fn risk_cap_transition_error_predicate_matches() {
        assert!(is_bankruptcy_risk_cap_transition_error(
            &TransitionError::BankruptcyRiskCapExceeded
        ));
        assert!(!is_bankruptcy_risk_cap_transition_error(
            &TransitionError::StakeBalanceExceeded
        ));
    }

    #[test]
    fn tx_kind_label_known_arms() {
        assert_eq!(tx_kind_label_for_risk_cap_rejection(1), "work");
        assert_eq!(tx_kind_label_for_risk_cap_rejection(2), "verify");
        assert_eq!(tx_kind_label_for_risk_cap_rejection(3), "challenge");
        assert_eq!(
            tx_kind_label_for_risk_cap_rejection(15),
            "buy_with_coin_router"
        );
        assert_eq!(tx_kind_label_for_risk_cap_rejection(999), "other");
    }

    #[test]
    fn payout_breakdown_no_claim_returns_zero_deltas() {
        // claim_id unknown to q_pre.claims_t → breakdown reports zero
        // verifier delta but non-zero solver delta (= fr.reward).
        let q_pre = EconomicState::default();
        let fr = FinalizeRewardTx::default();
        let b = compute_finalize_reward_payout_breakdown(&q_pre, &fr);
        assert_eq!(b.verifier_bond_return_delta_micro, 0);
        assert_eq!(b.escrow_plus_bonds_at_pre_micro, 0);
        // Bounded invariant should fail because reward > 0 and escrow = 0
        // — but FinalizeRewardTx::default() has reward = 0, so invariant
        // holds trivially.
        assert_eq!(b.solver_reward_delta_micro, 0);
        assert!(assert_finalize_reward_payout_bounded(&b).is_ok());
    }

    #[test]
    fn payout_breakdown_bounded_invariant_catches_overpay() {
        // Manually construct a breakdown where total > escrow_plus_bonds.
        let b = FinalizeRewardPayoutBreakdown {
            claim_id: "claim-test".into(),
            task_id: TaskId("t-1".into()),
            solver: AgentId("Agent_0".into()),
            solver_reward_delta_micro: 200_000,
            verifier_bond_return_delta_micro: 50_000,
            other_settlement_delta_micro: 0,
            total_payout_delta_micro: 250_000,
            escrow_plus_bonds_at_pre_micro: 200_000,
        };
        let result = assert_finalize_reward_payout_bounded(&b);
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(msg.contains("invariant violated"));
        assert!(msg.contains("250000"));
        assert!(msg.contains("200000"));
    }

    #[test]
    fn payout_breakdown_default_report_is_empty() {
        let r = RiskCapImpactReport::default();
        assert_eq!(r.total_rejections, 0);
        assert!(r.rows.is_empty());
    }
}
