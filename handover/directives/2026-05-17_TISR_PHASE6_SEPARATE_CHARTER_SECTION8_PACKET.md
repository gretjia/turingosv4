# TISR Phase 6.0/6.1 — Separate Charter Section 8 Packet

**Status**: RATIFIED — activated by explicit user-as-architect Section 8 sign-off.
**Date**: 2026-05-17.
**Branch**: `worktree-tisr-2026-05-17`.
**TISR parent commit**: `ff71406c` (`TISR-001: dual-axis interaction substrate research`).
**Ratification record**:
`handover/directives/2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_SIGN_OFF.md`.

This document is the active Section 8 ratification packet for the first safe
implementation slice after TISR Phase 0-5 research. Its active scope is exactly
the scope in Sections 1-6 and the exclusions in Section 7.

## 1. Scope

Ratify only a narrow Phase 6.0/6.1 start:

1. Build a local `turingos` CLI MVP shell and P0 wrappers.
2. Build a local generative UI IR spike that consumes fixture or read-only
   ChainTape/CAS-derived views.
3. Run a real CLI happy-path witness after implementation evidence exists.
4. Preserve `lean_market` compatibility and treat it as the baseline user CLI.

This packet does not ratify Phase 7 Web MVP, production web serving,
multimodal artifact storage, CAS `ObjectType` additions, new typed transaction
variants, new signature types, sequencer admission changes, or any Trust Root
rehash.

## 2. FC Mapping

Touched or constrained nodes:

- FC1-N5 / FC1-N6: CLI and UI read views must remain scoped, shielded, and
  reconstructable from existing ChainTape/CAS evidence.
- FC1-N10 / FC1-N13: any write action must use existing CLI/bin surfaces or
  existing sequencer APIs; no new admission path.
- FC2-N16 / FC2-N21: `turingos init` and batch startup may wrap existing boot
  flows but must not introduce memory-only preseed or post-hoc genesis.
- FC3-N31 / FC3-N39: dashboards, UI IR, and generated views are materialized
  views only; they are never source of truth.

Invariants:

- No tape, no test: any conclusion-bearing run must produce real replayable
  evidence.
- Reports, dashboards, UI, and stdout remain views, not authority.
- Price, reputation, and UI confidence do not enter predicate truth or
  sequencer admission.

## 3. Risk Class

Default risk class for this packet: **Class 2**.

Lower-risk subwork:

- Class 0: docs, charter updates, issue lists, UI sketches.
- Class 1: additive CLI parser, formatters, local fixture-based UI IR renderer.
- Class 2: production CLI wrappers, runner integration, happy-path witness.

Automatic escalation:

- Any edit to `src/state/typed_tx.rs`, `src/state/sequencer.rs`,
  `src/bottom_white/cas/schema.rs`, `src/kernel.rs`, `src/bus.rs`,
  `src/sdk/tools/wallet.rs`, canonical signing payloads, RootBox, or sequencer
  admission is **not authorized** here and requires a separate Class 4 atom.
- Any required Trust Root pinned-file rehash is **not authorized** here and
  requires a separate explicit packet.
- Any evidence-bearing run that writes `handover/evidence/` requires preflight
  and must not rewrite historical evidence.

## 4. Allowed Paths

Allowed implementation surfaces:

- `src/bin/turingos.rs`
- `src/bin/turingos/**`
- `src/cli/**`
- `tests/cli_*.rs`
- `tests/constitution_tisr_provenance.rs`
- `experiments/tisr_ui_spike/**`
- `handover/directives/2026-05-17_TISR_PHASE6_*`
- `handover/reports/TISR_PHASE6_*`
- `handover/evidence/stage_phase6_cli_*`

Conditional path:

- `Cargo.toml` and `Cargo.lock` only if the implementation needs already
  standard CLI dependencies such as `clap`, and only after a diff note confirms
  this does not pull a frontend stack into Phase 6.

Not allowed in this packet:

- `src/runtime/mod.rs`
- `src/bin/audit_dashboard.rs`
- `experiments/minif2f_v4/src/bin/evaluator.rs`
- `genesis_payload.toml`
- Any restricted surface named in Section 3.

If the implementation cannot proceed without a disallowed path, stop and write
a new ratification request. Do not smuggle it under this packet.

## 5. Implementation Gates

Before implementation:

1. Fix the known TISR Phase 0-5 documentary hygiene issues or explicitly carry
   them as non-shipping debt:
   - `git show --check ff71406c` currently reports trailing whitespace.
   - TISR index/summary still have stale HEAD/doc-count lines.
   - No GitHub PR exists yet for `worktree-tisr-2026-05-17`.
2. Confirm current base against `origin/main`.
3. Record whether `src/state/typed_tx.rs` has changed since 2026-05-17.

During implementation:

1. Add or identify the smallest failing gate first.
2. Keep the first slice small: CLI parser plus read-only wrappers before any
   write-capable command.
3. For the local generative UI IR spike, use fixture or read-only view input
   and make it non-authoritative.
4. Do not start Phase 7 frontend or web server work under this charter.

Exit gates:

1. `git diff --check`
2. Targeted CLI tests for the touched command set
3. `cargo check`
4. `cargo test --workspace --no-fail-fast` before any ship claim
5. `bash scripts/run_constitution_gates.sh` before any ship claim
6. One clean-context Codex audit with verdict `PROCEED` for the Phase 6 slice

## 6. Real Witness Requirement

The first ship claim under this packet needs a real local witness:

1. `turingos init`
2. create or point to a runtime repo
3. open or wrap one Lean task through the existing lawful path
4. render a read-only UI IR view from the resulting state/evidence
5. export an evidence bundle
6. verify replay or audit using existing tools

Witness output path:

```text
handover/evidence/stage_phase6_cli_<timestamp>/
```

The witness may be negative or partial. Do not convert a failed witness into a
dashboard-only proof.

## 7. Explicit Non-Authorization

This packet does not authorize:

- Phase 7 Web MVP.
- React/frontend production stack.
- CAS `ObjectType` expansion.
- `UIEventCapsule` or `A2AMessageCapsule` as new CAS object variants.
- `AgentProposedTaskOpen`, `AgentMarketSeeding`, `DirectSwapTx`, or any new
  typed transaction variant.
- `HumanSignature` or any new signature type.
- reputation-bound or price-bound admission filters.
- autonomous agent market creation.
- live REAL-6B.
- claims of E2, E3, E4, spontaneous market emergence, or real-world readiness.

## 8. Relationship To TISR Phase 0-5

TISR Phase 0-5 remains a forward-bound research package. This packet uses it as
design input but does not promote the whole package to an approved architecture.

The approved implementation source for this candidate is only the narrow Phase
6.0/6.1 scope above. Any broader item in TISR docs remains forward-bound until a
later separate charter.

## 9. Required Architect Sign-Off Form

To activate this packet, the user-as-architect must reply with an explicit
multi-clause authorization. Minimal acceptable form:

```text
我以架构师身份批准 TISR Phase 6.0/6.1 separate charter Section 8:
授权在 `worktree-tisr-2026-05-17` 或后续 `codex/tisr-phase6-cli` 分支上实施
Phase 6.0/6.1 narrow CLI MVP + local generative UI IR spike。
授权范围仅限本 packet Sections 1-6 的 allowed paths 和 gates。
不授权 Phase 7 Web、CAS schema、typed_tx、sequencer、signing、Trust Root rehash、
或任何 Class 4 surface 修改。
若实现触碰 Section 3/4 禁区，必须停止并另开独立 §8 packet。
```

Short forms such as `ok`, `go`, `继续`, `可以`, or "你来做" do not activate this
packet.

## 10. Post-Sign-Off Actions

After explicit Section 8 sign-off:

1. Create or switch to a dedicated implementation branch.
2. Open a `turingos_dev` run if the slice becomes evidence-bearing.
3. Implement the smallest CLI/UI IR slice.
4. Run the gates in Section 5.
5. Request clean-context Codex audit.
6. Only after `PROCEED`, decide whether to ship or continue to the next slice.

**End of TISR Phase 6.0/6.1 Section 8 candidate packet.**
