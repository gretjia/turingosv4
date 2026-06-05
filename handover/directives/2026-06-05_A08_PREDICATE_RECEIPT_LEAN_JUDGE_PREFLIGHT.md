# A08 PredicateReceipt And LeanJudge Preflight

Date: 2026-06-05

Parent plan:
`handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

Atom: A08. PredicateReceipt and LeanJudge Axiom Gate

Document role: Class 0 preflight. This document does not authorize predicate
authority, sequencer admission, typed transaction, CAS schema, or Lean runner
changes by itself.

## Decision

A08 is correct as an architectural need, but the original path list is not the
current repo surface. The planned files do not exist. Existing Lean/predicate
authority is split across the predicate registry, LeanResult/VerificationResult
CAS evidence, and sequencer predicate verification.

Safe work now:

- docs-only preflight
- standalone LeanJudge test-shape design
- PredicateReceipt data-shape design
- explicit authority-wiring split

Blocked without further ratification:

- wiring a new LeanJudge into predicate pass/fail authority
- changing `LeanArtifactPredicate` or the boot predicate catalog
- changing sequencer admission or typed tx predicate-result schema
- adding a dedicated CAS `ObjectType` for PredicateReceipt
- claiming `#print axioms` soundness from exit-code-only Lean success

## Hard Blockers

- A08-HB1: Predicate admission authority, sequencer admission, typed tx schema,
  boot predicate catalog, or dedicated PredicateReceipt CAS `ObjectType` changes
  are blocked without explicit ratification.
- A08-HB2: Lean exit code 0 cannot be accepted as proof without a failing
  `#print axioms` whitelist gate and forbidden-source positive controls.
- A08-HB3: `lean_market_agent` and `market_tape_shared` are forbidden in A08
  verifier work.
- A08-HB4: Standalone LeanJudge tests and predicate-authority wiring must remain
  separate acceptance scopes.

## Current-State Facts

Parent-plan A08 allowed paths as originally written:

```text
src/runtime/predicate_receipt.rs
tests/predicate_receipt_replay.rs
src/judges/lean_judge.rs
tests/lean_judge_axiom_gate.rs
```

Corrected implementation path inventory:

```text
src/runtime/predicate_receipt.rs
src/runtime/mod.rs
tests/predicate_receipt_replay.rs
src/judges/lean_judge.rs
src/judges/mod.rs
tests/lean_judge_axiom_gate.rs
src/top_white/predicates/registry.rs
src/runtime/attempt_telemetry.rs
src/runtime/verification_result.rs
src/runtime/predicate_registry_loader.rs
src/state/sequencer.rs
src/state/typed_tx.rs
tests/constitution_predicate_registry_binding.rs
tests/constitution_predicate_binding_activation.rs
tests/constitution_predicate_registry_replay.rs
tests/constitution_predicate_registry_immutability.rs
tests/constitution_predicate_result_wire_freeze.rs
tests/tb_18r_lean_verdict_kind_consistency.rs
tests/tb_18r_lean_result_cas_resolves.rs
tests/tb_18r_lean_reject_in_l4e.rs
```

Existence check:

```text
MISSING src/runtime/predicate_receipt.rs
MISSING tests/predicate_receipt_replay.rs
MISSING src/judges/lean_judge.rs
MISSING tests/lean_judge_axiom_gate.rs
EXISTS src/runtime/mod.rs
EXISTS src/judges/mod.rs
EXISTS src/top_white/predicates/registry.rs
EXISTS src/runtime/attempt_telemetry.rs
EXISTS src/runtime/verification_result.rs
EXISTS src/runtime/predicate_registry_loader.rs
EXISTS src/state/sequencer.rs
EXISTS src/state/typed_tx.rs
```

Dirty-path check for corrected inventory:

```text
pre-existing dirty paths include:
  src/judges/mod.rs
  src/judges/generate_judge.rs
  src/judges/injected_judge.rs
  src/judges/math_step_judge.rs
  src/judges/putnam_2024_a1_judge.rs
  src/judges/putnam_2025_b3_judge.rs
  src/runtime/mod.rs
  src/runtime/audit_assertions.rs
  src/state/sequencer.rs
  src/state/typed_tx.rs

Implementation must read and preserve those edits. Do not overwrite them as
part of A08 scaffolding.
```

Existing predicate and Lean witnesses:

```text
src/top_white/predicates/registry.rs:606
  boot predicate catalog includes forbidden_patterns_v1, sorry_free_v1,
  payload_size_v1, and lean_artifact_v1
src/top_white/predicates/registry.rs:625
  forbidden pattern scan includes native_decide, unsafe, and axiom
src/top_white/predicates/registry.rs:880
  SorryFree token scan rejects sorry/admit
src/runtime/attempt_telemetry.rs:633
  LeanResult carries exit_code, verified, shielded stdout/stderr CIDs,
  proof_artifact_cid, error_class, and LeanVerdictKind
src/runtime/attempt_telemetry.rs:720
  LeanResult typed-verdict consistency check
src/runtime/attempt_telemetry.rs:942
  LeanResult CAS write/read helpers
src/runtime/verification_result.rs:35
  older VerificationResult records Lean oracle verdict as CAS object
src/runtime/verification_result.rs:92
  VerificationResult::from_lean_run currently derives verified from
  lean_exit_code == 0
src/runtime/predicate_registry_loader.rs:8
  replay loader builds v8 production predicate registry
tests/constitution_predicate_registry_binding.rs:146
  forged predicate true is rejected when registry recomputes false
tests/constitution_predicate_registry_replay.rs:78
  replay reconstructs predicate activation with predicate CAS view
tests/constitution_predicate_registry_immutability.rs:3
  predicate registry mutation surface is crate-private
tests/constitution_predicate_result_wire_freeze.rs:9
  BoolWithProof / PredicateResultsBundle wire shape freeze
tests/tb_18r_lean_verdict_kind_consistency.rs:39
  canonical LeanResult verdict shapes and drift failures
tests/tb_18r_lean_result_cas_resolves.rs:23
  LeanResult CAS roundtrip with shielded stdout/stderr/proof CIDs
tests/tb_18r_lean_reject_in_l4e.rs:88
  sorry and Lean failure outcomes route to L4.E rejection classes
```

Missing soundness witness:

```text
No current standalone src/judges/lean_judge.rs module.
No current tests/lean_judge_axiom_gate.rs.
No current #print axioms fail-closed gate.
Current VerificationResult helper treats lean_exit_code == 0 as verified.
Current registry source scans forbid sorry/admit/native_decide/unsafe/axiom,
but that is not equivalent to an axiom whitelist from Lean kernel output.
```

## Risk Classification

Risk floor: Class 2 for standalone data types, tests, and a non-authoritative
LeanJudge helper.

Promote to Class 3 if:

- PredicateReceipt becomes an admission, settlement, or economy input
- LeanJudge output changes production pass/fail behavior
- predicate registry boot catalog changes
- Lean runner behavior changes in production

Promote to Class 4 if:

- sequencer admission changes
- typed tx schema or predicate-result wire shape changes
- canonical signing payload changes
- predicate registry authority model changes
- CAS ObjectType schema changes
- trust-root / constitution / flowchart authority changes

## Recommended Contract

Standalone LeanJudge output:

```text
LeanJudgeVerdict {
  attempt_id: TxId,
  exit_code: i32,
  kernel_verified: bool,
  axiom_check_status: AxiomCheckStatus,
  rejected_axioms: Vec<String>,
  proof_artifact_cid: Option<Cid>,
  stdout_cid: Option<Cid>,
  stderr_cid: Option<Cid>,
}

AxiomCheckStatus =
  PassedWhitelisted
  RejectedNonWhitelisted
  AxiomProbeFailed
  SourceForbiddenPattern
  LeanFailed
```

PredicateReceipt should remain a derived receipt over CAS/tape evidence until
an authority-wiring atom is explicitly ratified:

```text
PredicateReceipt {
  predicate_id: PredicateId,
  subject_tx_id: TxId,
  tape_event_id: Option<EventId>,
  input_cid: Cid,
  verdict_cid: Cid,
  registry_root: Hash,
  result: bool,
}
```

Use `ObjectType::Generic + schema_id` for PredicateReceipt CAS payloads unless
a dedicated CAS enum variant receives explicit schema-risk ratification.

## Atomized A08 Tasks

### A08.0 Preflight Lock

Description:
Record missing files, real authority surfaces, dirty-path conflicts, and the
split between standalone judge tests and production predicate authority.

Acceptance:

```bash
for f in \
  handover/directives/2026-06-05_A08_PREDICATE_RECEIPT_LEAN_JUDGE_PREFLIGHT.md \
  handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
do
  git diff --no-index --check /dev/null "$f" || true
done
```

Expected:

```text
no whitespace errors.
A08 preflight states A05 is predecessor for replay projection.
A08 preflight states authority wiring is not authorized by this document.
```

### A08.1 Standalone LeanJudge Axiom Gate

Description:
Add a non-authoritative LeanJudge helper and tests that fail closed on
non-whitelisted axioms, `#print axioms` probe failure, `sorry`, `admit`, and
unsafe shortcuts. This step may not change predicate admission behavior.

Acceptance:

```bash
cargo test lean_judge --lib --no-fail-fast
cargo test --test lean_judge_axiom_gate --no-fail-fast -- --test-threads=1
git diff --check
grep -RIn 'market_tape_shared\|lean_market_agent' src/judges src/runtime tests && exit 1 || true
```

Expected:

```text
non-whitelisted axiom -> axiom_rejected.
#print axioms failure -> axiom_rejected.
sorry/admit/native_decide/unsafe shortcut -> rejected.
standalone LeanJudge does not affect sequencer admission.
```

### A08.2 PredicateReceipt Replay Contract

Description:
After A05 exists, add PredicateReceipt as a derived receipt over TapeEvent/CAS
evidence. Do not make it a second predicate authority.

Acceptance:

```bash
cargo test --test predicate_receipt_replay --no-fail-fast -- --test-threads=1
cargo test --test constitution_predicate_registry_binding --no-fail-fast -- --test-threads=1
cargo test --test constitution_predicate_registry_replay --no-fail-fast -- --test-threads=1
cargo test --test constitution_predicate_result_wire_freeze --no-fail-fast
git diff --check
```

Expected:

```text
PredicateReceipt replay reconstructs verdict from tape/CAS and registry root.
BoolWithProof / PredicateResultsBundle wire shape remains unchanged.
PredicateReceipt is not a board or manifest-only proof.
```

### A08.3 Authority Wiring

Description:
Only after explicit ratification, wire LeanJudge/PredictateReceipt into
production predicate authority. This is not part of A08.0/A08.1/A08.2.

Acceptance:

```bash
cargo test --test constitution_predicate_registry_binding --no-fail-fast -- --test-threads=1
cargo test --test constitution_predicate_binding_activation --no-fail-fast -- --test-threads=1
cargo test --test constitution_predicate_registry_replay --no-fail-fast -- --test-threads=1
cargo test --test constitution_predicate_registry_immutability --no-fail-fast
cargo test --test tb_18r_lean_verdict_kind_consistency --no-fail-fast
cargo test --test tb_18r_lean_result_cas_resolves --no-fail-fast
cargo test --test tb_18r_lean_reject_in_l4e --no-fail-fast
bash scripts/run_constitution_gates.sh
git diff --check
```

Expected:

```text
predicate pass/fail is recomputed by active registry.
Lean exit 0 alone is not sufficient when axiom gate fails.
source scan and #print axioms gate both fail closed.
no sequencer/typed-tx/CAS schema changes occur without explicit ratification.
```

## Final Pre-Implementation Gate

A08 implementation may start only when all are true:

- A05 has landed or the work is explicitly limited to standalone tests/helpers
- the first code change is a failing LeanJudge or PredicateReceipt test
- `src/runtime/mod.rs` and `src/judges/mod.rs` export impacts are included
- dirty judge/runtime/sequencer/typed-tx edits are read and preserved
- authority wiring is either explicitly out of scope or has per-atom §8
  ratification

Clean-context audit input for a future implementation PR:

```text
Task brief: A08 PredicateReceipt and LeanJudge Axiom Gate.
Risk class: Class 2 for standalone judge/receipt tests; promote to Class 3/4
if predicate admission, sequencer, typed tx, CAS schema, or boot predicate
authority changes.
FC nodes: FC1-N11, FC1-N12, FC1-N14, FC1-N15.
Evidence: A05 predecessor evidence, Lean axiom gate tests, PredicateReceipt
replay tests, predicate registry tests, no-market_tape_shared grep,
constitution gates.
Verdict domain: NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE |
SECOND-SOURCE-DRIFT
```
