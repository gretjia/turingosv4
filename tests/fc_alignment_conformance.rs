// FC alignment conformance battery — DO-178C requirements traceability witness.
//
// Each test asserts that a constitutional FC element (FC1 / FC2 / FC3) is
// present in the codebase as a reachable symbol. This is the mechanical
// audit-trail artifact CLAUDE.md "Alignment Standard" demands:
//
//     "Conformance tests: tests/fc_alignment_conformance.rs — 每个 ✅ 行 ≥1
//      witness test"
//
// Ignored tests in this file are explicit red placeholders for missing or
// deferred runtime paths. They are not counted as green flowchart coverage.
//
// FC-trace: FC1 (basic cycle) + FC2 (init/halt/tick) + FC3 (system topology).
// Mapping authority: constitution.md FC blocks + pinned canonical hashes.
//
// Witness semantics: each test imports the FC-anchored symbol and references
// it. If the symbol is renamed, removed, or its public API breaks, this test
// fails to compile or panics — surfacing constitutional drift at `cargo test`
// time rather than at the next external audit.

#![allow(dead_code)]

use turingosv4::boot::{parse_trust_root_section, verify_trust_root, TrustRootError};
use turingosv4::bottom_white::ledger::transition_ledger::append as append_l4_root;
use turingosv4::bus::{BusConfig, BusResult, TuringBus};
use turingosv4::drivers::llm_http::ResilientLLMClient;
use turingosv4::kernel::Kernel;
use turingosv4::ledger::{EventType, Ledger, LedgerEvent, Tape};
use turingosv4::sdk::protocol::{parse_agent_output, AgentAction};
use turingosv4::sdk::snapshot::UniverseSnapshot;
use turingosv4::state::q_state::Hash;

// ─── FC1: basic cycle Q_t → rtool → input → AI(δ) → output → ∏p → wtool → Q_{t+1} ───

#[test]
fn fc1_n1_q_state_carrier_constructible_with_default_config() {
    // FC1-N1 Q_t = ⟨q_t, HEAD_t, tape_t⟩ — TuringBus is the constitutional
    // Q_t carrier. A0e-fix 2026-04-25: strengthened from type_name witness
    // (audit Q2 found weak compile-only witness didn't catch
    // behavioral regression). Now actually constructs the carrier.
    let kernel = Kernel::new();
    let bus = TuringBus::new(kernel, BusConfig::default());
    // Witness: behavioral — bus.kernel.tape exists + has empty time-arrow
    // on fresh construction (i.e., FC1-N3 HEAD = None).
    assert!(
        bus.kernel.tape.time_arrow().is_empty(),
        "FC1-N1: fresh bus must have empty time-arrow"
    );
}

#[test]
fn fc1_n4_tape_constructible_with_time_arrow() {
    // FC1-N4 tape_t — the constitutional tape exists, is constructible,
    // exposes a time-arrow accessor (the canonical FC1-N3 HEAD idiom is
    // tape.time_arrow().last()).
    let tape = Tape::new();
    assert!(
        tape.time_arrow().is_empty(),
        "fresh tape has empty time-arrow"
    );
}

#[test]
fn fc1_n7_delta_ai_client_constructible() {
    // FC1-N7 δ / AI = ResilientLLMClient. A0e-fix 2026-04-25: strengthened
    // from type_name to actual construction. Witness: ResilientLLMClient::new
    // exists + accepts (proxy_url, timeout, max_retries).
    let _client = ResilientLLMClient::new("http://localhost:8080", 30, 3);
}

#[test]
fn fc1_n6_input_universe_snapshot_via_bus() {
    // FC1-N6 input = ⟨q_i, s_i⟩ realized as UniverseSnapshot.
    // TB-14 Atom 6 (2026-05-03): post-CPMM-excision, the snapshot's signal
    // surface is `price_index` + `mask_set` — derived integer-rational
    // views over canonical EconomicState. Witness:
    // bus.snapshot() returns a UniverseSnapshot whose new fields are
    // structurally present and empty in legacy ledger-only mode (no
    // sequencer wired).
    let kernel = Kernel::new();
    let bus = TuringBus::new(kernel, BusConfig::default());
    let snap: UniverseSnapshot = bus.snapshot();
    assert!(
        snap.price_index.is_empty(),
        "FC1-N6: price_index empty when bus is sequencer-less"
    );
    assert!(
        snap.mask_set.is_empty(),
        "FC1-N6 / FC2-N28: mask_set empty when bus is sequencer-less"
    );
}

#[test]
fn fc1_n8_n9_n10_output_agent_output_parseable() {
    // FC1-N8 output = ⟨q_o, a_o⟩ realized as AgentAction (the v4 name;
    // The retired v3 label was "AgentOutput"; current v4 role is AgentAction.
    // FC1-N9 q_o + FC1-N10 a_o folded into AgentAction fields.
    let _: fn(&str) -> Result<AgentAction, _> = parse_agent_output;
}

#[test]
fn fc1_n13_wtool_typed_submit_surface_present() {
    // FC1-N13 current wtool ingress is the typed submission path. Full
    // accept/reject liveness is covered by constitution_flowchart_livenow.
    let _method = TuringBus::submit_typed_tx;
}

#[test]
fn legacy_bus_append_surface_present_but_not_current_wtool_authority() {
    // Legacy append remains constructible for compatibility, but this test is
    // not current FC1 wtool coverage.
    let kernel = Kernel::new();
    let mut bus = TuringBus::new(kernel, BusConfig::default());
    let _ = bus.append("Agent_Test", "test_payload", None);
}

#[test]
fn fc1_n11_n15_predicate_zero_preserves_q_t_via_forbidden_pattern() {
    // FC1 ∏p (forbidden_patterns inline check) +
    // Q_t branch (∏p=0 preserves current state) —
    // production-path ground-truth-feedback claim (thesis claim 7).
    let kernel = Kernel::new();
    let config = BusConfig {
        forbidden_patterns: vec!["FORBIDDEN_PATTERN_TEST".into()],
        ..BusConfig::default()
    };
    let mut bus = TuringBus::new(kernel, config);
    let result = bus.append(
        "Agent_X",
        "this contains FORBIDDEN_PATTERN_TEST inline",
        None,
    );
    assert!(
        matches!(result, Ok(BusResult::Vetoed { .. })),
        "FC1 predicate failure must veto and preserve Q_t"
    );
}

// ─── FC2: init / halt / tick ───

#[test]
fn fc2_n22_halt_via_halt_and_settle() {
    // FC2-N22 HALT — TuringBus::halt_and_settle is the entry point
    // (after the ∏p=1 path that produces a golden path).
    let kernel = Kernel::new();
    let mut bus = TuringBus::new(kernel, BusConfig::default());
    let result = bus.halt_and_settle(&[]);
    // Witness: API exists + is callable. (Empty golden_path is allowed
    // for the witness; production path provides real path.)
    let _ = result;
}

#[test]
fn fc2_map_reduce_tick_tape_visible_surface_is_live() {
    use turingosv4::bottom_white::ledger::transition_ledger::TxKind;
    use turingosv4::state::sequencer::SystemEmitCommand;
    use turingosv4::state::typed_tx::TickKind;

    assert_eq!(TxKind::MapReduceTick as u8, 20);
    let _ = SystemEmitCommand::MapReduceTick {
        tick_kind: TickKind::Scheduled,
    };
}

// ─── FC3: system topology, readonly subgraph, boot, logs archive ───

#[test]
fn fc3_n34_readonly_guard_verify_trust_root_intact_repo() {
    // FC3-N34 readonly guard (B7 implementation). SHA-256 verification
    // on the live repo must pass.
    use std::path::PathBuf;
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    verify_trust_root(&repo_root).expect("FC3-N34: intact repo Trust Root verifies");
}

#[test]
fn fc3_n34_trust_root_error_taxonomy_present() {
    // FC3-N34 (failure variants) — TrustRootError taxonomy is the
    // diagnostic surface for the readonly guard.
    let _: Option<TrustRootError> = None;
}

#[test]
fn fc3_n34_parse_trust_root_section_helper() {
    // FC3-N34 helper used by trust_root_immutability conformance battery.
    let result = parse_trust_root_section("[trust_root]\n\"foo.rs\" = \"deadbeef\"\n");
    assert!(result.is_ok());
}

#[test]
fn fc3_n31_logs_archive_l4_root_fold_is_live() {
    // FC3-N31 logs archive = canonical ChainTape L4/L4.E/CAS archive.
    // WAL is legacy quarantine and must not be cited as current FC authority.
    let signing_digest = Hash::from_bytes([7u8; 32]);
    let next = append_l4_root(&Hash::ZERO, &signing_digest);
    assert_ne!(
        next,
        Hash::ZERO,
        "FC3-N31: L4 append root fold must move the canonical log root"
    );
}

#[test]
fn fc3_n39_log_ledger_present_and_appendable() {
    // FC3-N39 log = Ledger + LedgerEvent + Ledger::append.
    let mut ledger = Ledger::new();
    let event = ledger.append(EventType::RunStart, None, None, None);
    assert!(
        event.is_ok(),
        "FC3-N39: Ledger::append must succeed for RunStart"
    );
    let events: &[LedgerEvent] = ledger.events();
    assert_eq!(events.len(), 1, "FC3-N39: appended event present in ledger");
}

#[test]
fn fc3_e14_boot_panic_immediate_abort_documented() {
    // FC3 error → re-init → boot — the immediate-abort variant
    // is implemented in src/main.rs as panic on TrustRootError. The
    // OBS file documents why this is the FC3 immediate-abort leaf, not halt.
    let obs_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md");
    assert!(obs_path.exists(), "FC3 boot-failure OBS must exist");
}

#[test]
fn fc3_s3_readonly_subgraph_manifest_size() {
    // FC3 readonly subgraph — the historical trace recorded manifest size as
    // 20 files (8 PREREG base + 6 audit-add + 1 B6 + 1 B7-extra + 4 round-1
    // audit-fix). Witness: parse the live manifest, assert it has >= 20
    // entries.
    use std::fs;
    use std::path::PathBuf;
    let genesis =
        fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("genesis_payload.toml"))
            .expect("genesis_payload.toml exists");
    let entries = parse_trust_root_section(&genesis).expect("trust_root parses");
    assert!(
        entries.len() >= 20,
        "FC3 readonly manifest must have >= 20 entries (current: {}). \
         If this assertion fires, refresh the derived matrix from constitution.md.",
        entries.len()
    );
}

// ─── Additional typed surfaces and runtime meta roles ───
// FC3 ArchitectAI/Veto-AI are now runtime typed surfaces, not external-only
// governance placeholders.

#[test]
fn fc2_n23_terminal_run_outcome_taxonomy_typed() {
    // FC2-N22/N23 — the current terminal anchor is not the retired
    // enum-based halted-state shape. It is TerminalSummaryTx.run_outcome,
    // backed by the typed RunOutcome enum and ExhaustionReason projection.
    use turingosv4::state::typed_tx::{ExhaustionReason, RunOutcome, TerminalSummaryTx};

    let variants = [
        RunOutcome::OmegaAccepted,
        RunOutcome::MaxTxExhausted,
        RunOutcome::WallClockCap,
        RunOutcome::ComputeCap,
        RunOutcome::ErrorHalt,
        RunOutcome::DegradedLLM,
    ];
    assert_eq!(variants.len(), 6, "FC2-N23: RunOutcome taxonomy size");

    assert_eq!(
        ExhaustionReason::MaxTxExhausted.to_run_outcome(),
        RunOutcome::MaxTxExhausted
    );
    assert_eq!(
        ExhaustionReason::WallClockCap.to_run_outcome(),
        RunOutcome::WallClockCap
    );
    assert_eq!(
        ExhaustionReason::ComputeCap.to_run_outcome(),
        RunOutcome::ComputeCap
    );
    assert_eq!(
        ExhaustionReason::ProtocolCollapse.to_run_outcome(),
        RunOutcome::ErrorHalt
    );
    assert_eq!(
        ExhaustionReason::SolverGiveUp.to_run_outcome(),
        RunOutcome::ErrorHalt
    );
    assert_eq!(
        ExhaustionReason::DegradedLLM.to_run_outcome(),
        RunOutcome::DegradedLLM
    );

    let tx = TerminalSummaryTx {
        run_outcome: RunOutcome::MaxTxExhausted,
        ..TerminalSummaryTx::default()
    };
    assert_eq!(tx.run_outcome, RunOutcome::MaxTxExhausted);
}

#[test]
fn fc3_n32_veto_ai_runtime() {
    use turingosv4::bottom_white::cas::schema::Cid;
    use turingosv4::bottom_white::ledger::transition_ledger::TxKind;
    use turingosv4::state::q_state::{Hash, TxId};
    use turingosv4::state::sequencer::SystemEmitCommand;
    use turingosv4::state::typed_tx::{
        TypedTx, VetoDecisionTx, VetoReasonCode, VetoVerdict, VETO_DECISION_SCHEMA_ID,
    };

    assert_eq!(TxKind::VetoDecision as u8, 25);
    assert_eq!(
        TypedTx::VetoDecision(VetoDecisionTx::default()).tx_kind(),
        TxKind::VetoDecision
    );
    let _ = SystemEmitCommand::VetoDecision {
        proposal_tx_id: TxId("fc3-runtime-proposal".to_string()),
        decision_capsule_cid: Cid::from_content(VETO_DECISION_SCHEMA_ID.as_bytes()),
    };
    let verdict_domain = [VetoVerdict::Pass, VetoVerdict::Veto];
    assert_eq!(verdict_domain.len(), 2);
    assert_eq!(VetoReasonCode::ConstitutionMutationForbidden as u8, 1);
    let _root = Hash::ZERO;
}

#[test]
fn fc3_n33_architect_ai_runtime() {
    use turingosv4::bottom_white::cas::schema::Cid;
    use turingosv4::bottom_white::ledger::transition_ledger::TxKind;
    use turingosv4::state::q_state::TxId;
    use turingosv4::state::sequencer::SystemEmitCommand;
    use turingosv4::state::typed_tx::{
        ArchitectCommitTx, ArchitectProposalKind, ArchitectProposalTx, TypedTx,
        ARCHITECT_COMMIT_SCHEMA_ID, ARCHITECT_PROPOSAL_SCHEMA_ID,
    };

    assert_eq!(TxKind::ArchitectProposal as u8, 24);
    assert_eq!(TxKind::ArchitectCommit as u8, 26);
    assert_eq!(
        TypedTx::ArchitectProposal(ArchitectProposalTx::default()).tx_kind(),
        TxKind::ArchitectProposal
    );
    assert_eq!(
        TypedTx::ArchitectCommit(ArchitectCommitTx::default()).tx_kind(),
        TxKind::ArchitectCommit
    );
    let _ = SystemEmitCommand::ArchitectProposal {
        feedback_tx_id: TxId("fc3-runtime-feedback".to_string()),
        proposal_capsule_cid: Cid::from_content(ARCHITECT_PROPOSAL_SCHEMA_ID.as_bytes()),
    };
    let _ = SystemEmitCommand::ArchitectCommit {
        veto_tx_id: TxId("fc3-runtime-veto".to_string()),
        commit_capsule_cid: Cid::from_content(ARCHITECT_COMMIT_SCHEMA_ID.as_bytes()),
    };
    assert_eq!(ArchitectProposalKind::ToolRegistryPatch as u8, 1);
}

#[test]
fn fc3_support_deep_history_default_deny_runtime_witness() {
    // Deep-history reads are support-invariant gated behavior, not the
    // constitutional logs→ArchitectAI feedback edge.
    use turingosv4::runtime::markov_capsule::{
        try_deep_history_read_with_override_check, MarkovGenError,
    };

    assert!(matches!(
        try_deep_history_read_with_override_check(false),
        Err(MarkovGenError::DeepHistoryReadDenied)
    ));
    assert!(try_deep_history_read_with_override_check(true).is_ok());
}

#[test]
fn fc3_logs_feedback_typed_surface_is_live() {
    use turingosv4::bottom_white::cas::schema::Cid;
    use turingosv4::bottom_white::ledger::transition_ledger::TxKind;
    use turingosv4::state::sequencer::SystemEmitCommand;
    use turingosv4::state::typed_tx::VetoVerdict;

    assert_eq!(TxKind::LogFeedbackArchive as u8, 21);
    let _ = SystemEmitCommand::LogFeedbackArchive {
        feedback_capsule_cid: Cid::from_content(b"fc3-feedback-surface"),
        veto_verdict: VetoVerdict::Pass,
    };
}

#[test]
fn fc3_reinit_typed_surface_is_live() {
    use turingosv4::bottom_white::cas::schema::Cid;
    use turingosv4::bottom_white::ledger::transition_ledger::TxKind;
    use turingosv4::state::q_state::TxId;
    use turingosv4::state::sequencer::SystemEmitCommand;
    use turingosv4::state::typed_tx::{BootProfileId, ReinitReason};

    assert_eq!(TxKind::ReinitRequest as u8, 22);
    assert_eq!(TxKind::ReinitBoot as u8, 23);
    let _ = SystemEmitCommand::ReinitRequest {
        trigger_entry: 1,
        error_evidence_cid: Cid::from_content(b"fc3-reinit-reason"),
        reason: ReinitReason::TerminalErrorHalt,
        target_boot_profile: BootProfileId("default".to_string()),
    };
    let _ = SystemEmitCommand::ReinitBoot {
        request_tx_id: TxId("fc3-reinit-request".to_string()),
        boot_profile: BootProfileId("default".to_string()),
    };
}

#[test]
fn fc3_e15_e16_e17_constitutional_signaling() {
    let typed_tx_src = include_str!("../src/state/typed_tx.rs");
    let sequencer_src = include_str!("../src/state/sequencer.rs");
    assert!(typed_tx_src.contains("constitution_hash: Hash"));
    assert!(sequencer_src.contains("deterministic_veto_ai_verdict"));
    assert!(sequencer_src.contains("ConstitutionMutationForbidden"));
    assert!(sequencer_src.contains("MetaRoleMode::Runtime"));
}

#[test]
fn fc1_n11_predicate_trait_registry_binding_live() {
    // W3-2 landed the Predicate trait + executable registry binding. The old
    // phase-z-wtool-tools ignored stub is no longer true.
    use turingosv4::state::typed_tx::PredicateId;
    use turingosv4::top_white::predicates::registry::{
        BootPredicateManifest, PredicateBundleMap, PredicateRegistry,
    };

    let registry = PredicateRegistry::from_boot_manifest(BootPredicateManifest::v8_production())
        .expect("v8 production predicate registry");
    let acc1 = PredicateId("acc1".to_string());
    let entry = registry
        .entry(&acc1)
        .expect("acc1 executable predicate entry");
    assert_eq!(entry.impl_arc.predicate_id(), "acc1");
    assert_eq!(
        registry.code_hash_for(&acc1),
        Some(entry.impl_arc.code_hash())
    );
    assert!(registry
        .required_predicates(PredicateBundleMap::Acceptance)
        .contains(&acc1));
}

#[test]
fn fc2_n16_chaintape_boot_factory_is_current_initai_surface() {
    // FC2-N16 current executable surface is the ChainTape bootstrap factory,
    // not the retired evaluator-binary orchestration shape.
    let _: fn(
        &turingosv4::runtime::RuntimeChaintapeConfig,
    )
        -> Result<turingosv4::runtime::ChaintapeBundle, turingosv4::runtime::BootstrapError> =
        turingosv4::runtime::build_chaintape_sequencer;
    let _: fn(
        &turingosv4::runtime::RuntimeChaintapeConfig,
        turingosv4::state::q_state::QState,
    )
        -> Result<turingosv4::runtime::ChaintapeBundle, turingosv4::runtime::BootstrapError> =
        turingosv4::runtime::build_chaintape_sequencer_with_initial_q;
}

#[test]
#[ignore = "Cross-crate — Lean4Oracle in minif2f_v4 sub-crate; covered in experiments/minif2f_v4/tests/fc_alignment_conformance.rs (separate file, separate atom)"]
fn fc1_n12_lean4_oracle_ground_truth_predicate() {
    panic!("FC1-N12 cross-crate — see experiments/minif2f_v4/tests/");
}

// ───────────────────────────────────────────────────────────────────────
// TB-14 Atom 2 — price-index support witness.
// Derived matrix maps this to src/state/price_index.rs:compute_price_index
// (architect 2026-05-03 ruling §5.1 + charter §3 Atom 2). Pure deterministic
// fn over canonical EconomicState; no env / clock / RNG; replay-identical.
// ───────────────────────────────────────────────────────────────────────

#[test]
fn support_price_index_pure_fn_witness() {
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::state::q_state::AgentId;
    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
    use turingosv4::state::{compute_price_index, EconomicState, RationalPrice, TaskId, TxId};

    // Construct minimal EconomicState with one Long position.
    let mut econ = EconomicState::default();
    econ.node_positions_t.0.insert(
        TxId("witness_pos".into()),
        NodePosition {
            position_id: TxId("witness_pos".into()),
            node_id: TxId("witness_node".into()),
            task_id: TaskId("witness_task".into()),
            owner: AgentId("witness_agent".into()),
            side: PositionSide::Long,
            kind: PositionKind::FirstLong,
            amount: MicroCoin::from_micro_units(500_000),
            source_tx: TxId("witness_pos".into()),
            opened_at_round: 1,
        },
    );

    let idx = compute_price_index(&econ);
    let entry = idx
        .get(&TxId("witness_node".into()))
        .expect("price-index witness_node must appear in PriceIndex");

    // FR-14.1: price_yes derived from long_interest only.
    assert_eq!(
        entry.price_yes,
        Some(RationalPrice {
            numerator: 500_000,
            denominator: 500_000,
        }),
        "price-index witness: price_yes must follow FR-14.1"
    );

    // Replay determinism (Art.0.2): repeated calls return identical output.
    assert_eq!(
        compute_price_index(&econ),
        idx,
        "price-index witness: compute_price_index must be replay-deterministic"
    );
}

// ───────────────────────────────────────────────────────────────────────
// TB-14 Atom 3 — FC2-N28 (mask_set publication) witness.
// TRACE_MATRIX FC2-N28 maps to AgentVisibleProjection.mask_set field
// (src/state/q_state.rs:121-138) plus the derivation function
// compute_mask_set in src/state/price_index.rs (architect §5.5 +
// charter §3 Atom 3). Read-view filter; never deletes from ChainTape
// (CR-14.3 + halt-trigger #3).
// ───────────────────────────────────────────────────────────────────────

#[test]
fn fc2_n28_mask_set_publication_witness() {
    use std::collections::{BTreeMap, BTreeSet};
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::state::q_state::{AgentId, AgentVisibleProjection};
    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
    use turingosv4::state::{
        compute_mask_set, compute_price_index, BoltzmannMaskPolicy, CanonicalNodeGraph,
        EconomicState, TaskId, TxId,
    };

    // FC2-N28 (a): AgentVisibleProjection has a mask_set field of the
    // expected type, defaulting to empty.
    let proj = AgentVisibleProjection::default();
    assert!(
        proj.mask_set.is_empty(),
        "FC2-N28: AgentVisibleProjection.mask_set defaults to empty BTreeSet"
    );

    // FC2-N28 (b): compute_mask_set produces a populated set when child
    // dominates parent under the default policy.
    //
    // TB-14 Atom 6 B′ step 4 (architect ruling 2026-05-03 §3+§4): the
    // edge map is a `CanonicalNodeGraph` (BTreeMap<TxId, BTreeSet<TxId>>)
    // keyed by canonical TxIds, NOT a shadow `Tape`. The canonical IDs
    // here MUST match the NodePosition.node_id values in the EconomicState
    // — that is the post-B′-step-4 invariant envelope.
    let mut edges: CanonicalNodeGraph = BTreeMap::new();
    let mut children = BTreeSet::new();
    children.insert(TxId("child_n".into()));
    edges.insert(TxId("parent_n".into()), children);

    let mut econ = EconomicState::default();
    let mk_pos =
        |pid: &str, node: &str, side: PositionSide, kind: PositionKind, amt: i64| -> NodePosition {
            NodePosition {
                position_id: TxId(pid.into()),
                node_id: TxId(node.into()),
                task_id: TaskId("t".into()),
                owner: AgentId("a".into()),
                side,
                kind,
                amount: MicroCoin::from_micro_units(amt),
                source_tx: TxId(pid.into()),
                opened_at_round: 1,
            }
        };
    for p in [
        mk_pos(
            "p1",
            "parent_n",
            PositionSide::Long,
            PositionKind::FirstLong,
            500_000,
        ),
        mk_pos(
            "p2",
            "parent_n",
            PositionSide::Short,
            PositionKind::ChallengeShort,
            500_000,
        ),
        mk_pos(
            "p3",
            "child_n",
            PositionSide::Long,
            PositionKind::FirstLong,
            2_000_000,
        ),
    ] {
        econ.node_positions_t.0.insert(p.position_id.clone(), p);
    }

    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &edges, &policy, &price_index);

    assert!(
        mask.contains(&TxId("parent_n".into())),
        "FC2-N28: compute_mask_set must mask dominated parent"
    );

    // FC2-N28 (c): determinism — repeated calls produce identical output.
    assert_eq!(
        compute_mask_set(&econ, &edges, &policy, &price_index),
        mask,
        "FC2-N28: compute_mask_set must be replay-deterministic"
    );
}

// ───────────────────────────────────────────────────────────────────────
// TB-14 Atom 5 — boltzmann_select_parent_v2 support witness.
// Derived matrix maps this to src/sdk/actor.rs::boltzmann_select_parent_v2
// (architect §5.5 SG-14.4 + SG-14.5 + charter §3 Atom 5). Integer-rational
// argmax + epsilon-greedy; mask_set read-view filter; predicate-blind by
// type signature (Option<TxId>, no acceptance verdict).
// ───────────────────────────────────────────────────────────────────────

#[test]
fn support_boltzmann_select_parent_v2_witness() {
    use rand::SeedableRng;
    use std::collections::{BTreeMap, BTreeSet};
    use turingosv4::sdk::actor::boltzmann_select_parent_v2;
    use turingosv4::state::{BoltzmannMaskPolicy, NodeMarketEntry, RationalPrice, TxId};

    // Support witness (a): with epsilon=0, v2 picks the argmax candidate.
    let mut price_index: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
    price_index.insert(
        TxId("low_node".into()),
        NodeMarketEntry {
            price_yes: Some(RationalPrice {
                numerator: 30,
                denominator: 100,
            }),
            ..Default::default()
        },
    );
    price_index.insert(
        TxId("high_node".into()),
        NodeMarketEntry {
            price_yes: Some(RationalPrice {
                numerator: 80,
                denominator: 100,
            }),
            ..Default::default()
        },
    );
    let mask: BTreeSet<TxId> = BTreeSet::new();
    let argmax_policy = BoltzmannMaskPolicy {
        epsilon_exploration_num: 0,
        epsilon_exploration_den: 1,
        ..BoltzmannMaskPolicy::default()
    };
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let pick = boltzmann_select_parent_v2(&price_index, &mask, &argmax_policy, &mut rng);
    assert_eq!(
        pick,
        Some(TxId("high_node".into())),
        "boltzmann_select_parent_v2: argmax selection picks highest price_yes"
    );

    // Support witness (b): mask_set filters out candidates.
    let mut mask_high: BTreeSet<TxId> = BTreeSet::new();
    mask_high.insert(TxId("high_node".into()));
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let pick = boltzmann_select_parent_v2(&price_index, &mask_high, &argmax_policy, &mut rng);
    assert_eq!(
        pick,
        Some(TxId("low_node".into())),
        "boltzmann_select_parent_v2: mask_set filter removes high_node from candidates"
    );

    // Support witness (c): determinism under fixed seed.
    let run1: Vec<Option<TxId>> = {
        let mut rng = rand::rngs::StdRng::seed_from_u64(99);
        (0..30)
            .map(|_| {
                boltzmann_select_parent_v2(
                    &price_index,
                    &mask,
                    &BoltzmannMaskPolicy::default(),
                    &mut rng,
                )
            })
            .collect()
    };
    let run2: Vec<Option<TxId>> = {
        let mut rng = rand::rngs::StdRng::seed_from_u64(99);
        (0..30)
            .map(|_| {
                boltzmann_select_parent_v2(
                    &price_index,
                    &mask,
                    &BoltzmannMaskPolicy::default(),
                    &mut rng,
                )
            })
            .collect()
    };
    assert_eq!(
        run1, run2,
        "boltzmann_select_parent_v2 deterministic under fixed seed"
    );
}

// ───────────────────────────────────────────────────────────────────────
// TB-15 — autopsy, clustering, and Markov support witnesses.
// Architect §6.2 ruling 2026-05-02 + 2026-05-03. Lamarckian Autopsy +
// Markov EvidenceCapsule.
// ───────────────────────────────────────────────────────────────────────

/// TB-15 Atom 2: write_autopsy_capsule writer surface exists +
/// is callable; capsule.capsule_id is sha256-derived (deterministic);
/// privacy default = AuditOnly. Witness for src/runtime/autopsy_capsule.rs.
#[test]
fn support_write_autopsy_capsule_witness() {
    use std::sync::{Arc, RwLock};
    use tempfile::TempDir;
    use turingosv4::bottom_white::cas::schema::Cid;
    use turingosv4::bottom_white::cas::store::CasStore;
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::runtime::autopsy_capsule::{write_autopsy_capsule, LossReasonClass};
    use turingosv4::state::q_state::{AgentId, TaskId};
    use turingosv4::state::typed_tx::{CapsulePrivacyPolicy, EventId};

    let tmp = TempDir::new().unwrap();
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).unwrap()));
    let cap = write_autopsy_capsule(
        &cas,
        AgentId("witness".into()),
        EventId(TaskId("witness:event".into())),
        MicroCoin::from_micro_units(100),
        LossReasonClass::Bankruptcy,
        None,
        None,
        vec![],
        b"witness-private-detail",
        CapsulePrivacyPolicy::AuditOnly,
        "fc-witness",
        1,
        0,
    )
    .expect("autopsy capsule writer must succeed");
    assert_ne!(cap.capsule_id, Cid::default());
    assert_eq!(cap.capsule_id.0, cap.sha256.0);
    assert_eq!(cap.privacy_policy, CapsulePrivacyPolicy::AuditOnly);
}

/// TB-15 Atom 3: derive_autopsies_for_bankruptcy is a pure
/// deterministic helper consumed by both the dispatch arm + apply_one
/// hook. Witness: same inputs → same Cids.
#[test]
fn support_derive_autopsies_witness() {
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::runtime::autopsy_capsule::derive_autopsies_for_bankruptcy;
    use turingosv4::state::q_state::{AgentId, EconomicState, StakeEntry, TaskId, TxId};
    use turingosv4::state::typed_tx::TaskBankruptcyTx;

    let mut econ = EconomicState::default();
    econ.stakes_t.0.insert(
        TxId("stake_w".into()),
        StakeEntry {
            amount: MicroCoin::from_micro_units(500),
            staker: AgentId("witness_staker".into()),
            task_id: TaskId("witness:bk".into()),
        },
    );
    let bk = TaskBankruptcyTx {
        task_id: TaskId("witness:bk".into()),
        timestamp_logical: 5,
        ..Default::default()
    };
    let a = derive_autopsies_for_bankruptcy(&econ, &bk, 1, 5);
    let b = derive_autopsies_for_bankruptcy(&econ, &bk, 1, 5);
    assert_eq!(a.len(), 1);
    assert_eq!(
        a[0].capsule.capsule_id, b[0].capsule.capsule_id,
        "derive_autopsies_for_bankruptcy: deterministic Cid"
    );
}

/// TB-15 Atom 4: cluster_autopsies pure aggregator. Witness:
/// 3 same-class autopsies → 1 TypicalErrorSummary (architect §3.2.3
/// threshold). Output uses public_summary text + capsule_id Cids only.
#[test]
fn support_cluster_autopsies_witness() {
    use turingosv4::bottom_white::cas::schema::Cid;
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::runtime::autopsy_capsule::{
        cluster_autopsies, AgentAutopsyCapsule, LossReasonClass,
    };
    use turingosv4::state::q_state::{AgentId, Hash, TaskId};
    use turingosv4::state::typed_tx::{CapsulePrivacyPolicy, EventId};

    let mk = |agent: &str| AgentAutopsyCapsule {
        capsule_id: Cid::from_content(agent.as_bytes()),
        agent_id: AgentId(agent.into()),
        event_id: EventId(TaskId("e".into())),
        loss_amount: MicroCoin::from_micro_units(1),
        loss_reason_class: LossReasonClass::Bankruptcy,
        violated_risk_rule: None,
        suggested_policy_patch: None,
        evidence_cids: vec![],
        public_summary: format!("agent={} lost 1μC reason=Bankruptcy", agent),
        private_detail_cid: Cid::default(),
        privacy_policy: CapsulePrivacyPolicy::AuditOnly,
        sha256: Hash::ZERO,
        created_at_logical_t: 0,
        created_at_round: 0,
    };
    let autopsies = vec![mk("A"), mk("B"), mk("C")];
    let summaries = cluster_autopsies(&autopsies, 3);
    assert_eq!(
        summaries.len(),
        1,
        "cluster_autopsies: 3 same-class -> 1 broadcast"
    );
    assert_eq!(summaries[0].count, 3);
}

/// TB-15 Atom 5: MarkovEvidenceCapsule + writer + default-deny
/// gate witness. Capsule references constitution_hash (SG-15.7);
/// deep-history default-deny without override (FR-15.5 + halt-trigger #6).
#[test]
fn support_markov_capsule_witness() {
    use turingosv4::runtime::markov_capsule::{
        try_deep_history_read_with_override_check, MarkovEvidenceCapsule, MarkovGenError,
    };

    // SG-15.7: constitution_hash field plumbed through.
    let cap = MarkovEvidenceCapsule::with_constitution_hash([0xAB; 32]);
    assert_eq!(cap.constitution_hash.0, [0xAB; 32]);

    // FR-15.5 + halt-trigger #6: default-deny without override.
    match try_deep_history_read_with_override_check(false) {
        Err(MarkovGenError::DeepHistoryReadDenied) => {}
        other => panic!("Markov deep-history gate: expected DeepHistoryReadDenied; got {other:?}"),
    }
    assert!(try_deep_history_read_with_override_check(true).is_ok());
}
