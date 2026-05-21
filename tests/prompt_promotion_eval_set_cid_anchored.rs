//! C10 gate: eval_set_cid must always be present in the receipt (no anonymous eval).
//!
//! FC-trace: FC3 (eval evidence binding — audit Agent 3 C.5)
//! Risk class: Class 3

use turingosv4::runtime::prompt_promotion::{
    write_promotion_receipt, PromptPromotionReceipt, PromotionDecision,
    PROMPT_PROMOTION_RECEIPT_SCHEMA_ID, sha256_hex_of_prompt,
};
use turingosv4::runtime::spec_capsule::cas_path;
use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;

#[test]
fn test_eval_set_cid_required_on_write() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let from = sha256_hex_of_prompt(b"from prompt");
    let to = sha256_hex_of_prompt(b"to prompt");

    let receipt = PromptPromotionReceipt {
        schema_id: PROMPT_PROMOTION_RECEIPT_SCHEMA_ID.to_string(),
        from_prompt_cid: from.clone(),
        to_prompt_cid: to.clone(),
        eval_set_cid: String::new(), // EMPTY — must fail
        eval_before_cid: from.clone(),
        eval_after_cid: to.clone(),
        promotion_decision: PromotionDecision::Promote,
        logical_t: 1000,
    };

    let result = write_promotion_receipt(dir.path(), &receipt, 1000);
    assert!(result.is_err(), "empty eval_set_cid must be rejected by write");
}

#[test]
fn test_eval_set_cid_anchored_in_cas_receipt() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();
    let from = sha256_hex_of_prompt(b"from prompt v1");
    let to = sha256_hex_of_prompt(b"to prompt v2");
    let eval_set = "e".repeat(64); // non-empty anchored eval set CID

    let receipt = PromptPromotionReceipt {
        schema_id: PROMPT_PROMOTION_RECEIPT_SCHEMA_ID.to_string(),
        from_prompt_cid: from.clone(),
        to_prompt_cid: to.clone(),
        eval_set_cid: eval_set.clone(),
        eval_before_cid: from.clone(),
        eval_after_cid: to.clone(),
        promotion_decision: PromotionDecision::Promote,
        logical_t: 2000,
    };

    let cid_hex = write_promotion_receipt(ws, &receipt, 2000).expect("write");
    assert!(!cid_hex.is_empty(), "receipt CID must not be empty");

    // Read back from CAS and verify eval_set_cid is anchored.
    let cas_dir = cas_path(ws);
    let mut store = CasStore::open(&cas_dir).expect("open store");
    let _ = store.reload_index_from_sidecar();

    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    let mut found = false;
    for cid in cids {
        let meta = match store.metadata(&cid) { Some(m) => m, None => continue };
        if meta.schema_id.as_deref() != Some(PROMPT_PROMOTION_RECEIPT_SCHEMA_ID) { continue; }
        let bytes = store.get(&cid).expect("read");
        let r: PromptPromotionReceipt = serde_json::from_slice(&bytes).expect("deserialize");
        assert_eq!(r.eval_set_cid, eval_set, "eval_set_cid must be preserved in CAS");
        assert!(!r.eval_set_cid.is_empty(), "eval_set_cid in CAS must never be empty");
        found = true;
    }
    assert!(found, "receipt not found in CAS");
}
