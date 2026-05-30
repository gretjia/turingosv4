//! §4.5 diagnostic — does routing-on-GRADED-progress beat single-agent?
//!
//! H0 showed the market ties/loses single-agent on MONOLITHIC Lean theorems (binary
//! kernel verdict → no partial-progress gradient for price to route on). This probe
//! tests the architect's §4.5 hypothesis on the OPPOSITE structure: a k-part
//! conjunction `C_1 ∧ … ∧ C_k` where progress is GRADED = number of conjuncts whose
//! proof independently Lean-verifies. A node carries its closed-set; the market extends
//! the highest-score node (pooling different agents' partial progress); single-agent
//! accumulates on one chain. Diagnostic only — NOT an OMEGA, NOT a constitutional run.
//!
//! market beats single here  -> §4.5 confirmed: the market's home is graded/decomposable
//!                               tasks -> path B warranted.
//! market still <= single     -> §4.5 refuted -> global NO-GO (path A), stronger.
//!
//! Class 1 (additive diagnostic bin; no §6 surface, no chain/CAS/replay).

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::Instant;
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};

/// Graded theorems: id -> conjunct goal strings (each independently Lean-provable,
/// verified in /tmp/grad_verify.lean before commit). Reference proofs are NOT here —
/// agents must find them; we only check the agent's proof of each conjunct.
fn graded_theorem(id: &str) -> Option<Vec<&'static str>> {
    match id {
        "grad_nt" => Some(vec![
            "Nat.gcd 48 36 = 12",
            "Nat.Coprime 25 14",
            "Nat.Prime 101",
            "(15 : ℕ) ∣ 225",
            "Nat.gcd 0 7 = 7",
            "∀ n : ℕ, n ∣ n ^ 2",
        ]),
        "grad_alg" => Some(vec![
            "∀ a b : ℤ, (a + b) ^ 2 = a ^ 2 + 2 * a * b + b ^ 2",
            "∀ a b : ℤ, a - b = -(b - a)",
            "(2 : ℤ) ^ 10 = 1024",
            "∀ n : ℕ, Even (n + n)",
            "∀ a b c : ℤ, a * (b + c) = a * b + a * c",
            "∀ a b : ℤ, (a - b) * (a + b) = a ^ 2 - b ^ 2",
        ]),
        "grad_ord" => Some(vec![
            "∀ a b : ℕ, a ≤ max a b",
            "∀ a b : ℕ, min a b ≤ a",
            "∀ a : ℕ, a ≤ a + 1",
            "∀ a b : ℕ, a ≤ b → a ≤ b + 1",
            "List.length [1, 2, 3, 4] = 4",
            "∀ a b : ℝ, a ≤ b ∨ b ≤ a",
        ]),
        // grad_hard: the 3 already-calibrated 33%-at-budget headroom theorems AS conjuncts —
        // individually hard (so single rarely closes all 3 in one chain), diverse (different
        // attempts close different ones), so pooling can matter. The §4.5 decisive case.
        "grad_hard" => Some(vec![
            "∀ n : ℕ, (∑ i ∈ Finset.range (n + 1), i ^ 3) * 4 = (n * (n + 1)) ^ 2",
            "StrictMono (fun x : ℝ => x ^ 3 + x)",
            "∀ n : ℕ, Polynomial.eval 2 (∑ i ∈ Finset.range n, (Polynomial.X : Polynomial ℤ) ^ i) = 2 ^ n - 1",
        ]),
        // grad_calibrated: 6 conjuncts of MODERATE single-shot difficulty (induction /
        // nlinarith-with-hint / ring) — the ~33% band where partial progress is graded and
        // different attempts plausibly close different ones. The §4.5 decisive task.
        "grad_calibrated" => Some(vec![
            "∀ n : ℕ, ∑ i ∈ Finset.range n, (2*i+1) = n^2",
            "∀ a b : ℝ, a^2 + b^2 ≥ 2*a*b",
            "∀ a b : ℝ, (a+b)^2 ≤ 2*(a^2+b^2)",
            "∀ a b : ℝ, a*b ≤ (a^2 + b^2)/2",
            "∀ a b c : ℝ, a^2+b^2+c^2 ≥ a*b+b*c+c*a",
            "∀ n : ℕ, n^2 + n = n*(n+1)",
        ]),
        _ => None,
    }
}

// DIAGNOSTIC-ONLY bypass set: ban only `sorry`/`admit` (which prove NOTHING). `decide` /
// `native_decide` are SOUND decision procedures (kernel-trust-bypassing, but they DO decide
// the proposition) — irrelevant to the §4.5 routing question this probe tests, and banning
// them just makes every decidable conjunct artificially fail (deepseek's default is
// native_decide). This is NOT an OMEGA path; the strong no-native_decide rule stays in
// LeanJudge for the real market. Graded differentiation here comes from the ∀/lemma conjuncts.
const BYPASS: [&str; 2] = ["sorry", "admit"];

#[derive(Clone)]
struct Node {
    closed: BTreeSet<usize>,         // conjunct indices proven
    proofs: BTreeMap<usize, String>, // index -> verified proof text
}
impl Node {
    fn score(&self) -> usize {
        self.closed.len()
    }
}

struct Args {
    theorem: String,
    policy: String,
    n_agents: usize,
    n_rounds: usize,
    seed: u64,
    proxy: String,
    model: String,
    mathlib_dir: PathBuf,
    out: PathBuf,
}

fn parse_args() -> Result<Args, String> {
    let a: Vec<String> = std::env::args().collect();
    let get = |k: &str| -> Option<String> {
        a.iter().position(|x| x == k).and_then(|i| a.get(i + 1).cloned())
    };
    Ok(Args {
        theorem: get("--theorem").ok_or("--theorem required")?,
        policy: get("--policy").unwrap_or_else(|| "market".into()),
        n_agents: get("--n-agents").and_then(|s| s.parse().ok()).unwrap_or(4),
        n_rounds: get("--n-rounds").and_then(|s| s.parse().ok()).unwrap_or(2),
        seed: get("--seed").and_then(|s| s.parse().ok()).unwrap_or(1),
        proxy: get("--proxy").unwrap_or_else(|| "http://localhost:8123".into()),
        model: get("--model").unwrap_or_else(|| "deepseek-chat".into()),
        mathlib_dir: get("--mathlib-dir").map(Into::into).ok_or("--mathlib-dir required")?,
        out: get("--out").map(Into::into).unwrap_or_else(|| "/tmp/grad_diag.json".into()),
    })
}

/// Verify one conjunct's proof in isolation under Lean+Mathlib (exit 0 + no sorry).
fn verify_conjunct(goal: &str, proof: &str, lean_bin: &Path, mathlib_dir: &Path, lean_path: &str, tag: &str) -> bool {
    let lower = proof.to_lowercase();
    if BYPASS.iter().any(|b| lower.contains(b)) {
        return false;
    }
    let src = format!("import Mathlib\nopen Finset in\ntheorem diag_{tag} : {goal} := {proof}\n");
    let file = std::env::temp_dir().join(format!("gradc_{tag}.lean"));
    if std::fs::write(&file, &src).is_err() {
        return false;
    }
    match std::process::Command::new(lean_bin)
        .arg(&file)
        .current_dir(mathlib_dir)
        .env("LEAN_PATH", lean_path)
        .output()
    {
        Ok(o) => {
            let out = format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
            o.status.success() && !out.to_lowercase().contains("sorry")
        }
        Err(_) => false,
    }
}

fn lean_path(mathlib_dir: &Path) -> Option<String> {
    let out = std::process::Command::new(format!("{}/.elan/bin/lake", std::env::var("HOME").ok()?))
        .args(["env", "printenv", "LEAN_PATH"])
        .current_dir(mathlib_dir)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        None
    }
}

fn default_lean_bin() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    let pinned = PathBuf::from(&home)
        .join(".elan/toolchains/leanprover--lean4---v4.24.0/bin/lean");
    if pinned.exists() {
        pinned
    } else {
        PathBuf::from("lean")
    }
}

fn extract_json(s: &str) -> Option<serde_json::Value> {
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(s.trim()) {
        return Some(v);
    }
    let start = s.find('{')?;
    let end = s.rfind('}')?;
    serde_json::from_str(&s[start..=end]).ok()
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = parse_args()?;
    let goals = graded_theorem(&args.theorem).ok_or(format!("unknown theorem {}", args.theorem))?;
    let k = goals.len();
    let llm = ResilientLLMClient::new(&args.proxy, 120, 3);
    let lean_bin = default_lean_bin();
    let lp = lean_path(&args.mathlib_dir).unwrap_or_default();
    if lp.is_empty() {
        return Err("could not resolve Mathlib LEAN_PATH (lake env failed)".into());
    }
    let t0 = Instant::now();

    let mut nodes: Vec<Node> = Vec::new();
    let mut own_last: BTreeMap<usize, usize> = BTreeMap::new(); // agent -> node idx (single/own-chain)
    let mut rng = StdRng::seed_from_u64(args.seed);
    let mut tokens = 0u64;
    let mut llm_calls = 0usize;
    let mut verify_calls = 0usize;
    let mut best_score = 0usize;

    'outer: for round in 0..args.n_rounds {
        for ai in 0..args.n_agents {
            // ---- parent selection by policy ----
            let parent: Option<usize> = if nodes.is_empty() {
                None
            } else {
                match args.policy.as_str() {
                    // MARKET: extend the highest-score node (price = graded progress), with
                    // epsilon exploration. This is the mechanism §4.5 predicts should win.
                    "market" => {
                        if rng.gen_bool(0.2) {
                            Some(rng.gen_range(0..nodes.len()))
                        } else {
                            Some((0..nodes.len()).max_by_key(|&i| nodes[i].score()).unwrap())
                        }
                    }
                    // A0: price signal destroyed — uniform-random parent.
                    "shuffled" => Some(rng.gen_range(0..nodes.len())),
                    // single / own-chain: extend only this agent's last node.
                    _ => own_last.get(&ai).copied(),
                }
            };
            let base = parent.map(|i| nodes[i].clone()).unwrap_or(Node {
                closed: BTreeSet::new(),
                proofs: BTreeMap::new(),
            });

            // ---- which conjuncts remain ----
            let remaining: Vec<usize> = (0..k).filter(|i| !base.closed.contains(i)).collect();
            if remaining.is_empty() {
                best_score = k;
                break 'outer;
            }

            // ---- prompt: close ONE more conjunct ----
            let goal_list: String = goals
                .iter()
                .enumerate()
                .map(|(i, g)| {
                    let mark = if base.closed.contains(&i) { "[DONE]" } else { "[OPEN]" };
                    format!("  {i}. {mark} {g}")
                })
                .collect::<Vec<_>>()
                .join("\n");
            let prompt = format!(
                "You are proving conjuncts of a Lean 4 + Mathlib theorem, one at a time. The target \
                 is a conjunction of {k} independent propositions:\n{goal_list}\n\nPick ONE [OPEN] \
                 conjunct and give a Lean 4 proof of THAT proposition alone (a term or `by ...`). \
                 Output ONLY JSON: {{\"index\": <i>, \"proof\": \"<lean proof of conjunct i>\"}}. \
                 The proof must compile as `theorem t : <conjunct i> := <proof>`. No `sorry`.",
            );
            let resp = match llm
                .generate(&GenerateRequest {
                    model: args.model.clone(),
                    messages: vec![Message { role: "user".into(), content: prompt }],
                    temperature: Some(0.4),
                    max_tokens: Some(400),
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

            // ---- parse + verify the claimed conjunct ----
            let mut node = base.clone();
            if let Some(v) = extract_json(&resp.content) {
                let idx = v.get("index").and_then(|x| x.as_u64()).map(|x| x as usize);
                let proof = v.get("proof").and_then(|x| x.as_str()).map(|s| s.to_string());
                if let (Some(idx), Some(proof)) = (idx, proof) {
                    if idx < k && !node.closed.contains(&idx) {
                        verify_calls += 1;
                        let tag = format!("{}_{}_{}_{}", args.theorem, round, ai, idx);
                        if verify_conjunct(goals[idx], &proof, &lean_bin, &args.mathlib_dir, &lp, &tag) {
                            node.closed.insert(idx);
                            node.proofs.insert(idx, proof);
                        }
                    }
                }
            }

            let new_idx = nodes.len();
            best_score = best_score.max(node.score());
            nodes.push(node);
            own_last.insert(ai, new_idx);
            if best_score == k {
                break 'outer;
            }
        }
    }

    let wall = t0.elapsed().as_secs_f64();
    let solved = best_score == k;
    let manifest = serde_json::json!({
        "schema": "lean_graded_diag.v1",
        "theorem": args.theorem, "k": k, "policy": args.policy,
        "n_agents": args.n_agents, "n_rounds": args.n_rounds, "seed": args.seed,
        "best_score": best_score, "solved": solved,
        "llm_calls": llm_calls, "verify_calls": verify_calls,
        "tokens": tokens, "wall_s": wall, "nodes": nodes.len(),
    });
    let _ = std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).unwrap());
    println!(
        "graded_diag[{}] thm={} k={} agents={} rounds={} best_score={}/{} solved={} llm={} verify={} tokens={} wall={:.1}s",
        args.policy, args.theorem, k, args.n_agents, args.n_rounds, best_score, k, solved,
        llm_calls, verify_calls, tokens, wall
    );
    Ok(())
}
