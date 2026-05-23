# Platform adapter — Claude Code

How `./SKILL.md` maps to Claude Code (Anthropic's IDE/CLI agent runtime).

## Tier → model mapping

| Tier | Claude model | Thinking depth |
|---|---|---|
| **BRIEF** | `claude-haiku-4-5` (or current Haiku class) | low |
| **STANDARD** | `claude-sonnet-4-5` (or current Sonnet class) | medium |
| **DELIBERATIVE** | `claude-opus-4-5` OR `claude-sonnet-4-5` + thinking-max | max |

Notes:
- The orchestrator picks model at dispatch time via the `Agent` tool's
  `model` parameter (`"sonnet" | "opus" | "haiku"` short names) AND/OR
  passes thinking-depth hints in the prompt.
- For DELIBERATIVE-tier critics + witness, prefer Opus when available;
  fall back to Sonnet + explicit "think deeply, max thinking" framing
  in the brief if Opus capacity-constrained.

## Sub-agent dispatch

Claude Code provides the `Agent` tool. Standard invocation:

```typescript
Agent({
  description: "<short 3-5 word role label>",
  subagent_type: "general-purpose",  // or a custom subagent type
  model: "sonnet",                    // BRIEF=haiku, STANDARD=sonnet, DELIBERATIVE=opus
  prompt: "<the 9-section brief>",
  run_in_background: true,            // for parallel dispatch
})
```

For PARALLEL dispatch (Phase 1 Researchers / Phase 3 Implementers / Phase 5
Critics): **send ONE message with multiple `Agent` tool uses**. Claude
Code executes them in parallel.

## Subagent type selection

- `general-purpose` — default for most roles (Researcher / Implementer /
  Critic / Witness / User-Simulator)
- `Explore` — read-only investigation; use for Researcher when the agent
  needs no write tools
- `Plan` — design-time validation; use for Researcher when validating
  an implementation design before code is written

## Output handling

Background agents (`run_in_background: true`) return a task-id. Claude
main is auto-notified when each completes. Do NOT poll the agent's
output file — the harness handles re-invocation.

## Plan mode integration

When operating in Claude Code's plan mode (`EnterPlanMode`):
- Phase 1 (Researcher) and Phase 5 (Critic / Witness) are compatible
  — they're read-only
- Phase 3 (Implementer) is NOT plan-mode-compatible — implementers
  edit files. Defer to ExitPlanMode → Auto Mode for execution.
- Phase 6 (orchestrator triage) is compatible — read-only
- Phase 8 (commit/push/PR) requires Auto Mode

## Tool permissions

Implementer briefs should specify `allowed-tools` minimally:
- File edits: `Edit`, `Write`, `Read`
- Verification: `Bash` (limited to test/build commands)
- NO `git` write tools — orchestrator handles commit/push/PR

Critic + Witness briefs: read-only by design. `Read`, `Bash` (limited
to grep/test-running), `Grep`.

## PR-only workflow

Per `AGENTS.md §14a` PR-only workflow: implementer sub-agents do NOT
commit / push / open PRs. Three layers of enforcement:
1. GitHub branch protection (server-side)
2. `scripts/hooks/pre-push.harden` git hook
3. Claude Code `.claude/hooks/validate_git_push.sh` (Claude-specific
   extra layer)

Phase 8 in this skill = orchestrator (Claude main) runs `gh pr create`
with appropriate body. Sub-agents must NOT.

## Plan-grill on Claude Code

`./plan-grill.md` uses `AskUserQuestion` tool. On Claude Code:

```typescript
AskUserQuestion({
  questions: [{
    question: "<stem ≤30 words>",
    header: "<≤12-char chip label>",
    multiSelect: false,
    options: [
      { label: "<5-word label> (推荐)",
        description: "<1-sentence trade-off including why this is the orchestrator's default>" },
      { label: "<alt-1>",
        description: "<1-sentence trade-off>" },
      { label: "<alt-2>",
        description: "<1-sentence trade-off>" },
    ],
  }],
})
```

Max 4 questions per `AskUserQuestion` call. The framework provides an
"Other" option automatically — DO NOT include "Other" in your options
list (per `plan-grill.md` anti-pattern).

## Workspace conventions

- Skill location: `<repo>/skills/orchestrate/` (source of truth)
- Global mirror: `~/.claude/skills/orchestrate/` (symlink to the
  whole folder)
- TRACE_MATRIX backlinks required on public Rust items via R-022
  check; skill docs themselves don't trigger R-022

## Plan file storage

Plans go in `~/.claude/plans/<short-slug>.md`. The plan workflow has
its own state machine (`EnterPlanMode` / `ExitPlanMode`). This skill
is invoked DURING plan execution, not as a replacement for plan mode.
