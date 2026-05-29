# OBS R-022 — `turingos_dev` self-hosting sidecar removed (TRACE_MATRIX backlinks retired)

**Date**: 2026-05-29 (Harness platform-agnostic unification, Phase 2).
**Triggered by**: commit-msg hook R-022 (TRACE_MATRIX pub-symbol-block, removal arm / I-REMOVAL).
**Authorization**: user §8 ratification 2026-05-29 — "§8 批准 turingos_dev 移除 + genesis_payload.toml:210 重算 pin" (Class-4 per-atom sign-off; the rehash is the Class-4 act, see Trust-root note).
**Removed symbols**: the `pub mod dev_harness;` declaration at `src/runtime/mod.rs` and every `pub` item in the deleted `src/runtime/dev_harness.rs` (the dev-run task contract, open/record-diff/record-command/record-audit/validate/close API, hash-chain validation view, fail-closed error surface) plus the `src/bin/turingos_dev.rs` binary entrypoint.
**Removed backlinks**: 19 × `/// TRACE_MATRIX FC3-N33 + FC3-N43` doc-comments — 1 in `src/runtime/mod.rs` (the "Unified Agent Harness 2026-05-13" module-decl block) and 18 in the deleted `src/runtime/dev_harness.rs`. `src/bin/turingos_dev.rs` carried none.

## Why removal is correct

`turingos_dev` was a **development-evidence sidecar**, never a canonical tape: a JSONL hash-chain recorder for harness/code dev runs (open → record-diff → record-command → record-audit → validate → close). It was introduced 2026-05-13 ("Unified Agent Harness") and wired into `src/runtime/mod.rs` only as an auto-discovered binary plus a non-authoritative module.

Per architect directive 2026-05-29 (this Phase-2 atom), the sidecar is retired: the project's audit doctrine is now a **platform-agnostic clean-context audit** (Phase 1, commit `fd02260a`), which does not depend on a bespoke self-hosting evidence recorder. The sidecar added a second evidence-recording surface to maintain with no canonical-tape role; removing it is a Karpathy "delete dead tooling" simplification, not a behavior change to any canonical path.

No sequencer admission rule, TypedTx schema / discriminant / signing payload, wallet, kernel, bus, CAS `ObjectType` schema, or genesis constitutional layout was touched. The change is the deletion of a dev-only module + its binary, the matching test (`tests/constitution_dev_harness.rs`), its constitution-gate triple (manifest entry + matrix-drift allowlist entry), its liveness-fixture group, its sanitized-runner shell-out exceptions, and the now-dead `AGENTS.md §10` / `HARNESS*` runbook prose.

## Why the TRACE_MATRIX entries do not need a replacement

The removed backlinks pointed at **FC3-N33** (runtime ArchitectAI proposal node) and **FC3-N43** (meta-architecture feedback / shielding / dashboard node). These are **shared, heavily-anchored FC3 meta nodes — not dev-harness-private nodes.** After this removal they remain richly witnessed by live production code, e.g.:

- **FC3-N33**: `src/state/sequencer.rs` (ArchitectAI proposal accept-state + verify), `src/state/typed_tx.rs` (proposal class + canonical bytes + signing payload), `src/bottom_white/ledger/system_keypair.rs` + `transition_ledger.rs` (proposal capsule signing + root binding).
- **FC3-N43**: `src/runtime/markov_capsule.rs`, `agent_role_classifier.rs`, `g7_structural_smoke.rs`, `market_opportunity_trace.rs`, `economic_judgment.rs`, `real5_roles.rs`, `agent_scheduler.rs`, and the feedback-archive / Veto-AI surfaces in `sequencer.rs` / `typed_tx.rs`.

`turingos_dev` was one *consumer* that framed itself as realizing the FC3 feedback/architect-proposal concept for dev runs. Removing that consumer removes its backlinks without weakening either node — both keep dozens of live anchors and their existing witnesses.

This is a **code-symbol removal, not a flowchart change.** `constitution.md` is explicitly out of scope for this upgrade; the three canonical flowchart hashes in `TRACE_FLOWCHART_MATRIX.md` are unchanged. FC3-N33 / FC3-N43 do not appear as rows in `TRACE_MATRIX_v3_2026-04-27.md` § J (no orphan registration is being vacated).

## Behavioral preservation evidence

- `cargo test --workspace --no-fail-fast` → exit 0 (whole workspace green post-removal).
- `bash scripts/run_constitution_gates.sh` → `[k-1-5] total=164 failed=0` (was 165; −1 is exactly the removed `constitution_dev_harness` gate — bidirectional manifest↔test consistency holds).
- `cargo test --test constitution_matrix_drift` → 3 passed (incl. `manifest_gates_subset_of_matrix_plus_allowlist` and `allowlist_doesnt_grow_silently`; `K23_SHIP_ALLOWLIST_SIZE` assert is `<=`, stays green after the allowlist entry is dropped).
- The KEEP-set `dev_harness` **script-classification label** in `tests/constitution_script_liveness_inventory.rs` + `tests/fixtures/liveness/script_liveness_inventory.toml` is an unrelated script-type category (not the deleted Rust module) and is intentionally untouched; that gate stays green.

## Trust-root note (the Class-4 act)

Removing the `pub mod dev_harness;` line changed the SHA-256 of `src/runtime/mod.rs`, which is Trust-Root-pinned in `genesis_payload.toml`. The pin was rehashed in the **same atomic commit** (`a9560e3a…daeead2e` → `cdfbbede…51990`) with a forward-only comment recording the §8 grant; the 2026-05-13 sidecar-inclusion comment above it is preserved per `feedback_no_retroactive_evidence_rewrite`. Verification: `sha256sum src/runtime/mod.rs` equals the new pin, and `cargo run --bin turingosv4` prints "Trust Root verified" (no `TRUST_ROOT_TAMPERED` panic).

## Cross-references

- Authorization: user §8 ratification 2026-05-29 (in-session; recorded here and in the `genesis_payload.toml` rehash comment).
- Phase 1 predecessor (audit doctrine generalized): commit `fd02260a` "docs(harness): platform-agnostic audit doctrine".
- Forward-supersedes the 2026-05-13 "Unified Agent Harness" sidecar-inclusion decision (no historical evidence rewritten).
- Precedents (removal + rehash discipline): `OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md`, `OBS_R022_TB14_PRICEINDEX_REMOVED_2026-05-03.md`.
