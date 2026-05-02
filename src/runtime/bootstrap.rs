//! TB-10 Atom 1 — Reusable preseed factory for chaintape genesis QState.
//!
//! Single source of truth for the initial `balances_t` map populated when a
//! fresh chaintape is bootstrapped. Both the evaluator binary and the new
//! `lean_market` user CLI call this factory so that whichever process
//! bootstraps the chain first produces the SAME genesis QState — ensuring
//! the user CLI and evaluator can both attach to the same on-disk chaintape
//! and observe consistent balances.
//!
//! **Constitutional gate** (Art. III.4 / P3 kill #1 — "no post-init mint"):
//! this factory is consumed ONLY at chaintape bootstrap (genesis QState
//! construction via `runtime::adapter::genesis_with_balances`). It is NOT a
//! runtime mint path. `assert_no_post_init_mint` continues to fire on every
//! subsequent typed_tx and rejects any non-genesis mint attempt.
//!
//! **Replay determinism**: the function is pure (no env reads, no clock,
//! no randomness). Two calls produce byte-identical Vec output. Past chains
//! continue to replay from their on-disk genesis_report.json regardless of
//! future edits to this factory; only fresh bootstraps consume the current
//! version.
//!
//! Per `handover/audits/CHARTER_RATIFICATION_TB_10_2026-05-02.md` §1 Q2 +
//! §2.4. Consolidates the inline literal previously at
//! `experiments/minif2f_v4/src/bin/evaluator.rs:716-731`.

use crate::economy::money::MicroCoin;
use crate::state::q_state::AgentId;

/// TRACE_MATRIX FC2 Boot: TB-10 Atom 1 — sponsor + user-sponsor + 10 solver agent budgets.
///
/// The 12 entries (in stable insertion order):
///
/// 1. `tb7-7-sponsor` (10_000_000 micro = 10 Coin) — TB-7.7 D3 self-funded
///    sponsor used by evaluator's `--task-mode self|both` preseed branch
///    (`evaluator.rs:864-922`). Preserved for back-compat with TB-7+
///    smoke harness.
/// 2. `Agent_user_0` (10_000_000 micro = 10 Coin) — **TB-10 Atom 1 net-new**;
///    sponsor identity used by `lean_market post-task` subcommand.
///    `Agent_user_` prefix is the audit_dashboard §11 filter convention
///    (per ratification §2.3).
/// 3-12. `Agent_0..9` (1_000_000 micro = 1 Coin each) — solver budgets;
///    plenty for ~1000 WorkTx.stake at 1_000 each.
///
/// Total preseed supply = 10_000_000 + 10_000_000 + 10 × 1_000_000 = 30_000_000 micro
/// = 30 Coin.
///
/// **Why not env-driven**: env-driven preseed would break replay determinism
/// (genesis QState would depend on env at bootstrap time). The factory is
/// the deterministic substrate; specific runs that need different starting
/// balances should construct their own preseed Vec and call
/// `genesis_with_balances` directly.
pub fn default_pput_preseed_pairs() -> Vec<(AgentId, MicroCoin)> {
    let mut pairs: Vec<(AgentId, MicroCoin)> = vec![
        (
            AgentId("tb7-7-sponsor".into()),
            MicroCoin::from_micro_units(10_000_000),
        ),
        (
            AgentId("Agent_user_0".into()),
            MicroCoin::from_micro_units(10_000_000),
        ),
    ];
    for i in 0..10 {
        pairs.push((
            AgentId(format!("Agent_{i}")),
            MicroCoin::from_micro_units(1_000_000),
        ));
    }
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    /// U1 — factory returns 12 entries: 1 tb7-7-sponsor + 1 Agent_user_0 + 10 Agent_i.
    #[test]
    fn returns_12_entries() {
        let pairs = default_pput_preseed_pairs();
        assert_eq!(pairs.len(), 12, "expected 12 preseed entries");
    }

    /// U2 — every entry has positive balance (no zero-funded agent).
    #[test]
    fn every_entry_has_positive_balance() {
        for (agent, balance) in default_pput_preseed_pairs() {
            assert!(
                balance.micro_units() > 0,
                "agent {} has zero balance",
                agent.0
            );
        }
    }

    /// U3 — Agent_user_0 is present with the documented sponsor budget.
    #[test]
    fn agent_user_0_present_with_sponsor_budget() {
        let pairs = default_pput_preseed_pairs();
        let user_entry = pairs
            .iter()
            .find(|(a, _)| a.0 == "Agent_user_0")
            .expect("Agent_user_0 must be in preseed list");
        assert_eq!(
            user_entry.1.micro_units(),
            10_000_000,
            "Agent_user_0 sponsor budget"
        );
    }

    /// U4 — tb7-7-sponsor is preserved (back-compat with TB-7.7 D3 evaluator preseed).
    #[test]
    fn tb_7_7_sponsor_preserved() {
        let pairs = default_pput_preseed_pairs();
        let sponsor_entry = pairs
            .iter()
            .find(|(a, _)| a.0 == "tb7-7-sponsor")
            .expect("tb7-7-sponsor must be in preseed list");
        assert_eq!(
            sponsor_entry.1.micro_units(),
            10_000_000,
            "tb7-7-sponsor budget"
        );
    }

    /// U5 — 10 solver agents Agent_0..Agent_9 each at 1_000_000 micro.
    #[test]
    fn ten_solver_agents_each_one_coin() {
        let pairs = default_pput_preseed_pairs();
        for i in 0..10 {
            let id = format!("Agent_{i}");
            let entry = pairs
                .iter()
                .find(|(a, _)| a.0 == id)
                .unwrap_or_else(|| panic!("Agent_{i} must be in preseed list"));
            assert_eq!(entry.1.micro_units(), 1_000_000, "Agent_{i} budget");
        }
    }

    /// U6 — total preseed supply is 30_000_000 micro.
    #[test]
    fn total_preseed_supply_30m() {
        let total: i64 = default_pput_preseed_pairs()
            .iter()
            .map(|(_, m)| m.micro_units())
            .sum();
        assert_eq!(total, 30_000_000, "total preseed micro");
    }

    /// U7 — factory is deterministic: two calls produce byte-identical output.
    #[test]
    fn deterministic_across_calls() {
        let a = default_pput_preseed_pairs();
        let b = default_pput_preseed_pairs();
        assert_eq!(a.len(), b.len());
        for ((a_id, a_m), (b_id, b_m)) in a.iter().zip(b.iter()) {
            assert_eq!(a_id.0, b_id.0);
            assert_eq!(a_m.micro_units(), b_m.micro_units());
        }
    }

    /// U8 — feeding the factory output into genesis_with_balances yields a
    /// QState whose balances_t Σ matches the documented 30M total.
    #[test]
    fn genesis_construction_matches_total() {
        use crate::runtime::adapter::genesis_with_balances;
        let pairs = default_pput_preseed_pairs();
        let q = genesis_with_balances(&pairs);
        let total: i64 = q
            .economic_state_t
            .balances_t
            .0
            .values()
            .map(|m| m.micro_units())
            .sum();
        assert_eq!(total, 30_000_000, "genesis balances Σ");
    }
}
