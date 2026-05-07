# OBS — TB-Wave 1/2 AMBER Closure Forward Binding (2026-05-07)

**Status**: FORWARD-BOUND (not closed this session).
**Source**: Stage A2 of `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md`
§3.1 + LATEST.md session #17 next-step #3 + Gemini Q7 PASS confirming "Wave 1/2 deferral
defensible" (`handover/audits/GEMINI_CONSTITUTION_LANDING_FIRST_SANITY_2026-05-07_R1.md`).

**Decision**: Wave 1/2 AMBER closure deferred to a forward TB charter; not blocker for
TB-18R FINAL ship (already executed this session) or TB-18B / TB-18C / TB-18D charter
landing. Binding made explicit so the work isn't silently retired.

---

## §1. Items deferred

Per session #17 next-step #3 + matrix scan:

| # | Item | Currently | Target | Class | Notes |
|---|------|-----------|--------|-------|-------|
| 1 | Wilson 95% CI helper in `src/` (closes Art. I.2 PPUT report discipline AMBER) | report-side discipline only (`CLAUDE.md §17`); no helper | additive helper module + matrix promotion | 1 | TB-18B atom R2 covers this (already in TB-18B charter §5 atom R2); can move to TB-Wave12 if TB-18B doesn't ship soon |
| 2 | `parent_selection_entropy` + `pairwise_payload_diversity_mean` in WAVE3_AGGREGATE.json shape (closes Art. II.2.1) | `tests/six_axioms_alignment.rs::axiom_2_payload_diversity` GREEN structurally; aggregate shape lacks fields | aggregate runner emits both fields; matrix row promotes | 1 | TB-18B atom R3 covers this |
| 3 | Goodhart selector-blindness gate (strengthens Art. III.4 shielding from structural-only) | `tests/constitution_shielding_gate.rs::l4e_public_summary_low_pollution` AMBER (structural-only) | fixture-style test that builds a real `UniverseSnapshot` and asserts selector cannot read Lean stderr text body | 1 | independent of TB-18B; could be part of TB-Wave12 |
| 4 | Agent-prompt-no-raw-stderr fixture-style gate (strengthens Art. II.1 + Art. III.1 from AMBER) | `raw_lean_stderr_not_in_agent_read_view` + `private_diagnostic_cid_not_serialized_publicly` (latter GREEN; former AMBER) | fixture-style test that renders the actual agent prompt and grep-asserts no raw stderr | 1 | independent of TB-18B; could be part of TB-Wave12 |

## §2. Why deferred (not closed this session)

1. **Class-1 work scope**: The four items are Class-1 (additive tests + helper module).
   None are STEP_B-restricted. But promoting matrix rows from AMBER → GREEN requires
   that the new tests genuinely exercise the production path under CR-C0.7 ("test
   exercises real path AND passes"). Writing fixture-style tests that drive
   `UniverseSnapshot` / `Prompt` builders correctly requires reading the surrounding
   src/ files in detail; doing this in the same session as the TB-18R FINAL ship +
   four forward charters would risk green-baseline breakage.

2. **Gemini Q7 PASS validates the deferral**:
   > "Deferring Wave 1/2 cleanup is a defensible 'strict-alignment win,' not covert
   > sequencing manipulation. The development effort correctly prioritized the
   > load-bearing blockers identified in PROJECT_PLAN §3 as formal 'resume conditions.'
   > Tackling these hard dependencies first to un-gate the project is a sign of
   > rational, goal-oriented execution."

3. **Architect alignment doc Stage A2 explicitly marks these "independently valuable;
   not §3 blocker"** — the path is to TB-18B / TB-18D, not to gate them on Wave 1/2
   completion.

4. **No regression risk**: matrix RED rows = 0; AMBER rows are not blocking any
   currently-shipping work. Forward-binding preserves the work item without forcing it
   into the same session as ship + charter drafts.

## §3. Forward TB candidates

Two reasonable homes for these items:

### §3.1. TB-18B (preferred for items 1 + 2)

TB-18B charter §5 atom R2 + R3 already enumerate items 1 + 2 as required deliverables
for the M1/M2 benchmark report shape. So those will close as part of TB-18B execution.

### §3.2. TB-Wave12 (proposed; for items 3 + 4)

Items 3 + 4 don't naturally belong in TB-18B (they're independent harness hardening,
not benchmark-shape items). Proposed forward TB: `TB-Wave12_charter_YYYY-MM-DD.md` —
Class-1 additive test pass with the four fixture-style tests, scoped specifically to
matrix AMBER → GREEN promotion of:

- Art. I.2 PPUT report discipline (item 1, if not closed by TB-18B)
- Art. II.2.1 exploration/exploitation (item 2, if not closed by TB-18B)
- Art. III.1 shield errors (item 4)
- Art. III.4 shield Goodhart (item 3)
- Art. III.2 / III.3 secondary shielding rows that may benefit from same fixture style

Charter length estimate: 1-3 days Class-1 work per session #17 estimate.

## §4. Action this session

| Item | Status |
|------|--------|
| Bind items 1-4 as forward TB scope | DONE — this OBS |
| Cite this OBS from TB-18B charter §5 atom R2/R3 | DONE — TB-18B already lists them as charter atoms |
| Cite this OBS from session #18 LATEST.md | TODO — session-end update |
| Open TB-Wave12 charter | DEFERRED — when TB-18B ships and items 1+2 close, decide whether 3+4 still need a separate TB or can fold into TB-18B follow-up |

## §5. Cross-references

- Stage A2 source: `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md` §3.1
- Matrix AMBER survey: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` (rows §A Art.0.x + §B Art.I.2 + §C + §D + §E + §F + §I + §K)
- Gemini Q7 PASS: `handover/audits/GEMINI_CONSTITUTION_LANDING_FIRST_SANITY_2026-05-07_R1.md`
- Session #17 next-step #3: `handover/ai-direct/LATEST.md` (session #17 entry)
- TB-18B charter (homes items 1+2): `handover/tracer_bullets/TB-18B_charter_2026-05-07.md`
- TB-Wave12 charter (proposed): not yet written
- Constitution gap analysis (broader AMBER survey): `handover/alignment/CONSTITUTION_GAP_ANALYSIS_2026-05-07.md`
