//! TB-G G2.2 — `audit_dashboard --run-report` §F NoTradeReason rows (Class 2).
//!
//! Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
//! §1 Module G2 atom G2.2.
//!
//! Directive: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
//! §G2 SG-G2.3 "NoTradeReason appears in dashboard and CAS".
//!
//! Ship gates:
//! - SG-G2.4 (charter §1 Module G2.2 row): "§F renders per-`NoTradeReason`
//!   count + submitted/traced ratio". This file binds the contract:
//!   - SG-G2.4.a — `total_traces` + `outcome[*]` + `submitted_vs_traced_ratio`
//!     all rendered on a non-empty fixture batch.
//!   - SG-G2.4.b — `## §F.A NoTradeReason exhaustive breakdown` block lists
//!     every variant in `NoTradeReason::ALL` (13 rows in stable insertion
//!     order), zeros included, so forward audits can grep any architect-spec
//!     variant name regardless of whether the live batch produced it.
//!   - SG-G2.4.c — `submitted_vs_traced_ratio` renders `n/a` on an empty
//!     fixture batch (zero-batch render safety).
//!   - SG-G2.4.d — integer-rational percent in the ratio row (CLAUDE.md §13
//!     no-f64-in-money-path discipline extended to user-facing ratios).
//!   - SG-G2.4.e — source-grep: the renderer used by the binary is the
//!     library helper, not a duplicate inline walker.
//!
//! `FC-trace: FC1-N5 + §17 dashboard / report standard — §F MarketDecisionTrace
//! summary materializes the architect §G2 13-variant taxonomy as a stable
//! column shape; SG-G2.4 binds the row contract at test time.`

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::market_decision_trace::{
    write_market_decision_trace_to_cas, MarketDecisionTrace, NoTradeReason,
};
use turingosv4::runtime::market_decision_trace_summary::MarketDecisionTraceSummary;
use turingosv4::state::q_state::{AgentId, TxId};
use turingosv4::state::typed_tx::BuyDirection;

const AUDIT_DASHBOARD_SRC: &str = "src/bin/audit_dashboard.rs";

fn fixture_cas() -> (tempfile::TempDir, CasStore) {
    let dir = tempfile::tempdir().expect("tempdir");
    let cas = CasStore::open(dir.path()).expect("cas open");
    (dir, cas)
}

fn put_no_trade(cas: &mut CasStore, agent: &str, reason: NoTradeReason, t: u64) {
    let trace = MarketDecisionTrace::no_trade(
        AgentId(agent.into()),
        None,
        None,
        None,
        reason,
        format!("fixture {}", reason.label()),
    );
    write_market_decision_trace_to_cas(cas, &trace, &format!("{}-{}", agent, t), t)
        .expect("cas put");
}

fn put_submitted(cas: &mut CasStore, agent: &str, t: u64) {
    let trace = MarketDecisionTrace::submitted(
        AgentId(agent.into()),
        TxId(format!("worktx-{agent}-{t}")),
        BuyDirection::BuyYes,
        500_000,
        TxId(format!("router-{agent}-{t}")),
        "fixture submitted",
    );
    write_market_decision_trace_to_cas(cas, &trace, &format!("{}-{}", agent, t), t)
        .expect("cas put");
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.4.a — total_traces + outcome[*] + submitted_vs_traced_ratio render
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2_4_a_total_traces_outcomes_and_ratio_rendered() {
    let (_dir, mut cas) = fixture_cas();
    put_submitted(&mut cas, "Agent_0", 1);
    put_submitted(&mut cas, "Agent_1", 2);
    put_no_trade(&mut cas, "Agent_2", NoTradeReason::NoPool, 3);
    put_no_trade(&mut cas, "Agent_3", NoTradeReason::RouterRejected, 4);
    let summary = MarketDecisionTraceSummary::compute_from_cas(&cas);
    let out = summary.render_section_f();
    assert!(out.contains("## §F MarketDecisionTrace summary"));
    assert!(
        out.contains("total_traces: 4"),
        "SG-G2.4.a: total_traces row missing/wrong: {out}"
    );
    assert!(out.contains("outcome[submitted] = 2"));
    assert!(out.contains("outcome[no_trade] = 2"));
    assert!(
        out.contains("submitted_vs_traced_ratio: 2/4 = 50%"),
        "SG-G2.4.a: ratio row missing/wrong: {out}"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.4.b — §F.A exhaustive 13-row stable breakdown (zeros included)
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2_4_b_exhaustive_thirteen_row_breakdown_present() {
    let (_dir, mut cas) = fixture_cas();
    // Only NoPool + NoPerceivedEdge present in the fixture batch; every
    // other variant must still render with count = 0 per architect §G2
    // SG-G2.3 stable-column contract.
    put_no_trade(&mut cas, "Agent_0", NoTradeReason::NoPool, 1);
    put_no_trade(&mut cas, "Agent_1", NoTradeReason::NoPerceivedEdge, 2);
    let summary = MarketDecisionTraceSummary::compute_from_cas(&cas);
    let out = summary.render_section_f();
    assert!(
        out.contains("## §F.A NoTradeReason exhaustive breakdown"),
        "SG-G2.4.b: §F.A section header missing: {out}"
    );
    // The 2 observed variants render non-zero rows.
    assert!(out.contains("  no_pool = 1"));
    assert!(out.contains("  no_perceived_edge = 1"));
    // The 11 un-observed variants each render exactly one zero row.
    for &reason in NoTradeReason::ALL {
        let label = reason.label();
        let needle = format!("  {} =", label);
        let occurrences = out.matches(&needle).count();
        // In the §F.A block every variant renders exactly once. The
        // dynamic "observed" breakdown may also reference the same label
        // for variants that ARE observed (NoPool / NoPerceivedEdge), so
        // those two appear up to 2× — bound by ≤ 2 + .A row.
        assert!(
            occurrences >= 1 && occurrences <= 3,
            "SG-G2.4.b: variant {label:?} row occurs {occurrences}× (expected 1..=3)"
        );
    }
    // Stable insertion order: verify NoPool appears before NoPerceivedEdge
    // in the §F.A block (NoPool index 5 vs NoPerceivedEdge index 11).
    let fa_section_start = out
        .find("## §F.A NoTradeReason exhaustive breakdown")
        .expect("§F.A section");
    let fa_slice = &out[fa_section_start..];
    let pos_no_pool = fa_slice.find("  no_pool = ").expect("no_pool row");
    let pos_npe = fa_slice
        .find("  no_perceived_edge = ")
        .expect("no_perceived_edge row");
    assert!(
        pos_no_pool < pos_npe,
        "SG-G2.4.b: §F.A must render NoTradeReason::ALL in stable insertion order"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.4.c — empty batch renders `n/a` ratio + all-zero rows
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2_4_c_empty_batch_render_safety() {
    let (_dir, cas) = fixture_cas();
    let summary = MarketDecisionTraceSummary::compute_from_cas(&cas);
    let out = summary.render_section_f();
    assert!(out.contains("total_traces: 0"));
    assert!(
        out.contains("submitted_vs_traced_ratio: 0/0 = n/a (no traces)"),
        "SG-G2.4.c: empty-batch ratio must be `n/a`: {out}"
    );
    for &reason in NoTradeReason::ALL {
        assert!(
            out.contains(&format!("  {} = 0", reason.label())),
            "SG-G2.4.c: zero row for {} missing on empty batch",
            reason.label()
        );
    }
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.4.d — integer-rational percent (no f64 in the ratio surface)
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2_4_d_ratio_is_integer_percent_no_f64() {
    let (_dir, mut cas) = fixture_cas();
    // 1 submitted / 3 total → 33% (integer floor; not 33.33...)
    put_submitted(&mut cas, "Agent_0", 1);
    put_no_trade(&mut cas, "Agent_1", NoTradeReason::NoPool, 2);
    put_no_trade(&mut cas, "Agent_2", NoTradeReason::RouterRejected, 3);
    let s = MarketDecisionTraceSummary::compute_from_cas(&cas);
    let ratio = s.submitted_vs_traced_ratio_str();
    assert_eq!(ratio, "1/3 = 33%");
    // No decimal point in the rendered ratio (no f64 path).
    assert!(!ratio.contains('.'), "SG-G2.4.d: ratio must be integer-rational");
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.4.e — audit_dashboard binary uses the library helper, not an
// inline duplicate walker. Source-grep locks the wire in place.
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2_4_e_audit_dashboard_uses_library_helper() {
    let src = std::fs::read_to_string(AUDIT_DASHBOARD_SRC).expect("read dashboard src");
    assert!(
        src.contains(
            "turingosv4::runtime::market_decision_trace_summary::\
             MarketDecisionTraceSummary::compute_from_cas"
        ) || src.contains("MarketDecisionTraceSummary::compute_from_cas"),
        "SG-G2.4.e: binary must call MarketDecisionTraceSummary::compute_from_cas \
         (library helper); inline duplicate walker would surface here"
    );
    assert!(
        src.contains(".render_section_f()"),
        "SG-G2.4.e: binary must call render_section_f() from the helper"
    );
    // The pre-G2.2 inline walker had a unique signature `for entry in
    // cas.list_all_cids() {` immediately followed by a MarketDecisionTrace
    // deserialize — assert that block was removed from the binary scope
    // (the library helper still uses it, which is fine).
    let inline_walker_pattern = "let mut total_traces: u64 = 0;";
    assert!(
        !src.contains(inline_walker_pattern),
        "SG-G2.4.e: inline §F walker (pre-G2.2) still present in {AUDIT_DASHBOARD_SRC}; \
         binary should consume the library helper exclusively"
    );
}
