//! Market Autonomy Lab ARH-v2 envelope gates.
//!
//! These gates make the autonomous research envelope executable enough that
//! Codex cannot silently fall back to ship-mode hard stops or overclaim E2.

use std::path::Path;

fn read(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

#[test]
fn research_envelope_v2_declares_research_only_not_ship() {
    let envelope = read("handover/directives/market_autonomy_lab/RESEARCH_ENVELOPE_V2.md");
    for required in [
        "Constitutional Research Mode only",
        "not a ship authorization",
        "not a main-branch merge authorization",
        "not permission to claim E2/E3/E4 achieved",
        "Never write `E2 achieved`",
        "E2 candidate pending audit",
    ] {
        assert!(
            envelope.contains(required),
            "missing envelope marker: {required}"
        );
    }
}

#[test]
fn research_envelope_v2_lists_allowed_and_forbidden_surfaces() {
    let envelope = read("handover/directives/market_autonomy_lab/RESEARCH_ENVELOPE_V2.md");
    for allowed in [
        "experiments/minif2f_v4/src/bin/evaluator.rs",
        "src/runtime/librarian_broadcast.rs",
        "src/runtime/ev_decision_trace.rs",
        "src/runtime/policy_trader_trace.rs",
        "src/bin/audit_dashboard.rs",
        "scripts/run_real12_task_market_probe.sh",
        "scripts/run_real13_market_pressure_probe.sh",
        "h_vppu_history.json",
        "genesis_payload.toml",
    ] {
        assert!(
            envelope.contains(allowed),
            "missing allowed surface: {allowed}"
        );
    }
    for forbidden in [
        "constitution.md",
        "src/state/typed_tx.rs",
        "src/state/sequencer.rs",
        "src/bottom_white/cas/schema.rs",
        "src/kernel.rs",
        "src/bus.rs",
        "src/sdk/tools/wallet.rs",
        "canonical signing payload code",
        "CAS ObjectType schema",
    ] {
        assert!(
            envelope.contains(forbidden),
            "missing forbidden surface: {forbidden}"
        );
    }
}

#[test]
fn research_envelope_v2_requires_stop_proof_before_hard_stop() {
    let policy = read("handover/directives/market_autonomy_lab/ARH_V2_STOP_POLICY.md");
    let template = read("handover/directives/market_autonomy_lab/STOP_PROOF_TEMPLATE.md");
    for required in [
        "Level 0",
        "Level 1",
        "Level 2",
        "Level 3",
        "write `STOP_PROOF.md` before stopping",
        "Clean-negative is not completion",
        "allowed Trust Root rehash may proceed automatically",
    ] {
        assert!(
            policy.contains(required),
            "missing stop policy marker: {required}"
        );
    }
    for field in [
        "exact_stop_clause",
        "in_envelope_alternatives_considered",
        "why_each_alternative_failed",
        "next_required_user_or_architect_action",
    ] {
        assert!(
            template.contains(field),
            "missing STOP_PROOF field: {field}"
        );
    }
}

#[test]
fn research_envelope_v2_forbids_clean_negative_as_completion() {
    let envelope = read("handover/directives/market_autonomy_lab/RESEARCH_ENVELOPE_V2.md");
    let policy = read("handover/directives/market_autonomy_lab/ARH_V2_STOP_POLICY.md");
    let goal = read("handover/directives/market_autonomy_lab/CODEX_GOAL_PROMPT.md");
    for text in [envelope, policy, goal] {
        assert!(
            text.contains("Clean-negative") || text.contains("clean-negative"),
            "document must discuss clean-negative continuation"
        );
        assert!(
            text.contains("not completion")
                || text.contains("not treat clean-negative as completion")
                || text.contains("No E2 candidate is not completion"),
            "document must forbid clean-negative/no-E2 as completion"
        );
    }
}

#[test]
fn research_envelope_v2_preserves_architect_source_verbatim() {
    let path = "handover/directives/2026-05-16_MARKET_AUTONOMY_LAB_ARCHITECT_ORIGINAL.md";
    assert!(Path::new(path).exists(), "architect original must exist");
    let original = read(path);
    for required in [
        "## 总裁决",
        "不可以强制交易",
        "PositiveEVIgnored",
        "PolicyTrader",
    ] {
        assert!(
            original.contains(required),
            "architect original missing expected phrase: {required}"
        );
    }
}

#[test]
fn market_autonomy_research_preflight_guard_exists_and_names_envelope() {
    let script = read("scripts/run_market_autonomy_research_preflight.sh");
    for required in [
        "MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2",
        "TURINGOS_RESEARCH_ENVELOPE",
        "forbidden_surfaces",
        "stop_level=",
        "STOP_PROOF.md",
    ] {
        assert!(
            script.contains(required),
            "missing preflight guard marker: {required}"
        );
    }
}
