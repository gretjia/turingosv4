# TB-18R R2 STEP_B-style Preflight — Evaluator Per-LLM-Call Externalization

**Atom**: TB-18R R2
**Class**: 3 (`experiments/minif2f_v4/src/bin/evaluator.rs` is NOT in CLAUDE.md restricted file list; direct-edit on `main` permitted)
**STEP_B_PROTOCOL**: NOT mechanically required, but this preflight follows the STEP_B template per user "复活咒语" 2026-05-06 because R2 is the wire-up that makes R1's CAS schema actually appear on disk in benchmark runs (the very surface the M1 VETO fingered).
**Date**: 2026-05-06
**Author**: Claude orchestrator (post-R1 SHIP, post-user "go" on R2)
**Predecessor**: TB-18R R1 SHIPPED 2026-05-06 (commit `9f8ce1f` via merge `bbee847`); workspace 963→998 (+35 net).
**Charter**: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md` §2 atom table row R2 + §1.2 FR-18R.1 v2 + §1.3 CR-18R.4 v2 + §1.4 SG-18R.1.

---

## §1 Atom scope (binding per charter §2 R2 row + §1.2 FR-18R.1 + §1.4 SG-18R.1)

R2 wires every externalized LLM-Lean cycle in `experiments/minif2f_v4/src/bin/evaluator.rs` to produce an on-disk `AttemptTelemetry` CAS object via the R1-shipped helpers. Six call-sites (per VETO archive §C.1 / charter §0):

| # | path | evaluator.rs line | tool_dist key | outcome (AttemptOutcome) | candidate bytes | LeanResult written? |
|---|---|---|---|---|---|---|
| 1 | omega-full | 2317 | `omega_wtool` | `LeanPass` | `payload.as_bytes()` | yes (exit=0, verified=true, proof_artifact_cid=Some) |
| 2 | omega-pertactic | 2861 | `omega_wtool` | `LeanPass` | `tactic.as_bytes()` | yes (exit=0, verified=true, proof_artifact_cid=Some) |
| 3 | step_partial_ok | 3236 | `step_partial_ok` | `LeanPass` (intermediate) | `tactic.as_bytes()` | yes (exit=0, verified=false, proof_artifact_cid=None) |
| 4 | step_reject | 3263 | `step_reject` | `LeanFail` or `SorryBlock` | `tactic.as_bytes()` | yes (exit=1, verified=false, error_class=Some) |
| 5 | parse_fail | 3275 | `parse_fail` | `ParseFail` | `b"tb-18r-parse-fail-no-candidate"` sentinel | no (lean_result_cid=None) |
| 6 | llm_err | 3289 | `llm_err` | `LlmErr` | `b"tb-18r-llm-err-no-candidate"` sentinel | no (lean_result_cid=None) |

Outcome values per R1 doc-comment at `src/runtime/attempt_telemetry.rs` lines 136-167. Path-1 / Path-2 / Path-3 outcome=LeanPass aligned with R1 `attempt_telemetry.rs` line 137: "step_partial_ok → LeanPass with proof_artifact_cid = None if intermediate".

R2 produces (per externalized LLM-Lean cycle) **on disk** in CAS:
1. one `ObjectType::ProposalPayload` blob holding `parsed_candidate_bytes` (paths 1-4) or sentinel marker bytes (paths 5-6);
2. one `ObjectType::LeanResult` blob (paths 1-4 only);
3. one `ObjectType::AttemptTelemetry` blob (all 6 paths) carrying `candidate_payload_cid` + `lean_result_cid` references.

## §2 Out of scope for R2 (forwarded to R3 / R4 / R5 / R6 / R7)

- **Not in R2**: actual `WorkTx` submission with `proposal_cid` pointing at AttemptTelemetry. R2 keeps the existing TB-7 Atom-2/3 `ProposalTelemetry`-based `WorkTx` pipeline intact (still references `tel_cid` from `proposal_telemetry::write_to_cas`). The cutover of `WorkTx.proposal_cid` to AttemptTelemetry is **R3 scope** (admission expansion to L4.E for runtime-path WorkTx).
- **Not in R2**: routing failure-path attempts to L4.E. R3 expands sequencer admission + `RejectionClass` tail-append (LeanFailed=6 / ParseFailed=7 / SorryBlocked=8 / LlmError=9). R2 only writes CAS objects; chain-side routing happens in R3.
- **Not in R2**: `attempt_count_invariant()` ship-gate equation in `chain_derived_run_facts`. R4 scope.
- **Not in R2**: audit_tape sampler extension. R5 scope.
- **Not in R2**: P23/P38/P49 + M0 rerun evidence. R6/R7 scope.
- **Not in R2**: TerminalAbortRecord wire-up for budget-cap halts. R4 scope (the equation hides the abort accounting; R2 doesn't yet emit aborts on this path).
- **Not in R2**: `attempt_chain_root` Merkle root over constituent attempts on the final composite. R5 will compute and verify the Merkle. R2 sets `attempt_chain_root: None` on every AttemptTelemetry it writes (per R1 schema: `Some` only on terminal composite).

## §3 Design decisions

### §3.1 `prompt_context_hash` derivation without adding `sha2` to evaluator deps

R1 `AttemptTelemetry.prompt_context_hash: Hash` is a 32-byte SHA-256. The evaluator (`experiments/minif2f_v4/Cargo.toml`) intentionally has **no direct `sha2` dep** (per evaluator.rs line 41-49 comment: avoiding `sha2` to prevent `Cargo.lock` mutation that would trip the Trust Root gate; `genesis_payload.toml` is STEP_B-protected).

**Decision**: reuse the existing `turingosv4::bottom_white::cas::schema::Cid::from_content(bytes)` (already SHA-256-of-bytes) and **cast** the resulting 32-byte array into `Hash`:

```rust
let prompt_cid = turingosv4::bottom_white::cas::schema::Cid::from_content(prompt.as_bytes());
let prompt_ctx_hash = turingosv4::state::q_state::Hash(prompt_cid.0);
```

`Hash` and `Cid` are both `(pub [u8; 32])` newtypes; the bytes are byte-identical SHA-256 digests. This avoids:
  - adding `sha2` to `experiments/minif2f_v4/Cargo.toml` → no Cargo.lock churn → no Trust Root re-hash;
  - any post-R1 modification to `src/runtime/attempt_telemetry.rs`.

Trade-off: type-semantically the cast bridges Cid (content-address) ↔ Hash (general digest). Since both are SHA-256 of the same bytes, the cast is semantics-preserving for the prompt-context use case. The evaluator existing `prompt_hash_hex` (DefaultHasher 64-bit) stays unchanged; R2's stronger SHA-256 prompt_context_hash lives only on AttemptTelemetry.

### §3.2 `candidate_payload_cid` privacy invariant (CR-18R.4 v2 + Codex Q3)

Per R1 schema doc-comment + CR-18R.4: **NEVER raw LLM response**. R2 stores only:
  - paths 1-4: the **parsed** external candidate bytes (`payload` / `tactic` strings as parsed by `parse_agent_output`). These are the bytes actually sent to Lean via `Lean4Oracle::verify_omega_detailed` / `verify_partial`. Raw `response.content` is NOT stored.
  - paths 5-6: a **sentinel** marker (`b"tb-18r-parse-fail-no-candidate"` / `b"tb-18r-llm-err-no-candidate"`). The reason these paths cannot store the raw model output: parse_fail means the parser failed → no parsed candidate exists; llm_err means the LLM call itself failed → no response to parse. Storing the raw response would violate CR-18R.4 (privacy-leak path).

The R1 integration test `tb_18r_no_raw_response_in_attempt_payload.rs` verifies the structural fence; R2 inherits the fence (it does not weaken it). R2 adds NO new bytes that resemble raw LLM JSON to CAS.

### §3.3 `attempt_id` minting

Per R1 doc-comment line 226-228: "Same TxId used on the `WorkTx.tx_id` if the attempt routes to L4 accepted, or `RejectedSubmissionRecord.submit_id` if it routes to L4.E."

  - Paths 1-2 (omega-full / omega-pertactic): R2 mints `attempt_id` to **match** the WorkTx tx_id minted by the existing TB-7 Atom-3 path. Specifically: `worktx-{task_id_str}-{suffix}` where `suffix = "omega-full-{proposal_count}"` or `"omega-pertactic-{proposal_count}"`. The AttemptTelemetry is written **before** the WorkTx submit (so the CAS object exists by the time the chain references the proposal_cid; deferred to R3).
  - Paths 3-6 (step_partial_ok / step_reject / parse_fail / llm_err): no WorkTx in current pipeline (R3 will add). R2 mints `att-{run_id}-{agent_id}-{proposal_count}-{path_label}`. R3 will later rebind these to `RejectedSubmissionRecord.submit_id` at the admission boundary. The `att-` prefix avoids collision with `worktx-` prefix.

### §3.4 `lean_result.exit_code` convention

Lean's actual exit code is not exposed by the current `Lean4Oracle::verify_omega_detailed` / `verify_partial` signatures (they return `Result<(bool, String), ...>` / `PartialVerdict`, not `i32`). R2 uses a **convention** for the LeanResult:
  - paths 1-2 (verdict accepts): `exit_code = 0`, `verified = true`, `proof_artifact_cid = Some(persist_proof_artifact_cid)`.
  - path 3 (PartialOk): `exit_code = 0` (Lean did not error), `verified = false` (no Complete verdict), `proof_artifact_cid = None`, `error_class = None`.
  - path 4 (Reject): `exit_code = 1` (placeholder non-zero; actual exit not exposed), `verified = false`, `error_class = Some(LeanFailed or SorryBlocked)`.

Capturing actual stderr/stdout bytes into CAS is **NOT R2 scope** (would require modifying `Lean4Oracle` to expose them); `LeanResult.stderr_cid` / `stdout_cid` are set to `None`. This is acceptable per R1 schema (both fields are `Option<Cid>`). A later TB (or R5+ extension) may capture stderr/stdout bytes into CAS for `audit_tape.sample_lean_stderr_tamper_detected` (FR-18R.7).

### §3.5 `error_class` derivation for step_reject (path 4)

The existing evaluator code at line 3256 already distinguishes sorry-block from generic Lean error:
```rust
if reason.contains("sorry") || reason.contains("forbidden_payload") {
    tb11_sorry_block_count += 1;
} else {
    tb11_lean_error_count += 1;
}
```

R2 mirrors this directly into AttemptOutcome / LeanErrorClass:
```rust
let (lec, outcome) = if reason.contains("sorry") || reason.contains("forbidden_payload") {
    (LeanErrorClass::SorryBlocked, AttemptOutcome::SorryBlock)
} else {
    (LeanErrorClass::LeanFailed, AttemptOutcome::LeanFail)
};
```

### §3.6 Fail-close vs warn-only for AttemptTelemetry CAS write failure

The TB-7 Atom-2/3 paths are FAIL-CLOSED on `cas_store.put` / `submit_typed_tx` failure (`std::process::exit(3)`). For R2 AttemptTelemetry writes:

  - **Chaintape mode** (`chaintape_bundle.is_some() && agent_keypairs.is_some()`): FAIL-CLOSED. Per FR-18R.1 + SG-18R.1 "every externalized LLM-Lean cycle produces a CAS AttemptTelemetry object". A silent skip would re-introduce the very failure-path asymmetry that triggered the M1 VETO. Exit code 3 + structured error log mirroring TB-7 pattern.
  - **Legacy mode** (no chaintape_bundle): silent skip. Per charter §6 FREEZE list, "Any ship doc citing M1 SOLVED counts as benchmark evidence BLOCKED" — pre-chaintape mode is already disallowed for benchmark; smoke tests in this mode never hit M-ladder. Skipping AttemptTelemetry there does not reopen the asymmetry.

### §3.8 Trust Root manifest rehash for `evaluator.rs`

The Trust Root manifest at `genesis_payload.toml` pins a SHA-256 of `experiments/minif2f_v4/src/bin/evaluator.rs` (current pin: `7d70c2f0...`, set by TB-18 Atom B Phase 2 2026-05-05). Any modification to evaluator.rs — including R2's pure-additive instrumentation — changes the file SHA and trips the Trust Root invariant via three test sites:
  - `experiments/minif2f_v4/tests/trust_root_immutability.rs::test_trust_root_immutable_at_boot`
  - `src/boot.rs::tests::verify_trust_root_passes_on_intact_repo`
  - `tests/fc_alignment_conformance.rs::fc3_n34_readonly_guard_verify_trust_root_intact_repo`

R2 follows the established TB-18 Atom B routine-rehash pattern (precedents at the same line in `genesis_payload.toml`): the new SHA is computed and pinned in the same R2 commit, with a comment-chain entry citing this preflight + the affected paths. Per CLAUDE.md current STEP_B restricted-file list (kernel.rs / bus.rs / wallet.rs / sequencer.rs / typed_tx.rs / cas/schema.rs), `genesis_payload.toml` is NOT itself STEP_B-restricted — only the SCHEMA of the manifest (adding/removing pins, restructuring) requires special ratification; routine rehash for an already-listed file follows the code change.

### §3.7 `tool_name` field on AttemptTelemetry

Mirror the existing `tool_dist` keys at the increment lines (per R1 doc-comment at attempt_telemetry.rs line 261-262). For paths 1-2 the existing `tool_dist` key is `"omega_wtool"` for both — R2 disambiguates on AttemptTelemetry via `tool_name`:
  - path 1: `"omega_wtool"`
  - path 2: `"omega_wtool_pertactic"`

Path 3-6 keep the same string as `tool_dist` (`"step_partial_ok"` / `"step_reject"` / `"parse_fail"` / `"llm_err"`).

## §4 Privacy invariant (carried over from R1; CR-18R.4 v2)

Reaffirmed: NEITHER raw LLM response NOR private CoT is stored in `candidate_payload_cid`. The 6-path mapping in §1 + sentinel-only design in §3.2 make this structural, not procedural. No new state surface in R2 stores raw model JSON.

The R1 integration test `tb_18r_no_raw_response_in_attempt_payload.rs` already fences the structural shape; R2 does not modify that test (the fence applies module-wide). R2 adds 6 new path-specific tests (§6) that each construct an AttemptTelemetry with the correct candidate-payload shape per path.

## §5 Files touched (canonical list)

| File | Status | Diff intent |
|---|---|---|
| `experiments/minif2f_v4/src/bin/evaluator.rs` | MOD (additive) | Insert AttemptTelemetry write at 6 sites (lines ~2317 / ~2861 / ~3236 / ~3263 / ~3275 / ~3289) + new file-scope `R2AttemptArgs` struct + `r2_write_attempt_telemetry` helper + `r2_prompt_ctx_hash` per-iteration SHA-256 derivation. Each insert is local; no existing code path removed. |
| `experiments/minif2f_v4/Cargo.toml` | NO MOD | No new deps (per §3.1 Cid-cast trick) |
| `genesis_payload.toml` | MOD (rehash; line ~164) | Routine evaluator.rs SHA rehash per §3.8. Predecessor TB-18 Atom B Phase 2 hash `7d70c2f0…` superseded; comment-chain entry cites this preflight. |
| `tests/tb_18r_attempt_telemetry_per_llm_call.rs` | NEW | 6 unit-style tests covering each path's AttemptTelemetry shape using the R1 `write_attempt_telemetry_to_cas` helper directly (NOT spinning up a full evaluator binary) |
| `src/runtime/attempt_telemetry.rs` | NO MOD | R1 schema unchanged |
| `src/bottom_white/cas/schema.rs` | NO MOD | R1 ObjectType variants unchanged |
| `src/state/typed_tx.rs` | NO MOD | (Design B from R1: WorkTx wire bytes unchanged) |
| `src/state/sequencer.rs` | NO MOD | R3 scope |
| `src/runtime/chain_derived_run_facts.rs` | NO MOD | R4 scope |

Net delta target: ≥ +6 integration tests (one per path) → workspace 998 → ≥ 1004. Plus unit tests inside `evaluator.rs` (path-shape sanity) if needed.

## §6 Test plan — `tests/tb_18r_attempt_telemetry_per_llm_call.rs`

Per SG-18R.1 charter mandate ("Every externalized LLM-Lean cycle produces a CAS AttemptTelemetry object. Test: `tb_18r_attempt_telemetry_per_llm_call.rs` covering all 6 paths").

**Strategy**: each test constructs the `AttemptEnvelope` + `LeanResult` (where applicable) + `AttemptTelemetry` shape that R2 produces for that path, writes it via `write_attempt_telemetry_to_cas` / `write_lean_result_to_cas`, then reads back and asserts the round-trip preserves the per-path field set.

Tests do NOT spin up a full evaluator process (would require LLM API + Lean toolchain; out-of-scope for unit/integration test layer). The tests verify the schema shape contract that R2 implements; the actual evaluator wire-up is verified by R6/R7 evidence runs (P23/P38/P49 reruns + M0 small batch produce on-disk CAS objects with the same shape).

| Test fn | Path | Asserts |
|---|---|---|
| `omega_wtool_full_path_attempt_telemetry_shape` | 1 | outcome=LeanPass, tool_name="omega_wtool", lean_result.exit_code=0, lean_result.verified=true, proof_artifact_cid=Some, candidate_payload_cid resolves to non-sentinel bytes, attempt_chain_root=None |
| `omega_wtool_pertactic_path_attempt_telemetry_shape` | 2 | outcome=LeanPass, tool_name="omega_wtool_pertactic", lean_result.exit_code=0, lean_result.verified=true, proof_artifact_cid=Some, candidate_payload_cid resolves, attempt_chain_root=None |
| `step_partial_ok_path_attempt_telemetry_shape` | 3 | outcome=LeanPass, tool_name="step_partial_ok", lean_result.exit_code=0, lean_result.verified=false, proof_artifact_cid=None, error_class=None |
| `step_reject_lean_failed_path_attempt_telemetry_shape` | 4a | outcome=LeanFail, tool_name="step_reject", lean_result.exit_code=1, lean_result.verified=false, error_class=Some(LeanFailed) |
| `step_reject_sorry_block_path_attempt_telemetry_shape` | 4b | outcome=SorryBlock, tool_name="step_reject", lean_result.exit_code=1, error_class=Some(SorryBlocked) |
| `parse_fail_path_attempt_telemetry_shape` | 5 | outcome=ParseFail, tool_name="parse_fail", lean_result_cid=None, candidate_payload_cid resolves to `b"tb-18r-parse-fail-no-candidate"` sentinel |
| `llm_err_path_attempt_telemetry_shape` | 6 | outcome=LlmErr, tool_name="llm_err", lean_result_cid=None, candidate_payload_cid resolves to `b"tb-18r-llm-err-no-candidate"` sentinel |

**Note**: 7 test fns to cover the 6 paths because step_reject splits sorry-block from generic Lean fail (matching R1 LeanErrorClass discriminator + the existing evaluator branching at line 3256). Charter SG-18R.1 says "all 6 paths" — the 7-test count is an honest more-coverage. Net delta still ≥ +6.

## §7 Risk matrix

| Risk | Probability | Mitigation |
|---|---|---|
| Adding direct `sha2` dep to evaluator → Cargo.lock churn → Trust Root gate trip | Mitigated by §3.1 Cid-cast | n/a (decision avoids dep) |
| AttemptTelemetry CAS write fails silently in chaintape mode → reintroduce M1 VETO defect | Low | §3.6 FAIL-CLOSED in chaintape mode |
| `attempt_id` collision between R2's mint scheme and existing TB-7 worktx-tx_id space | Low | §3.3 prefixes (`worktx-` for TB-7 success paths inherited; `att-` for failure paths only) — disjoint prefix sets |
| Future tx_id format change in TB-7 breaks AttemptTelemetry.attempt_id consistency | Medium (governance, not impl) | R3 will make this contract explicit at the admission boundary; R2 just preserves the existing TB-7 tx_id format |
| Path 5/6 sentinel bytes look like raw model response to a future audit | Low (sentinel is fixed ASCII tag) | §3.2 sentinel string is structural marker, not LLM-derived; R5 audit fence test uses these as KNOWN-good shapes |
| `cargo test --workspace` regression on existing 998 baseline | Low (additive only) | §9 cargo check + cargo test --workspace before any commit; new tests only |
| Cycle: AttemptTelemetry refers to ProposalTelemetry CID, but ProposalTelemetry.proposal_artifact_cid points at the same payload — double CAS-put of identical bytes | Low (CAS is content-addressed; idempotent put → same CID) | The R2 `cas_store.put(parsed_candidate_bytes, ObjectType::ProposalPayload, ...)` produces the same Cid as the existing `proposal_artifact_cid` from `ProposalTelemetry::build_for_evaluator_append_with_parent`. Same bytes → same Cid. Test asserts byte-equal. |
| Per-path mint of AttemptTelemetry adds 6 additional CAS writes per LLM-Lean cycle → I/O overhead | Low for M0 (≤ 20 problems × 32 attempts × 3 puts = ~2k ops; CAS is ~100µs/put on local disk) | Not a perf concern at M-ladder scale; R7 evidence will report wall-clock |

## §8 STEP_B-style (advisory) parallel-branch plan

Since R2 is NOT in CLAUDE.md restricted file list, `main` direct edit is permitted per charter §0.A `STEP_B_PROTOCOL flag` row 2 ("Atom R2 — NOT in STEP_B restricted set; direct edit OK"). However, this preflight follows STEP_B-style discipline:

1. **Phase 0** (this doc): scope + design + risks + tests planned **before** any code edit.
2. **Phase 1**: implement on `main` (no worktree). cargo check + cargo test --workspace must be green before commit.
3. **Phase 2** (skipped): no A/B statistical test; R6/R7 evidence runs serve as the empirical test (per charter §2 atom table).
4. **Phase 3**: commit on `main` with message referencing this preflight.

**Iteration cap** (per charter §0.A iteration cap row R2): **72h-to-feedback-loop**. Started 2026-05-06 R2 task launch; deadline 2026-05-09 R2 task launch + 72h.

## §9 Validation steps before R2 ship

1. `cargo check --workspace` — clean compile.
2. `cargo test --workspace` — report `command/workspace_count/failed/ignored` per `feedback_workspace_test_canonical`.
3. Visual diff review: confirm 6 insertion sites only; no removal of existing logic.
4. Smoke probe: run evaluator on **one** problem (e.g. P01_mathd_algebra_107 from tb_18 H0 preflight) in chaintape mode; assert CAS dir contains AttemptTelemetry / LeanResult / ProposalPayload object-type counts ≥ 6 (one per LLM call); per `feedback_smoke_before_batch`.
5. Commit on `main` referencing this preflight.
6. Report ship status to TB_LOG.tsv R2 row.

## §10 Forbidden in R2

- Touching `WorkTx` canonical wire bytes (Design B inherited from R1).
- Touching `Sequencer` dispatch logic (R3 scope).
- Touching `RejectionClass` enum (R3 scope).
- Touching `chain_derived_run_facts.rs` (R4 scope).
- Routing failed AttemptTelemetry to L4.E in this atom (R3 scope; R2 only writes CAS objects, no admission change).
- Computing `attempt_chain_root` Merkle (R5 scope; R2 sets `None` everywhere).
- Storing raw LLM responses in candidate_payload (CR-18R.4 v2; per §3.2).
- Adding `sha2` direct dep to `experiments/minif2f_v4/Cargo.toml` (per §3.1).
- Modifying any historical M1 evidence (per `feedback_no_retroactive_evidence_rewrite`; charter FR-18R.10 v2).
- ±N tolerance in any new field (per Codex Q4 ratified; charter FR-18R.3 v2).

## §11 Sign-off

This preflight grants implementation authority for R2 within the listed scope.
- Pre-impl: this doc filed under `handover/ai-direct/`.
- Post-impl: ship report appended to `TB_LOG.tsv` TB-18R row R2; commit on `main`.
- Pre-merge-to-main: no separate merge step (direct edit). User has already authorized R2 launch via 2026-05-06 "复活咒语".

**End of R2 preflight. Awaits implementation start.**
