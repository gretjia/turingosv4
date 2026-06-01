//! TP-0A.7 conformance gate for the MarketTape-lite PPUT replay substrate.
//!
//! Deterministic, no-LLM, MUST-be-able-to-fail tests of the tape → manifest reconstruction contract:
//! GenesisPin-first, hash-chain tamper-evidence, failure branches on tape, and the derive_* headline
//! reconstruction (banked / cost / cost_of_pass / tokens / llm_calls) recomputed from the tape ALONE.
//! Pulls in the SAME shared module the producer bin + verify_market_tape link, via #[path].

#[path = "../src/market_tape_shared.rs"]
mod mt;
use mt::{MarketEvent, MarketTape};

/// A representative pinned-arm tape: GenesisPin first, two LLM calls (reasoner + a Qwen bettor), one banked
/// theorem (Verify true → Resolve YES) and one failed branch (Verify false → Resolve NO).
fn fixture() -> MarketTape {
    let mut t = MarketTape::new();
    t.record(&MarketEvent::GenesisPin {
        run_id: "test__seed7".into(), seed: 7, policy: "market".into(),
        model_roster: vec!["deepseek-reasoner".into(), "Qwen/Qwen3-32B".into()],
        budget_b: 4000, axiom_whitelist: vec!["propext".into(), "Classical.choice".into(), "Quot.sound".into()],
        head_commit_sha: "unknown".into(),
    });
    t.record(&MarketEvent::LlmCall { model: "deepseek-reasoner".into(), prompt_tokens: 100, completion_tokens: 200 });
    t.record(&MarketEvent::Verify { claim: 0, verdict: true, reject_class: "none".into() });
    t.record(&MarketEvent::Resolve { claim: 0, outcome: "YES".into() });
    t.record(&MarketEvent::LlmCall { model: "Qwen/Qwen3-32B".into(), prompt_tokens: 50, completion_tokens: 80 });
    t.record(&MarketEvent::Verify { claim: 1, verdict: false, reject_class: "reasoner_failed".into() }); // failed branch
    t.record(&MarketEvent::Resolve { claim: 1, outcome: "NO".into() });
    t
}

#[test]
fn headline_reconstructs_from_tape_alone() {
    let t = fixture();
    let l = &t.lines;
    // banked = count Resolve-YES = 1
    assert_eq!(mt::derive_banked(l), 1, "banked@B = one Resolve-YES");
    // cost = Σ call_micro_usd over LLMCall (recomputed from tape, never read from a manifest)
    let expect_cost = mt::call_micro_usd("deepseek-reasoner", 100, 200) + mt::call_micro_usd("Qwen/Qwen3-32B", 50, 80);
    assert_eq!(mt::derive_cost(l), expect_cost, "cost = Σ call_micro_usd(LLMCall)");
    assert!(expect_cost > 0, "fixture exercises a non-trivial cost");
    // cost-of-pass = cost / banked
    assert_eq!(mt::derive_cost_of_pass(l), expect_cost / 1);
    // total completion tokens = 200 + 80
    assert_eq!(mt::derive_total_completion(l), 280);
    assert_eq!(mt::derive_llm_calls(l), 2);
}

#[test]
fn genesis_pin_must_be_first() {
    let t = fixture();
    assert!(mt::first_is_genesis(&t.lines), "fixture starts with GenesisPin");
    // a tape that does NOT lead with GenesisPin must fail the invariant (the gate can fail)
    let mut bad = MarketTape::new();
    bad.record(&MarketEvent::LlmCall { model: "deepseek-reasoner".into(), prompt_tokens: 1, completion_tokens: 1 });
    assert!(!mt::first_is_genesis(&bad.lines), "non-genesis-first tape is rejected");
    // and the pinned identity is recoverable from the good tape
    let g = mt::derive_genesis(&t.lines).expect("genesis present");
    assert_eq!(g["seed"], 7);
    assert_eq!(g["budget_b"], 4000);
    assert!(g["axiom_whitelist"].as_array().unwrap().iter().any(|x| x == "Classical.choice"));
}

#[test]
fn one_byte_tamper_breaks_the_chain() {
    let t = fixture();
    assert!(mt::verify_chain_lines(&t.lines), "untampered chain verifies");
    // flip one byte in a middle line → the prev_hash chain must break (tamper-evident)
    let mut tampered = t.lines.clone();
    let mid = tampered.len() / 2;
    let bytes: Vec<char> = tampered[mid].chars().collect();
    let pos = bytes.iter().position(|c| c.is_ascii_digit()).unwrap_or(1);
    let mut s: Vec<char> = bytes;
    s[pos] = if s[pos] == '9' { '0' } else { ((s[pos] as u8) + 1) as char };
    tampered[mid] = s.into_iter().collect();
    assert!(!mt::verify_chain_lines(&tampered), "a one-byte tamper is detected");
}

#[test]
fn failed_branches_appear_on_tape() {
    let t = fixture();
    // the parse-fail/Lean-rejected attempt is auditable as a Verify{verdict:false} node
    assert_eq!(mt::derive_failures(&t.lines), 1, "one failed branch on tape");
}

#[test]
fn derive_cost_is_recomputed_not_read() {
    // structural guarantee: derive_cost takes ONLY the tape lines — there is no manifest parameter, so the
    // PPUT cost cannot be a read-back of a producer-reported number.
    let t = fixture();
    let from_tape = mt::derive_cost(&t.lines);
    // a manifest claiming a different cost is IRRELEVANT to the derivation (recompute wins).
    let lying_manifest_cost = from_tape + 999_999;
    assert_ne!(from_tape, lying_manifest_cost);
    assert_eq!(mt::derive_cost(&t.lines), from_tape, "recompute is stable + manifest-independent");
}

/// End-to-end contract of the verify_market_tape BIN (not just derive_*): exit 0 on a fixture tape whose
/// manifest matches the reconstruction, exit non-zero when the manifest disagrees with the frozen tape.
#[test]
fn verify_market_tape_bin_exit_code_contract() {
    let t = fixture();
    let dir = std::env::temp_dir();
    let tape_path = dir.join("tp0a_fixture.tape");
    std::fs::write(&tape_path, t.lines.join("\n")).unwrap();
    let l = &t.lines;
    let good = serde_json::json!({
        "schema": "lean_hayek_alloc.v2", "banked_at_B": mt::derive_banked(l), "micro_usd": mt::derive_cost(l),
        "cost_of_pass_micro_usd": mt::derive_cost_of_pass(l), "reasoner_completion_tokens": mt::derive_total_completion(l),
        "chat_completion_tokens": 0, "llm_calls": mt::derive_llm_calls(l),
    });
    let mpath = dir.join("tp0a_fixture.json");
    std::fs::write(&mpath, serde_json::to_string(&good).unwrap()).unwrap();
    let bin = env!("CARGO_BIN_EXE_verify_market_tape");
    let run = |mp: &std::path::Path| std::process::Command::new(bin)
        .args(["--tape", tape_path.to_str().unwrap(), "--manifest", mp.to_str().unwrap(), "--out", dir.join("tp0a_rr.json").to_str().unwrap()])
        .output().unwrap();
    assert!(run(&mpath).status.success(), "verify_market_tape exits 0 on a matching fixture");
    // a manifest that disagrees with the frozen tape (wrong banked) must be rejected (exit != 0)
    let mut bad = good.clone();
    bad["banked_at_B"] = serde_json::json!(mt::derive_banked(l) + 99);
    std::fs::write(&mpath, serde_json::to_string(&bad).unwrap()).unwrap();
    assert!(!run(&mpath).status.success(), "verify_market_tape rejects a manifest that disagrees with the tape");
}
