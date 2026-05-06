# TB-18R R1 STEP_B Preflight — Class 4 Schema Additions

**Atom**: TB-18R R1
**Class**: 4 (typed-tx schema + CAS schema + canonical proposal-payload schema)
**STEP_B_PROTOCOL**: applies (parallel-branch worktree A/B; no direct edit on `main`)
**Date**: 2026-05-06
**Author**: Claude orchestrator (post-Codex Gate 1 v2 charter ratification)

## §1 Atom scope (binding per TB-18R charter v2 §1 + §2 atom table)

R1 produces three pure-additive schema artifacts:

1. **NEW** module `src/runtime/attempt_telemetry.rs` — defines:
   - `AttemptKind` enum (tail-extensible; TB-18R = `ExternalizedLlmCycle`; reserved `Tactic` / `ExternalToolCall` for TB-8+)
   - `AttemptOutcome` enum (separate from `AttemptKind`: `LeanPass` / `LeanFail` / `ParseFail` / `SorryBlock` / `LlmErr` / `Aborted`)
   - `AttemptTelemetry` struct (incl. `schema_version: u32`, `candidate_payload_cid: Cid` privacy invariant per CR-18R.4 v2, optional `attempt_chain_root: Option<Hash>` for final composite)
   - `LeanResult` struct (incl. `exit_code`, `verified`, `stderr_cid` shielded, `stdout_cid` shielded, `proof_artifact_cid` opt, `error_class`)
   - `LeanErrorClass` enum (LeanFailed / ParseFailed / SorryBlocked / LlmError; mirrors R3 `RejectionClass` tail-append values 6..9)
   - `AttemptEnvelope` struct (evaluator-side pre-Lean-check input; not all fields persisted to CAS; bridges R2 evaluator hot path → R1 CAS schema)
   - `TerminalAbortRecord` struct (per FR-18R.3 v2; explicit per-aborted-attempt record for drain-barrier accounting)
   - canonical encoding helpers (bincode v2 BE + fixed-int, mirroring `ProposalTelemetry::canonical_encode` precedent)

2. **MOD** `src/bottom_white/cas/schema.rs` — tail-append three `ObjectType` variants:
   - `AttemptTelemetry`
   - `LeanResult`
   - `TerminalAbortRecord`
   - (existing `Generic` variant remains tail-most for unknown blobs; serde-derived encoding so order matters less than the variant set)

3. **MOD** `src/state/typed_tx.rs` — re-export for downstream consumers; **NO** mutation of `WorkTx` canonical wire bytes (see §3 design decision below).

## §2 Out of scope for R1 (forwarded to R2 / R3 / R4 / R5)

- `bus.submit_typed_tx` calls in evaluator.rs hot path (R2)
- `RejectionClass` tail-append in `src/bottom_white/ledger/rejection_evidence.rs` (R3)
- L4.E admission for runtime-path WorkTx with new rejection_class variants (R3)
- `chain_derived_run_facts` exact equation + drain barrier (R4)
- `audit_tape` sampler extension (R5)
- Any per-iteration loop change to `experiments/minif2f_v4/src/bin/evaluator.rs` (R2)

## §3 Design decision — `attempt_chain_root` placement

**Codex Q8 found**: charter v1 placed `attempt_chain_root` payload schema in R5 (audit-only); should be R1 (Class 4 ratified).

**Two candidate designs**:

| | (A) `WorkTx.attempt_chain_root: Option<Hash>` field | (B) `AttemptTelemetry.attempt_chain_root: Option<Hash>` field |
|---|---|---|
| Backward compat | ❌ WorkTx canonical wire bytes change; sequencer admission needs migration | ✅ WorkTx unchanged; pre-TB-18R chains replay byte-identical |
| Schema purity | ✅ field on the chain-resident tx | ✅ field on the CAS-resident proposal preimage |
| Reconstructability | WorkTx → attempt_chain_root direct | WorkTx → proposal_cid → AttemptTelemetry → attempt_chain_root (one hop) |
| Constitutional fit | ⚠️ adds canonical bytes to a TB-1+ stable schema | ✅ pure-additive on a new struct |
| Codex Q4 drain-barrier compat | n/a | ✅ AttemptTelemetry already part of post-drain CAS reachability |

**Decision**: **Design (B)**.

`attempt_chain_root` is a field on the new `AttemptTelemetry` struct. When the AttemptTelemetry represents a final composite proof (the OMEGA-accept terminal attempt), it is `Some(merkle_root_over_constituent_attempt_ids)`. For intermediate attempts, it is `None`.

WorkTx schema is unchanged. Replay reconstruction: `WorkTx.proposal_cid → AttemptTelemetry CAS object → attempt_chain_root → constituent attempt_ids → individual AttemptTelemetry CAS objects`. Pre-TB-18R WorkTx entries continue to deserialize byte-identical (per `feedback_no_retroactive_evidence_rewrite`).

**This decision is binding for R1 and reflected in tests `tb_18r_attempt_chain_root_payload_schema.rs` + `tb_18r_attempt_telemetry_serialize.rs`.**

## §4 Privacy invariant (Codex Q3 ratified; CR-18R.4 v2)

`candidate_payload_cid: Cid` MUST point to **parsed external candidate bytes**, never raw LLM response containing private CoT.

**Implementation enforcement**:
- AttemptTelemetry struct includes a doc-comment FORBIDDEN list mirroring `ProposalTelemetry` precedent (proposal_telemetry.rs:80-88): "raw model deliberation, raw tool transcripts, internal reasoning, raw prompt/completion strings".
- Test `tb_18r_no_raw_response_in_attempt_payload.rs` constructs an AttemptTelemetry with a candidate_payload that resembles raw model JSON (`{"role": "assistant", "content": "...", "thinking": "..."}`) and asserts it FAILS a structural sanity check — the test verifies the NEGATIVE invariant.
- The actual evaluator-side enforcement lives in R2 (post-parse extraction); R1 ratifies the schema constraint as documentation + a test that demonstrates the structural fence.

## §5 Schema versioning (Codex Q5 ratified)

`AttemptTelemetry.schema_version: u32` initialized at `1` for TB-18R.

Rationale:
- Future TB (e.g. TB-8 per-tactic decomposition) may extend the schema.
- `schema_version` prevents silent semantic drift on replay.
- Tail-additive new fields use `#[serde(default)]` for forward compat at version 1; major schema-changing additions bump to 2.

`AttemptKind` enum is **tail-extensible** with reserved variants:
```rust
pub enum AttemptKind {
    /// TB-18R: 1 LLM call → 1 compound payload = 1 Attempt Node
    /// (per `feedback_chaintape_externalized_proposal`).
    ExternalizedLlmCycle = 0,
    /// **Reserved for TB-8+** (per-tactic decomposition; not used in TB-18R).
    Tactic = 1,
    /// **Reserved for TB-8+** (external tool calls beyond Lean).
    ExternalToolCall = 2,
}
```

`AttemptOutcome` enum kept separate from `AttemptKind`:
```rust
pub enum AttemptOutcome {
    LeanPass,
    LeanFail,
    ParseFail,
    SorryBlock,
    LlmErr,
    Aborted,
}
```

## §6 Canonical encoding strategy

Mirror `ProposalTelemetry` precedent (proposal_telemetry.rs):
- bincode v2 with BigEndian + fixed-int encoding (byte-stable across compiler / arch)
- `canonical_encode` / `canonical_decode` exposed publicly
- `ObjectType::AttemptTelemetry` + schema_id `turingosv4.attempt_telemetry.v1`
- `ObjectType::LeanResult` + schema_id `turingosv4.lean_result.v1`
- `ObjectType::TerminalAbortRecord` + schema_id `turingosv4.terminal_abort_record.v1`

## §7 Test coverage plan

Unit tests in `src/runtime/attempt_telemetry.rs`:
1. `attempt_kind_repr_stable` — discriminator values 0/1/2 byte-stable
2. `attempt_outcome_serde_round_trip` — all 6 variants
3. `attempt_telemetry_canonical_encode_deterministic` — encode-decode-encode equality
4. `lean_result_shielded_stderr_cid_present` — stderr_cid is Cid, not raw bytes
5. `terminal_abort_record_canonical_encode` — round-trip
6. `attempt_chain_root_some_only_for_composite` — None for intermediate, Some(merkle_root) for terminal

Integration tests under `tests/`:
1. `tb_18r_attempt_telemetry_serialize.rs` — full struct round-trip via canonical_encode + CAS put/get
2. `tb_18r_lean_result_cas_resolves.rs` — LeanResult written to CAS, resolved by Cid, byte-identical
3. `tb_18r_no_raw_response_in_attempt_payload.rs` — privacy invariant: structural fence catches raw-response shape
4. `tb_18r_attempt_chain_root_payload_schema.rs` — final composite AttemptTelemetry carries Some(merkle_root); intermediate carries None; reconstruct chain from CAS

ObjectType variant unit tests in `src/bottom_white/cas/schema.rs`:
- `object_type_attempt_telemetry_canonical_hash_distinct`
- `object_type_lean_result_canonical_hash_distinct`
- `object_type_terminal_abort_record_canonical_hash_distinct`
- `object_type_pre_tb_18r_variants_unchanged` — existing variants serialize byte-identical

## §8 Risk assessment

| Risk | Probability | Mitigation |
|---|---|---|
| WorkTx canonical wire bytes accidentally mutated | Low (Design B chosen) | Test `tb_18r_pre_tb_18r_worktx_canonical_unchanged.rs` constructs a pre-TB-18R-shape WorkTx and asserts byte-identical canonical_encode |
| ObjectType variant serialization drift | Low (serde-derived; tail-append) | Unit test `object_type_pre_tb_18r_variants_unchanged` asserts 13 pre-existing variants byte-identical |
| Privacy invariant accidentally bypassed in R2 | Medium (R2 implementer might serialize raw response to candidate_payload_cid) | (a) doc-comment FORBIDDEN list on AttemptTelemetry; (b) `tb_18r_no_raw_response_in_attempt_payload.rs` structural fence test; (c) R2 STEP_B preflight will require explicit reference to this fence |
| Schema version starts at 1 vs 0 | Low (off-by-one bikeshed) | Match `ProposalTelemetry.v1` precedent → schema_version=1 from start |
| AttemptTelemetry name collision with planned `feedback_chaintape_externalized_proposal` "Attempt Node" terminology | Low | Naming aligned: `AttemptTelemetry` is the CAS object; "Attempt Node" is the conceptual term in VETO archive §A.6 §3 |
| `cargo test --workspace` regression on existing 963 baseline | Low (pure additive; no existing logic touched) | Workspace test in worktree before any merge to main |

## §9 STEP_B parallel-branch plan

1. `git worktree add .claude/worktrees/tb-18r-r1-schema -b tb-18r-r1-schema main`
2. cd to worktree; implement R1; cargo check + cargo test --workspace
3. If green: report deltas to user; user reviews; merge to main on user approval
4. If red: iterate in worktree without polluting main

Worktree branch: `tb-18r-r1-schema` (off `main` at HEAD `46d79ca` + post-restart uncommitted state).

## §10 Files touched (canonical list)

| File | Status | Diff intent |
|---|---|---|
| `src/runtime/attempt_telemetry.rs` | NEW | Module: AttemptKind + AttemptOutcome + AttemptTelemetry + LeanResult + LeanErrorClass + AttemptEnvelope + TerminalAbortRecord + canonical encoding + 6 unit tests |
| `src/bottom_white/cas/schema.rs` | MOD (additive) | Tail-append 3 ObjectType variants + 4 unit tests |
| `src/runtime/mod.rs` | MOD (additive) | `pub mod attempt_telemetry;` |
| `src/state/typed_tx.rs` | NO MOD | (Design B; no WorkTx schema change) |
| `tests/tb_18r_attempt_telemetry_serialize.rs` | NEW | Integration: full round-trip |
| `tests/tb_18r_lean_result_cas_resolves.rs` | NEW | Integration: LeanResult CAS round-trip |
| `tests/tb_18r_no_raw_response_in_attempt_payload.rs` | NEW | Integration: privacy structural fence |
| `tests/tb_18r_attempt_chain_root_payload_schema.rs` | NEW | Integration: terminal-vs-intermediate `attempt_chain_root` |

Net delta target: ~+10 unit tests + 4 integration tests = ~+14 tests vs baseline 963.

## §11 Forbidden in R1

- Touching `WorkTx` canonical wire bytes (Design B chosen)
- Touching `Sequencer` dispatch logic (R3 scope)
- Touching `evaluator.rs` (R2 scope)
- Touching `RejectionClass` enum (R3 scope; uses `LeanErrorClass` mirror in R1 to communicate target values)
- Storing raw LLM responses anywhere in AttemptTelemetry (CR-18R.4 v2)
- ±N tolerance fields in any struct (per Codex Q4 ratified deterministic equation)

## §12 Sign-off

This preflight doc grants implementation authority for R1 within the listed scope.
- Pre-impl: this doc filed under `handover/ai-direct/`
- Post-impl: ship report appended to `TB_LOG.tsv` TB-18R row + R1 commit on worktree branch
- Pre-merge-to-main: user reviews this preflight + the worktree diff
