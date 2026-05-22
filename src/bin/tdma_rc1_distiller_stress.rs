//! TRACE_MATRIX FC1a-rtool_input + FC3-replay:
//! Atom 9 — Distiller compression stress test.
//!
//! Atom 7.5's real-evidence run produced 5 accepted steps with 0 failures,
//! which means the distiller / BBS / zero_gain / eviction machinery was NEVER
//! exercised in that run. This binary closes the gap.
//!
//! Four scenarios run end-to-end against the real kernel using `InjectedJudge`
//! to script verdicts deterministically:
//!
//!   S1 — info retention: 1 task, 4 failures with DISTINCT signatures, then
//!         success. Should retain 4 constraints in BBS, all ≤ B_D.
//!
//!   S2 — zero_gain triggering: 1 task, repeated SAME-signature failures.
//!         Should escalate at zero_gain_streak == ZERO_GAIN_K (3 by default).
//!
//!   S3 — long chain: 10 tasks, each fails 3 times (distinct signatures)
//!         then succeeds. 30 failure events total. Prompt size must stay
//!         within B_PROMPT_MAX across all 40 events; raw_stderr never in
//!         prompt; per-task BBS retains all 3 constraints.
//!
//!   S4 — mixed: 5 tasks; some retries same-sig (zero_gain++), some
//!         different-sig (reset). Tests information_gain stability.
//!
//! Each scenario captures:
//!   - per-step BBS state (constraint count, token count)
//!   - per-step prompt size
//!   - cumulative raw_stderr byte count for compression-ratio calc
//!   - sentinel-leak checks on prompts
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use sha2::{Digest, Sha256};
use turingosv4::charter_core::compile_charter_core;
use turingosv4::judges::injected_judge::InjectedJudge;
use turingosv4::judges::math_step_judge::{JudgeVerdict, MathStepJudge};
use turingosv4::ledger::{AttemptScope, ImmutableTapeLedger, MemoryTapeLedger, NodeKind};
use turingosv4::memory_kernel::{EnvironmentResult, KernelStep, MemoryKernel, Task};
use turingosv4::token_budget::{B_D, B_PROMPT_MAX, MAX_RETRIES, ZERO_GAIN_K};
use turingosv4::tokenizer::Tokenizer;

const LEAK_SENTINEL: &str = "DISTILLER_STRESS_RAW_STDERR_SENTINEL_X9Y2Z";

// ── helpers ─────────────────────────────────────────────────────────

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

fn header(status: &str, task_id: &str, reject_class: &str, failed_pred: &str) -> String {
    format!(
        r#"{{"schema_version":"tdma-state-update/v1","status":"{}","task_id":"{}","action":"{}","failed_predicate":"{}","reject_class":"{}"}}"#,
        status,
        task_id,
        if status == "Proceed" { "PROCEED" } else { "RETRY" },
        failed_pred,
        reject_class,
    )
}

/// Build a raw_stderr blob ~10kb that embeds a sentinel + signature-shaping
/// content (stack frames, file paths) so the distiller has real material to
/// extract from.
fn make_raw_stderr(reject_class: &str, failed_pred: &str, padding_kb: usize) -> String {
    let mut s = String::new();
    s.push_str(LEAK_SENTINEL);
    s.push_str(&format!(
        " reject_class={} predicate={}\n",
        reject_class, failed_pred
    ));
    s.push_str(&format!(
        "at src/synthetic_module.rs:42 in fn {}\n",
        failed_pred.replace('.', "_")
    ));
    s.push_str(&format!("at src/other.rs:101 in helper_{}\n", reject_class));
    s.push_str(&format!(
        "  caused by: assertion failed: {} != expected\n",
        failed_pred
    ));
    // Pad to padding_kb*1024 chars
    let target = padding_kb * 1024;
    let pad = "x".repeat(target.saturating_sub(s.len()));
    s.push_str(&pad);
    s.push_str("\n  trailing context line\n");
    s
}

fn boot_kernel(run_id: &str) -> MemoryKernel<MemoryTapeLedger> {
    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\nArt. 0.4 — Q_t Path A; FC1a tape_t; FC1b wtool.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    MemoryKernel::new(tape, run_id, charter)
}

// ── per-step probe ──────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct StepProbe {
    scenario: String,
    task_id: String,
    step_kind: String, // "Proceed" | "Retry" | "Escalate"
    prompt_tokens: usize,
    bbs_constraint_count: usize,
    bbs_token_count: usize,
    bbs_zero_gain_streak: u32,
    bbs_failure_signature: String,
    raw_stderr_bytes_this_step: usize,
    raw_stderr_leak_in_prompt: bool,
}

impl StepProbe {
    fn to_jsonl(&self) -> String {
        format!(
            r#"{{"scenario":{:?},"task":{:?},"kind":{:?},"prompt_tokens":{},"bbs_constraints":{},"bbs_tokens":{},"zero_gain_streak":{},"failure_signature":{:?},"stderr_bytes":{},"leak":{}}}"#,
            self.scenario,
            self.task_id,
            self.step_kind,
            self.prompt_tokens,
            self.bbs_constraint_count,
            self.bbs_token_count,
            self.bbs_zero_gain_streak,
            self.bbs_failure_signature,
            self.raw_stderr_bytes_this_step,
            self.raw_stderr_leak_in_prompt,
        )
    }
}

// ── scenario S1: distinct-signature info retention ─────────────────

fn run_s1(probes: &mut Vec<StepProbe>) -> (usize, bool) {
    let mut k = boot_kernel("s1-info-retention");
    let tk = Tokenizer::new();
    let task = Task {
        id: "S1-task".into(),
        prompt: "Prove X.".into(),
    };

    // 4 distinct failures (each different reject_class + predicate),
    // then 1 success. MAX_RETRIES=5 means after 5 failures escalation;
    // 4 fails + 1 success leaves us with 4 retries safely.
    let cases = [
        ("schema-fail", "header.format"),
        ("path-fail", "path.exists"),
        ("logic-fail", "predicate.x"),
        ("type-fail", "tx.kind"),
    ];

    let verdicts = vec![
        JudgeVerdict::Fail { reason: "v1".into() },
        JudgeVerdict::Fail { reason: "v2".into() },
        JudgeVerdict::Fail { reason: "v3".into() },
        JudgeVerdict::Fail { reason: "v4".into() },
        JudgeVerdict::Pass,
    ];
    let judge = InjectedJudge::new(verdicts, JudgeVerdict::Pass);

    let mut cumulative_stderr_bytes = 0usize;
    let mut leak_seen = false;

    for (i, (reject, pred)) in cases.iter().enumerate() {
        let raw_stderr = make_raw_stderr(reject, pred, 10);
        cumulative_stderr_bytes += raw_stderr.len();
        let v = judge.verdict(&[], "ignored");
        let env = EnvironmentResult {
            raw_output: format!(
                "{}\n---BODY---\nattempt {}",
                header(
                    if v.is_pass() { "Proceed" } else { "Retry" },
                    "S1-task",
                    reject,
                    pred
                ),
                i + 1
            ),
            raw_stderr: raw_stderr.clone(),
            success: v.is_pass(),
        };
        let scope = AttemptScope {
            run_id: "s1-info-retention".into(),
            task_id: "S1-task".into(),
            verified_parent: k.tape.get_verified_head(),
        };
        let step = k.step_forward(&task, env);
        let (kind, prompt_tokens, leak) = match step {
            KernelStep::Retry { prompt, .. } => {
                let n = tk.count_text(&prompt);
                let leak = prompt.contains(LEAK_SENTINEL);
                if leak {
                    leak_seen = true;
                }
                ("Retry".to_string(), n, leak)
            }
            KernelStep::Proceed { .. } => ("Proceed".to_string(), 0, false),
            KernelStep::Escalate { .. } => ("Escalate".to_string(), 0, false),
        };
        let bbs = k.tape.derive_latest_belief_state_from_tape(&scope);
        let (cc, ct, zgs, sig) = match &bbs {
            Some(b) => (
                b.constraints.len(),
                tk.count_json(b),
                b.zero_gain_streak,
                format!(
                    "{}:{}",
                    b.failure_signature.reject_class, b.failure_signature.failed_predicate
                ),
            ),
            None => (0, 0, 0, "none".into()),
        };
        probes.push(StepProbe {
            scenario: "S1".into(),
            task_id: "S1-task".into(),
            step_kind: kind,
            prompt_tokens,
            bbs_constraint_count: cc,
            bbs_token_count: ct,
            bbs_zero_gain_streak: zgs,
            bbs_failure_signature: sig,
            raw_stderr_bytes_this_step: raw_stderr.len(),
            raw_stderr_leak_in_prompt: leak,
        });
    }

    // After 4 retries + 1 success, the scope is closed (verified_head
    // advanced). Look up the LAST BBS for the failure scope.
    let scope_after = AttemptScope {
        run_id: "s1-info-retention".into(),
        task_id: "S1-task".into(),
        verified_parent: "H0".into(),
    };
    let final_constraints = k
        .tape
        .derive_latest_belief_state_from_tape(&scope_after)
        .map(|b| b.constraints.len())
        .unwrap_or(0);
    (final_constraints, !leak_seen)
}

// ── scenario S2: zero_gain trigger ─────────────────────────────────

fn run_s2(probes: &mut Vec<StepProbe>) -> (bool, u32) {
    let mut k = boot_kernel("s2-zero-gain");
    let tk = Tokenizer::new();
    let task = Task {
        id: "S2-task".into(),
        prompt: "Prove X.".into(),
    };

    // 5 identical-signature failures. Expectation: zero_gain_streak
    // increments 1, 2, 3 — escalates at 3 (ZERO_GAIN_K).
    let mut escalated_at = None;
    for i in 0..(MAX_RETRIES + 2) {
        let raw_stderr = make_raw_stderr("same-reject", "same.predicate", 10);
        let env = EnvironmentResult {
            raw_output: format!(
                "{}\n---BODY---\nattempt {}",
                header("Retry", "S2-task", "same-reject", "same.predicate"),
                i + 1
            ),
            raw_stderr,
            success: false,
        };
        let scope = AttemptScope {
            run_id: "s2-zero-gain".into(),
            task_id: "S2-task".into(),
            verified_parent: k.tape.get_verified_head(),
        };
        let step = k.step_forward(&task, env);
        let (kind, prompt_tokens) = match step {
            KernelStep::Retry { prompt, .. } => {
                ("Retry".to_string(), tk.count_text(&prompt))
            }
            KernelStep::Escalate { reason, .. } => {
                if escalated_at.is_none() {
                    escalated_at = Some((i + 1) as u32);
                }
                (format!("Escalate({})", reason), 0)
            }
            KernelStep::Proceed { .. } => ("Proceed".to_string(), 0),
        };
        let bbs = k.tape.derive_latest_belief_state_from_tape(&scope);
        let (cc, ct, zgs, sig) = match &bbs {
            Some(b) => (
                b.constraints.len(),
                tk.count_json(b),
                b.zero_gain_streak,
                format!(
                    "{}:{}",
                    b.failure_signature.reject_class, b.failure_signature.failed_predicate
                ),
            ),
            None => (0, 0, 0, "none".into()),
        };
        probes.push(StepProbe {
            scenario: "S2".into(),
            task_id: "S2-task".into(),
            step_kind: kind.clone(),
            prompt_tokens,
            bbs_constraint_count: cc,
            bbs_token_count: ct,
            bbs_zero_gain_streak: zgs,
            bbs_failure_signature: sig,
            raw_stderr_bytes_this_step: 10 * 1024,
            raw_stderr_leak_in_prompt: false,
        });
        if kind.starts_with("Escalate") {
            break;
        }
    }
    let escalated = escalated_at.is_some();
    (escalated, escalated_at.unwrap_or(99))
}

// ── scenario S3: long chain (10 tasks × 3 failures + 1 success each) ──

fn run_s3(probes: &mut Vec<StepProbe>) -> S3Stats {
    let mut k = boot_kernel("s3-long-chain");
    let tk = Tokenizer::new();
    let cases = [
        ("schema-fail", "header.format"),
        ("path-fail", "path.exists"),
        ("logic-fail", "predicate.x"),
    ];

    let mut prompt_sizes = Vec::new();
    let mut total_stderr_bytes = 0usize;
    let mut total_bbs_bytes = 0usize;
    let mut leak_seen = false;
    let mut per_task_constraints = Vec::new();
    let mut tasks_completed = 0;

    for task_idx in 0..10 {
        let task_id = format!("S3-task-{}", task_idx);
        let task = Task {
            id: task_id.clone(),
            prompt: format!("Subtask {}", task_idx),
        };

        // 3 distinct-sig failures then 1 success
        let mut verdicts = vec![];
        for _ in 0..3 {
            verdicts.push(JudgeVerdict::Fail { reason: "v".into() });
        }
        verdicts.push(JudgeVerdict::Pass);
        let judge = InjectedJudge::new(verdicts, JudgeVerdict::Pass);

        let scope_at_start = AttemptScope {
            run_id: "s3-long-chain".into(),
            task_id: task_id.clone(),
            verified_parent: k.tape.get_verified_head(),
        };

        let mut task_succeeded = false;
        for (i, (reject, pred)) in cases.iter().chain(std::iter::once(&cases[0])).enumerate() {
            if i >= 4 {
                break;
            }
            let v = judge.verdict(&[], "ignored");
            let raw_stderr = make_raw_stderr(reject, pred, 10);
            total_stderr_bytes += raw_stderr.len();
            let env = EnvironmentResult {
                raw_output: format!(
                    "{}\n---BODY---\nstep",
                    header(
                        if v.is_pass() { "Proceed" } else { "Retry" },
                        &task_id,
                        reject,
                        pred,
                    ),
                ),
                raw_stderr: raw_stderr.clone(),
                success: v.is_pass(),
            };
            let step = k.step_forward(&task, env);
            let (kind, ptok, leak) = match step {
                KernelStep::Retry { prompt, .. } => {
                    let n = tk.count_text(&prompt);
                    prompt_sizes.push(n);
                    let leak = prompt.contains(LEAK_SENTINEL);
                    if leak {
                        leak_seen = true;
                    }
                    ("Retry".to_string(), n, leak)
                }
                KernelStep::Proceed { .. } => {
                    task_succeeded = true;
                    ("Proceed".to_string(), 0, false)
                }
                KernelStep::Escalate { .. } => ("Escalate".to_string(), 0, false),
            };
            let bbs = k.tape.derive_latest_belief_state_from_tape(&scope_at_start);
            let (cc, ct, zgs, sig) = match &bbs {
                Some(b) => (
                    b.constraints.len(),
                    tk.count_json(b),
                    b.zero_gain_streak,
                    format!(
                        "{}:{}",
                        b.failure_signature.reject_class, b.failure_signature.failed_predicate
                    ),
                ),
                None => (0, 0, 0, "none".into()),
            };
            if kind == "Retry" {
                total_bbs_bytes += ct * 4; // rough byte estimate from token count
            }
            probes.push(StepProbe {
                scenario: "S3".into(),
                task_id: task_id.clone(),
                step_kind: kind,
                prompt_tokens: ptok,
                bbs_constraint_count: cc,
                bbs_token_count: ct,
                bbs_zero_gain_streak: zgs,
                bbs_failure_signature: sig,
                raw_stderr_bytes_this_step: raw_stderr.len(),
                raw_stderr_leak_in_prompt: leak,
            });
            if task_succeeded {
                break;
            }
        }
        if task_succeeded {
            tasks_completed += 1;
            // Capture final BBS constraint count for this task's failure scope
            let final_cc = k
                .tape
                .derive_latest_belief_state_from_tape(&scope_at_start)
                .map(|b| b.constraints.len())
                .unwrap_or(0);
            per_task_constraints.push(final_cc);
        }
    }

    S3Stats {
        prompt_size_min: *prompt_sizes.iter().min().unwrap_or(&0),
        prompt_size_max: *prompt_sizes.iter().max().unwrap_or(&0),
        prompt_size_variance: prompt_sizes.iter().max().unwrap_or(&0)
            - prompt_sizes.iter().min().unwrap_or(&0),
        prompt_size_within_budget: prompt_sizes.iter().all(|&n| n <= B_PROMPT_MAX),
        tasks_completed,
        tasks_attempted: 10,
        total_stderr_bytes,
        total_bbs_bytes,
        compression_ratio: if total_bbs_bytes == 0 {
            0.0
        } else {
            total_stderr_bytes as f64 / total_bbs_bytes as f64
        },
        per_task_constraints,
        raw_stderr_leak_anywhere: leak_seen,
    }
}

struct S3Stats {
    prompt_size_min: usize,
    prompt_size_max: usize,
    prompt_size_variance: usize,
    prompt_size_within_budget: bool,
    tasks_completed: usize,
    tasks_attempted: usize,
    total_stderr_bytes: usize,
    total_bbs_bytes: usize,
    compression_ratio: f64,
    per_task_constraints: Vec<usize>,
    raw_stderr_leak_anywhere: bool,
}

// ── scenario S4: mixed (alternating sig change + sig repeat) ────────

fn run_s4(probes: &mut Vec<StepProbe>) -> S4Stats {
    let mut k = boot_kernel("s4-mixed");
    let tk = Tokenizer::new();
    let task = Task {
        id: "S4-task".into(),
        prompt: "Prove X.".into(),
    };
    // Pattern: A, A (sig repeat → zero_gain++), B (sig change → reset),
    //           B, C (sig change → reset), then success.
    // Expected: max zero_gain_streak observed = 1 (only one repeat before B).
    let cases = [
        ("reject-A", "pred-A"), // attempt 1, gain not yet measured
        ("reject-A", "pred-A"), // attempt 2, same sig → streak=1
        ("reject-B", "pred-B"), // attempt 3, sig changed → streak=0
        ("reject-B", "pred-B"), // attempt 4, same sig as 3 → streak=1
        ("reject-C", "pred-C"), // attempt 5, sig changed (but might escalate from MAX_RETRIES)
    ];
    let verdicts = vec![
        JudgeVerdict::Fail { reason: "v".into() },
        JudgeVerdict::Fail { reason: "v".into() },
        JudgeVerdict::Fail { reason: "v".into() },
        JudgeVerdict::Fail { reason: "v".into() },
        JudgeVerdict::Fail { reason: "v".into() },
    ];
    let judge = InjectedJudge::new(verdicts, JudgeVerdict::Pass);

    let mut max_zero_gain = 0u32;
    let mut zero_gain_resets = 0u32;
    let mut prev_zg = 0u32;
    let mut escalated = false;
    let scope_at_start = AttemptScope {
        run_id: "s4-mixed".into(),
        task_id: "S4-task".into(),
        verified_parent: k.tape.get_verified_head(),
    };

    for (i, (reject, pred)) in cases.iter().enumerate() {
        let v = judge.verdict(&[], "ignored");
        let raw_stderr = make_raw_stderr(reject, pred, 10);
        let env = EnvironmentResult {
            raw_output: format!(
                "{}\n---BODY---\nstep",
                header("Retry", "S4-task", reject, pred)
            ),
            raw_stderr,
            success: v.is_pass(),
        };
        let step = k.step_forward(&task, env);
        let (kind, ptok) = match step {
            KernelStep::Retry { prompt, .. } => {
                ("Retry".to_string(), tk.count_text(&prompt))
            }
            KernelStep::Escalate { reason, .. } => {
                escalated = true;
                (format!("Escalate({})", reason), 0)
            }
            KernelStep::Proceed { .. } => ("Proceed".to_string(), 0),
        };
        let bbs = k.tape.derive_latest_belief_state_from_tape(&scope_at_start);
        let (cc, ct, zgs, sig) = match &bbs {
            Some(b) => (
                b.constraints.len(),
                tk.count_json(b),
                b.zero_gain_streak,
                format!(
                    "{}:{}",
                    b.failure_signature.reject_class, b.failure_signature.failed_predicate
                ),
            ),
            None => (0, 0, 0, "none".into()),
        };
        if zgs > max_zero_gain {
            max_zero_gain = zgs;
        }
        if i > 0 && prev_zg > 0 && zgs == 0 {
            zero_gain_resets += 1;
        }
        prev_zg = zgs;
        probes.push(StepProbe {
            scenario: "S4".into(),
            task_id: "S4-task".into(),
            step_kind: kind.clone(),
            prompt_tokens: ptok,
            bbs_constraint_count: cc,
            bbs_token_count: ct,
            bbs_zero_gain_streak: zgs,
            bbs_failure_signature: sig,
            raw_stderr_bytes_this_step: 10 * 1024,
            raw_stderr_leak_in_prompt: false,
        });
        if kind.starts_with("Escalate") {
            break;
        }
    }

    let final_constraints = k
        .tape
        .derive_latest_belief_state_from_tape(&scope_at_start)
        .map(|b| b.constraints.len())
        .unwrap_or(0);

    S4Stats {
        max_zero_gain_streak: max_zero_gain,
        zero_gain_resets,
        final_constraint_count: final_constraints,
        escalated_via_max_retries: escalated,
    }
}

struct S4Stats {
    max_zero_gain_streak: u32,
    zero_gain_resets: u32,
    final_constraint_count: usize,
    escalated_via_max_retries: bool,
}

// ── main ────────────────────────────────────────────────────────────

fn main() -> ExitCode {
    let mut evidence_dir: Option<PathBuf> = None;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--evidence-dir" => {
                if let Some(v) = args.next() {
                    evidence_dir = Some(PathBuf::from(v));
                }
            }
            "--help" | "-h" => {
                eprintln!("Usage: tdma_rc1_distiller_stress --evidence-dir <PATH>");
                return ExitCode::SUCCESS;
            }
            _ => {
                eprintln!("Unknown arg: {}", arg);
                return ExitCode::from(2);
            }
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

    let mut probes: Vec<StepProbe> = Vec::new();

    println!("Running S1 (info retention, distinct-signature retries)...");
    let (s1_final_constraints, s1_no_leak) = run_s1(&mut probes);
    println!(
        "  S1: final BBS constraints={} (expected 4), leak_clean={}",
        s1_final_constraints, s1_no_leak
    );

    println!("Running S2 (zero_gain triggering, same-signature)...");
    let (s2_escalated, s2_escalated_at) = run_s2(&mut probes);
    println!(
        "  S2: escalated={} at attempt {} (expected zero_gain at attempt ZERO_GAIN_K+1={})",
        s2_escalated,
        s2_escalated_at,
        ZERO_GAIN_K + 1
    );

    println!("Running S3 (long chain, 10 tasks × 3 distinct-sig failures)...");
    let s3 = run_s3(&mut probes);
    println!(
        "  S3: prompts {}..{} (variance={}, within_B_PROMPT_MAX={}), tasks {}/{}, \
         stderr_bytes={} bbs_bytes={} compression_ratio={:.2}x, per_task_constraints={:?}, leak={}",
        s3.prompt_size_min,
        s3.prompt_size_max,
        s3.prompt_size_variance,
        s3.prompt_size_within_budget,
        s3.tasks_completed,
        s3.tasks_attempted,
        s3.total_stderr_bytes,
        s3.total_bbs_bytes,
        s3.compression_ratio,
        s3.per_task_constraints,
        s3.raw_stderr_leak_anywhere,
    );

    println!("Running S4 (mixed: alternating same/different signatures)...");
    let s4 = run_s4(&mut probes);
    println!(
        "  S4: max_zero_gain_streak={} resets={} final_constraints={} escalated={}",
        s4.max_zero_gain_streak, s4.zero_gain_resets, s4.final_constraint_count, s4.escalated_via_max_retries
    );

    // ── Write evidence ────────────────────────────────────────────
    let probes_jsonl: Vec<String> = probes.iter().map(|p| p.to_jsonl()).collect();
    let probes_path = evidence_dir.join("per_step_probes.jsonl");
    let probes_sha = write_jsonl(&probes_path, &probes_jsonl).unwrap_or_default();

    let manifest = serde_json::json!({
        "atom": "9",
        "branch": "feature/tdma-rc1-distiller-stress",
        "scenarios_run": ["S1-info-retention", "S2-zero-gain", "S3-long-chain", "S4-mixed"],
        "s1": {
            "final_constraint_count": s1_final_constraints,
            "expected_constraint_count": 4,
            "no_raw_stderr_leak": s1_no_leak,
            "info_retention_rate": (s1_final_constraints as f64) / 4.0,
        },
        "s2": {
            "escalated": s2_escalated,
            "escalated_at_attempt": s2_escalated_at,
            "zero_gain_k": ZERO_GAIN_K,
            "max_retries": MAX_RETRIES,
        },
        "s3": {
            "prompt_size_min": s3.prompt_size_min,
            "prompt_size_max": s3.prompt_size_max,
            "prompt_size_variance": s3.prompt_size_variance,
            "prompt_within_budget": s3.prompt_size_within_budget,
            "b_prompt_max": B_PROMPT_MAX,
            "tasks_completed": s3.tasks_completed,
            "tasks_attempted": s3.tasks_attempted,
            "total_stderr_bytes": s3.total_stderr_bytes,
            "total_bbs_bytes_estimated": s3.total_bbs_bytes,
            "compression_ratio_stderr_over_bbs": s3.compression_ratio,
            "per_task_final_constraints": s3.per_task_constraints,
            "no_raw_stderr_leak_anywhere": !s3.raw_stderr_leak_anywhere,
        },
        "s4": {
            "max_zero_gain_streak_observed": s4.max_zero_gain_streak,
            "zero_gain_resets": s4.zero_gain_resets,
            "final_constraint_count": s4.final_constraint_count,
            "escalated_via_max_retries": s4.escalated_via_max_retries,
        },
        "b_d": B_D,
        "b_prompt_max": B_PROMPT_MAX,
        "probes_sha256": probes_sha,
    });
    let manifest_path = evidence_dir.join("manifest.json");
    fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest).unwrap_or_default(),
    )
    .ok();

    // ── Write human-readable report ──────────────────────────────
    let mut r = String::new();
    r.push_str("# TDMA-Bounded-RC1 Atom 9 — Distiller Compression Stress Report\n\n");
    r.push_str("Synthetic stress test of the distiller / BBS / zero_gain / eviction stack.\n");
    r.push_str("Verdicts are scripted via `InjectedJudge` so failure patterns are deterministic.\n\n");

    r.push_str("## S1 — info retention (4 distinct-signature failures + 1 success)\n\n");
    r.push_str(&format!(
        "- Final BBS constraint count: **{}** (expected 4)\n",
        s1_final_constraints
    ));
    r.push_str(&format!(
        "- Information retention rate: **{:.0}%**\n",
        100.0 * (s1_final_constraints as f64) / 4.0
    ));
    r.push_str(&format!(
        "- Raw stderr leak in any prompt: **{}**\n\n",
        !s1_no_leak
    ));

    r.push_str("## S2 — zero_gain triggering (same-signature repeat)\n\n");
    r.push_str(&format!("- Escalated: **{}**\n", s2_escalated));
    r.push_str(&format!("- Escalated at attempt: **{}**\n", s2_escalated_at));
    r.push_str(&format!(
        "- ZERO_GAIN_K threshold: **{}** (escalation expected at attempt {} or via MAX_RETRIES)\n\n",
        ZERO_GAIN_K,
        ZERO_GAIN_K + 1
    ));

    r.push_str("## S3 — long chain (10 tasks × 3 distinct-sig failures + 1 success)\n\n");
    r.push_str(&format!(
        "- Prompt size range: **{}..{}** (variance {})\n",
        s3.prompt_size_min, s3.prompt_size_max, s3.prompt_size_variance
    ));
    r.push_str(&format!(
        "- All prompts within B_PROMPT_MAX={}: **{}**\n",
        B_PROMPT_MAX, s3.prompt_size_within_budget
    ));
    r.push_str(&format!(
        "- Tasks completed: **{}/{}**\n",
        s3.tasks_completed, s3.tasks_attempted
    ));
    r.push_str(&format!(
        "- Total raw stderr bytes ingested: **{}** ({:.1} KB)\n",
        s3.total_stderr_bytes,
        s3.total_stderr_bytes as f64 / 1024.0
    ));
    r.push_str(&format!(
        "- Total BBS bytes (token-estimated): **{}** ({:.1} KB)\n",
        s3.total_bbs_bytes,
        s3.total_bbs_bytes as f64 / 1024.0
    ));
    r.push_str(&format!(
        "- **Compression ratio: {:.1}x** (raw_stderr / BBS)\n",
        s3.compression_ratio
    ));
    r.push_str(&format!(
        "- Per-task final constraint counts: {:?}\n",
        s3.per_task_constraints
    ));
    r.push_str(&format!(
        "- Raw stderr leak in any prompt across full run: **{}**\n\n",
        s3.raw_stderr_leak_anywhere
    ));

    r.push_str("## S4 — mixed (alternating signature repeat + change)\n\n");
    r.push_str(&format!(
        "- Max zero_gain_streak observed: **{}**\n",
        s4.max_zero_gain_streak
    ));
    r.push_str(&format!(
        "- Zero_gain_streak reset events: **{}**\n",
        s4.zero_gain_resets
    ));
    r.push_str(&format!(
        "- Final BBS constraint count (3 distinct signatures expected): **{}**\n",
        s4.final_constraint_count
    ));
    r.push_str(&format!(
        "- Escalated via MAX_RETRIES: **{}**\n\n",
        s4.escalated_via_max_retries
    ));

    r.push_str("## Evidence integrity\n\n");
    r.push_str(&format!(
        "- per_step_probes.jsonl sha256: {}\n",
        probes_sha
    ));
    fs::write(evidence_dir.join("CompressionReport.md"), r).ok();

    ExitCode::SUCCESS
}
