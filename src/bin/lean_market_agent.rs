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
// REAL librarian (src/runtime/librarian_broadcast.rs): CAS-derived, role-scoped, shielded
// collective digest of prior attempts. Fed by the LeanResult sidecar written below; the
// previous experiment-local `librarian_digest` lookalike is removed.
use turingosv4::runtime::librarian_broadcast::{
    build_librarian_digest, derive_current_run_cas_root, project_role_notifications,
    select_librarian_events, validate_librarian_source_scope, LibrarianSourceScope,
};
use turingosv4::runtime::attempt_telemetry::{write_lean_result_to_cas, LeanResult};
use turingosv4::runtime::real5_roles::AgentRole;
use turingosv4::sdk::actor::boltzmann_softmax_select_parent;
use turingosv4::state::price_index::compute_price_index;
use turingosv4::state::q_state::{AgentId, Hash, TaskId, TaskMarketState, TxId};
use turingosv4::state::sequencer::{Sequencer, SystemEmitCommand};
use turingosv4::state::typed_tx::{OutcomeSide, TypedTx};
use turingosv4::state::NodeMarketEntry;

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
    Autonomous,
    RandomBear,
    FixedBear,
    ShuffledPrice,
    NoPrice,
    Single,
    Parallel,
    Majority,
    BestFirst,
    SkepticRerank,
}

impl Policy {
    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "market" => Ok(Policy::Market),
            "autonomous" => Ok(Policy::Autonomous),
            "random_bear" => Ok(Policy::RandomBear),
            "fixed_bear" => Ok(Policy::FixedBear),
            "shuffled_price" => Ok(Policy::ShuffledPrice),
            "no_price" => Ok(Policy::NoPrice),
            "single" => Ok(Policy::Single),
            "parallel" => Ok(Policy::Parallel),
            "majority" => Ok(Policy::Majority),
            "best_first" => Ok(Policy::BestFirst),
            "skeptic_rerank" => Ok(Policy::SkepticRerank),
            _ => Err(format!("unknown policy `{s}`")),
        }
    }
    fn label(self) -> &'static str {
        match self {
            Policy::Market => "market",
            Policy::Autonomous => "autonomous",
            Policy::RandomBear => "random_bear",
            Policy::FixedBear => "fixed_bear",
            Policy::ShuffledPrice => "shuffled_price",
            Policy::NoPrice => "no_price",
            Policy::Single => "single",
            Policy::Parallel => "parallel",
            Policy::Majority => "majority",
            Policy::BestFirst => "best_first",
            Policy::SkepticRerank => "skeptic_rerank",
        }
    }
    /// Price-family policies emit a Bear ChallengeTx (short) per node; the
    /// non-market baselines are Bulls-only (no short, no price game).
    fn emits_challenges(self) -> bool {
        matches!(self, Policy::Market | Policy::Autonomous | Policy::RandomBear | Policy::FixedBear | Policy::ShuffledPrice | Policy::NoPrice)
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
    boltzmann_temp: f64,
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
    // Route telemetry (Class 1, autonomous arm only; telemetry-only, no behavior change). Splits
    // every resolve_parent_index outcome so the run can prove WHICH routing actually fired:
    //   deliberate_fresh_root — model returned the -1 sentinel (intentional new branch)
    //   valid_index_hit       — model named a real in-range node (genuine non-local routing)
    //   hallucinated_out_of_range — model named an out-of-range index, fail-open bailed to a root
    // Without this split a loss cannot be attributed to the routing PRINCIPLE vs hallucination/bail.
    route_deliberate_fresh_root: usize,
    route_valid_index_hit: usize,
    route_hallucinated_out_of_range: usize,
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
        boltzmann_temp: get("boltzmann-temp").and_then(|s| s.parse().ok()).unwrap_or(0.15),
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
    node_doubt: &BTreeMap<String, i64>,
    temp: f64,
    rng: &mut StdRng,
) -> Option<TxId> {
    match policy {
        // AUTONOMOUS: the LLM picks its own parent index INSIDE the proposal call, so the
        // pre-call selector is a no-op (None). The real parent is parsed from the model's
        // {parent_node} field and validated against node_tx_ids after the LLM returns.
        Policy::Autonomous => None,
        // TRUE Boltzmann softmax (Art. II.2.1): distribute attention across promising nodes
        // (incl. early ones → non-local re-expansion / new branches), NOT argmax-collapse.
        Policy::Market | Policy::RandomBear | Policy::FixedBear => boltzmann_softmax_select_parent(pi, &BTreeSet::new(), temp, rng)
            .or_else(|| all_nodes.last().cloned()),
        Policy::ShuffledPrice => {
            let shuffled = shuffle_prices(pi, rng);
            boltzmann_softmax_select_parent(&shuffled, &BTreeSet::new(), temp, rng)
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
        // B6 skeptic-rerank: extend the LOWEST-doubt node per the SAME skeptic (critic-matched
        // budget); shared tape, NO price, NO short — isolates the critic heuristic from the market.
        Policy::SkepticRerank => all_nodes
            .iter()
            .min_by_key(|t| node_doubt.get(&t.0).copied().unwrap_or(i64::MAX))
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

/// Shield a raw Lean error down to an opaque CLASS (constitution: no raw stderr / private diagnostics
/// in ordinary agent prompts; the librarian source-scope rule). Used in the collective digest.
fn classify_lean_error(fb: &str) -> &'static str {
    let f = fb.to_lowercase();
    if f.contains("unsolved goals") { "unsolved_goals" }
    else if f.contains("type mismatch") { "type_mismatch" }
    else if f.contains("unknown identifier") || f.contains("unknown constant") { "unknown_identifier" }
    else if f.contains("rewrite") && f.contains("fail") { "rewrite_failed" }
    else if f.contains("nlinarith") || f.contains("linarith") || f.contains("positivity") { "arith_failed" }
    else if f.contains("unexpected") || f.contains("syntax") || f.contains("expected") { "syntax_error" }
    else if f.contains("no progress") { "no_progress" }
    else if f.trim().is_empty() { "no_feedback" }
    else { "other_error" }
}

/// REAL librarian collective digest (src/runtime/librarian_broadcast.rs — the full
/// constitutional mechanism, NOT a lookalike). Reads the typed LeanResult sidecars this
/// run already wrote into CAS, builds a deterministic shielded `LibrarianDigest`, and
/// projects the Solver crop into a bounded "=== Librarian Notices ===" prompt block.
/// Everything that transits is an opaque error CLASS / pre-written public_summary —
/// `assert_no_forbidden_broadcast_material` runs on every event + cluster + rendered line.
///
/// Returns "" (no librarian section) when the source scope is invalid, no typed evidence
/// exists yet, or the Solver crop is empty (e.g. <2 of any one error class → no cluster).
/// Read-only: opens a FRESH `CasStore` (open-per-read, mirrors the bin's put helpers) and
/// never mutates the run.
fn real_librarian_solver_notice(cas_path: &PathBuf, current_head_t: u64, problem: &str) -> String {
    let cas = match CasStore::open(cas_path) {
        Ok(c) => c,
        Err(_) => return String::new(),
    };
    let scope = LibrarianSourceScope {
        current_run_cas_root: derive_current_run_cas_root(&cas), // run-local surrogate, NOT a global pointer
        prior_capsule_cids: vec![],
        max_prior_batches: 0,
        task_tags: vec![problem.to_string()], // problem id; fail-closed if it contains latest/pointer/.txt
    };
    if validate_librarian_source_scope(&scope, &cas).is_err() {
        return String::new();
    }
    let events = match select_librarian_events(&cas) {
        Ok(e) => e,
        Err(_) => return String::new(), // fail-closed selector errored (e.g. unknown schema) → no section
    };
    if events.is_empty() {
        return String::new();
    }
    let digest = match build_librarian_digest(scope, current_head_t, events) {
        Ok(d) => d,
        Err(_) => return String::new(),
    };
    let view = match project_role_notifications(&digest, AgentRole::Solver, 10) {
        Ok(v) => v,
        Err(_) => return String::new(),
    };
    // Empty-crop sentinel: don't inject a section that says nothing actionable.
    if view.rendered_notice.contains("No librarian notices for this role at current scope") {
        return String::new();
    }
    format!("\n{}", view.rendered_notice)
}

fn build_prompt(theorem: &LeanTheorem, parent_body: Option<&str>, parent_feedback: Option<&str>, librarian: &str) -> String {
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
    if !librarian.is_empty() {
        p.push_str(librarian);
    }
    p.push_str(
        "\nReturn EXACTLY: {\"proof_body\":\"<the Lean tactic block AFTER `:= by`, no theorem signature, no imports>\",\"confidence\":0.0-1.0}\n",
    );
    p
}

/// AUTONOMOUS landscape prompt: shows the model the FULL frontier of prior attempts (every
/// node, including early ones) and lets it FREELY pick which to extend — by index — or start
/// fresh (`-1`). Inverts the market control flow (parent chosen by the LLM, not pre-selected).
/// SHIELDING: each node is shown as (index, price_yes ratio, confidence, error-CLASS via
/// `classify_lean_error`, body-snippet). EQUAL-RIGOR DEPTH (fairness fix): for the top-k nodes
/// by price the row ALSO carries that node's `node_feedback` — which is ALREADY the bounded
/// shielded `error:` line produced by `shield_lean_diagnostic` (FEEDBACK_MAX=240, lean_judge.rs)
/// and stored on tape — the SAME text the market arm injects via `build_prompt`'s
/// `parent_feedback` for its ONE pre-selected parent. This adds NO new information channel and NO
/// raw stderr: it is the identical already-shielded diagnostic, plumbed breadth-wise so the
/// autonomous arm repairs WITH detail at equal 1-call budget instead of BLIND-to-detail. The SAME
/// shielded collective librarian digest is injected (requirement A for this arm). ONE proposal
/// call per turn (identical budget to market).
const AUTONOMOUS_FEEDBACK_TOPK: usize = 6;
fn build_autonomous_prompt(
    theorem: &LeanTheorem,
    node_tx_ids: &[TxId],
    node_body: &BTreeMap<String, String>,
    node_feedback: &BTreeMap<String, String>,
    node_conf: &BTreeMap<String, u64>,
    pi: &BTreeMap<TxId, NodeMarketEntry>,
    librarian: &str,
) -> String {
    let mut p = String::new();
    p.push_str(
        "You are proving a theorem in Lean 4 (Mathlib is available) inside a proof-search market. \
         You see the FULL landscape of prior attempts (the search frontier). FREELY CHOOSE which \
         attempt to extend (give its index) OR start fresh (index -1). Prefer a promising but \
         unfinished line; you MAY branch from an EARLY attempt if later ones are dead ends. \
         Output ONLY a JSON object.\n\n",
    );
    p.push_str("=== Target (prove the goal after `:= by`) ===\n");
    p.push_str(&theorem.preamble);
    p.push('\n');
    if node_tx_ids.is_empty() {
        p.push_str("\n=== Landscape: EMPTY (you are the first attempt; use parent_node = -1) ===\n");
    } else {
        // The top-k nodes by price_yes get the SAME shielded `error:` diagnostic the market arm
        // sees for its single chosen parent (depth parity); the rest carry the coarse class only.
        let price_of = |tx: &TxId| -> f64 {
            pi.get(tx)
                .and_then(|e| e.price_yes.as_ref())
                .map(|r| (r.numerator as f64) / (r.denominator.max(1) as f64))
                .unwrap_or(0.0)
        };
        let mut ranked: Vec<usize> = (0..node_tx_ids.len()).collect();
        ranked.sort_by(|&a, &b| {
            price_of(&node_tx_ids[b])
                .partial_cmp(&price_of(&node_tx_ids[a]))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let detail_set: BTreeSet<usize> = ranked.into_iter().take(AUTONOMOUS_FEEDBACK_TOPK).collect();
        p.push_str("\n=== Landscape — all prior attempts (index : price_yes : confidence : error-class : body [FULL for top-priced nodes, else snippet] [: shielded Lean error for top-priced nodes]) ===\n");
        for (idx, tx) in node_tx_ids.iter().enumerate() {
            let body = node_body.get(&tx.0).map(|b| b.trim().replace('\n', " ")).unwrap_or_default();
            let fb = node_feedback.get(&tx.0);
            let class = fb.map(|f| classify_lean_error(f)).unwrap_or("pending");
            let conf = node_conf.get(&tx.0).copied().unwrap_or(0);
            let (pn, pd) = pi
                .get(tx)
                .and_then(|e| e.price_yes.as_ref())
                .map(|r| (r.numerator, r.denominator))
                .unwrap_or((0, 0));
            // BODY depth-parity (§17 rigged-arm fix): the top-k price nodes carry the FULL node_body
            // — the SAME untruncated text build_prompt feeds the market arm for its single chosen
            // parent (`p.push_str(body)`). The rest carry the coarse 110-char snippet. This matches
            // the breadth the FEEDBACK channel already uses (detail_set top-k get the full shielded
            // error, rest get class only); it adds NO new information channel and NO second call —
            // node_body already holds the full body on-tape — it only stops strawmanning a free
            // chooser that previously could not read the line it chose to extend. ONE call, same budget.
            let body_shown: String = if detail_set.contains(&idx) {
                body.clone()
            } else {
                body.chars().take(110).collect()
            };
            p.push_str(&format!("[{idx}] price={pn}/{pd} conf={conf}% class={class} :: `{body_shown}`"));
            // Depth-parity: the chosen-parent-grade shielded diagnostic (already FEEDBACK_MAX=240,
            // already error:-line only) for the top-priced nodes — the same text build_prompt feeds.
            if detail_set.contains(&idx) {
                if let Some(diag) = fb.filter(|d| !d.trim().is_empty()) {
                    let diag1 = diag.replace('\n', " ");
                    p.push_str(&format!("\n      lean-error: {diag1}"));
                }
            }
            p.push('\n');
        }
    }
    if !librarian.is_empty() {
        p.push_str(librarian); // (A) shielded collective-failure digest, same as market
    }
    p.push_str(
        "\nReturn EXACTLY: {\"parent_node\":<integer index from the landscape, or -1 for a fresh root>,\
         \"proof_body\":\"<the Lean tactic block AFTER `:= by`, no theorem signature, no imports>\",\
         \"confidence\":0.0-1.0}\n",
    );
    p
}

/// Resolve the model-chosen `parent_node` index against the canonical live node list.
/// FAIL-OPEN to a fresh root: a negative index OR an out-of-range (hallucinated) index → None
/// (do NOT panic, do NOT parse-fail — that would shrink the autonomous arm's node count below
/// market's and break budget parity). A valid index → the real WorkTx id at that position.
fn resolve_parent_index(node_tx_ids: &[TxId], chosen: i64) -> Option<TxId> {
    if chosen < 0 {
        None
    } else {
        node_tx_ids.get(chosen as usize).cloned()
    }
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
    // BUDGET PARITY (forensic fix 2026-06-01): every policy gets the SAME total proposal budget
    // = args.n_agents * args.n_rounds LLM proposals (+ the matching Lean verifies). Single is forced
    // to 1 agent, so it must run that many ROUNDS to match — else `market` silently gets n_agents× the
    // compute and any "market > single" is a budget artifact, not a market effect.
    let effective_rounds = if args.policy == Policy::Single { args.n_rounds * args.n_agents } else { args.n_rounds };
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
    let mut node_doubt: BTreeMap<String, i64> = BTreeMap::new();
    let mut verified_agents: BTreeSet<String> = BTreeSet::new();
    let majority_threshold = agents.len() / 2 + 1;
    let (mut llm_calls, mut parse_fails, mut verified_count, mut failed_count) = (0usize, 0usize, 0usize, 0usize);
    let (mut bear_calls, mut bear_tokens_total) = (0usize, 0u64);
    // Route telemetry counters (autonomous arm; Class 1, no behavior change).
    let (mut route_fresh, mut route_hit, mut route_halluc) = (0usize, 0usize, 0usize);
    let mut omega_node: Option<String> = None;
    let mut time_to_first_proof_s: Option<f64> = None;
    let mut step_idx = 0u64;

    'outer: for round in 0..effective_rounds {
        for ai in 0..agents.len() {
            let agent = agents[ai].clone();
            let q = seq.q_snapshot().map_err(|e| format!("{e:?}"))?;
            root = q.state_root_t;
            let pi = compute_price_index(&q.economic_state_t);

            // Parent selection (policy-governed).
            let mut rng = StdRng::seed_from_u64(args.seed + round as u64 * 131 + ai as u64);
            let parent_tx = select_parent(args.policy, &pi, &node_tx_ids, own_last.get(&agent), &node_conf, &node_doubt, args.boltzmann_temp, &mut rng);
            let (parent_body, parent_feedback) = match &parent_tx {
                Some(t) => (node_body.get(&t.0).cloned(), node_feedback.get(&t.0).cloned()),
                None => (None, None),
            };

            // REAL librarian: shielded collective failure memory derived from the typed
            // LeanResult sidecars written into CAS on prior attempts (all agents). `lt` is the
            // run's monotonic logical clock → meaningful staleness; the problem id is the scope tag.
            let lib = real_librarian_solver_notice(&args.cas, lt, &args.problem);
            let prompt = if args.policy == Policy::Autonomous {
                build_autonomous_prompt(&theorem, &node_tx_ids, &node_body, &node_feedback, &node_conf, &pi, &lib)
            } else {
                build_prompt(&theorem, parent_body.as_deref(), parent_feedback.as_deref(), &lib)
            };
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

            // AUTONOMOUS: the model picked its own parent index; validate it against the live
            // node list (fail-open to a fresh root on a hallucinated/out-of-range index — never
            // crash, never parse-fail). `select_parent` returned None for this arm (STEP 0); we
            // shadow it here with the model's choice. Non-autonomous arms keep the pre-call pick.
            let mut parent_tx = parent_tx;
            if args.policy == Policy::Autonomous {
                let chosen = v.get("parent_node").and_then(|x| x.as_i64()).unwrap_or(-1);
                parent_tx = resolve_parent_index(&node_tx_ids, chosen);
                // Route telemetry (Class 1, no behavior change): split the fail-open resolve into
                // {deliberate_fresh_root, valid_index_hit, hallucinated_out_of_range} so the run can
                // prove the headline mechanism fired (real non-local routing) vs a bailed-out
                // hallucination. resolve_parent_index above is unchanged; we only observe `chosen`.
                if chosen < 0 {
                    route_fresh += 1;
                } else if parent_tx.is_some() {
                    route_hit += 1;
                } else {
                    route_halluc += 1;
                }
            }

            // ── Real Lean kernel verdict ─────────────────────────────
            let outcome = judge.verify(&body);
            let is_verified = outcome.is_verified();
            if is_verified {
                verified_count += 1;
            } else {
                failed_count += 1;
            }

            // Feed the REAL librarian: write the typed LeanResult sidecar that
            // `select_librarian_events` consumes. Raw stderr is NOT broadcast (stderr_cid=None);
            // the librarian reads only the shielded error CLASS / verdict kind. Pass LeanOutcome's
            // own fields verbatim so the 4-arm (exit_code, verified, error_class, verdict_kind)
            // byte-consistency (assert_45) holds. Open-per-write (mirrors put_proposal). Non-fatal.
            if let Ok(mut cas_w) = CasStore::open(&args.cas) {
                let lean_result = LeanResult {
                    attempt_id: TxId(format!("lm-node{step_idx}-{}", args.run_id)),
                    exit_code: outcome.exit_code,
                    verified: is_verified,
                    stderr_cid: None,
                    stdout_cid: None,
                    proof_artifact_cid: None,
                    error_class: outcome.error_class,
                    verdict_kind: outcome.verdict_kind,
                };
                if let Err(e) = write_lean_result_to_cas(&mut cas_w, &lean_result, "lm-lean-result", lt) {
                    eprintln!("lm lean_result write skip node{step_idx}: {e:?}");
                }
                lt += 1;
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

            // B6 skeptic-rerank: the SAME skeptic scores each node (critic-matched budget) to
            // drive argmin-doubt selection — NOT a market short. Isolates "a critic helped" from
            // "the market helped" (prereg v2 rule 7). Bear tokens count toward budget.
            if args.policy == Policy::SkepticRerank {
                let (doubt_micro, bear_tok) = bear_doubt_short(&llm, &args.model, &theorem, &body).await;
                bear_calls += 1;
                bear_tokens_total += bear_tok;
                node_doubt.insert(work_tx_id.clone(), doubt_micro);
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
        route_deliberate_fresh_root: route_fresh,
        route_valid_index_hit: route_hit,
        route_hallucinated_out_of_range: route_halluc,
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
            let got = select_parent(p, &pi, &nodes, Some(&own), &conf, &BTreeMap::new(), 0.15, &mut rng);
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
        assert_eq!(select_parent(Policy::Parallel, &pi, &nodes, None, &conf, &BTreeMap::new(), 0.15, &mut rng), None);
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
            select_parent(Policy::BestFirst, &pi, &nodes, None, &conf, &BTreeMap::new(), 0.15, &mut rng),
            Some(TxId("hi".into()))
        );
    }

    #[test]
    fn only_price_family_emits_bear_shorts() {
        for p in [Policy::Market, Policy::Autonomous, Policy::RandomBear, Policy::FixedBear, Policy::ShuffledPrice, Policy::NoPrice] {
            assert!(p.emits_challenges(), "{p:?} is price-family");
        }
        for p in [Policy::Single, Policy::Parallel, Policy::Majority, Policy::BestFirst, Policy::SkepticRerank] {
            assert!(!p.emits_challenges(), "{p:?} is Bulls-only");
        }
    }

    #[test]
    fn policy_parse_roundtrips_all_arms() {
        for s in ["market", "autonomous", "random_bear", "fixed_bear", "shuffled_price", "no_price", "single", "parallel", "majority", "best_first", "skeptic_rerank"] {
            assert_eq!(Policy::parse(s).unwrap().label(), s);
        }
        assert!(Policy::parse("bogus").is_err());
    }

    #[test]
    fn autonomous_select_parent_is_precall_noop() {
        // The autonomous parent is chosen by the LLM, not by select_parent; the pre-call
        // selector MUST be a no-op (None) regardless of the landscape, so the post-parse
        // shadow is the sole source of the parent.
        let mut pi = BTreeMap::new();
        let nodes = vec![TxId("n0".into()), TxId("n1".into())];
        // even with a fully-priced landscape, autonomous pre-call selection is None.
        pi.insert(TxId("n0".into()), NodeMarketEntry::default());
        let mut rng = StdRng::seed_from_u64(7);
        assert_eq!(
            select_parent(Policy::Autonomous, &pi, &nodes, Some(&TxId("n0".into())), &BTreeMap::new(), &BTreeMap::new(), 0.15, &mut rng),
            None
        );
    }

    #[test]
    fn autonomous_parent_index_resolves_and_fails_open() {
        let nodes = vec![TxId("n0".into()), TxId("n1".into())];
        // valid index → that node
        assert_eq!(resolve_parent_index(&nodes, 1), Some(TxId("n1".into())));
        assert_eq!(resolve_parent_index(&nodes, 0), Some(TxId("n0".into())));
        // fresh-root sentinel → None
        assert_eq!(resolve_parent_index(&nodes, -1), None);
        // out-of-range (hallucinated) → fail-OPEN to None (not a panic, not a parse-fail)
        assert_eq!(resolve_parent_index(&nodes, 5), None);
        // empty landscape → None for any index
        assert_eq!(resolve_parent_index(&[], 0), None);
    }
}
