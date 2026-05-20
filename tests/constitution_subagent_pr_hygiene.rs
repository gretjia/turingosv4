//! K-HARDEN-4 — Meta-gate verifying L5/L7/L8 hardening infrastructure is in place.
//!
//! These tests do NOT exercise subagents at runtime; they verify the
//! mechanical artifacts (hooks, scripts, skill, .gitignore entries) that
//! collectively close the L5/L7/L8 risk surface. If any of these files
//! goes missing or loses key content, the harness loses its protections
//! and this gate fails.
//!
//! ## Mapping to lessons
//!
//! - **L5 branch entanglement** → `WorktreeCreate` hook + `.gitignore` entries
//! - **L7 report-vs-actual divergence** → SUBAGENT_HARNESS skill + dispatch wrapper
//! - **L8 dirty-tree pickup** → PreToolUse `git add` validator + pre-commit chain
//!
//! ## Reference
//!
//! - `handover/architect-insights/K_HARDEN_PROPOSAL_2026-05-20.md`
//! - `handover/architect-insights/MULTI_AGENT_ISOLATION_RESEARCH_2026-05-20.md`

use std::fs;
use std::path::Path;

#[test]
fn l5_worktree_create_hook_exists_and_executable() {
    let path = ".claude/hooks/create_worktree.sh";
    assert!(
        Path::new(path).exists(),
        "L5 mitigation hook missing: {}",
        path
    );
    let metadata = fs::metadata(path).expect("hook stat");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        assert!(
            mode & 0o111 != 0,
            "L5 hook must be executable: {} (mode={:o})",
            path,
            mode
        );
    }
}

#[test]
fn l5_worktree_create_hook_uses_lock_and_no_track() {
    let content = fs::read_to_string(".claude/hooks/create_worktree.sh")
        .expect("WorktreeCreate hook readable");
    assert!(
        content.contains("--lock"),
        "L5 hook must use --lock flag (prevent auto-prune race)"
    );
    assert!(
        content.contains("--no-track"),
        "L5 hook must use --no-track flag (avoid .git/config.lock race)"
    );
    assert!(
        content.contains("origin/main"),
        "L5 hook must specify explicit origin/main start point"
    );
    assert!(
        content.contains("git clean -fdx"),
        "L5 hook must run git clean -fdx in new worktree (paranoia)"
    );
}

#[test]
fn l5_gitignore_blocks_dev_evidence_sidecar() {
    let content = fs::read_to_string(".gitignore").expect(".gitignore readable");
    assert!(
        content.contains("handover/evidence/dev_self_hosting/dev_"),
        ".gitignore must block dev_self_hosting/dev_*/ sidecar paths (L5/L8)"
    );
}

#[test]
fn l5_gitignore_blocks_claude_worktrees() {
    let content = fs::read_to_string(".gitignore").expect(".gitignore readable");
    assert!(
        content.contains(".claude/worktrees/"),
        ".gitignore must block .claude/worktrees/ subagent workspaces"
    );
}

#[test]
fn l8_git_add_hook_exists_and_executable() {
    let path = ".claude/hooks/validate_git_add.sh";
    assert!(
        Path::new(path).exists(),
        "L8 mitigation hook missing: {}",
        path
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(path).expect("hook stat").permissions().mode();
        assert!(
            mode & 0o111 != 0,
            "L8 hook must be executable: {} (mode={:o})",
            path,
            mode
        );
    }
}

#[test]
fn l8_git_add_hook_emits_deny_for_wildcard_staging() {
    let content = fs::read_to_string(".claude/hooks/validate_git_add.sh")
        .expect("validate_git_add hook readable");
    assert!(
        content.contains("permissionDecision"),
        "L8 hook must emit permissionDecision JSON (Anthropic-official deny mechanism)"
    );
    assert!(
        content.contains(r#""deny""#),
        "L8 hook must use deny verdict explicitly"
    );
    // The regex shape — boundary anchor + dot/-A/--all/-u
    assert!(
        content.contains("git add") && (content.contains(r"-A") || content.contains("--all")),
        "L8 hook must pattern-match wildcard staging forms"
    );
}

#[test]
fn l8_pre_commit_hook_chains_k_harden_2_block() {
    let content = fs::read_to_string("scripts/hooks/pre-commit.r022")
        .expect("pre-commit.r022 readable");
    assert!(
        content.contains("K-HARDEN-2") || content.contains("sidecar"),
        "scripts/hooks/pre-commit.r022 must chain K-HARDEN-2 sidecar block (defense-in-depth at git layer)"
    );
    assert!(
        content.contains("handover/evidence/dev_self_hosting/dev_"),
        "pre-commit must check for dev_self_hosting/dev_*/ pattern"
    );
    assert!(
        content.contains("check_trace_matrix.py"),
        "pre-commit must preserve existing R-022 trace-matrix check"
    );
}

#[test]
fn l7_subagent_harness_skill_exists_with_postlude() {
    let path = "skills/SUBAGENT_HARNESS.md";
    assert!(Path::new(path).exists(), "L7 skill missing: {}", path);
    let content = fs::read_to_string(path).expect("skill readable");
    assert!(
        content.contains("POSTLUDE"),
        "L7 skill must define mandatory POSTLUDE verification block"
    );
    assert!(
        content.contains("headRefOid") || content.contains("headRefSha"),
        "L7 skill POSTLUDE must compare PR's head SHA against local git rev-parse HEAD"
    );
    assert!(
        content.contains("BRANCH:") && content.contains("HEAD_SHA:") && content.contains("PR_NUMBER:"),
        "L7 skill must mandate 5-field final-report format"
    );
}

#[test]
fn l7_dispatch_wrapper_exists() {
    let path = "scripts/dispatch_subagent.sh";
    assert!(Path::new(path).exists(), "L7 dispatch wrapper missing: {}", path);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(path).expect("script stat").permissions().mode();
        assert!(
            mode & 0o111 != 0,
            "L7 dispatch wrapper must be executable: {} (mode={:o})",
            path,
            mode
        );
    }
    let content = fs::read_to_string(path).expect("dispatch script readable");
    assert!(
        content.contains("pre-flight") || content.contains("Pre-flight"),
        "L7 dispatch wrapper must have pre-flight cleanliness section"
    );
    assert!(
        content.contains("post-dispatch") || content.contains("Post-dispatch") || content.contains("post:"),
        "L7 dispatch wrapper must have post-dispatch contamination scan"
    );
}

#[test]
fn settings_json_wires_worktree_create_and_validate_git_add() {
    let content = fs::read_to_string(".claude/settings.json")
        .expect(".claude/settings.json readable");
    assert!(
        content.contains("WorktreeCreate"),
        ".claude/settings.json must wire WorktreeCreate hook (K-HARDEN-1)"
    );
    assert!(
        content.contains("create_worktree.sh"),
        ".claude/settings.json WorktreeCreate must point at create_worktree.sh"
    );
    assert!(
        content.contains("validate_git_add.sh"),
        ".claude/settings.json must wire validate_git_add.sh on PreToolUse Bash matcher (K-HARDEN-2)"
    );
}
