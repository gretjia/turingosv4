/// TB-14 Halt-Trigger Fixture (architect §5.7)
///
/// 6 tests that must ALL be green before TB-14 ships.
/// Tests are filled in progressively per atom:
///   Atom 2: #4 (no_f64) + #5 (zero_liquidity)
///   Atom 3: #3 (parent_not_deleted) + #6 (unresolved_challenge)
///   Atom 5: #1 (price_vs_predicate) + #2 (price_vs_l4)
///
/// Any atom that flips a green test to red = immediate halt (no round-2).
/// TRACE_MATRIX FC3-N42 + FC2-N28 + FC2-N29

// ────────────────────────────────────────────────────────────────────
// Halt-trigger #1
// price_does_not_affect_predicate_result
//
// A WorkTx with price_yes=Some(near-1) but acceptance.value=false
// must still return AcceptancePredicateFailed from dispatch_transition.
// Price signal MUST NOT override the predicate gate at sequencer.rs:522-527.
// ────────────────────────────────────────────────────────────────────
#[test]
fn price_does_not_affect_predicate_result() {
    // Filled in Atom 5 after boltzmann_select_parent_v2 + integer Boltzmann.
    // Stub: ensures test exists and will fail at compile-time if signature changes.
    unimplemented!(
        "Atom 5 — verify dispatch_transition rejects WorkTx with acceptance=false \
         regardless of node price_yes in PriceIndex"
    )
}

// ────────────────────────────────────────────────────────────────────
// Halt-trigger #2
// price_does_not_change_l4_decision
//
// A tx that fails L4 (AcceptancePredicateFailed) must enter L4.E,
// not L4, even when the node has a high price_yes in PriceIndex.
// price_index_t is never read inside dispatch_transition.
// ────────────────────────────────────────────────────────────────────
#[test]
fn price_does_not_change_l4_decision() {
    // Filled in Atom 5.
    unimplemented!(
        "Atom 5 — verify L4.E classification for predicate-failed tx \
         is unchanged by presence of high price in PriceIndex"
    )
}

// ────────────────────────────────────────────────────────────────────
// Halt-trigger #3
// parent_not_deleted_from_chaintape
//
// After compute_mask_set includes a parent_id, the full Tape iteration
// (tape.nodes()) must still yield that parent node.
// mask_set filters the SCHEDULER read-view, not ChainTape storage.
// ────────────────────────────────────────────────────────────────────
#[test]
fn parent_not_deleted_from_chaintape() {
    // Filled in Atom 3 after compute_mask_set + AgentVisibleProjection.mask_set.
    unimplemented!(
        "Atom 3 — tape.nodes().contains_key(parent_id) must be true \
         even when parent_id is in compute_mask_set result"
    )
}

// ────────────────────────────────────────────────────────────────────
// Halt-trigger #4
// no_f64_in_tb_14_modules
//
// src/state/price_index.rs and the TB-14 spans of src/sdk/actor.rs
// must contain zero occurrences of f64 or f32.
// ────────────────────────────────────────────────────────────────────
#[test]
fn no_f64_in_tb_14_modules() {
    // Filled in Atom 2 when src/state/price_index.rs is created.
    unimplemented!(
        "Atom 2 — grep src/state/price_index.rs for f64/f32 tokens; \
         assert count == 0"
    )
}

// ────────────────────────────────────────────────────────────────────
// Halt-trigger #5
// zero_liquidity_returns_none
//
// compute_price_index over an EconomicState where a node_id has
// zero long AND zero short interest must return an entry where
// price_yes == None AND price_no == None (FR-14.3).
// Non-None price for zero-liquidity = forbidden.
// ────────────────────────────────────────────────────────────────────
#[test]
fn zero_liquidity_returns_none() {
    // Filled in Atom 2 after compute_price_index exists.
    unimplemented!(
        "Atom 2 — compute_price_index with empty node_positions_t or \
         zero-stake NodePositions must yield price_yes=None, price_no=None"
    )
}

// ────────────────────────────────────────────────────────────────────
// Halt-trigger #6
// unresolved_challenge_blocks_masking
//
// If a child node has a ChallengeCase with status=Open targeting it,
// compute_mask_set must NOT include the parent in the mask_set
// even if child.price_yes dominates parent.price_yes by price_margin.
// (CR-14.5 + SG-14.7)
// ────────────────────────────────────────────────────────────────────
#[test]
fn unresolved_challenge_blocks_masking() {
    // Filled in Atom 3 after compute_mask_set exists.
    unimplemented!(
        "Atom 3 — child with ChallengeCase status=Open: \
         parent must NOT appear in compute_mask_set result"
    )
}
