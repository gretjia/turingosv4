//! TRACE_MATRIX FC1a-rtool + FC1a-judge_pi + FC3-replay:
//! TDMA-Bounded shared runner library (Atom 18 — K10 + K11 refactor).
//!
//! Consolidates the kernel-driving loop that was previously duplicated across:
//!   - src/bin/tdma_rc1_deepseek_nesbitt.rs        (Atom 12)
//!   - src/bin/tdma_rc1_deepseek_putnam_a1.rs      (Atom 13)
//!   - src/bin/tdma_rc1_deepseek_putnam_2025_b3.rs (Atom 14)
//!   - src/bin/turingos/cmd_tdma.rs                (Atoms 15-17)
//!
//! Each callsite now reduces to: build a `RunConfig`, build an `AnyJudge`,
//! provide an `LlmCall` closure, call `run_proof`. The runner handles the
//! kernel boot, prompt assembly, judge dispatch, evidence capture, and
//! ChainTape serialization once.
//!
//! K10 fix: `AnyJudge` is a sum-type enum (not a trait with 3 single-method
//! impls). Match-based dispatch — ~3x fewer LOC than the trait approach in
//! Atom 17's cmd_tdma.rs.
//!
//! K11 fix: shared helpers (sha256_hex, write_jsonl, extract_body,
//! make_judge_stderr, Probe struct, kernel boot block, manifest writer) live
//! here in one place. Each callsite contributes only the (cfg, judge, LLM)
//! tuple.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use sha2::{Digest, Sha256};

use crate::charter_core::compile_charter_core;
use crate::judges::generate_judge::{GenerateJudge, GenerateStage};
use crate::judges::math_step_judge::JudgeVerdict;
use crate::judges::nesbitt_step_judge::{NesbittStage, NesbittStepJudge};
use crate::judges::putnam_2024_a1_judge::{PutnamA1Judge, PutnamA1Stage};
use crate::judges::putnam_2025_b3_judge::{PutnamB3Judge, PutnamB3Stage};
use crate::ledger::{AttemptScope, ImmutableTapeLedger, MemoryTapeLedger};
use crate::memory_kernel::{EnvironmentResult, KernelStep, MemoryKernel, Task};
use crate::token_budget::B_PROMPT_MAX;
use crate::tokenizer::Tokenizer;

// ── Public types ────────────────────────────────────────────────────

/// TRACE_MATRIX FC1a-judge_pi: Selectable judge sum-type. K10 fix —
/// replaces the JudgeDriver trait + 3 impls from Atom 17 with one enum.
/// Match-based dispatch reduces ~180 LOC of trait scaffolding to ~60.
pub enum AnyJudge {
    Nesbitt {
        judge: NesbittStepJudge,
        stages: Vec<NesbittStage>,
        cursor: usize,
    },
    PutnamA1 {
        judge: PutnamA1Judge,
        stages: Vec<PutnamA1Stage>,
        cursor: usize,
    },
    PutnamB3 {
        judge: PutnamB3Judge,
        stages: Vec<PutnamB3Stage>,
        cursor: usize,
    },
    Generate {
        judge: GenerateJudge,
        stages: Vec<GenerateStage>,
        cursor: usize,
    },
}

impl AnyJudge {
    /// TRACE_MATRIX FC1a-judge_pi: Construct a Nesbitt judge.
    pub fn nesbitt() -> Self {
        Self::Nesbitt {
            judge: NesbittStepJudge::new(),
            stages: vec![
                NesbittStage::Step1Substitute,
                NesbittStage::Step2Rewrite,
                NesbittStage::Step3Expand,
                NesbittStage::Step4Group,
                NesbittStage::Step5ApplyAmGm,
                NesbittStage::Step6Sum,
                NesbittStage::Step7Subtract,
                NesbittStage::Step8ConcludeAndEq,
            ],
            cursor: 0,
        }
    }

    /// TRACE_MATRIX FC1a-judge_pi: Construct a Putnam 2024 A1 judge.
    pub fn putnam_a1() -> Self {
        Self::PutnamA1 {
            judge: PutnamA1Judge::new(),
            stages: vec![
                PutnamA1Stage::Stage1WitnessN1,
                PutnamA1Stage::Stage2WlogGcd,
                PutnamA1Stage::Stage3N2Mod3,
                PutnamA1Stage::Stage4N2ContradictB,
                PutnamA1Stage::Stage5N3DescentB,
                PutnamA1Stage::Stage6N3DescentA,
                PutnamA1Stage::Stage7N3DescentC,
                PutnamA1Stage::Stage8Conclude,
            ],
            cursor: 0,
        }
    }

    /// TRACE_MATRIX FC1a-judge_pi: Construct a Putnam 2025 B3 judge.
    pub fn putnam_b3() -> Self {
        Self::PutnamB3 {
            judge: PutnamB3Judge::new(),
            stages: vec![
                PutnamB3Stage::Stage1Simplify,
                PutnamB3Stage::Stage2Factor2010,
                PutnamB3Stage::Stage3Closure,
                PutnamB3Stage::Stage4Counterex,
                PutnamB3Stage::Stage5ConcludeNo,
            ],
            cursor: 0,
        }
    }

    /// TRACE_MATRIX FC1a-judge_pi: Construct a single-stage Generate judge.
    pub fn generate(expected_entrypoint: String, enable_compile_check: bool) -> Self {
        Self::Generate {
            judge: GenerateJudge::new(expected_entrypoint, enable_compile_check),
            stages: vec![GenerateStage::Compile],
            cursor: 0,
        }
    }

    /// TRACE_MATRIX FC1a-judge_pi: Total canonical stages in the proof.
    pub fn total_stages(&self) -> usize {
        match self {
            Self::Nesbitt { stages, .. } => stages.len(),
            Self::PutnamA1 { stages, .. } => stages.len(),
            Self::PutnamB3 { stages, .. } => stages.len(),
            Self::Generate { stages, .. } => stages.len(),
        }
    }

    /// TRACE_MATRIX FC1a-judge_pi: Human-readable current stage label.
    pub fn current_stage_label(&self) -> String {
        match self {
            Self::Nesbitt { stages, cursor, .. } => stages[*cursor].label().to_string(),
            Self::PutnamA1 { stages, cursor, .. } => stages[*cursor].label().to_string(),
            Self::PutnamB3 { stages, cursor, .. } => stages[*cursor].label().to_string(),
            Self::Generate { stages, cursor, .. } => stages[*cursor].label().to_string(),
        }
    }

    /// TRACE_MATRIX FC1a-judge_pi: Run verdict on candidate body.
    /// Returns (success, reject_class_str, failed_predicate_str, judge_reason).
    pub fn verdict(&self, body: &str, accepted_steps: &[String]) -> (bool, String, String, String) {
        let (v, class_str, pred_str): (JudgeVerdict, String, String) = match self {
            Self::Nesbitt {
                judge,
                stages,
                cursor,
            } => {
                let stage = stages[*cursor];
                let (v, c) = judge.verdict_for_stage(body, stage, accepted_steps);
                let cs = c.map(|x| x.reject_class_str().to_string());
                let ps = c.map(|x| x.failed_predicate_str().to_string());
                (
                    v,
                    cs.unwrap_or_else(|| "pass".to_string()),
                    ps.unwrap_or_else(|| "pass".to_string()),
                )
            }
            Self::PutnamA1 {
                judge,
                stages,
                cursor,
            } => {
                let stage = stages[*cursor];
                let (v, c) = judge.verdict_for_stage(body, stage, accepted_steps);
                let cs = c.map(|x| x.reject_class_str().to_string());
                let ps = c.map(|x| x.failed_predicate_str().to_string());
                (
                    v,
                    cs.unwrap_or_else(|| "pass".to_string()),
                    ps.unwrap_or_else(|| "pass".to_string()),
                )
            }
            Self::PutnamB3 {
                judge,
                stages,
                cursor,
            } => {
                let stage = stages[*cursor];
                let (v, c) = judge.verdict_for_stage(body, stage, accepted_steps);
                let cs = c.map(|x| x.reject_class_str().to_string());
                let ps = c.map(|x| x.failed_predicate_str().to_string());
                (
                    v,
                    cs.unwrap_or_else(|| "pass".to_string()),
                    ps.unwrap_or_else(|| "pass".to_string()),
                )
            }
            Self::Generate {
                judge,
                stages,
                cursor,
            } => {
                let stage = stages[*cursor];
                let (v, c) = judge.verdict_for_stage(body, stage, accepted_steps);
                let cs = c.map(|x| x.reject_class_str().to_string());
                let ps = c.map(|x| x.failed_predicate_str().to_string());
                (
                    v,
                    cs.unwrap_or_else(|| "pass".to_string()),
                    ps.unwrap_or_else(|| "pass".to_string()),
                )
            }
        };
        let success = v.is_pass();
        let reason = match v {
            JudgeVerdict::Pass => "passed".to_string(),
            JudgeVerdict::Fail { reason } => reason,
            JudgeVerdict::NeedsClarification { question } => question,
        };
        (success, class_str, pred_str, reason)
    }

    /// TRACE_MATRIX FC1a-judge_pi: Promote stage after a successful step.
    pub fn advance(&mut self) {
        match self {
            Self::Nesbitt {
                judge,
                stages,
                cursor,
            } => {
                judge.advance();
                if *cursor + 1 < stages.len() {
                    *cursor += 1;
                }
            }
            Self::PutnamA1 {
                judge,
                stages,
                cursor,
            } => {
                judge.advance();
                if *cursor + 1 < stages.len() {
                    *cursor += 1;
                }
            }
            Self::PutnamB3 {
                judge,
                stages,
                cursor,
            } => {
                judge.advance();
                if *cursor + 1 < stages.len() {
                    *cursor += 1;
                }
            }
            Self::Generate {
                judge,
                stages,
                cursor,
            } => {
                judge.advance();
                if *cursor + 1 < stages.len() {
                    *cursor += 1;
                }
            }
        }
    }
}

/// TRACE_MATRIX FC1a-rtool: One round of LLM response data from an external caller.
pub struct LlmResponse {
    pub content: String,
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
}

/// TRACE_MATRIX FC1a-rtool: Configuration for a single `run_proof` invocation.
pub struct RunConfig {
    pub run_id: String,
    pub model_label: String,
    pub problem_label: String,
    pub leak_sentinel: String,
    pub system_prompt_for_stage: Box<dyn Fn(&str) -> String>,
    pub user_prompt_for_stage: Box<dyn Fn(&str, &[String]) -> String>,
    pub problem_text: String,
    pub evidence_dir: PathBuf,
    pub temperature: f32,
    pub max_tokens: u32,
    pub max_attempts_per_stage: usize,
}

/// TRACE_MATRIX FC3-replay: Per-attempt probe record (single source of truth
/// for what the run captured).
#[derive(Debug, Clone)]
pub struct Probe {
    pub stage: String,
    pub attempt: usize,
    pub kernel_step: String,
    pub judge_class: String,
    pub completion_tokens: u32,
    pub prompt_tokens_from_llm: u32,
    pub judge_reason: String,
    pub candidate_body_preview: String,
    pub bbs_constraint_count: usize,
    pub bbs_token_count: usize,
    pub bbs_zero_gain: u32,
    pub prompt_tokens: usize,
    pub raw_stderr_bytes: usize,
    pub leak_in_prompt: bool,
    pub wall_clock_ms: u128,
}

impl Probe {
    /// TRACE_MATRIX FC3-replay: Serialize one probe record as a JSON line.
    pub fn to_jsonl(&self) -> String {
        serde_json::json!({
            "stage": self.stage,
            "attempt": self.attempt,
            "kernel_step": self.kernel_step,
            "judge_class": self.judge_class,
            "completion_tokens": self.completion_tokens,
            "prompt_tokens_from_llm": self.prompt_tokens_from_llm,
            "judge_reason": self.judge_reason,
            "candidate_body_preview": self.candidate_body_preview,
            "bbs_constraint_count": self.bbs_constraint_count,
            "bbs_token_count": self.bbs_token_count,
            "bbs_zero_gain": self.bbs_zero_gain,
            "prompt_tokens": self.prompt_tokens,
            "raw_stderr_bytes": self.raw_stderr_bytes,
            "leak_in_prompt": self.leak_in_prompt,
            "wall_clock_ms": self.wall_clock_ms as u64,
        })
        .to_string()
    }
}

/// TRACE_MATRIX FC3-replay: Summary of a single proof run.
pub struct RunSummary {
    pub stages_total: usize,
    pub stages_completed: usize,
    pub stages_escalated: Vec<String>,
    pub probes: Vec<Probe>,
    pub total_raw_stderr_bytes: usize,
    pub total_llm_completion_tokens: u32,
    pub total_llm_prompt_tokens: u32,
    pub leak_anywhere: bool,
    pub total_wall_clock_ms: u128,
    pub per_stage_attempts: Vec<(String, usize, usize, String)>,
}

// ── Helpers ─────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-replay: SHA-256 hex over arbitrary bytes.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

/// TRACE_MATRIX FC3-replay: Write JSON-lines file and return its sha256.
pub fn write_jsonl(path: &Path, lines: &[String]) -> std::io::Result<String> {
    let body = if lines.is_empty() {
        String::new()
    } else {
        lines.join("\n") + "\n"
    };
    fs::write(path, &body)?;
    Ok(sha256_hex(body.as_bytes()))
}

/// TRACE_MATRIX FC1a-output_edge: Extract the body after the `---BODY---`
/// separator (or return the full content trimmed if the marker is absent).
pub fn extract_body(raw: &str) -> String {
    if let Some(idx) = raw.find("---BODY---") {
        raw[idx + "---BODY---".len()..].trim().to_string()
    } else {
        raw.trim().to_string()
    }
}

/// TRACE_MATRIX FC1a-rtool_input: Build a synthetic ~10 KB raw_stderr blob
/// embedding the leak-sentinel + judge verdict context. The kernel commits
/// the full blob to tape (as evidence) but the distiller compresses it
/// before it ever flows back into a prompt.
pub fn make_judge_stderr(
    leak_sentinel: &str,
    stage_label: &str,
    class_str: &str,
    pred_str: &str,
    reason: &str,
    candidate_body: &str,
    attempt: usize,
) -> String {
    let mut s = String::new();
    s.push_str(leak_sentinel);
    s.push('\n');
    s.push_str(&format!(
        "Judge rejected stage {} (attempt {})\n",
        stage_label, attempt
    ));
    s.push_str(&format!("reject_class: {}\n", class_str));
    s.push_str(&format!("failed_predicate: {}\n", pred_str));
    s.push_str(&format!("judge_reason: {}\n", reason));
    s.push_str("traceback:\n");
    s.push_str("  at src/judges/* in verdict_for_stage\n");
    s.push_str("  at src/memory_kernel.rs in handle_rejection\n");
    s.push_str(&format!(
        "\nCandidate body:\n{}\n",
        candidate_body.chars().take(2500).collect::<String>()
    ));
    let template = format!(
        "  > Judge rejection at stage {}: class={} predicate={}. ",
        stage_label, class_str, pred_str
    );
    while s.len() < 10 * 1024 {
        s.push_str(&template);
    }
    s.push_str("\n[end stderr]\n");
    s
}

// ── Core runner ─────────────────────────────────────────────────────

/// TRACE_MATRIX FC1a-rtool + FC1b-wtool: Single canonical TDMA-Bounded proof
/// runner. Replaces the 4 duplicate loops in Atoms 12-17 with one
/// parameterized implementation.
///
/// The LLM call is supplied by the caller via the `llm_call` closure so this
/// function does not depend on either the production chat_client (used
/// by cmd_tdma) or the test-proxy `ResilientLLMClient` (used by the standalone
/// evidence binaries).
pub fn run_proof<F>(cfg: RunConfig, judge: &mut AnyJudge, llm_call: F) -> Result<RunSummary, String>
where
    F: FnMut(&str, &str) -> Result<LlmResponse, String>,
{
    // Atom 20: thin wrapper over the generic implementation. Default ledger
    // remains MemoryTapeLedger; new callers (Atom 25+) use
    // run_proof_with_ledger directly with a GitTapeLedger.
    let tape = MemoryTapeLedger::new();
    run_proof_with_ledger(cfg, judge, tape, llm_call)
}

/// TRACE_MATRIX FC1a-rtool + FC1a-substrate_seam: Generic-over-ledger variant
/// of `run_proof`. Atom 20 introduces this seam so Atoms 24+ can wire
/// `GitTapeLedger` as the substrate without forking the runner body.
pub fn run_proof_with_ledger<F, L>(
    cfg: RunConfig,
    judge: &mut AnyJudge,
    mut tape: L,
    mut llm_call: F,
) -> Result<RunSummary, String>
where
    F: FnMut(&str, &str) -> Result<LlmResponse, String>,
    L: ImmutableTapeLedger,
{
    fs::create_dir_all(&cfg.evidence_dir)
        .map_err(|e| format!("cannot create evidence-dir: {}", e))?;
    // Atom 27 (F1 cross-CLI kernel resume): only initialize verified_head to
    // "H0" if there isn't already a real one. This preserves kernel-semantic
    // state across CLI invocations when using a durable substrate
    // (GitTapeLedger). For MemoryTapeLedger, this is a no-op (empty ledger
    // returns "H0" sentinel anyway). For GitTapeLedger, the prior session's
    // verified_head is read from refs/tdma/verified_head and chained forward.
    let existing_head = tape.get_verified_head();
    if existing_head.is_empty() || existing_head == "H0" {
        tape.set_verified_head("H0".into());
    } else {
        eprintln!(
            "[tdma_runner] resuming from existing verified_head: {}",
            existing_head
        );
    }
    let charter = compile_charter_core(
        "# Constitution\nArt. 0.4 — Q_t Path A; FC1a tape_t; FC1b wtool.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    let mut kernel = MemoryKernel::new(tape, cfg.run_id.clone(), charter);
    let tk = Tokenizer::new();

    let mut accepted_steps: Vec<String> = Vec::new();
    let mut probes: Vec<Probe> = Vec::new();
    let mut total_stderr_bytes = 0usize;
    let mut total_completion_tokens = 0u32;
    let mut total_prompt_tokens = 0u32;
    let mut leak_anywhere = false;
    let mut stages_completed = 0usize;
    let mut stages_escalated: Vec<String> = Vec::new();
    let mut per_stage_attempts: Vec<(String, usize, usize, String)> = Vec::new();
    let run_start = Instant::now();

    let total_stages = judge.total_stages();
    'outer: for _stage_idx in 0..total_stages {
        let stage_label = judge.current_stage_label();
        let task_id = format!("{}-{}", cfg.run_id, stage_label);
        let task = Task {
            id: task_id.clone(),
            prompt: (cfg.user_prompt_for_stage)(&stage_label, &accepted_steps),
        };
        let scope_at_start = AttemptScope {
            run_id: cfg.run_id.clone(),
            task_id: task_id.clone(),
            verified_parent: kernel.tape.get_verified_head(),
        };

        let mut attempts_used = 0usize;
        let mut stage_outcome = "incomplete".to_string();
        let mut next_retry_prompt: Option<String> = None;

        loop {
            if attempts_used >= cfg.max_attempts_per_stage {
                stage_outcome = "cap-reached".into();
                break;
            }
            attempts_used += 1;

            let attempt_start = Instant::now();
            let system_prompt = (cfg.system_prompt_for_stage)(&stage_label);
            let user_prompt = next_retry_prompt.take().unwrap_or_else(|| {
                format!(
                    "{}{}",
                    (cfg.user_prompt_for_stage)(&stage_label, &accepted_steps),
                    if attempts_used > 1 {
                        "\n\n[NOTE: prior attempt was rejected by the verifier — provide more explicit reasoning.]"
                    } else {
                        ""
                    }
                )
            });

            let llm_resp = match llm_call(&system_prompt, &user_prompt) {
                Ok(r) => r,
                Err(e) => {
                    stages_escalated.push(format!("{}/llm-error", stage_label));
                    per_stage_attempts.push((
                        stage_label.clone(),
                        attempts_used,
                        0,
                        format!("llm-error: {}", e),
                    ));
                    break 'outer;
                }
            };
            total_completion_tokens += llm_resp.completion_tokens;
            total_prompt_tokens += llm_resp.prompt_tokens;

            let body = extract_body(&llm_resp.content);
            eprintln!(
                "[tdma_runner] {} attempt {} | tokens={}/{} | body[0..120]: {}",
                stage_label,
                attempts_used,
                llm_resp.prompt_tokens,
                llm_resp.completion_tokens,
                body.chars().take(120).collect::<String>()
            );

            let (success, judge_class_str, failed_pred_str, judge_reason) =
                judge.verdict(&body, &accepted_steps);

            let raw_stderr = if success {
                String::new()
            } else {
                let s = make_judge_stderr(
                    &cfg.leak_sentinel,
                    &stage_label,
                    &judge_class_str,
                    &failed_pred_str,
                    &judge_reason,
                    &body,
                    attempts_used,
                );
                total_stderr_bytes += s.len();
                s
            };

            let env = EnvironmentResult {
                raw_output: llm_resp.content.clone(),
                raw_stderr: raw_stderr.clone(),
                success,
            };
            let step = kernel.step_forward(&task, env);

            let bbs = kernel
                .tape
                .derive_latest_belief_state_from_tape(&scope_at_start);
            let (cc, ct, zgs) = match &bbs {
                Some(b) => (b.constraints.len(), tk.count_json(b), b.zero_gain_streak),
                None => (0, 0, 0),
            };

            let (kernel_step_str, prompt_tokens, leak) = match step {
                KernelStep::Proceed { .. } => {
                    accepted_steps.push(body.clone());
                    judge.advance();
                    stage_outcome = "passed".into();
                    ("Proceed".to_string(), 0, false)
                }
                KernelStep::Retry { prompt, .. } => {
                    let n = tk.count_text(&prompt);
                    let leak = prompt.contains(&cfg.leak_sentinel);
                    if leak {
                        leak_anywhere = true;
                    }
                    next_retry_prompt = Some(prompt);
                    ("Retry".to_string(), n, leak)
                }
                KernelStep::Escalate { reason, .. } => {
                    stages_escalated.push(format!("{}/{}", stage_label, reason));
                    stage_outcome = format!("escalate-{}", reason);
                    (format!("Escalate({})", reason), 0, false)
                }
            };

            probes.push(Probe {
                stage: stage_label.clone(),
                attempt: attempts_used,
                kernel_step: kernel_step_str.clone(),
                judge_class: judge_class_str,
                completion_tokens: llm_resp.completion_tokens,
                prompt_tokens_from_llm: llm_resp.prompt_tokens,
                judge_reason: judge_reason.clone(),
                candidate_body_preview: body.chars().take(220).collect::<String>(),
                bbs_constraint_count: cc,
                bbs_token_count: ct,
                bbs_zero_gain: zgs,
                prompt_tokens,
                raw_stderr_bytes: raw_stderr.len(),
                leak_in_prompt: leak,
                wall_clock_ms: attempt_start.elapsed().as_millis(),
            });

            if kernel_step_str == "Proceed" {
                let final_cc = kernel
                    .tape
                    .derive_latest_belief_state_from_tape(&scope_at_start)
                    .map(|b| b.constraints.len())
                    .unwrap_or(0);
                per_stage_attempts.push((
                    stage_label.clone(),
                    attempts_used,
                    final_cc,
                    stage_outcome.clone(),
                ));
                stages_completed += 1;
                continue 'outer;
            }
            if kernel_step_str.starts_with("Escalate") {
                let final_cc = kernel
                    .tape
                    .derive_latest_belief_state_from_tape(&scope_at_start)
                    .map(|b| b.constraints.len())
                    .unwrap_or(0);
                per_stage_attempts.push((
                    stage_label.clone(),
                    attempts_used,
                    final_cc,
                    stage_outcome.clone(),
                ));
                break 'outer;
            }
        }

        if stage_outcome == "cap-reached" {
            per_stage_attempts.push((stage_label.clone(), attempts_used, 0, stage_outcome));
            break 'outer;
        }
    }

    // Write evidence
    let probe_lines: Vec<String> = probes.iter().map(|p| p.to_jsonl()).collect();
    let probes_sha = write_jsonl(
        &cfg.evidence_dir.join("per_attempt_probes.jsonl"),
        &probe_lines,
    )
    .unwrap_or_default();

    let mut chaintape_lines: Vec<String> = Vec::new();
    for (h, node) in kernel.tape.dump_all_nodes().iter() {
        chaintape_lines.push(
            serde_json::json!({
                "hash": h,
                "kind": serde_json::to_value(&node.kind).unwrap_or(serde_json::json!(null)),
                "verified": node.verified,
                "parent": node.parent,
                "scope": node.scope,
                "attempt_ordinal": node.attempt_ordinal,
                "reject_class": node.reject_class,
            })
            .to_string(),
        );
    }
    let chaintape_sha = write_jsonl(&cfg.evidence_dir.join("chaintape.jsonl"), &chaintape_lines)
        .unwrap_or_default();

    let retry_probes: Vec<&Probe> = probes
        .iter()
        .filter(|p| p.kernel_step.starts_with("Retry"))
        .collect();
    let prompt_min = retry_probes
        .iter()
        .map(|p| p.prompt_tokens)
        .min()
        .unwrap_or(0);
    let prompt_max = retry_probes
        .iter()
        .map(|p| p.prompt_tokens)
        .max()
        .unwrap_or(0);
    let total_bbs_bytes: usize = retry_probes.iter().map(|p| p.bbs_token_count * 4).sum();
    let compression_ratio = if total_bbs_bytes == 0 {
        0.0
    } else {
        total_stderr_bytes as f64 / total_bbs_bytes as f64
    };
    let mut classes_seen = BTreeSet::new();
    for p in &retry_probes {
        classes_seen.insert(p.judge_class.clone());
    }
    let zero_gain_max = retry_probes
        .iter()
        .map(|p| p.bbs_zero_gain)
        .max()
        .unwrap_or(0);

    let manifest = serde_json::json!({
        "run_id": cfg.run_id,
        "model_label": cfg.model_label,
        "problem_label": cfg.problem_label,
        "temperature": cfg.temperature,
        "max_attempts_per_stage": cfg.max_attempts_per_stage,
        "stages_total": total_stages,
        "stages_completed": stages_completed,
        "stages_escalated": stages_escalated.clone(),
        "total_attempts": probes.len(),
        "total_failed_attempts": retry_probes.len(),
        "total_raw_stderr_bytes": total_stderr_bytes,
        "total_bbs_bytes_estimated": total_bbs_bytes,
        "compression_ratio": compression_ratio,
        "prompt_tokens_min": prompt_min,
        "prompt_tokens_max": prompt_max,
        "prompt_tokens_variance": prompt_max.saturating_sub(prompt_min),
        "all_prompts_within_budget": retry_probes.iter().all(|p| p.prompt_tokens <= B_PROMPT_MAX),
        "b_prompt_max": B_PROMPT_MAX,
        "max_zero_gain_streak": zero_gain_max,
        "distinct_judge_classes": classes_seen.iter().cloned().collect::<Vec<_>>(),
        "leak_in_any_prompt": leak_anywhere,
        "total_wall_clock_ms": run_start.elapsed().as_millis() as u64,
        "total_llm_completion_tokens": total_completion_tokens,
        "total_llm_prompt_tokens": total_prompt_tokens,
        "per_stage": per_stage_attempts.iter().map(|(s, a, c, o)| {
            serde_json::json!({"stage": s, "attempts_used": a, "final_constraints": c, "outcome": o})
        }).collect::<Vec<_>>(),
        "probes_sha256": probes_sha,
        "chaintape_sha256": chaintape_sha,
    });
    fs::write(
        cfg.evidence_dir.join("manifest.json"),
        serde_json::to_string_pretty(&manifest).unwrap_or_default(),
    )
    .map_err(|e| format!("cannot write manifest.json: {}", e))?;

    Ok(RunSummary {
        stages_total: total_stages,
        stages_completed,
        stages_escalated,
        probes,
        total_raw_stderr_bytes: total_stderr_bytes,
        total_llm_completion_tokens: total_completion_tokens,
        total_llm_prompt_tokens: total_prompt_tokens,
        leak_anywhere,
        total_wall_clock_ms: run_start.elapsed().as_millis(),
        per_stage_attempts,
    })
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn any_judge_construction_each_variant() {
        let n = AnyJudge::nesbitt();
        assert_eq!(n.total_stages(), 8);
        let a = AnyJudge::putnam_a1();
        assert_eq!(a.total_stages(), 8);
        let b = AnyJudge::putnam_b3();
        assert_eq!(b.total_stages(), 5);
    }

    #[test]
    fn any_judge_stage_labels_are_distinct_per_judge() {
        let n = AnyJudge::nesbitt();
        let a = AnyJudge::putnam_a1();
        let b = AnyJudge::putnam_b3();
        assert!(n.current_stage_label().starts_with("Step"));
        assert!(a.current_stage_label().starts_with("Stage"));
        assert!(b.current_stage_label().starts_with("Stage"));
    }

    #[test]
    fn any_judge_advance_increments_cursor() {
        let mut n = AnyJudge::nesbitt();
        let label0 = n.current_stage_label();
        n.advance();
        let label1 = n.current_stage_label();
        assert_ne!(label0, label1);
    }

    #[test]
    fn make_judge_stderr_embeds_sentinel_and_size() {
        let s = make_judge_stderr(
            "SENTINEL_TEST_XYZ",
            "Step1",
            "off-stage",
            "stage.unrecognized",
            "judge said no",
            "candidate text",
            1,
        );
        assert!(s.contains("SENTINEL_TEST_XYZ"));
        assert!(s.contains("off-stage"));
        assert!(s.len() >= 10 * 1024);
    }

    #[test]
    fn cap_reached_reports_actual_attempt_count() {
        let tmp = tempfile::TempDir::new().expect("tempdir");
        let cfg = RunConfig {
            run_id: "cap-test".to_string(),
            model_label: "mock-model".to_string(),
            problem_label: "mock proof".to_string(),
            leak_sentinel: "CAP_SENTINEL".to_string(),
            system_prompt_for_stage: Box::new(|stage| format!("system {stage}")),
            user_prompt_for_stage: Box::new(|stage, _accepted| format!("user {stage}")),
            problem_text: String::new(),
            evidence_dir: tmp.path().to_path_buf(),
            temperature: 0.0,
            max_tokens: 128,
            max_attempts_per_stage: 2,
        };
        let mut judge = AnyJudge::putnam_b3();
        let summary = run_proof(cfg, &mut judge, |_sys, _user| {
            Ok(LlmResponse {
                content: "too short".to_string(),
                completion_tokens: 2,
                prompt_tokens: 3,
            })
        })
        .expect("cap-reached is written as evidence, not a runner panic");

        assert_eq!(summary.stages_completed, 0);
        assert_eq!(summary.probes.len(), 2);
        assert_eq!(summary.per_stage_attempts.len(), 1);
        assert_eq!(summary.per_stage_attempts[0].0, "Stage1-Simplify-2010n");
        assert_eq!(summary.per_stage_attempts[0].1, 2);
        assert_eq!(summary.per_stage_attempts[0].3, "cap-reached");

        let manifest: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(tmp.path().join("manifest.json")).expect("manifest"),
        )
        .expect("manifest json");
        assert_eq!(manifest["total_attempts"], 2);
        assert_eq!(manifest["per_stage"][0]["attempts_used"], 2);
    }

    #[test]
    fn retry_attempt_uses_kernel_belief_state_prompt() {
        let tmp = tempfile::TempDir::new().expect("tempdir");
        let cfg = RunConfig {
            run_id: "retry-prompt-test".to_string(),
            model_label: "mock-model".to_string(),
            problem_label: "mock proof".to_string(),
            leak_sentinel: "RETRY_SENTINEL".to_string(),
            system_prompt_for_stage: Box::new(|stage| format!("system {stage}")),
            user_prompt_for_stage: Box::new(|stage, _accepted| format!("base user {stage}")),
            problem_text: String::new(),
            evidence_dir: tmp.path().to_path_buf(),
            temperature: 0.0,
            max_tokens: 128,
            max_attempts_per_stage: 2,
        };
        let seen_prompts: RefCell<Vec<String>> = RefCell::new(Vec::new());
        let mut judge = AnyJudge::putnam_b3();

        let _summary = run_proof(cfg, &mut judge, |_sys, user| {
            seen_prompts.borrow_mut().push(user.to_string());
            let call_idx = seen_prompts.borrow().len();
            let content = if call_idx == 1 {
                "too short".to_string()
            } else if call_idx == 2 {
                [
                    r#"{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"Stage1-Simplify-2010n","action":"PROPOSE","failed_predicate":null,"reject_class":null,"next_action_hint":null,"evidence_hash":null}"#,
                    "---BODY---",
                    "For any n in S, the expression 2025n - 15n is exactly (2025 - 15)n = 2010n. This explicit simplification identifies the divisor rule as taking every positive divisor of 2010n, which is the arithmetic form needed for the later prime-factor closure argument.",
                ]
                .join("\n")
            } else {
                "too short".to_string()
            };
            Ok(LlmResponse {
                content,
                completion_tokens: 2,
                prompt_tokens: 3,
            })
        })
        .expect("runner writes cap evidence instead of panicking");

        let prompts = seen_prompts.borrow();
        assert!(prompts.len() >= 2);
        assert!(prompts[0].contains("base user Stage1-Simplify-2010n"));
        assert!(prompts[1].contains("[RETRY BELIEF STATE]"));
        assert!(prompts[1].contains("[EVIDENCE POINTERS]"));
        assert!(!prompts[1].contains("RETRY_SENTINEL"));
    }
}
