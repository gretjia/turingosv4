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
//!
//! **TB-14 Atom 6 B′ step 4 (architect ruling 2026-05-03 §3+§4)**: this suite
//! was rewritten to consume `CanonicalNodeGraph` (canonical-keyed parent →
//! children edge map) in place of the legacy shadow `Tape`. The shadow Tape
//! lived in a different id namespace and produced empty mask_set in
//! production (Codex R1 ship audit VETO). All tests below build a
//! `BTreeMap<TxId, BTreeSet<TxId>>` directly with the same canonical IDs
//! used in the EconomicState's NodePositions — the post-B′-step-4 invariant
//! envelope.

use std::collections::{BTreeMap, BTreeSet};

use turingosv4::economy::money::MicroCoin;
use turingosv4::state::price_index::compute_mask_set;
use turingosv4::state::q_state::{AgentId, ChallengeCase, ChallengeStatus};
use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
use turingosv4::state::{
    compute_price_index, BoltzmannMaskPolicy, CanonicalNodeGraph, EconomicState,
    RationalPrice, TaskId, TxId,
};

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

/// Build a minimal `CanonicalNodeGraph` with one parent → one child edge,
/// keyed by canonical TxIds matching the NodePositions in the
/// accompanying `EconomicState`. This is the post-B′-step-4 (architect
/// ruling 2026-05-03 §3+§4) replacement for the legacy `Tape`-based
/// helper — the canonical id namespace is unified between the price
/// index, the edge map, and the challenge case targets.
fn baseline_canonical_graph() -> CanonicalNodeGraph {
    let mut edges: CanonicalNodeGraph = BTreeMap::new();
    let mut children = BTreeSet::new();
    children.insert(TxId("child_node".into()));
    edges.insert(TxId("parent_node".into()), children);
    edges
}

/// Build an EconomicState with the architect's standard parent + child
/// shape: parent has long+short positions; child has long+short positions.
/// Both keyed by canonical `node_id` matching `baseline_canonical_graph`.
fn baseline_econ_with_parent_child(
    parent_long: i64,
    parent_short: i64,
    child_long: i64,
    child_short: i64,
) -> (EconomicState, CanonicalNodeGraph) {
    let edges = baseline_canonical_graph();

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

    (econ, edges)
}

/// SG-14.3 — parent_id may appear in mask_set, but the canonical edge map
/// (and the price_index entry, and the canonical L4 chain — represented
/// here by the edge map keyed by accepted-WorkTx tx_ids) still yields it.
/// Mask is read-view, NOT deletion (architect ruling 2026-05-03 §3+§4
/// preserves CR-14.3 / SG-14.3 across the canonical-graph rewire).
#[test]
fn sg_14_3_parent_not_deleted_from_chaintape_after_masking() {
    // Parent has 50/50 long/short (price_yes = 0.5); child has 100/0 long/short
    // (price_yes = 1.0). Gap = 0.5; default policy margin = 0.10. Child masks parent.
    let (econ, edges) = baseline_econ_with_parent_child(500_000, 500_000, 2_000_000, 0);
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &edges, &policy, &price_index);

    assert!(
        mask.contains(&TxId("parent_node".into())),
        "SG-14.3 prerequisite: parent must be masked when child dominates"
    );

    // SG-14.3 post-B′-step-4: the canonical edge map still yields the
    // parent → child relation after masking. The mask is a separate
    // derived BTreeSet; it does NOT mutate `edges`. The L4 chain (which
    // `edges` is derived from at bus snapshot time) is therefore
    // canonically unchanged across mask computation.
    assert!(
        edges.contains_key(&TxId("parent_node".into())),
        "SG-14.3: canonical edges MUST still contain masked parent (read-view mask only, not deletion)"
    );
    assert!(
        edges.get(&TxId("parent_node".into()))
            .map(|s| s.contains(&TxId("child_node".into())))
            .unwrap_or(false),
        "SG-14.3: canonical parent → child edge MUST be preserved across mask computation"
    );
    // And the price_index entry is unchanged.
    assert!(
        price_index.contains_key(&TxId("parent_node".into())),
        "SG-14.3: price_index entry for masked parent MUST be preserved (mask is read-view, not deletion of derived state)"
    );
    assert!(
        price_index.contains_key(&TxId("child_node".into())),
        "SG-14.3: price_index entry for child MUST be preserved"
    );
}

/// SG-14.7 / CR-14.5 — open challenge against child blocks masking.
#[test]
fn sg_14_7_unresolved_challenge_blocks_masking() {
    let (mut econ, edges) =
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
    let mask = compute_mask_set(&econ, &edges, &policy, &price_index);

    assert!(
        !mask.contains(&TxId("parent_node".into())),
        "SG-14.7: open challenge against child MUST block parent masking, even though child price would otherwise dominate"
    );
}

/// SG-14.7 boundary — Released challenge does NOT block masking (only Open does).
#[test]
fn sg_14_7_released_challenge_does_not_block_masking() {
    let (mut econ, edges) =
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
    let mask = compute_mask_set(&econ, &edges, &policy, &price_index);

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
    let (econ, edges) = baseline_econ_with_parent_child(500_000, 500_000, 100, 0);
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &edges, &policy, &price_index);

    assert!(
        !mask.contains(&TxId("parent_node".into())),
        "SG-14.8: child below min_liquidity MUST NOT mask parent (low-liquidity manipulation guard)"
    );
}

/// Happy path: child clearly dominates parent → parent masked.
#[test]
fn child_dominates_parent_inserts_into_mask_set() {
    let (econ, edges) = baseline_econ_with_parent_child(500_000, 500_000, 2_000_000, 0);
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &edges, &policy, &price_index);

    assert_eq!(mask.len(), 1, "exactly one parent should be masked");
    assert!(mask.contains(&TxId("parent_node".into())));
}

/// Boundary: child price equal to parent price → does NOT mask (gap = 0 < margin).
#[test]
fn child_with_equal_price_does_not_mask() {
    let (econ, edges) = baseline_econ_with_parent_child(500_000, 500_000, 1_000_000, 1_000_000);
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &edges, &policy, &price_index);

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
    let (econ, edges) = baseline_econ_with_parent_child(500_000, 500_000, 1_100_000, 900_000);
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &edges, &policy, &price_index);

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
    let (econ, edges) = baseline_econ_with_parent_child(500_000, 500_000, 1_200_000, 800_000);
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &edges, &policy, &price_index);

    assert!(
        mask.contains(&TxId("parent_node".into())),
        "child gap (0.10) == margin threshold MUST mask (dominates_by uses >=)"
    );
}

/// Determinism: identical inputs yield identical mask_set output.
#[test]
fn compute_mask_set_is_replay_deterministic() {
    let (econ, edges) = baseline_econ_with_parent_child(500_000, 500_000, 2_000_000, 0);
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let first = compute_mask_set(&econ, &edges, &policy, &price_index);
    for _ in 0..10 {
        assert_eq!(
            compute_mask_set(&econ, &edges, &policy, &price_index),
            first,
            "compute_mask_set must be replay-deterministic (Art.0.2)"
        );
    }
}

/// Empty inputs: no nodes, empty mask.
#[test]
fn empty_inputs_yield_empty_mask() {
    let econ = EconomicState::default();
    let edges: CanonicalNodeGraph = BTreeMap::new();
    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &edges, &policy, &price_index);
    assert!(mask.is_empty());
}

/// Stricter margin: doubling the policy margin should leave previously-masking
/// child below the new threshold.
#[test]
fn stricter_margin_demasks_borderline_child() {
    // Parent 50/50, child 60/40 (gap = 0.10). Default margin = 0.10 → masks.
    // With margin = 0.20, no longer masks.
    let (econ, edges) = baseline_econ_with_parent_child(500_000, 500_000, 1_200_000, 800_000);
    let strict_policy = BoltzmannMaskPolicy {
        price_margin: RationalPrice {
            numerator: 1,
            denominator: 5,
        },
        ..BoltzmannMaskPolicy::default()
    };
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &edges, &strict_policy, &price_index);
    assert!(
        !mask.contains(&TxId("parent_node".into())),
        "strict margin (0.20) demasks child whose gap is exactly 0.10"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// TB-14 Atom 6 B′ step 4 — explicit canonical-namespace witness tests.
// These pin the architect ruling 2026-05-03 §3+§4 invariant that the
// CanonicalNodeGraph is the SOLE input to compute_mask_set's parent →
// children lookup; shadow IDs cannot leak in.
// ─────────────────────────────────────────────────────────────────────────

/// Architect §3 binding amend: PriceIndex + canonical-graph operate in the
/// SAME id namespace (canonical accepted WorkTx.tx_id). A canonical edge
/// pointing to a child whose TxId is NOT in price_index is silently
/// ignored — no mask flows from a phantom child.
#[test]
fn b_prime_step_4_phantom_canonical_child_does_not_mask() {
    // Build edges with a child whose TxId is NOT in the EconomicState
    // node_positions. price_index will not have an entry for this child,
    // so the dominance check has no input to evaluate → no mask.
    let (econ, _) = baseline_econ_with_parent_child(500_000, 500_000, 0, 0);
    let mut edges: CanonicalNodeGraph = BTreeMap::new();
    let mut children = BTreeSet::new();
    children.insert(TxId("phantom_child_not_in_price_index".into()));
    edges.insert(TxId("parent_node".into()), children);

    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &edges, &policy, &price_index);

    assert!(
        mask.is_empty(),
        "B′ step 4: a canonical edge pointing to a TxId not in price_index \
         MUST NOT produce a mask (architect §3 canonical-namespace invariant: \
         price_index + canonical-graph operate in the same id namespace)"
    );
}

/// Architect §3 binding amend: the SHADOW kernel.tape id namespace
/// (`tx_{count}_by_{author}`) is NOT consumed by compute_mask_set.
/// Building a CanonicalNodeGraph with shadow-style ids must yield empty
/// mask because none of those ids match canonical price_index keys
/// (the price_index is keyed by NodePosition.node_id which is the
/// canonical accepted WorkTx.tx_id).
#[test]
fn b_prime_step_4_shadow_style_ids_in_graph_yield_empty_mask() {
    let (econ, _) = baseline_econ_with_parent_child(500_000, 500_000, 2_000_000, 0);
    // Build a shadow-style graph (the legacy `tx_{count}_by_{author}`
    // namespace). This is what the pre-B′-step-4 bug was passing in.
    let mut edges: CanonicalNodeGraph = BTreeMap::new();
    let mut shadow_children = BTreeSet::new();
    shadow_children.insert(TxId("tx_1_by_A0".into())); // shadow id
    edges.insert(TxId("tx_0_by_A0".into()), shadow_children); // shadow id

    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &edges, &policy, &price_index);

    assert!(
        mask.is_empty(),
        "B′ step 4: shadow-style ids in the canonical-graph cannot mask \
         canonical price_index entries (architect §3 binding amend: shadow \
         tape ids are legacy/local only; compute_mask_set must operate in \
         the canonical namespace)"
    );
}
