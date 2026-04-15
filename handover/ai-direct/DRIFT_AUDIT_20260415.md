# Drift Audit — 2026-04-15

**Auditor**: JudgeAI-advisory (Art. V.1.3)
**Role**: Advisory only — cannot block work, surfaces findings for human review
**Status**: YELLOW

---

## STEP 1 — TOOLING SELF-CHECK

**Result: PASS (positive verification)**

- `BusConfig.forbidden_patterns: Vec<String>` confirmed at `src/bus.rs:21`
- Literal list in evaluator at `experiments/minif2f_v4/src/bin/evaluator.rs:175-179`:
  ```
  "native_decide", "#eval", "IO.Process", "IO.FS", "run_tac", "unsafe"
  ```
- **`"native_decide"` PRESENT** ✅ — required entry confirmed
- Secondary confirmation: `lean4_oracle.rs:17` includes same pattern with comment `// bytecode bypass`
- Tooling is NOT blind. Auditor can see forbidden_patterns correctly.

---

## STEP 2 — CONSTITUTION + CASES COVERAGE MAP

### Art. IDs extracted from constitution.md (20 total)

| Article | Title (abbr.) | Case Coverage |
|---------|--------------|---------------|
| Art. I | 信号量化 | via sub-articles |
| Art. I.1 | 布尔信号 | COVERED (≥5 cases: C-012, C-015, C-016, etc.) |
| Art. I.1.1 | PCP 谓词 | COVERED (2 cases) |
| Art. I.2 | 统计信号 | COVERED (2 cases) |
| Art. II | 选择性广播 | via sub-articles |
| Art. II.1 | 广播典型错误 | COVERED (2 cases) |
| Art. II.2 | 广播价格信号 | COVERED (2 cases) |
| Art. II.2.1 | 探索与利用 | COVERED (3 cases) |
| Art. III | 选择性屏蔽 | via sub-articles |
| Art. III.1 | 屏蔽错误 | COVERED (1 case) |
| Art. III.2 | 封装细节 | COVERED (2 cases) |
| Art. III.3 | 屏蔽相关性 | COVERED (2 cases) |
| Art. III.4 | 屏蔽 Goodhart | COVERED (2 cases) |
| Art. IV | Boot | COVERED (4 cases: C-027, C-028, C-030, C-031) |
| Art. V | Go Meta | COVERED (2 general cases: C-034) |
| Art. V.1 | 三权分立 | COVERED (3 cases, incl. C-023) |
| **Art. V.1.1** | **宪法 = 唯一 Ground Truth** | **ZERO COVERAGE ⚠️** |
| **Art. V.1.2** | **ArchitectAI = 提出者** | **ZERO COVERAGE ⚠️** |
| Art. V.1.3 | JudgeAI = 验证者 | COVERED (1 case) |
| Art. V.2 | 宪法界限 | COVERED (3 cases: C-033, C-035) |

---

## STEP 3 — ACTIVE-USE COVERAGE

### Handover files scanned
- `handover/ai-direct/LATEST.md` (2026-04-13) — only file present, no daily plan archive

### Art. IDs actively cited in recent plans (LATEST.md)
- Art. V.1 (三权分立: ArchitectAI → JudgeAI → Codex) — **heavily cited**
- Art. II / Art. III (bus.rs constitutional basis)
- Art. I.1 (布尔谓词, oracle)
- Art. II.2.1 (Boltzmann parent selection)
- Art. I.1.1 (protocol layer)
- Law 1 / Law 2

### ACTIVE_USE_GAP — cited in design, ZERO case coverage

| Article | Active Use in LATEST.md | Case Coverage |
|---------|------------------------|---------------|
| **Art. V.1.2** | "ArchitectAI (Claude Opus)" outer loop is **Next Steps #6** — about to be activated | **0 cases** |
| **Art. V.1.1** | Constitution described as "ground truth" in design rationale | **0 cases** |

**Significance**: Art. V.1.2 is the highest-priority gap. The ArchitectAI outer loop is the immediate next step in LATEST.md. If the proposer role is invoked without any case precedent defining its boundaries, there is no reference point if ArchitectAI overreaches (e.g., proposing changes to src/ without JudgeAI review). The three-power separation only works if all three powers have defined precedent.

---

## STEP 4 — DRIFT SCAN

### Git log reviewed (7 commits total)
```
a935e92 Adopt PPUT as sole optimization metric
0e01cfc Rewrite v4 core from constitution: 3762 lines Rust, 102 tests
56a9698 Update LATEST.md — session handover
df7660d Fix legacy § refs, expand test suite to 48 tests
a76127c Formalize Common Law system: 35 cases, article IDs
932a0fe Remove bible.md — constitution.md is now sole alignment document
967fac1 Initial commit: TuringOS v4 microkernel
```

### Red Flag Checks

**C-011 — omega/decide in agent-facing prompts**: CLEAN ✅
- `native_decide` appears only in `forbidden_patterns` list and oracle test assertions
- `src/sdk/prompt.rs` contains zero mentions of "native_decide", "omega", or "decide"
- Agent prompt template is state-only (balance, chain, market ticker, errors)

**C-032/C-033 — n3 labeled as hybrid with dormant components**: CLEAN ✅
- `"n3"` in evaluator.rs:78 is a run-mode selector, not a causal attribution claim
- In n3 mode: Boltzmann routing IS active (evaluator.rs:238: `boltzmann_select_parent`)
- Prediction markets ARE created per node (bus.rs Phase 4, line 183)
- Tape IS used (evaluator.rs:207-213: `snap.tape.time_arrow()`)
- No architecture-win claim with dormant components found

**C-027 — hardcoded thresholds without env override**: YELLOW ⚠️

| Parameter | Location | Env Override? |
|-----------|----------|---------------|
| `max_transactions = 200` | evaluator.rs:199 | **NO** — local variable, no env var |
| `temperature: Some(0.2)` | evaluator.rs:129, 224 | **NO** |
| `max_tokens: Some(8000)` | evaluator.rs:130 | **NO** |
| `max_tokens: Some(4000)` | evaluator.rs:225 | **NO** |
| `DEFAULT_MINIF2F_DIR` | evaluator.rs:25 | YES (`MINIF2F_DIR` env) ✅ |
| `BoltzmannParams` | evaluator.rs:198 | YES (`BoltzmannParams::from_env()`) ✅ |

C-027 precedent #1: "所有影响行为的参数必须可通过环境变量/配置覆盖"
C-027 precedent #2: "Default 值 OK，但不可是 const/hardcode"

`max_transactions=200` is the most significant: it caps swarm exploration depth and directly affects PPUT. The absence of `MAX_TRANSACTIONS` env override means this cannot be tuned without code changes.

**C-033 — citation vs. precedent cross-check**: CLEAN ✅
- LATEST.md cites "三权分立 (Art. V.1)" for the ArchitectAI → JudgeAI flow — correct
- PPUT adoption cites Art. I.2 (统计信号) — correct (PPUT is a statistical scalar)
- bus.rs cites Art. II/III — correct (serial reactor + forbidden-pattern veto matches those articles)

### Additional Operational Note (not a constitutional violation)
- `DEFAULT_MINIF2F_DIR` at evaluator.rs:25 references `/home/zephryj/projects/turingosv3/...`
  This is a different user's path and will silently fall through to a non-existent directory on
  the current system (`/home/user/`). Override via `MINIF2F_DIR` env is required for any run.
  Not a constitutional violation (env override exists), but operational risk for first run.

---

## STEP 5 — SUMMARY

### Status: YELLOW

**Reason for YELLOW (not GREEN):**
Positive verification of Step 1 passed. No RED flags in drift scan. YELLOW because:
1. Art. V.1.1 and Art. V.1.2 have zero case coverage while Art. V.1.2 (ArchitectAI proposer) is the **immediate next active step** per LATEST.md. Operating an ArchitectAI outer loop without case precedent defining its boundaries is a coverage gap in the most active zone of the system.
2. `max_transactions=200` has no env override — mild C-027 violation in the production evaluator path.

**Reason NOT RED:**
- Step 1 positive verification succeeded — `native_decide` confirmed present
- C-011 clean — no omega/decide in agent prompts
- C-032/C-033 clean — n3 mode has Boltzmann + tape + market all active
- No forbidden_patterns regression

---

## ACTIVE_USE_GAP (requires attention before ArchitectAI outer loop activation)

1. **Art. V.1.2 — ArchitectAI proposer boundaries**: No case defines what ArchitectAI is allowed to propose and what requires human escalation. When the outer loop runs for the first time, there is no case precedent to catch a proposal that violates separation of powers. *Advisory: ArchitectAI (Art. V.1.2) should have at least one foundational case before the outer loop is activated.*

2. **Art. V.1.1 — Constitution as sole Ground Truth**: No case documents what happens when there is conflict between constitution.md and a session plan. *Advisory: Low urgency if constitution.md is treated as immutable, but worth documenting precedent.*

---

## LOW-PRIORITY THEORETICAL GAPS (informational)

- Art. I, Art. II, Art. III (high-level articles): No cases directly cite these top-level articles, only sub-articles. Acceptable — sub-article coverage is sufficient.
- Art. III.1 has only 1 case. If garbage-collection / context-pruning becomes active, a second case would strengthen coverage.
- Handover archive has only 1 file (LATEST.md). No daily plan history means Step 3 active-use analysis was based on a single data point. This will improve as sessions accumulate.

---

*Auditor: JudgeAI-advisory (Art. V.1.3) | Clean session, no local bias | 2026-04-15*
