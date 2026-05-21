//! C10 gate: canonical prompt change requires a promotion receipt.
//!
//! Simulates a workspace where the on-disk prompt bytes change (v1 → v2)
//! and verifies the guard correctly requires a receipt matching the v2 SHA-256.
//!
//! FC-trace: FC2 (prompt boot), FC3 (eval evidence binding)
//! Risk class: Class 3

use turingosv4::runtime::prompt_promotion::{
    check_promotion_guard, write_promotion_receipt,
    PromptPromotionReceipt, PromotionDecision,
    sha256_hex_of_prompt, PROMPT_PROMOTION_RECEIPT_SCHEMA_ID,
};

fn make_receipt(
    from: &str,
    to: &str,
    eval_set: &str,
    decision: PromotionDecision,
) -> PromptPromotionReceipt {
    PromptPromotionReceipt {
        schema_id: PROMPT_PROMOTION_RECEIPT_SCHEMA_ID.to_string(),
        from_prompt_cid: from.to_string(),
        to_prompt_cid: to.to_string(),
        eval_set_cid: eval_set.to_string(),
        eval_before_cid: from.to_string(),
        eval_after_cid: to.to_string(),
        promotion_decision: decision,
        logical_t: 1000,
    }
}

#[test]
fn test_canonical_prompt_change_requires_receipt() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();

    // Write v1 and v2 prompts
    let v1 = b"v1 system prompt content";
    let v2 = b"v2 system prompt content (improved)";
    let v1_cid = sha256_hex_of_prompt(v1);
    let v2_cid = sha256_hex_of_prompt(v2);

    // Guard should reject v2 (no receipt yet)
    let r1 = check_promotion_guard(ws, &v2_cid);
    assert!(r1.is_err(), "guard must reject v2 with no receipt");

    // Write a receipt for v1 → v2
    let eval_set = "f".repeat(64);
    let receipt = make_receipt(&v1_cid, &v2_cid, &eval_set, PromotionDecision::Promote);
    write_promotion_receipt(ws, &receipt, 1001).expect("write");

    // Guard now passes for v2
    let r2 = check_promotion_guard(ws, &v2_cid);
    assert!(r2.is_ok(), "guard must pass for v2 after receipt: {:?}", r2);

    // Guard still fails for a hypothetical v3 (no receipt)
    let v3_cid = sha256_hex_of_prompt(b"v3 prompt hypothetical");
    let r3 = check_promotion_guard(ws, &v3_cid);
    assert!(r3.is_err(), "guard must fail for v3 with no receipt: {:?}", r3);
}

#[test]
fn test_receipt_presence_flips_guard() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();

    let from_cid = sha256_hex_of_prompt(b"from bytes");
    let to_cid = sha256_hex_of_prompt(b"to bytes v2 alt");
    let eval_set = "9".repeat(64);

    // No receipt → fails
    assert!(check_promotion_guard(ws, &to_cid).is_err());

    // Add promote receipt → passes
    let receipt = make_receipt(&from_cid, &to_cid, &eval_set, PromotionDecision::Promote);
    write_promotion_receipt(ws, &receipt, 2000).expect("write");
    assert!(check_promotion_guard(ws, &to_cid).is_ok(), "guard must pass after receipt");
}

#[test]
fn test_receipt_with_mismatched_to_cid_does_not_help() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();

    let from_cid = sha256_hex_of_prompt(b"from bytes ok");
    let to_cid = sha256_hex_of_prompt(b"actual to bytes");
    let wrong_to_cid = sha256_hex_of_prompt(b"wrong to bytes");
    let eval_set = "7".repeat(64);

    // Write receipt for wrong_to_cid
    let receipt = make_receipt(&from_cid, &wrong_to_cid, &eval_set, PromotionDecision::Promote);
    write_promotion_receipt(ws, &receipt, 3000).expect("write");

    // Guard for to_cid must still fail (wrong to_cid)
    let result = check_promotion_guard(ws, &to_cid);
    assert!(result.is_err(), "guard must fail when receipt has wrong to_prompt_cid");
}
