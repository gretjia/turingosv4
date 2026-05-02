// Tier 2: WalletTool — read-only projection over EconomicState.balances_t.
//
// **TB-9 collapse (2026-05-02)**: per architect directive 2026-05-02 Part C
// line 1574 ("WalletTool read-only projection / EconomicState canonical /
// no f64 mutation"), this tool no longer carries owned f64 ledger state.
// Balances are projected from `EconomicState.balances_t: BTreeMap<AgentId,
// MicroCoin>` (the canonical chain-derived ledger written by typed_tx
// dispatch arms since TB-3).
//
// Pre-TB-9 surface (deleted in this collapse):
//   - `balances: HashMap<String, f64>` field
//   - `portfolios: HashMap<String, Portfolio>` field
//   - `genesis_done`, `genesis_coins`
//   - `deduct / credit / record_shares / ensure_agents`
//   - `save_to_disk / load_from_disk` (legacy v3 cross-problem-continuity hook)
//   - `on_init` minted f64 balances at genesis (replaced by `q_state::genesis()`
//      writing `EconomicState.balances_t`)
//
// All deletions are permanent; bus.rs market f64 mutator path deleted in
// the same atom (Atom 4). MicroCoin is the only currency unit going forward.

use crate::sdk::tool::{ToolSignal, TuringTool};
use crate::economy::money::MicroCoin;
use crate::state::q_state::{AgentId, EconomicState};
use serde::{Deserialize, Serialize};
use std::any::Any;

/// TB-9 collapse: read-only projection wrapper. Holds zero owned ledger state.
/// Retained as a `TuringTool` for tool-discovery code; balance reads route to
/// `EconomicState.balances_t`.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WalletTool;

impl WalletTool {
    /// TB-9: zero-state constructor. Pre-TB-9 callers passed `genesis_coins`;
    /// that field no longer exists (genesis is `q_state::genesis()` writing
    /// `EconomicState.balances_t`).
    pub fn new() -> Self {
        Self
    }

    /// TB-9: read-only balance projection from canonical `EconomicState`.
    /// Returns `MicroCoin::zero()` for absent agents (NOT a panic — the chain
    /// path treats unknown AgentId as "no balance row" by construction).
    pub fn balance(&self, agent: &AgentId, econ: &EconomicState) -> MicroCoin {
        econ.balances_t.0.get(agent).copied().unwrap_or_else(MicroCoin::zero)
    }
}

impl TuringTool for WalletTool {
    fn manifest(&self) -> &str {
        "wallet"
    }

    /// TB-9: no-op (genesis is now `q_state::genesis()`).
    fn on_init(&mut self, _agent_ids: &[String]) {}

    /// TB-9: appends are free per Law 1; admission gates own all veto logic
    /// at the typed_tx layer. Returns `Pass` unconditionally.
    fn on_pre_append(&mut self, _author: &str, _payload: &str) -> ToolSignal {
        ToolSignal::Pass
    }

    fn on_halt(&mut self, _golden_path: &[String]) {}

    /// TB-9: balance queries via TuringTool's stringly-typed surface return
    /// `None` because no `EconomicState` reference is plumbed through the
    /// trait. Callers who need a balance read MUST use
    /// `WalletTool::balance(&AgentId, &EconomicState)` directly.
    fn query_state(&self, _key: &str) -> Option<String> {
        None
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::q_state::{BalancesIndex, EconomicState};

    fn econ_with(agent: &str, micro: i64) -> EconomicState {
        let mut econ = EconomicState::default();
        let mut bal = BalancesIndex::default();
        bal.0.insert(AgentId(agent.to_string()), MicroCoin::from_micro_units(micro));
        econ.balances_t = bal;
        econ
    }

    /// U-W9.a — projection returns canonical `EconomicState.balances_t` value.
    #[test]
    fn projects_balance_from_economic_state() {
        let wallet = WalletTool::new();
        let econ = econ_with("A0", 7_500_000);
        let bal = wallet.balance(&AgentId("A0".into()), &econ);
        assert_eq!(bal, MicroCoin::from_micro_units(7_500_000));
    }

    /// U-W9.b — projection on absent AgentId returns zero (no panic).
    #[test]
    fn projects_zero_for_absent_agent() {
        let wallet = WalletTool::new();
        let econ = EconomicState::default();
        let bal = wallet.balance(&AgentId("ghost".into()), &econ);
        assert_eq!(bal, MicroCoin::zero());
    }

    /// U-W9.c — `on_init` is a no-op (genesis is q_state::genesis() now).
    #[test]
    fn on_init_is_noop() {
        let mut wallet = WalletTool::new();
        wallet.on_init(&["A0".into(), "A1".into()]);
        // No state to inspect; assertion: no panic + no error.
    }

    /// U-W9.d — `on_pre_append` always passes.
    #[test]
    fn on_pre_append_always_passes() {
        let mut wallet = WalletTool::new();
        let signal = wallet.on_pre_append("any-author", "any-payload");
        assert!(matches!(signal, ToolSignal::Pass));
    }

    /// U-W9.e — `query_state` returns None for any key (TuringTool surface
    /// can't carry an `EconomicState` ref by design; callers use `.balance(&AgentId, &EconomicState)`).
    #[test]
    fn query_state_returns_none() {
        let wallet = WalletTool::new();
        assert!(wallet.query_state("balance_A0").is_none());
        assert!(wallet.query_state("any_key").is_none());
    }
}
