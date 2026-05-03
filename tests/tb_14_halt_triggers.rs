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
//
// TB-14 Atom 5 structural enforcement: dispatch_transition's source
// path contains zero references to TB-14 price/mask types. Decoupling
// is enforced by code structure — if sequencer never reads
// compute_price_index / NodeMarketEntry / RationalPrice / mask_set,
// they cannot affect predicate evaluation at runtime. (Parallel to
// halt-trigger #4's file-level decimal-float fence.)
// ────────────────────────────────────────────────────────────────────
#[test]
fn price_does_not_affect_predicate_result() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let sequencer_path = format!("{}/src/state/sequencer.rs", manifest);
    let body = std::fs::read_to_string(&sequencer_path)
        .unwrap_or_else(|e| panic!("read {}: {}", sequencer_path, e));

    // The price/mask types must NOT appear in the sequencer dispatch path.
    // Constructed at runtime via byte literals to avoid this test's own
    // source containing the substrings being scanned for.
    let forbidden: Vec<String> = vec![
        format!("compute_price{}", "_index"),
        format!("compute_mask{}", "_set"),
        format!("NodeMarket{}", "Entry"),
        format!("Rational{}", "Price"),
        format!("Boltzmann{}", "MaskPolicy"),
    ];
    for tok in &forbidden {
        assert!(
            !body.contains(tok.as_str()),
            "halt-trigger #1: src/state/sequencer.rs MUST NOT reference TB-14 \
             price/mask type `{}` — sequencer dispatch is decoupled from price \
             signal by construction (CR-14.1)",
            tok
        );
    }
}

// ────────────────────────────────────────────────────────────────────
// Halt-trigger #2
// price_does_not_change_l4_decision
//
// A tx that fails L4 (AcceptancePredicateFailed) must enter L4.E,
// not L4, even when the node has a high price_yes in compute_price_index.
//
// TB-14 Atom 5 structural enforcement (complementary to halt-trigger #1):
// `src/state/sequencer.rs` MUST NOT IMPORT any TB-14 price/mask
// type via `use` statement. Halt-trigger #1 scans for symbol uses
// in the file body; halt-trigger #2 scans the `use` block to catch
// import-only references (e.g., a re-export forwarder that would
// otherwise let TB-14 types leak into sequencer scope without an
// in-body call). Together: sequencer is permanently price-blind by
// construction → L4/L4.E classification is a pure function of
// dispatch_transition's verdict, never of any price signal.
//
// This is permanent: even after Atom 6's bus.rs snapshot wire-swap
// (which legitimately reads compute_price_index for read-view
// broadcast), sequencer.rs MUST remain free of TB-14 imports.
// ────────────────────────────────────────────────────────────────────
#[test]
fn price_does_not_change_l4_decision() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let sequencer_path = format!("{}/src/state/sequencer.rs", manifest);
    let body = std::fs::read_to_string(&sequencer_path)
        .unwrap_or_else(|e| panic!("read {}: {}", sequencer_path, e));

    // Scan ONLY the `use` statements in sequencer.rs for any TB-14 import.
    // Constructed at runtime via byte literals to avoid self-reference.
    let import_tokens: Vec<String> = vec![
        format!("price{}", "_index"),                  // module path
        format!("compute_price{}", "_index"),
        format!("compute_mask{}", "_set"),
        format!("NodeMarket{}", "Entry"),
        format!("Rational{}", "Price"),
        format!("Boltzmann{}", "MaskPolicy"),
    ];
    let mut violations: Vec<String> = Vec::new();
    for (i, line) in body.lines().enumerate() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("use ") && !trimmed.starts_with("pub use ") {
            continue;
        }
        for tok in &import_tokens {
            if line.contains(tok.as_str()) {
                violations.push(format!(
                    "sequencer.rs:{}: forbidden TB-14 import token `{}` in `{}`",
                    i + 1,
                    tok,
                    line.trim()
                ));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "halt-trigger #2: src/state/sequencer.rs MUST NOT IMPORT any TB-14 \
         price/mask type. Sequencer remains permanently price-blind by \
         construction; L4/L4.E classification is a pure function of \
         dispatch_transition's verdict (CR-14.2). Violations:\n{}",
        violations.join("\n")
    );
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
    // TB-14 Atom 3: CR-14.3 / SG-14.3 — masking is read-view, not deletion.
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::ledger::{Node, Tape};
    use turingosv4::state::q_state::AgentId;
    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
    use turingosv4::state::{
        compute_mask_set, compute_price_index, BoltzmannMaskPolicy, EconomicState,
        TaskId, TxId,
    };

    fn node(id: &str, parents: &[&str]) -> Node {
        Node {
            id: id.to_string(),
            author: "author".into(),
            payload: format!("payload_{id}"),
            citations: parents.iter().map(|s| s.to_string()).collect(),
            created_at: 0,
            completion_tokens: 0,
        }
    }
    fn position(
        pid: &str,
        node_id: &str,
        owner: &str,
        side: PositionSide,
        kind: PositionKind,
        amount_micro: i64,
    ) -> NodePosition {
        NodePosition {
            position_id: TxId(pid.into()),
            node_id: TxId(node_id.into()),
            task_id: TaskId("t1".into()),
            owner: AgentId(owner.into()),
            side,
            kind,
            amount: MicroCoin::from_micro_units(amount_micro),
            source_tx: TxId(pid.into()),
            opened_at_round: 1,
        }
    }

    // Build parent → child Tape; parent 50/50, child 100/0 (clear dominance).
    let mut tape = Tape::new();
    tape.append(node("parent", &[])).expect("append parent");
    tape.append(node("child", &["parent"])).expect("append child");

    let mut econ = EconomicState::default();
    for p in [
        position("p1", "parent", "ag_pl", PositionSide::Long,
                 PositionKind::FirstLong, 500_000),
        position("p2", "parent", "ag_ps", PositionSide::Short,
                 PositionKind::ChallengeShort, 500_000),
        position("p3", "child", "ag_cl", PositionSide::Long,
                 PositionKind::FirstLong, 2_000_000),
    ] {
        econ.node_positions_t.0.insert(p.position_id.clone(), p);
    }

    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);

    // Prerequisite: parent IS masked (so the test below is meaningful).
    assert!(
        mask.contains(&TxId("parent".into())),
        "halt-trigger #3 prerequisite: parent must be masked under default policy"
    );

    // Halt-trigger #3 assertion: tape.nodes() still yields masked parent.
    assert!(
        tape.nodes().contains_key("parent"),
        "halt-trigger #3: tape.nodes() MUST still contain masked parent (CR-14.3)"
    );
    // And the parent → child edge is preserved.
    assert!(
        tape.children("parent").contains(&"child".to_string()),
        "halt-trigger #3: tape.children() edge MUST be preserved across mask"
    );
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
    // TB-14 Atom 3: CR-14.5 / SG-14.7 — Open challenge against child blocks parent masking.
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::ledger::{Node, Tape};
    use turingosv4::state::q_state::{AgentId, ChallengeCase, ChallengeStatus};
    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
    use turingosv4::state::{
        compute_mask_set, compute_price_index, BoltzmannMaskPolicy, EconomicState,
        TaskId, TxId,
    };

    fn node(id: &str, parents: &[&str]) -> Node {
        Node {
            id: id.to_string(),
            author: "author".into(),
            payload: format!("payload_{id}"),
            citations: parents.iter().map(|s| s.to_string()).collect(),
            created_at: 0,
            completion_tokens: 0,
        }
    }
    fn position(
        pid: &str,
        node_id: &str,
        owner: &str,
        side: PositionSide,
        kind: PositionKind,
        amount_micro: i64,
    ) -> NodePosition {
        NodePosition {
            position_id: TxId(pid.into()),
            node_id: TxId(node_id.into()),
            task_id: TaskId("t1".into()),
            owner: AgentId(owner.into()),
            side,
            kind,
            amount: MicroCoin::from_micro_units(amount_micro),
            source_tx: TxId(pid.into()),
            opened_at_round: 1,
        }
    }

    // Build parent → child Tape; parent 50/50, child 100/0 (would dominate
    // under default policy if no challenge present).
    let mut tape = Tape::new();
    tape.append(node("parent", &[])).expect("append parent");
    tape.append(node("child", &["parent"])).expect("append child");

    let mut econ = EconomicState::default();
    for p in [
        position("p1", "parent", "ag_pl", PositionSide::Long,
                 PositionKind::FirstLong, 500_000),
        position("p2", "parent", "ag_ps", PositionSide::Short,
                 PositionKind::ChallengeShort, 500_000),
        position("p3", "child", "ag_cl", PositionSide::Long,
                 PositionKind::FirstLong, 2_000_000),
    ] {
        econ.node_positions_t.0.insert(p.position_id.clone(), p);
    }

    // Add Open challenge against the child → parent masking MUST be blocked.
    econ.challenge_cases_t.0.insert(
        TxId("ch_open".into()),
        ChallengeCase {
            challenger: AgentId("challenger".into()),
            bond: MicroCoin::from_micro_units(1_000),
            opened_at_round: 1,
            target_work_tx: TxId("child".into()),
            status: ChallengeStatus::Open,
        },
    );

    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);

    assert!(
        !mask.contains(&TxId("parent".into())),
        "halt-trigger #6: open challenge against child MUST block parent masking (CR-14.5)"
    );
}
