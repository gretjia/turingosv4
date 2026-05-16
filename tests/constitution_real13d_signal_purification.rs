//! REAL-13D — report-side signal purification gate.

use turingosv4::runtime::signal_purification::{classify_economic_signal, EconomicSignalClass};

#[test]
fn forced_stakes_are_not_voluntary_market_signal() {
    assert_eq!(
        classify_economic_signal("work"),
        EconomicSignalClass::CreatorBond
    );
    assert_eq!(
        classify_economic_signal("challenge"),
        EconomicSignalClass::ChallengeBond
    );
    assert_eq!(
        classify_economic_signal("verify"),
        EconomicSignalClass::VerificationBond
    );
    assert_eq!(
        classify_economic_signal("buy_with_coin_router"),
        EconomicSignalClass::VoluntaryMarketPosition
    );
}

#[test]
fn e2_counts_only_voluntary_router_positions() {
    assert!(!classify_economic_signal("work").counts_for_e2());
    assert!(!classify_economic_signal("challenge").counts_for_e2());
    assert!(!classify_economic_signal("market_seed").counts_for_e2());
    assert!(classify_economic_signal("buy_with_coin_router").counts_for_e2());
}
