//! TB-G G2P.1 — Pending Peer Reviews per-viewer renderer.
//!
//! Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
//! §1 Module G2P atom G2P.1.
//! G-Phase directive: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
//! §0.6 amendment G-2 verbatim "verify_peer=0 比 invest=0 更危险" +
//! §8.2 "Peer Verification Bridge".
//!
//! Closes user 2026-05-12 病灶3 "0 verify" surfaced in
//! `handover/evidence/g_phase_g1_2_full_2026-05-11T23-36-38Z/CROSS_PROBLEM_PERSISTENCE_REPORT.md`
//! §4 Q6.4 + §4 Q6.6 mechanism-bottleneck #2 (agents lack a
//! pending-peer-review prompt block so they have no signal to pick
//! `verify_peer` over propose-and-give-up).
//!
//! **Constitutional binding** (CLAUDE.md §15 selective shielding + §13
//! verify/bond agency layer): each viewer sees ONLY the queue of accepted
//! WorkTxs they could verify next — never another viewer's queue, never
//! the proposer's private CoT, never the rejected-attempt log. The block
//! is per-viewer scoped from the ground up: filter by `viewer.staker !=
//! self` AND `(viewer, target) ∉ agent_verifications_t`.
//!
//! **Strict scope**: read-only projection of chain-derived public state.
//! No mutation. Reads only:
//! - `q.economic_state_t.stakes_t` (canonical accepted-WorkTx index)
//! - `q.economic_state_t.agent_verifications_t` (canonical verifier-set)
//!
//! NEVER reads attempt telemetry, prompt capsules, proof artifacts, raw
//! Lean stderr, private diagnostic CIDs, or any other agent's per-viewer
//! state.

use crate::state::q_state::{AgentId, QState};

/// TRACE_MATRIX FC1-N7 + §15 (selective shielding bounded-context-window)
/// — TB-G G2P.1 charter §1 Module G2P. Default cap on rows rendered.
/// Keeps prompt context bounded; the agent can still verify any WorkTx
/// visible in `=== Current Chain ===` directly — this block is a curated
/// suggestion list, not the full set.
pub const DEFAULT_PENDING_REVIEWS_K: usize = 5;

/// TRACE_MATRIX FC1-N7 + §15 + G-Phase directive §0.6 amendment G-2:
/// agent perceives the queue of accepted peer WorkTxs eligible for its
/// own `verify_peer` tool call at the δ / Agent externalized output node.
///
/// Render the pending-peer-reviews block (body only; the
/// `=== Pending Peer Reviews ===` heading is added by the prompt builder).
///
/// Per-viewer filter:
/// 1. iterate `q.economic_state_t.stakes_t.0` (canonical L4-accepted-WorkTx index)
/// 2. drop `staker == viewer` (no self-verify)
/// 3. drop `(viewer, tx_id) ∈ agent_verifications_t` (no duplicate-verify griefing)
/// 4. compute `already_verified_by_peers = |{v : (v, tx_id) ∈ agent_verifications_t}|`
///
/// Sort: stake desc (highest-stake WorkTxs first → market signal +
/// highest verify reward potential under TB-N1 A4 reputation accrual),
/// tiebreak by `tx_id` asc for determinism.
///
/// Returns empty string when no candidates exist; the prompt builder
/// suppresses the entire block in that case (mirrors `econ_position.rs`
/// suppression contract — V3L-40 layout stability across runs).
///
/// `K` caps the number of rows rendered. `DEFAULT_PENDING_REVIEWS_K = 5`
/// keeps prompt budget bounded.
pub fn render_pending_peer_reviews(q: &QState, viewer: &AgentId, k: usize) -> String {
    let mut candidates: Vec<(String, String, String, i64, usize)> = Vec::new();
    for (tx_id, stake_entry) in q.economic_state_t.stakes_t.0.iter() {
        if &stake_entry.staker == viewer {
            continue;
        }
        if q
            .economic_state_t
            .agent_verifications_t
            .0
            .contains(&(viewer.clone(), tx_id.clone()))
        {
            continue;
        }
        let already_verified_by = q
            .economic_state_t
            .agent_verifications_t
            .0
            .iter()
            .filter(|(_, t)| t == tx_id)
            .count();
        candidates.push((
            tx_id.0.clone(),
            stake_entry.staker.0.clone(),
            stake_entry.task_id.0.clone(),
            stake_entry.amount.micro_units(),
            already_verified_by,
        ));
    }
    if candidates.is_empty() {
        return String::new();
    }
    candidates.sort_by(|a, b| b.3.cmp(&a.3).then_with(|| a.0.cmp(&b.0)));
    let mut s = String::new();
    for (tx_id, proposer, task_id, stake_micro, verifier_count) in candidates.iter().take(k) {
        s.push_str(&format!(
            "- work_tx {tx_id}: proposer={proposer} task={task_id} stake={stake_micro} \u{03BC}Coin already_verified_by={verifier_count} peer(s)\n"
        ));
    }
    s.push_str(
        "Submit a `verify_peer` action against one of the above target_work_tx_ids if you can judge the proof step \
         (verdict=confirm if correct; verdict=deny if wrong). Your bond is debited from balance until the run resolves; correct verdict earns reward.\n",
    );
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::economy::money::{MicroCoin, StakeMicroCoin};
    use crate::state::q_state::{QState, StakeEntry, TaskId, TxId};

    fn empty_q() -> QState {
        QState::default()
    }

    fn seed_stake(q: &mut QState, tx_id: &str, staker: &str, task: &str, amount_micro: i64) {
        q.economic_state_t.stakes_t.0.insert(
            TxId(tx_id.into()),
            StakeEntry {
                amount: MicroCoin::from_micro_units(amount_micro),
                staker: AgentId(staker.into()),
                task_id: TaskId(task.into()),
            },
        );
        // Suppress the unused-import diagnostic for StakeMicroCoin which
        // mirrors the wire shape of StakeEntry.amount in production paths
        // (TB-N1 A4 + Stage A* stakes_t writes); kept here as a structural
        // anchor so future schema rotations surface in this test too.
        let _ = StakeMicroCoin::from_micro_units(0);
    }

    fn seed_verification(q: &mut QState, verifier: &str, target: &str) {
        q.economic_state_t
            .agent_verifications_t
            .0
            .insert((AgentId(verifier.into()), TxId(target.into())));
    }

    #[test]
    fn empty_q_yields_empty_block_suppresses_prompt_section() {
        let q = empty_q();
        let viewer = AgentId("Agent_0".into());
        let s = render_pending_peer_reviews(&q, &viewer, DEFAULT_PENDING_REVIEWS_K);
        assert!(s.is_empty(), "empty stakes_t → suppressed block; got: {s}");
    }

    #[test]
    fn viewer_does_not_see_own_work_tx_as_pending_review() {
        let mut q = empty_q();
        seed_stake(&mut q, "worktx-self", "Agent_0", "task-A", 1_000_000);
        let viewer = AgentId("Agent_0".into());
        let s = render_pending_peer_reviews(&q, &viewer, DEFAULT_PENDING_REVIEWS_K);
        assert!(
            s.is_empty(),
            "viewer must NOT see its own WorkTx as pending peer review; got: {s}"
        );
    }

    #[test]
    fn viewer_sees_peer_work_tx_as_pending_review() {
        let mut q = empty_q();
        seed_stake(&mut q, "worktx-peer", "Agent_5", "task-A", 1_000_000);
        let viewer = AgentId("Agent_0".into());
        let s = render_pending_peer_reviews(&q, &viewer, DEFAULT_PENDING_REVIEWS_K);
        assert!(s.contains("worktx-peer"), "block must reference peer work_tx_id; got: {s}");
        assert!(s.contains("Agent_5"), "block must reference peer proposer; got: {s}");
        assert!(s.contains("task-A"), "block must reference task_id; got: {s}");
        assert!(
            s.contains("verify_peer"),
            "block must invite a verify_peer action; got: {s}"
        );
    }

    #[test]
    fn viewer_already_verified_target_is_filtered_out() {
        let mut q = empty_q();
        seed_stake(&mut q, "worktx-peer", "Agent_5", "task-A", 1_000_000);
        seed_verification(&mut q, "Agent_0", "worktx-peer");
        let viewer = AgentId("Agent_0".into());
        let s = render_pending_peer_reviews(&q, &viewer, DEFAULT_PENDING_REVIEWS_K);
        assert!(
            s.is_empty(),
            "viewer that already verified the target must not see it as pending; got: {s}"
        );
    }

    #[test]
    fn other_viewers_queue_does_not_leak_to_this_viewer() {
        // Selective shielding: Agent_0's queue must not contain a row that
        // says "Agent_5 has not yet verified <tx>". Each viewer sees only
        // their own personal eligibility filter.
        let mut q = empty_q();
        seed_stake(&mut q, "worktx-by-A1", "Agent_1", "task-X", 500_000);
        // Agent_0 already verified it (so Agent_0's view: empty).
        seed_verification(&mut q, "Agent_0", "worktx-by-A1");
        let viewer0 = AgentId("Agent_0".into());
        let s0 = render_pending_peer_reviews(&q, &viewer0, DEFAULT_PENDING_REVIEWS_K);
        assert!(s0.is_empty(), "Agent_0 already verified → empty queue; got: {s0}");

        // Agent_5 has not verified it → still pending in Agent_5's view.
        let viewer5 = AgentId("Agent_5".into());
        let s5 = render_pending_peer_reviews(&q, &viewer5, DEFAULT_PENDING_REVIEWS_K);
        assert!(s5.contains("worktx-by-A1"), "Agent_5 still has it pending; got: {s5}");
    }

    #[test]
    fn already_verified_by_peers_count_renders_correctly() {
        let mut q = empty_q();
        seed_stake(&mut q, "worktx-popular", "Agent_5", "task-A", 1_000_000);
        seed_verification(&mut q, "Agent_1", "worktx-popular");
        seed_verification(&mut q, "Agent_2", "worktx-popular");
        seed_verification(&mut q, "Agent_3", "worktx-popular");
        let viewer = AgentId("Agent_0".into());
        let s = render_pending_peer_reviews(&q, &viewer, DEFAULT_PENDING_REVIEWS_K);
        assert!(
            s.contains("already_verified_by=3 peer(s)"),
            "count must reflect 3 prior verifiers; got: {s}"
        );
    }

    #[test]
    fn sort_is_stake_desc_with_tx_id_asc_tiebreak() {
        let mut q = empty_q();
        seed_stake(&mut q, "worktx-c-large", "Agent_5", "task-A", 5_000_000);
        seed_stake(&mut q, "worktx-a-small", "Agent_5", "task-A", 1_000_000);
        seed_stake(&mut q, "worktx-b-small", "Agent_5", "task-A", 1_000_000);
        let viewer = AgentId("Agent_0".into());
        let s = render_pending_peer_reviews(&q, &viewer, DEFAULT_PENDING_REVIEWS_K);
        let pos_large = s.find("worktx-c-large").expect("c-large present");
        let pos_a = s.find("worktx-a-small").expect("a-small present");
        let pos_b = s.find("worktx-b-small").expect("b-small present");
        assert!(pos_large < pos_a, "stake desc: c-large(5M) before a-small(1M)");
        assert!(pos_a < pos_b, "tx_id asc tiebreak: a-small before b-small at same stake");
    }

    #[test]
    fn k_caps_row_count() {
        let mut q = empty_q();
        for i in 0..10 {
            seed_stake(&mut q, &format!("worktx-{i:02}"), "Agent_5", "task-A", 1_000_000);
        }
        let viewer = AgentId("Agent_0".into());
        let s = render_pending_peer_reviews(&q, &viewer, 3);
        let row_count = s.lines().filter(|l| l.starts_with("- work_tx ")).count();
        assert_eq!(row_count, 3, "k=3 must cap rows; got {row_count}\nblock:\n{s}");
    }
}
