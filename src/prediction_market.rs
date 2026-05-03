//! # LEGACY — TB-3..TB-10 Phase-3A Hayek bounty-market scaffolding
//!
//! TRACE_MATRIX TB-13 Atom 0.5 (architect 2026-05-03 ruling Part A §4.2):
//! This module is **legacy**, **not constitutional**, **not RSP-M**, and
//! **not production market path** in the post-2026-05 architect roadmap.
//! It is:
//!
//! - **NOT** the canonical YES/NO claim system. TB-13 introduces
//!   `CompleteSetMintTx` + `ConditionalShareBalances` for that.
//! - **NOT** the canonical price index. TB-14 introduces `PriceIndex`
//!   derived from `node_positions_t` long/short interest.
//! - **NOT** authorized for extension or new use sites.
//!
//! ## Constitutional non-compliance (forward-binding rules)
//!
//! - **f64 in money path** — every reserve / price / lp field below uses
//!   `f64`; the post-2026-05 architect directive forbids f64 in money /
//!   collateral / share path (TB-13 SG-13.0.2; CR-13 forbidden list).
//! - **automatic liquidity** via constant-product market-maker — TB-13
//!   forbidden list explicitly bans automatic liquidity / ghost
//!   liquidity / automatic YES+NO injection.
//! - **trading semantics** (`buy_yes` / `buy_no`) — TB-13 forbidden list
//!   bans MarketBuy / MarketSell / MarketOrderTx / MarketTradeTx; TB-14
//!   forbids price-as-truth / price-based settlement.
//!
//! ## Carry-forward
//!
//! Removal is a **TB-14 SHIP prerequisite** per
//! `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md`.
//! Retroactive deletion in TB-13 would break the production evaluator
//! (`experiments/minif2f_v4/src/bin/evaluator.rs:1323` calls
//! `bus.kernel.market_ticker(5)` on a non-empty `markets` HashMap when
//! the bus opens per-node markets via `bus.rs:327` `create_market(...)`)
//! and the bus-level bounty market wiring at `bus.rs:206`
//! `kernel.open_bounty_market(lp)` plus 10+ test files. That refactor is
//! out-of-scope per `feedback_no_retroactive_evidence_rewrite` and
//! architect §4.2 halting-trigger semantics, which target NEW TB-13 code,
//! not existing scaffolding.
//!
//! ## Forward-fence (in place)
//!
//! `tests/tb_13_legacy_cpmm_forward_fence.rs` enforces that NEW TB-13
//! modules cannot import this file's types or call its f64 trading API.
//! The fence is the SG-13.0.1 / SG-13.0.2 / SG-13.0.3 ship gate.

// Tier 0: CPMM Binary Market — pure math, no I/O
// Constitutional basis: Law 2 (1 Coin = 1 YES + 1 NO, CTF conservation)
// V3 lessons: V3L-41/42/43 (no post-genesis minting), V3L-44 (no fixed tax)

use serde::{Deserialize, Serialize};
use std::fmt;

// ── Core types ──────────────────────────────────────────────────

/// A binary prediction market using Constant Product Market Maker (CPMM).
///
/// Invariants (Law 2 — CTF Conservation):
/// - `yes_reserve * no_reserve = k` (constant product, never changes during trading)
/// - `yes_price + no_price = 1.0` (always)
/// - 1 Coin = 1 YES + 1 NO (physical conservation)
/// - Resolved markets cannot be traded
///
/// V3L-41/42/43: No post-genesis minting. The only way coins enter is at creation.
/// V3L-44: No fixed tax. Price discovery is purely market-driven.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryMarket {
    node_id: String,
    yes_reserve: f64,
    no_reserve: f64,
    k: f64,
    pub(crate) resolved: Option<bool>,
    lp_total: f64,
}

impl BinaryMarket {
    pub fn node_id(&self) -> &str { &self.node_id }
    pub fn yes_reserve(&self) -> f64 { self.yes_reserve }
    pub fn no_reserve(&self) -> f64 { self.no_reserve }
    pub fn k(&self) -> f64 { self.k }
    pub fn lp_total(&self) -> f64 { self.lp_total }
}

/// Outcome of a buy operation.
#[derive(Debug, Clone)]
pub struct BuyOutcome {
    pub shares_received: f64,
    pub new_yes_price: f64,
    pub new_no_price: f64,
}

// ── Implementation ──────────────────────────────────────────────

impl BinaryMarket {
    /// Create a new market for a given node.
    /// `lp_coins` are split equally: lp_coins → YES reserve + NO reserve.
    /// k = (lp_coins/2)^2
    ///
    /// This is the ONLY way reserves are initialized — no post-genesis injection.
    pub fn create(node_id: String, lp_coins: f64) -> Result<Self, MarketError> {
        if lp_coins <= 0.0 {
            return Err(MarketError::InvalidAmount("LP coins must be positive".into()));
        }

        let half = lp_coins / 2.0;
        Ok(BinaryMarket {
            node_id,
            yes_reserve: half,
            no_reserve: half,
            k: half * half,
            resolved: None,
            lp_total: 1.0,
        })
    }

    /// P(YES) = no_reserve / (yes_reserve + no_reserve)
    /// This IS the Bayesian probability that the market assigns to YES.
    pub fn yes_price(&self) -> f64 {
        self.no_reserve / (self.yes_reserve + self.no_reserve)
    }

    /// P(NO) = yes_reserve / (yes_reserve + no_reserve)
    pub fn no_price(&self) -> f64 {
        self.yes_reserve / (self.yes_reserve + self.no_reserve)
    }

    /// Buy YES shares.
    ///
    /// Mechanism (Law 2 compliant):
    /// 1. Mint `coins_in` YES + `coins_in` NO (conservation: 1 Coin = 1 YES + 1 NO)
    /// 2. Sell `coins_in` NO into the pool → receive extra YES from pool
    /// 3. Total YES = coins_in (minted) + yes_from_pool (swapped)
    pub fn buy_yes(&mut self, coins_in: f64) -> Result<BuyOutcome, MarketError> {
        self.check_tradeable()?;
        if coins_in <= 0.0 {
            return Err(MarketError::InvalidAmount("Must buy positive amount".into()));
        }

        // Step 1: Mint coins_in YES + coins_in NO
        // Step 2: Sell coins_in NO into pool
        let new_no_reserve = self.no_reserve + coins_in;
        let new_yes_reserve = self.k / new_no_reserve;
        let yes_from_pool = self.yes_reserve - new_yes_reserve;

        self.yes_reserve = new_yes_reserve;
        self.no_reserve = new_no_reserve;

        // Total YES shares = minted + swapped
        let total_yes = coins_in + yes_from_pool;

        Ok(BuyOutcome {
            shares_received: total_yes,
            new_yes_price: self.yes_price(),
            new_no_price: self.no_price(),
        })
    }

    /// Buy NO shares (symmetric to buy_yes).
    pub fn buy_no(&mut self, coins_in: f64) -> Result<BuyOutcome, MarketError> {
        self.check_tradeable()?;
        if coins_in <= 0.0 {
            return Err(MarketError::InvalidAmount("Must buy positive amount".into()));
        }

        let new_yes_reserve = self.yes_reserve + coins_in;
        let new_no_reserve = self.k / new_yes_reserve;
        let no_from_pool = self.no_reserve - new_no_reserve;

        self.yes_reserve = new_yes_reserve;
        self.no_reserve = new_no_reserve;

        let total_no = coins_in + no_from_pool;

        Ok(BuyOutcome {
            shares_received: total_no,
            new_yes_price: self.yes_price(),
            new_no_price: self.no_price(),
        })
    }

    /// Oracle resolution — irreversible.
    /// `yes_wins = true` means the proposition was verified (e.g., Lean proof accepted).
    pub fn resolve(&mut self, yes_wins: bool) -> Result<(), MarketError> {
        if self.resolved.is_some() {
            return Err(MarketError::AlreadyResolved);
        }
        self.resolved = Some(yes_wins);
        Ok(())
    }

    /// Redeem shares after resolution.
    /// If YES wins: 1 YES = 1 Coin, NO = 0
    /// If NO wins: 1 NO = 1 Coin, YES = 0
    pub fn redeem(&self, yes_shares: f64, no_shares: f64) -> Result<f64, MarketError> {
        match self.resolved {
            None => Err(MarketError::NotResolved),
            Some(true) => Ok(yes_shares),
            Some(false) => Ok(no_shares),
        }
    }

    fn check_tradeable(&self) -> Result<(), MarketError> {
        if self.resolved.is_some() {
            return Err(MarketError::AlreadyResolved);
        }
        Ok(())
    }
}

// ── Errors ──────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum MarketError {
    InvalidAmount(String),
    AlreadyResolved,
    NotResolved,
}

impl fmt::Display for MarketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarketError::InvalidAmount(msg) => write!(f, "Invalid amount: {}", msg),
            MarketError::AlreadyResolved => write!(f, "Market already resolved"),
            MarketError::NotResolved => write!(f, "Market not yet resolved"),
        }
    }
}

impl std::error::Error for MarketError {}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-9;

    fn assert_approx(a: f64, b: f64, msg: &str) {
        assert!((a - b).abs() < EPSILON, "{}: {} != {} (diff {})", msg, a, b, (a - b).abs());
    }

    #[test]
    fn test_create_market() {
        let m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
        assert_eq!(m.yes_reserve, 1000.0);
        assert_eq!(m.no_reserve, 1000.0);
        assert_eq!(m.k, 1_000_000.0);
        assert!(m.resolved.is_none());
    }

    #[test]
    fn test_initial_price_is_50_50() {
        let m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
        assert_approx(m.yes_price(), 0.5, "initial yes_price");
        assert_approx(m.no_price(), 0.5, "initial no_price");
    }

    #[test]
    fn test_prices_sum_to_one() {
        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
        // After various trades, prices must always sum to 1.0
        m.buy_yes(100.0).unwrap();
        assert_approx(m.yes_price() + m.no_price(), 1.0, "after buy_yes");

        m.buy_no(200.0).unwrap();
        assert_approx(m.yes_price() + m.no_price(), 1.0, "after buy_no");

        m.buy_yes(50.0).unwrap();
        assert_approx(m.yes_price() + m.no_price(), 1.0, "after second buy_yes");
    }

    #[test]
    fn test_constant_product_invariant() {
        // Law 2: k = yes_reserve * no_reserve must stay constant
        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
        let k_initial = m.k;

        m.buy_yes(100.0).unwrap();
        assert_approx(m.yes_reserve * m.no_reserve, k_initial, "k after buy_yes");

        m.buy_no(300.0).unwrap();
        assert_approx(m.yes_reserve * m.no_reserve, k_initial, "k after buy_no");
    }

    #[test]
    fn test_buy_yes_increases_yes_price() {
        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
        let price_before = m.yes_price();
        m.buy_yes(100.0).unwrap();
        assert!(m.yes_price() > price_before, "buy_yes should increase yes_price");
    }

    #[test]
    fn test_buy_no_increases_no_price() {
        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
        let price_before = m.no_price();
        m.buy_no(100.0).unwrap();
        assert!(m.no_price() > price_before, "buy_no should increase no_price");
    }

    #[test]
    fn test_no_trading_after_resolution() {
        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
        m.resolve(true).unwrap();
        assert!(matches!(m.buy_yes(10.0), Err(MarketError::AlreadyResolved)));
        assert!(matches!(m.buy_no(10.0), Err(MarketError::AlreadyResolved)));
    }

    #[test]
    fn test_no_double_resolution() {
        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
        m.resolve(true).unwrap();
        assert!(matches!(m.resolve(false), Err(MarketError::AlreadyResolved)));
    }

    #[test]
    fn test_pioneer_profit() {
        // Buy YES at 50%, resolve YES → profit
        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
        let outcome = m.buy_yes(100.0).unwrap();
        m.resolve(true).unwrap();
        let payout = m.redeem(outcome.shares_received, 0.0).unwrap();
        assert!(payout > 100.0, "Pioneer should profit: paid 100, got {}", payout);
    }

    #[test]
    fn test_assassin_profit() {
        // Buy NO (short), resolve NO → profit
        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
        let outcome = m.buy_no(100.0).unwrap();
        m.resolve(false).unwrap();
        let payout = m.redeem(0.0, outcome.shares_received).unwrap();
        assert!(payout > 100.0, "Assassin should profit: paid 100, got {}", payout);
    }

    #[test]
    fn test_redeem_requires_resolution() {
        let m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
        assert!(matches!(m.redeem(100.0, 0.0), Err(MarketError::NotResolved)));
    }

    #[test]
    fn test_reject_zero_or_negative_amounts() {
        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
        assert!(m.buy_yes(0.0).is_err());
        assert!(m.buy_yes(-10.0).is_err());
        assert!(m.buy_no(0.0).is_err());
        assert!(BinaryMarket::create("n1".into(), 0.0).is_err());
        assert!(BinaryMarket::create("n1".into(), -100.0).is_err());
    }

    #[test]
    fn test_ctf_conservation_1_coin_1_yes_1_no() {
        // Law 2: 1 Coin = 1 YES + 1 NO
        // After buying 100 coins worth of YES, the total YES+NO minted = 200 (100 each)
        // The buyer gets coins_in YES (minted) + extra YES from pool swap
        // The pool absorbs coins_in NO
        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
        let yes_before = m.yes_reserve;
        let no_before = m.no_reserve;

        let outcome = m.buy_yes(100.0).unwrap();

        // Pool YES decreased, pool NO increased by exactly coins_in
        let pool_yes_delta = yes_before - m.yes_reserve; // positive (YES left pool)
        let pool_no_delta = m.no_reserve - no_before;     // positive (NO entered pool)

        assert_approx(pool_no_delta, 100.0, "NO entering pool = coins_in");
        // Total YES received = minted (100) + from_pool
        assert_approx(outcome.shares_received, 100.0 + pool_yes_delta,
                      "total YES = minted + swapped");
    }

    #[test]
    fn test_multiple_traders_price_discovery() {
        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();

        // Multiple traders buy YES → price should climb
        for _ in 0..10 {
            m.buy_yes(50.0).unwrap();
        }
        assert!(m.yes_price() > 0.6, "Heavy YES buying should push price above 0.6, got {}", m.yes_price());

        // Counterparty buys NO → price should come back down
        for _ in 0..20 {
            m.buy_no(50.0).unwrap();
        }
        assert!(m.yes_price() < 0.5, "Heavy NO buying should push YES price below 0.5, got {}", m.yes_price());
    }
}
