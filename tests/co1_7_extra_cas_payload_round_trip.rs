//! CO1.7-extra D4 (substrate-independent): CAS payload round-trip + CID
//! stability across cold restart.
//!
//! Verifies that CO1.4-extra sidecar persistence makes CasStore content
//! reachable across cold-start, which is a precondition for CO1.7.5
//! FullTransition replay (deferred; gated on CO P2.x substrate).
//!
//! Substrate-independent: uses only CasStore + ObjectType (CO1.4 +
//! CO1.4-extra shipped surfaces); does NOT depend on CO P2.x.

use turingosv4::bottom_white::cas::{CasStore, ObjectType};

#[test]
fn cas_payload_round_trip_with_cid_stability_across_restart() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let payload: &[u8] = b"co1.7-extra-deterministic-payload-v1";

    // Phase 1: open fresh CasStore, put payload, capture CID.
    let cid_first = {
        let mut cas = CasStore::open(tmp.path()).expect("first open");
        cas.put(
            payload,
            ObjectType::ProposalPayload,
            "test-epoch",
            1,
            Some("CO1.7-extra".into()),
        )
        .expect("put")
    };
    // CasStore handle dropped here → simulates cold restart.

    // Phase 2: reopen CasStore (cold-start path via CO1.4-extra sidecar
    // index persistence). get must return the same bytes that were put.
    let bytes = {
        let cas = CasStore::open(tmp.path()).expect("reopen post-restart");
        cas.get(&cid_first).expect("get post-restart")
    };

    assert_eq!(
        bytes.as_slice(),
        payload,
        "CO1.4-extra sidecar persistence must round-trip CAS payload across cold restart"
    );
}
