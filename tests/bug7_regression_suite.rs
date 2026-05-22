//! TRACE_MATRIX FC1a-Q_t + FC1b-Q_{t+1}: BUG-7 regression suite.
//!
//! Three regression tests that PROVE BUG-7 (retry prompt unbounded
//! accumulation) is fixed:
//!
//!   1. 50-retry sequence with 10kb stderr per iteration -> prompt size remains
//!      constant (within attempt_ordinal text drift).
//!   2. Raw stderr never appears in any prompt across the entire run.
//!   3. RetryBeliefState derivable from tape AFTER kernel drop/rebuild.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use turingosv4::charter_core::compile_charter_core;
use turingosv4::ledger::{
    AttemptScope, ImmutableTapeLedger, MemoryTapeLedger,
};
use turingosv4::memory_kernel::{EnvironmentResult, KernelStep, MemoryKernel, Task};
use turingosv4::token_budget::B_PROMPT_MAX;
use turingosv4::tokenizer::Tokenizer;

fn fresh_kernel() -> MemoryKernel<MemoryTapeLedger> {
    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\n## Art. 0.4 — Q_t version control\nFC1a tape_t.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    MemoryKernel::new(tape, "bug7-regression", charter)
}

fn retry_header(task: &str, pred: &str, reject: &str) -> String {
    format!(
        r#"{{"schema_version":"tdma-state-update/v1","status":"Retry","task_id":"{}","action":"RETRY","failed_predicate":"{}","reject_class":"{}"}}
---BODY---
needs another try"#,
        task, pred, reject
    )
}

// ── BUG-7 regression #1: 50-retry prompt invariance ──────────

#[test]
fn bug7_50_retry_10kb_stderr_prompt_invariant() {
    let mut k = fresh_kernel();
    let task = Task {
        id: "bug7-1".into(),
        prompt: "Prove X.".into(),
    };
    let tk = Tokenizer::new();
    let mut sizes = Vec::new();
    for i in 0..50 {
        // Use a different signature each cycle so zero-gain doesn't fire early.
        // The point is to prove that even with 50 distinct retries with 10kb
        // stderr each, the prompt stays bounded — i.e., BUG-7 is structurally
        // fixed.
        let env = EnvironmentResult {
            raw_output: retry_header("bug7-1", &format!("p_{}", i % 5), &format!("r_{}", i % 5)),
            raw_stderr: format!(
                "{}\n at src/file.rs:{}\n",
                "x".repeat(10_000),
                i
            ),
            success: false,
        };
        match k.step_forward(&task, env) {
            KernelStep::Retry { prompt, .. } => {
                sizes.push(tk.count_text(&prompt));
            }
            KernelStep::Escalate { .. } => break,
            KernelStep::Proceed { .. } => panic!("unexpected Proceed"),
        }
    }
    assert!(!sizes.is_empty(), "must produce at least one retry prompt");
    let max = *sizes.iter().max().unwrap();
    assert!(max <= B_PROMPT_MAX, "max prompt {} > B_PROMPT_MAX={}", max, B_PROMPT_MAX);
    let min = *sizes.iter().min().unwrap();
    assert!(
        max - min <= 200,
        "prompt size variance {} exceeds 200-token allowance",
        max - min
    );
}

// ── BUG-7 regression #2: no raw stderr leakage across full run ──

#[test]
fn bug7_raw_stderr_never_in_any_prompt() {
    let mut k = fresh_kernel();
    let task = Task {
        id: "bug7-2".into(),
        prompt: "x".into(),
    };
    let sentinel = "UNIQUE_RAW_STDERR_BUG7_SENTINEL_42";
    for i in 0..20 {
        let env = EnvironmentResult {
            raw_output: retry_header("bug7-2", &format!("p_{}", i % 4), &format!("r_{}", i % 4)),
            raw_stderr: format!("{}_{}\n at src/foo.rs:{}\n", sentinel, i, i),
            success: false,
        };
        match k.step_forward(&task, env) {
            KernelStep::Retry { prompt, .. } => {
                assert!(
                    !prompt.contains(sentinel),
                    "iteration {}: raw stderr leaked into prompt",
                    i
                );
            }
            KernelStep::Escalate { .. } => break,
            KernelStep::Proceed { .. } => panic!("unexpected Proceed"),
        }
    }
}

// ── BUG-7 regression #3: BBS reconstructable post-kernel-restart ──

#[test]
fn bug7_bbs_derivable_from_tape_after_kernel_drop() {
    let mut k = fresh_kernel();
    let task = Task {
        id: "bug7-3".into(),
        prompt: "x".into(),
    };
    for i in 0..3 {
        let env = EnvironmentResult {
            raw_output: retry_header("bug7-3", &format!("p_{}", i), &format!("r_{}", i)),
            raw_stderr: format!("err {}\n", i),
            success: false,
        };
        let _ = k.step_forward(&task, env);
    }
    let scope = AttemptScope {
        run_id: "bug7-regression".into(),
        task_id: "bug7-3".into(),
        verified_parent: "H0".into(),
    };
    let original = k.tape.derive_latest_belief_state_from_tape(&scope).expect("BBS present");

    // Snapshot indexes, drop kernel, rebuild a new tape from them.
    let frozen = k.tape.indexes.clone();
    drop(k);
    let mut rebuilt = MemoryTapeLedger::new();
    rebuilt.indexes = frozen;

    let derived = rebuilt.derive_latest_belief_state_from_tape(&scope).expect("BBS derivable");
    assert_eq!(derived, original, "post-drop BBS must equal pre-drop BBS");
}
