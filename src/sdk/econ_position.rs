//! TB-N1-AGENT-ECONOMY A2 (session #35 2026-05-10) — agent's
//! economic position renderer.
//!
//! Reads the canonical `EconomicState` for a single `AgentId` and produces
//! a human-readable block to embed in the agent prompt under the
//! `=== Your Economic Position ===` heading.
//!
//! **Constitutional binding** (CLAUDE.md §13 economy laws + Art. III.2
//! progressive disclosure): the agent must perceive its *own* economy
//! to act on the `writes/append/challenge/verify/settle require
//! stake/escrow/bond` substrate. Pre-A2, the prompt rendered a single
//! `Balance: N Coins` line with no escrow / claim / stake / reputation
//! visibility — economy was bookkeeping at system layer but invisible
//! to agent. A2 closes that perception gap.
//!
//! **Strict scope**: read-only projection. No state mutation. Per
//! `feedback_no_workarounds_strict_constitution`, this is NOT a quick
//! prompt fix — every line traces to a real on-tape canonical state
//! field (balances_t / stakes_t / claims_t / reputations_t).

use crate::state::q_state::{AgentId, QState};

/// TRACE_MATRIX FC1-N7 + §13 + Art. III.2: agent perceives its own
/// economic state at the δ / Agent externalized output node. Returns
/// the canonical economic-position block embedded in the per-tx prompt
/// context.
///
/// Render the agent's economic position block (body only; the
/// `=== Your Economic Position ===` heading is added by the prompt
/// builder).
///
/// Lines:
/// - `Balance: <N> μCoin (<X.X> Coins)` from `balances_t.get(agent_id)`
/// - `Active stakes: <K> μCoin across <W> pending WorkTx` from
///   `stakes_t.values().filter(staker == agent_id)`
/// - `Pending claims: <C> μCoin (earned, not yet settled)` from
///   `claims_t.values().filter(claimant == agent_id)`
/// - `Reputation: <R>` from `reputations_t.get(agent_id)`
///
/// All lines are rendered uniformly even when zero, so the layout is
/// stable across runs and agents (V3L-40: no value-anchored
/// conditional lines).
///
/// Callers should test for an empty `QState.economic_state_t` and pass
/// the rendered string regardless; the prompt builder treats empty
/// input as "no econ block".
pub fn render_econ_position(q: &QState, agent_id: &AgentId) -> String {
    let balance_micro: i64 = q
        .economic_state_t
        .balances_t
        .0
        .get(agent_id)
        .map(|m| m.micro_units())
        .unwrap_or(0);

    let mut active_stake_total_micro: i64 = 0;
    let mut active_stake_count: usize = 0;
    for entry in q.economic_state_t.stakes_t.0.values() {
        if &entry.staker == agent_id {
            active_stake_total_micro += entry.amount.micro_units();
            active_stake_count += 1;
        }
    }

    let mut pending_claims_micro: i64 = 0;
    for entry in q.economic_state_t.claims_t.0.values() {
        if &entry.claimant == agent_id {
            pending_claims_micro += entry.amount.micro_units();
        }
    }

    let reputation: i64 = q
        .economic_state_t
        .reputations_t
        .0
        .get(agent_id)
        .map(|r| r.0)
        .unwrap_or(0);

    let balance_coins = balance_micro as f64 / 1_000_000.0;

    format!(
        "Balance: {balance_micro} μCoin ({balance_coins:.2} Coins)\n\
         Active stakes: {active_stake_total_micro} μCoin across {active_stake_count} pending WorkTx\n\
         Pending claims: {pending_claims_micro} μCoin (earned, not yet settled)\n\
         Reputation: {reputation}\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::economy::money::MicroCoin;
    use crate::state::q_state::{
        ClaimEntry, EconomicState, QState, Reputation, StakeEntry, TaskId, TxId,
    };

    fn empty_q() -> QState {
        QState::default()
    }

    #[test]
    fn empty_econ_state_yields_zero_balance_block() {
        let q = empty_q();
        let agent = AgentId("Agent_0".into());
        let s = render_econ_position(&q, &agent);
        assert!(s.contains("Balance: 0 μCoin"));
        assert!(s.contains("Active stakes: 0 μCoin across 0 pending WorkTx"));
        assert!(s.contains("Pending claims: 0 μCoin"));
        assert!(s.contains("Reputation: 0"));
    }

    #[test]
    fn preseeded_agent_balance_renders() {
        let mut q = empty_q();
        q.economic_state_t
            .balances_t
            .0
            .insert(AgentId("Agent_0".into()), MicroCoin::from_micro_units(1_000_000));
        let s = render_econ_position(&q, &AgentId("Agent_0".into()));
        assert!(s.contains("Balance: 1000000 μCoin (1.00 Coins)"));
    }

    #[test]
    fn other_agent_balance_does_not_leak() {
        let mut q = empty_q();
        q.economic_state_t
            .balances_t
            .0
            .insert(AgentId("Agent_5".into()), MicroCoin::from_micro_units(7_000_000));
        let s = render_econ_position(&q, &AgentId("Agent_0".into()));
        assert!(s.contains("Balance: 0 μCoin"));
        assert!(!s.contains("7000000"));
    }

    #[test]
    fn active_stakes_aggregate_across_pending_worktx() {
        let mut q = empty_q();
        let agent = AgentId("Agent_0".into());
        // Two pending stakes for Agent_0; one for someone else.
        q.economic_state_t.stakes_t.0.insert(
            TxId("worktx-1".into()),
            StakeEntry {
                amount: MicroCoin::from_micro_units(1000),
                staker: agent.clone(),
                task_id: TaskId("task-A".into()),
            },
        );
        q.economic_state_t.stakes_t.0.insert(
            TxId("worktx-2".into()),
            StakeEntry {
                amount: MicroCoin::from_micro_units(2500),
                staker: agent.clone(),
                task_id: TaskId("task-A".into()),
            },
        );
        q.economic_state_t.stakes_t.0.insert(
            TxId("worktx-3".into()),
            StakeEntry {
                amount: MicroCoin::from_micro_units(99999),
                staker: AgentId("Agent_1".into()),
                task_id: TaskId("task-A".into()),
            },
        );
        let s = render_econ_position(&q, &agent);
        assert!(
            s.contains("Active stakes: 3500 μCoin across 2 pending WorkTx"),
            "expected 1000+2500=3500, count=2; got: {s}"
        );
    }

    #[test]
    fn pending_claims_sum_for_agent_only() {
        let mut q = empty_q();
        let agent = AgentId("Agent_0".into());
        q.economic_state_t.claims_t.0.insert(
            TxId("claim-1".into()),
            ClaimEntry {
                amount: MicroCoin::from_micro_units(500),
                claimant: agent.clone(),
                task_id: TaskId("task-A".into()),
                escrow_lock_tx_id: TxId("escrow-1".into()),
                ..Default::default()
            },
        );
        q.economic_state_t.claims_t.0.insert(
            TxId("claim-2".into()),
            ClaimEntry {
                amount: MicroCoin::from_micro_units(700),
                claimant: agent.clone(),
                task_id: TaskId("task-B".into()),
                escrow_lock_tx_id: TxId("escrow-2".into()),
                ..Default::default()
            },
        );
        q.economic_state_t.claims_t.0.insert(
            TxId("claim-3".into()),
            ClaimEntry {
                amount: MicroCoin::from_micro_units(99999),
                claimant: AgentId("Agent_5".into()),
                task_id: TaskId("task-B".into()),
                escrow_lock_tx_id: TxId("escrow-3".into()),
                ..Default::default()
            },
        );
        let s = render_econ_position(&q, &agent);
        assert!(
            s.contains("Pending claims: 1200 μCoin"),
            "expected 500+700=1200; got: {s}"
        );
    }

    #[test]
    fn reputation_renders() {
        let mut q = empty_q();
        q.economic_state_t
            .reputations_t
            .0
            .insert(AgentId("Agent_0".into()), Reputation(42));
        let s = render_econ_position(&q, &AgentId("Agent_0".into()));
        assert!(s.contains("Reputation: 42"));
    }

    #[test]
    fn block_layout_is_uniform_across_zero_and_nonzero() {
        // V3L-40: layout must be stable; do not conditionally omit lines.
        let zero_block = render_econ_position(&empty_q(), &AgentId("Agent_0".into()));
        let mut q = empty_q();
        q.economic_state_t.balances_t.0.insert(
            AgentId("Agent_0".into()),
            MicroCoin::from_micro_units(123456),
        );
        let nonzero_block = render_econ_position(&q, &AgentId("Agent_0".into()));
        let zero_lines = zero_block.lines().count();
        let nonzero_lines = nonzero_block.lines().count();
        assert_eq!(
            zero_lines, nonzero_lines,
            "block must have same line count for zero vs non-zero (V3L-40 stability); zero={zero_block}, nonzero={nonzero_block}"
        );
    }
}
