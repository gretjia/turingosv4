//! TB-G G2P.2 — Peer-verify-coverage walker + §F.X dashboard (Class 2).
//!
//! Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
//! §1 Module G2P atom G2P.2.
//! G-Phase directive §0.6 amendment G-2 verbatim "verify_peer=0 比 invest=0
//! 更危险" + §8.2 Peer Verification Bridge (architect ship gate: "at least
//! one non-solver VerifyTx on another agent's WorkTx").
//!
//! Quantifies user 2026-05-12 病灶3 "0 verify" in the post-batch dashboard
//! so silent-zero outcomes surface explicit mechanism-bottleneck
//! explanations per architect §8.5 + `CROSS_PROBLEM_PERSISTENCE_REPORT.md`
//! §4 Q6.6 (≥3 candidate causes required when coverage = 0).
//!
//! Ship gates:
//! - SG-G2P.3 — walker emits per-agent `peer_verify_count` from canonical
//!   L4 + CAS evidence (no private CoT / telemetry reads).
//! - SG-G2P.4 — `audit_dashboard --run-report` §F.X renders
//!   `coverage_pct`, `peer_verifications_total`, `non_solver_verifications`,
//!   and the per-agent breakdown. Wired through
//!   `render_tb_n3_run_report`.
//! - SG-G2P.5 — when `non_solver_verifications == 0` the rendered §F.X
//!   block includes an EXPLICIT MECHANISM BOTTLENECK explanation with
//!   ≥3 candidate causes (silent-zero is forbidden per
//!   `feedback_no_workarounds_strict_constitution`); when ≥1 non-solver
//!   VerifyTx is observed the bottleneck block is OMITTED.
//!
//! `FC-trace: FC1-N7 δ Agent externalized output enriched with
//! peer-verify coverage view + §15 selective shielding (chain-derived
//! public state only) + §17 reporting standard (mechanism bottleneck must
//! be explicit, never silent zero).`

use turingosv4::runtime::peer_verify_coverage::{compute_peer_verify_coverage, PeerVerifyCoverage};
use turingosv4::state::q_state::{AgentId, TxId};

// ────────────────────────────────────────────────────────────────────────
// SG-G2P.3 — walker emits per-agent peer_verify_count
// ────────────────────────────────────────────────────────────────────────

const PEER_VERIFY_COVERAGE_SRC: &str = "src/runtime/peer_verify_coverage.rs";

/// SG-G2P.3.a — walker is reachable as a public symbol on the
/// `runtime::peer_verify_coverage` namespace, accepts a generic
/// `LedgerWriter` (trait-object) + `CasStore` so production AND tests
/// can exercise it without forcing a real git repo.
#[test]
fn sg_g2p_3_walker_is_public_and_accepts_trait_object_writer() {
    let src = std::fs::read_to_string(PEER_VERIFY_COVERAGE_SRC)
        .expect("peer_verify_coverage.rs readable");
    assert!(
        src.contains("pub fn compute_peer_verify_coverage(")
            && (src.contains("writer: &dyn LedgerWriter")
                || src.contains("writer:&dyn LedgerWriter")),
        "SG-G2P.3.a: compute_peer_verify_coverage must be pub AND accept a \
         trait-object LedgerWriter so tests can exercise the walker against \
         an InMemoryLedgerWriter without standing up a real git repo."
    );
    assert!(
        src.contains("pub fn compute_peer_verify_coverage_from_paths("),
        "SG-G2P.3.a: a path-based wrapper must exist so audit_dashboard \
         can wire the walker via Git2LedgerWriter::open."
    );
}

/// SG-G2P.3.b — walker output exposes the per-agent peer_verify_count
/// map (architect §8.2 ship-gate axis).
#[test]
fn sg_g2p_3_walker_output_exposes_per_agent_peer_verify_count() {
    let src = std::fs::read_to_string(PEER_VERIFY_COVERAGE_SRC)
        .expect("peer_verify_coverage.rs readable");
    assert!(
        src.contains("pub per_verifier_count:"),
        "SG-G2P.3.b: PeerVerifyCoverage must expose `per_verifier_count` \
         (BTreeMap<AgentId, u64>) so the dashboard can render the per-agent \
         breakdown."
    );
    assert!(
        src.contains("pub non_solver_verifications:"),
        "SG-G2P.3.b: PeerVerifyCoverage must expose `non_solver_verifications` \
         — the architect §8.2 ship-gate signal."
    );
    assert!(
        src.contains("pub solver_agents:"),
        "SG-G2P.3.b: PeerVerifyCoverage must expose `solver_agents` so \
         non_solver classification is reproducible from the struct alone."
    );
}

/// SG-G2P.3.c — shielding gate: walker source must NOT reference any
/// private surface (attempt telemetry, prompt capsules, proof artifacts,
/// raw Lean stderr, private diagnostic CIDs). Mirrors the
/// `tests/constitution_shielding_gate.rs` pattern.
#[test]
fn sg_g2p_3_walker_does_not_reference_private_surfaces() {
    let src = std::fs::read_to_string(PEER_VERIFY_COVERAGE_SRC)
        .expect("peer_verify_coverage.rs readable");
    // Scan only the production (non-test) section. By Rust convention the
    // `#[cfg(test)] mod tests {` block lives at the bottom; test fixtures
    // legitimately mention `WorkTx { proposal_cid: ... }` to construct
    // test values, but that mention is structural, not a walker read of
    // private CAS metadata. The shielding gate enforces the walker's
    // production code only references chain-derived public state.
    let production_src: &str = src
        .split_once("#[cfg(test)]")
        .map(|(prod, _)| prod)
        .unwrap_or(&src);
    let code: String = production_src
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
    ];
    for needle in &forbidden {
        assert!(
            !code.contains(needle),
            "SG-G2P.3.c shielding violation: walker code references forbidden \
             private surface `{needle}` — would couple the public-state \
             walker to private/audit-only payloads per CLAUDE.md §15."
        );
    }
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2P.4 — §F.X renders coverage % (audit_dashboard binding)
// ────────────────────────────────────────────────────────────────────────

const AUDIT_DASHBOARD_SRC: &str = "src/bin/audit_dashboard.rs";

/// SG-G2P.4.a — `audit_dashboard --run-report` source wires the
/// `peer_verify_coverage` walker into `render_tb_n3_run_report`.
#[test]
fn sg_g2p_4_audit_dashboard_wires_peer_verify_coverage_walker() {
    let src = std::fs::read_to_string(AUDIT_DASHBOARD_SRC).expect("audit_dashboard.rs readable");
    assert!(
        src.contains("peer_verify_coverage::compute_peer_verify_coverage_from_paths"),
        "SG-G2P.4.a: audit_dashboard.rs must call \
         `peer_verify_coverage::compute_peer_verify_coverage_from_paths` so \
         §F.X is wired from the canonical L4 + CAS evidence walker."
    );
    assert!(
        src.contains("render_section_f_x"),
        "SG-G2P.4.a: audit_dashboard.rs must call `render_section_f_x` on \
         the walker output so coverage / per-agent breakdown / bottleneck \
         explanation surface in the run report."
    );
}

/// SG-G2P.4.b — fixture renders: positive control. A walker output with
/// non-zero coverage produces a §F.X block containing the canonical
/// `coverage_pct: N%` line + per-agent breakdown rows.
#[test]
fn sg_g2p_4_fixture_renders_coverage_pct_line_and_per_agent_rows() {
    let mut cov = PeerVerifyCoverage::default();
    cov.accepted_worktx_total = 4;
    cov.accepted_worktx_with_verify = 3;
    cov.coverage_pct = 75;
    cov.peer_verifications_total = 3;
    cov.non_solver_verifications = 2;
    cov.per_verifier_count.insert(AgentId("Agent_0".into()), 2);
    cov.per_verifier_count.insert(AgentId("Agent_5".into()), 1);
    cov.per_target_count.insert(TxId("worktx-1".into()), 1);
    cov.solver_agents.insert(AgentId("Agent_5".into()));
    let block = cov.render_section_f_x();
    assert!(
        block.contains("## §F.X Peer-verify coverage"),
        "SG-G2P.4.b: canonical heading missing; block:\n{block}"
    );
    assert!(
        block.contains("coverage_pct: 75%"),
        "SG-G2P.4.b: coverage_pct row missing; block:\n{block}"
    );
    assert!(
        block.contains("peer_verifications_total: 3"),
        "SG-G2P.4.b: peer_verifications_total row missing; block:\n{block}"
    );
    assert!(
        block.contains("non_solver_verifications: 2"),
        "SG-G2P.4.b: non_solver_verifications row missing; block:\n{block}"
    );
    assert!(
        block.contains("Agent_0 (non_solver): 2"),
        "SG-G2P.4.b: per-agent row for Agent_0 (non_solver) missing; block:\n{block}"
    );
    assert!(
        block.contains("Agent_5 (solver): 1"),
        "SG-G2P.4.b: per-agent row for Agent_5 (solver) missing; block:\n{block}"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2P.5 — persistent-batch ≥1 non-solver VerifyTx OR explicit bottleneck
// ────────────────────────────────────────────────────────────────────────

/// SG-G2P.5.a — silent-zero is forbidden. When the walker reports
/// `non_solver_verifications == 0` the rendered §F.X block MUST emit an
/// explicit MECHANISM BOTTLENECK explanation with at least 3 candidate
/// causes per `CROSS_PROBLEM_PERSISTENCE_REPORT.md` §4 Q6.6 + architect
/// §8.5 "empty market as valid empirical result".
#[test]
fn sg_g2p_5_zero_non_solver_emits_bottleneck_with_three_candidate_causes() {
    let mut cov = PeerVerifyCoverage::default();
    cov.accepted_worktx_total = 1;
    cov.solver_agents.insert(AgentId("Agent_5".into()));
    // non_solver_verifications = 0 by default; coverage_pct = 0.
    let block = cov.render_section_f_x();
    assert!(
        block.contains("MECHANISM BOTTLENECK"),
        "SG-G2P.5.a: silent-zero violation — bottleneck block absent when \
         non_solver_verifications == 0. block:\n{block}"
    );
    // Architect §8.5 / CROSS_PROBLEM_PERSISTENCE_REPORT §4 Q6.6 calls for
    // ≥3 candidate causes. Verify by counting numbered cause anchors.
    let cause_anchors = ["1.", "2.", "3."];
    for anchor in &cause_anchors {
        assert!(
            block.contains(anchor),
            "SG-G2P.5.a: bottleneck block must enumerate ≥3 candidate causes; \
             anchor `{anchor}` missing from block:\n{block}"
        );
    }
    // The three architect-named root causes should all be referenced.
    assert!(
        block.contains("round-robin scheduler") || block.contains("opportunity scheduler"),
        "SG-G2P.5.a: bottleneck block must name scheduler cause (amendment G-4)"
    );
    assert!(
        block.contains("Pending Peer Reviews"),
        "SG-G2P.5.a: bottleneck block must name the prompt-block cause (G2P.1)"
    );
}

/// SG-G2P.5.b — positive control: when `non_solver_verifications > 0`
/// the bottleneck block is OMITTED so the ship-gate signal is positive
/// rather than diagnostic noise.
#[test]
fn sg_g2p_5_positive_non_solver_count_omits_bottleneck() {
    let mut cov = PeerVerifyCoverage::default();
    cov.accepted_worktx_total = 1;
    cov.accepted_worktx_with_verify = 1;
    cov.coverage_pct = 100;
    cov.peer_verifications_total = 1;
    cov.non_solver_verifications = 1;
    cov.per_verifier_count.insert(AgentId("Agent_0".into()), 1);
    cov.solver_agents.insert(AgentId("Agent_5".into()));
    let block = cov.render_section_f_x();
    assert!(
        !block.contains("MECHANISM BOTTLENECK"),
        "SG-G2P.5.b: non_solver_verifications > 0 must omit the bottleneck \
         block (architect ship gate green). block:\n{block}"
    );
}

/// SG-G2P.5.c — defensive: empty chain still renders the §F.X heading
/// + explicit bottleneck (zero coverage is the most acute version of
/// the verify=0 surface — never let the dashboard show "blank §F.X").
#[test]
fn sg_g2p_5_empty_chain_still_renders_explicit_bottleneck() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let cas = turingosv4::bottom_white::cas::store::CasStore::open(tmp.path()).expect("cas open");
    let writer = turingosv4::bottom_white::ledger::transition_ledger::InMemoryLedgerWriter::new();
    let cov = compute_peer_verify_coverage(&writer, &cas).expect("walker");
    assert_eq!(cov.peer_verifications_total, 0);
    assert_eq!(cov.non_solver_verifications, 0);
    let block = cov.render_section_f_x();
    assert!(block.contains("## §F.X Peer-verify coverage"));
    assert!(
        block.contains("MECHANISM BOTTLENECK"),
        "SG-G2P.5.c: empty-chain dashboard must still emit explicit \
         mechanism-bottleneck explanation. block:\n{block}"
    );
}
