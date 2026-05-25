//! True-suite market/economy evidence helper.
//!
//! This binary is a runner helper: it asks an external LLM, via the local
//! OpenAI-compatible proxy, for one market action, then submits that action as
//! a signed `BuyWithCoinRouterTx` through the current ChainTape sequencer.
//! The agent is outside the kernel; the kernel only sees signed typed txs.

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use serde::Serialize;
use sha2::{Digest, Sha256};

use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::runtime::adapter::{
    genesis_with_balances, make_real_cpmm_pool_signed_by, make_real_market_seed_signed_by,
    make_real_task_open_signed_by, tb8_await_state_root_advance,
    tb_real6a_invest_task_outcome_to_router_tx,
};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::bootstrap::default_pput_preseed_pairs;
use turingosv4::runtime::genesis_report::GenesisReport;
use turingosv4::runtime::{build_chaintape_sequencer_with_initial_q, RuntimeChaintapeConfig};
use turingosv4::state::q_state::{AgentId, TaskId};
use turingosv4::state::typed_tx::{BuyDirection, EventId, TypedTx};

const SPONSOR_AGENT: &str = "Agent_user_0";
const MARKET_PROVIDER_AGENT: &str = "MarketMakerBudget";
const TRADER_AGENT: &str = "Agent_0";
const DEFAULT_MODEL: &str = "deepseek-chat";
const DEFAULT_AMOUNT_MICRO: i64 = 1_000;
const MARKET_SEED_MICRO: i64 = 100_000;

#[derive(Debug)]
struct Args {
    runtime_repo: PathBuf,
    cas: PathBuf,
    run_id: String,
    constitution: PathBuf,
    llm_proxy_url: String,
    model: String,
    out: PathBuf,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum ParsedDirection {
    Yes,
    No,
}

impl ParsedDirection {
    fn as_buy_direction(self) -> BuyDirection {
        match self {
            Self::Yes => BuyDirection::BuyYes,
            Self::No => BuyDirection::BuyNo,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct AgentDecision {
    direction: ParsedDirection,
    amount_micro: i64,
}

#[derive(Debug, Serialize)]
struct MarketEvidenceManifest {
    schema_version: &'static str,
    run_id: String,
    model_requested: String,
    model_returned: String,
    llm_proxy_url: String,
    prompt_sha256: String,
    agent_response_sha256: String,
    external_agent_id: String,
    event_task_id: String,
    direction: ParsedDirection,
    amount_micro: i64,
    router_tx_id: String,
    router_landed: bool,
    pool_active: bool,
    final_state_root_hex: String,
    runtime_repo: String,
    cas: String,
    notes: Vec<&'static str>,
}

fn usage() -> &'static str {
    "usage: market_external_agent_current_kernel --runtime-repo <PATH> --cas <PATH> --run-id <ID> --constitution <constitution.md> --llm-proxy-url <URL> [--model <MODEL>] [--out <PATH>]"
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut runtime_repo: Option<PathBuf> = None;
    let mut cas: Option<PathBuf> = None;
    let mut run_id: Option<String> = None;
    let mut constitution: Option<PathBuf> = None;
    let mut llm_proxy_url: Option<String> = None;
    let mut model: Option<String> = None;
    let mut out: Option<PathBuf> = None;
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--runtime-repo" => {
                i += 1;
                runtime_repo = Some(
                    argv.get(i)
                        .ok_or("missing value after --runtime-repo")?
                        .into(),
                );
            }
            "--cas" => {
                i += 1;
                cas = Some(argv.get(i).ok_or("missing value after --cas")?.into());
            }
            "--run-id" => {
                i += 1;
                run_id = Some(argv.get(i).ok_or("missing value after --run-id")?.clone());
            }
            "--constitution" => {
                i += 1;
                constitution = Some(
                    argv.get(i)
                        .ok_or("missing value after --constitution")?
                        .into(),
                );
            }
            "--llm-proxy-url" => {
                i += 1;
                llm_proxy_url = Some(
                    argv.get(i)
                        .ok_or("missing value after --llm-proxy-url")?
                        .clone(),
                );
            }
            "--model" => {
                i += 1;
                model = Some(argv.get(i).ok_or("missing value after --model")?.clone());
            }
            "--out" => {
                i += 1;
                out = Some(argv.get(i).ok_or("missing value after --out")?.into());
            }
            "--help" | "-h" => return Err(usage().into()),
            other => return Err(format!("unknown arg: {other}")),
        }
        i += 1;
    }
    let runtime_repo = runtime_repo.ok_or("--runtime-repo required")?;
    let cas = cas.ok_or("--cas required")?;
    Ok(Args {
        out: out.unwrap_or_else(|| runtime_repo.join("external_agent_market_manifest.json")),
        runtime_repo,
        cas,
        run_id: run_id.ok_or("--run-id required")?,
        constitution: constitution.ok_or("--constitution required")?,
        llm_proxy_url: llm_proxy_url.ok_or("--llm-proxy-url required")?,
        model: model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
    })
}

fn sha256_hex(bytes: impl AsRef<[u8]>) -> String {
    let digest = Sha256::digest(bytes.as_ref());
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

fn hash_hex(h: &turingosv4::state::q_state::Hash) -> String {
    h.0.iter().map(|b| format!("{b:02x}")).collect()
}

fn extract_json_object(content: &str) -> Result<serde_json::Value, String> {
    let trimmed = content.trim();
    if let Ok(v) = serde_json::from_str(trimmed) {
        return Ok(v);
    }
    let start = trimmed
        .find('{')
        .ok_or("external agent response did not contain a JSON object")?;
    let end = trimmed
        .rfind('}')
        .ok_or("external agent response had no JSON object terminator")?;
    serde_json::from_str(&trimmed[start..=end])
        .map_err(|e| format!("parse external agent JSON object: {e}"))
}

fn parse_decision(content: &str) -> Result<AgentDecision, String> {
    let value = extract_json_object(content)?;
    let direction_raw = value
        .get("direction")
        .or_else(|| value.get("side"))
        .and_then(serde_json::Value::as_str)
        .ok_or("external agent JSON missing string `direction`")?
        .to_ascii_lowercase();
    let direction = match direction_raw.as_str() {
        "yes" | "buy_yes" | "buyyes" | "long_yes" | "long" => ParsedDirection::Yes,
        "no" | "buy_no" | "buyno" | "long_no" | "short" => ParsedDirection::No,
        other => return Err(format!("unsupported external agent direction `{other}`")),
    };
    let amount_micro = value
        .get("amount_micro")
        .or_else(|| value.get("amount"))
        .and_then(serde_json::Value::as_i64)
        .unwrap_or(DEFAULT_AMOUNT_MICRO);
    if !(1..=50_000).contains(&amount_micro) {
        return Err(format!(
            "external agent amount_micro must be in 1..=50000, got {amount_micro}"
        ));
    }
    Ok(AgentDecision {
        direction,
        amount_micro,
    })
}

fn build_agent_prompt(event_task_id: &str) -> String {
    format!(
        "You are an external TuringOS market participant, not kernel code.\n\
         Public event: task `{event_task_id}` has an active YES/NO constant-product market pool.\n\
         Decide one small test trade using public information only. Price is a signal, not truth.\n\
         Output exactly one JSON object with fields: direction = yes|no, amount_micro = integer 1..50000.\n\
         Do not include markdown, explanation, or private reasoning."
    )
}

async fn ask_external_agent(args: &Args, event_task_id: &str) -> Result<(String, String), String> {
    let prompt = build_agent_prompt(event_task_id);
    let client = ResilientLLMClient::new(&args.llm_proxy_url, 120, 2);
    let response = client
        .generate(&GenerateRequest {
            model: args.model.clone(),
            messages: vec![
                Message {
                    role: "system".into(),
                    content: "Return strict JSON only.".into(),
                },
                Message {
                    role: "user".into(),
                    content: prompt,
                },
            ],
            temperature: Some(0.0),
            max_tokens: Some(80),
        })
        .await
        .map_err(|e| format!("external agent LLM call failed: {e}"))?;
    Ok((response.content, response.model))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&argv) {
        Ok(args) => args,
        Err(msg) => {
            eprintln!("market_external_agent_current_kernel: {msg}");
            eprintln!("{}", usage());
            return ExitCode::from(2);
        }
    };

    if let Err(err) = run(args).await {
        eprintln!("market_external_agent_current_kernel: {err}");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

async fn run(args: Args) -> Result<(), String> {
    let event_task_id = format!("true-suite-market-{}", args.run_id);
    let prompt_sha256 = sha256_hex(build_agent_prompt(&event_task_id));
    let (agent_content, model_returned) = ask_external_agent(&args, &event_task_id).await?;
    let agent_response_sha256 = sha256_hex(&agent_content);
    let decision = parse_decision(&agent_content)?;

    let preseed = default_pput_preseed_pairs();
    let initial_q = genesis_with_balances(&preseed);
    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: args.runtime_repo.clone(),
        cas_path: args.cas.clone(),
        run_id: args.run_id.clone(),
        queue_capacity: 16,
        resume_existing_chain: false,
    };
    let bundle = build_chaintape_sequencer_with_initial_q(&cfg, initial_q)
        .map_err(|e| format!("fresh market boot failed: {e}"))?;
    let seq = bundle.sequencer.clone();

    let mut keypairs =
        AgentKeypairRegistry::open(&cfg.runtime_repo_path).map_err(|e| format!("{e}"))?;
    for id in [SPONSOR_AGENT, MARKET_PROVIDER_AGENT, TRADER_AGENT] {
        keypairs
            .get_or_create(&AgentId(id.to_string()))
            .map_err(|e| format!("create keypair for {id}: {e}"))?;
    }
    seq.set_agent_pubkeys(Arc::new(keypairs.manifest()))
        .map_err(|_| "agent pubkey manifest already set".to_string())?;

    let initial_root = seq
        .q_snapshot()
        .map_err(|e| format!("q_snapshot initial: {e:?}"))?
        .state_root_t;
    let task_open = make_real_task_open_signed_by(
        &mut keypairs,
        &event_task_id,
        SPONSOR_AGENT,
        initial_root,
        "true-suite-market",
        10,
    )
    .map_err(|e| format!("build TaskOpenTx: {e}"))?;
    seq.submit_agent_tx(task_open)
        .await
        .map_err(|e| format!("submit TaskOpenTx: {e:?}"))?;
    let after_open = tb8_await_state_root_advance(&seq, initial_root, 5_000)
        .await
        .map_err(|_| "TaskOpenTx did not advance state_root".to_string())?;

    let seed = make_real_market_seed_signed_by(
        &mut keypairs,
        after_open,
        &event_task_id,
        MARKET_PROVIDER_AGENT,
        MARKET_SEED_MICRO,
        "true-suite-market",
        11,
    )
    .map_err(|e| format!("build MarketSeedTx: {e}"))?;
    seq.submit_agent_tx(seed)
        .await
        .map_err(|e| format!("submit MarketSeedTx: {e:?}"))?;
    let after_seed = tb8_await_state_root_advance(&seq, after_open, 5_000)
        .await
        .map_err(|_| "MarketSeedTx did not advance state_root".to_string())?;

    let pool = make_real_cpmm_pool_signed_by(
        &mut keypairs,
        after_seed,
        &event_task_id,
        MARKET_PROVIDER_AGENT,
        MARKET_SEED_MICRO as u128,
        "true-suite-market",
    )
    .map_err(|e| format!("build CpmmPoolTx: {e}"))?;
    seq.submit_agent_tx(pool)
        .await
        .map_err(|e| format!("submit CpmmPoolTx: {e:?}"))?;
    let after_pool = tb8_await_state_root_advance(&seq, after_seed, 5_000)
        .await
        .map_err(|_| "CpmmPoolTx did not advance state_root".to_string())?;

    let pre_router_q = seq
        .q_snapshot()
        .map_err(|e| format!("q_snapshot before router: {e:?}"))?;
    let router = tb_real6a_invest_task_outcome_to_router_tx(
        &mut keypairs,
        after_pool,
        Some(&pre_router_q),
        TRADER_AGENT,
        &event_task_id,
        decision.direction.as_buy_direction(),
        decision.amount_micro,
        0,
        "true-suite-market",
    )
    .map_err(|e| format!("build external-agent router tx: {e:?}"))?;
    let router_tx_id = match &router {
        TypedTx::BuyWithCoinRouter(r) => r.tx_id.0.clone(),
        _ => unreachable!("router helper returns BuyWithCoinRouter"),
    };
    seq.submit_agent_tx(router)
        .await
        .map_err(|e| format!("submit BuyWithCoinRouterTx: {e:?}"))?;
    let after_router = tb8_await_state_root_advance(&seq, after_pool, 5_000)
        .await
        .map_err(|_| "BuyWithCoinRouterTx did not advance state_root".to_string())?;

    let seq_handle = seq.clone();
    bundle
        .shutdown()
        .await
        .map_err(|e| format!("market chaintape shutdown failed: {e}"))?;
    let post_q = seq_handle
        .q_snapshot()
        .map_err(|e| format!("post-drain q_snapshot: {e:?}"))?;
    let event_id = EventId(TaskId(event_task_id.clone()));
    let router_landed = post_q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId(TRADER_AGENT.to_string()))
        .and_then(|by_event| by_event.get(&event_id))
        .is_some();
    let pool_active = post_q
        .economic_state_t
        .cpmm_pools_t
        .0
        .contains_key(&event_id);

    let report = GenesisReport {
        constitution_hash: GenesisReport::hash_constitution_md(&args.constitution),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas_path: args.cas.display().to_string(),
        system_pubkey_hash: GenesisReport::hash_system_pubkey_manifest(&args.runtime_repo),
        agent_pubkeys_path: "agent_pubkeys.json".to_string(),
        initial_balances: preseed
            .iter()
            .map(|(agent, balance)| (agent.0.clone(), balance.micro_units()))
            .collect(),
        task_id: Some(event_task_id.clone()),
        task_open_tx: None,
        escrow_lock_tx: None,
        agent_model_assignment: vec![],
        model_assignment_manifest_cid: None,
        agent_role_assignment: vec![],
        role_assignment_manifest_cid: None,
    };
    report
        .write_to_runtime_repo(&args.runtime_repo)
        .map_err(|e| format!("write genesis_report.json: {e}"))?;

    let manifest = MarketEvidenceManifest {
        schema_version: "turingosv4.true_suite.market_external_agent.v1",
        run_id: args.run_id.clone(),
        model_requested: args.model,
        model_returned,
        llm_proxy_url: args.llm_proxy_url,
        prompt_sha256,
        agent_response_sha256,
        external_agent_id: TRADER_AGENT.to_string(),
        event_task_id,
        direction: decision.direction,
        amount_micro: decision.amount_micro,
        router_tx_id,
        router_landed,
        pool_active,
        final_state_root_hex: hash_hex(&after_router),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas: args.cas.display().to_string(),
        notes: vec![
            "external agent decision came from local LLM proxy before router tx construction",
            "raw prompt and raw response are not persisted; only sha256 hashes and parsed decision are recorded",
            "economic action is signed by AgentKeypairRegistry and submitted through Sequencer::submit_agent_tx",
        ],
    };
    let manifest_json =
        serde_json::to_string_pretty(&manifest).map_err(|e| format!("serialize manifest: {e}"))?;
    if let Some(parent) = args.out.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("create manifest parent: {e}"))?;
    }
    std::fs::write(&args.out, manifest_json).map_err(|e| format!("write manifest: {e}"))?;

    println!(
        "market_external_agent_current_kernel: router_tx_id={} direction={:?} amount_micro={} manifest={}",
        manifest.router_tx_id,
        manifest.direction,
        manifest.amount_micro,
        args.out.display()
    );
    Ok(())
}
