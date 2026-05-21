//! C10 gate: prompt promotion requires an eval receipt.
//!
//! Verifies that check_promotion_guard returns Err when no receipt exists,
//! and Ok when a valid Promote receipt is written to CAS.
//!
//! FC-trace: FC2 (prompt boot), FC3 (eval evidence binding)
//! Risk class: Class 3

use std::time::{SystemTime, UNIX_EPOCH};
use turingosv4::runtime::prompt_promotion::{
    check_promotion_guard, write_promotion_receipt,
    PromptPromotionReceipt, PromotionDecision,
    sha256_hex_of_prompt, PROMPT_PROMOTION_RECEIPT_SCHEMA_ID,
};
use turingosv4::runtime::spec_capsule::cas_path;

fn now_t() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1000)
}

fn make_receipt(
    from_cid: &str,
    to_cid: &str,
    eval_set: &str,
    decision: PromotionDecision,
) -> PromptPromotionReceipt {
    PromptPromotionReceipt {
        schema_id: PROMPT_PROMOTION_RECEIPT_SCHEMA_ID.to_string(),
        from_prompt_cid: from_cid.to_string(),
        to_prompt_cid: to_cid.to_string(),
        eval_set_cid: eval_set.to_string(),
        eval_before_cid: from_cid.to_string(),
        eval_after_cid: to_cid.to_string(),
        promotion_decision: decision,
        logical_t: now_t(),
    }
}

#[test]
fn test_guard_fails_without_receipt() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    // Ensure CAS dir exists but has no receipts
    let cas = cas_path(dir.path());
    std::fs::create_dir_all(&cas).expect("create cas dir");
    let _ = turingosv4::bottom_white::cas::store::CasStore::open(&cas).expect("open");

    let to_cid = sha256_hex_of_prompt(b"v2 prompt content");
    let result = check_promotion_guard(dir.path(), &to_cid);
    assert!(result.is_err(), "guard must fail with no receipt: {:?}", result);
    let msg = format!("{}", result.unwrap_err());
    assert!(msg.contains("no PromptPromotionReceipt"), "expected guard message: {msg}");
}

#[test]
fn test_guard_passes_with_promote_receipt() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();

    let from_bytes = b"v1 prompt content";
    let to_bytes = b"v2 prompt content";
    let from_cid = sha256_hex_of_prompt(from_bytes);
    let to_cid = sha256_hex_of_prompt(to_bytes);
    let eval_set = "a".repeat(64);

    let receipt = make_receipt(&from_cid, &to_cid, &eval_set, PromotionDecision::Promote);
    write_promotion_receipt(ws, &receipt, now_t()).expect("write receipt");

    let result = check_promotion_guard(ws, &to_cid);
    assert!(result.is_ok(), "guard must pass with Promote receipt: {:?}", result);
}

#[test]
fn test_guard_blocks_with_reject_receipt() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();

    let from_cid = sha256_hex_of_prompt(b"v1 prompt");
    let to_cid = sha256_hex_of_prompt(b"v2 prompt bad");
    let eval_set = "b".repeat(64);

    let receipt = make_receipt(&from_cid, &to_cid, &eval_set, PromotionDecision::Reject);
    write_promotion_receipt(ws, &receipt, now_t()).expect("write receipt");

    let result = check_promotion_guard(ws, &to_cid);
    assert!(result.is_err(), "guard must block with Reject receipt");
    let msg = format!("{}", result.unwrap_err());
    assert!(msg.contains("decision=reject"), "expected reject message: {msg}");
}

#[test]
fn test_guard_rejects_empty_eval_set_cid() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let from_cid = sha256_hex_of_prompt(b"p1");
    let to_cid = sha256_hex_of_prompt(b"p2");
    let receipt = make_receipt(&from_cid, &to_cid, "", PromotionDecision::Promote);
    let result = write_promotion_receipt(dir.path(), &receipt, now_t());
    assert!(result.is_err(), "write must fail with empty eval_set_cid");
}

#[test]
fn test_guard_passes_after_rollback_receipt() {
    // Rollback: new receipt from v2 → v1 with Promote reverts.
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();

    let v1_cid = sha256_hex_of_prompt(b"original v1 prompt");
    let v2_cid = sha256_hex_of_prompt(b"v2 prompt");
    let eval_set = "c".repeat(64);

    // First promote v1 → v2.
    let receipt_fwd = make_receipt(&v1_cid, &v2_cid, &eval_set, PromotionDecision::Promote);
    write_promotion_receipt(ws, &receipt_fwd, now_t()).expect("write v1→v2 receipt");

    // Now rollback: promote v2 → v1 (to_prompt_cid = v1_cid).
    let receipt_rev = make_receipt(&v2_cid, &v1_cid, &eval_set, PromotionDecision::Promote);
    write_promotion_receipt(ws, &receipt_rev, now_t() + 1).expect("write v2→v1 receipt");

    // Guard for v1 should now pass (rollback receipt).
    let result = check_promotion_guard(ws, &v1_cid);
    assert!(result.is_ok(), "guard must pass after rollback: {:?}", result);
}
