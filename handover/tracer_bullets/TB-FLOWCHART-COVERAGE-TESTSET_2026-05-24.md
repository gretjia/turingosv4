# TB-FLOWCHART-COVERAGE-TESTSET -- Test Set Design

Date: 2026-05-24
Risk class: Class 2 harness / constitution-gate design
Branch: codex/flowchart-coverage-test-design
User obligation: OBL-005

## 1. Objective

Design an executable test set that can prove the three constitutional
flowcharts are not merely documented:

- FC1 runtime loop participates in a real or deterministic replayable run.
- FC2 boot / replay / once-only initialization participates in the same
  state-machine path.
- FC3 meta-architecture boundaries participate as enforceable audit and
  evidence constraints.
- No production code surface is a zombie flowchart backlink.
- No flowchart node is marked covered while its current code or test surface
  is missing.

This document is intentionally a design artifact, not a pass certificate. The
research pass found current gaps that would make a strict all-green flowchart
coverage gate fail today.

## 2. Current Audit Verdict

Verdict: CHALLENGE

The current project has strong predicate-registry binding, ChainTape replay,
and FC3 shielding evidence after W3-2. The 2026-05-24 stale-mapping repair
replaced the known retired names in `TRACE_FLOWCHART_MATRIX.md`,
`CONSTITUTION_EXECUTION_MATRIX.md`, and `tests/fc_alignment_conformance.rs`
with current surfaces where they exist. A trustworthy test set still must
expose the remaining non-closed flowchart work before it can certify full
flowchart liveness.

Repair status after the 2026-05-24 stale-mapping pass:

1. FC1 stale surfaces `ReadTool::project`, `DefaultReadTool`,
   `WriteTool::write`, and `WriteTool::write_with_tools` were replaced by
   current surfaces: `Rtool::checkout_digest`, `SessionDigest`,
   `UniverseSnapshot`, `build_agent_prompt`, `TuringBus::submit_typed_tx`,
   `Sequencer::apply_one`, and legacy `TuringBus::append_oracle_accepted`.
2. FC1 still lacks one deterministic single-loop gate from `Q_t` to rtool/input to
   stub delta/output to predicate registry to sequencer accept/reject.
3. FC2 boot stale surfaces `src/runtime/evaluator.rs::run_swarm` /
   `run_oneshot` were replaced by the current ChainTape boot factory:
   `build_chaintape_sequencer`, `build_chaintape_sequencer_with_initial_q`,
   `QState::genesis`, and `initial_q_state.json`.
4. FC2 terminal stale surfaces `QState::Halted`, `HaltReason`, and
   `extract_halt_reason` were replaced by current typed chain anchors:
   `TerminalSummaryTx`, `RunOutcome`, `ExhaustionReason::to_run_outcome`, and
   `SystemEmitCommand::TerminalSummary`.
5. FC2 map-reduce tick remains missing on the current production path. The old
   `TICK_INTERVAL` / `emit_mr_tick_node` surface was retired with the
   pre-ChainTape evaluator and must not be counted green.
6. FC3 has real shielding and capsule evidence, but anti-oreo topology is
   mostly structural; there is no direct gate proving top-only management,
   agent/tool lifecycle order, and no lower-layer Q/log bypass.
7. FC3-N40 legacy `fc3_n40_logs_to_architect_feedback` stub was replaced by a
   live deep-history override witness matching current
   `TRACE_FLOWCHART_MATRIX.md`.
8. Some real-evidence checks can degrade to weak evidence when historical
   local evidence directories are absent.

## 3. Test Layers

### Layer A: Matrix Truthfulness Gates

These gates must run first. They are Class 2 and touch only tests/docs/scripts.

Test file target: `tests/constitution_flowchart_coverage.rs`

Required tests:

1. `flowchart_matrix_has_canonical_node_sets`
   - Parse `handover/alignment/TRACE_FLOWCHART_MATRIX.md`.
   - Require exact node sets:
     - FC1-N1..FC1-N15
     - FC2-N16..FC2-N28
     - FC3-N29..FC3-N40
   - Fail on duplicate, missing, or unexpected active node IDs.

2. `flowchart_matrix_code_surfaces_resolve`
   - For every non-N/A row, parse the `Code surface` column.
   - Each row must resolve at least one current source or document anchor.
   - Backtick paths must exist.
   - Backtick Rust symbols must resolve to a declaration or typed API use, not
     merely appear in comments. Allowed declaration patterns are `pub struct`,
     `pub enum`, `pub trait`, `pub fn`, `impl ...`, `fn`, `const`, `mod`, or a
     compile-time type assertion in a non-ignored test. The scanner must strip
     `//`, `//!`, `///`, and block comments before symbol matching.
   - A row can only pass through tests if the referenced test contains a
     non-ignored compile-time API binding, for example `let _: fn(...) = symbol`
     or a concrete constructor/call. A doc-comment occurrence is not a binding.
   - Stale names are failures unless the row explicitly labels them as
     `legacy` and points to the replacement surface.

3. `flowchart_matrix_tests_resolve`
   - For every non-N/A row, parse the `Constitution gate test` column.
   - Each row must resolve at least one non-ignored test function, test file,
     or manifest-authorized `constitution_*` gate.
   - `covered by existing` without a concrete file or function is a failure.
   - Ignored panic stubs do not count as coverage.

4. `flowchart_backlinks_do_not_point_to_missing_nodes`
   - Scan `src/`, `tests/`, and `scripts/` for `TRACE_MATRIX FC1-N*`,
     `FC2-N*`, `FC3-N*`.
   - Every canonical node reference must appear in
     `TRACE_FLOWCHART_MATRIX.md`.
   - Supplemental namespaces such as `FC1a`, `FC1b`, `FC1-N41+`, TB-specific
     orphans, and `FC3-N43` must be registered in an explicit supplemental
     namespace section, not silently counted as canonical FC1/FC2/FC3 coverage.

5. `flowchart_green_rows_have_no_ignored_stub_dependency`
   - Cross-check green rows against `#[ignore]` stubs in
     `tests/fc_alignment_conformance.rs` and other flowchart conformance files.
   - A green row fails if its only concrete witness is ignored or panics.

Expected current outcome if implemented strictly: FAIL until the stale matrix
surfaces and true missing coverage are repaired or explicitly reclassified.

### Layer B: Single-Flowchart Executable Gates

These gates prove the flowcharts participate in current code paths.

1. FC1: `fc1_single_runtime_loop_accept_and_reject`
   - Fixture:
     - `QState::genesis()` with activated `predicate_registry_root_t`.
     - `PredicateRegistry::from_boot_manifest(BootPredicateManifest::v8_production())`.
     - In-memory CAS/proof store and `ToolRegistry::new()`.
     - Two `TypedTx::Work` envelopes carrying parsed `AgentAction` payloads:
       one valid small proof artifact, one value that fails an executable
       predicate.
   - Assertions:
     - Accepted path returns a L4 `LedgerEntry` with `TxKind::Work`, nonzero
       `state_root_after`, and the active predicate registry root recorded.
     - Rejected path returns an L4.E record, does not append L4, and leaves
       `state_root_after == state_root_before`.
     - The test mutates runner-stamped predicate booleans to forged `true` and
       still observes registry recomputation reject the bad path.
   - Assert predicate verdicts are recomputed by executable registry code, not
     trusted from runner-stamped booleans.

2. FC1: `fc1_rtool_prompt_bridge_is_q_t_derived`
   - Fixture:
     - Build a `UniverseSnapshot` with `sequencer_wired = true`, `tx_count = 1`,
       one deterministic tape node label, one `price_index` entry, and one
       `mask_set` entry.
     - Call the current prompt surface with explicit fields:
       `chain_so_far = rendered snapshot/tape summary`, `market_ticker =
       rendered price_index`, `recent_errors = []`, `recent_search_hits = []`,
       `team_board = ""`, and a fixed `tools_description`.
   - Allowed prompt facts:
     - rendered tape/head identifier,
     - rendered Q-derived market ticker,
     - fixed skill/tool text supplied by the fixture.
   - Forbidden prompt facts:
     - raw stderr markers (`stderr`, `traceback`, `panicked at`, `Lean error:`),
     - filesystem-only paths not present in the fixture,
     - mutable dashboard-only counters,
     - hidden prompt variant/debug metadata.
   - Assertion method:
     - Each expected fixture token must appear exactly from the supplied
       fixture field.
     - Each forbidden token must be absent.
     - The test must fail if `build_agent_prompt` is fed data not derivable
       from the fixture's `Q_t`/tape/HEAD fields.

3. FC2: `fc2_boot_sequence_emits_activation_and_replays`
   - Use `build_chaintape_sequencer`.
   - Assert `PredicateBindingActivate` appears as a tape-visible system tx.
   - Assert `initial_q_state.json`, `pinned_pubkeys.json`, CAS, and registry
     root are present.
   - Run `verify_chaintape` and assert all replay indicators pass.

4. FC2: `fc2_map_reduce_tick_is_executable`
   - Assert a clock-triggered map-reduce tick has a current production entry
     point.
   - Assert the tick reads tape and writes a deterministic reduced output.
   - Current status: blocked, because current code does not expose
     `TICK_INTERVAL`, `emit_mr_tick_node`, or an equivalent current surface.

5. FC2: `fc2_halt_terminal_anchor_is_tape_visible`
   - Assert the terminal halt class is a typed chain-visible value.
   - Current likely replacement surface is `RunOutcome`, not the legacy
     `QState::Halted` / `HaltReason` shape.
   - Requires matrix repair before implementation can be honest.

6. FC3: `fc3_anti_oreo_topology_is_enforced`
   - Split this into two non-overlapping tests; do not pretend legacy tool
     hooks are proof for the typed sequencer path.
   - `fc3_legacy_tool_hook_order_is_not_l4_authority`
     - Fixture: define a test-only `RecordingTool` implementing `TuringTool`,
       with append-only vectors for `on_boot`, `on_init`, `on_pre_append`, and
       `on_post_append` events.
     - Exercise only the legacy `TuringBus::append` path, where these hooks
       actually run.
     - Assert lifecycle order is `on_boot -> on_init -> on_pre_append ->
       on_post_append` for accepted legacy appends.
     - Assert rejected legacy appends produce no `on_post_append` and are not
       counted as L4 authority; this test documents old hook behavior and does
       not certify FC1/FC2 ChainTape admission.
   - `fc3_typed_sequencer_path_has_no_tool_or_agent_direct_write`
     - Fixture: use `TuringBus::submit_typed_tx` / `Sequencer::apply_one` with
       a minimal typed tx path and a `ToolRegistry::new()` handle. Do not mount
       `RecordingTool`; `TuringTool` hooks are not invoked by typed submission
       today.
     - Assert all L4 / L4.E / state-root writes flow through
       `src/state/sequencer.rs` and
       `src/bottom_white/ledger/transition_ledger.rs`.
     - Scanner boundary: after stripping comments and tests, fail on agent/tool
       modules calling `Sequencer::apply_one`, `dispatch_transition`,
       `Wal::write_event`, low-level ChainTape ref writers, or direct
       `QState` root mutation. Allowed writers are restricted to
       `src/state/sequencer.rs`, `src/bottom_white/ledger/transition_ledger.rs`,
       `src/wal.rs`, and explicit boot/replay loaders.
     - Agent-facing modules may construct `AgentAction` / role actions and may
       submit typed tx through public ingress, but may not directly mutate
       Q/logs or bypass typed admission.
   - `handover/audits/` and directive files count as evidence for Veto-AI /
     ArchitectAI role boundaries, not as runtime code connectivity.

### Layer C: Real-Evidence Gates

These gates prevent synthetic-only closure.

1. `fc1_real_evidence_not_ignored`
   - Replace the ignored P38/P49 panic witness with a non-ignored real evidence
     reader.
   - Fail if required historical evidence is absent, unless a committed CI
     fixture supplies the same data.

2. `flowchart_real_evidence_replays_from_raw_tape_and_cas`
   - Regenerate facts from runtime repo + CAS rather than trusting
     precomputed `chain_invariant.json`.

3. `fc3_markov_capsule_real_run_binding`
   - Distinguish EvidenceCapsule regeneration from MarkovEvidenceCapsule
     runtime participation.

## 4. Zombie / Missing-Code Heuristics

The final suite should classify each hit into exactly one bucket:

- `canonical-flowchart-node`: FC1-N1..N15, FC2-N16..N28, FC3-N29..N40.
- `supplemental-namespace`: FC1a/FC1b, FC1-N41+, FC2-N29+, FC3-N41+,
  TB-specific additions with documented authority.
- `legacy-replaced`: old symbol retained only in docs with a replacement
  pointer.
- `missing-implementation`: flowchart row has no current code surface.
- `zombie-code`: code has a flowchart backlink but no matrix row, no
  supplemental namespace, and no orphan justification.

Only the first two buckets may be counted as coverage. `legacy-replaced`,
`missing-implementation`, and `zombie-code` are not coverage.

## 5. Implementation Sequencing

Recommended sequencing:

1. Land Layer A as an executable gate only after matrix rows are repaired to
   current code names or explicitly marked as gaps.
   - When adding `tests/constitution_flowchart_coverage.rs`, add an entry to
     `scripts/constitution_gates.manifest.toml` with this document as authority
     and add the gate name to `CONSTITUTION_EXECUTION_MATRIX.md`; otherwise
     `scripts/run_constitution_gates.sh` must fail closed.
2. Land FC1 and FC2 boot Layer B gates that can be implemented without
   restricted source edits.
3. Open a separate Class 4 charter if FC2 map-reduce tick requires typed
   transaction, sequencer, or canonical replay schema changes. Typed halt is
   currently represented by `TerminalSummaryTx.run_outcome`.
4. Land FC3 topology gate after deciding whether Veto-AI / ArchitectAI remain
   external structural roles or become runtime agents.
5. Promote Layer C real-evidence gates after CI fixtures are available and no
   test silently returns early on missing evidence.

## 6. Verification Commands

Once the strict gate is implemented and repaired, the acceptance command set is:

```bash
cargo test --test constitution_flowchart_coverage
cargo test --test constitution_fc1_runtime_loop
cargo test --test constitution_fc2_boot
cargo test --test constitution_fc3_meta
cargo test --test constitution_fc3_evidence_binding
cargo test --test constitution_matrix_drift
bash scripts/run_constitution_gates.sh
```

The suite may only claim `PREDICATES-GREEN` after all commands exit 0 and no
active flowchart row is green solely through ignored stubs, stale code names, or
dashboard/evidence summaries that cannot be reconstructed from tape/CAS.

## 7. Decision Needed Before Full Implementation

The design exposes at least one likely Class 4 fork:

- If FC2 map-reduce tick and terminal halt are constitutional runtime
  requirements, current code appears to need production implementation or schema
  repair.
- If those nodes have been superseded by `RunOutcome`, ChainTape replay, and
  derived facts, `TRACE_FLOWCHART_MATRIX.md` and
  `CONSTITUTION_EXECUTION_MATRIX.md` must be revised with explicit authority.

Until that fork is resolved, the honest status is:

`DESIGN-COMPLETE / FULL-FLOWCHART-CERTIFICATION-BLOCKED`.
