use turingosv4::runtime::market_tx_category::{
    classify_market_tx, real10_category_counts, MarketTxCategory, MarketTxProvenance,
};

#[test]
fn market_tx_classifier_separates_structural_resolution_scripted_and_live_agent_action() {
    assert_eq!(
        classify_market_tx("MarketSeedTx", MarketTxProvenance::System),
        MarketTxCategory::StructuralMarketTx
    );
    assert_eq!(
        classify_market_tx("CpmmPoolTx", MarketTxProvenance::System),
        MarketTxCategory::StructuralMarketTx
    );
    assert_eq!(
        classify_market_tx("EventResolveTx", MarketTxProvenance::System),
        MarketTxCategory::ResolutionTx
    );
    assert_eq!(
        classify_market_tx("BuyWithCoinRouterTx", MarketTxProvenance::ScriptedFixture),
        MarketTxCategory::ScriptedFixtureTx,
        "scripted router fixtures must not count as E2"
    );
    assert_eq!(
        classify_market_tx("BuyWithCoinRouterTx", MarketTxProvenance::Missing),
        MarketTxCategory::ScriptedFixtureTx,
        "missing provenance must conservatively remain non-E2"
    );
    assert_eq!(
        classify_market_tx(
            "BuyWithCoinRouterTx",
            MarketTxProvenance::LiveAgentNonScripted {
                forced: false,
                prompt_or_trace_linked: true,
                chaintape_anchor: true,
                audit_proceed: true,
            },
        ),
        MarketTxCategory::AgentEconomicActionTx
    );
    assert_eq!(
        classify_market_tx(
            "BuyWithCoinRouterTx",
            MarketTxProvenance::LiveAgentNonScripted {
                forced: true,
                prompt_or_trace_linked: true,
                chaintape_anchor: true,
                audit_proceed: true,
            },
        ),
        MarketTxCategory::ScriptedFixtureTx,
        "forced live actions are non-E2"
    );
}

#[test]
fn real10_rerender_keeps_agent_economic_action_zero_and_excludes_scripted_from_e2() {
    let counts = real10_category_counts();

    assert_eq!(counts.arm("A").unwrap().agent_economic_action_tx_count, 0);
    assert_eq!(counts.arm("B").unwrap().agent_economic_action_tx_count, 0);
    assert_eq!(counts.arm("C").unwrap().agent_economic_action_tx_count, 0);
    assert_eq!(counts.arm("D").unwrap().agent_economic_action_tx_count, 0);

    assert_eq!(counts.arm("A").unwrap().market_tx_total(), 0);
    assert_eq!(counts.arm("B").unwrap().market_tx_total(), 10);
    assert_eq!(counts.arm("C").unwrap().market_tx_total(), 42);
    assert_eq!(counts.arm("D").unwrap().market_tx_total(), 38);
    assert!(
        counts.arm("D").unwrap().scripted_fixture_tx_count > 0,
        "Arm D scripted AttemptPrediction fixture must be split away from E2"
    );
}

#[test]
fn real11_market_tx_category_report_contains_split_columns_and_no_e2_overclaim() {
    let report = std::fs::read_to_string("handover/reports/REAL11_MARKET_TX_CATEGORY_REPORT.md")
        .expect("REAL-11 market tx category report must exist");

    for required in [
        "structural_market_tx_count",
        "agent_economic_action_tx_count",
        "scripted_fixture_tx_count",
        "resolution_tx_count",
        "buy_with_coin_router=0",
        "scripted AttemptPrediction fixture does not count as E2",
    ] {
        assert!(
            report.contains(required),
            "report missing required split/claim-boundary text: {required}"
        );
    }
}

#[test]
fn audit_dashboard_renders_real11_market_tx_category_split() {
    let source = std::fs::read_to_string("src/bin/audit_dashboard.rs")
        .expect("audit dashboard source exists");

    for required in [
        "## §C.1 REAL-11 Market tx categories",
        "structural_market_tx_count",
        "agent_economic_action_tx_count",
        "scripted_or_unproven_router_tx_count",
        "resolution_tx_count",
        "structural market activity is not E2",
        "scripted/unproven router tx is not E2",
    ] {
        assert!(
            source.contains(required),
            "dashboard missing REAL-11 split rendering: {required}"
        );
    }
}
