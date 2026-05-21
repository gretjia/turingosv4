# OBS R-022 — read_api_key_env_var TRACE_MATRIX backlink relocated, not removed

**Date**: 2026-05-21 patch P2 (plan: multi-agents-orchestrator-flash-agents-dazzling-eich).
**Triggered by**: pre-commit hook R-022 (TRACE_MATRIX backlink removal detector).
**Detected removal** (in `src/bin/turingos/cmd_llm.rs`):
1. `/// TRACE_MATRIX FC2-N16: env-var NAME lookup (never the key value).` (was on `read_api_key_env_var`).

## Why this is a replacement, not an orphan removal

`read_api_key_env_var` was a single function with a Meta→Blackbox fallback chain
that silently borrowed the other role's key when only one slot was configured.
This patch (P2 of the dual-key plan) deletes that function and replaces it with
two independent role-scoped readers:

| Deleted symbol | Replacement symbols |
|----------------|---------------------|
| `read_api_key_env_var` (FC2-N16 backlink) | `read_meta_api_key_env` (FC2-N16 backlink) |
|                                             | `read_blackbox_api_key_env` (FC2-N16 backlink) |

Both replacement functions carry `/// TRACE_MATRIX FC2-N16:` doc-comments.
The constitutional FC2-N16 node (LLM client boot adapter) is covered by two
symbols instead of one; no FC2-N16 coverage is lost.

## Validation

- `cargo check`: exit 0 (warnings only, pre-existing).
- `cargo test --test dual_key_no_fallback`: 2 passed.
- `bash scripts/run_constitution_gates.sh`: 133 tested, 0 failed.

## Cross-references

- Plan: `.claude/plans/multi-agents-orchestrator-flash-agents-dazzling-eich.md` (patch P2).
- Precedent: `handover/alignment/OBS_R022_C2_C8_REJECTION_CAPSULE_RELOCATION_2026-05-21.md`.
