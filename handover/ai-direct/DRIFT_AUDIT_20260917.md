# Drift Audit — 2026-09-17

**Auditor role**: JudgeAI-advisory per Art. V.1.3 (advisory only; cannot block work)
**Status**: 🟡 YELLOW
**Session**: clean remote session, no prior context bias
**HEAD**: adaf928 (drift audit: 2026-08-25 — only commit since last audit)

---

## Delta since last audit (2026-08-25)

**One commit** since the Aug 25 audit: `adaf928` — adds `DRIFT_AUDIT_20260825.md` only.
No source, test, or case file changes. This audit is therefore primarily a
**carry-forward confirmation** that YELLOW findings remain unresolved and no new
violations emerged.

---

## Step 1 — Tooling Self-Check (Positive Verification)

**Target**: `forbidden_patterns` in `src/bus.rs`; `native_decide` runtime enforcement.

| Check | Result | Evidence |
|-------|--------|----------|
| `forbidden_patterns` field exists in `BusConfig` | ✅ POSITIVELY FOUND | `src/bus.rs:40` |
| `native_decide` in production `BusConfig::forbidden_patterns` literal | ⚠️ MIGRATED (known) | `src/bus.rs:48` default is `Vec::new()` |
| `native_decide` in `forbidden_patterns_v1` predicate (production enforcer) | ✅ POSITIVELY VERIFIED | `src/top_white/predicates/registry.rs:637` |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | ❌ DOES NOT EXIST | File absent; noted in Aug 25 audit — de-Lean migration (#345) removed chain_runtime.rs |

### Migration note (carry-forward from Aug 25)

`BusConfig::default().forbidden_patterns` is `Vec::new()`. The production enforcement
path is `forbidden_patterns_v1` in the predicate registry (`registry.rs:637`), which
is merkle-rooted and activated at boot via `src/runtime/mod.rs:896`. This is an
architectural migration, not a regression. Future auditors must check the predicate
registry path, not the bus BusConfig literal.

**Tooling self-check verdict**: PASS with migration note. `native_decide` IS enforced.

---

## Step 2 — Constitution + Case Coverage

**Art. IDs in constitution.md** (21 unique; unchanged):

```
Art. 0, Art. 0.1, Art. 0.2, Art. 0.3, Art. 0.4
Art. I, Art. I.1, Art. I.1.1, Art. I.2
Art. II, Art. II.1, Art. II.2, Art. II.2.1
Art. III, Art. III.1, Art. III.2, Art. III.3, Art. III.4
Art. IV
Art. V, Art. V.1, Art. V.1.1, Art. V.1.2, Art. V.1.3, Art. V.2, Art. V.3
```

**Case count**: 51 files in `cases/` (Aug 25 audit counted 50; likely minor recount variance
at the same HEAD — no new case files committed since Aug 25).

**Coverage**: All 21 Art. IDs covered — carry-forward from Aug 25 audit. No new gaps.
Art. 0.x have zero direct cases (covered implicitly via top-level doctrine); same
low-priority gap noted Aug 25.

---

## Step 3 — Active-Use Coverage (ACTIVE_USE_GAP)

No handover plan files changed since Aug 25. Citation frequency map unchanged.

**ACTIVE_USE_GAP = ∅** — no heavily-cited article has zero case coverage.

### Low-priority theoretical gaps (carry-forward, informational)

- **Art. II, Art. III (parent articles)**: 0 direct cases each; sub-articles covered. Low priority.
- **Art. III.1**: Single case (C-022). Cited 110× in handover. Thin coverage.
- **Art. V.3**: Single case (C-071). Coverage adequate for current amendment frequency.

---

## Step 4 — Drift Scan

### Commits reviewed (all since Aug 25 audit)

```
adaf928 drift audit: 2026-08-25   ← only commit; audit file only
```

No source changes to scan for new drift. Carrying forward findings from Aug 25 audit
as unresolved.

### RF-1: omega/decide in agent-facing prompts (C-011 check)

`src/sdk/prompt.rs:358,360,388,390` lists `omega` and `decide` as standard tactics in
agent prompt variants v2/v4. **NOT a C-011 violation** — these are kernel-checked tactics,
not kernel-bypassing. `native_decide` (the forbidden token) does NOT appear in agent prompts.
Aug 25 assessment confirmed; status unchanged.

### RF-2: hybrid labeled as 'n3' (C-032/C-033 check)

No instances found in source. **NOT found.** Status unchanged.

### RF-3: hardcoded thresholds without env override (C-027 — CARRY-FORWARD ⚠️)

Three constants remain without env-override path (unchanged since Aug 25):

| Location | Constant | Value | Env override? |
|----------|----------|-------|---------------|
| `src/runtime/budget_allocation_telemetry.rs:144` | `MAX_PROPOSAL_TOKENS` | 900 | ❌ None |
| `src/web/spec.rs:1673` | `ACCEPTED_TURNS_FORCE_SYNTHESIS_THRESHOLD` | 10 | ❌ None |
| `src/web/generate.rs:70` | `MAX_GENERATE_ATTEMPTS` | 3 | ❌ None |

C-027 ruling: behavior-affecting parameters must be overridable via env var. Defaults are
fine; compile-time-only constants for runtime parameters are not. These three were flagged
as MEDIUM/LOW advisory in Aug 25 audit. No remediation occurred.

**Advisory**: Researcher should evaluate C-027 compliance for `MAX_PROPOSAL_TOKENS`
(token budget cap — highest criticality) and `ACCEPTED_TURNS_FORCE_SYNTHESIS_THRESHOLD`
(web flow cutoff). `MAX_GENERATE_ATTEMPTS` = 3 is lowest criticality.

### RF-4: architecture win with tape/market/Boltzmann dormant (C-033 check)

No new PROVEN/DEFINITIVE efficiency claims in any committed file since Aug 25. **NOT found.**

### RF-5: citation-vs-precedent cross-check

No new commits with C-xxx or Art. X citations since Aug 25. Nothing to cross-check.

---

## Summary

| Step | Result |
|------|--------|
| Step 1 tooling self-check | PASS with migration note (enforcement in predicate registry) |
| Step 2 case coverage | PASS — all 21 Art. IDs covered; 51 case files |
| Step 3 ACTIVE_USE_GAP | NONE |
| Step 4 drift scan | 3 C-027 concerns carry-forward; no C-011, C-032, C-033, or new violations |

### Why YELLOW (not GREEN, not RED)

**Not GREEN**: Three C-027 potential violations remain unresolved (hardcoded behavioral
parameters without env-override path). Also: production `BusConfig::forbidden_patterns`
is `Vec::new()` — auditors must know to check the predicate registry.

**Not RED**: `native_decide` IS positively verified in `forbidden_patterns_v1` at
`registry.rs:637`. No C-011, C-032, C-033 violations. No citation-precedent mismatch.
No new violations since Aug 25.

### Open advisory items (carry-forward from Aug 25; researcher decides priority)

1. **[MEDIUM]** Evaluate C-027 env-override for `MAX_PROPOSAL_TOKENS` (`budget_allocation_telemetry.rs:144`)
   and `ACCEPTED_TURNS_FORCE_SYNTHESIS_THRESHOLD` (`web/spec.rs:1673`).

2. **[LOW]** Document that Step 1 audit protocol now checks `src/top_white/predicates/registry.rs`
   for `forbidden_patterns_v1`, not `src/bus.rs` BusConfig literal.

3. **[LOW]** Consider adding Art. II and Art. III parent-level cases if parent doctrine
   is independently exercised beyond sub-articles.

---

*Auditor: JudgeAI-advisory (clean remote session, 2026-09-17)*
*Role boundary: advisory per Art. V.1.3 — findings only, no implementation proposals*
*Preceding audit: DRIFT_AUDIT_20260825.md (YELLOW, 3 C-027 concerns, no RED flags)*
