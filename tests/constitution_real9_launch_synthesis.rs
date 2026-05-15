//! REAL-9 launch synthesis gates.

use std::fs;

#[test]
fn real9_whitepaper_update_preserves_required_architect_claims() {
    let doc = fs::read_to_string(
        "handover/whitepapers/TURINGOS_GENERATIVE_ECONOMY_WHITEPAPER_UPDATE_REAL9.md",
    )
    .expect("REAL-9 whitepaper update exists");

    for expected in [
        "v4 does not copy v3.",
        "v4 rebuilds v3's economic pressure under constitution.",
        "price = signal, not truth.",
        "market = role-specific institution, not prompt decoration.",
        "v3 taught us pressure.",
        "v4 gives us law.",
        "Next phase must build lawful pressure.",
    ] {
        assert!(
            doc.contains(expected),
            "REAL-9 whitepaper update must preserve architect line: {expected}"
        );
    }
}

#[test]
fn real9_market_developer_manual_preserves_lawful_market_boundary() {
    let doc = fs::read_to_string("handover/whitepapers/TURINGOS_MARKET_DEVELOPER_MANUAL_REAL9.md")
        .expect("REAL-9 market developer manual exists");

    for expected in [
        "v4 does not copy v3.",
        "v4 rebuilds v3's economic pressure under constitution.",
        "price = signal, not truth.",
        "market = role-specific institution, not prompt decoration.",
        "no forced trades",
        "no price-as-truth",
        "no ghost liquidity",
        "no f64 economy",
        "no off-tape WAL as truth",
        "no private CoT recording",
        "no raw-log broadcast",
        "ChainTape/CAS-backed",
        "Dashboards and reports are materialized views",
    ] {
        assert!(
            doc.contains(expected),
            "REAL-9 market developer manual must preserve boundary: {expected}"
        );
    }
}
