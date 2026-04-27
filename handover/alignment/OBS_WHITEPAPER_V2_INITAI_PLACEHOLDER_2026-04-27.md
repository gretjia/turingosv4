# OBS — InitAI Conceptual Placeholder (Whitepaper v2 § 11.2)

**Date**: 2026-04-27
**Type**: alignment observation
**Source**: v2 whitepaper § 11.2

---

## Observation

v2 § 11.2 names **InitAI** in the boot sequence:

```
human architect provides spec
InitAI compiles spec into predicates and tools
Judge checks constitutional constraints
system creates Q_0
on_init mints initial Coin state
ledger writes genesis block
runtime enters loop
```

InitAI is positioned as the boot-time compiler that turns human spec into machine-executable predicates and tools.

---

## Status: conceptual placeholder

InitAI is **not currently a system component**. It does not exist in:
- `src/`
- Plan v3.2 atom list
- TRACE_MATRIX FC node IDs
- any current tooling

Equivalent functions today are split across:
- `src/boot.rs::verify_constitution_root_section` — checks constitution hash + signature
- `genesis_payload.toml` — holds boot-time inputs
- `src/top_white/predicates/registry::register` + `src/bottom_white/tools/registry::register` — manual predicate / tool registration (no LLM-driven compilation)

---

## Interpretation choice (forward-looking)

There are three plausible interpretations of InitAI's eventual form:

### A. Pure conceptual / never implemented as an LLM
InitAI is just shorthand for "the boot-time compilation step." It can remain a manual / scripted process forever. v2 § 11.2 is descriptive of role, not prescriptive of agent.

### B. ArchitectAI's boot-time mode
The same ArchitectAI persona (per Art. V.1) that proposes meta-changes during runtime also handles boot-time spec compilation. No new system component; just a mode of an existing one.

### C. Full new component
InitAI is a separate LLM-backed agent with its own protocol, audited separately, registered as a system role. Would require a new atom in Plan v3.2 (e.g., CO1.0.x InitAI runtime) + dual external audit.

**Default interpretation**: A or B (no new component required). Promote to C only if:
- Boot-time spec compilation becomes too complex for manual / scripted handling, AND
- A future v2.x whitepaper revision endorses InitAI as a distinct agent

---

## Why this matters now

If a reader of v2 assumes interpretation C, they may file an unnecessary atom. If a reader assumes interpretation A, they may not notice that boot-time spec compilation is currently entirely manual (which is fine for current scale, but a scaling risk).

The OBS file makes the ambiguity explicit so future planning rounds choose deliberately.

---

## Open questions

1. Which interpretation does the v2 author prefer? (Default: A.)
2. If C is ever chosen, how does InitAI relate to ArchitectAI's three-way separation (Constitution / ArchitectAI / JudgeAI / Human)?
3. Should genesis_payload.toml grow a `compiled_by` provenance field?

---

## Status

**Captured. No action.** Default interpretation: A (conceptual placeholder, no system component). Promote to atomic plan only on explicit user instruction or v2.x revision.

— ArchitectAI, 2026-04-27
