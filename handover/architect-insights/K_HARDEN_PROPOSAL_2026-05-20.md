# K-HARDEN: Multi-Agent Flash Harness Hardening Proposal

**Date**: 2026-05-20
**Trigger**: User request post Karpathy v3 plan completion: "research 如何尽可能加强这个问题的 harness" — referring to L5/L7/L8 lessons that required Claude main supervision.
**Research basis**: `/tmp/multi_agent_isolation_research.md` (700 lines, sonnet-model research over 10+ WebSearches + Anthropic docs / GitHub issues / git-scm.com).
**Goal**: Eliminate or drastically reduce the need for Claude-main supervision of flash-multi-agent execution.

---

## 1. Root cause summary (confirmed by research)

### L5 — branch entanglement
**NOT prompt-side error** — Anthropic has 5 confirmed GitHub issues on `isolation: "worktree"`:
- **#51596** (closed-stale, April 2026): 8-hex agentId prefix collision → stale branch reuse (silent contamination)
- **#43535** (closed dup): wrong base commit pick when not `main`
- **#34645** (closed): `.git/config.lock` race on concurrent `git worktree add`
- **#33045** (open): isolation silently ignored for team-agent path
- **#48927** (open, data-loss): cleanup deletes `.git/`

Mitigation path is **explicit `WorktreeCreate` hook** that takes control of git plumbing with collision-resistant naming + `--lock` + `--no-track` + explicit `origin/main` start point + post-create `git clean -fdx`.

### L7 — report-vs-reality divergence
**No Anthropic official guidance exists**. Subagents report success that didn't happen because:
- `gh pr create` ran on wrong branch (L5 cascade)
- push silently rejected, agent didn't verify return code
- agent hallucinated confirmation from context

Mitigation path is **mandatory POST-CREATE predicate** that subagent runs `gh pr view --json headRefName,headRefSha` + compares against `git rev-parse HEAD` + branch.

### L8 — dirty-tree pickup
**Confirmed root cause**: Haiku-class models default to `git add .` despite explicit prompts saying "git add file1 file2". `cargo test` (and similar) generates sidecar evidence in `handover/evidence/dev_*/`; haiku stages it inadvertently.

Mitigation path is **`PreToolUse` hook (Anthropic-official mechanism)** that intercepts `Bash(git add *)` and **hard-blocks** any form using `.`, `-A`, `--all`, `-u`. Exit code 2 with `permissionDecision: deny` JSON output forces specific file paths.

---

## 2. Concrete deliverable design — 5 K-HARDEN atoms

Each atom Class 0 or 1 (additive, no STEP_B touch). Total estimated effort: ~6-8 hours.

| Atom | Scope | Risk | Effort | Fix surface |
|------|-------|------|--------|-------------|
| **K-HARDEN-1** | WorktreeCreate hook + `.gitignore` | Class-1 | 1-2h | L5 |
| **K-HARDEN-2** | PreToolUse `git add` validator hook + git pre-commit hook installer | Class-1 | 1-2h | L8 |
| **K-HARDEN-3** | SUBAGENT_HARNESS.md skill + dispatch_subagent.sh orchestrator wrapper | Class-0 | 2-3h | L7 + dispatch hygiene |
| **K-HARDEN-4** | `tests/constitution_subagent_pr_hygiene.rs` gate ensuring hooks stay wired | Class-1 | 1h | meta-safety |
| **K-HARDEN-5** (optional) | `.github/workflows/validate-agent-pr.yml` server-side PR diff validation | Class-1 | 30min | defense-in-depth |

---

## 3. K-HARDEN-1: WorktreeCreate hook

**Files**:
- `.claude/hooks/create_worktree.sh` (NEW, ~30 LOC)
- `.gitignore` (modify, add 2 lines)
- `.claude/settings.json` (modify, wire hook + set `worktree.baseRef: fresh`)

**Mechanism** (per research §6.1):
1. Generate collision-resistant branch name: `worktree-agent-${TIMESTAMP}-${RAND4}`
2. `git fetch origin main` first
3. `git worktree add --lock --no-track -b "$BRANCH" "$WT_DIR" origin/main`
4. `git clean -fdx` inside new worktree (paranoia)
5. Verify `rev-parse --abbrev-ref HEAD == $BRANCH`

**Predicate verification** (acceptance):
- `test -x .claude/hooks/create_worktree.sh`
- `grep -q "handover/evidence/dev_" .gitignore`
- `grep -q "\.claude/worktrees/" .gitignore`
- `grep -q "WorktreeCreate" .claude/settings.json`
- New subagent dispatch produces worktree with unique timestamp-based branch name (sample verify)

---

## 4. K-HARDEN-2: PreToolUse git-add validator + pre-commit hook

**Files**:
- `.claude/hooks/validate_git_add.sh` (NEW, ~25 LOC)
- `.claude/settings.json` (modify, wire PreToolUse hook)
- `scripts/install_git_hooks.sh` (NEW, ~15 LOC — installs `.git/hooks/pre-commit`)
- `scripts/git_hooks/pre-commit` (NEW source, ~10 LOC — pre-commit hook content)

**Hook mechanism**:
- Parse `tool_input.command` from JSON stdin
- If matches `git add (\.|(-A|--all|-u)\b)`: emit `permissionDecision: deny` JSON, exit 0
- Otherwise: exit 0 (allow)

**Pre-commit hook** (defense-in-depth at git level for human commits too):
```bash
STAGED=$(git diff --cached --name-only)
echo "$STAGED" | grep -q '^handover/evidence/dev_' && {
  echo "ERROR: staged sidecar evidence files"; exit 1; }
```

**Predicate verification**:
- `test -x .claude/hooks/validate_git_add.sh`
- `grep -q "validate_git_add.sh" .claude/settings.json`
- `test -x scripts/install_git_hooks.sh`
- Run `bash scripts/install_git_hooks.sh && test -x .git/hooks/pre-commit`
- Simulate: `echo '{"tool_input":{"command":"git add ."}}' | bash .claude/hooks/validate_git_add.sh | jq -r '.hookSpecificOutput.permissionDecision'` == "deny"
- Simulate: `echo '{"tool_input":{"command":"git add src/foo.rs"}}' | bash .claude/hooks/validate_git_add.sh` exits 0 with no deny

---

## 5. K-HARDEN-3: SUBAGENT_HARNESS.md skill + dispatch wrapper

**Files**:
- `skills/SUBAGENT_HARNESS.md` (NEW, ~80 LOC) — canonical prompt template
- `scripts/dispatch_subagent.sh` (NEW, ~40 LOC) — orchestrator helper

**Skill content** (`SUBAGENT_HARNESS.md`):
- Mandatory PRELUDE (pwd guard + branch base from origin/main)
- Mandatory MIDFLIGHT (`git add SPECIFIC_FILE_LIST` only — never `.`/`-A`)
- Mandatory POSTLUDE (PR verification predicate with SHA + branch comparison)
- Standard report fields: BRANCH / HEAD_SHA / PR_NUMBER / PR_URL / VERIFICATION

**Dispatch script** (`scripts/dispatch_subagent.sh`):
- Pre-flight: assert worktree clean + no stash entries
- Record START_SHA / START_BRANCH
- (Placeholder: invoke `claude -p` or equivalent; current Claude Code uses Agent tool, this script is for future external orchestrators)
- Post-execution: assert no `handover/evidence/dev_` files in diff
- Print structured result

**Predicate verification**:
- `test -f skills/SUBAGENT_HARNESS.md`
- `grep -q "POSTLUDE" skills/SUBAGENT_HARNESS.md`
- `grep -q "headRefSha" skills/SUBAGENT_HARNESS.md`
- `test -x scripts/dispatch_subagent.sh`
- `bash scripts/dispatch_subagent.sh --help` (or smoke test pre-flight section)

---

## 6. K-HARDEN-4: constitution_subagent_pr_hygiene.rs gate

**File**: `tests/constitution_subagent_pr_hygiene.rs` (NEW, ~100 LOC)

**Tests** (each #[test]):
1. `l5_worktree_create_hook_exists` — `.claude/hooks/create_worktree.sh` is executable
2. `l5_gitignore_blocks_dev_evidence` — `.gitignore` contains `handover/evidence/dev_*/`
3. `l5_gitignore_blocks_worktrees_dir` — `.gitignore` contains `.claude/worktrees/`
4. `l8_git_add_hook_exists` — `.claude/hooks/validate_git_add.sh` is executable
5. `l8_pre_commit_installer_exists` — `scripts/install_git_hooks.sh` is executable
6. `l7_subagent_harness_skill_exists` — `skills/SUBAGENT_HARNESS.md` contains POSTLUDE + headRefSha references
7. `l7_dispatch_script_exists` — `scripts/dispatch_subagent.sh` is executable
8. `settings_wires_all_hooks` — `.claude/settings.json` contains both WorktreeCreate and PreToolUse hook registrations

**Manifest + matrix**:
- Add manifest entry with `authority = "K-HARDEN-4 (L5+L7+L8 hardening verification)"`
- Add to BASELINE_ALLOWLIST or matrix (likely allowlist since meta-test)

---

## 7. K-HARDEN-5 (optional): GitHub Action server-side validation

**File**: `.github/workflows/validate-agent-pr.yml` (NEW, ~30 LOC)

**Mechanism**: On every PR open/sync:
- Get PR file list via `gh pr view`
- If any `handover/evidence/dev_*/` file present → fail
- If PR title starts with `karpathy/` or `adversarial-` (agent-created) → require explicit allowlist label OR ≤ 5 files

**Predicate verification**:
- `.github/workflows/validate-agent-pr.yml` exists
- File contains `evidence/dev_` grep check
- (Server-side firing tested by creating a deliberate-contamination PR in a test branch)

---

## 8. Sequencing recommendation

```
Phase A (sequential, can be self-only since Claude main does it):
  1. K-HARDEN-1 (worktree hook)
  2. K-HARDEN-2 (git-add hook)
  3. Verify L5/L8 fixes by dispatching 1 haiku subagent and observing improved behavior

Phase B (additive):
  4. K-HARDEN-3 (skill + dispatch script)
  5. K-HARDEN-4 (constitution gate)

Phase C (optional, server-side):
  6. K-HARDEN-5 (GitHub Action)
```

After Phase A: re-run the 4-haiku adversarial test (Tasks A/B/E/F equivalent). If contamination + branch entanglement no longer occur, declare K-HARDEN ship-ready.

---

## 9. Expected outcome

**Before K-HARDEN** (current state):
- L5 branch entanglement: 100% incidence on 4-parallel haiku (entire adversarial set)
- L7 report divergence: ~50% incidence (Task B reported wrong PR)
- L8 dirty-tree: ~75% incidence (3 of 4 adversarial PRs contaminated)
- Claude-main intervention required: yes, untangle cherry-pick

**After K-HARDEN Phase A** (worktree hook + git-add hook):
- L5: bug surface largely closed by `--lock` + unique branch names + `--no-track`
- L7: still requires prompt-level POSTLUDE (K-HARDEN-3) — not yet fixed by Phase A
- L8: hard-blocked by PreToolUse hook (Anthropic-official mechanism)
- Claude-main intervention: required for L7 only

**After K-HARDEN Phase B** (+ skill + dispatch + gate):
- L7: subagent prompt template enforces POSTLUDE verification
- Claude-main intervention: minimized to escalation cases (e.g., hook bug)

**After K-HARDEN Phase C** (+ GitHub Action):
- Even if agent + Claude main both miss something, server-side CI catches before merge.

---

## 10. Open questions for user before implementation

1. **Repo hooks vs Claude hooks scope**: K-HARDEN-2 includes a `.git/hooks/pre-commit` installer. This is a per-clone hook (not committed). Some teams reject git hooks because of installation friction. **Should I include the install script, or rely on `.claude/hooks/` only?**

2. **GitHub Action**: K-HARDEN-5 is server-side. **Worth doing now or defer until first post-hardening contamination incident?**

3. **Hook severity**: K-HARDEN-2 hard-denies `git add .`. **Some legitimate flows (e.g., initial commit of new module dir) might be inconvenienced. Acceptable trade-off?**

4. **Scheduling**: User goal said "尽可能加强" (maximize hardening). **Proceed with all 5 atoms, or just Phase A (highest ROI) first?**

5. **Test of fix**: After K-HARDEN ship, **do you want a fresh adversarial run (4 new haiku tasks) to validate? This would burn ~30 min + tokens but confirms L5/L8 elimination.**

---

## 11. Citations

Full research report at `/tmp/multi_agent_isolation_research.md`. Sources:
- Anthropic Claude Code docs (worktrees + hooks)
- GitHub issues #51596 #43535 #34645 #33045 #48927
- git-scm.com worktree docs
- Industry patterns: ccswarm (nwiizo), SWE-agent paper, Cursor 2.0, Windsurf 2.0
- IBM Research microVM cold-start benchmarks
- Simon Willison "Agent Committed Wrong Files" pattern guide
