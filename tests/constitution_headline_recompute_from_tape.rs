//! Forensic gate vs failure-mode #1 (replay-green ≠ correctness) — 2026-06-01
//! retrospective (`handover/reports/SESSION_FORENSIC_RETROSPECTIVE_2026-06-01.md` §1.D).
//!
//! The tape chain verifier (`market_tape_shared::verify_chain` / `verify_chain_lines`)
//! checks **byte-integrity ONLY** — it never recomputes the policy result. This session
//! treated "replay-green / matrix_drift 3/3" as a correctness warrant on flagship arms
//! whose headline Δ numbers were **not on any tape**, and whose manifest was therefore
//! never cross-checked against a recomputation. The badge can be green while the
//! headline is fabricated.
//!
//! This gate locks the discipline that catches that class, structurally:
//!   1. RECOMPUTE: the headline integers (banked, cost, tokens, llm_calls) are
//!      reconstructed from the frozen tape LINES ALONE via the shared `derive_*` — and
//!      reproduce an HONEST manifest exactly.
//!   2. RECOMPUTE CATCHES A LYING MANIFEST: a manifest that disagrees with the tape is
//!      caught by `derive_* != manifest`, even though…
//!   3. …the BYTE CHAIN STAYS GREEN under that same lying manifest — proving
//!      `verify_chain_lines` is **anti-tamper + provenance-pinned, NOT a correctness
//!      warrant**. (Relabel the badge accordingly; see AGENTS.md §17.)
//!   4. The `derive_*` are a function of the TAPE, not the manifest (mutate the tape →
//!      derived moves; manifest fixed) — so they cannot be a silent read-back.
//!
//! The verifier substrate is pulled in EXACTLY as the producer bin (`lean_hayek_market`)
//! and the standalone verifier (`verify_market_tape`) pull it — `#[path]`, no lib.rs
//! `mod` (adding a `mod` to lib.rs is a trust-root touch). Per AGENTS.md §7 each
//! property is paired with a control proving the check bites.

#[path = "../src/market_tape_shared.rs"]
mod market_tape_shared;
use market_tape_shared as mt;
use mt::{MarketEvent, MarketTape};

/// A small, fully-deterministic, chain-valid tape: 2 claims banked (Verify-true), 1
/// failed attempt (Verify-false), 3 LLM calls. GenesisPin is the mandatory first event.
fn synthetic_tape() -> Vec<String> {
    let mut t = MarketTape::new();
    t.record(&MarketEvent::GenesisPin {
        run_id: "forensic-recompute-fixture".into(),
        seed: 7,
        policy: "market".into(),
        model_roster: vec!["deepseek".into()],
        budget_b: 1000,
        axiom_whitelist: vec!["propext".into(), "Classical.choice".into(), "Quot.sound".into()],
        head_commit_sha: "unknown".into(),
    });
    t.record(&MarketEvent::MarketOpen { claim: 0, claim_type: "long".into() });
    t.record(&MarketEvent::Invest {
        agent: 0,
        claim: 0,
        side: "YES".into(),
        amount_micro: 1_000,
        model_hash: "h0".into(),
        confidence: 80,
    });
    t.record(&MarketEvent::LlmCall { model: "deepseek".into(), prompt_tokens: 100, completion_tokens: 200 });
    t.record(&MarketEvent::Verify { claim: 0, verdict: true, reject_class: "".into() });
    t.record(&MarketEvent::MarketOpen { claim: 1, claim_type: "long".into() });
    t.record(&MarketEvent::LlmCall { model: "deepseek".into(), prompt_tokens: 50, completion_tokens: 50 });
    t.record(&MarketEvent::Verify { claim: 1, verdict: false, reject_class: "lean_error".into() });
    t.record(&MarketEvent::LlmCall { model: "deepseek".into(), prompt_tokens: 80, completion_tokens: 120 });
    t.record(&MarketEvent::Verify { claim: 1, verdict: true, reject_class: "".into() });
    assert!(t.verify_chain(), "fixture tape must be chain-valid by construction");
    t.lines
}

/// The honest manifest = what an unbugged producer writes (each field IS the tape's
/// recomputed value). Built once from the tape so the test asserts reproduction, not a
/// hand-copied magic number.
fn honest_manifest(lines: &[String]) -> serde_json::Value {
    serde_json::json!({
        "banked": mt::derive_banked(lines),
        "micro_usd": mt::derive_cost(lines),
        "total_completion_tokens": mt::derive_total_completion(lines),
        "llm_calls": mt::derive_llm_calls(lines),
    })
}

#[test]
fn headline_integers_recompute_from_tape() {
    let lines = synthetic_tape();
    // banked = distinct claims with a Verify{true}: claim 0 and claim 1 → 2.
    assert_eq!(mt::derive_banked(&lines), 2, "banked recomputed from Verify-true events");
    // 3 LLMCall events; total completion = 200 + 50 + 120 = 370.
    assert_eq!(mt::derive_llm_calls(&lines), 3);
    assert_eq!(mt::derive_total_completion(&lines), 370);
    // 1 failed branch on tape (Verify-false).
    assert_eq!(mt::derive_failures(&lines), 1);
    // cost recomputed via the shared MODEL_RATES — positive and equal to the per-call sum.
    let by_call: i64 = [(100u64, 200u64), (50, 50), (80, 120)]
        .iter()
        .map(|&(p, c)| mt::call_micro_usd("deepseek", p, c))
        .sum();
    assert_eq!(mt::derive_cost(&lines), by_call, "derive_cost == Σ call_micro_usd over LLMCall events");
    assert!(by_call > 0);
}

#[test]
fn recompute_reproduces_an_honest_manifest() {
    let lines = synthetic_tape();
    let m = honest_manifest(&lines);
    // The verifier's contract: every derived integer equals the manifest integer.
    assert_eq!(mt::derive_banked(&lines) as i64, m["banked"].as_i64().unwrap());
    assert_eq!(mt::derive_cost(&lines), m["micro_usd"].as_i64().unwrap());
    assert_eq!(mt::derive_total_completion(&lines) as i64, m["total_completion_tokens"].as_i64().unwrap());
    assert_eq!(mt::derive_llm_calls(&lines) as i64, m["llm_calls"].as_i64().unwrap());
}

#[test]
fn recompute_catches_a_lying_manifest_while_the_byte_chain_stays_green() {
    // THE failure-mode-#1 case. A manifest claims a headline the tape does not support
    // (banked 99, cost 1, tokens 9_999_999). The tape itself is untouched and BYTE-VALID.
    let lines = synthetic_tape();
    let lying = serde_json::json!({
        "banked": 99,
        "micro_usd": 1,
        "total_completion_tokens": 9_999_999,
        "llm_calls": 99,
    });

    // (a) the byte chain is GREEN — verify_chain_lines cannot see the lie. This is the
    //     anti-tamper badge; it is NOT a correctness warrant.
    assert!(
        mt::verify_chain_lines(&lines),
        "byte chain must stay green: the lie is in the manifest, not the tape bytes"
    );
    assert!(mt::first_is_genesis(&lines), "GenesisPin-first must hold (provenance pin)");

    // (b) RECOMPUTE catches every lied-about field.
    assert_ne!(mt::derive_banked(&lines) as i64, lying["banked"].as_i64().unwrap(),
        "recompute must reject a fabricated banked count");
    assert_ne!(mt::derive_cost(&lines), lying["micro_usd"].as_i64().unwrap(),
        "recompute must reject a fabricated cost");
    assert_ne!(mt::derive_total_completion(&lines) as i64, lying["total_completion_tokens"].as_i64().unwrap(),
        "recompute must reject fabricated token totals");

    // (c) the conjunction the gate enforces: a real headline needs BOTH a green chain
    //     AND a recompute-match. Green-chain alone admits the lie; recompute closes it.
    let chain_green = mt::verify_chain_lines(&lines);
    let recompute_match = mt::derive_banked(&lines) as i64 == lying["banked"].as_i64().unwrap();
    assert!(chain_green && !recompute_match,
        "demonstrates byte-chain-green ∧ ¬recompute-match — exactly the gap the badge hid");
}

#[test]
fn derived_values_are_a_function_of_the_tape_not_the_manifest() {
    // Recompute cannot be a silent read-back of the manifest: append one more banked
    // claim to the TAPE and the derived banked MUST move, with no manifest involved.
    let base = synthetic_tape();
    let banked_before = mt::derive_banked(&base);

    let mut t = MarketTape::new();
    // rebuild the same prefix, then extend with a third banked claim.
    t.record(&MarketEvent::GenesisPin {
        run_id: "forensic-recompute-fixture".into(),
        seed: 7,
        policy: "market".into(),
        model_roster: vec!["deepseek".into()],
        budget_b: 1000,
        axiom_whitelist: vec!["propext".into(), "Classical.choice".into(), "Quot.sound".into()],
        head_commit_sha: "unknown".into(),
    });
    t.record(&MarketEvent::Verify { claim: 0, verdict: true, reject_class: "".into() });
    t.record(&MarketEvent::Verify { claim: 1, verdict: true, reject_class: "".into() });
    t.record(&MarketEvent::Verify { claim: 2, verdict: true, reject_class: "".into() });
    let banked_after = mt::derive_banked(&t.lines);

    assert_eq!(banked_before, 2);
    assert_eq!(banked_after, 3, "derive_banked tracks the tape's Verify-true claims, not any manifest");
    assert!(banked_after > banked_before);
}

// ── Controls (§7 — the byte chain IS a real anti-tamper mechanism; prove it bites) ──

#[test]
fn one_byte_tamper_breaks_the_chain() {
    let mut lines = synthetic_tape();
    assert!(mt::verify_chain_lines(&lines), "pristine tape is chain-valid");
    // Flip a digit in the Invest amount on line 2 (index 2). The hash chain over the
    // following lines no longer matches → verify_chain_lines must reject.
    lines[2] = lines[2].replacen("1000", "9000", 1);
    assert!(
        !mt::verify_chain_lines(&lines),
        "a one-byte payload tamper must break the prev_hash chain (anti-tamper still works)"
    );
}

#[test]
fn genesis_first_invariant_is_enforced() {
    // A headline-bearing tape MUST open with GenesisPin (the provenance pin the flagship
    // arms omitted). A tape missing it must be rejected by the entry check.
    let lines = synthetic_tape();
    assert!(mt::first_is_genesis(&lines));
    let no_genesis: Vec<String> = lines.into_iter().skip(1).collect();
    assert!(
        !mt::first_is_genesis(&no_genesis),
        "a tape whose first event is not GenesisPin must fail the provenance entry check"
    );
}
