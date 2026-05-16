//! REAL-BCAST-1 — no raw leakage gates.

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::runtime::librarian_broadcast::{
    assert_no_forbidden_broadcast_material, build_librarian_digest, project_role_notifications,
    render_librarian_notices_section, LibrarianEvidenceEvent, LibrarianEvidenceKind,
    LibrarianSourceScope,
};
use turingosv4::runtime::real5_roles::AgentRole;

#[test]
fn digest_crop_and_prompt_reject_forbidden_raw_material() {
    for raw in [
        "raw Lean stderr",
        "raw prompt",
        "raw completion",
        "private CoT",
        "raw diagnostics",
        "untriaged historical logs",
    ] {
        assert!(assert_no_forbidden_broadcast_material(raw)
            .unwrap_err()
            .contains("forbidden broadcast material"));
    }
}

#[test]
fn sanitized_digest_and_prompt_pass_no_leak_scan() {
    let scope = LibrarianSourceScope {
        current_run_cas_root: Cid::from_content(b"root"),
        prior_capsule_cids: vec![],
        max_prior_batches: 0,
        task_tags: vec!["algebra".into()],
    };
    let digest = build_librarian_digest(
        scope,
        10,
        vec![
            LibrarianEvidenceEvent {
                cid: Cid::from_content(b"e1"),
                kind: LibrarianEvidenceKind::LeanError,
                class_label: "err:type_mismatch".into(),
                task_id: Some("task".into()),
                public_summary: "Repeated type mismatch class".into(),
                head_t: 9,
            },
            LibrarianEvidenceEvent {
                cid: Cid::from_content(b"e2"),
                kind: LibrarianEvidenceKind::LeanError,
                class_label: "err:type_mismatch".into(),
                task_id: Some("task".into()),
                public_summary: "Repeated type mismatch class".into(),
                head_t: 10,
            },
        ],
    )
    .unwrap();
    let crop = project_role_notifications(&digest, AgentRole::Solver, 4).unwrap();
    let digest_json = serde_json::to_string(&digest).unwrap();
    let crop_json = serde_json::to_string(&crop).unwrap();
    assert_no_forbidden_broadcast_material(&digest_json).unwrap();
    assert_no_forbidden_broadcast_material(&crop_json).unwrap();
    let prompt =
        render_librarian_notices_section("cid:digest", &[crop.rendered_notice], 4).unwrap();
    assert_no_forbidden_broadcast_material(&prompt).unwrap();
}
