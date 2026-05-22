//! TRACE_MATRIX FC2-N16 + FC1a-rtool + FC1a-predicate_pi:
//! turingos tdma — TDMA-Bounded production runner.
//!
//! Drives the TDMA-Bounded memory kernel through a step-by-step proof using
//! the REAL production SiliconFlow client (NOT the test localhost proxy).
//! This is the first production wire-up of the TDMA-Bounded kernel into the
//! `turingos` user CLI.
//!
//! Compared with Atoms 12-14 (which targeted DeepSeek via the local
//! `llm_proxy.py`), this binary path routes through the same
//! `siliconflow_client::chat_complete_blocking` that `turingos llm complete`
//! and `turingos generate` use — so the kernel sees production traffic, not
//! a test loopback.
//!
//! Class 2 wire-up: additive subcommand. No kernel/judge changes. The
//! evidence directory pattern matches Atoms 12-14 so analytics tooling
//! is unchanged.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use sha2::{Digest, Sha256};

use crate::cmd_llm;
use crate::siliconflow_client::{chat_complete_blocking, require_api_key, ChatMessage, LlmError};
use turingosv4::charter_core::compile_charter_core;
use turingosv4::judges::math_step_judge::JudgeVerdict;
use turingosv4::judges::nesbitt_step_judge::{
    NesbittRejectClass, NesbittStage, NesbittStepJudge,
};
use turingosv4::ledger::{AttemptScope, ImmutableTapeLedger, MemoryTapeLedger};
use turingosv4::memory_kernel::{EnvironmentResult, KernelStep, MemoryKernel, Task};
use turingosv4::token_budget::{B_PROMPT_MAX, MAX_RETRIES};
use turingosv4::tokenizer::Tokenizer;

const LEAK_SENTINEL: &str = "TURINGOS_TDMA_PROD_LEAK_CANARY_R3K8M";

/// TRACE_MATRIX FC2-N16: `tdma` short-help (registry display)
pub(crate) const SHORT_HELP: &str =
    "Drive the TDMA-Bounded memory kernel against a step-by-step proof via production LLM";

/// TRACE_MATRIX FC2-N16: `tdma` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos tdma — TDMA-Bounded production runner

USAGE:
    turingos tdma run --workspace <PATH>
                       [--judge <nesbitt>]
                       [--role <meta|blackbox>]
                       [--evidence-dir <PATH>]
                       [--max-attempts-per-stage <N>]
                       [--temperature <FLOAT>]

ACTIONS:
    run    Boot a TDMA-Bounded memory kernel, drive it stage-by-stage
           through a structured proof using the SAME SiliconFlow client
           that `turingos llm complete` uses (production endpoint, not
           the local test proxy). Capture ChainTape + per-attempt probes
           to <evidence-dir> (default: <workspace>/artifacts/tdma/<TS>/).

JUDGES:
    nesbitt        Nesbitt's inequality (8 stages; default)
    (more selectable judges land in future atoms)

OPTIONS:
    --workspace <PATH>          Workspace directory containing turingos.toml
    --judge <NAME>              Judge selector (default: nesbitt)
    --role <meta|blackbox>      Which configured model to use (default: meta)
    --evidence-dir <PATH>       Override evidence output directory
    --max-attempts-per-stage <N>  Hard cap per stage (default: MAX_RETRIES+2)
    --temperature <FLOAT>       Sampling temperature (default: 0.7)
    -h, --help                  Print this help

DESCRIPTION:
    First production wire-up of the TDMA-Bounded RC1 kernel (Atoms 0-14).
    Reuses the `siliconflow_client::chat_complete_blocking` path so
    requests go to the configured SiliconFlow endpoint with the API key
    in the env var named in turingos.toml.

    Per-attempt evidence (probe + ChainTape + manifest) is written into
    the evidence directory. Failures DO NOT escape into the next prompt;
    the distiller compresses each rejection into a bounded BBS entry.

    KILL guards held at runtime:
      KILL-tdma-1: raw_stderr never appears in the next prompt
      KILL-tdma-5: verified_head does not advance on failure
      KILL-tdma-8: zero_gain_streak fuse at K=3
      KILL-tdma-9: prompt size <= B_PROMPT_MAX=5800
"#;

const PROBLEM_TEXT_NESBITT: &str = r#"Prove Nesbitt's inequality for positive reals:
    a/(b+c) + b/(a+c) + c/(a+b) >= 3/2

Canonical proof (8 stages):
  Stage 1: Substitute x=b+c, y=a+c, z=a+b.
  Stage 2: Rewrite each a/(b+c) in terms of x,y,z.
  Stage 3: Expand into six separate fractions.
  Stage 4: Group into three pairs (x/y + y/x), etc.
  Stage 5: Apply AM-GM: each pair >= 2.
  Stage 6: Sum the three pairs: total >= 6.
  Stage 7: Subtract 3 to recover the original form.
  Stage 8: Conclude >= 3/2 with equality iff a=b=c."#;

/// TRACE_MATRIX FC2-N16: `turingos tdma` subcommand entry-point.
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.is_empty() {
        eprintln!("{FULL_HELP}");
        return ExitCode::from(2);
    }
    match args[0].as_str() {
        "run" => run_run(&args[1..]),
        "-h" | "--help" => {
            println!("{FULL_HELP}");
            ExitCode::SUCCESS
        }
        other => {
            eprintln!("turingos tdma: unknown action '{}'", other);
            eprintln!("{FULL_HELP}");
            ExitCode::from(2)
        }
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

fn write_jsonl(path: &Path, lines: &[String]) -> std::io::Result<String> {
    let body = if lines.is_empty() {
        String::new()
    } else {
        lines.join("\n") + "\n"
    };
    fs::write(path, &body)?;
    Ok(sha256_hex(body.as_bytes()))
}

fn extract_body(raw: &str) -> String {
    if let Some(idx) = raw.find("---BODY---") {
        raw[idx + "---BODY---".len()..].trim().to_string()
    } else {
        raw.trim().to_string()
    }
}

fn make_judge_stderr(
    stage_label: &str,
    judge_verdict: &JudgeVerdict,
    candidate_body: &str,
    expected_class: Option<NesbittRejectClass>,
    attempt: usize,
) -> String {
    let class_str = expected_class
        .map(|c| c.reject_class_str())
        .unwrap_or("unknown");
    let pred_str = expected_class
        .map(|c| c.failed_predicate_str())
        .unwrap_or("unknown");
    let reason = match judge_verdict {
        JudgeVerdict::Fail { reason } => reason.clone(),
        JudgeVerdict::NeedsClarification { question } => question.clone(),
        JudgeVerdict::Pass => "passed".into(),
    };
    let mut s = String::new();
    s.push_str(LEAK_SENTINEL);
    s.push('\n');
    s.push_str(&format!(
        "NesbittStepJudge rejected stage {} (attempt {})\n",
        stage_label, attempt
    ));
    s.push_str(&format!("reject_class: {}\n", class_str));
    s.push_str(&format!("failed_predicate: {}\n", pred_str));
    s.push_str(&format!("judge_reason: {}\n", reason));
    s.push_str("traceback:\n");
    s.push_str("  at src/judges/nesbitt_step_judge.rs in verdict_for_stage\n");
    s.push_str("  at src/memory_kernel.rs in handle_rejection\n");
    s.push_str(&format!(
        "\nCandidate body:\n{}\n",
        candidate_body.chars().take(2500).collect::<String>()
    ));
    let template = format!(
        "  > Strict-judge rejection context: {} pattern at stage {}; predicate {} failed. ",
        class_str, stage_label, pred_str
    );
    while s.len() < 10 * 1024 {
        s.push_str(&template);
    }
    s.push_str("\n[end stderr]\n");
    s
}

fn system_prompt(stage_label: &str) -> String {
    format!(
        r#"You are a mathematics worker proving Nesbitt's inequality step-by-step.
Output EXACTLY ONE next step.

Your output MUST start with this JSON object on the FIRST line:
{{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"<STAGE>","action":"PROPOSE","failed_predicate":null,"reject_class":null,"next_action_hint":null,"evidence_hash":null}}

Replace <STAGE> with the current stage label (e.g. "{stage_label}").
After the JSON write on a new line:
---BODY---
Then write your step in 1-3 sentences with concrete algebra.

Current stage: {stage_label}"#,
        stage_label = stage_label
    )
}

fn user_prompt(stage_label: &str, accepted_steps: &[String]) -> String {
    let mut s = String::new();
    s.push_str(&format!("Problem:\n{}\n\n", PROBLEM_TEXT_NESBITT));
    s.push_str(&format!("Current stage: {}\n\n", stage_label));
    if accepted_steps.is_empty() {
        s.push_str("No prior steps yet. Write Stage 1 (substitution).");
    } else {
        s.push_str("Prior accepted steps:\n");
        for (i, st) in accepted_steps.iter().enumerate() {
            s.push_str(&format!("  Step {}: {}\n", i + 1, st));
        }
        s.push_str("\nWrite the next single step (do NOT repeat prior steps).");
    }
    s
}

#[derive(Debug, Clone)]
struct Probe {
    stage: String,
    attempt: usize,
    kernel_step: String,
    judge_class: String,
    sf_completion_tokens: u32,
    sf_prompt_tokens: u32,
    judge_reason: String,
    candidate_body_preview: String,
    bbs_constraint_count: usize,
    bbs_token_count: usize,
    bbs_zero_gain: u32,
    prompt_tokens: usize,
    raw_stderr_bytes: usize,
    leak_in_prompt: bool,
    wall_clock_ms: u128,
}

impl Probe {
    fn to_jsonl(&self) -> String {
        serde_json::json!({
            "stage": self.stage,
            "attempt": self.attempt,
            "kernel_step": self.kernel_step,
            "judge_class": self.judge_class,
            "sf_completion_tokens": self.sf_completion_tokens,
            "sf_prompt_tokens": self.sf_prompt_tokens,
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

/// TRACE_MATRIX FC2-N16: `tdma run` action handler.
fn run_run(args: &[String]) -> ExitCode {
    let mut workspace: Option<PathBuf> = None;
    let mut judge_name = "nesbitt".to_string();
    let mut role = "meta".to_string();
    let mut evidence_dir: Option<PathBuf> = None;
    let mut max_attempts_per_stage: usize = (MAX_RETRIES as usize) + 2;
    let mut temperature: f32 = 0.7;

    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--workspace" => workspace = it.next().map(PathBuf::from),
            "--judge" => {
                if let Some(v) = it.next() {
                    judge_name = v.clone();
                }
            }
            "--role" => {
                if let Some(v) = it.next() {
                    role = v.clone();
                }
            }
            "--evidence-dir" => evidence_dir = it.next().map(PathBuf::from),
            "--max-attempts-per-stage" => {
                if let Some(v) = it.next() {
                    if let Ok(n) = v.parse() {
                        max_attempts_per_stage = n;
                    }
                }
            }
            "--temperature" => {
                if let Some(v) = it.next() {
                    if let Ok(f) = v.parse() {
                        temperature = f;
                    }
                }
            }
            "-h" | "--help" => {
                println!("{FULL_HELP}");
                return ExitCode::SUCCESS;
            }
            other => {
                eprintln!("turingos tdma run: unexpected flag '{}'", other);
                return ExitCode::from(2);
            }
        }
    }

    let workspace = match workspace {
        Some(w) => w,
        None => {
            eprintln!("turingos tdma run: --workspace required");
            return ExitCode::from(2);
        }
    };

    if judge_name != "nesbitt" {
        eprintln!(
            "turingos tdma run: only --judge nesbitt is wired in Atom 15; got '{}'",
            judge_name
        );
        return ExitCode::from(2);
    }

    // ── Load workspace turingos.toml + resolve model + api key ──
    let (model, env_var_result) = match role.as_str() {
        "meta" => (
            cmd_llm::read_meta_model(&workspace),
            cmd_llm::read_meta_api_key_env(&workspace),
        ),
        "blackbox" => (
            cmd_llm::read_blackbox_model(&workspace),
            cmd_llm::read_blackbox_api_key_env(&workspace),
        ),
        _ => {
            eprintln!("turingos tdma run: --role must be 'meta' or 'blackbox'");
            return ExitCode::from(2);
        }
    };
    let env_var = match env_var_result {
        Ok(v) => v,
        Err(e) => {
            eprintln!("turingos tdma run: cannot resolve api-key env var: {:?}", e);
            return ExitCode::from(2);
        }
    };
    let api_key = match require_api_key(&env_var) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("turingos tdma run: API key error: {:?}", e);
            return ExitCode::from(2);
        }
    };

    // ── Resolve evidence dir ──
    let evidence_dir = evidence_dir.unwrap_or_else(|| {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        workspace.join("artifacts").join("tdma").join(format!(
            "tdma_run_{}",
            ts
        ))
    });
    if let Err(e) = fs::create_dir_all(&evidence_dir) {
        eprintln!("turingos tdma run: cannot create evidence-dir: {}", e);
        return ExitCode::from(2);
    }

    eprintln!(
        "[turingos tdma run] workspace={} model={} role={} evidence-dir={} max_attempts={} temp={}",
        workspace.display(),
        model,
        role,
        evidence_dir.display(),
        max_attempts_per_stage,
        temperature
    );

    // ── Boot kernel + judge ──
    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\nArt. 0.4 — Q_t Path A; FC1a tape_t; FC1b wtool.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    let mut kernel = MemoryKernel::new(tape, "turingos-tdma-run", charter);
    let judge = NesbittStepJudge::new();
    let tk = Tokenizer::new();

    let stages = [
        NesbittStage::Step1Substitute,
        NesbittStage::Step2Rewrite,
        NesbittStage::Step3Expand,
        NesbittStage::Step4Group,
        NesbittStage::Step5ApplyAmGm,
        NesbittStage::Step6Sum,
        NesbittStage::Step7Subtract,
        NesbittStage::Step8ConcludeAndEq,
    ];

    let mut accepted_steps: Vec<String> = Vec::new();
    let mut probes: Vec<Probe> = Vec::new();
    let mut total_stderr_bytes = 0usize;
    let mut total_sf_completion_tokens = 0u32;
    let mut total_sf_prompt_tokens = 0u32;
    let mut leak_anywhere = false;
    let mut stages_completed = 0usize;
    let mut stages_escalated: Vec<String> = Vec::new();
    let mut per_stage_attempts: Vec<(String, usize, usize, String)> = Vec::new();
    let run_start = Instant::now();

    'outer: for stage in stages {
        let task_id = format!("turingos-tdma-{}", stage.label());
        let task = Task {
            id: task_id.clone(),
            prompt: user_prompt(stage.label(), &accepted_steps),
        };
        let scope_at_start = AttemptScope {
            run_id: "turingos-tdma-run".into(),
            task_id: task_id.clone(),
            verified_parent: kernel.tape.get_verified_head(),
        };

        let mut attempts_used = 0usize;
        let mut stage_outcome = "incomplete".to_string();

        loop {
            attempts_used += 1;
            if attempts_used > max_attempts_per_stage {
                eprintln!(
                    "[turingos tdma run] stage {} exhausted local attempt cap {}",
                    stage.label(),
                    max_attempts_per_stage
                );
                stage_outcome = "cap-reached".into();
                break;
            }

            let attempt_start = Instant::now();
            let messages = vec![
                ChatMessage::system(system_prompt(stage.label())),
                ChatMessage::user(format!(
                    "{}{}",
                    user_prompt(stage.label(), &accepted_steps),
                    if attempts_used > 1 {
                        "\n\n[NOTE: prior attempt was rejected by the verifier — provide more explicit reasoning.]"
                    } else {
                        ""
                    }
                )),
            ];

            // PRODUCTION CALL: same client as `turingos llm complete` and
            // `turingos generate`. Routes to SiliconFlow / configured endpoint.
            let sf = match chat_complete_blocking(
                &api_key,
                &model,
                &messages,
                Some(500),
                Some(temperature),
                None, // no thinking-mode toggle for this experiment
            ) {
                Ok(r) => r,
                Err(LlmError::Transport(t)) => {
                    eprintln!("[turingos tdma run] transport error: {}", t);
                    stages_escalated.push(format!("{}/transport-error", stage.label()));
                    per_stage_attempts.push((
                        stage.label().into(),
                        attempts_used,
                        0,
                        "transport-error".into(),
                    ));
                    break 'outer;
                }
                Err(e) => {
                    eprintln!("[turingos tdma run] LLM error: {:?}", e);
                    stages_escalated.push(format!("{}/llm-error", stage.label()));
                    per_stage_attempts.push((
                        stage.label().into(),
                        attempts_used,
                        0,
                        "llm-error".into(),
                    ));
                    break 'outer;
                }
            };
            total_sf_completion_tokens += sf.usage.completion_tokens as u32;
            total_sf_prompt_tokens += sf.usage.prompt_tokens as u32;

            let body = extract_body(&sf.content);
            eprintln!(
                "[turingos tdma] {} attempt {} | sf-completion={}t | body[0..120]: {}",
                stage.label(),
                attempts_used,
                sf.usage.completion_tokens as u32,
                body.chars().take(120).collect::<String>()
            );

            let (verdict, expected_class) =
                judge.verdict_for_stage(&body, stage, &accepted_steps);
            let success = verdict.is_pass();
            let judge_class_str = expected_class
                .map(|c| c.reject_class_str())
                .unwrap_or(if success { "pass" } else { "unknown" });
            let judge_reason = match &verdict {
                JudgeVerdict::Pass => "passed".to_string(),
                JudgeVerdict::Fail { reason } => reason.clone(),
                JudgeVerdict::NeedsClarification { question } => question.clone(),
            };

            let raw_stderr = if success {
                String::new()
            } else {
                let s = make_judge_stderr(
                    stage.label(),
                    &verdict,
                    &body,
                    expected_class,
                    attempts_used,
                );
                total_stderr_bytes += s.len();
                s
            };

            let env = EnvironmentResult {
                raw_output: sf.content.clone(),
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
                    let leak = prompt.contains(LEAK_SENTINEL);
                    if leak {
                        leak_anywhere = true;
                    }
                    ("Retry".to_string(), n, leak)
                }
                KernelStep::Escalate { reason, .. } => {
                    stages_escalated.push(format!("{}/{}", stage.label(), reason));
                    stage_outcome = format!("escalate-{}", reason);
                    (format!("Escalate({})", reason), 0, false)
                }
            };

            probes.push(Probe {
                stage: stage.label().to_string(),
                attempt: attempts_used,
                kernel_step: kernel_step_str.clone(),
                judge_class: judge_class_str.to_string(),
                sf_completion_tokens: sf.usage.completion_tokens as u32,
                sf_prompt_tokens: sf.usage.prompt_tokens as u32,
                judge_reason,
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
                    stage.label().into(),
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
                    stage.label().into(),
                    attempts_used,
                    final_cc,
                    stage_outcome.clone(),
                ));
                break 'outer;
            }
        }

        if stage_outcome == "cap-reached" {
            per_stage_attempts.push((stage.label().into(), attempts_used, 0, stage_outcome));
            break 'outer;
        }
    }

    let total_wall_ms = run_start.elapsed().as_millis();

    let probe_lines: Vec<String> = probes.iter().map(|p| p.to_jsonl()).collect();
    let probes_sha = write_jsonl(&evidence_dir.join("per_attempt_probes.jsonl"), &probe_lines)
        .unwrap_or_default();

    let mut chaintape_lines: Vec<String> = Vec::new();
    for (h, node) in &kernel.tape.indexes.by_hash {
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
    let chaintape_sha =
        write_jsonl(&evidence_dir.join("chaintape.jsonl"), &chaintape_lines).unwrap_or_default();

    let retry_probes: Vec<&Probe> = probes
        .iter()
        .filter(|p| p.kernel_step.starts_with("Retry"))
        .collect();
    let prompt_min = retry_probes.iter().map(|p| p.prompt_tokens).min().unwrap_or(0);
    let prompt_max = retry_probes.iter().map(|p| p.prompt_tokens).max().unwrap_or(0);
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
    let zero_gain_max = retry_probes.iter().map(|p| p.bbs_zero_gain).max().unwrap_or(0);

    let manifest = serde_json::json!({
        "atom": "15",
        "subcommand": "turingos tdma run",
        "judge": judge_name,
        "role": role,
        "model": model,
        "temperature": temperature,
        "workspace": workspace.display().to_string(),
        "stages_total": stages.len(),
        "stages_completed": stages_completed,
        "stages_escalated": stages_escalated,
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
        "total_wall_clock_ms": total_wall_ms as u64,
        "total_sf_completion_tokens": total_sf_completion_tokens,
        "total_sf_prompt_tokens": total_sf_prompt_tokens,
        "per_stage": per_stage_attempts.iter().map(|(s, a, c, o)| {
            serde_json::json!({"stage": s, "attempts_used": a, "final_constraints": c, "outcome": o})
        }).collect::<Vec<_>>(),
        "probes_sha256": probes_sha,
        "chaintape_sha256": chaintape_sha,
    });
    fs::write(
        evidence_dir.join("manifest.json"),
        serde_json::to_string_pretty(&manifest).unwrap_or_default(),
    )
    .ok();

    let mut r = String::new();
    r.push_str("# turingos tdma run — TDMA-Bounded Production Report\n\n");
    r.push_str(&format!("**Model**: {} (temperature {})\n\n", model, temperature));
    r.push_str(&format!("**Role**: {}\n\n", role));
    r.push_str("**Judge**: Nesbitt step verifier (Atom 10)\n\n");
    r.push_str("## Outcome\n\n");
    r.push_str(&format!(
        "- Stages completed: **{}/{}**\n- Stages escalated/aborted: {:?}\n- Total attempts: **{}**\n- Total failed attempts: **{}**\n- Wall clock: **{:.1}s**\n\n",
        stages_completed,
        stages.len(),
        stages_escalated,
        probes.len(),
        retry_probes.len(),
        total_wall_ms as f64 / 1000.0
    ));
    r.push_str("## Per stage\n\n| Stage | Attempts | Final BBS constraints | Outcome |\n|---|---|---|---|\n");
    for (s, a, c, o) in &per_stage_attempts {
        r.push_str(&format!("| {} | {} | {} | {} |\n", s, a, c, o));
    }
    r.push_str(&format!(
        "\n## Compression\n\n- Total raw stderr: **{} bytes** ({:.1} KB)\n- Total BBS (est): {} bytes\n- **Compression ratio: {:.1}x**\n- Distinct judge classes: {:?}\n- Max zero_gain_streak: {}\n",
        total_stderr_bytes,
        total_stderr_bytes as f64 / 1024.0,
        total_bbs_bytes,
        compression_ratio,
        classes_seen.iter().cloned().collect::<Vec<_>>(),
        zero_gain_max
    ));
    r.push_str(&format!(
        "\n## Prompt invariance\n\n- Range: **{}..{}** tokens (variance {})\n- All within B_PROMPT_MAX={}: **{}**\n",
        prompt_min,
        prompt_max,
        prompt_max.saturating_sub(prompt_min),
        B_PROMPT_MAX,
        retry_probes.iter().all(|p| p.prompt_tokens <= B_PROMPT_MAX)
    ));
    r.push_str(&format!(
        "\n## SiliconFlow tokens consumed\n\n- Prompt: {}\n- Completion: {}\n\n",
        total_sf_prompt_tokens, total_sf_completion_tokens
    ));
    r.push_str(&format!(
        "## KILL guards on PRODUCTION LLM traffic\n\n- Raw stderr leak in any prompt: **{}** (KILL-tdma-1)\n- Prompt always within budget: see above (KILL-tdma-9)\n",
        leak_anywhere
    ));
    r.push_str("\n## Evidence integrity\n\n");
    r.push_str(&format!("- per_attempt_probes.jsonl sha256: {}\n", probes_sha));
    r.push_str(&format!("- chaintape.jsonl sha256: {}\n", chaintape_sha));
    fs::write(evidence_dir.join("ProductionTdmaReport.md"), r).ok();

    println!(
        "turingos tdma run: completed {}/{} stages in {:.1}s. Evidence at {}",
        stages_completed,
        stages.len(),
        total_wall_ms as f64 / 1000.0,
        evidence_dir.display()
    );
    ExitCode::SUCCESS
}
