//! TB-14 Atom 3 — SG-14.3 + SG-14.7 + SG-14.8 explicit witness suite for
//! `compute_mask_set`.
//!
//! TRACE_MATRIX TB-14 SG-14.3 / SG-14.7 / SG-14.8 (charter §6 ship-gates table).
//! These three ship gates are the named integration-test targets per
//! `handover/tracer_bullets/TB-14_charter_2026-05-03.md` §6:
//!
//!   SG-14.3  Parent not deleted from ChainTape after masking.
//!   SG-14.7  Unresolved challenge blocks masking.
//!   SG-14.8  Low-liquidity manipulation cannot mask parent.
//!
//! Plus: CR-14.4 (low-liquidity boundary) + CR-14.5 (open-challenge boundary)
//! explicit witnesses + happy-path "child dominates parent" mask insertion.

use turingosv4::economy::money::MicroCoin;
use turingosv4::ledger::{Node, Tape};
use turingosv4::state::q_state::{AgentId, ChallengeCase, ChallengeStatus};
use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
use turingosv4::state::{
    compute_price_index, BoltzmannMaskPolicy, EconomicState, RationalPrice, TaskId, TxId,
};
use turingosv4::state::price_index::compute_mask_set;

fn make_position(
    position_id: &str,
    node_id: &str,
    task_id: &str,
    owner: &str,
    side: PositionSide,
    kind: PositionKind,
    amount_micro: i64,
) -> NodePosition {
    NodePosition {
        position_id: TxId(position_id.into()),
        node_id: TxId(node_id.into()),
        task_id: TaskId(task_id.into()),
        owner: AgentId(owner.into()),
        side,
        kind,
        amount: MicroCoin::from_micro_units(amount_micro),
        source_tx: TxId(position_id.into()),
        opened_at_round: 1,
    }
}

fn make_node(id: &str, citations: &[&str]) -> Node {
    Node {
        id: id.to_string(),
        author: "test_author".to_string(),
        payload: format!("payload_for_{id}"),
        citations: citations.iter().map(|s| s.to_string()).collect(),
        created_at: 0,
        completion_tokens: 0,
    }
}

/// Build a minimal Tape + EconomicState + ChallengeCases triple for mask
/// testing. Parent node has Long-only positions (price_yes near 1); child
/// node has Long-only positions (price_yes near 1) by default — tests that
/// want a dominance gap or a domination block adjust per case.
fn baseline_econ_with_parent_child(
    parent_long: i64,
    parent_short: i64,
    child_long: i64,
    child_short: i64,
) -> (EconomicState, Tape) {
    let mut tape = Tape::new();
    tape.append(make_node("parent_node", &[])).expect("append parent");
    tape.append(make_node("child_node", &["parent_node"]))
        .expect("append child");

    let mut econ = EconomicState::default();
    if parent_long > 0 {
        let p = make_position(
            "parent_long_pos",
            "parent_node",
            "task_p",
            "agent_pl",
            PositionSide::Long,
            PositionKind::FirstLong,
            parent_long,
        );
        econ.node_positions_t.0.insert(p.position_id.clone(), p);
    }
    if parent_short > 0 {
        let p = make_position(
            "parent_short_pos",
            "parent_node",
            "task_p",
            "agent_ps",
            PositionSide::Short,
            PositionKind::ChallengeShort,
            parent_short,
        );
        econ.node_positions_t.0.insert(p.position_id.clone(), p);
    }
    if child_long > 0 {
        let c = make_position(
            "child_long_pos",
            "child_node",
            "task_c",
            "agent_cl",
            PositionSide::Long,
            PositionKind::FirstLong,
            child_long,
        );
        econ.node_positions_t.0.insert(c.position_id.clone(), c);
    }
    if child_short > 0 {
        let c = make_position(
            "child_short_pos",
            "child_node",
            "task_c",
            "agent_cs",
            PositionSide::Short,
            PositionKind::ChallengeShort,
            child_short,
        );
        econ.node_positions_t.0.insert(c.position_id.clone(), c);
    }

    (econ, tape)
}

/// SG-14.3 — parent_id may appear in mask_set, but tape.nodes() still yields it.
#[test]
fn sg_14_3_parent_not_deleted_from_chaintape_after_masking() {
    // Parent has 50/50 long/short (price_yes = 0.5); child has 100/0 long/short
    // (price_yes = 1.0). Gap = 0.5; default policy margin = 0.10. Child masks parent.
    let (econ, tape) = baseline_econ_with_parent_child(500_000, 500_000, 2_000_000, 0);
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);

    assert!(
        mask.contains(&TxId("parent_node".into())),
        "SG-14.3 prerequisite: parent must be masked when child dominates"
    );

    // SG-14.3: tape.nodes() still yields the masked parent.
    assert!(
        tape.nodes().contains_key("parent_node"),
        "SG-14.3: tape.nodes() MUST still contain masked parent (read-view mask only, not deletion)"
    );
    assert!(
        tape.nodes().contains_key("child_node"),
        "SG-14.3: tape.nodes() MUST still contain child"
    );
    // Tape children edge from parent → child preserved.
    assert!(
        tape.children("parent_node").contains(&"child_node".to_string()),
        "SG-14.3: tape.children() relationship MUST be preserved"
    );
}

/// SG-14.7 / CR-14.5 — open challenge against child blocks masking.
#[test]
fn sg_14_7_unresolved_challenge_blocks_masking() {
    let (mut econ, tape) =
        baseline_econ_with_parent_child(500_000, 500_000, 2_000_000, 0);
    // Add a ChallengeCase against the child with status = Open.
    econ.challenge_cases_t.0.insert(
        TxId("ch_against_child".into()),
        ChallengeCase {
            challenger: AgentId("challenger".into()),
            bond: MicroCoin::from_micro_units(1_000),
            opened_at_round: 1,
            target_work_tx: TxId("child_node".into()),
            status: ChallengeStatus::Open,
        },
    );

    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);

    assert!(
        !mask.contains(&TxId("parent_node".into())),
        "SG-14.7: open challenge against child MUST block parent masking, even though child price would otherwise dominate"
    );
}

/// SG-14.7 boundary — Released challenge does NOT block masking (only Open does).
#[test]
fn sg_14_7_released_challenge_does_not_block_masking() {
    let (mut econ, tape) =
        baseline_econ_with_parent_child(500_000, 500_000, 2_000_000, 0);
    econ.challenge_cases_t.0.insert(
        TxId("ch_resolved".into()),
        ChallengeCase {
            challenger: AgentId("challenger".into()),
            bond: MicroCoin::from_micro_units(1_000),
            opened_at_round: 1,
            target_work_tx: TxId("child_node".into()),
            status: ChallengeStatus::Released,
        },
    );

    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);

    assert!(
        mask.contains(&TxId("parent_node".into())),
        "SG-14.7 boundary: Released challenge does NOT block masking"
    );
}

/// SG-14.8 / CR-14.4 — child below `min_liquidity` cannot mask parent.
#[test]
fn sg_14_8_low_liquidity_child_cannot_mask_parent() {
    // Parent 50/50, child has only 100 micro-units of liquidity (well below
    // the 1_000_000 micro min_liquidity default).
    let (econ, tape) = baseline_econ_with_parent_child(500_000, 500_000, 100, 0);
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);

    assert!(
        !mask.contains(&TxId("parent_node".into())),
        "SG-14.8: child below min_liquidity MUST NOT mask parent (low-liquidity manipulation guard)"
    );
}

/// Happy path: child clearly dominates parent → parent masked.
#[test]
fn child_dominates_parent_inserts_into_mask_set() {
    let (econ, tape) = baseline_econ_with_parent_child(500_000, 500_000, 2_000_000, 0);
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);

    assert_eq!(mask.len(), 1, "exactly one parent should be masked");
    assert!(mask.contains(&TxId("parent_node".into())));
}

/// Boundary: child price equal to parent price → does NOT mask (gap = 0 < margin).
#[test]
fn child_with_equal_price_does_not_mask() {
    let (econ, tape) = baseline_econ_with_parent_child(500_000, 500_000, 1_000_000, 1_000_000);
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);

    assert!(
        !mask.contains(&TxId("parent_node".into())),
        "child price = parent price (gap = 0) MUST NOT mask"
    );
}

/// Boundary: child gap below margin → does NOT mask.
/// Parent 50/50 (price_yes = 0.5); child 55/45 (price_yes = 0.55). Gap = 0.05.
/// Default margin = 0.10. 0.05 < 0.10 → no mask.
#[test]
fn child_with_gap_below_margin_does_not_mask() {
    let (econ, tape) = baseline_econ_with_parent_child(500_000, 500_000, 1_100_000, 900_000);
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);

    assert!(
        !mask.contains(&TxId("parent_node".into())),
        "child gap (0.05) below margin (0.10) MUST NOT mask"
    );
}

/// Boundary: child price exactly at the margin threshold → masks (>=).
/// Parent 50/50; child 60/40 (price_yes = 0.6). Gap = 0.10 = margin exactly.
/// dominates_by uses >= so this masks.
#[test]
fn child_at_margin_threshold_masks() {
    let (econ, tape) = baseline_econ_with_parent_child(500_000, 500_000, 1_200_000, 800_000);
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);

    assert!(
        mask.contains(&TxId("parent_node".into())),
        "child gap (0.10) == margin threshold MUST mask (dominates_by uses >=)"
    );
}

/// Determinism: identical inputs yield identical mask_set output.
#[test]
fn compute_mask_set_is_replay_deterministic() {
    let (econ, tape) = baseline_econ_with_parent_child(500_000, 500_000, 2_000_000, 0);
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let first = compute_mask_set(&econ, &tape, &policy, &price_index);
    for _ in 0..10 {
        assert_eq!(
            compute_mask_set(&econ, &tape, &policy, &price_index),
            first,
            "compute_mask_set must be replay-deterministic (Art.0.2)"
        );
    }
}

/// Empty inputs: no nodes, empty mask.
#[test]
fn empty_inputs_yield_empty_mask() {
    let econ = EconomicState::default();
    let tape = Tape::new();
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);
    assert!(mask.is_empty());
}

/// Stricter margin: doubling the policy margin should leave previously-masking
/// child below the new threshold.
#[test]
fn stricter_margin_demasks_borderline_child() {
    // Parent 50/50, child 60/40 (gap = 0.10). Default margin = 0.10 → masks.
    // With margin = 0.20, no longer masks.
    let (econ, tape) = baseline_econ_with_parent_child(500_000, 500_000, 1_200_000, 800_000);
    let strict_policy = BoltzmannMaskPolicy {
        price_margin: RationalPrice {
            numerator: 1,
            denominator: 5,
        },
        ..BoltzmannMaskPolicy::default()
    };
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &tape, &strict_policy, &price_index);
    assert!(
        !mask.contains(&TxId("parent_node".into())),
        "strict margin (0.20) demasks child whose gap is exactly 0.10"
    );
}
