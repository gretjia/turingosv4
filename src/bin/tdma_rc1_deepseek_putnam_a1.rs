//! TRACE_MATRIX FC1a-rtool + FC1a-judge_pi + FC3-replay:
//! Atom 13 — Real DeepSeek vs Putnam 2024 A1 (via local llm_proxy).
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

const PROBLEM_TEXT: &str = r#"Putnam 2024 A1.
Determine all positive integers n such that there exist positive integers
a, b, c with 2*a^n + 3*b^n = 4*c^n.

Canonical proof outline (8 stages):
  Stage 1: Verify n = 1 works via the witness (a,b,c) = (1, 2, 2): check 2*1 + 3*2 = 8 = 4*2.
  Stage 2: For n >= 2, WLOG assume gcd(a,b,c) = 1 (otherwise divide through).
  Stage 3: Case n = 2 - derive a^2 + c^2 ≡ 0 (mod 3) and use that 0, 1 are the only squares mod 3.
  Stage 4: Show b is also a multiple of 3, contradicting gcd = 1.
  Stage 5: Case n >= 3 - from 3*b^n = 4*c^n - 2*a^n, derive that b is even.
  Stage 6: Rewriting then forces a to be even.
  Stage 7: One more rewriting forces c to be even.
  Stage 8: All three even contradicts gcd = 1; therefore n = 1 is the unique solution."#;

fn system_prompt(stage_label: &str) -> String {
    format!(
        r#"You are a mathematics worker proving Putnam 2024 A1 step-by-step.
Output EXACTLY ONE next step of the proof.

Your output MUST start with this JSON object on the FIRST line:
{{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"<STAGE>","action":"PROPOSE","failed_predicate":null,"reject_class":null,"next_action_hint":null,"evidence_hash":null}}

Replace <STAGE> with the current stage label (e.g. "{stage_label}").
After the JSON, write on a new line exactly:
---BODY---
Then write your single step in 2-5 sentences. Be RIGOROUS — explicit
modular arithmetic, explicit "WLOG gcd(a,b,c)=1", explicit
"b is even", etc. Hand-waving will be REJECTED by the verifier.

Current stage: {stage_label}"#,
        stage_label = stage_label
    )
}

fn user_prompt(stage_label: &str, accepted_steps: &[String]) -> String {
    let mut s = String::new();
    s.push_str(&format!("Problem:\n{}\n\n", PROBLEM_TEXT));
    s.push_str(&format!("Current stage to prove: {}\n\n", stage_label));
    if accepted_steps.is_empty() {
        s.push_str("No prior steps yet. Write Stage 1: verify the n=1 witness.");
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
        "[atom13-shim] proxy={} model={} max_attempts_per_stage={} temperature={}",
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
        run_id: "atom13-deepseek-putnam-a1".to_string(),
        model_label: model.clone(),
        problem_label: "Putnam 2024 A1 (real DeepSeek)".into(),
        leak_sentinel: "PUTNAM_A1_STDERR_LEAK_CANARY_F7K3X".into(),
        system_prompt_for_stage: Box::new(system_prompt),
        user_prompt_for_stage: Box::new(user_prompt),
        problem_text: PROBLEM_TEXT.into(),
        evidence_dir,
        temperature,
        max_tokens: 600,
        max_attempts_per_stage,
    };

    let mut judge = AnyJudge::putnam_a1();

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
            max_tokens: Some(600),
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
                "[atom13-shim] {} / {} stages completed in {:.1}s",
                summary.stages_completed,
                summary.stages_total,
                summary.total_wall_clock_ms as f64 / 1000.0
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("[atom13-shim] run_proof failed: {}", e);
            ExitCode::from(3)
        }
    }
}
