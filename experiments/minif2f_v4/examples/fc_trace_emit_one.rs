// Phase A atom A6 helper — emit one fc_event for the smoke test.
// Used by tests/fc_trace_smoke.rs to exercise the FC_TRACE=1 +
// FC_TRACE_FILE=<path> code path in a fresh OnceLock state. Production
// callers go through the run_swarm wiring, not this binary.

fn main() {
    minif2f_v4::fc_trace::emit_event(
        minif2f_v4::fc_trace::FcId::Fc2N22,
        "smoke_run_001",
        Some(42),
        Some("Agent_2"),
        &[
            ("reason", minif2f_v4::fc_trace::json_str("OmegaAccepted")),
            ("gp_nodes", "7".to_string()),
        ],
    );
}
