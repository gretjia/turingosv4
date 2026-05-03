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
// Price signal MUST NOT override the predicate gate at sequencer.rs:516-558.
// ────────────────────────────────────────────────────────────────────
#[test]
fn price_does_not_affect_predicate_result() {
    // Filled in Atom 5 after boltzmann_select_parent_v2 + integer Boltzmann.
    // Stub: ensures test exists and will fail at compile-time if signature changes.
    unimplemented!(
        "Atom 5 — verify dispatch_transition rejects WorkTx with acceptance=false \
         regardless of NodeMarketEntry.price_yes from compute_price_index"
    )
}

// ────────────────────────────────────────────────────────────────────
// Halt-trigger #2
// price_does_not_change_l4_decision
//
// A tx that fails L4 (AcceptancePredicateFailed) must enter L4.E,
// not L4, even when the node has a high price_yes in compute_price_index.
// compute_price_index result is never read inside dispatch_transition.
// ────────────────────────────────────────────────────────────────────
#[test]
fn price_does_not_change_l4_decision() {
    // Filled in Atom 5.
    unimplemented!(
        "Atom 5 — verify L4.E classification for predicate-failed tx \
         is unchanged by presence of high price in compute_price_index"
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
// must contain zero occurrences of decimal-float-type tokens.
// ────────────────────────────────────────────────────────────────────
#[test]
fn no_f64_in_tb_14_modules() {
    // TB-14 Atom 2: enforce zero decimal-float-type tokens in TB-14 modules.
    // Plan v2 G1: this test reads `src/state/price_index.rs` at runtime via
    // `std::fs::read_to_string` (NEVER `include_str!`, which would inline
    // this very test's assertion strings — a self-reference trap that
    // sank the previous /opusplan attempt). Plan v2 G1 also requires
    // `src/state/price_index.rs` to contain zero substrings of the
    // forbidden types ANYWHERE — including comments — so the check is a
    // trivial substring search with no comment-stripping needed.
    //
    // The forbidden tokens are constructed at runtime from byte literals
    // joined into a String, so this test's source code does not contain
    // the literal substrings being scanned for.
    let forbidden: Vec<String> = vec![
        format!("{}{}", "f", "64"),
        format!("{}{}", "f", "32"),
    ];

    let manifest = env!("CARGO_MANIFEST_DIR");
    let price_index_path = format!("{}/src/state/price_index.rs", manifest);
    let body = std::fs::read_to_string(&price_index_path)
        .unwrap_or_else(|e| panic!("read {}: {}", price_index_path, e));
    for tok in &forbidden {
        assert!(
            !body.contains(tok.as_str()),
            "TB-14 halt-trigger #4 violated: src/state/price_index.rs contains forbidden \
             decimal-float-type token `{}` somewhere (Plan v2 G1 requires zero substring \
             occurrences anywhere in the file, including comments)",
            tok
        );
    }
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
    // TB-14 Atom 2: FR-14.3 — empty / zero-stake node yields None price.
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::state::{compute_price_index, EconomicState, TaskId, TxId};
    use turingosv4::state::q_state::AgentId;
    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};

    // Case A: completely empty state → empty index (no entries at all).
    let econ_a = EconomicState::default();
    let idx_a = compute_price_index(&econ_a);
    assert!(
        idx_a.is_empty(),
        "TB-14 halt-trigger #5: empty node_positions_t → empty PriceIndex"
    );

    // Case B: a node with one zero-amount Long position → entry exists,
    // price_yes = None AND price_no = None per FR-14.3.
    let mut econ_b = EconomicState::default();
    econ_b.node_positions_t.0.insert(
        TxId("zero_pos".into()),
        NodePosition {
            position_id: TxId("zero_pos".into()),
            node_id: TxId("zero_node".into()),
            task_id: TaskId("zero_task".into()),
            owner: AgentId("zero_agent".into()),
            side: PositionSide::Long,
            kind: PositionKind::FirstLong,
            amount: MicroCoin::zero(),
            source_tx: TxId("zero_pos".into()),
            opened_at_round: 1,
        },
    );
    let idx_b = compute_price_index(&econ_b);
    let entry = idx_b
        .get(&TxId("zero_node".into()))
        .expect("zero_node entry must be present in index");
    assert_eq!(
        entry.price_yes, None,
        "TB-14 halt-trigger #5: zero stake → price_yes MUST be None (FR-14.3)"
    );
    assert_eq!(
        entry.price_no, None,
        "TB-14 halt-trigger #5: zero stake → price_no MUST be None (FR-14.3)"
    );
    assert_eq!(entry.long_interest, MicroCoin::zero());
    assert_eq!(entry.short_interest, MicroCoin::zero());
    assert_eq!(entry.liquidity_depth, MicroCoin::zero());
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
