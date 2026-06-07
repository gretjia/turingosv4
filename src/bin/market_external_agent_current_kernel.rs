//! True-suite market/economy evidence helper.
//!
//! This binary is a runner helper: it asks an external LLM, via the local
//! OpenAI-compatible proxy, for two role-separated market actions, then
//! submits those actions as signed `BuyWithCoinRouterTx`s through the current
//! ChainTape sequencer.
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
    tb8_await_state_root_advance, tb_real6a_invest_task_outcome_to_router_tx,
};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::bootstrap::default_pput_preseed_pairs;
use turingosv4::runtime::genesis_report::GenesisReport;
use turingosv4::runtime::proposal_telemetry::{
    write_to_cas as write_proposal_telemetry_to_cas, ProposalTelemetry, TokenCounts,
};
use turingosv4::runtime::{build_chaintape_sequencer_with_initial_q, RuntimeChaintapeConfig};
use turingosv4::state::q_state::{AgentId, CpmmPool, EconomicState, Hash, TaskId, TxId};
use turingosv4::state::router_quote::{quote_buy_with_coin_router, QuoteDirection};
use turingosv4::state::typed_tx::{BuyDirection, EventId, TypedTx};

const SPONSOR_AGENT: &str = "Agent_user_0";
const MARKET_PROVIDER_AGENT: &str = "Agent_2";
const TRADER_AGENT: &str = "Agent_0";
const COUNTER_TRADER_AGENT: &str = "Agent_3";
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone)]
struct TradePlan {
    external_agent_id: &'static str,
    role: &'static str,
    required_direction: ParsedDirection,
    decision_logical_t: u64,
    router_seq_no: u128,
}

#[derive(Debug, Clone, Serialize)]
struct RouterTradeManifest {
    external_agent_id: String,
    role: &'static str,
    model_returned: String,
    prompt_sha256: String,
    agent_response_sha256: String,
    direction: ParsedDirection,
    amount_micro: i64,
    decision_capsule_cid: String,
    router_tx_id: String,
    router_landed: bool,
    router_economics: RouterEconomicsSnapshot,
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
    // CONFORMANCE FIX #2 (evidence-cas-anchor): `direction` / `amount_micro` are
    // now optional and the capsule carries `parse_error` / `direction_mismatch`
    // so that EVERY completed external LLM call (token already spent) is anchored
    // in CAS — including the failure branch — mirroring swebench's
    // `parse_patch_claim` which captures `parse_error` and ALWAYS writes its
    // claim capsule. `expected_direction` records the role's required side for
    // the mismatch case. See tests/constitution_external_attempt_anchored_on_failure.rs.
    direction: Option<ParsedDirection>,
    amount_micro: Option<i64>,
    expected_direction: ParsedDirection,
    parse_error: Option<String>,
    direction_mismatch: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct MarketEvaluationCapsule {
    schema_version: &'static str,
    run_id: String,
    decision_capsule_cid: String,
    router_tx_id: String,
    router_tx_ids: Vec<String>,
    buy_yes_count: usize,
    buy_no_count: usize,
    no_side_market_action_txs: usize,
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
    router_tx_ids: Vec<String>,
    router_trades: Vec<RouterTradeManifest>,
    buy_yes_count: usize,
    buy_no_count: usize,
    no_side_market_action_txs: usize,
    router_landed: bool,
    work_tx_id: String,
    work_tx_landed: bool,
    work_tx_count_for_task: usize,
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

fn direction_token(direction: ParsedDirection) -> &'static str {
    match direction {
        ParsedDirection::Yes => "yes",
        ParsedDirection::No => "no",
    }
}

fn build_agent_prompt(
    event_task_id: &str,
    role: &str,
    required_direction: ParsedDirection,
) -> String {
    let prompt = format!(
        "You are an external TuringOS market participant, not kernel code.\n\
         Role: {role}.\n\
         Public event: task `{event_task_id}` has an active YES/NO constant-product market pool.\n\
         For this two-sided market liveness probe, your assigned public side is `{}`.\n\
         Decide one small test trade using public information only. Price is a signal, not truth.\n\
         Output exactly one JSON object with fields: direction = yes|no, amount_micro = integer 1..50000.\n\
         The direction field must be `{}` for this role.\n\
         Do not include markdown, explanation, or private reasoning."
        ,
        direction_token(required_direction),
        direction_token(required_direction),
    );
    // CONFORMANCE FIX #5 (goodhart-shield): runtime PPUT-context-leak guard at
    // the market prompt-delivery boundary (Art. III.4), before this prompt
    // crosses the external-LLM call. Defense-in-depth: no PPUT scalar may enter
    // agent context even if a future template change or interpolated field leaks
    // one. Guard lives in the trust-root-pinned `prompt_guard.rs` (unchanged).
    // Gate: tests/constitution_metric_leak_guard_wired.rs.
    turingosv4::sdk::prompt_guard::assert_no_metric_leak(&prompt);
    prompt
}

async fn ask_external_agent(args: &Args, prompt: String) -> Result<(String, String), String> {
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
    let trade_plans = [
        TradePlan {
            external_agent_id: TRADER_AGENT,
            role: "optimistic YES-side market participant",
            required_direction: ParsedDirection::Yes,
            decision_logical_t: 2,
            router_seq_no: 0,
        },
        TradePlan {
            external_agent_id: COUNTER_TRADER_AGENT,
            role: "skeptical NO-side market participant",
            required_direction: ParsedDirection::No,
            decision_logical_t: 3,
            router_seq_no: 1,
        },
    ];

    let mut initial_balances = default_pput_preseed_pairs();
    for required_agent in [MARKET_PROVIDER_AGENT, TRADER_AGENT, COUNTER_TRADER_AGENT] {
        if !initial_balances
            .iter()
            .any(|(agent, _)| agent.0 == required_agent)
        {
            initial_balances.push((
                AgentId(required_agent.to_string()),
                MicroCoin::from_micro_units(5_000_000),
            ));
        }
    }
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
    for id in [
        SPONSOR_AGENT,
        MARKET_PROVIDER_AGENT,
        TRADER_AGENT,
        COUNTER_TRADER_AGENT,
    ] {
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

    let event_id = EventId(TaskId(event_task_id.clone()));
    let mut current_root = after_pool;
    let mut router_trades = Vec::new();

    for plan in trade_plans {
        let prompt = build_agent_prompt(&event_task_id, plan.role, plan.required_direction);
        let prompt_sha256 = sha256_hex(&prompt);
        let (agent_content, model_returned) = ask_external_agent(&args, prompt).await?;
        let agent_response_sha256 = sha256_hex(&agent_content);

        // CONFORMANCE FIX #2 (evidence-cas-anchor): the external LLM call above
        // has ALREADY completed — tokens are spent and the response is the
        // run's externalized attempt. Previously `parse_decision(..)?` and the
        // direction-mismatch `return Err` short-circuited BEFORE the first
        // `put_json`, so a parse/guard failure left NO CAS object, NO L4 WorkTx,
        // and NO L4.E — the spent attempt was unreconstructable from tape. We now
        // capture the parse/direction error into capsule fields and ALWAYS write
        // the `MarketDecisionCapsule` (mirroring swebench's `parse_patch_claim`,
        // which records `parse_error` and always anchors its claim capsule). Only
        // AFTER the capsule is anchored do we decide whether to abort.
        let parsed = parse_decision(&agent_content);
        let parse_error = parsed.as_ref().err().cloned();
        let (decision_direction, decision_amount_micro) = match parsed.as_ref() {
            Ok(d) => (Some(d.direction), Some(d.amount_micro)),
            Err(_) => (None, None),
        };
        let direction_mismatch = match decision_direction {
            Some(dir) if dir != plan.required_direction => Some(format!(
                "external agent {} returned {:?}, expected {:?} for role {}",
                plan.external_agent_id, dir, plan.required_direction, plan.role
            )),
            _ => None,
        };

        let decision_capsule = MarketDecisionCapsule {
            schema_version: "turingosv4.true_suite.market_decision_capsule.v1",
            run_id: args.run_id.clone(),
            external_agent_id: plan.external_agent_id.to_string(),
            event_task_id: event_task_id.clone(),
            model_returned: model_returned.clone(),
            prompt_sha256: prompt_sha256.clone(),
            agent_response_sha256: agent_response_sha256.clone(),
            direction: decision_direction,
            amount_micro: decision_amount_micro,
            expected_direction: plan.required_direction,
            parse_error: parse_error.clone(),
            direction_mismatch: direction_mismatch.clone(),
        };
        let decision_capsule_cid = put_json(
            &args.cas,
            &decision_capsule,
            ObjectType::EvidenceCapsule,
            "market-decision",
            plan.decision_logical_t,
            "turingosv4.true_suite.market_decision_capsule.v1",
        )?;

        // The completed attempt is now anchored in CAS. Abort the run only AFTER
        // the evidence is durable — never before.
        if let Some(err) = parse_error {
            return Err(format!(
                "external agent {} response failed to parse (decision capsule {} anchored): {err}",
                plan.external_agent_id,
                decision_capsule_cid.hex()
            ));
        }
        if let Some(mismatch) = direction_mismatch {
            return Err(format!(
                "{mismatch} (decision capsule {} anchored)",
                decision_capsule_cid.hex()
            ));
        }
        let decision = parsed.expect("parse_error handled above");

        let pre_router_q = seq
            .q_snapshot()
            .map_err(|e| format!("q_snapshot before router: {e:?}"))?;
        let buyer_id = AgentId(plan.external_agent_id.to_string());
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
            current_root,
            Some(&pre_router_q),
            plan.external_agent_id,
            &event_task_id,
            decision.direction.as_buy_direction(),
            decision.amount_micro,
            plan.router_seq_no,
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
        let after_router = tb8_await_state_root_advance(&seq, current_root, 5_000)
            .await
            .map_err(|_| "BuyWithCoinRouterTx did not advance state_root".to_string())?;

        let post_router_q = seq
            .q_snapshot()
            .map_err(|e| format!("post-router q_snapshot: {e:?}"))?;
        let router_landed = post_router_q
            .economic_state_t
            .conditional_share_balances_t
            .0
            .get(&buyer_id)
            .and_then(|by_event| by_event.get(&event_id))
            .is_some();
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
                pool_after.pool_no.units
                    == pool_before.pool_no.units + decision.amount_micro as u128
                    && pool_after.pool_yes.units + quote.out_shares.units
                        == pool_before.pool_yes.units
            }
            ParsedDirection::No => {
                pool_after.pool_yes.units
                    == pool_before.pool_yes.units + decision.amount_micro as u128
                    && pool_after.pool_no.units + quote.out_shares.units
                        == pool_before.pool_no.units
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

        router_trades.push(RouterTradeManifest {
            external_agent_id: plan.external_agent_id.to_string(),
            role: plan.role,
            model_returned,
            prompt_sha256,
            agent_response_sha256,
            direction: decision.direction,
            amount_micro: decision.amount_micro,
            decision_capsule_cid: decision_capsule_cid.hex(),
            router_tx_id,
            router_landed,
            router_economics,
        });
        current_root = after_router;
    }

    let buy_yes_count = router_trades
        .iter()
        .filter(|trade| trade.direction == ParsedDirection::Yes)
        .count();
    let buy_no_count = router_trades
        .iter()
        .filter(|trade| trade.direction == ParsedDirection::No)
        .count();
    let no_side_market_action_txs = buy_no_count;
    let primary_trade = router_trades
        .first()
        .cloned()
        .ok_or("market runner produced no router trades")?;
    let router_tx_ids: Vec<String> = router_trades
        .iter()
        .map(|trade| trade.router_tx_id.clone())
        .collect();
    let router_landed = router_trades.iter().all(|trade| trade.router_landed);
    let pool_active = router_trades
        .last()
        .map(|trade| trade.router_economics.pool_after.status == "Active")
        .unwrap_or(false);
    let market_invariants_hold = router_trades.iter().all(|trade| {
        let economics = &trade.router_economics;
        trade.router_landed
            && economics.k_non_decreasing
            && economics.pool_delta_matches_quote
            && economics.mint_and_swap_retained_plus_out_holds
            && economics.buyer_coin_debited_exactly
            && economics.total_coin_conserved
            && economics.complete_set_balanced_after
    });
    let evaluation = MarketEvaluationCapsule {
        schema_version: "turingosv4.true_suite.market_evaluation_capsule.v1",
        run_id: args.run_id.clone(),
        decision_capsule_cid: primary_trade.decision_capsule_cid.clone(),
        router_tx_id: primary_trade.router_tx_id.clone(),
        router_tx_ids: router_tx_ids.clone(),
        buy_yes_count,
        buy_no_count,
        no_side_market_action_txs,
        router_landed,
        pool_active,
        k_non_decreasing: router_trades
            .iter()
            .all(|trade| trade.router_economics.k_non_decreasing),
        pool_delta_matches_quote: router_trades
            .iter()
            .all(|trade| trade.router_economics.pool_delta_matches_quote),
        mint_and_swap_retained_plus_out_holds: router_trades
            .iter()
            .all(|trade| trade.router_economics.mint_and_swap_retained_plus_out_holds),
        buyer_coin_debited_exactly: router_trades
            .iter()
            .all(|trade| trade.router_economics.buyer_coin_debited_exactly),
        total_coin_conserved: router_trades
            .iter()
            .all(|trade| trade.router_economics.total_coin_conserved),
        complete_set_balanced_after: router_trades
            .iter()
            .all(|trade| trade.router_economics.complete_set_balanced_after),
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
            hash_from_hex_digest(&primary_trade.prompt_sha256)?,
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
        current_root,
        "true-suite-market",
        12,
    )
    .map_err(|e| format!("build EscrowLockTx: {e}"))?;
    seq.submit_agent_tx(escrow)
        .await
        .map_err(|e| format!("submit EscrowLockTx: {e:?}"))?;
    let after_escrow = tb8_await_state_root_advance(&seq, current_root, 5_000)
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
    let work_tx_id_key = TxId(work_tx_id.clone());
    let work_tx_landed = post_q
        .economic_state_t
        .stakes_t
        .0
        .contains_key(&work_tx_id_key);
    let work_tx_count_for_task = post_q
        .economic_state_t
        .stakes_t
        .0
        .values()
        .filter(|entry| entry.task_id.0 == event_task_id)
        .count();
    let market_domain_final_closure_possible = buy_yes_count > 0
        && buy_no_count > 0
        && market_invariants_hold
        && work_tx_landed
        && work_tx_count_for_task == 1;

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
        model_returned: primary_trade.model_returned.clone(),
        llm_proxy_url: args.llm_proxy_url,
        prompt_sha256: primary_trade.prompt_sha256.clone(),
        agent_response_sha256: primary_trade.agent_response_sha256.clone(),
        external_agent_id: primary_trade.external_agent_id.clone(),
        event_task_id,
        direction: primary_trade.direction,
        amount_micro: primary_trade.amount_micro,
        decision_capsule_cid: primary_trade.decision_capsule_cid.clone(),
        evaluation_capsule_cid: evaluation_capsule_cid.hex(),
        proposal_telemetry_cid: proposal_telemetry_cid.hex(),
        router_tx_id: primary_trade.router_tx_id.clone(),
        router_tx_ids,
        router_trades,
        buy_yes_count,
        buy_no_count,
        no_side_market_action_txs,
        router_landed,
        work_tx_id: work_tx_id.clone(),
        work_tx_landed,
        work_tx_count_for_task,
        pool_active,
        router_economics: primary_trade.router_economics.clone(),
        closure_scope: "two_sided_market_external_agent_full_system_liveness",
        full_system_participation_required: true,
        final_closure_possible: market_domain_final_closure_possible,
        final_state_root_hex: hash_hex(&after_work),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas: args.cas.display().to_string(),
        notes: vec![
            "role-separated external agent decisions came from local LLM proxy before router tx construction",
            "raw prompts and raw responses are not persisted; only sha256 hashes and parsed decisions are recorded",
            "YES and NO market actions are signed by AgentKeypairRegistry and submitted through Sequencer::submit_agent_tx",
            "the reward path remains a single WorkTx for the task escrow; two-sided market activity is router-side",
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
