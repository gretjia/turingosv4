//! Stage C P-M7 / Phase F.6 — CPMM Mint-and-Swap router quote (architect manual §7.8).
//!
//! TRACE_MATRIX FC1-Append Stage C P-M7 (architect §7.8 verbatim):
//!
//! ```
//! price_yes_effective = quote_payC / quote_getY
//! price_no_effective  = quote_payC / quote_getN
//! ```
//!
//! Where `quote_getY = payC + outY` (BuyYesWithCoinRouter cumulative ledger
//! effect per architect §7.7 step 9). `outY = floor(payC * pool_yes /
//! (pool_no + payC))`. Symmetric for `getN` (BuyNo direction).
//!
//! **Architect §7.8 explicit gate**: "Do not use price to decide predicate
//! truth." Witnessed by `price_signal_not_predicate` source-grep test:
//! the sequencer admission arms (predicate-gating dispatch) MUST NOT
//! reference any function from this module. The quote is a derived view —
//! pure over `(&CpmmPool, MicroCoin) → Option<RouterQuote>` — and never
//! mutates state (witnessed by `price_quote_does_not_change_state` test).
//!
//! **Integer-rational only** (charter §5 forbidden + halt-trigger #4):
//! all arithmetic is u128 checked; no f64 / f32 / float casts. Reuses the
//! existing `RationalPrice { numerator, denominator }` from
//! `crate::state::price_index` (TB-14 Atom 2 architect §5.2 verbatim).
//!
//! **Replay-determinism** (Art.0.2): pure over caller-supplied pool ref +
//! pay_coin; no env / clock / RNG. Replayer constructs the same quote
//! given the same inputs.

use crate::economy::money::MicroCoin;
use crate::state::price_index::RationalPrice;
use crate::state::q_state::{CpmmPool, PoolStatus};
use crate::state::typed_tx::ShareAmount;

/// TRACE_MATRIX FC1-Append Stage C P-M7 / Phase F.6 (architect §7.8): swap
/// direction discriminator for the router quote API. Mirrors
/// `BuyDirection` from `typed_tx.rs` semantically — kept separate to
/// avoid cross-module coupling between the typed-tx admission ABI
/// (kernel-adjacent; STEP_B-restricted) and the view-only quote helper.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteDirection {
    /// Architect §7.7/§7.8 BuyYesWithCoinRouter quote.
    BuyYes,
    /// Architect §7.7/§7.8 BuyNoWithCoinRouter quote.
    BuyNo,
}

/// TRACE_MATRIX FC1-Append Stage C P-M7 / Phase F.6 (architect §7.8): low-
/// liquidity classification carried in the quote output. Pure marker;
/// callers may render or filter on this without changing canonical state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiquidityWarning {
    /// Pool reserves are sufficient; quote is meaningful.
    None,
    /// Pool input or other side is at-or-below the configurable
    /// `min_liquidity_units` threshold; quote is non-empty but should be
    /// flagged in dashboard / audit views.
    LowLiquidity,
    /// `out_shares == 0` — the floor formula returned zero output. The
    /// quote's `price_effective` is `None` because the implied price is
    /// undefined (would-be denominator `get_shares == pay_coin` only,
    /// no swap value extracted).
    NoOutput,
}

/// TRACE_MATRIX FC1-Append Stage C P-M7 / Phase F.6 (architect §7.8): output
/// of `quote_buy_with_coin_router`. Reports the architect §7.8 effective
/// price + the swap intermediates (out_shares from CPMM formula +
/// get_shares = pay_coin + out_shares cumulative) + a low-liquidity
/// warning marker.
///
/// `price_effective` is `Option<RationalPrice>` — `None` when liquidity
/// warning is `NoOutput` OR pool is degenerate (pre-mutation guard
/// catches this). Otherwise `numerator = pay_coin.micro_units` and
/// `denominator = get_shares.units` (both u128). The fraction is in
/// `(0, 1]` for healthy quotes (payC ≤ getY because outY ≥ 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouterQuote {
    /// Echo of caller's `pay_coin` input.
    pub pay_coin: MicroCoin,
    /// Direction echoed back (BuyYes / BuyNo).
    pub direction: QuoteDirection,
    /// Computed `out_shares = floor(pay_coin.micro * pool_other /
    /// (pool_input + pay_coin.micro))` per architect §7.7 / §7.8.
    /// Zero when input is too small relative to pool ratio (low
    /// liquidity); cf. `LiquidityWarning::NoOutput`.
    pub out_shares: ShareAmount,
    /// `get_shares = pay_coin.micro + out_shares` (architect §7.7 step 9
    /// cumulative ledger effect: buyer holds payC retained + outY swap
    /// output of the preferred side).
    pub get_shares: ShareAmount,
    /// Architect §7.8 verbatim `price_yes_effective = quote_payC /
    /// quote_getY` (or symmetric for `BuyNo`). `None` iff
    /// `LiquidityWarning::NoOutput` (would-be denominator zero or
    /// undefined). Integer-rational; no float.
    pub price_effective: Option<RationalPrice>,
    /// Liquidity classification — pure marker; never gates state mutation
    /// (cf. architect §7.8 "Price is signal only").
    pub liquidity_warning: LiquidityWarning,
}

/// TRACE_MATRIX FC1-Append Stage C P-M7 / Phase F.6 (architect §7.8 default
/// minimum-liquidity threshold): pools with input OR other side below
/// this many share units (default `1_000_000` = 1 Coin worth in micro-
/// units) are flagged `LowLiquidity`. The threshold is conservative; a
/// production deployment can override via `quote_buy_with_coin_router_with_
/// liquidity_threshold` if a different floor is desired.
pub const DEFAULT_MIN_LIQUIDITY_UNITS: u128 = 1_000_000;

/// TRACE_MATRIX FC1-Append Stage C P-M7 / Phase F.6 (architect §7.8): pure
/// quote function over `(&CpmmPool, MicroCoin, QuoteDirection)`.
///
/// Returns `None` when the quote cannot be computed at all:
/// - Pool status is not `Active` (Resolved / Closed pools — quote not meaningful).
/// - `pay_coin.micro_units() <= 0` (zero / negative input — degenerate; matches `RouterZeroPay` admission gate).
/// - Either pool reserve is zero (degenerate pool — should have been rejected by `InvalidPoolSeed` at creation; defense-in-depth).
///
/// Returns `Some(RouterQuote)` for any computable case (including
/// `out_shares == 0` floor — the quote carries a `NoOutput` warning).
///
/// **Architect §7.8 invariants enforced by this fn**:
/// - **Pure**: `&CpmmPool` (immutable ref); no state mutation possible.
/// - **Integer-only**: `u128` checked_add + checked_mul + integer division.
/// - **No predicate side-effect**: this fn is never called from
///   `dispatch_transition` (witnessed by `price_signal_not_predicate`
///   source-grep gate).
pub fn quote_buy_with_coin_router(
    pool: &CpmmPool,
    pay_coin: MicroCoin,
    direction: QuoteDirection,
) -> Option<RouterQuote> {
    quote_buy_with_coin_router_with_liquidity_threshold(
        pool,
        pay_coin,
        direction,
        DEFAULT_MIN_LIQUIDITY_UNITS,
    )
}

/// TRACE_MATRIX FC1-Append Stage C P-M7 / Phase F.6 (architect §7.8): same
/// as `quote_buy_with_coin_router` but accepts a caller-supplied
/// `min_liquidity_units` threshold for the `LowLiquidity` warning.
///
/// Threshold semantics: if EITHER `pool_input < min_liquidity_units` OR
/// `pool_other < min_liquidity_units`, the warning is `LowLiquidity`.
/// `0` disables the warning (only `NoOutput` remains as a possible
/// non-`None` warning).
pub fn quote_buy_with_coin_router_with_liquidity_threshold(
    pool: &CpmmPool,
    pay_coin: MicroCoin,
    direction: QuoteDirection,
    min_liquidity_units: u128,
) -> Option<RouterQuote> {
    // Refuse non-positive payC (matches admission `RouterZeroPay` rejection).
    if pay_coin.micro_units() <= 0 {
        return None;
    }
    // Refuse non-active pools (matches admission `RouterPoolNotActive`).
    if pool.status != PoolStatus::Active {
        return None;
    }
    // Refuse degenerate pool reserves (defense-in-depth; admission rejects
    // `InvalidPoolSeed` at pool creation).
    if pool.pool_yes.units == 0 || pool.pool_no.units == 0 {
        return None;
    }

    let pay_coin_units: u128 = pay_coin.micro_units() as u128;
    // Per direction: BuyYes input side is NO, other is YES; BuyNo mirror.
    let (pool_input_units, pool_other_units) = match direction {
        QuoteDirection::BuyYes => (pool.pool_no.units, pool.pool_yes.units),
        QuoteDirection::BuyNo => (pool.pool_yes.units, pool.pool_no.units),
    };

    let denom = pool_input_units.checked_add(pay_coin_units)?;
    let numer = pay_coin_units.checked_mul(pool_other_units)?;
    let out_units: u128 = numer / denom;

    // get_shares = pay_coin.micro + out_shares (architect §7.7 step 9).
    let get_units = pay_coin_units.checked_add(out_units)?;

    // Liquidity classification.
    let liquidity_warning = if out_units == 0 {
        LiquidityWarning::NoOutput
    } else if pool_input_units < min_liquidity_units
        || pool_other_units < min_liquidity_units
    {
        LiquidityWarning::LowLiquidity
    } else {
        LiquidityWarning::None
    };

    // price_effective = pay_coin.micro / get_shares (architect §7.8
    // verbatim). Integer rational; no float. None when out == 0
    // (NoOutput case: get_shares == pay_coin.micro; numerator == denominator
    // would imply price 1.0 which is meaningless when no swap value was
    // extracted).
    let price_effective = if matches!(liquidity_warning, LiquidityWarning::NoOutput) {
        None
    } else {
        Some(RationalPrice {
            numerator: pay_coin_units,
            denominator: get_units,
        })
    };

    Some(RouterQuote {
        pay_coin,
        direction,
        out_shares: ShareAmount::from_units(out_units),
        get_shares: ShareAmount::from_units(get_units),
        price_effective,
        liquidity_warning,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::q_state::LpShareAmount;
    use crate::state::typed_tx::EventId;
    use crate::state::TaskId;

    fn pool_5m_5m_active(event: &str) -> CpmmPool {
        CpmmPool {
            event_id: EventId(TaskId(event.into())),
            pool_yes: ShareAmount::from_units(5_000_000),
            pool_no: ShareAmount::from_units(5_000_000),
            lp_total_shares: LpShareAmount::from_units(5_000_000),
            status: PoolStatus::Active,
        }
    }

    #[test]
    fn quote_returns_some_for_healthy_pool() {
        let pool = pool_5m_5m_active("evt-A");
        let q = quote_buy_with_coin_router(
            &pool,
            MicroCoin::from_micro_units(1_000_000),
            QuoteDirection::BuyYes,
        )
        .expect("healthy quote");

        // outY = floor(1_000_000 * 5_000_000 / 6_000_000) = 833_333.
        assert_eq!(q.out_shares.units, 833_333);
        // get_shares = pay_coin + outY = 1_833_333.
        assert_eq!(q.get_shares.units, 1_833_333);
        // price_effective = pay_coin / get_shares = 1M / 1_833_333.
        let p = q.price_effective.expect("price computed");
        assert_eq!(p.numerator, 1_000_000);
        assert_eq!(p.denominator, 1_833_333);
        assert_eq!(q.liquidity_warning, LiquidityWarning::None);
    }

    #[test]
    fn quote_symmetric_for_buy_no_direction() {
        let pool = pool_5m_5m_active("evt-B");
        let q = quote_buy_with_coin_router(
            &pool,
            MicroCoin::from_micro_units(1_000_000),
            QuoteDirection::BuyNo,
        )
        .expect("healthy quote");

        // outN = floor(1M * 5M / 6M) = 833_333 (same as YES — symmetric pool).
        assert_eq!(q.out_shares.units, 833_333);
        assert_eq!(q.get_shares.units, 1_833_333);
    }

    #[test]
    fn quote_returns_none_on_zero_pay_coin() {
        let pool = pool_5m_5m_active("evt-C");
        assert!(quote_buy_with_coin_router(
            &pool,
            MicroCoin::zero(),
            QuoteDirection::BuyYes
        )
        .is_none());
    }

    #[test]
    fn quote_returns_none_on_resolved_pool() {
        let mut pool = pool_5m_5m_active("evt-D");
        pool.status = PoolStatus::Resolved;
        assert!(quote_buy_with_coin_router(
            &pool,
            MicroCoin::from_micro_units(1_000_000),
            QuoteDirection::BuyYes
        )
        .is_none());
    }

    #[test]
    fn quote_no_output_warning_when_floor_yields_zero() {
        // Asymmetric pool where dust input floors to zero: poolY = 1,
        // poolN = 1_000_000_000, pay_coin (BuyYes input is NO side) = 1.
        // outY = floor(1 * 1 / 1_000_000_001) = 0.
        let pool = CpmmPool {
            event_id: EventId(TaskId("evt-E".into())),
            pool_yes: ShareAmount::from_units(1),
            pool_no: ShareAmount::from_units(1_000_000_000),
            lp_total_shares: LpShareAmount::from_units(1),
            status: PoolStatus::Active,
        };
        let q = quote_buy_with_coin_router(
            &pool,
            MicroCoin::from_micro_units(1),
            QuoteDirection::BuyYes,
        )
        .expect("quote computed (some)");
        assert_eq!(q.out_shares.units, 0);
        assert!(q.price_effective.is_none(), "no-output → price None");
        assert_eq!(q.liquidity_warning, LiquidityWarning::NoOutput);
    }

    #[test]
    fn quote_low_liquidity_warning_below_threshold() {
        // Tiny pool: 100/100 (well below 1_000_000 default threshold).
        let pool = CpmmPool {
            event_id: EventId(TaskId("evt-F".into())),
            pool_yes: ShareAmount::from_units(100),
            pool_no: ShareAmount::from_units(100),
            lp_total_shares: LpShareAmount::from_units(100),
            status: PoolStatus::Active,
        };
        let q = quote_buy_with_coin_router(
            &pool,
            MicroCoin::from_micro_units(50),
            QuoteDirection::BuyYes,
        )
        .expect("quote computed");
        assert_eq!(q.liquidity_warning, LiquidityWarning::LowLiquidity);
        // out > 0, so price IS computed even though warning is LowLiquidity.
        assert!(q.out_shares.units > 0);
        assert!(q.price_effective.is_some());
    }
}
