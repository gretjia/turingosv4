//! D1 (OBL-018): the PRODUCTION `lean_market_agent` shared the het probe's false-negative
//! de-alignment bug, and this test pins the FIX.
//!
//! Before the fix, `lean_market_agent` (`src/bin/lean_market_agent.rs:1692-1719`) extracted
//! a model's JSON `proof_body` and handed the RAW body straight to `LeanJudge::verify`
//! (→ `assemble` → the conservative shared `dedent`) with NO `realign`. So a first-line-
//! shallow `proof_body` (first tactic flush, a sibling indented — the model's natural
//! shape) had a CORRECT proof mislabeled Failed (the same false negative that corrupted the
//! het run). D1 confirmed this by real Lean.
//!
//! The fix (lib-refactor): `realign` now lives in `lean_judge` and `lean_market_agent`
//! applies it to the extracted `proof_body` before verify/assemble. This test proves, on
//! the public API + real Lean, that (1) the raw body still fails through the conservative
//! judge — documenting the bug that WAS there — and (2) `lean_judge::realign(body)` (the
//! exact transform `lean_market_agent` now applies) makes the SAME known-good proof verify.

use turingosv4::judges::lean_judge::{default_lean_bin, realign, LeanJudge};

#[test]
fn lean_market_agent_realign_cures_dealign_false_negative() {
    let bin = default_lean_bin();
    if !(bin.is_absolute() && bin.exists()) {
        eprintln!("skip: pinned Lean toolchain not present");
        return;
    }
    let preamble = "theorem t (p q : Prop) (hp : p) (hq : q) : p ∧ q := by";

    // The model's JSON, first tactic flush + indented sibling (the de-aligned shape).
    let raw = r#"{"proof_body":"constructor\n  exact hp\n  exact hq","confidence":0.9}"#;
    let v: serde_json::Value = serde_json::from_str(raw).expect("parse proof_body json");
    let body = v["proof_body"].as_str().expect("proof_body str");

    // BEFORE (the bug D1 confirmed): raw proof_body straight to verify → mislabeled Failed.
    let mut j_raw = LeanJudge::new(preamble);
    j_raw.lean_bin = bin.clone();
    assert!(
        !j_raw.verify(body).is_verified(),
        "precondition: the raw first-line-shallow body must fail through the conservative judge"
    );

    // AFTER (the OBL-018 fix lean_market_agent now applies): realign(body) → verifies.
    let mut j_fixed = LeanJudge::new(preamble);
    j_fixed.lean_bin = bin.clone();
    assert!(
        j_fixed.verify(&realign(body)).is_verified(),
        "fix: lean_market_agent realigns proof_body before verify → known-good proof verifies"
    );
    eprintln!(
        "D1 fix verified: lean_judge::realign cures lean_market_agent's first-line-shallow false negative."
    );
}
