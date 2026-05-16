//! REAL-BCAST-1 — role-cropped Librarian notices.

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::runtime::librarian_broadcast::{
    build_librarian_digest, project_role_notifications, LibrarianEvidenceEvent,
    LibrarianEvidenceKind, LibrarianSourceScope,
};
use turingosv4::runtime::real5_roles::AgentRole;

fn digest() -> turingosv4::runtime::librarian_broadcast::LibrarianDigest {
    let scope = LibrarianSourceScope {
        current_run_cas_root: Cid::from_content(b"root"),
        prior_capsule_cids: vec![],
        max_prior_batches: 0,
        task_tags: vec!["algebra".into()],
    };
    build_librarian_digest(
        scope,
        10,
        vec![
            LibrarianEvidenceEvent {
                cid: Cid::from_content(b"e1"),
                kind: LibrarianEvidenceKind::LeanError,
                class_label: "err:type_mismatch".into(),
                task_id: Some("task".into()),
                public_summary: "Repeated Lean type mismatch".into(),
                head_t: 8,
            },
            LibrarianEvidenceEvent {
                cid: Cid::from_content(b"e2"),
                kind: LibrarianEvidenceKind::LeanError,
                class_label: "err:type_mismatch".into(),
                task_id: Some("task".into()),
                public_summary: "Repeated Lean type mismatch".into(),
                head_t: 9,
            },
            LibrarianEvidenceEvent {
                cid: Cid::from_content(b"ev"),
                kind: LibrarianEvidenceKind::EVReason,
                class_label: "ev:NegativeEV".into(),
                task_id: Some("task".into()),
                public_summary: "Trader abstained because EV was negative".into(),
                head_t: 9,
            },
        ],
    )
    .unwrap()
}

#[test]
fn projector_produces_role_specific_crops() {
    let digest = digest();
    let solver = project_role_notifications(&digest, AgentRole::Solver, 8).unwrap();
    let trader = project_role_notifications(&digest, AgentRole::BullTrader, 8).unwrap();
    let verifier = project_role_notifications(&digest, AgentRole::Verifier, 8).unwrap();

    assert!(solver.rendered_notice.contains("type_mismatch"));
    assert!(!solver.rendered_notice.contains("price anomaly"));
    assert!(trader.rendered_notice.contains("NegativeEV"));
    assert!(verifier.rendered_notice.contains("proof-risk"));
    assert_ne!(solver.rendered_notice, trader.rendered_notice);
}

#[test]
fn role_crops_reference_source_digest_and_redactions() {
    let digest = digest();
    let crop = project_role_notifications(&digest, AgentRole::Challenger, 4).unwrap();
    assert_eq!(crop.source_digest_cid, digest.digest_id);
    assert!(crop
        .redacted_fields
        .iter()
        .any(|v| v.ends_with("_redacted")));
    assert!(!crop.redacted_fields.iter().any(|v| v.contains("raw")));
    assert!(crop.rendered_notice.contains("raw logs redacted"));
}
