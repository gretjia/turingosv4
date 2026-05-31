//! LEAN-EMERGENCE — the strong-model capability study (Stage 1: p-vector + joint hit matrix).
//!
//! Architect's thrust: approach HARD theorems with STRONG models. Two capabilities, both measured here:
//!   (A) verifier-backed COVERAGE — strong models have p>0 but high VARIANCE (a draw mis-emits Lean4 syntax,
//!       the next draws `simp`); a free Lean #print-axioms verifier picking ANY correct sample makes
//!       pass@k = 1-(1-p)^k climb with no Condorcet 1/2 ceiling. (Field consensus / table-stakes.)
//!   (B) heterogeneous strong-model COMBINATION — the OPEN lane no SOTA prover does (all are single-backbone):
//!       does combining distinct foundation models (different training → complementary errors) solve theorems
//!       NO single model solves alone? Stage-1 measures the JOINT HIT MATRIX that identifies the
//!       combination-target set (theorems in the union but in no single model's solo set) for Stage 2.
//!
//! Stage-1 = for each STRONG model × theorem, draw k samples (multi-round Lean-feedback per attempt — proven
//! necessary, single-shot under-measures p), verify every accepted proof under a REAL #print-axioms gate
//! (whitelist {propext, Classical.choice, Quot.sound}; reject sorryAx/native_decide), record the
//! per-(model,theorem) hit count. Output: per-model pass@k curve (A) + the joint hit matrix (B partition).
//!
//! Class 1-2 diagnostic bin; real Lean ground truth; FC1/2/3 untouched; no §6. Small models (--shakeout)
//! are for harness bug-shakeout only (mathematically futile on the graduate band).

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};

// the axiom whitelist: a proof whose footprint ⊄ this set is rejected (catches sorryAx + native_decide trust).
const AXIOM_WHITELIST: [&str; 3] = ["propext", "Classical.choice", "Quot.sound"];

struct Args {
    bank: PathBuf,
    models: Vec<String>,       // the strong models to measure (CSV via --models)
    k_samples: usize,          // draws per (model, theorem)
    max_rounds: usize,         // Lean-feedback rounds per draw
    temp: f64,
    seed: u64,
    n_theorems: usize,         // 0 = all
    proxy: String,
    mathlib_dir: PathBuf,
    out: PathBuf,
    resume: bool,              // if set, preload <out>.partial.json and skip already-computed (model,theorem) cells
}

fn parse_args() -> Result<Args, String> {
    let a: Vec<String> = std::env::args().collect();
    let get = |k: &str| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1).cloned());
    Ok(Args {
        bank: get("--bank").map(Into::into).unwrap_or_else(|| "tests/fixtures/lean_theorems_pool.jsonl".into()),
        models: get("--models").map(|s| s.split(',').map(|x| x.trim().to_string()).collect()).unwrap_or_else(|| vec!["deepseek-reasoner".into()]),
        k_samples: get("--k").and_then(|s| s.parse().ok()).unwrap_or(8),
        max_rounds: get("--max-rounds").and_then(|s| s.parse().ok()).unwrap_or(2),
        temp: get("--temp").and_then(|s| s.parse().ok()).unwrap_or(0.7),
        seed: get("--seed").and_then(|s| s.parse().ok()).unwrap_or(1),
        n_theorems: get("--n-theorems").and_then(|s| s.parse().ok()).unwrap_or(0),
        proxy: get("--proxy").unwrap_or_else(|| "http://localhost:8123".into()),
        mathlib_dir: get("--mathlib-dir").map(Into::into).ok_or("--mathlib-dir required")?,
        out: get("--out").map(Into::into).unwrap_or_else(|| "/tmp/emergence.json".into()),
        resume: a.iter().any(|x| x == "--resume"),
    })
}

struct Thm { id: String, preamble: String }
fn load_bank(path: &Path, n: usize) -> Vec<Thm> {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with("//") { continue; }
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(t) {
            if let (Some(id), Some(p)) = (v["id"].as_str(), v["preamble"].as_str()) {
                out.push(Thm { id: id.into(), preamble: p.into() });
            }
        }
    }
    if n > 0 && n < out.len() { out.truncate(n); }
    out
}

fn lean_path(mathlib_dir: &Path) -> Option<String> {
    let out = std::process::Command::new(format!("{}/.elan/bin/lake", std::env::var("HOME").ok()?))
        .args(["env", "printenv", "LEAN_PATH"]).current_dir(mathlib_dir).output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}
fn default_lean_bin() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    let p = PathBuf::from(&home).join(".elan/toolchains/leanprover--lean4---v4.24.0/bin/lean");
    if p.exists() { p } else { PathBuf::from("lean") }
}

/// Sidecar checkpoint path for the run: `<out>` + `.partial.json`. Written after each model completes
/// its theorem loop so a crash/sleep/kill mid-run leaves completed models' hit counts recoverable.
fn partial_path(out: &Path) -> PathBuf {
    let mut s = out.as_os_str().to_os_string();
    s.push(".partial.json");
    PathBuf::from(s)
}

/// Verify a candidate proof body under Lean WITH the real #print-axioms gate. Returns (clean, feedback).
/// clean iff: compiles, no error, no sorry/admit/native_decide in source, AND axiom footprint ⊆ whitelist.
fn verify_axiom_clean(preamble: &str, body: &str, lean_bin: &Path, mathlib_dir: &Path, lp: &str, tag: &str) -> (bool, String) {
    let low = body.to_lowercase();
    if low.contains("sorry") || low.contains("admit") || low.contains("native_decide") {
        return (false, "source contains sorry/admit/native_decide".into());
    }
    // name the theorem so we can `#print axioms` it. The preamble ends with `:= by` for an anon `theorem`/`example`;
    // rewrite the head to a named theorem `_emerge`.
    let head = preamble.trim_end();
    // strip a trailing ":= by" and the leading `theorem <name...>`/`example` → re-emit as `theorem _emerge ... := by`
    let body_indented: String = body.lines().map(|l| format!("  {l}")).collect::<Vec<_>>().join("\n");
    // ROBUST naming (the prior splitn logic corrupted `theorem name {M:Type*}` signatures → even REFERENCE
    // proofs failed = every p was a harness artifact). Strategy: KEEP the statement verbatim; reuse the
    // existing theorem name if present (just append `#print axioms <name>`); only inject a name for an
    // anonymous `example`, line-anchored so we don't touch the word "example" inside the goal.
    let (named, thm_name) = if let Some(name) = extract_theorem_name(head) {
        (head.to_string(), name)
    } else {
        // anonymous example → rename the FIRST `example` token at a line start to a named theorem.
        let renamed = head.replacen("example", "theorem _emerge", 1);
        (renamed, "_emerge".to_string())
    };
    let src = format!("{named}\n{body_indented}\n#print axioms {thm_name}\n");
    let file = std::env::temp_dir().join(format!("emerge_{tag}.lean"));
    if std::fs::write(&file, &src).is_err() { return (false, "write fail".into()); }
    let out = match std::process::Command::new(lean_bin).arg(&file).current_dir(mathlib_dir).env("LEAN_PATH", lp).output() {
        Ok(o) => o, Err(_) => return (false, "lean spawn fail".into()),
    };
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    let low_t = text.to_lowercase();
    if !out.status.success() || low_t.contains("error") {
        // return a bounded Lean error as retry feedback (the public diagnostic on the agent's own candidate).
        let fb: String = text.lines().filter(|l| l.to_lowercase().contains("error")).take(2).collect::<Vec<_>>().join(" | ");
        return (false, fb.chars().take(240).collect());
    }
    // parse the `'_emerge' depends on axioms: [a, b, ...]` line.
    if let Some(idx) = text.find("depends on axioms:") {
        let tail = &text[idx..];
        let inside = tail.find('[').and_then(|s| tail.find(']').map(|e| &tail[s + 1..e])).unwrap_or("");
        let axs: Vec<String> = inside.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        let bad: Vec<&String> = axs.iter().filter(|a| !AXIOM_WHITELIST.contains(&a.as_str())).collect();
        if bad.is_empty() { return (true, format!("axioms {axs:?}")); }
        return (false, format!("non-whitelist axioms: {bad:?}"));
    }
    // compiled clean but no axiom line (e.g. fully constructive) → accept.
    (true, "no axiom dependency".into())
}

/// Extract the existing theorem name from a preamble (the token after `theorem`/`lemma`), if any.
/// Returns None for an anonymous `example`. Names stop at whitespace, `(`, `{`, `[`, or `:`.
fn extract_theorem_name(preamble: &str) -> Option<String> {
    for kw in ["theorem ", "lemma "] {
        if let Some(i) = preamble.find(kw) {
            let after = &preamble[i + kw.len()..];
            let name: String = after.chars().take_while(|c| !c.is_whitespace() && !matches!(c, '(' | '{' | '[' | ':')).collect();
            if !name.is_empty() { return Some(name); }
        }
    }
    None
}

/// Extract the proof body from a model reply. Robust to the REAL failure modes observed with strong
/// reasoners emitting LONG multi-line proofs: (1) clean JSON, (2) markdown ```json fences, (3) bare
/// literal newlines INSIDE the JSON string value (a formatting slip that makes serde reject the whole
/// object — the bug that mis-measured reasoner p as ~0). The last fallback regex-grabs the value between
/// `"tactic":` and the final `"` and un-escapes \n/\t/\", so a model that "knows the proof" is not lost
/// to a JSON quoting slip.
fn extract_tactic(s: &str) -> Option<String> {
    let cleaned = s.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
    // 1) strict JSON
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(cleaned) {
        if let Some(t) = v.get("tactic").and_then(|x| x.as_str()) { return Some(t.trim().to_string()); }
    }
    // 2) the {...} substring as strict JSON
    if let (Some(a), Some(b)) = (cleaned.find('{'), cleaned.rfind('}')) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&cleaned[a..=b]) {
            if let Some(t) = v.get("tactic").and_then(|x| x.as_str()) { return Some(t.trim().to_string()); }
        }
    }
    // 3) TOLERANT: grab everything between the FIRST `"tactic":` value-quote and the LAST quote before `}`.
    //    Handles bare literal newlines in the value (invalid JSON but recoverable). Un-escape common escapes.
    if let Some(ti) = cleaned.find("\"tactic\"") {
        let after = &cleaned[ti + 8..];
        if let Some(q1) = after.find('"') {
            let val_region = &after[q1 + 1..];
            // the value ends at the last `"` that precedes the final `}` (or end of string)
            let end = val_region.rfind("\"}").or_else(|| val_region.rfind('"')).unwrap_or(val_region.len());
            let raw = &val_region[..end];
            let unescaped = raw.replace("\\n", "\n").replace("\\t", "\t").replace("\\\"", "\"").replace("\\\\", "\\");
            let t = unescaped.trim();
            if !t.is_empty() { return Some(t.to_string()); }
        }
    }
    None
}

/// On-disk checkpoint payload: the partial hit matrix + per-model tokens, written after each model and
/// reloaded by `--resume`. Typed so resume uses a structured parser (constitution §12), not Value walking.
/// k_samples/max_rounds/seed are carried for a resume-time sanity check against the current args.
#[derive(serde::Deserialize)]
struct PartialState {
    #[serde(default)] hit: BTreeMap<String, BTreeMap<String, usize>>,
    #[serde(default)] tokens_by_model: BTreeMap<String, u64>,
    #[serde(default)] k_samples: usize,
    #[serde(default)] max_rounds: usize,
    #[serde(default)] seed: u64,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = parse_args()?;
    let thms = load_bank(&args.bank, args.n_theorems);
    if thms.is_empty() { return Err("empty bank".into()); }
    let llm = ResilientLLMClient::new(&args.proxy, 180, 3);
    let lean_bin = default_lean_bin();
    let lp = lean_path(&args.mathlib_dir).ok_or("Mathlib LEAN_PATH unresolved")?;
    let mut rng = StdRng::seed_from_u64(args.seed);
    let t0 = Instant::now();

    // hit[model][thm_id] = number of the k draws that produced an axiom-clean proof.
    let mut hit: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut tokens_by_model: BTreeMap<String, u64> = BTreeMap::new();
    let mut llm_calls = 0usize; let mut lean_calls = 0usize;

    // --resume: preload completed (model,theorem) cells from the sidecar so a re-launched run skips them.
    let partial = partial_path(&args.out);
    if args.resume {
        match std::fs::read_to_string(&partial) {
            Ok(text) => match serde_json::from_str::<PartialState>(&text) {
                Ok(p) => {
                    if p.k_samples != 0 && (p.k_samples != args.k_samples || p.max_rounds != args.max_rounds || p.seed != args.seed) {
                        eprintln!("[resume] WARN checkpoint params (k={} rounds={} seed={}) differ from current (k={} rounds={} seed={}); loaded hit counts reflect the OLD params",
                            p.k_samples, p.max_rounds, p.seed, args.k_samples, args.max_rounds, args.seed);
                    }
                    let cells: usize = p.hit.values().map(|m| m.len()).sum();
                    hit = p.hit;
                    tokens_by_model = p.tokens_by_model;
                    eprintln!("[resume] loaded {cells} prior (model,theorem) cell(s) across {} model(s) from {}", hit.len(), partial.display());
                }
                Err(e) => eprintln!("[resume] partial file {} unreadable ({e}); starting fresh", partial.display()),
            },
            Err(_) => eprintln!("[resume] no partial checkpoint at {}; starting fresh", partial.display()),
        }
    }

    for model in &args.models {
        {
            let mh = hit.entry(model.clone()).or_default();
            for thm in &thms {
                // --resume skip: this (model,theorem) cell was already computed in a prior run.
                if args.resume {
                    if let Some(&prev) = mh.get(&thm.id) {
                        eprintln!("  [resume] skip {model} {} ({prev}/{} draws clean, from checkpoint)", thm.id, args.k_samples);
                        continue;
                    }
                }
                let mut hits = 0usize;
                for s in 0..args.k_samples {
                    // one DRAW = up to max_rounds of propose→Lean→feedback→retry (multi-round; single-shot under-measures p).
                    let mut msgs = vec![Message { role: "user".into(), content: format!(
                        "Prove this Lean 4 (Mathlib) theorem. Use Lean4 syntax (fun x => ... , induction n with | zero => | succ k ih =>). Output ONLY JSON {{\"tactic\":\"<proof body>\"}}. No sorry/admit/native_decide.\n\n{}", thm.preamble) }];
                    let mut solved = false;
                    for r in 0..args.max_rounds {
                        let temp = args.temp + 0.05 * s as f64; // spread temperature across draws for diversity
                        let resp = match llm.generate(&GenerateRequest { model: model.clone(), messages: msgs.clone(), temperature: Some(temp.min(1.2)), max_tokens: Some(8000) }).await {
                            Ok(x) => { llm_calls += 1; *tokens_by_model.entry(model.clone()).or_default() += (x.prompt_tokens + x.completion_tokens) as u64; x }
                            Err(_) => break,
                        };
                        let tac = match extract_tactic(&resp.content) { Some(t) if !t.trim().is_empty() => t, _ => break };
                        lean_calls += 1;
                        let (ok, fb) = verify_axiom_clean(&thm.preamble, &tac, &lean_bin, &args.mathlib_dir, &lp, &format!("{}_{}_{}_{}", args.seed, model.replace('/', "_"), thm.id, s));
                        if ok { solved = true; break; }
                        if r + 1 < args.max_rounds {
                            msgs.push(Message { role: "assistant".into(), content: resp.content });
                            msgs.push(Message { role: "user".into(), content: format!("Lean REJECTED that: {fb}\nFix it. Output ONLY JSON {{\"tactic\":\"<proof body>\"}}.") });
                        }
                    }
                    if solved { hits += 1; }
                }
                mh.insert(thm.id.clone(), hits);
                let _ = rng.gen::<u8>(); // keep rng live per (model,thm) for reproducible temp spread
                eprintln!("  {model} {} : {hits}/{} draws clean", thm.id, args.k_samples);
            }
        } // drop mh's mutable borrow of `hit` before serializing the whole map below

        // CHECKPOINT (after this model's theorem loop): persist the partial hit matrix + tokens so a crash,
        // sleep, or kill during the multi-hour run keeps every COMPLETED model's per-theorem hit counts.
        // The final full write at the end of main is left unchanged; this is purely additive durability.
        let snapshot = serde_json::json!({
            "schema": "lean_emergence_stage1.partial.v1",
            "seed": args.seed, "k_samples": args.k_samples, "max_rounds": args.max_rounds,
            "models": args.models, "completed_through_model": model,
            "hit": hit, "tokens_by_model": tokens_by_model,
        });
        match serde_json::to_string_pretty(&snapshot) {
            Ok(s) => match std::fs::write(&partial, s) {
                Ok(()) => eprintln!("[checkpoint] wrote partial after model {model} → {}", partial.display()),
                Err(e) => eprintln!("[checkpoint] WARN could not write {}: {e}", partial.display()),
            },
            Err(e) => eprintln!("[checkpoint] WARN could not serialize partial: {e}"),
        }
    }

    // ── Stage-1 analysis: per-model pass@k (coverage A) + joint-hit partition (combination targets B) ──
    let k = args.k_samples as f64;
    let mut per_model = serde_json::Map::new();
    for (model, mh) in &hit {
        let solved_any: Vec<&String> = mh.iter().filter(|(_, &h)| h > 0).map(|(id, _)| id).collect();
        // empirical per-theorem p_hat = hits/k; pass@k for a fresh k' uses 1-(1-p)^k'.
        let phat: BTreeMap<&String, f64> = mh.iter().map(|(id, &h)| (id, h as f64 / k)).collect();
        let mean_p: f64 = phat.values().sum::<f64>() / phat.len().max(1) as f64;
        per_model.insert(model.clone(), serde_json::json!({
            "solved_count_at_k": solved_any.len(),
            "solved_ids": solved_any,
            "mean_p_hat": mean_p,
            "p_hat": phat.iter().map(|(id, p)| ((*id).clone(), p)).collect::<BTreeMap<_,_>>(),
            "tokens": tokens_by_model.get(model).copied().unwrap_or(0),
        }));
    }
    // union solved + combination-target = in union but in NO single model's solo set... but a theorem solved
    // by model X alone IS in X's solo set, so "combination-target" at Stage-1 = theorems with p=0 for EVERY
    // model individually (these are the ones only a COMPOSITIONAL pipeline could reach — flagged for Stage 2
    // where the proposer/filler split may lift p>0). Also report the union (reachable-by-some-model).
    let all_ids: Vec<String> = thms.iter().map(|t| t.id.clone()).collect();
    let union_solved: Vec<&String> = all_ids.iter().filter(|id| hit.values().any(|mh| mh.get(*id).copied().unwrap_or(0) > 0)).collect();
    let none_solved: Vec<&String> = all_ids.iter().filter(|id| hit.values().all(|mh| mh.get(*id).copied().unwrap_or(0) == 0)).collect();
    // complementary pairs: theorems solved by model A but NOT B (and vice-versa) → heterogeneity signal.
    let models: Vec<&String> = hit.keys().collect();
    let mut complementary = serde_json::Map::new();
    for i in 0..models.len() { for j in 0..models.len() { if i >= j { continue; }
        let a = models[i]; let b = models[j];
        let a_only: Vec<&String> = all_ids.iter().filter(|id| hit[a].get(*id).copied().unwrap_or(0) > 0 && hit[b].get(*id).copied().unwrap_or(0) == 0).collect();
        let b_only: Vec<&String> = all_ids.iter().filter(|id| hit[b].get(*id).copied().unwrap_or(0) > 0 && hit[a].get(*id).copied().unwrap_or(0) == 0).collect();
        complementary.insert(format!("{a}__vs__{b}"), serde_json::json!({"a_only": a_only.len(), "b_only": b_only.len(), "a_only_ids": a_only, "b_only_ids": b_only}));
    }}

    let manifest = serde_json::json!({
        "schema": "lean_emergence_stage1.v1", "seed": args.seed, "k_samples": args.k_samples,
        "max_rounds": args.max_rounds, "temp": args.temp, "n_theorems": thms.len(), "models": args.models,
        "per_model": per_model,
        "union_solved_count": union_solved.len(), "union_solved_ids": union_solved,
        "none_solved_count": none_solved.len(), "none_solved_ids": none_solved,
        "complementary_pairs": complementary,
        "llm_calls": llm_calls, "lean_calls": lean_calls, "wall_s": t0.elapsed().as_secs_f64(),
        "axiom_whitelist": AXIOM_WHITELIST,
    });
    std::fs::write(&args.out, serde_json::to_string_pretty(&manifest).unwrap()).map_err(|e| format!("write: {e}"))?;
    println!("emergence-stage1 models={} theorems={} k={} → union_solved={}/{} none_solved={} wall={:.1}s out={}",
        args.models.len(), thms.len(), args.k_samples, union_solved.len(), thms.len(), none_solved.len(), t0.elapsed().as_secs_f64(), args.out.display());
    Ok(())
}
