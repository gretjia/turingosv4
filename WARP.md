# Warp cold-start entry (K-HARDEN-8 cross-CLI alignment)

This project uses a cross-agent harness. Warp Agent follows the same canonical
contract as Claude Code, Codex CLI, Gemini CLI, Aider, Cursor, and Copilot.

## Read order

1. AGENTS.md (canonical shared agent contract)
2. HARNESS_PLAYBOOK.md (operating manual)
3. skills/SUBAGENT_HARNESS.md (subagent dispatch template)
4. constitution.md (axiom layer)

## Hard rules (enforced mechanically)

- **PR-only**: never `git push origin main`. Feature branch + PR via `gh pr create`.
  Server-side GitHub branch protection blocks direct push universally; local
  git pre-push hook (`scripts/hooks/pre-push.harden`) blocks before push.
- **No wildcard staging**: never `git add .` / `-A` / `--all`. Explicit paths only.
- **No sidecar contamination**: never stage `handover/evidence/dev_self_hosting/dev_*/`.
- **Restricted surfaces** (AGENTS.md §6): need per-atom architect §8.

## Coding style

- skills/KARPATHY_ARCHITECT.md (first-principles, monolithic-flat)
- skills/KARPATHY_SIMPLE_CODE.md (transparent data flow, no Manager/Factory/Engine)

## Setup once per clone

```
bash scripts/install_hooks.sh
bash scripts/setup_branch_protection.sh   # admin only
```

AGENTS.md is canonical. Conflicts → AGENTS.md wins.
