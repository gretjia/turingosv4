# Drift Audit — 2026-09-23

**Auditor role**: JudgeAI-advisory per Art. V.1.3 (advisory only; cannot block work)
**Status**: 🟡 YELLOW
**Session**: clean remote session, no prior context bias
**HEAD**: adaf928 (drift audit: 2026-08-25 — prior audit commit, no code changes since)

---

## Step 1 — Tooling Self-Check (Positive Verification)

**Target**: `forbidden_patterns` enforcement in production code.

### Findings

| Check | Result | Evidence |
|-------|--------|----------|
| `forbidden_patterns` field exists in `BusConfig` (`src/bus.rs:40`) | ✅ FOUND | `src/bus.rs:40` |
| `native_decide` in production `BusConfig::forbidden_patterns` literal | ⚠️ MIGRATED (same as prior audit) | `src/bus.rs:48` is `Vec::new()` |
| `native_decide` in `forbidden_patterns_v1` predicate (production enforcement) | ✅ POSITIVELY VERIFIED | `src/top_white/predicates/registry.rs:637` |
| `unsafe` and `axiom ` also in `forbidden_patterns_v1` | ✅ CONFIRMED | `src/top_white/predicates/registry.rs:638-639` |

**Migration note (carried from 2026-08-25 audit)**: `BusConfig::default()` now has
`forbidden_patterns: Vec::new()`. The production enforcement path is the predicate
registry (`forbidden_patterns_v1` at `registry.rs:631-643`), activated at ChainTape
boot. This is an architectural migration, not a regression. `native_decide` IS enforced.

**Tooling self-check verdict**: PASS — `native_decide` positively verified in production
enforcement surface at `src/top_white/predicates/registry.rs:637`.

---

## Step 2 — Constitution + Case Coverage

### Art. IDs in `constitution.md`

```
Art. I, Art. I.1, Art. I.1.1, Art. I.2
Art. II, Art. II.1, Art. II.2, Art. II.2.1
Art. III, Art. III.1, Art. III.2, Art. III.3, Art. III.4
Art. IV
Art. V, Art. V.1, Art. V.1.1, Art. V.1.2, Art. V.1.3, Art. V.2, Art. V.3
```

Total: **21 unique Art. IDs**. 51 case files in `cases/` (one more than prior audit: C-076 added).

### Coverage map (minimum case counts per Art. ID)

| Art. ID | # case files referencing | Status |
|---------|--------------------------|--------|
| Art. I | 37 | ✅ |
| Art. I.1 | 11 | ✅ |
| Art. I.1.1 | 2 | ✅ |
| Art. I.2 | 5 | ✅ |
| Art. II | 17 (via sub-articles) | ✅ |
| Art. II.1 | 3 | ✅ |
| Art. II.2 | 7 | ✅ |
| Art. II.2.1 | 4 | ✅ |
| Art. III | 7 (via sub-articles) | ✅ |
| Art. III.1 | 1 (thin) | ✅ |
| Art. III.2 | 2 | ✅ |
| Art. III.3 | 2 | ✅ |
| Art. III.4 | 2 | ✅ |
| Art. IV | 10 | ✅ |
| Art. V | 19 | ✅ |
| Art. V.1 | 13 | ✅ |
| Art. V.1.1 | 4 | ✅ |
| Art. V.1.2 | 4 | ✅ |
| Art. V.1.3 | 5 | ✅ |
| Art. V.2 | 4 | ✅ |
| Art. V.3 | 1 (thin) | ✅ |

**All 21 Art. IDs have coverage. No zero-coverage article.**

**Non-constitution citations**: `Art. VI` (bibliography, N/A) and `Art. X`
(appears only in regex pattern examples in audit prompts). Neither is a
constitutional article; neither is being cited as a ruling precedent.

---

## Step 3 — Active-Use Coverage (ACTIVE_USE_GAP Analysis)

### Citation frequency across handover files

Top-cited articles (occurrence count, all handover docs):

```
Art. V      1226    Art. IV      788    Art. II.1    460
Art. V.1.2   382    Art. V.1.3   350    Art. III.3   330
Art. I.1     314    Art. III.4   306    Art. II.2.1  306
Art. I.2     301    Art. III.2   293    Art. V.1.1   290
Art. III     252    Art. I.1.1   207    Art. V.3     199
Art. II.2    175    Art. V.1     126    Art. I       123
Art. V.2     118    Art. III.1   114    Art. II       74
```

### ACTIVE_USE_GAP

Intersect: "heavily cited in recent plans/handover" ∩ "zero case files" =

**ACTIVE_USE_GAP = ∅** — no heavily-cited article has zero coverage.

### Low-priority theoretical coverage gaps (informational, not RED)

These are carried unchanged from the 2026-08-25 audit:

- **Art. II (parent)**: 74 citations, 0 direct case files. Sub-articles covered.
- **Art. III (parent)**: 252 citations, 0 direct case files. Sub-articles covered.
- **Art. III.1**: Single case (C-022). 114 citations. Coverage thin.
- **Art. V.3**: Single case (C-071). 199 citations. Amendment process used once.

---

## Step 4 — Drift Scan

### Code change status since last audit

**No new code commits since the 2026-08-25 drift audit.**

```
adaf928 drift audit: 2026-08-25   ← HEAD (only new commit is the prior audit itself)
b180971 H-HET-2 converge: de-Lean merge + Codex P1 fixes + Gate-D/r022 (#347)
78c3300 scripts: BSD/macOS-portable constitution gate runner (POSIX sed, wc strip) (#346)
b16b9de de-Lean kernel migration (Class 4): genericize Lean-branded kernel types, wire-safe (#345)
231ec17 autoloop: audited autoresearch loop harness (GATE-1 pre-flight + breakers + GATE-2 audit) (#344)
```

The 20 most recent commits are all pre-August-25 code changes (reviewed by prior audit)
or the prior audit document itself. Drift scan focuses on confirming no regression in
those findings.

### Red-flag scan results (full re-verification)

#### RF-1: omega/decide in agent-facing prompts (C-011 check)

`src/bin/lean_market_agent.rs:1292` contains `"you decide for yourself"` — this is
English language "decide" in a prose instruction to the agent, NOT the Lean tactic
`decide`. The sentence reads: "No role has been assigned to you. You are shown ONLY
market signals — you decide for yourself what to do."

Lean tactic `decide` and `omega` appear as legitimate tactic listings in agent prompts
(prior audit: `src/sdk/prompt.rs:358,360`). These are kernel-validated tactics.
`native_decide` (kernel-bypass) is NOT listed in agent prompts.

**Verdict**: NOT a C-011 violation. Enforcement at predicate gate, not prompt exclusion.

#### RF-2: hybrid condition labeled as 'n3' (C-032/C-033 check)

`src/runtime/proposal_telemetry.rs:714` uses `"n3"` as a branch_id label in a test
(`fresh_record("n3", "n3.b0")`). Context: this is a schema v2 roundtrip test; "n3"
is an arbitrary branch name string (alongside examples "swarm_a", "n1"), not a
causal label for a hybrid experimental condition. No production code uses "n3" as
a hybrid condition identifier.

**Verdict**: NOT a C-032/C-033 violation. The n3 usage is test scaffolding only.

#### RF-3: hardcoded thresholds without env override (C-027 check)

**Persistent from 2026-08-25 audit — unresolved:**

| Location | Constant | Value | Env override? |
|----------|----------|-------|---------------|
| `src/runtime/budget_allocation_telemetry.rs:144` | `MAX_PROPOSAL_TOKENS` | 900 | ❌ None |
| `src/web/spec.rs:1673` | `ACCEPTED_TURNS_FORCE_SYNTHESIS_THRESHOLD` | 10 | ❌ None |
| `src/web/generate.rs:70` | `MAX_GENERATE_ATTEMPTS` | 3 | ❌ None |

These were flagged in the prior audit (2026-08-25) and remain unchanged.
No env override paths have been added for any of these three constants.

C-027 ruling: behavior parameters must be overridable via env var; default values OK
but not const/hardcode. These affect runtime behavior (token budget cap, forced-synthesis
threshold, retry count) without a recompile path.

**Verdict**: YELLOW concern — same as prior audit. Advisory only.

#### RF-4: claimed architecture win with tape/market/Boltzmann dormant (C-033 check)

`LATEST.md` (current head of handover state) explicitly describes the P1 result as
a NO-GO for price-routing, confirmed by clean-context audit `NO-VIOLATION`/`PROCEED`.
The directional finding is explicitly labeled non-significant. No PROVEN/DEFINITIVE
efficiency claim in active documents.

**Verdict**: NOT found. No C-033 violation.

#### RF-5: citation-vs-precedent cross-check

Three most recently modified case files (C-070, C-071, C-072) were reviewed:
- C-070 cites Art. I.2, V.1, C-023, C-010 — all correctly applied to the pre-registration
  and multiplicity discipline incident.
- C-071 cites Art. V.1.1, V.1.2, V.1.3, V.3 — all correctly applied to the amendment
  process incident.
- C-072 cites Art. V.1.3 — correctly applied to Veto-AI scope narrowing.

No precedent is cited in a way that contradicts its actual ruling.

**Verdict**: No citation-vs-precedent mismatch found.

---

## Summary

| Step | Result |
|------|--------|
| Step 1 tooling self-check | ✅ PASS — `native_decide` positively verified in `registry.rs:637` |
| Step 2 case coverage | ✅ PASS — all 21 Art. IDs covered |
| Step 3 ACTIVE_USE_GAP | ✅ NONE |
| Step 4 drift scan | 🟡 3 persistent C-027 concerns (unchanged since 2026-08-25); no new violations |

### Why YELLOW (not GREEN, not RED)

**Not GREEN** because:
- Three C-027 concerns from the prior audit remain open with no env override added.
  GREEN requires positive verification that no behavioral parameters are hardcoded
  without override; these are not resolved.
- `BusConfig::default()` still has `forbidden_patterns: Vec::new()` — the migration
  note from the prior audit has not been formalized (step 1 protocol update pending).

**Not RED** because:
- `native_decide` IS positively verified in the production enforcement surface
  (`registry.rs:637`) — the gate is active, not dormant.
- No new code commits since prior audit — no regression introduced.
- No C-011, C-032, C-033, or citation-vs-precedent violations found.
- No PROVEN/DEFINITIVE efficiency claims in active documents.

### Action items for researcher (advisory, not blocking)

These carry forward unchanged from the 2026-08-25 audit:

1. **[MEDIUM]** Evaluate env-override for `MAX_PROPOSAL_TOKENS`
   (`src/runtime/budget_allocation_telemetry.rs:144`) — affects token budget cap
   without recompile.

2. **[MEDIUM]** Evaluate env-override for `ACCEPTED_TURNS_FORCE_SYNTHESIS_THRESHOLD`
   (`src/web/spec.rs:1673`) — affects web UX flow cutoff without recompile.

3. **[LOW]** Evaluate env-override for `MAX_GENERATE_ATTEMPTS`
   (`src/web/generate.rs:70`).

4. **[LOW]** Update drift audit Step 1 protocol doc: note that production
   `forbidden_patterns` now lives in `src/top_white/predicates/registry.rs`, not
   `src/bus.rs` BusConfig literal.

---

*Auditor: JudgeAI-advisory (clean remote session, 2026-09-23)*
*Role boundary: advisory per Art. V.1.3 — findings only, no implementation proposals*
*Prior audit: `handover/ai-direct/DRIFT_AUDIT_20260825.md` (YELLOW — same concerns)*
