# Subagent Harness — Standard Dispatch Template

K-HARDEN-3 (2026-05-20) — canonical pattern for dispatching flash-multi-agent
Claude Code subagents safely. Closes lesson L5/L7/L8 from Karpathy v3 plan
adversarial validation.

## Background

When Claude main dispatches multiple haiku-class subagents in parallel via the
Agent tool with `isolation: "worktree"`, three failure modes are confirmed:

- **L5 branch entanglement** — subagents share `.git/refs/` and end up
  stacking commits on each other's branches. Anthropic upstream bugs
  #51596 #43535 #34645 #33045 #48927. **Fix at infrastructure layer**:
  K-HARDEN-1 `WorktreeCreate` hook (`.claude/hooks/create_worktree.sh`).
- **L7 report-vs-reality divergence** — subagent claims "PR #N created"
  but actual remote state shows different PR or no commit pushed.
  **Fix at prompt layer**: this skill (mandatory POSTLUDE verification).
- **L8 dirty-tree pickup** — subagent uses `git add .` / `-A` despite
  explicit prompt instructions, sweeping sidecar evidence files into commit.
  **Fix at hook layer**: K-HARDEN-2 `.claude/hooks/validate_git_add.sh`
  (hard-denies wildcard staging via permissionDecision=deny).
- **L9 push-to-main bypass** — subagent accidentally exits its isolation
  worktree, commits + pushes from main worktree, bypassing PR review.
  Surfaced during K-HARDEN validation run (2026-05-20, commit ccf7a38c).
  **Fix at hook layer**: K-HARDEN-6 `.claude/hooks/validate_git_push.sh`
  hard-denies `git push origin main` / `--all` / push-when-on-main. Legitimate
  bypass via `GIT_HARDEN_ALLOW_MAIN=1` env var (audited via shell history).

This skill provides the prompt-template patterns L7 needs. L5 + L8 are
already enforced by hooks; this skill exists primarily to (a) document the
PR self-verification step and (b) give Claude main a canonical prompt
prelude/postlude block to reuse.

## Mandatory PRELUDE (every subagent prompt's top section)

Every subagent prompt MUST start with these guards. Failure to follow means
the subagent's work may end up on the wrong branch or with contamination.

```text
## MANDATORY PRELUDE — DO NOT SKIP

1. Verify you are in an isolation worktree:
   pwd | grep -q "\.claude/worktrees/" || { echo "FATAL: not in isolation worktree (pwd: $(pwd))"; exit 1; }
   echo "[isolation OK] worktree: $(pwd)"

2. Branch base from origin/main (NEVER local main; L4 fix):
   git fetch origin main
   git checkout -b "<UNIQUE-BRANCH-NAME>" origin/main
   echo "[branch base OK] HEAD: $(git rev-parse HEAD)"

3. Confirm clean working tree (defense against L5 entanglement):
   [ -z "$(git status --porcelain)" ] || {
     echo "FATAL: worktree starts dirty:"
     git status --porcelain
     exit 1
   }
```

The `<UNIQUE-BRANCH-NAME>` should follow `<atom-id>-<short-purpose>` form,
e.g. `harden/k-harden-3-subagent-skill` or `adversarial-task-a-new-gate`.
Avoid generic names that risk collision.

## Mandatory MIDFLIGHT (during subagent work)

- **Never `git add .` / `-A` / `--all`**. The K-HARDEN-2 hook will hard-deny.
  Always stage explicit file paths: `git add src/foo.rs tests/bar.rs`.
- **Before commit, verify staged file list matches expected**:
  `git diff --cached --name-only | sort` should equal the file list named
  in the task's Allowed paths.

## Mandatory POSTLUDE (every PR-creating subagent's bottom section)

After `gh pr create`, the subagent MUST run this PR-self-verification block:

```bash
# Capture PR number from gh output
CAPTURE_PR=$(gh pr create --base main --head "$BRANCH" --title "<title>" --body "<body>" 2>&1)
PR_NUMBER=$(echo "$CAPTURE_PR" | grep -oP '(?<=/pull/)\d+' | head -1)

if [ -z "$PR_NUMBER" ]; then
  echo "FATAL: gh pr create did not return a PR number"
  echo "$CAPTURE_PR"
  exit 1
fi

# Verify PR points to OUR branch + commit
ACTUAL=$(gh pr view "$PR_NUMBER" --json headRefName,headRefOid)
ACTUAL_BRANCH=$(echo "$ACTUAL" | jq -r '.headRefName')
ACTUAL_SHA=$(echo "$ACTUAL" | jq -r '.headRefOid')
LOCAL_SHA=$(git rev-parse HEAD)
LOCAL_BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [ "$ACTUAL_BRANCH" != "$LOCAL_BRANCH" ]; then
  echo "FATAL: PR #$PR_NUMBER branch mismatch — PR=$ACTUAL_BRANCH local=$LOCAL_BRANCH"
  exit 1
fi
if [ "$ACTUAL_SHA" != "$LOCAL_SHA" ]; then
  echo "FATAL: PR #$PR_NUMBER SHA mismatch — PR=$ACTUAL_SHA local=$LOCAL_SHA"
  exit 1
fi

# Verify no contamination in PR diff
DIFF_FILES=$(gh pr view "$PR_NUMBER" --json files --jq '.files[].path')
if echo "$DIFF_FILES" | grep -qE '^handover/evidence/dev_self_hosting/dev_|^\.claude/worktrees/'; then
  echo "FATAL: PR #$PR_NUMBER contains forbidden sidecar paths:"
  echo "$DIFF_FILES" | grep -E '^handover/evidence/dev_self_hosting/dev_|^\.claude/worktrees/'
  exit 1
fi

echo "VERIFIED: PR #$PR_NUMBER on $ACTUAL_BRANCH@$ACTUAL_SHA, no contamination"
```

## Mandatory FINAL REPORT FORMAT

The subagent's final report MUST include these 5 fields literally:

```text
BRANCH: <branch-name>
HEAD_SHA: <git rev-parse HEAD>
PR_NUMBER: <N>
PR_URL: <url>
VERIFICATION: PASS
```

Claude main verifies independently by running `gh pr view $PR_NUMBER` and
comparing. Mismatch → reject the subagent result, do not merge.

## Forbidden paths (every subagent prompt)

In addition to v3 plan §2 STEP_B red lines, all subagents must NOT touch:

- `src/kernel.rs`, `src/bus.rs`, `src/state/sequencer.rs`, `src/state/typed_tx.rs`,
  `src/state/transition_ledger.rs`, `src/state/q_state.rs`,
  `src/state/head_t_witness.rs`, `src/state/monetary_invariant.rs`,
  `src/bottom_white/cas/schema.rs`, `src/bottom_white/cas/store.rs`,
  `src/runtime/run_summary.rs`, `src/runtime/verify.rs`, `constitution.md`
- `rules/engine.py`, `rules/active/`, `.claude/hooks/judge.sh`
- The 4 binding test files referenced by constitution gates:
  `tests/constitution_fc1_runtime_loop.rs`, `tests/constitution_fc2_boot.rs`,
  `tests/constitution_no_evidence_drift_in_tests.rs`,
  `tests/constitution_predicate_gate.rs`

## When to use this skill

- Dispatching haiku-class subagents in parallel
- Dispatching ANY subagent that creates a PR
- Designing new adversarial harness tests

## When NOT to use this skill

- Diagnostic-only / read-only subagents (no PR created)
- Tasks where Claude main does the work directly (no Agent tool dispatch)
- Subagents on independent (non-shared) git remotes

## Related skills

- `runner-preflight` — pre-action gate for runner scripts mutating evidence/
- `constitution-landing-check` — pre-TB-charter gate
- `harness-reflect` — post-TB-ship retrospective

## Reference docs

- `handover/architect-insights/MULTI_AGENT_ISOLATION_RESEARCH_2026-05-20.md`
  (954-line research on industry patterns + Anthropic bug status)
- `handover/architect-insights/K_HARDEN_PROPOSAL_2026-05-20.md`
  (233-line K-HARDEN-1..5 atom design)
