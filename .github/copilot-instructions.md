# GitHub Copilot cold-start instructions (K-HARDEN-8 cross-CLI alignment)

This project uses a cross-agent harness. GitHub Copilot follows the same
canonical contract as Claude Code, Codex CLI, Gemini CLI, Aider, Cursor,
and Windsurf.

## Read order (mental order Copilot should attend to)

1. **AGENTS.md** — canonical shared agent contract
2. **HARNESS_PLAYBOOK.md** — full operating manual
3. **skills/SUBAGENT_HARNESS.md** — subagent dispatch template
4. **constitution.md** — axiom layer (constitution invariants)

## Hard rules (enforced mechanically)

- **PR-only workflow**: never `git push origin main`. Always create a feature
  branch (`git checkout -b feat/whatever origin/main`) and push there. Open
  a PR via the GitHub UI or `gh pr create`. Direct push blocked at:
  - GitHub server-side branch protection (universal across all clients)
  - Local git pre-push hook (`scripts/hooks/pre-push.harden`)
- **No wildcard staging**: never `git add .`, `git add -A`, or `git add --all`.
  Use explicit file paths. Enforced at the pre-commit layer.
- **No sidecar contamination**: never stage files under
  `handover/evidence/dev_self_hosting/dev_*/` or `.claude/worktrees/`.
- **Restricted surfaces** (see `AGENTS.md` §6): touching `src/kernel.rs`,
  `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `constitution.md`,
  `rules/engine.py`, etc. requires per-atom architect §8.

## Coding style

- `skills/KARPATHY_ARCHITECT.md` — first-principles architecture
- `skills/KARPATHY_SIMPLE_CODE.md` — transparent data flow, no Manager/Factory/Engine

## Setup (one time per clone)

```bash
bash scripts/install_hooks.sh
bash scripts/setup_branch_protection.sh    # admin only
```

## Conflict resolution

If anything here contradicts `AGENTS.md`, `AGENTS.md` wins. This file exists
so Copilot's repo-instructions feature picks it up automatically.
