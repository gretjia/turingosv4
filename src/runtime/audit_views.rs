//! Stage C P-M8 — audit views (architect manual §7.9).
//!
//! Pure read-side projections from `EconomicState` for the architect-mandated
//! `audit_tape view-shares` / `view-pools` / `view-prices` / `view-positions`
//! audit subcommands. Functions are pure; they NEVER mutate state and NEVER
//! drive predicate verdicts (signal-only invariant per architect §7.8).
//!
//! Subcommands consume the output of these functions to render the audit
//! dashboard; CLI integration in `src/bin/audit_tape.rs` is a forward step
//! (the pure functions here are the constitution-gate-relevant surface and
//! are tested directly).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::state::price_index::cpmm_price_quote;
use crate::state::q_state::{
    AgentId, CpmmPool, EconomicState, ShareSidePair, TxId,
};
use crate::state::typed_tx::{EventId, NodePosition};

/// TRACE_MATRIX Stage C P-M8 (architect manual §7.9 verbatim
/// `audit_tape view-shares`): per-(owner, event) YES + NO share holdings,
/// canonically ordered Owner-major / Event-minor for replay-deterministic
/// dashboard regeneration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ViewShares {
    pub holdings: BTreeMap<AgentId, BTreeMap<EventId, ShareSidePair>>,
}

/// TRACE_MATRIX Stage C P-M8 (architect manual §7.9): pure projection of
/// `EconomicState.conditional_share_balances_t` into a stable view shape.
pub fn view_shares(econ: &EconomicState) -> ViewShares {
    ViewShares {
        holdings: econ.conditional_share_balances_t.0.clone(),
    }
}

/// TRACE_MATRIX Stage C P-M8 (architect manual §7.9 verbatim
/// `audit_tape view-pools`): per-event CPMM pool reserves + LP supply +
/// status. Plus the `conditional_collateral_t[event_id]` Coin units
/// covering the pool inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ViewPools {
    pub pools: BTreeMap<EventId, ViewPoolEntry>,
}

/// TRACE_MATRIX Stage C P-M8 (architect manual §7.9): per-event entry in
/// `ViewPools` — pool reserves + LP supply + status + the backing
/// collateral micro-units (from `conditional_collateral_t[event_id]`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ViewPoolEntry {
    pub pool: CpmmPool,
    pub collateral_micro_units: i64,
}

/// TRACE_MATRIX Stage C P-M8 (architect manual §7.9): pure projection of
/// `EconomicState.cpmm_pools_t` joined with `conditional_collateral_t`.
pub fn view_pools(econ: &EconomicState) -> ViewPools {
    let mut pools = BTreeMap::new();
    for (event_id, pool) in econ.cpmm_pools_t.0.iter() {
        let collateral_micro_units = econ
            .conditional_collateral_t
            .0
            .get(event_id)
            .map(|c| c.micro_units())
            .unwrap_or(0);
        pools.insert(
            event_id.clone(),
            ViewPoolEntry {
                pool: *pool,
                collateral_micro_units,
            },
        );
    }
    ViewPools { pools }
}

/// TRACE_MATRIX Stage C P-M8 (architect manual §7.9 verbatim
/// `audit_tape view-prices`): per-event price quotes for `BuyYes` and
/// `BuyNo` directions at a probe `pay_units`, plus low-liquidity warning.
/// All values are signal-only (architect §7.8 + universal forbidden list
/// "no price-as-truth"); `audit_tape view-prices` reports them but no
/// admission path consumes them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ViewPrices {
    pub probe_pay_units: u128,
    pub prices: BTreeMap<EventId, ViewPriceEntry>,
}

/// TRACE_MATRIX Stage C P-M8 (architect manual §7.9): per-event entry in
/// `ViewPrices` — BuyYes + BuyNo `getY/getN` totals (signal-only) at the
/// view's probe `pay_units` + low-liquidity warning. None values mean the
/// quote is invalid (zero pay, overflow, or formula yields zero out).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ViewPriceEntry {
    /// `getY` total (= probe_pay_units + outY) per BuyYes direction;
    /// `None` if quote is invalid (zero pay, overflow, or formula yields
    /// zero out).
    pub buy_yes_get_total: Option<u128>,
    /// `getN` total per BuyNo direction.
    pub buy_no_get_total: Option<u128>,
    /// Low-liquidity warning per `STAGE_C_LOW_LIQUIDITY_THRESHOLD_UNITS`.
    pub low_liquidity_warning: bool,
}

/// TRACE_MATRIX Stage C P-M8 (architect manual §7.9): pure projection of
/// `EconomicState.cpmm_pools_t` into per-event price quotes for both
/// directions at a fixed probe `pay_units`. Quotes use the pure
/// `cpmm_price_quote` helper; signal-only.
pub fn view_prices(econ: &EconomicState, probe_pay_units: u128) -> ViewPrices {
    let mut prices = BTreeMap::new();
    for (event_id, pool) in econ.cpmm_pools_t.0.iter() {
        // BuyYes: pool_input = NO, pool_output = YES.
        let yes_quote = cpmm_price_quote(
            pool.pool_no.units,
            pool.pool_yes.units,
            probe_pay_units,
        );
        // BuyNo: pool_input = YES, pool_output = NO.
        let no_quote = cpmm_price_quote(
            pool.pool_yes.units,
            pool.pool_no.units,
            probe_pay_units,
        );
        let low_liquidity = match (yes_quote, no_quote) {
            (Some((_, w)), _) | (_, Some((_, w))) => w,
            _ => false,
        };
        prices.insert(
            event_id.clone(),
            ViewPriceEntry {
                buy_yes_get_total: yes_quote.map(|(g, _)| g),
                buy_no_get_total: no_quote.map(|(g, _)| g),
                low_liquidity_warning: low_liquidity,
            },
        );
    }
    ViewPrices {
        probe_pay_units,
        prices,
    }
}

/// TRACE_MATRIX Stage C P-M8 (architect manual §7.9 verbatim
/// `audit_tape view-positions`): TB-12 NodePositions exposure index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ViewPositions {
    pub positions: BTreeMap<TxId, NodePosition>,
}

/// TRACE_MATRIX Stage C P-M8 (architect manual §7.9): pure projection of
/// `EconomicState.node_positions_t`.
pub fn view_positions(econ: &EconomicState) -> ViewPositions {
    ViewPositions {
        positions: econ.node_positions_t.0.clone(),
    }
}
