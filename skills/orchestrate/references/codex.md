# Platform adapter — Codex CLI

How `./SKILL.md` maps to OpenAI's Codex CLI agent runtime.

## Tier → model mapping

| Tier | Codex / OpenAI model | Reasoning effort |
|---|---|---|
| **BRIEF** | `gpt-mini`-class fast models | low |
| **STANDARD** | `gpt-balanced` / `gpt-4o`-class | medium |
| **DELIBERATIVE** | `o-series` reasoning models / `gpt-pro` | high |

Notes:
- Codex CLI exposes reasoning-effort knobs at dispatch time — use them
  to match the tier (BRIEF=low, STANDARD=medium, DELIBERATIVE=high).
- For DELIBERATIVE-tier critics + witness, prefer o-series reasoning
  models when available.

## Sub-agent dispatch

Codex CLI follows the OpenAI Swarm handoff convention (one agent
explicitly hands off to another via a function call). Adapted to the
orchestrator pattern:

- Orchestrator agent = the long-lived planning context
- Sub-agents = stateless function-handoff targets
- Each handoff includes the FULL 9-section brief as the new agent's
  initial message

Parallel dispatch on Codex: spawn N parallel sub-shells, each running
its own Codex session with the agent's brief. The orchestrator polls
each session's output upon completion.

## Output handling

Codex sub-agents don't have a built-in "background task" mode like
Claude Code's `Agent` tool. Practical pattern:

1. Write each sub-agent brief to a file (`/tmp/agent-<role>-<id>.md`)
2. Spawn parallel `codex --headless --input <file>` invocations
3. Poll the output streams; collect when done
4. Read each agent's stdout transcript into the orchestrator context

## Plan-grill on Codex

`./plan-grill.md` doesn't have a structured-question-tool primitive on
Codex (no equivalent of `AskUserQuestion`). Fallback:

Render the question as plain text following the mandatory format:

```
Question: <stem ≤30 words>

A. <label> — <one-sentence trade-off>
B. <label> (recommended) — <one-sentence trade-off + why default>
C. <label> — <one-sentence trade-off>

Reply with A, B, or C (or custom text).
```

Parse the reply with simple regex against A/B/C/D.

## PR-only workflow

Per the universal `AGENTS.md §14a` PR-only workflow:
- Implementer sub-agents must NOT `git commit` / `git push` /
  `gh pr create`
- `scripts/hooks/pre-push.harden` git hook applies to Codex agents
  too — they respect git hooks by default
- Codex CLI does NOT have a Claude-Code-style additional hook layer;
  the universal git-hook layer is the only client-side enforcement

## Tool permissions

Codex CLI sub-agents inherit the parent shell's tool access. To restrict
an implementer's tool surface:
- Run the agent in a Docker container with limited mounts
- Use OS-level permission scoping
- OR rely on the brief's Hard Constraints (Section 4) + Abort Protocol
  (Section 8) and trust the agent to self-restrict

The orchestrator's pre-dispatch self-check should verify the brief's
Hard Constraints list is complete before spawning the sub-agent.

## Workspace conventions

- Skill location: `<repo>/skills/orchestrate/` (same as Claude)
- Global mirror: depends on Codex's user-config directory
- AGENTS.md remains the canonical cross-platform contract (per its
  own §2)
