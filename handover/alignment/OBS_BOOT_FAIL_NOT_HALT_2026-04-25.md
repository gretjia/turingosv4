# OBS — Boot failure (TRUST_ROOT_TAMPERED) is not a constitutional HALT

**Date**: 2026-04-25 (Phase B B7 alignment follow-up)
**Author**: Claude (auto, B7 alignment cycle)
**Type**: alignment observation, no constitution change requested
**Per CLAUDE.md "Alignment Standard"**: constitution.md hygiene observations登记到 handover/alignment/OBS_*.md, 不改宪法.

---

## Observation

Phase B B7 added a Boot-time Trust Root verification (`turingosv4::boot::verify_trust_root`) and wired it into `src/main.rs`. On any SHA-256 mismatch, the process panics with `TRUST_ROOT_TAMPERED: ...`.

This panic is **not** a FC2-N22 HALT, even though the surface effect ("process stops") looks similar. The mapping question matters because TRACE_MATRIX_v1 needed to decide where to attach this code path in the constitutional flowcharts.

## Why TRUST_ROOT_TAMPERED ≠ FC2-N22 HALT

| Property | FC2-N22 HALT | TRUST_ROOT_TAMPERED panic |
|---|---|---|
| Triggered by | constitutional condition (∏p=0 cumulative, max-tx exhausted, OmegaAccepted, wall-clock cap, compute-cap, error-halt) | manifest hash mismatch in genesis_payload.toml |
| Pre-condition | kernel + bus + agents initialized; tape exists | nothing initialized — `Kernel::new()` has not been called |
| State after | `QState::Halted { reason: HaltReason::* }` durable in the bus | process exit; no QState exists |
| Halt reason emitted? | Yes, via `TuringBus::halt_with_reason` → JSONL row | No — emit channel doesn't exist yet |
| Recoverable in-process | sometimes (depends on HaltReason) | never — process gone |

FC2 has a `HALT` node (FC2-N22) that lives *inside* the boot/tick lifecycle. TRUST_ROOT_TAMPERED fires *before* the boot lifecycle — it is a precondition violation on the readonly base.

## Closer constitutional match: FC3-E14 (`error → re-init → boot`)

FC3 has a top-level edge `init → error → re-init → boot` (line 711 of constitution.md, FC3-E13/E14). The semantics:

- Something during init detected a problem ("need to improve?" rhombus, FC3-N12).
- Control returns to `boot` (FC3-N1).
- The system tries again.

Trust Root verification *is* an init-time precondition check. If it fails, the constitutional response is "re-init" — return to boot. In v4, "return to boot" = the process exits and the surrounding harness (shell, batch runner) restarts it. The `panic!` is the immediate-abort leaf of FC3-E14.

This matches even though FC3 conceives of the error→re-init loop as automated. In Phase B v4, automation lives outside the binary (TRACE_MATRIX_v0 § 1 row FC3-N41 currently 📅 Phase 11+).

## Why not introduce a HaltReason::TrustRootTampered variant

Tempting, but wrong:

1. The bus is not initialized when the panic fires; there is no place to record `QState::Halted { reason: TrustRootTampered }`.
2. Adding the variant would lie about the constitutional structure — we would be claiming Trust Root verification is part of the FC2 boot/tick cycle, when it is in fact a precondition for FC2 to begin.
3. Doing so would invite future code to *catch* the variant and try to recover, which is exactly the opposite of the intent: a tampered manifest must abort, not be handled.

## Proposed action

**None now**. This OBS file documents the decision. No constitution change requested.

If/when Phase 11+ lands an in-process re-init mechanism (TRACE_MATRIX FC3-N41), it can use `Result<(), TrustRootError>` from `verify_trust_root` instead of letting the panic propagate. At that point the panic becomes the *uncaught* path and structured retry becomes the *caught* path. Constitution still does not need to change — both paths land at FC3-E14.

## Cross-references

- `src/main.rs:11-14` — call site, TRACE_MATRIX backlink to FC3-E14
- `src/boot.rs:62` — `verify_trust_root` itself, TRACE_MATRIX backlink to FC3-N34
- `handover/alignment/TRACE_MATRIX_v1_2026-04-25.md` § 5 — summary in the matrix
- `constitution.md` line 670-714 (FC3 mermaid block) — source flowchart
