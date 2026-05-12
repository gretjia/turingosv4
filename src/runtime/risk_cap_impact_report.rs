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

use serde::{Deserialize, Serialize};

use crate::state::q_state::{AgentId, EconomicState, TaskId};
use crate::state::typed_tx::{FinalizeRewardTx, RejectionClass, RunOutcome, TransitionError};

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
                verifier_bond_return_delta_micro = verifier_bond_return_delta_micro
                    .saturating_add(amt);
                verifier_bonds_at_pre_micro = verifier_bonds_at_pre_micro
                    .saturating_add(amt);
            }
        }
        // Escrow at pre = escrows_t[claim.escrow_lock_tx_id].amount.
        if let Some(esc) = q_pre.escrows_t.0.get(&claim.escrow_lock_tx_id) {
            escrow_at_pre_micro = esc.amount.micro_units();
        }
    }

    let total_payout_delta_micro = solver_reward_delta_micro
        .saturating_add(verifier_bond_return_delta_micro);
    let escrow_plus_bonds_at_pre_micro = escrow_at_pre_micro
        .saturating_add(verifier_bonds_at_pre_micro);

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
