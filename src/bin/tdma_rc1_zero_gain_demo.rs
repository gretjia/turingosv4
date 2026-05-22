//! TRACE_MATRIX FC1a-rtool_input + FC1a-zero_gain_fuse:
//! Atom 11 — Zero-gain escalation end-to-end demo.
//!
//! Atom 10's Nesbitt run produced max zero_gain_streak=1 because the worker
//! variants alternated between same-class and other-class failures. This
//! binary closes that gap: it constructs a deliberately "stuck-LLM" scenario
//! where the worker produces 4 CONSECUTIVE direction-reversal failures on
//! Step 5 (AM-GM application). Expected outcome:
//!
//!   - attempt 1: streak=0  (no prev)
//!   - attempt 2: streak=1  (same signature; information_gain < EPSILON)
//!   - attempt 3: streak=2
//!   - attempt 4: streak=3  → exceeds ZERO_GAIN_K=3 → ESCALATE
//!
//! The kernel MUST escalate with reason="ZERO_GAIN", NOT "MAX_RETRIES"
//! (which would fire at attempt 5+).
//!
//! This proves the zero_gain fuse is a structurally independent line of
//! defense (KILL-tdma-8): even if MAX_RETRIES=5 would tolerate 5 attempts,
//! the zero_gain fuse cuts the loop at 4 because the LLM is "stuck".
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

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
use turingosv4::ledger::{AttemptScope, ImmutableTapeLedger, MemoryTapeLedger};
use turingosv4::memory_kernel::{EnvironmentResult, KernelStep, MemoryKernel, Task};
use turingosv4::token_budget::{MAX_RETRIES, ZERO_GAIN_K};
use turingosv4::tokenizer::Tokenizer;

const LEAK_SENTINEL: &str = "ZERO_GAIN_DEMO_RAW_STDERR_SENTINEL_A7B9";

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

/// Build a realistic 12 KB raw_stderr for a stuck direction-reversal failure.
fn make_stuck_stderr(attempt_idx: usize) -> String {
    let mut s = String::new();
    s.push_str(LEAK_SENTINEL);
    s.push('\n');
    s.push_str(&format!(
        "IneqMath judge rejected Step5-ApplyAMGM (attempt {})\n",
        attempt_idx
    ));
    s.push_str("reject_class: direction-reversal\n");
    s.push_str("failed_predicate: amgm.direction\n");
    s.push_str("traceback:\n");
    s.push_str("  at src/judges/nesbitt_step_judge.rs:42 in verdict_for_stage\n");
    s.push_str("  at src/memory_kernel.rs:178 in handle_rejection\n");
    s.push_str(
        "  caused by: structural mismatch at stage Step5-ApplyAMGM (direction-reversal)\n",
    );
    s.push_str("\nFull rejection context:\n");
    let template =
        "  > Attempt body line: candidate step text included the direction-reversal pattern, \
         which fails the amgm.direction predicate. The judge expected AM-GM applied with \
         arithmetic mean ≥ geometric mean, but the candidate said ≤. ";
    while s.len() < 12 * 1024 {
        s.push_str(template);
    }
    s.push_str("\n[end of stderr]\n");
    s
}

fn main() -> ExitCode {
    let mut evidence_dir: Option<PathBuf> = None;
    let mut args = env::args().skip(1);
    while let Some(a) = args.next() {
        if a == "--evidence-dir" {
            if let Some(v) = args.next() {
                evidence_dir = Some(PathBuf::from(v));
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

    // ── Boot kernel and judge ──
    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\nArt. 0.4 — Q_t Path A; FC1a tape_t; FC1b wtool.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    let mut kernel = MemoryKernel::new(tape, "atom11-zero-gain-demo", charter);

    // Pre-advance the judge to Step 5 (skip Steps 1-4 by hand for focused demo).
    // We seed accepted_steps so the LogicalGap check at Step 5 doesn't fire.
    let judge = NesbittStepJudge::new();
    for _ in 0..4 {
        judge.advance();
    }
    assert_eq!(judge.current_stage.get(), NesbittStage::Step5ApplyAmGm);

    let accepted_steps: Vec<String> = (1..=4).map(|i| format!("step {} accepted", i)).collect();

    let task = Task {
        id: "stuck-stage5".into(),
        prompt: "Apply AM-GM to the three pairs.".into(),
    };

    let scope_at_start = AttemptScope {
        run_id: "atom11-zero-gain-demo".into(),
        task_id: "stuck-stage5".into(),
        verified_parent: kernel.tape.get_verified_head(),
    };

    // ── Stuck loop: 5 consecutive direction-reversal attempts ──
    // (The kernel should escalate at attempt 4 with reason=ZERO_GAIN; we
    // try a 5th to confirm escalation already happened.)
    let stuck_step =
        "By AM-GM with arithmetic mean ≤ geometric mean, each pair x/y + y/x ≤ 2.";

    let mut probes: Vec<String> = Vec::new();
    let mut escalated_at: Option<usize> = None;
    let mut escalated_reason: Option<String> = None;
    let mut zg_streak_history: Vec<u32> = Vec::new();
    let mut prompt_sizes: Vec<usize> = Vec::new();
    let tk = Tokenizer::new();

    for attempt in 1..=(MAX_RETRIES as usize + 2) {
        let (verdict, expected_class) = judge.verdict_for_stage(
            stuck_step,
            NesbittStage::Step5ApplyAmGm,
            &accepted_steps,
        );
        assert!(
            matches!(verdict, JudgeVerdict::Fail { .. }),
            "judge must reject same direction-reversal step"
        );
        assert_eq!(
            expected_class,
            Some(NesbittRejectClass::DirectionReversal),
            "every attempt must produce the SAME reject_class for zero_gain to trigger"
        );

        let raw_stderr = make_stuck_stderr(attempt);
        let env = EnvironmentResult {
            raw_output: format!(
                "{}\n---BODY---\n{}",
                header_text(
                    "Retry",
                    "stuck-stage5",
                    NesbittRejectClass::DirectionReversal.reject_class_str(),
                    NesbittRejectClass::DirectionReversal.failed_predicate_str(),
                ),
                stuck_step
            ),
            raw_stderr,
            success: false,
        };
        let step = kernel.step_forward(&task, env);

        let bbs = kernel
            .tape
            .derive_latest_belief_state_from_tape(&scope_at_start);
        let (zg, cc, ct, sig_str) = match &bbs {
            Some(b) => (
                b.zero_gain_streak,
                b.constraints.len(),
                tk.count_json(b),
                format!(
                    "{}:{}",
                    b.failure_signature.reject_class, b.failure_signature.failed_predicate
                ),
            ),
            None => (0, 0, 0, "none".into()),
        };
        zg_streak_history.push(zg);

        let (kernel_step_str, prompt_tokens, leak) = match step {
            KernelStep::Retry { prompt, .. } => {
                let n = tk.count_text(&prompt);
                prompt_sizes.push(n);
                let leak = prompt.contains(LEAK_SENTINEL);
                ("Retry".to_string(), n, leak)
            }
            KernelStep::Escalate { reason, .. } => {
                if escalated_at.is_none() {
                    escalated_at = Some(attempt);
                    escalated_reason = Some(reason.clone());
                }
                (format!("Escalate({})", reason), 0, false)
            }
            KernelStep::Proceed { .. } => ("Proceed".to_string(), 0, false),
        };

        probes.push(format!(
            r#"{{"attempt":{},"kernel_step":{:?},"zero_gain_streak":{},"bbs_constraints":{},"bbs_tokens":{},"signature":{:?},"prompt_tokens":{},"leak":{}}}"#,
            attempt, kernel_step_str, zg, cc, ct, sig_str, prompt_tokens, leak
        ));

        if kernel_step_str.starts_with("Escalate") {
            break;
        }
    }

    // ── Analytics ──
    let escalated = escalated_at.is_some();
    let escalated_at_attempt = escalated_at.unwrap_or(0);
    let reason = escalated_reason.unwrap_or_else(|| "NONE".into());

    // Expected: escalate at attempt ZERO_GAIN_K + 1 = 4 with reason "ZERO_GAIN".
    let expected_attempt = (ZERO_GAIN_K + 1) as usize;
    let timing_correct = escalated_at_attempt == expected_attempt;
    let reason_correct = reason == "ZERO_GAIN";
    // Also confirm we did NOT hit MAX_RETRIES — escalation should fire BEFORE
    // attempt MAX_RETRIES+1=6.
    let beat_max_retries = escalated_at_attempt < (MAX_RETRIES as usize + 1);

    // ── Write evidence ──
    let probes_sha = write_jsonl(&evidence_dir.join("per_attempt_probes.jsonl"), &probes)
        .unwrap_or_default();

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

    let manifest = serde_json::json!({
        "atom": "11",
        "branch": "feature/tdma-rc1-zero-gain-demo",
        "purpose": "Demonstrate zero_gain_streak escalation end-to-end on a stuck-LLM scenario",
        "zero_gain_k": ZERO_GAIN_K,
        "max_retries": MAX_RETRIES,
        "escalated": escalated,
        "escalated_at_attempt": escalated_at_attempt,
        "escalated_reason": reason,
        "expected_attempt": expected_attempt,
        "timing_correct": timing_correct,
        "reason_correct": reason_correct,
        "escalation_beat_max_retries": beat_max_retries,
        "zero_gain_streak_history": zg_streak_history,
        "prompt_tokens_per_attempt": prompt_sizes,
        "probes_sha256": probes_sha,
        "chaintape_sha256": chaintape_sha,
    });
    fs::write(
        evidence_dir.join("manifest.json"),
        serde_json::to_string_pretty(&manifest).unwrap_or_default(),
    )
    .ok();

    // Human-readable report
    let mut r = String::new();
    r.push_str("# TDMA-Bounded-RC1 Atom 11 — Zero-Gain Escalation Demo\n\n");
    r.push_str(
        "Demonstrates the zero_gain fuse (KILL-tdma-8) firing on a realistic stuck-LLM scenario.\n\n",
    );
    r.push_str("**Setup**: judge advanced to Step 5 of Nesbitt's proof; worker produces ");
    r.push_str("4+ consecutive direction-reversal failures (same signature, same predicate).\n\n");
    r.push_str("**Expected behavior**: kernel escalates at attempt ZERO_GAIN_K+1=4 with ");
    r.push_str("reason=`ZERO_GAIN`, NOT `MAX_RETRIES` (which would fire at attempt 6+).\n\n");

    r.push_str("## Outcome\n\n");
    r.push_str(&format!("- Escalated: **{}**\n", escalated));
    r.push_str(&format!(
        "- Escalated at attempt: **{}** (expected {})\n",
        escalated_at_attempt, expected_attempt
    ));
    r.push_str(&format!(
        "- Reason: **{}** (expected `ZERO_GAIN`)\n",
        reason
    ));
    r.push_str(&format!("- Timing correct: **{}**\n", timing_correct));
    r.push_str(&format!("- Reason correct: **{}**\n", reason_correct));
    r.push_str(&format!(
        "- Escalation beat MAX_RETRIES={}: **{}** (so it was the zero_gain fuse, NOT the retry counter)\n\n",
        MAX_RETRIES, beat_max_retries
    ));

    r.push_str("## zero_gain_streak history per attempt\n\n");
    r.push_str("```\n");
    for (i, zg) in zg_streak_history.iter().enumerate() {
        r.push_str(&format!("attempt {}: zero_gain_streak = {}\n", i + 1, zg));
    }
    r.push_str("```\n\n");
    r.push_str(&format!(
        "Expected progression: 0, 1, 2, [escalate before reaching attempt that would log streak ≥ ZERO_GAIN_K={}].\n\n",
        ZERO_GAIN_K
    ));

    r.push_str("## Prompt size per retry attempt\n\n");
    r.push_str("```\n");
    for (i, p) in prompt_sizes.iter().enumerate() {
        r.push_str(&format!("attempt {}: prompt = {} tokens\n", i + 1, p));
    }
    r.push_str("```\n\n");

    r.push_str("## Verdict\n\n");
    let pass = escalated && timing_correct && reason_correct && beat_max_retries;
    r.push_str(&format!(
        "**Overall: {}**\n",
        if pass { "PASS" } else { "FAIL" }
    ));
    r.push_str(&format!(
        "- per_attempt_probes.jsonl sha256: {}\n",
        probes_sha
    ));
    r.push_str(&format!("- chaintape.jsonl sha256: {}\n", chaintape_sha));

    fs::write(evidence_dir.join("ZeroGainReport.md"), r).ok();

    if escalated && timing_correct && reason_correct && beat_max_retries {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(3)
    }
}
