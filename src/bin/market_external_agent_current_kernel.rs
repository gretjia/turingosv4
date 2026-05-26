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

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::adapter::{
    genesis_with_balances, make_real_cpmm_pool_signed_by, make_real_escrow_lock_signed_by,
    make_real_market_seed_signed_by, make_real_task_open_signed_by, make_real_worktx_signed_by,
    tb_real6a_invest_task_outcome_to_router_tx, tb8_await_state_root_advance,
};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::bootstrap::default_pput_preseed_pairs;
use turingosv4::runtime::genesis_report::GenesisReport;
use turingosv4::runtime::proposal_telemetry::{
    ProposalTelemetry, TokenCounts, write_to_cas as write_proposal_telemetry_to_cas,
};
use turingosv4::runtime::{RuntimeChaintapeConfig, build_chaintape_sequencer_with_initial_q};
use turingosv4::state::q_state::{AgentId, CpmmPool, EconomicState, Hash, TaskId, TxId};
use turingosv4::state::router_quote::{QuoteDirection, quote_buy_with_coin_router};
use turingosv4::state::typed_tx::{BuyDirection, EventId, TypedTx};

const SPONSOR_AGENT: &str = "Agent_user_0";
const MARKET_PROVIDER_AGENT: &str = "ExternalMarketMakerBudget";
const TRADER_AGENT: &str = "Agent_0";
const DEFAULT_MODEL: &str = "deepseek-chat";
const DEFAULT_AMOUNT_MICRO: i64 = 1_000;
const MARKET_SEED_MICRO: i64 = 100_000;
const TASK_ESCROW_MICRO: i64 = 10_000;
const WORK_STAKE_MICRO: i64 = 100;

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

#[derive(Debug, Clone, Serialize)]
struct PoolReserveSnapshot {
    pool_yes_units: u128,
    pool_no_units: u128,
    k_product: u128,
    status: String,
}

#[derive(Debug, Clone, Serialize)]
struct RouterEconomicsSnapshot {
    pay_coin_micro: i64,
    pool_before: PoolReserveSnapshot,
    pool_after: PoolReserveSnapshot,
    quote_out_shares_units: u128,
    quote_get_shares_units: u128,
    price_effective_numerator: Option<u128>,
    price_effective_denominator: Option<u128>,
    quote_liquidity_warning: String,
    buyer_coin_before_micro: i64,
    buyer_coin_after_micro: i64,
    buyer_coin_delta_micro: i64,
    buyer_chosen_side_before_units: u128,
    buyer_chosen_side_after_units: u128,
    buyer_chosen_side_delta_units: u128,
    collateral_before_micro: i64,
    collateral_after_micro: i64,
    total_coin_before_micro: i64,
    total_coin_after_micro: i64,
    sum_yes_after_units: u128,
    sum_no_after_units: u128,
    k_non_decreasing: bool,
    pool_delta_matches_quote: bool,
    mint_and_swap_retained_plus_out_holds: bool,
    buyer_coin_debited_exactly: bool,
    total_coin_conserved: bool,
    complete_set_balanced_after: bool,
}

#[derive(Debug, Clone, Serialize)]
struct MarketDecisionCapsule {
    schema_version: &'static str,
    run_id: String,
    external_agent_id: String,
    event_task_id: String,
    model_returned: String,
    prompt_sha256: String,
    agent_response_sha256: String,
    direction: ParsedDirection,
    amount_micro: i64,
}

#[derive(Debug, Clone, Serialize)]
struct MarketEvaluationCapsule {
    schema_version: &'static str,
    run_id: String,
    decision_capsule_cid: String,
    router_tx_id: String,
    router_landed: bool,
    pool_active: bool,
    k_non_decreasing: bool,
    pool_delta_matches_quote: bool,
    mint_and_swap_retained_plus_out_holds: bool,
    buyer_coin_debited_exactly: bool,
    total_coin_conserved: bool,
    complete_set_balanced_after: bool,
    benchmark_verdict: String,
    failure_class: Option<String>,
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
    decision_capsule_cid: String,
    evaluation_capsule_cid: String,
    proposal_telemetry_cid: String,
    router_tx_id: String,
    router_landed: bool,
    work_tx_id: String,
    work_tx_landed: bool,
    pool_active: bool,
    router_economics: RouterEconomicsSnapshot,
    closure_scope: &'static str,
    full_system_participation_required: bool,
    final_closure_possible: bool,
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

fn hash_from_hex_digest(hex: &str) -> Result<Hash, String> {
    if hex.len() != 64 {
        return Err(format!("sha256 hex digest must be 64 chars, got {hex}"));
    }
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        bytes[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)
            .map_err(|e| format!("parse sha256 hex byte {i}: {e}"))?;
    }
    Ok(Hash::from_bytes(bytes))
}

fn put_json<T: Serialize>(
    cas_path: &PathBuf,
    value: &T,
    object_type: ObjectType,
    creator: &str,
    logical_t: u64,
    schema_id: &str,
) -> Result<Cid, String> {
    let bytes =
        serde_json::to_vec(value).map_err(|e| format!("serialize CAS object {schema_id}: {e}"))?;
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    cas.put(
        &bytes,
        object_type,
        creator,
        logical_t,
        Some(schema_id.to_string()),
    )
    .map_err(|e| format!("put CAS object {schema_id}: {e}"))
}

fn quote_direction(direction: ParsedDirection) -> QuoteDirection {
    match direction {
        ParsedDirection::Yes => QuoteDirection::BuyYes,
        ParsedDirection::No => QuoteDirection::BuyNo,
    }
}

fn pool_snapshot(pool: &CpmmPool) -> PoolReserveSnapshot {
    PoolReserveSnapshot {
        pool_yes_units: pool.pool_yes.units,
        pool_no_units: pool.pool_no.units,
        k_product: pool.pool_yes.units * pool.pool_no.units,
        status: format!("{:?}", pool.status),
    }
}

fn buyer_side_units(
    econ: &EconomicState,
    buyer: &AgentId,
    event_id: &EventId,
    direction: ParsedDirection,
) -> u128 {
    econ.conditional_share_balances_t
        .0
        .get(buyer)
        .and_then(|by_event| by_event.get(event_id))
        .map(|pair| match direction {
            ParsedDirection::Yes => pair.yes.units,
            ParsedDirection::No => pair.no.units,
        })
        .unwrap_or(0)
}

fn coin_balance_micro(econ: &EconomicState, agent: &AgentId) -> i64 {
    econ.balances_t
        .0
        .get(agent)
        .copied()
        .unwrap_or_default()
        .micro_units()
}

fn collateral_micro(econ: &EconomicState, event_id: &EventId) -> i64 {
    econ.conditional_collateral_t
        .0
        .get(event_id)
        .copied()
        .unwrap_or_default()
        .micro_units()
}

fn sum_yes_no_for_event(econ: &EconomicState, event_id: &EventId) -> (u128, u128) {
    let mut yes: u128 = 0;
    let mut no: u128 = 0;
    for owner_map in econ.conditional_share_balances_t.0.values() {
        if let Some(pair) = owner_map.get(event_id) {
            yes += pair.yes.units;
            no += pair.no.units;
        }
    }
    if let Some(pool) = econ.cpmm_pools_t.0.get(event_id) {
        yes += pool.pool_yes.units;
        no += pool.pool_no.units;
    }
    (yes, no)
}

fn total_coin_micro(econ: &EconomicState) -> Result<i64, String> {
    let mut sum: i128 = 0;
    for v in econ.balances_t.0.values() {
        sum += v.micro_units() as i128;
    }
    for esc in econ.escrows_t.0.values() {
        sum += esc.amount.micro_units() as i128;
    }
    for stake in econ.stakes_t.0.values() {
        sum += stake.amount.micro_units() as i128;
    }
    for case in econ.challenge_cases_t.0.values() {
        sum += case.bond.micro_units() as i128;
    }
    for v in econ.conditional_collateral_t.0.values() {
        sum += v.micro_units() as i128;
    }
    i64::try_from(sum).map_err(|_| format!("total coin sum out of i64 range: {sum}"))
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
    let decision_capsule = MarketDecisionCapsule {
        schema_version: "turingosv4.true_suite.market_decision_capsule.v1",
        run_id: args.run_id.clone(),
        external_agent_id: TRADER_AGENT.to_string(),
        event_task_id: event_task_id.clone(),
        model_returned: model_returned.clone(),
        prompt_sha256: prompt_sha256.clone(),
        agent_response_sha256: agent_response_sha256.clone(),
        direction: decision.direction,
        amount_micro: decision.amount_micro,
    };
    let decision_capsule_cid = put_json(
        &args.cas,
        &decision_capsule,
        ObjectType::EvidenceCapsule,
        "market-decision",
        2,
        "turingosv4.true_suite.market_decision_capsule.v1",
    )?;

    let mut initial_balances = default_pput_preseed_pairs();
    initial_balances.push((
        AgentId(MARKET_PROVIDER_AGENT.to_string()),
        MicroCoin::from_micro_units(5_000_000),
    ));
    let initial_q = genesis_with_balances(&initial_balances);
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
    let event_id = EventId(TaskId(event_task_id.clone()));
    let buyer_id = AgentId(TRADER_AGENT.to_string());
    let pool_before = pre_router_q
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&event_id)
        .cloned()
        .ok_or("pool missing before router")?;
    let quote = quote_buy_with_coin_router(
        &pool_before,
        turingosv4::economy::money::MicroCoin::from_micro_units(decision.amount_micro),
        quote_direction(decision.direction),
    )
    .ok_or("router quote unavailable before external-agent tx")?;
    let buyer_coin_before_micro = coin_balance_micro(&pre_router_q.economic_state_t, &buyer_id);
    let buyer_chosen_side_before_units = buyer_side_units(
        &pre_router_q.economic_state_t,
        &buyer_id,
        &event_id,
        decision.direction,
    );
    let collateral_before_micro = collateral_micro(&pre_router_q.economic_state_t, &event_id);
    let total_coin_before_micro = total_coin_micro(&pre_router_q.economic_state_t)?;
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

    let post_router_q = seq
        .q_snapshot()
        .map_err(|e| format!("post-router q_snapshot: {e:?}"))?;
    let router_landed = post_router_q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId(TRADER_AGENT.to_string()))
        .and_then(|by_event| by_event.get(&event_id))
        .is_some();
    let pool_active = post_router_q
        .economic_state_t
        .cpmm_pools_t
        .0
        .contains_key(&event_id);
    let pool_after = post_router_q
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&event_id)
        .cloned()
        .ok_or("pool missing after router")?;
    let buyer_coin_after_micro = coin_balance_micro(&post_router_q.economic_state_t, &buyer_id);
    let buyer_chosen_side_after_units = buyer_side_units(
        &post_router_q.economic_state_t,
        &buyer_id,
        &event_id,
        decision.direction,
    );
    let collateral_after_micro = collateral_micro(&post_router_q.economic_state_t, &event_id);
    let total_coin_after_micro = total_coin_micro(&post_router_q.economic_state_t)?;
    let (sum_yes_after_units, sum_no_after_units) =
        sum_yes_no_for_event(&post_router_q.economic_state_t, &event_id);
    let pool_delta_matches_quote = match decision.direction {
        ParsedDirection::Yes => {
            pool_after.pool_no.units == pool_before.pool_no.units + decision.amount_micro as u128
                && pool_after.pool_yes.units + quote.out_shares.units == pool_before.pool_yes.units
        }
        ParsedDirection::No => {
            pool_after.pool_yes.units == pool_before.pool_yes.units + decision.amount_micro as u128
                && pool_after.pool_no.units + quote.out_shares.units == pool_before.pool_no.units
        }
    };
    let buyer_chosen_side_delta_units =
        buyer_chosen_side_after_units.saturating_sub(buyer_chosen_side_before_units);
    let router_economics = RouterEconomicsSnapshot {
        pay_coin_micro: decision.amount_micro,
        pool_before: pool_snapshot(&pool_before),
        pool_after: pool_snapshot(&pool_after),
        quote_out_shares_units: quote.out_shares.units,
        quote_get_shares_units: quote.get_shares.units,
        price_effective_numerator: quote.price_effective.map(|p| p.numerator),
        price_effective_denominator: quote.price_effective.map(|p| p.denominator),
        quote_liquidity_warning: format!("{:?}", quote.liquidity_warning),
        buyer_coin_before_micro,
        buyer_coin_after_micro,
        buyer_coin_delta_micro: buyer_coin_before_micro - buyer_coin_after_micro,
        buyer_chosen_side_before_units,
        buyer_chosen_side_after_units,
        buyer_chosen_side_delta_units,
        collateral_before_micro,
        collateral_after_micro,
        total_coin_before_micro,
        total_coin_after_micro,
        sum_yes_after_units,
        sum_no_after_units,
        k_non_decreasing: pool_after.pool_yes.units * pool_after.pool_no.units
            >= pool_before.pool_yes.units * pool_before.pool_no.units,
        pool_delta_matches_quote,
        mint_and_swap_retained_plus_out_holds: quote.get_shares.units
            == decision.amount_micro as u128 + quote.out_shares.units
            && buyer_chosen_side_delta_units == quote.get_shares.units,
        buyer_coin_debited_exactly: buyer_coin_before_micro - buyer_coin_after_micro
            == decision.amount_micro,
        total_coin_conserved: total_coin_before_micro == total_coin_after_micro,
        complete_set_balanced_after: sum_yes_after_units == sum_no_after_units
            && sum_yes_after_units == collateral_after_micro as u128,
    };
    let market_invariants_hold = router_landed
        && pool_active
        && router_economics.k_non_decreasing
        && router_economics.pool_delta_matches_quote
        && router_economics.mint_and_swap_retained_plus_out_holds
        && router_economics.buyer_coin_debited_exactly
        && router_economics.total_coin_conserved
        && router_economics.complete_set_balanced_after;
    let evaluation = MarketEvaluationCapsule {
        schema_version: "turingosv4.true_suite.market_evaluation_capsule.v1",
        run_id: args.run_id.clone(),
        decision_capsule_cid: decision_capsule_cid.hex(),
        router_tx_id: router_tx_id.clone(),
        router_landed,
        pool_active,
        k_non_decreasing: router_economics.k_non_decreasing,
        pool_delta_matches_quote: router_economics.pool_delta_matches_quote,
        mint_and_swap_retained_plus_out_holds: router_economics
            .mint_and_swap_retained_plus_out_holds,
        buyer_coin_debited_exactly: router_economics.buyer_coin_debited_exactly,
        total_coin_conserved: router_economics.total_coin_conserved,
        complete_set_balanced_after: router_economics.complete_set_balanced_after,
        benchmark_verdict: if market_invariants_hold {
            "market_router_invariants_hold"
        } else {
            "market_router_invariant_failure"
        }
        .to_string(),
        failure_class: (!market_invariants_hold).then(|| "kernel_invariant_failure".to_string()),
    };
    let evaluation_capsule_cid = put_json(
        &args.cas,
        &evaluation,
        ObjectType::ProposalPayload,
        "market-evaluation",
        3,
        "turingosv4.true_suite.market_evaluation_capsule.v1",
    )?;
    let proposal_telemetry_cid = {
        let telemetry = ProposalTelemetry::new_root(
            AgentId(TRADER_AGENT.to_string()),
            hash_from_hex_digest(&prompt_sha256)?,
            evaluation_capsule_cid,
            "market_external_agent_decision".to_string(),
            TokenCounts {
                prompt_tokens: 0,
                completion_tokens: 0,
                tool_tokens: 1,
            },
            format!("{TRADER_AGENT}.market.b0"),
        );
        let mut cas = CasStore::open(&args.cas).map_err(|e| format!("open CAS: {e}"))?;
        write_proposal_telemetry_to_cas(&mut cas, &telemetry, "market-proposal-telemetry", 4)
            .map_err(|e| format!("write ProposalTelemetry: {e}"))?
    };

    let escrow = make_real_escrow_lock_signed_by(
        &mut keypairs,
        &event_task_id,
        SPONSOR_AGENT,
        TASK_ESCROW_MICRO,
        after_router,
        "true-suite-market",
        12,
    )
    .map_err(|e| format!("build EscrowLockTx: {e}"))?;
    seq.submit_agent_tx(escrow)
        .await
        .map_err(|e| format!("submit EscrowLockTx: {e:?}"))?;
    let after_escrow = tb8_await_state_root_advance(&seq, after_router, 5_000)
        .await
        .map_err(|_| "EscrowLockTx did not advance state_root".to_string())?;

    let work = make_real_worktx_signed_by(
        &mut keypairs,
        &event_task_id,
        TRADER_AGENT,
        after_escrow,
        WORK_STAKE_MICRO,
        "true-suite-market",
        proposal_telemetry_cid,
        true,
        13,
    )
    .map_err(|e| format!("build WorkTx: {e}"))?;
    let work_tx_id = match &work {
        TypedTx::Work(w) => w.tx_id.0.clone(),
        _ => unreachable!("work helper returns WorkTx"),
    };
    seq.submit_agent_tx(work)
        .await
        .map_err(|e| format!("submit WorkTx: {e:?}"))?;
    let after_work = tb8_await_state_root_advance(&seq, after_escrow, 5_000)
        .await
        .map_err(|_| "WorkTx did not advance state_root".to_string())?;

    let seq_handle = seq.clone();
    bundle
        .shutdown()
        .await
        .map_err(|e| format!("market chaintape shutdown failed: {e}"))?;
    let post_q = seq_handle
        .q_snapshot()
        .map_err(|e| format!("post-drain q_snapshot: {e:?}"))?;

    let report = GenesisReport {
        constitution_hash: GenesisReport::hash_constitution_md(&args.constitution),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas_path: args.cas.display().to_string(),
        system_pubkey_hash: GenesisReport::hash_system_pubkey_manifest(&args.runtime_repo),
        agent_pubkeys_path: "agent_pubkeys.json".to_string(),
        initial_balances: initial_balances
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
        decision_capsule_cid: decision_capsule_cid.hex(),
        evaluation_capsule_cid: evaluation_capsule_cid.hex(),
        proposal_telemetry_cid: proposal_telemetry_cid.hex(),
        router_tx_id,
        router_landed,
        work_tx_id: work_tx_id.clone(),
        work_tx_landed: post_q
            .economic_state_t
            .stakes_t
            .0
            .contains_key(&TxId(work_tx_id)),
        pool_active,
        router_economics,
        closure_scope: "single_sample_market_external_agent_full_system_liveness",
        full_system_participation_required: true,
        final_closure_possible: false,
        final_state_root_hex: hash_hex(&after_work),
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
