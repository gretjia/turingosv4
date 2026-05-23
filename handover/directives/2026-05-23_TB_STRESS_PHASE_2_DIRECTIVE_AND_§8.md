# TB-STRESS-PHASE-2 — Package §8 directive

**Date**: 2026-05-23
**Risk classes covered**: 0 / 1 / 2 (no Class 3, no Class 4)
**Charter**: `handover/tracer_bullets/TB-STRESS-PHASE-2_charter_2026-05-23.md`

## §8 architect ratification

Author/architect (gretjia, 2026-05-23): TB-STRESS-PHASE-2 is authorized as
an evidence-only adversarial battery on top of TB-SOFTWARE-3-0
(merged 2026-05-23) and Phase E Phase 1 (merged 2026-05-23). The 10
stress tests in the charter are authorized at "severe" intensity per
prior user direction ("全 10 项 / 重档"). Real LLM calls authorized
within the budget of ~$18.50 total.

Scope freeze (binding):
- NO edit to src/state/typed_tx.rs, src/state/sequencer.rs, src/bus.rs
- NO edit to src/bottom_white/cas/schema.rs
- NO edit to constitution.md, genesis_payload.toml
- NO new src/runtime/mod.rs export
- NO new CAS ObjectType
- NO new provider abstraction layer

If a stress test surfaces a real production defect, escalate as a
SEPARATE Class 2/3 PR outside this package. Stress tests are observation,
not implementation.

## Class boundaries (this package)

| Artifact | Class |
|----------|-------|
| Charter + this directive | 0 |
| `scripts/stress/*.{py,sh}` runner scripts | 1 (additive, isolated) |
| `handover/evidence/stress_*` directories | 2 (real-run evidence) |
| Aggregate ship report + audit reports | 0 |

Class 2 evidence directories ARE allowed to be written to during the
package — they are the package's output. They MUST NOT mutate any
existing `handover/evidence/*` directory (no retroactive evidence rewrite
per AGENTS.md §8).

## Forbidden actions

- mocking the database, sequencer admission, or canonical signing path
- using `f64`/`f32` in any new analysis code on money paths
- introducing new admission/signing keys
- editing existing capsules to "fix" KILL failures (the KILL is the signal)

## Cumulative audit

Post-execution, dispatch:
- 1 clean-context constitution audit (verdict domain: NO-VIOLATION / VIOLATION-FOUND / RECONSTRUCTION-FAILURE / SECOND-SOURCE-DRIFT)
- 1 clean-context Karpathy audit (verdict domain: PASS / CHALLENGE / VETO)

Both audits write to `handover/audits/STRESS_PHASE_2_VAL_*.md`. Stop hook
condition (per user `/goal`) releases when both audit PRs + the
aggregate ship PR merge to main.
