# OBS — TB-15 dashboard §15 live regeneration deferred to TB-16

**Date**: 2026-05-04 (post Codex R1 dual audit Q9 closure).
**Status**: **CLOSED 2026-05-04** by TB-16 Atom 4 (`build_report` now reconstructs `EconomicState` via `replay_full_transition` + populates `autopsy_event_counts` from `replayed_econ.agent_autopsies_t`; `rebuild_autopsy_event_counts` helper in `src/bin/audit_dashboard.rs`; verified by `tests/tb_16_dashboard_live_regen.rs`).
**Triggered by**: Codex TB-15 R1 ship audit Q9 CHALLENGE
(`handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md`).

## Summary

Codex R1 Q9 CHALLENGE: dashboard §15 (`render_section_15` in `src/bin/audit_dashboard.rs`) is privacy-safe by construction (input signature accepts only `&[(String, u32)]` event counts + `Option<&str>` Markov pointer hex — no raw bytes possible) — BUT the field `autopsy_event_counts: Vec<(String, u32)>` is hard-coded `Vec::new()` in `build_report` because the dashboard's L4 walk does not currently rebuild full `EconomicState` post-replay. The structural privacy fence holds; the regeneration capability is forward-prepared scaffolding.

> **Codex verbatim**: "dashboard §15 is privacy-safe but does not regenerate from ChainTape + CAS; `autopsy_event_counts` is hard-coded empty (`src/bin/audit_dashboard.rs:954`, `handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:124`)."

## Why this is OBS, not blocking

1. **Privacy contract HELD**: render_section_15's input signature (`&[(String, u32)]` + `Option<&str>`) is structurally incapable of leaking raw bytes regardless of where the data comes from. The "hard-coded empty" issue is about COMPLETENESS of regeneration, not SAFETY of regeneration.
2. **Original ship-status doc explicit deferral**: `handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md` lines 124-125 already document this as a known limitation: *"build_report does not currently rebuild full EconomicState from chain (TB-14 dashboard pattern is exposure-row accumulation); for v0 we leave this empty + populated by future TB-16 controlled-arena wiring."*
3. **TB-16 charter scope**: TB-16 (Controlled Market Smoke Arena per architect §7) is an end-to-end integration smoke that produces a multi-tx ChainTape including TaskBankruptcyTx → autopsy emission. TB-16's audit-from-tape work (per `handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md`) requires walking the replayed `EconomicState` to populate `autopsy_event_counts` for SG-15.6 verification at the test boundary. The dashboard rebuild is naturally that work.
4. **Per `feedback_audit_obs_bias`**: cheap fixes get fixed; multi-hour future-arch OBS-deferred. Implementing the live rebuild requires a moderate refactor of `build_report` (add an EconomicState reconstruction pass; ~1-2 hr of careful work) and is fundamentally TB-16 scope (the live walk depends on TB-16's chain artifacts to be useful).

## Closure plan (TB-16 prerequisite)

When TB-16 implements the audit-from-tape harness:

1. Extend `build_report` in `src/bin/audit_dashboard.rs` with an EconomicState reconstruction step (re-use the chain replay path or call into `replay_full_transition`).
2. Walk `replayed_econ.agent_autopsies_t.0` and populate `autopsy_event_counts: Vec<(String, u32)>` as `(event_id_string, cid_count)`.
3. Add a TB-16 integration test that produces a chain with TaskBankruptcyTx → AgentAutopsyCapsule emission, then asserts dashboard §15 renders the expected `autopsy_event_counts`.
4. Close this OBS by reference in TB-16 ship status.

## Cross-references

- Codex R1 audit Q9: `handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md` line 7892
- TB-15 R1→R2 closure: `handover/audits/RECURSIVE_AUDIT_TB_15_2026-05-04.md`
- TB-15 ship status: `handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md` §"Open follow-ups"
- TB-16 design: `handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md`
- Memory: `feedback_audit_obs_bias` (cheap fixes get fixed; multi-hour future-arch OBS-deferred)
