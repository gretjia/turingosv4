// Phase 8.C v3 (Codex VETO → R1-α / C-067) regression test
// Ed25519 capability: bus rejects blessed write if ANY of:
//   (a) payload-hash binding (content tampered)
//   (b) context-hash binding (cross-parent replay)
//   (c) signature doesn't verify under issuer_pub
//   (d) issuer_pub not registered with this bus
//   (e) oracles_frozen and caller tries to register new pubkey post-init

use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
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
    TuringBus::new(kernel, config)
}

fn bus_with_registered(sk: &SigningKey) -> TuringBus {
    let mut bus = make_bus();
    bus.register_oracle(sk.verifying_key().to_bytes()).expect("register_oracle pre-init");
    bus.mount_tool(Box::new(WalletTool::new(10_000.0)));
    bus.init(&["Agent_0".into()]);
    bus
}

#[test]
fn blessed_write_accepts_matching_receipt() {
    let sk = SigningKey::generate(&mut OsRng);
    let mut bus = bus_with_registered(&sk);
    let payload = "by linarith";
    let receipt = OracleReceipt::sign_new(
        payload, None, Verdict::Complete, PredicateKind::Lean4Boolean, &sk,
    );
    let res = bus.append_oracle_accepted("Agent_0", payload, None, &receipt)
        .expect("matching receipt should be accepted");
    assert!(matches!(res, BusResult::Appended { .. }));
}

#[test]
fn blessed_write_rejects_hash_mismatch() {
    let sk = SigningKey::generate(&mut OsRng);
    let mut bus = bus_with_registered(&sk);
    let receipt = OracleReceipt::sign_new(
        "by linarith", None, Verdict::Complete, PredicateKind::Lean4Boolean, &sk,
    );
    let err = bus.append_oracle_accepted(
        "Agent_0", "by native_decide", None, &receipt,
    ).expect_err("tampered payload must be rejected");
    assert!(err.contains("payload_hash"), "got: {}", err);
}

#[test]
fn blessed_write_rejects_reject_verdict() {
    let sk = SigningKey::generate(&mut OsRng);
    let mut bus = bus_with_registered(&sk);
    let payload = "by something_bad";
    let receipt = OracleReceipt::sign_new(
        payload, None,
        Verdict::Reject("malformed".into()),
        PredicateKind::Lean4Boolean, &sk,
    );
    let err = bus.append_oracle_accepted("Agent_0", payload, None, &receipt)
        .expect_err("Reject verdict must be rejected");
    assert!(err.contains("Reject"), "got: {}", err);
}

#[test]
fn blessed_write_partial_ok_receipt_succeeds() {
    let sk = SigningKey::generate(&mut OsRng);
    let mut bus = bus_with_registered(&sk);
    let payload = "have h1 : 1 = 1 := rfl";
    let receipt = OracleReceipt::sign_new(
        payload, None,
        Verdict::PartialOk { confidence: 1.0 },
        PredicateKind::Lean4Boolean, &sk,
    );
    let res = bus.append_oracle_accepted("Agent_0", payload, None, &receipt)
        .expect("PartialOk receipt should be accepted");
    assert!(matches!(res, BusResult::Appended { .. }));
}

#[test]
fn blessed_write_rejects_unregistered_issuer() {
    // R1-α core fix: receipt from an oracle whose pubkey was never
    // registered is rejected before signature check.
    let real_sk = SigningKey::generate(&mut OsRng);
    let mut bus = bus_with_registered(&real_sk);
    // Attacker generates own key.
    let attacker_sk = SigningKey::generate(&mut OsRng);
    let payload = "by linarith";
    let receipt = OracleReceipt::sign_new(
        payload, None, Verdict::Complete, PredicateKind::Lean4Boolean, &attacker_sk,
    );
    let err = bus.append_oracle_accepted("Agent_0", payload, None, &receipt)
        .expect_err("unregistered issuer must be rejected");
    assert!(err.contains("issuer pubkey not registered"), "got: {}", err);
}

#[test]
fn blessed_write_rejects_cross_context_replay() {
    let sk = SigningKey::generate(&mut OsRng);
    let mut bus = bus_with_registered(&sk);
    // Seed a real node so we have a valid parent id.
    let seed_receipt = OracleReceipt::sign_new(
        "seed", None, Verdict::Complete, PredicateKind::Lean4Boolean, &sk,
    );
    let seed_node = match bus.append_oracle_accepted("Agent_0", "seed", None, &seed_receipt).unwrap() {
        BusResult::Appended { node_id } => node_id,
        other => panic!("unexpected: {:?}", other),
    };
    // Receipt issued for parent = None, attacker tries parent = seed_node.
    let r = OracleReceipt::sign_new(
        "by attack", None, Verdict::Complete, PredicateKind::Lean4Boolean, &sk,
    );
    let err = bus.append_oracle_accepted(
        "Agent_0", "by attack", Some(&seed_node), &r,
    ).expect_err("cross-context replay must be rejected");
    assert!(err.contains("context_hash"), "got: {}", err);
}

#[test]
fn post_init_register_oracle_fails() {
    // R1-α freeze: once init() runs, register_oracle returns Err so an
    // attacker with &mut Bus can't inject their own trusted pubkey.
    let sk = SigningKey::generate(&mut OsRng);
    let bus = bus_with_registered(&sk);  // init already done inside helper
    assert!(bus.oracles_frozen(), "init must have frozen oracle registration");
    // Try to register a new (attacker) pubkey post-init.
    let mut bus = bus;
    let attacker_sk = SigningKey::generate(&mut OsRng);
    let err = bus.register_oracle(attacker_sk.verifying_key().to_bytes())
        .expect_err("post-init register_oracle must return Err");
    assert!(err.contains("frozen"), "got: {}", err);
}

#[test]
fn attacker_with_mut_bus_cannot_forge_post_init() {
    // End-to-end attack scenario from Codex VETO:
    //  1. Attacker has &mut Bus (e.g., via test helper harness).
    //  2. Attacker generates own SigningKey.
    //  3. Attacker tries to register own pubkey → FAILS (frozen).
    //  4. Attacker tries to submit receipt signed by their key → rejected
    //     at the `issuer pubkey not registered` step.
    let real_sk = SigningKey::generate(&mut OsRng);
    let mut bus = bus_with_registered(&real_sk);

    let attacker_sk = SigningKey::generate(&mut OsRng);
    // Step 3: register fails
    let reg_err = bus.register_oracle(attacker_sk.verifying_key().to_bytes())
        .expect_err("frozen");
    assert!(reg_err.contains("frozen"));

    // Step 4: receipt signed by attacker, sent anyway
    let payload = "by native_decide";  // attempting to bypass forbidden_patterns
    let forged = OracleReceipt::sign_new(
        payload, None, Verdict::Complete, PredicateKind::Lean4Boolean, &attacker_sk,
    );
    let err = bus.append_oracle_accepted("Agent_0", payload, None, &forged)
        .expect_err("forged receipt from unregistered attacker must be rejected");
    assert!(err.contains("issuer pubkey not registered"),
        "attacker receipt must be rejected at registration check; got: {}", err);
}
