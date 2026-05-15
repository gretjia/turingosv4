//! `MicroCoin(i64)` — v4 monetary unit per Plan v3.2-fix3 CO1.0a + STATE_TRANSITION_SPEC v1.3.
//!
//! Constitution authority:
//! - Laws 基本法 1 (Coin 守恒): monetary conservation MUST be exact
//! - Inv 3 (escrow only): payouts come from pre-locked escrow; integer arithmetic prevents drift
//! - Inv 4 (no post-init mint): mint API guarded; only genesis sets initial supply
//!
//! Spec authority:
//! - STATE_TRANSITION_SPEC v1.3 § 1 typed schemas — all monetary fields are MicroCoin
//! - § 2 hidden-input table: f64 BANNED in `src/economy/`
//! - § 3.4 finalize_reward stage 3c royalty math: `royalty_micro = reward_micro * weight_micro / 1_000_000` (integer floor)
//!
//! Unit: 1 MicroCoin = 10⁻⁶ base coin. Range: i64 = ±9.2 × 10¹⁸ micro = ±9.2 × 10¹² base coin.
//!
//! Design:
//! - Newtype around i64 to prevent accidental mixing with u64/u32/f64
//! - All arithmetic returns Option (checked); panics not allowed in production paths
//! - Display formats as base.fraction (e.g., "12.345678 coin")
//! - serde-compatible for L4 transition_tx serialization
//! - Hash + Ord + Eq for use as BTreeMap key (per § 2 I-BTREE)
//!
//! /// TRACE_MATRIX I-MICROCOIN + Inv-3 + Inv-4: monetary type for v4

use serde::{Deserialize, Serialize};
use std::fmt;

/// A monetary value in micro-coin (10⁻⁶ base coin) as a signed 64-bit integer.
///
/// Negative values are allowed at the type level (e.g., signed deltas in tests),
/// but balance / escrow / stake fields enforce non-negative invariants at the
/// business logic layer (not in this type).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(transparent)]
pub struct MicroCoin(i64);

/// 1 base coin in micro-units (= 10⁶).
pub const MICRO_PER_COIN: i64 = 1_000_000;

impl MicroCoin {
    /// Construct from raw micro-units (signed).
    pub const fn from_micro_units(micro: i64) -> Self {
        Self(micro)
    }

    /// Construct from whole base coin (multiplied by `MICRO_PER_COIN`); `None` on overflow.
    pub fn from_coin(coin: i64) -> Option<Self> {
        coin.checked_mul(MICRO_PER_COIN).map(Self)
    }

    /// Zero (additive identity).
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Get raw micro-units (signed i64).
    pub const fn micro_units(self) -> i64 {
        self.0
    }

    /// Whole base-coin component (truncates toward zero).
    pub const fn coin_component(self) -> i64 {
        self.0 / MICRO_PER_COIN
    }

    /// Fractional micro component in `[-999_999, 999_999]` (sign matches whole).
    pub const fn micro_fraction_component(self) -> i64 {
        self.0 % MICRO_PER_COIN
    }

    /// Checked addition. `None` on overflow.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// Checked subtraction. `None` on overflow.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// Royalty / weight multiplication with integer-floor rounding.
    ///
    /// Implements the spec § 3.4 stage 3c rule:
    ///     royalty_micro = reward_micro × weight_micro_fraction / 1_000_000
    ///
    /// `weight_micro_fraction` is interpreted as a rational in `[0.0, 1.0]`
    /// scaled to micro-units (`1_000_000` = 1.0). Returns `None` on overflow
    /// at the intermediate `reward_micro × weight` product step.
    ///
    /// Determinism: integer floor (`a / b` rounds toward zero in Rust). For
    /// non-negative inputs this is round-down (floor). Negative values are
    /// rejected (returns `None`) to keep monetary math non-negative-by-default.
    pub fn checked_mul_floor_micro(self, weight_micro_fraction: i64) -> Option<Self> {
        if self.0 < 0 || weight_micro_fraction < 0 {
            return None;
        }
        if weight_micro_fraction > MICRO_PER_COIN {
            // weight > 1.0 not allowed at type level
            return None;
        }
        let prod = self.0.checked_mul(weight_micro_fraction)?;
        Some(Self(prod / MICRO_PER_COIN))
    }

    /// True if value is zero.
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// True if value is strictly positive.
    pub const fn is_positive(self) -> bool {
        self.0 > 0
    }

    /// True if value is strictly negative.
    pub const fn is_negative(self) -> bool {
        self.0 < 0
    }
}

impl fmt::Display for MicroCoin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let coin = self.0 / MICRO_PER_COIN;
        let frac = (self.0 % MICRO_PER_COIN).abs();
        if self.0 < 0 && coin == 0 {
            write!(f, "-0.{:06} coin", frac)
        } else {
            write!(f, "{}.{:06} coin", coin, frac)
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// StakeMicroCoin — newtype for stake fields per CO1.1.4-pre1 § 3
// ────────────────────────────────────────────────────────────────────────────

/// Newtype on `MicroCoin` for `WorkTx::stake`, `VerifyTx::bond`, `ChallengeTx::stake`
/// fields. Non-negative is a runtime invariant per Inv 3 (escrow only); the
/// type-level newtype prevents accidental mixing with general-purpose
/// `MicroCoin` (e.g. crediting a balance with a stake amount or vice versa).
///
/// `#[serde(transparent)]` — wire format identical to `MicroCoin`, so adding
/// the newtype is non-breaking for canonical encoding.
///
/// /// TRACE_MATRIX I-MICROCOIN + I-STAKE: stake-typed monetary newtype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StakeMicroCoin(pub MicroCoin);

impl Default for StakeMicroCoin {
    fn default() -> Self {
        Self::zero()
    }
}

impl StakeMicroCoin {
    pub const fn from_micro_units(micro: i64) -> Self {
        Self(MicroCoin::from_micro_units(micro))
    }
    pub const fn zero() -> Self {
        Self(MicroCoin::zero())
    }
    pub const fn micro_units(self) -> i64 {
        self.0.micro_units()
    }
    pub const fn as_micro_coin(self) -> MicroCoin {
        self.0
    }
}

impl fmt::Display for StakeMicroCoin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "stake({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_coin_round_trip() {
        let m = MicroCoin::from_coin(5).unwrap();
        assert_eq!(m.micro_units(), 5_000_000);
        assert_eq!(m.coin_component(), 5);
        assert_eq!(m.micro_fraction_component(), 0);
    }

    #[test]
    fn from_micro_units_zero() {
        let m = MicroCoin::from_micro_units(0);
        assert!(m.is_zero());
        assert!(!m.is_positive());
        assert!(!m.is_negative());
    }

    #[test]
    fn checked_add_normal() {
        let a = MicroCoin::from_coin(10).unwrap();
        let b = MicroCoin::from_coin(5).unwrap();
        let sum = a.checked_add(b).unwrap();
        assert_eq!(sum, MicroCoin::from_coin(15).unwrap());
    }

    #[test]
    fn checked_sub_normal() {
        let a = MicroCoin::from_coin(10).unwrap();
        let b = MicroCoin::from_coin(3).unwrap();
        let diff = a.checked_sub(b).unwrap();
        assert_eq!(diff, MicroCoin::from_coin(7).unwrap());
    }

    #[test]
    fn checked_add_overflow_returns_none() {
        let a = MicroCoin::from_micro_units(i64::MAX);
        let b = MicroCoin::from_micro_units(1);
        assert_eq!(a.checked_add(b), None);
    }

    #[test]
    fn from_coin_overflow_returns_none() {
        // i64::MAX / MICRO_PER_COIN ≈ 9.2 × 10¹²; one above causes overflow
        let big = i64::MAX / MICRO_PER_COIN + 1;
        assert_eq!(MicroCoin::from_coin(big), None);
    }

    #[test]
    fn royalty_10_percent_rounds_down() {
        // 500 base coin × 0.10 = 50 base coin (exact)
        let reward = MicroCoin::from_coin(500).unwrap();
        let weight = 100_000; // 0.10 in micro fraction
        let royalty = reward.checked_mul_floor_micro(weight).unwrap();
        assert_eq!(royalty, MicroCoin::from_coin(50).unwrap());
    }

    #[test]
    fn royalty_floor_dust() {
        // 1 base coin × 0.333333 should floor (not round up)
        // 1_000_000 × 333_333 = 333_333_000_000; / 1_000_000 = 333_333 micro
        let reward = MicroCoin::from_coin(1).unwrap();
        let weight = 333_333; // 0.333333 in micro fraction
        let royalty = reward.checked_mul_floor_micro(weight).unwrap();
        assert_eq!(royalty.micro_units(), 333_333);
    }

    #[test]
    fn royalty_rejects_negative() {
        let reward = MicroCoin::from_micro_units(-100);
        assert_eq!(reward.checked_mul_floor_micro(500_000), None);

        let pos = MicroCoin::from_coin(1).unwrap();
        assert_eq!(pos.checked_mul_floor_micro(-100_000), None);
    }

    #[test]
    fn royalty_rejects_weight_above_1() {
        let reward = MicroCoin::from_coin(100).unwrap();
        // weight > 1.0 (i.e., > 1_000_000 micro) not allowed
        assert_eq!(reward.checked_mul_floor_micro(MICRO_PER_COIN + 1), None);
    }

    #[test]
    fn display_zero() {
        assert_eq!(MicroCoin::zero().to_string(), "0.000000 coin");
    }

    #[test]
    fn display_positive() {
        let m = MicroCoin::from_micro_units(12_345_678);
        assert_eq!(m.to_string(), "12.345678 coin");
    }

    #[test]
    fn ordering_for_btreemap() {
        // I-BTREE invariant: MicroCoin must be Ord for use as BTreeMap value/key.
        let mut v = vec![
            MicroCoin::from_coin(3).unwrap(),
            MicroCoin::from_coin(1).unwrap(),
            MicroCoin::from_coin(2).unwrap(),
        ];
        v.sort();
        assert_eq!(v[0], MicroCoin::from_coin(1).unwrap());
        assert_eq!(v[2], MicroCoin::from_coin(3).unwrap());
    }

    #[test]
    fn serde_round_trip_json() {
        let m = MicroCoin::from_micro_units(123_456_789);
        let s = serde_json::to_string(&m).unwrap();
        let m2: MicroCoin = serde_json::from_str(&s).unwrap();
        assert_eq!(m, m2);
    }

    #[test]
    fn serde_transparent_format() {
        // #[serde(transparent)] means MicroCoin serializes as the inner i64
        let m = MicroCoin::from_micro_units(42);
        let s = serde_json::to_string(&m).unwrap();
        assert_eq!(s, "42");
    }

    #[test]
    fn conservation_law_basic() {
        // Direct test of Inv 3 monetary conservation: redistribute 500 coin
        // among 3 parties; total in == total out.
        let total_in = MicroCoin::from_coin(500).unwrap();
        let alice_share = MicroCoin::from_coin(300).unwrap();
        let bob_share = MicroCoin::from_coin(150).unwrap();
        let dust_share = MicroCoin::from_coin(50).unwrap();

        let total_out = alice_share
            .checked_add(bob_share)
            .unwrap()
            .checked_add(dust_share)
            .unwrap();
        assert_eq!(total_in, total_out, "Inv 3 conservation");
    }
}
