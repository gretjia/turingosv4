//! REAL-10 Atom 3 — E1/E2/E3/E4 emergence-metric claim-boundary gates.

use std::fs;

#[test]
fn real10_emergence_metrics_define_e1_e2_e3_e4() {
    let doc = fs::read_to_string("handover/alignment/EMERGENCE_METRICS_E1_E2_E3_E4.md")
        .expect("metrics doc");
    for expected in [
        "E1 — Market Visibility",
        "E2 — Spontaneous Market Action",
        "E3 — Persistent Role Differentiation",
        "E4 — Causal Performance Signal",
        "MarketDecisionTrace / NoTradeReason is tape-visible",
        "live, non-scripted, agent-generated BuyWithCoinRouterTx",
        "Derived from ChainTape/CAS, not prompt labels",
        "statistically meaningful difference",
    ] {
        assert!(
            doc.contains(expected),
            "metrics doc must contain architect metric text: {expected}"
        );
    }
}

#[test]
fn real10_emergence_metrics_forbid_scripted_e2_label_only_e3_small_n_e4() {
    let doc = fs::read_to_string("handover/alignment/EMERGENCE_METRICS_E1_E2_E3_E4.md")
        .expect("metrics doc");
    for expected in [
        "Scripted actions cannot satisfy E2",
        "Role labels alone cannot satisfy E3",
        "Small-n descriptive evidence cannot claim E4",
        "Activity increase alone is not emergence",
        "market_tx_count increase alone",
    ] {
        assert!(
            doc.contains(expected),
            "metrics doc must block overclaim path: {expected}"
        );
    }
}

#[test]
fn real10_narrow_ratification_records_real8_numbers_and_nonclaims() {
    let ratification =
        fs::read_to_string("handover/directives/2026-05-15_REAL5S_REAL9_NARROW_RATIFICATION.md")
            .expect("ratification");
    for expected in [
        "| A | 3 | 0 | PROCEED | 0 | 2/3 |",
        "| B | 3 | 0 | PROCEED | 4 | 2/3 |",
        "| C | 3 | 0 | PROCEED | 10 | 2/3 |",
        "| D | 3 | 0 | PROCEED | 10 | 2/3 |",
        "activity increased",
        "no solve-rate separation",
        "does not claim",
        "live REAL-6B real-LLM AttemptPrediction approval",
    ] {
        assert!(
            ratification.contains(expected),
            "narrow ratification must preserve REAL-8 evidence boundary: {expected}"
        );
    }
}

#[test]
fn real10_execution_plan_preserves_live_real6b_deferral_packet() {
    let plan = fs::read_to_string(
        "handover/directives/2026-05-15_REAL10_CONTROLLED_MARKET_EVIDENCE_EXPANSION_EXECUTION_PLAN.md",
    )
    .expect("execution plan");
    for expected in [
        "REAL-10 must not run live real-LLM AttemptPrediction",
        "candidate timing",
        "market close timing",
        "oracle resolution order",
        "settlement semantics",
        "abort path",
        "replay invariants",
        "no price-as-truth proof",
        "sleep-based timing",
        "price affecting Lean verification",
        "market price deciding L4/L4.E",
    ] {
        assert!(
            plan.contains(expected),
            "execution plan must preserve REAL-6B deferral detail: {expected}"
        );
    }
}
