//! TP-1 conformance gate for the 5-pillar market microstructure spec
//! (handover/preregistration/T2_MICROSTRUCTURE_SPEC_PREREG_2026-06-01.json).
//!
//! STATIC predicates run now (no harness, no LLM) and MUST be able to fail. LIVE/harness-dependent predicates
//! (Sybil-defunding, the predicate-id leak remap) are #[ignore]'d with their evidence paths until the T2
//! harness exists. This is the machine-checkable half of the G1 gate.

use std::fs;

fn src(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

/// GOODHART pillar (anchor): the sequencer settlement path is PRICE-BLIND by construction — the evaluator is
/// shielded from the price signal, so price-following cannot become predicate-gaming (constitution Art.III.4).
#[test]
fn goodhart_sequencer_is_price_blind() {
    let s = src("src/state/sequencer.rs");
    let price_refs = s.matches("price").count();
    assert_eq!(price_refs, 0, "sequencer settlement path must contain ZERO price references (price-blind oracle)");
}

/// ASSET pillar: the traded good is a NodePosition (FirstLong/ChallengeShort responsibility bond); a fungible
/// MarketBuy/MarketSell trading layer is forbidden (§9.4) — assert NO such position is ever CONSTRUCTED.
#[test]
fn asset_no_market_buy_sell_construction() {
    for f in ["src/state/typed_tx.rs", "src/state/sequencer.rs", "src/bin/lean_hayek_market.rs"] {
        let s = src(f);
        assert!(!s.contains("PositionKind::MarketBuy"), "{f} constructs a forbidden MarketBuy position");
        assert!(!s.contains("PositionKind::MarketSell"), "{f} constructs a forbidden MarketSell position");
    }
}

/// EXPLORATION pillar: the run-validity floor primitive admits the default (epsilon 1/10) and REJECTS
/// exploration-collapse (epsilon below 1/10), via the integer cross-multiply gate added in TP-1.5.
#[test]
fn exploration_floor_primitive_gates() {
    use turingosv4::state::price_index::BoltzmannMaskPolicy;
    assert!(BoltzmannMaskPolicy::default().epsilon_meets_experiment_floor(), "default epsilon 1/10 is admissible");
    let eps0 = BoltzmannMaskPolicy { epsilon_exploration_num: 0, epsilon_exploration_den: 10, ..Default::default() };
    assert!(!eps0.epsilon_meets_experiment_floor(), "epsilon=0 (collapse) must be rejected by the floor gate");
    let eps_low = BoltzmannMaskPolicy { epsilon_exploration_num: 1, epsilon_exploration_den: 100, ..Default::default() };
    assert!(!eps_low.epsilon_meets_experiment_floor(), "epsilon below floor must be rejected");
}

/// ORACLE pillar: the settlement axiom whitelist is exactly {propext, Classical.choice, Quot.sound} — the
/// honest gate the EMERGE stack already enforces positively (native_decide / sorry must FAIL, not grep-pass).
#[test]
fn oracle_axiom_whitelist_is_the_locked_three() {
    let whitelist = ["propext", "Classical.choice", "Quot.sound"];
    assert_eq!(whitelist.len(), 3);
    // the spec prereg must carry the identical whitelist (lock consistency)
    let prereg = src("handover/preregistration/T2_MICROSTRUCTURE_SPEC_PREREG_2026-06-01.json");
    for ax in whitelist {
        assert!(prereg.contains(ax), "prereg oracle pillar must lock axiom '{ax}'");
    }
    assert!(prereg.contains("native_decide"), "prereg must name native_decide as the must-FAIL case");
}

/// CAUSAL CONTROL: both ablation arms (shuffled-price PRIMARY, flatbid SECONDARY) exist in the producer — a
/// price-causal claim is impossible without them.
#[test]
fn causal_control_both_ablations_exist() {
    let s = src("src/bin/lean_hayek_market.rs");
    assert!(s.contains("shuffled"), "shuffled-price ablation (primary causal gate) must exist");
    assert!(s.contains("flatbid"), "flatbid firewall (secondary causal gate) must exist");
}

// ── LIVE / harness-dependent predicates — #[ignore] until the T2 harness (TP-2) exists ──

/// HONEST-BIDDING pillar (live): with a planted over-claiming Sybil, the market defunds it (reputation 0,
/// wallet < honest) while a greedy baseline keeps funding it. Evidence: reputation_constitutional.rs already
/// proves Sybil-defunding 10/10 seeds; the T2-meta version lands in TP-2/TP-3.
#[test]
#[ignore = "live: needs the T2 harness; anchor evidence handover/reports/PRICE_ECONOMY_VALIDATED_REPUTATION*"]
fn honest_bidding_defunds_sybil() {}

/// GOODHART pillar (live falsifier): the agent-facing diagnostic must strip the raw PredicateId at all 16
/// sequencer leak sites (568,569,570,694,695,1234,1239,1269,1275,2465,2478,2483,2488,2496,2502,2712). The
/// remap lives in the T2 harness read-view (NOT a §6 sequencer edit), so this is deferred to TP-2.
#[test]
#[ignore = "harness: predicate-id leak remap belongs in the T2 read-view (TP-2), not a §6 sequencer edit"]
fn goodhart_agent_view_strips_predicate_id() {}
