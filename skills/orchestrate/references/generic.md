# Platform adapter — generic fallback

For any orchestrator runtime not specifically covered by `claude.md`,
`codex.md`, or `gemini.md`. Use this template to derive a new
platform-specific adapter.

## Tier → model mapping (placeholder)

| Tier | Generic role | Pick on your platform |
|---|---|---|
| **BRIEF** | Fast reactive; ≤200 LoC; narrow scope | Platform's fastest / cheapest reasoning model |
| **STANDARD** | Multi-constraint impl; 200-800 LoC | Platform's balanced general-purpose model |
| **DELIBERATIVE** | Architecture / critic / witness | Platform's deepest reasoning / longest thinking |

The orchestrator picks tier at dispatch time. The platform-specific
model identifier is chosen by the orchestrator from what the runtime
exposes.

## Sub-agent dispatch (generic pattern)

Two dispatch primitives are needed:
1. **Spawn**: start a sub-agent with the 9-section brief as initial input
2. **Collect**: read the sub-agent's structured final report

For PARALLEL dispatch (Phase 1 / Phase 3 / Phase 5), spawn N agents
"at once" — whatever the platform's primitive is for concurrent
execution. Then collect their reports when they finish.

If the platform has no built-in parallel-agent primitive:
- Run agents sequentially with the SAME brief input + record
  outputs separately
- Loss: latency. Gain: works on any platform.

## Plan-grill on generic platforms

If the platform has no structured-question tool, use plain-text
decision-fork (same pattern as Codex / Gemini adapters):

```
Question: <stem ≤30 words>

A. <label> — <one-sentence trade-off>
B. <label> (recommended) — <one-sentence trade-off + reasoning>
C. <label> — <one-sentence trade-off>

Reply with one of A/B/C, or custom text.
```

Parse the reply against the option labels.

## Output handling

If the platform has no background-task / async-callback primitive,
poll the sub-agent's output file or stdout until it produces a
final-report sentinel (per the standard brief's Section 9 Final
Report Format).

## PR-only workflow

The PR-only workflow (per `AGENTS.md §14a`) is enforced at THREE
layers, two of which are platform-independent:

1. **GitHub branch protection** (server-side; works for any client)
2. **`scripts/hooks/pre-push.harden` git hook** (works for any agent
   that respects git hooks — most do)
3. (Optional) Platform-specific extra hook layer if available

The first two are sufficient. Implementer sub-agents on any platform
must NOT commit / push / open PRs.

## Tool permissions

If the platform has no per-agent tool governance:
- Run the implementer agent in a restricted shell (Docker, jail,
  sandbox)
- OR rely entirely on the brief's Hard Constraints (Section 4) +
  Abort Protocol (Section 8) and trust the agent to self-restrict
- The orchestrator's pre-dispatch self-check must verify the brief
  is complete before spawning

## Workspace conventions

- Skill folder lives at `<repo>/skills/orchestrate/` regardless of
  platform
- Global mirror path depends on platform's user-config convention
- `AGENTS.md` is the canonical cross-platform contract — `GEMINI.md`
  / `CLAUDE.md` / `.cursorrules` / `WARP.md` etc. are thin pointers
  to it

## How to author your own platform adapter

Copy this file as the template. Replace:
1. **Tier mapping** — name the platform's actual model identifiers
2. **Dispatch primitive** — how you spawn parallel agents
3. **Question primitive** — whether there's a structured-question tool
   or plain-text fallback
4. **Output collection** — how the orchestrator reads sub-agent
   reports
5. **Tool governance** — what the platform exposes for per-agent
   restriction

Then submit a PR adding your adapter alongside this file.
