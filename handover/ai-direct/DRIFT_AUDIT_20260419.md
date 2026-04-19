# Drift Audit — 2026-04-19

**Auditor**: JudgeAI-advisory (Art. V.1.3)
**Role**: Advisory only — cannot block work, surfaces findings for human review
**Status**: YELLOW
**Prior audit**: DRIFT_AUDIT_20260415.md (YELLOW) — 4-day delta

---

## STEP 1 — TOOLING SELF-CHECK

**Result: PASS (positive verification)**

- `BusConfig.forbidden_patterns: Vec<String>` confirmed at `src/bus.rs:21`
- Literal list constructed in `experiments/minif2f_v4/src/bin/evaluator.rs:221-225` (run_swarm):
  ```rust
  forbidden_patterns: vec![
      "native_decide".into(), "decide".into(), "omega".into(),
      "#eval".into(), "IO.Process".into(),
      "IO.FS".into(), "run_tac".into(), "unsafe".into(),
  ],
  ```
- **`"native_decide"` PRESENT** — required entry confirmed ✅
- **Improvement noted**: list grew from 6 entries (2026-04-15 audit) to 8 entries. `"decide"` and `"omega"` are new additions since last audit — constitutional strengthening, not drift.
- Tooling is NOT blind. Auditor can positively verify forbidden_patterns.

---

## STEP 2 — CONSTITUTION + CASES COVERAGE MAP

### Art. IDs extracted from constitution.md (20 total)

| Article | Title (abbr.) | Cases (count) | Status |
|---------|--------------|---------------|--------|
| Art. I | 信号量化 | 0 direct | via sub-articles — acceptable |
| Art. I.1 | 布尔信号 | 9 | COVERED |
| Art. I.1.1 | PCP 谓词 | 3 | COVERED |
| Art. I.2 | 统计信号 | 3 | COVERED |
| Art. II | 选择性广播 | 0 direct | via sub-articles — acceptable |
| Art. II.1 | 广播典型错误 | 5 | COVERED |
| Art. II.2 | 广播价格信号 | 3 | COVERED |
| Art. II.2.1 | 探索与利用 | 5 | COVERED |
| Art. III | 选择性屏蔽 | 0 direct | via sub-articles — acceptable |
| Art. III.1 | 屏蔽错误 | 2 | COVERED |
| Art. III.2 | 封装细节 | 3 | COVERED |
| Art. III.3 | 屏蔽相关性 | 4 | COVERED |
| Art. III.4 | 屏蔽 Goodhart | 4 | COVERED |
| Art. IV | Boot | 10 | COVERED |
| Art. V | Go Meta | 4 | COVERED (C-034 etc.) |
| Art. V.1 | 三权分立 | 6 | COVERED (C-023 etc.) |
| **Art. V.1.1** | **宪法 = 唯一 Ground Truth** | **0** | **ZERO COVERAGE — ACTIVE_USE_GAP** |
| **Art. V.1.2** | **ArchitectAI = 提出者** | **0** | **ZERO COVERAGE — ACTIVE_USE_GAP** |
| Art. V.1.3 | JudgeAI = 验证者 | 1 | COVERED (thin) |
| Art. V.2 | 宪法界限 | 5 | COVERED |

Coverage source: `cases/C-001` through `C-035` (35 cases total, none added since C-035 dated 2026-04-13).

---

## STEP 3 — ACTIVE-USE COVERAGE

### Handover archive scanned (19 files, 2026-04-14 through 2026-04-17)

Top active citations across all handover plans:

| Article | Citations in Plans | Case Coverage | Gap? |
|---------|-------------------|---------------|------|
| Art. II.1 | 51 | COVERED | No |
| Art. III.3 | 10 | COVERED | No |
| Art. I.2 | 9 | COVERED | No |
| Art. V.2 | 8 | COVERED | No |
| **Art. V.1.2** | **8** | **0 cases** | **ACTIVE_USE_GAP** |
| Art. II.2.1 | 8 | COVERED | No |
| Art. III.2 | 7 | COVERED | No |
| Art. I.1 | 7 | COVERED | No |
| Art. V.1.3 | 6 | COVERED (thin) | No |
| **Art. V.1.1** | **6** | **0 cases** | **ACTIVE_USE_GAP** |
| Art. II.2 | 6 | COVERED | No |

### ACTIVE_USE_GAP (actionable)

1. **Art. V.1.2 — ArchitectAI proposer** (8 active citations, 0 cases):
   No case defines what ArchitectAI is allowed to propose vs. what requires human escalation.
   PLAN_PHASE3_CONSTITUTIONAL_LOOP_2026-04-17.md proposes significant architectural restructuring
   (incremental oracle, unified action model, removal of append/complete distinction).
   Operating ArchitectAI without case precedent on proposer boundaries leaves no reference if
   proposals overreach.

2. **Art. V.1.1 — Constitution as sole Ground Truth** (6 active citations, 0 cases):
   No case documents conflict resolution when a session plan contradicts constitution.md.
   LOW_URGENCY if constitution.md is de facto immutable, but gap persists from 2026-04-15 audit.

**Persistence note**: Both gaps were flagged in DRIFT_AUDIT_20260415.md. Four days and 16+ commits later, they remain unaddressed. The gap is widening as more architecture decisions are made against Art. V.1.2 without precedent.

---

## STEP 4 — DRIFT SCAN

### Commits since 2026-04-15 audit (16 commits reviewed)

```
28fa25d generic nN condition: supports N-scaling (n1,n2,n3,n5,n8,...)
3a24cb9 #16 map-reduce tick (Art. IV: clock→mr→tape statistics every 20 tx)
02d5828 #15 Librarian DNA compression activation (Art. III.2)
9e144aa commit C: Step-B v3 Art. II.1 — end-to-end write-site abstraction shield
689c6aa commit B: Step-B v3 Art. II.1 abstract broadcast (treatment only)
2eb84f9 Phase 1 constitutional落地: invest handler + forbidden + agent roles
a91ccb3 session handover: 12 experiments + Phase 3 constitutional insight
41617fb commit A: provenance stamping + seeded Boltzmann RNG (infra for Step-B A/B)
7a90865 run_interleaved: mandatory smoke probe before 50-problem batch
3e3c498 v3.2 blocker fix: model-aware max_tokens
3b5d3df v3.2 pre-reg fixes per Codex re-audit
513966f v3.2 pre-registration freeze (addressing Codex HOLD)
e58e021 v3.1 M4: experiment results + src fixes + research docs
2255759 frozen_analysis: defensive skip for abort-marker rows
c4ee080 harness: add routines/, analysis/ scaffolding, and plan docs
5fa3803 drift audit: 2026-04-15
```

### Red Flag Checks

**C-011 — omega/decide in agent-facing prompts**: CLEAN ✅
- `src/sdk/prompt.rs` contains zero mentions of "omega", "decide", or "native_decide"
- Agent prompt is state-only: chain, skill, market ticker, recent errors, balance, tools
- `verify_omega` (function name in evaluator) and "OMEGA" log labels never surface to agents
- Confirmed: `build_agent_prompt()` takes `tools_description: &str`; caller passes `"append, complete, invest, search"` — no forbidden patterns in tools description

**C-032/C-033 — hybrid labeled as n3, or dormant-component win claim**: CLEAN ✅
- `hybrid_v1` condition correctly uses `"hybrid_v1"` label (evaluator.rs:107), not `"n3"`
- generic nN (`28fa25d`): `format!("n{}", n_agents)` for swarm conditions — correct labeling
- Tape: active in oracle-cache branch (depth 18.8 per AUTO_RUN_SUMMARY_20260417.md)
- Markets: created per node (bus.rs Phase 4, line 195)
- Boltzmann: active (`boltzmann_select_parent` at evaluator.rs:329)
- AUTO_RUN_SUMMARY claims "n3>n1 signal present (6>5 on N=20)" — labeled "signal", not "win".
  N=20, delta=1 problem: appropriately qualified. C-033 discipline maintained.
- PLAN_PHASE3 explicitly documents the current code violates verify-before-write order (known gap,
  not hidden drift). Self-audit is constitutional.

**C-027 — hardcoded thresholds without env override**: YELLOW ⚠️ (PERSISTS from 2026-04-15)

| Parameter | Location | Env Override? | C-027 Status |
|-----------|----------|---------------|--------------|
| `max_transactions = 200` | evaluator.rs:259 | **NO** | VIOLATION |
| `temperature: Some(0.2)` | evaluator.rs:168, 312 | **NO** | VIOLATION |
| `max_payload_chars: 1200` | evaluator.rs:217 | **NO** | VIOLATION |
| `max_payload_lines: 18` | evaluator.rs:218 | **NO** | VIOLATION |
| `DEFAULT_BOLTZMANN_SEED` | evaluator.rs:28 | YES (`BOLTZMANN_SEED`) ✅ | OK |
| `TICK_INTERVAL` | evaluator.rs:262 | YES (`TICK_INTERVAL`) ✅ | OK |
| `MINIF2F_DIR` | evaluator.rs:72 | YES ✅ | OK |
| `BoltzmannParams` | evaluator.rs:253 | YES (`from_env()`) ✅ | OK |

C-027 precedent #1: "所有影响行为的参数必须可通过环境变量/配置覆盖"
C-027 precedent #2: "Default 值 OK，但不可是 const/hardcode"

`max_transactions=200` is most critical: caps swarm exploration depth, directly affects PPUT comparisons between conditions. Absence of `MAX_TRANSACTIONS` env override means A/B conditions share the same cap but it cannot be tuned without code changes — experimental hygiene risk.

This violation persists unaddressed across 16 commits since last audit.

**Citation cross-check — recent plans vs. cited precedents**:

- PLAN_PHASE3 cites Art. IV mermaid (verify-then-write loop) correctly — the plan is proposing to FIX code to match constitution. Correct use of citation.
- STEPB_ART_II1_V3 cites Art. II.1 for TopKClasses broadcast — correct (Art. II.1: broadcast typical errors to whole group).
- AUTO_RUN_SUMMARY: "Bernoulli excess from -31% to +0.7%" — cites F-2026-04-16-07 (internal finding), no constitutional citation. Correct (this is an empirical result, not constitutional ruling).
- N3_DIAGNOSIS correctly cites C-033 and Art. II.1 for the broadcast-mechanism diagnosis — verified accurate.

No citation-vs-precedent mismatch found.

---

## STEP 5 — SUMMARY

### Status: YELLOW

**Reason for YELLOW (not GREEN):**

1. **Art. V.1.2 and Art. V.1.1 — ACTIVE_USE_GAP, persistent**:
   Both articles have zero case coverage despite 8 and 6 citations respectively in active plans.
   This gap was flagged 2026-04-15 and remains unresolved. PLAN_PHASE3 is the most recent
   ArchitectAI-level proposal — operating without V.1.2 precedent on proposer boundaries.

2. **C-027 — max_transactions=200 still hardcoded**:
   Also persists from 2026-04-15 audit. 4 params lack env override: max_transactions, temperature,
   max_payload_chars (swarm-specific), max_payload_lines (swarm-specific).

**Reason NOT RED:**
- Step 1 POSITIVE verification: `native_decide` confirmed present; forbidden list grew (stronger)
- C-011 clean: zero omega/decide exposure in agent-facing prompts
- C-032/C-033 clean: hybrid labeled correctly; oracle-cache win claim appropriately qualified
- All constitutional citations in recent plans are accurate

---

## ACTIVE_USE_GAP (requires attention)

| Priority | Article | Active Citations | Case Coverage | Advisory |
|----------|---------|-----------------|---------------|---------|
| HIGH | Art. V.1.2 | 8 in plans | 0 | Create at least one foundational case before ArchitectAI outer loop |
| MEDIUM | Art. V.1.1 | 6 in plans | 0 | Document conflict-resolution precedent |

---

## LOW-PRIORITY THEORETICAL GAPS (informational)

- Art. V.1.3 has only 1 case (C-023). JudgeAI role is the auditor's own role — thin coverage, though operational (this audit functions as ongoing precedent).
- Art. III.1 has 2 cases. If garbage-collection / context-pruning activates, more coverage warranted.
- Top-level Art. I, II, III: zero direct cases, covered via sub-articles. Acceptable.
- No cases filed after 2026-04-13 despite significant architectural activity (16 commits, 12 experiments). The Common Law is not keeping pace with system evolution.

---

*Auditor: JudgeAI-advisory (Art. V.1.3) | Clean remote session, no local bias | 2026-04-19*
