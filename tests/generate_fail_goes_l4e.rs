//! C8 gate: rejection capsules go to the L4.E lane (EvidenceCapsule with
//! rejection schema_id), never into the L4 accepted state ledger.
//!
//! Verifies that `write_generate_rejection_capsule()` produces a CAS object
//! whose `ObjectType` is `EvidenceCapsule` and whose `schema_id` is
//! `turingos-generate-rejection-v1`. Confirms separation from any L4
//! "accepted" lane (the rejection capsule must NOT be tagged with any
//! schema_id implying acceptance).
//!
//! FC-trace: FC1 (failure-path externalization), FC3 (L4.E binding)
//! Risk class: Class 3

use std::time::{SystemTime, UNIX_EPOCH};

use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::generation_attempt::{
    GenerateRejectionCapsule, RejectClass, write_generate_rejection_capsule,
    GENERATE_REJECTION_CAPSULE_SCHEMA_ID,
};
use turingosv4::runtime::spec_capsule::cas_path;

fn now_t() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1000)
}

#[test]
fn test_rejection_capsule_lands_in_l4e_lane() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let workspace = dir.path();

    let t = now_t();
    let rej = GenerateRejectionCapsule {
        schema_id: GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string(),
        session_id: "test-l4e".to_string(),
        spec_capsule_cid: None,
        generation_attempt_cid: None,
        triage_attempted: false,
        reject_class: RejectClass::LlmApiError,
        public_error_summary: "LLM API call failed".to_string(),
        reason: "llm_api_error".to_string(),
        private_diagnostic_cid: None,
        retryable: true,
        world_head_unchanged: true,
        logical_t: t,
    };

    let cid_hex = write_generate_rejection_capsule(workspace, &rej).expect("write rejection");

    // Open CAS and verify the capsule exists with the right schema_id.
    let cas_dir = cas_path(workspace);
    let mut store = CasStore::open(&cas_dir).expect("open cas");
    let _ = store.reload_index_from_sidecar();

    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    let mut found = false;
    for cid in cids {
        if cid.hex() != cid_hex {
            continue;
        }
        let meta = store.metadata(&cid).expect("metadata");
        assert_eq!(
            meta.schema_id.as_deref(),
            Some(GENERATE_REJECTION_CAPSULE_SCHEMA_ID),
            "rejection capsule must be tagged with its L4.E schema_id"
        );
        found = true;
    }
    assert!(found, "rejection capsule {} not found in CAS", cid_hex);

    // Negative invariant: no capsule with an "accepted" schema_id was written
    // for this rejection (the rejection lane is L4.E only).
    let all_cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    for cid in all_cids {
        if let Some(meta) = store.metadata(&cid) {
            let sid = meta.schema_id.as_deref().unwrap_or("");
            // No accepted-lane schema_ids should exist for this session.
            assert!(
                !sid.contains("accepted") && !sid.contains("l4-accepted"),
                "unexpected accepted-lane capsule found: schema_id={}",
                sid
            );
        }
    }
}
