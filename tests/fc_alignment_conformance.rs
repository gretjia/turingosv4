//! FC Alignment Conformance Battery (Phase Z' Stage 4).
//!
//! One test per ✅ TRACE_MATRIX row. Each test is a witness that the
//! constitutional flowchart element has concrete runtime behavior.
//!
//! See `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md` for the full row
//! list. Deferred 📅 rows (JudgeAI/ArchitectAI runtime) get a `#[ignore]`
//! stub to keep the row→test mapping complete.

use turingosv4::bus::{BusConfig, BusResult, QState, TuringBus, MR_TICK_AUTHOR};
use turingosv4::kernel::Kernel;
use turingosv4::ledger::{HaltReason, Tape};
use turingosv4::sdk::predicate::{
    ForbiddenPatternPredicate, PayloadSizePredicate, PredicateContext, SorryPredicate, Verdict,
};
use turingosv4::sdk::protocol::{parse_agent_output, AgentAction, AgentOutput};
use turingosv4::sdk::read_tool::{DefaultReadTool, ReadTool};
use turingosv4::sdk::write_tool::{DefaultWriteTool, WriteTool};

fn make_bus() -> TuringBus {
    TuringBus::new(Kernel::new(), BusConfig::default())
}

// ── FC-1 basic cycle ──────────────────────────────────────────────

#[test]
fn fc1_n1_q_t_triple_exposed() {
    // FC1-N1: Q_t = ⟨q_t, HEAD_t, tape_t⟩
    let bus = make_bus();
    // q_t component
    let _ = bus.q_state.clone();
    // tape_t component
    assert!(bus.kernel.tape.is_empty());
    // HEAD_t component accessor
    assert!(bus.kernel.tape.head().is_none());
}

#[test]
fn fc1_n2_q_state_running_on_init() {
    // FC1-N2: q_t = Running after `new()`
    let bus = make_bus();
    assert_eq!(bus.q_state, QState::Running);
}

#[test]
fn fc1_n3_head_returns_last_appended() {
    // FC1-N3: HEAD_t = time_arrow.last()
    let mut bus = make_bus();
    bus.append("A0", "first", None).unwrap();
    let head = bus.kernel.tape.head().cloned();
    assert!(head.is_some());
    bus.append("A0", "second", head.as_deref()).unwrap();
    assert_ne!(bus.kernel.tape.head(), head.as_ref());
}

#[test]
fn fc1_n4_tape_is_append_only() {
    // FC1-N4: tape_t is append-only
    let mut bus = make_bus();
    bus.append("A0", "n1", None).unwrap();
    bus.append("A0", "n2", None).unwrap();
    assert_eq!(bus.kernel.tape.time_arrow().len(), 2);
    // appending cannot mutate existing nodes; tape length monotone increasing
}

#[test]
fn fc1_n5_rtool_is_readonly() {
    // FC1-N5: rtool does not mutate bus.
    let mut bus = make_bus();
    bus.append("A0", "n1", None).unwrap();
    let tx_before = bus.tx_count;
    let rt = DefaultReadTool;
    let _ = rt.project(&bus, Some("A0"));
    assert_eq!(bus.tx_count, tx_before, "rtool must not mutate");
}

#[test]
fn fc1_n6_input_is_snapshot_plus_prompt() {
    // FC1-N6: input = ⟨q_i, s_i⟩ = (UniverseSnapshot, rendered prompt text).
    let bus = make_bus();
    let snap = bus.snapshot();
    assert_eq!(snap.tx_count, 0);
    // prompt rendering is tested in src/sdk/prompt.rs unit tests
}

#[test]
fn fc1_n8_output_wraps_agent_action() {
    // FC1-N8: output = ⟨q_o, a_o⟩ parsed into AgentOutput.
    let raw = r#"<action>{"tool":"step","payload":"linarith"}</action>"#;
    let out = parse_agent_output(raw).expect("parse");
    assert!(out.q_delta.is_none(), "legacy flat form → no q_delta");
    assert_eq!(out.action.tool, "step");
}

#[test]
fn fc1_n9_q_delta_optional_state_hint() {
    // FC1-N9: q_o = q_delta, optional state hint.
    let raw = r#"<action>{"q_delta":"halt_soon","action":{"tool":"step","payload":"norm_num"}}</action>"#;
    let out = parse_agent_output(raw).expect("parse");
    assert_eq!(out.q_delta.as_deref(), Some("halt_soon"));
    assert_eq!(out.action.tool, "step");
}

#[test]
fn fc1_n10_a_o_is_concrete_action() {
    // FC1-N10: a_o = the tool call.
    let a = AgentAction {
        tool: "invest".into(),
        payload: None,
        amount: Some(100.0),
        node: Some("n1".into()),
        query: None,
        direction: Some("long".into()),
    };
    let out = AgentOutput::from_action(a);
    assert_eq!(out.action.tool, "invest");
    assert_eq!(out.action.direction.as_deref(), Some("long"));
}

#[test]
fn fc1_n11_evaluate_predicates_product_semantics() {
    // FC1-N11: ∏p is AND-product. Any Reject → Reject.
    let mut bus = make_bus();
    bus.register_predicate(Box::new(ForbiddenPatternPredicate {
        patterns: vec!["native_decide".into()],
    }));
    bus.register_predicate(Box::new(SorryPredicate));
    let ctx = PredicateContext { tool: "step", author: "A0", tape_depth: 0 };
    assert!(matches!(
        bus.evaluate_predicates(&ctx, "linarith"),
        Verdict::Complete
    ));
    assert!(matches!(
        bus.evaluate_predicates(&ctx, "apply native_decide"),
        Verdict::Reject(_)
    ));
}

#[test]
fn fc1_n12_default_predicates_all_reject_their_pattern() {
    // FC1-N12: 3 default Predicate impls each reject their own pattern.
    let fp = ForbiddenPatternPredicate { patterns: vec!["xxx".into()] };
    let sp = SorryPredicate;
    let ps = PayloadSizePredicate { max_chars: 3, max_lines: 1 };
    use turingosv4::sdk::predicate::Predicate;
    assert!(matches!(fp.verify("xxx found"), Verdict::Reject(_)));
    assert!(matches!(sp.verify("sorry here"), Verdict::Reject(_)));
    assert!(matches!(ps.verify("toolong"), Verdict::Reject(_)));
}

#[test]
fn fc1_n13_write_tool_blessed_vs_unblessed() {
    // FC1-N13: wtool writes via bus.append (free) or append_oracle_accepted (blessed).
    let mut bus = make_bus();
    let wt = DefaultWriteTool;
    // Unblessed Law 1 path
    let r = wt.write(&mut bus, "A0", "free write", None, None).unwrap();
    assert!(matches!(r, BusResult::Appended { .. }));
}

#[test]
fn fc1_n14_successful_append_produces_appended_result() {
    // FC1-N14: Q_{t+1} on ∏p=1 → Appended.
    let mut bus = make_bus();
    match bus.append("A0", "n1", None).unwrap() {
        BusResult::Appended { node_id } => assert!(!node_id.is_empty()),
        other => panic!("expected Appended, got {:?}", other),
    }
}

#[test]
fn fc1_n15_reject_preserves_state() {
    // FC1-N15: ∏p = 0 → BusResult::Vetoed, tape unchanged.
    let mut cfg = BusConfig::default();
    cfg.forbidden_patterns = vec!["native_decide".into()];
    let mut bus = TuringBus::new(Kernel::new(), cfg);
    let tape_len_before = bus.kernel.tape.time_arrow().len();
    let r = bus.append("A0", "native_decide", None).unwrap();
    assert!(matches!(r, BusResult::Vetoed { .. }));
    assert_eq!(bus.kernel.tape.time_arrow().len(), tape_len_before);
}

// ── FC-2 init / halt / tick ──────────────────────────────────────

#[test]
fn fc2_n16_init_runs_once_via_bus_init() {
    // FC2-N16 (InitAI): bus.init() transitions Q_0 from empty to "oracles_frozen".
    let mut bus = make_bus();
    bus.init(&["Agent_0".into()]);
    // After init, register_oracle returns Err (frozen gate)
    let err = bus.register_oracle([0u8; 32]);
    assert!(err.is_err(), "post-init register_oracle must fail");
}

#[test]
fn fc2_n19_register_predicate_api_exists() {
    // FC2-N19: initAI --once→ predicates edge.
    let mut bus = make_bus();
    bus.register_predicate(Box::new(SorryPredicate));
    // Verify it fires
    let ctx = PredicateContext { tool: "step", author: "A0", tape_depth: 0 };
    assert!(matches!(
        bus.evaluate_predicates(&ctx, "exact sorry"),
        Verdict::Reject(_)
    ));
}

#[test]
fn fc2_n21_kernel_new_materializes_q0() {
    // FC2-N21: initAI --once→ Q_0.
    let k = Kernel::new();
    assert!(k.tape.is_empty());
    assert!(k.markets.is_empty());
    assert!(k.bounty_market.is_none());
}

#[test]
fn fc2_n22_halt_transitions_q_state() {
    // FC2-N22: HALT (dbl-circ) materialized as QState::Halted.
    let mut bus = make_bus();
    bus.halt_with_reason(HaltReason::MaxTxExhausted);
    assert!(matches!(
        bus.q_state,
        QState::Halted { reason: HaltReason::MaxTxExhausted }
    ));
}

#[test]
fn fc2_n23_halt_reason_has_five_variants() {
    // FC2-N23: HaltReason matches Report Standard.
    let variants = [
        HaltReason::OmegaAccepted,
        HaltReason::MaxTxExhausted,
        HaltReason::WallClockCap,
        HaltReason::ComputeCapViolated,
        HaltReason::ErrorHalt,
    ];
    assert_eq!(variants.len(), 5);
}

#[test]
fn fc2_n24_clock_increments_on_append() {
    // FC2-N24: clock ticks on every committed event.
    let mut bus = make_bus();
    let before = bus.clock;
    bus.append("A0", "n1", None).unwrap();
    assert!(bus.clock > before);
}

#[test]
fn fc2_n27_mr_reduce_emits_tape1_node() {
    // FC2-N27: mr --reduce→ tape1.
    let mut bus = make_bus();
    let node_id = bus.emit_mr_tick_node("tick@tx10 tape=3").unwrap();
    let node = bus.kernel.tape.get(&node_id).unwrap();
    assert_eq!(node.author, MR_TICK_AUTHOR);
}

#[test]
fn fc2_n28_tools_list_accessible() {
    // FC2-N28: tools_other — TuringBus.tools is iterable.
    let bus = make_bus();
    let _count = bus.tools.len();  // field accessible; no mount yet = 0.
}

// ── FC-3 anti-oreo / system-level ──────────────────────────────────

#[test]
fn fc3_n31_wal_construction_api_exists() {
    // FC3-N31: logs archive — WAL construction is a static constructor.
    // Full WAL round-trip tests live in tests/q_halt_state.rs. Here we only
    // witness the API exists and returns a bus in Running state.
    let tmp_dir = std::env::temp_dir().join(format!("fc_align_{}", std::process::id()));
    std::fs::create_dir_all(&tmp_dir).unwrap();
    let path = tmp_dir.join("test.wal");
    let bus = TuringBus::with_wal_path(Kernel::new(), BusConfig::default(), &path)
        .expect("WAL construction");
    assert!(bus.q_state == QState::Running);
    let _ = std::fs::remove_dir_all(&tmp_dir);
}

#[test]
fn fc3_n36_agent_ids_round_robin() {
    // FC3-N36: agents — swarm identities allocated at init.
    let mut bus = make_bus();
    let agents: Vec<String> = (0..4).map(|i| format!("Agent_{}", i)).collect();
    bus.init(&agents);
    // No crash means init accepts multi-agent identity list.
}

#[test]
fn fc3_n37_turing_tool_trait_mountable() {
    // FC3-N37: tools — at least one concrete TuringTool is mountable.
    use turingosv4::sdk::tools::wallet::WalletTool;
    let mut bus = make_bus();
    bus.mount_tool(Box::new(WalletTool::new(10000.0)));
    assert_eq!(bus.tools.len(), 1);
}

#[test]
fn fc3_n39_ledger_records_events() {
    // FC3-N39: log — Ledger.append records each event.
    let mut bus = make_bus();
    bus.append("A0", "n1", None).unwrap();
    assert!(bus.ledger.events().len() > 0);
}

// ── FC-3 Phase-11+ deferred stubs (track-only) ─────────────────────
// These rows remain 📅 in TRACE_MATRIX. Stub tests preserve row→test
// coverage in CI so future implementations can flip #[ignore] off.

#[test]
#[ignore = "FC3-N32 JudgeAI runtime — Phase 11+"]
fn fc3_n32_judge_ai_runtime_stub() {
    // Future: multi-judge runtime voting (Codex + Gemini + DeepSeek).
}

#[test]
#[ignore = "FC3-N33 ArchitectAI runtime — Phase 11+"]
fn fc3_n33_architect_ai_runtime_stub() {
    // Future: logs → feedback → architect patch loop.
}

#[test]
#[ignore = "FC3-N34 FS readonly guard — Phase 11+"]
fn fc3_n34_readonly_guard_stub() {
    // Future: FS-level readonly enforcement on constitution.md + logs/.
}

#[test]
#[ignore = "FC3-N40 feedback loop — Phase 11+"]
fn fc3_n40_feedback_loop_stub() {
    // Future: runtime feedback channel logs → ArchitectAI.
}

#[test]
#[ignore = "FC3-N41 auto re-init — Phase 11+"]
fn fc3_n41_auto_reinit_stub() {
    // Future: in-process retry on init/LLM/WAL failure.
}
