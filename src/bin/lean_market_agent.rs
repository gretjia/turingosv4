//! lean_market_agent — price-routed Lean proof-search market (P0-A/C/G).
//!
//! The capability experiment for the Hard Lean Market Go/No-Go. N live DeepSeek
//! agents (via the local provider proxy) collaboratively search for a Lean proof of
//! a FIXED target theorem on the canonical ChainTape. Each agent reads a SHIELDED
//! market view (node ids + integer-rational prices + recent attempts' bodies + their
//! Lean error feedback — no judge internals, no other balances), picks a prior
//! attempt to refine (parent selection governed by `--policy`), calls DeepSeek for a
//! refined proof BODY, and the **real Lean kernel** (`LeanJudge`) verdicts it.
//!
//! Model: per node = ONE proof attempt. EVERY attempt (Verified or Failed) becomes a
//! priced per-task node (WorkTx-Long confidence-scaled + ChallengeTx-Short) so the
//! market can route refinement effort by price; failed attempts stay on tape as
//! `is_verified=false` nodes (the market's search frontier). OMEGA fires ONLY on a
//! `LeanVerdictKind::Verified` attempt — never a `sorry` (prereg §3). PPUT =
//! golden-path tokens / (total tokens × wall-clock).
//!
//! `--policy` (one binary, all arms — covers P0-A market + P0-G A0 + P0-C baselines):
//!   market         price-routed parent selection (boltzmann over the live price index)
//!   shuffled_price A0 ablation: byte-identical to market EXCEPT the price vector fed
//!                  to parent-selection is randomly permuted each round (kills routing)
//!   no_price       shared tape, uniform-random parent (prices stripped from selection)
//!   single         one agent refining its own chain (B1)
//!   {parallel,majority,best_first} land in P0-C.
//!
//! Class 2 (new binary; reuses g1 tx machinery + LeanJudge; no §6 surface).

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::economy::money::MicroCoin;
use turingosv4::judges::lean_judge::default_lean_bin;
use turingosv4::judges::lean_theorem_bank::{
    default_lake_bin, load_bank, mathlib_lean_path, LeanTheorem,
};
use turingosv4::runtime::adapter::{
    genesis_with_balances, make_real_challengetx_signed_by, make_real_cpmm_pool_signed_by,
    make_real_escrow_lock_signed_by, make_real_market_seed_signed_by, make_real_task_open_signed_by,
    make_real_verifytx_signed_by, make_real_worktx_signed_by, tb8_await_state_root_advance,
};
use turingosv4::runtime::verification_result::{
    write_to_cas as write_verification_result_to_cas, VerificationResult,
};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::bootstrap::default_pput_preseed_pairs;
use turingosv4::runtime::proposal_telemetry::{
    write_to_cas as write_proposal_telemetry_to_cas, ProposalTelemetry, TokenCounts,
};
use turingosv4::runtime::{build_chaintape_sequencer_with_initial_q, RuntimeChaintapeConfig};
use turingosv4::sdk::actor::boltzmann_select_parent_v2;
use turingosv4::state::price_index::compute_price_index;
use turingosv4::state::q_state::{AgentId, Hash, TaskId, TaskMarketState, TxId};
use turingosv4::state::sequencer::{Sequencer, SystemEmitCommand};
use turingosv4::state::typed_tx::{OutcomeSide, TypedTx};
use turingosv4::state::{BoltzmannMaskPolicy, NodeMarketEntry};

const SPONSOR_AGENT: &str = "Agent_user_0";
const PROVIDER_AGENT: &str = "Agent_user_1";
const MARKET_SEED_MICRO: i64 = 100_000;
const TASK_ESCROW_MICRO: i64 = 2_000;
const CHALLENGE_STAKE_MICRO: i64 = 500;
const MIN_SHORT_MICRO: i64 = 250;
const MAX_SHORT_MICRO: i64 = 8_000;
const MIN_STAKE_MICRO: i64 = 250;
const MAX_STAKE_MICRO: i64 = 20_000;
const BASE_WORK_STAKE: i64 = 1_000;
const VERIFIER_AGENT: &str = "Agent_lm_verifier";
const VERIFY_BOND_MICRO: i64 = 500;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Policy {
    Market,
    RandomBear,
    FixedBear,
    ShuffledPrice,
    NoPrice,
    Single,
    Parallel,
    Majority,
    BestFirst,
}

impl Policy {
    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "market" => Ok(Policy::Market),
            "random_bear" => Ok(Policy::RandomBear),
            "fixed_bear" => Ok(Policy::FixedBear),
            "shuffled_price" => Ok(Policy::ShuffledPrice),
            "no_price" => Ok(Policy::NoPrice),
            "single" => Ok(Policy::Single),
            "parallel" => Ok(Policy::Parallel),
            "majority" => Ok(Policy::Majority),
            "best_first" => Ok(Policy::BestFirst),
            _ => Err(format!("unknown policy `{s}`")),
        }
    }
    fn label(self) -> &'static str {
        match self {
            Policy::Market => "market",
            Policy::RandomBear => "random_bear",
            Policy::FixedBear => "fixed_bear",
            Policy::ShuffledPrice => "shuffled_price",
            Policy::NoPrice => "no_price",
            Policy::Single => "single",
            Policy::Parallel => "parallel",
            Policy::Majority => "majority",
            Policy::BestFirst => "best_first",
        }
    }
    /// Price-family policies emit a Bear ChallengeTx (short) per node; the
    /// non-market baselines are Bulls-only (no short, no price game).
    fn emits_challenges(self) -> bool {
        matches!(self, Policy::Market | Policy::RandomBear | Policy::FixedBear | Policy::ShuffledPrice | Policy::NoPrice)
    }
}

struct Args {
    runtime_repo: PathBuf,
    cas: PathBuf,
    run_id: String,
    out: PathBuf,
    proxy_url: String,
    model: String,
    bank: PathBuf,
    problem: String,
    mathlib_dir: Option<PathBuf>,
    policy: Policy,
    n_agents: usize,
    n_rounds: usize,
    seed: u64,
    continue_past_omega: bool,
}

#[derive(Debug, Clone, Serialize)]
struct AttemptNode {
    node_tx: String,
    task: String,
    by_agent: String,
    parent_tx: Option<String>,
    confidence_pct: u64,
    work_stake_micro: i64,
    price_yes_num: Option<u128>,
    price_yes_den: Option<u128>,
    verdict: String,
    is_verified: bool,
    body_preview: String,
    feedback: String,
    tokens: u64,
}

#[derive(Debug, Serialize)]
struct Manifest {
    schema_version: &'static str,
    run_id: String,
    policy: &'static str,
    model: String,
    problem: String,
    needs_mathlib: bool,
    n_agents: usize,
    n_rounds: usize,
    seed: u64,
    llm_calls: usize,
    bear_calls: usize,
    bear_tokens: u64,
    parse_fails: usize,
    verified_count: usize,
    failed_count: usize,
    distinct_price_ratios: usize,
    price_discovery: bool,
    omega_reached: bool,
    omega_node: Option<String>,
    time_to_first_proof_s: Option<f64>,
    golden_path: Vec<String>,
    golden_path_tokens: u64,
    total_tokens: u64,
    wall_clock_s: f64,
    pput: f64,
    final_state_root_hex: String,
    runtime_repo: String,
    cas: String,
    nodes: Vec<AttemptNode>,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut m: BTreeMap<String, String> = BTreeMap::new();
    let mut i = 0;
    while i < argv.len() {
        let k = &argv[i];
        if let Some(stripped) = k.strip_prefix("--") {
            let v = argv.get(i + 1).cloned().ok_or(format!("missing value after {k}"))?;
            m.insert(stripped.to_string(), v);
            i += 2;
        } else {
            return Err(format!("unexpected arg {k}"));
        }
    }
    let get = |k: &str| m.get(k).cloned();
    let runtime_repo: PathBuf = get("runtime-repo").ok_or("--runtime-repo required")?.into();
    Ok(Args {
        out: get("out").map(Into::into).unwrap_or_else(|| runtime_repo.join("lean_market_manifest.json")),
        runtime_repo,
        cas: get("cas").ok_or("--cas required")?.into(),
        run_id: get("run-id").ok_or("--run-id required")?,
        proxy_url: get("proxy-url").unwrap_or_else(|| "http://localhost:8123".into()),
        model: get("model").unwrap_or_else(|| "deepseek-chat".into()),
        bank: get("bank").map(Into::into).unwrap_or_else(|| "tests/fixtures/lean_theorems.jsonl".into()),
        problem: get("problem").ok_or("--problem <theorem id> required")?,
        mathlib_dir: get("mathlib-dir").map(Into::into),
        policy: Policy::parse(&get("policy").unwrap_or_else(|| "market".into()))?,
        n_agents: get("n-agents").and_then(|s| s.parse().ok()).unwrap_or(8),
        n_rounds: get("n-rounds").and_then(|s| s.parse().ok()).unwrap_or(6),
        seed: get("seed").and_then(|s| s.parse().ok()).unwrap_or(0xB01),
        continue_past_omega: get("continue-past-omega").map(|s| s == "true").unwrap_or(false),
    })
}

fn hash_hex(h: &Hash) -> String {
    h.0.iter().map(|b| format!("{b:02x}")).collect()
}

fn stake_from_confidence(confidence_pct: u64) -> i64 {
    let mult_num = (25 + 375 * confidence_pct.min(100) as i64 / 100).max(25);
    (BASE_WORK_STAKE.saturating_mul(mult_num) / 100).clamp(MIN_STAKE_MICRO, MAX_STAKE_MICRO)
}

fn extract_json_object(content: &str) -> Option<serde_json::Value> {
    let t = content.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
    if let Ok(v) = serde_json::from_str(t) {
        return Some(v);
    }
    let start = t.find('{')?;
    let end = t.rfind('}')?;
    serde_json::from_str(&t[start..=end]).ok()
}

/// A0: permute the price values among the node keys, so parent selection runs on a
/// randomized routing signal (same nodes, same compute, signal destroyed).
fn shuffle_prices(
    pi: &BTreeMap<TxId, NodeMarketEntry>,
    rng: &mut StdRng,
) -> BTreeMap<TxId, NodeMarketEntry> {
    let keys: Vec<TxId> = pi.keys().cloned().collect();
    let mut vals: Vec<NodeMarketEntry> = pi.values().cloned().collect();
    for i in (1..vals.len()).rev() {
        let j = rng.gen_range(0..=i);
        vals.swap(i, j);
    }
    keys.into_iter().zip(vals).collect()
}

/// Parent selection by policy. Returns the parent attempt node to refine (or None
/// for a fresh root attempt).
fn select_parent(
    policy: Policy,
    pi: &BTreeMap<TxId, NodeMarketEntry>,
    all_nodes: &[TxId],
    own_last: Option<&TxId>,
    node_conf: &BTreeMap<String, u64>,
    rng: &mut StdRng,
) -> Option<TxId> {
    match policy {
        Policy::Market | Policy::RandomBear | Policy::FixedBear => boltzmann_select_parent_v2(pi, &BTreeSet::new(), &BoltzmannMaskPolicy::default(), rng)
            .or_else(|| all_nodes.last().cloned()),
        Policy::ShuffledPrice => {
            let shuffled = shuffle_prices(pi, rng);
            boltzmann_select_parent_v2(&shuffled, &BTreeSet::new(), &BoltzmannMaskPolicy::default(), rng)
                .or_else(|| all_nodes.last().cloned())
        }
        Policy::NoPrice => {
            if all_nodes.is_empty() {
                None
            } else {
                Some(all_nodes[rng.gen_range(0..all_nodes.len())].clone())
            }
        }
        // Own-chain baselines (no shared routing): refine only this agent's last node.
        Policy::Single | Policy::Parallel | Policy::Majority => own_last.cloned(),
        // Greedy best-first: extend the highest-confidence node on the shared tape,
        // with NO price and NO Bear short — isolates the priced market from plain greed.
        Policy::BestFirst => all_nodes
            .iter()
            .max_by_key(|t| node_conf.get(&t.0).copied().unwrap_or(0))
            .cloned(),
    }
}

async fn submit_await(seq: &Sequencer, tx: TypedTx, pre: Hash, label: &str) -> Result<Hash, String> {
    seq.submit_agent_tx(tx).await.map_err(|e| format!("submit {label}: {e:?}"))?;
    tb8_await_state_root_advance(seq, pre, 5_000).await.map_err(|_| format!("{label} did not advance"))
}

fn put_proposal(cas_path: &PathBuf, run_id: &str, agent: &str, idx: u64, parent: Option<TxId>, body: &str, tokens: TokenCounts, lt: u64) -> Result<Cid, String> {
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    let tel = ProposalTelemetry::build_for_evaluator_append_with_parent(
        &mut cas, run_id, agent, idx, body.as_bytes(), "lm_proof", tokens, "lm-agent", lt, parent,
    ).map_err(|e| format!("ProposalTelemetry: {e}"))?;
    write_proposal_telemetry_to_cas(&mut cas, &tel, "lm-proposal-telemetry", lt + 1).map_err(|e| format!("write telemetry: {e}"))
}

fn put_counterexample(cas_path: &PathBuf, work_tx: &str, lt: u64) -> Result<Cid, String> {
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    let blob = serde_json::json!({"schema":"lm.counterexample.v1","target":work_tx});
    cas.put(serde_json::to_vec(&blob).unwrap().as_slice(), ObjectType::EvidenceCapsule, "lm-challenger", lt, Some("lm.counterexample.v1".into()))
        .map_err(|e| format!("put counterexample: {e}"))
}

fn put_proof_artifact(cas_path: &PathBuf, source: &str, lt: u64) -> Result<Cid, String> {
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    cas.put(source.as_bytes(), ObjectType::Generic, "lm-verifier", lt, Some("lm.proof_artifact.v1".into()))
        .map_err(|e| format!("put proof artifact: {e}"))
}

/// GCD for reducing price fractions so equal ratios (e.g. 4000/4000 == 250/250 == 1/1)
/// collapse — `distinct_price_ratios` must count distinct PRICES, not distinct stakes.
fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn build_prompt(theorem: &LeanTheorem, parent_body: Option<&str>, parent_feedback: Option<&str>) -> String {
    let mut p = String::new();
    p.push_str("You are proving a theorem in Lean 4 (Mathlib is available). Output ONLY a JSON object.\n\n");
    p.push_str("=== Target (prove the goal after `:= by`) ===\n");
    p.push_str(&theorem.preamble);
    p.push('\n');
    if let (Some(body), Some(fb)) = (parent_body, parent_feedback) {
        p.push_str("\n=== A previous attempt FAILED — fix it ===\n--- attempt body ---\n");
        p.push_str(body);
        p.push_str("\n--- Lean error ---\n");
        p.push_str(fb);
        p.push('\n');
    }
    p.push_str(
        "\nReturn EXACTLY: {\"proof_body\":\"<the Lean tactic block AFTER `:= by`, no theorem signature, no imports>\",\"confidence\":0.0-1.0}\n",
    );
    p
}

/// Informed Bear short (P0-E): an independent skeptic LLM estimates P(this proof does NOT
/// compile); the short stake scales with that doubt, so weak proofs get a big short (low
/// price_yes) and strong ones a small short (high price_yes) — the price-discovery signal
/// the market routes on. Without it, every Long pins to max stake (agents are ~100%
/// confident) and every price is identical, making MARKET and A0 indistinguishable.
/// Money math is integer (doubt → integer percent → integer stake). Returns
/// (short_micro, tokens). Falls back to a flat short on LLM/parse error.
async fn bear_doubt_short(
    llm: &ResilientLLMClient,
    model: &str,
    theorem: &LeanTheorem,
    body: &str,
) -> (i64, u64) {
    let prompt = format!(
        "You are a SKEPTIC in a proof market. A prover submitted the Lean 4 proof body below \
         for the goal. Estimate the probability it does NOT compile under the Lean kernel \
         (0.0 = certainly compiles, 1.0 = certainly fails). Judge ONLY from the text; be \
         calibrated (most terse first attempts fail). Output ONLY JSON.\n\n\
         === Goal ===\n{}\n\n=== Proof body ===\n{}\n\nReturn EXACTLY: {{\"doubt\":0.0-1.0}}",
        theorem.preamble, body
    );
    match llm
        .generate(&GenerateRequest {
            model: model.into(),
            messages: vec![Message { role: "user".into(), content: prompt }],
            temperature: Some(0.3),
            max_tokens: Some(60),
        })
        .await
    {
        Ok(r) => {
            let doubt = extract_json_object(&r.content)
                .and_then(|v| v.get("doubt").and_then(|x| x.as_f64()))
                .unwrap_or(0.5)
                .clamp(0.0, 1.0);
            // probability → integer percent (not a money op); stake math stays integer.
            let doubt_pct = (doubt * 100.0) as i64;
            let short = MIN_SHORT_MICRO + (MAX_SHORT_MICRO - MIN_SHORT_MICRO) * doubt_pct / 100;
            (short, (r.prompt_tokens + r.completion_tokens) as u64)
        }
        Err(_) => (CHALLENGE_STAKE_MICRO, 0),
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("lean_market_agent: {e}");
            return ExitCode::from(2);
        }
    };
    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("lean_market_agent: {e}");
            ExitCode::from(1)
        }
    }
}

async fn run(args: Args) -> Result<(), String> {
    let t0 = Instant::now();

    // ── Problem + LeanJudge ──────────────────────────────────────────
    let bank = load_bank(&args.bank)?;
    let theorem = bank
        .iter()
        .find(|t| t.id == args.problem)
        .ok_or_else(|| format!("problem `{}` not in bank {}", args.problem, args.bank.display()))?
        .clone();
    let lean_bin = default_lean_bin();
    let mathlib_lp = if theorem.needs_mathlib {
        let dir = args.mathlib_dir.clone().ok_or("theorem needs Mathlib but --mathlib-dir not given")?;
        Some(mathlib_lean_path(&dir, &default_lake_bin()).ok_or("could not resolve Mathlib LEAN_PATH (lake env failed)")?)
    } else {
        None
    };
    let judge = theorem.judge(lean_bin, mathlib_lp.as_deref());

    let n_agents = if args.policy == Policy::Single { 1 } else { args.n_agents };
    let market_task = format!("lm-market-{}", args.run_id);
    let agents: Vec<String> = (0..n_agents).map(|i| format!("Agent_{i}")).collect();
    let challengers: Vec<String> = (0..n_agents).map(|i| format!("Chal_{i}")).collect();

    // ── Genesis + keypairs ───────────────────────────────────────────
    let mut balances = default_pput_preseed_pairs();
    for extra in [SPONSOR_AGENT, PROVIDER_AGENT, VERIFIER_AGENT] {
        if !balances.iter().any(|(a, _)| a.0 == extra) {
            balances.push((AgentId(extra.into()), MicroCoin::from_micro_units(5_000_000)));
        }
    }
    for a in agents.iter().chain(challengers.iter()) {
        if !balances.iter().any(|(x, _)| &x.0 == a) {
            balances.push((AgentId(a.clone()), MicroCoin::from_micro_units(5_000_000)));
        }
    }
    let initial_q = genesis_with_balances(&balances);
    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: args.runtime_repo.clone(),
        cas_path: args.cas.clone(),
        run_id: args.run_id.clone(),
        queue_capacity: 64,
        resume_existing_chain: false,
    };
    let bundle = build_chaintape_sequencer_with_initial_q(&cfg, initial_q).map_err(|e| format!("boot: {e}"))?;
    let seq = bundle.sequencer.clone();
    let mut kp = AgentKeypairRegistry::open(&cfg.runtime_repo_path).map_err(|e| format!("{e}"))?;
    let mut all: Vec<&str> = vec![SPONSOR_AGENT, PROVIDER_AGENT, VERIFIER_AGENT];
    all.extend(agents.iter().map(|s| s.as_str()));
    all.extend(challengers.iter().map(|s| s.as_str()));
    for id in &all {
        kp.get_or_create(&AgentId(id.to_string())).map_err(|e| format!("keypair {id}: {e}"))?;
    }
    seq.set_agent_pubkeys(std::sync::Arc::new(kp.manifest())).map_err(|_| "pubkeys set".to_string())?;

    // ── Market task scaffold ─────────────────────────────────────────
    let mut root = seq.q_snapshot().map_err(|e| format!("{e:?}"))?.state_root_t;
    let mut lt = 10u64;
    root = submit_await(&seq, make_real_task_open_signed_by(&mut kp, &market_task, SPONSOR_AGENT, root, "lm", lt).map_err(|e| format!("TaskOpen: {e}"))?, root, "TaskOpen").await?;
    lt += 1;
    root = submit_await(&seq, make_real_market_seed_signed_by(&mut kp, root, &market_task, PROVIDER_AGENT, MARKET_SEED_MICRO, "lm", lt).map_err(|e| format!("Seed: {e}"))?, root, "MarketSeed").await?;
    lt += 1;
    root = submit_await(&seq, make_real_cpmm_pool_signed_by(&mut kp, root, &market_task, PROVIDER_AGENT, MARKET_SEED_MICRO as u128, "lm").map_err(|e| format!("Pool: {e}"))?, root, "CpmmPool").await?;
    lt += 1;

    let llm = ResilientLLMClient::new(&args.proxy_url, 180, 3);
    let sys = Message {
        role: "system".into(),
        content: "You are a Lean 4 theorem-proving agent in a proof-search market. Return ONLY a JSON object, no markdown.".into(),
    };

    let mut nodes: Vec<AttemptNode> = Vec::new();
    let mut node_tx_ids: Vec<TxId> = Vec::new();
    let mut node_body: BTreeMap<String, String> = BTreeMap::new();
    let mut node_feedback: BTreeMap<String, String> = BTreeMap::new();
    let mut own_last: BTreeMap<String, TxId> = BTreeMap::new();
    let mut node_conf: BTreeMap<String, u64> = BTreeMap::new();
    let mut verified_agents: BTreeSet<String> = BTreeSet::new();
    let majority_threshold = agents.len() / 2 + 1;
    let (mut llm_calls, mut parse_fails, mut verified_count, mut failed_count) = (0usize, 0usize, 0usize, 0usize);
    let (mut bear_calls, mut bear_tokens_total) = (0usize, 0u64);
    let mut omega_node: Option<String> = None;
    let mut time_to_first_proof_s: Option<f64> = None;
    let mut step_idx = 0u64;

    'outer: for round in 0..args.n_rounds {
        for ai in 0..agents.len() {
            let agent = agents[ai].clone();
            let q = seq.q_snapshot().map_err(|e| format!("{e:?}"))?;
            root = q.state_root_t;
            let pi = compute_price_index(&q.economic_state_t);

            // Parent selection (policy-governed).
            let mut rng = StdRng::seed_from_u64(args.seed + round as u64 * 131 + ai as u64);
            let parent_tx = select_parent(args.policy, &pi, &node_tx_ids, own_last.get(&agent), &node_conf, &mut rng);
            let (parent_body, parent_feedback) = match &parent_tx {
                Some(t) => (node_body.get(&t.0).cloned(), node_feedback.get(&t.0).cloned()),
                None => (None, None),
            };

            let prompt = build_prompt(&theorem, parent_body.as_deref(), parent_feedback.as_deref());
            let resp = match llm
                .generate(&GenerateRequest {
                    model: args.model.clone(),
                    messages: vec![sys.clone(), Message { role: "user".into(), content: prompt }],
                    temperature: Some(0.7),
                    max_tokens: Some(900),
                })
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("lm llm_err {agent}: {e:?}");
                    continue;
                }
            };
            llm_calls += 1;
            let tokens = TokenCounts {
                prompt_tokens: resp.prompt_tokens as u64,
                completion_tokens: resp.completion_tokens as u64,
                tool_tokens: 0,
            };
            let v = match extract_json_object(&resp.content) {
                Some(v) => v,
                None => {
                    parse_fails += 1;
                    continue;
                }
            };
            let body = v.get("proof_body").and_then(|x| x.as_str()).unwrap_or("").to_string();
            if body.trim().is_empty() {
                parse_fails += 1;
                continue;
            }
            let confidence_pct = (v.get("confidence").and_then(|x| x.as_f64()).unwrap_or(0.6).clamp(0.0, 1.0) * 100.0) as u64;

            // ── Real Lean kernel verdict ─────────────────────────────
            let outcome = judge.verify(&body);
            let is_verified = outcome.is_verified();
            if is_verified {
                verified_count += 1;
            } else {
                failed_count += 1;
            }

            // ── Per-task node (EVERY attempt — Verified or Failed) ────
            let work_stake = stake_from_confidence(confidence_pct);
            let node_task = format!("lm-node{step_idx}-{}", args.run_id);
            root = submit_await(&seq, make_real_task_open_signed_by(&mut kp, &node_task, SPONSOR_AGENT, root, "lm", lt).map_err(|e| format!("TaskOpen node: {e}"))?, root, "TaskOpen(node)").await?;
            lt += 1;
            root = submit_await(&seq, make_real_escrow_lock_signed_by(&mut kp, &node_task, SPONSOR_AGENT, TASK_ESCROW_MICRO, root, "lm", lt).map_err(|e| format!("Escrow node: {e}"))?, root, "Escrow(node)").await?;
            lt += 1;
            let pcid = put_proposal(&args.cas, &args.run_id, &agent, step_idx, parent_tx.clone(), &body, tokens, lt)?;
            lt += 2;
            let work = make_real_worktx_signed_by(&mut kp, &node_task, &agent, root, work_stake, "lm", pcid, true, lt).map_err(|e| format!("WorkTx: {e}"))?;
            let work_tx_id = match &work {
                TypedTx::Work(w) => w.tx_id.0.clone(),
                _ => return Err("not WorkTx".into()),
            };
            root = submit_await(&seq, work, root, "WorkTx").await?;
            lt += 1;
            node_tx_ids.push(TxId(work_tx_id.clone()));
            own_last.insert(agent.clone(), TxId(work_tx_id.clone()));
            node_body.insert(work_tx_id.clone(), body.clone());
            node_feedback.insert(work_tx_id.clone(), outcome.feedback.clone());
            node_conf.insert(work_tx_id.clone(), confidence_pct);

            // Short challenge → price_yes (price-family policies only; non-market
            // baselines are Bulls-only). Non-fatal.
            if args.policy.emits_challenges() {
                // Bear short by policy: informed (skeptic-LLM doubt) for market/shuffled/no_price;
                // random U(0,1) with NO skeptic call (M1); or fixed constant (M2). M1/M2 isolate
                // whether the *informed* price signal (vs noise / vs a constant) does the work.
                let (short_micro, bear_tok) = match args.policy {
                    Policy::RandomBear => {
                        let doubt_pct = rng.gen_range(0..=100) as i64;
                        (MIN_SHORT_MICRO + (MAX_SHORT_MICRO - MIN_SHORT_MICRO) * doubt_pct / 100, 0u64)
                    }
                    Policy::FixedBear => (CHALLENGE_STAKE_MICRO, 0u64),
                    _ => bear_doubt_short(&llm, &args.model, &theorem, &body).await,
                };
                bear_calls += 1;
                bear_tokens_total += bear_tok;
                let challenger = challengers[ai % challengers.len()].clone();
                if let Ok(ce) = put_counterexample(&args.cas, &work_tx_id, lt) {
                    lt += 1;
                    match make_real_challengetx_signed_by(&mut kp, root, TxId(work_tx_id.clone()), &challenger, short_micro, ce, &format!("lm{step_idx}"), lt) {
                        Ok(chal) => match submit_await(&seq, chal, root, "ChallengeTx").await {
                            Ok(r) => {
                                root = r;
                                lt += 1;
                            }
                            Err(e) => eprintln!("lm challenge skip node{step_idx}: {e}"),
                        },
                        Err(e) => eprintln!("lm challenge build skip: {e}"),
                    }
                }
            }

            // Chain-record the Lean verdict so the OMEGA is reconstructable from tape
            // (not just in-memory): a VerificationResult CAS object + a Confirm/Doubt
            // VerifyTx targeting the WorkTx. Confirm <=> kernel-Verified. Unique suffix
            // per node (avoids verifytx-id collision when the verifier is reused).
            let assembled = judge.assemble(&body);
            if let Ok(artifact_cid) = put_proof_artifact(&args.cas, &assembled, lt) {
                lt += 1;
                let vr = VerificationResult::from_lean_run(
                    TxId(work_tx_id.clone()),
                    AgentId(VERIFIER_AGENT.into()),
                    outcome.exit_code,
                    artifact_cid,
                    &format!("lm-node{step_idx}.lean"),
                    assembled.as_bytes(),
                );
                if let Ok(mut cas) = CasStore::open(&args.cas) {
                    let _ = write_verification_result_to_cas(&mut cas, &vr, "lm-verifier", lt);
                }
                lt += 1;
                match make_real_verifytx_signed_by(&mut kp, root, TxId(work_tx_id.clone()), VERIFIER_AGENT, VERIFY_BOND_MICRO, &format!("lmv{step_idx}"), is_verified, lt) {
                    Ok(vtx) => match submit_await(&seq, vtx, root, "VerifyTx").await {
                        Ok(r) => {
                            root = r;
                            lt += 1;
                        }
                        Err(e) => eprintln!("lm verify skip node{step_idx}: {e}"),
                    },
                    Err(e) => eprintln!("lm verify build skip: {e}"),
                }
            }

            let price = compute_price_index(&seq.q_snapshot().map_err(|e| format!("{e:?}"))?.economic_state_t);
            let pe = price.get(&TxId(work_tx_id.clone()));
            nodes.push(AttemptNode {
                node_tx: work_tx_id.clone(),
                task: node_task,
                by_agent: agent.clone(),
                parent_tx: parent_tx.map(|t| t.0),
                confidence_pct,
                work_stake_micro: work_stake,
                price_yes_num: pe.and_then(|e| e.price_yes.as_ref().map(|p| p.numerator)),
                price_yes_den: pe.and_then(|e| e.price_yes.as_ref().map(|p| p.denominator)),
                verdict: format!("{:?}", outcome.verdict_kind),
                is_verified,
                body_preview: body.chars().take(120).collect(),
                feedback: outcome.feedback.chars().take(160).collect(),
                tokens: tokens.prompt_tokens + tokens.completion_tokens,
            });
            step_idx += 1;
            if is_verified {
                verified_agents.insert(agent.clone());
                // Majority/self-consistency: OMEGA only once a strict majority of
                // DISTINCT agents have each produced a Verified proof. All other
                // policies settle on the first Verified node.
                let omega_now =
                    args.policy != Policy::Majority || verified_agents.len() >= majority_threshold;
                if omega_now && omega_node.is_none() {
                    omega_node = Some(work_tx_id.clone());
                    time_to_first_proof_s = Some(t0.elapsed().as_secs_f64());
                }
                if omega_node.is_some() && !args.continue_past_omega {
                    break 'outer;
                }
            }
        }
    }

    // ── Settlement ───────────────────────────────────────────────────
    let outcome_side = if omega_node.is_some() { OutcomeSide::Yes } else { OutcomeSide::No };
    if seq.emit_system_tx(SystemEmitCommand::EventResolve { task_id: TaskId(market_task.clone()), outcome: outcome_side }).await.is_ok() {
        let _ = tb8_await_state_root_advance(&seq, root, 5_000).await;
    }
    let _ = seq.q_snapshot().map_err(|e| format!("{e:?}"))?.economic_state_t.task_markets_t.0.get(&TaskId(market_task.clone())).map(|m| m.state != TaskMarketState::Open);

    let seq_handle = seq.clone();
    bundle.shutdown().await.map_err(|e| format!("shutdown: {e}"))?;
    let final_root = seq_handle.q_snapshot().map_err(|e| format!("{e:?}"))?.state_root_t;

    // ── Golden path (ancestor chain of OMEGA) + PPUT ─────────────────
    let parent_of: BTreeMap<String, Option<String>> = nodes.iter().map(|n| (n.node_tx.clone(), n.parent_tx.clone())).collect();
    let tokens_of: BTreeMap<String, u64> = nodes.iter().map(|n| (n.node_tx.clone(), n.tokens)).collect();
    let mut golden_path: Vec<String> = Vec::new();
    let mut golden_path_tokens = 0u64;
    if let Some(o) = &omega_node {
        let mut cur = Some(o.clone());
        while let Some(c) = cur {
            golden_path.push(c.clone());
            golden_path_tokens += tokens_of.get(&c).copied().unwrap_or(0);
            cur = parent_of.get(&c).cloned().flatten();
        }
        golden_path.reverse();
    }
    let total_tokens: u64 = nodes.iter().map(|n| n.tokens).sum::<u64>() + bear_tokens_total;
    let wall_clock_s = t0.elapsed().as_secs_f64();
    let pput = if omega_node.is_none() || wall_clock_s <= 0.0 { 0.0 } else { golden_path_tokens as f64 / wall_clock_s };

    let mut ratios: BTreeSet<(u128, u128)> = BTreeSet::new();
    for n in &nodes {
        if let (Some(a), Some(b)) = (n.price_yes_num, n.price_yes_den) {
            let g = gcd_u128(a, b).max(1);
            ratios.insert((a / g, b / g));
        }
    }
    let distinct_price_ratios = ratios.len();

    let manifest = Manifest {
        schema_version: "turingosv4.lean_market.v1",
        run_id: args.run_id.clone(),
        policy: args.policy.label(),
        model: args.model.clone(),
        problem: args.problem.clone(),
        needs_mathlib: theorem.needs_mathlib,
        n_agents,
        n_rounds: args.n_rounds,
        seed: args.seed,
        llm_calls,
        bear_calls,
        bear_tokens: bear_tokens_total,
        parse_fails,
        verified_count,
        failed_count,
        distinct_price_ratios,
        price_discovery: distinct_price_ratios > 1,
        omega_reached: omega_node.is_some(),
        omega_node: omega_node.clone(),
        time_to_first_proof_s,
        golden_path,
        golden_path_tokens,
        total_tokens,
        wall_clock_s,
        pput,
        final_state_root_hex: hash_hex(&final_root),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas: args.cas.display().to_string(),
        nodes,
    };
    if let Some(p) = args.out.parent() {
        std::fs::create_dir_all(p).ok();
    }
    std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).map_err(|e| format!("ser: {e}"))?).map_err(|e| format!("write: {e}"))?;
    println!(
        "lean_market[{}] problem={} agents={} rounds={} llm={} bear={} parse_fail={} verified={} failed={} nodes={} distinct_prices={} omega={} ttfp={:?}s gp_tokens={} total_tokens={} wall={:.1}s pput={:.2} manifest={}",
        args.policy.label(), args.problem, n_agents, args.n_rounds, llm_calls, bear_calls, parse_fails, verified_count, failed_count,
        manifest.nodes.len(), distinct_price_ratios, manifest.omega_reached, time_to_first_proof_s,
        golden_path_tokens, total_tokens, wall_clock_s, pput, args.out.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn own_chain_policies_refine_own_last_not_others() {
        let pi = BTreeMap::new();
        let conf = BTreeMap::new();
        let nodes = vec![TxId("n_other".into()), TxId("n_mine".into())];
        let own = TxId("n_mine".into());
        let mut rng = StdRng::seed_from_u64(7);
        for p in [Policy::Single, Policy::Parallel, Policy::Majority] {
            let got = select_parent(p, &pi, &nodes, Some(&own), &conf, &mut rng);
            assert_eq!(got, Some(TxId("n_mine".into())), "{p:?} must refine own_last");
        }
    }

    #[test]
    fn parallel_without_own_last_starts_fresh_root() {
        let pi = BTreeMap::new();
        let conf = BTreeMap::new();
        let nodes = vec![TxId("someone_elses".into())];
        let mut rng = StdRng::seed_from_u64(7);
        // No shared tape: a parallel agent never adopts another agent's node.
        assert_eq!(select_parent(Policy::Parallel, &pi, &nodes, None, &conf, &mut rng), None);
    }

    #[test]
    fn best_first_extends_highest_confidence_node() {
        let pi = BTreeMap::new();
        let mut conf = BTreeMap::new();
        conf.insert("lo".to_string(), 30);
        conf.insert("hi".to_string(), 95);
        conf.insert("mid".to_string(), 60);
        let nodes = vec![TxId("lo".into()), TxId("hi".into()), TxId("mid".into())];
        let mut rng = StdRng::seed_from_u64(7);
        assert_eq!(
            select_parent(Policy::BestFirst, &pi, &nodes, None, &conf, &mut rng),
            Some(TxId("hi".into()))
        );
    }

    #[test]
    fn only_price_family_emits_bear_shorts() {
        for p in [Policy::Market, Policy::RandomBear, Policy::FixedBear, Policy::ShuffledPrice, Policy::NoPrice] {
            assert!(p.emits_challenges(), "{p:?} is price-family");
        }
        for p in [Policy::Single, Policy::Parallel, Policy::Majority, Policy::BestFirst] {
            assert!(!p.emits_challenges(), "{p:?} is Bulls-only");
        }
    }

    #[test]
    fn policy_parse_roundtrips_all_arms() {
        for s in ["market", "random_bear", "fixed_bear", "shuffled_price", "no_price", "single", "parallel", "majority", "best_first"] {
            assert_eq!(Policy::parse(s).unwrap().label(), s);
        }
        assert!(Policy::parse("bogus").is_err());
    }
}
