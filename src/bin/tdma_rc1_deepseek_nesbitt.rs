//! TRACE_MATRIX FC1a-rtool + FC1a-predicate_pi + FC3-replay:
//! Atom 12 — Real DeepSeek vs Nesbitt's inequality (via local llm_proxy).
//!
//! Atom 18 thin-shim refactor: all the kernel-driving / probe-capture /
//! evidence-writing logic now lives in `turingosv4::tdma_runner`. This
//! binary contributes only:
//!   * the local-proxy LLM-call closure (ResilientLLMClient wrapped in a
//!     current-thread tokio block-on),
//!   * the per-stage prompt builders for Nesbitt,
//!   * the run configuration (run_id, evidence-dir, problem text).
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::tdma_runner::{run_proof, AnyJudge, LlmResponse, RunConfig};

const PROBLEM_TEXT: &str = r#"Prove Nesbitt's inequality for positive reals:
    a/(b+c) + b/(a+c) + c/(a+b) >= 3/2

Use the AM-GM substitution proof: let x=b+c, y=a+c, z=a+b, then
rewrite, expand into six fractions, group into three pairs, apply
AM-GM to each pair, sum, subtract, and conclude with the equality
case at a=b=c."#;

fn system_prompt(stage_label: &str) -> String {
    format!(
        r#"You are a mathematics worker proving Nesbitt's inequality step-by-step.
You will be given the prior accepted steps and the current stage label.
Output EXACTLY ONE next step of the proof.

Your output MUST start with this JSON object on the first line (compact, single line):
{{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"<STAGE>","action":"PROPOSE","failed_predicate":null,"reject_class":null,"next_action_hint":null,"evidence_hash":null}}

Replace <STAGE> with the current stage label (e.g. "{stage_label}").
After the JSON line, write on a new line exactly:
---BODY---
Then write your single proof step in 1-3 sentences. Be concrete and
mathematically precise. Use ASCII math freely. Do not reproduce the entire
proof; only one step for the current stage.

Current stage: {stage_label}"#,
        stage_label = stage_label
    )
}

fn user_prompt(stage_label: &str, accepted_steps: &[String]) -> String {
    let mut s = String::new();
    s.push_str(&format!("Problem:\n{}\n\n", PROBLEM_TEXT));
    s.push_str(&format!("Current stage: {}\n\n", stage_label));
    if accepted_steps.is_empty() {
        s.push_str("No prior steps yet. Write the FIRST step (substitution).");
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
        "[atom12-shim] proxy={} model={} max_attempts_per_stage={} temperature={}",
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
        run_id: "atom12-deepseek-nesbitt".to_string(),
        model_label: model.clone(),
        problem_label: "Nesbitt's inequality (real DeepSeek)".into(),
        leak_sentinel: "DEEPSEEK_RAW_STDERR_SENTINEL_K7L2M".into(),
        system_prompt_for_stage: Box::new(system_prompt),
        user_prompt_for_stage: Box::new(user_prompt),
        problem_text: PROBLEM_TEXT.into(),
        evidence_dir,
        temperature,
        max_tokens: 500,
        max_attempts_per_stage,
    };

    let mut judge = AnyJudge::nesbitt();

    // Bridge the local tokio runtime to the runner's sync closure.
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
            max_tokens: Some(500),
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
                "[atom12-shim] {} / {} stages completed in {:.1}s",
                summary.stages_completed,
                summary.stages_total,
                summary.total_wall_clock_ms as f64 / 1000.0
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("[atom12-shim] run_proof failed: {}", e);
            ExitCode::from(3)
        }
    }
}
