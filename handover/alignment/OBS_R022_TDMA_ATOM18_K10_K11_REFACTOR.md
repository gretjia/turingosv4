# OBS-R022 — TDMA-Bounded-RC1 Atom 18 (K10+K11 refactor)

**Date**: 2026-05-22
**Author**: Claude Opus 4.7 (orchestrator)
**Affected commit**: TB-TDMA-BOUNDED-RC1 Atom 18 — K10+K11 Karpathy-audit refactor
**R-022 finding**: removed TRACE_MATRIX backlinks in `src/bin/turingos/cmd_tdma.rs`

## What R-022 saw

The Atom-18 refactor moves the JudgeDriver trait + 3 driver impls + Probe
struct + helpers (`make_judge_stderr_generic`, `sha256_hex`, `write_jsonl`,
`extract_body`) OUT of `src/bin/turingos/cmd_tdma.rs` and INTO the new
library module `src/tdma_runner.rs`. The pre-commit R-022 hook detects this
as "removed `/// TRACE_MATRIX <FC>:` doc-comments" in `cmd_tdma.rs` because
the backlinked items no longer exist at their original file paths.

## Why this is a move, not a removal

Every removed backlink in `cmd_tdma.rs` has a **direct counterpart** in
`src/tdma_runner.rs` with the same `TRACE_MATRIX FC<id>:` prefix:

| Symbol moved | Old location | New location |
|---|---|---|
| `JudgeDriver` trait | `src/bin/turingos/cmd_tdma.rs:51` | `src/tdma_runner.rs` `AnyJudge` enum (replaces it; sum-type per K10) |
| `NesbittDriver` impl | `src/bin/turingos/cmd_tdma.rs:99` | `src/tdma_runner.rs:74` `AnyJudge::nesbitt()` arm |
| `PutnamA1Driver` impl | `src/bin/turingos/cmd_tdma.rs:169` | `src/tdma_runner.rs:90` `AnyJudge::putnam_a1()` arm |
| `PutnamB3Driver` impl | `src/bin/turingos/cmd_tdma.rs:236` | `src/tdma_runner.rs:108` `AnyJudge::putnam_b3()` arm |
| `Probe` struct | `src/bin/turingos/cmd_tdma.rs:466` | `src/tdma_runner.rs:225` (preserved; new TRACE_MATRIX backlink) |
| `make_judge_stderr_generic` | `src/bin/turingos/cmd_tdma.rs:331` | `src/tdma_runner.rs:285` `make_judge_stderr` (renamed; new TRACE_MATRIX backlink) |

The four new public items in `src/tdma_runner.rs` (the `AnyJudge` sum-type
plus `LlmResponse`, `RunConfig`, `Probe`, `RunSummary`, `run_proof`, etc.)
ALL carry `/// TRACE_MATRIX FC<id>:` backlinks pointing to `FC1a-rtool`,
`FC1a-predicate_pi`, or `FC3-replay` as appropriate.

The R-022 backlink **inventory is preserved** at the workspace level; the
hook just doesn't track cross-file moves.

## Karpathy-audit grounding

Both refactors (K10 sum-type, K11 shared library) were called out by the
post-Atom-17 Karpathy audit (clean-context Sonnet 4.6 review of Atoms
11-17 dispatched after PR #107 merged). The audit's exact wording was:

> K10 — JudgeDriver trait has 3 impls so it clears the "single-impl trait"
> anti-pattern, but the 8-method × 3-impl trait shape costs ~120 LOC of
> mechanical duplication where a sum-type `enum AnyJudge { ... }` would
> express the same heterogeneity in ~40 LOC.
>
> K11 — Three near-clone runner binaries (`src/bin/tdma_rc1_deepseek_*`)
> totaling 1816 LOC with 78% structural overlap. A
> `src/bin/tdma_rc1_runner_lib.rs` plus three thin `main()` shims would
> collapse this to ~700 LOC.

Atom 18 implements exactly these two recommendations. The Karpathy
findings serve as the design rationale.

## R-022-skip rationale

Use token `[R-022-skip: OBS_R022_TDMA_ATOM18_K10_K11_REFACTOR.md REQUIRED]`
in the commit message. The skip is justified because:

1. The TRACE_MATRIX inventory is **preserved at workspace level** (every
   moved symbol's backlink lives at its new path in `src/tdma_runner.rs`).
2. The refactor is **Class 2 additive + thin-shim removal**, not a
   constitutional change.
3. The audit dispatch that mandated this refactor (Karpathy-audit K10/K11)
   is recorded in the session log.
4. The pattern of "moved with backlink-preservation" is the same one
   accepted by prior atoms (e.g., the library-ization of
   `spec_capsule` from `src/bin/turingos/spec_capsule.rs` to
   `src/runtime/spec_capsule.rs` during TISR Phase 6 A6).
