// Phase 8.C v2 (Codex V-1 / C-067) regression test
// Bus must reject blessed write if OracleReceipt fails ANY of:
//   (a) payload-hash binding (content tampered)
//   (b) context-hash binding (cross-parent replay)
//   (c) oracle nonce not registered with this bus

use turingosv4::bus::{BusConfig, BusResult, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::sdk::oracle_receipt::{hash_context, hash_payload, OracleReceipt};
use turingosv4::sdk::predicate::{PredicateKind, Verdict};
use turingosv4::sdk::tools::wallet::WalletTool;

const TEST_NONCE: u64 = 0xCAFE_u64;
const OTHER_NONCE: u64 = 0xBEEF_u64;

fn make_bus_and_register() -> TuringBus {
    let kernel = Kernel::new();
    let config = BusConfig {
        max_payload_chars: 500,
        max_payload_lines: 20,
        system_lp_amount: 200.0,
        forbidden_patterns: vec!["native_decide".into()],
        min_class_count_to_broadcast: 3,
    };
    let mut bus = TuringBus::new(kernel, config);
    bus.register_oracle(TEST_NONCE);
    bus.mount_tool(Box::new(WalletTool::new(10_000.0)));
    bus.init(&["Agent_0".into()]);
    bus
}

#[test]
fn blessed_write_accepts_matching_receipt() {
    let mut bus = make_bus_and_register();
    let payload = "by linarith";
    let receipt = OracleReceipt::new_lean4_complete(payload, None, TEST_NONCE);
    let res = bus.append_oracle_accepted("Agent_0", payload, None, &receipt)
        .expect("matching receipt should be accepted");
    assert!(matches!(res, BusResult::Appended { .. }));
}

#[test]
fn blessed_write_rejects_hash_mismatch() {
    let mut bus = make_bus_and_register();
    let receipt = OracleReceipt::new_lean4_complete("by linarith", None, TEST_NONCE);
    let err = bus.append_oracle_accepted(
        "Agent_0", "by native_decide", None, &receipt,
    ).expect_err("should reject tampered payload");
    assert!(err.contains("payload_hash"), "got: {}", err);
}

#[test]
fn blessed_write_rejects_reject_verdict() {
    let mut bus = make_bus_and_register();
    let payload = "by something_bad";
    let receipt = OracleReceipt::new(
        hash_payload(payload),
        hash_context(None),
        TEST_NONCE,
        Verdict::Reject("malformed".into()),
        PredicateKind::Lean4Boolean,
    );
    let err = bus.append_oracle_accepted("Agent_0", payload, None, &receipt)
        .expect_err("Reject verdict must be rejected");
    assert!(err.contains("Reject"), "got: {}", err);
}

#[test]
fn blessed_write_partial_ok_receipt_succeeds() {
    let mut bus = make_bus_and_register();
    let payload = "have h1 : 1 = 1 := rfl";
    let receipt = OracleReceipt::new_lean4_partial(payload, None, TEST_NONCE);
    let res = bus.append_oracle_accepted("Agent_0", payload, None, &receipt)
        .expect("PartialOk receipt should be accepted");
    assert!(matches!(res, BusResult::Appended { .. }));
}

#[test]
fn blessed_write_bypasses_forbidden_patterns_after_valid_receipt() {
    let mut bus = make_bus_and_register();
    // Legitimate proof, even if it contains words that agent scratch would
    // have been vetoed for. The oracle has certified it; bus trusts receipt.
    let legitimate_proof = "by have h : 1 = 1 := rfl; exact h";
    let receipt = OracleReceipt::new_lean4_complete(legitimate_proof, None, TEST_NONCE);
    let res = bus.append_oracle_accepted("Agent_0", legitimate_proof, None, &receipt)
        .expect("legitimate proof must be appended");
    assert!(matches!(res, BusResult::Appended { .. }));
}

#[test]
fn blessed_write_rejects_unregistered_oracle_nonce() {
    // V-1 core VETO fix: receipt from unregistered oracle is unforgeable.
    let mut bus = make_bus_and_register();
    let payload = "by linarith";
    // Construct a receipt with a DIFFERENT nonce (attacker tries to forge).
    let forged = OracleReceipt::new_lean4_complete(payload, None, OTHER_NONCE);
    let err = bus.append_oracle_accepted("Agent_0", payload, None, &forged)
        .expect_err("forged receipt (wrong nonce) must be rejected");
    assert!(err.contains("oracle nonce not registered"), "got: {}", err);
}

#[test]
fn blessed_write_rejects_cross_context_replay() {
    // Step-mode VETO fix: a receipt issued for parent A cannot be replayed
    // for parent B. Prevents tactic hijack across proof chains.
    let mut bus = make_bus_and_register();
    // First append a seed node so we have a real parent id.
    let seed_payload = "seed";
    let seed_receipt = OracleReceipt::new_lean4_complete(seed_payload, None, TEST_NONCE);
    let seed_node = match bus.append_oracle_accepted("Agent_0", seed_payload, None, &seed_receipt).unwrap() {
        BusResult::Appended { node_id } => node_id,
        other => panic!("unexpected: {:?}", other),
    };

    // Issue a receipt bound to parent = None, but attacker tries to replay
    // under parent = seed_node.
    let malicious_payload = "by attack";
    let receipt_for_no_parent = OracleReceipt::new_lean4_complete(malicious_payload, None, TEST_NONCE);
    let err = bus.append_oracle_accepted(
        "Agent_0", malicious_payload, Some(&seed_node), &receipt_for_no_parent,
    ).expect_err("cross-context replay must be rejected");
    assert!(err.contains("context_hash"), "got: {}", err);
}
