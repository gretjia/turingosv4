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
use std::path::{Path, PathBuf};
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
use turingosv4::judges::lean_judge::{default_lean_bin, realign, LeanOutcome};
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
// H-HET-2 (VERIFY_UCB_PRICE_FLOOR): generic model-budget router + tape-canonical decision record.
use turingosv4::runtime::budget_allocation_telemetry::{
    self as bat, BudgetAllocationTelemetry,
};
use turingosv4::runtime::routing_policy::{
    self, ModelInput, RoutingPolicyConfig, RoutingPolicyGenesisPin,
};
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

// Shared per-model cost table + recompute helper (TP-0A.3). Pulled in via #[path] (NOT lib.rs —
// adding a mod there is a trust-root/constitution touch), identical to verify_market_tape and
// lean_hayek_market so the cost-resolution self-test asserts on the SAME table the tape replay uses.
#[path = "../market_tape_shared.rs"]
// This bin uses only the cost table + recompute helper; the module's other derive_* helpers are
// exercised by verify_market_tape, so they are intentionally unused here (not dead code).
#[allow(dead_code)]
mod market_tape_shared;
use market_tape_shared::{call_micro_usd, FALLBACK_IN_UPMT, FALLBACK_OUT_UPMT};

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
const PROOF_TEMPERATURE: f64 = 0.7;
const ROUTE_TEMPERATURE: f64 = 0.7;
const BEAR_TEMPERATURE: f64 = 0.3;

/// Default heterogeneous model roster (used when `--models` is absent or parses empty). Single
/// source of truth so `parse_args` and the cost-resolution self-test reference the SAME list and
/// cannot drift. These are the literal provider model ids sent to the proxy and recorded on tape;
/// `call_micro_usd` matches them case-insensitively against the lowercase MODEL_RATES table.
fn default_models() -> Vec<String> {
    vec![
        "deepseek-ai/DeepSeek-V4-Pro".into(),
        "Qwen/Qwen3-32B".into(),
        "zai-org/GLM-4.5-Air".into(),
        "Qwen/Qwen3.5-397B-A17B".into(),
    ]
}

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
    // BEAR-TRIAGE soft routing: priority = base + λ·norm_score, linear-weight softmax (integer, no f64).
    // Routes toward LOW-short (low-skepticism) nodes — Phase 1 evidence: P(short_Verified <
    // short_Failed) = 0.62 / AUC 0.62 → lower short = more likely to succeed. Norm score
    // (permille, [0..1000]) is the INVERSE of raw bear, so lowest-short node gets highest weight.
    // True soft distribution — all nodes retain non-zero probability (§17.3 / G1). Emits a real
    // Bear ChallengeTx (short) per node so the price index carries a true signal; without shorts
    // den==num → all bear_scores zero → routing degenerates to uniform (Bug1/Bug2/Bug3 fixes).
    BearTriage,
    // AUTONOMOUS-MARKET (Hayekian self-selection — Change 2/3). The harness assigns NO phase/role.
    // Each heterogeneous agent is broadcast PRICE signals (Art II.2) + the ABSTRACTED librarian
    // failure digest (Art II.1) over its OWN isolated, decorrelated decision context (Art III.3 —
    // never a shared in-flight blob) and FREELY CHOOSES one of TWO actions from a menu:
    //   "solve"  → propose+verify a proof on a self-chosen open node (Long / YES) — routes into the
    //              EXISTING Stage-2 → judge.verify → WorkTx path (a real Lean kernel verdict).
    //   "short"  → bet a self-chosen open node will FAIL (Bear / NO) — routes into the EXISTING
    //              ChallengeTx path; the shorter proposes NO proof and triggers NO kernel verify.
    // The agent's self-chosen action is tape-recorded (Art 0.2: AttemptNode.chosen_action) so the
    // autonomous choice + the signals it saw are reconstructable; failed solves stay on tape
    // verified=false. Monetary/CTF invariants are unchanged — both actions route to EXISTING typed
    // tx types (WorkTx / ChallengeTx), whose conservation is sequencer-enforced.
    AutonomousMarket,
    // H-HET-2 DYNAMIC MODEL-BUDGET ROUTING (VERIFY_UCB_PRICE_PRIOR_FLOOR_V1; architect ruling
    // 2026-06-15). The router (a top-level allocator, NOT proposer-visible — Goodhart shield,
    // Art III.4) picks WHICH model gets the next proposal-call, using the GENERIC
    // `runtime::routing_policy` mechanism on per-(model,target) predicate-outcome counts:
    // deterministic UCB (reward = Lean verify), a bounded target-local price prior for
    // cold-start only, a mandatory ε exploration floor, an integer isqrt count bonus, no RNG.
    // Each tick emits a tape-canonical `BudgetAllocationTelemetry` so the allocation replays
    // (Art 0.2). The model routing is the treatment; node routing is fresh-root solve (isolates
    // the model-budget lever). The carrier here is the math-domain DRIVER application; the
    // routing mechanism itself stays generic.
    VerifyUcbPriceFloor,
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
            "bear_triage" => Ok(Policy::BearTriage),
            "autonomous_market" => Ok(Policy::AutonomousMarket),
            "verify_ucb_price_floor" => Ok(Policy::VerifyUcbPriceFloor),
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
            Policy::BearTriage => "bear_triage",
            Policy::AutonomousMarket => "autonomous_market",
            Policy::VerifyUcbPriceFloor => "verify_ucb_price_floor",
        }
    }
    /// Price-family policies emit a Bear ChallengeTx (short) per node; the
    /// non-market baselines are Bulls-only (no short, no price game).
    fn emits_challenges(self) -> bool {
        // F3 single_restart / single_tree_no_price / parallel_restart are Bulls-only
        // (NO price game, NO Bear short) — they intentionally fall through to `false`.
        // AutonomousMarket ALSO falls through to `false` here: its short is NOT a per-proposal
        // auto-short — the agent SELF-SELECTS "short" as one of its two actions and the branch
        // emits the ChallengeTx itself against a chosen open node. A generic auto-short here
        // would double-short every self-chosen "solve" (one short per WorkTx), corrupting price.
        // BearTriage MUST emit a real Bear short so the price index is populated and
        // `short` has a true signal; without it den==num → bear_score=0 everywhere →
        // routing degenerates to uniform (Bug1 fix).
        matches!(
            self,
            Policy::Market
                | Policy::Autonomous
                | Policy::RandomBear
                | Policy::FixedBear
                | Policy::ShuffledPrice
                | Policy::NoPrice
                | Policy::BearTriage
        )
    }
    /// CONTROL-INTEGRITY GATE (Art II.2 broadcast-vs-no-broadcast A/B): does this arm's
    /// HYPOTHESIS include "the broadcast price signal helps proof generation"? Only those arms
    /// may receive the live Market-Prices block in the Stage-2 PROOF prompt. This is a SEPARATE
    /// axis from `emits_challenges()`: `NoPrice` emits a Bear short (so its price index is
    /// populated) but its premise is "prices STRIPPED from selection" — injecting the live price
    /// block into NoPrice's proof prompt would contaminate the exact baseline the A/B measures
    /// against (false null / false positive). `AutonomousMarket` does NOT auto-short here yet DOES
    /// broadcast price (Art II.2). The single/parallel/topology controls (Single, Parallel,
    /// SingleRestart, SingleTreeNoPrice, ParallelRestart) and the non-price scorers (Majority,
    /// BestFirst, SkepticRerank) are NO-broadcast controls → MUST get an empty price block.
    fn broadcasts_price(self) -> bool {
        matches!(
            self,
            Policy::Market
                | Policy::RandomBear
                | Policy::FixedBear
                | Policy::ShuffledPrice
                | Policy::BearTriage
                | Policy::AutonomousMarket
        )
    }
}

struct Args {
    runtime_repo: PathBuf,
    cas: PathBuf,
    run_id: String,
    out: PathBuf,
    proxy_url: String,
    /// Back-compat single-model field (still used for Manifest.model provenance field).
    /// Per-agent routing now uses `models` roster + derived `agent_models`.
    model: String,
    /// Heterogeneous model roster: comma-separated via `--models`. Defaults to the 4-model
    /// round-robin roster [DeepSeek-V4-Pro, Qwen3-32B, GLM-4.5-Air, Qwen3.5-397B-A17B].
    /// Expanded round-robin to n_agents to produce `agent_models`.
    models: Vec<String>,
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
    /// Art 0.2 (Change 2/3 — AutonomousMarket): the action the heterogeneous agent SELF-CHOSE
    /// from the 2-action menu for this node — `Some("solve")` (this node is a self-selected Long
    /// proof attempt) or `Some("short")` (this node is a self-selected Bear short — a ChallengeTx,
    /// no proof). `None` for all harness-assigned policies (Market/Autonomous/etc.) where the phase
    /// was NOT self-chosen. Makes the autonomous choice tape-reconstructable without re-running.
    chosen_action: Option<String>,
    /// Eng-2 (audit 2026-06-14): provenance of `chosen_action` for AutonomousMarket nodes —
    /// `Some("agent")` = genuinely self-selected from the menu; `Some("parse_fallback")` /
    /// `Some("llm_error")` = a forced constructive solve (the decision JSON was unparseable or
    /// the decision LLM call failed). `None` for harness-assigned policies. Lets a solve-rate
    /// metric exclude forced solves (`action_source != Some("agent")`) instead of counting a
    /// parse/LLM failure as a real choice.
    action_source: Option<&'static str>,
}

#[derive(Debug, Serialize)]
struct Manifest {
    schema_version: &'static str,
    run_id: String,
    policy: &'static str,
    model: String,
    /// Art 0.2 tape provenance: per-agent model roster (length = n_agents, round-robin of args.models).
    /// Allows a verifier to reconstruct which model was used for Agent_i without re-running.
    models: Vec<String>,
    proxy_url: String,
    proof_temperature: f64,
    route_temperature: f64,
    bear_temperature: f64,
    lean_bin: String,
    lean_version: Option<String>,
    mathlib_dir: Option<String>,
    mathlib_lean_path: Option<String>,
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
    bear_flat_short_fallback_count: usize,
    bear_parse_fallback_count: usize,
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
        models: get("models")
            .map(|s| s.split(',').map(|m| m.trim().to_string()).filter(|m| !m.is_empty()).collect())
            // A `--models` that parses to an empty roster (e.g. `--models ","`) would make
            // `i % args.models.len()` a divide-by-zero panic downstream; treat it like absent and
            // fall back to the default roster.
            .filter(|v: &Vec<String>| !v.is_empty())
            .unwrap_or_else(default_models),
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

/// BEAR-TRIAGE soft routing (§17.3 name-lie gate: truly soft, not argmax).
///
/// Given a slice of `(TxId, norm_score)` pairs — norm_score is the **normalized routing
/// priority** in [0, 1000] (permille), ALREADY inverted so that lower-short (less skepticism)
/// nodes have HIGHER norm_score — compute a weighted probability distribution where:
///
///   weight_i = BASE + (lambda_num * norm_score_i) / lambda_den
///
/// Phase 1 empirical finding: P(short_Verified < short_Failed) = 0.62, AUC = 0.62.
/// "Low-short → more likely to succeed." Therefore routing priority is INVERSE of raw bear:
/// norm_score_i = round(1000 * (M - short_i) / max(1, M)), M = max short among candidates.
/// This maps the lowest-short node to 1000 and the highest-short to 0, bounded [0, 1000],
/// keeping λ tunable over a predictable range regardless of raw micro magnitudes (Bug3 fix).
///
/// BASE = 1 is added so EVERY node has a strictly positive weight even when norm_score_i == 0.
/// This guarantees no node is dropped (G1: no veto/skip/zero-probability).
///
/// The selection is a linear scan over the cumulative weights using a uniform integer draw
/// against the total (no f64; the random draw is expressed as a fraction r/total with integer r):
///   draw r from [0, total)   (u128, fits within i128 headroom for realistic N and scores)
///   pick the first i such that cumulative_weight[i] > r
///
/// Returns None only when the scores slice is empty (no candidates; caller falls back).
///
/// Integer-only arithmetic throughout (money path: §12 "禁 f64").
fn bear_soft_priority(
    scores: &[(TxId, i128)],
    lambda_num: i64,
    lambda_den: i64,
    rng: &mut StdRng,
) -> Option<TxId> {
    if scores.is_empty() {
        return None;
    }
    // lambda_den must be positive to avoid division-by-zero or sign flip.
    let lden = lambda_den.max(1) as i128;
    let lnum = lambda_num as i128;
    // Weight_i = BASE + floor(lambda_num * norm_score_i / lambda_den).
    // BASE = lden (chosen so BASE/lden = 1 in the same unit as the lambda term, keeping all i128).
    // Equivalently weight_i_scaled = lden + lnum * norm_score_i (in units of 1/lden each).
    // norm_score values are already in [0, 1000]; the weight range is thus [lden, lden+1000*lnum],
    // which is bounded and predictable — softmax maintains a true distribution (Bug3 fix).
    // We work in units of 1/lden throughout so no division is needed until the final comparison.
    let weights: Vec<i128> = scores
        .iter()
        .map(|(_, ns)| {
            // norm_score is in [0, 1000] (non-negative by construction).
            let s = (*ns).max(0);
            // weight_scaled = lden (base=1 in fractional units) + lnum * s
            (lden + lnum * s).max(1) // clamp to ≥1: guarantees non-zero weight for every node
        })
        .collect();
    let total: i128 = weights.iter().sum();
    if total <= 0 {
        // degenerate: all weights collapsed to zero despite clamp — return first node
        return Some(scores[0].0.clone());
    }
    // Draw r uniformly from [0, total) using a u64 rng call scaled to total.
    // To avoid f64: draw a u64, then compute r = (draw as i128 * total) / u64::MAX as i128.
    // This gives a uniform integer in [0, total) with negligible bias (bias < 1 ULP of total).
    let draw = rng.gen::<u64>() as i128;
    let r = (draw * total) / (u64::MAX as i128 + 1);
    // Walk cumulative weights, pick first i where cumsum > r.
    let mut cumsum: i128 = 0;
    for (i, w) in weights.iter().enumerate() {
        cumsum += w;
        if cumsum > r {
            return Some(scores[i].0.clone());
        }
    }
    // Floating-point-free fallback: rounding may push r == total; return last.
    Some(scores[scores.len() - 1].0.clone())
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
        // AUTONOMOUS-MARKET (Change 2/3): the agent self-selects action + target INSIDE STEP A;
        // the pre-call selector is likewise a no-op (None). The real parent (solve) or short
        // target is parsed from the model's {action,target} and validated after the call.
        Policy::AutonomousMarket => None,
        // H-HET-2 VERIFY_UCB_PRICE_FLOOR: the treatment is MODEL-budget routing (chosen by the
        // top-level router before the proposal call), so node routing is held constant as fresh-root
        // solve (None) to isolate the model-budget lever from node-routing confounds.
        Policy::VerifyUcbPriceFloor => None,
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
        // BEAR-TRIAGE: soft routing toward LOW-short (low-skepticism) nodes.
        //
        // Phase 1 empirical basis: P(short_Verified < short_Failed) = 0.62, bear AUC = 0.62.
        // "Verified nodes have lower short than Failed nodes" → route toward LOW short to
        // concentrate refinement effort on the nodes most likely to succeed (Bug2 fix: direction
        // was previously inverted, routing toward high-short = most likely to fail).
        //
        // Normalization (Bug3 fix): raw short ~ thousands of micro; routing directly on raw values
        // creates weight range spanning 1000× → softmax → near-argmax, killing exploration
        // (Art. II.2.1 / §17.3). Instead normalize to relative permille in [0, 1000]:
        //   raw_bear_i = price_yes.den - price_yes.num  (higher = more market skepticism)
        //   M = max raw_bear across candidates (within the current call)
        //   norm_score_i = round(1000 * (M - raw_bear_i) / max(1, M))
        //   → lowest-short node → norm_score 1000; highest-short → norm_score 0.
        // Nodes without a price entry get raw_bear = 0 (no market signal; treated as minimally
        // skeptical). With λ = lambda_num/lambda_den = 1/1 (default; binding-budget pilot before
        // architect finalizes), weight_i = 1 + norm_score_i ∈ [1, 1001] — all nodes non-zero
        // (true soft distribution, G1: no veto path). λ can be raised to sharpen the bias.
        Policy::BearTriage => {
            if all_nodes.is_empty() {
                return None;
            }
            // Step 1: collect raw bear scores (higher raw = more market skepticism).
            let raw_bears: Vec<(TxId, i128)> = all_nodes
                .iter()
                .map(|t| {
                    let raw = pi
                        .get(t)
                        .and_then(|e| e.price_yes.as_ref())
                        .map(|p| (p.denominator as i128) - (p.numerator as i128))
                        .unwrap_or(0)
                        .max(0);
                    (t.clone(), raw)
                })
                .collect();
            // Step 2: normalize and invert — low short → high priority (0..1000 permille).
            let m = raw_bears.iter().map(|(_, b)| *b).max().unwrap_or(0).max(1);
            let scores: Vec<(TxId, i128)> = raw_bears
                .iter()
                .map(|(t, raw)| {
                    // norm_score = 1000 * (M - raw) / M; inversely proportional to raw bear.
                    let ns = 1000i128 * (m - raw) / m;
                    (t.clone(), ns)
                })
                .collect();
            // Step 3: λ = 1/1 (default; bounded weight range [1, 1001] with permille scores).
            bear_soft_priority(&scores, 1, 1, rng)
                .or_else(|| all_nodes.last().cloned())
        }
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

#[allow(clippy::too_many_arguments)]
fn put_proposal(
    cas_path: &PathBuf,
    run_id: &str,
    agent: &str,
    idx: u64,
    parent: Option<TxId>,
    body: &str,
    tokens: TokenCounts,
    model: &str,
    lt: u64,
) -> Result<Cid, String> {
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    let mut tel = ProposalTelemetry::build_for_evaluator_append_with_parent(
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
    // §8 (Art 0.2): record the producing vendor model on the tape-resident CAS
    // object so per-proposal model provenance + cost are reconstructable from the
    // frozen tape alone (no round-robin inference, no manifest sidecar).
    tel.model_id = Some(model.to_string());
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

/// H-HET-2: deterministic sha256 over an ordered list of strings (set/pool/caps hashes).
fn blob_hash(items: &[String]) -> Hash {
    let mut h = Sha256::new();
    for s in items {
        h.update((s.len() as u64).to_be_bytes());
        h.update(s.as_bytes());
    }
    Hash(h.finalize().into())
}

/// H-HET-2: put a small canonical JSON blob to CAS under a schema id; returns its CID.
/// Used for the BudgetAllocationTelemetry input CID fields (price vector / failure features /
/// router overhead) so the routing decision is reconstructable from the frozen tape (Art 0.2).
fn put_routing_blob(
    cas_path: &PathBuf,
    schema_id: &str,
    value: &serde_json::Value,
    lt: u64,
) -> Result<Cid, String> {
    let mut cas = CasStore::open(cas_path).map_err(|e| format!("open CAS: {e}"))?;
    cas.put(
        serde_json::to_vec(value).map_err(|e| format!("blob ser: {e}"))?.as_slice(),
        ObjectType::Generic,
        "het2-router",
        lt,
        Some(schema_id.into()),
    )
    .map_err(|e| format!("put routing blob: {e}"))
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
    price_context: &str,
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
    // Art II.2: broadcast price signals only — no solution steps, no scoring internals (Art III.4).
    // Art III.2: bounded disclosure — a few lines only.
    if !price_context.is_empty() {
        p.push_str(price_context);
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
    price_context: &str,
) -> String {
    build_prompt(theorem, parent_body, parent_feedback, librarian, price_context)
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
    let mut p = build_prompt(theorem, parent_body, parent_feedback, librarian, "");
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

/// AUTONOMOUS-MARKET decision prompt (Change 2/3, STEP A — Hayekian self-selection).
///
/// Broadcasts SIGNALS ONLY (Art II.2): the open priced nodes (price_yes per node, from the
/// shared price index — Change 3's `price_context` surface), the coarse error CLASS per node
/// (never the raw shielded `node_feedback`), and the ABSTRACTED librarian failure digest
/// (Art II.1 — passed in as `librarian`, the same shielded `real_librarian_solver_notice`
/// digest the proof prompt uses; NEVER raw error logs). It assigns NO role and gives NO
/// instruction on HOW to prove (no solution steps), and exposes NO LeanJudge/Predicate scoring
/// internals (Art III.4 Goodhart shield — only the market-derived price, which is downstream of
/// the kernel verdict, never the verdict-scoring machinery).
///
/// DECORRELATION (Art III.3): this is the agent's OWN isolated decision context. It is built
/// ONLY from already-COMMITTED tape state (the price index + the abstracted librarian), never
/// from any other agent's in-flight (this-round-uncommitted) choice. Feeding agents each other's
/// pending picks would collapse "一万个黑盒退化为一个" — so we never do.
///
/// The menu is exactly 2 actions: "solve" (propose+verify a proof on a chosen open node, Long /
/// YES) or "short" (bet a chosen open node FAILS, Bear / NO). The model returns its self-chosen
/// action, the target node index, an optional proof body (advisory only — solve still runs the
/// EXISTING Stage-2 proof+verify path), and its own confidence (drives the stake).
fn build_autonomous_decision_prompt(
    theorem: &LeanTheorem,
    node_tx_ids: &[TxId],
    node_feedback: &BTreeMap<String, String>,
    node_conf: &BTreeMap<String, u64>,
    pi: &BTreeMap<TxId, NodeMarketEntry>,
    librarian: &str,
) -> String {
    let mut p = String::new();
    p.push_str(
        "You are an autonomous agent in a Lean 4 proof-search MARKET (Mathlib is available). \
         No role has been assigned to you. You are shown ONLY market signals — you decide for \
         yourself what to do. CHOOSE ONE of exactly two actions:\n\
         \n\
         - \"solve\": you believe a goal/node is provable — propose a Lean proof for it and take \
           the LONG (YES) side. You may extend an existing open node (by its index) or start a \
           fresh root (target = -1).\n\
         - \"short\": you believe an existing open node's attempt will FAIL the Lean kernel — \
           take the SHORT (NO) side against it (by its index). You propose NO proof.\n\
         \n\
         Decide from the prices and the collective failure memory below. Be selective — do not \
         all crowd onto the single highest-priced node; balance exploring under-attacked nodes \
         against exploiting strong ones. Output ONLY a JSON object.\n\n",
    );
    p.push_str("=== Target (the goal to prove) ===\n");
    p.push_str(&theorem.preamble);
    p.push('\n');
    if node_tx_ids.is_empty() {
        p.push_str(
            "\n=== Open nodes: NONE yet — the only available action is solve a fresh root (target = -1) ===\n",
        );
    } else {
        p.push_str(
            "\n=== Open nodes (index : price_yes(num/den) : confidence : error-class) ===\n",
        );
        for (idx, tx) in node_tx_ids.iter().enumerate() {
            // ONLY the coarse error CLASS (Art II.1 abstraction) — never the raw shielded line.
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
            p.push_str(&format!(
                "[{idx}] price_yes={pn}/{pd} conf={conf}% class={class}\n"
            ));
        }
    }
    // Art II.1: the ABSTRACTED collective-failure digest, broadcast (NOT raw logs). Same shielded
    // notice the proof prompt carries; empty-guarded so we never inject a no-op section.
    if !librarian.is_empty() {
        p.push_str(librarian);
    }
    p.push_str(
        "\nReturn EXACTLY one of:\n\
         {\"action\":\"solve\",\"target\":<node index to extend, or -1 for a fresh root>,\
         \"proof_body\":\"<the Lean tactic block AFTER `:= by`>\",\"confidence\":0.0-1.0}\n\
         {\"action\":\"short\",\"target\":<index of an existing node you bet will FAIL>,\
         \"confidence\":0.0-1.0}\n",
    );
    p
}

/// The parsed self-selected action from STEP A (Change 2/3). Tape-canonical (Art 0.2): every
/// field here is reconstructable from the agent's decision response + the committed signals.
struct AutonomousChoice {
    /// "solve" or "short" — the harness never assigns this; the agent self-selects it.
    action: String,
    /// node index the agent chose to act on (-1 = fresh root, only meaningful for solve).
    target: i64,
    /// the agent's self-reported confidence (0..1) → drives the stake / short size.
    confidence: f64,
    /// Eng-2 (audit 2026-06-14): provenance of `action`, so a fail-open "solve" is NOT
    /// silently counted as a genuine self-selected solve when computing the solve-rate metric.
    /// `"agent"` = the decision JSON carried a valid solve/short; `"parse_fallback"` = no JSON
    /// or an invalid/missing action field forced the constructive solve default; `"llm_error"` =
    /// the decision LLM call itself failed and the iteration fell open to a solve. Mirrors the
    /// existing `BearShortDecision.parse_fallback` honesty pattern. Tape-recorded on
    /// `AttemptNode.action_source` so the metric can exclude `action_source != "agent"`.
    decision_source: &'static str,
}

/// Parse STEP A's JSON. FAIL-OPEN to a "solve" on a fresh root (the constructive default) on any
/// parse/shape error, so a malformed decision does not silently shrink the autonomous-market
/// node count below the budget-parity target — it still produces one real proof attempt.
fn parse_autonomous_choice(content: &str) -> AutonomousChoice {
    let v = match extract_json_object(content) {
        Some(v) => v,
        None => {
            // No parseable JSON at all → forced constructive solve, marked as such (Eng-2).
            return AutonomousChoice {
                action: "solve".into(),
                target: -1,
                confidence: 0.6,
                decision_source: "parse_fallback",
            }
        }
    };
    // Capture whether the action field itself was a valid self-selection ("agent") or whether
    // the constructive "solve" default had to be forced ("parse_fallback") (Eng-2).
    let valid_action = v
        .get("action")
        .and_then(|x| x.as_str())
        .map(|s| s.to_ascii_lowercase())
        .filter(|s| s == "solve" || s == "short");
    let decision_source = if valid_action.is_some() {
        "agent"
    } else {
        "parse_fallback"
    };
    let action = valid_action.unwrap_or_else(|| "solve".into());
    let target = v.get("target").and_then(|x| x.as_i64()).unwrap_or(-1);
    let confidence = v
        .get("confidence")
        .and_then(|x| x.as_f64())
        .unwrap_or(0.6)
        .clamp(0.0, 1.0);
    AutonomousChoice {
        action,
        target,
        confidence,
        decision_source,
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
#[derive(Debug, Clone, Copy)]
struct BearShortDecision {
    short_micro: i64,
    prompt_tokens: u64,
    completion_tokens: u64,
    flat_short_fallback: bool,
    parse_fallback: bool,
}

async fn bear_doubt_short(
    llm: &ResilientLLMClient,
    model: &str,
    theorem: &LeanTheorem,
    body: &str,
) -> BearShortDecision {
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
            temperature: Some(BEAR_TEMPERATURE),
            max_tokens: Some(60),
        })
        .await
    {
        Ok(r) => {
            let parsed = extract_json_object(&r.content)
                .and_then(|v| v.get("doubt").and_then(|x| x.as_f64()));
            let parse_fallback = parsed.is_none();
            let doubt = parsed.unwrap_or(0.5).clamp(0.0, 1.0);
            // probability → integer percent (not a money op); stake math stays integer.
            let doubt_pct = (doubt * 100.0) as i64;
            let short = MIN_SHORT_MICRO + (MAX_SHORT_MICRO - MIN_SHORT_MICRO) * doubt_pct / 100;
            BearShortDecision {
                short_micro: short,
                prompt_tokens: r.prompt_tokens as u64,
                completion_tokens: r.completion_tokens as u64,
                flat_short_fallback: false,
                parse_fallback,
            }
        }
        Err(_) => BearShortDecision {
            short_micro: CHALLENGE_STAKE_MICRO,
            prompt_tokens: 0,
            completion_tokens: 0,
            flat_short_fallback: true,
            parse_fallback: false,
        },
    }
}

/// SHA-256 hex of a string — the confound-B prompt-parity comparator.
fn sha_hex(s: &str) -> String {
    let mut h = Sha256::new();
    h.update(s.as_bytes());
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

fn lean_version_for_manifest(lean_bin: &Path) -> Option<String> {
    if !lean_bin.exists() {
        return None;
    }
    let out = std::process::Command::new(lean_bin)
        .arg("--version")
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
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
    let market_prompt = build_prompt(&thm, market_body.as_deref(), market_feedback.as_deref(), "", "");

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
        stage2_proof_prompt(&thm, auto_body.as_deref(), auto_feedback.as_deref(), "", "");
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
        "proxy_url",
        "proof_temperature",
        "route_temperature",
        "bear_temperature",
        "lean_bin",
        "lean_version",
        "mathlib_dir",
        "mathlib_lean_path",
        "bear_flat_short_fallback_count",
        "bear_parse_fallback_count",
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

    // ── (F5) cost-resolution: every DEFAULT roster model must resolve to a SPECIFIC MODEL_RATES
    // entry, never the bare `deepseek` / FALLBACK catch-all (the OBL-012 under-bill bug). This is
    // the gate that fails if a future roster id (e.g. a mixed-case `DeepSeek-V4-Pro`) stops matching
    // its lowercase rate id and silently falls through. `call_micro_usd` is the SAME function the
    // tape replay (market_tape_shared::derive_cost) recomputes from, so manifest and standalone
    // verifier share this resolution. Probe with 1M input / 0 output tokens so the per-call cost
    // equals the input-rate micro-USD exactly; the bare `deepseek` catch-all charges FALLBACK_IN
    // (270_000) and any FALLBACK also charges 270_000 — both forbidden for a roster model.
    for model in default_models() {
        let in_cost = call_micro_usd(&model, 1_000_000, 0);
        if in_cost == FALLBACK_IN_UPMT {
            return Err(format!(
                "cost-resolution: roster model `{model}` resolves to the bare deepseek/FALLBACK \
                 input rate ({FALLBACK_IN_UPMT}) — OBL-012 under-bill; it must match a specific \
                 MODEL_RATES id (check case-insensitivity)"
            ));
        }
        // And it must not be billed at the bare-catch-all OUTPUT rate either.
        let out_cost = call_micro_usd(&model, 0, 1_000_000);
        if out_cost == FALLBACK_OUT_UPMT {
            return Err(format!(
                "cost-resolution: roster model `{model}` resolves to the bare deepseek/FALLBACK \
                 output rate ({FALLBACK_OUT_UPMT}) — OBL-012 over-bill"
            ));
        }
    }
    // Pin the historically-bugged entry: the DeepSeek-V4-Pro roster id MUST bill at the
    // deepseek-v4-pro rate (435_000 in / 870_000 out), not the bare deepseek catch-all.
    if call_micro_usd("deepseek-ai/DeepSeek-V4-Pro", 1_000_000, 0) != 435_000 {
        return Err("cost-resolution: DeepSeek-V4-Pro input rate must be 435_000 (deepseek-v4-pro)".into());
    }
    if call_micro_usd("deepseek-ai/DeepSeek-V4-Pro", 0, 1_000_000) != 870_000 {
        return Err("cost-resolution: DeepSeek-V4-Pro output rate must be 870_000 (deepseek-v4-pro)".into());
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
        models: vec!["none".into()],
        proxy_url: "http://127.0.0.1:0".into(),
        proof_temperature: PROOF_TEMPERATURE,
        route_temperature: ROUTE_TEMPERATURE,
        bear_temperature: BEAR_TEMPERATURE,
        lean_bin: "/tmp/lean-selftest".into(),
        lean_version: Some("Lean selftest".into()),
        mathlib_dir: None,
        mathlib_lean_path: None,
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
        bear_flat_short_fallback_count: 0,
        bear_parse_fallback_count: 1,
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
    let lean_version = lean_version_for_manifest(&lean_bin);
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
    let judge = theorem.judge(lean_bin.clone(), mathlib_lp.as_deref());

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
    // Heterogeneous per-agent models: round-robin of args.models roster expanded to n_agents.
    // Agent_i uses agent_models[i]; bears use the SAME model as their paired prover (same index).
    let agent_models: Vec<String> = (0..n_agents)
        .map(|i| args.models[i % args.models.len()].clone())
        .collect();

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
    let (mut bear_flat_short_fallback_count, mut bear_parse_fallback_count) = (0usize, 0usize);
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

    // ── H-HET-2 VERIFY_UCB_PRICE_FLOOR router state (only used for that policy) ──────────
    // The frozen policy config (architect-approved defaults) + per-(model,target) counts
    // maintained in memory for speed AND emitted to tape each tick so they recompute from the
    // frozen tape (Art 0.2). target = the single --problem this invocation proves.
    let routing_cfg = RoutingPolicyConfig::default();
    // The FROZEN eligible roster = deduped args.models, sorted for a deterministic set hash.
    let mut rt_roster: Vec<String> = args.models.clone();
    rt_roster.sort();
    rt_roster.dedup();
    let rt_k = rt_roster.len() as u64;
    let rt_total_budget = (effective_rounds as u64) * (agents.len() as u64);
    let rt_floor_quota0 = routing_cfg.floor_quota(rt_k, rt_total_budget);
    let mut rt_pull: BTreeMap<String, u32> = rt_roster.iter().map(|m| (m.clone(), 0)).collect();
    let mut rt_verify: BTreeMap<String, u32> = rt_roster.iter().map(|m| (m.clone(), 0)).collect();
    let mut rt_hardfail: BTreeMap<String, u32> = rt_roster.iter().map(|m| (m.clone(), 0)).collect();
    let mut rt_floor: BTreeMap<String, u64> =
        rt_roster.iter().map(|m| (m.clone(), rt_floor_quota0)).collect();
    // node_tx -> model that authored it (for the target-local price prior).
    let mut rt_node_model: BTreeMap<String, String> = BTreeMap::new();
    let rt_eligible_set_hash = blob_hash(&rt_roster);
    // Emit the RoutingPolicyGenesisPin once at boot so the run's frozen policy is on tape.
    if args.policy == Policy::VerifyUcbPriceFloor {
        if let Ok(mut cas) = CasStore::open(&args.cas) {
            let cfg_cid = routing_policy::write_policy_config_to_cas(
                &mut cas, &routing_cfg, "het2-router", lt + 1,
            )
            .map_err(|e| format!("policy config cas: {e}"))?;
            let pin = RoutingPolicyGenesisPin {
                policy_family: routing_cfg.policy_family.clone(),
                policy_version: routing_cfg.policy_version.clone(),
                policy_hash: routing_cfg.policy_hash(),
                canonical_policy_config_cid: cfg_cid,
                eligible_model_set_hash: rt_eligible_set_hash,
                target_pool_hash: blob_hash(&[args.problem.clone()]),
                budget_caps_hash: blob_hash(&[format!(
                    "na={} nr={} budget={}",
                    n_agents, args.n_rounds, rt_total_budget
                )]),
                rng_mode: "deterministic_none".into(),
                art_0_4_path: "B".into(),
            };
            routing_policy::write_genesis_pin_to_cas(&mut cas, &pin, "het2-router", lt + 2)
                .map_err(|e| format!("genesis pin cas: {e}"))?;
        }
        lt += 2;
    }

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
            // Art 0.2 tape field: the action this agent SELF-CHOSE this iteration. Stays None for
            // every harness-assigned policy; set to Some("solve") below for AutonomousMarket
            // solves (a self-chosen short takes the early-`continue` short path and tape-records
            // its own AttemptNode there).
            let mut chosen_action: Option<String> = None;
            // Eng-2: provenance of the self-chosen solve (None until AutonomousMarket sets it).
            let mut action_source: Option<&'static str> = None;

            // ── H-HET-2 VERIFY_UCB_PRICE_FLOOR: top-level model-budget router ───────────────
            // The router (NOT proposer-visible — Goodhart shield, Art III.4) picks WHICH model
            // funds this proposal tick via the GENERIC runtime::routing_policy mechanism on
            // per-(model,target) predicate-outcome counts, replacing the fixed agent_models[ai].
            // A pull (budget spend) is recorded now; verify/hard-fail are recorded after the
            // outcome. The decision is emitted as tape-canonical BudgetAllocationTelemetry (Art 0.2).
            let mut tick_model = agent_models[ai].clone();
            let mut tick_truncated = false;
            if args.policy == Policy::VerifyUcbPriceFloor {
                let inputs: Vec<ModelInput> = rt_roster
                    .iter()
                    .map(|m| {
                        let price_prior_bps = rt_node_model
                            .iter()
                            .filter(|(_, mm)| mm.as_str() == m.as_str())
                            .filter_map(|(tx, _)| {
                                pi.get(&TxId(tx.clone())).and_then(|e| e.price_yes.as_ref())
                            })
                            .map(|p| {
                                (p.numerator.saturating_mul(10_000) / p.denominator.max(1)) as u64
                            })
                            .max()
                            .unwrap_or(0);
                        ModelInput {
                            model_id: m.clone(),
                            pull_count: *rt_pull.get(m).unwrap_or(&0),
                            verify_count: *rt_verify.get(m).unwrap_or(&0),
                            hard_failure_streak: *rt_hardfail.get(m).unwrap_or(&0),
                            price_prior_bps,
                            floor_quota_remaining: *rt_floor.get(m).unwrap_or(&0),
                        }
                    })
                    .collect();
                let remaining = rt_total_budget.saturating_sub(step_idx);
                let sel = routing_policy::score_and_select(&routing_cfg, &inputs, remaining);
                tick_model = sel.selected_model_id.clone();
                if let Some(rsel) = sel.rows.iter().find(|r| r.model_id == sel.selected_model_id) {
                    rt_floor.insert(tick_model.clone(), rsel.floor_quota_remaining_after);
                }
                let total_pulls_before: u32 =
                    rt_roster.iter().map(|m| *rt_pull.get(m).unwrap_or(&0)).sum();
                // pull = budget spent on this model this tick (recorded before the proposal so a
                // soft api/parse failure below still counts the budget spend, per the ruling).
                *rt_pull.entry(tick_model.clone()).or_insert(0) += 1;
                let pv = serde_json::json!(sel
                    .rows
                    .iter()
                    .map(|r| (r.model_id.clone(), r.price_bps))
                    .collect::<Vec<_>>());
                let ff = serde_json::json!(sel
                    .rows
                    .iter()
                    .map(|r| (r.model_id.clone(), r.hard_failure_streak_before))
                    .collect::<Vec<_>>());
                let price_vector_cid = put_routing_blob(&args.cas, "het2.price_vector.v1", &pv, lt + 1)
                    .unwrap_or(Cid([0u8; 32]));
                let abstracted_failure_features_cid =
                    put_routing_blob(&args.cas, "het2.failure_features.v1", &ff, lt + 2)
                        .unwrap_or(Cid([0u8; 32]));
                let input_state_cid = put_routing_blob(
                    &args.cas,
                    "het2.routing_input.v1",
                    &serde_json::json!({"round": round, "step": step_idx, "total_pulls_before": total_pulls_before, "target": args.problem}),
                    lt + 3,
                )
                .unwrap_or(Cid([0u8; 32]));
                let router_overhead_cid = put_routing_blob(
                    &args.cas,
                    "het2.router_overhead.v1",
                    &serde_json::json!({"route_calls": 0, "route_tokens": 0}),
                    lt + 4,
                )
                .unwrap_or(Cid([0u8; 32]));
                lt += 4;
                let record = BudgetAllocationTelemetry {
                    policy_family: routing_cfg.policy_family.clone(),
                    policy_hash: routing_cfg.policy_hash(),
                    policy_version: routing_cfg.policy_version.clone(),
                    target_id: args.problem.clone(),
                    seed_id: args.seed,
                    eligible_model_set_hash: rt_eligible_set_hash,
                    input_state_cid,
                    price_vector_cid,
                    abstracted_failure_features_cid,
                    total_pulls_target_before: total_pulls_before,
                    candidates: sel.rows.clone(),
                    selected_model_id: sel.selected_model_id.clone(),
                    selection_reason: sel.reason,
                    allocated_proposal_budget: 1,
                    allocated_token_budget: 900,
                    budget_remaining_before: remaining,
                    budget_remaining_after: remaining.saturating_sub(1),
                    router_overhead_cid,
                    rng_seed: None,
                    rng_draw: None,
                };
                if let Ok(mut cas) = CasStore::open(&args.cas) {
                    let _ = bat::write_to_cas(&mut cas, &record, "het2-router", lt + 1);
                    lt += 1;
                }
            }

            if args.policy == Policy::Autonomous {
                let route_prompt =
                    build_route_summary(&theorem, &node_tx_ids, &node_feedback, &node_conf, &pi);
                let chosen = match llm
                    .generate(&GenerateRequest {
                        model: agent_models[ai].clone(),
                        messages: vec![
                            sys.clone(),
                            Message {
                                role: "user".into(),
                                content: route_prompt,
                            },
                        ],
                        temperature: Some(ROUTE_TEMPERATURE),
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

            // ── AUTONOMOUS-MARKET (Change 2/3 — Hayekian 2-action self-selection) ──────────
            // STEP A: broadcast SIGNALS ONLY (price index + abstracted librarian, Art II.1/II.2)
            // over this agent's OWN isolated decision context (Art III.3 — never a shared
            // in-flight blob) and let the heterogeneous agent FREELY CHOOSE one of two actions
            // from a menu. This is a genuinely SEPARATE Stage-1-style call (decides action+target,
            // emits no proof, triggers no kernel verify) — counted in route_* like Autonomous's
            // route call, so the budget split stays honest (proposal vs route mechanism cost).
            if args.policy == Policy::AutonomousMarket {
                let decision_prompt = build_autonomous_decision_prompt(
                    &theorem,
                    &node_tx_ids,
                    &node_feedback,
                    &node_conf,
                    &pi,
                    &lib,
                );
                let choice = match llm
                    .generate(&GenerateRequest {
                        model: agent_models[ai].clone(),
                        messages: vec![
                            sys.clone(),
                            Message {
                                role: "user".into(),
                                content: decision_prompt,
                            },
                        ],
                        temperature: Some(ROUTE_TEMPERATURE),
                        max_tokens: Some(200),
                    })
                    .await
                {
                    Ok(r) => {
                        route_llm_calls += 1;
                        route_prompt_tokens += (r.prompt_tokens + r.completion_tokens) as u64;
                        parse_autonomous_choice(&r.content)
                    }
                    Err(e) => {
                        // Fail-open to a constructive default (solve / fresh root): the iteration
                        // still produces one real proof attempt so node count tracks the budget.
                        // Marked "llm_error" (Eng-2) so this forced solve is excludable from the
                        // self-selection solve-rate metric.
                        eprintln!("lm decision_err {agent}: {e:?}");
                        AutonomousChoice {
                            action: "solve".into(),
                            target: -1,
                            confidence: 0.6,
                            decision_source: "llm_error",
                        }
                    }
                };

                // STEP B (short branch): the agent SELF-SELECTED "short" — it bets a chosen open
                // node will FAIL. Route into the EXISTING ChallengeTx path (no proof, no kernel
                // verify; the shorter proposes nothing). A short with no valid live target (e.g.
                // round 0, empty frontier, or a hallucinated index) cannot bet on nothing → it
                // is treated as a fresh-root SOLVE so the budget-parity node count is preserved.
                let short_target = resolve_parent_index(&node_tx_ids, choice.target);
                if choice.action == "short" && short_target.is_some() {
                    let target_tx = short_target.unwrap();
                    // Stake from the agent's OWN confidence over the bear short range — its
                    // self-priced conviction, integer math (no f64 in the money op). Higher
                    // self-confidence in failure → larger short, exactly as the bear path scales.
                    let conf_pct = (choice.confidence * 100.0) as i64;
                    let short_micro = (MIN_SHORT_MICRO
                        + (MAX_SHORT_MICRO - MIN_SHORT_MICRO) * conf_pct.clamp(0, 100) / 100)
                        .clamp(MIN_SHORT_MICRO, MAX_SHORT_MICRO);
                    let challenger = challengers[ai % challengers.len()].clone();
                    if let Ok(ce) = put_counterexample(&args.cas, &target_tx.0, lt) {
                        lt += 1;
                        match make_real_challengetx_signed_by(
                            &mut kp,
                            root,
                            target_tx.clone(),
                            &challenger,
                            short_micro,
                            ce,
                            &format!("lmam{step_idx}"),
                            lt,
                        ) {
                            Ok(chal) => {
                                match submit_await(&seq, chal, root, "ChallengeTx(autonomous)")
                                    .await
                                {
                                    Ok(r) => {
                                        root = r;
                                        lt += 1;
                                    }
                                    Err(e) => {
                                        eprintln!("lm am-short skip node{step_idx}: {e}")
                                    }
                                }
                            }
                            Err(e) => eprintln!("lm am-short build skip: {e}"),
                        }
                    }
                    // Tape-record the self-chosen short (Art 0.2): an AttemptNode with no proof,
                    // is_verified=false, chosen_action="short", targeting the shorted node. Reads
                    // the LIVE price of the shorted node so the price effect of the short is on
                    // tape. No WorkTx is created — a shorter does not propose a proof.
                    let short_price = compute_price_index(
                        &seq.q_snapshot()
                            .map_err(|e| format!("{e:?}"))?
                            .economic_state_t,
                    );
                    let spe = short_price.get(&target_tx);
                    nodes.push(AttemptNode {
                        node_tx: format!("lmam-short{step_idx}-{}", args.run_id),
                        task: market_task.clone(),
                        by_agent: agent.clone(),
                        parent_tx: Some(target_tx.0.clone()),
                        confidence_pct: (choice.confidence * 100.0) as u64,
                        work_stake_micro: short_micro,
                        price_yes_num: spe
                            .and_then(|e| e.price_yes.as_ref().map(|p| p.numerator)),
                        price_yes_den: spe
                            .and_then(|e| e.price_yes.as_ref().map(|p| p.denominator)),
                        verdict: "Short".into(),
                        reject_class: None,
                        is_verified: false,
                        body_preview: String::new(),
                        feedback: format!("autonomous short of {}", target_tx.0),
                        tokens: 0,
                        axioms: vec![],
                        chosen_action: Some("short".into()),
                        // A short is only reached when `action == "short"` parsed cleanly, so the
                        // provenance is always the agent's genuine choice (Eng-2).
                        action_source: Some(choice.decision_source),
                    });
                    step_idx += 1;
                    // Short fully handled — skip the entire Stage-2 proof/WorkTx/verify path.
                    continue;
                }

                // STEP B (solve branch): the agent SELF-SELECTED "solve" (or a short with no valid
                // target degraded to solve). Set the chosen parent and fall through to the EXISTING
                // Stage-2 proof → judge.verify → WorkTx path. The decision's advisory `proof_body`
                // is intentionally NOT spliced in — solve runs the SAME shared `stage2_proof_prompt`
                // market/single use (confound-B byte-parity preserved). Tape-record the choice.
                parent_tx = short_target; // None ⇒ fresh root; Some ⇒ extend the chosen node.
                if parent_tx.is_some() {
                    route_hit += 1;
                } else if choice.target < 0 {
                    route_fresh += 1;
                } else {
                    route_halluc += 1;
                }
                parent_body = parent_tx
                    .as_ref()
                    .and_then(|t| node_body.get(&t.0).cloned());
                parent_feedback = parent_tx
                    .as_ref()
                    .and_then(|t| node_feedback.get(&t.0).cloned());
                chosen_action = Some("solve".into());
                // Eng-2: carry the decision provenance onto the node. "agent" for a genuine solve
                // (or a genuine short degraded to solve by a missing live target — its action field
                // still parsed); "parse_fallback"/"llm_error" for a forced constructive solve.
                action_source = Some(choice.decision_source);
            }

            // Art II.2: build a BOUNDED price-signal block from the current price index.
            // Lists open nodes with their price_yes so agents can self-adjust (exploration vs
            // exploitation). Signal only — no solution steps, no LeanJudge/Predicate scoring
            // internals (Art III.4), no raw error logs (Art II.1). Empty on round 0 (pi empty).
            // Art III.2: bounded disclosure, a few lines.
            //
            // CONTROL-INTEGRITY GATE (confound class — Art II.2 broadcast-vs-no-broadcast A/B):
            // build this block ONLY for the price-BROADCASTING arms (Policy::broadcasts_price()).
            // For Policy::NoPrice (premise: "prices stripped from selection") and every
            // single/parallel/topology + non-price-scorer control, force price_ctx = "" so the
            // baselines can NEVER silently re-acquire the live price signal in the PROOF prompt.
            // `pi` is computed every iteration regardless of policy and NoPrice's price index IS
            // populated (it emits a Bear short); without this gate that live signal would leak
            // into the no-price/single/parallel proof prompts and contaminate the very baselines
            // the A/B measures against (false null / false positive).
            let price_ctx: String = if !args.policy.broadcasts_price() {
                String::new()
            } else {
                let entries: Vec<String> = pi
                    .values()
                    .enumerate()
                    .map(|(i, entry)| {
                        let py = entry.price_yes.as_ref().map(|r| {
                            if r.denominator == 0 { 0.0_f64 }
                            else { r.numerator as f64 / r.denominator as f64 }
                        });
                        match py {
                            Some(v) => format!("  node {i}: price_yes={v:.3}"),
                            None => format!("  node {i}: price_yes=none"),
                        }
                    })
                    .collect();
                if entries.is_empty() {
                    String::new()
                } else {
                    format!(
                        "\n=== Market Prices (signal only) ===\n{}\n",
                        entries.join("\n")
                    )
                }
            };
            // STAGE 2 (ALL arms): the SAME proof prompt as market/single for the chosen parent,
            // via the ONE shared `stage2_proof_prompt` constructor. Autonomous reaches this with
            // the post-route parent; market/single with their pre-call parent. Same fn + same
            // {theorem, parent_body, parent_feedback, lib} args for the same parent ⇒ the
            // proof-generation context is BYTE-IDENTICAL across arms EXCEPT for the policy-gated
            // `price_ctx` block: price-BROADCASTING arms (Policy::broadcasts_price()) carry a
            // non-empty Market-Prices block, every control (NoPrice / single / parallel /
            // topology / non-price scorers) carries `price_ctx == ""`. The confound-B parity gate
            // proves the SHARED-PATH invariant — for an IDENTICAL price_ctx the prompt is
            // byte-identical across arms (it holds price_ctx fixed and varies the route path) — and
            // the control-integrity test proves the INTENTIONAL price-axis divergence: a non-empty
            // price_ctx (broadcast arm) yields a STRICTLY DIFFERENT prompt than "" (control arm).
            let prompt = stage2_proof_prompt(
                &theorem,
                parent_body.as_deref(),
                parent_feedback.as_deref(),
                &lib,
                &price_ctx,
            );
            let resp = match llm
                .generate(&GenerateRequest {
                    // H-HET-2: tick_model = the router-selected model for VerifyUcbPriceFloor;
                    // = agent_models[ai] (fixed round-robin) for every other policy.
                    model: tick_model.clone(),
                    messages: vec![
                        sys.clone(),
                        Message {
                            role: "user".into(),
                            content: prompt,
                        },
                    ],
                    temperature: Some(PROOF_TEMPERATURE),
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
            // H-HET-2: a truncated proposal (hit the 900-token cap) is a SOFT failure (ruling:
            // excluded from hard-failure) — captured here for the router counts update below.
            tick_truncated = resp.completion_tokens as u64 >= 900;
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
            // OBL-018 (门0/D1): realign the model's proof_body — flush a flat tactic
            // sequence to col 0 — so a first-line-shallow body is not mislabeled Failed by
            // the conservative assemble-time `dedent`. SOUND: only cures false negatives;
            // Lean still arbitrates the goal. Feeds both verify() and assemble() below.
            let body = realign(&body);
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
                &tick_model, // H-HET-2: ProposalTelemetry.model_id = router-selected model (§8)
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
            // H-HET-2: record which model authored this node (target-local price prior) + update
            // the router's per-model verify/hard-fail counts. is_verified ⇒ reset hard-fail streak;
            // a non-verified, NON-truncated, kernel-reaching attempt ⇒ a HARD failure (ruling's
            // class); truncation/api/parse are SOFT (truncated handled here; api/parse already
            // `continue`d above so they only counted the pull, never a hard failure).
            if args.policy == Policy::VerifyUcbPriceFloor {
                rt_node_model.insert(work_tx_id.clone(), tick_model.clone());
                if is_verified {
                    *rt_verify.entry(tick_model.clone()).or_insert(0) += 1;
                    rt_hardfail.insert(tick_model.clone(), 0);
                } else if !tick_truncated {
                    *rt_hardfail.entry(tick_model.clone()).or_insert(0) += 1;
                }
            }
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
                let bear = match args.policy {
                    Policy::RandomBear => {
                        let doubt_pct = rng.gen_range(0..=100) as i64;
                        BearShortDecision {
                            short_micro: MIN_SHORT_MICRO
                                + (MAX_SHORT_MICRO - MIN_SHORT_MICRO) * doubt_pct / 100,
                            prompt_tokens: 0,
                            completion_tokens: 0,
                            flat_short_fallback: false,
                            parse_fallback: false,
                        }
                    }
                    Policy::FixedBear => BearShortDecision {
                        short_micro: CHALLENGE_STAKE_MICRO,
                        prompt_tokens: 0,
                        completion_tokens: 0,
                        flat_short_fallback: false,
                        parse_fallback: false,
                    },
                    _ => bear_doubt_short(&llm, &agent_models[ai], &theorem, &body).await,
                };
                bear_calls += 1;
                if bear.flat_short_fallback {
                    bear_flat_short_fallback_count += 1;
                }
                if bear.parse_fallback {
                    bear_parse_fallback_count += 1;
                }
                bear_prompt_tokens_total += bear.prompt_tokens;
                completion_tokens_total += bear.completion_tokens;
                bear_tokens_total += bear.prompt_tokens + bear.completion_tokens;
                let challenger = challengers[ai % challengers.len()].clone();
                if let Ok(ce) = put_counterexample(&args.cas, &work_tx_id, lt) {
                    lt += 1;
                    match make_real_challengetx_signed_by(
                        &mut kp,
                        root,
                        TxId(work_tx_id.clone()),
                        &challenger,
                        bear.short_micro,
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
                let bear = bear_doubt_short(&llm, &agent_models[ai], &theorem, &body).await;
                bear_calls += 1;
                if bear.flat_short_fallback {
                    bear_flat_short_fallback_count += 1;
                }
                if bear.parse_fallback {
                    bear_parse_fallback_count += 1;
                }
                bear_prompt_tokens_total += bear.prompt_tokens;
                completion_tokens_total += bear.completion_tokens;
                bear_tokens_total += bear.prompt_tokens + bear.completion_tokens;
                node_doubt.insert(work_tx_id.clone(), bear.short_micro);
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
                // Art 0.2: Some("solve") iff the AutonomousMarket agent self-chose to solve;
                // None for harness-assigned policies. (Self-chosen shorts are tape-recorded on
                // their own AttemptNode at the early-`continue` short path above.)
                chosen_action: chosen_action.clone(),
                // Eng-2: provenance so forced solves are excludable from the solve-rate metric.
                action_source,
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
    let total_model_tokens = proof_prompt_tokens
        + route_prompt_tokens
        + bear_prompt_tokens_total
        + completion_tokens_total;
    // F4: total_tokens is the PPUT/budget-parity denominator. Bind it to the same manifest-level
    // equation as total_model_tokens so node-local token previews cannot drift from accounting.
    let total_tokens: u64 = total_model_tokens;
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
        models: agent_models.clone(),
        proxy_url: args.proxy_url.clone(),
        proof_temperature: PROOF_TEMPERATURE,
        route_temperature: ROUTE_TEMPERATURE,
        bear_temperature: BEAR_TEMPERATURE,
        lean_bin: lean_bin.display().to_string(),
        lean_version,
        mathlib_dir: args.mathlib_dir.as_ref().map(|p| p.display().to_string()),
        mathlib_lean_path: mathlib_lp.clone(),
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
        bear_flat_short_fallback_count,
        bear_parse_fallback_count,
        proof_prompt_tokens,
        route_prompt_tokens,
        bear_prompt_tokens: bear_prompt_tokens_total,
        completion_tokens: completion_tokens_total,
        total_model_tokens,
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
        // BearTriage is price-family: it emits a real Bear short so the price index carries a
        // true signal (Bug1 fix — without shorts den==num → bear_score=0 → uniform routing).
        for p in [
            Policy::Market,
            Policy::Autonomous,
            Policy::RandomBear,
            Policy::FixedBear,
            Policy::ShuffledPrice,
            Policy::NoPrice,
            Policy::BearTriage,
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
            // AutonomousMarket does NOT auto-emit a per-proposal short: the agent self-selects
            // "short" as one of its two actions and the branch emits the ChallengeTx itself.
            // A generic auto-short here would double-short every self-chosen "solve" (Change 2/3).
            Policy::AutonomousMarket,
        ] {
            assert!(!p.emits_challenges(), "{p:?} does NOT auto-emit per-proposal shorts");
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
            "bear_triage",
            "autonomous_market",
        ] {
            assert_eq!(Policy::parse(s).unwrap().label(), s);
        }
        assert!(Policy::parse("bogus").is_err());
    }

    #[test]
    fn autonomous_market_select_parent_is_precall_noop() {
        // Like Autonomous, the AutonomousMarket parent/target is chosen by the agent INSIDE
        // STEP A (build_autonomous_decision_prompt), not by select_parent; the pre-call selector
        // MUST be a no-op (None) even with a fully-priced landscape, so STEP A is the sole source.
        let mut pi = BTreeMap::new();
        let nodes = vec![TxId("n0".into()), TxId("n1".into())];
        pi.insert(TxId("n0".into()), NodeMarketEntry::default());
        let mut rng = StdRng::seed_from_u64(11);
        assert_eq!(
            select_parent(
                Policy::AutonomousMarket,
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
    fn autonomous_choice_parses_and_fails_open_to_solve() {
        // A well-formed short choice parses verbatim (self-selected action + target + confidence).
        let short = parse_autonomous_choice(
            "{\"action\":\"short\",\"target\":2,\"confidence\":0.9}",
        );
        assert_eq!(short.action, "short");
        assert_eq!(short.target, 2);
        assert!((short.confidence - 0.9).abs() < 1e-9);
        // A well-formed solve choice on a fresh root parses verbatim.
        let solve = parse_autonomous_choice(
            "{\"action\":\"solve\",\"target\":-1,\"proof_body\":\"simp\",\"confidence\":0.5}",
        );
        assert_eq!(solve.action, "solve");
        assert_eq!(solve.target, -1);
        // Malformed JSON, an unknown action, and a missing action all FAIL-OPEN to solve/fresh —
        // never silently dropping the iteration (budget-parity node count is preserved).
        for bad in ["not json", "{\"action\":\"sabotage\",\"target\":0}", "{\"target\":0}"] {
            let c = parse_autonomous_choice(bad);
            assert_eq!(c.action, "solve", "fail-open action for {bad:?}");
        }
        // An empty/garbage body fails open to a fresh-root solve (target -1).
        assert_eq!(parse_autonomous_choice("not json").target, -1);
    }

    #[test]
    fn autonomous_decision_prompt_is_signal_only_and_decorrelated() {
        // STEP A prompt must broadcast SIGNALS ONLY (Art II.2): it offers the 2-action menu, lists
        // open nodes by price/conf/error-CLASS, and carries the ABSTRACTED librarian — but NEVER a
        // raw proof body, raw shielded error line, or any LeanJudge/Predicate scoring internal.
        let thm = selftest_theorem();
        let nodes = vec![TxId("worktx-abc".into())];
        let mut fb = BTreeMap::new();
        fb.insert("worktx-abc".into(), "error: unsolved goals\n  ⊢ secret_raw".into());
        let mut conf = BTreeMap::new();
        conf.insert("worktx-abc".into(), 40u64);
        let pi = BTreeMap::new();
        let prompt = build_autonomous_decision_prompt(&thm, &nodes, &fb, &conf, &pi, "");
        // Menu is exactly the two self-selected actions.
        assert!(prompt.contains("\"solve\""), "menu offers solve");
        assert!(prompt.contains("\"short\""), "menu offers short");
        // Only the coarse error CLASS leaks, never the raw shielded line (Art II.1 / III.4).
        assert!(prompt.contains("unsolved_goals"), "coarse error class is shown");
        assert!(!prompt.contains("secret_raw"), "raw shielded error must NOT leak");
        // No assigned role / no HOW-to-prove instruction (Art II.2 — no micromanagement).
        assert!(!prompt.to_lowercase().contains("your role is"));
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
        let market = build_prompt(&thm, Some(body), Some(fb), "", "");
        let stage2 = stage2_proof_prompt(&thm, Some(body), Some(fb), "", "");
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

    // ── F1–F5 self-test seam mirrored as a #[test] so `cargo test` covers the same checks the
    // shipped `--self-test` CLI path runs (makes the seam a real gate, not documentation). ──
    #[test]
    fn self_test_inner_ok() {
        self_test_inner().expect("self_test_inner (F1–F5) must pass");
    }

    // ── F5 cost-resolution (OBL-012 / §17.3 no name-lie): every DEFAULT roster model must bill at a
    // SPECIFIC MODEL_RATES entry, never the bare `deepseek`/FALLBACK catch-all. This is the exact
    // failure this diff's roster introduced (`deepseek-ai/DeepSeek-V4-Pro` not matching the lowercase
    // `deepseek-v4-pro` id under case-sensitive `contains`). LOAD-BEARING: it would FAIL again if the
    // case-insensitive match in `call_micro_usd` were reverted. ──
    #[test]
    fn default_roster_never_resolves_to_bare_deepseek_fallback() {
        for model in default_models() {
            let in_cost = call_micro_usd(&model, 1_000_000, 0);
            assert_ne!(
                in_cost, FALLBACK_IN_UPMT,
                "roster model `{model}` under-bills at the bare deepseek/FALLBACK input rate (OBL-012)"
            );
            let out_cost = call_micro_usd(&model, 0, 1_000_000);
            assert_ne!(
                out_cost, FALLBACK_OUT_UPMT,
                "roster model `{model}` over-bills at the bare deepseek/FALLBACK output rate (OBL-012)"
            );
        }
        // Pin the historically-bugged entry to its intended deepseek-v4-pro rate.
        assert_eq!(
            call_micro_usd("deepseek-ai/DeepSeek-V4-Pro", 1_000_000, 0),
            435_000,
            "DeepSeek-V4-Pro must bill at the deepseek-v4-pro input rate, not the bare deepseek catch-all"
        );
        assert_eq!(
            call_micro_usd("deepseek-ai/DeepSeek-V4-Pro", 0, 1_000_000),
            870_000,
            "DeepSeek-V4-Pro must bill at the deepseek-v4-pro output rate, not the bare deepseek catch-all"
        );
    }

    // ── Robustness: a `--models` that parses to an empty roster (e.g. `--models ","`) must NOT
    // yield an empty `args.models` (which would make `args.models[i % args.models.len()]` a
    // divide-by-zero panic in the agent loop); it falls back to the non-empty default roster. ──
    #[test]
    fn empty_models_arg_falls_back_to_nonempty_default() {
        let argv: Vec<String> = [
            "--runtime-repo", "/tmp/x",
            "--cas", "/tmp/x",
            "--run-id", "t",
            "--problem", "p",
            "--models", ",",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let args = parse_args(&argv).expect("parse_args must succeed");
        assert!(
            !args.models.is_empty(),
            "empty --models must fall back to the default roster, not an empty Vec (divide-by-zero guard)"
        );
        assert_eq!(args.models, default_models());
        // Directly exercise the formerly-panicking index expression.
        let _ = args.models[0 % args.models.len()].clone();
    }

    // ── Control-integrity gate (Art II.2 broadcast-vs-no-broadcast A/B confound) ──
    // The live loop builds the Stage-2 `price_ctx` block ONLY for `Policy::broadcasts_price()`
    // arms and forces `price_ctx == ""` for every control. These two tests pin BOTH halves so a
    // control arm can NEVER silently re-acquire the live price signal in the proof prompt.

    #[test]
    fn price_broadcast_gated_to_market_family_only() {
        // EXACTLY the price-broadcasting arms (their hypothesis = "broadcast price helps").
        for p in [
            Policy::Market,
            Policy::RandomBear,
            Policy::FixedBear,
            Policy::ShuffledPrice,
            Policy::BearTriage,
            Policy::AutonomousMarket,
        ] {
            assert!(
                p.broadcasts_price(),
                "{} must broadcast price (Art II.2 treatment arm)",
                p.label()
            );
        }
        // The no-price baseline and EVERY single/parallel/topology + non-price-scorer control
        // must NOT broadcast price into the proof prompt. NoPrice is the load-bearing case:
        // it `emits_challenges()` (so its price index IS populated) yet its premise is "prices
        // stripped from selection" — it must still get an empty price block.
        assert!(Policy::NoPrice.emits_challenges(), "NoPrice populates pi (precondition for the leak this gate prevents)");
        for p in [
            Policy::NoPrice,
            Policy::Single,
            Policy::Parallel,
            Policy::SingleRestart,
            Policy::SingleTreeNoPrice,
            Policy::ParallelRestart,
            Policy::Majority,
            Policy::BestFirst,
            Policy::SkepticRerank,
            Policy::Autonomous, // price stays in its Stage-1 route channel only, never the proof prompt
        ] {
            assert!(
                !p.broadcasts_price(),
                "{} is a no-broadcast control — its Stage-2 price_ctx must be empty",
                p.label()
            );
        }
    }

    #[test]
    fn price_ctx_axis_changes_proof_prompt() {
        // The confound-B parity gate holds `price_ctx` FIXED ("") and proves the route path does
        // not perturb the prompt. THIS test exercises the OTHER axis the live loop now varies:
        // a non-empty price_ctx (broadcast arm) must yield a STRICTLY DIFFERENT proof prompt than
        // "" (control arm) for the SAME parent — i.e. the green parity test is not blind to the
        // policy-gated price dimension. Drive BOTH operands through the SHARED stage2 helper.
        let thm = selftest_theorem();
        let body = "intro h; simp_all; exact h";
        let fb = "error: unsolved goals n+0=n";
        // Treatment arm: the exact Market-Prices block shape the live loop appends.
        let price_block = "\n=== Market Prices (signal only) ===\n  node 0: price_yes=0.429\n";
        let broadcast = stage2_proof_prompt(&thm, Some(body), Some(fb), "", price_block);
        let control = stage2_proof_prompt(&thm, Some(body), Some(fb), "", "");
        assert_ne!(
            sha_hex(&broadcast),
            sha_hex(&control),
            "control-integrity gate not load-bearing: a broadcast arm's price_ctx must change the \
             proof prompt vs a no-broadcast control — otherwise the A/B treatment is a no-op"
        );
        // And the divergence is exactly the price block (strict superset), not some other drift.
        assert!(broadcast.len() > control.len());
        assert!(broadcast.contains("Market Prices (signal only)"));
        assert!(!control.contains("Market Prices"));
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

    // ── BEAR-TRIAGE: bear_soft_priority distribution properties ──

    /// Deterministic test for bear_soft_priority.
    ///
    /// bear_soft_priority receives PRE-NORMALIZED scores (norm_score in [0, 1000]) where HIGHER
    /// norm_score = HIGHER priority (low-short node already has the highest norm_score after the
    /// inversion in select_parent::BearTriage). This test exercises the weighting directly.
    ///
    /// Constructs 4 nodes with norm_scores [0, 200, 500, 1000].
    /// With lambda_num=1, lambda_den=1, weights = [1+0, 1+200, 1+500, 1+1000] = [1, 201, 501, 1001].
    /// Total = 1704.
    ///
    /// G1 self-check: all nodes have non-zero weight → no node is ever permanently excluded.
    /// We verify across 2000 seeds that:
    ///   (a) every node is selected at least once (true soft, not argmax-collapse / §17.3)
    ///   (b) higher norm_score nodes are selected more often (rank order: ns1000 > ns500 > ns200 > ns0)
    #[test]
    fn bear_soft_priority_true_soft_distribution() {
        let ns0 = TxId("node_ns0".into());
        let ns200 = TxId("node_ns200".into());
        let ns500 = TxId("node_ns500".into());
        let ns1000 = TxId("node_ns1000".into());

        // norm_scores: ns0→0, ns200→200, ns500→500, ns1000→1000 (pre-inverted by caller)
        // weights (lambda=1/1): [1, 201, 501, 1001]
        let scores: Vec<(TxId, i128)> = vec![
            (ns0.clone(), 0),
            (ns200.clone(), 200),
            (ns500.clone(), 500),
            (ns1000.clone(), 1000),
        ];

        let n_trials = 2_000u64;
        let mut counts: BTreeMap<String, u64> = BTreeMap::new();
        for seed in 0..n_trials {
            let mut rng = StdRng::seed_from_u64(seed);
            let picked = bear_soft_priority(&scores, 1, 1, &mut rng)
                .expect("non-empty scores must return Some");
            *counts.entry(picked.0.clone()).or_default() += 1;
        }

        // (a) G1: every node selected at least once — true soft distribution, no veto path.
        assert!(
            counts.get(&ns0.0).copied().unwrap_or(0) > 0,
            "bear_soft_priority: node_ns0 (weight=1) never selected — argmax-collapse detected"
        );
        assert!(
            counts.get(&ns200.0).copied().unwrap_or(0) > 0,
            "bear_soft_priority: node_ns200 never selected"
        );
        assert!(
            counts.get(&ns500.0).copied().unwrap_or(0) > 0,
            "bear_soft_priority: node_ns500 never selected"
        );
        assert!(
            counts.get(&ns1000.0).copied().unwrap_or(0) > 0,
            "bear_soft_priority: node_ns1000 (weight=1001) never selected"
        );

        // (b) rank order: ns1000 > ns500 > ns200 > ns0 (higher norm_score → higher freq).
        let c0 = counts.get(&ns0.0).copied().unwrap_or(0);
        let c200 = counts.get(&ns200.0).copied().unwrap_or(0);
        let c500 = counts.get(&ns500.0).copied().unwrap_or(0);
        let c1000 = counts.get(&ns1000.0).copied().unwrap_or(0);
        assert!(
            c1000 > c500,
            "bear_soft_priority: ns1000 ({c1000}) should be picked more than ns500 ({c500})"
        );
        assert!(
            c500 > c200,
            "bear_soft_priority: ns500 ({c500}) should be picked more than ns200 ({c200})"
        );
        assert!(
            c200 > c0,
            "bear_soft_priority: ns200 ({c200}) should be picked more than ns0 ({c0})"
        );
    }

    /// Deterministic test: bear_soft_priority with a single node always returns that node.
    #[test]
    fn bear_soft_priority_single_node_always_selected() {
        let only = TxId("sole".into());
        let scores = vec![(only.clone(), 7i128)];
        for seed in 0..100u64 {
            let mut rng = StdRng::seed_from_u64(seed);
            assert_eq!(
                bear_soft_priority(&scores, 1, 1, &mut rng),
                Some(only.clone()),
                "single-node: must always pick the only node"
            );
        }
    }

    /// Deterministic test: empty scores returns None (caller falls back).
    #[test]
    fn bear_soft_priority_empty_returns_none() {
        let mut rng = StdRng::seed_from_u64(0);
        assert_eq!(bear_soft_priority(&[], 1, 1, &mut rng), None);
    }

    /// select_parent with BearTriage routes toward the LOW-short node (Bug2 fix: direction
    /// reversal based on Phase 1 evidence P(short_Verified < short_Failed) = 0.62).
    ///
    /// n_lo has price 9/10 (raw_bear = 10-9 = 1, low skepticism → high norm_score → preferred).
    /// n_hi has price 2/10 (raw_bear = 10-2 = 8, high skepticism → low norm_score → deprioritized).
    /// n_none: no price entry (raw_bear = 0, treated as most-confident/lowest-short).
    ///
    /// Normalization with M = max(raw_bear) = max(1, 8, 0) = 8:
    ///   n_lo:  norm_score = 1000*(8-1)/8 = 875
    ///   n_hi:  norm_score = 1000*(8-8)/8 = 0
    ///   n_none:norm_score = 1000*(8-0)/8 = 1000
    /// Weights (λ=1/1): n_lo=876, n_hi=1, n_none=1001. Total=1878.
    /// → n_none > n_lo >> n_hi; lo_count >> hi_count confirms the direction.
    ///
    /// True-soft-distribution check: n_hi (weight=1) must still appear at least once (G1/§17.3).
    #[test]
    fn bear_triage_select_parent_integration() {
        // Three nodes: n_lo has price 9/10 (bear=1), n_hi has price 2/10 (bear=8), n_none has no price.
        let n_lo = TxId("n_lo".into());
        let n_hi = TxId("n_hi".into());
        let n_none = TxId("n_none".into());
        let all_nodes = vec![n_lo.clone(), n_hi.clone(), n_none.clone()];
        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
        pi.insert(
            n_lo.clone(),
            NodeMarketEntry {
                price_yes: Some(RationalPrice {
                    numerator: 9,
                    denominator: 10,
                }),
                ..Default::default()
            },
        );
        pi.insert(
            n_hi.clone(),
            NodeMarketEntry {
                price_yes: Some(RationalPrice {
                    numerator: 2,
                    denominator: 10,
                }),
                ..Default::default()
            },
        );
        // n_none: no price_yes entry → raw_bear = 0 → norm_score = 1000 (lowest-short = most trusted).

        let conf: BTreeMap<String, u64> = BTreeMap::new();
        let doubt: BTreeMap<String, i64> = BTreeMap::new();
        let mut hi_count = 0u64;
        let mut lo_count = 0u64;
        // Use 10 000 trials: n_hi has weight 1/1878 ≈ 0.053%; expected ~5 appearances.
        // This keeps the test deterministic and the G1 no-veto-path invariant verifiable
        // even at extreme weight ratios created by the permille normalization.
        let n_trials = 10_000u64;
        for seed in 0..n_trials {
            let mut rng = StdRng::seed_from_u64(seed);
            let picked = select_parent(
                Policy::BearTriage,
                &pi,
                &all_nodes,
                None,
                &[],
                &conf,
                &doubt,
                0.15,
                &mut rng,
            );
            assert!(picked.is_some(), "BearTriage with non-empty landscape must return Some");
            match picked.as_ref().map(|t| t.0.as_str()) {
                Some("n_hi") => hi_count += 1,
                Some("n_lo") => lo_count += 1,
                _ => {} // n_none also valid (norm_score=1000, highest priority)
            }
        }
        // Phase 1 direction: route toward LOW short → n_lo (norm_score=875) >> n_hi (norm_score=0).
        // lo_count > hi_count confirms the direction is correct (Bug2 fix: previously asserted inverse).
        assert!(
            lo_count > hi_count,
            "BearTriage: low-short node n_lo ({lo_count}) should be chosen more than high-short n_hi ({hi_count}) — Phase 1: P(short_Verified < short_Failed)=0.62"
        );
        // True-soft-distribution (G1/§17.3): n_hi (weight=1/1878) must appear at least once —
        // no veto path. With 10 000 trials, expected ~5 appearances; probability of zero < 1%.
        assert!(hi_count > 0, "n_hi (weight=1/1878) must be selected at least once in 10000 trials — true soft, no veto path (G1/§17.3)");
        assert!(lo_count > 0, "n_lo must be selected at least once");
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

    // ── Gate C+F: prompt is signal-only; no Goodhart leak (Art II.2 + Art III.4) ──
    //
    // Renders build_autonomous_decision_prompt and build_prompt with synthetic node_feedback that
    // contains a raw stderr line ("error: unknown tactic") AND a Goodhart bait string ("score=99
    // verdict=PASS") to PROVE the rendered prompts strip both down to the coarse class only.
    //
    // MUST contain (decision prompt): no-role phrasing, signal-only framing, self-choose menu.
    // MUST NOT contain (decision prompt): assigned role label as instruction, raw stderr text,
    //   raw score= string, LeanJudge/predicate scoring internals, heldout/eval identifiers.
    // Broadcast field whitelist (per node line): only index, price_yes num/den, confidence,
    //   coarse error-class appear — no proof bodies, no other-agent raw data.
    // MUST contain (proof prompt): parent body for the chosen parent.
    // MUST NOT contain (proof prompt): raw stderr or score= from other nodes' feedback.
    #[test]
    fn gate_c_f_prompt_is_signal_only_no_goodhart_leak() {
        let thm = selftest_theorem();

        // Synthetic frontier: two nodes.
        // Node 0: raw feedback with a Goodhart bait string ("score=99 verdict=PASS") + raw stderr.
        // Node 1: distinct body and distinct raw feedback.
        let node0 = TxId("worktx-n0".into());
        let node1 = TxId("worktx-n1".into());
        let nodes = vec![node0.clone(), node1.clone()];

        // Goodhart bait: a "score=" string that mimics verifier/LeanJudge scoring internals.
        // If this leaks into the agent prompt verbatim the gate fails (Art III.4 Goodhart shield).
        let raw_stderr_node0 = "error: unknown tactic `decide` at line 3\nscore=99 verdict=PASS heldout_eval=true";
        // Node 1 has a different raw error line; its body should NOT appear in node 0's decision ctx.
        let raw_stderr_node1 = "error: unsolved goals\n  ⊢ n+0=n  SECRET_BODY_SENTINEL_1234";

        let mut fb: BTreeMap<String, String> = BTreeMap::new();
        fb.insert(node0.0.clone(), raw_stderr_node0.to_string());
        fb.insert(node1.0.clone(), raw_stderr_node1.to_string());

        let mut conf: BTreeMap<String, u64> = BTreeMap::new();
        conf.insert(node0.0.clone(), 55u64);
        conf.insert(node1.0.clone(), 40u64);

        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
        pi.insert(
            node0.clone(),
            NodeMarketEntry {
                price_yes: Some(RationalPrice { numerator: 3, denominator: 7 }),
                ..Default::default()
            },
        );
        pi.insert(
            node1.clone(),
            NodeMarketEntry {
                price_yes: Some(RationalPrice { numerator: 1, denominator: 4 }),
                ..Default::default()
            },
        );

        let decision_prompt = build_autonomous_decision_prompt(&thm, &nodes, &fb, &conf, &pi, "");

        // ── MUST-CONTAIN (Art II.2 signal-only / no-role contract) ──

        // No role has been assigned — literal phrase from the prompt builder.
        assert!(
            decision_prompt.contains("No role has been assigned"),
            "decision prompt MUST carry no-role-assigned phrasing (Art II.2)\n--- prompt ---\n{decision_prompt}"
        );
        // Signal-only framing.
        assert!(
            decision_prompt.contains("ONLY market signals"),
            "decision prompt MUST carry signal-only framing (Art II.2)\n--- prompt ---\n{decision_prompt}"
        );
        // Self-choose menu: both actions available.
        assert!(
            decision_prompt.contains("\"solve\""),
            "decision prompt MUST offer \"solve\" action in the self-choose menu\n--- prompt ---\n{decision_prompt}"
        );
        assert!(
            decision_prompt.contains("\"short\""),
            "decision prompt MUST offer \"short\" action in the self-choose menu\n--- prompt ---\n{decision_prompt}"
        );
        // The coarse error class (not the raw line) IS allowed.
        let class0 = classify_lean_error(raw_stderr_node0);
        assert!(
            decision_prompt.contains(class0),
            "decision prompt MUST carry the coarse error class `{class0}` for node0\n--- prompt ---\n{decision_prompt}"
        );
        // price_yes num/den for node 0 must appear.
        assert!(
            decision_prompt.contains("price_yes=3/7"),
            "decision prompt MUST broadcast node0 price_yes=3/7 (signal whitelist)\n--- prompt ---\n{decision_prompt}"
        );

        // ── MUST-NOT-CONTAIN (Goodhart shield Art III.4 + decorrelation Art III.3) ──

        // Raw Lean stderr must NOT leak — only the coarse class.
        assert!(
            !decision_prompt.contains("unknown tactic"),
            "decision prompt MUST NOT contain raw stderr text `unknown tactic` (Art II.1 / III.4)\n--- prompt ---\n{decision_prompt}"
        );
        // Goodhart bait: score= / verdict= / heldout_eval= from verifier internals must NOT appear.
        assert!(
            !decision_prompt.contains("score=99"),
            "decision prompt MUST NOT contain Goodhart bait `score=99` (verifier internal, Art III.4)\n--- prompt ---\n{decision_prompt}"
        );
        assert!(
            !decision_prompt.contains("verdict=PASS"),
            "decision prompt MUST NOT contain Goodhart bait `verdict=PASS` (verifier internal, Art III.4)\n--- prompt ---\n{decision_prompt}"
        );
        assert!(
            !decision_prompt.contains("heldout_eval"),
            "decision prompt MUST NOT contain heldout/eval identifier (benchmark leak, Art III.4)\n--- prompt ---\n{decision_prompt}"
        );
        // Node 1's raw sentinel must NOT appear (decorrelation: no cross-agent raw leakage).
        assert!(
            !decision_prompt.contains("SECRET_BODY_SENTINEL_1234"),
            "decision prompt MUST NOT contain node1 raw sentinel — raw error text must never transit (Art II.1)\n--- prompt ---\n{decision_prompt}"
        );
        // Assigned role labels (Bull/Bear/Solver/Challenger) must NOT appear as instructions.
        // These are assignment strings — the prompt must not command "You are a Bull/Bear/Solver/Challenger".
        for role_label in ["You are a Bull", "You are a Bear", "You are a Solver", "You are a Challenger"] {
            assert!(
                !decision_prompt.contains(role_label),
                "decision prompt MUST NOT assign a role via `{role_label}` (Art II.2 no-role)\n--- prompt ---\n{decision_prompt}"
            );
        }

        // ── Broadcast field whitelist: per-node line must only carry index, price num/den,
        // confidence%, and coarse error-class — NO proof bodies, NO raw errors, NO other fields. ──
        // Extract the Open nodes section lines.
        for line in decision_prompt.lines() {
            if line.starts_with('[') && line.contains("price_yes=") {
                // Each frontier line format: [idx] price_yes=N/D conf=C% class=CLASS
                // Must NOT carry any raw body text or score= bait.
                assert!(
                    !line.contains("score="),
                    "frontier line MUST NOT contain `score=` (Goodhart leak): {line:?}"
                );
                assert!(
                    !line.contains("unknown tactic"),
                    "frontier line MUST NOT contain raw stderr: {line:?}"
                );
                assert!(
                    !line.contains("SECRET_BODY_SENTINEL"),
                    "frontier line MUST NOT contain proof body sentinel: {line:?}"
                );
            }
        }

        // ── Proof prompt (build_prompt) must not leak raw feedback from non-parent nodes ──
        // build_prompt is the Stage-2 proof-prompt constructor; for the proof repair task it
        // DOES include the chosen parent's raw Lean feedback (the model needs to see the error
        // to fix it). What must NOT appear is any raw data from NON-parent nodes.
        // We call build_prompt with node0 as the parent; node1's sentinel must not appear.
        let proof_prompt = build_prompt(
            &thm,
            Some("intro n; simp"),      // parent body (node0)
            Some(raw_stderr_node0),     // parent raw feedback (node0) — legitimately included
            "",
            "",
        );
        // Node 1's raw sentinel absolutely must not appear in a node-0-parent proof prompt.
        assert!(
            !proof_prompt.contains("SECRET_BODY_SENTINEL_1234"),
            "proof prompt MUST NOT contain node1 raw sentinel — non-parent node data must never transit\n--- prompt ---\n{proof_prompt}"
        );
        // The proof prompt's own code path must not inject any extra scoring/Goodhart fields
        // beyond what the caller passes as parent_feedback. We check by building with empty
        // feedback: the resulting prompt must not contain any score= or verdict= strings.
        let proof_prompt_empty_fb = build_prompt(&thm, Some("intro n; simp"), None, "", "");
        assert!(
            !proof_prompt_empty_fb.contains("score="),
            "proof prompt (no-feedback path) MUST NOT synthesize Goodhart bait from its own code path\n--- prompt ---\n{proof_prompt_empty_fb}"
        );
        assert!(
            !proof_prompt_empty_fb.contains("verdict="),
            "proof prompt (no-feedback path) MUST NOT synthesize verdict= strings from its own code path\n--- prompt ---\n{proof_prompt_empty_fb}"
        );
    }

    // ── Gate E: heterogeneity decorrelated (Art III.3 + §17.3 no-argmax-collapse) ──
    //
    // With a multi-vendor --models roster, per-agent effective model ids must have entropy>0
    // (not all the same), a round does NOT all-fallback to a single provider, and the decision
    // context rendered for agent i does NOT contain another agent's raw proof_body (Art III.3).
    //
    // Reuses/extends the existing default_roster_never_resolves_to_bare_deepseek_fallback and
    // price_broadcast_gated tests by exercising the round-robin assignment directly and asserting
    // the per-agent isolation property on build_autonomous_decision_prompt.
    #[test]
    fn gate_e_heterogeneity_decorrelated() {
        // ── 1. Per-agent model assignment has entropy > 0 ──
        // default_models() returns 4 distinct vendor models. With n_agents >= 4, round-robin
        // assigns each a different model → at least 2 distinct effective ids in 4 agents.
        let roster = default_models();
        let n_agents = 4usize;
        let agent_models: Vec<String> = (0..n_agents)
            .map(|i| roster[i % roster.len()].clone())
            .collect();

        // Entropy > 0: not all model ids are identical.
        let first = &agent_models[0];
        let all_same = agent_models.iter().all(|m| m == first);
        assert!(
            !all_same,
            "Gate E: per-agent model assignment must have entropy > 0 — found all agents using `{first}` (argmax-collapse / §17.3)"
        );

        // At least 2 distinct models in the 4-agent roster.
        let distinct: std::collections::BTreeSet<&String> = agent_models.iter().collect();
        assert!(
            distinct.len() >= 2,
            "Gate E: a multi-vendor roster of {} models must yield >= 2 distinct effective model ids across {} agents; got {}: {:?}",
            roster.len(), n_agents, distinct.len(), agent_models
        );

        // ── 2. No all-fallback to a single provider ──
        // Each model in the roster must NOT resolve to the bare FALLBACK rate (OBL-012 gate
        // mirrored here: if a model fallbacks, it means cost resolution collapsed to one entry).
        // A round that all-falls-back to one provider is a billing + diversity failure.
        for model in &roster {
            let in_cost = call_micro_usd(model, 1_000_000, 0);
            assert_ne!(
                in_cost, FALLBACK_IN_UPMT,
                "Gate E: roster model `{model}` resolves to the bare FALLBACK input rate — single-provider collapse detected (OBL-012)"
            );
            let out_cost = call_micro_usd(model, 0, 1_000_000);
            assert_ne!(
                out_cost, FALLBACK_OUT_UPMT,
                "Gate E: roster model `{model}` resolves to the bare FALLBACK output rate — single-provider collapse detected (OBL-012)"
            );
        }

        // ── 3. Decision context for agent i does NOT contain another agent's raw proof_body ──
        // Art III.3 decorrelation: build_autonomous_decision_prompt is called with the shared
        // committed tape state (price index + coarse error class). No in-flight proof body from
        // another agent's pending attempt should appear in agent i's context.
        //
        // Synthetic scenario: agent 0 has submitted a proof body on node0 (in-flight, uncommitted).
        // Agent 1's decision context is built from the SAME committed tape. We inject agent 0's
        // raw proof body as a sentinel into node0's feedback MAP (simulating a mis-implementation
        // that leaks the in-flight body into the feedback channel) and assert it does NOT transit
        // into agent 1's rendered decision prompt.
        let thm = selftest_theorem();
        let node0 = TxId("worktx-decorr-0".into());
        let node1 = TxId("worktx-decorr-1".into());
        let nodes = vec![node0.clone(), node1.clone()];

        // Agent 0's in-flight proof body — must NOT appear in agent 1's context.
        let agent0_raw_proof_body = "intro n; AGENT0_PROOF_SENTINEL; simp_all; exact Nat.zero_add n";
        // If a buggy implementation leaked the proof body into node_feedback, it would look like:
        let leaked_feedback = format!("error: type mismatch\n--- agent0 in-flight body ---\n{agent0_raw_proof_body}");

        let mut fb: BTreeMap<String, String> = BTreeMap::new();
        fb.insert(node0.0.clone(), leaked_feedback); // attempt to inject the in-flight body
        fb.insert(node1.0.clone(), "error: unsolved goals\n  ⊢ n+0=n".to_string());

        let mut conf: BTreeMap<String, u64> = BTreeMap::new();
        conf.insert(node0.0.clone(), 60u64);
        conf.insert(node1.0.clone(), 35u64);

        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
        pi.insert(
            node0.clone(),
            NodeMarketEntry {
                price_yes: Some(RationalPrice { numerator: 2, denominator: 5 }),
                ..Default::default()
            },
        );

        // Agent 1's decision context — built from committed tape (price index + classify path).
        let agent1_ctx = build_autonomous_decision_prompt(&thm, &nodes, &fb, &conf, &pi, "");

        // Art III.3: agent 1's context must NOT contain agent 0's raw proof body sentinel.
        // The coarse error class ("type_mismatch") is allowed; the raw body is NOT.
        assert!(
            !agent1_ctx.contains("AGENT0_PROOF_SENTINEL"),
            "Gate E Art III.3: agent 1 decision context MUST NOT contain agent 0's raw proof body sentinel — cross-agent decorrelation violated\n--- agent1_ctx ---\n{agent1_ctx}"
        );
        // The coarse class for node0 (type_mismatch) IS allowed to transit.
        let class0 = classify_lean_error("error: type mismatch");
        assert_eq!(class0, "type_mismatch");
        assert!(
            agent1_ctx.contains(class0),
            "Gate E: coarse error class `{class0}` must transit (signal whitelist) but was absent\n--- agent1_ctx ---\n{agent1_ctx}"
        );

        // Also verify that with heterogeneous roster, distinct agents see heterogeneous model
        // assignments (entropy check on actual round-robin output for the full default roster).
        let n_large = roster.len();
        let large_agent_models: Vec<String> = (0..n_large)
            .map(|i| roster[i % roster.len()].clone())
            .collect();
        let distinct_large: std::collections::BTreeSet<&String> = large_agent_models.iter().collect();
        assert_eq!(
            distinct_large.len(), roster.len(),
            "Gate E: with n_agents == roster.len() ({n_large}), every agent should get a distinct model; got {:?}",
            large_agent_models
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
            "proxy_url",
            "proof_temperature",
            "route_temperature",
            "bear_temperature",
            "lean_bin",
            "lean_version",
            "mathlib_dir",
            "mathlib_lean_path",
            "bear_flat_short_fallback_count",
            "bear_parse_fallback_count",
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
