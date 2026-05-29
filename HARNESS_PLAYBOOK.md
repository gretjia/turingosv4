# Harness Playbook — Portable Bootstrap for Constitutional Multi-Agent Repos

**Version**: 1.0 (2026-05-20)
**Source repo**: gretjia/turingosv4
**Audience**: future Claude / Codex / Gemini agents + the human orchestrator,
who want to bootstrap this discipline in a new repository.

This is the **canonical reusable manual**. Drop it into a new repo's root,
follow Part IV to install, then Part V to operate. Adapt the project-specific
content (constitution.md, sequencer surfaces, etc.) — the harness machinery
itself is project-agnostic.

---

## Table of contents

- [Part I — Concept](#part-i--concept)
  - [I.1 Why this harness exists](#i1-why-this-harness-exists)
  - [I.2 Core illusion (the Q_t framework)](#i2-core-illusion-the-q_t-framework)
  - [I.3 Truth order — 3 tiers](#i3-truth-order--3-tiers)
  - [I.4 Audit boundary — predicates vs witness](#i4-audit-boundary--predicates-vs-witness)
- [Part II — Architecture](#part-ii--architecture)
  - [II.5 Constitution gates](#ii5-constitution-gates)
  - [II.6 Manifest + matrix drift](#ii6-manifest--matrix-drift)
  - [II.7 Three-layer push-to-main defense](#ii7-three-layer-push-to-main-defense)
- [Part III — Multi-agent operations](#part-iii--multi-agent-operations)
  - [III.8 Subagent dispatch model](#iii8-subagent-dispatch-model)
  - [III.9 Lesson library L1-L9](#iii9-lesson-library-l1-l9)
  - [III.10 K-HARDEN atom catalog](#iii10-k-harden-atom-catalog)
  - [III.11 PR-only workflow (cross-agent)](#iii11-pr-only-workflow-cross-agent)
  - [III.12 Orchestrator merge protocol](#iii12-orchestrator-merge-protocol)
- [Part IV — Bootstrap a new project](#part-iv--bootstrap-a-new-project)
  - [IV.13 Skeleton to copy](#iv13-skeleton-to-copy)
  - [IV.14 Adapt the gates](#iv14-adapt-the-gates)
  - [IV.15 Enable GitHub branch protection](#iv15-enable-github-branch-protection)
  - [IV.16 Install local hooks](#iv16-install-local-hooks)
  - [IV.17 Wire the SUBAGENT_HARNESS skill](#iv17-wire-the-subagent_harness-skill)
- [Part V — Operating procedures](#part-v--operating-procedures)
  - [V.18 Daily workflow (orchestrator)](#v18-daily-workflow-orchestrator)
  - [V.19 New-atom checklist](#v19-new-atom-checklist)
  - [V.20 Adversarial validation cadence](#v20-adversarial-validation-cadence)
  - [V.21 Recovery from contamination](#v21-recovery-from-contamination)
- [Appendix](#appendix)
  - [A. L1-L9 reference card](#a-l1-l9-reference-card)
  - [B. Hook contracts](#b-hook-contracts)
  - [C. Script catalog](#c-script-catalog)
  - [D. Constitution gate template](#d-constitution-gate-template)

---

## Part I — Concept

### I.1 Why this harness exists

A constitutional multi-agent repo has two competing pressures:

1. **Velocity** — flash-class agents (Claude Haiku / Codex CLI / Gemini CLI)
   parallelize work cheaply, but they hallucinate, ignore prompts, contaminate
   commits, and stack on each other's branches.
2. **Discipline** — high-stakes systems (governance, money, formal proofs)
   demand append-only tape, mechanical predicates, and audit trails.

The harness binds these two pressures together. It lets flash agents do the
mechanical work safely while a Claude-main orchestrator (or human) supervises
ratification + merge. Every safety net is **enforced mechanically**, not via
prompt instruction alone.

### I.2 Core illusion (the Q_t framework)

The repo is physically a **state-transition machine on `Q_t = ⟨q_t, HEAD_t, tape_t⟩`**:

- `q_t` — internal state (EconomicState / agent registry / etc.)
- `HEAD_t` — version-controlled head pointer (a 6-field witness: state_root +
  l4_head + l4e_head + cas_root + economic_state_root + run_id)
- `tape_t` — ChainTape file content (append-only events + L4.E rejections + CAS)

This frames every change as a transition `Q_t → Q_{t+1}` mediated by predicates.
The harness's job is to make sure every transition is:
- Reconstructible from append-only facts (tape + CAS)
- Predicate-checked at the boundary (constitution gates)
- Witnessed by independent audit when high-stakes

### I.3 Truth order — 3 tiers

Truth in the repo flows top-down across **3 flat tiers** (per K-2.2 receipt-driven analysis):

| Tier | Content | Mutability |
|------|---------|-----------|
| 1. **Axioms** | `constitution.md` + 3 canonical flowchart hashes | Slow-moving; Class-4 §8 to change |
| 2. **Facts** | ChainTape + CAS + replay/audit verifier | Append-only; never rewritten |
| 3. **Workspace pointers** | Current TB charter + `handover/ai-direct/LATEST.md` | Mutable but explicit derived view |

**Everything else is a derived view** — matrix, trace_matrix, TB_LOG,
dashboards, reports. If a derived view contradicts ChainTape, trust ChainTape.
If ChainTape contradicts constitution, stop.

### I.4 Audit boundary — predicates vs witness

The harness has a **double-tiered judgment structure** (K-HARDEN §1a):

**Hard judges (predicates layer, machine-deterministic, 0/1 output)**:
- `cargo test --workspace --no-fail-fast` exit code
- `bash scripts/run_constitution_gates.sh` exit code
- `cargo test --test constitution_matrix_drift` exit code
- `cargo test --test 'constitution_rules_*'` exit code
- `rules/engine.py` PreToolUse synchronous block (exit 2 = block)
- Per-atom predicate verification recipes in PR body

**Soft witness (independent audit, can flag but not judge alone)**:
- Clean-context audit by a fresh agent on any capable platform (Claude / Codex / Antigravity / …)
- **Legal output space**: `{NO-VIOLATION, VIOLATION-FOUND, RECONSTRUCTION-FAILURE, SECOND-SOURCE-DRIFT}`
- **Illegal output space** (out-of-scope, can reject report): subjective code
  style / performance / coverage / architecture preference

**Ship gate composition**:
```
ship_OK = (all predicates GREEN)
          AND (audit witness ≠ unresolved violation)
```

The witness role is bounded by the constitutional Veto-AI doctrine: output
domain is `{PASS, VETO}` *only* for constitutional violation, never for taste.

---

## Part II — Architecture

### II.5 Constitution gates

Constitution clauses are encoded as `tests/constitution_*.rs` Rust integration
test files. Each file is its own test crate; each `#[test] fn` is a predicate
binding one constitution invariant.

**Discovery + manifest binding**:

```bash
# tests/constitution_*.rs is auto-discovered by the runner
ls tests/constitution_*.rs | xargs -n1 basename | sed 's/\.rs$//'

# Manifest cross-references each authorized gate
grep -oP '^name = "\K[^"]+' scripts/constitution_gates.manifest.toml
```

The runner (`scripts/run_constitution_gates.sh`, ~50 lines) cross-checks
discovered = manifest before running tests. Drift either direction = fail.

**Per-gate manifest entry format**:

```toml
[[gate]]
name = "constitution_fc1_runtime_loop"
authority = "handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md §4.1"
added = "2026-05-06"
```

The `authority` field must point to a specific directive or charter — not a
placeholder. K-1.5a established the rule: every gate has a binding source.

### II.6 Manifest + matrix drift

`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` is the human-readable
clause-to-gate mapping. K-2.3 introduced a Rust drift gate
(`tests/constitution_matrix_drift.rs`) that:

- Asserts every manifest gate is referenced in the matrix OR in a baseline
  allowlist
- Has a hard `K23_SHIP_ALLOWLIST_SIZE` cap (cannot grow silently)
- Bans `assert!(true)` and similar tautologies in constitution_* tests

The allowlist shrinks as the matrix expands (K-2.3 Task B established the
graduation protocol).

### II.7 Three-layer push-to-main defense

The harness's cross-agent universal block on direct push to main has **three
independent layers** (K-HARDEN-1/2/6/7):

| Layer | Universal scope | Bypass | Enforces |
|-------|----------------|--------|----------|
| **GitHub branch protection** | All agents, all clients | Repo admin only | Server-side rejection |
| **Git pre-push hook** (`scripts/hooks/pre-push.harden`) | Any agent respecting git hooks (Codex/Gemini/Aider/cursor/human) | `--no-verify` or `GIT_HARDEN_ALLOW_MAIN=1` | Client-side rejection before push |
| **Claude PreToolUse hook** (`.claude/hooks/validate_git_push.sh`) | Claude Code only | Switching to other CLI | Earlier-in-loop feedback |

The legitimate orchestrator bypass is **explicit** and audited:

```bash
GIT_HARDEN_ALLOW_MAIN=1 git push origin main
```

Logged in shell history + git reflog. Reserved for: orchestrator merging a
locally-vetted PR + pushing the merge commit.

---

## Part III — Multi-agent operations

### III.8 Subagent dispatch model

The orchestrator (Claude main, or a human) dispatches flash subagents
(haiku-class) for mechanical tasks. Each subagent:

1. Spawns in an **isolation worktree** under `.claude/worktrees/`
2. Has read-only access to repo refs but writes to its own branch
3. Receives a **self-contained prompt** with PRELUDE + task + POSTLUDE
4. Produces a **PR**, never merging directly to main

The orchestrator verifies subagent output independently (trust-but-verify) and
then merges.

### III.9 Lesson library L1-L9

Each lesson was surfaced empirically in adversarial validation. Each has a
mechanical fix.

| L# | Pattern | Empirical cost | Mechanical fix |
|----|---------|---------------|----------------|
| **L1** | Haiku ignores worktree isolation (cd's out, writes to main worktree) | K-1.5 stacked PRs | Mandatory PRELUDE `pwd \| grep .claude/worktrees/` |
| **L2** | Manifest authority placeholder strings | K-1.5 charter-binding loss | K-1.5a backfill with directive paths |
| **L3** | Verification pollutes main worktree (`git checkout other-branch -- file`) | Self-inflicted contamination | Use `git show <branch>:<file>` (no disk write) |
| **L4** | `git checkout -b NEW` from local main (not origin/main) inherits stale state | PR base wrong | `git fetch origin main && git checkout -b NEW origin/main` |
| **L5** | Multi-haiku branch entanglement (shared `.git/refs/` despite worktree) | All 4 Phase 2 PRs entangled | K-HARDEN-1 WorktreeCreate hook + timestamp+random branch names |
| **L6** | K-1.6 report claim of "identical pattern" was approximate; 8 variants found | Pilot scope wrong | Audit per-file pre-substitute |
| **L7** | Haiku reports PR-URL that doesn't match actual remote state | False success claims | K-HARDEN-3 SUBAGENT_HARNESS POSTLUDE: `gh pr view --json headRefOid` vs `git rev-parse HEAD` |
| **L8** | Haiku uses `git add .` / `-A` despite prompt; sweeps sidecar evidence | 18 contaminated files / PR | K-HARDEN-2 `validate_git_add.sh` hard-deny + `.gitignore` block + R-022 pre-commit chain |
| **L9** | Haiku exits isolation worktree, pushes directly to main from main worktree | commit `ccf7a38c` on main without PR | K-HARDEN-6 Claude hook + **K-HARDEN-7 universal git pre-push + GitHub branch protection** |

**The library is append-only**. New lessons get new L# numbers; existing
fixes don't get rewritten.

### III.10 K-HARDEN atom catalog

The 8 K-HARDEN atoms (all merged to main) are the codified mechanical defenses:

| Atom | PR | Touches | Fixes |
|------|----|---------|-------|
| **K-HARDEN-1** | #34 | `.claude/hooks/create_worktree.sh` + `.gitignore` + `.claude/settings.json` | L5 (worktree isolation) |
| **K-HARDEN-2** | #35 | `.claude/hooks/validate_git_add.sh` + extended `scripts/hooks/pre-commit.r022` | L8 (wildcard staging) |
| **K-HARDEN-3** | #36 | `skills/SUBAGENT_HARNESS.md` + `scripts/dispatch_subagent.sh` | L7 (report verification) |
| **K-HARDEN-4** | #37 | `tests/constitution_subagent_pr_hygiene.rs` (now 20 tests) | meta-safety |
| **K-HARDEN-5** | #38 | `.github/workflows/validate-agent-pr.yml` | server-side PR diff scan |
| **K-HARDEN-6** | #39 | `.claude/hooks/validate_git_push.sh` | L9 Claude layer |
| **K-HARDEN-7** | #40 | `scripts/hooks/pre-push.harden` + `scripts/setup_branch_protection.sh` + GitHub API config + AGENTS.md §14a | **L9 universal cross-agent** |
| **K-HARDEN-8** | tbd | 8 CLI cold-start entry files (`GEMINI.md`, `CONVENTIONS.md`, `.aider.conf.yml`, `.cursorrules`, `.cursor/rules/000-agents-alignment.mdc`, `.windsurfrules`, `.github/copilot-instructions.md`, `WARP.md`) + AGENTS.md §2 rewrite | **cross-CLI cold-start alignment** |

### III.10a Cross-CLI cold-start matrix (K-HARDEN-8)

Each CLI has its own discovery convention. K-HARDEN-8 lifts every CLI to read
the same canonical `AGENTS.md` regardless of entry point. **`AGENTS.md` is
the only canonical source; all other entry files are thin pointers that
defer to it.**

| CLI | Cold-start file | Format | K-HARDEN-8 file |
|-----|----------------|--------|-----------------|
| Claude Code | `CLAUDE.md` | Markdown w/ `@AGENTS.md` import | already exists |
| Codex CLI (OpenAI) | `AGENTS.md` | Markdown (canonical) | already exists ✓ |
| Gemini CLI (Google) | `GEMINI.md` | Markdown, hierarchical | `GEMINI.md` |
| Aider | `CONVENTIONS.md` + `.aider.conf.yml` | Markdown + YAML auto-load | `CONVENTIONS.md` + `.aider.conf.yml` |
| Cursor (modern) | `.cursor/rules/*.mdc` | MDC w/ frontmatter | `.cursor/rules/000-agents-alignment.mdc` |
| Cursor (legacy) | `.cursorrules` | plain text | `.cursorrules` (fallback) |
| Windsurf | `.windsurfrules` | plain text | `.windsurfrules` |
| GitHub Copilot | `.github/copilot-instructions.md` | Markdown | same |
| Warp | `WARP.md` | Markdown | `WARP.md` |

**Conflict resolution rule** (in every entry file): if anything contradicts
`AGENTS.md`, `AGENTS.md` wins. This makes Claude (which reads CLAUDE.md →
@AGENTS.md) the de-facto alignment anchor — every other CLI ends up reading
the same AGENTS.md content.

**Test gate**: `tests/constitution_subagent_pr_hygiene.rs` has 4 K-HARDEN-8
tests verifying (a) all 8 entry files exist, (b) all reference `AGENTS.md`,
(c) all (except YAML config) document PR-only workflow, (d) `AGENTS.md` §2
explicitly declares its canonical universal-entry role.

### III.11 PR-only workflow (cross-agent)

**The rule**: every coding agent (Claude / Codex / Gemini / Aider / Cursor /
human / CI bot) **may only create pull requests**. Approval and merge are the
orchestrator's responsibility.

**Why it's enforceable universally**:
- GitHub branch protection rejects direct push to main at the server. No
  local hook bypass affects this.
- Local git `pre-push` hook gives faster feedback for agents that respect
  hooks.
- Claude PreToolUse hook gives even faster feedback inside Claude Code.

**What a subagent's life looks like**:
```
spawn (in isolation worktree)
  → PRELUDE (pwd guard + origin/main branch + clean-tree check)
  → mechanical work (read files, edit files, never `git add .`)
  → cargo test + predicate verification
  → git push -u origin <feature-branch>  (NOT main)
  → gh pr create
  → POSTLUDE (gh pr view ↔ git rev-parse comparison)
  → final report with BRANCH/HEAD_SHA/PR_NUMBER/PR_URL/VERIFICATION fields
exit
```

**What the orchestrator does**:
```
read subagent report
  → verify report fields against gh pr view (independent verification)
  → if mismatch: reject subagent result, untangle as needed
  → if match: verify predicate recipe
  → if predicates GREEN: gh pr merge --merge --delete-branch
  → if predicates RED: comment + close OR fix
```

### III.12 Orchestrator merge protocol

The orchestrator role can be played by:
- A human user with admin perms on the repo
- Claude main (NOT a subagent — Claude main has full context + tool access)

**Orchestrator merge sequence** (per K-HARDEN-7 PR-only rule):
1. Subagent reports PR URL + claimed predicates
2. Orchestrator runs `gh pr view <N> --json files,headRefOid,headRefName --jq` and verifies
3. Orchestrator runs the predicate recipe locally (or trusts CI if K-HARDEN-5 covers it)
4. Orchestrator runs `gh pr merge <N> --merge --delete-branch`
5. Orchestrator `git pull --ff-only origin main` to sync local

**If GitHub-side merge is unavailable** (offline, network issue):
```bash
# Merge PR locally + push the merge commit
gh pr checkout <N>
git checkout main
git merge --no-ff <pr-branch>
GIT_HARDEN_ALLOW_MAIN=1 git push origin main
```

The bypass env var is the **only authorized way to push to main**, and it's
audit-logged.

---

## Part IV — Bootstrap a new project

### IV.13 Skeleton to copy

For a new project (let's say `myrepo`), copy the following files:

```
myrepo/
├── HARNESS_PLAYBOOK.md                # this file
├── AGENTS.md                          # adapt §14a from gretjia/turingosv4
├── CLAUDE.md                          # adapt to your runtime
├── .gitignore                         # add the 2 K-HARDEN-1 patterns
├── .claude/
│   ├── settings.json                  # wire hooks
│   └── hooks/
│       ├── create_worktree.sh         # K-HARDEN-1
│       ├── validate_git_add.sh        # K-HARDEN-2
│       └── validate_git_push.sh       # K-HARDEN-6
├── .github/
│   └── workflows/
│       └── validate-agent-pr.yml      # K-HARDEN-5
├── scripts/
│   ├── hooks/
│   │   ├── pre-commit.r022            # repo-specific (you may rename)
│   │   └── pre-push.harden            # K-HARDEN-7 universal
│   ├── install_hooks.sh               # idempotent installer
│   ├── setup_branch_protection.sh     # one-time GitHub setup
│   ├── run_constitution_gates.sh      # gate runner (~50 lines)
│   ├── constitution_gates.manifest.toml # initially empty `[[gate]]` array
│   └── dispatch_subagent.sh           # external orchestrator helper
├── skills/
│   ├── SUBAGENT_HARNESS.md            # K-HARDEN-3 (mandatory prompt template)
│   ├── KARPATHY_ARCHITECT.md          # optional design discipline reference
│   └── KARPATHY_SIMPLE_CODE.md        # optional coding discipline reference
└── tests/
    ├── support/
    │   └── mod.rs                     # shared test Harness (lift from your sequencer)
    ├── constitution_matrix_drift.rs   # K-2.3 drift gate
    ├── constitution_rules_ci_mirror.rs# K-3.1' CI redundancy mirror
    └── constitution_subagent_pr_hygiene.rs # K-HARDEN-4 meta-gate
```

**Files that must be project-specific** (not copied verbatim):
- `constitution.md` — your axiom layer
- `tests/constitution_*.rs` — your project's invariants
- `scripts/hooks/pre-commit.*` — if you have project-specific pre-commit logic
- `AGENTS.md` / `CLAUDE.md` — adapt to your domain

### IV.14 Adapt the gates

Each `tests/constitution_*.rs` should encode one constitution invariant. Use
this template:

```rust
//! K-<atom-id> — <one-line description>
//!
//! ## Constitutional binding
//! - Clause: <constitution.md §X Art. Y.Z>
//! - Authority: <handover/directives/YYYY-MM-DD_*.md>
//! - Kill condition: <what makes this gate fire RED>
//!
//! ## Mechanism
//! <how the test detects violation>

use std::fs;
use std::path::Path;

#[test]
fn invariant_named_after_what_it_proves() {
    // Read the canonical artifact
    let content = fs::read_to_string("path/to/artifact").expect("artifact exists");

    // Assert the invariant
    assert!(
        condition_that_proves_invariant(&content),
        "Constitution violation: <why this is bad>"
    );
}
```

Then add the manifest entry:

```toml
[[gate]]
name = "constitution_<gate_id>"
authority = "<specific directive file path>"
added = "<YYYY-MM-DD>"
```

And either add a row to the matrix or add the gate name to K-2.3 allowlist
(with `K23_SHIP_ALLOWLIST_SIZE` bumped accordingly).

### IV.15 Enable GitHub branch protection

**This is the most important universal fix.** Run once per repo, after
authenticating `gh` with admin perms:

```bash
bash scripts/setup_branch_protection.sh
```

The script sets:
- `required_pull_request_reviews.required_approving_review_count: 0`
  (orchestrator can self-merge; raise to 1+ for stricter teams)
- `allow_force_pushes: false`
- `allow_deletions: false`
- `enforce_admins: false` (orchestrator can override via gh UI emergencies)

Verify:
```bash
gh api repos/$OWNER/$REPO/branches/main/protection | jq
```

**After this is enabled, NO agent (Claude/Codex/Gemini/human/CI) can push
directly to main**, regardless of local hook configuration.

### IV.16 Install local hooks

Per clone (each developer, each CI runner):

```bash
bash scripts/install_hooks.sh
```

This installs:
- `.git/hooks/pre-commit` → `scripts/hooks/pre-commit.r022` (sidecar block +
  project-specific checks)
- `.git/hooks/pre-push` → `scripts/hooks/pre-push.harden` (universal
  push-to-main block)

Idempotent — safe to re-run.

### IV.17 Wire the SUBAGENT_HARNESS skill

In your subagent dispatch prompts (`Agent` tool invocations from Claude main),
**always reference `skills/SUBAGENT_HARNESS.md`** in the prelude:

```
You are doing <task>. Read skills/SUBAGENT_HARNESS.md FIRST and follow its
mandatory PRELUDE + MIDFLIGHT + POSTLUDE sections verbatim.

## Task
<your task spec>

## Allowed paths
<list>

## Forbidden paths
<list>
```

The skill enforces the PR-only workflow + verification protocol mechanically.

---

## Part V — Operating procedures

### V.18 Daily workflow (orchestrator)

```
Morning:
  git pull --ff-only origin main
  bash scripts/run_constitution_gates.sh   # baseline health check
  gh pr list --state open                  # outstanding work

When dispatching a subagent:
  read skills/SUBAGENT_HARNESS.md once
  write prompt with PRELUDE + task + POSTLUDE
  Agent({subagent_type, isolation: "worktree", model: "haiku", prompt: ...})

When subagent returns:
  gh pr view <N> --json files,headRefOid,additions,deletions,headRefName
  (compare to subagent's reported BRANCH/HEAD_SHA/PR_NUMBER)
  if mismatch: untangle (cherry-pick discrete files to clean branch)
  if match: verify predicates, merge

End of phase:
  bash scripts/run_constitution_gates.sh   # confirm green
  retrospective: any new L#?
  if yes: add to lesson library + design new K-HARDEN atom
```

### V.19 New-atom checklist

When adding a new atom (per Class-by-Class Cadence in AGENTS.md §14):

| Class | Charter | Directive | Matrix | §8 | Audit | Memory |
|-------|---------|-----------|--------|-----|-------|--------|
| 0 docs | no | no | no | no | none | only recurring rule |
| 1 additive | no | no | no | no | predicate self-test | only recurring rule |
| 2 wire-up | brief | optional | yes | no | clean-context audit | surprise only |
| 3 auth/money/CAS | TB charter | yes | yes | required | clean-context audit (any platform) | yes |
| 4 constitution/sequencer | TB charter | yes | yes | per-atom §8 | clean-context audit PRE-§8 (any platform) | yes |

For every Class-2+ atom:
1. Write the gate test FIRST (predicate)
2. Run it (must fail)
3. Implement until gate passes
4. Add manifest entry + matrix row (or allowlist)
5. PR + orchestrator review + merge

### V.20 Adversarial validation cadence

After every major harness change, dispatch a small "adversarial probe":
- 1-4 haiku subagents doing trivial tasks (add a gate, shrink allowlist,
  diagnostic report, safety-net probe)
- Observe whether hooks/gates fire as designed
- If something silently fails: new L# lesson, new K-HARDEN atom

We did this for K-HARDEN-1..6 → surfaced L9 → built K-HARDEN-7. This is the
**spiral-up loop**.

Cadence:
- After 5+ K-HARDEN-style ship: re-run 4-haiku adversarial set
- After any new K-HARDEN atom: dispatch 1 probe to verify hook fires

### V.21 Recovery from contamination

When a subagent commits contamination (sidecar files, wrong branch, etc.):

**For wrong-branch incidents (L5)**:
```bash
# Cherry-pick the GOOD commits onto a clean branch
git checkout -b recovery-<atom-id> origin/main
git cherry-pick <good-commit-sha>
# Drop the bad files
git rm <bad-files>
git commit --amend --no-edit
git push -u origin recovery-<atom-id>
gh pr create ...
# Close the contaminated PR
gh pr close <contaminated-N> --comment "Closed in favor of clean recovery"
```

**For push-to-main incidents (L9 — should be impossible after K-HARDEN-7,
but recovery doc):**
```bash
# If somehow main got a bad commit pushed
git revert <bad-sha> --no-edit
GIT_HARDEN_ALLOW_MAIN=1 git push origin main
# Or, less destructively, leave it + add follow-up commit
```

**For sidecar contamination (L8)**:
- K-HARDEN-2 hook should have blocked it. If it leaked:
  - `git reset --soft HEAD~1` (undo last commit, keep changes staged)
  - `git restore --staged <sidecar-files>`
  - `git commit -m "..."` (re-commit with clean staging)

---

## Appendix

### A. L1-L9 reference card

```
L1 — Haiku exits isolation worktree
L2 — Manifest authority is placeholder
L3 — Verification pollutes main tree
L4 — git checkout -b from local (not origin/main)
L5 — Multi-haiku branch entanglement (Anthropic upstream bugs)
L6 — Reports of "identical pattern" are approximate
L7 — Subagent report-vs-remote divergence
L8 — git add . wildcard staging
L9 — Direct push to main bypassing PR (the rule that triggers PR-only)
```

### B. Hook contracts

**Claude Code PreToolUse hook**:
- Receives JSON on stdin: `{"tool_input": {"command": "..."}}`
- Output options:
  - exit 0 + no output → allow
  - exit 0 + JSON `{"hookSpecificOutput": {"permissionDecision": "deny", ...}}` → block with reason
  - exit 2 + stderr → block with stderr shown to model

**Claude Code WorktreeCreate hook**:
- Receives JSON: `{"base_path": "..."}`
- Outputs worktree path on stdout
- Exit 0 = success; non-zero = error

**Git pre-push hook**:
- Receives stdin: `<local_ref> <local_sha> <remote_ref> <remote_sha>` per ref
- Args: `$1 = remote_name $2 = remote_url`
- exit 0 = allow; non-zero = abort push

**Git pre-commit hook**:
- No stdin, no args
- Inspect `git diff --cached --name-only`
- exit 0 = allow; non-zero = abort commit (bypassable via `--no-verify`)

### C. Script catalog

| Script | Purpose | When to run |
|--------|---------|-------------|
| `scripts/install_hooks.sh` | Install pre-commit + pre-push hooks | Once per clone |
| `scripts/setup_branch_protection.sh` | Enable GitHub branch protection on main | Once per repo (admin) |
| `scripts/run_constitution_gates.sh` | Discover + run all constitution gates | CI + local sanity |
| `scripts/dispatch_subagent.sh` | External orchestrator wrapper | When orchestrating outside Claude Code |
| `scripts/check_trace_matrix.py` | R-022 backlink check (project-specific) | Pre-commit |
| `scripts/generate_constitution_matrix.py` (optional) | Auto-generate matrix from gates | Manual or hooked |

### D. Constitution gate template

Save as `tests/constitution_<gate_id>.rs`:

```rust
//! K-<atom-id> — <single-line purpose>
//!
//! Constitutional binding:
//!   §<letter> Art. <Y.Z> — <invariant name>
//!
//! Authority: handover/directives/<YYYY-MM-DD>_<topic>.md
//!
//! Mechanism: <how the test asserts the invariant>

use std::fs;
use std::path::Path;

#[test]
fn invariant_descriptive_name() {
    let artifact = fs::read_to_string("path/to/artifact").expect("artifact must exist");
    assert!(
        condition_that_holds_iff_invariant_holds(&artifact),
        "Constitution violation: <human-readable explanation>"
    );
}

#[test]
fn boundary_descriptive_name() {
    // Boundary check: the invariant must STILL hold under stress
    // e.g., specific edge cases, error paths, etc.
}
```

Add to manifest:

```toml
[[gate]]
name = "constitution_<gate_id>"
authority = "handover/directives/<YYYY-MM-DD>_<topic>.md"
added = "<YYYY-MM-DD>"
```

Add to matrix or allowlist (in `tests/constitution_matrix_drift.rs`).

---

## License + provenance

This playbook is part of the gretjia/turingosv4 harness (2026-05-20).
Reusable in any project. Copy + adapt freely. Cite source if helpful:

```
TuringOS Harness Playbook v1.0 (2026-05-20)
gretjia/turingosv4 — Karpathy v3 plan + K-HARDEN 1-7 atoms
https://github.com/gretjia/turingosv4
```

For the full backstory:
- `handover/architect-insights/MULTI_AGENT_ISOLATION_RESEARCH_2026-05-20.md` (954-line research)
- `handover/architect-insights/K_HARDEN_PROPOSAL_2026-05-20.md` (233-line proposal)
- v3 plan: `~/.claude/plans/karpathy-architect-md-turingosv4-harnes-splendid-sunbeam.md`

---

*End of HARNESS_PLAYBOOK.md*
