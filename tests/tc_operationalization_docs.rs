use std::fs;
use std::path::Path;

fn strong_claim_words() -> [String; 2] {
    [["PRO", "VEN"].concat(), ["DEFIN", "ITIVE"].concat()]
}

#[test]
fn tc_000_path_b_decision_is_explicit_and_scoped() {
    let body = fs::read_to_string("handover/directives/TC_000_PATH_B_DECISION.md")
        .expect("TC_000_PATH_B_DECISION.md exists");
    assert!(body.contains("Decision: Path B"));
    assert!(body.contains("Path A: rejected"));
    assert!(body.contains("Path C: rejected"));
    assert!(body.contains("refs/chaintape/l4"));
    assert!(body.contains("refs/chaintape/l4e"));
    assert!(body.contains("refs/chaintape/cas"));
    for word in strong_claim_words() {
        assert!(!body.contains(&word));
    }
}

#[test]
fn tc_001_veto_scope_lock_has_constitution_only_domain() {
    let body = fs::read_to_string("handover/directives/TC_001_VETO_AI_SCOPE_LOCK.md")
        .expect("TC_001_VETO_AI_SCOPE_LOCK.md exists");
    assert!(body.contains("Veto-AI output domain: `{PASS,VETO}`"));
    assert!(body.contains("does not review code style"));
    assert!(body.contains("does not review performance"));
    assert!(body.contains("does not review coverage"));
    for word in strong_claim_words() {
        assert!(!body.contains(&word));
    }
}

#[test]
fn tc_taskpacket_directory_is_complete_and_locks_lean_boundary() {
    let root = Path::new("handover/directives/tc_taskpackets_2026-06-04");
    let index_path = root.join("INDEX.md");
    let index = fs::read_to_string(&index_path).expect("TC taskpacket INDEX.md exists");
    assert!(index.contains("Lean is a"));
    assert!(index.contains("not the TuringOS kernel"));
    assert!(index.contains("OBL-014 remains open until"));

    let required_packets = [
        "TC-ORCH-000-index.md",
        "TC-ORCH-001-gate-policy.md",
        "TC-ORCH-002-worker-prompt.md",
        "TC-ORCH-003-reviewer-prompts.md",
        "TC-Q000-dirty-quarantine.md",
        "TC-Q001-clean-worktree.md",
        "TC-000-path-b-decision.md",
        "TC-001-veto-scope.md",
        "TC-002-boot-trust-root.md",
        "TC-003a-ref-contract.md",
        "TC-003b-fail-closed-refs.md",
        "TC-003c-reopen-sequence.md",
        "TC-004a-rtool-head-witness.md",
        "TC-004b-wtool-accepted-write.md",
        "TC-005a-rejection-split.md",
        "TC-005b-l4e-reconstruction.md",
        "TC-101-gateway-fact.md",
        "TC-102-wal-fact.md",
        "TC-103-map-reduce-tick-fact.md",
        "TC-104-llm-call-fact.md",
        "TC-105-wallet-derived-fact.md",
        "TC-106-search-fact.md",
        "TC-107-board-derived-fact.md",
        "TC-108-lean-error-fact.md",
        "TC-109-halt-fact.md",
        "TC-110-boot-provenance-fact.md",
        "TC-007a-durable-outbox.md",
        "TC-007b-crash-terminal-mapping.md",
        "TC-007c-production-llm-wrapper.md",
        "TC-007d-clean-halt-gate.md",
        "TC-009a-minsky-core.md",
        "TC-009b-minsky-replay.md",
        "TC-010a-brainfuck-core.md",
        "TC-010b-brainfuck-replay.md",
        "TC-011A-lean-schema-lock.md",
        "TC-011B-lean-step-fixtures.md",
        "TC-011C-lean-final-recert.md",
        "TC-012A-g0-manifest-freeze.md",
        "TC-012B-g0-ast-rank.md",
        "TC-012C-g0-enumerator.md",
        "TC-013A-strict-dovetail.md",
        "TC-013B-market-invariance.md",
        "TC-014A-duplicate-pointer.md",
        "TC-014B-poisoned-odd-queue.md",
        "TC-016-market-legalization.md",
        "TC-017-autonomous-legalization.md",
        "TC-018A-agent-view-renderer.md",
        "TC-018B-prompt-guard.md",
        "TC-015A-crash-matrix-driver.md",
        "TC-019A-difficulty-ladder.md",
        "TC-020A-prereg-parity.md",
        "TC-021A-audit-packet-export.md",
        "TC-021B-clean-checkout-replay.md",
    ];

    for packet in required_packets {
        assert!(
            index.contains(packet),
            "INDEX.md must reference packet {packet}"
        );
        let packet_path = root.join("packets").join(packet);
        assert!(packet_path.exists(), "packet must exist: {packet}");
        let body = fs::read_to_string(&packet_path).expect("packet is readable");
        assert!(body.contains("Active obligations: OBL-014(open) -> this atom"));
        assert!(body.contains("Ship gate:"));
        assert!(!body.contains("TBD"));
        assert!(!body.contains("TODO"));
    }

    let worker_prompt = fs::read_to_string(root.join("templates/LOW_REASONING_WORKER_PROMPT.md"))
        .expect("worker prompt exists");
    assert!(worker_prompt.contains("Lean is not the TuringOS kernel"));
    assert!(worker_prompt.contains("Do not edit files outside"));
}

#[test]
fn tc_plan_structural_gates_are_changed_file_scoped() {
    let body = fs::read_to_string(
        "handover/directives/TC_OPERATIONALIZATION_FULL_EXECUTION_PLAN_2026-06-04.md",
    )
    .expect("TC full execution plan exists");

    assert!(body.contains("changed_files"));
    assert!(body.contains("changed-file scoped"));
    let recursive_grep = ["grep", "-RInE"].join(" ");
    let historical_scope = ["handover", "src", "tests"].join(" ");
    assert!(!body.contains(&recursive_grep));
    assert!(!body.contains(&historical_scope));
}
