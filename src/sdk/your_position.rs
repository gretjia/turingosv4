//! TB-G G3.3 — `=== Your Position ===` per-viewer prompt block (Drucker framing).
//!
//! Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
//! §1 Module G3 atom G3.3.
//!
//! Directive: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
//! §G3 verbatim Drucker framing — `What gets measured gets managed`.
//!
//! Mirror-pattern siblings:
//! - `src/sdk/pending_peer_reviews.rs` (TB-G G2P.1; per-viewer queue block)
//! - `src/sdk/econ_position.rs` (TB-N1 A2; per-viewer economic block)
//!
//! **Constitutional binding** (CLAUDE.md §15 selective shielding + Art. III
//! progressive disclosure + Drucker framing per architect §G3):
//! - Each viewer sees ONLY its own 7-field `AgentMarketStateView` (balance,
//!   open positions, realized/unrealized PnL, solvency status, reputation).
//! - NEVER renders another agent's PnL — the per-viewer filter is enforced
//!   by `compute_agent_pnl(q, viewer, ...)` which scans by `&AgentId` and
//!   only emits matching rows (Art. III.4 "Other Agents cannot retrieve").
//! - Drucker verbatim framing line:
//!   `"Drucker: 'What gets measured gets managed' — your position drives your next decision."`
//!   This is the architect's framing for the prompt block (§G3 charter row
//!   G3.3); modifying the string would break SG-G3.13.
//!
//! **Strict scope**: read-only projection of canonical `EconomicState`.
//! No mutation. Pure derivation via `compute_agent_pnl`.

use crate::runtime::agent_pnl::{
    compute_agent_pnl, initial_balance_micro_from_default_preseed, OpenPosition, SolvencyStatus,
};
use crate::state::q_state::{AgentId, QState};

/// TRACE_MATRIX FC1-N7 + §15 (TB-G G3.3 2026-05-12; G-Phase directive §G3
/// SG-G3.6 + SG-G3.7 + SG-G3.13 Drucker verbatim framing): the architect-
/// verbatim framing string. SG-G3.13 binds this exact text into the
/// rendered block; any drift fails the gate test.
pub const DRUCKER_FRAMING_LINE: &str =
    "Drucker: 'What gets measured gets managed' \u{2014} your position drives your next decision.";

/// TRACE_MATRIX FC1-N7 + §15 + Art. III.2 progressive disclosure (TB-G
/// G3.3 2026-05-12; charter §1 Module G3 atom G3.3 + G-Phase directive §G3
/// verbatim 7-field shape + Drucker framing): agent perceives its own
/// PnL state at the δ / Agent externalized output node.
///
/// Render the per-viewer `=== Your Position ===` block (body only; the
/// heading is added by the prompt builder).
///
/// Pulls `compute_agent_pnl(q, viewer, initial_balance_micro)` from the
/// canonical G3.1 view. Reads ONLY `viewer`'s own slice of EconomicState;
/// never aggregates across agents.
///
/// Layout (V3L-40 stable across zero/nonzero):
/// ```text
/// Drucker: 'What gets measured gets managed' — your position drives your next decision.
/// Balance: <N> μC (initial <I>)
/// Realized PnL: <signed> μC (current balance − initial)
/// Unrealized PnL: <signed> μC (mark-to-market on open share positions)
/// Solvency: <solvent | near_insolvent | bankrupt>
/// Reputation: <N>
/// Open positions: <count>
/// <up to OPEN_POS_RENDER_K rows of position summaries (per-viewer)>
/// ```
///
/// Returns empty string when there is neither a balances_t entry nor open
/// positions for `viewer`; the prompt builder suppresses the block in
/// that case (mirrors `econ_position` + `pending_peer_reviews`
/// suppression contracts).
pub fn render_your_position(q: &QState, viewer: &AgentId) -> String {
    let initial_balance_micro = initial_balance_micro_from_default_preseed(viewer);
    let view = compute_agent_pnl(q, viewer, initial_balance_micro);

    // Suppress block entirely when the viewer has no economic footprint
    // (no balances_t entry AND no open positions). Avoids polluting
    // unit-test fixtures + minimal callers without sequencer access.
    if view.balance == 0 && view.open_positions.is_empty() && initial_balance_micro == 0 {
        return String::new();
    }

    let solvency_label = match view.solvency_status {
        SolvencyStatus::Solvent => "solvent",
        SolvencyStatus::NearInsolvent => "near_insolvent",
        SolvencyStatus::Bankrupt => "bankrupt",
    };

    let mut s = String::new();
    s.push_str(DRUCKER_FRAMING_LINE);
    s.push('\n');
    s.push_str(&format!(
        "Balance: {} \u{03BC}C (initial {})\n",
        view.balance, initial_balance_micro
    ));
    s.push_str(&format!(
        "Realized PnL: {} \u{03BC}C (current balance \u{2212} initial)\n",
        view.realized_pnl
    ));
    s.push_str(&format!(
        "Unrealized PnL: {} \u{03BC}C (mark-to-market on open share positions)\n",
        view.unrealized_pnl
    ));
    s.push_str(&format!("Solvency: {solvency_label}\n"));
    s.push_str(&format!("Reputation: {}\n", view.reputation_score));
    s.push_str(&format!("Open positions: {}\n", view.open_positions.len()));
    for pos in view.open_positions.iter().take(OPEN_POS_RENDER_K) {
        match pos {
            OpenPosition::Stake {
                tx_id,
                amount_micro,
            } => s.push_str(&format!(
                "  - stake on {}: {} \u{03BC}C\n",
                tx_id.0, amount_micro
            )),
            OpenPosition::Claim {
                tx_id,
                amount_micro,
            } => s.push_str(&format!(
                "  - pending claim on {}: {} \u{03BC}C\n",
                tx_id.0, amount_micro
            )),
            OpenPosition::ConditionalShare {
                event_id,
                side,
                units,
            } => s.push_str(&format!(
                "  - shares {} on event {}: {} units\n",
                match side {
                    crate::state::typed_tx::OutcomeSide::Yes => "YES",
                    crate::state::typed_tx::OutcomeSide::No => "NO",
                },
                event_id.0 .0,
                units
            )),
            OpenPosition::LpShare { event_id, units } => s.push_str(&format!(
                "  - LP shares on event {}: {} units\n",
                event_id.0 .0, units
            )),
            OpenPosition::NodePosition {
                node_id,
                side,
                amount_micro,
                ..
            } => {
                s.push_str(&format!(
                    "  - node position {} on {}: {} \u{03BC}C\n",
                    match side {
                        crate::state::typed_tx::PositionSide::Long => "Long",
                        crate::state::typed_tx::PositionSide::Short => "Short",
                    },
                    node_id.0,
                    amount_micro
                ));
            }
        }
    }
    let conviction_budget =
        crate::runtime::real6_conviction_budget::render_scoped_conviction_budget_summary(q, viewer);
    if !conviction_budget.is_empty() {
        if !s.ends_with('\n') {
            s.push('\n');
        }
        s.push_str(&conviction_budget);
    }
    s
}

/// TRACE_MATRIX FC1-N7 (TB-G G3.3 2026-05-12; bounded-context-window):
/// cap on per-viewer position rows rendered in the prompt block. Mirrors
/// `pending_peer_reviews::DEFAULT_PENDING_REVIEWS_K` framing — the agent
/// still has the full canonical state via `=== Current Chain ===`; this
/// block is a curated summary.
pub const OPEN_POS_RENDER_K: usize = 8;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::economy::money::MicroCoin;
    use crate::state::q_state::{
        ClaimEntry, ClaimStatus, QState, Reputation, StakeEntry, TaskId, TxId,
    };

    fn agent(name: &str) -> AgentId {
        AgentId(name.into())
    }

    fn empty_q() -> QState {
        QState::default()
    }

    /// U1 — empty QState + unknown viewer returns empty string (block suppressed).
    #[test]
    fn unknown_viewer_yields_empty_block() {
        let q = empty_q();
        let s = render_your_position(&q, &agent("Agent_unknown_xyz"));
        assert!(
            s.is_empty(),
            "unknown viewer should suppress block; got: {s:?}"
        );
    }

    /// U2 — preseed agent (Agent_0) at zero balance still renders block
    /// because the canonical preseed lookup confirms identity.
    #[test]
    fn preseed_agent_at_zero_balance_renders_block() {
        let q = empty_q();
        let s = render_your_position(&q, &agent("Agent_0"));
        // initial = 1_000_000 per default_pput_preseed_pairs
        assert!(s.contains("Balance: 0"));
        assert!(s.contains("initial 1000000"));
        assert!(s.contains("Realized PnL: -1000000"));
        assert!(s.contains("Solvency: bankrupt"));
    }

    /// U3 — Drucker verbatim framing line present at the head of the block.
    #[test]
    fn drucker_framing_line_present() {
        let mut q = empty_q();
        q.economic_state_t
            .balances_t
            .0
            .insert(agent("Agent_0"), MicroCoin::from_micro_units(1_000_000));
        let s = render_your_position(&q, &agent("Agent_0"));
        assert!(
            s.contains("Drucker:") && s.contains("What gets measured gets managed"),
            "Drucker verbatim framing must appear; got: {s}"
        );
        assert!(s.starts_with(DRUCKER_FRAMING_LINE));
    }

    /// U4 — per-viewer isolation: another agent's stake does not appear in
    /// our viewer's block (Art. III.4 binding).
    #[test]
    fn per_viewer_no_cross_agent_leak() {
        let mut q = empty_q();
        let alice = agent("Agent_0");
        let bob = agent("Agent_1");
        q.economic_state_t
            .balances_t
            .0
            .insert(alice.clone(), MicroCoin::from_micro_units(1_000_000));
        q.economic_state_t
            .balances_t
            .0
            .insert(bob.clone(), MicroCoin::from_micro_units(500_000));
        q.economic_state_t.stakes_t.0.insert(
            TxId("bobs-stake".into()),
            StakeEntry {
                amount: MicroCoin::from_micro_units(100_000),
                staker: bob.clone(),
                task_id: TaskId("task-B".into()),
            },
        );
        let alice_block = render_your_position(&q, &alice);
        assert!(!alice_block.contains("bobs-stake"));
        assert!(!alice_block.contains("100000 \u{03BC}C"));
        assert!(alice_block.contains("Open positions: 0"));
    }

    /// U5 — agent's own stake and pending claim render in the open-positions
    /// list with truncated detail (≤ OPEN_POS_RENDER_K).
    #[test]
    fn own_positions_render_with_details() {
        let mut q = empty_q();
        let a = agent("Agent_0");
        q.economic_state_t
            .balances_t
            .0
            .insert(a.clone(), MicroCoin::from_micro_units(800_000));
        q.economic_state_t.stakes_t.0.insert(
            TxId("worktx-1".into()),
            StakeEntry {
                amount: MicroCoin::from_micro_units(50_000),
                staker: a.clone(),
                task_id: TaskId("task-A".into()),
            },
        );
        q.economic_state_t.claims_t.0.insert(
            TxId("claim-1".into()),
            ClaimEntry {
                amount: MicroCoin::from_micro_units(30_000),
                claimant: a.clone(),
                task_id: TaskId("task-A".into()),
                status: ClaimStatus::Open,
                ..Default::default()
            },
        );
        let s = render_your_position(&q, &a);
        assert!(s.contains("Open positions: 2"));
        assert!(s.contains("stake on worktx-1: 50000"));
        assert!(s.contains("pending claim on claim-1: 30000"));
    }

    /// U6 — reputation surfaces in the block.
    #[test]
    fn reputation_renders() {
        let mut q = empty_q();
        let a = agent("Agent_0");
        q.economic_state_t
            .balances_t
            .0
            .insert(a.clone(), MicroCoin::from_micro_units(1_000_000));
        q.economic_state_t
            .reputations_t
            .0
            .insert(a.clone(), Reputation(42));
        let s = render_your_position(&q, &a);
        assert!(s.contains("Reputation: 42"));
    }
}
