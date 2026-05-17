# OBS_R022 — TISR Phase 6.1 CLI Dispatch Surface

**Date**: 2026-05-17
**Branch**: `codex/tisr-phase6-cli`
**Scope**: `pub(crate)` items added under `src/bin/turingos/**`

This OBS documents the R-022 reverse-map rationale for the
TISR Phase 6.1 CLI dispatch surface added in atoms W0 through W4.

Each `src/bin/turingos/cmd_<NAME>.rs` module exports three `pub(crate)` items:

- `SHORT_HELP: &str` — short registry-line description
- `FULL_HELP: &str` — full `--help` text
- `run(args: &[String]) -> ExitCode` — entry handler

Plus shared helpers in `src/bin/turingos/common.rs`:

- `shell_quote_path(path: &Path) -> String`
- `run_external(bin_name: &str, args: &[String]) -> ExitCode`

The entry binary `src/bin/turingos.rs` exposes:

- `Subcommand` struct (CLI dispatch table entry type)

All items carry `/// TRACE_MATRIX FC2-N16: <role>` doc-comments
immediately preceding the declaration. This satisfies R-022
PASS via the contiguous-preceding-block rule
(`scripts/check_trace_matrix.py` lines 190–200).

These items are NOT public API; they are CLI dispatch internals
visible only within the `turingos` binary crate (`pub(crate)` scope).
Future Phase 6.2+ extensions follow the same pattern.

Justification anchor for Wave 1–4 atom commits if the pre-commit
hook requires an explicit skip token:

```
[R-022-skip: see OBS_R022_TISR_PHASE6_1_CLI_DISPATCH.md]
```

FC-trace: FC2-N16
