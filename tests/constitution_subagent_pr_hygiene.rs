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
    // R022_HOOK_FIX_2026-05-22: R-022 trace-matrix check moved from pre-commit
    // to commit-msg (see l8_commit_msg_hook_runs_r022_check below). pre-commit
    // must NOT call check_trace_matrix.py anymore — the migration is binding.
    assert!(
        !content.contains("check_trace_matrix.py"),
        "pre-commit.r022 must NOT invoke check_trace_matrix.py after R022_HOOK_FIX_2026-05-22 (moved to commit-msg.r022)"
    );
}

#[test]
fn l8_commit_msg_hook_runs_r022_check() {
    // R022_HOOK_FIX_2026-05-22: the in-flight commit message is only accessible
    // to commit-msg hooks (git passes the message file path as $1). Pre-commit
    // sees a stale .git/COMMIT_EDITMSG for `git commit -m` / `-F`, which made
    // valid [R-022-skip: ...] tokens invisible during the 2026-05-22 Plan v7 R1
    // hotfix. This test binds the new architecture.
    let path = "scripts/hooks/commit-msg.r022";
    assert!(
        Path::new(path).exists(),
        "R-022 commit-msg hook missing: {}",
        path
    );
    let content = fs::read_to_string(path).expect("commit-msg.r022 readable");
    assert!(
        content.contains("check_trace_matrix.py"),
        "commit-msg.r022 must invoke scripts/check_trace_matrix.py (R-022 trace-matrix check)"
    );
    assert!(
        content.contains("--mode commit"),
        "commit-msg.r022 must pass --mode commit to the checker"
    );
    assert!(
        content.contains("--message-file"),
        "commit-msg.r022 must pass the in-flight message file via --message-file $1 (the architectural fix for the COMMIT_EDITMSG footgun)"
    );
    // Verify executable bit on Unix (Windows file mode is meaningless).
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(path)
            .expect("commit-msg.r022 metadata")
            .permissions()
            .mode();
        assert!(
            mode & 0o111 != 0,
            "commit-msg.r022 must be executable (mode={:o})",
            mode
        );
    }
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
    assert!(
        content.contains("validate_git_push.sh"),
        ".claude/settings.json must wire validate_git_push.sh on PreToolUse Bash matcher (K-HARDEN-6)"
    );
}

#[test]
fn l9_git_push_hook_exists_and_blocks_main() {
    let path = ".claude/hooks/validate_git_push.sh";
    assert!(Path::new(path).exists(), "L9 mitigation hook missing: {}", path);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(path).expect("hook stat").permissions().mode();
        assert!(
            mode & 0o111 != 0,
            "L9 hook must be executable: {} (mode={:o})",
            path,
            mode
        );
    }
    let content = fs::read_to_string(path).expect("hook readable");
    assert!(
        content.contains("permissionDecision") && content.contains(r#""deny""#),
        "L9 hook must emit permissionDecision=deny JSON"
    );
    assert!(
        content.contains("origin") && content.contains("main"),
        "L9 hook must check for push to origin/main"
    );
    assert!(
        content.contains("GIT_HARDEN_ALLOW_MAIN"),
        "L9 hook must support legitimate-bypass env var"
    );
}

#[test]
fn l9_universal_git_pre_push_hook_exists() {
    let path = "scripts/hooks/pre-push.harden";
    assert!(
        Path::new(path).exists(),
        "K-HARDEN-7 universal pre-push hook missing: {} — required for cross-agent (Codex/Gemini/etc.) push-to-main block",
        path
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(path).expect("hook stat").permissions().mode();
        assert!(
            mode & 0o111 != 0,
            "Universal pre-push hook must be executable: {} (mode={:o})",
            path,
            mode
        );
    }
    let content = fs::read_to_string(path).expect("hook readable");
    assert!(
        content.contains("refs/heads/main"),
        "Pre-push hook must check refs/heads/main target"
    );
    assert!(
        content.contains("GIT_HARDEN_ALLOW_MAIN"),
        "Pre-push hook must support legitimate-bypass env var"
    );
    assert!(
        content.contains("exit 1"),
        "Pre-push hook must exit 1 on violation (git aborts push)"
    );
}

#[test]
fn install_hooks_script_installs_pre_push() {
    let content = fs::read_to_string("scripts/install_hooks.sh")
        .expect("install_hooks.sh readable");
    assert!(
        content.contains("pre-push"),
        "scripts/install_hooks.sh must install pre-push hook (K-HARDEN-7)"
    );
    assert!(
        content.contains("pre-push.harden"),
        "scripts/install_hooks.sh must link to scripts/hooks/pre-push.harden"
    );
}

#[test]
fn install_hooks_script_installs_commit_msg() {
    // R022_HOOK_FIX_2026-05-22: existing clones must re-run install_hooks.sh
    // after the R-022 migration to pick up the new commit-msg symlink. Bind
    // the requirement in a gate so future install_hooks.sh edits don't drop it.
    let content = fs::read_to_string("scripts/install_hooks.sh")
        .expect("install_hooks.sh readable");
    assert!(
        content.contains("commit-msg"),
        "scripts/install_hooks.sh must install commit-msg hook (R022_HOOK_FIX_2026-05-22)"
    );
    assert!(
        content.contains("commit-msg.r022"),
        "scripts/install_hooks.sh must link to scripts/hooks/commit-msg.r022"
    );
}

#[test]
fn setup_branch_protection_script_exists() {
    let path = "scripts/setup_branch_protection.sh";
    assert!(
        Path::new(path).exists(),
        "K-HARDEN-7 branch protection setup script missing: {} — required for server-side cross-agent enforcement",
        path
    );
    let content = fs::read_to_string(path).expect("script readable");
    assert!(
        content.contains("branches/main/protection"),
        "Branch protection script must target main branch protection API"
    );
    assert!(
        content.contains("allow_force_pushes") && content.contains("allow_deletions"),
        "Branch protection script must lock force-push and delete on main"
    );
}

#[test]
fn agents_md_documents_pr_only_workflow() {
    let content = fs::read_to_string("AGENTS.md").expect("AGENTS.md readable");
    assert!(
        content.contains("PR-only") || content.contains("PR only"),
        "AGENTS.md must document PR-only workflow rule (K-HARDEN-7)"
    );
    assert!(
        content.contains("K-HARDEN-7") || content.contains("branch protection"),
        "AGENTS.md must reference K-HARDEN-7 or branch protection mechanism"
    );
}

#[test]
fn subagent_skill_documents_pr_only_workflow() {
    let content = fs::read_to_string("skills/SUBAGENT_HARNESS.md")
        .expect("SUBAGENT_HARNESS readable");
    assert!(
        content.contains("PR-only") || content.contains("PR only"),
        "SUBAGENT_HARNESS skill must document PR-only workflow rule"
    );
    assert!(
        content.contains("Codex") || content.contains("Gemini"),
        "SUBAGENT_HARNESS skill must mention cross-agent (Codex/Gemini) scope"
    );
}

// ─── K-HARDEN-8 cross-CLI cold-start alignment tests ─────────────────────

const CROSS_CLI_ENTRY_FILES: &[(&str, &str)] = &[
    ("GEMINI.md", "Gemini CLI"),
    ("CONVENTIONS.md", "Aider"),
    (".aider.conf.yml", "Aider config"),
    (".cursorrules", "Cursor legacy"),
    (".cursor/rules/000-agents-alignment.mdc", "Cursor modern"),
    (".windsurfrules", "Windsurf"),
    (".github/copilot-instructions.md", "GitHub Copilot"),
    ("WARP.md", "Warp"),
];

#[test]
fn k_harden_8_all_cli_entry_files_exist() {
    for (path, cli) in CROSS_CLI_ENTRY_FILES {
        assert!(
            Path::new(path).exists(),
            "K-HARDEN-8: cold-start entry file missing for {}: {}",
            cli,
            path
        );
    }
}

#[test]
fn k_harden_8_all_cli_entries_point_to_agents_md() {
    // Each CLI entry file must explicitly reference AGENTS.md as canonical.
    for (path, cli) in CROSS_CLI_ENTRY_FILES {
        if path.ends_with(".yml") {
            // .aider.conf.yml: must include AGENTS.md in the read list
            let content = fs::read_to_string(path)
                .unwrap_or_else(|_| panic!("readable: {}", path));
            assert!(
                content.contains("AGENTS.md"),
                "K-HARDEN-8: {} ({}) must auto-load AGENTS.md",
                path,
                cli
            );
            continue;
        }
        let content = fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("readable: {}", path));
        assert!(
            content.contains("AGENTS.md"),
            "K-HARDEN-8: {} ({}) must reference AGENTS.md as canonical source",
            path,
            cli
        );
    }
}

#[test]
fn k_harden_8_all_cli_entries_mention_pr_only_rule() {
    // Each CLI entry file must document the PR-only workflow rule.
    for (path, cli) in CROSS_CLI_ENTRY_FILES {
        if path.ends_with(".yml") {
            // .aider.conf.yml is just config — skip prose check
            continue;
        }
        let content = fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("readable: {}", path));
        let mentions_pr = content.contains("PR-only")
            || content.contains("PR only")
            || content.contains("never `git push origin main`")
            || content.contains("never git push origin main");
        assert!(
            mentions_pr,
            "K-HARDEN-8: {} ({}) must document PR-only workflow rule",
            path,
            cli
        );
    }
}

#[test]
fn k_harden_8_agents_md_documents_universal_entry() {
    let content = fs::read_to_string("AGENTS.md").expect("AGENTS.md readable");
    assert!(
        content.contains("canonical universal entry") || content.contains("canonical entry"),
        "AGENTS.md §2 must explicitly state it is the canonical universal entry for all CLIs"
    );
    // Must list at least 5 CLI families to show cross-agent intent
    let cli_mentions: usize = ["Claude", "Codex", "Gemini", "Aider", "Cursor", "Windsurf", "Copilot", "Warp"]
        .iter()
        .filter(|c| content.contains(*c))
        .count();
    assert!(
        cli_mentions >= 5,
        "AGENTS.md §2 must mention ≥5 CLI families (found {}); cross-CLI intent unclear",
        cli_mentions
    );
}
