// PPUT-CCL JSONL schema v2 — proposal-level + run-level records.
//
// Authoritative spec:
//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B1
//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 5 (definitions)
//
// Versioning: every v2 record carries `schema_version = "v2.0"`. Legacy Paper-1
// era jsonl rows (the `PputResult` shape emitted by evaluator before this commit)
// have NO `schema_version` field, so `RunRecord::from_json` discriminates on
// presence and routes to `LegacyRunAggregate`. No on-disk artifact is rewritten
// by this commit; downstream tooling is the upgrade boundary.
//
// B1 scope: schema definition + round-trip + legacy-compat + zero-progress
// invariant. B2/B3/B4 wire the new fields into evaluator emission paths.

use serde::{Deserialize, Serialize};

pub const SCHEMA_VERSION_V2: &str = "v2.0";

/// Per-proposal row (one per LLM call / append / complete attempt).
///
/// Currently no evaluator emit path produces these — B2 (cost aggregator) and
/// B3 (wall-time) will add the emit sites. This struct is the contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProposalRow {
    pub run_id: String,
    pub problem_id: String,
    pub agent_id: String,
    pub role: String,
    pub branch_id: String,
    pub proposal_hash: String,
    pub accepted: bool,

    /// "adaptation" | "meta_validation" | "heldout"
    pub split: String,
    pub schema_version: String,
    /// SHA-256 of input prompt (retrieval-equivalence audit).
    pub context_hash: String,
    /// Runtime predicate accept = 1, reject = 0.
    pub predicate_result: i32,
    /// Lean post-hoc verify: 1 / 0 / null = not yet checked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_truth_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lean_error_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_error_hash: Option<String>,
    /// Hash of Q^world snapshot to roll back to (PREREG ArtifactState).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_to: Option<String>,

    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    /// Length of all tool stdout summed (B2).
    pub tool_tokens: u64,
    /// = prompt + completion + tool.
    pub total_tokens: u64,
    pub wall_time_ms: u64,
    /// ISO 8601 UTC.
    pub start_time: String,
    pub end_time: String,
    pub ast_depth: u32,
    pub peer_agents_in_branch: Vec<String>,
    /// SHA-256 of concatenated tool stdout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_stdout_hash: Option<String>,
    pub is_on_golden_path: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub golden_path_id: Option<String>,
    /// Phase D+ meta-loop attribution; nullable in Phase B.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architect_artifact_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditor_attestation: Option<String>,
}

/// Per-run aggregate row.
///
/// `pput_runtime` = legacy / runtime-accept-based — NEVER the North Star.
/// `pput_verified` = Lean post-hoc verified — H-VPPUT input.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunAggregate {
    pub run_id: String,
    pub problem_id: String,
    pub solved: bool,

    pub schema_version: String,
    pub split: String,
    /// Lean post-hoc PASS (B4).
    pub verified: bool,
    pub golden_path_token_count: u64,
    /// C_i — sum over all proposals (B2).
    pub total_run_token_count: u64,
    /// T_i — wall-clock first-read → final-accept (B3).
    pub total_wall_time_ms: u64,
    /// 0 or 1 (Lean ground truth).
    pub progress: u8,
    /// Runtime/accept-based; may inflate under Soft Law (H1 detection).
    pub pput_runtime: f64,
    /// Verified PPUT — Progress / (C_i × T_i / 1000), units = 1/(token·second).
    pub pput_verified: f64,
    /// 10^6 × pput_verified — display unit (PREREG § 5).
    pub pput_m_verified: f64,
    pub failed_branch_count: u32,
    pub rollback_count: u32,

    /// Phase A atom A4: did the run reach `max_transactions` without OMEGA?
    /// True iff the natural max-tx exhaustion path fired. False on OMEGA
    /// accept, on the B7-extra synthetic short-circuit (which exits
    /// EARLY at the rollback threshold — counted under
    /// `synthetic_short_circuit`, not here), and on oneshot (no max-tx
    /// concept; only one LLM call). Co-reported with `solved` so
    /// downstream analysis can split `(solve_rate)` from `(PPUT on solved)`
    /// per Gemini N-agents brainstorm 2026-04-25 § A.4.
    pub hit_max_tx: bool,
    /// Phase A atom A4: distinct-payload-hash / total-proposal ratio
    /// across every parsed `append`/`complete`/`step` payload in the run.
    /// Range [0.0, 1.0]; 1.0 = every proposal unique; 0 proposals → 0.0
    /// by convention (synthetic / measurement-failure runs). Cheap proxy
    /// for the semantic-diversity metric proposed in the N-agents
    /// brainstorms (Gemini § A "Search Party"); embedding distance is
    /// Phase D+ work.
    pub tactic_diversity: f64,
    /// Phase A atom A4: cumulative wall-clock spent inside Lean verifier
    /// calls (`verify_omega` / `verify_omega_detailed` / `verify_partial`)
    /// across the full run, in milliseconds. By construction
    /// `verifier_wait_ms ≤ total_wall_time_ms`. Enables the Amdahl /
    /// USL decomposition Codex N-agents brainstorm § C proposed
    /// (parallel LLM time vs serial Lean time).
    pub verifier_wait_ms: u64,

    /// Phase A atom A5 (FC2-N22 HALT decomposition): which
    /// budget-regime governed the loop bound for this run. Stable
    /// label string (`total_proposal` | `per_agent` | `token_total` |
    /// `wall_clock`). PREREG_AMENDMENT_p0_defer § 3 condition 3 names
    /// this stamp as a re-calibration prerequisite — without it,
    /// `MaxTxExhausted` rows are ambiguous about which budget
    /// partitioning rule produced them. Oneshot runs (no swarm loop)
    /// stamp `total_proposal` + `budget_max_transactions=1`.
    pub budget_regime: String,
    /// Phase A atom A5: base transaction budget BEFORE the regime's
    /// scaling rule was applied. Under `total_proposal` the loop bound
    /// equals this; under `per_agent` the loop bound = base × n_agents.
    /// Stamping the base (not the effective bound) keeps cross-N
    /// comparisons interpretable in downstream analysis.
    pub budget_max_transactions: u32,

    pub far: f64,
    pub err: f64,
    pub iac: f64,
    pub cpr: f64,

    /// Exact model id + API revision (drift defense per F-2026-04-22-08).
    pub model_snapshot: String,
    pub git_sha: String,
    pub binary_sha256: String,
    /// "full" | "panopticon" | "amnesia" | "soft_law" | "homogeneous".
    pub mode: String,
}

impl RunAggregate {
    /// Compute pput_verified per PREREG § 5:
    ///   pput_verified = progress / (c_i * t_i_ms / 1000)
    /// Returns 0.0 when progress is 0, OR when c_i or t_i_ms is 0
    /// (synthetic / degenerate runs; real runs always have positive cost+time).
    pub fn compute_pput_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
        if progress == 0 || c_i == 0 || t_i_ms == 0 {
            return 0.0;
        }
        let denom = (c_i as f64) * (t_i_ms as f64) / 1000.0;
        (progress as f64) / denom
    }

    /// Display unit: 10^6 × pput_verified.
    pub fn compute_pput_m_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
        1.0e6 * Self::compute_pput_verified(progress, c_i, t_i_ms)
    }
}

/// Phase A atom A4 (FC1-N11 ∏p decision diversity): tactic_diversity
/// = distinct / total. 0 proposals → 0.0 by convention (no signal to
/// report). All-distinct → 1.0; all-identical → 1/total.
pub fn compute_tactic_diversity(distinct_proposals: u64, total_proposals: u64) -> f64 {
    if total_proposals == 0 {
        return 0.0;
    }
    let r = (distinct_proposals as f64) / (total_proposals as f64);
    // distinct must not exceed total (caller bug); clamp to [0, 1].
    r.clamp(0.0, 1.0)
}

/// Legacy v1 run row — mirrors the pre-v2 `PputResult` shape emitted by the
/// evaluator before this commit (Paper 1 era, e.g.
/// `discarded_12way_run_2026-04-24/E1v2_Abl_*.jsonl`).
///
/// All v3-era extension fields (reputation_at_end, halt_reason, gp_*) are
/// captured by `extra` so a legacy line round-trips losslessly through
/// serde_json::Value.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LegacyRunAggregate {
    pub problem: String,
    pub condition: String,
    pub model: String,
    pub has_golden_path: bool,
    pub time_secs: f64,
    pub pput: f64,
    pub gp_token_count: u64,
    pub gp_node_count: usize,
    pub tx_count: u64,
    /// Catch-all for v3.x optional fields (reputation_at_end, halt_reason,
    /// gp_payload, gp_path, gp_proof_file, classifier_version, build_sha, ...).
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Discriminated record for backward-compatible reading.
#[derive(Debug)]
pub enum RunRecord {
    V2(RunAggregate),
    Legacy(LegacyRunAggregate),
}

impl RunRecord {
    /// Parse one jsonl line. v2 if `schema_version` present, else legacy.
    /// Returns the raw serde error for genuinely malformed input.
    pub fn from_json(line: &str) -> Result<Self, serde_json::Error> {
        let v: serde_json::Value = serde_json::from_str(line)?;
        let is_v2 = v
            .get("schema_version")
            .and_then(|s| s.as_str())
            .map(|s| s.starts_with("v2"))
            .unwrap_or(false);
        if is_v2 {
            Ok(RunRecord::V2(serde_json::from_value(v)?))
        } else {
            Ok(RunRecord::Legacy(serde_json::from_value(v)?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_run() -> RunAggregate {
        RunAggregate {
            run_id: "r-001".into(),
            problem_id: "mathd_algebra_44".into(),
            solved: true,
            schema_version: SCHEMA_VERSION_V2.into(),
            split: "adaptation".into(),
            verified: true,
            golden_path_token_count: 512,
            total_run_token_count: 4096,
            total_wall_time_ms: 12_000,
            progress: 1,
            pput_runtime: 0.5,
            pput_verified: RunAggregate::compute_pput_verified(1, 4096, 12_000),
            pput_m_verified: RunAggregate::compute_pput_m_verified(1, 4096, 12_000),
            failed_branch_count: 3,
            rollback_count: 0,
            hit_max_tx: false,
            tactic_diversity: 1.0,
            verifier_wait_ms: 4_500,
            budget_regime: "total_proposal".into(),
            budget_max_transactions: 200,
            far: 0.0,
            err: 0.0,
            iac: 0.0,
            cpr: 0.0,
            model_snapshot: "deepseek-v4-flash@2026-04-26".into(),
            git_sha: "913255d".into(),
            binary_sha256: "deadbeef".into(),
            mode: "full".into(),
        }
    }

    #[test]
    fn test_jsonl_schema_v2_round_trip() {
        let original = sample_run();
        let line = serde_json::to_string(&original).expect("serialize");
        let parsed: RunAggregate = serde_json::from_str(&line).expect("deserialize");
        assert_eq!(parsed, original, "v2 RunAggregate must round-trip");
        assert!(
            line.contains("\"schema_version\":\"v2.0\""),
            "serialized line must stamp schema_version"
        );
    }

    #[test]
    fn test_pput_verified_zero_when_progress_zero() {
        // PREREG § 3 anti-Goodhart: a run that did not verify must report
        // pput_verified = 0 regardless of cost / wall-time.
        assert_eq!(RunAggregate::compute_pput_verified(0, 1000, 5000), 0.0);
        assert_eq!(RunAggregate::compute_pput_m_verified(0, 1000, 5000), 0.0);

        // And the struct round-trips with the zero stamped in.
        let mut r = sample_run();
        r.solved = false;
        r.verified = false;
        r.progress = 0;
        r.pput_verified =
            RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
        r.pput_m_verified =
            RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
        assert_eq!(r.pput_verified, 0.0);
        assert_eq!(r.pput_m_verified, 0.0);

        // Defensive: degenerate cost/time also clamps to 0 (synthetic test fixtures).
        assert_eq!(RunAggregate::compute_pput_verified(1, 0, 5000), 0.0);
        assert_eq!(RunAggregate::compute_pput_verified(1, 1000, 0), 0.0);
    }

    #[test]
    fn test_legacy_jsonl_still_readable() {
        // Verbatim shape of a Paper-1 era line
        // (discarded_12way_run_2026-04-24/E1v2_Abl_s141421_n8_20260424T080939.jsonl).
        let legacy_line = r#"{"problem":"/tmp/foo.lean","condition":"n8","model":"deepseek-chat","has_golden_path":true,"time_secs":781.99,"pput":0.127,"gp_token_count":769,"gp_node_count":7,"tx_count":16,"build_sha":"61ccc21","classifier_version":"v1_2026-04-16-a","boltzmann_seed":141421,"halt_reason":"OmegaAccepted","reputation_at_end":{"Agent_1":2}}"#;

        match RunRecord::from_json(legacy_line).expect("legacy line parses") {
            RunRecord::Legacy(l) => {
                assert_eq!(l.condition, "n8");
                assert_eq!(l.has_golden_path, true);
                assert_eq!(l.gp_token_count, 769);
                // v3.x extension fields land in `extra`.
                assert_eq!(
                    l.extra.get("halt_reason").and_then(|v| v.as_str()),
                    Some("OmegaAccepted")
                );
                assert!(l.extra.get("reputation_at_end").is_some());
            }
            RunRecord::V2(_) => panic!("legacy line misclassified as v2"),
        }

        // And a v2 line dispatches the other way.
        let v2_line = serde_json::to_string(&sample_run()).unwrap();
        match RunRecord::from_json(&v2_line).expect("v2 line parses") {
            RunRecord::V2(_) => {}
            RunRecord::Legacy(_) => panic!("v2 line misclassified as legacy"),
        }
    }

    // Phase A atom A4: decomposed metrics (Codex / Gemini N-agents brainstorm).
    // The 3 new fields (hit_max_tx, tactic_diversity, verifier_wait_ms) must
    // round-trip and obey their invariants; every emitted v2 row carries them
    // (they are non-Optional in the schema).

    #[test]
    fn test_a4_decomposed_metrics_round_trip() {
        let mut r = sample_run();
        r.hit_max_tx = true;
        r.tactic_diversity = 0.42;
        r.verifier_wait_ms = 1234;
        let line = serde_json::to_string(&r).unwrap();
        assert!(line.contains("\"hit_max_tx\":true"));
        assert!(line.contains("\"tactic_diversity\":0.42"));
        assert!(line.contains("\"verifier_wait_ms\":1234"));
        let parsed: RunAggregate = serde_json::from_str(&line).unwrap();
        assert_eq!(parsed, r);
    }

    #[test]
    fn test_a4_tactic_diversity_helper() {
        // All-distinct → 1.0
        assert_eq!(compute_tactic_diversity(8, 8), 1.0);
        // All-identical → 1/N
        assert!((compute_tactic_diversity(1, 8) - 0.125).abs() < 1e-12);
        // 0 proposals → 0.0 (no signal)
        assert_eq!(compute_tactic_diversity(0, 0), 0.0);
        assert_eq!(compute_tactic_diversity(0, 5), 0.0);
        // Caller bug (distinct > total) clamps to 1.0, never panics — keeps
        // emit path crash-free under accumulator wiring regression.
        assert_eq!(compute_tactic_diversity(9, 8), 1.0);
    }

    #[test]
    fn test_a5_budget_regime_round_trip() {
        // Phase A atom A5: every emitted v2 row must carry the budget
        // regime + base. The stable string labels and the u32 base
        // both serialize/deserialize cleanly, including the
        // non-default `per_agent` regime that scales with N.
        let mut r = sample_run();
        r.budget_regime = "per_agent".into();
        r.budget_max_transactions = 50;
        let line = serde_json::to_string(&r).unwrap();
        assert!(line.contains("\"budget_regime\":\"per_agent\""));
        assert!(line.contains("\"budget_max_transactions\":50"));
        let parsed: RunAggregate = serde_json::from_str(&line).unwrap();
        assert_eq!(parsed, r);
    }

    #[test]
    fn test_a4_verifier_wait_bounded_by_total_wall_time() {
        // Invariant required at every emit site: verifier wait is a strict
        // sub-interval of total wall time. Test the contract; emit-site
        // wiring is asserted in the conformance battery.
        let r = sample_run();
        assert!(
            r.verifier_wait_ms <= r.total_wall_time_ms,
            "verifier_wait_ms ({}) must be <= total_wall_time_ms ({})",
            r.verifier_wait_ms,
            r.total_wall_time_ms
        );
    }
}
