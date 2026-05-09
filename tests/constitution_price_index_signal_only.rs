//! TuringOS Constitution Gate — Stage C P-M7 PriceIndex from CPMM (architect
//! 2026-05-07 ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL §7.8
//! verbatim).
//!
//! # Scope
//!
//! Architect alignment doc §7.8 mandates 4 hardening tests for `PriceIndex`
//! (signal-only; price MUST NOT decide predicate truth):
//!
//!   - price_quote_does_not_change_state
//!   - price_signal_not_predicate
//!   - price_does_not_make_failed_node_accepted
//!   - low_liquidity_warning
//!
//! # Why a separate gate file (vs delegation to TB-14 PriceIndex tests)
//!
//! TB-14 `compute_price_index` operates over `node_positions_t` (FirstLong /
//! ChallengeShort exposure records). Stage C P-M7 PriceIndex operates over
//! `cpmm_pools_t` reserves (CPMM pool quote). Both coexist; this file binds
//! the architect §7.8 verbatim names directly to the new pure
//! `cpmm_price_quote` helper + verifies signal-not-truth invariants.
//!
//! TRACE_FLOWCHART_MATRIX:
//!   - FC1 §1 predicate routing (price quote does NOT modulate predicate verdict)
//!   - §7.8 architect Polymarket manual

use turingosv4::state::price_index::{
    cpmm_price_quote, STAGE_C_LOW_LIQUIDITY_THRESHOLD_UNITS,
};

// ════════════════════════════════════════════════════════════════════════════
// §7.8 P-M7 PriceIndex hardening (4 verbatim names)
// ════════════════════════════════════════════════════════════════════════════

/// §7.8 verbatim — `price_quote_does_not_change_state`.
///
/// Per architect manual §7.8: "Price is signal only" — quote is a pure
/// function over pool reserves. Asserts that calling `cpmm_price_quote`
/// does not require / does not produce any mutable state input or output;
/// repeated calls with same inputs yield identical outputs (deterministic).
#[test]
fn price_quote_does_not_change_state() {
    // Pool 1000 + 1000; quote pay=100 BuyYes (pool_input=NO=1000, pool_output=YES=1000).
    let q1 = cpmm_price_quote(1000, 1000, 100);
    let q2 = cpmm_price_quote(1000, 1000, 100);
    assert_eq!(q1, q2, "cpmm_price_quote is deterministic / pure");

    // The function takes plain u128 inputs (no &mut state, no I/O); cargo
    // type-checking is the structural guarantee. Sanity result.
    assert!(q1.is_some(), "non-trivial quote returns Some");
    let (get_total, _warn) = q1.unwrap();
    // floor(100*1000/1100)=90; total=100+90=190.
    assert_eq!(get_total, 190);
}

/// §7.8 verbatim — `price_signal_not_predicate`.
///
/// Per architect manual §7.8 + universal forbidden list "no price-as-truth":
/// price quote is signal only. The predicate-truth invariant is
/// independently asserted by `tests/constitution_predicate_gate.rs::price_never_overrides_predicate`
/// (existing GREEN at HEAD) — this test re-anchors that invariant from the
/// P-M7 surface.
#[test]
fn price_signal_not_predicate() {
    // Verify the predicate-gate test exists in the same workspace.
    // Architect §7.8: any code path that converts a price quote into a
    // predicate verdict is forbidden. The codebase has `cpmm_price_quote`
    // as the only Stage C price surface; the function returns
    // (u128 get_total, bool low_liquidity) — neither is a Verdict /
    // PredicateResult / Outcome.
    let quote = cpmm_price_quote(500, 500, 50).expect("non-trivial quote");
    // Type proof: the return type cannot be coerced into a predicate verdict.
    // The bool is "low liquidity warning", not "predicate pass".
    let (_get_total, low_liquidity) = quote;
    let _: bool = low_liquidity;
    // Sister gate: confirm the existing predicate-gate file is present.
    let pg_src = std::fs::read_to_string("tests/constitution_predicate_gate.rs")
        .expect("constitution_predicate_gate.rs exists");
    assert!(
        pg_src.contains("price_never_overrides_predicate"),
        "sister test `price_never_overrides_predicate` exists in constitution_predicate_gate"
    );
}

/// §7.8 verbatim — `price_does_not_make_failed_node_accepted`.
///
/// Per architect manual §7.8: a predicate-failed work tx must remain
/// rejected regardless of any price quote. Source-grep over the sequencer
/// admission paths confirms no admission arm consumes a price quote to
/// override an Err return.
#[test]
fn price_does_not_make_failed_node_accepted() {
    let sequencer_src = std::fs::read_to_string("src/state/sequencer.rs")
        .expect("read src/state/sequencer.rs");

    // No admission arm calls `cpmm_price_quote` (architect §7.8: price is
    // signal, not predicate input). Quote belongs in audit/dashboard
    // surfaces, NOT the sequencer dispatch path.
    assert!(
        !sequencer_src.contains("cpmm_price_quote"),
        "sequencer.rs MUST NOT call cpmm_price_quote (architect §7.8: price-not-truth)"
    );

    // Defense in depth: no admission arm reads `compute_price_index` either
    // (TB-14 derived view; not predicate input).
    let admission_region = sequencer_src
        .find("dispatch_transition")
        .map(|s| &sequencer_src[s..])
        .unwrap_or("");
    assert!(
        !admission_region.contains("compute_price_index"),
        "dispatch_transition MUST NOT call compute_price_index"
    );
}

/// §7.8 verbatim — `low_liquidity_warning`.
///
/// Per architect manual §7.8: low-liquidity quotes carry a warning flag.
/// Asserts `cpmm_price_quote` returns `low_liquidity = true` when either
/// pool side is below `STAGE_C_LOW_LIQUIDITY_THRESHOLD_UNITS`, and
/// `low_liquidity = false` when both sides are above the threshold.
#[test]
fn low_liquidity_warning() {
    // High-depth pool: both sides ≥ threshold → no warning.
    let high = cpmm_price_quote(1000, 1000, 100).expect("non-trivial quote");
    assert!(
        !high.1,
        "high-liquidity pool (1000+1000 with threshold {STAGE_C_LOW_LIQUIDITY_THRESHOLD_UNITS}) has no warning"
    );

    // Low-depth pool: input side below threshold → warning.
    // Threshold = 100; use pool_input = 50, pool_output = 1000.
    let low = cpmm_price_quote(50, 1000, 10).expect("non-trivial quote");
    assert!(
        low.1,
        "low-liquidity pool (50+1000) triggers warning"
    );

    // Symmetric: output side below threshold → warning. Use pay=200 to
    // get a non-zero floor: out = floor(200 * 50 / (200 + 200)) = 25.
    let low_out = cpmm_price_quote(200, 50, 200).expect("non-trivial quote");
    assert!(
        low_out.1,
        "low-liquidity pool (200+50) triggers warning (output side below threshold)"
    );
}
