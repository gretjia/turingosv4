# Tri-Model Orchestration Protocol — TuringOS v4

> **Mandate**: User 2026-04-26 — "本项目由你负责组织 codex 和 gemini 共同完成，非常细致的原子化执行"
>
> **Status**: ArchitectAI v1 protocol; supersedes the "external auditors only" framing in earlier CLAUDE.md. Codex + Gemini are now **co-executors**, not just gate-keepers.
>
> **Scope**: applies to all CO P0 + P1 + P2 atoms in `CO_MEGA_PLAN_v3.1_2026-04-26.md`.

---

## § 1 Three Roles

### Claude Opus 4.7 (1M context) — **Orchestrator**
- **Atom spec drafting** — every atom starts as a Claude-written spec (file path + WP § + test file + acceptance criteria + STEP_B flag)
- **Assignment** — decides per atom: Claude implements? Codex implements? Both via parallel branches?
- **Integration** — receives PRs from co-executors, runs `cargo check` + conformance tests, merges
- **TRACE_MATRIX_v3 maintenance** — updates bidirectional matrix on every atom commit
- **Per-phase orchestration** — sprint launch, atom dependency tracking, gate audits
- **User communication** — single channel; Codex/Gemini do not speak to user directly

### Codex (`codex:rescue` subagent) — **Co-Implementer (heavy)**
- **STEP_B atom implementation** — parallel-branch A/B via `EnterWorktree`; preferred for any atom touching `src/{bus,kernel,wal,ledger}.rs` or new restricted files (state/q_state.rs, transition/*, bottom_white/tape/*, bottom_white/ledger/*, economy/escrow_vault.rs, economy/settlement_engine.rs)
- **Deep investigation** — root-cause diagnostics when Claude is stuck; "second-implementation" verification on architecturally critical atoms (CO1.1.4 bus.rs split, CO1.1.5 kernel.rs split, CO1.3.1 gix spike, CO2.4.* AttributionEngine determinism)
- **Independent code review** — for atoms Claude implemented, Codex reviews with `codex:rescue` lens (looks for missed edge cases, not stylistic)
- **Receives**: Claude's atom spec + relevant file context + WP § excerpt
- **Returns**: PR-style diff or critique; never speaks to user

### Gemini DeepThink (via web/API) — **Strategic Architect Reviewer**
- **Cross-§ coherence audit** — does the atom respect the ENTIRE white paper, not just the cited §? Catches "implements WP § 4 but breaks WP § 9.4" class issues
- **Bidirectional trace matrix audit** — at CO P1.13 + every phase exit, validates every WP § has ≥1 code symbol AND every pub symbol has a backlink
- **Conformance test design review** — for the 47+ conformance tests, Gemini asks "does this test actually catch the violation it claims to catch?"
- **Constitutional alignment** — every constitutional Article must have a Gemini sign-off at phase exit
- **Receives**: full white paper + constitution + atom spec(s) + Claude's draft / Codex's PR
- **Returns**: PASS / CHALLENGE / VETO + reasoning; never implements

---

## § 2 Atom Lifecycle (Standard, Non-STEP_B)

```
[1] Claude drafts ATOM SPEC
    ├─ file path + WP § ref + Const Art ref + test file + acceptance criteria
    ├─ saved to handover/atoms/CO{P}.{N}.{M}_<slug>.md
    └─ assignment: who implements + who tests
[2] Gemini reviews ATOM SPEC (PASS/CHALLENGE)
    └─ if CHALLENGE: Claude revises, re-submits
[3] Implementer writes code (Claude OR Codex)
    └─ commit on feature branch atoms/{atom-id}
[4] Non-implementer writes conformance test
    └─ Generator ≠ Evaluator (CLAUDE.md Audit Standard)
[5] Codex code-review (independent of Step 3 implementer)
    └─ PASS/CHALLENGE/VETO
[6] cargo check + cargo test + trace_matrix_v3 conformance
[7] TRACE_MATRIX_v3 update commit
[8] Merge to main when [5]=PASS AND [6]=green AND [7]=updated
```

## § 3 Atom Lifecycle (STEP_B)

For atoms touching restricted files:

```
[1] Claude drafts ATOM SPEC + STEP_B parallel-branch plan
[2] Gemini reviews ATOM SPEC for cross-§ impact
[3] PARALLEL IMPLEMENTATION:
    ├─ Branch A: Claude implements via Edit
    └─ Branch B: Codex implements via worktree (EnterWorktree)
[4] Cross-comparison:
    ├─ Diff Branch A vs Branch B
    └─ If structurally equivalent: pick A (default), merge
       If divergent: Gemini casts tie-breaker review
[5] Conformance test by 3rd party (whichever model didn't implement winner)
[6] Codex final review
[7] cargo check + cargo test + trace_matrix
[8] STEP_B audit ledger entry: cost, models, decision
[9] Merge to main
```

STEP_B atoms inventory (~22 atoms) listed in `CO_MEGA_PLAN_v3.1_2026-04-26.md` § 7.

## § 4 Decision Protocol

| Codex verdict | Gemini verdict | Outcome |
|---|---|---|
| PASS | PASS | merge |
| PASS | CHALLENGE | revise per Gemini → re-review |
| PASS | VETO | block; Claude triages with both |
| CHALLENGE | PASS | revise per Codex → re-review |
| CHALLENGE | CHALLENGE | revise → re-review (must converge) |
| CHALLENGE | VETO | block; surface to user |
| VETO | PASS | block; Claude triages; if Codex VETO valid → user surface |
| VETO | CHALLENGE | block; surface to user |
| VETO | VETO | hard block; user veto-override required (sudo) |

**Rule**: VETO > CHALLENGE > PASS. Conservative wins. Never merge with active VETO unless user explicitly sudos.

## § 5 Cadence + Cost

### Atom-level (~107-142 atoms across CO P0+P1+P2)
- **Standard atoms** (~110): Codex review-only @ ~$2-5 each = $220-550
- **STEP_B atoms** (~22): Codex implements + reviews @ ~$5-10 each = $110-220
- **Gemini per-atom review** (heavy atoms only ~30): ~$1-2 each = $30-60

### Phase-exit dual audit (CO P0.7, P1.14, P2.12)
- Codex full audit: ~$15-25 each × 3 = $45-75
- Gemini full audit: ~$10-15 each × 3 = $30-45

### TOTAL v4 BUDGET
- Atom-level: $360-830
- Phase-exit: $75-120
- **Grand total: $435-950** — exceeds $250-500 estimate in CO_MEGA_PLAN v3.1 § 6 because v3.1 assumed Codex/Gemini as auditors only, not co-executors

**Action**: amend Plan v3.1 § 6 audit budget to $450-950 (this protocol's tri-model co-execution cost).

## § 6 Audit Ledger

`handover/audits/AUDIT_LEDGER.md` (NEW; created at CO P0.6) tracks every Codex/Gemini invocation:

```
| timestamp | atom | model | role | cost ($) | verdict | notes |
```

Closed at every phase exit; running total reported in user weekly check-in.

## § 7 What Each Model Sees Per Invocation

To avoid context bloat:

### Codex receives (per atom invocation)
- Atom spec (1 page)
- Relevant src/ file content (Read by Codex itself)
- Cited WP § excerpt (extract by Claude before invocation, ~200 lines max)
- For STEP_B: Branch A diff if reviewing
- Constitution Art ref text (~50 lines max)

### Gemini receives (per atom or phase invocation)
- Full white paper (1 file)
- Full constitution (1 file)
- Atom spec OR phase exit packet
- Bidirectional trace matrix current state (for phase exit)
- Diff to be reviewed

Both receive the SAME spec; Claude does NOT pre-bias either.

## § 8 Disagreement Escalation to User

Surface to user (D1-D6 style decision request) when:
- Codex VETO + Gemini PASS (or vice versa) — Claude cannot resolve
- Codex CHALLENGE + Gemini VETO — substantive design issue
- Both VETO — hard block; user must adjust scope or sudo
- Atom blocked > 3 days waiting on convergence

User sees: atom ID, both verdicts in full, Claude's interpretation, recommended path.

## § 9 Generator-Evaluator Separation

Per CLAUDE.md Audit Standard + C-010 + C-023 + C-035:

| Atom output | Generator | Evaluator |
|---|---|---|
| Code | Claude OR Codex | Codex (review), Gemini (cross-§) |
| Conformance test | THE OTHER MODEL from code | All three at gate |
| Atom spec | Claude | Gemini (strategy), Codex (feasibility) |
| Phase exit | All three | User (final sudo) |

Hard rule: **the model that wrote the code MUST NOT write the test that gates that code**.

**Hard rule 2 (added per Gemini CO P0.7 audit run 1, Q6 CHALLENGE — closes Codex self-review loophole)**: the model that **implemented** an atom MUST NOT be the **primary code reviewer** for that atom either. Specifically:

| Implementer | Mandatory primary reviewer |
|---|---|
| Claude (orchestrator) | Codex |
| Codex (heavy implementer) | a sandboxed Claude instance via `auditor` subagent (read-only, fresh context, no prior conversation memory) |
| Both via STEP_B parallel branches | the LOSING-branch model casts primary review on the WINNING-branch code; Gemini casts tie-breaker if branches converge |

This rule is **non-negotiable** for process integrity. The intent of "redundancy as trust restoration" requires that the same model never simultaneously generate AND validate the same artifact.

**Atom lifecycle update reflecting Hard rule 2**:

```
[5] PRIMARY CODE REVIEW (mandatory non-implementer):
    - if Step 3 implementer = Claude → Codex reviews
    - if Step 3 implementer = Codex → fresh Claude `auditor` subagent reviews
[5b] STRATEGIC REVIEW (Gemini): cross-§ coherence, reuses Step 2 lens
[5c] CONFORMANCE TEST AUTHOR (Step 4): also non-implementer, may be a different model from Step 5 reviewer (so up to 3 different models touch the atom: implementer, primary reviewer, test author)
```

Cost impact: Hard rule 2 may add 1 extra Claude `auditor` invocation per Codex-implemented atom (~$1-3 each × ~22 STEP_B atoms = ~$22-66 added to budget). Budget cap remains $950 (within tolerance).

## § 10 Risk Adjustments to Plan v3.1

Three risks added to v3.1 § 9 by virtue of tri-model:

- **Risk #12**: Codex/Gemini API outage stalls atom — mitigation: 24h grace period, then Claude solo-implements with retroactive review when service restored
- **Risk #13**: Cost overrun beyond $950 — mitigation: weekly burn-rate check, escalate to user at 80% threshold
- **Risk #14**: Tri-model "groupthink" (all three miss the same architectural rot) — mitigation: every CO Phase exit invites a 4th audit (auditor subagent fresh context, no prior conversation)

## § 11 v4 → v4.1 Hand-Off

At v4 ship (post-CO P2 exit), this protocol gets re-evaluated:
- Was tri-model worth the cost? (compare audit-found-bug count vs cost)
- Should v4.1 (CO P3 MetaTape) keep tri-model or shift to ArchitectAI/JudgeAI runtime auto-audit?

Decision deferred; data collected in AUDIT_LEDGER.md.

— ArchitectAI, 2026-04-26
