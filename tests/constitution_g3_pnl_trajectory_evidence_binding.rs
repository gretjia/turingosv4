//! TB-G G3.4 — §G PnL trajectory dashboard section binding tests (Class 2).
//!
//! Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
//! §1 Module G3 atom G3.4.
//!
//! Directive: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
//! §G3 SG-G3.5 "PnL is visible in dashboard as materialized view".
//!
//! Charter row G3.4 ship gates:
//! - **SG-G3.8** dashboard §G has ≥1 non-flat trajectory row OR a
//!   MECHANISM BOTTLENECK explainer with ≥3 candidate causes (silent-
//!   zero-forbidden contract per `feedback_no_workarounds_strict_constitution`).
//! - **SG-G1.7-bind** dual-binding: G1's "one continuous ChainTape"
//!   ship gate is reinforced — the trajectory walker MUST replay the
//!   full L4 chain (not stitch independent QStates) to obtain a single
//!   final `QState`. The walker delegates to the canonical
//!   `replay_full_transition` FC2 Boot primitive.
//!
//! Additional fixture-binding gates:
//! - **SG-G3.8.a** synthetic fixture: a `QState` carrying one
//!   accepted-WorkTx stake (per the G1.2-7 R2 baseline shape) yields a
//!   `PnlTrajectorySection` with at least one non-flat row.
//! - **SG-G3.8.b** all-zero fixture (empty `QState`): the `all_flat`
//!   flag is `true` AND the rendered string contains
//!   `MECHANISM BOTTLENECK` with ≥3 distinguishable candidate causes
//!   (numbered list items 1./2./3.).
//! - **SG-G3.8.c** render contract: the rendered string contains the
//!   architect-mandated `## §G PnL trajectory` header and one row per
//!   canonical preseed agent (13 rows from
//!   `default_pput_preseed_pairs` per TB-N3 A0.5).
//! - **SG-G3.8.d** integer-only rendering: no `f64` / `f32` tokens in
//!   the render output (CLAUDE.md §13 no-f64-in-money-path).
//!
//! `FC-trace: FC1-N7 + FC2-Boot — §G dashboard is a derived view over
//! ChainTape + CAS; replay-deterministic; per-viewer aggregation of the
//! G3.1 per-agent renderer.`

use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::adapter::genesis_with_balances;
use turingosv4::runtime::agent_pnl::{
    compute_pnl_trajectory_from_paths, PnlTrajectorySection, SolvencyStatus,
};
use turingosv4::runtime::bootstrap::default_pput_preseed_pairs;
use turingosv4::state::q_state::{
    AgentId, QState, StakeEntry, TaskId, TxId,
};

// ────────────────────────────────────────────────────────────────────────
// SG-G3.8.a — synthetic-fixture non-flat row
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_8_a_synthetic_fixture_emits_non_flat_row() {
    // Mirror the G1.2-7 R2 baseline: one accepted WorkTx for Agent_0
    // with a 1_000 μC stake. balances_t drops 1_000_000 → 999_000.
    let mut q = QState::default();
    q.economic_state_t.balances_t.0.insert(
        AgentId("Agent_0".into()),
        MicroCoin::from_micro_units(999_000),
    );
    q.economic_state_t.stakes_t.0.insert(
        TxId("worktx-baseline".into()),
        StakeEntry {
            amount: MicroCoin::from_micro_units(1_000),
            staker: AgentId("Agent_0".into()),
            task_id: TaskId("task-baseline".into()),
        },
    );

    let section = PnlTrajectorySection::compute_from_q(&q);
    assert!(
        !section.all_flat,
        "SG-G3.8.a: synthetic accepted-WorkTx fixture must produce ≥1 non-flat row; rows={:?}",
        section.rows
    );
    let agent_0_row = section
        .rows
        .iter()
        .find(|r| r.agent_id.0 == "Agent_0")
        .expect("Agent_0 must appear in trajectory");
    assert_eq!(
        agent_0_row.realized_pnl, -1_000,
        "SG-G3.8.a: Agent_0 realized PnL = balance_delta = -1000 μC (stake locked)"
    );
    assert_eq!(
        agent_0_row.open_position_count, 1,
        "SG-G3.8.a: Agent_0 has 1 open stake position"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.8.b — empty-fixture renders MECHANISM BOTTLENECK with ≥3 causes
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_8_b_empty_fixture_triggers_mechanism_bottleneck() {
    // Genesis QState seeded with the canonical preseed pairs: each agent
    // gets their initial balance, but no economic action has occurred —
    // so realized=0, unrealized=0, no positions, no reputation. Every
    // row is flat ⇒ MECHANISM BOTTLENECK must fire.
    let q = genesis_with_balances(&default_pput_preseed_pairs());
    let section = PnlTrajectorySection::compute_from_q(&q);
    assert!(
        section.all_flat,
        "preseed-funded but no-action QState ⇒ all rows flat; rows={:?}",
        section.rows
    );
    let rendered = section.render_section_g();
    assert!(
        rendered.contains("MECHANISM BOTTLENECK"),
        "SG-G3.8.b: empty-fixture render must include MECHANISM BOTTLENECK; got:\n{rendered}"
    );
    // ≥ 3 enumerated candidate causes:
    for marker in ["1.", "2.", "3."] {
        assert!(
            rendered.contains(marker),
            "SG-G3.8.b: MECHANISM BOTTLENECK must enumerate cause {marker:?}; got:\n{rendered}"
        );
    }
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.8.c — §G render contract: header + 13 preseed rows
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_8_c_render_has_g_header_and_13_preseed_rows() {
    let q = QState::default();
    let section = PnlTrajectorySection::compute_from_q(&q);
    let rendered = section.render_section_g();
    assert!(
        rendered.contains("## §G PnL trajectory"),
        "SG-G3.8.c: render must contain the §G header; got:\n{rendered}"
    );
    // 13 preseed rows per TB-N3 A0.5 (tb7-7-sponsor + Agent_user_0 +
    // Agent_0..9 + MarketMakerBudget).
    assert_eq!(
        section.rows.len(),
        13,
        "SG-G3.8.c: trajectory must have 13 preseed rows (TB-N3 A0.5); got {}",
        section.rows.len()
    );
    let expected_agents = [
        "tb7-7-sponsor",
        "Agent_user_0",
        "Agent_0",
        "Agent_1",
        "Agent_2",
        "Agent_3",
        "Agent_4",
        "Agent_5",
        "Agent_6",
        "Agent_7",
        "Agent_8",
        "Agent_9",
        "MarketMakerBudget",
    ];
    for agent in expected_agents {
        let present = section.rows.iter().any(|r| r.agent_id.0 == agent);
        assert!(
            present,
            "SG-G3.8.c: preseed agent {agent:?} must appear in trajectory"
        );
    }
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.8.d — no f64 / f32 tokens in render
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_8_d_render_is_integer_only() {
    let q = QState::default();
    let section = PnlTrajectorySection::compute_from_q(&q);
    let rendered = section.render_section_g();
    // No decimal-point integer rendering (CLAUDE.md §13 economy laws).
    // The render uses raw μC integers throughout (no `1.23 Coins` style).
    for forbidden in [".0 ", ".5 ", ".25 ", ".75 "] {
        assert!(
            !rendered.contains(forbidden),
            "SG-G3.8.d: render must use integer μC only; forbidden token {forbidden:?} in:\n{rendered}"
        );
    }
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.8.e — solvency tier transitions visible in render
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_8_e_solvency_tiers_visible_in_render() {
    let mut q = QState::default();
    // Solvent: Agent_0 at full preseed.
    q.economic_state_t.balances_t.0.insert(
        AgentId("Agent_0".into()),
        MicroCoin::from_micro_units(1_000_000),
    );
    // NearInsolvent: Agent_1 at 50k (< 10% of 1M baseline).
    q.economic_state_t.balances_t.0.insert(
        AgentId("Agent_1".into()),
        MicroCoin::from_micro_units(50_000),
    );
    // Bankrupt: Agent_2 at 0 (default).

    let section = PnlTrajectorySection::compute_from_q(&q);
    let rendered = section.render_section_g();
    assert!(
        rendered.contains("solvent"),
        "SG-G3.8.e: Solvent tier must render"
    );
    assert!(
        rendered.contains("near_insolvent"),
        "SG-G3.8.e: NearInsolvent tier must render"
    );
    assert!(
        rendered.contains("bankrupt"),
        "SG-G3.8.e: Bankrupt tier must render"
    );

    let a0 = section.rows.iter().find(|r| r.agent_id.0 == "Agent_0").unwrap();
    let a1 = section.rows.iter().find(|r| r.agent_id.0 == "Agent_1").unwrap();
    let a2 = section.rows.iter().find(|r| r.agent_id.0 == "Agent_2").unwrap();
    assert!(matches!(a0.solvency_status, SolvencyStatus::Solvent));
    assert!(matches!(a1.solvency_status, SolvencyStatus::NearInsolvent));
    assert!(matches!(a2.solvency_status, SolvencyStatus::Bankrupt));
}

// ────────────────────────────────────────────────────────────────────────
// SG-G1.7-bind — dual-binding: real-evidence walker reads runtime_repo
// and yields a trajectory (smoke-check: missing evidence path returns
// a typed error, not a panic).
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g1_7_bind_missing_evidence_dir_returns_typed_error() {
    let bogus_runtime = std::path::Path::new("/nonexistent/runtime_repo");
    let bogus_cas = std::path::Path::new("/nonexistent/cas");
    let result = compute_pnl_trajectory_from_paths(bogus_runtime, bogus_cas);
    assert!(
        result.is_err(),
        "SG-G1.7-bind: missing-evidence path must return Err, not panic"
    );
}
