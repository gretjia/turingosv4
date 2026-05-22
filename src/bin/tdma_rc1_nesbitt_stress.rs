//! TRACE_MATRIX FC1a-rtool_input + FC3-replay:
//! Atom 10 — Nesbitt's inequality real-world stress test.
//!
//! Drives the TDMA-Bounded kernel through the canonical 8-step AM-GM proof of
//! Nesbitt's inequality (a/(b+c) + b/(a+c) + c/(a+b) ≥ 3/2 for a,b,c>0) using
//! the IneqMath-style multi-category judge in src/judges/nesbitt_step_judge.rs.
//!
//! The worker simulator emits a realistic mix of failed and correct attempts
//! per stage, drawn from IneqMath's documented LLM failure taxonomy
//! (arxiv.org/abs/2506.07927): direction-reversal, bad-substitution,
//! algebra-error, logical-gap, missing-equality-case, off-stage. The
//! kernel's distiller + BBS must:
//!
//!   1. Capture each failure with a distinct signature when wrong-patterns
//!      vary, accumulate them in BBS up to B_D=400 tokens.
//!   2. Trigger zero_gain escalation when the same wrong pattern repeats.
//!   3. Hold prompt size ≤ B_PROMPT_MAX=5800 tokens across the entire run.
//!   4. Never leak raw_stderr (which carries 10-20 KB of fake-LLM rejection
//!      text per failure) into any prompt.
//!
//! The proof's final stage either succeeds (kernel returns Proceed with
//! verified_head advanced through all 8 stages) or escalates (some stage
//! exhausts MAX_RETRIES). Both outcomes produce a complete tape — the goal
//! is NOT to "win" the proof but to exercise the compression machinery on
//! a realistic, multi-failure-per-step long-chain run.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md
//! Real-world basis: IneqMath benchmark (arxiv.org/abs/2506.07927) — step-level
//! LLM accuracy on this problem class is ≤ 10% per the paper's eval.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use sha2::{Digest, Sha256};
use turingosv4::charter_core::compile_charter_core;
use turingosv4::judges::math_step_judge::JudgeVerdict;
use turingosv4::judges::nesbitt_step_judge::{
    NesbittRejectClass, NesbittStage, NesbittStepJudge,
};
use turingosv4::ledger::{
    AttemptScope, ImmutableTapeLedger, MemoryTapeLedger, NodeKind,
};
use turingosv4::memory_kernel::{EnvironmentResult, KernelStep, MemoryKernel, Task};
use turingosv4::token_budget::{B_D, B_PROMPT_MAX, MAX_RETRIES};
use turingosv4::tokenizer::Tokenizer;

const RAW_STDERR_LEAK_SENTINEL: &str = "NESBITT_RAW_STDERR_LEAK_CANARY_X3Y9Z";

// ── helpers ──────────────────────────────────────────────────────

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

/// One simulated worker attempt: a step text plus optionally the wrong-pattern
/// label (for `bad` attempts; correct attempts have `None`).
fn attempt_text(stage: NesbittStage, variant: usize) -> (String, Option<NesbittRejectClass>) {
    // For each stage we list 3-4 typical wrong attempts (matching IneqMath
    // failure modes) plus one canonical correct attempt at the end.
    let table: HashMap<NesbittStage, Vec<(&str, Option<NesbittRejectClass>)>> = HashMap::from([
        (
            NesbittStage::Step1Substitute,
            vec![
                (
                    "Substitute the answer is 3/2 directly. No setup needed.",
                    Some(NesbittRejectClass::LogicalGap),
                ),
                (
                    "Let x = a+b and let x = b+c. Substitute both.",
                    Some(NesbittRejectClass::BadSubstitution),
                ),
                (
                    "Apply Cauchy-Schwarz on the cyclic sum.",
                    Some(NesbittRejectClass::OffStage),
                ),
                (
                    "Let x = b+c, y = a+c, z = a+b. Substitute these into the cyclic sum.",
                    None,
                ),
            ],
        ),
        (
            NesbittStage::Step2Rewrite,
            vec![
                (
                    "Rewrite each term as (x+z+y)/(2y). The numerator captures a+b+c.",
                    Some(NesbittRejectClass::AlgebraError),
                ),
                (
                    "Rewrite each term as (x+y+z)/(2y) — combining all variables in numerator.",
                    Some(NesbittRejectClass::AlgebraError), // intentional same-class repeat → zero_gain++
                ),
                (
                    "Consider the polynomial (x-y)(y-z)(z-x) and its roots.",
                    Some(NesbittRejectClass::OffStage),
                ),
                (
                    "Rewrite a/(b+c) = (x+z-y)/(2y) in terms of x, y, z; do similar for the other two cyclic terms.",
                    None,
                ),
            ],
        ),
        (
            NesbittStage::Step3Expand,
            vec![
                (
                    "Expand to a single fraction with denominator xyz.",
                    Some(NesbittRejectClass::AlgebraError),
                ),
                (
                    "Expand to one big fraction with denominator xyz — clearing all denominators.",
                    Some(NesbittRejectClass::AlgebraError), // intentional same-class repeat
                ),
                (
                    "By induction on n, the result follows.",
                    Some(NesbittRejectClass::OffStage),
                ),
                (
                    "Expand into six separate fractions: (x+z)/y + (y+z)/x + (x+y)/z - 3 (after multiplying by 2).",
                    None,
                ),
            ],
        ),
        (
            NesbittStage::Step4Group,
            vec![
                (
                    "Apply the rearrangement inequality directly.",
                    Some(NesbittRejectClass::OffStage),
                ),
                (
                    "Apply Schur's inequality to the six fractions.",
                    Some(NesbittRejectClass::OffStage), // same-class repeat
                ),
                (
                    "Group the six fractions into three pairs: (x/y + y/x) + (y/z + z/y) + (x/z + z/x).",
                    None,
                ),
            ],
        ),
        (
            NesbittStage::Step5ApplyAmGm,
            vec![
                (
                    "By AM-GM applied with arithmetic mean ≤ geometric mean, each pair x/y + y/x ≤ 2.",
                    Some(NesbittRejectClass::DirectionReversal),
                ),
                (
                    "By AM-GM, arithmetic mean ≤ geometric mean, so x/y + y/x is less than 2.",
                    Some(NesbittRejectClass::DirectionReversal), // same-class repeat
                ),
                (
                    "Apply Power Mean inequality instead of AM-GM.",
                    Some(NesbittRejectClass::OffStage),
                ),
                (
                    "Use Jensen's inequality on a convex function.",
                    Some(NesbittRejectClass::OffStage), // same-class repeat
                ),
                (
                    "By AM-GM, each pair x/y + y/x ≥ 2, since the arithmetic mean ≥ geometric mean = sqrt(xy/xy) = 1.",
                    None,
                ),
            ],
        ),
        (
            NesbittStage::Step6Sum,
            vec![
                (
                    "Sum the three pairs: total ≥ 4 (since each is ≥ 2 but there are only 2 distinct values).",
                    Some(NesbittRejectClass::AlgebraError),
                ),
                (
                    "Sum the three pairs: total ≥ 8 by Cauchy-Schwarz.",
                    Some(NesbittRejectClass::AlgebraError),
                ),
                (
                    "Sum the three pairs: total = 3 · 2 = 6, i.e. ≥ 6.",
                    None,
                ),
            ],
        ),
        (
            NesbittStage::Step7Subtract,
            vec![
                (
                    "Multiply both sides by 2.",
                    Some(NesbittRejectClass::OffStage),
                ),
                (
                    "Subtract 3 from both sides: LHS_unscaled = sum - 3 ≥ 6 - 3 = 3.",
                    None,
                ),
            ],
        ),
        (
            NesbittStage::Step8ConcludeAndEq,
            vec![
                (
                    "Divide by 2 to conclude: a/(b+c) + b/(a+c) + c/(a+b) ≥ 3/2.",
                    Some(NesbittRejectClass::MissingEqualityCase),
                ),
                (
                    "By the AM-HM inequality, conclude ≥ 3/2.",
                    Some(NesbittRejectClass::MissingEqualityCase),
                ),
                (
                    "Divide by 2 to conclude a/(b+c) + b/(a+c) + c/(a+b) ≥ 3/2. Equality holds iff x = y = z, equivalently a = b = c.",
                    None,
                ),
            ],
        ),
    ]);
    let variants = table.get(&stage).expect("stage table missing");
    let idx = variant.min(variants.len() - 1);
    let (text, cls) = variants[idx];
    (text.to_string(), cls)
}

/// Build a synthetic 10-15 KB raw_stderr for a failed attempt, embedding the
/// leak sentinel + stage label + reject class.
fn make_failure_stderr(
    stage: NesbittStage,
    rc: Option<NesbittRejectClass>,
    attempt_idx: usize,
) -> String {
    let class_str = rc.map(|c| c.reject_class_str()).unwrap_or("unknown");
    let pred_str = rc.map(|c| c.failed_predicate_str()).unwrap_or("none");
    let mut s = String::new();
    s.push_str(RAW_STDERR_LEAK_SENTINEL);
    s.push('\n');
    s.push_str(&format!(
        "IneqMath-style judge rejected stage {} (attempt {})\n",
        stage.label(),
        attempt_idx,
    ));
    s.push_str(&format!("reject_class: {}\n", class_str));
    s.push_str(&format!("failed_predicate: {}\n", pred_str));
    s.push_str("traceback:\n");
    s.push_str(&format!(
        "  at src/judges/nesbitt_step_judge.rs:42 in verdict_for_stage\n"
    ));
    s.push_str(&format!(
        "  at src/memory_kernel.rs:178 in handle_rejection\n"
    ));
    s.push_str(&format!(
        "  caused by: structural mismatch at stage {} ({})\n",
        stage.label(),
        class_str
    ));
    // Realistic stderr would include the LLM's failed attempt text + judge's
    // critique. We simulate that with 12 KB of plausible-looking content.
    s.push_str("\nFull rejection context:\n");
    let target_bytes = 12 * 1024;
    let template = format!(
        "  > Attempt body line: candidate step text included the {} pattern, \
         which fails the {} predicate. Specifically, the judge looked for the \
         canonical {} form but saw a deviation. ",
        class_str,
        pred_str,
        stage.label(),
    );
    while s.len() < target_bytes {
        s.push_str(&template);
    }
    s.push_str("\n[end of stderr]\n");
    s
}

fn header_text(status: &str, task_id: &str, reject_class: &str, failed_pred: &str) -> String {
    format!(
        r#"{{"schema_version":"tdma-state-update/v1","status":"{}","task_id":"{}","action":"{}","failed_predicate":"{}","reject_class":"{}"}}"#,
        status,
        task_id,
        if status == "Proceed" { "PROCEED" } else { "RETRY" },
        failed_pred,
        reject_class,
    )
}

// ── per-attempt probe ───────────────────────────────────────────

#[derive(Debug, Clone)]
struct AttemptProbe {
    stage: String,
    task_id: String,
    attempt_idx: usize,
    kernel_step: String, // "Proceed" | "Retry" | "Escalate(reason)"
    reject_class: String,
    prompt_tokens: usize,
    bbs_constraint_count: usize,
    bbs_token_count: usize,
    bbs_zero_gain_streak: u32,
    raw_stderr_bytes: usize,
    leak_in_prompt: bool,
}

impl AttemptProbe {
    fn to_jsonl(&self) -> String {
        format!(
            r#"{{"stage":{:?},"task":{:?},"attempt":{},"kernel_step":{:?},"reject_class":{:?},"prompt_tokens":{},"bbs_constraints":{},"bbs_tokens":{},"zero_gain":{},"stderr_bytes":{},"leak":{}}}"#,
            self.stage,
            self.task_id,
            self.attempt_idx,
            self.kernel_step,
            self.reject_class,
            self.prompt_tokens,
            self.bbs_constraint_count,
            self.bbs_token_count,
            self.bbs_zero_gain_streak,
            self.raw_stderr_bytes,
            self.leak_in_prompt,
        )
    }
}

// ── main run loop ───────────────────────────────────────────────

fn main() -> ExitCode {
    let mut evidence_dir: Option<PathBuf> = None;
    let mut args = env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--evidence-dir" => {
                if let Some(v) = args.next() {
                    evidence_dir = Some(PathBuf::from(v));
                }
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
    if let Err(e) = fs::create_dir_all(&evidence_dir) {
        eprintln!("create_dir_all failed: {}", e);
        return ExitCode::from(2);
    }

    // ── Boot kernel ──
    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\nArt. 0.4 — Q_t Path A; FC1a tape_t; FC1b wtool.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    let mut kernel = MemoryKernel::new(tape, "atom10-nesbitt", charter);
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
    let mut probes: Vec<AttemptProbe> = Vec::new();
    let mut total_stderr_bytes = 0usize;
    let mut leak_anywhere = false;
    let mut stages_completed = 0usize;
    let mut stages_escalated: Vec<String> = Vec::new();
    let mut per_stage_attempts: Vec<(String, usize, usize)> = Vec::new(); // (stage, attempts_used, final_constraints)

    'stages: for (stage_idx, stage) in stages.iter().enumerate() {
        // Worker simulator: try variants in order — wrongs first, then correct.
        // For some stages we also inject a same-signature repeat to test
        // zero_gain (Stage 5 — direction-reversal — has 2 same-class entries).
        let task_id = format!("nesbitt-{}", stage.label());
        let task = Task {
            id: task_id.clone(),
            prompt: format!(
                "Prove the next step of Nesbitt's inequality.\n\
                 Accepted so far ({}):\n{}\n",
                accepted_steps.len(),
                accepted_steps.join("\n")
            ),
        };

        let scope_at_start = AttemptScope {
            run_id: "atom10-nesbitt".into(),
            task_id: task_id.clone(),
            verified_parent: kernel.tape.get_verified_head(),
        };

        let mut variant_idx = 0usize;
        let mut stage_succeeded = false;
        let mut attempts_used = 0usize;

        loop {
            attempts_used += 1;
            let (step_text, expected_reject) = attempt_text(*stage, variant_idx);
            let verdict_judge = judge.verdict_for_stage(&step_text, *stage, &accepted_steps);
            let success = verdict_judge.0.is_pass();

            let (reject_class_str, failed_pred_str) = match expected_reject {
                Some(c) => (c.reject_class_str(), c.failed_predicate_str()),
                None => (
                    if success { "none" } else { "unrecognized" },
                    if success { "none" } else { "unknown" },
                ),
            };

            let raw_stderr = if success {
                String::new()
            } else {
                let s = make_failure_stderr(*stage, expected_reject, attempts_used);
                total_stderr_bytes += s.len();
                s
            };

            let env = EnvironmentResult {
                raw_output: format!(
                    "{}\n---BODY---\n{}",
                    header_text(
                        if success { "Proceed" } else { "Retry" },
                        &task_id,
                        reject_class_str,
                        failed_pred_str,
                    ),
                    step_text
                ),
                raw_stderr: raw_stderr.clone(),
                success,
            };

            let step = kernel.step_forward(&task, env);
            let (kernel_step_str, prompt_tokens, leak) = match step {
                KernelStep::Proceed { .. } => {
                    accepted_steps.push(step_text.clone());
                    judge.advance();
                    stage_succeeded = true;
                    ("Proceed".to_string(), 0, false)
                }
                KernelStep::Retry { prompt, .. } => {
                    let n = tk.count_text(&prompt);
                    let leak = prompt.contains(RAW_STDERR_LEAK_SENTINEL);
                    if leak {
                        leak_anywhere = true;
                    }
                    ("Retry".to_string(), n, leak)
                }
                KernelStep::Escalate { reason, .. } => {
                    stages_escalated.push(stage.label().to_string());
                    (format!("Escalate({})", reason), 0, false)
                }
            };

            let bbs = kernel
                .tape
                .derive_latest_belief_state_from_tape(&scope_at_start);
            let (cc, ct, zgs) = match &bbs {
                Some(b) => (b.constraints.len(), tk.count_json(b), b.zero_gain_streak),
                None => (0, 0, 0),
            };

            probes.push(AttemptProbe {
                stage: stage.label().to_string(),
                task_id: task_id.clone(),
                attempt_idx: attempts_used,
                kernel_step: kernel_step_str.clone(),
                reject_class: reject_class_str.to_string(),
                prompt_tokens,
                bbs_constraint_count: cc,
                bbs_token_count: ct,
                bbs_zero_gain_streak: zgs,
                raw_stderr_bytes: raw_stderr.len(),
                leak_in_prompt: leak,
            });

            if stage_succeeded {
                let final_cc = kernel
                    .tape
                    .derive_latest_belief_state_from_tape(&scope_at_start)
                    .map(|b| b.constraints.len())
                    .unwrap_or(0);
                per_stage_attempts.push((stage.label().into(), attempts_used, final_cc));
                stages_completed += 1;
                continue 'stages;
            }
            if kernel_step_str.starts_with("Escalate") {
                let final_cc = kernel
                    .tape
                    .derive_latest_belief_state_from_tape(&scope_at_start)
                    .map(|b| b.constraints.len())
                    .unwrap_or(0);
                per_stage_attempts.push((stage.label().into(), attempts_used, final_cc));
                break 'stages;
            }

            variant_idx += 1;
            if variant_idx >= 6 {
                // Safety bound; should not hit (each stage has at most 4-5 variants).
                break;
            }
        }

        let _ = stage_idx;
    }

    // ── Serialize tape + write evidence ──
    let probe_jsonl: Vec<String> = probes.iter().map(|p| p.to_jsonl()).collect();
    let probes_path = evidence_dir.join("per_attempt_probes.jsonl");
    let probes_sha = write_jsonl(&probes_path, &probe_jsonl).unwrap_or_default();

    let mut chaintape_lines: Vec<String> = Vec::new();
    for (h, node) in &kernel.tape.indexes.by_hash {
        let json = serde_json::json!({
            "hash": h,
            "kind": serde_json::to_value(&node.kind).unwrap_or(serde_json::json!(null)),
            "verified": node.verified,
            "parent": node.parent,
            "scope": node.scope,
            "attempt_ordinal": node.attempt_ordinal,
            "reject_class": node.reject_class,
        });
        chaintape_lines.push(serde_json::to_string(&json).unwrap_or_default());
    }
    let chaintape_sha = write_jsonl(&evidence_dir.join("chaintape.jsonl"), &chaintape_lines)
        .unwrap_or_default();

    // ── Compute analytics ──
    let retry_probes: Vec<&AttemptProbe> = probes
        .iter()
        .filter(|p| p.kernel_step.starts_with("Retry"))
        .collect();
    let prompt_min = retry_probes.iter().map(|p| p.prompt_tokens).min().unwrap_or(0);
    let prompt_max = retry_probes.iter().map(|p| p.prompt_tokens).max().unwrap_or(0);
    let prompt_variance = prompt_max - prompt_min;
    let prompts_within_budget = retry_probes.iter().all(|p| p.prompt_tokens <= B_PROMPT_MAX);

    // Total BBS bytes: sum of bbs_token_count * 4 (rough byte estimate) across
    // retry probes. (BBS token counts are estimator-based; * 4 reverses the
    // 4-chars-per-token heuristic to get a byte-ish number for comparison.)
    let total_bbs_bytes: usize = retry_probes.iter().map(|p| p.bbs_token_count * 4).sum();
    let compression_ratio = if total_bbs_bytes == 0 {
        0.0
    } else {
        total_stderr_bytes as f64 / total_bbs_bytes as f64
    };

    let total_attempts = probes.len();
    let total_failed_attempts = retry_probes.len() + stages_escalated.len();
    let zero_gain_max = retry_probes.iter().map(|p| p.bbs_zero_gain_streak).max().unwrap_or(0);
    let max_constraints_in_any_bbs = retry_probes
        .iter()
        .map(|p| p.bbs_constraint_count)
        .max()
        .unwrap_or(0);

    // Distinct reject_classes observed (in retries; excludes escalation entries)
    let mut classes_seen = std::collections::BTreeSet::new();
    for p in &retry_probes {
        classes_seen.insert(p.reject_class.clone());
    }

    let manifest = serde_json::json!({
        "atom": "10",
        "branch": "feature/tdma-rc1-nesbitt-stress",
        "problem": "Nesbitt's inequality (a/(b+c)+b/(a+c)+c/(a+b) >= 3/2)",
        "ineqmath_paper": "arxiv.org/abs/2506.07927",
        "judge_backend": "NesbittStepJudge (IneqMath-style 5-category)",
        "stages_total": stages.len(),
        "stages_completed": stages_completed,
        "stages_escalated": stages_escalated,
        "total_attempts": total_attempts,
        "total_failed_attempts": total_failed_attempts,
        "total_raw_stderr_bytes": total_stderr_bytes,
        "total_bbs_bytes_estimated": total_bbs_bytes,
        "compression_ratio_stderr_over_bbs": compression_ratio,
        "prompt_tokens_min": prompt_min,
        "prompt_tokens_max": prompt_max,
        "prompt_tokens_variance": prompt_variance,
        "prompts_within_budget": prompts_within_budget,
        "b_prompt_max": B_PROMPT_MAX,
        "b_d": B_D,
        "max_retries": MAX_RETRIES,
        "max_constraints_in_any_bbs": max_constraints_in_any_bbs,
        "max_zero_gain_streak_observed": zero_gain_max,
        "distinct_reject_classes_observed": classes_seen.iter().cloned().collect::<Vec<_>>(),
        "leak_in_any_prompt": leak_anywhere,
        "per_stage": per_stage_attempts.iter().map(|(s, a, c)| {
            serde_json::json!({"stage": s, "attempts_used": a, "final_constraints": c})
        }).collect::<Vec<_>>(),
        "probes_sha256": probes_sha,
        "chaintape_sha256": chaintape_sha,
    });
    fs::write(
        evidence_dir.join("manifest.json"),
        serde_json::to_string_pretty(&manifest).unwrap_or_default(),
    )
    .ok();

    // ── Human-readable report ──
    let mut r = String::new();
    r.push_str("# TDMA-Bounded-RC1 Atom 10 — Nesbitt's Inequality Stress Report\n\n");
    r.push_str("**Problem**: Prove a/(b+c) + b/(a+c) + c/(a+b) ≥ 3/2 for a, b, c > 0.\n\n");
    r.push_str("**Real-world basis**: IneqMath benchmark (arxiv.org/abs/2506.07927). ");
    r.push_str("LLM step-level accuracy on this problem class is ≤ 10% per the paper.\n\n");
    r.push_str("**Judge backend**: NesbittStepJudge — 5-category IneqMath-style judge ");
    r.push_str("(direction-reversal, bad-substitution, algebra-error, logical-gap, missing-equality).\n\n");

    r.push_str("## Proof progress\n\n");
    r.push_str(&format!(
        "- Canonical stages: {}\n- Stages completed: **{}**\n- Stages escalated: {:?}\n\n",
        stages.len(),
        stages_completed,
        stages_escalated
    ));

    r.push_str("## Per-stage attempt counts\n\n");
    r.push_str("| Stage | Attempts | Final BBS constraints |\n");
    r.push_str("|---|---|---|\n");
    for (s, a, c) in &per_stage_attempts {
        r.push_str(&format!("| {} | {} | {} |\n", s, a, c));
    }

    r.push_str("\n## Compression evidence\n\n");
    r.push_str(&format!(
        "- Total attempts: **{}** ({} failed + {} successful + {} escalated)\n",
        total_attempts,
        retry_probes.len(),
        stages_completed,
        stages_escalated.len()
    ));
    r.push_str(&format!(
        "- Total raw stderr bytes ingested: **{}** ({:.1} KB)\n",
        total_stderr_bytes,
        total_stderr_bytes as f64 / 1024.0
    ));
    r.push_str(&format!(
        "- Total BBS bytes (estimated): **{}** ({:.1} KB)\n",
        total_bbs_bytes,
        total_bbs_bytes as f64 / 1024.0
    ));
    r.push_str(&format!(
        "- **Compression ratio: {:.1}x** (raw stderr / BBS)\n\n",
        compression_ratio
    ));

    r.push_str("## Prompt size invariance\n\n");
    r.push_str(&format!(
        "- Range: **{}..{}** tokens (variance {})\n",
        prompt_min, prompt_max, prompt_variance
    ));
    r.push_str(&format!(
        "- All within B_PROMPT_MAX={}: **{}**\n\n",
        B_PROMPT_MAX, prompts_within_budget
    ));

    r.push_str("## BBS / distiller machinery\n\n");
    r.push_str(&format!(
        "- Max constraints ever in any single BBS: **{}**\n",
        max_constraints_in_any_bbs
    ));
    r.push_str(&format!(
        "- Max zero_gain_streak observed: **{}**\n",
        zero_gain_max
    ));
    r.push_str(&format!(
        "- Distinct reject_classes observed: **{:?}**\n\n",
        classes_seen.iter().cloned().collect::<Vec<_>>()
    ));

    r.push_str("## KILL guard surface\n\n");
    r.push_str(&format!(
        "- Raw stderr leak in any prompt: **{}** (KILL-tdma-1)\n",
        leak_anywhere
    ));
    r.push_str("- Prompt within B_PROMPT_MAX in every retry: see above (KILL-tdma-9)\n");
    r.push_str("- verified_head never advanced on failure: structurally enforced by the kernel (KILL-tdma-5)\n\n");

    r.push_str("## Evidence integrity\n\n");
    r.push_str(&format!("- per_attempt_probes.jsonl sha256: {}\n", probes_sha));
    r.push_str(&format!("- chaintape.jsonl sha256: {}\n", chaintape_sha));

    fs::write(evidence_dir.join("CompressionReport.md"), r).ok();

    ExitCode::SUCCESS
}
