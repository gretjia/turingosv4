# Drift Audit — 2026-04-27

**Status: YELLOW**
**Auditor**: JudgeAI-advisory (Art. V.1.3) — clean remote session, no local bias
**Date**: 2026-04-27
**Scope**: constitution.md + cases/*.yaml + evaluator.rs + recent plans (last ~30 days)

---

## Step 1 — Tooling Self-Check

**Result: POSITIVE VERIFICATION PASS ✅**

- `forbidden_patterns` field located at `src/bus.rs:21` (`pub forbidden_patterns: Vec<String>`)
- Literal list confirmed at `experiments/minif2f_v4/src/bin/evaluator.rs:575-583`:
  ```
  "native_decide", "decide", "omega", "#eval", "IO.Process", "IO.FS", "run_tac", "unsafe"
  ```
- `native_decide` confirmed present ✅
- Fallback config (WAL open-fail path) at evaluator.rs:606-613 has identical list ✅
- Auditor is NOT blind. Tooling is intact.

---

## Step 2 — Constitution Art. ID Inventory

26 Art. IDs extracted from `constitution.md`:

```
Art. 0, 0.1, 0.2, 0.3, 0.4
Art. I, I.1, I.1.1, I.2
Art. II, II.1, II.2, II.2.1
Art. III, III.1, III.2, III.3, III.4
Art. IV
Art. V, V.1, V.1.1, V.1.2, V.1.3, V.2, V.3
```

### Coverage Map (Art. ID → cases/*.yaml count)

| Art. ID | Case Count | Sample Files |
|---------|------------|--------------|
| Art. 0 | **0** | ← ZERO |
| Art. 0.1 | **0** | ← ZERO |
| Art. 0.2 | **0** | ← ZERO |
| Art. 0.3 | **0** | ← ZERO |
| Art. 0.4 | **0** | ← ZERO |
| Art. I | 37 | many |
| Art. I.1 | 11 | C-001, C-009, C-011, C-014, C-015, C-016, C-039, C-052, C-070, C-072 |
| Art. I.1.1 | 2 | C-009, C-014 |
| Art. I.2 | 5 | C-012, C-013, C-036, C-052, C-070 |
| Art. II | 17 | many |
| Art. II.1 | 3 | C-009, C-017, C-018 |
| Art. II.2 | 7 | C-005, C-019, C-020, C-021, C-030, C-036, C-069 |
| Art. II.2.1 | 4 | C-005, C-021, C-030, C-036 |
| Art. III | 7 | C-003, C-006, C-022, C-023, C-024, C-025, C-026 |
| Art. III.1 | 1 | C-022 |
| Art. III.2 | 2 | C-003, C-023 |
| Art. III.3 | 2 | C-024, C-025 |
| Art. III.4 | 2 | C-006, C-026 |
| Art. IV | 10 | C-007, C-008, C-027, C-028, C-029, C-030, C-037, C-041, C-043, C-069 |
| Art. V | 19 | many |
| Art. V.1 | 13 | many |
| Art. V.1.1 | 4 | C-071, C-072, C-073, C-075 |
| Art. V.1.2 | 4 | C-071, C-072, C-073, C-076 |
| Art. V.1.3 | 5 | C-010, C-066, C-071, C-072, C-074 |
| Art. V.2 | 4 | C-001, C-016, C-035, C-039 |
| Art. V.3 | 1 | C-071 |

---

## Step 3 — Active-Use Coverage (ACTIVE_USE_GAP Analysis)

### Recently-cited Art. IDs in handover plans (last 30 days)

From `LATEST.md`, `OPEN_DECISIONS_2026-04-26.md`, `V4_PROJECT_OVERVIEW_2026-04-27.md`:

| Art. ID | Citation Count in Plans | Case Coverage |
|---------|------------------------|---------------|
| Art. 0.4 | 3 | **ZERO — ACTIVE_USE_GAP** |
| Art. 0.2 | 2 | **ZERO — ACTIVE_USE_GAP** |
| Art. 0.3 | 1 | **ZERO — ACTIVE_USE_GAP** |
| Art. 0.1 | 1 | **ZERO — ACTIVE_USE_GAP** |
| Art. 0 | 1 | **ZERO — ACTIVE_USE_GAP** |

**ACTIVE_USE_GAP**: All five Art. 0.x articles were added on 2026-04-26 (yesterday) and are already being actively cited for critical architectural decisions (D-VETO-6 in LATEST.md cites Art. 0.2 as the "sidecar warning" authority). No case precedent exists for any of them. This is expected for brand-new articles, but the gap must be tracked and cases written before Art. 0.x is used as authority for VETO decisions.

### Low-priority theoretical gaps (not ACTIVE_USE, informational only)

- Art. I.1.1: only 2 cases (C-009, C-014) — thin precedent for PCP predicate edge cases
- Art. V.2: 4 cases — adequate but constitution examples not all covered
- Art. V.3: 1 case (C-071) — amendment process barely precedented

---

## Step 4 — Drift Scan

### Red Flag 1: omega/decide in agent-facing prompts (C-011)
**CLEAN ✅**

All occurrences of "omega", "decide", "native_decide" outside of comments in evaluator.rs are:
- `forbidden_patterns` literal list (lines 575-583, 606-613) — correct
- `verify_omega`, `omega_payload_hashes`, `omega_attempts`, `omega_node_id`, `omega_wtool` — internal evaluator variable names, not injected into agent prompts

Agent-facing system prompt (lines 395-400) contains only: Lean 4 proof completion instruction, no brute-force tactic hints. C-011 compliant.

### Red Flag 2: hybrid condition labeled as 'n3' (C-032/C-033)
**CLEAN ✅**

`n3` in evaluator.rs:251, 1826 is a valid swarm-size condition `CONDITION=n3` (N=3 agents). The `hybrid_v1` condition at evaluator.rs:297-308 is explicitly deprecated with comment:
> "hybrid_v1 was a Paper 1 era ... arc does NOT use hybrid_v1 — it operates exclusively on oneshot"

The `parse_swarm_condition_n("hybrid_v1")` returns `None` (line 1842). No hybrid condition is mislabeled as n3.

### Red Flag 3: Hardcoded thresholds without env override (C-027)
**YELLOW ⚠️ — Two findings:**

**Finding A** — `temperature: Some(0.2)` at `evaluator.rs:405` (inside `run_oneshot_problem`):
- The N-agent path has `TEMP_LADDER` env var (lines 725, 1017) to override temperatures.
- The `oneshot` path has **no env override**. Temperature is unconditionally hardcoded.
- This is a C-027 violation: a behavior-affecting parameter not env/config overridable.
- Evidence: `grep TEMP evaluator.rs` returns only `TEMP_LADDER` references (lines 725, 1017), not applicable to the oneshot function.

**Finding B** — `system_lp_amount: 200.0` at `evaluator.rs:573,605` and `src/bus.rs:29`:
- `BOUNTY_LP` env override exists only for the `HAYEK_BOUNTY` bounty market path (`bus.rs:147-148`).
- The per-node market liquidity `system_lp_amount` used in `bus.rs:289` (regular market creation) has **no `SYSTEM_LP_AMOUNT` env override**.
- Three hardcoded occurrences: `evaluator.rs:573`, `evaluator.rs:605` (WAL fallback), `bus.rs:29` (default).
- This is a C-027 violation: market liquidity behavior-affecting parameter not env-overridable.

### Red Flag 4: Architecture win claimed with tape/market/Boltzmann dormant (C-033)
**CLEAN ✅**

- **Boltzmann**: ACTIVE — `BoltzmannParams::from_env()` at evaluator.rs:692; `boltzmann_select_parent` called at evaluator.rs:1055. Seed is env-controlled (BOLTZMANN_SEED).
- **Markets**: ACTIVE — `bus.kernel.markets.len()` checked at evaluator.rs:839; `bus.kernel.create_market` called in `bus.rs:289`; market_ticker emitted in tick log (line 840).
- **Tape**: ACTIVE — WAL persistence available via WAL_DIR env.

No C-033 violation found. Architecture claims with dormant-mechanism evidence absent.

### Red Flag 5: Phantom Art. ID citation
**YELLOW ⚠️ (minor)**

`Art. 0.5` appears at `handover/ai-direct/LATEST.md:144` in the pending-actions table:
> "Constitution Art. 0.5 enactment (cp workflow) — only after D2 confirmed"

Art. 0.5 does NOT exist in `constitution.md`. LATEST.md itself correctly notes in its back-out section: _"DRAFT documents (Art 0.5, PREREG_v2): never enacted; safe to discard or rewrite."_ This is not a live constitutional citation — it is a forward-reference to a planned future amendment. However, if an agent reads the LATEST.md pending-actions table without reading the back-out disclaimer, it may treat Art. 0.5 as live authority. Recommend LATEST.md table mark this entry explicitly as `[DRAFT — not yet in constitution]`.

### Commit-to-citation cross-check (recent log)

```
8d144d8  CO1.3.1 git substrate spike — 8/8 PASS
1e3f2ff  spec v1.3 + Plan v3.2-fix3: Codex round-2 re-audit fixes
8285e66  spec v1.2 re-audit launched
6d0e6d9  spec v1.2 + Plan v3.2-fix2: combined Codex+Gemini freeze audit fixes
0101fa0  fix TR: refresh SHAs for ART_0_2_REINTERPRETATION + ENACTMENT_PROCEDURE
```

- `8d144d8` (CO1.3.1 git spike): References Art. 0.4 path decision (A/B/C). Spike is in `spike/gix_capability/` isolated workspace; no changes to `src/`. Valid use of Art. 0.4's "Path B: true git substrate" language. Citation-to-precedent: Art. 0.4 supports this inquiry ✅
- `0101fa0` (TR refresh): "ART_0_2_REINTERPRETATION" — references Art. 0.2 as authority for tape canonical interpretation. Art. 0.2 in constitution explicitly states the tape canonical axiom. Citation valid ✅
- No commit found using C-xxx citation where the case ruling contradicts the claim.

---

## Step 5 — Summary

### Status: YELLOW

**YELLOW reason 1 (ACTIVE_USE_GAP)**: Art. 0.x articles (all five: 0, 0.1, 0.2, 0.3, 0.4) added 2026-04-26 have ZERO case coverage. They are being actively cited in critical architectural decisions (D-VETO-6 in LATEST.md). No common-law precedent exists. Cases should be written before Art. 0.x is used as sole VETO authority.

**YELLOW reason 2 (C-027 hardcodes × 2)**:
- `evaluator.rs:405`: `temperature: Some(0.2)` in oneshot path — no env override
- `evaluator.rs:573,605` + `bus.rs:29`: `system_lp_amount: 200.0` — no `SYSTEM_LP_AMOUNT` env override

**YELLOW reason 3 (phantom Art. 0.5)**: Non-existent article referenced in LATEST.md pending-actions table without clear "DRAFT" marker in the table cell itself.

### What is GREEN (positive confirmations)

| Check | Result |
|-------|--------|
| forbidden_patterns present + native_decide confirmed | ✅ PASS |
| omega/decide NOT in agent-facing prompts | ✅ CLEAN |
| n3 NOT mislabeled as hybrid | ✅ CLEAN |
| Boltzmann routing ACTIVE | ✅ CLEAN |
| Prediction markets ACTIVE | ✅ CLEAN |
| No citation-vs-precedent mismatch in recent commits | ✅ CLEAN |
| constitution.md exists and readable | ✅ CLEAN |
| cases/ directory exists (48 files) | ✅ CLEAN |
| git repo intact | ✅ CLEAN |

### Low-Priority Informational Notes

- Art. I.1.1: only 2 cases (thin PCP predicate coverage)
- Art. V.3: only 1 case (C-071) — amendment process barely precedented
- Art. III.1: only 1 case (C-022) — error-shielding precedent thin

---

*Advisory report only. Auditor has no VETO authority. All findings are advisory per Art. V.1.3.*
*Auditor role: JudgeAI-advisory (clean remote session, zero local context bias)*
