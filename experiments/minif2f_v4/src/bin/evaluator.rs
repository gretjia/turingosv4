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
use std::sync::{Arc, Mutex};
use std::time::Instant;
use log::{info, warn, error};
use rand::SeedableRng;
use rand::rngs::StdRng;

/// TB-1 Day-1 spike (2026-04-29): hex digest of an LLM prompt body.
/// Used as `PputResult.prompt_context_hash` so Phase D CCL can join
/// prompt-context → outcome trajectories without leaking the prompt
/// itself. Day-1 uses `DefaultHasher` (same non-cryptographic hash
/// already used for proof-artifact filenames at `persist_proof_artifact`)
/// to avoid a new direct sha2 dep that would mutate the workspace
/// `Cargo.lock` and trip the Trust Root gate (genesis_payload.toml is
/// STEP_B-protected). Day-4 upgrades to SHA-256 in the same commit
/// that re-hashes the Trust Root manifest with sudo authorization.
///
/// TRACE_MATRIX FC1-N12: oracle scope — the prompt is the pre-Lean
/// step-1 proposal input; this hash makes it auditable from the v2 jsonl
/// row alone.
fn prompt_hash_hex(prompt_body: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    let mut h = DefaultHasher::new();
    prompt_body.hash(&mut h);
    format!("{:016x}", h.finish())
}

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
    /// TB-1 Day-1 spike (2026-04-29): hex digest of the agent prompt content
    /// delivered to the LLM in this run. Populated at the prompt-build site
    /// (run_oneshot today; run_swarm in subsequent TB-1 days). Phase D CCL
    /// consumer joins prompt-context → outcome trajectories on this hash;
    /// equality across runs of the same problem indicates retrieval-equivalence
    /// (no capability compilation occurred), inequality indicates that some
    /// step-4 component injected new context (winning tactic, peer payload,
    /// past gp_payload). Optional for legacy compat; emit-side guarantees
    /// presence at every prompt-build site by TB-1 Day 4.
    ///
    /// Day-1 hash = DefaultHasher (16-char hex, non-cryptographic) to keep
    /// workspace `Cargo.lock` stable for the Trust Root gate. Day-4 upgrades
    /// to SHA-256 (64-char hex) under sudo manifest re-hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_context_hash: Option<String>,
    /// TB-1 Day-1 spike (2026-04-29): held-out verified PPUT — the
    /// PREREG North Star metric (`H-VPPUT`), computed as
    /// `pput_verified` of this run divided by the rolling mean of
    /// `pput_verified` across N=1-3 prior runs of the same problem
    /// (caller-supplied history). Day-1 stamps None — actual
    /// computation lands at TB-1 Day 4 once the per-problem history
    /// store + windowing rule are written. Optional so absence is
    /// explicit (vs. 0.0, which carries Goodhart-shield semantics).
    #[serde(skip_serializing_if = "Option::is_none")]
    h_vppu: Option<f64>,
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

    let mut args: Vec<String> = std::env::args().collect();

    // Phase C atom C1a (PREREG § 6 C1): extract --mode flag BEFORE
    // problem_file positional parsing. Resolve + validate against the
    // 5-mode enum + Phase A scope (Full only) BEFORE the first LLM
    // call, so a misconfigured --mode=soft_law (etc.) aborts startup
    // with a typed error instead of burning budget under the wrong
    // constitutional regime. CLI > MODE env > default Full.
    let mode_cli = minif2f_v4::experiment_mode::extract_mode_flag(&mut args);
    let resolved_mode = match minif2f_v4::experiment_mode::resolve_experiment_mode(
        mode_cli.as_deref(),
    ) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("evaluator: --mode validation failed: {e}");
            std::process::exit(1);
        }
    };
    // Stamp the resolved label back onto the MODE env var so the
    // existing make_pput reader (lib jsonl emit site) picks up the
    // validated value without further plumbing changes.
    std::env::set_var(
        minif2f_v4::experiment_mode::EXPERIMENT_MODE_ENV_VAR,
        resolved_mode.label(),
    );

    if args.len() < 2 {
        eprintln!("Usage: evaluator [--mode <mode>] <problem_file.lean>");
        eprintln!("  --mode: full|panopticon|amnesia|soft_law|homogeneous (default: full)");
        eprintln!("          All 5 modes wired post-C1e for Phase C ablation per PREREG § 6 C1.");
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
    info!("Problem: {} | Condition: {} | Model: {} | Mode: {}",
          problem_file, condition, model, resolved_mode.label());

    let mut result = match condition.as_str() {
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

    // TB-1 Day-4 (2026-04-29): stamp h_vppu by querying the persisted
    // per-problem rolling history of pput_verified, then record the
    // current run's pput_verified for future invocations. Order is
    // load → query (excluding current) → stamp → record → save, so
    // the current run does NOT self-reference its own value when
    // computing the ratio.
    //
    // Storage: $EXPERIMENT_DIR/h_vppu_history.json (or cwd if unset).
    // Failure to load/save degrades quietly — h_vppu is a P6 non-
    // blocking metric per recharter Day-5 Tier-B. Saving failure logs
    // a warning but never aborts the run.
    let h_vppu_path = std::path::PathBuf::from(
        std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into()),
    )
    .join("h_vppu_history.json");
    let mut h_vppu_history =
        minif2f_v4::h_vppu_history::HVppuHistory::load_from(&h_vppu_path);
    result.h_vppu = h_vppu_history.h_vppu_for(&result.problem_id, result.pput_verified);
    h_vppu_history.record(&result.problem_id, result.pput_verified);
    if let Err(e) = h_vppu_history.save_to(&h_vppu_path) {
        log::warn!(
            "[h_vppu_history] save to {:?} failed: {}; next run will start without prior history",
            h_vppu_path, e
        );
    }

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

    // Phase C atom C1b: resolve experiment mode once at function entry
    // from the MODE env (validated by main() at startup, so this can
    // only fail under deliberate process-global tampering after the
    // gate; expect-unwrap is correct). Used at every make_pput call
    // site below via `apply_mode_to_accept(mode, lean_rt, lean_ph)`.
    let mode = minif2f_v4::experiment_mode::parse_experiment_mode(
        &std::env::var(minif2f_v4::experiment_mode::EXPERIMENT_MODE_ENV_VAR)
            .unwrap_or_default(),
    ).expect("MODE env validated at main() startup");

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

    // TB-1 Day-1 spike (2026-04-29): hash the assembled prompt body BEFORE the
    // LLM call. Stamped onto every PputResult produced below so Phase D CCL
    // can join run rows on `prompt_context_hash` without touching the prompt
    // body. Same hash on retried oneshot of the same problem ⟹ no step-4
    // capability compilation occurred yet (TB-1 acceptance test 5 watches
    // this evolve once swarm is wired).
    let prompt_hash = prompt_hash_hex(&prompt);
    let stamp = |mut r: PputResult| -> PputResult {
        r.prompt_context_hash = Some(prompt_hash.clone());
        r
    };

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
                // Fence reject = neither Lean leg fired (no proposal to verify).
                // C1b: route through apply_mode_to_accept; Soft Law turns
                // this into (true, false) — fakes runtime accept on garbage
                // payload, post-hoc reflects "no Lean truth observed".
                // A4: no Lean call reached → verifier_wait_ms=0;
                // 1 proposal made (the LLM response), 1 distinct.
                let (rt, ph) = minif2f_v4::experiment_mode::apply_mode_to_accept(
                    mode, false, false,
                );
                return stamp(make_pput(problem_file, "oneshot", model,
                                 rt, ph, start, 0, 0, 1,
                                 None, None, None, None, None,
                                 acc.total_run_token_count(),
                                 acc.failed_branch_count,
                                 wc.elapsed_ms().unwrap_or(0),
                                 false, 1, 1, verifier_wait_ms,
                                 oneshot_regime, oneshot_budget_base, &run_id));
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
                    // so both legs hold. C1b: apply_mode_to_accept passes
                    // (true, true) through unchanged for Full + SoftLaw alike.
                    let (rt, ph) = minif2f_v4::experiment_mode::apply_mode_to_accept(
                        mode, true, true,
                    );
                    stamp(make_pput(problem_file, "oneshot", model,
                              rt, ph, start, gp_tokens, 1, 1,
                              None, None, Some(response.content.clone()),
                              Some("alone".to_string()), proof_file,
                              acc.total_run_token_count(),
                              acc.failed_branch_count,
                              wc.elapsed_ms().unwrap_or(0),
                              false, 1, 1, verifier_wait_ms,
                              oneshot_regime, oneshot_budget_base, &run_id))
                }
                Ok(false) => {
                    // Lean rejected → Full: (false, false). SoftLaw: (true, false).
                    // C1b H1 DETECTION POINT — Soft Law's pput_runtime > 0 with
                    // pput_verified = 0 originates here.
                    let (rt, ph) = minif2f_v4::experiment_mode::apply_mode_to_accept(
                        mode, false, false,
                    );
                    stamp(make_pput(problem_file, "oneshot", model,
                              rt, ph, start, 0, 0, 1,
                              None, None, None, None, None,
                              acc.total_run_token_count(),
                              acc.failed_branch_count,
                              wc.elapsed_ms().unwrap_or(0),
                              false, 1, 1, verifier_wait_ms,
                              oneshot_regime, oneshot_budget_base, &run_id))
                }
                Err(e) => {
                    warn!("Oracle error: {}", e);
                    // Lean error → measurement failure → Full: neither leg.
                    // C1b: SoftLaw still fakes runtime accept; ph stays false
                    // because Lean didn't deliver a verdict.
                    let (rt, ph) = minif2f_v4::experiment_mode::apply_mode_to_accept(
                        mode, false, false,
                    );
                    stamp(make_pput(problem_file, "oneshot", model,
                              rt, ph, start, 0, 0, 1,
                              None, None, None, None, None,
                              acc.total_run_token_count(),
                              acc.failed_branch_count,
                              wc.elapsed_ms().unwrap_or(0),
                              false, 1, 1, verifier_wait_ms,
                              oneshot_regime, oneshot_budget_base, &run_id))
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

    // Phase C atom C1b: resolve experiment mode once at function entry
    // from the MODE env (validated by main() at startup). Used at every
    // make_pput call site below via apply_mode_to_accept.
    let mode = minif2f_v4::experiment_mode::parse_experiment_mode(
        &std::env::var(minif2f_v4::experiment_mode::EXPERIMENT_MODE_ENV_VAR)
            .unwrap_or_default(),
    ).expect("MODE env validated at main() startup");

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

    // TB-6 Atom 1.3: chaintape mode (TURINGOS_CHAINTAPE_PATH).
    // When set, build a production-mode Sequencer + Git2LedgerWriter (L4) +
    // JSONL-backed RejectionEvidenceWriter (L4.E) + driver wrapper, and route
    // bus construction through TuringBus::with_sequencer instead of the legacy
    // WAL_DIR / TuringBus::new paths. Both env vars set → chain wins; WAL_DIR
    // is silently disabled with an info!() log per preflight v2.1 §3.6.
    // Bundle is held across the run; bundle.shutdown().await is invoked at
    // the implicit final return to drain queued submissions.
    // TB-7 Atom 1.7 (Codex audit cc7b3dd action item #1): fail-closed when
    // TURINGOS_CHAINTAPE_PATH is set but bootstrap fails. Silent fallback
    // to legacy mode is the same anti-pattern as legacy `bus.append` as
    // authoritative state mutation (TB-7 charter §4.0 + §6 #31). When the
    // operator declares ChainTape mode, we either get ChainTape or we
    // exit non-zero — never quietly degrade to legacy.
    // TB-7.7 D3: optional pre-seed for L4 accept. Reading
    // TURINGOS_CHAINTAPE_PRESEED=1 enables a custom genesis QState with
    // pre-seeded balances for: (a) `tb7-7-sponsor` (for TaskOpen +
    // EscrowLock), and (b) every Agent_i (for WorkTx.stake admission).
    // Without preseed, real LLM WorkTx with non-zero stake would fail
    // admission with InsufficientBalance → L4.E. With preseed, the
    // chain shows ≥1 accepted L4 WorkTx for the first time.
    let chaintape_preseed_enabled = std::env::var("TURINGOS_CHAINTAPE_PRESEED")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let chaintape_bundle: Option<turingosv4::runtime::ChaintapeBundle> =
        match turingosv4::runtime::RuntimeChaintapeConfig::from_env() {
            None => None, // env unset = legacy mode is the explicit choice
            Some(cfg) => {
                let result = if chaintape_preseed_enabled {
                    // Pre-seeded genesis: sponsor + per-agent balances.
                    // Sponsor balance = 10_000_000 micro (10 coins) — plenty
                    // for ~100 EscrowLock at 100_000 each.
                    // Per-agent balance = 1_000_000 micro (1 coin) — plenty
                    // for ~1000 WorkTx.stake at 1_000 each.
                    use turingosv4::economy::money::MicroCoin;
                    use turingosv4::state::q_state::AgentId;
                    let mut pairs: Vec<(AgentId, MicroCoin)> = vec![
                        (AgentId("tb7-7-sponsor".into()),
                         MicroCoin::from_micro_units(10_000_000)),
                    ];
                    // Pre-seed up to N=10 agents (covers n1, n3, n5, n10).
                    for i in 0..10 {
                        pairs.push((
                            AgentId(format!("Agent_{i}")),
                            MicroCoin::from_micro_units(1_000_000),
                        ));
                    }
                    let initial_q = turingosv4::runtime::adapter::genesis_with_balances(&pairs);
                    info!("[chaintape/d3] pre-seed enabled: sponsor + 10 agent balances");
                    turingosv4::runtime::build_chaintape_sequencer_with_initial_q(
                        &cfg, initial_q,
                    )
                } else {
                    turingosv4::runtime::build_chaintape_sequencer(&cfg)
                };
                match result {
                    Ok(b) => Some(b),
                    Err(e) => {
                        error!(
                            "[chaintape] bootstrap failed under TURINGOS_CHAINTAPE_PATH (declared \
                             ChainTape mode); exiting non-zero per TB-7 Atom 1.7 fail-closed \
                             (Codex audit action #1). Error: {e}"
                        );
                        std::process::exit(2);
                    }
                }
            }
        };
    if chaintape_bundle.is_some() && std::env::var("WAL_DIR").is_ok() {
        info!("[chaintape] WAL_DIR ignored when TURINGOS_CHAINTAPE_PATH is set");
    }

    // TB-7 Atom 2: per-run AgentKeypairRegistry holds run-local Ed25519
    // keypairs for every distinct agent_id that submits a real-LLM proposal
    // through bus.submit_typed_tx. Keys auto-generate on first use; pubkeys
    // are persisted to <runtime_repo>/agent_pubkeys.json so verify_chaintape
    // (Atom 4) can re-verify replayed signatures. Per ARCHITECT_RULING D2,
    // these are run-local identities, NOT durable reputation.
    //
    // Wrapped in Arc<Mutex<>> so the registry can be shared across the async
    // run loop (interior mutability needed for AgentKeypairRegistry::sign).
    let agent_keypairs: Option<Arc<Mutex<turingosv4::runtime::agent_keypairs::AgentKeypairRegistry>>> =
        chaintape_bundle.as_ref().map(|b| {
            let reg = turingosv4::runtime::agent_keypairs::AgentKeypairRegistry::open(
                &b.runtime_repo_path,
            )
            .expect(
                "[chaintape/atom2] agent_keypairs init must succeed (fresh runtime_repo guarantees \
                 manifest absent; if you see this on a non-fresh dir, see TB-6 NonEmptyRuntimeRepo)",
            );
            Arc::new(Mutex::new(reg))
        });

    // TB-7.7 D2: last submitted tx per agent (for ProposalTelemetry.parent_tx).
    // Map of agent_id → last tx_id submitted via bus.submit_typed_tx (Work or
    // Verify). Root proposals leave parent_tx = None; subsequent same-agent
    // proposals get the previous tx_id as parent. This is what unlocks
    // citation-tree / DAG-edge analysis on chain artifacts.
    let mut last_tx_by_agent: std::collections::HashMap<String, turingosv4::state::q_state::TxId> =
        std::collections::HashMap::new();

    // Phase 1: opt-in tape persistence via env. WAL_DIR=<dir> enables WAL
    // writes to <dir>/<problem>_<timestamp>.jsonl; resumes if file exists.
    // Default off for backward-compat baseline runs.
    let mut bus = if let Some(ref bundle) = chaintape_bundle {
        info!(
            "[chaintape] bus wired with Sequencer + on-disk ChainTape at {:?}",
            bundle.runtime_repo_path
        );
        TuringBus::with_sequencer(kernel, config, bundle.sequencer.clone())
    } else if let Ok(wal_dir) = std::env::var("WAL_DIR") {
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
    // TB-6 Atom 3: when chaintape mode is on, seed the on-disk chain with a
    // minimal pair of envelopes — one accepted TaskOpenTx (produces an L4
    // entry) and one rejected zero-stake WorkTx (produces an L4.E entry with
    // synthetic_rejection_for_l4e_gate=true label per architect ruling
    // 2026-05-01 § 3.6 Atom 3). The "real LLM" aspect is the parallel evaluator
    // run on the smoke problem; the synthetic seed satisfies the architect's
    // ≥1 L4 + ≥1 L4.E minimum without requiring per-proposal WorkTx routing
    // (deferred to a future TB).
    //
    // TB-6 Atom 5: each synthetic envelope is also recorded as an
    // AgentProposalRecord in CAS + indexed under tx_id in
    // agent_audit_trail.jsonl. This demonstrates the audit-trail surface
    // end-to-end on the production-binary path. Per-LLM-proposal main-loop
    // routing (run_swarm "append" branch hook) remains a deferred surface
    // — same pattern as Atom 3's deferral.
    if let Some(ref bundle) = chaintape_bundle {
        // TB-7.7 D3: when preseed is enabled, also submit a TaskOpen +
        // EscrowLock for "task-{run_id}" (the SAME task_id that real
        // Agent_i WorkTx submissions use in Atom 2/3 hot path). With
        // pre-seeded sponsor balance, the EscrowLock will succeed,
        // populating task_markets_t["task-{run_id}"].total_escrow > 0.
        // Combined with pre-seeded Agent_i balance, real LLM WorkTx
        // with stake > 0 can now reach L4 accepted.
        if chaintape_preseed_enabled {
            use turingosv4::economy::money::MicroCoin;
            let real_task_id = format!("task-{}", run_id);
            let task_open_real = turingosv4::runtime::adapter::make_synthetic_task_open(
                &real_task_id,
                "tb7-7-sponsor",
                turingosv4::state::q_state::Hash::ZERO,
                "tb7-7-d3-seed",
            );
            if let Err(e) = bus.submit_typed_tx(task_open_real).await {
                error!("[chaintape/d3] preseed TaskOpen submit failed: {e}");
            } else {
                info!("[chaintape/d3] preseed TaskOpen for {real_task_id}");
            }
            // Read escrow amount from env (default 100_000 micro = 0.1 coin).
            let escrow_micro: i64 = std::env::var("TURINGOS_CHAINTAPE_PRESEED_TASK_ESCROW_MICRO")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100_000);
            let escrow_lock = turingosv4::runtime::adapter::make_synthetic_escrow_lock(
                &real_task_id,
                "tb7-7-sponsor",
                escrow_micro,
                turingosv4::state::q_state::Hash::ZERO,
                "tb7-7-d3-escrow",
            );
            if let Err(e) = bus.submit_typed_tx(escrow_lock).await {
                error!("[chaintape/d3] preseed EscrowLock submit failed: {e}");
            } else {
                info!("[chaintape/d3] preseed EscrowLock {escrow_micro} micro for {real_task_id}");
            }
        }

        let task_id_str = format!("smoke-{}", run_id);
        let task_open = turingosv4::runtime::adapter::make_synthetic_task_open(
            &task_id_str,
            "tb6-smoke-sponsor",
            turingosv4::state::q_state::Hash::ZERO,
            "atom3-seed",
        );
        let task_open_tx_id =
            turingosv4::state::q_state::TxId(format!("taskopen-{}-atom3-seed", task_id_str));
        if let Err(e) = bus.submit_typed_tx(task_open).await {
            error!("[chaintape] synthetic TaskOpen submit failed: {e}");
        } else {
            info!("[chaintape] seeded synthetic TaskOpen for {}", task_id_str);
        }
        let bad_worktx = turingosv4::runtime::adapter::make_synthetic_worktx(
            &task_id_str,
            "tb6-smoke-agent",
            turingosv4::state::q_state::Hash::ZERO,
            0,
            "atom3-l4e-synthetic-rejection",
            true,
        );
        let bad_worktx_tx_id = turingosv4::state::q_state::TxId(format!(
            "worktx-{}-atom3-l4e-synthetic-rejection",
            task_id_str
        ));
        if let Err(e) = bus.submit_typed_tx(bad_worktx).await {
            error!("[chaintape] synthetic zero-stake WorkTx submit failed: {e}");
        } else {
            info!(
                "[chaintape] seeded synthetic zero-stake WorkTx \
                 (synthetic_rejection_for_l4e_gate=true) for {}",
                task_id_str
            );
        }
        // Mark the synthetic-seed in the evidence dir so verify_chaintape (Atom 4)
        // can distinguish synthetic-rejection from natural rejection.
        let label_path = bundle.runtime_repo_path.join("synthetic_rejection_label.json");
        let _ = std::fs::write(
            &label_path,
            format!(
                r#"{{"synthetic_rejection_for_l4e_gate": true, "run_id": "{}", "atom": "TB-6 Atom 3", "rationale": "≥1 L4.E entry seeded via zero-stake WorkTx; per architect ruling 2026-05-01 § 3.6 Atom 3"}}"#,
                run_id
            ),
        );

        // TB-6 Atom 5: write AgentProposalRecord pairs to CAS + index for both
        // synthetic envelopes. Each record carries the architect's 9 fields
        // + logical_t. The index links L4 / L4.E tx_id → CAS record CID.
        if let Err(e) = turingosv4::runtime::agent_audit_trail::write_synthetic_seed_audit_pair(
            &bundle.cas_path,
            &bundle.runtime_repo_path,
            &run_id,
            &task_open_tx_id,
            &bad_worktx_tx_id,
        ) {
            error!("[chaintape] Atom 5 audit-trail write failed: {e}");
        } else {
            info!(
                "[chaintape] Atom 5 audit-trail records written to CAS + indexed for {}",
                task_id_str
            );
        }
    }

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
    // C1c: skill resolution flows through experiment_mode::skill_index_for_agent
    // so the startup echo + per-tx skill lookup share one source of truth.
    // Homogeneous mode pins every agent to skill[0]; other modes cycle.
    let temp_ladder_on = std::env::var("TEMP_LADDER").ok().as_deref() == Some("1");
    let agent_cfg: Vec<String> = (0..n_agents).map(|i| {
        let s = minif2f_v4::experiment_mode::skill_index_for_agent(
            mode, i, agent_skills.len(),
        );
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
            // C1b: route accept legs through apply_mode_to_accept; under
            // SoftLaw the synthetic short-circuit also flips runtime to
            // true, contributing to the pput_runtime/pput_verified gap.
            let (rt, ph) = minif2f_v4::experiment_mode::apply_mode_to_accept(
                mode, false, false,
            );
            let mut result = make_pput(problem_file, &condition, &run_model_label,
                                       rt, ph, start, 0, 0,
                                       tx as u64, Some(tool_dist), None,
                                       None, None, None,
                                       acc.total_run_token_count(),
                                       acc.failed_branch_count,
                                       wc.elapsed_ms().unwrap_or(0),
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

        // C1e Amnesia: agent loses memory of L_t. Force the chain
        // projection to problem-statement-only (the same shape used
        // when snap.tape is genuinely empty), so every proposal is
        // re-derived from scratch every tx — ERR=0 + time/token
        // inflation are the H detection mechanisms. Internal
        // verification paths (tape+payload Lean re-verify) below
        // are NOT gated on is_amnesia; that's verifier mechanics,
        // not agent memory.
        let chain = if minif2f_v4::experiment_mode::is_amnesia(mode) || snap.tape.is_empty() {
            problem_statement.to_string()
        } else {
            let nodes: Vec<String> = snap.tape.time_arrow().iter()
                .filter_map(|id| snap.tape.get(id))
                .map(|n| format!("[{}] {}: {}", n.id, n.author, n.payload))
                .collect();
            format!("{}\n\n=== Proof Chain ===\n{}", problem_statement, nodes.join("\n"))
        };

        let errors = bus.recent_rejections(agent_id, 3);
        // Art. II.2.1: per-agent skill specialization + Librarian learned memory.
        // C1c: route skill index through experiment_mode helper so Homogeneous
        // mode pins every agent_idx to 0 (Paper-1 era A condition; H4 detection).
        let skill_idx = minif2f_v4::experiment_mode::skill_index_for_agent(
            mode, agent_idx, agent_skills.len(),
        );
        let base_skill = agent_skills.get(skill_idx).unwrap_or(&"");
        // C1d Panopticon: in cognitive-isolation-breach mode, the focal
        // agent's prompt receives the merged learned-memory of ALL agents,
        // labeled with each source agent_id. Context grows ~O(N) per tx
        // → tokens↑ → PPUT↓; H2 detection mechanism. Full / SoftLaw /
        // Homogeneous / Amnesia keep the per-agent fetch.
        let learned = if minif2f_v4::experiment_mode::is_panopticon(mode) {
            bus.tools.iter()
                .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
                .map(|lib| {
                    agent_ids.iter()
                        .filter_map(|a| lib.read_agent_memory(a).map(|m| format!("[{}] {}", a, m)))
                        .collect::<Vec<_>>()
                        .join("\n---\n")
                })
                .unwrap_or_default()
        } else {
            bus.tools.iter()
                .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
                .and_then(|lib| lib.read_agent_memory(agent_id))
                .unwrap_or_default()
        };
        let skill = if learned.is_empty() {
            base_skill.to_string()
        } else {
            format!("{}\n\n{}", base_skill, learned)
        };
        // A8e14 R2 (Gemini R12): when an agent hits SEARCH_CAP we strip the
        // search tool — but pre-R2 the cached hits from its last search kept
        // appearing in every subsequent prompt, leaving the agent reasoning
        // from stale results for the rest of the run. Single cap_hit gate
        // for both the tool list AND the cache injection.
        let cap_hit = search_count.get(agent_id).copied().unwrap_or(0) >= search_cap;
        let hits_ref: Vec<String> = if cap_hit {
            Vec::new()
        } else {
            search_cache.get(agent_id).cloned().unwrap_or_default()
        };
        let tools_desc = if cap_hit {
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

                                // ── TB-7 Atom 2: AUTHORITATIVE per-LLM-proposal routing ──
                                //
                                // Real LLM proposal → ProposalTelemetry CAS object →
                                // real-signature WorkTx → bus.submit_typed_tx → Sequencer →
                                // L4 (accepted) or L4.E (rejected). This is the Frame B
                                // closure path per TB-7 charter §4.0 + §8 Gate 1.
                                //
                                // Authoritative for ChainTape state (L4 captures the
                                // proposal byte-deterministically). The bus.append call
                                // BELOW is shadow_only (kernel.tape view sync for the next
                                // agent's prompt context — NOT canonical state).
                                // TB-7.5 fix #1 (Codex audit 492e86c action #1, BLOCKING):
                                // FAIL-CLOSED authoritative routing. Any failure of
                                // q_snapshot / CAS open / proposal_telemetry write /
                                // make_real_worktx_signed_by / submit_typed_tx exits
                                // the evaluator with code 3 and an error message —
                                // shadow_only kernel.tape sync MUST NOT be the only
                                // state mutation after an authoritative-path failure
                                // in ChainTape mode. Per TB-7 §4.0 + §6 #31.
                                if let (Some(bundle), Some(reg)) =
                                    (chaintape_bundle.as_ref(), agent_keypairs.as_ref())
                                {
                                    let q = match bundle.sequencer.q_snapshot() {
                                        Ok(q) => q,
                                        Err(e) => {
                                            error!("[chaintape/atom2] FAIL-CLOSED: q_snapshot failed under ChainTape mode: {e:?}");
                                            std::process::exit(3);
                                        }
                                    };
                                    let parent_state_root = q.state_root_t;
                                    let logical_t = bundle.sequencer.next_logical_t_peek();
                                    let task_id_str = format!("task-{}", run_id);

                                    // TB-7.7 D1: open CAS FIRST so build_for_evaluator_append
                                    // can durably store proposal_artifact_cid.
                                    let mut cas_store = match turingosv4::bottom_white::cas::store::CasStore::open(&bundle.cas_path) {
                                        Ok(c) => c,
                                        Err(e) => {
                                            error!("[chaintape/atom2] FAIL-CLOSED: cas open failed under ChainTape mode: {e}");
                                            std::process::exit(3);
                                        }
                                    };

                                    // TB-7.7 D2: parent_tx from last submission per agent (root if first).
                                    let parent_tx: Option<turingosv4::state::q_state::TxId> =
                                        last_tx_by_agent.get(agent_id).cloned();

                                    let pt = match turingosv4::runtime::proposal_telemetry::ProposalTelemetry::build_for_evaluator_append_with_parent(
                                        &mut cas_store,
                                        &run_id,
                                        agent_id,
                                        proposal_count as u64,
                                        payload.as_bytes(),
                                        "append",
                                        turingosv4::runtime::proposal_telemetry::TokenCounts {
                                            prompt_tokens: response.prompt_tokens as u64,
                                            completion_tokens: response.completion_tokens as u64,
                                            tool_tokens: 0,
                                        },
                                        "tb7-atom2-evaluator-payload",
                                        logical_t,
                                        parent_tx,
                                    ) {
                                        Ok(p) => p,
                                        Err(e) => {
                                            error!("[chaintape/atom2] FAIL-CLOSED: proposal_artifact CAS put failed: {e}");
                                            std::process::exit(3);
                                        }
                                    };

                                    let tel_cid = match turingosv4::runtime::proposal_telemetry::write_to_cas(
                                        &mut cas_store,
                                        &pt,
                                        "tb7-atom2-evaluator",
                                        logical_t,
                                    ) {
                                        Ok(c) => c,
                                        Err(e) => {
                                            error!("[chaintape/atom2] FAIL-CLOSED: proposal_telemetry CAS write failed: {e}");
                                            std::process::exit(3);
                                        }
                                    };
                                    let real_worktx = {
                                        let mut reg_guard = match reg.lock() {
                                            Ok(g) => g,
                                            Err(p) => p.into_inner(),
                                        };
                                        let suffix = format!("p{}", proposal_count);
                                        // TB-7.7 D3: stake from env (default 1000 micro-units = 0.001 coin)
                                        // for admission-gate clearance under pre-seeded escrow.
                                        // Pre-TB-7.7 stake was hardcoded 0 → all WorkTx → L4.E.
                                        let stake_micro: i64 = std::env::var("TURINGOS_CHAINTAPE_PROPOSAL_STAKE_MICRO")
                                            .ok()
                                            .and_then(|s| s.parse().ok())
                                            .unwrap_or(1_000);
                                        match turingosv4::runtime::adapter::make_real_worktx_signed_by(
                                            &mut *reg_guard,
                                            &task_id_str,
                                            agent_id,
                                            parent_state_root,
                                            stake_micro,
                                            &suffix,
                                            tel_cid,
                                            true,
                                            logical_t,
                                        ) {
                                            Ok(tx) => tx,
                                            Err(e) => {
                                                error!("[chaintape/atom2] FAIL-CLOSED: make_real_worktx_signed_by failed: {e}");
                                                std::process::exit(3);
                                            }
                                        }
                                    };
                                    // TB-7.7 D2: capture tx_id before move into submit_typed_tx.
                                    let real_worktx_tx_id = match &real_worktx {
                                        turingosv4::state::typed_tx::TypedTx::Work(w) => Some(w.tx_id.clone()),
                                        _ => None,
                                    };
                                    if let Err(e) = bus.submit_typed_tx(real_worktx).await {
                                        error!("[chaintape/atom2] FAIL-CLOSED: submit_typed_tx failed: {e:?}");
                                        std::process::exit(3);
                                    }
                                    // TB-7.7 D2: record this WorkTx as parent for next same-agent proposal.
                                    if let Some(tx_id) = real_worktx_tx_id {
                                        last_tx_by_agent.insert(agent_id.to_string(), tx_id);
                                    }
                                }

                                // shadow_only: kernel.tape view sync for next-agent prompt
                                // context. NOT authoritative state — the L4 chain above is
                                // canonical. This call exists so the in-memory tape used by
                                // the next iteration's prompt builder reflects this
                                // proposal. Per TB-7 §4.0 option (3) + §6 #31 inheritance,
                                // this is annotated shadow_only and does NOT constitute
                                // authoritative state mutation. Removal contingent on
                                // kernel.tape becoming L4-derived (post-MVP refactor).
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

                                        // ── TB-7 Atom 3: AUTHORITATIVE OMEGA-branch routing ──
                                        //
                                        // OMEGA accept (full proof) → WorkTx (predicate_passes=true)
                                        // + VerifyTx (verdict=Confirm) pair via bus.submit_typed_tx.
                                        // Per ARCHITECT_RULING D3 + charter §4.3: ChallengeWindow
                                        // stays OPEN; NO FinalizeRewardTx, NO SlashTx, NO
                                        // settlement (RSP-4 / TB-9 territory).
                                        // TB-7.5 fix #1 (Codex audit 492e86c action #1, BLOCKING):
                                        // FAIL-CLOSED authoritative routing for OMEGA full-proof
                                        // branch. Any failure exits the evaluator with code 3.
                                        if let (Some(bundle), Some(reg)) =
                                            (chaintape_bundle.as_ref(), agent_keypairs.as_ref())
                                        {
                                            let q = match bundle.sequencer.q_snapshot() {
                                                Ok(q) => q,
                                                Err(e) => {
                                                    error!("[chaintape/atom3-omega] FAIL-CLOSED: q_snapshot: {e:?}");
                                                    std::process::exit(3);
                                                }
                                            };
                                            let parent_state_root = q.state_root_t;
                                            let logical_t = bundle.sequencer.next_logical_t_peek();
                                            let task_id_str = format!("task-{}", run_id);
                                            // TB-7.7 D1: open CAS first.
                                            let mut cas_store = match turingosv4::bottom_white::cas::store::CasStore::open(&bundle.cas_path) {
                                                Ok(c) => c,
                                                Err(e) => {
                                                    error!("[chaintape/atom3-omega] FAIL-CLOSED: cas open: {e}");
                                                    std::process::exit(3);
                                                }
                                            };
                                            // TB-7.7 D2: parent_tx for branch lineage.
                                            let parent_tx_for_pt: Option<turingosv4::state::q_state::TxId> =
                                                last_tx_by_agent.get(agent_id).cloned();
                                            let pt_partial = match turingosv4::runtime::proposal_telemetry::ProposalTelemetry::build_for_evaluator_append_with_parent(
                                                &mut cas_store,
                                                &run_id,
                                                agent_id,
                                                proposal_count as u64,
                                                payload.as_bytes(),
                                                "complete",
                                                turingosv4::runtime::proposal_telemetry::TokenCounts {
                                                    prompt_tokens: response.prompt_tokens as u64,
                                                    completion_tokens: response.completion_tokens as u64,
                                                    tool_tokens: 0,
                                                },
                                                "tb7-atom3-omega-full-payload",
                                                logical_t,
                                                parent_tx_for_pt,
                                            ) {
                                                Ok(p) => p,
                                                Err(e) => {
                                                    error!("[chaintape/atom3-omega] FAIL-CLOSED: proposal_artifact CAS put: {e}");
                                                    std::process::exit(3);
                                                }
                                            };
                                            // TB-7.7 D4: build VerificationResult for the OMEGA-accept (Lean
                                            // accepted; verified=true). Deterministic work_tx_id is
                                            // `worktx-<task>-<suffix>` per make_real_worktx_signed_by.
                                            let suffix = format!("omega-full-{}", proposal_count);
                                            let work_tx_id_pre =
                                                turingosv4::state::q_state::TxId(format!(
                                                    "worktx-{}-{}",
                                                    task_id_str, suffix
                                                ));
                                            let vr = turingosv4::runtime::verification_result::VerificationResult::from_lean_run(
                                                work_tx_id_pre.clone(),
                                                turingosv4::state::q_state::AgentId(agent_id.into()),
                                                0, // OMEGA-accept = Lean exit 0
                                                pt_partial.proposal_artifact_cid,
                                                proof_file.as_deref().unwrap_or(""),
                                                payload.as_bytes(),
                                            );
                                            let vr_cid = match turingosv4::runtime::verification_result::write_to_cas(
                                                &mut cas_store,
                                                &vr,
                                                "tb7-atom3-omega-full-vr",
                                                logical_t,
                                            ) {
                                                Ok(c) => c,
                                                Err(e) => {
                                                    error!("[chaintape/atom3-omega] FAIL-CLOSED: VerificationResult CAS put: {e}");
                                                    std::process::exit(3);
                                                }
                                            };
                                            let pt = pt_partial.with_verification_result(vr_cid);
                                            let tel_cid = match turingosv4::runtime::proposal_telemetry::write_to_cas(
                                                &mut cas_store,
                                                &pt,
                                                "tb7-atom3-omega-full",
                                                logical_t,
                                            ) {
                                                Ok(c) => c,
                                                Err(e) => {
                                                    error!("[chaintape/atom3-omega] FAIL-CLOSED: telemetry CAS write: {e}");
                                                    std::process::exit(3);
                                                }
                                            };
                                            // TB-7.7 D3: stake from env (default 1000 micro-units).
                                            let stake_micro: i64 = std::env::var("TURINGOS_CHAINTAPE_PROPOSAL_STAKE_MICRO")
                                                .ok()
                                                .and_then(|s| s.parse().ok())
                                                .unwrap_or(1_000);
                                            let (work_tx, verify_tx) = {
                                                let mut reg_guard = match reg.lock() {
                                                    Ok(g) => g,
                                                    Err(p) => p.into_inner(),
                                                };
                                                let w = match turingosv4::runtime::adapter::make_real_worktx_signed_by(
                                                    &mut *reg_guard,
                                                    &task_id_str,
                                                    agent_id,
                                                    parent_state_root,
                                                    stake_micro,
                                                    &suffix,
                                                    tel_cid,
                                                    true,
                                                    logical_t,
                                                ) {
                                                    Ok(tx) => tx,
                                                    Err(e) => {
                                                        error!("[chaintape/atom3-omega] FAIL-CLOSED: make_real_worktx: {e}");
                                                        std::process::exit(3);
                                                    }
                                                };
                                                let work_tx_id = match &w {
                                                    turingosv4::state::typed_tx::TypedTx::Work(w) => w.tx_id.clone(),
                                                    _ => {
                                                        error!("[chaintape/atom3-omega] FAIL-CLOSED: make_real_worktx returned non-Work variant");
                                                        std::process::exit(3);
                                                    }
                                                };
                                                let v = match turingosv4::runtime::adapter::make_real_verifytx_signed_by(
                                                    &mut *reg_guard,
                                                    parent_state_root,
                                                    work_tx_id,
                                                    agent_id,
                                                    0,
                                                    &suffix,
                                                    true,
                                                    logical_t,
                                                ) {
                                                    Ok(tx) => tx,
                                                    Err(e) => {
                                                        error!("[chaintape/atom3-omega] FAIL-CLOSED: make_real_verifytx: {e}");
                                                        std::process::exit(3);
                                                    }
                                                };
                                                (w, v)
                                            };
                                            // TB-7.7 D2: capture tx_ids before move.
                                            let work_tx_id = match &work_tx {
                                                turingosv4::state::typed_tx::TypedTx::Work(w) => Some(w.tx_id.clone()),
                                                _ => None,
                                            };
                                            let verify_tx_id = match &verify_tx {
                                                turingosv4::state::typed_tx::TypedTx::Verify(v) => Some(v.tx_id.clone()),
                                                _ => None,
                                            };
                                            if let Err(e) = bus.submit_typed_tx(work_tx).await {
                                                error!("[chaintape/atom3-omega] FAIL-CLOSED: WorkTx submit_typed_tx: {e:?}");
                                                std::process::exit(3);
                                            }
                                            if let Err(e) = bus.submit_typed_tx(verify_tx).await {
                                                error!("[chaintape/atom3-omega] FAIL-CLOSED: VerifyTx submit_typed_tx: {e:?}");
                                                std::process::exit(3);
                                            }
                                            // TB-7.7 D2: VerifyTx is the most recent same-agent submission;
                                            // record it as parent for any subsequent same-agent proposal.
                                            // (For root-of-tree analysis the WorkTx is the true parent of
                                            // child branches; VerifyTx is the latest event chronologically.
                                            // We pick VerifyTx since it represents the latest LOGICAL_T
                                            // advance for this agent.)
                                            if let Some(tx_id) = verify_tx_id.or(work_tx_id) {
                                                last_tx_by_agent.insert(agent_id.to_string(), tx_id);
                                            }
                                        }

                                        // shadow_only: kernel.tape view sync for halt-and-settle +
                                        // GP traversal. NOT authoritative state — the L4 chain above
                                        // is canonical (WorkTx + VerifyTx pair). Per TB-7 §4.0
                                        // option (3) + §6 #31 inheritance.
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
                                        // (Ok((true, _))). Both legs hold. C1b: route through
                                        // apply_mode_to_accept; (true, true) passes through
                                        // unchanged for Full + SoftLaw alike at this site.
                                        let (rt, ph) = minif2f_v4::experiment_mode::apply_mode_to_accept(
                                            mode, true, true,
                                        );
                                        return make_pput(problem_file, &condition, &run_model_label,
                                                        rt, ph,
                                                        start, gp_tokens, gp_nodes, tx as u64 + 1,
                                                        Some(tool_dist), upr,
                                                        Some(full_proof.clone()),
                                                        Some(path_choice.to_string()),
                                                        proof_file,
                                                        acc.total_run_token_count(),
                                                        acc.failed_branch_count,
                                                        wc.elapsed_ms().unwrap_or(0),
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

                                        // ── TB-7 Atom 3: AUTHORITATIVE OMEGA-branch routing (per-tactic) ──
                                        //
                                        // PartialVerdict::Complete via step → WorkTx + VerifyTx pair.
                                        // Same shape as the full-proof OMEGA path above; the only
                                        // differences are gp_path label = "per_tactic" and the
                                        // proposal payload bytes are `tactic` (the closing step)
                                        // rather than `payload` (the full proof).
                                        // TB-7.5 fix #1 (Codex audit 492e86c action #1, BLOCKING):
                                        // FAIL-CLOSED authoritative routing for OMEGA per-tactic
                                        // branch.
                                        if let (Some(bundle), Some(reg)) =
                                            (chaintape_bundle.as_ref(), agent_keypairs.as_ref())
                                        {
                                            let q = match bundle.sequencer.q_snapshot() {
                                                Ok(q) => q,
                                                Err(e) => {
                                                    error!("[chaintape/atom3-omega-pertactic] FAIL-CLOSED: q_snapshot: {e:?}");
                                                    std::process::exit(3);
                                                }
                                            };
                                            let parent_state_root = q.state_root_t;
                                            let logical_t = bundle.sequencer.next_logical_t_peek();
                                            let task_id_str = format!("task-{}", run_id);
                                            // TB-7.7 D1: open CAS first.
                                            let mut cas_store = match turingosv4::bottom_white::cas::store::CasStore::open(&bundle.cas_path) {
                                                Ok(c) => c,
                                                Err(e) => {
                                                    error!("[chaintape/atom3-omega-pertactic] FAIL-CLOSED: cas open: {e}");
                                                    std::process::exit(3);
                                                }
                                            };
                                            // TB-7.7 D2: parent_tx for branch lineage.
                                            let parent_tx_for_pt: Option<turingosv4::state::q_state::TxId> =
                                                last_tx_by_agent.get(agent_id).cloned();
                                            let pt_partial = match turingosv4::runtime::proposal_telemetry::ProposalTelemetry::build_for_evaluator_append_with_parent(
                                                &mut cas_store,
                                                &run_id,
                                                agent_id,
                                                proposal_count as u64,
                                                tactic.as_bytes(),
                                                "step_complete",
                                                turingosv4::runtime::proposal_telemetry::TokenCounts {
                                                    prompt_tokens: response.prompt_tokens as u64,
                                                    completion_tokens: response.completion_tokens as u64,
                                                    tool_tokens: 0,
                                                },
                                                "tb7-atom3-omega-pertactic-payload",
                                                logical_t,
                                                parent_tx_for_pt,
                                            ) {
                                                Ok(p) => p,
                                                Err(e) => {
                                                    error!("[chaintape/atom3-omega-pertactic] FAIL-CLOSED: proposal_artifact CAS put: {e}");
                                                    std::process::exit(3);
                                                }
                                            };
                                            // TB-7.7 D4: VerificationResult for OMEGA-pertactic accept.
                                            let suffix = format!("omega-pertactic-{}", proposal_count);
                                            let work_tx_id_pre =
                                                turingosv4::state::q_state::TxId(format!(
                                                    "worktx-{}-{}",
                                                    task_id_str, suffix
                                                ));
                                            let vr = turingosv4::runtime::verification_result::VerificationResult::from_lean_run(
                                                work_tx_id_pre.clone(),
                                                turingosv4::state::q_state::AgentId(agent_id.into()),
                                                0, // OMEGA-accept (PartialVerdict::Complete) = Lean exit 0
                                                pt_partial.proposal_artifact_cid,
                                                proof_file.as_deref().unwrap_or(""),
                                                tactic.as_bytes(),
                                            );
                                            let vr_cid = match turingosv4::runtime::verification_result::write_to_cas(
                                                &mut cas_store,
                                                &vr,
                                                "tb7-atom3-omega-pertactic-vr",
                                                logical_t,
                                            ) {
                                                Ok(c) => c,
                                                Err(e) => {
                                                    error!("[chaintape/atom3-omega-pertactic] FAIL-CLOSED: VerificationResult CAS put: {e}");
                                                    std::process::exit(3);
                                                }
                                            };
                                            let pt = pt_partial.with_verification_result(vr_cid);
                                            let tel_cid = match turingosv4::runtime::proposal_telemetry::write_to_cas(
                                                &mut cas_store,
                                                &pt,
                                                "tb7-atom3-omega-pertactic",
                                                logical_t,
                                            ) {
                                                Ok(c) => c,
                                                Err(e) => {
                                                    error!("[chaintape/atom3-omega-pertactic] FAIL-CLOSED: telemetry CAS write: {e}");
                                                    std::process::exit(3);
                                                }
                                            };
                                            // TB-7.7 D3: stake from env (default 1000 micro-units).
                                            let stake_micro: i64 = std::env::var("TURINGOS_CHAINTAPE_PROPOSAL_STAKE_MICRO")
                                                .ok()
                                                .and_then(|s| s.parse().ok())
                                                .unwrap_or(1_000);
                                            let (work_tx, verify_tx) = {
                                                let mut reg_guard = match reg.lock() {
                                                    Ok(g) => g,
                                                    Err(p) => p.into_inner(),
                                                };
                                                let w = match turingosv4::runtime::adapter::make_real_worktx_signed_by(
                                                    &mut *reg_guard,
                                                    &task_id_str,
                                                    agent_id,
                                                    parent_state_root,
                                                    stake_micro,
                                                    &suffix,
                                                    tel_cid,
                                                    true,
                                                    logical_t,
                                                ) {
                                                    Ok(tx) => tx,
                                                    Err(e) => {
                                                        error!("[chaintape/atom3-omega-pertactic] FAIL-CLOSED: make_real_worktx: {e}");
                                                        std::process::exit(3);
                                                    }
                                                };
                                                let work_tx_id = match &w {
                                                    turingosv4::state::typed_tx::TypedTx::Work(w) => w.tx_id.clone(),
                                                    _ => {
                                                        error!("[chaintape/atom3-omega-pertactic] FAIL-CLOSED: make_real_worktx returned non-Work variant");
                                                        std::process::exit(3);
                                                    }
                                                };
                                                let v = match turingosv4::runtime::adapter::make_real_verifytx_signed_by(
                                                    &mut *reg_guard,
                                                    parent_state_root,
                                                    work_tx_id,
                                                    agent_id,
                                                    0,
                                                    &suffix,
                                                    true,
                                                    logical_t,
                                                ) {
                                                    Ok(tx) => tx,
                                                    Err(e) => {
                                                        error!("[chaintape/atom3-omega-pertactic] FAIL-CLOSED: make_real_verifytx: {e}");
                                                        std::process::exit(3);
                                                    }
                                                };
                                                (w, v)
                                            };
                                            // TB-7.7 D2: capture tx_ids before move.
                                            let work_tx_id = match &work_tx {
                                                turingosv4::state::typed_tx::TypedTx::Work(w) => Some(w.tx_id.clone()),
                                                _ => None,
                                            };
                                            let verify_tx_id = match &verify_tx {
                                                turingosv4::state::typed_tx::TypedTx::Verify(v) => Some(v.tx_id.clone()),
                                                _ => None,
                                            };
                                            if let Err(e) = bus.submit_typed_tx(work_tx).await {
                                                error!("[chaintape/atom3-omega-pertactic] FAIL-CLOSED: WorkTx submit_typed_tx: {e:?}");
                                                std::process::exit(3);
                                            }
                                            if let Err(e) = bus.submit_typed_tx(verify_tx).await {
                                                error!("[chaintape/atom3-omega-pertactic] FAIL-CLOSED: VerifyTx submit_typed_tx: {e:?}");
                                                std::process::exit(3);
                                            }
                                            // TB-7.7 D2: record latest tx as parent for next same-agent proposal.
                                            if let Some(tx_id) = verify_tx_id.or(work_tx_id) {
                                                last_tx_by_agent.insert(agent_id.to_string(), tx_id);
                                            }
                                        }

                                        // shadow_only: kernel.tape view sync; L4 chain above is
                                        // canonical. Per TB-7 §4.0 option (3) + §6 #31.
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
                                        // Both legs hold. C1b: route through apply_mode_to_accept;
                                        // (true, true) passes through unchanged for Full + SoftLaw.
                                        let (rt, ph) = minif2f_v4::experiment_mode::apply_mode_to_accept(
                                            mode, true, true,
                                        );
                                        return make_pput(problem_file, &condition, &run_model_label,
                                                        rt, ph,
                                                        start, gp_tokens, gp_nodes, tx as u64 + 1,
                                                        Some(tool_dist), upr,
                                                        Some(prefix.clone()),
                                                        Some("per_tactic".to_string()),
                                                        proof_file,
                                                        acc.total_run_token_count(),
                                                        acc.failed_branch_count,
                                                        wc.elapsed_ms().unwrap_or(0),
                                                        false,
                                                        proposal_hashes.len() as u64,
                                                        proposal_count,
                                                        verifier_wait_ms,
                                                        budget_regime, budget_max_tx_base, &run_id);
                                    }
                                    PartialVerdict::PartialOk => {
                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
                                        // shadow_only: PartialOk is intermediate progress, not OMEGA
                                        // accept. The authoritative routing for intermediate
                                        // progress is the append-branch routing at evaluator.rs
                                        // line ~1283 (Atom 2). This call writes only to kernel.tape
                                        // for next-iteration prompt context. Per TB-7 §4.0 option
                                        // (3) + §6 #31; will be removed when kernel.tape is
                                        // L4-derived.
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
    // C1b: route through apply_mode_to_accept; SoftLaw fakes runtime
    // accept here too — pput_runtime registers a positive value despite
    // the budget-exhausted no-proof outcome. pput_verified stays 0.
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
    let (rt, ph) = minif2f_v4::experiment_mode::apply_mode_to_accept(
        mode, false, false,
    );
    let pput_result = make_pput(problem_file, &condition, &run_model_label,
              rt, ph, start, 0, 0,
              max_transactions as u64, Some(tool_dist), upr,
              None, None, None,
              acc.total_run_token_count(),
              acc.failed_branch_count,
              wc.elapsed_ms().unwrap_or(0),
              true,
              proposal_hashes.len() as u64,
              proposal_count,
              verifier_wait_ms,
              budget_regime, budget_max_tx_base, &run_id);
    // TB-6 Atom 1.3: drain chaintape bundle before final return so queued
    // submissions are committed to on-disk ChainTape. shutdown_tx + driver
    // JoinHandle wired in src/runtime/mod.rs; per preflight v2.1 §3.2 the
    // wrapper uses tokio::select! to honor the signal and drain queue_rx.
    // NOTE: early-return paths (timeout / max-tx exhausted with `return
    // make_pput(...)`) drop the bundle WITHOUT explicit shutdown(); the
    // driver still terminates cleanly via shutdown_tx-drop → shutdown_rx-Err
    // path, but explicit drain is best-effort only on the canonical exit.
    if let Some(bundle) = chaintape_bundle {
        // TB-6 Atom 6: build the RunSummary from the on-disk chain BEFORE the
        // bundle is consumed by `shutdown()` (RunSummary needs the runtime_repo
        // path + cas path + a final read of L4 / L4.E). Caller-supplied
        // `failed_branch_count` and `rollback_count` mirror PputResult.
        let runtime_repo_path = bundle.runtime_repo_path.clone();
        let cas_path = bundle.cas_path.clone();
        if let Err(e) = bundle.shutdown().await {
            error!("[chaintape] driver shutdown returned error: {e}");
        }
        match turingosv4::runtime::run_summary::RunSummary::from_chaintape(
            &runtime_repo_path,
            &cas_path,
            &run_id,
            acc.failed_branch_count as u64,
            // PputResult.rollback_count is hard-coded to 0 in make_pput;
            // mirror that here so the summary stays cross-consistent until a
            // future TB threads a real rollback counter.
            0,
        ) {
            Ok(summary) => {
                if let Err(e) = summary.write_canonical(&runtime_repo_path) {
                    error!("[chaintape] RunSummary write failed: {e}");
                } else {
                    info!(
                        "[chaintape] Atom 6 RunSummary written ({} L4 + {} L4.E entries; \
                         {} accepted_tx_ids, {} rejected_tx_ids)",
                        summary.l4_entries,
                        summary.l4e_entries,
                        summary.accepted_tx_ids.len(),
                        summary.rejected_tx_ids.len(),
                    );
                }
            }
            Err(e) => error!("[chaintape] RunSummary build failed: {e}"),
        }
    }
    pput_result
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
    total_run_token_count: u64,
    failed_branch_count: u32,
    total_wall_time_ms: u64,
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

    // PREREG § 5 constitutional notation: C_i (full-run cost) + T_i (wall clock).
    let c_i = total_run_token_count;
    let t_i = total_wall_time_ms;

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
        failed_branch_count,
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
        // TB-1 Day-1: stamped post-construction at the prompt-build site
        // (run_oneshot today). Default None lets non-prompt-stamping
        // callers (tests, error-path returns before prompt build) round-trip.
        prompt_context_hash: None,
        // TB-1 Day-1: declared field; computation lands TB-1 Day 4.
        h_vppu: None,
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
            2000, 0, 15_000,
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
            2000, 0, 15_000,
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
            8_000, 199, 120_000,
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
            2_000, 49, 40_000,
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

    /// Phase C atom C5 conformance (PREREG § 6 C5): mode-purity test.
    ///
    /// "Running all 5 modes on the same problem produces jsonl with
    /// **identical `git_sha`, `binary_sha256`, and `model_snapshot`
    /// fields** — only the `mode` field differs. Any drift = BLOCKER
    /// (rules out 'Soft Law happened to use a different binary' confound)."
    ///
    /// We test the schema discipline at the make_pput layer: with all
    /// other inputs held identical (model arg, env vars BINARY_SHA256
    /// + MODEL_SNAPSHOT, build_sha provided by the build), only varying
    /// the MODE env var should change the `mode` field — never the
    /// build/binary/model identity fields. The C2 100-row batch is the
    /// integration-level companion (5 modes × 10 problems × 2 seeds);
    /// post-hoc analysis on those 100 rows verifies the same property
    /// end-to-end.
    #[test]
    fn c5_mode_flag_binary_purity() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var("SPLIT", "adaptation");
        std::env::set_var("BINARY_SHA256", "sha256:c5_test_pin_binary_identity");
        std::env::set_var("MODEL_SNAPSHOT", "deepseek-v4-flash@2026-04-26");

        let modes = ["full", "soft_law", "homogeneous", "panopticon", "amnesia"];
        let mut results = Vec::with_capacity(modes.len());

        for m in modes {
            std::env::set_var("MODE", m);
            // All inputs to make_pput identical across modes; only MODE
            // env differs. Note: this test exercises the schema discipline
            // directly — apply_mode_to_accept's runtime transform is NOT
            // exercised here, since the test asserts the binary-identity
            // axis (orthogonal to the accept axis).
            let r = make_pput(
                "test_problem.lean", "oneshot", "deepseek-v4-flash",
                true, true, Instant::now(),
                500, 1, 1,
                None, None, None, None, None,
                2000, 0, 15_000,
                false, 1, 1, 4_500,
                minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 1,
                "test_run_id",
            );
            results.push(r);
        }

        // Sanity: 5 distinct mode labels observed.
        let modes_observed: std::collections::HashSet<String> =
            results.iter().map(|r| r.mode.clone()).collect();
        assert_eq!(modes_observed.len(), 5,
            "expected 5 distinct mode labels stamped on the rows; got {:?}",
            modes_observed);

        // Mode-invariant identity fields: every row's (git_sha, binary_sha256,
        // model_snapshot, split) must be identical to row 0's.
        let r0 = &results[0];
        for r in &results[1..] {
            assert_eq!(r.git_sha, r0.git_sha,
                "git_sha must be mode-invariant; mode '{}' differs (got {:?} vs {:?})",
                r.mode, r.git_sha, r0.git_sha);
            assert_eq!(r.binary_sha256, r0.binary_sha256,
                "binary_sha256 must be mode-invariant; mode '{}' differs (got {:?} vs {:?})",
                r.mode, r.binary_sha256, r0.binary_sha256);
            assert_eq!(r.model_snapshot, r0.model_snapshot,
                "model_snapshot must be mode-invariant; mode '{}' differs",
                r.mode);
            assert_eq!(r.split, r0.split,
                "split must be mode-invariant; mode '{}' differs", r.mode);
        }

        // Confirm the env-pinned values actually flowed through to the rows
        // (otherwise the equality above would be vacuously true on empty strings).
        assert_eq!(r0.binary_sha256, "sha256:c5_test_pin_binary_identity",
            "BINARY_SHA256 env did not flow to the row");
        assert_eq!(r0.model_snapshot, "deepseek-v4-flash@2026-04-26",
            "MODEL_SNAPSHOT env did not flow to the row");

        std::env::remove_var("SPLIT");
        std::env::remove_var("MODE");
        std::env::remove_var("BINARY_SHA256");
        std::env::remove_var("MODEL_SNAPSHOT");
    }
}
