// MiniF2F v4 Evaluator — oneshot and swarm modes
//
// Sole optimization metric: PPUT (Progress Per Unit Time)
//   Progress = 100% if Golden Path exists (OMEGA reached), 0% otherwise
//   PPUT = 100% / time_to_omega (seconds)
//   No GP → PPUT = 0 → problem not worth attacking in current iteration
//
// Constitutional basis: Art. I.1 (boolean predicate), Art. I.2 (statistical signal = PPUT)

use minif2f_v4::lean4_oracle::{Lean4Oracle, PartialVerdict, derive_lean_path, load_problem};
use minif2f_v4::cost_aggregator::RunCostAccumulator;
use minif2f_v4::wall_clock::RunWallClock;
use minif2f_v4::post_hoc_verifier::{
    compute_progress_runtime, compute_progress_verified, compute_pput, compute_pput_m,
};
use turingosv4::bus::{BusConfig, BusResult, TuringBus};
use turingosv4::sdk::error_abstraction::{classify_lean_error, classify_parse_error, CLASSIFIER_VERSION};
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::kernel::Kernel;
use turingosv4::sdk::actor::{BoltzmannParams, boltzmann_select_parent};
use turingosv4::sdk::prompt::build_agent_prompt;
use turingosv4::sdk::prompt_guard::assert_no_metric_leak;
use turingosv4::sdk::protocol::parse_agent_output;
use turingosv4::sdk::tools::wallet::WalletTool;
use turingosv4::sdk::tools::search::SearchTool;
use turingosv4::sdk::tools::librarian::LibrarianTool;

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::time::Instant;
use log::{info, warn, error};
use rand::SeedableRng;
use rand::rngs::StdRng;

const DEFAULT_BOLTZMANN_SEED: u64 = 74677;  // same as sample seed (BTC/USD external)

const DEFAULT_MINIF2F_DIR: &str = "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4";

/// PPUT result for a single problem — the only output that matters.
///
/// Mid-term audit P0-B fix 2026-04-25: this struct now carries every B1
/// `RunAggregate` v2 field as a non-Optional, so emitted jsonl rows are
/// dispatched as `RunRecord::V2` by `RunRecord::from_json` (presence of
/// `schema_version` is the discriminant). Legacy diagnostic fields below
/// are kept as Option/skip-if-None for downstream tooling that already
/// reads them; serde silently drops them when parsing as `RunAggregate`
/// (no `deny_unknown_fields`), so V2-tooling reads the v2 contract while
/// PputResult-tooling sees the full diagnostic envelope.
#[derive(Debug, serde::Serialize)]
struct PputResult {
    // ── B1 RunAggregate v2 schema fields (all REQUIRED — non-Optional) ──
    /// Always "v2.0" — RunRecord::from_json discriminator.
    schema_version: String,
    /// Per-run identifier: condition + problem + timestamp.
    run_id: String,
    /// Problem identifier: theorem stem (basename of .lean without extension).
    problem_id: String,
    /// Legacy "did the run reach OMEGA" boolean (= runtime_accepted in B4 vocab).
    /// B1 v2 mandates this as `solved: bool`.
    solved: bool,
    /// "adaptation" | "meta_validation" | "heldout" — read from SPLIT env;
    /// default "adaptation" with stderr warning per Phase B convention.
    split: String,
    /// B4 dual-PPUT: post-hoc Lean verified result. Phase B == solved.
    verified: bool,
    /// Token count of the winning golden path (0 if no GP).
    golden_path_token_count: u64,
    /// B2 C_i — full-run token cost across all proposals.
    total_run_token_count: u64,
    /// B3 T_i — first agent prompt → final Lean call, in milliseconds.
    total_wall_time_ms: u64,
    /// 0 or 1 — Lean ground truth (= 1 iff runtime_accepted AND post_hoc_verified).
    progress: u8,
    /// B4 dual-PPUT: pput_runtime = progress_runtime / (C_i × T_i / 1000).
    pput_runtime: f64,
    /// B4 dual-PPUT: pput_verified = progress_verified / (C_i × T_i / 1000).
    pput_verified: f64,
    /// 10^6 × pput_verified — display unit per PREREG § 5.
    pput_m_verified: f64,
    /// B2 C_i sub-counter: count of proposals that did NOT verify.
    failed_branch_count: u32,
    /// Phase B always 0; Phase C+ when ArtifactState rollbacks land.
    rollback_count: u32,
    /// Phase A atom A4 (FC2-N22 HALT decomposition): true iff the run
    /// reached `max_transactions` without OMEGA. Distinguishes a real
    /// budget-exhausted run from an OMEGA-accept exit at the same
    /// `tx_count`. False on B7-extra synthetic short-circuit (which
    /// exits EARLY at the rollback threshold; that path is tagged via
    /// `synthetic_short_circuit` instead). False on oneshot (no max-tx
    /// concept). Co-reported with `solved` so analysis can split
    /// `(solve_rate)` from `(PPUT on solved)` per Gemini brainstorm.
    hit_max_tx: bool,
    /// Phase A atom A4 (FC1-N11 ∏p decision diversity): distinct /
    /// total over every parsed proposal payload (append/complete/step)
    /// in the run. 0 proposals → 0.0 by convention.
    tactic_diversity: f64,
    /// Phase A atom A4 (FC1-N12 oracle scope): cumulative wall-clock
    /// inside Lean verifier calls in milliseconds. Strict sub-interval
    /// of `total_wall_time_ms`. Enables Amdahl/USL serial-vs-parallel
    /// decomposition per Codex brainstorm § C.
    verifier_wait_ms: u64,
    /// Phase A atom A5 (FC2-N22 HALT decomposition): label of the
    /// budget regime that governed this run's loop bound. One of
    /// `total_proposal` | `per_agent` | `token_total` | `wall_clock`
    /// (the latter two declared but startup-fatal in Phase A). Required
    /// by PREREG_AMENDMENT_p0_defer § 3 condition 3 to disambiguate
    /// `MaxTxExhausted` rows across N values.
    budget_regime: String,
    /// Phase A atom A5: base transaction budget BEFORE regime scaling.
    /// Under `total_proposal` the effective loop bound = this value;
    /// under `per_agent` = this value × n_agents. Oneshot stamps 1
    /// (single LLM call, no loop concept).
    budget_max_transactions: u32,
    /// FAR guardrail (Phase B not yet computed; emit 0.0 placeholder).
    far: f64,
    /// ERR guardrail (Phase B not yet computed).
    err: f64,
    /// IAC guardrail (Phase B not yet computed).
    iac: f64,
    /// CPR guardrail (Phase B not yet computed).
    cpr: f64,
    /// Exact model id + API revision (drift defense per F-2026-04-22-08).
    model_snapshot: String,
    /// Trust Root provenance — git commit SHA at boot.
    git_sha: String,
    /// Trust Root binary fingerprint — Phase B placeholder; B7 fills.
    binary_sha256: String,
    /// "full" | "panopticon" | "amnesia" | "soft_law" | "homogeneous" — from
    /// MODE env, default "full" Phase B.
    mode: String,

    // ── Legacy diagnostic fields (preserved for downstream tooling) ──
    problem: String,
    condition: String,
    model: String,
    has_golden_path: bool,         // alias of `solved`; legacy field name
    time_secs: f64,                // wall time elapsed (function-entry bracket; legacy)
    pput: f64,                     // 100/time if GP, 0 otherwise (legacy display)
    gp_token_count: u64,           // alias of golden_path_token_count
    gp_node_count: usize,          // nodes on golden path (0 if no GP)
    tx_count: u64,                 // total transactions attempted
    // C-012 provenance: stamp per-row commit SHA + classifier version + RNG seed.
    // All Optional; serialize-skip when None (backward compat with v3.1/v3.2 artifacts).
    #[serde(skip_serializing_if = "Option::is_none")]
    build_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classifier_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boltzmann_seed: Option<u64>,
    // C-036 harness telemetry: bypass-detection signals for multi-agent runs.
    // tool_dist: counts per tool ({complete, append, invest, parse_fail, llm_err}).
    //   complete=N append=0 ⇒ tape-bypass (Art. II.1 broadcast unused).
    // unique_payload_ratio: distinct OMEGA payloads / total OMEGA attempts.
    //   <0.30 ⇒ catastrophic agent correlation (F-2026-04-18-01).
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_dist: Option<HashMap<String, u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_payload_ratio: Option<f64>,
    // Phase 0 (C-039 candidate): persisted full proof + path so external verifiers can
    // re-run `lean --stdin` from disk artifacts alone, without trusting in-memory runtime.
    // gp_payload = the exact text fed to oracle.verify_omega_detailed at OMEGA accept.
    // gp_path = "alone" (payload self-contained) or "tape+payload" (Art. IV dual-path 2).
    // gp_proof_file = relative path to the standalone .lean archive (problem + proof).
    #[serde(skip_serializing_if = "Option::is_none")]
    gp_payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gp_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gp_proof_file: Option<String>,
    /// PPUT-CCL B7-extra (PREREG § 5.5 calibration treatment): set to
    /// `Some(true)` iff the synthetic rollback short-circuit fired in
    /// this run — i.e. SIMULATE_ROLLBACK_AT_TX_50=1 AND the run reached
    /// `rollback_sim::ROLLBACK_TX_THRESHOLD`. Distinguishes calibration
    /// treatment exits from natural max-tx exhaustions (both stamp the
    /// same legacy halt path; this field is the disambiguator).
    ///
    /// Crucially: when `synthetic_short_circuit == Some(true)`, the run's
    /// `total_run_token_count` (C_i) is **understated** vs a true 150-tx
    /// vetoed loop, because the LLM calls for tx 51-199 never happened.
    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
    /// p_0 estimation is unaffected; downstream PPUT analysis on these
    /// rows MUST honor this flag and exclude or specially treat them.
    #[serde(skip_serializing_if = "Option::is_none")]
    synthetic_short_circuit: Option<bool>,
    // Note (mid-term audit P0-B fix 2026-04-25): the prior Option versions of
    // total_run_token_count / failed_branch_count / total_wall_time_ms /
    // verified / pput_runtime / pput_verified / pput_m_verified were promoted
    // to non-Optional v2 fields above. Phase B always has values for them.
}

#[tokio::main]
async fn main() {
    env_logger::init();

    // Audit-fix 2026-04-25 (Codex B1 + Q2 — both auditors flagged): the
    // production batch runs *this* binary, not `src/main.rs`. Without a
    // verify_trust_root call here, the FC3-S3 readonly subgraph + FC2-N16
    // InitAI Trust Root enforcement does NOT actually fire on the calibration
    // batch. Boot must happen here, at the production entry point, before
    // any LLM call or jsonl emit.
    //
    // Repo root: CARGO_MANIFEST_DIR is `experiments/minif2f_v4`; repo root
    // is two levels up. canonicalize so a deployed binary still resolves
    // the genesis path it was built against.
    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("evaluator: repo root resolves at build time");
    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
        // FC3-E14 immediate-abort variant. See OBS_BOOT_FAIL_NOT_HALT.
        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
    }

    // Step-B v3 treatment binary: stamp classifier version in every emitted PputResult.
    // Control binary (main branch) has no such set_var → classifier_version serializes as None.
    // This makes it impossible to mistake one binary for the other in post-hoc analysis.
    std::env::set_var("CLASSIFIER_VERSION", CLASSIFIER_VERSION);

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: evaluator <problem_file.lean>");
        eprintln!("  CONDITION env: oneshot|n1|n3 (default: oneshot)");
        eprintln!("  MINIF2F_DIR, LLM_PROXY_URL, ACTIVE_MODEL env vars");
        std::process::exit(1);
    }

    let problem_file = &args[1];
    let condition = std::env::var("CONDITION").unwrap_or_else(|_| "oneshot".into());
    let minif2f_dir = std::env::var("MINIF2F_DIR").unwrap_or_else(|_| DEFAULT_MINIF2F_DIR.into());
    let proxy_url = std::env::var("LLM_PROXY_URL").unwrap_or_else(|_| "http://localhost:8080".into());
    // A0e-fix 2026-04-25 (Codex finding 3 + R-019): canonical name per
    // PREREG § 1.8. Was "deepseek-reasoner" (deprecated alias). Phase B+C
    // pinned model = deepseek-v4-flash thinking-off backend.
    // FC-trace: FC1-N7 (δ/AI canonical identity) + memory project_deepseek_drift_2026-04-24.
    let model = std::env::var("ACTIVE_MODEL").unwrap_or_else(|_| "deepseek-v4-flash".into());

    // Resolve problem path
    let problem_path = resolve_problem_path(problem_file, &minif2f_dir);
    let (problem_statement, theorem_name) = match load_problem(&problem_path) {
        Ok(v) => v,
        Err(e) => { eprintln!("Failed to load: {}", e); std::process::exit(1); }
    };

    let lean_path = derive_lean_path(&minif2f_dir);
    info!("Problem: {} | Condition: {} | Model: {}", problem_file, condition, model);

    let result = match condition.as_str() {
        "oneshot" => {
            run_oneshot(problem_file, &problem_statement, &theorem_name,
                       &lean_path, &proxy_url, &model).await
        }
        // Generic nN: parse any "n<digits>" → run_swarm with N agents.
        // Supports N-scaling experiment (percolation curve mapping).
        // **swarm_N=1** (CONDITION=n1) is the critical baseline for the
        // 2026-04-25 N-experiments arc: same code path as n3/n8 swarm
        // but with a single agent. NOT the same as `oneshot` (which
        // skips the swarm loop, tape, mr ticks, ∏p product, etc.).
        // Per Plan-agent NEXT-1 / Codex E0 / Gemini E1-Prime: every
        // N-curve experiment MUST use n1 (not oneshot) as the N=1
        // baseline to avoid code-path confound. Validated by unit
        // test below: parse_swarm_condition_n("n1") == Some(1).
        c if parse_swarm_condition_n(c).is_some() => {
            let n = parse_swarm_condition_n(c).unwrap();
            run_swarm(problem_file, &problem_statement, &theorem_name,
                     &lean_path, &proxy_url, &model, n).await
        }
        "hybrid_v1" => {
            // Mid-term audit P0-D fix 2026-04-25: hybrid_v1 was a Paper 1 era
            // condition that ran run_oneshot, then on failure ran run_swarm,
            // and merged via `..r2` field-spread. Codex flagged that the spread
            // dropped the failed oneshot's C_i (failed_branch_count and
            // total_run_token_count from r1 were silently discarded). PPUT-CCL
            // arc does NOT use hybrid_v1 — it operates exclusively on `oneshot`
            // and `n<N>` conditions per PREREG. Disabling here forces any
            // pipeline that ships a stale hybrid_v1 invocation to surface the
            // deprecation immediately rather than emit a corrupt C_i.
            eprintln!("hybrid_v1 condition is deprecated for PPUT-CCL arc and was \
                       disabled in mid-term audit P0-D fix 2026-04-25. The prior \
                       implementation dropped the failed oneshot leg's C_i via a \
                       `..r2` field-spread, corrupting full-run cost accounting. \
                       Use `oneshot` or `n<N>` instead.");
            std::process::exit(1);
        }
        other => { eprintln!("Unknown condition: {}", other); std::process::exit(1); }
    };

    // Output PPUT result as JSON (machine-readable for batch runner)
    let json = serde_json::to_string(&result).unwrap();
    println!("PPUT_RESULT:{}", json);

    if result.has_golden_path {
        info!("PPUT = {:.2}%/s (GP: {} nodes, {} tokens, {:.1}s)",
              result.pput, result.gp_node_count, result.gp_token_count, result.time_secs);
    } else {
        info!("PPUT = 0 (no golden path in {:.1}s, {} tx)", result.time_secs, result.tx_count);
    }
}

fn resolve_problem_path(problem_file: &str, minif2f_dir: &str) -> String {
    if PathBuf::from(problem_file).exists() {
        return problem_file.to_string();
    }
    let test_path = format!("{}/MiniF2F/Test/{}", minif2f_dir, problem_file);
    if PathBuf::from(&test_path).exists() { return test_path; }
    let valid_path = format!("{}/MiniF2F/Valid/{}", minif2f_dir, problem_file);
    if PathBuf::from(&valid_path).exists() { return valid_path; }
    eprintln!("Problem file not found: {}", problem_file);
    std::process::exit(1);
}

/// Oneshot: single LLM call → verify → PPUT.
async fn run_oneshot(
    problem_file: &str, problem_statement: &str, theorem_name: &str,
    lean_path: &str, proxy_url: &str, model: &str,
) -> PputResult {
    let start = Instant::now();
    let mut acc = RunCostAccumulator::new();
    let mut wc = RunWallClock::new();
    // Phase A atom A4 (FC1-N12 oracle scope): cumulative wall-clock
    // inside Lean for this oneshot run. A single verify_omega call,
    // but bracket so future Phase C Soft Law mode that double-verifies
    // accumulates correctly.
    let mut verifier_wait_ms: u64 = 0;
    // Phase A atom A5 (FC2-N22 budget regime stamp): oneshot has no
    // transaction loop — it issues exactly one LLM call and returns.
    // Stamp `total_proposal` + base=1 so downstream PPUT analysis can
    // join oneshot rows on the same regime axis as swarm rows without
    // a special case. The regime is informational here; no scaling.
    let oneshot_regime = minif2f_v4::budget_regime::BudgetRegime::TotalProposal;
    let oneshot_budget_base: u32 = 1;

    // A8e fix F1 (Codex#2 + Gemini Q4): one run_id minted at function
    // entry, passed to both fc_event!s and make_pput. Eliminates the
    // millisecond drift between `run_corr_id` (was generated here) and
    // make_pput's internal recomputation. Phase D consumers can now
    // join FC events to v2 jsonl rows by `run_id` equality.
    let run_id = minif2f_v4::run_id::mint_run_id("oneshot", problem_file);

    let oracle = Lean4Oracle::new(
        problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
    );

    // PPUT-CCL B3 (mid-term audit P0-C fix 2026-04-25): open the wall-clock
    // bracket BEFORE prompt construction. PREREG § 5 / plan B3 define T_i
    // as "first agent prompt construction → final Lean call". Marking after
    // the construction (prior wiring) under-counted prompt-build time and
    // forced the conformance test to relax its 7100ms assertion.
    wc.mark_first_read();

    // R-22 v2 clause 4 stays reject-only; the prompt must prevent fences at the source.
    // Chat models (deepseek-chat, 2026-04-22) default to ```lean fences; verifier hard-rejects
    // any response containing ``` so the instruction must be explicit. See F-2026-04-22-08.
    let prompt = format!(
        "Complete the following Lean 4 proof. Output ONLY the tactic proof body as raw Lean \
         tokens. DO NOT wrap in markdown code fences (no ```). No prose, no backticks.\n\n{}",
        problem_statement
    );

    let client = ResilientLLMClient::new(proxy_url, 1800, 2);
    // Model-aware max_tokens: deepseek-chat caps at 8192; reasoner needs 16000 for thinking.
    let max_toks = if model.contains("chat") { 8000 } else { 16000 };
    let request = GenerateRequest {
        model: model.to_string(),
        messages: vec![Message { role: "user".into(), content: prompt }],
        temperature: Some(0.2),
        max_tokens: Some(max_toks),
    };

    // PPUT-CCL B6 runtime gate: scan the assembled prompt for PPUT scalars
    // before the call goes out. Any leak aborts deterministically — Goodhart
    // shield at the LLM-call boundary.
    assert_no_metric_leak(&request.messages[0].content);
    match client.generate(&request).await {
        Ok(response) => {
            acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
            acc.record_proposal(false);
            // Rule 22 v2 clause 4: reject markdown fences
            if response.content.contains("```") {
                wc.mark_final_accept();
                // P0-A: caller declares both runtime + post-hoc legs.
                // Fence reject = neither leg fired.
                // A4: no Lean call reached → verifier_wait_ms=0;
                // 1 proposal made (the LLM response), 1 distinct.
                return make_pput(problem_file, "oneshot", model,
                                 false, false, start, 0, 0, 1,
                                 None, None, None, None, None,
                                 Some(acc.total_run_token_count()),
                                 Some(acc.failed_branch_count),
                                 wc.elapsed_ms(),
                                 false, 1, 1, verifier_wait_ms,
                                 oneshot_regime, oneshot_budget_base, &run_id);
            }

            // Phase A atom A4 (FC1-N12): bracket every Lean call so verifier
            // wait is observable in the emitted v2 row.
            let v_t0 = Instant::now();
            let verdict = oracle.verify_omega(&response.content);
            let v_elapsed = v_t0.elapsed().as_millis() as u64;
            verifier_wait_ms += v_elapsed;
            // A6 FC1-N12 (Lean oracle scope): per-call event with verdict
            // + elapsed_ms. Phase D consumer derives the verifier-cost
            // distribution and the verify-success rate. Run-level emit
            // (no agent_id; oneshot has only one virtual agent).
            let verdict_str = match &verdict {
                Ok(true) => "Ok(true)",
                Ok(false) => "Ok(false)",
                Err(_) => "Err",
            };
            minif2f_v4::fc_trace::emit_event(
                minif2f_v4::fc_trace::FcId::Fc1N12,
                // A8e fix F1: stamp the unified run_id (not the
                // round-1 `oneshot_{problem_file}` placeholder) so
                // Phase D can join by equality.
                &run_id, None, None,
                &[
                    ("verdict", minif2f_v4::fc_trace::json_str(verdict_str)),
                    ("elapsed_ms", v_elapsed.to_string()),
                ],
            );
            // B3: close the bracket AFTER the Lean call returns, regardless of
            // verdict. Soft Law mode (Phase C) cannot escape the verify-time
            // accounting by short-circuiting on runtime accept.
            wc.mark_final_accept();
            match verdict {
                Ok(true) => {
                    acc.flip_last_failed_to_accepted();
                    let gp_tokens = response.completion_tokens as u64;
                    let preview: String = response.content.chars().take(500).collect();
                    info!(">>> OMEGA ACCEPTED <<< (path=alone, payload[0..500]={:?})", preview);
                    let proof_file = persist_proof_artifact(
                        problem_file, theorem_name, problem_statement,
                        &response.content, "alone", "oneshot",
                    );
                    // P0-A: Phase B oneshot success — runtime gate IS the
                    // Lean verify call (oracle.verify_omega returned Ok(true)),
                    // so both legs hold. Phase C Soft Law would inject a
                    // separate `verify_post_hoc(&oracle, &response.content)`
                    // call here and pass its result as post_hoc_verified.
                    make_pput(problem_file, "oneshot", model,
                              true, true, start, gp_tokens, 1, 1,
                              None, None, Some(response.content.clone()),
                              Some("alone".to_string()), proof_file,
                              Some(acc.total_run_token_count()),
                              Some(acc.failed_branch_count),
                              wc.elapsed_ms(),
                              false, 1, 1, verifier_wait_ms,
                              oneshot_regime, oneshot_budget_base, &run_id)
                }
                Ok(false) => {
                    // Lean rejected → neither leg.
                    make_pput(problem_file, "oneshot", model,
                              false, false, start, 0, 0, 1,
                              None, None, None, None, None,
                              Some(acc.total_run_token_count()),
                              Some(acc.failed_branch_count),
                              wc.elapsed_ms(),
                              false, 1, 1, verifier_wait_ms,
                              oneshot_regime, oneshot_budget_base, &run_id)
                }
                Err(e) => {
                    warn!("Oracle error: {}", e);
                    // Lean error → measurement failure → neither leg.
                    make_pput(problem_file, "oneshot", model,
                              false, false, start, 0, 0, 1,
                              None, None, None, None, None,
                              Some(acc.total_run_token_count()),
                              Some(acc.failed_branch_count),
                              wc.elapsed_ms(),
                              false, 1, 1, verifier_wait_ms,
                              oneshot_regime, oneshot_budget_base, &run_id)
                }
            }
        }
        Err(e) => {
            // C-012: measurement failure ≠ verified failure.
            // Do not emit PPUT_RESULT — batch runner must retry on resume.
            // C-017: broadcast error explicitly (stderr, non-zero exit).
            error!("LLM error: {}", e);
            eprintln!("MEASUREMENT_ERROR oneshot LLM: {}", e);
            std::process::exit(2);
        }
    }
}

/// Swarm: N agents, prediction market, Boltzmann routing → PPUT.
async fn run_swarm(
    problem_file: &str, problem_statement: &str, theorem_name: &str,
    lean_path: &str, proxy_url: &str, model: &str, n_agents: usize,
) -> PputResult {
    let start = Instant::now();
    let condition = format!("n{}", n_agents);

    // A8e fix F1 (Codex#2 + Gemini Q4): single run_id minted ONCE per
    // run, threaded into both fc_event!s and make_pput. Replaces the
    // round-1 `run_corr_id` (FC events) ↔ make_pput-internal `run_id`
    // (v2 jsonl) split that introduced millisecond drift on the join key.
    let run_id = minif2f_v4::run_id::mint_run_id(&condition, problem_file);

    let kernel = Kernel::new();
    let config = BusConfig {
        // Phase 2.1 (C-043 candidate): OMEGA-accepted proofs are auto-written
        // as tape nodes (mandatory wtool per Art. IV). Full proofs can be
        // long; raise bus caps so winning nodes don't get size-vetoed. Agent
        // partials still typically <1200; no behavioural regression.
        max_payload_chars: 8000,
        max_payload_lines: 200,
        system_lp_amount: 200.0,
        // C-011: decide/omega/native_decide forbidden (brute-force precedent)
        forbidden_patterns: vec![
            "native_decide".into(), "decide".into(), "omega".into(),
            "#eval".into(), "IO.Process".into(),
            "IO.FS".into(), "run_tac".into(), "unsafe".into(),
        ],
    };

    // Phase 1: opt-in tape persistence via env. WAL_DIR=<dir> enables WAL
    // writes to <dir>/<problem>_<timestamp>.jsonl; resumes if file exists.
    // Default off for backward-compat baseline runs.
    let mut bus = if let Ok(wal_dir) = std::env::var("WAL_DIR") {
        let problem_stem = std::path::Path::new(problem_file)
            .file_stem().map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown".into());
        let resume_id = std::env::var("WAL_RESUME_ID").ok();
        let id = resume_id.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs().to_string())
                .unwrap_or_else(|_| "0".into())
        });
        let wal_path = std::path::Path::new(&wal_dir)
            .join(format!("{}_{}.jsonl", problem_stem, id));
        info!("[wal] using {:?}", wal_path);
        match TuringBus::with_wal_path(kernel, config, wal_path) {
            Ok(b) => b,
            Err(e) => {
                error!("[wal] open failed: {} — falling back to in-memory", e);
                TuringBus::new(Kernel::new(), BusConfig {
                    max_payload_chars: 1200, max_payload_lines: 18,
                    system_lp_amount: 200.0,
                    forbidden_patterns: vec![
                        "native_decide".into(), "decide".into(), "omega".into(),
                        "#eval".into(), "IO.Process".into(), "IO.FS".into(),
                        "run_tac".into(), "unsafe".into(),
                    ],
                })
            }
        }
    } else {
        TuringBus::new(kernel, config)
    };
    // Phase 4 (C-041 candidate): cross-problem wallet persistence. WALLET_STATE
    // env points to a json file; if it exists we load agents' carried-over
    // balances/portfolios, otherwise fresh genesis. No second mint under Law 2:
    // genesis_done is serialised, so on_init is a no-op post first boot.
    let wallet_state_path: Option<std::path::PathBuf> = std::env::var("WALLET_STATE")
        .ok().map(std::path::PathBuf::from);
    let wallet = wallet_state_path.as_ref()
        .and_then(|p| WalletTool::load_from_disk(p))
        .unwrap_or_else(|| WalletTool::new(10000.0));
    if wallet_state_path.is_some() && wallet.genesis_done {
        info!("[wallet] resumed from {:?}; existing agents carry balances",
              wallet_state_path);
    }
    bus.mount_tool(Box::new(wallet));
    bus.mount_tool(Box::new(Lean4Oracle::new(
        problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
    )));
    bus.mount_tool(Box::new(SearchTool::new(
        vec![format!("{}/MiniF2F/Test", std::env::var("MINIF2F_DIR")
            .unwrap_or_else(|_| DEFAULT_MINIF2F_DIR.into()))], 20,
    )));
    bus.mount_tool(Box::new(LibrarianTool::new(
        &format!("{}/skills", std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into())), 8,
    )));

    let agent_ids: Vec<String> = (0..n_agents).map(|i| format!("Agent_{}", i)).collect();
    bus.init(&agent_ids);
    // Phase 4: top-up ensure_agents for any IDs not in the loaded state (zero
    // balance if post-genesis, genesis_coins only on first-ever boot).
    if let Some(wallet) = bus.tools.iter_mut()
        .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
    {
        wallet.ensure_agents(&agent_ids);
    }

    // Phase A atom A3 (FC1-N7 δ/AI): per-agent model assignment via the
    // `AGENT_MODELS` env var. Default (unset/empty) broadcasts the global
    // `model` to every Agent_i. Heterogeneous payloads require
    // `PHASE_D_HETERO_OK=1` (Phase B+C single-model invariant — see
    // `agent_models.rs` module header). Failure is fatal at startup so a
    // misconfigured swarm cannot burn LLM budget on bad model identity.
    let agent_models = match minif2f_v4::agent_models::resolve_agent_models(model, n_agents) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("AGENT_MODELS resolution failed: {}", e);
            std::process::exit(1);
        }
    };
    // Stamp on jsonl: uniform → single canonical name; heterogeneous (Phase D
    // only, gated) → `hetero:{m1|m2|...}` so downstream PPUT analysis can
    // distinguish single-model runs from heterogeneous swarm runs without
    // having to crack open the genesis_payload model_snapshot field.
    let run_model_label: String = {
        let first = &agent_models[0];
        if agent_models.iter().all(|m| m == first) {
            first.clone()
        } else {
            let mut sorted: Vec<&str> = agent_models.iter().map(String::as_str).collect();
            sorted.sort();
            sorted.dedup();
            format!("hetero:{}", sorted.join("|"))
        }
    };
    info!("[swarm/{}] agent_models = [{}] (label={})", condition,
          agent_models.join(","), run_model_label);

    // Art. II.2.1: "不能抹杀群体异质性" — distinct skills per agent.
    // V3 had Math/Bull/Bear roles. V4: tactic-strategy specialization.
    let agent_skills: Vec<&str> = vec![
        "Focus on algebraic simplification: ring, field_simp, linarith, nlinarith.",
        "Focus on structural reasoning: induction, cases, rcases, constructor.",
        "Focus on rewriting and normalization: simp, norm_num, rw, calc.",
    ];

    let client = ResilientLLMClient::new(proxy_url, 1800, 2);
    let params = BoltzmannParams::from_env();
    // C-012: seed the Boltzmann RNG so A/B runs are reproducible.
    // Only the LLM sampling remains stochastic; same-problem paired comparison absorbs that.
    let boltzmann_seed: u64 = std::env::var("BOLTZMANN_SEED")
        .ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_BOLTZMANN_SEED);
    let mut boltz_rng = StdRng::seed_from_u64(boltzmann_seed);

    // Phase A atom A5 (FC2-N22 budget regime resolution): read
    // BUDGET_REGIME + MAX_TRANSACTIONS env, validate at startup, and
    // compute the loop bound. Errors abort BEFORE any LLM call so a
    // misconfigured run cannot consume API budget. Default
    // (env unset) = TotalProposal × 200, preserving Phase B baseline
    // bit-for-bit. PREREG_AMENDMENT_p0_defer § 3 condition 3.
    let (budget_regime, budget_max_tx_base, max_transactions) =
        match minif2f_v4::budget_regime::resolve_budget(n_agents) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("BUDGET_REGIME resolution failed: {}", e);
                std::process::exit(1);
            }
        };
    info!("[budget] regime={} base={} effective_max_tx={} (n_agents={})",
          budget_regime.label(), budget_max_tx_base, max_transactions, n_agents);
    let max_transactions = max_transactions as usize;

    // Art. IV map-reduce tick: periodic tape statistics (clock → mr → map/reduce)
    let tick_interval: usize = std::env::var("TICK_INTERVAL")
        .ok().and_then(|s| s.parse().ok()).unwrap_or(20);

    // C-036 startup echo: per-agent (skill, temp) so debugging never grep-source.
    let temp_ladder_on = std::env::var("TEMP_LADDER").ok().as_deref() == Some("1");
    let agent_cfg: Vec<String> = (0..n_agents).map(|i| {
        let s = i % agent_skills.len();
        let t = if temp_ladder_on { (0.10_f64 + (i as f64) * 0.15).min(1.30) } else { 0.2 };
        format!("Agent_{}:skill{}:t={:.2}", i, s, t)
    }).collect();
    info!("[swarm/{}] {}", condition, agent_cfg.join(" "));

    // C-036 telemetry counters.
    let mut tool_dist: HashMap<String, u32> = HashMap::new();
    let mut omega_payload_hashes: HashSet<u64> = HashSet::new();
    let mut omega_attempts: u32 = 0;
    let mut zero_ticks_run: u32 = 0;
    let mut zero_tick_warned = false;
    // Phase A atom A4 (FC1-N11 ∏p decision diversity): hash every parsed
    // proposal payload (append/complete/step) — broader than `omega_*`
    // which only counts OMEGA attempts. Cheap proxy for semantic
    // diversity (full embedding distance is Phase D+ work).
    let mut proposal_hashes: HashSet<u64> = HashSet::new();
    let mut proposal_count: u64 = 0;
    // Phase A atom A4 (FC1-N12 oracle scope): cumulative wall-clock
    // inside Lean for THIS run. Each verify_omega_detailed and
    // verify_partial call brackets its own elapsed and adds it here.
    let mut verifier_wait_ms: u64 = 0;
    // PPUT-CCL B2: full-run cost C_i — every LLM call + tool stdout summed
    // across all proposals (winning + failed branches). Read at terminal
    // make_pput sites and stamped on the emitted jsonl row.
    let mut acc = RunCostAccumulator::new();
    // PPUT-CCL B3: full-run wall-clock T_i — first agent prompt → final Lean
    // call. Opened on first tx's prompt build, closed before each return.
    let mut wc = RunWallClock::new();
    // Art. III.2: per-agent search result cache (bounded), fed into next prompt.
    let mut search_cache: HashMap<String, Vec<String>> = HashMap::new();
    // F-2026-04-19-05: cap searches per agent; beyond cap we remove `search`
    // from the tool list so agents stop wasting budget on name-match misses.
    let search_cap: u32 = std::env::var("SEARCH_CAP")
        .ok().and_then(|s| s.parse().ok()).unwrap_or(20);
    let mut search_count: HashMap<String, u32> = HashMap::new();
    // PPUT-CCL B7-extra (PREREG § 5.5): calibration treatment toggle.
    // When enabled, every proposal at tx >= ROLLBACK_TX_THRESHOLD is
    // synthetically vetoed. Constitutionally that is FC1-E18 (∏p=0 → Q_t)
    // applied repeatedly; the run then exhausts at FC2-N22 HALT via
    // `HaltReason::MaxTxExhausted`. We short-circuit at the threshold tx
    // for efficiency — see `rollback_sim.rs` module header for why this
    // is observably equivalent to running the loop to natural exhaustion.
    let rollback_sim_on = minif2f_v4::rollback_sim::rollback_simulation_enabled();
    if rollback_sim_on {
        info!("[rollback_sim] PREREG § 5.5 calibration treatment ON \
               (synthetic veto at tx >= {})", minif2f_v4::rollback_sim::ROLLBACK_TX_THRESHOLD);
    }

    for tx in 0..max_transactions {
        // PPUT-CCL B7-extra: short-circuit guard. Constitutional anchor
        // FC1-E18 + FC2-N22 (existing MaxTxExhausted variant). Stamps
        // tx_count at the threshold, not at max_transactions, so jsonl
        // analysis can distinguish a calibration treatment exit from a
        // real natural exhaustion.
        if minif2f_v4::rollback_sim::should_simulate_rollback(tx as u64, rollback_sim_on) {
            warn!("[rollback_sim] firing at tx={} — synthetic ∏p=0 from this tx, \
                   short-circuit to MaxTxExhausted exit (cost-asymmetric: skips \
                   ~150 LLM calls vs honest vetoed loop; downstream PPUT analysis \
                   MUST honor synthetic_short_circuit=true on this row)", tx);
            // A6 FC2-N22 (HALT): synthetic short-circuit path. Phase D
            // join key: reason="SyntheticShortCircuit" disambiguates from
            // natural MaxTxExhausted (which exits at tx=max_transactions).
            minif2f_v4::fc_trace::emit_event(
                minif2f_v4::fc_trace::FcId::Fc2N22,
                &run_id, Some(tx as u64), None,
                &[("reason", minif2f_v4::fc_trace::json_str("SyntheticShortCircuit"))],
            );
            wc.mark_final_accept();
            // A4: synthetic short-circuit is NOT a max-tx exhaustion (it
            // exits ~150 tx EARLY at the rollback threshold). hit_max_tx
            // stays false — synthetic_short_circuit is the disambiguator
            // for this calibration-treatment path.
            let mut result = make_pput(problem_file, &condition, &run_model_label,
                                       false, false, start, 0, 0,
                                       tx as u64, Some(tool_dist), None,
                                       None, None, None,
                                       Some(acc.total_run_token_count()),
                                       Some(acc.failed_branch_count),
                                       wc.elapsed_ms(),
                                       false,
                                       proposal_hashes.len() as u64,
                                       proposal_count,
                                       verifier_wait_ms,
                                       budget_regime, budget_max_tx_base, &run_id);
            // B7-extra disambiguator: distinguish this calibration-treatment
            // exit from a natural max-tx exhaustion in downstream PPUT
            // analysis. See PputResult::synthetic_short_circuit doc-comment
            // for the cost-asymmetry note.
            result.synthetic_short_circuit = Some(true);
            return result;
        }

        // PPUT-CCL B3 (mid-term audit P0-C fix 2026-04-25): open the wall-clock
        // bracket at the top of the FIRST tx (before chain/skill/board build
        // and before build_agent_prompt). Idempotent — only the first tx's
        // call sticks; subsequent calls no-op. PREREG § 5 / plan B3 define
        // T_i as "first agent prompt construction"; this is the earliest
        // moment the agent begins constructing its prompt.
        wc.mark_first_read();

        // Map-reduce tick (Art. IV mermaid: clock → mr → tape)
        if tick_interval > 0 && tx > 0 && tx % tick_interval == 0 {
            let tape_len = bus.kernel.tape.time_arrow().len();
            let market_count = bus.kernel.markets.len();
            let ticker = bus.kernel.market_ticker(5);
            let top_prices: Vec<String> = ticker.iter()
                .map(|(id, p)| format!("{}:{:.0}%", id, p * 100.0))
                .collect();
            info!("[tick@tx{}] tape={} markets={} top={}", tx, tape_len, market_count,
                top_prices.join(", "));
            // A6 FC2-N20 (mr tick): clock → mr → tape per Art. IV.
            // Phase D consumer joins on (run_id, tx) to derive the
            // tape-growth curve and detect zero-tick stalls before they
            // become C-036 alarm events.
            minif2f_v4::fc_trace::emit_event(
                minif2f_v4::fc_trace::FcId::Fc2N20,
                &run_id, Some(tx as u64), None,
                &[
                    ("tape_len", tape_len.to_string()),
                    ("market_count", market_count.to_string()),
                ],
            );
            // Phase 6-emergent: refresh shared team board from facts only.
            // Per-agent cumulative balance + recent tape-node authorship counts
            // + top market prices. No instructions, no "should" — just state.
            if std::env::var("EMERGENT_ROLES").ok().as_deref() == Some("1") {
                let agents_sorted: Vec<String> = agent_ids.clone();
                let mut author_counts: std::collections::HashMap<String, u32> =
                    std::collections::HashMap::new();
                for nid in bus.kernel.tape.time_arrow() {
                    if let Some(n) = bus.kernel.tape.get(nid) {
                        *author_counts.entry(n.author.clone()).or_insert(0) += 1;
                    }
                }
                let wallet_balances: std::collections::HashMap<String, f64> =
                    bus.tools.iter()
                        .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
                        .map(|w| w.balances.clone())
                        .unwrap_or_default();
                let mut board = format!("# tick@tx{} (tape_nodes={})\n", tx, tape_len);
                for a in &agents_sorted {
                    let bal = wallet_balances.get(a).copied().unwrap_or(10000.0);
                    let delta = bal - 10000.0;
                    let nodes = author_counts.get(a).copied().unwrap_or(0);
                    board.push_str(&format!(
                        "- {}: balance={:.0} (Δ{:+.0}), tape_nodes_authored={}\n",
                        a, bal, delta, nodes));
                }
                if !top_prices.is_empty() {
                    board.push_str(&format!("markets: {}\n", top_prices.join(", ")));
                }
                // Preserve any agent posts that were already in the file (append-only).
                if let Some(lib) = bus.tools.iter()
                    .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
                {
                    let existing = lib.read_board();
                    // Keep only the POST lines (they carry agent-originated intent).
                    let posts: String = existing.lines()
                        .filter(|l| l.starts_with("## POST") || (l.starts_with(" ") == false && !l.starts_with("#") && !l.starts_with("-") && !l.starts_with("markets:")))
                        .collect::<Vec<_>>()
                        .join("\n");
                    let full = if posts.is_empty() {
                        board
                    } else {
                        format!("{}\n{}\n", board, posts)
                    };
                    let _ = lib.write_board(&full);
                }
            }
            // C-036 zero-tick alarm: 5 consecutive ticks with no constitutional engine activity.
            if tape_len == 0 && market_count == 0 {
                zero_ticks_run += 1;
                if zero_ticks_run >= 5 && !zero_tick_warned {
                    warn!("[harness] {} consecutive zero-ticks (tape & markets idle) — \
                           constitutional engines bypassed (Art. II.1/II.2 unused)", zero_ticks_run);
                    zero_tick_warned = true;
                }
            } else {
                zero_ticks_run = 0;
            }
        }

        let agent_idx = tx % n_agents;
        let agent_id = &agent_ids[agent_idx];
        let snap = bus.snapshot();

        let chain = if snap.tape.is_empty() {
            problem_statement.to_string()
        } else {
            let nodes: Vec<String> = snap.tape.time_arrow().iter()
                .filter_map(|id| snap.tape.get(id))
                .map(|n| format!("[{}] {}: {}", n.id, n.author, n.payload))
                .collect();
            format!("{}\n\n=== Proof Chain ===\n{}", problem_statement, nodes.join("\n"))
        };

        let errors = bus.recent_rejections(agent_id, 3);
        // Art. II.2.1: per-agent skill specialization + Librarian learned memory
        let base_skill = agent_skills.get(agent_idx % agent_skills.len()).unwrap_or(&"");
        let learned = bus.tools.iter()
            .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
            .and_then(|lib| lib.read_agent_memory(agent_id))
            .unwrap_or_default();
        let skill = if learned.is_empty() {
            base_skill.to_string()
        } else {
            format!("{}\n\n{}", base_skill, learned)
        };
        let hits_ref: Vec<String> = search_cache.get(agent_id).cloned().unwrap_or_default();
        let tools_desc = if search_count.get(agent_id).copied().unwrap_or(0) >= search_cap {
            "append, complete, invest"
        } else {
            "append, complete, invest, search"
        };
        // Phase 6-emergent: read the shared team board. Gated by EMERGENT_ROLES=1
        // so baseline behaviour is untouched. Board content is built by
        // Librarian at periodic ticks (see refresh_board below).
        let team_board: String = if std::env::var("EMERGENT_ROLES").ok().as_deref() == Some("1") {
            bus.tools.iter()
                .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
                .map(|l| l.read_board())
                .unwrap_or_default()
        } else {
            String::new()
        };
        let prompt = build_agent_prompt(
            &chain, &skill, &snap.market_ticker, &errors, &hits_ref,
            snap.get_balance(agent_id), tools_desc, &team_board,
        );

        // Phase A atom A3: bind δ for this agent_idx (same vector resolved
        // once at run_swarm entry from AGENT_MODELS env). In Phase B+C this
        // is uniform across all agent_idx; in Phase D it may diverge.
        let agent_model = &agent_models[agent_idx];
        // Model-aware max_tokens (same rule as oneshot branch). Per-agent so
        // a heterogeneous Phase D swarm mixing chat + reasoner backbones gets
        // the right ceiling per-call instead of a single global heuristic.
        let max_toks = if agent_model.contains("chat") { 8000 } else { 16000 };
        // Art. II.2.1 anti-homogeneity: per-agent temperature ladder breaks
        // sampling correlation among role-distinct agents (F-2026-04-18-03).
        // Disabled (keep at 0.2) when TEMP_LADDER!=1 to isolate the mechanism.
        let temp: f64 = if std::env::var("TEMP_LADDER").ok().as_deref() == Some("1") {
            (0.10_f64 + (agent_idx as f64) * 0.15).min(1.30)
        } else {
            0.2
        };
        let request = GenerateRequest {
            model: agent_model.clone(),
            messages: vec![Message { role: "user".into(), content: prompt }],
            temperature: Some(temp),
            max_tokens: Some(max_toks),
        };

        // PPUT-CCL B6 runtime gate (swarm path): swarm prompts include
        // tape contents, board posts, search hits, and learned memory —
        // any of these state surfaces could in principle inject a PPUT
        // value at runtime even when the prompt builder is clean. Gate
        // every tx, every agent, every iteration.
        assert_no_metric_leak(&request.messages[0].content);
        match client.generate(&request).await {
            Ok(response) => {
                acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
                // PPUT-CCL B2: every parsed proposal default-records as failed.
                // OMEGA-accept return paths flip the last record before returning.
                acc.record_proposal(false);
                match parse_agent_output(&response.content) {
                    Ok(action) => match action.tool.as_str() {
                        "append" => {
                            *tool_dist.entry("append".into()).or_insert(0) += 1;
                            if let Some(payload) = &action.payload {
                                // A4: record proposal for tactic_diversity.
                                let mut ph = std::collections::hash_map::DefaultHasher::new();
                                payload.hash(&mut ph);
                                proposal_hashes.insert(ph.finish());
                                proposal_count += 1;
                                let prices: std::collections::HashMap<String, f64> =
                                    snap.markets.iter()
                                        .map(|(id, m)| (id.clone(), m.yes_price))
                                        .collect();
                                let parent = boltzmann_select_parent(
                                    &snap.tape, &prices, &params, &mut boltz_rng
                                );
                                match bus.append(agent_id, payload, parent.as_deref()) {
                                    Ok(BusResult::Appended { node_id }) => {
                                        info!("[tx {}] {} +{}", tx, agent_id, node_id);
                                        // Art. III.2 Librarian: every compress_interval appends,
                                        // write mechanical summary (TopK error classes) to agent's
                                        // learned.md. This is white-box compression (Art. I.2:
                                        // deterministic statistical algorithm), not LLM-based.
                                        if let Some(lib) = bus.tools.iter()
                                            .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>()) {
                                            if lib.should_compress() {
                                                let errors = bus.recent_rejections(agent_id, 10);
                                                let summary = format!(
                                                    "# Learned patterns (auto-compressed)\n\
                                                     Common errors: {}\n\
                                                     Tape depth: {}\n",
                                                    errors.join(", "),
                                                    snap.tape.time_arrow().len(),
                                                );
                                                let _ = lib.write_agent_memory(agent_id, &summary);
                                                info!("[tx {}] Librarian compressed for {}", tx, agent_id);
                                            }
                                        }
                                    }
                                    Ok(BusResult::Vetoed { reason }) => {
                                        warn!("[tx {}] VETO: {}", tx, reason);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "complete" => {
                            *tool_dist.entry("complete".into()).or_insert(0) += 1;
                            if let Some(payload) = &action.payload {
                                // Art. IV (∏p(output | Q_t)): Q_t (tape) feeds the verification
                                // predicate. Dual-path: try payload-alone first (standalone proof
                                // preserved), then tape+payload (tape-built proof). Accept whichever
                                // succeeds. This keeps Q_t in the ∏p domain without punishing
                                // self-contained proofs that ignored tape.
                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
                                    .filter_map(|id| bus.kernel.tape.get(id))
                                    .map(|n| n.payload.clone())
                                    .collect::<Vec<_>>()
                                    .join("\n");
                                let tape_len = bus.kernel.tape.time_arrow().len();
                                // C-036: track payload diversity over what agent proposed.
                                let mut h = std::collections::hash_map::DefaultHasher::new();
                                payload.hash(&mut h);
                                omega_payload_hashes.insert(h.finish());
                                omega_attempts += 1;
                                // A4: also record into the broader proposal set
                                // for tactic_diversity (covers append/complete/step).
                                proposal_hashes.insert(h.finish());
                                proposal_count += 1;
                                info!("[tx {}] OMEGA claim by {} (tape_nodes={}, payload_len={})",
                                      tx, agent_id, tape_len, payload.len());
                                let oracle = Lean4Oracle::new(
                                    problem_statement.to_string(),
                                    theorem_name.to_string(),
                                    lean_path.to_string(),
                                );
                                // Path 1: payload alone (A4 verifier_wait bracket)
                                let v_t0 = Instant::now();
                                let r_alone = oracle.verify_omega_detailed(payload);
                                let v_alone_elapsed = v_t0.elapsed().as_millis() as u64;
                                verifier_wait_ms += v_alone_elapsed;
                                // A8e fix F4 (Codex#3): emit FC1-N12 for the swarm
                                // verify_omega_detailed call. Round-1 audit showed
                                // FC1-N12 was only emitted in oneshot, leaving the
                                // primary swarm verify path invisible to Phase D.
                                let r_alone_verdict = match &r_alone {
                                    Ok((true, _)) => "Ok(true)",
                                    Ok((false, _)) => "Ok(false)",
                                    Err(_) => "Err",
                                };
                                minif2f_v4::fc_trace::emit_event(
                                    minif2f_v4::fc_trace::FcId::Fc1N12,
                                    &run_id, Some(tx as u64), Some(agent_id.as_str()),
                                    &[
                                        ("verdict", minif2f_v4::fc_trace::json_str(r_alone_verdict)),
                                        ("elapsed_ms", v_alone_elapsed.to_string()),
                                        ("path", minif2f_v4::fc_trace::json_str("alone")),
                                    ],
                                );
                                let (full_proof, path_choice, r_final) = match &r_alone {
                                    Ok((true, _)) => (payload.clone(), "alone", r_alone.clone()),
                                    _ if !tape_chain.is_empty() => {
                                        // Path 2: tape + payload (A4 verifier_wait bracket)
                                        let combined = format!("{}\n{}", tape_chain, payload);
                                        let v_t1 = Instant::now();
                                        let r_combined = oracle.verify_omega_detailed(&combined);
                                        let v_combined_elapsed = v_t1.elapsed().as_millis() as u64;
                                        verifier_wait_ms += v_combined_elapsed;
                                        // A8e fix F4: FC1-N12 for the tape+payload retry.
                                        let r_combined_verdict = match &r_combined {
                                            Ok((true, _)) => "Ok(true)",
                                            Ok((false, _)) => "Ok(false)",
                                            Err(_) => "Err",
                                        };
                                        minif2f_v4::fc_trace::emit_event(
                                            minif2f_v4::fc_trace::FcId::Fc1N12,
                                            &run_id, Some(tx as u64), Some(agent_id.as_str()),
                                            &[
                                                ("verdict", minif2f_v4::fc_trace::json_str(r_combined_verdict)),
                                                ("elapsed_ms", v_combined_elapsed.to_string()),
                                                ("path", minif2f_v4::fc_trace::json_str("tape+payload")),
                                            ],
                                        );
                                        if matches!(r_combined, Ok((true, _))) {
                                            *tool_dist.entry("complete_via_tape".into()).or_insert(0) += 1;
                                        }
                                        (combined, "tape+payload", r_combined)
                                    }
                                    _ => (payload.clone(), "alone", r_alone.clone()),
                                };
                                // PPUT-CCL B3: close bracket AFTER both Lean verify paths return.
                                // Soft Law (Phase C) cannot exit ahead of verify-time accounting.
                                wc.mark_final_accept();
                                match r_final {
                                    Ok((true, _)) => {
                                        // PPUT-CCL B2: this proposal verified — flip the failed
                                        // record made at parse time into the run's accepted slot.
                                        acc.flip_last_failed_to_accepted();
                                        // Phase 0 (C-039): persist the winning artifact so external
                                        // verifiers can re-run lean from disk alone.
                                        let preview: String = full_proof.chars().take(500).collect();
                                        info!(">>> OMEGA ACCEPTED <<< (path={}, payload[0..500]={:?})",
                                              path_choice, preview);
                                        let proof_file = persist_proof_artifact(
                                            problem_file, &theorem_name, &problem_statement,
                                            &full_proof, path_choice, agent_id,
                                        );
                                        // Phase 2.1 (C-043 candidate): mandatory wtool. Art. IV says
                                        // `∏p = 1 ⟹ Q_{t+1} = wtool(output)`. Before halting, write
                                        // the winning payload as a tape node through the standard
                                        // append pipeline. This automatically fires founder grant
                                        // (Phase 2 reward-pull) for the winning author and makes
                                        // every solve end with a canonical tape node on the GP.
                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
                                        *tool_dist.entry("omega_wtool".into()).or_insert(0) += 1;
                                        // Use oracle-blessed path: Lean has already accepted this
                                        // payload, so bus-level forbidden_patterns and size caps
                                        // would only re-reject legitimate tactics (e.g. `omega`,
                                        // `decide` used inside a verified proof — not brute-force).
                                        let omega_node_id = match bus.append_oracle_accepted(
                                            agent_id, payload, parent.as_deref(),
                                        ) {
                                            Ok(BusResult::Appended { node_id }) => Some(node_id),
                                            Ok(BusResult::Vetoed { reason }) => {
                                                warn!("[art-iv] OMEGA wtool VETO (unexpected after oracle accept): {}", reason);
                                                None
                                            }
                                            _ => None,
                                        };
                                        let tape_tokens: u64 = bus.kernel.tape.time_arrow().iter()
                                            .filter_map(|id| bus.kernel.tape.get(id))
                                            .map(|n| n.payload.len() as u64)
                                            .sum();
                                        // C-012: gp_tokens reflects the actual tape (now containing
                                        // the winner), no double-count needed.
                                        let gp_tokens = tape_tokens.max(response.completion_tokens as u64);
                                        let gp = bus.kernel.tape.time_arrow().to_vec();
                                        let gp_nodes = gp.len();
                                        if omega_node_id.is_some() {
                                            info!("[art-iv] OMEGA written as tape node; gp_nodes={}", gp_nodes);
                                        }
                                        bus.halt_and_settle(&gp).ok();
                                        // A6 FC2-N22 (HALT — OmegaAccepted via full proof): the
                                        // canonical success-path event. Phase D filters on
                                        // reason="OmegaAccepted" + gp_path="alone|tape+payload" to
                                        // build the OMEGA accept-rate timeseries.
                                        minif2f_v4::fc_trace::emit_event(
                                            minif2f_v4::fc_trace::FcId::Fc2N22,
                                            &run_id, Some(tx as u64), Some(agent_id.as_str()),
                                            &[
                                                ("reason", minif2f_v4::fc_trace::json_str("OmegaAccepted")),
                                                ("gp_path", minif2f_v4::fc_trace::json_str(path_choice)),
                                                ("gp_nodes", gp_nodes.to_string()),
                                            ],
                                        );
                                        // Phase 4: persist wallet state so next problem's run
                                        // inherits carried-over balances (reputation).
                                        if let Some(ref wp) = wallet_state_path {
                                            if let Some(w) = bus.tools.iter()
                                                .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
                                            {
                                                if let Err(e) = w.save_to_disk(wp) {
                                                    warn!("[wallet] save failed to {:?}: {}", wp, e);
                                                }
                                            }
                                        }
                                        let upr = if omega_attempts > 0 {
                                            Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
                                        } else { None };
                                        // P0-A: Phase B swarm complete — runtime gate IS the
                                        // Lean verify_omega_detailed call we just consumed
                                        // (Ok((true, _))). Both legs hold. Phase C Soft Law
                                        // would inject `verify_post_hoc(&oracle, &full_proof)`
                                        // here and pass its result as post_hoc_verified.
                                        return make_pput(problem_file, &condition, &run_model_label,
                                                        true, true,
                                                        start, gp_tokens, gp_nodes, tx as u64 + 1,
                                                        Some(tool_dist), upr,
                                                        Some(full_proof.clone()),
                                                        Some(path_choice.to_string()),
                                                        proof_file,
                                                        Some(acc.total_run_token_count()),
                                                        Some(acc.failed_branch_count),
                                                        wc.elapsed_ms(),
                                                        false,
                                                        proposal_hashes.len() as u64,
                                                        proposal_count,
                                                        verifier_wait_ms,
                                                        budget_regime, budget_max_tx_base, &run_id);
                                    }
                                    Ok((false, err_detail)) => {
                                        // Step-B v3: classify + record class label (C-022 shield).
                                        let class = classify_lean_error(&err_detail);
                                        bus.record_rejection(agent_id, class.label());
                                        // PPUT-CCL B2: rejection error feeds back into next prompt's
                                        // recent_rejections — count those bytes against C_i.
                                        acc.record_tool_stdout(&err_detail);
                                        let preview: String = payload.chars().take(300).collect();
                                        warn!("[tx {}] OMEGA rejected ({}). payload[0..300]={:?}", tx, class.label(), preview);
                                    }
                                    Err(e) => {
                                        warn!("[tx {}] OMEGA oracle error: {}", tx, e);
                                    }
                                }
                            }
                        }
                        "invest" => {
                            *tool_dist.entry("invest".into()).or_insert(0) += 1;
                            // Law 2: Only Investment Costs Money (1 Coin = 1 YES + 1 NO).
                            // Agent bets on a tape node's quality. This drives price signals
                            // (Art. II.2) which guide Boltzmann routing (Art. II.2.1).
                            // Direction: prefer explicit `direction` field (long/short);
                            // fall back to sign of amount (positive=long, negative=short).
                            // Bidirectional signals let agents express dissent (Art. II.2).
                            if let (Some(node_id), Some(amount)) = (&action.node, action.amount) {
                                let amt = amount.abs();
                                if amt > 0.0 {
                                    let buy_yes = match action.direction.as_deref() {
                                        Some("long") | Some("yes") | Some("LONG") | Some("YES") => true,
                                        Some("short") | Some("no") | Some("SHORT") | Some("NO") => false,
                                        _ => amount > 0.0,  // sign-based fallback
                                    };
                                    // Law 2 conservation: validate market BEFORE debit (no coin-loss path)
                                    let market_exists = bus.kernel.yes_price(node_id).is_some();
                                    if !market_exists {
                                        warn!("[tx {}] invest: no market for {} (hallucinated node?)", tx, node_id);
                                    } else {
                                        // Debit wallet → buy shares → record (atomic intent)
                                        let wallet_ok = bus.tools.iter_mut()
                                            .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
                                            .map(|w| w.deduct(agent_id, amt).is_ok())
                                            .unwrap_or(false);
                                        if wallet_ok {
                                            let result = if buy_yes {
                                                bus.kernel.buy_yes(node_id, amt)
                                            } else {
                                                bus.kernel.buy_no(node_id, amt)
                                            };
                                            match result {
                                                Ok(shares) => {
                                                    info!("[tx {}] {} invested {:.0} {} on {} → {:.1} shares",
                                                        tx, agent_id, amt,
                                                        if buy_yes { "YES" } else { "NO" },
                                                        node_id, shares);
                                                    if let Some(w) = bus.tools.iter_mut()
                                                        .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>()) {
                                                        if buy_yes {
                                                            w.record_shares(agent_id, node_id, shares, 0.0, 0.0);
                                                        } else {
                                                            w.record_shares(agent_id, node_id, 0.0, shares, 0.0);
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    // Market existed at check but buy failed — should not happen
                                                    warn!("[tx {}] invest buy error: {} (coins debited, shares not granted — Law 2 violation logged)", tx, e);
                                                }
                                            }
                                        } else {
                                            warn!("[tx {}] {} insufficient balance for invest", tx, agent_id);
                                        }
                                    }
                                }
                            }
                        }
                        "search" => {
                            // F-2026-04-19-05 cap: if over budget this agent's turn the
                            // search slot shouldn't even be offered, but the LLM may still
                            // emit `search` ignoring the prompt — record and skip execute.
                            let cnt = search_count.entry(agent_id.clone()).or_insert(0);
                            if *cnt >= search_cap {
                                *tool_dist.entry("search_capped".into()).or_insert(0) += 1;
                            } else {
                                *cnt += 1;
                                *tool_dist.entry("search".into()).or_insert(0) += 1;
                                // Law 1: search is free. Execute and cache top hits (Art. III.2).
                                if let Some(query) = &action.query {
                                    let hits = bus.tools.iter()
                                        .find_map(|t| t.as_any().downcast_ref::<SearchTool>())
                                        .map(|s| s.search(query))
                                        .unwrap_or_default();
                                    let trimmed: Vec<String> = hits.iter().take(5)
                                        .map(|p| p.rsplit('/').next().unwrap_or(p).to_string())
                                        .collect();
                                    // PPUT-CCL B2: search hits feed `hits_ref` into next prompt —
                                    // count the cached bytes against C_i.
                                    acc.record_tool_stdout(&trimmed.join("\n"));
                                    info!("[tx {}] {} search({:?}) → {} hits: {}",
                                          tx, agent_id, query, hits.len(), trimmed.join(","));
                                    search_cache.insert(agent_id.clone(), trimmed);
                                }
                            }
                        }
                        "post" => {
                            *tool_dist.entry("post".into()).or_insert(0) += 1;
                            // Phase 6-emergent: agent posts a short message to the
                            // shared Librarian board. Other agents see it on next
                            // prompt. State-only; no central role planner.
                            if let Some(msg) = &action.payload {
                                if let Some(lib) = bus.tools.iter()
                                    .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
                                {
                                    if let Err(e) = lib.post_to_board(agent_id, msg) {
                                        warn!("[tx {}] post failed: {}", tx, e);
                                    } else {
                                        info!("[tx {}] {} posted to board", tx, agent_id);
                                    }
                                }
                            }
                        }
                        "step" => {
                            // Phase 7 (C-043+ Turing δ-step): submit ONE tactic,
                            // oracle classifies the accumulated tape+tactic prefix
                            // as Complete / PartialOk / Reject. Writes a tape node
                            // on PartialOk and Complete so the DAG grows one cell
                            // at a time — the Art. IV semantics Turing 1936 defines.
                            *tool_dist.entry("step".into()).or_insert(0) += 1;
                            if let Some(tactic) = &action.payload {
                                // A4: record proposal for tactic_diversity.
                                let mut ph = std::collections::hash_map::DefaultHasher::new();
                                tactic.hash(&mut ph);
                                proposal_hashes.insert(ph.finish());
                                proposal_count += 1;
                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
                                    .filter_map(|id| bus.kernel.tape.get(id))
                                    .map(|n| n.payload.clone())
                                    .collect::<Vec<_>>()
                                    .join("\n");
                                let prefix = if tape_chain.is_empty() {
                                    tactic.clone()
                                } else {
                                    format!("{}\n{}", tape_chain, tactic)
                                };
                                let oracle = Lean4Oracle::new(
                                    problem_statement.to_string(),
                                    theorem_name.to_string(),
                                    lean_path.to_string(),
                                );
                                // A4: bracket the Lean partial-verify call.
                                let v_t0 = Instant::now();
                                let verdict = oracle.verify_partial(&prefix);
                                let v_partial_elapsed = v_t0.elapsed().as_millis() as u64;
                                verifier_wait_ms += v_partial_elapsed;
                                // A8e fix F4 (Codex#3): FC1-N12 emit for the
                                // step-verify path. Closes the swarm-side gap
                                // round-1 audit flagged.
                                let partial_verdict_str = match &verdict {
                                    PartialVerdict::Complete => "Complete",
                                    PartialVerdict::PartialOk => "PartialOk",
                                    PartialVerdict::Reject(_) => "Reject",
                                };
                                minif2f_v4::fc_trace::emit_event(
                                    minif2f_v4::fc_trace::FcId::Fc1N12,
                                    &run_id, Some(tx as u64), Some(agent_id.as_str()),
                                    &[
                                        ("verdict", minif2f_v4::fc_trace::json_str(partial_verdict_str)),
                                        ("elapsed_ms", v_partial_elapsed.to_string()),
                                        ("path", minif2f_v4::fc_trace::json_str("partial")),
                                    ],
                                );
                                // PPUT-CCL B3: close bracket after step-verify returns.
                                wc.mark_final_accept();
                                match verdict {
                                    PartialVerdict::Complete => {
                                        acc.flip_last_failed_to_accepted();
                                        info!(">>> OMEGA ACCEPTED <<< via step (depth={} after this write)",
                                              bus.kernel.tape.time_arrow().len() + 1);
                                        let proof_file = persist_proof_artifact(
                                            problem_file, &theorem_name, &problem_statement,
                                            &prefix, "per_tactic", agent_id,
                                        );
                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
                                        *tool_dist.entry("omega_wtool".into()).or_insert(0) += 1;
                                        let _ = bus.append_oracle_accepted(
                                            agent_id, tactic, parent.as_deref(),
                                        );
                                        let tape_tokens: u64 = bus.kernel.tape.time_arrow().iter()
                                            .filter_map(|id| bus.kernel.tape.get(id))
                                            .map(|n| n.payload.len() as u64)
                                            .sum();
                                        let gp_tokens = tape_tokens.max(response.completion_tokens as u64);
                                        let gp = bus.kernel.tape.time_arrow().to_vec();
                                        let gp_nodes = gp.len();
                                        bus.halt_and_settle(&gp).ok();
                                        let upr = if omega_attempts > 0 {
                                            Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
                                        } else { None };
                                        // A6 FC2-N22 (HALT — OmegaAccepted via per-tactic
                                        // PartialVerdict::Complete). Distinguished from the
                                        // full-proof OMEGA path by gp_path="per_tactic"; both
                                        // share reason="OmegaAccepted".
                                        minif2f_v4::fc_trace::emit_event(
                                            minif2f_v4::fc_trace::FcId::Fc2N22,
                                            &run_id, Some(tx as u64), Some(agent_id.as_str()),
                                            &[
                                                ("reason", minif2f_v4::fc_trace::json_str("OmegaAccepted")),
                                                ("gp_path", minif2f_v4::fc_trace::json_str("per_tactic")),
                                                ("gp_nodes", gp_nodes.to_string()),
                                            ],
                                        );
                                        // P0-A: Phase B swarm step Complete — runtime gate IS
                                        // the Lean verify_partial call (PartialVerdict::Complete).
                                        // Both legs hold. Phase C Soft Law diverges here.
                                        return make_pput(problem_file, &condition, &run_model_label,
                                                        true, true,
                                                        start, gp_tokens, gp_nodes, tx as u64 + 1,
                                                        Some(tool_dist), upr,
                                                        Some(prefix.clone()),
                                                        Some("per_tactic".to_string()),
                                                        proof_file,
                                                        Some(acc.total_run_token_count()),
                                                        Some(acc.failed_branch_count),
                                                        wc.elapsed_ms(),
                                                        false,
                                                        proposal_hashes.len() as u64,
                                                        proposal_count,
                                                        verifier_wait_ms,
                                                        budget_regime, budget_max_tx_base, &run_id);
                                    }
                                    PartialVerdict::PartialOk => {
                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
                                        match bus.append_oracle_accepted(
                                            agent_id, tactic, parent.as_deref(),
                                        ) {
                                            Ok(BusResult::Appended { node_id }) => {
                                                *tool_dist.entry("step_partial_ok".into()).or_insert(0) += 1;
                                                info!("[tx {}] {} step+{} partial OK (depth={})",
                                                      tx, agent_id, node_id,
                                                      bus.kernel.tape.time_arrow().len());
                                            }
                                            Ok(BusResult::Vetoed { reason }) => {
                                                warn!("[tx {}] step partial OK but bus vetoed: {}", tx, reason);
                                            }
                                            _ => {}
                                        }
                                    }
                                    PartialVerdict::Reject(reason) => {
                                        let class = classify_lean_error(&reason);
                                        bus.record_rejection(agent_id, class.label());
                                        // PPUT-CCL B2: step rejection reason flows into next prompt.
                                        acc.record_tool_stdout(&reason);
                                        *tool_dist.entry("step_reject".into()).or_insert(0) += 1;
                                        let preview = reason.chars().take(200).collect::<String>();
                                        warn!("[tx {}] step rejected ({}): {}", tx, class.label(), preview);
                                    }
                                }
                            }
                        }
                        other => {
                            *tool_dist.entry(format!("other:{}", other)).or_insert(0) += 1;
                        }
                    },
                    Err(e) => {
                        *tool_dist.entry("parse_fail".into()).or_insert(0) += 1;
                        // Step-B v3: parse failures feed the class graveyard too.
                        let class = classify_parse_error(&format!("{}", e));
                        bus.record_rejection(agent_id, class.label());
                        // PPUT-CCL B2: classifier label flows into next prompt's errors.
                        acc.record_tool_stdout(class.label());
                        warn!("[tx {}] parse: {} ({})", tx, e, class.label());
                    }
                }
            }
            Err(e) => {
                *tool_dist.entry("llm_err".into()).or_insert(0) += 1;
                warn!("[tx {}] LLM: {}", tx, e);
            }
        }
    }

    let upr = if omega_attempts > 0 {
        Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
    } else { None };
    // Phase 4: also save wallet state on no-OMEGA exit. Agents may have
    // invested/lost Coin during the run; durability should not depend on a win.
    if let Some(ref wp) = wallet_state_path {
        if let Some(w) = bus.tools.iter()
            .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
        {
            let _ = w.save_to_disk(wp);
        }
    }
    // No OMEGA found → PPUT = 0
    // B3: close bracket on max-tx exhaustion path.
    // P0-A: max-tx exhaustion → neither leg fired.
    // A4: this is the canonical hit_max_tx=true site (ran the full
    // for-loop without OMEGA and without firing the synthetic
    // short-circuit, which would have returned earlier).
    wc.mark_final_accept();
    // A6 FC2-N22 (HALT — natural MaxTxExhausted): the canonical
    // budget-exhausted exit. Phase D filters reason="MaxTxExhausted"
    // to compute solve_rate-vs-budget curves; pairs with the A5
    // budget_regime stamp on the v2 jsonl row.
    minif2f_v4::fc_trace::emit_event(
        minif2f_v4::fc_trace::FcId::Fc2N22,
        &run_id, Some(max_transactions as u64), None,
        &[
            ("reason", minif2f_v4::fc_trace::json_str("MaxTxExhausted")),
            ("budget_regime", minif2f_v4::fc_trace::json_str(budget_regime.label())),
            ("budget_max_transactions", budget_max_tx_base.to_string()),
            ("proposal_count", proposal_count.to_string()),
        ],
    );
    make_pput(problem_file, &condition, &run_model_label,
              false, false, start, 0, 0,
              max_transactions as u64, Some(tool_dist), upr,
              None, None, None,
              Some(acc.total_run_token_count()),
              Some(acc.failed_branch_count),
              wc.elapsed_ms(),
              true,
              proposal_hashes.len() as u64,
              proposal_count,
              verifier_wait_ms,
              budget_regime, budget_max_tx_base, &run_id)
}

fn make_pput(
    problem: &str, condition: &str, model: &str,
    runtime_accepted: bool, post_hoc_verified: bool, start: Instant,
    gp_tokens: u64, gp_nodes: usize, tx_count: u64,
    tool_dist: Option<HashMap<String, u32>>,
    unique_payload_ratio: Option<f64>,
    gp_payload: Option<String>,
    gp_path: Option<String>,
    gp_proof_file: Option<String>,
    total_run_token_count: Option<u64>,
    failed_branch_count: Option<u32>,
    total_wall_time_ms: Option<u64>,
    // Phase A atom A4 (decomposed metrics). All callers must pass
    // explicit values — the v2 fields are non-Optional.
    hit_max_tx: bool,
    distinct_proposals: u64,
    total_proposals: u64,
    verifier_wait_ms: u64,
    // Phase A atom A5 (FC2-N22 budget regime stamp). Caller declares
    // the regime + base BEFORE the loop so MaxTxExhausted rows are
    // unambiguous about which partitioning rule produced them.
    budget_regime: minif2f_v4::budget_regime::BudgetRegime,
    budget_max_transactions: u32,
    // A8e fix F1 (Codex#2 + Gemini Q4): run_id minted by caller (run_swarm
    // or run_oneshot) at function entry; passed in here so the v2 jsonl
    // row stamps the SAME identifier the FC events used. No more ms drift.
    run_id: &str,
) -> PputResult {
    // PPUT-CCL Phase B B4 (mid-term audit P0-A fix 2026-04-25):
    // make_pput is now PURELY computational. The caller MUST decide both
    // `runtime_accepted` (did the evaluator's runtime gate fire?) and
    // `post_hoc_verified` (did Lean independently confirm the proof?). The
    // prior implementation derived `post_hoc_verified = has_gp` internally,
    // which would have laundered Phase C Soft Law fake-accepts into the
    // North Star pput_verified. Forcing the caller to pass both legs makes
    // Soft Law's design point unmissable: any caller that fakes runtime
    // accept must explicitly pass post_hoc_verified=verify_post_hoc(...)
    // or the divergence will surface immediately.
    //
    // Phase B all callers pass `(runtime_accepted, post_hoc_verified) = (X, X)`
    // because runtime IS Lean today. Phase C diverges at the Soft Law
    // mode call site, not inside this function.
    let has_gp = runtime_accepted; // legacy `has_golden_path` field semantics
    let elapsed = start.elapsed().as_secs_f64();
    let pput = if has_gp && elapsed > 0.0 { 100.0 / elapsed } else { 0.0 };
    // C-012 provenance: populated from env vars; None when unset (backward compat).
    let build_sha = std::env::var("BUILD_SHA").ok();
    let classifier_version = std::env::var("CLASSIFIER_VERSION").ok();
    let boltzmann_seed = std::env::var("BOLTZMANN_SEED")
        .ok().and_then(|s| s.parse::<u64>().ok());

    // Mid-term audit P0-B fix 2026-04-25: collapse Optional accumulator/clock
    // values into required v2 fields. Phase B always has values for these
    // (B2 + B3 wire them at every emit site); the prior Option wrapping was
    // overly defensive and let the v2 schema slip from the contract.
    let c_i = total_run_token_count.unwrap_or(0);
    let t_i = total_wall_time_ms.unwrap_or(0);
    let failed_count = failed_branch_count.unwrap_or(0);

    let progress_runtime = compute_progress_runtime(runtime_accepted);
    let progress_verified =
        compute_progress_verified(runtime_accepted, post_hoc_verified);
    let pput_runtime = compute_pput(progress_runtime, c_i, t_i);
    let pput_verified = compute_pput(progress_verified, c_i, t_i);
    let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i);

    // V2 fields read from env (per-process globals).
    let split = std::env::var("SPLIT").unwrap_or_else(|_| {
        eprintln!("[v2-emit] SPLIT env unset; defaulting to 'adaptation' \
                   (Phase B convention; pre-registration requires SPLIT \
                   for Phase C+ ablation runs)");
        "adaptation".to_string()
    });
    let mode = std::env::var("MODE").unwrap_or_else(|_| "full".to_string());
    let model_snapshot = std::env::var("MODEL_SNAPSHOT")
        .unwrap_or_else(|_| model.to_string());
    let git_sha = build_sha.clone().unwrap_or_default();
    let binary_sha256 = std::env::var("BINARY_SHA256").unwrap_or_default();

    // problem_id = basename without .lean
    let problem_id = std::path::Path::new(problem)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(problem)
        .to_string();

    PputResult {
        // ── B1 v2 schema fields ──
        schema_version: "v2.0".to_string(),
        // A8e fix F1: caller-supplied run_id (matches the FC-trace
        // correlation key emitted at every fc_event! site). No more
        // ms drift between the two identifiers.
        run_id: run_id.to_string(),
        problem_id,
        solved: runtime_accepted,
        split,
        verified: post_hoc_verified,
        golden_path_token_count: gp_tokens,
        total_run_token_count: c_i,
        total_wall_time_ms: t_i,
        progress: progress_verified,
        pput_runtime,
        pput_verified,
        pput_m_verified,
        failed_branch_count: failed_count,
        // Phase B placeholders — Phase C+ wires these as the modes activate.
        rollback_count: 0,
        hit_max_tx,
        tactic_diversity: minif2f_v4::jsonl_schema::compute_tactic_diversity(
            distinct_proposals, total_proposals,
        ),
        verifier_wait_ms,
        budget_regime: budget_regime.label().to_string(),
        budget_max_transactions,
        far: 0.0, err: 0.0, iac: 0.0, cpr: 0.0,
        model_snapshot,
        git_sha,
        binary_sha256,
        mode,
        // ── Legacy diagnostic fields ──
        problem: problem.to_string(),
        condition: condition.to_string(),
        model: model.to_string(),
        has_golden_path: has_gp,
        time_secs: elapsed,
        pput,
        gp_token_count: gp_tokens,
        gp_node_count: gp_nodes,
        tx_count,
        build_sha,
        classifier_version,
        boltzmann_seed,
        tool_dist,
        unique_payload_ratio,
        gp_payload,
        gp_path,
        gp_proof_file,
        // B7-extra: only the calibration-treatment short-circuit site mutates
        // this to Some(true). Default = None (most callers).
        synthetic_short_circuit: None,
    }
}

/// Phase 0 (C-039 candidate): persist a self-contained, re-verifiable proof artifact.
/// Writes <EXPERIMENT_DIR>/proofs/<theorem>_<timestamp>_<short_hash>.lean containing
/// the exact code that the Lean oracle accepted. An external verifier can run
/// `lean --stdin < <file>` with the matching toolchain + Mathlib and reproduce the result.
/// Returns the relative path (for embedding in PputResult) or None on I/O failure.
fn persist_proof_artifact(
    problem_file: &str, theorem_name: &str, problem_statement: &str,
    full_proof: &str, path_choice: &str, agent_id: &str,
) -> Option<String> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let exp_dir = std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into());
    let proofs_dir = std::path::Path::new(&exp_dir).join("proofs");
    if let Err(e) = std::fs::create_dir_all(&proofs_dir) {
        log::warn!("[audit] cannot create proofs dir {:?}: {}", proofs_dir, e);
        return None;
    }
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let mut h = DefaultHasher::new();
    full_proof.hash(&mut h);
    let short = format!("{:x}", h.finish() & 0xFFFFFFFF);
    let fname = format!("{}_{}_{}.lean", theorem_name, ts, short);
    let path = proofs_dir.join(&fname);
    let header = format!(
        "-- TuringOS v4 Phase 0 audit artifact (C-039 candidate)\n\
         -- problem_file: {}\n\
         -- theorem: {}\n\
         -- path_choice: {} (alone | tape+payload)\n\
         -- accepted_by_agent: {}\n\
         -- timestamp_unix: {}\n\
         -- Reproduce: LEAN_PATH=<mathlib paths> lean --stdin < this_file\n\
         --\n",
        problem_file, theorem_name, path_choice, agent_id, ts
    );
    let body = format!("{}\n{}\n{}", header, problem_statement, full_proof);
    match std::fs::write(&path, body) {
        Ok(_) => Some(format!("proofs/{}", fname)),
        Err(e) => {
            log::warn!("[audit] cannot write proof artifact {:?}: {}", path, e);
            None
        }
    }
}

/// A2 (Phase A engineering atom 2 of 8): swarm-condition parser.
///
/// Returns `Some(N)` if `condition` matches `n<digits>` for any positive
/// integer N (including N=1, the swarm_N=1 baseline). Returns `None` for
/// `oneshot`, `hybrid_v1`, malformed (`n-1`, `nfoo`, ``, etc).
///
/// Per Plan-agent NEXT-1 / Codex E0 / Gemini E1-Prime brainstorm
/// (handover/brainstorms/): EVERY N-curve experiment in the 2026-04-25
/// N-experiments arc MUST use `n1` (not `oneshot`) as the N=1 baseline,
/// because `oneshot` skips the swarm loop, tape, mr ticks, and ∏p
/// product. Without this discrimination, any N→PPUT curve confounds
/// "agent count effect" with "different runtime architecture".
///
/// FC-trace: FC2-N16 InitAI orchestration entry — discriminates between
/// the two registered InitAI shapes (oneshot vs swarm). FC1-N11 ∏p path
/// is reached only via swarm (n*) condition.
pub(crate) fn parse_swarm_condition_n(condition: &str) -> Option<usize> {
    if !condition.starts_with('n') { return None; }
    let rest = &condition[1..];
    if rest.is_empty() { return None; }
    rest.parse::<usize>().ok().filter(|&n| n >= 1)
}

#[cfg(test)]
mod swarm_condition_tests {
    use super::parse_swarm_condition_n;

    #[test]
    fn parses_valid_n_swarm_conditions() {
        assert_eq!(parse_swarm_condition_n("n1"), Some(1));   // swarm_N=1 baseline
        assert_eq!(parse_swarm_condition_n("n3"), Some(3));   // current default swarm size
        assert_eq!(parse_swarm_condition_n("n8"), Some(8));   // hetero candidate size
        assert_eq!(parse_swarm_condition_n("n16"), Some(16)); // upper N for stress test
        assert_eq!(parse_swarm_condition_n("n100"), Some(100));
    }

    #[test]
    fn rejects_oneshot_condition() {
        // Critical: 'oneshot' MUST NOT parse as a swarm condition.
        // It's a different code path (single LLM call, no tape, no
        // ∏p product). The N-experiments arc relies on this distinction.
        assert_eq!(parse_swarm_condition_n("oneshot"), None);
    }

    #[test]
    fn rejects_hybrid_v1_and_other_named_conditions() {
        assert_eq!(parse_swarm_condition_n("hybrid_v1"), None);
        assert_eq!(parse_swarm_condition_n("full"), None);
        assert_eq!(parse_swarm_condition_n("soft_law"), None);
        assert_eq!(parse_swarm_condition_n("panopticon"), None);
        assert_eq!(parse_swarm_condition_n("amnesia"), None);
        assert_eq!(parse_swarm_condition_n("homogeneous"), None);
    }

    #[test]
    fn rejects_malformed_n_conditions() {
        assert_eq!(parse_swarm_condition_n(""), None);          // empty
        assert_eq!(parse_swarm_condition_n("n"), None);         // just prefix
        assert_eq!(parse_swarm_condition_n("nfoo"), None);      // non-digit
        assert_eq!(parse_swarm_condition_n("n-1"), None);       // negative (parses fail on usize)
        assert_eq!(parse_swarm_condition_n("n0"), None);        // zero (filter rejects)
        assert_eq!(parse_swarm_condition_n("n 3"), None);       // whitespace
        assert_eq!(parse_swarm_condition_n("3"), None);         // missing 'n' prefix
        assert_eq!(parse_swarm_condition_n("N3"), None);        // case-sensitive
    }

    #[test]
    fn n1_is_distinct_from_oneshot() {
        // The discriminant test: n1 and oneshot are different conditions
        // even though both run with effectively 1 agent. The PARSER
        // returns Some(1) for n1 and None for oneshot, which routes
        // them to different code paths in main().
        assert_eq!(parse_swarm_condition_n("n1"), Some(1));
        assert_eq!(parse_swarm_condition_n("oneshot"), None);
    }
}

#[cfg(test)]
mod v2_emit_tests {
    use super::*;
    use minif2f_v4::jsonl_schema::RunRecord;
    use std::sync::Mutex;

    // Per feedback_env_var_test_lock: tests that mutate process-global env
    // vars must serialize to survive cargo's parallel runner.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    /// Mid-term audit P0-B fix conformance:
    /// Every emitted PputResult row must dispatch as `RunRecord::V2(_)`,
    /// not `RunRecord::Legacy(_)`. The pre-fix evaluator emitted rows with
    /// no `schema_version` field, which forced B1's dispatcher to classify
    /// new B2-B4 output as Legacy + extras, silently breaking the v2 schema
    /// contract. This test fails the build if a future change drops the
    /// `schema_version` stamp or any required v2 field.
    #[test]
    fn test_emit_dispatches_as_v2() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var("SPLIT", "adaptation");
        std::env::set_var("MODE", "full");

        // Phase B success path: runtime + post-hoc both fired.
        let result = make_pput(
            "test_problem.lean", "oneshot", "deepseek-v4-flash",
            true, true, Instant::now(),
            500, 1, 1,
            None, None, None, None, None,
            Some(2000), Some(0), Some(15_000),
            // A4: oneshot success — no max-tx, 1/1 unique, 4500ms in Lean.
            false, 1, 1, 4_500,
            // A5: oneshot stamps total_proposal + base=1 (single LLM call).
            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 1,
            "test_run_id",
        );

        let line = serde_json::to_string(&result).expect("serialize PputResult");

        // Schema discriminator must be present.
        assert!(
            line.contains("\"schema_version\":\"v2.0\""),
            "v2 emit must stamp schema_version=v2.0; got: {}",
            line
        );

        // Round-trip via RunRecord::from_json — must dispatch to V2.
        match RunRecord::from_json(&line).expect("v2 line parses") {
            RunRecord::V2(agg) => {
                assert_eq!(agg.schema_version, "v2.0");
                assert_eq!(agg.split, "adaptation");
                assert_eq!(agg.mode, "full");
                assert_eq!(agg.solved, true);
                assert_eq!(agg.verified, true);
                assert_eq!(agg.progress, 1u8);
                assert_eq!(agg.total_run_token_count, 2000);
                assert_eq!(agg.total_wall_time_ms, 15_000);
                assert!(agg.pput_verified > 0.0);
                assert_eq!(agg.pput_runtime, agg.pput_verified,
                    "Phase B: runtime IS Lean — pput_runtime must equal pput_verified");
                // A4 fields round-trip through emit.
                assert_eq!(agg.hit_max_tx, false);
                assert_eq!(agg.tactic_diversity, 1.0);
                assert_eq!(agg.verifier_wait_ms, 4_500);
                assert!(agg.verifier_wait_ms <= agg.total_wall_time_ms,
                    "A4 invariant: verifier_wait_ms must not exceed total_wall_time_ms");
            }
            RunRecord::Legacy(_) => panic!(
                "v2 emit MUST dispatch to RunRecord::V2, not Legacy. \
                 Schema-v2 contract regression — see B5 deferral checklist P0-B. \
                 Line was: {}", line
            ),
        }

        std::env::remove_var("SPLIT");
        std::env::remove_var("MODE");
    }

    /// Mid-term audit P0-B fix conformance (Soft Law H1 detection at the
    /// emit boundary): when runtime accepts but post-hoc Lean rejects, the
    /// emitted v2 row must show progress=0 and pput_verified=0 even with
    /// pput_runtime > 0. This is the divergence signal Phase C will scan.
    #[test]
    fn test_emit_soft_law_divergence_signal() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var("SPLIT", "adaptation");
        std::env::set_var("MODE", "soft_law");

        // Synthetic Soft Law-style emit: runtime says yes, Lean says no.
        let result = make_pput(
            "test_problem.lean", "oneshot", "deepseek-v4-flash",
            /*runtime_accepted=*/ true,
            /*post_hoc_verified=*/ false,
            Instant::now(),
            500, 1, 1,
            None, None, None, None, None,
            Some(2000), Some(0), Some(15_000),
            // A4: same shape as success path; A4 fields are independent
            // of the H1 divergence signal we're testing here.
            false, 1, 1, 4_500,
            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 1,
            "test_run_id",
        );

        assert_eq!(result.progress, 0u8,
            "Lean rejected → progress MUST be 0 (North Star truth)");
        assert_eq!(result.verified, false);
        assert!(result.pput_runtime > 0.0,
            "pput_runtime inflates under runtime accept (the divergence signal)");
        assert_eq!(result.pput_verified, 0.0,
            "pput_verified MUST collapse to 0 when Lean rejects");
        assert!(result.pput_runtime - result.pput_verified > 0.0,
            "(pput_runtime - pput_verified) > 0 ⟺ Soft Law divergence detected");

        std::env::remove_var("SPLIT");
        std::env::remove_var("MODE");
    }

    /// Phase A atom A4 conformance: max-tx exhaustion path stamps
    /// `hit_max_tx=true` AND splits `solve_rate` from `tokens_per_solve`
    /// + `time_per_solve` correctly (per Gemini brainstorm 2026-04-25
    /// § A.4). This is the "swarm spent the budget but didn't solve"
    /// row that downstream analysis must distinguish from OMEGA accept.
    #[test]
    fn test_a4_emit_max_tx_exhaustion_row() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var("SPLIT", "adaptation");
        std::env::set_var("MODE", "full");

        // Synthetic max-tx exhaustion: 200 tx, neither leg fired, swarm
        // proposed 50 unique payloads out of 200 tries (collision rate
        // typical of mid-N swarm on a hard problem).
        let result = make_pput(
            "test_problem.lean", "n3", "deepseek-v4-flash",
            false, false, Instant::now(),
            0, 0, 200,
            None, None, None, None, None,
            Some(8_000), Some(199), Some(120_000),
            true, 50, 200, 90_000,
            // A5: canonical Phase B baseline = total_proposal × 200.
            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 200,
            "test_run_id",
        );

        let line = serde_json::to_string(&result).expect("serialize PputResult");
        match RunRecord::from_json(&line).expect("v2 line parses") {
            RunRecord::V2(agg) => {
                // Decomposed-metric rule (Gemini brainstorm): on a max-tx
                // exhaustion, solve_rate=0 but tokens_per_solve / time_per_solve
                // are UNDEFINED (not 0). The contract here is that progress=0
                // → pput_verified=0, and downstream analysis must filter on
                // progress before averaging tokens/time.
                assert_eq!(agg.hit_max_tx, true);
                assert_eq!(agg.solved, false);
                assert_eq!(agg.progress, 0u8);
                assert_eq!(agg.pput_verified, 0.0);
                // tactic_diversity = 50/200 = 0.25 (notable correlation,
                // worth flagging — see C-036 unique_payload_ratio < 0.30
                // catastrophic-correlation threshold; A4 generalizes it).
                assert!((agg.tactic_diversity - 0.25).abs() < 1e-9);
                // verifier_wait_ms ≤ total_wall_time_ms invariant.
                assert!(agg.verifier_wait_ms <= agg.total_wall_time_ms);
                assert_eq!(agg.verifier_wait_ms, 90_000);
                assert_eq!(agg.total_wall_time_ms, 120_000);
            }
            RunRecord::Legacy(_) => panic!(
                "A4 max-tx row MUST dispatch to RunRecord::V2"
            ),
        }

        std::env::remove_var("SPLIT");
        std::env::remove_var("MODE");
    }

    /// Phase A atom A4 conformance: B7-extra synthetic short-circuit
    /// MUST NOT set hit_max_tx=true. The two exit paths look identical
    /// at `tx_count` time but mean different things — synthetic exits
    /// EARLY at the rollback threshold (~50 tx) and is tagged via
    /// `synthetic_short_circuit`; natural exhaustion runs the full
    /// 200 tx and is tagged via `hit_max_tx`. Conflating them
    /// neutralizes the calibration-treatment vs production split.
    #[test]
    fn test_a4_synthetic_short_circuit_does_not_set_hit_max_tx() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var("SPLIT", "adaptation");
        std::env::set_var("MODE", "full");

        // Mirror the synthetic short-circuit return shape (evaluator.rs
        // line ~622): hit_max_tx=false, then caller sets
        // synthetic_short_circuit=Some(true) on the result.
        let mut result = make_pput(
            "test_problem.lean", "n3", "deepseek-v4-flash",
            false, false, Instant::now(),
            0, 0, 50,
            None, None, None, None, None,
            Some(2_000), Some(49), Some(40_000),
            false, 20, 50, 25_000,
            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 200,
            "test_run_id",
        );
        result.synthetic_short_circuit = Some(true);

        let line = serde_json::to_string(&result).expect("serialize PputResult");
        match RunRecord::from_json(&line).expect("v2 line parses") {
            RunRecord::V2(agg) => {
                // The disambiguator: hit_max_tx stays false on the
                // synthetic-treatment row even though the run did not
                // OMEGA. synthetic_short_circuit lives in the legacy
                // diagnostic envelope (not in v2 RunAggregate); the
                // raw `line` carries it for downstream tools.
                assert_eq!(agg.hit_max_tx, false,
                    "synthetic short-circuit MUST NOT set hit_max_tx — it exits EARLY");
            }
            RunRecord::Legacy(_) => panic!("A4 short-circuit row must dispatch as v2"),
        }
        assert!(line.contains("\"synthetic_short_circuit\":true"),
            "synthetic short-circuit must remain visible on the raw row");

        std::env::remove_var("SPLIT");
        std::env::remove_var("MODE");
    }
}
