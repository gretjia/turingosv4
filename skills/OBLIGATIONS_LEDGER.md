# Obligation Ledger Skill — Cross-Agent Requirement Continuity

K-OBL-1 (2026-05-24) — prevents requirement evaporation across multi-turn
agent conversations.

Lesson source: V-010 Phase 7 nightly E2E pivot incident — a "15-persona
DeepSeek Chrome E2E" mandate was silently replaced by a "node/polymarket UI
visibility fix" mid-conversation; multi-agent audit returned `PROCEED` on the
substitution, not on the original mandate. Audit scope contracted to the
substituted task; original obligation never executed.

Applies to all coding agents — Claude Code, Codex CLI, Gemini CLI, Aider,
Cursor, Windsurf, Copilot, Warp, future runtimes. Enforced via prompt-layer
convention + repository file + audit verdict extension. No new runtime, no
daemon, no service, no database.

## 1. The failure mode this skill blocks

**Conversational Anchor Drift** — the structural pattern:

1. User states task T with N hard requirements R1..RN.
2. Agent acknowledges T.
3. Mid-flight, user offers debug input D (e.g. "step 3 doesn't work").
4. Agent implicitly redefines `T := fix D`.
5. R1..RN minus the one related to D quietly evaporate.
6. Multi-agent audit scopes only to the substituted task.
7. Agent declares done; user discovers R1..RN-1 were never executed.

This is **not** plain context-window compaction. It is **implicit task
redefinition during interactive debugging**. Compaction may make it worse,
but the root cause is conversational momentum, not token loss.

## 2. Mechanism — one file, one schema

Single per-project file at working-directory root:

```
<project_root>/OBLIGATIONS.md
```

Plain markdown. One `##` section per obligation. Schema:

```markdown
## OBL-<NNN>: <short imperative title>
- Source: "<verbatim user quote; redact secrets/PII; keep enough context to disambiguate>"
- Level: must | should
- Status: open | satisfied | superseded | blocked
- Evidence: <repo-relative path, URL, or "TBD">
- Superseded-by: OBL-<id>          # only when Status = superseded
- Blocker: <one-line reason>       # only when Status = blocked
- Last-touched: YYYY-MM-DD
```

`OBL-NNN` is a monotonic integer prefixed `OBL-` (OBL-001, OBL-002, ...). IDs
are never reused, never renumbered.

Markdown over JSON deliberately: agents parse text natively, users can edit
the file directly, no schema-tool dependency, no `jq` in the loop.

## 3. The four rules (mandatory)

### Rule 1 — Create on first imperative

At the start of any task at Class ≥ 1 (see `AGENTS.md §5`), if
`OBLIGATIONS.md` does not exist, create it. Extract the user's initial
imperatives into OBL-001..OBL-00N. Skip for pure Class 0 single-file doc
edits.

### Rule 2 — Every-turn reconcile

Every assistant turn that proposes **implementation**, **audit**, or
**completion** must begin with a one-line obligation header:

```
Active obligations: OBL-001 (open), OBL-002 (satisfied), OBL-003 (blocked) → <next action>
```

Required even when nothing changed. Makes drift visible turn-by-turn so the
user can interrupt before it compounds.

### Rule 3 — Implicit redefinition is forbidden

User debug input / mid-flight clarification = **input to an existing
obligation or a new sub-obligation**, never a replacement.

Replacement requires an explicit user trigger phrase, examples:
- "取消 OBL-NNN" / "cancel OBL-NNN"
- "不要 X 了" / "drop X"
- "改用 Y 代替 X" / "supersede X with Y"
- "停掉 X，改做 Y" / "stop X, do Y instead"

Absent such phrase, treat the new input as:
- **Evidence/debug for the current OBL-NNN** → append to the `Evidence` line.
- **A new requirement** → append a new `OBL-NNN+1`.

Never as: "OK, the task is now this new thing the user just mentioned."

### Rule 4 — Done gate

No `done` / `完成` / `shipped` / `PROCEED` / `complete` statement may be
issued while any OBL with `Level=must` has `Status=open`. The only valid
closure states for a `must` are:

- `satisfied` — must include a concrete `Evidence:` path that the user could
  click and verify.
- `blocked` — must include a `Blocker:` line and `Evidence:` pointing to
  proof of the blocker (e.g. external API down, missing credential).
- `superseded` — must include `Superseded-by:` AND a quoted user trigger
  phrase from Rule 3 in the `Evidence:` line.

Multi-agent audits (per `AGENTS.md §14`) must include an **Obligation
Completeness** witness whose sole verdict is one of:

- `OBL-ALL-CLOSED`
- `OBL-OPEN-MUST <OBL-id>`
- `OBL-EVIDENCE-MISSING <OBL-id>`
- `OBL-BLOCKER-UNVERIFIED <OBL-id>`

A `PROCEED` from any other audit witness is **invalid** if the obligation
witness verdict is not `OBL-ALL-CLOSED`.

## 4. Cross-platform installation

Single repo file → every agent reads same source. No agent-specific code.

| Agent | Discovery path |
|-------|----------------|
| Claude Code | `CLAUDE.md §5` pre-action gate references this skill |
| Codex CLI | `AGENTS.md §16` (canonical) |
| Gemini CLI | `GEMINI.md` → `AGENTS.md §16` |
| Aider | `CONVENTIONS.md` / `.aider.conf.yml` → `AGENTS.md §16` |
| Cursor | `.cursor/rules/000-agents-alignment.mdc` or `.cursorrules` → `AGENTS.md §16` |
| Windsurf | `.windsurfrules` → `AGENTS.md §16` |
| Copilot | `.github/copilot-instructions.md` → `AGENTS.md §16` |
| Warp | `WARP.md` → `AGENTS.md §16` |

All thin discovery files already redirect to `AGENTS.md`. Adding §16 propagates
to every agent in one edit. No per-agent shim.

## 5. Lifecycle

- **Create**: first non-trivial user message in a fresh session OR first user
  imperative in an existing session that lacks `OBLIGATIONS.md`.
- **Update**: every new user imperative → new OBL entry; every status change
  → update `Status` + `Last-touched` + `Evidence` together (never one without
  the others).
- **Never silently delete**: closed entries stay for replay/audit. If file
  grows unwieldy, archive to
  `handover/obligations/OBL-archive-<YYYY-MM-DD>.md` and link from
  `OBLIGATIONS.md`. Never drop without an archive link.
- **No global pointer**: each project owns its own `OBLIGATIONS.md`. No
  cross-project ledger, no "latest" pointer.

## 6. Karpathy alignment

- One file. One schema. No daemon, no service, no DB, no MCP server.
- Markdown over JSON: humans edit directly, agents parse as text.
- Convention enforced by prompt + audit, not by middleware.
- Failure mode covered by one synthetic drift case (see §8).
- Bias to the least ceremony that closes the drift channel.

If a future iteration is tempted to add a CLI, a daemon, a schema validator
binary, or a separate service: stop. The point is one file the agent reads
and writes in plain text. Tooling around it is the failure mode this skill
was created to avoid.

## 7. Verdict domain extension

Existing audit verdict domain (`AGENTS.md §14`) for clean-context audit:
- `NO-VIOLATION`
- `VIOLATION-FOUND <clause> <file>:<line>`
- `RECONSTRUCTION-FAILURE <path>`
- `SECOND-SOURCE-DRIFT <view>`

**Obligation-completeness witness only** adds:
- `OBL-ALL-CLOSED`
- `OBL-OPEN-MUST <OBL-id>`
- `OBL-EVIDENCE-MISSING <OBL-id>`
- `OBL-BLOCKER-UNVERIFIED <OBL-id>`

Other audit witnesses retain their original domain. The obligation witness
does not opine on code, style, performance, or architecture — only on
ledger closure. Subjective opinions are out of scope per the same Veto-AI
boundary that constrains the other witnesses.

## 8. Synthetic drift test

Minimum acceptance test for this skill (run by hand or in CI):

1. Seed `OBLIGATIONS.md` with two `Level=must` entries, both `Status=open`.
2. Feed agent a debug clarification on one of them.
3. Have agent attempt a `done` / `PROCEED` statement.
4. Expect: agent refuses with reference to the still-open obligation, or an
   audit witness returns `OBL-OPEN-MUST`.

If the agent ships without reconciling, the harness is broken at the prompt
layer, not the file layer. Patch the prompt (this file) and retest.

## 9. Recovery from prior drift

When introducing this skill into an existing task that already drifted:

1. Walk the conversation from the earliest user message; extract every
   imperative.
2. Map each to one of: still-open / satisfied (with evidence path) /
   superseded (must quote the user trigger phrase) / blocked (with proof).
3. Append an OBL for "install obligation ledger harness itself".
4. State current overall status as **PARTIAL** until original opens close.

The V-010 incident recovery is the canonical example — see this repo's
`OBLIGATIONS.md`.

## 10. Non-goals

This skill does **not**:
- Replace TB charters or per-atom architect §8 sign-off.
- Replace ChainTape/CAS as the truth layer for runtime evidence.
- Track engineering tasks (use git/PRs/`handover/tracer_bullets/TB_LOG.tsv`).
- Provide a UI or dashboard (`OBLIGATIONS.md` IS the UI).
- Persist beyond the session's project root.

It exclusively tracks **user-stated obligations to the agent** for the
purpose of preventing implicit drift between user mandate and agent
delivery.
