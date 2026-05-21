# Aider conventions (K-HARDEN-8 cross-CLI cold-start entry)

This project uses a **cross-agent harness**. Aider must follow the same
contract as Claude Code, Codex CLI, Gemini CLI, and every other coding agent.

## Mandatory read order

When you start a session, also read these (load via `--read` if not done by
`.aider.conf.yml`):

1. `AGENTS.md` — canonical agent contract
2. `HARNESS_PLAYBOOK.md` — full operating manual
3. `skills/SUBAGENT_HARNESS.md` — subagent dispatch template

## Hard rules (enforced mechanically by hooks)

- **PR-only workflow**: never `git push origin main`. Create a feature branch,
  push to it, open a PR with `gh pr create`. Direct push blocked at:
  (a) GitHub server-side, (b) local `.git/hooks/pre-push`.
- **No wildcard staging**: never `git add .` or `git add -A`. Use explicit paths.
- **Restricted surfaces** (see `AGENTS.md` §6): `src/kernel.rs`,
  `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `constitution.md`,
  `rules/engine.py`, `rules/active/*.yaml` — require per-atom architect §8.
- **No sidecar contamination**: `handover/evidence/dev_self_hosting/dev_*/`
  must never be staged. `.gitignore` blocks it; pre-commit hook double-checks.

## Coding style

Two reference files in `skills/`:
- `KARPATHY_ARCHITECT.md` — first-principles architecture, monolithic-flat default
- `KARPATHY_SIMPLE_CODE.md` — transparent data flow, no Manager/Factory/Engine
  abstractions, surgical changes

## Setup (one time per clone)

```bash
bash scripts/install_hooks.sh
bash scripts/setup_branch_protection.sh  # admin only
```

## Verification

```bash
bash scripts/run_constitution_gates.sh
cargo test --test constitution_subagent_pr_hygiene
```

This file is intentionally under 100 lines per Aider best practice (rule
adherence degrades past ~200 lines). Full detail in `AGENTS.md`. If anything
here conflicts with `AGENTS.md`, `AGENTS.md` wins.
