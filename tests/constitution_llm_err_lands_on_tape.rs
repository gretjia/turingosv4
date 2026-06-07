//! LIVE CONSTITUTION GATE (conformance #1) — append-only-rubber witness:
//! every `llm_call` failure arm in the shared TDMA runner must land a tape/CAS
//! commit BEFORE it breaks out of the proof loop.
//!
//! ── WHAT THIS GATE ENFORCES (the append-only-completeness invariant) ───────
//! FC1 canonical attempt accounting requires
//!   `evaluator_reported_completed_llm_calls = step + parse_fail + llm_err`
//! to be reconstructable from tape alone (CLAUDE.md "Canonical FC1 invariant").
//! `llm_err` is a first-class FC1 outcome
//! (`AttemptOutcome::LlmErr` / `RejectionClass::LlmError`). Every SIBLING
//! failure class in `src/tdma_runner.rs::run_proof_with_ledger`
//! (judge-reject / parse_fail / escalate) lands a verified=false `AgentProposal`
//! on tape through `kernel.step_forward_with_claims` → `handle_rejection`. The
//! `llm_call` transport-error arm used to `break 'outer` WITHOUT entering the
//! kernel, so the durable `GitTapeLedger` (production: `turingos tdma run
//! --tape-backend git`) recorded ZERO rows for that call and the FC1 equality
//! became non-reconstructable.
//!
//! The invariant is structural and completeness-shaped: it ENUMERATES every
//! `Err(` arm that lives inside a `match llm_call( … ) {` block in the canonical
//! runner source and asserts that EACH such arm performs a tape/CAS commit
//! (`step_forward_with_claims` / `step_forward_with_workspace` /
//! `tape.commit` / `put_*`) before its `break`/`continue`/`return`. A future
//! parallel `llm_call` site (a new stage loop, a second runner fn) that forgets
//! to commit before breaking turns this gate RED. It is a SOURCE-STRUCTURAL
//! witness (same family as `tests/constitution_single_admission_contract.rs`):
//! it greps the canonical source file, so it cannot be satisfied by a vacuous
//! `assert!(true)` and fails RED if the commit is removed or a new uncommitted
//! arm is added.
//!
//! ── TRIPLE-COUPLING ──────────────────────────────────────────────────────
//! Registered in `scripts/constitution_gates.manifest.toml`
//! (`constitution_llm_err_lands_on_tape`) and referenced in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`. Discovered by the flat
//! `ls tests/constitution_*.rs` glob in `scripts/run_constitution_gates.sh`.

use std::fs;

fn tdma_runner_src() -> String {
    fs::read_to_string("src/tdma_runner.rs").expect("src/tdma_runner.rs readable")
}

/// Strip `//`-to-EOL line comments so the fix's own explanatory comment (which
/// MENTIONS `step_forward_with_claims`) cannot keep the structural scan green
/// after a behavioral revert. Same helper shape as conformance gates #2/#4
/// (`constitution_external_attempt_anchored_on_failure.rs` /
/// `constitution_judge_reason_no_raw_subprocess_stderr.rs`). (Block comments are
/// not used in the policed arm; if they ever are, extend this.)
fn strip_line_comments(src: &str) -> String {
    src.lines()
        .map(|line| match line.find("//") {
            Some(i) => &line[..i],
            None => line,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Return the byte spans of every `match llm_call( … ) { … }` block body in the
/// source. Each entry is the slice between the opening `{` of the match and its
/// balanced closing `}`. There must be at least one — the canonical runner makes
/// the externalized LLM call here.
fn llm_call_match_bodies(src: &str) -> Vec<String> {
    const ANCHOR: &str = "match llm_call(";
    let mut bodies = Vec::new();
    let mut search_from = 0usize;
    while let Some(rel) = src[search_from..].find(ANCHOR) {
        let anchor_at = search_from + rel;
        // Find the `{` that opens the match arm block (first `{` after the `)`
        // that closes the `llm_call(...)` call + match scrutinee).
        let brace_open = src[anchor_at..]
            .find('{')
            .map(|o| anchor_at + o)
            .expect("match llm_call( must be followed by a `{`");
        // Walk to the balanced closing brace.
        let bytes = src.as_bytes();
        let mut depth = 0usize;
        let mut i = brace_open;
        let mut close = None;
        while i < bytes.len() {
            match bytes[i] {
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        close = Some(i);
                        break;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        let close = close.expect("match llm_call( block must have a balanced closing brace");
        bodies.push(src[brace_open..=close].to_string());
        search_from = close + 1;
    }
    bodies
}

/// There is at least one externalized `llm_call` match in the canonical runner.
/// (If the call site is renamed/removed this gate must be re-pointed, not
/// silently satisfied.)
#[test]
fn canonical_runner_has_an_llm_call_match() {
    let bodies = llm_call_match_bodies(&tdma_runner_src());
    assert!(
        !bodies.is_empty(),
        "append-only-rubber gate: no `match llm_call( … ) {{` block found in \
         src/tdma_runner.rs. The shared TDMA runner's externalized LLM call is \
         the FC1 attempt boundary; if it was renamed, re-point this gate to the \
         new call site — do NOT delete the completeness witness."
    );
}

/// ENUMERATE-ALL-SITES completeness: every `Err(` arm inside every
/// `match llm_call( … )` block must perform a tape/CAS commit BEFORE it leaves
/// the loop. We locate each `Err(` arm inside each match body, take the slice up
/// to its terminating `break`/`continue`/`return`, and require a commit call in
/// that slice.
#[test]
fn every_llm_call_err_arm_commits_before_leaving_the_loop() {
    const COMMIT_NEEDLES: &[&str] = &[
        "step_forward_with_claims",
        "step_forward_with_workspace",
        ".tape.commit(",
        "put_json",
        "put_blob",
    ];
    const EXIT_NEEDLES: &[&str] = &["break ", "break;", "continue", "return "];

    let bodies = llm_call_match_bodies(&tdma_runner_src());
    assert!(
        !bodies.is_empty(),
        "append-only-rubber gate: no `match llm_call(` block to scan."
    );

    let mut err_arms_checked = 0usize;
    for body in &bodies {
        // Find every `Err(` arm inside this match body.
        let mut from = 0usize;
        while let Some(rel) = body[from..].find("Err(") {
            let arm_at = from + rel;
            from = arm_at + 4;

            // The arm body runs until the first loop-exit keyword. We require the
            // arm to leave the loop (it is a terminal failure), and we require a
            // commit to appear strictly before that exit.
            let rest = &body[arm_at..];
            let exit_at = EXIT_NEEDLES
                .iter()
                .filter_map(|e| rest.find(e))
                .min()
                .unwrap_or_else(|| {
                    panic!(
                        "append-only-rubber gate: an `Err(` arm inside the \
                         `match llm_call(` block in src/tdma_runner.rs does not \
                         leave the loop (no break/continue/return). An \
                         externalized-LLM failure arm that neither commits nor \
                         exits is undefined behaviour for FC1 accounting."
                    )
                });
            let arm_slice = &rest[..exit_at];

            // Scan comment-stripped code so the fix's OWN explanatory comment
            // (which names `step_forward_with_claims`) cannot keep the gate green
            // after a behavioral revert — mutation-tight, matching gates #2/#4
            // which both key on `strip_line_comments(..)` output.
            let arm_code = strip_line_comments(arm_slice);
            let committed = COMMIT_NEEDLES.iter().any(|n| arm_code.contains(n));
            assert!(
                committed,
                "append-only-rubber violation (conformance #1): an `Err(` arm \
                 inside `match llm_call(` in src/tdma_runner.rs reaches its \
                 loop-exit (`break`/`continue`/`return`) WITHOUT a tape/CAS \
                 commit ({COMMIT_NEEDLES:?}). Every externalized-LLM failure must \
                 land a verified=false node on tape (reject_class=\"llm_err\") \
                 BEFORE leaving the loop, mirroring `handle_rejection`, so FC1's \
                 `completed_llm_calls = step + parse_fail + llm_err` stays \
                 reconstructable from tape. Offending arm slice:\n{arm_slice}"
            );
            err_arms_checked += 1;
        }
    }

    assert!(
        err_arms_checked >= 1,
        "append-only-rubber gate: zero `Err(` arms found inside any \
         `match llm_call(` block. The transport-error arm is mandatory; if the \
         match was rewritten to a different error-handling shape, re-point this \
         gate — do NOT let it pass vacuously."
    );
}

/// Belt-and-suspenders: the specific `reject_class="llm_err"` tag must appear in
/// the runner, so the committed node is the FC1-canonical `llm_err` outcome and
/// not some generic rejection class. This binds the fix's semantics, not just
/// its presence.
#[test]
fn llm_err_reject_class_tag_is_present() {
    let src = tdma_runner_src();
    assert!(
        src.contains(r#""reject_class":"llm_err""#) || src.contains("reject_class=\"llm_err\""),
        "append-only-rubber gate: src/tdma_runner.rs does not tag the LLM \
         transport-error commit with `reject_class=\"llm_err\"`. The landed node \
         must carry the FC1-canonical `llm_err` class so the per-class attempt \
         counts (step / parse_fail / llm_err) are distinguishable on tape."
    );
}
