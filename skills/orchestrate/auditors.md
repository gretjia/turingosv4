---
name: auditors
description: Menu of audit archetypes for Phase 5 of orchestrate. Pre-defined starting set (Constitution / Karpathy / Security / Performance / UX / Accessibility / API-Contract / Data-Integrity / Test-Coverage / Cost-Budget / User-Simulator). Orchestrator picks 1-N based on task domains AND/OR invents new archetypes per the 4-step protocol. NOT a closed set — explicitly alive.
allowed-tools: []
auditor-archetypes: [Constitution, Karpathy, Security, Performance, Accessibility, UX, API-Contract, Data-Integrity, Test-Coverage, Cost-Budget, User-Simulator]
extensible: true
---

# Auditors — the alive menu for Phase 5

This file is a sub-protocol auto-consulted by `./SKILL.md` at Phase 5
(Adversarial-Critic pass). End-users do not invoke it directly.
Orchestrators picking adversarial reviewers for the current task consult
this file.

## Core principle (the framing this file leads with)

> Phase 5 is **dynamic, not fixed**. Orchestrator picks 1-N auditors from
> the menu below based on what the task actually touches. If no
> pre-defined auditor fits the task's domain, **orchestrator invents a
> new auditor archetype** following the 4-step protocol at the end of
> this file. **The list below is a starting menu, NOT a closed set.**

A rigid "always run Constitution + Karpathy" pattern is the wrong
abstraction — it only works in narrow domains (governance + architecture).
Different domains need different lenses:

- A schema-migration PR needs Data-Integrity, not Karpathy.
- A user-facing UI PR needs UX + Accessibility, not Constitution.
- A new LLM-pipeline PR needs Cost-Budget + User-Simulator, not just
  Karpathy.
- A prediction-market mechanism PR needs Game-Theory (invent new
  archetype) on top of Constitution.

## Phase 5 dispatch decision protocol (how to pick)

When the orchestrator enters Phase 5, it MUST follow this protocol:

1. **List task domains** touched by the diff/artifact:
   governance / architecture / security / UX / data / API /
   performance / cost / accessibility / domain-specific (game theory,
   ML statistics, legal compliance, ...)
2. **Match each domain to an auditor**: pick the archetype from the
   menu below; OR invent a new one per the 4-step protocol if no
   archetype fits the domain.
3. **Minimum 2 auditors** (the dual-gate property).
   **Maximum 5** (beyond that is over-instrumenting and the orchestrator
   loses signal in mushy critique).
4. **All auditors run IN PARALLEL** — single message, multiple Agent tool
   dispatches. Same DELIBERATIVE tier for all of them.
5. **Each auditor's brief MUST include the anti-overlap clause** from
   the archetype below — prevents two critics opining on the same axis
   (e.g., both critiquing "naming" when only one was supposed to).

## Pre-defined auditor archetypes (~11 to start)

Each archetype documents:
- **Domain trigger** — when to pick this one
- **Attack vectors** — 5-7 specific things this auditor scans for
- **Output structure** — what their report looks like
- **Anti-overlap clause** — what this auditor explicitly does NOT
  comment on
- **Concrete example finding** — from a real shipping cycle

---

### 1. Constitution Auditor

**Domain trigger**: task touches governance / domain rules / monetary
law / conservation invariants / shielding / canonical state structure /
restricted surface.

**Attack vectors**:
1. Restricted surface modified without explicit per-atom sign-off
2. Money path uses non-integer math (f64/f32 in MicroCoin flows)
3. Canonical state has a shadow / memory-only / dashboard-only
   second source of truth
4. Conservation invariant violated (sum-of-X must equal Y, but new
   code lets it drift)
5. Shielding boundary breached (one agent reads private data of
   another agent against the spec)
6. Class classification mis-stated (Class-3 work classified as Class-2)
7. Reconstructability lost — canonical state cannot be replayed from
   tape + CAS

**Output structure**: restricted verdict domain
```
{ NO-VIOLATION,
  VIOLATION-FOUND <clause> <file>:<line>,
  RECONSTRUCTION-FAILURE <which-path>,
  SECOND-SOURCE-DRIFT <which-derived-view> }
```
Plus enumerated findings (open list) backing the verdict.

**Anti-overlap clause**:
> Constitution Auditor does NOT comment on code style, performance,
> test coverage, naming aesthetics, or architectural taste. Those are
> the domains of other auditors. Constitution comments ONLY on whether
> the diff violates documented domain rules.

**Concrete example**: PR #121 first pass — Constitution Auditor flagged
"SECOND-SOURCE-DRIFT: `market_view.rs:2` docstring claims pure projection
over `transition_ledger + EconomicState`, but file reads only pre-existing
CAS capsules + hardcoded constants" — file:line citation with clause
reference. Forced revision.

---

### 2. Karpathy Architect Auditor

**Domain trigger**: task introduces new abstractions, new modules, new
patterns; refactor; first impl of a new feature; orchestrator suspects
over-engineering.

**Attack vectors**:
1. Core Illusion test — can the system be described in ONE sentence?
   If not, ceremony has accumulated.
2. Data shapes vs logic — are the new structs PROJECTIONS over existing
   state, or are they DUPLICATING / CACHING state?
3. Micro-implementation test — what's the smallest e2e that proves the
   core loop? Is the diff close to that micro, or 3× too big?
4. Anti-pattern scan — `Manager` / `Factory` / `Engine` / `Platform` /
   `Framework` in names; generic plugin framework for one adapter;
   second truth source.
5. Fake future extensibility — `_v1` / `_v2` / `PR1_*` / temporal
   namespacing; "for future use" knobs that aren't exercised yet.
6. Antifragility — if the binary restarts mid-flow, what survives?
   Can replay reconstruct?
7. Single source of truth — is "this is the winner" stored once, or
   derivable from multiple paths?

**Output structure**: verdict from
```
{ SHIP-AS-IS, SIMPLIFY-FIRST, RESTART-FROM-MICRO }
```
Plus a Karpathy MetaAI checklist filled in (Core Illusion / Core data
shapes / Micro e2e / Single source of truth / Physical bottleneck /
Why not fake future extensibility / Runtime truth boundary).

**Anti-overlap clause**:
> Karpathy Auditor does NOT comment on security, accessibility,
> compliance, performance numbers, or domain-specific correctness.
> Those are other auditors' domains. Karpathy comments ONLY on
> architectural simplicity + anti-pattern + fake-extensibility.

**Concrete example**: PR #121 first pass — Karpathy returned
"RESTART-FROM-MICRO" with 5 hard rejections including `Pr1*` temporal
namespacing + 444-LoC custom element for 1 card + 170-LoC hand-rolled
TOML parser when `toml` crate was in tree.

---

### 3. Security Auditor

**Domain trigger**: task touches auth, crypto, secret handling, API
boundaries, user input, file uploads, network surface, permission
checks.

**Attack vectors**:
1. Secrets in logs / commits / config files / URL parameters
2. Auth check missing or wrong scope (privilege escalation)
3. SQL / shell injection on user input
4. XSS on rendered HTML / unescaped user-provided strings
5. CSRF / replay attack on state-changing endpoints
6. Crypto misuse (weak algorithm, hardcoded IV, no key rotation)
7. SSRF / file-path traversal / unsafe deserialization
8. Permission boundary breach (cross-tenant data leak)

**Output structure**: list of findings, each with
```
- severity: { Critical, High, Medium, Low, Informational }
- CWE class
- file:line
- repro steps
- remediation
```

**Anti-overlap clause**:
> Security Auditor does NOT comment on naming, architecture, or
> performance unless they directly enable an exploit. Style critique
> belongs in Karpathy. Performance critique belongs in Performance
> Auditor.

**Concrete example**: For a Polymarket integration — would flag that
worker `agent_id` is generated from `Ed25519Keypair::generate_with_secure_entropy()`
once per call without verifying the keypair is signed by a pinned root,
opening a "anyone can claim to be worker-alpha" vector.

---

### 4. Performance Auditor

**Domain trigger**: task touches a hot path; introduces new LLM calls /
DB queries / sync I/O; processes >1MB of data; new caching layer; new
parallelism.

**Attack vectors**:
1. N+1 patterns (loop with per-iteration DB query / LLM call)
2. Sync I/O on async hot path
3. Unbounded memory growth (Vec push without cap, in-memory cache
   without eviction)
4. Wasted LLM calls (separate triage + meta calls when one combined
   prompt would do)
5. Latency budget breach (named latency target exceeded by new code
   path)
6. Token-budget breach (LLM prompt grew past model context window)
7. Cache invalidation broken (stale reads after writes)
8. Cold-start cost (binary startup now includes new heavy init)

**Output structure**: per-finding
```
- hot path: <named code path>
- baseline: <observed metric>
- after change: <observed metric>
- budget: <named budget>
- breach: yes/no
- mitigation
```

**Anti-overlap clause**:
> Performance Auditor does NOT comment on correctness, architecture,
> or security. Only quantifiable performance budgets and hot paths.

**Concrete example**: For a multi-agent dispatch — would flag that 6
parallel Critic dispatches with DELIBERATIVE tier each = 6 × Opus-class
inference cost = ~$X per ship; recommend dropping to 2 Critics OR
demoting 4 to STANDARD tier.

---

### 5. Accessibility Auditor

**Domain trigger**: any user-facing artifact (HTML / mobile / desktop
UI / printed material / videos).

**Attack vectors**:
1. WCAG 2.x contrast ratio failures (text/background, focus indicator)
2. Keyboard navigation broken (tab order, focus traps, no-mouse
   workflow)
3. Screen reader incompatibility (missing alt text, aria-label,
   semantic HTML)
4. Touch target size < 44×44 px (mobile)
5. Color-only information conveyance (red/green status with no icon)
6. Animation without `prefers-reduced-motion` opt-out
7. Form errors not associated with their fields
8. Modal traps without escape key + close-button

**Output structure**: per-finding
```
- WCAG criterion: <e.g., 1.4.3 Contrast Minimum>
- severity: { Critical, Major, Minor, Cosmetic }
- file:line + selector
- repro: how a screen-reader user / keyboard user / color-blind user encounters it
- remediation
```

**Anti-overlap clause**:
> Accessibility Auditor does NOT comment on visual aesthetic,
> brand guidelines, or copy tone (those belong in UX). Only WCAG-
> based accessibility per documented criteria.

**Concrete example**: For a TuringOS spec-view HTML page — would flag
that the side panel uses ✅/❌ emoji as the ONLY indicator of L4 state
(no `aria-label`, no text alternative); screen-reader users get
"check mark emoji" or nothing.

---

### 6. UX Auditor

**Domain trigger**: any user-facing flow / artifact / interaction;
new endpoint that users see; UI redesign; copy / tone / messaging
changes.

**Attack vectors**:
1. Does the user UNDERSTAND what just happened? (phenomenology)
2. Flow friction — too many clicks, hidden state, surprising defaults
3. Error recovery — when something fails, can the user retry / undo?
4. Cognitive load — does the user have to remember state across
   screens?
5. Empty state / loading state / error state all designed?
6. Copy clarity — does jargon match the user's vocabulary?
7. Trust signals — does the user know the system did the work, or
   does it feel like magic?

**Output structure**: per-finding
```
- friction moment: <named user moment>
- what the user experiences: <one sentence>
- what they'd expect: <one sentence>
- alternative: <one concrete suggestion>
```

**Anti-overlap clause**:
> UX Auditor does NOT comment on WCAG accessibility (that's Accessibility
> Auditor) or technical implementation details. UX comments on the
> phenomenology of the user experience.

**Concrete example**: For the agent-attempts side panel — User-Simulator
real-user test caught that the panel displayed `winner_agent_id: "worker-alpha"`
WHILE `market_state: "open"`. UX Auditor (had it run) would have caught
this semantically: "user reads 'market is open' and 'winner is X' as
contradictory."

---

### 7. API-Contract Auditor

**Domain trigger**: task changes any public API surface — HTTP routes,
RPC signatures, library exports, wire schemas, CLI flags.

**Attack vectors**:
1. Backward compatibility — does this break existing callers?
2. Versioning strategy — bumped major / minor / patch correctly?
3. Schema migration path — old clients can still parse new responses?
4. Deprecation policy — old API still works during transition?
5. Wire format stability — JSON field order, enum values, error codes
6. Idempotency — repeated calls produce same result for state-changing
   endpoints?
7. Error contract — error responses follow documented schema?

**Output structure**: list of breaking changes + migration plan per
breaking change.

**Anti-overlap clause**:
> API-Contract Auditor does NOT comment on the API's semantics or
> usefulness — only on whether the diff breaks existing consumers
> in a way the version-bump doesn't cover.

**Concrete example**: If a new PR changed `GET /api/spec/turn` response
from `{ question_text: string }` to `{ question: string }`, API-Contract
Auditor flags "breaking rename without major-version bump + no
backward-compat shim".

---

### 8. Data-Integrity Auditor

**Domain trigger**: task touches a data store, schema migration, ledger
append, CAS write, conservation law.

**Attack vectors**:
1. Schema migration safety — can run on live data without downtime?
2. Conservation laws — total-X invariant violated after migration?
3. No shadow state — derived view doesn't usurp canonical state
4. Idempotent writes — retry-safe at the storage layer
5. Foreign-key / referential integrity — orphan rows possible?
6. Audit-trail intactness — can replay reconstruct history?
7. Backward-compat read — old code can still read new data?

**Output structure**: per-finding
```
- invariant: <named conservation / referential / replay property>
- diff that breaks it: file:line
- test that would have caught it
```

**Anti-overlap clause**:
> Data-Integrity Auditor does NOT comment on query performance (that's
> Performance), API surface (that's API-Contract), or constitutional
> rules (that's Constitution). Only data-store correctness.

**Concrete example**: For a schema-migration PR — would flag that a
new column with `NOT NULL` constraint added without DEFAULT value =
migration fails on existing rows.

---

### 9. Test-Coverage / Falsifiability Auditor

**Domain trigger**: every PR (this one runs by default for non-trivial
changes); test additions / changes / removals.

**Attack vectors**:
1. Does the test prove the CLAIM, or just that the function runs?
2. Are gates actually falsifiable — could the test ever fail?
3. Test asserts stderr substrings (anti-pattern) vs asserts state
4. Test mocks the very thing being tested
5. Cold-restart / replay assertions present for canonical state?
6. Grep-negative-pattern checks present (e.g., "no `Pr1_*` symbols
   remain")?
7. Acceptance criteria → test mapping is 1:1?

**Output structure**: per-finding
```
- claim in the brief: <quoted>
- test that purports to verify it: <file:line>
- gap: <one sentence>
- falsifier: <a concrete change to the test that would catch a regression>
```

**Anti-overlap clause**:
> Test-Coverage Auditor does NOT comment on what the test is testing
> being correct (that's other auditors). Only on whether the test
> actually proves what it claims.

**Concrete example**: For PR #121 first pass — would flag that the e2e
test `generate_emits_work_tx_smoke` asserted stderr substring `[polymarket-pr1] WorkTx admitted`
instead of asserting `<workspace>/runtime_repo` contains the L4 entry.
Stderr can be printed without admission ever happening; the test was
not falsifiable.

---

### 10. Cost-Budget Auditor

**Domain trigger**: multi-agent dispatch / multi-LLM workflow / new
inference call introduced / new external API integration / batch job.

**Attack vectors**:
1. Token cost — per-invocation budget vs justification
2. Latency cost — wall-clock cost of dispatching N agents in parallel
3. Orchestrator attention cost — does this require a human to babysit
   N rounds?
4. Cold-start cost — does each invocation re-initialize heavy state?
5. Failure recovery cost — if 1 of 3 parallel agents fails, what's the
   redispatch cost?
6. N-agent ceremony — is N=3 worth 3× cost vs N=1 + explicit "deferred
   to next PR" doc?

**Output structure**: per-phase budget table
```
| Phase | Agents | Tier | Token cost | Latency | Justification |
| 1     | 3      | STANDARD | ~9k | ~30s | research breadth |
| 3     | 2      | DELIBERATIVE | ~16k | ~120s | implementation |
| 5     | 4      | DELIBERATIVE | ~32k | ~120s | adversarial breadth |
```
+ verdict: `{ COST-JUSTIFIED, OVER-INSTRUMENTED, UNDER-INSTRUMENTED }`

**Anti-overlap clause**:
> Cost-Budget Auditor does NOT comment on whether the work itself is
> valuable. Only on the cost/value ratio given the named budget.

**Concrete example**: For a "let's run 6 critics on this PR" plan —
would flag "OVER-INSTRUMENTED: 6 critics × Opus-class = ~6× cost of
2 critics; the additional 4 critics would surface marginal findings the
top 2 already cover. Drop to 2 OR demote 4 to STANDARD tier."

---

### 11. User-Simulator Auditor (the ONLY one that catches real-API edge cases)

**Domain trigger**: any user-facing artifact has shipped; integration
involves an external LLM / external service / real-user input.

**Distinct from in-session Adversarial-Critics**: runs against the
DEPLOYED artifact, NOT the diff. Real LLM calls, real user input, real
environment.

**Attack vectors**:
1. LLM API edge cases — token budget truncation, CoT vs JSON-only
   modes, provider-specific error formats, rate limits
2. Real user input — adversarial input (XSS, SQL, ridiculous Unicode),
   minimal input, maximal input, empty input
3. Multi-step flow — does the user reach the artifact via the intended
   path?
4. Cross-browser / cross-device variance (if web)
5. Cold-start UX — first-time user with no prior context
6. Config drift — workspace config vs runtime config mismatch
7. Real LLM behavior — does the system survive a real (non-mocked) LLM
   round-trip?

**Output structure**: phenomenology report
```
- Setup: <real-environment setup>
- Real flow attempted: <step-by-step>
- Real outcome: <what actually happened>
- Bugs caught (severity + repro)
- Bonus: structural correctness checklist (subset of Adversarial-Critic findings,
  verified against real environment not diff)
```

**Anti-overlap clause**:
> User-Simulator Auditor reports REAL-environment findings only. In-
> session critics handle diff-level findings. Don't re-litigate
> findings the Adversarial-Critics already raised — focus on what
> couldn't be seen from reading the code.

**Concrete example**: PR #121 — User-Simulator with a real DeepSeek
API call caught: `cmd_llm.rs:1386` triage `max_tokens=50` is incompatible
with DeepSeek V4 Pro (which generates ~294 tokens of chain-of-thought
before its JSON output, so the 50-token cap truncates → parse_failed:
EOF → kernel injects nudge → grill stuck at Q1 forever for any real user).
No diff-level critic could see this — it requires real API calls.

---

## How to invent a new auditor (4-step protocol)

When the orchestrator's task touches a domain none of the 11 pre-defined
auditors cover (e.g., "game-theory soundness of a prediction-market
mechanism", "statistical robustness of an ML pipeline", "legal
compliance of a data-collection flow", "internationalization of a
multi-locale UI"), DO NOT force-fit an existing auditor. Invent a new
archetype using this 4-step protocol:

### Step 1 — Name the domain

One phrase, no ceremony. The name must be a NOUN PHRASE that describes
what the auditor LOOKS AT, not what it OPINES ABOUT.

Good names:
- "Game-Theory Auditor" (looks at mechanism design)
- "Statistical-Robustness Auditor" (looks at ML pipeline stability)
- "I18n Auditor" (looks at locale handling)

Bad names:
- "Quality Auditor" (too vague — quality of what?)
- "Best-Practice Auditor" (subjective; not a domain)
- "ImprovementSuggester" (verb; not a noun phrase)

### Step 2 — List 5-7 attack vectors

Each must be FALSIFIABLE. A yes/no/quantified answer must exist.

Good attack vector:
- "Mechanism is gameable by self-trading (one agent trades both sides
  of a market it participates in)"

Bad attack vector:
- "Mechanism feels right" (un-falsifiable)
- "Code is well-organized" (subjective; not a falsifiable test)

### Step 3 — Define output structure

Decide:
- **Verdict token** (closed set) vs **enumerated findings** (open set)
  — usually open for in-session Critics, closed for Witness
- **Per-finding fields** — severity? location? remediation? repro?
- **Restricted output domain** if applicable (don't let the auditor
  drift into adjacent domains)

### Step 4 — List anti-overlap clauses

What this auditor explicitly does NOT comment on. Cross-reference
adjacent auditors:

```
> Game-Theory Auditor does NOT comment on architecture quality
> (that's Karpathy's domain), on monetary conservation (that's
> Constitution's domain), or on test falsifiability (that's
> Test-Coverage's domain). Only mechanism-design gameability.
```

### After invention

Construct the new auditor's brief using the standard 9-section sub-agent
template from `./SKILL.md`. Dispatch in parallel with the other auditors
chosen for Phase 5. If the new auditor produced value, the orchestrator
MAY save the new archetype back into this file (this list is alive)
via a follow-up PR — community contribution loop.

## Case studies (3 real + 1 invented)

### Case 1: Polymarket integration (governance + architecture)

Domains touched: governance (Constitution), architecture (Karpathy).
Auditors picked: Constitution + Karpathy. Both DELIBERATIVE tier, both
opus + max-thinking equivalent on Claude. Both REJECTED first pass with
different lenses (Constitution: SECOND-SOURCE-DRIFT + RECONSTRUCTION-
FAILURE; Karpathy: RESTART-FROM-MICRO). Orchestrator triaged + revised.

### Case 2: Spec HTML view PR — what we MISSED

Domains touched: user-facing artifact (UX + Accessibility), visual
design. Auditors that SHOULD have been picked: UX + Accessibility.
Actually picked: just Karpathy. Result: Karpathy flagged ceremony (444
LoC for one card) but completely missed:
- No `aria-label` on status emojis
- No keyboard focus order on the side panel
- No `prefers-reduced-motion` opt-out for polling animations
- No reduced-color-vision fallback (winner-crown 👑 alone is the only
  signal)

Documented as a pattern gap. Future user-facing PRs MUST include UX +
Accessibility auditors at minimum.

### Case 3: Multi-PR Polymarket sequence — Cost-Budget that should have run

Domain touched: multi-PR planning, multi-agent workflow, LLM cost.
Auditor that SHOULD have been picked: Cost-Budget. Actually picked:
none. Result: orchestrator planned N=3 fan-out for PR2 without
quantifying the 3× LLM-cost-per-Generate-call vs the marginal value
of running 3 workers vs 1. Cost-Budget Auditor would have asked:
"is the 3× cost worth it for PR2's expected value, given PR3's
ChallengeTx is what really exercises multi-agent? Maybe PR2 stays
N=1 and PR3 jumps directly to N=3+critic."

### Case 4: (Invented) Game-Theory Auditor

Domain: prediction-market mechanism design. None of the 11 pre-defined
auditors cover this:
- Constitution knows monetary conservation but not strategic gaming
- Karpathy knows architecture but not Nash equilibria
- Security knows auth boundaries but not market manipulation

Invented archetype:

```
Name: Game-Theory Auditor

Attack vectors:
1. Rope-a-dope: can one agent profit by trading against itself?
2. Pseudo-diversity: are the N participating agents truly independent
   samples, or correlated (e.g., 3 personas of one LLM)?
3. Orchestrator collusion: same entity picks participants AND scores
   outcomes — incentive alignment risk?
4. Front-running: can an observer profit by seeing pending tx before
   they admit?
5. Bribery / side-channel: can payouts be redirected via non-on-chain
   coordination?
6. Endgame instability: does the market settle at a Schelling point or
   oscillate?
7. Entropy floor: minimum diversity required for "wisdom of crowds"?

Output: enumerated findings + concrete gaming exploit per finding +
mitigation.

Anti-overlap clause:
> Game-Theory Auditor does NOT comment on monetary conservation
> (Constitution) or architectural simplicity (Karpathy). Only
> strategic gaming + mechanism-design soundness.
```

This is exactly the auditor PR #121 needed. Without it, the Constitution
agent flagged "self-trading possible" as a generic concern; a dedicated
Game-Theory auditor would have enumerated specific exploits.

## When NOT to invent

- Domain is already covered by a pre-defined auditor (don't proliferate
  archetypes)
- Task is small enough that 2 pre-defined auditors suffice
- The "invented" auditor's attack vectors are not falsifiable (it's
  taste critique, not audit)

## Relationship to SKILL.md

`./SKILL.md` Phase 5 dispatches 2-5 auditors picked from this file
AND/OR invented per the 4-step protocol. Each auditor's brief embeds:
- The archetype's attack vectors (as the brief's "what to scan for")
- The archetype's output structure (as the brief's Section 9 Final
  Report Format)
- The archetype's anti-overlap clause (so the auditor stays in its lane)

## Related skills

- `./SKILL.md` — the orchestrator workflow that invokes this
- `./plan-grill.md` — for resolving authorization questions after Phase 5
  findings (Phase 6 triage)
- `./interface-contract-lock.md` — locked contracts are what auditors
  audit against
