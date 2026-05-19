# A8b — Synthesis prompt: inline literal → runtime fs load

**Status**: Done inline by orchestrator (2026-05-19). Cargo check clean; all 79 web_spec tests pass.

**Pre-fix HEAD**: `3e0fa79c` (uncommitted on `codex/tisr-phase6-3-x-grill-driven`).

## Problem (surfaced by A8 agent)

`cmd_spec.rs::system_prompt(lang) -> String` at lines 1505–1554 carried the synthesis prompt as an **inline Rust string literal** with hardcoded zh + en variants. The `assets/prompts/grill_synthesis_{zh,en}.md` files were doc mirrors only — **runtime ignored them**.

Consequence: A8's drafted v2 prompts (`grill_synthesis_zh_v2.md` + `grill_synthesis_en_v2.md`) cannot take effect by a file swap. Π4 E2E test would need either a rebuild-each-swap workflow OR a runtime load.

## Fix

- Stripped `# TuringOS 合成提示 — 中文版 v1` (and English equivalent) header + blank line from both `.md` files so they match the inline literal byte-for-byte.
- Rewrote `system_prompt()` as a thin wrapper around new `system_prompt_from(lang, workspace)` that:
  1. Tries `std::fs::read_to_string(<workspace>/assets/prompts/grill_synthesis_<lang>.md)`
  2. Falls back to `include_str!(...)` baked-in v1 content on read failure (so test environments without a workspace asset never crash)
- Mirrors F4 pattern (meta-prompt) for runtime fs load consistency.

## Files touched

- `src/bin/turingos/cmd_spec.rs:1505-1554` — function rewrite (3 call sites at lines 286, 311, 1333 unchanged because old `system_prompt(lang)` signature preserved as alias)
- `assets/prompts/grill_synthesis_zh.md` — removed 2 header lines (now byte-equal to inline literal)
- `assets/prompts/grill_synthesis_en.md` — removed 2 header lines

## Verification

- `cargo check --features web` — clean (existing 7 warnings only)
- `cargo test --features web --test web_spec_turn_endpoint` — 79/79 pass
- `cargo test --features web --bin turingos system_prompt` — 0 specific tests (function untested by name) but binary builds + existing cmd_spec tests pass

## Net effect

- Default behavior unchanged (workspace-relative read finds canonical v1 .md → returns same content as old inline literal).
- A8 v2 prompts can now be A/B-tested via file swap: `cp grill_synthesis_zh_v2.md grill_synthesis_zh.md` → next session uses v2. No rebuild needed (unlike include_str!-only).
- Restored to canonical at close per orchestrator discipline.

## Followup

- A8b enables Π4 mini-wave that swaps A8 v2 into active position and validates D-NEW-1 hallucination fix.
- Long-term: consider extracting a single helper `load_prompt_or_fallback(role, lang, workspace)` since this is now the 3rd runtime-fs-load site (meta, triage, synthesis). Defer as A12 atom.

## Constraints honored

- Branch unchanged (`codex/tisr-phase6-3-x-grill-driven` HEAD `3e0fa79c`)
- No Class-4 surface touched
- No Cargo.toml/Cargo.lock change
- No commit/push
