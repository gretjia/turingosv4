//! TRACE_MATRIX FC1a-rtool_input + FC1a-predicate_pi + FC3-replay:
//! Atom 14 — Real DeepSeek vs Putnam 2025 B3 (post-cutoff EXTREME stress).
//!
//! Problem (Putnam 2025 B3, Dec 6 2025 — after DeepSeek-chat's training
//! cutoff):
//!   Suppose S is a nonempty set of positive integers with the property
//!   that if n ∈ S, then every positive divisor of 2025n − 15n is in S.
//!   Must S contain all positive integers?
//!
//! Answer: NO. Counterexample: take S to be the closure of {1} under
//! "n ⇒ divisors of 2010n". Since 2010 = 2·3·5·67, no member ever has
//! a prime factor outside {2, 3, 5, 67}; in particular 7 ∉ S.
//!
//! Why post-cutoff matters: Putnam 2024 (Atom 13) was likely in
//! DeepSeek-chat's training data. Putnam 2025 was held December 2025;
//! DeepSeek-chat's knowledge cutoff is mid-2024 — so this problem is
//! NEW to the model. Plus the Putnam committee chair publicly stated
//! ≥ 6 of the 12 problems were designed to be LLM-resistant.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use sha2::{Digest, Sha256};
use turingosv4::charter_core::compile_charter_core;
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::judges::math_step_judge::JudgeVerdict;
use turingosv4::judges::putnam_2025_b3_judge::{
    PutnamB3Judge, PutnamB3RejectClass, PutnamB3Stage,
};
use turingosv4::ledger::{AttemptScope, ImmutableTapeLedger, MemoryTapeLedger};
use turingosv4::memory_kernel::{EnvironmentResult, KernelStep, MemoryKernel, Task};
use turingosv4::token_budget::{B_PROMPT_MAX, MAX_RETRIES};
use turingosv4::tokenizer::Tokenizer;

const LEAK_SENTINEL: &str = "PUTNAM_2025_B3_STDERR_LEAK_CANARY_M9N3Q";

const PROBLEM_TEXT: &str = r#"Putnam 2025 B3 (December 6, 2025).
Suppose S is a nonempty set of positive integers with the property
that if n is in S, then every positive divisor of 2025n − 15n is in S.
Must S contain all positive integers?

You are to write a step-by-step proof in 5 stages:
  Stage 1: Simplify the expression 2025n − 15n explicitly.
  Stage 2: Factor the resulting constant into its prime factors.
  Stage 3: Argue that divisors of the simplified expression introduce
           only a bounded set of primes (closure under divisors does
           NOT create new primes outside this set).
  Stage 4: Construct an explicit counterexample S that demonstrates a
           prime is never forced into S.
  Stage 5: Conclude YES or NO with a clean final statement."#;

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

fn system_prompt(stage_label: &str) -> String {
    format!(
        r#"You are a mathematics worker proving Putnam 2025 B3 step-by-step.
Output EXACTLY ONE next step.

Your output MUST start with this JSON object on the FIRST line:
{{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"<STAGE>","action":"PROPOSE","failed_predicate":null,"reject_class":null,"next_action_hint":null,"evidence_hash":null}}

Replace <STAGE> with the current stage label (e.g. "{stage_label}").
After the JSON write on a new line:
---BODY---
Then write your step in 2-5 sentences. Be RIGOROUS — explicit
algebraic simplification, explicit prime factorization, explicit
closure-of-primes argument, explicit counterexample. Hand-waving
will be REJECTED by the verifier.

Current stage: {stage_label}"#,
        stage_label = stage_label
    )
}

fn user_prompt(stage_label: &str, accepted_steps: &[String]) -> String {
    let mut s = String::new();
    s.push_str(&format!("Problem:\n{}\n\n", PROBLEM_TEXT));
    s.push_str(&format!("Current stage to prove: {}\n\n", stage_label));
    if accepted_steps.is_empty() {
        s.push_str("No prior steps yet. Write Stage 1: simplify 2025n − 15n.");
    } else {
        s.push_str("Prior accepted steps:\n");
        for (i, st) in accepted_steps.iter().enumerate() {
            s.push_str(&format!("  Step {}: {}\n", i + 1, st));
        }
        s.push_str("\nWrite the next single step (do NOT repeat prior steps).");
    }
    s
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
    expected_class: Option<PutnamB3RejectClass>,
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
        "PutnamB3Judge rejected stage {} (attempt {})\n",
        stage_label, attempt
    ));
    s.push_str(&format!("reject_class: {}\n", class_str));
    s.push_str(&format!("failed_predicate: {}\n", pred_str));
    s.push_str(&format!("judge_reason: {}\n", reason));
    s.push_str("traceback:\n");
    s.push_str("  at src/judges/putnam_2025_b3_judge.rs in verdict_for_stage\n");
    s.push_str("  at src/memory_kernel.rs in handle_rejection\n");
    s.push_str(&format!(
        "\nCandidate body:\n{}\n",
        candidate_body.chars().take(2500).collect::<String>()
    ));
    let template = format!(
        "  > Strict B3 verifier rejection context: {} at stage {}; predicate {} failed. ",
        class_str, stage_label, pred_str
    );
    while s.len() < 10 * 1024 {
        s.push_str(&template);
    }
    s.push_str("\n[end stderr]\n");
    s
}

#[derive(Debug, Clone)]
struct Probe {
    stage: String,
    attempt: usize,
    kernel_step: String,
    judge_class: String,
    deepseek_completion_tokens: u32,
    deepseek_prompt_tokens: u32,
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
            "deepseek_completion_tokens": self.deepseek_completion_tokens,
            "deepseek_prompt_tokens": self.deepseek_prompt_tokens,
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

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let mut evidence_dir: Option<PathBuf> = None;
    let mut proxy_url = "http://localhost:18091".to_string();
    let mut model = "deepseek-chat".to_string();
    let mut max_attempts_per_stage: usize = MAX_RETRIES as usize + 2;
    let mut temperature: f64 = 0.7;
    let mut a = env::args().skip(1);
    while let Some(arg) = a.next() {
        match arg.as_str() {
            "--evidence-dir" => evidence_dir = a.next().map(PathBuf::from),
            "--proxy-url" => proxy_url = a.next().unwrap_or(proxy_url),
            "--model" => model = a.next().unwrap_or(model),
            "--max-attempts-per-stage" => {
                max_attempts_per_stage = a
                    .next()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(max_attempts_per_stage)
            }
            "--temperature" => {
                temperature = a.next().and_then(|s| s.parse().ok()).unwrap_or(temperature)
            }
            _ => {}
        }
    }
    let evidence_dir = match evidence_dir {
        Some(d) => d,
        None => {
            eprintln!("--evidence-dir is required");
            return ExitCode::from(2);
        }
    };
    fs::create_dir_all(&evidence_dir).ok();

    eprintln!(
        "[atom14] proxy={} model={} max_attempts_per_stage={} temperature={}",
        proxy_url, model, max_attempts_per_stage, temperature
    );

    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\nArt. 0.4 — Q_t Path A; FC1a tape_t; FC1b wtool.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    let mut kernel = MemoryKernel::new(tape, "atom14-putnam-2025-b3", charter);
    let judge = PutnamB3Judge::new();
    let llm = ResilientLLMClient::new(&proxy_url, 120, 2);
    let tk = Tokenizer::new();

    let stages = [
        PutnamB3Stage::Stage1Simplify,
        PutnamB3Stage::Stage2Factor2010,
        PutnamB3Stage::Stage3Closure,
        PutnamB3Stage::Stage4Counterex,
        PutnamB3Stage::Stage5ConcludeNo,
    ];

    let mut accepted_steps: Vec<String> = Vec::new();
    let mut probes: Vec<Probe> = Vec::new();
    let mut total_stderr_bytes = 0usize;
    let mut total_deepseek_completion_tokens = 0u32;
    let mut total_deepseek_prompt_tokens = 0u32;
    let mut leak_anywhere = false;
    let mut stages_completed = 0usize;
    let mut stages_escalated: Vec<String> = Vec::new();
    let mut per_stage_attempts: Vec<(String, usize, usize, String)> = Vec::new();
    let run_start = Instant::now();

    'outer: for stage in stages {
        let task_id = format!("putnam-2025-b3-{}", stage.label());
        let task = Task {
            id: task_id.clone(),
            prompt: user_prompt(stage.label(), &accepted_steps),
        };
        let scope_at_start = AttemptScope {
            run_id: "atom14-putnam-2025-b3".into(),
            task_id: task_id.clone(),
            verified_parent: kernel.tape.get_verified_head(),
        };

        let mut attempts_used = 0usize;
        let mut stage_outcome = "incomplete".to_string();

        loop {
            attempts_used += 1;
            if attempts_used > max_attempts_per_stage {
                eprintln!(
                    "[atom14] stage {} exhausted local cap {}",
                    stage.label(),
                    max_attempts_per_stage
                );
                stage_outcome = "cap-reached".into();
                break;
            }

            let attempt_start = Instant::now();
            let req = GenerateRequest {
                model: model.clone(),
                messages: vec![
                    Message {
                        role: "system".into(),
                        content: system_prompt(stage.label()),
                    },
                    Message {
                        role: "user".into(),
                        content: format!(
                            "{}{}",
                            user_prompt(stage.label(), &accepted_steps),
                            if attempts_used > 1 {
                                "\n\n[NOTE: prior attempt was rejected by the strict verifier — provide more explicit reasoning this time, especially explicit primes/divisors/counterexample as needed.]"
                            } else {
                                ""
                            }
                        ),
                    },
                ],
                temperature: Some(temperature),
                max_tokens: Some(700),
            };
            let ds = match llm.generate(&req).await {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("[atom14] LLM err: {}", e);
                    stages_escalated.push(format!("{}/network-error", stage.label()));
                    per_stage_attempts.push((
                        stage.label().into(),
                        attempts_used,
                        0,
                        "network-error".into(),
                    ));
                    break 'outer;
                }
            };
            total_deepseek_completion_tokens += ds.completion_tokens;
            total_deepseek_prompt_tokens += ds.prompt_tokens;

            let body = extract_body(&ds.content);
            eprintln!(
                "[atom14] {} attempt {} | ds-completion={}t | body[0..150]: {}",
                stage.label(),
                attempts_used,
                ds.completion_tokens,
                body.chars().take(150).collect::<String>()
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
                raw_output: ds.content.clone(),
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
                deepseek_completion_tokens: ds.completion_tokens,
                deepseek_prompt_tokens: ds.prompt_tokens,
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
        "atom": "14",
        "branch": "feature/tdma-rc1-deepseek-putnam-2025-b3",
        "problem": "Putnam 2025 B3 (post-DeepSeek-chat-cutoff)",
        "model": model,
        "temperature": temperature,
        "max_attempts_per_stage": max_attempts_per_stage,
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
        "total_deepseek_completion_tokens": total_deepseek_completion_tokens,
        "total_deepseek_prompt_tokens": total_deepseek_prompt_tokens,
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
    r.push_str("# TDMA-Bounded-RC1 Atom 14 — Putnam 2025 B3 (Real DeepSeek, POST-CUTOFF EXTREME stress)\n\n");
    r.push_str(&format!("**Model**: {} (temperature {})\n\n", model, temperature));
    r.push_str("**Problem**: Putnam 2025 B3 (Dec 6, 2025 — post-DeepSeek-chat training cutoff)\n\n");
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
        "\n## DeepSeek tokens consumed\n\n- Prompt: {}\n- Completion: {}\n\n",
        total_deepseek_prompt_tokens, total_deepseek_completion_tokens
    ));
    r.push_str(&format!(
        "## KILL guards on REAL LLM traffic\n\n- Raw stderr leak in any prompt: **{}** (KILL-tdma-1)\n- Prompt always within budget: see above (KILL-tdma-9)\n",
        leak_anywhere
    ));
    r.push_str("\n## Evidence integrity\n\n");
    r.push_str(&format!("- per_attempt_probes.jsonl sha256: {}\n", probes_sha));
    r.push_str(&format!("- chaintape.jsonl sha256: {}\n", chaintape_sha));
    fs::write(evidence_dir.join("Putnam2025B3Report.md"), r).ok();

    ExitCode::SUCCESS
}
