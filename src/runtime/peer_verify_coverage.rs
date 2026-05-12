//! TB-G G2P.2 — peer-verify-coverage walker.
//!
//! Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
//! §1 Module G2P atom G2P.2.
//! G-Phase directive: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
//! §0.6 amendment G-2 verbatim "verify_peer=0 比 invest=0 更危险".
//!
//! Closes user 2026-05-12 病灶3 "0 verify" surfaced in G1.2-7 R2
//! 9-task batch (`CROSS_PROBLEM_PERSISTENCE_REPORT.md` §4 Q6.4 reports
//! `verify=0`). G2P.1 surfaces the per-viewer prompt block; G2P.2 binds
//! the post-batch evidence walker that quantifies coverage.
//!
//! **Constitutional binding** (CLAUDE.md §17 reporting standard + §15
//! shielding): the walker reads ONLY canonical chain-derived public
//! state — L4 ledger entries decoded as `TypedTx`. No CAS attempt
//! telemetry, no private diagnostics, no prompt capsules. Output is a
//! dashboard-only materialized view per CLAUDE.md §1 hierarchy "dashboard
//! is not source of truth".
//!
//! Render contract (CLAUDE.md §17 + architect §8.2 ship gate):
//! - aggregate `accepted_worktx_total`, `accepted_worktx_with_verify`,
//!   `coverage_pct`, `peer_verifications_total`, `non_solver_verifications`
//! - per-verifier `peer_verify_count` map
//! - per-target `verifier_count` map
//! - when `non_solver_verifications == 0`, the rendered §F.X block
//!   includes an EXPLICIT mechanism-bottleneck explanation (architect
//!   §8.5 empty-market is a valid result + `CROSS_PROBLEM_PERSISTENCE_REPORT.md`
//!   §4 Q6.6 cause #2). Silent zero is forbidden per
//!   `feedback_no_workarounds_strict_constitution` (no AMBER residual).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::transition_ledger::{
    canonical_decode, Git2LedgerWriter, LedgerWriter, LedgerWriterError,
};
use crate::state::q_state::{AgentId, TxId};
use crate::state::typed_tx::TypedTx;

/// TRACE_MATRIX FC1-N7 + §17 reporting standard — TB-G G2P.2 charter §1
/// Module G2P atom G2P.2. Aggregate + per-axis peer-verification
/// coverage view derived from L4 walk.
///
/// `solver_agents` = agents who submitted ≥1 accepted WorkTx on this
/// chain. `non_solver_verifications` counts accepted VerifyTxs whose
/// `verifier_agent ∉ solver_agents` — the architect §8.2 ship gate
/// signal ("at least one non-solver VerifyTx on another agent's
/// WorkTx").
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PeerVerifyCoverage {
    /// Number of distinct accepted WorkTx tx_ids on the chain.
    pub accepted_worktx_total: u64,
    /// Number of accepted WorkTx tx_ids that have ≥1 accepted VerifyTx
    /// targeting them.
    pub accepted_worktx_with_verify: u64,
    /// `floor(100 * accepted_worktx_with_verify / accepted_worktx_total)`.
    /// Zero when `accepted_worktx_total == 0` (no division-by-zero panic).
    pub coverage_pct: u64,
    /// Total accepted VerifyTxs on the chain.
    pub peer_verifications_total: u64,
    /// Accepted VerifyTxs whose `verifier_agent` did NOT also submit any
    /// accepted WorkTx on this chain — the architect §8.2 health signal.
    pub non_solver_verifications: u64,
    /// per-verifier accepted-VerifyTx count (`verifier_agent` → count).
    pub per_verifier_count: BTreeMap<AgentId, u64>,
    /// per-target accepted-VerifyTx count (`target_work_tx` → count).
    pub per_target_count: BTreeMap<TxId, u64>,
    /// Agents who submitted ≥1 accepted WorkTx on this chain.
    pub solver_agents: BTreeSet<AgentId>,
}

/// TRACE_MATRIX § 3 orphan — TB-G G2P.2 charter §1 Module G2P. Walker
/// error wrapping the underlying ledger / CAS open + decode failures.
#[derive(Debug)]
pub enum PeerVerifyCoverageError {
    LedgerOpen(LedgerWriterError),
    LedgerRead(LedgerWriterError),
    CasOpen(String),
}

impl std::fmt::Display for PeerVerifyCoverageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LedgerOpen(e) => write!(f, "open L4 ledger: {e}"),
            Self::LedgerRead(e) => write!(f, "read L4 entry: {e}"),
            Self::CasOpen(e) => write!(f, "open CAS: {e}"),
        }
    }
}

impl std::error::Error for PeerVerifyCoverageError {}

impl From<LedgerWriterError> for PeerVerifyCoverageError {
    fn from(e: LedgerWriterError) -> Self {
        Self::LedgerRead(e)
    }
}

/// TRACE_MATRIX FC1-N7 — TB-G G2P.2 SG-G2P.3 walker: emit per-agent
/// `peer_verify_count` from canonical L4 + CAS evidence.
///
/// Walks `1..=writer.len()`, decodes each entry's payload from CAS as
/// `TypedTx`, and accumulates the per-axis counts. Trait-object-typed
/// to admit both `Git2LedgerWriter` (production) and
/// `InMemoryLedgerWriter` (tests).
pub fn compute_peer_verify_coverage(
    writer: &dyn LedgerWriter,
    cas: &CasStore,
) -> Result<PeerVerifyCoverage, PeerVerifyCoverageError> {
    let mut cov = PeerVerifyCoverage::default();
    let mut accepted_worktx_ids: BTreeSet<TxId> = BTreeSet::new();
    let l4_count = writer.len();
    for t in 1..=l4_count {
        let entry = writer.read_at(t)?;
        let payload = match cas.get(&entry.tx_payload_cid) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let typed_tx: TypedTx = match canonical_decode(&payload) {
            Ok(t) => t,
            Err(_) => continue,
        };
        match &typed_tx {
            TypedTx::Work(work) => {
                accepted_worktx_ids.insert(work.tx_id.clone());
                cov.solver_agents.insert(work.agent_id.clone());
            }
            TypedTx::Verify(verify) => {
                cov.peer_verifications_total += 1;
                *cov.per_verifier_count
                    .entry(verify.verifier_agent.clone())
                    .or_insert(0) += 1;
                *cov.per_target_count
                    .entry(verify.target_work_tx.clone())
                    .or_insert(0) += 1;
            }
            _ => {}
        }
    }
    cov.accepted_worktx_total = accepted_worktx_ids.len() as u64;
    cov.accepted_worktx_with_verify = accepted_worktx_ids
        .iter()
        .filter(|id| cov.per_target_count.get(id).copied().unwrap_or(0) > 0)
        .count() as u64;
    cov.coverage_pct = if cov.accepted_worktx_total > 0 {
        (cov.accepted_worktx_with_verify.saturating_mul(100))
            / cov.accepted_worktx_total
    } else {
        0
    };
    cov.non_solver_verifications = cov
        .per_verifier_count
        .iter()
        .filter(|(verifier, _)| !cov.solver_agents.contains(*verifier))
        .map(|(_, n)| *n)
        .sum();
    Ok(cov)
}

/// TRACE_MATRIX FC1-N7 — TB-G G2P.2 path-based wrapper for the
/// `audit_dashboard --run-report` §F.X integration. Opens the
/// `Git2LedgerWriter` at `runtime_repo_path` + `CasStore` at `cas_path`
/// and delegates to `compute_peer_verify_coverage`.
pub fn compute_peer_verify_coverage_from_paths(
    runtime_repo_path: &Path,
    cas_path: &Path,
) -> Result<PeerVerifyCoverage, PeerVerifyCoverageError> {
    let writer = Git2LedgerWriter::open(runtime_repo_path)
        .map_err(PeerVerifyCoverageError::LedgerOpen)?;
    let cas = CasStore::open(cas_path).map_err(|e| PeerVerifyCoverageError::CasOpen(e.to_string()))?;
    compute_peer_verify_coverage(&writer, &cas)
}

impl PeerVerifyCoverage {
    /// TRACE_MATRIX FC1-N7 — TB-G G2P.2 SG-G2P.4 + SG-G2P.5 render: emit
    /// the `## §F.X Peer-verify coverage` dashboard section. Includes an
    /// EXPLICIT mechanism-bottleneck explanation when
    /// `non_solver_verifications == 0` (no silent zero per
    /// `feedback_no_workarounds_strict_constitution`).
    pub fn render_section_f_x(&self) -> String {
        let mut out = String::new();
        out.push_str("\n## §F.X Peer-verify coverage\n");
        out.push_str(&format!(
            "  accepted_worktx_total: {}\n",
            self.accepted_worktx_total
        ));
        out.push_str(&format!(
            "  accepted_worktx_with_verify: {}\n",
            self.accepted_worktx_with_verify
        ));
        out.push_str(&format!("  coverage_pct: {}%\n", self.coverage_pct));
        out.push_str(&format!(
            "  peer_verifications_total: {}\n",
            self.peer_verifications_total
        ));
        out.push_str(&format!(
            "  non_solver_verifications: {}\n",
            self.non_solver_verifications
        ));
        if !self.per_verifier_count.is_empty() {
            out.push_str("  per-agent peer_verify_count:\n");
            let mut rows: Vec<(&AgentId, &u64)> = self.per_verifier_count.iter().collect();
            rows.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.0.cmp(&b.0.0)));
            for (agent, n) in rows.iter().take(20) {
                let role = if self.solver_agents.contains(agent) {
                    "solver"
                } else {
                    "non_solver"
                };
                out.push_str(&format!("    - {} ({}): {}\n", agent.0, role, n));
            }
        }
        if self.non_solver_verifications == 0 {
            out.push_str("  MECHANISM BOTTLENECK (architect §8.2 ship gate unmet — no non-solver VerifyTx):\n");
            out.push_str("    1. round-robin scheduler `agent_idx = tx % n_agents`\n");
            out.push_str("       (G-Phase directive amendment G-4 \"伪多智能体\"):\n");
            out.push_str("       Agent_i may never be selected for verify path;\n");
            out.push_str("       G5.1 opportunity scheduler + 7-action menu is the\n");
            out.push_str("       forward fix.\n");
            out.push_str("    2. `=== Pending Peer Reviews ===` prompt block (G2P.1)\n");
            out.push_str("       must be active on the swarm prompt path so agents\n");
            out.push_str("       perceive eligible targets at the δ Agent externalized\n");
            out.push_str("       output node.\n");
            out.push_str("    3. agent verify_peer bond budget (TB-N1 A4 admission step-2.5)\n");
            out.push_str("       may exceed balance after WorkTx stake locked — confirm\n");
            out.push_str("       persistent-batch preseed (TURINGOS_CHAINTAPE_PRESEED=1)\n");
            out.push_str("       seeds non-solver agents with adequate balance.\n");
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bottom_white::cas::schema::ObjectType;
    use crate::bottom_white::cas::store::CasStore;
    use crate::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
    use crate::bottom_white::ledger::transition_ledger::{
        append, canonical_encode, InMemoryLedgerWriter, LedgerEntry,
        LedgerEntrySigningPayload, LedgerWriter, TxKind,
    };
    use crate::economy::money::StakeMicroCoin;
    use crate::state::q_state::{Hash, TaskId, TxId};
    use crate::state::typed_tx::{
        AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey,
        SafetyOrCreation, VerifyTx, VerifyVerdict, WorkTx, WriteKey,
    };
    use std::collections::{BTreeMap, BTreeSet};
    use tempfile::TempDir;

    fn fresh_cas(tmp: &TempDir) -> CasStore {
        CasStore::open(tmp.path()).expect("cas open")
    }

    fn write_typed_tx(
        cas: &mut CasStore,
        writer: &mut dyn LedgerWriter,
        typed: TypedTx,
        kind: TxKind,
    ) {
        let bytes = canonical_encode(&typed).expect("encode");
        let cid = cas
            .put(
                &bytes,
                ObjectType::ProposalPayload,
                "g2p2-test",
                writer.len() + 1,
                None,
            )
            .expect("cas put");
        let logical_t = writer.len() + 1;
        let parent_ledger_root = if logical_t == 1 {
            Hash::ZERO
        } else {
            writer.read_at(logical_t - 1).expect("prev").resulting_ledger_root
        };
        let signing = LedgerEntrySigningPayload {
            logical_t,
            parent_state_root: Hash::ZERO,
            parent_ledger_root,
            tx_kind: kind,
            tx_payload_cid: cid,
            resulting_state_root: Hash::ZERO,
            timestamp_logical: logical_t,
            epoch: SystemEpoch::new(1),
            extensions: BTreeMap::new(),
        };
        let signing_digest = signing.canonical_digest();
        let resulting_ledger_root = append(&parent_ledger_root, &signing_digest);
        let entry = LedgerEntry {
            logical_t,
            parent_state_root: signing.parent_state_root,
            parent_ledger_root,
            tx_kind: kind,
            tx_payload_cid: cid,
            resulting_state_root: Hash::ZERO,
            resulting_ledger_root,
            timestamp_logical: logical_t,
            epoch: SystemEpoch::new(1),
            extensions: BTreeMap::new(),
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        };
        writer.commit(&entry).expect("commit");
    }

    fn make_worktx(tx_id: &str, agent: &str, task: &str) -> TypedTx {
        let mut acceptance = BTreeMap::new();
        acceptance.insert(
            PredicateId("acc1".into()),
            BoolWithProof { value: true, proof_cid: None },
        );
        let mut read_set = BTreeSet::new();
        read_set.insert(ReadKey("k".into()));
        let mut write_set = BTreeSet::new();
        write_set.insert(WriteKey("k".into()));
        TypedTx::Work(WorkTx {
            tx_id: TxId(tx_id.into()),
            task_id: TaskId(task.into()),
            parent_state_root: Hash::ZERO,
            agent_id: AgentId(agent.into()),
            read_set,
            write_set,
            proposal_cid: Default::default(),
            predicate_results: PredicateResultsBundle {
                acceptance,
                settlement: BTreeMap::new(),
                safety_class: SafetyOrCreation::Safety,
            },
            stake: StakeMicroCoin::from_micro_units(1_000),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 1,
        })
    }

    fn make_verifytx(tx_id: &str, verifier: &str, target: &str) -> TypedTx {
        TypedTx::Verify(VerifyTx {
            tx_id: TxId(tx_id.into()),
            parent_state_root: Hash::ZERO,
            target_work_tx: TxId(target.into()),
            verifier_agent: AgentId(verifier.into()),
            bond: StakeMicroCoin::from_micro_units(100),
            verdict: VerifyVerdict::Confirm,
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 1,
        })
    }

    #[test]
    fn empty_chain_yields_zero_coverage() {
        let tmp = TempDir::new().unwrap();
        let cas = fresh_cas(&tmp);
        let writer = InMemoryLedgerWriter::new();
        let cov = compute_peer_verify_coverage(&writer, &cas).expect("walker");
        assert_eq!(cov.accepted_worktx_total, 0);
        assert_eq!(cov.coverage_pct, 0);
        assert_eq!(cov.peer_verifications_total, 0);
        assert_eq!(cov.non_solver_verifications, 0);
    }

    #[test]
    fn walker_emits_per_verifier_peer_verify_count() {
        let tmp = TempDir::new().unwrap();
        let mut cas = fresh_cas(&tmp);
        let mut writer = InMemoryLedgerWriter::new();
        // 1 accepted WorkTx by Agent_5.
        write_typed_tx(
            &mut cas,
            &mut writer,
            make_worktx("worktx-1", "Agent_5", "task-A"),
            TxKind::Work,
        );
        // 2 accepted VerifyTxs by Agent_0 (non-solver) on different targets.
        write_typed_tx(
            &mut cas,
            &mut writer,
            make_verifytx("verifytx-a", "Agent_0", "worktx-1"),
            TxKind::Verify,
        );
        // Another WorkTx + Verify pair.
        write_typed_tx(
            &mut cas,
            &mut writer,
            make_worktx("worktx-2", "Agent_5", "task-B"),
            TxKind::Work,
        );
        write_typed_tx(
            &mut cas,
            &mut writer,
            make_verifytx("verifytx-b", "Agent_0", "worktx-2"),
            TxKind::Verify,
        );

        let cov = compute_peer_verify_coverage(&writer, &cas).expect("walker");
        assert_eq!(cov.accepted_worktx_total, 2);
        assert_eq!(cov.accepted_worktx_with_verify, 2);
        assert_eq!(cov.coverage_pct, 100);
        assert_eq!(cov.peer_verifications_total, 2);
        assert_eq!(cov.non_solver_verifications, 2);
        assert_eq!(
            cov.per_verifier_count.get(&AgentId("Agent_0".into())).copied(),
            Some(2)
        );
        assert!(cov.solver_agents.contains(&AgentId("Agent_5".into())));
        assert!(!cov.solver_agents.contains(&AgentId("Agent_0".into())));
    }

    #[test]
    fn coverage_pct_handles_partial_verification() {
        let tmp = TempDir::new().unwrap();
        let mut cas = fresh_cas(&tmp);
        let mut writer = InMemoryLedgerWriter::new();
        // 4 WorkTx; only 1 has a Verify.
        for i in 0..4 {
            write_typed_tx(
                &mut cas,
                &mut writer,
                make_worktx(&format!("worktx-{i}"), "Agent_5", "task-A"),
                TxKind::Work,
            );
        }
        write_typed_tx(
            &mut cas,
            &mut writer,
            make_verifytx("verifytx-1", "Agent_0", "worktx-0"),
            TxKind::Verify,
        );
        let cov = compute_peer_verify_coverage(&writer, &cas).expect("walker");
        assert_eq!(cov.accepted_worktx_total, 4);
        assert_eq!(cov.accepted_worktx_with_verify, 1);
        assert_eq!(cov.coverage_pct, 25);
    }

    #[test]
    fn non_solver_classification_excludes_proposer_self_verifies() {
        // Defensive: even if a solver-agent verifies a peer's WorkTx, the
        // verifier still counts (it's an accepted verify); but they do NOT
        // count toward `non_solver_verifications`. Closes architect §8.2
        // "non-solver VerifyTx" ship-gate semantic.
        let tmp = TempDir::new().unwrap();
        let mut cas = fresh_cas(&tmp);
        let mut writer = InMemoryLedgerWriter::new();
        // Two solver agents.
        write_typed_tx(
            &mut cas,
            &mut writer,
            make_worktx("worktx-A5", "Agent_5", "task-A"),
            TxKind::Work,
        );
        write_typed_tx(
            &mut cas,
            &mut writer,
            make_worktx("worktx-A6", "Agent_6", "task-A"),
            TxKind::Work,
        );
        // Agent_6 verifies Agent_5's WorkTx (peer, but Agent_6 is itself
        // a solver — should NOT count for non_solver_verifications).
        write_typed_tx(
            &mut cas,
            &mut writer,
            make_verifytx("verifytx-x", "Agent_6", "worktx-A5"),
            TxKind::Verify,
        );
        // Agent_0 verifies Agent_6's WorkTx (non-solver verifier).
        write_typed_tx(
            &mut cas,
            &mut writer,
            make_verifytx("verifytx-y", "Agent_0", "worktx-A6"),
            TxKind::Verify,
        );
        let cov = compute_peer_verify_coverage(&writer, &cas).expect("walker");
        assert_eq!(cov.peer_verifications_total, 2);
        assert_eq!(
            cov.non_solver_verifications, 1,
            "only Agent_0 (non-solver) counts; Agent_6 is itself a solver. cov={cov:#?}"
        );
    }

    #[test]
    fn section_f_x_render_includes_coverage_pct_line() {
        let mut cov = PeerVerifyCoverage::default();
        cov.accepted_worktx_total = 9;
        cov.accepted_worktx_with_verify = 3;
        cov.coverage_pct = 33;
        cov.peer_verifications_total = 5;
        cov.non_solver_verifications = 4;
        cov.per_verifier_count
            .insert(AgentId("Agent_0".into()), 4);
        cov.per_verifier_count
            .insert(AgentId("Agent_5".into()), 1);
        cov.solver_agents.insert(AgentId("Agent_5".into()));
        let rendered = cov.render_section_f_x();
        assert!(rendered.contains("## §F.X Peer-verify coverage"));
        assert!(rendered.contains("coverage_pct: 33%"));
        assert!(rendered.contains("non_solver_verifications: 4"));
        assert!(rendered.contains("Agent_0 (non_solver): 4"));
        assert!(rendered.contains("Agent_5 (solver): 1"));
        // Positive case: no mechanism-bottleneck block.
        assert!(
            !rendered.contains("MECHANISM BOTTLENECK"),
            "non_solver_verifications > 0 must not render the bottleneck block; \
             rendered:\n{rendered}"
        );
    }

    #[test]
    fn section_f_x_render_zero_non_solver_emits_explicit_bottleneck() {
        // SG-G2P.5: zero non-solver verifications → explicit explanation,
        // NOT silent zero.
        let mut cov = PeerVerifyCoverage::default();
        cov.accepted_worktx_total = 1;
        cov.accepted_worktx_with_verify = 0;
        cov.coverage_pct = 0;
        cov.peer_verifications_total = 0;
        cov.non_solver_verifications = 0;
        cov.solver_agents.insert(AgentId("Agent_5".into()));
        let rendered = cov.render_section_f_x();
        assert!(rendered.contains("MECHANISM BOTTLENECK"));
        assert!(rendered.contains("round-robin scheduler"));
        assert!(rendered.contains("Pending Peer Reviews"));
        assert!(rendered.contains("G5.1 opportunity scheduler"));
    }
}
