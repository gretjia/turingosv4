---
name: constitution-landing-check
description: Pre-action gate before drafting any TB charter, dispatching any G1 audit, or planning "next atom" sequence. Surfaces AMBER rows in CONSTITUTION_EXECUTION_MATRIX.md and forces explicit acknowledgment that AMBER row work supersedes forward charter/audit work. Catches mode regression to Atomic Agentic Engineering.
user_invocable: true
---

# /constitution-landing-check — Pre-Charter / Pre-Audit Gate

## When to fire (mandatory)

Before ANY of:
- Drafting a new TB charter (`handover/tracer_bullets/TB-*_charter_*.md`)
- Drafting a stage charter (`handover/tracer_bullets/STAGE_*_charter_*.md`)
- Dispatching G1 charter ratification audit (Codex / Gemini before execution)
- Planning a "pick highest-ROI atom" / "next execution atom" sequence
- Processing G1 verdict → charter v2 amendment log
- Treating any session task as `blockedBy <audit-task>`

These are the patterns enumerated in `feedback_constitutional_harness_engineering` Anti-patterns §1-6.

## Why this gate exists

Per CLAUDE.md §2.1 Constitutional Harness Engineering required loop:
```
constitution gate → real run → debug → fix → rerun → audit → ship
```

Forbidden old loop (Atomic Agentic Engineering):
```
charter → atom → self-audit → external audit → more docs → delayed test
```

Session #19 root-cause incident (2026-05-07): three Codex G1 charter audits dispatched in parallel before any harness test ran, with task graph `pick atom blockedBy G1`. Verbatim user correction: "我不喜欢这个工作逻辑，我要的是宪法的完整落地，而且我们的harness已经从atomic agentic engineering转变为constitutional engineering". This skill is the mechanism (per `feedback_norm_needs_mechanism`) preventing recurrence.

## Stages

### 1. Surface AMBER row count

```bash
grep -cE '🟡 AMBER' /home/zephryj/projects/turingosv4/handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md
```

Expected output: integer N. If N > 0, AMBER rows exist; charter / audit work is mode regression unless Stage 4 explicitly justifies it.

### 2. Enumerate AMBER rows by Article

```bash
awk '
  /^## §[A-Z]/ { section = $0; next }
  /🟡 AMBER/ {
    if (count[section]++ == 0) print "\n--- " section " ---"
    # First field is the clause/test name in the | clause | column
    split($0, cols, "|")
    print "  - " cols[2]
  }
' /home/zephryj/projects/turingosv4/handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md
```

Output is the AMBER row inventory. These are the candidate work items for THIS session.

### 3. Classify each AMBER row by addressability

For each AMBER row, decide one of:
- **chain-resident, mechanically-closeable** — has test, needs real-tape evidence under load → land NOW
- **structural-only by design** — kill condition is procedural / can't be chain-witnessed (typical for FC3 meta nodes) → flag as "structural cap" not work item
- **authority-bound** — needs architect / human signature (Art. V.* surfaces) → forward-bind, not session work
- **forward-TB scope** — depends on Class-4 STEP_B substrate that doesn't exist yet → forward-bind to TB charter, not session work

Row goes to session work queue ONLY if classified as `chain-resident, mechanically-closeable`.

### 4. If user / session prompt insists on charter / audit work

Charter / G1 audit work is justified ONLY when:
- (a) all AMBER rows are classified as `structural-only`, `authority-bound`, or `forward-TB scope` AND
- (b) the charter / audit is for a forward TB whose first executable atom requires the substrate the charter describes AND
- (c) the audit is G2 (AFTER-evidence), not G1 (BEFORE-evidence)

If any of (a)/(b)/(c) fails: STOP. Pivot to AMBER row work. Charter / G1 audit is mode regression.

### 5. Cross-check `feedback_constitutional_harness_engineering` Anti-patterns §1-6

If session work matches any anti-pattern: emit "MODE REGRESSION DETECTED — pivot required" and refuse to proceed with charter / audit work.

## Output format

```
=== CONSTITUTION LANDING CHECK ===
1. AMBER row count:        [N]
2. AMBER inventory:        [list by Article]
3. Mechanically-closeable: [list of row IDs]
4. Charter/audit justified: [yes — reason | no — pivot required]
5. Anti-pattern match:     [none | §1-6 matched: <pattern>]
VERDICT: [PROCEED with charter/audit | PIVOT to AMBER row work]
Recommended first row: [row ID + 1-line work plan]
```

## On VERDICT = PIVOT

- Do NOT proceed with charter draft / G1 audit dispatch / atom planning.
- Pick the first `mechanically-closeable` AMBER row from Stage 3 list.
- For each AMBER row: write executable harness test → run real evidence → debug → flip to GREEN.
- Update `CONSTITUTION_EXECUTION_MATRIX.md` row status with evidence binding.

## Linked rules
- `feedback_constitutional_harness_engineering` — parent rule + anti-pattern enumeration
- `feedback_norm_needs_mechanism` — why this skill exists (meta-rule)
- `feedback_audit_after_evidence` — G2 not G1
- `feedback_tape_first_real_tests` — no tape, no test
- `feedback_real_problems_not_designed` — every clause needs real-problem witness
- CLAUDE.md §2 PRIME OPERATING MODE
- CLAUDE.md §7 Constitution Landing Policy (matrix discipline)
- CLAUDE.md §19 No Manipulation by Sequencing (don't close easy gaps to claim progress while load-bearing AMBER remains)
