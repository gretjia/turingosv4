# Constitutional Drift Audit — 2026-09-01

**Auditor**: JudgeAI-advisory (clean remote session)
**Role**: Advisory per Art. V.1.3 — findings only, no implementation proposals
**Previous audit**: 2026-08-25 (commit adaf928)
**Date**: 2026-09-01

---

## Status: YELLOW (unchanged from 2026-08-25)

**Not GREEN** because:
- Production `BusConfig::forbidden_patterns` defaults to `Vec::new()` — enforcement
  has migrated to the predicate registry (`registry.rs:637`), not the BusConfig literal.
  Same structural migration note as prior audit.
- Three C-027 potential violations (hardcoded behavioral params) remain open from
  prior audit — researcher has not yet addressed them.

**Not RED** because:
- `native_decide` IS positively verified in `forbidden_patterns_v1` at
  `src/top_white/predicates/registry.rs:637`
- No C-011 violations (omega/decide in agent prompts cleared in 2026-08-25 audit)
- No new code commits since 2026-08-25 audit — repo is stable
- No C-032, C-033, or citation-vs-precedent mismatches

---

## Step 1 — Tooling Self-Check

**Target**: `src/bus.rs` and `experiments/minif2f_v4/src/bin/evaluator.rs`

| Check | Result |
|-------|--------|
| `BusConfig.forbidden_patterns` field exists | ✅ `src/bus.rs:40` |
| `native_decide` in `BusConfig.forbidden_patterns` literal | ⚠️ MIGRATED (see note) |
| `native_decide` in `forbidden_patterns_v1` predicate (production enforcement) | ✅ `src/top_white/predicates/registry.rs:637` |
| Evaluator constructs `BusConfig` with `forbidden_patterns` | ⚠️ No `forbidden_patterns` set in evaluator — default `Vec::new()` |

**Migration note** (unchanged from 2026-08-25):
- `BusConfig::default()` has `forbidden_patterns: Vec::new()` (bus.rs:48)
- The only test fixture uses `vec!["FORBIDDEN".to_string()]` (bus.rs:545) — not production
- Production enforcement of `native_decide` is at
  `src/top_white/predicates/registry.rs:631–650` via `forbidden_patterns_v1` boot predicate:
  `patterns: ["native_decide", "unsafe", "axiom "]`
- The bus.rs comment (line 183) explicitly cites `native_decide` as an example of what
  the "math-domain checker driver supplies" — this architecture is intentional, not an oversight
- C-011 ruling clause 3 says "decide/omega/native_decide 应在 bus.rs forbidden_patterns
  中拦截" — the predicate path satisfies the enforcement intent even though the
  enforcement point has migrated from BusConfig literal to predicate registry

**Tooling self-check verdict**: PASS with migration note — `native_decide` IS enforced;
enforcement path is `predicates/registry.rs`, not `BusConfig` literal.

---

## Step 2 — Constitution + Cases Coverage

**Art. IDs in constitution.md**: 21 identifiers

```
Art. I, Art. I.1, Art. I.1.1, Art. I.2
Art. II, Art. II.1, Art. II.2, Art. II.2.1
Art. III, Art. III.1, Art. III.2, Art. III.3, Art. III.4
Art. IV
Art. V, Art. V.1, Art. V.1.1, Art. V.1.2, Art. V.1.3, Art. V.2, Art. V.3
```

**Coverage map**: All 21 Art. IDs covered by ≥1 case file. No uncovered articles.

| Article | Sample cases |
|---------|-------------|
| Art. I | C-001, C-003, C-004, C-005, C-011, C-015, C-016, ... (many) |
| Art. I.1 | C-001, C-004, C-011, C-015, C-016, C-039, C-052, C-070, C-072 |
| Art. I.1.1 | C-009, C-014 |
| Art. I.2 | C-012, C-013, C-036, C-052, C-070 |
| Art. II | C-003, C-005, C-006, C-017, C-018, C-019, C-020, C-021, C-022, C-023 |
| Art. II.1 | C-009, C-017, C-018 |
| Art. II.2 | C-005, C-019, C-020, C-021, C-030, C-036, C-069 |
| Art. II.2.1 | C-005, C-021, C-030, C-036 |
| Art. III | C-003, C-006, C-022, C-023, C-024, C-025, C-026 |
| Art. III.1 | C-022 |
| Art. III.2 | C-003, C-023 |
| Art. III.3 | C-024, C-025 |
| Art. III.4 | C-006, C-026 |
| Art. IV | C-007, C-008, C-027, C-028, C-029, C-030, C-037, C-041, C-043, C-069 |
| Art. V | C-001, C-010, C-016, C-031, C-032, C-033, C-034, C-035, C-039, C-066, C-068, C-069, C-070, C-071, C-072, C-073, C-074, C-075, C-076 |
| Art. V.1 | C-010, C-032, C-033, C-066, C-068, C-069, C-070, C-071, C-072, C-073, C-074, C-075, C-076 |
| Art. V.1.1 | C-071, C-072, C-073, C-075 |
| Art. V.1.2 | C-071, C-072, C-073, C-076 |
| Art. V.1.3 | C-010, C-066, C-071, C-072, C-074 |
| Art. V.2 | C-001, C-016, C-035, C-039 |
| Art. V.3 | C-071 |

**Coverage verdict**: ✅ FULL — no uncovered articles.

---

## Step 3 — Active-Use Coverage (ACTIVE_USE_GAP)

**Recent activity**: `git log --oneline --since=2026-08-25` returns only the prior
drift audit commit (`adaf928 drift audit: 2026-08-25`). No code commits in the past 7 days.

**Recent handover plans**: The most recent non-audit plans are TB-series from 2026-05.
Art. IDs heavily cited in `LATEST.md` and recent TB plans: Art. I, Art. I.1, Art. IV,
Art. V, Art. V.1, Art. V.1.2, Art. V.1.3.

**ACTIVE_USE_GAP**: NONE — all actively-cited Art. IDs have ≥1 case coverage.

---

## Step 4 — Drift Scan

### Commits since 2026-08-25

```
adaf928 drift audit: 2026-08-25   (only commit)
```

**No new code commits since the last audit.** The repository is in a stable state.

### Recent cases (3 most recently modified)

**C-072** (Veto-AI scope narrowing): Well-formed. Correctly cites Art. V.1.3 and FC3-N5/E17.
Output domain `{PASS, VETO}` properly frozen. No anomaly.

**C-073** (ArchitectAI commit authority): Well-formed. Correctly cites Art. V.1.2 amendment
and Art. V.1.1. Distinguishes sudo (constitution.md only) from ArchitectAI commit scope.
No anomaly.

**C-074** (FC-first problem handling): Well-formed. Cites FC1, FC2, FC3 nodes correctly.
R-016 `fc_trace_in_commit` hook referenced as implemented (commit 2e7f75a). No anomaly.

### RF-1: omega/decide in agent-facing prompts (C-011 check)

`src/sdk/prompt.rs:358,360,388,390` lists `omega` and `decide` in v2/v4 tactic search
guidance. This is **unchanged** from 2026-08-25 audit.

**Verdict**: NOT a C-011 violation (same as 2026-08-25):
- `decide` and `omega` are kernel-checked tactics — valid proof steps
- `native_decide` (kernel-bypassing) is NOT listed in agent prompts
- The `forbidden_patterns_v1` predicate correctly forbids `native_decide`, not bare `decide`/`omega`
- C-011 intent (prevent kernel bypass) is satisfied

### RF-2: hybrid condition labeled as 'n3' (C-032/C-033 check)

`src/runtime/adapter.rs` uses `tb_n3_*` function names (e.g., `tb_n3_emit_node_market_after_work_accept`,
`tb_n3_invest_to_router_tx`). These names use `n3` as an identifier for a legitimate
runtime function family, not as a causal attribution label for a hybrid condition.
No evidence of "hybrid condition labeled as n3" in any report, commit, or LATEST.md.

**Verdict**: NOT a C-032/C-033 violation.

### RF-3: hardcoded thresholds without env override (C-027 check)

Three persistent concerns (unchanged from 2026-08-25 audit):

1. `src/bin/het_capability_probe.rs:63` — `const PROOF_TEMPERATURE: f64 = 0.7` (hardcoded)
2. `src/bin/turingos/cmd_tdma.rs:320` — `let mut temperature: f32 = 0.7` (mutable default, no env)
3. `src/bin/turingos/cmd_generate.rs:1352` — `temperature: 0.2` (hardcoded in struct literal)

These are **unchanged** from the 2026-08-25 audit. Researcher has not addressed these.
They remain YELLOW-priority items.

### RF-4: claimed architecture win with tape/market/Boltzmann dormant (C-033 check)

No new `PROVEN`/`DEFINITIVE` efficiency headlines in any commit subject, LATEST.md, or
recent plan since 2026-08-25. No C-033 violation.

### RF-5: citation-vs-precedent cross-check

C-072, C-073, C-074 all cite correct constitutional articles and their rulings accurately
reflect the stated constitutional clauses. No mismatch.

---

## Summary

| Step | Result |
|------|--------|
| Step 1 tooling self-check | PASS with migration note (same as 2026-08-25) |
| Step 2 case coverage | ✅ FULL — all 21 Art. IDs covered |
| Step 3 ACTIVE_USE_GAP | NONE |
| Step 4 drift scan | No new violations; 3 persistent C-027 concerns (unchanged) |
| Commits since last audit | 0 code commits — repo stable |

### Why YELLOW (not GREEN, not RED)

**Not GREEN** because:
- Production `BusConfig::forbidden_patterns` is `Vec::new()` (structural migration note
  still requires future auditors to look at predicate registry, not BusConfig literal)
- Three C-027 hardcoded temperature params persist without env override

**Not RED** because:
- `native_decide` positively verified in `forbidden_patterns_v1` at `registry.rs:637`
- No C-011, C-032, C-033, or citation-vs-precedent violations
- No PROVEN/DEFINITIVE efficiency claims in active documents
- No code changes since 2026-08-25 — no regression opportunity

### Open action items (advisory, not blocking — carried from 2026-08-25)

1. **[LOW]** Update drift audit Step 1 doctrine to explicitly reference
   `src/top_white/predicates/registry.rs` as the production enforcement location for
   `native_decide` prohibition, superseding the `bus.rs BusConfig literal` reference.

2. **[MEDIUM]** Evaluate env-override paths for:
   - `PROOF_TEMPERATURE` in `het_capability_probe.rs:63`
   - `temperature` default in `cmd_tdma.rs:320`
   - `temperature` in `cmd_generate.rs:1352`
   All affect runtime behavior without recompile path (C-027 spirit).

3. **[LOW]** Consider Art. II and Art. III parent-level standalone cases if the parent
   articles carry doctrine distinct from their sub-articles.

---

*Auditor: JudgeAI-advisory (clean remote session, 2026-09-01)*
*Role boundary: advisory per Art. V.1.3 — findings only, no implementation proposals*
