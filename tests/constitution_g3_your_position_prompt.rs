//! TB-G G3.3 — `=== Your Position ===` per-viewer prompt block binding tests
//! (Class 3 — prompt-block + signature bump under parent §8 G-Phase
//! autonomous-forward authorization).
//!
//! Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
//! §1 Module G3 atom G3.3.
//!
//! Directive: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
//! §G3 verbatim 7-field `AgentMarketState` shape + Drucker framing string.
//!
//! Charter row G3.3 ship gates:
//!
//! - **SG-G3.6** per-viewer source-grep: `src/sdk/your_position.rs`
//!   renders ONLY the viewer's slice of `EconomicState`; never aggregates
//!   across agents. The source must read `compute_agent_pnl(q, viewer,
//!   _)` (per-viewer filter via the agent_pnl helper) and must NOT call
//!   any aggregating helper across `balances_t.0.values()` /
//!   `stakes_t.0.values()` without a per-viewer filter.
//! - **SG-G3.7** non-default render witnessed: when the viewer's
//!   `EconomicState` slice is non-empty (preseed balance + ≥1 open
//!   position), the rendered block carries a per-viewer-specific row
//!   (e.g. stake/claim line) — not just the default zero-state.
//! - **SG-G3.13** Drucker verbatim framing string is present at the
//!   head of the rendered block exactly as the architect specified.
//!
//! Additional binding gates:
//! - **SG-G3.13.a** no other agent's PnL or position leak: a fixture
//!   carrying Bob's stake + Alice's call produces a block that does
//!   NOT mention Bob's stake amount or tx_id.
//! - **SG-G3.13.b** `build_agent_prompt` signature carries the 10th
//!   `your_position: &str` parameter (source-grep witness).
//! - **SG-G3.13.c** the evaluator wire-up call site renders
//!   `your_position` via `your_position::render_your_position(q, agent_id)`
//!   (source-grep witness).
//!
//! `FC-trace: FC1-N7 + §15 + Art. III.2 — agent perceives its own PnL
//! state at the δ / Agent externalized output node; per-viewer; no cross-
//! agent leak. Drucker framing per architect §G3.`

use turingosv4::economy::money::MicroCoin;
use turingosv4::sdk::prompt::build_agent_prompt;
use turingosv4::sdk::your_position::{render_your_position, DRUCKER_FRAMING_LINE};
use turingosv4::state::q_state::{AgentId, QState, StakeEntry, TaskId, TxId};

const YOUR_POSITION_SRC: &str = "src/sdk/your_position.rs";
const PROMPT_SRC: &str = "src/sdk/prompt.rs";
const EVALUATOR_SRC: &str = "experiments/minif2f_v4/src/bin/evaluator.rs";

fn agent(name: &str) -> AgentId {
    AgentId(name.into())
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.6 — per-viewer source-grep witness
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_6_per_viewer_source_grep() {
    let src = std::fs::read_to_string(YOUR_POSITION_SRC).expect("read your_position src");
    // The renderer must call compute_agent_pnl with the viewer parameter,
    // which is the per-viewer filter.
    assert!(
        src.contains("compute_agent_pnl(q, viewer"),
        "SG-G3.6: source must call compute_agent_pnl(q, viewer, ...) for per-viewer filtering"
    );
    // The renderer must NOT iterate balances_t.0.values() (which would
    // aggregate across agents). Iterating with a filter on `viewer` is
    // OK; we ban the value-only iterator since it implies aggregation.
    assert!(
        !src.contains("balances_t.0.values()"),
        "SG-G3.6: aggregating iteration without viewer filter is forbidden"
    );
    // Per-viewer suppression: empty-string fall-through for unfunded viewers.
    assert!(
        src.contains("return String::new()"),
        "SG-G3.6: empty-string-suppression contract for unfunded viewers must be present"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.7 — non-default render witnessed
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_7_non_default_render_witnessed() {
    let mut q = QState::default();
    let a = agent("Agent_0");
    q.economic_state_t
        .balances_t
        .0
        .insert(a.clone(), MicroCoin::from_micro_units(800_000));
    q.economic_state_t.stakes_t.0.insert(
        TxId("worktx-witness".into()),
        StakeEntry {
            amount: MicroCoin::from_micro_units(50_000),
            staker: a.clone(),
            task_id: TaskId("task-witness".into()),
        },
    );
    let rendered = render_your_position(&q, &a);
    // SG-G3.7 binding: a non-default fixture produces a per-viewer-specific
    // row identifying the stake by its tx_id.
    assert!(
        rendered.contains("worktx-witness"),
        "SG-G3.7: non-default fixture must produce per-viewer-specific row; got:\n{rendered}"
    );
    assert!(
        rendered.contains("Open positions: 1"),
        "SG-G3.7: open_position count witnessed in block"
    );
    assert!(
        rendered.contains("Balance: 800000"),
        "SG-G3.7: balance witnessed in block"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.13 — Drucker verbatim framing
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_13_drucker_verbatim_framing() {
    let mut q = QState::default();
    let a = agent("Agent_0");
    q.economic_state_t
        .balances_t
        .0
        .insert(a.clone(), MicroCoin::from_micro_units(1_000_000));
    let rendered = render_your_position(&q, &a);
    assert!(
        rendered.starts_with(DRUCKER_FRAMING_LINE),
        "SG-G3.13: Drucker verbatim framing must be the first line; got start:\n{}",
        &rendered[..rendered.len().min(200)]
    );
    // SG-G3.13 also verifies the verbatim subphrases independently:
    for phrase in [
        "Drucker:",
        "What gets measured gets managed",
        "your position drives your next decision",
    ] {
        assert!(
            rendered.contains(phrase),
            "SG-G3.13: Drucker framing subphrase {phrase:?} must be present; got:\n{rendered}"
        );
    }
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.13.a — no other-agent PnL / position leak
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_13_a_no_other_agent_leak() {
    let mut q = QState::default();
    let alice = agent("Agent_0");
    let bob = agent("Agent_1");
    q.economic_state_t
        .balances_t
        .0
        .insert(alice.clone(), MicroCoin::from_micro_units(1_000_000));
    q.economic_state_t
        .balances_t
        .0
        .insert(bob.clone(), MicroCoin::from_micro_units(500_000));
    q.economic_state_t.stakes_t.0.insert(
        TxId("bobs-secret-stake".into()),
        StakeEntry {
            amount: MicroCoin::from_micro_units(99_999),
            staker: bob.clone(),
            task_id: TaskId("task-bob".into()),
        },
    );
    let alice_block = render_your_position(&q, &alice);
    assert!(
        !alice_block.contains("bobs-secret-stake"),
        "SG-G3.13.a: another agent's tx_id must not leak; got:\n{alice_block}"
    );
    assert!(
        !alice_block.contains("99999"),
        "SG-G3.13.a: another agent's stake amount must not leak; got:\n{alice_block}"
    );
    assert!(
        !alice_block.contains("Agent_1"),
        "SG-G3.13.a: another agent's id must not leak; got:\n{alice_block}"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.13.b — build_agent_prompt signature carries 10th param
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_13_b_build_agent_prompt_signature_has_10th_param() {
    let src = std::fs::read_to_string(PROMPT_SRC).expect("read prompt src");
    assert!(
        src.contains("your_position: &str"),
        "SG-G3.13.b: build_agent_prompt must carry the 10th `your_position: &str` parameter"
    );
    // === Your Position === heading is emitted by the prompt builder.
    assert!(
        src.contains("=== Your Position ==="),
        "SG-G3.13.b: prompt builder must emit `=== Your Position ===` heading"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.13.c — evaluator wires render_your_position into the call site
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_13_c_evaluator_wires_render_your_position() {
    let src = std::fs::read_to_string(EVALUATOR_SRC).expect("read evaluator src");
    assert!(
        src.contains("your_position::render_your_position"),
        "SG-G3.13.c: evaluator must invoke `your_position::render_your_position` at the prompt-build call site"
    );
    // The prompt build site passes `&your_position` as the 10th arg
    // (witnessed structurally — render_your_position output threaded
    // into build_agent_prompt).
    assert!(
        src.contains("&your_position"),
        "SG-G3.13.c: evaluator must pass &your_position as the 10th build_agent_prompt arg"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.13.d — block integrates into full prompt with proper heading
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_13_d_full_prompt_carries_your_position_block() {
    let body = "Drucker: 'What gets measured gets managed' \u{2014} your position drives your next decision.\nBalance: 1000000 \u{03BC}C (initial 1000000)\nRealized PnL: 0 \u{03BC}C (current balance \u{2212} initial)\nUnrealized PnL: 0 \u{03BC}C (mark-to-market on open share positions)\nSolvency: solvent\nReputation: 0\nOpen positions: 0\n";
    let prompt = build_agent_prompt("", "", "", &[], &[], "", "", "", "", body);
    assert!(
        prompt.contains("=== Your Position ==="),
        "SG-G3.13.d: full prompt must contain `=== Your Position ===` heading; got:\n{prompt}"
    );
    assert!(
        prompt.contains("Drucker:"),
        "SG-G3.13.d: full prompt must contain Drucker framing line; got:\n{prompt}"
    );
    assert!(
        prompt.contains("Balance: 1000000"),
        "SG-G3.13.d: full prompt must carry per-viewer balance line; got:\n{prompt}"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G3.13.e — empty your_position suppresses the block
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_13_e_empty_your_position_suppresses_block() {
    let prompt = build_agent_prompt("", "", "", &[], &[], "", "", "", "", "");
    assert!(
        !prompt.contains("=== Your Position ==="),
        "SG-G3.13.e: empty your_position must suppress the entire block; got:\n{prompt}"
    );
    assert!(
        !prompt.contains("Drucker:"),
        "SG-G3.13.e: empty your_position must not leak the framing string; got:\n{prompt}"
    );
}
