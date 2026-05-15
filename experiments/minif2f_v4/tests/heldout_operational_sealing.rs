// PPUT-CCL Phase B B5 — heldout operational sealing battery (PREREG § 2.3).
//
// 5 layers of operational defense against heldout leakage. PREREG § 2.3
// is explicit that this is INTEGRITY (tamper detection via SHA-256) +
// OPERATIONAL (access patterns), NOT cryptographic confidentiality —
// the heldout list is in cleartext at PPUT_CCL_SPLITS_2026-04-26.json.
// Sealing is enforced by access-path isolation across all phases.
//
// Each layer is a static-analysis grep test on agent-reachable code
// paths. Runtime enforcement (L3 tool-call pre-flight, L5 path
// enumeration block) is Phase B6/B7 work — these tests check that
// agent code does NOT currently contain bypass patterns. Once B6/B7
// add the runtime gates, additional positive-check tests will assert
// the gate code is present.
//
// Whitelist convention: bin/heldout_evaluator.rs (Phase E sealed-eval
// gate runner) and split_pput_ccl.py (Phase A2 split generator) are
// the only paths legitimately allowed to read heldout content. Both
// are Phase-bound (E only / A only) and never invoked from B-D agents.

use std::fs;
use std::path::{Path, PathBuf};

const SEED_STRING: &str = "20260426_PPUT_CCL";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn collect_rs_files(roots: &[&str]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let root = repo_root();
    for r in roots {
        walk(&root.join(r), &mut out);
    }
    out
}

fn walk(p: &Path, out: &mut Vec<PathBuf>) {
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
        if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
            if name == "target" || name == ".git" || name.starts_with('.') {
                return;
            }
        }
        if let Ok(entries) = fs::read_dir(p) {
            for e in entries.flatten() {
                walk(&e.path(), out);
            }
        }
    }
}

fn read_text(p: &Path) -> String {
    fs::read_to_string(p).unwrap_or_default()
}

fn strip_test_scope_and_comments(body: &str) -> String {
    let prod = match body.find("#[cfg(test)]") {
        Some(idx) => &body[..idx],
        None => body,
    };
    prod.lines()
        .filter(|line| {
            let t = line.trim_start();
            !(t.starts_with("//") || t.starts_with("/*") || t.starts_with("* "))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Agent-reachable code roots (excludes bin/heldout_evaluator.rs which is
/// the Phase E sealed-eval whitelist; doesn't exist yet but pre-listed).
const AGENT_CODE_ROOTS: &[&str] = &[
    "src/sdk",
    "src/drivers",
    "experiments/minif2f_v4/src/lean4_oracle.rs",
    "experiments/minif2f_v4/src/cost_aggregator.rs",
    "experiments/minif2f_v4/src/wall_clock.rs",
    "experiments/minif2f_v4/src/post_hoc_verifier.rs",
    "experiments/minif2f_v4/src/jsonl_schema.rs",
    "experiments/minif2f_v4/src/bin/evaluator.rs",
];

const WHITELIST_PHASE_BOUND: &[&str] = &[
    "bin/heldout_evaluator.rs",             // Phase E only
    "scripts/split_pput_ccl.py",            // Phase A2 only
    "tests/heldout_operational_sealing.rs", // this file
];

fn is_whitelisted(p: &Path) -> bool {
    let s = p.to_string_lossy();
    WHITELIST_PHASE_BOUND.iter().any(|w| s.contains(w))
}

// ── L1 — File-path read isolation ───────────────────────────────────

/// Agent code paths must not read heldout-bearing files (heldout_*.txt,
/// the splits JSON's heldout key). PREREG § 2.3 L1 (existing, kept).
#[test]
fn test_l1_file_path_read_isolation() {
    let forbidden_path_patterns = ["heldout_", "PPUT_CCL_SPLITS"];
    let files = collect_rs_files(AGENT_CODE_ROOTS);
    for f in files {
        if is_whitelisted(&f) {
            continue;
        }
        let body = strip_test_scope_and_comments(&read_text(&f));
        for needle in &forbidden_path_patterns {
            // Look for opens/reads of these paths (not just any mention).
            // Crude detector: same line contains `read_to_string`/`File::open`
            // AND the needle.
            for line in body.lines() {
                let has_read = line.contains("read_to_string")
                    || line.contains("File::open")
                    || line.contains("fs::read");
                if has_read && line.contains(needle) {
                    panic!(
                        "L1 sealing violation at {}: {}\n\
                         Agent code path reads heldout-bearing file '{}'. \
                         PREREG § 2.3 L1 — only bin/heldout_evaluator.rs \
                         (Phase E) may read these.",
                        f.display(),
                        line,
                        needle
                    );
                }
            }
        }
    }
}

// ── L2 — Agent prompt context blacklist ─────────────────────────────

/// At prompt construction, file BYTES from the seed-aware list must not
/// enter the prompt. Defensive: agent prompt builder must not embed any
/// file containing the seed string. PREREG § 2.3 L2.
#[test]
fn test_l2_agent_prompt_context_blacklist() {
    // The prompt builder lives in src/sdk/prompt.rs. Verify it does NOT
    // import or embed any of the listed file paths.
    let prompt_builder = repo_root().join("src/sdk/prompt.rs");
    if !prompt_builder.exists() {
        return;
    }
    let body = strip_test_scope_and_comments(&read_text(&prompt_builder));

    // The prompt builder must not name these files anywhere in production code.
    let forbidden_path_substrings = [
        "PREREG_PPUT_CCL",
        "PPUT_CCL_SPLITS",
        "split_pput_ccl",
        "PPUT_DRIVEN_FULL_PASS",
        "GEMINI_DEEPTHINK_FULL_PASS",
        "CODEX_PPUT_CCL_AUDIT",
        "GEMINI_PPUT_CCL_AUDIT",
        "DUAL_AUDIT_PPUT_CCL",
        SEED_STRING,
    ];
    for needle in &forbidden_path_substrings {
        assert!(
            !body.contains(needle),
            "L2 sealing violation: prompt builder src/sdk/prompt.rs contains \
             reference to '{}'. PREREG § 2.3 L2: agent prompt construction \
             must never embed seed-bearing files.",
            needle
        );
    }
}

// ── L3 — Tool call pre-flight filter (static check) ─────────────────

/// Agent-readable code must not contain shell hash invocations or
/// inline hash-library calls in proximity to seed substring. The
/// runtime tool-call pre-flight gate is Phase B6 work; this static
/// check is the negative-grep precondition.
#[test]
fn test_l3_tool_call_no_hash_invocation_in_agent_code() {
    let shell_hash_binaries = [
        "sha256sum",
        "sha1sum",
        "sha224sum",
        "sha384sum",
        "sha512sum",
        "md5sum",
        "b3sum",
        "b2sum",
        "openssl dgst",
    ];
    let inline_hash_calls = [
        "hashlib",
        "cryptography.hazmat.primitives.hashes",
        "Crypto.Hash",
        "crypto.createHash",
        "subtle.digest",
    ];
    let files = collect_rs_files(AGENT_CODE_ROOTS);
    for f in files {
        if is_whitelisted(&f) {
            continue;
        }
        let body = strip_test_scope_and_comments(&read_text(&f));
        for h in shell_hash_binaries.iter().chain(inline_hash_calls.iter()) {
            assert!(
                !body.contains(h),
                "L3 sealing violation at {}: agent code contains hash \
                 invocation '{}'. PREREG § 2.3 L3 — agent code paths cannot \
                 invoke hash functions (defends against seed-substring \
                 recomputation). Whitelist allowed only for Phase A/E paths.",
                f.display(),
                h
            );
        }
    }
}

// ── L4 — Hash + seed substring co-occurrence gate ───────────────────

/// Conformance: agent-readable code path containing BOTH a hash function
/// AND the seed substring is BLOCKED. PREREG § 2.3 L4 (round 4).
#[test]
fn test_l4_hash_and_seed_substring_co_occurrence() {
    let rust_hash_crates = [
        "sha2",
        "sha1",
        "sha3",
        "blake2",
        "blake3",
        "md5",
        "md-5",
        "ring::digest",
        "openssl::hash",
    ];
    let files = collect_rs_files(AGENT_CODE_ROOTS);
    for f in files {
        if is_whitelisted(&f) {
            continue;
        }
        let body = strip_test_scope_and_comments(&read_text(&f));
        let has_seed =
            body.contains(SEED_STRING) || (body.contains("20260426") && body.contains("PPUT_CCL"));
        if !has_seed {
            continue;
        }
        for h in &rust_hash_crates {
            assert!(
                !body.contains(h),
                "L4 sealing violation at {}: agent code path contains BOTH \
                 hash crate '{}' AND seed substring. PREREG § 2.3 L4: this \
                 combination is blocked at conformance test time.",
                f.display(),
                h
            );
        }
    }
}

// ── L5 — MiniF2F/Test source-pool enumeration block ─────────────────

/// Agent code must not enumerate the 244-problem MiniF2F/Test source
/// pool — that would let the agent infer the heldout 54 by exclusion
/// from the visible adaptation 144 + meta_val 46. PREREG § 2.3 L5.
///
/// Note: SearchTool (src/sdk/tools/search.rs) currently scans MiniF2F/Test
/// for filename matches under user-issued queries. This is OK in Phase B
/// because Phase B doesn't run on heldout problems. Once Phase E sealed
/// eval lands, search must be restricted to adaptation+meta_val paths
/// only — flagged as a B7 / Phase E precondition.
#[test]
fn test_l5_source_pool_enumeration_block() {
    let enumeration_patterns = ["read_dir(", "glob.glob", "os.listdir"];
    // Files that MAY mention MiniF2F/Test paths in benign config (e.g.,
    // evaluator.rs constructs the path for SearchTool/oracle args). The
    // gate tightens at Phase B7/Phase E to restrict directory traversal
    // away from MiniF2F/Test.
    let benign_path_mention_whitelist = [
        "bin/evaluator.rs", // sets up SearchTool path; Phase E will tighten
        "lean4_oracle.rs",  // accepts minif2f_dir as input arg
    ];
    let files = collect_rs_files(AGENT_CODE_ROOTS);
    for f in files {
        if is_whitelisted(&f) {
            continue;
        }
        let body = strip_test_scope_and_comments(&read_text(&f));
        let has_minif2f_test_path = body.contains("MiniF2F/Test");
        if !has_minif2f_test_path {
            continue;
        }

        // If the file just NAMES the path (config), it's benign in Phase B.
        // The gate is: name + enumeration call together = leak.
        let has_enumeration = enumeration_patterns.iter().any(|pat| body.contains(pat));
        if has_enumeration {
            // SearchTool legitimately enumerates MiniF2F/Test directory in Phase B
            // (heldout not touched until Phase E). Whitelist search.rs explicitly.
            let s = f.to_string_lossy();
            if s.ends_with("src/sdk/tools/search.rs") {
                continue;
            }
            if benign_path_mention_whitelist.iter().any(|w| s.contains(w)) {
                continue;
            }
            panic!(
                "L5 sealing violation at {}: agent code path enumerates \
                 MiniF2F/Test source pool. PREREG § 2.3 L5: directory \
                 enumeration is blocked for non-Phase-E paths. Phase B7 \
                 must add a runtime gate; Phase E whitelist is \
                 bin/heldout_evaluator.rs.",
                f.display()
            );
        }
    }
}
