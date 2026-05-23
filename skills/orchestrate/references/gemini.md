# Platform adapter — Gemini CLI

How `./SKILL.md` maps to Google's Gemini CLI / Vertex AI Agent runtime.

## Tier → model mapping

| Tier | Gemini model | Notes |
|---|---|---|
| **BRIEF** | `gemini-2.5-flash` / `gemini-flash-lite` | thinking-off |
| **STANDARD** | `gemini-2.5-pro` | thinking-default |
| **DELIBERATIVE** | `gemini-2.5-pro` + thinking-on / future Ultra | extended-thinking mode |

Notes:
- Gemini positions Flash for speed/cost (agentic coding, production),
  Pro for deeper reasoning, with optional extended-thinking mode.
- For DELIBERATIVE-tier critics + witness, enable extended-thinking on
  Pro OR upgrade to Ultra tier when available.

## Sub-agent dispatch

Gemini's Vertex AI Agent Builder provides a graph-based orchestration
engine with supervisor + specialist agents. Adapted to this skill's
orchestrator pattern:

- Orchestrator = the supervisor agent
- Sub-agents (Researcher / Implementer / Critic / Witness / User-Simulator)
  = specialist agents in the graph

The Agent Development Kit (ADK) handles tool governance, sub-second
cold starts, and session management. Cross-agent communication
happens via the graph engine, not direct function calls.

Parallel dispatch: define the graph with parallel branches for the
same parent node. The graph engine handles concurrent execution.

## Plan-grill on Gemini

Gemini doesn't have a built-in structured-question UI primitive.
Fallback pattern same as Codex:

Render as plain-text decision-fork:

```
Question: <stem ≤30 words>

A. <label> — <trade-off>
B. <label> (recommended) — <trade-off + why>
C. <label> — <trade-off>

Reply with one of A/B/C, or custom text.
```

## Output handling

Vertex AI agents emit structured events via the ADK. The orchestrator
listens on the agent's event stream. Each agent's final report (per
Section 9 of the standard brief) goes into the event stream as a
structured "report" event.

## PR-only workflow

Per the universal `AGENTS.md §14a` PR-only workflow:
- Implementer sub-agents must NOT commit / push / open PRs
- `scripts/hooks/pre-push.harden` git hook applies to Gemini agents
  too — they respect git hooks
- Gemini CLI has no Claude-Code-style additional hook layer; rely on
  the universal git-hook layer

## Tool permissions

Vertex AI ADK provides per-agent tool governance natively. Use it to
restrict implementer-agent tool surface:
- `allowed_tools` per agent spec
- Tool execution audit logged by ADK

## Workspace conventions

- Skill location: `<repo>/skills/orchestrate/` (same path as Claude /
  Codex)
- Gemini CLI reads `GEMINI.md` (a thin pointer to `AGENTS.md`); that
  file remains the cross-platform contract
- Vertex AI Agent Builder graphs are defined in YAML/Python in the
  project — they reference this skill folder via path
