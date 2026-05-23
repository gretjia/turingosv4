---
name: interface-contract-lock
description: Contract-locking pattern for parallel-implementer dispatch. Locks data shapes / API signatures / function contracts in code-form BEFORE dispatching multiple implementers, so they build to the same target without merge conflicts or invention drift. Auto-invoked from SKILL.md at Phase 2.
allowed-tools: []
---

# Interface Contract Lock — pre-dispatch contract authorship

This file is a sub-protocol auto-invoked by `./SKILL.md` at Phase 2.
End-users do not invoke it directly. Orchestrators acting as the
**Contract-Architect** role consult this file before dispatching parallel
Implementers.

## Why this matters (the single highest-leverage mechanism)

In a real shipping cycle, the orchestrator dispatched 2 parallel
Implementers (Backend + Frontend) that needed to agree on:
- A struct shape (the data carried in memory)
- A JSON shape (the HTTP wire format)
- An HTTP endpoint signature (route + method + params)
- A custom-element attribute contract (DOM API for the frontend)
- A genesis TOML schema (config-file shape)

Without pre-locked contracts, two parallel implementers would invent
incompatible shapes and the merge would require a third reconciliation
pass. With pre-locked contracts, both implementers built to the SAME
target. Zero merge friction.

**This is the single most valuable mechanism in the orchestration
pattern.** Skipping it (treating contracts as "we'll figure out the
interface as we go") leads to:

- Implementers inventing parallel non-compatible shapes
- Per-agent invention drift (naming, field order, type choice)
- Late merge conflicts that require a third agent to reconcile
- Critics in Phase 5 finding "structural mismatch" instead of real bugs

## When to lock (the gate)

Lock contracts BEFORE dispatching parallel Implementers when ANY hold:

- 2+ Implementers will run in parallel
- 1 Implementer's output is consumed by another agent's brief later
- An external system (HTTP client, frontend, downstream tool) depends
  on a specific shape
- Any data flows across a process / module / file boundary

Skip locking when:
- Single Implementer, single file, no consumers
- Refactor that preserves existing contracts (locking is redundant)

## The contract-lock pattern (the format)

Contracts are written by the orchestrator (Contract-Architect role) in
**code-form**, not prose. Named C1, C2, C3, ... so sub-agents can refer
to them by name.

### Example structure

```
## Interface Contracts (LOCKED before dispatch)

### C1. <name>

```rust
pub struct WorkerPersona {
    agent_id: &'static str,
    system_prompt_suffix: &'static str,
    temperature: f32,
    initial_balance_micro: u64,
}
const WORKERS: &[WorkerPersona] = &[ /* α/β/γ */ ];
```

### C2. <name>

```rust
pub struct TournamentOutcome {
    pub session_id: String,
    pub task_id: String,
    pub candidates: Vec<CandidateRecord>,
    pub winner_agent_id: Option<String>,
    pub market_state: MarketState,
    pub treasury_bounty_micro: u64,
}
```

### C3. HTTP API contract

```
GET /api/market/by-session/:session_id
  → 200 application/json:
    {
      "session_id": "...",
      "task_id": "...",
      "market_state": "open" | "finalized" | "all_rejected",
      "treasury_bounty_micro": 1000,
      "candidates": [...],
      "winner_agent_id": null | "worker-alpha"
    }
  → 404 if session has no admissions yet
  → 400 if session_id format invalid
```

### C4. Frontend custom-element contract

```html
<tos-agent-attempts-panel
  session-id="<uuid>"
  data-spec-grill-mount="<uuid>"
></tos-agent-attempts-panel>
```
- Polls `/api/market/by-session/<id>` every 2s while market_state === "open"
- Stops polling on finalized / all_rejected
- Strictly read-only (no betting UI)

### C5. <name>

(...)
```

Each contract is **named** (C1, C2, ...), **typed** (Rust struct / JSON
schema / HTML attribute spec / etc.), and **bounded** (what's in vs out
of this contract).

## Hard rules

1. **Contracts MUST be code, not prose.** "Response includes session_id
   and candidate list" is prose — too vague. A Rust struct definition,
   JSON schema, or HTTP route signature is code.

2. **Contracts are LOCKED.** Once dispatched, sub-agents may not rename
   fields, restructure types, or add fields without coming back to the
   orchestrator. This prevents per-agent invention drift.

3. **Contracts are NAMED.** C1, C2, ... — so every sub-agent brief can
   say "honor contract C2 exactly" without ambiguity.

4. **Contracts go in the brief, verbatim.** Each Implementer brief
   includes the full text of every contract that Implementer touches.
   Not "see the locked contract in the plan" — copy it into the brief.

5. **Contracts have a verifier.** Each contract should be testable
   with a deterministic check (`serde` roundtrip, integration test
   asserting JSON shape, etc.). Acceptance Criteria in the sub-agent
   brief MUST include this verifier.

## Anti-patterns

### Anti-pattern 1: Prose-form contract
> "The endpoint should return the candidates with their stake and
> predicate results."

Diagnosis: not falsifiable. Two implementers will produce two different
JSON shapes both "matching" this description. Replace with a typed
JSON schema.

### Anti-pattern 2: Contract drift mid-implementation
Sub-agent encounters a contract that doesn't fit + invents an
extension on the fly.

Diagnosis: orchestrator's contract was wrong — but silent extension
breaks the locked-contract guarantee. Sub-agent should HALT (per
Section 8 of the standard brief — Abort/Escalation Protocol) and
return `STUCK: contract C2 cannot accommodate <reason>`. Orchestrator
revises the contract + re-dispatches.

### Anti-pattern 3: Contract too narrow
Contract specifies only the happy path, not error cases.

Diagnosis: orchestrator did half the work. Each contract MUST specify
the full output domain (success cases + error cases) so sub-agents
don't invent inconsistent error shapes.

### Anti-pattern 4: Unlocked contracts in parallel dispatch
Orchestrator dispatches 3 parallel Implementers without locking
contracts first.

Diagnosis: invention drift guaranteed. The orchestrator is gambling
that 3 agents will independently invent compatible shapes. They won't.

## Case study: PR #121 (TuringOS Polymarket integration)

Phase 2 locked 5 contracts (C1-C5):

- **C1**: `WorkerPersona` struct (hardcoded const array, not runtime config)
- **C2**: `TournamentOutcome` aggregate shape (initially included a
  `winner_agent_id` field that Phase 5 adversarial review caught as
  shadow-ledger violation — the orchestrator REVISED C2 in Phase 6 + re-
  dispatched the Revision agent)
- **C3**: HTTP API contract — `GET /api/market/by-session/:session_id`
  JSON shape (the contract Frontend Implementer consumed without ever
  reading Backend Implementer's code)
- **C4**: Frontend custom-element contract — `<tos-agent-attempts-panel>`
  attribute spec
- **C5**: Genesis TOML schema — `[treasury]` + `[worker_wallets]` table
  shape

Backend Implementer (opus, max thinking) + Frontend Implementer (sonnet,
medium thinking) ran in PARALLEL, each holding the full text of C1-C5.
At merge time, the JSON shape backend emitted was byte-exactly what
frontend consumed. Zero reconciliation pass needed.

When Phase 5 critics revealed C2 was a shadow ledger, the orchestrator
revised C2 — NOT the implementations. The Revision agent then re-
implemented to the new contract. This is the contract-lock pattern
working as designed: contracts are the single source of truth across
agents.

## Verifier patterns

Each contract gets a deterministic check in the Acceptance Criteria
section of sub-agent briefs:

- **Rust struct contract**: `serde` roundtrip test — serialize the
  struct, deserialize, assert byte-equal
- **JSON shape contract**: integration test that calls the endpoint
  + asserts every required field is present with the right type
- **HTTP route contract**: route registration test — verify the path
  + method exists in the router
- **DOM API contract**: custom-element test asserting the attribute
  parses correctly + the right events fire
- **TOML schema contract**: TOML parse test asserting all required
  tables + fields are present

## Relationship to SKILL.md

`./SKILL.md` Phase 2 (Contract-Architect locks interface contracts)
references this file. The orchestrator, before dispatching Phase 3
parallel Implementers, writes the locked-contracts block per this
file's format. Each Implementer brief includes the locked contracts
verbatim. Each Implementer's Acceptance Criteria includes the
verifier per the contract type.

## When NOT to lock contracts

- Single Implementer task, no parallel agents, no consumers
- Pure read-only research (Phase 1) — Researchers produce reports,
  not contracts
- Trivial additive change where the existing contract is unchanged

## Related skills

- `./SKILL.md` — the orchestrator workflow that invokes this at Phase 2
- `./plan-grill.md` — for resolving ambiguous contract choices via
  fact-based clarification with the human
- (External) `skills/SUBAGENT_HARNESS.md` — predecessor harness skill;
  this file extracts the contract-lock mechanism that was implicit
  there
