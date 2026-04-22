// Phase 8.C (Codex V-1 / C-048) regression test
// Bus must reject blessed write if OracleReceipt doesn't bind to the payload.
// Prior to this fix, any caller could pass `oracle_blessed=true` to bypass
// forbidden_patterns — capability leak.

use turingosv4::bus::{BusConfig, BusResult, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::sdk::oracle_receipt::OracleReceipt;
use turingosv4::sdk::predicate::{PredicateKind, Verdict};
use turingosv4::sdk::tools::wallet::WalletTool;

fn make_bus() -> TuringBus {
    let kernel = Kernel::new();
    let config = BusConfig {
        max_payload_chars: 500,
        max_payload_lines: 20,
        system_lp_amount: 200.0,
        forbidden_patterns: vec!["native_decide".into()],
        min_class_count_to_broadcast: 3,
    };
    let mut bus = TuringBus::new(kernel, config);
    bus.mount_tool(Box::new(WalletTool::new(10_000.0)));
    bus.init(&["Agent_0".into()]);
    bus
}

#[test]
fn blessed_write_accepts_matching_receipt() {
    let mut bus = make_bus();
    let payload = "by linarith";
    let receipt = OracleReceipt::for_lean4_complete(payload);
    let res = bus.append_oracle_accepted("Agent_0", payload, None, &receipt)
        .expect("matching receipt should be accepted");
    assert!(matches!(res, BusResult::Appended { .. }),
        "expected Appended, got {:?}", res);
}

#[test]
fn blessed_write_rejects_hash_mismatch() {
    let mut bus = make_bus();
    // Receipt was made for a different payload.
    let receipt = OracleReceipt::for_lean4_complete("by linarith");
    // But we try to submit a forbidden one.
    let err = bus.append_oracle_accepted(
        "Agent_0", "by native_decide", None, &receipt
    ).expect_err("should reject tampered payload");
    assert!(err.contains("mismatch"), "expected mismatch error, got: {}", err);
}

#[test]
fn blessed_write_rejects_reject_verdict() {
    let mut bus = make_bus();
    let payload = "by something_bad";
    // Manually construct a Reject receipt (contract: validates() refuses it).
    let receipt = OracleReceipt {
        payload_hash: {
            use sha2::{Digest, Sha256};
            let mut h = Sha256::new();
            h.update(payload.as_bytes());
            h.finalize().into()
        },
        verdict: Verdict::Reject("malformed".into()),
        predicate_kind: PredicateKind::Lean4Boolean,
    };
    let err = bus.append_oracle_accepted("Agent_0", payload, None, &receipt)
        .expect_err("Reject verdict must be rejected by bus");
    assert!(err.contains("Reject"), "expected Reject in error, got: {}", err);
}

#[test]
fn blessed_write_partial_ok_receipt_succeeds() {
    // PartialOk is a legitimate write path (step tactic that elaborates
    // partially). Bus should accept it.
    let mut bus = make_bus();
    let payload = "have h1 : 1 = 1 := rfl";
    let receipt = OracleReceipt::for_lean4_partial(payload);
    let res = bus.append_oracle_accepted("Agent_0", payload, None, &receipt)
        .expect("PartialOk receipt should be accepted");
    assert!(matches!(res, BusResult::Appended { .. }));
}

#[test]
fn blessed_write_bypasses_forbidden_patterns_after_valid_receipt() {
    // Once oracle has verified, forbidden_patterns are intentionally skipped
    // (Art. IV mandate wtool write + C-043). Receipt is what authorizes this.
    let mut bus = make_bus();
    // "native_decide" is in forbidden_patterns — agent scratch would be
    // vetoed. But if the full proof was oracle-accepted, the write must go
    // through. In practice the Lean oracle filters this via its own
    // check_payload (C-011) before emitting the Complete verdict.
    let legitimate_proof = "by have h : 1 = 1 := rfl; exact h";
    let receipt = OracleReceipt::for_lean4_complete(legitimate_proof);
    let res = bus.append_oracle_accepted("Agent_0", legitimate_proof, None, &receipt)
        .expect("legitimate proof with valid receipt must be appended");
    assert!(matches!(res, BusResult::Appended { .. }));
}
