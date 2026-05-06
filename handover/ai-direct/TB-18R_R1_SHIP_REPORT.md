# TB-18R R1 Ship Report — Class 4 Schema Additions

**Atom**: TB-18R R1
**Class**: 4 (typed-tx schema + CAS schema; STEP_B_PROTOCOL applied)
**Branch**: `stepb-tb18r-r1-schema` (worktree at `.claude/worktrees/stepb-tb18r-r1-schema/`)
**Base commit**: `46d79ca` (main HEAD at worktree creation)
**Date**: 2026-05-06
**Status**: READY FOR REVIEW + USER APPROVAL TO MERGE

## §1 Workspace test count (SG-18R.11 ship gate)

```
command  = cargo test --workspace --no-fail-fast
location = .claude/worktrees/stepb-tb18r-r1-schema (off main 46d79ca + R1 patches)
passed   = 998
failed   = 1 (pre-existing; main also fails — see §1.1 below)
ignored  = 150
total    = 1149

baseline (TB-18 Atom B-impl ship 15b662c) = 963 / 0 / 150
net delta (R1 only)                      = +35 / 0 / 0   ← passes SG-18R.11 (≥+25 target)
```

**SG-18R.11 PASS**: `cargo test --workspace` is canonical (per `feedback_workspace_test_canonical`); +35 net passed exceeds the ≥+25 target.

### §1.1 The single failure is pre-existing on main, not caused by R1

```
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured;
                     0 filtered out; finished in 0.00s
  test comprehensive_arena_plan_only_emits_plan ... FAILED
  panic: ARENA_PLAN.md missing at "/tmp/tb16_arena_smoke_<pid>/ARENA_PLAN.md"
```

Verified pre-existing by running `cargo test -p minif2f_v4 --test tb_16_comprehensive_arena_smoke` on main worktree (commit `46d79ca`): same FAILED. Belongs to a separate cleanup TB; not in R1 scope.

A second worktree-specific failure (`rebuild_autopsy_event_counts_returns_empty_on_pre_tb15_chain` in `tests/tb_16_dashboard_live_regen.rs`) was caused by missing runtime-generated fixture files (`pinned_pubkeys.json` + git-internal CAS objects) that exist in main's working tree but are not git-tracked. Confirmed environmental-only by symlinking the fixture from main; test then passes. Will pass automatically on main post-merge.

## §2 R1 diff manifest (Class 4 surfaces only)

| File | Status | LoC | Class-4 ownership |
|---|---|---|---|
| `src/runtime/attempt_telemetry.rs` | NEW | ~720 | YES — typed-tx schema + canonical encoding |
| `src/bottom_white/cas/schema.rs` | MOD (additive) | +170 | YES — `ObjectType` enum tail-append |
| `src/runtime/mod.rs` | MOD (additive) | +3 | NO — module declaration only |
| `genesis_payload.toml` | MOD (rehash) | 2 lines | YES — Trust Root rehash for 2 modified files |
| `tests/tb_18r_attempt_telemetry_serialize.rs` | NEW | ~140 | NO — integration test |
| `tests/tb_18r_lean_result_cas_resolves.rs` | NEW | ~135 | NO — integration test |
| `tests/tb_18r_no_raw_response_in_attempt_payload.rs` | NEW | ~150 | NO — integration test |
| `tests/tb_18r_attempt_chain_root_payload_schema.rs` | NEW | ~165 | NO — integration test |

**No mutation of**:
- `src/state/typed_tx.rs` (per Design B, preflight §3 + Codex Q8: `attempt_chain_root` is a field on the new `AttemptTelemetry` struct, NOT on `WorkTx`)
- `src/state/sequencer.rs` (R3 scope)
- `src/bottom_white/ledger/rejection_evidence.rs` (R3 scope; tail-append of LeanFailed=6/ParseFailed=7/SorryBlocked=8/LlmError=9)
- `experiments/minif2f_v4/src/bin/evaluator.rs` (R2 scope)

## §3 Charter v2 mapping (Codex Gate 1 remediations)

| Codex Q | Remediation in R1 | Test witness |
|---|---|---|
| Q1 (R4 Class 4 inconsistency) | R1 doesn't touch R4 directly; R4-impl atom is the next item | n/a |
| Q3 (candidate_payload_cid privacy) | `AttemptTelemetry.candidate_payload_cid` doc-comment FORBIDDEN list mirroring `ProposalTelemetry::ToolCallRecord` precedent | `tb_18r_no_raw_response_in_attempt_payload.rs` (4 fence tests: parsed-Lean-pass, raw-Anthropic-shape-fail, raw-OpenAI-shape-fail, end-to-end-fence-in-action) |
| Q5 (schema_version + AttemptKind/AttemptOutcome separation) | `schema_version: u32 = 1`; `AttemptKind` tail-extensible enum (`ExternalizedLlmCycle=0`, reserved `Tactic=1`, `ExternalToolCall=2`); `AttemptOutcome` separate enum (`LeanPass=0..Aborted=5`) | `attempt_kind_repr_stable` + `attempt_outcome_repr_stable` + `outcome_distinct_from_kind_in_canonical_encoding` + `schema_version_starts_at_one` |
| Q8 (a) (`attempt_chain_root` Class 4 ownership) | Field placed on `AttemptTelemetry` (new struct) NOT on `WorkTx` (preserves WorkTx canonical wire bytes per Design B) | `attempt_chain_root_some_only_for_terminal_composite` + `tb_18r_attempt_chain_root_payload_schema.rs` (5 tests) |
| Q8 (b) (LeanFailed-already-exists wrong claim) | `LeanErrorClass` enum mirrors target R3 RejectionClass tail-append values 6..9; R3 will tail-append per Codex source-grounded finding (current enum: PredicateFailed=0..InsufficientBalance=5) | `lean_error_class_repr_mirrors_r3_rejection_class_tail_append` |
| Q8 (c) (rejection_evidence.rs missing from R3 surface) | charter v2 already updated R3 surface; R1 doesn't touch this file | n/a (R3 scope) |

## §4 Unit-test breakdown

`src/runtime/attempt_telemetry.rs` unit tests (12):
- `attempt_kind_repr_stable`
- `attempt_outcome_repr_stable`
- `lean_error_class_repr_mirrors_r3_rejection_class_tail_append`
- `attempt_telemetry_canonical_encode_deterministic`
- `attempt_telemetry_canonical_round_trip`
- `attempt_telemetry_cas_round_trip`
- `lean_result_canonical_round_trip`
- `lean_result_cas_round_trip`
- `terminal_abort_record_canonical_round_trip`
- `terminal_abort_record_cas_round_trip`
- `attempt_chain_root_some_only_for_terminal_composite`
- `schema_version_starts_at_one`
- `lean_result_shielded_stderr_cid_is_cid_not_bytes`
- `outcome_distinct_from_kind_in_canonical_encoding`

`src/bottom_white/cas/schema.rs` ObjectType unit tests (4 NEW; 6 pre-existing unchanged):
- `object_type_attempt_telemetry_canonical_hash_distinct`
- `object_type_lean_result_canonical_hash_distinct`
- `object_type_terminal_abort_record_canonical_hash_distinct`
- `object_type_pre_tb_18r_variants_unchanged` (pins all 14 pre-existing variants byte-identical)

Integration tests (4 files, ~15 tests total):
- `tb_18r_attempt_telemetry_serialize.rs` — 3 tests: round-trip via CAS, failure-path outcomes round-trip, idempotent CID
- `tb_18r_lean_result_cas_resolves.rs` — 3 tests: verified-pass round-trip, failure-path with shielded stderr, all 4 error classes round-trip
- `tb_18r_no_raw_response_in_attempt_payload.rs` — 4 tests: fence passes for Lean candidate, rejects raw Anthropic envelope, rejects raw OpenAI envelope, end-to-end fence in action
- `tb_18r_attempt_chain_root_payload_schema.rs` — 5 tests: intermediate=None, terminal=Some, CAS round-trip, deterministic over IDs, pre-TB-18R WorkTx canonical unchanged

## §5 Privacy invariant enforcement (CR-18R.4 v2)

**Schema-level enforcement** (compile-time):
- `LeanResult.stderr_cid: Option<Cid>` and `stdout_cid: Option<Cid>` — NOT `Option<Vec<u8>>`. Type system prevents inline raw bytes.

**Documentation-level enforcement**:
- Module-level FORBIDDEN list in `attempt_telemetry.rs` mirroring TB-7 `ProposalTelemetry::ToolCallRecord` precedent.

**Test-level enforcement**:
- `tb_18r_no_raw_response_in_attempt_payload.rs::looks_like_parsed_candidate` — structural fence detecting JSON envelopes with role/content/thinking/reasoning markers. R2 evaluator hot path will use the same check pre-CAS-write.

## §6 Hard-sequencing rules respected (charter v2 §2)

- `R0 before G1` ✅ (charter v2 + Codex Gate 1 audit committed first)
- `G1 before R1` ✅ (Codex Gate 1 verdict CHALLENGE-but-ship-clean; 7 remediations applied; user "go" granted 2026-05-06)
- `R1 before R2` ✅ (R1 schema lands first; R2 R3 R4 R5 not yet implemented)

## §7 Recommended merge process (STEP_B)

1. User reviews this ship report + the worktree diff:
   ```
   cd .claude/worktrees/stepb-tb18r-r1-schema
   git diff main -- src/ tests/ genesis_payload.toml
   ```
2. User approves merge.
3. Merge to main:
   ```
   cd /home/zephryj/projects/turingosv4
   git merge --no-ff stepb-tb18r-r1-schema -m "TB-18R R1 SHIPPED — AttemptTelemetry + LeanResult + TerminalAbortRecord schemas (Class 4)"
   ```
4. Post-merge: cargo test --workspace on main should produce 998/0/150 (the 1 pre-existing comprehensive_arena failure remains; R1's source diff doesn't change that test's status).

## §8 Forward triggers (NOT R1 scope)

- **R2** evaluator hot-path wire-up (per-iteration loop externalization at all 6 paths in `experiments/minif2f_v4/src/bin/evaluator.rs`)
- **R3** sequencer L4.E admission expansion (`src/state/sequencer.rs` + `src/bottom_white/ledger/rejection_evidence.rs` RejectionClass tail-append LeanFailed=6/ParseFailed=7/SorryBlocked=8/LlmError=9)
- **R4** chain_derived_run_facts exact equation + drain barrier (Class 4 G1-ratified; STEP_B)
- **R5** audit_tape sampler extension
- **R6/R7** evidence reruns (P23/P38/P49 + M0)
- **G2** Codex + Gemini ship audit covering R0..R7
- **Ship**: TB-18R SHIPPED FINAL gate; unblocks TB-18-resume

## §9 Sign-off

R1 is implementation-complete on the `stepb-tb18r-r1-schema` worktree branch. SG-18R.11 ship gate met (+35 net passed). Awaits user review + merge approval per `feedback_step_b_protocol`.
