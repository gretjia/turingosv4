//! Tactic-level proof-TREE market — the constitution's actual mechanism (Art. II.2.1 + §B
//! "Node = git commit / branch = git branch / Boltzmann routing = git branch").
//!
//! H0/H1 showed the market ≈ single because nodes were FULL proof attempts (refining one ≈ a
//! single agent's retry) and routing was argmax (collapse to one chain). This models the proof
//! as a real TREE of PARTIAL Lean tactic-states:
//!   node = (tactic script so far, parsed remaining goals)
//!   expand = LLM proposes the NEXT tactic given the remaining goals → Lean → child state
//!   branch = different next-tactics from the SAME state; dead-end = a state whose expansions fail
//!   OMEGA  = a tactic script that closes all goals (Lean exit 0, no sorry)
//!
//! market (policy=market): Boltzmann-softmax tree search over a heuristic node VALUE
//!   (progress − stuck-penalty), so attention DISTRIBUTES across promising partial states INCLUDING
//!   early ones (non-local re-expansion / new branch / backtrack). The architect's vision.
//!   CORRECTION 2026-06-01 (forensic retrospective §0/§1): this was earlier labeled a price-based
//!   tree search, but `value()` is a HEURISTIC — the loss-bearing market signal is ABSENT here
//!   (grep: zero `price` identifier in code). The ONLY bin with loss-bearing price + true softmax
//!   over the live index is `src/bin/lean_market_agent.rs`. Treat this bin as heuristic tree
//!   search ONLY; do NOT cite it as price-market evidence.
//! single (policy=single): one DFS chain — always extend the agent's own deepest live node; on a
//!   failed tactic it retries the SAME node (no jump to a different branch). The control.
//!
//! Real Lean proofs have choice points + dead ends, so a tree-searching market that backtracks
//! from a stuck branch should beat a single agent committed to one tactic path — even with a
//! homogeneous agent pool. Diagnostic-grade (no chain/CAS/replay); allows decide/native_decide
//! (sound; the no-native_decide rule stays in LeanJudge for the real OMEGA market).
//!
//! Class 1 additive bin.

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::path::{Path, PathBuf};
use std::time::Instant;
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};

/// (preamble ending in ":= by", initial goal text shown to the LLM at the root).
fn theorem(id: &str) -> Option<(&'static str, &'static str)> {
    match id {
        "tm_cube4" => Some((
            "import Mathlib\nopen Finset in\ntheorem tm (n : ℕ) : (∑ i ∈ Finset.range (n + 1), i ^ 3) * 4 = (n * (n + 1)) ^ 2 := by",
            "(n : ℕ) ⊢ (∑ i ∈ Finset.range (n + 1), i ^ 3) * 4 = (n * (n + 1)) ^ 2",
        )),
        "tm_mono" => Some((
            "import Mathlib\ntheorem tm : StrictMono (fun x : ℝ => x ^ 3 + x) := by",
            "⊢ StrictMono (fun x : ℝ => x ^ 3 + x)",
        )),
        "tm_sqrt2" => Some((
            "import Mathlib\ntheorem tm : Irrational (Real.sqrt 2) := by",
            "⊢ Irrational (Real.sqrt 2)",
        )),
        "tm_sumodd" => Some((
            "import Mathlib\nopen Finset in\ntheorem tm (n : ℕ) : ∑ i ∈ Finset.range n, (2 * i + 1) = n ^ 2 := by",
            "(n : ℕ) ⊢ ∑ i ∈ Finset.range n, (2 * i + 1) = n ^ 2",
        )),
        // Branchy candidates (multiple plausible first moves, most dead-ending) for the
        // Goldilocks search band. All verified provable in /tmp before adding.
        "tm_sumsq" => Some((
            "import Mathlib\nopen Finset in\ntheorem tm (n : ℕ) : (∑ i ∈ Finset.range (n+1), i) * 2 = n * (n+1) := by",
            "(n : ℕ) ⊢ (∑ i ∈ Finset.range (n+1), i) * 2 = n * (n+1)",
        )),
        "tm_dvd" => Some((
            "import Mathlib\ntheorem tm (n : ℕ) : 6 ∣ n * (n+1) * (n+2) := by",
            "(n : ℕ) ⊢ 6 ∣ n * (n+1) * (n+2)",
        )),
        "tm_amgm3" => Some((
            "import Mathlib\ntheorem tm (a b c : ℝ) (ha : 0 ≤ a) (hb : 0 ≤ b) (hc : 0 ≤ c) : a^2+b^2+c^2 ≥ a*b+b*c+c*a := by",
            "(a b c : ℝ) (ha : 0 ≤ a) (hb : 0 ≤ b) (hc : 0 ≤ c) ⊢ a^2+b^2+c^2 ≥ a*b+b*c+c*a",
        )),
        "tm_powineq" => Some((
            "import Mathlib\ntheorem tm (n : ℕ) (hn : 1 ≤ n) : n + 1 ≤ 2^n := by",
            "(n : ℕ) (hn : 1 ≤ n) ⊢ n + 1 ≤ 2^n",
        )),
        // Genuinely multi-step (induction → sum_range_succ → ih → push_cast/ring): with enforced
        // atomicity these are 4-5 node chains where the middle steps (rw/push_cast) dead-end if
        // wrong → the search band where tree branching + pooling can beat a single DFS chain.
        "tm_sq" => Some((
            "import Mathlib\nopen Finset in\ntheorem tm (n : ℕ) : (∑ i ∈ Finset.range (n+1), (i:ℤ)^2) * 6 = n*(n+1)*(2*n+1) := by",
            "(n : ℕ) ⊢ (∑ i ∈ Finset.range (n+1), (i:ℤ)^2) * 6 = n*(n+1)*(2*n+1)",
        )),
        "tm_geom" => Some((
            "import Mathlib\nopen Finset in\ntheorem tm (n : ℕ) : (∑ i ∈ Finset.range n, (2:ℤ)^i) + 1 = 2^n := by",
            "(n : ℕ) ⊢ (∑ i ∈ Finset.range n, (2:ℤ)^i) + 1 = 2^n",
        )),
        _ => None,
    }
}

const BYPASS: [&str; 2] = ["sorry", "admit"];

#[derive(Clone)]
struct Node {
    parent: Option<usize>,
    tactics: Vec<String>,
    goals: String, // remaining goal text ("" never — root holds the initial goal)
    n_goals: usize,
    stuck: u32,    // failed expansion attempts from this node (crude dead-end signal)
    depth: usize,
}

enum Eval {
    /// carries the EXACT source string Lean verified, so the emitted proof re-verifies byte-identically
    Omega { source: String },
    Partial { goals: String, n: usize },
    Invalid,
}

fn lean_path(mathlib_dir: &Path) -> Option<String> {
    let out = std::process::Command::new(format!("{}/.elan/bin/lake", std::env::var("HOME").ok()?))
        .args(["env", "printenv", "LEAN_PATH"])
        .current_dir(mathlib_dir)
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn default_lean_bin() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    let p = PathBuf::from(&home).join(".elan/toolchains/leanprover--lean4---v4.24.0/bin/lean");
    if p.exists() {
        p
    } else {
        PathBuf::from("lean")
    }
}

/// Run preamble + tactic script under Lean; classify the result.
fn eval_proof(
    preamble: &str,
    tactics: &[String],
    lean_bin: &Path,
    mathlib_dir: &Path,
    lp: &str,
    tag: &str,
) -> Eval {
    // Indent EVERY line of EACH tactic (deepseek often returns multi-line tactics like
    // `induction n with | zero => ... | succ k ih => ...`; indenting only the first line
    // breaks Lean's whitespace-sensitive tactic block → spurious Invalid → tree never grows).
    let body = tactics
        .iter()
        .map(|t| {
            t.lines()
                .map(|l| format!("  {l}"))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let src = format!("{preamble}\n{body}\n");
    if BYPASS.iter().any(|b| src.to_lowercase().contains(b)) {
        return Eval::Invalid;
    }
    let file = std::env::temp_dir().join(format!("tmtree_{tag}.lean"));
    if std::fs::write(&file, &src).is_err() {
        return Eval::Invalid;
    }
    let out = match std::process::Command::new(lean_bin)
        .arg(&file)
        .current_dir(mathlib_dir)
        .env("LEAN_PATH", lp)
        .output()
    {
        Ok(o) => o,
        Err(_) => return Eval::Invalid,
    };
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let low = text.to_lowercase();
    if out.status.success() && !low.contains("sorry") && !low.contains("error") {
        return Eval::Omega { source: src };
    }
    // Partial iff the ONLY error is "unsolved goals" (no tactic/elab error).
    let other_error = low.contains("error")
        && (low.contains("failed")
            || low.contains("unknown")
            || low.contains("unexpected")
            || low.contains("type mismatch")
            || low.contains("invalid"));
    if low.contains("unsolved goals") && !other_error {
        // extract goal text after the "unsolved goals" marker
        let idx = low.find("unsolved goals").unwrap();
        let tail = &text[idx..];
        let goals = tail
            .lines()
            .skip(1)
            .take(40)
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();
        let n = tail.matches('⊢').count().max(1);
        return Eval::Partial { goals, n };
    }
    Eval::Invalid
}

/// True if the tactic is COMPOUND (closes multiple steps/cases at once) → rejected so proofs
/// must be built one atomic step at a time (genuine tree). `| arm =>` case blocks and `<;>`
/// combinators are compound; `rcases h with ⟨a,b⟩` (no `=>`) is atomic and allowed.
fn is_compound_tactic(t: &str) -> bool {
    let s = t.trim();
    s.contains("<;>")
        || s.contains("=>")              // case arms / match arms / `with | ... =>`
        || s.contains('\n')              // multi-line = multiple steps
        || s.contains(';')               // tactic sequencing
        || (s.contains(" with ") && s.contains('|')) // induction/cases ... with | ...
}

fn extract_json(s: &str) -> Option<serde_json::Value> {
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(s.trim()) {
        return Some(v);
    }
    let a = s.find('{')?;
    let b = s.rfind('}')?;
    serde_json::from_str(&s[a..=b]).ok()
}

struct Args {
    theorem: String,
    policy: String,
    n_agents: usize,
    n_rounds: usize,
    seed: u64,
    temp: f64,
    proxy: String,
    model: String,
    mathlib_dir: PathBuf,
    out: PathBuf,
}

fn parse_args() -> Result<Args, String> {
    let a: Vec<String> = std::env::args().collect();
    let get = |k: &str| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1).cloned());
    Ok(Args {
        theorem: get("--theorem").ok_or("--theorem required")?,
        policy: get("--policy").unwrap_or_else(|| "market".into()),
        n_agents: get("--n-agents").and_then(|s| s.parse().ok()).unwrap_or(4),
        n_rounds: get("--n-rounds").and_then(|s| s.parse().ok()).unwrap_or(8),
        seed: get("--seed").and_then(|s| s.parse().ok()).unwrap_or(1),
        temp: get("--temp").and_then(|s| s.parse().ok()).unwrap_or(0.25),
        proxy: get("--proxy").unwrap_or_else(|| "http://localhost:8123".into()),
        model: get("--model").unwrap_or_else(|| "deepseek-chat".into()),
        mathlib_dir: get("--mathlib-dir").map(Into::into).ok_or("--mathlib-dir required")?,
        out: get("--out").map(Into::into).unwrap_or_else(|| "/tmp/tmtree.json".into()),
    })
}

/// progress-based value in [0,1]: more goals closed (relative to root) − stuck penalty.
fn value(node: &Node, root_goals: usize) -> f64 {
    let rg = root_goals.max(1) as f64;
    let progress = (rg - node.n_goals as f64).max(0.0) / rg + (node.depth as f64) * 0.05;
    (progress - 0.4 * node.stuck as f64).max(0.01)
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = parse_args()?;
    let (preamble, init_goal) = theorem(&args.theorem).ok_or(format!("unknown theorem {}", args.theorem))?;
    let llm = ResilientLLMClient::new(&args.proxy, 120, 3);
    let lean_bin = default_lean_bin();
    let lp = lean_path(&args.mathlib_dir).unwrap_or_default();
    if lp.is_empty() {
        return Err("Mathlib LEAN_PATH unresolved".into());
    }
    let mut rng = StdRng::seed_from_u64(args.seed);
    let t0 = Instant::now();

    // root node = empty tactics, full goal
    let root_goals_n = init_goal.matches('⊢').count().max(1);
    let mut nodes: Vec<Node> = vec![Node {
        parent: None,
        tactics: vec![],
        goals: init_goal.to_string(),
        n_goals: root_goals_n,
        stuck: 0,
        depth: 0,
    }];
    let mut own_last: std::collections::BTreeMap<usize, usize> = std::collections::BTreeMap::new();
    let mut tokens = 0u64;
    let mut llm_calls = 0usize;
    let mut lean_calls = 0usize;
    let mut omega: Option<usize> = None;
    let mut omega_source: Option<String> = None;
    let mut best_progress = 0usize; // max goals-closed depth reached

    'outer: for round in 0..args.n_rounds {
        for ai in 0..args.n_agents {
            // ---- node (proof-state) selection ----
            let live: Vec<usize> = (0..nodes.len()).collect();
            let pick = match args.policy.as_str() {
                // MARKET: Boltzmann-softmax over node value → distribute across promising partial
                // states incl early ones (backtrack / new branch). Art. II.2.1.
                "market" => {
                    let vals: Vec<f64> = live.iter().map(|&i| value(&nodes[i], root_goals_n)).collect();
                    let maxv = vals.iter().cloned().fold(f64::MIN, f64::max);
                    let t = args.temp.max(1e-6);
                    let w: Vec<f64> = vals.iter().map(|v| ((v - maxv) / t).exp()).collect();
                    let sum: f64 = w.iter().sum();
                    let mut r = rng.gen::<f64>() * sum;
                    let mut chosen = live[0];
                    for (k, &i) in live.iter().enumerate() {
                        r -= w[k];
                        if r <= 0.0 {
                            chosen = i;
                            break;
                        }
                    }
                    chosen
                }
                // SINGLE: DFS — extend this agent's own deepest live node (no cross-branch jump).
                _ => *own_last.get(&ai).unwrap_or(&0),
            };

            // ---- LLM proposes the NEXT tactic given the remaining goals ----
            // Per-agent LENS: distinct agents prefer distinct tactic families, so different
            // branches get explored from the SAME state — the heterogeneity the market pools
            // over (constitution Art. II.2.1: 不能抹杀群体异质性). Homogeneous prompts → every
            // agent proposes the same next tactic → no real branching.
            let lenses = [
                "Prefer INDUCTION (`induction n with | zero => ?_ | succ k ih => ?_`) when the goal is over ℕ.",
                "Prefer REWRITING with a specific Mathlib lemma (`rw [lemma]` / `simp only [lemma]`).",
                "Prefer ALGEBRA/ARITH closers (`ring`, `ring_nf`, `nlinarith [sq_nonneg ..]`, `omega`).",
                "Prefer STRUCTURAL intro/refine (`intro`, `refine`, `constructor`, `obtain`).",
            ];
            // FAIR diversity: market agents SPECIALIZE (lens by agent → heterogeneous pool the
            // market pools over); single ROTATES lenses by round (one agent, SAME tactic
            // diversity, sequential on its own chain). Without this, single with n_agents=1
            // would be locked to lens[0] (induction only) — a crippled, rigged baseline that
            // makes the market look artificially good.
            let lens = if args.policy == "single" {
                lenses[round % lenses.len()]
            } else {
                lenses[ai % lenses.len()]
            };
            let parent = &nodes[pick];
            let prompt = format!(
                "You are proving a Lean 4 (Mathlib) theorem ONE ATOMIC TACTIC AT A TIME. Rules: apply \
                 EXACTLY ONE atomic tactic; do NOT submit a whole proof; NO `<;>`, NO `;` sequencing, \
                 NO `| case =>` arms, NO `=>`. For induction use bare `induction n` (NOT `induction n \
                 with | ...`) — I will then show you each resulting case to solve separately. Examples \
                 of ONE atomic tactic: `intro n`, `induction n`, `simp`, `rw [Finset.sum_range_succ]`, \
                 `push_cast`, `ring`, `nlinarith [sq_nonneg (a-b)]`, `omega`, `constructor`. Your \
                 preferred style: {lens}\n\nTactics applied so far:\n{}\n\nCurrent remaining goal \
                 state:\n{}\n\nGive the single next atomic tactic valid from THIS state. Output ONLY \
                 JSON: {{\"tactic\": \"<one atomic tactic>\"}}. No `sorry`.",
                if parent.tactics.is_empty() { "(none)".into() } else { parent.tactics.join("\n") },
                parent.goals,
            );
            let resp = match llm
                .generate(&GenerateRequest {
                    model: args.model.clone(),
                    messages: vec![Message { role: "user".into(), content: prompt }],
                    temperature: Some(0.6),
                    max_tokens: Some(300),
                })
                .await
            {
                Ok(r) => {
                    tokens += (r.prompt_tokens + r.completion_tokens) as u64;
                    llm_calls += 1;
                    r
                }
                Err(_) => continue,
            };
            let tac = match extract_json(&resp.content).and_then(|v| v.get("tactic").and_then(|x| x.as_str()).map(String::from)) {
                Some(t) if !t.trim().is_empty() => t.trim().to_string(),
                _ => {
                    nodes[pick].stuck += 1;
                    continue;
                }
            };
            // NOTE: atomicity enforcement was tried (is_compound_tactic) and REVERTED — it broke
            // solving (after a bare `induction n` the proof can't close the two named cases
            // without the `with | zero => … | succ => …` form, so BOTH arms went 0/8). Allowing
            // compound tactics is correct: deepseek's partial compound tactics (e.g. close zero,
            // leave succ) create genuine partial states, and different per-agent lenses still
            // produce real branching (market built 14-18-node branch-4-5 trees and solved tm_sumsq
            // 3/5 while single's branch-0 chain solved 0/5). The tree comes from lens diversity +
            // partial-progress states, NOT from forbidding compound steps.
            let _ = is_compound_tactic; // retained for reference; intentionally not enforced

            // ---- apply + Lean-evaluate the new partial proof ----
            let mut tactics = nodes[pick].tactics.clone();
            tactics.push(tac);
            lean_calls += 1;
            let tag = format!("{}_{}_{}_{}", args.theorem, round, ai, nodes.len());
            match eval_proof(preamble, &tactics, &lean_bin, &args.mathlib_dir, &lp, &tag) {
                Eval::Omega { source } => {
                    omega_source = Some(source);
                    let new = Node { parent: Some(pick), tactics, goals: String::new(), n_goals: 0, stuck: 0, depth: nodes[pick].depth + 1 };
                    nodes.push(new);
                    omega = Some(nodes.len() - 1);
                    best_progress = root_goals_n;
                    break 'outer;
                }
                Eval::Partial { goals, n } => {
                    let closed = root_goals_n.saturating_sub(n);
                    best_progress = best_progress.max(closed);
                    let depth = nodes[pick].depth + 1;
                    // only keep a child that made PROGRESS (fewer goals) or went deeper meaningfully
                    let new = Node { parent: Some(pick), tactics, goals, n_goals: n, stuck: 0, depth };
                    nodes.push(new);
                    own_last.insert(ai, nodes.len() - 1);
                }
                Eval::Invalid => {
                    nodes[pick].stuck += 1;
                }
            }
        }
    }

    let wall = t0.elapsed().as_secs_f64();
    let solved = omega.is_some();
    // tree shape: branching = parents with >1 child; max_depth; distinct parents extended.
    let mut child_count: std::collections::BTreeMap<usize, usize> = std::collections::BTreeMap::new();
    for n in &nodes {
        if let Some(p) = n.parent {
            *child_count.entry(p).or_insert(0) += 1;
        }
    }
    let branching_parents = child_count.values().filter(|&&c| c > 1).count();
    let max_children = child_count.values().copied().max().unwrap_or(0);
    let max_depth = nodes.iter().map(|n| n.depth).max().unwrap_or(0);
    let manifest = serde_json::json!({
        "omega_proof": omega_source,
        "schema": "lean_tree_market.v2", "theorem": args.theorem, "policy": args.policy,
        "n_agents": args.n_agents, "n_rounds": args.n_rounds, "seed": args.seed, "temp": args.temp,
        "solved": solved, "best_goals_closed": best_progress, "root_goals": root_goals_n,
        "nodes": nodes.len(), "branching_parents": branching_parents, "max_children": max_children,
        "max_depth": max_depth, "llm_calls": llm_calls, "lean_calls": lean_calls,
        "tokens": tokens, "wall_s": wall,
    });
    let _ = std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).unwrap());
    println!(
        "tree_market[{}] thm={} agents={} rounds={} solved={} goals_closed={}/{} nodes={} llm={} lean={} tokens={} wall={:.1}s",
        args.policy, args.theorem, args.n_agents, args.n_rounds, solved, best_progress, root_goals_n,
        nodes.len(), llm_calls, lean_calls, tokens, wall
    );
    Ok(())
}
