// MiniF2F v4 Evaluator — oneshot and swarm modes
//
// Sole optimization metric: PPUT (Progress Per Unit Time)
//   Progress = 100% if Golden Path exists (OMEGA reached), 0% otherwise
//   PPUT = 100% / time_to_omega (seconds)
//   No GP → PPUT = 0 → problem not worth attacking in current iteration
//
// Constitutional basis: Art. I.1 (boolean predicate), Art. I.2 (statistical signal = PPUT)

use minif2f_v4::lean4_oracle::{Lean4Oracle, PartialVerdict, derive_lean_path, load_problem};
use turingosv4::bus::{BusConfig, BusResult, TuringBus};
use turingosv4::sdk::error_abstraction::{classify_lean_error, classify_parse_error, CLASSIFIER_VERSION};
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::kernel::Kernel;
use turingosv4::sdk::actor::{BoltzmannParams, boltzmann_select_parent};
use turingosv4::sdk::prompt::build_agent_prompt;
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
#[derive(Debug, serde::Serialize)]
struct PputResult {
    problem: String,
    condition: String,
    model: String,
    has_golden_path: bool,         // true = OMEGA reached
    time_secs: f64,                // wall time elapsed
    pput: f64,                     // 100/time if GP, 0 otherwise
    gp_token_count: u64,           // token count of golden path (0 if no GP)
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
    // Phase 9 § 0 (2026-04-22): Report Standard fields required for Gate 9
    // auxiliary criteria (CLAUDE.md + C-053 + C-055 + C-061 + C-059).
    // `skip_serializing_if` preserves backward compat with pre-9.0 jsonl rows.
    #[serde(skip_serializing_if = "Option::is_none")]
    reputation_at_end: Option<HashMap<String, u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    halt_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pairwise_diversity_mean: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_selection_entropy: Option<f64>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
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
    let model = std::env::var("ACTIVE_MODEL").unwrap_or_else(|_| "deepseek-reasoner".into());

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
        c if c.starts_with('n') && c[1..].parse::<usize>().is_ok() => {
            let n: usize = c[1..].parse().unwrap();
            run_swarm(problem_file, &problem_statement, &theorem_name,
                     &lean_path, &proxy_url, &model, n).await
        }
        "hybrid_v1" => {
            // RCA (2026-04-14): agent_0's swarm prompt (tape/market/errors/tools)
            // underperforms oneshot's bare prompt on problems DeepSeek can 1-shot.
            // This condition uses oneshot prompt first; only if that fails does
            // the swarm fire. Separately named per C-032/C-033 causal attribution.
            let start = Instant::now();
            let r = run_oneshot(problem_file, &problem_statement, &theorem_name,
                               &lean_path, &proxy_url, &model).await;
            if r.has_golden_path {
                PputResult { condition: "hybrid_v1".into(), ..r }
            } else {
                let r2 = run_swarm(problem_file, &problem_statement, &theorem_name,
                                  &lean_path, &proxy_url, &model, 3).await;
                let elapsed = start.elapsed().as_secs_f64();
                PputResult {
                    condition: "hybrid_v1".into(),
                    time_secs: elapsed,
                    pput: if r2.has_golden_path { 100.0 / elapsed } else { 0.0 },
                    tx_count: 1 + r2.tx_count,
                    ..r2
                }
            }
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

    let oracle = Lean4Oracle::new(
        problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
    );

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

    match client.generate(&request).await {
        Ok(response) => {
            // Rule 22 v2 clause 4: reject markdown fences
            if response.content.contains("```") {
                return make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1,
                                 None, None, None, None, None);
            }

            match oracle.verify_omega(&response.content) {
                Ok(true) => {
                    let gp_tokens = response.completion_tokens as u64;
                    let preview: String = response.content.chars().take(500).collect();
                    info!(">>> OMEGA ACCEPTED <<< (path=alone, payload[0..500]={:?})", preview);
                    let proof_file = persist_proof_artifact(
                        problem_file, theorem_name, problem_statement,
                        &response.content, "alone", "oneshot",
                    );
                    // Phase 8.B + R2 (Codex N-1 / VETO Q6 / C-048): oneshot must
                    // route through a WAL-backed bus to satisfy Art. IV (∏p=1 →
                    // wtool → Q_{t+1}) + C-043 (mandatory wtool) + C-061 (durable
                    // q-halt). Per-problem WAL file → durable ledger record;
                    // ephemeral bus reference (per-problem) preserves oneshot
                    // stateless semantics but the Halt event survives disk.
                    use turingosv4::bus::{BusConfig, TuringBus};
                    use turingosv4::kernel::Kernel;
                    use turingosv4::sdk::oracle_receipt::OracleReceipt;
                    let problem_stem = std::path::Path::new(problem_file)
                        .file_stem().map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "unknown".into());
                    let ts = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs().to_string())
                        .unwrap_or_else(|_| "0".into());
                    let wal_dir = std::env::var("WAL_DIR").unwrap_or_else(|_| {
                        // Default: project-local wal/oneshot/ directory.
                        std::path::PathBuf::from("wal/oneshot").to_string_lossy().into_owned()
                    });
                    let _ = std::fs::create_dir_all(&wal_dir);
                    let wal_path = std::path::Path::new(&wal_dir)
                        .join(format!("{}_{}.wal.jsonl", problem_stem, ts));
                    let config = BusConfig {
                        max_payload_chars: usize::MAX,
                        max_payload_lines: usize::MAX,
                        system_lp_amount: 0.0,
                        forbidden_patterns: vec![],
                        min_class_count_to_broadcast: 3,
                    };
                    // R2 (Codex residual CHALLENGE): WAL open failure must
                    // NOT silently degrade to in-memory — that would make
                    // Art. IV "durable" conditional. Treat as ErrorHalt:
                    // skip the wtool step, record measurement error, return
                    // solved-but-unverified. Downstream batch runner can
                    // investigate filesystem / disk space / permissions.
                    // R2 v2 (Codex CHALLENGE): batch runners only treat the
                    // run as retry-worthy when NO `PPUT_RESULT` is emitted
                    // (mirrors the existing LLM-error pattern below). So we
                    // exit(2) with MEASUREMENT_ERROR on stderr — never call
                    // make_pput, never emit PPUT_RESULT.
                    let mut oneshot_bus = match TuringBus::with_wal_path(
                        Kernel::new(), config, &wal_path,
                    ) {
                        Ok(b) => b,
                        Err(e) => {
                            error!("[art-iv] oneshot WAL open failed ({}); \
                                    refusing to degrade to in-memory bus \
                                    (C-067 durability requirement).", e);
                            eprintln!("MEASUREMENT_ERROR oneshot WAL: {}", e);
                            std::process::exit(2);
                        }
                    };
                    // R1-α: register oracle Ed25519 pubkey BEFORE init so
                    // `init()` can freeze the trusted pub set.
                    if let Err(e) = oneshot_bus.register_oracle(oracle.public_key()) {
                        warn!("[art-iv] oneshot register_oracle: {}", e);
                    }
                    oneshot_bus.init(&["oneshot_agent".into()]);
                    let receipt = oracle.issue_complete_receipt(&response.content, None);
                    match oneshot_bus.append_oracle_accepted(
                        "oneshot_agent", &response.content, None, &receipt,
                    ) {
                        Ok(turingosv4::bus::BusResult::Appended { node_id }) => {
                            // R2: durable Halt event via halt_and_settle on GP=[node_id].
                            if let Err(e) = oneshot_bus.halt_and_settle(&[node_id]) {
                                warn!("[art-iv] oneshot halt_and_settle: {}", e);
                            }
                        }
                        Ok(other) => warn!("[art-iv] oneshot wtool unexpected: {:?}", other),
                        Err(e) => warn!("[art-iv] oneshot wtool failed (receipt validation): {}", e),
                    }
                    // Phase 9 § 0: surface Report Standard aux fields from bus state.
                    let reputation = Some(oneshot_bus.kernel.tape.reputation().clone());
                    let halt_reason = extract_halt_reason(&oneshot_bus);
                    make_pput_full(
                        problem_file, "oneshot", model, true, start, gp_tokens, 1, 1,
                        None, None, Some(response.content.clone()),
                        Some("alone".to_string()), proof_file,
                        reputation, halt_reason,
                        None,  // pairwise_diversity: oneshot = 1 agent, N/A
                        None,  // parent_selection_entropy: N/A
                    )
                }
                Ok(false) => {
                    make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1,
                              None, None, None, None, None)
                }
                Err(e) => {
                    warn!("Oracle error: {}", e);
                    make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1,
                              None, None, None, None, None)
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
        // NB: bus-level substring match catches cases like `native_decide`;
        // lean4_oracle also enforces C-050 (Mathlib-qualified allow, bare ban).
        forbidden_patterns: vec![
            "native_decide".into(),
            "#eval".into(), "IO.Process".into(),
            "IO.FS".into(), "run_tac".into(), "unsafe".into(),
        ],
        min_class_count_to_broadcast: 3,
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
        // R2 v2 (Gemini + Codex residual CHALLENGE): same as oneshot —
        // don't silently degrade; don't emit PPUT_RESULT; exit(2) so batch
        // runners treat as retry-worthy (see run_batch.sh:162-180 semantics).
        match TuringBus::with_wal_path(kernel, config, wal_path) {
            Ok(b) => b,
            Err(e) => {
                error!("[wal] open failed: {} — refusing to degrade to in-memory \
                        (C-067 durability).", e);
                eprintln!("MEASUREMENT_ERROR swarm WAL: {}", e);
                std::process::exit(2);
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
    // R1-α: single Lean4Oracle for the whole run. Same instance serves (a)
    // the bus-mounted TuringTool role and (b) receipt issuance for OMEGA /
    // step outcomes. Clone is cheap (Strings + Ed25519 SigningKey Clone).
    let oracle = Lean4Oracle::new(
        problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
    );
    bus.mount_tool(Box::new(oracle.clone()));
    bus.mount_tool(Box::new(SearchTool::new(
        vec![format!("{}/MiniF2F/Test", std::env::var("MINIF2F_DIR")
            .unwrap_or_else(|_| DEFAULT_MINIF2F_DIR.into()))], 20,
    )));
    bus.mount_tool(Box::new(LibrarianTool::new(
        &format!("{}/skills", std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into())), 8,
    )));

    // R1-α: register oracle pubkey BEFORE init so init() can freeze the
    // trusted set. Post-init registration would return Err.
    if let Err(e) = bus.register_oracle(oracle.public_key()) {
        error!("[art-iv] bus.register_oracle failed: {} — aborting run", e);
        return make_pput(problem_file, &condition, model, false, start, 0, 0, 0,
                         None, None, None, None, None);
    }

    let agent_ids: Vec<String> = (0..n_agents).map(|i| format!("Agent_{}", i)).collect();
    bus.init(&agent_ids);
    // Phase 4: top-up ensure_agents for any IDs not in the loaded state (zero
    // balance if post-genesis, genesis_coins only on first-ever boot).
    if let Some(wallet) = bus.tools.iter_mut()
        .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
    {
        wallet.ensure_agents(&agent_ids);
    }

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
    let max_transactions = 200;

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
    // Art. III.2: per-agent search result cache (bounded), fed into next prompt.
    let mut search_cache: HashMap<String, Vec<String>> = HashMap::new();
    // F-2026-04-19-05: cap searches per agent; beyond cap we remove `search`
    // from the tool list so agents stop wasting budget on name-match misses.
    let search_cap: u32 = std::env::var("SEARCH_CAP")
        .ok().and_then(|s| s.parse().ok()).unwrap_or(20);
    let mut search_count: HashMap<String, u32> = HashMap::new();

    for tx in 0..max_transactions {
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
            snap.get_balance(agent_id),
            // Phase 8.F v2 (C-053): surface own reputation to agent prompt.
            snap.get_reputation(agent_id),
            tools_desc, &team_board,
        );

        // Model-aware max_tokens (same rule as oneshot branch).
        let max_toks = if model.contains("chat") { 8000 } else { 16000 };
        // Art. II.2.1 anti-homogeneity: per-agent temperature ladder breaks
        // sampling correlation among role-distinct agents (F-2026-04-18-03).
        // Disabled (keep at 0.2) when TEMP_LADDER!=1 to isolate the mechanism.
        let temp: f64 = if std::env::var("TEMP_LADDER").ok().as_deref() == Some("1") {
            (0.10_f64 + (agent_idx as f64) * 0.15).min(1.30)
        } else {
            0.2
        };
        let request = GenerateRequest {
            model: model.to_string(),
            messages: vec![Message { role: "user".into(), content: prompt }],
            temperature: Some(temp),
            max_tokens: Some(max_toks),
        };

        match client.generate(&request).await {
            Ok(response) => {
                match parse_agent_output(&response.content) {
                    Ok(action) => match action.tool.as_str() {
                        "append" => {
                            *tool_dist.entry("append".into()).or_insert(0) += 1;
                            if let Some(payload) = &action.payload {
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
                                info!("[tx {}] OMEGA claim by {} (tape_nodes={}, payload_len={})",
                                      tx, agent_id, tape_len, payload.len());
                                // R1-α: use outer `oracle` (registered pre-init).
                                // Path 1: payload alone
                                let r_alone = oracle.verify_omega_detailed(payload);
                                let (full_proof, path_choice, r_final) = match &r_alone {
                                    Ok((true, _)) => (payload.clone(), "alone", r_alone.clone()),
                                    _ if !tape_chain.is_empty() => {
                                        // Path 2: tape + payload
                                        let combined = format!("{}\n{}", tape_chain, payload);
                                        let r_combined = oracle.verify_omega_detailed(&combined);
                                        if matches!(r_combined, Ok((true, _))) {
                                            *tool_dist.entry("complete_via_tape".into()).or_insert(0) += 1;
                                        }
                                        (combined, "tape+payload", r_combined)
                                    }
                                    _ => (payload.clone(), "alone", r_alone.clone()),
                                };
                                match r_final {
                                    Ok((true, _)) => {
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
                                        let receipt = oracle.issue_complete_receipt(payload, parent.as_deref());
                                        let omega_node_id = match bus.append_oracle_accepted(
                                            agent_id, payload, parent.as_deref(), &receipt,
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
                                        // Phase 9 § 0: Report Standard aux fields.
                                        let reputation = Some(bus.kernel.tape.reputation().clone());
                                        let halt_reason = extract_halt_reason(&bus);
                                        let diversity = pairwise_diversity_by_hash(&omega_payload_hashes, omega_attempts as usize);
                                        return make_pput_full(
                                            problem_file, &condition, model, true,
                                            start, gp_tokens, gp_nodes, tx as u64 + 1,
                                            Some(tool_dist), upr,
                                            Some(full_proof.clone()),
                                            Some(path_choice.to_string()),
                                            proof_file,
                                            reputation, halt_reason, diversity,
                                            None,  // parent_selection_entropy: TODO Phase 10 (needs Boltzmann trace)
                                        );
                                    }
                                    Ok((false, err_detail)) => {
                                        // Step-B v3: classify + record class label (C-022 shield).
                                        let class = classify_lean_error(&err_detail);
                                        bus.record_rejection(agent_id, class.label());
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
                                // R1-α: use outer `oracle` (registered pre-init).
                                match oracle.verify_partial(&prefix) {
                                    PartialVerdict::Complete => {
                                        info!(">>> OMEGA ACCEPTED <<< via step (depth={} after this write)",
                                              bus.kernel.tape.time_arrow().len() + 1);
                                        let proof_file = persist_proof_artifact(
                                            problem_file, &theorem_name, &problem_statement,
                                            &prefix, "per_tactic", agent_id,
                                        );
                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
                                        *tool_dist.entry("omega_wtool".into()).or_insert(0) += 1;
                                        let receipt = oracle.issue_complete_receipt(tactic, parent.as_deref());
                                        let _ = bus.append_oracle_accepted(
                                            agent_id, tactic, parent.as_deref(), &receipt,
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
                                        // Phase 9 § 0: Report Standard aux fields.
                                        let reputation = Some(bus.kernel.tape.reputation().clone());
                                        let halt_reason = extract_halt_reason(&bus);
                                        let diversity = pairwise_diversity_by_hash(&omega_payload_hashes, omega_attempts as usize);
                                        return make_pput_full(
                                            problem_file, &condition, model, true,
                                            start, gp_tokens, gp_nodes, tx as u64 + 1,
                                            Some(tool_dist), upr,
                                            Some(prefix.clone()),
                                            Some("per_tactic".to_string()),
                                            proof_file,
                                            reputation, halt_reason, diversity,
                                            None,  // parent_selection_entropy: TODO Phase 10
                                        );
                                    }
                                    PartialVerdict::PartialOk => {
                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
                                        let receipt = oracle.issue_partial_receipt(tactic, parent.as_deref());
                                        match bus.append_oracle_accepted(
                                            agent_id, tactic, parent.as_deref(), &receipt,
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
    make_pput(problem_file, &condition, model, false, start, 0, 0,
              max_transactions as u64, Some(tool_dist), upr,
              None, None, None)
}

fn make_pput(
    problem: &str, condition: &str, model: &str,
    has_gp: bool, start: Instant,
    gp_tokens: u64, gp_nodes: usize, tx_count: u64,
    tool_dist: Option<HashMap<String, u32>>,
    unique_payload_ratio: Option<f64>,
    gp_payload: Option<String>,
    gp_path: Option<String>,
    gp_proof_file: Option<String>,
) -> PputResult {
    make_pput_full(
        problem, condition, model, has_gp, start, gp_tokens, gp_nodes, tx_count,
        tool_dist, unique_payload_ratio, gp_payload, gp_path, gp_proof_file,
        None, None, None, None,
    )
}

/// Phase 9 § 0: extended make_pput that accepts Report Standard aux fields.
/// `make_pput` retained as convenience for callers that don't track these.
fn make_pput_full(
    problem: &str, condition: &str, model: &str,
    has_gp: bool, start: Instant,
    gp_tokens: u64, gp_nodes: usize, tx_count: u64,
    tool_dist: Option<HashMap<String, u32>>,
    unique_payload_ratio: Option<f64>,
    gp_payload: Option<String>,
    gp_path: Option<String>,
    gp_proof_file: Option<String>,
    reputation_at_end: Option<HashMap<String, u32>>,
    halt_reason: Option<String>,
    pairwise_diversity_mean: Option<f64>,
    parent_selection_entropy: Option<f64>,
) -> PputResult {
    let elapsed = start.elapsed().as_secs_f64();
    let pput = if has_gp && elapsed > 0.0 { 100.0 / elapsed } else { 0.0 };
    // C-012 provenance: populated from env vars; None when unset (backward compat).
    let build_sha = std::env::var("BUILD_SHA").ok();
    let classifier_version = std::env::var("CLASSIFIER_VERSION").ok();
    let boltzmann_seed = std::env::var("BOLTZMANN_SEED")
        .ok().and_then(|s| s.parse::<u64>().ok());
    PputResult {
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
        reputation_at_end,
        halt_reason,
        pairwise_diversity_mean,
        parent_selection_entropy,
    }
}

/// Extract halt_reason as a display string from the bus's ledger (latest
/// Halt event). Returns None if no Halt event was emitted. Phase 9 § 0.
fn extract_halt_reason(bus: &turingosv4::bus::TuringBus) -> Option<String> {
    for event in bus.ledger.events().iter().rev() {
        if let turingosv4::ledger::EventType::Halt { reason } = &event.event_type {
            return Some(format!("{}", reason));
        }
    }
    None
}

/// Pairwise payload diversity via hash identity: 1 - (identical_pairs/max_pairs).
/// 0.0 = all identical hashes, 1.0 = fully distinct hashes. Phase 9 § 0 (C-059).
///
/// This is a fast proxy; full token-level Jaccard would catch near-duplicates
/// with minor spacing/formatting changes. F-2026-04-18-01 warns that hash
/// uniqueness can overstate diversity (byte-identical proofs differ by 1 char).
/// For Gate 9 the proxy suffices; Paper 2+ should use full Jaccard.
fn pairwise_diversity_by_hash(hashes: &std::collections::HashSet<u64>, total_attempts: usize) -> Option<f64> {
    if total_attempts < 2 { return None; }
    // Unique hashes / total attempts = diversity proxy. 1.0 = all distinct.
    Some(hashes.len() as f64 / total_attempts as f64)
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
