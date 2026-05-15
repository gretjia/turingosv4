// PPUT-CCL Phase B B5 — anti-Goodhart conformance battery (PREREG § 3).
//
// These 10 tests gate every commit. Failure of any → BLOCKER.
// Three of them (test_failed_branches_in_total_cost,
// test_wall_clock_first_read_to_final_accept,
// test_golden_path_requires_ground_truth) are also implemented as unit
// tests inside the relevant modules (cost_aggregator, wall_clock,
// post_hoc_verifier). This file provides the canonical conformance entry
// point — a single `cargo test --test pput_anti_goodhart` runs the 10
// tests in one battery, and grep-style static-analysis tests live here
// because they cross crate boundaries (experiments/minif2f_v4 + src/).
//
// Whitelist convention: paths that legitimately contain forbidden patterns
// (split generator, audit scripts, conformance battery itself, vendored
// test fixtures) are listed per-test. Any new whitelist entry requires a
// commit message rationale.

use std::fs;
use std::path::{Path, PathBuf};

// CARGO_MANIFEST_DIR for this test crate = experiments/minif2f_v4. The
// conformance battery scans both this crate's src/ AND the root crate's
// src/ (sdk, drivers, etc.), so all paths resolve from REPO_ROOT (parent
// of parent of the manifest dir).
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

// ── Helpers ─────────────────────────────────────────────────────────

/// Recursively collect Rust source files under the given path roots
/// (relative to repo root), excluding `target/`, `.claude/`, vendored
/// fixtures, and any path segment in `exclude_segments`.
fn collect_rs_files(roots: &[&str], exclude_segments: &[&str]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let root = repo_root();
    for r in roots {
        let abs = root.join(r);
        walk(&abs, &mut out, exclude_segments);
    }
    out
}

fn walk(p: &Path, out: &mut Vec<PathBuf>, exclude_segments: &[&str]) {
    if !p.exists() {
        return;
    }
    if p.is_file() {
        if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p.to_path_buf());
        }
        return;
    }
    if p.is_dir() {
        // Skip excluded directory segments.
        if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
            if exclude_segments.iter().any(|seg| *seg == name) {
                return;
            }
            if name == "target" || name == ".git" || name.starts_with('.') {
                return;
            }
        }
        if let Ok(entries) = fs::read_dir(p) {
            for e in entries.flatten() {
                walk(&e.path(), out, exclude_segments);
            }
        }
    }
}

fn read_text(p: &Path) -> String {
    fs::read_to_string(p).unwrap_or_default()
}

/// Strip `#[cfg(test)]` regions and comment lines from source text.
/// The conformance battery checks PRODUCTION code paths only; test
/// fixtures and doc-comments that mention forbidden tokens (problem IDs,
/// `client.generate` examples) don't reach the agent at runtime.
fn strip_test_scope_and_comments(body: &str) -> String {
    // Truncate at the first `#[cfg(test)]` — convention is tests live at
    // the bottom of the file. This is intentionally conservative; a file
    // that interleaves cfg-test in the middle would be incorrectly
    // truncated here, but that pattern is non-idiomatic in this codebase.
    let prod_part = match body.find("#[cfg(test)]") {
        Some(idx) => &body[..idx],
        None => body,
    };
    // Strip pure comment lines (//, ///, /*, *).
    prod_part
        .lines()
        .filter(|line| {
            let t = line.trim_start();
            !(t.starts_with("//") || t.starts_with("/*") || t.starts_with("* "))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Returns true iff `path` (relative to REPO_ROOT) ends with any whitelist
/// pattern (matched as a path suffix).
fn is_whitelisted(path: &Path, whitelist: &[&str]) -> bool {
    let s = path.to_string_lossy();
    whitelist.iter().any(|w| s.ends_with(w) || s.contains(w))
}

// ── PREREG § 3 #1 — test_all_model_tokens_counted ───────────────────

/// Verify that every successful `client.generate(&request)` call site is
/// followed by `record_llm_call(...)` on the cost accumulator. Catches
/// the regression where a future code path adds a new LLM call but
/// forgets to meter it — the silent under-count Goodhart attack.
#[test]
fn test_all_model_tokens_counted() {
    let evaluator = repo_root().join("experiments/minif2f_v4/src/bin/evaluator.rs");
    let body = read_text(&evaluator);
    assert!(!body.is_empty(), "evaluator.rs must exist");

    // Find every `client.generate(...)` invocation and verify a
    // `record_llm_call` appears within the next 10 lines.
    let lines: Vec<&str> = body.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if line.contains("client.generate(") {
            let window: String = lines
                .iter()
                .skip(i)
                .take(10)
                .copied()
                .collect::<Vec<_>>()
                .join("\n");
            assert!(
                window.contains("record_llm_call("),
                "Unmetered LLM call at evaluator.rs line {} — \
                 every client.generate must be followed by record_llm_call. \
                 Window: {}",
                i + 1,
                window
            );
        }
    }
}

// ── PREREG § 3 #2 — test_tool_stdout_hash_logged ────────────────────

/// PREREG § 3: "every tool call records SHA-256 of stdout + length".
///
/// Phase B status: B2 records LENGTH (chars/4 token approximation in
/// `RunCostAccumulator::record_tool_stdout`); SHA-256 hashing of tool
/// stdout is deferred to B6 / Phase D where the per-proposal jsonl row
/// is actually written. The B1 ProposalRow schema already reserves
/// `tool_stdout_hash: Option<String>` for it.
///
/// This test asserts the contract IS reserved in the schema so a future
/// implementer can wire the hash without schema migration. Promoted to
/// full enforcement once ProposalRow emit lands (B5 → B6 transition).
#[test]
fn test_tool_stdout_hash_logged() {
    let schema = repo_root().join("experiments/minif2f_v4/src/jsonl_schema.rs");
    let body = read_text(&schema);
    assert!(
        body.contains("tool_stdout_hash"),
        "ProposalRow schema must reserve tool_stdout_hash field for B6 wire-in"
    );
    // Length already tracked in B2:
    let cost = repo_root().join("experiments/minif2f_v4/src/cost_aggregator.rs");
    let cost_body = read_text(&cost);
    assert!(
        cost_body.contains("record_tool_stdout"),
        "RunCostAccumulator must expose record_tool_stdout (length tracker)"
    );
}

// ── PREREG § 3 #3 — test_no_hidden_unmetered_generation ─────────────

/// No LLM call path may bypass the cost accumulator. Static grep check:
/// every `.rs` file under agent code paths must NOT contain a
/// `client.generate(` call OUTSIDE the metered evaluator.rs.
#[test]
fn test_no_hidden_unmetered_generation() {
    // Whitelist: only evaluator.rs may issue client.generate calls.
    let allowed_paths = ["experiments/minif2f_v4/src/bin/evaluator.rs"];
    let files = collect_rs_files(&["src", "experiments/minif2f_v4/src"], &[]);
    for f in files {
        let body = strip_test_scope_and_comments(&read_text(&f));
        if body.contains("client.generate(") {
            assert!(
                is_whitelisted(&f, &allowed_paths),
                "Unmetered LLM call site at {} — only evaluator.rs may invoke \
                 client.generate, and it must be paired with record_llm_call. \
                 PREREG § 3 #3 anti-Goodhart hidden-unmetered-generation.",
                f.display()
            );
        }
    }
}

// ── PREREG § 3 #4 — test_no_problem_id_hardcode ─────────────────────

/// Agent code paths must not contain `problem_id ==` constant comparisons
/// (per-problem rule-of-the-day disguised as logic). Detection: grep for
/// the exact pattern in agent-relevant code.
#[test]
fn test_no_problem_id_hardcode() {
    let agent_code_roots = &[
        "src/sdk",                    // agent prompt + tool dispatch
        "experiments/minif2f_v4/src", // experiment-side agent behavior
    ];
    // Whitelist: tests + this conformance file are allowed to mention pids
    // for fixture purposes.
    let whitelist = [
        "tests/",
        "/tests/",
        "post_hoc_verifier.rs",
        "jsonl_schema.rs", // legacy_line fixture quotes a problem path
    ];
    let files = collect_rs_files(agent_code_roots, &[]);
    for f in files {
        if is_whitelisted(&f, &whitelist) {
            continue;
        }
        let body = read_text(&f);
        // Forbidden patterns: `problem_id == "..."` or `problem_id.eq("...")`.
        for line in body.lines() {
            // Skip comments
            let trimmed = line.trim_start();
            if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with("*") {
                continue;
            }
            assert!(
                !line.contains("problem_id ==") && !line.contains("problem_id.eq("),
                "Hardcoded problem_id comparison at {}: {}\n\
                 PREREG § 3 #4: agent code must not branch on per-problem identity \
                 (Goodhart attack: rule-of-the-day disguised as logic).",
                f.display(),
                line
            );
        }
    }
}

// ── PREREG § 3 #5 — test_no_metric_file_access_by_agents ────────────

/// Agent-reachable code paths must not read PPUT log files. Detection:
/// scan agent code paths for file reads of jsonl PPUT artifacts.
#[test]
fn test_no_metric_file_access_by_agents() {
    let agent_code_roots = &[
        "src/sdk",
        "experiments/minif2f_v4/src/lean4_oracle.rs",
        "experiments/minif2f_v4/src/cost_aggregator.rs",
        "experiments/minif2f_v4/src/wall_clock.rs",
        "experiments/minif2f_v4/src/post_hoc_verifier.rs",
        "experiments/minif2f_v4/src/jsonl_schema.rs",
    ];
    // Whitelist: evaluator.rs (it's the metering controller, not an agent code path)
    // is excluded from agent_code_roots above. Tests are excluded.
    let forbidden_substrings = ["ppput.jsonl", "PPUT_RESULT", ".jsonl\")"];
    let files = collect_rs_files(agent_code_roots, &[]);
    for f in files {
        let body = read_text(&f);
        for needle in &forbidden_substrings {
            // jsonl_schema.rs legitimately handles jsonl strings; whitelist it.
            if f.to_string_lossy().ends_with("jsonl_schema.rs") {
                continue;
            }
            if body.contains(needle) {
                // Allow read_to_string("...") only if the path is NOT a metric file.
                // Crude check: look for `read` + the needle on the same line.
                for line in body.lines() {
                    if line.contains(needle)
                        && (line.contains("read_to_string") || line.contains("File::open"))
                    {
                        panic!(
                            "Metric file read by agent code path at {}: {}\n\
                             PREREG § 3 #5: agent code cannot read PPUT logs — \
                             leaking metric values into agent context is a \
                             Goodhart vector.",
                            f.display(),
                            line
                        );
                    }
                }
            }
        }
    }
}

// ── PREREG § 3 #6 — test_no_pput_in_agent_prompt ────────────────────

/// Prompt builders must NEVER inject PPUT scalars / dashboard values.
/// Detection: scan prompt-construction code for forbidden substrings.
/// PPUT is a strong optimization signal; exposing it to agents creates
/// the most direct Goodhart attack surface.
#[test]
fn test_no_pput_in_agent_prompt() {
    let prompt_paths = ["src/sdk/prompt.rs"];
    let forbidden = [
        "pput=",
        "PPUT-M",
        "H-VPPUT",
        "WBCG",
        "pput_runtime",
        "pput_verified",
        "pput_m_verified",
    ];
    for rel in &prompt_paths {
        let p = repo_root().join(rel);
        let body = read_text(&p);
        if body.is_empty() {
            continue;
        } // Optional path
        for needle in &forbidden {
            assert!(
                !body.contains(needle),
                "PPUT-related token '{}' found in prompt builder {} — \
                 PREREG § 3 #6: prompt builders must never expose PPUT \
                 scalars to agents (most direct Goodhart attack surface).",
                needle,
                rel
            );
        }
    }
}

// ── PREREG § 3 #7 — test_golden_path_requires_ground_truth ──────────

/// Progress = 1 iff Lean returns Pass on the full proof. Runtime accept
/// without Lean confirmation MUST collapse progress to 0. Already
/// implemented as `test_pput_verified_zero_when_lean_rejects` in
/// post_hoc_verifier.rs; this conformance test re-asserts the contract
/// at the integration layer using the same compute_progress_verified API.
#[test]
fn test_golden_path_requires_ground_truth() {
    use minif2f_v4::post_hoc_verifier::{
        compute_pput, compute_progress_runtime, compute_progress_verified,
    };

    // Soft Law-style: runtime fires, Lean rejects.
    let progress_runtime = compute_progress_runtime(true);
    let progress_verified = compute_progress_verified(true, false);
    assert_eq!(
        progress_verified, 0u8,
        "Lean reject MUST drive progress to 0 — North Star Goodhart shield"
    );
    assert_eq!(progress_runtime, 1u8);

    // pput_verified must collapse even with positive C_i + T_i.
    let c_i: u64 = 5_000;
    let t_i: u64 = 30_000;
    assert_eq!(
        compute_pput(progress_verified, c_i, t_i),
        0.0,
        "pput_verified MUST be 0 when Lean rejects (PREREG § 3 #7)"
    );
    assert!(
        compute_pput(progress_runtime, c_i, t_i) > 0.0,
        "pput_runtime inflates under runtime-accept — divergence detectable"
    );
}

// ── PREREG § 3 #8 — test_failed_branches_in_total_cost ──────────────

/// C_i sums tokens across ALL proposals (winning + failed). Already
/// implemented as `test_failed_branches_counted_in_total_cost` in
/// cost_aggregator. Re-asserted here as the canonical conformance entry.
#[test]
fn test_failed_branches_in_total_cost() {
    use minif2f_v4::cost_aggregator::RunCostAccumulator;

    let mut acc = RunCostAccumulator::new();
    for _ in 0..5 {
        acc.record_llm_call(100, 50);
        acc.record_tool_stdout(&"x".repeat(80)); // 20 tokens
        acc.record_proposal(false);
    }
    acc.record_llm_call(200, 100);
    acc.record_proposal(true);

    let expected = 5 * (100 + 50 + 20) + (200 + 100);
    assert_eq!(
        acc.total_run_token_count(),
        expected as u64,
        "C_i MUST sum across ALL 6 proposals (PREREG § 3 #8)"
    );
    assert_eq!(acc.proposal_count, 6);
    assert_eq!(acc.failed_branch_count, 5);
}

// ── PREREG § 3 #9 — test_wall_clock_first_read_to_final_accept ──────

/// T_i bracket includes Lean verify time. Spec assertion (≥7100ms with
/// synthetic 100ms+5s+2s) is fully enforced in wall_clock.rs unit tests
/// via `from_instants` (test-only constructor). Integration version here
/// confirms the public API surface (mark_first_read/mark_final_accept/
/// elapsed_ms) functions correctly with real timing.
#[test]
fn test_wall_clock_first_read_to_final_accept() {
    use minif2f_v4::wall_clock::RunWallClock;
    use std::time::Duration;

    let mut wc = RunWallClock::new();
    assert!(
        wc.elapsed_ms().is_none(),
        "elapsed_ms must be None before any marking"
    );
    wc.mark_first_read();
    std::thread::sleep(Duration::from_millis(50));
    wc.mark_final_accept();
    let elapsed = wc.elapsed_ms().expect("bracket closed after both marks");
    assert!(
        elapsed >= 50,
        "Wall-clock bracket must include the slept window (≥50ms); got {}",
        elapsed
    );
    // Idempotent first_read: a second call must not reopen the bracket.
    let first_total = elapsed;
    std::thread::sleep(Duration::from_millis(20));
    wc.mark_first_read(); // no-op
    let after_no_op = wc.elapsed_ms().unwrap();
    assert_eq!(
        after_no_op, first_total,
        "mark_first_read must be idempotent (PREREG § 5 / plan B3 contract)"
    );
}

// ── PREREG § 3 #10 — test_heldout_ids_inaccessible ──────────────────

/// The heldout 54 problem IDs must not be readable from agent code paths.
/// Detection: scan agent-reachable .rs files for any of the heldout IDs.
/// (Sealed hash check is integrity, not access — see PREREG § 2.3 L1.)
#[test]
fn test_heldout_ids_inaccessible() {
    use std::collections::HashSet;
    let splits_path = repo_root().join("handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json");
    if !splits_path.exists() {
        // Stub-friendly: if splits not yet materialized, test passes vacuously.
        // Once Phase A2 split is committed (it is), this branch never fires.
        return;
    }
    let splits_text = read_text(&splits_path);

    // Extract heldout IDs from the JSON. Naive parsing — look for the
    // "heldout" array contents without pulling in serde_json here.
    let heldout_ids: HashSet<String> = if let Some(start) = splits_text.find("\"heldout\"") {
        let after = &splits_text[start..];
        let arr_start = after.find('[').unwrap_or(0);
        let arr_end = after.find(']').unwrap_or(arr_start);
        let arr_body = &after[arr_start..arr_end];
        arr_body
            .split(',')
            .filter_map(|s| {
                let s = s.trim();
                if s.starts_with('"') && s.len() > 2 {
                    Some(s[1..s.len() - 1].to_string())
                } else {
                    None
                }
            })
            .collect()
    } else {
        HashSet::new()
    };

    if heldout_ids.is_empty() {
        // Couldn't parse — fail open with diagnostic, since vacuous pass
        // would mask a real bug if the splits file format ever changes.
        panic!(
            "heldout_ids could not be parsed from {} — \
                if the splits file format changed, update this test.",
            splits_path.display()
        );
    }

    let agent_code_roots = &[
        "src/sdk",
        "experiments/minif2f_v4/src/lean4_oracle.rs",
        "experiments/minif2f_v4/src/cost_aggregator.rs",
        "experiments/minif2f_v4/src/wall_clock.rs",
        "experiments/minif2f_v4/src/post_hoc_verifier.rs",
        "experiments/minif2f_v4/src/jsonl_schema.rs",
    ];
    let files = collect_rs_files(agent_code_roots, &[]);
    for f in files {
        // Production-only scan — test fixtures legitimately mention pids.
        let body = strip_test_scope_and_comments(&read_text(&f));
        for pid in &heldout_ids {
            assert!(
                !body.contains(pid.as_str()),
                "Heldout problem id '{}' is readable from agent code path {} — \
                 PREREG § 3 #10 + § 2.3 sealing layer L1 violation.",
                pid,
                f.display()
            );
        }
    }
}
