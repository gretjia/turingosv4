// Phase 8.E (C-061) regression test
// Constitutional Art. IV mermaid: `Q_t = ⟨q_t, HEAD_t, tape_t⟩`.
// `q_t ∈ {Running, Halted}` must be an explicit first-class field; halting
// must emit a Halt event with reason so halt_reason_distribution
// (CLAUDE.md Report Standard) is derivable from the ledger.
//
// Prior to this fix, halt_and_settle only emitted RunEnd (no reason); the
// distinction between "ran to tx_cap" / "wall-clock cap" / "OMEGA success"
// was not recorded.

use turingosv4::bus::{BusConfig, BusResult, QState, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::ledger::{EventType, HaltReason};
use turingosv4::sdk::tools::wallet::WalletTool;

fn make_bus() -> TuringBus {
    let kernel = Kernel::new();
    let config = BusConfig {
        max_payload_chars: 500,
        max_payload_lines: 20,
        system_lp_amount: 200.0,
        forbidden_patterns: vec![],
        min_class_count_to_broadcast: 3,
    };
    let mut bus = TuringBus::new(kernel, config);
    bus.mount_tool(Box::new(WalletTool::new(10_000.0)));
    bus.init(&["Alice".into()]);
    bus
}

#[test]
fn q_state_starts_running() {
    let bus = make_bus();
    assert_eq!(bus.q_state, QState::Running,
        "fresh bus must be Running (no Halt event yet)");
}

#[test]
fn halt_with_reason_transitions_and_emits_event() {
    let mut bus = make_bus();
    bus.halt_with_reason(HaltReason::MaxTxExhausted);
    assert_eq!(
        bus.q_state,
        QState::Halted { reason: HaltReason::MaxTxExhausted },
        "q_state must flip to Halted with given reason"
    );

    // Ledger must contain exactly one Halt event with matching reason.
    let halt_events: Vec<_> = bus.ledger.events().iter()
        .filter(|e| matches!(e.event_type, EventType::Halt { .. }))
        .collect();
    assert_eq!(halt_events.len(), 1, "expected 1 Halt event, got {}", halt_events.len());
    match &halt_events[0].event_type {
        EventType::Halt { reason } => {
            assert_eq!(*reason, HaltReason::MaxTxExhausted);
        }
        _ => unreachable!(),
    }
}

#[test]
fn halt_is_idempotent() {
    let mut bus = make_bus();
    bus.halt_with_reason(HaltReason::ErrorHalt);
    bus.halt_with_reason(HaltReason::MaxTxExhausted);  // second call

    // q_state reflects the latest reason (overwrite semantics).
    assert_eq!(
        bus.q_state,
        QState::Halted { reason: HaltReason::MaxTxExhausted },
        "second halt_with_reason should update reason"
    );

    // But ledger contains only one Halt event (avoid WAL pollution).
    let halt_events: Vec<_> = bus.ledger.events().iter()
        .filter(|e| matches!(e.event_type, EventType::Halt { .. }))
        .collect();
    assert_eq!(halt_events.len(), 1,
        "idempotence: only first Halt call emits an event");
    // The emitted event records the FIRST reason (ErrorHalt) — a truthful
    // record of "when and why q first flipped to Halted".
    match &halt_events[0].event_type {
        EventType::Halt { reason } => {
            assert_eq!(*reason, HaltReason::ErrorHalt,
                "first Halt event preserves initial reason");
        }
        _ => unreachable!(),
    }
}

#[test]
fn halt_and_settle_emits_omega_accepted() {
    let mut bus = make_bus();
    let node_id = match bus.append("Alice", "proof step", None).unwrap() {
        BusResult::Appended { node_id } => node_id,
        other => panic!("expected Appended, got {:?}", other),
    };
    bus.halt_and_settle(&[node_id]).expect("halt_and_settle ok");

    assert_eq!(
        bus.q_state,
        QState::Halted { reason: HaltReason::OmegaAccepted },
        "halt_and_settle must record OmegaAccepted reason"
    );
    let halt_events: Vec<_> = bus.ledger.events().iter()
        .filter(|e| matches!(e.event_type, EventType::Halt { .. }))
        .collect();
    assert_eq!(halt_events.len(), 1);
}

#[test]
fn halt_event_type_displays_reason() {
    // Display string is what CHECKPOINT reporting tools render.
    let e = EventType::Halt { reason: HaltReason::WallClockCap };
    let s = format!("{}", e);
    assert!(s.contains("Halt") && s.contains("WallClockCap"),
        "Display should surface reason; got {}", s);
}

#[test]
fn halt_reason_variants_are_distinguishable() {
    // Ensure enum variants don't collide — halt_reason_distribution relies
    // on Eq.
    assert_ne!(HaltReason::OmegaAccepted, HaltReason::MaxTxExhausted);
    assert_ne!(HaltReason::WallClockCap, HaltReason::ComputeCapViolated);
    assert_ne!(HaltReason::OmegaAccepted, HaltReason::ErrorHalt);
}
