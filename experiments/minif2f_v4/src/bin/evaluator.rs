// MiniF2F v4 Evaluator — oneshot and swarm modes
// Constitutional basis: Art. I.1 (single boolean metric: solved or not)
// Single universal metric: solved_count / 488 (Karpathy principle)

use minif2f_v4::lean4_oracle::{Lean4Oracle, derive_lean_path, load_problem};
use turingosv4::bus::{BusConfig, BusResult, TuringBus};
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::kernel::Kernel;
use turingosv4::ledger::EventType;
use turingosv4::sdk::actor::{BoltzmannParams, MinerTx, boltzmann_select_parent};
use turingosv4::sdk::prompt::build_agent_prompt;
use turingosv4::sdk::protocol::parse_agent_output;
use turingosv4::sdk::tools::wallet::WalletTool;
use turingosv4::sdk::tools::search::SearchTool;
use turingosv4::sdk::tools::librarian::LibrarianTool;

use std::path::PathBuf;
use log::{info, warn, error};

const DEFAULT_MINIF2F_DIR: &str = "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4";

#[tokio::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: evaluator <problem_file.lean> [--condition oneshot|n1|n3]");
        eprintln!("Example: evaluator mathd_algebra_48.lean");
        eprintln!("  MINIF2F_DIR env: path to minif2f_data_lean4 (default: v3 path)");
        eprintln!("  LLM_PROXY_URL env: local LLM proxy (default: http://localhost:8080)");
        eprintln!("  ACTIVE_MODEL env: model name (default: deepseek-reasoner)");
        eprintln!("  CONDITION env: oneshot|n1|n3 (default: oneshot)");
        std::process::exit(1);
    }

    let problem_file = &args[1];
    let condition = std::env::var("CONDITION").unwrap_or_else(|_| "oneshot".into());
    let minif2f_dir = std::env::var("MINIF2F_DIR").unwrap_or_else(|_| DEFAULT_MINIF2F_DIR.into());
    let proxy_url = std::env::var("LLM_PROXY_URL").unwrap_or_else(|_| "http://localhost:8080".into());
    let model = std::env::var("ACTIVE_MODEL").unwrap_or_else(|_| "deepseek-reasoner".into());

    // Determine problem path
    let problem_path = if PathBuf::from(problem_file).exists() {
        problem_file.clone()
    } else {
        // Search in Test/ and Valid/ splits
        let test_path = format!("{}/MiniF2F/Test/{}", minif2f_dir, problem_file);
        let valid_path = format!("{}/MiniF2F/Valid/{}", minif2f_dir, problem_file);
        if PathBuf::from(&test_path).exists() {
            test_path
        } else if PathBuf::from(&valid_path).exists() {
            valid_path
        } else {
            eprintln!("Problem file not found: {}", problem_file);
            std::process::exit(1);
        }
    };

    // Load problem
    let (problem_statement, theorem_name) = match load_problem(&problem_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to load problem: {}", e);
            std::process::exit(1);
        }
    };

    info!("Loaded problem: {} (theorem: {})", problem_file, theorem_name);
    info!("=== MiniF2F v4 (Polymarket + Lean 4 Oracle) ===");
    info!("Condition: {}, Model: {}", condition, model);

    // Derive LEAN_PATH
    let lean_path = derive_lean_path(&minif2f_dir);
    info!("LEAN_PATH: {}", lean_path);

    match condition.as_str() {
        "oneshot" => {
            run_oneshot(&problem_statement, &theorem_name, &lean_path,
                       &proxy_url, &model).await;
        }
        "n1" | "n1_turingos" => {
            run_swarm(&problem_statement, &theorem_name, &lean_path,
                     &proxy_url, &model, 1).await;
        }
        "n3" | "n3_turingos" => {
            run_swarm(&problem_statement, &theorem_name, &lean_path,
                     &proxy_url, &model, 3).await;
        }
        other => {
            eprintln!("Unknown condition: {}. Use oneshot, n1, or n3.", other);
            std::process::exit(1);
        }
    }
}

/// Oneshot mode: single LLM call, no swarm, no market.
/// Baseline measurement for the auto-research loop.
async fn run_oneshot(
    problem_statement: &str,
    theorem_name: &str,
    lean_path: &str,
    proxy_url: &str,
    model: &str,
) {
    info!("--- ONESHOT MODE ---");

    let oracle = Lean4Oracle::new(
        problem_statement.to_string(),
        theorem_name.to_string(),
        lean_path.to_string(),
    );

    // Build minimal prompt (no history, no market, no REPL)
    let prompt = format!(
        "Complete the following Lean 4 proof. Output ONLY the tactic proof body.\n\n{}",
        problem_statement
    );

    // LLM call
    // DeepSeek timeout handling: generous timeout + retry (per user feedback)
    let client = ResilientLLMClient::new(proxy_url, 120, 3);
    let request = GenerateRequest {
        model: model.to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.2),
        max_tokens: Some(8000),
    };

    info!("Calling LLM ({})...", model);

    match client.generate(&request).await {
        Ok(response) => {
            info!("LLM response: {} tokens", response.completion_tokens);

            // Rule 22 v2 clause 4: reject markdown fences
            if response.content.contains("```") {
                warn!("REJECTED: LLM output contains markdown fences");
                info!("RESULT: OmegaRejected (format violation)");
                return;
            }

            // Verify with oracle
            info!("Verifying with Lean 4 oracle...");
            match oracle.verify_omega(&response.content) {
                Ok(true) => {
                    info!(">>> OMEGA ACCEPTED <<<");
                    info!("RESULT: OmegaAccepted");
                }
                Ok(false) => {
                    info!("RESULT: OmegaRejected (verification failed)");
                }
                Err(e) => {
                    warn!("RESULT: OmegaError ({})", e);
                }
            }
        }
        Err(e) => {
            error!("LLM call failed: {}", e);
            info!("RESULT: OmegaError (LLM failure)");
        }
    }
}

/// Swarm mode: N agents with prediction market coordination.
async fn run_swarm(
    problem_statement: &str,
    theorem_name: &str,
    lean_path: &str,
    proxy_url: &str,
    model: &str,
    n_agents: usize,
) {
    info!("--- SWARM MODE (N={}) ---", n_agents);

    // Initialize bus with tools
    let kernel = Kernel::new();
    let config = BusConfig {
        max_payload_chars: 1200,
        max_payload_lines: 18,
        system_lp_amount: 200.0,
        forbidden_patterns: vec![
            "native_decide".into(),
            "#eval".into(),
            "IO.Process".into(),
            "IO.FS".into(),
            "run_tac".into(),
            "unsafe".into(),
        ],
    };

    let mut bus = TuringBus::new(kernel, config);

    // Mount tools
    bus.mount_tool(Box::new(WalletTool::new(10000.0)));
    bus.mount_tool(Box::new(Lean4Oracle::new(
        problem_statement.to_string(),
        theorem_name.to_string(),
        lean_path.to_string(),
    )));
    bus.mount_tool(Box::new(SearchTool::new(
        vec![format!("{}/MiniF2F/Test", std::env::var("MINIF2F_DIR")
            .unwrap_or_else(|_| DEFAULT_MINIF2F_DIR.into()))],
        20,
    )));
    bus.mount_tool(Box::new(LibrarianTool::new(
        &format!("{}/skills",
            std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into())),
        8,
    )));

    // Initialize agents
    let agent_ids: Vec<String> = (0..n_agents).map(|i| format!("Agent_{}", i)).collect();
    bus.init(&agent_ids);

    info!("Genesis: {} agents x 10000 Coins", n_agents);

    // LLM client
    let client = ResilientLLMClient::new(proxy_url, 120, 3);
    let params = BoltzmannParams::from_env();

    let max_transactions = 200;
    let mut omega_found = false;

    for tx in 0..max_transactions {
        if omega_found { break; }

        // Round-robin agent selection
        let agent_id = &agent_ids[tx % n_agents];

        // Build snapshot for agent
        let snap = bus.snapshot();
        let chain = if snap.tape.is_empty() {
            problem_statement.to_string()
        } else {
            // Build proof chain from tape
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

        // LLM call
        let request = GenerateRequest {
            model: model.to_string(),
            messages: vec![Message { role: "user".into(), content: prompt }],
            temperature: Some(0.2),
            max_tokens: Some(4000),
        };

        match client.generate(&request).await {
            Ok(response) => {
                match parse_agent_output(&response.content) {
                    Ok(action) => {
                        match action.tool.as_str() {
                            "append" => {
                                if let Some(payload) = &action.payload {
                                    // Select parent via Boltzmann
                                    let prices: std::collections::HashMap<String, f64> =
                                        snap.markets.iter()
                                            .map(|(id, m)| (id.clone(), m.yes_price))
                                            .collect();
                                    let parent = boltzmann_select_parent(
                                        &snap.tape, &prices, &params, &mut rand::thread_rng()
                                    );
                                    match bus.append(agent_id, payload, parent.as_deref()) {
                                        Ok(BusResult::Appended { node_id }) => {
                                            info!("[tx {}] {} appended {}", tx, agent_id, node_id);
                                        }
                                        Ok(BusResult::Vetoed { reason }) => {
                                            warn!("[tx {}] {} vetoed: {}", tx, agent_id, reason);
                                        }
                                        Ok(BusResult::Invested { .. }) => {}
                                        Err(e) => warn!("[tx {}] error: {}", tx, e),
                                    }
                                }
                            }
                            "complete" => {
                                if let Some(payload) = &action.payload {
                                    info!("[tx {}] {} claims OMEGA", tx, agent_id);
                                    let oracle = Lean4Oracle::new(
                                        problem_statement.to_string(),
                                        theorem_name.to_string(),
                                        lean_path.to_string(),
                                    );
                                    match oracle.verify_omega(payload) {
                                        Ok(true) => {
                                            info!(">>> OMEGA ACCEPTED <<<");
                                            omega_found = true;
                                        }
                                        Ok(false) => {
                                            warn!("[tx {}] OMEGA rejected", tx);
                                        }
                                        Err(e) => {
                                            warn!("[tx {}] OMEGA error: {}", tx, e);
                                        }
                                    }
                                }
                            }
                            _ => {
                                info!("[tx {}] {} action: {}", tx, agent_id, action.tool);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("[tx {}] {} parse error: {}", tx, agent_id, e);
                    }
                }
            }
            Err(e) => {
                warn!("[tx {}] {} LLM error: {}", tx, agent_id, e);
            }
        }
    }

    if omega_found {
        info!("=== PROBLEM SOLVED ===");
        // Settlement
        let gp: Vec<String> = bus.kernel.tape.time_arrow().to_vec();
        bus.halt_and_settle(&gp).ok();
    } else {
        info!("=== MAX TRANSACTIONS REACHED — NOT SOLVED ===");
    }
}
