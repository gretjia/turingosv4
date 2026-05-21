# Gemini CLI cold-start entry (K-HARDEN-8)

This project uses a **cross-agent harness** with Claude Code as the alignment
standard. Every CLI (Gemini / Codex / Aider / Cursor / Windsurf / Copilot /
Warp / Claude Code / human shell) MUST read the same canonical files.

## Mandatory read order (cold start)

1. **`AGENTS.md`** — canonical shared agent contract (truth order, class
   taxonomy, restricted surfaces, done definition, §14a PR-only workflow)
2. **`HARNESS_PLAYBOOK.md`** — full 673-line operating manual
3. **`skills/SUBAGENT_HARNESS.md`** — mandatory PRELUDE/MIDFLIGHT/POSTLUDE
   template if you dispatch subagents
4. **`constitution.md`** — axiom layer (constitution invariants the harness binds to)
5. **`handover/ai-direct/LATEST.md`** — current session state (derived view)

## Hard rules for Gemini CLI

- **PR-only**: you may NEVER `git push origin main`. Open a PR via `gh pr create`.
  Direct push to main is blocked by (a) GitHub server-side branch protection
  and (b) the local `.git/hooks/pre-push` hook. Use `git push -u origin <feature-branch>` only.
- **No wildcard staging**: never use `git add .` or `git add -A`. Stage explicit
  file paths only. This rule is enforced by `scripts/hooks/pre-commit.r022` at git layer.
- **Restricted surfaces**: see `AGENTS.md` §6. Touching `src/kernel.rs`,
  `src/state/sequencer.rs`, `constitution.md`, etc. requires per-atom architect §8.
- **Subagent dispatch**: if you dispatch Gemini subagents, ensure each prompt
  includes the SUBAGENT_HARNESS PRELUDE + POSTLUDE block.

## Setup (one time per clone)

```bash
bash scripts/install_hooks.sh        # local pre-commit + pre-push hooks
bash scripts/setup_branch_protection.sh  # GitHub server-side (admin only)
```

## Verification (cold-start sanity)

```bash
bash scripts/run_constitution_gates.sh  # all gates must be GREEN
cargo test --test constitution_subagent_pr_hygiene  # 16 meta-tests must pass
```

## Why this file is thin

Gemini CLI hierarchical context loader will read this file first (per
geminicli.com/docs/cli/gemini-md). All operating detail lives in `AGENTS.md`
and `HARNESS_PLAYBOOK.md` — both shared across every CLI. This file exists
only so Gemini CLI's hierarchical loader auto-discovers the harness contract.

If you find any rule here that contradicts `AGENTS.md`, `AGENTS.md` wins.
