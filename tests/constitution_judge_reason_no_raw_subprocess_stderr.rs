//! LIVE CONSTITUTION GATE (conformance #4) — raw-diagnostic-shield witness:
//! no judge may route raw subprocess stderr into the diagnostic reason that
//! reaches the LLM retry prompt.
//!
//! ── WHAT THIS GATE ENFORCES (the shielding invariant) ─────────────────────
//! Art. III shielding (CLAUDE.md / AGENTS.md §12): "Agent read views must be
//! scoped, reconstructable, and shielded. Do NOT expose raw Lean stderr, raw
//! autopsy logs, private diagnostics … in ordinary agent prompts." A judge's
//! `Fail.reason` / returned diagnostic flows
//! (`tdma_runner` → `distiller::extract_first_failed_predicate` →
//! `memory_kernel::compress_belief_state` → BBS → `assemble_o1_prompt`) into the
//! retry prompt verbatim. So a `Fail.reason` built from an UNBOUNDED raw
//! subprocess stderr tail (`tail_chars(&String::from_utf8_lossy(stderr), 400)`)
//! leaks raw tracebacks / harness internals into agent context — the swebench
//! bypass this gate was written to catch.
//!
//! The invariant is structural and completeness-shaped: it ENUMERATES every
//! `src/judges/*.rs` source and asserts NONE applies a raw-tail helper to
//! stderr-derived content (`tail_chars(` over a `stderr` / `from_utf8_lossy`
//! argument). The shielded references stay GREEN: lean routes stderr through
//! `shield_lean_diagnostic` (single bounded `error:` line), swebench (post-fix)
//! routes it through `classify_harness_error` (a fixed `&'static str` class
//! label), and nesbitt/putnam/generate emit deterministic structured strings.
//! A future judge that pipes a raw stderr tail into its reason turns this gate
//! RED. It is a SOURCE-STRUCTURAL witness (same family as
//! `tests/constitution_single_admission_contract.rs`): it greps the canonical
//! judge sources, so it cannot be satisfied by a vacuous `assert!(true)`.
//!
//! ── TRIPLE-COUPLING ──────────────────────────────────────────────────────
//! Registered in `scripts/constitution_gates.manifest.toml`
//! (`constitution_judge_reason_no_raw_subprocess_stderr`) and referenced in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`. Discovered by the flat
//! `ls tests/constitution_*.rs` glob in `scripts/run_constitution_gates.sh`.

use std::fs;
use std::path::PathBuf;

/// Enumerate every judge source file. This is the class S over which the
/// no-raw-stderr-passthrough invariant must hold.
fn judge_sources() -> Vec<(PathBuf, String)> {
    let mut out = Vec::new();
    for entry in fs::read_dir("src/judges").expect("src/judges readable") {
        let path = entry.expect("dir entry").path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !name.ends_with(".rs") || name == "mod.rs" {
            continue;
        }
        let src = fs::read_to_string(&path).expect("judge source readable");
        out.push((path, src));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// Strip `//`-to-EOL line comments so the fix's own explanatory comments (which
/// MENTION the forbidden `tail_chars(stderr, 400)` shape) do not trip the scan.
fn strip_line_comments(src: &str) -> String {
    src.lines()
        .map(|line| match line.find("//") {
            Some(i) => &line[..i],
            None => line,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The judge class is populated and includes the reference shielded judges.
#[test]
fn judge_class_is_populated_and_includes_reference_judges() {
    let judges = judge_sources();
    let names: Vec<String> = judges
        .iter()
        .map(|(p, _)| p.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    assert!(
        names.iter().any(|n| n == "swebench_test_judge.rs"),
        "raw-diagnostic-shield gate: swebench_test_judge.rs (the #4 site) not \
         found among src/judges/*.rs. Found: {names:?}"
    );
    assert!(
        names.iter().any(|n| n == "lean_judge.rs"),
        "raw-diagnostic-shield gate: lean_judge.rs (shielded reference) not \
         found among src/judges/*.rs. Found: {names:?}"
    );
}

/// ENUMERATE-ALL-SITES completeness: no judge applies a raw-tail helper to
/// stderr-derived content. We forbid any `tail_chars(` call whose argument
/// region (up to the statement `;`) mentions `stderr` or `from_utf8_lossy` —
/// i.e. the raw-subprocess-stderr-into-reason passthrough. `tail_chars` may
/// still EXIST and be unit-tested on literal strings; it just may not be applied
/// to stderr.
#[test]
fn no_judge_pipes_raw_stderr_tail_into_a_reason() {
    let judges = judge_sources();
    assert!(!judges.is_empty(), "no judge sources to scan");

    for (path, raw) in &judges {
        let name = path.file_name().unwrap().to_string_lossy();
        let code = strip_line_comments(raw);

        // Walk every `tail_chars(` call site (skip the `fn tail_chars` definition
        // and `assert_eq!(tail_chars(` unit-test lines, which operate on literal
        // strings, not stderr).
        let mut from = 0usize;
        while let Some(rel) = code[from..].find("tail_chars(") {
            let at = from + rel;
            from = at + "tail_chars(".len();

            // Skip the function definition itself.
            let preceding = &code[at.saturating_sub(8)..at];
            if preceding.contains("fn ") {
                continue;
            }

            // The call's argument region: from the `(` to the statement end.
            let arg_end = code[at..].find(';').map(|e| at + e).unwrap_or(code.len());
            let arg_region = &code[at..arg_end];

            let touches_stderr =
                arg_region.contains("stderr") || arg_region.contains("from_utf8_lossy");
            assert!(
                !touches_stderr,
                "raw-diagnostic-shield violation (conformance #4): src/judges/{name} \
                 applies `tail_chars(` to raw subprocess stderr \
                 (arg region mentions `stderr`/`from_utf8_lossy`). That tail flows \
                 verbatim into the LLM retry prompt. Route stderr through a bounded \
                 SHIELD instead — a fixed class label (swebench \
                 `classify_harness_error`) or a single bounded diagnostic line \
                 (lean `shield_lean_diagnostic`). Offending tail_chars arg:\n{arg_region}"
            );
        }
    }
}

/// Positive binding: the swebench harness-error path must route stderr through
/// the bounded classifier `classify_harness_error`, returning a fixed class
/// label — not the raw tail. This pins the FIX's semantics so a silent revert to
/// a raw passthrough turns the gate RED even if it stops using `tail_chars`.
#[test]
fn swebench_harness_reason_uses_bounded_classifier() {
    let src = fs::read_to_string("src/judges/swebench_test_judge.rs")
        .expect("swebench_test_judge.rs readable");
    assert!(
        src.contains("fn classify_harness_error"),
        "raw-diagnostic-shield gate: src/judges/swebench_test_judge.rs no longer \
         defines `classify_harness_error`. The harness-error reason must derive \
         from a bounded classifier, not a raw stderr tail."
    );
    let code = strip_line_comments(&src);
    assert!(
        code.contains("classify_harness_error(&String::from_utf8_lossy(stderr))"),
        "raw-diagnostic-shield gate: src/judges/swebench_test_judge.rs does not \
         feed stderr through `classify_harness_error(&String::from_utf8_lossy(\
         stderr))`. The harness-error `Fail.reason` must be built from the \
         bounded class label, never the raw subprocess tail."
    );
}
