# Clean-context audit — `turingos_dev` removal (Class 4)

**Date**: 2026-05-29.
**Subject commit**: `4264b14f` "refactor: retire turingos_dev sidecar (Class 4)" (parent `fd02260a`).
**Risk class**: 4 (trust-root pin rehash). User §8 per-atom sign-off granted 2026-05-29.
**Auditor**: fresh clean-context agent (platform-agnostic clean-context audit per `AGENTS.md §9`); did not hold the implementation transcript; reviewed only the isolated diff `fd02260a..4264b14f` + cited artifacts, running its own verification commands.

## Verdict

**PROCEED** — witness domain **NO-VIOLATION** (scanned `AGENTS.md §6` restricted-surface, `§8` no-retroactive-rewrite, constitution-gate triple-coupling, trust-root pin invariant).

## Independently verified findings

1. **Trust-root rehash correct.** `sha256sum src/runtime/mod.rs` = `cdfbbede0ad3f6346ee5dea5bdd8954cdc4eb01e85fbd064ce88440b50451990`, exactly matching the new pin at `genesis_payload.toml:212`. The mod.rs block deletion and the pin rehash are in the SAME commit — no mid-state where boot would panic `TRUST_ROOT_TAMPERED`. Old pin `a9560e3a…` named as superseded.
2. **Restricted-surface honesty.** `git diff --name-only` touches only the dev-sidecar files (deleted), the gate triple, the liveness fixture, the sanitized-runner exceptions, the genesis pin, the doc files, and the new OBS doc. NO sequencer / typed_tx / wallet / kernel / bus / CAS-schema content change. The mod.rs diff is a pure block deletion.
3. **Gate triple coupling atomic.** `constitution_dev_harness` removed from all three legs in one commit: test file deleted, manifest `[[gate]]` block removed, matrix-drift allowlist entry (`constitution_matrix_drift.rs:44`) removed. The drift test is a subset check + a `len() <= 69` ceiling (`:169`); dropping one entry keeps both green.
4. **No-retroactive-rewrite.** The 2026-05-13 sidecar-inclusion comment and the 2026-05-26 WAL note in `genesis_payload.toml` are PRESERVED; the new rehash comment forward-supersedes them. The OBS_R022 doc is added-only (single-commit history).
5. **FC3-N33/N43 not orphaned.** Both nodes retain dozens of live anchors in `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `src/bottom_white/ledger/{transition_ledger,system_keypair}.rs`, and other `src/runtime/*` files. The 19 removed backlinks strand nothing.
6. **Reconstruction intact.** Nothing in the diff alters ChainTape/CAS canonical paths; prior runs remain replayable.

Bonus independent check: `production_module_liveness.rs` uses no hardcoded group count (only `>= N` floors + duplicate/path checks), so removing the `dev_harness` fixture group breaks no count assertion. The residual `"dev_harness"` strings in `script_liveness_inventory.toml` / `constitution_script_liveness_inventory.rs` are a generic script-classification label (`.claude/hooks`, `rules`, `tools`), not references to the retired module — no dangling reference.

## Gate evidence (from implementation, consistent with audit)

- `cargo run --bin turingosv4` → "Trust Root verified" (no panic).
- `cargo test --workspace --no-fail-fast` → exit 0.
- `bash scripts/run_constitution_gates.sh` → `total=164 failed=0` (was 165; −1 = removed `constitution_dev_harness`).
- `cargo test --test constitution_matrix_drift` → 3 passed.

## Cross-references

- Justification doc (R-022 backlink removal): `handover/alignment/OBS_R022_TURINGOS_DEV_REMOVAL_2026-05-29.md`.
- Authorization: user §8 ratification 2026-05-29 (recorded in the genesis rehash comment + the OBS doc).
- Phase 1 predecessor: commit `fd02260a` (platform-agnostic audit doctrine).
