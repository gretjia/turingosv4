---
name: orchestrate-claude
description: Claude dynamic-Workflow edition of /orchestrate. Same orchestrate contract (6 roles, locked interface contracts, "Critic enumerates / Witness adjudicates" dual gate, dynamic auditor menu, closed verdict sets, 9-section briefs, checkability test) realized through Claude Code's deterministic **Workflow tool** instead of model-driven Agent dispatch. User invokes /orchestrate-claude; the main loop runs Phase 0 triage + plan-grill + contract-lock interactively, then launches ONE Workflow that runs Phase 1–7 (research → implement → verify → adversarial critics → shipping witness) deterministically, then the main loop ships (Phase 8–10). Use when the fan-out + dual review can run deterministically once contracts + auditors are locked.
allowed-tools: ["*"]
mechanism: claude-workflow-tool          # vs orchestrate's model-driven Agent dispatch
roles: [Researcher, Contract-Architect, Implementer, Adversarial-Critic, Shipping-Witness, User-Simulator]
tiers: [BRIEF, STANDARD, DELIBERATIVE]
reuses:                                  # Layer-2 ADD only — do NOT restate these; point here
  - ../orchestrate/SKILL.md
  - ../orchestrate/plan-grill.md
  - ../orchestrate/interface-contract-lock.md
  - ../orchestrate/auditors.md
  - ../orchestrate/references/claude.md
workflow-template: ./workflow-template.js
---

# Orchestrate-Claude — orchestrate, realized through Claude's dynamic Workflow

This is the **same orchestrate pattern**, run on a different **mechanism**.

- `/orchestrate` dispatches sub-agents with the **`Agent` tool** — *model-driven*:
  the main loop decides each dispatch turn-by-turn, reads each result, decides
  the next. Maximum human-in-the-loop; maximum flexibility; the orchestrator
  babysits every phase.
- `/orchestrate-claude` encodes Phase 1–7 as a **`Workflow` script** —
  *deterministic*: a JS harness (`agent()` / `parallel()` / `pipeline()` /
  `phase()` / `schema`) runs the fan-out + dual review in the background and
  returns a structured result. The control flow is in code, not in the model.

**The contract is identical. Only the engine changes.** Per `AGENTS.md §2`
two-layer model, this skill is a **Layer-2 ADD**: it reuses orchestrate's roles,
tiers, 9-section briefs, contract-lock, auditor menu, and verdict domains
verbatim (see `reuses:` above) and adds *only* the Workflow-mechanism layer. It
never restates-and-narrows a rule from `../orchestrate/`.

## Read this first (do not skip)

Before using this skill, the orchestrator MUST have internalized the sibling
`../orchestrate/SKILL.md` — the 8 core principles, the 6 roles, the 3 tiers, the
9-section brief, the "Critic enumerates / Witness adjudicates" dual gate, and the
"checkability test" for acceptance criteria. This file does **not** repeat them.
It maps them onto the Workflow tool and documents the seam + the Workflow-native
upgrades.

## Which one to reach for

| | `/orchestrate` (Agent tool) | `/orchestrate-claude` (Workflow tool) |
|---|---|---|
| Control flow | Model-driven (main loop decides each step) | Deterministic JS (loops/pipeline/parallel in code) |
| Human ratification | Possible at **every** phase | Only at the **seams** (Phase 0 pre, Phase 8 post) |
| Phase 5 critics | Parallel Agent dispatches, prose verdict | `pipeline()` + **per-finding adversarial verify** |
| Witness verdict | Prose "return ONE token" (discipline) | **JSON-schema enum** (mechanically enforced) |
| Clean-context audit (§9) | By discipline (don't pass transcript) | **By construction** (each `agent()` is fresh-context) |
| Best when | You must ratify mid-flight; scope is fluid | Contracts + auditors lock cleanly; fan-out is heavy |
| Resumable | No (conversation-bound) | Yes (`resumeFromRunId`) |

Reach for `/orchestrate-claude` when, **after Phase 0 + contract-lock, the rest
can run without you** — heavy parallel research, a cohesive implement, a 2–5
auditor adversarial pass, and a single witness gate. Reach for `/orchestrate`
when you need to stop and ratify at unpredictable points, or scope is still
moving.

## The hybrid seam (this skill's core architecture)

A Workflow runs in the **background**. It can `log()` progress but **cannot**
`AskUserQuestion` — there is no interactive human-input primitive mid-run.
orchestrate's human touchpoints (plan-grill at Phase 0/6, ship ratification at
Phase 8, merge at Phase 10) and `AGENTS.md §14a` ("coding agents may only create
PRs; sub-agents NEVER commit/push/merge") therefore **force** a three-band split:

```
┌─ MAIN LOOP (interactive, before launch) ───────────────────────────────────┐
│ Phase 0  Triage (decision tree). If unclassifiable → plan-grill            │
│          (AskUserQuestion, per ../orchestrate/plan-grill.md).              │
│ Phase 2  Contract-Architect: author + LOCK interface contracts C1..Cn      │
│          in code-form (per ../orchestrate/interface-contract-lock.md).     │
│          Pick the 2–5 auditors (per ../orchestrate/auditors.md).           │
│          Pick the Witness closed verdict set + the pass-token.             │
│          Build `args` (the task spec) and emit the Workflow script.        │
└─────────────────────────────────────────────────────────────────────────────┘
                                   │  Workflow({ script, args })
                                   ▼
┌─ WORKFLOW (deterministic, background) ─ ./workflow-template.js ─────────────┐
│ Phase 1  Research      parallel Explore agents (optional; impl-time only)   │
│ Phase 3  Implement     single in-place Implementer (worktree = opt-in)      │
│ Phase 4  Verify        REAL commands; bounded self-repair loop (≤K)         │
│ Phase 5  Critique      pipeline(auditors → enumerate → per-finding REFUTE)  │
│ Phase 6  Triage        must-fix | nice | wontfix; bounded auto-remediation; │
│                        user-only forks → return `needsHumanDecision`        │
│ Phase 7  Witness       1 agent, fresh context, **schema-enforced** verdict  │
│          returns { witness, confirmedFindings, verify, impl, shipReady }    │
└─────────────────────────────────────────────────────────────────────────────┘
                                   │  structured result
                                   ▼
┌─ MAIN LOOP (interactive, after return) ────────────────────────────────────┐
│ Phase 6′ If `needsHumanDecision` → plan-grill, then RESUME the workflow     │
│          (`resumeFromRunId`) with the decision folded into `args`.          │
│ Phase 8  If shipReady → orchestrator commits + opens PR (NEVER a sub-agent).│
│ Phase 9  Closing audit (a second Workflow in closing-mode, or reuse).       │
│ Phase 10 Orchestrator merges (the §14a legitimate bypass; never a sub-agent).│
│ Phase 11 User-Simulator vs the DEPLOYED artifact (real API) — main loop.    │
└─────────────────────────────────────────────────────────────────────────────┘
```

Why this seam and not "one autonomous run":

1. **plan-grill's soul is human ratification.** It lives where the human is —
   the main loop — not inside a headless script. Decisions are resolved *before*
   launch and folded into `args`; mid-run forks the script cannot resolve come
   back as `needsHumanDecision` and the main loop resumes the run.
2. **Contracts want judgment + a human's eyes.** Contract-lock is "the single
   highest-leverage mechanism" — it stays in the main loop, authored before the
   parallel Implementers ever spawn.
3. **`§14a` is not a free choice.** Workflow agents are sub-agents; sub-agents
   never commit/push/merge. The script *must* end at the Witness verdict + diff.
   The orchestrator (main loop) ships.

## Phase → Workflow primitive mapping

| Phase | orchestrate role | Workflow realization |
|---|---|---|
| 1 Research | Researcher (read-only) | `parallel()` of `agent(..., {agentType:'Explore', model:'haiku'})`, each a 9-section research brief |
| 3 Implement | Implementer | one `agent(..., {model: tierModel(tier)})` in-place; `schema: IMPLEMENT` (carries `stuck` = Section-8 abort) |
| 4 Verify | (gate) | `agent(verifyBrief, {schema: VERIFY})` runs REAL commands; `while` self-repair loop ≤K |
| 5 Critique | Adversarial-Critic (open) | `pipeline(auditors, enumerate→`FINDINGS`, perFinding→`parallel(refute)`→`VERDICT`)`; keep `isReal` |
| 6 Triage | (orchestrator) | `agent(triageBrief, {schema: TRIAGE})`; bounded remediation; `needsHumanDecision` escape |
| 7 Witness | Shipping-Witness (closed) | one `agent(witnessBrief, {schema: witnessSchema(verdictSet)})` — enum-enforced single token |

The 9-section brief (`../orchestrate/SKILL.md` "Standard sub-agent prompt
template") is **mandatory** for every `agent()` prompt in the script. The
template's brief-builder functions assemble all 9 sections from `args`.

## Workflow-native upgrades (faithful enhancements, honestly labeled)

The latitude to "optimize the flow" is spent on upgrades that *strengthen*
orchestrate's existing doctrine using mechanism the Agent-tool edition lacks.
Each is traceable to an existing orchestrate principle:

1. **Schema-enforced Witness verdict** → kills Anti-pattern D ("open verdict
   domain on Witness"). The closed set becomes a JSON-Schema `enum`; the Witness
   *cannot* emit prose or a 47-finding list. `../orchestrate/SKILL.md`'s "Closed
   / Restricted / Non-subjective" shape is now mechanically guaranteed, not
   discipline-hoped.
2. **Per-finding adversarial verify** → realizes "Critic enumerates" and then
   adds a refutation stage the Agent edition never had. Each enumerated finding
   gets an independent skeptic prompted to **refute** it; only `isReal` findings
   reach Triage. Plausible-but-wrong findings die before they block the ship.
3. **Clean-context audit by construction** → `AGENTS.md §9` demands "a fresh
   agent that does NOT have the implementation transcript." Every `agent()` call
   is fresh-context, so the Witness and the Critics structurally cannot see the
   Implementer's transcript. The Witness sees ONLY what its brief carries
   (`§9`'s allowed list). The doctrine is satisfied by the engine, not by care.
4. **Bounded self-repair loop** → realizes `AGENTS.md §4` ("real run → debug →
   fix → rerun") as deterministic control flow: verify fails → re-dispatch the
   Implementer with the failure as input, ≤K times, then BLOCK. "Real test beats
   review" — the gate is a real exit code, never a vibe.
5. **Budget-scaled breadth** → the `budget` global scales auditor count /
   loop-until-dry depth to a "+Nk" directive, honoring `Cost-Budget` auditor
   discipline without over-instrumenting by default.

None of these change the contract. They make the existing gates falsifiable in
code rather than in prose.

## Orchestrator pre-launch checklist (before emitting the Workflow script)

Extends `../orchestrate/SKILL.md`'s 4-item pre-dispatch self-check with the
seam-specific items. ALL must hold before calling `Workflow`:

1. [ ] Phase 0 triage done; if unclassifiable, plan-grill **already** resolved
       it (the script gets answers, not questions).
2. [ ] Interface contracts C1..Cn **locked in code-form** and placed in `args`
       (or task is small enough to skip Phase 2).
3. [ ] Every acceptance criterion is an **exact command + expected output**
       (the checkability test) — they go into `args.verify` and the Witness brief.
4. [ ] The 2–5 auditors are chosen (menu or invented) with attack-vectors +
       anti-overlap clauses, in `args.auditors`.
5. [ ] The Witness **closed verdict set** + the single **pass-token** are
       declared in `args.witness` (3–5 tokens, no mushy middle, one = passes gate).
6. [ ] Required reading per agent ≤ 10 paths; each agent brief has the Section-8
       abort protocol.
7. [ ] **worktree guard**: implementers run **in-place** (single, cohesive
       deliverable). Parallel-file implementers needing `isolation:'worktree'`
       are opt-in AND the orchestrator has confirmed the worktree will branch
       from the CURRENT branch, not `origin/HEAD` (known drift; prefer in-place).
8. [ ] The script contains **no** `git commit/push`, `gh pr create`, or merge —
       `§14a` ends the script at the Witness verdict.

## Hard rules (inherited + seam-specific)

- **§14a boundary is absolute.** The Workflow script never commits, pushes,
  opens, or merges a PR. It returns a verdict + a diff summary. The orchestrator
  ships in the main loop (Phase 8 / 10).
- **No `isolation:'worktree'` by default.** Memory: worktree can branch from
  `origin/HEAD` not the current branch. Default to a single in-place Implementer
  (orchestrate already says "never >3", "1–2 parallel"). Worktree is the
  documented exception, not the default.
- **Witness sees only the §9 allowed list.** Brief, risk class, touched FC
  nodes, current diff, evidence paths, acceptance criteria + actual command
  outputs. Never the implementation transcript (free, by fresh-context).
- **Real commands only at Phase 4.** The verify gate runs the actual checks
  (`cargo check` / `cargo test --workspace --no-fail-fast` /
  `bash scripts/run_constitution_gates.sh` per `AGENTS.md §7`) and gates on real
  exit codes. A test that cannot fail is documentation, not a gate.
- **`/no-proven-checklist` still applies.** If the run produces a
  `PROVEN`/`DEFINITIVE`/`X > Y` headline, `AGENTS.md §17` G1–G6 gate it — the
  Witness verdict is not a substitute.

## Runtime procedure (what the orchestrator does when /orchestrate-claude fires)

1. **Triage (Phase 0).** Run the `../orchestrate/SKILL.md` Phase-0 decision
   tree. Single small artifact → consider plain `/orchestrate` or a 1-implementer
   script. Research-only → the research-only script branch. Multi-artifact /
   cross-cutting / irreversible → full template. Unclassifiable → plan-grill now.
2. **Lock contracts + pick auditors + declare verdict set (Phase 2).** Author
   C1..Cn (code-form). Choose 2–5 auditors from `../orchestrate/auditors.md`
   (or invent per its 4-step protocol). Declare the Witness closed set + pass-token.
3. **Build `args`** matching `./workflow-template.js`'s documented shape.
4. **Adapt the template** minimally (most task-specifics live in `args`; edit the
   script body only for structural changes like research-only or parallel impl).
   Read `./workflow-template.js`, copy it, adjust, and pass it **inline** as
   `Workflow({ script, args })`. Persisted scriptPath is returned for iteration.
5. **On completion** (auto-notified): read the structured result.
   - `needsHumanDecision` non-empty → plan-grill, then `Workflow({ scriptPath,
     resumeFromRunId, args: <augmented> })`.
   - `blocked` → surface the blocker; re-enter Phase 6 in the main loop.
   - `shipReady` true → Phase 8: orchestrator commits + opens PR with a body that
     cites contracts, acceptance outputs, confirmed findings, and the verdict.
6. **Phase 9 closing audit** — a second Workflow in closing-mode (auditors return
   the `{ READY-TO-MERGE | EXTERNAL-FEEDBACK-REQUIRES-REVISION | IMPL-DRIFT-FROM-PLAN
   | DEFERRAL-MISSING }` closed set as a schema enum), gating Phase 10 merge.
7. **Phase 10 merge** — orchestrator only (`§14a` legitimate bypass).
8. **Phase 11 User-Simulator** — main loop, against the DEPLOYED artifact with a
   real API. The ONLY role that catches real-API edge cases; never inside the
   pre-ship workflow (it needs the shipped artifact, not the diff).

## When NOT to use this skill (honest, per orchestrate's failure-mode ethos)

- **You need to ratify at every phase.** If the task's scope keeps moving and you
  want a human checkpoint after each agent, use `/orchestrate` (Agent tool) —
  deterministic control flow is the wrong tool for fluid scope.
- **Contracts cannot be locked up front.** If the interface only becomes clear
  *during* implementation, the locked-contract precondition fails; explore with
  `/orchestrate` first.
- **The artifact is a single trivial change.** Skip orchestration entirely
  (`../orchestrate/SKILL.md` "When NOT to use").
- **Real-API behavior is the actual risk.** The pre-ship workflow reads code; it
  cannot simulate a real LLM round-trip. That is Phase 11 (User-Simulator), in
  the main loop, post-ship. Documented honestly: the deterministic fan-out does
  NOT catch token-truncation / provider-error / rate-limit classes.

## Failure modes this edition adds (honest)

1. **Frozen `args`.** Once launched, the script runs to its return; a fork it
   cannot resolve must be *anticipated* as a `needsHumanDecision` branch or it
   blocks. Mitigation: spend Phase 0 / contract-lock generously; the cost of a
   bad `args` is a full re-run (mitigated by `resumeFromRunId`).
2. **Schema rigidity vs. reality.** A Witness verdict set that omits the real
   outcome forces a wrong token. Mitigation: the 3–5 tokens must cover the
   outcome space; when unsure, prefer `/orchestrate`'s prose witness.
3. **Worktree drift** (carried from memory). If a future task genuinely needs
   parallel-file implementers, `isolation:'worktree'` may branch from
   `origin/HEAD`. Mitigation: in-place default; worktree only with an explicit
   current-branch confirmation.
4. **Name-match predicates false-ABORT** (first real run, disk-cleanup
   2026-06-04). A Witness/safety predicate that tests **identity** ("is there a
   process whose path contains the project name?") rather than **actual
   coupling** to the target will ABORT on innocent bystanders. The first run's
   `ps | grep 'turingos'` correctly caught a live `target/debug/turingos_web`
   (genuine catch) but then false-ABORTed on an unrelated `src/drivers/
   llm_proxy.py` from a *different* worktree holding zero `target/` handles.
   Mitigation: a pre-irreversible-action predicate must test the real hazard
   relation — **executes-from** (`ps | grep 'target/(debug|release)/'`),
   **holds-handle** (`lsof +D <target>`, count non-header lines), or
   **compiler-writing** — never a bare name match. And verify "is this evidence
   or a build artifact?" by **content** (`file` → Mach-O = regenerable), not by
   a `-iname evidence` directory search (which missed three
   `constitution_gate_report.*` evidence *files* in round 1). Don't override an
   ABORT — fix the *predicate*, then re-audit.

## Related

- `../orchestrate/SKILL.md` — the canonical contract this edition realizes.
- `../orchestrate/plan-grill.md` — fact-based disambiguation (main loop, Phase 0/6).
- `../orchestrate/interface-contract-lock.md` — code-form contract locking (Phase 2).
- `../orchestrate/auditors.md` — the alive auditor menu (Phase 5).
- `../orchestrate/references/claude.md` — tier→model + Agent/AskUserQuestion syntax.
- `./workflow-template.js` — the canonical Phase 1–7 Workflow script to adapt.
