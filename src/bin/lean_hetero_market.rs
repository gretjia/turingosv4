//! Heterogeneous-agent market — the ONE untested market premise (the likeliest real emergence).
//!
//! Prior runs used a HOMOGENEOUS pool (one model, varied only by temperature/prompt-lens): any
//! agent could attempt anything any other could, so the market's "combine complementary agents"
//! premise was never actually exercised, and market never beat single (finding C; the tree-search
//! aggregate 0/24 vs 2/24). This tests the real claim: a pool of SPECIALISTS, none of which can
//! finish alone, where the MARKET combines their complementary partial contributions to solve what
//! no single specialist solves — and beats a single GENERALIST agent at equal budget.
//!
//! Task: a conjunction C1 ∧ C2 ∧ C3 ∧ C4 where each conjunct needs a DIFFERENT tactic family
//! (omega / ring / induction / nlinarith). Score = conjuncts independently Lean-verified.
//!
//! Arms:
//!  market    : N specialist agents (agent i locked to tactic family i); the market routes each
//!              OPEN conjunct to a specialist whose family fits it (which specialist self-selects
//!              via SKIP). Combines complementary specialists. OMEGA = all conjuncts closed.
//!
//!  CORRECTION 2026-06-01 (forensic retrospective §1.A): an earlier framing implied a market
//!  price selected the specialist. That was a NAME-LIE — this bin has ZERO market / Invest /
//!  wallet machinery (grep: no `price` identifier in code). Selection is round-robin over the
//!  roster + SKIP self-selection, NOT a loss-bearing market. The reported "market 3.81 > single
//!  3.00 > single_spec 1.50 PROVEN" was coverage / prompt-shaping and was ERASED at equal budget
//!  (Stage-2 JUST_SAMPLING). Do NOT cite this bin as price-market evidence.
//!  single    : ONE generalist agent (all tactic families) refining sequentially — the honest
//!              control ("can one capable agent just do it all?").
//!  single_spec: ONE specialist (family 0 only) — sanity floor: must cap at 1/4 (proves no single
//!              specialist can finish → the market's win is genuinely from COMBINING).
//!
//! market > single_generalist  → emergence from combining complementary limited agents (the
//!                               constitution's actual thesis), not "a giant model wins".
//! Diagnostic-grade (no chain/CAS/replay; allows decide/native_decide — irrelevant to the routing
//! question). The strong no-sorry rule stays for any conjunct's accepted proof.
//!
//! Class 1 additive bin.

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::Instant;
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};

/// (conjunct goal text, the tactic FAMILY that closes it). Families: omega/ring/induction/nlinarith.
fn task(id: &str) -> Option<Vec<(&'static str, &'static str)>> {
    match id {
        // het4: 4 conjuncts, 4 distinct families. Verified provable in /tmp.
        "het4" => Some(vec![
            ("2 * n + 3 ≤ 5 * n + 3 + 1", "omega"),
            ("(a + b)^2 = a^2 + 2*a*b + b^2", "ring"),
            ("(∑ i ∈ Finset.range (n+1), (i:ℤ)) * 2 = n * (n+1)", "induction"),
            ("a^2 + b^2 ≥ 2*a*b", "nlinarith"),
        ]),
        // het6: harder, 6 conjuncts across the same 4 families (some families repeat).
        "het6" => Some(vec![
            ("3 * n + 1 ≤ 7 * n + 2", "omega"),
            ("(a - b)^2 = a^2 - 2*a*b + b^2", "ring"),
            ("(∑ i ∈ Finset.range (n+1), (i:ℤ)) * 2 = n * (n+1)", "induction"),
            ("a^2 + b^2 ≥ 2*a*b", "nlinarith"),
            ("n + 5 ≤ 2 * n + 5 + n", "omega"),
            ("(a + b) * (a - b) = a^2 - b^2", "ring"),
        ]),
        _ => None,
    }
}

const PREAMBLE_VARS: &str = "(n : ℕ) (a b : ℝ)";

/// The tactic families. An agent specialized to family k may ONLY emit tactics of that family;
/// a generalist may emit any.
const FAMILIES: [&str; 4] = ["omega", "ring", "induction", "nlinarith"];

fn family_hint(fam: &str) -> &'static str {
    match fam {
        "omega" => "You may ONLY use the `omega` tactic (linear integer/nat arithmetic). If the goal is not linear arithmetic, output {\"tactic\":\"SKIP\"}.",
        "ring" => "You may ONLY use `ring` or `ring_nf` (commutative-ring identities). If the goal is not a ring identity, output {\"tactic\":\"SKIP\"}.",
        "induction" => "You may ONLY use an `induction n with | zero => ... | succ k ih => ...` proof (for goals over ℕ, e.g. Finset sums). If induction does not apply, output {\"tactic\":\"SKIP\"}.",
        "nlinarith" => "You may ONLY use `nlinarith [...]` with square hints like `sq_nonneg (a-b)` (nonlinear real inequalities). If the goal is not such an inequality, output {\"tactic\":\"SKIP\"}.",
        _ => "Use any single appropriate tactic.",
    }
}

fn lean_path(mathlib_dir: &Path) -> Option<String> {
    let out = std::process::Command::new(format!("{}/.elan/bin/lake", std::env::var("HOME").ok()?))
        .args(["env", "printenv", "LEAN_PATH"])
        .current_dir(mathlib_dir)
        .output()
        .ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn default_lean_bin() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    let p = PathBuf::from(&home).join(".elan/toolchains/leanprover--lean4---v4.24.0/bin/lean");
    if p.exists() { p } else { PathBuf::from("lean") }
}

/// Verify one conjunct's proof in isolation (full theorem with the conjunct as the goal).
fn verify_conjunct(goal: &str, proof: &str, lean_bin: &Path, mathlib_dir: &Path, lp: &str, tag: &str) -> bool {
    let low = proof.to_lowercase();
    if low.contains("sorry") || low.contains("admit") {
        return false;
    }
    // wrap: the conjunct may mention n,a,b → bind them all.
    let src = format!(
        "import Mathlib\nopen Finset in\ntheorem c_{tag} {PREAMBLE_VARS} : {goal} := by\n{}\n",
        proof.lines().map(|l| format!("  {l}")).collect::<Vec<_>>().join("\n")
    );
    let file = std::env::temp_dir().join(format!("het_{tag}.lean"));
    if std::fs::write(&file, &src).is_err() {
        return false;
    }
    match std::process::Command::new(lean_bin).arg(&file).current_dir(mathlib_dir).env("LEAN_PATH", lp).output() {
        Ok(o) => {
            let t = format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr)).to_lowercase();
            o.status.success() && !t.contains("sorry") && !t.contains("error")
        }
        Err(_) => false,
    }
}

fn extract_tactic(s: &str) -> Option<String> {
    let v: serde_json::Value = if let Ok(v) = serde_json::from_str(s.trim()) {
        v
    } else {
        let a = s.find('{')?;
        let b = s.rfind('}')?;
        serde_json::from_str(&s[a..=b]).ok()?
    };
    v.get("tactic").and_then(|x| x.as_str()).map(|s| s.trim().to_string())
}

struct Args {
    task: String,
    policy: String,
    n_rounds: usize,
    seed: u64,
    proxy: String,
    model: String,
    mathlib_dir: PathBuf,
    out: PathBuf,
}

fn parse_args() -> Result<Args, String> {
    let a: Vec<String> = std::env::args().collect();
    let get = |k: &str| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1).cloned());
    Ok(Args {
        task: get("--task").ok_or("--task required")?,
        policy: get("--policy").unwrap_or_else(|| "market".into()),
        n_rounds: get("--n-rounds").and_then(|s| s.parse().ok()).unwrap_or(8),
        seed: get("--seed").and_then(|s| s.parse().ok()).unwrap_or(1),
        proxy: get("--proxy").unwrap_or_else(|| "http://localhost:8123".into()),
        model: get("--model").unwrap_or_else(|| "deepseek-chat".into()),
        mathlib_dir: get("--mathlib-dir").map(Into::into).ok_or("--mathlib-dir required")?,
        out: get("--out").map(Into::into).unwrap_or_else(|| "/tmp/het.json".into()),
    })
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = parse_args()?;
    let conjuncts = task(&args.task).ok_or(format!("unknown task {}", args.task))?;
    let k = conjuncts.len();
    let llm = ResilientLLMClient::new(&args.proxy, 120, 3);
    let lean_bin = default_lean_bin();
    let lp = lean_path(&args.mathlib_dir).unwrap_or_default();
    if lp.is_empty() {
        return Err("Mathlib LEAN_PATH unresolved".into());
    }
    let mut rng = StdRng::seed_from_u64(args.seed);
    let t0 = Instant::now();

    // agent roster by policy
    let agents: Vec<Option<&str>> = match args.policy.as_str() {
        // market: 4 specialists, one per family
        "market" => FAMILIES.iter().map(|f| Some(*f)).collect(),
        // single generalist: one agent, no family lock
        "single" => vec![None],
        // single specialist floor: one agent locked to family 0
        "single_spec" => vec![Some(FAMILIES[0])],
        other => return Err(format!("unknown policy {other}")),
    };

    let mut closed: BTreeSet<usize> = BTreeSet::new();
    let mut proofs: Vec<String> = vec![String::new(); k];
    let mut llm_calls = 0usize;
    let mut lean_calls = 0usize;
    let mut skips = 0usize;
    let mut tokens = 0u64;
    // budget parity: total expansions = roster_len * n_rounds, but single gets the SAME total.
    let total_expansions = match args.policy.as_str() {
        "market" => agents.len() * args.n_rounds,
        _ => agents.len() * args.n_rounds * 4, // single: 1 agent but 4x rounds to equal market's 4 specialists
    };

    'outer: for step in 0..total_expansions {
        // pick an agent (round-robin over roster) and an OPEN conjunct.
        let ai = step % agents.len();
        let fam = agents[ai];
        let open: Vec<usize> = (0..k).filter(|i| !closed.contains(i)).collect();
        if open.is_empty() {
            break;
        }
        // HONEST routing: agents are NOT told which conjunct fits them. Each takes a random OPEN
        // conjunct and SELF-SELECTS via SKIP — a specialist whose family doesn't apply outputs SKIP
        // (cheap, no Lean call), so over rounds each conjunct is naturally attempted by every
        // specialist until the matching one closes it. The "routing" is emergent from specialists
        // declining work outside their skill, NOT from the harness assigning matches. (This is the
        // market's value: heterogeneous agents self-allocate to where their skill fits.)
        let target = open[rng.gen_range(0..open.len())];

        let (goal, _truth_fam) = conjuncts[target];
        let role = match fam {
            Some(f) => format!("You are a SPECIALIST agent. {}", family_hint(f)),
            None => "You are a generalist Lean 4 prover; use any single appropriate tactic.".to_string(),
        };
        let prompt = format!(
            "{role}\n\nProve this Lean 4 (Mathlib) goal. Context binds `(n : ℕ) (a b : ℝ)`.\n\nGoal:\n{goal}\n\nGive a Lean proof (a `by`-block body, one or more tactics) that closes THIS goal. Output ONLY JSON: {{\"tactic\": \"<proof body>\"}}. If your specialty does not apply, output {{\"tactic\": \"SKIP\"}}. No `sorry`.",
        );
        let resp = match llm.generate(&GenerateRequest {
            model: args.model.clone(),
            messages: vec![Message { role: "user".into(), content: prompt }],
            temperature: Some(0.3),
            max_tokens: Some(300),
        }).await {
            Ok(r) => { tokens += (r.prompt_tokens + r.completion_tokens) as u64; llm_calls += 1; r }
            Err(_) => continue,
        };
        let tac = match extract_tactic(&resp.content) {
            Some(t) if !t.is_empty() && t.to_uppercase() != "SKIP" => t,
            _ => { skips += 1; continue; }
        };
        // ENFORCE the specialty at the HARNESS (deepseek ignores the prompt's "only X" rule):
        // a specialist's proof MUST use its locked tactic family and NOT another family's closer.
        // This makes specialists genuinely LIMITED, so a single specialist provably caps below k
        // and any market win is real complementary combination. Generalist (None) is unconstrained.
        if let Some(f) = fam {
            let low = tac.to_lowercase();
            let uses_own = low.contains(f);
            // reject if it reaches for a DIFFERENT family's signature closer
            let foreign = FAMILIES.iter().any(|g| *g != f && low.contains(g));
            if !uses_own || foreign {
                skips += 1;
                continue;
            }
        }
        lean_calls += 1;
        let tag = format!("{}_{}_{}_{}", args.task, args.policy, step, target);
        if verify_conjunct(goal, &tac, &lean_bin, &args.mathlib_dir, &lp, &tag) {
            closed.insert(target);
            proofs[target] = tac;
            if closed.len() == k {
                break 'outer;
            }
        }
    }

    let wall = t0.elapsed().as_secs_f64();
    let solved = closed.len() == k;
    let manifest = serde_json::json!({
        "schema": "lean_hetero_market.v1", "task": args.task, "policy": args.policy,
        "k": k, "closed": closed.len(), "solved": solved, "n_rounds": args.n_rounds,
        "seed": args.seed, "total_expansions": total_expansions,
        "llm_calls": llm_calls, "lean_calls": lean_calls, "skips": skips, "tokens": tokens, "wall_s": wall,
    });
    let _ = std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).unwrap());
    println!(
        "hetero[{}] task={} closed={}/{} solved={} expansions={} llm={} lean={} skips={} tokens={} wall={:.1}s",
        args.policy, args.task, closed.len(), k, solved, total_expansions, llm_calls, lean_calls, skips, tokens, wall
    );
    Ok(())
}
