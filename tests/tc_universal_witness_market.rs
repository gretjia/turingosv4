#[path = "support/tc_universal_witness.rs"]
mod tc_universal_witness;

use tc_universal_witness::{
    verify_universal_witness, MarketSettlementWitness, UniversalWitnessKind, UniversalWitnessRun,
    WitnessError, WitnessExpectation,
};

#[test]
fn market_witness_requires_predicate_receipt_and_integer_conservation() {
    let mut run = UniversalWitnessRun::new(
        "a12-market",
        UniversalWitnessKind::MarketSettlement,
        "git-head-a12",
        WitnessExpectation::MarketSettled,
    );
    run.market_settlement = Some(MarketSettlementWitness {
        predicate_receipt_pass: true,
        before_total_micro: 10_000,
        after_total_micro: 10_000,
    });

    let verified = verify_universal_witness(&run).expect("market witness verifies");

    assert!(verified.market_settlement_checked);
}

#[test]
fn market_witness_fails_closed_on_missing_predicate_pass() {
    let mut run = UniversalWitnessRun::new(
        "a12-market-negative",
        UniversalWitnessKind::MarketSettlement,
        "git-head-a12",
        WitnessExpectation::MarketSettled,
    );
    run.market_settlement = Some(MarketSettlementWitness {
        predicate_receipt_pass: false,
        before_total_micro: 10_000,
        after_total_micro: 10_000,
    });

    let err = verify_universal_witness(&run).expect_err("predicate PASS is required");

    assert_eq!(err, WitnessError::PredicateReceiptNotPass);
}
