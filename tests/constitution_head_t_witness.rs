//! Constitution Landing Gate — `HEAD_t` C1 6-field witness (G-009).
//!
//! Authority:
//!   - HARNESS.md §3 G-009 (architect ruling 2026-05-07)
//!   - CLAUDE.md §4.1 — Path-C hybrid; immediate C1 witness first, libgit2 later
//!
//! Closes G-009 strategic blocker substrate. The C1 witness is **derived**
//! over `QState` + caller-supplied L4.E head + CAS root + run_id.
//! Path-C2 production refs (`refs/chaintape/{l4, l4e, cas}`) is the forward
//! step — this test pins the immediate C1 witness shape so dashboards and
//! audit can rely on it from now.
//!
//! Tests (canonical 4 from HARNESS.md §3):
//!   - head_t_advances_on_l4
//!   - head_t_does_not_advance_on_l4e_only
//!   - head_t_reconstructs_from_replay
//!   - dashboard_reads_head_t_derived_state
//!
//! Plus structural gate:
//!   - head_t_witness_has_six_canonical_fields
//!
//! `FC-trace: FC1-INV1 + FC2-INV1 + Art-0.4 + G-009`.

use turingosv4::state::head_t_witness::HeadTWitness;
use turingosv4::state::q_state::{Hash, NodeId, QState};

fn fixture_q_with_head(head_oid: &str) -> QState {
    let mut q = QState::genesis();
    q.head_t = NodeId(head_oid.into());
    q.state_root_t = Hash([0x11; 32]);
    q
}

/// Architect-pinned 6-field shape (CLAUDE.md §4.1). Adding or removing a
/// field requires a constitution amendment.
#[test]
fn head_t_witness_has_six_canonical_fields() {
    let q = QState::genesis();
    let w = HeadTWitness::from_q_state(&q, "run-gate", None, None);
    let v = serde_json::to_value(&w).expect("serialize");
    let obj = v.as_object().expect("witness is a JSON object");
    assert_eq!(
        obj.len(),
        6,
        "HEAD_t witness must have exactly 6 canonical fields per architect §4.1; got {}",
        obj.len()
    );
    for f in [
        "state_root",
        "l4_head",
        "l4e_head",
        "cas_root",
        "economic_state_root",
        "run_id",
    ] {
        assert!(
            obj.contains_key(f),
            "HEAD_t witness missing canonical field `{f}` (architect §4.1 6-field shape)"
        );
    }
}

/// G-009 — every accepted L4 transition advances the witness `l4_head`. We
/// assert by comparing two QStates whose only difference is `head_t`.
#[test]
fn head_t_advances_on_l4() {
    let q1 = fixture_q_with_head(&"a".repeat(40));
    let q2 = fixture_q_with_head(&"b".repeat(40));
    let w1 = HeadTWitness::from_q_state(&q1, "run-gate", None, None);
    let w2 = HeadTWitness::from_q_state(&q2, "run-gate", None, None);
    assert_ne!(w1.l4_head, w2.l4_head, "l4_head must reflect QState.head_t");
    assert_ne!(
        w1.canonical_hash(),
        w2.canonical_hash(),
        "canonical hash must change on l4_head advance"
    );
}

/// G-009 — L4.E-only events MUST NOT advance `l4_head`. The `l4e_head`
/// field tracks them independently.
#[test]
fn head_t_does_not_advance_on_l4e_only() {
    let q = fixture_q_with_head(&"a".repeat(40));
    let pre = HeadTWitness::from_q_state(&q, "run-gate", None, None);
    let post = HeadTWitness::from_q_state(&q, "run-gate", Some(NodeId("ff".repeat(20))), None);
    assert_eq!(
        pre.l4_head, post.l4_head,
        "l4_head must remain stable when only L4.E advanced — Art. III L4/L4.E ledger separation"
    );
    assert_ne!(pre.l4e_head, post.l4e_head);
    assert_ne!(
        pre.canonical_hash(),
        post.canonical_hash(),
        "canonical hash must reflect l4e_head advance even when l4_head unchanged"
    );
}

/// G-009 / FC2 — replay equivalence. Same QState + run_id + l4e_head +
/// cas_root inputs MUST yield the same witness. This is the property that
/// makes the witness usable for `fc2_run_replayable_from_genesis_tape_cas`
/// post-fix verification.
#[test]
fn head_t_reconstructs_from_replay() {
    let q = fixture_q_with_head(&"a".repeat(40));
    let l4e = Some(NodeId("ff".repeat(20)));
    let cas = Some(Hash([0x22; 32]));
    let original = HeadTWitness::from_q_state(&q, "run-gate", l4e.clone(), cas);
    let replayed = HeadTWitness::from_q_state(&q, "run-gate", l4e, cas);
    assert_eq!(original, replayed);
    assert_eq!(original.canonical_hash(), replayed.canonical_hash());
}

/// FC1 / FC3 / Art-0.4 — dashboards read **derived** state. The witness is
/// computed FROM `QState`, never the other way around. This test asserts
/// the constructor is one-way: a witness alone cannot be used to mutate
/// `QState` (the `from_q_state` API takes `&QState`, not `&mut QState`).
///
/// Source-level structural assertion: the witness module exposes only
/// derived constructors, no mutators on `QState`.
#[test]
fn dashboard_reads_head_t_derived_state() {
    let src_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/state/head_t_witness.rs");
    let src = std::fs::read_to_string(&src_path).expect("read head_t_witness.rs");

    // Constructor must take `&QState`, not `&mut QState` — derived-view
    // discipline (no canonical mutation through the witness path).
    assert!(
        src.contains("from_q_state(\n        q: &QState,")
            || src.contains("from_q_state(q: &QState,"),
        "HeadTWitness::from_q_state must take &QState (read-only) so the witness \
         is a derived view, never a mutation surface (CLAUDE.md §5.4 — dashboard / report)"
    );
    assert!(
        !src.contains("&mut QState"),
        "HEAD_t witness module MUST NOT take &mut QState — it is a derived view, \
         not a sequencer admission surface. If you need to mutate, do it in \
         src/state/sequencer.rs and re-derive the witness."
    );
}
