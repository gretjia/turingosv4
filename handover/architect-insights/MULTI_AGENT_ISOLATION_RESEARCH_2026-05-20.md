# Multi-Agent Code Execution Isolation + Report Verification
## Research Report — TuringOS Harness Hardening

**Date:** 2026-05-20  
**Scope:** L5 branch entanglement, L7 report-vs-reality divergence, L8 dirty-tree pickup  
**Methodology:** WebSearch (10+ queries), WebFetch on official sources (Anthropic docs, GitHub issues, git-scm.com), community pattern analysis  
**Confidence labeling:** [ANTHROPIC-OFFICIAL] = Anthropic docs; [GITHUB-CONFIRMED] = reproducible GitHub issue; [COMMUNITY-PATTERN] = practitioner consensus without official backing; [SPECULATION] = untested hypothesis flagged as such

---

## Executive Summary

### L5 — Branch Entanglement

Multiple parallel subagents end up stacking on each other's branches because `isolation: "worktree"` has at least three confirmed bugs in the Anthropic issue tracker that can silently violate the isolation guarantee: (a) 8-hex agentId prefix collisions cause stale branch reuse (issue #51596, closed-stale), (b) the `.git/config.lock` is not serialized for concurrent `git worktree add` calls (issue #34645, affects Windows, reportedly also affects Linux under high parallelism), and (c) `isolation: "worktree"` is silently ignored for team agents in some versions (issue #33045). The root cause is that Claude Code derives branch names from a short agentId prefix, does not guarantee a fresh clean base, and does not serialize worktree creation. The correct mental model is: `isolation: "worktree"` provides file-level isolation *in the normal case* but is not a hard cryptographic guarantee of branch cleanliness. Hardens require: explicit timestamp-qualified branch names, a `WorktreeCreate` hook that takes control of the git plumbing, and a PRELUDE step that verifies worktree state before the agent proceeds.

### L7 — Report-vs-Reality Divergence

Subagents report "PR created at #N" but the actual git/gh state shows a different PR because (a) the subagent ran `gh pr create` on the wrong branch (inherited stale HEAD from L5), (b) the push succeeded but was force-rejected silently and the agent never verified return code, or (c) the agent read its own prior output from context and hallucinated confirmation of a past action. The industry pattern for closing this gap is a POST-COMMIT verification predicate: after every `gh pr create`, the agent must run `gh pr view --json number,headRefName,headSha` and compare the returned values against its local `git rev-parse HEAD` and its local branch name. If they do not match, the subagent must report failure, not success. No Anthropic official guidance covers this verification step explicitly.

### L8 — Dirty-Tree Pickup

Subagents run `cargo test` (or equivalent) which generates evidence files in watched directories like `handover/evidence/`. The Haiku-class model then uses `git add .` or `git add -A` despite explicit prompt instructions to use a specific file list. This happens because (a) Haiku models are less instruction-following than Sonnet/Opus, (b) `git add .` is a default learned behavior that conflicts with situational instruction, and (c) there is no mechanical enforcement. The correct fix is a `PreToolUse` hook (Anthropic-official mechanism) that intercepts every `Bash(git add *)` call, re-parses the command, and blocks calls that include `.`, `-A`, or `--all`. This is a hard block, not a prompt instruction. Separately, evidence directories should be added to `.gitignore` and a git pre-commit hook should verify the staged set against an allowlist.

---

## 1. Industry Approaches to Multi-Agent Git Isolation

### 1.1 The Dominant Pattern: One Task → One Branch → One Worktree → One Agent

[COMMUNITY-PATTERN] The consensus pattern across Augment Code, ccswarm, MindStudio, and Cursor 2.0 is:

```
one agent instance
  → one dedicated git branch
  → one git worktree directory
  → file-level isolation
```

Deviating from this (two agents on same branch, one worktree for two agents) immediately re-introduces race conditions. The principle is stated explicitly in multiple sources: "Deviate from this and you're back to race conditions." [COMMUNITY-PATTERN — multiple blog sources, not formally verified]

Cursor 2.0 (October 2025) natively supports up to 8 concurrent AI coding agents using git worktrees or remote machines per agent. Windsurf 2.0 introduced "Spaces" (agent sessions bundled with PRs, files, and context) to persist agent state across restarts. [COMMUNITY-PATTERN from search summaries — not independently verified]

### 1.2 SWE-agent Approach

[COMMUNITY-PATTERN + search result] SWE-agent uses per-task Docker containers, not git worktrees, as its primary isolation primitive. The SWE-MiniSandbox paper (arXiv 2602.11210) describes a container-free alternative that uses per-instance mount namespaces and chroot-based filesystem isolation instead of prebuilt containers, motivated by storage overhead (prebuilt container images require GB-scale storage per task). SWE-agent does not use Claude Code's `isolation: "worktree"` — it predates that mechanism.

**Lesson for our case:** The SWE-agent approach is orthogonal. We are inside Claude Code's worktree model, not dispatching Docker containers. SWE-agent's container-per-task is more robust against filesystem cross-contamination but has higher overhead.

### 1.3 ccswarm Implementation

[GITHUB-CONFIRMED from repo] ccswarm (open-source, nwiizo/ccswarm) is the most directly relevant third-party implementation. It uses:
- A **Git Worktree Manager** component with create/list/remove/prune operations
- Specialized agent pools (Frontend, Backend, DevOps, QA) each in isolated worktrees
- PTY sessions via `ai-session` crate (not Claude SDK isolation parameter)
- 30-second analysis intervals for autonomous orchestration

**Gap:** ccswarm's documentation does not specify what git commands it uses for worktree creation or how it handles branch name collisions. Source inspection would be required. This is a gap in the research — treat ccswarm as "uses worktrees" but not as a reference for hardened creation sequences.

### 1.4 Aider Approach

[SPECULATION — not found in search] The search did not return specific information about Aider's parallel agent isolation strategy. Aider is primarily a single-agent tool; its parallel dispatch story is not well-documented in the sources found. Do not treat this gap as evidence that Aider handles it well.

### 1.5 The "Agent Committed Wrong Files" Pattern

[COMMUNITY-PATTERN from Simon Willison's guide, confirmed] The practitioner pattern for "agent committed wrong files" is:
1. Use `git reset --soft HEAD~1` to undo the commit without losing changes
2. Surgically re-stage with explicit file names
3. Re-commit

Simon Willison's guide notes this as an explicit use case: "Remove uv.lock from that last commit" as a prompt to the agent. However, this is a *recovery* pattern, not a *prevention* pattern. The prevention pattern requires pre-commit hooks (see Section 5).

---

## 2. Anthropic-Specific Findings

### 2.1 Official `isolation: "worktree"` Semantics

[ANTHROPIC-OFFICIAL — code.claude.com/docs/en/worktrees]

Key confirmed behaviors:
- Worktrees branch from `origin/HEAD` by default (the remote default branch)
- Fallback to local `HEAD` if remote is unreachable or not configured
- `worktree.baseRef` setting accepts `"fresh"` or `"head"` — `"head"` makes worktrees branch from current local HEAD
- Worktrees are placed at `.claude/worktrees/<name>/` relative to repo root
- Branch name is `worktree-<name>` for `--worktree`, and `worktree-agent-<8hex>` for subagent-created worktrees
- Cleanup: auto-removed if no changes, prompts if changes exist
- `.worktreeinclude` file copies gitignored files (e.g., `.env`) into new worktrees
- Recommendation: add `.claude/worktrees/` to `.gitignore`

**Critical: There is NO `--clean-untracked` flag or equivalent.** The docs do not mention any mechanism to guarantee a worktree is clean before the agent starts. This is a documentation gap and a real operational gap.

**Critical: untracked files in the parent's worktree are NOT copied** to a new worktree unless explicitly listed in `.worktreeinclude`. [CONFIRMED by git-scm.com docs — untracked files live in the working directory, not shared git metadata] However, tracked-but-modified files in the parent's index are also not reflected in a new worktree branch. The new worktree starts at whatever commit the base ref points to, with a clean index.

### 2.2 Known Bugs in `isolation: "worktree"` — Confirmed GitHub Issues

**Issue #51596 — Stale branch reuse on agentId prefix collision** [GITHUB-CONFIRMED, closed-stale, April 21, 2026]

Root cause: Branch names are derived from 8-hex-character prefix of agentId. When prior sessions created branches with the same prefix and those branches were not pruned, new agents silently inherit:
- Uncommitted file changes
- Stash stack entries (labeled "other-agent WIP")
- Outdated base commit (days old)

Impact: silent data corruption, cross-scope file leakage, 31 minutes of wasted agent work before context overflow.

Proposed fixes (from issue, not implemented by Anthropic):
- Option A: always use unique branch names (timestamp + longer prefix)
- Option B: hard-reset if collision detected (`git clean -fdx` + `git stash clear`)

Workaround: manually prune old worktree branches before parallel dispatch. No official fix deployed.

**Issue #34645 — `.git/config.lock` race condition on concurrent worktree add** [GITHUB-CONFIRMED, closed, marked platform:windows but affects concurrent execution generally]

Root cause: Multiple `git worktree add` commands run simultaneously; each needs to write to `.git/config`. Git uses `.git/config.lock` for mutual exclusion. Concurrent writes cause lock contention.

Error:
```
error: could not lock config file .git/config: File exists
error: unable to write upstream branch configuration
```

Workaround: serialize `git worktree add` calls (run them sequentially, not all at once). The fix `git worktree add --lock` atomically creates and locks the worktree to prevent auto-pruning, but does NOT serialize against concurrent add calls for `.git/config`.

**Issue #48927 — Worktree cleanup destroys `.git/` directory** [GITHUB-CONFIRMED, open, data-loss label]

Root cause: Cleanup path confusion in worktree lifecycle management causes the cleanup mechanism to operate on the main working tree instead of the isolated worktree directory.

Impact: entire `.git/` directory deleted, all source code deleted, 4 commits permanently lost.

Related issues: #38287 (cleanup silently deletes branches with unmerged commits), #29110 (spawned agents worktree data loss), #37331 (Claude deleted all files, `.git/` replaced).

**Issue #43535 — Worktree created from wrong commit when base branch is not `main`** [GITHUB-CONFIRMED, closed as duplicate]

Root cause: `git worktree add -b <branch> <path>` without explicit start-point argument causes git to use heuristics that pick `origin/main` instead of current `HEAD`.

Fix: use `git worktree add --no-track -b <branch> <path> HEAD` (explicit start point).

**Issue #33045 — `isolation: "worktree"` silently ignored for team agents** [GITHUB-CONFIRMED, open, March 2026]

Root cause: The isolation parameter has no effect when agents are spawned via `TeamCreate` + agent dispatch path. The agent runs directly in the main repo.

Workaround: manually create worktrees before spawning team agents, pass path in prompt as `project_root`.

### 2.3 Official `WorktreeCreate` Hook

[ANTHROPIC-OFFICIAL — code.claude.com/docs/en/hooks]

Claude Code provides a `WorktreeCreate` hook that **replaces the default git worktree logic entirely**. Input schema via stdin:
```json
{
  "session_id": "...",
  "cwd": "/repo",
  "hook_event_name": "WorktreeCreate",
  "branch": "feature-branch",
  "base_path": "/repo"
}
```
The hook must print the worktree path to stdout and exit 0. Any non-zero exit fails worktree creation.

**This is the correct intervention point for L5.** By implementing `WorktreeCreate`, the harness takes full control of branch naming, locking, and base commit selection.

### 2.4 Official `PreToolUse` Hook for Git Operation Interception

[ANTHROPIC-OFFICIAL — code.claude.com/docs/en/hooks]

`PreToolUse` hooks run before any tool call. Pattern for blocking `git add .`:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(git add *)",
            "command": ".claude/hooks/validate-git-add.sh"
          }
        ]
      }
    ]
  }
}
```

Hook script can output:
```json
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "git add . is forbidden; use explicit file paths"
  }
}
```

**Critical:** This hook works even in `bypassPermissions` mode and with `--dangerously-skip-permissions`. The bypass skips interactive confirmations only, not system hooks.

### 2.5 SDK `AgentDefinition` — No `isolation` Field

[ANTHROPIC-OFFICIAL — code.claude.com/docs/en/agent-sdk/subagents]

The programmatic `AgentDefinition` (Python/TypeScript SDK) does not expose an `isolation` field. Isolation via worktree is only accessible through:
- `.claude/agents/<name>.md` frontmatter: `isolation: worktree`
- The `Agent` tool call parameter (used by Claude itself internally)

The SDK docs do not document any mechanism for verifying that a subagent ran in the correct worktree. This is a verification gap.

### 2.6 No Official PR Verification Pattern

[ANTHROPIC-OFFICIAL by absence] The official Anthropic docs do not provide any guidance on verifying that a subagent's `gh pr create` call succeeded with the correct PR number, correct branch, and correct diff. This is an undocumented gap. The only hint is: "status line JSON input now includes GitHub repo and PR information when detected" (Claude Code May 2026 release notes).

---

## 3. Git Worktree Mechanism Deep-Dive

### 3.1 Untracked Files: Confirmed NOT Inherited

[CONFIRMED — git-scm.com official docs + Claude Code official docs]

When `git worktree add` creates a new worktree:
- Tracked files at the target commit are checked out
- Untracked files from the parent worktree are NOT present
- Gitignored files are NOT present unless listed in `.worktreeinclude` (Claude Code extension)
- The new worktree has a clean working directory matching the commit

**Implication for L8:** Evidence files generated by `cargo test` in the parent worktree will NOT appear in a subagent's worktree. However, if the subagent itself runs `cargo test`, it generates evidence files in its own worktree, and those files are local and untracked. A careless `git add .` will stage them.

### 3.2 Branch Exclusivity: Hard Git Constraint

[CONFIRMED — git-scm.com official docs]

Git enforces: a branch can only be checked out in ONE worktree at a time. Attempting to check out a branch already checked out elsewhere fails with an error unless `--force` is used.

This is relevant to L5: if the parent session is on `main`, a subagent cannot check out `main` in its worktree. But if the subagent creates a new branch `worktree-agent-XXXX`, that branch can be used, unless it was used by a prior run and not deleted.

**Failure sequence for L5:**
1. Previous run created `worktree-agent-aae6004a`, ran, and was killed before cleanup
2. Branch `worktree-agent-aae6004a` still exists with uncommitted changes
3. New run with same agentId prefix collides
4. Claude Code either: (a) reuses the existing branch (silent contamination) or (b) fails with lock error

### 3.3 Concurrent `git worktree add` — Lock Contention

[GITHUB-CONFIRMED — issue #34645]

Git uses `.git/config.lock` as a mutex for writing to `.git/config`. Multiple concurrent `git worktree add` calls fail with:
```
error: could not lock config file .git/config: File exists
```

The fix Git 2.22 introduced (mkdir loop for EEXIST) applies to directory creation races, not to config file lock contention. These are separate issues.

**Implication:** When dispatching 4+ parallel subagents all with `isolation: "worktree"`, the worktree add calls may serialize automatically due to Claude Code's internal sequencing, OR they may race. If they race on the same git repo, some will fail. Claude Code's behavior on such failure is undocumented (does it retry? does it run the agent without isolation?).

### 3.4 The `--lock` Flag

[CONFIRMED — git-scm.com]

`git worktree add --lock` atomically creates the worktree AND locks it (creates a `locked` file in `.git/worktrees/<id>/`). A locked worktree is exempt from automatic pruning by `git gc`. This prevents the cleanup-destroys-active-worktree race.

**Recommendation:** Use `--lock` in any custom `WorktreeCreate` hook, and explicitly unlock with `git worktree unlock` when done.

### 3.5 Correct Command for Clean Fresh Worktree

[CONFIRMED — issue #43535 root cause fix + git-scm.com]

The canonical command to create a fresh worktree on a new branch tracking a specific clean commit:

```bash
# Ensure remote is up to date
git fetch origin main

# Create worktree at specific commit with explicit start point
git worktree add \
  --lock \
  --no-track \
  -b "worktree-agent-$(date +%s)-$(openssl rand -hex 4)" \
  ".claude/worktrees/$(date +%s)-$(openssl rand -hex 4)" \
  "origin/main"
```

Key flags:
- `--lock`: prevents auto-prune race
- `--no-track`: does not set upstream tracking (avoids `--lock` upstream write that triggers `.git/config.lock`)
- explicit start point `origin/main`: avoids the heuristic-pick-wrong-commit bug (#43535)
- timestamp + random suffix in branch name: avoids agentId prefix collision (#51596)

**Note:** `--no-track` + explicit start point is the combination that fixes both #43535 (wrong commit) and reduces `.git/config.lock` contention (no upstream write required).

### 3.6 `git config worktree.useRelativePaths`

[CONFIRMED — git-scm.com] Git 2.45+ supports `worktree.useRelativePaths` to store relative paths in `.git/worktrees/<id>/gitdir`. This makes worktrees portable when the repo is moved. It does NOT improve isolation. Not directly relevant to L5/L7/L8.

### 3.7 Refs Isolation

[CONFIRMED — git-scm.com]

Refs that are per-worktree (each worktree has its own):
- `HEAD`
- `refs/bisect/*`
- `refs/worktree/*`
- `refs/rewritten/*`

All other refs are shared across all worktrees. This means `refs/heads/<branch>` is shared. When subagent A pushes a commit to `refs/heads/worktree-agent-XXXX`, all other worktrees can see that branch. This is a feature for merge coordination but also a source of confusion if branch names collide.

---

## 4. Sandbox / Container Isolation Alternatives

### 4.1 Docker Containers Per Agent

[COMMUNITY-PATTERN + Docker official docs]

Docker Desktop 4.60+ "Sandboxes" run inside dedicated microVMs (not just containers). Each sandbox mounts the project directory at the same absolute path and preserves git configuration. This provides:
- Hard isolation (microVM boundary)
- No shared filesystem between agents
- git configuration inherited but file state fully isolated

**Cost:** 200–600ms cold-start per invocation (IBM Research benchmarks). For long-running subagent tasks (minutes), this overhead is acceptable. For shell-per-command patterns, it is prohibitive.

**Limitation:** Docker Sandboxes do not solve L7 (PR verification) or L8 (wrong git add). They solve L5 at the filesystem level but require separate PR verification logic.

### 4.2 Bubblewrap (bwrap) — What Claude Code Already Uses

[ANTHROPIC-OFFICIAL by reference from search results]

Claude Code on Linux uses Bubblewrap (bwrap) for sandboxing, but it is **off by default** as of the research date. OpenAI Codex uses Landlock + seccomp and is the only major agent with sandboxing enabled by default.

Bubblewrap provides:
- No-root namespace isolation (CLONE_NEWUSER)
- ~50KB binary, ~4000 lines of C
- Maintained by the GNOME team, tested at scale by Flatpak

**For L8 specifically:** bwrap could namespace the filesystem so that `cargo test` writes evidence files to an ephemeral tmpfs that is discarded after the subagent run. This is a strong mechanical fix but requires configuring bwrap correctly, which is non-trivial.

### 4.3 Firejail

[NOT RECOMMENDED — confirmed by sources]

Firejail requires setuid root, which is contradictory for agent isolation purposes. Multiple sources flag this as problematic.

### 4.4 SWE-MiniSandbox Approach

[RESEARCH PAPER — arXiv 2602.11210, community pattern, not production-tested for our case]

Per-instance mount namespaces + chroot-based isolation without Docker. Avoids container storage overhead. Requires Linux kernel mount namespaces (available on modern Linux). Implementation is non-trivial to set up manually.

### 4.5 Cost vs. Robustness Summary

| Approach | L5 isolation strength | L7 fix | L8 fix | Setup cost |
|---|---|---|---|---|
| git worktree (current) | Weak (multiple confirmed bugs) | None | None | Zero |
| git worktree + WorktreeCreate hook | Strong (harness-controlled) | None | None | 1 day |
| git worktree + PreToolUse hook | Strong + L8 fixed | None | Strong | 1 day |
| Docker Sandbox microVM | Strong | None | Partial | 1 week |
| bwrap + ephemeral tmpfs | Strong | None | Strong | 3 days |
| ccswarm-style PTY orchestration | Medium | None | None | 2 weeks |

**Recommendation for TuringOS:** WorktreeCreate hook + PreToolUse hook is the highest-ROI intervention. It costs ~1 day, fixes L5 and L8 mechanically, and does not require infrastructure changes.

---

## 5. PR Verification Pattern Catalog

### 5.1 What the Industry Does

[COMMUNITY-PATTERN] Standard CI/CD PR validation tools (GitHub Marketplace) verify:
- PR title matches Conventional Commits specification
- PR commits pass lint/test gates
- PR is reviewed before merge

None of these tools verify that "agent A created PR #N and PR #N has the expected diff." This is a gap the industry has not standardized.

### 5.2 The Correct Verification Predicate

[NOT ANTHROPIC-OFFICIAL — derived from first principles + search results]

After a subagent runs `gh pr create --base main --head <branch>`:

```bash
# Step 1: capture pr number from gh output
PR_NUMBER=$(gh pr create --base main --head "$BRANCH" ... | grep -oP '(?<=/pull/)\d+')

# Step 2: verify the PR exists and matches
ACTUAL_BRANCH=$(gh pr view "$PR_NUMBER" --json headRefName --jq '.headRefName')
ACTUAL_SHA=$(gh pr view "$PR_NUMBER" --json headRefSha --jq '.headRefSha')
LOCAL_SHA=$(git rev-parse HEAD)
LOCAL_BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [ "$ACTUAL_BRANCH" != "$LOCAL_BRANCH" ] || [ "$ACTUAL_SHA" != "$LOCAL_SHA" ]; then
  echo "ERROR: PR #$PR_NUMBER is on $ACTUAL_BRANCH@$ACTUAL_SHA but local is $LOCAL_BRANCH@$LOCAL_SHA"
  exit 1
fi
```

This verifies:
1. The PR exists (not hallucinated)
2. The PR points to the correct branch
3. The PR points to the correct commit SHA

### 5.3 Commit-SHA Anchoring in Subagent Report

[COMMUNITY-PATTERN — derived from CI/CD best practice]

Require every subagent's final report to include:
```
BRANCH: <branch-name>
HEAD_SHA: <git rev-parse HEAD>
PR_NUMBER: <N>
PR_URL: <url>
VERIFIED: <yes/no>
```

The orchestrator then independently runs `gh pr view <N> --json number,headRefName,headRefSha` and compares against the report. If they disagree, the subagent result is rejected.

### 5.4 GitHub Actions for Agent-Created PR Validation

[CONFIRMED — GitHub Marketplace searches]

Available GitHub Actions:
- `amannn/action-semantic-pull-request`: validates PR title against Conventional Commits
- `pr-commits-and-diff-validation`: validates PR commits against `.changes.yaml`
- `conventional-commit-checker`: checks all commits in PR for conventional format

**For our case:** a custom GitHub Action that verifies:
1. PR was created by the expected agent (check author)
2. PR diff contains only the expected files (diff predicate)
3. No sidecar evidence files are included in the diff

This can be implemented as:
```yaml
# .github/workflows/validate-agent-pr.yml
on:
  pull_request:
    types: [opened, synchronize]

jobs:
  validate-agent-pr:
    runs-on: ubuntu-latest
    steps:
      - name: Validate no evidence files in diff
        run: |
          DIFF_FILES=$(gh pr view ${{ github.event.pull_request.number }} --json files --jq '.files[].path')
          if echo "$DIFF_FILES" | grep -q '^handover/evidence/'; then
            echo "ERROR: PR contains evidence files from handover/evidence/"
            exit 1
          fi
        env:
          GH_TOKEN: ${{ github.token }}
```

### 5.5 JSON-Schema-Based PR Diff Predicates

[NOT FOUND — confirmed gap] The search did not find any project using JSON-Schema validation of PR diffs. This is a gap in the industry. The closest pattern is `changes.yaml`-based validation, which is YAML-driven but not JSON-Schema.

[SPECULATION — not tested] A harness could implement this by:
1. Defining an `expected_diff_schema.json` for each subagent task
2. Running `git diff origin/main...HEAD --name-only` after agent completion
3. Validating the resulting file list against the schema
4. Failing if the schema is violated

This would catch L8 (sidecar files) at commit time, before PR creation.

---

## 6. Recommended Harness Improvements — Concrete and Specific

### 6.1 Fix for L5: `WorktreeCreate` Hook

**Mechanism:** Replace Claude Code's default `git worktree add` with a controlled script that:
1. Generates a collision-resistant branch name (timestamp + random)
2. Runs `git fetch origin main` first
3. Uses explicit `origin/main` as start point
4. Uses `--no-track` to avoid `.git/config.lock` upstream write
5. Uses `--lock` to prevent auto-prune
6. Runs `git clean -fdx` inside the new worktree after creation
7. Verifies the worktree state before returning path

**File: `.claude/hooks/create_worktree.sh`**
```bash
#!/usr/bin/env bash
set -euo pipefail

INPUT=$(cat)
BASE_PATH=$(echo "$INPUT" | jq -r '.base_path')
REPO_ROOT="$BASE_PATH"

# 1. Generate collision-resistant branch name
TIMESTAMP=$(date +%s)
RAND=$(openssl rand -hex 4)
BRANCH="worktree-agent-${TIMESTAMP}-${RAND}"
WORKTREE_DIR="${REPO_ROOT}/.claude/worktrees/${TIMESTAMP}-${RAND}"

# 2. Fetch remote to ensure origin/main is current
git -C "$REPO_ROOT" fetch origin main --quiet 2>&1 >&2 || true

# 3. Create worktree with explicit start point, no upstream tracking
git -C "$REPO_ROOT" worktree add \
  --lock \
  --no-track \
  -b "$BRANCH" \
  "$WORKTREE_DIR" \
  "origin/main" \
  2>&1 >&2

# 4. Clean any stale untracked files that might exist (paranoia)
git -C "$WORKTREE_DIR" clean -fdx 2>&1 >&2

# 5. Verify worktree state
ACTUAL_BRANCH=$(git -C "$WORKTREE_DIR" rev-parse --abbrev-ref HEAD)
if [ "$ACTUAL_BRANCH" != "$BRANCH" ]; then
  echo "ERROR: worktree branch mismatch: expected $BRANCH, got $ACTUAL_BRANCH" >&2
  exit 1
fi

# 6. Output worktree path for Claude Code
echo "$WORKTREE_DIR"
exit 0
```

**File: `.claude/settings.json` (add to hooks section):**
```json
{
  "worktree": {
    "baseRef": "fresh"
  },
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/create_worktree.sh"
          }
        ]
      }
    ]
  }
}
```

**Expected outcome:** Eliminates stale branch reuse (#51596), wrong commit base (#43535), and reduces lock contention (#34645) by using `--no-track`.

### 6.2 Fix for L8: `PreToolUse` Hook to Block `git add .`

**Mechanism:** Intercept every `git add` command and block any form that stages non-specific paths.

**File: `.claude/hooks/validate_git_add.sh`**
```bash
#!/usr/bin/env bash
set -euo pipefail

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // ""')

# Only process git add commands
if ! echo "$COMMAND" | grep -q 'git add'; then
  exit 0
fi

# Block: git add .
# Block: git add -A
# Block: git add --all
# Block: git add -u (updates all tracked — also potentially broad)
if echo "$COMMAND" | grep -qE 'git add (\.|(-A|--all|-u)\b)'; then
  cat <<'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "git add . / -A / --all is forbidden in this harness. You MUST use explicit file paths: git add src/foo.rs tests/bar.rs. Check git status first and list only intentional files.",
    "additionalContext": "The handover/evidence/ directory contains auto-generated sidecar files that must NOT be committed. Never use git add with wildcard or bulk-stage flags."
  }
}
EOF
  exit 0
fi

# Allow: git add with specific file paths
exit 0
```

**Wire in `.claude/settings.json`:**
```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(git add *)",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/validate_git_add.sh"
          }
        ]
      }
    ]
  }
}
```

**Additional fix:** Add to `.gitignore`:
```gitignore
# Auto-generated sidecar evidence — never commit
handover/evidence/dev_*/
.claude/worktrees/
```

**Additional fix:** Add git pre-commit hook (applies even without Claude Code):
```bash
# .git/hooks/pre-commit
#!/usr/bin/env bash
STAGED=$(git diff --cached --name-only)
if echo "$STAGED" | grep -q '^handover/evidence/dev_'; then
  echo "ERROR: pre-commit: staged files include handover/evidence/ sidecar files"
  echo "Staged: $(echo "$STAGED" | grep '^handover/evidence/dev_')"
  exit 1
fi
```

### 6.3 Fix for L7: Post-Action PR Verification in Subagent Prompt

**Mechanism:** Require every subagent that creates a PR to include a mandatory verification block in its task definition.

**Subagent prompt template addition:**
```
After creating the PR, you MUST run these verification steps in this exact order:

1. CAPTURE_PR=$(gh pr create ... 2>&1)
2. PR_NUMBER=$(echo "$CAPTURE_PR" | grep -oP '(?<=/pull/)\d+' | head -1)
3. If PR_NUMBER is empty: echo "FATAL: gh pr create did not return a PR number" && exit 1
4. VERIFY=$(gh pr view "$PR_NUMBER" --json number,headRefName,headRefSha)
5. ACTUAL_BRANCH=$(echo "$VERIFY" | jq -r '.headRefName')
6. ACTUAL_SHA=$(echo "$VERIFY" | jq -r '.headRefSha')
7. LOCAL_SHA=$(git rev-parse HEAD)
8. LOCAL_BRANCH=$(git rev-parse --abbrev-ref HEAD)
9. If ACTUAL_BRANCH != LOCAL_BRANCH: echo "FATAL: PR branch mismatch: PR=$ACTUAL_BRANCH local=$LOCAL_BRANCH" && exit 1
10. If ACTUAL_SHA != LOCAL_SHA: echo "FATAL: PR SHA mismatch: PR=$ACTUAL_SHA local=$LOCAL_SHA" && exit 1

Your final report MUST include:
BRANCH: <branch>
HEAD_SHA: <sha>
PR_NUMBER: <n>
PR_URL: <url>
VERIFICATION: PASS
```

**Mechanism:** Orchestrator independently verifies by running `gh pr view $PR_NUMBER --json headRefName,headRefSha` and comparing against what the subagent reported.

### 6.4 Concrete Script: `scripts/dispatch_subagent.sh`

**Mechanism:** A wrapper script that each parallel subagent dispatch goes through. It:
1. Verifies the worktree is clean before the agent starts
2. Records the start state (HEAD SHA, branch, status)
3. After agent completes, diffs what changed
4. Optionally runs the PR verification predicate

```bash
#!/usr/bin/env bash
# scripts/dispatch_subagent.sh
# Usage: ./scripts/dispatch_subagent.sh <worktree_path> <agent_prompt_file>
set -euo pipefail

WORKTREE="$1"
PROMPT_FILE="$2"

# Pre-flight: verify worktree state
if ! git -C "$WORKTREE" diff --quiet 2>/dev/null; then
  echo "ERROR: worktree $WORKTREE has uncommitted changes before agent starts"
  git -C "$WORKTREE" status
  exit 1
fi

if [ -n "$(git -C "$WORKTREE" stash list 2>/dev/null)" ]; then
  echo "ERROR: worktree $WORKTREE has stash entries — possible stale state"
  git -C "$WORKTREE" stash list
  exit 1
fi

START_SHA=$(git -C "$WORKTREE" rev-parse HEAD)
START_BRANCH=$(git -C "$WORKTREE" rev-parse --abbrev-ref HEAD)
echo "DISPATCH: worktree=$WORKTREE branch=$START_BRANCH sha=$START_SHA"

# Run agent (placeholder — integrate with actual dispatch mechanism)
# claude -p "$(cat "$PROMPT_FILE")" --cwd "$WORKTREE" ...

END_SHA=$(git -C "$WORKTREE" rev-parse HEAD)
CHANGED_FILES=$(git -C "$WORKTREE" diff --name-only "$START_SHA" "$END_SHA")

echo "RESULT: new_sha=$END_SHA"
echo "CHANGED_FILES:"
echo "$CHANGED_FILES"

# Check for forbidden sidecar files
if echo "$CHANGED_FILES" | grep -q '^handover/evidence/dev_'; then
  echo "ERROR: agent committed sidecar evidence files:"
  echo "$CHANGED_FILES" | grep '^handover/evidence/dev_'
  exit 1
fi
```

### 6.5 Concrete Test: `tests/constitution_subagent_pr_hygiene.rs`

**Mechanism:** A Rust test module that verifies the harness mechanisms are in place. This is Class 1 (additive isolated) — no real subagents, tests static configuration and hook existence.

```rust
// tests/constitution_subagent_pr_hygiene.rs
// Verifies that L5/L7/L8 mitigation infrastructure exists and is correctly wired.

#[test]
fn l8_git_add_hook_exists() {
    let hook_path = ".claude/hooks/validate_git_add.sh";
    assert!(
        std::path::Path::new(hook_path).exists(),
        "L8 mitigation hook missing: {}", hook_path
    );
    // Verify it's executable
    let meta = std::fs::metadata(hook_path).unwrap();
    use std::os::unix::fs::PermissionsExt;
    assert!(
        meta.permissions().mode() & 0o111 != 0,
        "L8 hook is not executable: {}", hook_path
    );
}

#[test]
fn l5_worktree_create_hook_exists() {
    let hook_path = ".claude/hooks/create_worktree.sh";
    assert!(
        std::path::Path::new(hook_path).exists(),
        "L5 mitigation hook missing: {}", hook_path
    );
    let meta = std::fs::metadata(hook_path).unwrap();
    use std::os::unix::fs::PermissionsExt;
    assert!(
        meta.permissions().mode() & 0o111 != 0,
        "L5 hook is not executable: {}", hook_path
    );
}

#[test]
fn l8_gitignore_covers_evidence_dev_dirs() {
    let gitignore = std::fs::read_to_string(".gitignore")
        .expect(".gitignore must exist");
    assert!(
        gitignore.contains("handover/evidence/dev_"),
        ".gitignore must exclude handover/evidence/dev_* sidecar dirs"
    );
}

#[test]
fn l8_gitignore_covers_claude_worktrees() {
    let gitignore = std::fs::read_to_string(".gitignore")
        .expect(".gitignore must exist");
    assert!(
        gitignore.contains(".claude/worktrees/"),
        ".gitignore must exclude .claude/worktrees/ per Anthropic official docs"
    );
}

#[test]
fn l5_settings_json_wires_worktree_create_hook() {
    let settings_raw = std::fs::read_to_string(".claude/settings.json")
        .expect(".claude/settings.json must exist");
    let settings: serde_json::Value = serde_json::from_str(&settings_raw)
        .expect("settings.json must be valid JSON");
    
    let has_hook = settings
        .get("hooks")
        .and_then(|h| h.get("WorktreeCreate"))
        .map(|v| !v.as_array().unwrap_or(&vec![]).is_empty())
        .unwrap_or(false);
    
    assert!(has_hook, "settings.json must wire WorktreeCreate hook for L5 mitigation");
}

#[test]
fn l8_settings_json_wires_git_add_hook() {
    let settings_raw = std::fs::read_to_string(".claude/settings.json")
        .expect(".claude/settings.json must exist");
    let settings: serde_json::Value = serde_json::from_str(&settings_raw)
        .expect("settings.json must be valid JSON");
    
    let pre_tool_use = settings
        .get("hooks")
        .and_then(|h| h.get("PreToolUse"))
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    
    let has_bash_git_add_hook = pre_tool_use.iter().any(|entry| {
        let matcher = entry.get("matcher")
            .and_then(|m| m.as_str())
            .unwrap_or("");
        matcher.contains("Bash") || matcher == "Bash"
    });
    
    assert!(has_bash_git_add_hook, "settings.json must include a PreToolUse Bash hook for L8 mitigation");
}

#[test]
fn l7_pre_commit_hook_exists() {
    let hook_path = ".git/hooks/pre-commit";
    if !std::path::Path::new(hook_path).exists() {
        // This is a warning, not a failure — .git/hooks are not committed
        eprintln!("WARNING: .git/hooks/pre-commit not installed — L7 pre-commit check disabled");
        return;
    }
    let content = std::fs::read_to_string(hook_path).unwrap();
    assert!(
        content.contains("handover/evidence"),
        ".git/hooks/pre-commit should check for evidence sidecar files"
    );
}
```

### 6.6 Subagent Prompt Template Additions

**Add to every parallel subagent's system prompt that performs git operations:**

```markdown
## GIT DISCIPLINE (MANDATORY — NO EXCEPTIONS)

### Staging files
FORBIDDEN: git add .
FORBIDDEN: git add -A
FORBIDDEN: git add --all
FORBIDDEN: git add -u

REQUIRED: git add <explicit-file-1> <explicit-file-2> ...
Before staging: run `git status` and read every file listed. Stage ONLY files that are part of your assigned task.

### Evidence sidecar files
Running `cargo test` may generate files in `handover/evidence/dev_*/`. 
These files MUST NOT be staged or committed. They are not part of your task.
If `git status` shows files in `handover/evidence/`, do NOT stage them.

### PR creation verification (MANDATORY)
After `gh pr create`, run:
```
PR_NUMBER=$(gh pr create ... | grep -oP '(?<=/pull/)\d+')
gh pr view "$PR_NUMBER" --json number,headRefName,headRefSha
git rev-parse HEAD
git rev-parse --abbrev-ref HEAD
```
Compare the values. If PR branch or SHA does not match local branch or HEAD, report FAILURE immediately. Do NOT report success if you cannot verify.

### Final report MUST include:
BRANCH: <your branch name>
HEAD_SHA: <git rev-parse HEAD output>
PR_NUMBER: <number or NONE>
PR_URL: <url or NONE>
VERIFICATION: PASS|FAIL|SKIPPED
```

---

## 7. Findings on Unresolved Questions

### 7.1 Does `isolation: "worktree"` serialize worktree creation?

[NOT CONFIRMED] The search did not find official Anthropic documentation confirming whether Claude Code serializes concurrent `git worktree add` calls internally. Issue #34645 (lock contention) is closed but the fix status is unclear. **Assume it does NOT serialize** and implement `WorktreeCreate` hook which gives you control.

### 7.2 Is there a `--clean-untracked` flag?

[CONFIRMED ABSENT] No such flag exists in Claude Code or git worktree. The worktree starts clean because it is a fresh checkout, but stale branches (from prior runs) may have untracked files that were not cleaned up. The `WorktreeCreate` hook running `git clean -fdx` after creation is the correct mitigation.

### 7.3 Does `worktree.baseRef: "fresh"` fix L5?

[PARTIALLY — ANTHROPIC-OFFICIAL] Setting `baseRef: "fresh"` makes worktrees branch from `origin/HEAD` (the remote default). This fixes the wrong-commit bug (#43535) for workflows where `origin/HEAD` points to `main`. It does NOT fix stale branch reuse (#51596) or lock contention (#34645). It is a necessary but not sufficient fix.

### 7.4 Can `refs/worktree/*` provide stronger per-worktree isolation?

[CONFIRMED by git-scm.com — SPECULATION on utility] `refs/worktree/*` refs are per-worktree (not shared). They could theoretically be used to store per-worktree state without polluting global refs. However, this is not commonly used and would require custom tooling. Not a near-term recommendation.

---

## 8. Flags and Skepticism Notes

### 8.1 Possible Prompt Injection in Search Results

No clear prompt injection was detected in search results. All sources cited are either official documentation (Anthropic, git-scm.com) or identifiable repositories/articles. Community blog posts are labeled [COMMUNITY-PATTERN]. 

The zylos.ai and penligent.ai sources are not well-known organizations — their content was used for the "failure modes" section but labeled [COMMUNITY-PATTERN]. Do not treat them as authoritative without independent verification.

### 8.2 Issue Tracking Uncertainty

Several GitHub issues were labeled "stale" or "closed" without a confirmed fix from Anthropic. "Closed" does not mean "fixed" — in GitHub issue tracker, "closed" can mean "fixed", "won't fix", "duplicate", or "stale". Issues #51596 and #33045 are particularly concerning because they are recent (2026) and described as "closed" without clear resolution.

### 8.3 Claude Code Version Sensitivity

Many of the issues are version-specific (e.g., #34645 "regression in v2.1.76+"). The behavior may differ in the version of Claude Code running in this project. Verify which Claude Code version is in use and cross-reference against issue timelines before assuming bugs are present or fixed.

### 8.4 The "silent" in "silently ignored"

Issue #33045 claims `isolation: "worktree"` is silently ignored for team agents. If this is still true in the current version, then ALL the other mitigations for L5 that assume worktrees are created are moot for team-agent dispatch paths. Verify this explicitly by checking `git worktree list` after a dispatch.

---

## 9. Summary Table: Root Causes and Mitigations

| Failure Mode | Root Cause | Confirmed? | Mitigation | Effort |
|---|---|---|---|---|
| L5: stale branch reuse | agentId 8-hex prefix collision | Yes (#51596) | WorktreeCreate hook with timestamp branch name | Low |
| L5: wrong base commit | git worktree add without explicit start point | Yes (#43535) | WorktreeCreate hook uses `origin/main` explicit | Low |
| L5: lock contention | concurrent git worktree add on .git/config.lock | Yes (#34645) | `--no-track` reduces writes; serialize dispatch | Medium |
| L5: isolation silently ignored | team agent dispatch path bug | Yes (#33045) | verify `git worktree list` post-dispatch | Low |
| L5: worktree cleanup destroys .git | path confusion in cleanup | Yes (#48927) | never auto-cleanup; use explicit `git worktree remove` | Low |
| L7: PR number hallucination | agent reads own prior context | Suspected | post-create `gh pr view` verification | Low |
| L7: push failed silently | git push error not checked | Suspected | verify exit code + remote state | Low |
| L8: git add . includes sidecar files | Haiku instruction following | Yes (observed) | PreToolUse hook blocks git add . | Low |
| L8: cargo test generates evidence files | side effect of test runner | Yes (observed) | .gitignore + pre-commit hook + prompt | Low |

---

## 10. References

### Official Sources

- [Run parallel sessions with worktrees — Claude Code Docs](https://code.claude.com/docs/en/worktrees)
- [Create custom subagents — Claude Code Docs](https://code.claude.com/docs/en/sub-agents)
- [Subagents in the SDK — Claude Agent SDK Docs](https://code.claude.com/docs/en/agent-sdk/subagents)
- [Automate workflows with hooks — Claude Code Docs](https://code.claude.com/docs/en/hooks)
- [git-worktree Documentation — git-scm.com](https://git-scm.com/docs/git-worktree)

### Confirmed GitHub Issues

- [Issue #51596: Agent tool isolation worktree silently reuses stale branches](https://github.com/anthropics/claude-code/issues/51596)
- [Issue #48927: Parallel subagent worktree cleanup destroys .git directory](https://github.com/anthropics/claude-code/issues/48927)
- [Issue #43535: isolation worktree creates worktrees from wrong commit](https://github.com/anthropics/claude-code/issues/43535)
- [Issue #34645: Parallel subagents with worktree isolation fail due to git config lock contention](https://github.com/anthropics/claude-code/issues/34645)
- [Issue #33045: Agent tool isolation worktree has no effect for team agents](https://github.com/anthropics/claude-code/issues/33045)

### Community Sources (COMMUNITY-PATTERN label)

- [Augment Code: Git Worktrees for Parallel AI Agent Execution](https://www.augmentcode.com/guides/git-worktrees-parallel-ai-agent-execution)
- [Zylos Research: Git Worktree Isolation Patterns](https://zylos.ai/research/2026-02-22-git-worktree-parallel-ai-development)
- [Penligent: Git Worktrees Need Runtime Isolation](https://www.penligent.ai/hackinglabs/git-worktrees-need-runtime-isolation-for-parallel-ai-agent-development/)
- [ccswarm — nwiizo](https://github.com/nwiizo/ccswarm)
- [GitButler: Agentic Safety](https://blog.gitbutler.com/agentic-safety)
- [4 Claude Code Subagent Mistakes — DEV Community](https://dev.to/alireza_rezvani/4-claude-code-subagent-mistakes-that-kill-your-workflow-and-the-fixes-3n72)
- [Simon Willison: Using Git with Coding Agents](https://simonwillison.net/guides/agentic-engineering-patterns/using-git-with-coding-agents/)
- [Docker Sandboxes — Docker Blog](https://www.docker.com/blog/docker-sandboxes-run-agents-in-yolo-mode-safely/)
- [AI Agent Sandboxing — SoftwareSeni](https://www.softwareseni.com/ai-agent-sandboxing-explained-why-docker-is-not-enough-and-what-actually-works/)
- [SWE-MiniSandbox — arXiv 2602.11210](https://arxiv.org/pdf/2602.11210)

### GitHub Actions for PR Validation

- [amannn/action-semantic-pull-request](https://github.com/amannn/action-semantic-pull-request)
- [PR commits and diff validation — GitHub Marketplace](https://github.com/marketplace/actions/pr-commits-and-diff-validation)
- [Conventional Commit Checker — GitHub Marketplace](https://github.com/marketplace/actions/conventional-commit-checker)
