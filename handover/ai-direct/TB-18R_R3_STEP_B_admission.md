# TB-18R R3 STEP_B Preflight — Sequencer L4.E Admission Expansion + RejectionClass Tail-Append

**Atom**: TB-18R R3
**Class**: **4** (sequencer admission semantics + `RejectionClass` enum bump on the canonical L4.E hash domain). Per `feedback_class4_cannot_hide_in_class3`: "sequencer admission / typed-tx schema bumps / canonical-signing-payload changes need separate ratification". Codex Gate 1 (charter ratification 2026-05-06) authorizes the SHAPE of R3 per charter §0.A Q8 remediation; this preflight specifies the IMPLEMENTATION shape inside that ratified envelope and depends on user "go" per charter §1.1 Opening rule for Class-4 atoms.
**STEP_B_PROTOCOL**: **applies** (charter §0.A row 3). Parallel-branch worktree A/B; no direct edit on `main`. Worktree branch: `tb-18r-r3-admission` (off `main` at HEAD `aa08f50`).
**Date**: 2026-05-06
**Author**: Claude orchestrator (post-R1 + R2 SHIP, pre-user "go" on R3 implementation)
**Predecessor**: TB-18R R2 SHIPPED 2026-05-06 (commit `35389d0`; TB_LOG row at `aa08f50`).
**Charter**: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md` §2 atom table row R3 + §1.2 FR-18R.2 + FR-18R.5 + §1.3 CR-18R.5 + §1.4 SG-18R.2 + SG-18R.5.

---

## §1 Atom scope (binding per charter §2 R3 row + §0.A Q8 remediation)

R3 has **three sub-surfaces** wired together as a single atom:

### §1.1 Class-4 schema (STEP_B-restricted)

  - **`src/bottom_white/ledger/rejection_evidence.rs`**: tail-append four `RejectionClass` variants without renumbering the existing 0..5:
    - `LeanFailed = 6`        (mirrors `LeanErrorClass::LeanFailed = 6` from R1)
    - `ParseFailed = 7`       (mirrors `LeanErrorClass::ParseFailed = 7`)
    - `SorryBlocked = 8`      (mirrors `LeanErrorClass::SorryBlocked = 8`)
    - `LlmError = 9`          (mirrors `LeanErrorClass::LlmError = 9`)
  - `#[repr(u8)]` discriminator preserved; `compute_hash` already incorporates `rejection_class as u8` (rejection_evidence.rs:237) — adding new variants is byte-stable for pre-TB-18R rows (their byte stream includes only `0..5` discriminators).

### §1.2 Class-4 sequencer admission (STEP_B-restricted)

  - **`src/state/sequencer.rs`**: when a runtime-path `WorkTx` is rejected on the `predicate_passes=false` arm, the sequencer derives a fine-grained `RejectionClass` from the `WorkTx.proposal_cid` if (and only if) that CID resolves to an `AttemptTelemetry` CAS object. Mapping:
    - `AttemptTelemetry.outcome == LeanFail`   → `RejectionClass::LeanFailed = 6`
    - `AttemptTelemetry.outcome == ParseFail`  → `RejectionClass::ParseFailed = 7`
    - `AttemptTelemetry.outcome == SorryBlock` → `RejectionClass::SorryBlocked = 8`
    - `AttemptTelemetry.outcome == LlmErr`     → `RejectionClass::LlmError = 9`
    - `AttemptTelemetry.outcome == LeanPass`   → **not a rejection arm** (would be a misuse; R3 sequencer panics in test-build, logs warn + falls back to `PredicateFailed=0` in release-build — non-fatal divergence)
    - proposal_cid resolves to a non-AttemptTelemetry CAS object (legacy `ProposalTelemetry`-only path) → fall back to existing `RejectionClass::PredicateFailed = 0`. Legacy WorkTx flow byte-identical pre/post-R3.

  - This is the **Design D** decision (see §3.1 below). Pure-additive on the sequencer; backward-compatible with all pre-TB-18R chains; no `WorkTx` canonical wire-byte change (preserves R1 Design B commitment).

### §1.3 Class-3 evaluator wire-up (NOT STEP_B-restricted; piggybacked in R3 atom for cohesion)

  - **`experiments/minif2f_v4/src/bin/evaluator.rs`**: at the four failure paths (`step_partial_ok` ~3236, `step_reject` ~3263, `parse_fail` ~3275, `llm_err` ~3289) instrumented by R2, additionally **submit a runtime-path WorkTx** with:
    - `predicate_passes = false`
    - `proposal_cid` = the AttemptTelemetry CID minted by R2's `r2_write_attempt_telemetry` helper (return value already available — R2 helper returns `Result<Cid, String>`)
    - `stake = 0` (zero-stake; preflight §3.4)
  - Sequencer (per §1.2) maps `AttemptTelemetry.outcome` → fine-grained `RejectionClass` and writes a `RejectedSubmissionRecord` to L4.E.
  - For paths 1-2 (`omega-full` / `omega-pertactic`, `predicate_passes=true`), the existing TB-7 Atom-3 WorkTx pipeline is unchanged. Their AttemptTelemetry already lands on L4 via the existing `proposal_cid → ProposalTelemetry` path. **[SUPERSEDED 2026-05-06 by §3.5 amendment — see below.]** The original draft proposed cutting omega WorkTx's `proposal_cid` to AttemptTelemetry CID; the amendment keeps it as ProposalTelemetry CID to preserve TB-7 audit-chain backward compatibility. The unified-schema goal is achieved at the L4.E-only failure-path layer (where AttemptTelemetry is the proposal_cid target); L4 omega keeps ProposalTelemetry as the proposal_cid target.

### §1.4 Net effect post-R3

Every per-LLM-call externalized cycle produces:
  - one `AttemptTelemetry` CAS object (R2 already does this)
  - one chain-side record: either L4 accepted `WorkTx` (omega paths) OR L4.E `RejectedSubmissionRecord` (failure paths) — **R3 closes the M1 VETO failure-path asymmetry at the chain layer**
  - a 1:1 correspondence `AttemptTelemetry.attempt_id ↔ {WorkTx.tx_id | RejectedSubmissionRecord.submit_id}` per R1 schema doc-comment line 226-228

This is the structural shape that R4's `attempt_count_invariant` equation will assert (`evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count`).

## §2 Out of scope for R3 (forwarded to R4 / R5 / R6 / R7)

- `chain_derived_run_facts.attempt_count_invariant()` ship-gate equation + drain barrier — R4.
- `audit_tape` sampler extension to AttemptTelemetry / LeanResult — R5.
- `attempt_chain_root` Merkle root computation on final composite — R5.
- Capturing actual Lean stderr/stdout bytes into CAS (currently `LeanResult.stderr_cid = None` from R2) — out of scope for both R3 and R4; reconsidered for R5+.
- TerminalAbortRecord wire-up for budget-cap halts — R4 (with the drain barrier).
- P23/P38/P49 + M0 reruns — R6/R7.
- Any modification to historical M1 evidence (per `feedback_no_retroactive_evidence_rewrite`).

## §3 Design decisions

### §3.1 Sequencer rejection-class derivation: Design D (CAS-side mapping; no WorkTx schema change)

**Constraint**: R1 already committed to Design B for `attempt_chain_root` (no WorkTx canonical wire-byte change; preflight `handover/ai-direct/TB-18R_R1_STEP_B_schema.md` §3 + §11 Forbidden). Adding any field to WorkTx in R3 would contradict R1's commitment and break the canonical-signing-payload invariant.

**Candidates** (rejected vs chosen):

| | (A) WorkTx.rejection_class_hint | (B) RejectionClaimTx new variant | (C) Sequencer side-channel | (D) Sequencer reads AttemptTelemetry via proposal_cid |
|---|---|---|---|---|
| WorkTx wire-bytes | ❌ change | ✅ unchanged | ✅ unchanged | ✅ unchanged |
| New TypedTx variant | ✅ none | ❌ new variant | ✅ none | ✅ none |
| Sequencer reads CAS | ✅ no | ✅ no | ❌ yes | ❌ yes |
| Backward compat | ❌ legacy WorkTx breaks | ✅ legacy WorkTx unchanged | ✅ unchanged | ✅ unchanged (legacy proposal_cid → ProposalTelemetry → fall-back PredicateFailed) |
| Codex Q3 privacy fence | ⚠️ class hint near payload | ✅ unaffected | ✅ unaffected | ✅ unaffected (sequencer only reads `outcome` discriminator, never `candidate_payload_cid`) |
| Test surface | small | medium | medium | small (CAS read + match on `outcome` enum) |

**Decision**: **Design D**. Rationale:
  - Preserves R1 commitment (no WorkTx wire-byte change).
  - Pure-additive on sequencer; legacy chains replay byte-identical.
  - Sequencer already opens CAS for other paths (e.g., predicate evaluation against `proposal_cid` content); reading AttemptTelemetry is a familiar operation, not a new dependency.
  - Privacy fence holds: sequencer reads `AttemptTelemetry.outcome: AttemptOutcome` (a u8 discriminator), NEVER `candidate_payload_cid` content.

**The decision is binding for R3 and reflected in test `tb_18r_attempt_routes_to_l4_or_l4e.rs`.**

### §3.2 Stable-repr-u8 invariant (Codex Q8 ratified)

The four new variants tail-append in order `LeanFailed=6, ParseFailed=7, SorryBlocked=8, LlmError=9`. Existing variants `0..5` are NOT renumbered. Pre-TB-18R L4.E rows replay byte-identical because:
  - their `compute_hash` input includes `rejection_class as u8 ∈ {0..5}` exclusively;
  - `Serialize`/`Deserialize` derives use the same discriminators;
  - new variants only appear in chains created post-R3.

Tested by `tb_18r_rejection_class_repr_stability.rs`.

### §3.3 `From<LeanErrorClass> for RejectionClass`

Per R1 doc-comment at `attempt_telemetry.rs` lines 178-205: "R3 wires `From<LeanErrorClass>` to `RejectionClass` at the sequencer admission boundary." R3 implements this trait inside `rejection_evidence.rs`:

```rust
impl From<LeanErrorClass> for RejectionClass {
    fn from(lec: LeanErrorClass) -> Self {
        match lec {
            LeanErrorClass::LeanFailed   => RejectionClass::LeanFailed,
            LeanErrorClass::ParseFailed  => RejectionClass::ParseFailed,
            LeanErrorClass::SorryBlocked => RejectionClass::SorryBlocked,
            LeanErrorClass::LlmError     => RejectionClass::LlmError,
        }
    }
}
```

The discriminator values match (6/7/8/9 on both sides); the `From` impl is a no-op-discriminator transcode that preserves the u8 representation. Tested by `tb_18r_rejection_class_repr_stability.rs::lean_error_class_to_rejection_class_repr_preserved`.

### §3.4 Zero-stake for failure-path WorkTx (CR-18R.5 origin-tag)

Per CR-18R.5 ("No L4.E pollution from synthetic test fixtures — pre-R3 L4.E contains Atom 3 fixtures only; post-R3 L4.E adds runtime-path rejections; both coexist with origin-tag field"):

  - **Atom-3 fixture** L4.E records carry `synthetic_rejection_for_l4e_gate=true` on the WorkTx (TB-6 lift of synthetic gate; preserved verbatim in R3).
  - **Runtime-path** L4.E records (R3 NEW): WorkTx carries `predicate_passes=false` + `stake=0` + `proposal_cid → AttemptTelemetry`. The `synthetic_rejection_for_l4e_gate` flag is **explicitly false**.

Origin-tag distinguishes the two sources without a schema bump (the existing flag IS the origin-tag for fixture-vs-runtime; R3 documents this binding; no new field).

Zero-stake choice rationale:
  - Failure-path WorkTx represents a rejected attempt, not an economic commitment. Charging stake for rejected attempts would burn agent balance on every failure → economic-engine misuse.
  - Per existing sequencer admission: `stake=0` is admissible for `predicate_passes=false` WorkTx (no balance debit on rejection; balance unchanged).
  - L4.E counts are unaffected by stake amount; the `RejectedSubmissionRecord` records the rejection regardless.

### §3.5 Omega-path proposal_cid: NO cutover (revised 2026-05-06 post-implementation-audit)

**Original draft proposed**: cut omega-path WorkTx.proposal_cid from ProposalTelemetry CID to AttemptTelemetry CID.

**Revised decision**: **omega-path WorkTx.proposal_cid stays as ProposalTelemetry CID (`tel_cid`)** — no cutover.

**Reason**: implementation grep surfaced two existing audit walks that consume `L4 WorkTx.proposal_cid → ProposalTelemetry` semantics:
  - `src/runtime/verify.rs:420` Gate 5 — verify proposal_cid resolves to a ProposalTelemetry (TB-7 charter §8 Gate 5 evidence)
  - `src/runtime/audit_assertions.rs:1583` id=24 `proposal_telemetry_chain` (Layer E HALT-on-mismatch)
  - `src/runtime/audit_assertions.rs:1925` id=43 `boltzmann_parent_selection_diversity`
  - `tests/tb_7_atom6_chain_backed_smoke.rs:207` "Gate 5: every WorkTx.proposal_cid must resolve to CAS ProposalTelemetry"

These audits walk **L4 only** (the accepted spine) — they never visit L4.E. So:
  - **Omega WorkTx** (predicate_passes=true; lands on L4): `proposal_cid` MUST stay as ProposalTelemetry CID, otherwise these 4 audit walks halt at id=24. **Unchanged from R2 ship.**
  - **Failure-path WorkTx** (predicate_passes=false; R3 NEW; lands on L4.E only): `proposal_cid = AttemptTelemetry CID` is fine, since the L4-only audits never visit L4.E.

The sequencer's R3 refinement helper (preflight §1.2 Design D) is well-typed: legacy omega WorkTx hits the rejection arm only on edge cases (e.g., StaleParent), where its proposal_cid → ProposalTelemetry → `read_attempt_telemetry_from_cas` returns `Err(Codec(...))` → fall back to base class (PredicateFailed). Failure-path WorkTx hits the rejection arm by design, where its proposal_cid → AttemptTelemetry → fine-grained mapping fires.

**Net effect**: AttemptTelemetry coexists with ProposalTelemetry; both serve their respective roles (TB-7 audit chain = ProposalTelemetry; TB-18R failure-path L4.E = AttemptTelemetry). The R1 `AttemptTelemetry.proposal_telemetry_cid: Option<Cid>` field stays `None` for omega paths in R3 (could be wired in R5+ if a future audit needs the cross-link, but not load-bearing now).

**R4 invariant equation impact**: the equation `evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count` does not depend on the unified-CAS-object-walking the original §3.5 envisioned; R4 counts WorkTx by chain-side identity, not by CAS object schema. No regression.

### §3.6 Fail-close vs warn-only for sequencer CAS-read failure

In Design D, the sequencer reads AttemptTelemetry via WorkTx.proposal_cid. CAS read failures (corrupt CID, missing object, schema decode fail) need a defined behavior:

  - **In-test mode** (`#[cfg(test)]` or `cfg!(debug_assertions)`): panic with structured error so test failures are immediate.
  - **Release/runtime mode**: log warn + fall back to `RejectionClass::PredicateFailed = 0` (legacy default). Rationale: a corrupt CAS in production should NOT halt the chain (sequencer is hot-path); it degrades to coarse classification and the chain continues. R5 audit_tape sampler will detect missing AttemptTelemetry CIDs as a separate signal.

Tested by `tb_18r_lean_reject_in_l4e.rs::cas_read_failure_falls_back_to_predicate_failed`.

### §3.7 Drain barrier deferred to R4

R3 does NOT add the drain barrier (`Sequencer::drain_until_quiescent()` or equivalent) per FR-18R.3 v2 — that is R4's responsibility. R3 only ensures every `predicate_passes=false` runtime-path WorkTx that enters the sequencer leaves with an L4.E record (no silent drops). R4's drain barrier ensures every submitted WorkTx has reached terminal state before the invariant equation is evaluated.

## §4 Files touched (canonical list)

| File | Status | Surface | Diff intent |
|---|---|---|---|
| `src/bottom_white/ledger/rejection_evidence.rs` | MOD (additive) | Class 4 (STEP_B) | Tail-append 4 `RejectionClass` variants + `From<LeanErrorClass>` trait impl + 2 unit tests |
| `src/state/sequencer.rs` | MOD (additive) | Class 4 (STEP_B) | Add CAS-read + outcome-mapping logic on the `predicate_passes=false` rejection arm; no other admission logic touched |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | MOD (additive) | Class 3 (NOT STEP_B; piggyback) | Submit failure-path WorkTx at 4 paths (~3236/~3263/~3275/~3289); cut omega-path WorkTx proposal_cid to AttemptTelemetry CID at 2 paths (~2317/~2861) |
| `genesis_payload.toml` | MOD (rehash) | TR pin | Routine evaluator.rs SHA rehash (R3 changes evaluator.rs); per R2 §3.8 precedent |
| `tests/tb_18r_rejection_class_repr_stability.rs` | NEW | Class 4 witness | u8-discriminator stability + `From<LeanErrorClass>` round-trip + pre-TB-18R L4.E byte-identity |
| `tests/tb_18r_attempt_routes_to_l4_or_l4e.rs` | NEW | Class 4 witness | end-to-end: submit WorkTx with proposal_cid → AttemptTelemetry, predicate_passes={true,false}, assert L4 vs L4.E + RejectionClass mapping |
| `tests/tb_18r_lean_reject_in_l4e.rs` | NEW | Class 4 witness | failure-path round-trip per outcome (LeanFail / ParseFail / SorryBlock / LlmErr) ↦ RejectedSubmissionRecord with correct rejection_class |

Net delta target: **≥ +6 tests** (3 new test files × ≥2 fns each). Workspace 1006 → ≥ 1012.

## §5 Test plan

| Test fn | File | Asserts |
|---|---|---|
| `rejection_class_repr_stable_with_new_variants` | `tb_18r_rejection_class_repr_stability.rs` | Variants 0..5 unchanged repr; new 6..9 byte-stable; serde round-trip all 10 |
| `lean_error_class_to_rejection_class_repr_preserved` | `tb_18r_rejection_class_repr_stability.rs` | `From<LeanErrorClass>` preserves u8 discriminator (6→6, 7→7, 8→8, 9→9) |
| `pre_tb_18r_l4e_record_canonical_hash_byte_identical` | `tb_18r_rejection_class_repr_stability.rs` | A pre-R3-shape RejectedSubmissionRecord (rejection_class ∈ 0..5) `compute_hash` byte-identical pre/post-R3 source |
| `worktx_with_attempt_telemetry_proposal_cid_predicate_pass_routes_to_l4` | `tb_18r_attempt_routes_to_l4_or_l4e.rs` | predicate_passes=true + AttemptTelemetry proposal_cid → LedgerEntry on L4 (existing behavior preserved) |
| `worktx_predicate_fail_with_lean_failed_attempt_telemetry_routes_to_l4e_with_lean_failed_class` | `tb_18r_attempt_routes_to_l4_or_l4e.rs` | predicate_passes=false + AttemptTelemetry(outcome=LeanFail) → L4.E with rejection_class=LeanFailed=6 |
| `worktx_predicate_fail_legacy_proposal_telemetry_falls_back_to_predicate_failed` | `tb_18r_attempt_routes_to_l4_or_l4e.rs` | predicate_passes=false + ProposalTelemetry proposal_cid (legacy shape) → L4.E with rejection_class=PredicateFailed=0 (backward compat) |
| `cas_read_failure_falls_back_to_predicate_failed` | `tb_18r_attempt_routes_to_l4_or_l4e.rs` | predicate_passes=false + missing/corrupt CAS object → release mode warn-and-fallback to PredicateFailed=0 (debug mode panics — tested with cfg(release_mode)-emulated path) |
| `parse_fail_attempt_routes_to_l4e_with_parse_failed_class` | `tb_18r_lean_reject_in_l4e.rs` | AttemptTelemetry(outcome=ParseFail) → L4.E rejection_class=ParseFailed=7 |
| `sorry_block_attempt_routes_to_l4e_with_sorry_blocked_class` | `tb_18r_lean_reject_in_l4e.rs` | AttemptTelemetry(outcome=SorryBlock) → L4.E rejection_class=SorryBlocked=8 |
| `llm_err_attempt_routes_to_l4e_with_llm_error_class` | `tb_18r_lean_reject_in_l4e.rs` | AttemptTelemetry(outcome=LlmErr) → L4.E rejection_class=LlmError=9 |

10 tests total. Plus existing R1 tests (`tb_18r_attempt_telemetry_serialize`, `tb_18r_lean_result_cas_resolves`, `tb_18r_no_raw_response_in_attempt_payload`, `tb_18r_attempt_chain_root_payload_schema`) and R2 tests (`tb_18r_attempt_telemetry_per_llm_call` 8 fns) all stay green.

## §6 Risk matrix

| Risk | Probability | Mitigation |
|---|---|---|
| `RejectionClass` enum tail-append breaks pre-R3 L4.E canonical hash | Low (repr-u8 stable; new variants only on new chains) | `pre_tb_18r_l4e_record_canonical_hash_byte_identical` test; manual smoke against historical L4.E fixture |
| Sequencer CAS-read introduces hot-path latency | Low (CAS is local SSD; ~100µs per read; only on rejection path which is already slow due to RejectedSubmissionRecord write) | R5/R7 evidence runs measure rejection-path wall-clock; if regression > 10% revisit Design D |
| AttemptTelemetry decode failure cascades into chain halt | Mitigated (preflight §3.6: warn-and-fallback in release mode) | `cas_read_failure_falls_back_to_predicate_failed` test |
| Omega-path proposal_cid cutover breaks TB-7 Atom-3 audit assertions | Medium (TB-7 audit assertions check `proposal_cid → ProposalTelemetry` resolves) | R3 keeps the existing `ProposalTelemetry::write_to_cas` call; only the `WorkTx.proposal_cid` field flips to AttemptTelemetry CID. The AttemptTelemetry's `proposal_telemetry_cid` field preserves the link; tests for TB-7 audit chain assertions still resolve via the back-pointer |
| Class-4 schema change requires post-G2 architect re-ratification | Mitigated (Codex G1 already authorized the SHAPE per Q8 remediation) | Charter §0.A row Q8 specifies exact discriminator values + tail-append rule; G2 audits IMPLEMENTATION compliance |
| Evaluator wire-up at failure paths introduces FAIL-CLOSED divergence from R2 | Low (R3 reuses R2's chaintape gate + FAIL-CLOSED pattern) | `cargo test --workspace` covers; smoke probe per §8 step 4 |
| `From<LeanErrorClass>` trait impl + tail-append in same commit doubles the schema-test surface | Low | Both tests in `tb_18r_rejection_class_repr_stability.rs`; single test file; ~80 lines |

## §7 STEP_B parallel-branch plan (binding)

Per CLAUDE.md STEP_B_PROTOCOL + R1 STEP_B precedent (`handover/ai-direct/TB-18R_R1_STEP_B_schema.md` §9 + §10):

1. **Phase 0 — necessity audit (this preflight)**: Design D vs A/B/C tradeoff documented (§3.1); user reviews preflight before worktree creation.
2. **Phase 1a — worktree spawn**:
   ```bash
   git worktree add .claude/worktrees/stepb-tb18r-r3-admission -b tb-18r-r3-admission main
   ```
   Off `main` at HEAD `aa08f50` (post-R2 ship + TB_LOG row).
3. **Phase 1b — implementation in isolation**:
   - Implement §1.1 schema tail-append + `From<LeanErrorClass>` trait + 2 unit tests in rejection_evidence.rs.
   - Implement §1.2 sequencer admission CAS-read + mapping in sequencer.rs.
   - Implement §1.3 evaluator wire-up at 4 failure paths. **[SUPERSEDED 2026-05-06 by §3.5 amendment]** Original line additionally listed "omega proposal_cid cutover at 2 success paths"; the amendment removed that work — omega-path WorkTx.proposal_cid stays as ProposalTelemetry CID. As-shipped: 4 failure-path wire-ups only.
   - Update genesis_payload.toml Trust Root pin for evaluator.rs.
   - Add 3 new integration test files (10 tests total).
   - `cargo check --workspace` clean compile.
   - `cargo test --workspace --no-fail-fast` → workspace 1006 → ≥ 1012; 0 R3-attributable failures.
4. **Phase 1c — implementation audit**:
   - Diff review (no statistical A/B; this is mechanism-edit not behavior-edit; per STEP_B_PROTOCOL.md "Phase 2 statistical A/B" applies to behavior changes; R3 is a schema + admission change validated by mechanism tests).
   - User reviews diff before merge.
5. **Phase 2 (skipped)**: no statistical A/B test; R6/R7 evidence runs serve as the empirical test of the full TB-18R chain.
6. **Phase 3 — merge**:
   - On user approval: `git merge tb-18r-r3-admission --no-ff` on main.
   - TB_LOG.tsv R3 ship row appended.
   - Worktree removed: `git worktree remove .claude/worktrees/stepb-tb18r-r3-admission`.
7. **Phase 3b — abandonment** (if anything goes red): branch archived as `archive/tb-18r-r3-admission_2026-05-06`; main remains at `aa08f50`.

**Iteration cap** (charter §0.A R3 row): **72h post-Gate-1**. Gate 1 closed 2026-05-06; deadline = 2026-05-09 + STEP_B preflight time. R3 launches when user grants explicit "go" on this preflight.

## §8 Validation steps before R3 ship

1. `cargo check --workspace` clean compile.
2. `cargo test --workspace --no-fail-fast` → 1006 → ≥ 1012; 0 R3-attributable failures.
3. Diff review: confirm `WorkTx` canonical wire-bytes unchanged (per R1 Design B commitment).
4. Smoke probe: run evaluator on **one** problem (e.g. P01_mathd_algebra_107) in chaintape mode; assert `runtime_repo/L4_E.jsonl` (or equivalent L4.E sidecar) contains ≥1 RejectedSubmissionRecord with `rejection_class ∈ {6, 7, 8, 9}`. Closes the M1 VETO empirical test (P49 expected 32 attempts; smoke can validate even with 1-attempt run since the path now writes to L4.E).
5. Commit on worktree branch; report deltas to user; user reviews + merges.
6. TB_LOG.tsv R3 ship row appended.

## §9 Forbidden in R3

- Touching `WorkTx` canonical wire bytes (Design B inherited from R1; preflight §3.1 + §3.5).
- Renumbering existing `RejectionClass` variants `0..5` (Codex Q8 ratified stable-repr-u8 invariant).
- Adding a new TypedTx variant (Design B from §3.1; Class-4 typed-tx territory; would require separate ratification).
- Routing `LeanPass`-outcome AttemptTelemetry to L4.E (semantically wrong; preflight §1.2 panics in test-build).
- Computing `attempt_chain_root` Merkle (R5 scope).
- Touching `chain_derived_run_facts.rs` (R4 scope).
- Adding the drain barrier (R4 scope).
- Any retroactive modification to historical L4.E records (per `feedback_no_retroactive_evidence_rewrite`; CR-18R.1).
- Charging non-zero stake for failure-path WorkTx (preflight §3.4 economic-engine misuse argument).
- Skipping the `From<LeanErrorClass>` trait impl (R1 doc-comment promise; R3 binding).

## §10 Sign-off

This preflight grants implementation authority for R3 within the listed scope **upon user "go"** per charter §1.1 Opening rule for Class-4 atoms (Codex Gate 1 authorizes the SHAPE; user explicit "go" authorizes the IMPLEMENTATION).

  - Pre-impl: this doc filed under `handover/ai-direct/`; user reviews + grants "go".
  - Phase 1a: worktree + branch creation.
  - Phase 1b: implementation + tests in worktree.
  - Phase 1c: diff review (Claude self-pass + user review).
  - Phase 3: merge to main on user approval; TB_LOG row appended.

**End of R3 preflight. Awaits user "go" before Phase 1a worktree creation.**
