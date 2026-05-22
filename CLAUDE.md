# TuringOS v4 — CLAUDE.md (Claude Code adapter)

@AGENTS.md

This file is the Claude Code adapter. Cross-agent harness contract lives in
AGENTS.md; this file holds only Claude-Code-specific operating detail. If
duplicated, treat AGENTS.md as the shared router.

## 1. Identity & truth order

See `AGENTS.md §1`. If LATEST.md conflicts with ChainTape evidence, ChainTape
wins. If ChainTape contradicts constitution gates, stop.

## 2. Prime operating mode

Constitutional Harness Engineering (`AGENTS.md §4`). Required loop: constitution
gate → real run → debug → fix → rerun → audit → ship.

## 3. Risk class & STEP_B surfaces

See `AGENTS.md §5–§6`. STEP_B Trust Root files require per-atom architect §8.

## 4. Engineering rules

Required:
- `cargo check`
- `cargo test --workspace --no-fail-fast`
- `bash scripts/run_constitution_gates.sh`

Forbidden:
- `.env` commit
- hardcoded behavior parameter
- `f64` in money path
- memory-only canonical state
- shadow ledger source of truth
- dashboard-only source of truth

Money/economy paths must use integer math only.

Agent read views must be scoped, reconstructable, shielded. Do not expose raw
Lean stderr, raw autopsy, private diagnostics, benchmark leaks in ordinary
agent prompts.

Canonical IDs and shadow IDs must not be mixed.

No workaround closures: do not turn a failing gate into a skip, null pointer,
empty evidence path, or dashboard-only proof.

### Canonical FC1 invariant (preserved for tooling binding)

Externalized attempt count equality (per OBS_TB18R_INV1_NONLLM_TX_2026-05-07
clarification):

```
evaluator_reported_completed_llm_calls
=
  tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err
```

This is the canonical LHS scope. The LHS must NOT use
`evaluator_reported_tx_count` (which inflates with architect-mandated non-LLM
admin scaffold). Each of `step` / `parse_fail` / `llm_err` corresponds to one
`r2_write_attempt_telemetry` call site — one externalized LLM-Lean cycle.

If this equality fails: HALT; do not continue benchmark; do not audit as pass.

## 5. Pre-action skill gates

Before drafting TB charter / dispatching G1 audit: `/constitution-landing-check`

Before drafting a charter that will touch `src/` or `scripts/`: check in-flight
PRs for path overlap (see `AGENTS.md §4.1`). One-liner, no new mechanism.

Before runner script that mutates handover/evidence/: `/runner-preflight`

Before writing new feedback_*.md: ask "what mechanism enforces this?"

After TB SHIPPED FINAL or audit rounds > 3: `/harness-reflect`

Before starting a grill session where answer vagueness or contradiction is
expected: `/grill-recursive`

Before calling `turingos generate` when the artifact is an HTML UI app:
`/spec-html-renderer`

## 6. Audit boundary

See `AGENTS.md §15` for audit class-by-class cadence and verdict domain.

Clean-context Codex audit is NOT a judge. Its legal output space:
`{NO-VIOLATION, VIOLATION-FOUND, RECONSTRUCTION-FAILURE, SECOND-SOURCE-DRIFT}`.
Subjective opinions are out-of-scope.

## 7. Read order for new session

1. `CLAUDE.md` (this file)
2. `AGENTS.md`
3. `constitution.md`
4. `handover/ai-direct/LATEST.md`
5. Key Coding Principles: [KARPATHY_ARCHITECT.md](file:///home/zephryj/projects/turingosv4/skills/KARPATHY_ARCHITECT.md) & [KARPATHY_SIMPLE_CODE.md](file:///home/zephryj/projects/turingosv4/skills/KARPATHY_SIMPLE_CODE.md)
6. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
7. current TB charter / directive
8. Only then supporting docs

## 8. Handover discipline (Claude-specific)

Dynamic state belongs in `handover/ai-direct/LATEST.md` and
`handover/tracer_bullets/TB_LOG.tsv`. CLAUDE.md must not encode current ship
status, gate counts, HEADs, freeze details.

Claude memory files live in
`~/.claude/projects/-home-zephryj-projects-turingosv4/memory/`.
Memory is for recurring rules, surprises, mechanisms. Do not duplicate TB_LOG
facts into memory.

## 9. User context

The user is a solo researcher and vibe coder with limited programming background.

Default user-facing language: Chinese. Technical terms may remain English.

Prioritize:
- clear decisions
- explicit gates
- exact instructions for AI coder
- no fake certainty
- no ceremonial process
- tape-first implementation
- fast real-run feedback

Never hide behind process if the tape is wrong.

## Historical archive

The full 909-line pre-K-2.1 CLAUDE.md is at
`handover/architect-insights/CLAUDE_MD_PRE_K21_ARCHIVE.md`.
