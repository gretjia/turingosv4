# Drift Audit — 2026-09-09

**Auditor role**: JudgeAI-advisory per Art. V.1.3 (advisory only; cannot block work)
**Status**: 🟡 YELLOW
**Session**: clean remote session, no prior context bias
**HEAD**: adaf928 (drift audit: 2026-08-25) — no new commits since last audit

---

## Step 1 — Tooling Self-Check (Positive Verification)

**Target**: `forbidden_patterns` struct in `src/bus.rs`; evaluator binary.

| Check | Result | Evidence |
|-------|--------|----------|
| `forbidden_patterns` field in `BusConfig` struct | ✅ FOUND | `src/bus.rs:40` |
| `BusConfig::default()` production literal | ⚠️ `Vec::new()` | `src/bus.rs:48` — empty, not containing `native_decide` |
| `native_decide` in `forbidden_patterns_v1` predicate (production enforcement) | ✅ POSITIVELY VERIFIED | `src/top_white/predicates/registry.rs:637` |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | ❌ FILE NOT FOUND | Removed by de-Lean migration (#345, commit `b16b9de`) — no replacement located |

### Structural drift note (inherited from 2026-08-25 audit, still present)

`BusConfig::default()` uses `Vec::new()`. Production enforcement of `native_decide` has
migrated from the former `chain_runtime.rs` literal into the predicate registry:

- `src/top_white/predicates/registry.rs:631–643` registers `forbidden_patterns_v1` with
  patterns `["native_decide", "unsafe", "axiom "]`
- Boot path: `src/runtime/mod.rs:896` calls `activate_predicate_binding_for_boot()`
- Gate: `src/state/sequencer.rs:1937` calls `verify_work_predicates(...)` on every WorkTx

**Verdict**: `native_decide` enforcement is POSITIVELY VERIFIED via predicate path.
The evaluator.rs absence is a known migration artifact. Step 1 PASSES but carries a
persisting structural drift note. This note is now two audit cycles old; researcher
should evaluate whether a doc update to the audit protocol is warranted.

---

## Step 2 — Constitution + Cases

**Constitution**: `/home/user/turingosv4/constitution.md` — present ✅

**Art. IDs found** (21 total):
`Art. I`, `Art. I.1`, `Art. I.1.1`, `Art. I.2`, `Art. II`, `Art. II.1`, `Art. II.2`,
`Art. II.2.1`, `Art. III`, `Art. III.1`, `Art. III.2`, `Art. III.3`, `Art. III.4`,
`Art. IV`, `Art. V`, `Art. V.1`, `Art. V.1.1`, `Art. V.1.2`, `Art. V.1.3`, `Art. V.2`,
`Art. V.3`

**Coverage summary**:

| Article | Case file count |
|---------|----------------|
| Art. I.1.1 | 4 |
| Art. III.1 | 3 |
| Art. III.2 | 4 |
| Art. III.3 | 4 |
| Art. III.4 | 4 |
| Art. V.3 | 1 (only C-071) |
| All others | ≥5 |

**Verdict**: All 21 Art. IDs have at least one case file. Art. V.3 (Constitutional
Amendment Log) has only C-071 — single case, low but non-zero. No gaps to RED status.

---

## Step 3 — Active-Use Coverage (last 30 days)

**Last 30 days**: 2026-08-09 to 2026-09-09.

Only one handover document was produced in this window: `DRIFT_AUDIT_20260825.md`.
No new plan files, TB charters, or LATEST.md updates were found.

**Active articles in recent plans** (inherited from Aug 25 analysis, no new plans):
- Art. V.1.3 (audit role) — actively cited in drift audits
- Art. I, Art. II.1, Art. V.1 — cited in TB charters (older)

**ACTIVE_USE_GAP**: NONE IDENTIFIED. The absence of new plan activity means there are
no newly-cited articles without case coverage.

**Low-priority theoretical coverage note** (informational, not RED):
- Art. V.3 remains at 1 case (C-071). If amendment-process activity resumes, consider
  adding a second case with a concrete failure scenario.

---

## Step 4 — Drift Scan (since 2026-08-25)

**Git log since last audit**: No new commits. HEAD remains `adaf928` (drift audit: 2026-08-25).

No new code, plans, or commits to scan. All findings below are persistence checks on
open items from the Aug 25 audit.

### RF-1: omega/decide in agent-facing prompts (C-011 check)

**Finding**: `src/sdk/prompt.rs:358,360,388,390` lists `omega` and `decide` as example
Lean tactics in v2/v4 agent prompt variants.

**Status**: UNCHANGED since Aug 25. The Aug 25 audit determined this is **NOT a C-011
violation**:
- `omega` and `decide` are kernel-checked tactics (the proof kernel validates them)
- `native_decide` (kernel-bypassing) is NOT in the agent prompt
- C-011 intent (preventing kernel bypass) is satisfied
- The URGENT_20260516.md alert was resolved by the prior audit: the May 2026 auditor
  misread the tactic list; Aug 25 correctly cleared it

**Verdict**: NOT a violation. Close the May 2026 URGENT note.

### RF-2: hybrid condition labeled as 'n3' (C-032/C-033)

**Finding**: None. No instances located. Same result as Aug 25.

**Verdict**: No violation.

### RF-3: Hardcoded thresholds without env override (C-027) — OPEN ITEMS

Three behavioral constants remain hardcoded without env-override paths:

| File | Constant | Value | Env override? |
|------|----------|-------|---------------|
| `src/runtime/budget_allocation_telemetry.rs:144` | `MAX_PROPOSAL_TOKENS` | 900 | None |
| `src/web/spec.rs:1673` | `ACCEPTED_TURNS_FORCE_SYNTHESIS_THRESHOLD` | 10 | None |
| `src/web/generate.rs:70` | `MAX_GENERATE_ATTEMPTS` | 3 | None |

**Status**: UNCHANGED from Aug 25. No fix landed. These remain potential C-027
concerns: "所有影响行为的参数必须可通过环境变量/配置覆盖".

**Priority assessment** (advisory):
- `MAX_PROPOSAL_TOKENS` (MEDIUM): token budget cap — affects cost at scale
- `ACCEPTED_TURNS_FORCE_SYNTHESIS_THRESHOLD` (LOW): web flow UX only, `#[cfg(feature = "web")]`
- `MAX_GENERATE_ATTEMPTS` (LOW): low-frequency retry path

**Verdict**: C-027 potential concerns persist. Advisory; researcher decides priority.

### RF-4: Architecture wins claimed with tape dormant (C-033)

**Finding**: No new `PROVEN`/`DEFINITIVE`/`causal` efficiency headlines in any document.
No new commits to scan.

**Verdict**: No violation.

### RF-5: Citation-vs-precedent cross-check

No new plan or commit to cross-check. No violations found in the 20 commits
predating Aug 25 (verified by prior audit).

---

## Summary

| Step | Result |
|------|--------|
| Step 1 tooling self-check | PASS with persisting structural drift note (2nd cycle) |
| Step 2 case coverage | PASS — all 21 Art. IDs covered |
| Step 3 ACTIVE_USE_GAP | NONE |
| Step 4 drift scan | No new violations; 3 C-027 concerns still open from Aug 25 |

### Why YELLOW (not GREEN, not RED)

**Not GREEN** because:
1. Step 1 structural drift: `BusConfig::default()` is `Vec::new()`; enforcement migrated
   to predicate registry. Positive verification of `native_decide` succeeds via that path,
   but the audit task specification still names `BusConfig`/evaluator.rs as the check target —
   future auditors must consult `registry.rs:637`, not `bus.rs:48`
2. Three C-027 potential violations (hardcoded behavioral params without env override) are
   open for a second consecutive audit cycle with no fix

**Not RED** because:
- `native_decide` IS positively verified in production enforcement path (predicate registry)
- No C-011, C-032, C-033, or citation-vs-precedent violations found
- No `PROVEN`/`DEFINITIVE` efficiency claims in active documents
- constitution.md and cases/ are present and complete

### Action items for researcher (advisory, not blocking)

1. **[MEDIUM — second cycle]** Evaluate C-027 env-override for `MAX_PROPOSAL_TOKENS`
   (`src/runtime/budget_allocation_telemetry.rs:144`). Two audit cycles open without
   action is a signal this needs a decision: accept as deliberate design choice or add
   `std::env::var("TURINGOS_MAX_PROPOSAL_TOKENS")` override.

2. **[LOW — second cycle]** Audit protocol doc update: the scheduled task still references
   `experiments/minif2f_v4/src/bin/evaluator.rs` (removed in #345) and `BusConfig`
   literal as the Step 1 check target. Update to reference
   `src/top_white/predicates/registry.rs:forbidden_patterns_v1` as the canonical location.

3. **[LOW]** Art. V.3 covered by only C-071. Consider adding a second case if amendment
   activity resumes.

4. **[INFO]** The URGENT_20260516.md alert (C-011 / prompt.rs omega/decide) was resolved
   by the Aug 25 audit analysis. It can be archived; no action required.

---

*Auditor: JudgeAI-advisory (clean remote session, 2026-09-09)*
*Role boundary: advisory per Art. V.1.3 — findings only, no implementation proposals*
*Prior audit: DRIFT_AUDIT_20260825.md (YELLOW)*
