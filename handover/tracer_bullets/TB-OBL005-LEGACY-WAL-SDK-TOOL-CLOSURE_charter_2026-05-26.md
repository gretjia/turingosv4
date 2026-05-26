# TB-OBL005-LEGACY-WAL-SDK-TOOL-CLOSURE — Class 3 Charter

**Date**: 2026-05-26  
**Risk class**: Class 3  
**Phase ID**: `Phase-OBL005-LegacyWalSdkToolClosure`  
**Source of truth**: `constitution.md` + ChainTape/CAS; this charter is a derived planning artifact.

## 1. Constitutional Anchor

The OBL-005 stop condition requires that retained production code is both necessary and lit by real evidence. The three constitutional flowcharts require a single tape/log substrate:

- FC1: state advances through typed wtool admission into the canonical tape.
- FC2: boot/replay reconstructs from ChainTape/CAS.
- FC3: logs archive feeds typed ArchitectAI/Veto-AI/reinit paths through ChainTape/CAS, not off-tape side channels.

After PR #186, active matrices and `fc_alignment_conformance` no longer cite `src/wal.rs` as FC authority. Remaining legacy quarantine is therefore a source-level closure problem, not a documentation problem.

## 2. Current Code Fact

Remaining liveness quarantine:

```toml
id = "legacy_wal_and_sdk_tool_surfaces"
module_ids = ["wal", "sdk::tool"]
paths = ["src/wal.rs", "src/sdk/tool.rs"]
allowed_as_fc_authority = false
```

Actual dependencies:

- `src/lib.rs` still exports `pub mod wal`.
- `src/bus.rs` still has `wal: Option<crate::wal::Wal>` and `TuringBus::with_wal_path`.
- `tests/wal_resume.rs` still proves legacy WAL resume.
- `src/sdk/tool.rs::ToolSignal` still contains legacy f64 economic variants:
  - `YieldReward { reward: f64 }`
  - `InvestOnly { amount: f64, ... }`
- `src/bus.rs` still matches those variants even though TB-9/TB-14 disabled f64 economy mutation.
- `WalletTool` still implements `TuringTool`, but wallet is now read-only economy substrate and should not depend on f64-capable hook variants.

## 3. Required Closure Shape

### A. Delete legacy WAL production surface

Remove:

- `src/wal.rs`
- `pub mod wal` from `src/lib.rs`
- `wal: Option<crate::wal::Wal>` from `TuringBus`
- `TuringBus::with_wal_path`
- WAL writes from `TuringBus::init`, `append_internal`, and `halt_and_settle`
- `tests/wal_resume.rs`

Replace tests that used WAL resume with ChainTape boot/resume tests already present:

- `tests/constitution_flowchart_livenow.rs::fc2_boot_replay_and_resume_are_live`
- `tests/constitution_g1_resume.rs`
- `tests/constitution_true_suite_boot_cli_runner.rs`

### B. Remove f64 economic signal variants from the SDK tool hook

Remove from `src/sdk/tool.rs`:

- `ToolSignal::YieldReward { reward: f64 }`
- `ToolSignal::InvestOnly { amount: f64, ... }`
- `BetDirection` if it becomes unused

Update `src/bus.rs` to match only:

- `ToolSignal::Pass`
- `ToolSignal::Veto(reason)`

This keeps `TuringTool` as a bounded lifecycle hook for read-only/projector tools while removing the dead economic mutation vocabulary.

### C. Keep typed current surfaces intact

Do not touch:

- `src/state/sequencer.rs`
- `src/state/typed_tx.rs`
- `src/bottom_white/cas/schema.rs`
- canonical signing payloads
- market/economy state machines

If implementation discovers one of these is required, abort and re-scope as Class 4.

## 4. Acceptance Criteria

1. `rg -n "pub mod wal|crate::wal|with_wal_path|Wal::|src/wal.rs" src tests handover/alignment/TRACE_FLOWCHART_MATRIX.md handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` returns no active production/test authority hits. Historical archive hits are allowed only in clearly historical files.
2. `rg -n "YieldReward|InvestOnly|BetDirection|f64" src/sdk/tool.rs src/bus.rs src/sdk/tools/wallet.rs` returns no legacy f64 tool-signal economy hits.
3. `tests/fixtures/liveness/production_module_liveness.toml` has no `legacy_wal_and_sdk_tool_surfaces` group, or the group is empty and fails closed if reintroduced.
4. `tests/constitution_production_module_liveness.rs` asserts no exported module is assigned to that legacy group.
5. Existing current-path gates stay green:
   - `cargo test --test constitution_flowchart_livenow`
   - `cargo test --test constitution_g1_resume`
   - `cargo test --test constitution_economy_gate`
   - `cargo test --test fc_alignment_conformance`
   - `cargo test --test constitution_production_module_liveness`
   - `cargo test --test constitution_matrix_drift`
6. Full gate stays green:
   - `bash scripts/run_constitution_gates.sh`
7. `cargo check --workspace` stays green.

## 5. Kill / Re-scope Criteria

Abort this atom and open a Class 4 charter if closure requires:

- typed tx schema/discriminant change
- sequencer admission-rule change
- CAS `ObjectType` schema change
- canonical signing payload change
- replacing ChainTape replay semantics

Abort or split if:

- deleting `TuringTool` entirely breaks active wallet/economy/tool registry paths
- a real current-kernel runner still requires WAL-only persistence

## 6. Implementation Notes

Preferred implementation path:

1. Delete WAL first and run compile to reveal remaining references.
2. Remove f64 `ToolSignal` variants and simplify bus matching.
3. Update liveness manifest/test to remove the legacy group.
4. Update active docs only where they refer to current authority.
5. Run targeted gates, then full gates.
6. Request clean-context audit witness before PR merge.

This atom should be one PR if compile impact remains limited to `src/bus.rs`, `src/lib.rs`, `src/sdk/tool.rs`, `src/sdk/tools/wallet.rs`, tests, and liveness docs. Split if `TuringTool` execution itself must be redesigned.

## 7. §8 Ratification Block

This Class 3 charter requires explicit user/architect sign-off before implementation:

```text
[ ] APPROVED-WAL-SDKTOOL-CLOSURE — delete legacy WAL and remove f64 ToolSignal variants in one PR
[ ] APPROVED-WAL-ONLY — delete legacy WAL first; defer ToolSignal cleanup
[ ] APPROVED-SDKTOOL-ONLY — remove f64 ToolSignal variants first; defer WAL deletion
[ ] DEFER — keep quarantine; OBL-005 remains open
[ ] REVISE — return with modifications
```

**Sign-off line**: `_________________________________`  
**Date signed**: `___________`

## 8. Closing Verdict Domain

Implementation audit must return one of:

- `NO-VIOLATION`
- `VIOLATION-FOUND <constitutional-clause> <file>:<line>`
- `RECONSTRUCTION-FAILURE <which-tape-or-cas-path-cannot-be-reconstructed>`
- `SECOND-SOURCE-DRIFT <which-derived-view-is-usurping-ground-truth>`

