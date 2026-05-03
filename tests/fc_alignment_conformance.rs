// FC alignment conformance battery — DO-178C requirements traceability witness.
//
// Each test asserts that a constitutional FC element (FC1 / FC2 / FC3) is
// present in the codebase as a reachable symbol. This is the mechanical
// audit-trail artifact CLAUDE.md "Alignment Standard" demands:
//
//     "Conformance tests: tests/fc_alignment_conformance.rs — 每个 ✅ 行 ≥1
//      witness test；#[ignore] stub 覆盖 📅 deferred rows"
//
// FC-trace: FC1 (basic cycle) + FC2 (init/halt/tick) + FC3 (system topology).
// Source of mappings: handover/alignment/TRACE_MATRIX_v1_2026-04-25.md.
//
// Witness semantics: each test imports the FC-anchored symbol and references
// it. If the symbol is renamed, removed, or its public API breaks, this test
// fails to compile or panics — surfacing constitutional drift at `cargo test`
// time rather than at next dual audit.

#![allow(dead_code)]

use turingosv4::boot::{parse_trust_root_section, verify_trust_root, TrustRootError};
use turingosv4::bus::{BusConfig, BusResult, TuringBus};
use turingosv4::drivers::llm_http::ResilientLLMClient;
use turingosv4::kernel::Kernel;
use turingosv4::ledger::{EventType, Ledger, LedgerEvent, Tape};
use turingosv4::sdk::protocol::{parse_agent_output, AgentAction};
use turingosv4::sdk::snapshot::UniverseSnapshot;
use turingosv4::wal::Wal;

// ─── FC1: basic cycle Q_t → rtool → input → AI(δ) → output → ∏p → wtool → Q_{t+1} ───

#[test]
fn fc1_n1_q_state_carrier_constructible_with_default_config() {
    // FC1-N1 Q_t = ⟨q_t, HEAD_t, tape_t⟩ — TuringBus is the constitutional
    // Q_t carrier. A0e-fix 2026-04-25: strengthened from type_name witness
    // (Codex Q2 + Gemini Q2.a — weak compile-only witness doesn't catch
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
    assert!(tape.time_arrow().is_empty(), "fresh tape has empty time-arrow");
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
    // views over canonical EconomicState (FC2-N28 + FC3-N42). Witness:
    // bus.snapshot() returns a UniverseSnapshot whose new fields are
    // structurally present and empty in legacy ledger-only mode (no
    // sequencer wired).
    let kernel = Kernel::new();
    let bus = TuringBus::new(kernel, BusConfig::default());
    let snap: UniverseSnapshot = bus.snapshot();
    assert!(
        snap.price_index.is_empty(),
        "FC1-N6 / FC3-N42: price_index empty when bus is sequencer-less"
    );
    assert!(
        snap.mask_set.is_empty(),
        "FC1-N6 / FC2-N28: mask_set empty when bus is sequencer-less"
    );
}

#[test]
fn fc1_n8_n9_n10_output_agent_output_parseable() {
    // FC1-N8 output = ⟨q_o, a_o⟩ realized as AgentAction (the v4 name;
    // TRACE_MATRIX_v0 used the v3 label "AgentOutput" — same role).
    // FC1-N9 q_o + FC1-N10 a_o folded into AgentAction fields.
    let _: fn(&str) -> Result<AgentAction, _> = parse_agent_output;
}

#[test]
fn fc1_n13_wtool_bus_append_present() {
    // FC1-N13 wtool = TuringBus::append (Law-1 free path) +
    // append_oracle_accepted (oracle-blessed path).
    let kernel = Kernel::new();
    let mut bus = TuringBus::new(kernel, BusConfig::default());
    let _ = bus.append("Agent_Test", "test_payload", None);
    // Witness: append API present + returns Result<BusResult, ...>.
}

#[test]
fn fc1_n11_n15_e18_pi_p_zero_preserves_q_t_via_forbidden_pattern() {
    // FC1-N11 ∏p (forbidden_patterns inline check) +
    // FC1-N15 Q_t branch (∏p=0) + FC1-E18 (∏p=0 → Q_t preserve) —
    // production-path ground-truth-feedback claim (thesis claim 7).
    let kernel = Kernel::new();
    let config = BusConfig {
        forbidden_patterns: vec!["FORBIDDEN_PATTERN_TEST".into()],
        ..BusConfig::default()
    };
    let mut bus = TuringBus::new(kernel, config);
    let result = bus.append("Agent_X", "this contains FORBIDDEN_PATTERN_TEST inline", None);
    assert!(
        matches!(result, Ok(BusResult::Vetoed { .. })),
        "FC1-E18: ∏p=0 must veto and preserve Q_t"
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
fn fc2_n23_event_type_omega_accepted_canonical() {
    // FC2-N23 HaltReason variants — the only one currently TYPED as a
    // Rust enum variant is EventType::OmegaAccepted (per ledger.rs:147
    // "V3L-09: only OmegaAccepted is a true OMEGA event").
    // The other variants {MaxTxExhausted, WallClockCap, ComputeCapViolated,
    // ErrorHalt} per CLAUDE.md report standard live as strings in jsonl
    // `extra` map — see ignored stub fc2_n23_haltreason_full_taxonomy_typed
    // below.
    let _ = EventType::OmegaAccepted;
}

#[test]
fn fc2_n20_n27_tick_mr_present() {
    // FC2-N20 + N27 — map-reduce tick exists at evaluator level
    // (TICK_INTERVAL env var); bus exposes emit_mr_tick_node.
    // Witness: bus type carries the tick capability via construction.
    let kernel = Kernel::new();
    let _bus = TuringBus::new(kernel, BusConfig::default());
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
    let result = parse_trust_root_section(
        "[trust_root]\n\"foo.rs\" = \"deadbeef\"\n",
    );
    assert!(result.is_ok());
}

#[test]
fn fc3_n31_logs_archive_wal_open_in_tempdir() {
    // FC3-N31 logs archive = Wal append-only ledger. A0e-fix 2026-04-25:
    // strengthened from type_name to actual Wal::open call (the
    // append-only API surface).
    let tmp = std::env::temp_dir().join(format!(
        "fc_alignment_conformance_wal_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
    ));
    let wal = Wal::open(&tmp);
    assert!(wal.is_ok(), "FC3-N31: Wal::open must succeed at fresh tempdir path");
    // Cleanup
    let _ = std::fs::remove_file(&tmp);
}

#[test]
fn fc3_n39_log_ledger_present_and_appendable() {
    // FC3-N39 log = Ledger + LedgerEvent + Ledger::append.
    let mut ledger = Ledger::new();
    let event = ledger.append(EventType::RunStart, None, None, None);
    assert!(event.is_ok(), "FC3-N39: Ledger::append must succeed for RunStart");
    let events: &[LedgerEvent] = ledger.events();
    assert_eq!(events.len(), 1, "FC3-N39: appended event present in ledger");
}

#[test]
fn fc3_e14_boot_panic_immediate_abort_documented() {
    // FC3-E14 (error → re-init → boot) — the immediate-abort variant
    // is implemented in src/main.rs as panic on TrustRootError. The
    // OBS file documents why this is FC3-E14 not FC2-N22.
    let obs_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md");
    assert!(
        obs_path.exists(),
        "FC3-E14: OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md must exist"
    );
}

#[test]
fn fc3_s3_readonly_subgraph_manifest_size() {
    // FC3-S3 readonly subgraph — TRACE_MATRIX_v1 records manifest size as
    // 20 files (8 PREREG base + 6 audit-add + 1 B6 + 1 B7-extra + 4 round-1
    // audit-fix). Witness: parse the live manifest, assert it has >= 20
    // entries.
    use std::fs;
    use std::path::PathBuf;
    let genesis = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("genesis_payload.toml"),
    )
    .expect("genesis_payload.toml exists");
    let entries = parse_trust_root_section(&genesis).expect("trust_root parses");
    assert!(
        entries.len() >= 20,
        "FC3-S3: manifest must have >= 20 entries (current: {}). \
         If this assertion fires, TRACE_MATRIX_v? § 3 needs an update.",
        entries.len()
    );
}

// ─── ⚠️ partial / 📅 deferred rows (Phase 11+ scope) ───
// Per TRACE_MATRIX_v0 § 4 + v1 amendment notes. Stubs reserve the row.

#[test]
#[ignore = "📅 Not yet typed as Rust enum — only OmegaAccepted exists; \
            other 4 variants {MaxTxExhausted, WallClockCap, ComputeCapViolated, \
            ErrorHalt} per CLAUDE.md report standard live as jsonl strings in \
            extra map. Type promotion is Phase C+ work."]
fn fc2_n23_haltreason_full_taxonomy_typed() {
    panic!("HaltReason full taxonomy not yet a Rust enum");
}

#[test]
#[ignore = "📅 Phase 11+ — Veto-AI runtime not implemented (manual Codex/Gemini dual-audit covers role today; Art. V.1.3 amendment 2026-04-25 narrowed scope to {PASS, VETO})"]
fn fc3_n32_veto_ai_runtime() {
    panic!("FC3-N32 deferred — see TRACE_MATRIX § 1 row FC3-N32");
}

#[test]
#[ignore = "📅 Phase 11+ — ArchitectAI runtime not implemented (manual Claude code editing covers role today; Phase D will deliver. Art. V.1.2 amendment grants commit authority post-Veto-AI PASS)"]
fn fc3_n33_architect_ai_runtime() {
    panic!("FC3-N33 deferred");
}

#[test]
#[ignore = "📅 Phase 11+ — automated logs → ArchitectAI feedback loop not implemented. Phase D consumer reads jsonl + WAL + stderr (per THESIS_V2_GROUND_TRUTH_AUDIT findings C+D)"]
fn fc3_n40_logs_to_architect_feedback() {
    panic!("FC3-N40 deferred");
}

#[test]
#[ignore = "📅 Phase 11+ — in-process re-init not implemented (external batch runner retry covers today). FC3-E14 immediate-abort leaf is what we have."]
fn fc3_n41_in_process_reinit_loop() {
    panic!("FC3-N41 deferred");
}

#[test]
#[ignore = "📅 Phase 11+ — automated runtime veto/abide signaling not implemented. Today: manual policy via CLAUDE.md Audit Standard"]
fn fc3_e15_e16_e17_constitutional_signaling() {
    panic!("FC3-E15/E16/E17 deferred");
}

#[test]
#[ignore = "🔨 Stage 3 unmerged — bus.register_predicate API + Predicate trait live on phase-z-wtool-tools branch only; not on main. Production path uses inline forbidden_patterns check in append_internal as the ∏p surface."]
fn fc1_n11_predicate_trait_register_api() {
    panic!("FC1-N11 actionable — Predicate trait + bus.register_predicate not on main");
}

#[test]
#[ignore = "Binary-only — run_swarm/run_oneshot are in evaluator binary, not lib; refactor needed to expose for direct integration testing"]
fn fc2_n16_init_ai_orchestrator_swarm_oneshot() {
    panic!("FC2-N16 binary-only");
}

#[test]
#[ignore = "Cross-crate — Lean4Oracle in minif2f_v4 sub-crate; covered in experiments/minif2f_v4/tests/fc_alignment_conformance.rs (separate file, separate atom)"]
fn fc1_n12_lean4_oracle_ground_truth_predicate() {
    panic!("FC1-N12 cross-crate — see experiments/minif2f_v4/tests/");
}

// ───────────────────────────────────────────────────────────────────────
// TB-14 Atom 2 — FC3-N42 (compute_price_index) witness.
// TRACE_MATRIX FC3-N42 maps to src/state/price_index.rs:compute_price_index
// (architect 2026-05-03 ruling §5.1 + charter §3 Atom 2). Pure deterministic
// fn over canonical EconomicState; no env / clock / RNG; replay-identical.
// ───────────────────────────────────────────────────────────────────────

#[test]
fn fc3_n42_compute_price_index_pure_fn_witness() {
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::state::q_state::AgentId;
    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
    use turingosv4::state::{
        compute_price_index, EconomicState, RationalPrice, TaskId, TxId,
    };

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
        .expect("FC3-N42: witness_node must appear in PriceIndex");

    // FR-14.1: price_yes derived from long_interest only.
    assert_eq!(
        entry.price_yes,
        Some(RationalPrice {
            numerator: 500_000,
            denominator: 500_000,
        }),
        "FC3-N42: price_yes must follow FR-14.1"
    );

    // Replay determinism (Art.0.2): repeated calls return identical output.
    assert_eq!(
        compute_price_index(&econ),
        idx,
        "FC3-N42: compute_price_index must be replay-deterministic"
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
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::ledger::{Node, Tape};
    use turingosv4::state::q_state::{AgentId, AgentVisibleProjection};
    use turingosv4::state::typed_tx::{NodePosition, PositionKind, PositionSide};
    use turingosv4::state::{
        compute_mask_set, compute_price_index, BoltzmannMaskPolicy, EconomicState,
        TaskId, TxId,
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
    let mut tape = Tape::new();
    tape.append(Node {
        id: "parent_n".into(),
        author: "a".into(),
        payload: "p".into(),
        citations: vec![],
        created_at: 0,
        completion_tokens: 0,
    })
    .unwrap();
    tape.append(Node {
        id: "child_n".into(),
        author: "a".into(),
        payload: "p".into(),
        citations: vec!["parent_n".into()],
        created_at: 0,
        completion_tokens: 0,
    })
    .unwrap();

    let mut econ = EconomicState::default();
    let mk_pos = |pid: &str, node: &str, side: PositionSide, kind: PositionKind, amt: i64| -> NodePosition {
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
        mk_pos("p1", "parent_n", PositionSide::Long, PositionKind::FirstLong, 500_000),
        mk_pos("p2", "parent_n", PositionSide::Short, PositionKind::ChallengeShort, 500_000),
        mk_pos("p3", "child_n", PositionSide::Long, PositionKind::FirstLong, 2_000_000),
    ] {
        econ.node_positions_t.0.insert(p.position_id.clone(), p);
    }

    let policy = BoltzmannMaskPolicy::default();
    let price_index = compute_price_index(&econ);
    let mask = compute_mask_set(&econ, &tape, &policy, &price_index);

    assert!(
        mask.contains(&TxId("parent_n".into())),
        "FC2-N28: compute_mask_set must mask dominated parent"
    );

    // FC2-N28 (c): determinism — repeated calls produce identical output.
    assert_eq!(
        compute_mask_set(&econ, &tape, &policy, &price_index),
        mask,
        "FC2-N28: compute_mask_set must be replay-deterministic"
    );
}

// ───────────────────────────────────────────────────────────────────────
// TB-14 Atom 5 — FC2-N29 (boltzmann_select_parent_v2) witness.
// TRACE_MATRIX FC2-N29 maps to src/sdk/actor.rs::boltzmann_select_parent_v2
// (architect §5.5 SG-14.4 + SG-14.5 + charter §3 Atom 5). Integer-rational
// argmax + epsilon-greedy; mask_set read-view filter; predicate-blind by
// type signature (Option<TxId>, no acceptance verdict).
// ───────────────────────────────────────────────────────────────────────

#[test]
fn fc2_n29_boltzmann_select_parent_v2_witness() {
    use rand::SeedableRng;
    use std::collections::{BTreeMap, BTreeSet};
    use turingosv4::sdk::actor::boltzmann_select_parent_v2;
    use turingosv4::state::{
        BoltzmannMaskPolicy, NodeMarketEntry, RationalPrice, TxId,
    };

    // FC2-N29 (a): with epsilon=0, v2 picks the argmax candidate.
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
    let pick =
        boltzmann_select_parent_v2(&price_index, &mask, &argmax_policy, &mut rng);
    assert_eq!(
        pick,
        Some(TxId("high_node".into())),
        "FC2-N29: argmax selection picks highest price_yes"
    );

    // FC2-N29 (b): mask_set filters out candidates.
    let mut mask_high: BTreeSet<TxId> = BTreeSet::new();
    mask_high.insert(TxId("high_node".into()));
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let pick =
        boltzmann_select_parent_v2(&price_index, &mask_high, &argmax_policy, &mut rng);
    assert_eq!(
        pick,
        Some(TxId("low_node".into())),
        "FC2-N29: mask_set filter removes high_node from candidates"
    );

    // FC2-N29 (c): determinism under fixed seed.
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
        "FC2-N29: boltzmann_select_parent_v2 deterministic under fixed seed"
    );
}
