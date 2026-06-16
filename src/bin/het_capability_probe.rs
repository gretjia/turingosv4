//! HET-CAP-1 (HARDENED): Heterogeneity-capability probe — cross-lab model overnight run.
//!
//! Scientific question: can a CROSS-LAB model solve a Lean theorem that the
//! homogeneous deepseek-v4-pro swarm never could (support-expansion / emergence
//! definition #1)?
//!
//! Hardening vs smoke edition (2026-06-14):
//!   - HTTP timeout 120 s; retry up to 6× with exponential backoff + jitter.
//!   - `build_judge` + `judge.verify` wrapped in `catch_unwind` → JudgePanic
//!     verdict on panic; run continues.
//!   - Resume-skip on startup: reads existing records.jsonl; skips any
//!     (theorem, model, attempt) already recorded with a REAL verdict
//!     (Verified/Failed/ParseError/JudgePanic); RE-RUNS ApiError cells.
//!   - `extract_proof_body` handles GLM/Qwen fence formats, bare proof blocks,
//!     and non-JSON thinking-model wrappers.
//!   - Budget cap: aborts cleanly after MAX_CALLS model calls.
//!
//! For each (theorem, model, attempt) triple this bin:
//!   1. Builds the SAME proof prompt `lean_market_agent` uses (fresh attempt).
//!   2. Calls the model via the SiliconFlow OpenAI-compatible endpoint.
//!   3. Extracts the Lean proof body from the JSON response.
//!   4. Runs the REAL LeanJudge (axiom-clean Verified only — no sorry, no native_decide).
//!   5. Appends a JSONL line to `handover/evidence/het_probe_run/records.jsonl`.
//!
//! Outputs:
//!   handover/evidence/het_probe_run/records.jsonl  — append; one line per attempt
//!
//! Configuration: SILICONFLOW_API_KEY + SILICONFLOW_ENDPOINT from env.
//! Key is NEVER printed, logged, or written to any output file.
//!
//! Class 1 (additive new bin; reuses LeanJudge + lean_theorem_bank; no §6 surface).

use std::collections::HashSet;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::time::Duration;

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::time::sleep;

use turingosv4::judges::lean_judge::{default_lean_bin, realign, LeanJudge};
use turingosv4::judges::lean_theorem_bank::{
    default_lake_bin, load_bank, mathlib_lean_path, LeanTheorem,
};

// ── Config ────────────────────────────────────────────────────────────────────

/// The 6 never-solved theorems (overnight run targets).
const THEOREM_IDS: &[&str] = &["lm_det_zero", "lm_c", "lm_coeff_mul", "lm_e", "lm_lim1", "lm_nt_cop_cubic"];

/// Cross-lab models under test (SiliconFlow ids, non-thinking / non-reasoning mode).
const MODELS: &[&str] = &["deepseek-ai/DeepSeek-V4-Pro", "Qwen/Qwen3-32B", "zai-org/GLM-4.5-Air", "Qwen/Qwen3.5-397B-A17B"];

/// Attempts per (theorem, model) cell.
const K: usize = 3;

/// Hard budget cap: abort after this many model API calls.
const MAX_CALLS: usize = 140;

/// Temperature matching lean_market_agent's PROOF_TEMPERATURE.
const PROOF_TEMPERATURE: f64 = 0.7;

/// Max tokens for proof response.
const MAX_TOKENS: u32 = 2048;
/// 门2 (OBL-018, architect 2026-06-14): a uniform NON-THINKING regime across ALL models —
/// send `enable_thinking:false` (SiliconFlow flat param; the old GLM nested form is
/// deprecated — handoff §6, verified_on 2026-06-14) so deepseek vs Qwen/GLM default
/// reasoning differences cannot confound the measurement. Regime EFFECTIVENESS is
/// validated at 门5 by the completion-token fingerprint (non-thinking ct should sit well
/// below MAX_TOKENS, not pin at it), not at compile time.
const ENABLE_THINKING: bool = false;

/// HTTP timeout per call (seconds). Must be > typical 10 s slow call.
const HTTP_TIMEOUT_SECS: u64 = 120;

/// Retry delays (seconds) for transient network errors. 6 retries = delays 2,4,8,16,32,64.
/// Jitter ±30% is added to each to avoid thundering-herd on contended machine.
const RETRY_BASE_SECS: u64 = 2;
const MAX_RETRIES: usize = 6;

// ── Output types ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
struct ProbeRecord {
    /// Source bin that wrote this record (for mixed-record files).
    probe: String,
    theorem: String,
    model: String,
    attempt: usize,
    prompt_tokens: u32,
    completion_tokens: u32,
    /// "Verified" | "Failed" | "SorryBlocked" | "ApiError" | "ParseError" | "JudgePanic"
    verdict: String,
    is_verified: bool,
    /// True only when LeanJudge reports Verified AND axiom gate passed.
    axiom_clean: bool,
    /// Transitive axiom set from #print axioms (empty unless compiled exit-0).
    axioms: Vec<String>,
    /// Short human-readable note (error class on failure, empty on verified).
    note: String,
}

/// Key that uniquely identifies a cell for resume-skip.
#[derive(Debug, PartialEq, Eq, Hash)]
struct CellKey {
    theorem: String,
    model: String,
    attempt: usize,
}

// ── Resume: load already-completed cells ──────────────────────────────────────

/// Returns the set of (theorem, model, attempt) that already have a REAL verdict
/// (Verified, Failed, ParseError, JudgePanic — NOT ApiError) in the records file.
/// ApiError cells are re-queued because those were network failures, not real attempts.
fn load_completed_cells(records_path: &PathBuf) -> HashSet<CellKey> {
    let mut done = HashSet::new();
    let text = match std::fs::read_to_string(records_path) {
        Ok(t) => t,
        Err(_) => return done, // file doesn't exist yet
    };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(rec) = serde_json::from_str::<ProbeRecord>(line) {
            // Only skip if it has a real verdict (not ApiError).
            let real_verdict = rec.verdict != "ApiError";
            if real_verdict {
                done.insert(CellKey {
                    theorem: rec.theorem,
                    model: rec.model,
                    attempt: rec.attempt,
                });
            }
        }
    }
    done
}

// ── Proof prompt (mirrors lean_market_agent::build_prompt, fresh attempt) ────

fn build_proof_prompt(theorem: &LeanTheorem) -> String {
    let mut p = String::new();
    p.push_str(
        "You are proving a theorem in Lean 4 (Mathlib is available). Output ONLY a JSON object.\n\n",
    );
    p.push_str("=== Target (prove the goal after `:= by`) ===\n");
    p.push_str(&theorem.preamble);
    p.push('\n');
    p.push_str(
        "\nReturn EXACTLY: {\"proof_body\":\"<the Lean tactic block AFTER `:= by`, no theorem signature, no imports>\",\"confidence\":0.0-1.0}\n",
    );
    p
}

// ── LLM HTTP (direct to SiliconFlow; key loaded from env at runtime) ─────────

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f64,
    max_tokens: u32,
    /// 门2: uniform non-thinking regime — always `false` for this probe (see ENABLE_THINKING).
    enable_thinking: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug)]
struct LlmResponse {
    content: String,
    prompt_tokens: u32,
    completion_tokens: u32,
    /// OpenAI-style `choices[0].finish_reason` ("stop" | "length" | …); "" if absent.
    /// `"length"` is the authoritative truncation signal (§11.2 regime artifact).
    finish_reason: String,
}

#[derive(Debug)]
enum LlmError {
    Auth,
    RateLimit,
    Network(String),
    Parse(String),
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmError::Auth => write!(f, "auth error (401/403)"),
            LlmError::RateLimit => write!(f, "rate limited (429)"),
            LlmError::Network(s) => write!(f, "network: {s}"),
            LlmError::Parse(s) => write!(f, "parse: {s}"),
        }
    }
}

/// Call the SiliconFlow endpoint once. Key is read from env and dropped immediately.
/// Returns Err on unrecoverable failures.
async fn call_model(
    client: &reqwest::Client,
    endpoint: &str,
    model: &str,
    prompt: &str,
) -> Result<LlmResponse, LlmError> {
    // Load key fresh each call — never stored in a struct field.
    let key = std::env::var("SILICONFLOW_API_KEY").map_err(|_| LlmError::Auth)?;

    let req = ChatRequest {
        model: model.to_string(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
        temperature: PROOF_TEMPERATURE,
        max_tokens: MAX_TOKENS,
        enable_thinking: ENABLE_THINKING,
    };

    let resp = client
        .post(endpoint)
        .bearer_auth(&key)
        .json(&req)
        .send()
        .await
        .map_err(|e| LlmError::Network(e.to_string()))?;

    // Drop the key immediately — we don't need it anymore.
    drop(key);

    let status = resp.status();
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        return Err(LlmError::Auth);
    }
    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        return Err(LlmError::RateLimit);
    }
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        // Truncate — never print full body (may contain key echoes).
        let safe_body: String = body.chars().take(200).collect();
        return Err(LlmError::Network(format!("HTTP {}: {}", status, safe_body)));
    }

    let body: Value = resp
        .json()
        .await
        .map_err(|e| LlmError::Parse(e.to_string()))?;

    let content = body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();
    let prompt_tokens = body["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32;
    let completion_tokens = body["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32;
    let finish_reason = body["choices"][0]["finish_reason"]
        .as_str()
        .unwrap_or("")
        .to_string();

    Ok(LlmResponse {
        content,
        prompt_tokens,
        completion_tokens,
        finish_reason,
    })
}

/// Call with exponential-backoff retry on transient errors (network / rate-limit).
/// Auth errors are never retried. After MAX_RETRIES exhausted, returns last error.
/// Delays: 2, 4, 8, 16, 32, 64 s (base 2^i) ±30% jitter to reduce thundering herd.
async fn call_model_with_retry(
    client: &reqwest::Client,
    endpoint: &str,
    model: &str,
    prompt: &str,
) -> Result<LlmResponse, LlmError> {
    let mut rng = rand::thread_rng();
    let mut last_err = LlmError::Network("no attempts".to_string());

    for attempt in 0..=MAX_RETRIES {
        if attempt > 0 {
            // Exponential backoff: base * 2^(attempt-1), capped at 64 s.
            let base = RETRY_BASE_SECS * (1u64 << (attempt - 1).min(5));
            // ±30% jitter
            let jitter_factor = rng.gen_range(0.70_f64..1.30_f64);
            let delay_secs = ((base as f64) * jitter_factor) as u64;
            eprintln!(
                "  [retry {attempt}/{MAX_RETRIES}] model={model} sleeping {delay_secs}s after: {last_err}"
            );
            sleep(Duration::from_secs(delay_secs)).await;
        }

        match call_model(client, endpoint, model, prompt).await {
            Ok(r) => return Ok(r),
            Err(LlmError::Auth) => return Err(LlmError::Auth), // auth never retries
            Err(e) => {
                last_err = e;
            }
        }
    }

    Err(last_err)
}

/// Classify whether a response was cut off by the token budget — a REGIME artifact
/// (§11.2), not a genuine capability failure — so it can be recorded as a distinct
/// `Truncated` verdict instead of being silently bucketed into ParseError/Failed.
/// Trusts the API `finish_reason` when present; falls back to "completion filled the
/// whole budget" only when the provider omits finish_reason.
fn is_truncated(finish_reason: &str, completion_tokens: u32, max_tokens: u32) -> bool {
    finish_reason == "length"
        || (finish_reason.is_empty() && completion_tokens >= max_tokens)
}

// ── Proof body extraction ─────────────────────────────────────────────────────
//
// Handles multiple model output formats:
//   1. Clean JSON: {"proof_body": "..."} possibly wrapped in ```json ... ``` fences.
//   2. JSON embedded in prose / thinking text.
//   3. GLM / thinking models that output <think>...</think> then a proof.
//   4. Markdown ```lean ... ``` fence (bare proof block, no JSON).
//   5. `theorem Foo ... := by\n  <tactics>` (inline theorem + proof).
//   6. Bare tactic block (indented or `by ...` prefix).

fn extract_proof_body(content: &str) -> Option<String> {
    // ── Strategy 1: JSON object with proof_body field ──────────────────────────

    // Strip any outer ```json or ``` code fences.
    let stripped = strip_outer_fence(content);

    // Try full parse first.
    if let Ok(v) = serde_json::from_str::<Value>(stripped.trim()) {
        if let Some(body) = v.get("proof_body").and_then(|b| b.as_str()) {
            // realign: flush a flat tactic sequence to col 0; defer to conservative
            // dedent when the body has genuine nesting. Cures IP1 de-alignment.
            let b = realign(body);
            if !b.is_empty() {
                return Some(b);
            }
        }
    }

    // Try extracting a JSON object from within the text (handles prose before/after).
    if let Some(body) = extract_json_proof_body(content) {
        return Some(body);
    }

    // ── Strategy 2: ```lean ... ``` fence ────────────────────────────────────

    if let Some(body) = extract_lean_fence(content) {
        return Some(body);
    }

    // ── Strategy 3: `theorem ... := by` inline pattern ───────────────────────

    if let Some(body) = extract_after_by(content) {
        return Some(body);
    }

    // ── Strategy 4: bare tactic block (after stripping think tags) ───────────

    if let Some(body) = extract_bare_tactics(content) {
        return Some(body);
    }

    None
}

/// Strip outermost ```json or ``` ... ``` fences, returning the inner text.
fn strip_outer_fence(s: &str) -> &str {
    let t = s.trim();
    // Handle ```json\n...\n``` or ```\n...\n```
    for prefix in &["```json", "```lean", "```"] {
        if t.starts_with(prefix) {
            let after = &t[prefix.len()..];
            // Skip optional newline.
            let after = after.trim_start_matches('\n').trim_start_matches('\r');
            if let Some(end) = after.rfind("```") {
                return &after[..end];
            }
        }
    }
    t
}

/// Scan for the first {...} block that contains a "proof_body" key.
fn extract_json_proof_body(content: &str) -> Option<String> {
    // Find all '{' positions and try to parse from each.
    let bytes = content.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b != b'{' {
            continue;
        }
        // Walk forward looking for matching '}'.
        let mut depth = 0i32;
        let mut end = None;
        for (j, &c) in bytes[i..].iter().enumerate() {
            match c {
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(i + j);
                        break;
                    }
                }
                _ => {}
            }
        }
        if let Some(end_idx) = end {
            let candidate = &content[i..=end_idx];
            if let Ok(v) = serde_json::from_str::<Value>(candidate) {
                if let Some(body) = v.get("proof_body").and_then(|b| b.as_str()) {
                    let b = realign(body);
                    if !b.is_empty() {
                        return Some(b);
                    }
                }
            }
        }
    }
    None
}

/// Extract the content inside the first ```lean ... ``` block.
fn extract_lean_fence(content: &str) -> Option<String> {
    for fence_start in &["```lean\n", "```lean\r\n", "```Lean\n"] {
        if let Some(start) = content.find(fence_start) {
            let after = &content[start + fence_start.len()..];
            if let Some(end) = after.find("```") {
                // Keep per-line indentation intact (operate on the RAW fence body); a flat
                // `.trim()` here would de-align a uniformly-indented block. `dedent` does
                // the column normalization at return.
                let raw = &after[..end];
                if !raw.trim().is_empty() {
                    // BUGFIX (audit wpgyhkjxc): a fenced block may contain the full
                    // `theorem ... := by` signature. assemble() prepends the preamble
                    // (already ending in `:= by`), so returning the signature here
                    // DOUBLES it and turns any valid proof into a compile failure.
                    // Return ONLY the post-`:= by` tactics when the fence is a full decl.
                    let ts = raw.trim_start();
                    if ts.starts_with("theorem") || ts.starts_with("lemma") || ts.starts_with("example") {
                        if let Some(pos) = raw.find(":= by") {
                            let tactics = realign(&raw[pos + 5..]);
                            if !tactics.is_empty() {
                                return Some(tactics);
                            }
                        }
                    }
                    let body = realign(raw);
                    if !body.is_empty() {
                        return Some(body);
                    }
                }
            }
        }
    }
    None
}

/// Extract everything after `:= by` in a theorem declaration (handles GLM inline proofs).
fn extract_after_by(content: &str) -> Option<String> {
    // Strip <think>...</think> blocks that GLM / thinking models emit.
    let clean = strip_think_tags(content);
    // Look for `:= by` pattern.
    if let Some(pos) = clean.find(":= by") {
        // Keep per-line indentation intact; strip only a trailing code fence, then
        // dedent (NOT a flat trim, which de-aligns a uniformly-indented block).
        let raw = clean[pos + 5..].trim_end();
        let raw = raw.strip_suffix("```").unwrap_or(raw);
        let body = realign(raw);
        if !body.is_empty() {
            return Some(body);
        }
    }
    None
}

/// Heuristic: if the content (after stripping think tags) looks like Lean tactics,
/// return it directly. Tactics often start with `exact`, `simp`, `ring`, `omega`,
/// `apply`, `intro`, `have`, `rw`, `norm_num`, `decide`, `linarith`, `nlinarith`.
fn extract_bare_tactics(content: &str) -> Option<String> {
    let clean = strip_think_tags(content);
    let first_line = clean.lines().find(|l| !l.trim().is_empty())?;
    let tl = first_line.trim().to_lowercase();
    let tactic_prefixes = [
        "exact ", "simp", "ring", "omega", "apply ", "intro ", "have ", "rw ", "norm_num",
        "decide", "linarith", "nlinarith", "field_simp", "aesop", "tauto", "trivial",
        "constructor", "use ", "refine ", "push_neg", "contrapose",
    ];
    for prefix in &tactic_prefixes {
        if tl.starts_with(prefix) {
            let body = realign(&clean);
            if !body.is_empty() {
                return Some(body);
            }
        }
    }
    None
}

/// Reasoning-wrapper tags emitted by thinking models, stripped (with their content) so
/// chain-of-thought never leaks into proof extraction. Covers the `<think>` family AND
/// the `<thinking>` / `<thought>` / `<reasoning>` variants other model templates use
/// (§11.3). A well-formed `<tag>…</tag>` pair is removed; an UNCLOSED opener — the
/// truncated / max_tokens case (§11.2) where `</tag>` never arrives — is dropped from the
/// opener to end-of-string, since truncated reasoning has no terminated proof after it to
/// preserve. Case-sensitive (models emit these tags lowercased).
const THINK_TAGS: &[&str] = &["think", "thinking", "thought", "reasoning"];

fn strip_think_tags(s: &str) -> String {
    let mut result = s.to_string();
    loop {
        // Earliest opener of any known reasoning tag (byte offsets into `result`).
        let mut best: Option<(usize, &str)> = None;
        for tag in THINK_TAGS {
            if let Some(i) = result.find(&format!("<{tag}>")) {
                if best.map_or(true, |(bi, _)| i < bi) {
                    best = Some((i, tag));
                }
            }
        }
        let Some((start, tag)) = best else { break };
        let open = format!("<{tag}>");
        let close = format!("</{tag}>");
        match result[start + open.len()..].find(&close) {
            Some(rel) => {
                let end = start + open.len() + rel + close.len();
                result.replace_range(start..end, "");
            }
            None => {
                // Unclosed opener (truncated reasoning): drop opener → EOF.
                result.truncate(start);
                break;
            }
        }
    }
    result
}

// ── LeanJudge builder (mirrors het_calibration_probe::build_judge) ────────────

fn build_judge(theorem: &LeanTheorem, lean_bin: PathBuf, mathlib_lp: Option<&str>) -> LeanJudge {
    let mut j = LeanJudge::new(theorem.preamble.clone());
    j.lean_bin = lean_bin;
    // Use the mathlib dir as cwd so lake-relative olean paths resolve.
    j.cwd = PathBuf::from("/Users/zephryj/work/mathlib4");
    j.timeout = Duration::from_secs(120);
    if theorem.needs_mathlib {
        if let Some(lp) = mathlib_lp {
            j.extra_env.push(("LEAN_PATH".to_string(), lp.to_string()));
        }
    }
    j
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    // ── 1. Load env (key validated below; endpoint required) ──────────────────
    let endpoint = std::env::var("SILICONFLOW_ENDPOINT")
        .unwrap_or_else(|_| "https://api.siliconflow.cn/v1/chat/completions".to_string());

    // Validate key exists (don't print it).
    if std::env::var("SILICONFLOW_API_KEY").is_err() {
        eprintln!("ERROR: SILICONFLOW_API_KEY not set in environment.");
        eprintln!("  Source the .env file first: source .env");
        std::process::exit(1);
    }
    eprintln!("[het_cap] endpoint={endpoint}");
    eprintln!("[het_cap] key loaded (not printed)");

    // Env overrides (default to consts) — clean, prereg-recordable pilot/full runs.
    let theorem_ids: Vec<String> = std::env::var("HET_THEOREMS")
        .ok()
        .map(|s| {
            s.split(',')
                .map(|x| x.trim().to_string())
                .filter(|x| !x.is_empty())
                .collect()
        })
        .unwrap_or_else(|| THEOREM_IDS.iter().map(|s| s.to_string()).collect());
    let k: usize = std::env::var("HET_K").ok().and_then(|s| s.parse().ok()).unwrap_or(K);
    let max_calls: usize = std::env::var("HET_MAX_CALLS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(MAX_CALLS);
    eprintln!(
        "[het_cap] config: {} theorems × {} models × K={k} = {} cells (cap={max_calls})",
        theorem_ids.len(),
        MODELS.len(),
        theorem_ids.len() * MODELS.len() * k
    );

    // ── 2. Resolve lean binary + mathlib ──────────────────────────────────────
    let lean_bin = default_lean_bin();
    if !(lean_bin.is_absolute() && lean_bin.exists()) {
        eprintln!(
            "BLOCKER: Lean pinned toolchain not found at {}",
            lean_bin.display()
        );
        std::process::exit(1);
    }
    let lean_version = std::process::Command::new(&lean_bin)
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "unknown".to_string());
    eprintln!(
        "[het_cap] lean_bin={} version={}",
        lean_bin.display(),
        lean_version
    );

    // Resolve Mathlib LEAN_PATH via the pointer file (or a known path).
    let mathlib_dir_path = {
        let pointer = PathBuf::from("handover/lean_env/mathlib_dir.txt");
        let from_pointer = std::fs::read_to_string(&pointer)
            .ok()
            .map(|s| PathBuf::from(s.trim()))
            .filter(|p| p.exists());
        from_pointer.or_else(|| {
            let p = PathBuf::from("/Users/zephryj/work/mathlib4");
            if p.exists() { Some(p) } else { None }
        })
    };
    let mathlib_lp = mathlib_dir_path
        .as_ref()
        .and_then(|d| mathlib_lean_path(d, &default_lake_bin()));
    match &mathlib_lp {
        Some(lp) => {
            let short: String = lp.chars().take(80).collect();
            eprintln!("[het_cap] mathlib LEAN_PATH set ({}...)", short);
        }
        None => {
            eprintln!("[het_cap] BLOCKER: no Mathlib LEAN_PATH — theorems need Mathlib");
            std::process::exit(1);
        }
    }

    // ── 3. Load theorem bank ──────────────────────────────────────────────────
    let bank_path = PathBuf::from("tests/fixtures/lean_theorems_pool.jsonl");
    let pool_bank = match load_bank(&bank_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!(
                "BLOCKER: could not load theorem bank {}: {e}",
                bank_path.display()
            );
            std::process::exit(1);
        }
    };

    // Filter to requested theorem IDs (warn but continue on missing).
    let theorems: Vec<LeanTheorem> = theorem_ids
        .iter()
        .filter_map(|id| {
            let found = pool_bank.iter().find(|t| &t.id == id).cloned();
            if found.is_none() {
                eprintln!("WARN: theorem id={id} not found in bank — skipping");
            }
            found
        })
        .collect();

    if theorems.is_empty() {
        eprintln!("BLOCKER: no theorems found in bank.");
        std::process::exit(1);
    }
    eprintln!(
        "[het_cap] theorems: {:?}",
        theorems.iter().map(|t| &t.id).collect::<Vec<_>>()
    );
    eprintln!("[het_cap] models: {:?}", MODELS);

    // ── 4. Output paths ───────────────────────────────────────────────────────
    let out_dir = PathBuf::from(
        std::env::var("HET_OUT_DIR").unwrap_or_else(|_| "handover/evidence/het_probe_run".to_string()),
    );
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!(
            "ERROR: could not create output dir {}: {e}",
            out_dir.display()
        );
        std::process::exit(1);
    }
    let records_path = out_dir.join("records.jsonl");

    // ── 5. Resume: load already-completed cells ───────────────────────────────
    let completed = load_completed_cells(&records_path);
    let skipped = completed.len();
    eprintln!("[het_cap] resume: {skipped} cells already completed (skipping), ApiError cells will re-run");

    // Open records file for append.
    use std::io::Write;
    let mut records_file = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&records_path)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("ERROR: could not open records file: {e}");
            std::process::exit(1);
        }
    };

    // ── 6. HTTP client ────────────────────────────────────────────────────────
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(HTTP_TIMEOUT_SECS))
        .build()
        .expect("build reqwest client");

    // ── 7. Main loop ──────────────────────────────────────────────────────────
    let mut call_count = 0usize;
    let mut new_records = 0usize;

    'outer: for theorem in &theorems {
        for model in MODELS {
            for attempt in 0..k {
                // Budget cap.
                if call_count >= max_calls {
                    eprintln!(
                        "[het_cap] BUDGET CAP: {call_count}/{max_calls} calls reached — aborting cleanly"
                    );
                    break 'outer;
                }

                // Resume-skip: skip if this cell already has a real verdict.
                let key = CellKey {
                    theorem: theorem.id.clone(),
                    model: model.to_string(),
                    attempt,
                };
                if completed.contains(&key) {
                    eprintln!(
                        "[het_cap] SKIP (already done) theorem={} model={} attempt={}",
                        theorem.id, model, attempt
                    );
                    continue;
                }

                eprintln!(
                    "[het_cap] [{}/{}] theorem={} model={} attempt={}",
                    call_count + 1,
                    max_calls,
                    theorem.id,
                    model,
                    attempt
                );

                let prompt = build_proof_prompt(theorem);

                // Call model — failures are recorded, not fatal.
                let llm_result =
                    call_model_with_retry(&client, &endpoint, model, &prompt).await;
                call_count += 1;

                let record = match llm_result {
                    Err(e) => {
                        eprintln!("  => API error (all retries exhausted): {e}");
                        ProbeRecord {
                            probe: "het_capability_probe".to_string(),
                            theorem: theorem.id.clone(),
                            model: model.to_string(),
                            attempt,
                            prompt_tokens: 0,
                            completion_tokens: 0,
                            verdict: "ApiError".to_string(),
                            is_verified: false,
                            axiom_clean: false,
                            axioms: vec![],
                            note: format!("api_error: {e}"),
                        }
                    }
                    Ok(resp) => {
                        let pt = resp.prompt_tokens;
                        let ct = resp.completion_tokens;
                        // §11.2: a budget-truncated response is a regime artifact, not a
                        // capability failure — classify it as a distinct Truncated verdict.
                        let truncated = is_truncated(&resp.finish_reason, ct, MAX_TOKENS);
                        eprintln!(
                            "  => response received (pt={pt} ct={ct} finish={} truncated={truncated})",
                            resp.finish_reason
                        );

                        // Extract proof body.
                        match extract_proof_body(&resp.content) {
                            None => {
                                eprintln!("  => parse error: no proof_body in response");
                                ProbeRecord {
                                    probe: "het_capability_probe".to_string(),
                                    theorem: theorem.id.clone(),
                                    model: model.to_string(),
                                    attempt,
                                    prompt_tokens: pt,
                                    completion_tokens: ct,
                                    verdict: if truncated { "Truncated" } else { "ParseError" }
                                        .to_string(),
                                    is_verified: false,
                                    axiom_clean: false,
                                    axioms: vec![],
                                    note: if truncated {
                                        format!(
                                            "truncated finish_reason={} ct={ct}: no_proof_body",
                                            resp.finish_reason
                                        )
                                    } else {
                                        "no_proof_body_in_response".to_string()
                                    },
                                }
                            }
                            Some(body) => {
                                eprintln!("  => proof body extracted ({} chars), running LeanJudge...", body.len());

                                // ── Crash-safe judge invocation ────────────────
                                // Wrap build_judge + verify in catch_unwind so a
                                // panic in LeanJudge never kills the overnight run.
                                let judge_result = {
                                    let theorem_c = theorem.clone();
                                    let lean_bin_c = lean_bin.clone();
                                    let mathlib_lp_c = mathlib_lp.clone();
                                    let body_c = body.clone();
                                    catch_unwind(AssertUnwindSafe(move || {
                                        let judge = build_judge(
                                            &theorem_c,
                                            lean_bin_c,
                                            mathlib_lp_c.as_deref(),
                                        );
                                        judge.verify(&body_c)
                                    }))
                                };

                                match judge_result {
                                    Err(panic_val) => {
                                        let msg = panic_val
                                            .downcast_ref::<String>()
                                            .map(|s| s.as_str())
                                            .or_else(|| panic_val.downcast_ref::<&str>().copied())
                                            .unwrap_or("unknown panic");
                                        eprintln!("  => PANIC in LeanJudge: {msg}");
                                        ProbeRecord {
                                            probe: "het_capability_probe".to_string(),
                                            theorem: theorem.id.clone(),
                                            model: model.to_string(),
                                            attempt,
                                            prompt_tokens: pt,
                                            completion_tokens: ct,
                                            verdict: "JudgePanic".to_string(),
                                            is_verified: false,
                                            axiom_clean: false,
                                            axioms: vec![],
                                            note: format!("judge_panic: {}", &msg.chars().take(120).collect::<String>()),
                                        }
                                    }
                                    Ok(outcome) => {
                                        let verdict_str = format!("{:?}", outcome.verdict_kind);
                                        let is_ver = outcome.is_verified();
                                        let axiom_clean = is_ver && !outcome.axiom_rejected;

                                        eprintln!(
                                            "  => verdict={verdict_str} is_verified={is_ver} axioms={:?}",
                                            outcome.axioms
                                        );

                                        ProbeRecord {
                                            probe: "het_capability_probe".to_string(),
                                            theorem: theorem.id.clone(),
                                            model: model.to_string(),
                                            attempt,
                                            prompt_tokens: pt,
                                            completion_tokens: ct,
                                            verdict: if !is_ver && truncated {
                                                "Truncated".to_string()
                                            } else {
                                                verdict_str
                                            },
                                            is_verified: is_ver,
                                            axiom_clean,
                                            axioms: outcome.axioms,
                                            note: if is_ver {
                                                String::new()
                                            } else if truncated {
                                                format!(
                                                    "truncated finish_reason={} ct={ct}: {}",
                                                    resp.finish_reason,
                                                    outcome.feedback.chars().take(80).collect::<String>()
                                                )
                                            } else {
                                                outcome.feedback.chars().take(120).collect()
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                };

                // Write JSONL line immediately (crash-safe: flushed per record).
                let line = serde_json::to_string(&record).expect("serialize record");
                writeln!(records_file, "{line}").expect("write record");
                records_file.flush().expect("flush record");
                new_records += 1;

                eprintln!(
                    "[het_cap] record written: theorem={} model={} attempt={} verdict={}",
                    record.theorem, record.model, record.attempt, record.verdict
                );

                // Brief inter-call pause — polite to API and shared machine.
                sleep(Duration::from_millis(1200)).await;
            }
        }
    }

    // ── 8. Done ───────────────────────────────────────────────────────────────
    eprintln!(
        "[het_cap] DONE. new_records={new_records} call_count={call_count} cap={max_calls}"
    );
    eprintln!("[het_cap] records: {}", records_path.display());

    // Security: assert no key in records file.
    let records_text = std::fs::read_to_string(&records_path).unwrap_or_default();
    let key_leak_zero = !records_text.contains("sk-");
    if !key_leak_zero {
        eprintln!("SECURITY WARNING: records file may contain a key-like string 'sk-'");
        std::process::exit(2);
    }
    assert!(key_leak_zero, "SECURITY: records file contains 'sk-'");

    println!("records_path={}", records_path.display());
    println!("new_records={new_records}");
    println!("call_count={call_count}");
    println!("key_leak_grep_zero={key_leak_zero}");
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Pure extraction-dedent tests (always run; no toolchain) ──────────────────
    //
    // Regression for the de-alignment bug: every extraction strategy must DEDENT
    // (strip the shared leading indent) rather than flat-`.trim()`, which would leave
    // line 1 at col 0 and later siblings deeper — outside the Lean `by` block.

    #[test]
    fn extract_after_by_dedents_indented_body() {
        // Raw `theorem … := by` block with a uniformly 2-space-indented body (the shape
        // `extract_after_by` slices). Output tactics must all sit at col 0.
        let content = "theorem foo (p q : Prop) (hp : p) (hq : q) : p ∧ q := by\n  constructor\n  exact hp\n  exact hq";
        let body = extract_proof_body(content).expect("body");
        assert_eq!(body, "constructor\nexact hp\nexact hq");
        // No line may start with whitespace — the de-alignment signature.
        assert!(body.lines().all(|l| l == l.trim_start()), "got: {body:?}");
    }

    #[test]
    fn extract_json_dedents_uniformly_indented_body() {
        // Mirrors the bank's lm_det_zero reference_body (both tactics 2-space indented).
        let content = r#"{"proof_body":"  simp [Matrix.det_fin_three]\n  ring","confidence":0.9}"#;
        let body = extract_proof_body(content).expect("body");
        assert_eq!(body, "simp [Matrix.det_fin_three]\nring");
    }

    #[test]
    fn extract_json_col0_body_unchanged() {
        // The common JSON path (bodies already col-0) must NOT regress.
        let content = r#"{"proof_body":"simp\nnorm_num","confidence":0.5}"#;
        assert_eq!(
            extract_proof_body(content).as_deref(),
            Some("simp\nnorm_num")
        );
    }

    #[test]
    fn extract_lean_fence_dedents_indented_body() {
        let content = "```lean\n  intro h\n  exact h\n```";
        let body = extract_proof_body(content).expect("body");
        assert_eq!(body, "intro h\nexact h");
    }

    #[test]
    fn extract_lean_fence_full_decl_dedents_tactics() {
        // Full-decl fence (audit wpgyhkjxc path): strip the signature AND dedent the
        // post-`:= by` tactics so they anchor at col 0.
        let content = "```lean\ntheorem t : True := by\n  constructor\n```";
        let body = extract_proof_body(content).expect("body");
        assert_eq!(body, "constructor");
    }

    // ── Real-run end-to-end (gated on the pinned toolchain being present) ────────

    #[test]
    fn extract_then_verify_indented_body_real_lean() {
        // Full het-probe path: raw model content (uniformly-indented multi-tactic body)
        // → extract_proof_body → real LeanJudge → Verified. Pre-dedent this body
        // de-aligned and was mislabeled Failed. Mathlib-free, runs on the pinned bin.
        let bin = default_lean_bin();
        if !(bin.is_absolute() && bin.exists()) {
            eprintln!("skip: pinned Lean toolchain not present");
            return;
        }
        let content = "theorem t (p q : Prop) (hp : p) (hq : q) : p ∧ q := by\n  constructor\n  exact hp\n  exact hq";
        let body = extract_proof_body(content).expect("body");
        let mut j = LeanJudge::new("theorem t (p q : Prop) (hp : p) (hq : q) : p ∧ q := by");
        j.lean_bin = bin;
        let o = j.verify(&body);
        assert!(o.is_verified(), "extracted body must verify, got {o:?}");
    }

    // ── realign: cure first-line-shallow de-alignment (OBL-018, third bug) ───────
    //
    // The conservative shared `dedent` CANNOT recover a body whose first line is
    // shallower than a sibling; `realign` flushes a FLAT tactic sequence to col 0 at
    // extraction time (defers genuinely-nested bodies to `dedent`).

    #[test]
    fn realign_flushes_first_line_shallow_json_sibling() {
        // IP1: a model JSON body, first tactic flush, sibling indented (`simp\n  ring`).
        // Conservative dedent (common prefix "") leaves it de-aligned → the indented
        // sibling falls OUTSIDE the `by` block → a correct proof mislabeled Failed.
        let content = r#"{"proof_body":"simp\n  ring","confidence":0.8}"#;
        assert_eq!(extract_proof_body(content).as_deref(), Some("simp\nring"));
    }

    #[test]
    fn realign_inline_by_first_tactic_sibling() {
        // IP2: GLM-style inline `:= by <tac>` with an indented continuation. The
        // `extract_after_by` slice yields " simp\n  ring" (1-space / 2-space) — common
        // prefix " " → dedent leaves col0/col1, still de-aligned. realign flushes it.
        let content = "theorem t : True := by simp\n  ring";
        assert_eq!(extract_proof_body(content).as_deref(), Some("simp\nring"));
    }

    #[test]
    fn realign_preserves_genuine_nesting() {
        // A body with a real nested block (`have … := by` opener) must NOT be flattened:
        // realign defers to conservative dedent, keeping the inner step relatively indented.
        let content =
            r#"{"proof_body":"  have h : True := by\n    trivial\n  exact h","confidence":0.7}"#;
        assert_eq!(
            extract_proof_body(content).as_deref(),
            Some("have h : True := by\n  trivial\nexact h")
        );
    }

    #[test]
    fn extract_then_verify_first_line_shallow_real_lean() {
        // DECISIVE end-to-end (mathlib-free, pinned bin): the IP1 de-aligned shape that
        // the conservative judge dedent cannot recover (tests/het_third_bug_dealign_*)
        // must now VERIFY once realign flushes the flat sequence at extraction.
        let bin = default_lean_bin();
        if !(bin.is_absolute() && bin.exists()) {
            eprintln!("skip: pinned Lean toolchain not present");
            return;
        }
        // first tactic flush, siblings indented — the third-bug shape, in a JSON body.
        let content =
            r#"{"proof_body":"constructor\n  exact hp\n  exact hq","confidence":0.9}"#;
        let body = extract_proof_body(content).expect("body");
        assert!(
            body.lines().all(|l| l == l.trim_start()),
            "realign must flush every sibling to col 0, got {body:?}"
        );
        let mut j = LeanJudge::new("theorem t (p q : Prop) (hp : p) (hq : q) : p ∧ q := by");
        j.lean_bin = bin;
        let o = j.verify(&body);
        assert!(
            o.is_verified(),
            "first-line-shallow body must verify after realign, got {o:?}"
        );
    }

    #[test]
    fn extraction_adversarial_formatting_variants_all_verify_real_lean() {
        // §11.1 adversarial 4th-bug hunt: ONE known-good proof, rendered in many realistic
        // model-output FORMATS, must always extract→verify. Formatting must never change
        // verifiability; any flip is an extraction/de-alignment bug (4th+). Mathlib-free,
        // pinned bin. Collects ALL failures so every surviving bug is reported at once.
        let bin = default_lean_bin();
        if !(bin.is_absolute() && bin.exists()) {
            eprintln!("skip: pinned Lean toolchain not present");
            return;
        }
        let preamble = "theorem t (p q : Prop) (hp : p) (hq : q) : p ∧ q := by";
        // Same proof in every arm (constructor; exact hp; exact hq, or a focus-dot form);
        // only presentation differs. `\\n` inside a JSON arm is a literal backslash-n that
        // serde unescapes; a real `\n` is a genuine newline (the non-JSON arms).
        let variants: &[(&str, &str)] = &[
            ("json_uniform_2sp", "{\"proof_body\":\"  constructor\\n  exact hp\\n  exact hq\"}"),
            ("json_flush_col0", "{\"proof_body\":\"constructor\\nexact hp\\nexact hq\"}"),
            ("json_first_line_shallow_IP1", "{\"proof_body\":\"constructor\\n  exact hp\\n  exact hq\"}"),
            ("json_4space_uniform", "{\"proof_body\":\"    constructor\\n    exact hp\\n    exact hq\"}"),
            ("json_tabs", "{\"proof_body\":\"\\tconstructor\\n\\texact hp\\n\\texact hq\"}"),
            ("json_mixed_tab_space", "{\"proof_body\":\"\\tconstructor\\n  exact hp\\n  exact hq\"}"),
            ("json_crlf", "{\"proof_body\":\"constructor\\r\\n  exact hp\\r\\n  exact hq\"}"),
            ("json_blank_lines", "{\"proof_body\":\"constructor\\n\\n  exact hp\\n\\n  exact hq\"}"),
            ("json_fenced", "```json\n{\"proof_body\":\"constructor\\n  exact hp\\n  exact hq\"}\n```"),
            ("json_prose_wrapped", "Sure! Here is the proof:\n{\"proof_body\":\"constructor\\n  exact hp\\n  exact hq\"}\nHope that helps."),
            ("think_then_json", "<think>use constructor then exact</think>\n{\"proof_body\":\"constructor\\n  exact hp\\n  exact hq\"}"),
            ("inline_by_IP2", "theorem t (p q : Prop) (hp : p) (hq : q) : p ∧ q := by constructor\n  exact hp\n  exact hq"),
            ("lean_fence_full_decl", "```lean\ntheorem t (p q : Prop) (hp : p) (hq : q) : p ∧ q := by\n  constructor\n  exact hp\n  exact hq\n```"),
            ("lean_fence_bare_tactics", "```lean\n  constructor\n  exact hp\n  exact hq\n```"),
            ("focus_dots_nested", "{\"proof_body\":\"  constructor\\n  · exact hp\\n  · exact hq\"}"),
        ];
        let mut failures: Vec<String> = Vec::new();
        for (label, raw) in variants {
            match extract_proof_body(raw) {
                None => failures.push(format!("{label}: extract_proof_body returned None")),
                Some(body) => {
                    let mut j = LeanJudge::new(preamble);
                    j.lean_bin = bin.clone();
                    let o = j.verify(&body);
                    if !o.is_verified() {
                        failures.push(format!(
                            "{label}: NOT verified | body={body:?} | feedback={:?}",
                            o.feedback.chars().take(90).collect::<String>()
                        ));
                    }
                }
            }
        }
        assert!(
            failures.is_empty(),
            "4th-bug hunt found {} formatting-induced failure(s):\n{}",
            failures.len(),
            failures.join("\n")
        );
        eprintln!(
            "4th-bug hunt: all {} formatting variants of a known-good proof extracted + verified.",
            variants.len()
        );
    }

    // ── 门1: truncation classification + think-tag hardening ─────────────────────

    #[test]
    fn is_truncated_trusts_finish_reason_then_budget() {
        assert!(is_truncated("length", 2048, 2048)); // explicit length
        assert!(is_truncated("length", 10, 2048)); // length wins over low ct
        assert!(!is_truncated("stop", 2048, 2048)); // provider said stop → trust it
        assert!(!is_truncated("stop", 10, 2048));
        assert!(is_truncated("", 2048, 2048)); // no finish_reason + budget-filled
        assert!(!is_truncated("", 10, 2048)); // no finish_reason + room left
    }

    #[test]
    fn strip_think_handles_tag_name_variants() {
        assert_eq!(strip_think_tags("<think>reason</think>simp"), "simp");
        assert_eq!(strip_think_tags("<thinking>reason</thinking>simp"), "simp");
        assert_eq!(strip_think_tags("<reasoning>r</reasoning>\nsimp"), "\nsimp");
        assert_eq!(strip_think_tags("before<thought>x</thought>after"), "beforeafter");
        // a tactic body that merely mentions "thinking" in prose is untouched
        assert_eq!(strip_think_tags("simp\nring"), "simp\nring");
    }

    #[test]
    fn strip_think_drops_unclosed_opener_to_eof() {
        // Truncated reasoning (max_tokens cut before </think>): everything from the opener
        // on is dropped — there is no terminated proof after it.
        assert_eq!(
            strip_think_tags("intro h\n<think>let me work this out and"),
            "intro h\n"
        );
        assert_eq!(strip_think_tags("<thinking>cut off mid thought"), "");
    }

    #[test]
    fn chat_request_pins_uniform_non_thinking_regime() {
        // 门2: every model's request must carry enable_thinking:false @ MAX_TOKENS, so the
        // deepseek/Qwen/GLM default-reasoning difference cannot confound the measurement.
        let req = ChatRequest {
            model: "deepseek-ai/DeepSeek-V4-Pro".into(),
            messages: vec![],
            temperature: PROOF_TEMPERATURE,
            max_tokens: MAX_TOKENS,
            enable_thinking: ENABLE_THINKING,
        };
        let j = serde_json::to_value(&req).expect("serialize ChatRequest");
        assert_eq!(j["enable_thinking"], serde_json::json!(false));
        assert_eq!(j["max_tokens"], serde_json::json!(2048));
    }
}
