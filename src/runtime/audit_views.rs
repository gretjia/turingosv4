//! Stage C P-M8 / Phase F.7 — audit views (architect manual §7.9).
//!
//! TRACE_MATRIX FC1-Append Stage C P-M8 (architect §7.9 verbatim):
//! "Add: `audit_tape view-shares / view-pools / view-prices /
//! view-positions`. Must show: owner YES/NO shares, conditional
//! collateral, pool reserves, LP shares, NodePositions, price signal."
//!
//! This module provides 4 pure view-aggregator functions over
//! `EconomicState`:
//! - `audit_view_shares` — owner YES/NO holdings + conditional collateral.
//! - `audit_view_pools` — CpmmPool reserves + LP share holdings.
//! - `audit_view_prices` — router quote signal per pool over a caller-
//!   supplied set of `pay_coin` sample sizes (read-only quote derivation).
//! - `audit_view_positions` — TB-12 NodePosition exposure aggregation.
//!
//! All functions are pure (`&EconomicState` immutable ref); no state
//! mutation; replay-deterministic (no env / clock / RNG); integer-only
//! (Coin in MicroCoin; shares in u128 ShareAmount; prices in
//! `RationalPrice`).
//!
//! Per architect §7.9 + dashboard discipline (CLAUDE.md §17):
//! - These views are derived from canonical `EconomicState` only — no
//!   stdout / private logs / hidden pointers consulted.
//! - Dashboard regenerates from these views (architect-mandated test
//!   `dashboard_regenerates_market_view`).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::economy::money::MicroCoin;
use crate::state::price_index::RationalPrice;
use crate::state::q_state::{
    AgentId, EconomicState, LpShareAmount, PoolStatus, ShareSidePair,
};
use crate::state::router_quote::{
    quote_buy_with_coin_router, LiquidityWarning, QuoteDirection,
};
use crate::state::typed_tx::{
    EventId, PositionKind, PositionSide, ShareAmount,
};
use crate::state::TxId;

// ─────────────────────────────────────────────────────────────────────────
// audit_view_shares
// ─────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9): per-
/// `(owner, event_id)` YES + NO holdings combined with the per-event
/// `conditional_collateral_t` total. Mirrors architect §7.9 spec line:
/// "owner YES/NO shares, conditional collateral".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SharesView {
    /// Per-`(owner, event_id)` YES + NO share holdings, sorted by owner
    /// then event_id (BTreeMap iteration order).
    pub owner_shares: Vec<OwnerSharesRow>,
    /// Per-event total locked collateral (Coin holding; in MicroCoin).
    pub conditional_collateral: Vec<EventCollateralRow>,
}

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9):
/// per-`(owner, event_id)` YES + NO share holding row in the audit
/// shares view. Mirrors the underlying `state::q_state::ShareSidePair`
/// (TB-13 architect §4.3 + FR-13.3) for the given owner+event tuple.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnerSharesRow {
    pub owner: AgentId,
    pub event_id: EventId,
    pub yes_units: u128,
    pub no_units: u128,
}

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9):
/// per-event collateral lockup row in the audit shares view. Mirrors
/// `state::q_state::ConditionalCollateralIndex` (TB-13 architect §4.3
/// + CR-13.3 — collateral is Coin holding; YES/NO claims are NOT Coin).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventCollateralRow {
    pub event_id: EventId,
    pub locked_micro_coin: i64,
}

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9):
/// pure view aggregator. Returns the per-`(owner, event)` YES/NO share
/// holdings + per-event conditional collateral lockup. Order is
/// canonical (BTreeMap iteration).
pub fn audit_view_shares(econ: &EconomicState) -> SharesView {
    let mut owner_shares: Vec<OwnerSharesRow> = Vec::new();
    for (owner, event_map) in econ.conditional_share_balances_t.0.iter() {
        for (event_id, pair) in event_map.iter() {
            // Skip empty rows (both sides zero) — they're not informative
            // and noise the view. Audit-significant rows have non-zero
            // exposure on at least one side.
            if pair.yes.units == 0 && pair.no.units == 0 {
                continue;
            }
            owner_shares.push(OwnerSharesRow {
                owner: owner.clone(),
                event_id: event_id.clone(),
                yes_units: pair.yes.units,
                no_units: pair.no.units,
            });
        }
    }
    let mut conditional_collateral: Vec<EventCollateralRow> = Vec::new();
    for (event_id, locked) in econ.conditional_collateral_t.0.iter() {
        conditional_collateral.push(EventCollateralRow {
            event_id: event_id.clone(),
            locked_micro_coin: locked.micro_units(),
        });
    }
    SharesView { owner_shares, conditional_collateral }
}

// ─────────────────────────────────────────────────────────────────────────
// audit_view_pools
// ─────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9): per-
/// event `CpmmPool` reserves + per-`(provider, event)` LP share holdings.
/// Mirrors architect §7.9 spec line: "pool reserves, LP shares".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PoolsView {
    pub pools: Vec<PoolRow>,
    pub lp_holdings: Vec<LpHoldingRow>,
}

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9):
/// per-event CpmmPool reserve row in the audit pools view. Mirrors the
/// underlying `state::q_state::CpmmPool` shape (P-M4 Atom; architect
/// §7.5 verbatim 5-field state struct) projected for wire serialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoolRow {
    pub event_id: EventId,
    pub pool_yes_units: u128,
    pub pool_no_units: u128,
    pub lp_total_shares_units: u128,
    pub status: PoolStatus,
    /// Constant-product `k = pool_yes * pool_no`. Useful for monitoring
    /// k-drift (architect §7.6 floor leaves dust in pool — k strictly
    /// non-decreasing across swaps; never decreasing).
    pub k_product: u128,
}

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9):
/// per-`(provider, event_id)` LP share holding row. Mirrors the
/// underlying `state::q_state::LpShareBalancesIndex` entry shape (P-M4
/// Atom; architect §7.5 rule 3 "lp shares are not Coin").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LpHoldingRow {
    pub provider: AgentId,
    pub event_id: EventId,
    pub lp_units: u128,
}

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9):
/// pure view aggregator. Returns per-event pool reserves + per-
/// `(provider, event)` LP holdings.
pub fn audit_view_pools(econ: &EconomicState) -> PoolsView {
    let mut pools: Vec<PoolRow> = Vec::new();
    for (event_id, pool) in econ.cpmm_pools_t.0.iter() {
        // k computed in u128; saturate on overflow (extreme edge — not
        // expected in normal operation).
        let k = pool
            .pool_yes
            .units
            .checked_mul(pool.pool_no.units)
            .unwrap_or(u128::MAX);
        pools.push(PoolRow {
            event_id: event_id.clone(),
            pool_yes_units: pool.pool_yes.units,
            pool_no_units: pool.pool_no.units,
            lp_total_shares_units: pool.lp_total_shares.units,
            status: pool.status,
            k_product: k,
        });
    }
    let mut lp_holdings: Vec<LpHoldingRow> = Vec::new();
    for ((provider, event_id), lp) in econ.lp_share_balances_t.0.iter() {
        if lp.units == 0 {
            continue;
        }
        lp_holdings.push(LpHoldingRow {
            provider: provider.clone(),
            event_id: event_id.clone(),
            lp_units: lp.units,
        });
    }
    PoolsView { pools, lp_holdings }
}

// ─────────────────────────────────────────────────────────────────────────
// audit_view_prices
// ─────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9): per-
/// pool router quote signal for caller-supplied `pay_coin` sample sizes
/// + both `BuyDirection` directions. Mirrors architect §7.9 spec line:
/// "price signal".
///
/// Architect §7.8 discipline: prices are signal only — this view exists
/// for audit / dashboard rendering, never for predicate decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PricesView {
    pub price_quotes: Vec<PriceQuoteRow>,
}

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9): one
/// row per `(pool, direction, pay_coin_sample)` triple in the audit
/// prices view. Mirrors `state::router_quote::RouterQuote` shape
/// projected for wire serialization (architect §7.8 integer-rational
/// price; numerator/denominator both u128; floats forbidden).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PriceQuoteRow {
    pub event_id: EventId,
    pub direction: PriceQuoteDirection,
    pub pay_coin_micro: i64,
    pub out_shares_units: u128,
    pub get_shares_units: u128,
    pub price_numerator: Option<u128>,
    pub price_denominator: Option<u128>,
    pub liquidity_warning: PriceLiquidityWarning,
}

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9):
/// wire-shape mirror of `state::router_quote::QuoteDirection` (kept
/// separate to avoid cross-module Serialize coupling on the underlying
/// enum). Audit views render this verbatim into the price quote rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PriceQuoteDirection {
    BuyYes,
    BuyNo,
}

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9):
/// wire-shape mirror of `state::router_quote::LiquidityWarning`. Audit
/// views render this directly so dashboard / audit_tape consumers can
/// flag low-liquidity quotes without re-running the quote function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PriceLiquidityWarning {
    None,
    LowLiquidity,
    NoOutput,
}

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9):
/// pure view aggregator. For each `(pool, direction, pay_coin_sample)`
/// triple, computes the router quote signal. Pools with non-Active
/// status emit no rows (quotes are not meaningful). Empty
/// `pay_coin_samples` yields an empty view.
pub fn audit_view_prices(
    econ: &EconomicState,
    pay_coin_samples: &[MicroCoin],
) -> PricesView {
    let mut price_quotes: Vec<PriceQuoteRow> = Vec::new();
    for (event_id, pool) in econ.cpmm_pools_t.0.iter() {
        if pool.status != PoolStatus::Active {
            continue;
        }
        for &pay in pay_coin_samples {
            for (dir, dir_wire) in [
                (QuoteDirection::BuyYes, PriceQuoteDirection::BuyYes),
                (QuoteDirection::BuyNo, PriceQuoteDirection::BuyNo),
            ] {
                let q = quote_buy_with_coin_router(pool, pay, dir);
                let row = match q {
                    Some(q) => PriceQuoteRow {
                        event_id: event_id.clone(),
                        direction: dir_wire,
                        pay_coin_micro: pay.micro_units(),
                        out_shares_units: q.out_shares.units,
                        get_shares_units: q.get_shares.units,
                        price_numerator: q.price_effective.map(|p| p.numerator),
                        price_denominator: q
                            .price_effective
                            .map(|p| p.denominator),
                        liquidity_warning: match q.liquidity_warning {
                            LiquidityWarning::None => PriceLiquidityWarning::None,
                            LiquidityWarning::LowLiquidity => {
                                PriceLiquidityWarning::LowLiquidity
                            }
                            LiquidityWarning::NoOutput => {
                                PriceLiquidityWarning::NoOutput
                            }
                        },
                    },
                    None => continue, // skip non-quotable inputs (zero pay, etc.)
                };
                price_quotes.push(row);
            }
        }
    }
    PricesView { price_quotes }
}

// ─────────────────────────────────────────────────────────────────────────
// audit_view_positions
// ─────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9): TB-12
/// `NodePosition` exposure aggregation. Mirrors architect §7.9 spec line:
/// "NodePositions".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PositionsView {
    pub positions: Vec<PositionRow>,
}

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9):
/// per-`NodePosition` exposure row in the audit view output. Mirrors
/// the underlying `state::typed_tx::NodePosition` shape (TB-12 Atom 1)
/// projected for wire serialization (avoids cross-module Serialize
/// coupling on the canonical typed-tx struct).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositionRow {
    pub node_id: TxId,
    pub holder: AgentId,
    pub side: PositionSide,
    pub kind: PositionKind,
    pub amount_micro: i64,
}

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect §7.9):
/// pure view aggregator. Returns the per-NodePosition exposure rows
/// directly from `node_positions_t`. NodePosition is a CR-12.1 immutable
/// exposure record (NOT Coin holding); architect §7.9 audit views must
/// expose them for dashboard read.
pub fn audit_view_positions(econ: &EconomicState) -> PositionsView {
    let mut positions: Vec<PositionRow> = Vec::new();
    for position in econ.node_positions_t.0.values() {
        positions.push(PositionRow {
            node_id: position.node_id.clone(),
            holder: position.owner.clone(),
            side: position.side,
            kind: position.kind,
            amount_micro: position.amount.micro_units(),
        });
    }
    positions
        .sort_by(|a, b| a.node_id.0.cmp(&b.node_id.0).then_with(|| a.holder.0.cmp(&b.holder.0)));
    PositionsView { positions }
}

// ─────────────────────────────────────────────────────────────────────────
// Tests (lib unit; constitution-gate tests live in
// tests/constitution_audit_views.rs)
// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::q_state::CpmmPool;
    use crate::state::TaskId;

    #[test]
    fn shares_view_empty_econ_state() {
        let econ = EconomicState::default();
        let v = audit_view_shares(&econ);
        assert!(v.owner_shares.is_empty());
        assert!(v.conditional_collateral.is_empty());
    }

    #[test]
    fn pools_view_empty_econ_state() {
        let econ = EconomicState::default();
        let v = audit_view_pools(&econ);
        assert!(v.pools.is_empty());
        assert!(v.lp_holdings.is_empty());
    }

    #[test]
    fn prices_view_empty_econ_state() {
        let econ = EconomicState::default();
        let v = audit_view_prices(&econ, &[MicroCoin::from_micro_units(1_000_000)]);
        assert!(v.price_quotes.is_empty());
    }

    #[test]
    fn positions_view_empty_econ_state() {
        let econ = EconomicState::default();
        let v = audit_view_positions(&econ);
        assert!(v.positions.is_empty());
    }

    #[test]
    fn pools_view_active_pool_emits_row_with_k_product() {
        let mut econ = EconomicState::default();
        let event_id = EventId(TaskId("evt-1".into()));
        econ.cpmm_pools_t.0.insert(
            event_id.clone(),
            CpmmPool {
                event_id: event_id.clone(),
                pool_yes: ShareAmount::from_units(5_000_000),
                pool_no: ShareAmount::from_units(5_000_000),
                lp_total_shares: LpShareAmount::from_units(5_000_000),
                status: PoolStatus::Active,
            },
        );
        let v = audit_view_pools(&econ);
        assert_eq!(v.pools.len(), 1);
        assert_eq!(v.pools[0].pool_yes_units, 5_000_000);
        assert_eq!(v.pools[0].k_product, 25_000_000_000_000_u128);
    }
}
