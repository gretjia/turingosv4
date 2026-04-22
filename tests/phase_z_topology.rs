//! Phase Z integration tests — ∏p formalization, ReadTool, WriteTool.
//!
//! Constitutional basis: Art. IV mermaid.
//! - ∏p product evaluation through `TuringBus::evaluate_predicates`
//! - rtool identity projection via `DefaultReadTool::project`
//! - wtool blessed/unblessed write via `DefaultWriteTool::write`

use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::sdk::predicate::{
    ForbiddenPatternPredicate, PayloadSizePredicate, Predicate, PredicateContext, PredicateKind,
    SorryPredicate, Verdict,
};
use turingosv4::sdk::read_tool::{DefaultReadTool, ReadTool};
use turingosv4::sdk::write_tool::{DefaultWriteTool, WriteTool};

fn make_bus() -> TuringBus {
    TuringBus::new(Kernel::new(), BusConfig::default())
}

fn ctx<'a>(tool: &'a str) -> PredicateContext<'a> {
    PredicateContext { tool, author: "A0", tape_depth: 0 }
}

#[test]
fn empty_predicate_chain_accepts_anything() {
    let bus = make_bus();
    let v = bus.evaluate_predicates(&ctx("step"), "anything goes");
    assert!(matches!(v, Verdict::Complete));
}

#[test]
fn single_forbidden_pattern_rejects() {
    let mut bus = make_bus();
    bus.register_predicate(Box::new(ForbiddenPatternPredicate {
        patterns: vec!["native_decide".into()],
    }));
    match bus.evaluate_predicates(&ctx("step"), "apply native_decide") {
        Verdict::Reject(r) => assert!(r.contains("native_decide")),
        v => panic!("expected reject, got {:?}", v),
    }
}

#[test]
fn product_short_circuits_on_first_reject() {
    // Register two predicates; the first rejects → second should not be
    // asked. We verify this via ordering: first predicate's reason wins.
    let mut bus = make_bus();
    bus.register_predicate(Box::new(ForbiddenPatternPredicate {
        patterns: vec!["first_bad".into()],
    }));
    bus.register_predicate(Box::new(ForbiddenPatternPredicate {
        patterns: vec!["second_bad".into()],
    }));
    match bus.evaluate_predicates(&ctx("step"), "first_bad and second_bad") {
        Verdict::Reject(r) => assert!(r.contains("first_bad"), "got: {}", r),
        v => panic!("expected first reject, got {:?}", v),
    }
}

#[test]
fn all_predicates_pass_yields_complete() {
    let mut bus = make_bus();
    bus.register_predicate(Box::new(ForbiddenPatternPredicate {
        patterns: vec!["native_decide".into()],
    }));
    bus.register_predicate(Box::new(SorryPredicate));
    bus.register_predicate(Box::new(PayloadSizePredicate {
        max_chars: 100,
        max_lines: 10,
    }));
    let v = bus.evaluate_predicates(&ctx("step"), "linarith");
    assert!(matches!(v, Verdict::Complete));
}

// Tool-scoped predicate: only fires on "invest".
struct InvestOnlyPredicate;
impl Predicate for InvestOnlyPredicate {
    fn name(&self) -> &str { "invest_only_test" }
    fn kind(&self) -> PredicateKind { PredicateKind::WalletBalance }
    fn applies_to(&self, ctx: &PredicateContext) -> bool { ctx.tool == "invest" }
    fn verify(&self, _payload: &str) -> Verdict {
        Verdict::Reject("invest denied".into())
    }
}

#[test]
fn predicate_respects_applies_to_filter() {
    let mut bus = make_bus();
    bus.register_predicate(Box::new(InvestOnlyPredicate));
    // tool="step" → predicate skipped → Complete.
    assert!(matches!(
        bus.evaluate_predicates(&ctx("step"), "payload"),
        Verdict::Complete
    ));
    // tool="invest" → predicate fires → Reject.
    assert!(matches!(
        bus.evaluate_predicates(&ctx("invest"), "payload"),
        Verdict::Reject(_)
    ));
}

#[test]
fn default_read_tool_returns_full_snapshot_for_every_agent() {
    let bus = make_bus();
    let rt = DefaultReadTool;
    let s_a = rt.project(&bus, Some("Agent_0"));
    let s_b = rt.project(&bus, Some("Agent_1"));
    let s_n = rt.project(&bus, None);
    // All three projections identical at genesis (no filters yet).
    assert_eq!(s_a.tx_count, s_b.tx_count);
    assert_eq!(s_a.tx_count, s_n.tx_count);
    assert_eq!(s_a.generation, s_n.generation);
}

#[test]
fn default_write_tool_unblessed_path_appends() {
    let mut bus = make_bus();
    let wt = DefaultWriteTool;
    // Unblessed path: Law 1 free topology.
    let res = wt.write(&mut bus, "A0", "hello tape", None, None);
    assert!(res.is_ok(), "unblessed write failed: {:?}", res.err());
    match res.unwrap() {
        turingosv4::bus::BusResult::Appended { node_id } => {
            assert!(!node_id.is_empty());
        }
        other => panic!("expected Appended, got {:?}", other),
    }
}

#[test]
fn evaluate_predicates_with_context_different_tools() {
    // Same predicates, different contexts: ensures context is actually
    // plumbed through (not ignored).
    let mut bus = make_bus();
    bus.register_predicate(Box::new(InvestOnlyPredicate));
    let step_v = bus.evaluate_predicates(&ctx("step"), "x");
    let invest_v = bus.evaluate_predicates(&ctx("invest"), "x");
    assert!(matches!(step_v, Verdict::Complete));
    assert!(matches!(invest_v, Verdict::Reject(_)));
}
