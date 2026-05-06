# TB-18R R5 Preflight — Audit-Tape Sampler + Tamper Extension to AttemptTelemetry / LeanResult

**Atom**: TB-18R R5
**Class**: **3** (audit_assertions.rs additive; no STEP_B-restricted file).
Per charter §0.A: "Atom R5 (`src/runtime/audit_assertions.rs` or
equivalent) — **NOT restricted**; direct edit OK."
**STEP_B_PROTOCOL**: **does NOT apply**. Direct edit on `main`.
**Date**: 2026-05-06
**Author**: Claude orchestrator (post-R4 SHIPPED, autonomous mode per
user 2026-05-06 "自主执行直到本TB ship").
**Predecessor**: TB-18R R4 SHIPPED 2026-05-06 (commit `d34f428` via
merge `41aae74`; main HEAD `ae7681f`; workspace 1038/1/150).
**Charter**: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md` §2 R5
+ §1.2 FR-18R.6 + FR-18R.7 + FR-18R.8 + FR-18R.9 + §1.4 SG-18R.6 +
SG-18R.7 + SG-18R.8 + SG-18R.9.

---

## §1 Scope

### §1.1 In scope (R5 atomic surface)

- **`src/runtime/audit_assertions.rs`** (Class 3):
  - **`assert_44_attempt_telemetry_retrievable_from_cas`** — walk CAS
    objects of type `AttemptTelemetry`; verify each is
    canonical-decodable + `candidate_payload_cid` resolves.
  - **`assert_45_lean_result_retrievable_from_cas`** — walk CAS
    objects of type `LeanResult`; verify each is canonical-decodable +
    referenced from a non-zero count of AttemptTelemetry objects.
  - **`assert_46_attempt_chain_root_schema_well_formed`** — verify the
    R1 schema invariant: every AttemptTelemetry has a well-typed
    `attempt_chain_root` field (`[u8; 32]`); zero-bytes admissible for
    R3-amended omega path (per R3 preflight §3.5).
  - **`assert_47_random_attempt_payload_tamper_detected`** — Layer H
    stub (exercised by `audit_tape_tamper` binary per existing
    assert_36..38 precedent).
  - **`assert_48_random_lean_stderr_tamper_detected`** — Layer H stub.
  - Wire all 5 into `run_all_assertions` battery.

- **Tests** (charter §1.4 binding test names):
  - `tests/tb_18r_audit_sampler_attempt_payload.rs` — exercises
    assert_44 + assert_45.
  - `tests/tb_18r_audit_lean_stderr_tamper_detected.rs` — exercises
    Layer H tamper pattern (`audit_tape_tamper` extension).
  - `tests/tb_18r_final_composite_attempt_chain_root.rs` — schema
    validity for R1 attempt_chain_root field.
  - `tests/tb_18r_markov_failure_cluster_from_chain.rs` — confirms
    AttemptTelemetry CAS path is markov-cluster source-eligible (per
    FR-18R.6 verification at the type-system level).
  - `tests/tb_18r_dashboard_attempt_dag_replay.rs` — confirms
    `audit_dashboard` binary runs successfully on a TB-18R-shape chain
    (R6 evidence-run dependency satisfied at smoke level).

### §1.2 Out of scope (forwarded to OBS / future TB)

- **SG-18R.9 full dashboard DAG render** — adding a §17 dashboard
  section that renders the attempt DAG (accepted state nodes +
  rejection evidence nodes + golden path + failed branches) is a
  larger refactor of `audit_dashboard.rs`. R5 closes the binding
  shape (a smoke-level test that the binary runs on TB-18R-shape
  chain); the full-render refinement is filed to
  `handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md`
  with G2 forward-binding.
- **FR-18R.6 markov cluster source change** at runtime — the existing
  TB-15 `cluster_autopsies` works on `AgentAutopsyCapsule` (which is
  itself derived from chain). With AttemptTelemetry now populated
  per R2/R3, future markov capsule generation can read AttemptTelemetry
  outcome distribution as a cluster source. R5 verifies the path
  exists via the test; full re-wire (changing
  `markov_capsule::generate` to read AttemptTelemetry CAS directly)
  is forward-bound to a future TB.

## §2 Design

### §2.1 Sampler design (assert_44 / assert_45)

Walk the `LoadedTape.cas` `BTreeMap<Cid, CasObjectMetadata>` index
filtered by `object_type ∈ {AttemptTelemetry, LeanResult}`. For each:

- **AttemptTelemetry**: `read_attempt_telemetry_from_cas`; assert
  `candidate_payload_cid` resolves via `cas.get` (returns Ok). Per
  CR-18R.4 v2: bytes are parsed-candidate-only; assertion does NOT
  inspect contents (privacy fence preserved).
- **LeanResult**: `read_lean_result_from_cas`; assert exit_code is
  consistent with verified flag (verified=true ↔ exit_code=0).

Empty-tape (no AttemptTelemetry or LeanResult): SKIPPED (not a failure).

### §2.2 Tamper assertions (Layer H stubs; assert_47 / assert_48)

Follow existing `assert_36`..`assert_38` pattern: return
`AssertionResult::skipped` with a doc-comment pointing at the
`audit_tape_tamper` binary that exercises the actual tamper run. The
test file `tests/tb_18r_audit_lean_stderr_tamper_detected.rs` runs the
tamper binary against a fixture and asserts the verdict includes the
tamper-detected positive signal.

The tamper test surface is built on the existing TB-16 audit_tape_tamper
binary (`src/bin/audit_tape_tamper.rs`); R5 adds two new tamper modes:

  - `attempt_payload`: flip a random byte in a CAS-stored
    `AttemptTelemetry.candidate_payload_cid` blob; assert audit
    detects via Cid mismatch.
  - `lean_stderr`: flip a random byte in a CAS-stored `LeanResult`
    or its stderr blob; assert detection.

(Implementation note: existing `audit_tape_tamper.rs` already exercises
generic CAS-flip-detection per assert_37. R5 piggy-backs by adding
two AttemptTelemetry/LeanResult fixture classes to the binary's
tamper-targets enum.)

### §2.3 attempt_chain_root schema test (assert_46)

Per R3 §3.5 amended: omega-path WorkTx.proposal_cid stays as
ProposalTelemetry CID (no cutover). Therefore final-composite
`attempt_chain_root` is not yet wired into the omega flow at R5 time.
R5 schema test verifies:

  - The `attempt_chain_root: Hash` field exists on AttemptTelemetry.
  - Encoded canonical bytes round-trip through serialize/deserialize.
  - Zero-bytes default is admissible.

Full attempt-chain-root population (computing the Merkle root over
constituent attempt_ids) is forward-binding for a future TB when the
omega path uses AttemptTelemetry as proposal_cid (currently TB-7 audit
backward-compat constraint blocks the cutover).

### §2.4 markov failure cluster verification (FR-18R.6)

Light test: confirm that AttemptTelemetry CAS objects with `outcome ∈
{LeanFail, ParseFail, SorryBlock, LlmErr}` exist on disk for a
TB-18R-shape run, and that their outcome discriminator is type-safe
input for a future markov clustering function. The test asserts the
CAS path + outcome enum coverage; the full re-wire of markov generator
to read this path is OBS-deferred.

## §3 Files touched

| File | Status | Class | Diff |
|---|---|---|---|
| `src/runtime/audit_assertions.rs` | MOD (additive) | 3 | +5 pub fns (assert_44..48) + battery wiring |
| `src/runtime/attempt_telemetry.rs` | MOD (read-only API only; no schema change) | 3 | n/a (R1 schema unchanged) |
| `src/bin/audit_tape_tamper.rs` | MOD (additive) | 3 | +2 tamper modes (attempt_payload + lean_stderr) |
| `tests/tb_18r_audit_sampler_attempt_payload.rs` | NEW | 3 witness | 2 tests |
| `tests/tb_18r_audit_lean_stderr_tamper_detected.rs` | NEW | 3 witness | 2 tests |
| `tests/tb_18r_final_composite_attempt_chain_root.rs` | NEW | 3 witness | 2 tests |
| `tests/tb_18r_markov_failure_cluster_from_chain.rs` | NEW | 3 witness | 2 tests |
| `tests/tb_18r_dashboard_attempt_dag_replay.rs` | NEW | 3 witness | 1 test |
| `handover/alignment/OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06.md` | NEW | 0 | OBS forward-binding |
| `genesis_payload.toml` | MOD (rehash if audit_assertions pinned) | TR pin | check + update if needed |

Net delta target: ≥ +6 net (3 mandatory R5 charter SGs × 2 tests min);
charter SG-18R.11 cumulative target unchanged (R4 already at +49 vs
baseline 963).

## §4 Forbidden in R5

- Modify `WorkTx` canonical wire bytes (R1 Design B inherited).
- Modify R1 schema (typed_tx.rs / cas/schema.rs / attempt_telemetry.rs
  schema; only read-only API consumption).
- Modify R3 sequencer admission semantics.
- Modify R4 invariant equation or chain_derived_run_facts new fields.
- Pollute `tool_dist` source-of-truth (CR-18R.6).
- Add new TypedTx variant.
- Compute attempt_chain_root over actual omega-path WorkTx (deferred
  pending omega cutover; R3 §3.5 amended constraint).
- Mutate predicate registry from autopsy / cluster output (CR-18R.9).
- Retroactive M1 evidence rewrite (CR-18R.1).

## §5 Iteration cap

Charter §0.A R5 row: 24h post-Gate-1. Gate 1 closed 2026-05-06.

## §6 Sign-off

Direct edit on `main`. No STEP_B preflight gate. User authorization:
2026-05-06 "自主执行直到本TB ship".

**End of R5 preflight. Implementation begins immediately.**
