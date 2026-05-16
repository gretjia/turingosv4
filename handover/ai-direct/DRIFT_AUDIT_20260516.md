# Drift Audit — 2026-05-16

**Auditor role**: JudgeAI-advisory per Art. V.1.3 (advisory only; cannot block work)
**Status**: 🔴 RED
**Session**: clean remote session, no prior context bias

---

## Step 1 — Tooling Self-Check (Positive Verification)

**Target**: `forbidden_patterns` struct in `src/bus.rs`; runtime config in
`experiments/minif2f_v4/src/chain_runtime.rs`.

### Findings

| Check | Result | Evidence |
|-------|--------|----------|
| `forbidden_patterns` field exists in `BusConfig` | ✅ FOUND | `src/bus.rs:41` |
| `native_decide` present in production `forbidden_patterns` | ✅ POSITIVELY VERIFIED | `experiments/minif2f_v4/src/chain_runtime.rs:128` (primary) and `:336` (fallback) |
| `decide` present | ✅ | `:129, :337` |
| `omega` present | ✅ | `:130, :338` |
| Oracle-level `native_decide` check | ✅ | `experiments/minif2f_v4/src/lean4_oracle.rs:21` |

**Tooling self-check: PASS** — `forbidden_patterns` located; expected entries present in both
production BusConfig construction sites (primary chain_runtime path and WAL fallback path).

Note: the test fixture at `src/bus.rs:602` uses only `"FORBIDDEN"` — this is a unit-test
scaffold, not a production path. Not a regression.

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
Total: 21 unique Art. IDs.

### Coverage map

| Art. ID | Case files count | Status |
|---------|-----------------|--------|
| Art. I | 37 | ✅ |
| Art. I.1 | 11 | ✅ |
| Art. I.1.1 | 2 | ✅ |
| Art. I.2 | 5 | ✅ |
| Art. II | 17 | ✅ |
| Art. II.1 | 3 | ✅ |
| Art. II.2 | 7 | ✅ |
| Art. II.2.1 | 4 | ✅ |
| Art. III | 7 | ✅ |
| Art. III.1 | 1 | ✅ (minimal) |
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
| Art. V.3 | 1 | ✅ (minimal) |

**All 21 Art. IDs have at least one case file. No zero-coverage article.**

---

## Step 3 — Active-Use Coverage (ACTIVE_USE_GAP Analysis)

### Art. IDs heavily cited in recent plans (last 30 days, LATEST.md + recent TB docs)

Top citations in `LATEST.md`:
- Art. II.1 × 10, Art. III.1 × 8, Art. III × 8, Art. IV × 5, Art. III.4 × 4
- Art. I.1.1 × 4, Art. III.3 × 3, Art. III.2 × 3, Art. I.2 × 3

From TB ship docs and STEP_B docs (last 30 days):
- Art. II.1 — heavily cited in `STEPB_ART_II1_*` documents (3 files, multiple instances)
- Art. III.4 — cited in TB-3_RSP1
- Art. I.1 — cited in TB-2_SEQUENCER

### ACTIVE_USE_GAP analysis

Cross-referencing the most-cited articles against zero-case coverage:
**None found** — every heavily-cited article already has case coverage.

Low-priority theoretical gaps (informational):
- **Art. III.1** has only 1 case (C-022). It is cited 8 times in LATEST.md
  (shielding / context-poisoning). Single-case coverage is thin for an
  actively exercised article. Recommend adding a second case capturing
  the REAL-series shielding enforcement if an incident recurs.
- **Art. V.3** has only 1 case (C-071). The amendment log is now active
  (Art. 0 amendment 2026-04-26). If future amendments occur without the
  4-step process, C-071 should be the cite — coverage is adequate but
  narrow.

**ACTIVE_USE_GAP = empty** (no article with active plan citations and zero cases).

---

## Step 4 — Drift Scan

### 4.1 Recent commits (last 20)

```
04d1d9f Record REAL-12 R-022 enforcement log
8942b57 REAL-12 role-specialized economic agents
ac4f936 REAL-11 agent economic action activation
d2bfee5 REAL-10 controlled market evidence expansion
a1decf2 Add REAL-5S REAL-9 comprehensive progress report
0f22843 Record REAL-5S REAL-9 R-022 skip log
d87ad6c Ship REAL-5S through REAL-9 market activation
5239f4b Record REAL-4 prompt-only market activation evidence
0512b50 Close TB-G phase aggregate
a42b170 ship G4.2 model identity replay
...
```

### 4.2 🔴 RED FLAG FOUND: omega/decide in agent-facing prompts (C-011 violation)

**File**: `src/sdk/prompt.rs`
**Lines**: 347, 349 (v2 variant) and 377, 379 (v4 variant)

Content injected into agent-facing prompt for variants `v2` and `v4`:
```
arithmetic decision: omega / linarith / nlinarith / polyrith
simplification:      simp / aesop / decide
```

**Why this is a C-011 violation**:

C-011 (`cases/C-011_brute_force_formalization.yaml`) precedent §3 states:
> decide/omega/native_decide 应在 bus.rs forbidden_patterns 中拦截

The bus correctly blocks these. But `src/sdk/prompt.rs:347-379` actively suggests
`omega` and `decide` to agents as valid tactic families in the v2/v4 prompt variants.
This directly contradicts C-011's spirit: the case arose specifically from agents using
`decide` to brute-force enumerate instead of constructing proofs.

**Regression timeline**:
- `DRIFT_AUDIT_20260419.md:122` (2026-04-19) reported: "C-011 — omega/decide in
  agent-facing prompts: CLEAN ✅ — src/sdk/prompt.rs contains zero mentions of
  omega, decide, or native_decide"
- Session #34 (2026-05-10) introduced prompt variant experiment:
  commit `9b8c847` ("Prompt-variant experiment harness") added v2/v4 variants
  containing these tactic examples.
- The regression is confirmed by grep: `src/sdk/prompt.rs` now contains `omega`
  at lines 347, 377 and `decide` at lines 349, 379.

**Functional impact assessment**:
- The experiment results (`PROMPT_VARIANT_EXPERIMENT_RESULTS_2026-05-10.md`) showed
  v2/v4 had **zero behavioral effect** — the model ignored the extra prompt text.
- The bus enforcement still works: any step containing `omega` or `decide` is still
  vetoed via `chain_runtime.rs:128-130`.
- Risk: when running under `TURINGOS_PROMPT_VARIANT=v2` or `v4`, agents are told
  to try tactics the bus will always reject, wasting budget on guaranteed-veto steps.

**Resolution path** (advisory only — for ArchitectAI):
Either (a) remove `omega` and `decide` from the v2/v4 example lists, replacing
with bus-allowed alternatives (e.g., `linarith`, `nlinarith`, `simp`, `ring`), or
(b) explicitly ratify keeping these examples with a C-011 annotation explaining the
pedagogical purpose and the fact that the bus remains the enforcement layer.

### 4.3 CLEAN — n3 hybrid condition labeling (C-032/C-033)

`n3` appears in `experiments/minif2f_v4/src/bin/evaluator.rs` and `run_id.rs`
exclusively as a swarm-size parameter (3 agents). It is never labeled "hybrid" in
oracle-mode sense. `hybrid_v1` is explicitly deprecated at evaluator.rs:1433 with
a comment calling it a "Paper 1 era" artifact. No evidence of hybrid oracle mode
being mislabeled as a swarm condition.

**C-032/C-033 status: CLEAN.**

### 4.4 CLEAN — Hardcoded thresholds without env override (C-027)

- `agent_temperature_milli()` at `evaluator.rs:63-69` reads `TEMP_LADDER` env var.
- Budget regime reads `MAX_TX_OVERRIDE`, `BUDGET_REGIME`, `TURINGOS_CHAINTAPE_PATH`,
  etc. from env.
- `BusConfig` forbidden_patterns and size caps are constants at the runtime level,
  which is appropriate (not user-tunable benchmarking parameters).

**C-027 status: CLEAN.**

### 4.5 CLEAN — Architecture win claims with dormant tape/market/Boltzmann (C-033)

Recent REAL-10/REAL-11/REAL-12 commits and LATEST.md explicitly state:
- "E2 NOT ACHIEVED: no live non-scripted agent-generated router/short action"
- "E3 is not achieved: no persistent behavioral role differentiation claim"
- "No spontaneous market emergence claim"
- Boltzmann sequencer enforcement deferred to TB-19 (OBS_R024); this is a known
  open item, not a new regression. `BoltzmannMaskPolicy` is in production for
  observation; enforcement gate is explicitly deferred.

**C-033 status: CLEAN. No architecture win overclaim found in recent commits.**

### 4.6 CLEAN — Citation vs. precedent integrity

Spot-checked C-071 (constitution amendment process) vs. Art. V.1.1/V.1.2/V.3:
the case's ruling correctly cites the R-018 sudo gate mechanism and the 4-step
amendment process. Matches the constitution.md Art. V.3 amendment log content.

Spot-checked C-069 (constitutional alignment audit protocol) vs. Art. I/IV/V.1:
the case correctly identifies TRACE_MATRIX conformance as the enforcement mechanism.
No citation-vs-precedent mismatch found.

---

## Step 5 — Summary

### Overall Status: 🔴 RED

**Reason**: C-011 violation — `omega` and `decide` appear in agent-facing prompt
variants v2 and v4 at `src/sdk/prompt.rs:347, 349, 377, 379`. The previous clean
audit (2026-04-19) reported zero occurrences; the regression was introduced by the
2026-05-10 prompt variant experiment.

### Tooling self-check
✅ `forbidden_patterns` located and positively verified. `native_decide`, `decide`,
and `omega` confirmed present in all production BusConfig instantiations.

### ACTIVE_USE_GAP
None found. All Art. IDs cited in recent plans have case coverage.

### Drift findings
| Finding | Severity | File:line | Status |
|---------|----------|-----------|--------|
| omega/decide in agent prompt v2/v4 | 🔴 C-011 | `src/sdk/prompt.rs:347,349,377,379` | NEW regression (introduced 2026-05-10) |
| n3 hybrid mislabeling | ✅ CLEAN | — | no violation |
| Hardcoded thresholds (C-027) | ✅ CLEAN | — | env overrides present |
| Architecture win overclaim (C-033) | ✅ CLEAN | — | explicit non-claims documented |

### Low-priority theoretical coverage gaps (informational)
- Art. III.1 minimal coverage (1 case). Thin for active shielding article.
- Art. V.3 minimal coverage (1 case). Adequate but narrow for amendment log.
- Boltzmann sequencer enforcement gap (OBS_R024) is a known open item, deferred
  to TB-19. Not a new regression. Not escalated here.

---

*Audit written: 2026-05-16. Read-only except this file and URGENT companion.*
*Commit: drift audit: 2026-05-16*
