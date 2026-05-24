//! TRACE_MATRIX FC1a-rtool + FC1a-judge_pi + FC3-replay:
//! Atom 14 — Real DeepSeek vs Putnam 2025 B3 (post-cutoff, via local llm_proxy).
//!
//! Atom 18 thin-shim refactor: all the kernel-driving / probe-capture /
//! evidence-writing logic now lives in `turingosv4::tdma_runner`.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::tdma_runner::{run_proof, AnyJudge, LlmResponse, RunConfig};

const PROBLEM_TEXT: &str = r#"Putnam 2025 B3 (December 6, 2025).
Suppose S is a nonempty set of positive integers with the property
that if n is in S, then every positive divisor of 2025n − 15n is in S.
Must S contain all positive integers?

You are to write a step-by-step proof in 5 stages:
  Stage 1: Simplify the expression 2025n − 15n explicitly.
  Stage 2: Factor the resulting constant into its prime factors.
  Stage 3: Argue that divisors of the simplified expression introduce
           only a bounded set of primes.
  Stage 4: Construct an explicit counterexample S that demonstrates a
           prime is never forced into S.
  Stage 5: Conclude YES or NO with a clean final statement."#;

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
closure-of-primes argument, explicit counterexample.

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

fn main() -> ExitCode {
    let mut evidence_dir: Option<PathBuf> = None;
    let mut proxy_url = "http://localhost:18091".to_string();
    let mut model = "deepseek-chat".to_string();
    let mut max_attempts_per_stage: usize = 6;
    let mut temperature: f32 = 0.7;
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

    eprintln!(
        "[atom14-shim] proxy={} model={} max_attempts_per_stage={} temperature={}",
        proxy_url, model, max_attempts_per_stage, temperature
    );

    let llm = ResilientLLMClient::new(&proxy_url, 120, 2);
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("cannot build tokio runtime: {}", e);
            return ExitCode::from(2);
        }
    };

    let cfg = RunConfig {
        run_id: "atom14-deepseek-putnam-2025-b3".to_string(),
        model_label: model.clone(),
        problem_label: "Putnam 2025 B3 (post-cutoff, real DeepSeek)".into(),
        leak_sentinel: "PUTNAM_2025_B3_STDERR_LEAK_CANARY_M9N3Q".into(),
        system_prompt_for_stage: Box::new(system_prompt),
        user_prompt_for_stage: Box::new(user_prompt),
        problem_text: PROBLEM_TEXT.into(),
        evidence_dir,
        temperature,
        max_tokens: 700,
        max_attempts_per_stage,
    };

    let mut judge = AnyJudge::putnam_b3();

    let llm_call = |sys: &str, user: &str| -> Result<LlmResponse, String> {
        let req = GenerateRequest {
            model: model.clone(),
            messages: vec![
                Message {
                    role: "system".into(),
                    content: sys.to_string(),
                },
                Message {
                    role: "user".into(),
                    content: user.to_string(),
                },
            ],
            temperature: Some(temperature as f64),
            max_tokens: Some(700),
        };
        let resp = rt
            .block_on(llm.generate(&req))
            .map_err(|e| format!("llm: {}", e))?;
        Ok(LlmResponse {
            content: resp.content,
            completion_tokens: resp.completion_tokens,
            prompt_tokens: resp.prompt_tokens,
        })
    };

    match run_proof(cfg, &mut judge, llm_call) {
        Ok(summary) => {
            eprintln!(
                "[atom14-shim] {} / {} stages completed in {:.1}s",
                summary.stages_completed,
                summary.stages_total,
                summary.total_wall_clock_ms as f64 / 1000.0
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("[atom14-shim] run_proof failed: {}", e);
            ExitCode::from(3)
        }
    }
}
