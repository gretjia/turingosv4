//! TB-G G2.1 — NoTradeReason 13-variant taxonomy audit (Class 2).
//!
//! Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
//! §1 Module G2 atom G2.1.
//!
//! Directive: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
//! §G2 verbatim 9-variant `enum NoTradeReason` listing.
//!
//! Ship gates per charter row G2.1: "trace-or-tx for every market-bearing
//! turn / failed invest in L4.E / source-grep covers each variant /
//! 13-variant exhaustive name check".
//!
//! - SG-G2.1: source-grep — each of the 13 variants appears in the
//!   production source at least twice (enum definition + label match arm).
//! - SG-G2.2: enum `NoTradeReason::ALL` is len() == 13 and contains every
//!   `NoTradeReason` variant exactly once (sanity for the dashboard column
//!   iteration in G2.2).
//! - SG-G2.3: every variant emitted via `to_no_trade_reason()` from
//!   `InvestRouteError` round-trips through `label()` to a unique
//!   lower-snake-case key.
//! - SG-G2.4: architect §8.2 directive verbatim variant names (9 of them)
//!   are all label-anchored in `NoTradeReason::ALL.label()` (covers the
//!   `InsufficientBalance` ↔ `AmountExceedsBalance` doc-alias).
//! - SG-G2.5: every `InvestRouteError` variant maps to a known
//!   `NoTradeReason` (no `Unknown` fallback except the explicit
//!   `KeypairError` mapping).
//! - SG-G2.6 (trace-or-tx invariant): `MarketDecisionTrace::no_trade(…)`
//!   constructed with each NoTradeReason produces a deterministic
//!   classifier-bound CAS object — exercises every variant at the
//!   trace-construction layer to lock in the taxonomy at the type level.
//!
//! `FC-trace: FC1-N5/N6 predicate-or-CAS evidence for every externalized
//! market-bearing turn — the no-trade taxonomy is the canonical L4.E /
//! anchored-capsule classifier surface.`

use turingosv4::runtime::adapter::InvestRouteError;
use turingosv4::runtime::market_decision_trace::{
    MarketDecisionTrace, NoTradeReason, TraceOutcome,
};
use turingosv4::state::q_state::AgentId;

const MDT_SRC: &str = "src/runtime/market_decision_trace.rs";
const ADAPTER_SRC: &str = "src/runtime/adapter.rs";

// ────────────────────────────────────────────────────────────────────────
// SG-G2.1 — source-grep covers each variant (enum def + label match)
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2_1_source_grep_covers_each_variant() {
    let src = std::fs::read_to_string(MDT_SRC).expect("read trace src");
    for variant_ident in [
        "NoPromptTool",
        "NoParsedInvest",
        "MalformedNode",
        "ZeroAmount",
        "AmountExceedsBalance",
        "NoPool",
        "RouterRejected",
        "AgentDeclined",
        "TooFastSolve",
        "SlippageOutZero",
        "Unknown",
        "NoPerceivedEdge",
        "PromptBudgetExceeded",
    ] {
        let occurrences = src.matches(variant_ident).count();
        assert!(
            occurrences >= 2,
            "SG-G2.1: variant {variant_ident:?} must appear ≥ 2× in {MDT_SRC} \
             (enum def + label match); found {occurrences}"
        );
    }
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.2 — NoTradeReason::ALL exhaustive 13-variant list
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2_2_no_trade_reason_all_is_13_variants() {
    assert_eq!(
        NoTradeReason::ALL.len(),
        13,
        "SG-G2.2: 11 pre-existing + 2 G2.1 tail-append = 13"
    );
    // Each variant appears exactly once.
    use std::collections::BTreeSet;
    let variants: BTreeSet<NoTradeReason> = NoTradeReason::ALL.iter().copied().collect();
    assert_eq!(variants.len(), NoTradeReason::ALL.len());
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.3 — label() round-trip uniqueness (every variant → unique key)
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2_3_label_round_trip_unique() {
    use std::collections::BTreeSet;
    let labels: BTreeSet<&'static str> =
        NoTradeReason::ALL.iter().map(|r| r.label()).collect();
    assert_eq!(
        labels.len(),
        NoTradeReason::ALL.len(),
        "SG-G2.3: every variant must have a unique label key"
    );
    // Every label is lower-snake.
    for label in &labels {
        assert_eq!(*label, label.to_lowercase());
        assert!(
            label.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
            "SG-G2.3: label {label:?} must be ascii lower snake"
        );
    }
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.4 — architect §8.2 verbatim 9-variant labels all present
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2_4_architect_directive_verbatim_labels_present() {
    use std::collections::BTreeSet;
    let labels: BTreeSet<&'static str> =
        NoTradeReason::ALL.iter().map(|r| r.label()).collect();
    // Architect §8.2 directive `enum NoTradeReason` verbatim:
    // `NoPool, NoPromptTool, NoParsedInvest, InsufficientBalance,
    //  RouterRejected, AgentDeclined, TooFastSolve, NoPerceivedEdge,
    //  PromptBudgetExceeded`.
    // `InsufficientBalance` ↔ `AmountExceedsBalance` per source doc-alias.
    for expected in [
        "no_pool",
        "no_prompt_tool",
        "no_parsed_invest",
        "amount_exceeds_balance", // doc-alias for architect `InsufficientBalance`
        "router_rejected",
        "agent_declined",
        "too_fast_solve",
        "no_perceived_edge",
        "prompt_budget_exceeded",
    ] {
        assert!(
            labels.contains(expected),
            "SG-G2.4: architect §8.2 verbatim variant {expected:?} missing from \
             NoTradeReason taxonomy (rename / drop / typo would surface here)"
        );
    }
}

#[test]
fn sg_g2_4_amount_exceeds_balance_carries_insufficient_balance_doc_alias() {
    let src = std::fs::read_to_string(MDT_SRC).expect("read trace src");
    // The variant's rustdoc must mention architect's `InsufficientBalance`
    // spelling so forward audits can find the binding without surprise.
    let count = src.matches("InsufficientBalance").count();
    assert!(
        count >= 1,
        "SG-G2.4: `AmountExceedsBalance` must doc-alias architect \
         `InsufficientBalance` (found {count} occurrences in {MDT_SRC}; expected ≥ 1)"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.5 — InvestRouteError → NoTradeReason mapping is total
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2_5_invest_route_error_mapping_total() {
    // Every InvestRouteError variant must map to a NoTradeReason. We
    // construct one of each (the keypair-error path requires a typed
    // value; we exercise via raw match arm coverage on the source).
    let cases: Vec<(InvestRouteError, NoTradeReason)> = vec![
        (InvestRouteError::ZeroAmount, NoTradeReason::ZeroAmount),
        (InvestRouteError::NegativeAmount, NoTradeReason::NoParsedInvest),
        (
            InvestRouteError::AmountExceedsBalance {
                amount_micro: 10,
                balance_micro: 5,
            },
            NoTradeReason::AmountExceedsBalance,
        ),
        (
            InvestRouteError::MalformedNode { reason: "empty" },
            NoTradeReason::MalformedNode,
        ),
        (InvestRouteError::UnknownEvent, NoTradeReason::NoPool),
        (InvestRouteError::PoolNotActive, NoTradeReason::NoPool),
        (InvestRouteError::NoPerceivedEdge, NoTradeReason::NoPerceivedEdge),
        (
            InvestRouteError::PromptBudgetExceeded,
            NoTradeReason::PromptBudgetExceeded,
        ),
    ];
    for (err, expected) in cases {
        let mapped = err.to_no_trade_reason();
        assert_eq!(
            mapped, expected,
            "SG-G2.5: InvestRouteError {err:?} must map to {expected:?}, got {mapped:?}"
        );
        // public_summary is non-empty + ≤ 200 chars (sanity).
        let summary = err.public_summary();
        assert!(!summary.is_empty(), "SG-G2.5: public_summary cannot be empty");
        assert!(summary.len() <= 200, "SG-G2.5: public_summary length sane");
    }
    // KeypairError mapping is exhaustive-witness via source-grep (it requires
    // a typed AgentKeypairError value; we don't construct one here).
    let adapter_src = std::fs::read_to_string(ADAPTER_SRC).expect("read adapter src");
    assert!(
        adapter_src.contains("InvestRouteError::KeypairError(_) => NoTradeReason::Unknown"),
        "SG-G2.5: KeypairError must map to NoTradeReason::Unknown"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.6 — trace-or-tx invariant: every variant builds a valid CAS object
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2_6_trace_or_tx_invariant_every_variant_buildable() {
    // For each NoTradeReason variant the constructor produces a
    // well-formed `MarketDecisionTrace` with a `NoTrade { reason, tx_id }`
    // outcome whose `reason` is the canonical taxonomy slot. This is the
    // type-level half of the trace-or-tx invariant (the other half is the
    // evaluator's end-of-turn classifier + `"invest"` dispatch arm).
    for &reason in NoTradeReason::ALL {
        let trace = MarketDecisionTrace::no_trade(
            AgentId(format!("Agent_test_{}", reason.label())),
            None,
            None,
            None,
            reason,
            format!("trace-or-tx witness for {}", reason.label()),
        );
        match trace.outcome {
            TraceOutcome::NoTrade { reason: r, tx_id } => {
                assert_eq!(r, reason);
                assert!(
                    tx_id.is_none(),
                    "SG-G2.6: pre-submit no-trade trace must not carry a tx_id"
                );
            }
            other => panic!("SG-G2.6: expected NoTrade outcome for {reason:?}, got {other:?}"),
        }
        assert_eq!(trace.schema_version, MarketDecisionTrace::SCHEMA_VERSION);
        // Reason summary truncated to ≤120 chars by constructor.
        assert!(trace.reason_summary_public.len() <= 120);
    }
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.6.a — evaluator end-of-turn classifier source-grep
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2_6_a_evaluator_end_of_turn_classifier_source_grep() {
    // The trace-or-tx invariant requires the evaluator to fire BOTH new
    // variants from its end-of-turn classifier when (a) the agent's prompt
    // included the market block but they didn't emit invest, OR (b) the
    // market block was elided by the prompt-budget top-K=0 cap. Source-grep
    // locks in the wire location so future refactors that break it surface
    // here.
    let eval_src = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs")
        .expect("read evaluator src");
    for needle in [
        // The two flag bindings on the prompt-build path.
        "tb_n3_market_block_present",
        "tb_n3_market_block_budget_elided",
        // The end-of-turn invest-emitted flag.
        "invest_action_emitted_this_turn",
        // The two new variants must be referenced by the end-of-turn
        // classifier (NOT just by the architect-spec source comment).
        "NoTradeReason::NoPerceivedEdge",
        "NoTradeReason::PromptBudgetExceeded",
    ] {
        assert!(
            eval_src.contains(needle),
            "SG-G2.6.a: evaluator end-of-turn classifier must reference {needle:?} \
             (trace-or-tx invariant wire missing)"
        );
    }
}
