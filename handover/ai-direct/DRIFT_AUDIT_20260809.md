# Constitutional Drift Audit — 2026-08-09

**Status: YELLOW**
**Auditor role**: JudgeAI-advisory (Art. V.1.3) — advisory only, cannot block work
**Session**: Clean remote session, no implementation transcript context

---

## Step 1 — Tooling Self-Check

**Outcome: PASS (positive verification)**

- `forbidden_patterns` field located: `src/bus.rs:40` in `BusConfig`
- `forbidden_patterns_v1` predicate boot spec located: `src/top_white/predicates/registry.rs:631-640`
- **`native_decide` CONFIRMED PRESENT** in `forbidden_patterns_v1` literal list (line 637)
- Full list: `["native_decide", "unsafe", "axiom "]`
- Bus enforcement logic confirmed at `src/bus.rs:207` (Phase 0 check, skipped for oracle-accepted payloads)
- `KERNEL_BYPASS_TOKENS` in `src/judges/lean_judge.rs:61`: `["sorry", "admit", "native_decide", "unsafe"]` — also contains `native_decide`

Tooling is not blind. Auditor can proceed with confidence.

---

## Step 2 — Constitution + Cases Coverage Map

**Constitution Art. IDs found** (26 total, including Art. 0.x):

| Article | Case Files |
|---------|-----------|
| Art. 0  | **0** |
| Art. 0.1 | **0** |
| Art. 0.2 | **0** |
| Art. 0.3 | **0** |
| Art. 0.4 | **0** |
| Art. I   | 39 |
| Art. I.1 | 13 |
| Art. I.1.1 | 4 |
| Art. I.2 | 7 |
| Art. II  | 19 |
| Art. II.1 | 5 |
| Art. II.2 | 9 |
| Art. II.2.1 | 6 |
| Art. III | 9 |
| Art. III.1 | 3 |
| Art. III.2 | 4 |
| Art. III.3 | 4 |
| Art. III.4 | 4 |
| Art. IV  | 12 |
| Art. V   | 21 |
| Art. V.1 | 15 |
| Art. V.1.1 | 5 |
| Art. V.1.2 | 5 |
| Art. V.1.3 | 6 |
| Art. V.2 | 6 |
| Art. V.3 | 1 |

---

## Step 3 — Active-Use Coverage (ACTIVE_USE_GAP)

**ACTIVE_USE_GAP: Art. 0, Art. 0.1, Art. 0.2, Art. 0.3, Art. 0.4**

These five articles have **zero case files** yet are heavily cited in active work:
- TB_LOG.tsv contains **21 rows** citing `Art. 0.x`
- LATEST.md cites `Art. 0.4` twice (TDMA substrate; CAS-GIT-REPAIR)
- Recent shipped TBs cite `Art. 0.2` (CAS canonical tape) and `Art. 0.4` (Path A/B version-control substrate)
- The entire CAS-GIT-REPAIR tracer bullet series is anchored to Art. 0.2 and Art. 0.4

Art. 0.x covers:
- Art. 0: Turing machine axioms (paper/pencil/rubber/discipline)
- Art. 0.1: Four-element mapping
- Art. 0.2: Tape Canonical axiom (all signals reconstructable from tape)
- Art. 0.3: Blockchain reserve
- Art. 0.4: Q_t is version-controlled state (HEAD_t/tape_t three-tuple)

**There are no case files that codify rulings, incidents, or precedents for the entire Art. 0 chapter.** Any constitutional engineer who consults only cases/ for Art. 0.x guidance finds nothing.

Low-priority: Art. V.3 has only 1 case file (C-071, constitution amendment process). This is used less often in active plans so is informational, not a gap.

---

## Step 4 — Drift Scan

### 4.1 omega/decide in agent-facing prompts — YELLOW finding

**File**: `src/sdk/prompt.rs:358` and `:388`

**Evidence**:
```
arithmetic decision: omega / linarith / nlinarith / polyrith
simplification:      simp / aesop / decide
```

These strings appear in agent-facing retry prompt text (injected when a tactic was rejected). The prompt is active production code, not a comment.

**Cross-check against C-011**:
- C-011 precedent (rule R-004, 2026-04-06): *"decide/omega/native_decide 应在 bus.rs forbidden_patterns 中拦截"*
- `src/bus.rs` comment (line 183): *"the math-domain checker driver supplies the concrete forbidden-token catalog; in the Lean driver that is e.g. bare `decide` / `omega` / `native_decide`"*
- **Actual enforcement**: `omega` is in neither `KERNEL_BYPASS_TOKENS` nor `forbidden_patterns_v1` nor `BusConfig.forbidden_patterns` (which defaults to empty)

**Interpretation**: The prompt actively suggests `omega` while C-011 precedent says it should be blocked. This may be an undocumented intentional post-C-011 design decision — `omega` for linear arithmetic is a constructive solver, not brute-force enumeration; the C-011 case was specifically about `Finset.range 100 ... decide` exhaustive search. The H-HET-2 work (`b180971`) explicitly uses "reached omega" as a positive verification milestone. If omega were blocked, this milestone could not be demonstrated.

**Advisory note**: Either (a) C-011 precedent item 3 should be narrowed to `native_decide` and `decide`-on-finite-set patterns only, or (b) the policy to block `omega` should be implemented and the prompt updated. The current state is underdocumented. Not flagging RED because no clear constitutional text forbids omega; the C-011 case's ruling is about brute-force enumeration, not arithmetic decision procedures.

### 4.2 hybrid/n3 labeling — NO VIOLATION

`tb_n3` references in source code are technical identifiers for "Tracer Bullet N3" experiment artifacts:
- `src/runtime/adapter.rs:1127`: `tb_n3_emit_node_market_after_work_accept` — function naming an adapter for a specific TB
- `src/runtime/market_decision_trace.rs:222`: schema version `tb_n3.market_decision_trace.v1` — a versioned schema identifier

These are artifact/schema labels, not causal attribution claims. No evidence of hybrid oracle mode being labeled as a different mode (C-032) or causal wins being attributed to n3-routed results without causal chain (C-033).

### 4.3 hardcoded thresholds — INFORMATIONAL

Commit `61ec26c` (2026-06-02) already surfaced and documented C8 [MEDIUM]: G0 "market activation 11/11" mixing real checks with hardcoded literals under one headline. This was explicitly corrected as a CORRECTION banner. No new hardcoded threshold regressions found in the most recent commits (`b180971`, `b16b9de`, `231ec17`).

`src/market_tape_shared.rs:148,154` contains hardcoded token cost tables (e.g., `("Qwen/Qwen3-32B", 140_000, 570_000)`). These are pricing constants, not behavioral thresholds. Informational note; not a C-027 violation unless they affect routing decisions without env-override capability.

### 4.4 Recent commit citation cross-check — NO VIOLATIONS

| Commit | Subject | Art./C citation | Finding |
|--------|---------|-----------------|---------|
| b180971 | H-HET-2 converge | "replay-green tape" (Art. 0.2 implicit) | Legitimate — tape active, cited correctly |
| b16b9de | de-Lean migration (Class 4) | No Art. citation in subject | §8 referenced in commit body, appropriate |
| 231ec17 | autoloop | "reach omega" as positive control | Legitimate verification milestone |
| 61ec26c | U1 CORRECTION banners | No Art. citation | Honest corrective, GOOD |
| 7298b927 | Merge #340 Class-4 §8 | OBS_AGENT_SIG_REPLAY_GAP | §8 documented, trust-root pins rehashed — appropriate |

No citation-vs-precedent mismatches found. No claimed wins with tape/market dormant. No `omega/decide` in unexpected system-instruction positions.

---

## Summary

**Status: YELLOW**

**Why not GREEN**: Active-use gap (Art. 0.x articles with heavy recent citations but zero cases) and one interpretation discrepancy (C-011 and omega) prevent positive-verification of clean coverage.

**Why not RED**: 
- Step 1 POSITIVELY VERIFIED: `native_decide` in `forbidden_patterns_v1` 
- No C-011 structural violation (bus-level forbidden_patterns is the configurable gate; KERNEL_BYPASS_TOKENS blocks native_decide at judge level)
- No C-032/C-033 oracle-mode mislabeling or causal attribution gap
- No new hardcoded threshold regression in recent commits
- All recent commit citation cross-checks clean

**ACTIVE_USE_GAP**: Art. 0, Art. 0.1, Art. 0.2, Art. 0.3, Art. 0.4 — 21 TB_LOG citations in active work, 0 case files. Recommendation (advisory): ArchitectAI should consider whether C-037 (tape WAL) and C-039 (proof artifact persistence) already partially cover Art. 0.2/0.4, and if so, add explicit constitution cross-references. If not, new case files for at least Art. 0.2 and Art. 0.4 would close the interpretive gap.

**Drift finding (C-011 / omega)**: `src/sdk/prompt.rs:358,388` suggests `omega` to agents; C-011 precedent says omega should be blocked; `omega` is absent from all enforcement lists. Advisory recommendation: document in C-011 or a new case whether `omega` is intentionally exempted post-H-HET-2 or whether enforcement should be added.

**Low-priority**: Art. V.3 single case (C-071) — low active-use, not a gap.

---

*Audit generated by clean-context JudgeAI session (Art. V.1.3, advisory). Cannot block work.*
*Evidence references: src/bus.rs, src/top_white/predicates/registry.rs, src/judges/lean_judge.rs, src/sdk/prompt.rs, cases/C-011_brute_force_formalization.yaml, handover/tracer_bullets/TB_LOG.tsv, handover/ai-direct/LATEST.md*
