//! TB-G G2P.1 — Pending Peer Reviews prompt block (Class 2).
//!
//! Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
//! §1 Module G2P atom G2P.1.
//! G-Phase directive: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
//! §0.6 amendment G-2 verbatim "verify_peer=0 比 invest=0 更危险" +
//! §8.2 "Peer Verification Bridge".
//!
//! Closes user 2026-05-12 病灶3 "0 verify" surfaced in the G1.2-7 R2
//! 9-task batch evidence (`reputations_t Empty`, `verify=0` per
//! `CROSS_PROBLEM_PERSISTENCE_REPORT.md` §3 + §4 Q6.4).
//!
//! Ship gates:
//! - SG-G2P.1 — source-grep gate: `src/sdk/pending_peer_reviews.rs` is
//!   per-viewer scoped (takes `viewer: &AgentId`), reads only the
//!   chain-derived public indices (`stakes_t` + `agent_verifications_t`),
//!   and references NO private-CoT / private-diagnostic / raw-stderr /
//!   PPUT surfaces (CLAUDE.md §15 selective shielding).
//! - SG-G2P.2 — fixture-renders gate: when seeded with a peer-submitted
//!   accepted WorkTx the viewer has not yet verified, the rendered
//!   block contains the target work_tx_id, the proposer agent id, and a
//!   `verify_peer` invitation; the viewer's own WorkTx is not surfaced;
//!   already-verified targets are filtered out.
//!
//! `FC-trace: FC1-N7 δ Agent externalized output enriched with
//! peer-verification queue + §15 selective shielding (per-viewer scope)
//! + §13 verify/bond agency layer.`

use turingosv4::economy::money::MicroCoin;
use turingosv4::sdk::pending_peer_reviews::{
    render_pending_peer_reviews, DEFAULT_PENDING_REVIEWS_K,
};
use turingosv4::state::q_state::{AgentId, QState, StakeEntry, TaskId, TxId};

// ────────────────────────────────────────────────────────────────────────
// Fixtures
// ────────────────────────────────────────────────────────────────────────

fn empty_q() -> QState {
    QState::default()
}

fn seed_accepted_worktx(q: &mut QState, tx_id: &str, staker: &str, task: &str, amount_micro: i64) {
    q.economic_state_t.stakes_t.0.insert(
        TxId(tx_id.into()),
        StakeEntry {
            amount: MicroCoin::from_micro_units(amount_micro),
            staker: AgentId(staker.into()),
            task_id: TaskId(task.into()),
        },
    );
}

fn seed_existing_verification(q: &mut QState, verifier: &str, target_work_tx: &str) {
    q.economic_state_t
        .agent_verifications_t
        .0
        .insert((AgentId(verifier.into()), TxId(target_work_tx.into())));
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2P.1 — source-grep gate (per-viewer scope; no CoT leak)
// ────────────────────────────────────────────────────────────────────────

const PENDING_PEER_REVIEWS_SRC: &str = "src/sdk/pending_peer_reviews.rs";

/// SG-G2P.1.a — `render_pending_peer_reviews` MUST accept a `viewer`
/// parameter so each agent gets a per-viewer-filtered queue (not a
/// broadcast).
#[test]
fn sg_g2p_1_renderer_takes_per_viewer_id() {
    let src = std::fs::read_to_string(PENDING_PEER_REVIEWS_SRC)
        .expect("pending_peer_reviews.rs readable");
    let has_viewer_arg = src.contains("pub fn render_pending_peer_reviews(")
        && (src.contains("viewer: &AgentId") || src.contains("viewer:&AgentId"));
    assert!(
        has_viewer_arg,
        "SG-G2P.1: renderer must be per-viewer scoped (signature must declare \
         `viewer: &AgentId`) so each agent sees only its own pending queue \
         — never a broadcast of peer queues per CLAUDE.md §15."
    );
}

/// SG-G2P.1.b — the renderer reads only chain-derived public state.
/// `stakes_t` is the canonical accepted-WorkTx index; `agent_verifications_t`
/// is the canonical (verifier, target) set used by sequencer admission
/// step-3.5. Anything else (attempt telemetry, prompt capsules, proof
/// artifacts, raw stderr) is forbidden — those are private/audit-only
/// per §15.
#[test]
fn sg_g2p_1_renderer_reads_only_public_chain_indices() {
    let src = std::fs::read_to_string(PENDING_PEER_REVIEWS_SRC)
        .expect("pending_peer_reviews.rs readable");
    assert!(
        src.contains("stakes_t"),
        "SG-G2P.1: renderer must derive candidates from canonical \
         `stakes_t` (the L4-accepted-WorkTx index)."
    );
    assert!(
        src.contains("agent_verifications_t"),
        "SG-G2P.1: renderer must filter against canonical \
         `agent_verifications_t` (TB-N1 A4 duplicate-verify gate)."
    );
}

/// SG-G2P.1.c — the renderer source MUST NOT reference any private
/// surface that would leak peer CoT, raw Lean output, or private
/// diagnostic CIDs into the viewer's prompt. Mirrors the shielding gate
/// `tests/constitution_shielding_gate.rs` pattern; explicit per-symbol
/// negative tokens.
#[test]
fn sg_g2p_1_renderer_does_not_reference_private_surfaces() {
    let src = std::fs::read_to_string(PENDING_PEER_REVIEWS_SRC)
        .expect("pending_peer_reviews.rs readable");
    // Strip comment lines so that doc-prose mentioning "raw Lean stderr"
    // (as a description of what is forbidden) doesn't trip the gate.
    // Forbidden tokens must not appear in code (non-comment) lines.
    let code: String = src
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            !t.starts_with("//") && !t.starts_with("///") && !t.starts_with("//!")
        })
        .collect::<Vec<&str>>()
        .join("\n");
    let forbidden = [
        "attempt_telemetry",
        "AttemptTelemetry",
        "prompt_capsule",
        "PromptCapsule",
        "proof_artifact",
        "proposal_cid",
        "raw_stderr",
        "lean_stderr",
        "private_diagnostic",
        "chain_of_thought",
        "pput",
        "PPUT",
    ];
    for needle in &forbidden {
        assert!(
            !code.contains(needle),
            "SG-G2P.1 shielding violation: renderer code references \
             forbidden private surface `{needle}` — would leak peer CoT \
             or private-diagnostic content per CLAUDE.md §15."
        );
    }
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2P.2 — fixture renders pending-review row
// ────────────────────────────────────────────────────────────────────────

/// SG-G2P.2.a — viewer sees a peer's accepted WorkTx as a pending review
/// row and is invited to submit a `verify_peer` action.
#[test]
fn sg_g2p_2_fixture_renders_peer_work_tx_pending_review_row() {
    let mut q = empty_q();
    seed_accepted_worktx(&mut q, "worktx-peer-1", "Agent_5", "task-A", 1_000_000);
    let viewer = AgentId("Agent_0".into());
    let block = render_pending_peer_reviews(&q, &viewer, DEFAULT_PENDING_REVIEWS_K);
    assert!(
        !block.is_empty(),
        "SG-G2P.2: viewer with eligible peer target must see a non-empty \
         pending-review block; got empty."
    );
    assert!(
        block.contains("worktx-peer-1"),
        "SG-G2P.2: rendered block must reference the peer's work_tx_id; \
         block was:\n{block}"
    );
    assert!(
        block.contains("Agent_5"),
        "SG-G2P.2: rendered block must reference the peer proposer agent_id; \
         block was:\n{block}"
    );
    assert!(
        block.contains("task-A"),
        "SG-G2P.2: rendered block must reference the task_id; block was:\n{block}"
    );
    assert!(
        block.contains("verify_peer"),
        "SG-G2P.2: rendered block must invite a `verify_peer` action so the \
         agent has a concrete next step; block was:\n{block}"
    );
}

/// SG-G2P.2.b — selective-shielding witness: the viewer's OWN accepted
/// WorkTx is filtered out of its own pending queue (no self-verify), AND
/// targets the viewer has already verified are filtered out (no
/// duplicate-verify griefing — closes admission step-3.5 by removing the
/// signal from the prompt entirely).
#[test]
fn sg_g2p_2_fixture_filters_self_work_tx_and_already_verified_targets() {
    let mut q = empty_q();
    // Viewer's own WorkTx (must NOT appear in viewer's queue).
    seed_accepted_worktx(&mut q, "worktx-self", "Agent_0", "task-A", 1_000_000);
    // Peer WorkTx already verified by viewer (must NOT appear).
    seed_accepted_worktx(&mut q, "worktx-peer-done", "Agent_5", "task-A", 1_000_000);
    seed_existing_verification(&mut q, "Agent_0", "worktx-peer-done");
    // Peer WorkTx still pending viewer's verdict (MUST appear).
    seed_accepted_worktx(
        &mut q,
        "worktx-peer-pending",
        "Agent_5",
        "task-A",
        1_000_000,
    );

    let viewer = AgentId("Agent_0".into());
    let block = render_pending_peer_reviews(&q, &viewer, DEFAULT_PENDING_REVIEWS_K);

    assert!(
        !block.contains("worktx-self"),
        "SG-G2P.2 shielding: viewer's own work_tx_id must NOT appear in its \
         pending-peer-reviews queue (no self-verify); block was:\n{block}"
    );
    assert!(
        !block.contains("worktx-peer-done"),
        "SG-G2P.2 shielding: target already verified by viewer must NOT \
         reappear in pending queue (avoids duplicate-verify griefing); \
         block was:\n{block}"
    );
    assert!(
        block.contains("worktx-peer-pending"),
        "SG-G2P.2: still-eligible peer target MUST be surfaced; \
         block was:\n{block}"
    );
}

/// SG-G2P.2.c — mechanism-binding gate: `build_agent_prompt` in
/// `src/sdk/prompt.rs` actually wires the pending-peer-reviews body
/// under the canonical `=== Pending Peer Reviews ===` heading, and the
/// experiments/minif2f_v4/src/bin/evaluator.rs swarm path calls
/// `render_pending_peer_reviews` so the block reaches the LLM at
/// runtime. Without this binding, the per-viewer renderer is unreachable
/// and architect §8.2 ship gate ("at least one non-solver VerifyTx on
/// another agent's WorkTx") cannot turn green.
#[test]
fn sg_g2p_2_prompt_builder_and_evaluator_wire_the_block() {
    let prompt_src = std::fs::read_to_string("src/sdk/prompt.rs").expect("prompt.rs readable");
    assert!(
        prompt_src.contains("=== Pending Peer Reviews ==="),
        "SG-G2P.2 binding: prompt.rs must render the canonical heading \
         `=== Pending Peer Reviews ===`."
    );
    assert!(
        prompt_src.contains("pending_peer_reviews: &str"),
        "SG-G2P.2 binding: build_agent_prompt must accept a \
         `pending_peer_reviews: &str` parameter."
    );

    let evaluator_src = std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs")
        .expect("evaluator.rs readable");
    assert!(
        evaluator_src.contains("pending_peer_reviews::render_pending_peer_reviews"),
        "SG-G2P.2 binding: evaluator.rs swarm path must call \
         `pending_peer_reviews::render_pending_peer_reviews` so the per-viewer \
         block reaches the LLM at runtime."
    );
}
