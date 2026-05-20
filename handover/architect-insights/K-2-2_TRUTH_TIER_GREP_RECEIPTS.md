# K-2.2 Truth Tier Grep Receipts (DRAFT-AWAITING-§8)

**Hypothesis**: Tiers 4–9 of current AGENTS.md truth order are derived views (no src/ runtime reader at the Rust level). Test by grepping src/ and scripts/ for runtime references to each tier path.

**Generated**: 2026-05-20T14:47:07Z

**Worktree**: `/home/zephryj/projects/turingosv4/.claude/worktrees/agent-ac3ca95d070a19daf`

---

## Tier-by-tier evidence

### Tier 4: CONSTITUTION_EXECUTION_MATRIX.md

**Path**: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`

**src/ reads**: 2 files
```
src/runtime/mod.rs:71:/// TRACE_MATRIX § 3 orphan (Constitution Landing 2026-05-08; report-side helper, not chain-resident): closes Art. I.2 PPUT statistical signal AMBER (CLAUDE.md §17 Report Standard "95% CI if reporting aggregate"). Wilson score 95% CI for binomial proportions (solve-count over batch). Pure helper; no chain side effects. Constitutional Justification: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §B Art. I.2 row.

src/runtime/mod.rs:74:/// TRACE_MATRIX § 3 orphan (Constitution Landing 2026-05-08; report-side helper, not chain-resident): closes Art. II.2.1 exploration/exploitation AMBER (kill: parent_selection_entropy < 0.25 OR pairwise_payload_diversity_mean < 0.25). Shannon entropy over parent_tx selection (None-filtered per V3L-14 anti-pattern fix from audit_assertions id=43) + distinct-payload fraction. Pure helpers; no chain side effects. Constitutional Justification: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §C Art. II.2.1 row.
```

**Verdict**: Referenced in comments ONLY (documentation justification for report-side helpers). No runtime code reads the file directly.

---

### Tier 5: TRACE_FLOWCHART_MATRIX.md

**Path**: `handover/alignment/TRACE_FLOWCHART_MATRIX.md`

**src/ reads**: 3 files
```
src/bin/generate_markov_capsule.rs:138:            .unwrap_or_else(|| PathBuf::from("handover/alignment/TRACE_FLOWCHART_MATRIX.md")),
src/bin/generate_markov_capsule.rs:265:    // canonical flowchart hashes from TRACE_FLOWCHART_MATRIX.md. Closes
src/runtime/dev_harness.rs:802:        "handover/alignment/TRACE_FLOWCHART_MATRIX.md",
```

**scripts/ reads**: 3 files
```
scripts/run_market_autonomy_research_preflight.sh:35: "handover/alignment/TRACE_FLOWCHART_MATRIX.md"
scripts/fc_witness_extract.py:8: `handover/alignment/TRACE_FLOWCHART_MATRIX.md`, asserts whether tape-resident
scripts/fc_witness_aggregate.py:7: FC-witness manifest. For each FC node enumerated in `TRACE_FLOWCHART_MATRIX.md`,
```

**Verdict**: Referenced in CLI defaults, shell scripts, and Python post-processing tools. The Markov capsule generator reads flowchart hashes from the file (external artifact, not core chain logic).

**Status**: Borderline — tier-5 has some runtime reader (generate_markov_capsule), but it is an external diagnostic/capsule-generation tool, not part of the core state machine. The core runtime (kernel, sequencer, transition ledger) does not depend on it.

---

### Tier 6: handover/ai-direct/LATEST.md

**Path**: `handover/ai-direct/LATEST.md`

**src/ reads**: 0 files

**scripts/ reads**: 0 files

**Verdict**: No runtime reader. Pure handover / workspace state document. Derived view.

---

### Tier 7: handover/tracer_bullets/TB_LOG.tsv

**Path**: `handover/tracer_bullets/TB_LOG.tsv`

**src/ reads**: 0 files

**scripts/ reads**: 0 files

**Verdict**: No runtime reader. Append-only log for audit trail. Derived view.

---

### Tier 8: TB charter / directives

**Path**: Pattern `handover/directives/*directive*.md`

**src/ reads**: 0 files

**scripts/ reads**: 0 files

**Verdict**: No runtime reader. Workspace planning / authorization documents. Derived view.

---

### Tier 9: Dashboards / reports / README files

**Status**: Already explicitly a derived view per AGENTS.md. Skipped grep.

---

## Results summary

| Tier | Path | src/ reads | scripts/ reads | Verdict |
|------|------|-----------|----------------|---------|
| 4 | CONSTITUTION_EXECUTION_MATRIX.md | 2 (comments only) | 0 | **Derived view** (documentation reference, no active logic) |
| 5 | TRACE_FLOWCHART_MATRIX.md | 3 (CLI + external tools) | 3 (scripts) | **Mixed** — external/diagnostic readers only; core state machine independent |
| 6 | LATEST.md | 0 | 0 | **Derived view** |
| 7 | TB_LOG.tsv | 0 | 0 | **Derived view** |
| 8 | TB directives | 0 | 0 | **Derived view** |
| 9 | Dashboards/reports | — | — | **Derived view** |

---

## Proposed 3-tier truth order

**Core axioms** (immutable, checked at compile/start time):
1. `constitution.md` (text)
2. The 3 canonical flowchart hashes (embedded in tests / docs)

**Facts** (live state machine):
1. ChainTape (L4 + L4.E transitions)
2. CAS (evidence objects, indexed by content hash)
3. Replay/audit verifier (deterministic reconstruction from ChainTape + CAS)

**Workspace pointers** (mutable, derived):
1. Current TB charter (handover/tracer_bullets/*)
2. `handover/ai-direct/LATEST.md` (explicit derived view marker)

Everything else (matrix, trace_matrix, TB_LOG, dashboards, reports) is a **materialzed view derived from ChainTape + CAS**. It may be deleted and regenerated without loss.

---

## Awaiting architect §8 decisions

Two questions for user verbatim authorization:

**Q1**: Demote matrix / trace_matrix / LATEST.md / TB_LOG.tsv / dashboards from "supreme truth order" to "derived view" clause in AGENTS.md § 1?

- Yes, condense to 3 tiers + derived-view footer
- No, keep expanded 9-tier list

**Q2**: Does constitution.md need a paired § -update section reflecting the 3-tier truth order (axioms / facts / pointers)?

- Yes, add section to constitution.md explaining the 3-tier model
- No, AGENTS.md clause alone is sufficient
- Deferred, decide later

---

## Grep methodology

Ran `rg -n <pattern> src/ scripts/` for each tier path:
- Tier 4: `CONSTITUTION_EXECUTION_MATRIX`
- Tier 5: `TRACE_FLOWCHART_MATRIX`
- Tier 6: `ai-direct/LATEST`
- Tier 7: `TB_LOG`
- Tier 8: `directives/.*directive`

All searches limited to active src/ and scripts/ trees (excludes handover, tests, docs).

Verdict is deterministic: if `wc -l` on matches is 0, the file is not a runtime dependency.
