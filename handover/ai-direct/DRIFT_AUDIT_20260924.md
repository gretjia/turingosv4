# Drift Audit — 2026-09-24

**Auditor role**: JudgeAI-advisory per Art. V.1.3 (advisory only; cannot block work)
**Status**: 🟡 YELLOW
**Session**: clean remote session, no prior context bias
**HEAD**: adaf928 (drift audit: 2026-08-25)
**Prior audit**: DRIFT_AUDIT_20260825.md (HEAD b180971)
**Code delta since last audit**: **NONE** — zero new commits. HEAD = adaf928 (the prior audit commit itself).

---

## Step 1 — Tooling Self-Check (Positive Verification)

**Target**: `forbidden_patterns` struct in `src/bus.rs`; note that
`experiments/minif2f_v4/src/bin/evaluator.rs` was removed in de-Lean migration (#345).

### Findings

| Check | Result | Evidence |
|-------|--------|----------|
| `forbidden_patterns` field exists in `BusConfig` | ✅ FOUND | `src/bus.rs:40` |
| `native_decide` in production `BusConfig::forbidden_patterns` default | ⚠️ EMPTY — documented migration | `src/bus.rs:48` (`Vec::new()`) |
| `native_decide` in `forbidden_patterns_v1` predicate (production enforcement) | ✅ POSITIVELY VERIFIED | `src/top_white/predicates/registry.rs:637` |
| Predicate enforcement active at boot | ✅ VERIFIED (prior audit) | `src/runtime/mod.rs:896`, `src/state/sequencer.rs:1937` |

**Assessment (same as prior audit)**: `native_decide` IS enforced via the predicate
registry, not via `BusConfig::forbidden_patterns`. This architectural migration was
Class 4 (de-Lean, PR #345) and is documented in DRIFT_AUDIT_20260825.md. No regression
from prior audit state.

**Tooling self-check verdict**: PASS — positive verification achieved via predicate registry.

---

## Step 2 — Constitution + Case Coverage

### Art. IDs extracted from `constitution.md`

My extraction yields **26 unique Art. IDs**:

```
Art. 0,  Art. 0.1, Art. 0.2, Art. 0.3, Art. 0.4   ← Chapter "图灵机原教旨"
Art. I,  Art. I.1, Art. I.1.1, Art. I.2
Art. II, Art. II.1, Art. II.2, Art. II.2.1
Art. III, Art. III.1, Art. III.2, Art. III.3, Art. III.4
Art. IV
Art. V,  Art. V.1, Art. V.1.1, Art. V.1.2, Art. V.1.3, Art. V.2, Art. V.3
```

**⚠️ PRIOR AUDIT CORRECTION**: DRIFT_AUDIT_20260825.md reported "21 unique Art. IDs"
and stated "All 21 Art. IDs have coverage." That count missed Art. 0, 0.1, 0.2, 0.3,
0.4 (the "Turing Machine Orthodoxy" chapter added 2026-04-26). The prior audit's
coverage-complete claim was incorrect.

### Coverage map (cases/*.yaml)

Total case files in `cases/`: 47 YAML files.

| Art. ID | Cases | Status |
|---------|-------|--------|
| **Art. 0** | 0 | ❌ NO COVERAGE |
| **Art. 0.1** | 0 | ❌ NO COVERAGE |
| **Art. 0.2** | 0 | ❌ NO COVERAGE |
| **Art. 0.3** | 0 | ❌ NO COVERAGE |
| **Art. 0.4** | 0 | ❌ NO COVERAGE |
| Art. I | ≥1 (C-069) | ✅ |
| Art. I.1 | ≥8 | ✅ |
| Art. I.1.1 | 2 | ✅ |
| Art. I.2 | ≥5 | ✅ |
| Art. II | 0 direct; sub-articles covered | ⚠️ minimal (same as prior) |
| Art. II.1 | ≥3 | ✅ |
| Art. II.2 | ≥3 | ✅ |
| Art. II.2.1 | ≥4 | ✅ |
| Art. III | 0 direct; sub-articles covered | ⚠️ minimal (same as prior) |
| Art. III.1 | 1 (C-022) | ✅ thin |
| Art. III.2–III.4 | ≥2 each | ✅ |
| Art. IV | ≥10 | ✅ |
| Art. V | ≥4 | ✅ |
| Art. V.1–V.3 | ≥1 each | ✅ |

**Summary**: Art. 0.x (5 articles) = zero case coverage. All other articles covered.

---

## Step 3 — Active-Use Coverage (ACTIVE_USE_GAP)

### Citation frequency across ALL handover docs

```
Art. 0.2:   629 citations     ← THE MOST CITED ARTICLE IN HANDOVER, ZERO CASES
Art. 0.4:   242 citations     ← zero cases
Art. 0:     116 citations     ← zero cases
Art. 0.3:    48 citations     ← zero cases
Art. 0.1:    42 citations     ← zero cases
```

### ACTIVE_USE_GAP (heavily cited AND zero cases)

| Art. ID | Citations | Cases | Gap severity |
|---------|-----------|-------|--------------|
| Art. 0.2 (Tape Canonical axiom) | 629 | 0 | **HIGH** |
| Art. 0.4 (Q_t version-controlled state) | 242 | 0 | **HIGH** |
| Art. 0 (Turing Machine Orthodoxy parent) | 116 | 0 | **MEDIUM** |
| Art. 0.3 (Blockchain reserved) | 48 | 0 | LOW |
| Art. 0.1 (Four-element mapping) | 42 | 0 | LOW |

**ACTIVE_USE_GAP = {Art. 0.2, Art. 0.4, Art. 0}** (HIGH + HIGH + MEDIUM)

Art. 0.2 is the single most-cited article in the entire handover corpus (629 hits vs.
next-highest among any article) and governs the Tape Canonical axiom — all signals
must be reconstructable from tape. This article is actively invoked in TB charters and
LATEST.md yet has no test case to anchor what "violation" looks like.

### Phantom citation note

**Art. 0.5**: 50 citations in handover, NOT present in constitution.md.
The prior audit correctly identified this as a DRAFT reference that was never enacted.
LATEST.md back-out section states: _"DRAFT documents (Art 0.5, PREREG_v2): never enacted;
safe to discard or rewrite."_ However, the LATEST.md pending-actions table still lists
it without a `[DRAFT]` marker — unresolved from prior audit's recommendation.

---

## Step 4 — Drift Scan

### Code delta since prior audit

**Zero code commits** since 2026-08-25. HEAD = adaf928 (the audit commit itself).
All prior-audit red-flag scan results carry forward unchanged.

### Red-flag scan results (carry-forward + spot verification)

#### RF-1: omega/decide in agent-facing prompts (C-011)

`src/bin/livefc1_lean_runner.rs:1003` still lists `decide` and `omega` as Lean tactics.
This is NOT a C-011 violation (they are kernel-checked tactics; `native_decide` is the
kernel-bypass token and it is not present in agent-facing prompts).

**Verdict**: NOT a violation. Confirmed clean.

#### RF-2: hybrid condition labeled as 'n3' (C-032/C-033)

`src/runtime/proposal_telemetry.rs:714`: `fresh_record("n3", "n3.b0")` is in a unit test
(§8 round-trip test for model_id field). Not a production hybrid condition label.

**Verdict**: NOT a violation. No C-032/C-033 issue.

#### RF-3: hardcoded thresholds without env override (C-027) — CARRY-FORWARD

Three items from prior audit unresolved (zero commits = zero changes):

| Location | Constant | Value | Env override |
|----------|----------|-------|--------------|
| `src/runtime/budget_allocation_telemetry.rs:144` | `MAX_PROPOSAL_TOKENS` | 900 | None |
| `src/web/spec.rs:1673` | `ACCEPTED_TURNS_FORCE_SYNTHESIS_THRESHOLD` | 10 | None |
| `src/web/generate.rs:70` | `MAX_GENERATE_ATTEMPTS` | 3 | None |

Also flagged (new this audit):
| `src/bin/lean_market_agent.rs:117-119` | `PROOF_TEMPERATURE`, `ROUTE_TEMPERATURE`, `BEAR_TEMPERATURE` | 0.7/0.7/0.3 | None |
| `src/bin/het_capability_probe.rs:63` | `PROOF_TEMPERATURE` | 0.7 | None |

Temperature constants are behavior-affecting (C-027: "所有影响行为的参数必须可通过环境变量覆盖").
However they ARE exposed as CLI args (`proof_temperature` arg in lean_market_agent at lines
1753-1755, 1854-1855) which provides override capability via command-line even without env var.
This partially satisfies C-027 intent. CLI arg path ≠ env var, but the override path exists.

**Verdict**: POTENTIAL C-027 concern for the 3 runtime constants. Temperature constants are
partially mitigated by CLI args. Researcher's call on priority.

#### RF-4: PROVEN/causal headline check (§17.1)

No new commits → no new claims. `LATEST.md` header continues to show the
P1 market-routing NO-GO verdict (external-audited) with no new causal efficiency claims.

**Verdict**: Clean.

#### RF-5: Citation-vs-precedent check

Recent commit subjects reviewed:
- `adaf928` "drift audit: 2026-08-25" — no code changes, no mismatch
- `b180971` "H-HET-2 converge" (last real change, 2026-06-18) — prior audit verified

**Verdict**: No citation-vs-precedent mismatch found.

---

## Summary

| Step | Result |
|------|--------|
| Step 1 tooling self-check | PASS — `native_decide` enforced via predicate registry |
| Step 2 case coverage | **NEW FINDING** — 5 Art. 0.x articles missing; prior audit overclaimed "all covered" |
| Step 3 ACTIVE_USE_GAP | **HIGH** — Art. 0.2 (629 cites, 0 cases), Art. 0.4 (242 cites, 0 cases) |
| Step 4 drift scan | No new violations; 3+ C-027 items carry forward from prior audit |

### Why YELLOW (not GREEN, not RED)

**Not GREEN** because:
1. Art. 0.x (5 articles) have zero case coverage despite very high citation rates
2. Art. 0.2 (629 citations) and Art. 0.4 (242 citations) are ACTIVE_USE_GAP
3. Three C-027 constants carry forward unresolved from prior audit
4. Art. 0.5 phantom citation still unresolved in LATEST.md table

**Not RED** because:
- Step 1 positive verification PASSED (`native_decide` IS enforced)
- No C-011, C-032/C-033, or citation-vs-precedent violations
- No PROVEN/causal claims in active documents
- Gaps are coverage gaps (no cases for existing articles), not regressions or violations

### Action items for researcher (advisory, not blocking)

1. **[HIGH]** Create case files for Art. 0.2 (Tape Canonical axiom) and Art. 0.4
   (Q_t version-controlled state). With 629 and 242 handover citations respectively,
   these are the most-invoked unanchored articles. A case for "claim X requires tape
   reconstruction" and "HEAD_t/q_t/rtool/wtool triple not implemented" would give
   auditors concrete precedent to cite.

2. **[MEDIUM]** Correct LATEST.md pending-actions table: mark the Art. 0.5 entry as
   `[DRAFT — not enacted in constitution.md]` per prior audit's recommendation.

3. **[MEDIUM]** Evaluate C-027 env-override for `MAX_PROPOSAL_TOKENS` and
   `ACCEPTED_TURNS_FORCE_SYNTHESIS_THRESHOLD` (carry-forward from prior audit).

4. **[LOW]** Update drift audit Step 2 methodology documentation: constitution grep
   for Art. IDs must include `Art\. 0` prefix to catch the Turing Machine Orthodoxy
   chapter (prior audit's regex may have started matching from Roman numerals).

---

## Continuity note

Next drift audit should check:
- Whether Art. 0.x case files were created (closing the ACTIVE_USE_GAP)
- Whether Art. 0.5 LATEST.md table entry was marked DRAFT
- Whether any C-027 env-override work was done
- Whether new code commits introduced new red flags

---

*Auditor: JudgeAI-advisory (clean remote session, 2026-09-24)*
*Role boundary: advisory per Art. V.1.3 — findings only, no implementation proposals*
