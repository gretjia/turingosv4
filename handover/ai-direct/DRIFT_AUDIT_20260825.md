# Drift Audit — 2026-08-25

**Auditor role**: JudgeAI-advisory per Art. V.1.3 (advisory only; cannot block work)
**Status**: 🟡 YELLOW
**Session**: clean remote session, no prior context bias
**HEAD**: b180971 (H-HET-2 converge: de-Lean merge + Codex P1 fixes + Gate-D/r022 #347)

---

## Step 1 — Tooling Self-Check (Positive Verification)

**Target**: `forbidden_patterns` struct in `src/bus.rs`; runtime config in
`experiments/minif2f_v4/src/bin/evaluator.rs`.

### Findings

| Check | Result | Evidence |
|-------|--------|----------|
| `forbidden_patterns` field exists in `BusConfig` | ✅ FOUND | `src/bus.rs:40` |
| `native_decide` in production `BusConfig::forbidden_patterns` literal | ⚠️ MIGRATED | See note below |
| `native_decide` in `forbidden_patterns_v1` predicate (production enforcement) | ✅ POSITIVELY VERIFIED | `src/top_white/predicates/registry.rs:637` |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | ❌ NOT FOUND | File does not exist; evaluator is separate binary |
| `experiments/minif2f_v4/src/chain_runtime.rs` (prior audit verified here) | ❌ REMOVED | De-Lean migration (#345, 2026-06-17) removed this file |

### DRIFT NOTE — enforcement mechanism migration

Prior audit (2026-05-16) positively verified `native_decide` in
`experiments/minif2f_v4/src/chain_runtime.rs:128,336` — that file configured
`BusConfig::forbidden_patterns` directly with `["native_decide", "decide", "omega", ...]`.

That file has since been **removed** by the de-Lean kernel migration (Class 4, commit `b16b9de`, PR #345).

**Current enforcement path** (as of HEAD b180971):

1. `src/top_white/predicates/registry.rs:631-643` registers `forbidden_patterns_v1`
   predicate with patterns: `["native_decide", "unsafe", "axiom "]`
2. `src/runtime/mod.rs:896` calls `activate_predicate_binding_for_boot()` at production ChainTape boot
3. `src/state/sequencer.rs:1937` calls `verify_work_predicates(...)` on every WorkTx submission
4. WorkTx payloads containing `native_decide` are REJECTED by the predicate gate

The production `BusConfig::default()` now has `forbidden_patterns: Vec::new()` (empty list,
`src/bus.rs:48`). The bus-level pattern gate is functionally bypassed; the predicate registry
is the **sole enforcer** of `native_decide` prohibition at WorkTx admission.

**Assessment**: This is an architectural migration, not a regression. The predicate registry
path is cryptographically stronger (merkle-rooted, on-chain). HOWEVER:
- The audit protocol points to `src/bus.rs` for positive verification
- The `BusConfig::forbidden_patterns` literal list is now always `[]` in production
- Future auditors must check the predicate registry, not the bus config

**Tooling self-check verdict**: PASS with migration note — `native_decide` IS enforced;
the enforcement surface has moved from bus-level to predicate registry.

---

## Step 2 — Constitution + Case Coverage

### Art. IDs extracted from `constitution.md`

```
Art. I, Art. I.1, Art. I.1.1, Art. I.2
Art. II, Art. II.1, Art. II.2, Art. II.2.1
Art. III, Art. III.1, Art. III.2, Art. III.3, Art. III.4
Art. IV
Art. V, Art. V.1, Art. V.1.1, Art. V.1.2, Art. V.1.3, Art. V.2, Art. V.3
```

Total: **21 unique Art. IDs**. 50 case files in `cases/`.

### Coverage map

| Art. ID | Case files | Status |
|---------|-----------|--------|
| Art. I | C-069 | ✅ |
| Art. I.1 | C-001, C-004, C-009, C-011, C-015, C-016, C-039, C-052, C-070, C-072 | ✅ |
| Art. I.1.1 | C-009, C-014 | ✅ |
| Art. I.2 | C-012, C-013, C-036, C-052, C-070 | ✅ |
| Art. II | (0 direct cases; covered via sub-articles) | ⚠️ minimal |
| Art. II.1 | C-009, C-017, C-018 | ✅ |
| Art. II.2 | C-019, C-020, C-069 | ✅ |
| Art. II.2.1 | C-005, C-021, C-030, C-036 | ✅ |
| Art. III | (0 direct cases; covered via sub-articles) | ⚠️ minimal |
| Art. III.1 | C-022 | ✅ (single case — thin) |
| Art. III.2 | C-003, C-023 | ✅ |
| Art. III.3 | C-024, C-025 | ✅ |
| Art. III.4 | C-006, C-026 | ✅ |
| Art. IV | C-007, C-008, C-027, C-028, C-029, C-030, C-037, C-041, C-043, C-069 | ✅ |
| Art. V | C-031, C-034, C-071, C-072 | ✅ |
| Art. V.1 | C-010, C-032, C-033, C-068, C-069, C-070 | ✅ |
| Art. V.1.1 | C-071, C-072, C-073, C-075 | ✅ |
| Art. V.1.2 | C-071, C-072, C-073, C-076 | ✅ |
| Art. V.1.3 | C-010, C-066, C-071, C-072, C-074 | ✅ |
| Art. V.2 | C-001, C-016, C-035, C-039 | ✅ |
| Art. V.3 | C-071 | ✅ (single case — thin) |

**All 21 Art. IDs have coverage (direct or via sub-articles). No zero-coverage article.**

---

## Step 3 — Active-Use Coverage (ACTIVE_USE_GAP Analysis)

### Citation frequency in handover files (all-time, n=all files)

Top-cited articles by occurrence count across `handover/`:

```
Art. V        1223    Art. IV       785    Art. II.1     457
Art. V.1.2     379    Art. V.1.3    345    Art. III.3    327
Art. I.1       311    Art. III.4    303    Art. II.2.1   303
Art. I.2       298    Art. III.2    290    Art. V.1.1    287
Art. III       247    Art. I.1.1    204    Art. V.3      195
Art. II.2      172    Art. V.1      123    Art. V.2      115
Art. I         115    Art. III.1    110    Art. II        68
```

### ACTIVE_USE_GAP

Intersect: "heavily cited in recent plans" ∩ "zero case files" =

**ACTIVE_USE_GAP = ∅** — no heavily-cited article has zero coverage.

### Low-priority theoretical gaps (informational, not RED)

- **Art. II (parent)**: 68 citations in handover docs, 0 direct case files.
  Sub-articles II.1 / II.2 / II.2.1 each have cases. Parent-level doctrine is
  implicit in the sub-article coverage. Low-priority to add an explicit
  parent-level case.

- **Art. III (parent)**: 247 citations, 0 direct case files.
  Sub-articles III.1-III.4 each have cases. Same situation as Art. II.
  Low-priority.

- **Art. III.1**: Single case (C-022 context_poisoning). Cited 110 times.
  The shielding doctrine is actively exercised; thin coverage. Recommend a
  second case if a shielding incident recurs.

- **Art. V.3**: Single case (C-071). The amendment process has been exercised
  once (Art. 0 amendment, 2026-04-26). Coverage adequate for now.

---

## Step 4 — Drift Scan

### Recent commits reviewed

```
b180971 H-HET-2 converge: de-Lean merge + Codex P1 fixes + Gate-D/r022 (#347)
78c3300 scripts: BSD/macOS-portable constitution gate runner (POSIX sed, wc strip) (#346)
b16b9de de-Lean kernel migration (Class 4): genericize Lean-branded kernel types, wire-safe (#345)
231ec17 autoloop: audited autoresearch loop harness (GATE-1 pre-flight + breakers + GATE-2 audit) (#344)
61ec26c U1: CORRECTION banners + forensic retrospective for 7 over-claiming reports (Class 0, R1.9B omega U1) (#342)
383c7e5 docs: sync README/LATEST/TB_LOG to main@7298b927 (#341)
7298b92 Merge #340: fail-closed agent-sig ingress (Class-4, §8 ALL-12)
09b3903 feat(tdma): surface operator VPPUT in tdma run (Class 1)
6649c68 Merge #338: e2e-1.0-hardening
```

### Red-flag scan results

#### RF-1: omega/decide in agent-facing prompts (C-011 check)

**Finding**: `src/sdk/prompt.rs:358,360,388,390` lists `omega` and `decide` as examples
of valid tactic families in agent prompts (prompt variants v2 and v4).

```
arithmetic decision: omega / linarith / nlinarith / polyrith
simplification:      simp / aesop / decide
```

**Verdict**: NOT a C-011 violation.
- `decide` and `omega` are kernel-checked tactics (proof kernel validates them)
- `native_decide` (kernel-bypassing) is NOT listed in agent prompts
- The `forbidden_patterns_v1` predicate correctly forbids `native_decide`, not bare `decide`/`omega`
- C-011 intent (prevent kernel bypass) is satisfied; listing legitimate tactics in prompts is correct

#### RF-2: hybrid condition labeled as 'n3' (C-032/C-033 check)

**Finding**: No instances of hybrid conditions labeled as `n3` in production source.

```
grep -rn "n3\b" src/ | grep -i "hybrid|condition|label|causal" → 0 hits
```

**Verdict**: NOT found. No C-032/C-033 violation detected.

#### RF-3: hardcoded thresholds without env override (C-027 check)

**Finding**: Three behavior-affecting constants found without env override:

| Location | Constant | Value | Env override? |
|----------|----------|-------|---------------|
| `src/runtime/budget_allocation_telemetry.rs:144` | `MAX_PROPOSAL_TOKENS` | 900 | None found |
| `src/web/spec.rs:1673` | `ACCEPTED_TURNS_FORCE_SYNTHESIS_THRESHOLD` | 10 | None found |
| `src/web/generate.rs:70` | `MAX_GENERATE_ATTEMPTS` | 3 | None found |

C-027 ruling: "所有影响行为的参数必须可通过环境变量/配置覆盖 — Default 值 OK，但不可是 const/hardcode"

**Verdict**: POTENTIAL C-027 CONCERN — these constants affect runtime behavior
(proposal token budget, synthesis forced after N turns, max generate retries). No
`std::env::var(...)` override path was found for any of them. This is advisory:
C-027 was triggered by critical failures (API limits, deadlocks); these constants
are lower-criticality. Researcher should evaluate whether env override is warranted,
especially `MAX_PROPOSAL_TOKENS` (token budget cap) and
`ACCEPTED_TURNS_FORCE_SYNTHESIS_THRESHOLD` (web UX flow).

Note: `ACCEPTED_TURNS_FORCE_SYNTHESIS_THRESHOLD` is `#[cfg(feature = "web")]` — only
affects the web feature. The rationale comment (`persona_1 reached 13 accepted turns ...
session timed out`) explains the design intent; env override would allow tuning without
recompile.

#### RF-4: claimed architecture win with tape/market/Boltzmann dormant (C-033 check)

**Finding**: No `PROVEN`/`DEFINITIVE` efficiency headlines found in `LATEST.md` or recent
commits. `LATEST.md` line 19 explicitly confirms a prior causal claim was VETOed by external
auditors and corrected. Commit #342 is a CORRECTION banner run, not a new causal claim.

**Verdict**: NOT found. No C-033 violation detected in active documents.

#### RF-5: citation-vs-precedent cross-check

No recent commit or plan was found to cite a C-xxx or Art. X precedent in a way that
contradicts the precedent's actual ruling. C-076 (most recently modified case) is an
accurate description of commit-claim-diff parity issues actually observed in Phase A/B
audit rounds.

---

## Summary

| Step | Result |
|------|--------|
| Step 1 tooling self-check | PASS with DRIFT NOTE (enforcement migrated to predicate registry) |
| Step 2 case coverage | PASS — all 21 Art. IDs covered |
| Step 3 ACTIVE_USE_GAP | NONE |
| Step 4 drift scan | 3 potential C-027 concerns; no C-011, C-032, C-033 violations |

### Why YELLOW (not GREEN, not RED)

**Not GREEN** because:
- Step 1 migration: the production `BusConfig::forbidden_patterns` is now `Vec::new()`; this
  is a structural change from prior audit's verified state that future auditors must know
- Three C-027 potential violations (hardcoded behavioral params) require researcher evaluation

**Not RED** because:
- `native_decide` IS positively verified in `forbidden_patterns_v1` at `registry.rs:637`
- The predicate enforcement path is active at boot (not dormant)
- No C-011, C-032, C-033, or citation-vs-precedent violations found
- No PROVEN/DEFINITIVE efficiency claims in active documents

### Action items for researcher (advisory, not blocking)

1. **[LOW]** Update drift audit Step 1 protocol: document that production `forbidden_patterns`
   check should now look at `src/top_white/predicates/registry.rs` (predicate registry), not
   `src/bus.rs` BusConfig literal. Update or supersede the chain_runtime.rs reference in prior
   audit doctrine.

2. **[MEDIUM]** Evaluate C-027 env-override for `MAX_PROPOSAL_TOKENS` (token budget) and
   `ACCEPTED_TURNS_FORCE_SYNTHESIS_THRESHOLD` (web flow cutoff). Both affect runtime behavior
   without recompile path. Low-risk constants; researcher decides priority.

3. **[LOW]** Consider adding direct Art. II and Art. III parent-level cases if the parent
   articles carry distinct doctrine beyond their sub-articles.

---

*Auditor: JudgeAI-advisory (clean remote session, 2026-08-25)*
*Role boundary: advisory per Art. V.1.3 — findings only, no implementation proposals*
