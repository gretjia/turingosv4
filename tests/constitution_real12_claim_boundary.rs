//! REAL-12 Atom 0 — REAL-10/REAL-11 narrow claim boundary.
//!
//! These are documentation gates, but they are intentionally executable:
//! the ratification must not drift into E2/E3/E4, live REAL-6B, or scripted
//! trade overclaim.

#[test]
fn real12_real10_real11_ratification_is_narrow_and_explicit() {
    let ratification =
        include_str!("../handover/directives/2026-05-16_REAL10_REAL11_NARROW_RATIFICATION.md");

    for required in [
        "REAL-10 proves E1 only",
        "does not prove E2",
        "does not prove E3",
        "does not prove E4",
        "REAL-11 proves router scripted positive-control works",
        "buy_with_coin_router=0",
        "Scripted buys are not E2",
        "No live REAL-6B approval",
        "No forced trade",
        "No price-as-truth",
        "No ghost liquidity",
    ] {
        assert!(
            ratification.contains(required),
            "missing boundary: {required}"
        );
    }
}

#[test]
fn real12_plan_forbids_the_known_overclaims() {
    let plan = include_str!(
        "../handover/directives/2026-05-16_REAL12_ROLE_SPECIALIZED_ECONOMIC_AGENTS_EXECUTION_PLAN.md"
    );

    for forbidden in [
        "No forced trade as E2 evidence",
        "No price-as-truth",
        "No ghost liquidity",
        "No f64/f32 money",
        "No off-tape WAL as truth",
        "No private CoT recording",
        "No raw-log broadcast",
        "No dashboard/report as source of truth",
        "No live REAL-6B in REAL-12",
        "No sequencer / TypedTx / signing payload changes without separate Class-4 ratification",
    ] {
        assert!(
            plan.contains(forbidden),
            "missing forbidden item: {forbidden}"
        );
    }

    for overclaim in [
        "spontaneous market emergence achieved",
        "causal improvement achieved",
        "model ranking achieved",
        "live REAL-6B approved",
        "scripted buys count as E2",
    ] {
        assert!(
            !plan.contains(overclaim),
            "REAL-12 plan must not contain overclaim: {overclaim}"
        );
    }
}
