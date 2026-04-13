// MiniF2F v4 Evaluator — oneshot and swarm modes
//
// Sole optimization metric: PPUT (Progress Per Unit Time)
//   Progress = 100% if Golden Path exists (OMEGA reached), 0% otherwise
//   PPUT = 100% / time_to_omega (seconds)
//   No GP → PPUT = 0 → problem not worth attacking in current iteration
//
// Constitutional basis: Art. I.1 (boolean predicate), Art. I.2 (statistical signal = PPUT)

use minif2f_v4::lean4_oracle::{Lean4Oracle, derive_lean_path, load_problem};
use turingosv4::bus::{BusConfig, BusResult, TuringBus};
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::kernel::Kernel;
use turingosv4::sdk::actor::{BoltzmannParams, boltzmann_select_parent};
use turingosv4::sdk::prompt::build_agent_prompt;
use turingosv4::sdk::protocol::parse_agent_output;
use turingosv4::sdk::tools::wallet::WalletTool;
use turingosv4::sdk::tools::search::SearchTool;
use turingosv4::sdk::tools::librarian::LibrarianTool;

use std::path::PathBuf;
use std::time::Instant;
use log::{info, warn, error};

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
}

#[tokio::main]
async fn main() {
    env_logger::init();

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
        "n1" | "n1_turingos" => {
            run_swarm(problem_file, &problem_statement, &theorem_name,
                     &lean_path, &proxy_url, &model, 1).await
        }
        "n3" | "n3_turingos" => {
            run_swarm(problem_file, &problem_statement, &theorem_name,
                     &lean_path, &proxy_url, &model, 3).await
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

    let prompt = format!(
        "Complete the following Lean 4 proof. Output ONLY the tactic proof body.\n\n{}",
        problem_statement
    );

    let client = ResilientLLMClient::new(proxy_url, 120, 3);
    let request = GenerateRequest {
        model: model.to_string(),
        messages: vec![Message { role: "user".into(), content: prompt }],
        temperature: Some(0.2),
        max_tokens: Some(8000),
    };

    match client.generate(&request).await {
        Ok(response) => {
            // Rule 22 v2 clause 4: reject markdown fences
            if response.content.contains("```") {
                return make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1);
            }

            match oracle.verify_omega(&response.content) {
                Ok(true) => {
                    let gp_tokens = response.completion_tokens as u64;
                    info!(">>> OMEGA ACCEPTED <<<");
                    make_pput(problem_file, "oneshot", model, true, start, gp_tokens, 1, 1)
                }
                Ok(false) => {
                    make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1)
                }
                Err(e) => {
                    warn!("Oracle error: {}", e);
                    make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1)
                }
            }
        }
        Err(e) => {
            error!("LLM error: {}", e);
            make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1)
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
        max_payload_chars: 1200,
        max_payload_lines: 18,
        system_lp_amount: 200.0,
        forbidden_patterns: vec![
            "native_decide".into(), "#eval".into(), "IO.Process".into(),
            "IO.FS".into(), "run_tac".into(), "unsafe".into(),
        ],
    };

    let mut bus = TuringBus::new(kernel, config);
    bus.mount_tool(Box::new(WalletTool::new(10000.0)));
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

    let client = ResilientLLMClient::new(proxy_url, 120, 3);
    let params = BoltzmannParams::from_env();
    let max_transactions = 200;

    for tx in 0..max_transactions {
        let agent_id = &agent_ids[tx % n_agents];
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
        let prompt = build_agent_prompt(
            &chain, "", &snap.market_ticker, &errors,
            snap.get_balance(agent_id), "append, complete, search",
        );

        let request = GenerateRequest {
            model: model.to_string(),
            messages: vec![Message { role: "user".into(), content: prompt }],
            temperature: Some(0.2),
            max_tokens: Some(4000),
        };

        match client.generate(&request).await {
            Ok(response) => {
                match parse_agent_output(&response.content) {
                    Ok(action) => match action.tool.as_str() {
                        "append" => {
                            if let Some(payload) = &action.payload {
                                let prices: std::collections::HashMap<String, f64> =
                                    snap.markets.iter()
                                        .map(|(id, m)| (id.clone(), m.yes_price))
                                        .collect();
                                let parent = boltzmann_select_parent(
                                    &snap.tape, &prices, &params, &mut rand::thread_rng()
                                );
                                match bus.append(agent_id, payload, parent.as_deref()) {
                                    Ok(BusResult::Appended { node_id }) => {
                                        info!("[tx {}] {} +{}", tx, agent_id, node_id);
                                    }
                                    Ok(BusResult::Vetoed { reason }) => {
                                        warn!("[tx {}] VETO: {}", tx, reason);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "complete" => {
                            if let Some(payload) = &action.payload {
                                info!("[tx {}] OMEGA claim by {}", tx, agent_id);
                                let oracle = Lean4Oracle::new(
                                    problem_statement.to_string(),
                                    theorem_name.to_string(),
                                    lean_path.to_string(),
                                );
                                if let Ok(true) = oracle.verify_omega(payload) {
                                    info!(">>> OMEGA ACCEPTED <<<");
                                    // GP = full tape ancestry from this node
                                    let gp: Vec<String> = bus.kernel.tape.time_arrow().to_vec();
                                    let gp_tokens: u64 = gp.iter()
                                        .filter_map(|id| bus.kernel.tape.get(id))
                                        .map(|n| n.payload.len() as u64)
                                        .sum();
                                    let gp_nodes = gp.len();
                                    bus.halt_and_settle(&gp).ok();
                                    return make_pput(problem_file, &condition, model, true,
                                                    start, gp_tokens, gp_nodes, tx as u64 + 1);
                                } else {
                                    warn!("[tx {}] OMEGA rejected", tx);
                                }
                            }
                        }
                        _ => {}
                    },
                    Err(e) => { warn!("[tx {}] parse: {}", tx, e); }
                }
            }
            Err(e) => { warn!("[tx {}] LLM: {}", tx, e); }
        }
    }

    // No OMEGA found → PPUT = 0
    make_pput(problem_file, &condition, model, false, start, 0, 0, max_transactions as u64)
}

fn make_pput(
    problem: &str, condition: &str, model: &str,
    has_gp: bool, start: Instant,
    gp_tokens: u64, gp_nodes: usize, tx_count: u64,
) -> PputResult {
    let elapsed = start.elapsed().as_secs_f64();
    let pput = if has_gp && elapsed > 0.0 { 100.0 / elapsed } else { 0.0 };
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
    }
}
