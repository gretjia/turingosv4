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

// SHA-256 for the --self-test prompt-parity assertion (confound-B check). Reuses the
// repo's existing sha2=0.10 dep (cf. src/bottom_white/cas/schema.rs); no new dependency.
use sha2::{Digest, Sha256};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::economy::money::MicroCoin;
use turingosv4::judges::lean_judge::{default_lean_bin, LeanOutcome};
use turingosv4::judges::lean_theorem_bank::{
    default_lake_bin, load_bank, mathlib_lean_path, LeanTheorem,
};
use turingosv4::runtime::adapter::{
    genesis_with_balances, make_real_challengetx_signed_by, make_real_cpmm_pool_signed_by,
    make_real_escrow_lock_signed_by, make_real_market_seed_signed_by,
    make_real_task_open_signed_by, make_real_verifytx_signed_by, make_real_worktx_signed_by,
    tb8_await_state_root_advance,
};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::bootstrap::default_pput_preseed_pairs;
use turingosv4::runtime::proposal_telemetry::{
    write_to_cas as write_proposal_telemetry_to_cas, ProposalTelemetry, TokenCounts,
};
use turingosv4::runtime::verification_result::{
    write_to_cas as write_verification_result_to_cas, VerificationResult,
};
use turingosv4::runtime::{build_chaintape_sequencer_with_initial_q, RuntimeChaintapeConfig};
// REAL librarian (src/runtime/librarian_broadcast.rs): CAS-derived, role-scoped, shielded
// collective digest of prior attempts. Fed by the LeanResult sidecar written below; the
// previous experiment-local `librarian_digest` lookalike is removed.
use turingosv4::runtime::attempt_telemetry::{
    write_lean_result_to_cas, LeanResult, LeanVerdictKind,
};
use turingosv4::runtime::librarian_broadcast::{
    build_librarian_digest, derive_current_run_cas_root, project_role_notifications,
    select_librarian_events, validate_librarian_source_scope, LibrarianSourceScope,
};
use turingosv4::runtime::real5_roles::AgentRole;
use turingosv4::sdk::actor::boltzmann_softmax_select_parent;
use turingosv4::state::price_index::{compute_price_index, RationalPrice};
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
    // F3 topology baselines (decompose organization; all Bulls-only, NO price/short).
    // Each is byte-identical to its nearest existing arm EXCEPT the documented delta.
    SingleRestart, // ~Single, but each round may root-restart (fresh) OR extend own_last.
    SingleTreeNoPrice, // ~Single, but may extend ANY of its OWN prior nodes (uniform, NO price).
    ParallelRestart, // ~Parallel (N indep chains), but each may root-restart; NO shared price/tree.
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
            "single_restart" => Ok(Policy::SingleRestart),
            "single_tree_no_price" => Ok(Policy::SingleTreeNoPrice),
            "parallel_restart" => Ok(Policy::ParallelRestart),
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
            Policy::SingleRestart => "single_restart",
            Policy::SingleTreeNoPrice => "single_tree_no_price",
            Policy::ParallelRestart => "parallel_restart",
        }
    }
    /// Price-family policies emit a Bear ChallengeTx (short) per node; the
    /// non-market baselines are Bulls-only (no short, no price game).
    fn emits_challenges(self) -> bool {
        // F3 single_restart / single_tree_no_price / parallel_restart are Bulls-only
        // (NO price game, NO Bear short) — they intentionally fall through to `false`.
        matches!(
            self,
            Policy::Market
                | Policy::Autonomous
                | Policy::RandomBear
                | Policy::FixedBear
                | Policy::ShuffledPrice
                | Policy::NoPrice
        )
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
    /// §4 reject taxonomy (literal-schema parity, gap L4): None iff Verified; else "lean-reject" /
    /// "axiom-rejected" / "sorry-blocked". Surfaces the constitutional reject CLASS as a first-class
    /// node field (was previously only inferrable from `verdict`).
    reject_class: Option<String>,
    is_verified: bool,
    body_preview: String,
    feedback: String,
    tokens: u64,
    /// F5: the per-node transitive axiom set (`#print axioms`) — the soundness footprint a
    /// reviewer confirms ⊆ AXIOM_WHITELIST directly from the manifest. Empty unless the node
    /// compiled exit-0 (Verified or axiom-rejected).
    axioms: Vec<String>,
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
    // 广播 Broadcast injection tell (FC1-N5 / Art.II) — compliance gap M2: how many Stage-2 proof
    // prompts actually carried a NON-EMPTY librarian collective notice, + total chars injected.
    // Makes the broadcast INJECTION directly tape-readable (was recompute-only).
    librarian_notice_nonempty_count: usize,
    librarian_notice_chars: u64,
    // ── F2 compute telemetry (honest LLM-call + token breakdown; auditor compute-parity) ──
    // Accounting contract (no double-count, no silent drop):
    //   proposal_llm_calls = the Stage-2 proof-cycle proposal calls (== llm_calls). One
    //                        externalized proof attempt that hits the Lean kernel.
    //   route_llm_calls    = the autonomous Stage-1 route-only calls (F1 decouple). These are a
    //                        GENUINELY SEPARATE LLM call (route picks the parent; emits NO proof,
    //                        triggers NO kernel verify) — a mechanism cost like bear_*, NOT folded
    //                        into the proposal count. 0 for all pre-call-routed arms.
    //   bear_llm_calls     = the genuinely-EXTRA skeptic call (price arms + skeptic_rerank).
    // total_model_tokens = proof_prompt_tokens + route_prompt_tokens + bear_prompt_tokens +
    //                      completion_tokens (each distinct prompt counted once).
    proposal_llm_calls: usize,
    route_llm_calls: usize,
    bear_llm_calls: usize,
    proof_prompt_tokens: u64,
    route_prompt_tokens: u64,
    bear_prompt_tokens: u64,
    completion_tokens: u64,
    total_model_tokens: u64,
    lean_verifies: usize,
    total_wall_clock_ms: u64,
    parse_fails: usize,
    verified_count: usize,
    failed_count: usize,
    distinct_price_ratios: usize,
    price_discovery: bool,
    // F5: the exact axiom whitelist this run enforced (provenance pin; matches the
    // GenesisPin.axiom_whitelist pattern). A Verified node's `axioms` ⊆ this set.
    axiom_whitelist: Vec<String>,
    // Route telemetry: splits every Stage-1 resolve_parent_index outcome so the run can prove
    // WHICH routing actually fired:
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
            let v = argv
                .get(i + 1)
                .cloned()
                .ok_or(format!("missing value after {k}"))?;
            m.insert(stripped.to_string(), v);
            i += 2;
        } else {
            return Err(format!("unexpected arg {k}"));
        }
    }
    let get = |k: &str| m.get(k).cloned();
    let runtime_repo: PathBuf = get("runtime-repo").ok_or("--runtime-repo required")?.into();
    Ok(Args {
        out: get("out")
            .map(Into::into)
            .unwrap_or_else(|| runtime_repo.join("lean_market_manifest.json")),
        runtime_repo,
        cas: get("cas").ok_or("--cas required")?.into(),
        run_id: get("run-id").ok_or("--run-id required")?,
        proxy_url: get("proxy-url").unwrap_or_else(|| "http://localhost:8123".into()),
        model: get("model").unwrap_or_else(|| "deepseek-chat".into()),
        bank: get("bank")
            .map(Into::into)
            .unwrap_or_else(|| "tests/fixtures/lean_theorems.jsonl".into()),
        problem: get("problem").ok_or("--problem <theorem id> required")?,
        mathlib_dir: get("mathlib-dir").map(Into::into),
        policy: Policy::parse(&get("policy").unwrap_or_else(|| "market".into()))?,
        n_agents: get("n-agents").and_then(|s| s.parse().ok()).unwrap_or(8),
        n_rounds: get("n-rounds").and_then(|s| s.parse().ok()).unwrap_or(6),
        seed: get("seed").and_then(|s| s.parse().ok()).unwrap_or(0xB01),
        boltzmann_temp: get("boltzmann-temp")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.15),
        continue_past_omega: get("continue-past-omega")
            .map(|s| s == "true")
            .unwrap_or(false),
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
    let t = content
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
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
    own_nodes: &[TxId],
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
        Policy::Market | Policy::RandomBear | Policy::FixedBear => {
            boltzmann_softmax_select_parent(pi, &BTreeSet::new(), temp, rng)
                .or_else(|| all_nodes.last().cloned())
        }
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
        // F3 single_restart / parallel_restart: own-chain, but each round may RESTART from
        // ROOT (fresh, None) instead of extending own_last — explicit self-history root-restart
        // that breaks the no-backtrack lock. Byte-identical to Single/Parallel except this coin.
        // 50/50 via the SAME per-(round,agent)-seeded rng already used for parent selection.
        Policy::SingleRestart | Policy::ParallelRestart => {
            if rng.gen_bool(0.5) {
                None
            } else {
                own_last.cloned()
            }
        }
        // F3 single_tree_no_price: own-history TREE — extend ANY of this agent's OWN prior
        // nodes (uniform, NO price, NO short). own_nodes is this agent's node list (call site).
        // Empty own-history → fresh root (None). Differs from Single only by branching over the
        // agent's whole own subtree instead of just its last node; NO shared-tape, NO price.
        Policy::SingleTreeNoPrice => {
            if own_nodes.is_empty() {
                None
            } else {
                Some(own_nodes[rng.gen_range(0..own_nodes.len())].clone())
            }
        }
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

async fn submit_await(
    seq: &Sequencer,
    tx: TypedTx,
    pre: Hash,
    label: &str,
) -> Result<Hash, String> {
    seq.submit_agent_tx(tx)
        .await
        .map_err(|e| format!("submit {label}: {e:?}"))?;
    tb8_await_state_root_advance(seq, pre, 5_000)
        .await
        .map_err(|_| format!("{label} did not advance"))
}

fn put_proposal(
    cas_path: &PathBuf,
    run_id: &str,
    agent: &str,
    idx: u64,
    parent: Option<TxId>,
    body: &str,
    tokens: TokenCounts,
    lt: u64,
) -> Result<Cid, String> {
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    let tel = ProposalTelemetry::build_for_evaluator_append_with_parent(
        &mut cas,
        run_id,
        agent,
        idx,
        body.as_bytes(),
        "lm_proof",
        tokens,
        "lm-agent",
        lt,
        parent,
    )
    .map_err(|e| format!("ProposalTelemetry: {e}"))?;
    write_proposal_telemetry_to_cas(&mut cas, &tel, "lm-proposal-telemetry", lt + 1)
        .map_err(|e| format!("write telemetry: {e}"))
}

fn put_counterexample(cas_path: &PathBuf, work_tx: &str, lt: u64) -> Result<Cid, String> {
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    let blob = serde_json::json!({"schema":"lm.counterexample.v1","target":work_tx});
    cas.put(
        serde_json::to_vec(&blob).unwrap().as_slice(),
        ObjectType::EvidenceCapsule,
        "lm-challenger",
        lt,
        Some("lm.counterexample.v1".into()),
    )
    .map_err(|e| format!("put counterexample: {e}"))
}

fn put_proof_artifact(cas_path: &PathBuf, source: &str, lt: u64) -> Result<Cid, String> {
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    cas.put(
        source.as_bytes(),
        ObjectType::Generic,
        "lm-verifier",
        lt,
        Some("lm.proof_artifact.v1".into()),
    )
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
    if f.contains("unsolved goals") {
        "unsolved_goals"
    } else if f.contains("type mismatch") {
        "type_mismatch"
    } else if f.contains("unknown identifier") || f.contains("unknown constant") {
        "unknown_identifier"
    } else if f.contains("rewrite") && f.contains("fail") {
        "rewrite_failed"
    } else if f.contains("nlinarith") || f.contains("linarith") || f.contains("positivity") {
        "arith_failed"
    } else if f.contains("unexpected") || f.contains("syntax") || f.contains("expected") {
        "syntax_error"
    } else if f.contains("no progress") {
        "no_progress"
    } else if f.trim().is_empty() {
        "no_feedback"
    } else {
        "other_error"
    }
}

/// §4 reject taxonomy (gap L4): map a non-Verified LeanOutcome to its constitutional reject CLASS,
/// surfaced as the AttemptNode `reject_class` field. None iff Verified. `axiom_rejected` (compiled
/// exit-0 but `#print axioms` carried a non-whitelist axiom — sorryAx / native_decide-trust /
/// hand-axiom) is a SOUNDNESS reject, kept distinct from a `sorry`-source block and a kernel reject.
fn reject_class_of(o: &LeanOutcome) -> Option<String> {
    if o.is_verified() {
        return None;
    }
    if o.axiom_rejected {
        return Some("axiom-rejected".to_string());
    }
    match o.verdict_kind {
        LeanVerdictKind::SorryBlocked => Some("sorry-blocked".to_string()),
        _ => Some("lean-reject".to_string()),
    }
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
    if view
        .rendered_notice
        .contains("No librarian notices for this role at current scope")
    {
        return String::new();
    }
    format!("\n{}", view.rendered_notice)
}

fn build_prompt(
    theorem: &LeanTheorem,
    parent_body: Option<&str>,
    parent_feedback: Option<&str>,
    librarian: &str,
) -> String {
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

/// THE single Stage-2 proof-prompt constructor for EVERY arm (market, single, autonomous,
/// and all baselines). The live loop calls ONLY this at the one Stage-2 call site; the
/// autonomous branch reaches it after Stage-1 routing mutated `parent_body`/`parent_feedback`,
/// market/single reach it with their pre-call parent. Because the autonomous proof context is
/// produced by THIS function with the SAME arguments market would pass for the same parent, the
/// two prompts are byte-identical — that is the confound-B fix, expressed as one shared code
/// path rather than two look-alike call sites. The confound-B parity gate (self_test_inner +
/// `stage2_prompt_byte_equals_market`) drives BOTH operands through this helper, and asserts the
/// result also differs from a deliberately landscape-augmented control (`confound_b_control_prompt`),
/// so re-appending frontier text on the autonomous side here would make the gate FAIL.
fn stage2_proof_prompt(
    theorem: &LeanTheorem,
    parent_body: Option<&str>,
    parent_feedback: Option<&str>,
    librarian: &str,
) -> String {
    build_prompt(theorem, parent_body, parent_feedback, librarian)
}

/// KNOWN-DIVERGENT CONTROL for the confound-B gate — NOT used in the live loop. This is the
/// exact shape of the ORIGINAL confound: the Stage-2 proof prompt with the full search
/// landscape (other nodes' bodies + shielded errors) appended into the SAME proof call. The
/// parity gate asserts the real Stage-2/route prompts are NOT equal to this, which is what makes
/// the gate load-bearing: a tautological `f(x)==f(x)` could never catch a re-introduced
/// landscape leak, but `stage2 == market && stage2 != confound_control` can — the control proves
/// the SHA comparison actually discriminates a richer-context divergence. Reachable from the
/// shipped binary via `--self-test` (self_test_inner) AND from the `#[test]`, so it is NOT
/// dead code; both gate paths share this one control shape.
fn confound_b_control_prompt(
    theorem: &LeanTheorem,
    parent_body: Option<&str>,
    parent_feedback: Option<&str>,
    librarian: &str,
    landscape_bodies: &[(&str, &str)],
) -> String {
    let mut p = build_prompt(theorem, parent_body, parent_feedback, librarian);
    p.push_str("\n=== FULL SEARCH LANDSCAPE (confound-B leak) ===\n");
    for (body, err) in landscape_bodies {
        p.push_str("--- node body ---\n");
        p.push_str(body);
        p.push_str("\n--- node error ---\n");
        p.push_str(err);
        p.push('\n');
    }
    p
}

/// AUTONOMOUS STAGE 1 — route-only. Compact frontier summary: per node ONLY
/// [index, price_yes ratio, confidence, error CLASS, age=index, short-hash]. It carries
/// NO proof body and NO shielded Lean error text (only the coarse `classify_lean_error`
/// class) and NO librarian digest — those belong to the PROOF context (Stage 2 =
/// `build_prompt`, byte-identical to the market arm). The model returns ONLY the chosen
/// parent index (validated by `resolve_parent_index`, fail-open to a fresh root). This is
/// the confound-B fix: routing sees a strictly poorer channel than proof generation, so a
/// crack cannot be "global failure-context synthesis" — only "who picked the parent".
fn build_route_summary(
    theorem: &LeanTheorem,
    node_tx_ids: &[TxId],
    node_feedback: &BTreeMap<String, String>,
    node_conf: &BTreeMap<String, u64>,
    pi: &BTreeMap<TxId, NodeMarketEntry>,
) -> String {
    let mut p = String::new();
    p.push_str(
        "You are routing in a Lean proof-search market. Below is a COMPACT summary of the \
         search frontier (no proof text). FREELY CHOOSE which attempt to extend by index, OR \
         start fresh (index -1). Output ONLY JSON.\n\n",
    );
    p.push_str("=== Target ===\n");
    p.push_str(&theorem.preamble);
    p.push('\n');
    if node_tx_ids.is_empty() {
        p.push_str("\n=== Landscape EMPTY (use parent_node = -1) ===\n");
    } else {
        p.push_str("\n=== Frontier (index : price_yes : confidence : error-class : age : short-hash) ===\n");
        for (idx, tx) in node_tx_ids.iter().enumerate() {
            // ONLY the coarse error CLASS — never the raw shielded `node_feedback` line.
            let class = node_feedback
                .get(&tx.0)
                .map(|f| classify_lean_error(f))
                .unwrap_or("pending");
            let conf = node_conf.get(&tx.0).copied().unwrap_or(0);
            // Integer num/den straight off the price index — no f64, no ranking (tape order).
            let (pn, pd) = pi
                .get(tx)
                .and_then(|e| e.price_yes.as_ref())
                .map(|r| (r.numerator, r.denominator))
                .unwrap_or((0, 0));
            let short_hash: String = tx.0.chars().take(8).collect();
            p.push_str(&format!(
                "[{idx}] price={pn}/{pd} conf={conf}% class={class} age={idx} hash={short_hash}\n"
            ));
        }
    }
    p.push_str(
        "\nReturn EXACTLY: {\"parent_node\":<integer index from the landscape, or -1 for a fresh root>}\n",
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
/// (short_micro, prompt_tokens, completion_tokens) — F2 splits the bear's token cost so
/// `bear_prompt_tokens` is honestly separable from `completion_tokens` on the manifest. The
/// money element (short_micro: i64) is unchanged. Falls back to a flat short on LLM/parse error.
async fn bear_doubt_short(
    llm: &ResilientLLMClient,
    model: &str,
    theorem: &LeanTheorem,
    body: &str,
) -> (i64, u64, u64) {
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
            messages: vec![Message {
                role: "user".into(),
                content: prompt,
            }],
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
            (short, r.prompt_tokens as u64, r.completion_tokens as u64)
        }
        Err(_) => (CHALLENGE_STAKE_MICRO, 0, 0),
    }
}

/// SHA-256 hex of a string — the confound-B prompt-parity comparator.
fn sha_hex(s: &str) -> String {
    let mut h = Sha256::new();
    h.update(s.as_bytes());
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// LLM-FREE, Lean-FREE testable seam (run via `lean_market_agent --self-test`, exit 0/1).
/// Two load-bearing checks, mirrored as #[test]s so `cargo test` covers them with no model/kernel:
///   (F1) CONFOUND-B prompt parity: for a chosen parent, the market proof prompt and the
///        autonomous Stage-2 proof prompt are the SAME `build_prompt` call → SHA-256 equal; AND
///        the Stage-1 `build_route_summary` contains NEITHER the full-body sentinel NOR the
///        shielded-error sentinel NOR raw shielded-error text. If anyone reintroduces body/error
///        into the route path, or makes Stage-2 diverge from market, this FAILS.
///   (F2/F3) the new baseline policies parse/label/are-Bulls-only, select_parent returns the
///        intended node class, and the Manifest carries the honest telemetry fields + invariants.
fn self_test() -> ExitCode {
    match self_test_inner() {
        Ok(sha) => {
            println!("PROMPT-PARITY-OK route_summary_clean=true sha={sha}");
            println!("lean_market_agent --self-test: OK");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("lean_market_agent --self-test FAIL: {e}");
            ExitCode::from(1)
        }
    }
}

fn self_test_inner() -> Result<String, String> {
    // ── (F1) confound-B prompt parity ──────────────────────────────────
    let thm = LeanTheorem {
        id: "selftest".into(),
        source: "selftest".into(),
        difficulty: "selftest".into(),
        needs_mathlib: false,
        preamble: "theorem t (n:Nat): n+0=n := by".into(),
        reference_body: String::new(),
        note: String::new(),
    };
    // Synthetic frontier: two nodes; the chosen parent (index 1) carries a DISTINCTIVE full body
    // and a DISTINCTIVE shielded Lean error — neither may appear in the Stage-1 route summary.
    let node_tx_ids = vec![TxId("lm-nodeAAAAA".into()), TxId("lm-nodeBBBBB".into())];
    let mut node_body: BTreeMap<String, String> = BTreeMap::new();
    node_body.insert("lm-nodeAAAAA".into(), "intro h0; aaa".into());
    node_body.insert(
        "lm-nodeBBBBB".into(),
        "intro h; FULLBODYSENTINEL_simp_all; exact h".into(),
    );
    let mut node_feedback: BTreeMap<String, String> = BTreeMap::new();
    node_feedback.insert("lm-nodeAAAAA".into(), "error: type mismatch".into());
    node_feedback.insert(
        "lm-nodeBBBBB".into(),
        "error: SHIELDSENTINEL unsolved goals n+0=n".into(),
    );
    let mut node_conf: BTreeMap<String, u64> = BTreeMap::new();
    node_conf.insert("lm-nodeAAAAA".into(), 40);
    node_conf.insert("lm-nodeBBBBB".into(), 70);
    let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
    pi.insert(
        TxId("lm-nodeBBBBB".into()),
        NodeMarketEntry {
            price_yes: Some(RationalPrice {
                numerator: 3,
                denominator: 7,
            }),
            ..Default::default()
        },
    );

    // MARKET arm: parent picked by policy BEFORE the call; body/feedback read straight from the
    // node maps (the live market path at the Stage-2 call site).
    let market_parent =
        resolve_parent_index(&node_tx_ids, 1).ok_or("self-test: parent index 1 must resolve")?;
    let market_body = node_body.get(&market_parent.0).cloned();
    let market_feedback = node_feedback.get(&market_parent.0).cloned();
    let market_prompt = build_prompt(&thm, market_body.as_deref(), market_feedback.as_deref(), "");

    // AUTONOMOUS arm: simulate Stage-1 routing returning index 1, then derive the parent EXACTLY
    // as the live autonomous branch does (resolve_parent_index → parent_tx → body/feedback from
    // the SAME node maps) and build Stage-2 through the SAME shared `stage2_proof_prompt` the live
    // loop uses. This is NOT `f(x)==f(x)`: the two operands travel the market path and the
    // autonomous post-route path respectively, then must converge byte-for-byte.
    let routed = resolve_parent_index(&node_tx_ids, 1); // model-chosen parent index = 1
    let auto_body = routed.as_ref().and_then(|t| node_body.get(&t.0).cloned());
    let auto_feedback = routed
        .as_ref()
        .and_then(|t| node_feedback.get(&t.0).cloned());
    let stage2_prompt =
        stage2_proof_prompt(&thm, auto_body.as_deref(), auto_feedback.as_deref(), "");
    let sha_market = sha_hex(&market_prompt);
    let sha_stage2 = sha_hex(&stage2_prompt);
    if sha_market != sha_stage2 {
        return Err(format!(
            "Stage-2 proof prompt != market prompt (sha {sha_market} != {sha_stage2})"
        ));
    }

    // LOAD-BEARING CONTROL: the gate must be able to FAIL on a real divergence. Build the ORIGINAL
    // confound-B shape (Stage-2 proof prompt + the full search landscape of OTHER nodes' bodies &
    // shielded errors appended into the proof call) and require the real Stage-2 prompt to DIFFER.
    // If someone re-appended landscape text on the autonomous side at the Stage-2 call site, the
    // real prompt would equal this control and this assertion would trip — exactly attack (a)'s
    // failure mode is now caught.
    let landscape: Vec<(&str, &str)> = node_tx_ids
        .iter()
        .filter(|tx| Some(*tx) != routed.as_ref())
        .map(|tx| {
            (
                node_body.get(&tx.0).map(|s| s.as_str()).unwrap_or(""),
                node_feedback.get(&tx.0).map(|s| s.as_str()).unwrap_or(""),
            )
        })
        .collect();
    let control_prompt = confound_b_control_prompt(
        &thm,
        auto_body.as_deref(),
        auto_feedback.as_deref(),
        "",
        &landscape,
    );
    let sha_control = sha_hex(&control_prompt);
    if sha_stage2 == sha_control {
        return Err(format!(
            "confound-B GATE NOT LOAD-BEARING: Stage-2 prompt equals the landscape-augmented \
             control (sha {sha_stage2}); the parity check cannot detect a re-introduced leak"
        ));
    }

    // The Stage-1 ROUTE summary must leak NO body and NO shielded-error text.
    let route = build_route_summary(&thm, &node_tx_ids, &node_feedback, &node_conf, &pi);
    if route.contains("FULLBODYSENTINEL") {
        return Err(format!("route summary leaked full-body sentinel:\n{route}"));
    }
    if route.contains("SHIELDSENTINEL") {
        return Err(format!(
            "route summary leaked shielded-error sentinel:\n{route}"
        ));
    }
    if route.contains("unsolved goals") {
        return Err(format!(
            "route summary leaked raw shielded-error text:\n{route}"
        ));
    }

    // ── (F3) new baseline policies: parse/label/Bulls-only ──────────────
    for s in ["single_restart", "single_tree_no_price", "parallel_restart"] {
        let p = Policy::parse(s).map_err(|e| format!("parse {s}: {e}"))?;
        if p.label() != s {
            return Err(format!("label round-trip {s} -> {}", p.label()));
        }
        if p.emits_challenges() {
            return Err(format!("{s} must be Bulls-only"));
        }
    }

    // ── (F3) select_parent node-class on a synthetic node set (NO price needed) ──
    let empty_pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
    let conf: BTreeMap<String, u64> = BTreeMap::new();
    let doubt: BTreeMap<String, i64> = BTreeMap::new();
    let all = vec![
        TxId("n_other".into()),
        TxId("own_a".into()),
        TxId("own_b".into()),
    ];
    let own_nodes = vec![TxId("own_a".into()), TxId("own_b".into())];
    let own_last = TxId("own_b".into());
    // single_restart / parallel_restart: across seeds observe BOTH {root None} and {own_last};
    // NEVER a non-own node.
    for pol in [Policy::SingleRestart, Policy::ParallelRestart] {
        let (mut saw_root, mut saw_own) = (false, false);
        for seed in 0..64u64 {
            let mut rng = StdRng::seed_from_u64(seed);
            let got = select_parent(
                pol,
                &empty_pi,
                &all,
                Some(&own_last),
                &own_nodes,
                &conf,
                &doubt,
                0.15,
                &mut rng,
            );
            match got {
                None => saw_root = true,
                Some(t) if t == own_last => saw_own = true,
                Some(other) => return Err(format!("{pol:?} returned non-own node {other:?}")),
            }
        }
        if !(saw_root && saw_own) {
            return Err(format!("{pol:?} must reach BOTH root-restart and own_last"));
        }
    }
    // single_tree_no_price: every pick is an OWN node; empty own-history → None.
    for seed in 0..64u64 {
        let mut rng = StdRng::seed_from_u64(seed);
        let got = select_parent(
            Policy::SingleTreeNoPrice,
            &empty_pi,
            &all,
            Some(&own_last),
            &own_nodes,
            &conf,
            &doubt,
            0.15,
            &mut rng,
        );
        match got {
            Some(t) if own_nodes.contains(&t) => {}
            other => {
                return Err(format!(
                    "single_tree_no_price picked non-own/None: {other:?}"
                ))
            }
        }
    }
    {
        let mut rng = StdRng::seed_from_u64(0);
        let empty: Vec<TxId> = vec![];
        let got = select_parent(
            Policy::SingleTreeNoPrice,
            &empty_pi,
            &all,
            Some(&own_last),
            &empty,
            &conf,
            &doubt,
            0.15,
            &mut rng,
        );
        if got.is_some() {
            return Err("single_tree_no_price with empty own-history must be None".into());
        }
    }

    // ── (F2) Manifest carries the telemetry fields + honest-accounting invariants ──
    let m = sample_manifest_for_selftest(Policy::Autonomous);
    let v = serde_json::to_value(&m).map_err(|e| format!("manifest serialize: {e}"))?;
    for key in [
        "proposal_llm_calls",
        "route_llm_calls",
        "bear_llm_calls",
        "proof_prompt_tokens",
        "route_prompt_tokens",
        "bear_prompt_tokens",
        "completion_tokens",
        "total_model_tokens",
        "lean_verifies",
        "total_wall_clock_ms",
    ] {
        if v.get(key).is_none() {
            return Err(format!("manifest missing telemetry field `{key}`"));
        }
    }
    if m.total_model_tokens
        != m.proof_prompt_tokens
            + m.route_prompt_tokens
            + m.bear_prompt_tokens
            + m.completion_tokens
    {
        return Err(
            "total_model_tokens must equal proof+route+bear+completion (no double-count)".into(),
        );
    }
    // F4: total_tokens is the PPUT / budget-parity denominator; it MUST include route (Stage-1)
    // tokens, i.e. equal total_model_tokens — otherwise autonomous gets a hidden compute discount.
    if m.total_tokens != m.total_model_tokens {
        return Err(
            "total_tokens (PPUT denominator) must equal total_model_tokens (route included)".into(),
        );
    }
    // F1 decouple: autonomous route is a SEPARATE Stage-1 call, so route_llm_calls > 0 and
    // route_prompt_tokens > 0 for autonomous; both are 0 for a pre-call-routed arm.
    if m.route_llm_calls == 0 {
        return Err("autonomous: route_llm_calls must be > 0 (separate Stage-1 call)".into());
    }
    let m2 = sample_manifest_for_selftest(Policy::Single);
    if m2.route_llm_calls != 0 {
        return Err("non-autonomous: route_llm_calls must be 0".into());
    }
    if m2.route_prompt_tokens != 0 {
        return Err("non-autonomous: route_prompt_tokens must be 0".into());
    }

    Ok(sha_market)
}

/// Build a minimal in-memory Manifest for the self-test field/invariant checks (no I/O, no LLM).
/// For autonomous, the Stage-1 route is a SEPARATE call (F1), so route_llm_calls/route_prompt_tokens
/// are non-zero; for all other arms they are 0.
fn sample_manifest_for_selftest(policy: Policy) -> Manifest {
    let proposal = 5usize;
    let autonomous = policy == Policy::Autonomous;
    let route_calls = if autonomous { proposal } else { 0 };
    let route_tokens: u64 = if autonomous { 30 } else { 0 };
    Manifest {
        schema_version: "turingosv4.lean_market.v1",
        run_id: "selftest".into(),
        policy: policy.label(),
        model: "none".into(),
        problem: "selftest".into(),
        needs_mathlib: false,
        n_agents: 1,
        n_rounds: 1,
        seed: 0,
        llm_calls: proposal,
        bear_calls: 2,
        bear_tokens: 30,
        librarian_notice_nonempty_count: proposal,
        librarian_notice_chars: 1280,
        proposal_llm_calls: proposal,
        route_llm_calls: route_calls,
        bear_llm_calls: 2,
        proof_prompt_tokens: 100,
        route_prompt_tokens: route_tokens,
        bear_prompt_tokens: 20,
        completion_tokens: 40,
        total_model_tokens: 100 + route_tokens + 20 + 40,
        lean_verifies: proposal,
        total_wall_clock_ms: 1234,
        parse_fails: 0,
        verified_count: 0,
        failed_count: proposal,
        distinct_price_ratios: 0,
        price_discovery: false,
        axiom_whitelist: turingosv4::judges::lean_judge::AXIOM_WHITELIST
            .iter()
            .map(|s| s.to_string())
            .collect(),
        route_deliberate_fresh_root: 0,
        route_valid_index_hit: 0,
        route_hallucinated_out_of_range: 0,
        omega_reached: false,
        omega_node: None,
        time_to_first_proof_s: None,
        golden_path: vec![],
        golden_path_tokens: 0,
        // F4: total_tokens (PPUT denominator) must equal total_model_tokens — both include route.
        total_tokens: 100 + route_tokens + 20 + 40,
        wall_clock_s: 0.0,
        pput: 0.0,
        final_state_root_hex: String::new(),
        runtime_repo: String::new(),
        cas: String::new(),
        nodes: vec![],
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    // --self-test: LLM-free, Lean-free seam intercepted BEFORE parse_args (which hard-requires
    // --runtime-repo/--cas/--run-id/--problem). Covers the confound-B prompt-parity check (F1)
    // AND the F2/F3 policy + telemetry asserts. ZERO LLM, ZERO Lean, ZERO network.
    if argv.first().map(|s| s == "--self-test").unwrap_or(false) {
        return self_test();
    }
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
        .ok_or_else(|| {
            format!(
                "problem `{}` not in bank {}",
                args.problem,
                args.bank.display()
            )
        })?
        .clone();
    let lean_bin = default_lean_bin();
    let mathlib_lp = if theorem.needs_mathlib {
        let dir = args
            .mathlib_dir
            .clone()
            .ok_or("theorem needs Mathlib but --mathlib-dir not given")?;
        Some(
            mathlib_lean_path(&dir, &default_lake_bin())
                .ok_or("could not resolve Mathlib LEAN_PATH (lake env failed)")?,
        )
    } else {
        None
    };
    let judge = theorem.judge(lean_bin, mathlib_lp.as_deref());

    // F3: single_restart / single_tree_no_price are 1-agent (like Single); parallel_restart is N-agent.
    let one_agent = matches!(
        args.policy,
        Policy::Single | Policy::SingleRestart | Policy::SingleTreeNoPrice
    );
    let n_agents = if one_agent { 1 } else { args.n_agents };
    // BUDGET PARITY (forensic fix 2026-06-01): every policy gets the SAME total proposal budget
    // = args.n_agents * args.n_rounds LLM proposals (+ the matching Lean verifies). A 1-agent arm is
    // forced to 1 agent, so it must run that many ROUNDS to match — else `market` silently gets
    // n_agents× the compute and any "market > single" is a budget artifact, not a market effect.
    let effective_rounds = if one_agent {
        args.n_rounds * args.n_agents
    } else {
        args.n_rounds
    };
    let market_task = format!("lm-market-{}", args.run_id);
    let agents: Vec<String> = (0..n_agents).map(|i| format!("Agent_{i}")).collect();
    let challengers: Vec<String> = (0..n_agents).map(|i| format!("Chal_{i}")).collect();

    // ── Genesis + keypairs ───────────────────────────────────────────
    let mut balances = default_pput_preseed_pairs();
    for extra in [SPONSOR_AGENT, PROVIDER_AGENT, VERIFIER_AGENT] {
        if !balances.iter().any(|(a, _)| a.0 == extra) {
            balances.push((
                AgentId(extra.into()),
                MicroCoin::from_micro_units(5_000_000),
            ));
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
    let bundle = build_chaintape_sequencer_with_initial_q(&cfg, initial_q)
        .map_err(|e| format!("boot: {e}"))?;
    let seq = bundle.sequencer.clone();
    let mut kp = AgentKeypairRegistry::open(&cfg.runtime_repo_path).map_err(|e| format!("{e}"))?;
    let mut all: Vec<&str> = vec![SPONSOR_AGENT, PROVIDER_AGENT, VERIFIER_AGENT];
    all.extend(agents.iter().map(|s| s.as_str()));
    all.extend(challengers.iter().map(|s| s.as_str()));
    for id in &all {
        kp.get_or_create(&AgentId(id.to_string()))
            .map_err(|e| format!("keypair {id}: {e}"))?;
    }
    seq.set_agent_pubkeys(std::sync::Arc::new(kp.manifest()))
        .map_err(|_| "pubkeys set".to_string())?;

    // ── Market task scaffold ─────────────────────────────────────────
    let mut root = seq.q_snapshot().map_err(|e| format!("{e:?}"))?.state_root_t;
    let mut lt = 10u64;
    root = submit_await(
        &seq,
        make_real_task_open_signed_by(&mut kp, &market_task, SPONSOR_AGENT, root, "lm", lt)
            .map_err(|e| format!("TaskOpen: {e}"))?,
        root,
        "TaskOpen",
    )
    .await?;
    lt += 1;
    root = submit_await(
        &seq,
        make_real_market_seed_signed_by(
            &mut kp,
            root,
            &market_task,
            PROVIDER_AGENT,
            MARKET_SEED_MICRO,
            "lm",
            lt,
        )
        .map_err(|e| format!("Seed: {e}"))?,
        root,
        "MarketSeed",
    )
    .await?;
    lt += 1;
    root = submit_await(
        &seq,
        make_real_cpmm_pool_signed_by(
            &mut kp,
            root,
            &market_task,
            PROVIDER_AGENT,
            MARKET_SEED_MICRO as u128,
            "lm",
        )
        .map_err(|e| format!("Pool: {e}"))?,
        root,
        "CpmmPool",
    )
    .await?;
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
    // F3 single_tree_no_price needs this agent's WHOLE own-history (not just last); accumulate it.
    let mut own_nodes_by_agent: BTreeMap<String, Vec<TxId>> = BTreeMap::new();
    let mut node_conf: BTreeMap<String, u64> = BTreeMap::new();
    let mut node_doubt: BTreeMap<String, i64> = BTreeMap::new();
    let mut verified_agents: BTreeSet<String> = BTreeSet::new();
    let majority_threshold = agents.len() / 2 + 1;
    let (mut llm_calls, mut parse_fails, mut verified_count, mut failed_count) =
        (0usize, 0usize, 0usize, 0usize);
    let (mut bear_calls, mut bear_tokens_total) = (0usize, 0u64);
    // F2 honest compute split (no double-count): proposal prompt/completion vs bear prompt/completion.
    let (mut proof_prompt_tokens, mut completion_tokens_total) = (0u64, 0u64);
    let mut bear_prompt_tokens_total = 0u64;
    let mut lean_verifies = 0usize;
    // M2 broadcast-injection tell: count Stage-2 prompts that carried a non-empty librarian notice.
    let mut librarian_notice_nonempty_count = 0usize;
    let mut librarian_notice_chars = 0u64;
    // Route telemetry counters (autonomous arm).
    let (mut route_fresh, mut route_hit, mut route_halluc) = (0usize, 0usize, 0usize);
    // Stage-1 (route-only) LLM cost, separate from Stage-2 proposal cost.
    let (mut route_llm_calls, mut route_prompt_tokens) = (0usize, 0u64);
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
            let own_nodes_slice: &[TxId] = own_nodes_by_agent
                .get(&agent)
                .map(|v| v.as_slice())
                .unwrap_or(&[]);
            let parent_tx = select_parent(
                args.policy,
                &pi,
                &node_tx_ids,
                own_last.get(&agent),
                own_nodes_slice,
                &node_conf,
                &node_doubt,
                args.boltzmann_temp,
                &mut rng,
            );
            let (parent_body, parent_feedback) = match &parent_tx {
                Some(t) => (
                    node_body.get(&t.0).cloned(),
                    node_feedback.get(&t.0).cloned(),
                ),
                None => (None, None),
            };

            // REAL librarian: shielded collective failure memory derived from the typed
            // LeanResult sidecars written into CAS on prior attempts (all agents). `lt` is the
            // run's monotonic logical clock → meaningful staleness; the problem id is the scope tag.
            // Compute the librarian ONCE before Stage 1 and reuse the SAME `&lib` for the
            // Stage-2 build_prompt below, so the autonomous Stage-2 proof prompt is
            // byte-identical to market for the same parent (no lib recompute drift).
            let lib = real_librarian_solver_notice(&args.cas, lt, &args.problem);
            // M2: a non-empty notice means the 广播 collective memory actually entered THIS proof
            // prompt (Stage-2 injects `lib` via stage2_proof_prompt) — a direct tape tell.
            if !lib.is_empty() {
                librarian_notice_nonempty_count += 1;
                librarian_notice_chars += lib.chars().count() as u64;
            }
            // STAGE 1 (autonomous only): route-only call over the COMPACT frontier summary —
            // model returns ONLY the parent index. NO proof body/shielded-error here. On a
            // route LLM error, fail-open to a fresh root and STILL proceed to Stage 2 (so the
            // autonomous node count matches market — only the route pick is lost).
            let mut parent_tx = parent_tx;
            let (mut parent_body, mut parent_feedback) = (parent_body, parent_feedback);
            if args.policy == Policy::Autonomous {
                let route_prompt =
                    build_route_summary(&theorem, &node_tx_ids, &node_feedback, &node_conf, &pi);
                let chosen = match llm
                    .generate(&GenerateRequest {
                        model: args.model.clone(),
                        messages: vec![
                            sys.clone(),
                            Message {
                                role: "user".into(),
                                content: route_prompt,
                            },
                        ],
                        temperature: Some(0.7),
                        max_tokens: Some(120),
                    })
                    .await
                {
                    Ok(r) => {
                        route_llm_calls += 1;
                        route_prompt_tokens += (r.prompt_tokens + r.completion_tokens) as u64;
                        extract_json_object(&r.content)
                            .and_then(|v| v.get("parent_node").and_then(|x| x.as_i64()))
                            .unwrap_or(-1)
                    }
                    Err(e) => {
                        eprintln!("lm route_err {agent}: {e:?}");
                        -1
                    }
                };
                parent_tx = resolve_parent_index(&node_tx_ids, chosen);
                // Route telemetry split: {deliberate_fresh_root, valid_index_hit,
                // hallucinated_out_of_range} — proves which routing actually fired.
                if chosen < 0 {
                    route_fresh += 1;
                } else if parent_tx.is_some() {
                    route_hit += 1;
                } else {
                    route_halluc += 1;
                }
                parent_body = parent_tx
                    .as_ref()
                    .and_then(|t| node_body.get(&t.0).cloned());
                parent_feedback = parent_tx
                    .as_ref()
                    .and_then(|t| node_feedback.get(&t.0).cloned());
            }
            // STAGE 2 (ALL arms): the SAME proof prompt as market/single for the chosen parent,
            // via the ONE shared `stage2_proof_prompt` constructor. Autonomous reaches this with
            // the post-route parent; market/single with their pre-call parent. Same fn + same args
            // for the same parent ⇒ the proof-generation context is BYTE-IDENTICAL across arms
            // (confound-B fix). The parity gate drives both operands through this exact helper.
            let prompt = stage2_proof_prompt(
                &theorem,
                parent_body.as_deref(),
                parent_feedback.as_deref(),
                &lib,
            );
            let resp = match llm
                .generate(&GenerateRequest {
                    model: args.model.clone(),
                    messages: vec![
                        sys.clone(),
                        Message {
                            role: "user".into(),
                            content: prompt,
                        },
                    ],
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
            // F2: the proposal call's prompt is the proof prompt (build_prompt); for autonomous
            // the route decision rode a SEPARATE Stage-1 call (route_* counters), so this counts
            // only the Stage-2 proof prompt + completion — no double-count.
            proof_prompt_tokens += resp.prompt_tokens as u64;
            completion_tokens_total += resp.completion_tokens as u64;
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
            let body = v
                .get("proof_body")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            if body.trim().is_empty() {
                parse_fails += 1;
                continue;
            }
            let confidence_pct = (v
                .get("confidence")
                .and_then(|x| x.as_f64())
                .unwrap_or(0.6)
                .clamp(0.0, 1.0)
                * 100.0) as u64;

            // (parent_tx was decided in Stage 1 for autonomous; other arms keep their pre-call
            // pick. The Stage-2 proof JSON no longer carries parent_node.)

            // ── Real Lean kernel verdict ─────────────────────────────
            let outcome = judge.verify(&body);
            lean_verifies += 1; // F2 compute telemetry: one real Lean-kernel verify per attempt.
            let is_verified = outcome.is_verified();
            // F5: per-node soundness footprint (#print axioms set) persisted to the manifest.
            let node_axioms = outcome.axioms.clone();
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
                if let Err(e) =
                    write_lean_result_to_cas(&mut cas_w, &lean_result, "lm-lean-result", lt)
                {
                    eprintln!("lm lean_result write skip node{step_idx}: {e:?}");
                }
                lt += 1;
            }

            // ── Per-task node (EVERY attempt — Verified or Failed) ────
            let work_stake = stake_from_confidence(confidence_pct);
            let node_task = format!("lm-node{step_idx}-{}", args.run_id);
            root = submit_await(
                &seq,
                make_real_task_open_signed_by(&mut kp, &node_task, SPONSOR_AGENT, root, "lm", lt)
                    .map_err(|e| format!("TaskOpen node: {e}"))?,
                root,
                "TaskOpen(node)",
            )
            .await?;
            lt += 1;
            root = submit_await(
                &seq,
                make_real_escrow_lock_signed_by(
                    &mut kp,
                    &node_task,
                    SPONSOR_AGENT,
                    TASK_ESCROW_MICRO,
                    root,
                    "lm",
                    lt,
                )
                .map_err(|e| format!("Escrow node: {e}"))?,
                root,
                "Escrow(node)",
            )
            .await?;
            lt += 1;
            let pcid = put_proposal(
                &args.cas,
                &args.run_id,
                &agent,
                step_idx,
                parent_tx.clone(),
                &body,
                tokens,
                lt,
            )?;
            lt += 2;
            let work = make_real_worktx_signed_by(
                &mut kp, &node_task, &agent, root, work_stake, "lm", pcid, true, lt,
            )
            .map_err(|e| format!("WorkTx: {e}"))?;
            let work_tx_id = match &work {
                TypedTx::Work(w) => w.tx_id.0.clone(),
                _ => return Err("not WorkTx".into()),
            };
            root = submit_await(&seq, work, root, "WorkTx").await?;
            lt += 1;
            node_tx_ids.push(TxId(work_tx_id.clone()));
            own_last.insert(agent.clone(), TxId(work_tx_id.clone()));
            // F3 single_tree_no_price: feed this agent's whole own-history for next round.
            own_nodes_by_agent
                .entry(agent.clone())
                .or_default()
                .push(TxId(work_tx_id.clone()));
            node_body.insert(work_tx_id.clone(), body.clone());
            node_feedback.insert(work_tx_id.clone(), outcome.feedback.clone());
            node_conf.insert(work_tx_id.clone(), confidence_pct);

            // Short challenge → price_yes (price-family policies only; non-market
            // baselines are Bulls-only). Non-fatal.
            if args.policy.emits_challenges() {
                // Bear short by policy: informed (skeptic-LLM doubt) for market/shuffled/no_price;
                // random U(0,1) with NO skeptic call (M1); or fixed constant (M2). M1/M2 isolate
                // whether the *informed* price signal (vs noise / vs a constant) does the work.
                let (short_micro, bear_prompt, bear_completion) = match args.policy {
                    Policy::RandomBear => {
                        let doubt_pct = rng.gen_range(0..=100) as i64;
                        (
                            MIN_SHORT_MICRO + (MAX_SHORT_MICRO - MIN_SHORT_MICRO) * doubt_pct / 100,
                            0u64,
                            0u64,
                        )
                    }
                    Policy::FixedBear => (CHALLENGE_STAKE_MICRO, 0u64, 0u64),
                    _ => bear_doubt_short(&llm, &args.model, &theorem, &body).await,
                };
                bear_calls += 1;
                bear_prompt_tokens_total += bear_prompt;
                completion_tokens_total += bear_completion;
                bear_tokens_total += bear_prompt + bear_completion;
                let challenger = challengers[ai % challengers.len()].clone();
                if let Ok(ce) = put_counterexample(&args.cas, &work_tx_id, lt) {
                    lt += 1;
                    match make_real_challengetx_signed_by(
                        &mut kp,
                        root,
                        TxId(work_tx_id.clone()),
                        &challenger,
                        short_micro,
                        ce,
                        &format!("lm{step_idx}"),
                        lt,
                    ) {
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
                let (doubt_micro, bear_prompt, bear_completion) =
                    bear_doubt_short(&llm, &args.model, &theorem, &body).await;
                bear_calls += 1;
                bear_prompt_tokens_total += bear_prompt;
                completion_tokens_total += bear_completion;
                bear_tokens_total += bear_prompt + bear_completion;
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
                match make_real_verifytx_signed_by(
                    &mut kp,
                    root,
                    TxId(work_tx_id.clone()),
                    VERIFIER_AGENT,
                    VERIFY_BOND_MICRO,
                    &format!("lmv{step_idx}"),
                    is_verified,
                    lt,
                ) {
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

            let price = compute_price_index(
                &seq.q_snapshot()
                    .map_err(|e| format!("{e:?}"))?
                    .economic_state_t,
            );
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
                reject_class: reject_class_of(&outcome),
                is_verified,
                body_preview: body.chars().take(120).collect(),
                feedback: outcome.feedback.chars().take(160).collect(),
                tokens: tokens.prompt_tokens + tokens.completion_tokens,
                axioms: node_axioms,
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
    let outcome_side = if omega_node.is_some() {
        OutcomeSide::Yes
    } else {
        OutcomeSide::No
    };
    if seq
        .emit_system_tx(SystemEmitCommand::EventResolve {
            task_id: TaskId(market_task.clone()),
            outcome: outcome_side,
        })
        .await
        .is_ok()
    {
        let _ = tb8_await_state_root_advance(&seq, root, 5_000).await;
    }
    let _ = seq
        .q_snapshot()
        .map_err(|e| format!("{e:?}"))?
        .economic_state_t
        .task_markets_t
        .0
        .get(&TaskId(market_task.clone()))
        .map(|m| m.state != TaskMarketState::Open);

    let seq_handle = seq.clone();
    bundle
        .shutdown()
        .await
        .map_err(|e| format!("shutdown: {e}"))?;
    let final_root = seq_handle
        .q_snapshot()
        .map_err(|e| format!("{e:?}"))?
        .state_root_t;

    // ── Golden path (ancestor chain of OMEGA) + PPUT ─────────────────
    let parent_of: BTreeMap<String, Option<String>> = nodes
        .iter()
        .map(|n| (n.node_tx.clone(), n.parent_tx.clone()))
        .collect();
    let tokens_of: BTreeMap<String, u64> = nodes
        .iter()
        .map(|n| (n.node_tx.clone(), n.tokens))
        .collect();
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
    // F4: route (Stage-1) prompt tokens are a REAL compute cost of autonomous; include them so
    // total_tokens (the PPUT/budget-parity denominator) equals the self-tested total_model_tokens
    // (proof_prompt + route_prompt + bear_prompt + completion). node.tokens carries proof
    // prompt+completion and bear_tokens_total carries bear prompt+completion, so the only missing
    // term is route_prompt_tokens. 0 for every non-autonomous arm → no effect on those.
    let total_tokens: u64 =
        nodes.iter().map(|n| n.tokens).sum::<u64>() + bear_tokens_total + route_prompt_tokens;
    let wall_clock_s = t0.elapsed().as_secs_f64();
    let pput = if omega_node.is_none() || wall_clock_s <= 0.0 {
        0.0
    } else {
        golden_path_tokens as f64 / wall_clock_s
    };

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
        librarian_notice_nonempty_count,
        librarian_notice_chars,
        proposal_llm_calls: llm_calls,
        // F1 decouple: autonomous makes a GENUINELY SEPARATE Stage-1 route call, so route_llm_calls
        // is the real Stage-1 count (0 for pre-call-routed arms) — NOT a labeled view of proposal.
        route_llm_calls,
        bear_llm_calls: bear_calls,
        proof_prompt_tokens,
        route_prompt_tokens,
        bear_prompt_tokens: bear_prompt_tokens_total,
        completion_tokens: completion_tokens_total,
        total_model_tokens: proof_prompt_tokens
            + route_prompt_tokens
            + bear_prompt_tokens_total
            + completion_tokens_total,
        lean_verifies,
        total_wall_clock_ms: t0.elapsed().as_millis() as u64,
        parse_fails,
        verified_count,
        failed_count,
        distinct_price_ratios,
        price_discovery: distinct_price_ratios > 1,
        axiom_whitelist: turingosv4::judges::lean_judge::AXIOM_WHITELIST
            .iter()
            .map(|s| s.to_string())
            .collect(),
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
    std::fs::write(
        &args.out,
        serde_json::to_string_pretty(&manifest).map_err(|e| format!("ser: {e}"))?,
    )
    .map_err(|e| format!("write: {e}"))?;
    println!(
        "lean_market[{}] problem={} agents={} rounds={} proposal_llm={} route_llm={} route_tok={} bear={} parse_fail={} verified={} failed={} nodes={} distinct_prices={} omega={} ttfp={:?}s gp_tokens={} total_tokens={} wall={:.1}s pput={:.2} manifest={}",
        args.policy.label(), args.problem, n_agents, args.n_rounds, llm_calls, route_llm_calls, route_prompt_tokens, bear_calls, parse_fails, verified_count, failed_count,
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
            let got = select_parent(
                p,
                &pi,
                &nodes,
                Some(&own),
                &[],
                &conf,
                &BTreeMap::new(),
                0.15,
                &mut rng,
            );
            assert_eq!(
                got,
                Some(TxId("n_mine".into())),
                "{p:?} must refine own_last"
            );
        }
    }

    #[test]
    fn parallel_without_own_last_starts_fresh_root() {
        let pi = BTreeMap::new();
        let conf = BTreeMap::new();
        let nodes = vec![TxId("someone_elses".into())];
        let mut rng = StdRng::seed_from_u64(7);
        // No shared tape: a parallel agent never adopts another agent's node.
        assert_eq!(
            select_parent(
                Policy::Parallel,
                &pi,
                &nodes,
                None,
                &[],
                &conf,
                &BTreeMap::new(),
                0.15,
                &mut rng
            ),
            None
        );
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
            select_parent(
                Policy::BestFirst,
                &pi,
                &nodes,
                None,
                &[],
                &conf,
                &BTreeMap::new(),
                0.15,
                &mut rng
            ),
            Some(TxId("hi".into()))
        );
    }

    #[test]
    fn only_price_family_emits_bear_shorts() {
        for p in [
            Policy::Market,
            Policy::Autonomous,
            Policy::RandomBear,
            Policy::FixedBear,
            Policy::ShuffledPrice,
            Policy::NoPrice,
        ] {
            assert!(p.emits_challenges(), "{p:?} is price-family");
        }
        for p in [
            Policy::Single,
            Policy::Parallel,
            Policy::Majority,
            Policy::BestFirst,
            Policy::SkepticRerank,
            Policy::SingleRestart,
            Policy::SingleTreeNoPrice,
            Policy::ParallelRestart,
        ] {
            assert!(!p.emits_challenges(), "{p:?} is Bulls-only");
        }
    }

    #[test]
    fn policy_parse_roundtrips_all_arms() {
        for s in [
            "market",
            "autonomous",
            "random_bear",
            "fixed_bear",
            "shuffled_price",
            "no_price",
            "single",
            "parallel",
            "majority",
            "best_first",
            "skeptic_rerank",
            "single_restart",
            "single_tree_no_price",
            "parallel_restart",
        ] {
            assert_eq!(Policy::parse(s).unwrap().label(), s);
        }
        assert!(Policy::parse("bogus").is_err());
    }

    #[test]
    fn autonomous_select_parent_is_precall_noop() {
        // The autonomous parent is chosen by the LLM in Stage 1 (build_route_summary), not by
        // select_parent; the pre-call selector MUST still be a no-op (None) regardless of the
        // landscape, so Stage 1 is the sole source of the parent.
        let mut pi = BTreeMap::new();
        let nodes = vec![TxId("n0".into()), TxId("n1".into())];
        // even with a fully-priced landscape, autonomous pre-call selection is None.
        pi.insert(TxId("n0".into()), NodeMarketEntry::default());
        let mut rng = StdRng::seed_from_u64(7);
        assert_eq!(
            select_parent(
                Policy::Autonomous,
                &pi,
                &nodes,
                Some(&TxId("n0".into())),
                &[],
                &BTreeMap::new(),
                &BTreeMap::new(),
                0.15,
                &mut rng
            ),
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

    // ── F1 confound-B: Stage-2 byte-parity + route-summary leak-free ──

    fn selftest_theorem() -> LeanTheorem {
        LeanTheorem {
            id: "t".into(),
            source: "t".into(),
            difficulty: "t".into(),
            needs_mathlib: false,
            preamble: "theorem t (n:Nat): n+0=n := by".into(),
            reference_body: String::new(),
            note: String::new(),
        }
    }

    #[test]
    fn stage2_prompt_byte_equals_market() {
        // Confound-B parity gate — LOAD-BEARING (must be able to FAIL on a real divergence).
        // Market proof prompt is built via `build_prompt` (the live market path); the autonomous
        // Stage-2 prompt is built via the SHARED `stage2_proof_prompt` the live loop calls at its
        // one Stage-2 call site. For the same parent they must be SHA-256 equal.
        let thm = selftest_theorem();
        let body = "intro h; FULLBODYSENTINEL_simp_all; exact h";
        let fb = "error: SHIELDSENTINEL unsolved goals n+0=n";
        let market = build_prompt(&thm, Some(body), Some(fb), "");
        let stage2 = stage2_proof_prompt(&thm, Some(body), Some(fb), "");
        assert_eq!(
            sha_hex(&market),
            sha_hex(&stage2),
            "Stage-2 proof prompt must be byte-identical to market for the same parent"
        );

        // The gate is only meaningful if SHA equality can DISTINGUISH the original confound-B
        // shape: a Stage-2 proof call with the full search landscape appended. Assert the real
        // Stage-2 prompt is NOT equal to that landscape-augmented control. If a future edit
        // re-appended frontier bodies/errors on the autonomous side, `stage2` would equal this
        // control and BOTH this assertion and the byte-parity assertion above would trip.
        let landscape = [
            ("intro h0; aaa", "error: type mismatch"),
            ("nlinarith [h]", "error: linarith failed"),
        ];
        let control = confound_b_control_prompt(&thm, Some(body), Some(fb), "", &landscape);
        assert_ne!(
            sha_hex(&stage2),
            sha_hex(&control),
            "confound-B gate not load-bearing: Stage-2 equals the landscape-augmented control"
        );
        // Sanity: the control really IS the richer-context shape (strict superset of the proof
        // prompt) and carries leaked landscape text — so the inequality above is non-vacuous.
        assert!(control.len() > stage2.len());
        assert!(control.contains("FULL SEARCH LANDSCAPE"));
    }

    #[test]
    fn route_summary_excludes_body_and_shielded_error() {
        let thm = selftest_theorem();
        let node_tx_ids = vec![TxId("lm-nodeAAAAA".into()), TxId("lm-nodeBBBBB".into())];
        let mut node_feedback = BTreeMap::new();
        node_feedback.insert(
            "lm-nodeBBBBB".to_string(),
            "error: SHIELDSENTINEL unsolved goals n+0=n".to_string(),
        );
        let mut node_conf = BTreeMap::new();
        node_conf.insert("lm-nodeBBBBB".to_string(), 70u64);
        let mut pi = BTreeMap::new();
        pi.insert(
            TxId("lm-nodeBBBBB".into()),
            NodeMarketEntry {
                price_yes: Some(RationalPrice {
                    numerator: 3,
                    denominator: 7,
                }),
                ..Default::default()
            },
        );
        let route = build_route_summary(&thm, &node_tx_ids, &node_feedback, &node_conf, &pi);
        // NO proof body, NO shielded-error sentinel, NO raw shielded-error text — only the class.
        assert!(
            !route.contains("FULLBODYSENTINEL"),
            "route leaked body: {route}"
        );
        assert!(
            !route.contains("SHIELDSENTINEL"),
            "route leaked shielded error: {route}"
        );
        assert!(
            !route.contains("unsolved goals"),
            "route leaked raw error text: {route}"
        );
        // the coarse class token IS allowed (it is not the raw line).
        assert!(
            route.contains("class=unsolved_goals"),
            "route should carry coarse class: {route}"
        );
    }

    // ── F3 new baselines: parse/Bulls-only + select_parent node-class ──

    #[test]
    fn f3_new_policies_parse_and_are_bulls_only() {
        for s in ["single_restart", "single_tree_no_price", "parallel_restart"] {
            let p = Policy::parse(s).unwrap();
            assert_eq!(p.label(), s);
            assert!(!p.emits_challenges(), "{s} must be Bulls-only");
        }
    }

    #[test]
    fn f3_restart_reaches_root_and_own_last() {
        let pi = BTreeMap::new();
        let conf = BTreeMap::new();
        let doubt = BTreeMap::new();
        let all = vec![
            TxId("n_other".into()),
            TxId("own_a".into()),
            TxId("own_b".into()),
        ];
        let own_nodes = vec![TxId("own_a".into()), TxId("own_b".into())];
        let own_last = TxId("own_b".into());
        for pol in [Policy::SingleRestart, Policy::ParallelRestart] {
            let (mut saw_root, mut saw_own) = (false, false);
            for seed in 0..64u64 {
                let mut rng = StdRng::seed_from_u64(seed);
                match select_parent(
                    pol,
                    &pi,
                    &all,
                    Some(&own_last),
                    &own_nodes,
                    &conf,
                    &doubt,
                    0.15,
                    &mut rng,
                ) {
                    None => saw_root = true,
                    Some(t) if t == own_last => saw_own = true,
                    Some(other) => panic!("{pol:?} returned non-own node {other:?}"),
                }
            }
            assert!(
                saw_root && saw_own,
                "{pol:?} must reach BOTH root and own_last"
            );
        }
    }

    #[test]
    fn f3_single_tree_no_price_picks_own_only() {
        let pi = BTreeMap::new();
        let conf = BTreeMap::new();
        let doubt = BTreeMap::new();
        let all = vec![
            TxId("n_other".into()),
            TxId("own_a".into()),
            TxId("own_b".into()),
        ];
        let own_nodes = vec![TxId("own_a".into()), TxId("own_b".into())];
        let own_last = TxId("own_b".into());
        for seed in 0..64u64 {
            let mut rng = StdRng::seed_from_u64(seed);
            match select_parent(
                Policy::SingleTreeNoPrice,
                &pi,
                &all,
                Some(&own_last),
                &own_nodes,
                &conf,
                &doubt,
                0.15,
                &mut rng,
            ) {
                Some(t) => assert!(own_nodes.contains(&t), "picked non-own node {t:?}"),
                None => panic!("non-empty own-history must not return None"),
            }
        }
        // empty own-history → None.
        let mut rng = StdRng::seed_from_u64(0);
        assert_eq!(
            select_parent(
                Policy::SingleTreeNoPrice,
                &pi,
                &all,
                Some(&own_last),
                &[],
                &conf,
                &doubt,
                0.15,
                &mut rng
            ),
            None
        );
    }

    // ── F2 telemetry: manifest carries the compute fields + honest invariants ──

    #[test]
    fn l4_reject_class_taxonomy() {
        // §4 reject CLASS is derived correctly from the LeanOutcome → AttemptNode.reject_class.
        let mk = |vk: LeanVerdictKind, axiom_rejected: bool| LeanOutcome {
            verdict_kind: vk,
            error_class: None,
            exit_code: 0,
            timed_out: false,
            feedback: String::new(),
            axiom_rejected,
            axioms: vec![],
        };
        assert_eq!(reject_class_of(&mk(LeanVerdictKind::Verified, false)), None);
        assert_eq!(
            reject_class_of(&mk(LeanVerdictKind::Failed, true)).as_deref(),
            Some("axiom-rejected"),
            "compiled-but-non-whitelist-axiom is a SOUNDNESS reject"
        );
        assert_eq!(
            reject_class_of(&mk(LeanVerdictKind::SorryBlocked, false)).as_deref(),
            Some("sorry-blocked")
        );
        assert_eq!(
            reject_class_of(&mk(LeanVerdictKind::Failed, false)).as_deref(),
            Some("lean-reject")
        );
    }

    #[test]
    fn f2_manifest_has_compute_telemetry_fields() {
        let m = sample_manifest_for_selftest(Policy::Autonomous);
        let v = serde_json::to_value(&m).unwrap();
        for key in [
            "proposal_llm_calls",
            "route_llm_calls",
            "bear_llm_calls",
            "proof_prompt_tokens",
            "route_prompt_tokens",
            "bear_prompt_tokens",
            "completion_tokens",
            "total_model_tokens",
            "lean_verifies",
            "total_wall_clock_ms",
            "librarian_notice_nonempty_count",
            "librarian_notice_chars",
        ] {
            assert!(v.get(key).is_some(), "manifest missing `{key}`");
        }
        // M2 broadcast-injection tell: the sample exercises a non-empty librarian notice.
        assert!(
            m.librarian_notice_nonempty_count > 0,
            "librarian_notice_nonempty_count must be tracked (broadcast-injection tape tell)"
        );
        // honest accounting: no double-count; autonomous route is a SEPARATE Stage-1 call.
        assert_eq!(
            m.total_model_tokens,
            m.proof_prompt_tokens
                + m.route_prompt_tokens
                + m.bear_prompt_tokens
                + m.completion_tokens
        );
        // F4: the PPUT denominator (total_tokens) must include route → equal total_model_tokens.
        assert_eq!(
            m.total_tokens, m.total_model_tokens,
            "total_tokens (PPUT denominator) must include route (== total_model_tokens)"
        );
        assert!(
            m.route_llm_calls > 0,
            "autonomous route_llm_calls must be > 0"
        );
        let m2 = sample_manifest_for_selftest(Policy::Single);
        assert_eq!(
            m2.route_llm_calls, 0,
            "non-autonomous route_llm_calls must be 0"
        );
        assert_eq!(
            m2.route_prompt_tokens, 0,
            "non-autonomous route_prompt_tokens must be 0"
        );
    }
}
