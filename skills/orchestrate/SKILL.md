---
name: orchestrate
description: One-entry multi-agent orchestration workflow. User invokes /orchestrate; the skill internally fires plan-grill for human disambiguation, interface-contract-lock at Phase 2, swarm dispatch at Phase 3, adversarial dual review at Phase 5, shipping-witness audit at Phase 7 — all automatic. Use whenever a task warrants >1 sub-agent or needs dual review before ship. Cross-platform (Claude / Codex / Gemini / any orchestrator).
allowed-tools: ["*"]
roles: [Researcher, Implementer, Contract-Architect, Adversarial-Critic, Shipping-Witness, User-Simulator]
tiers: [BRIEF, STANDARD, DELIBERATIVE]
sub-protocols:
  - ./plan-grill.md
  - ./interface-contract-lock.md
  - ./auditors.md
references:
  - ./references/claude.md
  - ./references/codex.md
  - ./references/gemini.md
  - ./references/generic.md
---

# Orchestrate — Cross-platform multi-agent orchestration

Use this skill when a task is too big or too critical for a single agent
pass, AND the work benefits from **role-specialized sub-agents** + **dual
adversarial review** + **single witness audit** before ship.

This skill is the **ONE user-facing entry**. Sub-protocols
(`plan-grill.md`, `interface-contract-lock.md`, `auditors.md`) are auto-invoked
by the orchestrator at the right phase — the user never needs to invoke them
manually.

## Background

This pattern was derived from a real shipping cycle (PR #121, TuringOS
Polymarket integration, 2026-05-23) where ~15 sub-agents in carefully shaped
roles produced a CI-green, adversarial-review-passed ship — AND a real-user
e2e test surfaced bugs every prior layer had missed. The pattern is what
made that loop converge AND honestly documented what it still didn't catch.

The pattern is platform-agnostic. It uses **role names** (what the agent
produces) and **capability tiers** (process-framed depth), not model
names. A platform adapter file in `./references/<platform>.md` maps tiers
to actual model identifiers on each platform — Claude, Codex, Gemini, or
any future orchestrator runtime.

## Core principles (read once, internalize, do not skip)

1. **One user entry.** The user invokes `/orchestrate` and nothing else.
   Sub-protocol files are reached transitively. The orchestrator never
   asks the user "now invoke plan-grill" — it just does it.

2. **Role names describe OUTPUT shape, not seniority.** A `Researcher`
   produces a read-only investigation report. A `Critic` enumerates
   violations. A `Witness` adjudicates with a single token. If two roles
   produce the same output shape, they should be the same role.

3. **Tiers are process-framed, not capability-framed.** `BRIEF /
   STANDARD / DELIBERATIVE` survives model churn. Today's DELIBERATIVE is
   tomorrow's STANDARD.

4. **Interface contracts are LOCKED before dispatch.** If 3 implementer
   agents are running in parallel, they must build to a contract the
   orchestrator wrote BEFORE dispatching them. Otherwise they diverge.
   See `./interface-contract-lock.md`.

5. **Disambiguation ≠ excavation.** When the orchestrator hits a fact /
   decision-fork it can't resolve, fire `./plan-grill.md` (fact-based
   options + trade-offs). DO NOT psychologically probe the user — that's
   a different skill for a different domain (eliciting end-user specs).

6. **Adversarial review is alive, not fixed.** Phase 5 dispatches 2-5
   auditors selected from `./auditors.md` based on what domains the task
   actually touches — AND the orchestrator may invent new auditor
   archetypes when no pre-defined one fits. The auditor menu is a
   starting menu, NOT a closed set.

7. **Critic enumerates; Witness adjudicates.** Phase 5 critics may
   return 12 findings each. Phase 7 witness returns ONE token from a
   restricted closed set. Collapsing the two loses the dual-gate
   property.

8. **Honest about failure modes.** A documented "what this pattern does
   NOT catch" beats a polished claim of completeness. See "Failure
   modes" below.

## Role vocabulary (6 roles)

| Role | Output shape | Typical dispatch |
|---|---|---|
| **Researcher** | Read-only investigation report | 1-3 in parallel at Phase 1 |
| **Contract-Architect** | Locked interface contracts (code-form) | Usually = orchestrator at Phase 2 |
| **Implementer** | Artifact + diff summary | 1-3 in parallel at Phase 3 (never >3 for one cohesive deliverable) |
| **Adversarial-Critic** | Enumerated violations (open domain) | 2-5 in parallel at Phase 5, DELIBERATIVE tier |
| **Shipping-Witness** | Single verdict token from restricted closed set | 1 sequential at Phase 7, DELIBERATIVE tier |
| **User-Simulator** | E2E phenomenology report + UX critique | Post-ship; the ONLY role that catches real-API edge cases |

Sharpening rule: **Critic enumerates; Witness adjudicates.** If they
collapse into the same brief, the dual-gate property is lost.

## Tier vocabulary (3 tiers, process-framed)

- **BRIEF** — fast reactive, narrow scope, ≤200 LoC-equivalent artifact, no
  cross-cutting constraints. Implementer dispatched to a single file.
- **STANDARD** — multi-constraint implementation, 200-800 LoC-equivalent,
  needs interface contracts but no cross-system reasoning.
- **DELIBERATIVE** — architecture decisions, adversarial review, complex
  orchestration, cross-system reasoning, irreversible action.

Inline platform mapping (per `./references/<platform>.md` for full detail):

| Tier | Claude | Codex | Gemini | Generic fallback |
|---|---|---|---|---|
| BRIEF | Haiku | gpt-mini class | Flash | platform-fastest |
| STANDARD | Sonnet | gpt-balanced | Pro | platform-medium |
| DELIBERATIVE | Opus / Sonnet+thinking-max | o-series / gpt-pro | Pro-thinking-on | platform-deepest |

**Dynamic, not fixed.** The orchestrator picks tier based on task
difficulty assessed at Phase 0 — not by a fixed lookup table.

## Phase ordering (canonical)

```
Phase 0 (orchestrator only): Triage
  Decision tree:
  - Single artifact, <300 LoC-equivalent, no cross-cutting constraints
    → skip swarm; 1 Implementer (BRIEF) + 1 Critic (STANDARD)
  - Research-only, no artifact
    → Researchers + synthesis, skip Phases 3-7
  - Multiple artifacts OR cross-cutting constraints OR irreversible action
    → full pattern below
  ⚠ If task difficulty / authorization / scope CANNOT be classified by the
    orchestrator alone → invoke ./plan-grill.md to disambiguate with the
    human BEFORE proceeding. (Plan-grill is fact-based, NOT psychological
    excavation.)

Phase 1: Researcher pass (1-3 parallel, BRIEF or STANDARD tier)

Phase 2: Contract-Architect locks interface contracts.
  See ./interface-contract-lock.md for the locked-contracts pattern.
  The orchestrator (acting as Contract-Architect) writes contracts in
  code-form (struct fields, JSON shapes, HTTP routes, etc.) — NOT in
  prose — BEFORE dispatching Implementers.

Phase 3: Implementer pass (1-2 parallel; never >3 for one cohesive
         deliverable). Each Implementer gets the standard 9-section
         brief (template below).

Phase 4: Local verification gate (pinned generic checks + 1 domain check
         named at Phase 0 — load-bearing falsifiable gate).
         Generic checks vary by domain:
           - Code: compile + lint + type-check + format + secret scan
           - Docs: markdownlint + link integrity + YAML frontmatter parse
           - Data: schema validation + invariant assertions
         The "1 domain check" is task-specific and named explicitly at
         Phase 0 so it's falsifiable, not a moving target.

Phase 5: Adversarial-Critic pass (2-5 parallel, DELIBERATIVE tier).
  ⚠ Auditor selection is DYNAMIC. Consult ./auditors.md for the menu
    of pre-defined archetypes (Constitution / Karpathy / Security /
    Performance / Accessibility / UX / API-Contract / Data-Integrity /
    Test-Coverage / Cost-Budget / User-Simulator) AND/OR invent new
    archetypes per the 4-step protocol in that file.
    The list is alive, NOT closed.
  Orchestrator picks based on task domains:
    - governance / monetary law → Constitution
    - architecture / new abstractions → Karpathy
    - user-facing artifact → UX + Accessibility
    - API surface → API-Contract
    - data store → Data-Integrity
    - multi-agent workflow → Cost-Budget
    - touches user → User-Simulator (post-ship)
  Minimum 2 auditors; maximum 5 (beyond that is over-instrumenting).
  All auditors run IN PARALLEL (single message, multiple dispatches).

Phase 6: Orchestrator triage of findings
  Categorize each finding: must-fix | nice-to-fix | wontfix-with-reason.
  ⚠ If must-fix items require user-only decisions (authorization, scope
    extension, sudo bypass) → invoke ./plan-grill.md to resurface the
    trade-off cleanly before deciding.
  Then dispatch focused remediation OR orchestrator-edits-directly.

Phase 7: Shipping-Witness audit (1 agent, DELIBERATIVE tier, restricted
         verdict domain). Witness sees ONLY:
           - task brief + risk class
           - touched-FC-nodes / touched-domains
           - current diff
           - evidence paths
           - acceptance criteria + actual command outputs
         Witness returns ONE token from a closed set (see "Verdict domain"
         below).

Phase 8: Ship to PR
  If Witness verdict = clean: commit + push + open PR (per platform's
  PR-only workflow). Sub-agents NEVER merge — orchestrator does it in
  Phase 10.
  If Witness verdict = unresolved violation: STOP. Re-enter Phase 6.

Phase 9: Closing audit — MANDATORY before merge
  After CI passes + external review (Codex bot / human reviewers /
  AGENTS.md §9 default Codex audit) leaves comments AND orchestrator has
  addressed P1/P2 findings, fire a CLOSING audit team to verify the
  SHIPPED artifact still matches the approved plan + external feedback
  has been absorbed cleanly.

  ⚠ DEFAULT: REUSE the Phase 5 auditor team via context handoff —
    same auditors, same preserved context. The auditors already know the
    design rationale + trade-offs from Phase 5; reusing them eliminates
    re-onboarding cost and ensures consistency between "design we
    approved" and "ship we approved".
  See "Closing audit" section below for the dispatch protocol +
    distinct closing-mode verdict domain.
  ⚠ If closing audit returns IMPL-DRIFT-FROM-PLAN or EXTERNAL-FEEDBACK-
    REQUIRES-REVISION that needs user-only decision → invoke
    ./plan-grill.md to disambiguate before looping back.

Phase 10: Merge to main
  The orchestrator (NOT a sub-agent) executes the merge per the
  platform's PR-only workflow. Per AGENTS.md §14a, the legitimate-
  bypass `GIT_HARDEN_ALLOW_MAIN=1 git push origin main` is reserved
  for the orchestrator merging a vetted PR locally; sub-agents NEVER
  do this regardless of role or tier.

  Gate: ALL Phase 9 closing-audit verdicts MUST be READY-TO-MERGE.
  Any other verdict → loop back to Phase 6 (findings triage).
```

## Standard sub-agent prompt template (MANDATORY 9 sections)

Every sub-agent prompt MUST have these 9 sections, in this order:

### 1. Identity / Role

```
You are <Role> for <task>. Tier: <BRIEF|STANDARD|DELIBERATIVE>. Thinking
depth: <low|medium|max>. You MUST NOT exceed your role boundary:
<role-specific constraint>.
```

### 2. Brief

Problem + outcome + relevant user rulings + prior adversarial findings.
≤300 words. Self-contained — sub-agent has no context from prior turns.

### 3. Required reading

≤10 paths with explicit order. Include rationale next to each path
(why this one).

### 4. Hard constraints

Numbered DO-NOT list. Each numbered item ≤15 words.

```
1. Do NOT modify files outside <scope>.
2. Do NOT commit, push, or open PRs.
3. Do NOT invent new abstractions when <existing-utility> solves it.
...
```

### 5. Interface contracts

Locked data shapes in code-form (Rust struct / TypeScript interface /
JSON schema / etc.). NOT in prose. For complex multi-contract work, the
contracts are pre-locked in a separate orchestrator-written artifact
referenced here. See `./interface-contract-lock.md`.

### 6. Acceptance criteria

Each criterion = **exact command + expected output**. Vague pass/fail
is forbidden. Include grep-negative-pattern checks where applicable.

Example:
```
1. `cargo check --bin X --features Y` → exit 0
2. `cargo test --test foo` → "1 passed; 0 failed"
3. `grep -n "PR1_" src/` → returns ONLY the audit-trail comment in bootstrap.rs
4. JSON response shape: { agent_id: string, stake: number, l4_state: enum }
   tested via inline serde roundtrip
```

### 7. NOT IN SCOPE

Explicit deferral list. Each item gets a "deferred to <where>"
attribution.

### 8. Abort / Escalation Protocol

```
If any of these conditions hold, HALT and return:
  STUCK: <one-line reason> <what-you-tried>
Conditions:
  - Required reading file missing or empty
  - Interface contract cannot be satisfied as specified
  - Acceptance criterion is logically unprovable
  - Hard constraint conflicts with a required behavior
```

Without this section, sub-agents either spin forever or fabricate.

### 9. Final report format

Conditioned by **role**, not tier:

- **Researcher** → structured findings report (sections + citations)
- **Implementer** → artifact + diff summary
- **Critic** → enumeration of findings (open domain)
- **Witness** → single verdict token from closed set
- **User-Simulator** → e2e phenomenology + structural correctness checklist

The skill provides a template per role under `./references/<platform>.md`.

## Verdict domain (Shipping-Witness, Phase 7)

The Witness MUST return EXACTLY ONE token from a **domain-specific closed
restricted set** that the orchestrator declares in the brief at dispatch
time. Three SHAPE constraints are universal across all domains:

1. **Closed** — finite enumerated list; the Witness picks ONE; no
   inventing new tokens at response time
2. **Restricted** — NOT free-form enumeration; NOT a list of findings
3. **Non-subjective** — every token corresponds to a falsifiable
   property of the artifact, NOT to taste or preference

The orchestrator picks (or copies) the domain's closed set when
authoring the Witness brief. Subjective opinions ("I think the code
style …" / "Performance could be improved …" / "Architecture would be
better if …") are ALWAYS out-of-domain regardless of the chosen token
set — orchestrator REJECTS such responses and re-dispatches.

### Canonical token set — code / governance domain

For TuringOS-style code + governance work, use the 4-token set from
`AGENTS.md §14`:

- `NO-VIOLATION` — scanned N clauses, no violations found
- `VIOLATION-FOUND <clause> <file>:<line>` — specific clause cited
- `RECONSTRUCTION-FAILURE <which-path>` — state cannot be re-derived
- `SECOND-SOURCE-DRIFT <which-derived-view>` — derived view usurping
  ground truth

This is the default for code shipping in this repo; it maps directly to
the cadence table in AGENTS.md.

### Domain-specific closed sets — other domains derive their own

Outside code/governance, the orchestrator picks a closed set tailored to
the domain. The 4-token code set above does NOT translate. Concrete
domain-specific sets used in the case studies below:

- **Long-form writing**: `{ READY-TO-PUBLISH | NEEDS-ONE-MORE-PASS |
  RESTART-FROM-OUTLINE }` — three closed states gating publication
- **Literature synthesis**: `{ COVERAGE-OK | GAPS-FOUND |
  SOURCES-WEAK }` — three closed states gating sharing

Each domain-specific set follows the same SHAPE: closed, restricted,
non-subjective. The orchestrator's Phase 7 brief explicitly lists the
chosen tokens; the Witness picks ONE.

### How to derive a new domain's verdict set

When neither the canonical code set nor an existing case-study set
applies (e.g., a new domain like clinical-trial design / legal review),
follow these constraints:

1. **3-5 tokens** (fewer = under-discriminating; more = mushy)
2. **All-or-nothing decisions** the orchestrator gates on (e.g., "do we
   ship?" / "do we publish?" / "do we file?")
3. **Each token names a specific falsifiable artifact property**
4. **NO ambiguous middle tokens** (e.g., "MOSTLY-OK" is forbidden —
   either the gate condition holds or it doesn't)
5. **One token MUST mean "passes the gate"** — there's always a clean
   ship verdict

### Adversarial-Critic verdict (Phase 5) is OPEN, distinct from Witness

(Adversarial-Critic in Phase 5 has OPEN domain — Critics enumerate
N findings each. Only the final Witness in Phase 7 is restricted to a
single closed-set token. **Critic enumerates; Witness adjudicates.**)

## Closing audit (Phase 9) — mandatory team review before merge

Phase 7 Shipping-Witness audits the DIFF before the PR opens. Phase 9
Closing audit verifies the SHIPPED artifact AFTER the PR opens — once
CI has run, external review (Codex bot / human reviewers) has weighed
in, and orchestrator has addressed P1/P2 findings.

This is the LAST gate before merge to main. It is MANDATORY for any
ship-class work (Class ≥ 2 in TuringOS taxonomy; or any irreversible
public-facing ship in other domains).

### Default mode: REUSE the Phase 5 auditor team (cost-saving)

Cost / efficiency optimization (per user direction, 2026-05-23):

> 最后审计的这个 multiagent 团队和计划阶段的可以共用一个团队，也可以
> 共用一个 context。这样的话是最节省，也是效率最高的

The Phase 5 auditors already know the design rationale + trade-offs +
all the violations they previously raised + the orchestrator's
remediation. Reusing them at Phase 9:

- **Eliminates re-onboarding cost** — no need to re-explain the design
- **Ensures consistency** — same lens for "design we approved" and
  "ship we approved"
- **Surfaces drift cleanly** — the auditor that approved the design is
  the one best positioned to spot ship-time drift from it

#### Implementation on Claude Code (and any platform with persistent agent context)

- Phase 5 auditors run as background agents with stable agent IDs
- At Phase 9, `SendMessage` to each Phase 5 agent ID with the closing
  brief:
  - PR HEAD SHA + diff link
  - External review summary (Codex bot findings, human reviewer
    comments, CI logs of interest)
  - Orchestrator's responses to external feedback
  - Pinned reference to their Phase 5 findings + verdict
  - Request: closing-mode verdict from the restricted set below
- Each auditor returns a closing verdict in the closing-mode domain
  (different from Phase 5's open enumeration; see verdict domain below)

#### Implementation on platforms without persistent agent context

When the platform has no SendMessage / context-handoff primitive, OR
the Phase 5 agents have timed out / been garbage-collected:

- Save each Phase 5 agent's final output as a "planning-context bundle"
  (the orchestrator does this at the end of Phase 5)
- At Phase 9, spawn NEW auditors of the same archetypes briefed with
  the bundle as required reading + the closing-mode brief
- Cost: extra context re-load tokens. Benefit: auditors still don't
  re-derive the design rationale from scratch — they read the prior
  team's recorded reasoning

### Verdict domain (Closing audit) — distinct from Phase 7 Witness

Closing audit verdicts answer a different question than Phase 7 Witness:
"is the SHIP right per the original design + external feedback?", not
"is the diff structurally clean?".

Each closing-audit auditor returns ONE token from this closed set:

- `READY-TO-MERGE` — original concerns addressed + external feedback
  absorbed + no plan drift; auditor signs off on merge
- `EXTERNAL-FEEDBACK-REQUIRES-REVISION <which-feedback> <which-impact>` —
  external review surfaced an issue the planning team didn't anticipate;
  loop back to Phase 6 triage
- `IMPL-DRIFT-FROM-PLAN <which-axis>` — shipped diff doesn't match
  approved plan in some axis (named field / API contract / data shape
  drift); loop back to Phase 6
- `DEFERRAL-MISSING <what-was-deferred-but-not-documented>` — post-ship
  TODO list is incomplete; document deferrals before merge

Orchestrator gates Phase 10 (merge) on ALL closing-audit verdicts being
`READY-TO-MERGE`. Any other verdict from any auditor → loop back to
Phase 6.

### When to add NEW auditors at Phase 9 (override the reuse default)

Phase 5 reuse is the default but NOT mandatory. Add new auditors when:

- **External review raised a new domain.** Example: Phase 5 had
  Constitution + Karpathy auditors; external Codex bot review surfaced
  a UX concern. Phase 9 should add a UX auditor in addition to reusing
  Constitution + Karpathy.
- **Phase 5 used invented archetypes that aren't in the standard menu.**
  Those agents may be lost across sessions; spawn fresh equivalents
  briefed with the planning-phase bundle.
- **Significant time has elapsed.** If days passed between Phase 5 and
  Phase 9, agent context may have stale assumptions about the codebase.
  Spawn fresh agents with the bundle + a "current state of main" pre-read.

### Orchestrator's pre-Phase-9 self-check

Before firing closing audit:
1. [ ] CI is green on the PR HEAD
2. [ ] External review (Codex bot / human / AGENTS.md §9 audit) has
       completed
3. [ ] P1 / P2 findings from external review have been addressed
       OR explicitly deferred with rationale
4. [ ] PR body cites the original plan / charter / case studies the
       closing auditors will check against
5. [ ] If reusing Phase 5 agents: their context is still intact (test
       with a small SendMessage ping before the full closing brief)

## Acceptance criteria patterns (the "checkability test")

The single most leveraged mechanism. Every criterion MUST pair with
an exact, runnable check.

### Anti-patterns (forbidden)

- "Tests should pass" — vague
- "Implementation should be clean" — subjective
- "No regressions" — un-grep-able
- "Documentation is good" — un-falsifiable
- "Performance is acceptable" — undefined threshold

### Good patterns (examples)

1. **Exact command + expected exit code**:
   `cargo test --test constitution_polymarket_smoke` → exit 0, 7/7 PASS
2. **Pinned test name + state**:
   "5 critical regression tests STAY GREEN: [explicit list]"
3. **Grep-negative**:
   `grep -rn 'Pr1\|pr1_' src/` returns NO Rust symbol (only an audit-trail
   comment in one file)
4. **Artifact content check**:
   "PR body explicitly cites (a) X, (b) Y, (c) Z" — grep-able
5. **Pinned JSON shape**:
   Endpoint returns `{ session_id, market_state, candidates[] }` with all
   fields non-null; deterministic structural assertion

## Operating restrictions for sub-agents

Every Implementer brief MUST include:

- NO `git commit` (orchestrator does this in Phase 8)
- NO `git push`
- NO `gh pr create` / merge
- NO modifying files outside declared scope
- NO inventing new abstractions when existing utilities solve the problem
- NO temporal namespacing (e.g., `Pr1*`, `V2_*`) — that's fake future
  extensibility (Karpathy K6 lesson)

## Orchestrator pre-dispatch self-check (4 items)

Before dispatching ANY sub-agent, the orchestrator runs this checklist:

1. [ ] Interface contracts locked (or task is small enough to skip Phase 2)?
2. [ ] Acceptance criteria are grep-checkable (no vague pass/fail)?
3. [ ] Required reading list ≤10 items?
4. [ ] Abort protocol (Section 8) included in the brief?

If any unchecked → fix the brief BEFORE dispatching.

## Anti-patterns gallery

Five concrete bad-brief patterns observed in the field. Each entry: the
anti-pattern + the diagnosis.

### A. "We just want it to work"
Brief without acceptance criteria. Diagnosis: the orchestrator hasn't
done its job — every brief MUST translate "it works" into an exact
command + expected output.

### B. 6+ agents when 2 would suffice
Brief that dispatches 6 implementers for what is structurally a 2-PR
change. Diagnosis: agent-ceremony as proxy for thoroughness. 2 agents
with crisp contracts beat 6 with vague boundaries. Karpathy K8 lesson.

### C. Temporal namespacing in code (`Pr1_*`)
Brief that lets implementer name symbols by PR number / version stage.
Diagnosis: fake future extensibility. Symbols rot the moment the PR
merges. Use domain names.

### D. Open verdict domain on Witness
Phase 7 Witness brief without restricted output set. Diagnosis: Witness
returns 47 findings, orchestrator can't triage, ship is blocked on
mushy opinions. Witness MUST return one token from a closed set.

### E. Final report format unconditioned by role
Implementer returns Critic-style enumeration when artifact summary was
needed; Witness returns Implementer-style diff summary when one
verdict token was needed. Diagnosis: brief missed Section 9 role-
conditioning.

## Case studies

### Code case study: TuringOS Polymarket PR1 (2026-05-23)

Full lifecycle in one paragraph:

Phase 0 (Triage) ruled "multi-artifact + cross-cutting + irreversible" →
full pattern. Phase 1 dispatched 3 parallel Researchers (R1/R2/R3) for
grill recursive design + visual HTML + skill codification. Phase 2 locked
5 interface contracts (C1-C5) in code-form. Phase 3 dispatched 2 parallel
Implementers (Backend opus/max + Frontend sonnet/medium). Phase 4 ran the
5 critical regression tests + R-022 + cargo check. Phase 5 dispatched 2
Adversarial-Critics (Constitution + Karpathy, both DELIBERATIVE) — BOTH
REJECTED first impl (SECOND-SOURCE-DRIFT + RECONSTRUCTION-FAILURE + 5
hard rejections). Phase 6 triaged → revision agent rewrote (deleted
ephemeral kernel dance, used canonical sequencer factory). Phase 5 ran
again (single Codex-style Witness this time) → NO-VIOLATION. Phase 8
shipped PR #121, CI green. POST-SHIP User-Simulator caught 3 more bugs
(triage token-budget, winner_agent_id semantic contradiction, workspace
turingos.toml model name mismatch) — documented as patterns the
adversarial review missed.

What made it work: locked contracts in Phase 2 (no impl drift), dual
critic in Phase 5 (caught design violations), Witness in Phase 7
(single ship-gate token), User-Simulator post-ship (caught the API edge
cases nothing else saw).

### Writing case study: Long-form essay

Phase 1: 2 parallel Researchers gather sources from 2 different topic
angles. Phase 2: orchestrator locks essay structure as outline (H1 + H2s
+ thesis sentence per H2). Phase 3: 1 Implementer drafts to the outline.
Phase 5: 2 Critics dispatched — UX Auditor (cognitive load, flow
friction, reading level) + a domain-specific Fact-Check Auditor invented
per the 4-step protocol. Phase 7: Witness adjudicates against a
**domain-specific closed verdict set** (writing domain — NOT the canonical
code set; declared in the Witness brief): `{ READY-TO-PUBLISH |
NEEDS-ONE-MORE-PASS | RESTART-FROM-OUTLINE }`. See "Verdict domain" section
above for how non-code domains derive their own closed sets.

### Research case study: Literature synthesis

Phase 1: 3 parallel Researchers, each tackling a sub-question with
non-overlapping source scope (assigned by orchestrator at Phase 2 as the
"contract"). Phase 3 skipped (research-only task per the Phase 0
decision tree). Phase 5: 1 Adversarial-Critic checks for source-coverage
gaps + double-counting + citation accuracy. Phase 7: Witness adjudicates
synthesis quality against a **domain-specific closed verdict set**
(literature-synthesis domain — NOT the canonical code set; declared in the
Witness brief): `{ COVERAGE-OK | GAPS-FOUND | SOURCES-WEAK }`. The
domain-specific set follows the same SHAPE constraints (closed, restricted,
non-subjective) as the canonical code set; only the tokens differ to match
the artifact being shipped.

## Failure modes this pattern STILL doesn't catch (honest)

Documented so future orchestrators know to add User-Simulator as a real
e2e step for any user-facing artifact:

1. **API edge cases** — e.g., LLM provider token-budget truncation,
   provider-specific error formats, rate-limit retry semantics. Only
   the User-Simulator role (running against the deployed artifact, NOT
   the diff) reliably finds these. Adversarial-Critics read code; they
   don't simulate real API behavior.

2. **Ceremonial naming residue** — e.g., `Pr1_*` / `V2_*` / `Tmp_*`
   that survives revision passes because Critics focused on architecture
   not naming hygiene. Mitigation: include "no temporal namespacing" as
   an explicit Hard Constraint in every Implementer brief.

3. **Late-stage configuration drift** — e.g., a workspace-path mismatch
   between CLI mode and web mode that only manifests when the deployed
   web flow calls the CLI. Caught by Codex bot in PR review AFTER PR
   opened in our session. Mitigation: Phase 4 must include a path
   round-trip test for any code that constructs paths from environment.

## When to use this skill

- Task has >1 sub-deliverable AND >1 domain (e.g., backend + frontend)
- Task is irreversible (commit/publish/deploy) AND nontrivial
- Task touches restricted surfaces requiring dual review
- Cross-cutting constraints (constitution / monetary / security)
- Multiple contributors must converge without merge conflicts

## When NOT to use this skill

- Single typo fix or trivial rename
- Single-file additive change <100 LoC
- Pure read-only research with no follow-up implementation
- One-off exploration that doesn't ship

## Related skills

- `./plan-grill.md` — fact-based orchestrator↔human disambiguation
  (auto-invoked at Phase 0 + Phase 6)
- `./interface-contract-lock.md` — contract-locking pattern (auto-invoked
  at Phase 2)
- `./auditors.md` — alive menu of audit archetypes (auto-consulted at
  Phase 5)
- `./references/<platform>.md` — platform-specific tier mappings + dispatch
  syntax
- `skills/SUBAGENT_HARNESS.md` — the prior K-HARDEN harness skill; this
  skill subsumes + generalizes that pattern across platforms
- `skills/KARPATHY_ARCHITECT.md` + `skills/KARPATHY_SIMPLE_CODE.md` —
  the architectural lens behind several anti-patterns documented above
